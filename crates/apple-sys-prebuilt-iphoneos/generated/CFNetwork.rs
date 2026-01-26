#[allow(unused_imports)]
use crate::CoreFoundation::*;
#[allow(unused_imports)]
use crate::CoreText::*;

#[allow(unused_imports)]
use objc2::msg_send;
pub type UInt8 = ::std::os::raw::c_uchar;
pub type SInt32 = ::std::os::raw::c_int;
pub type CFOptionFlags = ::std::os::raw::c_ulong;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __CFAllocator {
    _unused: [u8; 0],
}
pub type CFAllocatorRetainCallBack = ::std::option::Option<
    unsafe extern "C" fn(info: *const ::std::os::raw::c_void) -> *const ::std::os::raw::c_void,
>;
pub type CFAllocatorReleaseCallBack =
    ::std::option::Option<unsafe extern "C" fn(info: *const ::std::os::raw::c_void)>;
pub type CFAllocatorCopyDescriptionCallBack =
    ::std::option::Option<unsafe extern "C" fn(info: *const ::std::os::raw::c_void) -> CFStringRef>;
pub type CFNetworkErrors = ::std::os::raw::c_int;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __CFHost {
    _unused: [u8; 0],
}
pub type CFHostRef = *mut __CFHost;
pub type CFHostInfoType = ::std::os::raw::c_int;
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct CFHostClientContext {
    pub version: CFIndex,
    pub info: *mut ::std::os::raw::c_void,
    pub retain: CFAllocatorRetainCallBack,
    pub release: CFAllocatorReleaseCallBack,
    pub copyDescription: CFAllocatorCopyDescriptionCallBack,
}
pub type CFHostClientCallBack = ::std::option::Option<
    unsafe extern "C" fn(
        theHost: CFHostRef,
        typeInfo: CFHostInfoType,
        error: *const CFStreamError,
        info: *mut ::std::os::raw::c_void,
    ),
>;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __CFNetService {
    _unused: [u8; 0],
}
pub type CFNetServiceRef = *mut __CFNetService;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __CFNetServiceMonitor {
    _unused: [u8; 0],
}
pub type CFNetServiceMonitorRef = *mut __CFNetServiceMonitor;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __CFNetServiceBrowser {
    _unused: [u8; 0],
}
pub type CFNetServiceBrowserRef = *mut __CFNetServiceBrowser;
pub type CFNetServicesError = ::std::os::raw::c_int;
pub type CFNetServiceMonitorType = ::std::os::raw::c_int;
pub type CFNetServiceRegisterFlags = CFOptionFlags;
pub type CFNetServiceBrowserFlags = CFOptionFlags;
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct CFNetServiceClientContext {
    pub version: CFIndex,
    pub info: *mut ::std::os::raw::c_void,
    pub retain: CFAllocatorRetainCallBack,
    pub release: CFAllocatorReleaseCallBack,
    pub copyDescription: CFAllocatorCopyDescriptionCallBack,
}
pub type CFNetServiceClientCallBack = ::std::option::Option<
    unsafe extern "C" fn(
        theService: CFNetServiceRef,
        error: *mut CFStreamError,
        info: *mut ::std::os::raw::c_void,
    ),
>;
pub type CFNetServiceMonitorClientCallBack = ::std::option::Option<
    unsafe extern "C" fn(
        theMonitor: CFNetServiceMonitorRef,
        theService: CFNetServiceRef,
        typeInfo: CFNetServiceMonitorType,
        rdata: CFDataRef,
        error: *mut CFStreamError,
        info: *mut ::std::os::raw::c_void,
    ),
>;
pub type CFNetServiceBrowserClientCallBack = ::std::option::Option<
    unsafe extern "C" fn(
        browser: CFNetServiceBrowserRef,
        flags: CFOptionFlags,
        domainOrService: CFTypeRef,
        error: *mut CFStreamError,
        info: *mut ::std::os::raw::c_void,
    ),
>;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __CFHTTPMessage {
    _unused: [u8; 0],
}
pub type CFHTTPMessageRef = *mut __CFHTTPMessage;
pub type CFStreamErrorHTTP = ::std::os::raw::c_int;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _CFHTTPAuthentication {
    _unused: [u8; 0],
}
pub type CFHTTPAuthenticationRef = *mut _CFHTTPAuthentication;
pub type CFStreamErrorHTTPAuthentication = ::std::os::raw::c_int;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __CFNetDiagnostic {
    _unused: [u8; 0],
}
pub type CFNetDiagnosticRef = *mut __CFNetDiagnostic;
pub type CFNetDiagnosticStatusValues = ::std::os::raw::c_int;
pub type CFNetDiagnosticStatus = CFIndex;
pub type CFProxyAutoConfigurationResultCallback = ::std::option::Option<
    unsafe extern "C" fn(
        client: *mut ::std::os::raw::c_void,
        proxyList: CFArrayRef,
        error: CFErrorRef,
    ),
>;
unsafe extern "C" {
    pub static kCFErrorDomainCFNetwork: CFStringRef;
}
unsafe extern "C" {
    pub static kCFErrorDomainWinSock: CFStringRef;
}
unsafe extern "C" {
    pub static kCFURLErrorFailingURLErrorKey: CFStringRef;
}
unsafe extern "C" {
    pub static kCFURLErrorFailingURLStringErrorKey: CFStringRef;
}
unsafe extern "C" {
    pub static kCFGetAddrInfoFailureKey: CFStringRef;
}
unsafe extern "C" {
    pub static kCFSOCKSStatusCodeKey: CFStringRef;
}
unsafe extern "C" {
    pub static kCFSOCKSVersionKey: CFStringRef;
}
unsafe extern "C" {
    pub static kCFSOCKSNegotiationMethodKey: CFStringRef;
}
unsafe extern "C" {
    pub static kCFDNSServiceFailureKey: CFStringRef;
}
unsafe extern "C" {
    pub static kCFFTPStatusCodeKey: CFStringRef;
}
unsafe extern "C" {
    pub static kCFStreamErrorDomainNetDB: SInt32;
}
unsafe extern "C" {
    pub static kCFStreamErrorDomainSystemConfiguration: SInt32;
}
unsafe extern "C" {
    pub fn CFHostGetTypeID() -> CFTypeID;
}
unsafe extern "C" {
    pub fn CFHostCreateWithName(allocator: CFAllocatorRef, hostname: CFStringRef) -> CFHostRef;
}
unsafe extern "C" {
    pub fn CFHostCreateWithAddress(allocator: CFAllocatorRef, addr: CFDataRef) -> CFHostRef;
}
unsafe extern "C" {
    pub fn CFHostCreateCopy(alloc: CFAllocatorRef, host: CFHostRef) -> CFHostRef;
}
unsafe extern "C" {
    pub fn CFHostStartInfoResolution(
        theHost: CFHostRef,
        info: CFHostInfoType,
        error: *mut CFStreamError,
    ) -> Boolean;
}
unsafe extern "C" {
    pub fn CFHostGetAddressing(theHost: CFHostRef, hasBeenResolved: *mut Boolean) -> CFArrayRef;
}
unsafe extern "C" {
    pub fn CFHostGetNames(theHost: CFHostRef, hasBeenResolved: *mut Boolean) -> CFArrayRef;
}
unsafe extern "C" {
    pub fn CFHostGetReachability(theHost: CFHostRef, hasBeenResolved: *mut Boolean) -> CFDataRef;
}
unsafe extern "C" {
    pub fn CFHostCancelInfoResolution(theHost: CFHostRef, info: CFHostInfoType);
}
unsafe extern "C" {
    pub fn CFHostSetClient(
        theHost: CFHostRef,
        clientCB: CFHostClientCallBack,
        clientContext: *mut CFHostClientContext,
    ) -> Boolean;
}
unsafe extern "C" {
    pub fn CFHostScheduleWithRunLoop(
        theHost: CFHostRef,
        runLoop: CFRunLoopRef,
        runLoopMode: CFStringRef,
    );
}
unsafe extern "C" {
    pub fn CFHostUnscheduleFromRunLoop(
        theHost: CFHostRef,
        runLoop: CFRunLoopRef,
        runLoopMode: CFStringRef,
    );
}
unsafe extern "C" {
    pub static kCFStreamErrorDomainMach: SInt32;
}
unsafe extern "C" {
    pub static kCFStreamErrorDomainNetServices: SInt32;
}
unsafe extern "C" {
    pub fn CFNetServiceGetTypeID() -> CFTypeID;
}
unsafe extern "C" {
    pub fn CFNetServiceMonitorGetTypeID() -> CFTypeID;
}
unsafe extern "C" {
    pub fn CFNetServiceBrowserGetTypeID() -> CFTypeID;
}
unsafe extern "C" {
    pub fn CFNetServiceCreate(
        alloc: CFAllocatorRef,
        domain: CFStringRef,
        serviceType: CFStringRef,
        name: CFStringRef,
        port: SInt32,
    ) -> CFNetServiceRef;
}
unsafe extern "C" {
    pub fn CFNetServiceCreateCopy(
        alloc: CFAllocatorRef,
        service: CFNetServiceRef,
    ) -> CFNetServiceRef;
}
unsafe extern "C" {
    pub fn CFNetServiceGetDomain(theService: CFNetServiceRef) -> CFStringRef;
}
unsafe extern "C" {
    pub fn CFNetServiceGetType(theService: CFNetServiceRef) -> CFStringRef;
}
unsafe extern "C" {
    pub fn CFNetServiceGetName(theService: CFNetServiceRef) -> CFStringRef;
}
unsafe extern "C" {
    pub fn CFNetServiceRegisterWithOptions(
        theService: CFNetServiceRef,
        options: CFOptionFlags,
        error: *mut CFStreamError,
    ) -> Boolean;
}
unsafe extern "C" {
    pub fn CFNetServiceResolveWithTimeout(
        theService: CFNetServiceRef,
        timeout: CFTimeInterval,
        error: *mut CFStreamError,
    ) -> Boolean;
}
unsafe extern "C" {
    pub fn CFNetServiceCancel(theService: CFNetServiceRef);
}
unsafe extern "C" {
    pub fn CFNetServiceGetTargetHost(theService: CFNetServiceRef) -> CFStringRef;
}
unsafe extern "C" {
    pub fn CFNetServiceGetPortNumber(theService: CFNetServiceRef) -> SInt32;
}
unsafe extern "C" {
    pub fn CFNetServiceGetAddressing(theService: CFNetServiceRef) -> CFArrayRef;
}
unsafe extern "C" {
    pub fn CFNetServiceGetTXTData(theService: CFNetServiceRef) -> CFDataRef;
}
unsafe extern "C" {
    pub fn CFNetServiceSetTXTData(theService: CFNetServiceRef, txtRecord: CFDataRef) -> Boolean;
}
unsafe extern "C" {
    pub fn CFNetServiceCreateDictionaryWithTXTData(
        alloc: CFAllocatorRef,
        txtRecord: CFDataRef,
    ) -> CFDictionaryRef;
}
unsafe extern "C" {
    pub fn CFNetServiceCreateTXTDataWithDictionary(
        alloc: CFAllocatorRef,
        keyValuePairs: CFDictionaryRef,
    ) -> CFDataRef;
}
unsafe extern "C" {
    pub fn CFNetServiceSetClient(
        theService: CFNetServiceRef,
        clientCB: CFNetServiceClientCallBack,
        clientContext: *mut CFNetServiceClientContext,
    ) -> Boolean;
}
unsafe extern "C" {
    pub fn CFNetServiceScheduleWithRunLoop(
        theService: CFNetServiceRef,
        runLoop: CFRunLoopRef,
        runLoopMode: CFStringRef,
    );
}
unsafe extern "C" {
    pub fn CFNetServiceUnscheduleFromRunLoop(
        theService: CFNetServiceRef,
        runLoop: CFRunLoopRef,
        runLoopMode: CFStringRef,
    );
}
unsafe extern "C" {
    pub fn CFNetServiceMonitorCreate(
        alloc: CFAllocatorRef,
        theService: CFNetServiceRef,
        clientCB: CFNetServiceMonitorClientCallBack,
        clientContext: *mut CFNetServiceClientContext,
    ) -> CFNetServiceMonitorRef;
}
unsafe extern "C" {
    pub fn CFNetServiceMonitorInvalidate(monitor: CFNetServiceMonitorRef);
}
unsafe extern "C" {
    pub fn CFNetServiceMonitorStart(
        monitor: CFNetServiceMonitorRef,
        recordType: CFNetServiceMonitorType,
        error: *mut CFStreamError,
    ) -> Boolean;
}
unsafe extern "C" {
    pub fn CFNetServiceMonitorStop(monitor: CFNetServiceMonitorRef, error: *mut CFStreamError);
}
unsafe extern "C" {
    pub fn CFNetServiceMonitorScheduleWithRunLoop(
        monitor: CFNetServiceMonitorRef,
        runLoop: CFRunLoopRef,
        runLoopMode: CFStringRef,
    );
}
unsafe extern "C" {
    pub fn CFNetServiceMonitorUnscheduleFromRunLoop(
        monitor: CFNetServiceMonitorRef,
        runLoop: CFRunLoopRef,
        runLoopMode: CFStringRef,
    );
}
unsafe extern "C" {
    pub fn CFNetServiceBrowserCreate(
        alloc: CFAllocatorRef,
        clientCB: CFNetServiceBrowserClientCallBack,
        clientContext: *mut CFNetServiceClientContext,
    ) -> CFNetServiceBrowserRef;
}
unsafe extern "C" {
    pub fn CFNetServiceBrowserInvalidate(browser: CFNetServiceBrowserRef);
}
unsafe extern "C" {
    pub fn CFNetServiceBrowserSearchForDomains(
        browser: CFNetServiceBrowserRef,
        registrationDomains: Boolean,
        error: *mut CFStreamError,
    ) -> Boolean;
}
unsafe extern "C" {
    pub fn CFNetServiceBrowserSearchForServices(
        browser: CFNetServiceBrowserRef,
        domain: CFStringRef,
        serviceType: CFStringRef,
        error: *mut CFStreamError,
    ) -> Boolean;
}
unsafe extern "C" {
    pub fn CFNetServiceBrowserStopSearch(
        browser: CFNetServiceBrowserRef,
        error: *mut CFStreamError,
    );
}
unsafe extern "C" {
    pub fn CFNetServiceBrowserScheduleWithRunLoop(
        browser: CFNetServiceBrowserRef,
        runLoop: CFRunLoopRef,
        runLoopMode: CFStringRef,
    );
}
unsafe extern "C" {
    pub fn CFNetServiceBrowserUnscheduleFromRunLoop(
        browser: CFNetServiceBrowserRef,
        runLoop: CFRunLoopRef,
        runLoopMode: CFStringRef,
    );
}
unsafe extern "C" {
    pub static kCFStreamPropertySSLContext: CFStringRef;
}
unsafe extern "C" {
    pub static kCFStreamPropertySSLPeerTrust: CFStringRef;
}
unsafe extern "C" {
    pub static kCFStreamSSLValidatesCertificateChain: CFStringRef;
}
unsafe extern "C" {
    pub static kCFStreamPropertySSLSettings: CFStringRef;
}
unsafe extern "C" {
    pub static kCFStreamSSLLevel: CFStringRef;
}
unsafe extern "C" {
    pub static kCFStreamSSLPeerName: CFStringRef;
}
unsafe extern "C" {
    pub static kCFStreamSSLCertificates: CFStringRef;
}
unsafe extern "C" {
    pub static kCFStreamSSLIsServer: CFStringRef;
}
unsafe extern "C" {
    pub static kCFStreamNetworkServiceType: CFStringRef;
}
unsafe extern "C" {
    pub static kCFStreamNetworkServiceTypeVideo: CFStringRef;
}
unsafe extern "C" {
    pub static kCFStreamNetworkServiceTypeVoice: CFStringRef;
}
unsafe extern "C" {
    pub static kCFStreamNetworkServiceTypeBackground: CFStringRef;
}
unsafe extern "C" {
    pub static kCFStreamNetworkServiceTypeResponsiveData: CFStringRef;
}
unsafe extern "C" {
    pub static kCFStreamNetworkServiceTypeCallSignaling: CFStringRef;
}
unsafe extern "C" {
    pub static kCFStreamNetworkServiceTypeAVStreaming: CFStringRef;
}
unsafe extern "C" {
    pub static kCFStreamNetworkServiceTypeResponsiveAV: CFStringRef;
}
unsafe extern "C" {
    pub static kCFStreamNetworkServiceTypeVoIP: CFStringRef;
}
unsafe extern "C" {
    pub static kCFStreamPropertyNoCellular: CFStringRef;
}
unsafe extern "C" {
    pub static kCFStreamPropertyConnectionIsCellular: CFStringRef;
}
unsafe extern "C" {
    pub static kCFStreamPropertyAllowExpensiveNetworkAccess: CFStringRef;
}
unsafe extern "C" {
    pub static kCFStreamPropertyConnectionIsExpensive: CFStringRef;
}
unsafe extern "C" {
    pub static kCFStreamPropertyAllowConstrainedNetworkAccess: CFStringRef;
}
unsafe extern "C" {
    pub static kCFStreamErrorDomainWinSock: CFIndex;
}
unsafe extern "C" {
    pub static kCFStreamPropertyProxyLocalBypass: CFStringRef;
}
unsafe extern "C" {
    pub static kCFStreamPropertySocketRemoteHost: CFStringRef;
}
unsafe extern "C" {
    pub static kCFStreamPropertySocketRemoteNetService: CFStringRef;
}
unsafe extern "C" {
    pub static kCFStreamPropertySocketExtendedBackgroundIdleMode: CFStringRef;
}
unsafe extern "C" {
    pub fn CFStreamCreatePairWithSocketToCFHost(
        alloc: CFAllocatorRef,
        host: CFHostRef,
        port: SInt32,
        readStream: *mut CFReadStreamRef,
        writeStream: *mut CFWriteStreamRef,
    );
}
unsafe extern "C" {
    pub fn CFStreamCreatePairWithSocketToNetService(
        alloc: CFAllocatorRef,
        service: CFNetServiceRef,
        readStream: *mut CFReadStreamRef,
        writeStream: *mut CFWriteStreamRef,
    );
}
unsafe extern "C" {
    pub static kCFStreamPropertySSLPeerCertificates: CFStringRef;
}
unsafe extern "C" {
    pub static kCFStreamSSLAllowsExpiredCertificates: CFStringRef;
}
unsafe extern "C" {
    pub static kCFStreamSSLAllowsExpiredRoots: CFStringRef;
}
unsafe extern "C" {
    pub static kCFStreamSSLAllowsAnyRoot: CFStringRef;
}
unsafe extern "C" {
    pub static kCFStreamErrorDomainFTP: SInt32;
}
unsafe extern "C" {
    pub static kCFStreamPropertyFTPUserName: CFStringRef;
}
unsafe extern "C" {
    pub static kCFStreamPropertyFTPPassword: CFStringRef;
}
unsafe extern "C" {
    pub static kCFStreamPropertyFTPUsePassiveMode: CFStringRef;
}
unsafe extern "C" {
    pub static kCFStreamPropertyFTPResourceSize: CFStringRef;
}
unsafe extern "C" {
    pub static kCFStreamPropertyFTPFetchResourceInfo: CFStringRef;
}
unsafe extern "C" {
    pub static kCFStreamPropertyFTPFileTransferOffset: CFStringRef;
}
unsafe extern "C" {
    pub static kCFStreamPropertyFTPAttemptPersistentConnection: CFStringRef;
}
unsafe extern "C" {
    pub static kCFStreamPropertyFTPProxy: CFStringRef;
}
unsafe extern "C" {
    pub static kCFStreamPropertyFTPProxyHost: CFStringRef;
}
unsafe extern "C" {
    pub static kCFStreamPropertyFTPProxyPort: CFStringRef;
}
unsafe extern "C" {
    pub static kCFStreamPropertyFTPProxyUser: CFStringRef;
}
unsafe extern "C" {
    pub static kCFStreamPropertyFTPProxyPassword: CFStringRef;
}
unsafe extern "C" {
    pub static kCFFTPResourceMode: CFStringRef;
}
unsafe extern "C" {
    pub static kCFFTPResourceName: CFStringRef;
}
unsafe extern "C" {
    pub static kCFFTPResourceOwner: CFStringRef;
}
unsafe extern "C" {
    pub static kCFFTPResourceGroup: CFStringRef;
}
unsafe extern "C" {
    pub static kCFFTPResourceLink: CFStringRef;
}
unsafe extern "C" {
    pub static kCFFTPResourceSize: CFStringRef;
}
unsafe extern "C" {
    pub static kCFFTPResourceType: CFStringRef;
}
unsafe extern "C" {
    pub static kCFFTPResourceModDate: CFStringRef;
}
unsafe extern "C" {
    pub fn CFReadStreamCreateWithFTPURL(alloc: CFAllocatorRef, ftpURL: CFURLRef)
        -> CFReadStreamRef;
}
unsafe extern "C" {
    pub fn CFFTPCreateParsedResourceListing(
        alloc: CFAllocatorRef,
        buffer: *const UInt8,
        bufferLength: CFIndex,
        parsed: *mut CFDictionaryRef,
    ) -> CFIndex;
}
unsafe extern "C" {
    pub fn CFWriteStreamCreateWithFTPURL(
        alloc: CFAllocatorRef,
        ftpURL: CFURLRef,
    ) -> CFWriteStreamRef;
}
unsafe extern "C" {
    pub static kCFHTTPVersion1_0: CFStringRef;
}
unsafe extern "C" {
    pub static kCFHTTPVersion1_1: CFStringRef;
}
unsafe extern "C" {
    pub static kCFHTTPVersion2_0: CFStringRef;
}
unsafe extern "C" {
    pub static kCFHTTPVersion3_0: CFStringRef;
}
unsafe extern "C" {
    pub static kCFHTTPAuthenticationSchemeBasic: CFStringRef;
}
unsafe extern "C" {
    pub static kCFHTTPAuthenticationSchemeDigest: CFStringRef;
}
unsafe extern "C" {
    pub static kCFHTTPAuthenticationSchemeNTLM: CFStringRef;
}
unsafe extern "C" {
    pub static kCFHTTPAuthenticationSchemeKerberos: CFStringRef;
}
unsafe extern "C" {
    pub static kCFHTTPAuthenticationSchemeNegotiate: CFStringRef;
}
unsafe extern "C" {
    pub static kCFHTTPAuthenticationSchemeNegotiate2: CFStringRef;
}
unsafe extern "C" {
    pub static kCFHTTPAuthenticationSchemeXMobileMeAuthToken: CFStringRef;
}
unsafe extern "C" {
    pub fn CFHTTPMessageGetTypeID() -> CFTypeID;
}
unsafe extern "C" {
    pub fn CFHTTPMessageCreateRequest(
        alloc: CFAllocatorRef,
        requestMethod: CFStringRef,
        url: CFURLRef,
        httpVersion: CFStringRef,
    ) -> CFHTTPMessageRef;
}
unsafe extern "C" {
    pub fn CFHTTPMessageCreateResponse(
        alloc: CFAllocatorRef,
        statusCode: CFIndex,
        statusDescription: CFStringRef,
        httpVersion: CFStringRef,
    ) -> CFHTTPMessageRef;
}
unsafe extern "C" {
    pub fn CFHTTPMessageCreateEmpty(alloc: CFAllocatorRef, isRequest: Boolean) -> CFHTTPMessageRef;
}
unsafe extern "C" {
    pub fn CFHTTPMessageCreateCopy(
        alloc: CFAllocatorRef,
        message: CFHTTPMessageRef,
    ) -> CFHTTPMessageRef;
}
unsafe extern "C" {
    pub fn CFHTTPMessageIsRequest(message: CFHTTPMessageRef) -> Boolean;
}
unsafe extern "C" {
    pub fn CFHTTPMessageCopyVersion(message: CFHTTPMessageRef) -> CFStringRef;
}
unsafe extern "C" {
    pub fn CFHTTPMessageCopyBody(message: CFHTTPMessageRef) -> CFDataRef;
}
unsafe extern "C" {
    pub fn CFHTTPMessageSetBody(message: CFHTTPMessageRef, bodyData: CFDataRef);
}
unsafe extern "C" {
    pub fn CFHTTPMessageCopyHeaderFieldValue(
        message: CFHTTPMessageRef,
        headerField: CFStringRef,
    ) -> CFStringRef;
}
unsafe extern "C" {
    pub fn CFHTTPMessageCopyAllHeaderFields(message: CFHTTPMessageRef) -> CFDictionaryRef;
}
unsafe extern "C" {
    pub fn CFHTTPMessageSetHeaderFieldValue(
        message: CFHTTPMessageRef,
        headerField: CFStringRef,
        value: CFStringRef,
    );
}
unsafe extern "C" {
    pub fn CFHTTPMessageAppendBytes(
        message: CFHTTPMessageRef,
        newBytes: *const UInt8,
        numBytes: CFIndex,
    ) -> Boolean;
}
unsafe extern "C" {
    pub fn CFHTTPMessageIsHeaderComplete(message: CFHTTPMessageRef) -> Boolean;
}
unsafe extern "C" {
    pub fn CFHTTPMessageCopySerializedMessage(message: CFHTTPMessageRef) -> CFDataRef;
}
unsafe extern "C" {
    pub fn CFHTTPMessageCopyRequestURL(request: CFHTTPMessageRef) -> CFURLRef;
}
unsafe extern "C" {
    pub fn CFHTTPMessageCopyRequestMethod(request: CFHTTPMessageRef) -> CFStringRef;
}
unsafe extern "C" {
    pub fn CFHTTPMessageAddAuthentication(
        request: CFHTTPMessageRef,
        authenticationFailureResponse: CFHTTPMessageRef,
        username: CFStringRef,
        password: CFStringRef,
        authenticationScheme: CFStringRef,
        forProxy: Boolean,
    ) -> Boolean;
}
unsafe extern "C" {
    pub fn CFHTTPMessageGetResponseStatusCode(response: CFHTTPMessageRef) -> CFIndex;
}
unsafe extern "C" {
    pub fn CFHTTPMessageCopyResponseStatusLine(response: CFHTTPMessageRef) -> CFStringRef;
}
unsafe extern "C" {
    pub static kCFStreamErrorDomainHTTP: SInt32;
}
unsafe extern "C" {
    pub static kCFStreamPropertyHTTPResponseHeader: CFStringRef;
}
unsafe extern "C" {
    pub static kCFStreamPropertyHTTPFinalURL: CFStringRef;
}
unsafe extern "C" {
    pub static kCFStreamPropertyHTTPFinalRequest: CFStringRef;
}
unsafe extern "C" {
    pub static kCFStreamPropertyHTTPProxy: CFStringRef;
}
unsafe extern "C" {
    pub static kCFStreamPropertyHTTPProxyHost: CFStringRef;
}
unsafe extern "C" {
    pub static kCFStreamPropertyHTTPProxyPort: CFStringRef;
}
unsafe extern "C" {
    pub static kCFStreamPropertyHTTPSProxyHost: CFStringRef;
}
unsafe extern "C" {
    pub static kCFStreamPropertyHTTPSProxyPort: CFStringRef;
}
unsafe extern "C" {
    pub static kCFStreamPropertyHTTPShouldAutoredirect: CFStringRef;
}
unsafe extern "C" {
    pub static kCFStreamPropertyHTTPAttemptPersistentConnection: CFStringRef;
}
unsafe extern "C" {
    pub static kCFStreamPropertyHTTPRequestBytesWrittenCount: CFStringRef;
}
unsafe extern "C" {
    pub fn CFReadStreamCreateForHTTPRequest(
        alloc: CFAllocatorRef,
        request: CFHTTPMessageRef,
    ) -> CFReadStreamRef;
}
unsafe extern "C" {
    pub fn CFReadStreamCreateForStreamedHTTPRequest(
        alloc: CFAllocatorRef,
        requestHeaders: CFHTTPMessageRef,
        requestBody: CFReadStreamRef,
    ) -> CFReadStreamRef;
}
unsafe extern "C" {
    pub static kCFHTTPAuthenticationUsername: CFStringRef;
}
unsafe extern "C" {
    pub static kCFHTTPAuthenticationPassword: CFStringRef;
}
unsafe extern "C" {
    pub static kCFHTTPAuthenticationAccountDomain: CFStringRef;
}
unsafe extern "C" {
    pub fn CFHTTPAuthenticationGetTypeID() -> CFTypeID;
}
unsafe extern "C" {
    pub fn CFHTTPAuthenticationCreateFromResponse(
        alloc: CFAllocatorRef,
        response: CFHTTPMessageRef,
    ) -> CFHTTPAuthenticationRef;
}
unsafe extern "C" {
    pub fn CFHTTPAuthenticationIsValid(
        auth: CFHTTPAuthenticationRef,
        error: *mut CFStreamError,
    ) -> Boolean;
}
unsafe extern "C" {
    pub fn CFHTTPAuthenticationAppliesToRequest(
        auth: CFHTTPAuthenticationRef,
        request: CFHTTPMessageRef,
    ) -> Boolean;
}
unsafe extern "C" {
    pub fn CFHTTPAuthenticationRequiresOrderedRequests(auth: CFHTTPAuthenticationRef) -> Boolean;
}
unsafe extern "C" {
    pub fn CFHTTPMessageApplyCredentials(
        request: CFHTTPMessageRef,
        auth: CFHTTPAuthenticationRef,
        username: CFStringRef,
        password: CFStringRef,
        error: *mut CFStreamError,
    ) -> Boolean;
}
unsafe extern "C" {
    pub fn CFHTTPMessageApplyCredentialDictionary(
        request: CFHTTPMessageRef,
        auth: CFHTTPAuthenticationRef,
        dict: CFDictionaryRef,
        error: *mut CFStreamError,
    ) -> Boolean;
}
unsafe extern "C" {
    pub fn CFHTTPAuthenticationCopyRealm(auth: CFHTTPAuthenticationRef) -> CFStringRef;
}
unsafe extern "C" {
    pub fn CFHTTPAuthenticationCopyDomains(auth: CFHTTPAuthenticationRef) -> CFArrayRef;
}
unsafe extern "C" {
    pub fn CFHTTPAuthenticationCopyMethod(auth: CFHTTPAuthenticationRef) -> CFStringRef;
}
unsafe extern "C" {
    pub fn CFHTTPAuthenticationRequiresUserNameAndPassword(
        auth: CFHTTPAuthenticationRef,
    ) -> Boolean;
}
unsafe extern "C" {
    pub fn CFHTTPAuthenticationRequiresAccountDomain(auth: CFHTTPAuthenticationRef) -> Boolean;
}
unsafe extern "C" {
    pub fn CFNetDiagnosticCreateWithStreams(
        alloc: CFAllocatorRef,
        readStream: CFReadStreamRef,
        writeStream: CFWriteStreamRef,
    ) -> CFNetDiagnosticRef;
}
unsafe extern "C" {
    pub fn CFNetDiagnosticCreateWithURL(alloc: CFAllocatorRef, url: CFURLRef)
        -> CFNetDiagnosticRef;
}
unsafe extern "C" {
    pub fn CFNetDiagnosticSetName(details: CFNetDiagnosticRef, name: CFStringRef);
}
unsafe extern "C" {
    pub fn CFNetDiagnosticDiagnoseProblemInteractively(
        details: CFNetDiagnosticRef,
    ) -> CFNetDiagnosticStatus;
}
unsafe extern "C" {
    pub fn CFNetDiagnosticCopyNetworkStatusPassively(
        details: CFNetDiagnosticRef,
        description: *mut CFStringRef,
    ) -> CFNetDiagnosticStatus;
}
unsafe extern "C" {
    pub fn CFNetworkCopySystemProxySettings() -> CFDictionaryRef;
}
unsafe extern "C" {
    pub fn CFNetworkCopyProxiesForURL(url: CFURLRef, proxySettings: CFDictionaryRef) -> CFArrayRef;
}
unsafe extern "C" {
    pub fn CFNetworkCopyProxiesForAutoConfigurationScript(
        proxyAutoConfigurationScript: CFStringRef,
        targetURL: CFURLRef,
        error: *mut CFErrorRef,
    ) -> CFArrayRef;
}
unsafe extern "C" {
    pub fn CFNetworkExecuteProxyAutoConfigurationScript(
        proxyAutoConfigurationScript: CFStringRef,
        targetURL: CFURLRef,
        cb: CFProxyAutoConfigurationResultCallback,
        clientContext: *mut CFStreamClientContext,
    ) -> CFRunLoopSourceRef;
}
unsafe extern "C" {
    pub fn CFNetworkExecuteProxyAutoConfigurationURL(
        proxyAutoConfigURL: CFURLRef,
        targetURL: CFURLRef,
        cb: CFProxyAutoConfigurationResultCallback,
        clientContext: *mut CFStreamClientContext,
    ) -> CFRunLoopSourceRef;
}
unsafe extern "C" {
    pub static kCFProxyTypeKey: CFStringRef;
}
unsafe extern "C" {
    pub static kCFProxyHostNameKey: CFStringRef;
}
unsafe extern "C" {
    pub static kCFProxyPortNumberKey: CFStringRef;
}
unsafe extern "C" {
    pub static kCFProxyAutoConfigurationURLKey: CFStringRef;
}
unsafe extern "C" {
    pub static kCFProxyAutoConfigurationJavaScriptKey: CFStringRef;
}
unsafe extern "C" {
    pub static kCFProxyUsernameKey: CFStringRef;
}
unsafe extern "C" {
    pub static kCFProxyPasswordKey: CFStringRef;
}
unsafe extern "C" {
    pub static kCFProxyTypeNone: CFStringRef;
}
unsafe extern "C" {
    pub static kCFProxyTypeHTTP: CFStringRef;
}
unsafe extern "C" {
    pub static kCFProxyTypeHTTPS: CFStringRef;
}
unsafe extern "C" {
    pub static kCFProxyTypeSOCKS: CFStringRef;
}
unsafe extern "C" {
    pub static kCFProxyTypeFTP: CFStringRef;
}
unsafe extern "C" {
    pub static kCFProxyTypeAutoConfigurationURL: CFStringRef;
}
unsafe extern "C" {
    pub static kCFProxyTypeAutoConfigurationJavaScript: CFStringRef;
}
unsafe extern "C" {
    pub static kCFProxyAutoConfigurationHTTPResponseKey: CFStringRef;
}
unsafe extern "C" {
    pub static kCFNetworkProxiesHTTPEnable: CFStringRef;
}
unsafe extern "C" {
    pub static kCFNetworkProxiesHTTPPort: CFStringRef;
}
unsafe extern "C" {
    pub static kCFNetworkProxiesHTTPProxy: CFStringRef;
}
unsafe extern "C" {
    pub static kCFNetworkProxiesProxyAutoConfigEnable: CFStringRef;
}
unsafe extern "C" {
    pub static kCFNetworkProxiesProxyAutoConfigURLString: CFStringRef;
}
unsafe extern "C" {
    pub static kCFNetworkProxiesProxyAutoConfigJavaScript: CFStringRef;
}

unsafe impl objc2::encode::RefEncode for __CFAllocator {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for __CFAllocator {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("__CFAllocator", &[]);
}
unsafe impl objc2::encode::RefEncode for __CFHost {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for __CFHost {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("__CFHost", &[]);
}
unsafe impl objc2::encode::RefEncode for CFHostClientContext {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CFHostClientContext {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("CFHostClientContext", &[]);
}
unsafe impl objc2::encode::RefEncode for __CFNetService {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for __CFNetService {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("__CFNetService", &[]);
}
unsafe impl objc2::encode::RefEncode for __CFNetServiceMonitor {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for __CFNetServiceMonitor {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("__CFNetServiceMonitor", &[]);
}
unsafe impl objc2::encode::RefEncode for __CFNetServiceBrowser {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for __CFNetServiceBrowser {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("__CFNetServiceBrowser", &[]);
}
unsafe impl objc2::encode::RefEncode for CFNetServiceClientContext {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CFNetServiceClientContext {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("CFNetServiceClientContext", &[]);
}
unsafe impl objc2::encode::RefEncode for __CFHTTPMessage {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for __CFHTTPMessage {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("__CFHTTPMessage", &[]);
}
unsafe impl objc2::encode::RefEncode for _CFHTTPAuthentication {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for _CFHTTPAuthentication {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("_CFHTTPAuthentication", &[]);
}
unsafe impl objc2::encode::RefEncode for __CFNetDiagnostic {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for __CFNetDiagnostic {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("__CFNetDiagnostic", &[]);
}
