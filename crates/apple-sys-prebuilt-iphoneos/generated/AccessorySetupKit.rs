#[allow(unused_imports)]
use crate::objc::*;
#[allow(unused_imports)]
use crate::CoreBluetooth::*;
#[allow(unused_imports)]
use crate::Foundation::*;
#[allow(unused_imports)]
use crate::Security::*;
#[allow(unused_imports)]
use crate::UIKit::*;

#[allow(unused_imports)]
use objc2::msg_send;
pub type ASBluetoothCompanyIdentifier = u16;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct ASPropertyCompareString(pub id);
impl std::ops::Deref for ASPropertyCompareString {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for ASPropertyCompareString {}
impl ASPropertyCompareString {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"ASPropertyCompareString").unwrap(), alloc) })
    }
}
impl INSObject for ASPropertyCompareString {}
impl PNSObject for ASPropertyCompareString {}
impl std::convert::TryFrom<NSObject> for ASPropertyCompareString {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<ASPropertyCompareString, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"ASPropertyCompareString").unwrap()) };
        if is_kind_of {
            Ok(ASPropertyCompareString(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to ASPropertyCompareString")
        }
    }
}
impl IASPropertyCompareString for ASPropertyCompareString {}
pub trait IASPropertyCompareString: Sized + std::ops::Deref {
    unsafe fn initWithString_compareOptions_(
        &self,
        string: NSString,
        compareOptions: NSStringCompareOptions,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithString : string, compareOptions : compareOptions)
    }
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn new(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, new)
    }
    unsafe fn string(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, string)
    }
    unsafe fn compareOptions(&self) -> NSStringCompareOptions
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, compareOptions)
    }
}
pub type ASErrorCode = NSInteger;
pub type ASAccessoryState = NSInteger;
pub type ASAccessoryRenameOptions = NSUInteger;
pub type ASAccessorySupportOptions = NSUInteger;
pub type ASAccessoryWiFiAwarePairedDeviceID = u64;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct ASAccessory(pub id);
impl std::ops::Deref for ASAccessory {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for ASAccessory {}
impl ASAccessory {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"ASAccessory").unwrap(), alloc) })
    }
}
impl INSObject for ASAccessory {}
impl PNSObject for ASAccessory {}
impl std::convert::TryFrom<NSObject> for ASAccessory {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<ASAccessory, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"ASAccessory").unwrap()) };
        if is_kind_of {
            Ok(ASAccessory(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to ASAccessory")
        }
    }
}
impl IASAccessory for ASAccessory {}
pub trait IASAccessory: Sized + std::ops::Deref {
    unsafe fn state(&self) -> ASAccessoryState
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, state)
    }
    unsafe fn bluetoothIdentifier(&self) -> NSUUID
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, bluetoothIdentifier)
    }
    unsafe fn bluetoothTransportBridgingIdentifier(&self) -> NSData
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, bluetoothTransportBridgingIdentifier)
    }
    unsafe fn displayName(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, displayName)
    }
    unsafe fn SSID(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, SSID)
    }
    unsafe fn wifiAwarePairedDeviceID(&self) -> ASAccessoryWiFiAwarePairedDeviceID
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, wifiAwarePairedDeviceID)
    }
    unsafe fn descriptor(&self) -> ASDiscoveryDescriptor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, descriptor)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct ASDiscoveredAccessory(pub id);
impl std::ops::Deref for ASDiscoveredAccessory {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for ASDiscoveredAccessory {}
impl ASDiscoveredAccessory {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"ASDiscoveredAccessory").unwrap(), alloc) })
    }
}
impl IASAccessory for ASDiscoveredAccessory {}
impl From<ASDiscoveredAccessory> for ASAccessory {
    fn from(child: ASDiscoveredAccessory) -> ASAccessory {
        ASAccessory(child.0)
    }
}
impl std::convert::TryFrom<ASAccessory> for ASDiscoveredAccessory {
    type Error = &'static str;
    fn try_from(parent: ASAccessory) -> Result<ASDiscoveredAccessory, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"ASDiscoveredAccessory").unwrap()) };
        if is_kind_of {
            Ok(ASDiscoveredAccessory(parent.0))
        } else {
            Err("This ASAccessory cannot be downcasted to ASDiscoveredAccessory")
        }
    }
}
impl INSObject for ASDiscoveredAccessory {}
impl PNSObject for ASDiscoveredAccessory {}
impl IASDiscoveredAccessory for ASDiscoveredAccessory {}
pub trait IASDiscoveredAccessory: Sized + std::ops::Deref {
    unsafe fn bluetoothAdvertisementData(&self) -> NSDictionary
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, bluetoothAdvertisementData)
    }
    unsafe fn bluetoothRSSI(&self) -> NSNumber
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, bluetoothRSSI)
    }
}
pub type ASAccessoryEventType = NSInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct ASAccessoryEvent(pub id);
impl std::ops::Deref for ASAccessoryEvent {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for ASAccessoryEvent {}
impl ASAccessoryEvent {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"ASAccessoryEvent").unwrap(), alloc) })
    }
}
impl INSObject for ASAccessoryEvent {}
impl PNSObject for ASAccessoryEvent {}
impl std::convert::TryFrom<NSObject> for ASAccessoryEvent {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<ASAccessoryEvent, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"ASAccessoryEvent").unwrap()) };
        if is_kind_of {
            Ok(ASAccessoryEvent(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to ASAccessoryEvent")
        }
    }
}
impl IASAccessoryEvent for ASAccessoryEvent {}
pub trait IASAccessoryEvent: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn new(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, new)
    }
    unsafe fn eventType(&self) -> ASAccessoryEventType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, eventType)
    }
    unsafe fn accessory(&self) -> ASAccessory
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, accessory)
    }
    unsafe fn error(&self) -> NSError
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, error)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct ASAccessorySession(pub id);
impl std::ops::Deref for ASAccessorySession {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for ASAccessorySession {}
impl ASAccessorySession {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"ASAccessorySession").unwrap(), alloc) })
    }
}
impl INSObject for ASAccessorySession {}
impl PNSObject for ASAccessorySession {}
impl std::convert::TryFrom<NSObject> for ASAccessorySession {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<ASAccessorySession, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"ASAccessorySession").unwrap()) };
        if is_kind_of {
            Ok(ASAccessorySession(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to ASAccessorySession")
        }
    }
}
impl IASAccessorySession for ASAccessorySession {}
pub trait IASAccessorySession: Sized + std::ops::Deref {
    unsafe fn activateWithQueue_eventHandler_(
        &self,
        queue: NSObject,
        eventHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, activateWithQueue : queue, eventHandler : eventHandler)
    }
    unsafe fn invalidate(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, invalidate)
    }
    unsafe fn showPickerWithCompletionHandler_(
        &self,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, showPickerWithCompletionHandler : completionHandler)
    }
    unsafe fn showPickerForDisplayItems_completionHandler_(
        &self,
        displayItems: NSArray,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, showPickerForDisplayItems : displayItems, completionHandler : completionHandler)
    }
    unsafe fn finishAuthorization_settings_completionHandler_(
        &self,
        accessory: ASAccessory,
        settings: ASAccessorySettings,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, finishAuthorization : accessory, settings : settings, completionHandler : completionHandler)
    }
    unsafe fn failAuthorization_completionHandler_(
        &self,
        accessory: ASAccessory,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, failAuthorization : accessory, completionHandler : completionHandler)
    }
    unsafe fn removeAccessory_completionHandler_(
        &self,
        accessory: ASAccessory,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, removeAccessory : accessory, completionHandler : completionHandler)
    }
    unsafe fn renameAccessory_options_completionHandler_(
        &self,
        accessory: ASAccessory,
        renameOptions: ASAccessoryRenameOptions,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, renameAccessory : accessory, options : renameOptions, completionHandler : completionHandler)
    }
    unsafe fn updateAuthorization_descriptor_completionHandler_(
        &self,
        accessory: ASAccessory,
        descriptor: ASDiscoveryDescriptor,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, updateAuthorization : accessory, descriptor : descriptor, completionHandler : completionHandler)
    }
    unsafe fn updatePickerShowingDiscoveredDisplayItems_completionHandler_(
        &self,
        displayItems: NSArray,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, updatePickerShowingDiscoveredDisplayItems : displayItems, completionHandler : completionHandler)
    }
    unsafe fn finishPickerDiscovery_(&self, completionHandler: *mut ::std::os::raw::c_void)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, finishPickerDiscovery : completionHandler)
    }
    unsafe fn accessories(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, accessories)
    }
    unsafe fn pickerDisplaySettings(&self) -> ASPickerDisplaySettings
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, pickerDisplaySettings)
    }
    unsafe fn setPickerDisplaySettings_(&self, pickerDisplaySettings: ASPickerDisplaySettings)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPickerDisplaySettings : pickerDisplaySettings)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct ASAccessorySettings(pub id);
impl std::ops::Deref for ASAccessorySettings {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for ASAccessorySettings {}
impl ASAccessorySettings {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"ASAccessorySettings").unwrap(), alloc) })
    }
}
impl INSObject for ASAccessorySettings {}
impl PNSObject for ASAccessorySettings {}
impl std::convert::TryFrom<NSObject> for ASAccessorySettings {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<ASAccessorySettings, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"ASAccessorySettings").unwrap()) };
        if is_kind_of {
            Ok(ASAccessorySettings(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to ASAccessorySettings")
        }
    }
}
impl IASAccessorySettings for ASAccessorySettings {}
pub trait IASAccessorySettings: Sized + std::ops::Deref {
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
    unsafe fn bluetoothTransportBridgingIdentifier(&self) -> NSData
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, bluetoothTransportBridgingIdentifier)
    }
    unsafe fn setBluetoothTransportBridgingIdentifier_(
        &self,
        bluetoothTransportBridgingIdentifier: NSData,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setBluetoothTransportBridgingIdentifier : bluetoothTransportBridgingIdentifier)
    }
    unsafe fn defaultSettings() -> ASAccessorySettings
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"ASAccessorySettings").unwrap(), defaultSettings)
    }
}
pub type ASDiscoveryDescriptorRange = NSInteger;
pub type ASDiscoveryDescriptorWiFiAwareServiceRole = NSInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct ASDiscoveryDescriptor(pub id);
impl std::ops::Deref for ASDiscoveryDescriptor {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for ASDiscoveryDescriptor {}
impl ASDiscoveryDescriptor {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"ASDiscoveryDescriptor").unwrap(), alloc) })
    }
}
impl INSObject for ASDiscoveryDescriptor {}
impl PNSObject for ASDiscoveryDescriptor {}
impl std::convert::TryFrom<NSObject> for ASDiscoveryDescriptor {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<ASDiscoveryDescriptor, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"ASDiscoveryDescriptor").unwrap()) };
        if is_kind_of {
            Ok(ASDiscoveryDescriptor(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to ASDiscoveryDescriptor")
        }
    }
}
impl IASDiscoveryDescriptor for ASDiscoveryDescriptor {}
pub trait IASDiscoveryDescriptor: Sized + std::ops::Deref {
    unsafe fn supportedOptions(&self) -> ASAccessorySupportOptions
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, supportedOptions)
    }
    unsafe fn setSupportedOptions_(&self, supportedOptions: ASAccessorySupportOptions)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSupportedOptions : supportedOptions)
    }
    unsafe fn bluetoothCompanyIdentifier(&self) -> ASBluetoothCompanyIdentifier
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, bluetoothCompanyIdentifier)
    }
    unsafe fn setBluetoothCompanyIdentifier_(
        &self,
        bluetoothCompanyIdentifier: ASBluetoothCompanyIdentifier,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setBluetoothCompanyIdentifier : bluetoothCompanyIdentifier)
    }
    unsafe fn bluetoothManufacturerDataBlob(&self) -> NSData
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, bluetoothManufacturerDataBlob)
    }
    unsafe fn setBluetoothManufacturerDataBlob_(&self, bluetoothManufacturerDataBlob: NSData)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setBluetoothManufacturerDataBlob : bluetoothManufacturerDataBlob)
    }
    unsafe fn bluetoothManufacturerDataMask(&self) -> NSData
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, bluetoothManufacturerDataMask)
    }
    unsafe fn setBluetoothManufacturerDataMask_(&self, bluetoothManufacturerDataMask: NSData)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setBluetoothManufacturerDataMask : bluetoothManufacturerDataMask)
    }
    unsafe fn bluetoothNameSubstringCompareOptions(&self) -> NSStringCompareOptions
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, bluetoothNameSubstringCompareOptions)
    }
    unsafe fn setBluetoothNameSubstringCompareOptions_(
        &self,
        bluetoothNameSubstringCompareOptions: NSStringCompareOptions,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setBluetoothNameSubstringCompareOptions : bluetoothNameSubstringCompareOptions)
    }
    unsafe fn bluetoothNameSubstring(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, bluetoothNameSubstring)
    }
    unsafe fn setBluetoothNameSubstring_(&self, bluetoothNameSubstring: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setBluetoothNameSubstring : bluetoothNameSubstring)
    }
    unsafe fn bluetoothRange(&self) -> ASDiscoveryDescriptorRange
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, bluetoothRange)
    }
    unsafe fn setBluetoothRange_(&self, bluetoothRange: ASDiscoveryDescriptorRange)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setBluetoothRange : bluetoothRange)
    }
    unsafe fn bluetoothServiceDataBlob(&self) -> NSData
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, bluetoothServiceDataBlob)
    }
    unsafe fn setBluetoothServiceDataBlob_(&self, bluetoothServiceDataBlob: NSData)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setBluetoothServiceDataBlob : bluetoothServiceDataBlob)
    }
    unsafe fn bluetoothServiceDataMask(&self) -> NSData
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, bluetoothServiceDataMask)
    }
    unsafe fn setBluetoothServiceDataMask_(&self, bluetoothServiceDataMask: NSData)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setBluetoothServiceDataMask : bluetoothServiceDataMask)
    }
    unsafe fn bluetoothServiceUUID(&self) -> CBUUID
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, bluetoothServiceUUID)
    }
    unsafe fn setBluetoothServiceUUID_(&self, bluetoothServiceUUID: CBUUID)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setBluetoothServiceUUID : bluetoothServiceUUID)
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
    unsafe fn SSIDPrefix(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, SSIDPrefix)
    }
    unsafe fn setSSIDPrefix_(&self, SSIDPrefix: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSSIDPrefix : SSIDPrefix)
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
    unsafe fn wifiAwareServiceRole(&self) -> ASDiscoveryDescriptorWiFiAwareServiceRole
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, wifiAwareServiceRole)
    }
    unsafe fn setWifiAwareServiceRole_(
        &self,
        wifiAwareServiceRole: ASDiscoveryDescriptorWiFiAwareServiceRole,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setWifiAwareServiceRole : wifiAwareServiceRole)
    }
    unsafe fn wifiAwareModelNameMatch(&self) -> ASPropertyCompareString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, wifiAwareModelNameMatch)
    }
    unsafe fn setWifiAwareModelNameMatch_(&self, wifiAwareModelNameMatch: ASPropertyCompareString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setWifiAwareModelNameMatch : wifiAwareModelNameMatch)
    }
    unsafe fn wifiAwareVendorNameMatch(&self) -> ASPropertyCompareString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, wifiAwareVendorNameMatch)
    }
    unsafe fn setWifiAwareVendorNameMatch_(&self, wifiAwareVendorNameMatch: ASPropertyCompareString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setWifiAwareVendorNameMatch : wifiAwareVendorNameMatch)
    }
}
pub type ASPickerDisplayItemSetupOptions = NSUInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct ASPickerDisplayItem(pub id);
impl std::ops::Deref for ASPickerDisplayItem {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for ASPickerDisplayItem {}
impl ASPickerDisplayItem {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"ASPickerDisplayItem").unwrap(), alloc) })
    }
}
impl INSObject for ASPickerDisplayItem {}
impl PNSObject for ASPickerDisplayItem {}
impl std::convert::TryFrom<NSObject> for ASPickerDisplayItem {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<ASPickerDisplayItem, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"ASPickerDisplayItem").unwrap()) };
        if is_kind_of {
            Ok(ASPickerDisplayItem(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to ASPickerDisplayItem")
        }
    }
}
impl IASPickerDisplayItem for ASPickerDisplayItem {}
pub trait IASPickerDisplayItem: Sized + std::ops::Deref {
    unsafe fn initWithName_productImage_descriptor_(
        &self,
        name: NSString,
        productImage: UIImage,
        descriptor: ASDiscoveryDescriptor,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithName : name, productImage : productImage, descriptor : descriptor)
    }
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn new(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, new)
    }
    unsafe fn name(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, name)
    }
    unsafe fn productImage(&self) -> UIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, productImage)
    }
    unsafe fn descriptor(&self) -> ASDiscoveryDescriptor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, descriptor)
    }
    unsafe fn renameOptions(&self) -> ASAccessoryRenameOptions
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, renameOptions)
    }
    unsafe fn setRenameOptions_(&self, renameOptions: ASAccessoryRenameOptions)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRenameOptions : renameOptions)
    }
    unsafe fn setupOptions(&self) -> ASPickerDisplayItemSetupOptions
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, setupOptions)
    }
    unsafe fn setSetupOptions_(&self, setupOptions: ASPickerDisplayItemSetupOptions)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSetupOptions : setupOptions)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct ASMigrationDisplayItem(pub id);
impl std::ops::Deref for ASMigrationDisplayItem {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for ASMigrationDisplayItem {}
impl ASMigrationDisplayItem {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"ASMigrationDisplayItem").unwrap(), alloc) })
    }
}
impl IASPickerDisplayItem for ASMigrationDisplayItem {}
impl From<ASMigrationDisplayItem> for ASPickerDisplayItem {
    fn from(child: ASMigrationDisplayItem) -> ASPickerDisplayItem {
        ASPickerDisplayItem(child.0)
    }
}
impl std::convert::TryFrom<ASPickerDisplayItem> for ASMigrationDisplayItem {
    type Error = &'static str;
    fn try_from(parent: ASPickerDisplayItem) -> Result<ASMigrationDisplayItem, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"ASMigrationDisplayItem").unwrap()) };
        if is_kind_of {
            Ok(ASMigrationDisplayItem(parent.0))
        } else {
            Err("This ASPickerDisplayItem cannot be downcasted to ASMigrationDisplayItem")
        }
    }
}
impl INSObject for ASMigrationDisplayItem {}
impl PNSObject for ASMigrationDisplayItem {}
impl IASMigrationDisplayItem for ASMigrationDisplayItem {}
pub trait IASMigrationDisplayItem: Sized + std::ops::Deref {
    unsafe fn peripheralIdentifier(&self) -> NSUUID
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, peripheralIdentifier)
    }
    unsafe fn setPeripheralIdentifier_(&self, peripheralIdentifier: NSUUID)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPeripheralIdentifier : peripheralIdentifier)
    }
    unsafe fn hotspotSSID(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, hotspotSSID)
    }
    unsafe fn setHotspotSSID_(&self, hotspotSSID: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setHotspotSSID : hotspotSSID)
    }
    unsafe fn wifiAwarePairedDeviceID(&self) -> ASAccessoryWiFiAwarePairedDeviceID
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, wifiAwarePairedDeviceID)
    }
    unsafe fn setWifiAwarePairedDeviceID_(
        &self,
        wifiAwarePairedDeviceID: ASAccessoryWiFiAwarePairedDeviceID,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setWifiAwarePairedDeviceID : wifiAwarePairedDeviceID)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct ASDiscoveredDisplayItem(pub id);
impl std::ops::Deref for ASDiscoveredDisplayItem {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for ASDiscoveredDisplayItem {}
impl ASDiscoveredDisplayItem {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"ASDiscoveredDisplayItem").unwrap(), alloc) })
    }
}
impl IASPickerDisplayItem for ASDiscoveredDisplayItem {}
impl std::convert::TryFrom<ASPickerDisplayItem> for ASDiscoveredDisplayItem {
    type Error = &'static str;
    fn try_from(parent: ASPickerDisplayItem) -> Result<ASDiscoveredDisplayItem, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"ASDiscoveredDisplayItem").unwrap()) };
        if is_kind_of {
            Ok(ASDiscoveredDisplayItem(parent.0))
        } else {
            Err("This ASPickerDisplayItem cannot be downcasted to ASDiscoveredDisplayItem")
        }
    }
}
impl INSObject for ASDiscoveredDisplayItem {}
impl PNSObject for ASDiscoveredDisplayItem {}
impl IASDiscoveredDisplayItem for ASDiscoveredDisplayItem {}
pub trait IASDiscoveredDisplayItem: Sized + std::ops::Deref {
    unsafe fn initWithName_productImage_accessory_(
        &self,
        name: NSString,
        productImage: UIImage,
        accessory: ASDiscoveredAccessory,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithName : name, productImage : productImage, accessory : accessory)
    }
    unsafe fn initWithName_productImage_descriptor_(
        &self,
        name: NSString,
        productImage: UIImage,
        descriptor: ASDiscoveryDescriptor,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithName : name, productImage : productImage, descriptor : descriptor)
    }
}
pub type ASPickerDisplaySettingsOptions = NSUInteger;
pub type ASPickerDisplaySettingsDiscoveryTimeout = NSTimeInterval;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct ASPickerDisplaySettings(pub id);
impl std::ops::Deref for ASPickerDisplaySettings {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for ASPickerDisplaySettings {}
impl ASPickerDisplaySettings {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"ASPickerDisplaySettings").unwrap(), alloc) })
    }
}
impl INSObject for ASPickerDisplaySettings {}
impl PNSObject for ASPickerDisplaySettings {}
impl std::convert::TryFrom<NSObject> for ASPickerDisplaySettings {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<ASPickerDisplaySettings, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"ASPickerDisplaySettings").unwrap()) };
        if is_kind_of {
            Ok(ASPickerDisplaySettings(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to ASPickerDisplaySettings")
        }
    }
}
impl IASPickerDisplaySettings for ASPickerDisplaySettings {}
pub trait IASPickerDisplaySettings: Sized + std::ops::Deref {
    unsafe fn discoveryTimeout(&self) -> ASPickerDisplaySettingsDiscoveryTimeout
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, discoveryTimeout)
    }
    unsafe fn setDiscoveryTimeout_(&self, discoveryTimeout: ASPickerDisplaySettingsDiscoveryTimeout)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDiscoveryTimeout : discoveryTimeout)
    }
    unsafe fn options(&self) -> ASPickerDisplaySettingsOptions
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, options)
    }
    unsafe fn setOptions_(&self, options: ASPickerDisplaySettingsOptions)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setOptions : options)
    }
    unsafe fn defaultSettings() -> ASPickerDisplaySettings
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"ASPickerDisplaySettings").unwrap(), defaultSettings)
    }
}
unsafe extern "C" {
    pub static ASErrorDomain: NSErrorDomain;
}
unsafe extern "C" {
    pub static ASPickerDisplaySettingsDiscoveryTimeoutShort:
        ASPickerDisplaySettingsDiscoveryTimeout;
}
unsafe extern "C" {
    pub static ASPickerDisplaySettingsDiscoveryTimeoutMedium:
        ASPickerDisplaySettingsDiscoveryTimeout;
}
unsafe extern "C" {
    pub static ASPickerDisplaySettingsDiscoveryTimeoutLong: ASPickerDisplaySettingsDiscoveryTimeout;
}
unsafe extern "C" {
    pub static ASPickerDisplaySettingsDiscoveryTimeoutUnbounded:
        ASPickerDisplaySettingsDiscoveryTimeout;
}

unsafe impl objc2::encode::RefEncode for ASPropertyCompareString {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for ASPropertyCompareString {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for ASAccessory {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for ASAccessory {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for ASDiscoveredAccessory {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for ASDiscoveredAccessory {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for ASAccessoryEvent {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for ASAccessoryEvent {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for ASAccessorySession {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for ASAccessorySession {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for ASAccessorySettings {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for ASAccessorySettings {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for ASDiscoveryDescriptor {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for ASDiscoveryDescriptor {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for ASPickerDisplayItem {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for ASPickerDisplayItem {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for ASMigrationDisplayItem {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for ASMigrationDisplayItem {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for ASDiscoveredDisplayItem {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for ASDiscoveredDisplayItem {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for ASPickerDisplaySettings {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for ASPickerDisplaySettings {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
