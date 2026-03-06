#[allow(unused_imports)]
use crate::objc::*;
#[allow(unused_imports)]
use crate::Foundation::*;
#[allow(unused_imports)]
use crate::Security::*;

#[allow(unused_imports)]
use objc2::msg_send;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct BGTaskRequest(pub id);
impl std::ops::Deref for BGTaskRequest {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for BGTaskRequest {}
impl BGTaskRequest {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"BGTaskRequest").unwrap(), alloc) })
    }
}
impl PNSCopying for BGTaskRequest {}
impl INSObject for BGTaskRequest {}
impl PNSObject for BGTaskRequest {}
impl std::convert::TryFrom<NSObject> for BGTaskRequest {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<BGTaskRequest, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"BGTaskRequest").unwrap()) };
        if is_kind_of {
            Ok(BGTaskRequest(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to BGTaskRequest")
        }
    }
}
impl IBGTaskRequest for BGTaskRequest {}
pub trait IBGTaskRequest: Sized + std::ops::Deref {
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
    unsafe fn earliestBeginDate(&self) -> NSDate
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, earliestBeginDate)
    }
    unsafe fn setEarliestBeginDate_(&self, earliestBeginDate: NSDate)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setEarliestBeginDate : earliestBeginDate)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"BGTaskRequest").unwrap(), new)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct BGAppRefreshTaskRequest(pub id);
impl std::ops::Deref for BGAppRefreshTaskRequest {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for BGAppRefreshTaskRequest {}
impl BGAppRefreshTaskRequest {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"BGAppRefreshTaskRequest").unwrap(), alloc) })
    }
}
impl IBGTaskRequest for BGAppRefreshTaskRequest {}
impl PNSCopying for BGAppRefreshTaskRequest {}
impl From<BGAppRefreshTaskRequest> for BGTaskRequest {
    fn from(child: BGAppRefreshTaskRequest) -> BGTaskRequest {
        BGTaskRequest(child.0)
    }
}
impl std::convert::TryFrom<BGTaskRequest> for BGAppRefreshTaskRequest {
    type Error = &'static str;
    fn try_from(parent: BGTaskRequest) -> Result<BGAppRefreshTaskRequest, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"BGAppRefreshTaskRequest").unwrap()) };
        if is_kind_of {
            Ok(BGAppRefreshTaskRequest(parent.0))
        } else {
            Err("This BGTaskRequest cannot be downcasted to BGAppRefreshTaskRequest")
        }
    }
}
impl INSObject for BGAppRefreshTaskRequest {}
impl PNSObject for BGAppRefreshTaskRequest {}
impl IBGAppRefreshTaskRequest for BGAppRefreshTaskRequest {}
pub trait IBGAppRefreshTaskRequest: Sized + std::ops::Deref {
    unsafe fn initWithIdentifier_(&self, identifier: NSString) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithIdentifier : identifier)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct BGProcessingTaskRequest(pub id);
impl std::ops::Deref for BGProcessingTaskRequest {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for BGProcessingTaskRequest {}
impl BGProcessingTaskRequest {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"BGProcessingTaskRequest").unwrap(), alloc) })
    }
}
impl IBGTaskRequest for BGProcessingTaskRequest {}
impl PNSCopying for BGProcessingTaskRequest {}
impl std::convert::TryFrom<BGTaskRequest> for BGProcessingTaskRequest {
    type Error = &'static str;
    fn try_from(parent: BGTaskRequest) -> Result<BGProcessingTaskRequest, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"BGProcessingTaskRequest").unwrap()) };
        if is_kind_of {
            Ok(BGProcessingTaskRequest(parent.0))
        } else {
            Err("This BGTaskRequest cannot be downcasted to BGProcessingTaskRequest")
        }
    }
}
impl INSObject for BGProcessingTaskRequest {}
impl PNSObject for BGProcessingTaskRequest {}
impl IBGProcessingTaskRequest for BGProcessingTaskRequest {}
pub trait IBGProcessingTaskRequest: Sized + std::ops::Deref {
    unsafe fn initWithIdentifier_(&self, identifier: NSString) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithIdentifier : identifier)
    }
    unsafe fn requiresNetworkConnectivity(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, requiresNetworkConnectivity)
    }
    unsafe fn setRequiresNetworkConnectivity_(&self, requiresNetworkConnectivity: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRequiresNetworkConnectivity : requiresNetworkConnectivity)
    }
    unsafe fn requiresExternalPower(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, requiresExternalPower)
    }
    unsafe fn setRequiresExternalPower_(&self, requiresExternalPower: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRequiresExternalPower : requiresExternalPower)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct BGHealthResearchTaskRequest(pub id);
impl std::ops::Deref for BGHealthResearchTaskRequest {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for BGHealthResearchTaskRequest {}
impl BGHealthResearchTaskRequest {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"BGHealthResearchTaskRequest").unwrap(), alloc) })
    }
}
impl IBGProcessingTaskRequest for BGHealthResearchTaskRequest {}
impl From<BGHealthResearchTaskRequest> for BGProcessingTaskRequest {
    fn from(child: BGHealthResearchTaskRequest) -> BGProcessingTaskRequest {
        BGProcessingTaskRequest(child.0)
    }
}
impl std::convert::TryFrom<BGProcessingTaskRequest> for BGHealthResearchTaskRequest {
    type Error = &'static str;
    fn try_from(
        parent: BGProcessingTaskRequest,
    ) -> Result<BGHealthResearchTaskRequest, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"BGHealthResearchTaskRequest").unwrap()) };
        if is_kind_of {
            Ok(BGHealthResearchTaskRequest(parent.0))
        } else {
            Err("This BGProcessingTaskRequest cannot be downcasted to BGHealthResearchTaskRequest")
        }
    }
}
impl IBGTaskRequest for BGHealthResearchTaskRequest {}
impl PNSCopying for BGHealthResearchTaskRequest {}
impl INSObject for BGHealthResearchTaskRequest {}
impl PNSObject for BGHealthResearchTaskRequest {}
impl IBGHealthResearchTaskRequest for BGHealthResearchTaskRequest {}
pub trait IBGHealthResearchTaskRequest: Sized + std::ops::Deref {
    unsafe fn protectionTypeOfRequiredData(&self) -> NSFileProtectionType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, protectionTypeOfRequiredData)
    }
    unsafe fn setProtectionTypeOfRequiredData_(&self, protectionTypeOfRequiredData: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setProtectionTypeOfRequiredData : protectionTypeOfRequiredData)
    }
}
pub type BGContinuedProcessingTaskRequestSubmissionStrategy = NSInteger;
pub type BGContinuedProcessingTaskRequestResources = NSInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct BGContinuedProcessingTaskRequest(pub id);
impl std::ops::Deref for BGContinuedProcessingTaskRequest {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for BGContinuedProcessingTaskRequest {}
impl BGContinuedProcessingTaskRequest {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"BGContinuedProcessingTaskRequest").unwrap(), alloc) })
    }
}
impl IBGTaskRequest for BGContinuedProcessingTaskRequest {}
impl PNSCopying for BGContinuedProcessingTaskRequest {}
impl std::convert::TryFrom<BGTaskRequest> for BGContinuedProcessingTaskRequest {
    type Error = &'static str;
    fn try_from(parent: BGTaskRequest) -> Result<BGContinuedProcessingTaskRequest, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"BGContinuedProcessingTaskRequest").unwrap())
        };
        if is_kind_of {
            Ok(BGContinuedProcessingTaskRequest(parent.0))
        } else {
            Err("This BGTaskRequest cannot be downcasted to BGContinuedProcessingTaskRequest")
        }
    }
}
impl INSObject for BGContinuedProcessingTaskRequest {}
impl PNSObject for BGContinuedProcessingTaskRequest {}
impl IBGContinuedProcessingTaskRequest for BGContinuedProcessingTaskRequest {}
pub trait IBGContinuedProcessingTaskRequest: Sized + std::ops::Deref {
    unsafe fn initWithIdentifier_title_subtitle_(
        &self,
        identifier: NSString,
        title: NSString,
        subtitle: NSString,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithIdentifier : identifier, title : title, subtitle : subtitle)
    }
    unsafe fn title(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, title)
    }
    unsafe fn setTitle_(&self, title: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setTitle : title)
    }
    unsafe fn subtitle(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, subtitle)
    }
    unsafe fn setSubtitle_(&self, subtitle: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSubtitle : subtitle)
    }
    unsafe fn strategy(&self) -> BGContinuedProcessingTaskRequestSubmissionStrategy
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, strategy)
    }
    unsafe fn setStrategy_(&self, strategy: BGContinuedProcessingTaskRequestSubmissionStrategy)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setStrategy : strategy)
    }
    unsafe fn requiredResources(&self) -> BGContinuedProcessingTaskRequestResources
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, requiredResources)
    }
    unsafe fn setRequiredResources_(
        &self,
        requiredResources: BGContinuedProcessingTaskRequestResources,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRequiredResources : requiredResources)
    }
}
pub type BGTaskSchedulerErrorCode = NSInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct BGTaskScheduler(pub id);
impl std::ops::Deref for BGTaskScheduler {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for BGTaskScheduler {}
impl BGTaskScheduler {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"BGTaskScheduler").unwrap(), alloc) })
    }
}
impl INSObject for BGTaskScheduler {}
impl PNSObject for BGTaskScheduler {}
impl std::convert::TryFrom<NSObject> for BGTaskScheduler {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<BGTaskScheduler, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"BGTaskScheduler").unwrap()) };
        if is_kind_of {
            Ok(BGTaskScheduler(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to BGTaskScheduler")
        }
    }
}
impl IBGTaskScheduler for BGTaskScheduler {}
pub trait IBGTaskScheduler: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn registerForTaskWithIdentifier_usingQueue_launchHandler_(
        &self,
        identifier: NSString,
        queue: NSObject,
        launchHandler: *mut ::std::os::raw::c_void,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, registerForTaskWithIdentifier : identifier, usingQueue : queue, launchHandler : launchHandler)
    }
    unsafe fn submitTaskRequest_error_(
        &self,
        taskRequest: BGTaskRequest,
        error: *mut NSError,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, submitTaskRequest : taskRequest, error : error)
    }
    unsafe fn cancelTaskRequestWithIdentifier_(&self, identifier: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, cancelTaskRequestWithIdentifier : identifier)
    }
    unsafe fn cancelAllTaskRequests(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, cancelAllTaskRequests)
    }
    unsafe fn getPendingTaskRequestsWithCompletionHandler_(
        &self,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, getPendingTaskRequestsWithCompletionHandler : completionHandler)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"BGTaskScheduler").unwrap(), new)
    }
    unsafe fn sharedScheduler() -> BGTaskScheduler
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"BGTaskScheduler").unwrap(), sharedScheduler)
    }
    unsafe fn supportedResources() -> BGContinuedProcessingTaskRequestResources
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"BGTaskScheduler").unwrap(), supportedResources)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct BGTask(pub id);
impl std::ops::Deref for BGTask {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for BGTask {}
impl BGTask {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"BGTask").unwrap(), alloc) })
    }
}
impl INSObject for BGTask {}
impl PNSObject for BGTask {}
impl std::convert::TryFrom<NSObject> for BGTask {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<BGTask, Self::Error> {
        let is_kind_of: bool = unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"BGTask").unwrap()) };
        if is_kind_of {
            Ok(BGTask(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to BGTask")
        }
    }
}
impl IBGTask for BGTask {}
pub trait IBGTask: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn setTaskCompletedWithSuccess_(&self, success: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setTaskCompletedWithSuccess : success)
    }
    unsafe fn identifier(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, identifier)
    }
    unsafe fn expirationHandler(&self) -> *mut ::std::os::raw::c_void
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, expirationHandler)
    }
    unsafe fn setExpirationHandler_(&self, expirationHandler: *mut ::std::os::raw::c_void)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setExpirationHandler : expirationHandler)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"BGTask").unwrap(), new)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct BGProcessingTask(pub id);
impl std::ops::Deref for BGProcessingTask {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for BGProcessingTask {}
impl BGProcessingTask {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"BGProcessingTask").unwrap(), alloc) })
    }
}
impl IBGTask for BGProcessingTask {}
impl From<BGProcessingTask> for BGTask {
    fn from(child: BGProcessingTask) -> BGTask {
        BGTask(child.0)
    }
}
impl std::convert::TryFrom<BGTask> for BGProcessingTask {
    type Error = &'static str;
    fn try_from(parent: BGTask) -> Result<BGProcessingTask, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"BGProcessingTask").unwrap()) };
        if is_kind_of {
            Ok(BGProcessingTask(parent.0))
        } else {
            Err("This BGTask cannot be downcasted to BGProcessingTask")
        }
    }
}
impl INSObject for BGProcessingTask {}
impl PNSObject for BGProcessingTask {}
impl IBGProcessingTask for BGProcessingTask {}
pub trait IBGProcessingTask: Sized + std::ops::Deref {}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct BGHealthResearchTask(pub id);
impl std::ops::Deref for BGHealthResearchTask {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for BGHealthResearchTask {}
impl BGHealthResearchTask {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"BGHealthResearchTask").unwrap(), alloc) })
    }
}
impl IBGProcessingTask for BGHealthResearchTask {}
impl From<BGHealthResearchTask> for BGProcessingTask {
    fn from(child: BGHealthResearchTask) -> BGProcessingTask {
        BGProcessingTask(child.0)
    }
}
impl std::convert::TryFrom<BGProcessingTask> for BGHealthResearchTask {
    type Error = &'static str;
    fn try_from(parent: BGProcessingTask) -> Result<BGHealthResearchTask, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"BGHealthResearchTask").unwrap()) };
        if is_kind_of {
            Ok(BGHealthResearchTask(parent.0))
        } else {
            Err("This BGProcessingTask cannot be downcasted to BGHealthResearchTask")
        }
    }
}
impl IBGTask for BGHealthResearchTask {}
impl INSObject for BGHealthResearchTask {}
impl PNSObject for BGHealthResearchTask {}
impl IBGHealthResearchTask for BGHealthResearchTask {}
pub trait IBGHealthResearchTask: Sized + std::ops::Deref {}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct BGAppRefreshTask(pub id);
impl std::ops::Deref for BGAppRefreshTask {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for BGAppRefreshTask {}
impl BGAppRefreshTask {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"BGAppRefreshTask").unwrap(), alloc) })
    }
}
impl IBGTask for BGAppRefreshTask {}
impl std::convert::TryFrom<BGTask> for BGAppRefreshTask {
    type Error = &'static str;
    fn try_from(parent: BGTask) -> Result<BGAppRefreshTask, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"BGAppRefreshTask").unwrap()) };
        if is_kind_of {
            Ok(BGAppRefreshTask(parent.0))
        } else {
            Err("This BGTask cannot be downcasted to BGAppRefreshTask")
        }
    }
}
impl INSObject for BGAppRefreshTask {}
impl PNSObject for BGAppRefreshTask {}
impl IBGAppRefreshTask for BGAppRefreshTask {}
pub trait IBGAppRefreshTask: Sized + std::ops::Deref {}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct BGContinuedProcessingTask(pub id);
impl std::ops::Deref for BGContinuedProcessingTask {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for BGContinuedProcessingTask {}
impl BGContinuedProcessingTask {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"BGContinuedProcessingTask").unwrap(), alloc) })
    }
}
impl PNSProgressReporting for BGContinuedProcessingTask {}
impl IBGTask for BGContinuedProcessingTask {}
impl std::convert::TryFrom<BGTask> for BGContinuedProcessingTask {
    type Error = &'static str;
    fn try_from(parent: BGTask) -> Result<BGContinuedProcessingTask, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"BGContinuedProcessingTask").unwrap()) };
        if is_kind_of {
            Ok(BGContinuedProcessingTask(parent.0))
        } else {
            Err("This BGTask cannot be downcasted to BGContinuedProcessingTask")
        }
    }
}
impl INSObject for BGContinuedProcessingTask {}
impl PNSObject for BGContinuedProcessingTask {}
impl IBGContinuedProcessingTask for BGContinuedProcessingTask {}
pub trait IBGContinuedProcessingTask: Sized + std::ops::Deref {
    unsafe fn updateTitle_subtitle_(&self, title: NSString, subtitle: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, updateTitle : title, subtitle : subtitle)
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
}
unsafe extern "C" {
    pub static BGTaskSchedulerErrorDomain: NSErrorDomain;
}

unsafe impl objc2::encode::RefEncode for BGTaskRequest {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for BGTaskRequest {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for BGAppRefreshTaskRequest {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for BGAppRefreshTaskRequest {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for BGProcessingTaskRequest {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for BGProcessingTaskRequest {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for BGHealthResearchTaskRequest {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for BGHealthResearchTaskRequest {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for BGContinuedProcessingTaskRequest {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for BGContinuedProcessingTaskRequest {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for BGTaskScheduler {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for BGTaskScheduler {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for BGTask {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for BGTask {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for BGProcessingTask {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for BGProcessingTask {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for BGHealthResearchTask {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for BGHealthResearchTask {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for BGAppRefreshTask {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for BGAppRefreshTask {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for BGContinuedProcessingTask {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for BGContinuedProcessingTask {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
