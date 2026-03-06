#[allow(unused_imports)]
use crate::objc::*;
#[allow(unused_imports)]
use crate::Foundation::*;
#[allow(unused_imports)]
use crate::Intents::*;
#[allow(unused_imports)]
use crate::Security::*;
#[allow(unused_imports)]
use crate::UIKit::*;

#[allow(unused_imports)]
use objc2::msg_send;
pub type CLKComplicationFamily = NSInteger;
pub type CLKComplicationTimeTravelDirections = NSUInteger;
pub type CLKComplicationPrivacyBehavior = NSUInteger;
pub type CLKComplicationTimelineAnimationBehavior = NSUInteger;
pub type CLKComplicationColumnAlignment = NSInteger;
pub type CLKComplicationRingStyle = NSInteger;
pub type CLKGaugeProviderStyle = NSInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CLKComplication(pub id);
impl std::ops::Deref for CLKComplication {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CLKComplication {}
impl CLKComplication {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CLKComplication").unwrap(), alloc) })
    }
}
impl PNSCopying for CLKComplication {}
impl INSObject for CLKComplication {}
impl PNSObject for CLKComplication {}
impl std::convert::TryFrom<NSObject> for CLKComplication {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<CLKComplication, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CLKComplication").unwrap()) };
        if is_kind_of {
            Ok(CLKComplication(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to CLKComplication")
        }
    }
}
impl ICLKComplication for CLKComplication {}
pub trait ICLKComplication: Sized + std::ops::Deref {
    unsafe fn family(&self) -> CLKComplicationFamily
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, family)
    }
    unsafe fn identifier(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, identifier)
    }
    unsafe fn userInfo(&self) -> NSDictionary
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, userInfo)
    }
    unsafe fn userActivity(&self) -> NSUserActivity
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, userActivity)
    }
}
pub trait PCLKComplicationDataSource: Sized + std::ops::Deref {
    unsafe fn getTimelineEndDateForComplication_withHandler_(
        &self,
        complication: CLKComplication,
        handler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, getTimelineEndDateForComplication : complication, withHandler : handler)
    }
    unsafe fn getPrivacyBehaviorForComplication_withHandler_(
        &self,
        complication: CLKComplication,
        handler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, getPrivacyBehaviorForComplication : complication, withHandler : handler)
    }
    unsafe fn getTimelineAnimationBehaviorForComplication_withHandler_(
        &self,
        complication: CLKComplication,
        handler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, getTimelineAnimationBehaviorForComplication : complication, withHandler : handler)
    }
    unsafe fn getAlwaysOnTemplateForComplication_withHandler_(
        &self,
        complication: CLKComplication,
        handler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, getAlwaysOnTemplateForComplication : complication, withHandler : handler)
    }
    unsafe fn getCurrentTimelineEntryForComplication_withHandler_(
        &self,
        complication: CLKComplication,
        handler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, getCurrentTimelineEntryForComplication : complication, withHandler : handler)
    }
    unsafe fn getTimelineEntriesForComplication_afterDate_limit_withHandler_(
        &self,
        complication: CLKComplication,
        date: NSDate,
        limit: NSUInteger,
        handler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, getTimelineEntriesForComplication : complication, afterDate : date, limit : limit, withHandler : handler)
    }
    unsafe fn getLocalizableSampleTemplateForComplication_withHandler_(
        &self,
        complication: CLKComplication,
        handler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, getLocalizableSampleTemplateForComplication : complication, withHandler : handler)
    }
    unsafe fn getComplicationDescriptorsWithHandler_(&self, handler: *mut ::std::os::raw::c_void)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, getComplicationDescriptorsWithHandler : handler)
    }
    unsafe fn handleSharedComplicationDescriptors_(&self, complicationDescriptors: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, handleSharedComplicationDescriptors : complicationDescriptors)
    }
    unsafe fn getPlaceholderTemplateForComplication_withHandler_(
        &self,
        complication: CLKComplication,
        handler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, getPlaceholderTemplateForComplication : complication, withHandler : handler)
    }
    unsafe fn getSupportedTimeTravelDirectionsForComplication_withHandler_(
        &self,
        complication: CLKComplication,
        handler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, getSupportedTimeTravelDirectionsForComplication : complication, withHandler : handler)
    }
    unsafe fn getTimelineStartDateForComplication_withHandler_(
        &self,
        complication: CLKComplication,
        handler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, getTimelineStartDateForComplication : complication, withHandler : handler)
    }
    unsafe fn getTimelineEntriesForComplication_beforeDate_limit_withHandler_(
        &self,
        complication: CLKComplication,
        date: NSDate,
        limit: NSUInteger,
        handler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, getTimelineEntriesForComplication : complication, beforeDate : date, limit : limit, withHandler : handler)
    }
    unsafe fn getNextRequestedUpdateDateWithHandler_(&self, handler: *mut ::std::os::raw::c_void)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, getNextRequestedUpdateDateWithHandler : handler)
    }
    unsafe fn requestedUpdateDidBegin(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, requestedUpdateDidBegin)
    }
    unsafe fn requestedUpdateBudgetExhausted(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, requestedUpdateBudgetExhausted)
    }
    unsafe fn widgetMigrator(&self) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, widgetMigrator)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CLKComplicationServer(pub id);
impl std::ops::Deref for CLKComplicationServer {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CLKComplicationServer {}
impl CLKComplicationServer {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CLKComplicationServer").unwrap(), alloc) })
    }
}
impl INSObject for CLKComplicationServer {}
impl PNSObject for CLKComplicationServer {}
impl std::convert::TryFrom<NSObject> for CLKComplicationServer {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<CLKComplicationServer, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CLKComplicationServer").unwrap()) };
        if is_kind_of {
            Ok(CLKComplicationServer(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to CLKComplicationServer")
        }
    }
}
impl ICLKComplicationServer for CLKComplicationServer {}
pub trait ICLKComplicationServer: Sized + std::ops::Deref {
    unsafe fn reloadTimelineForComplication_(&self, complication: CLKComplication)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, reloadTimelineForComplication : complication)
    }
    unsafe fn extendTimelineForComplication_(&self, complication: CLKComplication)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, extendTimelineForComplication : complication)
    }
    unsafe fn reloadComplicationDescriptors(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, reloadComplicationDescriptors)
    }
    unsafe fn activeComplications(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, activeComplications)
    }
    unsafe fn earliestTimeTravelDate(&self) -> NSDate
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, earliestTimeTravelDate)
    }
    unsafe fn latestTimeTravelDate(&self) -> NSDate
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, latestTimeTravelDate)
    }
    unsafe fn sharedInstance() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CLKComplicationServer").unwrap(), sharedInstance)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CLKComplicationTemplate(pub id);
impl std::ops::Deref for CLKComplicationTemplate {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CLKComplicationTemplate {}
impl CLKComplicationTemplate {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CLKComplicationTemplate").unwrap(), alloc) })
    }
}
impl PNSCopying for CLKComplicationTemplate {}
impl INSObject for CLKComplicationTemplate {}
impl PNSObject for CLKComplicationTemplate {}
impl std::convert::TryFrom<NSObject> for CLKComplicationTemplate {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<CLKComplicationTemplate, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CLKComplicationTemplate").unwrap()) };
        if is_kind_of {
            Ok(CLKComplicationTemplate(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to CLKComplicationTemplate")
        }
    }
}
impl ICLKComplicationTemplate for CLKComplicationTemplate {}
pub trait ICLKComplicationTemplate: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn tintColor(&self) -> UIColor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, tintColor)
    }
    unsafe fn setTintColor_(&self, tintColor: UIColor)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setTintColor : tintColor)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CLKComplicationTemplate").unwrap(), new)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CLKComplicationTemplateModularSmallSimpleText(pub id);
impl std::ops::Deref for CLKComplicationTemplateModularSmallSimpleText {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CLKComplicationTemplateModularSmallSimpleText {}
impl CLKComplicationTemplateModularSmallSimpleText {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CLKComplicationTemplateModularSmallSimpleText").unwrap(), alloc) })
    }
}
impl ICLKComplicationTemplate for CLKComplicationTemplateModularSmallSimpleText {}
impl PNSCopying for CLKComplicationTemplateModularSmallSimpleText {}
impl From<CLKComplicationTemplateModularSmallSimpleText> for CLKComplicationTemplate {
    fn from(child: CLKComplicationTemplateModularSmallSimpleText) -> CLKComplicationTemplate {
        CLKComplicationTemplate(child.0)
    }
}
impl std::convert::TryFrom<CLKComplicationTemplate>
    for CLKComplicationTemplateModularSmallSimpleText
{
    type Error = &'static str;
    fn try_from(
        parent: CLKComplicationTemplate,
    ) -> Result<CLKComplicationTemplateModularSmallSimpleText, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CLKComplicationTemplateModularSmallSimpleText").unwrap())
        };
        if is_kind_of {
            Ok(CLKComplicationTemplateModularSmallSimpleText(parent.0))
        } else {
            Err ("This CLKComplicationTemplate cannot be downcasted to CLKComplicationTemplateModularSmallSimpleText" ,)
        }
    }
}
impl INSObject for CLKComplicationTemplateModularSmallSimpleText {}
impl PNSObject for CLKComplicationTemplateModularSmallSimpleText {}
impl ICLKComplicationTemplateModularSmallSimpleText
    for CLKComplicationTemplateModularSmallSimpleText
{
}
pub trait ICLKComplicationTemplateModularSmallSimpleText: Sized + std::ops::Deref {
    unsafe fn initWithTextProvider_(&self, textProvider: CLKTextProvider) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithTextProvider : textProvider)
    }
    unsafe fn textProvider(&self) -> CLKTextProvider
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, textProvider)
    }
    unsafe fn setTextProvider_(&self, textProvider: CLKTextProvider)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setTextProvider : textProvider)
    }
    unsafe fn templateWithTextProvider_(textProvider: CLKTextProvider) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CLKComplicationTemplateModularSmallSimpleText").unwrap(), templateWithTextProvider : textProvider)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CLKComplicationTemplateModularSmallSimpleImage(pub id);
impl std::ops::Deref for CLKComplicationTemplateModularSmallSimpleImage {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CLKComplicationTemplateModularSmallSimpleImage {}
impl CLKComplicationTemplateModularSmallSimpleImage {
    pub fn alloc() -> Self {
        Self(unsafe {
            msg_send!(&*objc2::runtime::AnyClass::get(c"CLKComplicationTemplateModularSmallSimpleImage").unwrap(), alloc)
        })
    }
}
impl ICLKComplicationTemplate for CLKComplicationTemplateModularSmallSimpleImage {}
impl PNSCopying for CLKComplicationTemplateModularSmallSimpleImage {}
impl std::convert::TryFrom<CLKComplicationTemplate>
    for CLKComplicationTemplateModularSmallSimpleImage
{
    type Error = &'static str;
    fn try_from(
        parent: CLKComplicationTemplate,
    ) -> Result<CLKComplicationTemplateModularSmallSimpleImage, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CLKComplicationTemplateModularSmallSimpleImage").unwrap())
        };
        if is_kind_of {
            Ok(CLKComplicationTemplateModularSmallSimpleImage(parent.0))
        } else {
            Err ("This CLKComplicationTemplate cannot be downcasted to CLKComplicationTemplateModularSmallSimpleImage" ,)
        }
    }
}
impl INSObject for CLKComplicationTemplateModularSmallSimpleImage {}
impl PNSObject for CLKComplicationTemplateModularSmallSimpleImage {}
impl ICLKComplicationTemplateModularSmallSimpleImage
    for CLKComplicationTemplateModularSmallSimpleImage
{
}
pub trait ICLKComplicationTemplateModularSmallSimpleImage: Sized + std::ops::Deref {
    unsafe fn initWithImageProvider_(&self, imageProvider: CLKImageProvider) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithImageProvider : imageProvider)
    }
    unsafe fn imageProvider(&self) -> CLKImageProvider
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, imageProvider)
    }
    unsafe fn setImageProvider_(&self, imageProvider: CLKImageProvider)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setImageProvider : imageProvider)
    }
    unsafe fn templateWithImageProvider_(imageProvider: CLKImageProvider) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CLKComplicationTemplateModularSmallSimpleImage").unwrap(), templateWithImageProvider : imageProvider)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CLKComplicationTemplateModularSmallRingText(pub id);
impl std::ops::Deref for CLKComplicationTemplateModularSmallRingText {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CLKComplicationTemplateModularSmallRingText {}
impl CLKComplicationTemplateModularSmallRingText {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CLKComplicationTemplateModularSmallRingText").unwrap(), alloc) })
    }
}
impl ICLKComplicationTemplate for CLKComplicationTemplateModularSmallRingText {}
impl PNSCopying for CLKComplicationTemplateModularSmallRingText {}
impl std::convert::TryFrom<CLKComplicationTemplate>
    for CLKComplicationTemplateModularSmallRingText
{
    type Error = &'static str;
    fn try_from(
        parent: CLKComplicationTemplate,
    ) -> Result<CLKComplicationTemplateModularSmallRingText, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CLKComplicationTemplateModularSmallRingText").unwrap())
        };
        if is_kind_of {
            Ok(CLKComplicationTemplateModularSmallRingText(parent.0))
        } else {
            Err ("This CLKComplicationTemplate cannot be downcasted to CLKComplicationTemplateModularSmallRingText" ,)
        }
    }
}
impl INSObject for CLKComplicationTemplateModularSmallRingText {}
impl PNSObject for CLKComplicationTemplateModularSmallRingText {}
impl ICLKComplicationTemplateModularSmallRingText for CLKComplicationTemplateModularSmallRingText {}
pub trait ICLKComplicationTemplateModularSmallRingText: Sized + std::ops::Deref {
    unsafe fn initWithTextProvider_fillFraction_ringStyle_(
        &self,
        textProvider: CLKTextProvider,
        fillFraction: f32,
        ringStyle: CLKComplicationRingStyle,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithTextProvider : textProvider, fillFraction : fillFraction, ringStyle : ringStyle)
    }
    unsafe fn textProvider(&self) -> CLKTextProvider
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, textProvider)
    }
    unsafe fn setTextProvider_(&self, textProvider: CLKTextProvider)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setTextProvider : textProvider)
    }
    unsafe fn fillFraction(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, fillFraction)
    }
    unsafe fn setFillFraction_(&self, fillFraction: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setFillFraction : fillFraction)
    }
    unsafe fn ringStyle(&self) -> CLKComplicationRingStyle
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, ringStyle)
    }
    unsafe fn setRingStyle_(&self, ringStyle: CLKComplicationRingStyle)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRingStyle : ringStyle)
    }
    unsafe fn templateWithTextProvider_fillFraction_ringStyle_(
        textProvider: CLKTextProvider,
        fillFraction: f32,
        ringStyle: CLKComplicationRingStyle,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CLKComplicationTemplateModularSmallRingText").unwrap(), templateWithTextProvider : textProvider, fillFraction : fillFraction, ringStyle : ringStyle)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CLKComplicationTemplateModularSmallRingImage(pub id);
impl std::ops::Deref for CLKComplicationTemplateModularSmallRingImage {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CLKComplicationTemplateModularSmallRingImage {}
impl CLKComplicationTemplateModularSmallRingImage {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CLKComplicationTemplateModularSmallRingImage").unwrap(), alloc) })
    }
}
impl ICLKComplicationTemplate for CLKComplicationTemplateModularSmallRingImage {}
impl PNSCopying for CLKComplicationTemplateModularSmallRingImage {}
impl std::convert::TryFrom<CLKComplicationTemplate>
    for CLKComplicationTemplateModularSmallRingImage
{
    type Error = &'static str;
    fn try_from(
        parent: CLKComplicationTemplate,
    ) -> Result<CLKComplicationTemplateModularSmallRingImage, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CLKComplicationTemplateModularSmallRingImage").unwrap())
        };
        if is_kind_of {
            Ok(CLKComplicationTemplateModularSmallRingImage(parent.0))
        } else {
            Err ("This CLKComplicationTemplate cannot be downcasted to CLKComplicationTemplateModularSmallRingImage" ,)
        }
    }
}
impl INSObject for CLKComplicationTemplateModularSmallRingImage {}
impl PNSObject for CLKComplicationTemplateModularSmallRingImage {}
impl ICLKComplicationTemplateModularSmallRingImage
    for CLKComplicationTemplateModularSmallRingImage
{
}
pub trait ICLKComplicationTemplateModularSmallRingImage: Sized + std::ops::Deref {
    unsafe fn initWithImageProvider_fillFraction_ringStyle_(
        &self,
        imageProvider: CLKImageProvider,
        fillFraction: f32,
        ringStyle: CLKComplicationRingStyle,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithImageProvider : imageProvider, fillFraction : fillFraction, ringStyle : ringStyle)
    }
    unsafe fn imageProvider(&self) -> CLKImageProvider
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, imageProvider)
    }
    unsafe fn setImageProvider_(&self, imageProvider: CLKImageProvider)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setImageProvider : imageProvider)
    }
    unsafe fn fillFraction(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, fillFraction)
    }
    unsafe fn setFillFraction_(&self, fillFraction: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setFillFraction : fillFraction)
    }
    unsafe fn ringStyle(&self) -> CLKComplicationRingStyle
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, ringStyle)
    }
    unsafe fn setRingStyle_(&self, ringStyle: CLKComplicationRingStyle)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRingStyle : ringStyle)
    }
    unsafe fn templateWithImageProvider_fillFraction_ringStyle_(
        imageProvider: CLKImageProvider,
        fillFraction: f32,
        ringStyle: CLKComplicationRingStyle,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CLKComplicationTemplateModularSmallRingImage").unwrap(), templateWithImageProvider : imageProvider, fillFraction : fillFraction, ringStyle : ringStyle)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CLKComplicationTemplateModularSmallStackText(pub id);
impl std::ops::Deref for CLKComplicationTemplateModularSmallStackText {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CLKComplicationTemplateModularSmallStackText {}
impl CLKComplicationTemplateModularSmallStackText {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CLKComplicationTemplateModularSmallStackText").unwrap(), alloc) })
    }
}
impl ICLKComplicationTemplate for CLKComplicationTemplateModularSmallStackText {}
impl PNSCopying for CLKComplicationTemplateModularSmallStackText {}
impl std::convert::TryFrom<CLKComplicationTemplate>
    for CLKComplicationTemplateModularSmallStackText
{
    type Error = &'static str;
    fn try_from(
        parent: CLKComplicationTemplate,
    ) -> Result<CLKComplicationTemplateModularSmallStackText, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CLKComplicationTemplateModularSmallStackText").unwrap())
        };
        if is_kind_of {
            Ok(CLKComplicationTemplateModularSmallStackText(parent.0))
        } else {
            Err ("This CLKComplicationTemplate cannot be downcasted to CLKComplicationTemplateModularSmallStackText" ,)
        }
    }
}
impl INSObject for CLKComplicationTemplateModularSmallStackText {}
impl PNSObject for CLKComplicationTemplateModularSmallStackText {}
impl ICLKComplicationTemplateModularSmallStackText
    for CLKComplicationTemplateModularSmallStackText
{
}
pub trait ICLKComplicationTemplateModularSmallStackText: Sized + std::ops::Deref {
    unsafe fn initWithLine1TextProvider_line2TextProvider_(
        &self,
        line1TextProvider: CLKTextProvider,
        line2TextProvider: CLKTextProvider,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithLine1TextProvider : line1TextProvider, line2TextProvider : line2TextProvider)
    }
    unsafe fn line1TextProvider(&self) -> CLKTextProvider
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, line1TextProvider)
    }
    unsafe fn setLine1TextProvider_(&self, line1TextProvider: CLKTextProvider)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setLine1TextProvider : line1TextProvider)
    }
    unsafe fn line2TextProvider(&self) -> CLKTextProvider
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, line2TextProvider)
    }
    unsafe fn setLine2TextProvider_(&self, line2TextProvider: CLKTextProvider)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setLine2TextProvider : line2TextProvider)
    }
    unsafe fn highlightLine2(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, highlightLine2)
    }
    unsafe fn setHighlightLine2_(&self, highlightLine2: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setHighlightLine2 : highlightLine2)
    }
    unsafe fn templateWithLine1TextProvider_line2TextProvider_(
        line1TextProvider: CLKTextProvider,
        line2TextProvider: CLKTextProvider,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CLKComplicationTemplateModularSmallStackText").unwrap(), templateWithLine1TextProvider : line1TextProvider, line2TextProvider : line2TextProvider)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CLKComplicationTemplateModularSmallStackImage(pub id);
impl std::ops::Deref for CLKComplicationTemplateModularSmallStackImage {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CLKComplicationTemplateModularSmallStackImage {}
impl CLKComplicationTemplateModularSmallStackImage {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CLKComplicationTemplateModularSmallStackImage").unwrap(), alloc) })
    }
}
impl ICLKComplicationTemplate for CLKComplicationTemplateModularSmallStackImage {}
impl PNSCopying for CLKComplicationTemplateModularSmallStackImage {}
impl std::convert::TryFrom<CLKComplicationTemplate>
    for CLKComplicationTemplateModularSmallStackImage
{
    type Error = &'static str;
    fn try_from(
        parent: CLKComplicationTemplate,
    ) -> Result<CLKComplicationTemplateModularSmallStackImage, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CLKComplicationTemplateModularSmallStackImage").unwrap())
        };
        if is_kind_of {
            Ok(CLKComplicationTemplateModularSmallStackImage(parent.0))
        } else {
            Err ("This CLKComplicationTemplate cannot be downcasted to CLKComplicationTemplateModularSmallStackImage" ,)
        }
    }
}
impl INSObject for CLKComplicationTemplateModularSmallStackImage {}
impl PNSObject for CLKComplicationTemplateModularSmallStackImage {}
impl ICLKComplicationTemplateModularSmallStackImage
    for CLKComplicationTemplateModularSmallStackImage
{
}
pub trait ICLKComplicationTemplateModularSmallStackImage: Sized + std::ops::Deref {
    unsafe fn initWithLine1ImageProvider_line2TextProvider_(
        &self,
        line1ImageProvider: CLKImageProvider,
        line2TextProvider: CLKTextProvider,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithLine1ImageProvider : line1ImageProvider, line2TextProvider : line2TextProvider)
    }
    unsafe fn line1ImageProvider(&self) -> CLKImageProvider
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, line1ImageProvider)
    }
    unsafe fn setLine1ImageProvider_(&self, line1ImageProvider: CLKImageProvider)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setLine1ImageProvider : line1ImageProvider)
    }
    unsafe fn line2TextProvider(&self) -> CLKTextProvider
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, line2TextProvider)
    }
    unsafe fn setLine2TextProvider_(&self, line2TextProvider: CLKTextProvider)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setLine2TextProvider : line2TextProvider)
    }
    unsafe fn highlightLine2(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, highlightLine2)
    }
    unsafe fn setHighlightLine2_(&self, highlightLine2: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setHighlightLine2 : highlightLine2)
    }
    unsafe fn templateWithLine1ImageProvider_line2TextProvider_(
        line1ImageProvider: CLKImageProvider,
        line2TextProvider: CLKTextProvider,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CLKComplicationTemplateModularSmallStackImage").unwrap(), templateWithLine1ImageProvider : line1ImageProvider, line2TextProvider : line2TextProvider)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CLKComplicationTemplateModularSmallColumnsText(pub id);
impl std::ops::Deref for CLKComplicationTemplateModularSmallColumnsText {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CLKComplicationTemplateModularSmallColumnsText {}
impl CLKComplicationTemplateModularSmallColumnsText {
    pub fn alloc() -> Self {
        Self(unsafe {
            msg_send!(&*objc2::runtime::AnyClass::get(c"CLKComplicationTemplateModularSmallColumnsText").unwrap(), alloc)
        })
    }
}
impl ICLKComplicationTemplate for CLKComplicationTemplateModularSmallColumnsText {}
impl PNSCopying for CLKComplicationTemplateModularSmallColumnsText {}
impl std::convert::TryFrom<CLKComplicationTemplate>
    for CLKComplicationTemplateModularSmallColumnsText
{
    type Error = &'static str;
    fn try_from(
        parent: CLKComplicationTemplate,
    ) -> Result<CLKComplicationTemplateModularSmallColumnsText, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CLKComplicationTemplateModularSmallColumnsText").unwrap())
        };
        if is_kind_of {
            Ok(CLKComplicationTemplateModularSmallColumnsText(parent.0))
        } else {
            Err ("This CLKComplicationTemplate cannot be downcasted to CLKComplicationTemplateModularSmallColumnsText" ,)
        }
    }
}
impl INSObject for CLKComplicationTemplateModularSmallColumnsText {}
impl PNSObject for CLKComplicationTemplateModularSmallColumnsText {}
impl ICLKComplicationTemplateModularSmallColumnsText
    for CLKComplicationTemplateModularSmallColumnsText
{
}
pub trait ICLKComplicationTemplateModularSmallColumnsText: Sized + std::ops::Deref {
    unsafe fn initWithRow1Column1TextProvider_row1Column2TextProvider_row2Column1TextProvider_row2Column2TextProvider_(
        &self,
        row1Column1TextProvider: CLKTextProvider,
        row1Column2TextProvider: CLKTextProvider,
        row2Column1TextProvider: CLKTextProvider,
        row2Column2TextProvider: CLKTextProvider,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithRow1Column1TextProvider : row1Column1TextProvider, row1Column2TextProvider : row1Column2TextProvider, row2Column1TextProvider : row2Column1TextProvider, row2Column2TextProvider : row2Column2TextProvider)
    }
    unsafe fn row1Column1TextProvider(&self) -> CLKTextProvider
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, row1Column1TextProvider)
    }
    unsafe fn setRow1Column1TextProvider_(&self, row1Column1TextProvider: CLKTextProvider)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRow1Column1TextProvider : row1Column1TextProvider)
    }
    unsafe fn row1Column2TextProvider(&self) -> CLKTextProvider
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, row1Column2TextProvider)
    }
    unsafe fn setRow1Column2TextProvider_(&self, row1Column2TextProvider: CLKTextProvider)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRow1Column2TextProvider : row1Column2TextProvider)
    }
    unsafe fn row2Column1TextProvider(&self) -> CLKTextProvider
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, row2Column1TextProvider)
    }
    unsafe fn setRow2Column1TextProvider_(&self, row2Column1TextProvider: CLKTextProvider)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRow2Column1TextProvider : row2Column1TextProvider)
    }
    unsafe fn row2Column2TextProvider(&self) -> CLKTextProvider
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, row2Column2TextProvider)
    }
    unsafe fn setRow2Column2TextProvider_(&self, row2Column2TextProvider: CLKTextProvider)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRow2Column2TextProvider : row2Column2TextProvider)
    }
    unsafe fn column2Alignment(&self) -> CLKComplicationColumnAlignment
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, column2Alignment)
    }
    unsafe fn setColumn2Alignment_(&self, column2Alignment: CLKComplicationColumnAlignment)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setColumn2Alignment : column2Alignment)
    }
    unsafe fn highlightColumn2(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, highlightColumn2)
    }
    unsafe fn setHighlightColumn2_(&self, highlightColumn2: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setHighlightColumn2 : highlightColumn2)
    }
    unsafe fn templateWithRow1Column1TextProvider_row1Column2TextProvider_row2Column1TextProvider_row2Column2TextProvider_(
        row1Column1TextProvider: CLKTextProvider,
        row1Column2TextProvider: CLKTextProvider,
        row2Column1TextProvider: CLKTextProvider,
        row2Column2TextProvider: CLKTextProvider,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CLKComplicationTemplateModularSmallColumnsText").unwrap(), templateWithRow1Column1TextProvider : row1Column1TextProvider, row1Column2TextProvider : row1Column2TextProvider, row2Column1TextProvider : row2Column1TextProvider, row2Column2TextProvider : row2Column2TextProvider)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CLKComplicationTemplateModularLargeStandardBody(pub id);
impl std::ops::Deref for CLKComplicationTemplateModularLargeStandardBody {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CLKComplicationTemplateModularLargeStandardBody {}
impl CLKComplicationTemplateModularLargeStandardBody {
    pub fn alloc() -> Self {
        Self(unsafe {
            msg_send!(&*objc2::runtime::AnyClass::get(c"CLKComplicationTemplateModularLargeStandardBody").unwrap(), alloc)
        })
    }
}
impl ICLKComplicationTemplate for CLKComplicationTemplateModularLargeStandardBody {}
impl PNSCopying for CLKComplicationTemplateModularLargeStandardBody {}
impl std::convert::TryFrom<CLKComplicationTemplate>
    for CLKComplicationTemplateModularLargeStandardBody
{
    type Error = &'static str;
    fn try_from(
        parent: CLKComplicationTemplate,
    ) -> Result<CLKComplicationTemplateModularLargeStandardBody, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CLKComplicationTemplateModularLargeStandardBody").unwrap())
        };
        if is_kind_of {
            Ok(CLKComplicationTemplateModularLargeStandardBody(parent.0))
        } else {
            Err ("This CLKComplicationTemplate cannot be downcasted to CLKComplicationTemplateModularLargeStandardBody" ,)
        }
    }
}
impl INSObject for CLKComplicationTemplateModularLargeStandardBody {}
impl PNSObject for CLKComplicationTemplateModularLargeStandardBody {}
impl ICLKComplicationTemplateModularLargeStandardBody
    for CLKComplicationTemplateModularLargeStandardBody
{
}
pub trait ICLKComplicationTemplateModularLargeStandardBody: Sized + std::ops::Deref {
    unsafe fn initWithHeaderTextProvider_body1TextProvider_(
        &self,
        headerTextProvider: CLKTextProvider,
        body1TextProvider: CLKTextProvider,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithHeaderTextProvider : headerTextProvider, body1TextProvider : body1TextProvider)
    }
    unsafe fn initWithHeaderTextProvider_body1TextProvider_body2TextProvider_(
        &self,
        headerTextProvider: CLKTextProvider,
        body1TextProvider: CLKTextProvider,
        body2TextProvider: CLKTextProvider,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithHeaderTextProvider : headerTextProvider, body1TextProvider : body1TextProvider, body2TextProvider : body2TextProvider)
    }
    unsafe fn initWithHeaderImageProvider_headerTextProvider_body1TextProvider_(
        &self,
        headerImageProvider: CLKImageProvider,
        headerTextProvider: CLKTextProvider,
        body1TextProvider: CLKTextProvider,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithHeaderImageProvider : headerImageProvider, headerTextProvider : headerTextProvider, body1TextProvider : body1TextProvider)
    }
    unsafe fn initWithHeaderImageProvider_headerTextProvider_body1TextProvider_body2TextProvider_(
        &self,
        headerImageProvider: CLKImageProvider,
        headerTextProvider: CLKTextProvider,
        body1TextProvider: CLKTextProvider,
        body2TextProvider: CLKTextProvider,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithHeaderImageProvider : headerImageProvider, headerTextProvider : headerTextProvider, body1TextProvider : body1TextProvider, body2TextProvider : body2TextProvider)
    }
    unsafe fn headerImageProvider(&self) -> CLKImageProvider
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, headerImageProvider)
    }
    unsafe fn setHeaderImageProvider_(&self, headerImageProvider: CLKImageProvider)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setHeaderImageProvider : headerImageProvider)
    }
    unsafe fn headerTextProvider(&self) -> CLKTextProvider
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, headerTextProvider)
    }
    unsafe fn setHeaderTextProvider_(&self, headerTextProvider: CLKTextProvider)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setHeaderTextProvider : headerTextProvider)
    }
    unsafe fn body1TextProvider(&self) -> CLKTextProvider
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, body1TextProvider)
    }
    unsafe fn setBody1TextProvider_(&self, body1TextProvider: CLKTextProvider)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setBody1TextProvider : body1TextProvider)
    }
    unsafe fn body2TextProvider(&self) -> CLKTextProvider
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, body2TextProvider)
    }
    unsafe fn setBody2TextProvider_(&self, body2TextProvider: CLKTextProvider)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setBody2TextProvider : body2TextProvider)
    }
    unsafe fn templateWithHeaderTextProvider_body1TextProvider_(
        headerTextProvider: CLKTextProvider,
        body1TextProvider: CLKTextProvider,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CLKComplicationTemplateModularLargeStandardBody").unwrap(), templateWithHeaderTextProvider : headerTextProvider, body1TextProvider : body1TextProvider)
    }
    unsafe fn templateWithHeaderTextProvider_body1TextProvider_body2TextProvider_(
        headerTextProvider: CLKTextProvider,
        body1TextProvider: CLKTextProvider,
        body2TextProvider: CLKTextProvider,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CLKComplicationTemplateModularLargeStandardBody").unwrap(), templateWithHeaderTextProvider : headerTextProvider, body1TextProvider : body1TextProvider, body2TextProvider : body2TextProvider)
    }
    unsafe fn templateWithHeaderImageProvider_headerTextProvider_body1TextProvider_(
        headerImageProvider: CLKImageProvider,
        headerTextProvider: CLKTextProvider,
        body1TextProvider: CLKTextProvider,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CLKComplicationTemplateModularLargeStandardBody").unwrap(), templateWithHeaderImageProvider : headerImageProvider, headerTextProvider : headerTextProvider, body1TextProvider : body1TextProvider)
    }
    unsafe fn templateWithHeaderImageProvider_headerTextProvider_body1TextProvider_body2TextProvider_(
        headerImageProvider: CLKImageProvider,
        headerTextProvider: CLKTextProvider,
        body1TextProvider: CLKTextProvider,
        body2TextProvider: CLKTextProvider,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CLKComplicationTemplateModularLargeStandardBody").unwrap(), templateWithHeaderImageProvider : headerImageProvider, headerTextProvider : headerTextProvider, body1TextProvider : body1TextProvider, body2TextProvider : body2TextProvider)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CLKComplicationTemplateModularLargeTallBody(pub id);
impl std::ops::Deref for CLKComplicationTemplateModularLargeTallBody {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CLKComplicationTemplateModularLargeTallBody {}
impl CLKComplicationTemplateModularLargeTallBody {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CLKComplicationTemplateModularLargeTallBody").unwrap(), alloc) })
    }
}
impl ICLKComplicationTemplate for CLKComplicationTemplateModularLargeTallBody {}
impl PNSCopying for CLKComplicationTemplateModularLargeTallBody {}
impl std::convert::TryFrom<CLKComplicationTemplate>
    for CLKComplicationTemplateModularLargeTallBody
{
    type Error = &'static str;
    fn try_from(
        parent: CLKComplicationTemplate,
    ) -> Result<CLKComplicationTemplateModularLargeTallBody, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CLKComplicationTemplateModularLargeTallBody").unwrap())
        };
        if is_kind_of {
            Ok(CLKComplicationTemplateModularLargeTallBody(parent.0))
        } else {
            Err ("This CLKComplicationTemplate cannot be downcasted to CLKComplicationTemplateModularLargeTallBody" ,)
        }
    }
}
impl INSObject for CLKComplicationTemplateModularLargeTallBody {}
impl PNSObject for CLKComplicationTemplateModularLargeTallBody {}
impl ICLKComplicationTemplateModularLargeTallBody for CLKComplicationTemplateModularLargeTallBody {}
pub trait ICLKComplicationTemplateModularLargeTallBody: Sized + std::ops::Deref {
    unsafe fn initWithHeaderTextProvider_bodyTextProvider_(
        &self,
        headerTextProvider: CLKTextProvider,
        bodyTextProvider: CLKTextProvider,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithHeaderTextProvider : headerTextProvider, bodyTextProvider : bodyTextProvider)
    }
    unsafe fn headerTextProvider(&self) -> CLKTextProvider
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, headerTextProvider)
    }
    unsafe fn setHeaderTextProvider_(&self, headerTextProvider: CLKTextProvider)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setHeaderTextProvider : headerTextProvider)
    }
    unsafe fn bodyTextProvider(&self) -> CLKTextProvider
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, bodyTextProvider)
    }
    unsafe fn setBodyTextProvider_(&self, bodyTextProvider: CLKTextProvider)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setBodyTextProvider : bodyTextProvider)
    }
    unsafe fn templateWithHeaderTextProvider_bodyTextProvider_(
        headerTextProvider: CLKTextProvider,
        bodyTextProvider: CLKTextProvider,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CLKComplicationTemplateModularLargeTallBody").unwrap(), templateWithHeaderTextProvider : headerTextProvider, bodyTextProvider : bodyTextProvider)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CLKComplicationTemplateModularLargeTable(pub id);
impl std::ops::Deref for CLKComplicationTemplateModularLargeTable {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CLKComplicationTemplateModularLargeTable {}
impl CLKComplicationTemplateModularLargeTable {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CLKComplicationTemplateModularLargeTable").unwrap(), alloc) })
    }
}
impl ICLKComplicationTemplate for CLKComplicationTemplateModularLargeTable {}
impl PNSCopying for CLKComplicationTemplateModularLargeTable {}
impl std::convert::TryFrom<CLKComplicationTemplate> for CLKComplicationTemplateModularLargeTable {
    type Error = &'static str;
    fn try_from(
        parent: CLKComplicationTemplate,
    ) -> Result<CLKComplicationTemplateModularLargeTable, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CLKComplicationTemplateModularLargeTable").unwrap())
        };
        if is_kind_of {
            Ok(CLKComplicationTemplateModularLargeTable(parent.0))
        } else {
            Err ("This CLKComplicationTemplate cannot be downcasted to CLKComplicationTemplateModularLargeTable" ,)
        }
    }
}
impl INSObject for CLKComplicationTemplateModularLargeTable {}
impl PNSObject for CLKComplicationTemplateModularLargeTable {}
impl ICLKComplicationTemplateModularLargeTable for CLKComplicationTemplateModularLargeTable {}
pub trait ICLKComplicationTemplateModularLargeTable: Sized + std::ops::Deref {
    unsafe fn initWithHeaderTextProvider_row1Column1TextProvider_row1Column2TextProvider_row2Column1TextProvider_row2Column2TextProvider_(
        &self,
        headerTextProvider: CLKTextProvider,
        row1Column1TextProvider: CLKTextProvider,
        row1Column2TextProvider: CLKTextProvider,
        row2Column1TextProvider: CLKTextProvider,
        row2Column2TextProvider: CLKTextProvider,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithHeaderTextProvider : headerTextProvider, row1Column1TextProvider : row1Column1TextProvider, row1Column2TextProvider : row1Column2TextProvider, row2Column1TextProvider : row2Column1TextProvider, row2Column2TextProvider : row2Column2TextProvider)
    }
    unsafe fn initWithHeaderImageProvider_headerTextProvider_row1Column1TextProvider_row1Column2TextProvider_row2Column1TextProvider_row2Column2TextProvider_(
        &self,
        headerImageProvider: CLKImageProvider,
        headerTextProvider: CLKTextProvider,
        row1Column1TextProvider: CLKTextProvider,
        row1Column2TextProvider: CLKTextProvider,
        row2Column1TextProvider: CLKTextProvider,
        row2Column2TextProvider: CLKTextProvider,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithHeaderImageProvider : headerImageProvider, headerTextProvider : headerTextProvider, row1Column1TextProvider : row1Column1TextProvider, row1Column2TextProvider : row1Column2TextProvider, row2Column1TextProvider : row2Column1TextProvider, row2Column2TextProvider : row2Column2TextProvider)
    }
    unsafe fn headerImageProvider(&self) -> CLKImageProvider
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, headerImageProvider)
    }
    unsafe fn setHeaderImageProvider_(&self, headerImageProvider: CLKImageProvider)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setHeaderImageProvider : headerImageProvider)
    }
    unsafe fn headerTextProvider(&self) -> CLKTextProvider
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, headerTextProvider)
    }
    unsafe fn setHeaderTextProvider_(&self, headerTextProvider: CLKTextProvider)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setHeaderTextProvider : headerTextProvider)
    }
    unsafe fn row1Column1TextProvider(&self) -> CLKTextProvider
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, row1Column1TextProvider)
    }
    unsafe fn setRow1Column1TextProvider_(&self, row1Column1TextProvider: CLKTextProvider)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRow1Column1TextProvider : row1Column1TextProvider)
    }
    unsafe fn row1Column2TextProvider(&self) -> CLKTextProvider
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, row1Column2TextProvider)
    }
    unsafe fn setRow1Column2TextProvider_(&self, row1Column2TextProvider: CLKTextProvider)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRow1Column2TextProvider : row1Column2TextProvider)
    }
    unsafe fn row2Column1TextProvider(&self) -> CLKTextProvider
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, row2Column1TextProvider)
    }
    unsafe fn setRow2Column1TextProvider_(&self, row2Column1TextProvider: CLKTextProvider)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRow2Column1TextProvider : row2Column1TextProvider)
    }
    unsafe fn row2Column2TextProvider(&self) -> CLKTextProvider
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, row2Column2TextProvider)
    }
    unsafe fn setRow2Column2TextProvider_(&self, row2Column2TextProvider: CLKTextProvider)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRow2Column2TextProvider : row2Column2TextProvider)
    }
    unsafe fn column2Alignment(&self) -> CLKComplicationColumnAlignment
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, column2Alignment)
    }
    unsafe fn setColumn2Alignment_(&self, column2Alignment: CLKComplicationColumnAlignment)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setColumn2Alignment : column2Alignment)
    }
    unsafe fn templateWithHeaderTextProvider_row1Column1TextProvider_row1Column2TextProvider_row2Column1TextProvider_row2Column2TextProvider_(
        headerTextProvider: CLKTextProvider,
        row1Column1TextProvider: CLKTextProvider,
        row1Column2TextProvider: CLKTextProvider,
        row2Column1TextProvider: CLKTextProvider,
        row2Column2TextProvider: CLKTextProvider,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CLKComplicationTemplateModularLargeTable").unwrap(), templateWithHeaderTextProvider : headerTextProvider, row1Column1TextProvider : row1Column1TextProvider, row1Column2TextProvider : row1Column2TextProvider, row2Column1TextProvider : row2Column1TextProvider, row2Column2TextProvider : row2Column2TextProvider)
    }
    unsafe fn templateWithHeaderImageProvider_headerTextProvider_row1Column1TextProvider_row1Column2TextProvider_row2Column1TextProvider_row2Column2TextProvider_(
        headerImageProvider: CLKImageProvider,
        headerTextProvider: CLKTextProvider,
        row1Column1TextProvider: CLKTextProvider,
        row1Column2TextProvider: CLKTextProvider,
        row2Column1TextProvider: CLKTextProvider,
        row2Column2TextProvider: CLKTextProvider,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CLKComplicationTemplateModularLargeTable").unwrap(), templateWithHeaderImageProvider : headerImageProvider, headerTextProvider : headerTextProvider, row1Column1TextProvider : row1Column1TextProvider, row1Column2TextProvider : row1Column2TextProvider, row2Column1TextProvider : row2Column1TextProvider, row2Column2TextProvider : row2Column2TextProvider)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CLKComplicationTemplateModularLargeColumns(pub id);
impl std::ops::Deref for CLKComplicationTemplateModularLargeColumns {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CLKComplicationTemplateModularLargeColumns {}
impl CLKComplicationTemplateModularLargeColumns {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CLKComplicationTemplateModularLargeColumns").unwrap(), alloc) })
    }
}
impl ICLKComplicationTemplate for CLKComplicationTemplateModularLargeColumns {}
impl PNSCopying for CLKComplicationTemplateModularLargeColumns {}
impl std::convert::TryFrom<CLKComplicationTemplate> for CLKComplicationTemplateModularLargeColumns {
    type Error = &'static str;
    fn try_from(
        parent: CLKComplicationTemplate,
    ) -> Result<CLKComplicationTemplateModularLargeColumns, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CLKComplicationTemplateModularLargeColumns").unwrap())
        };
        if is_kind_of {
            Ok(CLKComplicationTemplateModularLargeColumns(parent.0))
        } else {
            Err ("This CLKComplicationTemplate cannot be downcasted to CLKComplicationTemplateModularLargeColumns" ,)
        }
    }
}
impl INSObject for CLKComplicationTemplateModularLargeColumns {}
impl PNSObject for CLKComplicationTemplateModularLargeColumns {}
impl ICLKComplicationTemplateModularLargeColumns for CLKComplicationTemplateModularLargeColumns {}
pub trait ICLKComplicationTemplateModularLargeColumns: Sized + std::ops::Deref {
    unsafe fn initWithRow1Column1TextProvider_row1Column2TextProvider_row2Column1TextProvider_row2Column2TextProvider_row3Column1TextProvider_row3Column2TextProvider_(
        &self,
        row1Column1TextProvider: CLKTextProvider,
        row1Column2TextProvider: CLKTextProvider,
        row2Column1TextProvider: CLKTextProvider,
        row2Column2TextProvider: CLKTextProvider,
        row3Column1TextProvider: CLKTextProvider,
        row3Column2TextProvider: CLKTextProvider,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithRow1Column1TextProvider : row1Column1TextProvider, row1Column2TextProvider : row1Column2TextProvider, row2Column1TextProvider : row2Column1TextProvider, row2Column2TextProvider : row2Column2TextProvider, row3Column1TextProvider : row3Column1TextProvider, row3Column2TextProvider : row3Column2TextProvider)
    }
    unsafe fn initWithRow1ImageProvider_row1Column1TextProvider_row1Column2TextProvider_row2ImageProvider_row2Column1TextProvider_row2Column2TextProvider_row3ImageProvider_row3Column1TextProvider_row3Column2TextProvider_(
        &self,
        row1ImageProvider: CLKImageProvider,
        row1Column1TextProvider: CLKTextProvider,
        row1Column2TextProvider: CLKTextProvider,
        row2ImageProvider: CLKImageProvider,
        row2Column1TextProvider: CLKTextProvider,
        row2Column2TextProvider: CLKTextProvider,
        row3ImageProvider: CLKImageProvider,
        row3Column1TextProvider: CLKTextProvider,
        row3Column2TextProvider: CLKTextProvider,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithRow1ImageProvider : row1ImageProvider, row1Column1TextProvider : row1Column1TextProvider, row1Column2TextProvider : row1Column2TextProvider, row2ImageProvider : row2ImageProvider, row2Column1TextProvider : row2Column1TextProvider, row2Column2TextProvider : row2Column2TextProvider, row3ImageProvider : row3ImageProvider, row3Column1TextProvider : row3Column1TextProvider, row3Column2TextProvider : row3Column2TextProvider)
    }
    unsafe fn row1ImageProvider(&self) -> CLKImageProvider
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, row1ImageProvider)
    }
    unsafe fn setRow1ImageProvider_(&self, row1ImageProvider: CLKImageProvider)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRow1ImageProvider : row1ImageProvider)
    }
    unsafe fn row1Column1TextProvider(&self) -> CLKTextProvider
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, row1Column1TextProvider)
    }
    unsafe fn setRow1Column1TextProvider_(&self, row1Column1TextProvider: CLKTextProvider)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRow1Column1TextProvider : row1Column1TextProvider)
    }
    unsafe fn row1Column2TextProvider(&self) -> CLKTextProvider
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, row1Column2TextProvider)
    }
    unsafe fn setRow1Column2TextProvider_(&self, row1Column2TextProvider: CLKTextProvider)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRow1Column2TextProvider : row1Column2TextProvider)
    }
    unsafe fn row2ImageProvider(&self) -> CLKImageProvider
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, row2ImageProvider)
    }
    unsafe fn setRow2ImageProvider_(&self, row2ImageProvider: CLKImageProvider)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRow2ImageProvider : row2ImageProvider)
    }
    unsafe fn row2Column1TextProvider(&self) -> CLKTextProvider
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, row2Column1TextProvider)
    }
    unsafe fn setRow2Column1TextProvider_(&self, row2Column1TextProvider: CLKTextProvider)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRow2Column1TextProvider : row2Column1TextProvider)
    }
    unsafe fn row2Column2TextProvider(&self) -> CLKTextProvider
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, row2Column2TextProvider)
    }
    unsafe fn setRow2Column2TextProvider_(&self, row2Column2TextProvider: CLKTextProvider)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRow2Column2TextProvider : row2Column2TextProvider)
    }
    unsafe fn row3ImageProvider(&self) -> CLKImageProvider
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, row3ImageProvider)
    }
    unsafe fn setRow3ImageProvider_(&self, row3ImageProvider: CLKImageProvider)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRow3ImageProvider : row3ImageProvider)
    }
    unsafe fn row3Column1TextProvider(&self) -> CLKTextProvider
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, row3Column1TextProvider)
    }
    unsafe fn setRow3Column1TextProvider_(&self, row3Column1TextProvider: CLKTextProvider)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRow3Column1TextProvider : row3Column1TextProvider)
    }
    unsafe fn row3Column2TextProvider(&self) -> CLKTextProvider
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, row3Column2TextProvider)
    }
    unsafe fn setRow3Column2TextProvider_(&self, row3Column2TextProvider: CLKTextProvider)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRow3Column2TextProvider : row3Column2TextProvider)
    }
    unsafe fn column2Alignment(&self) -> CLKComplicationColumnAlignment
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, column2Alignment)
    }
    unsafe fn setColumn2Alignment_(&self, column2Alignment: CLKComplicationColumnAlignment)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setColumn2Alignment : column2Alignment)
    }
    unsafe fn templateWithRow1Column1TextProvider_row1Column2TextProvider_row2Column1TextProvider_row2Column2TextProvider_row3Column1TextProvider_row3Column2TextProvider_(
        row1Column1TextProvider: CLKTextProvider,
        row1Column2TextProvider: CLKTextProvider,
        row2Column1TextProvider: CLKTextProvider,
        row2Column2TextProvider: CLKTextProvider,
        row3Column1TextProvider: CLKTextProvider,
        row3Column2TextProvider: CLKTextProvider,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CLKComplicationTemplateModularLargeColumns").unwrap(), templateWithRow1Column1TextProvider : row1Column1TextProvider, row1Column2TextProvider : row1Column2TextProvider, row2Column1TextProvider : row2Column1TextProvider, row2Column2TextProvider : row2Column2TextProvider, row3Column1TextProvider : row3Column1TextProvider, row3Column2TextProvider : row3Column2TextProvider)
    }
    unsafe fn templateWithRow1ImageProvider_row1Column1TextProvider_row1Column2TextProvider_row2ImageProvider_row2Column1TextProvider_row2Column2TextProvider_row3ImageProvider_row3Column1TextProvider_row3Column2TextProvider_(
        row1ImageProvider: CLKImageProvider,
        row1Column1TextProvider: CLKTextProvider,
        row1Column2TextProvider: CLKTextProvider,
        row2ImageProvider: CLKImageProvider,
        row2Column1TextProvider: CLKTextProvider,
        row2Column2TextProvider: CLKTextProvider,
        row3ImageProvider: CLKImageProvider,
        row3Column1TextProvider: CLKTextProvider,
        row3Column2TextProvider: CLKTextProvider,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CLKComplicationTemplateModularLargeColumns").unwrap(), templateWithRow1ImageProvider : row1ImageProvider, row1Column1TextProvider : row1Column1TextProvider, row1Column2TextProvider : row1Column2TextProvider, row2ImageProvider : row2ImageProvider, row2Column1TextProvider : row2Column1TextProvider, row2Column2TextProvider : row2Column2TextProvider, row3ImageProvider : row3ImageProvider, row3Column1TextProvider : row3Column1TextProvider, row3Column2TextProvider : row3Column2TextProvider)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CLKComplicationTemplateUtilitarianSmallSquare(pub id);
impl std::ops::Deref for CLKComplicationTemplateUtilitarianSmallSquare {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CLKComplicationTemplateUtilitarianSmallSquare {}
impl CLKComplicationTemplateUtilitarianSmallSquare {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CLKComplicationTemplateUtilitarianSmallSquare").unwrap(), alloc) })
    }
}
impl ICLKComplicationTemplate for CLKComplicationTemplateUtilitarianSmallSquare {}
impl PNSCopying for CLKComplicationTemplateUtilitarianSmallSquare {}
impl std::convert::TryFrom<CLKComplicationTemplate>
    for CLKComplicationTemplateUtilitarianSmallSquare
{
    type Error = &'static str;
    fn try_from(
        parent: CLKComplicationTemplate,
    ) -> Result<CLKComplicationTemplateUtilitarianSmallSquare, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CLKComplicationTemplateUtilitarianSmallSquare").unwrap())
        };
        if is_kind_of {
            Ok(CLKComplicationTemplateUtilitarianSmallSquare(parent.0))
        } else {
            Err ("This CLKComplicationTemplate cannot be downcasted to CLKComplicationTemplateUtilitarianSmallSquare" ,)
        }
    }
}
impl INSObject for CLKComplicationTemplateUtilitarianSmallSquare {}
impl PNSObject for CLKComplicationTemplateUtilitarianSmallSquare {}
impl ICLKComplicationTemplateUtilitarianSmallSquare
    for CLKComplicationTemplateUtilitarianSmallSquare
{
}
pub trait ICLKComplicationTemplateUtilitarianSmallSquare: Sized + std::ops::Deref {
    unsafe fn initWithImageProvider_(&self, imageProvider: CLKImageProvider) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithImageProvider : imageProvider)
    }
    unsafe fn imageProvider(&self) -> CLKImageProvider
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, imageProvider)
    }
    unsafe fn setImageProvider_(&self, imageProvider: CLKImageProvider)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setImageProvider : imageProvider)
    }
    unsafe fn templateWithImageProvider_(imageProvider: CLKImageProvider) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CLKComplicationTemplateUtilitarianSmallSquare").unwrap(), templateWithImageProvider : imageProvider)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CLKComplicationTemplateUtilitarianSmallRingText(pub id);
impl std::ops::Deref for CLKComplicationTemplateUtilitarianSmallRingText {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CLKComplicationTemplateUtilitarianSmallRingText {}
impl CLKComplicationTemplateUtilitarianSmallRingText {
    pub fn alloc() -> Self {
        Self(unsafe {
            msg_send!(&*objc2::runtime::AnyClass::get(c"CLKComplicationTemplateUtilitarianSmallRingText").unwrap(), alloc)
        })
    }
}
impl ICLKComplicationTemplate for CLKComplicationTemplateUtilitarianSmallRingText {}
impl PNSCopying for CLKComplicationTemplateUtilitarianSmallRingText {}
impl std::convert::TryFrom<CLKComplicationTemplate>
    for CLKComplicationTemplateUtilitarianSmallRingText
{
    type Error = &'static str;
    fn try_from(
        parent: CLKComplicationTemplate,
    ) -> Result<CLKComplicationTemplateUtilitarianSmallRingText, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CLKComplicationTemplateUtilitarianSmallRingText").unwrap())
        };
        if is_kind_of {
            Ok(CLKComplicationTemplateUtilitarianSmallRingText(parent.0))
        } else {
            Err ("This CLKComplicationTemplate cannot be downcasted to CLKComplicationTemplateUtilitarianSmallRingText" ,)
        }
    }
}
impl INSObject for CLKComplicationTemplateUtilitarianSmallRingText {}
impl PNSObject for CLKComplicationTemplateUtilitarianSmallRingText {}
impl ICLKComplicationTemplateUtilitarianSmallRingText
    for CLKComplicationTemplateUtilitarianSmallRingText
{
}
pub trait ICLKComplicationTemplateUtilitarianSmallRingText: Sized + std::ops::Deref {
    unsafe fn initWithTextProvider_fillFraction_ringStyle_(
        &self,
        textProvider: CLKTextProvider,
        fillFraction: f32,
        ringStyle: CLKComplicationRingStyle,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithTextProvider : textProvider, fillFraction : fillFraction, ringStyle : ringStyle)
    }
    unsafe fn textProvider(&self) -> CLKTextProvider
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, textProvider)
    }
    unsafe fn setTextProvider_(&self, textProvider: CLKTextProvider)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setTextProvider : textProvider)
    }
    unsafe fn fillFraction(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, fillFraction)
    }
    unsafe fn setFillFraction_(&self, fillFraction: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setFillFraction : fillFraction)
    }
    unsafe fn ringStyle(&self) -> CLKComplicationRingStyle
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, ringStyle)
    }
    unsafe fn setRingStyle_(&self, ringStyle: CLKComplicationRingStyle)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRingStyle : ringStyle)
    }
    unsafe fn templateWithTextProvider_fillFraction_ringStyle_(
        textProvider: CLKTextProvider,
        fillFraction: f32,
        ringStyle: CLKComplicationRingStyle,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CLKComplicationTemplateUtilitarianSmallRingText").unwrap(), templateWithTextProvider : textProvider, fillFraction : fillFraction, ringStyle : ringStyle)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CLKComplicationTemplateUtilitarianSmallRingImage(pub id);
impl std::ops::Deref for CLKComplicationTemplateUtilitarianSmallRingImage {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CLKComplicationTemplateUtilitarianSmallRingImage {}
impl CLKComplicationTemplateUtilitarianSmallRingImage {
    pub fn alloc() -> Self {
        Self(unsafe {
            msg_send!(&*objc2::runtime::AnyClass::get(c"CLKComplicationTemplateUtilitarianSmallRingImage").unwrap(), alloc)
        })
    }
}
impl ICLKComplicationTemplate for CLKComplicationTemplateUtilitarianSmallRingImage {}
impl PNSCopying for CLKComplicationTemplateUtilitarianSmallRingImage {}
impl std::convert::TryFrom<CLKComplicationTemplate>
    for CLKComplicationTemplateUtilitarianSmallRingImage
{
    type Error = &'static str;
    fn try_from(
        parent: CLKComplicationTemplate,
    ) -> Result<CLKComplicationTemplateUtilitarianSmallRingImage, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CLKComplicationTemplateUtilitarianSmallRingImage").unwrap())
        };
        if is_kind_of {
            Ok(CLKComplicationTemplateUtilitarianSmallRingImage(parent.0))
        } else {
            Err ("This CLKComplicationTemplate cannot be downcasted to CLKComplicationTemplateUtilitarianSmallRingImage" ,)
        }
    }
}
impl INSObject for CLKComplicationTemplateUtilitarianSmallRingImage {}
impl PNSObject for CLKComplicationTemplateUtilitarianSmallRingImage {}
impl ICLKComplicationTemplateUtilitarianSmallRingImage
    for CLKComplicationTemplateUtilitarianSmallRingImage
{
}
pub trait ICLKComplicationTemplateUtilitarianSmallRingImage: Sized + std::ops::Deref {
    unsafe fn initWithImageProvider_fillFraction_ringStyle_(
        &self,
        imageProvider: CLKImageProvider,
        fillFraction: f32,
        ringStyle: CLKComplicationRingStyle,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithImageProvider : imageProvider, fillFraction : fillFraction, ringStyle : ringStyle)
    }
    unsafe fn imageProvider(&self) -> CLKImageProvider
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, imageProvider)
    }
    unsafe fn setImageProvider_(&self, imageProvider: CLKImageProvider)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setImageProvider : imageProvider)
    }
    unsafe fn fillFraction(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, fillFraction)
    }
    unsafe fn setFillFraction_(&self, fillFraction: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setFillFraction : fillFraction)
    }
    unsafe fn ringStyle(&self) -> CLKComplicationRingStyle
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, ringStyle)
    }
    unsafe fn setRingStyle_(&self, ringStyle: CLKComplicationRingStyle)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRingStyle : ringStyle)
    }
    unsafe fn templateWithImageProvider_fillFraction_ringStyle_(
        imageProvider: CLKImageProvider,
        fillFraction: f32,
        ringStyle: CLKComplicationRingStyle,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CLKComplicationTemplateUtilitarianSmallRingImage").unwrap(), templateWithImageProvider : imageProvider, fillFraction : fillFraction, ringStyle : ringStyle)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CLKComplicationTemplateUtilitarianSmallFlat(pub id);
impl std::ops::Deref for CLKComplicationTemplateUtilitarianSmallFlat {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CLKComplicationTemplateUtilitarianSmallFlat {}
impl CLKComplicationTemplateUtilitarianSmallFlat {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CLKComplicationTemplateUtilitarianSmallFlat").unwrap(), alloc) })
    }
}
impl ICLKComplicationTemplate for CLKComplicationTemplateUtilitarianSmallFlat {}
impl PNSCopying for CLKComplicationTemplateUtilitarianSmallFlat {}
impl std::convert::TryFrom<CLKComplicationTemplate>
    for CLKComplicationTemplateUtilitarianSmallFlat
{
    type Error = &'static str;
    fn try_from(
        parent: CLKComplicationTemplate,
    ) -> Result<CLKComplicationTemplateUtilitarianSmallFlat, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CLKComplicationTemplateUtilitarianSmallFlat").unwrap())
        };
        if is_kind_of {
            Ok(CLKComplicationTemplateUtilitarianSmallFlat(parent.0))
        } else {
            Err ("This CLKComplicationTemplate cannot be downcasted to CLKComplicationTemplateUtilitarianSmallFlat" ,)
        }
    }
}
impl INSObject for CLKComplicationTemplateUtilitarianSmallFlat {}
impl PNSObject for CLKComplicationTemplateUtilitarianSmallFlat {}
impl ICLKComplicationTemplateUtilitarianSmallFlat for CLKComplicationTemplateUtilitarianSmallFlat {}
pub trait ICLKComplicationTemplateUtilitarianSmallFlat: Sized + std::ops::Deref {
    unsafe fn initWithTextProvider_(&self, textProvider: CLKTextProvider) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithTextProvider : textProvider)
    }
    unsafe fn initWithTextProvider_imageProvider_(
        &self,
        textProvider: CLKTextProvider,
        imageProvider: CLKImageProvider,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithTextProvider : textProvider, imageProvider : imageProvider)
    }
    unsafe fn textProvider(&self) -> CLKTextProvider
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, textProvider)
    }
    unsafe fn setTextProvider_(&self, textProvider: CLKTextProvider)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setTextProvider : textProvider)
    }
    unsafe fn imageProvider(&self) -> CLKImageProvider
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, imageProvider)
    }
    unsafe fn setImageProvider_(&self, imageProvider: CLKImageProvider)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setImageProvider : imageProvider)
    }
    unsafe fn templateWithTextProvider_(textProvider: CLKTextProvider) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CLKComplicationTemplateUtilitarianSmallFlat").unwrap(), templateWithTextProvider : textProvider)
    }
    unsafe fn templateWithTextProvider_imageProvider_(
        textProvider: CLKTextProvider,
        imageProvider: CLKImageProvider,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CLKComplicationTemplateUtilitarianSmallFlat").unwrap(), templateWithTextProvider : textProvider, imageProvider : imageProvider)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CLKComplicationTemplateUtilitarianLargeFlat(pub id);
impl std::ops::Deref for CLKComplicationTemplateUtilitarianLargeFlat {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CLKComplicationTemplateUtilitarianLargeFlat {}
impl CLKComplicationTemplateUtilitarianLargeFlat {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CLKComplicationTemplateUtilitarianLargeFlat").unwrap(), alloc) })
    }
}
impl ICLKComplicationTemplate for CLKComplicationTemplateUtilitarianLargeFlat {}
impl PNSCopying for CLKComplicationTemplateUtilitarianLargeFlat {}
impl std::convert::TryFrom<CLKComplicationTemplate>
    for CLKComplicationTemplateUtilitarianLargeFlat
{
    type Error = &'static str;
    fn try_from(
        parent: CLKComplicationTemplate,
    ) -> Result<CLKComplicationTemplateUtilitarianLargeFlat, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CLKComplicationTemplateUtilitarianLargeFlat").unwrap())
        };
        if is_kind_of {
            Ok(CLKComplicationTemplateUtilitarianLargeFlat(parent.0))
        } else {
            Err ("This CLKComplicationTemplate cannot be downcasted to CLKComplicationTemplateUtilitarianLargeFlat" ,)
        }
    }
}
impl INSObject for CLKComplicationTemplateUtilitarianLargeFlat {}
impl PNSObject for CLKComplicationTemplateUtilitarianLargeFlat {}
impl ICLKComplicationTemplateUtilitarianLargeFlat for CLKComplicationTemplateUtilitarianLargeFlat {}
pub trait ICLKComplicationTemplateUtilitarianLargeFlat: Sized + std::ops::Deref {
    unsafe fn initWithTextProvider_(&self, textProvider: CLKTextProvider) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithTextProvider : textProvider)
    }
    unsafe fn initWithTextProvider_imageProvider_(
        &self,
        textProvider: CLKTextProvider,
        imageProvider: CLKImageProvider,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithTextProvider : textProvider, imageProvider : imageProvider)
    }
    unsafe fn textProvider(&self) -> CLKTextProvider
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, textProvider)
    }
    unsafe fn setTextProvider_(&self, textProvider: CLKTextProvider)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setTextProvider : textProvider)
    }
    unsafe fn imageProvider(&self) -> CLKImageProvider
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, imageProvider)
    }
    unsafe fn setImageProvider_(&self, imageProvider: CLKImageProvider)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setImageProvider : imageProvider)
    }
    unsafe fn templateWithTextProvider_(textProvider: CLKTextProvider) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CLKComplicationTemplateUtilitarianLargeFlat").unwrap(), templateWithTextProvider : textProvider)
    }
    unsafe fn templateWithTextProvider_imageProvider_(
        textProvider: CLKTextProvider,
        imageProvider: CLKImageProvider,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CLKComplicationTemplateUtilitarianLargeFlat").unwrap(), templateWithTextProvider : textProvider, imageProvider : imageProvider)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CLKComplicationTemplateCircularSmallSimpleText(pub id);
impl std::ops::Deref for CLKComplicationTemplateCircularSmallSimpleText {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CLKComplicationTemplateCircularSmallSimpleText {}
impl CLKComplicationTemplateCircularSmallSimpleText {
    pub fn alloc() -> Self {
        Self(unsafe {
            msg_send!(&*objc2::runtime::AnyClass::get(c"CLKComplicationTemplateCircularSmallSimpleText").unwrap(), alloc)
        })
    }
}
impl ICLKComplicationTemplate for CLKComplicationTemplateCircularSmallSimpleText {}
impl PNSCopying for CLKComplicationTemplateCircularSmallSimpleText {}
impl std::convert::TryFrom<CLKComplicationTemplate>
    for CLKComplicationTemplateCircularSmallSimpleText
{
    type Error = &'static str;
    fn try_from(
        parent: CLKComplicationTemplate,
    ) -> Result<CLKComplicationTemplateCircularSmallSimpleText, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CLKComplicationTemplateCircularSmallSimpleText").unwrap())
        };
        if is_kind_of {
            Ok(CLKComplicationTemplateCircularSmallSimpleText(parent.0))
        } else {
            Err ("This CLKComplicationTemplate cannot be downcasted to CLKComplicationTemplateCircularSmallSimpleText" ,)
        }
    }
}
impl INSObject for CLKComplicationTemplateCircularSmallSimpleText {}
impl PNSObject for CLKComplicationTemplateCircularSmallSimpleText {}
impl ICLKComplicationTemplateCircularSmallSimpleText
    for CLKComplicationTemplateCircularSmallSimpleText
{
}
pub trait ICLKComplicationTemplateCircularSmallSimpleText: Sized + std::ops::Deref {
    unsafe fn initWithTextProvider_(&self, textProvider: CLKTextProvider) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithTextProvider : textProvider)
    }
    unsafe fn textProvider(&self) -> CLKTextProvider
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, textProvider)
    }
    unsafe fn setTextProvider_(&self, textProvider: CLKTextProvider)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setTextProvider : textProvider)
    }
    unsafe fn templateWithTextProvider_(textProvider: CLKTextProvider) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CLKComplicationTemplateCircularSmallSimpleText").unwrap(), templateWithTextProvider : textProvider)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CLKComplicationTemplateCircularSmallSimpleImage(pub id);
impl std::ops::Deref for CLKComplicationTemplateCircularSmallSimpleImage {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CLKComplicationTemplateCircularSmallSimpleImage {}
impl CLKComplicationTemplateCircularSmallSimpleImage {
    pub fn alloc() -> Self {
        Self(unsafe {
            msg_send!(&*objc2::runtime::AnyClass::get(c"CLKComplicationTemplateCircularSmallSimpleImage").unwrap(), alloc)
        })
    }
}
impl ICLKComplicationTemplate for CLKComplicationTemplateCircularSmallSimpleImage {}
impl PNSCopying for CLKComplicationTemplateCircularSmallSimpleImage {}
impl std::convert::TryFrom<CLKComplicationTemplate>
    for CLKComplicationTemplateCircularSmallSimpleImage
{
    type Error = &'static str;
    fn try_from(
        parent: CLKComplicationTemplate,
    ) -> Result<CLKComplicationTemplateCircularSmallSimpleImage, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CLKComplicationTemplateCircularSmallSimpleImage").unwrap())
        };
        if is_kind_of {
            Ok(CLKComplicationTemplateCircularSmallSimpleImage(parent.0))
        } else {
            Err ("This CLKComplicationTemplate cannot be downcasted to CLKComplicationTemplateCircularSmallSimpleImage" ,)
        }
    }
}
impl INSObject for CLKComplicationTemplateCircularSmallSimpleImage {}
impl PNSObject for CLKComplicationTemplateCircularSmallSimpleImage {}
impl ICLKComplicationTemplateCircularSmallSimpleImage
    for CLKComplicationTemplateCircularSmallSimpleImage
{
}
pub trait ICLKComplicationTemplateCircularSmallSimpleImage: Sized + std::ops::Deref {
    unsafe fn initWithImageProvider_(&self, imageProvider: CLKImageProvider) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithImageProvider : imageProvider)
    }
    unsafe fn imageProvider(&self) -> CLKImageProvider
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, imageProvider)
    }
    unsafe fn setImageProvider_(&self, imageProvider: CLKImageProvider)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setImageProvider : imageProvider)
    }
    unsafe fn templateWithImageProvider_(imageProvider: CLKImageProvider) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CLKComplicationTemplateCircularSmallSimpleImage").unwrap(), templateWithImageProvider : imageProvider)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CLKComplicationTemplateCircularSmallRingText(pub id);
impl std::ops::Deref for CLKComplicationTemplateCircularSmallRingText {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CLKComplicationTemplateCircularSmallRingText {}
impl CLKComplicationTemplateCircularSmallRingText {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CLKComplicationTemplateCircularSmallRingText").unwrap(), alloc) })
    }
}
impl ICLKComplicationTemplate for CLKComplicationTemplateCircularSmallRingText {}
impl PNSCopying for CLKComplicationTemplateCircularSmallRingText {}
impl std::convert::TryFrom<CLKComplicationTemplate>
    for CLKComplicationTemplateCircularSmallRingText
{
    type Error = &'static str;
    fn try_from(
        parent: CLKComplicationTemplate,
    ) -> Result<CLKComplicationTemplateCircularSmallRingText, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CLKComplicationTemplateCircularSmallRingText").unwrap())
        };
        if is_kind_of {
            Ok(CLKComplicationTemplateCircularSmallRingText(parent.0))
        } else {
            Err ("This CLKComplicationTemplate cannot be downcasted to CLKComplicationTemplateCircularSmallRingText" ,)
        }
    }
}
impl INSObject for CLKComplicationTemplateCircularSmallRingText {}
impl PNSObject for CLKComplicationTemplateCircularSmallRingText {}
impl ICLKComplicationTemplateCircularSmallRingText
    for CLKComplicationTemplateCircularSmallRingText
{
}
pub trait ICLKComplicationTemplateCircularSmallRingText: Sized + std::ops::Deref {
    unsafe fn initWithTextProvider_fillFraction_ringStyle_(
        &self,
        textProvider: CLKTextProvider,
        fillFraction: f32,
        ringStyle: CLKComplicationRingStyle,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithTextProvider : textProvider, fillFraction : fillFraction, ringStyle : ringStyle)
    }
    unsafe fn textProvider(&self) -> CLKTextProvider
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, textProvider)
    }
    unsafe fn setTextProvider_(&self, textProvider: CLKTextProvider)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setTextProvider : textProvider)
    }
    unsafe fn fillFraction(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, fillFraction)
    }
    unsafe fn setFillFraction_(&self, fillFraction: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setFillFraction : fillFraction)
    }
    unsafe fn ringStyle(&self) -> CLKComplicationRingStyle
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, ringStyle)
    }
    unsafe fn setRingStyle_(&self, ringStyle: CLKComplicationRingStyle)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRingStyle : ringStyle)
    }
    unsafe fn templateWithTextProvider_fillFraction_ringStyle_(
        textProvider: CLKTextProvider,
        fillFraction: f32,
        ringStyle: CLKComplicationRingStyle,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CLKComplicationTemplateCircularSmallRingText").unwrap(), templateWithTextProvider : textProvider, fillFraction : fillFraction, ringStyle : ringStyle)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CLKComplicationTemplateCircularSmallRingImage(pub id);
impl std::ops::Deref for CLKComplicationTemplateCircularSmallRingImage {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CLKComplicationTemplateCircularSmallRingImage {}
impl CLKComplicationTemplateCircularSmallRingImage {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CLKComplicationTemplateCircularSmallRingImage").unwrap(), alloc) })
    }
}
impl ICLKComplicationTemplate for CLKComplicationTemplateCircularSmallRingImage {}
impl PNSCopying for CLKComplicationTemplateCircularSmallRingImage {}
impl std::convert::TryFrom<CLKComplicationTemplate>
    for CLKComplicationTemplateCircularSmallRingImage
{
    type Error = &'static str;
    fn try_from(
        parent: CLKComplicationTemplate,
    ) -> Result<CLKComplicationTemplateCircularSmallRingImage, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CLKComplicationTemplateCircularSmallRingImage").unwrap())
        };
        if is_kind_of {
            Ok(CLKComplicationTemplateCircularSmallRingImage(parent.0))
        } else {
            Err ("This CLKComplicationTemplate cannot be downcasted to CLKComplicationTemplateCircularSmallRingImage" ,)
        }
    }
}
impl INSObject for CLKComplicationTemplateCircularSmallRingImage {}
impl PNSObject for CLKComplicationTemplateCircularSmallRingImage {}
impl ICLKComplicationTemplateCircularSmallRingImage
    for CLKComplicationTemplateCircularSmallRingImage
{
}
pub trait ICLKComplicationTemplateCircularSmallRingImage: Sized + std::ops::Deref {
    unsafe fn initWithImageProvider_fillFraction_ringStyle_(
        &self,
        imageProvider: CLKImageProvider,
        fillFraction: f32,
        ringStyle: CLKComplicationRingStyle,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithImageProvider : imageProvider, fillFraction : fillFraction, ringStyle : ringStyle)
    }
    unsafe fn imageProvider(&self) -> CLKImageProvider
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, imageProvider)
    }
    unsafe fn setImageProvider_(&self, imageProvider: CLKImageProvider)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setImageProvider : imageProvider)
    }
    unsafe fn fillFraction(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, fillFraction)
    }
    unsafe fn setFillFraction_(&self, fillFraction: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setFillFraction : fillFraction)
    }
    unsafe fn ringStyle(&self) -> CLKComplicationRingStyle
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, ringStyle)
    }
    unsafe fn setRingStyle_(&self, ringStyle: CLKComplicationRingStyle)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRingStyle : ringStyle)
    }
    unsafe fn templateWithImageProvider_fillFraction_ringStyle_(
        imageProvider: CLKImageProvider,
        fillFraction: f32,
        ringStyle: CLKComplicationRingStyle,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CLKComplicationTemplateCircularSmallRingImage").unwrap(), templateWithImageProvider : imageProvider, fillFraction : fillFraction, ringStyle : ringStyle)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CLKComplicationTemplateCircularSmallStackText(pub id);
impl std::ops::Deref for CLKComplicationTemplateCircularSmallStackText {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CLKComplicationTemplateCircularSmallStackText {}
impl CLKComplicationTemplateCircularSmallStackText {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CLKComplicationTemplateCircularSmallStackText").unwrap(), alloc) })
    }
}
impl ICLKComplicationTemplate for CLKComplicationTemplateCircularSmallStackText {}
impl PNSCopying for CLKComplicationTemplateCircularSmallStackText {}
impl std::convert::TryFrom<CLKComplicationTemplate>
    for CLKComplicationTemplateCircularSmallStackText
{
    type Error = &'static str;
    fn try_from(
        parent: CLKComplicationTemplate,
    ) -> Result<CLKComplicationTemplateCircularSmallStackText, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CLKComplicationTemplateCircularSmallStackText").unwrap())
        };
        if is_kind_of {
            Ok(CLKComplicationTemplateCircularSmallStackText(parent.0))
        } else {
            Err ("This CLKComplicationTemplate cannot be downcasted to CLKComplicationTemplateCircularSmallStackText" ,)
        }
    }
}
impl INSObject for CLKComplicationTemplateCircularSmallStackText {}
impl PNSObject for CLKComplicationTemplateCircularSmallStackText {}
impl ICLKComplicationTemplateCircularSmallStackText
    for CLKComplicationTemplateCircularSmallStackText
{
}
pub trait ICLKComplicationTemplateCircularSmallStackText: Sized + std::ops::Deref {
    unsafe fn initWithLine1TextProvider_line2TextProvider_(
        &self,
        line1TextProvider: CLKTextProvider,
        line2TextProvider: CLKTextProvider,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithLine1TextProvider : line1TextProvider, line2TextProvider : line2TextProvider)
    }
    unsafe fn line1TextProvider(&self) -> CLKTextProvider
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, line1TextProvider)
    }
    unsafe fn setLine1TextProvider_(&self, line1TextProvider: CLKTextProvider)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setLine1TextProvider : line1TextProvider)
    }
    unsafe fn line2TextProvider(&self) -> CLKTextProvider
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, line2TextProvider)
    }
    unsafe fn setLine2TextProvider_(&self, line2TextProvider: CLKTextProvider)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setLine2TextProvider : line2TextProvider)
    }
    unsafe fn templateWithLine1TextProvider_line2TextProvider_(
        line1TextProvider: CLKTextProvider,
        line2TextProvider: CLKTextProvider,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CLKComplicationTemplateCircularSmallStackText").unwrap(), templateWithLine1TextProvider : line1TextProvider, line2TextProvider : line2TextProvider)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CLKComplicationTemplateCircularSmallStackImage(pub id);
impl std::ops::Deref for CLKComplicationTemplateCircularSmallStackImage {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CLKComplicationTemplateCircularSmallStackImage {}
impl CLKComplicationTemplateCircularSmallStackImage {
    pub fn alloc() -> Self {
        Self(unsafe {
            msg_send!(&*objc2::runtime::AnyClass::get(c"CLKComplicationTemplateCircularSmallStackImage").unwrap(), alloc)
        })
    }
}
impl ICLKComplicationTemplate for CLKComplicationTemplateCircularSmallStackImage {}
impl PNSCopying for CLKComplicationTemplateCircularSmallStackImage {}
impl std::convert::TryFrom<CLKComplicationTemplate>
    for CLKComplicationTemplateCircularSmallStackImage
{
    type Error = &'static str;
    fn try_from(
        parent: CLKComplicationTemplate,
    ) -> Result<CLKComplicationTemplateCircularSmallStackImage, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CLKComplicationTemplateCircularSmallStackImage").unwrap())
        };
        if is_kind_of {
            Ok(CLKComplicationTemplateCircularSmallStackImage(parent.0))
        } else {
            Err ("This CLKComplicationTemplate cannot be downcasted to CLKComplicationTemplateCircularSmallStackImage" ,)
        }
    }
}
impl INSObject for CLKComplicationTemplateCircularSmallStackImage {}
impl PNSObject for CLKComplicationTemplateCircularSmallStackImage {}
impl ICLKComplicationTemplateCircularSmallStackImage
    for CLKComplicationTemplateCircularSmallStackImage
{
}
pub trait ICLKComplicationTemplateCircularSmallStackImage: Sized + std::ops::Deref {
    unsafe fn initWithLine1ImageProvider_line2TextProvider_(
        &self,
        line1ImageProvider: CLKImageProvider,
        line2TextProvider: CLKTextProvider,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithLine1ImageProvider : line1ImageProvider, line2TextProvider : line2TextProvider)
    }
    unsafe fn line1ImageProvider(&self) -> CLKImageProvider
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, line1ImageProvider)
    }
    unsafe fn setLine1ImageProvider_(&self, line1ImageProvider: CLKImageProvider)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setLine1ImageProvider : line1ImageProvider)
    }
    unsafe fn line2TextProvider(&self) -> CLKTextProvider
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, line2TextProvider)
    }
    unsafe fn setLine2TextProvider_(&self, line2TextProvider: CLKTextProvider)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setLine2TextProvider : line2TextProvider)
    }
    unsafe fn templateWithLine1ImageProvider_line2TextProvider_(
        line1ImageProvider: CLKImageProvider,
        line2TextProvider: CLKTextProvider,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CLKComplicationTemplateCircularSmallStackImage").unwrap(), templateWithLine1ImageProvider : line1ImageProvider, line2TextProvider : line2TextProvider)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CLKComplicationTemplateExtraLargeSimpleText(pub id);
impl std::ops::Deref for CLKComplicationTemplateExtraLargeSimpleText {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CLKComplicationTemplateExtraLargeSimpleText {}
impl CLKComplicationTemplateExtraLargeSimpleText {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CLKComplicationTemplateExtraLargeSimpleText").unwrap(), alloc) })
    }
}
impl ICLKComplicationTemplate for CLKComplicationTemplateExtraLargeSimpleText {}
impl PNSCopying for CLKComplicationTemplateExtraLargeSimpleText {}
impl std::convert::TryFrom<CLKComplicationTemplate>
    for CLKComplicationTemplateExtraLargeSimpleText
{
    type Error = &'static str;
    fn try_from(
        parent: CLKComplicationTemplate,
    ) -> Result<CLKComplicationTemplateExtraLargeSimpleText, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CLKComplicationTemplateExtraLargeSimpleText").unwrap())
        };
        if is_kind_of {
            Ok(CLKComplicationTemplateExtraLargeSimpleText(parent.0))
        } else {
            Err ("This CLKComplicationTemplate cannot be downcasted to CLKComplicationTemplateExtraLargeSimpleText" ,)
        }
    }
}
impl INSObject for CLKComplicationTemplateExtraLargeSimpleText {}
impl PNSObject for CLKComplicationTemplateExtraLargeSimpleText {}
impl ICLKComplicationTemplateExtraLargeSimpleText for CLKComplicationTemplateExtraLargeSimpleText {}
pub trait ICLKComplicationTemplateExtraLargeSimpleText: Sized + std::ops::Deref {
    unsafe fn initWithTextProvider_(&self, textProvider: CLKTextProvider) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithTextProvider : textProvider)
    }
    unsafe fn textProvider(&self) -> CLKTextProvider
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, textProvider)
    }
    unsafe fn setTextProvider_(&self, textProvider: CLKTextProvider)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setTextProvider : textProvider)
    }
    unsafe fn templateWithTextProvider_(textProvider: CLKTextProvider) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CLKComplicationTemplateExtraLargeSimpleText").unwrap(), templateWithTextProvider : textProvider)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CLKComplicationTemplateExtraLargeSimpleImage(pub id);
impl std::ops::Deref for CLKComplicationTemplateExtraLargeSimpleImage {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CLKComplicationTemplateExtraLargeSimpleImage {}
impl CLKComplicationTemplateExtraLargeSimpleImage {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CLKComplicationTemplateExtraLargeSimpleImage").unwrap(), alloc) })
    }
}
impl ICLKComplicationTemplate for CLKComplicationTemplateExtraLargeSimpleImage {}
impl PNSCopying for CLKComplicationTemplateExtraLargeSimpleImage {}
impl std::convert::TryFrom<CLKComplicationTemplate>
    for CLKComplicationTemplateExtraLargeSimpleImage
{
    type Error = &'static str;
    fn try_from(
        parent: CLKComplicationTemplate,
    ) -> Result<CLKComplicationTemplateExtraLargeSimpleImage, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CLKComplicationTemplateExtraLargeSimpleImage").unwrap())
        };
        if is_kind_of {
            Ok(CLKComplicationTemplateExtraLargeSimpleImage(parent.0))
        } else {
            Err ("This CLKComplicationTemplate cannot be downcasted to CLKComplicationTemplateExtraLargeSimpleImage" ,)
        }
    }
}
impl INSObject for CLKComplicationTemplateExtraLargeSimpleImage {}
impl PNSObject for CLKComplicationTemplateExtraLargeSimpleImage {}
impl ICLKComplicationTemplateExtraLargeSimpleImage
    for CLKComplicationTemplateExtraLargeSimpleImage
{
}
pub trait ICLKComplicationTemplateExtraLargeSimpleImage: Sized + std::ops::Deref {
    unsafe fn initWithImageProvider_(&self, imageProvider: CLKImageProvider) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithImageProvider : imageProvider)
    }
    unsafe fn imageProvider(&self) -> CLKImageProvider
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, imageProvider)
    }
    unsafe fn setImageProvider_(&self, imageProvider: CLKImageProvider)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setImageProvider : imageProvider)
    }
    unsafe fn templateWithImageProvider_(imageProvider: CLKImageProvider) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CLKComplicationTemplateExtraLargeSimpleImage").unwrap(), templateWithImageProvider : imageProvider)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CLKComplicationTemplateExtraLargeRingText(pub id);
impl std::ops::Deref for CLKComplicationTemplateExtraLargeRingText {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CLKComplicationTemplateExtraLargeRingText {}
impl CLKComplicationTemplateExtraLargeRingText {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CLKComplicationTemplateExtraLargeRingText").unwrap(), alloc) })
    }
}
impl ICLKComplicationTemplate for CLKComplicationTemplateExtraLargeRingText {}
impl PNSCopying for CLKComplicationTemplateExtraLargeRingText {}
impl std::convert::TryFrom<CLKComplicationTemplate> for CLKComplicationTemplateExtraLargeRingText {
    type Error = &'static str;
    fn try_from(
        parent: CLKComplicationTemplate,
    ) -> Result<CLKComplicationTemplateExtraLargeRingText, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CLKComplicationTemplateExtraLargeRingText").unwrap())
        };
        if is_kind_of {
            Ok(CLKComplicationTemplateExtraLargeRingText(parent.0))
        } else {
            Err ("This CLKComplicationTemplate cannot be downcasted to CLKComplicationTemplateExtraLargeRingText" ,)
        }
    }
}
impl INSObject for CLKComplicationTemplateExtraLargeRingText {}
impl PNSObject for CLKComplicationTemplateExtraLargeRingText {}
impl ICLKComplicationTemplateExtraLargeRingText for CLKComplicationTemplateExtraLargeRingText {}
pub trait ICLKComplicationTemplateExtraLargeRingText: Sized + std::ops::Deref {
    unsafe fn initWithTextProvider_fillFraction_ringStyle_(
        &self,
        textProvider: CLKTextProvider,
        fillFraction: f32,
        ringStyle: CLKComplicationRingStyle,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithTextProvider : textProvider, fillFraction : fillFraction, ringStyle : ringStyle)
    }
    unsafe fn textProvider(&self) -> CLKTextProvider
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, textProvider)
    }
    unsafe fn setTextProvider_(&self, textProvider: CLKTextProvider)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setTextProvider : textProvider)
    }
    unsafe fn fillFraction(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, fillFraction)
    }
    unsafe fn setFillFraction_(&self, fillFraction: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setFillFraction : fillFraction)
    }
    unsafe fn ringStyle(&self) -> CLKComplicationRingStyle
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, ringStyle)
    }
    unsafe fn setRingStyle_(&self, ringStyle: CLKComplicationRingStyle)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRingStyle : ringStyle)
    }
    unsafe fn templateWithTextProvider_fillFraction_ringStyle_(
        textProvider: CLKTextProvider,
        fillFraction: f32,
        ringStyle: CLKComplicationRingStyle,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CLKComplicationTemplateExtraLargeRingText").unwrap(), templateWithTextProvider : textProvider, fillFraction : fillFraction, ringStyle : ringStyle)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CLKComplicationTemplateExtraLargeRingImage(pub id);
impl std::ops::Deref for CLKComplicationTemplateExtraLargeRingImage {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CLKComplicationTemplateExtraLargeRingImage {}
impl CLKComplicationTemplateExtraLargeRingImage {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CLKComplicationTemplateExtraLargeRingImage").unwrap(), alloc) })
    }
}
impl ICLKComplicationTemplate for CLKComplicationTemplateExtraLargeRingImage {}
impl PNSCopying for CLKComplicationTemplateExtraLargeRingImage {}
impl std::convert::TryFrom<CLKComplicationTemplate> for CLKComplicationTemplateExtraLargeRingImage {
    type Error = &'static str;
    fn try_from(
        parent: CLKComplicationTemplate,
    ) -> Result<CLKComplicationTemplateExtraLargeRingImage, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CLKComplicationTemplateExtraLargeRingImage").unwrap())
        };
        if is_kind_of {
            Ok(CLKComplicationTemplateExtraLargeRingImage(parent.0))
        } else {
            Err ("This CLKComplicationTemplate cannot be downcasted to CLKComplicationTemplateExtraLargeRingImage" ,)
        }
    }
}
impl INSObject for CLKComplicationTemplateExtraLargeRingImage {}
impl PNSObject for CLKComplicationTemplateExtraLargeRingImage {}
impl ICLKComplicationTemplateExtraLargeRingImage for CLKComplicationTemplateExtraLargeRingImage {}
pub trait ICLKComplicationTemplateExtraLargeRingImage: Sized + std::ops::Deref {
    unsafe fn initWithImageProvider_fillFraction_ringStyle_(
        &self,
        imageProvider: CLKImageProvider,
        fillFraction: f32,
        ringStyle: CLKComplicationRingStyle,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithImageProvider : imageProvider, fillFraction : fillFraction, ringStyle : ringStyle)
    }
    unsafe fn imageProvider(&self) -> CLKImageProvider
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, imageProvider)
    }
    unsafe fn setImageProvider_(&self, imageProvider: CLKImageProvider)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setImageProvider : imageProvider)
    }
    unsafe fn fillFraction(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, fillFraction)
    }
    unsafe fn setFillFraction_(&self, fillFraction: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setFillFraction : fillFraction)
    }
    unsafe fn ringStyle(&self) -> CLKComplicationRingStyle
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, ringStyle)
    }
    unsafe fn setRingStyle_(&self, ringStyle: CLKComplicationRingStyle)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRingStyle : ringStyle)
    }
    unsafe fn templateWithImageProvider_fillFraction_ringStyle_(
        imageProvider: CLKImageProvider,
        fillFraction: f32,
        ringStyle: CLKComplicationRingStyle,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CLKComplicationTemplateExtraLargeRingImage").unwrap(), templateWithImageProvider : imageProvider, fillFraction : fillFraction, ringStyle : ringStyle)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CLKComplicationTemplateExtraLargeStackText(pub id);
impl std::ops::Deref for CLKComplicationTemplateExtraLargeStackText {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CLKComplicationTemplateExtraLargeStackText {}
impl CLKComplicationTemplateExtraLargeStackText {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CLKComplicationTemplateExtraLargeStackText").unwrap(), alloc) })
    }
}
impl ICLKComplicationTemplate for CLKComplicationTemplateExtraLargeStackText {}
impl PNSCopying for CLKComplicationTemplateExtraLargeStackText {}
impl std::convert::TryFrom<CLKComplicationTemplate> for CLKComplicationTemplateExtraLargeStackText {
    type Error = &'static str;
    fn try_from(
        parent: CLKComplicationTemplate,
    ) -> Result<CLKComplicationTemplateExtraLargeStackText, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CLKComplicationTemplateExtraLargeStackText").unwrap())
        };
        if is_kind_of {
            Ok(CLKComplicationTemplateExtraLargeStackText(parent.0))
        } else {
            Err ("This CLKComplicationTemplate cannot be downcasted to CLKComplicationTemplateExtraLargeStackText" ,)
        }
    }
}
impl INSObject for CLKComplicationTemplateExtraLargeStackText {}
impl PNSObject for CLKComplicationTemplateExtraLargeStackText {}
impl ICLKComplicationTemplateExtraLargeStackText for CLKComplicationTemplateExtraLargeStackText {}
pub trait ICLKComplicationTemplateExtraLargeStackText: Sized + std::ops::Deref {
    unsafe fn initWithLine1TextProvider_line2TextProvider_(
        &self,
        line1TextProvider: CLKTextProvider,
        line2TextProvider: CLKTextProvider,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithLine1TextProvider : line1TextProvider, line2TextProvider : line2TextProvider)
    }
    unsafe fn line1TextProvider(&self) -> CLKTextProvider
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, line1TextProvider)
    }
    unsafe fn setLine1TextProvider_(&self, line1TextProvider: CLKTextProvider)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setLine1TextProvider : line1TextProvider)
    }
    unsafe fn line2TextProvider(&self) -> CLKTextProvider
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, line2TextProvider)
    }
    unsafe fn setLine2TextProvider_(&self, line2TextProvider: CLKTextProvider)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setLine2TextProvider : line2TextProvider)
    }
    unsafe fn highlightLine2(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, highlightLine2)
    }
    unsafe fn setHighlightLine2_(&self, highlightLine2: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setHighlightLine2 : highlightLine2)
    }
    unsafe fn templateWithLine1TextProvider_line2TextProvider_(
        line1TextProvider: CLKTextProvider,
        line2TextProvider: CLKTextProvider,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CLKComplicationTemplateExtraLargeStackText").unwrap(), templateWithLine1TextProvider : line1TextProvider, line2TextProvider : line2TextProvider)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CLKComplicationTemplateExtraLargeStackImage(pub id);
impl std::ops::Deref for CLKComplicationTemplateExtraLargeStackImage {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CLKComplicationTemplateExtraLargeStackImage {}
impl CLKComplicationTemplateExtraLargeStackImage {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CLKComplicationTemplateExtraLargeStackImage").unwrap(), alloc) })
    }
}
impl ICLKComplicationTemplate for CLKComplicationTemplateExtraLargeStackImage {}
impl PNSCopying for CLKComplicationTemplateExtraLargeStackImage {}
impl std::convert::TryFrom<CLKComplicationTemplate>
    for CLKComplicationTemplateExtraLargeStackImage
{
    type Error = &'static str;
    fn try_from(
        parent: CLKComplicationTemplate,
    ) -> Result<CLKComplicationTemplateExtraLargeStackImage, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CLKComplicationTemplateExtraLargeStackImage").unwrap())
        };
        if is_kind_of {
            Ok(CLKComplicationTemplateExtraLargeStackImage(parent.0))
        } else {
            Err ("This CLKComplicationTemplate cannot be downcasted to CLKComplicationTemplateExtraLargeStackImage" ,)
        }
    }
}
impl INSObject for CLKComplicationTemplateExtraLargeStackImage {}
impl PNSObject for CLKComplicationTemplateExtraLargeStackImage {}
impl ICLKComplicationTemplateExtraLargeStackImage for CLKComplicationTemplateExtraLargeStackImage {}
pub trait ICLKComplicationTemplateExtraLargeStackImage: Sized + std::ops::Deref {
    unsafe fn initWithLine1ImageProvider_line2TextProvider_(
        &self,
        line1ImageProvider: CLKImageProvider,
        line2TextProvider: CLKTextProvider,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithLine1ImageProvider : line1ImageProvider, line2TextProvider : line2TextProvider)
    }
    unsafe fn line1ImageProvider(&self) -> CLKImageProvider
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, line1ImageProvider)
    }
    unsafe fn setLine1ImageProvider_(&self, line1ImageProvider: CLKImageProvider)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setLine1ImageProvider : line1ImageProvider)
    }
    unsafe fn line2TextProvider(&self) -> CLKTextProvider
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, line2TextProvider)
    }
    unsafe fn setLine2TextProvider_(&self, line2TextProvider: CLKTextProvider)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setLine2TextProvider : line2TextProvider)
    }
    unsafe fn highlightLine2(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, highlightLine2)
    }
    unsafe fn setHighlightLine2_(&self, highlightLine2: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setHighlightLine2 : highlightLine2)
    }
    unsafe fn templateWithLine1ImageProvider_line2TextProvider_(
        line1ImageProvider: CLKImageProvider,
        line2TextProvider: CLKTextProvider,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CLKComplicationTemplateExtraLargeStackImage").unwrap(), templateWithLine1ImageProvider : line1ImageProvider, line2TextProvider : line2TextProvider)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CLKComplicationTemplateExtraLargeColumnsText(pub id);
impl std::ops::Deref for CLKComplicationTemplateExtraLargeColumnsText {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CLKComplicationTemplateExtraLargeColumnsText {}
impl CLKComplicationTemplateExtraLargeColumnsText {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CLKComplicationTemplateExtraLargeColumnsText").unwrap(), alloc) })
    }
}
impl ICLKComplicationTemplate for CLKComplicationTemplateExtraLargeColumnsText {}
impl PNSCopying for CLKComplicationTemplateExtraLargeColumnsText {}
impl std::convert::TryFrom<CLKComplicationTemplate>
    for CLKComplicationTemplateExtraLargeColumnsText
{
    type Error = &'static str;
    fn try_from(
        parent: CLKComplicationTemplate,
    ) -> Result<CLKComplicationTemplateExtraLargeColumnsText, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CLKComplicationTemplateExtraLargeColumnsText").unwrap())
        };
        if is_kind_of {
            Ok(CLKComplicationTemplateExtraLargeColumnsText(parent.0))
        } else {
            Err ("This CLKComplicationTemplate cannot be downcasted to CLKComplicationTemplateExtraLargeColumnsText" ,)
        }
    }
}
impl INSObject for CLKComplicationTemplateExtraLargeColumnsText {}
impl PNSObject for CLKComplicationTemplateExtraLargeColumnsText {}
impl ICLKComplicationTemplateExtraLargeColumnsText
    for CLKComplicationTemplateExtraLargeColumnsText
{
}
pub trait ICLKComplicationTemplateExtraLargeColumnsText: Sized + std::ops::Deref {
    unsafe fn initWithRow1Column1TextProvider_row1Column2TextProvider_row2Column1TextProvider_row2Column2TextProvider_(
        &self,
        row1Column1TextProvider: CLKTextProvider,
        row1Column2TextProvider: CLKTextProvider,
        row2Column1TextProvider: CLKTextProvider,
        row2Column2TextProvider: CLKTextProvider,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithRow1Column1TextProvider : row1Column1TextProvider, row1Column2TextProvider : row1Column2TextProvider, row2Column1TextProvider : row2Column1TextProvider, row2Column2TextProvider : row2Column2TextProvider)
    }
    unsafe fn row1Column1TextProvider(&self) -> CLKTextProvider
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, row1Column1TextProvider)
    }
    unsafe fn setRow1Column1TextProvider_(&self, row1Column1TextProvider: CLKTextProvider)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRow1Column1TextProvider : row1Column1TextProvider)
    }
    unsafe fn row1Column2TextProvider(&self) -> CLKTextProvider
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, row1Column2TextProvider)
    }
    unsafe fn setRow1Column2TextProvider_(&self, row1Column2TextProvider: CLKTextProvider)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRow1Column2TextProvider : row1Column2TextProvider)
    }
    unsafe fn row2Column1TextProvider(&self) -> CLKTextProvider
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, row2Column1TextProvider)
    }
    unsafe fn setRow2Column1TextProvider_(&self, row2Column1TextProvider: CLKTextProvider)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRow2Column1TextProvider : row2Column1TextProvider)
    }
    unsafe fn row2Column2TextProvider(&self) -> CLKTextProvider
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, row2Column2TextProvider)
    }
    unsafe fn setRow2Column2TextProvider_(&self, row2Column2TextProvider: CLKTextProvider)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRow2Column2TextProvider : row2Column2TextProvider)
    }
    unsafe fn column2Alignment(&self) -> CLKComplicationColumnAlignment
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, column2Alignment)
    }
    unsafe fn setColumn2Alignment_(&self, column2Alignment: CLKComplicationColumnAlignment)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setColumn2Alignment : column2Alignment)
    }
    unsafe fn highlightColumn2(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, highlightColumn2)
    }
    unsafe fn setHighlightColumn2_(&self, highlightColumn2: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setHighlightColumn2 : highlightColumn2)
    }
    unsafe fn templateWithRow1Column1TextProvider_row1Column2TextProvider_row2Column1TextProvider_row2Column2TextProvider_(
        row1Column1TextProvider: CLKTextProvider,
        row1Column2TextProvider: CLKTextProvider,
        row2Column1TextProvider: CLKTextProvider,
        row2Column2TextProvider: CLKTextProvider,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CLKComplicationTemplateExtraLargeColumnsText").unwrap(), templateWithRow1Column1TextProvider : row1Column1TextProvider, row1Column2TextProvider : row1Column2TextProvider, row2Column1TextProvider : row2Column1TextProvider, row2Column2TextProvider : row2Column2TextProvider)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CLKComplicationTemplateGraphicCornerGaugeText(pub id);
impl std::ops::Deref for CLKComplicationTemplateGraphicCornerGaugeText {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CLKComplicationTemplateGraphicCornerGaugeText {}
impl CLKComplicationTemplateGraphicCornerGaugeText {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CLKComplicationTemplateGraphicCornerGaugeText").unwrap(), alloc) })
    }
}
impl ICLKComplicationTemplate for CLKComplicationTemplateGraphicCornerGaugeText {}
impl PNSCopying for CLKComplicationTemplateGraphicCornerGaugeText {}
impl std::convert::TryFrom<CLKComplicationTemplate>
    for CLKComplicationTemplateGraphicCornerGaugeText
{
    type Error = &'static str;
    fn try_from(
        parent: CLKComplicationTemplate,
    ) -> Result<CLKComplicationTemplateGraphicCornerGaugeText, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CLKComplicationTemplateGraphicCornerGaugeText").unwrap())
        };
        if is_kind_of {
            Ok(CLKComplicationTemplateGraphicCornerGaugeText(parent.0))
        } else {
            Err ("This CLKComplicationTemplate cannot be downcasted to CLKComplicationTemplateGraphicCornerGaugeText" ,)
        }
    }
}
impl INSObject for CLKComplicationTemplateGraphicCornerGaugeText {}
impl PNSObject for CLKComplicationTemplateGraphicCornerGaugeText {}
impl ICLKComplicationTemplateGraphicCornerGaugeText
    for CLKComplicationTemplateGraphicCornerGaugeText
{
}
pub trait ICLKComplicationTemplateGraphicCornerGaugeText: Sized + std::ops::Deref {
    unsafe fn initWithGaugeProvider_outerTextProvider_(
        &self,
        gaugeProvider: CLKGaugeProvider,
        outerTextProvider: CLKTextProvider,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithGaugeProvider : gaugeProvider, outerTextProvider : outerTextProvider)
    }
    unsafe fn initWithGaugeProvider_leadingTextProvider_trailingTextProvider_outerTextProvider_(
        &self,
        gaugeProvider: CLKGaugeProvider,
        leadingTextProvider: CLKTextProvider,
        trailingTextProvider: CLKTextProvider,
        outerTextProvider: CLKTextProvider,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithGaugeProvider : gaugeProvider, leadingTextProvider : leadingTextProvider, trailingTextProvider : trailingTextProvider, outerTextProvider : outerTextProvider)
    }
    unsafe fn gaugeProvider(&self) -> CLKGaugeProvider
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, gaugeProvider)
    }
    unsafe fn setGaugeProvider_(&self, gaugeProvider: CLKGaugeProvider)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setGaugeProvider : gaugeProvider)
    }
    unsafe fn leadingTextProvider(&self) -> CLKTextProvider
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, leadingTextProvider)
    }
    unsafe fn setLeadingTextProvider_(&self, leadingTextProvider: CLKTextProvider)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setLeadingTextProvider : leadingTextProvider)
    }
    unsafe fn trailingTextProvider(&self) -> CLKTextProvider
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, trailingTextProvider)
    }
    unsafe fn setTrailingTextProvider_(&self, trailingTextProvider: CLKTextProvider)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setTrailingTextProvider : trailingTextProvider)
    }
    unsafe fn outerTextProvider(&self) -> CLKTextProvider
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, outerTextProvider)
    }
    unsafe fn setOuterTextProvider_(&self, outerTextProvider: CLKTextProvider)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setOuterTextProvider : outerTextProvider)
    }
    unsafe fn templateWithGaugeProvider_outerTextProvider_(
        gaugeProvider: CLKGaugeProvider,
        outerTextProvider: CLKTextProvider,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CLKComplicationTemplateGraphicCornerGaugeText").unwrap(), templateWithGaugeProvider : gaugeProvider, outerTextProvider : outerTextProvider)
    }
    unsafe fn templateWithGaugeProvider_leadingTextProvider_trailingTextProvider_outerTextProvider_(
        gaugeProvider: CLKGaugeProvider,
        leadingTextProvider: CLKTextProvider,
        trailingTextProvider: CLKTextProvider,
        outerTextProvider: CLKTextProvider,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CLKComplicationTemplateGraphicCornerGaugeText").unwrap(), templateWithGaugeProvider : gaugeProvider, leadingTextProvider : leadingTextProvider, trailingTextProvider : trailingTextProvider, outerTextProvider : outerTextProvider)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CLKComplicationTemplateGraphicCornerGaugeImage(pub id);
impl std::ops::Deref for CLKComplicationTemplateGraphicCornerGaugeImage {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CLKComplicationTemplateGraphicCornerGaugeImage {}
impl CLKComplicationTemplateGraphicCornerGaugeImage {
    pub fn alloc() -> Self {
        Self(unsafe {
            msg_send!(&*objc2::runtime::AnyClass::get(c"CLKComplicationTemplateGraphicCornerGaugeImage").unwrap(), alloc)
        })
    }
}
impl ICLKComplicationTemplate for CLKComplicationTemplateGraphicCornerGaugeImage {}
impl PNSCopying for CLKComplicationTemplateGraphicCornerGaugeImage {}
impl std::convert::TryFrom<CLKComplicationTemplate>
    for CLKComplicationTemplateGraphicCornerGaugeImage
{
    type Error = &'static str;
    fn try_from(
        parent: CLKComplicationTemplate,
    ) -> Result<CLKComplicationTemplateGraphicCornerGaugeImage, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CLKComplicationTemplateGraphicCornerGaugeImage").unwrap())
        };
        if is_kind_of {
            Ok(CLKComplicationTemplateGraphicCornerGaugeImage(parent.0))
        } else {
            Err ("This CLKComplicationTemplate cannot be downcasted to CLKComplicationTemplateGraphicCornerGaugeImage" ,)
        }
    }
}
impl INSObject for CLKComplicationTemplateGraphicCornerGaugeImage {}
impl PNSObject for CLKComplicationTemplateGraphicCornerGaugeImage {}
impl ICLKComplicationTemplateGraphicCornerGaugeImage
    for CLKComplicationTemplateGraphicCornerGaugeImage
{
}
pub trait ICLKComplicationTemplateGraphicCornerGaugeImage: Sized + std::ops::Deref {
    unsafe fn initWithGaugeProvider_imageProvider_(
        &self,
        gaugeProvider: CLKGaugeProvider,
        imageProvider: CLKFullColorImageProvider,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithGaugeProvider : gaugeProvider, imageProvider : imageProvider)
    }
    unsafe fn initWithGaugeProvider_leadingTextProvider_trailingTextProvider_imageProvider_(
        &self,
        gaugeProvider: CLKGaugeProvider,
        leadingTextProvider: CLKTextProvider,
        trailingTextProvider: CLKTextProvider,
        imageProvider: CLKFullColorImageProvider,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithGaugeProvider : gaugeProvider, leadingTextProvider : leadingTextProvider, trailingTextProvider : trailingTextProvider, imageProvider : imageProvider)
    }
    unsafe fn gaugeProvider(&self) -> CLKGaugeProvider
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, gaugeProvider)
    }
    unsafe fn setGaugeProvider_(&self, gaugeProvider: CLKGaugeProvider)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setGaugeProvider : gaugeProvider)
    }
    unsafe fn leadingTextProvider(&self) -> CLKTextProvider
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, leadingTextProvider)
    }
    unsafe fn setLeadingTextProvider_(&self, leadingTextProvider: CLKTextProvider)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setLeadingTextProvider : leadingTextProvider)
    }
    unsafe fn trailingTextProvider(&self) -> CLKTextProvider
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, trailingTextProvider)
    }
    unsafe fn setTrailingTextProvider_(&self, trailingTextProvider: CLKTextProvider)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setTrailingTextProvider : trailingTextProvider)
    }
    unsafe fn imageProvider(&self) -> CLKFullColorImageProvider
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, imageProvider)
    }
    unsafe fn setImageProvider_(&self, imageProvider: CLKFullColorImageProvider)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setImageProvider : imageProvider)
    }
    unsafe fn templateWithGaugeProvider_imageProvider_(
        gaugeProvider: CLKGaugeProvider,
        imageProvider: CLKFullColorImageProvider,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CLKComplicationTemplateGraphicCornerGaugeImage").unwrap(), templateWithGaugeProvider : gaugeProvider, imageProvider : imageProvider)
    }
    unsafe fn templateWithGaugeProvider_leadingTextProvider_trailingTextProvider_imageProvider_(
        gaugeProvider: CLKGaugeProvider,
        leadingTextProvider: CLKTextProvider,
        trailingTextProvider: CLKTextProvider,
        imageProvider: CLKFullColorImageProvider,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CLKComplicationTemplateGraphicCornerGaugeImage").unwrap(), templateWithGaugeProvider : gaugeProvider, leadingTextProvider : leadingTextProvider, trailingTextProvider : trailingTextProvider, imageProvider : imageProvider)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CLKComplicationTemplateGraphicCornerTextImage(pub id);
impl std::ops::Deref for CLKComplicationTemplateGraphicCornerTextImage {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CLKComplicationTemplateGraphicCornerTextImage {}
impl CLKComplicationTemplateGraphicCornerTextImage {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CLKComplicationTemplateGraphicCornerTextImage").unwrap(), alloc) })
    }
}
impl ICLKComplicationTemplate for CLKComplicationTemplateGraphicCornerTextImage {}
impl PNSCopying for CLKComplicationTemplateGraphicCornerTextImage {}
impl std::convert::TryFrom<CLKComplicationTemplate>
    for CLKComplicationTemplateGraphicCornerTextImage
{
    type Error = &'static str;
    fn try_from(
        parent: CLKComplicationTemplate,
    ) -> Result<CLKComplicationTemplateGraphicCornerTextImage, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CLKComplicationTemplateGraphicCornerTextImage").unwrap())
        };
        if is_kind_of {
            Ok(CLKComplicationTemplateGraphicCornerTextImage(parent.0))
        } else {
            Err ("This CLKComplicationTemplate cannot be downcasted to CLKComplicationTemplateGraphicCornerTextImage" ,)
        }
    }
}
impl INSObject for CLKComplicationTemplateGraphicCornerTextImage {}
impl PNSObject for CLKComplicationTemplateGraphicCornerTextImage {}
impl ICLKComplicationTemplateGraphicCornerTextImage
    for CLKComplicationTemplateGraphicCornerTextImage
{
}
pub trait ICLKComplicationTemplateGraphicCornerTextImage: Sized + std::ops::Deref {
    unsafe fn initWithTextProvider_imageProvider_(
        &self,
        textProvider: CLKTextProvider,
        imageProvider: CLKFullColorImageProvider,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithTextProvider : textProvider, imageProvider : imageProvider)
    }
    unsafe fn textProvider(&self) -> CLKTextProvider
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, textProvider)
    }
    unsafe fn setTextProvider_(&self, textProvider: CLKTextProvider)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setTextProvider : textProvider)
    }
    unsafe fn imageProvider(&self) -> CLKFullColorImageProvider
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, imageProvider)
    }
    unsafe fn setImageProvider_(&self, imageProvider: CLKFullColorImageProvider)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setImageProvider : imageProvider)
    }
    unsafe fn templateWithTextProvider_imageProvider_(
        textProvider: CLKTextProvider,
        imageProvider: CLKFullColorImageProvider,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CLKComplicationTemplateGraphicCornerTextImage").unwrap(), templateWithTextProvider : textProvider, imageProvider : imageProvider)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CLKComplicationTemplateGraphicCornerStackText(pub id);
impl std::ops::Deref for CLKComplicationTemplateGraphicCornerStackText {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CLKComplicationTemplateGraphicCornerStackText {}
impl CLKComplicationTemplateGraphicCornerStackText {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CLKComplicationTemplateGraphicCornerStackText").unwrap(), alloc) })
    }
}
impl ICLKComplicationTemplate for CLKComplicationTemplateGraphicCornerStackText {}
impl PNSCopying for CLKComplicationTemplateGraphicCornerStackText {}
impl std::convert::TryFrom<CLKComplicationTemplate>
    for CLKComplicationTemplateGraphicCornerStackText
{
    type Error = &'static str;
    fn try_from(
        parent: CLKComplicationTemplate,
    ) -> Result<CLKComplicationTemplateGraphicCornerStackText, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CLKComplicationTemplateGraphicCornerStackText").unwrap())
        };
        if is_kind_of {
            Ok(CLKComplicationTemplateGraphicCornerStackText(parent.0))
        } else {
            Err ("This CLKComplicationTemplate cannot be downcasted to CLKComplicationTemplateGraphicCornerStackText" ,)
        }
    }
}
impl INSObject for CLKComplicationTemplateGraphicCornerStackText {}
impl PNSObject for CLKComplicationTemplateGraphicCornerStackText {}
impl ICLKComplicationTemplateGraphicCornerStackText
    for CLKComplicationTemplateGraphicCornerStackText
{
}
pub trait ICLKComplicationTemplateGraphicCornerStackText: Sized + std::ops::Deref {
    unsafe fn initWithInnerTextProvider_outerTextProvider_(
        &self,
        innerTextProvider: CLKTextProvider,
        outerTextProvider: CLKTextProvider,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithInnerTextProvider : innerTextProvider, outerTextProvider : outerTextProvider)
    }
    unsafe fn innerTextProvider(&self) -> CLKTextProvider
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, innerTextProvider)
    }
    unsafe fn setInnerTextProvider_(&self, innerTextProvider: CLKTextProvider)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInnerTextProvider : innerTextProvider)
    }
    unsafe fn outerTextProvider(&self) -> CLKTextProvider
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, outerTextProvider)
    }
    unsafe fn setOuterTextProvider_(&self, outerTextProvider: CLKTextProvider)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setOuterTextProvider : outerTextProvider)
    }
    unsafe fn templateWithInnerTextProvider_outerTextProvider_(
        innerTextProvider: CLKTextProvider,
        outerTextProvider: CLKTextProvider,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CLKComplicationTemplateGraphicCornerStackText").unwrap(), templateWithInnerTextProvider : innerTextProvider, outerTextProvider : outerTextProvider)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CLKComplicationTemplateGraphicCornerCircularImage(pub id);
impl std::ops::Deref for CLKComplicationTemplateGraphicCornerCircularImage {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CLKComplicationTemplateGraphicCornerCircularImage {}
impl CLKComplicationTemplateGraphicCornerCircularImage {
    pub fn alloc() -> Self {
        Self(unsafe {
            msg_send!(&*objc2::runtime::AnyClass::get(c"CLKComplicationTemplateGraphicCornerCircularImage").unwrap(), alloc)
        })
    }
}
impl ICLKComplicationTemplate for CLKComplicationTemplateGraphicCornerCircularImage {}
impl PNSCopying for CLKComplicationTemplateGraphicCornerCircularImage {}
impl std::convert::TryFrom<CLKComplicationTemplate>
    for CLKComplicationTemplateGraphicCornerCircularImage
{
    type Error = &'static str;
    fn try_from(
        parent: CLKComplicationTemplate,
    ) -> Result<CLKComplicationTemplateGraphicCornerCircularImage, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CLKComplicationTemplateGraphicCornerCircularImage").unwrap())
        };
        if is_kind_of {
            Ok(CLKComplicationTemplateGraphicCornerCircularImage(parent.0))
        } else {
            Err ("This CLKComplicationTemplate cannot be downcasted to CLKComplicationTemplateGraphicCornerCircularImage" ,)
        }
    }
}
impl INSObject for CLKComplicationTemplateGraphicCornerCircularImage {}
impl PNSObject for CLKComplicationTemplateGraphicCornerCircularImage {}
impl ICLKComplicationTemplateGraphicCornerCircularImage
    for CLKComplicationTemplateGraphicCornerCircularImage
{
}
pub trait ICLKComplicationTemplateGraphicCornerCircularImage: Sized + std::ops::Deref {
    unsafe fn initWithImageProvider_(
        &self,
        imageProvider: CLKFullColorImageProvider,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithImageProvider : imageProvider)
    }
    unsafe fn imageProvider(&self) -> CLKFullColorImageProvider
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, imageProvider)
    }
    unsafe fn setImageProvider_(&self, imageProvider: CLKFullColorImageProvider)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setImageProvider : imageProvider)
    }
    unsafe fn templateWithImageProvider_(imageProvider: CLKFullColorImageProvider) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CLKComplicationTemplateGraphicCornerCircularImage").unwrap(), templateWithImageProvider : imageProvider)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CLKComplicationTemplateGraphicCircular(pub id);
impl std::ops::Deref for CLKComplicationTemplateGraphicCircular {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CLKComplicationTemplateGraphicCircular {}
impl CLKComplicationTemplateGraphicCircular {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CLKComplicationTemplateGraphicCircular").unwrap(), alloc) })
    }
}
impl ICLKComplicationTemplate for CLKComplicationTemplateGraphicCircular {}
impl PNSCopying for CLKComplicationTemplateGraphicCircular {}
impl std::convert::TryFrom<CLKComplicationTemplate> for CLKComplicationTemplateGraphicCircular {
    type Error = &'static str;
    fn try_from(
        parent: CLKComplicationTemplate,
    ) -> Result<CLKComplicationTemplateGraphicCircular, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CLKComplicationTemplateGraphicCircular").unwrap())
        };
        if is_kind_of {
            Ok(CLKComplicationTemplateGraphicCircular(parent.0))
        } else {
            Err ("This CLKComplicationTemplate cannot be downcasted to CLKComplicationTemplateGraphicCircular" ,)
        }
    }
}
impl INSObject for CLKComplicationTemplateGraphicCircular {}
impl PNSObject for CLKComplicationTemplateGraphicCircular {}
impl ICLKComplicationTemplateGraphicCircular for CLKComplicationTemplateGraphicCircular {}
pub trait ICLKComplicationTemplateGraphicCircular: Sized + std::ops::Deref {}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CLKComplicationTemplateGraphicCircularImage(pub id);
impl std::ops::Deref for CLKComplicationTemplateGraphicCircularImage {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CLKComplicationTemplateGraphicCircularImage {}
impl CLKComplicationTemplateGraphicCircularImage {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CLKComplicationTemplateGraphicCircularImage").unwrap(), alloc) })
    }
}
impl ICLKComplicationTemplateGraphicCircular for CLKComplicationTemplateGraphicCircularImage {}
impl From<CLKComplicationTemplateGraphicCircularImage> for CLKComplicationTemplateGraphicCircular {
    fn from(
        child: CLKComplicationTemplateGraphicCircularImage,
    ) -> CLKComplicationTemplateGraphicCircular {
        CLKComplicationTemplateGraphicCircular(child.0)
    }
}
impl std::convert::TryFrom<CLKComplicationTemplateGraphicCircular>
    for CLKComplicationTemplateGraphicCircularImage
{
    type Error = &'static str;
    fn try_from(
        parent: CLKComplicationTemplateGraphicCircular,
    ) -> Result<CLKComplicationTemplateGraphicCircularImage, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CLKComplicationTemplateGraphicCircularImage").unwrap())
        };
        if is_kind_of {
            Ok(CLKComplicationTemplateGraphicCircularImage(parent.0))
        } else {
            Err ("This CLKComplicationTemplateGraphicCircular cannot be downcasted to CLKComplicationTemplateGraphicCircularImage" ,)
        }
    }
}
impl ICLKComplicationTemplate for CLKComplicationTemplateGraphicCircularImage {}
impl PNSCopying for CLKComplicationTemplateGraphicCircularImage {}
impl INSObject for CLKComplicationTemplateGraphicCircularImage {}
impl PNSObject for CLKComplicationTemplateGraphicCircularImage {}
impl ICLKComplicationTemplateGraphicCircularImage for CLKComplicationTemplateGraphicCircularImage {}
pub trait ICLKComplicationTemplateGraphicCircularImage: Sized + std::ops::Deref {
    unsafe fn initWithImageProvider_(
        &self,
        imageProvider: CLKFullColorImageProvider,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithImageProvider : imageProvider)
    }
    unsafe fn imageProvider(&self) -> CLKFullColorImageProvider
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, imageProvider)
    }
    unsafe fn setImageProvider_(&self, imageProvider: CLKFullColorImageProvider)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setImageProvider : imageProvider)
    }
    unsafe fn templateWithImageProvider_(imageProvider: CLKFullColorImageProvider) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CLKComplicationTemplateGraphicCircularImage").unwrap(), templateWithImageProvider : imageProvider)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CLKComplicationTemplateGraphicCircularOpenGaugeRangeText(pub id);
impl std::ops::Deref for CLKComplicationTemplateGraphicCircularOpenGaugeRangeText {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CLKComplicationTemplateGraphicCircularOpenGaugeRangeText {}
impl CLKComplicationTemplateGraphicCircularOpenGaugeRangeText {
    pub fn alloc() -> Self {
        Self(unsafe {
            msg_send!(&*objc2::runtime::AnyClass::get(c"CLKComplicationTemplateGraphicCircularOpenGaugeRangeText").unwrap(), alloc)
        })
    }
}
impl ICLKComplicationTemplateGraphicCircular
    for CLKComplicationTemplateGraphicCircularOpenGaugeRangeText
{
}
impl std::convert::TryFrom<CLKComplicationTemplateGraphicCircular>
    for CLKComplicationTemplateGraphicCircularOpenGaugeRangeText
{
    type Error = &'static str;
    fn try_from(
        parent: CLKComplicationTemplateGraphicCircular,
    ) -> Result<CLKComplicationTemplateGraphicCircularOpenGaugeRangeText, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CLKComplicationTemplateGraphicCircularOpenGaugeRangeText").unwrap())
        };
        if is_kind_of {
            Ok(CLKComplicationTemplateGraphicCircularOpenGaugeRangeText(
                parent.0,
            ))
        } else {
            Err ("This CLKComplicationTemplateGraphicCircular cannot be downcasted to CLKComplicationTemplateGraphicCircularOpenGaugeRangeText" ,)
        }
    }
}
impl ICLKComplicationTemplate for CLKComplicationTemplateGraphicCircularOpenGaugeRangeText {}
impl PNSCopying for CLKComplicationTemplateGraphicCircularOpenGaugeRangeText {}
impl INSObject for CLKComplicationTemplateGraphicCircularOpenGaugeRangeText {}
impl PNSObject for CLKComplicationTemplateGraphicCircularOpenGaugeRangeText {}
impl ICLKComplicationTemplateGraphicCircularOpenGaugeRangeText
    for CLKComplicationTemplateGraphicCircularOpenGaugeRangeText
{
}
pub trait ICLKComplicationTemplateGraphicCircularOpenGaugeRangeText:
    Sized + std::ops::Deref
{
    unsafe fn initWithGaugeProvider_leadingTextProvider_trailingTextProvider_centerTextProvider_(
        &self,
        gaugeProvider: CLKGaugeProvider,
        leadingTextProvider: CLKTextProvider,
        trailingTextProvider: CLKTextProvider,
        centerTextProvider: CLKTextProvider,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithGaugeProvider : gaugeProvider, leadingTextProvider : leadingTextProvider, trailingTextProvider : trailingTextProvider, centerTextProvider : centerTextProvider)
    }
    unsafe fn gaugeProvider(&self) -> CLKGaugeProvider
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, gaugeProvider)
    }
    unsafe fn setGaugeProvider_(&self, gaugeProvider: CLKGaugeProvider)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setGaugeProvider : gaugeProvider)
    }
    unsafe fn leadingTextProvider(&self) -> CLKTextProvider
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, leadingTextProvider)
    }
    unsafe fn setLeadingTextProvider_(&self, leadingTextProvider: CLKTextProvider)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setLeadingTextProvider : leadingTextProvider)
    }
    unsafe fn trailingTextProvider(&self) -> CLKTextProvider
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, trailingTextProvider)
    }
    unsafe fn setTrailingTextProvider_(&self, trailingTextProvider: CLKTextProvider)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setTrailingTextProvider : trailingTextProvider)
    }
    unsafe fn centerTextProvider(&self) -> CLKTextProvider
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, centerTextProvider)
    }
    unsafe fn setCenterTextProvider_(&self, centerTextProvider: CLKTextProvider)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCenterTextProvider : centerTextProvider)
    }
    unsafe fn templateWithGaugeProvider_leadingTextProvider_trailingTextProvider_centerTextProvider_(
        gaugeProvider: CLKGaugeProvider,
        leadingTextProvider: CLKTextProvider,
        trailingTextProvider: CLKTextProvider,
        centerTextProvider: CLKTextProvider,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CLKComplicationTemplateGraphicCircularOpenGaugeRangeText").unwrap(), templateWithGaugeProvider : gaugeProvider, leadingTextProvider : leadingTextProvider, trailingTextProvider : trailingTextProvider, centerTextProvider : centerTextProvider)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CLKComplicationTemplateGraphicCircularOpenGaugeSimpleText(pub id);
impl std::ops::Deref for CLKComplicationTemplateGraphicCircularOpenGaugeSimpleText {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CLKComplicationTemplateGraphicCircularOpenGaugeSimpleText {}
impl CLKComplicationTemplateGraphicCircularOpenGaugeSimpleText {
    pub fn alloc() -> Self {
        Self(unsafe {
            msg_send!(&*objc2::runtime::AnyClass::get(c"CLKComplicationTemplateGraphicCircularOpenGaugeSimpleText").unwrap(), alloc)
        })
    }
}
impl ICLKComplicationTemplateGraphicCircular
    for CLKComplicationTemplateGraphicCircularOpenGaugeSimpleText
{
}
impl std::convert::TryFrom<CLKComplicationTemplateGraphicCircular>
    for CLKComplicationTemplateGraphicCircularOpenGaugeSimpleText
{
    type Error = &'static str;
    fn try_from(
        parent: CLKComplicationTemplateGraphicCircular,
    ) -> Result<CLKComplicationTemplateGraphicCircularOpenGaugeSimpleText, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CLKComplicationTemplateGraphicCircularOpenGaugeSimpleText").unwrap())
        };
        if is_kind_of {
            Ok(CLKComplicationTemplateGraphicCircularOpenGaugeSimpleText(
                parent.0,
            ))
        } else {
            Err ("This CLKComplicationTemplateGraphicCircular cannot be downcasted to CLKComplicationTemplateGraphicCircularOpenGaugeSimpleText" ,)
        }
    }
}
impl ICLKComplicationTemplate for CLKComplicationTemplateGraphicCircularOpenGaugeSimpleText {}
impl PNSCopying for CLKComplicationTemplateGraphicCircularOpenGaugeSimpleText {}
impl INSObject for CLKComplicationTemplateGraphicCircularOpenGaugeSimpleText {}
impl PNSObject for CLKComplicationTemplateGraphicCircularOpenGaugeSimpleText {}
impl ICLKComplicationTemplateGraphicCircularOpenGaugeSimpleText
    for CLKComplicationTemplateGraphicCircularOpenGaugeSimpleText
{
}
pub trait ICLKComplicationTemplateGraphicCircularOpenGaugeSimpleText:
    Sized + std::ops::Deref
{
    unsafe fn initWithGaugeProvider_bottomTextProvider_centerTextProvider_(
        &self,
        gaugeProvider: CLKGaugeProvider,
        bottomTextProvider: CLKTextProvider,
        centerTextProvider: CLKTextProvider,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithGaugeProvider : gaugeProvider, bottomTextProvider : bottomTextProvider, centerTextProvider : centerTextProvider)
    }
    unsafe fn gaugeProvider(&self) -> CLKGaugeProvider
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, gaugeProvider)
    }
    unsafe fn setGaugeProvider_(&self, gaugeProvider: CLKGaugeProvider)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setGaugeProvider : gaugeProvider)
    }
    unsafe fn bottomTextProvider(&self) -> CLKTextProvider
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, bottomTextProvider)
    }
    unsafe fn setBottomTextProvider_(&self, bottomTextProvider: CLKTextProvider)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setBottomTextProvider : bottomTextProvider)
    }
    unsafe fn centerTextProvider(&self) -> CLKTextProvider
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, centerTextProvider)
    }
    unsafe fn setCenterTextProvider_(&self, centerTextProvider: CLKTextProvider)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCenterTextProvider : centerTextProvider)
    }
    unsafe fn templateWithGaugeProvider_bottomTextProvider_centerTextProvider_(
        gaugeProvider: CLKGaugeProvider,
        bottomTextProvider: CLKTextProvider,
        centerTextProvider: CLKTextProvider,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CLKComplicationTemplateGraphicCircularOpenGaugeSimpleText").unwrap(), templateWithGaugeProvider : gaugeProvider, bottomTextProvider : bottomTextProvider, centerTextProvider : centerTextProvider)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CLKComplicationTemplateGraphicCircularOpenGaugeImage(pub id);
impl std::ops::Deref for CLKComplicationTemplateGraphicCircularOpenGaugeImage {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CLKComplicationTemplateGraphicCircularOpenGaugeImage {}
impl CLKComplicationTemplateGraphicCircularOpenGaugeImage {
    pub fn alloc() -> Self {
        Self(unsafe {
            msg_send!(&*objc2::runtime::AnyClass::get(c"CLKComplicationTemplateGraphicCircularOpenGaugeImage").unwrap(), alloc)
        })
    }
}
impl ICLKComplicationTemplateGraphicCircular
    for CLKComplicationTemplateGraphicCircularOpenGaugeImage
{
}
impl std::convert::TryFrom<CLKComplicationTemplateGraphicCircular>
    for CLKComplicationTemplateGraphicCircularOpenGaugeImage
{
    type Error = &'static str;
    fn try_from(
        parent: CLKComplicationTemplateGraphicCircular,
    ) -> Result<CLKComplicationTemplateGraphicCircularOpenGaugeImage, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CLKComplicationTemplateGraphicCircularOpenGaugeImage").unwrap())
        };
        if is_kind_of {
            Ok(CLKComplicationTemplateGraphicCircularOpenGaugeImage(
                parent.0,
            ))
        } else {
            Err ("This CLKComplicationTemplateGraphicCircular cannot be downcasted to CLKComplicationTemplateGraphicCircularOpenGaugeImage" ,)
        }
    }
}
impl ICLKComplicationTemplate for CLKComplicationTemplateGraphicCircularOpenGaugeImage {}
impl PNSCopying for CLKComplicationTemplateGraphicCircularOpenGaugeImage {}
impl INSObject for CLKComplicationTemplateGraphicCircularOpenGaugeImage {}
impl PNSObject for CLKComplicationTemplateGraphicCircularOpenGaugeImage {}
impl ICLKComplicationTemplateGraphicCircularOpenGaugeImage
    for CLKComplicationTemplateGraphicCircularOpenGaugeImage
{
}
pub trait ICLKComplicationTemplateGraphicCircularOpenGaugeImage: Sized + std::ops::Deref {
    unsafe fn initWithGaugeProvider_bottomImageProvider_centerTextProvider_(
        &self,
        gaugeProvider: CLKGaugeProvider,
        bottomImageProvider: CLKFullColorImageProvider,
        centerTextProvider: CLKTextProvider,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithGaugeProvider : gaugeProvider, bottomImageProvider : bottomImageProvider, centerTextProvider : centerTextProvider)
    }
    unsafe fn gaugeProvider(&self) -> CLKGaugeProvider
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, gaugeProvider)
    }
    unsafe fn setGaugeProvider_(&self, gaugeProvider: CLKGaugeProvider)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setGaugeProvider : gaugeProvider)
    }
    unsafe fn bottomImageProvider(&self) -> CLKFullColorImageProvider
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, bottomImageProvider)
    }
    unsafe fn setBottomImageProvider_(&self, bottomImageProvider: CLKFullColorImageProvider)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setBottomImageProvider : bottomImageProvider)
    }
    unsafe fn centerTextProvider(&self) -> CLKTextProvider
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, centerTextProvider)
    }
    unsafe fn setCenterTextProvider_(&self, centerTextProvider: CLKTextProvider)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCenterTextProvider : centerTextProvider)
    }
    unsafe fn templateWithGaugeProvider_bottomImageProvider_centerTextProvider_(
        gaugeProvider: CLKGaugeProvider,
        bottomImageProvider: CLKFullColorImageProvider,
        centerTextProvider: CLKTextProvider,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CLKComplicationTemplateGraphicCircularOpenGaugeImage").unwrap(), templateWithGaugeProvider : gaugeProvider, bottomImageProvider : bottomImageProvider, centerTextProvider : centerTextProvider)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CLKComplicationTemplateGraphicCircularClosedGaugeText(pub id);
impl std::ops::Deref for CLKComplicationTemplateGraphicCircularClosedGaugeText {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CLKComplicationTemplateGraphicCircularClosedGaugeText {}
impl CLKComplicationTemplateGraphicCircularClosedGaugeText {
    pub fn alloc() -> Self {
        Self(unsafe {
            msg_send!(&*objc2::runtime::AnyClass::get(c"CLKComplicationTemplateGraphicCircularClosedGaugeText").unwrap(), alloc)
        })
    }
}
impl ICLKComplicationTemplateGraphicCircular
    for CLKComplicationTemplateGraphicCircularClosedGaugeText
{
}
impl std::convert::TryFrom<CLKComplicationTemplateGraphicCircular>
    for CLKComplicationTemplateGraphicCircularClosedGaugeText
{
    type Error = &'static str;
    fn try_from(
        parent: CLKComplicationTemplateGraphicCircular,
    ) -> Result<CLKComplicationTemplateGraphicCircularClosedGaugeText, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CLKComplicationTemplateGraphicCircularClosedGaugeText").unwrap())
        };
        if is_kind_of {
            Ok(CLKComplicationTemplateGraphicCircularClosedGaugeText(
                parent.0,
            ))
        } else {
            Err ("This CLKComplicationTemplateGraphicCircular cannot be downcasted to CLKComplicationTemplateGraphicCircularClosedGaugeText" ,)
        }
    }
}
impl ICLKComplicationTemplate for CLKComplicationTemplateGraphicCircularClosedGaugeText {}
impl PNSCopying for CLKComplicationTemplateGraphicCircularClosedGaugeText {}
impl INSObject for CLKComplicationTemplateGraphicCircularClosedGaugeText {}
impl PNSObject for CLKComplicationTemplateGraphicCircularClosedGaugeText {}
impl ICLKComplicationTemplateGraphicCircularClosedGaugeText
    for CLKComplicationTemplateGraphicCircularClosedGaugeText
{
}
pub trait ICLKComplicationTemplateGraphicCircularClosedGaugeText: Sized + std::ops::Deref {
    unsafe fn initWithGaugeProvider_centerTextProvider_(
        &self,
        gaugeProvider: CLKGaugeProvider,
        centerTextProvider: CLKTextProvider,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithGaugeProvider : gaugeProvider, centerTextProvider : centerTextProvider)
    }
    unsafe fn gaugeProvider(&self) -> CLKGaugeProvider
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, gaugeProvider)
    }
    unsafe fn setGaugeProvider_(&self, gaugeProvider: CLKGaugeProvider)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setGaugeProvider : gaugeProvider)
    }
    unsafe fn centerTextProvider(&self) -> CLKTextProvider
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, centerTextProvider)
    }
    unsafe fn setCenterTextProvider_(&self, centerTextProvider: CLKTextProvider)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCenterTextProvider : centerTextProvider)
    }
    unsafe fn templateWithGaugeProvider_centerTextProvider_(
        gaugeProvider: CLKGaugeProvider,
        centerTextProvider: CLKTextProvider,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CLKComplicationTemplateGraphicCircularClosedGaugeText").unwrap(), templateWithGaugeProvider : gaugeProvider, centerTextProvider : centerTextProvider)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CLKComplicationTemplateGraphicCircularClosedGaugeImage(pub id);
impl std::ops::Deref for CLKComplicationTemplateGraphicCircularClosedGaugeImage {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CLKComplicationTemplateGraphicCircularClosedGaugeImage {}
impl CLKComplicationTemplateGraphicCircularClosedGaugeImage {
    pub fn alloc() -> Self {
        Self(unsafe {
            msg_send!(&*objc2::runtime::AnyClass::get(c"CLKComplicationTemplateGraphicCircularClosedGaugeImage").unwrap(), alloc)
        })
    }
}
impl ICLKComplicationTemplateGraphicCircular
    for CLKComplicationTemplateGraphicCircularClosedGaugeImage
{
}
impl std::convert::TryFrom<CLKComplicationTemplateGraphicCircular>
    for CLKComplicationTemplateGraphicCircularClosedGaugeImage
{
    type Error = &'static str;
    fn try_from(
        parent: CLKComplicationTemplateGraphicCircular,
    ) -> Result<CLKComplicationTemplateGraphicCircularClosedGaugeImage, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CLKComplicationTemplateGraphicCircularClosedGaugeImage").unwrap())
        };
        if is_kind_of {
            Ok(CLKComplicationTemplateGraphicCircularClosedGaugeImage(
                parent.0,
            ))
        } else {
            Err ("This CLKComplicationTemplateGraphicCircular cannot be downcasted to CLKComplicationTemplateGraphicCircularClosedGaugeImage" ,)
        }
    }
}
impl ICLKComplicationTemplate for CLKComplicationTemplateGraphicCircularClosedGaugeImage {}
impl PNSCopying for CLKComplicationTemplateGraphicCircularClosedGaugeImage {}
impl INSObject for CLKComplicationTemplateGraphicCircularClosedGaugeImage {}
impl PNSObject for CLKComplicationTemplateGraphicCircularClosedGaugeImage {}
impl ICLKComplicationTemplateGraphicCircularClosedGaugeImage
    for CLKComplicationTemplateGraphicCircularClosedGaugeImage
{
}
pub trait ICLKComplicationTemplateGraphicCircularClosedGaugeImage: Sized + std::ops::Deref {
    unsafe fn initWithGaugeProvider_imageProvider_(
        &self,
        gaugeProvider: CLKGaugeProvider,
        imageProvider: CLKFullColorImageProvider,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithGaugeProvider : gaugeProvider, imageProvider : imageProvider)
    }
    unsafe fn gaugeProvider(&self) -> CLKGaugeProvider
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, gaugeProvider)
    }
    unsafe fn setGaugeProvider_(&self, gaugeProvider: CLKGaugeProvider)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setGaugeProvider : gaugeProvider)
    }
    unsafe fn imageProvider(&self) -> CLKFullColorImageProvider
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, imageProvider)
    }
    unsafe fn setImageProvider_(&self, imageProvider: CLKFullColorImageProvider)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setImageProvider : imageProvider)
    }
    unsafe fn templateWithGaugeProvider_imageProvider_(
        gaugeProvider: CLKGaugeProvider,
        imageProvider: CLKFullColorImageProvider,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CLKComplicationTemplateGraphicCircularClosedGaugeImage").unwrap(), templateWithGaugeProvider : gaugeProvider, imageProvider : imageProvider)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CLKComplicationTemplateGraphicCircularStackText(pub id);
impl std::ops::Deref for CLKComplicationTemplateGraphicCircularStackText {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CLKComplicationTemplateGraphicCircularStackText {}
impl CLKComplicationTemplateGraphicCircularStackText {
    pub fn alloc() -> Self {
        Self(unsafe {
            msg_send!(&*objc2::runtime::AnyClass::get(c"CLKComplicationTemplateGraphicCircularStackText").unwrap(), alloc)
        })
    }
}
impl ICLKComplicationTemplateGraphicCircular for CLKComplicationTemplateGraphicCircularStackText {}
impl std::convert::TryFrom<CLKComplicationTemplateGraphicCircular>
    for CLKComplicationTemplateGraphicCircularStackText
{
    type Error = &'static str;
    fn try_from(
        parent: CLKComplicationTemplateGraphicCircular,
    ) -> Result<CLKComplicationTemplateGraphicCircularStackText, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CLKComplicationTemplateGraphicCircularStackText").unwrap())
        };
        if is_kind_of {
            Ok(CLKComplicationTemplateGraphicCircularStackText(parent.0))
        } else {
            Err ("This CLKComplicationTemplateGraphicCircular cannot be downcasted to CLKComplicationTemplateGraphicCircularStackText" ,)
        }
    }
}
impl ICLKComplicationTemplate for CLKComplicationTemplateGraphicCircularStackText {}
impl PNSCopying for CLKComplicationTemplateGraphicCircularStackText {}
impl INSObject for CLKComplicationTemplateGraphicCircularStackText {}
impl PNSObject for CLKComplicationTemplateGraphicCircularStackText {}
impl ICLKComplicationTemplateGraphicCircularStackText
    for CLKComplicationTemplateGraphicCircularStackText
{
}
pub trait ICLKComplicationTemplateGraphicCircularStackText: Sized + std::ops::Deref {
    unsafe fn initWithLine1TextProvider_line2TextProvider_(
        &self,
        line1TextProvider: CLKTextProvider,
        line2TextProvider: CLKTextProvider,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithLine1TextProvider : line1TextProvider, line2TextProvider : line2TextProvider)
    }
    unsafe fn line1TextProvider(&self) -> CLKTextProvider
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, line1TextProvider)
    }
    unsafe fn setLine1TextProvider_(&self, line1TextProvider: CLKTextProvider)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setLine1TextProvider : line1TextProvider)
    }
    unsafe fn line2TextProvider(&self) -> CLKTextProvider
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, line2TextProvider)
    }
    unsafe fn setLine2TextProvider_(&self, line2TextProvider: CLKTextProvider)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setLine2TextProvider : line2TextProvider)
    }
    unsafe fn templateWithLine1TextProvider_line2TextProvider_(
        line1TextProvider: CLKTextProvider,
        line2TextProvider: CLKTextProvider,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CLKComplicationTemplateGraphicCircularStackText").unwrap(), templateWithLine1TextProvider : line1TextProvider, line2TextProvider : line2TextProvider)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CLKComplicationTemplateGraphicCircularStackImage(pub id);
impl std::ops::Deref for CLKComplicationTemplateGraphicCircularStackImage {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CLKComplicationTemplateGraphicCircularStackImage {}
impl CLKComplicationTemplateGraphicCircularStackImage {
    pub fn alloc() -> Self {
        Self(unsafe {
            msg_send!(&*objc2::runtime::AnyClass::get(c"CLKComplicationTemplateGraphicCircularStackImage").unwrap(), alloc)
        })
    }
}
impl ICLKComplicationTemplateGraphicCircular for CLKComplicationTemplateGraphicCircularStackImage {}
impl std::convert::TryFrom<CLKComplicationTemplateGraphicCircular>
    for CLKComplicationTemplateGraphicCircularStackImage
{
    type Error = &'static str;
    fn try_from(
        parent: CLKComplicationTemplateGraphicCircular,
    ) -> Result<CLKComplicationTemplateGraphicCircularStackImage, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CLKComplicationTemplateGraphicCircularStackImage").unwrap())
        };
        if is_kind_of {
            Ok(CLKComplicationTemplateGraphicCircularStackImage(parent.0))
        } else {
            Err ("This CLKComplicationTemplateGraphicCircular cannot be downcasted to CLKComplicationTemplateGraphicCircularStackImage" ,)
        }
    }
}
impl ICLKComplicationTemplate for CLKComplicationTemplateGraphicCircularStackImage {}
impl PNSCopying for CLKComplicationTemplateGraphicCircularStackImage {}
impl INSObject for CLKComplicationTemplateGraphicCircularStackImage {}
impl PNSObject for CLKComplicationTemplateGraphicCircularStackImage {}
impl ICLKComplicationTemplateGraphicCircularStackImage
    for CLKComplicationTemplateGraphicCircularStackImage
{
}
pub trait ICLKComplicationTemplateGraphicCircularStackImage: Sized + std::ops::Deref {
    unsafe fn initWithLine1ImageProvider_line2TextProvider_(
        &self,
        line1ImageProvider: CLKFullColorImageProvider,
        line2TextProvider: CLKTextProvider,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithLine1ImageProvider : line1ImageProvider, line2TextProvider : line2TextProvider)
    }
    unsafe fn line1ImageProvider(&self) -> CLKFullColorImageProvider
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, line1ImageProvider)
    }
    unsafe fn setLine1ImageProvider_(&self, line1ImageProvider: CLKFullColorImageProvider)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setLine1ImageProvider : line1ImageProvider)
    }
    unsafe fn line2TextProvider(&self) -> CLKTextProvider
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, line2TextProvider)
    }
    unsafe fn setLine2TextProvider_(&self, line2TextProvider: CLKTextProvider)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setLine2TextProvider : line2TextProvider)
    }
    unsafe fn templateWithLine1ImageProvider_line2TextProvider_(
        line1ImageProvider: CLKFullColorImageProvider,
        line2TextProvider: CLKTextProvider,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CLKComplicationTemplateGraphicCircularStackImage").unwrap(), templateWithLine1ImageProvider : line1ImageProvider, line2TextProvider : line2TextProvider)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CLKComplicationTemplateGraphicBezelCircularText(pub id);
impl std::ops::Deref for CLKComplicationTemplateGraphicBezelCircularText {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CLKComplicationTemplateGraphicBezelCircularText {}
impl CLKComplicationTemplateGraphicBezelCircularText {
    pub fn alloc() -> Self {
        Self(unsafe {
            msg_send!(&*objc2::runtime::AnyClass::get(c"CLKComplicationTemplateGraphicBezelCircularText").unwrap(), alloc)
        })
    }
}
impl ICLKComplicationTemplate for CLKComplicationTemplateGraphicBezelCircularText {}
impl PNSCopying for CLKComplicationTemplateGraphicBezelCircularText {}
impl std::convert::TryFrom<CLKComplicationTemplate>
    for CLKComplicationTemplateGraphicBezelCircularText
{
    type Error = &'static str;
    fn try_from(
        parent: CLKComplicationTemplate,
    ) -> Result<CLKComplicationTemplateGraphicBezelCircularText, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CLKComplicationTemplateGraphicBezelCircularText").unwrap())
        };
        if is_kind_of {
            Ok(CLKComplicationTemplateGraphicBezelCircularText(parent.0))
        } else {
            Err ("This CLKComplicationTemplate cannot be downcasted to CLKComplicationTemplateGraphicBezelCircularText" ,)
        }
    }
}
impl INSObject for CLKComplicationTemplateGraphicBezelCircularText {}
impl PNSObject for CLKComplicationTemplateGraphicBezelCircularText {}
impl ICLKComplicationTemplateGraphicBezelCircularText
    for CLKComplicationTemplateGraphicBezelCircularText
{
}
pub trait ICLKComplicationTemplateGraphicBezelCircularText: Sized + std::ops::Deref {
    unsafe fn initWithCircularTemplate_(
        &self,
        circularTemplate: CLKComplicationTemplateGraphicCircular,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithCircularTemplate : circularTemplate)
    }
    unsafe fn initWithCircularTemplate_textProvider_(
        &self,
        circularTemplate: CLKComplicationTemplateGraphicCircular,
        textProvider: CLKTextProvider,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithCircularTemplate : circularTemplate, textProvider : textProvider)
    }
    unsafe fn circularTemplate(&self) -> CLKComplicationTemplateGraphicCircular
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, circularTemplate)
    }
    unsafe fn setCircularTemplate_(&self, circularTemplate: CLKComplicationTemplateGraphicCircular)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCircularTemplate : circularTemplate)
    }
    unsafe fn textProvider(&self) -> CLKTextProvider
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, textProvider)
    }
    unsafe fn setTextProvider_(&self, textProvider: CLKTextProvider)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setTextProvider : textProvider)
    }
    unsafe fn templateWithCircularTemplate_(
        circularTemplate: CLKComplicationTemplateGraphicCircular,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CLKComplicationTemplateGraphicBezelCircularText").unwrap(), templateWithCircularTemplate : circularTemplate)
    }
    unsafe fn templateWithCircularTemplate_textProvider_(
        circularTemplate: CLKComplicationTemplateGraphicCircular,
        textProvider: CLKTextProvider,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CLKComplicationTemplateGraphicBezelCircularText").unwrap(), templateWithCircularTemplate : circularTemplate, textProvider : textProvider)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CLKComplicationTemplateGraphicRectangularFullImage(pub id);
impl std::ops::Deref for CLKComplicationTemplateGraphicRectangularFullImage {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CLKComplicationTemplateGraphicRectangularFullImage {}
impl CLKComplicationTemplateGraphicRectangularFullImage {
    pub fn alloc() -> Self {
        Self(unsafe {
            msg_send!(&*objc2::runtime::AnyClass::get(c"CLKComplicationTemplateGraphicRectangularFullImage").unwrap(), alloc)
        })
    }
}
impl ICLKComplicationTemplate for CLKComplicationTemplateGraphicRectangularFullImage {}
impl PNSCopying for CLKComplicationTemplateGraphicRectangularFullImage {}
impl std::convert::TryFrom<CLKComplicationTemplate>
    for CLKComplicationTemplateGraphicRectangularFullImage
{
    type Error = &'static str;
    fn try_from(
        parent: CLKComplicationTemplate,
    ) -> Result<CLKComplicationTemplateGraphicRectangularFullImage, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CLKComplicationTemplateGraphicRectangularFullImage").unwrap())
        };
        if is_kind_of {
            Ok(CLKComplicationTemplateGraphicRectangularFullImage(parent.0))
        } else {
            Err ("This CLKComplicationTemplate cannot be downcasted to CLKComplicationTemplateGraphicRectangularFullImage" ,)
        }
    }
}
impl INSObject for CLKComplicationTemplateGraphicRectangularFullImage {}
impl PNSObject for CLKComplicationTemplateGraphicRectangularFullImage {}
impl ICLKComplicationTemplateGraphicRectangularFullImage
    for CLKComplicationTemplateGraphicRectangularFullImage
{
}
pub trait ICLKComplicationTemplateGraphicRectangularFullImage: Sized + std::ops::Deref {
    unsafe fn initWithImageProvider_(
        &self,
        imageProvider: CLKFullColorImageProvider,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithImageProvider : imageProvider)
    }
    unsafe fn imageProvider(&self) -> CLKFullColorImageProvider
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, imageProvider)
    }
    unsafe fn setImageProvider_(&self, imageProvider: CLKFullColorImageProvider)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setImageProvider : imageProvider)
    }
    unsafe fn templateWithImageProvider_(imageProvider: CLKFullColorImageProvider) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CLKComplicationTemplateGraphicRectangularFullImage").unwrap(), templateWithImageProvider : imageProvider)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CLKComplicationTemplateGraphicRectangularLargeImage(pub id);
impl std::ops::Deref for CLKComplicationTemplateGraphicRectangularLargeImage {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CLKComplicationTemplateGraphicRectangularLargeImage {}
impl CLKComplicationTemplateGraphicRectangularLargeImage {
    pub fn alloc() -> Self {
        Self(unsafe {
            msg_send!(&*objc2::runtime::AnyClass::get(c"CLKComplicationTemplateGraphicRectangularLargeImage").unwrap(), alloc)
        })
    }
}
impl ICLKComplicationTemplate for CLKComplicationTemplateGraphicRectangularLargeImage {}
impl PNSCopying for CLKComplicationTemplateGraphicRectangularLargeImage {}
impl std::convert::TryFrom<CLKComplicationTemplate>
    for CLKComplicationTemplateGraphicRectangularLargeImage
{
    type Error = &'static str;
    fn try_from(
        parent: CLKComplicationTemplate,
    ) -> Result<CLKComplicationTemplateGraphicRectangularLargeImage, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CLKComplicationTemplateGraphicRectangularLargeImage").unwrap())
        };
        if is_kind_of {
            Ok(CLKComplicationTemplateGraphicRectangularLargeImage(
                parent.0,
            ))
        } else {
            Err ("This CLKComplicationTemplate cannot be downcasted to CLKComplicationTemplateGraphicRectangularLargeImage" ,)
        }
    }
}
impl INSObject for CLKComplicationTemplateGraphicRectangularLargeImage {}
impl PNSObject for CLKComplicationTemplateGraphicRectangularLargeImage {}
impl ICLKComplicationTemplateGraphicRectangularLargeImage
    for CLKComplicationTemplateGraphicRectangularLargeImage
{
}
pub trait ICLKComplicationTemplateGraphicRectangularLargeImage: Sized + std::ops::Deref {
    unsafe fn initWithTextProvider_imageProvider_(
        &self,
        textProvider: CLKTextProvider,
        imageProvider: CLKFullColorImageProvider,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithTextProvider : textProvider, imageProvider : imageProvider)
    }
    unsafe fn textProvider(&self) -> CLKTextProvider
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, textProvider)
    }
    unsafe fn setTextProvider_(&self, textProvider: CLKTextProvider)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setTextProvider : textProvider)
    }
    unsafe fn imageProvider(&self) -> CLKFullColorImageProvider
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, imageProvider)
    }
    unsafe fn setImageProvider_(&self, imageProvider: CLKFullColorImageProvider)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setImageProvider : imageProvider)
    }
    unsafe fn templateWithTextProvider_imageProvider_(
        textProvider: CLKTextProvider,
        imageProvider: CLKFullColorImageProvider,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CLKComplicationTemplateGraphicRectangularLargeImage").unwrap(), templateWithTextProvider : textProvider, imageProvider : imageProvider)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CLKComplicationTemplateGraphicRectangularStandardBody(pub id);
impl std::ops::Deref for CLKComplicationTemplateGraphicRectangularStandardBody {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CLKComplicationTemplateGraphicRectangularStandardBody {}
impl CLKComplicationTemplateGraphicRectangularStandardBody {
    pub fn alloc() -> Self {
        Self(unsafe {
            msg_send!(&*objc2::runtime::AnyClass::get(c"CLKComplicationTemplateGraphicRectangularStandardBody").unwrap(), alloc)
        })
    }
}
impl ICLKComplicationTemplate for CLKComplicationTemplateGraphicRectangularStandardBody {}
impl PNSCopying for CLKComplicationTemplateGraphicRectangularStandardBody {}
impl std::convert::TryFrom<CLKComplicationTemplate>
    for CLKComplicationTemplateGraphicRectangularStandardBody
{
    type Error = &'static str;
    fn try_from(
        parent: CLKComplicationTemplate,
    ) -> Result<CLKComplicationTemplateGraphicRectangularStandardBody, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CLKComplicationTemplateGraphicRectangularStandardBody").unwrap())
        };
        if is_kind_of {
            Ok(CLKComplicationTemplateGraphicRectangularStandardBody(
                parent.0,
            ))
        } else {
            Err ("This CLKComplicationTemplate cannot be downcasted to CLKComplicationTemplateGraphicRectangularStandardBody" ,)
        }
    }
}
impl INSObject for CLKComplicationTemplateGraphicRectangularStandardBody {}
impl PNSObject for CLKComplicationTemplateGraphicRectangularStandardBody {}
impl ICLKComplicationTemplateGraphicRectangularStandardBody
    for CLKComplicationTemplateGraphicRectangularStandardBody
{
}
pub trait ICLKComplicationTemplateGraphicRectangularStandardBody: Sized + std::ops::Deref {
    unsafe fn initWithHeaderTextProvider_body1TextProvider_(
        &self,
        headerTextProvider: CLKTextProvider,
        body1TextProvider: CLKTextProvider,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithHeaderTextProvider : headerTextProvider, body1TextProvider : body1TextProvider)
    }
    unsafe fn initWithHeaderTextProvider_body1TextProvider_body2TextProvider_(
        &self,
        headerTextProvider: CLKTextProvider,
        body1TextProvider: CLKTextProvider,
        body2TextProvider: CLKTextProvider,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithHeaderTextProvider : headerTextProvider, body1TextProvider : body1TextProvider, body2TextProvider : body2TextProvider)
    }
    unsafe fn initWithHeaderImageProvider_headerTextProvider_body1TextProvider_(
        &self,
        headerImageProvider: CLKFullColorImageProvider,
        headerTextProvider: CLKTextProvider,
        body1TextProvider: CLKTextProvider,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithHeaderImageProvider : headerImageProvider, headerTextProvider : headerTextProvider, body1TextProvider : body1TextProvider)
    }
    unsafe fn initWithHeaderImageProvider_headerTextProvider_body1TextProvider_body2TextProvider_(
        &self,
        headerImageProvider: CLKFullColorImageProvider,
        headerTextProvider: CLKTextProvider,
        body1TextProvider: CLKTextProvider,
        body2TextProvider: CLKTextProvider,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithHeaderImageProvider : headerImageProvider, headerTextProvider : headerTextProvider, body1TextProvider : body1TextProvider, body2TextProvider : body2TextProvider)
    }
    unsafe fn headerImageProvider(&self) -> CLKFullColorImageProvider
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, headerImageProvider)
    }
    unsafe fn setHeaderImageProvider_(&self, headerImageProvider: CLKFullColorImageProvider)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setHeaderImageProvider : headerImageProvider)
    }
    unsafe fn headerTextProvider(&self) -> CLKTextProvider
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, headerTextProvider)
    }
    unsafe fn setHeaderTextProvider_(&self, headerTextProvider: CLKTextProvider)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setHeaderTextProvider : headerTextProvider)
    }
    unsafe fn body1TextProvider(&self) -> CLKTextProvider
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, body1TextProvider)
    }
    unsafe fn setBody1TextProvider_(&self, body1TextProvider: CLKTextProvider)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setBody1TextProvider : body1TextProvider)
    }
    unsafe fn body2TextProvider(&self) -> CLKTextProvider
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, body2TextProvider)
    }
    unsafe fn setBody2TextProvider_(&self, body2TextProvider: CLKTextProvider)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setBody2TextProvider : body2TextProvider)
    }
    unsafe fn templateWithHeaderTextProvider_body1TextProvider_(
        headerTextProvider: CLKTextProvider,
        body1TextProvider: CLKTextProvider,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CLKComplicationTemplateGraphicRectangularStandardBody").unwrap(), templateWithHeaderTextProvider : headerTextProvider, body1TextProvider : body1TextProvider)
    }
    unsafe fn templateWithHeaderTextProvider_body1TextProvider_body2TextProvider_(
        headerTextProvider: CLKTextProvider,
        body1TextProvider: CLKTextProvider,
        body2TextProvider: CLKTextProvider,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CLKComplicationTemplateGraphicRectangularStandardBody").unwrap(), templateWithHeaderTextProvider : headerTextProvider, body1TextProvider : body1TextProvider, body2TextProvider : body2TextProvider)
    }
    unsafe fn templateWithHeaderImageProvider_headerTextProvider_body1TextProvider_(
        headerImageProvider: CLKFullColorImageProvider,
        headerTextProvider: CLKTextProvider,
        body1TextProvider: CLKTextProvider,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CLKComplicationTemplateGraphicRectangularStandardBody").unwrap(), templateWithHeaderImageProvider : headerImageProvider, headerTextProvider : headerTextProvider, body1TextProvider : body1TextProvider)
    }
    unsafe fn templateWithHeaderImageProvider_headerTextProvider_body1TextProvider_body2TextProvider_(
        headerImageProvider: CLKFullColorImageProvider,
        headerTextProvider: CLKTextProvider,
        body1TextProvider: CLKTextProvider,
        body2TextProvider: CLKTextProvider,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CLKComplicationTemplateGraphicRectangularStandardBody").unwrap(), templateWithHeaderImageProvider : headerImageProvider, headerTextProvider : headerTextProvider, body1TextProvider : body1TextProvider, body2TextProvider : body2TextProvider)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CLKComplicationTemplateGraphicRectangularTextGauge(pub id);
impl std::ops::Deref for CLKComplicationTemplateGraphicRectangularTextGauge {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CLKComplicationTemplateGraphicRectangularTextGauge {}
impl CLKComplicationTemplateGraphicRectangularTextGauge {
    pub fn alloc() -> Self {
        Self(unsafe {
            msg_send!(&*objc2::runtime::AnyClass::get(c"CLKComplicationTemplateGraphicRectangularTextGauge").unwrap(), alloc)
        })
    }
}
impl ICLKComplicationTemplate for CLKComplicationTemplateGraphicRectangularTextGauge {}
impl PNSCopying for CLKComplicationTemplateGraphicRectangularTextGauge {}
impl std::convert::TryFrom<CLKComplicationTemplate>
    for CLKComplicationTemplateGraphicRectangularTextGauge
{
    type Error = &'static str;
    fn try_from(
        parent: CLKComplicationTemplate,
    ) -> Result<CLKComplicationTemplateGraphicRectangularTextGauge, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CLKComplicationTemplateGraphicRectangularTextGauge").unwrap())
        };
        if is_kind_of {
            Ok(CLKComplicationTemplateGraphicRectangularTextGauge(parent.0))
        } else {
            Err ("This CLKComplicationTemplate cannot be downcasted to CLKComplicationTemplateGraphicRectangularTextGauge" ,)
        }
    }
}
impl INSObject for CLKComplicationTemplateGraphicRectangularTextGauge {}
impl PNSObject for CLKComplicationTemplateGraphicRectangularTextGauge {}
impl ICLKComplicationTemplateGraphicRectangularTextGauge
    for CLKComplicationTemplateGraphicRectangularTextGauge
{
}
pub trait ICLKComplicationTemplateGraphicRectangularTextGauge: Sized + std::ops::Deref {
    unsafe fn initWithHeaderTextProvider_body1TextProvider_gaugeProvider_(
        &self,
        headerTextProvider: CLKTextProvider,
        body1TextProvider: CLKTextProvider,
        gaugeProvider: CLKGaugeProvider,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithHeaderTextProvider : headerTextProvider, body1TextProvider : body1TextProvider, gaugeProvider : gaugeProvider)
    }
    unsafe fn initWithHeaderImageProvider_headerTextProvider_body1TextProvider_gaugeProvider_(
        &self,
        headerImageProvider: CLKFullColorImageProvider,
        headerTextProvider: CLKTextProvider,
        body1TextProvider: CLKTextProvider,
        gaugeProvider: CLKGaugeProvider,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithHeaderImageProvider : headerImageProvider, headerTextProvider : headerTextProvider, body1TextProvider : body1TextProvider, gaugeProvider : gaugeProvider)
    }
    unsafe fn headerImageProvider(&self) -> CLKFullColorImageProvider
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, headerImageProvider)
    }
    unsafe fn setHeaderImageProvider_(&self, headerImageProvider: CLKFullColorImageProvider)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setHeaderImageProvider : headerImageProvider)
    }
    unsafe fn headerTextProvider(&self) -> CLKTextProvider
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, headerTextProvider)
    }
    unsafe fn setHeaderTextProvider_(&self, headerTextProvider: CLKTextProvider)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setHeaderTextProvider : headerTextProvider)
    }
    unsafe fn body1TextProvider(&self) -> CLKTextProvider
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, body1TextProvider)
    }
    unsafe fn setBody1TextProvider_(&self, body1TextProvider: CLKTextProvider)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setBody1TextProvider : body1TextProvider)
    }
    unsafe fn gaugeProvider(&self) -> CLKGaugeProvider
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, gaugeProvider)
    }
    unsafe fn setGaugeProvider_(&self, gaugeProvider: CLKGaugeProvider)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setGaugeProvider : gaugeProvider)
    }
    unsafe fn templateWithHeaderTextProvider_body1TextProvider_gaugeProvider_(
        headerTextProvider: CLKTextProvider,
        body1TextProvider: CLKTextProvider,
        gaugeProvider: CLKGaugeProvider,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CLKComplicationTemplateGraphicRectangularTextGauge").unwrap(), templateWithHeaderTextProvider : headerTextProvider, body1TextProvider : body1TextProvider, gaugeProvider : gaugeProvider)
    }
    unsafe fn templateWithHeaderImageProvider_headerTextProvider_body1TextProvider_gaugeProvider_(
        headerImageProvider: CLKFullColorImageProvider,
        headerTextProvider: CLKTextProvider,
        body1TextProvider: CLKTextProvider,
        gaugeProvider: CLKGaugeProvider,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CLKComplicationTemplateGraphicRectangularTextGauge").unwrap(), templateWithHeaderImageProvider : headerImageProvider, headerTextProvider : headerTextProvider, body1TextProvider : body1TextProvider, gaugeProvider : gaugeProvider)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CLKComplicationTemplateGraphicExtraLargeCircular(pub id);
impl std::ops::Deref for CLKComplicationTemplateGraphicExtraLargeCircular {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CLKComplicationTemplateGraphicExtraLargeCircular {}
impl CLKComplicationTemplateGraphicExtraLargeCircular {
    pub fn alloc() -> Self {
        Self(unsafe {
            msg_send!(&*objc2::runtime::AnyClass::get(c"CLKComplicationTemplateGraphicExtraLargeCircular").unwrap(), alloc)
        })
    }
}
impl ICLKComplicationTemplate for CLKComplicationTemplateGraphicExtraLargeCircular {}
impl PNSCopying for CLKComplicationTemplateGraphicExtraLargeCircular {}
impl std::convert::TryFrom<CLKComplicationTemplate>
    for CLKComplicationTemplateGraphicExtraLargeCircular
{
    type Error = &'static str;
    fn try_from(
        parent: CLKComplicationTemplate,
    ) -> Result<CLKComplicationTemplateGraphicExtraLargeCircular, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CLKComplicationTemplateGraphicExtraLargeCircular").unwrap())
        };
        if is_kind_of {
            Ok(CLKComplicationTemplateGraphicExtraLargeCircular(parent.0))
        } else {
            Err ("This CLKComplicationTemplate cannot be downcasted to CLKComplicationTemplateGraphicExtraLargeCircular" ,)
        }
    }
}
impl INSObject for CLKComplicationTemplateGraphicExtraLargeCircular {}
impl PNSObject for CLKComplicationTemplateGraphicExtraLargeCircular {}
impl ICLKComplicationTemplateGraphicExtraLargeCircular
    for CLKComplicationTemplateGraphicExtraLargeCircular
{
}
pub trait ICLKComplicationTemplateGraphicExtraLargeCircular: Sized + std::ops::Deref {}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CLKComplicationTemplateGraphicExtraLargeCircularImage(pub id);
impl std::ops::Deref for CLKComplicationTemplateGraphicExtraLargeCircularImage {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CLKComplicationTemplateGraphicExtraLargeCircularImage {}
impl CLKComplicationTemplateGraphicExtraLargeCircularImage {
    pub fn alloc() -> Self {
        Self(unsafe {
            msg_send!(&*objc2::runtime::AnyClass::get(c"CLKComplicationTemplateGraphicExtraLargeCircularImage").unwrap(), alloc)
        })
    }
}
impl ICLKComplicationTemplateGraphicExtraLargeCircular
    for CLKComplicationTemplateGraphicExtraLargeCircularImage
{
}
impl From<CLKComplicationTemplateGraphicExtraLargeCircularImage>
    for CLKComplicationTemplateGraphicExtraLargeCircular
{
    fn from(
        child: CLKComplicationTemplateGraphicExtraLargeCircularImage,
    ) -> CLKComplicationTemplateGraphicExtraLargeCircular {
        CLKComplicationTemplateGraphicExtraLargeCircular(child.0)
    }
}
impl std::convert::TryFrom<CLKComplicationTemplateGraphicExtraLargeCircular>
    for CLKComplicationTemplateGraphicExtraLargeCircularImage
{
    type Error = &'static str;
    fn try_from(
        parent: CLKComplicationTemplateGraphicExtraLargeCircular,
    ) -> Result<CLKComplicationTemplateGraphicExtraLargeCircularImage, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CLKComplicationTemplateGraphicExtraLargeCircularImage").unwrap())
        };
        if is_kind_of {
            Ok(CLKComplicationTemplateGraphicExtraLargeCircularImage(
                parent.0,
            ))
        } else {
            Err ("This CLKComplicationTemplateGraphicExtraLargeCircular cannot be downcasted to CLKComplicationTemplateGraphicExtraLargeCircularImage" ,)
        }
    }
}
impl ICLKComplicationTemplate for CLKComplicationTemplateGraphicExtraLargeCircularImage {}
impl PNSCopying for CLKComplicationTemplateGraphicExtraLargeCircularImage {}
impl INSObject for CLKComplicationTemplateGraphicExtraLargeCircularImage {}
impl PNSObject for CLKComplicationTemplateGraphicExtraLargeCircularImage {}
impl ICLKComplicationTemplateGraphicExtraLargeCircularImage
    for CLKComplicationTemplateGraphicExtraLargeCircularImage
{
}
pub trait ICLKComplicationTemplateGraphicExtraLargeCircularImage: Sized + std::ops::Deref {
    unsafe fn initWithImageProvider_(
        &self,
        imageProvider: CLKFullColorImageProvider,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithImageProvider : imageProvider)
    }
    unsafe fn imageProvider(&self) -> CLKFullColorImageProvider
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, imageProvider)
    }
    unsafe fn setImageProvider_(&self, imageProvider: CLKFullColorImageProvider)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setImageProvider : imageProvider)
    }
    unsafe fn templateWithImageProvider_(imageProvider: CLKFullColorImageProvider) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CLKComplicationTemplateGraphicExtraLargeCircularImage").unwrap(), templateWithImageProvider : imageProvider)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CLKComplicationTemplateGraphicExtraLargeCircularOpenGaugeRangeText(pub id);
impl std::ops::Deref for CLKComplicationTemplateGraphicExtraLargeCircularOpenGaugeRangeText {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CLKComplicationTemplateGraphicExtraLargeCircularOpenGaugeRangeText {}
impl CLKComplicationTemplateGraphicExtraLargeCircularOpenGaugeRangeText {
    pub fn alloc() -> Self {
        Self(unsafe {
            msg_send!(&*objc2::runtime::AnyClass::get(c"CLKComplicationTemplateGraphicExtraLargeCircularOpenGaugeRangeText").unwrap(), alloc)
        })
    }
}
impl ICLKComplicationTemplateGraphicExtraLargeCircular
    for CLKComplicationTemplateGraphicExtraLargeCircularOpenGaugeRangeText
{
}
impl std::convert::TryFrom<CLKComplicationTemplateGraphicExtraLargeCircular>
    for CLKComplicationTemplateGraphicExtraLargeCircularOpenGaugeRangeText
{
    type Error = &'static str;
    fn try_from(
        parent: CLKComplicationTemplateGraphicExtraLargeCircular,
    ) -> Result<CLKComplicationTemplateGraphicExtraLargeCircularOpenGaugeRangeText, Self::Error>
    {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CLKComplicationTemplateGraphicExtraLargeCircularOpenGaugeRangeText").unwrap())
        };
        if is_kind_of {
            Ok(CLKComplicationTemplateGraphicExtraLargeCircularOpenGaugeRangeText(parent.0))
        } else {
            Err ("This CLKComplicationTemplateGraphicExtraLargeCircular cannot be downcasted to CLKComplicationTemplateGraphicExtraLargeCircularOpenGaugeRangeText" ,)
        }
    }
}
impl ICLKComplicationTemplate
    for CLKComplicationTemplateGraphicExtraLargeCircularOpenGaugeRangeText
{
}
impl PNSCopying for CLKComplicationTemplateGraphicExtraLargeCircularOpenGaugeRangeText {}
impl INSObject for CLKComplicationTemplateGraphicExtraLargeCircularOpenGaugeRangeText {}
impl PNSObject for CLKComplicationTemplateGraphicExtraLargeCircularOpenGaugeRangeText {}
impl ICLKComplicationTemplateGraphicExtraLargeCircularOpenGaugeRangeText
    for CLKComplicationTemplateGraphicExtraLargeCircularOpenGaugeRangeText
{
}
pub trait ICLKComplicationTemplateGraphicExtraLargeCircularOpenGaugeRangeText:
    Sized + std::ops::Deref
{
    unsafe fn initWithGaugeProvider_leadingTextProvider_trailingTextProvider_centerTextProvider_(
        &self,
        gaugeProvider: CLKGaugeProvider,
        leadingTextProvider: CLKTextProvider,
        trailingTextProvider: CLKTextProvider,
        centerTextProvider: CLKTextProvider,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithGaugeProvider : gaugeProvider, leadingTextProvider : leadingTextProvider, trailingTextProvider : trailingTextProvider, centerTextProvider : centerTextProvider)
    }
    unsafe fn gaugeProvider(&self) -> CLKGaugeProvider
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, gaugeProvider)
    }
    unsafe fn setGaugeProvider_(&self, gaugeProvider: CLKGaugeProvider)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setGaugeProvider : gaugeProvider)
    }
    unsafe fn leadingTextProvider(&self) -> CLKTextProvider
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, leadingTextProvider)
    }
    unsafe fn setLeadingTextProvider_(&self, leadingTextProvider: CLKTextProvider)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setLeadingTextProvider : leadingTextProvider)
    }
    unsafe fn trailingTextProvider(&self) -> CLKTextProvider
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, trailingTextProvider)
    }
    unsafe fn setTrailingTextProvider_(&self, trailingTextProvider: CLKTextProvider)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setTrailingTextProvider : trailingTextProvider)
    }
    unsafe fn centerTextProvider(&self) -> CLKTextProvider
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, centerTextProvider)
    }
    unsafe fn setCenterTextProvider_(&self, centerTextProvider: CLKTextProvider)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCenterTextProvider : centerTextProvider)
    }
    unsafe fn templateWithGaugeProvider_leadingTextProvider_trailingTextProvider_centerTextProvider_(
        gaugeProvider: CLKGaugeProvider,
        leadingTextProvider: CLKTextProvider,
        trailingTextProvider: CLKTextProvider,
        centerTextProvider: CLKTextProvider,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CLKComplicationTemplateGraphicExtraLargeCircularOpenGaugeRangeText").unwrap(), templateWithGaugeProvider : gaugeProvider, leadingTextProvider : leadingTextProvider, trailingTextProvider : trailingTextProvider, centerTextProvider : centerTextProvider)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CLKComplicationTemplateGraphicExtraLargeCircularOpenGaugeSimpleText(pub id);
impl std::ops::Deref for CLKComplicationTemplateGraphicExtraLargeCircularOpenGaugeSimpleText {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CLKComplicationTemplateGraphicExtraLargeCircularOpenGaugeSimpleText {}
impl CLKComplicationTemplateGraphicExtraLargeCircularOpenGaugeSimpleText {
    pub fn alloc() -> Self {
        Self(unsafe {
            msg_send!(&*objc2::runtime::AnyClass::get(c"CLKComplicationTemplateGraphicExtraLargeCircularOpenGaugeSimpleText").unwrap(), alloc)
        })
    }
}
impl ICLKComplicationTemplateGraphicExtraLargeCircular
    for CLKComplicationTemplateGraphicExtraLargeCircularOpenGaugeSimpleText
{
}
impl std::convert::TryFrom<CLKComplicationTemplateGraphicExtraLargeCircular>
    for CLKComplicationTemplateGraphicExtraLargeCircularOpenGaugeSimpleText
{
    type Error = &'static str;
    fn try_from(
        parent: CLKComplicationTemplateGraphicExtraLargeCircular,
    ) -> Result<CLKComplicationTemplateGraphicExtraLargeCircularOpenGaugeSimpleText, Self::Error>
    {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CLKComplicationTemplateGraphicExtraLargeCircularOpenGaugeSimpleText").unwrap())
        };
        if is_kind_of {
            Ok(CLKComplicationTemplateGraphicExtraLargeCircularOpenGaugeSimpleText(parent.0))
        } else {
            Err ("This CLKComplicationTemplateGraphicExtraLargeCircular cannot be downcasted to CLKComplicationTemplateGraphicExtraLargeCircularOpenGaugeSimpleText" ,)
        }
    }
}
impl ICLKComplicationTemplate
    for CLKComplicationTemplateGraphicExtraLargeCircularOpenGaugeSimpleText
{
}
impl PNSCopying for CLKComplicationTemplateGraphicExtraLargeCircularOpenGaugeSimpleText {}
impl INSObject for CLKComplicationTemplateGraphicExtraLargeCircularOpenGaugeSimpleText {}
impl PNSObject for CLKComplicationTemplateGraphicExtraLargeCircularOpenGaugeSimpleText {}
impl ICLKComplicationTemplateGraphicExtraLargeCircularOpenGaugeSimpleText
    for CLKComplicationTemplateGraphicExtraLargeCircularOpenGaugeSimpleText
{
}
pub trait ICLKComplicationTemplateGraphicExtraLargeCircularOpenGaugeSimpleText:
    Sized + std::ops::Deref
{
    unsafe fn initWithGaugeProvider_bottomTextProvider_centerTextProvider_(
        &self,
        gaugeProvider: CLKGaugeProvider,
        bottomTextProvider: CLKTextProvider,
        centerTextProvider: CLKTextProvider,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithGaugeProvider : gaugeProvider, bottomTextProvider : bottomTextProvider, centerTextProvider : centerTextProvider)
    }
    unsafe fn gaugeProvider(&self) -> CLKGaugeProvider
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, gaugeProvider)
    }
    unsafe fn setGaugeProvider_(&self, gaugeProvider: CLKGaugeProvider)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setGaugeProvider : gaugeProvider)
    }
    unsafe fn bottomTextProvider(&self) -> CLKTextProvider
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, bottomTextProvider)
    }
    unsafe fn setBottomTextProvider_(&self, bottomTextProvider: CLKTextProvider)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setBottomTextProvider : bottomTextProvider)
    }
    unsafe fn centerTextProvider(&self) -> CLKTextProvider
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, centerTextProvider)
    }
    unsafe fn setCenterTextProvider_(&self, centerTextProvider: CLKTextProvider)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCenterTextProvider : centerTextProvider)
    }
    unsafe fn templateWithGaugeProvider_bottomTextProvider_centerTextProvider_(
        gaugeProvider: CLKGaugeProvider,
        bottomTextProvider: CLKTextProvider,
        centerTextProvider: CLKTextProvider,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CLKComplicationTemplateGraphicExtraLargeCircularOpenGaugeSimpleText").unwrap(), templateWithGaugeProvider : gaugeProvider, bottomTextProvider : bottomTextProvider, centerTextProvider : centerTextProvider)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CLKComplicationTemplateGraphicExtraLargeCircularOpenGaugeImage(pub id);
impl std::ops::Deref for CLKComplicationTemplateGraphicExtraLargeCircularOpenGaugeImage {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CLKComplicationTemplateGraphicExtraLargeCircularOpenGaugeImage {}
impl CLKComplicationTemplateGraphicExtraLargeCircularOpenGaugeImage {
    pub fn alloc() -> Self {
        Self(unsafe {
            msg_send!(&*objc2::runtime::AnyClass::get(c"CLKComplicationTemplateGraphicExtraLargeCircularOpenGaugeImage").unwrap(), alloc)
        })
    }
}
impl ICLKComplicationTemplateGraphicExtraLargeCircular
    for CLKComplicationTemplateGraphicExtraLargeCircularOpenGaugeImage
{
}
impl std::convert::TryFrom<CLKComplicationTemplateGraphicExtraLargeCircular>
    for CLKComplicationTemplateGraphicExtraLargeCircularOpenGaugeImage
{
    type Error = &'static str;
    fn try_from(
        parent: CLKComplicationTemplateGraphicExtraLargeCircular,
    ) -> Result<CLKComplicationTemplateGraphicExtraLargeCircularOpenGaugeImage, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CLKComplicationTemplateGraphicExtraLargeCircularOpenGaugeImage").unwrap())
        };
        if is_kind_of {
            Ok(CLKComplicationTemplateGraphicExtraLargeCircularOpenGaugeImage(parent.0))
        } else {
            Err ("This CLKComplicationTemplateGraphicExtraLargeCircular cannot be downcasted to CLKComplicationTemplateGraphicExtraLargeCircularOpenGaugeImage" ,)
        }
    }
}
impl ICLKComplicationTemplate for CLKComplicationTemplateGraphicExtraLargeCircularOpenGaugeImage {}
impl PNSCopying for CLKComplicationTemplateGraphicExtraLargeCircularOpenGaugeImage {}
impl INSObject for CLKComplicationTemplateGraphicExtraLargeCircularOpenGaugeImage {}
impl PNSObject for CLKComplicationTemplateGraphicExtraLargeCircularOpenGaugeImage {}
impl ICLKComplicationTemplateGraphicExtraLargeCircularOpenGaugeImage
    for CLKComplicationTemplateGraphicExtraLargeCircularOpenGaugeImage
{
}
pub trait ICLKComplicationTemplateGraphicExtraLargeCircularOpenGaugeImage:
    Sized + std::ops::Deref
{
    unsafe fn initWithGaugeProvider_bottomImageProvider_centerTextProvider_(
        &self,
        gaugeProvider: CLKGaugeProvider,
        bottomImageProvider: CLKFullColorImageProvider,
        centerTextProvider: CLKTextProvider,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithGaugeProvider : gaugeProvider, bottomImageProvider : bottomImageProvider, centerTextProvider : centerTextProvider)
    }
    unsafe fn gaugeProvider(&self) -> CLKGaugeProvider
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, gaugeProvider)
    }
    unsafe fn setGaugeProvider_(&self, gaugeProvider: CLKGaugeProvider)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setGaugeProvider : gaugeProvider)
    }
    unsafe fn bottomImageProvider(&self) -> CLKFullColorImageProvider
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, bottomImageProvider)
    }
    unsafe fn setBottomImageProvider_(&self, bottomImageProvider: CLKFullColorImageProvider)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setBottomImageProvider : bottomImageProvider)
    }
    unsafe fn centerTextProvider(&self) -> CLKTextProvider
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, centerTextProvider)
    }
    unsafe fn setCenterTextProvider_(&self, centerTextProvider: CLKTextProvider)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCenterTextProvider : centerTextProvider)
    }
    unsafe fn templateWithGaugeProvider_bottomImageProvider_centerTextProvider_(
        gaugeProvider: CLKGaugeProvider,
        bottomImageProvider: CLKFullColorImageProvider,
        centerTextProvider: CLKTextProvider,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CLKComplicationTemplateGraphicExtraLargeCircularOpenGaugeImage").unwrap(), templateWithGaugeProvider : gaugeProvider, bottomImageProvider : bottomImageProvider, centerTextProvider : centerTextProvider)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CLKComplicationTemplateGraphicExtraLargeCircularClosedGaugeText(pub id);
impl std::ops::Deref for CLKComplicationTemplateGraphicExtraLargeCircularClosedGaugeText {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CLKComplicationTemplateGraphicExtraLargeCircularClosedGaugeText {}
impl CLKComplicationTemplateGraphicExtraLargeCircularClosedGaugeText {
    pub fn alloc() -> Self {
        Self(unsafe {
            msg_send!(&*objc2::runtime::AnyClass::get(c"CLKComplicationTemplateGraphicExtraLargeCircularClosedGaugeText").unwrap(), alloc)
        })
    }
}
impl ICLKComplicationTemplateGraphicExtraLargeCircular
    for CLKComplicationTemplateGraphicExtraLargeCircularClosedGaugeText
{
}
impl std::convert::TryFrom<CLKComplicationTemplateGraphicExtraLargeCircular>
    for CLKComplicationTemplateGraphicExtraLargeCircularClosedGaugeText
{
    type Error = &'static str;
    fn try_from(
        parent: CLKComplicationTemplateGraphicExtraLargeCircular,
    ) -> Result<CLKComplicationTemplateGraphicExtraLargeCircularClosedGaugeText, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CLKComplicationTemplateGraphicExtraLargeCircularClosedGaugeText").unwrap())
        };
        if is_kind_of {
            Ok(CLKComplicationTemplateGraphicExtraLargeCircularClosedGaugeText(parent.0))
        } else {
            Err ("This CLKComplicationTemplateGraphicExtraLargeCircular cannot be downcasted to CLKComplicationTemplateGraphicExtraLargeCircularClosedGaugeText" ,)
        }
    }
}
impl ICLKComplicationTemplate for CLKComplicationTemplateGraphicExtraLargeCircularClosedGaugeText {}
impl PNSCopying for CLKComplicationTemplateGraphicExtraLargeCircularClosedGaugeText {}
impl INSObject for CLKComplicationTemplateGraphicExtraLargeCircularClosedGaugeText {}
impl PNSObject for CLKComplicationTemplateGraphicExtraLargeCircularClosedGaugeText {}
impl ICLKComplicationTemplateGraphicExtraLargeCircularClosedGaugeText
    for CLKComplicationTemplateGraphicExtraLargeCircularClosedGaugeText
{
}
pub trait ICLKComplicationTemplateGraphicExtraLargeCircularClosedGaugeText:
    Sized + std::ops::Deref
{
    unsafe fn initWithGaugeProvider_centerTextProvider_(
        &self,
        gaugeProvider: CLKGaugeProvider,
        centerTextProvider: CLKTextProvider,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithGaugeProvider : gaugeProvider, centerTextProvider : centerTextProvider)
    }
    unsafe fn gaugeProvider(&self) -> CLKGaugeProvider
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, gaugeProvider)
    }
    unsafe fn setGaugeProvider_(&self, gaugeProvider: CLKGaugeProvider)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setGaugeProvider : gaugeProvider)
    }
    unsafe fn centerTextProvider(&self) -> CLKTextProvider
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, centerTextProvider)
    }
    unsafe fn setCenterTextProvider_(&self, centerTextProvider: CLKTextProvider)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCenterTextProvider : centerTextProvider)
    }
    unsafe fn templateWithGaugeProvider_centerTextProvider_(
        gaugeProvider: CLKGaugeProvider,
        centerTextProvider: CLKTextProvider,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CLKComplicationTemplateGraphicExtraLargeCircularClosedGaugeText").unwrap(), templateWithGaugeProvider : gaugeProvider, centerTextProvider : centerTextProvider)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CLKComplicationTemplateGraphicExtraLargeCircularClosedGaugeImage(pub id);
impl std::ops::Deref for CLKComplicationTemplateGraphicExtraLargeCircularClosedGaugeImage {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CLKComplicationTemplateGraphicExtraLargeCircularClosedGaugeImage {}
impl CLKComplicationTemplateGraphicExtraLargeCircularClosedGaugeImage {
    pub fn alloc() -> Self {
        Self(unsafe {
            msg_send!(&*objc2::runtime::AnyClass::get(c"CLKComplicationTemplateGraphicExtraLargeCircularClosedGaugeImage").unwrap(), alloc)
        })
    }
}
impl ICLKComplicationTemplateGraphicExtraLargeCircular
    for CLKComplicationTemplateGraphicExtraLargeCircularClosedGaugeImage
{
}
impl std::convert::TryFrom<CLKComplicationTemplateGraphicExtraLargeCircular>
    for CLKComplicationTemplateGraphicExtraLargeCircularClosedGaugeImage
{
    type Error = &'static str;
    fn try_from(
        parent: CLKComplicationTemplateGraphicExtraLargeCircular,
    ) -> Result<CLKComplicationTemplateGraphicExtraLargeCircularClosedGaugeImage, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CLKComplicationTemplateGraphicExtraLargeCircularClosedGaugeImage").unwrap())
        };
        if is_kind_of {
            Ok(CLKComplicationTemplateGraphicExtraLargeCircularClosedGaugeImage(parent.0))
        } else {
            Err ("This CLKComplicationTemplateGraphicExtraLargeCircular cannot be downcasted to CLKComplicationTemplateGraphicExtraLargeCircularClosedGaugeImage" ,)
        }
    }
}
impl ICLKComplicationTemplate for CLKComplicationTemplateGraphicExtraLargeCircularClosedGaugeImage {}
impl PNSCopying for CLKComplicationTemplateGraphicExtraLargeCircularClosedGaugeImage {}
impl INSObject for CLKComplicationTemplateGraphicExtraLargeCircularClosedGaugeImage {}
impl PNSObject for CLKComplicationTemplateGraphicExtraLargeCircularClosedGaugeImage {}
impl ICLKComplicationTemplateGraphicExtraLargeCircularClosedGaugeImage
    for CLKComplicationTemplateGraphicExtraLargeCircularClosedGaugeImage
{
}
pub trait ICLKComplicationTemplateGraphicExtraLargeCircularClosedGaugeImage:
    Sized + std::ops::Deref
{
    unsafe fn initWithGaugeProvider_imageProvider_(
        &self,
        gaugeProvider: CLKGaugeProvider,
        imageProvider: CLKFullColorImageProvider,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithGaugeProvider : gaugeProvider, imageProvider : imageProvider)
    }
    unsafe fn gaugeProvider(&self) -> CLKGaugeProvider
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, gaugeProvider)
    }
    unsafe fn setGaugeProvider_(&self, gaugeProvider: CLKGaugeProvider)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setGaugeProvider : gaugeProvider)
    }
    unsafe fn imageProvider(&self) -> CLKFullColorImageProvider
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, imageProvider)
    }
    unsafe fn setImageProvider_(&self, imageProvider: CLKFullColorImageProvider)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setImageProvider : imageProvider)
    }
    unsafe fn templateWithGaugeProvider_imageProvider_(
        gaugeProvider: CLKGaugeProvider,
        imageProvider: CLKFullColorImageProvider,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CLKComplicationTemplateGraphicExtraLargeCircularClosedGaugeImage").unwrap(), templateWithGaugeProvider : gaugeProvider, imageProvider : imageProvider)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CLKComplicationTemplateGraphicExtraLargeCircularStackText(pub id);
impl std::ops::Deref for CLKComplicationTemplateGraphicExtraLargeCircularStackText {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CLKComplicationTemplateGraphicExtraLargeCircularStackText {}
impl CLKComplicationTemplateGraphicExtraLargeCircularStackText {
    pub fn alloc() -> Self {
        Self(unsafe {
            msg_send!(&*objc2::runtime::AnyClass::get(c"CLKComplicationTemplateGraphicExtraLargeCircularStackText").unwrap(), alloc)
        })
    }
}
impl ICLKComplicationTemplateGraphicExtraLargeCircular
    for CLKComplicationTemplateGraphicExtraLargeCircularStackText
{
}
impl std::convert::TryFrom<CLKComplicationTemplateGraphicExtraLargeCircular>
    for CLKComplicationTemplateGraphicExtraLargeCircularStackText
{
    type Error = &'static str;
    fn try_from(
        parent: CLKComplicationTemplateGraphicExtraLargeCircular,
    ) -> Result<CLKComplicationTemplateGraphicExtraLargeCircularStackText, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CLKComplicationTemplateGraphicExtraLargeCircularStackText").unwrap())
        };
        if is_kind_of {
            Ok(CLKComplicationTemplateGraphicExtraLargeCircularStackText(
                parent.0,
            ))
        } else {
            Err ("This CLKComplicationTemplateGraphicExtraLargeCircular cannot be downcasted to CLKComplicationTemplateGraphicExtraLargeCircularStackText" ,)
        }
    }
}
impl ICLKComplicationTemplate for CLKComplicationTemplateGraphicExtraLargeCircularStackText {}
impl PNSCopying for CLKComplicationTemplateGraphicExtraLargeCircularStackText {}
impl INSObject for CLKComplicationTemplateGraphicExtraLargeCircularStackText {}
impl PNSObject for CLKComplicationTemplateGraphicExtraLargeCircularStackText {}
impl ICLKComplicationTemplateGraphicExtraLargeCircularStackText
    for CLKComplicationTemplateGraphicExtraLargeCircularStackText
{
}
pub trait ICLKComplicationTemplateGraphicExtraLargeCircularStackText:
    Sized + std::ops::Deref
{
    unsafe fn initWithLine1TextProvider_line2TextProvider_(
        &self,
        line1TextProvider: CLKTextProvider,
        line2TextProvider: CLKTextProvider,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithLine1TextProvider : line1TextProvider, line2TextProvider : line2TextProvider)
    }
    unsafe fn line1TextProvider(&self) -> CLKTextProvider
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, line1TextProvider)
    }
    unsafe fn setLine1TextProvider_(&self, line1TextProvider: CLKTextProvider)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setLine1TextProvider : line1TextProvider)
    }
    unsafe fn line2TextProvider(&self) -> CLKTextProvider
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, line2TextProvider)
    }
    unsafe fn setLine2TextProvider_(&self, line2TextProvider: CLKTextProvider)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setLine2TextProvider : line2TextProvider)
    }
    unsafe fn templateWithLine1TextProvider_line2TextProvider_(
        line1TextProvider: CLKTextProvider,
        line2TextProvider: CLKTextProvider,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CLKComplicationTemplateGraphicExtraLargeCircularStackText").unwrap(), templateWithLine1TextProvider : line1TextProvider, line2TextProvider : line2TextProvider)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CLKComplicationTemplateGraphicExtraLargeCircularStackImage(pub id);
impl std::ops::Deref for CLKComplicationTemplateGraphicExtraLargeCircularStackImage {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CLKComplicationTemplateGraphicExtraLargeCircularStackImage {}
impl CLKComplicationTemplateGraphicExtraLargeCircularStackImage {
    pub fn alloc() -> Self {
        Self(unsafe {
            msg_send!(&*objc2::runtime::AnyClass::get(c"CLKComplicationTemplateGraphicExtraLargeCircularStackImage").unwrap(), alloc)
        })
    }
}
impl ICLKComplicationTemplateGraphicExtraLargeCircular
    for CLKComplicationTemplateGraphicExtraLargeCircularStackImage
{
}
impl std::convert::TryFrom<CLKComplicationTemplateGraphicExtraLargeCircular>
    for CLKComplicationTemplateGraphicExtraLargeCircularStackImage
{
    type Error = &'static str;
    fn try_from(
        parent: CLKComplicationTemplateGraphicExtraLargeCircular,
    ) -> Result<CLKComplicationTemplateGraphicExtraLargeCircularStackImage, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CLKComplicationTemplateGraphicExtraLargeCircularStackImage").unwrap())
        };
        if is_kind_of {
            Ok(CLKComplicationTemplateGraphicExtraLargeCircularStackImage(
                parent.0,
            ))
        } else {
            Err ("This CLKComplicationTemplateGraphicExtraLargeCircular cannot be downcasted to CLKComplicationTemplateGraphicExtraLargeCircularStackImage" ,)
        }
    }
}
impl ICLKComplicationTemplate for CLKComplicationTemplateGraphicExtraLargeCircularStackImage {}
impl PNSCopying for CLKComplicationTemplateGraphicExtraLargeCircularStackImage {}
impl INSObject for CLKComplicationTemplateGraphicExtraLargeCircularStackImage {}
impl PNSObject for CLKComplicationTemplateGraphicExtraLargeCircularStackImage {}
impl ICLKComplicationTemplateGraphicExtraLargeCircularStackImage
    for CLKComplicationTemplateGraphicExtraLargeCircularStackImage
{
}
pub trait ICLKComplicationTemplateGraphicExtraLargeCircularStackImage:
    Sized + std::ops::Deref
{
    unsafe fn initWithLine1ImageProvider_line2TextProvider_(
        &self,
        line1ImageProvider: CLKFullColorImageProvider,
        line2TextProvider: CLKTextProvider,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithLine1ImageProvider : line1ImageProvider, line2TextProvider : line2TextProvider)
    }
    unsafe fn line1ImageProvider(&self) -> CLKFullColorImageProvider
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, line1ImageProvider)
    }
    unsafe fn setLine1ImageProvider_(&self, line1ImageProvider: CLKFullColorImageProvider)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setLine1ImageProvider : line1ImageProvider)
    }
    unsafe fn line2TextProvider(&self) -> CLKTextProvider
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, line2TextProvider)
    }
    unsafe fn setLine2TextProvider_(&self, line2TextProvider: CLKTextProvider)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setLine2TextProvider : line2TextProvider)
    }
    unsafe fn templateWithLine1ImageProvider_line2TextProvider_(
        line1ImageProvider: CLKFullColorImageProvider,
        line2TextProvider: CLKTextProvider,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CLKComplicationTemplateGraphicExtraLargeCircularStackImage").unwrap(), templateWithLine1ImageProvider : line1ImageProvider, line2TextProvider : line2TextProvider)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CLKComplicationTimelineEntry(pub id);
impl std::ops::Deref for CLKComplicationTimelineEntry {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CLKComplicationTimelineEntry {}
impl CLKComplicationTimelineEntry {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CLKComplicationTimelineEntry").unwrap(), alloc) })
    }
}
impl INSObject for CLKComplicationTimelineEntry {}
impl PNSObject for CLKComplicationTimelineEntry {}
impl std::convert::TryFrom<NSObject> for CLKComplicationTimelineEntry {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<CLKComplicationTimelineEntry, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CLKComplicationTimelineEntry").unwrap()) };
        if is_kind_of {
            Ok(CLKComplicationTimelineEntry(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to CLKComplicationTimelineEntry")
        }
    }
}
impl ICLKComplicationTimelineEntry for CLKComplicationTimelineEntry {}
pub trait ICLKComplicationTimelineEntry: Sized + std::ops::Deref {
    unsafe fn date(&self) -> NSDate
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, date)
    }
    unsafe fn setDate_(&self, date: NSDate)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDate : date)
    }
    unsafe fn complicationTemplate(&self) -> CLKComplicationTemplate
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, complicationTemplate)
    }
    unsafe fn setComplicationTemplate_(&self, complicationTemplate: CLKComplicationTemplate)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setComplicationTemplate : complicationTemplate)
    }
    unsafe fn timelineAnimationGroup(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, timelineAnimationGroup)
    }
    unsafe fn setTimelineAnimationGroup_(&self, timelineAnimationGroup: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setTimelineAnimationGroup : timelineAnimationGroup)
    }
    unsafe fn entryWithDate_complicationTemplate_(
        date: NSDate,
        complicationTemplate: CLKComplicationTemplate,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CLKComplicationTimelineEntry").unwrap(), entryWithDate : date, complicationTemplate : complicationTemplate)
    }
    unsafe fn entryWithDate_complicationTemplate_timelineAnimationGroup_(
        date: NSDate,
        complicationTemplate: CLKComplicationTemplate,
        timelineAnimationGroup: NSString,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CLKComplicationTimelineEntry").unwrap(), entryWithDate : date, complicationTemplate : complicationTemplate, timelineAnimationGroup : timelineAnimationGroup)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CLKImageProvider(pub id);
impl std::ops::Deref for CLKImageProvider {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CLKImageProvider {}
impl CLKImageProvider {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CLKImageProvider").unwrap(), alloc) })
    }
}
impl PNSCopying for CLKImageProvider {}
impl INSObject for CLKImageProvider {}
impl PNSObject for CLKImageProvider {}
impl std::convert::TryFrom<NSObject> for CLKImageProvider {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<CLKImageProvider, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CLKImageProvider").unwrap()) };
        if is_kind_of {
            Ok(CLKImageProvider(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to CLKImageProvider")
        }
    }
}
impl ICLKImageProvider for CLKImageProvider {}
pub trait ICLKImageProvider: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn initWithOnePieceImage_(&self, onePieceImage: UIImage) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithOnePieceImage : onePieceImage)
    }
    unsafe fn initWithOnePieceImage_twoPieceImageBackground_twoPieceImageForeground_(
        &self,
        onePieceImage: UIImage,
        twoPieceImageBackground: UIImage,
        twoPieceImageForeground: UIImage,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithOnePieceImage : onePieceImage, twoPieceImageBackground : twoPieceImageBackground, twoPieceImageForeground : twoPieceImageForeground)
    }
    unsafe fn onePieceImage(&self) -> UIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, onePieceImage)
    }
    unsafe fn setOnePieceImage_(&self, onePieceImage: UIImage)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setOnePieceImage : onePieceImage)
    }
    unsafe fn tintColor(&self) -> UIColor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, tintColor)
    }
    unsafe fn setTintColor_(&self, tintColor: UIColor)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setTintColor : tintColor)
    }
    unsafe fn twoPieceImageBackground(&self) -> UIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, twoPieceImageBackground)
    }
    unsafe fn setTwoPieceImageBackground_(&self, twoPieceImageBackground: UIImage)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setTwoPieceImageBackground : twoPieceImageBackground)
    }
    unsafe fn twoPieceImageForeground(&self) -> UIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, twoPieceImageForeground)
    }
    unsafe fn setTwoPieceImageForeground_(&self, twoPieceImageForeground: UIImage)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setTwoPieceImageForeground : twoPieceImageForeground)
    }
    unsafe fn accessibilityLabel(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, accessibilityLabel)
    }
    unsafe fn setAccessibilityLabel_(&self, accessibilityLabel: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAccessibilityLabel : accessibilityLabel)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CLKImageProvider").unwrap(), new)
    }
    unsafe fn imageProviderWithOnePieceImage_(onePieceImage: UIImage) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CLKImageProvider").unwrap(), imageProviderWithOnePieceImage : onePieceImage)
    }
    unsafe fn imageProviderWithOnePieceImage_twoPieceImageBackground_twoPieceImageForeground_(
        onePieceImage: UIImage,
        twoPieceImageBackground: UIImage,
        twoPieceImageForeground: UIImage,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CLKImageProvider").unwrap(), imageProviderWithOnePieceImage : onePieceImage, twoPieceImageBackground : twoPieceImageBackground, twoPieceImageForeground : twoPieceImageForeground)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CLKTextProvider(pub id);
impl std::ops::Deref for CLKTextProvider {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CLKTextProvider {}
impl CLKTextProvider {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CLKTextProvider").unwrap(), alloc) })
    }
}
impl PNSCopying for CLKTextProvider {}
impl INSObject for CLKTextProvider {}
impl PNSObject for CLKTextProvider {}
impl std::convert::TryFrom<NSObject> for CLKTextProvider {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<CLKTextProvider, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CLKTextProvider").unwrap()) };
        if is_kind_of {
            Ok(CLKTextProvider(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to CLKTextProvider")
        }
    }
}
impl ICLKTextProvider for CLKTextProvider {}
pub trait ICLKTextProvider: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn tintColor(&self) -> UIColor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, tintColor)
    }
    unsafe fn setTintColor_(&self, tintColor: UIColor)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setTintColor : tintColor)
    }
    unsafe fn accessibilityLabel(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, accessibilityLabel)
    }
    unsafe fn setAccessibilityLabel_(&self, accessibilityLabel: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAccessibilityLabel : accessibilityLabel)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CLKTextProvider").unwrap(), new)
    }
    unsafe fn textProviderWithFormat_(format: NSString) -> CLKTextProvider
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CLKTextProvider").unwrap(), textProviderWithFormat : format)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CLKSimpleTextProvider(pub id);
impl std::ops::Deref for CLKSimpleTextProvider {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CLKSimpleTextProvider {}
impl CLKSimpleTextProvider {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CLKSimpleTextProvider").unwrap(), alloc) })
    }
}
impl ICLKTextProvider for CLKSimpleTextProvider {}
impl PNSCopying for CLKSimpleTextProvider {}
impl From<CLKSimpleTextProvider> for CLKTextProvider {
    fn from(child: CLKSimpleTextProvider) -> CLKTextProvider {
        CLKTextProvider(child.0)
    }
}
impl std::convert::TryFrom<CLKTextProvider> for CLKSimpleTextProvider {
    type Error = &'static str;
    fn try_from(parent: CLKTextProvider) -> Result<CLKSimpleTextProvider, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CLKSimpleTextProvider").unwrap()) };
        if is_kind_of {
            Ok(CLKSimpleTextProvider(parent.0))
        } else {
            Err("This CLKTextProvider cannot be downcasted to CLKSimpleTextProvider")
        }
    }
}
impl INSObject for CLKSimpleTextProvider {}
impl PNSObject for CLKSimpleTextProvider {}
impl ICLKSimpleTextProvider for CLKSimpleTextProvider {}
pub trait ICLKSimpleTextProvider: Sized + std::ops::Deref {
    unsafe fn initWithText_(&self, text: NSString) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithText : text)
    }
    unsafe fn initWithText_shortText_(&self, text: NSString, shortText: NSString) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithText : text, shortText : shortText)
    }
    unsafe fn initWithText_shortText_accessibilityLabel_(
        &self,
        text: NSString,
        shortText: NSString,
        accessibilityLabel: NSString,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithText : text, shortText : shortText, accessibilityLabel : accessibilityLabel)
    }
    unsafe fn text(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, text)
    }
    unsafe fn setText_(&self, text: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setText : text)
    }
    unsafe fn shortText(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, shortText)
    }
    unsafe fn setShortText_(&self, shortText: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setShortText : shortText)
    }
    unsafe fn textProviderWithText_(text: NSString) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CLKSimpleTextProvider").unwrap(), textProviderWithText : text)
    }
    unsafe fn textProviderWithText_shortText_(text: NSString, shortText: NSString) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CLKSimpleTextProvider").unwrap(), textProviderWithText : text, shortText : shortText)
    }
    unsafe fn textProviderWithText_shortText_accessibilityLabel_(
        text: NSString,
        shortText: NSString,
        accessibilityLabel: NSString,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CLKSimpleTextProvider").unwrap(), textProviderWithText : text, shortText : shortText, accessibilityLabel : accessibilityLabel)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CLKDateTextProvider(pub id);
impl std::ops::Deref for CLKDateTextProvider {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CLKDateTextProvider {}
impl CLKDateTextProvider {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CLKDateTextProvider").unwrap(), alloc) })
    }
}
impl ICLKTextProvider for CLKDateTextProvider {}
impl PNSCopying for CLKDateTextProvider {}
impl std::convert::TryFrom<CLKTextProvider> for CLKDateTextProvider {
    type Error = &'static str;
    fn try_from(parent: CLKTextProvider) -> Result<CLKDateTextProvider, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CLKDateTextProvider").unwrap()) };
        if is_kind_of {
            Ok(CLKDateTextProvider(parent.0))
        } else {
            Err("This CLKTextProvider cannot be downcasted to CLKDateTextProvider")
        }
    }
}
impl INSObject for CLKDateTextProvider {}
impl PNSObject for CLKDateTextProvider {}
impl ICLKDateTextProvider for CLKDateTextProvider {}
pub trait ICLKDateTextProvider: Sized + std::ops::Deref {
    unsafe fn initWithDate_units_(
        &self,
        date: NSDate,
        calendarUnits: NSCalendarUnit,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithDate : date, units : calendarUnits)
    }
    unsafe fn initWithDate_units_timeZone_(
        &self,
        date: NSDate,
        calendarUnits: NSCalendarUnit,
        timeZone: NSTimeZone,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithDate : date, units : calendarUnits, timeZone : timeZone)
    }
    unsafe fn date(&self) -> NSDate
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, date)
    }
    unsafe fn setDate_(&self, date: NSDate)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDate : date)
    }
    unsafe fn calendarUnits(&self) -> NSCalendarUnit
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, calendarUnits)
    }
    unsafe fn setCalendarUnits_(&self, calendarUnits: NSCalendarUnit)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCalendarUnits : calendarUnits)
    }
    unsafe fn timeZone(&self) -> NSTimeZone
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, timeZone)
    }
    unsafe fn setTimeZone_(&self, timeZone: NSTimeZone)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setTimeZone : timeZone)
    }
    unsafe fn uppercase(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, uppercase)
    }
    unsafe fn setUppercase_(&self, uppercase: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setUppercase : uppercase)
    }
    unsafe fn textProviderWithDate_units_(
        date: NSDate,
        calendarUnits: NSCalendarUnit,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CLKDateTextProvider").unwrap(), textProviderWithDate : date, units : calendarUnits)
    }
    unsafe fn textProviderWithDate_units_timeZone_(
        date: NSDate,
        calendarUnits: NSCalendarUnit,
        timeZone: NSTimeZone,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CLKDateTextProvider").unwrap(), textProviderWithDate : date, units : calendarUnits, timeZone : timeZone)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CLKTimeTextProvider(pub id);
impl std::ops::Deref for CLKTimeTextProvider {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CLKTimeTextProvider {}
impl CLKTimeTextProvider {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CLKTimeTextProvider").unwrap(), alloc) })
    }
}
impl ICLKTextProvider for CLKTimeTextProvider {}
impl PNSCopying for CLKTimeTextProvider {}
impl std::convert::TryFrom<CLKTextProvider> for CLKTimeTextProvider {
    type Error = &'static str;
    fn try_from(parent: CLKTextProvider) -> Result<CLKTimeTextProvider, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CLKTimeTextProvider").unwrap()) };
        if is_kind_of {
            Ok(CLKTimeTextProvider(parent.0))
        } else {
            Err("This CLKTextProvider cannot be downcasted to CLKTimeTextProvider")
        }
    }
}
impl INSObject for CLKTimeTextProvider {}
impl PNSObject for CLKTimeTextProvider {}
impl ICLKTimeTextProvider for CLKTimeTextProvider {}
pub trait ICLKTimeTextProvider: Sized + std::ops::Deref {
    unsafe fn initWithDate_(&self, date: NSDate) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithDate : date)
    }
    unsafe fn initWithDate_timeZone_(&self, date: NSDate, timeZone: NSTimeZone) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithDate : date, timeZone : timeZone)
    }
    unsafe fn date(&self) -> NSDate
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, date)
    }
    unsafe fn setDate_(&self, date: NSDate)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDate : date)
    }
    unsafe fn timeZone(&self) -> NSTimeZone
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, timeZone)
    }
    unsafe fn setTimeZone_(&self, timeZone: NSTimeZone)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setTimeZone : timeZone)
    }
    unsafe fn textProviderWithDate_(date: NSDate) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CLKTimeTextProvider").unwrap(), textProviderWithDate : date)
    }
    unsafe fn textProviderWithDate_timeZone_(date: NSDate, timeZone: NSTimeZone) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CLKTimeTextProvider").unwrap(), textProviderWithDate : date, timeZone : timeZone)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CLKTimeIntervalTextProvider(pub id);
impl std::ops::Deref for CLKTimeIntervalTextProvider {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CLKTimeIntervalTextProvider {}
impl CLKTimeIntervalTextProvider {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CLKTimeIntervalTextProvider").unwrap(), alloc) })
    }
}
impl ICLKTextProvider for CLKTimeIntervalTextProvider {}
impl PNSCopying for CLKTimeIntervalTextProvider {}
impl std::convert::TryFrom<CLKTextProvider> for CLKTimeIntervalTextProvider {
    type Error = &'static str;
    fn try_from(parent: CLKTextProvider) -> Result<CLKTimeIntervalTextProvider, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CLKTimeIntervalTextProvider").unwrap()) };
        if is_kind_of {
            Ok(CLKTimeIntervalTextProvider(parent.0))
        } else {
            Err("This CLKTextProvider cannot be downcasted to CLKTimeIntervalTextProvider")
        }
    }
}
impl INSObject for CLKTimeIntervalTextProvider {}
impl PNSObject for CLKTimeIntervalTextProvider {}
impl ICLKTimeIntervalTextProvider for CLKTimeIntervalTextProvider {}
pub trait ICLKTimeIntervalTextProvider: Sized + std::ops::Deref {
    unsafe fn initWithStartDate_endDate_(&self, startDate: NSDate, endDate: NSDate) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithStartDate : startDate, endDate : endDate)
    }
    unsafe fn initWithStartDate_endDate_timeZone_(
        &self,
        startDate: NSDate,
        endDate: NSDate,
        timeZone: NSTimeZone,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithStartDate : startDate, endDate : endDate, timeZone : timeZone)
    }
    unsafe fn startDate(&self) -> NSDate
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, startDate)
    }
    unsafe fn setStartDate_(&self, startDate: NSDate)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setStartDate : startDate)
    }
    unsafe fn endDate(&self) -> NSDate
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, endDate)
    }
    unsafe fn setEndDate_(&self, endDate: NSDate)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setEndDate : endDate)
    }
    unsafe fn timeZone(&self) -> NSTimeZone
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, timeZone)
    }
    unsafe fn setTimeZone_(&self, timeZone: NSTimeZone)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setTimeZone : timeZone)
    }
    unsafe fn textProviderWithStartDate_endDate_(startDate: NSDate, endDate: NSDate) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CLKTimeIntervalTextProvider").unwrap(), textProviderWithStartDate : startDate, endDate : endDate)
    }
    unsafe fn textProviderWithStartDate_endDate_timeZone_(
        startDate: NSDate,
        endDate: NSDate,
        timeZone: NSTimeZone,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CLKTimeIntervalTextProvider").unwrap(), textProviderWithStartDate : startDate, endDate : endDate, timeZone : timeZone)
    }
}
pub type CLKRelativeDateStyle = NSInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CLKRelativeDateTextProvider(pub id);
impl std::ops::Deref for CLKRelativeDateTextProvider {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CLKRelativeDateTextProvider {}
impl CLKRelativeDateTextProvider {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CLKRelativeDateTextProvider").unwrap(), alloc) })
    }
}
impl ICLKTextProvider for CLKRelativeDateTextProvider {}
impl PNSCopying for CLKRelativeDateTextProvider {}
impl std::convert::TryFrom<CLKTextProvider> for CLKRelativeDateTextProvider {
    type Error = &'static str;
    fn try_from(parent: CLKTextProvider) -> Result<CLKRelativeDateTextProvider, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CLKRelativeDateTextProvider").unwrap()) };
        if is_kind_of {
            Ok(CLKRelativeDateTextProvider(parent.0))
        } else {
            Err("This CLKTextProvider cannot be downcasted to CLKRelativeDateTextProvider")
        }
    }
}
impl INSObject for CLKRelativeDateTextProvider {}
impl PNSObject for CLKRelativeDateTextProvider {}
impl ICLKRelativeDateTextProvider for CLKRelativeDateTextProvider {}
pub trait ICLKRelativeDateTextProvider: Sized + std::ops::Deref {
    unsafe fn initWithDate_style_units_(
        &self,
        date: NSDate,
        style: CLKRelativeDateStyle,
        calendarUnits: NSCalendarUnit,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithDate : date, style : style, units : calendarUnits)
    }
    unsafe fn initWithDate_relativeToDate_style_units_(
        &self,
        date: NSDate,
        relativeDate: NSDate,
        style: CLKRelativeDateStyle,
        calendarUnits: NSCalendarUnit,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithDate : date, relativeToDate : relativeDate, style : style, units : calendarUnits)
    }
    unsafe fn date(&self) -> NSDate
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, date)
    }
    unsafe fn setDate_(&self, date: NSDate)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDate : date)
    }
    unsafe fn relativeToDate(&self) -> NSDate
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, relativeToDate)
    }
    unsafe fn setRelativeToDate_(&self, relativeToDate: NSDate)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRelativeToDate : relativeToDate)
    }
    unsafe fn relativeDateStyle(&self) -> CLKRelativeDateStyle
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, relativeDateStyle)
    }
    unsafe fn setRelativeDateStyle_(&self, relativeDateStyle: CLKRelativeDateStyle)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRelativeDateStyle : relativeDateStyle)
    }
    unsafe fn calendarUnits(&self) -> NSCalendarUnit
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, calendarUnits)
    }
    unsafe fn setCalendarUnits_(&self, calendarUnits: NSCalendarUnit)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCalendarUnits : calendarUnits)
    }
    unsafe fn textProviderWithDate_style_units_(
        date: NSDate,
        style: CLKRelativeDateStyle,
        calendarUnits: NSCalendarUnit,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CLKRelativeDateTextProvider").unwrap(), textProviderWithDate : date, style : style, units : calendarUnits)
    }
    unsafe fn textProviderWithDate_relativeToDate_style_units_(
        date: NSDate,
        relativeToDate: NSDate,
        style: CLKRelativeDateStyle,
        calendarUnits: NSCalendarUnit,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CLKRelativeDateTextProvider").unwrap(), textProviderWithDate : date, relativeToDate : relativeToDate, style : style, units : calendarUnits)
    }
}
impl CLKTextProvider_Localizable for CLKTextProvider {}
pub trait CLKTextProvider_Localizable: Sized + std::ops::Deref {
    unsafe fn localizableTextProviderWithStringsFileTextKey_(textKey: NSString) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CLKTextProvider").unwrap(), localizableTextProviderWithStringsFileTextKey : textKey)
    }
    unsafe fn localizableTextProviderWithStringsFileTextKey_shortTextKey_(
        textKey: NSString,
        shortTextKey: NSString,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CLKTextProvider").unwrap(), localizableTextProviderWithStringsFileTextKey : textKey, shortTextKey : shortTextKey)
    }
    unsafe fn localizableTextProviderWithStringsFileFormatKey_textProviders_(
        formatKey: NSString,
        textProviders: NSArray,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CLKTextProvider").unwrap(), localizableTextProviderWithStringsFileFormatKey : formatKey, textProviders : textProviders)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CLKGaugeProvider(pub id);
impl std::ops::Deref for CLKGaugeProvider {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CLKGaugeProvider {}
impl CLKGaugeProvider {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CLKGaugeProvider").unwrap(), alloc) })
    }
}
impl PNSCopying for CLKGaugeProvider {}
impl INSObject for CLKGaugeProvider {}
impl PNSObject for CLKGaugeProvider {}
impl std::convert::TryFrom<NSObject> for CLKGaugeProvider {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<CLKGaugeProvider, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CLKGaugeProvider").unwrap()) };
        if is_kind_of {
            Ok(CLKGaugeProvider(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to CLKGaugeProvider")
        }
    }
}
impl ICLKGaugeProvider for CLKGaugeProvider {}
pub trait ICLKGaugeProvider: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn style(&self) -> CLKGaugeProviderStyle
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, style)
    }
    unsafe fn gaugeColors(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, gaugeColors)
    }
    unsafe fn gaugeColorLocations(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, gaugeColorLocations)
    }
    unsafe fn accessibilityLabel(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, accessibilityLabel)
    }
    unsafe fn setAccessibilityLabel_(&self, accessibilityLabel: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAccessibilityLabel : accessibilityLabel)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CLKGaugeProvider").unwrap(), new)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CLKSimpleGaugeProvider(pub id);
impl std::ops::Deref for CLKSimpleGaugeProvider {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CLKSimpleGaugeProvider {}
impl CLKSimpleGaugeProvider {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CLKSimpleGaugeProvider").unwrap(), alloc) })
    }
}
impl ICLKGaugeProvider for CLKSimpleGaugeProvider {}
impl PNSCopying for CLKSimpleGaugeProvider {}
impl From<CLKSimpleGaugeProvider> for CLKGaugeProvider {
    fn from(child: CLKSimpleGaugeProvider) -> CLKGaugeProvider {
        CLKGaugeProvider(child.0)
    }
}
impl std::convert::TryFrom<CLKGaugeProvider> for CLKSimpleGaugeProvider {
    type Error = &'static str;
    fn try_from(parent: CLKGaugeProvider) -> Result<CLKSimpleGaugeProvider, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CLKSimpleGaugeProvider").unwrap()) };
        if is_kind_of {
            Ok(CLKSimpleGaugeProvider(parent.0))
        } else {
            Err("This CLKGaugeProvider cannot be downcasted to CLKSimpleGaugeProvider")
        }
    }
}
impl INSObject for CLKSimpleGaugeProvider {}
impl PNSObject for CLKSimpleGaugeProvider {}
impl ICLKSimpleGaugeProvider for CLKSimpleGaugeProvider {}
pub trait ICLKSimpleGaugeProvider: Sized + std::ops::Deref {
    unsafe fn fillFraction(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, fillFraction)
    }
    unsafe fn gaugeProviderWithStyle_gaugeColors_gaugeColorLocations_fillFraction_(
        style: CLKGaugeProviderStyle,
        gaugeColors: NSArray,
        gaugeColorLocations: NSArray,
        fillFraction: f32,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CLKSimpleGaugeProvider").unwrap(), gaugeProviderWithStyle : style, gaugeColors : gaugeColors, gaugeColorLocations : gaugeColorLocations, fillFraction : fillFraction)
    }
    unsafe fn gaugeProviderWithStyle_gaugeColor_fillFraction_(
        style: CLKGaugeProviderStyle,
        color: UIColor,
        fillFraction: f32,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CLKSimpleGaugeProvider").unwrap(), gaugeProviderWithStyle : style, gaugeColor : color, fillFraction : fillFraction)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CLKTimeIntervalGaugeProvider(pub id);
impl std::ops::Deref for CLKTimeIntervalGaugeProvider {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CLKTimeIntervalGaugeProvider {}
impl CLKTimeIntervalGaugeProvider {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CLKTimeIntervalGaugeProvider").unwrap(), alloc) })
    }
}
impl ICLKGaugeProvider for CLKTimeIntervalGaugeProvider {}
impl PNSCopying for CLKTimeIntervalGaugeProvider {}
impl std::convert::TryFrom<CLKGaugeProvider> for CLKTimeIntervalGaugeProvider {
    type Error = &'static str;
    fn try_from(parent: CLKGaugeProvider) -> Result<CLKTimeIntervalGaugeProvider, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CLKTimeIntervalGaugeProvider").unwrap()) };
        if is_kind_of {
            Ok(CLKTimeIntervalGaugeProvider(parent.0))
        } else {
            Err("This CLKGaugeProvider cannot be downcasted to CLKTimeIntervalGaugeProvider")
        }
    }
}
impl INSObject for CLKTimeIntervalGaugeProvider {}
impl PNSObject for CLKTimeIntervalGaugeProvider {}
impl ICLKTimeIntervalGaugeProvider for CLKTimeIntervalGaugeProvider {}
pub trait ICLKTimeIntervalGaugeProvider: Sized + std::ops::Deref {
    unsafe fn startDate(&self) -> NSDate
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, startDate)
    }
    unsafe fn endDate(&self) -> NSDate
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, endDate)
    }
    unsafe fn startFillFraction(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, startFillFraction)
    }
    unsafe fn endFillFraction(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, endFillFraction)
    }
    unsafe fn gaugeProviderWithStyle_gaugeColors_gaugeColorLocations_startDate_endDate_(
        style: CLKGaugeProviderStyle,
        gaugeColors: NSArray,
        gaugeColorLocations: NSArray,
        startDate: NSDate,
        endDate: NSDate,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CLKTimeIntervalGaugeProvider").unwrap(), gaugeProviderWithStyle : style, gaugeColors : gaugeColors, gaugeColorLocations : gaugeColorLocations, startDate : startDate, endDate : endDate)
    }
    unsafe fn gaugeProviderWithStyle_gaugeColors_gaugeColorLocations_startDate_startFillFraction_endDate_endFillFraction_(
        style: CLKGaugeProviderStyle,
        gaugeColors: NSArray,
        gaugeColorLocations: NSArray,
        startDate: NSDate,
        startFillFraction: f32,
        endDate: NSDate,
        endFillFraction: f32,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CLKTimeIntervalGaugeProvider").unwrap(), gaugeProviderWithStyle : style, gaugeColors : gaugeColors, gaugeColorLocations : gaugeColorLocations, startDate : startDate, startFillFraction : startFillFraction, endDate : endDate, endFillFraction : endFillFraction)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CLKFullColorImageProvider(pub id);
impl std::ops::Deref for CLKFullColorImageProvider {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CLKFullColorImageProvider {}
impl CLKFullColorImageProvider {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CLKFullColorImageProvider").unwrap(), alloc) })
    }
}
impl PNSCopying for CLKFullColorImageProvider {}
impl INSObject for CLKFullColorImageProvider {}
impl PNSObject for CLKFullColorImageProvider {}
impl std::convert::TryFrom<NSObject> for CLKFullColorImageProvider {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<CLKFullColorImageProvider, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CLKFullColorImageProvider").unwrap()) };
        if is_kind_of {
            Ok(CLKFullColorImageProvider(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to CLKFullColorImageProvider")
        }
    }
}
impl ICLKFullColorImageProvider for CLKFullColorImageProvider {}
pub trait ICLKFullColorImageProvider: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn initWithFullColorImage_(&self, image: UIImage) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithFullColorImage : image)
    }
    unsafe fn initWithFullColorImage_tintedImageProvider_(
        &self,
        image: UIImage,
        tintedImageProvider: CLKImageProvider,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithFullColorImage : image, tintedImageProvider : tintedImageProvider)
    }
    unsafe fn image(&self) -> UIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, image)
    }
    unsafe fn setImage_(&self, image: UIImage)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setImage : image)
    }
    unsafe fn tintedImageProvider(&self) -> CLKImageProvider
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, tintedImageProvider)
    }
    unsafe fn setTintedImageProvider_(&self, tintedImageProvider: CLKImageProvider)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setTintedImageProvider : tintedImageProvider)
    }
    unsafe fn accessibilityLabel(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, accessibilityLabel)
    }
    unsafe fn setAccessibilityLabel_(&self, accessibilityLabel: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAccessibilityLabel : accessibilityLabel)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CLKFullColorImageProvider").unwrap(), new)
    }
    unsafe fn providerWithFullColorImage_(image: UIImage) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CLKFullColorImageProvider").unwrap(), providerWithFullColorImage : image)
    }
    unsafe fn providerWithFullColorImage_tintedImageProvider_(
        image: UIImage,
        tintedImageProvider: CLKImageProvider,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CLKFullColorImageProvider").unwrap(), providerWithFullColorImage : image, tintedImageProvider : tintedImageProvider)
    }
}
pub type CLKWatchFaceLibraryErrorCode = NSInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CLKWatchFaceLibrary(pub id);
impl std::ops::Deref for CLKWatchFaceLibrary {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CLKWatchFaceLibrary {}
impl CLKWatchFaceLibrary {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CLKWatchFaceLibrary").unwrap(), alloc) })
    }
}
impl INSObject for CLKWatchFaceLibrary {}
impl PNSObject for CLKWatchFaceLibrary {}
impl std::convert::TryFrom<NSObject> for CLKWatchFaceLibrary {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<CLKWatchFaceLibrary, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CLKWatchFaceLibrary").unwrap()) };
        if is_kind_of {
            Ok(CLKWatchFaceLibrary(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to CLKWatchFaceLibrary")
        }
    }
}
impl ICLKWatchFaceLibrary for CLKWatchFaceLibrary {}
pub trait ICLKWatchFaceLibrary: Sized + std::ops::Deref {
    unsafe fn addWatchFaceAtURL_completionHandler_(
        &self,
        fileURL: NSURL,
        handler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, addWatchFaceAtURL : fileURL, completionHandler : handler)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CLKComplicationDescriptor(pub id);
impl std::ops::Deref for CLKComplicationDescriptor {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CLKComplicationDescriptor {}
impl CLKComplicationDescriptor {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CLKComplicationDescriptor").unwrap(), alloc) })
    }
}
impl INSObject for CLKComplicationDescriptor {}
impl PNSObject for CLKComplicationDescriptor {}
impl std::convert::TryFrom<NSObject> for CLKComplicationDescriptor {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<CLKComplicationDescriptor, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CLKComplicationDescriptor").unwrap()) };
        if is_kind_of {
            Ok(CLKComplicationDescriptor(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to CLKComplicationDescriptor")
        }
    }
}
impl ICLKComplicationDescriptor for CLKComplicationDescriptor {}
pub trait ICLKComplicationDescriptor: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn initWithIdentifier_displayName_supportedFamilies_(
        &self,
        identifier: NSString,
        displayName: NSString,
        supportedFamilies: NSArray,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithIdentifier : identifier, displayName : displayName, supportedFamilies : supportedFamilies)
    }
    unsafe fn initWithIdentifier_displayName_supportedFamilies_userInfo_(
        &self,
        identifier: NSString,
        displayName: NSString,
        supportedFamilies: NSArray,
        userInfo: NSDictionary,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithIdentifier : identifier, displayName : displayName, supportedFamilies : supportedFamilies, userInfo : userInfo)
    }
    unsafe fn initWithIdentifier_displayName_supportedFamilies_userActivity_(
        &self,
        identifier: NSString,
        displayName: NSString,
        supportedFamilies: NSArray,
        userActivity: NSUserActivity,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithIdentifier : identifier, displayName : displayName, supportedFamilies : supportedFamilies, userActivity : userActivity)
    }
    unsafe fn identifier(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, identifier)
    }
    unsafe fn displayName(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, displayName)
    }
    unsafe fn supportedFamilies(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, supportedFamilies)
    }
    unsafe fn userInfo(&self) -> NSDictionary
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, userInfo)
    }
    unsafe fn userActivity(&self) -> NSUserActivity
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, userActivity)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CLKComplicationDescriptor").unwrap(), new)
    }
}
pub trait PCLKComplicationWidgetMigrator: Sized + std::ops::Deref {
    unsafe fn getWidgetConfigurationFrom_completionHandler_(
        &self,
        complicationDescriptor: CLKComplicationDescriptor,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, getWidgetConfigurationFrom : complicationDescriptor, completionHandler : completionHandler)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CLKComplicationWidgetMigrationConfiguration(pub id);
impl std::ops::Deref for CLKComplicationWidgetMigrationConfiguration {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CLKComplicationWidgetMigrationConfiguration {}
impl CLKComplicationWidgetMigrationConfiguration {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CLKComplicationWidgetMigrationConfiguration").unwrap(), alloc) })
    }
}
impl PNSCopying for CLKComplicationWidgetMigrationConfiguration {}
impl INSObject for CLKComplicationWidgetMigrationConfiguration {}
impl PNSObject for CLKComplicationWidgetMigrationConfiguration {}
impl std::convert::TryFrom<NSObject> for CLKComplicationWidgetMigrationConfiguration {
    type Error = &'static str;
    fn try_from(
        parent: NSObject,
    ) -> Result<CLKComplicationWidgetMigrationConfiguration, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CLKComplicationWidgetMigrationConfiguration").unwrap())
        };
        if is_kind_of {
            Ok(CLKComplicationWidgetMigrationConfiguration(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to CLKComplicationWidgetMigrationConfiguration")
        }
    }
}
impl ICLKComplicationWidgetMigrationConfiguration for CLKComplicationWidgetMigrationConfiguration {}
pub trait ICLKComplicationWidgetMigrationConfiguration: Sized + std::ops::Deref {
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
        msg_send!(&*objc2::runtime::AnyClass::get(c"CLKComplicationWidgetMigrationConfiguration").unwrap(), new)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CLKComplicationIntentWidgetMigrationConfiguration(pub id);
impl std::ops::Deref for CLKComplicationIntentWidgetMigrationConfiguration {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CLKComplicationIntentWidgetMigrationConfiguration {}
impl CLKComplicationIntentWidgetMigrationConfiguration {
    pub fn alloc() -> Self {
        Self(unsafe {
            msg_send!(&*objc2::runtime::AnyClass::get(c"CLKComplicationIntentWidgetMigrationConfiguration").unwrap(), alloc)
        })
    }
}
impl ICLKComplicationWidgetMigrationConfiguration
    for CLKComplicationIntentWidgetMigrationConfiguration
{
}
impl PNSCopying for CLKComplicationIntentWidgetMigrationConfiguration {}
impl From<CLKComplicationIntentWidgetMigrationConfiguration>
    for CLKComplicationWidgetMigrationConfiguration
{
    fn from(
        child: CLKComplicationIntentWidgetMigrationConfiguration,
    ) -> CLKComplicationWidgetMigrationConfiguration {
        CLKComplicationWidgetMigrationConfiguration(child.0)
    }
}
impl std::convert::TryFrom<CLKComplicationWidgetMigrationConfiguration>
    for CLKComplicationIntentWidgetMigrationConfiguration
{
    type Error = &'static str;
    fn try_from(
        parent: CLKComplicationWidgetMigrationConfiguration,
    ) -> Result<CLKComplicationIntentWidgetMigrationConfiguration, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CLKComplicationIntentWidgetMigrationConfiguration").unwrap())
        };
        if is_kind_of {
            Ok(CLKComplicationIntentWidgetMigrationConfiguration(parent.0))
        } else {
            Err ("This CLKComplicationWidgetMigrationConfiguration cannot be downcasted to CLKComplicationIntentWidgetMigrationConfiguration" ,)
        }
    }
}
impl INSObject for CLKComplicationIntentWidgetMigrationConfiguration {}
impl PNSObject for CLKComplicationIntentWidgetMigrationConfiguration {}
impl ICLKComplicationIntentWidgetMigrationConfiguration
    for CLKComplicationIntentWidgetMigrationConfiguration
{
}
pub trait ICLKComplicationIntentWidgetMigrationConfiguration: Sized + std::ops::Deref {
    unsafe fn initWithKind_extensionBundleIdentifier_intent_localizedDisplayName_(
        &self,
        kind: NSString,
        extensionBundleIdentifier: NSString,
        intent: INIntent,
        localizedDisplayName: NSString,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithKind : kind, extensionBundleIdentifier : extensionBundleIdentifier, intent : intent, localizedDisplayName : localizedDisplayName)
    }
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn kind(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, kind)
    }
    unsafe fn extensionBundleIdentifier(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, extensionBundleIdentifier)
    }
    unsafe fn intent(&self) -> INIntent
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, intent)
    }
    unsafe fn localizedDisplayName(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, localizedDisplayName)
    }
    unsafe fn intentWidgetMigrationConfigurationWithKind_extensionBundleIdentifier_intent_localizedDisplayName_(
        kind: NSString,
        extensionBundleIdentifier: NSString,
        intent: INIntent,
        localizedDisplayName: NSString,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CLKComplicationIntentWidgetMigrationConfiguration").unwrap(), intentWidgetMigrationConfigurationWithKind : kind, extensionBundleIdentifier : extensionBundleIdentifier, intent : intent, localizedDisplayName : localizedDisplayName)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CLKComplicationIntentWidgetMigrationConfiguration").unwrap(), new)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CLKComplicationStaticWidgetMigrationConfiguration(pub id);
impl std::ops::Deref for CLKComplicationStaticWidgetMigrationConfiguration {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CLKComplicationStaticWidgetMigrationConfiguration {}
impl CLKComplicationStaticWidgetMigrationConfiguration {
    pub fn alloc() -> Self {
        Self(unsafe {
            msg_send!(&*objc2::runtime::AnyClass::get(c"CLKComplicationStaticWidgetMigrationConfiguration").unwrap(), alloc)
        })
    }
}
impl ICLKComplicationWidgetMigrationConfiguration
    for CLKComplicationStaticWidgetMigrationConfiguration
{
}
impl PNSCopying for CLKComplicationStaticWidgetMigrationConfiguration {}
impl std::convert::TryFrom<CLKComplicationWidgetMigrationConfiguration>
    for CLKComplicationStaticWidgetMigrationConfiguration
{
    type Error = &'static str;
    fn try_from(
        parent: CLKComplicationWidgetMigrationConfiguration,
    ) -> Result<CLKComplicationStaticWidgetMigrationConfiguration, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CLKComplicationStaticWidgetMigrationConfiguration").unwrap())
        };
        if is_kind_of {
            Ok(CLKComplicationStaticWidgetMigrationConfiguration(parent.0))
        } else {
            Err ("This CLKComplicationWidgetMigrationConfiguration cannot be downcasted to CLKComplicationStaticWidgetMigrationConfiguration" ,)
        }
    }
}
impl INSObject for CLKComplicationStaticWidgetMigrationConfiguration {}
impl PNSObject for CLKComplicationStaticWidgetMigrationConfiguration {}
impl ICLKComplicationStaticWidgetMigrationConfiguration
    for CLKComplicationStaticWidgetMigrationConfiguration
{
}
pub trait ICLKComplicationStaticWidgetMigrationConfiguration: Sized + std::ops::Deref {
    unsafe fn initWithKind_extensionBundleIdentifier_(
        &self,
        kind: NSString,
        extensionBundleIdentifier: NSString,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithKind : kind, extensionBundleIdentifier : extensionBundleIdentifier)
    }
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn kind(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, kind)
    }
    unsafe fn extensionBundleIdentifier(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, extensionBundleIdentifier)
    }
    unsafe fn staticWidgetMigrationConfigurationWithKind_extensionBundleIdentifier_(
        kind: NSString,
        extensionBundleIdentifier: NSString,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CLKComplicationStaticWidgetMigrationConfiguration").unwrap(), staticWidgetMigrationConfigurationWithKind : kind, extensionBundleIdentifier : extensionBundleIdentifier)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CLKComplicationStaticWidgetMigrationConfiguration").unwrap(), new)
    }
}
unsafe extern "C" {
    pub static CLKSimpleGaugeProviderFillFractionEmpty: f32;
}
unsafe extern "C" {
    pub static CLKLaunchedTimelineEntryDateKey: NSString;
}
unsafe extern "C" {
    pub static CLKLaunchedComplicationIdentifierKey: NSString;
}
unsafe extern "C" {
    pub static CLKDefaultComplicationIdentifier: NSString;
}
unsafe extern "C" {
    pub fn CLKAllComplicationFamilies() -> NSArray;
}
unsafe extern "C" {
    pub static CLKComplicationServerActiveComplicationsDidChangeNotification: NSString;
}
unsafe extern "C" {
    pub static CLKWatchFaceLibraryErrorDomain: NSString;
}

unsafe impl objc2::encode::RefEncode for CLKComplication {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CLKComplication {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CLKComplicationServer {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CLKComplicationServer {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CLKComplicationTemplate {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CLKComplicationTemplate {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CLKComplicationTemplateModularSmallSimpleText {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CLKComplicationTemplateModularSmallSimpleText {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CLKComplicationTemplateModularSmallSimpleImage {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CLKComplicationTemplateModularSmallSimpleImage {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CLKComplicationTemplateModularSmallRingText {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CLKComplicationTemplateModularSmallRingText {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CLKComplicationTemplateModularSmallRingImage {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CLKComplicationTemplateModularSmallRingImage {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CLKComplicationTemplateModularSmallStackText {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CLKComplicationTemplateModularSmallStackText {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CLKComplicationTemplateModularSmallStackImage {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CLKComplicationTemplateModularSmallStackImage {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CLKComplicationTemplateModularSmallColumnsText {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CLKComplicationTemplateModularSmallColumnsText {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CLKComplicationTemplateModularLargeStandardBody {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CLKComplicationTemplateModularLargeStandardBody {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CLKComplicationTemplateModularLargeTallBody {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CLKComplicationTemplateModularLargeTallBody {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CLKComplicationTemplateModularLargeTable {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CLKComplicationTemplateModularLargeTable {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CLKComplicationTemplateModularLargeColumns {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CLKComplicationTemplateModularLargeColumns {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CLKComplicationTemplateUtilitarianSmallSquare {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CLKComplicationTemplateUtilitarianSmallSquare {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CLKComplicationTemplateUtilitarianSmallRingText {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CLKComplicationTemplateUtilitarianSmallRingText {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CLKComplicationTemplateUtilitarianSmallRingImage {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CLKComplicationTemplateUtilitarianSmallRingImage {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CLKComplicationTemplateUtilitarianSmallFlat {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CLKComplicationTemplateUtilitarianSmallFlat {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CLKComplicationTemplateUtilitarianLargeFlat {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CLKComplicationTemplateUtilitarianLargeFlat {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CLKComplicationTemplateCircularSmallSimpleText {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CLKComplicationTemplateCircularSmallSimpleText {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CLKComplicationTemplateCircularSmallSimpleImage {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CLKComplicationTemplateCircularSmallSimpleImage {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CLKComplicationTemplateCircularSmallRingText {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CLKComplicationTemplateCircularSmallRingText {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CLKComplicationTemplateCircularSmallRingImage {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CLKComplicationTemplateCircularSmallRingImage {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CLKComplicationTemplateCircularSmallStackText {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CLKComplicationTemplateCircularSmallStackText {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CLKComplicationTemplateCircularSmallStackImage {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CLKComplicationTemplateCircularSmallStackImage {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CLKComplicationTemplateExtraLargeSimpleText {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CLKComplicationTemplateExtraLargeSimpleText {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CLKComplicationTemplateExtraLargeSimpleImage {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CLKComplicationTemplateExtraLargeSimpleImage {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CLKComplicationTemplateExtraLargeRingText {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CLKComplicationTemplateExtraLargeRingText {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CLKComplicationTemplateExtraLargeRingImage {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CLKComplicationTemplateExtraLargeRingImage {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CLKComplicationTemplateExtraLargeStackText {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CLKComplicationTemplateExtraLargeStackText {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CLKComplicationTemplateExtraLargeStackImage {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CLKComplicationTemplateExtraLargeStackImage {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CLKComplicationTemplateExtraLargeColumnsText {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CLKComplicationTemplateExtraLargeColumnsText {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CLKComplicationTemplateGraphicCornerGaugeText {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CLKComplicationTemplateGraphicCornerGaugeText {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CLKComplicationTemplateGraphicCornerGaugeImage {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CLKComplicationTemplateGraphicCornerGaugeImage {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CLKComplicationTemplateGraphicCornerTextImage {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CLKComplicationTemplateGraphicCornerTextImage {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CLKComplicationTemplateGraphicCornerStackText {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CLKComplicationTemplateGraphicCornerStackText {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CLKComplicationTemplateGraphicCornerCircularImage {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CLKComplicationTemplateGraphicCornerCircularImage {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CLKComplicationTemplateGraphicCircular {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CLKComplicationTemplateGraphicCircular {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CLKComplicationTemplateGraphicCircularImage {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CLKComplicationTemplateGraphicCircularImage {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CLKComplicationTemplateGraphicCircularOpenGaugeRangeText {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CLKComplicationTemplateGraphicCircularOpenGaugeRangeText {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CLKComplicationTemplateGraphicCircularOpenGaugeSimpleText {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CLKComplicationTemplateGraphicCircularOpenGaugeSimpleText {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CLKComplicationTemplateGraphicCircularOpenGaugeImage {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CLKComplicationTemplateGraphicCircularOpenGaugeImage {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CLKComplicationTemplateGraphicCircularClosedGaugeText {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CLKComplicationTemplateGraphicCircularClosedGaugeText {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CLKComplicationTemplateGraphicCircularClosedGaugeImage {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CLKComplicationTemplateGraphicCircularClosedGaugeImage {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CLKComplicationTemplateGraphicCircularStackText {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CLKComplicationTemplateGraphicCircularStackText {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CLKComplicationTemplateGraphicCircularStackImage {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CLKComplicationTemplateGraphicCircularStackImage {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CLKComplicationTemplateGraphicBezelCircularText {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CLKComplicationTemplateGraphicBezelCircularText {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CLKComplicationTemplateGraphicRectangularFullImage {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CLKComplicationTemplateGraphicRectangularFullImage {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CLKComplicationTemplateGraphicRectangularLargeImage {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CLKComplicationTemplateGraphicRectangularLargeImage {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CLKComplicationTemplateGraphicRectangularStandardBody {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CLKComplicationTemplateGraphicRectangularStandardBody {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CLKComplicationTemplateGraphicRectangularTextGauge {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CLKComplicationTemplateGraphicRectangularTextGauge {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CLKComplicationTemplateGraphicExtraLargeCircular {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CLKComplicationTemplateGraphicExtraLargeCircular {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CLKComplicationTemplateGraphicExtraLargeCircularImage {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CLKComplicationTemplateGraphicExtraLargeCircularImage {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CLKComplicationTemplateGraphicExtraLargeCircularOpenGaugeRangeText {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CLKComplicationTemplateGraphicExtraLargeCircularOpenGaugeRangeText {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CLKComplicationTemplateGraphicExtraLargeCircularOpenGaugeSimpleText {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CLKComplicationTemplateGraphicExtraLargeCircularOpenGaugeSimpleText {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CLKComplicationTemplateGraphicExtraLargeCircularOpenGaugeImage {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CLKComplicationTemplateGraphicExtraLargeCircularOpenGaugeImage {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CLKComplicationTemplateGraphicExtraLargeCircularClosedGaugeText {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CLKComplicationTemplateGraphicExtraLargeCircularClosedGaugeText {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CLKComplicationTemplateGraphicExtraLargeCircularClosedGaugeImage {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CLKComplicationTemplateGraphicExtraLargeCircularClosedGaugeImage {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CLKComplicationTemplateGraphicExtraLargeCircularStackText {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CLKComplicationTemplateGraphicExtraLargeCircularStackText {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CLKComplicationTemplateGraphicExtraLargeCircularStackImage {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CLKComplicationTemplateGraphicExtraLargeCircularStackImage {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CLKComplicationTimelineEntry {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CLKComplicationTimelineEntry {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CLKImageProvider {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CLKImageProvider {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CLKTextProvider {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CLKTextProvider {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CLKSimpleTextProvider {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CLKSimpleTextProvider {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CLKDateTextProvider {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CLKDateTextProvider {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CLKTimeTextProvider {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CLKTimeTextProvider {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CLKTimeIntervalTextProvider {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CLKTimeIntervalTextProvider {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CLKRelativeDateTextProvider {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CLKRelativeDateTextProvider {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CLKGaugeProvider {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CLKGaugeProvider {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CLKSimpleGaugeProvider {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CLKSimpleGaugeProvider {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CLKTimeIntervalGaugeProvider {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CLKTimeIntervalGaugeProvider {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CLKFullColorImageProvider {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CLKFullColorImageProvider {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CLKWatchFaceLibrary {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CLKWatchFaceLibrary {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CLKComplicationDescriptor {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CLKComplicationDescriptor {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CLKComplicationWidgetMigrationConfiguration {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CLKComplicationWidgetMigrationConfiguration {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CLKComplicationIntentWidgetMigrationConfiguration {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CLKComplicationIntentWidgetMigrationConfiguration {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CLKComplicationStaticWidgetMigrationConfiguration {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CLKComplicationStaticWidgetMigrationConfiguration {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
