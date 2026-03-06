#[allow(unused_imports)]
use crate::objc::*;
#[allow(unused_imports)]
use crate::Foundation::*;
#[allow(unused_imports)]
use crate::Network::*;
#[allow(unused_imports)]
use crate::Security::*;
#[allow(unused_imports)]
use crate::UniformTypeIdentifiers::*;

#[allow(unused_imports)]
use objc2::msg_send;
pub type DDDeviceProtocolString = NSString;
pub type DDDeviceProtocol = NSInteger;
pub type DDDeviceCategory = NSInteger;
pub type DDDeviceState = NSInteger;
pub type DDDeviceMediaPlaybackState = NSInteger;
pub type DDDeviceSupports = NSUInteger;
pub type DDDeviceWiFiAwareServiceRole = NSInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct DDDevice(pub id);
impl std::ops::Deref for DDDevice {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for DDDevice {}
impl DDDevice {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"DDDevice").unwrap(), alloc) })
    }
}
impl INSObject for DDDevice {}
impl PNSObject for DDDevice {}
impl std::convert::TryFrom<NSObject> for DDDevice {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<DDDevice, Self::Error> {
        let is_kind_of: bool = unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"DDDevice").unwrap()) };
        if is_kind_of {
            Ok(DDDevice(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to DDDevice")
        }
    }
}
impl IDDDevice for DDDevice {}
pub trait IDDDevice: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn initWithDisplayName_category_protocolType_identifier_(
        &self,
        displayName: NSString,
        category: DDDeviceCategory,
        protocolType: UTType,
        identifier: NSString,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithDisplayName : displayName, category : category, protocolType : protocolType, identifier : identifier)
    }
    unsafe fn deviceSupports(&self) -> DDDeviceSupports
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, deviceSupports)
    }
    unsafe fn setDeviceSupports_(&self, deviceSupports: DDDeviceSupports)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDeviceSupports : deviceSupports)
    }
    unsafe fn bluetoothIdentifier(&self) -> NSUUID
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, bluetoothIdentifier)
    }
    unsafe fn setBluetoothIdentifier_(&self, bluetoothIdentifier: NSUUID)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setBluetoothIdentifier : bluetoothIdentifier)
    }
    unsafe fn category(&self) -> DDDeviceCategory
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, category)
    }
    unsafe fn setCategory_(&self, category: DDDeviceCategory)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCategory : category)
    }
    unsafe fn displayImageName(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, displayImageName)
    }
    unsafe fn setDisplayImageName_(&self, displayImageName: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDisplayImageName : displayImageName)
    }
    unsafe fn displayName(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, displayName)
    }
    unsafe fn setDisplayName_(&self, displayName: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDisplayName : displayName)
    }
    unsafe fn identifier(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, identifier)
    }
    unsafe fn setIdentifier_(&self, identifier: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setIdentifier : identifier)
    }
    unsafe fn mediaPlaybackState(&self) -> DDDeviceMediaPlaybackState
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, mediaPlaybackState)
    }
    unsafe fn setMediaPlaybackState_(&self, mediaPlaybackState: DDDeviceMediaPlaybackState)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMediaPlaybackState : mediaPlaybackState)
    }
    unsafe fn mediaContentTitle(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, mediaContentTitle)
    }
    unsafe fn setMediaContentTitle_(&self, mediaContentTitle: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMediaContentTitle : mediaContentTitle)
    }
    unsafe fn mediaContentSubtitle(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, mediaContentSubtitle)
    }
    unsafe fn setMediaContentSubtitle_(&self, mediaContentSubtitle: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMediaContentSubtitle : mediaContentSubtitle)
    }
    unsafe fn networkEndpoint(&self) -> nw_endpoint_t
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, networkEndpoint)
    }
    unsafe fn setNetworkEndpoint_(&self, networkEndpoint: NSObject)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setNetworkEndpoint : networkEndpoint)
    }
    unsafe fn protocol(&self) -> DDDeviceProtocol
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, protocol)
    }
    unsafe fn setProtocol_(&self, protocol: DDDeviceProtocol)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setProtocol : protocol)
    }
    unsafe fn protocolType(&self) -> UTType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, protocolType)
    }
    unsafe fn setProtocolType_(&self, protocolType: UTType)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setProtocolType : protocolType)
    }
    unsafe fn state(&self) -> DDDeviceState
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, state)
    }
    unsafe fn setState_(&self, state: DDDeviceState)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setState : state)
    }
    unsafe fn SSID(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, SSID)
    }
    unsafe fn setSSID_(&self, SSID: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSSID : SSID)
    }
    unsafe fn supportsGrouping(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, supportsGrouping)
    }
    unsafe fn setSupportsGrouping_(&self, supportsGrouping: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSupportsGrouping : supportsGrouping)
    }
    unsafe fn txtRecordData(&self) -> NSData
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, txtRecordData)
    }
    unsafe fn setTxtRecordData_(&self, txtRecordData: NSData)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setTxtRecordData : txtRecordData)
    }
    unsafe fn url(&self) -> NSURL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, url)
    }
    unsafe fn setUrl_(&self, url: NSURL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setUrl : url)
    }
    unsafe fn wifiAwareServiceName(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, wifiAwareServiceName)
    }
    unsafe fn setWifiAwareServiceName_(&self, wifiAwareServiceName: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setWifiAwareServiceName : wifiAwareServiceName)
    }
    unsafe fn wifiAwareServiceRole(&self) -> DDDeviceWiFiAwareServiceRole
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, wifiAwareServiceRole)
    }
    unsafe fn setWifiAwareServiceRole_(&self, wifiAwareServiceRole: DDDeviceWiFiAwareServiceRole)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setWifiAwareServiceRole : wifiAwareServiceRole)
    }
    unsafe fn wifiAwareModelName(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, wifiAwareModelName)
    }
    unsafe fn setWifiAwareModelName_(&self, wifiAwareModelName: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setWifiAwareModelName : wifiAwareModelName)
    }
    unsafe fn wifiAwareVendorName(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, wifiAwareVendorName)
    }
    unsafe fn setWifiAwareVendorName_(&self, wifiAwareVendorName: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setWifiAwareVendorName : wifiAwareVendorName)
    }
}
pub type DDErrorCode = NSInteger;
pub type DDEventType = NSInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct DDDeviceEvent(pub id);
impl std::ops::Deref for DDDeviceEvent {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for DDDeviceEvent {}
impl DDDeviceEvent {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"DDDeviceEvent").unwrap(), alloc) })
    }
}
impl INSObject for DDDeviceEvent {}
impl PNSObject for DDDeviceEvent {}
impl std::convert::TryFrom<NSObject> for DDDeviceEvent {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<DDDeviceEvent, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"DDDeviceEvent").unwrap()) };
        if is_kind_of {
            Ok(DDDeviceEvent(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to DDDeviceEvent")
        }
    }
}
impl IDDDeviceEvent for DDDeviceEvent {}
pub trait IDDDeviceEvent: Sized + std::ops::Deref {
    unsafe fn initWithEventType_device_(&self, type_: DDEventType, device: DDDevice) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithEventType : type_, device : device)
    }
    unsafe fn device(&self) -> DDDevice
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, device)
    }
    unsafe fn eventType(&self) -> DDEventType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, eventType)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct DDDiscoverySession(pub id);
impl std::ops::Deref for DDDiscoverySession {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for DDDiscoverySession {}
impl DDDiscoverySession {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"DDDiscoverySession").unwrap(), alloc) })
    }
}
impl INSObject for DDDiscoverySession {}
impl PNSObject for DDDiscoverySession {}
impl std::convert::TryFrom<NSObject> for DDDiscoverySession {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<DDDiscoverySession, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"DDDiscoverySession").unwrap()) };
        if is_kind_of {
            Ok(DDDiscoverySession(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to DDDiscoverySession")
        }
    }
}
impl IDDDiscoverySession for DDDiscoverySession {}
pub trait IDDDiscoverySession: Sized + std::ops::Deref {
    unsafe fn reportEvent_(&self, inEvent: DDDeviceEvent)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, reportEvent : inEvent)
    }
}
unsafe extern "C" {
    pub static DDDeviceProtocolStringInvalid: DDDeviceProtocolString;
}
unsafe extern "C" {
    pub static DDDeviceProtocolStringDIAL: DDDeviceProtocolString;
}
unsafe extern "C" {
    pub fn DDDeviceProtocolToString(inValue: DDDeviceProtocol) -> NSString;
}
unsafe extern "C" {
    pub fn DDDeviceCategoryToString(inValue: DDDeviceCategory) -> NSString;
}
unsafe extern "C" {
    pub fn DDDeviceStateToString(inValue: DDDeviceState) -> NSString;
}
unsafe extern "C" {
    pub fn DDDeviceMediaPlaybackStateToString(inValue: DDDeviceMediaPlaybackState) -> NSString;
}
unsafe extern "C" {
    pub static DDErrorDomain: NSString;
}
unsafe extern "C" {
    pub fn DDEventTypeToString(inValue: DDEventType) -> NSString;
}

unsafe impl objc2::encode::RefEncode for DDDevice {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for DDDevice {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for DDDeviceEvent {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for DDDeviceEvent {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for DDDiscoverySession {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for DDDiscoverySession {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
