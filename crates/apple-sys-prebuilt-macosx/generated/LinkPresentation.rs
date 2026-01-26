#[allow(unused_imports)]
use crate::objc::*;
#[allow(unused_imports)]
use crate::CoreFoundation::*;
#[allow(unused_imports)]
use crate::Foundation::*;

#[allow(unused_imports)]
use objc2::msg_send;
pub type LPErrorCode = NSInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct LPLinkMetadata(pub id);
impl std::ops::Deref for LPLinkMetadata {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for LPLinkMetadata {}
impl LPLinkMetadata {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"LPLinkMetadata").unwrap(), alloc) })
    }
}
impl PNSCopying for LPLinkMetadata {}
impl PNSSecureCoding for LPLinkMetadata {}
impl INSObject for LPLinkMetadata {}
impl PNSObject for LPLinkMetadata {}
impl std::convert::TryFrom<NSObject> for LPLinkMetadata {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<LPLinkMetadata, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"LPLinkMetadata").unwrap()) };
        if is_kind_of {
            Ok(LPLinkMetadata(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to LPLinkMetadata")
        }
    }
}
impl ILPLinkMetadata for LPLinkMetadata {}
pub trait ILPLinkMetadata: Sized + std::ops::Deref {
    unsafe fn originalURL(&self) -> NSURL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, originalURL)
    }
    unsafe fn setOriginalURL_(&self, originalURL: NSURL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setOriginalURL : originalURL)
    }
    unsafe fn URL(&self) -> NSURL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, URL)
    }
    unsafe fn setURL_(&self, URL: NSURL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setURL : URL)
    }
    unsafe fn title(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, title)
    }
    unsafe fn setTitle_(&self, title: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setTitle : title)
    }
    unsafe fn iconProvider(&self) -> NSItemProvider
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, iconProvider)
    }
    unsafe fn setIconProvider_(&self, iconProvider: NSItemProvider)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setIconProvider : iconProvider)
    }
    unsafe fn imageProvider(&self) -> NSItemProvider
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, imageProvider)
    }
    unsafe fn setImageProvider_(&self, imageProvider: NSItemProvider)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setImageProvider : imageProvider)
    }
    unsafe fn videoProvider(&self) -> NSItemProvider
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, videoProvider)
    }
    unsafe fn setVideoProvider_(&self, videoProvider: NSItemProvider)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setVideoProvider : videoProvider)
    }
    unsafe fn remoteVideoURL(&self) -> NSURL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, remoteVideoURL)
    }
    unsafe fn setRemoteVideoURL_(&self, remoteVideoURL: NSURL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRemoteVideoURL : remoteVideoURL)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct LPLinkView(pub id);
impl std::ops::Deref for LPLinkView {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for LPLinkView {}
impl LPLinkView {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"LPLinkView").unwrap(), alloc) })
    }
}
impl PNSCoding for LPLinkView {}
impl INSObject for LPLinkView {}
impl PNSObject for LPLinkView {}
impl std::convert::TryFrom<NSObject> for LPLinkView {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<LPLinkView, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"LPLinkView").unwrap()) };
        if is_kind_of {
            Ok(LPLinkView(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to LPLinkView")
        }
    }
}
impl ILPLinkView for LPLinkView {}
pub trait ILPLinkView: Sized + std::ops::Deref {
    unsafe fn initWithCoder_(&self, coder: NSCoder) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithCoder : coder)
    }
    unsafe fn encodeWithCoder_(&self, coder: NSCoder)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, encodeWithCoder : coder)
    }
    unsafe fn initWithURL_(&self, URL: NSURL) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithURL : URL)
    }
    unsafe fn initWithMetadata_(&self, metadata: LPLinkMetadata) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithMetadata : metadata)
    }
    unsafe fn metadata(&self) -> LPLinkMetadata
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, metadata)
    }
    unsafe fn setMetadata_(&self, metadata: LPLinkMetadata)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMetadata : metadata)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct LPMetadataProvider(pub id);
impl std::ops::Deref for LPMetadataProvider {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for LPMetadataProvider {}
impl LPMetadataProvider {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"LPMetadataProvider").unwrap(), alloc) })
    }
}
impl INSObject for LPMetadataProvider {}
impl PNSObject for LPMetadataProvider {}
impl std::convert::TryFrom<NSObject> for LPMetadataProvider {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<LPMetadataProvider, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"LPMetadataProvider").unwrap()) };
        if is_kind_of {
            Ok(LPMetadataProvider(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to LPMetadataProvider")
        }
    }
}
impl ILPMetadataProvider for LPMetadataProvider {}
pub trait ILPMetadataProvider: Sized + std::ops::Deref {
    unsafe fn startFetchingMetadataForURL_completionHandler_(
        &self,
        URL: NSURL,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, startFetchingMetadataForURL : URL, completionHandler : completionHandler)
    }
    unsafe fn startFetchingMetadataForRequest_completionHandler_(
        &self,
        request: NSURLRequest,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, startFetchingMetadataForRequest : request, completionHandler : completionHandler)
    }
    unsafe fn cancel(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, cancel)
    }
    unsafe fn shouldFetchSubresources(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, shouldFetchSubresources)
    }
    unsafe fn setShouldFetchSubresources_(&self, shouldFetchSubresources: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setShouldFetchSubresources : shouldFetchSubresources)
    }
    unsafe fn timeout(&self) -> NSTimeInterval
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, timeout)
    }
    unsafe fn setTimeout_(&self, timeout: NSTimeInterval)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setTimeout : timeout)
    }
}
pub trait PNSAccessibilityElement: Sized + std::ops::Deref {
    unsafe fn accessibilityFrame(&self) -> NSRect
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, accessibilityFrame)
    }
    unsafe fn accessibilityParent(&self) -> id
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, accessibilityParent)
    }
    unsafe fn isAccessibilityFocused(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isAccessibilityFocused)
    }
    unsafe fn accessibilityIdentifier(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, accessibilityIdentifier)
    }
}
pub trait PNSAnimatablePropertyContainer: Sized + std::ops::Deref {
    unsafe fn animator(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, animator)
    }
    unsafe fn animationForKey_(&self, key: NSString) -> id
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, animationForKey : key)
    }
    unsafe fn animations(&self) -> NSDictionary
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, animations)
    }
    unsafe fn setAnimations_(&self, animations: NSDictionary)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAnimations : animations)
    }
    unsafe fn defaultAnimationForKey_(key: NSString) -> id
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NSAnimatablePropertyContainer").unwrap(), defaultAnimationForKey : key)
    }
}
impl PNSAnimatablePropertyContainer for LPLinkView {}
impl PNSAccessibilityElement for LPLinkView {}
unsafe extern "C" {
    pub static LPErrorDomain: NSErrorDomain;
}

unsafe impl objc2::encode::RefEncode for LPLinkMetadata {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for LPLinkMetadata {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for LPLinkView {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for LPLinkView {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for LPMetadataProvider {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for LPMetadataProvider {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
