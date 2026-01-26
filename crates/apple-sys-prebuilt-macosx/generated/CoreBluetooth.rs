#[allow(unused_imports)]
use crate::objc::*;
#[allow(unused_imports)]
use crate::CoreFoundation::*;
#[allow(unused_imports)]
use crate::Foundation::*;

#[allow(unused_imports)]
use objc2::msg_send;
pub type CBConnectionEventMatchingOption = NSString;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CBManager(pub id);
impl std::ops::Deref for CBManager {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CBManager {}
impl CBManager {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CBManager").unwrap(), alloc) })
    }
}
impl INSObject for CBManager {}
impl PNSObject for CBManager {}
impl std::convert::TryFrom<NSObject> for CBManager {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<CBManager, Self::Error> {
        let is_kind_of: bool = unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CBManager").unwrap()) };
        if is_kind_of {
            Ok(CBManager(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to CBManager")
        }
    }
}
impl ICBManager for CBManager {}
pub trait ICBManager: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn state(&self) -> CBManagerState
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, state)
    }
    unsafe fn authorization(&self) -> CBManagerAuthorization
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, authorization)
    }
    unsafe fn class_authorization() -> CBManagerAuthorization
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CBManager").unwrap(), authorization)
    }
}
pub type CBManagerState = NSInteger;
pub type CBManagerAuthorization = NSInteger;
pub type CBCentralManagerState = NSInteger;
pub type CBConnectionEvent = NSInteger;
pub type CBCentralManagerFeature = NSUInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CBCentralManager(pub id);
impl std::ops::Deref for CBCentralManager {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CBCentralManager {}
impl CBCentralManager {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CBCentralManager").unwrap(), alloc) })
    }
}
impl ICBManager for CBCentralManager {}
impl From<CBCentralManager> for CBManager {
    fn from(child: CBCentralManager) -> CBManager {
        CBManager(child.0)
    }
}
impl std::convert::TryFrom<CBManager> for CBCentralManager {
    type Error = &'static str;
    fn try_from(parent: CBManager) -> Result<CBCentralManager, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CBCentralManager").unwrap()) };
        if is_kind_of {
            Ok(CBCentralManager(parent.0))
        } else {
            Err("This CBManager cannot be downcasted to CBCentralManager")
        }
    }
}
impl INSObject for CBCentralManager {}
impl PNSObject for CBCentralManager {}
impl ICBCentralManager for CBCentralManager {}
pub trait ICBCentralManager: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn initWithDelegate_queue_(&self, delegate: *mut u64, queue: NSObject) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithDelegate : delegate, queue : queue)
    }
    unsafe fn initWithDelegate_queue_options_(
        &self,
        delegate: *mut u64,
        queue: NSObject,
        options: NSDictionary,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithDelegate : delegate, queue : queue, options : options)
    }
    unsafe fn retrievePeripheralsWithIdentifiers_(&self, identifiers: NSArray) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, retrievePeripheralsWithIdentifiers : identifiers)
    }
    unsafe fn retrieveConnectedPeripheralsWithServices_(&self, serviceUUIDs: NSArray) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, retrieveConnectedPeripheralsWithServices : serviceUUIDs)
    }
    unsafe fn scanForPeripheralsWithServices_options_(
        &self,
        serviceUUIDs: NSArray,
        options: NSDictionary,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, scanForPeripheralsWithServices : serviceUUIDs, options : options)
    }
    unsafe fn stopScan(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, stopScan)
    }
    unsafe fn connectPeripheral_options_(&self, peripheral: CBPeripheral, options: NSDictionary)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, connectPeripheral : peripheral, options : options)
    }
    unsafe fn cancelPeripheralConnection_(&self, peripheral: CBPeripheral)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, cancelPeripheralConnection : peripheral)
    }
    unsafe fn registerForConnectionEventsWithOptions_(&self, options: NSDictionary)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, registerForConnectionEventsWithOptions : options)
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
    unsafe fn isScanning(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isScanning)
    }
    unsafe fn supportsFeatures_(features: CBCentralManagerFeature) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CBCentralManager").unwrap(), supportsFeatures : features)
    }
}
pub trait PCBCentralManagerDelegate: Sized + std::ops::Deref {
    unsafe fn centralManagerDidUpdateState_(&self, central: CBCentralManager)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, centralManagerDidUpdateState : central)
    }
    unsafe fn centralManager_willRestoreState_(&self, central: CBCentralManager, dict: NSDictionary)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, centralManager : central, willRestoreState : dict)
    }
    unsafe fn centralManager_didDiscoverPeripheral_advertisementData_RSSI_(
        &self,
        central: CBCentralManager,
        peripheral: CBPeripheral,
        advertisementData: NSDictionary,
        RSSI: NSNumber,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, centralManager : central, didDiscoverPeripheral : peripheral, advertisementData : advertisementData, RSSI : RSSI)
    }
    unsafe fn centralManager_didConnectPeripheral_(
        &self,
        central: CBCentralManager,
        peripheral: CBPeripheral,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, centralManager : central, didConnectPeripheral : peripheral)
    }
    unsafe fn centralManager_didFailToConnectPeripheral_error_(
        &self,
        central: CBCentralManager,
        peripheral: CBPeripheral,
        error: NSError,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, centralManager : central, didFailToConnectPeripheral : peripheral, error : error)
    }
    unsafe fn centralManager_didDisconnectPeripheral_error_(
        &self,
        central: CBCentralManager,
        peripheral: CBPeripheral,
        error: NSError,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, centralManager : central, didDisconnectPeripheral : peripheral, error : error)
    }
    unsafe fn centralManager_didDisconnectPeripheral_timestamp_isReconnecting_error_(
        &self,
        central: CBCentralManager,
        peripheral: CBPeripheral,
        timestamp: CFAbsoluteTime,
        isReconnecting: BOOL,
        error: NSError,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, centralManager : central, didDisconnectPeripheral : peripheral, timestamp : timestamp, isReconnecting : isReconnecting, error : error)
    }
    unsafe fn centralManager_connectionEventDidOccur_forPeripheral_(
        &self,
        central: CBCentralManager,
        event: CBConnectionEvent,
        peripheral: CBPeripheral,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, centralManager : central, connectionEventDidOccur : event, forPeripheral : peripheral)
    }
    unsafe fn centralManager_didUpdateANCSAuthorizationForPeripheral_(
        &self,
        central: CBCentralManager,
        peripheral: CBPeripheral,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, centralManager : central, didUpdateANCSAuthorizationForPeripheral : peripheral)
    }
}
pub type CBError = NSInteger;
pub type CBATTError = NSInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CBPeer(pub id);
impl std::ops::Deref for CBPeer {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CBPeer {}
impl CBPeer {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CBPeer").unwrap(), alloc) })
    }
}
impl PNSCopying for CBPeer {}
impl INSObject for CBPeer {}
impl PNSObject for CBPeer {}
impl std::convert::TryFrom<NSObject> for CBPeer {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<CBPeer, Self::Error> {
        let is_kind_of: bool = unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CBPeer").unwrap()) };
        if is_kind_of {
            Ok(CBPeer(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to CBPeer")
        }
    }
}
impl ICBPeer for CBPeer {}
pub trait ICBPeer: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn identifier(&self) -> NSUUID
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, identifier)
    }
}
pub type CBL2CAPPSM = u16;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CBL2CAPChannel(pub id);
impl std::ops::Deref for CBL2CAPChannel {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CBL2CAPChannel {}
impl CBL2CAPChannel {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CBL2CAPChannel").unwrap(), alloc) })
    }
}
impl INSObject for CBL2CAPChannel {}
impl PNSObject for CBL2CAPChannel {}
impl std::convert::TryFrom<NSObject> for CBL2CAPChannel {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<CBL2CAPChannel, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CBL2CAPChannel").unwrap()) };
        if is_kind_of {
            Ok(CBL2CAPChannel(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to CBL2CAPChannel")
        }
    }
}
impl ICBL2CAPChannel for CBL2CAPChannel {}
pub trait ICBL2CAPChannel: Sized + std::ops::Deref {
    unsafe fn peer(&self) -> CBPeer
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, peer)
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
    unsafe fn PSM(&self) -> CBL2CAPPSM
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, PSM)
    }
}
pub type CBPeripheralManagerAuthorizationStatus = NSInteger;
pub type CBPeripheralManagerState = NSInteger;
pub type CBPeripheralManagerConnectionLatency = NSInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CBPeripheralManager(pub id);
impl std::ops::Deref for CBPeripheralManager {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CBPeripheralManager {}
impl CBPeripheralManager {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CBPeripheralManager").unwrap(), alloc) })
    }
}
impl ICBManager for CBPeripheralManager {}
impl std::convert::TryFrom<CBManager> for CBPeripheralManager {
    type Error = &'static str;
    fn try_from(parent: CBManager) -> Result<CBPeripheralManager, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CBPeripheralManager").unwrap()) };
        if is_kind_of {
            Ok(CBPeripheralManager(parent.0))
        } else {
            Err("This CBManager cannot be downcasted to CBPeripheralManager")
        }
    }
}
impl INSObject for CBPeripheralManager {}
impl PNSObject for CBPeripheralManager {}
impl ICBPeripheralManager for CBPeripheralManager {}
pub trait ICBPeripheralManager: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn initWithDelegate_queue_(&self, delegate: *mut u64, queue: NSObject) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithDelegate : delegate, queue : queue)
    }
    unsafe fn initWithDelegate_queue_options_(
        &self,
        delegate: *mut u64,
        queue: NSObject,
        options: NSDictionary,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithDelegate : delegate, queue : queue, options : options)
    }
    unsafe fn startAdvertising_(&self, advertisementData: NSDictionary)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, startAdvertising : advertisementData)
    }
    unsafe fn stopAdvertising(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, stopAdvertising)
    }
    unsafe fn setDesiredConnectionLatency_forCentral_(
        &self,
        latency: CBPeripheralManagerConnectionLatency,
        central: CBCentral,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDesiredConnectionLatency : latency, forCentral : central)
    }
    unsafe fn addService_(&self, service: CBMutableService)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, addService : service)
    }
    unsafe fn removeService_(&self, service: CBMutableService)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, removeService : service)
    }
    unsafe fn removeAllServices(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, removeAllServices)
    }
    unsafe fn respondToRequest_withResult_(&self, request: CBATTRequest, result: CBATTError)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, respondToRequest : request, withResult : result)
    }
    unsafe fn updateValue_forCharacteristic_onSubscribedCentrals_(
        &self,
        value: NSData,
        characteristic: CBMutableCharacteristic,
        centrals: NSArray,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, updateValue : value, forCharacteristic : characteristic, onSubscribedCentrals : centrals)
    }
    unsafe fn publishL2CAPChannelWithEncryption_(&self, encryptionRequired: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, publishL2CAPChannelWithEncryption : encryptionRequired)
    }
    unsafe fn unpublishL2CAPChannel_(&self, PSM: CBL2CAPPSM)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, unpublishL2CAPChannel : PSM)
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
    unsafe fn isAdvertising(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isAdvertising)
    }
    unsafe fn authorizationStatus() -> CBPeripheralManagerAuthorizationStatus
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CBPeripheralManager").unwrap(), authorizationStatus)
    }
}
pub trait PCBPeripheralManagerDelegate: Sized + std::ops::Deref {
    unsafe fn peripheralManagerDidUpdateState_(&self, peripheral: CBPeripheralManager)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, peripheralManagerDidUpdateState : peripheral)
    }
    unsafe fn peripheralManager_willRestoreState_(
        &self,
        peripheral: CBPeripheralManager,
        dict: NSDictionary,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, peripheralManager : peripheral, willRestoreState : dict)
    }
    unsafe fn peripheralManagerDidStartAdvertising_error_(
        &self,
        peripheral: CBPeripheralManager,
        error: NSError,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, peripheralManagerDidStartAdvertising : peripheral, error : error)
    }
    unsafe fn peripheralManager_didAddService_error_(
        &self,
        peripheral: CBPeripheralManager,
        service: CBService,
        error: NSError,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, peripheralManager : peripheral, didAddService : service, error : error)
    }
    unsafe fn peripheralManager_central_didSubscribeToCharacteristic_(
        &self,
        peripheral: CBPeripheralManager,
        central: CBCentral,
        characteristic: CBCharacteristic,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, peripheralManager : peripheral, central : central, didSubscribeToCharacteristic : characteristic)
    }
    unsafe fn peripheralManager_central_didUnsubscribeFromCharacteristic_(
        &self,
        peripheral: CBPeripheralManager,
        central: CBCentral,
        characteristic: CBCharacteristic,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, peripheralManager : peripheral, central : central, didUnsubscribeFromCharacteristic : characteristic)
    }
    unsafe fn peripheralManager_didReceiveReadRequest_(
        &self,
        peripheral: CBPeripheralManager,
        request: CBATTRequest,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, peripheralManager : peripheral, didReceiveReadRequest : request)
    }
    unsafe fn peripheralManager_didReceiveWriteRequests_(
        &self,
        peripheral: CBPeripheralManager,
        requests: NSArray,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, peripheralManager : peripheral, didReceiveWriteRequests : requests)
    }
    unsafe fn peripheralManagerIsReadyToUpdateSubscribers_(&self, peripheral: CBPeripheralManager)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, peripheralManagerIsReadyToUpdateSubscribers : peripheral)
    }
    unsafe fn peripheralManager_didPublishL2CAPChannel_error_(
        &self,
        peripheral: CBPeripheralManager,
        PSM: CBL2CAPPSM,
        error: NSError,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, peripheralManager : peripheral, didPublishL2CAPChannel : PSM, error : error)
    }
    unsafe fn peripheralManager_didUnpublishL2CAPChannel_error_(
        &self,
        peripheral: CBPeripheralManager,
        PSM: CBL2CAPPSM,
        error: NSError,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, peripheralManager : peripheral, didUnpublishL2CAPChannel : PSM, error : error)
    }
    unsafe fn peripheralManager_didOpenL2CAPChannel_error_(
        &self,
        peripheral: CBPeripheralManager,
        channel: CBL2CAPChannel,
        error: NSError,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, peripheralManager : peripheral, didOpenL2CAPChannel : channel, error : error)
    }
}
pub type CBPeripheralState = NSInteger;
pub type CBCharacteristicWriteType = NSInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CBPeripheral(pub id);
impl std::ops::Deref for CBPeripheral {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CBPeripheral {}
impl CBPeripheral {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CBPeripheral").unwrap(), alloc) })
    }
}
impl ICBPeer for CBPeripheral {}
impl PNSCopying for CBPeripheral {}
impl From<CBPeripheral> for CBPeer {
    fn from(child: CBPeripheral) -> CBPeer {
        CBPeer(child.0)
    }
}
impl std::convert::TryFrom<CBPeer> for CBPeripheral {
    type Error = &'static str;
    fn try_from(parent: CBPeer) -> Result<CBPeripheral, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CBPeripheral").unwrap()) };
        if is_kind_of {
            Ok(CBPeripheral(parent.0))
        } else {
            Err("This CBPeer cannot be downcasted to CBPeripheral")
        }
    }
}
impl INSObject for CBPeripheral {}
impl PNSObject for CBPeripheral {}
impl ICBPeripheral for CBPeripheral {}
pub trait ICBPeripheral: Sized + std::ops::Deref {
    unsafe fn readRSSI(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, readRSSI)
    }
    unsafe fn discoverServices_(&self, serviceUUIDs: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, discoverServices : serviceUUIDs)
    }
    unsafe fn discoverIncludedServices_forService_(
        &self,
        includedServiceUUIDs: NSArray,
        service: CBService,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, discoverIncludedServices : includedServiceUUIDs, forService : service)
    }
    unsafe fn discoverCharacteristics_forService_(
        &self,
        characteristicUUIDs: NSArray,
        service: CBService,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, discoverCharacteristics : characteristicUUIDs, forService : service)
    }
    unsafe fn readValueForCharacteristic_(&self, characteristic: CBCharacteristic)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, readValueForCharacteristic : characteristic)
    }
    unsafe fn maximumWriteValueLengthForType_(&self, type_: CBCharacteristicWriteType) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, maximumWriteValueLengthForType : type_)
    }
    unsafe fn writeValue_forCharacteristic_type_(
        &self,
        data: NSData,
        characteristic: CBCharacteristic,
        type_: CBCharacteristicWriteType,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, writeValue : data, forCharacteristic : characteristic, r#type : type_)
    }
    unsafe fn setNotifyValue_forCharacteristic_(
        &self,
        enabled: BOOL,
        characteristic: CBCharacteristic,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setNotifyValue : enabled, forCharacteristic : characteristic)
    }
    unsafe fn discoverDescriptorsForCharacteristic_(&self, characteristic: CBCharacteristic)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, discoverDescriptorsForCharacteristic : characteristic)
    }
    unsafe fn readValueForDescriptor_(&self, descriptor: CBDescriptor)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, readValueForDescriptor : descriptor)
    }
    unsafe fn writeValue_forDescriptor_(&self, data: NSData, descriptor: CBDescriptor)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, writeValue : data, forDescriptor : descriptor)
    }
    unsafe fn openL2CAPChannel_(&self, PSM: CBL2CAPPSM)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, openL2CAPChannel : PSM)
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
    unsafe fn name(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, name)
    }
    unsafe fn RSSI(&self) -> NSNumber
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, RSSI)
    }
    unsafe fn state(&self) -> CBPeripheralState
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, state)
    }
    unsafe fn services(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, services)
    }
    unsafe fn canSendWriteWithoutResponse(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, canSendWriteWithoutResponse)
    }
    unsafe fn ancsAuthorized(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, ancsAuthorized)
    }
}
pub trait PCBPeripheralDelegate: Sized + std::ops::Deref {
    unsafe fn peripheralDidUpdateName_(&self, peripheral: CBPeripheral)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, peripheralDidUpdateName : peripheral)
    }
    unsafe fn peripheral_didModifyServices_(
        &self,
        peripheral: CBPeripheral,
        invalidatedServices: NSArray,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, peripheral : peripheral, didModifyServices : invalidatedServices)
    }
    unsafe fn peripheralDidUpdateRSSI_error_(&self, peripheral: CBPeripheral, error: NSError)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, peripheralDidUpdateRSSI : peripheral, error : error)
    }
    unsafe fn peripheral_didReadRSSI_error_(
        &self,
        peripheral: CBPeripheral,
        RSSI: NSNumber,
        error: NSError,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, peripheral : peripheral, didReadRSSI : RSSI, error : error)
    }
    unsafe fn peripheral_didDiscoverServices_(&self, peripheral: CBPeripheral, error: NSError)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, peripheral : peripheral, didDiscoverServices : error)
    }
    unsafe fn peripheral_didDiscoverIncludedServicesForService_error_(
        &self,
        peripheral: CBPeripheral,
        service: CBService,
        error: NSError,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, peripheral : peripheral, didDiscoverIncludedServicesForService : service, error : error)
    }
    unsafe fn peripheral_didDiscoverCharacteristicsForService_error_(
        &self,
        peripheral: CBPeripheral,
        service: CBService,
        error: NSError,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, peripheral : peripheral, didDiscoverCharacteristicsForService : service, error : error)
    }
    unsafe fn peripheral_didUpdateValueForCharacteristic_error_(
        &self,
        peripheral: CBPeripheral,
        characteristic: CBCharacteristic,
        error: NSError,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, peripheral : peripheral, didUpdateValueForCharacteristic : characteristic, error : error)
    }
    unsafe fn peripheral_didWriteValueForCharacteristic_error_(
        &self,
        peripheral: CBPeripheral,
        characteristic: CBCharacteristic,
        error: NSError,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, peripheral : peripheral, didWriteValueForCharacteristic : characteristic, error : error)
    }
    unsafe fn peripheral_didUpdateNotificationStateForCharacteristic_error_(
        &self,
        peripheral: CBPeripheral,
        characteristic: CBCharacteristic,
        error: NSError,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, peripheral : peripheral, didUpdateNotificationStateForCharacteristic : characteristic, error : error)
    }
    unsafe fn peripheral_didDiscoverDescriptorsForCharacteristic_error_(
        &self,
        peripheral: CBPeripheral,
        characteristic: CBCharacteristic,
        error: NSError,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, peripheral : peripheral, didDiscoverDescriptorsForCharacteristic : characteristic, error : error)
    }
    unsafe fn peripheral_didUpdateValueForDescriptor_error_(
        &self,
        peripheral: CBPeripheral,
        descriptor: CBDescriptor,
        error: NSError,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, peripheral : peripheral, didUpdateValueForDescriptor : descriptor, error : error)
    }
    unsafe fn peripheral_didWriteValueForDescriptor_error_(
        &self,
        peripheral: CBPeripheral,
        descriptor: CBDescriptor,
        error: NSError,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, peripheral : peripheral, didWriteValueForDescriptor : descriptor, error : error)
    }
    unsafe fn peripheralIsReadyToSendWriteWithoutResponse_(&self, peripheral: CBPeripheral)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, peripheralIsReadyToSendWriteWithoutResponse : peripheral)
    }
    unsafe fn peripheral_didOpenL2CAPChannel_error_(
        &self,
        peripheral: CBPeripheral,
        channel: CBL2CAPChannel,
        error: NSError,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, peripheral : peripheral, didOpenL2CAPChannel : channel, error : error)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CBCentral(pub id);
impl std::ops::Deref for CBCentral {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CBCentral {}
impl CBCentral {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CBCentral").unwrap(), alloc) })
    }
}
impl ICBPeer for CBCentral {}
impl PNSCopying for CBCentral {}
impl std::convert::TryFrom<CBPeer> for CBCentral {
    type Error = &'static str;
    fn try_from(parent: CBPeer) -> Result<CBCentral, Self::Error> {
        let is_kind_of: bool = unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CBCentral").unwrap()) };
        if is_kind_of {
            Ok(CBCentral(parent.0))
        } else {
            Err("This CBPeer cannot be downcasted to CBCentral")
        }
    }
}
impl INSObject for CBCentral {}
impl PNSObject for CBCentral {}
impl ICBCentral for CBCentral {}
pub trait ICBCentral: Sized + std::ops::Deref {
    unsafe fn maximumUpdateValueLength(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, maximumUpdateValueLength)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CBAttribute(pub id);
impl std::ops::Deref for CBAttribute {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CBAttribute {}
impl CBAttribute {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CBAttribute").unwrap(), alloc) })
    }
}
impl INSObject for CBAttribute {}
impl PNSObject for CBAttribute {}
impl std::convert::TryFrom<NSObject> for CBAttribute {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<CBAttribute, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CBAttribute").unwrap()) };
        if is_kind_of {
            Ok(CBAttribute(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to CBAttribute")
        }
    }
}
impl ICBAttribute for CBAttribute {}
pub trait ICBAttribute: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn UUID(&self) -> CBUUID
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, UUID)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CBService(pub id);
impl std::ops::Deref for CBService {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CBService {}
impl CBService {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CBService").unwrap(), alloc) })
    }
}
impl ICBAttribute for CBService {}
impl From<CBService> for CBAttribute {
    fn from(child: CBService) -> CBAttribute {
        CBAttribute(child.0)
    }
}
impl std::convert::TryFrom<CBAttribute> for CBService {
    type Error = &'static str;
    fn try_from(parent: CBAttribute) -> Result<CBService, Self::Error> {
        let is_kind_of: bool = unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CBService").unwrap()) };
        if is_kind_of {
            Ok(CBService(parent.0))
        } else {
            Err("This CBAttribute cannot be downcasted to CBService")
        }
    }
}
impl INSObject for CBService {}
impl PNSObject for CBService {}
impl ICBService for CBService {}
pub trait ICBService: Sized + std::ops::Deref {
    unsafe fn peripheral(&self) -> CBPeripheral
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, peripheral)
    }
    unsafe fn isPrimary(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isPrimary)
    }
    unsafe fn includedServices(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, includedServices)
    }
    unsafe fn characteristics(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, characteristics)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CBMutableService(pub id);
impl std::ops::Deref for CBMutableService {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CBMutableService {}
impl CBMutableService {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CBMutableService").unwrap(), alloc) })
    }
}
impl ICBService for CBMutableService {}
impl From<CBMutableService> for CBService {
    fn from(child: CBMutableService) -> CBService {
        CBService(child.0)
    }
}
impl std::convert::TryFrom<CBService> for CBMutableService {
    type Error = &'static str;
    fn try_from(parent: CBService) -> Result<CBMutableService, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CBMutableService").unwrap()) };
        if is_kind_of {
            Ok(CBMutableService(parent.0))
        } else {
            Err("This CBService cannot be downcasted to CBMutableService")
        }
    }
}
impl ICBAttribute for CBMutableService {}
impl INSObject for CBMutableService {}
impl PNSObject for CBMutableService {}
impl ICBMutableService for CBMutableService {}
pub trait ICBMutableService: Sized + std::ops::Deref {
    unsafe fn initWithType_primary_(&self, UUID: CBUUID, isPrimary: BOOL) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithType : UUID, primary : isPrimary)
    }
    unsafe fn includedServices(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, includedServices)
    }
    unsafe fn setIncludedServices_(&self, includedServices: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setIncludedServices : includedServices)
    }
    unsafe fn characteristics(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, characteristics)
    }
    unsafe fn setCharacteristics_(&self, characteristics: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCharacteristics : characteristics)
    }
}
pub type CBCharacteristicProperties = NSUInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CBCharacteristic(pub id);
impl std::ops::Deref for CBCharacteristic {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CBCharacteristic {}
impl CBCharacteristic {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CBCharacteristic").unwrap(), alloc) })
    }
}
impl ICBAttribute for CBCharacteristic {}
impl std::convert::TryFrom<CBAttribute> for CBCharacteristic {
    type Error = &'static str;
    fn try_from(parent: CBAttribute) -> Result<CBCharacteristic, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CBCharacteristic").unwrap()) };
        if is_kind_of {
            Ok(CBCharacteristic(parent.0))
        } else {
            Err("This CBAttribute cannot be downcasted to CBCharacteristic")
        }
    }
}
impl INSObject for CBCharacteristic {}
impl PNSObject for CBCharacteristic {}
impl ICBCharacteristic for CBCharacteristic {}
pub trait ICBCharacteristic: Sized + std::ops::Deref {
    unsafe fn service(&self) -> CBService
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, service)
    }
    unsafe fn properties(&self) -> CBCharacteristicProperties
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, properties)
    }
    unsafe fn value(&self) -> NSData
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, value)
    }
    unsafe fn descriptors(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, descriptors)
    }
    unsafe fn isBroadcasted(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isBroadcasted)
    }
    unsafe fn isNotifying(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isNotifying)
    }
}
pub type CBAttributePermissions = NSUInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CBMutableCharacteristic(pub id);
impl std::ops::Deref for CBMutableCharacteristic {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CBMutableCharacteristic {}
impl CBMutableCharacteristic {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CBMutableCharacteristic").unwrap(), alloc) })
    }
}
impl ICBCharacteristic for CBMutableCharacteristic {}
impl From<CBMutableCharacteristic> for CBCharacteristic {
    fn from(child: CBMutableCharacteristic) -> CBCharacteristic {
        CBCharacteristic(child.0)
    }
}
impl std::convert::TryFrom<CBCharacteristic> for CBMutableCharacteristic {
    type Error = &'static str;
    fn try_from(parent: CBCharacteristic) -> Result<CBMutableCharacteristic, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CBMutableCharacteristic").unwrap()) };
        if is_kind_of {
            Ok(CBMutableCharacteristic(parent.0))
        } else {
            Err("This CBCharacteristic cannot be downcasted to CBMutableCharacteristic")
        }
    }
}
impl ICBAttribute for CBMutableCharacteristic {}
impl INSObject for CBMutableCharacteristic {}
impl PNSObject for CBMutableCharacteristic {}
impl ICBMutableCharacteristic for CBMutableCharacteristic {}
pub trait ICBMutableCharacteristic: Sized + std::ops::Deref {
    unsafe fn initWithType_properties_value_permissions_(
        &self,
        UUID: CBUUID,
        properties: CBCharacteristicProperties,
        value: NSData,
        permissions: CBAttributePermissions,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithType : UUID, properties : properties, value : value, permissions : permissions)
    }
    unsafe fn permissions(&self) -> CBAttributePermissions
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, permissions)
    }
    unsafe fn setPermissions_(&self, permissions: CBAttributePermissions)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPermissions : permissions)
    }
    unsafe fn subscribedCentrals(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, subscribedCentrals)
    }
    unsafe fn properties(&self) -> CBCharacteristicProperties
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, properties)
    }
    unsafe fn setProperties_(&self, properties: CBCharacteristicProperties)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setProperties : properties)
    }
    unsafe fn value(&self) -> NSData
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, value)
    }
    unsafe fn setValue_(&self, value: NSData)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setValue : value)
    }
    unsafe fn descriptors(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, descriptors)
    }
    unsafe fn setDescriptors_(&self, descriptors: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDescriptors : descriptors)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CBDescriptor(pub id);
impl std::ops::Deref for CBDescriptor {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CBDescriptor {}
impl CBDescriptor {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CBDescriptor").unwrap(), alloc) })
    }
}
impl ICBAttribute for CBDescriptor {}
impl std::convert::TryFrom<CBAttribute> for CBDescriptor {
    type Error = &'static str;
    fn try_from(parent: CBAttribute) -> Result<CBDescriptor, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CBDescriptor").unwrap()) };
        if is_kind_of {
            Ok(CBDescriptor(parent.0))
        } else {
            Err("This CBAttribute cannot be downcasted to CBDescriptor")
        }
    }
}
impl INSObject for CBDescriptor {}
impl PNSObject for CBDescriptor {}
impl ICBDescriptor for CBDescriptor {}
pub trait ICBDescriptor: Sized + std::ops::Deref {
    unsafe fn characteristic(&self) -> CBCharacteristic
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, characteristic)
    }
    unsafe fn value(&self) -> id
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, value)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CBMutableDescriptor(pub id);
impl std::ops::Deref for CBMutableDescriptor {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CBMutableDescriptor {}
impl CBMutableDescriptor {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CBMutableDescriptor").unwrap(), alloc) })
    }
}
impl ICBDescriptor for CBMutableDescriptor {}
impl From<CBMutableDescriptor> for CBDescriptor {
    fn from(child: CBMutableDescriptor) -> CBDescriptor {
        CBDescriptor(child.0)
    }
}
impl std::convert::TryFrom<CBDescriptor> for CBMutableDescriptor {
    type Error = &'static str;
    fn try_from(parent: CBDescriptor) -> Result<CBMutableDescriptor, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CBMutableDescriptor").unwrap()) };
        if is_kind_of {
            Ok(CBMutableDescriptor(parent.0))
        } else {
            Err("This CBDescriptor cannot be downcasted to CBMutableDescriptor")
        }
    }
}
impl ICBAttribute for CBMutableDescriptor {}
impl INSObject for CBMutableDescriptor {}
impl PNSObject for CBMutableDescriptor {}
impl ICBMutableDescriptor for CBMutableDescriptor {}
pub trait ICBMutableDescriptor: Sized + std::ops::Deref {
    unsafe fn initWithType_value_(&self, UUID: CBUUID, value: id) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithType : UUID, value : value)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CBUUID(pub id);
impl std::ops::Deref for CBUUID {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CBUUID {}
impl CBUUID {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CBUUID").unwrap(), alloc) })
    }
}
impl PNSCopying for CBUUID {}
impl INSObject for CBUUID {}
impl PNSObject for CBUUID {}
impl std::convert::TryFrom<NSObject> for CBUUID {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<CBUUID, Self::Error> {
        let is_kind_of: bool = unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CBUUID").unwrap()) };
        if is_kind_of {
            Ok(CBUUID(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to CBUUID")
        }
    }
}
impl ICBUUID for CBUUID {}
pub trait ICBUUID: Sized + std::ops::Deref {
    unsafe fn data(&self) -> NSData
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, data)
    }
    unsafe fn UUIDString(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, UUIDString)
    }
    unsafe fn UUIDWithString_(theString: NSString) -> CBUUID
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CBUUID").unwrap(), UUIDWithString : theString)
    }
    unsafe fn UUIDWithData_(theData: NSData) -> CBUUID
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CBUUID").unwrap(), UUIDWithData : theData)
    }
    unsafe fn UUIDWithCFUUID_(theUUID: CFUUIDRef) -> CBUUID
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CBUUID").unwrap(), UUIDWithCFUUID : theUUID)
    }
    unsafe fn UUIDWithNSUUID_(theUUID: NSUUID) -> CBUUID
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CBUUID").unwrap(), UUIDWithNSUUID : theUUID)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CBATTRequest(pub id);
impl std::ops::Deref for CBATTRequest {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CBATTRequest {}
impl CBATTRequest {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CBATTRequest").unwrap(), alloc) })
    }
}
impl INSObject for CBATTRequest {}
impl PNSObject for CBATTRequest {}
impl std::convert::TryFrom<NSObject> for CBATTRequest {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<CBATTRequest, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CBATTRequest").unwrap()) };
        if is_kind_of {
            Ok(CBATTRequest(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to CBATTRequest")
        }
    }
}
impl ICBATTRequest for CBATTRequest {}
pub trait ICBATTRequest: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn central(&self) -> CBCentral
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, central)
    }
    unsafe fn characteristic(&self) -> CBCharacteristic
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, characteristic)
    }
    unsafe fn offset(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, offset)
    }
    unsafe fn value(&self) -> NSData
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, value)
    }
    unsafe fn setValue_(&self, value: NSData)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setValue : value)
    }
}
unsafe extern "C" {
    pub static CBAdvertisementDataLocalNameKey: NSString;
}
unsafe extern "C" {
    pub static CBAdvertisementDataTxPowerLevelKey: NSString;
}
unsafe extern "C" {
    pub static CBAdvertisementDataServiceUUIDsKey: NSString;
}
unsafe extern "C" {
    pub static CBAdvertisementDataServiceDataKey: NSString;
}
unsafe extern "C" {
    pub static CBAdvertisementDataManufacturerDataKey: NSString;
}
unsafe extern "C" {
    pub static CBAdvertisementDataOverflowServiceUUIDsKey: NSString;
}
unsafe extern "C" {
    pub static CBAdvertisementDataIsConnectable: NSString;
}
unsafe extern "C" {
    pub static CBAdvertisementDataSolicitedServiceUUIDsKey: NSString;
}
unsafe extern "C" {
    pub static CBCentralManagerOptionShowPowerAlertKey: NSString;
}
unsafe extern "C" {
    pub static CBCentralManagerOptionRestoreIdentifierKey: NSString;
}
unsafe extern "C" {
    pub static CBCentralManagerOptionDeviceAccessForMedia: NSString;
}
unsafe extern "C" {
    pub static CBCentralManagerScanOptionAllowDuplicatesKey: NSString;
}
unsafe extern "C" {
    pub static CBCentralManagerScanOptionSolicitedServiceUUIDsKey: NSString;
}
unsafe extern "C" {
    pub static CBConnectPeripheralOptionNotifyOnConnectionKey: NSString;
}
unsafe extern "C" {
    pub static CBConnectPeripheralOptionNotifyOnDisconnectionKey: NSString;
}
unsafe extern "C" {
    pub static CBConnectPeripheralOptionNotifyOnNotificationKey: NSString;
}
unsafe extern "C" {
    pub static CBConnectPeripheralOptionStartDelayKey: NSString;
}
unsafe extern "C" {
    pub static CBConnectPeripheralOptionEnableTransportBridgingKey: NSString;
}
unsafe extern "C" {
    pub static CBConnectPeripheralOptionRequiresANCS: NSString;
}
unsafe extern "C" {
    pub static CBCentralManagerRestoredStatePeripheralsKey: NSString;
}
unsafe extern "C" {
    pub static CBCentralManagerRestoredStateScanServicesKey: NSString;
}
unsafe extern "C" {
    pub static CBCentralManagerRestoredStateScanOptionsKey: NSString;
}
unsafe extern "C" {
    pub static CBConnectionEventMatchingOptionServiceUUIDs: CBConnectionEventMatchingOption;
}
unsafe extern "C" {
    pub static CBConnectionEventMatchingOptionPeripheralUUIDs: CBConnectionEventMatchingOption;
}
unsafe extern "C" {
    pub static CBConnectPeripheralOptionEnableAutoReconnect: NSString;
}
unsafe extern "C" {
    pub static CBErrorDomain: NSString;
}
unsafe extern "C" {
    pub static CBATTErrorDomain: NSString;
}
unsafe extern "C" {
    pub static CBPeripheralManagerOptionShowPowerAlertKey: NSString;
}
unsafe extern "C" {
    pub static CBPeripheralManagerOptionRestoreIdentifierKey: NSString;
}
unsafe extern "C" {
    pub static CBPeripheralManagerRestoredStateServicesKey: NSString;
}
unsafe extern "C" {
    pub static CBPeripheralManagerRestoredStateAdvertisementDataKey: NSString;
}
unsafe extern "C" {
    pub static CBUUIDCharacteristicExtendedPropertiesString: NSString;
}
unsafe extern "C" {
    pub static CBUUIDCharacteristicUserDescriptionString: NSString;
}
unsafe extern "C" {
    pub static CBUUIDClientCharacteristicConfigurationString: NSString;
}
unsafe extern "C" {
    pub static CBUUIDServerCharacteristicConfigurationString: NSString;
}
unsafe extern "C" {
    pub static CBUUIDCharacteristicFormatString: NSString;
}
unsafe extern "C" {
    pub static CBUUIDCharacteristicAggregateFormatString: NSString;
}
unsafe extern "C" {
    pub static CBUUIDCharacteristicValidRangeString: NSString;
}
unsafe extern "C" {
    pub static CBUUIDCharacteristicObservationScheduleString: NSString;
}
unsafe extern "C" {
    pub static CBUUIDL2CAPPSMCharacteristicString: NSString;
}

unsafe impl objc2::encode::RefEncode for CBManager {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CBManager {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CBCentralManager {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CBCentralManager {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CBPeer {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CBPeer {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CBL2CAPChannel {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CBL2CAPChannel {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CBPeripheralManager {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CBPeripheralManager {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CBPeripheral {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CBPeripheral {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CBCentral {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CBCentral {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CBAttribute {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CBAttribute {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CBService {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CBService {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CBMutableService {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CBMutableService {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CBCharacteristic {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CBCharacteristic {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CBMutableCharacteristic {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CBMutableCharacteristic {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CBDescriptor {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CBDescriptor {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CBMutableDescriptor {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CBMutableDescriptor {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CBUUID {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CBUUID {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CBATTRequest {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CBATTRequest {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
