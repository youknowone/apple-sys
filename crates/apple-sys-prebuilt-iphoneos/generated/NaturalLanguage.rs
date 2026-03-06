#[allow(unused_imports)]
use crate::objc::*;
#[allow(unused_imports)]
use crate::CoreML::*;
#[allow(unused_imports)]
use crate::Foundation::*;
#[allow(unused_imports)]
use crate::Security::*;

#[allow(unused_imports)]
use objc2::msg_send;
pub type NLLanguage = NSString;
pub type NLDistanceType = NSInteger;
pub type NLDistance = f64;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct NLEmbedding(pub id);
impl std::ops::Deref for NLEmbedding {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for NLEmbedding {}
impl NLEmbedding {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"NLEmbedding").unwrap(), alloc) })
    }
}
impl INSObject for NLEmbedding {}
impl PNSObject for NLEmbedding {}
impl std::convert::TryFrom<NSObject> for NLEmbedding {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<NLEmbedding, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"NLEmbedding").unwrap()) };
        if is_kind_of {
            Ok(NLEmbedding(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to NLEmbedding")
        }
    }
}
impl INLEmbedding for NLEmbedding {}
pub trait INLEmbedding: Sized + std::ops::Deref {
    unsafe fn containsString_(&self, string: NSString) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, containsString : string)
    }
    unsafe fn distanceBetweenString_andString_distanceType_(
        &self,
        firstString: NSString,
        secondString: NSString,
        distanceType: NLDistanceType,
    ) -> NLDistance
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, distanceBetweenString : firstString, andString : secondString, distanceType : distanceType)
    }
    unsafe fn enumerateNeighborsForString_maximumCount_distanceType_usingBlock_(
        &self,
        string: NSString,
        maxCount: NSUInteger,
        distanceType: NLDistanceType,
        block: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, enumerateNeighborsForString : string, maximumCount : maxCount, distanceType : distanceType, usingBlock : block)
    }
    unsafe fn enumerateNeighborsForString_maximumCount_maximumDistance_distanceType_usingBlock_(
        &self,
        string: NSString,
        maxCount: NSUInteger,
        maxDistance: NLDistance,
        distanceType: NLDistanceType,
        block: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, enumerateNeighborsForString : string, maximumCount : maxCount, maximumDistance : maxDistance, distanceType : distanceType, usingBlock : block)
    }
    unsafe fn neighborsForString_maximumCount_distanceType_(
        &self,
        string: NSString,
        maxCount: NSUInteger,
        distanceType: NLDistanceType,
    ) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, neighborsForString : string, maximumCount : maxCount, distanceType : distanceType)
    }
    unsafe fn neighborsForString_maximumCount_maximumDistance_distanceType_(
        &self,
        string: NSString,
        maxCount: NSUInteger,
        maxDistance: NLDistance,
        distanceType: NLDistanceType,
    ) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, neighborsForString : string, maximumCount : maxCount, maximumDistance : maxDistance, distanceType : distanceType)
    }
    unsafe fn vectorForString_(&self, string: NSString) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, vectorForString : string)
    }
    unsafe fn getVector_forString_(&self, vector: *mut f32, string: NSString) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, getVector : vector, forString : string)
    }
    unsafe fn enumerateNeighborsForVector_maximumCount_distanceType_usingBlock_(
        &self,
        vector: NSArray,
        maxCount: NSUInteger,
        distanceType: NLDistanceType,
        block: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, enumerateNeighborsForVector : vector, maximumCount : maxCount, distanceType : distanceType, usingBlock : block)
    }
    unsafe fn enumerateNeighborsForVector_maximumCount_maximumDistance_distanceType_usingBlock_(
        &self,
        vector: NSArray,
        maxCount: NSUInteger,
        maxDistance: NLDistance,
        distanceType: NLDistanceType,
        block: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, enumerateNeighborsForVector : vector, maximumCount : maxCount, maximumDistance : maxDistance, distanceType : distanceType, usingBlock : block)
    }
    unsafe fn neighborsForVector_maximumCount_distanceType_(
        &self,
        vector: NSArray,
        maxCount: NSUInteger,
        distanceType: NLDistanceType,
    ) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, neighborsForVector : vector, maximumCount : maxCount, distanceType : distanceType)
    }
    unsafe fn neighborsForVector_maximumCount_maximumDistance_distanceType_(
        &self,
        vector: NSArray,
        maxCount: NSUInteger,
        maxDistance: NLDistance,
        distanceType: NLDistanceType,
    ) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, neighborsForVector : vector, maximumCount : maxCount, maximumDistance : maxDistance, distanceType : distanceType)
    }
    unsafe fn dimension(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, dimension)
    }
    unsafe fn vocabularySize(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, vocabularySize)
    }
    unsafe fn language(&self) -> NLLanguage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, language)
    }
    unsafe fn revision(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, revision)
    }
    unsafe fn wordEmbeddingForLanguage_(language: NSString) -> NLEmbedding
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NLEmbedding").unwrap(), wordEmbeddingForLanguage : language)
    }
    unsafe fn wordEmbeddingForLanguage_revision_(
        language: NSString,
        revision: NSUInteger,
    ) -> NLEmbedding
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NLEmbedding").unwrap(), wordEmbeddingForLanguage : language, revision : revision)
    }
    unsafe fn sentenceEmbeddingForLanguage_(language: NSString) -> NLEmbedding
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NLEmbedding").unwrap(), sentenceEmbeddingForLanguage : language)
    }
    unsafe fn sentenceEmbeddingForLanguage_revision_(
        language: NSString,
        revision: NSUInteger,
    ) -> NLEmbedding
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NLEmbedding").unwrap(), sentenceEmbeddingForLanguage : language, revision : revision)
    }
    unsafe fn embeddingWithContentsOfURL_error_(url: NSURL, error: *mut NSError) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NLEmbedding").unwrap(), embeddingWithContentsOfURL : url, error : error)
    }
    unsafe fn supportedRevisionsForLanguage_(language: NSString) -> NSIndexSet
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NLEmbedding").unwrap(), supportedRevisionsForLanguage : language)
    }
    unsafe fn currentRevisionForLanguage_(language: NSString) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NLEmbedding").unwrap(), currentRevisionForLanguage : language)
    }
    unsafe fn supportedSentenceEmbeddingRevisionsForLanguage_(language: NSString) -> NSIndexSet
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NLEmbedding").unwrap(), supportedSentenceEmbeddingRevisionsForLanguage : language)
    }
    unsafe fn currentSentenceEmbeddingRevisionForLanguage_(language: NSString) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NLEmbedding").unwrap(), currentSentenceEmbeddingRevisionForLanguage : language)
    }
    unsafe fn writeEmbeddingForDictionary_language_revision_toURL_error_(
        dictionary: NSDictionary,
        language: NSString,
        revision: NSUInteger,
        url: NSURL,
        error: *mut NSError,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NLEmbedding").unwrap(), writeEmbeddingForDictionary : dictionary, language : language, revision : revision, toURL : url, error : error)
    }
}
pub type NLScript = NSString;
pub type NLTokenUnit = NSInteger;
pub type NLTokenizerAttributes = NSUInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct NLTokenizer(pub id);
impl std::ops::Deref for NLTokenizer {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for NLTokenizer {}
impl NLTokenizer {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"NLTokenizer").unwrap(), alloc) })
    }
}
impl INSObject for NLTokenizer {}
impl PNSObject for NLTokenizer {}
impl std::convert::TryFrom<NSObject> for NLTokenizer {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<NLTokenizer, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"NLTokenizer").unwrap()) };
        if is_kind_of {
            Ok(NLTokenizer(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to NLTokenizer")
        }
    }
}
impl INLTokenizer for NLTokenizer {}
pub trait INLTokenizer: Sized + std::ops::Deref {
    unsafe fn initWithUnit_(&self, unit: NLTokenUnit) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithUnit : unit)
    }
    unsafe fn setLanguage_(&self, language: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setLanguage : language)
    }
    unsafe fn tokenRangeAtIndex_(&self, characterIndex: NSUInteger) -> NSRange
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, tokenRangeAtIndex : characterIndex)
    }
    unsafe fn tokenRangeForRange_(&self, range: NSRange) -> NSRange
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, tokenRangeForRange : range)
    }
    unsafe fn tokensForRange_(&self, range: NSRange) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, tokensForRange : range)
    }
    unsafe fn enumerateTokensInRange_usingBlock_(
        &self,
        range: NSRange,
        block: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, enumerateTokensInRange : range, usingBlock : block)
    }
    unsafe fn unit(&self) -> NLTokenUnit
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, unit)
    }
    unsafe fn string(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, string)
    }
    unsafe fn setString_(&self, string: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setString : string)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct NLContextualEmbedding(pub id);
impl std::ops::Deref for NLContextualEmbedding {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for NLContextualEmbedding {}
impl NLContextualEmbedding {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"NLContextualEmbedding").unwrap(), alloc) })
    }
}
impl INSObject for NLContextualEmbedding {}
impl PNSObject for NLContextualEmbedding {}
impl std::convert::TryFrom<NSObject> for NLContextualEmbedding {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<NLContextualEmbedding, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"NLContextualEmbedding").unwrap()) };
        if is_kind_of {
            Ok(NLContextualEmbedding(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to NLContextualEmbedding")
        }
    }
}
impl INLContextualEmbedding for NLContextualEmbedding {}
pub trait INLContextualEmbedding: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn loadWithError_(&self, error: *mut NSError) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, loadWithError : error)
    }
    unsafe fn unload(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, unload)
    }
    unsafe fn embeddingResultForString_language_error_(
        &self,
        string: NSString,
        language: NSString,
        error: *mut NSError,
    ) -> NLContextualEmbeddingResult
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, embeddingResultForString : string, language : language, error : error)
    }
    unsafe fn requestEmbeddingAssetsWithCompletionHandler_(
        &self,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, requestEmbeddingAssetsWithCompletionHandler : completionHandler)
    }
    unsafe fn modelIdentifier(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, modelIdentifier)
    }
    unsafe fn languages(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, languages)
    }
    unsafe fn scripts(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, scripts)
    }
    unsafe fn revision(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, revision)
    }
    unsafe fn dimension(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, dimension)
    }
    unsafe fn maximumSequenceLength(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, maximumSequenceLength)
    }
    unsafe fn hasAvailableAssets(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, hasAvailableAssets)
    }
    unsafe fn contextualEmbeddingWithModelIdentifier_(modelIdentifier: NSString) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NLContextualEmbedding").unwrap(), contextualEmbeddingWithModelIdentifier : modelIdentifier)
    }
    unsafe fn contextualEmbeddingsForValues_(valuesDictionary: NSDictionary) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NLContextualEmbedding").unwrap(), contextualEmbeddingsForValues : valuesDictionary)
    }
    unsafe fn contextualEmbeddingWithLanguage_(language: NSString) -> NLContextualEmbedding
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NLContextualEmbedding").unwrap(), contextualEmbeddingWithLanguage : language)
    }
    unsafe fn contextualEmbeddingWithScript_(script: NSString) -> NLContextualEmbedding
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NLContextualEmbedding").unwrap(), contextualEmbeddingWithScript : script)
    }
}
pub type NLContextualEmbeddingKey = NSString;
pub type NLContextualEmbeddingAssetsResult = NSInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct NLContextualEmbeddingResult(pub id);
impl std::ops::Deref for NLContextualEmbeddingResult {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for NLContextualEmbeddingResult {}
impl NLContextualEmbeddingResult {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"NLContextualEmbeddingResult").unwrap(), alloc) })
    }
}
impl INSObject for NLContextualEmbeddingResult {}
impl PNSObject for NLContextualEmbeddingResult {}
impl std::convert::TryFrom<NSObject> for NLContextualEmbeddingResult {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<NLContextualEmbeddingResult, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"NLContextualEmbeddingResult").unwrap()) };
        if is_kind_of {
            Ok(NLContextualEmbeddingResult(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to NLContextualEmbeddingResult")
        }
    }
}
impl INLContextualEmbeddingResult for NLContextualEmbeddingResult {}
pub trait INLContextualEmbeddingResult: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn enumerateTokenVectorsInRange_usingBlock_(
        &self,
        range: NSRange,
        block: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, enumerateTokenVectorsInRange : range, usingBlock : block)
    }
    unsafe fn tokenVectorAtIndex_tokenRange_(
        &self,
        characterIndex: NSUInteger,
        tokenRange: NSRangePointer,
    ) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, tokenVectorAtIndex : characterIndex, tokenRange : tokenRange)
    }
    unsafe fn string(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, string)
    }
    unsafe fn language(&self) -> NLLanguage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, language)
    }
    unsafe fn sequenceLength(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, sequenceLength)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct NLGazetteer(pub id);
impl std::ops::Deref for NLGazetteer {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for NLGazetteer {}
impl NLGazetteer {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"NLGazetteer").unwrap(), alloc) })
    }
}
impl INSObject for NLGazetteer {}
impl PNSObject for NLGazetteer {}
impl std::convert::TryFrom<NSObject> for NLGazetteer {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<NLGazetteer, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"NLGazetteer").unwrap()) };
        if is_kind_of {
            Ok(NLGazetteer(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to NLGazetteer")
        }
    }
}
impl INLGazetteer for NLGazetteer {}
pub trait INLGazetteer: Sized + std::ops::Deref {
    unsafe fn initWithContentsOfURL_error_(&self, url: NSURL, error: *mut NSError) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithContentsOfURL : url, error : error)
    }
    unsafe fn initWithData_error_(&self, data: NSData, error: *mut NSError) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithData : data, error : error)
    }
    unsafe fn initWithDictionary_language_error_(
        &self,
        dictionary: NSDictionary,
        language: NSString,
        error: *mut NSError,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithDictionary : dictionary, language : language, error : error)
    }
    unsafe fn labelForString_(&self, string: NSString) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, labelForString : string)
    }
    unsafe fn language(&self) -> NLLanguage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, language)
    }
    unsafe fn data(&self) -> NSData
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, data)
    }
    unsafe fn gazetteerWithContentsOfURL_error_(url: NSURL, error: *mut NSError) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NLGazetteer").unwrap(), gazetteerWithContentsOfURL : url, error : error)
    }
    unsafe fn writeGazetteerForDictionary_language_toURL_error_(
        dictionary: NSDictionary,
        language: NSString,
        url: NSURL,
        error: *mut NSError,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NLGazetteer").unwrap(), writeGazetteerForDictionary : dictionary, language : language, toURL : url, error : error)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct NLLanguageRecognizer(pub id);
impl std::ops::Deref for NLLanguageRecognizer {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for NLLanguageRecognizer {}
impl NLLanguageRecognizer {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"NLLanguageRecognizer").unwrap(), alloc) })
    }
}
impl INSObject for NLLanguageRecognizer {}
impl PNSObject for NLLanguageRecognizer {}
impl std::convert::TryFrom<NSObject> for NLLanguageRecognizer {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<NLLanguageRecognizer, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"NLLanguageRecognizer").unwrap()) };
        if is_kind_of {
            Ok(NLLanguageRecognizer(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to NLLanguageRecognizer")
        }
    }
}
impl INLLanguageRecognizer for NLLanguageRecognizer {}
pub trait INLLanguageRecognizer: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn processString_(&self, string: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, processString : string)
    }
    unsafe fn reset(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, reset)
    }
    unsafe fn languageHypothesesWithMaximum_(&self, maxHypotheses: NSUInteger) -> NSDictionary
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, languageHypothesesWithMaximum : maxHypotheses)
    }
    unsafe fn dominantLanguage(&self) -> NLLanguage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, dominantLanguage)
    }
    unsafe fn languageHints(&self) -> NSDictionary
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, languageHints)
    }
    unsafe fn setLanguageHints_(&self, languageHints: NSDictionary)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setLanguageHints : languageHints)
    }
    unsafe fn languageConstraints(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, languageConstraints)
    }
    unsafe fn setLanguageConstraints_(&self, languageConstraints: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setLanguageConstraints : languageConstraints)
    }
    unsafe fn dominantLanguageForString_(string: NSString) -> NLLanguage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NLLanguageRecognizer").unwrap(), dominantLanguageForString : string)
    }
}
pub type NLModelType = NSInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct NLModel(pub id);
impl std::ops::Deref for NLModel {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for NLModel {}
impl NLModel {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"NLModel").unwrap(), alloc) })
    }
}
impl INSObject for NLModel {}
impl PNSObject for NLModel {}
impl std::convert::TryFrom<NSObject> for NLModel {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<NLModel, Self::Error> {
        let is_kind_of: bool = unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"NLModel").unwrap()) };
        if is_kind_of {
            Ok(NLModel(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to NLModel")
        }
    }
}
impl INLModel for NLModel {}
pub trait INLModel: Sized + std::ops::Deref {
    unsafe fn predictedLabelForString_(&self, string: NSString) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, predictedLabelForString : string)
    }
    unsafe fn predictedLabelsForTokens_(&self, tokens: NSArray) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, predictedLabelsForTokens : tokens)
    }
    unsafe fn predictedLabelHypothesesForString_maximumCount_(
        &self,
        string: NSString,
        maximumCount: NSUInteger,
    ) -> NSDictionary
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, predictedLabelHypothesesForString : string, maximumCount : maximumCount)
    }
    unsafe fn predictedLabelHypothesesForTokens_maximumCount_(
        &self,
        tokens: NSArray,
        maximumCount: NSUInteger,
    ) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, predictedLabelHypothesesForTokens : tokens, maximumCount : maximumCount)
    }
    unsafe fn configuration(&self) -> NLModelConfiguration
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, configuration)
    }
    unsafe fn modelWithContentsOfURL_error_(url: NSURL, error: *mut NSError) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NLModel").unwrap(), modelWithContentsOfURL : url, error : error)
    }
    unsafe fn modelWithMLModel_error_(mlModel: MLModel, error: *mut NSError) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NLModel").unwrap(), modelWithMLModel : mlModel, error : error)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct NLModelConfiguration(pub id);
impl std::ops::Deref for NLModelConfiguration {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for NLModelConfiguration {}
impl NLModelConfiguration {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"NLModelConfiguration").unwrap(), alloc) })
    }
}
impl PNSCopying for NLModelConfiguration {}
impl PNSSecureCoding for NLModelConfiguration {}
impl INSObject for NLModelConfiguration {}
impl PNSObject for NLModelConfiguration {}
impl std::convert::TryFrom<NSObject> for NLModelConfiguration {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<NLModelConfiguration, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"NLModelConfiguration").unwrap()) };
        if is_kind_of {
            Ok(NLModelConfiguration(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to NLModelConfiguration")
        }
    }
}
impl INLModelConfiguration for NLModelConfiguration {}
pub trait INLModelConfiguration: Sized + std::ops::Deref {
    unsafe fn type_(&self) -> NLModelType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, type)
    }
    unsafe fn language(&self) -> NLLanguage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, language)
    }
    unsafe fn revision(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, revision)
    }
    unsafe fn supportedRevisionsForType_(type_: NLModelType) -> NSIndexSet
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NLModelConfiguration").unwrap(), supportedRevisionsForType : type_)
    }
    unsafe fn currentRevisionForType_(type_: NLModelType) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NLModelConfiguration").unwrap(), currentRevisionForType : type_)
    }
}
pub type NLTagScheme = NSString;
pub type NLTag = NSString;
pub type NLTaggerOptions = NSUInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct NLTagger(pub id);
impl std::ops::Deref for NLTagger {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for NLTagger {}
impl NLTagger {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"NLTagger").unwrap(), alloc) })
    }
}
impl INSObject for NLTagger {}
impl PNSObject for NLTagger {}
impl std::convert::TryFrom<NSObject> for NLTagger {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<NLTagger, Self::Error> {
        let is_kind_of: bool = unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"NLTagger").unwrap()) };
        if is_kind_of {
            Ok(NLTagger(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to NLTagger")
        }
    }
}
impl INLTagger for NLTagger {}
pub trait INLTagger: Sized + std::ops::Deref {
    unsafe fn initWithTagSchemes_(&self, tagSchemes: NSArray) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithTagSchemes : tagSchemes)
    }
    unsafe fn tokenRangeAtIndex_unit_(
        &self,
        characterIndex: NSUInteger,
        unit: NLTokenUnit,
    ) -> NSRange
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, tokenRangeAtIndex : characterIndex, unit : unit)
    }
    unsafe fn tokenRangeForRange_unit_(&self, range: NSRange, unit: NLTokenUnit) -> NSRange
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, tokenRangeForRange : range, unit : unit)
    }
    unsafe fn enumerateTagsInRange_unit_scheme_options_usingBlock_(
        &self,
        range: NSRange,
        unit: NLTokenUnit,
        scheme: NSString,
        options: NLTaggerOptions,
        block: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, enumerateTagsInRange : range, unit : unit, scheme : scheme, options : options, usingBlock : block)
    }
    unsafe fn tagAtIndex_unit_scheme_tokenRange_(
        &self,
        characterIndex: NSUInteger,
        unit: NLTokenUnit,
        scheme: NSString,
        tokenRange: NSRangePointer,
    ) -> NLTag
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, tagAtIndex : characterIndex, unit : unit, scheme : scheme, tokenRange : tokenRange)
    }
    unsafe fn tagsInRange_unit_scheme_options_tokenRanges_(
        &self,
        range: NSRange,
        unit: NLTokenUnit,
        scheme: NSString,
        options: NLTaggerOptions,
        tokenRanges: *mut NSArray,
    ) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, tagsInRange : range, unit : unit, scheme : scheme, options : options, tokenRanges : tokenRanges)
    }
    unsafe fn tagHypothesesAtIndex_unit_scheme_maximumCount_tokenRange_(
        &self,
        characterIndex: NSUInteger,
        unit: NLTokenUnit,
        scheme: NSString,
        maximumCount: NSUInteger,
        tokenRange: NSRangePointer,
    ) -> NSDictionary
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, tagHypothesesAtIndex : characterIndex, unit : unit, scheme : scheme, maximumCount : maximumCount, tokenRange : tokenRange)
    }
    unsafe fn setLanguage_range_(&self, language: NSString, range: NSRange)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setLanguage : language, range : range)
    }
    unsafe fn setOrthography_range_(&self, orthography: NSOrthography, range: NSRange)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setOrthography : orthography, range : range)
    }
    unsafe fn setModels_forTagScheme_(&self, models: NSArray, tagScheme: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setModels : models, forTagScheme : tagScheme)
    }
    unsafe fn modelsForTagScheme_(&self, tagScheme: NSString) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, modelsForTagScheme : tagScheme)
    }
    unsafe fn setGazetteers_forTagScheme_(&self, gazetteers: NSArray, tagScheme: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setGazetteers : gazetteers, forTagScheme : tagScheme)
    }
    unsafe fn gazetteersForTagScheme_(&self, tagScheme: NSString) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, gazetteersForTagScheme : tagScheme)
    }
    unsafe fn tagSchemes(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, tagSchemes)
    }
    unsafe fn string(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, string)
    }
    unsafe fn setString_(&self, string: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setString : string)
    }
    unsafe fn dominantLanguage(&self) -> NLLanguage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, dominantLanguage)
    }
    unsafe fn availableTagSchemesForUnit_language_(unit: NLTokenUnit, language: NSString) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NLTagger").unwrap(), availableTagSchemesForUnit : unit, language : language)
    }
    unsafe fn requestAssetsForLanguage_tagScheme_completionHandler_(
        language: NSString,
        tagScheme: NSString,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NLTagger").unwrap(), requestAssetsForLanguage : language, tagScheme : tagScheme, completionHandler : completionHandler)
    }
}
pub type NLTaggerAssetsResult = NSInteger;
unsafe extern "C" {
    pub static NLLanguageUndetermined: NLLanguage;
}
unsafe extern "C" {
    pub static NLLanguageAmharic: NLLanguage;
}
unsafe extern "C" {
    pub static NLLanguageArabic: NLLanguage;
}
unsafe extern "C" {
    pub static NLLanguageArmenian: NLLanguage;
}
unsafe extern "C" {
    pub static NLLanguageBengali: NLLanguage;
}
unsafe extern "C" {
    pub static NLLanguageBulgarian: NLLanguage;
}
unsafe extern "C" {
    pub static NLLanguageBurmese: NLLanguage;
}
unsafe extern "C" {
    pub static NLLanguageCatalan: NLLanguage;
}
unsafe extern "C" {
    pub static NLLanguageCherokee: NLLanguage;
}
unsafe extern "C" {
    pub static NLLanguageCroatian: NLLanguage;
}
unsafe extern "C" {
    pub static NLLanguageCzech: NLLanguage;
}
unsafe extern "C" {
    pub static NLLanguageDanish: NLLanguage;
}
unsafe extern "C" {
    pub static NLLanguageDutch: NLLanguage;
}
unsafe extern "C" {
    pub static NLLanguageEnglish: NLLanguage;
}
unsafe extern "C" {
    pub static NLLanguageFinnish: NLLanguage;
}
unsafe extern "C" {
    pub static NLLanguageFrench: NLLanguage;
}
unsafe extern "C" {
    pub static NLLanguageGeorgian: NLLanguage;
}
unsafe extern "C" {
    pub static NLLanguageGerman: NLLanguage;
}
unsafe extern "C" {
    pub static NLLanguageGreek: NLLanguage;
}
unsafe extern "C" {
    pub static NLLanguageGujarati: NLLanguage;
}
unsafe extern "C" {
    pub static NLLanguageHebrew: NLLanguage;
}
unsafe extern "C" {
    pub static NLLanguageHindi: NLLanguage;
}
unsafe extern "C" {
    pub static NLLanguageHungarian: NLLanguage;
}
unsafe extern "C" {
    pub static NLLanguageIcelandic: NLLanguage;
}
unsafe extern "C" {
    pub static NLLanguageIndonesian: NLLanguage;
}
unsafe extern "C" {
    pub static NLLanguageItalian: NLLanguage;
}
unsafe extern "C" {
    pub static NLLanguageJapanese: NLLanguage;
}
unsafe extern "C" {
    pub static NLLanguageKannada: NLLanguage;
}
unsafe extern "C" {
    pub static NLLanguageKhmer: NLLanguage;
}
unsafe extern "C" {
    pub static NLLanguageKorean: NLLanguage;
}
unsafe extern "C" {
    pub static NLLanguageLao: NLLanguage;
}
unsafe extern "C" {
    pub static NLLanguageMalay: NLLanguage;
}
unsafe extern "C" {
    pub static NLLanguageMalayalam: NLLanguage;
}
unsafe extern "C" {
    pub static NLLanguageMarathi: NLLanguage;
}
unsafe extern "C" {
    pub static NLLanguageMongolian: NLLanguage;
}
unsafe extern "C" {
    pub static NLLanguageNorwegian: NLLanguage;
}
unsafe extern "C" {
    pub static NLLanguageOriya: NLLanguage;
}
unsafe extern "C" {
    pub static NLLanguagePersian: NLLanguage;
}
unsafe extern "C" {
    pub static NLLanguagePolish: NLLanguage;
}
unsafe extern "C" {
    pub static NLLanguagePortuguese: NLLanguage;
}
unsafe extern "C" {
    pub static NLLanguagePunjabi: NLLanguage;
}
unsafe extern "C" {
    pub static NLLanguageRomanian: NLLanguage;
}
unsafe extern "C" {
    pub static NLLanguageRussian: NLLanguage;
}
unsafe extern "C" {
    pub static NLLanguageSimplifiedChinese: NLLanguage;
}
unsafe extern "C" {
    pub static NLLanguageSinhalese: NLLanguage;
}
unsafe extern "C" {
    pub static NLLanguageSlovak: NLLanguage;
}
unsafe extern "C" {
    pub static NLLanguageSpanish: NLLanguage;
}
unsafe extern "C" {
    pub static NLLanguageSwedish: NLLanguage;
}
unsafe extern "C" {
    pub static NLLanguageTamil: NLLanguage;
}
unsafe extern "C" {
    pub static NLLanguageTelugu: NLLanguage;
}
unsafe extern "C" {
    pub static NLLanguageThai: NLLanguage;
}
unsafe extern "C" {
    pub static NLLanguageTibetan: NLLanguage;
}
unsafe extern "C" {
    pub static NLLanguageTraditionalChinese: NLLanguage;
}
unsafe extern "C" {
    pub static NLLanguageTurkish: NLLanguage;
}
unsafe extern "C" {
    pub static NLLanguageUkrainian: NLLanguage;
}
unsafe extern "C" {
    pub static NLLanguageUrdu: NLLanguage;
}
unsafe extern "C" {
    pub static NLLanguageVietnamese: NLLanguage;
}
unsafe extern "C" {
    pub static NLLanguageKazakh: NLLanguage;
}
unsafe extern "C" {
    pub static NLScriptUndetermined: NLScript;
}
unsafe extern "C" {
    pub static NLScriptArabic: NLScript;
}
unsafe extern "C" {
    pub static NLScriptArmenian: NLScript;
}
unsafe extern "C" {
    pub static NLScriptBengali: NLScript;
}
unsafe extern "C" {
    pub static NLScriptCanadianAboriginalSyllabics: NLScript;
}
unsafe extern "C" {
    pub static NLScriptCherokee: NLScript;
}
unsafe extern "C" {
    pub static NLScriptCyrillic: NLScript;
}
unsafe extern "C" {
    pub static NLScriptDevanagari: NLScript;
}
unsafe extern "C" {
    pub static NLScriptEthiopic: NLScript;
}
unsafe extern "C" {
    pub static NLScriptGeorgian: NLScript;
}
unsafe extern "C" {
    pub static NLScriptGreek: NLScript;
}
unsafe extern "C" {
    pub static NLScriptGujarati: NLScript;
}
unsafe extern "C" {
    pub static NLScriptGurmukhi: NLScript;
}
unsafe extern "C" {
    pub static NLScriptHebrew: NLScript;
}
unsafe extern "C" {
    pub static NLScriptJapanese: NLScript;
}
unsafe extern "C" {
    pub static NLScriptKannada: NLScript;
}
unsafe extern "C" {
    pub static NLScriptKhmer: NLScript;
}
unsafe extern "C" {
    pub static NLScriptKorean: NLScript;
}
unsafe extern "C" {
    pub static NLScriptLao: NLScript;
}
unsafe extern "C" {
    pub static NLScriptLatin: NLScript;
}
unsafe extern "C" {
    pub static NLScriptMalayalam: NLScript;
}
unsafe extern "C" {
    pub static NLScriptMongolian: NLScript;
}
unsafe extern "C" {
    pub static NLScriptMyanmar: NLScript;
}
unsafe extern "C" {
    pub static NLScriptOriya: NLScript;
}
unsafe extern "C" {
    pub static NLScriptSimplifiedChinese: NLScript;
}
unsafe extern "C" {
    pub static NLScriptSinhala: NLScript;
}
unsafe extern "C" {
    pub static NLScriptTamil: NLScript;
}
unsafe extern "C" {
    pub static NLScriptTelugu: NLScript;
}
unsafe extern "C" {
    pub static NLScriptThai: NLScript;
}
unsafe extern "C" {
    pub static NLScriptTibetan: NLScript;
}
unsafe extern "C" {
    pub static NLScriptTraditionalChinese: NLScript;
}
unsafe extern "C" {
    pub static NLContextualEmbeddingKeyLanguages: NLContextualEmbeddingKey;
}
unsafe extern "C" {
    pub static NLContextualEmbeddingKeyScripts: NLContextualEmbeddingKey;
}
unsafe extern "C" {
    pub static NLContextualEmbeddingKeyRevision: NLContextualEmbeddingKey;
}
unsafe extern "C" {
    pub static NLTagSchemeTokenType: NLTagScheme;
}
unsafe extern "C" {
    pub static NLTagSchemeLexicalClass: NLTagScheme;
}
unsafe extern "C" {
    pub static NLTagSchemeNameType: NLTagScheme;
}
unsafe extern "C" {
    pub static NLTagSchemeNameTypeOrLexicalClass: NLTagScheme;
}
unsafe extern "C" {
    pub static NLTagSchemeLemma: NLTagScheme;
}
unsafe extern "C" {
    pub static NLTagSchemeLanguage: NLTagScheme;
}
unsafe extern "C" {
    pub static NLTagSchemeScript: NLTagScheme;
}
unsafe extern "C" {
    pub static NLTagSchemeSentimentScore: NLTagScheme;
}
unsafe extern "C" {
    pub static NLTagWord: NLTag;
}
unsafe extern "C" {
    pub static NLTagPunctuation: NLTag;
}
unsafe extern "C" {
    pub static NLTagWhitespace: NLTag;
}
unsafe extern "C" {
    pub static NLTagOther: NLTag;
}
unsafe extern "C" {
    pub static NLTagNoun: NLTag;
}
unsafe extern "C" {
    pub static NLTagVerb: NLTag;
}
unsafe extern "C" {
    pub static NLTagAdjective: NLTag;
}
unsafe extern "C" {
    pub static NLTagAdverb: NLTag;
}
unsafe extern "C" {
    pub static NLTagPronoun: NLTag;
}
unsafe extern "C" {
    pub static NLTagDeterminer: NLTag;
}
unsafe extern "C" {
    pub static NLTagParticle: NLTag;
}
unsafe extern "C" {
    pub static NLTagPreposition: NLTag;
}
unsafe extern "C" {
    pub static NLTagNumber: NLTag;
}
unsafe extern "C" {
    pub static NLTagConjunction: NLTag;
}
unsafe extern "C" {
    pub static NLTagInterjection: NLTag;
}
unsafe extern "C" {
    pub static NLTagClassifier: NLTag;
}
unsafe extern "C" {
    pub static NLTagIdiom: NLTag;
}
unsafe extern "C" {
    pub static NLTagOtherWord: NLTag;
}
unsafe extern "C" {
    pub static NLTagSentenceTerminator: NLTag;
}
unsafe extern "C" {
    pub static NLTagOpenQuote: NLTag;
}
unsafe extern "C" {
    pub static NLTagCloseQuote: NLTag;
}
unsafe extern "C" {
    pub static NLTagOpenParenthesis: NLTag;
}
unsafe extern "C" {
    pub static NLTagCloseParenthesis: NLTag;
}
unsafe extern "C" {
    pub static NLTagWordJoiner: NLTag;
}
unsafe extern "C" {
    pub static NLTagDash: NLTag;
}
unsafe extern "C" {
    pub static NLTagOtherPunctuation: NLTag;
}
unsafe extern "C" {
    pub static NLTagParagraphBreak: NLTag;
}
unsafe extern "C" {
    pub static NLTagOtherWhitespace: NLTag;
}
unsafe extern "C" {
    pub static NLTagPersonalName: NLTag;
}
unsafe extern "C" {
    pub static NLTagPlaceName: NLTag;
}
unsafe extern "C" {
    pub static NLTagOrganizationName: NLTag;
}

unsafe impl objc2::encode::RefEncode for NLEmbedding {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for NLEmbedding {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for NLTokenizer {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for NLTokenizer {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for NLContextualEmbedding {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for NLContextualEmbedding {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for NLContextualEmbeddingResult {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for NLContextualEmbeddingResult {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for NLGazetteer {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for NLGazetteer {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for NLLanguageRecognizer {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for NLLanguageRecognizer {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for NLModel {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for NLModel {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for NLModelConfiguration {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for NLModelConfiguration {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for NLTagger {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for NLTagger {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
