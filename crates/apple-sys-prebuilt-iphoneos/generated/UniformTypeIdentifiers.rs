#[allow(unused_imports)]
use crate::objc::*;
#[allow(unused_imports)]
use crate::Foundation::*;
#[allow(unused_imports)]
use crate::Security::*;

#[allow(unused_imports)]
use objc2::msg_send;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct UTType(pub id);
impl std::ops::Deref for UTType {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for UTType {}
impl UTType {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"UTType").unwrap(), alloc) })
    }
}
impl PNSCopying for UTType {}
impl PNSSecureCoding for UTType {}
impl INSObject for UTType {}
impl PNSObject for UTType {}
impl std::convert::TryFrom<NSObject> for UTType {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<UTType, Self::Error> {
        let is_kind_of: bool = unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"UTType").unwrap()) };
        if is_kind_of {
            Ok(UTType(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to UTType")
        }
    }
}
impl IUTType for UTType {}
pub trait IUTType: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn identifier(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, identifier)
    }
    unsafe fn preferredFilenameExtension(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, preferredFilenameExtension)
    }
    unsafe fn preferredMIMEType(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, preferredMIMEType)
    }
    unsafe fn localizedDescription(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, localizedDescription)
    }
    unsafe fn version(&self) -> NSNumber
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, version)
    }
    unsafe fn referenceURL(&self) -> NSURL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, referenceURL)
    }
    unsafe fn isDynamic(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isDynamic)
    }
    unsafe fn isDeclared(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isDeclared)
    }
    unsafe fn isPublicType(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isPublicType)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"UTType").unwrap(), new)
    }
    unsafe fn typeWithIdentifier_(identifier: NSString) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"UTType").unwrap(), typeWithIdentifier : identifier)
    }
    unsafe fn typeWithFilenameExtension_(filenameExtension: NSString) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"UTType").unwrap(), typeWithFilenameExtension : filenameExtension)
    }
    unsafe fn typeWithFilenameExtension_conformingToType_(
        filenameExtension: NSString,
        supertype: UTType,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"UTType").unwrap(), typeWithFilenameExtension : filenameExtension, conformingToType : supertype)
    }
    unsafe fn typeWithMIMEType_(mimeType: NSString) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"UTType").unwrap(), typeWithMIMEType : mimeType)
    }
    unsafe fn typeWithMIMEType_conformingToType_(
        mimeType: NSString,
        supertype: UTType,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"UTType").unwrap(), typeWithMIMEType : mimeType, conformingToType : supertype)
    }
}
impl UTType_Conformance for UTType {}
pub trait UTType_Conformance: Sized + std::ops::Deref {
    unsafe fn conformsToType_(&self, type_: UTType) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, conformsToType : type_)
    }
    unsafe fn isSupertypeOfType_(&self, type_: UTType) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, isSupertypeOfType : type_)
    }
    unsafe fn isSubtypeOfType_(&self, type_: UTType) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, isSubtypeOfType : type_)
    }
    unsafe fn supertypes(&self) -> NSSet
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, supertypes)
    }
}
impl UTType_UTTagSpecification for UTType {}
pub trait UTType_UTTagSpecification: Sized + std::ops::Deref {
    unsafe fn tags(&self) -> NSDictionary
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, tags)
    }
    unsafe fn typeWithTag_tagClass_conformingToType_(
        tag: NSString,
        tagClass: NSString,
        supertype: UTType,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"UTType").unwrap(), typeWithTag : tag, tagClass : tagClass, conformingToType : supertype)
    }
    unsafe fn typesWithTag_tagClass_conformingToType_(
        tag: NSString,
        tagClass: NSString,
        supertype: UTType,
    ) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"UTType").unwrap(), typesWithTag : tag, tagClass : tagClass, conformingToType : supertype)
    }
}
impl UTType_LocalConstants for UTType {}
pub trait UTType_LocalConstants: Sized + std::ops::Deref {
    unsafe fn exportedTypeWithIdentifier_(identifier: NSString) -> UTType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"UTType").unwrap(), exportedTypeWithIdentifier : identifier)
    }
    unsafe fn exportedTypeWithIdentifier_conformingToType_(
        identifier: NSString,
        parentType: UTType,
    ) -> UTType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"UTType").unwrap(), exportedTypeWithIdentifier : identifier, conformingToType : parentType)
    }
    unsafe fn importedTypeWithIdentifier_(identifier: NSString) -> UTType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"UTType").unwrap(), importedTypeWithIdentifier : identifier)
    }
    unsafe fn importedTypeWithIdentifier_conformingToType_(
        identifier: NSString,
        parentType: UTType,
    ) -> UTType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"UTType").unwrap(), importedTypeWithIdentifier : identifier, conformingToType : parentType)
    }
}
pub trait NSString_UTAdditions: Sized + std::ops::Deref {
    unsafe fn stringByAppendingPathComponent_conformingToType_(
        &self,
        partialName: NSString,
        contentType: UTType,
    ) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, stringByAppendingPathComponent : partialName, conformingToType : contentType)
    }
    unsafe fn stringByAppendingPathExtensionForType_(&self, contentType: UTType) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, stringByAppendingPathExtensionForType : contentType)
    }
}
pub trait NSURL_UTAdditions: Sized + std::ops::Deref {
    unsafe fn URLByAppendingPathComponent_conformingToType_(
        &self,
        partialName: NSString,
        contentType: UTType,
    ) -> NSURL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, URLByAppendingPathComponent : partialName, conformingToType : contentType)
    }
    unsafe fn URLByAppendingPathExtensionForType_(&self, contentType: UTType) -> NSURL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, URLByAppendingPathExtensionForType : contentType)
    }
}
pub trait NSItemProvider_UTType: Sized + std::ops::Deref {
    unsafe fn initWithContentsOfURL_contentType_openInPlace_coordinated_visibility_(
        &self,
        fileURL: NSURL,
        contentType: UTType,
        openInPlace: BOOL,
        coordinated: BOOL,
        visibility: NSItemProviderRepresentationVisibility,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithContentsOfURL : fileURL, contentType : contentType, openInPlace : openInPlace, coordinated : coordinated, visibility : visibility)
    }
    unsafe fn registerDataRepresentationForContentType_visibility_loadHandler_(
        &self,
        contentType: UTType,
        visibility: NSItemProviderRepresentationVisibility,
        loadHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, registerDataRepresentationForContentType : contentType, visibility : visibility, loadHandler : loadHandler)
    }
    unsafe fn registerFileRepresentationForContentType_visibility_openInPlace_loadHandler_(
        &self,
        contentType: UTType,
        visibility: NSItemProviderRepresentationVisibility,
        openInPlace: BOOL,
        loadHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, registerFileRepresentationForContentType : contentType, visibility : visibility, openInPlace : openInPlace, loadHandler : loadHandler)
    }
    unsafe fn registeredContentTypesConformingToContentType_(&self, contentType: UTType) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, registeredContentTypesConformingToContentType : contentType)
    }
    unsafe fn loadDataRepresentationForContentType_completionHandler_(
        &self,
        contentType: UTType,
        completionHandler: *mut ::std::os::raw::c_void,
    ) -> NSProgress
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, loadDataRepresentationForContentType : contentType, completionHandler : completionHandler)
    }
    unsafe fn loadFileRepresentationForContentType_openInPlace_completionHandler_(
        &self,
        contentType: UTType,
        openInPlace: BOOL,
        completionHandler: *mut ::std::os::raw::c_void,
    ) -> NSProgress
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, loadFileRepresentationForContentType : contentType, openInPlace : openInPlace, completionHandler : completionHandler)
    }
    unsafe fn registeredContentTypes(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, registeredContentTypes)
    }
    unsafe fn registeredContentTypesForOpenInPlace(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, registeredContentTypesForOpenInPlace)
    }
}
unsafe extern "C" {
    pub static UTTagClassFilenameExtension: NSString;
}
unsafe extern "C" {
    pub static UTTagClassMIMEType: NSString;
}
unsafe extern "C" {
    pub static UTTypeItem: UTType;
}
unsafe extern "C" {
    pub static UTTypeContent: UTType;
}
unsafe extern "C" {
    pub static UTTypeCompositeContent: UTType;
}
unsafe extern "C" {
    pub static UTTypeDiskImage: UTType;
}
unsafe extern "C" {
    pub static UTTypeData: UTType;
}
unsafe extern "C" {
    pub static UTTypeDirectory: UTType;
}
unsafe extern "C" {
    pub static UTTypeResolvable: UTType;
}
unsafe extern "C" {
    pub static UTTypeSymbolicLink: UTType;
}
unsafe extern "C" {
    pub static UTTypeExecutable: UTType;
}
unsafe extern "C" {
    pub static UTTypeMountPoint: UTType;
}
unsafe extern "C" {
    pub static UTTypeAliasFile: UTType;
}
unsafe extern "C" {
    pub static UTTypeURLBookmarkData: UTType;
}
unsafe extern "C" {
    pub static UTTypeURL: UTType;
}
unsafe extern "C" {
    pub static UTTypeFileURL: UTType;
}
unsafe extern "C" {
    pub static UTTypeText: UTType;
}
unsafe extern "C" {
    pub static UTTypePlainText: UTType;
}
unsafe extern "C" {
    pub static UTTypeUTF8PlainText: UTType;
}
unsafe extern "C" {
    pub static UTTypeUTF16ExternalPlainText: UTType;
}
unsafe extern "C" {
    pub static UTTypeUTF16PlainText: UTType;
}
unsafe extern "C" {
    pub static UTTypeDelimitedText: UTType;
}
unsafe extern "C" {
    pub static UTTypeCommaSeparatedText: UTType;
}
unsafe extern "C" {
    pub static UTTypeTabSeparatedText: UTType;
}
unsafe extern "C" {
    pub static UTTypeUTF8TabSeparatedText: UTType;
}
unsafe extern "C" {
    pub static UTTypeRTF: UTType;
}
unsafe extern "C" {
    pub static UTTypeHTML: UTType;
}
unsafe extern "C" {
    pub static UTTypeXML: UTType;
}
unsafe extern "C" {
    pub static UTTypeYAML: UTType;
}
unsafe extern "C" {
    pub static UTTypeCSS: UTType;
}
unsafe extern "C" {
    pub static UTTypeSourceCode: UTType;
}
unsafe extern "C" {
    pub static UTTypeAssemblyLanguageSource: UTType;
}
unsafe extern "C" {
    pub static UTTypeCSource: UTType;
}
unsafe extern "C" {
    pub static UTTypeObjectiveCSource: UTType;
}
unsafe extern "C" {
    pub static UTTypeSwiftSource: UTType;
}
unsafe extern "C" {
    pub static UTTypeCPlusPlusSource: UTType;
}
unsafe extern "C" {
    pub static UTTypeObjectiveCPlusPlusSource: UTType;
}
unsafe extern "C" {
    pub static UTTypeCHeader: UTType;
}
unsafe extern "C" {
    pub static UTTypeCPlusPlusHeader: UTType;
}
unsafe extern "C" {
    pub static UTTypeScript: UTType;
}
unsafe extern "C" {
    pub static UTTypeAppleScript: UTType;
}
unsafe extern "C" {
    pub static UTTypeOSAScript: UTType;
}
unsafe extern "C" {
    pub static UTTypeOSAScriptBundle: UTType;
}
unsafe extern "C" {
    pub static UTTypeJavaScript: UTType;
}
unsafe extern "C" {
    pub static UTTypeShellScript: UTType;
}
unsafe extern "C" {
    pub static UTTypePerlScript: UTType;
}
unsafe extern "C" {
    pub static UTTypePythonScript: UTType;
}
unsafe extern "C" {
    pub static UTTypeRubyScript: UTType;
}
unsafe extern "C" {
    pub static UTTypePHPScript: UTType;
}
unsafe extern "C" {
    pub static UTTypeMakefile: UTType;
}
unsafe extern "C" {
    pub static UTTypeJSON: UTType;
}
unsafe extern "C" {
    pub static UTTypePropertyList: UTType;
}
unsafe extern "C" {
    pub static UTTypeXMLPropertyList: UTType;
}
unsafe extern "C" {
    pub static UTTypeBinaryPropertyList: UTType;
}
unsafe extern "C" {
    pub static UTTypePDF: UTType;
}
unsafe extern "C" {
    pub static UTTypeRTFD: UTType;
}
unsafe extern "C" {
    pub static UTTypeFlatRTFD: UTType;
}
unsafe extern "C" {
    pub static UTTypeWebArchive: UTType;
}
unsafe extern "C" {
    pub static UTTypeImage: UTType;
}
unsafe extern "C" {
    pub static UTTypeJPEG: UTType;
}
unsafe extern "C" {
    pub static UTTypeTIFF: UTType;
}
unsafe extern "C" {
    pub static UTTypeGIF: UTType;
}
unsafe extern "C" {
    pub static UTTypePNG: UTType;
}
unsafe extern "C" {
    pub static UTTypeICNS: UTType;
}
unsafe extern "C" {
    pub static UTTypeBMP: UTType;
}
unsafe extern "C" {
    pub static UTTypeICO: UTType;
}
unsafe extern "C" {
    pub static UTTypeRAWImage: UTType;
}
unsafe extern "C" {
    pub static UTTypeSVG: UTType;
}
unsafe extern "C" {
    pub static UTTypeLivePhoto: UTType;
}
unsafe extern "C" {
    pub static UTTypeHEIF: UTType;
}
unsafe extern "C" {
    pub static UTTypeHEIC: UTType;
}
unsafe extern "C" {
    pub static UTTypeHEICS: UTType;
}
unsafe extern "C" {
    pub static UTTypeWebP: UTType;
}
unsafe extern "C" {
    pub static UTTypeEXR: UTType;
}
unsafe extern "C" {
    pub static UTTypeDNG: UTType;
}
unsafe extern "C" {
    pub static UTTypeJPEGXL: UTType;
}
unsafe extern "C" {
    pub static UTType3DContent: UTType;
}
unsafe extern "C" {
    pub static UTTypeUSD: UTType;
}
unsafe extern "C" {
    pub static UTTypeUSDZ: UTType;
}
unsafe extern "C" {
    pub static UTTypeRealityFile: UTType;
}
unsafe extern "C" {
    pub static UTTypeSceneKitScene: UTType;
}
unsafe extern "C" {
    pub static UTTypeARReferenceObject: UTType;
}
unsafe extern "C" {
    pub static UTTypeAudiovisualContent: UTType;
}
unsafe extern "C" {
    pub static UTTypeMovie: UTType;
}
unsafe extern "C" {
    pub static UTTypeVideo: UTType;
}
unsafe extern "C" {
    pub static UTTypeAudio: UTType;
}
unsafe extern "C" {
    pub static UTTypeQuickTimeMovie: UTType;
}
unsafe extern "C" {
    pub static UTTypeMPEG: UTType;
}
unsafe extern "C" {
    pub static UTTypeMPEG2Video: UTType;
}
unsafe extern "C" {
    pub static UTTypeMPEG2TransportStream: UTType;
}
unsafe extern "C" {
    pub static UTTypeMP3: UTType;
}
unsafe extern "C" {
    pub static UTTypeMPEG4Movie: UTType;
}
unsafe extern "C" {
    pub static UTTypeMPEG4Audio: UTType;
}
unsafe extern "C" {
    pub static UTTypeAppleProtectedMPEG4Audio: UTType;
}
unsafe extern "C" {
    pub static UTTypeAppleProtectedMPEG4Video: UTType;
}
unsafe extern "C" {
    pub static UTTypeAVI: UTType;
}
unsafe extern "C" {
    pub static UTTypeAIFF: UTType;
}
unsafe extern "C" {
    pub static UTTypeWAV: UTType;
}
unsafe extern "C" {
    pub static UTTypeMIDI: UTType;
}
unsafe extern "C" {
    pub static UTTypePlaylist: UTType;
}
unsafe extern "C" {
    pub static UTTypeM3UPlaylist: UTType;
}
unsafe extern "C" {
    pub static UTTypeFolder: UTType;
}
unsafe extern "C" {
    pub static UTTypeVolume: UTType;
}
unsafe extern "C" {
    pub static UTTypePackage: UTType;
}
unsafe extern "C" {
    pub static UTTypeBundle: UTType;
}
unsafe extern "C" {
    pub static UTTypePluginBundle: UTType;
}
unsafe extern "C" {
    pub static UTTypeSpotlightImporter: UTType;
}
unsafe extern "C" {
    pub static UTTypeQuickLookGenerator: UTType;
}
unsafe extern "C" {
    pub static UTTypeXPCService: UTType;
}
unsafe extern "C" {
    pub static UTTypeFramework: UTType;
}
unsafe extern "C" {
    pub static UTTypeApplication: UTType;
}
unsafe extern "C" {
    pub static UTTypeApplicationBundle: UTType;
}
unsafe extern "C" {
    pub static UTTypeApplicationExtension: UTType;
}
unsafe extern "C" {
    pub static UTTypeUnixExecutable: UTType;
}
unsafe extern "C" {
    pub static UTTypeEXE: UTType;
}
unsafe extern "C" {
    pub static UTTypeSystemPreferencesPane: UTType;
}
unsafe extern "C" {
    pub static UTTypeArchive: UTType;
}
unsafe extern "C" {
    pub static UTTypeGZIP: UTType;
}
unsafe extern "C" {
    pub static UTTypeBZ2: UTType;
}
unsafe extern "C" {
    pub static UTTypeZIP: UTType;
}
unsafe extern "C" {
    pub static UTTypeAppleArchive: UTType;
}
unsafe extern "C" {
    pub static UTTypeTarArchive: UTType;
}
unsafe extern "C" {
    pub static UTTypeSpreadsheet: UTType;
}
unsafe extern "C" {
    pub static UTTypePresentation: UTType;
}
unsafe extern "C" {
    pub static UTTypeDatabase: UTType;
}
unsafe extern "C" {
    pub static UTTypeMessage: UTType;
}
unsafe extern "C" {
    pub static UTTypeContact: UTType;
}
unsafe extern "C" {
    pub static UTTypeVCard: UTType;
}
unsafe extern "C" {
    pub static UTTypeToDoItem: UTType;
}
unsafe extern "C" {
    pub static UTTypeCalendarEvent: UTType;
}
unsafe extern "C" {
    pub static UTTypeEmailMessage: UTType;
}
unsafe extern "C" {
    pub static UTTypeInternetLocation: UTType;
}
unsafe extern "C" {
    pub static UTTypeInternetShortcut: UTType;
}
unsafe extern "C" {
    pub static UTTypeFont: UTType;
}
unsafe extern "C" {
    pub static UTTypeBookmark: UTType;
}
unsafe extern "C" {
    pub static UTTypePKCS12: UTType;
}
unsafe extern "C" {
    pub static UTTypeX509Certificate: UTType;
}
unsafe extern "C" {
    pub static UTTypeEPUB: UTType;
}
unsafe extern "C" {
    pub static UTTypeLog: UTType;
}
unsafe extern "C" {
    pub static UTTypeAHAP: UTType;
}
unsafe extern "C" {
    pub static UTTypeGeoJSON: UTType;
}
unsafe extern "C" {
    pub static UTTypeLinkPresentationMetadata: UTType;
}

unsafe impl objc2::encode::RefEncode for UTType {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for UTType {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
