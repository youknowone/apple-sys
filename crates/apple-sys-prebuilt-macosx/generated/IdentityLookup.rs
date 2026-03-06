#[allow(unused_imports)]
use crate::objc::*;
#[allow(unused_imports)]
use crate::CoreFoundation::*;
#[allow(unused_imports)]
use crate::Foundation::*;

#[allow(unused_imports)]
use objc2::msg_send;
pub type ILMessageFilterAction = NSInteger;
pub type ILMessageFilterSubAction = NSInteger;
pub type ILMessageFilterError = NSInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct ILNetworkResponse(pub id);
impl std::ops::Deref for ILNetworkResponse {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for ILNetworkResponse {}
impl ILNetworkResponse {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"ILNetworkResponse").unwrap(), alloc) })
    }
}
impl PNSSecureCoding for ILNetworkResponse {}
impl INSObject for ILNetworkResponse {}
impl PNSObject for ILNetworkResponse {}
impl std::convert::TryFrom<NSObject> for ILNetworkResponse {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<ILNetworkResponse, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"ILNetworkResponse").unwrap()) };
        if is_kind_of {
            Ok(ILNetworkResponse(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to ILNetworkResponse")
        }
    }
}
impl IILNetworkResponse for ILNetworkResponse {}
pub trait IILNetworkResponse: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn urlResponse(&self) -> NSHTTPURLResponse
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, urlResponse)
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
pub struct ILMessageFilterExtension(pub id);
impl std::ops::Deref for ILMessageFilterExtension {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for ILMessageFilterExtension {}
impl ILMessageFilterExtension {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"ILMessageFilterExtension").unwrap(), alloc) })
    }
}
impl INSObject for ILMessageFilterExtension {}
impl PNSObject for ILMessageFilterExtension {}
impl std::convert::TryFrom<NSObject> for ILMessageFilterExtension {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<ILMessageFilterExtension, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"ILMessageFilterExtension").unwrap()) };
        if is_kind_of {
            Ok(ILMessageFilterExtension(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to ILMessageFilterExtension")
        }
    }
}
impl IILMessageFilterExtension for ILMessageFilterExtension {}
pub trait IILMessageFilterExtension: Sized + std::ops::Deref {}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct ILMessageFilterExtensionContext(pub id);
impl std::ops::Deref for ILMessageFilterExtensionContext {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for ILMessageFilterExtensionContext {}
impl ILMessageFilterExtensionContext {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"ILMessageFilterExtensionContext").unwrap(), alloc) })
    }
}
impl INSExtensionContext for ILMessageFilterExtensionContext {}
impl std::convert::TryFrom<NSExtensionContext> for ILMessageFilterExtensionContext {
    type Error = &'static str;
    fn try_from(
        parent: NSExtensionContext,
    ) -> Result<ILMessageFilterExtensionContext, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"ILMessageFilterExtensionContext").unwrap())
        };
        if is_kind_of {
            Ok(ILMessageFilterExtensionContext(parent.0))
        } else {
            Err("This NSExtensionContext cannot be downcasted to ILMessageFilterExtensionContext")
        }
    }
}
impl INSObject for ILMessageFilterExtensionContext {}
impl PNSObject for ILMessageFilterExtensionContext {}
impl IILMessageFilterExtensionContext for ILMessageFilterExtensionContext {}
pub trait IILMessageFilterExtensionContext: Sized + std::ops::Deref {
    unsafe fn deferQueryRequestToNetworkWithCompletion_(
        &self,
        completion: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, deferQueryRequestToNetworkWithCompletion : completion)
    }
}
pub trait PILMessageFilterQueryHandling: Sized + std::ops::Deref {
    unsafe fn handleQueryRequest_context_completion_(
        &self,
        queryRequest: ILMessageFilterQueryRequest,
        context: ILMessageFilterExtensionContext,
        completion: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, handleQueryRequest : queryRequest, context : context, completion : completion)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct ILMessageFilterQueryRequest(pub id);
impl std::ops::Deref for ILMessageFilterQueryRequest {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for ILMessageFilterQueryRequest {}
impl ILMessageFilterQueryRequest {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"ILMessageFilterQueryRequest").unwrap(), alloc) })
    }
}
impl PNSSecureCoding for ILMessageFilterQueryRequest {}
impl INSObject for ILMessageFilterQueryRequest {}
impl PNSObject for ILMessageFilterQueryRequest {}
impl std::convert::TryFrom<NSObject> for ILMessageFilterQueryRequest {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<ILMessageFilterQueryRequest, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"ILMessageFilterQueryRequest").unwrap()) };
        if is_kind_of {
            Ok(ILMessageFilterQueryRequest(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to ILMessageFilterQueryRequest")
        }
    }
}
impl IILMessageFilterQueryRequest for ILMessageFilterQueryRequest {}
pub trait IILMessageFilterQueryRequest: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn sender(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, sender)
    }
    unsafe fn messageBody(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, messageBody)
    }
    unsafe fn receiverISOCountryCode(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, receiverISOCountryCode)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"ILMessageFilterQueryRequest").unwrap(), new)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct ILMessageFilterQueryResponse(pub id);
impl std::ops::Deref for ILMessageFilterQueryResponse {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for ILMessageFilterQueryResponse {}
impl ILMessageFilterQueryResponse {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"ILMessageFilterQueryResponse").unwrap(), alloc) })
    }
}
impl PNSSecureCoding for ILMessageFilterQueryResponse {}
impl INSObject for ILMessageFilterQueryResponse {}
impl PNSObject for ILMessageFilterQueryResponse {}
impl std::convert::TryFrom<NSObject> for ILMessageFilterQueryResponse {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<ILMessageFilterQueryResponse, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"ILMessageFilterQueryResponse").unwrap()) };
        if is_kind_of {
            Ok(ILMessageFilterQueryResponse(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to ILMessageFilterQueryResponse")
        }
    }
}
impl IILMessageFilterQueryResponse for ILMessageFilterQueryResponse {}
pub trait IILMessageFilterQueryResponse: Sized + std::ops::Deref {
    unsafe fn action(&self) -> ILMessageFilterAction
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, action)
    }
    unsafe fn setAction_(&self, action: ILMessageFilterAction)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAction : action)
    }
    unsafe fn subAction(&self) -> ILMessageFilterSubAction
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, subAction)
    }
    unsafe fn setSubAction_(&self, subAction: ILMessageFilterSubAction)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSubAction : subAction)
    }
}
pub trait PILMessageFilterCapabilitiesQueryHandling: Sized + std::ops::Deref {
    unsafe fn handleCapabilitiesQueryRequest_context_completion_(
        &self,
        capabilitiesQueryRequest: ILMessageFilterCapabilitiesQueryRequest,
        context: ILMessageFilterExtensionContext,
        completion: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, handleCapabilitiesQueryRequest : capabilitiesQueryRequest, context : context, completion : completion)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct ILMessageFilterCapabilitiesQueryRequest(pub id);
impl std::ops::Deref for ILMessageFilterCapabilitiesQueryRequest {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for ILMessageFilterCapabilitiesQueryRequest {}
impl ILMessageFilterCapabilitiesQueryRequest {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"ILMessageFilterCapabilitiesQueryRequest").unwrap(), alloc) })
    }
}
impl PNSSecureCoding for ILMessageFilterCapabilitiesQueryRequest {}
impl INSObject for ILMessageFilterCapabilitiesQueryRequest {}
impl PNSObject for ILMessageFilterCapabilitiesQueryRequest {}
impl std::convert::TryFrom<NSObject> for ILMessageFilterCapabilitiesQueryRequest {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<ILMessageFilterCapabilitiesQueryRequest, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"ILMessageFilterCapabilitiesQueryRequest").unwrap())
        };
        if is_kind_of {
            Ok(ILMessageFilterCapabilitiesQueryRequest(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to ILMessageFilterCapabilitiesQueryRequest")
        }
    }
}
impl IILMessageFilterCapabilitiesQueryRequest for ILMessageFilterCapabilitiesQueryRequest {}
pub trait IILMessageFilterCapabilitiesQueryRequest: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct ILMessageFilterCapabilitiesQueryResponse(pub id);
impl std::ops::Deref for ILMessageFilterCapabilitiesQueryResponse {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for ILMessageFilterCapabilitiesQueryResponse {}
impl ILMessageFilterCapabilitiesQueryResponse {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"ILMessageFilterCapabilitiesQueryResponse").unwrap(), alloc) })
    }
}
impl PNSSecureCoding for ILMessageFilterCapabilitiesQueryResponse {}
impl INSObject for ILMessageFilterCapabilitiesQueryResponse {}
impl PNSObject for ILMessageFilterCapabilitiesQueryResponse {}
impl std::convert::TryFrom<NSObject> for ILMessageFilterCapabilitiesQueryResponse {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<ILMessageFilterCapabilitiesQueryResponse, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"ILMessageFilterCapabilitiesQueryResponse").unwrap())
        };
        if is_kind_of {
            Ok(ILMessageFilterCapabilitiesQueryResponse(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to ILMessageFilterCapabilitiesQueryResponse")
        }
    }
}
impl IILMessageFilterCapabilitiesQueryResponse for ILMessageFilterCapabilitiesQueryResponse {}
pub trait IILMessageFilterCapabilitiesQueryResponse: Sized + std::ops::Deref {
    unsafe fn transactionalSubActions(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, transactionalSubActions)
    }
    unsafe fn setTransactionalSubActions_(&self, transactionalSubActions: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setTransactionalSubActions : transactionalSubActions)
    }
    unsafe fn promotionalSubActions(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, promotionalSubActions)
    }
    unsafe fn setPromotionalSubActions_(&self, promotionalSubActions: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPromotionalSubActions : promotionalSubActions)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct ILCommunication(pub id);
impl std::ops::Deref for ILCommunication {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for ILCommunication {}
impl ILCommunication {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"ILCommunication").unwrap(), alloc) })
    }
}
impl PNSSecureCoding for ILCommunication {}
impl INSObject for ILCommunication {}
impl PNSObject for ILCommunication {}
impl std::convert::TryFrom<NSObject> for ILCommunication {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<ILCommunication, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"ILCommunication").unwrap()) };
        if is_kind_of {
            Ok(ILCommunication(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to ILCommunication")
        }
    }
}
impl IILCommunication for ILCommunication {}
pub trait IILCommunication: Sized + std::ops::Deref {
    unsafe fn isEqualToCommunication_(&self, communication: ILCommunication) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, isEqualToCommunication : communication)
    }
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn sender(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, sender)
    }
    unsafe fn dateReceived(&self) -> NSDate
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, dateReceived)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct ILCallCommunication(pub id);
impl std::ops::Deref for ILCallCommunication {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for ILCallCommunication {}
impl ILCallCommunication {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"ILCallCommunication").unwrap(), alloc) })
    }
}
impl IILCommunication for ILCallCommunication {}
impl PNSSecureCoding for ILCallCommunication {}
impl From<ILCallCommunication> for ILCommunication {
    fn from(child: ILCallCommunication) -> ILCommunication {
        ILCommunication(child.0)
    }
}
impl std::convert::TryFrom<ILCommunication> for ILCallCommunication {
    type Error = &'static str;
    fn try_from(parent: ILCommunication) -> Result<ILCallCommunication, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"ILCallCommunication").unwrap()) };
        if is_kind_of {
            Ok(ILCallCommunication(parent.0))
        } else {
            Err("This ILCommunication cannot be downcasted to ILCallCommunication")
        }
    }
}
impl INSObject for ILCallCommunication {}
impl PNSObject for ILCallCommunication {}
impl IILCallCommunication for ILCallCommunication {}
pub trait IILCallCommunication: Sized + std::ops::Deref {
    unsafe fn isEqualToCallCommunication_(&self, communication: ILCallCommunication) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, isEqualToCallCommunication : communication)
    }
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct ILMessageCommunication(pub id);
impl std::ops::Deref for ILMessageCommunication {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for ILMessageCommunication {}
impl ILMessageCommunication {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"ILMessageCommunication").unwrap(), alloc) })
    }
}
impl IILCommunication for ILMessageCommunication {}
impl PNSSecureCoding for ILMessageCommunication {}
impl std::convert::TryFrom<ILCommunication> for ILMessageCommunication {
    type Error = &'static str;
    fn try_from(parent: ILCommunication) -> Result<ILMessageCommunication, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"ILMessageCommunication").unwrap()) };
        if is_kind_of {
            Ok(ILMessageCommunication(parent.0))
        } else {
            Err("This ILCommunication cannot be downcasted to ILMessageCommunication")
        }
    }
}
impl INSObject for ILMessageCommunication {}
impl PNSObject for ILMessageCommunication {}
impl IILMessageCommunication for ILMessageCommunication {}
pub trait IILMessageCommunication: Sized + std::ops::Deref {
    unsafe fn isEqualToMessageCommunication_(&self, communication: ILMessageCommunication) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, isEqualToMessageCommunication : communication)
    }
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn messageBody(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, messageBody)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct ILClassificationRequest(pub id);
impl std::ops::Deref for ILClassificationRequest {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for ILClassificationRequest {}
impl ILClassificationRequest {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"ILClassificationRequest").unwrap(), alloc) })
    }
}
impl PNSSecureCoding for ILClassificationRequest {}
impl INSObject for ILClassificationRequest {}
impl PNSObject for ILClassificationRequest {}
impl std::convert::TryFrom<NSObject> for ILClassificationRequest {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<ILClassificationRequest, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"ILClassificationRequest").unwrap()) };
        if is_kind_of {
            Ok(ILClassificationRequest(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to ILClassificationRequest")
        }
    }
}
impl IILClassificationRequest for ILClassificationRequest {}
pub trait IILClassificationRequest: Sized + std::ops::Deref {}
pub type ILClassificationAction = NSInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct ILClassificationResponse(pub id);
impl std::ops::Deref for ILClassificationResponse {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for ILClassificationResponse {}
impl ILClassificationResponse {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"ILClassificationResponse").unwrap(), alloc) })
    }
}
impl PNSSecureCoding for ILClassificationResponse {}
impl INSObject for ILClassificationResponse {}
impl PNSObject for ILClassificationResponse {}
impl std::convert::TryFrom<NSObject> for ILClassificationResponse {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<ILClassificationResponse, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"ILClassificationResponse").unwrap()) };
        if is_kind_of {
            Ok(ILClassificationResponse(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to ILClassificationResponse")
        }
    }
}
impl IILClassificationResponse for ILClassificationResponse {}
pub trait IILClassificationResponse: Sized + std::ops::Deref {
    unsafe fn initWithClassificationAction_(&self, action: ILClassificationAction) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithClassificationAction : action)
    }
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn action(&self) -> ILClassificationAction
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, action)
    }
    unsafe fn userString(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, userString)
    }
    unsafe fn setUserString_(&self, userString: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setUserString : userString)
    }
    unsafe fn userInfo(&self) -> NSDictionary
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, userInfo)
    }
    unsafe fn setUserInfo_(&self, userInfo: NSDictionary)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setUserInfo : userInfo)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct ILMessageClassificationRequest(pub id);
impl std::ops::Deref for ILMessageClassificationRequest {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for ILMessageClassificationRequest {}
impl ILMessageClassificationRequest {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"ILMessageClassificationRequest").unwrap(), alloc) })
    }
}
impl PNSSecureCoding for ILMessageClassificationRequest {}
impl IILClassificationRequest for ILMessageClassificationRequest {}
impl From<ILMessageClassificationRequest> for ILClassificationRequest {
    fn from(child: ILMessageClassificationRequest) -> ILClassificationRequest {
        ILClassificationRequest(child.0)
    }
}
impl std::convert::TryFrom<ILClassificationRequest> for ILMessageClassificationRequest {
    type Error = &'static str;
    fn try_from(
        parent: ILClassificationRequest,
    ) -> Result<ILMessageClassificationRequest, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"ILMessageClassificationRequest").unwrap())
        };
        if is_kind_of {
            Ok(ILMessageClassificationRequest(parent.0))
        } else {
            Err ("This ILClassificationRequest cannot be downcasted to ILMessageClassificationRequest" ,)
        }
    }
}
impl INSObject for ILMessageClassificationRequest {}
impl PNSObject for ILMessageClassificationRequest {}
impl IILMessageClassificationRequest for ILMessageClassificationRequest {}
pub trait IILMessageClassificationRequest: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn messageCommunications(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, messageCommunications)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct ILCallClassificationRequest(pub id);
impl std::ops::Deref for ILCallClassificationRequest {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for ILCallClassificationRequest {}
impl ILCallClassificationRequest {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"ILCallClassificationRequest").unwrap(), alloc) })
    }
}
impl PNSSecureCoding for ILCallClassificationRequest {}
impl IILClassificationRequest for ILCallClassificationRequest {}
impl std::convert::TryFrom<ILClassificationRequest> for ILCallClassificationRequest {
    type Error = &'static str;
    fn try_from(
        parent: ILClassificationRequest,
    ) -> Result<ILCallClassificationRequest, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"ILCallClassificationRequest").unwrap()) };
        if is_kind_of {
            Ok(ILCallClassificationRequest(parent.0))
        } else {
            Err("This ILClassificationRequest cannot be downcasted to ILCallClassificationRequest")
        }
    }
}
impl INSObject for ILCallClassificationRequest {}
impl PNSObject for ILCallClassificationRequest {}
impl IILCallClassificationRequest for ILCallClassificationRequest {}
pub trait IILCallClassificationRequest: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn callCommunications(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, callCommunications)
    }
}
unsafe extern "C" {
    pub static ILMessageFilterErrorDomain: NSErrorDomain;
}

unsafe impl objc2::encode::RefEncode for ILNetworkResponse {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for ILNetworkResponse {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for ILMessageFilterExtension {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for ILMessageFilterExtension {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for ILMessageFilterExtensionContext {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for ILMessageFilterExtensionContext {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for ILMessageFilterQueryRequest {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for ILMessageFilterQueryRequest {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for ILMessageFilterQueryResponse {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for ILMessageFilterQueryResponse {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for ILMessageFilterCapabilitiesQueryRequest {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for ILMessageFilterCapabilitiesQueryRequest {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for ILMessageFilterCapabilitiesQueryResponse {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for ILMessageFilterCapabilitiesQueryResponse {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for ILCommunication {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for ILCommunication {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for ILCallCommunication {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for ILCallCommunication {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for ILMessageCommunication {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for ILMessageCommunication {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for ILClassificationRequest {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for ILClassificationRequest {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for ILClassificationResponse {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for ILClassificationResponse {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for ILMessageClassificationRequest {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for ILMessageClassificationRequest {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for ILCallClassificationRequest {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for ILCallClassificationRequest {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
