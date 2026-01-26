#[allow(unused_imports)]
use crate::objc::*;
#[allow(unused_imports)]
use crate::AVFAudio::*;
#[allow(unused_imports)]
use crate::CoreFoundation::*;
#[allow(unused_imports)]
use crate::Foundation::*;

#[allow(unused_imports)]
use objc2::msg_send;
pub type CXErrorCode = NSInteger;
pub type CXErrorCodeIncomingCallError = NSInteger;
pub type CXErrorCodeRequestTransactionError = NSInteger;
pub type CXErrorCodeCallDirectoryManagerError = NSInteger;
pub type CXErrorCodeNotificationServiceExtensionError = NSInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CXAction(pub id);
impl std::ops::Deref for CXAction {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CXAction {}
impl CXAction {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CXAction").unwrap(), alloc) })
    }
}
impl PNSCopying for CXAction {}
impl PNSSecureCoding for CXAction {}
impl INSObject for CXAction {}
impl PNSObject for CXAction {}
impl std::convert::TryFrom<NSObject> for CXAction {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<CXAction, Self::Error> {
        let is_kind_of: bool = unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CXAction").unwrap()) };
        if is_kind_of {
            Ok(CXAction(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to CXAction")
        }
    }
}
impl ICXAction for CXAction {}
pub trait ICXAction: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn initWithCoder_(&self, aDecoder: NSCoder) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithCoder : aDecoder)
    }
    unsafe fn fulfill(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, fulfill)
    }
    unsafe fn fail(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, fail)
    }
    unsafe fn UUID(&self) -> NSUUID
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, UUID)
    }
    unsafe fn isComplete(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isComplete)
    }
    unsafe fn timeoutDate(&self) -> NSDate
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, timeoutDate)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CXTransaction(pub id);
impl std::ops::Deref for CXTransaction {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CXTransaction {}
impl CXTransaction {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CXTransaction").unwrap(), alloc) })
    }
}
impl PNSCopying for CXTransaction {}
impl PNSSecureCoding for CXTransaction {}
impl INSObject for CXTransaction {}
impl PNSObject for CXTransaction {}
impl std::convert::TryFrom<NSObject> for CXTransaction {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<CXTransaction, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CXTransaction").unwrap()) };
        if is_kind_of {
            Ok(CXTransaction(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to CXTransaction")
        }
    }
}
impl ICXTransaction for CXTransaction {}
pub trait ICXTransaction: Sized + std::ops::Deref {
    unsafe fn initWithActions_(&self, actions: NSArray) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithActions : actions)
    }
    unsafe fn initWithAction_(&self, action: CXAction) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithAction : action)
    }
    unsafe fn addAction_(&self, action: CXAction)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, addAction : action)
    }
    unsafe fn UUID(&self) -> NSUUID
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, UUID)
    }
    unsafe fn isComplete(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isComplete)
    }
    unsafe fn actions(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, actions)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CXCallUpdate(pub id);
impl std::ops::Deref for CXCallUpdate {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CXCallUpdate {}
impl CXCallUpdate {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CXCallUpdate").unwrap(), alloc) })
    }
}
impl PNSCopying for CXCallUpdate {}
impl INSObject for CXCallUpdate {}
impl PNSObject for CXCallUpdate {}
impl std::convert::TryFrom<NSObject> for CXCallUpdate {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<CXCallUpdate, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CXCallUpdate").unwrap()) };
        if is_kind_of {
            Ok(CXCallUpdate(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to CXCallUpdate")
        }
    }
}
impl ICXCallUpdate for CXCallUpdate {}
pub trait ICXCallUpdate: Sized + std::ops::Deref {
    unsafe fn remoteHandle(&self) -> CXHandle
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, remoteHandle)
    }
    unsafe fn setRemoteHandle_(&self, remoteHandle: CXHandle)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRemoteHandle : remoteHandle)
    }
    unsafe fn localizedCallerName(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, localizedCallerName)
    }
    unsafe fn setLocalizedCallerName_(&self, localizedCallerName: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setLocalizedCallerName : localizedCallerName)
    }
    unsafe fn supportsHolding(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, supportsHolding)
    }
    unsafe fn setSupportsHolding_(&self, supportsHolding: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSupportsHolding : supportsHolding)
    }
    unsafe fn supportsGrouping(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, supportsGrouping)
    }
    unsafe fn setSupportsGrouping_(&self, supportsGrouping: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSupportsGrouping : supportsGrouping)
    }
    unsafe fn supportsUngrouping(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, supportsUngrouping)
    }
    unsafe fn setSupportsUngrouping_(&self, supportsUngrouping: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSupportsUngrouping : supportsUngrouping)
    }
    unsafe fn supportsDTMF(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, supportsDTMF)
    }
    unsafe fn setSupportsDTMF_(&self, supportsDTMF: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSupportsDTMF : supportsDTMF)
    }
    unsafe fn hasVideo(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, hasVideo)
    }
    unsafe fn setHasVideo_(&self, hasVideo: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setHasVideo : hasVideo)
    }
}
pub type CXHandleType = NSInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CXHandle(pub id);
impl std::ops::Deref for CXHandle {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CXHandle {}
impl CXHandle {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CXHandle").unwrap(), alloc) })
    }
}
impl PNSCopying for CXHandle {}
impl PNSSecureCoding for CXHandle {}
impl INSObject for CXHandle {}
impl PNSObject for CXHandle {}
impl std::convert::TryFrom<NSObject> for CXHandle {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<CXHandle, Self::Error> {
        let is_kind_of: bool = unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CXHandle").unwrap()) };
        if is_kind_of {
            Ok(CXHandle(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to CXHandle")
        }
    }
}
impl ICXHandle for CXHandle {}
pub trait ICXHandle: Sized + std::ops::Deref {
    unsafe fn initWithType_value_(&self, type_: CXHandleType, value: NSString) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithType : type_, value : value)
    }
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn isEqualToHandle_(&self, handle: CXHandle) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, isEqualToHandle : handle)
    }
    unsafe fn type_(&self) -> CXHandleType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, type)
    }
    unsafe fn value(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, value)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CXCallAction(pub id);
impl std::ops::Deref for CXCallAction {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CXCallAction {}
impl CXCallAction {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CXCallAction").unwrap(), alloc) })
    }
}
impl ICXAction for CXCallAction {}
impl PNSCopying for CXCallAction {}
impl PNSSecureCoding for CXCallAction {}
impl From<CXCallAction> for CXAction {
    fn from(child: CXCallAction) -> CXAction {
        CXAction(child.0)
    }
}
impl std::convert::TryFrom<CXAction> for CXCallAction {
    type Error = &'static str;
    fn try_from(parent: CXAction) -> Result<CXCallAction, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CXCallAction").unwrap()) };
        if is_kind_of {
            Ok(CXCallAction(parent.0))
        } else {
            Err("This CXAction cannot be downcasted to CXCallAction")
        }
    }
}
impl INSObject for CXCallAction {}
impl PNSObject for CXCallAction {}
impl ICXCallAction for CXCallAction {}
pub trait ICXCallAction: Sized + std::ops::Deref {
    unsafe fn initWithCallUUID_(&self, callUUID: NSUUID) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithCallUUID : callUUID)
    }
    unsafe fn initWithCoder_(&self, aDecoder: NSCoder) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithCoder : aDecoder)
    }
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn callUUID(&self) -> NSUUID
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, callUUID)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CXStartCallAction(pub id);
impl std::ops::Deref for CXStartCallAction {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CXStartCallAction {}
impl CXStartCallAction {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CXStartCallAction").unwrap(), alloc) })
    }
}
impl ICXCallAction for CXStartCallAction {}
impl From<CXStartCallAction> for CXCallAction {
    fn from(child: CXStartCallAction) -> CXCallAction {
        CXCallAction(child.0)
    }
}
impl std::convert::TryFrom<CXCallAction> for CXStartCallAction {
    type Error = &'static str;
    fn try_from(parent: CXCallAction) -> Result<CXStartCallAction, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CXStartCallAction").unwrap()) };
        if is_kind_of {
            Ok(CXStartCallAction(parent.0))
        } else {
            Err("This CXCallAction cannot be downcasted to CXStartCallAction")
        }
    }
}
impl ICXAction for CXStartCallAction {}
impl PNSCopying for CXStartCallAction {}
impl PNSSecureCoding for CXStartCallAction {}
impl INSObject for CXStartCallAction {}
impl PNSObject for CXStartCallAction {}
impl ICXStartCallAction for CXStartCallAction {}
pub trait ICXStartCallAction: Sized + std::ops::Deref {
    unsafe fn initWithCallUUID_handle_(&self, callUUID: NSUUID, handle: CXHandle) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithCallUUID : callUUID, handle : handle)
    }
    unsafe fn initWithCoder_(&self, aDecoder: NSCoder) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithCoder : aDecoder)
    }
    unsafe fn initWithCallUUID_(&self, callUUID: NSUUID) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithCallUUID : callUUID)
    }
    unsafe fn fulfillWithDateStarted_(&self, dateStarted: NSDate)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, fulfillWithDateStarted : dateStarted)
    }
    unsafe fn handle(&self) -> CXHandle
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, handle)
    }
    unsafe fn setHandle_(&self, handle: CXHandle)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setHandle : handle)
    }
    unsafe fn contactIdentifier(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, contactIdentifier)
    }
    unsafe fn setContactIdentifier_(&self, contactIdentifier: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setContactIdentifier : contactIdentifier)
    }
    unsafe fn isVideo(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isVideo)
    }
    unsafe fn setVideo_(&self, video: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setVideo : video)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CXAnswerCallAction(pub id);
impl std::ops::Deref for CXAnswerCallAction {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CXAnswerCallAction {}
impl CXAnswerCallAction {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CXAnswerCallAction").unwrap(), alloc) })
    }
}
impl ICXCallAction for CXAnswerCallAction {}
impl std::convert::TryFrom<CXCallAction> for CXAnswerCallAction {
    type Error = &'static str;
    fn try_from(parent: CXCallAction) -> Result<CXAnswerCallAction, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CXAnswerCallAction").unwrap()) };
        if is_kind_of {
            Ok(CXAnswerCallAction(parent.0))
        } else {
            Err("This CXCallAction cannot be downcasted to CXAnswerCallAction")
        }
    }
}
impl ICXAction for CXAnswerCallAction {}
impl PNSCopying for CXAnswerCallAction {}
impl PNSSecureCoding for CXAnswerCallAction {}
impl INSObject for CXAnswerCallAction {}
impl PNSObject for CXAnswerCallAction {}
impl ICXAnswerCallAction for CXAnswerCallAction {}
pub trait ICXAnswerCallAction: Sized + std::ops::Deref {
    unsafe fn fulfillWithDateConnected_(&self, dateConnected: NSDate)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, fulfillWithDateConnected : dateConnected)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CXEndCallAction(pub id);
impl std::ops::Deref for CXEndCallAction {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CXEndCallAction {}
impl CXEndCallAction {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CXEndCallAction").unwrap(), alloc) })
    }
}
impl ICXCallAction for CXEndCallAction {}
impl std::convert::TryFrom<CXCallAction> for CXEndCallAction {
    type Error = &'static str;
    fn try_from(parent: CXCallAction) -> Result<CXEndCallAction, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CXEndCallAction").unwrap()) };
        if is_kind_of {
            Ok(CXEndCallAction(parent.0))
        } else {
            Err("This CXCallAction cannot be downcasted to CXEndCallAction")
        }
    }
}
impl ICXAction for CXEndCallAction {}
impl PNSCopying for CXEndCallAction {}
impl PNSSecureCoding for CXEndCallAction {}
impl INSObject for CXEndCallAction {}
impl PNSObject for CXEndCallAction {}
impl ICXEndCallAction for CXEndCallAction {}
pub trait ICXEndCallAction: Sized + std::ops::Deref {
    unsafe fn fulfillWithDateEnded_(&self, dateEnded: NSDate)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, fulfillWithDateEnded : dateEnded)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CXSetHeldCallAction(pub id);
impl std::ops::Deref for CXSetHeldCallAction {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CXSetHeldCallAction {}
impl CXSetHeldCallAction {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CXSetHeldCallAction").unwrap(), alloc) })
    }
}
impl ICXCallAction for CXSetHeldCallAction {}
impl std::convert::TryFrom<CXCallAction> for CXSetHeldCallAction {
    type Error = &'static str;
    fn try_from(parent: CXCallAction) -> Result<CXSetHeldCallAction, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CXSetHeldCallAction").unwrap()) };
        if is_kind_of {
            Ok(CXSetHeldCallAction(parent.0))
        } else {
            Err("This CXCallAction cannot be downcasted to CXSetHeldCallAction")
        }
    }
}
impl ICXAction for CXSetHeldCallAction {}
impl PNSCopying for CXSetHeldCallAction {}
impl PNSSecureCoding for CXSetHeldCallAction {}
impl INSObject for CXSetHeldCallAction {}
impl PNSObject for CXSetHeldCallAction {}
impl ICXSetHeldCallAction for CXSetHeldCallAction {}
pub trait ICXSetHeldCallAction: Sized + std::ops::Deref {
    unsafe fn initWithCallUUID_onHold_(&self, callUUID: NSUUID, onHold: BOOL) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithCallUUID : callUUID, onHold : onHold)
    }
    unsafe fn initWithCoder_(&self, aDecoder: NSCoder) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithCoder : aDecoder)
    }
    unsafe fn initWithCallUUID_(&self, callUUID: NSUUID) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithCallUUID : callUUID)
    }
    unsafe fn isOnHold(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isOnHold)
    }
    unsafe fn setOnHold_(&self, onHold: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setOnHold : onHold)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CXSetMutedCallAction(pub id);
impl std::ops::Deref for CXSetMutedCallAction {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CXSetMutedCallAction {}
impl CXSetMutedCallAction {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CXSetMutedCallAction").unwrap(), alloc) })
    }
}
impl ICXCallAction for CXSetMutedCallAction {}
impl std::convert::TryFrom<CXCallAction> for CXSetMutedCallAction {
    type Error = &'static str;
    fn try_from(parent: CXCallAction) -> Result<CXSetMutedCallAction, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CXSetMutedCallAction").unwrap()) };
        if is_kind_of {
            Ok(CXSetMutedCallAction(parent.0))
        } else {
            Err("This CXCallAction cannot be downcasted to CXSetMutedCallAction")
        }
    }
}
impl ICXAction for CXSetMutedCallAction {}
impl PNSCopying for CXSetMutedCallAction {}
impl PNSSecureCoding for CXSetMutedCallAction {}
impl INSObject for CXSetMutedCallAction {}
impl PNSObject for CXSetMutedCallAction {}
impl ICXSetMutedCallAction for CXSetMutedCallAction {}
pub trait ICXSetMutedCallAction: Sized + std::ops::Deref {
    unsafe fn initWithCallUUID_muted_(&self, callUUID: NSUUID, muted: BOOL) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithCallUUID : callUUID, muted : muted)
    }
    unsafe fn initWithCoder_(&self, aDecoder: NSCoder) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithCoder : aDecoder)
    }
    unsafe fn initWithCallUUID_(&self, callUUID: NSUUID) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithCallUUID : callUUID)
    }
    unsafe fn isMuted(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isMuted)
    }
    unsafe fn setMuted_(&self, muted: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMuted : muted)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CXSetGroupCallAction(pub id);
impl std::ops::Deref for CXSetGroupCallAction {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CXSetGroupCallAction {}
impl CXSetGroupCallAction {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CXSetGroupCallAction").unwrap(), alloc) })
    }
}
impl ICXCallAction for CXSetGroupCallAction {}
impl std::convert::TryFrom<CXCallAction> for CXSetGroupCallAction {
    type Error = &'static str;
    fn try_from(parent: CXCallAction) -> Result<CXSetGroupCallAction, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CXSetGroupCallAction").unwrap()) };
        if is_kind_of {
            Ok(CXSetGroupCallAction(parent.0))
        } else {
            Err("This CXCallAction cannot be downcasted to CXSetGroupCallAction")
        }
    }
}
impl ICXAction for CXSetGroupCallAction {}
impl PNSCopying for CXSetGroupCallAction {}
impl PNSSecureCoding for CXSetGroupCallAction {}
impl INSObject for CXSetGroupCallAction {}
impl PNSObject for CXSetGroupCallAction {}
impl ICXSetGroupCallAction for CXSetGroupCallAction {}
pub trait ICXSetGroupCallAction: Sized + std::ops::Deref {
    unsafe fn initWithCallUUID_callUUIDToGroupWith_(
        &self,
        callUUID: NSUUID,
        callUUIDToGroupWith: NSUUID,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithCallUUID : callUUID, callUUIDToGroupWith : callUUIDToGroupWith)
    }
    unsafe fn initWithCoder_(&self, aDecoder: NSCoder) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithCoder : aDecoder)
    }
    unsafe fn initWithCallUUID_(&self, callUUID: NSUUID) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithCallUUID : callUUID)
    }
    unsafe fn callUUIDToGroupWith(&self) -> NSUUID
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, callUUIDToGroupWith)
    }
    unsafe fn setCallUUIDToGroupWith_(&self, callUUIDToGroupWith: NSUUID)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCallUUIDToGroupWith : callUUIDToGroupWith)
    }
}
pub type CXPlayDTMFCallActionType = NSInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CXPlayDTMFCallAction(pub id);
impl std::ops::Deref for CXPlayDTMFCallAction {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CXPlayDTMFCallAction {}
impl CXPlayDTMFCallAction {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CXPlayDTMFCallAction").unwrap(), alloc) })
    }
}
impl ICXCallAction for CXPlayDTMFCallAction {}
impl std::convert::TryFrom<CXCallAction> for CXPlayDTMFCallAction {
    type Error = &'static str;
    fn try_from(parent: CXCallAction) -> Result<CXPlayDTMFCallAction, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CXPlayDTMFCallAction").unwrap()) };
        if is_kind_of {
            Ok(CXPlayDTMFCallAction(parent.0))
        } else {
            Err("This CXCallAction cannot be downcasted to CXPlayDTMFCallAction")
        }
    }
}
impl ICXAction for CXPlayDTMFCallAction {}
impl PNSCopying for CXPlayDTMFCallAction {}
impl PNSSecureCoding for CXPlayDTMFCallAction {}
impl INSObject for CXPlayDTMFCallAction {}
impl PNSObject for CXPlayDTMFCallAction {}
impl ICXPlayDTMFCallAction for CXPlayDTMFCallAction {}
pub trait ICXPlayDTMFCallAction: Sized + std::ops::Deref {
    unsafe fn initWithCallUUID_digits_type_(
        &self,
        callUUID: NSUUID,
        digits: NSString,
        type_: CXPlayDTMFCallActionType,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithCallUUID : callUUID, digits : digits, r#type : type_)
    }
    unsafe fn initWithCoder_(&self, aDecoder: NSCoder) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithCoder : aDecoder)
    }
    unsafe fn initWithCallUUID_(&self, callUUID: NSUUID) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithCallUUID : callUUID)
    }
    unsafe fn digits(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, digits)
    }
    unsafe fn setDigits_(&self, digits: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDigits : digits)
    }
    unsafe fn type_(&self) -> CXPlayDTMFCallActionType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, type)
    }
    unsafe fn setType_(&self, type_: CXPlayDTMFCallActionType)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setType : type_)
    }
}
pub type CXCallEndedReason = NSInteger;
pub trait PCXProviderDelegate: Sized + std::ops::Deref {
    unsafe fn providerDidReset_(&self, provider: CXProvider)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, providerDidReset : provider)
    }
    unsafe fn providerDidBegin_(&self, provider: CXProvider)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, providerDidBegin : provider)
    }
    unsafe fn provider_executeTransaction_(
        &self,
        provider: CXProvider,
        transaction: CXTransaction,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, provider : provider, executeTransaction : transaction)
    }
    unsafe fn provider_performStartCallAction_(
        &self,
        provider: CXProvider,
        action: CXStartCallAction,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, provider : provider, performStartCallAction : action)
    }
    unsafe fn provider_performAnswerCallAction_(
        &self,
        provider: CXProvider,
        action: CXAnswerCallAction,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, provider : provider, performAnswerCallAction : action)
    }
    unsafe fn provider_performEndCallAction_(&self, provider: CXProvider, action: CXEndCallAction)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, provider : provider, performEndCallAction : action)
    }
    unsafe fn provider_performSetHeldCallAction_(
        &self,
        provider: CXProvider,
        action: CXSetHeldCallAction,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, provider : provider, performSetHeldCallAction : action)
    }
    unsafe fn provider_performSetMutedCallAction_(
        &self,
        provider: CXProvider,
        action: CXSetMutedCallAction,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, provider : provider, performSetMutedCallAction : action)
    }
    unsafe fn provider_performSetGroupCallAction_(
        &self,
        provider: CXProvider,
        action: CXSetGroupCallAction,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, provider : provider, performSetGroupCallAction : action)
    }
    unsafe fn provider_performPlayDTMFCallAction_(
        &self,
        provider: CXProvider,
        action: CXPlayDTMFCallAction,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, provider : provider, performPlayDTMFCallAction : action)
    }
    unsafe fn provider_timedOutPerformingAction_(&self, provider: CXProvider, action: CXAction)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, provider : provider, timedOutPerformingAction : action)
    }
    unsafe fn provider_didActivateAudioSession_(
        &self,
        provider: CXProvider,
        audioSession: AVAudioSession,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, provider : provider, didActivateAudioSession : audioSession)
    }
    unsafe fn provider_didDeactivateAudioSession_(
        &self,
        provider: CXProvider,
        audioSession: AVAudioSession,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, provider : provider, didDeactivateAudioSession : audioSession)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CXProvider(pub id);
impl std::ops::Deref for CXProvider {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CXProvider {}
impl CXProvider {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CXProvider").unwrap(), alloc) })
    }
}
impl INSObject for CXProvider {}
impl PNSObject for CXProvider {}
impl std::convert::TryFrom<NSObject> for CXProvider {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<CXProvider, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CXProvider").unwrap()) };
        if is_kind_of {
            Ok(CXProvider(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to CXProvider")
        }
    }
}
impl ICXProvider for CXProvider {}
pub trait ICXProvider: Sized + std::ops::Deref {
    unsafe fn initWithConfiguration_(&self, configuration: CXProviderConfiguration) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithConfiguration : configuration)
    }
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn setDelegate_queue_(&self, delegate: *mut u64, queue: NSObject)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDelegate : delegate, queue : queue)
    }
    unsafe fn reportNewIncomingCallWithUUID_update_completion_(
        &self,
        UUID: NSUUID,
        update: CXCallUpdate,
        completion: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, reportNewIncomingCallWithUUID : UUID, update : update, completion : completion)
    }
    unsafe fn reportCallWithUUID_updated_(&self, UUID: NSUUID, update: CXCallUpdate)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, reportCallWithUUID : UUID, updated : update)
    }
    unsafe fn reportCallWithUUID_endedAtDate_reason_(
        &self,
        UUID: NSUUID,
        dateEnded: NSDate,
        endedReason: CXCallEndedReason,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, reportCallWithUUID : UUID, endedAtDate : dateEnded, reason : endedReason)
    }
    unsafe fn reportOutgoingCallWithUUID_startedConnectingAtDate_(
        &self,
        UUID: NSUUID,
        dateStartedConnecting: NSDate,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, reportOutgoingCallWithUUID : UUID, startedConnectingAtDate : dateStartedConnecting)
    }
    unsafe fn reportOutgoingCallWithUUID_connectedAtDate_(
        &self,
        UUID: NSUUID,
        dateConnected: NSDate,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, reportOutgoingCallWithUUID : UUID, connectedAtDate : dateConnected)
    }
    unsafe fn invalidate(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, invalidate)
    }
    unsafe fn pendingCallActionsOfClass_withCallUUID_(
        &self,
        callActionClass: Class,
        callUUID: NSUUID,
    ) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, pendingCallActionsOfClass : callActionClass, withCallUUID : callUUID)
    }
    unsafe fn configuration(&self) -> CXProviderConfiguration
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, configuration)
    }
    unsafe fn setConfiguration_(&self, configuration: CXProviderConfiguration)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setConfiguration : configuration)
    }
    unsafe fn pendingTransactions(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, pendingTransactions)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CXProvider").unwrap(), new)
    }
    unsafe fn reportNewIncomingVoIPPushPayload_completion_(
        dictionaryPayload: NSDictionary,
        completion: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CXProvider").unwrap(), reportNewIncomingVoIPPushPayload : dictionaryPayload, completion : completion)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CXProviderConfiguration(pub id);
impl std::ops::Deref for CXProviderConfiguration {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CXProviderConfiguration {}
impl CXProviderConfiguration {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CXProviderConfiguration").unwrap(), alloc) })
    }
}
impl PNSCopying for CXProviderConfiguration {}
impl INSObject for CXProviderConfiguration {}
impl PNSObject for CXProviderConfiguration {}
impl std::convert::TryFrom<NSObject> for CXProviderConfiguration {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<CXProviderConfiguration, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CXProviderConfiguration").unwrap()) };
        if is_kind_of {
            Ok(CXProviderConfiguration(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to CXProviderConfiguration")
        }
    }
}
impl ICXProviderConfiguration for CXProviderConfiguration {}
pub trait ICXProviderConfiguration: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn initWithLocalizedName_(&self, localizedName: NSString) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithLocalizedName : localizedName)
    }
    unsafe fn localizedName(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, localizedName)
    }
    unsafe fn ringtoneSound(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, ringtoneSound)
    }
    unsafe fn setRingtoneSound_(&self, ringtoneSound: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRingtoneSound : ringtoneSound)
    }
    unsafe fn iconTemplateImageData(&self) -> NSData
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, iconTemplateImageData)
    }
    unsafe fn setIconTemplateImageData_(&self, iconTemplateImageData: NSData)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setIconTemplateImageData : iconTemplateImageData)
    }
    unsafe fn maximumCallGroups(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, maximumCallGroups)
    }
    unsafe fn setMaximumCallGroups_(&self, maximumCallGroups: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMaximumCallGroups : maximumCallGroups)
    }
    unsafe fn maximumCallsPerCallGroup(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, maximumCallsPerCallGroup)
    }
    unsafe fn setMaximumCallsPerCallGroup_(&self, maximumCallsPerCallGroup: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMaximumCallsPerCallGroup : maximumCallsPerCallGroup)
    }
    unsafe fn includesCallsInRecents(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, includesCallsInRecents)
    }
    unsafe fn setIncludesCallsInRecents_(&self, includesCallsInRecents: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setIncludesCallsInRecents : includesCallsInRecents)
    }
    unsafe fn supportsVideo(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, supportsVideo)
    }
    unsafe fn setSupportsVideo_(&self, supportsVideo: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSupportsVideo : supportsVideo)
    }
    unsafe fn supportsAudioTranslation(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, supportsAudioTranslation)
    }
    unsafe fn setSupportsAudioTranslation_(&self, supportsAudioTranslation: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSupportsAudioTranslation : supportsAudioTranslation)
    }
    unsafe fn supportedHandleTypes(&self) -> NSSet
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, supportedHandleTypes)
    }
    unsafe fn setSupportedHandleTypes_(&self, supportedHandleTypes: NSSet)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSupportedHandleTypes : supportedHandleTypes)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CXCall(pub id);
impl std::ops::Deref for CXCall {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CXCall {}
impl CXCall {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CXCall").unwrap(), alloc) })
    }
}
impl INSObject for CXCall {}
impl PNSObject for CXCall {}
impl std::convert::TryFrom<NSObject> for CXCall {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<CXCall, Self::Error> {
        let is_kind_of: bool = unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CXCall").unwrap()) };
        if is_kind_of {
            Ok(CXCall(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to CXCall")
        }
    }
}
impl ICXCall for CXCall {}
pub trait ICXCall: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn isEqualToCall_(&self, call: CXCall) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, isEqualToCall : call)
    }
    unsafe fn UUID(&self) -> NSUUID
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, UUID)
    }
    unsafe fn isOutgoing(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isOutgoing)
    }
    unsafe fn isOnHold(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isOnHold)
    }
    unsafe fn hasConnected(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, hasConnected)
    }
    unsafe fn hasEnded(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, hasEnded)
    }
}
pub trait PCXCallObserverDelegate: Sized + std::ops::Deref {
    unsafe fn callObserver_callChanged_(&self, callObserver: CXCallObserver, call: CXCall)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, callObserver : callObserver, callChanged : call)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CXCallObserver(pub id);
impl std::ops::Deref for CXCallObserver {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CXCallObserver {}
impl CXCallObserver {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CXCallObserver").unwrap(), alloc) })
    }
}
impl INSObject for CXCallObserver {}
impl PNSObject for CXCallObserver {}
impl std::convert::TryFrom<NSObject> for CXCallObserver {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<CXCallObserver, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CXCallObserver").unwrap()) };
        if is_kind_of {
            Ok(CXCallObserver(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to CXCallObserver")
        }
    }
}
impl ICXCallObserver for CXCallObserver {}
pub trait ICXCallObserver: Sized + std::ops::Deref {
    unsafe fn setDelegate_queue_(&self, delegate: *mut u64, queue: NSObject)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDelegate : delegate, queue : queue)
    }
    unsafe fn calls(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, calls)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CXCallController(pub id);
impl std::ops::Deref for CXCallController {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CXCallController {}
impl CXCallController {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CXCallController").unwrap(), alloc) })
    }
}
impl INSObject for CXCallController {}
impl PNSObject for CXCallController {}
impl std::convert::TryFrom<NSObject> for CXCallController {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<CXCallController, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CXCallController").unwrap()) };
        if is_kind_of {
            Ok(CXCallController(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to CXCallController")
        }
    }
}
impl ICXCallController for CXCallController {}
pub trait ICXCallController: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn initWithQueue_(&self, queue: NSObject) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithQueue : queue)
    }
    unsafe fn requestTransaction_completion_(
        &self,
        transaction: CXTransaction,
        completion: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, requestTransaction : transaction, completion : completion)
    }
    unsafe fn requestTransactionWithActions_completion_(
        &self,
        actions: NSArray,
        completion: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, requestTransactionWithActions : actions, completion : completion)
    }
    unsafe fn requestTransactionWithAction_completion_(
        &self,
        action: CXAction,
        completion: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, requestTransactionWithAction : action, completion : completion)
    }
    unsafe fn callObserver(&self) -> CXCallObserver
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, callObserver)
    }
}
pub type CXCallDirectoryPhoneNumber = i64;
pub type CXCallDirectoryEnabledStatus = NSInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CXCallDirectoryManager(pub id);
impl std::ops::Deref for CXCallDirectoryManager {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CXCallDirectoryManager {}
impl CXCallDirectoryManager {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CXCallDirectoryManager").unwrap(), alloc) })
    }
}
impl INSObject for CXCallDirectoryManager {}
impl PNSObject for CXCallDirectoryManager {}
impl std::convert::TryFrom<NSObject> for CXCallDirectoryManager {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<CXCallDirectoryManager, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CXCallDirectoryManager").unwrap()) };
        if is_kind_of {
            Ok(CXCallDirectoryManager(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to CXCallDirectoryManager")
        }
    }
}
impl ICXCallDirectoryManager for CXCallDirectoryManager {}
pub trait ICXCallDirectoryManager: Sized + std::ops::Deref {
    unsafe fn reloadExtensionWithIdentifier_completionHandler_(
        &self,
        identifier: NSString,
        completion: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, reloadExtensionWithIdentifier : identifier, completionHandler : completion)
    }
    unsafe fn getEnabledStatusForExtensionWithIdentifier_completionHandler_(
        &self,
        identifier: NSString,
        completion: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, getEnabledStatusForExtensionWithIdentifier : identifier, completionHandler : completion)
    }
    unsafe fn openSettingsWithCompletionHandler_(&self, completion: *mut ::std::os::raw::c_void)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, openSettingsWithCompletionHandler : completion)
    }
    unsafe fn sharedInstance() -> CXCallDirectoryManager
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CXCallDirectoryManager").unwrap(), sharedInstance)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CXCallDirectoryProvider(pub id);
impl std::ops::Deref for CXCallDirectoryProvider {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CXCallDirectoryProvider {}
impl CXCallDirectoryProvider {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CXCallDirectoryProvider").unwrap(), alloc) })
    }
}
impl PNSExtensionRequestHandling for CXCallDirectoryProvider {}
impl INSObject for CXCallDirectoryProvider {}
impl PNSObject for CXCallDirectoryProvider {}
impl std::convert::TryFrom<NSObject> for CXCallDirectoryProvider {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<CXCallDirectoryProvider, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CXCallDirectoryProvider").unwrap()) };
        if is_kind_of {
            Ok(CXCallDirectoryProvider(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to CXCallDirectoryProvider")
        }
    }
}
impl ICXCallDirectoryProvider for CXCallDirectoryProvider {}
pub trait ICXCallDirectoryProvider: Sized + std::ops::Deref {
    unsafe fn beginRequestWithExtensionContext_(&self, context: CXCallDirectoryExtensionContext)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, beginRequestWithExtensionContext : context)
    }
}
pub trait PCXCallDirectoryExtensionContextDelegate: Sized + std::ops::Deref {
    unsafe fn requestFailedForExtensionContext_withError_(
        &self,
        extensionContext: CXCallDirectoryExtensionContext,
        error: NSError,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, requestFailedForExtensionContext : extensionContext, withError : error)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CXCallDirectoryExtensionContext(pub id);
impl std::ops::Deref for CXCallDirectoryExtensionContext {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CXCallDirectoryExtensionContext {}
impl CXCallDirectoryExtensionContext {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CXCallDirectoryExtensionContext").unwrap(), alloc) })
    }
}
impl INSExtensionContext for CXCallDirectoryExtensionContext {}
impl std::convert::TryFrom<NSExtensionContext> for CXCallDirectoryExtensionContext {
    type Error = &'static str;
    fn try_from(
        parent: NSExtensionContext,
    ) -> Result<CXCallDirectoryExtensionContext, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CXCallDirectoryExtensionContext").unwrap())
        };
        if is_kind_of {
            Ok(CXCallDirectoryExtensionContext(parent.0))
        } else {
            Err("This NSExtensionContext cannot be downcasted to CXCallDirectoryExtensionContext")
        }
    }
}
impl INSObject for CXCallDirectoryExtensionContext {}
impl PNSObject for CXCallDirectoryExtensionContext {}
impl ICXCallDirectoryExtensionContext for CXCallDirectoryExtensionContext {}
pub trait ICXCallDirectoryExtensionContext: Sized + std::ops::Deref {
    unsafe fn addBlockingEntryWithNextSequentialPhoneNumber_(
        &self,
        phoneNumber: CXCallDirectoryPhoneNumber,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, addBlockingEntryWithNextSequentialPhoneNumber : phoneNumber)
    }
    unsafe fn removeBlockingEntryWithPhoneNumber_(&self, phoneNumber: CXCallDirectoryPhoneNumber)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, removeBlockingEntryWithPhoneNumber : phoneNumber)
    }
    unsafe fn removeAllBlockingEntries(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, removeAllBlockingEntries)
    }
    unsafe fn addIdentificationEntryWithNextSequentialPhoneNumber_label_(
        &self,
        phoneNumber: CXCallDirectoryPhoneNumber,
        label: NSString,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, addIdentificationEntryWithNextSequentialPhoneNumber : phoneNumber, label : label)
    }
    unsafe fn removeIdentificationEntryWithPhoneNumber_(
        &self,
        phoneNumber: CXCallDirectoryPhoneNumber,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, removeIdentificationEntryWithPhoneNumber : phoneNumber)
    }
    unsafe fn removeAllIdentificationEntries(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, removeAllIdentificationEntries)
    }
    unsafe fn completeRequestWithCompletionHandler_(&self, completion: *mut ::std::os::raw::c_void)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, completeRequestWithCompletionHandler : completion)
    }
    unsafe fn completeRequestReturningItems_completionHandler_(
        &self,
        items: NSArray,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, completeRequestReturningItems : items, completionHandler : completionHandler)
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
    unsafe fn isIncremental(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isIncremental)
    }
}
unsafe extern "C" {
    pub static CXErrorDomain: NSErrorDomain;
}
unsafe extern "C" {
    pub static CXErrorDomainIncomingCall: NSErrorDomain;
}
unsafe extern "C" {
    pub static CXErrorDomainRequestTransaction: NSErrorDomain;
}
unsafe extern "C" {
    pub static CXErrorDomainCallDirectoryManager: NSErrorDomain;
}
unsafe extern "C" {
    pub static CXErrorDomainNotificationServiceExtension: NSErrorDomain;
}

unsafe impl objc2::encode::RefEncode for CXAction {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CXAction {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CXTransaction {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CXTransaction {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CXCallUpdate {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CXCallUpdate {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CXHandle {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CXHandle {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CXCallAction {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CXCallAction {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CXStartCallAction {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CXStartCallAction {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CXAnswerCallAction {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CXAnswerCallAction {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CXEndCallAction {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CXEndCallAction {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CXSetHeldCallAction {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CXSetHeldCallAction {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CXSetMutedCallAction {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CXSetMutedCallAction {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CXSetGroupCallAction {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CXSetGroupCallAction {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CXPlayDTMFCallAction {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CXPlayDTMFCallAction {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CXProvider {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CXProvider {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CXProviderConfiguration {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CXProviderConfiguration {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CXCall {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CXCall {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CXCallObserver {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CXCallObserver {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CXCallController {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CXCallController {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CXCallDirectoryManager {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CXCallDirectoryManager {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CXCallDirectoryProvider {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CXCallDirectoryProvider {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CXCallDirectoryExtensionContext {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CXCallDirectoryExtensionContext {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
