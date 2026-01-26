#[allow(unused_imports)]
use crate::objc::*;
#[allow(unused_imports)]
use crate::AVFAudio::*;
#[allow(unused_imports)]
use crate::AVFoundation::*;
#[allow(unused_imports)]
use crate::Foundation::*;
#[allow(unused_imports)]
use crate::ModelIO::*;
#[allow(unused_imports)]
use crate::Security::*;

#[allow(unused_imports)]
use objc2::msg_send;
pub type PHASEUpdateMode = NSInteger;
pub type PHASERenderingMode = NSInteger;
pub type PHASERenderingState = NSInteger;
pub type PHASESpatializationMode = NSInteger;
pub type PHASEReverbPreset = NSInteger;
pub type PHASEError = NSInteger;
pub type PHASESoundEventError = NSInteger;
pub type PHASEAssetError = NSInteger;
pub type PHASESoundEventPrepareHandlerReason = NSInteger;
pub type PHASESoundEventStartHandlerReason = NSInteger;
pub type PHASESoundEventSeekHandlerReason = NSInteger;
pub type PHASESoundEventPrepareState = NSInteger;
pub type PHASEAssetType = NSInteger;
pub type PHASECurveType = NSInteger;
pub type PHASECullOption = NSInteger;
pub type PHASEPlaybackMode = NSInteger;
pub type PHASENormalizationMode = NSInteger;
pub type PHASECalibrationMode = NSInteger;
pub type PHASEAutomaticHeadTrackingFlags = NSUInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct PHASENumericPair(pub id);
impl std::ops::Deref for PHASENumericPair {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for PHASENumericPair {}
impl PHASENumericPair {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"PHASENumericPair").unwrap(), alloc) })
    }
}
impl INSObject for PHASENumericPair {}
impl PNSObject for PHASENumericPair {}
impl std::convert::TryFrom<NSObject> for PHASENumericPair {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<PHASENumericPair, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"PHASENumericPair").unwrap()) };
        if is_kind_of {
            Ok(PHASENumericPair(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to PHASENumericPair")
        }
    }
}
impl IPHASENumericPair for PHASENumericPair {}
pub trait IPHASENumericPair: Sized + std::ops::Deref {
    unsafe fn initWithFirstValue_secondValue_(&self, first: f64, second: f64) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithFirstValue : first, secondValue : second)
    }
    unsafe fn first(&self) -> f64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, first)
    }
    unsafe fn setFirst_(&self, first: f64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setFirst : first)
    }
    unsafe fn second(&self) -> f64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, second)
    }
    unsafe fn setSecond_(&self, second: f64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSecond : second)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct PHASEEnvelopeSegment(pub id);
impl std::ops::Deref for PHASEEnvelopeSegment {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for PHASEEnvelopeSegment {}
impl PHASEEnvelopeSegment {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"PHASEEnvelopeSegment").unwrap(), alloc) })
    }
}
impl INSObject for PHASEEnvelopeSegment {}
impl PNSObject for PHASEEnvelopeSegment {}
impl std::convert::TryFrom<NSObject> for PHASEEnvelopeSegment {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<PHASEEnvelopeSegment, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"PHASEEnvelopeSegment").unwrap()) };
        if is_kind_of {
            Ok(PHASEEnvelopeSegment(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to PHASEEnvelopeSegment")
        }
    }
}
impl IPHASEEnvelopeSegment for PHASEEnvelopeSegment {}
pub trait IPHASEEnvelopeSegment: Sized + std::ops::Deref {
    unsafe fn initWithEndPoint_curveType_(
        &self,
        endPoint: simd_double2,
        curveType: PHASECurveType,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithEndPoint : endPoint, curveType : curveType)
    }
    unsafe fn endPoint(&self) -> simd_double2
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, endPoint)
    }
    unsafe fn setEndPoint_(&self, endPoint: simd_double2)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setEndPoint : endPoint)
    }
    unsafe fn curveType(&self) -> PHASECurveType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, curveType)
    }
    unsafe fn setCurveType_(&self, curveType: PHASECurveType)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCurveType : curveType)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct PHASEEnvelope(pub id);
impl std::ops::Deref for PHASEEnvelope {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for PHASEEnvelope {}
impl PHASEEnvelope {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"PHASEEnvelope").unwrap(), alloc) })
    }
}
impl INSObject for PHASEEnvelope {}
impl PNSObject for PHASEEnvelope {}
impl std::convert::TryFrom<NSObject> for PHASEEnvelope {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<PHASEEnvelope, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"PHASEEnvelope").unwrap()) };
        if is_kind_of {
            Ok(PHASEEnvelope(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to PHASEEnvelope")
        }
    }
}
impl IPHASEEnvelope for PHASEEnvelope {}
pub trait IPHASEEnvelope: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn initWithStartPoint_segments_(
        &self,
        startPoint: simd_double2,
        segments: NSArray,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithStartPoint : startPoint, segments : segments)
    }
    unsafe fn evaluateForValue_(&self, x: f64) -> f64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, evaluateForValue : x)
    }
    unsafe fn startPoint(&self) -> simd_double2
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, startPoint)
    }
    unsafe fn segments(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, segments)
    }
    unsafe fn domain(&self) -> PHASENumericPair
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, domain)
    }
    unsafe fn range(&self) -> PHASENumericPair
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, range)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"PHASEEnvelope").unwrap(), new)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct PHASEDefinition(pub id);
impl std::ops::Deref for PHASEDefinition {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for PHASEDefinition {}
impl PHASEDefinition {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"PHASEDefinition").unwrap(), alloc) })
    }
}
impl INSObject for PHASEDefinition {}
impl PNSObject for PHASEDefinition {}
impl std::convert::TryFrom<NSObject> for PHASEDefinition {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<PHASEDefinition, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"PHASEDefinition").unwrap()) };
        if is_kind_of {
            Ok(PHASEDefinition(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to PHASEDefinition")
        }
    }
}
impl IPHASEDefinition for PHASEDefinition {}
pub trait IPHASEDefinition: Sized + std::ops::Deref {
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
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"PHASEDefinition").unwrap(), new)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct PHASEMetaParameterDefinition(pub id);
impl std::ops::Deref for PHASEMetaParameterDefinition {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for PHASEMetaParameterDefinition {}
impl PHASEMetaParameterDefinition {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"PHASEMetaParameterDefinition").unwrap(), alloc) })
    }
}
impl IPHASEDefinition for PHASEMetaParameterDefinition {}
impl From<PHASEMetaParameterDefinition> for PHASEDefinition {
    fn from(child: PHASEMetaParameterDefinition) -> PHASEDefinition {
        PHASEDefinition(child.0)
    }
}
impl std::convert::TryFrom<PHASEDefinition> for PHASEMetaParameterDefinition {
    type Error = &'static str;
    fn try_from(parent: PHASEDefinition) -> Result<PHASEMetaParameterDefinition, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"PHASEMetaParameterDefinition").unwrap()) };
        if is_kind_of {
            Ok(PHASEMetaParameterDefinition(parent.0))
        } else {
            Err("This PHASEDefinition cannot be downcasted to PHASEMetaParameterDefinition")
        }
    }
}
impl INSObject for PHASEMetaParameterDefinition {}
impl PNSObject for PHASEMetaParameterDefinition {}
impl IPHASEMetaParameterDefinition for PHASEMetaParameterDefinition {}
pub trait IPHASEMetaParameterDefinition: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn value(&self) -> id
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, value)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"PHASEMetaParameterDefinition").unwrap(), new)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct PHASENumberMetaParameterDefinition(pub id);
impl std::ops::Deref for PHASENumberMetaParameterDefinition {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for PHASENumberMetaParameterDefinition {}
impl PHASENumberMetaParameterDefinition {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"PHASENumberMetaParameterDefinition").unwrap(), alloc) })
    }
}
impl IPHASEMetaParameterDefinition for PHASENumberMetaParameterDefinition {}
impl From<PHASENumberMetaParameterDefinition> for PHASEMetaParameterDefinition {
    fn from(child: PHASENumberMetaParameterDefinition) -> PHASEMetaParameterDefinition {
        PHASEMetaParameterDefinition(child.0)
    }
}
impl std::convert::TryFrom<PHASEMetaParameterDefinition> for PHASENumberMetaParameterDefinition {
    type Error = &'static str;
    fn try_from(
        parent: PHASEMetaParameterDefinition,
    ) -> Result<PHASENumberMetaParameterDefinition, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"PHASENumberMetaParameterDefinition").unwrap())
        };
        if is_kind_of {
            Ok(PHASENumberMetaParameterDefinition(parent.0))
        } else {
            Err ("This PHASEMetaParameterDefinition cannot be downcasted to PHASENumberMetaParameterDefinition" ,)
        }
    }
}
impl IPHASEDefinition for PHASENumberMetaParameterDefinition {}
impl INSObject for PHASENumberMetaParameterDefinition {}
impl PNSObject for PHASENumberMetaParameterDefinition {}
impl IPHASENumberMetaParameterDefinition for PHASENumberMetaParameterDefinition {}
pub trait IPHASENumberMetaParameterDefinition: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn initWithValue_identifier_(&self, value: f64, identifier: NSString) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithValue : value, identifier : identifier)
    }
    unsafe fn initWithValue_(&self, value: f64) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithValue : value)
    }
    unsafe fn initWithValue_minimum_maximum_identifier_(
        &self,
        value: f64,
        minimum: f64,
        maximum: f64,
        identifier: NSString,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithValue : value, minimum : minimum, maximum : maximum, identifier : identifier)
    }
    unsafe fn initWithValue_minimum_maximum_(
        &self,
        value: f64,
        minimum: f64,
        maximum: f64,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithValue : value, minimum : minimum, maximum : maximum)
    }
    unsafe fn minimum(&self) -> f64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, minimum)
    }
    unsafe fn maximum(&self) -> f64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, maximum)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"PHASENumberMetaParameterDefinition").unwrap(), new)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct PHASEStringMetaParameterDefinition(pub id);
impl std::ops::Deref for PHASEStringMetaParameterDefinition {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for PHASEStringMetaParameterDefinition {}
impl PHASEStringMetaParameterDefinition {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"PHASEStringMetaParameterDefinition").unwrap(), alloc) })
    }
}
impl IPHASEMetaParameterDefinition for PHASEStringMetaParameterDefinition {}
impl std::convert::TryFrom<PHASEMetaParameterDefinition> for PHASEStringMetaParameterDefinition {
    type Error = &'static str;
    fn try_from(
        parent: PHASEMetaParameterDefinition,
    ) -> Result<PHASEStringMetaParameterDefinition, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"PHASEStringMetaParameterDefinition").unwrap())
        };
        if is_kind_of {
            Ok(PHASEStringMetaParameterDefinition(parent.0))
        } else {
            Err ("This PHASEMetaParameterDefinition cannot be downcasted to PHASEStringMetaParameterDefinition" ,)
        }
    }
}
impl IPHASEDefinition for PHASEStringMetaParameterDefinition {}
impl INSObject for PHASEStringMetaParameterDefinition {}
impl PNSObject for PHASEStringMetaParameterDefinition {}
impl IPHASEStringMetaParameterDefinition for PHASEStringMetaParameterDefinition {}
pub trait IPHASEStringMetaParameterDefinition: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn initWithValue_identifier_(
        &self,
        value: NSString,
        identifier: NSString,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithValue : value, identifier : identifier)
    }
    unsafe fn initWithValue_(&self, value: NSString) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithValue : value)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"PHASEStringMetaParameterDefinition").unwrap(), new)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct PHASEMappedMetaParameterDefinition(pub id);
impl std::ops::Deref for PHASEMappedMetaParameterDefinition {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for PHASEMappedMetaParameterDefinition {}
impl PHASEMappedMetaParameterDefinition {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"PHASEMappedMetaParameterDefinition").unwrap(), alloc) })
    }
}
impl IPHASENumberMetaParameterDefinition for PHASEMappedMetaParameterDefinition {}
impl From<PHASEMappedMetaParameterDefinition> for PHASENumberMetaParameterDefinition {
    fn from(child: PHASEMappedMetaParameterDefinition) -> PHASENumberMetaParameterDefinition {
        PHASENumberMetaParameterDefinition(child.0)
    }
}
impl std::convert::TryFrom<PHASENumberMetaParameterDefinition>
    for PHASEMappedMetaParameterDefinition
{
    type Error = &'static str;
    fn try_from(
        parent: PHASENumberMetaParameterDefinition,
    ) -> Result<PHASEMappedMetaParameterDefinition, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"PHASEMappedMetaParameterDefinition").unwrap())
        };
        if is_kind_of {
            Ok(PHASEMappedMetaParameterDefinition(parent.0))
        } else {
            Err ("This PHASENumberMetaParameterDefinition cannot be downcasted to PHASEMappedMetaParameterDefinition" ,)
        }
    }
}
impl IPHASEMetaParameterDefinition for PHASEMappedMetaParameterDefinition {}
impl IPHASEDefinition for PHASEMappedMetaParameterDefinition {}
impl INSObject for PHASEMappedMetaParameterDefinition {}
impl PNSObject for PHASEMappedMetaParameterDefinition {}
impl IPHASEMappedMetaParameterDefinition for PHASEMappedMetaParameterDefinition {}
pub trait IPHASEMappedMetaParameterDefinition: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn initWithValue_identifier_(&self, value: f64, identifier: NSString) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithValue : value, identifier : identifier)
    }
    unsafe fn initWithValue_(&self, value: f64) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithValue : value)
    }
    unsafe fn initWithValue_minimum_maximum_identifier_(
        &self,
        value: f64,
        minimum: f64,
        maximum: f64,
        identifier: NSString,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithValue : value, minimum : minimum, maximum : maximum, identifier : identifier)
    }
    unsafe fn initWithValue_minimum_maximum_(
        &self,
        value: f64,
        minimum: f64,
        maximum: f64,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithValue : value, minimum : minimum, maximum : maximum)
    }
    unsafe fn initWithInputMetaParameterDefinition_envelope_identifier_(
        &self,
        inputMetaParameterDefinition: PHASENumberMetaParameterDefinition,
        envelope: PHASEEnvelope,
        identifier: NSString,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithInputMetaParameterDefinition : inputMetaParameterDefinition, envelope : envelope, identifier : identifier)
    }
    unsafe fn initWithInputMetaParameterDefinition_envelope_(
        &self,
        inputMetaParameterDefinition: PHASENumberMetaParameterDefinition,
        envelope: PHASEEnvelope,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithInputMetaParameterDefinition : inputMetaParameterDefinition, envelope : envelope)
    }
    unsafe fn envelope(&self) -> PHASEEnvelope
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, envelope)
    }
    unsafe fn inputMetaParameterDefinition(&self) -> PHASENumberMetaParameterDefinition
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inputMetaParameterDefinition)
    }
    unsafe fn minimum(&self) -> f64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, minimum)
    }
    unsafe fn maximum(&self) -> f64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, maximum)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"PHASEMappedMetaParameterDefinition").unwrap(), new)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct PHASEMetaParameter(pub id);
impl std::ops::Deref for PHASEMetaParameter {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for PHASEMetaParameter {}
impl PHASEMetaParameter {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"PHASEMetaParameter").unwrap(), alloc) })
    }
}
impl INSObject for PHASEMetaParameter {}
impl PNSObject for PHASEMetaParameter {}
impl std::convert::TryFrom<NSObject> for PHASEMetaParameter {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<PHASEMetaParameter, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"PHASEMetaParameter").unwrap()) };
        if is_kind_of {
            Ok(PHASEMetaParameter(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to PHASEMetaParameter")
        }
    }
}
impl IPHASEMetaParameter for PHASEMetaParameter {}
pub trait IPHASEMetaParameter: Sized + std::ops::Deref {
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
    unsafe fn value(&self) -> id
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, value)
    }
    unsafe fn setValue_(&self, value: id)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setValue : value)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"PHASEMetaParameter").unwrap(), new)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct PHASENumberMetaParameter(pub id);
impl std::ops::Deref for PHASENumberMetaParameter {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for PHASENumberMetaParameter {}
impl PHASENumberMetaParameter {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"PHASENumberMetaParameter").unwrap(), alloc) })
    }
}
impl IPHASEMetaParameter for PHASENumberMetaParameter {}
impl From<PHASENumberMetaParameter> for PHASEMetaParameter {
    fn from(child: PHASENumberMetaParameter) -> PHASEMetaParameter {
        PHASEMetaParameter(child.0)
    }
}
impl std::convert::TryFrom<PHASEMetaParameter> for PHASENumberMetaParameter {
    type Error = &'static str;
    fn try_from(parent: PHASEMetaParameter) -> Result<PHASENumberMetaParameter, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"PHASENumberMetaParameter").unwrap()) };
        if is_kind_of {
            Ok(PHASENumberMetaParameter(parent.0))
        } else {
            Err("This PHASEMetaParameter cannot be downcasted to PHASENumberMetaParameter")
        }
    }
}
impl INSObject for PHASENumberMetaParameter {}
impl PNSObject for PHASENumberMetaParameter {}
impl IPHASENumberMetaParameter for PHASENumberMetaParameter {}
pub trait IPHASENumberMetaParameter: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn fadeToValue_duration_(&self, value: f64, duration: NSTimeInterval)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, fadeToValue : value, duration : duration)
    }
    unsafe fn minimum(&self) -> f64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, minimum)
    }
    unsafe fn maximum(&self) -> f64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, maximum)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"PHASENumberMetaParameter").unwrap(), new)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct PHASEStringMetaParameter(pub id);
impl std::ops::Deref for PHASEStringMetaParameter {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for PHASEStringMetaParameter {}
impl PHASEStringMetaParameter {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"PHASEStringMetaParameter").unwrap(), alloc) })
    }
}
impl IPHASEMetaParameter for PHASEStringMetaParameter {}
impl std::convert::TryFrom<PHASEMetaParameter> for PHASEStringMetaParameter {
    type Error = &'static str;
    fn try_from(parent: PHASEMetaParameter) -> Result<PHASEStringMetaParameter, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"PHASEStringMetaParameter").unwrap()) };
        if is_kind_of {
            Ok(PHASEStringMetaParameter(parent.0))
        } else {
            Err("This PHASEMetaParameter cannot be downcasted to PHASEStringMetaParameter")
        }
    }
}
impl INSObject for PHASEStringMetaParameter {}
impl PNSObject for PHASEStringMetaParameter {}
impl IPHASEStringMetaParameter for PHASEStringMetaParameter {}
pub trait IPHASEStringMetaParameter: Sized + std::ops::Deref {
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
        msg_send!(&*objc2::runtime::AnyClass::get(c"PHASEStringMetaParameter").unwrap(), new)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct PHASEMixerDefinition(pub id);
impl std::ops::Deref for PHASEMixerDefinition {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for PHASEMixerDefinition {}
impl PHASEMixerDefinition {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"PHASEMixerDefinition").unwrap(), alloc) })
    }
}
impl IPHASEDefinition for PHASEMixerDefinition {}
impl std::convert::TryFrom<PHASEDefinition> for PHASEMixerDefinition {
    type Error = &'static str;
    fn try_from(parent: PHASEDefinition) -> Result<PHASEMixerDefinition, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"PHASEMixerDefinition").unwrap()) };
        if is_kind_of {
            Ok(PHASEMixerDefinition(parent.0))
        } else {
            Err("This PHASEDefinition cannot be downcasted to PHASEMixerDefinition")
        }
    }
}
impl INSObject for PHASEMixerDefinition {}
impl PNSObject for PHASEMixerDefinition {}
impl IPHASEMixerDefinition for PHASEMixerDefinition {}
pub trait IPHASEMixerDefinition: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn gain(&self) -> f64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, gain)
    }
    unsafe fn setGain_(&self, gain: f64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setGain : gain)
    }
    unsafe fn gainMetaParameterDefinition(&self) -> PHASENumberMetaParameterDefinition
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, gainMetaParameterDefinition)
    }
    unsafe fn setGainMetaParameterDefinition_(
        &self,
        gainMetaParameterDefinition: PHASENumberMetaParameterDefinition,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setGainMetaParameterDefinition : gainMetaParameterDefinition)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"PHASEMixerDefinition").unwrap(), new)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct PHASESpatialMixerDefinition(pub id);
impl std::ops::Deref for PHASESpatialMixerDefinition {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for PHASESpatialMixerDefinition {}
impl PHASESpatialMixerDefinition {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"PHASESpatialMixerDefinition").unwrap(), alloc) })
    }
}
impl IPHASEMixerDefinition for PHASESpatialMixerDefinition {}
impl From<PHASESpatialMixerDefinition> for PHASEMixerDefinition {
    fn from(child: PHASESpatialMixerDefinition) -> PHASEMixerDefinition {
        PHASEMixerDefinition(child.0)
    }
}
impl std::convert::TryFrom<PHASEMixerDefinition> for PHASESpatialMixerDefinition {
    type Error = &'static str;
    fn try_from(parent: PHASEMixerDefinition) -> Result<PHASESpatialMixerDefinition, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"PHASESpatialMixerDefinition").unwrap()) };
        if is_kind_of {
            Ok(PHASESpatialMixerDefinition(parent.0))
        } else {
            Err("This PHASEMixerDefinition cannot be downcasted to PHASESpatialMixerDefinition")
        }
    }
}
impl IPHASEDefinition for PHASESpatialMixerDefinition {}
impl INSObject for PHASESpatialMixerDefinition {}
impl PNSObject for PHASESpatialMixerDefinition {}
impl IPHASESpatialMixerDefinition for PHASESpatialMixerDefinition {}
pub trait IPHASESpatialMixerDefinition: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn initWithSpatialPipeline_(&self, spatialPipeline: PHASESpatialPipeline) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithSpatialPipeline : spatialPipeline)
    }
    unsafe fn initWithSpatialPipeline_identifier_(
        &self,
        spatialPipeline: PHASESpatialPipeline,
        identifier: NSString,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithSpatialPipeline : spatialPipeline, identifier : identifier)
    }
    unsafe fn spatialPipeline(&self) -> PHASESpatialPipeline
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, spatialPipeline)
    }
    unsafe fn distanceModelParameters(&self) -> PHASEDistanceModelParameters
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, distanceModelParameters)
    }
    unsafe fn setDistanceModelParameters_(
        &self,
        distanceModelParameters: PHASEDistanceModelParameters,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDistanceModelParameters : distanceModelParameters)
    }
    unsafe fn listenerDirectivityModelParameters(&self) -> PHASEDirectivityModelParameters
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, listenerDirectivityModelParameters)
    }
    unsafe fn setListenerDirectivityModelParameters_(
        &self,
        listenerDirectivityModelParameters: PHASEDirectivityModelParameters,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setListenerDirectivityModelParameters : listenerDirectivityModelParameters)
    }
    unsafe fn sourceDirectivityModelParameters(&self) -> PHASEDirectivityModelParameters
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, sourceDirectivityModelParameters)
    }
    unsafe fn setSourceDirectivityModelParameters_(
        &self,
        sourceDirectivityModelParameters: PHASEDirectivityModelParameters,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSourceDirectivityModelParameters : sourceDirectivityModelParameters)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"PHASESpatialMixerDefinition").unwrap(), new)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct PHASEAmbientMixerDefinition(pub id);
impl std::ops::Deref for PHASEAmbientMixerDefinition {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for PHASEAmbientMixerDefinition {}
impl PHASEAmbientMixerDefinition {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"PHASEAmbientMixerDefinition").unwrap(), alloc) })
    }
}
impl IPHASEMixerDefinition for PHASEAmbientMixerDefinition {}
impl std::convert::TryFrom<PHASEMixerDefinition> for PHASEAmbientMixerDefinition {
    type Error = &'static str;
    fn try_from(parent: PHASEMixerDefinition) -> Result<PHASEAmbientMixerDefinition, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"PHASEAmbientMixerDefinition").unwrap()) };
        if is_kind_of {
            Ok(PHASEAmbientMixerDefinition(parent.0))
        } else {
            Err("This PHASEMixerDefinition cannot be downcasted to PHASEAmbientMixerDefinition")
        }
    }
}
impl IPHASEDefinition for PHASEAmbientMixerDefinition {}
impl INSObject for PHASEAmbientMixerDefinition {}
impl PNSObject for PHASEAmbientMixerDefinition {}
impl IPHASEAmbientMixerDefinition for PHASEAmbientMixerDefinition {}
pub trait IPHASEAmbientMixerDefinition: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn inputChannelLayout(&self) -> AVAudioChannelLayout
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inputChannelLayout)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"PHASEAmbientMixerDefinition").unwrap(), new)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct PHASEChannelMixerDefinition(pub id);
impl std::ops::Deref for PHASEChannelMixerDefinition {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for PHASEChannelMixerDefinition {}
impl PHASEChannelMixerDefinition {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"PHASEChannelMixerDefinition").unwrap(), alloc) })
    }
}
impl IPHASEMixerDefinition for PHASEChannelMixerDefinition {}
impl std::convert::TryFrom<PHASEMixerDefinition> for PHASEChannelMixerDefinition {
    type Error = &'static str;
    fn try_from(parent: PHASEMixerDefinition) -> Result<PHASEChannelMixerDefinition, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"PHASEChannelMixerDefinition").unwrap()) };
        if is_kind_of {
            Ok(PHASEChannelMixerDefinition(parent.0))
        } else {
            Err("This PHASEMixerDefinition cannot be downcasted to PHASEChannelMixerDefinition")
        }
    }
}
impl IPHASEDefinition for PHASEChannelMixerDefinition {}
impl INSObject for PHASEChannelMixerDefinition {}
impl PNSObject for PHASEChannelMixerDefinition {}
impl IPHASEChannelMixerDefinition for PHASEChannelMixerDefinition {}
pub trait IPHASEChannelMixerDefinition: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn initWithChannelLayout_identifier_(
        &self,
        layout: AVAudioChannelLayout,
        identifier: NSString,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithChannelLayout : layout, identifier : identifier)
    }
    unsafe fn initWithChannelLayout_(&self, layout: AVAudioChannelLayout) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithChannelLayout : layout)
    }
    unsafe fn inputChannelLayout(&self) -> AVAudioChannelLayout
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inputChannelLayout)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"PHASEChannelMixerDefinition").unwrap(), new)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct PHASEMixer(pub id);
impl std::ops::Deref for PHASEMixer {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for PHASEMixer {}
impl PHASEMixer {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"PHASEMixer").unwrap(), alloc) })
    }
}
impl INSObject for PHASEMixer {}
impl PNSObject for PHASEMixer {}
impl std::convert::TryFrom<NSObject> for PHASEMixer {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<PHASEMixer, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"PHASEMixer").unwrap()) };
        if is_kind_of {
            Ok(PHASEMixer(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to PHASEMixer")
        }
    }
}
impl IPHASEMixer for PHASEMixer {}
pub trait IPHASEMixer: Sized + std::ops::Deref {
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
    unsafe fn gain(&self) -> f64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, gain)
    }
    unsafe fn gainMetaParameter(&self) -> PHASEMetaParameter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, gainMetaParameter)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"PHASEMixer").unwrap(), new)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct PHASEMixerParameters(pub id);
impl std::ops::Deref for PHASEMixerParameters {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for PHASEMixerParameters {}
impl PHASEMixerParameters {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"PHASEMixerParameters").unwrap(), alloc) })
    }
}
impl INSObject for PHASEMixerParameters {}
impl PNSObject for PHASEMixerParameters {}
impl std::convert::TryFrom<NSObject> for PHASEMixerParameters {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<PHASEMixerParameters, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"PHASEMixerParameters").unwrap()) };
        if is_kind_of {
            Ok(PHASEMixerParameters(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to PHASEMixerParameters")
        }
    }
}
impl IPHASEMixerParameters for PHASEMixerParameters {}
pub trait IPHASEMixerParameters: Sized + std::ops::Deref {
    unsafe fn addSpatialMixerParametersWithIdentifier_source_listener_(
        &self,
        identifier: NSString,
        source: PHASESource,
        listener: PHASEListener,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, addSpatialMixerParametersWithIdentifier : identifier, source : source, listener : listener)
    }
    unsafe fn addAmbientMixerParametersWithIdentifier_listener_(
        &self,
        identifier: NSString,
        listener: PHASEListener,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, addAmbientMixerParametersWithIdentifier : identifier, listener : listener)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct PHASEGroup(pub id);
impl std::ops::Deref for PHASEGroup {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for PHASEGroup {}
impl PHASEGroup {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"PHASEGroup").unwrap(), alloc) })
    }
}
impl INSObject for PHASEGroup {}
impl PNSObject for PHASEGroup {}
impl std::convert::TryFrom<NSObject> for PHASEGroup {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<PHASEGroup, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"PHASEGroup").unwrap()) };
        if is_kind_of {
            Ok(PHASEGroup(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to PHASEGroup")
        }
    }
}
impl IPHASEGroup for PHASEGroup {}
pub trait IPHASEGroup: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn initWithIdentifier_(&self, identifier: NSString) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithIdentifier : identifier)
    }
    unsafe fn registerWithEngine_(&self, engine: PHASEEngine)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, registerWithEngine : engine)
    }
    unsafe fn unregisterFromEngine(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, unregisterFromEngine)
    }
    unsafe fn fadeGain_duration_curveType_(
        &self,
        gain: f64,
        duration: f64,
        curveType: PHASECurveType,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, fadeGain : gain, duration : duration, curveType : curveType)
    }
    unsafe fn fadeRate_duration_curveType_(
        &self,
        rate: f64,
        duration: f64,
        curveType: PHASECurveType,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, fadeRate : rate, duration : duration, curveType : curveType)
    }
    unsafe fn mute(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, mute)
    }
    unsafe fn unmute(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, unmute)
    }
    unsafe fn solo(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, solo)
    }
    unsafe fn unsolo(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, unsolo)
    }
    unsafe fn identifier(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, identifier)
    }
    unsafe fn gain(&self) -> f64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, gain)
    }
    unsafe fn setGain_(&self, gain: f64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setGain : gain)
    }
    unsafe fn rate(&self) -> f64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, rate)
    }
    unsafe fn setRate_(&self, rate: f64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRate : rate)
    }
    unsafe fn isMuted(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isMuted)
    }
    unsafe fn isSoloed(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isSoloed)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"PHASEGroup").unwrap(), new)
    }
}
pub type PHASEPushStreamBufferOptions = NSUInteger;
pub type PHASEPushStreamCompletionCallbackCondition = NSInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct PHASESoundEventNodeDefinition(pub id);
impl std::ops::Deref for PHASESoundEventNodeDefinition {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for PHASESoundEventNodeDefinition {}
impl PHASESoundEventNodeDefinition {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"PHASESoundEventNodeDefinition").unwrap(), alloc) })
    }
}
impl IPHASEDefinition for PHASESoundEventNodeDefinition {}
impl std::convert::TryFrom<PHASEDefinition> for PHASESoundEventNodeDefinition {
    type Error = &'static str;
    fn try_from(parent: PHASEDefinition) -> Result<PHASESoundEventNodeDefinition, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"PHASESoundEventNodeDefinition").unwrap())
        };
        if is_kind_of {
            Ok(PHASESoundEventNodeDefinition(parent.0))
        } else {
            Err("This PHASEDefinition cannot be downcasted to PHASESoundEventNodeDefinition")
        }
    }
}
impl INSObject for PHASESoundEventNodeDefinition {}
impl PNSObject for PHASESoundEventNodeDefinition {}
impl IPHASESoundEventNodeDefinition for PHASESoundEventNodeDefinition {}
pub trait IPHASESoundEventNodeDefinition: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn children(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, children)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"PHASESoundEventNodeDefinition").unwrap(), new)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct PHASEGeneratorNodeDefinition(pub id);
impl std::ops::Deref for PHASEGeneratorNodeDefinition {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for PHASEGeneratorNodeDefinition {}
impl PHASEGeneratorNodeDefinition {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"PHASEGeneratorNodeDefinition").unwrap(), alloc) })
    }
}
impl IPHASESoundEventNodeDefinition for PHASEGeneratorNodeDefinition {}
impl From<PHASEGeneratorNodeDefinition> for PHASESoundEventNodeDefinition {
    fn from(child: PHASEGeneratorNodeDefinition) -> PHASESoundEventNodeDefinition {
        PHASESoundEventNodeDefinition(child.0)
    }
}
impl std::convert::TryFrom<PHASESoundEventNodeDefinition> for PHASEGeneratorNodeDefinition {
    type Error = &'static str;
    fn try_from(
        parent: PHASESoundEventNodeDefinition,
    ) -> Result<PHASEGeneratorNodeDefinition, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"PHASEGeneratorNodeDefinition").unwrap()) };
        if is_kind_of {
            Ok(PHASEGeneratorNodeDefinition(parent.0))
        } else {
            Err ("This PHASESoundEventNodeDefinition cannot be downcasted to PHASEGeneratorNodeDefinition" ,)
        }
    }
}
impl IPHASEDefinition for PHASEGeneratorNodeDefinition {}
impl INSObject for PHASEGeneratorNodeDefinition {}
impl PNSObject for PHASEGeneratorNodeDefinition {}
impl IPHASEGeneratorNodeDefinition for PHASEGeneratorNodeDefinition {}
pub trait IPHASEGeneratorNodeDefinition: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn setCalibrationMode_level_(&self, calibrationMode: PHASECalibrationMode, level: f64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCalibrationMode : calibrationMode, level : level)
    }
    unsafe fn calibrationMode(&self) -> PHASECalibrationMode
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, calibrationMode)
    }
    unsafe fn level(&self) -> f64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, level)
    }
    unsafe fn rate(&self) -> f64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, rate)
    }
    unsafe fn setRate_(&self, rate: f64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRate : rate)
    }
    unsafe fn group(&self) -> PHASEGroup
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, group)
    }
    unsafe fn setGroup_(&self, group: PHASEGroup)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setGroup : group)
    }
    unsafe fn gainMetaParameterDefinition(&self) -> PHASENumberMetaParameterDefinition
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, gainMetaParameterDefinition)
    }
    unsafe fn setGainMetaParameterDefinition_(
        &self,
        gainMetaParameterDefinition: PHASENumberMetaParameterDefinition,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setGainMetaParameterDefinition : gainMetaParameterDefinition)
    }
    unsafe fn rateMetaParameterDefinition(&self) -> PHASENumberMetaParameterDefinition
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, rateMetaParameterDefinition)
    }
    unsafe fn setRateMetaParameterDefinition_(
        &self,
        rateMetaParameterDefinition: PHASENumberMetaParameterDefinition,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRateMetaParameterDefinition : rateMetaParameterDefinition)
    }
    unsafe fn mixerDefinition(&self) -> PHASEMixerDefinition
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, mixerDefinition)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"PHASEGeneratorNodeDefinition").unwrap(), new)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct PHASESamplerNodeDefinition(pub id);
impl std::ops::Deref for PHASESamplerNodeDefinition {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for PHASESamplerNodeDefinition {}
impl PHASESamplerNodeDefinition {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"PHASESamplerNodeDefinition").unwrap(), alloc) })
    }
}
impl IPHASEGeneratorNodeDefinition for PHASESamplerNodeDefinition {}
impl From<PHASESamplerNodeDefinition> for PHASEGeneratorNodeDefinition {
    fn from(child: PHASESamplerNodeDefinition) -> PHASEGeneratorNodeDefinition {
        PHASEGeneratorNodeDefinition(child.0)
    }
}
impl std::convert::TryFrom<PHASEGeneratorNodeDefinition> for PHASESamplerNodeDefinition {
    type Error = &'static str;
    fn try_from(
        parent: PHASEGeneratorNodeDefinition,
    ) -> Result<PHASESamplerNodeDefinition, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"PHASESamplerNodeDefinition").unwrap()) };
        if is_kind_of {
            Ok(PHASESamplerNodeDefinition(parent.0))
        } else {
            Err ("This PHASEGeneratorNodeDefinition cannot be downcasted to PHASESamplerNodeDefinition" ,)
        }
    }
}
impl IPHASESoundEventNodeDefinition for PHASESamplerNodeDefinition {}
impl IPHASEDefinition for PHASESamplerNodeDefinition {}
impl INSObject for PHASESamplerNodeDefinition {}
impl PNSObject for PHASESamplerNodeDefinition {}
impl IPHASESamplerNodeDefinition for PHASESamplerNodeDefinition {}
pub trait IPHASESamplerNodeDefinition: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn initWithSoundAssetIdentifier_mixerDefinition_identifier_(
        &self,
        soundAssetIdentifier: NSString,
        mixerDefinition: PHASEMixerDefinition,
        identifier: NSString,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithSoundAssetIdentifier : soundAssetIdentifier, mixerDefinition : mixerDefinition, identifier : identifier)
    }
    unsafe fn initWithSoundAssetIdentifier_mixerDefinition_(
        &self,
        soundAssetIdentifier: NSString,
        mixerDefinition: PHASEMixerDefinition,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithSoundAssetIdentifier : soundAssetIdentifier, mixerDefinition : mixerDefinition)
    }
    unsafe fn assetIdentifier(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, assetIdentifier)
    }
    unsafe fn cullOption(&self) -> PHASECullOption
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, cullOption)
    }
    unsafe fn setCullOption_(&self, cullOption: PHASECullOption)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCullOption : cullOption)
    }
    unsafe fn playbackMode(&self) -> PHASEPlaybackMode
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, playbackMode)
    }
    unsafe fn setPlaybackMode_(&self, playbackMode: PHASEPlaybackMode)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPlaybackMode : playbackMode)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"PHASESamplerNodeDefinition").unwrap(), new)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct PHASEContainerNodeDefinition(pub id);
impl std::ops::Deref for PHASEContainerNodeDefinition {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for PHASEContainerNodeDefinition {}
impl PHASEContainerNodeDefinition {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"PHASEContainerNodeDefinition").unwrap(), alloc) })
    }
}
impl IPHASESoundEventNodeDefinition for PHASEContainerNodeDefinition {}
impl std::convert::TryFrom<PHASESoundEventNodeDefinition> for PHASEContainerNodeDefinition {
    type Error = &'static str;
    fn try_from(
        parent: PHASESoundEventNodeDefinition,
    ) -> Result<PHASEContainerNodeDefinition, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"PHASEContainerNodeDefinition").unwrap()) };
        if is_kind_of {
            Ok(PHASEContainerNodeDefinition(parent.0))
        } else {
            Err ("This PHASESoundEventNodeDefinition cannot be downcasted to PHASEContainerNodeDefinition" ,)
        }
    }
}
impl IPHASEDefinition for PHASEContainerNodeDefinition {}
impl INSObject for PHASEContainerNodeDefinition {}
impl PNSObject for PHASEContainerNodeDefinition {}
impl IPHASEContainerNodeDefinition for PHASEContainerNodeDefinition {}
pub trait IPHASEContainerNodeDefinition: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn initWithIdentifier_(&self, identifier: NSString) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithIdentifier : identifier)
    }
    unsafe fn addSubtree_(&self, subtree: PHASESoundEventNodeDefinition)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, addSubtree : subtree)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"PHASEContainerNodeDefinition").unwrap(), new)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct PHASEBlendNodeDefinition(pub id);
impl std::ops::Deref for PHASEBlendNodeDefinition {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for PHASEBlendNodeDefinition {}
impl PHASEBlendNodeDefinition {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"PHASEBlendNodeDefinition").unwrap(), alloc) })
    }
}
impl IPHASESoundEventNodeDefinition for PHASEBlendNodeDefinition {}
impl std::convert::TryFrom<PHASESoundEventNodeDefinition> for PHASEBlendNodeDefinition {
    type Error = &'static str;
    fn try_from(
        parent: PHASESoundEventNodeDefinition,
    ) -> Result<PHASEBlendNodeDefinition, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"PHASEBlendNodeDefinition").unwrap()) };
        if is_kind_of {
            Ok(PHASEBlendNodeDefinition(parent.0))
        } else {
            Err ("This PHASESoundEventNodeDefinition cannot be downcasted to PHASEBlendNodeDefinition" ,)
        }
    }
}
impl IPHASEDefinition for PHASEBlendNodeDefinition {}
impl INSObject for PHASEBlendNodeDefinition {}
impl PNSObject for PHASEBlendNodeDefinition {}
impl IPHASEBlendNodeDefinition for PHASEBlendNodeDefinition {}
pub trait IPHASEBlendNodeDefinition: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn initWithBlendMetaParameterDefinition_identifier_(
        &self,
        blendMetaParameterDefinition: PHASENumberMetaParameterDefinition,
        identifier: NSString,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithBlendMetaParameterDefinition : blendMetaParameterDefinition, identifier : identifier)
    }
    unsafe fn initWithBlendMetaParameterDefinition_(
        &self,
        blendMetaParameterDefinition: PHASENumberMetaParameterDefinition,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithBlendMetaParameterDefinition : blendMetaParameterDefinition)
    }
    unsafe fn initDistanceBlendWithSpatialMixerDefinition_identifier_(
        &self,
        spatialMixerDefinition: PHASESpatialMixerDefinition,
        identifier: NSString,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initDistanceBlendWithSpatialMixerDefinition : spatialMixerDefinition, identifier : identifier)
    }
    unsafe fn initDistanceBlendWithSpatialMixerDefinition_(
        &self,
        spatialMixerDefinition: PHASESpatialMixerDefinition,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initDistanceBlendWithSpatialMixerDefinition : spatialMixerDefinition)
    }
    unsafe fn addRangeForInputValuesBelow_fullGainAtValue_fadeCurveType_subtree_(
        &self,
        value: f64,
        fullGainAtValue: f64,
        fadeCurveType: PHASECurveType,
        subtree: PHASESoundEventNodeDefinition,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, addRangeForInputValuesBelow : value, fullGainAtValue : fullGainAtValue, fadeCurveType : fadeCurveType, subtree : subtree)
    }
    unsafe fn addRangeForInputValuesBetween_highValue_fullGainAtLowValue_fullGainAtHighValue_lowFadeCurveType_highFadeCurveType_subtree_(
        &self,
        lowValue: f64,
        highValue: f64,
        fullGainAtLowValue: f64,
        fullGainAtHighValue: f64,
        lowFadeCurveType: PHASECurveType,
        highFadeCurveType: PHASECurveType,
        subtree: PHASESoundEventNodeDefinition,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, addRangeForInputValuesBetween : lowValue, highValue : highValue, fullGainAtLowValue : fullGainAtLowValue, fullGainAtHighValue : fullGainAtHighValue, lowFadeCurveType : lowFadeCurveType, highFadeCurveType : highFadeCurveType, subtree : subtree)
    }
    unsafe fn addRangeForInputValuesAbove_fullGainAtValue_fadeCurveType_subtree_(
        &self,
        value: f64,
        fullGainAtValue: f64,
        fadeCurveType: PHASECurveType,
        subtree: PHASESoundEventNodeDefinition,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, addRangeForInputValuesAbove : value, fullGainAtValue : fullGainAtValue, fadeCurveType : fadeCurveType, subtree : subtree)
    }
    unsafe fn addRangeWithEnvelope_subtree_(
        &self,
        envelope: PHASEEnvelope,
        subtree: PHASESoundEventNodeDefinition,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, addRangeWithEnvelope : envelope, subtree : subtree)
    }
    unsafe fn blendParameterDefinition(&self) -> PHASENumberMetaParameterDefinition
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, blendParameterDefinition)
    }
    unsafe fn spatialMixerDefinitionForDistance(&self) -> PHASESpatialMixerDefinition
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, spatialMixerDefinitionForDistance)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"PHASEBlendNodeDefinition").unwrap(), new)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct PHASESwitchNodeDefinition(pub id);
impl std::ops::Deref for PHASESwitchNodeDefinition {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for PHASESwitchNodeDefinition {}
impl PHASESwitchNodeDefinition {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"PHASESwitchNodeDefinition").unwrap(), alloc) })
    }
}
impl IPHASESoundEventNodeDefinition for PHASESwitchNodeDefinition {}
impl std::convert::TryFrom<PHASESoundEventNodeDefinition> for PHASESwitchNodeDefinition {
    type Error = &'static str;
    fn try_from(
        parent: PHASESoundEventNodeDefinition,
    ) -> Result<PHASESwitchNodeDefinition, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"PHASESwitchNodeDefinition").unwrap()) };
        if is_kind_of {
            Ok(PHASESwitchNodeDefinition(parent.0))
        } else {
            Err ("This PHASESoundEventNodeDefinition cannot be downcasted to PHASESwitchNodeDefinition" ,)
        }
    }
}
impl IPHASEDefinition for PHASESwitchNodeDefinition {}
impl INSObject for PHASESwitchNodeDefinition {}
impl PNSObject for PHASESwitchNodeDefinition {}
impl IPHASESwitchNodeDefinition for PHASESwitchNodeDefinition {}
pub trait IPHASESwitchNodeDefinition: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn initWithSwitchMetaParameterDefinition_identifier_(
        &self,
        switchMetaParameterDefinition: PHASEStringMetaParameterDefinition,
        identifier: NSString,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithSwitchMetaParameterDefinition : switchMetaParameterDefinition, identifier : identifier)
    }
    unsafe fn initWithSwitchMetaParameterDefinition_(
        &self,
        switchMetaParameterDefinition: PHASEStringMetaParameterDefinition,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithSwitchMetaParameterDefinition : switchMetaParameterDefinition)
    }
    unsafe fn addSubtree_switchValue_(
        &self,
        subtree: PHASESoundEventNodeDefinition,
        switchValue: NSString,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, addSubtree : subtree, switchValue : switchValue)
    }
    unsafe fn switchMetaParameterDefinition(&self) -> PHASEStringMetaParameterDefinition
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, switchMetaParameterDefinition)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"PHASESwitchNodeDefinition").unwrap(), new)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct PHASERandomNodeDefinition(pub id);
impl std::ops::Deref for PHASERandomNodeDefinition {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for PHASERandomNodeDefinition {}
impl PHASERandomNodeDefinition {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"PHASERandomNodeDefinition").unwrap(), alloc) })
    }
}
impl IPHASESoundEventNodeDefinition for PHASERandomNodeDefinition {}
impl std::convert::TryFrom<PHASESoundEventNodeDefinition> for PHASERandomNodeDefinition {
    type Error = &'static str;
    fn try_from(
        parent: PHASESoundEventNodeDefinition,
    ) -> Result<PHASERandomNodeDefinition, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"PHASERandomNodeDefinition").unwrap()) };
        if is_kind_of {
            Ok(PHASERandomNodeDefinition(parent.0))
        } else {
            Err ("This PHASESoundEventNodeDefinition cannot be downcasted to PHASERandomNodeDefinition" ,)
        }
    }
}
impl IPHASEDefinition for PHASERandomNodeDefinition {}
impl INSObject for PHASERandomNodeDefinition {}
impl PNSObject for PHASERandomNodeDefinition {}
impl IPHASERandomNodeDefinition for PHASERandomNodeDefinition {}
pub trait IPHASERandomNodeDefinition: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn initWithIdentifier_(&self, identifier: NSString) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithIdentifier : identifier)
    }
    unsafe fn addSubtree_weight_(&self, subtree: PHASESoundEventNodeDefinition, weight: NSNumber)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, addSubtree : subtree, weight : weight)
    }
    unsafe fn uniqueSelectionQueueLength(&self) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, uniqueSelectionQueueLength)
    }
    unsafe fn setUniqueSelectionQueueLength_(&self, uniqueSelectionQueueLength: NSInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setUniqueSelectionQueueLength : uniqueSelectionQueueLength)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct PHASEStreamNode(pub id);
impl std::ops::Deref for PHASEStreamNode {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for PHASEStreamNode {}
impl PHASEStreamNode {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"PHASEStreamNode").unwrap(), alloc) })
    }
}
impl INSObject for PHASEStreamNode {}
impl PNSObject for PHASEStreamNode {}
impl std::convert::TryFrom<NSObject> for PHASEStreamNode {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<PHASEStreamNode, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"PHASEStreamNode").unwrap()) };
        if is_kind_of {
            Ok(PHASEStreamNode(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to PHASEStreamNode")
        }
    }
}
impl IPHASEStreamNode for PHASEStreamNode {}
pub trait IPHASEStreamNode: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn gainMetaParameter(&self) -> PHASENumberMetaParameter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, gainMetaParameter)
    }
    unsafe fn rateMetaParameter(&self) -> PHASENumberMetaParameter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, rateMetaParameter)
    }
    unsafe fn mixer(&self) -> PHASEMixer
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, mixer)
    }
    unsafe fn format(&self) -> AVAudioFormat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, format)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"PHASEStreamNode").unwrap(), new)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct PHASEPushStreamNodeDefinition(pub id);
impl std::ops::Deref for PHASEPushStreamNodeDefinition {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for PHASEPushStreamNodeDefinition {}
impl PHASEPushStreamNodeDefinition {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"PHASEPushStreamNodeDefinition").unwrap(), alloc) })
    }
}
impl IPHASEGeneratorNodeDefinition for PHASEPushStreamNodeDefinition {}
impl std::convert::TryFrom<PHASEGeneratorNodeDefinition> for PHASEPushStreamNodeDefinition {
    type Error = &'static str;
    fn try_from(
        parent: PHASEGeneratorNodeDefinition,
    ) -> Result<PHASEPushStreamNodeDefinition, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"PHASEPushStreamNodeDefinition").unwrap())
        };
        if is_kind_of {
            Ok(PHASEPushStreamNodeDefinition(parent.0))
        } else {
            Err ("This PHASEGeneratorNodeDefinition cannot be downcasted to PHASEPushStreamNodeDefinition" ,)
        }
    }
}
impl IPHASESoundEventNodeDefinition for PHASEPushStreamNodeDefinition {}
impl IPHASEDefinition for PHASEPushStreamNodeDefinition {}
impl INSObject for PHASEPushStreamNodeDefinition {}
impl PNSObject for PHASEPushStreamNodeDefinition {}
impl IPHASEPushStreamNodeDefinition for PHASEPushStreamNodeDefinition {}
pub trait IPHASEPushStreamNodeDefinition: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn initWithMixerDefinition_format_identifier_(
        &self,
        mixerDefinition: PHASEMixerDefinition,
        format: AVAudioFormat,
        identifier: NSString,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithMixerDefinition : mixerDefinition, format : format, identifier : identifier)
    }
    unsafe fn initWithMixerDefinition_format_(
        &self,
        mixerDefinition: PHASEMixerDefinition,
        format: AVAudioFormat,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithMixerDefinition : mixerDefinition, format : format)
    }
    unsafe fn format(&self) -> AVAudioFormat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, format)
    }
    unsafe fn normalize(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, normalize)
    }
    unsafe fn setNormalize_(&self, normalize: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setNormalize : normalize)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"PHASEPushStreamNodeDefinition").unwrap(), new)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct PHASEPushStreamNode(pub id);
impl std::ops::Deref for PHASEPushStreamNode {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for PHASEPushStreamNode {}
impl PHASEPushStreamNode {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"PHASEPushStreamNode").unwrap(), alloc) })
    }
}
impl IPHASEStreamNode for PHASEPushStreamNode {}
impl From<PHASEPushStreamNode> for PHASEStreamNode {
    fn from(child: PHASEPushStreamNode) -> PHASEStreamNode {
        PHASEStreamNode(child.0)
    }
}
impl std::convert::TryFrom<PHASEStreamNode> for PHASEPushStreamNode {
    type Error = &'static str;
    fn try_from(parent: PHASEStreamNode) -> Result<PHASEPushStreamNode, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"PHASEPushStreamNode").unwrap()) };
        if is_kind_of {
            Ok(PHASEPushStreamNode(parent.0))
        } else {
            Err("This PHASEStreamNode cannot be downcasted to PHASEPushStreamNode")
        }
    }
}
impl INSObject for PHASEPushStreamNode {}
impl PNSObject for PHASEPushStreamNode {}
impl IPHASEPushStreamNode for PHASEPushStreamNode {}
pub trait IPHASEPushStreamNode: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn scheduleBuffer_(&self, buffer: AVAudioPCMBuffer)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, scheduleBuffer : buffer)
    }
    unsafe fn scheduleBuffer_completionCallbackType_completionHandler_(
        &self,
        buffer: AVAudioPCMBuffer,
        completionCallbackType: PHASEPushStreamCompletionCallbackCondition,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, scheduleBuffer : buffer, completionCallbackType : completionCallbackType, completionHandler : completionHandler)
    }
    unsafe fn scheduleBuffer_atTime_options_(
        &self,
        buffer: AVAudioPCMBuffer,
        when: AVAudioTime,
        options: PHASEPushStreamBufferOptions,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, scheduleBuffer : buffer, atTime : when, options : options)
    }
    unsafe fn scheduleBuffer_atTime_options_completionCallbackType_completionHandler_(
        &self,
        buffer: AVAudioPCMBuffer,
        when: AVAudioTime,
        options: PHASEPushStreamBufferOptions,
        completionCallbackType: PHASEPushStreamCompletionCallbackCondition,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, scheduleBuffer : buffer, atTime : when, options : options, completionCallbackType : completionCallbackType, completionHandler : completionHandler)
    }
    unsafe fn gainMetaParameter(&self) -> PHASENumberMetaParameter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, gainMetaParameter)
    }
    unsafe fn rateMetaParameter(&self) -> PHASENumberMetaParameter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, rateMetaParameter)
    }
    unsafe fn mixer(&self) -> PHASEMixer
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, mixer)
    }
    unsafe fn format(&self) -> AVAudioFormat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, format)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"PHASEPushStreamNode").unwrap(), new)
    }
}
pub type PHASEPullStreamRenderBlock = *mut ::std::os::raw::c_void;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct PHASEPullStreamNodeDefinition(pub id);
impl std::ops::Deref for PHASEPullStreamNodeDefinition {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for PHASEPullStreamNodeDefinition {}
impl PHASEPullStreamNodeDefinition {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"PHASEPullStreamNodeDefinition").unwrap(), alloc) })
    }
}
impl IPHASEGeneratorNodeDefinition for PHASEPullStreamNodeDefinition {}
impl std::convert::TryFrom<PHASEGeneratorNodeDefinition> for PHASEPullStreamNodeDefinition {
    type Error = &'static str;
    fn try_from(
        parent: PHASEGeneratorNodeDefinition,
    ) -> Result<PHASEPullStreamNodeDefinition, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"PHASEPullStreamNodeDefinition").unwrap())
        };
        if is_kind_of {
            Ok(PHASEPullStreamNodeDefinition(parent.0))
        } else {
            Err ("This PHASEGeneratorNodeDefinition cannot be downcasted to PHASEPullStreamNodeDefinition" ,)
        }
    }
}
impl IPHASESoundEventNodeDefinition for PHASEPullStreamNodeDefinition {}
impl IPHASEDefinition for PHASEPullStreamNodeDefinition {}
impl INSObject for PHASEPullStreamNodeDefinition {}
impl PNSObject for PHASEPullStreamNodeDefinition {}
impl IPHASEPullStreamNodeDefinition for PHASEPullStreamNodeDefinition {}
pub trait IPHASEPullStreamNodeDefinition: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn initWithMixerDefinition_format_identifier_(
        &self,
        mixerDefinition: PHASEMixerDefinition,
        format: AVAudioFormat,
        identifier: NSString,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithMixerDefinition : mixerDefinition, format : format, identifier : identifier)
    }
    unsafe fn initWithMixerDefinition_format_(
        &self,
        mixerDefinition: PHASEMixerDefinition,
        format: AVAudioFormat,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithMixerDefinition : mixerDefinition, format : format)
    }
    unsafe fn format(&self) -> AVAudioFormat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, format)
    }
    unsafe fn normalize(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, normalize)
    }
    unsafe fn setNormalize_(&self, normalize: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setNormalize : normalize)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"PHASEPullStreamNodeDefinition").unwrap(), new)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct PHASEPullStreamNode(pub id);
impl std::ops::Deref for PHASEPullStreamNode {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for PHASEPullStreamNode {}
impl PHASEPullStreamNode {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"PHASEPullStreamNode").unwrap(), alloc) })
    }
}
impl IPHASEStreamNode for PHASEPullStreamNode {}
impl std::convert::TryFrom<PHASEStreamNode> for PHASEPullStreamNode {
    type Error = &'static str;
    fn try_from(parent: PHASEStreamNode) -> Result<PHASEPullStreamNode, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"PHASEPullStreamNode").unwrap()) };
        if is_kind_of {
            Ok(PHASEPullStreamNode(parent.0))
        } else {
            Err("This PHASEStreamNode cannot be downcasted to PHASEPullStreamNode")
        }
    }
}
impl INSObject for PHASEPullStreamNode {}
impl PNSObject for PHASEPullStreamNode {}
impl IPHASEPullStreamNode for PHASEPullStreamNode {}
pub trait IPHASEPullStreamNode: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn renderBlock(&self) -> PHASEPullStreamRenderBlock
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, renderBlock)
    }
    unsafe fn setRenderBlock_(&self, renderBlock: PHASEPullStreamRenderBlock)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRenderBlock : renderBlock)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"PHASEPullStreamNode").unwrap(), new)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct PHASEAsset(pub id);
impl std::ops::Deref for PHASEAsset {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for PHASEAsset {}
impl PHASEAsset {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"PHASEAsset").unwrap(), alloc) })
    }
}
impl INSObject for PHASEAsset {}
impl PNSObject for PHASEAsset {}
impl std::convert::TryFrom<NSObject> for PHASEAsset {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<PHASEAsset, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"PHASEAsset").unwrap()) };
        if is_kind_of {
            Ok(PHASEAsset(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to PHASEAsset")
        }
    }
}
impl IPHASEAsset for PHASEAsset {}
pub trait IPHASEAsset: Sized + std::ops::Deref {
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
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"PHASEAsset").unwrap(), new)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct PHASESoundAsset(pub id);
impl std::ops::Deref for PHASESoundAsset {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for PHASESoundAsset {}
impl PHASESoundAsset {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"PHASESoundAsset").unwrap(), alloc) })
    }
}
impl IPHASEAsset for PHASESoundAsset {}
impl From<PHASESoundAsset> for PHASEAsset {
    fn from(child: PHASESoundAsset) -> PHASEAsset {
        PHASEAsset(child.0)
    }
}
impl std::convert::TryFrom<PHASEAsset> for PHASESoundAsset {
    type Error = &'static str;
    fn try_from(parent: PHASEAsset) -> Result<PHASESoundAsset, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"PHASESoundAsset").unwrap()) };
        if is_kind_of {
            Ok(PHASESoundAsset(parent.0))
        } else {
            Err("This PHASEAsset cannot be downcasted to PHASESoundAsset")
        }
    }
}
impl INSObject for PHASESoundAsset {}
impl PNSObject for PHASESoundAsset {}
impl IPHASESoundAsset for PHASESoundAsset {}
pub trait IPHASESoundAsset: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn url(&self) -> NSURL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, url)
    }
    unsafe fn data(&self) -> NSData
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, data)
    }
    unsafe fn type_(&self) -> PHASEAssetType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, type)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"PHASESoundAsset").unwrap(), new)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct PHASESoundEventNodeAsset(pub id);
impl std::ops::Deref for PHASESoundEventNodeAsset {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for PHASESoundEventNodeAsset {}
impl PHASESoundEventNodeAsset {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"PHASESoundEventNodeAsset").unwrap(), alloc) })
    }
}
impl IPHASEAsset for PHASESoundEventNodeAsset {}
impl std::convert::TryFrom<PHASEAsset> for PHASESoundEventNodeAsset {
    type Error = &'static str;
    fn try_from(parent: PHASEAsset) -> Result<PHASESoundEventNodeAsset, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"PHASESoundEventNodeAsset").unwrap()) };
        if is_kind_of {
            Ok(PHASESoundEventNodeAsset(parent.0))
        } else {
            Err("This PHASEAsset cannot be downcasted to PHASESoundEventNodeAsset")
        }
    }
}
impl INSObject for PHASESoundEventNodeAsset {}
impl PNSObject for PHASESoundEventNodeAsset {}
impl IPHASESoundEventNodeAsset for PHASESoundEventNodeAsset {}
pub trait IPHASESoundEventNodeAsset: Sized + std::ops::Deref {
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
        msg_send!(&*objc2::runtime::AnyClass::get(c"PHASESoundEventNodeAsset").unwrap(), new)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct PHASEGlobalMetaParameterAsset(pub id);
impl std::ops::Deref for PHASEGlobalMetaParameterAsset {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for PHASEGlobalMetaParameterAsset {}
impl PHASEGlobalMetaParameterAsset {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"PHASEGlobalMetaParameterAsset").unwrap(), alloc) })
    }
}
impl IPHASEAsset for PHASEGlobalMetaParameterAsset {}
impl std::convert::TryFrom<PHASEAsset> for PHASEGlobalMetaParameterAsset {
    type Error = &'static str;
    fn try_from(parent: PHASEAsset) -> Result<PHASEGlobalMetaParameterAsset, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"PHASEGlobalMetaParameterAsset").unwrap())
        };
        if is_kind_of {
            Ok(PHASEGlobalMetaParameterAsset(parent.0))
        } else {
            Err("This PHASEAsset cannot be downcasted to PHASEGlobalMetaParameterAsset")
        }
    }
}
impl INSObject for PHASEGlobalMetaParameterAsset {}
impl PNSObject for PHASEGlobalMetaParameterAsset {}
impl IPHASEGlobalMetaParameterAsset for PHASEGlobalMetaParameterAsset {}
pub trait IPHASEGlobalMetaParameterAsset: Sized + std::ops::Deref {
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
        msg_send!(&*objc2::runtime::AnyClass::get(c"PHASEGlobalMetaParameterAsset").unwrap(), new)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct PHASEAssetRegistry(pub id);
impl std::ops::Deref for PHASEAssetRegistry {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for PHASEAssetRegistry {}
impl PHASEAssetRegistry {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"PHASEAssetRegistry").unwrap(), alloc) })
    }
}
impl INSObject for PHASEAssetRegistry {}
impl PNSObject for PHASEAssetRegistry {}
impl std::convert::TryFrom<NSObject> for PHASEAssetRegistry {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<PHASEAssetRegistry, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"PHASEAssetRegistry").unwrap()) };
        if is_kind_of {
            Ok(PHASEAssetRegistry(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to PHASEAssetRegistry")
        }
    }
}
impl IPHASEAssetRegistry for PHASEAssetRegistry {}
pub trait IPHASEAssetRegistry: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn registerGlobalMetaParameter_error_(
        &self,
        metaParameterDefinition: PHASEMetaParameterDefinition,
        error: *mut NSError,
    ) -> PHASEGlobalMetaParameterAsset
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, registerGlobalMetaParameter : metaParameterDefinition, error : error)
    }
    unsafe fn registerSoundEventAssetWithRootNode_identifier_error_(
        &self,
        rootNode: PHASESoundEventNodeDefinition,
        identifier: NSString,
        error: *mut NSError,
    ) -> PHASESoundEventNodeAsset
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, registerSoundEventAssetWithRootNode : rootNode, identifier : identifier, error : error)
    }
    unsafe fn registerSoundAssetAtURL_identifier_assetType_channelLayout_normalizationMode_error_(
        &self,
        url: NSURL,
        identifier: NSString,
        assetType: PHASEAssetType,
        channelLayout: AVAudioChannelLayout,
        normalizationMode: PHASENormalizationMode,
        error: *mut NSError,
    ) -> PHASESoundAsset
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, registerSoundAssetAtURL : url, identifier : identifier, assetType : assetType, channelLayout : channelLayout, normalizationMode : normalizationMode, error : error)
    }
    unsafe fn registerSoundAssetWithData_identifier_format_normalizationMode_error_(
        &self,
        data: NSData,
        identifier: NSString,
        format: AVAudioFormat,
        normalizationMode: PHASENormalizationMode,
        error: *mut NSError,
    ) -> PHASESoundAsset
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, registerSoundAssetWithData : data, identifier : identifier, format : format, normalizationMode : normalizationMode, error : error)
    }
    unsafe fn unregisterAssetWithIdentifier_completion_(
        &self,
        identifier: NSString,
        handler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, unregisterAssetWithIdentifier : identifier, completion : handler)
    }
    unsafe fn assetForIdentifier_(&self, identifier: NSString) -> PHASEAsset
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, assetForIdentifier : identifier)
    }
    unsafe fn globalMetaParameters(&self) -> NSDictionary
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, globalMetaParameters)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"PHASEAssetRegistry").unwrap(), new)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct PHASECardioidDirectivityModelSubbandParameters(pub id);
impl std::ops::Deref for PHASECardioidDirectivityModelSubbandParameters {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for PHASECardioidDirectivityModelSubbandParameters {}
impl PHASECardioidDirectivityModelSubbandParameters {
    pub fn alloc() -> Self {
        Self(unsafe {
            msg_send!(&*objc2::runtime::AnyClass::get(c"PHASECardioidDirectivityModelSubbandParameters").unwrap(), alloc)
        })
    }
}
impl INSObject for PHASECardioidDirectivityModelSubbandParameters {}
impl PNSObject for PHASECardioidDirectivityModelSubbandParameters {}
impl std::convert::TryFrom<NSObject> for PHASECardioidDirectivityModelSubbandParameters {
    type Error = &'static str;
    fn try_from(
        parent: NSObject,
    ) -> Result<PHASECardioidDirectivityModelSubbandParameters, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"PHASECardioidDirectivityModelSubbandParameters").unwrap())
        };
        if is_kind_of {
            Ok(PHASECardioidDirectivityModelSubbandParameters(parent.0))
        } else {
            Err ("This NSObject cannot be downcasted to PHASECardioidDirectivityModelSubbandParameters" ,)
        }
    }
}
impl IPHASECardioidDirectivityModelSubbandParameters
    for PHASECardioidDirectivityModelSubbandParameters
{
}
pub trait IPHASECardioidDirectivityModelSubbandParameters: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn frequency(&self) -> f64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, frequency)
    }
    unsafe fn setFrequency_(&self, frequency: f64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setFrequency : frequency)
    }
    unsafe fn pattern(&self) -> f64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, pattern)
    }
    unsafe fn setPattern_(&self, pattern: f64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPattern : pattern)
    }
    unsafe fn sharpness(&self) -> f64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, sharpness)
    }
    unsafe fn setSharpness_(&self, sharpness: f64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSharpness : sharpness)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct PHASEConeDirectivityModelSubbandParameters(pub id);
impl std::ops::Deref for PHASEConeDirectivityModelSubbandParameters {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for PHASEConeDirectivityModelSubbandParameters {}
impl PHASEConeDirectivityModelSubbandParameters {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"PHASEConeDirectivityModelSubbandParameters").unwrap(), alloc) })
    }
}
impl INSObject for PHASEConeDirectivityModelSubbandParameters {}
impl PNSObject for PHASEConeDirectivityModelSubbandParameters {}
impl std::convert::TryFrom<NSObject> for PHASEConeDirectivityModelSubbandParameters {
    type Error = &'static str;
    fn try_from(
        parent: NSObject,
    ) -> Result<PHASEConeDirectivityModelSubbandParameters, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"PHASEConeDirectivityModelSubbandParameters").unwrap())
        };
        if is_kind_of {
            Ok(PHASEConeDirectivityModelSubbandParameters(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to PHASEConeDirectivityModelSubbandParameters")
        }
    }
}
impl IPHASEConeDirectivityModelSubbandParameters for PHASEConeDirectivityModelSubbandParameters {}
pub trait IPHASEConeDirectivityModelSubbandParameters: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn setInnerAngle_outerAngle_(&self, innerAngle: f64, outerAngle: f64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInnerAngle : innerAngle, outerAngle : outerAngle)
    }
    unsafe fn frequency(&self) -> f64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, frequency)
    }
    unsafe fn setFrequency_(&self, frequency: f64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setFrequency : frequency)
    }
    unsafe fn innerAngle(&self) -> f64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, innerAngle)
    }
    unsafe fn outerAngle(&self) -> f64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, outerAngle)
    }
    unsafe fn outerGain(&self) -> f64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, outerGain)
    }
    unsafe fn setOuterGain_(&self, outerGain: f64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setOuterGain : outerGain)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct PHASEDirectivityModelParameters(pub id);
impl std::ops::Deref for PHASEDirectivityModelParameters {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for PHASEDirectivityModelParameters {}
impl PHASEDirectivityModelParameters {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"PHASEDirectivityModelParameters").unwrap(), alloc) })
    }
}
impl INSObject for PHASEDirectivityModelParameters {}
impl PNSObject for PHASEDirectivityModelParameters {}
impl std::convert::TryFrom<NSObject> for PHASEDirectivityModelParameters {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<PHASEDirectivityModelParameters, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"PHASEDirectivityModelParameters").unwrap())
        };
        if is_kind_of {
            Ok(PHASEDirectivityModelParameters(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to PHASEDirectivityModelParameters")
        }
    }
}
impl IPHASEDirectivityModelParameters for PHASEDirectivityModelParameters {}
pub trait IPHASEDirectivityModelParameters: Sized + std::ops::Deref {
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
        msg_send!(&*objc2::runtime::AnyClass::get(c"PHASEDirectivityModelParameters").unwrap(), new)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct PHASECardioidDirectivityModelParameters(pub id);
impl std::ops::Deref for PHASECardioidDirectivityModelParameters {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for PHASECardioidDirectivityModelParameters {}
impl PHASECardioidDirectivityModelParameters {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"PHASECardioidDirectivityModelParameters").unwrap(), alloc) })
    }
}
impl IPHASEDirectivityModelParameters for PHASECardioidDirectivityModelParameters {}
impl From<PHASECardioidDirectivityModelParameters> for PHASEDirectivityModelParameters {
    fn from(child: PHASECardioidDirectivityModelParameters) -> PHASEDirectivityModelParameters {
        PHASEDirectivityModelParameters(child.0)
    }
}
impl std::convert::TryFrom<PHASEDirectivityModelParameters>
    for PHASECardioidDirectivityModelParameters
{
    type Error = &'static str;
    fn try_from(
        parent: PHASEDirectivityModelParameters,
    ) -> Result<PHASECardioidDirectivityModelParameters, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"PHASECardioidDirectivityModelParameters").unwrap())
        };
        if is_kind_of {
            Ok(PHASECardioidDirectivityModelParameters(parent.0))
        } else {
            Err ("This PHASEDirectivityModelParameters cannot be downcasted to PHASECardioidDirectivityModelParameters" ,)
        }
    }
}
impl INSObject for PHASECardioidDirectivityModelParameters {}
impl PNSObject for PHASECardioidDirectivityModelParameters {}
impl IPHASECardioidDirectivityModelParameters for PHASECardioidDirectivityModelParameters {}
pub trait IPHASECardioidDirectivityModelParameters: Sized + std::ops::Deref {
    unsafe fn initWithSubbandParameters_(&self, subbandParameters: NSArray) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithSubbandParameters : subbandParameters)
    }
    unsafe fn subbandParameters(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, subbandParameters)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct PHASEConeDirectivityModelParameters(pub id);
impl std::ops::Deref for PHASEConeDirectivityModelParameters {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for PHASEConeDirectivityModelParameters {}
impl PHASEConeDirectivityModelParameters {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"PHASEConeDirectivityModelParameters").unwrap(), alloc) })
    }
}
impl IPHASEDirectivityModelParameters for PHASEConeDirectivityModelParameters {}
impl std::convert::TryFrom<PHASEDirectivityModelParameters>
    for PHASEConeDirectivityModelParameters
{
    type Error = &'static str;
    fn try_from(
        parent: PHASEDirectivityModelParameters,
    ) -> Result<PHASEConeDirectivityModelParameters, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"PHASEConeDirectivityModelParameters").unwrap())
        };
        if is_kind_of {
            Ok(PHASEConeDirectivityModelParameters(parent.0))
        } else {
            Err ("This PHASEDirectivityModelParameters cannot be downcasted to PHASEConeDirectivityModelParameters" ,)
        }
    }
}
impl INSObject for PHASEConeDirectivityModelParameters {}
impl PNSObject for PHASEConeDirectivityModelParameters {}
impl IPHASEConeDirectivityModelParameters for PHASEConeDirectivityModelParameters {}
pub trait IPHASEConeDirectivityModelParameters: Sized + std::ops::Deref {
    unsafe fn initWithSubbandParameters_(&self, subbandParameters: NSArray) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithSubbandParameters : subbandParameters)
    }
    unsafe fn subbandParameters(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, subbandParameters)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct PHASEDistanceModelFadeOutParameters(pub id);
impl std::ops::Deref for PHASEDistanceModelFadeOutParameters {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for PHASEDistanceModelFadeOutParameters {}
impl PHASEDistanceModelFadeOutParameters {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"PHASEDistanceModelFadeOutParameters").unwrap(), alloc) })
    }
}
impl INSObject for PHASEDistanceModelFadeOutParameters {}
impl PNSObject for PHASEDistanceModelFadeOutParameters {}
impl std::convert::TryFrom<NSObject> for PHASEDistanceModelFadeOutParameters {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<PHASEDistanceModelFadeOutParameters, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"PHASEDistanceModelFadeOutParameters").unwrap())
        };
        if is_kind_of {
            Ok(PHASEDistanceModelFadeOutParameters(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to PHASEDistanceModelFadeOutParameters")
        }
    }
}
impl IPHASEDistanceModelFadeOutParameters for PHASEDistanceModelFadeOutParameters {}
pub trait IPHASEDistanceModelFadeOutParameters: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn initWithCullDistance_(&self, cullDistance: f64) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithCullDistance : cullDistance)
    }
    unsafe fn cullDistance(&self) -> f64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, cullDistance)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"PHASEDistanceModelFadeOutParameters").unwrap(), new)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct PHASEDistanceModelParameters(pub id);
impl std::ops::Deref for PHASEDistanceModelParameters {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for PHASEDistanceModelParameters {}
impl PHASEDistanceModelParameters {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"PHASEDistanceModelParameters").unwrap(), alloc) })
    }
}
impl INSObject for PHASEDistanceModelParameters {}
impl PNSObject for PHASEDistanceModelParameters {}
impl std::convert::TryFrom<NSObject> for PHASEDistanceModelParameters {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<PHASEDistanceModelParameters, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"PHASEDistanceModelParameters").unwrap()) };
        if is_kind_of {
            Ok(PHASEDistanceModelParameters(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to PHASEDistanceModelParameters")
        }
    }
}
impl IPHASEDistanceModelParameters for PHASEDistanceModelParameters {}
pub trait IPHASEDistanceModelParameters: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn fadeOutParameters(&self) -> PHASEDistanceModelFadeOutParameters
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, fadeOutParameters)
    }
    unsafe fn setFadeOutParameters_(&self, fadeOutParameters: PHASEDistanceModelFadeOutParameters)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setFadeOutParameters : fadeOutParameters)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"PHASEDistanceModelParameters").unwrap(), new)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct PHASEGeometricSpreadingDistanceModelParameters(pub id);
impl std::ops::Deref for PHASEGeometricSpreadingDistanceModelParameters {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for PHASEGeometricSpreadingDistanceModelParameters {}
impl PHASEGeometricSpreadingDistanceModelParameters {
    pub fn alloc() -> Self {
        Self(unsafe {
            msg_send!(&*objc2::runtime::AnyClass::get(c"PHASEGeometricSpreadingDistanceModelParameters").unwrap(), alloc)
        })
    }
}
impl IPHASEDistanceModelParameters for PHASEGeometricSpreadingDistanceModelParameters {}
impl From<PHASEGeometricSpreadingDistanceModelParameters> for PHASEDistanceModelParameters {
    fn from(child: PHASEGeometricSpreadingDistanceModelParameters) -> PHASEDistanceModelParameters {
        PHASEDistanceModelParameters(child.0)
    }
}
impl std::convert::TryFrom<PHASEDistanceModelParameters>
    for PHASEGeometricSpreadingDistanceModelParameters
{
    type Error = &'static str;
    fn try_from(
        parent: PHASEDistanceModelParameters,
    ) -> Result<PHASEGeometricSpreadingDistanceModelParameters, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"PHASEGeometricSpreadingDistanceModelParameters").unwrap())
        };
        if is_kind_of {
            Ok(PHASEGeometricSpreadingDistanceModelParameters(parent.0))
        } else {
            Err ("This PHASEDistanceModelParameters cannot be downcasted to PHASEGeometricSpreadingDistanceModelParameters" ,)
        }
    }
}
impl INSObject for PHASEGeometricSpreadingDistanceModelParameters {}
impl PNSObject for PHASEGeometricSpreadingDistanceModelParameters {}
impl IPHASEGeometricSpreadingDistanceModelParameters
    for PHASEGeometricSpreadingDistanceModelParameters
{
}
pub trait IPHASEGeometricSpreadingDistanceModelParameters: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn rolloffFactor(&self) -> f64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, rolloffFactor)
    }
    unsafe fn setRolloffFactor_(&self, rolloffFactor: f64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRolloffFactor : rolloffFactor)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct PHASEEnvelopeDistanceModelParameters(pub id);
impl std::ops::Deref for PHASEEnvelopeDistanceModelParameters {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for PHASEEnvelopeDistanceModelParameters {}
impl PHASEEnvelopeDistanceModelParameters {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"PHASEEnvelopeDistanceModelParameters").unwrap(), alloc) })
    }
}
impl IPHASEDistanceModelParameters for PHASEEnvelopeDistanceModelParameters {}
impl std::convert::TryFrom<PHASEDistanceModelParameters> for PHASEEnvelopeDistanceModelParameters {
    type Error = &'static str;
    fn try_from(
        parent: PHASEDistanceModelParameters,
    ) -> Result<PHASEEnvelopeDistanceModelParameters, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"PHASEEnvelopeDistanceModelParameters").unwrap())
        };
        if is_kind_of {
            Ok(PHASEEnvelopeDistanceModelParameters(parent.0))
        } else {
            Err ("This PHASEDistanceModelParameters cannot be downcasted to PHASEEnvelopeDistanceModelParameters" ,)
        }
    }
}
impl INSObject for PHASEEnvelopeDistanceModelParameters {}
impl PNSObject for PHASEEnvelopeDistanceModelParameters {}
impl IPHASEEnvelopeDistanceModelParameters for PHASEEnvelopeDistanceModelParameters {}
pub trait IPHASEEnvelopeDistanceModelParameters: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn initWithEnvelope_(&self, envelope: PHASEEnvelope) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithEnvelope : envelope)
    }
    unsafe fn envelope(&self) -> PHASEEnvelope
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, envelope)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"PHASEEnvelopeDistanceModelParameters").unwrap(), new)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct PHASEDucker(pub id);
impl std::ops::Deref for PHASEDucker {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for PHASEDucker {}
impl PHASEDucker {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"PHASEDucker").unwrap(), alloc) })
    }
}
impl INSObject for PHASEDucker {}
impl PNSObject for PHASEDucker {}
impl std::convert::TryFrom<NSObject> for PHASEDucker {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<PHASEDucker, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"PHASEDucker").unwrap()) };
        if is_kind_of {
            Ok(PHASEDucker(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to PHASEDucker")
        }
    }
}
impl IPHASEDucker for PHASEDucker {}
pub trait IPHASEDucker: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn initWithEngine_sourceGroups_targetGroups_gain_attackTime_releaseTime_attackCurve_releaseCurve_(
        &self,
        engine: PHASEEngine,
        sourceGroups: NSSet,
        targetGroups: NSSet,
        gain: f64,
        attackTime: f64,
        releaseTime: f64,
        attackCurve: PHASECurveType,
        releaseCurve: PHASECurveType,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithEngine : engine, sourceGroups : sourceGroups, targetGroups : targetGroups, gain : gain, attackTime : attackTime, releaseTime : releaseTime, attackCurve : attackCurve, releaseCurve : releaseCurve)
    }
    unsafe fn activate(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, activate)
    }
    unsafe fn deactivate(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, deactivate)
    }
    unsafe fn sourceGroups(&self) -> NSSet
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, sourceGroups)
    }
    unsafe fn targetGroups(&self) -> NSSet
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, targetGroups)
    }
    unsafe fn isActive(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isActive)
    }
    unsafe fn gain(&self) -> f64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, gain)
    }
    unsafe fn attackTime(&self) -> f64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, attackTime)
    }
    unsafe fn releaseTime(&self) -> f64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, releaseTime)
    }
    unsafe fn attackCurve(&self) -> PHASECurveType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, attackCurve)
    }
    unsafe fn releaseCurve(&self) -> PHASECurveType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, releaseCurve)
    }
    unsafe fn identifier(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, identifier)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"PHASEDucker").unwrap(), new)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct PHASEGroupPresetSetting(pub id);
impl std::ops::Deref for PHASEGroupPresetSetting {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for PHASEGroupPresetSetting {}
impl PHASEGroupPresetSetting {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"PHASEGroupPresetSetting").unwrap(), alloc) })
    }
}
impl INSObject for PHASEGroupPresetSetting {}
impl PNSObject for PHASEGroupPresetSetting {}
impl std::convert::TryFrom<NSObject> for PHASEGroupPresetSetting {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<PHASEGroupPresetSetting, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"PHASEGroupPresetSetting").unwrap()) };
        if is_kind_of {
            Ok(PHASEGroupPresetSetting(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to PHASEGroupPresetSetting")
        }
    }
}
impl IPHASEGroupPresetSetting for PHASEGroupPresetSetting {}
pub trait IPHASEGroupPresetSetting: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn initWithGain_rate_gainCurveType_rateCurveType_(
        &self,
        gain: f64,
        rate: f64,
        gainCurveType: PHASECurveType,
        rateCurveType: PHASECurveType,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithGain : gain, rate : rate, gainCurveType : gainCurveType, rateCurveType : rateCurveType)
    }
    unsafe fn gain(&self) -> f64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, gain)
    }
    unsafe fn rate(&self) -> f64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, rate)
    }
    unsafe fn gainCurveType(&self) -> PHASECurveType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, gainCurveType)
    }
    unsafe fn rateCurveType(&self) -> PHASECurveType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, rateCurveType)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"PHASEGroupPresetSetting").unwrap(), new)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct PHASEGroupPreset(pub id);
impl std::ops::Deref for PHASEGroupPreset {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for PHASEGroupPreset {}
impl PHASEGroupPreset {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"PHASEGroupPreset").unwrap(), alloc) })
    }
}
impl INSObject for PHASEGroupPreset {}
impl PNSObject for PHASEGroupPreset {}
impl std::convert::TryFrom<NSObject> for PHASEGroupPreset {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<PHASEGroupPreset, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"PHASEGroupPreset").unwrap()) };
        if is_kind_of {
            Ok(PHASEGroupPreset(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to PHASEGroupPreset")
        }
    }
}
impl IPHASEGroupPreset for PHASEGroupPreset {}
pub trait IPHASEGroupPreset: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn initWithEngine_settings_timeToTarget_timeToReset_(
        &self,
        engine: PHASEEngine,
        settings: NSDictionary,
        timeToTarget: f64,
        timeToReset: f64,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithEngine : engine, settings : settings, timeToTarget : timeToTarget, timeToReset : timeToReset)
    }
    unsafe fn activate(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, activate)
    }
    unsafe fn activateWithTimeToTargetOverride_(&self, timeToTargetOverride: f64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, activateWithTimeToTargetOverride : timeToTargetOverride)
    }
    unsafe fn deactivate(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, deactivate)
    }
    unsafe fn deactivateWithTimeToResetOverride_(&self, timeToResetOverride: f64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, deactivateWithTimeToResetOverride : timeToResetOverride)
    }
    unsafe fn settings(&self) -> NSDictionary
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, settings)
    }
    unsafe fn timeToTarget(&self) -> f64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, timeToTarget)
    }
    unsafe fn timeToReset(&self) -> f64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, timeToReset)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"PHASEGroupPreset").unwrap(), new)
    }
}
pub type PHASEMediumPreset = NSInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct PHASEMedium(pub id);
impl std::ops::Deref for PHASEMedium {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for PHASEMedium {}
impl PHASEMedium {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"PHASEMedium").unwrap(), alloc) })
    }
}
impl INSObject for PHASEMedium {}
impl PNSObject for PHASEMedium {}
impl std::convert::TryFrom<NSObject> for PHASEMedium {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<PHASEMedium, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"PHASEMedium").unwrap()) };
        if is_kind_of {
            Ok(PHASEMedium(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to PHASEMedium")
        }
    }
}
impl IPHASEMedium for PHASEMedium {}
pub trait IPHASEMedium: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn initWithEngine_preset_(
        &self,
        engine: PHASEEngine,
        preset: PHASEMediumPreset,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithEngine : engine, preset : preset)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"PHASEMedium").unwrap(), new)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct PHASEObject(pub id);
impl std::ops::Deref for PHASEObject {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for PHASEObject {}
impl PHASEObject {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"PHASEObject").unwrap(), alloc) })
    }
}
impl PNSCopying for PHASEObject {}
impl INSObject for PHASEObject {}
impl PNSObject for PHASEObject {}
impl std::convert::TryFrom<NSObject> for PHASEObject {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<PHASEObject, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"PHASEObject").unwrap()) };
        if is_kind_of {
            Ok(PHASEObject(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to PHASEObject")
        }
    }
}
impl IPHASEObject for PHASEObject {}
pub trait IPHASEObject: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn initWithEngine_(&self, engine: PHASEEngine) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithEngine : engine)
    }
    unsafe fn addChild_error_(&self, child: PHASEObject, error: *mut NSError) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, addChild : child, error : error)
    }
    unsafe fn removeChild_(&self, child: PHASEObject)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, removeChild : child)
    }
    unsafe fn removeChildren(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, removeChildren)
    }
    unsafe fn parent(&self) -> PHASEObject
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, parent)
    }
    unsafe fn children(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, children)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"PHASEObject").unwrap(), new)
    }
    unsafe fn right() -> simd_float3
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"PHASEObject").unwrap(), right)
    }
    unsafe fn up() -> simd_float3
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"PHASEObject").unwrap(), up)
    }
    unsafe fn forward() -> simd_float3
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"PHASEObject").unwrap(), forward)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct PHASESoundEvent(pub id);
impl std::ops::Deref for PHASESoundEvent {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for PHASESoundEvent {}
impl PHASESoundEvent {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"PHASESoundEvent").unwrap(), alloc) })
    }
}
impl INSObject for PHASESoundEvent {}
impl PNSObject for PHASESoundEvent {}
impl std::convert::TryFrom<NSObject> for PHASESoundEvent {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<PHASESoundEvent, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"PHASESoundEvent").unwrap()) };
        if is_kind_of {
            Ok(PHASESoundEvent(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to PHASESoundEvent")
        }
    }
}
impl IPHASESoundEvent for PHASESoundEvent {}
pub trait IPHASESoundEvent: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn initWithEngine_assetIdentifier_mixerParameters_error_(
        &self,
        engine: PHASEEngine,
        assetIdentifier: NSString,
        mixerParameters: PHASEMixerParameters,
        error: *mut NSError,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithEngine : engine, assetIdentifier : assetIdentifier, mixerParameters : mixerParameters, error : error)
    }
    unsafe fn initWithEngine_assetIdentifier_error_(
        &self,
        engine: PHASEEngine,
        assetIdentifier: NSString,
        error: *mut NSError,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithEngine : engine, assetIdentifier : assetIdentifier, error : error)
    }
    unsafe fn prepareWithCompletion_(&self, handler: *mut ::std::os::raw::c_void)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, prepareWithCompletion : handler)
    }
    unsafe fn startWithCompletion_(&self, handler: *mut ::std::os::raw::c_void)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, startWithCompletion : handler)
    }
    unsafe fn startAtTime_completion_(
        &self,
        when: AVAudioTime,
        handler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, startAtTime : when, completion : handler)
    }
    unsafe fn seekToTime_completion_(&self, time: f64, handler: *mut ::std::os::raw::c_void)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, seekToTime : time, completion : handler)
    }
    unsafe fn seekToTime_resumeAtEngineTime_completion_(
        &self,
        time: f64,
        engineTime: AVAudioTime,
        handler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, seekToTime : time, resumeAtEngineTime : engineTime, completion : handler)
    }
    unsafe fn pause(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, pause)
    }
    unsafe fn resume(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, resume)
    }
    unsafe fn resumeAtTime_(&self, time: AVAudioTime)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, resumeAtTime : time)
    }
    unsafe fn stopAndInvalidate(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, stopAndInvalidate)
    }
    unsafe fn renderingState(&self) -> PHASERenderingState
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, renderingState)
    }
    unsafe fn prepareState(&self) -> PHASESoundEventPrepareState
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, prepareState)
    }
    unsafe fn metaParameters(&self) -> NSDictionary
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, metaParameters)
    }
    unsafe fn mixers(&self) -> NSDictionary
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, mixers)
    }
    unsafe fn pushStreamNodes(&self) -> NSDictionary
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, pushStreamNodes)
    }
    unsafe fn pullStreamNodes(&self) -> NSDictionary
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, pullStreamNodes)
    }
    unsafe fn isIndefinite(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isIndefinite)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"PHASESoundEvent").unwrap(), new)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct PHASEEngine(pub id);
impl std::ops::Deref for PHASEEngine {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for PHASEEngine {}
impl PHASEEngine {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"PHASEEngine").unwrap(), alloc) })
    }
}
impl INSObject for PHASEEngine {}
impl PNSObject for PHASEEngine {}
impl std::convert::TryFrom<NSObject> for PHASEEngine {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<PHASEEngine, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"PHASEEngine").unwrap()) };
        if is_kind_of {
            Ok(PHASEEngine(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to PHASEEngine")
        }
    }
}
impl IPHASEEngine for PHASEEngine {}
pub trait IPHASEEngine: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn initWithUpdateMode_(&self, updateMode: PHASEUpdateMode) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithUpdateMode : updateMode)
    }
    unsafe fn initWithUpdateMode_renderingMode_(
        &self,
        updateMode: PHASEUpdateMode,
        renderingMode: PHASERenderingMode,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithUpdateMode : updateMode, renderingMode : renderingMode)
    }
    unsafe fn startAndReturnError_(&self, error: *mut NSError) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, startAndReturnError : error)
    }
    unsafe fn pause(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, pause)
    }
    unsafe fn stop(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, stop)
    }
    unsafe fn update(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, update)
    }
    unsafe fn outputSpatializationMode(&self) -> PHASESpatializationMode
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, outputSpatializationMode)
    }
    unsafe fn setOutputSpatializationMode_(&self, outputSpatializationMode: PHASESpatializationMode)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setOutputSpatializationMode : outputSpatializationMode)
    }
    unsafe fn renderingState(&self) -> PHASERenderingState
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, renderingState)
    }
    unsafe fn rootObject(&self) -> PHASEObject
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, rootObject)
    }
    unsafe fn defaultMedium(&self) -> PHASEMedium
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, defaultMedium)
    }
    unsafe fn setDefaultMedium_(&self, defaultMedium: PHASEMedium)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDefaultMedium : defaultMedium)
    }
    unsafe fn defaultReverbPreset(&self) -> PHASEReverbPreset
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, defaultReverbPreset)
    }
    unsafe fn setDefaultReverbPreset_(&self, defaultReverbPreset: PHASEReverbPreset)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDefaultReverbPreset : defaultReverbPreset)
    }
    unsafe fn unitsPerSecond(&self) -> f64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, unitsPerSecond)
    }
    unsafe fn setUnitsPerSecond_(&self, unitsPerSecond: f64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setUnitsPerSecond : unitsPerSecond)
    }
    unsafe fn unitsPerMeter(&self) -> f64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, unitsPerMeter)
    }
    unsafe fn setUnitsPerMeter_(&self, unitsPerMeter: f64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setUnitsPerMeter : unitsPerMeter)
    }
    unsafe fn assetRegistry(&self) -> PHASEAssetRegistry
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, assetRegistry)
    }
    unsafe fn soundEvents(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, soundEvents)
    }
    unsafe fn groups(&self) -> NSDictionary
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, groups)
    }
    unsafe fn duckers(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, duckers)
    }
    unsafe fn activeGroupPreset(&self) -> PHASEGroupPreset
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, activeGroupPreset)
    }
    unsafe fn lastRenderTime(&self) -> AVAudioTime
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, lastRenderTime)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"PHASEEngine").unwrap(), new)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct PHASEListener(pub id);
impl std::ops::Deref for PHASEListener {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for PHASEListener {}
impl PHASEListener {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"PHASEListener").unwrap(), alloc) })
    }
}
impl IPHASEObject for PHASEListener {}
impl PNSCopying for PHASEListener {}
impl From<PHASEListener> for PHASEObject {
    fn from(child: PHASEListener) -> PHASEObject {
        PHASEObject(child.0)
    }
}
impl std::convert::TryFrom<PHASEObject> for PHASEListener {
    type Error = &'static str;
    fn try_from(parent: PHASEObject) -> Result<PHASEListener, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"PHASEListener").unwrap()) };
        if is_kind_of {
            Ok(PHASEListener(parent.0))
        } else {
            Err("This PHASEObject cannot be downcasted to PHASEListener")
        }
    }
}
impl INSObject for PHASEListener {}
impl PNSObject for PHASEListener {}
impl IPHASEListener for PHASEListener {}
pub trait IPHASEListener: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn initWithEngine_(&self, engine: PHASEEngine) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithEngine : engine)
    }
    unsafe fn gain(&self) -> f64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, gain)
    }
    unsafe fn setGain_(&self, gain: f64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setGain : gain)
    }
    unsafe fn automaticHeadTrackingFlags(&self) -> PHASEAutomaticHeadTrackingFlags
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, automaticHeadTrackingFlags)
    }
    unsafe fn setAutomaticHeadTrackingFlags_(
        &self,
        automaticHeadTrackingFlags: PHASEAutomaticHeadTrackingFlags,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAutomaticHeadTrackingFlags : automaticHeadTrackingFlags)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"PHASEListener").unwrap(), new)
    }
}
pub type PHASEMaterialPreset = NSInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct PHASEMaterial(pub id);
impl std::ops::Deref for PHASEMaterial {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for PHASEMaterial {}
impl PHASEMaterial {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"PHASEMaterial").unwrap(), alloc) })
    }
}
impl INSObject for PHASEMaterial {}
impl PNSObject for PHASEMaterial {}
impl std::convert::TryFrom<NSObject> for PHASEMaterial {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<PHASEMaterial, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"PHASEMaterial").unwrap()) };
        if is_kind_of {
            Ok(PHASEMaterial(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to PHASEMaterial")
        }
    }
}
impl IPHASEMaterial for PHASEMaterial {}
pub trait IPHASEMaterial: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn initWithEngine_preset_(
        &self,
        engine: PHASEEngine,
        preset: PHASEMaterialPreset,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithEngine : engine, preset : preset)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"PHASEMaterial").unwrap(), new)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct PHASEShapeElement(pub id);
impl std::ops::Deref for PHASEShapeElement {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for PHASEShapeElement {}
impl PHASEShapeElement {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"PHASEShapeElement").unwrap(), alloc) })
    }
}
impl INSObject for PHASEShapeElement {}
impl PNSObject for PHASEShapeElement {}
impl std::convert::TryFrom<NSObject> for PHASEShapeElement {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<PHASEShapeElement, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"PHASEShapeElement").unwrap()) };
        if is_kind_of {
            Ok(PHASEShapeElement(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to PHASEShapeElement")
        }
    }
}
impl IPHASEShapeElement for PHASEShapeElement {}
pub trait IPHASEShapeElement: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn material(&self) -> PHASEMaterial
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, material)
    }
    unsafe fn setMaterial_(&self, material: PHASEMaterial)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMaterial : material)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"PHASEShapeElement").unwrap(), new)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct PHASEShape(pub id);
impl std::ops::Deref for PHASEShape {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for PHASEShape {}
impl PHASEShape {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"PHASEShape").unwrap(), alloc) })
    }
}
impl PNSCopying for PHASEShape {}
impl INSObject for PHASEShape {}
impl PNSObject for PHASEShape {}
impl std::convert::TryFrom<NSObject> for PHASEShape {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<PHASEShape, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"PHASEShape").unwrap()) };
        if is_kind_of {
            Ok(PHASEShape(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to PHASEShape")
        }
    }
}
impl IPHASEShape for PHASEShape {}
pub trait IPHASEShape: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn initWithEngine_mesh_(&self, engine: PHASEEngine, mesh: MDLMesh) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithEngine : engine, mesh : mesh)
    }
    unsafe fn initWithEngine_mesh_materials_(
        &self,
        engine: PHASEEngine,
        mesh: MDLMesh,
        materials: NSArray,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithEngine : engine, mesh : mesh, materials : materials)
    }
    unsafe fn elements(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, elements)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"PHASEShape").unwrap(), new)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct PHASEOccluder(pub id);
impl std::ops::Deref for PHASEOccluder {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for PHASEOccluder {}
impl PHASEOccluder {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"PHASEOccluder").unwrap(), alloc) })
    }
}
impl IPHASEObject for PHASEOccluder {}
impl PNSCopying for PHASEOccluder {}
impl std::convert::TryFrom<PHASEObject> for PHASEOccluder {
    type Error = &'static str;
    fn try_from(parent: PHASEObject) -> Result<PHASEOccluder, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"PHASEOccluder").unwrap()) };
        if is_kind_of {
            Ok(PHASEOccluder(parent.0))
        } else {
            Err("This PHASEObject cannot be downcasted to PHASEOccluder")
        }
    }
}
impl INSObject for PHASEOccluder {}
impl PNSObject for PHASEOccluder {}
impl IPHASEOccluder for PHASEOccluder {}
pub trait IPHASEOccluder: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn initWithEngine_(&self, engine: PHASEEngine) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithEngine : engine)
    }
    unsafe fn initWithEngine_shapes_(&self, engine: PHASEEngine, shapes: NSArray) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithEngine : engine, shapes : shapes)
    }
    unsafe fn shapes(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, shapes)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"PHASEOccluder").unwrap(), new)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct PHASESource(pub id);
impl std::ops::Deref for PHASESource {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for PHASESource {}
impl PHASESource {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"PHASESource").unwrap(), alloc) })
    }
}
impl IPHASEObject for PHASESource {}
impl PNSCopying for PHASESource {}
impl std::convert::TryFrom<PHASEObject> for PHASESource {
    type Error = &'static str;
    fn try_from(parent: PHASEObject) -> Result<PHASESource, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"PHASESource").unwrap()) };
        if is_kind_of {
            Ok(PHASESource(parent.0))
        } else {
            Err("This PHASEObject cannot be downcasted to PHASESource")
        }
    }
}
impl INSObject for PHASESource {}
impl PNSObject for PHASESource {}
impl IPHASESource for PHASESource {}
pub trait IPHASESource: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn initWithEngine_(&self, engine: PHASEEngine) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithEngine : engine)
    }
    unsafe fn initWithEngine_shapes_(&self, engine: PHASEEngine, shapes: NSArray) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithEngine : engine, shapes : shapes)
    }
    unsafe fn gain(&self) -> f64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, gain)
    }
    unsafe fn setGain_(&self, gain: f64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setGain : gain)
    }
    unsafe fn shapes(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, shapes)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"PHASESource").unwrap(), new)
    }
}
pub type PHASESpatialCategory = NSString;
pub type PHASESpatialPipelineFlags = NSUInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct PHASESpatialPipelineEntry(pub id);
impl std::ops::Deref for PHASESpatialPipelineEntry {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for PHASESpatialPipelineEntry {}
impl PHASESpatialPipelineEntry {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"PHASESpatialPipelineEntry").unwrap(), alloc) })
    }
}
impl INSObject for PHASESpatialPipelineEntry {}
impl PNSObject for PHASESpatialPipelineEntry {}
impl std::convert::TryFrom<NSObject> for PHASESpatialPipelineEntry {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<PHASESpatialPipelineEntry, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"PHASESpatialPipelineEntry").unwrap()) };
        if is_kind_of {
            Ok(PHASESpatialPipelineEntry(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to PHASESpatialPipelineEntry")
        }
    }
}
impl IPHASESpatialPipelineEntry for PHASESpatialPipelineEntry {}
pub trait IPHASESpatialPipelineEntry: Sized + std::ops::Deref {
    unsafe fn sendLevel(&self) -> f64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, sendLevel)
    }
    unsafe fn setSendLevel_(&self, sendLevel: f64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSendLevel : sendLevel)
    }
    unsafe fn sendLevelMetaParameterDefinition(&self) -> PHASENumberMetaParameterDefinition
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, sendLevelMetaParameterDefinition)
    }
    unsafe fn setSendLevelMetaParameterDefinition_(
        &self,
        sendLevelMetaParameterDefinition: PHASENumberMetaParameterDefinition,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSendLevelMetaParameterDefinition : sendLevelMetaParameterDefinition)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct PHASESpatialPipeline(pub id);
impl std::ops::Deref for PHASESpatialPipeline {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for PHASESpatialPipeline {}
impl PHASESpatialPipeline {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"PHASESpatialPipeline").unwrap(), alloc) })
    }
}
impl INSObject for PHASESpatialPipeline {}
impl PNSObject for PHASESpatialPipeline {}
impl std::convert::TryFrom<NSObject> for PHASESpatialPipeline {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<PHASESpatialPipeline, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"PHASESpatialPipeline").unwrap()) };
        if is_kind_of {
            Ok(PHASESpatialPipeline(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to PHASESpatialPipeline")
        }
    }
}
impl IPHASESpatialPipeline for PHASESpatialPipeline {}
pub trait IPHASESpatialPipeline: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn initWithFlags_(&self, flags: PHASESpatialPipelineFlags) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithFlags : flags)
    }
    unsafe fn flags(&self) -> PHASESpatialPipelineFlags
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, flags)
    }
    unsafe fn entries(&self) -> NSDictionary
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, entries)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"PHASESpatialPipeline").unwrap(), new)
    }
}
unsafe extern "C" {
    pub static PHASEErrorDomain: NSErrorDomain;
}
unsafe extern "C" {
    pub static PHASESoundEventErrorDomain: NSErrorDomain;
}
unsafe extern "C" {
    pub static PHASEAssetErrorDomain: NSErrorDomain;
}
unsafe extern "C" {
    pub static PHASESpatialCategoryDirectPathTransmission: PHASESpatialCategory;
}
unsafe extern "C" {
    pub static PHASESpatialCategoryEarlyReflections: PHASESpatialCategory;
}
unsafe extern "C" {
    pub static PHASESpatialCategoryLateReverb: PHASESpatialCategory;
}

unsafe impl objc2::encode::RefEncode for PHASENumericPair {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for PHASENumericPair {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for PHASEEnvelopeSegment {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for PHASEEnvelopeSegment {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for PHASEEnvelope {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for PHASEEnvelope {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for PHASEDefinition {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for PHASEDefinition {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for PHASEMetaParameterDefinition {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for PHASEMetaParameterDefinition {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for PHASENumberMetaParameterDefinition {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for PHASENumberMetaParameterDefinition {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for PHASEStringMetaParameterDefinition {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for PHASEStringMetaParameterDefinition {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for PHASEMappedMetaParameterDefinition {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for PHASEMappedMetaParameterDefinition {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for PHASEMetaParameter {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for PHASEMetaParameter {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for PHASENumberMetaParameter {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for PHASENumberMetaParameter {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for PHASEStringMetaParameter {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for PHASEStringMetaParameter {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for PHASEMixerDefinition {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for PHASEMixerDefinition {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for PHASESpatialMixerDefinition {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for PHASESpatialMixerDefinition {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for PHASEAmbientMixerDefinition {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for PHASEAmbientMixerDefinition {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for PHASEChannelMixerDefinition {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for PHASEChannelMixerDefinition {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for PHASEMixer {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for PHASEMixer {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for PHASEMixerParameters {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for PHASEMixerParameters {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for PHASEGroup {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for PHASEGroup {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for PHASESoundEventNodeDefinition {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for PHASESoundEventNodeDefinition {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for PHASEGeneratorNodeDefinition {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for PHASEGeneratorNodeDefinition {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for PHASESamplerNodeDefinition {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for PHASESamplerNodeDefinition {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for PHASEContainerNodeDefinition {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for PHASEContainerNodeDefinition {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for PHASEBlendNodeDefinition {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for PHASEBlendNodeDefinition {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for PHASESwitchNodeDefinition {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for PHASESwitchNodeDefinition {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for PHASERandomNodeDefinition {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for PHASERandomNodeDefinition {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for PHASEStreamNode {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for PHASEStreamNode {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for PHASEPushStreamNodeDefinition {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for PHASEPushStreamNodeDefinition {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for PHASEPushStreamNode {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for PHASEPushStreamNode {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for PHASEPullStreamNodeDefinition {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for PHASEPullStreamNodeDefinition {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for PHASEPullStreamNode {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for PHASEPullStreamNode {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for PHASEAsset {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for PHASEAsset {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for PHASESoundAsset {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for PHASESoundAsset {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for PHASESoundEventNodeAsset {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for PHASESoundEventNodeAsset {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for PHASEGlobalMetaParameterAsset {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for PHASEGlobalMetaParameterAsset {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for PHASEAssetRegistry {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for PHASEAssetRegistry {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for PHASECardioidDirectivityModelSubbandParameters {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for PHASECardioidDirectivityModelSubbandParameters {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for PHASEConeDirectivityModelSubbandParameters {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for PHASEConeDirectivityModelSubbandParameters {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for PHASEDirectivityModelParameters {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for PHASEDirectivityModelParameters {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for PHASECardioidDirectivityModelParameters {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for PHASECardioidDirectivityModelParameters {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for PHASEConeDirectivityModelParameters {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for PHASEConeDirectivityModelParameters {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for PHASEDistanceModelFadeOutParameters {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for PHASEDistanceModelFadeOutParameters {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for PHASEDistanceModelParameters {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for PHASEDistanceModelParameters {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for PHASEGeometricSpreadingDistanceModelParameters {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for PHASEGeometricSpreadingDistanceModelParameters {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for PHASEEnvelopeDistanceModelParameters {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for PHASEEnvelopeDistanceModelParameters {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for PHASEDucker {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for PHASEDucker {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for PHASEGroupPresetSetting {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for PHASEGroupPresetSetting {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for PHASEGroupPreset {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for PHASEGroupPreset {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for PHASEMedium {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for PHASEMedium {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for PHASEObject {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for PHASEObject {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for PHASESoundEvent {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for PHASESoundEvent {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for PHASEEngine {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for PHASEEngine {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for PHASEListener {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for PHASEListener {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for PHASEMaterial {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for PHASEMaterial {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for PHASEShapeElement {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for PHASEShapeElement {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for PHASEShape {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for PHASEShape {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for PHASEOccluder {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for PHASEOccluder {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for PHASESource {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for PHASESource {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for PHASESpatialPipelineEntry {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for PHASESpatialPipelineEntry {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for PHASESpatialPipeline {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for PHASESpatialPipeline {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
