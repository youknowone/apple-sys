#[allow(unused_imports)]
use crate::objc::*;
#[allow(unused_imports)]
use crate::CoreFoundation::*;
#[allow(unused_imports)]
use crate::Foundation::*;
#[allow(unused_imports)]
use crate::Network::*;

#[allow(unused_imports)]
use objc2::msg_send;
pub type AVB17221ADPEntityCapabilities = u32;
pub type AVB17221ADPTalkerCapabilities = u16;
pub type AVB17221ADPListenerCapabilities = u16;
pub type AVB17221ADPControllerCapabilities = u32;
pub type AVB17221AECPMessageType = u8;
pub type AVB17221AECPStatusCode = u8;
pub type AVB17221ACMPMessageType = u8;
pub type AVB17221ACMPStatusCode = u8;
pub type AVB17221ACMPFlags = u16;
pub type AVB17221ACMPIPFlag = u16;
pub type AVB17221AEMCommandType = u16;
pub type AVB17221AECPAddressAccessTLVMode = u8;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct AVBCentralManager(pub id);
impl std::ops::Deref for AVBCentralManager {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for AVBCentralManager {}
impl AVBCentralManager {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"AVBCentralManager").unwrap(), alloc) })
    }
}
impl INSObject for AVBCentralManager {}
impl PNSObject for AVBCentralManager {}
impl std::convert::TryFrom<NSObject> for AVBCentralManager {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<AVBCentralManager, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"AVBCentralManager").unwrap()) };
        if is_kind_of {
            Ok(AVBCentralManager(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to AVBCentralManager")
        }
    }
}
impl IAVBCentralManager for AVBCentralManager {}
pub trait IAVBCentralManager: Sized + std::ops::Deref {
    unsafe fn startControllerMatching(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, startControllerMatching)
    }
    unsafe fn didAddInterface_(&self, interface: AVBInterface)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, didAddInterface : interface)
    }
    unsafe fn didRemoveInterface_(&self, interface: AVBInterface)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, didRemoveInterface : interface)
    }
    unsafe fn streamingEnabledInterfacesOnly(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, streamingEnabledInterfacesOnly)
    }
    unsafe fn nextAvailableDynamicEntityID() -> u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"AVBCentralManager").unwrap(), nextAvailableDynamicEntityID)
    }
    unsafe fn releaseDynamicEntityID_(entityID: u64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"AVBCentralManager").unwrap(), releaseDynamicEntityID : entityID)
    }
    unsafe fn nextAvailableDynamicEntityModelID() -> u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"AVBCentralManager").unwrap(), nextAvailableDynamicEntityModelID)
    }
    unsafe fn releaseDynamicEntityModelID_(entityModelID: u64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"AVBCentralManager").unwrap(), releaseDynamicEntityModelID : entityModelID)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct AVBInterface(pub id);
impl std::ops::Deref for AVBInterface {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for AVBInterface {}
impl AVBInterface {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"AVBInterface").unwrap(), alloc) })
    }
}
impl INSObject for AVBInterface {}
impl PNSObject for AVBInterface {}
impl std::convert::TryFrom<NSObject> for AVBInterface {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<AVBInterface, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"AVBInterface").unwrap()) };
        if is_kind_of {
            Ok(AVBInterface(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to AVBInterface")
        }
    }
}
impl IAVBInterface for AVBInterface {}
pub trait IAVBInterface: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn initWithInterfaceName_(&self, anInterfaceName: NSString) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithInterfaceName : anInterfaceName)
    }
    unsafe fn interfaceName(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, interfaceName)
    }
    unsafe fn entityDiscovery(&self) -> AVB17221EntityDiscovery
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, entityDiscovery)
    }
    unsafe fn aecp(&self) -> AVB17221AECPInterface
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, aecp)
    }
    unsafe fn acmp(&self) -> AVB17221ACMPInterface
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, acmp)
    }
    unsafe fn macAddressForInterfaceNamed_(anInterfaceName: NSString) -> AVBMACAddress
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"AVBInterface").unwrap(), macAddressForInterfaceNamed : anInterfaceName)
    }
    unsafe fn supportedInterfaces() -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"AVBInterface").unwrap(), supportedInterfaces)
    }
    unsafe fn isAVBEnabledOnInterfaceNamed_(anInterfaceName: NSString) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"AVBInterface").unwrap(), isAVBEnabledOnInterfaceNamed : anInterfaceName)
    }
    unsafe fn isAVBCapableInterfaceNamed_(anInterfaceName: NSString) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"AVBInterface").unwrap(), isAVBCapableInterfaceNamed : anInterfaceName)
    }
    unsafe fn myEntityID() -> u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"AVBInterface").unwrap(), myEntityID)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct AVBEthernetInterface(pub id);
impl std::ops::Deref for AVBEthernetInterface {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for AVBEthernetInterface {}
impl AVBEthernetInterface {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"AVBEthernetInterface").unwrap(), alloc) })
    }
}
impl IAVBInterface for AVBEthernetInterface {}
impl From<AVBEthernetInterface> for AVBInterface {
    fn from(child: AVBEthernetInterface) -> AVBInterface {
        AVBInterface(child.0)
    }
}
impl std::convert::TryFrom<AVBInterface> for AVBEthernetInterface {
    type Error = &'static str;
    fn try_from(parent: AVBInterface) -> Result<AVBEthernetInterface, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"AVBEthernetInterface").unwrap()) };
        if is_kind_of {
            Ok(AVBEthernetInterface(parent.0))
        } else {
            Err("This AVBInterface cannot be downcasted to AVBEthernetInterface")
        }
    }
}
impl INSObject for AVBEthernetInterface {}
impl PNSObject for AVBEthernetInterface {}
impl IAVBEthernetInterface for AVBEthernetInterface {}
pub trait IAVBEthernetInterface: Sized + std::ops::Deref {}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct AVBMACAddress(pub id);
impl std::ops::Deref for AVBMACAddress {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for AVBMACAddress {}
impl AVBMACAddress {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"AVBMACAddress").unwrap(), alloc) })
    }
}
impl PNSCopying for AVBMACAddress {}
impl INSObject for AVBMACAddress {}
impl PNSObject for AVBMACAddress {}
impl std::convert::TryFrom<NSObject> for AVBMACAddress {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<AVBMACAddress, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"AVBMACAddress").unwrap()) };
        if is_kind_of {
            Ok(AVBMACAddress(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to AVBMACAddress")
        }
    }
}
impl IAVBMACAddress for AVBMACAddress {}
pub trait IAVBMACAddress: Sized + std::ops::Deref {
    unsafe fn initWithBytes_(&self, bytes: *const u8) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithBytes : bytes)
    }
    unsafe fn bytes(&self) -> *const u8
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, bytes)
    }
    unsafe fn dataRepresentation(&self) -> NSData
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, dataRepresentation)
    }
    unsafe fn setDataRepresentation_(&self, dataRepresentation: NSData)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDataRepresentation : dataRepresentation)
    }
    unsafe fn stringRepresentation(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, stringRepresentation)
    }
    unsafe fn setStringRepresentation_(&self, stringRepresentation: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setStringRepresentation : stringRepresentation)
    }
    unsafe fn isMulticast(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isMulticast)
    }
    unsafe fn setMulticast_(&self, multicast: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMulticast : multicast)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct AVBIPAddress(pub id);
impl std::ops::Deref for AVBIPAddress {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for AVBIPAddress {}
impl AVBIPAddress {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"AVBIPAddress").unwrap(), alloc) })
    }
}
impl PNSCopying for AVBIPAddress {}
impl INSObject for AVBIPAddress {}
impl PNSObject for AVBIPAddress {}
impl std::convert::TryFrom<NSObject> for AVBIPAddress {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<AVBIPAddress, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"AVBIPAddress").unwrap()) };
        if is_kind_of {
            Ok(AVBIPAddress(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to AVBIPAddress")
        }
    }
}
impl IAVBIPAddress for AVBIPAddress {}
pub trait IAVBIPAddress: Sized + std::ops::Deref {
    unsafe fn initWithIPv6Address_(&self, ipv6Address: *const u8) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithIPv6Address : ipv6Address)
    }
    unsafe fn initWithIPv6AddressData_(&self, ipv6Address: NSData) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithIPv6AddressData : ipv6Address)
    }
    unsafe fn initWithIPv4Address_(&self, ipv4Address: u32) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithIPv4Address : ipv4Address)
    }
    unsafe fn representsIPv4Address(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, representsIPv4Address)
    }
    unsafe fn ipv6Address(&self) -> NSData
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, ipv6Address)
    }
    unsafe fn setIpv6Address_(&self, ipv6Address: NSData)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setIpv6Address : ipv6Address)
    }
    unsafe fn ipv4Address(&self) -> u32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, ipv4Address)
    }
    unsafe fn setIpv4Address_(&self, ipv4Address: u32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setIpv4Address : ipv4Address)
    }
    unsafe fn stringRepresentation(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, stringRepresentation)
    }
    unsafe fn setStringRepresentation_(&self, stringRepresentation: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setStringRepresentation : stringRepresentation)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct AVB1722ControlInterface(pub id);
impl std::ops::Deref for AVB1722ControlInterface {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for AVB1722ControlInterface {}
impl AVB1722ControlInterface {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"AVB1722ControlInterface").unwrap(), alloc) })
    }
}
impl INSObject for AVB1722ControlInterface {}
impl PNSObject for AVB1722ControlInterface {}
impl std::convert::TryFrom<NSObject> for AVB1722ControlInterface {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<AVB1722ControlInterface, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"AVB1722ControlInterface").unwrap()) };
        if is_kind_of {
            Ok(AVB1722ControlInterface(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to AVB1722ControlInterface")
        }
    }
}
impl IAVB1722ControlInterface for AVB1722ControlInterface {}
pub trait IAVB1722ControlInterface: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn initWithInterfaceName_(&self, anInterfaceName: NSString) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithInterfaceName : anInterfaceName)
    }
    unsafe fn initWithInterface_(&self, anInterface: AVBInterface) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithInterface : anInterface)
    }
    unsafe fn interfaceName(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, interfaceName)
    }
    unsafe fn interface(&self) -> AVBInterface
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, interface)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct AVB17221AECPMessage(pub id);
impl std::ops::Deref for AVB17221AECPMessage {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for AVB17221AECPMessage {}
impl AVB17221AECPMessage {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"AVB17221AECPMessage").unwrap(), alloc) })
    }
}
impl PNSCopying for AVB17221AECPMessage {}
impl INSObject for AVB17221AECPMessage {}
impl PNSObject for AVB17221AECPMessage {}
impl std::convert::TryFrom<NSObject> for AVB17221AECPMessage {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<AVB17221AECPMessage, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"AVB17221AECPMessage").unwrap()) };
        if is_kind_of {
            Ok(AVB17221AECPMessage(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to AVB17221AECPMessage")
        }
    }
}
impl IAVB17221AECPMessage for AVB17221AECPMessage {}
pub trait IAVB17221AECPMessage: Sized + std::ops::Deref {
    unsafe fn errorForStatusCode(&self) -> NSError
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, errorForStatusCode)
    }
    unsafe fn messageType(&self) -> AVB17221AECPMessageType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, messageType)
    }
    unsafe fn setMessageType_(&self, messageType: AVB17221AECPMessageType)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMessageType : messageType)
    }
    unsafe fn status(&self) -> AVB17221AECPStatusCode
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, status)
    }
    unsafe fn setStatus_(&self, status: AVB17221AECPStatusCode)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setStatus : status)
    }
    unsafe fn targetEntityID(&self) -> u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, targetEntityID)
    }
    unsafe fn setTargetEntityID_(&self, targetEntityID: u64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setTargetEntityID : targetEntityID)
    }
    unsafe fn controllerEntityID(&self) -> u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, controllerEntityID)
    }
    unsafe fn setControllerEntityID_(&self, controllerEntityID: u64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setControllerEntityID : controllerEntityID)
    }
    unsafe fn sequenceID(&self) -> u16
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, sequenceID)
    }
    unsafe fn setSequenceID_(&self, sequenceID: u16)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSequenceID : sequenceID)
    }
    unsafe fn sourceMAC(&self) -> AVBMACAddress
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, sourceMAC)
    }
    unsafe fn setSourceMAC_(&self, sourceMAC: AVBMACAddress)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSourceMAC : sourceMAC)
    }
    unsafe fn errorForStatusCode_(statusCode: AVB17221AECPStatusCode) -> NSError
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"AVB17221AECPMessage").unwrap(), errorForStatusCode : statusCode)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct AVB17221AECPAEMMessage(pub id);
impl std::ops::Deref for AVB17221AECPAEMMessage {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for AVB17221AECPAEMMessage {}
impl AVB17221AECPAEMMessage {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"AVB17221AECPAEMMessage").unwrap(), alloc) })
    }
}
impl IAVB17221AECPMessage for AVB17221AECPAEMMessage {}
impl PNSCopying for AVB17221AECPAEMMessage {}
impl From<AVB17221AECPAEMMessage> for AVB17221AECPMessage {
    fn from(child: AVB17221AECPAEMMessage) -> AVB17221AECPMessage {
        AVB17221AECPMessage(child.0)
    }
}
impl std::convert::TryFrom<AVB17221AECPMessage> for AVB17221AECPAEMMessage {
    type Error = &'static str;
    fn try_from(parent: AVB17221AECPMessage) -> Result<AVB17221AECPAEMMessage, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"AVB17221AECPAEMMessage").unwrap()) };
        if is_kind_of {
            Ok(AVB17221AECPAEMMessage(parent.0))
        } else {
            Err("This AVB17221AECPMessage cannot be downcasted to AVB17221AECPAEMMessage")
        }
    }
}
impl INSObject for AVB17221AECPAEMMessage {}
impl PNSObject for AVB17221AECPAEMMessage {}
impl IAVB17221AECPAEMMessage for AVB17221AECPAEMMessage {}
pub trait IAVB17221AECPAEMMessage: Sized + std::ops::Deref {
    unsafe fn commandType(&self) -> AVB17221AEMCommandType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, commandType)
    }
    unsafe fn setCommandType_(&self, commandType: AVB17221AEMCommandType)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCommandType : commandType)
    }
    unsafe fn isUnsolicited(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isUnsolicited)
    }
    unsafe fn setUnsolicited_(&self, unsolicited: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setUnsolicited : unsolicited)
    }
    unsafe fn isControllerRequest(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isControllerRequest)
    }
    unsafe fn setControllerRequest_(&self, controllerRequest: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setControllerRequest : controllerRequest)
    }
    unsafe fn commandSpecificData(&self) -> NSData
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, commandSpecificData)
    }
    unsafe fn setCommandSpecificData_(&self, commandSpecificData: NSData)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCommandSpecificData : commandSpecificData)
    }
    unsafe fn commandMessage() -> AVB17221AECPAEMMessage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"AVB17221AECPAEMMessage").unwrap(), commandMessage)
    }
    unsafe fn responseMessage() -> AVB17221AECPAEMMessage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"AVB17221AECPAEMMessage").unwrap(), responseMessage)
    }
    unsafe fn responseMessageFromCommandMessage_(
        commandMessage: AVB17221AECPAEMMessage,
    ) -> AVB17221AECPAEMMessage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"AVB17221AECPAEMMessage").unwrap(), responseMessageFromCommandMessage : commandMessage)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct AVB17221AECPAddressAccessMessage(pub id);
impl std::ops::Deref for AVB17221AECPAddressAccessMessage {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for AVB17221AECPAddressAccessMessage {}
impl AVB17221AECPAddressAccessMessage {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"AVB17221AECPAddressAccessMessage").unwrap(), alloc) })
    }
}
impl IAVB17221AECPMessage for AVB17221AECPAddressAccessMessage {}
impl PNSCopying for AVB17221AECPAddressAccessMessage {}
impl std::convert::TryFrom<AVB17221AECPMessage> for AVB17221AECPAddressAccessMessage {
    type Error = &'static str;
    fn try_from(
        parent: AVB17221AECPMessage,
    ) -> Result<AVB17221AECPAddressAccessMessage, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"AVB17221AECPAddressAccessMessage").unwrap())
        };
        if is_kind_of {
            Ok(AVB17221AECPAddressAccessMessage(parent.0))
        } else {
            Err("This AVB17221AECPMessage cannot be downcasted to AVB17221AECPAddressAccessMessage")
        }
    }
}
impl INSObject for AVB17221AECPAddressAccessMessage {}
impl PNSObject for AVB17221AECPAddressAccessMessage {}
impl IAVB17221AECPAddressAccessMessage for AVB17221AECPAddressAccessMessage {}
pub trait IAVB17221AECPAddressAccessMessage: Sized + std::ops::Deref {
    unsafe fn tlvs(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, tlvs)
    }
    unsafe fn setTlvs_(&self, tlvs: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setTlvs : tlvs)
    }
    unsafe fn commandMessage() -> AVB17221AECPAddressAccessMessage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"AVB17221AECPAddressAccessMessage").unwrap(), commandMessage)
    }
    unsafe fn responseMessage() -> AVB17221AECPAddressAccessMessage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"AVB17221AECPAddressAccessMessage").unwrap(), responseMessage)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct AVB17221AECPAddressAccessTLV(pub id);
impl std::ops::Deref for AVB17221AECPAddressAccessTLV {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for AVB17221AECPAddressAccessTLV {}
impl AVB17221AECPAddressAccessTLV {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"AVB17221AECPAddressAccessTLV").unwrap(), alloc) })
    }
}
impl INSObject for AVB17221AECPAddressAccessTLV {}
impl PNSObject for AVB17221AECPAddressAccessTLV {}
impl std::convert::TryFrom<NSObject> for AVB17221AECPAddressAccessTLV {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<AVB17221AECPAddressAccessTLV, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"AVB17221AECPAddressAccessTLV").unwrap()) };
        if is_kind_of {
            Ok(AVB17221AECPAddressAccessTLV(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to AVB17221AECPAddressAccessTLV")
        }
    }
}
impl IAVB17221AECPAddressAccessTLV for AVB17221AECPAddressAccessTLV {}
pub trait IAVB17221AECPAddressAccessTLV: Sized + std::ops::Deref {
    unsafe fn mode(&self) -> AVB17221AECPAddressAccessTLVMode
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, mode)
    }
    unsafe fn setMode_(&self, mode: AVB17221AECPAddressAccessTLVMode)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMode : mode)
    }
    unsafe fn address(&self) -> u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, address)
    }
    unsafe fn setAddress_(&self, address: u64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAddress : address)
    }
    unsafe fn memoryData(&self) -> NSData
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, memoryData)
    }
    unsafe fn setMemoryData_(&self, memoryData: NSData)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMemoryData : memoryData)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct AVB17221AECPAVCMessage(pub id);
impl std::ops::Deref for AVB17221AECPAVCMessage {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for AVB17221AECPAVCMessage {}
impl AVB17221AECPAVCMessage {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"AVB17221AECPAVCMessage").unwrap(), alloc) })
    }
}
impl IAVB17221AECPMessage for AVB17221AECPAVCMessage {}
impl PNSCopying for AVB17221AECPAVCMessage {}
impl std::convert::TryFrom<AVB17221AECPMessage> for AVB17221AECPAVCMessage {
    type Error = &'static str;
    fn try_from(parent: AVB17221AECPMessage) -> Result<AVB17221AECPAVCMessage, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"AVB17221AECPAVCMessage").unwrap()) };
        if is_kind_of {
            Ok(AVB17221AECPAVCMessage(parent.0))
        } else {
            Err("This AVB17221AECPMessage cannot be downcasted to AVB17221AECPAVCMessage")
        }
    }
}
impl INSObject for AVB17221AECPAVCMessage {}
impl PNSObject for AVB17221AECPAVCMessage {}
impl IAVB17221AECPAVCMessage for AVB17221AECPAVCMessage {}
pub trait IAVB17221AECPAVCMessage: Sized + std::ops::Deref {
    unsafe fn commandResponse(&self) -> NSData
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, commandResponse)
    }
    unsafe fn setCommandResponse_(&self, commandResponse: NSData)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCommandResponse : commandResponse)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct AVB17221AECPVendorMessage(pub id);
impl std::ops::Deref for AVB17221AECPVendorMessage {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for AVB17221AECPVendorMessage {}
impl AVB17221AECPVendorMessage {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"AVB17221AECPVendorMessage").unwrap(), alloc) })
    }
}
impl IAVB17221AECPMessage for AVB17221AECPVendorMessage {}
impl PNSCopying for AVB17221AECPVendorMessage {}
impl std::convert::TryFrom<AVB17221AECPMessage> for AVB17221AECPVendorMessage {
    type Error = &'static str;
    fn try_from(parent: AVB17221AECPMessage) -> Result<AVB17221AECPVendorMessage, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"AVB17221AECPVendorMessage").unwrap()) };
        if is_kind_of {
            Ok(AVB17221AECPVendorMessage(parent.0))
        } else {
            Err("This AVB17221AECPMessage cannot be downcasted to AVB17221AECPVendorMessage")
        }
    }
}
impl INSObject for AVB17221AECPVendorMessage {}
impl PNSObject for AVB17221AECPVendorMessage {}
impl IAVB17221AECPVendorMessage for AVB17221AECPVendorMessage {}
pub trait IAVB17221AECPVendorMessage: Sized + std::ops::Deref {
    unsafe fn protocolID(&self) -> u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, protocolID)
    }
    unsafe fn setProtocolID_(&self, protocolID: u64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setProtocolID : protocolID)
    }
    unsafe fn protocolSpecificData(&self) -> NSData
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, protocolSpecificData)
    }
    unsafe fn setProtocolSpecificData_(&self, protocolSpecificData: NSData)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setProtocolSpecificData : protocolSpecificData)
    }
}
pub trait PAVB17221AECPClient: Sized + std::ops::Deref {
    unsafe fn AECPDidReceiveCommand_onInterface_(
        &self,
        message: AVB17221AECPMessage,
        anInterface: AVB17221AECPInterface,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, AECPDidReceiveCommand : message, onInterface : anInterface)
    }
    unsafe fn AECPDidReceiveResponse_onInterface_(
        &self,
        message: AVB17221AECPMessage,
        anInterface: AVB17221AECPInterface,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, AECPDidReceiveResponse : message, onInterface : anInterface)
    }
}
pub type AVB17221AECPInterfaceCompletion = *mut ::std::os::raw::c_void;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct AVB17221AECPInterface(pub id);
impl std::ops::Deref for AVB17221AECPInterface {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for AVB17221AECPInterface {}
impl AVB17221AECPInterface {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"AVB17221AECPInterface").unwrap(), alloc) })
    }
}
impl IAVB1722ControlInterface for AVB17221AECPInterface {}
impl From<AVB17221AECPInterface> for AVB1722ControlInterface {
    fn from(child: AVB17221AECPInterface) -> AVB1722ControlInterface {
        AVB1722ControlInterface(child.0)
    }
}
impl std::convert::TryFrom<AVB1722ControlInterface> for AVB17221AECPInterface {
    type Error = &'static str;
    fn try_from(parent: AVB1722ControlInterface) -> Result<AVB17221AECPInterface, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"AVB17221AECPInterface").unwrap()) };
        if is_kind_of {
            Ok(AVB17221AECPInterface(parent.0))
        } else {
            Err("This AVB1722ControlInterface cannot be downcasted to AVB17221AECPInterface")
        }
    }
}
impl INSObject for AVB17221AECPInterface {}
impl PNSObject for AVB17221AECPInterface {}
impl IAVB17221AECPInterface for AVB17221AECPInterface {}
pub trait IAVB17221AECPInterface: Sized + std::ops::Deref {
    unsafe fn setCommandHandler_forEntityID_(&self, handler: *mut u64, targetEntityID: u64) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCommandHandler : handler, forEntityID : targetEntityID)
    }
    unsafe fn removeCommandHandlerForEntityID_(&self, targetEntityID: u64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, removeCommandHandlerForEntityID : targetEntityID)
    }
    unsafe fn setResponseHandler_forControllerEntityID_(
        &self,
        handler: *mut u64,
        controllerEntityID: u64,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setResponseHandler : handler, forControllerEntityID : controllerEntityID)
    }
    unsafe fn removeResponseHandlerForControllerEntityID_(&self, controllerEntityID: u64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, removeResponseHandlerForControllerEntityID : controllerEntityID)
    }
    unsafe fn sendCommand_toMACAddress_completionHandler_(
        &self,
        message: AVB17221AECPMessage,
        destMAC: AVBMACAddress,
        completionHandler: AVB17221AECPInterfaceCompletion,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, sendCommand : message, toMACAddress : destMAC, completionHandler : completionHandler)
    }
    unsafe fn sendResponse_toMACAddress_error_(
        &self,
        message: AVB17221AECPMessage,
        destMAC: AVBMACAddress,
        error: *mut NSError,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, sendResponse : message, toMACAddress : destMAC, error : error)
    }
    unsafe fn sendVendorUniqueCommand_toMACAddress_expectResponseWithinTimeout_completionHandler_(
        &self,
        message: AVB17221AECPVendorMessage,
        destMAC: AVBMACAddress,
        timeout: i64,
        completionHandler: AVB17221AECPInterfaceCompletion,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, sendVendorUniqueCommand : message, toMACAddress : destMAC, expectResponseWithinTimeout : timeout, completionHandler : completionHandler)
    }
    unsafe fn AECPInterfaceWithInterface_(anInterface: AVBInterface) -> AVB17221AECPInterface
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"AVB17221AECPInterface").unwrap(), AECPInterfaceWithInterface : anInterface)
    }
    unsafe fn AECPInterfaceWithInterfaceNamed_(anInterfaceName: NSString) -> AVB17221AECPInterface
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"AVB17221AECPInterface").unwrap(), AECPInterfaceWithInterfaceNamed : anInterfaceName)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct AVB17221ACMPMessage(pub id);
impl std::ops::Deref for AVB17221ACMPMessage {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for AVB17221ACMPMessage {}
impl AVB17221ACMPMessage {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"AVB17221ACMPMessage").unwrap(), alloc) })
    }
}
impl PNSCopying for AVB17221ACMPMessage {}
impl INSObject for AVB17221ACMPMessage {}
impl PNSObject for AVB17221ACMPMessage {}
impl std::convert::TryFrom<NSObject> for AVB17221ACMPMessage {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<AVB17221ACMPMessage, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"AVB17221ACMPMessage").unwrap()) };
        if is_kind_of {
            Ok(AVB17221ACMPMessage(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to AVB17221ACMPMessage")
        }
    }
}
impl IAVB17221ACMPMessage for AVB17221ACMPMessage {}
pub trait IAVB17221ACMPMessage: Sized + std::ops::Deref {
    unsafe fn errorForStatusCode(&self) -> NSError
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, errorForStatusCode)
    }
    unsafe fn messageType(&self) -> AVB17221ACMPMessageType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, messageType)
    }
    unsafe fn setMessageType_(&self, messageType: AVB17221ACMPMessageType)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMessageType : messageType)
    }
    unsafe fn status(&self) -> AVB17221ACMPStatusCode
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, status)
    }
    unsafe fn setStatus_(&self, status: AVB17221ACMPStatusCode)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setStatus : status)
    }
    unsafe fn streamID(&self) -> u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, streamID)
    }
    unsafe fn setStreamID_(&self, streamID: u64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setStreamID : streamID)
    }
    unsafe fn controllerEntityID(&self) -> u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, controllerEntityID)
    }
    unsafe fn setControllerEntityID_(&self, controllerEntityID: u64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setControllerEntityID : controllerEntityID)
    }
    unsafe fn talkerEntityID(&self) -> u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, talkerEntityID)
    }
    unsafe fn setTalkerEntityID_(&self, talkerEntityID: u64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setTalkerEntityID : talkerEntityID)
    }
    unsafe fn listenerEntityID(&self) -> u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, listenerEntityID)
    }
    unsafe fn setListenerEntityID_(&self, listenerEntityID: u64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setListenerEntityID : listenerEntityID)
    }
    unsafe fn talkerUniqueID(&self) -> u16
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, talkerUniqueID)
    }
    unsafe fn setTalkerUniqueID_(&self, talkerUniqueID: u16)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setTalkerUniqueID : talkerUniqueID)
    }
    unsafe fn listenerUniqueID(&self) -> u16
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, listenerUniqueID)
    }
    unsafe fn setListenerUniqueID_(&self, listenerUniqueID: u16)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setListenerUniqueID : listenerUniqueID)
    }
    unsafe fn destinationMAC(&self) -> AVBMACAddress
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, destinationMAC)
    }
    unsafe fn setDestinationMAC_(&self, destinationMAC: AVBMACAddress)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDestinationMAC : destinationMAC)
    }
    unsafe fn connectionCount(&self) -> u16
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, connectionCount)
    }
    unsafe fn setConnectionCount_(&self, connectionCount: u16)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setConnectionCount : connectionCount)
    }
    unsafe fn sequenceID(&self) -> u16
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, sequenceID)
    }
    unsafe fn setSequenceID_(&self, sequenceID: u16)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSequenceID : sequenceID)
    }
    unsafe fn flags(&self) -> AVB17221ACMPFlags
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, flags)
    }
    unsafe fn setFlags_(&self, flags: AVB17221ACMPFlags)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setFlags : flags)
    }
    unsafe fn vlanID(&self) -> u16
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, vlanID)
    }
    unsafe fn setVlanID_(&self, vlanID: u16)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setVlanID : vlanID)
    }
    unsafe fn connectedListenersEntries(&self) -> u16
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, connectedListenersEntries)
    }
    unsafe fn setConnectedListenersEntries_(&self, connectedListenersEntries: u16)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setConnectedListenersEntries : connectedListenersEntries)
    }
    unsafe fn connectedListenersEntriesValid(&self) -> bool
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, connectedListenersEntriesValid)
    }
    unsafe fn setConnectedListenersEntriesValid_(&self, connectedListenersEntriesValid: bool)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setConnectedListenersEntriesValid : connectedListenersEntriesValid)
    }
    unsafe fn ipFlags(&self) -> AVB17221ACMPIPFlag
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, ipFlags)
    }
    unsafe fn setIpFlags_(&self, ipFlags: AVB17221ACMPIPFlag)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setIpFlags : ipFlags)
    }
    unsafe fn sourcePort(&self) -> u16
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, sourcePort)
    }
    unsafe fn setSourcePort_(&self, sourcePort: u16)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSourcePort : sourcePort)
    }
    unsafe fn destinationPort(&self) -> u16
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, destinationPort)
    }
    unsafe fn setDestinationPort_(&self, destinationPort: u16)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDestinationPort : destinationPort)
    }
    unsafe fn sourceIPAddress(&self) -> AVBIPAddress
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, sourceIPAddress)
    }
    unsafe fn setSourceIPAddress_(&self, sourceIPAddress: AVBIPAddress)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSourceIPAddress : sourceIPAddress)
    }
    unsafe fn destinationIPAddress(&self) -> AVBIPAddress
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, destinationIPAddress)
    }
    unsafe fn setDestinationIPAddress_(&self, destinationIPAddress: AVBIPAddress)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDestinationIPAddress : destinationIPAddress)
    }
    unsafe fn sourceMAC(&self) -> AVBMACAddress
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, sourceMAC)
    }
    unsafe fn setSourceMAC_(&self, sourceMAC: AVBMACAddress)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSourceMAC : sourceMAC)
    }
    unsafe fn errorForStatusCode_(statusCode: AVB17221ACMPStatusCode) -> NSError
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"AVB17221ACMPMessage").unwrap(), errorForStatusCode : statusCode)
    }
}
pub type AVB17221ACMPInterfaceCompletion = *mut ::std::os::raw::c_void;
pub trait PAVB17221ACMPClient: Sized + std::ops::Deref {
    unsafe fn ACMPDidReceiveCommand_onInterface_(
        &self,
        message: AVB17221ACMPMessage,
        anInterface: AVB17221ACMPInterface,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, ACMPDidReceiveCommand : message, onInterface : anInterface)
    }
    unsafe fn ACMPDidReceiveResponse_onInterface_(
        &self,
        message: AVB17221ACMPMessage,
        anInterface: AVB17221ACMPInterface,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, ACMPDidReceiveResponse : message, onInterface : anInterface)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct AVB17221ACMPInterface(pub id);
impl std::ops::Deref for AVB17221ACMPInterface {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for AVB17221ACMPInterface {}
impl AVB17221ACMPInterface {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"AVB17221ACMPInterface").unwrap(), alloc) })
    }
}
impl IAVB1722ControlInterface for AVB17221ACMPInterface {}
impl std::convert::TryFrom<AVB1722ControlInterface> for AVB17221ACMPInterface {
    type Error = &'static str;
    fn try_from(parent: AVB1722ControlInterface) -> Result<AVB17221ACMPInterface, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"AVB17221ACMPInterface").unwrap()) };
        if is_kind_of {
            Ok(AVB17221ACMPInterface(parent.0))
        } else {
            Err("This AVB1722ControlInterface cannot be downcasted to AVB17221ACMPInterface")
        }
    }
}
impl INSObject for AVB17221ACMPInterface {}
impl PNSObject for AVB17221ACMPInterface {}
impl IAVB17221ACMPInterface for AVB17221ACMPInterface {}
pub trait IAVB17221ACMPInterface: Sized + std::ops::Deref {
    unsafe fn setHandler_forEntityID_(&self, handler: *mut u64, targetEntityID: u64) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setHandler : handler, forEntityID : targetEntityID)
    }
    unsafe fn removeHandlerForEntityID_(&self, targetEntityID: u64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, removeHandlerForEntityID : targetEntityID)
    }
    unsafe fn sendACMPResponseMessage_error_(
        &self,
        message: AVB17221ACMPMessage,
        error: *mut NSError,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, sendACMPResponseMessage : message, error : error)
    }
    unsafe fn sendACMPCommandMessage_completionHandler_(
        &self,
        message: AVB17221ACMPMessage,
        completionHandler: AVB17221ACMPInterfaceCompletion,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, sendACMPCommandMessage : message, completionHandler : completionHandler)
    }
    unsafe fn multicastDestinationAddress(&self) -> AVBMACAddress
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, multicastDestinationAddress)
    }
    unsafe fn ACMPInterfaceWithInterface_(anInterface: AVBInterface) -> AVB17221ACMPInterface
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"AVB17221ACMPInterface").unwrap(), ACMPInterfaceWithInterface : anInterface)
    }
    unsafe fn ACMPInterfaceWithInterfaceNamed_(anInterfaceName: NSString) -> AVB17221ACMPInterface
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"AVB17221ACMPInterface").unwrap(), ACMPInterfaceWithInterfaceNamed : anInterfaceName)
    }
}
pub type AVB17221EntityPropertyChanged = NSUInteger;
pub trait PAVB17221EntityDiscoveryDelegate: Sized + std::ops::Deref {
    unsafe fn didAddRemoteEntity_on17221EntityDiscovery_(
        &self,
        newEntity: AVB17221Entity,
        entityDiscovery: AVB17221EntityDiscovery,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, didAddRemoteEntity : newEntity, on17221EntityDiscovery : entityDiscovery)
    }
    unsafe fn didRemoveRemoteEntity_on17221EntityDiscovery_(
        &self,
        oldEntity: AVB17221Entity,
        entityDiscovery: AVB17221EntityDiscovery,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, didRemoveRemoteEntity : oldEntity, on17221EntityDiscovery : entityDiscovery)
    }
    unsafe fn didRediscoverRemoteEntity_on17221EntityDiscovery_(
        &self,
        entity: AVB17221Entity,
        entityDiscovery: AVB17221EntityDiscovery,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, didRediscoverRemoteEntity : entity, on17221EntityDiscovery : entityDiscovery)
    }
    unsafe fn didUpdateRemoteEntity_changedProperties_on17221EntityDiscovery_(
        &self,
        entity: AVB17221Entity,
        changedProperties: AVB17221EntityPropertyChanged,
        entityDiscovery: AVB17221EntityDiscovery,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, didUpdateRemoteEntity : entity, changedProperties : changedProperties, on17221EntityDiscovery : entityDiscovery)
    }
    unsafe fn didAddLocalEntity_on17221EntityDiscovery_(
        &self,
        newEntity: AVB17221Entity,
        entityDiscovery: AVB17221EntityDiscovery,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, didAddLocalEntity : newEntity, on17221EntityDiscovery : entityDiscovery)
    }
    unsafe fn didRemoveLocalEntity_on17221EntityDiscovery_(
        &self,
        oldEntity: AVB17221Entity,
        entityDiscovery: AVB17221EntityDiscovery,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, didRemoveLocalEntity : oldEntity, on17221EntityDiscovery : entityDiscovery)
    }
    unsafe fn didRediscoverLocalEntity_on17221EntityDiscovery_(
        &self,
        entity: AVB17221Entity,
        entityDiscovery: AVB17221EntityDiscovery,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, didRediscoverLocalEntity : entity, on17221EntityDiscovery : entityDiscovery)
    }
    unsafe fn didUpdateLocalEntity_changedProperties_on17221EntityDiscovery_(
        &self,
        entity: AVB17221Entity,
        changedProperties: AVB17221EntityPropertyChanged,
        entityDiscovery: AVB17221EntityDiscovery,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, didUpdateLocalEntity : entity, changedProperties : changedProperties, on17221EntityDiscovery : entityDiscovery)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct AVB17221EntityDiscovery(pub id);
impl std::ops::Deref for AVB17221EntityDiscovery {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for AVB17221EntityDiscovery {}
impl AVB17221EntityDiscovery {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"AVB17221EntityDiscovery").unwrap(), alloc) })
    }
}
impl INSObject for AVB17221EntityDiscovery {}
impl PNSObject for AVB17221EntityDiscovery {}
impl std::convert::TryFrom<NSObject> for AVB17221EntityDiscovery {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<AVB17221EntityDiscovery, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"AVB17221EntityDiscovery").unwrap()) };
        if is_kind_of {
            Ok(AVB17221EntityDiscovery(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to AVB17221EntityDiscovery")
        }
    }
}
impl IAVB17221EntityDiscovery for AVB17221EntityDiscovery {}
pub trait IAVB17221EntityDiscovery: Sized + std::ops::Deref {
    unsafe fn initWithInterfaceName_(&self, anInterfaceName: NSString) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithInterfaceName : anInterfaceName)
    }
    unsafe fn primeIterators(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, primeIterators)
    }
    unsafe fn discoverEntities(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, discoverEntities)
    }
    unsafe fn discoverEntity_(&self, entityID: u64) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, discoverEntity : entityID)
    }
    unsafe fn addLocalEntity_error_(&self, anEntity: AVB17221Entity, error: *mut NSError) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, addLocalEntity : anEntity, error : error)
    }
    unsafe fn removeLocalEntity_error_(&self, guid: u64, error: *mut NSError) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, removeLocalEntity : guid, error : error)
    }
    unsafe fn changeEntityWithEntityID_toNewGPTPGrandmasterID_error_(
        &self,
        entityID: u64,
        gPTPGrandmasterID: u64,
        error: *mut NSError,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, changeEntityWithEntityID : entityID, toNewGPTPGrandmasterID : gPTPGrandmasterID, error : error)
    }
    unsafe fn interfaceName(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, interfaceName)
    }
    unsafe fn setInterfaceName_(&self, interfaceName: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInterfaceName : interfaceName)
    }
    unsafe fn interface(&self) -> AVBInterface
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, interface)
    }
    unsafe fn discoveryDelegate(&self) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, discoveryDelegate)
    }
    unsafe fn setDiscoveryDelegate_(&self, discoveryDelegate: *mut u64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDiscoveryDelegate : discoveryDelegate)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct AVB17221Entity(pub id);
impl std::ops::Deref for AVB17221Entity {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for AVB17221Entity {}
impl AVB17221Entity {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"AVB17221Entity").unwrap(), alloc) })
    }
}
impl INSObject for AVB17221Entity {}
impl PNSObject for AVB17221Entity {}
impl std::convert::TryFrom<NSObject> for AVB17221Entity {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<AVB17221Entity, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"AVB17221Entity").unwrap()) };
        if is_kind_of {
            Ok(AVB17221Entity(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to AVB17221Entity")
        }
    }
}
impl IAVB17221Entity for AVB17221Entity {}
pub trait IAVB17221Entity: Sized + std::ops::Deref {
    unsafe fn isLocalEntity(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isLocalEntity)
    }
    unsafe fn setLocalEntity_(&self, localEntity: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setLocalEntity : localEntity)
    }
    unsafe fn timeToLive(&self) -> u8
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, timeToLive)
    }
    unsafe fn setTimeToLive_(&self, timeToLive: u8)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setTimeToLive : timeToLive)
    }
    unsafe fn entityID(&self) -> u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, entityID)
    }
    unsafe fn setEntityID_(&self, entityID: u64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setEntityID : entityID)
    }
    unsafe fn entityModelID(&self) -> u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, entityModelID)
    }
    unsafe fn setEntityModelID_(&self, entityModelID: u64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setEntityModelID : entityModelID)
    }
    unsafe fn entityCapabilities(&self) -> AVB17221ADPEntityCapabilities
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, entityCapabilities)
    }
    unsafe fn setEntityCapabilities_(&self, entityCapabilities: AVB17221ADPEntityCapabilities)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setEntityCapabilities : entityCapabilities)
    }
    unsafe fn talkerStreamSources(&self) -> u16
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, talkerStreamSources)
    }
    unsafe fn setTalkerStreamSources_(&self, talkerStreamSources: u16)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setTalkerStreamSources : talkerStreamSources)
    }
    unsafe fn talkerCapabilities(&self) -> AVB17221ADPTalkerCapabilities
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, talkerCapabilities)
    }
    unsafe fn setTalkerCapabilities_(&self, talkerCapabilities: AVB17221ADPTalkerCapabilities)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setTalkerCapabilities : talkerCapabilities)
    }
    unsafe fn listenerStreamSinks(&self) -> u16
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, listenerStreamSinks)
    }
    unsafe fn setListenerStreamSinks_(&self, listenerStreamSinks: u16)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setListenerStreamSinks : listenerStreamSinks)
    }
    unsafe fn listenerCapabilities(&self) -> AVB17221ADPListenerCapabilities
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, listenerCapabilities)
    }
    unsafe fn setListenerCapabilities_(&self, listenerCapabilities: AVB17221ADPListenerCapabilities)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setListenerCapabilities : listenerCapabilities)
    }
    unsafe fn controllerCapabilities(&self) -> AVB17221ADPControllerCapabilities
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, controllerCapabilities)
    }
    unsafe fn setControllerCapabilities_(
        &self,
        controllerCapabilities: AVB17221ADPControllerCapabilities,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setControllerCapabilities : controllerCapabilities)
    }
    unsafe fn availableIndex(&self) -> u32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, availableIndex)
    }
    unsafe fn setAvailableIndex_(&self, availableIndex: u32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAvailableIndex : availableIndex)
    }
    unsafe fn gPTPGrandmasterID(&self) -> u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, gPTPGrandmasterID)
    }
    unsafe fn setGPTPGrandmasterID_(&self, gPTPGrandmasterID: u64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setGPTPGrandmasterID : gPTPGrandmasterID)
    }
    unsafe fn gPTPDomainNumber(&self) -> u8
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, gPTPDomainNumber)
    }
    unsafe fn setGPTPDomainNumber_(&self, gPTPDomainNumber: u8)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setGPTPDomainNumber : gPTPDomainNumber)
    }
    unsafe fn identifyControlIndex(&self) -> u16
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, identifyControlIndex)
    }
    unsafe fn setIdentifyControlIndex_(&self, identifyControlIndex: u16)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setIdentifyControlIndex : identifyControlIndex)
    }
    unsafe fn interfaceIndex(&self) -> u16
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, interfaceIndex)
    }
    unsafe fn setInterfaceIndex_(&self, interfaceIndex: u16)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInterfaceIndex : interfaceIndex)
    }
    unsafe fn associationID(&self) -> u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, associationID)
    }
    unsafe fn setAssociationID_(&self, associationID: u64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAssociationID : associationID)
    }
    unsafe fn currentConfigurationIndex(&self) -> u16
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, currentConfigurationIndex)
    }
    unsafe fn setCurrentConfigurationIndex_(&self, currentConfigurationIndex: u16)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCurrentConfigurationIndex : currentConfigurationIndex)
    }
    unsafe fn macAddresses(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, macAddresses)
    }
    unsafe fn setMacAddresses_(&self, macAddresses: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMacAddresses : macAddresses)
    }
    unsafe fn entityDiscovery(&self) -> AVB17221EntityDiscovery
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, entityDiscovery)
    }
    unsafe fn setEntityDiscovery_(&self, entityDiscovery: AVB17221EntityDiscovery)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setEntityDiscovery : entityDiscovery)
    }
}
unsafe extern "C" {
    pub static AVBErrorDomain: NSString;
}
unsafe extern "C" {
    pub static AVBNullEUI64: u64;
}

unsafe impl objc2::encode::RefEncode for AVBCentralManager {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for AVBCentralManager {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for AVBInterface {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for AVBInterface {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for AVBEthernetInterface {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for AVBEthernetInterface {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for AVBMACAddress {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for AVBMACAddress {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for AVBIPAddress {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for AVBIPAddress {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for AVB1722ControlInterface {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for AVB1722ControlInterface {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for AVB17221AECPMessage {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for AVB17221AECPMessage {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for AVB17221AECPAEMMessage {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for AVB17221AECPAEMMessage {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for AVB17221AECPAddressAccessMessage {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for AVB17221AECPAddressAccessMessage {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for AVB17221AECPAddressAccessTLV {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for AVB17221AECPAddressAccessTLV {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for AVB17221AECPAVCMessage {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for AVB17221AECPAVCMessage {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for AVB17221AECPVendorMessage {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for AVB17221AECPVendorMessage {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for AVB17221AECPInterface {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for AVB17221AECPInterface {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for AVB17221ACMPMessage {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for AVB17221ACMPMessage {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for AVB17221ACMPInterface {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for AVB17221ACMPInterface {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for AVB17221EntityDiscovery {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for AVB17221EntityDiscovery {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for AVB17221Entity {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for AVB17221Entity {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
