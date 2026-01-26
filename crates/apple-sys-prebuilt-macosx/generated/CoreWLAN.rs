#[allow(unused_imports)]
use crate::objc::*;
#[allow(unused_imports)]
use crate::CoreFoundation::*;
#[allow(unused_imports)]
use crate::Foundation::*;
#[allow(unused_imports)]
use crate::Security::*;
#[allow(unused_imports)]
use crate::SecurityFoundation::*;

#[allow(unused_imports)]
use objc2::msg_send;
pub type CWErr = NSInteger;
pub type CWPHYMode = NSInteger;
pub type CWInterfaceMode = NSInteger;
pub type CWSecurity = NSInteger;
pub type CWIBSSModeSecurity = NSInteger;
pub type CWChannelWidth = NSInteger;
pub type CWChannelBand = NSInteger;
pub type CWCipherKeyFlags = NSUInteger;
pub type CWKeychainDomain = NSInteger;
pub type CWEventType = NSInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CWInterface(pub id);
impl std::ops::Deref for CWInterface {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CWInterface {}
impl CWInterface {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CWInterface").unwrap(), alloc) })
    }
}
impl INSObject for CWInterface {}
impl PNSObject for CWInterface {}
impl std::convert::TryFrom<NSObject> for CWInterface {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<CWInterface, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CWInterface").unwrap()) };
        if is_kind_of {
            Ok(CWInterface(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to CWInterface")
        }
    }
}
impl ICWInterface for CWInterface {}
pub trait ICWInterface: Sized + std::ops::Deref {
    unsafe fn powerOn(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, powerOn)
    }
    unsafe fn supportedWLANChannels(&self) -> NSSet
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, supportedWLANChannels)
    }
    unsafe fn wlanChannel(&self) -> CWChannel
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, wlanChannel)
    }
    unsafe fn activePHYMode(&self) -> CWPHYMode
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, activePHYMode)
    }
    unsafe fn ssid(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, ssid)
    }
    unsafe fn ssidData(&self) -> NSData
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, ssidData)
    }
    unsafe fn bssid(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, bssid)
    }
    unsafe fn rssiValue(&self) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, rssiValue)
    }
    unsafe fn noiseMeasurement(&self) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, noiseMeasurement)
    }
    unsafe fn security(&self) -> CWSecurity
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, security)
    }
    unsafe fn transmitRate(&self) -> f64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, transmitRate)
    }
    unsafe fn countryCode(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, countryCode)
    }
    unsafe fn interfaceMode(&self) -> CWInterfaceMode
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, interfaceMode)
    }
    unsafe fn transmitPower(&self) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, transmitPower)
    }
    unsafe fn hardwareAddress(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, hardwareAddress)
    }
    unsafe fn serviceActive(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, serviceActive)
    }
    unsafe fn cachedScanResults(&self) -> NSSet
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, cachedScanResults)
    }
    unsafe fn configuration(&self) -> CWConfiguration
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, configuration)
    }
    unsafe fn initWithInterfaceName_(&self, name: NSString) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithInterfaceName : name)
    }
    unsafe fn setPower_error_(&self, power: BOOL, error: *mut NSError) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPower : power, error : error)
    }
    unsafe fn setWLANChannel_error_(&self, channel: CWChannel, error: *mut NSError) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setWLANChannel : channel, error : error)
    }
    unsafe fn setPairwiseMasterKey_error_(&self, key: NSData, error: *mut NSError) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPairwiseMasterKey : key, error : error)
    }
    unsafe fn setWEPKey_flags_index_error_(
        &self,
        key: NSData,
        flags: CWCipherKeyFlags,
        index: NSInteger,
        error: *mut NSError,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setWEPKey : key, flags : flags, index : index, error : error)
    }
    unsafe fn scanForNetworksWithSSID_error_(&self, ssid: NSData, error: *mut NSError) -> NSSet
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, scanForNetworksWithSSID : ssid, error : error)
    }
    unsafe fn scanForNetworksWithSSID_includeHidden_error_(
        &self,
        ssid: NSData,
        includeHidden: BOOL,
        error: *mut NSError,
    ) -> NSSet
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, scanForNetworksWithSSID : ssid, includeHidden : includeHidden, error : error)
    }
    unsafe fn scanForNetworksWithName_error_(
        &self,
        networkName: NSString,
        error: *mut NSError,
    ) -> NSSet
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, scanForNetworksWithName : networkName, error : error)
    }
    unsafe fn scanForNetworksWithName_includeHidden_error_(
        &self,
        networkName: NSString,
        includeHidden: BOOL,
        error: *mut NSError,
    ) -> NSSet
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, scanForNetworksWithName : networkName, includeHidden : includeHidden, error : error)
    }
    unsafe fn associateToNetwork_password_error_(
        &self,
        network: CWNetwork,
        password: NSString,
        error: *mut NSError,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, associateToNetwork : network, password : password, error : error)
    }
    unsafe fn disassociate(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, disassociate)
    }
    unsafe fn associateToEnterpriseNetwork_identity_username_password_error_(
        &self,
        network: CWNetwork,
        identity: SecIdentityRef,
        username: NSString,
        password: NSString,
        error: *mut NSError,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, associateToEnterpriseNetwork : network, identity : identity, username : username, password : password, error : error)
    }
    unsafe fn startIBSSModeWithSSID_security_channel_password_error_(
        &self,
        ssidData: NSData,
        security: CWIBSSModeSecurity,
        channel: NSUInteger,
        password: NSString,
        error: *mut NSError,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, startIBSSModeWithSSID : ssidData, security : security, channel : channel, password : password, error : error)
    }
    unsafe fn commitConfiguration_authorization_error_(
        &self,
        configuration: CWConfiguration,
        authorization: SFAuthorization,
        error: *mut NSError,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, commitConfiguration : configuration, authorization : authorization, error : error)
    }
    unsafe fn interfaceName(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, interfaceName)
    }
    unsafe fn interfaceNames() -> NSSet
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CWInterface").unwrap(), interfaceNames)
    }
    unsafe fn interface() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CWInterface").unwrap(), interface)
    }
    unsafe fn interfaceWithName_(name: NSString) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CWInterface").unwrap(), interfaceWithName : name)
    }
}
pub trait PCWEventDelegate: Sized + std::ops::Deref {
    unsafe fn clientConnectionInterrupted(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, clientConnectionInterrupted)
    }
    unsafe fn clientConnectionInvalidated(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, clientConnectionInvalidated)
    }
    unsafe fn powerStateDidChangeForWiFiInterfaceWithName_(&self, interfaceName: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, powerStateDidChangeForWiFiInterfaceWithName : interfaceName)
    }
    unsafe fn ssidDidChangeForWiFiInterfaceWithName_(&self, interfaceName: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, ssidDidChangeForWiFiInterfaceWithName : interfaceName)
    }
    unsafe fn bssidDidChangeForWiFiInterfaceWithName_(&self, interfaceName: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, bssidDidChangeForWiFiInterfaceWithName : interfaceName)
    }
    unsafe fn countryCodeDidChangeForWiFiInterfaceWithName_(&self, interfaceName: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, countryCodeDidChangeForWiFiInterfaceWithName : interfaceName)
    }
    unsafe fn linkDidChangeForWiFiInterfaceWithName_(&self, interfaceName: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, linkDidChangeForWiFiInterfaceWithName : interfaceName)
    }
    unsafe fn linkQualityDidChangeForWiFiInterfaceWithName_rssi_transmitRate_(
        &self,
        interfaceName: NSString,
        rssi: NSInteger,
        transmitRate: f64,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, linkQualityDidChangeForWiFiInterfaceWithName : interfaceName, rssi : rssi, transmitRate : transmitRate)
    }
    unsafe fn modeDidChangeForWiFiInterfaceWithName_(&self, interfaceName: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, modeDidChangeForWiFiInterfaceWithName : interfaceName)
    }
    unsafe fn scanCacheUpdatedForWiFiInterfaceWithName_(&self, interfaceName: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, scanCacheUpdatedForWiFiInterfaceWithName : interfaceName)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CWWiFiClient(pub id);
impl std::ops::Deref for CWWiFiClient {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CWWiFiClient {}
impl CWWiFiClient {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CWWiFiClient").unwrap(), alloc) })
    }
}
impl INSObject for CWWiFiClient {}
impl PNSObject for CWWiFiClient {}
impl std::convert::TryFrom<NSObject> for CWWiFiClient {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<CWWiFiClient, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CWWiFiClient").unwrap()) };
        if is_kind_of {
            Ok(CWWiFiClient(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to CWWiFiClient")
        }
    }
}
impl ICWWiFiClient for CWWiFiClient {}
pub trait ICWWiFiClient: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn interface(&self) -> CWInterface
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, interface)
    }
    unsafe fn interfaceNames(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, interfaceNames)
    }
    unsafe fn interfaceWithName_(&self, interfaceName: NSString) -> CWInterface
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, interfaceWithName : interfaceName)
    }
    unsafe fn interfaces(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, interfaces)
    }
    unsafe fn startMonitoringEventWithType_error_(
        &self,
        type_: CWEventType,
        error: *mut NSError,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, startMonitoringEventWithType : type_, error : error)
    }
    unsafe fn stopMonitoringEventWithType_error_(
        &self,
        type_: CWEventType,
        error: *mut NSError,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, stopMonitoringEventWithType : type_, error : error)
    }
    unsafe fn stopMonitoringAllEventsAndReturnError_(&self, error: *mut NSError) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, stopMonitoringAllEventsAndReturnError : error)
    }
    unsafe fn delegate(&self) -> id
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, delegate)
    }
    unsafe fn setDelegate_(&self, delegate: id)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDelegate : delegate)
    }
    unsafe fn sharedWiFiClient() -> CWWiFiClient
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CWWiFiClient").unwrap(), sharedWiFiClient)
    }
    unsafe fn class_interfaceNames() -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CWWiFiClient").unwrap(), interfaceNames)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CWNetwork(pub id);
impl std::ops::Deref for CWNetwork {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CWNetwork {}
impl CWNetwork {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CWNetwork").unwrap(), alloc) })
    }
}
impl PNSCopying for CWNetwork {}
impl PNSSecureCoding for CWNetwork {}
impl INSObject for CWNetwork {}
impl PNSObject for CWNetwork {}
impl std::convert::TryFrom<NSObject> for CWNetwork {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<CWNetwork, Self::Error> {
        let is_kind_of: bool = unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CWNetwork").unwrap()) };
        if is_kind_of {
            Ok(CWNetwork(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to CWNetwork")
        }
    }
}
impl ICWNetwork for CWNetwork {}
pub trait ICWNetwork: Sized + std::ops::Deref {
    unsafe fn isEqualToNetwork_(&self, network: CWNetwork) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, isEqualToNetwork : network)
    }
    unsafe fn supportsSecurity_(&self, security: CWSecurity) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, supportsSecurity : security)
    }
    unsafe fn supportsPHYMode_(&self, phyMode: CWPHYMode) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, supportsPHYMode : phyMode)
    }
    unsafe fn ssid(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, ssid)
    }
    unsafe fn ssidData(&self) -> NSData
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, ssidData)
    }
    unsafe fn bssid(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, bssid)
    }
    unsafe fn wlanChannel(&self) -> CWChannel
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, wlanChannel)
    }
    unsafe fn rssiValue(&self) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, rssiValue)
    }
    unsafe fn noiseMeasurement(&self) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, noiseMeasurement)
    }
    unsafe fn informationElementData(&self) -> NSData
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, informationElementData)
    }
    unsafe fn countryCode(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, countryCode)
    }
    unsafe fn beaconInterval(&self) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, beaconInterval)
    }
    unsafe fn ibss(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, ibss)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CWConfiguration(pub id);
impl std::ops::Deref for CWConfiguration {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CWConfiguration {}
impl CWConfiguration {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CWConfiguration").unwrap(), alloc) })
    }
}
impl PNSCopying for CWConfiguration {}
impl PNSMutableCopying for CWConfiguration {}
impl PNSSecureCoding for CWConfiguration {}
impl INSObject for CWConfiguration {}
impl PNSObject for CWConfiguration {}
impl std::convert::TryFrom<NSObject> for CWConfiguration {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<CWConfiguration, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CWConfiguration").unwrap()) };
        if is_kind_of {
            Ok(CWConfiguration(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to CWConfiguration")
        }
    }
}
impl ICWConfiguration for CWConfiguration {}
pub trait ICWConfiguration: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn initWithConfiguration_(&self, configuration: CWConfiguration) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithConfiguration : configuration)
    }
    unsafe fn isEqualToConfiguration_(&self, configuration: CWConfiguration) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, isEqualToConfiguration : configuration)
    }
    unsafe fn networkProfiles(&self) -> NSOrderedSet
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, networkProfiles)
    }
    unsafe fn requireAdministratorForAssociation(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, requireAdministratorForAssociation)
    }
    unsafe fn requireAdministratorForPower(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, requireAdministratorForPower)
    }
    unsafe fn requireAdministratorForIBSSMode(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, requireAdministratorForIBSSMode)
    }
    unsafe fn rememberJoinedNetworks(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, rememberJoinedNetworks)
    }
    unsafe fn configuration() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CWConfiguration").unwrap(), configuration)
    }
    unsafe fn configurationWithConfiguration_(configuration: CWConfiguration) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CWConfiguration").unwrap(), configurationWithConfiguration : configuration)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CWMutableConfiguration(pub id);
impl std::ops::Deref for CWMutableConfiguration {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CWMutableConfiguration {}
impl CWMutableConfiguration {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CWMutableConfiguration").unwrap(), alloc) })
    }
}
impl ICWConfiguration for CWMutableConfiguration {}
impl PNSCopying for CWMutableConfiguration {}
impl PNSMutableCopying for CWMutableConfiguration {}
impl PNSSecureCoding for CWMutableConfiguration {}
impl From<CWMutableConfiguration> for CWConfiguration {
    fn from(child: CWMutableConfiguration) -> CWConfiguration {
        CWConfiguration(child.0)
    }
}
impl std::convert::TryFrom<CWConfiguration> for CWMutableConfiguration {
    type Error = &'static str;
    fn try_from(parent: CWConfiguration) -> Result<CWMutableConfiguration, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CWMutableConfiguration").unwrap()) };
        if is_kind_of {
            Ok(CWMutableConfiguration(parent.0))
        } else {
            Err("This CWConfiguration cannot be downcasted to CWMutableConfiguration")
        }
    }
}
impl INSObject for CWMutableConfiguration {}
impl PNSObject for CWMutableConfiguration {}
impl ICWMutableConfiguration for CWMutableConfiguration {}
pub trait ICWMutableConfiguration: Sized + std::ops::Deref {
    unsafe fn networkProfiles(&self) -> NSOrderedSet
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, networkProfiles)
    }
    unsafe fn setNetworkProfiles_(&self, networkProfiles: NSOrderedSet)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setNetworkProfiles : networkProfiles)
    }
    unsafe fn requireAdministratorForAssociation(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, requireAdministratorForAssociation)
    }
    unsafe fn setRequireAdministratorForAssociation_(
        &self,
        requireAdministratorForAssociation: BOOL,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRequireAdministratorForAssociation : requireAdministratorForAssociation)
    }
    unsafe fn requireAdministratorForPower(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, requireAdministratorForPower)
    }
    unsafe fn setRequireAdministratorForPower_(&self, requireAdministratorForPower: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRequireAdministratorForPower : requireAdministratorForPower)
    }
    unsafe fn requireAdministratorForIBSSMode(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, requireAdministratorForIBSSMode)
    }
    unsafe fn setRequireAdministratorForIBSSMode_(&self, requireAdministratorForIBSSMode: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRequireAdministratorForIBSSMode : requireAdministratorForIBSSMode)
    }
    unsafe fn rememberJoinedNetworks(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, rememberJoinedNetworks)
    }
    unsafe fn setRememberJoinedNetworks_(&self, rememberJoinedNetworks: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRememberJoinedNetworks : rememberJoinedNetworks)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CWNetworkProfile(pub id);
impl std::ops::Deref for CWNetworkProfile {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CWNetworkProfile {}
impl CWNetworkProfile {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CWNetworkProfile").unwrap(), alloc) })
    }
}
impl PNSCopying for CWNetworkProfile {}
impl PNSMutableCopying for CWNetworkProfile {}
impl PNSSecureCoding for CWNetworkProfile {}
impl INSObject for CWNetworkProfile {}
impl PNSObject for CWNetworkProfile {}
impl std::convert::TryFrom<NSObject> for CWNetworkProfile {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<CWNetworkProfile, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CWNetworkProfile").unwrap()) };
        if is_kind_of {
            Ok(CWNetworkProfile(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to CWNetworkProfile")
        }
    }
}
impl ICWNetworkProfile for CWNetworkProfile {}
pub trait ICWNetworkProfile: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn initWithNetworkProfile_(&self, networkProfile: CWNetworkProfile) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithNetworkProfile : networkProfile)
    }
    unsafe fn isEqualToNetworkProfile_(&self, networkProfile: CWNetworkProfile) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, isEqualToNetworkProfile : networkProfile)
    }
    unsafe fn ssid(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, ssid)
    }
    unsafe fn ssidData(&self) -> NSData
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, ssidData)
    }
    unsafe fn security(&self) -> CWSecurity
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, security)
    }
    unsafe fn networkProfile() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CWNetworkProfile").unwrap(), networkProfile)
    }
    unsafe fn networkProfileWithNetworkProfile_(networkProfile: CWNetworkProfile) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CWNetworkProfile").unwrap(), networkProfileWithNetworkProfile : networkProfile)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CWMutableNetworkProfile(pub id);
impl std::ops::Deref for CWMutableNetworkProfile {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CWMutableNetworkProfile {}
impl CWMutableNetworkProfile {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CWMutableNetworkProfile").unwrap(), alloc) })
    }
}
impl ICWNetworkProfile for CWMutableNetworkProfile {}
impl PNSCopying for CWMutableNetworkProfile {}
impl PNSMutableCopying for CWMutableNetworkProfile {}
impl PNSSecureCoding for CWMutableNetworkProfile {}
impl From<CWMutableNetworkProfile> for CWNetworkProfile {
    fn from(child: CWMutableNetworkProfile) -> CWNetworkProfile {
        CWNetworkProfile(child.0)
    }
}
impl std::convert::TryFrom<CWNetworkProfile> for CWMutableNetworkProfile {
    type Error = &'static str;
    fn try_from(parent: CWNetworkProfile) -> Result<CWMutableNetworkProfile, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CWMutableNetworkProfile").unwrap()) };
        if is_kind_of {
            Ok(CWMutableNetworkProfile(parent.0))
        } else {
            Err("This CWNetworkProfile cannot be downcasted to CWMutableNetworkProfile")
        }
    }
}
impl INSObject for CWMutableNetworkProfile {}
impl PNSObject for CWMutableNetworkProfile {}
impl ICWMutableNetworkProfile for CWMutableNetworkProfile {}
pub trait ICWMutableNetworkProfile: Sized + std::ops::Deref {
    unsafe fn ssidData(&self) -> NSData
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, ssidData)
    }
    unsafe fn setSsidData_(&self, ssidData: NSData)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSsidData : ssidData)
    }
    unsafe fn security(&self) -> CWSecurity
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, security)
    }
    unsafe fn setSecurity_(&self, security: CWSecurity)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSecurity : security)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CWChannel(pub id);
impl std::ops::Deref for CWChannel {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CWChannel {}
impl CWChannel {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CWChannel").unwrap(), alloc) })
    }
}
impl PNSCopying for CWChannel {}
impl PNSSecureCoding for CWChannel {}
impl INSObject for CWChannel {}
impl PNSObject for CWChannel {}
impl std::convert::TryFrom<NSObject> for CWChannel {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<CWChannel, Self::Error> {
        let is_kind_of: bool = unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CWChannel").unwrap()) };
        if is_kind_of {
            Ok(CWChannel(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to CWChannel")
        }
    }
}
impl ICWChannel for CWChannel {}
pub trait ICWChannel: Sized + std::ops::Deref {
    unsafe fn isEqualToChannel_(&self, channel: CWChannel) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, isEqualToChannel : channel)
    }
    unsafe fn channelNumber(&self) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, channelNumber)
    }
    unsafe fn channelWidth(&self) -> CWChannelWidth
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, channelWidth)
    }
    unsafe fn channelBand(&self) -> CWChannelBand
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, channelBand)
    }
}
unsafe extern "C" {
    pub static CWErrorDomain: NSString;
}
unsafe extern "C" {
    pub static CWPowerDidChangeNotification: NSString;
}
unsafe extern "C" {
    pub static CWSSIDDidChangeNotification: NSString;
}
unsafe extern "C" {
    pub static CWBSSIDDidChangeNotification: NSString;
}
unsafe extern "C" {
    pub static CWLinkDidChangeNotification: NSString;
}
unsafe extern "C" {
    pub static CWModeDidChangeNotification: NSString;
}
unsafe extern "C" {
    pub static CWCountryCodeDidChangeNotification: NSString;
}
unsafe extern "C" {
    pub static CWScanCacheDidUpdateNotification: NSString;
}
unsafe extern "C" {
    pub static CWLinkQualityDidChangeNotification: NSString;
}
unsafe extern "C" {
    pub static CWLinkQualityNotificationRSSIKey: NSString;
}
unsafe extern "C" {
    pub static CWLinkQualityNotificationTransmitRateKey: NSString;
}
unsafe extern "C" {
    pub fn CWKeychainFindWiFiPassword(
        domain: CWKeychainDomain,
        ssid: NSData,
        password: *mut NSString,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn CWKeychainSetWiFiPassword(
        domain: CWKeychainDomain,
        ssid: NSData,
        password: NSString,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn CWKeychainDeleteWiFiPassword(domain: CWKeychainDomain, ssid: NSData) -> OSStatus;
}
unsafe extern "C" {
    pub fn CWKeychainFindWiFiEAPUsernameAndPassword(
        domain: CWKeychainDomain,
        ssid: NSData,
        username: *mut NSString,
        password: *mut NSString,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn CWKeychainSetWiFiEAPUsernameAndPassword(
        domain: CWKeychainDomain,
        ssid: NSData,
        username: NSString,
        password: NSString,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn CWKeychainDeleteWiFiEAPUsernameAndPassword(
        domain: CWKeychainDomain,
        ssid: NSData,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn CWKeychainCopyWiFiEAPIdentity(
        domain: CWKeychainDomain,
        ssid: NSData,
        identity: *mut SecIdentityRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn CWKeychainSetWiFiEAPIdentity(
        domain: CWKeychainDomain,
        ssid: NSData,
        identity: SecIdentityRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn CWKeychainCopyEAPIdentityList(list: *mut CFArrayRef) -> OSStatus;
}
unsafe extern "C" {
    pub fn CWKeychainCopyEAPUsernameAndPassword(
        ssidData: CFDataRef,
        username: *mut CFStringRef,
        password: *mut CFStringRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn CWKeychainSetEAPUsernameAndPassword(
        ssidData: CFDataRef,
        username: CFStringRef,
        password: CFStringRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn CWKeychainDeleteEAPUsernameAndPassword(ssidData: CFDataRef) -> OSStatus;
}
unsafe extern "C" {
    pub fn CWKeychainCopyEAPIdentity(
        ssidData: CFDataRef,
        identity: *mut SecIdentityRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn CWKeychainSetEAPIdentity(ssidData: CFDataRef, identity: SecIdentityRef) -> OSStatus;
}
unsafe extern "C" {
    pub fn CWKeychainSetPassword(ssidData: CFDataRef, password: CFStringRef) -> OSStatus;
}
unsafe extern "C" {
    pub fn CWKeychainCopyPassword(ssidData: CFDataRef, password: *mut CFStringRef) -> OSStatus;
}
unsafe extern "C" {
    pub fn CWKeychainDeletePassword(ssidData: CFDataRef) -> OSStatus;
}
unsafe extern "C" {
    pub fn CWMergeNetworks(networks: NSSet) -> NSSet;
}

unsafe impl objc2::encode::RefEncode for CWInterface {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CWInterface {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CWWiFiClient {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CWWiFiClient {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CWNetwork {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CWNetwork {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CWConfiguration {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CWConfiguration {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CWMutableConfiguration {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CWMutableConfiguration {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CWNetworkProfile {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CWNetworkProfile {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CWMutableNetworkProfile {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CWMutableNetworkProfile {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CWChannel {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CWChannel {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
