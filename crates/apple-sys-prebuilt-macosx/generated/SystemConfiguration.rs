#[allow(unused_imports)]
use crate::objc::*;
#[allow(unused_imports)]
use crate::CoreFoundation::*;
#[allow(unused_imports)]
use crate::Network::*;
#[allow(unused_imports)]
use crate::Security::*;
#[allow(unused_imports)]
use libc::{gid_t, id_t, socklen_t, uid_t};

#[allow(unused_imports)]
use objc2::msg_send;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __SCDynamicStore {
    _unused: [u8; 0],
}
pub type SCDynamicStoreRef = *const __SCDynamicStore;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SCDynamicStoreContext {
    pub version: CFIndex,
    pub info: *mut ::std::os::raw::c_void,
    pub retain: ::std::option::Option<
        unsafe extern "C" fn(info: *const ::std::os::raw::c_void) -> *const ::std::os::raw::c_void,
    >,
    pub release: ::std::option::Option<unsafe extern "C" fn(info: *const ::std::os::raw::c_void)>,
    pub copyDescription: ::std::option::Option<
        unsafe extern "C" fn(info: *const ::std::os::raw::c_void) -> CFStringRef,
    >,
}
pub type SCDynamicStoreCallBack = ::std::option::Option<
    unsafe extern "C" fn(
        store: SCDynamicStoreRef,
        changedKeys: CFArrayRef,
        info: *mut ::std::os::raw::c_void,
    ),
>;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __SCPreferences {
    _unused: [u8; 0],
}
pub type SCPreferencesRef = *const __SCPreferences;
pub type SCPreferencesNotification = u32;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SCPreferencesContext {
    pub version: CFIndex,
    pub info: *mut ::std::os::raw::c_void,
    pub retain: ::std::option::Option<
        unsafe extern "C" fn(info: *const ::std::os::raw::c_void) -> *const ::std::os::raw::c_void,
    >,
    pub release: ::std::option::Option<unsafe extern "C" fn(info: *const ::std::os::raw::c_void)>,
    pub copyDescription: ::std::option::Option<
        unsafe extern "C" fn(info: *const ::std::os::raw::c_void) -> CFStringRef,
    >,
}
pub type SCPreferencesCallBack = ::std::option::Option<
    unsafe extern "C" fn(
        prefs: SCPreferencesRef,
        notificationType: SCPreferencesNotification,
        info: *mut ::std::os::raw::c_void,
    ),
>;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __SCNetworkInterface {
    _unused: [u8; 0],
}
pub type SCNetworkInterfaceRef = *const __SCNetworkInterface;
pub type SCBondInterfaceRef = SCNetworkInterfaceRef;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __SCBondStatus {
    _unused: [u8; 0],
}
pub type SCBondStatusRef = *const __SCBondStatus;
pub type SCVLANInterfaceRef = SCNetworkInterfaceRef;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __SCNetworkProtocol {
    _unused: [u8; 0],
}
pub type SCNetworkProtocolRef = *const __SCNetworkProtocol;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __SCNetworkService {
    _unused: [u8; 0],
}
pub type SCNetworkServiceRef = *const __SCNetworkService;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __SCNetworkSet {
    _unused: [u8; 0],
}
pub type SCNetworkSetRef = *const __SCNetworkSet;
pub type SCNetworkConnectionFlags = u32;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __SCNetworkReachability {
    _unused: [u8; 0],
}
pub type SCNetworkReachabilityRef = *const __SCNetworkReachability;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SCNetworkReachabilityContext {
    pub version: CFIndex,
    pub info: *mut ::std::os::raw::c_void,
    pub retain: ::std::option::Option<
        unsafe extern "C" fn(info: *const ::std::os::raw::c_void) -> *const ::std::os::raw::c_void,
    >,
    pub release: ::std::option::Option<unsafe extern "C" fn(info: *const ::std::os::raw::c_void)>,
    pub copyDescription: ::std::option::Option<
        unsafe extern "C" fn(info: *const ::std::os::raw::c_void) -> CFStringRef,
    >,
}
pub type SCNetworkReachabilityFlags = u32;
pub type SCNetworkReachabilityCallBack = ::std::option::Option<
    unsafe extern "C" fn(
        target: SCNetworkReachabilityRef,
        flags: SCNetworkReachabilityFlags,
        info: *mut ::std::os::raw::c_void,
    ),
>;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __SCNetworkConnection {
    _unused: [u8; 0],
}
pub type SCNetworkConnectionRef = *const __SCNetworkConnection;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SCNetworkConnectionContext {
    pub version: CFIndex,
    pub info: *mut ::std::os::raw::c_void,
    pub retain: ::std::option::Option<
        unsafe extern "C" fn(info: *const ::std::os::raw::c_void) -> *const ::std::os::raw::c_void,
    >,
    pub release: ::std::option::Option<unsafe extern "C" fn(info: *const ::std::os::raw::c_void)>,
    pub copyDescription: ::std::option::Option<
        unsafe extern "C" fn(info: *const ::std::os::raw::c_void) -> CFStringRef,
    >,
}
pub type SCNetworkConnectionStatus = i32;
pub type SCNetworkConnectionPPPStatus = i32;
pub type SCNetworkConnectionCallBack = ::std::option::Option<
    unsafe extern "C" fn(
        connection: SCNetworkConnectionRef,
        status: SCNetworkConnectionStatus,
        info: *mut ::std::os::raw::c_void,
    ),
>;
unsafe extern "C" {
    pub fn SCDynamicStoreGetTypeID() -> CFTypeID;
}
unsafe extern "C" {
    pub fn SCDynamicStoreCreate(
        allocator: CFAllocatorRef,
        name: CFStringRef,
        callout: SCDynamicStoreCallBack,
        context: *mut SCDynamicStoreContext,
    ) -> SCDynamicStoreRef;
}
unsafe extern "C" {
    pub fn SCDynamicStoreCreateWithOptions(
        allocator: CFAllocatorRef,
        name: CFStringRef,
        storeOptions: CFDictionaryRef,
        callout: SCDynamicStoreCallBack,
        context: *mut SCDynamicStoreContext,
    ) -> SCDynamicStoreRef;
}
unsafe extern "C" {
    pub static kSCDynamicStoreUseSessionKeys: CFStringRef;
}
unsafe extern "C" {
    pub fn SCDynamicStoreCreateRunLoopSource(
        allocator: CFAllocatorRef,
        store: SCDynamicStoreRef,
        order: CFIndex,
    ) -> CFRunLoopSourceRef;
}
unsafe extern "C" {
    pub fn SCDynamicStoreSetDispatchQueue(store: SCDynamicStoreRef, queue: NSObject) -> Boolean;
}
unsafe extern "C" {
    pub fn SCDynamicStoreCopyKeyList(store: SCDynamicStoreRef, pattern: CFStringRef) -> CFArrayRef;
}
unsafe extern "C" {
    pub fn SCDynamicStoreAddValue(
        store: SCDynamicStoreRef,
        key: CFStringRef,
        value: CFPropertyListRef,
    ) -> Boolean;
}
unsafe extern "C" {
    pub fn SCDynamicStoreAddTemporaryValue(
        store: SCDynamicStoreRef,
        key: CFStringRef,
        value: CFPropertyListRef,
    ) -> Boolean;
}
unsafe extern "C" {
    pub fn SCDynamicStoreCopyValue(store: SCDynamicStoreRef, key: CFStringRef)
        -> CFPropertyListRef;
}
unsafe extern "C" {
    pub fn SCDynamicStoreCopyMultiple(
        store: SCDynamicStoreRef,
        keys: CFArrayRef,
        patterns: CFArrayRef,
    ) -> CFDictionaryRef;
}
unsafe extern "C" {
    pub fn SCDynamicStoreSetValue(
        store: SCDynamicStoreRef,
        key: CFStringRef,
        value: CFPropertyListRef,
    ) -> Boolean;
}
unsafe extern "C" {
    pub fn SCDynamicStoreSetMultiple(
        store: SCDynamicStoreRef,
        keysToSet: CFDictionaryRef,
        keysToRemove: CFArrayRef,
        keysToNotify: CFArrayRef,
    ) -> Boolean;
}
unsafe extern "C" {
    pub fn SCDynamicStoreRemoveValue(store: SCDynamicStoreRef, key: CFStringRef) -> Boolean;
}
unsafe extern "C" {
    pub fn SCDynamicStoreNotifyValue(store: SCDynamicStoreRef, key: CFStringRef) -> Boolean;
}
unsafe extern "C" {
    pub fn SCDynamicStoreSetNotificationKeys(
        store: SCDynamicStoreRef,
        keys: CFArrayRef,
        patterns: CFArrayRef,
    ) -> Boolean;
}
unsafe extern "C" {
    pub fn SCDynamicStoreCopyNotifiedKeys(store: SCDynamicStoreRef) -> CFArrayRef;
}
unsafe extern "C" {
    pub fn SCDynamicStoreKeyCreate(allocator: CFAllocatorRef, fmt: CFStringRef, ...)
        -> CFStringRef;
}
unsafe extern "C" {
    pub fn SCDynamicStoreKeyCreateNetworkGlobalEntity(
        allocator: CFAllocatorRef,
        domain: CFStringRef,
        entity: CFStringRef,
    ) -> CFStringRef;
}
unsafe extern "C" {
    pub fn SCDynamicStoreKeyCreateNetworkInterface(
        allocator: CFAllocatorRef,
        domain: CFStringRef,
    ) -> CFStringRef;
}
unsafe extern "C" {
    pub fn SCDynamicStoreKeyCreateNetworkInterfaceEntity(
        allocator: CFAllocatorRef,
        domain: CFStringRef,
        ifname: CFStringRef,
        entity: CFStringRef,
    ) -> CFStringRef;
}
unsafe extern "C" {
    pub fn SCDynamicStoreKeyCreateNetworkServiceEntity(
        allocator: CFAllocatorRef,
        domain: CFStringRef,
        serviceID: CFStringRef,
        entity: CFStringRef,
    ) -> CFStringRef;
}
unsafe extern "C" {
    pub fn SCDynamicStoreKeyCreateComputerName(allocator: CFAllocatorRef) -> CFStringRef;
}
unsafe extern "C" {
    pub fn SCDynamicStoreKeyCreateConsoleUser(allocator: CFAllocatorRef) -> CFStringRef;
}
unsafe extern "C" {
    pub fn SCDynamicStoreKeyCreateHostNames(allocator: CFAllocatorRef) -> CFStringRef;
}
unsafe extern "C" {
    pub fn SCDynamicStoreKeyCreateLocation(allocator: CFAllocatorRef) -> CFStringRef;
}
unsafe extern "C" {
    pub fn SCDynamicStoreKeyCreateProxies(allocator: CFAllocatorRef) -> CFStringRef;
}
unsafe extern "C" {
    pub fn SCDynamicStoreCopyComputerName(
        store: SCDynamicStoreRef,
        nameEncoding: *mut CFStringEncoding,
    ) -> CFStringRef;
}
unsafe extern "C" {
    pub fn SCDynamicStoreCopyConsoleUser(
        store: SCDynamicStoreRef,
        uid: *mut uid_t,
        gid: *mut gid_t,
    ) -> CFStringRef;
}
unsafe extern "C" {
    pub fn SCDynamicStoreCopyLocalHostName(store: SCDynamicStoreRef) -> CFStringRef;
}
unsafe extern "C" {
    pub fn SCDynamicStoreCopyLocation(store: SCDynamicStoreRef) -> CFStringRef;
}
unsafe extern "C" {
    pub fn SCDynamicStoreCopyProxies(store: SCDynamicStoreRef) -> CFDictionaryRef;
}
unsafe extern "C" {
    pub fn SCPreferencesGetTypeID() -> CFTypeID;
}
unsafe extern "C" {
    pub fn SCPreferencesCreate(
        allocator: CFAllocatorRef,
        name: CFStringRef,
        prefsID: CFStringRef,
    ) -> SCPreferencesRef;
}
unsafe extern "C" {
    pub fn SCPreferencesCreateWithAuthorization(
        allocator: CFAllocatorRef,
        name: CFStringRef,
        prefsID: CFStringRef,
        authorization: AuthorizationRef,
    ) -> SCPreferencesRef;
}
unsafe extern "C" {
    pub fn SCPreferencesLock(prefs: SCPreferencesRef, wait: Boolean) -> Boolean;
}
unsafe extern "C" {
    pub fn SCPreferencesCommitChanges(prefs: SCPreferencesRef) -> Boolean;
}
unsafe extern "C" {
    pub fn SCPreferencesApplyChanges(prefs: SCPreferencesRef) -> Boolean;
}
unsafe extern "C" {
    pub fn SCPreferencesUnlock(prefs: SCPreferencesRef) -> Boolean;
}
unsafe extern "C" {
    pub fn SCPreferencesGetSignature(prefs: SCPreferencesRef) -> CFDataRef;
}
unsafe extern "C" {
    pub fn SCPreferencesCopyKeyList(prefs: SCPreferencesRef) -> CFArrayRef;
}
unsafe extern "C" {
    pub fn SCPreferencesGetValue(prefs: SCPreferencesRef, key: CFStringRef) -> CFPropertyListRef;
}
unsafe extern "C" {
    pub fn SCPreferencesAddValue(
        prefs: SCPreferencesRef,
        key: CFStringRef,
        value: CFPropertyListRef,
    ) -> Boolean;
}
unsafe extern "C" {
    pub fn SCPreferencesSetValue(
        prefs: SCPreferencesRef,
        key: CFStringRef,
        value: CFPropertyListRef,
    ) -> Boolean;
}
unsafe extern "C" {
    pub fn SCPreferencesRemoveValue(prefs: SCPreferencesRef, key: CFStringRef) -> Boolean;
}
unsafe extern "C" {
    pub fn SCPreferencesSetCallback(
        prefs: SCPreferencesRef,
        callout: SCPreferencesCallBack,
        context: *mut SCPreferencesContext,
    ) -> Boolean;
}
unsafe extern "C" {
    pub fn SCPreferencesScheduleWithRunLoop(
        prefs: SCPreferencesRef,
        runLoop: CFRunLoopRef,
        runLoopMode: CFStringRef,
    ) -> Boolean;
}
unsafe extern "C" {
    pub fn SCPreferencesUnscheduleFromRunLoop(
        prefs: SCPreferencesRef,
        runLoop: CFRunLoopRef,
        runLoopMode: CFStringRef,
    ) -> Boolean;
}
unsafe extern "C" {
    pub fn SCPreferencesSetDispatchQueue(prefs: SCPreferencesRef, queue: NSObject) -> Boolean;
}
unsafe extern "C" {
    pub fn SCPreferencesSynchronize(prefs: SCPreferencesRef);
}
unsafe extern "C" {
    pub fn SCPreferencesPathCreateUniqueChild(
        prefs: SCPreferencesRef,
        prefix: CFStringRef,
    ) -> CFStringRef;
}
unsafe extern "C" {
    pub fn SCPreferencesPathGetValue(prefs: SCPreferencesRef, path: CFStringRef)
        -> CFDictionaryRef;
}
unsafe extern "C" {
    pub fn SCPreferencesPathGetLink(prefs: SCPreferencesRef, path: CFStringRef) -> CFStringRef;
}
unsafe extern "C" {
    pub fn SCPreferencesPathSetValue(
        prefs: SCPreferencesRef,
        path: CFStringRef,
        value: CFDictionaryRef,
    ) -> Boolean;
}
unsafe extern "C" {
    pub fn SCPreferencesPathSetLink(
        prefs: SCPreferencesRef,
        path: CFStringRef,
        link: CFStringRef,
    ) -> Boolean;
}
unsafe extern "C" {
    pub fn SCPreferencesPathRemoveValue(prefs: SCPreferencesRef, path: CFStringRef) -> Boolean;
}
unsafe extern "C" {
    pub fn SCPreferencesSetComputerName(
        prefs: SCPreferencesRef,
        name: CFStringRef,
        nameEncoding: CFStringEncoding,
    ) -> Boolean;
}
unsafe extern "C" {
    pub fn SCPreferencesSetLocalHostName(prefs: SCPreferencesRef, name: CFStringRef) -> Boolean;
}
unsafe extern "C" {
    pub static kSCResvLink: CFStringRef;
}
unsafe extern "C" {
    pub static kSCResvInactive: CFStringRef;
}
unsafe extern "C" {
    pub static kSCPropInterfaceName: CFStringRef;
}
unsafe extern "C" {
    pub static kSCPropMACAddress: CFStringRef;
}
unsafe extern "C" {
    pub static kSCPropUserDefinedName: CFStringRef;
}
unsafe extern "C" {
    pub static kSCPropVersion: CFStringRef;
}
unsafe extern "C" {
    pub static kSCPrefCurrentSet: CFStringRef;
}
unsafe extern "C" {
    pub static kSCPrefNetworkServices: CFStringRef;
}
unsafe extern "C" {
    pub static kSCPrefSets: CFStringRef;
}
unsafe extern "C" {
    pub static kSCPrefSystem: CFStringRef;
}
unsafe extern "C" {
    pub static kSCCompNetwork: CFStringRef;
}
unsafe extern "C" {
    pub static kSCCompService: CFStringRef;
}
unsafe extern "C" {
    pub static kSCCompGlobal: CFStringRef;
}
unsafe extern "C" {
    pub static kSCCompHostNames: CFStringRef;
}
unsafe extern "C" {
    pub static kSCCompInterface: CFStringRef;
}
unsafe extern "C" {
    pub static kSCCompSystem: CFStringRef;
}
unsafe extern "C" {
    pub static kSCCompUsers: CFStringRef;
}
unsafe extern "C" {
    pub static kSCCompAnyRegex: CFStringRef;
}
unsafe extern "C" {
    pub static kSCEntNetAirPort: CFStringRef;
}
unsafe extern "C" {
    pub static kSCEntNetDHCP: CFStringRef;
}
unsafe extern "C" {
    pub static kSCEntNetDNS: CFStringRef;
}
unsafe extern "C" {
    pub static kSCEntNetEthernet: CFStringRef;
}
unsafe extern "C" {
    pub static kSCEntNetFireWire: CFStringRef;
}
unsafe extern "C" {
    pub static kSCEntNetInterface: CFStringRef;
}
unsafe extern "C" {
    pub static kSCEntNetIPSec: CFStringRef;
}
unsafe extern "C" {
    pub static kSCEntNetIPv4: CFStringRef;
}
unsafe extern "C" {
    pub static kSCEntNetIPv6: CFStringRef;
}
unsafe extern "C" {
    pub static kSCEntNetL2TP: CFStringRef;
}
unsafe extern "C" {
    pub static kSCEntNetLink: CFStringRef;
}
unsafe extern "C" {
    pub static kSCEntNetModem: CFStringRef;
}
unsafe extern "C" {
    pub static kSCEntNetPPP: CFStringRef;
}
unsafe extern "C" {
    pub static kSCEntNetPPPoE: CFStringRef;
}
unsafe extern "C" {
    pub static kSCEntNetPPPSerial: CFStringRef;
}
unsafe extern "C" {
    pub static kSCEntNetPPTP: CFStringRef;
}
unsafe extern "C" {
    pub static kSCEntNetProxies: CFStringRef;
}
unsafe extern "C" {
    pub static kSCEntNetSMB: CFStringRef;
}
unsafe extern "C" {
    pub static kSCEntNet6to4: CFStringRef;
}
unsafe extern "C" {
    pub static kSCPropNetOverridePrimary: CFStringRef;
}
unsafe extern "C" {
    pub static kSCPropNetServiceOrder: CFStringRef;
}
unsafe extern "C" {
    pub static kSCPropNetPPPOverridePrimary: CFStringRef;
}
unsafe extern "C" {
    pub static kSCPropNetInterfaces: CFStringRef;
}
unsafe extern "C" {
    pub static kSCPropNetLocalHostName: CFStringRef;
}
unsafe extern "C" {
    pub static kSCPropNetAirPortAllowNetCreation: CFStringRef;
}
unsafe extern "C" {
    pub static kSCPropNetAirPortAuthPassword: CFStringRef;
}
unsafe extern "C" {
    pub static kSCPropNetAirPortAuthPasswordEncryption: CFStringRef;
}
unsafe extern "C" {
    pub static kSCPropNetAirPortJoinMode: CFStringRef;
}
unsafe extern "C" {
    pub static kSCPropNetAirPortPowerEnabled: CFStringRef;
}
unsafe extern "C" {
    pub static kSCPropNetAirPortPreferredNetwork: CFStringRef;
}
unsafe extern "C" {
    pub static kSCPropNetAirPortSavePasswords: CFStringRef;
}
unsafe extern "C" {
    pub static kSCValNetAirPortJoinModeAutomatic: CFStringRef;
}
unsafe extern "C" {
    pub static kSCValNetAirPortJoinModePreferred: CFStringRef;
}
unsafe extern "C" {
    pub static kSCValNetAirPortJoinModeRanked: CFStringRef;
}
unsafe extern "C" {
    pub static kSCValNetAirPortJoinModeRecent: CFStringRef;
}
unsafe extern "C" {
    pub static kSCValNetAirPortJoinModeStrongest: CFStringRef;
}
unsafe extern "C" {
    pub static kSCValNetAirPortAuthPasswordEncryptionKeychain: CFStringRef;
}
unsafe extern "C" {
    pub static kSCPropNetDNSDomainName: CFStringRef;
}
unsafe extern "C" {
    pub static kSCPropNetDNSOptions: CFStringRef;
}
unsafe extern "C" {
    pub static kSCPropNetDNSSearchDomains: CFStringRef;
}
unsafe extern "C" {
    pub static kSCPropNetDNSSearchOrder: CFStringRef;
}
unsafe extern "C" {
    pub static kSCPropNetDNSServerAddresses: CFStringRef;
}
unsafe extern "C" {
    pub static kSCPropNetDNSServerPort: CFStringRef;
}
unsafe extern "C" {
    pub static kSCPropNetDNSServerTimeout: CFStringRef;
}
unsafe extern "C" {
    pub static kSCPropNetDNSSortList: CFStringRef;
}
unsafe extern "C" {
    pub static kSCPropNetDNSSupplementalMatchDomains: CFStringRef;
}
unsafe extern "C" {
    pub static kSCPropNetDNSSupplementalMatchOrders: CFStringRef;
}
unsafe extern "C" {
    pub static kSCPropNetEthernetMediaSubType: CFStringRef;
}
unsafe extern "C" {
    pub static kSCPropNetEthernetMediaOptions: CFStringRef;
}
unsafe extern "C" {
    pub static kSCPropNetEthernetMTU: CFStringRef;
}
unsafe extern "C" {
    pub static kSCPropNetInterfaceDeviceName: CFStringRef;
}
unsafe extern "C" {
    pub static kSCPropNetInterfaceHardware: CFStringRef;
}
unsafe extern "C" {
    pub static kSCPropNetInterfaceType: CFStringRef;
}
unsafe extern "C" {
    pub static kSCPropNetInterfaceSubType: CFStringRef;
}
unsafe extern "C" {
    pub static kSCPropNetInterfaceSupportsModemOnHold: CFStringRef;
}
unsafe extern "C" {
    pub static kSCValNetInterfaceTypeEthernet: CFStringRef;
}
unsafe extern "C" {
    pub static kSCValNetInterfaceTypeFireWire: CFStringRef;
}
unsafe extern "C" {
    pub static kSCValNetInterfaceTypePPP: CFStringRef;
}
unsafe extern "C" {
    pub static kSCValNetInterfaceType6to4: CFStringRef;
}
unsafe extern "C" {
    pub static kSCValNetInterfaceTypeIPSec: CFStringRef;
}
unsafe extern "C" {
    pub static kSCValNetInterfaceSubTypePPPoE: CFStringRef;
}
unsafe extern "C" {
    pub static kSCValNetInterfaceSubTypePPPSerial: CFStringRef;
}
unsafe extern "C" {
    pub static kSCValNetInterfaceSubTypePPTP: CFStringRef;
}
unsafe extern "C" {
    pub static kSCValNetInterfaceSubTypeL2TP: CFStringRef;
}
unsafe extern "C" {
    pub static kSCPropNetIPSecAuthenticationMethod: CFStringRef;
}
unsafe extern "C" {
    pub static kSCPropNetIPSecLocalCertificate: CFStringRef;
}
unsafe extern "C" {
    pub static kSCPropNetIPSecLocalIdentifier: CFStringRef;
}
unsafe extern "C" {
    pub static kSCPropNetIPSecLocalIdentifierType: CFStringRef;
}
unsafe extern "C" {
    pub static kSCPropNetIPSecSharedSecret: CFStringRef;
}
unsafe extern "C" {
    pub static kSCPropNetIPSecSharedSecretEncryption: CFStringRef;
}
unsafe extern "C" {
    pub static kSCPropNetIPSecConnectTime: CFStringRef;
}
unsafe extern "C" {
    pub static kSCPropNetIPSecRemoteAddress: CFStringRef;
}
unsafe extern "C" {
    pub static kSCPropNetIPSecStatus: CFStringRef;
}
unsafe extern "C" {
    pub static kSCPropNetIPSecXAuthEnabled: CFStringRef;
}
unsafe extern "C" {
    pub static kSCPropNetIPSecXAuthName: CFStringRef;
}
unsafe extern "C" {
    pub static kSCPropNetIPSecXAuthPassword: CFStringRef;
}
unsafe extern "C" {
    pub static kSCPropNetIPSecXAuthPasswordEncryption: CFStringRef;
}
unsafe extern "C" {
    pub static kSCValNetIPSecAuthenticationMethodSharedSecret: CFStringRef;
}
unsafe extern "C" {
    pub static kSCValNetIPSecAuthenticationMethodCertificate: CFStringRef;
}
unsafe extern "C" {
    pub static kSCValNetIPSecAuthenticationMethodHybrid: CFStringRef;
}
unsafe extern "C" {
    pub static kSCValNetIPSecLocalIdentifierTypeKeyID: CFStringRef;
}
unsafe extern "C" {
    pub static kSCValNetIPSecSharedSecretEncryptionKeychain: CFStringRef;
}
unsafe extern "C" {
    pub static kSCValNetIPSecXAuthPasswordEncryptionKeychain: CFStringRef;
}
unsafe extern "C" {
    pub static kSCValNetIPSecXAuthPasswordEncryptionPrompt: CFStringRef;
}
unsafe extern "C" {
    pub static kSCPropNetIPv4Addresses: CFStringRef;
}
unsafe extern "C" {
    pub static kSCPropNetIPv4ConfigMethod: CFStringRef;
}
unsafe extern "C" {
    pub static kSCPropNetIPv4DHCPClientID: CFStringRef;
}
unsafe extern "C" {
    pub static kSCPropNetIPv4Router: CFStringRef;
}
unsafe extern "C" {
    pub static kSCPropNetIPv4SubnetMasks: CFStringRef;
}
unsafe extern "C" {
    pub static kSCPropNetIPv4DestAddresses: CFStringRef;
}
unsafe extern "C" {
    pub static kSCPropNetIPv4BroadcastAddresses: CFStringRef;
}
unsafe extern "C" {
    pub static kSCValNetIPv4ConfigMethodAutomatic: CFStringRef;
}
unsafe extern "C" {
    pub static kSCValNetIPv4ConfigMethodBOOTP: CFStringRef;
}
unsafe extern "C" {
    pub static kSCValNetIPv4ConfigMethodDHCP: CFStringRef;
}
unsafe extern "C" {
    pub static kSCValNetIPv4ConfigMethodINFORM: CFStringRef;
}
unsafe extern "C" {
    pub static kSCValNetIPv4ConfigMethodLinkLocal: CFStringRef;
}
unsafe extern "C" {
    pub static kSCValNetIPv4ConfigMethodManual: CFStringRef;
}
unsafe extern "C" {
    pub static kSCValNetIPv4ConfigMethodPPP: CFStringRef;
}
unsafe extern "C" {
    pub static kSCPropNetIPv6Addresses: CFStringRef;
}
unsafe extern "C" {
    pub static kSCPropNetIPv6ConfigMethod: CFStringRef;
}
unsafe extern "C" {
    pub static kSCPropNetIPv6DestAddresses: CFStringRef;
}
unsafe extern "C" {
    pub static kSCPropNetIPv6Flags: CFStringRef;
}
unsafe extern "C" {
    pub static kSCPropNetIPv6PrefixLength: CFStringRef;
}
unsafe extern "C" {
    pub static kSCPropNetIPv6Router: CFStringRef;
}
unsafe extern "C" {
    pub static kSCValNetIPv6ConfigMethodAutomatic: CFStringRef;
}
unsafe extern "C" {
    pub static kSCValNetIPv6ConfigMethodLinkLocal: CFStringRef;
}
unsafe extern "C" {
    pub static kSCValNetIPv6ConfigMethodManual: CFStringRef;
}
unsafe extern "C" {
    pub static kSCValNetIPv6ConfigMethodRouterAdvertisement: CFStringRef;
}
unsafe extern "C" {
    pub static kSCValNetIPv6ConfigMethod6to4: CFStringRef;
}
unsafe extern "C" {
    pub static kSCPropNet6to4Relay: CFStringRef;
}
unsafe extern "C" {
    pub static kSCPropNetLinkActive: CFStringRef;
}
unsafe extern "C" {
    pub static kSCPropNetLinkDetaching: CFStringRef;
}
unsafe extern "C" {
    pub static kSCPropNetModemAccessPointName: CFStringRef;
}
unsafe extern "C" {
    pub static kSCPropNetModemConnectionPersonality: CFStringRef;
}
unsafe extern "C" {
    pub static kSCPropNetModemConnectionScript: CFStringRef;
}
unsafe extern "C" {
    pub static kSCPropNetModemConnectSpeed: CFStringRef;
}
unsafe extern "C" {
    pub static kSCPropNetModemDataCompression: CFStringRef;
}
unsafe extern "C" {
    pub static kSCPropNetModemDeviceContextID: CFStringRef;
}
unsafe extern "C" {
    pub static kSCPropNetModemDeviceModel: CFStringRef;
}
unsafe extern "C" {
    pub static kSCPropNetModemDeviceVendor: CFStringRef;
}
unsafe extern "C" {
    pub static kSCPropNetModemDialMode: CFStringRef;
}
unsafe extern "C" {
    pub static kSCPropNetModemErrorCorrection: CFStringRef;
}
unsafe extern "C" {
    pub static kSCPropNetModemHoldCallWaitingAudibleAlert: CFStringRef;
}
unsafe extern "C" {
    pub static kSCPropNetModemHoldDisconnectOnAnswer: CFStringRef;
}
unsafe extern "C" {
    pub static kSCPropNetModemHoldEnabled: CFStringRef;
}
unsafe extern "C" {
    pub static kSCPropNetModemHoldReminder: CFStringRef;
}
unsafe extern "C" {
    pub static kSCPropNetModemHoldReminderTime: CFStringRef;
}
unsafe extern "C" {
    pub static kSCPropNetModemNote: CFStringRef;
}
unsafe extern "C" {
    pub static kSCPropNetModemPulseDial: CFStringRef;
}
unsafe extern "C" {
    pub static kSCPropNetModemSpeaker: CFStringRef;
}
unsafe extern "C" {
    pub static kSCPropNetModemSpeed: CFStringRef;
}
unsafe extern "C" {
    pub static kSCValNetModemDialModeIgnoreDialTone: CFStringRef;
}
unsafe extern "C" {
    pub static kSCValNetModemDialModeManual: CFStringRef;
}
unsafe extern "C" {
    pub static kSCValNetModemDialModeWaitForDialTone: CFStringRef;
}
unsafe extern "C" {
    pub static kSCPropNetPPPACSPEnabled: CFStringRef;
}
unsafe extern "C" {
    pub static kSCPropNetPPPConnectTime: CFStringRef;
}
unsafe extern "C" {
    pub static kSCPropNetPPPDeviceLastCause: CFStringRef;
}
unsafe extern "C" {
    pub static kSCPropNetPPPDialOnDemand: CFStringRef;
}
unsafe extern "C" {
    pub static kSCPropNetPPPDisconnectOnFastUserSwitch: CFStringRef;
}
unsafe extern "C" {
    pub static kSCPropNetPPPDisconnectOnIdle: CFStringRef;
}
unsafe extern "C" {
    pub static kSCPropNetPPPDisconnectOnIdleTimer: CFStringRef;
}
unsafe extern "C" {
    pub static kSCPropNetPPPDisconnectOnLogout: CFStringRef;
}
unsafe extern "C" {
    pub static kSCPropNetPPPDisconnectOnSleep: CFStringRef;
}
unsafe extern "C" {
    pub static kSCPropNetPPPDisconnectTime: CFStringRef;
}
unsafe extern "C" {
    pub static kSCPropNetPPPIdleReminder: CFStringRef;
}
unsafe extern "C" {
    pub static kSCPropNetPPPIdleReminderTimer: CFStringRef;
}
unsafe extern "C" {
    pub static kSCPropNetPPPLastCause: CFStringRef;
}
unsafe extern "C" {
    pub static kSCPropNetPPPLogfile: CFStringRef;
}
unsafe extern "C" {
    pub static kSCPropNetPPPPlugins: CFStringRef;
}
unsafe extern "C" {
    pub static kSCPropNetPPPRetryConnectTime: CFStringRef;
}
unsafe extern "C" {
    pub static kSCPropNetPPPSessionTimer: CFStringRef;
}
unsafe extern "C" {
    pub static kSCPropNetPPPStatus: CFStringRef;
}
unsafe extern "C" {
    pub static kSCPropNetPPPUseSessionTimer: CFStringRef;
}
unsafe extern "C" {
    pub static kSCPropNetPPPVerboseLogging: CFStringRef;
}
unsafe extern "C" {
    pub static kSCPropNetPPPAuthEAPPlugins: CFStringRef;
}
unsafe extern "C" {
    pub static kSCPropNetPPPAuthName: CFStringRef;
}
unsafe extern "C" {
    pub static kSCPropNetPPPAuthPassword: CFStringRef;
}
unsafe extern "C" {
    pub static kSCPropNetPPPAuthPasswordEncryption: CFStringRef;
}
unsafe extern "C" {
    pub static kSCPropNetPPPAuthPrompt: CFStringRef;
}
unsafe extern "C" {
    pub static kSCPropNetPPPAuthProtocol: CFStringRef;
}
unsafe extern "C" {
    pub static kSCValNetPPPAuthPasswordEncryptionKeychain: CFStringRef;
}
unsafe extern "C" {
    pub static kSCValNetPPPAuthPasswordEncryptionToken: CFStringRef;
}
unsafe extern "C" {
    pub static kSCValNetPPPAuthPromptBefore: CFStringRef;
}
unsafe extern "C" {
    pub static kSCValNetPPPAuthPromptAfter: CFStringRef;
}
unsafe extern "C" {
    pub static kSCValNetPPPAuthProtocolCHAP: CFStringRef;
}
unsafe extern "C" {
    pub static kSCValNetPPPAuthProtocolEAP: CFStringRef;
}
unsafe extern "C" {
    pub static kSCValNetPPPAuthProtocolMSCHAP1: CFStringRef;
}
unsafe extern "C" {
    pub static kSCValNetPPPAuthProtocolMSCHAP2: CFStringRef;
}
unsafe extern "C" {
    pub static kSCValNetPPPAuthProtocolPAP: CFStringRef;
}
unsafe extern "C" {
    pub static kSCPropNetPPPCommAlternateRemoteAddress: CFStringRef;
}
unsafe extern "C" {
    pub static kSCPropNetPPPCommConnectDelay: CFStringRef;
}
unsafe extern "C" {
    pub static kSCPropNetPPPCommDisplayTerminalWindow: CFStringRef;
}
unsafe extern "C" {
    pub static kSCPropNetPPPCommRedialCount: CFStringRef;
}
unsafe extern "C" {
    pub static kSCPropNetPPPCommRedialEnabled: CFStringRef;
}
unsafe extern "C" {
    pub static kSCPropNetPPPCommRedialInterval: CFStringRef;
}
unsafe extern "C" {
    pub static kSCPropNetPPPCommRemoteAddress: CFStringRef;
}
unsafe extern "C" {
    pub static kSCPropNetPPPCommTerminalScript: CFStringRef;
}
unsafe extern "C" {
    pub static kSCPropNetPPPCommUseTerminalScript: CFStringRef;
}
unsafe extern "C" {
    pub static kSCPropNetPPPCCPEnabled: CFStringRef;
}
unsafe extern "C" {
    pub static kSCPropNetPPPCCPMPPE40Enabled: CFStringRef;
}
unsafe extern "C" {
    pub static kSCPropNetPPPCCPMPPE128Enabled: CFStringRef;
}
unsafe extern "C" {
    pub static kSCPropNetPPPIPCPCompressionVJ: CFStringRef;
}
unsafe extern "C" {
    pub static kSCPropNetPPPIPCPUsePeerDNS: CFStringRef;
}
unsafe extern "C" {
    pub static kSCPropNetPPPLCPEchoEnabled: CFStringRef;
}
unsafe extern "C" {
    pub static kSCPropNetPPPLCPEchoFailure: CFStringRef;
}
unsafe extern "C" {
    pub static kSCPropNetPPPLCPEchoInterval: CFStringRef;
}
unsafe extern "C" {
    pub static kSCPropNetPPPLCPCompressionACField: CFStringRef;
}
unsafe extern "C" {
    pub static kSCPropNetPPPLCPCompressionPField: CFStringRef;
}
unsafe extern "C" {
    pub static kSCPropNetPPPLCPMRU: CFStringRef;
}
unsafe extern "C" {
    pub static kSCPropNetPPPLCPMTU: CFStringRef;
}
unsafe extern "C" {
    pub static kSCPropNetPPPLCPReceiveACCM: CFStringRef;
}
unsafe extern "C" {
    pub static kSCPropNetPPPLCPTransmitACCM: CFStringRef;
}
unsafe extern "C" {
    pub static kSCPropNetL2TPIPSecSharedSecret: CFStringRef;
}
unsafe extern "C" {
    pub static kSCPropNetL2TPIPSecSharedSecretEncryption: CFStringRef;
}
unsafe extern "C" {
    pub static kSCPropNetL2TPTransport: CFStringRef;
}
unsafe extern "C" {
    pub static kSCValNetL2TPIPSecSharedSecretEncryptionKeychain: CFStringRef;
}
unsafe extern "C" {
    pub static kSCValNetL2TPTransportIP: CFStringRef;
}
unsafe extern "C" {
    pub static kSCValNetL2TPTransportIPSec: CFStringRef;
}
unsafe extern "C" {
    pub static kSCPropNetProxiesExceptionsList: CFStringRef;
}
unsafe extern "C" {
    pub static kSCPropNetProxiesExcludeSimpleHostnames: CFStringRef;
}
unsafe extern "C" {
    pub static kSCPropNetProxiesFTPEnable: CFStringRef;
}
unsafe extern "C" {
    pub static kSCPropNetProxiesFTPPassive: CFStringRef;
}
unsafe extern "C" {
    pub static kSCPropNetProxiesFTPPort: CFStringRef;
}
unsafe extern "C" {
    pub static kSCPropNetProxiesFTPProxy: CFStringRef;
}
unsafe extern "C" {
    pub static kSCPropNetProxiesFTPUser: CFStringRef;
}
unsafe extern "C" {
    pub static kSCPropNetProxiesGopherEnable: CFStringRef;
}
unsafe extern "C" {
    pub static kSCPropNetProxiesGopherPort: CFStringRef;
}
unsafe extern "C" {
    pub static kSCPropNetProxiesGopherProxy: CFStringRef;
}
unsafe extern "C" {
    pub static kSCPropNetProxiesGopherUser: CFStringRef;
}
unsafe extern "C" {
    pub static kSCPropNetProxiesHTTPEnable: CFStringRef;
}
unsafe extern "C" {
    pub static kSCPropNetProxiesHTTPPort: CFStringRef;
}
unsafe extern "C" {
    pub static kSCPropNetProxiesHTTPProxy: CFStringRef;
}
unsafe extern "C" {
    pub static kSCPropNetProxiesHTTPUser: CFStringRef;
}
unsafe extern "C" {
    pub static kSCPropNetProxiesHTTPSEnable: CFStringRef;
}
unsafe extern "C" {
    pub static kSCPropNetProxiesHTTPSPort: CFStringRef;
}
unsafe extern "C" {
    pub static kSCPropNetProxiesHTTPSProxy: CFStringRef;
}
unsafe extern "C" {
    pub static kSCPropNetProxiesHTTPSUser: CFStringRef;
}
unsafe extern "C" {
    pub static kSCPropNetProxiesRTSPEnable: CFStringRef;
}
unsafe extern "C" {
    pub static kSCPropNetProxiesRTSPPort: CFStringRef;
}
unsafe extern "C" {
    pub static kSCPropNetProxiesRTSPProxy: CFStringRef;
}
unsafe extern "C" {
    pub static kSCPropNetProxiesRTSPUser: CFStringRef;
}
unsafe extern "C" {
    pub static kSCPropNetProxiesSOCKSEnable: CFStringRef;
}
unsafe extern "C" {
    pub static kSCPropNetProxiesSOCKSPort: CFStringRef;
}
unsafe extern "C" {
    pub static kSCPropNetProxiesSOCKSProxy: CFStringRef;
}
unsafe extern "C" {
    pub static kSCPropNetProxiesSOCKSUser: CFStringRef;
}
unsafe extern "C" {
    pub static kSCPropNetProxiesProxyAutoConfigEnable: CFStringRef;
}
unsafe extern "C" {
    pub static kSCPropNetProxiesProxyAutoConfigJavaScript: CFStringRef;
}
unsafe extern "C" {
    pub static kSCPropNetProxiesProxyAutoConfigURLString: CFStringRef;
}
unsafe extern "C" {
    pub static kSCPropNetProxiesProxyAutoDiscoveryEnable: CFStringRef;
}
unsafe extern "C" {
    pub static kSCPropNetSMBNetBIOSName: CFStringRef;
}
unsafe extern "C" {
    pub static kSCPropNetSMBNetBIOSNodeType: CFStringRef;
}
unsafe extern "C" {
    pub static kSCPropNetSMBNetBIOSScope: CFStringRef;
}
unsafe extern "C" {
    pub static kSCPropNetSMBWINSAddresses: CFStringRef;
}
unsafe extern "C" {
    pub static kSCPropNetSMBWorkgroup: CFStringRef;
}
unsafe extern "C" {
    pub static kSCValNetSMBNetBIOSNodeTypeBroadcast: CFStringRef;
}
unsafe extern "C" {
    pub static kSCValNetSMBNetBIOSNodeTypePeer: CFStringRef;
}
unsafe extern "C" {
    pub static kSCValNetSMBNetBIOSNodeTypeMixed: CFStringRef;
}
unsafe extern "C" {
    pub static kSCValNetSMBNetBIOSNodeTypeHybrid: CFStringRef;
}
unsafe extern "C" {
    pub static kSCEntUsersConsoleUser: CFStringRef;
}
unsafe extern "C" {
    pub static kSCPropSystemComputerName: CFStringRef;
}
unsafe extern "C" {
    pub static kSCPropSystemComputerNameEncoding: CFStringRef;
}
unsafe extern "C" {
    pub static kSCDynamicStoreDomainFile: CFStringRef;
}
unsafe extern "C" {
    pub static kSCDynamicStoreDomainPlugin: CFStringRef;
}
unsafe extern "C" {
    pub static kSCDynamicStoreDomainSetup: CFStringRef;
}
unsafe extern "C" {
    pub static kSCDynamicStoreDomainState: CFStringRef;
}
unsafe extern "C" {
    pub static kSCDynamicStoreDomainPrefs: CFStringRef;
}
unsafe extern "C" {
    pub static kSCDynamicStorePropSetupCurrentSet: CFStringRef;
}
unsafe extern "C" {
    pub static kSCDynamicStorePropSetupLastUpdated: CFStringRef;
}
unsafe extern "C" {
    pub static kSCDynamicStorePropNetInterfaces: CFStringRef;
}
unsafe extern "C" {
    pub static kSCDynamicStorePropNetPrimaryInterface: CFStringRef;
}
unsafe extern "C" {
    pub static kSCDynamicStorePropNetPrimaryService: CFStringRef;
}
unsafe extern "C" {
    pub static kSCDynamicStorePropNetServiceIDs: CFStringRef;
}
unsafe extern "C" {
    pub static kSCPropUsersConsoleUserName: CFStringRef;
}
unsafe extern "C" {
    pub static kSCPropUsersConsoleUserUID: CFStringRef;
}
unsafe extern "C" {
    pub static kSCPropUsersConsoleUserGID: CFStringRef;
}
unsafe extern "C" {
    pub static kSCNetworkInterfaceType6to4: CFStringRef;
}
unsafe extern "C" {
    pub static kSCNetworkInterfaceTypeBluetooth: CFStringRef;
}
unsafe extern "C" {
    pub static kSCNetworkInterfaceTypeBond: CFStringRef;
}
unsafe extern "C" {
    pub static kSCNetworkInterfaceTypeEthernet: CFStringRef;
}
unsafe extern "C" {
    pub static kSCNetworkInterfaceTypeFireWire: CFStringRef;
}
unsafe extern "C" {
    pub static kSCNetworkInterfaceTypeIEEE80211: CFStringRef;
}
unsafe extern "C" {
    pub static kSCNetworkInterfaceTypeIPSec: CFStringRef;
}
unsafe extern "C" {
    pub static kSCNetworkInterfaceTypeIrDA: CFStringRef;
}
unsafe extern "C" {
    pub static kSCNetworkInterfaceTypeL2TP: CFStringRef;
}
unsafe extern "C" {
    pub static kSCNetworkInterfaceTypeModem: CFStringRef;
}
unsafe extern "C" {
    pub static kSCNetworkInterfaceTypePPP: CFStringRef;
}
unsafe extern "C" {
    pub static kSCNetworkInterfaceTypePPTP: CFStringRef;
}
unsafe extern "C" {
    pub static kSCNetworkInterfaceTypeSerial: CFStringRef;
}
unsafe extern "C" {
    pub static kSCNetworkInterfaceTypeVLAN: CFStringRef;
}
unsafe extern "C" {
    pub static kSCNetworkInterfaceTypeWWAN: CFStringRef;
}
unsafe extern "C" {
    pub static kSCNetworkInterfaceTypeIPv4: CFStringRef;
}
unsafe extern "C" {
    pub static kSCNetworkInterfaceIPv4: SCNetworkInterfaceRef;
}
unsafe extern "C" {
    pub static kSCBondStatusDeviceAggregationStatus: CFStringRef;
}
unsafe extern "C" {
    pub static kSCBondStatusDeviceCollecting: CFStringRef;
}
unsafe extern "C" {
    pub static kSCBondStatusDeviceDistributing: CFStringRef;
}
unsafe extern "C" {
    pub static kSCNetworkProtocolTypeDNS: CFStringRef;
}
unsafe extern "C" {
    pub static kSCNetworkProtocolTypeIPv4: CFStringRef;
}
unsafe extern "C" {
    pub static kSCNetworkProtocolTypeIPv6: CFStringRef;
}
unsafe extern "C" {
    pub static kSCNetworkProtocolTypeProxies: CFStringRef;
}
unsafe extern "C" {
    pub static kSCNetworkProtocolTypeSMB: CFStringRef;
}
unsafe extern "C" {
    pub fn SCNetworkInterfaceGetTypeID() -> CFTypeID;
}
unsafe extern "C" {
    pub fn SCNetworkInterfaceCopyAll() -> CFArrayRef;
}
unsafe extern "C" {
    pub fn SCNetworkInterfaceGetSupportedInterfaceTypes(
        interface: SCNetworkInterfaceRef,
    ) -> CFArrayRef;
}
unsafe extern "C" {
    pub fn SCNetworkInterfaceGetSupportedProtocolTypes(
        interface: SCNetworkInterfaceRef,
    ) -> CFArrayRef;
}
unsafe extern "C" {
    pub fn SCNetworkInterfaceCreateWithInterface(
        interface: SCNetworkInterfaceRef,
        interfaceType: CFStringRef,
    ) -> SCNetworkInterfaceRef;
}
unsafe extern "C" {
    pub fn SCNetworkInterfaceGetBSDName(interface: SCNetworkInterfaceRef) -> CFStringRef;
}
unsafe extern "C" {
    pub fn SCNetworkInterfaceGetConfiguration(interface: SCNetworkInterfaceRef) -> CFDictionaryRef;
}
unsafe extern "C" {
    pub fn SCNetworkInterfaceGetExtendedConfiguration(
        interface: SCNetworkInterfaceRef,
        extendedType: CFStringRef,
    ) -> CFDictionaryRef;
}
unsafe extern "C" {
    pub fn SCNetworkInterfaceGetHardwareAddressString(
        interface: SCNetworkInterfaceRef,
    ) -> CFStringRef;
}
unsafe extern "C" {
    pub fn SCNetworkInterfaceGetInterface(
        interface: SCNetworkInterfaceRef,
    ) -> SCNetworkInterfaceRef;
}
unsafe extern "C" {
    pub fn SCNetworkInterfaceGetInterfaceType(interface: SCNetworkInterfaceRef) -> CFStringRef;
}
unsafe extern "C" {
    pub fn SCNetworkInterfaceGetLocalizedDisplayName(
        interface: SCNetworkInterfaceRef,
    ) -> CFStringRef;
}
unsafe extern "C" {
    pub fn SCNetworkInterfaceSetConfiguration(
        interface: SCNetworkInterfaceRef,
        config: CFDictionaryRef,
    ) -> Boolean;
}
unsafe extern "C" {
    pub fn SCNetworkInterfaceSetExtendedConfiguration(
        interface: SCNetworkInterfaceRef,
        extendedType: CFStringRef,
        config: CFDictionaryRef,
    ) -> Boolean;
}
unsafe extern "C" {
    pub fn SCNetworkInterfaceCopyMediaOptions(
        interface: SCNetworkInterfaceRef,
        current: *mut CFDictionaryRef,
        active: *mut CFDictionaryRef,
        available: *mut CFArrayRef,
        filter: Boolean,
    ) -> Boolean;
}
unsafe extern "C" {
    pub fn SCNetworkInterfaceCopyMediaSubTypes(available: CFArrayRef) -> CFArrayRef;
}
unsafe extern "C" {
    pub fn SCNetworkInterfaceCopyMediaSubTypeOptions(
        available: CFArrayRef,
        subType: CFStringRef,
    ) -> CFArrayRef;
}
unsafe extern "C" {
    pub fn SCNetworkInterfaceCopyMTU(
        interface: SCNetworkInterfaceRef,
        mtu_cur: *mut ::std::os::raw::c_int,
        mtu_min: *mut ::std::os::raw::c_int,
        mtu_max: *mut ::std::os::raw::c_int,
    ) -> Boolean;
}
unsafe extern "C" {
    pub fn SCNetworkInterfaceSetMediaOptions(
        interface: SCNetworkInterfaceRef,
        subtype: CFStringRef,
        options: CFArrayRef,
    ) -> Boolean;
}
unsafe extern "C" {
    pub fn SCNetworkInterfaceSetMTU(
        interface: SCNetworkInterfaceRef,
        mtu: ::std::os::raw::c_int,
    ) -> Boolean;
}
unsafe extern "C" {
    pub fn SCNetworkInterfaceForceConfigurationRefresh(interface: SCNetworkInterfaceRef)
        -> Boolean;
}
unsafe extern "C" {
    pub fn SCBondInterfaceCopyAll(prefs: SCPreferencesRef) -> CFArrayRef;
}
unsafe extern "C" {
    pub fn SCBondInterfaceCopyAvailableMemberInterfaces(prefs: SCPreferencesRef) -> CFArrayRef;
}
unsafe extern "C" {
    pub fn SCBondInterfaceCreate(prefs: SCPreferencesRef) -> SCBondInterfaceRef;
}
unsafe extern "C" {
    pub fn SCBondInterfaceRemove(bond: SCBondInterfaceRef) -> Boolean;
}
unsafe extern "C" {
    pub fn SCBondInterfaceGetMemberInterfaces(bond: SCBondInterfaceRef) -> CFArrayRef;
}
unsafe extern "C" {
    pub fn SCBondInterfaceGetOptions(bond: SCBondInterfaceRef) -> CFDictionaryRef;
}
unsafe extern "C" {
    pub fn SCBondInterfaceSetMemberInterfaces(
        bond: SCBondInterfaceRef,
        members: CFArrayRef,
    ) -> Boolean;
}
unsafe extern "C" {
    pub fn SCBondInterfaceSetLocalizedDisplayName(
        bond: SCBondInterfaceRef,
        newName: CFStringRef,
    ) -> Boolean;
}
unsafe extern "C" {
    pub fn SCBondInterfaceSetOptions(
        bond: SCBondInterfaceRef,
        newOptions: CFDictionaryRef,
    ) -> Boolean;
}
unsafe extern "C" {
    pub fn SCBondInterfaceCopyStatus(bond: SCBondInterfaceRef) -> SCBondStatusRef;
}
unsafe extern "C" {
    pub fn SCBondStatusGetTypeID() -> CFTypeID;
}
unsafe extern "C" {
    pub fn SCBondStatusGetMemberInterfaces(bondStatus: SCBondStatusRef) -> CFArrayRef;
}
unsafe extern "C" {
    pub fn SCBondStatusGetInterfaceStatus(
        bondStatus: SCBondStatusRef,
        interface: SCNetworkInterfaceRef,
    ) -> CFDictionaryRef;
}
unsafe extern "C" {
    pub fn SCVLANInterfaceCopyAll(prefs: SCPreferencesRef) -> CFArrayRef;
}
unsafe extern "C" {
    pub fn SCVLANInterfaceCopyAvailablePhysicalInterfaces() -> CFArrayRef;
}
unsafe extern "C" {
    pub fn SCVLANInterfaceCreate(
        prefs: SCPreferencesRef,
        physical: SCNetworkInterfaceRef,
        tag: CFNumberRef,
    ) -> SCVLANInterfaceRef;
}
unsafe extern "C" {
    pub fn SCVLANInterfaceRemove(vlan: SCVLANInterfaceRef) -> Boolean;
}
unsafe extern "C" {
    pub fn SCVLANInterfaceGetPhysicalInterface(vlan: SCVLANInterfaceRef) -> SCNetworkInterfaceRef;
}
unsafe extern "C" {
    pub fn SCVLANInterfaceGetTag(vlan: SCVLANInterfaceRef) -> CFNumberRef;
}
unsafe extern "C" {
    pub fn SCVLANInterfaceGetOptions(vlan: SCVLANInterfaceRef) -> CFDictionaryRef;
}
unsafe extern "C" {
    pub fn SCVLANInterfaceSetPhysicalInterfaceAndTag(
        vlan: SCVLANInterfaceRef,
        physical: SCNetworkInterfaceRef,
        tag: CFNumberRef,
    ) -> Boolean;
}
unsafe extern "C" {
    pub fn SCVLANInterfaceSetLocalizedDisplayName(
        vlan: SCVLANInterfaceRef,
        newName: CFStringRef,
    ) -> Boolean;
}
unsafe extern "C" {
    pub fn SCVLANInterfaceSetOptions(
        vlan: SCVLANInterfaceRef,
        newOptions: CFDictionaryRef,
    ) -> Boolean;
}
unsafe extern "C" {
    pub fn SCNetworkProtocolGetTypeID() -> CFTypeID;
}
unsafe extern "C" {
    pub fn SCNetworkProtocolGetConfiguration(protocol: SCNetworkProtocolRef) -> CFDictionaryRef;
}
unsafe extern "C" {
    pub fn SCNetworkProtocolGetEnabled(protocol: SCNetworkProtocolRef) -> Boolean;
}
unsafe extern "C" {
    pub fn SCNetworkProtocolGetProtocolType(protocol: SCNetworkProtocolRef) -> CFStringRef;
}
unsafe extern "C" {
    pub fn SCNetworkProtocolSetConfiguration(
        protocol: SCNetworkProtocolRef,
        config: CFDictionaryRef,
    ) -> Boolean;
}
unsafe extern "C" {
    pub fn SCNetworkProtocolSetEnabled(protocol: SCNetworkProtocolRef, enabled: Boolean)
        -> Boolean;
}
unsafe extern "C" {
    pub fn SCNetworkServiceGetTypeID() -> CFTypeID;
}
unsafe extern "C" {
    pub fn SCNetworkServiceAddProtocolType(
        service: SCNetworkServiceRef,
        protocolType: CFStringRef,
    ) -> Boolean;
}
unsafe extern "C" {
    pub fn SCNetworkServiceCopyAll(prefs: SCPreferencesRef) -> CFArrayRef;
}
unsafe extern "C" {
    pub fn SCNetworkServiceCopyProtocols(service: SCNetworkServiceRef) -> CFArrayRef;
}
unsafe extern "C" {
    pub fn SCNetworkServiceCreate(
        prefs: SCPreferencesRef,
        interface: SCNetworkInterfaceRef,
    ) -> SCNetworkServiceRef;
}
unsafe extern "C" {
    pub fn SCNetworkServiceCopy(
        prefs: SCPreferencesRef,
        serviceID: CFStringRef,
    ) -> SCNetworkServiceRef;
}
unsafe extern "C" {
    pub fn SCNetworkServiceEstablishDefaultConfiguration(service: SCNetworkServiceRef) -> Boolean;
}
unsafe extern "C" {
    pub fn SCNetworkServiceGetEnabled(service: SCNetworkServiceRef) -> Boolean;
}
unsafe extern "C" {
    pub fn SCNetworkServiceGetInterface(service: SCNetworkServiceRef) -> SCNetworkInterfaceRef;
}
unsafe extern "C" {
    pub fn SCNetworkServiceGetName(service: SCNetworkServiceRef) -> CFStringRef;
}
unsafe extern "C" {
    pub fn SCNetworkServiceCopyProtocol(
        service: SCNetworkServiceRef,
        protocolType: CFStringRef,
    ) -> SCNetworkProtocolRef;
}
unsafe extern "C" {
    pub fn SCNetworkServiceGetServiceID(service: SCNetworkServiceRef) -> CFStringRef;
}
unsafe extern "C" {
    pub fn SCNetworkServiceRemove(service: SCNetworkServiceRef) -> Boolean;
}
unsafe extern "C" {
    pub fn SCNetworkServiceRemoveProtocolType(
        service: SCNetworkServiceRef,
        protocolType: CFStringRef,
    ) -> Boolean;
}
unsafe extern "C" {
    pub fn SCNetworkServiceSetEnabled(service: SCNetworkServiceRef, enabled: Boolean) -> Boolean;
}
unsafe extern "C" {
    pub fn SCNetworkServiceSetName(service: SCNetworkServiceRef, name: CFStringRef) -> Boolean;
}
unsafe extern "C" {
    pub fn SCNetworkSetGetTypeID() -> CFTypeID;
}
unsafe extern "C" {
    pub fn SCNetworkSetAddService(set: SCNetworkSetRef, service: SCNetworkServiceRef) -> Boolean;
}
unsafe extern "C" {
    pub fn SCNetworkSetContainsInterface(
        set: SCNetworkSetRef,
        interface: SCNetworkInterfaceRef,
    ) -> Boolean;
}
unsafe extern "C" {
    pub fn SCNetworkSetCopyAll(prefs: SCPreferencesRef) -> CFArrayRef;
}
unsafe extern "C" {
    pub fn SCNetworkSetCopyCurrent(prefs: SCPreferencesRef) -> SCNetworkSetRef;
}
unsafe extern "C" {
    pub fn SCNetworkSetCopyServices(set: SCNetworkSetRef) -> CFArrayRef;
}
unsafe extern "C" {
    pub fn SCNetworkSetCreate(prefs: SCPreferencesRef) -> SCNetworkSetRef;
}
unsafe extern "C" {
    pub fn SCNetworkSetCopy(prefs: SCPreferencesRef, setID: CFStringRef) -> SCNetworkSetRef;
}
unsafe extern "C" {
    pub fn SCNetworkSetGetName(set: SCNetworkSetRef) -> CFStringRef;
}
unsafe extern "C" {
    pub fn SCNetworkSetGetSetID(set: SCNetworkSetRef) -> CFStringRef;
}
unsafe extern "C" {
    pub fn SCNetworkSetGetServiceOrder(set: SCNetworkSetRef) -> CFArrayRef;
}
unsafe extern "C" {
    pub fn SCNetworkSetRemove(set: SCNetworkSetRef) -> Boolean;
}
unsafe extern "C" {
    pub fn SCNetworkSetRemoveService(set: SCNetworkSetRef, service: SCNetworkServiceRef)
        -> Boolean;
}
unsafe extern "C" {
    pub fn SCNetworkSetSetCurrent(set: SCNetworkSetRef) -> Boolean;
}
unsafe extern "C" {
    pub fn SCNetworkSetSetName(set: SCNetworkSetRef, name: CFStringRef) -> Boolean;
}
unsafe extern "C" {
    pub fn SCNetworkSetSetServiceOrder(set: SCNetworkSetRef, newOrder: CFArrayRef) -> Boolean;
}
unsafe extern "C" {
    pub fn SCNetworkCheckReachabilityByAddress(
        address: *const sockaddr,
        addrlen: socklen_t,
        flags: *mut SCNetworkConnectionFlags,
    ) -> Boolean;
}
unsafe extern "C" {
    pub fn SCNetworkCheckReachabilityByName(
        nodename: *const ::std::os::raw::c_char,
        flags: *mut SCNetworkConnectionFlags,
    ) -> Boolean;
}
unsafe extern "C" {
    pub fn SCNetworkInterfaceRefreshConfiguration(ifName: CFStringRef) -> Boolean;
}
unsafe extern "C" {
    pub fn SCNetworkReachabilityCreateWithAddress(
        allocator: CFAllocatorRef,
        address: *const sockaddr,
    ) -> SCNetworkReachabilityRef;
}
unsafe extern "C" {
    pub fn SCNetworkReachabilityCreateWithAddressPair(
        allocator: CFAllocatorRef,
        localAddress: *const sockaddr,
        remoteAddress: *const sockaddr,
    ) -> SCNetworkReachabilityRef;
}
unsafe extern "C" {
    pub fn SCNetworkReachabilityCreateWithName(
        allocator: CFAllocatorRef,
        nodename: *const ::std::os::raw::c_char,
    ) -> SCNetworkReachabilityRef;
}
unsafe extern "C" {
    pub fn SCNetworkReachabilityGetTypeID() -> CFTypeID;
}
unsafe extern "C" {
    pub fn SCNetworkReachabilityGetFlags(
        target: SCNetworkReachabilityRef,
        flags: *mut SCNetworkReachabilityFlags,
    ) -> Boolean;
}
unsafe extern "C" {
    pub fn SCNetworkReachabilitySetCallback(
        target: SCNetworkReachabilityRef,
        callout: SCNetworkReachabilityCallBack,
        context: *mut SCNetworkReachabilityContext,
    ) -> Boolean;
}
unsafe extern "C" {
    pub fn SCNetworkReachabilityScheduleWithRunLoop(
        target: SCNetworkReachabilityRef,
        runLoop: CFRunLoopRef,
        runLoopMode: CFStringRef,
    ) -> Boolean;
}
unsafe extern "C" {
    pub fn SCNetworkReachabilityUnscheduleFromRunLoop(
        target: SCNetworkReachabilityRef,
        runLoop: CFRunLoopRef,
        runLoopMode: CFStringRef,
    ) -> Boolean;
}
unsafe extern "C" {
    pub fn SCNetworkReachabilitySetDispatchQueue(
        target: SCNetworkReachabilityRef,
        queue: NSObject,
    ) -> Boolean;
}
unsafe extern "C" {
    pub fn SCNetworkConnectionGetTypeID() -> CFTypeID;
}
unsafe extern "C" {
    pub fn SCNetworkConnectionCopyUserPreferences(
        selectionOptions: CFDictionaryRef,
        serviceID: *mut CFStringRef,
        userOptions: *mut CFDictionaryRef,
    ) -> Boolean;
}
unsafe extern "C" {
    pub fn SCNetworkConnectionCreateWithServiceID(
        allocator: CFAllocatorRef,
        serviceID: CFStringRef,
        callout: SCNetworkConnectionCallBack,
        context: *mut SCNetworkConnectionContext,
    ) -> SCNetworkConnectionRef;
}
unsafe extern "C" {
    pub fn SCNetworkConnectionCopyServiceID(connection: SCNetworkConnectionRef) -> CFStringRef;
}
unsafe extern "C" {
    pub fn SCNetworkConnectionGetStatus(
        connection: SCNetworkConnectionRef,
    ) -> SCNetworkConnectionStatus;
}
unsafe extern "C" {
    pub fn SCNetworkConnectionCopyExtendedStatus(
        connection: SCNetworkConnectionRef,
    ) -> CFDictionaryRef;
}
unsafe extern "C" {
    pub fn SCNetworkConnectionCopyStatistics(connection: SCNetworkConnectionRef)
        -> CFDictionaryRef;
}
unsafe extern "C" {
    pub fn SCNetworkConnectionStart(
        connection: SCNetworkConnectionRef,
        userOptions: CFDictionaryRef,
        linger: Boolean,
    ) -> Boolean;
}
unsafe extern "C" {
    pub fn SCNetworkConnectionStop(
        connection: SCNetworkConnectionRef,
        forceDisconnect: Boolean,
    ) -> Boolean;
}
unsafe extern "C" {
    pub fn SCNetworkConnectionCopyUserOptions(
        connection: SCNetworkConnectionRef,
    ) -> CFDictionaryRef;
}
unsafe extern "C" {
    pub fn SCNetworkConnectionScheduleWithRunLoop(
        connection: SCNetworkConnectionRef,
        runLoop: CFRunLoopRef,
        runLoopMode: CFStringRef,
    ) -> Boolean;
}
unsafe extern "C" {
    pub fn SCNetworkConnectionUnscheduleFromRunLoop(
        connection: SCNetworkConnectionRef,
        runLoop: CFRunLoopRef,
        runLoopMode: CFStringRef,
    ) -> Boolean;
}
unsafe extern "C" {
    pub fn SCNetworkConnectionSetDispatchQueue(
        connection: SCNetworkConnectionRef,
        queue: NSObject,
    ) -> Boolean;
}
unsafe extern "C" {
    pub static kCFErrorDomainSystemConfiguration: CFStringRef;
}
unsafe extern "C" {
    pub fn SCCopyLastError() -> CFErrorRef;
}
unsafe extern "C" {
    pub fn SCError() -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn SCErrorString(status: ::std::os::raw::c_int) -> *const ::std::os::raw::c_char;
}
unsafe extern "C" {
    pub fn CNSetSupportedSSIDs(ssidArray: CFArrayRef) -> Boolean;
}
unsafe extern "C" {
    pub fn CNMarkPortalOnline(interfaceName: CFStringRef) -> Boolean;
}
unsafe extern "C" {
    pub fn CNMarkPortalOffline(interfaceName: CFStringRef) -> Boolean;
}
unsafe extern "C" {
    pub fn CNCopySupportedInterfaces() -> CFArrayRef;
}
unsafe extern "C" {
    pub static kCNNetworkInfoKeySSIDData: CFStringRef;
}
unsafe extern "C" {
    pub static kCNNetworkInfoKeySSID: CFStringRef;
}
unsafe extern "C" {
    pub static kCNNetworkInfoKeyBSSID: CFStringRef;
}
unsafe extern "C" {
    pub fn CNCopyCurrentNetworkInfo(interfaceName: CFStringRef) -> CFDictionaryRef;
}
unsafe extern "C" {
    pub fn DHCPClientPreferencesSetApplicationOptions(
        applicationID: CFStringRef,
        options: *const UInt8,
        count: CFIndex,
    ) -> Boolean;
}
unsafe extern "C" {
    pub fn DHCPClientPreferencesCopyApplicationOptions(
        applicationID: CFStringRef,
        count: *mut CFIndex,
    ) -> *mut UInt8;
}
unsafe extern "C" {
    pub fn SCDynamicStoreCopyDHCPInfo(
        store: SCDynamicStoreRef,
        serviceID: CFStringRef,
    ) -> CFDictionaryRef;
}
unsafe extern "C" {
    pub fn DHCPInfoGetOptionData(info: CFDictionaryRef, code: UInt8) -> CFDataRef;
}
unsafe extern "C" {
    pub fn DHCPInfoGetLeaseStartTime(info: CFDictionaryRef) -> CFDateRef;
}
unsafe extern "C" {
    pub fn DHCPInfoGetLeaseExpirationTime(info: CFDictionaryRef) -> CFDateRef;
}

unsafe impl objc2::encode::RefEncode for __SCDynamicStore {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for __SCDynamicStore {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("__SCDynamicStore", &[]);
}
unsafe impl objc2::encode::RefEncode for SCDynamicStoreContext {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for SCDynamicStoreContext {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("SCDynamicStoreContext", &[]);
}
unsafe impl objc2::encode::RefEncode for __SCPreferences {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for __SCPreferences {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("__SCPreferences", &[]);
}
unsafe impl objc2::encode::RefEncode for SCPreferencesContext {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for SCPreferencesContext {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("SCPreferencesContext", &[]);
}
unsafe impl objc2::encode::RefEncode for __SCNetworkInterface {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for __SCNetworkInterface {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("__SCNetworkInterface", &[]);
}
unsafe impl objc2::encode::RefEncode for __SCBondStatus {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for __SCBondStatus {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("__SCBondStatus", &[]);
}
unsafe impl objc2::encode::RefEncode for __SCNetworkProtocol {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for __SCNetworkProtocol {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("__SCNetworkProtocol", &[]);
}
unsafe impl objc2::encode::RefEncode for __SCNetworkService {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for __SCNetworkService {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("__SCNetworkService", &[]);
}
unsafe impl objc2::encode::RefEncode for __SCNetworkSet {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for __SCNetworkSet {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("__SCNetworkSet", &[]);
}
unsafe impl objc2::encode::RefEncode for __SCNetworkReachability {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for __SCNetworkReachability {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("__SCNetworkReachability", &[]);
}
unsafe impl objc2::encode::RefEncode for SCNetworkReachabilityContext {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for SCNetworkReachabilityContext {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("SCNetworkReachabilityContext", &[]);
}
unsafe impl objc2::encode::RefEncode for __SCNetworkConnection {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for __SCNetworkConnection {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("__SCNetworkConnection", &[]);
}
unsafe impl objc2::encode::RefEncode for SCNetworkConnectionContext {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for SCNetworkConnectionContext {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("SCNetworkConnectionContext", &[]);
}
