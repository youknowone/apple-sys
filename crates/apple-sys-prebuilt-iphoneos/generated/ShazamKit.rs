#[allow(unused_imports)]
use crate::objc::*;
#[allow(unused_imports)]
use crate::AVFAudio::*;
#[allow(unused_imports)]
use crate::AVFoundation::*;
#[allow(unused_imports)]
use crate::Foundation::*;
#[allow(unused_imports)]
use crate::Security::*;
#[allow(unused_imports)]
use crate::UniformTypeIdentifiers::*;

#[allow(unused_imports)]
use objc2::msg_send;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct SHCatalog(pub id);
impl std::ops::Deref for SHCatalog {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for SHCatalog {}
impl SHCatalog {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"SHCatalog").unwrap(), alloc) })
    }
}
impl INSObject for SHCatalog {}
impl PNSObject for SHCatalog {}
impl std::convert::TryFrom<NSObject> for SHCatalog {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<SHCatalog, Self::Error> {
        let is_kind_of: bool = unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"SHCatalog").unwrap()) };
        if is_kind_of {
            Ok(SHCatalog(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to SHCatalog")
        }
    }
}
impl ISHCatalog for SHCatalog {}
pub trait ISHCatalog: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn minimumQuerySignatureDuration(&self) -> NSTimeInterval
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, minimumQuerySignatureDuration)
    }
    unsafe fn maximumQuerySignatureDuration(&self) -> NSTimeInterval
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, maximumQuerySignatureDuration)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SHCatalog").unwrap(), new)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct SHSignature(pub id);
impl std::ops::Deref for SHSignature {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for SHSignature {}
impl SHSignature {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"SHSignature").unwrap(), alloc) })
    }
}
impl PNSSecureCoding for SHSignature {}
impl PNSCopying for SHSignature {}
impl INSObject for SHSignature {}
impl PNSObject for SHSignature {}
impl std::convert::TryFrom<NSObject> for SHSignature {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<SHSignature, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"SHSignature").unwrap()) };
        if is_kind_of {
            Ok(SHSignature(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to SHSignature")
        }
    }
}
impl ISHSignature for SHSignature {}
pub trait ISHSignature: Sized + std::ops::Deref {
    unsafe fn initWithDataRepresentation_error_(
        &self,
        dataRepresentation: NSData,
        error: *mut NSError,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithDataRepresentation : dataRepresentation, error : error)
    }
    unsafe fn duration(&self) -> NSTimeInterval
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, duration)
    }
    unsafe fn dataRepresentation(&self) -> NSData
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, dataRepresentation)
    }
    unsafe fn signatureWithDataRepresentation_error_(
        dataRepresentation: NSData,
        error: *mut NSError,
    ) -> SHSignature
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SHSignature").unwrap(), signatureWithDataRepresentation : dataRepresentation, error : error)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct SHRange(pub id);
impl std::ops::Deref for SHRange {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for SHRange {}
impl SHRange {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"SHRange").unwrap(), alloc) })
    }
}
impl PNSSecureCoding for SHRange {}
impl PNSCopying for SHRange {}
impl INSObject for SHRange {}
impl PNSObject for SHRange {}
impl std::convert::TryFrom<NSObject> for SHRange {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<SHRange, Self::Error> {
        let is_kind_of: bool = unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"SHRange").unwrap()) };
        if is_kind_of {
            Ok(SHRange(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to SHRange")
        }
    }
}
impl ISHRange for SHRange {}
pub trait ISHRange: Sized + std::ops::Deref {
    unsafe fn initWithLowerBound_upperBound_(
        &self,
        lowerBound: f64,
        upperBound: f64,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithLowerBound : lowerBound, upperBound : upperBound)
    }
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn lowerBound(&self) -> f64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, lowerBound)
    }
    unsafe fn upperBound(&self) -> f64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, upperBound)
    }
    unsafe fn rangeWithLowerBound_upperBound_(lowerBound: f64, upperBound: f64) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SHRange").unwrap(), rangeWithLowerBound : lowerBound, upperBound : upperBound)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SHRange").unwrap(), new)
    }
}
pub type SHMediaItemProperty = NSString;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct SHMediaItem(pub id);
impl std::ops::Deref for SHMediaItem {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for SHMediaItem {}
impl SHMediaItem {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"SHMediaItem").unwrap(), alloc) })
    }
}
impl PNSSecureCoding for SHMediaItem {}
impl PNSCopying for SHMediaItem {}
impl INSObject for SHMediaItem {}
impl PNSObject for SHMediaItem {}
impl std::convert::TryFrom<NSObject> for SHMediaItem {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<SHMediaItem, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"SHMediaItem").unwrap()) };
        if is_kind_of {
            Ok(SHMediaItem(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to SHMediaItem")
        }
    }
}
impl ISHMediaItem for SHMediaItem {}
pub trait ISHMediaItem: Sized + std::ops::Deref {
    unsafe fn valueForProperty_(&self, property: NSString) -> id
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, valueForProperty : property)
    }
    unsafe fn objectForKeyedSubscript_(&self, key: NSString) -> id
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, objectForKeyedSubscript : key)
    }
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn shazamID(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, shazamID)
    }
    unsafe fn title(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, title)
    }
    unsafe fn subtitle(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, subtitle)
    }
    unsafe fn artist(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, artist)
    }
    unsafe fn genres(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, genres)
    }
    unsafe fn appleMusicID(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, appleMusicID)
    }
    unsafe fn appleMusicURL(&self) -> NSURL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, appleMusicURL)
    }
    unsafe fn webURL(&self) -> NSURL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, webURL)
    }
    unsafe fn artworkURL(&self) -> NSURL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, artworkURL)
    }
    unsafe fn videoURL(&self) -> NSURL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, videoURL)
    }
    unsafe fn explicitContent(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, explicitContent)
    }
    unsafe fn isrc(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isrc)
    }
    unsafe fn timeRanges(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, timeRanges)
    }
    unsafe fn frequencySkewRanges(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, frequencySkewRanges)
    }
    unsafe fn creationDate(&self) -> NSDate
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, creationDate)
    }
    unsafe fn mediaItemWithProperties_(properties: NSDictionary) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SHMediaItem").unwrap(), mediaItemWithProperties : properties)
    }
    unsafe fn fetchMediaItemWithShazamID_completionHandler_(
        shazamID: NSString,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SHMediaItem").unwrap(), fetchMediaItemWithShazamID : shazamID, completionHandler : completionHandler)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SHMediaItem").unwrap(), new)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct SHCustomCatalog(pub id);
impl std::ops::Deref for SHCustomCatalog {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for SHCustomCatalog {}
impl SHCustomCatalog {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"SHCustomCatalog").unwrap(), alloc) })
    }
}
impl ISHCatalog for SHCustomCatalog {}
impl From<SHCustomCatalog> for SHCatalog {
    fn from(child: SHCustomCatalog) -> SHCatalog {
        SHCatalog(child.0)
    }
}
impl std::convert::TryFrom<SHCatalog> for SHCustomCatalog {
    type Error = &'static str;
    fn try_from(parent: SHCatalog) -> Result<SHCustomCatalog, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"SHCustomCatalog").unwrap()) };
        if is_kind_of {
            Ok(SHCustomCatalog(parent.0))
        } else {
            Err("This SHCatalog cannot be downcasted to SHCustomCatalog")
        }
    }
}
impl INSObject for SHCustomCatalog {}
impl PNSObject for SHCustomCatalog {}
impl ISHCustomCatalog for SHCustomCatalog {}
pub trait ISHCustomCatalog: Sized + std::ops::Deref {
    unsafe fn addReferenceSignature_representingMediaItems_error_(
        &self,
        signature: SHSignature,
        mediaItems: NSArray,
        error: *mut NSError,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, addReferenceSignature : signature, representingMediaItems : mediaItems, error : error)
    }
    unsafe fn addCustomCatalogFromURL_error_(
        &self,
        customCatalogURL: NSURL,
        error: *mut NSError,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, addCustomCatalogFromURL : customCatalogURL, error : error)
    }
    unsafe fn writeToURL_error_(&self, destinationURL: NSURL, error: *mut NSError) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, writeToURL : destinationURL, error : error)
    }
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn initWithDataRepresentation_error_(
        &self,
        dataRepresentation: NSData,
        error: *mut NSError,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithDataRepresentation : dataRepresentation, error : error)
    }
    unsafe fn dataRepresentation(&self) -> NSData
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, dataRepresentation)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SHCustomCatalog").unwrap(), new)
    }
}
pub type SHErrorCode = NSInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct SHMatchedMediaItem(pub id);
impl std::ops::Deref for SHMatchedMediaItem {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for SHMatchedMediaItem {}
impl SHMatchedMediaItem {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"SHMatchedMediaItem").unwrap(), alloc) })
    }
}
impl PNSSecureCoding for SHMatchedMediaItem {}
impl ISHMediaItem for SHMatchedMediaItem {}
impl PNSCopying for SHMatchedMediaItem {}
impl From<SHMatchedMediaItem> for SHMediaItem {
    fn from(child: SHMatchedMediaItem) -> SHMediaItem {
        SHMediaItem(child.0)
    }
}
impl std::convert::TryFrom<SHMediaItem> for SHMatchedMediaItem {
    type Error = &'static str;
    fn try_from(parent: SHMediaItem) -> Result<SHMatchedMediaItem, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"SHMatchedMediaItem").unwrap()) };
        if is_kind_of {
            Ok(SHMatchedMediaItem(parent.0))
        } else {
            Err("This SHMediaItem cannot be downcasted to SHMatchedMediaItem")
        }
    }
}
impl INSObject for SHMatchedMediaItem {}
impl PNSObject for SHMatchedMediaItem {}
impl ISHMatchedMediaItem for SHMatchedMediaItem {}
pub trait ISHMatchedMediaItem: Sized + std::ops::Deref {
    unsafe fn frequencySkew(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, frequencySkew)
    }
    unsafe fn matchOffset(&self) -> NSTimeInterval
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, matchOffset)
    }
    unsafe fn predictedCurrentMatchOffset(&self) -> NSTimeInterval
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, predictedCurrentMatchOffset)
    }
    unsafe fn confidence(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, confidence)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct SHMatch(pub id);
impl std::ops::Deref for SHMatch {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for SHMatch {}
impl SHMatch {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"SHMatch").unwrap(), alloc) })
    }
}
impl PNSSecureCoding for SHMatch {}
impl INSObject for SHMatch {}
impl PNSObject for SHMatch {}
impl std::convert::TryFrom<NSObject> for SHMatch {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<SHMatch, Self::Error> {
        let is_kind_of: bool = unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"SHMatch").unwrap()) };
        if is_kind_of {
            Ok(SHMatch(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to SHMatch")
        }
    }
}
impl ISHMatch for SHMatch {}
pub trait ISHMatch: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn mediaItems(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, mediaItems)
    }
    unsafe fn querySignature(&self) -> SHSignature
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, querySignature)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SHMatch").unwrap(), new)
    }
}
pub trait PSHSessionDelegate: Sized + std::ops::Deref {
    unsafe fn session_didFindMatch_(&self, session: SHSession, match_: SHMatch)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, session : session, didFindMatch : match_)
    }
    unsafe fn session_didNotFindMatchForSignature_error_(
        &self,
        session: SHSession,
        signature: SHSignature,
        error: NSError,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, session : session, didNotFindMatchForSignature : signature, error : error)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct SHSession(pub id);
impl std::ops::Deref for SHSession {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for SHSession {}
impl SHSession {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"SHSession").unwrap(), alloc) })
    }
}
impl INSObject for SHSession {}
impl PNSObject for SHSession {}
impl std::convert::TryFrom<NSObject> for SHSession {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<SHSession, Self::Error> {
        let is_kind_of: bool = unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"SHSession").unwrap()) };
        if is_kind_of {
            Ok(SHSession(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to SHSession")
        }
    }
}
impl ISHSession for SHSession {}
pub trait ISHSession: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn initWithCatalog_(&self, catalog: SHCatalog) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithCatalog : catalog)
    }
    unsafe fn matchStreamingBuffer_atTime_(&self, buffer: AVAudioPCMBuffer, time: AVAudioTime)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, matchStreamingBuffer : buffer, atTime : time)
    }
    unsafe fn matchSignature_(&self, signature: SHSignature)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, matchSignature : signature)
    }
    unsafe fn catalog(&self) -> SHCatalog
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, catalog)
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
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct SHSignatureGenerator(pub id);
impl std::ops::Deref for SHSignatureGenerator {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for SHSignatureGenerator {}
impl SHSignatureGenerator {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"SHSignatureGenerator").unwrap(), alloc) })
    }
}
impl INSObject for SHSignatureGenerator {}
impl PNSObject for SHSignatureGenerator {}
impl std::convert::TryFrom<NSObject> for SHSignatureGenerator {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<SHSignatureGenerator, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"SHSignatureGenerator").unwrap()) };
        if is_kind_of {
            Ok(SHSignatureGenerator(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to SHSignatureGenerator")
        }
    }
}
impl ISHSignatureGenerator for SHSignatureGenerator {}
pub trait ISHSignatureGenerator: Sized + std::ops::Deref {
    unsafe fn appendBuffer_atTime_error_(
        &self,
        buffer: AVAudioPCMBuffer,
        time: AVAudioTime,
        error: *mut NSError,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, appendBuffer : buffer, atTime : time, error : error)
    }
    unsafe fn signature(&self) -> SHSignature
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, signature)
    }
    unsafe fn generateSignatureFromAsset_completionHandler_(
        asset: AVAsset,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SHSignatureGenerator").unwrap(), generateSignatureFromAsset : asset, completionHandler : completionHandler)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct SHMediaLibrary(pub id);
impl std::ops::Deref for SHMediaLibrary {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for SHMediaLibrary {}
impl SHMediaLibrary {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"SHMediaLibrary").unwrap(), alloc) })
    }
}
impl INSObject for SHMediaLibrary {}
impl PNSObject for SHMediaLibrary {}
impl std::convert::TryFrom<NSObject> for SHMediaLibrary {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<SHMediaLibrary, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"SHMediaLibrary").unwrap()) };
        if is_kind_of {
            Ok(SHMediaLibrary(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to SHMediaLibrary")
        }
    }
}
impl ISHMediaLibrary for SHMediaLibrary {}
pub trait ISHMediaLibrary: Sized + std::ops::Deref {
    unsafe fn addMediaItems_completionHandler_(
        &self,
        mediaItems: NSArray,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, addMediaItems : mediaItems, completionHandler : completionHandler)
    }
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SHMediaLibrary").unwrap(), new)
    }
    unsafe fn defaultLibrary() -> SHMediaLibrary
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SHMediaLibrary").unwrap(), defaultLibrary)
    }
}
pub trait UTType_SHShazamAdditions: Sized + std::ops::Deref {
    unsafe fn SHSignatureContentType() -> UTType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"UTType").unwrap(), SHSignatureContentType)
    }
    unsafe fn SHCustomCatalogContentType() -> UTType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"UTType").unwrap(), SHCustomCatalogContentType)
    }
}
unsafe extern "C" {
    pub static SHMediaItemShazamID: SHMediaItemProperty;
}
unsafe extern "C" {
    pub static SHMediaItemTitle: SHMediaItemProperty;
}
unsafe extern "C" {
    pub static SHMediaItemSubtitle: SHMediaItemProperty;
}
unsafe extern "C" {
    pub static SHMediaItemArtist: SHMediaItemProperty;
}
unsafe extern "C" {
    pub static SHMediaItemWebURL: SHMediaItemProperty;
}
unsafe extern "C" {
    pub static SHMediaItemAppleMusicID: SHMediaItemProperty;
}
unsafe extern "C" {
    pub static SHMediaItemAppleMusicURL: SHMediaItemProperty;
}
unsafe extern "C" {
    pub static SHMediaItemArtworkURL: SHMediaItemProperty;
}
unsafe extern "C" {
    pub static SHMediaItemVideoURL: SHMediaItemProperty;
}
unsafe extern "C" {
    pub static SHMediaItemExplicitContent: SHMediaItemProperty;
}
unsafe extern "C" {
    pub static SHMediaItemGenres: SHMediaItemProperty;
}
unsafe extern "C" {
    pub static SHMediaItemISRC: SHMediaItemProperty;
}
unsafe extern "C" {
    pub static SHMediaItemTimeRanges: SHMediaItemProperty;
}
unsafe extern "C" {
    pub static SHMediaItemFrequencySkewRanges: SHMediaItemProperty;
}
unsafe extern "C" {
    pub static SHMediaItemCreationDate: SHMediaItemProperty;
}
unsafe extern "C" {
    pub static SHErrorDomain: NSErrorDomain;
}
unsafe extern "C" {
    pub static SHMediaItemMatchOffset: SHMediaItemProperty;
}
unsafe extern "C" {
    pub static SHMediaItemFrequencySkew: SHMediaItemProperty;
}
unsafe extern "C" {
    pub static SHMediaItemConfidence: SHMediaItemProperty;
}

unsafe impl objc2::encode::RefEncode for SHCatalog {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for SHCatalog {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for SHSignature {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for SHSignature {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for SHRange {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for SHRange {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for SHMediaItem {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for SHMediaItem {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for SHCustomCatalog {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for SHCustomCatalog {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for SHMatchedMediaItem {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for SHMatchedMediaItem {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for SHMatch {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for SHMatch {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for SHSession {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for SHSession {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for SHSignatureGenerator {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for SHSignatureGenerator {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for SHMediaLibrary {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for SHMediaLibrary {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
