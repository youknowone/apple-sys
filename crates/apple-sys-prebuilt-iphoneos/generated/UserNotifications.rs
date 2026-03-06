#[allow(unused_imports)]
use crate::objc::*;
#[allow(unused_imports)]
use crate::CoreLocation::*;
#[allow(unused_imports)]
use crate::Foundation::*;
#[allow(unused_imports)]
use crate::Intents::*;
#[allow(unused_imports)]
use crate::Security::*;

#[allow(unused_imports)]
use objc2::msg_send;
pub trait NSString_UNUserNotificationCenterSupport: Sized + std::ops::Deref {
    unsafe fn localizedUserNotificationStringForKey_arguments_(
        key: NSString,
        arguments: NSArray,
    ) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NSString").unwrap(), localizedUserNotificationStringForKey : key, arguments : arguments)
    }
}
pub type UNErrorCode = NSInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct UNNotification(pub id);
impl std::ops::Deref for UNNotification {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for UNNotification {}
impl UNNotification {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"UNNotification").unwrap(), alloc) })
    }
}
impl PNSCopying for UNNotification {}
impl PNSSecureCoding for UNNotification {}
impl INSObject for UNNotification {}
impl PNSObject for UNNotification {}
impl std::convert::TryFrom<NSObject> for UNNotification {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<UNNotification, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"UNNotification").unwrap()) };
        if is_kind_of {
            Ok(UNNotification(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to UNNotification")
        }
    }
}
impl IUNNotification for UNNotification {}
pub trait IUNNotification: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn date(&self) -> NSDate
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, date)
    }
    unsafe fn request(&self) -> UNNotificationRequest
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, request)
    }
}
pub type UNNotificationActionOptions = NSUInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct UNNotificationAction(pub id);
impl std::ops::Deref for UNNotificationAction {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for UNNotificationAction {}
impl UNNotificationAction {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"UNNotificationAction").unwrap(), alloc) })
    }
}
impl PNSCopying for UNNotificationAction {}
impl PNSSecureCoding for UNNotificationAction {}
impl INSObject for UNNotificationAction {}
impl PNSObject for UNNotificationAction {}
impl std::convert::TryFrom<NSObject> for UNNotificationAction {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<UNNotificationAction, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"UNNotificationAction").unwrap()) };
        if is_kind_of {
            Ok(UNNotificationAction(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to UNNotificationAction")
        }
    }
}
impl IUNNotificationAction for UNNotificationAction {}
pub trait IUNNotificationAction: Sized + std::ops::Deref {
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
    unsafe fn title(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, title)
    }
    unsafe fn options(&self) -> UNNotificationActionOptions
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, options)
    }
    unsafe fn icon(&self) -> UNNotificationActionIcon
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, icon)
    }
    unsafe fn actionWithIdentifier_title_options_(
        identifier: NSString,
        title: NSString,
        options: UNNotificationActionOptions,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"UNNotificationAction").unwrap(), actionWithIdentifier : identifier, title : title, options : options)
    }
    unsafe fn actionWithIdentifier_title_options_icon_(
        identifier: NSString,
        title: NSString,
        options: UNNotificationActionOptions,
        icon: UNNotificationActionIcon,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"UNNotificationAction").unwrap(), actionWithIdentifier : identifier, title : title, options : options, icon : icon)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct UNTextInputNotificationAction(pub id);
impl std::ops::Deref for UNTextInputNotificationAction {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for UNTextInputNotificationAction {}
impl UNTextInputNotificationAction {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"UNTextInputNotificationAction").unwrap(), alloc) })
    }
}
impl IUNNotificationAction for UNTextInputNotificationAction {}
impl PNSCopying for UNTextInputNotificationAction {}
impl PNSSecureCoding for UNTextInputNotificationAction {}
impl From<UNTextInputNotificationAction> for UNNotificationAction {
    fn from(child: UNTextInputNotificationAction) -> UNNotificationAction {
        UNNotificationAction(child.0)
    }
}
impl std::convert::TryFrom<UNNotificationAction> for UNTextInputNotificationAction {
    type Error = &'static str;
    fn try_from(
        parent: UNNotificationAction,
    ) -> Result<UNTextInputNotificationAction, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"UNTextInputNotificationAction").unwrap())
        };
        if is_kind_of {
            Ok(UNTextInputNotificationAction(parent.0))
        } else {
            Err("This UNNotificationAction cannot be downcasted to UNTextInputNotificationAction")
        }
    }
}
impl INSObject for UNTextInputNotificationAction {}
impl PNSObject for UNTextInputNotificationAction {}
impl IUNTextInputNotificationAction for UNTextInputNotificationAction {}
pub trait IUNTextInputNotificationAction: Sized + std::ops::Deref {
    unsafe fn textInputButtonTitle(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, textInputButtonTitle)
    }
    unsafe fn textInputPlaceholder(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, textInputPlaceholder)
    }
    unsafe fn actionWithIdentifier_title_options_textInputButtonTitle_textInputPlaceholder_(
        identifier: NSString,
        title: NSString,
        options: UNNotificationActionOptions,
        textInputButtonTitle: NSString,
        textInputPlaceholder: NSString,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"UNTextInputNotificationAction").unwrap(), actionWithIdentifier : identifier, title : title, options : options, textInputButtonTitle : textInputButtonTitle, textInputPlaceholder : textInputPlaceholder)
    }
    unsafe fn actionWithIdentifier_title_options_icon_textInputButtonTitle_textInputPlaceholder_(
        identifier: NSString,
        title: NSString,
        options: UNNotificationActionOptions,
        icon: UNNotificationActionIcon,
        textInputButtonTitle: NSString,
        textInputPlaceholder: NSString,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"UNTextInputNotificationAction").unwrap(), actionWithIdentifier : identifier, title : title, options : options, icon : icon, textInputButtonTitle : textInputButtonTitle, textInputPlaceholder : textInputPlaceholder)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct UNNotificationActionIcon(pub id);
impl std::ops::Deref for UNNotificationActionIcon {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for UNNotificationActionIcon {}
impl UNNotificationActionIcon {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"UNNotificationActionIcon").unwrap(), alloc) })
    }
}
impl PNSCopying for UNNotificationActionIcon {}
impl PNSSecureCoding for UNNotificationActionIcon {}
impl INSObject for UNNotificationActionIcon {}
impl PNSObject for UNNotificationActionIcon {}
impl std::convert::TryFrom<NSObject> for UNNotificationActionIcon {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<UNNotificationActionIcon, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"UNNotificationActionIcon").unwrap()) };
        if is_kind_of {
            Ok(UNNotificationActionIcon(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to UNNotificationActionIcon")
        }
    }
}
impl IUNNotificationActionIcon for UNNotificationActionIcon {}
pub trait IUNNotificationActionIcon: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn iconWithTemplateImageName_(templateImageName: NSString) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"UNNotificationActionIcon").unwrap(), iconWithTemplateImageName : templateImageName)
    }
    unsafe fn iconWithSystemImageName_(systemImageName: NSString) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"UNNotificationActionIcon").unwrap(), iconWithSystemImageName : systemImageName)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct UNNotificationAttachment(pub id);
impl std::ops::Deref for UNNotificationAttachment {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for UNNotificationAttachment {}
impl UNNotificationAttachment {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"UNNotificationAttachment").unwrap(), alloc) })
    }
}
impl PNSCopying for UNNotificationAttachment {}
impl PNSSecureCoding for UNNotificationAttachment {}
impl INSObject for UNNotificationAttachment {}
impl PNSObject for UNNotificationAttachment {}
impl std::convert::TryFrom<NSObject> for UNNotificationAttachment {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<UNNotificationAttachment, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"UNNotificationAttachment").unwrap()) };
        if is_kind_of {
            Ok(UNNotificationAttachment(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to UNNotificationAttachment")
        }
    }
}
impl IUNNotificationAttachment for UNNotificationAttachment {}
pub trait IUNNotificationAttachment: Sized + std::ops::Deref {
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
    unsafe fn URL(&self) -> NSURL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, URL)
    }
    unsafe fn type_(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, type)
    }
    unsafe fn attachmentWithIdentifier_URL_options_error_(
        identifier: NSString,
        URL: NSURL,
        options: NSDictionary,
        error: *mut NSError,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"UNNotificationAttachment").unwrap(), attachmentWithIdentifier : identifier, URL : URL, options : options, error : error)
    }
}
pub trait PUNNotificationContentProviding: Sized + std::ops::Deref {}
pub type UNNotificationInterruptionLevel = NSUInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct UNNotificationContent(pub id);
impl std::ops::Deref for UNNotificationContent {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for UNNotificationContent {}
impl UNNotificationContent {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"UNNotificationContent").unwrap(), alloc) })
    }
}
impl PNSCopying for UNNotificationContent {}
impl PNSMutableCopying for UNNotificationContent {}
impl PNSSecureCoding for UNNotificationContent {}
impl INSObject for UNNotificationContent {}
impl PNSObject for UNNotificationContent {}
impl std::convert::TryFrom<NSObject> for UNNotificationContent {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<UNNotificationContent, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"UNNotificationContent").unwrap()) };
        if is_kind_of {
            Ok(UNNotificationContent(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to UNNotificationContent")
        }
    }
}
impl IUNNotificationContent for UNNotificationContent {}
pub trait IUNNotificationContent: Sized + std::ops::Deref {
    unsafe fn contentByUpdatingWithProvider_error_(
        &self,
        provider: *mut u64,
        outError: *mut NSError,
    ) -> UNNotificationContent
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, contentByUpdatingWithProvider : provider, error : outError)
    }
    unsafe fn attachments(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, attachments)
    }
    unsafe fn badge(&self) -> NSNumber
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, badge)
    }
    unsafe fn body(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, body)
    }
    unsafe fn categoryIdentifier(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, categoryIdentifier)
    }
    unsafe fn launchImageName(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, launchImageName)
    }
    unsafe fn sound(&self) -> UNNotificationSound
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, sound)
    }
    unsafe fn subtitle(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, subtitle)
    }
    unsafe fn threadIdentifier(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, threadIdentifier)
    }
    unsafe fn title(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, title)
    }
    unsafe fn userInfo(&self) -> NSDictionary
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, userInfo)
    }
    unsafe fn summaryArgument(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, summaryArgument)
    }
    unsafe fn summaryArgumentCount(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, summaryArgumentCount)
    }
    unsafe fn targetContentIdentifier(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, targetContentIdentifier)
    }
    unsafe fn interruptionLevel(&self) -> UNNotificationInterruptionLevel
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, interruptionLevel)
    }
    unsafe fn relevanceScore(&self) -> f64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, relevanceScore)
    }
    unsafe fn filterCriteria(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, filterCriteria)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct UNMutableNotificationContent(pub id);
impl std::ops::Deref for UNMutableNotificationContent {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for UNMutableNotificationContent {}
impl UNMutableNotificationContent {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"UNMutableNotificationContent").unwrap(), alloc) })
    }
}
impl IUNNotificationContent for UNMutableNotificationContent {}
impl PNSCopying for UNMutableNotificationContent {}
impl PNSMutableCopying for UNMutableNotificationContent {}
impl PNSSecureCoding for UNMutableNotificationContent {}
impl From<UNMutableNotificationContent> for UNNotificationContent {
    fn from(child: UNMutableNotificationContent) -> UNNotificationContent {
        UNNotificationContent(child.0)
    }
}
impl std::convert::TryFrom<UNNotificationContent> for UNMutableNotificationContent {
    type Error = &'static str;
    fn try_from(
        parent: UNNotificationContent,
    ) -> Result<UNMutableNotificationContent, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"UNMutableNotificationContent").unwrap()) };
        if is_kind_of {
            Ok(UNMutableNotificationContent(parent.0))
        } else {
            Err("This UNNotificationContent cannot be downcasted to UNMutableNotificationContent")
        }
    }
}
impl INSObject for UNMutableNotificationContent {}
impl PNSObject for UNMutableNotificationContent {}
impl IUNMutableNotificationContent for UNMutableNotificationContent {}
pub trait IUNMutableNotificationContent: Sized + std::ops::Deref {
    unsafe fn attachments(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, attachments)
    }
    unsafe fn setAttachments_(&self, attachments: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAttachments : attachments)
    }
    unsafe fn badge(&self) -> NSNumber
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, badge)
    }
    unsafe fn setBadge_(&self, badge: NSNumber)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setBadge : badge)
    }
    unsafe fn body(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, body)
    }
    unsafe fn setBody_(&self, body: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setBody : body)
    }
    unsafe fn categoryIdentifier(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, categoryIdentifier)
    }
    unsafe fn setCategoryIdentifier_(&self, categoryIdentifier: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCategoryIdentifier : categoryIdentifier)
    }
    unsafe fn launchImageName(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, launchImageName)
    }
    unsafe fn setLaunchImageName_(&self, launchImageName: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setLaunchImageName : launchImageName)
    }
    unsafe fn sound(&self) -> UNNotificationSound
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, sound)
    }
    unsafe fn setSound_(&self, sound: UNNotificationSound)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSound : sound)
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
    unsafe fn threadIdentifier(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, threadIdentifier)
    }
    unsafe fn setThreadIdentifier_(&self, threadIdentifier: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setThreadIdentifier : threadIdentifier)
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
    unsafe fn summaryArgument(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, summaryArgument)
    }
    unsafe fn setSummaryArgument_(&self, summaryArgument: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSummaryArgument : summaryArgument)
    }
    unsafe fn summaryArgumentCount(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, summaryArgumentCount)
    }
    unsafe fn setSummaryArgumentCount_(&self, summaryArgumentCount: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSummaryArgumentCount : summaryArgumentCount)
    }
    unsafe fn targetContentIdentifier(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, targetContentIdentifier)
    }
    unsafe fn setTargetContentIdentifier_(&self, targetContentIdentifier: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setTargetContentIdentifier : targetContentIdentifier)
    }
    unsafe fn interruptionLevel(&self) -> UNNotificationInterruptionLevel
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, interruptionLevel)
    }
    unsafe fn setInterruptionLevel_(&self, interruptionLevel: UNNotificationInterruptionLevel)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInterruptionLevel : interruptionLevel)
    }
    unsafe fn relevanceScore(&self) -> f64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, relevanceScore)
    }
    unsafe fn setRelevanceScore_(&self, relevanceScore: f64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRelevanceScore : relevanceScore)
    }
    unsafe fn filterCriteria(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, filterCriteria)
    }
    unsafe fn setFilterCriteria_(&self, filterCriteria: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setFilterCriteria : filterCriteria)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct UNNotificationAttributedMessageContext(pub id);
impl std::ops::Deref for UNNotificationAttributedMessageContext {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for UNNotificationAttributedMessageContext {}
impl UNNotificationAttributedMessageContext {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"UNNotificationAttributedMessageContext").unwrap(), alloc) })
    }
}
impl PUNNotificationContentProviding for UNNotificationAttributedMessageContext {}
impl INSObject for UNNotificationAttributedMessageContext {}
impl PNSObject for UNNotificationAttributedMessageContext {}
impl std::convert::TryFrom<NSObject> for UNNotificationAttributedMessageContext {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<UNNotificationAttributedMessageContext, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"UNNotificationAttributedMessageContext").unwrap())
        };
        if is_kind_of {
            Ok(UNNotificationAttributedMessageContext(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to UNNotificationAttributedMessageContext")
        }
    }
}
impl IUNNotificationAttributedMessageContext for UNNotificationAttributedMessageContext {}
pub trait IUNNotificationAttributedMessageContext: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn contextWithSendMessageIntent_attributedContent_(
        sendMessageIntent: INSendMessageIntent,
        attributedContent: NSAttributedString,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"UNNotificationAttributedMessageContext").unwrap(), contextWithSendMessageIntent : sendMessageIntent, attributedContent : attributedContent)
    }
}
pub type UNNotificationCategoryOptions = NSUInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct UNNotificationCategory(pub id);
impl std::ops::Deref for UNNotificationCategory {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for UNNotificationCategory {}
impl UNNotificationCategory {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"UNNotificationCategory").unwrap(), alloc) })
    }
}
impl PNSCopying for UNNotificationCategory {}
impl PNSSecureCoding for UNNotificationCategory {}
impl INSObject for UNNotificationCategory {}
impl PNSObject for UNNotificationCategory {}
impl std::convert::TryFrom<NSObject> for UNNotificationCategory {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<UNNotificationCategory, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"UNNotificationCategory").unwrap()) };
        if is_kind_of {
            Ok(UNNotificationCategory(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to UNNotificationCategory")
        }
    }
}
impl IUNNotificationCategory for UNNotificationCategory {}
pub trait IUNNotificationCategory: Sized + std::ops::Deref {
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
    unsafe fn actions(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, actions)
    }
    unsafe fn intentIdentifiers(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, intentIdentifiers)
    }
    unsafe fn options(&self) -> UNNotificationCategoryOptions
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, options)
    }
    unsafe fn hiddenPreviewsBodyPlaceholder(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, hiddenPreviewsBodyPlaceholder)
    }
    unsafe fn categorySummaryFormat(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, categorySummaryFormat)
    }
    unsafe fn categoryWithIdentifier_actions_intentIdentifiers_options_(
        identifier: NSString,
        actions: NSArray,
        intentIdentifiers: NSArray,
        options: UNNotificationCategoryOptions,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"UNNotificationCategory").unwrap(), categoryWithIdentifier : identifier, actions : actions, intentIdentifiers : intentIdentifiers, options : options)
    }
    unsafe fn categoryWithIdentifier_actions_intentIdentifiers_hiddenPreviewsBodyPlaceholder_options_(
        identifier: NSString,
        actions: NSArray,
        intentIdentifiers: NSArray,
        hiddenPreviewsBodyPlaceholder: NSString,
        options: UNNotificationCategoryOptions,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"UNNotificationCategory").unwrap(), categoryWithIdentifier : identifier, actions : actions, intentIdentifiers : intentIdentifiers, hiddenPreviewsBodyPlaceholder : hiddenPreviewsBodyPlaceholder, options : options)
    }
    unsafe fn categoryWithIdentifier_actions_intentIdentifiers_hiddenPreviewsBodyPlaceholder_categorySummaryFormat_options_(
        identifier: NSString,
        actions: NSArray,
        intentIdentifiers: NSArray,
        hiddenPreviewsBodyPlaceholder: NSString,
        categorySummaryFormat: NSString,
        options: UNNotificationCategoryOptions,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"UNNotificationCategory").unwrap(), categoryWithIdentifier : identifier, actions : actions, intentIdentifiers : intentIdentifiers, hiddenPreviewsBodyPlaceholder : hiddenPreviewsBodyPlaceholder, categorySummaryFormat : categorySummaryFormat, options : options)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct UNNotificationRequest(pub id);
impl std::ops::Deref for UNNotificationRequest {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for UNNotificationRequest {}
impl UNNotificationRequest {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"UNNotificationRequest").unwrap(), alloc) })
    }
}
impl PNSCopying for UNNotificationRequest {}
impl PNSSecureCoding for UNNotificationRequest {}
impl INSObject for UNNotificationRequest {}
impl PNSObject for UNNotificationRequest {}
impl std::convert::TryFrom<NSObject> for UNNotificationRequest {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<UNNotificationRequest, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"UNNotificationRequest").unwrap()) };
        if is_kind_of {
            Ok(UNNotificationRequest(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to UNNotificationRequest")
        }
    }
}
impl IUNNotificationRequest for UNNotificationRequest {}
pub trait IUNNotificationRequest: Sized + std::ops::Deref {
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
    unsafe fn content(&self) -> UNNotificationContent
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, content)
    }
    unsafe fn trigger(&self) -> UNNotificationTrigger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, trigger)
    }
    unsafe fn requestWithIdentifier_content_trigger_(
        identifier: NSString,
        content: UNNotificationContent,
        trigger: UNNotificationTrigger,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"UNNotificationRequest").unwrap(), requestWithIdentifier : identifier, content : content, trigger : trigger)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct UNNotificationResponse(pub id);
impl std::ops::Deref for UNNotificationResponse {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for UNNotificationResponse {}
impl UNNotificationResponse {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"UNNotificationResponse").unwrap(), alloc) })
    }
}
impl PNSCopying for UNNotificationResponse {}
impl PNSSecureCoding for UNNotificationResponse {}
impl INSObject for UNNotificationResponse {}
impl PNSObject for UNNotificationResponse {}
impl std::convert::TryFrom<NSObject> for UNNotificationResponse {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<UNNotificationResponse, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"UNNotificationResponse").unwrap()) };
        if is_kind_of {
            Ok(UNNotificationResponse(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to UNNotificationResponse")
        }
    }
}
impl IUNNotificationResponse for UNNotificationResponse {}
pub trait IUNNotificationResponse: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn notification(&self) -> UNNotification
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, notification)
    }
    unsafe fn actionIdentifier(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, actionIdentifier)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct UNTextInputNotificationResponse(pub id);
impl std::ops::Deref for UNTextInputNotificationResponse {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for UNTextInputNotificationResponse {}
impl UNTextInputNotificationResponse {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"UNTextInputNotificationResponse").unwrap(), alloc) })
    }
}
impl IUNNotificationResponse for UNTextInputNotificationResponse {}
impl PNSCopying for UNTextInputNotificationResponse {}
impl PNSSecureCoding for UNTextInputNotificationResponse {}
impl From<UNTextInputNotificationResponse> for UNNotificationResponse {
    fn from(child: UNTextInputNotificationResponse) -> UNNotificationResponse {
        UNNotificationResponse(child.0)
    }
}
impl std::convert::TryFrom<UNNotificationResponse> for UNTextInputNotificationResponse {
    type Error = &'static str;
    fn try_from(
        parent: UNNotificationResponse,
    ) -> Result<UNTextInputNotificationResponse, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"UNTextInputNotificationResponse").unwrap())
        };
        if is_kind_of {
            Ok(UNTextInputNotificationResponse(parent.0))
        } else {
            Err ("This UNNotificationResponse cannot be downcasted to UNTextInputNotificationResponse" ,)
        }
    }
}
impl INSObject for UNTextInputNotificationResponse {}
impl PNSObject for UNTextInputNotificationResponse {}
impl IUNTextInputNotificationResponse for UNTextInputNotificationResponse {}
pub trait IUNTextInputNotificationResponse: Sized + std::ops::Deref {
    unsafe fn userText(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, userText)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct UNNotificationServiceExtension(pub id);
impl std::ops::Deref for UNNotificationServiceExtension {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for UNNotificationServiceExtension {}
impl UNNotificationServiceExtension {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"UNNotificationServiceExtension").unwrap(), alloc) })
    }
}
impl INSObject for UNNotificationServiceExtension {}
impl PNSObject for UNNotificationServiceExtension {}
impl std::convert::TryFrom<NSObject> for UNNotificationServiceExtension {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<UNNotificationServiceExtension, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"UNNotificationServiceExtension").unwrap())
        };
        if is_kind_of {
            Ok(UNNotificationServiceExtension(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to UNNotificationServiceExtension")
        }
    }
}
impl IUNNotificationServiceExtension for UNNotificationServiceExtension {}
pub trait IUNNotificationServiceExtension: Sized + std::ops::Deref {
    unsafe fn didReceiveNotificationRequest_withContentHandler_(
        &self,
        request: UNNotificationRequest,
        contentHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, didReceiveNotificationRequest : request, withContentHandler : contentHandler)
    }
    unsafe fn serviceExtensionTimeWillExpire(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, serviceExtensionTimeWillExpire)
    }
}
pub type UNAuthorizationStatus = NSInteger;
pub type UNShowPreviewsSetting = NSInteger;
pub type UNNotificationSetting = NSInteger;
pub type UNAlertStyle = NSInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct UNNotificationSettings(pub id);
impl std::ops::Deref for UNNotificationSettings {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for UNNotificationSettings {}
impl UNNotificationSettings {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"UNNotificationSettings").unwrap(), alloc) })
    }
}
impl PNSCopying for UNNotificationSettings {}
impl PNSSecureCoding for UNNotificationSettings {}
impl INSObject for UNNotificationSettings {}
impl PNSObject for UNNotificationSettings {}
impl std::convert::TryFrom<NSObject> for UNNotificationSettings {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<UNNotificationSettings, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"UNNotificationSettings").unwrap()) };
        if is_kind_of {
            Ok(UNNotificationSettings(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to UNNotificationSettings")
        }
    }
}
impl IUNNotificationSettings for UNNotificationSettings {}
pub trait IUNNotificationSettings: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn authorizationStatus(&self) -> UNAuthorizationStatus
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, authorizationStatus)
    }
    unsafe fn soundSetting(&self) -> UNNotificationSetting
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, soundSetting)
    }
    unsafe fn badgeSetting(&self) -> UNNotificationSetting
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, badgeSetting)
    }
    unsafe fn alertSetting(&self) -> UNNotificationSetting
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, alertSetting)
    }
    unsafe fn notificationCenterSetting(&self) -> UNNotificationSetting
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, notificationCenterSetting)
    }
    unsafe fn lockScreenSetting(&self) -> UNNotificationSetting
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, lockScreenSetting)
    }
    unsafe fn carPlaySetting(&self) -> UNNotificationSetting
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, carPlaySetting)
    }
    unsafe fn alertStyle(&self) -> UNAlertStyle
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, alertStyle)
    }
    unsafe fn showPreviewsSetting(&self) -> UNShowPreviewsSetting
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, showPreviewsSetting)
    }
    unsafe fn criticalAlertSetting(&self) -> UNNotificationSetting
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, criticalAlertSetting)
    }
    unsafe fn providesAppNotificationSettings(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, providesAppNotificationSettings)
    }
    unsafe fn announcementSetting(&self) -> UNNotificationSetting
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, announcementSetting)
    }
    unsafe fn timeSensitiveSetting(&self) -> UNNotificationSetting
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, timeSensitiveSetting)
    }
    unsafe fn scheduledDeliverySetting(&self) -> UNNotificationSetting
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, scheduledDeliverySetting)
    }
    unsafe fn directMessagesSetting(&self) -> UNNotificationSetting
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, directMessagesSetting)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct UNNotificationSound(pub id);
impl std::ops::Deref for UNNotificationSound {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for UNNotificationSound {}
impl UNNotificationSound {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"UNNotificationSound").unwrap(), alloc) })
    }
}
impl PNSCopying for UNNotificationSound {}
impl PNSSecureCoding for UNNotificationSound {}
impl INSObject for UNNotificationSound {}
impl PNSObject for UNNotificationSound {}
impl std::convert::TryFrom<NSObject> for UNNotificationSound {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<UNNotificationSound, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"UNNotificationSound").unwrap()) };
        if is_kind_of {
            Ok(UNNotificationSound(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to UNNotificationSound")
        }
    }
}
impl IUNNotificationSound for UNNotificationSound {}
pub trait IUNNotificationSound: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn defaultCriticalSoundWithAudioVolume_(volume: f32) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"UNNotificationSound").unwrap(), defaultCriticalSoundWithAudioVolume : volume)
    }
    unsafe fn soundNamed_(name: NSString) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"UNNotificationSound").unwrap(), soundNamed : name)
    }
    unsafe fn ringtoneSoundNamed_(name: NSString) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"UNNotificationSound").unwrap(), ringtoneSoundNamed : name)
    }
    unsafe fn criticalSoundNamed_(name: NSString) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"UNNotificationSound").unwrap(), criticalSoundNamed : name)
    }
    unsafe fn criticalSoundNamed_withAudioVolume_(name: NSString, volume: f32) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"UNNotificationSound").unwrap(), criticalSoundNamed : name, withAudioVolume : volume)
    }
    unsafe fn defaultSound() -> UNNotificationSound
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"UNNotificationSound").unwrap(), defaultSound)
    }
    unsafe fn defaultRingtoneSound() -> UNNotificationSound
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"UNNotificationSound").unwrap(), defaultRingtoneSound)
    }
    unsafe fn defaultCriticalSound() -> UNNotificationSound
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"UNNotificationSound").unwrap(), defaultCriticalSound)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct UNNotificationTrigger(pub id);
impl std::ops::Deref for UNNotificationTrigger {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for UNNotificationTrigger {}
impl UNNotificationTrigger {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"UNNotificationTrigger").unwrap(), alloc) })
    }
}
impl PNSCopying for UNNotificationTrigger {}
impl PNSSecureCoding for UNNotificationTrigger {}
impl INSObject for UNNotificationTrigger {}
impl PNSObject for UNNotificationTrigger {}
impl std::convert::TryFrom<NSObject> for UNNotificationTrigger {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<UNNotificationTrigger, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"UNNotificationTrigger").unwrap()) };
        if is_kind_of {
            Ok(UNNotificationTrigger(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to UNNotificationTrigger")
        }
    }
}
impl IUNNotificationTrigger for UNNotificationTrigger {}
pub trait IUNNotificationTrigger: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn repeats(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, repeats)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct UNPushNotificationTrigger(pub id);
impl std::ops::Deref for UNPushNotificationTrigger {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for UNPushNotificationTrigger {}
impl UNPushNotificationTrigger {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"UNPushNotificationTrigger").unwrap(), alloc) })
    }
}
impl IUNNotificationTrigger for UNPushNotificationTrigger {}
impl PNSCopying for UNPushNotificationTrigger {}
impl PNSSecureCoding for UNPushNotificationTrigger {}
impl From<UNPushNotificationTrigger> for UNNotificationTrigger {
    fn from(child: UNPushNotificationTrigger) -> UNNotificationTrigger {
        UNNotificationTrigger(child.0)
    }
}
impl std::convert::TryFrom<UNNotificationTrigger> for UNPushNotificationTrigger {
    type Error = &'static str;
    fn try_from(parent: UNNotificationTrigger) -> Result<UNPushNotificationTrigger, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"UNPushNotificationTrigger").unwrap()) };
        if is_kind_of {
            Ok(UNPushNotificationTrigger(parent.0))
        } else {
            Err("This UNNotificationTrigger cannot be downcasted to UNPushNotificationTrigger")
        }
    }
}
impl INSObject for UNPushNotificationTrigger {}
impl PNSObject for UNPushNotificationTrigger {}
impl IUNPushNotificationTrigger for UNPushNotificationTrigger {}
pub trait IUNPushNotificationTrigger: Sized + std::ops::Deref {}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct UNTimeIntervalNotificationTrigger(pub id);
impl std::ops::Deref for UNTimeIntervalNotificationTrigger {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for UNTimeIntervalNotificationTrigger {}
impl UNTimeIntervalNotificationTrigger {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"UNTimeIntervalNotificationTrigger").unwrap(), alloc) })
    }
}
impl IUNNotificationTrigger for UNTimeIntervalNotificationTrigger {}
impl PNSCopying for UNTimeIntervalNotificationTrigger {}
impl PNSSecureCoding for UNTimeIntervalNotificationTrigger {}
impl std::convert::TryFrom<UNNotificationTrigger> for UNTimeIntervalNotificationTrigger {
    type Error = &'static str;
    fn try_from(
        parent: UNNotificationTrigger,
    ) -> Result<UNTimeIntervalNotificationTrigger, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"UNTimeIntervalNotificationTrigger").unwrap())
        };
        if is_kind_of {
            Ok(UNTimeIntervalNotificationTrigger(parent.0))
        } else {
            Err ("This UNNotificationTrigger cannot be downcasted to UNTimeIntervalNotificationTrigger" ,)
        }
    }
}
impl INSObject for UNTimeIntervalNotificationTrigger {}
impl PNSObject for UNTimeIntervalNotificationTrigger {}
impl IUNTimeIntervalNotificationTrigger for UNTimeIntervalNotificationTrigger {}
pub trait IUNTimeIntervalNotificationTrigger: Sized + std::ops::Deref {
    unsafe fn nextTriggerDate(&self) -> NSDate
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, nextTriggerDate)
    }
    unsafe fn timeInterval(&self) -> NSTimeInterval
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, timeInterval)
    }
    unsafe fn triggerWithTimeInterval_repeats_(
        timeInterval: NSTimeInterval,
        repeats: BOOL,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"UNTimeIntervalNotificationTrigger").unwrap(), triggerWithTimeInterval : timeInterval, repeats : repeats)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct UNCalendarNotificationTrigger(pub id);
impl std::ops::Deref for UNCalendarNotificationTrigger {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for UNCalendarNotificationTrigger {}
impl UNCalendarNotificationTrigger {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"UNCalendarNotificationTrigger").unwrap(), alloc) })
    }
}
impl IUNNotificationTrigger for UNCalendarNotificationTrigger {}
impl PNSCopying for UNCalendarNotificationTrigger {}
impl PNSSecureCoding for UNCalendarNotificationTrigger {}
impl std::convert::TryFrom<UNNotificationTrigger> for UNCalendarNotificationTrigger {
    type Error = &'static str;
    fn try_from(
        parent: UNNotificationTrigger,
    ) -> Result<UNCalendarNotificationTrigger, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"UNCalendarNotificationTrigger").unwrap())
        };
        if is_kind_of {
            Ok(UNCalendarNotificationTrigger(parent.0))
        } else {
            Err("This UNNotificationTrigger cannot be downcasted to UNCalendarNotificationTrigger")
        }
    }
}
impl INSObject for UNCalendarNotificationTrigger {}
impl PNSObject for UNCalendarNotificationTrigger {}
impl IUNCalendarNotificationTrigger for UNCalendarNotificationTrigger {}
pub trait IUNCalendarNotificationTrigger: Sized + std::ops::Deref {
    unsafe fn nextTriggerDate(&self) -> NSDate
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, nextTriggerDate)
    }
    unsafe fn dateComponents(&self) -> NSDateComponents
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, dateComponents)
    }
    unsafe fn triggerWithDateMatchingComponents_repeats_(
        dateComponents: NSDateComponents,
        repeats: BOOL,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"UNCalendarNotificationTrigger").unwrap(), triggerWithDateMatchingComponents : dateComponents, repeats : repeats)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct UNLocationNotificationTrigger(pub id);
impl std::ops::Deref for UNLocationNotificationTrigger {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for UNLocationNotificationTrigger {}
impl UNLocationNotificationTrigger {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"UNLocationNotificationTrigger").unwrap(), alloc) })
    }
}
impl IUNNotificationTrigger for UNLocationNotificationTrigger {}
impl PNSCopying for UNLocationNotificationTrigger {}
impl PNSSecureCoding for UNLocationNotificationTrigger {}
impl std::convert::TryFrom<UNNotificationTrigger> for UNLocationNotificationTrigger {
    type Error = &'static str;
    fn try_from(
        parent: UNNotificationTrigger,
    ) -> Result<UNLocationNotificationTrigger, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"UNLocationNotificationTrigger").unwrap())
        };
        if is_kind_of {
            Ok(UNLocationNotificationTrigger(parent.0))
        } else {
            Err("This UNNotificationTrigger cannot be downcasted to UNLocationNotificationTrigger")
        }
    }
}
impl INSObject for UNLocationNotificationTrigger {}
impl PNSObject for UNLocationNotificationTrigger {}
impl IUNLocationNotificationTrigger for UNLocationNotificationTrigger {}
pub trait IUNLocationNotificationTrigger: Sized + std::ops::Deref {
    unsafe fn region(&self) -> CLRegion
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, region)
    }
    unsafe fn triggerWithRegion_repeats_(region: CLRegion, repeats: BOOL) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"UNLocationNotificationTrigger").unwrap(), triggerWithRegion : region, repeats : repeats)
    }
}
pub type UNAuthorizationOptions = NSUInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct UNUserNotificationCenter(pub id);
impl std::ops::Deref for UNUserNotificationCenter {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for UNUserNotificationCenter {}
impl UNUserNotificationCenter {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"UNUserNotificationCenter").unwrap(), alloc) })
    }
}
impl INSObject for UNUserNotificationCenter {}
impl PNSObject for UNUserNotificationCenter {}
impl std::convert::TryFrom<NSObject> for UNUserNotificationCenter {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<UNUserNotificationCenter, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"UNUserNotificationCenter").unwrap()) };
        if is_kind_of {
            Ok(UNUserNotificationCenter(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to UNUserNotificationCenter")
        }
    }
}
impl IUNUserNotificationCenter for UNUserNotificationCenter {}
pub trait IUNUserNotificationCenter: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn requestAuthorizationWithOptions_completionHandler_(
        &self,
        options: UNAuthorizationOptions,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, requestAuthorizationWithOptions : options, completionHandler : completionHandler)
    }
    unsafe fn setNotificationCategories_(&self, categories: NSSet)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setNotificationCategories : categories)
    }
    unsafe fn getNotificationCategoriesWithCompletionHandler_(
        &self,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, getNotificationCategoriesWithCompletionHandler : completionHandler)
    }
    unsafe fn getNotificationSettingsWithCompletionHandler_(
        &self,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, getNotificationSettingsWithCompletionHandler : completionHandler)
    }
    unsafe fn addNotificationRequest_withCompletionHandler_(
        &self,
        request: UNNotificationRequest,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, addNotificationRequest : request, withCompletionHandler : completionHandler)
    }
    unsafe fn getPendingNotificationRequestsWithCompletionHandler_(
        &self,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, getPendingNotificationRequestsWithCompletionHandler : completionHandler)
    }
    unsafe fn removePendingNotificationRequestsWithIdentifiers_(&self, identifiers: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, removePendingNotificationRequestsWithIdentifiers : identifiers)
    }
    unsafe fn removeAllPendingNotificationRequests(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, removeAllPendingNotificationRequests)
    }
    unsafe fn getDeliveredNotificationsWithCompletionHandler_(
        &self,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, getDeliveredNotificationsWithCompletionHandler : completionHandler)
    }
    unsafe fn removeDeliveredNotificationsWithIdentifiers_(&self, identifiers: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, removeDeliveredNotificationsWithIdentifiers : identifiers)
    }
    unsafe fn removeAllDeliveredNotifications(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, removeAllDeliveredNotifications)
    }
    unsafe fn setBadgeCount_withCompletionHandler_(
        &self,
        newBadgeCount: NSInteger,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setBadgeCount : newBadgeCount, withCompletionHandler : completionHandler)
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
    unsafe fn supportsContentExtensions(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, supportsContentExtensions)
    }
    unsafe fn currentNotificationCenter() -> UNUserNotificationCenter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"UNUserNotificationCenter").unwrap(), currentNotificationCenter)
    }
}
pub type UNNotificationPresentationOptions = NSUInteger;
pub trait PUNUserNotificationCenterDelegate: Sized + std::ops::Deref {
    unsafe fn userNotificationCenter_willPresentNotification_withCompletionHandler_(
        &self,
        center: UNUserNotificationCenter,
        notification: UNNotification,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, userNotificationCenter : center, willPresentNotification : notification, withCompletionHandler : completionHandler)
    }
    unsafe fn userNotificationCenter_didReceiveNotificationResponse_withCompletionHandler_(
        &self,
        center: UNUserNotificationCenter,
        response: UNNotificationResponse,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, userNotificationCenter : center, didReceiveNotificationResponse : response, withCompletionHandler : completionHandler)
    }
    unsafe fn userNotificationCenter_openSettingsForNotification_(
        &self,
        center: UNUserNotificationCenter,
        notification: UNNotification,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, userNotificationCenter : center, openSettingsForNotification : notification)
    }
}
unsafe extern "C" {
    pub static UNErrorDomain: NSString;
}
unsafe extern "C" {
    pub static UNNotificationAttachmentOptionsTypeHintKey: NSString;
}
unsafe extern "C" {
    pub static UNNotificationAttachmentOptionsThumbnailHiddenKey: NSString;
}
unsafe extern "C" {
    pub static UNNotificationAttachmentOptionsThumbnailClippingRectKey: NSString;
}
unsafe extern "C" {
    pub static UNNotificationAttachmentOptionsThumbnailTimeKey: NSString;
}
unsafe extern "C" {
    pub static UNNotificationDefaultActionIdentifier: NSString;
}
unsafe extern "C" {
    pub static UNNotificationDismissActionIdentifier: NSString;
}

unsafe impl objc2::encode::RefEncode for UNNotification {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for UNNotification {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for UNNotificationAction {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for UNNotificationAction {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for UNTextInputNotificationAction {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for UNTextInputNotificationAction {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for UNNotificationActionIcon {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for UNNotificationActionIcon {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for UNNotificationAttachment {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for UNNotificationAttachment {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for UNNotificationContent {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for UNNotificationContent {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for UNMutableNotificationContent {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for UNMutableNotificationContent {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for UNNotificationAttributedMessageContext {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for UNNotificationAttributedMessageContext {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for UNNotificationCategory {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for UNNotificationCategory {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for UNNotificationRequest {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for UNNotificationRequest {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for UNNotificationResponse {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for UNNotificationResponse {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for UNTextInputNotificationResponse {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for UNTextInputNotificationResponse {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for UNNotificationServiceExtension {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for UNNotificationServiceExtension {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for UNNotificationSettings {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for UNNotificationSettings {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for UNNotificationSound {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for UNNotificationSound {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for UNNotificationTrigger {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for UNNotificationTrigger {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for UNPushNotificationTrigger {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for UNPushNotificationTrigger {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for UNTimeIntervalNotificationTrigger {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for UNTimeIntervalNotificationTrigger {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for UNCalendarNotificationTrigger {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for UNCalendarNotificationTrigger {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for UNLocationNotificationTrigger {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for UNLocationNotificationTrigger {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for UNUserNotificationCenter {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for UNUserNotificationCenter {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
