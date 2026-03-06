#[allow(unused_imports)]
use crate::objc::*;
#[allow(unused_imports)]
use crate::CoreFoundation::*;
#[allow(unused_imports)]
use crate::Foundation::*;
#[allow(unused_imports)]
use libc::{id_t, pid_t};

#[allow(unused_imports)]
use objc2::msg_send;
pub type os_activity_id_t = u64;
pub type os_signpost_id_t = u64;
pub type OSLogEntryStoreCategory = NSInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct OSLogEntry(pub id);
impl std::ops::Deref for OSLogEntry {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for OSLogEntry {}
impl OSLogEntry {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"OSLogEntry").unwrap(), alloc) })
    }
}
impl INSObject for OSLogEntry {}
impl PNSObject for OSLogEntry {}
impl std::convert::TryFrom<NSObject> for OSLogEntry {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<OSLogEntry, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"OSLogEntry").unwrap()) };
        if is_kind_of {
            Ok(OSLogEntry(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to OSLogEntry")
        }
    }
}
impl IOSLogEntry for OSLogEntry {}
pub trait IOSLogEntry: Sized + std::ops::Deref {
    unsafe fn composedMessage(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, composedMessage)
    }
    unsafe fn date(&self) -> NSDate
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, date)
    }
    unsafe fn storeCategory(&self) -> OSLogEntryStoreCategory
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, storeCategory)
    }
}
pub trait POSLogEntryFromProcess: Sized + std::ops::Deref {
    unsafe fn activityIdentifier(&self) -> os_activity_id_t
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, activityIdentifier)
    }
    unsafe fn process(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, process)
    }
    unsafe fn processIdentifier(&self) -> pid_t
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, processIdentifier)
    }
    unsafe fn sender(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, sender)
    }
    unsafe fn threadIdentifier(&self) -> u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, threadIdentifier)
    }
}
pub trait POSLogEntryWithPayload: Sized + std::ops::Deref {
    unsafe fn category(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, category)
    }
    unsafe fn components(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, components)
    }
    unsafe fn formatString(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, formatString)
    }
    unsafe fn subsystem(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, subsystem)
    }
}
impl OSLogEntry_ for OSLogEntry {}
pub trait OSLogEntry_: Sized + std::ops::Deref {}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct OSLogEntryActivity(pub id);
impl std::ops::Deref for OSLogEntryActivity {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for OSLogEntryActivity {}
impl OSLogEntryActivity {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"OSLogEntryActivity").unwrap(), alloc) })
    }
}
impl POSLogEntryFromProcess for OSLogEntryActivity {}
impl IOSLogEntry for OSLogEntryActivity {}
impl From<OSLogEntryActivity> for OSLogEntry {
    fn from(child: OSLogEntryActivity) -> OSLogEntry {
        OSLogEntry(child.0)
    }
}
impl std::convert::TryFrom<OSLogEntry> for OSLogEntryActivity {
    type Error = &'static str;
    fn try_from(parent: OSLogEntry) -> Result<OSLogEntryActivity, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"OSLogEntryActivity").unwrap()) };
        if is_kind_of {
            Ok(OSLogEntryActivity(parent.0))
        } else {
            Err("This OSLogEntry cannot be downcasted to OSLogEntryActivity")
        }
    }
}
impl INSObject for OSLogEntryActivity {}
impl PNSObject for OSLogEntryActivity {}
impl IOSLogEntryActivity for OSLogEntryActivity {}
pub trait IOSLogEntryActivity: Sized + std::ops::Deref {
    unsafe fn parentActivityIdentifier(&self) -> os_activity_id_t
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, parentActivityIdentifier)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct OSLogEntryBoundary(pub id);
impl std::ops::Deref for OSLogEntryBoundary {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for OSLogEntryBoundary {}
impl OSLogEntryBoundary {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"OSLogEntryBoundary").unwrap(), alloc) })
    }
}
impl IOSLogEntry for OSLogEntryBoundary {}
impl std::convert::TryFrom<OSLogEntry> for OSLogEntryBoundary {
    type Error = &'static str;
    fn try_from(parent: OSLogEntry) -> Result<OSLogEntryBoundary, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"OSLogEntryBoundary").unwrap()) };
        if is_kind_of {
            Ok(OSLogEntryBoundary(parent.0))
        } else {
            Err("This OSLogEntry cannot be downcasted to OSLogEntryBoundary")
        }
    }
}
impl INSObject for OSLogEntryBoundary {}
impl PNSObject for OSLogEntryBoundary {}
impl IOSLogEntryBoundary for OSLogEntryBoundary {}
pub trait IOSLogEntryBoundary: Sized + std::ops::Deref {}
pub type OSLogEntryLogLevel = NSInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct OSLogEntryLog(pub id);
impl std::ops::Deref for OSLogEntryLog {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for OSLogEntryLog {}
impl OSLogEntryLog {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"OSLogEntryLog").unwrap(), alloc) })
    }
}
impl POSLogEntryFromProcess for OSLogEntryLog {}
impl POSLogEntryWithPayload for OSLogEntryLog {}
impl IOSLogEntry for OSLogEntryLog {}
impl std::convert::TryFrom<OSLogEntry> for OSLogEntryLog {
    type Error = &'static str;
    fn try_from(parent: OSLogEntry) -> Result<OSLogEntryLog, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"OSLogEntryLog").unwrap()) };
        if is_kind_of {
            Ok(OSLogEntryLog(parent.0))
        } else {
            Err("This OSLogEntry cannot be downcasted to OSLogEntryLog")
        }
    }
}
impl INSObject for OSLogEntryLog {}
impl PNSObject for OSLogEntryLog {}
impl IOSLogEntryLog for OSLogEntryLog {}
pub trait IOSLogEntryLog: Sized + std::ops::Deref {
    unsafe fn level(&self) -> OSLogEntryLogLevel
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, level)
    }
}
pub type OSLogEntrySignpostType = NSInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct OSLogEntrySignpost(pub id);
impl std::ops::Deref for OSLogEntrySignpost {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for OSLogEntrySignpost {}
impl OSLogEntrySignpost {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"OSLogEntrySignpost").unwrap(), alloc) })
    }
}
impl POSLogEntryFromProcess for OSLogEntrySignpost {}
impl POSLogEntryWithPayload for OSLogEntrySignpost {}
impl IOSLogEntry for OSLogEntrySignpost {}
impl std::convert::TryFrom<OSLogEntry> for OSLogEntrySignpost {
    type Error = &'static str;
    fn try_from(parent: OSLogEntry) -> Result<OSLogEntrySignpost, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"OSLogEntrySignpost").unwrap()) };
        if is_kind_of {
            Ok(OSLogEntrySignpost(parent.0))
        } else {
            Err("This OSLogEntry cannot be downcasted to OSLogEntrySignpost")
        }
    }
}
impl INSObject for OSLogEntrySignpost {}
impl PNSObject for OSLogEntrySignpost {}
impl IOSLogEntrySignpost for OSLogEntrySignpost {}
pub trait IOSLogEntrySignpost: Sized + std::ops::Deref {
    unsafe fn signpostIdentifier(&self) -> os_signpost_id_t
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, signpostIdentifier)
    }
    unsafe fn signpostName(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, signpostName)
    }
    unsafe fn signpostType(&self) -> OSLogEntrySignpostType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, signpostType)
    }
}
pub type OSLogEnumeratorOptions = NSUInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct OSLogEnumerator(pub id);
impl std::ops::Deref for OSLogEnumerator {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for OSLogEnumerator {}
impl OSLogEnumerator {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"OSLogEnumerator").unwrap(), alloc) })
    }
}
impl<ObjectType: 'static> INSEnumerator<ObjectType> for OSLogEnumerator {}
impl PNSFastEnumeration for OSLogEnumerator {}
impl INSObject for OSLogEnumerator {}
impl PNSObject for OSLogEnumerator {}
impl std::convert::TryFrom<NSObject> for OSLogEnumerator {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<OSLogEnumerator, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"OSLogEnumerator").unwrap()) };
        if is_kind_of {
            Ok(OSLogEnumerator(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to OSLogEnumerator")
        }
    }
}
impl IOSLogEnumerator for OSLogEnumerator {}
pub trait IOSLogEnumerator: Sized + std::ops::Deref {}
pub type OSLogMessageComponentArgumentCategory = NSInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct OSLogMessageComponent(pub id);
impl std::ops::Deref for OSLogMessageComponent {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for OSLogMessageComponent {}
impl OSLogMessageComponent {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"OSLogMessageComponent").unwrap(), alloc) })
    }
}
impl PNSSecureCoding for OSLogMessageComponent {}
impl INSObject for OSLogMessageComponent {}
impl PNSObject for OSLogMessageComponent {}
impl std::convert::TryFrom<NSObject> for OSLogMessageComponent {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<OSLogMessageComponent, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"OSLogMessageComponent").unwrap()) };
        if is_kind_of {
            Ok(OSLogMessageComponent(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to OSLogMessageComponent")
        }
    }
}
impl IOSLogMessageComponent for OSLogMessageComponent {}
pub trait IOSLogMessageComponent: Sized + std::ops::Deref {
    unsafe fn formatSubstring(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, formatSubstring)
    }
    unsafe fn placeholder(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, placeholder)
    }
    unsafe fn argumentCategory(&self) -> OSLogMessageComponentArgumentCategory
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, argumentCategory)
    }
    unsafe fn argumentDataValue(&self) -> NSData
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, argumentDataValue)
    }
    unsafe fn argumentDoubleValue(&self) -> f64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, argumentDoubleValue)
    }
    unsafe fn argumentInt64Value(&self) -> i64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, argumentInt64Value)
    }
    unsafe fn argumentNumberValue(&self) -> NSNumber
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, argumentNumberValue)
    }
    unsafe fn argumentStringValue(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, argumentStringValue)
    }
    unsafe fn argumentUInt64Value(&self) -> u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, argumentUInt64Value)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct OSLogPosition(pub id);
impl std::ops::Deref for OSLogPosition {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for OSLogPosition {}
impl OSLogPosition {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"OSLogPosition").unwrap(), alloc) })
    }
}
impl INSObject for OSLogPosition {}
impl PNSObject for OSLogPosition {}
impl std::convert::TryFrom<NSObject> for OSLogPosition {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<OSLogPosition, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"OSLogPosition").unwrap()) };
        if is_kind_of {
            Ok(OSLogPosition(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to OSLogPosition")
        }
    }
}
impl IOSLogPosition for OSLogPosition {}
pub trait IOSLogPosition: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
}
pub type OSLogStoreScope = NSInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct OSLogStore(pub id);
impl std::ops::Deref for OSLogStore {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for OSLogStore {}
impl OSLogStore {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"OSLogStore").unwrap(), alloc) })
    }
}
impl INSObject for OSLogStore {}
impl PNSObject for OSLogStore {}
impl std::convert::TryFrom<NSObject> for OSLogStore {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<OSLogStore, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"OSLogStore").unwrap()) };
        if is_kind_of {
            Ok(OSLogStore(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to OSLogStore")
        }
    }
}
impl IOSLogStore for OSLogStore {}
pub trait IOSLogStore: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn entriesEnumeratorWithOptions_position_predicate_error_(
        &self,
        options: OSLogEnumeratorOptions,
        position: OSLogPosition,
        predicate: NSPredicate,
        error: *mut NSError,
    ) -> OSLogEnumerator
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, entriesEnumeratorWithOptions : options, position : position, predicate : predicate, error : error)
    }
    unsafe fn entriesEnumeratorAndReturnError_(&self, error: *mut NSError) -> OSLogEnumerator
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, entriesEnumeratorAndReturnError : error)
    }
    unsafe fn positionWithDate_(&self, date: NSDate) -> OSLogPosition
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, positionWithDate : date)
    }
    unsafe fn positionWithTimeIntervalSinceEnd_(&self, seconds: NSTimeInterval) -> OSLogPosition
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, positionWithTimeIntervalSinceEnd : seconds)
    }
    unsafe fn positionWithTimeIntervalSinceLatestBoot_(
        &self,
        seconds: NSTimeInterval,
    ) -> OSLogPosition
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, positionWithTimeIntervalSinceLatestBoot : seconds)
    }
    unsafe fn localStoreAndReturnError_(error: *mut NSError) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"OSLogStore").unwrap(), localStoreAndReturnError : error)
    }
    unsafe fn storeWithScope_error_(scope: OSLogStoreScope, error: *mut NSError) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"OSLogStore").unwrap(), storeWithScope : scope, error : error)
    }
    unsafe fn storeWithURL_error_(url: NSURL, error: *mut NSError) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"OSLogStore").unwrap(), storeWithURL : url, error : error)
    }
}

unsafe impl objc2::encode::RefEncode for OSLogEntry {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for OSLogEntry {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for OSLogEntryActivity {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for OSLogEntryActivity {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for OSLogEntryBoundary {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for OSLogEntryBoundary {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for OSLogEntryLog {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for OSLogEntryLog {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for OSLogEntrySignpost {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for OSLogEntrySignpost {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for OSLogEnumerator {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for OSLogEnumerator {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for OSLogMessageComponent {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for OSLogMessageComponent {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for OSLogPosition {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for OSLogPosition {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for OSLogStore {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for OSLogStore {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
