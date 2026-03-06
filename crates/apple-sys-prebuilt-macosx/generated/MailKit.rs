#[allow(unused_imports)]
use crate::objc::*;
#[allow(unused_imports)]
use crate::CoreFoundation::*;
#[allow(unused_imports)]
use crate::Foundation::*;

#[allow(unused_imports)]
use objc2::msg_send;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MEEmailAddress(pub id);
impl std::ops::Deref for MEEmailAddress {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MEEmailAddress {}
impl MEEmailAddress {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MEEmailAddress").unwrap(), alloc) })
    }
}
impl PNSSecureCoding for MEEmailAddress {}
impl PNSCopying for MEEmailAddress {}
impl INSObject for MEEmailAddress {}
impl PNSObject for MEEmailAddress {}
impl std::convert::TryFrom<NSObject> for MEEmailAddress {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<MEEmailAddress, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MEEmailAddress").unwrap()) };
        if is_kind_of {
            Ok(MEEmailAddress(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to MEEmailAddress")
        }
    }
}
impl IMEEmailAddress for MEEmailAddress {}
pub trait IMEEmailAddress: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn initWithRawString_(&self, rawString: NSString) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithRawString : rawString)
    }
    unsafe fn rawString(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, rawString)
    }
    unsafe fn addressString(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, addressString)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MEEmailAddress").unwrap(), new)
    }
}
pub trait PMEExtension: Sized + std::ops::Deref {
    unsafe fn handlerForComposeSession_(&self, session: MEComposeSession) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, handlerForComposeSession : session)
    }
    unsafe fn handlerForMessageActions(&self) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, handlerForMessageActions)
    }
    unsafe fn handlerForContentBlocker(&self) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, handlerForContentBlocker)
    }
    unsafe fn handlerForMessageSecurity(&self) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, handlerForMessageSecurity)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MEExtensionViewController(pub id);
impl std::ops::Deref for MEExtensionViewController {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MEExtensionViewController {}
impl MEExtensionViewController {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MEExtensionViewController").unwrap(), alloc) })
    }
}
impl PNSCoding for MEExtensionViewController {}
impl INSObject for MEExtensionViewController {}
impl PNSObject for MEExtensionViewController {}
impl std::convert::TryFrom<NSObject> for MEExtensionViewController {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<MEExtensionViewController, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MEExtensionViewController").unwrap()) };
        if is_kind_of {
            Ok(MEExtensionViewController(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to MEExtensionViewController")
        }
    }
}
impl IMEExtensionViewController for MEExtensionViewController {}
pub trait IMEExtensionViewController: Sized + std::ops::Deref {}
pub type MEMessageState = NSInteger;
pub type MEMessageEncryptionState = NSInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MEMessage(pub id);
impl std::ops::Deref for MEMessage {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MEMessage {}
impl MEMessage {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MEMessage").unwrap(), alloc) })
    }
}
impl PNSSecureCoding for MEMessage {}
impl INSObject for MEMessage {}
impl PNSObject for MEMessage {}
impl std::convert::TryFrom<NSObject> for MEMessage {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<MEMessage, Self::Error> {
        let is_kind_of: bool = unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MEMessage").unwrap()) };
        if is_kind_of {
            Ok(MEMessage(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to MEMessage")
        }
    }
}
impl IMEMessage for MEMessage {}
pub trait IMEMessage: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn state(&self) -> MEMessageState
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, state)
    }
    unsafe fn encryptionState(&self) -> MEMessageEncryptionState
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, encryptionState)
    }
    unsafe fn subject(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, subject)
    }
    unsafe fn fromAddress(&self) -> MEEmailAddress
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, fromAddress)
    }
    unsafe fn toAddresses(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, toAddresses)
    }
    unsafe fn ccAddresses(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, ccAddresses)
    }
    unsafe fn bccAddresses(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, bccAddresses)
    }
    unsafe fn replyToAddresses(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, replyToAddresses)
    }
    unsafe fn allRecipientAddresses(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, allRecipientAddresses)
    }
    unsafe fn dateSent(&self) -> NSDate
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, dateSent)
    }
    unsafe fn dateReceived(&self) -> NSDate
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, dateReceived)
    }
    unsafe fn headers(&self) -> NSDictionary
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, headers)
    }
    unsafe fn rawData(&self) -> NSData
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, rawData)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MEMessage").unwrap(), new)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MEMessageActionDecision(pub id);
impl std::ops::Deref for MEMessageActionDecision {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MEMessageActionDecision {}
impl MEMessageActionDecision {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MEMessageActionDecision").unwrap(), alloc) })
    }
}
impl PNSSecureCoding for MEMessageActionDecision {}
impl INSObject for MEMessageActionDecision {}
impl PNSObject for MEMessageActionDecision {}
impl std::convert::TryFrom<NSObject> for MEMessageActionDecision {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<MEMessageActionDecision, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MEMessageActionDecision").unwrap()) };
        if is_kind_of {
            Ok(MEMessageActionDecision(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to MEMessageActionDecision")
        }
    }
}
impl IMEMessageActionDecision for MEMessageActionDecision {}
pub trait IMEMessageActionDecision: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn decisionApplyingAction_(action: MEMessageAction) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MEMessageActionDecision").unwrap(), decisionApplyingAction : action)
    }
    unsafe fn decisionApplyingActions_(actions: NSArray) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MEMessageActionDecision").unwrap(), decisionApplyingActions : actions)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MEMessageActionDecision").unwrap(), new)
    }
    unsafe fn invokeAgainWithBody() -> MEMessageActionDecision
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MEMessageActionDecision").unwrap(), invokeAgainWithBody)
    }
}
pub type MEMessageActionMessageColor = NSInteger;
pub type MEMessageActionFlag = NSInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MEMessageAction(pub id);
impl std::ops::Deref for MEMessageAction {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MEMessageAction {}
impl MEMessageAction {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MEMessageAction").unwrap(), alloc) })
    }
}
impl PNSSecureCoding for MEMessageAction {}
impl INSObject for MEMessageAction {}
impl PNSObject for MEMessageAction {}
impl std::convert::TryFrom<NSObject> for MEMessageAction {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<MEMessageAction, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MEMessageAction").unwrap()) };
        if is_kind_of {
            Ok(MEMessageAction(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to MEMessageAction")
        }
    }
}
impl IMEMessageAction for MEMessageAction {}
pub trait IMEMessageAction: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn flagActionWithFlag_(flag: MEMessageActionFlag) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MEMessageAction").unwrap(), flagActionWithFlag : flag)
    }
    unsafe fn setBackgroundColorActionWithColor_(color: MEMessageActionMessageColor) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MEMessageAction").unwrap(), setBackgroundColorActionWithColor : color)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MEMessageAction").unwrap(), new)
    }
    unsafe fn moveToTrashAction() -> MEMessageAction
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MEMessageAction").unwrap(), moveToTrashAction)
    }
    unsafe fn moveToArchiveAction() -> MEMessageAction
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MEMessageAction").unwrap(), moveToArchiveAction)
    }
    unsafe fn moveToJunkAction() -> MEMessageAction
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MEMessageAction").unwrap(), moveToJunkAction)
    }
    unsafe fn markAsReadAction() -> MEMessageAction
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MEMessageAction").unwrap(), markAsReadAction)
    }
    unsafe fn markAsUnreadAction() -> MEMessageAction
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MEMessageAction").unwrap(), markAsUnreadAction)
    }
}
pub trait PMEMessageActionHandler: Sized + std::ops::Deref {
    unsafe fn decideActionForMessage_completionHandler_(
        &self,
        message: MEMessage,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, decideActionForMessage : message, completionHandler : completionHandler)
    }
    unsafe fn requiredHeaders(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, requiredHeaders)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MEAddressAnnotation(pub id);
impl std::ops::Deref for MEAddressAnnotation {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MEAddressAnnotation {}
impl MEAddressAnnotation {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MEAddressAnnotation").unwrap(), alloc) })
    }
}
impl PNSSecureCoding for MEAddressAnnotation {}
impl INSObject for MEAddressAnnotation {}
impl PNSObject for MEAddressAnnotation {}
impl std::convert::TryFrom<NSObject> for MEAddressAnnotation {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<MEAddressAnnotation, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MEAddressAnnotation").unwrap()) };
        if is_kind_of {
            Ok(MEAddressAnnotation(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to MEAddressAnnotation")
        }
    }
}
impl IMEAddressAnnotation for MEAddressAnnotation {}
pub trait IMEAddressAnnotation: Sized + std::ops::Deref {
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
        msg_send!(&*objc2::runtime::AnyClass::get(c"MEAddressAnnotation").unwrap(), new)
    }
    unsafe fn errorWithLocalizedDescription_(localizedDescription: NSString) -> MEAddressAnnotation
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MEAddressAnnotation").unwrap(), errorWithLocalizedDescription : localizedDescription)
    }
    unsafe fn warningWithLocalizedDescription_(
        localizedDescription: NSString,
    ) -> MEAddressAnnotation
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MEAddressAnnotation").unwrap(), warningWithLocalizedDescription : localizedDescription)
    }
    unsafe fn successWithLocalizedDescription_(
        localizedDescription: NSString,
    ) -> MEAddressAnnotation
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MEAddressAnnotation").unwrap(), successWithLocalizedDescription : localizedDescription)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MEComposeSession(pub id);
impl std::ops::Deref for MEComposeSession {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MEComposeSession {}
impl MEComposeSession {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MEComposeSession").unwrap(), alloc) })
    }
}
impl PNSSecureCoding for MEComposeSession {}
impl INSObject for MEComposeSession {}
impl PNSObject for MEComposeSession {}
impl std::convert::TryFrom<NSObject> for MEComposeSession {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<MEComposeSession, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MEComposeSession").unwrap()) };
        if is_kind_of {
            Ok(MEComposeSession(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to MEComposeSession")
        }
    }
}
impl IMEComposeSession for MEComposeSession {}
pub trait IMEComposeSession: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn reloadSession(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, reloadSession)
    }
    unsafe fn sessionID(&self) -> NSUUID
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, sessionID)
    }
    unsafe fn mailMessage(&self) -> MEMessage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, mailMessage)
    }
    unsafe fn composeContext(&self) -> MEComposeContext
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, composeContext)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MEComposeSession").unwrap(), new)
    }
}
pub type MEComposeSessionErrorCode = NSInteger;
pub trait PMEComposeSessionHandler: Sized + std::ops::Deref {
    unsafe fn mailComposeSessionDidBegin_(&self, session: MEComposeSession)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, mailComposeSessionDidBegin : session)
    }
    unsafe fn mailComposeSessionDidEnd_(&self, session: MEComposeSession)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, mailComposeSessionDidEnd : session)
    }
    unsafe fn viewControllerForSession_(
        &self,
        session: MEComposeSession,
    ) -> MEExtensionViewController
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, viewControllerForSession : session)
    }
    unsafe fn session_annotateAddressesWithCompletionHandler_(
        &self,
        session: MEComposeSession,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, session : session, annotateAddressesWithCompletionHandler : completionHandler)
    }
    unsafe fn session_canSendMessageWithCompletionHandler_(
        &self,
        session: MEComposeSession,
        completion: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, session : session, canSendMessageWithCompletionHandler : completion)
    }
    unsafe fn additionalHeadersForSession_(&self, session: MEComposeSession) -> NSDictionary
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, additionalHeadersForSession : session)
    }
}
pub type MEComposeUserAction = NSInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MEComposeContext(pub id);
impl std::ops::Deref for MEComposeContext {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MEComposeContext {}
impl MEComposeContext {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MEComposeContext").unwrap(), alloc) })
    }
}
impl INSObject for MEComposeContext {}
impl PNSObject for MEComposeContext {}
impl std::convert::TryFrom<NSObject> for MEComposeContext {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<MEComposeContext, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MEComposeContext").unwrap()) };
        if is_kind_of {
            Ok(MEComposeContext(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to MEComposeContext")
        }
    }
}
impl IMEComposeContext for MEComposeContext {}
pub trait IMEComposeContext: Sized + std::ops::Deref {
    unsafe fn contextID(&self) -> NSUUID
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, contextID)
    }
    unsafe fn originalMessage(&self) -> MEMessage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, originalMessage)
    }
    unsafe fn action(&self) -> MEComposeUserAction
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, action)
    }
    unsafe fn isEncrypted(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isEncrypted)
    }
    unsafe fn shouldEncrypt(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, shouldEncrypt)
    }
    unsafe fn isSigned(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isSigned)
    }
    unsafe fn shouldSign(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, shouldSign)
    }
}
pub trait PMEContentBlocker: Sized + std::ops::Deref {
    unsafe fn contentRulesJSON(&self) -> NSData
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, contentRulesJSON)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MEExtensionManager(pub id);
impl std::ops::Deref for MEExtensionManager {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MEExtensionManager {}
impl MEExtensionManager {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MEExtensionManager").unwrap(), alloc) })
    }
}
impl INSObject for MEExtensionManager {}
impl PNSObject for MEExtensionManager {}
impl std::convert::TryFrom<NSObject> for MEExtensionManager {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<MEExtensionManager, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MEExtensionManager").unwrap()) };
        if is_kind_of {
            Ok(MEExtensionManager(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to MEExtensionManager")
        }
    }
}
impl IMEExtensionManager for MEExtensionManager {}
pub trait IMEExtensionManager: Sized + std::ops::Deref {
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
        msg_send!(&*objc2::runtime::AnyClass::get(c"MEExtensionManager").unwrap(), new)
    }
    unsafe fn reloadContentBlockerWithIdentifier_completionHandler_(
        identifier: NSString,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MEExtensionManager").unwrap(), reloadContentBlockerWithIdentifier : identifier, completionHandler : completionHandler)
    }
    unsafe fn reloadVisibleMessagesWithCompletionHandler_(
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MEExtensionManager").unwrap(), reloadVisibleMessagesWithCompletionHandler : completionHandler)
    }
}
pub trait PMEMessageEncoder: Sized + std::ops::Deref {
    unsafe fn getEncodingStatusForMessage_composeContext_completionHandler_(
        &self,
        message: MEMessage,
        composeContext: MEComposeContext,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, getEncodingStatusForMessage : message, composeContext : composeContext, completionHandler : completionHandler)
    }
    unsafe fn encodeMessage_composeContext_completionHandler_(
        &self,
        message: MEMessage,
        composeContext: MEComposeContext,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, encodeMessage : message, composeContext : composeContext, completionHandler : completionHandler)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MEEncodedOutgoingMessage(pub id);
impl std::ops::Deref for MEEncodedOutgoingMessage {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MEEncodedOutgoingMessage {}
impl MEEncodedOutgoingMessage {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MEEncodedOutgoingMessage").unwrap(), alloc) })
    }
}
impl PNSSecureCoding for MEEncodedOutgoingMessage {}
impl INSObject for MEEncodedOutgoingMessage {}
impl PNSObject for MEEncodedOutgoingMessage {}
impl std::convert::TryFrom<NSObject> for MEEncodedOutgoingMessage {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<MEEncodedOutgoingMessage, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MEEncodedOutgoingMessage").unwrap()) };
        if is_kind_of {
            Ok(MEEncodedOutgoingMessage(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to MEEncodedOutgoingMessage")
        }
    }
}
impl IMEEncodedOutgoingMessage for MEEncodedOutgoingMessage {}
pub trait IMEEncodedOutgoingMessage: Sized + std::ops::Deref {
    unsafe fn initWithRawData_isSigned_isEncrypted_(
        &self,
        rawData: NSData,
        isSigned: BOOL,
        isEncrypted: BOOL,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithRawData : rawData, isSigned : isSigned, isEncrypted : isEncrypted)
    }
    unsafe fn rawData(&self) -> NSData
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, rawData)
    }
    unsafe fn isSigned(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isSigned)
    }
    unsafe fn isEncrypted(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isEncrypted)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MEDecodedMessage(pub id);
impl std::ops::Deref for MEDecodedMessage {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MEDecodedMessage {}
impl MEDecodedMessage {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MEDecodedMessage").unwrap(), alloc) })
    }
}
impl PNSSecureCoding for MEDecodedMessage {}
impl INSObject for MEDecodedMessage {}
impl PNSObject for MEDecodedMessage {}
impl std::convert::TryFrom<NSObject> for MEDecodedMessage {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<MEDecodedMessage, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MEDecodedMessage").unwrap()) };
        if is_kind_of {
            Ok(MEDecodedMessage(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to MEDecodedMessage")
        }
    }
}
impl IMEDecodedMessage for MEDecodedMessage {}
pub trait IMEDecodedMessage: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn initWithData_securityInformation_context_(
        &self,
        rawData: NSData,
        securityInformation: MEMessageSecurityInformation,
        context: NSData,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithData : rawData, securityInformation : securityInformation, context : context)
    }
    unsafe fn initWithData_securityInformation_context_banner_(
        &self,
        rawData: NSData,
        securityInformation: MEMessageSecurityInformation,
        context: NSData,
        banner: MEDecodedMessageBanner,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithData : rawData, securityInformation : securityInformation, context : context, banner : banner)
    }
    unsafe fn rawData(&self) -> NSData
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, rawData)
    }
    unsafe fn securityInformation(&self) -> MEMessageSecurityInformation
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, securityInformation)
    }
    unsafe fn context(&self) -> NSData
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, context)
    }
    unsafe fn banner(&self) -> MEDecodedMessageBanner
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, banner)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MEDecodedMessage").unwrap(), new)
    }
}
pub trait PMEMessageDecoder: Sized + std::ops::Deref {
    unsafe fn decodedMessageForMessageData_(&self, data: NSData) -> MEDecodedMessage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, decodedMessageForMessageData : data)
    }
}
pub type MEMessageSecurityErrorCode = NSInteger;
pub trait PMEMessageSecurityHandler: Sized + std::ops::Deref {
    unsafe fn extensionViewControllerForMessageSigners_(
        &self,
        messageSigners: NSArray,
    ) -> MEExtensionViewController
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, extensionViewControllerForMessageSigners : messageSigners)
    }
    unsafe fn extensionViewControllerForMessageContext_(
        &self,
        context: NSData,
    ) -> MEExtensionViewController
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, extensionViewControllerForMessageContext : context)
    }
    unsafe fn primaryActionClickedForMessageContext_completionHandler_(
        &self,
        context: NSData,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, primaryActionClickedForMessageContext : context, completionHandler : completionHandler)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MEMessageEncodingResult(pub id);
impl std::ops::Deref for MEMessageEncodingResult {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MEMessageEncodingResult {}
impl MEMessageEncodingResult {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MEMessageEncodingResult").unwrap(), alloc) })
    }
}
impl PNSSecureCoding for MEMessageEncodingResult {}
impl INSObject for MEMessageEncodingResult {}
impl PNSObject for MEMessageEncodingResult {}
impl std::convert::TryFrom<NSObject> for MEMessageEncodingResult {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<MEMessageEncodingResult, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MEMessageEncodingResult").unwrap()) };
        if is_kind_of {
            Ok(MEMessageEncodingResult(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to MEMessageEncodingResult")
        }
    }
}
impl IMEMessageEncodingResult for MEMessageEncodingResult {}
pub trait IMEMessageEncodingResult: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn initWithEncodedMessage_signingError_encryptionError_(
        &self,
        encodedMessage: MEEncodedOutgoingMessage,
        signingError: NSError,
        encryptionError: NSError,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithEncodedMessage : encodedMessage, signingError : signingError, encryptionError : encryptionError)
    }
    unsafe fn encodedMessage(&self) -> MEEncodedOutgoingMessage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, encodedMessage)
    }
    unsafe fn signingError(&self) -> NSError
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, signingError)
    }
    unsafe fn encryptionError(&self) -> NSError
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, encryptionError)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MEMessageEncodingResult").unwrap(), new)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MEOutgoingMessageEncodingStatus(pub id);
impl std::ops::Deref for MEOutgoingMessageEncodingStatus {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MEOutgoingMessageEncodingStatus {}
impl MEOutgoingMessageEncodingStatus {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MEOutgoingMessageEncodingStatus").unwrap(), alloc) })
    }
}
impl PNSSecureCoding for MEOutgoingMessageEncodingStatus {}
impl INSObject for MEOutgoingMessageEncodingStatus {}
impl PNSObject for MEOutgoingMessageEncodingStatus {}
impl std::convert::TryFrom<NSObject> for MEOutgoingMessageEncodingStatus {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<MEOutgoingMessageEncodingStatus, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MEOutgoingMessageEncodingStatus").unwrap())
        };
        if is_kind_of {
            Ok(MEOutgoingMessageEncodingStatus(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to MEOutgoingMessageEncodingStatus")
        }
    }
}
impl IMEOutgoingMessageEncodingStatus for MEOutgoingMessageEncodingStatus {}
pub trait IMEOutgoingMessageEncodingStatus: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn initWithCanSign_canEncrypt_securityError_addressesFailingEncryption_(
        &self,
        canSign: BOOL,
        canEncrypt: BOOL,
        securityError: NSError,
        addressesFailingEncryption: NSArray,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithCanSign : canSign, canEncrypt : canEncrypt, securityError : securityError, addressesFailingEncryption : addressesFailingEncryption)
    }
    unsafe fn canSign(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, canSign)
    }
    unsafe fn canEncrypt(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, canEncrypt)
    }
    unsafe fn securityError(&self) -> NSError
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, securityError)
    }
    unsafe fn addressesFailingEncryption(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, addressesFailingEncryption)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MEOutgoingMessageEncodingStatus").unwrap(), new)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MEMessageSecurityInformation(pub id);
impl std::ops::Deref for MEMessageSecurityInformation {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MEMessageSecurityInformation {}
impl MEMessageSecurityInformation {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MEMessageSecurityInformation").unwrap(), alloc) })
    }
}
impl PNSSecureCoding for MEMessageSecurityInformation {}
impl INSObject for MEMessageSecurityInformation {}
impl PNSObject for MEMessageSecurityInformation {}
impl std::convert::TryFrom<NSObject> for MEMessageSecurityInformation {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<MEMessageSecurityInformation, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MEMessageSecurityInformation").unwrap()) };
        if is_kind_of {
            Ok(MEMessageSecurityInformation(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to MEMessageSecurityInformation")
        }
    }
}
impl IMEMessageSecurityInformation for MEMessageSecurityInformation {}
pub trait IMEMessageSecurityInformation: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn initWithSigners_isEncrypted_signingError_encryptionError_(
        &self,
        signers: NSArray,
        isEncrypted: BOOL,
        signingError: NSError,
        encryptionError: NSError,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithSigners : signers, isEncrypted : isEncrypted, signingError : signingError, encryptionError : encryptionError)
    }
    unsafe fn initWithSigners_isEncrypted_signingError_encryptionError_shouldBlockRemoteContent_localizedRemoteContentBlockingReason_(
        &self,
        signers: NSArray,
        isEncrypted: BOOL,
        signingError: NSError,
        encryptionError: NSError,
        shouldBlockRemoteContent: BOOL,
        localizedRemoteContentBlockingReason: NSString,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithSigners : signers, isEncrypted : isEncrypted, signingError : signingError, encryptionError : encryptionError, shouldBlockRemoteContent : shouldBlockRemoteContent, localizedRemoteContentBlockingReason : localizedRemoteContentBlockingReason)
    }
    unsafe fn signers(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, signers)
    }
    unsafe fn isEncrypted(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isEncrypted)
    }
    unsafe fn signingError(&self) -> NSError
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, signingError)
    }
    unsafe fn encryptionError(&self) -> NSError
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, encryptionError)
    }
    unsafe fn shouldBlockRemoteContent(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, shouldBlockRemoteContent)
    }
    unsafe fn localizedRemoteContentBlockingReason(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, localizedRemoteContentBlockingReason)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MEMessageSecurityInformation").unwrap(), new)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MEMessageSigner(pub id);
impl std::ops::Deref for MEMessageSigner {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MEMessageSigner {}
impl MEMessageSigner {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MEMessageSigner").unwrap(), alloc) })
    }
}
impl PNSSecureCoding for MEMessageSigner {}
impl INSObject for MEMessageSigner {}
impl PNSObject for MEMessageSigner {}
impl std::convert::TryFrom<NSObject> for MEMessageSigner {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<MEMessageSigner, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MEMessageSigner").unwrap()) };
        if is_kind_of {
            Ok(MEMessageSigner(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to MEMessageSigner")
        }
    }
}
impl IMEMessageSigner for MEMessageSigner {}
pub trait IMEMessageSigner: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn initWithEmailAddresses_signatureLabel_context_(
        &self,
        emailAddresses: NSArray,
        label: NSString,
        context: NSData,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithEmailAddresses : emailAddresses, signatureLabel : label, context : context)
    }
    unsafe fn emailAddresses(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, emailAddresses)
    }
    unsafe fn label(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, label)
    }
    unsafe fn context(&self) -> NSData
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, context)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MEMessageSigner").unwrap(), new)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MEDecodedMessageBanner(pub id);
impl std::ops::Deref for MEDecodedMessageBanner {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MEDecodedMessageBanner {}
impl MEDecodedMessageBanner {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MEDecodedMessageBanner").unwrap(), alloc) })
    }
}
impl PNSSecureCoding for MEDecodedMessageBanner {}
impl PNSCopying for MEDecodedMessageBanner {}
impl INSObject for MEDecodedMessageBanner {}
impl PNSObject for MEDecodedMessageBanner {}
impl std::convert::TryFrom<NSObject> for MEDecodedMessageBanner {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<MEDecodedMessageBanner, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MEDecodedMessageBanner").unwrap()) };
        if is_kind_of {
            Ok(MEDecodedMessageBanner(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to MEDecodedMessageBanner")
        }
    }
}
impl IMEDecodedMessageBanner for MEDecodedMessageBanner {}
pub trait IMEDecodedMessageBanner: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn initWithTitle_primaryActionTitle_dismissable_(
        &self,
        title: NSString,
        primaryActionTitle: NSString,
        dismissable: BOOL,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithTitle : title, primaryActionTitle : primaryActionTitle, dismissable : dismissable)
    }
    unsafe fn title(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, title)
    }
    unsafe fn primaryActionTitle(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, primaryActionTitle)
    }
    unsafe fn isDismissable(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isDismissable)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MEDecodedMessageBanner").unwrap(), new)
    }
}
pub trait PNSEditor: Sized + std::ops::Deref {
    unsafe fn discardEditing(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, discardEditing)
    }
    unsafe fn commitEditing(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, commitEditing)
    }
    unsafe fn commitEditingWithDelegate_didCommitSelector_contextInfo_(
        &self,
        delegate: id,
        didCommitSelector: objc2::runtime::Sel,
        contextInfo: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, commitEditingWithDelegate : delegate, didCommitSelector : didCommitSelector, contextInfo : contextInfo)
    }
    unsafe fn commitEditingAndReturnError_(&self, error: *mut NSError) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, commitEditingAndReturnError : error)
    }
}
impl PNSEditor for MEExtensionViewController {}
unsafe extern "C" {
    pub static MEComposeSessionErrorDomain: NSErrorDomain;
}
unsafe extern "C" {
    pub static MEMessageSecurityErrorDomain: NSErrorDomain;
}

unsafe impl objc2::encode::RefEncode for MEEmailAddress {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MEEmailAddress {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MEExtensionViewController {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MEExtensionViewController {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MEMessage {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MEMessage {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MEMessageActionDecision {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MEMessageActionDecision {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MEMessageAction {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MEMessageAction {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MEAddressAnnotation {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MEAddressAnnotation {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MEComposeSession {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MEComposeSession {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MEComposeContext {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MEComposeContext {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MEExtensionManager {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MEExtensionManager {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MEEncodedOutgoingMessage {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MEEncodedOutgoingMessage {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MEDecodedMessage {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MEDecodedMessage {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MEMessageEncodingResult {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MEMessageEncodingResult {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MEOutgoingMessageEncodingStatus {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MEOutgoingMessageEncodingStatus {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MEMessageSecurityInformation {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MEMessageSecurityInformation {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MEMessageSigner {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MEMessageSigner {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MEDecodedMessageBanner {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MEDecodedMessageBanner {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
