#[allow(unused_imports)]
use crate::objc::*;
#[allow(unused_imports)]
use crate::AVFoundation::*;
#[allow(unused_imports)]
use crate::CoreFoundation::*;
#[allow(unused_imports)]
use crate::Foundation::*;
#[allow(unused_imports)]
use crate::IOKit::*;
#[allow(unused_imports)]
use crate::IOSurface::*;
#[allow(unused_imports)]
use crate::QuartzCore::*;
#[allow(unused_imports)]
use crate::UniformTypeIdentifiers::*;
#[allow(unused_imports)]
use libc::mach_port_t;

#[allow(unused_imports)]
use objc2::msg_send;
pub type xpc_connection_t = xpc_object_t;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct BELayerHierarchyHandle(pub id);
impl std::ops::Deref for BELayerHierarchyHandle {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for BELayerHierarchyHandle {}
impl BELayerHierarchyHandle {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"BELayerHierarchyHandle").unwrap(), alloc) })
    }
}
impl PNSSecureCoding for BELayerHierarchyHandle {}
impl INSObject for BELayerHierarchyHandle {}
impl PNSObject for BELayerHierarchyHandle {}
impl std::convert::TryFrom<NSObject> for BELayerHierarchyHandle {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<BELayerHierarchyHandle, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"BELayerHierarchyHandle").unwrap()) };
        if is_kind_of {
            Ok(BELayerHierarchyHandle(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to BELayerHierarchyHandle")
        }
    }
}
impl IBELayerHierarchyHandle for BELayerHierarchyHandle {}
pub trait IBELayerHierarchyHandle: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn encodeWithBlock_(&self, block: *mut ::std::os::raw::c_void)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, encodeWithBlock : block)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"BELayerHierarchyHandle").unwrap(), new)
    }
    unsafe fn handleWithPort_data_error_(
        port: mach_port_t,
        data: NSData,
        error: *mut NSError,
    ) -> BELayerHierarchyHandle
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"BELayerHierarchyHandle").unwrap(), handleWithPort : port, data : data, error : error)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct BELayerHierarchy(pub id);
impl std::ops::Deref for BELayerHierarchy {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for BELayerHierarchy {}
impl BELayerHierarchy {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"BELayerHierarchy").unwrap(), alloc) })
    }
}
impl INSObject for BELayerHierarchy {}
impl PNSObject for BELayerHierarchy {}
impl std::convert::TryFrom<NSObject> for BELayerHierarchy {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<BELayerHierarchy, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"BELayerHierarchy").unwrap()) };
        if is_kind_of {
            Ok(BELayerHierarchy(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to BELayerHierarchy")
        }
    }
}
impl IBELayerHierarchy for BELayerHierarchy {}
pub trait IBELayerHierarchy: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn invalidate(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, invalidate)
    }
    unsafe fn handle(&self) -> BELayerHierarchyHandle
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, handle)
    }
    unsafe fn layer(&self) -> CALayer
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, layer)
    }
    unsafe fn setLayer_(&self, layer: CALayer)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setLayer : layer)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"BELayerHierarchy").unwrap(), new)
    }
    unsafe fn layerHierarchyWithError_(error: *mut NSError) -> BELayerHierarchy
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"BELayerHierarchy").unwrap(), layerHierarchyWithError : error)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct BELayerHierarchyHostingTransactionCoordinator(pub id);
impl std::ops::Deref for BELayerHierarchyHostingTransactionCoordinator {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for BELayerHierarchyHostingTransactionCoordinator {}
impl BELayerHierarchyHostingTransactionCoordinator {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"BELayerHierarchyHostingTransactionCoordinator").unwrap(), alloc) })
    }
}
impl PNSSecureCoding for BELayerHierarchyHostingTransactionCoordinator {}
impl INSObject for BELayerHierarchyHostingTransactionCoordinator {}
impl PNSObject for BELayerHierarchyHostingTransactionCoordinator {}
impl std::convert::TryFrom<NSObject> for BELayerHierarchyHostingTransactionCoordinator {
    type Error = &'static str;
    fn try_from(
        parent: NSObject,
    ) -> Result<BELayerHierarchyHostingTransactionCoordinator, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"BELayerHierarchyHostingTransactionCoordinator").unwrap())
        };
        if is_kind_of {
            Ok(BELayerHierarchyHostingTransactionCoordinator(parent.0))
        } else {
            Err ("This NSObject cannot be downcasted to BELayerHierarchyHostingTransactionCoordinator" ,)
        }
    }
}
impl IBELayerHierarchyHostingTransactionCoordinator
    for BELayerHierarchyHostingTransactionCoordinator
{
}
pub trait IBELayerHierarchyHostingTransactionCoordinator: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn addLayerHierarchy_(&self, layerHierarchy: BELayerHierarchy)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, addLayerHierarchy : layerHierarchy)
    }
    unsafe fn commit(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, commit)
    }
    unsafe fn encodeWithBlock_(&self, block: *mut ::std::os::raw::c_void)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, encodeWithBlock : block)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"BELayerHierarchyHostingTransactionCoordinator").unwrap(), new)
    }
    unsafe fn coordinatorWithError_(
        error: *mut NSError,
    ) -> BELayerHierarchyHostingTransactionCoordinator
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"BELayerHierarchyHostingTransactionCoordinator").unwrap(), coordinatorWithError : error)
    }
    unsafe fn coordinatorWithPort_data_error_(
        port: mach_port_t,
        data: NSData,
        error: *mut NSError,
    ) -> BELayerHierarchyHostingTransactionCoordinator
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"BELayerHierarchyHostingTransactionCoordinator").unwrap(), coordinatorWithPort : port, data : data, error : error)
    }
}
pub trait PBEExtensionProcess: Sized + std::ops::Deref {
    unsafe fn invalidate(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, invalidate)
    }
    unsafe fn makeLibXPCConnectionError_(&self, error: *mut NSError) -> xpc_connection_t
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, makeLibXPCConnectionError : error)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct BEWebContentProcess(pub id);
impl std::ops::Deref for BEWebContentProcess {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for BEWebContentProcess {}
impl BEWebContentProcess {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"BEWebContentProcess").unwrap(), alloc) })
    }
}
impl INSObject for BEWebContentProcess {}
impl PNSObject for BEWebContentProcess {}
impl std::convert::TryFrom<NSObject> for BEWebContentProcess {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<BEWebContentProcess, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"BEWebContentProcess").unwrap()) };
        if is_kind_of {
            Ok(BEWebContentProcess(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to BEWebContentProcess")
        }
    }
}
impl IBEWebContentProcess for BEWebContentProcess {}
pub trait IBEWebContentProcess: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn invalidate(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, invalidate)
    }
    unsafe fn makeLibXPCConnectionError_(&self, error: *mut NSError) -> xpc_connection_t
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, makeLibXPCConnectionError : error)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"BEWebContentProcess").unwrap(), new)
    }
    unsafe fn webContentProcessWithInterruptionHandler_completion_(
        interruptionHandler: *mut ::std::os::raw::c_void,
        completion: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"BEWebContentProcess").unwrap(), webContentProcessWithInterruptionHandler : interruptionHandler, completion : completion)
    }
    unsafe fn webContentProcessWithBundleID_interruptionHandler_completion_(
        bundleID: NSString,
        interruptionHandler: *mut ::std::os::raw::c_void,
        completion: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"BEWebContentProcess").unwrap(), webContentProcessWithBundleID : bundleID, interruptionHandler : interruptionHandler, completion : completion)
    }
}
impl BEWebContentProcess_BEExtensionProcessConformance for BEWebContentProcess {}
pub trait BEWebContentProcess_BEExtensionProcessConformance: Sized + std::ops::Deref {}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct BENetworkingProcess(pub id);
impl std::ops::Deref for BENetworkingProcess {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for BENetworkingProcess {}
impl BENetworkingProcess {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"BENetworkingProcess").unwrap(), alloc) })
    }
}
impl INSObject for BENetworkingProcess {}
impl PNSObject for BENetworkingProcess {}
impl std::convert::TryFrom<NSObject> for BENetworkingProcess {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<BENetworkingProcess, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"BENetworkingProcess").unwrap()) };
        if is_kind_of {
            Ok(BENetworkingProcess(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to BENetworkingProcess")
        }
    }
}
impl IBENetworkingProcess for BENetworkingProcess {}
pub trait IBENetworkingProcess: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn invalidate(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, invalidate)
    }
    unsafe fn makeLibXPCConnectionError_(&self, error: *mut NSError) -> xpc_connection_t
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, makeLibXPCConnectionError : error)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"BENetworkingProcess").unwrap(), new)
    }
    unsafe fn networkProcessWithInterruptionHandler_completion_(
        interruptionHandler: *mut ::std::os::raw::c_void,
        completion: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"BENetworkingProcess").unwrap(), networkProcessWithInterruptionHandler : interruptionHandler, completion : completion)
    }
    unsafe fn networkProcessWithBundleID_interruptionHandler_completion_(
        bundleID: NSString,
        interruptionHandler: *mut ::std::os::raw::c_void,
        completion: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"BENetworkingProcess").unwrap(), networkProcessWithBundleID : bundleID, interruptionHandler : interruptionHandler, completion : completion)
    }
}
impl BENetworkingProcess_BEExtensionProcessConformance for BENetworkingProcess {}
pub trait BENetworkingProcess_BEExtensionProcessConformance: Sized + std::ops::Deref {}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct BERenderingProcess(pub id);
impl std::ops::Deref for BERenderingProcess {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for BERenderingProcess {}
impl BERenderingProcess {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"BERenderingProcess").unwrap(), alloc) })
    }
}
impl INSObject for BERenderingProcess {}
impl PNSObject for BERenderingProcess {}
impl std::convert::TryFrom<NSObject> for BERenderingProcess {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<BERenderingProcess, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"BERenderingProcess").unwrap()) };
        if is_kind_of {
            Ok(BERenderingProcess(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to BERenderingProcess")
        }
    }
}
impl IBERenderingProcess for BERenderingProcess {}
pub trait IBERenderingProcess: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn invalidate(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, invalidate)
    }
    unsafe fn makeLibXPCConnectionError_(&self, error: *mut NSError) -> xpc_connection_t
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, makeLibXPCConnectionError : error)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"BERenderingProcess").unwrap(), new)
    }
    unsafe fn renderingProcessWithInterruptionHandler_completion_(
        interruptionHandler: *mut ::std::os::raw::c_void,
        completion: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"BERenderingProcess").unwrap(), renderingProcessWithInterruptionHandler : interruptionHandler, completion : completion)
    }
    unsafe fn renderingProcessWithBundleID_interruptionHandler_completion_(
        bundleID: NSString,
        interruptionHandler: *mut ::std::os::raw::c_void,
        completion: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"BERenderingProcess").unwrap(), renderingProcessWithBundleID : bundleID, interruptionHandler : interruptionHandler, completion : completion)
    }
}
impl BERenderingProcess_BEExtensionProcessConformance for BERenderingProcess {}
pub trait BERenderingProcess_BEExtensionProcessConformance: Sized + std::ops::Deref {}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct BEKeyEntryContext(pub id);
impl std::ops::Deref for BEKeyEntryContext {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for BEKeyEntryContext {}
impl BEKeyEntryContext {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"BEKeyEntryContext").unwrap(), alloc) })
    }
}
impl IBEKeyEntryContext for BEKeyEntryContext {}
pub trait IBEKeyEntryContext: Sized + std::ops::Deref {}
pub trait PBETextInputDelegate: Sized + std::ops::Deref {
    unsafe fn shouldDeferEventHandlingToSystemForTextInput_context_(
        &self,
        textInput: *mut u64,
        keyEventContext: BEKeyEntryContext,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, shouldDeferEventHandlingToSystemForTextInput : textInput, context : keyEventContext)
    }
    unsafe fn textInput_setCandidateSuggestions_(&self, textInput: *mut u64, suggestions: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, textInput : textInput, setCandidateSuggestions : suggestions)
    }
    unsafe fn selectionWillChangeForTextInput_(&self, textInput: *mut u64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, selectionWillChangeForTextInput : textInput)
    }
    unsafe fn selectionDidChangeForTextInput_(&self, textInput: *mut u64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, selectionDidChangeForTextInput : textInput)
    }
    unsafe fn textInput_deferReplaceTextActionToSystem_(&self, textInput: *mut u64, sender: id)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, textInput : textInput, deferReplaceTextActionToSystem : sender)
    }
    unsafe fn invalidateTextEntryContextForTextInput_(&self, textInput: *mut u64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, invalidateTextEntryContextForTextInput : textInput)
    }
}
pub type BEGestureType = NSInteger;
pub type BESelectionTouchPhase = NSInteger;
pub type BESelectionFlags = NSUInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct BETextAlternatives(pub id);
impl std::ops::Deref for BETextAlternatives {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for BETextAlternatives {}
impl BETextAlternatives {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"BETextAlternatives").unwrap(), alloc) })
    }
}
impl INSObject for BETextAlternatives {}
impl PNSObject for BETextAlternatives {}
impl std::convert::TryFrom<NSObject> for BETextAlternatives {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<BETextAlternatives, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"BETextAlternatives").unwrap()) };
        if is_kind_of {
            Ok(BETextAlternatives(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to BETextAlternatives")
        }
    }
}
impl IBETextAlternatives for BETextAlternatives {}
pub trait IBETextAlternatives: Sized + std::ops::Deref {
    unsafe fn new(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, new)
    }
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn primaryString(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, primaryString)
    }
    unsafe fn alternativeStrings(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, alternativeStrings)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct BETextSuggestion(pub id);
impl std::ops::Deref for BETextSuggestion {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for BETextSuggestion {}
impl BETextSuggestion {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"BETextSuggestion").unwrap(), alloc) })
    }
}
impl INSObject for BETextSuggestion {}
impl PNSObject for BETextSuggestion {}
impl std::convert::TryFrom<NSObject> for BETextSuggestion {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<BETextSuggestion, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"BETextSuggestion").unwrap()) };
        if is_kind_of {
            Ok(BETextSuggestion(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to BETextSuggestion")
        }
    }
}
impl IBETextSuggestion for BETextSuggestion {}
pub trait IBETextSuggestion: Sized + std::ops::Deref {
    unsafe fn initWithInputText_(&self, inputText: NSString) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithInputText : inputText)
    }
    unsafe fn new(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, new)
    }
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn inputText(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inputText)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct BETextInteraction(pub id);
impl std::ops::Deref for BETextInteraction {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for BETextInteraction {}
impl BETextInteraction {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"BETextInteraction").unwrap(), alloc) })
    }
}
impl IBETextInteraction for BETextInteraction {}
pub trait IBETextInteraction: Sized + std::ops::Deref {}
pub trait PBETextInteractionDelegate: Sized + std::ops::Deref {
    unsafe fn systemWillChangeSelectionForInteraction_(&self, textInteraction: BETextInteraction)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, systemWillChangeSelectionForInteraction : textInteraction)
    }
    unsafe fn systemDidChangeSelectionForInteraction_(&self, textInteraction: BETextInteraction)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, systemDidChangeSelectionForInteraction : textInteraction)
    }
}
pub trait PBEProcessCapabilityGrant: Sized + std::ops::Deref {
    unsafe fn invalidate(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, invalidate)
    }
    unsafe fn isValid(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isValid)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct BEMediaEnvironment(pub id);
impl std::ops::Deref for BEMediaEnvironment {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for BEMediaEnvironment {}
impl BEMediaEnvironment {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"BEMediaEnvironment").unwrap(), alloc) })
    }
}
impl INSObject for BEMediaEnvironment {}
impl PNSObject for BEMediaEnvironment {}
impl std::convert::TryFrom<NSObject> for BEMediaEnvironment {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<BEMediaEnvironment, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"BEMediaEnvironment").unwrap()) };
        if is_kind_of {
            Ok(BEMediaEnvironment(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to BEMediaEnvironment")
        }
    }
}
impl IBEMediaEnvironment for BEMediaEnvironment {}
pub trait IBEMediaEnvironment: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn initWithWebPageURL_(&self, url: NSURL) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithWebPageURL : url)
    }
    unsafe fn activateWithError_(&self, error: *mut NSError) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, activateWithError : error)
    }
    unsafe fn suspendWithError_(&self, error: *mut NSError) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, suspendWithError : error)
    }
    unsafe fn makeCaptureSessionWithError_(&self, error: *mut NSError) -> AVCaptureSession
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, makeCaptureSessionWithError : error)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"BEMediaEnvironment").unwrap(), new)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct BEProcessCapability(pub id);
impl std::ops::Deref for BEProcessCapability {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for BEProcessCapability {}
impl BEProcessCapability {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"BEProcessCapability").unwrap(), alloc) })
    }
}
impl INSObject for BEProcessCapability {}
impl PNSObject for BEProcessCapability {}
impl std::convert::TryFrom<NSObject> for BEProcessCapability {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<BEProcessCapability, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"BEProcessCapability").unwrap()) };
        if is_kind_of {
            Ok(BEProcessCapability(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to BEProcessCapability")
        }
    }
}
impl IBEProcessCapability for BEProcessCapability {}
pub trait IBEProcessCapability: Sized + std::ops::Deref {
    unsafe fn requestWithError_(&self, error: *mut NSError) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, requestWithError : error)
    }
    unsafe fn mediaPlaybackAndCaptureWithEnvironment_(
        environment: BEMediaEnvironment,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"BEProcessCapability").unwrap(), mediaPlaybackAndCaptureWithEnvironment : environment)
    }
    unsafe fn background() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"BEProcessCapability").unwrap(), background)
    }
    unsafe fn foreground() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"BEProcessCapability").unwrap(), foreground)
    }
    unsafe fn suspended() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"BEProcessCapability").unwrap(), suspended)
    }
}
impl BEWebContentProcess_Capability for BEWebContentProcess {}
pub trait BEWebContentProcess_Capability: Sized + std::ops::Deref {
    unsafe fn grantCapability_error_(
        &self,
        capability: BEProcessCapability,
        error: *mut NSError,
    ) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, grantCapability : capability, error : error)
    }
}
impl BEWebContentProcess_CapabilityInvalidationHandler for BEWebContentProcess {}
pub trait BEWebContentProcess_CapabilityInvalidationHandler: Sized + std::ops::Deref {
    unsafe fn grantCapability_error_invalidationHandler_(
        &self,
        capability: BEProcessCapability,
        error: *mut NSError,
        invalidationHandler: *mut ::std::os::raw::c_void,
    ) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, grantCapability : capability, error : error, invalidationHandler : invalidationHandler)
    }
}
impl BERenderingProcess_Capability for BERenderingProcess {}
pub trait BERenderingProcess_Capability: Sized + std::ops::Deref {
    unsafe fn grantCapability_error_(
        &self,
        capability: BEProcessCapability,
        error: *mut NSError,
    ) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, grantCapability : capability, error : error)
    }
}
impl BERenderingProcess_CapabilityInvalidationHandler for BERenderingProcess {}
pub trait BERenderingProcess_CapabilityInvalidationHandler: Sized + std::ops::Deref {
    unsafe fn grantCapability_error_invalidationHandler_(
        &self,
        capability: BEProcessCapability,
        error: *mut NSError,
        invalidationHandler: *mut ::std::os::raw::c_void,
    ) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, grantCapability : capability, error : error, invalidationHandler : invalidationHandler)
    }
}
impl BENetworkingProcess_Capability for BENetworkingProcess {}
pub trait BENetworkingProcess_Capability: Sized + std::ops::Deref {
    unsafe fn grantCapability_error_(
        &self,
        capability: BEProcessCapability,
        error: *mut NSError,
    ) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, grantCapability : capability, error : error)
    }
}
impl BENetworkingProcess_CapabilityInvalidationHandler for BENetworkingProcess {}
pub trait BENetworkingProcess_CapabilityInvalidationHandler: Sized + std::ops::Deref {
    unsafe fn grantCapability_error_invalidationHandler_(
        &self,
        capability: BEProcessCapability,
        error: *mut NSError,
        invalidationHandler: *mut ::std::os::raw::c_void,
    ) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, grantCapability : capability, error : error, invalidationHandler : invalidationHandler)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct BEWebAppManifest(pub id);
impl std::ops::Deref for BEWebAppManifest {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for BEWebAppManifest {}
impl BEWebAppManifest {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"BEWebAppManifest").unwrap(), alloc) })
    }
}
impl INSObject for BEWebAppManifest {}
impl PNSObject for BEWebAppManifest {}
impl std::convert::TryFrom<NSObject> for BEWebAppManifest {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<BEWebAppManifest, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"BEWebAppManifest").unwrap()) };
        if is_kind_of {
            Ok(BEWebAppManifest(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to BEWebAppManifest")
        }
    }
}
impl IBEWebAppManifest for BEWebAppManifest {}
pub trait IBEWebAppManifest: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn initWithJSONData_manifestURL_(
        &self,
        jsonData: NSData,
        manifestURL: NSURL,
    ) -> BEWebAppManifest
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithJSONData : jsonData, manifestURL : manifestURL)
    }
    unsafe fn jsonData(&self) -> NSData
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, jsonData)
    }
    unsafe fn manifestURL(&self) -> NSURL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, manifestURL)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct BEWebContentFilter(pub id);
impl std::ops::Deref for BEWebContentFilter {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for BEWebContentFilter {}
impl BEWebContentFilter {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"BEWebContentFilter").unwrap(), alloc) })
    }
}
impl INSObject for BEWebContentFilter {}
impl PNSObject for BEWebContentFilter {}
impl std::convert::TryFrom<NSObject> for BEWebContentFilter {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<BEWebContentFilter, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"BEWebContentFilter").unwrap()) };
        if is_kind_of {
            Ok(BEWebContentFilter(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to BEWebContentFilter")
        }
    }
}
impl IBEWebContentFilter for BEWebContentFilter {}
pub trait IBEWebContentFilter: Sized + std::ops::Deref {
    unsafe fn evaluateURL_completionHandler_(
        &self,
        url: NSURL,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, evaluateURL : url, completionHandler : completionHandler)
    }
    unsafe fn allowURL_completionHandler_(
        &self,
        url: NSURL,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, allowURL : url, completionHandler : completionHandler)
    }
    unsafe fn shouldEvaluateURLs() -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"BEWebContentFilter").unwrap(), shouldEvaluateURLs)
    }
}
pub type BEAccessibilityPressedState = NSInteger;
pub type BEAccessibilityContainerType = NSUInteger;
pub trait NSObject_BEAccessibility: Sized + std::ops::Deref {
    unsafe fn browserAccessibilitySelectedTextRange(&self) -> NSRange
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, browserAccessibilitySelectedTextRange)
    }
    unsafe fn browserAccessibilitySetSelectedTextRange_(&self, range: NSRange)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, browserAccessibilitySetSelectedTextRange : range)
    }
    unsafe fn browserAccessibilityValueInRange_(&self, range: NSRange) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, browserAccessibilityValueInRange : range)
    }
    unsafe fn browserAccessibilityAttributedValueInRange_(
        &self,
        range: NSRange,
    ) -> NSAttributedString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, browserAccessibilityAttributedValueInRange : range)
    }
    unsafe fn browserAccessibilityInsertTextAtCursor_(&self, text: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, browserAccessibilityInsertTextAtCursor : text)
    }
    unsafe fn browserAccessibilityDeleteTextAtCursor_(&self, numberOfCharacters: NSInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, browserAccessibilityDeleteTextAtCursor : numberOfCharacters)
    }
    unsafe fn accessibilityLineEndPositionFromCurrentSelection(&self) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, accessibilityLineEndPositionFromCurrentSelection)
    }
    unsafe fn accessibilityLineStartPositionFromCurrentSelection(&self) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, accessibilityLineStartPositionFromCurrentSelection)
    }
    unsafe fn accessibilityLineRangeForPosition_(&self, position: NSInteger) -> NSRange
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, accessibilityLineRangeForPosition : position)
    }
    unsafe fn browserAccessibilityCurrentStatus(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, browserAccessibilityCurrentStatus)
    }
    unsafe fn setBrowserAccessibilityCurrentStatus_(
        &self,
        browserAccessibilityCurrentStatus: NSString,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setBrowserAccessibilityCurrentStatus : browserAccessibilityCurrentStatus)
    }
    unsafe fn browserAccessibilitySortDirection(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, browserAccessibilitySortDirection)
    }
    unsafe fn setBrowserAccessibilitySortDirection_(
        &self,
        browserAccessibilitySortDirection: NSString,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setBrowserAccessibilitySortDirection : browserAccessibilitySortDirection)
    }
    unsafe fn browserAccessibilityRoleDescription(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, browserAccessibilityRoleDescription)
    }
    unsafe fn setBrowserAccessibilityRoleDescription_(
        &self,
        browserAccessibilityRoleDescription: NSString,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setBrowserAccessibilityRoleDescription : browserAccessibilityRoleDescription)
    }
    unsafe fn browserAccessibilityIsRequired(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, browserAccessibilityIsRequired)
    }
    unsafe fn setBrowserAccessibilityIsRequired_(&self, browserAccessibilityIsRequired: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setBrowserAccessibilityIsRequired : browserAccessibilityIsRequired)
    }
    unsafe fn browserAccessibilityPressedState(&self) -> BEAccessibilityPressedState
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, browserAccessibilityPressedState)
    }
    unsafe fn setBrowserAccessibilityPressedState_(
        &self,
        browserAccessibilityPressedState: BEAccessibilityPressedState,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setBrowserAccessibilityPressedState : browserAccessibilityPressedState)
    }
    unsafe fn browserAccessibilityHasDOMFocus(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, browserAccessibilityHasDOMFocus)
    }
    unsafe fn setBrowserAccessibilityHasDOMFocus_(&self, browserAccessibilityHasDOMFocus: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setBrowserAccessibilityHasDOMFocus : browserAccessibilityHasDOMFocus)
    }
    unsafe fn browserAccessibilityContainerType(&self) -> BEAccessibilityContainerType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, browserAccessibilityContainerType)
    }
    unsafe fn setBrowserAccessibilityContainerType_(
        &self,
        browserAccessibilityContainerType: BEAccessibilityContainerType,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setBrowserAccessibilityContainerType : browserAccessibilityContainerType)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct BEAccessibilityTextMarker(pub id);
impl std::ops::Deref for BEAccessibilityTextMarker {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for BEAccessibilityTextMarker {}
impl BEAccessibilityTextMarker {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"BEAccessibilityTextMarker").unwrap(), alloc) })
    }
}
impl PNSCopying for BEAccessibilityTextMarker {}
impl PNSSecureCoding for BEAccessibilityTextMarker {}
impl INSObject for BEAccessibilityTextMarker {}
impl PNSObject for BEAccessibilityTextMarker {}
impl std::convert::TryFrom<NSObject> for BEAccessibilityTextMarker {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<BEAccessibilityTextMarker, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"BEAccessibilityTextMarker").unwrap()) };
        if is_kind_of {
            Ok(BEAccessibilityTextMarker(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to BEAccessibilityTextMarker")
        }
    }
}
impl IBEAccessibilityTextMarker for BEAccessibilityTextMarker {}
pub trait IBEAccessibilityTextMarker: Sized + std::ops::Deref {}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct BEAccessibilityTextMarkerRange(pub id);
impl std::ops::Deref for BEAccessibilityTextMarkerRange {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for BEAccessibilityTextMarkerRange {}
impl BEAccessibilityTextMarkerRange {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"BEAccessibilityTextMarkerRange").unwrap(), alloc) })
    }
}
impl PNSCopying for BEAccessibilityTextMarkerRange {}
impl PNSSecureCoding for BEAccessibilityTextMarkerRange {}
impl INSObject for BEAccessibilityTextMarkerRange {}
impl PNSObject for BEAccessibilityTextMarkerRange {}
impl std::convert::TryFrom<NSObject> for BEAccessibilityTextMarkerRange {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<BEAccessibilityTextMarkerRange, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"BEAccessibilityTextMarkerRange").unwrap())
        };
        if is_kind_of {
            Ok(BEAccessibilityTextMarkerRange(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to BEAccessibilityTextMarkerRange")
        }
    }
}
impl IBEAccessibilityTextMarkerRange for BEAccessibilityTextMarkerRange {}
pub trait IBEAccessibilityTextMarkerRange: Sized + std::ops::Deref {
    unsafe fn startMarker(&self) -> BEAccessibilityTextMarker
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, startMarker)
    }
    unsafe fn setStartMarker_(&self, startMarker: BEAccessibilityTextMarker)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setStartMarker : startMarker)
    }
    unsafe fn endMarker(&self) -> BEAccessibilityTextMarker
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, endMarker)
    }
    unsafe fn setEndMarker_(&self, endMarker: BEAccessibilityTextMarker)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setEndMarker : endMarker)
    }
}
pub trait PBEAccessibilityTextMarkerSupport: Sized + std::ops::Deref {
    unsafe fn accessibilityBoundsForTextMarkerRange_(
        &self,
        range: BEAccessibilityTextMarkerRange,
    ) -> CGRect
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, accessibilityBoundsForTextMarkerRange : range)
    }
    unsafe fn accessibilityContentForTextMarkerRange_(
        &self,
        range: BEAccessibilityTextMarkerRange,
    ) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, accessibilityContentForTextMarkerRange : range)
    }
    unsafe fn accessibilityTextMarkerRangeForCurrentSelection(
        &self,
    ) -> BEAccessibilityTextMarkerRange
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, accessibilityTextMarkerRangeForCurrentSelection)
    }
    unsafe fn accessibilityTextMarkerRange(&self) -> BEAccessibilityTextMarkerRange
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, accessibilityTextMarkerRange)
    }
    unsafe fn accessibilityNextTextMarker_(
        &self,
        marker: BEAccessibilityTextMarker,
    ) -> BEAccessibilityTextMarker
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, accessibilityNextTextMarker : marker)
    }
    unsafe fn accessibilityPreviousTextMarker_(
        &self,
        marker: BEAccessibilityTextMarker,
    ) -> BEAccessibilityTextMarker
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, accessibilityPreviousTextMarker : marker)
    }
    unsafe fn accessibilityLineEndMarkerForMarker_(
        &self,
        marker: BEAccessibilityTextMarker,
    ) -> BEAccessibilityTextMarker
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, accessibilityLineEndMarkerForMarker : marker)
    }
    unsafe fn accessibilityLineStartMarkerForMarker_(
        &self,
        marker: BEAccessibilityTextMarker,
    ) -> BEAccessibilityTextMarker
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, accessibilityLineStartMarkerForMarker : marker)
    }
    unsafe fn accessibilityMarkerForPoint_(&self, point: CGPoint) -> BEAccessibilityTextMarker
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, accessibilityMarkerForPoint : point)
    }
    unsafe fn accessibilityTextMarkerForPosition_(
        &self,
        position: NSInteger,
    ) -> BEAccessibilityTextMarker
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, accessibilityTextMarkerForPosition : position)
    }
    unsafe fn accessibilityTextMarkerRangeForRange_(
        &self,
        range: NSRange,
    ) -> BEAccessibilityTextMarkerRange
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, accessibilityTextMarkerRangeForRange : range)
    }
    unsafe fn accessibilityRangeForTextMarkerRange_(
        &self,
        range: BEAccessibilityTextMarkerRange,
    ) -> NSRange
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, accessibilityRangeForTextMarkerRange : range)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct BEDownloadMonitorLocation(pub id);
impl std::ops::Deref for BEDownloadMonitorLocation {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for BEDownloadMonitorLocation {}
impl BEDownloadMonitorLocation {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"BEDownloadMonitorLocation").unwrap(), alloc) })
    }
}
impl INSObject for BEDownloadMonitorLocation {}
impl PNSObject for BEDownloadMonitorLocation {}
impl std::convert::TryFrom<NSObject> for BEDownloadMonitorLocation {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<BEDownloadMonitorLocation, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"BEDownloadMonitorLocation").unwrap()) };
        if is_kind_of {
            Ok(BEDownloadMonitorLocation(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to BEDownloadMonitorLocation")
        }
    }
}
impl IBEDownloadMonitorLocation for BEDownloadMonitorLocation {}
pub trait IBEDownloadMonitorLocation: Sized + std::ops::Deref {
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
    unsafe fn bookmarkData(&self) -> NSData
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, bookmarkData)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"BEDownloadMonitorLocation").unwrap(), new)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct BEDownloadMonitor(pub id);
impl std::ops::Deref for BEDownloadMonitor {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for BEDownloadMonitor {}
impl BEDownloadMonitor {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"BEDownloadMonitor").unwrap(), alloc) })
    }
}
impl INSObject for BEDownloadMonitor {}
impl PNSObject for BEDownloadMonitor {}
impl std::convert::TryFrom<NSObject> for BEDownloadMonitor {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<BEDownloadMonitor, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"BEDownloadMonitor").unwrap()) };
        if is_kind_of {
            Ok(BEDownloadMonitor(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to BEDownloadMonitor")
        }
    }
}
impl IBEDownloadMonitor for BEDownloadMonitor {}
pub trait IBEDownloadMonitor: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn initWithSourceURL_destinationURL_observedProgress_liveActivityAccessToken_(
        &self,
        sourceURL: NSURL,
        destinationURL: NSURL,
        observedProgress: NSProgress,
        liveActivityAccessToken: NSData,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithSourceURL : sourceURL, destinationURL : destinationURL, observedProgress : observedProgress, liveActivityAccessToken : liveActivityAccessToken)
    }
    unsafe fn useDownloadsFolderWithPlaceholderType_finalFileCreatedHandler_(
        &self,
        type_: UTType,
        finalFileCreatedHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, useDownloadsFolderWithPlaceholderType : type_, finalFileCreatedHandler : finalFileCreatedHandler)
    }
    unsafe fn beginMonitoring_(&self, completion: *mut ::std::os::raw::c_void)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, beginMonitoring : completion)
    }
    unsafe fn resumeMonitoring_completionHandler_(
        &self,
        url: NSURL,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, resumeMonitoring : url, completionHandler : completionHandler)
    }
    unsafe fn identifier(&self) -> NSUUID
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, identifier)
    }
    unsafe fn sourceURL(&self) -> NSURL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, sourceURL)
    }
    unsafe fn destinationURL(&self) -> NSURL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, destinationURL)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"BEDownloadMonitor").unwrap(), new)
    }
    unsafe fn createAccessToken() -> NSData
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"BEDownloadMonitor").unwrap(), createAccessToken)
    }
}

unsafe impl objc2::encode::RefEncode for BELayerHierarchyHandle {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for BELayerHierarchyHandle {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for BELayerHierarchy {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for BELayerHierarchy {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for BELayerHierarchyHostingTransactionCoordinator {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for BELayerHierarchyHostingTransactionCoordinator {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for BEWebContentProcess {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for BEWebContentProcess {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for BENetworkingProcess {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for BENetworkingProcess {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for BERenderingProcess {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for BERenderingProcess {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for BEKeyEntryContext {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for BEKeyEntryContext {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for BETextAlternatives {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for BETextAlternatives {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for BETextSuggestion {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for BETextSuggestion {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for BETextInteraction {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for BETextInteraction {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for BEMediaEnvironment {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for BEMediaEnvironment {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for BEProcessCapability {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for BEProcessCapability {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for BEWebAppManifest {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for BEWebAppManifest {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for BEWebContentFilter {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for BEWebContentFilter {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for BEAccessibilityTextMarker {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for BEAccessibilityTextMarker {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for BEAccessibilityTextMarkerRange {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for BEAccessibilityTextMarkerRange {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for BEDownloadMonitorLocation {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for BEDownloadMonitorLocation {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for BEDownloadMonitor {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for BEDownloadMonitor {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
