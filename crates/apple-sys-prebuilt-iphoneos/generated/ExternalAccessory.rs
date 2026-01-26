#[allow(unused_imports)]
use crate::objc::*;
#[allow(unused_imports)]
use crate::Foundation::*;
#[allow(unused_imports)]
use crate::Security::*;
#[allow(unused_imports)]
use crate::UIKit::*;

#[allow(unused_imports)]
use objc2::msg_send;
pub type EABluetoothAccessoryPickerErrorCode = NSInteger;
pub type EABluetoothAccessoryPickerCompletion = *mut ::std::os::raw::c_void;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct EAAccessoryManager(pub id);
impl std::ops::Deref for EAAccessoryManager {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for EAAccessoryManager {}
impl EAAccessoryManager {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"EAAccessoryManager").unwrap(), alloc) })
    }
}
impl INSObject for EAAccessoryManager {}
impl PNSObject for EAAccessoryManager {}
impl std::convert::TryFrom<NSObject> for EAAccessoryManager {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<EAAccessoryManager, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"EAAccessoryManager").unwrap()) };
        if is_kind_of {
            Ok(EAAccessoryManager(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to EAAccessoryManager")
        }
    }
}
impl IEAAccessoryManager for EAAccessoryManager {}
pub trait IEAAccessoryManager: Sized + std::ops::Deref {
    unsafe fn showBluetoothAccessoryPickerWithNameFilter_completion_(
        &self,
        predicate: NSPredicate,
        completion: EABluetoothAccessoryPickerCompletion,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, showBluetoothAccessoryPickerWithNameFilter : predicate, completion : completion)
    }
    unsafe fn registerForLocalNotifications(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, registerForLocalNotifications)
    }
    unsafe fn unregisterForLocalNotifications(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, unregisterForLocalNotifications)
    }
    unsafe fn connectedAccessories(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, connectedAccessories)
    }
    unsafe fn sharedAccessoryManager() -> EAAccessoryManager
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"EAAccessoryManager").unwrap(), sharedAccessoryManager)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct EAAccessory(pub id);
impl std::ops::Deref for EAAccessory {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for EAAccessory {}
impl EAAccessory {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"EAAccessory").unwrap(), alloc) })
    }
}
impl INSObject for EAAccessory {}
impl PNSObject for EAAccessory {}
impl std::convert::TryFrom<NSObject> for EAAccessory {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<EAAccessory, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"EAAccessory").unwrap()) };
        if is_kind_of {
            Ok(EAAccessory(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to EAAccessory")
        }
    }
}
impl IEAAccessory for EAAccessory {}
pub trait IEAAccessory: Sized + std::ops::Deref {
    unsafe fn isConnected(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isConnected)
    }
    unsafe fn connectionID(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, connectionID)
    }
    unsafe fn manufacturer(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, manufacturer)
    }
    unsafe fn name(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, name)
    }
    unsafe fn modelNumber(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, modelNumber)
    }
    unsafe fn serialNumber(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, serialNumber)
    }
    unsafe fn firmwareRevision(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, firmwareRevision)
    }
    unsafe fn hardwareRevision(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, hardwareRevision)
    }
    unsafe fn dockType(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, dockType)
    }
    unsafe fn protocolStrings(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, protocolStrings)
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
}
pub trait PEAAccessoryDelegate: Sized + std::ops::Deref {
    unsafe fn accessoryDidDisconnect_(&self, accessory: EAAccessory)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, accessoryDidDisconnect : accessory)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct EASession(pub id);
impl std::ops::Deref for EASession {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for EASession {}
impl EASession {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"EASession").unwrap(), alloc) })
    }
}
impl INSObject for EASession {}
impl PNSObject for EASession {}
impl std::convert::TryFrom<NSObject> for EASession {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<EASession, Self::Error> {
        let is_kind_of: bool = unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"EASession").unwrap()) };
        if is_kind_of {
            Ok(EASession(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to EASession")
        }
    }
}
impl IEASession for EASession {}
pub trait IEASession: Sized + std::ops::Deref {
    unsafe fn initWithAccessory_forProtocol_(
        &self,
        accessory: EAAccessory,
        protocolString: NSString,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithAccessory : accessory, forProtocol : protocolString)
    }
    unsafe fn accessory(&self) -> EAAccessory
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, accessory)
    }
    unsafe fn protocolString(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, protocolString)
    }
    unsafe fn inputStream(&self) -> NSInputStream
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inputStream)
    }
    unsafe fn outputStream(&self) -> NSOutputStream
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, outputStream)
    }
}
pub type EAWiFiUnconfiguredAccessoryBrowserState = NSInteger;
pub type EAWiFiUnconfiguredAccessoryConfigurationStatus = NSInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct EAWiFiUnconfiguredAccessoryBrowser(pub id);
impl std::ops::Deref for EAWiFiUnconfiguredAccessoryBrowser {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for EAWiFiUnconfiguredAccessoryBrowser {}
impl EAWiFiUnconfiguredAccessoryBrowser {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"EAWiFiUnconfiguredAccessoryBrowser").unwrap(), alloc) })
    }
}
impl INSObject for EAWiFiUnconfiguredAccessoryBrowser {}
impl PNSObject for EAWiFiUnconfiguredAccessoryBrowser {}
impl std::convert::TryFrom<NSObject> for EAWiFiUnconfiguredAccessoryBrowser {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<EAWiFiUnconfiguredAccessoryBrowser, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"EAWiFiUnconfiguredAccessoryBrowser").unwrap())
        };
        if is_kind_of {
            Ok(EAWiFiUnconfiguredAccessoryBrowser(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to EAWiFiUnconfiguredAccessoryBrowser")
        }
    }
}
impl IEAWiFiUnconfiguredAccessoryBrowser for EAWiFiUnconfiguredAccessoryBrowser {}
pub trait IEAWiFiUnconfiguredAccessoryBrowser: Sized + std::ops::Deref {
    unsafe fn initWithDelegate_queue_(&self, delegate: *mut u64, queue: NSObject) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithDelegate : delegate, queue : queue)
    }
    unsafe fn startSearchingForUnconfiguredAccessoriesMatchingPredicate_(
        &self,
        predicate: NSPredicate,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, startSearchingForUnconfiguredAccessoriesMatchingPredicate : predicate)
    }
    unsafe fn stopSearchingForUnconfiguredAccessories(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, stopSearchingForUnconfiguredAccessories)
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
    unsafe fn unconfiguredAccessories(&self) -> NSSet
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, unconfiguredAccessories)
    }
}
pub trait PEAWiFiUnconfiguredAccessoryBrowserDelegate: Sized + std::ops::Deref {
    unsafe fn accessoryBrowser_didUpdateState_(
        &self,
        browser: EAWiFiUnconfiguredAccessoryBrowser,
        state: EAWiFiUnconfiguredAccessoryBrowserState,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, accessoryBrowser : browser, didUpdateState : state)
    }
    unsafe fn accessoryBrowser_didFindUnconfiguredAccessories_(
        &self,
        browser: EAWiFiUnconfiguredAccessoryBrowser,
        accessories: NSSet,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, accessoryBrowser : browser, didFindUnconfiguredAccessories : accessories)
    }
    unsafe fn accessoryBrowser_didRemoveUnconfiguredAccessories_(
        &self,
        browser: EAWiFiUnconfiguredAccessoryBrowser,
        accessories: NSSet,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, accessoryBrowser : browser, didRemoveUnconfiguredAccessories : accessories)
    }
    unsafe fn accessoryBrowser_didFinishConfiguringAccessory_withStatus_(
        &self,
        browser: EAWiFiUnconfiguredAccessoryBrowser,
        accessory: EAWiFiUnconfiguredAccessory,
        status: EAWiFiUnconfiguredAccessoryConfigurationStatus,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, accessoryBrowser : browser, didFinishConfiguringAccessory : accessory, withStatus : status)
    }
}
pub type EAWiFiUnconfiguredAccessoryProperties = NSUInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct EAWiFiUnconfiguredAccessory(pub id);
impl std::ops::Deref for EAWiFiUnconfiguredAccessory {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for EAWiFiUnconfiguredAccessory {}
impl EAWiFiUnconfiguredAccessory {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"EAWiFiUnconfiguredAccessory").unwrap(), alloc) })
    }
}
impl INSObject for EAWiFiUnconfiguredAccessory {}
impl PNSObject for EAWiFiUnconfiguredAccessory {}
impl std::convert::TryFrom<NSObject> for EAWiFiUnconfiguredAccessory {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<EAWiFiUnconfiguredAccessory, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"EAWiFiUnconfiguredAccessory").unwrap()) };
        if is_kind_of {
            Ok(EAWiFiUnconfiguredAccessory(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to EAWiFiUnconfiguredAccessory")
        }
    }
}
impl IEAWiFiUnconfiguredAccessory for EAWiFiUnconfiguredAccessory {}
pub trait IEAWiFiUnconfiguredAccessory: Sized + std::ops::Deref {
    unsafe fn name(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, name)
    }
    unsafe fn manufacturer(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, manufacturer)
    }
    unsafe fn model(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, model)
    }
    unsafe fn ssid(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, ssid)
    }
    unsafe fn macAddress(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, macAddress)
    }
    unsafe fn properties(&self) -> EAWiFiUnconfiguredAccessoryProperties
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, properties)
    }
}
unsafe extern "C" {
    pub static EABluetoothAccessoryPickerErrorDomain: NSString;
}
unsafe extern "C" {
    pub static EAAccessoryDidConnectNotification: NSString;
}
unsafe extern "C" {
    pub static EAAccessoryDidDisconnectNotification: NSString;
}
unsafe extern "C" {
    pub static EAAccessoryKey: NSString;
}
unsafe extern "C" {
    pub static EAAccessorySelectedKey: NSString;
}

unsafe impl objc2::encode::RefEncode for EAAccessoryManager {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for EAAccessoryManager {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for EAAccessory {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for EAAccessory {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for EASession {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for EASession {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for EAWiFiUnconfiguredAccessoryBrowser {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for EAWiFiUnconfiguredAccessoryBrowser {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for EAWiFiUnconfiguredAccessory {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for EAWiFiUnconfiguredAccessory {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
