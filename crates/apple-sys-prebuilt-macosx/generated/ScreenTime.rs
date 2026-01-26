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
pub struct STScreenTimeConfiguration(pub id);
impl std::ops::Deref for STScreenTimeConfiguration {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for STScreenTimeConfiguration {}
impl STScreenTimeConfiguration {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"STScreenTimeConfiguration").unwrap(), alloc) })
    }
}
impl INSObject for STScreenTimeConfiguration {}
impl PNSObject for STScreenTimeConfiguration {}
impl std::convert::TryFrom<NSObject> for STScreenTimeConfiguration {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<STScreenTimeConfiguration, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"STScreenTimeConfiguration").unwrap()) };
        if is_kind_of {
            Ok(STScreenTimeConfiguration(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to STScreenTimeConfiguration")
        }
    }
}
impl ISTScreenTimeConfiguration for STScreenTimeConfiguration {}
pub trait ISTScreenTimeConfiguration: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn enforcesChildRestrictions(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, enforcesChildRestrictions)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"STScreenTimeConfiguration").unwrap(), new)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct STScreenTimeConfigurationObserver(pub id);
impl std::ops::Deref for STScreenTimeConfigurationObserver {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for STScreenTimeConfigurationObserver {}
impl STScreenTimeConfigurationObserver {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"STScreenTimeConfigurationObserver").unwrap(), alloc) })
    }
}
impl INSObject for STScreenTimeConfigurationObserver {}
impl PNSObject for STScreenTimeConfigurationObserver {}
impl std::convert::TryFrom<NSObject> for STScreenTimeConfigurationObserver {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<STScreenTimeConfigurationObserver, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"STScreenTimeConfigurationObserver").unwrap())
        };
        if is_kind_of {
            Ok(STScreenTimeConfigurationObserver(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to STScreenTimeConfigurationObserver")
        }
    }
}
impl ISTScreenTimeConfigurationObserver for STScreenTimeConfigurationObserver {}
pub trait ISTScreenTimeConfigurationObserver: Sized + std::ops::Deref {
    unsafe fn initWithUpdateQueue_(&self, updateQueue: NSObject) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithUpdateQueue : updateQueue)
    }
    unsafe fn startObserving(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, startObserving)
    }
    unsafe fn stopObserving(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, stopObserving)
    }
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn configuration(&self) -> STScreenTimeConfiguration
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, configuration)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"STScreenTimeConfigurationObserver").unwrap(), new)
    }
}
pub type STWebHistoryProfileIdentifier = NSString;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct STWebHistory(pub id);
impl std::ops::Deref for STWebHistory {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for STWebHistory {}
impl STWebHistory {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"STWebHistory").unwrap(), alloc) })
    }
}
impl INSObject for STWebHistory {}
impl PNSObject for STWebHistory {}
impl std::convert::TryFrom<NSObject> for STWebHistory {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<STWebHistory, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"STWebHistory").unwrap()) };
        if is_kind_of {
            Ok(STWebHistory(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to STWebHistory")
        }
    }
}
impl ISTWebHistory for STWebHistory {}
pub trait ISTWebHistory: Sized + std::ops::Deref {
    unsafe fn initWithBundleIdentifier_profileIdentifier_error_(
        &self,
        bundleIdentifier: NSString,
        profileIdentifier: NSString,
        error: *mut NSError,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithBundleIdentifier : bundleIdentifier, profileIdentifier : profileIdentifier, error : error)
    }
    unsafe fn initWithProfileIdentifier_(&self, profileIdentifier: NSString) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithProfileIdentifier : profileIdentifier)
    }
    unsafe fn initWithBundleIdentifier_error_(
        &self,
        bundleIdentifier: NSString,
        error: *mut NSError,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithBundleIdentifier : bundleIdentifier, error : error)
    }
    unsafe fn fetchHistoryDuringInterval_completionHandler_(
        &self,
        interval: NSDateInterval,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, fetchHistoryDuringInterval : interval, completionHandler : completionHandler)
    }
    unsafe fn fetchAllHistoryWithCompletionHandler_(
        &self,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, fetchAllHistoryWithCompletionHandler : completionHandler)
    }
    unsafe fn deleteHistoryForURL_(&self, url: NSURL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, deleteHistoryForURL : url)
    }
    unsafe fn deleteHistoryDuringInterval_(&self, interval: NSDateInterval)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, deleteHistoryDuringInterval : interval)
    }
    unsafe fn deleteAllHistory(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, deleteAllHistory)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct STWebpageController(pub id);
impl std::ops::Deref for STWebpageController {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for STWebpageController {}
impl STWebpageController {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"STWebpageController").unwrap(), alloc) })
    }
}
impl PNSCoding for STWebpageController {}
impl INSObject for STWebpageController {}
impl PNSObject for STWebpageController {}
impl std::convert::TryFrom<NSObject> for STWebpageController {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<STWebpageController, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"STWebpageController").unwrap()) };
        if is_kind_of {
            Ok(STWebpageController(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to STWebpageController")
        }
    }
}
impl ISTWebpageController for STWebpageController {}
pub trait ISTWebpageController: Sized + std::ops::Deref {
    unsafe fn setBundleIdentifier_error_(
        &self,
        bundleIdentifier: NSString,
        error: *mut NSError,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setBundleIdentifier : bundleIdentifier, error : error)
    }
    unsafe fn initWithNibName_bundle_(
        &self,
        nibNameOrNil: NSString,
        nibBundleOrNil: NSBundle,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithNibName : nibNameOrNil, bundle : nibBundleOrNil)
    }
    unsafe fn initWithCoder_(&self, aDecoder: NSCoder) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithCoder : aDecoder)
    }
    unsafe fn suppressUsageRecording(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, suppressUsageRecording)
    }
    unsafe fn setSuppressUsageRecording_(&self, suppressUsageRecording: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSuppressUsageRecording : suppressUsageRecording)
    }
    unsafe fn URL(&self) -> NSURL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, URL)
    }
    unsafe fn setURL_(&self, URL: NSURL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setURL : URL)
    }
    unsafe fn URLIsPlayingVideo(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, URLIsPlayingVideo)
    }
    unsafe fn setURLIsPlayingVideo_(&self, URLIsPlayingVideo: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setURLIsPlayingVideo : URLIsPlayingVideo)
    }
    unsafe fn URLIsPictureInPicture(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, URLIsPictureInPicture)
    }
    unsafe fn setURLIsPictureInPicture_(&self, URLIsPictureInPicture: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setURLIsPictureInPicture : URLIsPictureInPicture)
    }
    unsafe fn URLIsBlocked(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, URLIsBlocked)
    }
    unsafe fn profileIdentifier(&self) -> STWebHistoryProfileIdentifier
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, profileIdentifier)
    }
    unsafe fn setProfileIdentifier_(&self, profileIdentifier: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setProfileIdentifier : profileIdentifier)
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
impl PNSEditor for STWebpageController {}

unsafe impl objc2::encode::RefEncode for STScreenTimeConfiguration {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for STScreenTimeConfiguration {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for STScreenTimeConfigurationObserver {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for STScreenTimeConfigurationObserver {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for STWebHistory {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for STWebHistory {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for STWebpageController {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for STWebpageController {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
