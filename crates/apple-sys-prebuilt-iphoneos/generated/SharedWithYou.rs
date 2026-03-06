#[allow(unused_imports)]
use crate::objc::*;
#[allow(unused_imports)]
use crate::AddressBook::*;
#[allow(unused_imports)]
use crate::CoreGraphics::*;
#[allow(unused_imports)]
use crate::Foundation::*;
#[allow(unused_imports)]
use crate::Security::*;
#[allow(unused_imports)]
use crate::SharedWithYouCore::*;
#[allow(unused_imports)]
use crate::UniformTypeIdentifiers::*;

#[allow(unused_imports)]
use objc2::msg_send;
pub type SWHighlightCenterErrorCode = NSInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct SWHighlight(pub id);
impl std::ops::Deref for SWHighlight {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for SWHighlight {}
impl SWHighlight {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"SWHighlight").unwrap(), alloc) })
    }
}
impl PNSSecureCoding for SWHighlight {}
impl PNSCopying for SWHighlight {}
impl INSObject for SWHighlight {}
impl PNSObject for SWHighlight {}
impl std::convert::TryFrom<NSObject> for SWHighlight {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<SWHighlight, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"SWHighlight").unwrap()) };
        if is_kind_of {
            Ok(SWHighlight(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to SWHighlight")
        }
    }
}
impl ISWHighlight for SWHighlight {}
pub trait ISWHighlight: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn identifier(&self) -> *mut u64
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
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SWHighlight").unwrap(), new)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct SWCollaborationHighlight(pub id);
impl std::ops::Deref for SWCollaborationHighlight {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for SWCollaborationHighlight {}
impl SWCollaborationHighlight {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"SWCollaborationHighlight").unwrap(), alloc) })
    }
}
impl PNSSecureCoding for SWCollaborationHighlight {}
impl PNSCopying for SWCollaborationHighlight {}
impl ISWHighlight for SWCollaborationHighlight {}
impl From<SWCollaborationHighlight> for SWHighlight {
    fn from(child: SWCollaborationHighlight) -> SWHighlight {
        SWHighlight(child.0)
    }
}
impl std::convert::TryFrom<SWHighlight> for SWCollaborationHighlight {
    type Error = &'static str;
    fn try_from(parent: SWHighlight) -> Result<SWCollaborationHighlight, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"SWCollaborationHighlight").unwrap()) };
        if is_kind_of {
            Ok(SWCollaborationHighlight(parent.0))
        } else {
            Err("This SWHighlight cannot be downcasted to SWCollaborationHighlight")
        }
    }
}
impl INSObject for SWCollaborationHighlight {}
impl PNSObject for SWCollaborationHighlight {}
impl ISWCollaborationHighlight for SWCollaborationHighlight {}
pub trait ISWCollaborationHighlight: Sized + std::ops::Deref {
    unsafe fn collaborationIdentifier(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, collaborationIdentifier)
    }
    unsafe fn title(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, title)
    }
    unsafe fn creationDate(&self) -> NSDate
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, creationDate)
    }
    unsafe fn contentType(&self) -> UTType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, contentType)
    }
}
pub trait PSWHighlightEvent: Sized + std::ops::Deref {
    unsafe fn highlightURL(&self) -> NSURL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, highlightURL)
    }
}
pub trait PSWHighlightCenterDelegate: Sized + std::ops::Deref {
    unsafe fn highlightCenterHighlightsDidChange_(&self, highlightCenter: SWHighlightCenter)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, highlightCenterHighlightsDidChange : highlightCenter)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct SWHighlightCenter(pub id);
impl std::ops::Deref for SWHighlightCenter {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for SWHighlightCenter {}
impl SWHighlightCenter {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"SWHighlightCenter").unwrap(), alloc) })
    }
}
impl INSObject for SWHighlightCenter {}
impl PNSObject for SWHighlightCenter {}
impl std::convert::TryFrom<NSObject> for SWHighlightCenter {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<SWHighlightCenter, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"SWHighlightCenter").unwrap()) };
        if is_kind_of {
            Ok(SWHighlightCenter(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to SWHighlightCenter")
        }
    }
}
impl ISWHighlightCenter for SWHighlightCenter {}
pub trait ISWHighlightCenter: Sized + std::ops::Deref {
    unsafe fn getHighlightForURL_completionHandler_(
        &self,
        URL: NSURL,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, getHighlightForURL : URL, completionHandler : completionHandler)
    }
    unsafe fn collaborationHighlightForIdentifier_error_(
        &self,
        collaborationIdentifier: NSString,
        error: *mut NSError,
    ) -> SWCollaborationHighlight
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, collaborationHighlightForIdentifier : collaborationIdentifier, error : error)
    }
    unsafe fn getCollaborationHighlightForURL_completionHandler_(
        &self,
        URL: NSURL,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, getCollaborationHighlightForURL : URL, completionHandler : completionHandler)
    }
    unsafe fn postNoticeForHighlightEvent_(&self, event: *mut u64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, postNoticeForHighlightEvent : event)
    }
    unsafe fn clearNoticesForHighlight_(&self, highlight: SWCollaborationHighlight)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, clearNoticesForHighlight : highlight)
    }
    unsafe fn getSignedIdentityProofForCollaborationHighlight_usingData_completionHandler_(
        &self,
        collaborationHighlight: SWCollaborationHighlight,
        data: NSData,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, getSignedIdentityProofForCollaborationHighlight : collaborationHighlight, usingData : data, completionHandler : completionHandler)
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
    unsafe fn highlights(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, highlights)
    }
    unsafe fn highlightCollectionTitle() -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SWHighlightCenter").unwrap(), highlightCollectionTitle)
    }
    unsafe fn isSystemCollaborationSupportAvailable() -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SWHighlightCenter").unwrap(), isSystemCollaborationSupportAvailable)
    }
}
pub type SWHighlightChangeEventTrigger = NSInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct SWHighlightChangeEvent(pub id);
impl std::ops::Deref for SWHighlightChangeEvent {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for SWHighlightChangeEvent {}
impl SWHighlightChangeEvent {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"SWHighlightChangeEvent").unwrap(), alloc) })
    }
}
impl PSWHighlightEvent for SWHighlightChangeEvent {}
impl INSObject for SWHighlightChangeEvent {}
impl PNSObject for SWHighlightChangeEvent {}
impl std::convert::TryFrom<NSObject> for SWHighlightChangeEvent {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<SWHighlightChangeEvent, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"SWHighlightChangeEvent").unwrap()) };
        if is_kind_of {
            Ok(SWHighlightChangeEvent(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to SWHighlightChangeEvent")
        }
    }
}
impl ISWHighlightChangeEvent for SWHighlightChangeEvent {}
pub trait ISWHighlightChangeEvent: Sized + std::ops::Deref {
    unsafe fn initWithHighlight_trigger_(
        &self,
        highlight: SWHighlight,
        trigger: SWHighlightChangeEventTrigger,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithHighlight : highlight, trigger : trigger)
    }
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn changeEventTrigger(&self) -> SWHighlightChangeEventTrigger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, changeEventTrigger)
    }
    unsafe fn highlightURL(&self) -> NSURL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, highlightURL)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SWHighlightChangeEvent").unwrap(), new)
    }
}
pub type SWHighlightMembershipEventTrigger = NSInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct SWHighlightMembershipEvent(pub id);
impl std::ops::Deref for SWHighlightMembershipEvent {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for SWHighlightMembershipEvent {}
impl SWHighlightMembershipEvent {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"SWHighlightMembershipEvent").unwrap(), alloc) })
    }
}
impl PSWHighlightEvent for SWHighlightMembershipEvent {}
impl INSObject for SWHighlightMembershipEvent {}
impl PNSObject for SWHighlightMembershipEvent {}
impl std::convert::TryFrom<NSObject> for SWHighlightMembershipEvent {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<SWHighlightMembershipEvent, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"SWHighlightMembershipEvent").unwrap()) };
        if is_kind_of {
            Ok(SWHighlightMembershipEvent(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to SWHighlightMembershipEvent")
        }
    }
}
impl ISWHighlightMembershipEvent for SWHighlightMembershipEvent {}
pub trait ISWHighlightMembershipEvent: Sized + std::ops::Deref {
    unsafe fn initWithHighlight_trigger_(
        &self,
        highlight: SWHighlight,
        trigger: SWHighlightMembershipEventTrigger,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithHighlight : highlight, trigger : trigger)
    }
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn membershipEventTrigger(&self) -> SWHighlightMembershipEventTrigger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, membershipEventTrigger)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SWHighlightMembershipEvent").unwrap(), new)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct SWHighlightMentionEvent(pub id);
impl std::ops::Deref for SWHighlightMentionEvent {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for SWHighlightMentionEvent {}
impl SWHighlightMentionEvent {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"SWHighlightMentionEvent").unwrap(), alloc) })
    }
}
impl PSWHighlightEvent for SWHighlightMentionEvent {}
impl INSObject for SWHighlightMentionEvent {}
impl PNSObject for SWHighlightMentionEvent {}
impl std::convert::TryFrom<NSObject> for SWHighlightMentionEvent {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<SWHighlightMentionEvent, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"SWHighlightMentionEvent").unwrap()) };
        if is_kind_of {
            Ok(SWHighlightMentionEvent(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to SWHighlightMentionEvent")
        }
    }
}
impl ISWHighlightMentionEvent for SWHighlightMentionEvent {}
pub trait ISWHighlightMentionEvent: Sized + std::ops::Deref {
    unsafe fn initWithHighlight_mentionedPersonCloudKitShareHandle_(
        &self,
        highlight: SWHighlight,
        handle: NSString,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithHighlight : highlight, mentionedPersonCloudKitShareHandle : handle)
    }
    unsafe fn initWithHighlight_mentionedPersonIdentity_(
        &self,
        highlight: SWHighlight,
        identity: SWPersonIdentity,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithHighlight : highlight, mentionedPersonIdentity : identity)
    }
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn mentionedPersonHandle(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, mentionedPersonHandle)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SWHighlightMentionEvent").unwrap(), new)
    }
}
pub type SWHighlightPersistenceEventTrigger = NSInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct SWHighlightPersistenceEvent(pub id);
impl std::ops::Deref for SWHighlightPersistenceEvent {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for SWHighlightPersistenceEvent {}
impl SWHighlightPersistenceEvent {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"SWHighlightPersistenceEvent").unwrap(), alloc) })
    }
}
impl PSWHighlightEvent for SWHighlightPersistenceEvent {}
impl INSObject for SWHighlightPersistenceEvent {}
impl PNSObject for SWHighlightPersistenceEvent {}
impl std::convert::TryFrom<NSObject> for SWHighlightPersistenceEvent {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<SWHighlightPersistenceEvent, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"SWHighlightPersistenceEvent").unwrap()) };
        if is_kind_of {
            Ok(SWHighlightPersistenceEvent(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to SWHighlightPersistenceEvent")
        }
    }
}
impl ISWHighlightPersistenceEvent for SWHighlightPersistenceEvent {}
pub trait ISWHighlightPersistenceEvent: Sized + std::ops::Deref {
    unsafe fn initWithHighlight_trigger_(
        &self,
        highlight: SWHighlight,
        trigger: SWHighlightPersistenceEventTrigger,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithHighlight : highlight, trigger : trigger)
    }
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn persistenceEventTrigger(&self) -> SWHighlightPersistenceEventTrigger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, persistenceEventTrigger)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SWHighlightPersistenceEvent").unwrap(), new)
    }
}
pub type SWAttributionViewDisplayContext = NSInteger;
pub type SWAttributionViewHorizontalAlignment = NSInteger;
pub type SWAttributionViewBackgroundStyle = NSInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct SWAttributionView(pub id);
impl std::ops::Deref for SWAttributionView {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for SWAttributionView {}
impl SWAttributionView {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"SWAttributionView").unwrap(), alloc) })
    }
}
impl INSView for SWAttributionView {}
impl PNSAnimatablePropertyContainer for SWAttributionView {}
impl PNSUserInterfaceItemIdentification for SWAttributionView {}
impl PNSDraggingDestination for SWAttributionView {}
impl PNSAppearanceCustomization for SWAttributionView {}
impl PNSAccessibilityElement for SWAttributionView {}
impl PNSAccessibility for SWAttributionView {}
impl std::convert::TryFrom<NSView> for SWAttributionView {
    type Error = &'static str;
    fn try_from(parent: NSView) -> Result<SWAttributionView, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"SWAttributionView").unwrap()) };
        if is_kind_of {
            Ok(SWAttributionView(parent.0))
        } else {
            Err("This NSView cannot be downcasted to SWAttributionView")
        }
    }
}
impl INSResponder for SWAttributionView {}
impl PNSCoding for SWAttributionView {}
impl INSObject for SWAttributionView {}
impl PNSObject for SWAttributionView {}
impl ISWAttributionView for SWAttributionView {}
pub trait ISWAttributionView: Sized + std::ops::Deref {
    unsafe fn highlight(&self) -> SWHighlight
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, highlight)
    }
    unsafe fn setHighlight_(&self, highlight: SWHighlight)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setHighlight : highlight)
    }
    unsafe fn displayContext(&self) -> SWAttributionViewDisplayContext
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, displayContext)
    }
    unsafe fn setDisplayContext_(&self, displayContext: SWAttributionViewDisplayContext)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDisplayContext : displayContext)
    }
    unsafe fn horizontalAlignment(&self) -> SWAttributionViewHorizontalAlignment
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, horizontalAlignment)
    }
    unsafe fn setHorizontalAlignment_(
        &self,
        horizontalAlignment: SWAttributionViewHorizontalAlignment,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setHorizontalAlignment : horizontalAlignment)
    }
    unsafe fn backgroundStyle(&self) -> SWAttributionViewBackgroundStyle
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, backgroundStyle)
    }
    unsafe fn setBackgroundStyle_(&self, backgroundStyle: SWAttributionViewBackgroundStyle)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setBackgroundStyle : backgroundStyle)
    }
    unsafe fn preferredMaxLayoutWidth(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, preferredMaxLayoutWidth)
    }
    unsafe fn setPreferredMaxLayoutWidth_(&self, preferredMaxLayoutWidth: CGFloat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPreferredMaxLayoutWidth : preferredMaxLayoutWidth)
    }
    unsafe fn highlightMenu(&self) -> NSMenu
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, highlightMenu)
    }
    unsafe fn menuTitleForHideAction(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, menuTitleForHideAction)
    }
    unsafe fn setMenuTitleForHideAction_(&self, menuTitleForHideAction: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMenuTitleForHideAction : menuTitleForHideAction)
    }
    unsafe fn supplementalMenu(&self) -> NSMenuItem
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, supplementalMenu)
    }
    unsafe fn setSupplementalMenu_(&self, supplementalMenu: NSMenuItem)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSupplementalMenu : supplementalMenu)
    }
    unsafe fn enablesMarquee(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, enablesMarquee)
    }
    unsafe fn setEnablesMarquee_(&self, enablesMarquee: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setEnablesMarquee : enablesMarquee)
    }
}
pub trait PSWCollaborationViewDelegate: Sized + std::ops::Deref {
    unsafe fn collaborationViewShouldPresentPopover_(
        &self,
        collaborationView: SWCollaborationView,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, collaborationViewShouldPresentPopover : collaborationView)
    }
    unsafe fn collaborationViewWillPresentPopover_(&self, collaborationView: SWCollaborationView)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, collaborationViewWillPresentPopover : collaborationView)
    }
    unsafe fn collaborationViewDidDismissPopover_(&self, collaborationView: SWCollaborationView)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, collaborationViewDidDismissPopover : collaborationView)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct SWCollaborationView(pub id);
impl std::ops::Deref for SWCollaborationView {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for SWCollaborationView {}
impl SWCollaborationView {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"SWCollaborationView").unwrap(), alloc) })
    }
}
impl INSView for SWCollaborationView {}
impl PNSAnimatablePropertyContainer for SWCollaborationView {}
impl PNSUserInterfaceItemIdentification for SWCollaborationView {}
impl PNSDraggingDestination for SWCollaborationView {}
impl PNSAppearanceCustomization for SWCollaborationView {}
impl PNSAccessibilityElement for SWCollaborationView {}
impl PNSAccessibility for SWCollaborationView {}
impl std::convert::TryFrom<NSView> for SWCollaborationView {
    type Error = &'static str;
    fn try_from(parent: NSView) -> Result<SWCollaborationView, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"SWCollaborationView").unwrap()) };
        if is_kind_of {
            Ok(SWCollaborationView(parent.0))
        } else {
            Err("This NSView cannot be downcasted to SWCollaborationView")
        }
    }
}
impl INSResponder for SWCollaborationView {}
impl PNSCoding for SWCollaborationView {}
impl INSObject for SWCollaborationView {}
impl PNSObject for SWCollaborationView {}
impl ISWCollaborationView for SWCollaborationView {}
pub trait ISWCollaborationView: Sized + std::ops::Deref {
    unsafe fn setContentView_(&self, detailViewListContentView: NSView)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setContentView : detailViewListContentView)
    }
    unsafe fn initWithItemProvider_(&self, itemProvider: NSItemProvider) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithItemProvider : itemProvider)
    }
    unsafe fn dismissPopover_(&self, completion: *mut ::std::os::raw::c_void)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, dismissPopover : completion)
    }
    unsafe fn setShowManageButton_(&self, showManageButton: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setShowManageButton : showManageButton)
    }
    unsafe fn cloudSharingDelegate(&self) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, cloudSharingDelegate)
    }
    unsafe fn setCloudSharingDelegate_(&self, cloudSharingDelegate: *mut u64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCloudSharingDelegate : cloudSharingDelegate)
    }
    unsafe fn activeParticipantCount(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, activeParticipantCount)
    }
    unsafe fn setActiveParticipantCount_(&self, activeParticipantCount: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setActiveParticipantCount : activeParticipantCount)
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
    unsafe fn headerTitle(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, headerTitle)
    }
    unsafe fn setHeaderTitle_(&self, headerTitle: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setHeaderTitle : headerTitle)
    }
    unsafe fn headerSubtitle(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, headerSubtitle)
    }
    unsafe fn setHeaderSubtitle_(&self, headerSubtitle: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setHeaderSubtitle : headerSubtitle)
    }
    unsafe fn headerImage(&self) -> NSImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, headerImage)
    }
    unsafe fn setHeaderImage_(&self, headerImage: NSImage)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setHeaderImage : headerImage)
    }
    unsafe fn menuFormRepresentation(&self) -> NSMenuItem
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, menuFormRepresentation)
    }
    unsafe fn cloudSharingServiceDelegate(&self) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, cloudSharingServiceDelegate)
    }
    unsafe fn setCloudSharingServiceDelegate_(&self, cloudSharingServiceDelegate: *mut u64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCloudSharingServiceDelegate : cloudSharingServiceDelegate)
    }
    unsafe fn manageButtonTitle(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, manageButtonTitle)
    }
    unsafe fn setManageButtonTitle_(&self, manageButtonTitle: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setManageButtonTitle : manageButtonTitle)
    }
}
pub trait SWCollaborationMetadata_NSItemProvider: Sized + std::ops::Deref {}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct SWRemoveParticipantAlert(pub id);
impl std::ops::Deref for SWRemoveParticipantAlert {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for SWRemoveParticipantAlert {}
impl SWRemoveParticipantAlert {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"SWRemoveParticipantAlert").unwrap(), alloc) })
    }
}
impl INSObject for SWRemoveParticipantAlert {}
impl PNSObject for SWRemoveParticipantAlert {}
impl std::convert::TryFrom<NSObject> for SWRemoveParticipantAlert {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<SWRemoveParticipantAlert, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"SWRemoveParticipantAlert").unwrap()) };
        if is_kind_of {
            Ok(SWRemoveParticipantAlert(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to SWRemoveParticipantAlert")
        }
    }
}
impl ISWRemoveParticipantAlert for SWRemoveParticipantAlert {}
pub trait ISWRemoveParticipantAlert: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn showAlertWithParticipant_highlight_inWindow_(
        participant: SWPerson,
        highlight: SWCollaborationHighlight,
        window: NSWindow,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SWRemoveParticipantAlert").unwrap(), showAlertWithParticipant : participant, highlight : highlight, inWindow : window)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SWRemoveParticipantAlert").unwrap(), new)
    }
}
pub trait NSPasteboardItem_SWCollaborationMetadata: Sized + std::ops::Deref {
    unsafe fn collaborationMetadata(&self) -> SWCollaborationMetadata
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, collaborationMetadata)
    }
    unsafe fn setCollaborationMetadata_(&self, collaborationMetadata: SWCollaborationMetadata)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCollaborationMetadata : collaborationMetadata)
    }
}
unsafe extern "C" {
    pub static SWCollaborationMetadataTypeIdentifier: NSString;
}

unsafe impl objc2::encode::RefEncode for SWHighlight {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for SWHighlight {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for SWCollaborationHighlight {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for SWCollaborationHighlight {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for SWHighlightCenter {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for SWHighlightCenter {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for SWHighlightChangeEvent {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for SWHighlightChangeEvent {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for SWHighlightMembershipEvent {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for SWHighlightMembershipEvent {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for SWHighlightMentionEvent {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for SWHighlightMentionEvent {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for SWHighlightPersistenceEvent {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for SWHighlightPersistenceEvent {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for SWAttributionView {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for SWAttributionView {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for SWCollaborationView {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for SWCollaborationView {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for SWRemoveParticipantAlert {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for SWRemoveParticipantAlert {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
