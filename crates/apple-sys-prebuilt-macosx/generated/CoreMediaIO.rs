#[allow(unused_imports)]
use crate::objc::*;
#[allow(unused_imports)]
use crate::CoreFoundation::*;
#[allow(unused_imports)]
use crate::CoreMedia::*;
#[allow(unused_imports)]
use crate::CoreVideo::*;
#[allow(unused_imports)]
use crate::Foundation::*;
#[allow(unused_imports)]
use crate::IOKit::*;
#[allow(unused_imports)]
use libc::{id_t, pid_t};

#[allow(unused_imports)]
use objc2::msg_send;
pub type CMIOExtensionProperty = NSString;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CMIOExtensionPropertyAttributes(pub id);
impl std::ops::Deref for CMIOExtensionPropertyAttributes {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CMIOExtensionPropertyAttributes {}
impl CMIOExtensionPropertyAttributes {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CMIOExtensionPropertyAttributes").unwrap(), alloc) })
    }
}
impl PNSCopying for CMIOExtensionPropertyAttributes {}
impl PNSSecureCoding for CMIOExtensionPropertyAttributes {}
impl INSObject for CMIOExtensionPropertyAttributes {}
impl PNSObject for CMIOExtensionPropertyAttributes {}
impl std::convert::TryFrom<NSObject> for CMIOExtensionPropertyAttributes {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<CMIOExtensionPropertyAttributes, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CMIOExtensionPropertyAttributes").unwrap())
        };
        if is_kind_of {
            Ok(CMIOExtensionPropertyAttributes(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to CMIOExtensionPropertyAttributes")
        }
    }
}
impl<ObjectType: 'static> ICMIOExtensionPropertyAttributes<ObjectType>
    for CMIOExtensionPropertyAttributes
{
}
pub trait ICMIOExtensionPropertyAttributes<ObjectType: 'static>: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn initWithMinValue_maxValue_validValues_readOnly_(
        &self,
        minValue: id,
        maxValue: id,
        validValues: NSArray,
        readOnly: BOOL,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithMinValue : minValue, maxValue : maxValue, validValues : validValues, readOnly : readOnly)
    }
    unsafe fn minValue(&self) -> id
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, minValue)
    }
    unsafe fn maxValue(&self) -> id
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, maxValue)
    }
    unsafe fn validValues(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, validValues)
    }
    unsafe fn isReadOnly(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isReadOnly)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CMIOExtensionPropertyAttributes").unwrap(), new)
    }
    unsafe fn propertyAttributesWithMinValue_maxValue_validValues_readOnly_(
        minValue: id,
        maxValue: id,
        validValues: NSArray,
        readOnly: BOOL,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CMIOExtensionPropertyAttributes").unwrap(), propertyAttributesWithMinValue : minValue, maxValue : maxValue, validValues : validValues, readOnly : readOnly)
    }
    unsafe fn readOnlyPropertyAttribute() -> CMIOExtensionPropertyAttributes
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CMIOExtensionPropertyAttributes").unwrap(), readOnlyPropertyAttribute)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CMIOExtensionPropertyState(pub id);
impl std::ops::Deref for CMIOExtensionPropertyState {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CMIOExtensionPropertyState {}
impl CMIOExtensionPropertyState {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CMIOExtensionPropertyState").unwrap(), alloc) })
    }
}
impl PNSCopying for CMIOExtensionPropertyState {}
impl PNSSecureCoding for CMIOExtensionPropertyState {}
impl INSObject for CMIOExtensionPropertyState {}
impl PNSObject for CMIOExtensionPropertyState {}
impl std::convert::TryFrom<NSObject> for CMIOExtensionPropertyState {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<CMIOExtensionPropertyState, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CMIOExtensionPropertyState").unwrap()) };
        if is_kind_of {
            Ok(CMIOExtensionPropertyState(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to CMIOExtensionPropertyState")
        }
    }
}
impl<ObjectType: 'static> ICMIOExtensionPropertyState<ObjectType> for CMIOExtensionPropertyState {}
pub trait ICMIOExtensionPropertyState<ObjectType: 'static>: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn initWithValue_(&self, value: id) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithValue : value)
    }
    unsafe fn initWithValue_attributes_(
        &self,
        value: id,
        attributes: CMIOExtensionPropertyAttributes,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithValue : value, attributes : attributes)
    }
    unsafe fn value(&self) -> id
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, value)
    }
    unsafe fn attributes(&self) -> CMIOExtensionPropertyAttributes
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, attributes)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CMIOExtensionPropertyState").unwrap(), new)
    }
    unsafe fn propertyStateWithValue_(value: id) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CMIOExtensionPropertyState").unwrap(), propertyStateWithValue : value)
    }
    unsafe fn propertyStateWithValue_attributes_(
        value: id,
        attributes: CMIOExtensionPropertyAttributes,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CMIOExtensionPropertyState").unwrap(), propertyStateWithValue : value, attributes : attributes)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CMIOExtensionStreamCustomClockConfiguration(pub id);
impl std::ops::Deref for CMIOExtensionStreamCustomClockConfiguration {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CMIOExtensionStreamCustomClockConfiguration {}
impl CMIOExtensionStreamCustomClockConfiguration {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CMIOExtensionStreamCustomClockConfiguration").unwrap(), alloc) })
    }
}
impl PNSCopying for CMIOExtensionStreamCustomClockConfiguration {}
impl PNSSecureCoding for CMIOExtensionStreamCustomClockConfiguration {}
impl INSObject for CMIOExtensionStreamCustomClockConfiguration {}
impl PNSObject for CMIOExtensionStreamCustomClockConfiguration {}
impl std::convert::TryFrom<NSObject> for CMIOExtensionStreamCustomClockConfiguration {
    type Error = &'static str;
    fn try_from(
        parent: NSObject,
    ) -> Result<CMIOExtensionStreamCustomClockConfiguration, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CMIOExtensionStreamCustomClockConfiguration").unwrap())
        };
        if is_kind_of {
            Ok(CMIOExtensionStreamCustomClockConfiguration(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to CMIOExtensionStreamCustomClockConfiguration")
        }
    }
}
impl ICMIOExtensionStreamCustomClockConfiguration for CMIOExtensionStreamCustomClockConfiguration {}
pub trait ICMIOExtensionStreamCustomClockConfiguration: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn initWithClockName_sourceIdentifier_getTimeCallMinimumInterval_numberOfEventsForRateSmoothing_numberOfAveragesForRateSmoothing_(
        &self,
        clockName: NSString,
        sourceIdentifier: NSUUID,
        getTimeCallMinimumInterval: CMTime,
        numberOfEventsForRateSmoothing: u32,
        numberOfAveragesForRateSmoothing: u32,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithClockName : clockName, sourceIdentifier : sourceIdentifier, getTimeCallMinimumInterval : getTimeCallMinimumInterval, numberOfEventsForRateSmoothing : numberOfEventsForRateSmoothing, numberOfAveragesForRateSmoothing : numberOfAveragesForRateSmoothing)
    }
    unsafe fn clockName(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, clockName)
    }
    unsafe fn sourceIdentifier(&self) -> NSUUID
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, sourceIdentifier)
    }
    unsafe fn getTimeCallMinimumInterval(&self) -> CMTime
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, getTimeCallMinimumInterval)
    }
    unsafe fn numberOfEventsForRateSmoothing(&self) -> u32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, numberOfEventsForRateSmoothing)
    }
    unsafe fn numberOfAveragesForRateSmoothing(&self) -> u32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, numberOfAveragesForRateSmoothing)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CMIOExtensionStreamCustomClockConfiguration").unwrap(), new)
    }
    unsafe fn customClockConfigurationWithClockName_sourceIdentifier_getTimeCallMinimumInterval_numberOfEventsForRateSmoothing_numberOfAveragesForRateSmoothing_(
        clockName: NSString,
        sourceIdentifier: NSUUID,
        getTimeCallMinimumInterval: CMTime,
        numberOfEventsForRateSmoothing: u32,
        numberOfAveragesForRateSmoothing: u32,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CMIOExtensionStreamCustomClockConfiguration").unwrap(), customClockConfigurationWithClockName : clockName, sourceIdentifier : sourceIdentifier, getTimeCallMinimumInterval : getTimeCallMinimumInterval, numberOfEventsForRateSmoothing : numberOfEventsForRateSmoothing, numberOfAveragesForRateSmoothing : numberOfAveragesForRateSmoothing)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CMIOExtensionStreamFormat(pub id);
impl std::ops::Deref for CMIOExtensionStreamFormat {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CMIOExtensionStreamFormat {}
impl CMIOExtensionStreamFormat {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CMIOExtensionStreamFormat").unwrap(), alloc) })
    }
}
impl PNSCopying for CMIOExtensionStreamFormat {}
impl PNSSecureCoding for CMIOExtensionStreamFormat {}
impl INSObject for CMIOExtensionStreamFormat {}
impl PNSObject for CMIOExtensionStreamFormat {}
impl std::convert::TryFrom<NSObject> for CMIOExtensionStreamFormat {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<CMIOExtensionStreamFormat, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CMIOExtensionStreamFormat").unwrap()) };
        if is_kind_of {
            Ok(CMIOExtensionStreamFormat(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to CMIOExtensionStreamFormat")
        }
    }
}
impl ICMIOExtensionStreamFormat for CMIOExtensionStreamFormat {}
pub trait ICMIOExtensionStreamFormat: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn initWithFormatDescription_maxFrameDuration_minFrameDuration_validFrameDurations_(
        &self,
        formatDescription: CMFormatDescriptionRef,
        maxFrameDuration: CMTime,
        minFrameDuration: CMTime,
        validFrameDurations: NSArray,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithFormatDescription : formatDescription, maxFrameDuration : maxFrameDuration, minFrameDuration : minFrameDuration, validFrameDurations : validFrameDurations)
    }
    unsafe fn formatDescription(&self) -> CMFormatDescriptionRef
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, formatDescription)
    }
    unsafe fn minFrameDuration(&self) -> CMTime
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, minFrameDuration)
    }
    unsafe fn maxFrameDuration(&self) -> CMTime
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, maxFrameDuration)
    }
    unsafe fn validFrameDurations(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, validFrameDurations)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CMIOExtensionStreamFormat").unwrap(), new)
    }
    unsafe fn streamFormatWithFormatDescription_maxFrameDuration_minFrameDuration_validFrameDurations_(
        formatDescription: CMFormatDescriptionRef,
        maxFrameDuration: CMTime,
        minFrameDuration: CMTime,
        validFrameDurations: NSArray,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CMIOExtensionStreamFormat").unwrap(), streamFormatWithFormatDescription : formatDescription, maxFrameDuration : maxFrameDuration, minFrameDuration : minFrameDuration, validFrameDurations : validFrameDurations)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CMIOExtensionScheduledOutput(pub id);
impl std::ops::Deref for CMIOExtensionScheduledOutput {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CMIOExtensionScheduledOutput {}
impl CMIOExtensionScheduledOutput {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CMIOExtensionScheduledOutput").unwrap(), alloc) })
    }
}
impl PNSCopying for CMIOExtensionScheduledOutput {}
impl PNSSecureCoding for CMIOExtensionScheduledOutput {}
impl INSObject for CMIOExtensionScheduledOutput {}
impl PNSObject for CMIOExtensionScheduledOutput {}
impl std::convert::TryFrom<NSObject> for CMIOExtensionScheduledOutput {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<CMIOExtensionScheduledOutput, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CMIOExtensionScheduledOutput").unwrap()) };
        if is_kind_of {
            Ok(CMIOExtensionScheduledOutput(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to CMIOExtensionScheduledOutput")
        }
    }
}
impl ICMIOExtensionScheduledOutput for CMIOExtensionScheduledOutput {}
pub trait ICMIOExtensionScheduledOutput: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn initWithSequenceNumber_hostTimeInNanoseconds_(
        &self,
        sequenceNumber: u64,
        hostTimeInNanoseconds: u64,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithSequenceNumber : sequenceNumber, hostTimeInNanoseconds : hostTimeInNanoseconds)
    }
    unsafe fn sequenceNumber(&self) -> u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, sequenceNumber)
    }
    unsafe fn hostTimeInNanoseconds(&self) -> u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, hostTimeInNanoseconds)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CMIOExtensionScheduledOutput").unwrap(), new)
    }
    unsafe fn scheduledOutputWithSequenceNumber_hostTimeInNanoseconds_(
        sequenceNumber: u64,
        hostTimeInNanoseconds: u64,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CMIOExtensionScheduledOutput").unwrap(), scheduledOutputWithSequenceNumber : sequenceNumber, hostTimeInNanoseconds : hostTimeInNanoseconds)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CMIOExtensionClient(pub id);
impl std::ops::Deref for CMIOExtensionClient {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CMIOExtensionClient {}
impl CMIOExtensionClient {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CMIOExtensionClient").unwrap(), alloc) })
    }
}
impl PNSCopying for CMIOExtensionClient {}
impl INSObject for CMIOExtensionClient {}
impl PNSObject for CMIOExtensionClient {}
impl std::convert::TryFrom<NSObject> for CMIOExtensionClient {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<CMIOExtensionClient, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CMIOExtensionClient").unwrap()) };
        if is_kind_of {
            Ok(CMIOExtensionClient(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to CMIOExtensionClient")
        }
    }
}
impl ICMIOExtensionClient for CMIOExtensionClient {}
pub trait ICMIOExtensionClient: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn clientID(&self) -> NSUUID
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, clientID)
    }
    unsafe fn signingID(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, signingID)
    }
    unsafe fn pid(&self) -> pid_t
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, pid)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CMIOExtensionClient").unwrap(), new)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CMIOExtensionProviderProperties(pub id);
impl std::ops::Deref for CMIOExtensionProviderProperties {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CMIOExtensionProviderProperties {}
impl CMIOExtensionProviderProperties {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CMIOExtensionProviderProperties").unwrap(), alloc) })
    }
}
impl INSObject for CMIOExtensionProviderProperties {}
impl PNSObject for CMIOExtensionProviderProperties {}
impl std::convert::TryFrom<NSObject> for CMIOExtensionProviderProperties {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<CMIOExtensionProviderProperties, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CMIOExtensionProviderProperties").unwrap())
        };
        if is_kind_of {
            Ok(CMIOExtensionProviderProperties(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to CMIOExtensionProviderProperties")
        }
    }
}
impl ICMIOExtensionProviderProperties for CMIOExtensionProviderProperties {}
pub trait ICMIOExtensionProviderProperties: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn initWithDictionary_(&self, propertiesDictionary: NSDictionary) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithDictionary : propertiesDictionary)
    }
    unsafe fn setPropertyState_forProperty_(
        &self,
        propertyState: CMIOExtensionPropertyState,
        property: NSString,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPropertyState : propertyState, forProperty : property)
    }
    unsafe fn name(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, name)
    }
    unsafe fn setName_(&self, name: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setName : name)
    }
    unsafe fn manufacturer(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, manufacturer)
    }
    unsafe fn setManufacturer_(&self, manufacturer: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setManufacturer : manufacturer)
    }
    unsafe fn propertiesDictionary(&self) -> NSDictionary
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, propertiesDictionary)
    }
    unsafe fn setPropertiesDictionary_(&self, propertiesDictionary: NSDictionary)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPropertiesDictionary : propertiesDictionary)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CMIOExtensionProviderProperties").unwrap(), new)
    }
    unsafe fn providerPropertiesWithDictionary_(propertiesDictionary: NSDictionary) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CMIOExtensionProviderProperties").unwrap(), providerPropertiesWithDictionary : propertiesDictionary)
    }
}
pub trait PCMIOExtensionProviderSource: Sized + std::ops::Deref {
    unsafe fn connectClient_error_(
        &self,
        client: CMIOExtensionClient,
        outError: *mut NSError,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, connectClient : client, error : outError)
    }
    unsafe fn disconnectClient_(&self, client: CMIOExtensionClient)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, disconnectClient : client)
    }
    unsafe fn providerPropertiesForProperties_error_(
        &self,
        properties: NSSet,
        outError: *mut NSError,
    ) -> CMIOExtensionProviderProperties
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, providerPropertiesForProperties : properties, error : outError)
    }
    unsafe fn setProviderProperties_error_(
        &self,
        providerProperties: CMIOExtensionProviderProperties,
        outError: *mut NSError,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setProviderProperties : providerProperties, error : outError)
    }
    unsafe fn availableProperties(&self) -> NSSet
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, availableProperties)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CMIOExtensionProvider(pub id);
impl std::ops::Deref for CMIOExtensionProvider {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CMIOExtensionProvider {}
impl CMIOExtensionProvider {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CMIOExtensionProvider").unwrap(), alloc) })
    }
}
impl INSObject for CMIOExtensionProvider {}
impl PNSObject for CMIOExtensionProvider {}
impl std::convert::TryFrom<NSObject> for CMIOExtensionProvider {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<CMIOExtensionProvider, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CMIOExtensionProvider").unwrap()) };
        if is_kind_of {
            Ok(CMIOExtensionProvider(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to CMIOExtensionProvider")
        }
    }
}
impl ICMIOExtensionProvider for CMIOExtensionProvider {}
pub trait ICMIOExtensionProvider: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn initWithSource_clientQueue_(
        &self,
        source: *mut u64,
        clientQueue: NSObject,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithSource : source, clientQueue : clientQueue)
    }
    unsafe fn addDevice_error_(&self, device: CMIOExtensionDevice, outError: *mut NSError) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, addDevice : device, error : outError)
    }
    unsafe fn removeDevice_error_(
        &self,
        device: CMIOExtensionDevice,
        outError: *mut NSError,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, removeDevice : device, error : outError)
    }
    unsafe fn notifyPropertiesChanged_(&self, propertyStates: NSDictionary)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, notifyPropertiesChanged : propertyStates)
    }
    unsafe fn source(&self) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, source)
    }
    unsafe fn clientQueue(&self) -> dispatch_queue_t
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, clientQueue)
    }
    unsafe fn connectedClients(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, connectedClients)
    }
    unsafe fn devices(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, devices)
    }
    unsafe fn startServiceWithProvider_(provider: CMIOExtensionProvider)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CMIOExtensionProvider").unwrap(), startServiceWithProvider : provider)
    }
    unsafe fn stopServiceWithProvider_(provider: CMIOExtensionProvider)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CMIOExtensionProvider").unwrap(), stopServiceWithProvider : provider)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CMIOExtensionProvider").unwrap(), new)
    }
    unsafe fn providerWithSource_clientQueue_(
        source: *mut u64,
        clientQueue: NSObject,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CMIOExtensionProvider").unwrap(), providerWithSource : source, clientQueue : clientQueue)
    }
}
impl CMIOExtensionProvider_SignalHandling for CMIOExtensionProvider {}
pub trait CMIOExtensionProvider_SignalHandling: Sized + std::ops::Deref {
    unsafe fn ignoreSIGTERM()
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CMIOExtensionProvider").unwrap(), ignoreSIGTERM)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CMIOExtensionDeviceProperties(pub id);
impl std::ops::Deref for CMIOExtensionDeviceProperties {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CMIOExtensionDeviceProperties {}
impl CMIOExtensionDeviceProperties {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CMIOExtensionDeviceProperties").unwrap(), alloc) })
    }
}
impl INSObject for CMIOExtensionDeviceProperties {}
impl PNSObject for CMIOExtensionDeviceProperties {}
impl std::convert::TryFrom<NSObject> for CMIOExtensionDeviceProperties {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<CMIOExtensionDeviceProperties, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CMIOExtensionDeviceProperties").unwrap())
        };
        if is_kind_of {
            Ok(CMIOExtensionDeviceProperties(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to CMIOExtensionDeviceProperties")
        }
    }
}
impl ICMIOExtensionDeviceProperties for CMIOExtensionDeviceProperties {}
pub trait ICMIOExtensionDeviceProperties: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn initWithDictionary_(&self, propertiesDictionary: NSDictionary) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithDictionary : propertiesDictionary)
    }
    unsafe fn setPropertyState_forProperty_(
        &self,
        propertyState: CMIOExtensionPropertyState,
        property: NSString,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPropertyState : propertyState, forProperty : property)
    }
    unsafe fn model(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, model)
    }
    unsafe fn setModel_(&self, model: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setModel : model)
    }
    unsafe fn suspended(&self) -> NSNumber
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, suspended)
    }
    unsafe fn setSuspended_(&self, suspended: NSNumber)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSuspended : suspended)
    }
    unsafe fn transportType(&self) -> NSNumber
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, transportType)
    }
    unsafe fn setTransportType_(&self, transportType: NSNumber)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setTransportType : transportType)
    }
    unsafe fn linkedCoreAudioDeviceUID(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, linkedCoreAudioDeviceUID)
    }
    unsafe fn setLinkedCoreAudioDeviceUID_(&self, linkedCoreAudioDeviceUID: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setLinkedCoreAudioDeviceUID : linkedCoreAudioDeviceUID)
    }
    unsafe fn propertiesDictionary(&self) -> NSDictionary
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, propertiesDictionary)
    }
    unsafe fn setPropertiesDictionary_(&self, propertiesDictionary: NSDictionary)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPropertiesDictionary : propertiesDictionary)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CMIOExtensionDeviceProperties").unwrap(), new)
    }
    unsafe fn devicePropertiesWithDictionary_(propertiesDictionary: NSDictionary) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CMIOExtensionDeviceProperties").unwrap(), devicePropertiesWithDictionary : propertiesDictionary)
    }
}
pub trait PCMIOExtensionDeviceSource: Sized + std::ops::Deref {
    unsafe fn devicePropertiesForProperties_error_(
        &self,
        properties: NSSet,
        outError: *mut NSError,
    ) -> CMIOExtensionDeviceProperties
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, devicePropertiesForProperties : properties, error : outError)
    }
    unsafe fn setDeviceProperties_error_(
        &self,
        deviceProperties: CMIOExtensionDeviceProperties,
        outError: *mut NSError,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDeviceProperties : deviceProperties, error : outError)
    }
    unsafe fn availableProperties(&self) -> NSSet
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, availableProperties)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CMIOExtensionDevice(pub id);
impl std::ops::Deref for CMIOExtensionDevice {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CMIOExtensionDevice {}
impl CMIOExtensionDevice {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CMIOExtensionDevice").unwrap(), alloc) })
    }
}
impl INSObject for CMIOExtensionDevice {}
impl PNSObject for CMIOExtensionDevice {}
impl std::convert::TryFrom<NSObject> for CMIOExtensionDevice {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<CMIOExtensionDevice, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CMIOExtensionDevice").unwrap()) };
        if is_kind_of {
            Ok(CMIOExtensionDevice(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to CMIOExtensionDevice")
        }
    }
}
impl ICMIOExtensionDevice for CMIOExtensionDevice {}
pub trait ICMIOExtensionDevice: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn initWithLocalizedName_deviceID_legacyDeviceID_source_(
        &self,
        localizedName: NSString,
        deviceID: NSUUID,
        legacyDeviceID: NSString,
        source: *mut u64,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithLocalizedName : localizedName, deviceID : deviceID, legacyDeviceID : legacyDeviceID, source : source)
    }
    unsafe fn initWithLocalizedName_deviceID_source_(
        &self,
        localizedName: NSString,
        deviceID: NSUUID,
        source: *mut u64,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithLocalizedName : localizedName, deviceID : deviceID, source : source)
    }
    unsafe fn addStream_error_(&self, stream: CMIOExtensionStream, outError: *mut NSError) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, addStream : stream, error : outError)
    }
    unsafe fn removeStream_error_(
        &self,
        stream: CMIOExtensionStream,
        outError: *mut NSError,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, removeStream : stream, error : outError)
    }
    unsafe fn notifyPropertiesChanged_(&self, propertyStates: NSDictionary)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, notifyPropertiesChanged : propertyStates)
    }
    unsafe fn localizedName(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, localizedName)
    }
    unsafe fn deviceID(&self) -> NSUUID
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, deviceID)
    }
    unsafe fn legacyDeviceID(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, legacyDeviceID)
    }
    unsafe fn source(&self) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, source)
    }
    unsafe fn streams(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, streams)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CMIOExtensionDevice").unwrap(), new)
    }
    unsafe fn deviceWithLocalizedName_deviceID_legacyDeviceID_source_(
        localizedName: NSString,
        deviceID: NSUUID,
        legacyDeviceID: NSString,
        source: *mut u64,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CMIOExtensionDevice").unwrap(), deviceWithLocalizedName : localizedName, deviceID : deviceID, legacyDeviceID : legacyDeviceID, source : source)
    }
    unsafe fn deviceWithLocalizedName_deviceID_source_(
        localizedName: NSString,
        deviceID: NSUUID,
        source: *mut u64,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CMIOExtensionDevice").unwrap(), deviceWithLocalizedName : localizedName, deviceID : deviceID, source : source)
    }
}
pub type CMIOExtensionStreamDirection = NSInteger;
pub type CMIOExtensionStreamClockType = NSInteger;
pub type CMIOExtensionStreamDiscontinuityFlags = u32;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CMIOExtensionStreamProperties(pub id);
impl std::ops::Deref for CMIOExtensionStreamProperties {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CMIOExtensionStreamProperties {}
impl CMIOExtensionStreamProperties {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CMIOExtensionStreamProperties").unwrap(), alloc) })
    }
}
impl INSObject for CMIOExtensionStreamProperties {}
impl PNSObject for CMIOExtensionStreamProperties {}
impl std::convert::TryFrom<NSObject> for CMIOExtensionStreamProperties {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<CMIOExtensionStreamProperties, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CMIOExtensionStreamProperties").unwrap())
        };
        if is_kind_of {
            Ok(CMIOExtensionStreamProperties(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to CMIOExtensionStreamProperties")
        }
    }
}
impl ICMIOExtensionStreamProperties for CMIOExtensionStreamProperties {}
pub trait ICMIOExtensionStreamProperties: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn initWithDictionary_(&self, propertiesDictionary: NSDictionary) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithDictionary : propertiesDictionary)
    }
    unsafe fn setPropertyState_forProperty_(
        &self,
        propertyState: CMIOExtensionPropertyState,
        property: NSString,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPropertyState : propertyState, forProperty : property)
    }
    unsafe fn activeFormatIndex(&self) -> NSNumber
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, activeFormatIndex)
    }
    unsafe fn setActiveFormatIndex_(&self, activeFormatIndex: NSNumber)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setActiveFormatIndex : activeFormatIndex)
    }
    unsafe fn frameDuration(&self) -> NSDictionary
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, frameDuration)
    }
    unsafe fn setFrameDuration_(&self, frameDuration: NSDictionary)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setFrameDuration : frameDuration)
    }
    unsafe fn maxFrameDuration(&self) -> NSDictionary
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, maxFrameDuration)
    }
    unsafe fn setMaxFrameDuration_(&self, maxFrameDuration: NSDictionary)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMaxFrameDuration : maxFrameDuration)
    }
    unsafe fn sinkBufferQueueSize(&self) -> NSNumber
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, sinkBufferQueueSize)
    }
    unsafe fn setSinkBufferQueueSize_(&self, sinkBufferQueueSize: NSNumber)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSinkBufferQueueSize : sinkBufferQueueSize)
    }
    unsafe fn sinkBuffersRequiredForStartup(&self) -> NSNumber
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, sinkBuffersRequiredForStartup)
    }
    unsafe fn setSinkBuffersRequiredForStartup_(&self, sinkBuffersRequiredForStartup: NSNumber)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSinkBuffersRequiredForStartup : sinkBuffersRequiredForStartup)
    }
    unsafe fn sinkBufferUnderrunCount(&self) -> NSNumber
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, sinkBufferUnderrunCount)
    }
    unsafe fn setSinkBufferUnderrunCount_(&self, sinkBufferUnderrunCount: NSNumber)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSinkBufferUnderrunCount : sinkBufferUnderrunCount)
    }
    unsafe fn sinkEndOfData(&self) -> NSNumber
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, sinkEndOfData)
    }
    unsafe fn setSinkEndOfData_(&self, sinkEndOfData: NSNumber)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSinkEndOfData : sinkEndOfData)
    }
    unsafe fn propertiesDictionary(&self) -> NSDictionary
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, propertiesDictionary)
    }
    unsafe fn setPropertiesDictionary_(&self, propertiesDictionary: NSDictionary)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPropertiesDictionary : propertiesDictionary)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CMIOExtensionStreamProperties").unwrap(), new)
    }
    unsafe fn streamPropertiesWithDictionary_(propertiesDictionary: NSDictionary) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CMIOExtensionStreamProperties").unwrap(), streamPropertiesWithDictionary : propertiesDictionary)
    }
}
pub trait PCMIOExtensionStreamSource: Sized + std::ops::Deref {
    unsafe fn streamPropertiesForProperties_error_(
        &self,
        properties: NSSet,
        outError: *mut NSError,
    ) -> CMIOExtensionStreamProperties
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, streamPropertiesForProperties : properties, error : outError)
    }
    unsafe fn setStreamProperties_error_(
        &self,
        streamProperties: CMIOExtensionStreamProperties,
        outError: *mut NSError,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setStreamProperties : streamProperties, error : outError)
    }
    unsafe fn authorizedToStartStreamForClient_(&self, client: CMIOExtensionClient) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, authorizedToStartStreamForClient : client)
    }
    unsafe fn startStreamAndReturnError_(&self, outError: *mut NSError) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, startStreamAndReturnError : outError)
    }
    unsafe fn stopStreamAndReturnError_(&self, outError: *mut NSError) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, stopStreamAndReturnError : outError)
    }
    unsafe fn formats(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, formats)
    }
    unsafe fn availableProperties(&self) -> NSSet
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, availableProperties)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CMIOExtensionStream(pub id);
impl std::ops::Deref for CMIOExtensionStream {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CMIOExtensionStream {}
impl CMIOExtensionStream {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CMIOExtensionStream").unwrap(), alloc) })
    }
}
impl INSObject for CMIOExtensionStream {}
impl PNSObject for CMIOExtensionStream {}
impl std::convert::TryFrom<NSObject> for CMIOExtensionStream {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<CMIOExtensionStream, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CMIOExtensionStream").unwrap()) };
        if is_kind_of {
            Ok(CMIOExtensionStream(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to CMIOExtensionStream")
        }
    }
}
impl ICMIOExtensionStream for CMIOExtensionStream {}
pub trait ICMIOExtensionStream: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn initWithLocalizedName_streamID_direction_clockType_source_(
        &self,
        localizedName: NSString,
        streamID: NSUUID,
        direction: CMIOExtensionStreamDirection,
        clockType: CMIOExtensionStreamClockType,
        source: *mut u64,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithLocalizedName : localizedName, streamID : streamID, direction : direction, clockType : clockType, source : source)
    }
    unsafe fn initWithLocalizedName_streamID_direction_customClockConfiguration_source_(
        &self,
        localizedName: NSString,
        streamID: NSUUID,
        direction: CMIOExtensionStreamDirection,
        customClockConfiguration: CMIOExtensionStreamCustomClockConfiguration,
        source: *mut u64,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithLocalizedName : localizedName, streamID : streamID, direction : direction, customClockConfiguration : customClockConfiguration, source : source)
    }
    unsafe fn notifyPropertiesChanged_(&self, propertyStates: NSDictionary)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, notifyPropertiesChanged : propertyStates)
    }
    unsafe fn sendSampleBuffer_discontinuity_hostTimeInNanoseconds_(
        &self,
        sampleBuffer: CMSampleBufferRef,
        discontinuity: CMIOExtensionStreamDiscontinuityFlags,
        hostTimeInNanoseconds: u64,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, sendSampleBuffer : sampleBuffer, discontinuity : discontinuity, hostTimeInNanoseconds : hostTimeInNanoseconds)
    }
    unsafe fn consumeSampleBufferFromClient_completionHandler_(
        &self,
        client: CMIOExtensionClient,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, consumeSampleBufferFromClient : client, completionHandler : completionHandler)
    }
    unsafe fn notifyScheduledOutputChanged_(&self, scheduledOutput: CMIOExtensionScheduledOutput)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, notifyScheduledOutputChanged : scheduledOutput)
    }
    unsafe fn localizedName(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, localizedName)
    }
    unsafe fn streamID(&self) -> NSUUID
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, streamID)
    }
    unsafe fn direction(&self) -> CMIOExtensionStreamDirection
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, direction)
    }
    unsafe fn clockType(&self) -> CMIOExtensionStreamClockType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, clockType)
    }
    unsafe fn customClockConfiguration(&self) -> CMIOExtensionStreamCustomClockConfiguration
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, customClockConfiguration)
    }
    unsafe fn source(&self) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, source)
    }
    unsafe fn streamingClients(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, streamingClients)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CMIOExtensionStream").unwrap(), new)
    }
    unsafe fn streamWithLocalizedName_streamID_direction_clockType_source_(
        localizedName: NSString,
        streamID: NSUUID,
        direction: CMIOExtensionStreamDirection,
        clockType: CMIOExtensionStreamClockType,
        source: *mut u64,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CMIOExtensionStream").unwrap(), streamWithLocalizedName : localizedName, streamID : streamID, direction : direction, clockType : clockType, source : source)
    }
    unsafe fn streamWithLocalizedName_streamID_direction_customClockConfiguration_source_(
        localizedName: NSString,
        streamID: NSUUID,
        direction: CMIOExtensionStreamDirection,
        customClockConfiguration: CMIOExtensionStreamCustomClockConfiguration,
        source: *mut u64,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CMIOExtensionStream").unwrap(), streamWithLocalizedName : localizedName, streamID : streamID, direction : direction, customClockConfiguration : customClockConfiguration, source : source)
    }
}
pub type CMIOObjectPropertySelector = UInt32;
pub type CMIOObjectPropertyScope = UInt32;
pub type CMIOObjectPropertyElement = UInt32;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CMIOObjectPropertyAddress {
    pub mSelector: CMIOObjectPropertySelector,
    pub mScope: CMIOObjectPropertyScope,
    pub mElement: CMIOObjectPropertyElement,
}
pub type CMIOClassID = UInt32;
pub type CMIOObjectID = UInt32;
pub type CMIOObjectPropertyListenerProc = ::std::option::Option<
    unsafe extern "C" fn(
        objectID: CMIOObjectID,
        numberAddresses: UInt32,
        addresses: *const CMIOObjectPropertyAddress,
        clientData: *mut ::std::os::raw::c_void,
    ) -> OSStatus,
>;
pub type CMIOObjectPropertyListenerBlock = *mut ::std::os::raw::c_void;
pub type CMIOHardwarePropertyID = CMIOObjectPropertySelector;
pub type CMIOStreamID = CMIOObjectID;
pub type CMIODeviceStreamQueueAlteredProc = ::std::option::Option<
    unsafe extern "C" fn(
        streamID: CMIOStreamID,
        token: *mut ::std::os::raw::c_void,
        refCon: *mut ::std::os::raw::c_void,
    ),
>;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CMIOStreamDeck {
    pub mStatus: UInt32,
    pub mState: UInt32,
    pub mState2: UInt32,
}
pub type CMIOStreamScheduledOutputNotificationProc = ::std::option::Option<
    unsafe extern "C" fn(
        sequenceNumberOfBufferThatWasOutput: UInt64,
        outputHostTime: UInt64,
        scheduledOutputNotificationRefCon: *mut ::std::os::raw::c_void,
    ),
>;
#[repr(C, packed(4))]
#[derive(Debug, Copy, Clone)]
pub struct CMIOStreamScheduledOutputNotificationProcAndRefCon {
    pub scheduledOutputNotificationProc: CMIOStreamScheduledOutputNotificationProc,
    pub scheduledOutputNotificationRefCon: *mut ::std::os::raw::c_void,
}
pub type CMIODeviceID = CMIOObjectID;
pub type CMIODevicePropertyID = CMIOObjectPropertySelector;
#[repr(C)]
#[derive(Debug)]
pub struct CMIODeviceStreamConfiguration {
    pub mNumberStreams: UInt32,
    pub mNumberChannels: __IncompleteArrayField<UInt32>,
}
#[repr(C, packed(4))]
#[derive(Debug, Copy, Clone)]
pub struct CMIODeviceAVCCommand {
    pub mCommand: *mut UInt8,
    pub mCommandLength: UInt32,
    pub mResponse: *mut UInt8,
    pub mResponseLength: UInt32,
    pub mResponseUsed: UInt32,
}
#[repr(C, packed(4))]
#[derive(Debug, Copy, Clone)]
pub struct CMIODeviceRS422Command {
    pub mCommand: *mut UInt8,
    pub mCommandLength: UInt32,
    pub mResponse: *mut UInt8,
    pub mResponseLength: UInt32,
    pub mResponseUsed: UInt32,
}
pub type CMIODeviceGetSMPTETimeProc = ::std::option::Option<
    unsafe extern "C" fn(
        refCon: *mut ::std::os::raw::c_void,
        frameNumber: *mut UInt64,
        isDropFrame: *mut Boolean,
        tolerance: *mut UInt32,
    ) -> OSStatus,
>;
#[repr(C, packed(4))]
#[derive(Debug, Copy, Clone)]
pub struct CMIODeviceSMPTETimeCallback {
    pub mGetSMPTETimeProc: CMIODeviceGetSMPTETimeProc,
    pub mRefCon: *mut ::std::os::raw::c_void,
}
pub type CMIOControlID = CMIOObjectID;
pub type CMIOHardwarePlugInRef = *mut *mut CMIOHardwarePlugInInterface;
#[repr(C, packed(4))]
#[derive(Debug, Copy, Clone)]
pub struct CMIOHardwarePlugInInterface {
    pub _reserved: *mut ::std::os::raw::c_void,
    pub QueryInterface: ::std::option::Option<
        unsafe extern "C" fn(
            self_: *mut ::std::os::raw::c_void,
            uuid: REFIID,
            interface: *mut LPVOID,
        ) -> HRESULT,
    >,
    pub AddRef:
        ::std::option::Option<unsafe extern "C" fn(self_: *mut ::std::os::raw::c_void) -> ULONG>,
    pub Release:
        ::std::option::Option<unsafe extern "C" fn(self_: *mut ::std::os::raw::c_void) -> ULONG>,
    pub Initialize:
        ::std::option::Option<unsafe extern "C" fn(self_: CMIOHardwarePlugInRef) -> OSStatus>,
    pub InitializeWithObjectID: ::std::option::Option<
        unsafe extern "C" fn(self_: CMIOHardwarePlugInRef, objectID: CMIOObjectID) -> OSStatus,
    >,
    pub Teardown:
        ::std::option::Option<unsafe extern "C" fn(self_: CMIOHardwarePlugInRef) -> OSStatus>,
    pub ObjectShow: ::std::option::Option<
        unsafe extern "C" fn(self_: CMIOHardwarePlugInRef, objectID: CMIOObjectID),
    >,
    pub ObjectHasProperty: ::std::option::Option<
        unsafe extern "C" fn(
            self_: CMIOHardwarePlugInRef,
            objectID: CMIOObjectID,
            address: *const CMIOObjectPropertyAddress,
        ) -> Boolean,
    >,
    pub ObjectIsPropertySettable: ::std::option::Option<
        unsafe extern "C" fn(
            self_: CMIOHardwarePlugInRef,
            objectID: CMIOObjectID,
            address: *const CMIOObjectPropertyAddress,
            isSettable: *mut Boolean,
        ) -> OSStatus,
    >,
    pub ObjectGetPropertyDataSize: ::std::option::Option<
        unsafe extern "C" fn(
            self_: CMIOHardwarePlugInRef,
            objectID: CMIOObjectID,
            address: *const CMIOObjectPropertyAddress,
            qualifierDataSize: UInt32,
            qualifierData: *const ::std::os::raw::c_void,
            dataSize: *mut UInt32,
        ) -> OSStatus,
    >,
    pub ObjectGetPropertyData: ::std::option::Option<
        unsafe extern "C" fn(
            self_: CMIOHardwarePlugInRef,
            objectID: CMIOObjectID,
            address: *const CMIOObjectPropertyAddress,
            qualifierDataSize: UInt32,
            qualifierData: *const ::std::os::raw::c_void,
            dataSize: UInt32,
            dataUsed: *mut UInt32,
            data: *mut ::std::os::raw::c_void,
        ) -> OSStatus,
    >,
    pub ObjectSetPropertyData: ::std::option::Option<
        unsafe extern "C" fn(
            self_: CMIOHardwarePlugInRef,
            objectID: CMIOObjectID,
            address: *const CMIOObjectPropertyAddress,
            qualifierDataSize: UInt32,
            qualifierData: *const ::std::os::raw::c_void,
            dataSize: UInt32,
            data: *const ::std::os::raw::c_void,
        ) -> OSStatus,
    >,
    pub DeviceSuspend: ::std::option::Option<
        unsafe extern "C" fn(self_: CMIOHardwarePlugInRef, device: CMIODeviceID) -> OSStatus,
    >,
    pub DeviceResume: ::std::option::Option<
        unsafe extern "C" fn(self_: CMIOHardwarePlugInRef, device: CMIODeviceID) -> OSStatus,
    >,
    pub DeviceStartStream: ::std::option::Option<
        unsafe extern "C" fn(
            self_: CMIOHardwarePlugInRef,
            device: CMIODeviceID,
            stream: CMIOStreamID,
        ) -> OSStatus,
    >,
    pub DeviceStopStream: ::std::option::Option<
        unsafe extern "C" fn(
            self_: CMIOHardwarePlugInRef,
            device: CMIODeviceID,
            stream: CMIOStreamID,
        ) -> OSStatus,
    >,
    pub DeviceProcessAVCCommand: ::std::option::Option<
        unsafe extern "C" fn(
            self_: CMIOHardwarePlugInRef,
            device: CMIODeviceID,
            ioAVCCommand: *mut CMIODeviceAVCCommand,
        ) -> OSStatus,
    >,
    pub DeviceProcessRS422Command: ::std::option::Option<
        unsafe extern "C" fn(
            self_: CMIOHardwarePlugInRef,
            device: CMIODeviceID,
            ioRS422Command: *mut CMIODeviceRS422Command,
        ) -> OSStatus,
    >,
    pub StreamCopyBufferQueue: ::std::option::Option<
        unsafe extern "C" fn(
            self_: CMIOHardwarePlugInRef,
            stream: CMIOStreamID,
            queueAlteredProc: CMIODeviceStreamQueueAlteredProc,
            queueAlteredRefCon: *mut ::std::os::raw::c_void,
            queue: *mut CMSimpleQueueRef,
        ) -> OSStatus,
    >,
    pub StreamDeckPlay: ::std::option::Option<
        unsafe extern "C" fn(self_: CMIOHardwarePlugInRef, stream: CMIOStreamID) -> OSStatus,
    >,
    pub StreamDeckStop: ::std::option::Option<
        unsafe extern "C" fn(self_: CMIOHardwarePlugInRef, stream: CMIOStreamID) -> OSStatus,
    >,
    pub StreamDeckJog: ::std::option::Option<
        unsafe extern "C" fn(
            self_: CMIOHardwarePlugInRef,
            stream: CMIOStreamID,
            speed: SInt32,
        ) -> OSStatus,
    >,
    pub StreamDeckCueTo: ::std::option::Option<
        unsafe extern "C" fn(
            self_: CMIOHardwarePlugInRef,
            stream: CMIOStreamID,
            frameNumber: Float64,
            playOnCue: Boolean,
        ) -> OSStatus,
    >,
}
unsafe extern "C" {
    pub static CMIOExtensionPropertyProviderName: CMIOExtensionProperty;
}
unsafe extern "C" {
    pub static CMIOExtensionPropertyProviderManufacturer: CMIOExtensionProperty;
}
unsafe extern "C" {
    pub static CMIOExtensionPropertyDeviceModel: CMIOExtensionProperty;
}
unsafe extern "C" {
    pub static CMIOExtensionPropertyDeviceIsSuspended: CMIOExtensionProperty;
}
unsafe extern "C" {
    pub static CMIOExtensionPropertyDeviceTransportType: CMIOExtensionProperty;
}
unsafe extern "C" {
    pub static CMIOExtensionPropertyDeviceLinkedCoreAudioDeviceUID: CMIOExtensionProperty;
}
unsafe extern "C" {
    pub static CMIOExtensionPropertyDeviceCanBeDefaultInputDevice: CMIOExtensionProperty;
}
unsafe extern "C" {
    pub static CMIOExtensionPropertyDeviceCanBeDefaultOutputDevice: CMIOExtensionProperty;
}
unsafe extern "C" {
    pub static CMIOExtensionPropertyDeviceLatency: CMIOExtensionProperty;
}
unsafe extern "C" {
    pub static CMIOExtensionPropertyStreamActiveFormatIndex: CMIOExtensionProperty;
}
unsafe extern "C" {
    pub static CMIOExtensionPropertyStreamFrameDuration: CMIOExtensionProperty;
}
unsafe extern "C" {
    pub static CMIOExtensionPropertyStreamMaxFrameDuration: CMIOExtensionProperty;
}
unsafe extern "C" {
    pub static CMIOExtensionPropertyStreamSinkBufferQueueSize: CMIOExtensionProperty;
}
unsafe extern "C" {
    pub static CMIOExtensionPropertyStreamSinkBuffersRequiredForStartup: CMIOExtensionProperty;
}
unsafe extern "C" {
    pub static CMIOExtensionPropertyStreamSinkBufferUnderrunCount: CMIOExtensionProperty;
}
unsafe extern "C" {
    pub static CMIOExtensionPropertyStreamSinkEndOfData: CMIOExtensionProperty;
}
unsafe extern "C" {
    pub static CMIOExtensionPropertyStreamLatency: CMIOExtensionProperty;
}
unsafe extern "C" {
    pub static CMIOExtensionInfoDictionaryKey: NSString;
}
unsafe extern "C" {
    pub static CMIOExtensionMachServiceNameKey: NSString;
}
unsafe extern "C" {
    pub fn CMIOObjectShow(objectID: CMIOObjectID);
}
unsafe extern "C" {
    pub fn CMIOObjectHasProperty(
        objectID: CMIOObjectID,
        address: *const CMIOObjectPropertyAddress,
    ) -> Boolean;
}
unsafe extern "C" {
    pub fn CMIOObjectIsPropertySettable(
        objectID: CMIOObjectID,
        address: *const CMIOObjectPropertyAddress,
        isSettable: *mut Boolean,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn CMIOObjectGetPropertyDataSize(
        objectID: CMIOObjectID,
        address: *const CMIOObjectPropertyAddress,
        qualifierDataSize: UInt32,
        qualifierData: *const ::std::os::raw::c_void,
        dataSize: *mut UInt32,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn CMIOObjectGetPropertyData(
        objectID: CMIOObjectID,
        address: *const CMIOObjectPropertyAddress,
        qualifierDataSize: UInt32,
        qualifierData: *const ::std::os::raw::c_void,
        dataSize: UInt32,
        dataUsed: *mut UInt32,
        data: *mut ::std::os::raw::c_void,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn CMIOObjectSetPropertyData(
        objectID: CMIOObjectID,
        address: *const CMIOObjectPropertyAddress,
        qualifierDataSize: UInt32,
        qualifierData: *const ::std::os::raw::c_void,
        dataSize: UInt32,
        data: *const ::std::os::raw::c_void,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn CMIOObjectAddPropertyListener(
        objectID: CMIOObjectID,
        address: *const CMIOObjectPropertyAddress,
        listener: CMIOObjectPropertyListenerProc,
        clientData: *mut ::std::os::raw::c_void,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn CMIOObjectRemovePropertyListener(
        objectID: CMIOObjectID,
        address: *const CMIOObjectPropertyAddress,
        listener: CMIOObjectPropertyListenerProc,
        clientData: *mut ::std::os::raw::c_void,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn CMIOObjectAddPropertyListenerBlock(
        objectID: CMIOObjectID,
        address: *const CMIOObjectPropertyAddress,
        dispatchQueue: NSObject,
        listener: CMIOObjectPropertyListenerBlock,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn CMIOObjectRemovePropertyListenerBlock(
        objectID: CMIOObjectID,
        address: *const CMIOObjectPropertyAddress,
        dispatchQueue: NSObject,
        listener: CMIOObjectPropertyListenerBlock,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn CMIOStreamCopyBufferQueue(
        streamID: CMIOStreamID,
        queueAlteredProc: CMIODeviceStreamQueueAlteredProc,
        queueAlteredRefCon: *mut ::std::os::raw::c_void,
        queue: *mut CMSimpleQueueRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn CMIOStreamDeckPlay(streamID: CMIOStreamID) -> OSStatus;
}
unsafe extern "C" {
    pub fn CMIOStreamDeckStop(streamID: CMIOStreamID) -> OSStatus;
}
unsafe extern "C" {
    pub fn CMIOStreamDeckJog(streamID: CMIOStreamID, speed: SInt32) -> OSStatus;
}
unsafe extern "C" {
    pub fn CMIOStreamDeckCueTo(
        streamID: CMIOStreamID,
        frameNumber: UInt64,
        playOnCue: Boolean,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn CMIOStreamClockCreate(
        allocator: CFAllocatorRef,
        clockName: CFStringRef,
        sourceIdentifier: *const ::std::os::raw::c_void,
        getTimeCallMinimumInterval: CMTime,
        numberOfEventsForRateSmoothing: UInt32,
        numberOfAveragesForRateSmoothing: UInt32,
        clock: *mut CFTypeRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn CMIOStreamClockPostTimingEvent(
        eventTime: CMTime,
        hostTime: UInt64,
        resynchronize: Boolean,
        clock: CFTypeRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn CMIOStreamClockInvalidate(clock: CFTypeRef) -> OSStatus;
}
unsafe extern "C" {
    pub fn CMIOStreamClockConvertHostTimeToDeviceTime(hostTime: UInt64, clock: CFTypeRef)
        -> CMTime;
}
unsafe extern "C" {
    pub fn CMIODeviceStartStream(deviceID: CMIODeviceID, streamID: CMIOStreamID) -> OSStatus;
}
unsafe extern "C" {
    pub fn CMIODeviceStopStream(deviceID: CMIODeviceID, streamID: CMIOStreamID) -> OSStatus;
}
unsafe extern "C" {
    pub fn CMIODeviceProcessAVCCommand(
        deviceID: CMIODeviceID,
        ioAVCCommand: *mut CMIODeviceAVCCommand,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn CMIODeviceProcessRS422Command(
        deviceID: CMIODeviceID,
        ioRS422Command: *mut CMIODeviceRS422Command,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn CMIOObjectCreate(
        owningPlugIn: CMIOHardwarePlugInRef,
        owningObjectID: CMIOObjectID,
        classID: CMIOClassID,
        objectID: *mut CMIOObjectID,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn CMIOObjectsPublishedAndDied(
        owningPlugIn: CMIOHardwarePlugInRef,
        owningObjectID: CMIOObjectID,
        numberPublishedCMIOObjects: UInt32,
        publishedCMIOObjects: *const CMIOObjectID,
        numberDeadCMIOObjects: UInt32,
        deadCMIOObjects: *const CMIOObjectID,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn CMIOObjectPropertiesChanged(
        owningPlugIn: CMIOHardwarePlugInRef,
        objectID: CMIOObjectID,
        numberAddresses: UInt32,
        addresses: *const CMIOObjectPropertyAddress,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub static mut kCMIOSampleBufferAttachmentKey_DiscontinuityFlags: CFStringRef;
}
unsafe extern "C" {
    pub static mut kCMIOSampleBufferAttachmentKey_SequenceNumber: CFStringRef;
}
unsafe extern "C" {
    pub static mut kCMIOSampleBufferAttachmentKey_HDV1_PackData: CFStringRef;
}
unsafe extern "C" {
    pub static mut kCMIOSampleBufferAttachmentKey_HDV2_VAUX: CFStringRef;
}
unsafe extern "C" {
    pub static mut kCMIOSampleBufferAttachmentKey_CAAudioTimeStamp: CFStringRef;
}
unsafe extern "C" {
    pub static mut kCMIOSampleBufferAttachmentKey_SMPTETime: CFStringRef;
}
unsafe extern "C" {
    pub static mut kCMIOSampleBufferAttachmentKey_NativeSMPTEFrameCount: CFStringRef;
}
unsafe extern "C" {
    pub static mut kCMIOSampleBufferAttachmentKey_NumberOfVideoFramesInBuffer: CFStringRef;
}
unsafe extern "C" {
    pub static mut kCMIOSampleBufferAttachmentKey_NumberOfVideoFramesInGOP: CFStringRef;
}
unsafe extern "C" {
    pub static mut kCMIOSampleBufferAttachmentKey_MuxedSourcePresentationTimeStamp: CFStringRef;
}
unsafe extern "C" {
    pub static mut kCMIOSampleBufferAttachmentKey_HostTime: CFStringRef;
}
unsafe extern "C" {
    pub static mut kCMIOSampleBufferAttachmentKey_RepeatedBufferContents: CFStringRef;
}
unsafe extern "C" {
    pub static mut kCMIOSampleBufferAttachmentKey_SourceAudioFormatDescription: CFStringRef;
}
unsafe extern "C" {
    pub static mut kCMIOSampleBufferAttachmentKey_PulldownCadenceInfo: CFStringRef;
}
unsafe extern "C" {
    pub static mut kCMIOSampleBufferAttachmentKey_ClosedCaptionSampleBuffer: CFStringRef;
}
unsafe extern "C" {
    pub static mut kCMIOSampleBufferAttachmentKey_ClientSequenceID: CFStringRef;
}
unsafe extern "C" {
    pub static mut kCMIOSampleBufferAttachmentKey_MouseAndKeyboardModifiers: CFStringRef;
}
unsafe extern "C" {
    pub static mut kCMIOSampleBufferAttachment_MouseAndKeyboardModifiersKey_CursorPositionX:
        CFStringRef;
}
unsafe extern "C" {
    pub static mut kCMIOSampleBufferAttachment_MouseAndKeyboardModifiersKey_CursorPositionY:
        CFStringRef;
}
unsafe extern "C" {
    pub static mut kCMIOSampleBufferAttachment_MouseAndKeyboardModifiersKey_MouseButtonState:
        CFStringRef;
}
unsafe extern "C" {
    pub static mut kCMIOSampleBufferAttachment_MouseAndKeyboardModifiersKey_CursorIsVisible:
        CFStringRef;
}
unsafe extern "C" {
    pub static mut kCMIOSampleBufferAttachment_MouseAndKeyboardModifiersKey_CursorFrameRect:
        CFStringRef;
}
unsafe extern "C" {
    pub static mut kCMIOSampleBufferAttachment_MouseAndKeyboardModifiersKey_CursorReference:
        CFStringRef;
}
unsafe extern "C" {
    pub static mut kCMIOSampleBufferAttachment_MouseAndKeyboardModifiersKey_CursorSeed: CFStringRef;
}
unsafe extern "C" {
    pub static mut kCMIOSampleBufferAttachment_MouseAndKeyboardModifiersKey_CursorScale:
        CFStringRef;
}
unsafe extern "C" {
    pub static mut kCMIOSampleBufferAttachment_MouseAndKeyboardModifiersKey_CursorIsDrawnInFramebuffer:
        CFStringRef;
}
unsafe extern "C" {
    pub static mut kCMIOSampleBufferAttachment_MouseAndKeyboardModifiersKey_KeyboardModifiers:
        CFStringRef;
}
unsafe extern "C" {
    pub static mut kCMIOSampleBufferAttachment_MouseAndKeyboardModifiersKey_KeyboardModifiersEvent:
        CFStringRef;
}
unsafe extern "C" {
    pub static mut kCMIOSampleBufferAttachmentKey_PixelBufferOverlaidByStaticImage: CFStringRef;
}
unsafe extern "C" {
    pub static mut kCMIOSampleBufferAttachmentKey_NoDataMarker: CFStringRef;
}
unsafe extern "C" {
    pub static mut kCMIOBlockBufferAttachmentKey_CVPixelBufferReference: CFStringRef;
}
unsafe extern "C" {
    pub fn CMIOSampleBufferCreate(
        allocator: CFAllocatorRef,
        dataBuffer: CMBlockBufferRef,
        formatDescription: CMFormatDescriptionRef,
        numSamples: UInt32,
        numSampleTimingEntries: UInt32,
        sampleTimingArray: *const CMSampleTimingInfo,
        numSampleSizeEntries: UInt32,
        sampleSizeArray: *const usize,
        sequenceNumber: UInt64,
        discontinuityFlags: UInt32,
        sBufOut: *mut CMSampleBufferRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn CMIOSampleBufferCreateForImageBuffer(
        allocator: CFAllocatorRef,
        imageBuffer: CVImageBufferRef,
        formatDescription: CMVideoFormatDescriptionRef,
        sampleTiming: *const CMSampleTimingInfo,
        sequenceNumber: UInt64,
        discontinuityFlags: UInt32,
        sBufOut: *mut CMSampleBufferRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn CMIOSampleBufferCreateNoDataMarker(
        allocator: CFAllocatorRef,
        noDataEvent: UInt32,
        formatDescription: CMFormatDescriptionRef,
        sequenceNumber: UInt64,
        discontinuityFlags: UInt32,
        sBufOut: *mut CMSampleBufferRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn CMIOSampleBufferSetSequenceNumber(
        allocator: CFAllocatorRef,
        sbuf: CMSampleBufferRef,
        sequenceNumber: UInt64,
    );
}
unsafe extern "C" {
    pub fn CMIOSampleBufferGetSequenceNumber(sbuf: CMSampleBufferRef) -> UInt64;
}
unsafe extern "C" {
    pub fn CMIOSampleBufferSetDiscontinuityFlags(
        allocator: CFAllocatorRef,
        sbuf: CMSampleBufferRef,
        discontinuityFlags: UInt32,
    );
}
unsafe extern "C" {
    pub fn CMIOSampleBufferGetDiscontinuityFlags(sbuf: CMSampleBufferRef) -> UInt32;
}
unsafe extern "C" {
    pub fn CMIOSampleBufferCopyNonRequiredAttachments(
        sourceSBuf: CMSampleBufferRef,
        destSBuf: CMSampleBufferRef,
        attachmentMode: CMAttachmentMode,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn CMIOSampleBufferCopySampleAttachments(
        sourceSBuf: CMSampleBufferRef,
        destSBuf: CMSampleBufferRef,
    ) -> OSStatus;
}

unsafe impl objc2::encode::RefEncode for CMIOExtensionPropertyAttributes {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CMIOExtensionPropertyAttributes {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CMIOExtensionPropertyState {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CMIOExtensionPropertyState {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CMIOExtensionStreamCustomClockConfiguration {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CMIOExtensionStreamCustomClockConfiguration {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CMIOExtensionStreamFormat {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CMIOExtensionStreamFormat {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CMIOExtensionScheduledOutput {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CMIOExtensionScheduledOutput {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CMIOExtensionClient {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CMIOExtensionClient {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CMIOExtensionProviderProperties {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CMIOExtensionProviderProperties {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CMIOExtensionProvider {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CMIOExtensionProvider {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CMIOExtensionDeviceProperties {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CMIOExtensionDeviceProperties {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CMIOExtensionDevice {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CMIOExtensionDevice {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CMIOExtensionStreamProperties {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CMIOExtensionStreamProperties {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CMIOExtensionStream {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CMIOExtensionStream {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CMIOObjectPropertyAddress {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CMIOObjectPropertyAddress {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("CMIOObjectPropertyAddress", &[]);
}
unsafe impl objc2::encode::RefEncode for CMIOStreamDeck {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CMIOStreamDeck {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("CMIOStreamDeck", &[]);
}
unsafe impl objc2::encode::RefEncode for CMIOStreamScheduledOutputNotificationProcAndRefCon {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CMIOStreamScheduledOutputNotificationProcAndRefCon {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("CMIOStreamScheduledOutputNotificationProcAndRefCon", &[]);
}
unsafe impl objc2::encode::RefEncode for CMIODeviceStreamConfiguration {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CMIODeviceStreamConfiguration {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("CMIODeviceStreamConfiguration", &[]);
}
unsafe impl objc2::encode::RefEncode for CMIODeviceAVCCommand {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CMIODeviceAVCCommand {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("CMIODeviceAVCCommand", &[]);
}
unsafe impl objc2::encode::RefEncode for CMIODeviceRS422Command {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CMIODeviceRS422Command {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("CMIODeviceRS422Command", &[]);
}
unsafe impl objc2::encode::RefEncode for CMIODeviceSMPTETimeCallback {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CMIODeviceSMPTETimeCallback {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("CMIODeviceSMPTETimeCallback", &[]);
}
unsafe impl objc2::encode::RefEncode for CMIOHardwarePlugInInterface {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CMIOHardwarePlugInInterface {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("CMIOHardwarePlugInInterface", &[]);
}
