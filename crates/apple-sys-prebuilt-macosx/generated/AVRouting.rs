#[allow(unused_imports)]
use crate::objc::*;
#[allow(unused_imports)]
use crate::CoreFoundation::*;
#[allow(unused_imports)]
use crate::Foundation::*;
#[allow(unused_imports)]
use crate::Network::*;
#[allow(unused_imports)]
use crate::UniformTypeIdentifiers::*;

#[allow(unused_imports)]
use objc2::msg_send;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct AVCustomRoutingController(pub id);
impl std::ops::Deref for AVCustomRoutingController {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for AVCustomRoutingController {}
impl AVCustomRoutingController {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"AVCustomRoutingController").unwrap(), alloc) })
    }
}
impl INSObject for AVCustomRoutingController {}
impl PNSObject for AVCustomRoutingController {}
impl std::convert::TryFrom<NSObject> for AVCustomRoutingController {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<AVCustomRoutingController, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"AVCustomRoutingController").unwrap()) };
        if is_kind_of {
            Ok(AVCustomRoutingController(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to AVCustomRoutingController")
        }
    }
}
impl IAVCustomRoutingController for AVCustomRoutingController {}
pub trait IAVCustomRoutingController: Sized + std::ops::Deref {
    unsafe fn invalidateAuthorizationForRoute_(&self, route: AVCustomDeviceRoute)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, invalidateAuthorizationForRoute : route)
    }
    unsafe fn setActive_forRoute_(&self, active: BOOL, route: AVCustomDeviceRoute)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setActive : active, forRoute : route)
    }
    unsafe fn isRouteActive_(&self, route: AVCustomDeviceRoute) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, isRouteActive : route)
    }
    unsafe fn delegate(&self) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, delegate)
    }
    unsafe fn setDelegate_(&self, delegate: *mut u64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDelegate : delegate)
    }
    unsafe fn authorizedRoutes(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, authorizedRoutes)
    }
    unsafe fn knownRouteIPs(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, knownRouteIPs)
    }
    unsafe fn setKnownRouteIPs_(&self, knownRouteIPs: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setKnownRouteIPs : knownRouteIPs)
    }
    unsafe fn customActionItems(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, customActionItems)
    }
    unsafe fn setCustomActionItems_(&self, customActionItems: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCustomActionItems : customActionItems)
    }
}
pub trait PAVCustomRoutingControllerDelegate: Sized + std::ops::Deref {
    unsafe fn customRoutingController_handleEvent_completionHandler_(
        &self,
        controller: AVCustomRoutingController,
        event: AVCustomRoutingEvent,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, customRoutingController : controller, handleEvent : event, completionHandler : completionHandler)
    }
    unsafe fn customRoutingController_eventDidTimeOut_(
        &self,
        controller: AVCustomRoutingController,
        event: AVCustomRoutingEvent,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, customRoutingController : controller, eventDidTimeOut : event)
    }
    unsafe fn customRoutingController_didSelectItem_(
        &self,
        controller: AVCustomRoutingController,
        customActionItem: AVCustomRoutingActionItem,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, customRoutingController : controller, didSelectItem : customActionItem)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct AVCustomRoutingPartialIP(pub id);
impl std::ops::Deref for AVCustomRoutingPartialIP {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for AVCustomRoutingPartialIP {}
impl AVCustomRoutingPartialIP {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"AVCustomRoutingPartialIP").unwrap(), alloc) })
    }
}
impl INSObject for AVCustomRoutingPartialIP {}
impl PNSObject for AVCustomRoutingPartialIP {}
impl std::convert::TryFrom<NSObject> for AVCustomRoutingPartialIP {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<AVCustomRoutingPartialIP, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"AVCustomRoutingPartialIP").unwrap()) };
        if is_kind_of {
            Ok(AVCustomRoutingPartialIP(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to AVCustomRoutingPartialIP")
        }
    }
}
impl IAVCustomRoutingPartialIP for AVCustomRoutingPartialIP {}
pub trait IAVCustomRoutingPartialIP: Sized + std::ops::Deref {
    unsafe fn initWithAddress_mask_(&self, address: NSData, mask: NSData) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithAddress : address, mask : mask)
    }
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn address(&self) -> NSData
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, address)
    }
    unsafe fn mask(&self) -> NSData
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, mask)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"AVCustomRoutingPartialIP").unwrap(), new)
    }
}
pub type AVCustomRoutingEventReason = NSInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct AVCustomRoutingEvent(pub id);
impl std::ops::Deref for AVCustomRoutingEvent {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for AVCustomRoutingEvent {}
impl AVCustomRoutingEvent {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"AVCustomRoutingEvent").unwrap(), alloc) })
    }
}
impl INSObject for AVCustomRoutingEvent {}
impl PNSObject for AVCustomRoutingEvent {}
impl std::convert::TryFrom<NSObject> for AVCustomRoutingEvent {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<AVCustomRoutingEvent, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"AVCustomRoutingEvent").unwrap()) };
        if is_kind_of {
            Ok(AVCustomRoutingEvent(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to AVCustomRoutingEvent")
        }
    }
}
impl IAVCustomRoutingEvent for AVCustomRoutingEvent {}
pub trait IAVCustomRoutingEvent: Sized + std::ops::Deref {
    unsafe fn reason(&self) -> AVCustomRoutingEventReason
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, reason)
    }
    unsafe fn route(&self) -> AVCustomDeviceRoute
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, route)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct AVCustomRoutingActionItem(pub id);
impl std::ops::Deref for AVCustomRoutingActionItem {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for AVCustomRoutingActionItem {}
impl AVCustomRoutingActionItem {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"AVCustomRoutingActionItem").unwrap(), alloc) })
    }
}
impl INSObject for AVCustomRoutingActionItem {}
impl PNSObject for AVCustomRoutingActionItem {}
impl std::convert::TryFrom<NSObject> for AVCustomRoutingActionItem {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<AVCustomRoutingActionItem, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"AVCustomRoutingActionItem").unwrap()) };
        if is_kind_of {
            Ok(AVCustomRoutingActionItem(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to AVCustomRoutingActionItem")
        }
    }
}
impl IAVCustomRoutingActionItem for AVCustomRoutingActionItem {}
pub trait IAVCustomRoutingActionItem: Sized + std::ops::Deref {
    unsafe fn type_(&self) -> UTType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, type)
    }
    unsafe fn setType_(&self, type_: UTType)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setType : type_)
    }
    unsafe fn overrideTitle(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, overrideTitle)
    }
    unsafe fn setOverrideTitle_(&self, overrideTitle: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setOverrideTitle : overrideTitle)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct AVCustomDeviceRoute(pub id);
impl std::ops::Deref for AVCustomDeviceRoute {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for AVCustomDeviceRoute {}
impl AVCustomDeviceRoute {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"AVCustomDeviceRoute").unwrap(), alloc) })
    }
}
impl INSObject for AVCustomDeviceRoute {}
impl PNSObject for AVCustomDeviceRoute {}
impl std::convert::TryFrom<NSObject> for AVCustomDeviceRoute {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<AVCustomDeviceRoute, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"AVCustomDeviceRoute").unwrap()) };
        if is_kind_of {
            Ok(AVCustomDeviceRoute(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to AVCustomDeviceRoute")
        }
    }
}
impl IAVCustomDeviceRoute for AVCustomDeviceRoute {}
pub trait IAVCustomDeviceRoute: Sized + std::ops::Deref {
    unsafe fn networkEndpoint(&self) -> nw_endpoint_t
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, networkEndpoint)
    }
    unsafe fn bluetoothIdentifier(&self) -> NSUUID
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, bluetoothIdentifier)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct AVRoutingPlaybackArbiter(pub id);
impl std::ops::Deref for AVRoutingPlaybackArbiter {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for AVRoutingPlaybackArbiter {}
impl AVRoutingPlaybackArbiter {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"AVRoutingPlaybackArbiter").unwrap(), alloc) })
    }
}
impl INSObject for AVRoutingPlaybackArbiter {}
impl PNSObject for AVRoutingPlaybackArbiter {}
impl std::convert::TryFrom<NSObject> for AVRoutingPlaybackArbiter {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<AVRoutingPlaybackArbiter, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"AVRoutingPlaybackArbiter").unwrap()) };
        if is_kind_of {
            Ok(AVRoutingPlaybackArbiter(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to AVRoutingPlaybackArbiter")
        }
    }
}
impl IAVRoutingPlaybackArbiter for AVRoutingPlaybackArbiter {}
pub trait IAVRoutingPlaybackArbiter: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn preferredParticipantForNonMixableAudioRoutes(&self) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, preferredParticipantForNonMixableAudioRoutes)
    }
    unsafe fn setPreferredParticipantForNonMixableAudioRoutes_(
        &self,
        preferredParticipantForNonMixableAudioRoutes: *mut u64,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPreferredParticipantForNonMixableAudioRoutes : preferredParticipantForNonMixableAudioRoutes)
    }
    unsafe fn preferredParticipantForExternalPlayback(&self) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, preferredParticipantForExternalPlayback)
    }
    unsafe fn setPreferredParticipantForExternalPlayback_(
        &self,
        preferredParticipantForExternalPlayback: *mut u64,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPreferredParticipantForExternalPlayback : preferredParticipantForExternalPlayback)
    }
    unsafe fn sharedRoutingPlaybackArbiter() -> AVRoutingPlaybackArbiter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"AVRoutingPlaybackArbiter").unwrap(), sharedRoutingPlaybackArbiter)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"AVRoutingPlaybackArbiter").unwrap(), new)
    }
}
pub trait PAVRoutingPlaybackParticipant: Sized + std::ops::Deref {}
unsafe extern "C" {
    pub static AVCustomRoutingControllerAuthorizedRoutesDidChangeNotification: NSNotificationName;
}

unsafe impl objc2::encode::RefEncode for AVCustomRoutingController {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for AVCustomRoutingController {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for AVCustomRoutingPartialIP {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for AVCustomRoutingPartialIP {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for AVCustomRoutingEvent {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for AVCustomRoutingEvent {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for AVCustomRoutingActionItem {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for AVCustomRoutingActionItem {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for AVCustomDeviceRoute {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for AVCustomDeviceRoute {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for AVRoutingPlaybackArbiter {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for AVRoutingPlaybackArbiter {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
