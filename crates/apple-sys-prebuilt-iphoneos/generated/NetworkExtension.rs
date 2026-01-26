#[allow(unused_imports)]
use crate::objc::*;
#[allow(unused_imports)]
use crate::AccessorySetupKit::*;
#[allow(unused_imports)]
use crate::Foundation::*;
#[allow(unused_imports)]
use crate::Network::*;
#[allow(unused_imports)]
use crate::Security::*;
#[allow(unused_imports)]
use crate::SystemConfiguration::*;
#[allow(unused_imports)]
use libc::sa_family_t;

#[allow(unused_imports)]
use objc2::msg_send;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AuthorizationOpaqueRef {
    _unused: [u8; 0],
}
pub type NEAppProxyFlowError = NSInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct NEAppProxyFlow(pub id);
impl std::ops::Deref for NEAppProxyFlow {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for NEAppProxyFlow {}
impl NEAppProxyFlow {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"NEAppProxyFlow").unwrap(), alloc) })
    }
}
impl INSObject for NEAppProxyFlow {}
impl PNSObject for NEAppProxyFlow {}
impl std::convert::TryFrom<NSObject> for NEAppProxyFlow {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<NEAppProxyFlow, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"NEAppProxyFlow").unwrap()) };
        if is_kind_of {
            Ok(NEAppProxyFlow(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to NEAppProxyFlow")
        }
    }
}
impl INEAppProxyFlow for NEAppProxyFlow {}
pub trait INEAppProxyFlow: Sized + std::ops::Deref {
    unsafe fn openWithLocalFlowEndpoint_completionHandler_(
        &self,
        localEndpoint: NSObject,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, openWithLocalFlowEndpoint : localEndpoint, completionHandler : completionHandler)
    }
    unsafe fn openWithLocalEndpoint_completionHandler_(
        &self,
        localEndpoint: NWHostEndpoint,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, openWithLocalEndpoint : localEndpoint, completionHandler : completionHandler)
    }
    unsafe fn closeReadWithError_(&self, error: NSError)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, closeReadWithError : error)
    }
    unsafe fn closeWriteWithError_(&self, error: NSError)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, closeWriteWithError : error)
    }
    unsafe fn setMetadata_(&self, parameters: NSObject)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMetadata : parameters)
    }
    unsafe fn metaData(&self) -> NEFlowMetaData
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, metaData)
    }
    unsafe fn networkInterface(&self) -> nw_interface_t
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, networkInterface)
    }
    unsafe fn setNetworkInterface_(&self, networkInterface: NSObject)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setNetworkInterface : networkInterface)
    }
    unsafe fn remoteHostname(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, remoteHostname)
    }
    unsafe fn isBound(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isBound)
    }
}
pub type NEProviderStopReason = NSInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct NEProvider(pub id);
impl std::ops::Deref for NEProvider {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for NEProvider {}
impl NEProvider {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"NEProvider").unwrap(), alloc) })
    }
}
impl INSObject for NEProvider {}
impl PNSObject for NEProvider {}
impl std::convert::TryFrom<NSObject> for NEProvider {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<NEProvider, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"NEProvider").unwrap()) };
        if is_kind_of {
            Ok(NEProvider(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to NEProvider")
        }
    }
}
impl INEProvider for NEProvider {}
pub trait INEProvider: Sized + std::ops::Deref {
    unsafe fn sleepWithCompletionHandler_(&self, completionHandler: *mut ::std::os::raw::c_void)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, sleepWithCompletionHandler : completionHandler)
    }
    unsafe fn wake(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, wake)
    }
    unsafe fn createTCPConnectionToEndpoint_enableTLS_TLSParameters_delegate_(
        &self,
        remoteEndpoint: NWEndpoint,
        enableTLS: BOOL,
        TLSParameters: NWTLSParameters,
        delegate: id,
    ) -> NWTCPConnection
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, createTCPConnectionToEndpoint : remoteEndpoint, enableTLS : enableTLS, TLSParameters : TLSParameters, delegate : delegate)
    }
    unsafe fn createUDPSessionToEndpoint_fromEndpoint_(
        &self,
        remoteEndpoint: NWEndpoint,
        localEndpoint: NWHostEndpoint,
    ) -> NWUDPSession
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, createUDPSessionToEndpoint : remoteEndpoint, fromEndpoint : localEndpoint)
    }
    unsafe fn displayMessage_completionHandler_(
        &self,
        message: NSString,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, displayMessage : message, completionHandler : completionHandler)
    }
    unsafe fn defaultPath(&self) -> NWPath
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, defaultPath)
    }
    unsafe fn startSystemExtensionMode()
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NEProvider").unwrap(), startSystemExtensionMode)
    }
}
pub type NETunnelProviderError = NSInteger;
pub type NETunnelProviderRoutingMethod = NSInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct NETunnelProvider(pub id);
impl std::ops::Deref for NETunnelProvider {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for NETunnelProvider {}
impl NETunnelProvider {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"NETunnelProvider").unwrap(), alloc) })
    }
}
impl INEProvider for NETunnelProvider {}
impl From<NETunnelProvider> for NEProvider {
    fn from(child: NETunnelProvider) -> NEProvider {
        NEProvider(child.0)
    }
}
impl std::convert::TryFrom<NEProvider> for NETunnelProvider {
    type Error = &'static str;
    fn try_from(parent: NEProvider) -> Result<NETunnelProvider, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"NETunnelProvider").unwrap()) };
        if is_kind_of {
            Ok(NETunnelProvider(parent.0))
        } else {
            Err("This NEProvider cannot be downcasted to NETunnelProvider")
        }
    }
}
impl INSObject for NETunnelProvider {}
impl PNSObject for NETunnelProvider {}
impl INETunnelProvider for NETunnelProvider {}
pub trait INETunnelProvider: Sized + std::ops::Deref {
    unsafe fn handleAppMessage_completionHandler_(
        &self,
        messageData: NSData,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, handleAppMessage : messageData, completionHandler : completionHandler)
    }
    unsafe fn setTunnelNetworkSettings_completionHandler_(
        &self,
        tunnelNetworkSettings: NETunnelNetworkSettings,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setTunnelNetworkSettings : tunnelNetworkSettings, completionHandler : completionHandler)
    }
    unsafe fn protocolConfiguration(&self) -> NEVPNProtocol
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, protocolConfiguration)
    }
    unsafe fn appRules(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, appRules)
    }
    unsafe fn routingMethod(&self) -> NETunnelProviderRoutingMethod
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, routingMethod)
    }
    unsafe fn reasserting(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, reasserting)
    }
    unsafe fn setReasserting_(&self, reasserting: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setReasserting : reasserting)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct NEAppProxyProvider(pub id);
impl std::ops::Deref for NEAppProxyProvider {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for NEAppProxyProvider {}
impl NEAppProxyProvider {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"NEAppProxyProvider").unwrap(), alloc) })
    }
}
impl INETunnelProvider for NEAppProxyProvider {}
impl From<NEAppProxyProvider> for NETunnelProvider {
    fn from(child: NEAppProxyProvider) -> NETunnelProvider {
        NETunnelProvider(child.0)
    }
}
impl std::convert::TryFrom<NETunnelProvider> for NEAppProxyProvider {
    type Error = &'static str;
    fn try_from(parent: NETunnelProvider) -> Result<NEAppProxyProvider, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"NEAppProxyProvider").unwrap()) };
        if is_kind_of {
            Ok(NEAppProxyProvider(parent.0))
        } else {
            Err("This NETunnelProvider cannot be downcasted to NEAppProxyProvider")
        }
    }
}
impl INEProvider for NEAppProxyProvider {}
impl INSObject for NEAppProxyProvider {}
impl PNSObject for NEAppProxyProvider {}
impl INEAppProxyProvider for NEAppProxyProvider {}
pub trait INEAppProxyProvider: Sized + std::ops::Deref {
    unsafe fn startProxyWithOptions_completionHandler_(
        &self,
        options: NSDictionary,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, startProxyWithOptions : options, completionHandler : completionHandler)
    }
    unsafe fn stopProxyWithReason_completionHandler_(
        &self,
        reason: NEProviderStopReason,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, stopProxyWithReason : reason, completionHandler : completionHandler)
    }
    unsafe fn cancelProxyWithError_(&self, error: NSError)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, cancelProxyWithError : error)
    }
    unsafe fn handleNewFlow_(&self, flow: NEAppProxyFlow) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, handleNewFlow : flow)
    }
    unsafe fn handleNewUDPFlow_initialRemoteFlowEndpoint_(
        &self,
        flow: NEAppProxyUDPFlow,
        remoteEndpoint: NSObject,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, handleNewUDPFlow : flow, initialRemoteFlowEndpoint : remoteEndpoint)
    }
    unsafe fn handleNewUDPFlow_initialRemoteEndpoint_(
        &self,
        flow: NEAppProxyUDPFlow,
        remoteEndpoint: NWEndpoint,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, handleNewUDPFlow : flow, initialRemoteEndpoint : remoteEndpoint)
    }
}
pub type NEVPNError = NSInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct NEVPNManager(pub id);
impl std::ops::Deref for NEVPNManager {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for NEVPNManager {}
impl NEVPNManager {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"NEVPNManager").unwrap(), alloc) })
    }
}
impl INSObject for NEVPNManager {}
impl PNSObject for NEVPNManager {}
impl std::convert::TryFrom<NSObject> for NEVPNManager {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<NEVPNManager, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"NEVPNManager").unwrap()) };
        if is_kind_of {
            Ok(NEVPNManager(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to NEVPNManager")
        }
    }
}
impl INEVPNManager for NEVPNManager {}
pub trait INEVPNManager: Sized + std::ops::Deref {
    unsafe fn loadFromPreferencesWithCompletionHandler_(
        &self,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, loadFromPreferencesWithCompletionHandler : completionHandler)
    }
    unsafe fn removeFromPreferencesWithCompletionHandler_(
        &self,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, removeFromPreferencesWithCompletionHandler : completionHandler)
    }
    unsafe fn saveToPreferencesWithCompletionHandler_(
        &self,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, saveToPreferencesWithCompletionHandler : completionHandler)
    }
    unsafe fn setAuthorization_(&self, authorization: AuthorizationRef)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAuthorization : authorization)
    }
    unsafe fn onDemandRules(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, onDemandRules)
    }
    unsafe fn setOnDemandRules_(&self, onDemandRules: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setOnDemandRules : onDemandRules)
    }
    unsafe fn isOnDemandEnabled(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isOnDemandEnabled)
    }
    unsafe fn setOnDemandEnabled_(&self, onDemandEnabled: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setOnDemandEnabled : onDemandEnabled)
    }
    unsafe fn localizedDescription(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, localizedDescription)
    }
    unsafe fn setLocalizedDescription_(&self, localizedDescription: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setLocalizedDescription : localizedDescription)
    }
    unsafe fn protocol(&self) -> NEVPNProtocol
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, protocol)
    }
    unsafe fn setProtocol_(&self, protocol: NEVPNProtocol)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setProtocol : protocol)
    }
    unsafe fn protocolConfiguration(&self) -> NEVPNProtocol
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, protocolConfiguration)
    }
    unsafe fn setProtocolConfiguration_(&self, protocolConfiguration: NEVPNProtocol)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setProtocolConfiguration : protocolConfiguration)
    }
    unsafe fn connection(&self) -> NEVPNConnection
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, connection)
    }
    unsafe fn isEnabled(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isEnabled)
    }
    unsafe fn setEnabled_(&self, enabled: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setEnabled : enabled)
    }
    unsafe fn sharedManager() -> NEVPNManager
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NEVPNManager").unwrap(), sharedManager)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct NETunnelProviderManager(pub id);
impl std::ops::Deref for NETunnelProviderManager {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for NETunnelProviderManager {}
impl NETunnelProviderManager {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"NETunnelProviderManager").unwrap(), alloc) })
    }
}
impl INEVPNManager for NETunnelProviderManager {}
impl From<NETunnelProviderManager> for NEVPNManager {
    fn from(child: NETunnelProviderManager) -> NEVPNManager {
        NEVPNManager(child.0)
    }
}
impl std::convert::TryFrom<NEVPNManager> for NETunnelProviderManager {
    type Error = &'static str;
    fn try_from(parent: NEVPNManager) -> Result<NETunnelProviderManager, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"NETunnelProviderManager").unwrap()) };
        if is_kind_of {
            Ok(NETunnelProviderManager(parent.0))
        } else {
            Err("This NEVPNManager cannot be downcasted to NETunnelProviderManager")
        }
    }
}
impl INSObject for NETunnelProviderManager {}
impl PNSObject for NETunnelProviderManager {}
impl INETunnelProviderManager for NETunnelProviderManager {}
pub trait INETunnelProviderManager: Sized + std::ops::Deref {
    unsafe fn copyAppRules(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, copyAppRules)
    }
    unsafe fn routingMethod(&self) -> NETunnelProviderRoutingMethod
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, routingMethod)
    }
    unsafe fn safariDomains(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, safariDomains)
    }
    unsafe fn setSafariDomains_(&self, safariDomains: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSafariDomains : safariDomains)
    }
    unsafe fn mailDomains(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, mailDomains)
    }
    unsafe fn setMailDomains_(&self, mailDomains: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMailDomains : mailDomains)
    }
    unsafe fn calendarDomains(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, calendarDomains)
    }
    unsafe fn setCalendarDomains_(&self, calendarDomains: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCalendarDomains : calendarDomains)
    }
    unsafe fn contactsDomains(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, contactsDomains)
    }
    unsafe fn setContactsDomains_(&self, contactsDomains: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setContactsDomains : contactsDomains)
    }
    unsafe fn appRules(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, appRules)
    }
    unsafe fn setAppRules_(&self, appRules: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAppRules : appRules)
    }
    unsafe fn excludedDomains(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, excludedDomains)
    }
    unsafe fn setExcludedDomains_(&self, excludedDomains: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setExcludedDomains : excludedDomains)
    }
    unsafe fn associatedDomains(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, associatedDomains)
    }
    unsafe fn setAssociatedDomains_(&self, associatedDomains: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAssociatedDomains : associatedDomains)
    }
    unsafe fn loadAllFromPreferencesWithCompletionHandler_(
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NETunnelProviderManager").unwrap(), loadAllFromPreferencesWithCompletionHandler : completionHandler)
    }
    unsafe fn forPerAppVPN() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NETunnelProviderManager").unwrap(), forPerAppVPN)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct NEAppProxyProviderManager(pub id);
impl std::ops::Deref for NEAppProxyProviderManager {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for NEAppProxyProviderManager {}
impl NEAppProxyProviderManager {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"NEAppProxyProviderManager").unwrap(), alloc) })
    }
}
impl INETunnelProviderManager for NEAppProxyProviderManager {}
impl From<NEAppProxyProviderManager> for NETunnelProviderManager {
    fn from(child: NEAppProxyProviderManager) -> NETunnelProviderManager {
        NETunnelProviderManager(child.0)
    }
}
impl std::convert::TryFrom<NETunnelProviderManager> for NEAppProxyProviderManager {
    type Error = &'static str;
    fn try_from(parent: NETunnelProviderManager) -> Result<NEAppProxyProviderManager, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"NEAppProxyProviderManager").unwrap()) };
        if is_kind_of {
            Ok(NEAppProxyProviderManager(parent.0))
        } else {
            Err("This NETunnelProviderManager cannot be downcasted to NEAppProxyProviderManager")
        }
    }
}
impl INEVPNManager for NEAppProxyProviderManager {}
impl INSObject for NEAppProxyProviderManager {}
impl PNSObject for NEAppProxyProviderManager {}
impl INEAppProxyProviderManager for NEAppProxyProviderManager {}
pub trait INEAppProxyProviderManager: Sized + std::ops::Deref {
    unsafe fn loadAllFromPreferencesWithCompletionHandler_(
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NEAppProxyProviderManager").unwrap(), loadAllFromPreferencesWithCompletionHandler : completionHandler)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct NEAppProxyTCPFlow(pub id);
impl std::ops::Deref for NEAppProxyTCPFlow {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for NEAppProxyTCPFlow {}
impl NEAppProxyTCPFlow {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"NEAppProxyTCPFlow").unwrap(), alloc) })
    }
}
impl INEAppProxyFlow for NEAppProxyTCPFlow {}
impl From<NEAppProxyTCPFlow> for NEAppProxyFlow {
    fn from(child: NEAppProxyTCPFlow) -> NEAppProxyFlow {
        NEAppProxyFlow(child.0)
    }
}
impl std::convert::TryFrom<NEAppProxyFlow> for NEAppProxyTCPFlow {
    type Error = &'static str;
    fn try_from(parent: NEAppProxyFlow) -> Result<NEAppProxyTCPFlow, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"NEAppProxyTCPFlow").unwrap()) };
        if is_kind_of {
            Ok(NEAppProxyTCPFlow(parent.0))
        } else {
            Err("This NEAppProxyFlow cannot be downcasted to NEAppProxyTCPFlow")
        }
    }
}
impl INSObject for NEAppProxyTCPFlow {}
impl PNSObject for NEAppProxyTCPFlow {}
impl INEAppProxyTCPFlow for NEAppProxyTCPFlow {}
pub trait INEAppProxyTCPFlow: Sized + std::ops::Deref {
    unsafe fn readDataWithCompletionHandler_(&self, completionHandler: *mut ::std::os::raw::c_void)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, readDataWithCompletionHandler : completionHandler)
    }
    unsafe fn writeData_withCompletionHandler_(
        &self,
        data: NSData,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, writeData : data, withCompletionHandler : completionHandler)
    }
    unsafe fn remoteFlowEndpoint(&self) -> nw_endpoint_t
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, remoteFlowEndpoint)
    }
    unsafe fn remoteEndpoint(&self) -> NWEndpoint
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, remoteEndpoint)
    }
}
pub type NWEndpointArray = NSArray;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct NEAppProxyUDPFlow(pub id);
impl std::ops::Deref for NEAppProxyUDPFlow {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for NEAppProxyUDPFlow {}
impl NEAppProxyUDPFlow {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"NEAppProxyUDPFlow").unwrap(), alloc) })
    }
}
impl INEAppProxyFlow for NEAppProxyUDPFlow {}
impl std::convert::TryFrom<NEAppProxyFlow> for NEAppProxyUDPFlow {
    type Error = &'static str;
    fn try_from(parent: NEAppProxyFlow) -> Result<NEAppProxyUDPFlow, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"NEAppProxyUDPFlow").unwrap()) };
        if is_kind_of {
            Ok(NEAppProxyUDPFlow(parent.0))
        } else {
            Err("This NEAppProxyFlow cannot be downcasted to NEAppProxyUDPFlow")
        }
    }
}
impl INSObject for NEAppProxyUDPFlow {}
impl PNSObject for NEAppProxyUDPFlow {}
impl INEAppProxyUDPFlow for NEAppProxyUDPFlow {}
pub trait INEAppProxyUDPFlow: Sized + std::ops::Deref {
    unsafe fn readDatagramsAndFlowEndpointsWithCompletionHandler_(
        &self,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, readDatagramsAndFlowEndpointsWithCompletionHandler : completionHandler)
    }
    unsafe fn readDatagramsWithCompletionHandler_(
        &self,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, readDatagramsWithCompletionHandler : completionHandler)
    }
    unsafe fn writeDatagrams_sentByFlowEndpoints_completionHandler_(
        &self,
        datagrams: NSArray,
        remoteEndpoints: NSArray,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, writeDatagrams : datagrams, sentByFlowEndpoints : remoteEndpoints, completionHandler : completionHandler)
    }
    unsafe fn writeDatagrams_sentByEndpoints_completionHandler_(
        &self,
        datagrams: NSArray,
        remoteEndpoints: NSArray,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, writeDatagrams : datagrams, sentByEndpoints : remoteEndpoints, completionHandler : completionHandler)
    }
    unsafe fn localFlowEndpoint(&self) -> nw_endpoint_t
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, localFlowEndpoint)
    }
    unsafe fn localEndpoint(&self) -> NWEndpoint
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, localEndpoint)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct NEAppRule(pub id);
impl std::ops::Deref for NEAppRule {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for NEAppRule {}
impl NEAppRule {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"NEAppRule").unwrap(), alloc) })
    }
}
impl PNSSecureCoding for NEAppRule {}
impl PNSCopying for NEAppRule {}
impl INSObject for NEAppRule {}
impl PNSObject for NEAppRule {}
impl std::convert::TryFrom<NSObject> for NEAppRule {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<NEAppRule, Self::Error> {
        let is_kind_of: bool = unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"NEAppRule").unwrap()) };
        if is_kind_of {
            Ok(NEAppRule(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to NEAppRule")
        }
    }
}
impl INEAppRule for NEAppRule {}
pub trait INEAppRule: Sized + std::ops::Deref {
    unsafe fn initWithSigningIdentifier_(&self, signingIdentifier: NSString) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithSigningIdentifier : signingIdentifier)
    }
    unsafe fn initWithSigningIdentifier_designatedRequirement_(
        &self,
        signingIdentifier: NSString,
        designatedRequirement: NSString,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithSigningIdentifier : signingIdentifier, designatedRequirement : designatedRequirement)
    }
    unsafe fn matchSigningIdentifier(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, matchSigningIdentifier)
    }
    unsafe fn matchDesignatedRequirement(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, matchDesignatedRequirement)
    }
    unsafe fn matchPath(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, matchPath)
    }
    unsafe fn setMatchPath_(&self, matchPath: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMatchPath : matchPath)
    }
    unsafe fn matchDomains(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, matchDomains)
    }
    unsafe fn setMatchDomains_(&self, matchDomains: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMatchDomains : matchDomains)
    }
    unsafe fn matchTools(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, matchTools)
    }
    unsafe fn setMatchTools_(&self, matchTools: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMatchTools : matchTools)
    }
}
pub type NEDNSProxyManagerError = NSInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct NEDNSProxyManager(pub id);
impl std::ops::Deref for NEDNSProxyManager {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for NEDNSProxyManager {}
impl NEDNSProxyManager {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"NEDNSProxyManager").unwrap(), alloc) })
    }
}
impl INSObject for NEDNSProxyManager {}
impl PNSObject for NEDNSProxyManager {}
impl std::convert::TryFrom<NSObject> for NEDNSProxyManager {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<NEDNSProxyManager, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"NEDNSProxyManager").unwrap()) };
        if is_kind_of {
            Ok(NEDNSProxyManager(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to NEDNSProxyManager")
        }
    }
}
impl INEDNSProxyManager for NEDNSProxyManager {}
pub trait INEDNSProxyManager: Sized + std::ops::Deref {
    unsafe fn loadFromPreferencesWithCompletionHandler_(
        &self,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, loadFromPreferencesWithCompletionHandler : completionHandler)
    }
    unsafe fn removeFromPreferencesWithCompletionHandler_(
        &self,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, removeFromPreferencesWithCompletionHandler : completionHandler)
    }
    unsafe fn saveToPreferencesWithCompletionHandler_(
        &self,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, saveToPreferencesWithCompletionHandler : completionHandler)
    }
    unsafe fn localizedDescription(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, localizedDescription)
    }
    unsafe fn setLocalizedDescription_(&self, localizedDescription: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setLocalizedDescription : localizedDescription)
    }
    unsafe fn providerProtocol(&self) -> NEDNSProxyProviderProtocol
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, providerProtocol)
    }
    unsafe fn setProviderProtocol_(&self, providerProtocol: NEDNSProxyProviderProtocol)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setProviderProtocol : providerProtocol)
    }
    unsafe fn isEnabled(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isEnabled)
    }
    unsafe fn setEnabled_(&self, enabled: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setEnabled : enabled)
    }
    unsafe fn sharedManager() -> NEDNSProxyManager
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NEDNSProxyManager").unwrap(), sharedManager)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct NEDNSProxyProvider(pub id);
impl std::ops::Deref for NEDNSProxyProvider {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for NEDNSProxyProvider {}
impl NEDNSProxyProvider {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"NEDNSProxyProvider").unwrap(), alloc) })
    }
}
impl INEProvider for NEDNSProxyProvider {}
impl std::convert::TryFrom<NEProvider> for NEDNSProxyProvider {
    type Error = &'static str;
    fn try_from(parent: NEProvider) -> Result<NEDNSProxyProvider, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"NEDNSProxyProvider").unwrap()) };
        if is_kind_of {
            Ok(NEDNSProxyProvider(parent.0))
        } else {
            Err("This NEProvider cannot be downcasted to NEDNSProxyProvider")
        }
    }
}
impl INSObject for NEDNSProxyProvider {}
impl PNSObject for NEDNSProxyProvider {}
impl INEDNSProxyProvider for NEDNSProxyProvider {}
pub trait INEDNSProxyProvider: Sized + std::ops::Deref {
    unsafe fn startProxyWithOptions_completionHandler_(
        &self,
        options: NSDictionary,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, startProxyWithOptions : options, completionHandler : completionHandler)
    }
    unsafe fn stopProxyWithReason_completionHandler_(
        &self,
        reason: NEProviderStopReason,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, stopProxyWithReason : reason, completionHandler : completionHandler)
    }
    unsafe fn cancelProxyWithError_(&self, error: NSError)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, cancelProxyWithError : error)
    }
    unsafe fn handleNewFlow_(&self, flow: NEAppProxyFlow) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, handleNewFlow : flow)
    }
    unsafe fn handleNewUDPFlow_initialRemoteFlowEndpoint_(
        &self,
        flow: NEAppProxyUDPFlow,
        remoteEndpoint: NSObject,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, handleNewUDPFlow : flow, initialRemoteFlowEndpoint : remoteEndpoint)
    }
    unsafe fn handleNewUDPFlow_initialRemoteEndpoint_(
        &self,
        flow: NEAppProxyUDPFlow,
        remoteEndpoint: NWEndpoint,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, handleNewUDPFlow : flow, initialRemoteEndpoint : remoteEndpoint)
    }
    unsafe fn systemDNSSettings(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, systemDNSSettings)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct NEProxyServer(pub id);
impl std::ops::Deref for NEProxyServer {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for NEProxyServer {}
impl NEProxyServer {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"NEProxyServer").unwrap(), alloc) })
    }
}
impl PNSSecureCoding for NEProxyServer {}
impl PNSCopying for NEProxyServer {}
impl INSObject for NEProxyServer {}
impl PNSObject for NEProxyServer {}
impl std::convert::TryFrom<NSObject> for NEProxyServer {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<NEProxyServer, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"NEProxyServer").unwrap()) };
        if is_kind_of {
            Ok(NEProxyServer(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to NEProxyServer")
        }
    }
}
impl INEProxyServer for NEProxyServer {}
pub trait INEProxyServer: Sized + std::ops::Deref {
    unsafe fn initWithAddress_port_(&self, address: NSString, port: NSInteger) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithAddress : address, port : port)
    }
    unsafe fn address(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, address)
    }
    unsafe fn port(&self) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, port)
    }
    unsafe fn authenticationRequired(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, authenticationRequired)
    }
    unsafe fn setAuthenticationRequired_(&self, authenticationRequired: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAuthenticationRequired : authenticationRequired)
    }
    unsafe fn username(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, username)
    }
    unsafe fn setUsername_(&self, username: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setUsername : username)
    }
    unsafe fn password(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, password)
    }
    unsafe fn setPassword_(&self, password: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPassword : password)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct NEProxySettings(pub id);
impl std::ops::Deref for NEProxySettings {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for NEProxySettings {}
impl NEProxySettings {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"NEProxySettings").unwrap(), alloc) })
    }
}
impl PNSSecureCoding for NEProxySettings {}
impl PNSCopying for NEProxySettings {}
impl INSObject for NEProxySettings {}
impl PNSObject for NEProxySettings {}
impl std::convert::TryFrom<NSObject> for NEProxySettings {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<NEProxySettings, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"NEProxySettings").unwrap()) };
        if is_kind_of {
            Ok(NEProxySettings(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to NEProxySettings")
        }
    }
}
impl INEProxySettings for NEProxySettings {}
pub trait INEProxySettings: Sized + std::ops::Deref {
    unsafe fn autoProxyConfigurationEnabled(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, autoProxyConfigurationEnabled)
    }
    unsafe fn setAutoProxyConfigurationEnabled_(&self, autoProxyConfigurationEnabled: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAutoProxyConfigurationEnabled : autoProxyConfigurationEnabled)
    }
    unsafe fn proxyAutoConfigurationURL(&self) -> NSURL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, proxyAutoConfigurationURL)
    }
    unsafe fn setProxyAutoConfigurationURL_(&self, proxyAutoConfigurationURL: NSURL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setProxyAutoConfigurationURL : proxyAutoConfigurationURL)
    }
    unsafe fn proxyAutoConfigurationJavaScript(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, proxyAutoConfigurationJavaScript)
    }
    unsafe fn setProxyAutoConfigurationJavaScript_(
        &self,
        proxyAutoConfigurationJavaScript: NSString,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setProxyAutoConfigurationJavaScript : proxyAutoConfigurationJavaScript)
    }
    unsafe fn HTTPEnabled(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, HTTPEnabled)
    }
    unsafe fn setHTTPEnabled_(&self, HTTPEnabled: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setHTTPEnabled : HTTPEnabled)
    }
    unsafe fn HTTPServer(&self) -> NEProxyServer
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, HTTPServer)
    }
    unsafe fn setHTTPServer_(&self, HTTPServer: NEProxyServer)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setHTTPServer : HTTPServer)
    }
    unsafe fn HTTPSEnabled(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, HTTPSEnabled)
    }
    unsafe fn setHTTPSEnabled_(&self, HTTPSEnabled: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setHTTPSEnabled : HTTPSEnabled)
    }
    unsafe fn HTTPSServer(&self) -> NEProxyServer
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, HTTPSServer)
    }
    unsafe fn setHTTPSServer_(&self, HTTPSServer: NEProxyServer)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setHTTPSServer : HTTPSServer)
    }
    unsafe fn excludeSimpleHostnames(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, excludeSimpleHostnames)
    }
    unsafe fn setExcludeSimpleHostnames_(&self, excludeSimpleHostnames: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setExcludeSimpleHostnames : excludeSimpleHostnames)
    }
    unsafe fn exceptionList(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, exceptionList)
    }
    unsafe fn setExceptionList_(&self, exceptionList: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setExceptionList : exceptionList)
    }
    unsafe fn matchDomains(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, matchDomains)
    }
    unsafe fn setMatchDomains_(&self, matchDomains: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMatchDomains : matchDomains)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct NEVPNProtocol(pub id);
impl std::ops::Deref for NEVPNProtocol {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for NEVPNProtocol {}
impl NEVPNProtocol {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"NEVPNProtocol").unwrap(), alloc) })
    }
}
impl PNSCopying for NEVPNProtocol {}
impl PNSSecureCoding for NEVPNProtocol {}
impl INSObject for NEVPNProtocol {}
impl PNSObject for NEVPNProtocol {}
impl std::convert::TryFrom<NSObject> for NEVPNProtocol {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<NEVPNProtocol, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"NEVPNProtocol").unwrap()) };
        if is_kind_of {
            Ok(NEVPNProtocol(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to NEVPNProtocol")
        }
    }
}
impl INEVPNProtocol for NEVPNProtocol {}
pub trait INEVPNProtocol: Sized + std::ops::Deref {
    unsafe fn serverAddress(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, serverAddress)
    }
    unsafe fn setServerAddress_(&self, serverAddress: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setServerAddress : serverAddress)
    }
    unsafe fn username(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, username)
    }
    unsafe fn setUsername_(&self, username: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setUsername : username)
    }
    unsafe fn passwordReference(&self) -> NSData
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, passwordReference)
    }
    unsafe fn setPasswordReference_(&self, passwordReference: NSData)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPasswordReference : passwordReference)
    }
    unsafe fn identityReference(&self) -> NSData
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, identityReference)
    }
    unsafe fn setIdentityReference_(&self, identityReference: NSData)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setIdentityReference : identityReference)
    }
    unsafe fn identityData(&self) -> NSData
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, identityData)
    }
    unsafe fn setIdentityData_(&self, identityData: NSData)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setIdentityData : identityData)
    }
    unsafe fn identityDataPassword(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, identityDataPassword)
    }
    unsafe fn setIdentityDataPassword_(&self, identityDataPassword: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setIdentityDataPassword : identityDataPassword)
    }
    unsafe fn disconnectOnSleep(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, disconnectOnSleep)
    }
    unsafe fn setDisconnectOnSleep_(&self, disconnectOnSleep: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDisconnectOnSleep : disconnectOnSleep)
    }
    unsafe fn proxySettings(&self) -> NEProxySettings
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, proxySettings)
    }
    unsafe fn setProxySettings_(&self, proxySettings: NEProxySettings)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setProxySettings : proxySettings)
    }
    unsafe fn includeAllNetworks(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, includeAllNetworks)
    }
    unsafe fn setIncludeAllNetworks_(&self, includeAllNetworks: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setIncludeAllNetworks : includeAllNetworks)
    }
    unsafe fn excludeLocalNetworks(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, excludeLocalNetworks)
    }
    unsafe fn setExcludeLocalNetworks_(&self, excludeLocalNetworks: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setExcludeLocalNetworks : excludeLocalNetworks)
    }
    unsafe fn excludeCellularServices(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, excludeCellularServices)
    }
    unsafe fn setExcludeCellularServices_(&self, excludeCellularServices: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setExcludeCellularServices : excludeCellularServices)
    }
    unsafe fn excludeAPNs(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, excludeAPNs)
    }
    unsafe fn setExcludeAPNs_(&self, excludeAPNs: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setExcludeAPNs : excludeAPNs)
    }
    unsafe fn excludeDeviceCommunication(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, excludeDeviceCommunication)
    }
    unsafe fn setExcludeDeviceCommunication_(&self, excludeDeviceCommunication: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setExcludeDeviceCommunication : excludeDeviceCommunication)
    }
    unsafe fn enforceRoutes(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, enforceRoutes)
    }
    unsafe fn setEnforceRoutes_(&self, enforceRoutes: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setEnforceRoutes : enforceRoutes)
    }
    unsafe fn sliceUUID(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, sliceUUID)
    }
    unsafe fn setSliceUUID_(&self, sliceUUID: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSliceUUID : sliceUUID)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct NEDNSProxyProviderProtocol(pub id);
impl std::ops::Deref for NEDNSProxyProviderProtocol {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for NEDNSProxyProviderProtocol {}
impl NEDNSProxyProviderProtocol {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"NEDNSProxyProviderProtocol").unwrap(), alloc) })
    }
}
impl INEVPNProtocol for NEDNSProxyProviderProtocol {}
impl PNSCopying for NEDNSProxyProviderProtocol {}
impl PNSSecureCoding for NEDNSProxyProviderProtocol {}
impl From<NEDNSProxyProviderProtocol> for NEVPNProtocol {
    fn from(child: NEDNSProxyProviderProtocol) -> NEVPNProtocol {
        NEVPNProtocol(child.0)
    }
}
impl std::convert::TryFrom<NEVPNProtocol> for NEDNSProxyProviderProtocol {
    type Error = &'static str;
    fn try_from(parent: NEVPNProtocol) -> Result<NEDNSProxyProviderProtocol, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"NEDNSProxyProviderProtocol").unwrap()) };
        if is_kind_of {
            Ok(NEDNSProxyProviderProtocol(parent.0))
        } else {
            Err("This NEVPNProtocol cannot be downcasted to NEDNSProxyProviderProtocol")
        }
    }
}
impl INSObject for NEDNSProxyProviderProtocol {}
impl PNSObject for NEDNSProxyProviderProtocol {}
impl INEDNSProxyProviderProtocol for NEDNSProxyProviderProtocol {}
pub trait INEDNSProxyProviderProtocol: Sized + std::ops::Deref {
    unsafe fn providerConfiguration(&self) -> NSDictionary
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, providerConfiguration)
    }
    unsafe fn setProviderConfiguration_(&self, providerConfiguration: NSDictionary)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setProviderConfiguration : providerConfiguration)
    }
    unsafe fn providerBundleIdentifier(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, providerBundleIdentifier)
    }
    unsafe fn setProviderBundleIdentifier_(&self, providerBundleIdentifier: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setProviderBundleIdentifier : providerBundleIdentifier)
    }
}
pub type NEDNSProtocol = NSInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct NEDNSSettings(pub id);
impl std::ops::Deref for NEDNSSettings {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for NEDNSSettings {}
impl NEDNSSettings {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"NEDNSSettings").unwrap(), alloc) })
    }
}
impl PNSSecureCoding for NEDNSSettings {}
impl PNSCopying for NEDNSSettings {}
impl INSObject for NEDNSSettings {}
impl PNSObject for NEDNSSettings {}
impl std::convert::TryFrom<NSObject> for NEDNSSettings {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<NEDNSSettings, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"NEDNSSettings").unwrap()) };
        if is_kind_of {
            Ok(NEDNSSettings(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to NEDNSSettings")
        }
    }
}
impl INEDNSSettings for NEDNSSettings {}
pub trait INEDNSSettings: Sized + std::ops::Deref {
    unsafe fn initWithServers_(&self, servers: NSArray) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithServers : servers)
    }
    unsafe fn dnsProtocol(&self) -> NEDNSProtocol
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, dnsProtocol)
    }
    unsafe fn servers(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, servers)
    }
    unsafe fn searchDomains(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, searchDomains)
    }
    unsafe fn setSearchDomains_(&self, searchDomains: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSearchDomains : searchDomains)
    }
    unsafe fn domainName(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, domainName)
    }
    unsafe fn setDomainName_(&self, domainName: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDomainName : domainName)
    }
    unsafe fn matchDomains(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, matchDomains)
    }
    unsafe fn setMatchDomains_(&self, matchDomains: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMatchDomains : matchDomains)
    }
    unsafe fn matchDomainsNoSearch(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, matchDomainsNoSearch)
    }
    unsafe fn setMatchDomainsNoSearch_(&self, matchDomainsNoSearch: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMatchDomainsNoSearch : matchDomainsNoSearch)
    }
    unsafe fn allowFailover(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, allowFailover)
    }
    unsafe fn setAllowFailover_(&self, allowFailover: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAllowFailover : allowFailover)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct NEDNSOverTLSSettings(pub id);
impl std::ops::Deref for NEDNSOverTLSSettings {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for NEDNSOverTLSSettings {}
impl NEDNSOverTLSSettings {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"NEDNSOverTLSSettings").unwrap(), alloc) })
    }
}
impl INEDNSSettings for NEDNSOverTLSSettings {}
impl PNSSecureCoding for NEDNSOverTLSSettings {}
impl PNSCopying for NEDNSOverTLSSettings {}
impl From<NEDNSOverTLSSettings> for NEDNSSettings {
    fn from(child: NEDNSOverTLSSettings) -> NEDNSSettings {
        NEDNSSettings(child.0)
    }
}
impl std::convert::TryFrom<NEDNSSettings> for NEDNSOverTLSSettings {
    type Error = &'static str;
    fn try_from(parent: NEDNSSettings) -> Result<NEDNSOverTLSSettings, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"NEDNSOverTLSSettings").unwrap()) };
        if is_kind_of {
            Ok(NEDNSOverTLSSettings(parent.0))
        } else {
            Err("This NEDNSSettings cannot be downcasted to NEDNSOverTLSSettings")
        }
    }
}
impl INSObject for NEDNSOverTLSSettings {}
impl PNSObject for NEDNSOverTLSSettings {}
impl INEDNSOverTLSSettings for NEDNSOverTLSSettings {}
pub trait INEDNSOverTLSSettings: Sized + std::ops::Deref {
    unsafe fn serverName(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, serverName)
    }
    unsafe fn setServerName_(&self, serverName: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setServerName : serverName)
    }
    unsafe fn identityReference(&self) -> NSData
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, identityReference)
    }
    unsafe fn setIdentityReference_(&self, identityReference: NSData)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setIdentityReference : identityReference)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct NEDNSOverHTTPSSettings(pub id);
impl std::ops::Deref for NEDNSOverHTTPSSettings {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for NEDNSOverHTTPSSettings {}
impl NEDNSOverHTTPSSettings {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"NEDNSOverHTTPSSettings").unwrap(), alloc) })
    }
}
impl INEDNSSettings for NEDNSOverHTTPSSettings {}
impl PNSSecureCoding for NEDNSOverHTTPSSettings {}
impl PNSCopying for NEDNSOverHTTPSSettings {}
impl std::convert::TryFrom<NEDNSSettings> for NEDNSOverHTTPSSettings {
    type Error = &'static str;
    fn try_from(parent: NEDNSSettings) -> Result<NEDNSOverHTTPSSettings, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"NEDNSOverHTTPSSettings").unwrap()) };
        if is_kind_of {
            Ok(NEDNSOverHTTPSSettings(parent.0))
        } else {
            Err("This NEDNSSettings cannot be downcasted to NEDNSOverHTTPSSettings")
        }
    }
}
impl INSObject for NEDNSOverHTTPSSettings {}
impl PNSObject for NEDNSOverHTTPSSettings {}
impl INEDNSOverHTTPSSettings for NEDNSOverHTTPSSettings {}
pub trait INEDNSOverHTTPSSettings: Sized + std::ops::Deref {
    unsafe fn serverURL(&self) -> NSURL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, serverURL)
    }
    unsafe fn setServerURL_(&self, serverURL: NSURL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setServerURL : serverURL)
    }
    unsafe fn identityReference(&self) -> NSData
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, identityReference)
    }
    unsafe fn setIdentityReference_(&self, identityReference: NSData)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setIdentityReference : identityReference)
    }
}
pub type NEDNSSettingsManagerError = NSInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct NEDNSSettingsManager(pub id);
impl std::ops::Deref for NEDNSSettingsManager {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for NEDNSSettingsManager {}
impl NEDNSSettingsManager {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"NEDNSSettingsManager").unwrap(), alloc) })
    }
}
impl INSObject for NEDNSSettingsManager {}
impl PNSObject for NEDNSSettingsManager {}
impl std::convert::TryFrom<NSObject> for NEDNSSettingsManager {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<NEDNSSettingsManager, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"NEDNSSettingsManager").unwrap()) };
        if is_kind_of {
            Ok(NEDNSSettingsManager(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to NEDNSSettingsManager")
        }
    }
}
impl INEDNSSettingsManager for NEDNSSettingsManager {}
pub trait INEDNSSettingsManager: Sized + std::ops::Deref {
    unsafe fn loadFromPreferencesWithCompletionHandler_(
        &self,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, loadFromPreferencesWithCompletionHandler : completionHandler)
    }
    unsafe fn removeFromPreferencesWithCompletionHandler_(
        &self,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, removeFromPreferencesWithCompletionHandler : completionHandler)
    }
    unsafe fn saveToPreferencesWithCompletionHandler_(
        &self,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, saveToPreferencesWithCompletionHandler : completionHandler)
    }
    unsafe fn localizedDescription(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, localizedDescription)
    }
    unsafe fn setLocalizedDescription_(&self, localizedDescription: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setLocalizedDescription : localizedDescription)
    }
    unsafe fn dnsSettings(&self) -> NEDNSSettings
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, dnsSettings)
    }
    unsafe fn setDnsSettings_(&self, dnsSettings: NEDNSSettings)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDnsSettings : dnsSettings)
    }
    unsafe fn onDemandRules(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, onDemandRules)
    }
    unsafe fn setOnDemandRules_(&self, onDemandRules: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setOnDemandRules : onDemandRules)
    }
    unsafe fn isEnabled(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isEnabled)
    }
    unsafe fn sharedManager() -> NEDNSSettingsManager
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NEDNSSettingsManager").unwrap(), sharedManager)
    }
}
pub type NENetworkRuleProtocol = NSInteger;
pub type NETrafficDirection = NSInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct NENetworkRule(pub id);
impl std::ops::Deref for NENetworkRule {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for NENetworkRule {}
impl NENetworkRule {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"NENetworkRule").unwrap(), alloc) })
    }
}
impl PNSSecureCoding for NENetworkRule {}
impl PNSCopying for NENetworkRule {}
impl INSObject for NENetworkRule {}
impl PNSObject for NENetworkRule {}
impl std::convert::TryFrom<NSObject> for NENetworkRule {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<NENetworkRule, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"NENetworkRule").unwrap()) };
        if is_kind_of {
            Ok(NENetworkRule(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to NENetworkRule")
        }
    }
}
impl INENetworkRule for NENetworkRule {}
pub trait INENetworkRule: Sized + std::ops::Deref {
    unsafe fn initWithDestinationNetworkEndpoint_prefix_protocol_(
        &self,
        networkEndpoint: NSObject,
        destinationPrefix: NSUInteger,
        protocol: NENetworkRuleProtocol,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithDestinationNetworkEndpoint : networkEndpoint, prefix : destinationPrefix, protocol : protocol)
    }
    unsafe fn initWithDestinationNetwork_prefix_protocol_(
        &self,
        networkEndpoint: NWHostEndpoint,
        destinationPrefix: NSUInteger,
        protocol: NENetworkRuleProtocol,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithDestinationNetwork : networkEndpoint, prefix : destinationPrefix, protocol : protocol)
    }
    unsafe fn initWithDestinationHostEndpoint_protocol_(
        &self,
        hostEndpoint: NSObject,
        protocol: NENetworkRuleProtocol,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithDestinationHostEndpoint : hostEndpoint, protocol : protocol)
    }
    unsafe fn initWithDestinationHost_protocol_(
        &self,
        hostEndpoint: NWHostEndpoint,
        protocol: NENetworkRuleProtocol,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithDestinationHost : hostEndpoint, protocol : protocol)
    }
    unsafe fn initWithRemoteNetworkEndpoint_remotePrefix_localNetworkEndpoint_localPrefix_protocol_direction_(
        &self,
        remoteNetwork: NSObject,
        remotePrefix: NSUInteger,
        localNetwork: NSObject,
        localPrefix: NSUInteger,
        protocol: NENetworkRuleProtocol,
        direction: NETrafficDirection,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithRemoteNetworkEndpoint : remoteNetwork, remotePrefix : remotePrefix, localNetworkEndpoint : localNetwork, localPrefix : localPrefix, protocol : protocol, direction : direction)
    }
    unsafe fn initWithRemoteNetwork_remotePrefix_localNetwork_localPrefix_protocol_direction_(
        &self,
        remoteNetwork: NWHostEndpoint,
        remotePrefix: NSUInteger,
        localNetwork: NWHostEndpoint,
        localPrefix: NSUInteger,
        protocol: NENetworkRuleProtocol,
        direction: NETrafficDirection,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithRemoteNetwork : remoteNetwork, remotePrefix : remotePrefix, localNetwork : localNetwork, localPrefix : localPrefix, protocol : protocol, direction : direction)
    }
    unsafe fn matchRemoteHostOrNetworkEndpoint(&self) -> nw_endpoint_t
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, matchRemoteHostOrNetworkEndpoint)
    }
    unsafe fn matchRemoteEndpoint(&self) -> NWHostEndpoint
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, matchRemoteEndpoint)
    }
    unsafe fn matchRemotePrefix(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, matchRemotePrefix)
    }
    unsafe fn matchLocalNetworkEndpoint(&self) -> nw_endpoint_t
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, matchLocalNetworkEndpoint)
    }
    unsafe fn matchLocalNetwork(&self) -> NWHostEndpoint
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, matchLocalNetwork)
    }
    unsafe fn matchLocalPrefix(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, matchLocalPrefix)
    }
    unsafe fn matchProtocol(&self) -> NENetworkRuleProtocol
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, matchProtocol)
    }
    unsafe fn matchDirection(&self) -> NETrafficDirection
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, matchDirection)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct NEFilterFlow(pub id);
impl std::ops::Deref for NEFilterFlow {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for NEFilterFlow {}
impl NEFilterFlow {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"NEFilterFlow").unwrap(), alloc) })
    }
}
impl PNSSecureCoding for NEFilterFlow {}
impl PNSCopying for NEFilterFlow {}
impl INSObject for NEFilterFlow {}
impl PNSObject for NEFilterFlow {}
impl std::convert::TryFrom<NSObject> for NEFilterFlow {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<NEFilterFlow, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"NEFilterFlow").unwrap()) };
        if is_kind_of {
            Ok(NEFilterFlow(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to NEFilterFlow")
        }
    }
}
impl INEFilterFlow for NEFilterFlow {}
pub trait INEFilterFlow: Sized + std::ops::Deref {
    unsafe fn URL(&self) -> NSURL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, URL)
    }
    unsafe fn sourceAppUniqueIdentifier(&self) -> NSData
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, sourceAppUniqueIdentifier)
    }
    unsafe fn sourceAppIdentifier(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, sourceAppIdentifier)
    }
    unsafe fn sourceAppVersion(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, sourceAppVersion)
    }
    unsafe fn direction(&self) -> NETrafficDirection
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, direction)
    }
    unsafe fn sourceAppAuditToken(&self) -> NSData
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, sourceAppAuditToken)
    }
    unsafe fn sourceProcessAuditToken(&self) -> NSData
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, sourceProcessAuditToken)
    }
    unsafe fn identifier(&self) -> NSUUID
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, identifier)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct NEFilterBrowserFlow(pub id);
impl std::ops::Deref for NEFilterBrowserFlow {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for NEFilterBrowserFlow {}
impl NEFilterBrowserFlow {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"NEFilterBrowserFlow").unwrap(), alloc) })
    }
}
impl PNSSecureCoding for NEFilterBrowserFlow {}
impl PNSCopying for NEFilterBrowserFlow {}
impl INEFilterFlow for NEFilterBrowserFlow {}
impl From<NEFilterBrowserFlow> for NEFilterFlow {
    fn from(child: NEFilterBrowserFlow) -> NEFilterFlow {
        NEFilterFlow(child.0)
    }
}
impl std::convert::TryFrom<NEFilterFlow> for NEFilterBrowserFlow {
    type Error = &'static str;
    fn try_from(parent: NEFilterFlow) -> Result<NEFilterBrowserFlow, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"NEFilterBrowserFlow").unwrap()) };
        if is_kind_of {
            Ok(NEFilterBrowserFlow(parent.0))
        } else {
            Err("This NEFilterFlow cannot be downcasted to NEFilterBrowserFlow")
        }
    }
}
impl INSObject for NEFilterBrowserFlow {}
impl PNSObject for NEFilterBrowserFlow {}
impl INEFilterBrowserFlow for NEFilterBrowserFlow {}
pub trait INEFilterBrowserFlow: Sized + std::ops::Deref {
    unsafe fn request(&self) -> NSURLRequest
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, request)
    }
    unsafe fn response(&self) -> NSURLResponse
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, response)
    }
    unsafe fn parentURL(&self) -> NSURL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, parentURL)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct NEFilterSocketFlow(pub id);
impl std::ops::Deref for NEFilterSocketFlow {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for NEFilterSocketFlow {}
impl NEFilterSocketFlow {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"NEFilterSocketFlow").unwrap(), alloc) })
    }
}
impl PNSSecureCoding for NEFilterSocketFlow {}
impl PNSCopying for NEFilterSocketFlow {}
impl INEFilterFlow for NEFilterSocketFlow {}
impl std::convert::TryFrom<NEFilterFlow> for NEFilterSocketFlow {
    type Error = &'static str;
    fn try_from(parent: NEFilterFlow) -> Result<NEFilterSocketFlow, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"NEFilterSocketFlow").unwrap()) };
        if is_kind_of {
            Ok(NEFilterSocketFlow(parent.0))
        } else {
            Err("This NEFilterFlow cannot be downcasted to NEFilterSocketFlow")
        }
    }
}
impl INSObject for NEFilterSocketFlow {}
impl PNSObject for NEFilterSocketFlow {}
impl INEFilterSocketFlow for NEFilterSocketFlow {}
pub trait INEFilterSocketFlow: Sized + std::ops::Deref {
    unsafe fn remoteFlowEndpoint(&self) -> nw_endpoint_t
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, remoteFlowEndpoint)
    }
    unsafe fn remoteEndpoint(&self) -> NWEndpoint
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, remoteEndpoint)
    }
    unsafe fn remoteHostname(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, remoteHostname)
    }
    unsafe fn localFlowEndpoint(&self) -> nw_endpoint_t
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, localFlowEndpoint)
    }
    unsafe fn localEndpoint(&self) -> NWEndpoint
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, localEndpoint)
    }
    unsafe fn socketFamily(&self) -> ::std::os::raw::c_int
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, socketFamily)
    }
    unsafe fn socketType(&self) -> ::std::os::raw::c_int
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, socketType)
    }
    unsafe fn socketProtocol(&self) -> ::std::os::raw::c_int
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, socketProtocol)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct NEFilterProvider(pub id);
impl std::ops::Deref for NEFilterProvider {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for NEFilterProvider {}
impl NEFilterProvider {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"NEFilterProvider").unwrap(), alloc) })
    }
}
impl INEProvider for NEFilterProvider {}
impl std::convert::TryFrom<NEProvider> for NEFilterProvider {
    type Error = &'static str;
    fn try_from(parent: NEProvider) -> Result<NEFilterProvider, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"NEFilterProvider").unwrap()) };
        if is_kind_of {
            Ok(NEFilterProvider(parent.0))
        } else {
            Err("This NEProvider cannot be downcasted to NEFilterProvider")
        }
    }
}
impl INSObject for NEFilterProvider {}
impl PNSObject for NEFilterProvider {}
impl INEFilterProvider for NEFilterProvider {}
pub trait INEFilterProvider: Sized + std::ops::Deref {
    unsafe fn startFilterWithCompletionHandler_(
        &self,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, startFilterWithCompletionHandler : completionHandler)
    }
    unsafe fn stopFilterWithReason_completionHandler_(
        &self,
        reason: NEProviderStopReason,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, stopFilterWithReason : reason, completionHandler : completionHandler)
    }
    unsafe fn handleReport_(&self, report: NEFilterReport)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, handleReport : report)
    }
    unsafe fn filterConfiguration(&self) -> NEFilterProviderConfiguration
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, filterConfiguration)
    }
}
pub type NEFilterReportFrequency = NSInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct NEFilterVerdict(pub id);
impl std::ops::Deref for NEFilterVerdict {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for NEFilterVerdict {}
impl NEFilterVerdict {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"NEFilterVerdict").unwrap(), alloc) })
    }
}
impl PNSSecureCoding for NEFilterVerdict {}
impl PNSCopying for NEFilterVerdict {}
impl INSObject for NEFilterVerdict {}
impl PNSObject for NEFilterVerdict {}
impl std::convert::TryFrom<NSObject> for NEFilterVerdict {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<NEFilterVerdict, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"NEFilterVerdict").unwrap()) };
        if is_kind_of {
            Ok(NEFilterVerdict(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to NEFilterVerdict")
        }
    }
}
impl INEFilterVerdict for NEFilterVerdict {}
pub trait INEFilterVerdict: Sized + std::ops::Deref {
    unsafe fn shouldReport(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, shouldReport)
    }
    unsafe fn setShouldReport_(&self, shouldReport: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setShouldReport : shouldReport)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct NEFilterNewFlowVerdict(pub id);
impl std::ops::Deref for NEFilterNewFlowVerdict {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for NEFilterNewFlowVerdict {}
impl NEFilterNewFlowVerdict {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"NEFilterNewFlowVerdict").unwrap(), alloc) })
    }
}
impl PNSSecureCoding for NEFilterNewFlowVerdict {}
impl PNSCopying for NEFilterNewFlowVerdict {}
impl INEFilterVerdict for NEFilterNewFlowVerdict {}
impl From<NEFilterNewFlowVerdict> for NEFilterVerdict {
    fn from(child: NEFilterNewFlowVerdict) -> NEFilterVerdict {
        NEFilterVerdict(child.0)
    }
}
impl std::convert::TryFrom<NEFilterVerdict> for NEFilterNewFlowVerdict {
    type Error = &'static str;
    fn try_from(parent: NEFilterVerdict) -> Result<NEFilterNewFlowVerdict, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"NEFilterNewFlowVerdict").unwrap()) };
        if is_kind_of {
            Ok(NEFilterNewFlowVerdict(parent.0))
        } else {
            Err("This NEFilterVerdict cannot be downcasted to NEFilterNewFlowVerdict")
        }
    }
}
impl INSObject for NEFilterNewFlowVerdict {}
impl PNSObject for NEFilterNewFlowVerdict {}
impl INEFilterNewFlowVerdict for NEFilterNewFlowVerdict {}
pub trait INEFilterNewFlowVerdict: Sized + std::ops::Deref {
    unsafe fn statisticsReportFrequency(&self) -> NEFilterReportFrequency
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, statisticsReportFrequency)
    }
    unsafe fn setStatisticsReportFrequency_(
        &self,
        statisticsReportFrequency: NEFilterReportFrequency,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setStatisticsReportFrequency : statisticsReportFrequency)
    }
    unsafe fn needRulesVerdict() -> NEFilterNewFlowVerdict
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NEFilterNewFlowVerdict").unwrap(), needRulesVerdict)
    }
    unsafe fn allowVerdict() -> NEFilterNewFlowVerdict
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NEFilterNewFlowVerdict").unwrap(), allowVerdict)
    }
    unsafe fn dropVerdict() -> NEFilterNewFlowVerdict
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NEFilterNewFlowVerdict").unwrap(), dropVerdict)
    }
    unsafe fn remediateVerdictWithRemediationURLMapKey_remediationButtonTextMapKey_(
        remediationURLMapKey: NSString,
        remediationButtonTextMapKey: NSString,
    ) -> NEFilterNewFlowVerdict
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NEFilterNewFlowVerdict").unwrap(), remediateVerdictWithRemediationURLMapKey : remediationURLMapKey, remediationButtonTextMapKey : remediationButtonTextMapKey)
    }
    unsafe fn URLAppendStringVerdictWithMapKey_(urlAppendMapKey: NSString) -> NEFilterNewFlowVerdict
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NEFilterNewFlowVerdict").unwrap(), URLAppendStringVerdictWithMapKey : urlAppendMapKey)
    }
    unsafe fn filterDataVerdictWithFilterInbound_peekInboundBytes_filterOutbound_peekOutboundBytes_(
        filterInbound: BOOL,
        peekInboundBytes: NSUInteger,
        filterOutbound: BOOL,
        peekOutboundBytes: NSUInteger,
    ) -> NEFilterNewFlowVerdict
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NEFilterNewFlowVerdict").unwrap(), filterDataVerdictWithFilterInbound : filterInbound, peekInboundBytes : peekInboundBytes, filterOutbound : filterOutbound, peekOutboundBytes : peekOutboundBytes)
    }
    unsafe fn pauseVerdict() -> NEFilterNewFlowVerdict
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NEFilterNewFlowVerdict").unwrap(), pauseVerdict)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct NEFilterControlVerdict(pub id);
impl std::ops::Deref for NEFilterControlVerdict {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for NEFilterControlVerdict {}
impl NEFilterControlVerdict {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"NEFilterControlVerdict").unwrap(), alloc) })
    }
}
impl PNSSecureCoding for NEFilterControlVerdict {}
impl PNSCopying for NEFilterControlVerdict {}
impl INEFilterNewFlowVerdict for NEFilterControlVerdict {}
impl From<NEFilterControlVerdict> for NEFilterNewFlowVerdict {
    fn from(child: NEFilterControlVerdict) -> NEFilterNewFlowVerdict {
        NEFilterNewFlowVerdict(child.0)
    }
}
impl std::convert::TryFrom<NEFilterNewFlowVerdict> for NEFilterControlVerdict {
    type Error = &'static str;
    fn try_from(parent: NEFilterNewFlowVerdict) -> Result<NEFilterControlVerdict, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"NEFilterControlVerdict").unwrap()) };
        if is_kind_of {
            Ok(NEFilterControlVerdict(parent.0))
        } else {
            Err("This NEFilterNewFlowVerdict cannot be downcasted to NEFilterControlVerdict")
        }
    }
}
impl INEFilterVerdict for NEFilterControlVerdict {}
impl INSObject for NEFilterControlVerdict {}
impl PNSObject for NEFilterControlVerdict {}
impl INEFilterControlVerdict for NEFilterControlVerdict {}
pub trait INEFilterControlVerdict: Sized + std::ops::Deref {
    unsafe fn allowVerdictWithUpdateRules_(updateRules: BOOL) -> NEFilterControlVerdict
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NEFilterControlVerdict").unwrap(), allowVerdictWithUpdateRules : updateRules)
    }
    unsafe fn dropVerdictWithUpdateRules_(updateRules: BOOL) -> NEFilterControlVerdict
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NEFilterControlVerdict").unwrap(), dropVerdictWithUpdateRules : updateRules)
    }
    unsafe fn updateRules() -> NEFilterControlVerdict
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NEFilterControlVerdict").unwrap(), updateRules)
    }
}
pub type NEFilterAction = NSInteger;
pub type NEFilterReportEvent = NSInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct NEFilterReport(pub id);
impl std::ops::Deref for NEFilterReport {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for NEFilterReport {}
impl NEFilterReport {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"NEFilterReport").unwrap(), alloc) })
    }
}
impl PNSSecureCoding for NEFilterReport {}
impl PNSCopying for NEFilterReport {}
impl INSObject for NEFilterReport {}
impl PNSObject for NEFilterReport {}
impl std::convert::TryFrom<NSObject> for NEFilterReport {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<NEFilterReport, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"NEFilterReport").unwrap()) };
        if is_kind_of {
            Ok(NEFilterReport(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to NEFilterReport")
        }
    }
}
impl INEFilterReport for NEFilterReport {}
pub trait INEFilterReport: Sized + std::ops::Deref {
    unsafe fn flow(&self) -> NEFilterFlow
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, flow)
    }
    unsafe fn action(&self) -> NEFilterAction
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, action)
    }
    unsafe fn event(&self) -> NEFilterReportEvent
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, event)
    }
    unsafe fn bytesInboundCount(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, bytesInboundCount)
    }
    unsafe fn bytesOutboundCount(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, bytesOutboundCount)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct NEFilterControlProvider(pub id);
impl std::ops::Deref for NEFilterControlProvider {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for NEFilterControlProvider {}
impl NEFilterControlProvider {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"NEFilterControlProvider").unwrap(), alloc) })
    }
}
impl INEFilterProvider for NEFilterControlProvider {}
impl From<NEFilterControlProvider> for NEFilterProvider {
    fn from(child: NEFilterControlProvider) -> NEFilterProvider {
        NEFilterProvider(child.0)
    }
}
impl std::convert::TryFrom<NEFilterProvider> for NEFilterControlProvider {
    type Error = &'static str;
    fn try_from(parent: NEFilterProvider) -> Result<NEFilterControlProvider, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"NEFilterControlProvider").unwrap()) };
        if is_kind_of {
            Ok(NEFilterControlProvider(parent.0))
        } else {
            Err("This NEFilterProvider cannot be downcasted to NEFilterControlProvider")
        }
    }
}
impl INEProvider for NEFilterControlProvider {}
impl INSObject for NEFilterControlProvider {}
impl PNSObject for NEFilterControlProvider {}
impl INEFilterControlProvider for NEFilterControlProvider {}
pub trait INEFilterControlProvider: Sized + std::ops::Deref {
    unsafe fn handleRemediationForFlow_completionHandler_(
        &self,
        flow: NEFilterFlow,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, handleRemediationForFlow : flow, completionHandler : completionHandler)
    }
    unsafe fn handleNewFlow_completionHandler_(
        &self,
        flow: NEFilterFlow,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, handleNewFlow : flow, completionHandler : completionHandler)
    }
    unsafe fn notifyRulesChanged(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, notifyRulesChanged)
    }
    unsafe fn remediationMap(&self) -> NSDictionary
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, remediationMap)
    }
    unsafe fn setRemediationMap_(&self, remediationMap: NSDictionary)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRemediationMap : remediationMap)
    }
    unsafe fn URLAppendStringMap(&self) -> NSDictionary
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, URLAppendStringMap)
    }
    unsafe fn setURLAppendStringMap_(&self, URLAppendStringMap: NSDictionary)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setURLAppendStringMap : URLAppendStringMap)
    }
}
pub type NEFilterDataAttribute = NSInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct NEFilterDataProvider(pub id);
impl std::ops::Deref for NEFilterDataProvider {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for NEFilterDataProvider {}
impl NEFilterDataProvider {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"NEFilterDataProvider").unwrap(), alloc) })
    }
}
impl INEFilterProvider for NEFilterDataProvider {}
impl std::convert::TryFrom<NEFilterProvider> for NEFilterDataProvider {
    type Error = &'static str;
    fn try_from(parent: NEFilterProvider) -> Result<NEFilterDataProvider, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"NEFilterDataProvider").unwrap()) };
        if is_kind_of {
            Ok(NEFilterDataProvider(parent.0))
        } else {
            Err("This NEFilterProvider cannot be downcasted to NEFilterDataProvider")
        }
    }
}
impl INEProvider for NEFilterDataProvider {}
impl INSObject for NEFilterDataProvider {}
impl PNSObject for NEFilterDataProvider {}
impl INEFilterDataProvider for NEFilterDataProvider {}
pub trait INEFilterDataProvider: Sized + std::ops::Deref {
    unsafe fn handleNewFlow_(&self, flow: NEFilterFlow) -> NEFilterNewFlowVerdict
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, handleNewFlow : flow)
    }
    unsafe fn handleInboundDataFromFlow_readBytesStartOffset_readBytes_(
        &self,
        flow: NEFilterFlow,
        offset: NSUInteger,
        readBytes: NSData,
    ) -> NEFilterDataVerdict
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, handleInboundDataFromFlow : flow, readBytesStartOffset : offset, readBytes : readBytes)
    }
    unsafe fn handleOutboundDataFromFlow_readBytesStartOffset_readBytes_(
        &self,
        flow: NEFilterFlow,
        offset: NSUInteger,
        readBytes: NSData,
    ) -> NEFilterDataVerdict
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, handleOutboundDataFromFlow : flow, readBytesStartOffset : offset, readBytes : readBytes)
    }
    unsafe fn handleInboundDataCompleteForFlow_(&self, flow: NEFilterFlow) -> NEFilterDataVerdict
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, handleInboundDataCompleteForFlow : flow)
    }
    unsafe fn handleOutboundDataCompleteForFlow_(&self, flow: NEFilterFlow) -> NEFilterDataVerdict
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, handleOutboundDataCompleteForFlow : flow)
    }
    unsafe fn handleRemediationForFlow_(&self, flow: NEFilterFlow) -> NEFilterRemediationVerdict
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, handleRemediationForFlow : flow)
    }
    unsafe fn handleRulesChanged(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, handleRulesChanged)
    }
    unsafe fn applySettings_completionHandler_(
        &self,
        settings: NEFilterSettings,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, applySettings : settings, completionHandler : completionHandler)
    }
    unsafe fn resumeFlow_withVerdict_(&self, flow: NEFilterFlow, verdict: NEFilterVerdict)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, resumeFlow : flow, withVerdict : verdict)
    }
    unsafe fn updateFlow_usingVerdict_forDirection_(
        &self,
        flow: NEFilterSocketFlow,
        verdict: NEFilterDataVerdict,
        direction: NETrafficDirection,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, updateFlow : flow, usingVerdict : verdict, forDirection : direction)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct NEFilterDataVerdict(pub id);
impl std::ops::Deref for NEFilterDataVerdict {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for NEFilterDataVerdict {}
impl NEFilterDataVerdict {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"NEFilterDataVerdict").unwrap(), alloc) })
    }
}
impl PNSSecureCoding for NEFilterDataVerdict {}
impl PNSCopying for NEFilterDataVerdict {}
impl INEFilterVerdict for NEFilterDataVerdict {}
impl std::convert::TryFrom<NEFilterVerdict> for NEFilterDataVerdict {
    type Error = &'static str;
    fn try_from(parent: NEFilterVerdict) -> Result<NEFilterDataVerdict, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"NEFilterDataVerdict").unwrap()) };
        if is_kind_of {
            Ok(NEFilterDataVerdict(parent.0))
        } else {
            Err("This NEFilterVerdict cannot be downcasted to NEFilterDataVerdict")
        }
    }
}
impl INSObject for NEFilterDataVerdict {}
impl PNSObject for NEFilterDataVerdict {}
impl INEFilterDataVerdict for NEFilterDataVerdict {}
pub trait INEFilterDataVerdict: Sized + std::ops::Deref {
    unsafe fn statisticsReportFrequency(&self) -> NEFilterReportFrequency
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, statisticsReportFrequency)
    }
    unsafe fn setStatisticsReportFrequency_(
        &self,
        statisticsReportFrequency: NEFilterReportFrequency,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setStatisticsReportFrequency : statisticsReportFrequency)
    }
    unsafe fn allowVerdict() -> NEFilterDataVerdict
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NEFilterDataVerdict").unwrap(), allowVerdict)
    }
    unsafe fn dropVerdict() -> NEFilterDataVerdict
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NEFilterDataVerdict").unwrap(), dropVerdict)
    }
    unsafe fn remediateVerdictWithRemediationURLMapKey_remediationButtonTextMapKey_(
        remediationURLMapKey: NSString,
        remediationButtonTextMapKey: NSString,
    ) -> NEFilterDataVerdict
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NEFilterDataVerdict").unwrap(), remediateVerdictWithRemediationURLMapKey : remediationURLMapKey, remediationButtonTextMapKey : remediationButtonTextMapKey)
    }
    unsafe fn dataVerdictWithPassBytes_peekBytes_(
        passBytes: NSUInteger,
        peekBytes: NSUInteger,
    ) -> NEFilterDataVerdict
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NEFilterDataVerdict").unwrap(), dataVerdictWithPassBytes : passBytes, peekBytes : peekBytes)
    }
    unsafe fn needRulesVerdict() -> NEFilterDataVerdict
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NEFilterDataVerdict").unwrap(), needRulesVerdict)
    }
    unsafe fn pauseVerdict() -> NEFilterDataVerdict
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NEFilterDataVerdict").unwrap(), pauseVerdict)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct NEFilterRemediationVerdict(pub id);
impl std::ops::Deref for NEFilterRemediationVerdict {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for NEFilterRemediationVerdict {}
impl NEFilterRemediationVerdict {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"NEFilterRemediationVerdict").unwrap(), alloc) })
    }
}
impl PNSSecureCoding for NEFilterRemediationVerdict {}
impl PNSCopying for NEFilterRemediationVerdict {}
impl INEFilterVerdict for NEFilterRemediationVerdict {}
impl std::convert::TryFrom<NEFilterVerdict> for NEFilterRemediationVerdict {
    type Error = &'static str;
    fn try_from(parent: NEFilterVerdict) -> Result<NEFilterRemediationVerdict, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"NEFilterRemediationVerdict").unwrap()) };
        if is_kind_of {
            Ok(NEFilterRemediationVerdict(parent.0))
        } else {
            Err("This NEFilterVerdict cannot be downcasted to NEFilterRemediationVerdict")
        }
    }
}
impl INSObject for NEFilterRemediationVerdict {}
impl PNSObject for NEFilterRemediationVerdict {}
impl INEFilterRemediationVerdict for NEFilterRemediationVerdict {}
pub trait INEFilterRemediationVerdict: Sized + std::ops::Deref {
    unsafe fn allowVerdict() -> NEFilterRemediationVerdict
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NEFilterRemediationVerdict").unwrap(), allowVerdict)
    }
    unsafe fn dropVerdict() -> NEFilterRemediationVerdict
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NEFilterRemediationVerdict").unwrap(), dropVerdict)
    }
    unsafe fn needRulesVerdict() -> NEFilterRemediationVerdict
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NEFilterRemediationVerdict").unwrap(), needRulesVerdict)
    }
}
pub type NEFilterManagerError = NSInteger;
pub type NEFilterManagerGrade = NSInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct NEFilterManager(pub id);
impl std::ops::Deref for NEFilterManager {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for NEFilterManager {}
impl NEFilterManager {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"NEFilterManager").unwrap(), alloc) })
    }
}
impl INSObject for NEFilterManager {}
impl PNSObject for NEFilterManager {}
impl std::convert::TryFrom<NSObject> for NEFilterManager {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<NEFilterManager, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"NEFilterManager").unwrap()) };
        if is_kind_of {
            Ok(NEFilterManager(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to NEFilterManager")
        }
    }
}
impl INEFilterManager for NEFilterManager {}
pub trait INEFilterManager: Sized + std::ops::Deref {
    unsafe fn loadFromPreferencesWithCompletionHandler_(
        &self,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, loadFromPreferencesWithCompletionHandler : completionHandler)
    }
    unsafe fn removeFromPreferencesWithCompletionHandler_(
        &self,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, removeFromPreferencesWithCompletionHandler : completionHandler)
    }
    unsafe fn saveToPreferencesWithCompletionHandler_(
        &self,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, saveToPreferencesWithCompletionHandler : completionHandler)
    }
    unsafe fn localizedDescription(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, localizedDescription)
    }
    unsafe fn setLocalizedDescription_(&self, localizedDescription: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setLocalizedDescription : localizedDescription)
    }
    unsafe fn providerConfiguration(&self) -> NEFilterProviderConfiguration
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, providerConfiguration)
    }
    unsafe fn setProviderConfiguration_(&self, providerConfiguration: NEFilterProviderConfiguration)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setProviderConfiguration : providerConfiguration)
    }
    unsafe fn isEnabled(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isEnabled)
    }
    unsafe fn setEnabled_(&self, enabled: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setEnabled : enabled)
    }
    unsafe fn grade(&self) -> NEFilterManagerGrade
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, grade)
    }
    unsafe fn setGrade_(&self, grade: NEFilterManagerGrade)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setGrade : grade)
    }
    unsafe fn disableEncryptedDNSSettings(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, disableEncryptedDNSSettings)
    }
    unsafe fn setDisableEncryptedDNSSettings_(&self, disableEncryptedDNSSettings: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDisableEncryptedDNSSettings : disableEncryptedDNSSettings)
    }
    unsafe fn sharedManager() -> NEFilterManager
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NEFilterManager").unwrap(), sharedManager)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct NEFilterPacketContext(pub id);
impl std::ops::Deref for NEFilterPacketContext {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for NEFilterPacketContext {}
impl NEFilterPacketContext {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"NEFilterPacketContext").unwrap(), alloc) })
    }
}
impl INSObject for NEFilterPacketContext {}
impl PNSObject for NEFilterPacketContext {}
impl std::convert::TryFrom<NSObject> for NEFilterPacketContext {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<NEFilterPacketContext, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"NEFilterPacketContext").unwrap()) };
        if is_kind_of {
            Ok(NEFilterPacketContext(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to NEFilterPacketContext")
        }
    }
}
impl INEFilterPacketContext for NEFilterPacketContext {}
pub trait INEFilterPacketContext: Sized + std::ops::Deref {}
pub type NEFilterPacketProviderVerdict = NSInteger;
pub type NEFilterPacketHandler = *mut ::std::os::raw::c_void;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct NEFilterPacketProvider(pub id);
impl std::ops::Deref for NEFilterPacketProvider {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for NEFilterPacketProvider {}
impl NEFilterPacketProvider {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"NEFilterPacketProvider").unwrap(), alloc) })
    }
}
impl INEFilterProvider for NEFilterPacketProvider {}
impl std::convert::TryFrom<NEFilterProvider> for NEFilterPacketProvider {
    type Error = &'static str;
    fn try_from(parent: NEFilterProvider) -> Result<NEFilterPacketProvider, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"NEFilterPacketProvider").unwrap()) };
        if is_kind_of {
            Ok(NEFilterPacketProvider(parent.0))
        } else {
            Err("This NEFilterProvider cannot be downcasted to NEFilterPacketProvider")
        }
    }
}
impl INEProvider for NEFilterPacketProvider {}
impl INSObject for NEFilterPacketProvider {}
impl PNSObject for NEFilterPacketProvider {}
impl INEFilterPacketProvider for NEFilterPacketProvider {}
pub trait INEFilterPacketProvider: Sized + std::ops::Deref {
    unsafe fn delayCurrentPacket_(&self, context: NEFilterPacketContext) -> NEPacket
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, delayCurrentPacket : context)
    }
    unsafe fn allowPacket_(&self, packet: NEPacket)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, allowPacket : packet)
    }
    unsafe fn packetHandler(&self) -> NEFilterPacketHandler
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, packetHandler)
    }
    unsafe fn setPacketHandler_(&self, packetHandler: NEFilterPacketHandler)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPacketHandler : packetHandler)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct NEFilterProviderConfiguration(pub id);
impl std::ops::Deref for NEFilterProviderConfiguration {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for NEFilterProviderConfiguration {}
impl NEFilterProviderConfiguration {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"NEFilterProviderConfiguration").unwrap(), alloc) })
    }
}
impl PNSSecureCoding for NEFilterProviderConfiguration {}
impl PNSCopying for NEFilterProviderConfiguration {}
impl INSObject for NEFilterProviderConfiguration {}
impl PNSObject for NEFilterProviderConfiguration {}
impl std::convert::TryFrom<NSObject> for NEFilterProviderConfiguration {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<NEFilterProviderConfiguration, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"NEFilterProviderConfiguration").unwrap())
        };
        if is_kind_of {
            Ok(NEFilterProviderConfiguration(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to NEFilterProviderConfiguration")
        }
    }
}
impl INEFilterProviderConfiguration for NEFilterProviderConfiguration {}
pub trait INEFilterProviderConfiguration: Sized + std::ops::Deref {
    unsafe fn filterBrowsers(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, filterBrowsers)
    }
    unsafe fn setFilterBrowsers_(&self, filterBrowsers: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setFilterBrowsers : filterBrowsers)
    }
    unsafe fn filterSockets(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, filterSockets)
    }
    unsafe fn setFilterSockets_(&self, filterSockets: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setFilterSockets : filterSockets)
    }
    unsafe fn filterPackets(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, filterPackets)
    }
    unsafe fn setFilterPackets_(&self, filterPackets: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setFilterPackets : filterPackets)
    }
    unsafe fn vendorConfiguration(&self) -> NSDictionary
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, vendorConfiguration)
    }
    unsafe fn setVendorConfiguration_(&self, vendorConfiguration: NSDictionary)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setVendorConfiguration : vendorConfiguration)
    }
    unsafe fn serverAddress(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, serverAddress)
    }
    unsafe fn setServerAddress_(&self, serverAddress: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setServerAddress : serverAddress)
    }
    unsafe fn username(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, username)
    }
    unsafe fn setUsername_(&self, username: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setUsername : username)
    }
    unsafe fn organization(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, organization)
    }
    unsafe fn setOrganization_(&self, organization: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setOrganization : organization)
    }
    unsafe fn passwordReference(&self) -> NSData
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, passwordReference)
    }
    unsafe fn setPasswordReference_(&self, passwordReference: NSData)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPasswordReference : passwordReference)
    }
    unsafe fn identityReference(&self) -> NSData
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, identityReference)
    }
    unsafe fn setIdentityReference_(&self, identityReference: NSData)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setIdentityReference : identityReference)
    }
    unsafe fn filterDataProviderBundleIdentifier(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, filterDataProviderBundleIdentifier)
    }
    unsafe fn setFilterDataProviderBundleIdentifier_(
        &self,
        filterDataProviderBundleIdentifier: NSString,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setFilterDataProviderBundleIdentifier : filterDataProviderBundleIdentifier)
    }
    unsafe fn filterPacketProviderBundleIdentifier(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, filterPacketProviderBundleIdentifier)
    }
    unsafe fn setFilterPacketProviderBundleIdentifier_(
        &self,
        filterPacketProviderBundleIdentifier: NSString,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setFilterPacketProviderBundleIdentifier : filterPacketProviderBundleIdentifier)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct NEFilterRule(pub id);
impl std::ops::Deref for NEFilterRule {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for NEFilterRule {}
impl NEFilterRule {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"NEFilterRule").unwrap(), alloc) })
    }
}
impl PNSSecureCoding for NEFilterRule {}
impl PNSCopying for NEFilterRule {}
impl INSObject for NEFilterRule {}
impl PNSObject for NEFilterRule {}
impl std::convert::TryFrom<NSObject> for NEFilterRule {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<NEFilterRule, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"NEFilterRule").unwrap()) };
        if is_kind_of {
            Ok(NEFilterRule(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to NEFilterRule")
        }
    }
}
impl INEFilterRule for NEFilterRule {}
pub trait INEFilterRule: Sized + std::ops::Deref {
    unsafe fn initWithNetworkRule_action_(
        &self,
        networkRule: NENetworkRule,
        action: NEFilterAction,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithNetworkRule : networkRule, action : action)
    }
    unsafe fn networkRule(&self) -> NENetworkRule
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, networkRule)
    }
    unsafe fn action(&self) -> NEFilterAction
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, action)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct NEFilterSettings(pub id);
impl std::ops::Deref for NEFilterSettings {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for NEFilterSettings {}
impl NEFilterSettings {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"NEFilterSettings").unwrap(), alloc) })
    }
}
impl PNSSecureCoding for NEFilterSettings {}
impl PNSCopying for NEFilterSettings {}
impl INSObject for NEFilterSettings {}
impl PNSObject for NEFilterSettings {}
impl std::convert::TryFrom<NSObject> for NEFilterSettings {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<NEFilterSettings, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"NEFilterSettings").unwrap()) };
        if is_kind_of {
            Ok(NEFilterSettings(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to NEFilterSettings")
        }
    }
}
impl INEFilterSettings for NEFilterSettings {}
pub trait INEFilterSettings: Sized + std::ops::Deref {
    unsafe fn initWithRules_defaultAction_(
        &self,
        rules: NSArray,
        defaultAction: NEFilterAction,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithRules : rules, defaultAction : defaultAction)
    }
    unsafe fn rules(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, rules)
    }
    unsafe fn defaultAction(&self) -> NEFilterAction
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, defaultAction)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct NEFlowMetaData(pub id);
impl std::ops::Deref for NEFlowMetaData {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for NEFlowMetaData {}
impl NEFlowMetaData {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"NEFlowMetaData").unwrap(), alloc) })
    }
}
impl PNSCopying for NEFlowMetaData {}
impl PNSSecureCoding for NEFlowMetaData {}
impl INSObject for NEFlowMetaData {}
impl PNSObject for NEFlowMetaData {}
impl std::convert::TryFrom<NSObject> for NEFlowMetaData {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<NEFlowMetaData, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"NEFlowMetaData").unwrap()) };
        if is_kind_of {
            Ok(NEFlowMetaData(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to NEFlowMetaData")
        }
    }
}
impl INEFlowMetaData for NEFlowMetaData {}
pub trait INEFlowMetaData: Sized + std::ops::Deref {
    unsafe fn sourceAppUniqueIdentifier(&self) -> NSData
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, sourceAppUniqueIdentifier)
    }
    unsafe fn sourceAppSigningIdentifier(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, sourceAppSigningIdentifier)
    }
    unsafe fn sourceAppAuditToken(&self) -> NSData
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, sourceAppAuditToken)
    }
    unsafe fn filterFlowIdentifier(&self) -> NSUUID
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, filterFlowIdentifier)
    }
}
pub type NEHotspotNetworkSecurityType = NSInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct NEHotspotNetwork(pub id);
impl std::ops::Deref for NEHotspotNetwork {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for NEHotspotNetwork {}
impl NEHotspotNetwork {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"NEHotspotNetwork").unwrap(), alloc) })
    }
}
impl INSObject for NEHotspotNetwork {}
impl PNSObject for NEHotspotNetwork {}
impl std::convert::TryFrom<NSObject> for NEHotspotNetwork {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<NEHotspotNetwork, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"NEHotspotNetwork").unwrap()) };
        if is_kind_of {
            Ok(NEHotspotNetwork(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to NEHotspotNetwork")
        }
    }
}
impl INEHotspotNetwork for NEHotspotNetwork {}
pub trait INEHotspotNetwork: Sized + std::ops::Deref {
    unsafe fn SSID(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, SSID)
    }
    unsafe fn BSSID(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, BSSID)
    }
    unsafe fn securityType(&self) -> NEHotspotNetworkSecurityType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, securityType)
    }
    unsafe fn fetchCurrentWithCompletionHandler_(completionHandler: *mut ::std::os::raw::c_void)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NEHotspotNetwork").unwrap(), fetchCurrentWithCompletionHandler : completionHandler)
    }
}
pub type NEHotspotHelperCommandType = NSInteger;
pub type NEHotspotHelperResult = NSInteger;
pub type NEHotspotHelperConfidence = NSInteger;
impl NEHotspotNetwork_HotspotHelper for NEHotspotNetwork {}
pub trait NEHotspotNetwork_HotspotHelper: Sized + std::ops::Deref {
    unsafe fn setConfidence_(&self, confidence: NEHotspotHelperConfidence)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setConfidence : confidence)
    }
    unsafe fn setPassword_(&self, password: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPassword : password)
    }
    unsafe fn signalStrength(&self) -> f64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, signalStrength)
    }
    unsafe fn isSecure(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isSecure)
    }
    unsafe fn didAutoJoin(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, didAutoJoin)
    }
    unsafe fn didJustJoin(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, didJustJoin)
    }
    unsafe fn isChosenHelper(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isChosenHelper)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct NEHotspotHelperCommand(pub id);
impl std::ops::Deref for NEHotspotHelperCommand {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for NEHotspotHelperCommand {}
impl NEHotspotHelperCommand {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"NEHotspotHelperCommand").unwrap(), alloc) })
    }
}
impl INSObject for NEHotspotHelperCommand {}
impl PNSObject for NEHotspotHelperCommand {}
impl std::convert::TryFrom<NSObject> for NEHotspotHelperCommand {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<NEHotspotHelperCommand, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"NEHotspotHelperCommand").unwrap()) };
        if is_kind_of {
            Ok(NEHotspotHelperCommand(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to NEHotspotHelperCommand")
        }
    }
}
impl INEHotspotHelperCommand for NEHotspotHelperCommand {}
pub trait INEHotspotHelperCommand: Sized + std::ops::Deref {
    unsafe fn createResponse_(&self, result: NEHotspotHelperResult) -> NEHotspotHelperResponse
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, createResponse : result)
    }
    unsafe fn createTCPConnection_(&self, endpoint: NWEndpoint) -> NWTCPConnection
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, createTCPConnection : endpoint)
    }
    unsafe fn createUDPSession_(&self, endpoint: NWEndpoint) -> NWUDPSession
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, createUDPSession : endpoint)
    }
    unsafe fn commandType(&self) -> NEHotspotHelperCommandType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, commandType)
    }
    unsafe fn network(&self) -> NEHotspotNetwork
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, network)
    }
    unsafe fn networkList(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, networkList)
    }
    unsafe fn interface(&self) -> nw_interface_t
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, interface)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct NEHotspotHelperResponse(pub id);
impl std::ops::Deref for NEHotspotHelperResponse {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for NEHotspotHelperResponse {}
impl NEHotspotHelperResponse {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"NEHotspotHelperResponse").unwrap(), alloc) })
    }
}
impl INSObject for NEHotspotHelperResponse {}
impl PNSObject for NEHotspotHelperResponse {}
impl std::convert::TryFrom<NSObject> for NEHotspotHelperResponse {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<NEHotspotHelperResponse, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"NEHotspotHelperResponse").unwrap()) };
        if is_kind_of {
            Ok(NEHotspotHelperResponse(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to NEHotspotHelperResponse")
        }
    }
}
impl INEHotspotHelperResponse for NEHotspotHelperResponse {}
pub trait INEHotspotHelperResponse: Sized + std::ops::Deref {
    unsafe fn setNetwork_(&self, network: NEHotspotNetwork)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setNetwork : network)
    }
    unsafe fn setNetworkList_(&self, networkList: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setNetworkList : networkList)
    }
    unsafe fn deliver(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, deliver)
    }
}
pub type NEHotspotHelperHandler = *mut ::std::os::raw::c_void;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct NEHotspotHelper(pub id);
impl std::ops::Deref for NEHotspotHelper {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for NEHotspotHelper {}
impl NEHotspotHelper {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"NEHotspotHelper").unwrap(), alloc) })
    }
}
impl INSObject for NEHotspotHelper {}
impl PNSObject for NEHotspotHelper {}
impl std::convert::TryFrom<NSObject> for NEHotspotHelper {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<NEHotspotHelper, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"NEHotspotHelper").unwrap()) };
        if is_kind_of {
            Ok(NEHotspotHelper(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to NEHotspotHelper")
        }
    }
}
impl INEHotspotHelper for NEHotspotHelper {}
pub trait INEHotspotHelper: Sized + std::ops::Deref {
    unsafe fn registerWithOptions_queue_handler_(
        options: NSDictionary,
        queue: NSObject,
        handler: NEHotspotHelperHandler,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NEHotspotHelper").unwrap(), registerWithOptions : options, queue : queue, handler : handler)
    }
    unsafe fn logoff_(network: NEHotspotNetwork) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NEHotspotHelper").unwrap(), logoff : network)
    }
    unsafe fn supportedNetworkInterfaces() -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NEHotspotHelper").unwrap(), supportedNetworkInterfaces)
    }
}
pub trait NSMutableURLRequest_NEHotspotHelper: Sized + std::ops::Deref {
    unsafe fn bindToHotspotHelperCommand_(&self, command: NEHotspotHelperCommand)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, bindToHotspotHelperCommand : command)
    }
}
pub type NEHotspotConfigurationEAPType = NSInteger;
pub type NEHotspotConfigurationTTLSInnerAuthenticationType = NSInteger;
pub type NEHotspotConfigurationEAPTLSVersion = NSInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct NEHotspotHS20Settings(pub id);
impl std::ops::Deref for NEHotspotHS20Settings {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for NEHotspotHS20Settings {}
impl NEHotspotHS20Settings {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"NEHotspotHS20Settings").unwrap(), alloc) })
    }
}
impl PNSCopying for NEHotspotHS20Settings {}
impl PNSSecureCoding for NEHotspotHS20Settings {}
impl INSObject for NEHotspotHS20Settings {}
impl PNSObject for NEHotspotHS20Settings {}
impl std::convert::TryFrom<NSObject> for NEHotspotHS20Settings {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<NEHotspotHS20Settings, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"NEHotspotHS20Settings").unwrap()) };
        if is_kind_of {
            Ok(NEHotspotHS20Settings(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to NEHotspotHS20Settings")
        }
    }
}
impl INEHotspotHS20Settings for NEHotspotHS20Settings {}
pub trait INEHotspotHS20Settings: Sized + std::ops::Deref {
    unsafe fn initWithDomainName_roamingEnabled_(
        &self,
        domainName: NSString,
        roamingEnabled: BOOL,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithDomainName : domainName, roamingEnabled : roamingEnabled)
    }
    unsafe fn domainName(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, domainName)
    }
    unsafe fn isRoamingEnabled(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isRoamingEnabled)
    }
    unsafe fn setRoamingEnabled_(&self, roamingEnabled: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRoamingEnabled : roamingEnabled)
    }
    unsafe fn roamingConsortiumOIs(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, roamingConsortiumOIs)
    }
    unsafe fn setRoamingConsortiumOIs_(&self, roamingConsortiumOIs: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRoamingConsortiumOIs : roamingConsortiumOIs)
    }
    unsafe fn naiRealmNames(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, naiRealmNames)
    }
    unsafe fn setNaiRealmNames_(&self, naiRealmNames: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setNaiRealmNames : naiRealmNames)
    }
    unsafe fn MCCAndMNCs(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, MCCAndMNCs)
    }
    unsafe fn setMCCAndMNCs_(&self, MCCAndMNCs: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMCCAndMNCs : MCCAndMNCs)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct NEHotspotEAPSettings(pub id);
impl std::ops::Deref for NEHotspotEAPSettings {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for NEHotspotEAPSettings {}
impl NEHotspotEAPSettings {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"NEHotspotEAPSettings").unwrap(), alloc) })
    }
}
impl PNSCopying for NEHotspotEAPSettings {}
impl PNSSecureCoding for NEHotspotEAPSettings {}
impl INSObject for NEHotspotEAPSettings {}
impl PNSObject for NEHotspotEAPSettings {}
impl std::convert::TryFrom<NSObject> for NEHotspotEAPSettings {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<NEHotspotEAPSettings, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"NEHotspotEAPSettings").unwrap()) };
        if is_kind_of {
            Ok(NEHotspotEAPSettings(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to NEHotspotEAPSettings")
        }
    }
}
impl INEHotspotEAPSettings for NEHotspotEAPSettings {}
pub trait INEHotspotEAPSettings: Sized + std::ops::Deref {
    unsafe fn setIdentity_(&self, identity: SecIdentityRef) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setIdentity : identity)
    }
    unsafe fn setTrustedServerCertificates_(&self, certificates: NSArray) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setTrustedServerCertificates : certificates)
    }
    unsafe fn supportedEAPTypes(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, supportedEAPTypes)
    }
    unsafe fn setSupportedEAPTypes_(&self, supportedEAPTypes: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSupportedEAPTypes : supportedEAPTypes)
    }
    unsafe fn username(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, username)
    }
    unsafe fn setUsername_(&self, username: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setUsername : username)
    }
    unsafe fn outerIdentity(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, outerIdentity)
    }
    unsafe fn setOuterIdentity_(&self, outerIdentity: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setOuterIdentity : outerIdentity)
    }
    unsafe fn ttlsInnerAuthenticationType(
        &self,
    ) -> NEHotspotConfigurationTTLSInnerAuthenticationType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, ttlsInnerAuthenticationType)
    }
    unsafe fn setTtlsInnerAuthenticationType_(
        &self,
        ttlsInnerAuthenticationType: NEHotspotConfigurationTTLSInnerAuthenticationType,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setTtlsInnerAuthenticationType : ttlsInnerAuthenticationType)
    }
    unsafe fn password(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, password)
    }
    unsafe fn setPassword_(&self, password: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPassword : password)
    }
    unsafe fn trustedServerNames(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, trustedServerNames)
    }
    unsafe fn setTrustedServerNames_(&self, trustedServerNames: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setTrustedServerNames : trustedServerNames)
    }
    unsafe fn isTLSClientCertificateRequired(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isTLSClientCertificateRequired)
    }
    unsafe fn setTlsClientCertificateRequired_(&self, tlsClientCertificateRequired: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setTlsClientCertificateRequired : tlsClientCertificateRequired)
    }
    unsafe fn preferredTLSVersion(&self) -> NEHotspotConfigurationEAPTLSVersion
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, preferredTLSVersion)
    }
    unsafe fn setPreferredTLSVersion_(
        &self,
        preferredTLSVersion: NEHotspotConfigurationEAPTLSVersion,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPreferredTLSVersion : preferredTLSVersion)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct NEHotspotConfiguration(pub id);
impl std::ops::Deref for NEHotspotConfiguration {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for NEHotspotConfiguration {}
impl NEHotspotConfiguration {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"NEHotspotConfiguration").unwrap(), alloc) })
    }
}
impl PNSCopying for NEHotspotConfiguration {}
impl PNSSecureCoding for NEHotspotConfiguration {}
impl INSObject for NEHotspotConfiguration {}
impl PNSObject for NEHotspotConfiguration {}
impl std::convert::TryFrom<NSObject> for NEHotspotConfiguration {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<NEHotspotConfiguration, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"NEHotspotConfiguration").unwrap()) };
        if is_kind_of {
            Ok(NEHotspotConfiguration(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to NEHotspotConfiguration")
        }
    }
}
impl INEHotspotConfiguration for NEHotspotConfiguration {}
pub trait INEHotspotConfiguration: Sized + std::ops::Deref {
    unsafe fn initWithSSID_(&self, SSID: NSString) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithSSID : SSID)
    }
    unsafe fn initWithSSID_passphrase_isWEP_(
        &self,
        SSID: NSString,
        passphrase: NSString,
        isWEP: BOOL,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithSSID : SSID, passphrase : passphrase, isWEP : isWEP)
    }
    unsafe fn initWithSSID_eapSettings_(
        &self,
        SSID: NSString,
        eapSettings: NEHotspotEAPSettings,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithSSID : SSID, eapSettings : eapSettings)
    }
    unsafe fn initWithHS20Settings_eapSettings_(
        &self,
        hs20Settings: NEHotspotHS20Settings,
        eapSettings: NEHotspotEAPSettings,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithHS20Settings : hs20Settings, eapSettings : eapSettings)
    }
    unsafe fn initWithSSIDPrefix_(&self, SSIDPrefix: NSString) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithSSIDPrefix : SSIDPrefix)
    }
    unsafe fn initWithSSIDPrefix_passphrase_isWEP_(
        &self,
        SSIDPrefix: NSString,
        passphrase: NSString,
        isWEP: BOOL,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithSSIDPrefix : SSIDPrefix, passphrase : passphrase, isWEP : isWEP)
    }
    unsafe fn SSID(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, SSID)
    }
    unsafe fn SSIDPrefix(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, SSIDPrefix)
    }
    unsafe fn joinOnce(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, joinOnce)
    }
    unsafe fn setJoinOnce_(&self, joinOnce: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setJoinOnce : joinOnce)
    }
    unsafe fn lifeTimeInDays(&self) -> NSNumber
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, lifeTimeInDays)
    }
    unsafe fn setLifeTimeInDays_(&self, lifeTimeInDays: NSNumber)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setLifeTimeInDays : lifeTimeInDays)
    }
    unsafe fn hidden(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, hidden)
    }
    unsafe fn setHidden_(&self, hidden: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setHidden : hidden)
    }
}
pub type NEHotspotConfigurationError = NSInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct NEHotspotConfigurationManager(pub id);
impl std::ops::Deref for NEHotspotConfigurationManager {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for NEHotspotConfigurationManager {}
impl NEHotspotConfigurationManager {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"NEHotspotConfigurationManager").unwrap(), alloc) })
    }
}
impl INSObject for NEHotspotConfigurationManager {}
impl PNSObject for NEHotspotConfigurationManager {}
impl std::convert::TryFrom<NSObject> for NEHotspotConfigurationManager {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<NEHotspotConfigurationManager, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"NEHotspotConfigurationManager").unwrap())
        };
        if is_kind_of {
            Ok(NEHotspotConfigurationManager(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to NEHotspotConfigurationManager")
        }
    }
}
impl INEHotspotConfigurationManager for NEHotspotConfigurationManager {}
pub trait INEHotspotConfigurationManager: Sized + std::ops::Deref {
    unsafe fn applyConfiguration_completionHandler_(
        &self,
        configuration: NEHotspotConfiguration,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, applyConfiguration : configuration, completionHandler : completionHandler)
    }
    unsafe fn removeConfigurationForSSID_(&self, SSID: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, removeConfigurationForSSID : SSID)
    }
    unsafe fn removeConfigurationForHS20DomainName_(&self, domainName: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, removeConfigurationForHS20DomainName : domainName)
    }
    unsafe fn getConfiguredSSIDsWithCompletionHandler_(
        &self,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, getConfiguredSSIDsWithCompletionHandler : completionHandler)
    }
    unsafe fn joinAccessoryHotspot_passphrase_completionHandler_(
        &self,
        accessory: ASAccessory,
        passphrase: NSString,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, joinAccessoryHotspot : accessory, passphrase : passphrase, completionHandler : completionHandler)
    }
    unsafe fn joinAccessoryHotspotWithoutSecurity_completionHandler_(
        &self,
        accessory: ASAccessory,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, joinAccessoryHotspotWithoutSecurity : accessory, completionHandler : completionHandler)
    }
    unsafe fn sharedManager() -> NEHotspotConfigurationManager
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NEHotspotConfigurationManager").unwrap(), sharedManager)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct NEIPv4Settings(pub id);
impl std::ops::Deref for NEIPv4Settings {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for NEIPv4Settings {}
impl NEIPv4Settings {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"NEIPv4Settings").unwrap(), alloc) })
    }
}
impl PNSSecureCoding for NEIPv4Settings {}
impl PNSCopying for NEIPv4Settings {}
impl INSObject for NEIPv4Settings {}
impl PNSObject for NEIPv4Settings {}
impl std::convert::TryFrom<NSObject> for NEIPv4Settings {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<NEIPv4Settings, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"NEIPv4Settings").unwrap()) };
        if is_kind_of {
            Ok(NEIPv4Settings(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to NEIPv4Settings")
        }
    }
}
impl INEIPv4Settings for NEIPv4Settings {}
pub trait INEIPv4Settings: Sized + std::ops::Deref {
    unsafe fn initWithAddresses_subnetMasks_(
        &self,
        addresses: NSArray,
        subnetMasks: NSArray,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithAddresses : addresses, subnetMasks : subnetMasks)
    }
    unsafe fn addresses(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, addresses)
    }
    unsafe fn subnetMasks(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, subnetMasks)
    }
    unsafe fn router(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, router)
    }
    unsafe fn setRouter_(&self, router: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRouter : router)
    }
    unsafe fn includedRoutes(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, includedRoutes)
    }
    unsafe fn setIncludedRoutes_(&self, includedRoutes: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setIncludedRoutes : includedRoutes)
    }
    unsafe fn excludedRoutes(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, excludedRoutes)
    }
    unsafe fn setExcludedRoutes_(&self, excludedRoutes: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setExcludedRoutes : excludedRoutes)
    }
    unsafe fn settingsWithAutomaticAddressing() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NEIPv4Settings").unwrap(), settingsWithAutomaticAddressing)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct NEIPv4Route(pub id);
impl std::ops::Deref for NEIPv4Route {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for NEIPv4Route {}
impl NEIPv4Route {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"NEIPv4Route").unwrap(), alloc) })
    }
}
impl PNSSecureCoding for NEIPv4Route {}
impl PNSCopying for NEIPv4Route {}
impl INSObject for NEIPv4Route {}
impl PNSObject for NEIPv4Route {}
impl std::convert::TryFrom<NSObject> for NEIPv4Route {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<NEIPv4Route, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"NEIPv4Route").unwrap()) };
        if is_kind_of {
            Ok(NEIPv4Route(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to NEIPv4Route")
        }
    }
}
impl INEIPv4Route for NEIPv4Route {}
pub trait INEIPv4Route: Sized + std::ops::Deref {
    unsafe fn initWithDestinationAddress_subnetMask_(
        &self,
        address: NSString,
        subnetMask: NSString,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithDestinationAddress : address, subnetMask : subnetMask)
    }
    unsafe fn destinationAddress(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, destinationAddress)
    }
    unsafe fn destinationSubnetMask(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, destinationSubnetMask)
    }
    unsafe fn gatewayAddress(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, gatewayAddress)
    }
    unsafe fn setGatewayAddress_(&self, gatewayAddress: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setGatewayAddress : gatewayAddress)
    }
    unsafe fn defaultRoute() -> NEIPv4Route
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NEIPv4Route").unwrap(), defaultRoute)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct NEIPv6Settings(pub id);
impl std::ops::Deref for NEIPv6Settings {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for NEIPv6Settings {}
impl NEIPv6Settings {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"NEIPv6Settings").unwrap(), alloc) })
    }
}
impl PNSSecureCoding for NEIPv6Settings {}
impl PNSCopying for NEIPv6Settings {}
impl INSObject for NEIPv6Settings {}
impl PNSObject for NEIPv6Settings {}
impl std::convert::TryFrom<NSObject> for NEIPv6Settings {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<NEIPv6Settings, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"NEIPv6Settings").unwrap()) };
        if is_kind_of {
            Ok(NEIPv6Settings(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to NEIPv6Settings")
        }
    }
}
impl INEIPv6Settings for NEIPv6Settings {}
pub trait INEIPv6Settings: Sized + std::ops::Deref {
    unsafe fn initWithAddresses_networkPrefixLengths_(
        &self,
        addresses: NSArray,
        networkPrefixLengths: NSArray,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithAddresses : addresses, networkPrefixLengths : networkPrefixLengths)
    }
    unsafe fn addresses(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, addresses)
    }
    unsafe fn networkPrefixLengths(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, networkPrefixLengths)
    }
    unsafe fn includedRoutes(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, includedRoutes)
    }
    unsafe fn setIncludedRoutes_(&self, includedRoutes: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setIncludedRoutes : includedRoutes)
    }
    unsafe fn excludedRoutes(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, excludedRoutes)
    }
    unsafe fn setExcludedRoutes_(&self, excludedRoutes: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setExcludedRoutes : excludedRoutes)
    }
    unsafe fn settingsWithAutomaticAddressing() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NEIPv6Settings").unwrap(), settingsWithAutomaticAddressing)
    }
    unsafe fn settingsWithLinkLocalAddressing() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NEIPv6Settings").unwrap(), settingsWithLinkLocalAddressing)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct NEIPv6Route(pub id);
impl std::ops::Deref for NEIPv6Route {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for NEIPv6Route {}
impl NEIPv6Route {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"NEIPv6Route").unwrap(), alloc) })
    }
}
impl PNSSecureCoding for NEIPv6Route {}
impl PNSCopying for NEIPv6Route {}
impl INSObject for NEIPv6Route {}
impl PNSObject for NEIPv6Route {}
impl std::convert::TryFrom<NSObject> for NEIPv6Route {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<NEIPv6Route, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"NEIPv6Route").unwrap()) };
        if is_kind_of {
            Ok(NEIPv6Route(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to NEIPv6Route")
        }
    }
}
impl INEIPv6Route for NEIPv6Route {}
pub trait INEIPv6Route: Sized + std::ops::Deref {
    unsafe fn initWithDestinationAddress_networkPrefixLength_(
        &self,
        address: NSString,
        networkPrefixLength: NSNumber,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithDestinationAddress : address, networkPrefixLength : networkPrefixLength)
    }
    unsafe fn destinationAddress(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, destinationAddress)
    }
    unsafe fn destinationNetworkPrefixLength(&self) -> NSNumber
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, destinationNetworkPrefixLength)
    }
    unsafe fn gatewayAddress(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, gatewayAddress)
    }
    unsafe fn setGatewayAddress_(&self, gatewayAddress: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setGatewayAddress : gatewayAddress)
    }
    unsafe fn defaultRoute() -> NEIPv6Route
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NEIPv6Route").unwrap(), defaultRoute)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct NETunnelNetworkSettings(pub id);
impl std::ops::Deref for NETunnelNetworkSettings {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for NETunnelNetworkSettings {}
impl NETunnelNetworkSettings {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"NETunnelNetworkSettings").unwrap(), alloc) })
    }
}
impl PNSSecureCoding for NETunnelNetworkSettings {}
impl PNSCopying for NETunnelNetworkSettings {}
impl INSObject for NETunnelNetworkSettings {}
impl PNSObject for NETunnelNetworkSettings {}
impl std::convert::TryFrom<NSObject> for NETunnelNetworkSettings {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<NETunnelNetworkSettings, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"NETunnelNetworkSettings").unwrap()) };
        if is_kind_of {
            Ok(NETunnelNetworkSettings(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to NETunnelNetworkSettings")
        }
    }
}
impl INETunnelNetworkSettings for NETunnelNetworkSettings {}
pub trait INETunnelNetworkSettings: Sized + std::ops::Deref {
    unsafe fn initWithTunnelRemoteAddress_(&self, address: NSString) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithTunnelRemoteAddress : address)
    }
    unsafe fn tunnelRemoteAddress(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, tunnelRemoteAddress)
    }
    unsafe fn DNSSettings(&self) -> NEDNSSettings
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, DNSSettings)
    }
    unsafe fn setDNSSettings_(&self, DNSSettings: NEDNSSettings)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDNSSettings : DNSSettings)
    }
    unsafe fn proxySettings(&self) -> NEProxySettings
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, proxySettings)
    }
    unsafe fn setProxySettings_(&self, proxySettings: NEProxySettings)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setProxySettings : proxySettings)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct NEPacketTunnelNetworkSettings(pub id);
impl std::ops::Deref for NEPacketTunnelNetworkSettings {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for NEPacketTunnelNetworkSettings {}
impl NEPacketTunnelNetworkSettings {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"NEPacketTunnelNetworkSettings").unwrap(), alloc) })
    }
}
impl INETunnelNetworkSettings for NEPacketTunnelNetworkSettings {}
impl PNSSecureCoding for NEPacketTunnelNetworkSettings {}
impl PNSCopying for NEPacketTunnelNetworkSettings {}
impl From<NEPacketTunnelNetworkSettings> for NETunnelNetworkSettings {
    fn from(child: NEPacketTunnelNetworkSettings) -> NETunnelNetworkSettings {
        NETunnelNetworkSettings(child.0)
    }
}
impl std::convert::TryFrom<NETunnelNetworkSettings> for NEPacketTunnelNetworkSettings {
    type Error = &'static str;
    fn try_from(
        parent: NETunnelNetworkSettings,
    ) -> Result<NEPacketTunnelNetworkSettings, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"NEPacketTunnelNetworkSettings").unwrap())
        };
        if is_kind_of {
            Ok(NEPacketTunnelNetworkSettings(parent.0))
        } else {
            Err ("This NETunnelNetworkSettings cannot be downcasted to NEPacketTunnelNetworkSettings" ,)
        }
    }
}
impl INSObject for NEPacketTunnelNetworkSettings {}
impl PNSObject for NEPacketTunnelNetworkSettings {}
impl INEPacketTunnelNetworkSettings for NEPacketTunnelNetworkSettings {}
pub trait INEPacketTunnelNetworkSettings: Sized + std::ops::Deref {
    unsafe fn IPv4Settings(&self) -> NEIPv4Settings
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, IPv4Settings)
    }
    unsafe fn setIPv4Settings_(&self, IPv4Settings: NEIPv4Settings)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setIPv4Settings : IPv4Settings)
    }
    unsafe fn IPv6Settings(&self) -> NEIPv6Settings
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, IPv6Settings)
    }
    unsafe fn setIPv6Settings_(&self, IPv6Settings: NEIPv6Settings)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setIPv6Settings : IPv6Settings)
    }
    unsafe fn tunnelOverheadBytes(&self) -> NSNumber
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, tunnelOverheadBytes)
    }
    unsafe fn setTunnelOverheadBytes_(&self, tunnelOverheadBytes: NSNumber)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setTunnelOverheadBytes : tunnelOverheadBytes)
    }
    unsafe fn MTU(&self) -> NSNumber
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, MTU)
    }
    unsafe fn setMTU_(&self, MTU: NSNumber)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMTU : MTU)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct NEEthernetTunnelNetworkSettings(pub id);
impl std::ops::Deref for NEEthernetTunnelNetworkSettings {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for NEEthernetTunnelNetworkSettings {}
impl NEEthernetTunnelNetworkSettings {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"NEEthernetTunnelNetworkSettings").unwrap(), alloc) })
    }
}
impl INEPacketTunnelNetworkSettings for NEEthernetTunnelNetworkSettings {}
impl From<NEEthernetTunnelNetworkSettings> for NEPacketTunnelNetworkSettings {
    fn from(child: NEEthernetTunnelNetworkSettings) -> NEPacketTunnelNetworkSettings {
        NEPacketTunnelNetworkSettings(child.0)
    }
}
impl std::convert::TryFrom<NEPacketTunnelNetworkSettings> for NEEthernetTunnelNetworkSettings {
    type Error = &'static str;
    fn try_from(
        parent: NEPacketTunnelNetworkSettings,
    ) -> Result<NEEthernetTunnelNetworkSettings, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"NEEthernetTunnelNetworkSettings").unwrap())
        };
        if is_kind_of {
            Ok(NEEthernetTunnelNetworkSettings(parent.0))
        } else {
            Err ("This NEPacketTunnelNetworkSettings cannot be downcasted to NEEthernetTunnelNetworkSettings" ,)
        }
    }
}
impl INETunnelNetworkSettings for NEEthernetTunnelNetworkSettings {}
impl PNSSecureCoding for NEEthernetTunnelNetworkSettings {}
impl PNSCopying for NEEthernetTunnelNetworkSettings {}
impl INSObject for NEEthernetTunnelNetworkSettings {}
impl PNSObject for NEEthernetTunnelNetworkSettings {}
impl INEEthernetTunnelNetworkSettings for NEEthernetTunnelNetworkSettings {}
pub trait INEEthernetTunnelNetworkSettings: Sized + std::ops::Deref {
    unsafe fn initWithTunnelRemoteAddress_ethernetAddress_mtu_(
        &self,
        address: NSString,
        ethernetAddress: NSString,
        mtu: NSInteger,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithTunnelRemoteAddress : address, ethernetAddress : ethernetAddress, mtu : mtu)
    }
    unsafe fn ethernetAddress(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, ethernetAddress)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct NEPacketTunnelProvider(pub id);
impl std::ops::Deref for NEPacketTunnelProvider {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for NEPacketTunnelProvider {}
impl NEPacketTunnelProvider {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"NEPacketTunnelProvider").unwrap(), alloc) })
    }
}
impl INETunnelProvider for NEPacketTunnelProvider {}
impl std::convert::TryFrom<NETunnelProvider> for NEPacketTunnelProvider {
    type Error = &'static str;
    fn try_from(parent: NETunnelProvider) -> Result<NEPacketTunnelProvider, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"NEPacketTunnelProvider").unwrap()) };
        if is_kind_of {
            Ok(NEPacketTunnelProvider(parent.0))
        } else {
            Err("This NETunnelProvider cannot be downcasted to NEPacketTunnelProvider")
        }
    }
}
impl INEProvider for NEPacketTunnelProvider {}
impl INSObject for NEPacketTunnelProvider {}
impl PNSObject for NEPacketTunnelProvider {}
impl INEPacketTunnelProvider for NEPacketTunnelProvider {}
pub trait INEPacketTunnelProvider: Sized + std::ops::Deref {
    unsafe fn startTunnelWithOptions_completionHandler_(
        &self,
        options: NSDictionary,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, startTunnelWithOptions : options, completionHandler : completionHandler)
    }
    unsafe fn stopTunnelWithReason_completionHandler_(
        &self,
        reason: NEProviderStopReason,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, stopTunnelWithReason : reason, completionHandler : completionHandler)
    }
    unsafe fn cancelTunnelWithError_(&self, error: NSError)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, cancelTunnelWithError : error)
    }
    unsafe fn createTCPConnectionThroughTunnelToEndpoint_enableTLS_TLSParameters_delegate_(
        &self,
        remoteEndpoint: NWEndpoint,
        enableTLS: BOOL,
        TLSParameters: NWTLSParameters,
        delegate: id,
    ) -> NWTCPConnection
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, createTCPConnectionThroughTunnelToEndpoint : remoteEndpoint, enableTLS : enableTLS, TLSParameters : TLSParameters, delegate : delegate)
    }
    unsafe fn createUDPSessionThroughTunnelToEndpoint_fromEndpoint_(
        &self,
        remoteEndpoint: NWEndpoint,
        localEndpoint: NWHostEndpoint,
    ) -> NWUDPSession
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, createUDPSessionThroughTunnelToEndpoint : remoteEndpoint, fromEndpoint : localEndpoint)
    }
    unsafe fn packetFlow(&self) -> NEPacketTunnelFlow
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, packetFlow)
    }
    unsafe fn virtualInterface(&self) -> nw_interface_t
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, virtualInterface)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct NEEthernetTunnelProvider(pub id);
impl std::ops::Deref for NEEthernetTunnelProvider {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for NEEthernetTunnelProvider {}
impl NEEthernetTunnelProvider {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"NEEthernetTunnelProvider").unwrap(), alloc) })
    }
}
impl INEPacketTunnelProvider for NEEthernetTunnelProvider {}
impl From<NEEthernetTunnelProvider> for NEPacketTunnelProvider {
    fn from(child: NEEthernetTunnelProvider) -> NEPacketTunnelProvider {
        NEPacketTunnelProvider(child.0)
    }
}
impl std::convert::TryFrom<NEPacketTunnelProvider> for NEEthernetTunnelProvider {
    type Error = &'static str;
    fn try_from(parent: NEPacketTunnelProvider) -> Result<NEEthernetTunnelProvider, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"NEEthernetTunnelProvider").unwrap()) };
        if is_kind_of {
            Ok(NEEthernetTunnelProvider(parent.0))
        } else {
            Err("This NEPacketTunnelProvider cannot be downcasted to NEEthernetTunnelProvider")
        }
    }
}
impl INETunnelProvider for NEEthernetTunnelProvider {}
impl INEProvider for NEEthernetTunnelProvider {}
impl INSObject for NEEthernetTunnelProvider {}
impl PNSObject for NEEthernetTunnelProvider {}
impl INEEthernetTunnelProvider for NEEthernetTunnelProvider {}
pub trait INEEthernetTunnelProvider: Sized + std::ops::Deref {}
pub type NEOnDemandRuleAction = NSInteger;
pub type NEOnDemandRuleInterfaceType = NSInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct NEOnDemandRule(pub id);
impl std::ops::Deref for NEOnDemandRule {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for NEOnDemandRule {}
impl NEOnDemandRule {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"NEOnDemandRule").unwrap(), alloc) })
    }
}
impl PNSSecureCoding for NEOnDemandRule {}
impl PNSCopying for NEOnDemandRule {}
impl INSObject for NEOnDemandRule {}
impl PNSObject for NEOnDemandRule {}
impl std::convert::TryFrom<NSObject> for NEOnDemandRule {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<NEOnDemandRule, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"NEOnDemandRule").unwrap()) };
        if is_kind_of {
            Ok(NEOnDemandRule(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to NEOnDemandRule")
        }
    }
}
impl INEOnDemandRule for NEOnDemandRule {}
pub trait INEOnDemandRule: Sized + std::ops::Deref {
    unsafe fn action(&self) -> NEOnDemandRuleAction
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, action)
    }
    unsafe fn DNSSearchDomainMatch(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, DNSSearchDomainMatch)
    }
    unsafe fn setDNSSearchDomainMatch_(&self, DNSSearchDomainMatch: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDNSSearchDomainMatch : DNSSearchDomainMatch)
    }
    unsafe fn DNSServerAddressMatch(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, DNSServerAddressMatch)
    }
    unsafe fn setDNSServerAddressMatch_(&self, DNSServerAddressMatch: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDNSServerAddressMatch : DNSServerAddressMatch)
    }
    unsafe fn interfaceTypeMatch(&self) -> NEOnDemandRuleInterfaceType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, interfaceTypeMatch)
    }
    unsafe fn setInterfaceTypeMatch_(&self, interfaceTypeMatch: NEOnDemandRuleInterfaceType)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInterfaceTypeMatch : interfaceTypeMatch)
    }
    unsafe fn SSIDMatch(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, SSIDMatch)
    }
    unsafe fn setSSIDMatch_(&self, SSIDMatch: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSSIDMatch : SSIDMatch)
    }
    unsafe fn probeURL(&self) -> NSURL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, probeURL)
    }
    unsafe fn setProbeURL_(&self, probeURL: NSURL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setProbeURL : probeURL)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct NEOnDemandRuleConnect(pub id);
impl std::ops::Deref for NEOnDemandRuleConnect {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for NEOnDemandRuleConnect {}
impl NEOnDemandRuleConnect {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"NEOnDemandRuleConnect").unwrap(), alloc) })
    }
}
impl INEOnDemandRule for NEOnDemandRuleConnect {}
impl PNSSecureCoding for NEOnDemandRuleConnect {}
impl PNSCopying for NEOnDemandRuleConnect {}
impl From<NEOnDemandRuleConnect> for NEOnDemandRule {
    fn from(child: NEOnDemandRuleConnect) -> NEOnDemandRule {
        NEOnDemandRule(child.0)
    }
}
impl std::convert::TryFrom<NEOnDemandRule> for NEOnDemandRuleConnect {
    type Error = &'static str;
    fn try_from(parent: NEOnDemandRule) -> Result<NEOnDemandRuleConnect, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"NEOnDemandRuleConnect").unwrap()) };
        if is_kind_of {
            Ok(NEOnDemandRuleConnect(parent.0))
        } else {
            Err("This NEOnDemandRule cannot be downcasted to NEOnDemandRuleConnect")
        }
    }
}
impl INSObject for NEOnDemandRuleConnect {}
impl PNSObject for NEOnDemandRuleConnect {}
impl INEOnDemandRuleConnect for NEOnDemandRuleConnect {}
pub trait INEOnDemandRuleConnect: Sized + std::ops::Deref {}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct NEOnDemandRuleDisconnect(pub id);
impl std::ops::Deref for NEOnDemandRuleDisconnect {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for NEOnDemandRuleDisconnect {}
impl NEOnDemandRuleDisconnect {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"NEOnDemandRuleDisconnect").unwrap(), alloc) })
    }
}
impl INEOnDemandRule for NEOnDemandRuleDisconnect {}
impl PNSSecureCoding for NEOnDemandRuleDisconnect {}
impl PNSCopying for NEOnDemandRuleDisconnect {}
impl std::convert::TryFrom<NEOnDemandRule> for NEOnDemandRuleDisconnect {
    type Error = &'static str;
    fn try_from(parent: NEOnDemandRule) -> Result<NEOnDemandRuleDisconnect, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"NEOnDemandRuleDisconnect").unwrap()) };
        if is_kind_of {
            Ok(NEOnDemandRuleDisconnect(parent.0))
        } else {
            Err("This NEOnDemandRule cannot be downcasted to NEOnDemandRuleDisconnect")
        }
    }
}
impl INSObject for NEOnDemandRuleDisconnect {}
impl PNSObject for NEOnDemandRuleDisconnect {}
impl INEOnDemandRuleDisconnect for NEOnDemandRuleDisconnect {}
pub trait INEOnDemandRuleDisconnect: Sized + std::ops::Deref {}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct NEOnDemandRuleIgnore(pub id);
impl std::ops::Deref for NEOnDemandRuleIgnore {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for NEOnDemandRuleIgnore {}
impl NEOnDemandRuleIgnore {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"NEOnDemandRuleIgnore").unwrap(), alloc) })
    }
}
impl INEOnDemandRule for NEOnDemandRuleIgnore {}
impl PNSSecureCoding for NEOnDemandRuleIgnore {}
impl PNSCopying for NEOnDemandRuleIgnore {}
impl std::convert::TryFrom<NEOnDemandRule> for NEOnDemandRuleIgnore {
    type Error = &'static str;
    fn try_from(parent: NEOnDemandRule) -> Result<NEOnDemandRuleIgnore, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"NEOnDemandRuleIgnore").unwrap()) };
        if is_kind_of {
            Ok(NEOnDemandRuleIgnore(parent.0))
        } else {
            Err("This NEOnDemandRule cannot be downcasted to NEOnDemandRuleIgnore")
        }
    }
}
impl INSObject for NEOnDemandRuleIgnore {}
impl PNSObject for NEOnDemandRuleIgnore {}
impl INEOnDemandRuleIgnore for NEOnDemandRuleIgnore {}
pub trait INEOnDemandRuleIgnore: Sized + std::ops::Deref {}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct NEOnDemandRuleEvaluateConnection(pub id);
impl std::ops::Deref for NEOnDemandRuleEvaluateConnection {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for NEOnDemandRuleEvaluateConnection {}
impl NEOnDemandRuleEvaluateConnection {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"NEOnDemandRuleEvaluateConnection").unwrap(), alloc) })
    }
}
impl INEOnDemandRule for NEOnDemandRuleEvaluateConnection {}
impl PNSSecureCoding for NEOnDemandRuleEvaluateConnection {}
impl PNSCopying for NEOnDemandRuleEvaluateConnection {}
impl std::convert::TryFrom<NEOnDemandRule> for NEOnDemandRuleEvaluateConnection {
    type Error = &'static str;
    fn try_from(parent: NEOnDemandRule) -> Result<NEOnDemandRuleEvaluateConnection, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"NEOnDemandRuleEvaluateConnection").unwrap())
        };
        if is_kind_of {
            Ok(NEOnDemandRuleEvaluateConnection(parent.0))
        } else {
            Err("This NEOnDemandRule cannot be downcasted to NEOnDemandRuleEvaluateConnection")
        }
    }
}
impl INSObject for NEOnDemandRuleEvaluateConnection {}
impl PNSObject for NEOnDemandRuleEvaluateConnection {}
impl INEOnDemandRuleEvaluateConnection for NEOnDemandRuleEvaluateConnection {}
pub trait INEOnDemandRuleEvaluateConnection: Sized + std::ops::Deref {
    unsafe fn connectionRules(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, connectionRules)
    }
    unsafe fn setConnectionRules_(&self, connectionRules: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setConnectionRules : connectionRules)
    }
}
pub type NEEvaluateConnectionRuleAction = NSInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct NEEvaluateConnectionRule(pub id);
impl std::ops::Deref for NEEvaluateConnectionRule {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for NEEvaluateConnectionRule {}
impl NEEvaluateConnectionRule {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"NEEvaluateConnectionRule").unwrap(), alloc) })
    }
}
impl PNSSecureCoding for NEEvaluateConnectionRule {}
impl PNSCopying for NEEvaluateConnectionRule {}
impl INSObject for NEEvaluateConnectionRule {}
impl PNSObject for NEEvaluateConnectionRule {}
impl std::convert::TryFrom<NSObject> for NEEvaluateConnectionRule {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<NEEvaluateConnectionRule, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"NEEvaluateConnectionRule").unwrap()) };
        if is_kind_of {
            Ok(NEEvaluateConnectionRule(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to NEEvaluateConnectionRule")
        }
    }
}
impl INEEvaluateConnectionRule for NEEvaluateConnectionRule {}
pub trait INEEvaluateConnectionRule: Sized + std::ops::Deref {
    unsafe fn initWithMatchDomains_andAction_(
        &self,
        domains: NSArray,
        action: NEEvaluateConnectionRuleAction,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithMatchDomains : domains, andAction : action)
    }
    unsafe fn action(&self) -> NEEvaluateConnectionRuleAction
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, action)
    }
    unsafe fn matchDomains(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, matchDomains)
    }
    unsafe fn useDNSServers(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, useDNSServers)
    }
    unsafe fn setUseDNSServers_(&self, useDNSServers: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setUseDNSServers : useDNSServers)
    }
    unsafe fn probeURL(&self) -> NSURL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, probeURL)
    }
    unsafe fn setProbeURL_(&self, probeURL: NSURL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setProbeURL : probeURL)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct NEPacket(pub id);
impl std::ops::Deref for NEPacket {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for NEPacket {}
impl NEPacket {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"NEPacket").unwrap(), alloc) })
    }
}
impl PNSCopying for NEPacket {}
impl PNSSecureCoding for NEPacket {}
impl INSObject for NEPacket {}
impl PNSObject for NEPacket {}
impl std::convert::TryFrom<NSObject> for NEPacket {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<NEPacket, Self::Error> {
        let is_kind_of: bool = unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"NEPacket").unwrap()) };
        if is_kind_of {
            Ok(NEPacket(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to NEPacket")
        }
    }
}
impl INEPacket for NEPacket {}
pub trait INEPacket: Sized + std::ops::Deref {
    unsafe fn initWithData_protocolFamily_(
        &self,
        data: NSData,
        protocolFamily: sa_family_t,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithData : data, protocolFamily : protocolFamily)
    }
    unsafe fn data(&self) -> NSData
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, data)
    }
    unsafe fn protocolFamily(&self) -> sa_family_t
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, protocolFamily)
    }
    unsafe fn direction(&self) -> NETrafficDirection
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, direction)
    }
    unsafe fn metadata(&self) -> NEFlowMetaData
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, metadata)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct NEPacketTunnelFlow(pub id);
impl std::ops::Deref for NEPacketTunnelFlow {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for NEPacketTunnelFlow {}
impl NEPacketTunnelFlow {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"NEPacketTunnelFlow").unwrap(), alloc) })
    }
}
impl INSObject for NEPacketTunnelFlow {}
impl PNSObject for NEPacketTunnelFlow {}
impl std::convert::TryFrom<NSObject> for NEPacketTunnelFlow {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<NEPacketTunnelFlow, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"NEPacketTunnelFlow").unwrap()) };
        if is_kind_of {
            Ok(NEPacketTunnelFlow(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to NEPacketTunnelFlow")
        }
    }
}
impl INEPacketTunnelFlow for NEPacketTunnelFlow {}
pub trait INEPacketTunnelFlow: Sized + std::ops::Deref {
    unsafe fn readPacketsWithCompletionHandler_(
        &self,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, readPacketsWithCompletionHandler : completionHandler)
    }
    unsafe fn writePackets_withProtocols_(&self, packets: NSArray, protocols: NSArray) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, writePackets : packets, withProtocols : protocols)
    }
    unsafe fn readPacketObjectsWithCompletionHandler_(
        &self,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, readPacketObjectsWithCompletionHandler : completionHandler)
    }
    unsafe fn writePacketObjects_(&self, packets: NSArray) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, writePacketObjects : packets)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct NERelay(pub id);
impl std::ops::Deref for NERelay {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for NERelay {}
impl NERelay {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"NERelay").unwrap(), alloc) })
    }
}
impl PNSCopying for NERelay {}
impl PNSSecureCoding for NERelay {}
impl INSObject for NERelay {}
impl PNSObject for NERelay {}
impl std::convert::TryFrom<NSObject> for NERelay {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<NERelay, Self::Error> {
        let is_kind_of: bool = unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"NERelay").unwrap()) };
        if is_kind_of {
            Ok(NERelay(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to NERelay")
        }
    }
}
impl INERelay for NERelay {}
pub trait INERelay: Sized + std::ops::Deref {
    unsafe fn HTTP3RelayURL(&self) -> NSURL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, HTTP3RelayURL)
    }
    unsafe fn setHTTP3RelayURL_(&self, HTTP3RelayURL: NSURL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setHTTP3RelayURL : HTTP3RelayURL)
    }
    unsafe fn HTTP2RelayURL(&self) -> NSURL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, HTTP2RelayURL)
    }
    unsafe fn setHTTP2RelayURL_(&self, HTTP2RelayURL: NSURL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setHTTP2RelayURL : HTTP2RelayURL)
    }
    unsafe fn dnsOverHTTPSURL(&self) -> NSURL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, dnsOverHTTPSURL)
    }
    unsafe fn setDnsOverHTTPSURL_(&self, dnsOverHTTPSURL: NSURL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDnsOverHTTPSURL : dnsOverHTTPSURL)
    }
    unsafe fn syntheticDNSAnswerIPv4Prefix(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, syntheticDNSAnswerIPv4Prefix)
    }
    unsafe fn setSyntheticDNSAnswerIPv4Prefix_(&self, syntheticDNSAnswerIPv4Prefix: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSyntheticDNSAnswerIPv4Prefix : syntheticDNSAnswerIPv4Prefix)
    }
    unsafe fn syntheticDNSAnswerIPv6Prefix(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, syntheticDNSAnswerIPv6Prefix)
    }
    unsafe fn setSyntheticDNSAnswerIPv6Prefix_(&self, syntheticDNSAnswerIPv6Prefix: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSyntheticDNSAnswerIPv6Prefix : syntheticDNSAnswerIPv6Prefix)
    }
    unsafe fn additionalHTTPHeaderFields(&self) -> NSDictionary
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, additionalHTTPHeaderFields)
    }
    unsafe fn setAdditionalHTTPHeaderFields_(&self, additionalHTTPHeaderFields: NSDictionary)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAdditionalHTTPHeaderFields : additionalHTTPHeaderFields)
    }
    unsafe fn rawPublicKeys(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, rawPublicKeys)
    }
    unsafe fn setRawPublicKeys_(&self, rawPublicKeys: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRawPublicKeys : rawPublicKeys)
    }
    unsafe fn identityData(&self) -> NSData
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, identityData)
    }
    unsafe fn setIdentityData_(&self, identityData: NSData)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setIdentityData : identityData)
    }
    unsafe fn identityDataPassword(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, identityDataPassword)
    }
    unsafe fn setIdentityDataPassword_(&self, identityDataPassword: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setIdentityDataPassword : identityDataPassword)
    }
}
pub type NERelayManagerError = NSInteger;
pub type NERelayManagerClientError = NSInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct NERelayManager(pub id);
impl std::ops::Deref for NERelayManager {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for NERelayManager {}
impl NERelayManager {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"NERelayManager").unwrap(), alloc) })
    }
}
impl INSObject for NERelayManager {}
impl PNSObject for NERelayManager {}
impl std::convert::TryFrom<NSObject> for NERelayManager {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<NERelayManager, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"NERelayManager").unwrap()) };
        if is_kind_of {
            Ok(NERelayManager(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to NERelayManager")
        }
    }
}
impl INERelayManager for NERelayManager {}
pub trait INERelayManager: Sized + std::ops::Deref {
    unsafe fn loadFromPreferencesWithCompletionHandler_(
        &self,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, loadFromPreferencesWithCompletionHandler : completionHandler)
    }
    unsafe fn removeFromPreferencesWithCompletionHandler_(
        &self,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, removeFromPreferencesWithCompletionHandler : completionHandler)
    }
    unsafe fn saveToPreferencesWithCompletionHandler_(
        &self,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, saveToPreferencesWithCompletionHandler : completionHandler)
    }
    unsafe fn getLastClientErrors_completionHandler_(
        &self,
        seconds: NSTimeInterval,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, getLastClientErrors : seconds, completionHandler : completionHandler)
    }
    unsafe fn localizedDescription(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, localizedDescription)
    }
    unsafe fn setLocalizedDescription_(&self, localizedDescription: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setLocalizedDescription : localizedDescription)
    }
    unsafe fn isEnabled(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isEnabled)
    }
    unsafe fn setEnabled_(&self, enabled: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setEnabled : enabled)
    }
    unsafe fn isUIToggleEnabled(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isUIToggleEnabled)
    }
    unsafe fn setUIToggleEnabled_(&self, UIToggleEnabled: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setUIToggleEnabled : UIToggleEnabled)
    }
    unsafe fn isDNSFailoverAllowed(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isDNSFailoverAllowed)
    }
    unsafe fn setAllowDNSFailover_(&self, allowDNSFailover: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAllowDNSFailover : allowDNSFailover)
    }
    unsafe fn relays(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, relays)
    }
    unsafe fn setRelays_(&self, relays: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRelays : relays)
    }
    unsafe fn matchDomains(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, matchDomains)
    }
    unsafe fn setMatchDomains_(&self, matchDomains: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMatchDomains : matchDomains)
    }
    unsafe fn matchFQDNs(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, matchFQDNs)
    }
    unsafe fn setMatchFQDNs_(&self, matchFQDNs: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMatchFQDNs : matchFQDNs)
    }
    unsafe fn excludedDomains(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, excludedDomains)
    }
    unsafe fn setExcludedDomains_(&self, excludedDomains: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setExcludedDomains : excludedDomains)
    }
    unsafe fn excludedFQDNs(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, excludedFQDNs)
    }
    unsafe fn setExcludedFQDNs_(&self, excludedFQDNs: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setExcludedFQDNs : excludedFQDNs)
    }
    unsafe fn onDemandRules(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, onDemandRules)
    }
    unsafe fn setOnDemandRules_(&self, onDemandRules: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setOnDemandRules : onDemandRules)
    }
    unsafe fn sharedManager() -> NERelayManager
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NERelayManager").unwrap(), sharedManager)
    }
    unsafe fn loadAllManagersFromPreferencesWithCompletionHandler_(
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NERelayManager").unwrap(), loadAllManagersFromPreferencesWithCompletionHandler : completionHandler)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct NETransparentProxyManager(pub id);
impl std::ops::Deref for NETransparentProxyManager {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for NETransparentProxyManager {}
impl NETransparentProxyManager {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"NETransparentProxyManager").unwrap(), alloc) })
    }
}
impl INEVPNManager for NETransparentProxyManager {}
impl std::convert::TryFrom<NEVPNManager> for NETransparentProxyManager {
    type Error = &'static str;
    fn try_from(parent: NEVPNManager) -> Result<NETransparentProxyManager, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"NETransparentProxyManager").unwrap()) };
        if is_kind_of {
            Ok(NETransparentProxyManager(parent.0))
        } else {
            Err("This NEVPNManager cannot be downcasted to NETransparentProxyManager")
        }
    }
}
impl INSObject for NETransparentProxyManager {}
impl PNSObject for NETransparentProxyManager {}
impl INETransparentProxyManager for NETransparentProxyManager {}
pub trait INETransparentProxyManager: Sized + std::ops::Deref {
    unsafe fn loadAllFromPreferencesWithCompletionHandler_(
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NETransparentProxyManager").unwrap(), loadAllFromPreferencesWithCompletionHandler : completionHandler)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct NETransparentProxyNetworkSettings(pub id);
impl std::ops::Deref for NETransparentProxyNetworkSettings {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for NETransparentProxyNetworkSettings {}
impl NETransparentProxyNetworkSettings {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"NETransparentProxyNetworkSettings").unwrap(), alloc) })
    }
}
impl INETunnelNetworkSettings for NETransparentProxyNetworkSettings {}
impl PNSSecureCoding for NETransparentProxyNetworkSettings {}
impl PNSCopying for NETransparentProxyNetworkSettings {}
impl std::convert::TryFrom<NETunnelNetworkSettings> for NETransparentProxyNetworkSettings {
    type Error = &'static str;
    fn try_from(
        parent: NETunnelNetworkSettings,
    ) -> Result<NETransparentProxyNetworkSettings, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"NETransparentProxyNetworkSettings").unwrap())
        };
        if is_kind_of {
            Ok(NETransparentProxyNetworkSettings(parent.0))
        } else {
            Err ("This NETunnelNetworkSettings cannot be downcasted to NETransparentProxyNetworkSettings" ,)
        }
    }
}
impl INSObject for NETransparentProxyNetworkSettings {}
impl PNSObject for NETransparentProxyNetworkSettings {}
impl INETransparentProxyNetworkSettings for NETransparentProxyNetworkSettings {}
pub trait INETransparentProxyNetworkSettings: Sized + std::ops::Deref {
    unsafe fn includedNetworkRules(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, includedNetworkRules)
    }
    unsafe fn setIncludedNetworkRules_(&self, includedNetworkRules: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setIncludedNetworkRules : includedNetworkRules)
    }
    unsafe fn excludedNetworkRules(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, excludedNetworkRules)
    }
    unsafe fn setExcludedNetworkRules_(&self, excludedNetworkRules: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setExcludedNetworkRules : excludedNetworkRules)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct NETransparentProxyProvider(pub id);
impl std::ops::Deref for NETransparentProxyProvider {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for NETransparentProxyProvider {}
impl NETransparentProxyProvider {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"NETransparentProxyProvider").unwrap(), alloc) })
    }
}
impl INEAppProxyProvider for NETransparentProxyProvider {}
impl From<NETransparentProxyProvider> for NEAppProxyProvider {
    fn from(child: NETransparentProxyProvider) -> NEAppProxyProvider {
        NEAppProxyProvider(child.0)
    }
}
impl std::convert::TryFrom<NEAppProxyProvider> for NETransparentProxyProvider {
    type Error = &'static str;
    fn try_from(parent: NEAppProxyProvider) -> Result<NETransparentProxyProvider, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"NETransparentProxyProvider").unwrap()) };
        if is_kind_of {
            Ok(NETransparentProxyProvider(parent.0))
        } else {
            Err("This NEAppProxyProvider cannot be downcasted to NETransparentProxyProvider")
        }
    }
}
impl INETunnelProvider for NETransparentProxyProvider {}
impl INEProvider for NETransparentProxyProvider {}
impl INSObject for NETransparentProxyProvider {}
impl PNSObject for NETransparentProxyProvider {}
impl INETransparentProxyProvider for NETransparentProxyProvider {}
pub trait INETransparentProxyProvider: Sized + std::ops::Deref {}
pub type NEVPNStatus = NSInteger;
pub type NEVPNConnectionError = NSInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct NEVPNConnection(pub id);
impl std::ops::Deref for NEVPNConnection {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for NEVPNConnection {}
impl NEVPNConnection {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"NEVPNConnection").unwrap(), alloc) })
    }
}
impl INSObject for NEVPNConnection {}
impl PNSObject for NEVPNConnection {}
impl std::convert::TryFrom<NSObject> for NEVPNConnection {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<NEVPNConnection, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"NEVPNConnection").unwrap()) };
        if is_kind_of {
            Ok(NEVPNConnection(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to NEVPNConnection")
        }
    }
}
impl INEVPNConnection for NEVPNConnection {}
pub trait INEVPNConnection: Sized + std::ops::Deref {
    unsafe fn startVPNTunnelAndReturnError_(&self, error: *mut NSError) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, startVPNTunnelAndReturnError : error)
    }
    unsafe fn startVPNTunnelWithOptions_andReturnError_(
        &self,
        options: NSDictionary,
        error: *mut NSError,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, startVPNTunnelWithOptions : options, andReturnError : error)
    }
    unsafe fn stopVPNTunnel(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, stopVPNTunnel)
    }
    unsafe fn fetchLastDisconnectErrorWithCompletionHandler_(
        &self,
        handler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, fetchLastDisconnectErrorWithCompletionHandler : handler)
    }
    unsafe fn status(&self) -> NEVPNStatus
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, status)
    }
    unsafe fn connectedDate(&self) -> NSDate
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, connectedDate)
    }
    unsafe fn manager(&self) -> NEVPNManager
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, manager)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct NETunnelProviderSession(pub id);
impl std::ops::Deref for NETunnelProviderSession {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for NETunnelProviderSession {}
impl NETunnelProviderSession {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"NETunnelProviderSession").unwrap(), alloc) })
    }
}
impl INEVPNConnection for NETunnelProviderSession {}
impl From<NETunnelProviderSession> for NEVPNConnection {
    fn from(child: NETunnelProviderSession) -> NEVPNConnection {
        NEVPNConnection(child.0)
    }
}
impl std::convert::TryFrom<NEVPNConnection> for NETunnelProviderSession {
    type Error = &'static str;
    fn try_from(parent: NEVPNConnection) -> Result<NETunnelProviderSession, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"NETunnelProviderSession").unwrap()) };
        if is_kind_of {
            Ok(NETunnelProviderSession(parent.0))
        } else {
            Err("This NEVPNConnection cannot be downcasted to NETunnelProviderSession")
        }
    }
}
impl INSObject for NETunnelProviderSession {}
impl PNSObject for NETunnelProviderSession {}
impl INETunnelProviderSession for NETunnelProviderSession {}
pub trait INETunnelProviderSession: Sized + std::ops::Deref {
    unsafe fn startTunnelWithOptions_andReturnError_(
        &self,
        options: NSDictionary,
        error: *mut NSError,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, startTunnelWithOptions : options, andReturnError : error)
    }
    unsafe fn stopTunnel(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, stopTunnel)
    }
    unsafe fn sendProviderMessage_returnError_responseHandler_(
        &self,
        messageData: NSData,
        error: *mut NSError,
        responseHandler: *mut ::std::os::raw::c_void,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, sendProviderMessage : messageData, returnError : error, responseHandler : responseHandler)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct NETunnelProviderProtocol(pub id);
impl std::ops::Deref for NETunnelProviderProtocol {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for NETunnelProviderProtocol {}
impl NETunnelProviderProtocol {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"NETunnelProviderProtocol").unwrap(), alloc) })
    }
}
impl INEVPNProtocol for NETunnelProviderProtocol {}
impl PNSCopying for NETunnelProviderProtocol {}
impl PNSSecureCoding for NETunnelProviderProtocol {}
impl std::convert::TryFrom<NEVPNProtocol> for NETunnelProviderProtocol {
    type Error = &'static str;
    fn try_from(parent: NEVPNProtocol) -> Result<NETunnelProviderProtocol, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"NETunnelProviderProtocol").unwrap()) };
        if is_kind_of {
            Ok(NETunnelProviderProtocol(parent.0))
        } else {
            Err("This NEVPNProtocol cannot be downcasted to NETunnelProviderProtocol")
        }
    }
}
impl INSObject for NETunnelProviderProtocol {}
impl PNSObject for NETunnelProviderProtocol {}
impl INETunnelProviderProtocol for NETunnelProviderProtocol {}
pub trait INETunnelProviderProtocol: Sized + std::ops::Deref {
    unsafe fn providerConfiguration(&self) -> NSDictionary
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, providerConfiguration)
    }
    unsafe fn setProviderConfiguration_(&self, providerConfiguration: NSDictionary)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setProviderConfiguration : providerConfiguration)
    }
    unsafe fn providerBundleIdentifier(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, providerBundleIdentifier)
    }
    unsafe fn setProviderBundleIdentifier_(&self, providerBundleIdentifier: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setProviderBundleIdentifier : providerBundleIdentifier)
    }
}
pub type NEVPNIKEAuthenticationMethod = NSInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct NEVPNProtocolIPSec(pub id);
impl std::ops::Deref for NEVPNProtocolIPSec {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for NEVPNProtocolIPSec {}
impl NEVPNProtocolIPSec {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"NEVPNProtocolIPSec").unwrap(), alloc) })
    }
}
impl INEVPNProtocol for NEVPNProtocolIPSec {}
impl PNSCopying for NEVPNProtocolIPSec {}
impl PNSSecureCoding for NEVPNProtocolIPSec {}
impl std::convert::TryFrom<NEVPNProtocol> for NEVPNProtocolIPSec {
    type Error = &'static str;
    fn try_from(parent: NEVPNProtocol) -> Result<NEVPNProtocolIPSec, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"NEVPNProtocolIPSec").unwrap()) };
        if is_kind_of {
            Ok(NEVPNProtocolIPSec(parent.0))
        } else {
            Err("This NEVPNProtocol cannot be downcasted to NEVPNProtocolIPSec")
        }
    }
}
impl INSObject for NEVPNProtocolIPSec {}
impl PNSObject for NEVPNProtocolIPSec {}
impl INEVPNProtocolIPSec for NEVPNProtocolIPSec {}
pub trait INEVPNProtocolIPSec: Sized + std::ops::Deref {
    unsafe fn authenticationMethod(&self) -> NEVPNIKEAuthenticationMethod
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, authenticationMethod)
    }
    unsafe fn setAuthenticationMethod_(&self, authenticationMethod: NEVPNIKEAuthenticationMethod)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAuthenticationMethod : authenticationMethod)
    }
    unsafe fn useExtendedAuthentication(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, useExtendedAuthentication)
    }
    unsafe fn setUseExtendedAuthentication_(&self, useExtendedAuthentication: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setUseExtendedAuthentication : useExtendedAuthentication)
    }
    unsafe fn sharedSecretReference(&self) -> NSData
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, sharedSecretReference)
    }
    unsafe fn setSharedSecretReference_(&self, sharedSecretReference: NSData)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSharedSecretReference : sharedSecretReference)
    }
    unsafe fn localIdentifier(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, localIdentifier)
    }
    unsafe fn setLocalIdentifier_(&self, localIdentifier: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setLocalIdentifier : localIdentifier)
    }
    unsafe fn remoteIdentifier(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, remoteIdentifier)
    }
    unsafe fn setRemoteIdentifier_(&self, remoteIdentifier: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRemoteIdentifier : remoteIdentifier)
    }
}
pub type NEVPNIKEv2EncryptionAlgorithm = NSInteger;
pub type NEVPNIKEv2IntegrityAlgorithm = NSInteger;
pub type NEVPNIKEv2DeadPeerDetectionRate = NSInteger;
pub type NEVPNIKEv2DiffieHellmanGroup = NSInteger;
pub type NEVPNIKEv2PostQuantumKeyExchangeMethod = NSInteger;
pub type NEVPNIKEv2CertificateType = NSInteger;
pub type NEVPNIKEv2TLSVersion = NSInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct NEVPNIKEv2SecurityAssociationParameters(pub id);
impl std::ops::Deref for NEVPNIKEv2SecurityAssociationParameters {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for NEVPNIKEv2SecurityAssociationParameters {}
impl NEVPNIKEv2SecurityAssociationParameters {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"NEVPNIKEv2SecurityAssociationParameters").unwrap(), alloc) })
    }
}
impl PNSSecureCoding for NEVPNIKEv2SecurityAssociationParameters {}
impl PNSCopying for NEVPNIKEv2SecurityAssociationParameters {}
impl INSObject for NEVPNIKEv2SecurityAssociationParameters {}
impl PNSObject for NEVPNIKEv2SecurityAssociationParameters {}
impl std::convert::TryFrom<NSObject> for NEVPNIKEv2SecurityAssociationParameters {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<NEVPNIKEv2SecurityAssociationParameters, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"NEVPNIKEv2SecurityAssociationParameters").unwrap())
        };
        if is_kind_of {
            Ok(NEVPNIKEv2SecurityAssociationParameters(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to NEVPNIKEv2SecurityAssociationParameters")
        }
    }
}
impl INEVPNIKEv2SecurityAssociationParameters for NEVPNIKEv2SecurityAssociationParameters {}
pub trait INEVPNIKEv2SecurityAssociationParameters: Sized + std::ops::Deref {
    unsafe fn encryptionAlgorithm(&self) -> NEVPNIKEv2EncryptionAlgorithm
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, encryptionAlgorithm)
    }
    unsafe fn setEncryptionAlgorithm_(&self, encryptionAlgorithm: NEVPNIKEv2EncryptionAlgorithm)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setEncryptionAlgorithm : encryptionAlgorithm)
    }
    unsafe fn integrityAlgorithm(&self) -> NEVPNIKEv2IntegrityAlgorithm
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, integrityAlgorithm)
    }
    unsafe fn setIntegrityAlgorithm_(&self, integrityAlgorithm: NEVPNIKEv2IntegrityAlgorithm)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setIntegrityAlgorithm : integrityAlgorithm)
    }
    unsafe fn diffieHellmanGroup(&self) -> NEVPNIKEv2DiffieHellmanGroup
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, diffieHellmanGroup)
    }
    unsafe fn setDiffieHellmanGroup_(&self, diffieHellmanGroup: NEVPNIKEv2DiffieHellmanGroup)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDiffieHellmanGroup : diffieHellmanGroup)
    }
    unsafe fn postQuantumKeyExchangeMethods(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, postQuantumKeyExchangeMethods)
    }
    unsafe fn setPostQuantumKeyExchangeMethods_(&self, postQuantumKeyExchangeMethods: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPostQuantumKeyExchangeMethods : postQuantumKeyExchangeMethods)
    }
    unsafe fn lifetimeMinutes(&self) -> i32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, lifetimeMinutes)
    }
    unsafe fn setLifetimeMinutes_(&self, lifetimeMinutes: i32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setLifetimeMinutes : lifetimeMinutes)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct NEVPNIKEv2PPKConfiguration(pub id);
impl std::ops::Deref for NEVPNIKEv2PPKConfiguration {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for NEVPNIKEv2PPKConfiguration {}
impl NEVPNIKEv2PPKConfiguration {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"NEVPNIKEv2PPKConfiguration").unwrap(), alloc) })
    }
}
impl PNSCopying for NEVPNIKEv2PPKConfiguration {}
impl INSObject for NEVPNIKEv2PPKConfiguration {}
impl PNSObject for NEVPNIKEv2PPKConfiguration {}
impl std::convert::TryFrom<NSObject> for NEVPNIKEv2PPKConfiguration {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<NEVPNIKEv2PPKConfiguration, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"NEVPNIKEv2PPKConfiguration").unwrap()) };
        if is_kind_of {
            Ok(NEVPNIKEv2PPKConfiguration(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to NEVPNIKEv2PPKConfiguration")
        }
    }
}
impl INEVPNIKEv2PPKConfiguration for NEVPNIKEv2PPKConfiguration {}
pub trait INEVPNIKEv2PPKConfiguration: Sized + std::ops::Deref {
    unsafe fn initWithIdentifier_keychainReference_(
        &self,
        identifier: NSString,
        keychainReference: NSData,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithIdentifier : identifier, keychainReference : keychainReference)
    }
    unsafe fn identifier(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, identifier)
    }
    unsafe fn keychainReference(&self) -> NSData
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, keychainReference)
    }
    unsafe fn isMandatory(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isMandatory)
    }
    unsafe fn setIsMandatory_(&self, isMandatory: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setIsMandatory : isMandatory)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct NEVPNProtocolIKEv2(pub id);
impl std::ops::Deref for NEVPNProtocolIKEv2 {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for NEVPNProtocolIKEv2 {}
impl NEVPNProtocolIKEv2 {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"NEVPNProtocolIKEv2").unwrap(), alloc) })
    }
}
impl INEVPNProtocolIPSec for NEVPNProtocolIKEv2 {}
impl From<NEVPNProtocolIKEv2> for NEVPNProtocolIPSec {
    fn from(child: NEVPNProtocolIKEv2) -> NEVPNProtocolIPSec {
        NEVPNProtocolIPSec(child.0)
    }
}
impl std::convert::TryFrom<NEVPNProtocolIPSec> for NEVPNProtocolIKEv2 {
    type Error = &'static str;
    fn try_from(parent: NEVPNProtocolIPSec) -> Result<NEVPNProtocolIKEv2, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"NEVPNProtocolIKEv2").unwrap()) };
        if is_kind_of {
            Ok(NEVPNProtocolIKEv2(parent.0))
        } else {
            Err("This NEVPNProtocolIPSec cannot be downcasted to NEVPNProtocolIKEv2")
        }
    }
}
impl INEVPNProtocol for NEVPNProtocolIKEv2 {}
impl PNSCopying for NEVPNProtocolIKEv2 {}
impl PNSSecureCoding for NEVPNProtocolIKEv2 {}
impl INSObject for NEVPNProtocolIKEv2 {}
impl PNSObject for NEVPNProtocolIKEv2 {}
impl INEVPNProtocolIKEv2 for NEVPNProtocolIKEv2 {}
pub trait INEVPNProtocolIKEv2: Sized + std::ops::Deref {
    unsafe fn deadPeerDetectionRate(&self) -> NEVPNIKEv2DeadPeerDetectionRate
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, deadPeerDetectionRate)
    }
    unsafe fn setDeadPeerDetectionRate_(
        &self,
        deadPeerDetectionRate: NEVPNIKEv2DeadPeerDetectionRate,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDeadPeerDetectionRate : deadPeerDetectionRate)
    }
    unsafe fn serverCertificateIssuerCommonName(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, serverCertificateIssuerCommonName)
    }
    unsafe fn setServerCertificateIssuerCommonName_(
        &self,
        serverCertificateIssuerCommonName: NSString,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setServerCertificateIssuerCommonName : serverCertificateIssuerCommonName)
    }
    unsafe fn serverCertificateCommonName(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, serverCertificateCommonName)
    }
    unsafe fn setServerCertificateCommonName_(&self, serverCertificateCommonName: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setServerCertificateCommonName : serverCertificateCommonName)
    }
    unsafe fn certificateType(&self) -> NEVPNIKEv2CertificateType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, certificateType)
    }
    unsafe fn setCertificateType_(&self, certificateType: NEVPNIKEv2CertificateType)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCertificateType : certificateType)
    }
    unsafe fn useConfigurationAttributeInternalIPSubnet(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, useConfigurationAttributeInternalIPSubnet)
    }
    unsafe fn setUseConfigurationAttributeInternalIPSubnet_(
        &self,
        useConfigurationAttributeInternalIPSubnet: BOOL,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setUseConfigurationAttributeInternalIPSubnet : useConfigurationAttributeInternalIPSubnet)
    }
    unsafe fn IKESecurityAssociationParameters(&self) -> NEVPNIKEv2SecurityAssociationParameters
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, IKESecurityAssociationParameters)
    }
    unsafe fn childSecurityAssociationParameters(&self) -> NEVPNIKEv2SecurityAssociationParameters
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, childSecurityAssociationParameters)
    }
    unsafe fn disableMOBIKE(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, disableMOBIKE)
    }
    unsafe fn setDisableMOBIKE_(&self, disableMOBIKE: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDisableMOBIKE : disableMOBIKE)
    }
    unsafe fn disableRedirect(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, disableRedirect)
    }
    unsafe fn setDisableRedirect_(&self, disableRedirect: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDisableRedirect : disableRedirect)
    }
    unsafe fn enablePFS(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, enablePFS)
    }
    unsafe fn setEnablePFS_(&self, enablePFS: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setEnablePFS : enablePFS)
    }
    unsafe fn allowPostQuantumKeyExchangeFallback(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, allowPostQuantumKeyExchangeFallback)
    }
    unsafe fn setAllowPostQuantumKeyExchangeFallback_(
        &self,
        allowPostQuantumKeyExchangeFallback: BOOL,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAllowPostQuantumKeyExchangeFallback : allowPostQuantumKeyExchangeFallback)
    }
    unsafe fn enableRevocationCheck(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, enableRevocationCheck)
    }
    unsafe fn setEnableRevocationCheck_(&self, enableRevocationCheck: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setEnableRevocationCheck : enableRevocationCheck)
    }
    unsafe fn strictRevocationCheck(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, strictRevocationCheck)
    }
    unsafe fn setStrictRevocationCheck_(&self, strictRevocationCheck: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setStrictRevocationCheck : strictRevocationCheck)
    }
    unsafe fn minimumTLSVersion(&self) -> NEVPNIKEv2TLSVersion
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, minimumTLSVersion)
    }
    unsafe fn setMinimumTLSVersion_(&self, minimumTLSVersion: NEVPNIKEv2TLSVersion)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMinimumTLSVersion : minimumTLSVersion)
    }
    unsafe fn maximumTLSVersion(&self) -> NEVPNIKEv2TLSVersion
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, maximumTLSVersion)
    }
    unsafe fn setMaximumTLSVersion_(&self, maximumTLSVersion: NEVPNIKEv2TLSVersion)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMaximumTLSVersion : maximumTLSVersion)
    }
    unsafe fn enableFallback(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, enableFallback)
    }
    unsafe fn setEnableFallback_(&self, enableFallback: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setEnableFallback : enableFallback)
    }
    unsafe fn mtu(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, mtu)
    }
    unsafe fn setMtu_(&self, mtu: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMtu : mtu)
    }
    unsafe fn ppkConfiguration(&self) -> NEVPNIKEv2PPKConfiguration
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, ppkConfiguration)
    }
    unsafe fn setPpkConfiguration_(&self, ppkConfiguration: NEVPNIKEv2PPKConfiguration)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPpkConfiguration : ppkConfiguration)
    }
}
pub type NEAppPushManagerError = NSInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct NEPrivateLTENetwork(pub id);
impl std::ops::Deref for NEPrivateLTENetwork {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for NEPrivateLTENetwork {}
impl NEPrivateLTENetwork {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"NEPrivateLTENetwork").unwrap(), alloc) })
    }
}
impl PNSCopying for NEPrivateLTENetwork {}
impl PNSSecureCoding for NEPrivateLTENetwork {}
impl INSObject for NEPrivateLTENetwork {}
impl PNSObject for NEPrivateLTENetwork {}
impl std::convert::TryFrom<NSObject> for NEPrivateLTENetwork {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<NEPrivateLTENetwork, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"NEPrivateLTENetwork").unwrap()) };
        if is_kind_of {
            Ok(NEPrivateLTENetwork(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to NEPrivateLTENetwork")
        }
    }
}
impl INEPrivateLTENetwork for NEPrivateLTENetwork {}
pub trait INEPrivateLTENetwork: Sized + std::ops::Deref {
    unsafe fn mobileCountryCode(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, mobileCountryCode)
    }
    unsafe fn setMobileCountryCode_(&self, mobileCountryCode: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMobileCountryCode : mobileCountryCode)
    }
    unsafe fn mobileNetworkCode(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, mobileNetworkCode)
    }
    unsafe fn setMobileNetworkCode_(&self, mobileNetworkCode: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMobileNetworkCode : mobileNetworkCode)
    }
    unsafe fn trackingAreaCode(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, trackingAreaCode)
    }
    unsafe fn setTrackingAreaCode_(&self, trackingAreaCode: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setTrackingAreaCode : trackingAreaCode)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct NEAppPushManager(pub id);
impl std::ops::Deref for NEAppPushManager {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for NEAppPushManager {}
impl NEAppPushManager {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"NEAppPushManager").unwrap(), alloc) })
    }
}
impl INSObject for NEAppPushManager {}
impl PNSObject for NEAppPushManager {}
impl std::convert::TryFrom<NSObject> for NEAppPushManager {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<NEAppPushManager, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"NEAppPushManager").unwrap()) };
        if is_kind_of {
            Ok(NEAppPushManager(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to NEAppPushManager")
        }
    }
}
impl INEAppPushManager for NEAppPushManager {}
pub trait INEAppPushManager: Sized + std::ops::Deref {
    unsafe fn loadFromPreferencesWithCompletionHandler_(
        &self,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, loadFromPreferencesWithCompletionHandler : completionHandler)
    }
    unsafe fn removeFromPreferencesWithCompletionHandler_(
        &self,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, removeFromPreferencesWithCompletionHandler : completionHandler)
    }
    unsafe fn saveToPreferencesWithCompletionHandler_(
        &self,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, saveToPreferencesWithCompletionHandler : completionHandler)
    }
    unsafe fn matchSSIDs(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, matchSSIDs)
    }
    unsafe fn setMatchSSIDs_(&self, matchSSIDs: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMatchSSIDs : matchSSIDs)
    }
    unsafe fn matchPrivateLTENetworks(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, matchPrivateLTENetworks)
    }
    unsafe fn setMatchPrivateLTENetworks_(&self, matchPrivateLTENetworks: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMatchPrivateLTENetworks : matchPrivateLTENetworks)
    }
    unsafe fn matchEthernet(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, matchEthernet)
    }
    unsafe fn setMatchEthernet_(&self, matchEthernet: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMatchEthernet : matchEthernet)
    }
    unsafe fn providerConfiguration(&self) -> NSDictionary
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, providerConfiguration)
    }
    unsafe fn setProviderConfiguration_(&self, providerConfiguration: NSDictionary)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setProviderConfiguration : providerConfiguration)
    }
    unsafe fn providerBundleIdentifier(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, providerBundleIdentifier)
    }
    unsafe fn setProviderBundleIdentifier_(&self, providerBundleIdentifier: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setProviderBundleIdentifier : providerBundleIdentifier)
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
    unsafe fn localizedDescription(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, localizedDescription)
    }
    unsafe fn setLocalizedDescription_(&self, localizedDescription: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setLocalizedDescription : localizedDescription)
    }
    unsafe fn isEnabled(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isEnabled)
    }
    unsafe fn setEnabled_(&self, enabled: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setEnabled : enabled)
    }
    unsafe fn isActive(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isActive)
    }
    unsafe fn loadAllFromPreferencesWithCompletionHandler_(
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NEAppPushManager").unwrap(), loadAllFromPreferencesWithCompletionHandler : completionHandler)
    }
}
pub trait PNEAppPushDelegate: Sized + std::ops::Deref {
    unsafe fn appPushManager_didReceiveIncomingCallWithUserInfo_(
        &self,
        manager: NEAppPushManager,
        userInfo: NSDictionary,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, appPushManager : manager, didReceiveIncomingCallWithUserInfo : userInfo)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct NEAppPushProvider(pub id);
impl std::ops::Deref for NEAppPushProvider {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for NEAppPushProvider {}
impl NEAppPushProvider {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"NEAppPushProvider").unwrap(), alloc) })
    }
}
impl INEProvider for NEAppPushProvider {}
impl std::convert::TryFrom<NEProvider> for NEAppPushProvider {
    type Error = &'static str;
    fn try_from(parent: NEProvider) -> Result<NEAppPushProvider, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"NEAppPushProvider").unwrap()) };
        if is_kind_of {
            Ok(NEAppPushProvider(parent.0))
        } else {
            Err("This NEProvider cannot be downcasted to NEAppPushProvider")
        }
    }
}
impl INSObject for NEAppPushProvider {}
impl PNSObject for NEAppPushProvider {}
impl INEAppPushProvider for NEAppPushProvider {}
pub trait INEAppPushProvider: Sized + std::ops::Deref {
    unsafe fn startWithCompletionHandler_(&self, completionHandler: *mut ::std::os::raw::c_void)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, startWithCompletionHandler : completionHandler)
    }
    unsafe fn start(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, start)
    }
    unsafe fn stopWithReason_completionHandler_(
        &self,
        reason: NEProviderStopReason,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, stopWithReason : reason, completionHandler : completionHandler)
    }
    unsafe fn reportIncomingCallWithUserInfo_(&self, userInfo: NSDictionary)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, reportIncomingCallWithUserInfo : userInfo)
    }
    unsafe fn reportPushToTalkMessageWithUserInfo_(&self, userInfo: NSDictionary)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, reportPushToTalkMessageWithUserInfo : userInfo)
    }
    unsafe fn handleTimerEvent(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, handleTimerEvent)
    }
    unsafe fn unmatchEthernet(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, unmatchEthernet)
    }
    unsafe fn providerConfiguration(&self) -> NSDictionary
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, providerConfiguration)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct NWEndpoint(pub id);
impl std::ops::Deref for NWEndpoint {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for NWEndpoint {}
impl NWEndpoint {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"NWEndpoint").unwrap(), alloc) })
    }
}
impl PNSSecureCoding for NWEndpoint {}
impl PNSCopying for NWEndpoint {}
impl INSObject for NWEndpoint {}
impl PNSObject for NWEndpoint {}
impl std::convert::TryFrom<NSObject> for NWEndpoint {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<NWEndpoint, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"NWEndpoint").unwrap()) };
        if is_kind_of {
            Ok(NWEndpoint(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to NWEndpoint")
        }
    }
}
impl INWEndpoint for NWEndpoint {}
pub trait INWEndpoint: Sized + std::ops::Deref {}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct NWHostEndpoint(pub id);
impl std::ops::Deref for NWHostEndpoint {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for NWHostEndpoint {}
impl NWHostEndpoint {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"NWHostEndpoint").unwrap(), alloc) })
    }
}
impl INWEndpoint for NWHostEndpoint {}
impl PNSSecureCoding for NWHostEndpoint {}
impl PNSCopying for NWHostEndpoint {}
impl From<NWHostEndpoint> for NWEndpoint {
    fn from(child: NWHostEndpoint) -> NWEndpoint {
        NWEndpoint(child.0)
    }
}
impl std::convert::TryFrom<NWEndpoint> for NWHostEndpoint {
    type Error = &'static str;
    fn try_from(parent: NWEndpoint) -> Result<NWHostEndpoint, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"NWHostEndpoint").unwrap()) };
        if is_kind_of {
            Ok(NWHostEndpoint(parent.0))
        } else {
            Err("This NWEndpoint cannot be downcasted to NWHostEndpoint")
        }
    }
}
impl INSObject for NWHostEndpoint {}
impl PNSObject for NWHostEndpoint {}
impl INWHostEndpoint for NWHostEndpoint {}
pub trait INWHostEndpoint: Sized + std::ops::Deref {
    unsafe fn hostname(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, hostname)
    }
    unsafe fn port(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, port)
    }
    unsafe fn endpointWithHostname_port_(hostname: NSString, port: NSString) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NWHostEndpoint").unwrap(), endpointWithHostname : hostname, port : port)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct NWBonjourServiceEndpoint(pub id);
impl std::ops::Deref for NWBonjourServiceEndpoint {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for NWBonjourServiceEndpoint {}
impl NWBonjourServiceEndpoint {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"NWBonjourServiceEndpoint").unwrap(), alloc) })
    }
}
impl INWEndpoint for NWBonjourServiceEndpoint {}
impl PNSSecureCoding for NWBonjourServiceEndpoint {}
impl PNSCopying for NWBonjourServiceEndpoint {}
impl std::convert::TryFrom<NWEndpoint> for NWBonjourServiceEndpoint {
    type Error = &'static str;
    fn try_from(parent: NWEndpoint) -> Result<NWBonjourServiceEndpoint, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"NWBonjourServiceEndpoint").unwrap()) };
        if is_kind_of {
            Ok(NWBonjourServiceEndpoint(parent.0))
        } else {
            Err("This NWEndpoint cannot be downcasted to NWBonjourServiceEndpoint")
        }
    }
}
impl INSObject for NWBonjourServiceEndpoint {}
impl PNSObject for NWBonjourServiceEndpoint {}
impl INWBonjourServiceEndpoint for NWBonjourServiceEndpoint {}
pub trait INWBonjourServiceEndpoint: Sized + std::ops::Deref {
    unsafe fn name(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, name)
    }
    unsafe fn type_(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, type)
    }
    unsafe fn domain(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, domain)
    }
    unsafe fn endpointWithName_type_domain_(
        name: NSString,
        type_: NSString,
        domain: NSString,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NWBonjourServiceEndpoint").unwrap(), endpointWithName : name, r#type : type_, domain : domain)
    }
}
pub type NWPathStatus = NSInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct NWPath(pub id);
impl std::ops::Deref for NWPath {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for NWPath {}
impl NWPath {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"NWPath").unwrap(), alloc) })
    }
}
impl INSObject for NWPath {}
impl PNSObject for NWPath {}
impl std::convert::TryFrom<NSObject> for NWPath {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<NWPath, Self::Error> {
        let is_kind_of: bool = unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"NWPath").unwrap()) };
        if is_kind_of {
            Ok(NWPath(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to NWPath")
        }
    }
}
impl INWPath for NWPath {}
pub trait INWPath: Sized + std::ops::Deref {
    unsafe fn isEqualToPath_(&self, path: NWPath) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, isEqualToPath : path)
    }
    unsafe fn status(&self) -> NWPathStatus
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, status)
    }
    unsafe fn isExpensive(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isExpensive)
    }
    unsafe fn isConstrained(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isConstrained)
    }
}
pub type NWTCPConnectionState = NSInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct NWTCPConnection(pub id);
impl std::ops::Deref for NWTCPConnection {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for NWTCPConnection {}
impl NWTCPConnection {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"NWTCPConnection").unwrap(), alloc) })
    }
}
impl INSObject for NWTCPConnection {}
impl PNSObject for NWTCPConnection {}
impl std::convert::TryFrom<NSObject> for NWTCPConnection {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<NWTCPConnection, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"NWTCPConnection").unwrap()) };
        if is_kind_of {
            Ok(NWTCPConnection(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to NWTCPConnection")
        }
    }
}
impl INWTCPConnection for NWTCPConnection {}
pub trait INWTCPConnection: Sized + std::ops::Deref {
    unsafe fn initWithUpgradeForConnection_(&self, connection: NWTCPConnection) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithUpgradeForConnection : connection)
    }
    unsafe fn cancel(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, cancel)
    }
    unsafe fn readLength_completionHandler_(
        &self,
        length: NSUInteger,
        completion: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, readLength : length, completionHandler : completion)
    }
    unsafe fn readMinimumLength_maximumLength_completionHandler_(
        &self,
        minimum: NSUInteger,
        maximum: NSUInteger,
        completion: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, readMinimumLength : minimum, maximumLength : maximum, completionHandler : completion)
    }
    unsafe fn write_completionHandler_(&self, data: NSData, completion: *mut ::std::os::raw::c_void)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, write : data, completionHandler : completion)
    }
    unsafe fn writeClose(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, writeClose)
    }
    unsafe fn state(&self) -> NWTCPConnectionState
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, state)
    }
    unsafe fn isViable(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isViable)
    }
    unsafe fn hasBetterPath(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, hasBetterPath)
    }
    unsafe fn endpoint(&self) -> NWEndpoint
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, endpoint)
    }
    unsafe fn connectedPath(&self) -> NWPath
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, connectedPath)
    }
    unsafe fn localAddress(&self) -> NWEndpoint
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, localAddress)
    }
    unsafe fn remoteAddress(&self) -> NWEndpoint
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, remoteAddress)
    }
    unsafe fn txtRecord(&self) -> NSData
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, txtRecord)
    }
    unsafe fn error(&self) -> NSError
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, error)
    }
}
pub trait PNWTCPConnectionAuthenticationDelegate: Sized + std::ops::Deref {
    unsafe fn shouldProvideIdentityForConnection_(&self, connection: NWTCPConnection) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, shouldProvideIdentityForConnection : connection)
    }
    unsafe fn provideIdentityForConnection_completionHandler_(
        &self,
        connection: NWTCPConnection,
        completion: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, provideIdentityForConnection : connection, completionHandler : completion)
    }
    unsafe fn shouldEvaluateTrustForConnection_(&self, connection: NWTCPConnection) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, shouldEvaluateTrustForConnection : connection)
    }
    unsafe fn evaluateTrustForConnection_peerCertificateChain_completionHandler_(
        &self,
        connection: NWTCPConnection,
        peerCertificateChain: NSArray,
        completion: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, evaluateTrustForConnection : connection, peerCertificateChain : peerCertificateChain, completionHandler : completion)
    }
}
pub type NWUDPSessionState = NSInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct NWUDPSession(pub id);
impl std::ops::Deref for NWUDPSession {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for NWUDPSession {}
impl NWUDPSession {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"NWUDPSession").unwrap(), alloc) })
    }
}
impl INSObject for NWUDPSession {}
impl PNSObject for NWUDPSession {}
impl std::convert::TryFrom<NSObject> for NWUDPSession {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<NWUDPSession, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"NWUDPSession").unwrap()) };
        if is_kind_of {
            Ok(NWUDPSession(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to NWUDPSession")
        }
    }
}
impl INWUDPSession for NWUDPSession {}
pub trait INWUDPSession: Sized + std::ops::Deref {
    unsafe fn initWithUpgradeForSession_(&self, session: NWUDPSession) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithUpgradeForSession : session)
    }
    unsafe fn tryNextResolvedEndpoint(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, tryNextResolvedEndpoint)
    }
    unsafe fn setReadHandler_maxDatagrams_(
        &self,
        handler: *mut ::std::os::raw::c_void,
        maxDatagrams: NSUInteger,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setReadHandler : handler, maxDatagrams : maxDatagrams)
    }
    unsafe fn writeMultipleDatagrams_completionHandler_(
        &self,
        datagramArray: NSArray,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, writeMultipleDatagrams : datagramArray, completionHandler : completionHandler)
    }
    unsafe fn writeDatagram_completionHandler_(
        &self,
        datagram: NSData,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, writeDatagram : datagram, completionHandler : completionHandler)
    }
    unsafe fn cancel(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, cancel)
    }
    unsafe fn state(&self) -> NWUDPSessionState
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, state)
    }
    unsafe fn endpoint(&self) -> NWEndpoint
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, endpoint)
    }
    unsafe fn resolvedEndpoint(&self) -> NWEndpoint
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, resolvedEndpoint)
    }
    unsafe fn isViable(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isViable)
    }
    unsafe fn hasBetterPath(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, hasBetterPath)
    }
    unsafe fn currentPath(&self) -> NWPath
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, currentPath)
    }
    unsafe fn maximumDatagramLength(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, maximumDatagramLength)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct NWTLSParameters(pub id);
impl std::ops::Deref for NWTLSParameters {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for NWTLSParameters {}
impl NWTLSParameters {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"NWTLSParameters").unwrap(), alloc) })
    }
}
impl INSObject for NWTLSParameters {}
impl PNSObject for NWTLSParameters {}
impl std::convert::TryFrom<NSObject> for NWTLSParameters {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<NWTLSParameters, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"NWTLSParameters").unwrap()) };
        if is_kind_of {
            Ok(NWTLSParameters(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to NWTLSParameters")
        }
    }
}
impl INWTLSParameters for NWTLSParameters {}
pub trait INWTLSParameters: Sized + std::ops::Deref {
    unsafe fn TLSSessionID(&self) -> NSData
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, TLSSessionID)
    }
    unsafe fn setTLSSessionID_(&self, TLSSessionID: NSData)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setTLSSessionID : TLSSessionID)
    }
    unsafe fn SSLCipherSuites(&self) -> NSSet
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, SSLCipherSuites)
    }
    unsafe fn setSSLCipherSuites_(&self, SSLCipherSuites: NSSet)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSSLCipherSuites : SSLCipherSuites)
    }
    unsafe fn minimumSSLProtocolVersion(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, minimumSSLProtocolVersion)
    }
    unsafe fn setMinimumSSLProtocolVersion_(&self, minimumSSLProtocolVersion: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMinimumSSLProtocolVersion : minimumSSLProtocolVersion)
    }
    unsafe fn maximumSSLProtocolVersion(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, maximumSSLProtocolVersion)
    }
    unsafe fn setMaximumSSLProtocolVersion_(&self, maximumSSLProtocolVersion: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMaximumSSLProtocolVersion : maximumSSLProtocolVersion)
    }
}
pub type NEURLFilterVerdict = NSInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct NEURLFilter(pub id);
impl std::ops::Deref for NEURLFilter {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for NEURLFilter {}
impl NEURLFilter {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"NEURLFilter").unwrap(), alloc) })
    }
}
impl INSObject for NEURLFilter {}
impl PNSObject for NEURLFilter {}
impl std::convert::TryFrom<NSObject> for NEURLFilter {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<NEURLFilter, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"NEURLFilter").unwrap()) };
        if is_kind_of {
            Ok(NEURLFilter(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to NEURLFilter")
        }
    }
}
impl INEURLFilter for NEURLFilter {}
pub trait INEURLFilter: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn verdictForURL_completionHandler_(
        url: NSURL,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NEURLFilter").unwrap(), verdictForURL : url, completionHandler : completionHandler)
    }
}
unsafe extern "C" {
    pub static NEAppProxyErrorDomain: NSString;
}
unsafe extern "C" {
    pub static NETunnelProviderErrorDomain: NSString;
}
unsafe extern "C" {
    pub static NEVPNErrorDomain: NSString;
}
unsafe extern "C" {
    pub static NEVPNConfigurationChangeNotification: NSString;
}
unsafe extern "C" {
    pub static NEDNSProxyErrorDomain: NSString;
}
unsafe extern "C" {
    pub static NEDNSProxyConfigurationDidChangeNotification: NSString;
}
unsafe extern "C" {
    pub static NEDNSSettingsErrorDomain: NSString;
}
unsafe extern "C" {
    pub static NEDNSSettingsConfigurationDidChangeNotification: NSString;
}
unsafe extern "C" {
    pub static mut NEFilterProviderRemediationMapRemediationURLs: NSString;
}
unsafe extern "C" {
    pub static mut NEFilterProviderRemediationMapRemediationButtonTexts: NSString;
}
unsafe extern "C" {
    pub static NEFilterErrorDomain: NSString;
}
unsafe extern "C" {
    pub static NEFilterConfigurationDidChangeNotification: NSString;
}
unsafe extern "C" {
    pub static mut kNEHotspotHelperOptionDisplayName: NSString;
}
unsafe extern "C" {
    pub static NEHotspotConfigurationErrorDomain: NSString;
}
unsafe extern "C" {
    pub static NERelayErrorDomain: NSString;
}
unsafe extern "C" {
    pub static NERelayClientErrorDomain: NSString;
}
unsafe extern "C" {
    pub static NERelayConfigurationDidChangeNotification: NSString;
}
unsafe extern "C" {
    pub static NEVPNStatusDidChangeNotification: NSString;
}
unsafe extern "C" {
    pub static NEVPNConnectionStartOptionUsername: NSString;
}
unsafe extern "C" {
    pub static NEVPNConnectionStartOptionPassword: NSString;
}
unsafe extern "C" {
    pub static NEVPNConnectionErrorDomain: NSString;
}
unsafe extern "C" {
    pub static NEAppPushErrorDomain: NSErrorDomain;
}

unsafe impl objc2::encode::RefEncode for AuthorizationOpaqueRef {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for AuthorizationOpaqueRef {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("AuthorizationOpaqueRef", &[]);
}
unsafe impl objc2::encode::RefEncode for NEAppProxyFlow {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for NEAppProxyFlow {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for NEProvider {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for NEProvider {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for NETunnelProvider {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for NETunnelProvider {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for NEAppProxyProvider {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for NEAppProxyProvider {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for NEVPNManager {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for NEVPNManager {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for NETunnelProviderManager {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for NETunnelProviderManager {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for NEAppProxyProviderManager {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for NEAppProxyProviderManager {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for NEAppProxyTCPFlow {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for NEAppProxyTCPFlow {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for NEAppProxyUDPFlow {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for NEAppProxyUDPFlow {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for NEAppRule {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for NEAppRule {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for NEDNSProxyManager {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for NEDNSProxyManager {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for NEDNSProxyProvider {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for NEDNSProxyProvider {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for NEProxyServer {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for NEProxyServer {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for NEProxySettings {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for NEProxySettings {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for NEVPNProtocol {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for NEVPNProtocol {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for NEDNSProxyProviderProtocol {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for NEDNSProxyProviderProtocol {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for NEDNSSettings {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for NEDNSSettings {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for NEDNSOverTLSSettings {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for NEDNSOverTLSSettings {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for NEDNSOverHTTPSSettings {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for NEDNSOverHTTPSSettings {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for NEDNSSettingsManager {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for NEDNSSettingsManager {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for NENetworkRule {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for NENetworkRule {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for NEFilterFlow {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for NEFilterFlow {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for NEFilterBrowserFlow {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for NEFilterBrowserFlow {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for NEFilterSocketFlow {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for NEFilterSocketFlow {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for NEFilterProvider {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for NEFilterProvider {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for NEFilterVerdict {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for NEFilterVerdict {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for NEFilterNewFlowVerdict {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for NEFilterNewFlowVerdict {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for NEFilterControlVerdict {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for NEFilterControlVerdict {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for NEFilterReport {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for NEFilterReport {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for NEFilterControlProvider {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for NEFilterControlProvider {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for NEFilterDataProvider {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for NEFilterDataProvider {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for NEFilterDataVerdict {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for NEFilterDataVerdict {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for NEFilterRemediationVerdict {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for NEFilterRemediationVerdict {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for NEFilterManager {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for NEFilterManager {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for NEFilterPacketContext {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for NEFilterPacketContext {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for NEFilterPacketProvider {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for NEFilterPacketProvider {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for NEFilterProviderConfiguration {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for NEFilterProviderConfiguration {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for NEFilterRule {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for NEFilterRule {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for NEFilterSettings {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for NEFilterSettings {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for NEFlowMetaData {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for NEFlowMetaData {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for NEHotspotNetwork {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for NEHotspotNetwork {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for NEHotspotHelperCommand {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for NEHotspotHelperCommand {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for NEHotspotHelperResponse {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for NEHotspotHelperResponse {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for NEHotspotHelper {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for NEHotspotHelper {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for NEHotspotHS20Settings {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for NEHotspotHS20Settings {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for NEHotspotEAPSettings {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for NEHotspotEAPSettings {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for NEHotspotConfiguration {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for NEHotspotConfiguration {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for NEHotspotConfigurationManager {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for NEHotspotConfigurationManager {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for NEIPv4Settings {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for NEIPv4Settings {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for NEIPv4Route {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for NEIPv4Route {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for NEIPv6Settings {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for NEIPv6Settings {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for NEIPv6Route {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for NEIPv6Route {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for NETunnelNetworkSettings {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for NETunnelNetworkSettings {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for NEPacketTunnelNetworkSettings {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for NEPacketTunnelNetworkSettings {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for NEEthernetTunnelNetworkSettings {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for NEEthernetTunnelNetworkSettings {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for NEPacketTunnelProvider {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for NEPacketTunnelProvider {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for NEEthernetTunnelProvider {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for NEEthernetTunnelProvider {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for NEOnDemandRule {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for NEOnDemandRule {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for NEOnDemandRuleConnect {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for NEOnDemandRuleConnect {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for NEOnDemandRuleDisconnect {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for NEOnDemandRuleDisconnect {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for NEOnDemandRuleIgnore {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for NEOnDemandRuleIgnore {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for NEOnDemandRuleEvaluateConnection {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for NEOnDemandRuleEvaluateConnection {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for NEEvaluateConnectionRule {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for NEEvaluateConnectionRule {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for NEPacket {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for NEPacket {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for NEPacketTunnelFlow {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for NEPacketTunnelFlow {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for NERelay {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for NERelay {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for NERelayManager {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for NERelayManager {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for NETransparentProxyManager {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for NETransparentProxyManager {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for NETransparentProxyNetworkSettings {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for NETransparentProxyNetworkSettings {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for NETransparentProxyProvider {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for NETransparentProxyProvider {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for NEVPNConnection {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for NEVPNConnection {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for NETunnelProviderSession {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for NETunnelProviderSession {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for NETunnelProviderProtocol {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for NETunnelProviderProtocol {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for NEVPNProtocolIPSec {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for NEVPNProtocolIPSec {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for NEVPNIKEv2SecurityAssociationParameters {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for NEVPNIKEv2SecurityAssociationParameters {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for NEVPNIKEv2PPKConfiguration {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for NEVPNIKEv2PPKConfiguration {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for NEVPNProtocolIKEv2 {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for NEVPNProtocolIKEv2 {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for NEPrivateLTENetwork {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for NEPrivateLTENetwork {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for NEAppPushManager {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for NEAppPushManager {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for NEAppPushProvider {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for NEAppPushProvider {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for NWEndpoint {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for NWEndpoint {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for NWHostEndpoint {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for NWHostEndpoint {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for NWBonjourServiceEndpoint {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for NWBonjourServiceEndpoint {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for NWPath {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for NWPath {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for NWTCPConnection {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for NWTCPConnection {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for NWUDPSession {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for NWUDPSession {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for NWTLSParameters {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for NWTLSParameters {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for NEURLFilter {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for NEURLFilter {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
