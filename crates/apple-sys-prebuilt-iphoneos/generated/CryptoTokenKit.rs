#[allow(unused_imports)]
use crate::objc::*;
#[allow(unused_imports)]
use crate::CFNetwork::*;
#[allow(unused_imports)]
use crate::CoreServices::*;
#[allow(unused_imports)]
use crate::CoreText::*;
#[allow(unused_imports)]
use crate::Foundation::*;
#[allow(unused_imports)]
use crate::Security::*;

#[allow(unused_imports)]
use objc2::msg_send;
pub type TKErrorCode = NSInteger;
pub type TKTLVTag = UInt64;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct TKTLVRecord(pub id);
impl std::ops::Deref for TKTLVRecord {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for TKTLVRecord {}
impl TKTLVRecord {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"TKTLVRecord").unwrap(), alloc) })
    }
}
impl INSObject for TKTLVRecord {}
impl PNSObject for TKTLVRecord {}
impl std::convert::TryFrom<NSObject> for TKTLVRecord {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<TKTLVRecord, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"TKTLVRecord").unwrap()) };
        if is_kind_of {
            Ok(TKTLVRecord(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to TKTLVRecord")
        }
    }
}
impl ITKTLVRecord for TKTLVRecord {}
pub trait ITKTLVRecord: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn tag(&self) -> TKTLVTag
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, tag)
    }
    unsafe fn value(&self) -> NSData
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, value)
    }
    unsafe fn data(&self) -> NSData
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, data)
    }
    unsafe fn recordFromData_(data: NSData) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"TKTLVRecord").unwrap(), recordFromData : data)
    }
    unsafe fn sequenceOfRecordsFromData_(data: NSData) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"TKTLVRecord").unwrap(), sequenceOfRecordsFromData : data)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct TKBERTLVRecord(pub id);
impl std::ops::Deref for TKBERTLVRecord {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for TKBERTLVRecord {}
impl TKBERTLVRecord {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"TKBERTLVRecord").unwrap(), alloc) })
    }
}
impl ITKTLVRecord for TKBERTLVRecord {}
impl From<TKBERTLVRecord> for TKTLVRecord {
    fn from(child: TKBERTLVRecord) -> TKTLVRecord {
        TKTLVRecord(child.0)
    }
}
impl std::convert::TryFrom<TKTLVRecord> for TKBERTLVRecord {
    type Error = &'static str;
    fn try_from(parent: TKTLVRecord) -> Result<TKBERTLVRecord, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"TKBERTLVRecord").unwrap()) };
        if is_kind_of {
            Ok(TKBERTLVRecord(parent.0))
        } else {
            Err("This TKTLVRecord cannot be downcasted to TKBERTLVRecord")
        }
    }
}
impl INSObject for TKBERTLVRecord {}
impl PNSObject for TKBERTLVRecord {}
impl ITKBERTLVRecord for TKBERTLVRecord {}
pub trait ITKBERTLVRecord: Sized + std::ops::Deref {
    unsafe fn initWithTag_value_(&self, tag: TKTLVTag, value: NSData) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithTag : tag, value : value)
    }
    unsafe fn initWithTag_records_(&self, tag: TKTLVTag, records: NSArray) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithTag : tag, records : records)
    }
    unsafe fn dataForTag_(tag: TKTLVTag) -> NSData
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"TKBERTLVRecord").unwrap(), dataForTag : tag)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct TKSimpleTLVRecord(pub id);
impl std::ops::Deref for TKSimpleTLVRecord {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for TKSimpleTLVRecord {}
impl TKSimpleTLVRecord {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"TKSimpleTLVRecord").unwrap(), alloc) })
    }
}
impl ITKTLVRecord for TKSimpleTLVRecord {}
impl std::convert::TryFrom<TKTLVRecord> for TKSimpleTLVRecord {
    type Error = &'static str;
    fn try_from(parent: TKTLVRecord) -> Result<TKSimpleTLVRecord, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"TKSimpleTLVRecord").unwrap()) };
        if is_kind_of {
            Ok(TKSimpleTLVRecord(parent.0))
        } else {
            Err("This TKTLVRecord cannot be downcasted to TKSimpleTLVRecord")
        }
    }
}
impl INSObject for TKSimpleTLVRecord {}
impl PNSObject for TKSimpleTLVRecord {}
impl ITKSimpleTLVRecord for TKSimpleTLVRecord {}
pub trait ITKSimpleTLVRecord: Sized + std::ops::Deref {
    unsafe fn initWithTag_value_(&self, tag: UInt8, value: NSData) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithTag : tag, value : value)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct TKCompactTLVRecord(pub id);
impl std::ops::Deref for TKCompactTLVRecord {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for TKCompactTLVRecord {}
impl TKCompactTLVRecord {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"TKCompactTLVRecord").unwrap(), alloc) })
    }
}
impl ITKTLVRecord for TKCompactTLVRecord {}
impl std::convert::TryFrom<TKTLVRecord> for TKCompactTLVRecord {
    type Error = &'static str;
    fn try_from(parent: TKTLVRecord) -> Result<TKCompactTLVRecord, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"TKCompactTLVRecord").unwrap()) };
        if is_kind_of {
            Ok(TKCompactTLVRecord(parent.0))
        } else {
            Err("This TKTLVRecord cannot be downcasted to TKCompactTLVRecord")
        }
    }
}
impl INSObject for TKCompactTLVRecord {}
impl PNSObject for TKCompactTLVRecord {}
impl ITKCompactTLVRecord for TKCompactTLVRecord {}
pub trait ITKCompactTLVRecord: Sized + std::ops::Deref {
    unsafe fn initWithTag_value_(&self, tag: UInt8, value: NSData) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithTag : tag, value : value)
    }
}
pub type TKSmartCardProtocol = NSUInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct TKSmartCardATRInterfaceGroup(pub id);
impl std::ops::Deref for TKSmartCardATRInterfaceGroup {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for TKSmartCardATRInterfaceGroup {}
impl TKSmartCardATRInterfaceGroup {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"TKSmartCardATRInterfaceGroup").unwrap(), alloc) })
    }
}
impl INSObject for TKSmartCardATRInterfaceGroup {}
impl PNSObject for TKSmartCardATRInterfaceGroup {}
impl std::convert::TryFrom<NSObject> for TKSmartCardATRInterfaceGroup {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<TKSmartCardATRInterfaceGroup, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"TKSmartCardATRInterfaceGroup").unwrap()) };
        if is_kind_of {
            Ok(TKSmartCardATRInterfaceGroup(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to TKSmartCardATRInterfaceGroup")
        }
    }
}
impl ITKSmartCardATRInterfaceGroup for TKSmartCardATRInterfaceGroup {}
pub trait ITKSmartCardATRInterfaceGroup: Sized + std::ops::Deref {
    unsafe fn TA(&self) -> NSNumber
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, TA)
    }
    unsafe fn TB(&self) -> NSNumber
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, TB)
    }
    unsafe fn TC(&self) -> NSNumber
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, TC)
    }
    unsafe fn protocol(&self) -> NSNumber
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, protocol)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct TKSmartCardATR(pub id);
impl std::ops::Deref for TKSmartCardATR {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for TKSmartCardATR {}
impl TKSmartCardATR {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"TKSmartCardATR").unwrap(), alloc) })
    }
}
impl INSObject for TKSmartCardATR {}
impl PNSObject for TKSmartCardATR {}
impl std::convert::TryFrom<NSObject> for TKSmartCardATR {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<TKSmartCardATR, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"TKSmartCardATR").unwrap()) };
        if is_kind_of {
            Ok(TKSmartCardATR(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to TKSmartCardATR")
        }
    }
}
impl ITKSmartCardATR for TKSmartCardATR {}
pub trait ITKSmartCardATR: Sized + std::ops::Deref {
    unsafe fn initWithBytes_(&self, bytes: NSData) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithBytes : bytes)
    }
    unsafe fn initWithSource_(&self, source: *mut ::std::os::raw::c_void) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithSource : source)
    }
    unsafe fn interfaceGroupAtIndex_(&self, index: NSInteger) -> TKSmartCardATRInterfaceGroup
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, interfaceGroupAtIndex : index)
    }
    unsafe fn interfaceGroupForProtocol_(
        &self,
        protocol: TKSmartCardProtocol,
    ) -> TKSmartCardATRInterfaceGroup
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, interfaceGroupForProtocol : protocol)
    }
    unsafe fn bytes(&self) -> NSData
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, bytes)
    }
    unsafe fn protocols(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, protocols)
    }
    unsafe fn historicalBytes(&self) -> NSData
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, historicalBytes)
    }
    unsafe fn historicalRecords(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, historicalRecords)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct TKSmartCardSlotNFCSession(pub id);
impl std::ops::Deref for TKSmartCardSlotNFCSession {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for TKSmartCardSlotNFCSession {}
impl TKSmartCardSlotNFCSession {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"TKSmartCardSlotNFCSession").unwrap(), alloc) })
    }
}
impl INSObject for TKSmartCardSlotNFCSession {}
impl PNSObject for TKSmartCardSlotNFCSession {}
impl std::convert::TryFrom<NSObject> for TKSmartCardSlotNFCSession {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<TKSmartCardSlotNFCSession, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"TKSmartCardSlotNFCSession").unwrap()) };
        if is_kind_of {
            Ok(TKSmartCardSlotNFCSession(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to TKSmartCardSlotNFCSession")
        }
    }
}
impl ITKSmartCardSlotNFCSession for TKSmartCardSlotNFCSession {}
pub trait ITKSmartCardSlotNFCSession: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn updateWithMessage_error_(&self, message: NSString, error: *mut NSError) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, updateWithMessage : message, error : error)
    }
    unsafe fn endSession(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, endSession)
    }
    unsafe fn slotName(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, slotName)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct TKSmartCardSlotManager(pub id);
impl std::ops::Deref for TKSmartCardSlotManager {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for TKSmartCardSlotManager {}
impl TKSmartCardSlotManager {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"TKSmartCardSlotManager").unwrap(), alloc) })
    }
}
impl INSObject for TKSmartCardSlotManager {}
impl PNSObject for TKSmartCardSlotManager {}
impl std::convert::TryFrom<NSObject> for TKSmartCardSlotManager {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<TKSmartCardSlotManager, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"TKSmartCardSlotManager").unwrap()) };
        if is_kind_of {
            Ok(TKSmartCardSlotManager(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to TKSmartCardSlotManager")
        }
    }
}
impl ITKSmartCardSlotManager for TKSmartCardSlotManager {}
pub trait ITKSmartCardSlotManager: Sized + std::ops::Deref {
    unsafe fn getSlotWithName_reply_(&self, name: NSString, reply: *mut ::std::os::raw::c_void)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, getSlotWithName : name, reply : reply)
    }
    unsafe fn slotNamed_(&self, name: NSString) -> TKSmartCardSlot
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, slotNamed : name)
    }
    unsafe fn createNFCSlotWithMessage_completion_(
        &self,
        message: NSString,
        completion: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, createNFCSlotWithMessage : message, completion : completion)
    }
    unsafe fn isNFCSupported(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isNFCSupported)
    }
    unsafe fn slotNames(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, slotNames)
    }
    unsafe fn defaultManager() -> TKSmartCardSlotManager
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"TKSmartCardSlotManager").unwrap(), defaultManager)
    }
}
pub type TKSmartCardSlotState = NSInteger;
pub type TKSmartCardPINCharset = NSInteger;
pub type TKSmartCardPINEncoding = NSInteger;
pub type TKSmartCardPINJustification = NSInteger;
pub type TKSmartCardPINCompletion = NSUInteger;
pub type TKSmartCardPINConfirmation = NSUInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct TKSmartCardPINFormat(pub id);
impl std::ops::Deref for TKSmartCardPINFormat {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for TKSmartCardPINFormat {}
impl TKSmartCardPINFormat {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"TKSmartCardPINFormat").unwrap(), alloc) })
    }
}
impl INSObject for TKSmartCardPINFormat {}
impl PNSObject for TKSmartCardPINFormat {}
impl std::convert::TryFrom<NSObject> for TKSmartCardPINFormat {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<TKSmartCardPINFormat, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"TKSmartCardPINFormat").unwrap()) };
        if is_kind_of {
            Ok(TKSmartCardPINFormat(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to TKSmartCardPINFormat")
        }
    }
}
impl ITKSmartCardPINFormat for TKSmartCardPINFormat {}
pub trait ITKSmartCardPINFormat: Sized + std::ops::Deref {
    unsafe fn charset(&self) -> TKSmartCardPINCharset
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, charset)
    }
    unsafe fn setCharset_(&self, charset: TKSmartCardPINCharset)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCharset : charset)
    }
    unsafe fn encoding(&self) -> TKSmartCardPINEncoding
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, encoding)
    }
    unsafe fn setEncoding_(&self, encoding: TKSmartCardPINEncoding)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setEncoding : encoding)
    }
    unsafe fn minPINLength(&self) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, minPINLength)
    }
    unsafe fn setMinPINLength_(&self, minPINLength: NSInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMinPINLength : minPINLength)
    }
    unsafe fn maxPINLength(&self) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, maxPINLength)
    }
    unsafe fn setMaxPINLength_(&self, maxPINLength: NSInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMaxPINLength : maxPINLength)
    }
    unsafe fn PINBlockByteLength(&self) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, PINBlockByteLength)
    }
    unsafe fn setPINBlockByteLength_(&self, PINBlockByteLength: NSInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPINBlockByteLength : PINBlockByteLength)
    }
    unsafe fn PINJustification(&self) -> TKSmartCardPINJustification
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, PINJustification)
    }
    unsafe fn setPINJustification_(&self, PINJustification: TKSmartCardPINJustification)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPINJustification : PINJustification)
    }
    unsafe fn PINBitOffset(&self) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, PINBitOffset)
    }
    unsafe fn setPINBitOffset_(&self, PINBitOffset: NSInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPINBitOffset : PINBitOffset)
    }
    unsafe fn PINLengthBitOffset(&self) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, PINLengthBitOffset)
    }
    unsafe fn setPINLengthBitOffset_(&self, PINLengthBitOffset: NSInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPINLengthBitOffset : PINLengthBitOffset)
    }
    unsafe fn PINLengthBitSize(&self) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, PINLengthBitSize)
    }
    unsafe fn setPINLengthBitSize_(&self, PINLengthBitSize: NSInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPINLengthBitSize : PINLengthBitSize)
    }
}
pub trait PTKSmartCardUserInteractionDelegate: Sized + std::ops::Deref {
    unsafe fn characterEnteredInUserInteraction_(&self, interaction: TKSmartCardUserInteraction)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, characterEnteredInUserInteraction : interaction)
    }
    unsafe fn correctionKeyPressedInUserInteraction_(&self, interaction: TKSmartCardUserInteraction)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, correctionKeyPressedInUserInteraction : interaction)
    }
    unsafe fn validationKeyPressedInUserInteraction_(&self, interaction: TKSmartCardUserInteraction)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, validationKeyPressedInUserInteraction : interaction)
    }
    unsafe fn invalidCharacterEnteredInUserInteraction_(
        &self,
        interaction: TKSmartCardUserInteraction,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, invalidCharacterEnteredInUserInteraction : interaction)
    }
    unsafe fn oldPINRequestedInUserInteraction_(&self, interaction: TKSmartCardUserInteraction)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, oldPINRequestedInUserInteraction : interaction)
    }
    unsafe fn newPINRequestedInUserInteraction_(&self, interaction: TKSmartCardUserInteraction)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, newPINRequestedInUserInteraction : interaction)
    }
    unsafe fn newPINConfirmationRequestedInUserInteraction_(
        &self,
        interaction: TKSmartCardUserInteraction,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, newPINConfirmationRequestedInUserInteraction : interaction)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct TKSmartCardUserInteraction(pub id);
impl std::ops::Deref for TKSmartCardUserInteraction {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for TKSmartCardUserInteraction {}
impl TKSmartCardUserInteraction {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"TKSmartCardUserInteraction").unwrap(), alloc) })
    }
}
impl INSObject for TKSmartCardUserInteraction {}
impl PNSObject for TKSmartCardUserInteraction {}
impl std::convert::TryFrom<NSObject> for TKSmartCardUserInteraction {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<TKSmartCardUserInteraction, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"TKSmartCardUserInteraction").unwrap()) };
        if is_kind_of {
            Ok(TKSmartCardUserInteraction(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to TKSmartCardUserInteraction")
        }
    }
}
impl ITKSmartCardUserInteraction for TKSmartCardUserInteraction {}
pub trait ITKSmartCardUserInteraction: Sized + std::ops::Deref {
    unsafe fn runWithReply_(&self, reply: *mut ::std::os::raw::c_void)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, runWithReply : reply)
    }
    unsafe fn cancel(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, cancel)
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
    unsafe fn initialTimeout(&self) -> NSTimeInterval
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, initialTimeout)
    }
    unsafe fn setInitialTimeout_(&self, initialTimeout: NSTimeInterval)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInitialTimeout : initialTimeout)
    }
    unsafe fn interactionTimeout(&self) -> NSTimeInterval
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, interactionTimeout)
    }
    unsafe fn setInteractionTimeout_(&self, interactionTimeout: NSTimeInterval)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInteractionTimeout : interactionTimeout)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct TKSmartCardUserInteractionForPINOperation(pub id);
impl std::ops::Deref for TKSmartCardUserInteractionForPINOperation {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for TKSmartCardUserInteractionForPINOperation {}
impl TKSmartCardUserInteractionForPINOperation {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"TKSmartCardUserInteractionForPINOperation").unwrap(), alloc) })
    }
}
impl ITKSmartCardUserInteraction for TKSmartCardUserInteractionForPINOperation {}
impl From<TKSmartCardUserInteractionForPINOperation> for TKSmartCardUserInteraction {
    fn from(child: TKSmartCardUserInteractionForPINOperation) -> TKSmartCardUserInteraction {
        TKSmartCardUserInteraction(child.0)
    }
}
impl std::convert::TryFrom<TKSmartCardUserInteraction>
    for TKSmartCardUserInteractionForPINOperation
{
    type Error = &'static str;
    fn try_from(
        parent: TKSmartCardUserInteraction,
    ) -> Result<TKSmartCardUserInteractionForPINOperation, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"TKSmartCardUserInteractionForPINOperation").unwrap())
        };
        if is_kind_of {
            Ok(TKSmartCardUserInteractionForPINOperation(parent.0))
        } else {
            Err ("This TKSmartCardUserInteraction cannot be downcasted to TKSmartCardUserInteractionForPINOperation" ,)
        }
    }
}
impl INSObject for TKSmartCardUserInteractionForPINOperation {}
impl PNSObject for TKSmartCardUserInteractionForPINOperation {}
impl ITKSmartCardUserInteractionForPINOperation for TKSmartCardUserInteractionForPINOperation {}
pub trait ITKSmartCardUserInteractionForPINOperation: Sized + std::ops::Deref {
    unsafe fn PINCompletion(&self) -> TKSmartCardPINCompletion
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, PINCompletion)
    }
    unsafe fn setPINCompletion_(&self, PINCompletion: TKSmartCardPINCompletion)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPINCompletion : PINCompletion)
    }
    unsafe fn PINMessageIndices(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, PINMessageIndices)
    }
    unsafe fn setPINMessageIndices_(&self, PINMessageIndices: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPINMessageIndices : PINMessageIndices)
    }
    unsafe fn locale(&self) -> NSLocale
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, locale)
    }
    unsafe fn setLocale_(&self, locale: NSLocale)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setLocale : locale)
    }
    unsafe fn resultSW(&self) -> UInt16
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, resultSW)
    }
    unsafe fn setResultSW_(&self, resultSW: UInt16)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setResultSW : resultSW)
    }
    unsafe fn resultData(&self) -> NSData
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, resultData)
    }
    unsafe fn setResultData_(&self, resultData: NSData)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setResultData : resultData)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct TKSmartCardUserInteractionForSecurePINVerification(pub id);
impl std::ops::Deref for TKSmartCardUserInteractionForSecurePINVerification {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for TKSmartCardUserInteractionForSecurePINVerification {}
impl TKSmartCardUserInteractionForSecurePINVerification {
    pub fn alloc() -> Self {
        Self(unsafe {
            msg_send!(&*objc2::runtime::AnyClass::get(c"TKSmartCardUserInteractionForSecurePINVerification").unwrap(), alloc)
        })
    }
}
impl ITKSmartCardUserInteractionForPINOperation
    for TKSmartCardUserInteractionForSecurePINVerification
{
}
impl From<TKSmartCardUserInteractionForSecurePINVerification>
    for TKSmartCardUserInteractionForPINOperation
{
    fn from(
        child: TKSmartCardUserInteractionForSecurePINVerification,
    ) -> TKSmartCardUserInteractionForPINOperation {
        TKSmartCardUserInteractionForPINOperation(child.0)
    }
}
impl std::convert::TryFrom<TKSmartCardUserInteractionForPINOperation>
    for TKSmartCardUserInteractionForSecurePINVerification
{
    type Error = &'static str;
    fn try_from(
        parent: TKSmartCardUserInteractionForPINOperation,
    ) -> Result<TKSmartCardUserInteractionForSecurePINVerification, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"TKSmartCardUserInteractionForSecurePINVerification").unwrap())
        };
        if is_kind_of {
            Ok(TKSmartCardUserInteractionForSecurePINVerification(parent.0))
        } else {
            Err ("This TKSmartCardUserInteractionForPINOperation cannot be downcasted to TKSmartCardUserInteractionForSecurePINVerification" ,)
        }
    }
}
impl ITKSmartCardUserInteraction for TKSmartCardUserInteractionForSecurePINVerification {}
impl INSObject for TKSmartCardUserInteractionForSecurePINVerification {}
impl PNSObject for TKSmartCardUserInteractionForSecurePINVerification {}
impl ITKSmartCardUserInteractionForSecurePINVerification
    for TKSmartCardUserInteractionForSecurePINVerification
{
}
pub trait ITKSmartCardUserInteractionForSecurePINVerification: Sized + std::ops::Deref {}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct TKSmartCardUserInteractionForSecurePINChange(pub id);
impl std::ops::Deref for TKSmartCardUserInteractionForSecurePINChange {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for TKSmartCardUserInteractionForSecurePINChange {}
impl TKSmartCardUserInteractionForSecurePINChange {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"TKSmartCardUserInteractionForSecurePINChange").unwrap(), alloc) })
    }
}
impl ITKSmartCardUserInteractionForPINOperation for TKSmartCardUserInteractionForSecurePINChange {}
impl std::convert::TryFrom<TKSmartCardUserInteractionForPINOperation>
    for TKSmartCardUserInteractionForSecurePINChange
{
    type Error = &'static str;
    fn try_from(
        parent: TKSmartCardUserInteractionForPINOperation,
    ) -> Result<TKSmartCardUserInteractionForSecurePINChange, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"TKSmartCardUserInteractionForSecurePINChange").unwrap())
        };
        if is_kind_of {
            Ok(TKSmartCardUserInteractionForSecurePINChange(parent.0))
        } else {
            Err ("This TKSmartCardUserInteractionForPINOperation cannot be downcasted to TKSmartCardUserInteractionForSecurePINChange" ,)
        }
    }
}
impl ITKSmartCardUserInteraction for TKSmartCardUserInteractionForSecurePINChange {}
impl INSObject for TKSmartCardUserInteractionForSecurePINChange {}
impl PNSObject for TKSmartCardUserInteractionForSecurePINChange {}
impl ITKSmartCardUserInteractionForSecurePINChange
    for TKSmartCardUserInteractionForSecurePINChange
{
}
pub trait ITKSmartCardUserInteractionForSecurePINChange: Sized + std::ops::Deref {
    unsafe fn PINConfirmation(&self) -> TKSmartCardPINConfirmation
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, PINConfirmation)
    }
    unsafe fn setPINConfirmation_(&self, PINConfirmation: TKSmartCardPINConfirmation)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPINConfirmation : PINConfirmation)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct TKSmartCardSlot(pub id);
impl std::ops::Deref for TKSmartCardSlot {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for TKSmartCardSlot {}
impl TKSmartCardSlot {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"TKSmartCardSlot").unwrap(), alloc) })
    }
}
impl INSObject for TKSmartCardSlot {}
impl PNSObject for TKSmartCardSlot {}
impl std::convert::TryFrom<NSObject> for TKSmartCardSlot {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<TKSmartCardSlot, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"TKSmartCardSlot").unwrap()) };
        if is_kind_of {
            Ok(TKSmartCardSlot(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to TKSmartCardSlot")
        }
    }
}
impl ITKSmartCardSlot for TKSmartCardSlot {}
pub trait ITKSmartCardSlot: Sized + std::ops::Deref {
    unsafe fn makeSmartCard(&self) -> TKSmartCard
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, makeSmartCard)
    }
    unsafe fn state(&self) -> TKSmartCardSlotState
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, state)
    }
    unsafe fn ATR(&self) -> TKSmartCardATR
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, ATR)
    }
    unsafe fn name(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, name)
    }
    unsafe fn maxInputLength(&self) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, maxInputLength)
    }
    unsafe fn maxOutputLength(&self) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, maxOutputLength)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct TKSmartCard(pub id);
impl std::ops::Deref for TKSmartCard {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for TKSmartCard {}
impl TKSmartCard {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"TKSmartCard").unwrap(), alloc) })
    }
}
impl INSObject for TKSmartCard {}
impl PNSObject for TKSmartCard {}
impl std::convert::TryFrom<NSObject> for TKSmartCard {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<TKSmartCard, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"TKSmartCard").unwrap()) };
        if is_kind_of {
            Ok(TKSmartCard(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to TKSmartCard")
        }
    }
}
impl ITKSmartCard for TKSmartCard {}
pub trait ITKSmartCard: Sized + std::ops::Deref {
    unsafe fn beginSessionWithReply_(&self, reply: *mut ::std::os::raw::c_void)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, beginSessionWithReply : reply)
    }
    unsafe fn transmitRequest_reply_(&self, request: NSData, reply: *mut ::std::os::raw::c_void)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, transmitRequest : request, reply : reply)
    }
    unsafe fn endSession(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, endSession)
    }
    unsafe fn userInteractionForSecurePINVerificationWithPINFormat_APDU_PINByteOffset_(
        &self,
        PINFormat: TKSmartCardPINFormat,
        APDU: NSData,
        PINByteOffset: NSInteger,
    ) -> TKSmartCardUserInteractionForSecurePINVerification
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, userInteractionForSecurePINVerificationWithPINFormat : PINFormat, APDU : APDU, PINByteOffset : PINByteOffset)
    }
    unsafe fn userInteractionForSecurePINChangeWithPINFormat_APDU_currentPINByteOffset_newPINByteOffset_(
        &self,
        PINFormat: TKSmartCardPINFormat,
        APDU: NSData,
        currentPINByteOffset: NSInteger,
        newPINByteOffset: NSInteger,
    ) -> TKSmartCardUserInteractionForSecurePINChange
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, userInteractionForSecurePINChangeWithPINFormat : PINFormat, APDU : APDU, currentPINByteOffset : currentPINByteOffset, newPINByteOffset : newPINByteOffset)
    }
    unsafe fn slot(&self) -> TKSmartCardSlot
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, slot)
    }
    unsafe fn valid(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, valid)
    }
    unsafe fn allowedProtocols(&self) -> TKSmartCardProtocol
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, allowedProtocols)
    }
    unsafe fn setAllowedProtocols_(&self, allowedProtocols: TKSmartCardProtocol)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAllowedProtocols : allowedProtocols)
    }
    unsafe fn currentProtocol(&self) -> TKSmartCardProtocol
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, currentProtocol)
    }
    unsafe fn sensitive(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, sensitive)
    }
    unsafe fn setSensitive_(&self, sensitive: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSensitive : sensitive)
    }
    unsafe fn context(&self) -> id
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, context)
    }
    unsafe fn setContext_(&self, context: id)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setContext : context)
    }
}
impl TKSmartCard_APDULevelTransmit for TKSmartCard {}
pub trait TKSmartCard_APDULevelTransmit: Sized + std::ops::Deref {
    unsafe fn sendIns_p1_p2_data_le_reply_(
        &self,
        ins: UInt8,
        p1: UInt8,
        p2: UInt8,
        requestData: NSData,
        le: NSNumber,
        reply: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, sendIns : ins, p1 : p1, p2 : p2, data : requestData, le : le, reply : reply)
    }
    unsafe fn inSessionWithError_executeBlock_(
        &self,
        error: *mut NSError,
        block: *mut ::std::os::raw::c_void,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, inSessionWithError : error, executeBlock : block)
    }
    unsafe fn sendIns_p1_p2_data_le_sw_error_(
        &self,
        ins: UInt8,
        p1: UInt8,
        p2: UInt8,
        requestData: NSData,
        le: NSNumber,
        sw: *mut UInt16,
        error: *mut NSError,
    ) -> NSData
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, sendIns : ins, p1 : p1, p2 : p2, data : requestData, le : le, sw : sw, error : error)
    }
    unsafe fn cla(&self) -> UInt8
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, cla)
    }
    unsafe fn setCla_(&self, cla: UInt8)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCla : cla)
    }
    unsafe fn useExtendedLength(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, useExtendedLength)
    }
    unsafe fn setUseExtendedLength_(&self, useExtendedLength: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setUseExtendedLength : useExtendedLength)
    }
    unsafe fn useCommandChaining(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, useCommandChaining)
    }
    unsafe fn setUseCommandChaining_(&self, useCommandChaining: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setUseCommandChaining : useCommandChaining)
    }
}
pub type TKTokenObjectID = id;
pub type TKTokenInstanceID = NSString;
pub type TKTokenDriverClassID = NSString;
pub type TKTokenOperation = NSInteger;
pub type TKTokenOperationConstraint = id;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct TKTokenKeyAlgorithm(pub id);
impl std::ops::Deref for TKTokenKeyAlgorithm {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for TKTokenKeyAlgorithm {}
impl TKTokenKeyAlgorithm {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"TKTokenKeyAlgorithm").unwrap(), alloc) })
    }
}
impl INSObject for TKTokenKeyAlgorithm {}
impl PNSObject for TKTokenKeyAlgorithm {}
impl std::convert::TryFrom<NSObject> for TKTokenKeyAlgorithm {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<TKTokenKeyAlgorithm, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"TKTokenKeyAlgorithm").unwrap()) };
        if is_kind_of {
            Ok(TKTokenKeyAlgorithm(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to TKTokenKeyAlgorithm")
        }
    }
}
impl ITKTokenKeyAlgorithm for TKTokenKeyAlgorithm {}
pub trait ITKTokenKeyAlgorithm: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn isAlgorithm_(&self, algorithm: SecKeyAlgorithm) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, isAlgorithm : algorithm)
    }
    unsafe fn supportsAlgorithm_(&self, algorithm: SecKeyAlgorithm) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, supportsAlgorithm : algorithm)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct TKTokenKeyExchangeParameters(pub id);
impl std::ops::Deref for TKTokenKeyExchangeParameters {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for TKTokenKeyExchangeParameters {}
impl TKTokenKeyExchangeParameters {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"TKTokenKeyExchangeParameters").unwrap(), alloc) })
    }
}
impl INSObject for TKTokenKeyExchangeParameters {}
impl PNSObject for TKTokenKeyExchangeParameters {}
impl std::convert::TryFrom<NSObject> for TKTokenKeyExchangeParameters {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<TKTokenKeyExchangeParameters, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"TKTokenKeyExchangeParameters").unwrap()) };
        if is_kind_of {
            Ok(TKTokenKeyExchangeParameters(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to TKTokenKeyExchangeParameters")
        }
    }
}
impl ITKTokenKeyExchangeParameters for TKTokenKeyExchangeParameters {}
pub trait ITKTokenKeyExchangeParameters: Sized + std::ops::Deref {
    unsafe fn requestedSize(&self) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, requestedSize)
    }
    unsafe fn sharedInfo(&self) -> NSData
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, sharedInfo)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct TKTokenSession(pub id);
impl std::ops::Deref for TKTokenSession {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for TKTokenSession {}
impl TKTokenSession {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"TKTokenSession").unwrap(), alloc) })
    }
}
impl INSObject for TKTokenSession {}
impl PNSObject for TKTokenSession {}
impl std::convert::TryFrom<NSObject> for TKTokenSession {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<TKTokenSession, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"TKTokenSession").unwrap()) };
        if is_kind_of {
            Ok(TKTokenSession(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to TKTokenSession")
        }
    }
}
impl ITKTokenSession for TKTokenSession {}
pub trait ITKTokenSession: Sized + std::ops::Deref {
    unsafe fn initWithToken_(&self, token: TKToken) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithToken : token)
    }
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn token(&self) -> TKToken
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, token)
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
pub trait PTKTokenSessionDelegate: Sized + std::ops::Deref {
    unsafe fn tokenSession_beginAuthForOperation_constraint_error_(
        &self,
        session: TKTokenSession,
        operation: TKTokenOperation,
        constraint: TKTokenOperationConstraint,
        error: *mut NSError,
    ) -> TKTokenAuthOperation
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, tokenSession : session, beginAuthForOperation : operation, constraint : constraint, error : error)
    }
    unsafe fn tokenSession_supportsOperation_usingKey_algorithm_(
        &self,
        session: TKTokenSession,
        operation: TKTokenOperation,
        keyObjectID: TKTokenObjectID,
        algorithm: TKTokenKeyAlgorithm,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, tokenSession : session, supportsOperation : operation, usingKey : keyObjectID, algorithm : algorithm)
    }
    unsafe fn tokenSession_signData_usingKey_algorithm_error_(
        &self,
        session: TKTokenSession,
        dataToSign: NSData,
        keyObjectID: TKTokenObjectID,
        algorithm: TKTokenKeyAlgorithm,
        error: *mut NSError,
    ) -> NSData
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, tokenSession : session, signData : dataToSign, usingKey : keyObjectID, algorithm : algorithm, error : error)
    }
    unsafe fn tokenSession_decryptData_usingKey_algorithm_error_(
        &self,
        session: TKTokenSession,
        ciphertext: NSData,
        keyObjectID: TKTokenObjectID,
        algorithm: TKTokenKeyAlgorithm,
        error: *mut NSError,
    ) -> NSData
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, tokenSession : session, decryptData : ciphertext, usingKey : keyObjectID, algorithm : algorithm, error : error)
    }
    unsafe fn tokenSession_performKeyExchangeWithPublicKey_usingKey_algorithm_parameters_error_(
        &self,
        session: TKTokenSession,
        otherPartyPublicKeyData: NSData,
        objectID: TKTokenObjectID,
        algorithm: TKTokenKeyAlgorithm,
        parameters: TKTokenKeyExchangeParameters,
        error: *mut NSError,
    ) -> NSData
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, tokenSession : session, performKeyExchangeWithPublicKey : otherPartyPublicKeyData, usingKey : objectID, algorithm : algorithm, parameters : parameters, error : error)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct TKToken(pub id);
impl std::ops::Deref for TKToken {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for TKToken {}
impl TKToken {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"TKToken").unwrap(), alloc) })
    }
}
impl INSObject for TKToken {}
impl PNSObject for TKToken {}
impl std::convert::TryFrom<NSObject> for TKToken {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<TKToken, Self::Error> {
        let is_kind_of: bool = unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"TKToken").unwrap()) };
        if is_kind_of {
            Ok(TKToken(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to TKToken")
        }
    }
}
impl ITKToken for TKToken {}
pub trait ITKToken: Sized + std::ops::Deref {
    unsafe fn initWithTokenDriver_instanceID_(
        &self,
        tokenDriver: TKTokenDriver,
        instanceID: NSString,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithTokenDriver : tokenDriver, instanceID : instanceID)
    }
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn tokenDriver(&self) -> TKTokenDriver
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, tokenDriver)
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
    unsafe fn configuration(&self) -> TKTokenConfiguration
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, configuration)
    }
    unsafe fn keychainContents(&self) -> TKTokenKeychainContents
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, keychainContents)
    }
}
pub trait PTKTokenDelegate: Sized + std::ops::Deref {
    unsafe fn token_createSessionWithError_(
        &self,
        token: TKToken,
        error: *mut NSError,
    ) -> TKTokenSession
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, token : token, createSessionWithError : error)
    }
    unsafe fn token_terminateSession_(&self, token: TKToken, session: TKTokenSession)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, token : token, terminateSession : session)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct TKTokenDriver(pub id);
impl std::ops::Deref for TKTokenDriver {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for TKTokenDriver {}
impl TKTokenDriver {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"TKTokenDriver").unwrap(), alloc) })
    }
}
impl INSObject for TKTokenDriver {}
impl PNSObject for TKTokenDriver {}
impl std::convert::TryFrom<NSObject> for TKTokenDriver {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<TKTokenDriver, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"TKTokenDriver").unwrap()) };
        if is_kind_of {
            Ok(TKTokenDriver(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to TKTokenDriver")
        }
    }
}
impl ITKTokenDriver for TKTokenDriver {}
pub trait ITKTokenDriver: Sized + std::ops::Deref {
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
pub trait PTKTokenDriverDelegate: Sized + std::ops::Deref {
    unsafe fn tokenDriver_tokenForConfiguration_error_(
        &self,
        driver: TKTokenDriver,
        configuration: TKTokenConfiguration,
        error: *mut NSError,
    ) -> TKToken
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, tokenDriver : driver, tokenForConfiguration : configuration, error : error)
    }
    unsafe fn tokenDriver_terminateToken_(&self, driver: TKTokenDriver, token: TKToken)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, tokenDriver : driver, terminateToken : token)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct TKTokenAuthOperation(pub id);
impl std::ops::Deref for TKTokenAuthOperation {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for TKTokenAuthOperation {}
impl TKTokenAuthOperation {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"TKTokenAuthOperation").unwrap(), alloc) })
    }
}
impl PNSSecureCoding for TKTokenAuthOperation {}
impl INSObject for TKTokenAuthOperation {}
impl PNSObject for TKTokenAuthOperation {}
impl std::convert::TryFrom<NSObject> for TKTokenAuthOperation {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<TKTokenAuthOperation, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"TKTokenAuthOperation").unwrap()) };
        if is_kind_of {
            Ok(TKTokenAuthOperation(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to TKTokenAuthOperation")
        }
    }
}
impl ITKTokenAuthOperation for TKTokenAuthOperation {}
pub trait ITKTokenAuthOperation: Sized + std::ops::Deref {
    unsafe fn finishWithError_(&self, error: *mut NSError) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, finishWithError : error)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct TKTokenPasswordAuthOperation(pub id);
impl std::ops::Deref for TKTokenPasswordAuthOperation {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for TKTokenPasswordAuthOperation {}
impl TKTokenPasswordAuthOperation {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"TKTokenPasswordAuthOperation").unwrap(), alloc) })
    }
}
impl ITKTokenAuthOperation for TKTokenPasswordAuthOperation {}
impl PNSSecureCoding for TKTokenPasswordAuthOperation {}
impl From<TKTokenPasswordAuthOperation> for TKTokenAuthOperation {
    fn from(child: TKTokenPasswordAuthOperation) -> TKTokenAuthOperation {
        TKTokenAuthOperation(child.0)
    }
}
impl std::convert::TryFrom<TKTokenAuthOperation> for TKTokenPasswordAuthOperation {
    type Error = &'static str;
    fn try_from(parent: TKTokenAuthOperation) -> Result<TKTokenPasswordAuthOperation, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"TKTokenPasswordAuthOperation").unwrap()) };
        if is_kind_of {
            Ok(TKTokenPasswordAuthOperation(parent.0))
        } else {
            Err("This TKTokenAuthOperation cannot be downcasted to TKTokenPasswordAuthOperation")
        }
    }
}
impl INSObject for TKTokenPasswordAuthOperation {}
impl PNSObject for TKTokenPasswordAuthOperation {}
impl ITKTokenPasswordAuthOperation for TKTokenPasswordAuthOperation {}
pub trait ITKTokenPasswordAuthOperation: Sized + std::ops::Deref {
    unsafe fn password(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, password)
    }
    unsafe fn setPassword_(&self, password: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPassword : password)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct TKTokenKeychainItem(pub id);
impl std::ops::Deref for TKTokenKeychainItem {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for TKTokenKeychainItem {}
impl TKTokenKeychainItem {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"TKTokenKeychainItem").unwrap(), alloc) })
    }
}
impl INSObject for TKTokenKeychainItem {}
impl PNSObject for TKTokenKeychainItem {}
impl std::convert::TryFrom<NSObject> for TKTokenKeychainItem {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<TKTokenKeychainItem, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"TKTokenKeychainItem").unwrap()) };
        if is_kind_of {
            Ok(TKTokenKeychainItem(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to TKTokenKeychainItem")
        }
    }
}
impl ITKTokenKeychainItem for TKTokenKeychainItem {}
pub trait ITKTokenKeychainItem: Sized + std::ops::Deref {
    unsafe fn initWithObjectID_(&self, objectID: TKTokenObjectID) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithObjectID : objectID)
    }
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn objectID(&self) -> TKTokenObjectID
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, objectID)
    }
    unsafe fn label(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, label)
    }
    unsafe fn setLabel_(&self, label: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setLabel : label)
    }
    unsafe fn constraints(&self) -> NSDictionary
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, constraints)
    }
    unsafe fn setConstraints_(&self, constraints: NSDictionary)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setConstraints : constraints)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct TKTokenKeychainCertificate(pub id);
impl std::ops::Deref for TKTokenKeychainCertificate {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for TKTokenKeychainCertificate {}
impl TKTokenKeychainCertificate {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"TKTokenKeychainCertificate").unwrap(), alloc) })
    }
}
impl ITKTokenKeychainItem for TKTokenKeychainCertificate {}
impl From<TKTokenKeychainCertificate> for TKTokenKeychainItem {
    fn from(child: TKTokenKeychainCertificate) -> TKTokenKeychainItem {
        TKTokenKeychainItem(child.0)
    }
}
impl std::convert::TryFrom<TKTokenKeychainItem> for TKTokenKeychainCertificate {
    type Error = &'static str;
    fn try_from(parent: TKTokenKeychainItem) -> Result<TKTokenKeychainCertificate, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"TKTokenKeychainCertificate").unwrap()) };
        if is_kind_of {
            Ok(TKTokenKeychainCertificate(parent.0))
        } else {
            Err("This TKTokenKeychainItem cannot be downcasted to TKTokenKeychainCertificate")
        }
    }
}
impl INSObject for TKTokenKeychainCertificate {}
impl PNSObject for TKTokenKeychainCertificate {}
impl ITKTokenKeychainCertificate for TKTokenKeychainCertificate {}
pub trait ITKTokenKeychainCertificate: Sized + std::ops::Deref {
    unsafe fn initWithCertificate_objectID_(
        &self,
        certificateRef: SecCertificateRef,
        objectID: TKTokenObjectID,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithCertificate : certificateRef, objectID : objectID)
    }
    unsafe fn initWithObjectID_(&self, objectID: TKTokenObjectID) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithObjectID : objectID)
    }
    unsafe fn data(&self) -> NSData
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, data)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct TKTokenKeychainKey(pub id);
impl std::ops::Deref for TKTokenKeychainKey {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for TKTokenKeychainKey {}
impl TKTokenKeychainKey {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"TKTokenKeychainKey").unwrap(), alloc) })
    }
}
impl ITKTokenKeychainItem for TKTokenKeychainKey {}
impl std::convert::TryFrom<TKTokenKeychainItem> for TKTokenKeychainKey {
    type Error = &'static str;
    fn try_from(parent: TKTokenKeychainItem) -> Result<TKTokenKeychainKey, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"TKTokenKeychainKey").unwrap()) };
        if is_kind_of {
            Ok(TKTokenKeychainKey(parent.0))
        } else {
            Err("This TKTokenKeychainItem cannot be downcasted to TKTokenKeychainKey")
        }
    }
}
impl INSObject for TKTokenKeychainKey {}
impl PNSObject for TKTokenKeychainKey {}
impl ITKTokenKeychainKey for TKTokenKeychainKey {}
pub trait ITKTokenKeychainKey: Sized + std::ops::Deref {
    unsafe fn initWithCertificate_objectID_(
        &self,
        certificateRef: SecCertificateRef,
        objectID: TKTokenObjectID,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithCertificate : certificateRef, objectID : objectID)
    }
    unsafe fn initWithObjectID_(&self, objectID: TKTokenObjectID) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithObjectID : objectID)
    }
    unsafe fn keyType(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, keyType)
    }
    unsafe fn setKeyType_(&self, keyType: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setKeyType : keyType)
    }
    unsafe fn applicationTag(&self) -> NSData
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, applicationTag)
    }
    unsafe fn setApplicationTag_(&self, applicationTag: NSData)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setApplicationTag : applicationTag)
    }
    unsafe fn keySizeInBits(&self) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, keySizeInBits)
    }
    unsafe fn setKeySizeInBits_(&self, keySizeInBits: NSInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setKeySizeInBits : keySizeInBits)
    }
    unsafe fn publicKeyData(&self) -> NSData
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, publicKeyData)
    }
    unsafe fn setPublicKeyData_(&self, publicKeyData: NSData)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPublicKeyData : publicKeyData)
    }
    unsafe fn publicKeyHash(&self) -> NSData
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, publicKeyHash)
    }
    unsafe fn setPublicKeyHash_(&self, publicKeyHash: NSData)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPublicKeyHash : publicKeyHash)
    }
    unsafe fn canDecrypt(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, canDecrypt)
    }
    unsafe fn setCanDecrypt_(&self, canDecrypt: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCanDecrypt : canDecrypt)
    }
    unsafe fn canSign(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, canSign)
    }
    unsafe fn setCanSign_(&self, canSign: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCanSign : canSign)
    }
    unsafe fn canPerformKeyExchange(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, canPerformKeyExchange)
    }
    unsafe fn setCanPerformKeyExchange_(&self, canPerformKeyExchange: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCanPerformKeyExchange : canPerformKeyExchange)
    }
    unsafe fn isSuitableForLogin(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isSuitableForLogin)
    }
    unsafe fn setSuitableForLogin_(&self, suitableForLogin: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSuitableForLogin : suitableForLogin)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct TKTokenKeychainContents(pub id);
impl std::ops::Deref for TKTokenKeychainContents {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for TKTokenKeychainContents {}
impl TKTokenKeychainContents {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"TKTokenKeychainContents").unwrap(), alloc) })
    }
}
impl INSObject for TKTokenKeychainContents {}
impl PNSObject for TKTokenKeychainContents {}
impl std::convert::TryFrom<NSObject> for TKTokenKeychainContents {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<TKTokenKeychainContents, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"TKTokenKeychainContents").unwrap()) };
        if is_kind_of {
            Ok(TKTokenKeychainContents(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to TKTokenKeychainContents")
        }
    }
}
impl ITKTokenKeychainContents for TKTokenKeychainContents {}
pub trait ITKTokenKeychainContents: Sized + std::ops::Deref {
    unsafe fn fillWithItems_(&self, items: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, fillWithItems : items)
    }
    unsafe fn keyForObjectID_error_(
        &self,
        objectID: TKTokenObjectID,
        error: *mut NSError,
    ) -> TKTokenKeychainKey
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, keyForObjectID : objectID, error : error)
    }
    unsafe fn certificateForObjectID_error_(
        &self,
        objectID: TKTokenObjectID,
        error: *mut NSError,
    ) -> TKTokenKeychainCertificate
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, certificateForObjectID : objectID, error : error)
    }
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn items(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, items)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct TKTokenDriverConfiguration(pub id);
impl std::ops::Deref for TKTokenDriverConfiguration {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for TKTokenDriverConfiguration {}
impl TKTokenDriverConfiguration {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"TKTokenDriverConfiguration").unwrap(), alloc) })
    }
}
impl INSObject for TKTokenDriverConfiguration {}
impl PNSObject for TKTokenDriverConfiguration {}
impl std::convert::TryFrom<NSObject> for TKTokenDriverConfiguration {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<TKTokenDriverConfiguration, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"TKTokenDriverConfiguration").unwrap()) };
        if is_kind_of {
            Ok(TKTokenDriverConfiguration(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to TKTokenDriverConfiguration")
        }
    }
}
impl ITKTokenDriverConfiguration for TKTokenDriverConfiguration {}
pub trait ITKTokenDriverConfiguration: Sized + std::ops::Deref {
    unsafe fn addTokenConfigurationForTokenInstanceID_(
        &self,
        instanceID: NSString,
    ) -> TKTokenConfiguration
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, addTokenConfigurationForTokenInstanceID : instanceID)
    }
    unsafe fn removeTokenConfigurationForTokenInstanceID_(&self, instanceID: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, removeTokenConfigurationForTokenInstanceID : instanceID)
    }
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn classID(&self) -> TKTokenDriverClassID
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, classID)
    }
    unsafe fn tokenConfigurations(&self) -> NSDictionary
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, tokenConfigurations)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"TKTokenDriverConfiguration").unwrap(), new)
    }
    unsafe fn driverConfigurations() -> NSDictionary
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"TKTokenDriverConfiguration").unwrap(), driverConfigurations)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct TKTokenConfiguration(pub id);
impl std::ops::Deref for TKTokenConfiguration {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for TKTokenConfiguration {}
impl TKTokenConfiguration {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"TKTokenConfiguration").unwrap(), alloc) })
    }
}
impl INSObject for TKTokenConfiguration {}
impl PNSObject for TKTokenConfiguration {}
impl std::convert::TryFrom<NSObject> for TKTokenConfiguration {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<TKTokenConfiguration, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"TKTokenConfiguration").unwrap()) };
        if is_kind_of {
            Ok(TKTokenConfiguration(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to TKTokenConfiguration")
        }
    }
}
impl ITKTokenConfiguration for TKTokenConfiguration {}
pub trait ITKTokenConfiguration: Sized + std::ops::Deref {
    unsafe fn keyForObjectID_error_(
        &self,
        objectID: TKTokenObjectID,
        error: *mut NSError,
    ) -> TKTokenKeychainKey
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, keyForObjectID : objectID, error : error)
    }
    unsafe fn certificateForObjectID_error_(
        &self,
        objectID: TKTokenObjectID,
        error: *mut NSError,
    ) -> TKTokenKeychainCertificate
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, certificateForObjectID : objectID, error : error)
    }
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn instanceID(&self) -> TKTokenInstanceID
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, instanceID)
    }
    unsafe fn configurationData(&self) -> NSData
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, configurationData)
    }
    unsafe fn setConfigurationData_(&self, configurationData: NSData)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setConfigurationData : configurationData)
    }
    unsafe fn keychainItems(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, keychainItems)
    }
    unsafe fn setKeychainItems_(&self, keychainItems: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setKeychainItems : keychainItems)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"TKTokenConfiguration").unwrap(), new)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct TKTokenSmartCardPINAuthOperation(pub id);
impl std::ops::Deref for TKTokenSmartCardPINAuthOperation {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for TKTokenSmartCardPINAuthOperation {}
impl TKTokenSmartCardPINAuthOperation {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"TKTokenSmartCardPINAuthOperation").unwrap(), alloc) })
    }
}
impl ITKTokenAuthOperation for TKTokenSmartCardPINAuthOperation {}
impl PNSSecureCoding for TKTokenSmartCardPINAuthOperation {}
impl std::convert::TryFrom<TKTokenAuthOperation> for TKTokenSmartCardPINAuthOperation {
    type Error = &'static str;
    fn try_from(
        parent: TKTokenAuthOperation,
    ) -> Result<TKTokenSmartCardPINAuthOperation, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"TKTokenSmartCardPINAuthOperation").unwrap())
        };
        if is_kind_of {
            Ok(TKTokenSmartCardPINAuthOperation(parent.0))
        } else {
            Err ("This TKTokenAuthOperation cannot be downcasted to TKTokenSmartCardPINAuthOperation" ,)
        }
    }
}
impl INSObject for TKTokenSmartCardPINAuthOperation {}
impl PNSObject for TKTokenSmartCardPINAuthOperation {}
impl ITKTokenSmartCardPINAuthOperation for TKTokenSmartCardPINAuthOperation {}
pub trait ITKTokenSmartCardPINAuthOperation: Sized + std::ops::Deref {
    unsafe fn PINFormat(&self) -> TKSmartCardPINFormat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, PINFormat)
    }
    unsafe fn setPINFormat_(&self, PINFormat: TKSmartCardPINFormat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPINFormat : PINFormat)
    }
    unsafe fn APDUTemplate(&self) -> NSData
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, APDUTemplate)
    }
    unsafe fn setAPDUTemplate_(&self, APDUTemplate: NSData)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAPDUTemplate : APDUTemplate)
    }
    unsafe fn PINByteOffset(&self) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, PINByteOffset)
    }
    unsafe fn setPINByteOffset_(&self, PINByteOffset: NSInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPINByteOffset : PINByteOffset)
    }
    unsafe fn smartCard(&self) -> TKSmartCard
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, smartCard)
    }
    unsafe fn setSmartCard_(&self, smartCard: TKSmartCard)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSmartCard : smartCard)
    }
    unsafe fn PIN(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, PIN)
    }
    unsafe fn setPIN_(&self, PIN: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPIN : PIN)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct TKSmartCardTokenSession(pub id);
impl std::ops::Deref for TKSmartCardTokenSession {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for TKSmartCardTokenSession {}
impl TKSmartCardTokenSession {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"TKSmartCardTokenSession").unwrap(), alloc) })
    }
}
impl ITKTokenSession for TKSmartCardTokenSession {}
impl From<TKSmartCardTokenSession> for TKTokenSession {
    fn from(child: TKSmartCardTokenSession) -> TKTokenSession {
        TKTokenSession(child.0)
    }
}
impl std::convert::TryFrom<TKTokenSession> for TKSmartCardTokenSession {
    type Error = &'static str;
    fn try_from(parent: TKTokenSession) -> Result<TKSmartCardTokenSession, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"TKSmartCardTokenSession").unwrap()) };
        if is_kind_of {
            Ok(TKSmartCardTokenSession(parent.0))
        } else {
            Err("This TKTokenSession cannot be downcasted to TKSmartCardTokenSession")
        }
    }
}
impl INSObject for TKSmartCardTokenSession {}
impl PNSObject for TKSmartCardTokenSession {}
impl ITKSmartCardTokenSession for TKSmartCardTokenSession {}
pub trait ITKSmartCardTokenSession: Sized + std::ops::Deref {
    unsafe fn getSmartCardWithError_(&self, error: *mut NSError) -> TKSmartCard
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, getSmartCardWithError : error)
    }
    unsafe fn smartCard(&self) -> TKSmartCard
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, smartCard)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct TKSmartCardToken(pub id);
impl std::ops::Deref for TKSmartCardToken {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for TKSmartCardToken {}
impl TKSmartCardToken {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"TKSmartCardToken").unwrap(), alloc) })
    }
}
impl ITKToken for TKSmartCardToken {}
impl From<TKSmartCardToken> for TKToken {
    fn from(child: TKSmartCardToken) -> TKToken {
        TKToken(child.0)
    }
}
impl std::convert::TryFrom<TKToken> for TKSmartCardToken {
    type Error = &'static str;
    fn try_from(parent: TKToken) -> Result<TKSmartCardToken, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"TKSmartCardToken").unwrap()) };
        if is_kind_of {
            Ok(TKSmartCardToken(parent.0))
        } else {
            Err("This TKToken cannot be downcasted to TKSmartCardToken")
        }
    }
}
impl INSObject for TKSmartCardToken {}
impl PNSObject for TKSmartCardToken {}
impl ITKSmartCardToken for TKSmartCardToken {}
pub trait ITKSmartCardToken: Sized + std::ops::Deref {
    unsafe fn initWithSmartCard_AID_instanceID_tokenDriver_(
        &self,
        smartCard: TKSmartCard,
        AID: NSData,
        instanceID: NSString,
        tokenDriver: TKSmartCardTokenDriver,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithSmartCard : smartCard, AID : AID, instanceID : instanceID, tokenDriver : tokenDriver)
    }
    unsafe fn initWithTokenDriver_instanceID_(
        &self,
        tokenDriver: TKTokenDriver,
        instanceID: NSString,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithTokenDriver : tokenDriver, instanceID : instanceID)
    }
    unsafe fn AID(&self) -> NSData
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, AID)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct TKSmartCardTokenDriver(pub id);
impl std::ops::Deref for TKSmartCardTokenDriver {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for TKSmartCardTokenDriver {}
impl TKSmartCardTokenDriver {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"TKSmartCardTokenDriver").unwrap(), alloc) })
    }
}
impl ITKTokenDriver for TKSmartCardTokenDriver {}
impl From<TKSmartCardTokenDriver> for TKTokenDriver {
    fn from(child: TKSmartCardTokenDriver) -> TKTokenDriver {
        TKTokenDriver(child.0)
    }
}
impl std::convert::TryFrom<TKTokenDriver> for TKSmartCardTokenDriver {
    type Error = &'static str;
    fn try_from(parent: TKTokenDriver) -> Result<TKSmartCardTokenDriver, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"TKSmartCardTokenDriver").unwrap()) };
        if is_kind_of {
            Ok(TKSmartCardTokenDriver(parent.0))
        } else {
            Err("This TKTokenDriver cannot be downcasted to TKSmartCardTokenDriver")
        }
    }
}
impl INSObject for TKSmartCardTokenDriver {}
impl PNSObject for TKSmartCardTokenDriver {}
impl ITKSmartCardTokenDriver for TKSmartCardTokenDriver {}
pub trait ITKSmartCardTokenDriver: Sized + std::ops::Deref {}
pub trait PTKSmartCardTokenDriverDelegate: Sized + std::ops::Deref {
    unsafe fn tokenDriver_createTokenForSmartCard_AID_error_(
        &self,
        driver: TKSmartCardTokenDriver,
        smartCard: TKSmartCard,
        AID: NSData,
        error: *mut NSError,
    ) -> TKSmartCardToken
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, tokenDriver : driver, createTokenForSmartCard : smartCard, AID : AID, error : error)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct TKTokenWatcherTokenInfo(pub id);
impl std::ops::Deref for TKTokenWatcherTokenInfo {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for TKTokenWatcherTokenInfo {}
impl TKTokenWatcherTokenInfo {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"TKTokenWatcherTokenInfo").unwrap(), alloc) })
    }
}
impl INSObject for TKTokenWatcherTokenInfo {}
impl PNSObject for TKTokenWatcherTokenInfo {}
impl std::convert::TryFrom<NSObject> for TKTokenWatcherTokenInfo {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<TKTokenWatcherTokenInfo, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"TKTokenWatcherTokenInfo").unwrap()) };
        if is_kind_of {
            Ok(TKTokenWatcherTokenInfo(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to TKTokenWatcherTokenInfo")
        }
    }
}
impl ITKTokenWatcherTokenInfo for TKTokenWatcherTokenInfo {}
pub trait ITKTokenWatcherTokenInfo: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn tokenID(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, tokenID)
    }
    unsafe fn slotName(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, slotName)
    }
    unsafe fn driverName(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, driverName)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"TKTokenWatcherTokenInfo").unwrap(), new)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct TKTokenWatcher(pub id);
impl std::ops::Deref for TKTokenWatcher {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for TKTokenWatcher {}
impl TKTokenWatcher {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"TKTokenWatcher").unwrap(), alloc) })
    }
}
impl INSObject for TKTokenWatcher {}
impl PNSObject for TKTokenWatcher {}
impl std::convert::TryFrom<NSObject> for TKTokenWatcher {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<TKTokenWatcher, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"TKTokenWatcher").unwrap()) };
        if is_kind_of {
            Ok(TKTokenWatcher(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to TKTokenWatcher")
        }
    }
}
impl ITKTokenWatcher for TKTokenWatcher {}
pub trait ITKTokenWatcher: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn initWithInsertionHandler_(
        &self,
        insertionHandler: *mut ::std::os::raw::c_void,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithInsertionHandler : insertionHandler)
    }
    unsafe fn setInsertionHandler_(&self, insertionHandler: *mut ::std::os::raw::c_void)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInsertionHandler : insertionHandler)
    }
    unsafe fn addRemovalHandler_forTokenID_(
        &self,
        removalHandler: *mut ::std::os::raw::c_void,
        tokenID: NSString,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, addRemovalHandler : removalHandler, forTokenID : tokenID)
    }
    unsafe fn tokenInfoForTokenID_(&self, tokenID: NSString) -> TKTokenWatcherTokenInfo
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, tokenInfoForTokenID : tokenID)
    }
    unsafe fn tokenIDs(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, tokenIDs)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct TKSmartCardTokenRegistrationManager(pub id);
impl std::ops::Deref for TKSmartCardTokenRegistrationManager {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for TKSmartCardTokenRegistrationManager {}
impl TKSmartCardTokenRegistrationManager {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"TKSmartCardTokenRegistrationManager").unwrap(), alloc) })
    }
}
impl INSObject for TKSmartCardTokenRegistrationManager {}
impl PNSObject for TKSmartCardTokenRegistrationManager {}
impl std::convert::TryFrom<NSObject> for TKSmartCardTokenRegistrationManager {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<TKSmartCardTokenRegistrationManager, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"TKSmartCardTokenRegistrationManager").unwrap())
        };
        if is_kind_of {
            Ok(TKSmartCardTokenRegistrationManager(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to TKSmartCardTokenRegistrationManager")
        }
    }
}
impl ITKSmartCardTokenRegistrationManager for TKSmartCardTokenRegistrationManager {}
pub trait ITKSmartCardTokenRegistrationManager: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn registerSmartCardWithTokenID_promptMessage_error_(
        &self,
        tokenID: NSString,
        promptMessage: NSString,
        error: *mut NSError,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, registerSmartCardWithTokenID : tokenID, promptMessage : promptMessage, error : error)
    }
    unsafe fn unregisterSmartCardWithTokenID_error_(
        &self,
        tokenID: NSString,
        error: *mut NSError,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, unregisterSmartCardWithTokenID : tokenID, error : error)
    }
    unsafe fn registeredSmartCardTokens(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, registeredSmartCardTokens)
    }
    unsafe fn defaultManager() -> TKSmartCardTokenRegistrationManager
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"TKSmartCardTokenRegistrationManager").unwrap(), defaultManager)
    }
}
unsafe extern "C" {
    pub static TKErrorDomain: NSString;
}

unsafe impl objc2::encode::RefEncode for TKTLVRecord {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for TKTLVRecord {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for TKBERTLVRecord {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for TKBERTLVRecord {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for TKSimpleTLVRecord {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for TKSimpleTLVRecord {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for TKCompactTLVRecord {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for TKCompactTLVRecord {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for TKSmartCardATRInterfaceGroup {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for TKSmartCardATRInterfaceGroup {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for TKSmartCardATR {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for TKSmartCardATR {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for TKSmartCardSlotNFCSession {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for TKSmartCardSlotNFCSession {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for TKSmartCardSlotManager {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for TKSmartCardSlotManager {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for TKSmartCardPINFormat {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for TKSmartCardPINFormat {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for TKSmartCardUserInteraction {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for TKSmartCardUserInteraction {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for TKSmartCardUserInteractionForPINOperation {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for TKSmartCardUserInteractionForPINOperation {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for TKSmartCardUserInteractionForSecurePINVerification {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for TKSmartCardUserInteractionForSecurePINVerification {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for TKSmartCardUserInteractionForSecurePINChange {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for TKSmartCardUserInteractionForSecurePINChange {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for TKSmartCardSlot {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for TKSmartCardSlot {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for TKSmartCard {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for TKSmartCard {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for TKTokenKeyAlgorithm {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for TKTokenKeyAlgorithm {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for TKTokenKeyExchangeParameters {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for TKTokenKeyExchangeParameters {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for TKTokenSession {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for TKTokenSession {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for TKToken {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for TKToken {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for TKTokenDriver {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for TKTokenDriver {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for TKTokenAuthOperation {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for TKTokenAuthOperation {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for TKTokenPasswordAuthOperation {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for TKTokenPasswordAuthOperation {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for TKTokenKeychainItem {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for TKTokenKeychainItem {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for TKTokenKeychainCertificate {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for TKTokenKeychainCertificate {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for TKTokenKeychainKey {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for TKTokenKeychainKey {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for TKTokenKeychainContents {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for TKTokenKeychainContents {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for TKTokenDriverConfiguration {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for TKTokenDriverConfiguration {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for TKTokenConfiguration {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for TKTokenConfiguration {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for TKTokenSmartCardPINAuthOperation {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for TKTokenSmartCardPINAuthOperation {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for TKSmartCardTokenSession {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for TKSmartCardTokenSession {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for TKSmartCardToken {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for TKSmartCardToken {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for TKSmartCardTokenDriver {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for TKSmartCardTokenDriver {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for TKTokenWatcherTokenInfo {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for TKTokenWatcherTokenInfo {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for TKTokenWatcher {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for TKTokenWatcher {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for TKSmartCardTokenRegistrationManager {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for TKSmartCardTokenRegistrationManager {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
