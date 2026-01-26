#[allow(unused_imports)]
use crate::objc::*;
#[allow(unused_imports)]
use crate::CFNetwork::*;
#[allow(unused_imports)]
use crate::CoreFoundation::*;
#[allow(unused_imports)]
use crate::CoreText::*;
#[allow(unused_imports)]
use libc::dev_t;

#[allow(unused_imports)]
use objc2::msg_send;
pub type UInt64 = ::std::os::raw::c_ulonglong;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __CFUUID {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __CSIdentityAuthority {
    _unused: [u8; 0],
}
pub type CSIdentityAuthorityRef = *mut __CSIdentityAuthority;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __CSIdentity {
    _unused: [u8; 0],
}
pub type CSIdentityRef = *mut __CSIdentity;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __CSIdentityQuery {
    _unused: [u8; 0],
}
pub type CSIdentityQueryRef = *mut __CSIdentityQuery;
pub type CSIdentityClass = CFIndex;
pub type CSIdentityQueryFlags = CFOptionFlags;
pub type CSIdentityQueryStringComparisonMethod = CFIndex;
pub type CSIdentityQueryEvent = CFIndex;
pub type CSIdentityQueryReceiveEventCallback = ::std::option::Option<
    unsafe extern "C" fn(
        query: CSIdentityQueryRef,
        event: CSIdentityQueryEvent,
        identities: CFArrayRef,
        error: CFErrorRef,
        info: *mut ::std::os::raw::c_void,
    ),
>;
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct CSIdentityQueryClientContext {
    pub version: CFIndex,
    pub info: *mut ::std::os::raw::c_void,
    pub retainInfo: CFAllocatorRetainCallBack,
    pub releaseInfo: CFAllocatorReleaseCallBack,
    pub copyInfoDescription: CFAllocatorCopyDescriptionCallBack,
    pub receiveEvent: CSIdentityQueryReceiveEventCallback,
}
pub type FSEventStreamCreateFlags = UInt32;
pub type FSEventStreamEventFlags = UInt32;
pub type FSEventStreamEventId = UInt64;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __FSEventStream {
    _unused: [u8; 0],
}
pub type FSEventStreamRef = *mut __FSEventStream;
pub type ConstFSEventStreamRef = *const __FSEventStream;
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct FSEventStreamContext {
    pub version: CFIndex,
    pub info: *mut ::std::os::raw::c_void,
    pub retain: CFAllocatorRetainCallBack,
    pub release: CFAllocatorReleaseCallBack,
    pub copyDescription: CFAllocatorCopyDescriptionCallBack,
}
pub type FSEventStreamCallback = ::std::option::Option<
    unsafe extern "C" fn(
        streamRef: ConstFSEventStreamRef,
        clientCallBackInfo: *mut ::std::os::raw::c_void,
        numEvents: usize,
        eventPaths: *mut ::std::os::raw::c_void,
        eventFlags: *const FSEventStreamEventFlags,
        eventIds: *const FSEventStreamEventId,
    ),
>;
unsafe extern "C" {
    pub static kCSIdentityErrorDomain: CFStringRef;
}
unsafe extern "C" {
    pub fn CSIdentityAuthorityGetTypeID() -> CFTypeID;
}
unsafe extern "C" {
    pub fn CSIdentityAuthorityCopyLocalizedName(authority: CSIdentityAuthorityRef) -> CFStringRef;
}
unsafe extern "C" {
    pub fn CSIdentityGetTypeID() -> CFTypeID;
}
unsafe extern "C" {
    pub fn CSIdentityCreateCopy(
        allocator: CFAllocatorRef,
        identity: CSIdentityRef,
    ) -> CSIdentityRef;
}
unsafe extern "C" {
    pub fn CSIdentityGetClass(identity: CSIdentityRef) -> CSIdentityClass;
}
unsafe extern "C" {
    pub fn CSIdentityGetAuthority(identity: CSIdentityRef) -> CSIdentityAuthorityRef;
}
unsafe extern "C" {
    pub fn CSIdentityGetFullName(identity: CSIdentityRef) -> CFStringRef;
}
unsafe extern "C" {
    pub fn CSIdentityGetPosixName(identity: CSIdentityRef) -> CFStringRef;
}
unsafe extern "C" {
    pub fn CSIdentityCreatePersistentReference(
        allocator: CFAllocatorRef,
        identity: CSIdentityRef,
    ) -> CFDataRef;
}
unsafe extern "C" {
    pub fn CSIdentityQueryGetTypeID() -> CFTypeID;
}
unsafe extern "C" {
    pub fn CSIdentityQueryCreateForName(
        allocator: CFAllocatorRef,
        name: CFStringRef,
        comparisonMethod: CSIdentityQueryStringComparisonMethod,
        identityClass: CSIdentityClass,
        authority: CSIdentityAuthorityRef,
    ) -> CSIdentityQueryRef;
}
unsafe extern "C" {
    pub fn CSIdentityQueryCreateForPersistentReference(
        allocator: CFAllocatorRef,
        referenceData: CFDataRef,
    ) -> CSIdentityQueryRef;
}
unsafe extern "C" {
    pub fn CSIdentityQueryCopyResults(query: CSIdentityQueryRef) -> CFArrayRef;
}
unsafe extern "C" {
    pub fn CSIdentityQueryExecute(
        query: CSIdentityQueryRef,
        flags: CSIdentityQueryFlags,
        error: *mut CFErrorRef,
    ) -> Boolean;
}
unsafe extern "C" {
    pub fn CSIdentityQueryExecuteAsynchronously(
        query: CSIdentityQueryRef,
        flags: CSIdentityQueryFlags,
        clientContext: *const CSIdentityQueryClientContext,
        runLoop: CFRunLoopRef,
        runLoopMode: CFStringRef,
    ) -> Boolean;
}
unsafe extern "C" {
    pub fn CSIdentityQueryStop(query: CSIdentityQueryRef);
}
unsafe extern "C" {
    pub static kUTTypeItem: CFStringRef;
}
unsafe extern "C" {
    pub static kUTTypeContent: CFStringRef;
}
unsafe extern "C" {
    pub static kUTTypeCompositeContent: CFStringRef;
}
unsafe extern "C" {
    pub static kUTTypeMessage: CFStringRef;
}
unsafe extern "C" {
    pub static kUTTypeContact: CFStringRef;
}
unsafe extern "C" {
    pub static kUTTypeArchive: CFStringRef;
}
unsafe extern "C" {
    pub static kUTTypeDiskImage: CFStringRef;
}
unsafe extern "C" {
    pub static kUTTypeAliasRecord: CFStringRef;
}
unsafe extern "C" {
    pub static kUTTypeData: CFStringRef;
}
unsafe extern "C" {
    pub static kUTTypeDirectory: CFStringRef;
}
unsafe extern "C" {
    pub static kUTTypeResolvable: CFStringRef;
}
unsafe extern "C" {
    pub static kUTTypeSymLink: CFStringRef;
}
unsafe extern "C" {
    pub static kUTTypeExecutable: CFStringRef;
}
unsafe extern "C" {
    pub static kUTTypeMountPoint: CFStringRef;
}
unsafe extern "C" {
    pub static kUTTypeAliasFile: CFStringRef;
}
unsafe extern "C" {
    pub static kUTTypeURLBookmarkData: CFStringRef;
}
unsafe extern "C" {
    pub static kUTTypeURL: CFStringRef;
}
unsafe extern "C" {
    pub static kUTTypeFileURL: CFStringRef;
}
unsafe extern "C" {
    pub static kUTTypeText: CFStringRef;
}
unsafe extern "C" {
    pub static kUTTypePlainText: CFStringRef;
}
unsafe extern "C" {
    pub static kUTTypeUTF8PlainText: CFStringRef;
}
unsafe extern "C" {
    pub static kUTTypeUTF16ExternalPlainText: CFStringRef;
}
unsafe extern "C" {
    pub static kUTTypeUTF16PlainText: CFStringRef;
}
unsafe extern "C" {
    pub static kUTTypeDelimitedText: CFStringRef;
}
unsafe extern "C" {
    pub static kUTTypeCommaSeparatedText: CFStringRef;
}
unsafe extern "C" {
    pub static kUTTypeTabSeparatedText: CFStringRef;
}
unsafe extern "C" {
    pub static kUTTypeUTF8TabSeparatedText: CFStringRef;
}
unsafe extern "C" {
    pub static kUTTypeRTF: CFStringRef;
}
unsafe extern "C" {
    pub static kUTTypeHTML: CFStringRef;
}
unsafe extern "C" {
    pub static kUTTypeXML: CFStringRef;
}
unsafe extern "C" {
    pub static kUTTypeJavaSource: CFStringRef;
}
unsafe extern "C" {
    pub static kUTTypeSourceCode: CFStringRef;
}
unsafe extern "C" {
    pub static kUTTypeAssemblyLanguageSource: CFStringRef;
}
unsafe extern "C" {
    pub static kUTTypeCSource: CFStringRef;
}
unsafe extern "C" {
    pub static kUTTypeObjectiveCSource: CFStringRef;
}
unsafe extern "C" {
    pub static kUTTypeSwiftSource: CFStringRef;
}
unsafe extern "C" {
    pub static kUTTypeCPlusPlusSource: CFStringRef;
}
unsafe extern "C" {
    pub static kUTTypeObjectiveCPlusPlusSource: CFStringRef;
}
unsafe extern "C" {
    pub static kUTTypeCHeader: CFStringRef;
}
unsafe extern "C" {
    pub static kUTTypeCPlusPlusHeader: CFStringRef;
}
unsafe extern "C" {
    pub static kUTTypeScript: CFStringRef;
}
unsafe extern "C" {
    pub static kUTTypeAppleScript: CFStringRef;
}
unsafe extern "C" {
    pub static kUTTypeOSAScript: CFStringRef;
}
unsafe extern "C" {
    pub static kUTTypeOSAScriptBundle: CFStringRef;
}
unsafe extern "C" {
    pub static kUTTypeJavaScript: CFStringRef;
}
unsafe extern "C" {
    pub static kUTTypeShellScript: CFStringRef;
}
unsafe extern "C" {
    pub static kUTTypePerlScript: CFStringRef;
}
unsafe extern "C" {
    pub static kUTTypePythonScript: CFStringRef;
}
unsafe extern "C" {
    pub static kUTTypeRubyScript: CFStringRef;
}
unsafe extern "C" {
    pub static kUTTypePHPScript: CFStringRef;
}
unsafe extern "C" {
    pub static kUTTypeJSON: CFStringRef;
}
unsafe extern "C" {
    pub static kUTTypePropertyList: CFStringRef;
}
unsafe extern "C" {
    pub static kUTTypeXMLPropertyList: CFStringRef;
}
unsafe extern "C" {
    pub static kUTTypeBinaryPropertyList: CFStringRef;
}
unsafe extern "C" {
    pub static kUTTypeTXNTextAndMultimediaData: CFStringRef;
}
unsafe extern "C" {
    pub static kUTTypePDF: CFStringRef;
}
unsafe extern "C" {
    pub static kUTTypeRTFD: CFStringRef;
}
unsafe extern "C" {
    pub static kUTTypeFlatRTFD: CFStringRef;
}
unsafe extern "C" {
    pub static kUTTypeWebArchive: CFStringRef;
}
unsafe extern "C" {
    pub static kUTTypeJPEG2000: CFStringRef;
}
unsafe extern "C" {
    pub static kUTTypeQuickTimeImage: CFStringRef;
}
unsafe extern "C" {
    pub static kUTTypeImage: CFStringRef;
}
unsafe extern "C" {
    pub static kUTTypeJPEG: CFStringRef;
}
unsafe extern "C" {
    pub static kUTTypeTIFF: CFStringRef;
}
unsafe extern "C" {
    pub static kUTTypePICT: CFStringRef;
}
unsafe extern "C" {
    pub static kUTTypeGIF: CFStringRef;
}
unsafe extern "C" {
    pub static kUTTypePNG: CFStringRef;
}
unsafe extern "C" {
    pub static kUTTypeAppleICNS: CFStringRef;
}
unsafe extern "C" {
    pub static kUTTypeBMP: CFStringRef;
}
unsafe extern "C" {
    pub static kUTTypeICO: CFStringRef;
}
unsafe extern "C" {
    pub static kUTTypeRawImage: CFStringRef;
}
unsafe extern "C" {
    pub static kUTTypeScalableVectorGraphics: CFStringRef;
}
unsafe extern "C" {
    pub static kUTTypeLivePhoto: CFStringRef;
}
unsafe extern "C" {
    pub static kUTTypeAudiovisualContent: CFStringRef;
}
unsafe extern "C" {
    pub static kUTTypeMovie: CFStringRef;
}
unsafe extern "C" {
    pub static kUTTypeVideo: CFStringRef;
}
unsafe extern "C" {
    pub static kUTTypeAudio: CFStringRef;
}
unsafe extern "C" {
    pub static kUTTypeQuickTimeMovie: CFStringRef;
}
unsafe extern "C" {
    pub static kUTTypeMPEG: CFStringRef;
}
unsafe extern "C" {
    pub static kUTTypeMPEG2Video: CFStringRef;
}
unsafe extern "C" {
    pub static kUTTypeMPEG2TransportStream: CFStringRef;
}
unsafe extern "C" {
    pub static kUTTypeMP3: CFStringRef;
}
unsafe extern "C" {
    pub static kUTTypeMPEG4: CFStringRef;
}
unsafe extern "C" {
    pub static kUTTypeMPEG4Audio: CFStringRef;
}
unsafe extern "C" {
    pub static kUTTypeAppleProtectedMPEG4Audio: CFStringRef;
}
unsafe extern "C" {
    pub static kUTTypeAppleProtectedMPEG4Video: CFStringRef;
}
unsafe extern "C" {
    pub static kUTTypeAVIMovie: CFStringRef;
}
unsafe extern "C" {
    pub static kUTTypeAudioInterchangeFileFormat: CFStringRef;
}
unsafe extern "C" {
    pub static kUTTypeWaveformAudio: CFStringRef;
}
unsafe extern "C" {
    pub static kUTTypeMIDIAudio: CFStringRef;
}
unsafe extern "C" {
    pub static kUTTypePlaylist: CFStringRef;
}
unsafe extern "C" {
    pub static kUTTypeM3UPlaylist: CFStringRef;
}
unsafe extern "C" {
    pub static kUTTypeFolder: CFStringRef;
}
unsafe extern "C" {
    pub static kUTTypeVolume: CFStringRef;
}
unsafe extern "C" {
    pub static kUTTypePackage: CFStringRef;
}
unsafe extern "C" {
    pub static kUTTypeBundle: CFStringRef;
}
unsafe extern "C" {
    pub static kUTTypePluginBundle: CFStringRef;
}
unsafe extern "C" {
    pub static kUTTypeSpotlightImporter: CFStringRef;
}
unsafe extern "C" {
    pub static kUTTypeQuickLookGenerator: CFStringRef;
}
unsafe extern "C" {
    pub static kUTTypeXPCService: CFStringRef;
}
unsafe extern "C" {
    pub static kUTTypeFramework: CFStringRef;
}
unsafe extern "C" {
    pub static kUTTypeApplication: CFStringRef;
}
unsafe extern "C" {
    pub static kUTTypeApplicationBundle: CFStringRef;
}
unsafe extern "C" {
    pub static kUTTypeApplicationFile: CFStringRef;
}
unsafe extern "C" {
    pub static kUTTypeUnixExecutable: CFStringRef;
}
unsafe extern "C" {
    pub static kUTTypeJavaClass: CFStringRef;
}
unsafe extern "C" {
    pub static kUTTypeJavaArchive: CFStringRef;
}
unsafe extern "C" {
    pub static kUTTypeWindowsExecutable: CFStringRef;
}
unsafe extern "C" {
    pub static kUTTypeSystemPreferencesPane: CFStringRef;
}
unsafe extern "C" {
    pub static kUTTypeGNUZipArchive: CFStringRef;
}
unsafe extern "C" {
    pub static kUTTypeBzip2Archive: CFStringRef;
}
unsafe extern "C" {
    pub static kUTTypeZipArchive: CFStringRef;
}
unsafe extern "C" {
    pub static kUTTypeSpreadsheet: CFStringRef;
}
unsafe extern "C" {
    pub static kUTTypePresentation: CFStringRef;
}
unsafe extern "C" {
    pub static kUTTypeDatabase: CFStringRef;
}
unsafe extern "C" {
    pub static kUTTypeVCard: CFStringRef;
}
unsafe extern "C" {
    pub static kUTTypeToDoItem: CFStringRef;
}
unsafe extern "C" {
    pub static kUTTypeCalendarEvent: CFStringRef;
}
unsafe extern "C" {
    pub static kUTTypeEmailMessage: CFStringRef;
}
unsafe extern "C" {
    pub static kUTTypeInternetLocation: CFStringRef;
}
unsafe extern "C" {
    pub static kUTTypeInkText: CFStringRef;
}
unsafe extern "C" {
    pub static kUTTypeFont: CFStringRef;
}
unsafe extern "C" {
    pub static kUTTypeBookmark: CFStringRef;
}
unsafe extern "C" {
    pub static kUTType3DContent: CFStringRef;
}
unsafe extern "C" {
    pub static kUTTypePKCS12: CFStringRef;
}
unsafe extern "C" {
    pub static kUTTypeX509Certificate: CFStringRef;
}
unsafe extern "C" {
    pub static kUTTypeElectronicPublication: CFStringRef;
}
unsafe extern "C" {
    pub static kUTTypeLog: CFStringRef;
}
unsafe extern "C" {
    pub static kUTExportedTypeDeclarationsKey: CFStringRef;
}
unsafe extern "C" {
    pub static kUTImportedTypeDeclarationsKey: CFStringRef;
}
unsafe extern "C" {
    pub static kUTTypeIdentifierKey: CFStringRef;
}
unsafe extern "C" {
    pub static kUTTypeTagSpecificationKey: CFStringRef;
}
unsafe extern "C" {
    pub static kUTTypeConformsToKey: CFStringRef;
}
unsafe extern "C" {
    pub static kUTTypeDescriptionKey: CFStringRef;
}
unsafe extern "C" {
    pub static kUTTypeIconFileKey: CFStringRef;
}
unsafe extern "C" {
    pub static kUTTypeReferenceURLKey: CFStringRef;
}
unsafe extern "C" {
    pub static kUTTypeVersionKey: CFStringRef;
}
unsafe extern "C" {
    pub static kUTTagClassFilenameExtension: CFStringRef;
}
unsafe extern "C" {
    pub static kUTTagClassMIMEType: CFStringRef;
}
unsafe extern "C" {
    pub fn UTTypeCreatePreferredIdentifierForTag(
        inTagClass: CFStringRef,
        inTag: CFStringRef,
        inConformingToUTI: CFStringRef,
    ) -> CFStringRef;
}
unsafe extern "C" {
    pub fn UTTypeCreateAllIdentifiersForTag(
        inTagClass: CFStringRef,
        inTag: CFStringRef,
        inConformingToUTI: CFStringRef,
    ) -> CFArrayRef;
}
unsafe extern "C" {
    pub fn UTTypeCopyPreferredTagWithClass(
        inUTI: CFStringRef,
        inTagClass: CFStringRef,
    ) -> CFStringRef;
}
unsafe extern "C" {
    pub fn UTTypeCopyAllTagsWithClass(inUTI: CFStringRef, inTagClass: CFStringRef) -> CFArrayRef;
}
unsafe extern "C" {
    pub fn UTTypeEqual(inUTI1: CFStringRef, inUTI2: CFStringRef) -> Boolean;
}
unsafe extern "C" {
    pub fn UTTypeConformsTo(inUTI: CFStringRef, inConformsToUTI: CFStringRef) -> Boolean;
}
unsafe extern "C" {
    pub fn UTTypeCopyDescription(inUTI: CFStringRef) -> CFStringRef;
}
unsafe extern "C" {
    pub fn UTTypeIsDeclared(inUTI: CFStringRef) -> Boolean;
}
unsafe extern "C" {
    pub fn UTTypeIsDynamic(inUTI: CFStringRef) -> Boolean;
}
unsafe extern "C" {
    pub fn UTTypeCopyDeclaration(inUTI: CFStringRef) -> CFDictionaryRef;
}
unsafe extern "C" {
    pub fn UTTypeCopyDeclaringBundleURL(inUTI: CFStringRef) -> CFURLRef;
}
unsafe extern "C" {
    pub fn FSEventStreamCreate(
        allocator: CFAllocatorRef,
        callback: FSEventStreamCallback,
        context: *mut FSEventStreamContext,
        pathsToWatch: CFArrayRef,
        sinceWhen: FSEventStreamEventId,
        latency: CFTimeInterval,
        flags: FSEventStreamCreateFlags,
    ) -> FSEventStreamRef;
}
unsafe extern "C" {
    pub fn FSEventStreamCreateRelativeToDevice(
        allocator: CFAllocatorRef,
        callback: FSEventStreamCallback,
        context: *mut FSEventStreamContext,
        deviceToWatch: dev_t,
        pathsToWatchRelativeToDevice: CFArrayRef,
        sinceWhen: FSEventStreamEventId,
        latency: CFTimeInterval,
        flags: FSEventStreamCreateFlags,
    ) -> FSEventStreamRef;
}
unsafe extern "C" {
    pub fn FSEventStreamGetLatestEventId(streamRef: ConstFSEventStreamRef) -> FSEventStreamEventId;
}
unsafe extern "C" {
    pub fn FSEventStreamGetDeviceBeingWatched(streamRef: ConstFSEventStreamRef) -> dev_t;
}
unsafe extern "C" {
    pub fn FSEventStreamCopyPathsBeingWatched(streamRef: ConstFSEventStreamRef) -> CFArrayRef;
}
unsafe extern "C" {
    pub fn FSEventsGetCurrentEventId() -> FSEventStreamEventId;
}
unsafe extern "C" {
    pub fn FSEventsCopyUUIDForDevice(dev: dev_t) -> CFUUIDRef;
}
unsafe extern "C" {
    pub fn FSEventsGetLastEventIdForDeviceBeforeTime(
        dev: dev_t,
        time: CFAbsoluteTime,
    ) -> FSEventStreamEventId;
}
unsafe extern "C" {
    pub fn FSEventsPurgeEventsForDeviceUpToEventId(
        dev: dev_t,
        eventId: FSEventStreamEventId,
    ) -> Boolean;
}
unsafe extern "C" {
    pub fn FSEventStreamRetain(streamRef: FSEventStreamRef);
}
unsafe extern "C" {
    pub fn FSEventStreamRelease(streamRef: FSEventStreamRef);
}
unsafe extern "C" {
    pub fn FSEventStreamScheduleWithRunLoop(
        streamRef: FSEventStreamRef,
        runLoop: CFRunLoopRef,
        runLoopMode: CFStringRef,
    );
}
unsafe extern "C" {
    pub fn FSEventStreamUnscheduleFromRunLoop(
        streamRef: FSEventStreamRef,
        runLoop: CFRunLoopRef,
        runLoopMode: CFStringRef,
    );
}
unsafe extern "C" {
    pub fn FSEventStreamSetDispatchQueue(streamRef: FSEventStreamRef, q: NSObject);
}
unsafe extern "C" {
    pub fn FSEventStreamInvalidate(streamRef: FSEventStreamRef);
}
unsafe extern "C" {
    pub fn FSEventStreamStart(streamRef: FSEventStreamRef) -> Boolean;
}
unsafe extern "C" {
    pub fn FSEventStreamFlushAsync(streamRef: FSEventStreamRef) -> FSEventStreamEventId;
}
unsafe extern "C" {
    pub fn FSEventStreamFlushSync(streamRef: FSEventStreamRef);
}
unsafe extern "C" {
    pub fn FSEventStreamStop(streamRef: FSEventStreamRef);
}
unsafe extern "C" {
    pub fn FSEventStreamShow(streamRef: ConstFSEventStreamRef);
}
unsafe extern "C" {
    pub fn FSEventStreamCopyDescription(streamRef: ConstFSEventStreamRef) -> CFStringRef;
}
unsafe extern "C" {
    pub fn FSEventStreamSetExclusionPaths(
        streamRef: FSEventStreamRef,
        pathsToExclude: CFArrayRef,
    ) -> Boolean;
}

unsafe impl objc2::encode::RefEncode for __CFUUID {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for __CFUUID {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("__CFUUID", &[]);
}
unsafe impl objc2::encode::RefEncode for __CSIdentityAuthority {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for __CSIdentityAuthority {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("__CSIdentityAuthority", &[]);
}
unsafe impl objc2::encode::RefEncode for __CSIdentity {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for __CSIdentity {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("__CSIdentity", &[]);
}
unsafe impl objc2::encode::RefEncode for __CSIdentityQuery {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for __CSIdentityQuery {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("__CSIdentityQuery", &[]);
}
unsafe impl objc2::encode::RefEncode for CSIdentityQueryClientContext {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CSIdentityQueryClientContext {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("CSIdentityQueryClientContext", &[]);
}
unsafe impl objc2::encode::RefEncode for __FSEventStream {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for __FSEventStream {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("__FSEventStream", &[]);
}
unsafe impl objc2::encode::RefEncode for FSEventStreamContext {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for FSEventStreamContext {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("FSEventStreamContext", &[]);
}
