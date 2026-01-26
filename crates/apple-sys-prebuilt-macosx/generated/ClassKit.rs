#[allow(unused_imports)]
use crate::objc::*;
#[allow(unused_imports)]
use crate::CoreFoundation::*;
#[allow(unused_imports)]
use crate::CoreGraphics::*;
#[allow(unused_imports)]
use crate::Foundation::*;

#[allow(unused_imports)]
use objc2::msg_send;
pub type CLSErrorCode = NSInteger;
pub type CLSErrorUserInfoKey = NSString;
pub type CLSPredicateKeyPath = NSString;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CLSObject(pub id);
impl std::ops::Deref for CLSObject {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CLSObject {}
impl CLSObject {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CLSObject").unwrap(), alloc) })
    }
}
impl PNSSecureCoding for CLSObject {}
impl INSObject for CLSObject {}
impl PNSObject for CLSObject {}
impl std::convert::TryFrom<NSObject> for CLSObject {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<CLSObject, Self::Error> {
        let is_kind_of: bool = unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CLSObject").unwrap()) };
        if is_kind_of {
            Ok(CLSObject(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to CLSObject")
        }
    }
}
impl ICLSObject for CLSObject {}
pub trait ICLSObject: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn dateCreated(&self) -> NSDate
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, dateCreated)
    }
    unsafe fn dateLastModified(&self) -> NSDate
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, dateLastModified)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CLSObject").unwrap(), new)
    }
}
pub type CLSProgressReportingCapabilityKind = NSInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CLSProgressReportingCapability(pub id);
impl std::ops::Deref for CLSProgressReportingCapability {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CLSProgressReportingCapability {}
impl CLSProgressReportingCapability {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CLSProgressReportingCapability").unwrap(), alloc) })
    }
}
impl ICLSObject for CLSProgressReportingCapability {}
impl PNSSecureCoding for CLSProgressReportingCapability {}
impl From<CLSProgressReportingCapability> for CLSObject {
    fn from(child: CLSProgressReportingCapability) -> CLSObject {
        CLSObject(child.0)
    }
}
impl std::convert::TryFrom<CLSObject> for CLSProgressReportingCapability {
    type Error = &'static str;
    fn try_from(parent: CLSObject) -> Result<CLSProgressReportingCapability, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CLSProgressReportingCapability").unwrap())
        };
        if is_kind_of {
            Ok(CLSProgressReportingCapability(parent.0))
        } else {
            Err("This CLSObject cannot be downcasted to CLSProgressReportingCapability")
        }
    }
}
impl INSObject for CLSProgressReportingCapability {}
impl PNSObject for CLSProgressReportingCapability {}
impl ICLSProgressReportingCapability for CLSProgressReportingCapability {}
pub trait ICLSProgressReportingCapability: Sized + std::ops::Deref {
    unsafe fn initWithKind_details_(
        &self,
        kind: CLSProgressReportingCapabilityKind,
        details: NSString,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithKind : kind, details : details)
    }
    unsafe fn kind(&self) -> CLSProgressReportingCapabilityKind
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, kind)
    }
    unsafe fn details(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, details)
    }
}
pub type CLSContextType = NSInteger;
pub type CLSContextTopic = NSString;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CLSContext(pub id);
impl std::ops::Deref for CLSContext {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CLSContext {}
impl CLSContext {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CLSContext").unwrap(), alloc) })
    }
}
impl ICLSObject for CLSContext {}
impl PNSSecureCoding for CLSContext {}
impl std::convert::TryFrom<CLSObject> for CLSContext {
    type Error = &'static str;
    fn try_from(parent: CLSObject) -> Result<CLSContext, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CLSContext").unwrap()) };
        if is_kind_of {
            Ok(CLSContext(parent.0))
        } else {
            Err("This CLSObject cannot be downcasted to CLSContext")
        }
    }
}
impl INSObject for CLSContext {}
impl PNSObject for CLSContext {}
impl ICLSContext for CLSContext {}
pub trait ICLSContext: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn initWithType_identifier_title_(
        &self,
        type_: CLSContextType,
        identifier: NSString,
        title: NSString,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithType : type_, identifier : identifier, title : title)
    }
    unsafe fn becomeActive(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, becomeActive)
    }
    unsafe fn resignActive(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, resignActive)
    }
    unsafe fn setType_(&self, type_: CLSContextType)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setType : type_)
    }
    unsafe fn addProgressReportingCapabilities_(&self, capabilities: NSSet)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, addProgressReportingCapabilities : capabilities)
    }
    unsafe fn resetProgressReportingCapabilities(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, resetProgressReportingCapabilities)
    }
    unsafe fn identifierPath(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, identifierPath)
    }
    unsafe fn identifier(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, identifier)
    }
    unsafe fn universalLinkURL(&self) -> NSURL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, universalLinkURL)
    }
    unsafe fn setUniversalLinkURL_(&self, universalLinkURL: NSURL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setUniversalLinkURL : universalLinkURL)
    }
    unsafe fn type_(&self) -> CLSContextType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, type)
    }
    unsafe fn customTypeName(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, customTypeName)
    }
    unsafe fn setCustomTypeName_(&self, customTypeName: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCustomTypeName : customTypeName)
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
    unsafe fn displayOrder(&self) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, displayOrder)
    }
    unsafe fn setDisplayOrder_(&self, displayOrder: NSInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDisplayOrder : displayOrder)
    }
    unsafe fn topic(&self) -> CLSContextTopic
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, topic)
    }
    unsafe fn setTopic_(&self, topic: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setTopic : topic)
    }
    unsafe fn isAssignable(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isAssignable)
    }
    unsafe fn setAssignable_(&self, assignable: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAssignable : assignable)
    }
    unsafe fn suggestedAge(&self) -> NSRange
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, suggestedAge)
    }
    unsafe fn setSuggestedAge_(&self, suggestedAge: NSRange)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSuggestedAge : suggestedAge)
    }
    unsafe fn suggestedCompletionTime(&self) -> NSRange
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, suggestedCompletionTime)
    }
    unsafe fn setSuggestedCompletionTime_(&self, suggestedCompletionTime: NSRange)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSuggestedCompletionTime : suggestedCompletionTime)
    }
    unsafe fn progressReportingCapabilities(&self) -> NSSet
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, progressReportingCapabilities)
    }
    unsafe fn summary(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, summary)
    }
    unsafe fn setSummary_(&self, summary: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSummary : summary)
    }
    unsafe fn thumbnail(&self) -> CGImageRef
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, thumbnail)
    }
    unsafe fn setThumbnail_(&self, thumbnail: CGImageRef)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setThumbnail : thumbnail)
    }
    unsafe fn isActive(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isActive)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CLSContext").unwrap(), new)
    }
}
impl CLSContext_Hierarchy for CLSContext {}
pub trait CLSContext_Hierarchy: Sized + std::ops::Deref {
    unsafe fn removeFromParent(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, removeFromParent)
    }
    unsafe fn addChildContext_(&self, child: CLSContext)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, addChildContext : child)
    }
    unsafe fn descendantMatchingIdentifierPath_completion_(
        &self,
        identifierPath: NSArray,
        completion: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, descendantMatchingIdentifierPath : identifierPath, completion : completion)
    }
    unsafe fn addNavigationChildContext_(&self, child: CLSContext)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, addNavigationChildContext : child)
    }
    unsafe fn removeNavigationChildContext_(&self, child: CLSContext)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, removeNavigationChildContext : child)
    }
    unsafe fn parent(&self) -> CLSContext
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, parent)
    }
    unsafe fn navigationChildContexts(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, navigationChildContexts)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CLSActivityItem(pub id);
impl std::ops::Deref for CLSActivityItem {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CLSActivityItem {}
impl CLSActivityItem {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CLSActivityItem").unwrap(), alloc) })
    }
}
impl ICLSObject for CLSActivityItem {}
impl PNSSecureCoding for CLSActivityItem {}
impl std::convert::TryFrom<CLSObject> for CLSActivityItem {
    type Error = &'static str;
    fn try_from(parent: CLSObject) -> Result<CLSActivityItem, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CLSActivityItem").unwrap()) };
        if is_kind_of {
            Ok(CLSActivityItem(parent.0))
        } else {
            Err("This CLSObject cannot be downcasted to CLSActivityItem")
        }
    }
}
impl INSObject for CLSActivityItem {}
impl PNSObject for CLSActivityItem {}
impl ICLSActivityItem for CLSActivityItem {}
pub trait ICLSActivityItem: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
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
        msg_send!(&*objc2::runtime::AnyClass::get(c"CLSActivityItem").unwrap(), new)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CLSActivity(pub id);
impl std::ops::Deref for CLSActivity {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CLSActivity {}
impl CLSActivity {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CLSActivity").unwrap(), alloc) })
    }
}
impl ICLSObject for CLSActivity {}
impl PNSSecureCoding for CLSActivity {}
impl std::convert::TryFrom<CLSObject> for CLSActivity {
    type Error = &'static str;
    fn try_from(parent: CLSObject) -> Result<CLSActivity, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CLSActivity").unwrap()) };
        if is_kind_of {
            Ok(CLSActivity(parent.0))
        } else {
            Err("This CLSObject cannot be downcasted to CLSActivity")
        }
    }
}
impl INSObject for CLSActivity {}
impl PNSObject for CLSActivity {}
impl ICLSActivity for CLSActivity {}
pub trait ICLSActivity: Sized + std::ops::Deref {
    unsafe fn addProgressRangeFromStart_toEnd_(&self, start: f64, end: f64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, addProgressRangeFromStart : start, toEnd : end)
    }
    unsafe fn addAdditionalActivityItem_(&self, activityItem: CLSActivityItem)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, addAdditionalActivityItem : activityItem)
    }
    unsafe fn progress(&self) -> f64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, progress)
    }
    unsafe fn setProgress_(&self, progress: f64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setProgress : progress)
    }
    unsafe fn duration(&self) -> NSTimeInterval
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, duration)
    }
    unsafe fn primaryActivityItem(&self) -> CLSActivityItem
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, primaryActivityItem)
    }
    unsafe fn setPrimaryActivityItem_(&self, primaryActivityItem: CLSActivityItem)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPrimaryActivityItem : primaryActivityItem)
    }
    unsafe fn additionalActivityItems(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, additionalActivityItems)
    }
}
impl CLSActivity_Activation for CLSActivity {}
pub trait CLSActivity_Activation: Sized + std::ops::Deref {
    unsafe fn start(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, start)
    }
    unsafe fn stop(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, stop)
    }
    unsafe fn removeAllActivityItems(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, removeAllActivityItems)
    }
    unsafe fn isStarted(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isStarted)
    }
}
impl CLSContext_Activity for CLSContext {}
pub trait CLSContext_Activity: Sized + std::ops::Deref {
    unsafe fn createNewActivity(&self) -> CLSActivity
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, createNewActivity)
    }
    unsafe fn currentActivity(&self) -> CLSActivity
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, currentActivity)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CLSScoreItem(pub id);
impl std::ops::Deref for CLSScoreItem {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CLSScoreItem {}
impl CLSScoreItem {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CLSScoreItem").unwrap(), alloc) })
    }
}
impl ICLSActivityItem for CLSScoreItem {}
impl From<CLSScoreItem> for CLSActivityItem {
    fn from(child: CLSScoreItem) -> CLSActivityItem {
        CLSActivityItem(child.0)
    }
}
impl std::convert::TryFrom<CLSActivityItem> for CLSScoreItem {
    type Error = &'static str;
    fn try_from(parent: CLSActivityItem) -> Result<CLSScoreItem, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CLSScoreItem").unwrap()) };
        if is_kind_of {
            Ok(CLSScoreItem(parent.0))
        } else {
            Err("This CLSActivityItem cannot be downcasted to CLSScoreItem")
        }
    }
}
impl ICLSObject for CLSScoreItem {}
impl PNSSecureCoding for CLSScoreItem {}
impl INSObject for CLSScoreItem {}
impl PNSObject for CLSScoreItem {}
impl ICLSScoreItem for CLSScoreItem {}
pub trait ICLSScoreItem: Sized + std::ops::Deref {
    unsafe fn initWithIdentifier_title_score_maxScore_(
        &self,
        identifier: NSString,
        title: NSString,
        score: f64,
        maxScore: f64,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithIdentifier : identifier, title : title, score : score, maxScore : maxScore)
    }
    unsafe fn score(&self) -> f64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, score)
    }
    unsafe fn setScore_(&self, score: f64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setScore : score)
    }
    unsafe fn maxScore(&self) -> f64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, maxScore)
    }
    unsafe fn setMaxScore_(&self, maxScore: f64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMaxScore : maxScore)
    }
}
pub type CLSBinaryValueType = NSInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CLSBinaryItem(pub id);
impl std::ops::Deref for CLSBinaryItem {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CLSBinaryItem {}
impl CLSBinaryItem {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CLSBinaryItem").unwrap(), alloc) })
    }
}
impl ICLSActivityItem for CLSBinaryItem {}
impl std::convert::TryFrom<CLSActivityItem> for CLSBinaryItem {
    type Error = &'static str;
    fn try_from(parent: CLSActivityItem) -> Result<CLSBinaryItem, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CLSBinaryItem").unwrap()) };
        if is_kind_of {
            Ok(CLSBinaryItem(parent.0))
        } else {
            Err("This CLSActivityItem cannot be downcasted to CLSBinaryItem")
        }
    }
}
impl ICLSObject for CLSBinaryItem {}
impl PNSSecureCoding for CLSBinaryItem {}
impl INSObject for CLSBinaryItem {}
impl PNSObject for CLSBinaryItem {}
impl ICLSBinaryItem for CLSBinaryItem {}
pub trait ICLSBinaryItem: Sized + std::ops::Deref {
    unsafe fn initWithIdentifier_title_type_(
        &self,
        identifier: NSString,
        title: NSString,
        valueType: CLSBinaryValueType,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithIdentifier : identifier, title : title, r#type : valueType)
    }
    unsafe fn value(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, value)
    }
    unsafe fn setValue_(&self, value: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setValue : value)
    }
    unsafe fn valueType(&self) -> CLSBinaryValueType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, valueType)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CLSQuantityItem(pub id);
impl std::ops::Deref for CLSQuantityItem {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CLSQuantityItem {}
impl CLSQuantityItem {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CLSQuantityItem").unwrap(), alloc) })
    }
}
impl ICLSActivityItem for CLSQuantityItem {}
impl std::convert::TryFrom<CLSActivityItem> for CLSQuantityItem {
    type Error = &'static str;
    fn try_from(parent: CLSActivityItem) -> Result<CLSQuantityItem, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CLSQuantityItem").unwrap()) };
        if is_kind_of {
            Ok(CLSQuantityItem(parent.0))
        } else {
            Err("This CLSActivityItem cannot be downcasted to CLSQuantityItem")
        }
    }
}
impl ICLSObject for CLSQuantityItem {}
impl PNSSecureCoding for CLSQuantityItem {}
impl INSObject for CLSQuantityItem {}
impl PNSObject for CLSQuantityItem {}
impl ICLSQuantityItem for CLSQuantityItem {}
pub trait ICLSQuantityItem: Sized + std::ops::Deref {
    unsafe fn initWithIdentifier_title_(
        &self,
        identifier: NSString,
        title: NSString,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithIdentifier : identifier, title : title)
    }
    unsafe fn quantity(&self) -> f64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, quantity)
    }
    unsafe fn setQuantity_(&self, quantity: f64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setQuantity : quantity)
    }
}
pub trait PCLSDataStoreDelegate: Sized + std::ops::Deref {
    unsafe fn createContextForIdentifier_parentContext_parentIdentifierPath_(
        &self,
        identifier: NSString,
        parentContext: CLSContext,
        parentIdentifierPath: NSArray,
    ) -> CLSContext
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, createContextForIdentifier : identifier, parentContext : parentContext, parentIdentifierPath : parentIdentifierPath)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CLSDataStore(pub id);
impl std::ops::Deref for CLSDataStore {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CLSDataStore {}
impl CLSDataStore {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CLSDataStore").unwrap(), alloc) })
    }
}
impl INSObject for CLSDataStore {}
impl PNSObject for CLSDataStore {}
impl std::convert::TryFrom<NSObject> for CLSDataStore {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<CLSDataStore, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CLSDataStore").unwrap()) };
        if is_kind_of {
            Ok(CLSDataStore(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to CLSDataStore")
        }
    }
}
impl ICLSDataStore for CLSDataStore {}
pub trait ICLSDataStore: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn saveWithCompletion_(&self, completion: *mut ::std::os::raw::c_void)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, saveWithCompletion : completion)
    }
    unsafe fn completeAllAssignedActivitiesMatching_(&self, contextPath: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, completeAllAssignedActivitiesMatching : contextPath)
    }
    unsafe fn mainAppContext(&self) -> CLSContext
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, mainAppContext)
    }
    unsafe fn activeContext(&self) -> CLSContext
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, activeContext)
    }
    unsafe fn runningActivity(&self) -> CLSActivity
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, runningActivity)
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
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CLSDataStore").unwrap(), new)
    }
    unsafe fn shared() -> CLSDataStore
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CLSDataStore").unwrap(), shared)
    }
}
impl CLSDataStore_Contexts for CLSDataStore {}
pub trait CLSDataStore_Contexts: Sized + std::ops::Deref {
    unsafe fn contextsMatchingPredicate_completion_(
        &self,
        predicate: NSPredicate,
        completion: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, contextsMatchingPredicate : predicate, completion : completion)
    }
    unsafe fn contextsMatchingIdentifierPath_completion_(
        &self,
        identifierPath: NSArray,
        completion: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, contextsMatchingIdentifierPath : identifierPath, completion : completion)
    }
    unsafe fn removeContext_(&self, context: CLSContext)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, removeContext : context)
    }
    unsafe fn fetchActivityForURL_completion_(
        &self,
        url: NSURL,
        completion: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, fetchActivityForURL : url, completion : completion)
    }
}
pub trait NSUserActivity_CLSDeepLinks: Sized + std::ops::Deref {
    unsafe fn isClassKitDeepLink(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isClassKitDeepLink)
    }
    unsafe fn contextIdentifierPath(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, contextIdentifierPath)
    }
}
pub trait PCLSContextProvider: Sized + std::ops::Deref {
    unsafe fn updateDescendantsOfContext_completion_(
        &self,
        context: CLSContext,
        completion: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, updateDescendantsOfContext : context, completion : completion)
    }
}
unsafe extern "C" {
    pub static CLSErrorCodeDomain: NSString;
}
unsafe extern "C" {
    pub static CLSErrorObjectKey: CLSErrorUserInfoKey;
}
unsafe extern "C" {
    pub static CLSErrorUnderlyingErrorsKey: CLSErrorUserInfoKey;
}
unsafe extern "C" {
    pub static CLSErrorSuccessfulObjectsKey: CLSErrorUserInfoKey;
}
unsafe extern "C" {
    pub static CLSPredicateKeyPathDateCreated: CLSPredicateKeyPath;
}
unsafe extern "C" {
    pub static CLSPredicateKeyPathIdentifier: CLSPredicateKeyPath;
}
unsafe extern "C" {
    pub static CLSPredicateKeyPathTitle: CLSPredicateKeyPath;
}
unsafe extern "C" {
    pub static CLSPredicateKeyPathUniversalLinkURL: CLSPredicateKeyPath;
}
unsafe extern "C" {
    pub static CLSPredicateKeyPathTopic: CLSPredicateKeyPath;
}
unsafe extern "C" {
    pub static CLSPredicateKeyPathParent: CLSPredicateKeyPath;
}
unsafe extern "C" {
    pub static CLSContextTopicMath: CLSContextTopic;
}
unsafe extern "C" {
    pub static CLSContextTopicScience: CLSContextTopic;
}
unsafe extern "C" {
    pub static CLSContextTopicLiteracyAndWriting: CLSContextTopic;
}
unsafe extern "C" {
    pub static CLSContextTopicWorldLanguage: CLSContextTopic;
}
unsafe extern "C" {
    pub static CLSContextTopicSocialScience: CLSContextTopic;
}
unsafe extern "C" {
    pub static CLSContextTopicComputerScienceAndEngineering: CLSContextTopic;
}
unsafe extern "C" {
    pub static CLSContextTopicArtsAndMusic: CLSContextTopic;
}
unsafe extern "C" {
    pub static CLSContextTopicHealthAndFitness: CLSContextTopic;
}

unsafe impl objc2::encode::RefEncode for CLSObject {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CLSObject {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CLSProgressReportingCapability {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CLSProgressReportingCapability {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CLSContext {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CLSContext {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CLSActivityItem {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CLSActivityItem {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CLSActivity {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CLSActivity {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CLSScoreItem {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CLSScoreItem {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CLSBinaryItem {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CLSBinaryItem {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CLSQuantityItem {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CLSQuantityItem {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CLSDataStore {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CLSDataStore {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
