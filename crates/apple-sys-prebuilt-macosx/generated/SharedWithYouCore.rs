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
pub struct SWAction(pub id);
impl std::ops::Deref for SWAction {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for SWAction {}
impl SWAction {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"SWAction").unwrap(), alloc) })
    }
}
impl PNSCopying for SWAction {}
impl PNSSecureCoding for SWAction {}
impl INSObject for SWAction {}
impl PNSObject for SWAction {}
impl std::convert::TryFrom<NSObject> for SWAction {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<SWAction, Self::Error> {
        let is_kind_of: bool = unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"SWAction").unwrap()) };
        if is_kind_of {
            Ok(SWAction(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to SWAction")
        }
    }
}
impl ISWAction for SWAction {}
pub trait ISWAction: Sized + std::ops::Deref {
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
    unsafe fn uuid(&self) -> NSUUID
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, uuid)
    }
    unsafe fn isComplete(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isComplete)
    }
}
pub trait PSWCollaborationActionHandler: Sized + std::ops::Deref {
    unsafe fn collaborationCoordinator_handleStartCollaborationAction_(
        &self,
        coordinator: SWCollaborationCoordinator,
        action: SWStartCollaborationAction,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, collaborationCoordinator : coordinator, handleStartCollaborationAction : action)
    }
    unsafe fn collaborationCoordinator_handleUpdateCollaborationParticipantsAction_(
        &self,
        coordinator: SWCollaborationCoordinator,
        action: SWUpdateCollaborationParticipantsAction,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, collaborationCoordinator : coordinator, handleUpdateCollaborationParticipantsAction : action)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct SWCollaborationCoordinator(pub id);
impl std::ops::Deref for SWCollaborationCoordinator {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for SWCollaborationCoordinator {}
impl SWCollaborationCoordinator {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"SWCollaborationCoordinator").unwrap(), alloc) })
    }
}
impl INSObject for SWCollaborationCoordinator {}
impl PNSObject for SWCollaborationCoordinator {}
impl std::convert::TryFrom<NSObject> for SWCollaborationCoordinator {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<SWCollaborationCoordinator, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"SWCollaborationCoordinator").unwrap()) };
        if is_kind_of {
            Ok(SWCollaborationCoordinator(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to SWCollaborationCoordinator")
        }
    }
}
impl ISWCollaborationCoordinator for SWCollaborationCoordinator {}
pub trait ISWCollaborationCoordinator: Sized + std::ops::Deref {
    unsafe fn actionHandler(&self) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, actionHandler)
    }
    unsafe fn setActionHandler_(&self, actionHandler: *mut u64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setActionHandler : actionHandler)
    }
    unsafe fn sharedCoordinator() -> SWCollaborationCoordinator
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SWCollaborationCoordinator").unwrap(), sharedCoordinator)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct SWCollaborationOption(pub id);
impl std::ops::Deref for SWCollaborationOption {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for SWCollaborationOption {}
impl SWCollaborationOption {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"SWCollaborationOption").unwrap(), alloc) })
    }
}
impl PNSCopying for SWCollaborationOption {}
impl PNSSecureCoding for SWCollaborationOption {}
impl INSObject for SWCollaborationOption {}
impl PNSObject for SWCollaborationOption {}
impl std::convert::TryFrom<NSObject> for SWCollaborationOption {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<SWCollaborationOption, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"SWCollaborationOption").unwrap()) };
        if is_kind_of {
            Ok(SWCollaborationOption(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to SWCollaborationOption")
        }
    }
}
impl ISWCollaborationOption for SWCollaborationOption {}
pub trait ISWCollaborationOption: Sized + std::ops::Deref {
    unsafe fn initWithTitle_identifier_(
        &self,
        title: NSString,
        identifier: NSString,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithTitle : title, identifier : identifier)
    }
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
    unsafe fn isSelected(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isSelected)
    }
    unsafe fn setSelected_(&self, selected: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSelected : selected)
    }
    unsafe fn requiredOptionsIdentifiers(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, requiredOptionsIdentifiers)
    }
    unsafe fn setRequiredOptionsIdentifiers_(&self, requiredOptionsIdentifiers: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRequiredOptionsIdentifiers : requiredOptionsIdentifiers)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SWCollaborationOption").unwrap(), new)
    }
    unsafe fn optionWithTitle_identifier_(
        title: NSString,
        identifier: NSString,
    ) -> SWCollaborationOption
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SWCollaborationOption").unwrap(), optionWithTitle : title, identifier : identifier)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct SWCollaborationOptionsGroup(pub id);
impl std::ops::Deref for SWCollaborationOptionsGroup {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for SWCollaborationOptionsGroup {}
impl SWCollaborationOptionsGroup {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"SWCollaborationOptionsGroup").unwrap(), alloc) })
    }
}
impl PNSCopying for SWCollaborationOptionsGroup {}
impl PNSSecureCoding for SWCollaborationOptionsGroup {}
impl INSObject for SWCollaborationOptionsGroup {}
impl PNSObject for SWCollaborationOptionsGroup {}
impl std::convert::TryFrom<NSObject> for SWCollaborationOptionsGroup {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<SWCollaborationOptionsGroup, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"SWCollaborationOptionsGroup").unwrap()) };
        if is_kind_of {
            Ok(SWCollaborationOptionsGroup(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to SWCollaborationOptionsGroup")
        }
    }
}
impl ISWCollaborationOptionsGroup for SWCollaborationOptionsGroup {}
pub trait ISWCollaborationOptionsGroup: Sized + std::ops::Deref {
    unsafe fn initWithIdentifier_options_(
        &self,
        identifier: NSString,
        options: NSArray,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithIdentifier : identifier, options : options)
    }
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
    unsafe fn footer(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, footer)
    }
    unsafe fn setFooter_(&self, footer: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setFooter : footer)
    }
    unsafe fn options(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, options)
    }
    unsafe fn setOptions_(&self, options: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setOptions : options)
    }
    unsafe fn optionsGroupWithIdentifier_options_(
        identifier: NSString,
        options: NSArray,
    ) -> SWCollaborationOptionsGroup
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SWCollaborationOptionsGroup").unwrap(), optionsGroupWithIdentifier : identifier, options : options)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SWCollaborationOptionsGroup").unwrap(), new)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct SWCollaborationOptionsPickerGroup(pub id);
impl std::ops::Deref for SWCollaborationOptionsPickerGroup {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for SWCollaborationOptionsPickerGroup {}
impl SWCollaborationOptionsPickerGroup {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"SWCollaborationOptionsPickerGroup").unwrap(), alloc) })
    }
}
impl ISWCollaborationOptionsGroup for SWCollaborationOptionsPickerGroup {}
impl PNSCopying for SWCollaborationOptionsPickerGroup {}
impl PNSSecureCoding for SWCollaborationOptionsPickerGroup {}
impl From<SWCollaborationOptionsPickerGroup> for SWCollaborationOptionsGroup {
    fn from(child: SWCollaborationOptionsPickerGroup) -> SWCollaborationOptionsGroup {
        SWCollaborationOptionsGroup(child.0)
    }
}
impl std::convert::TryFrom<SWCollaborationOptionsGroup> for SWCollaborationOptionsPickerGroup {
    type Error = &'static str;
    fn try_from(
        parent: SWCollaborationOptionsGroup,
    ) -> Result<SWCollaborationOptionsPickerGroup, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"SWCollaborationOptionsPickerGroup").unwrap())
        };
        if is_kind_of {
            Ok(SWCollaborationOptionsPickerGroup(parent.0))
        } else {
            Err ("This SWCollaborationOptionsGroup cannot be downcasted to SWCollaborationOptionsPickerGroup" ,)
        }
    }
}
impl INSObject for SWCollaborationOptionsPickerGroup {}
impl PNSObject for SWCollaborationOptionsPickerGroup {}
impl ISWCollaborationOptionsPickerGroup for SWCollaborationOptionsPickerGroup {}
pub trait ISWCollaborationOptionsPickerGroup: Sized + std::ops::Deref {
    unsafe fn selectedOptionIdentifier(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, selectedOptionIdentifier)
    }
    unsafe fn setSelectedOptionIdentifier_(&self, selectedOptionIdentifier: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSelectedOptionIdentifier : selectedOptionIdentifier)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct SWCollaborationShareOptions(pub id);
impl std::ops::Deref for SWCollaborationShareOptions {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for SWCollaborationShareOptions {}
impl SWCollaborationShareOptions {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"SWCollaborationShareOptions").unwrap(), alloc) })
    }
}
impl PNSCopying for SWCollaborationShareOptions {}
impl PNSSecureCoding for SWCollaborationShareOptions {}
impl INSObject for SWCollaborationShareOptions {}
impl PNSObject for SWCollaborationShareOptions {}
impl std::convert::TryFrom<NSObject> for SWCollaborationShareOptions {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<SWCollaborationShareOptions, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"SWCollaborationShareOptions").unwrap()) };
        if is_kind_of {
            Ok(SWCollaborationShareOptions(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to SWCollaborationShareOptions")
        }
    }
}
impl ISWCollaborationShareOptions for SWCollaborationShareOptions {}
pub trait ISWCollaborationShareOptions: Sized + std::ops::Deref {
    unsafe fn initWithOptionsGroups_summary_(
        &self,
        optionsGroups: NSArray,
        summary: NSString,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithOptionsGroups : optionsGroups, summary : summary)
    }
    unsafe fn initWithOptionsGroups_(&self, optionsGroups: NSArray) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithOptionsGroups : optionsGroups)
    }
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn initWithCoder_(&self, coder: NSCoder) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithCoder : coder)
    }
    unsafe fn optionsGroups(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, optionsGroups)
    }
    unsafe fn setOptionsGroups_(&self, optionsGroups: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setOptionsGroups : optionsGroups)
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
    unsafe fn shareOptionsWithOptionsGroups_summary_(
        optionsGroups: NSArray,
        summary: NSString,
    ) -> SWCollaborationShareOptions
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SWCollaborationShareOptions").unwrap(), shareOptionsWithOptionsGroups : optionsGroups, summary : summary)
    }
    unsafe fn shareOptionsWithOptionsGroups_(optionsGroups: NSArray) -> SWCollaborationShareOptions
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SWCollaborationShareOptions").unwrap(), shareOptionsWithOptionsGroups : optionsGroups)
    }
}
pub type SWCollaborationIdentifier = NSString;
pub type SWLocalCollaborationIdentifier = NSString;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct SWCollaborationMetadata(pub id);
impl std::ops::Deref for SWCollaborationMetadata {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for SWCollaborationMetadata {}
impl SWCollaborationMetadata {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"SWCollaborationMetadata").unwrap(), alloc) })
    }
}
impl PNSSecureCoding for SWCollaborationMetadata {}
impl PNSCopying for SWCollaborationMetadata {}
impl PNSMutableCopying for SWCollaborationMetadata {}
impl INSObject for SWCollaborationMetadata {}
impl PNSObject for SWCollaborationMetadata {}
impl std::convert::TryFrom<NSObject> for SWCollaborationMetadata {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<SWCollaborationMetadata, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"SWCollaborationMetadata").unwrap()) };
        if is_kind_of {
            Ok(SWCollaborationMetadata(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to SWCollaborationMetadata")
        }
    }
}
impl ISWCollaborationMetadata for SWCollaborationMetadata {}
pub trait ISWCollaborationMetadata: Sized + std::ops::Deref {
    unsafe fn initWithLocalIdentifier_(&self, localIdentifier: NSString) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithLocalIdentifier : localIdentifier)
    }
    unsafe fn initWithCollaborationIdentifier_(
        &self,
        collaborationIdentifier: NSString,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithCollaborationIdentifier : collaborationIdentifier)
    }
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn collaborationIdentifier(&self) -> SWCollaborationIdentifier
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, collaborationIdentifier)
    }
    unsafe fn localIdentifier(&self) -> SWLocalCollaborationIdentifier
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, localIdentifier)
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
    unsafe fn defaultShareOptions(&self) -> SWCollaborationShareOptions
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, defaultShareOptions)
    }
    unsafe fn setDefaultShareOptions_(&self, defaultShareOptions: SWCollaborationShareOptions)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDefaultShareOptions : defaultShareOptions)
    }
    unsafe fn userSelectedShareOptions(&self) -> SWCollaborationShareOptions
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, userSelectedShareOptions)
    }
    unsafe fn setUserSelectedShareOptions_(
        &self,
        userSelectedShareOptions: SWCollaborationShareOptions,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setUserSelectedShareOptions : userSelectedShareOptions)
    }
    unsafe fn initiatorHandle(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, initiatorHandle)
    }
    unsafe fn setInitiatorHandle_(&self, initiatorHandle: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInitiatorHandle : initiatorHandle)
    }
    unsafe fn initiatorNameComponents(&self) -> NSPersonNameComponents
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, initiatorNameComponents)
    }
    unsafe fn setInitiatorNameComponents_(&self, initiatorNameComponents: NSPersonNameComponents)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInitiatorNameComponents : initiatorNameComponents)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SWCollaborationMetadata").unwrap(), new)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct SWPerson(pub id);
impl std::ops::Deref for SWPerson {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for SWPerson {}
impl SWPerson {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"SWPerson").unwrap(), alloc) })
    }
}
impl PNSSecureCoding for SWPerson {}
impl INSObject for SWPerson {}
impl PNSObject for SWPerson {}
impl std::convert::TryFrom<NSObject> for SWPerson {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<SWPerson, Self::Error> {
        let is_kind_of: bool = unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"SWPerson").unwrap()) };
        if is_kind_of {
            Ok(SWPerson(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to SWPerson")
        }
    }
}
impl ISWPerson for SWPerson {}
pub trait ISWPerson: Sized + std::ops::Deref {
    unsafe fn initWithHandle_identity_displayName_thumbnailImageData_(
        &self,
        handle: NSString,
        identity: SWPersonIdentity,
        displayName: NSString,
        thumbnailImageData: NSData,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithHandle : handle, identity : identity, displayName : displayName, thumbnailImageData : thumbnailImageData)
    }
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
        msg_send!(&*objc2::runtime::AnyClass::get(c"SWPerson").unwrap(), new)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct SWPersonIdentity(pub id);
impl std::ops::Deref for SWPersonIdentity {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for SWPersonIdentity {}
impl SWPersonIdentity {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"SWPersonIdentity").unwrap(), alloc) })
    }
}
impl PNSSecureCoding for SWPersonIdentity {}
impl PNSCopying for SWPersonIdentity {}
impl INSObject for SWPersonIdentity {}
impl PNSObject for SWPersonIdentity {}
impl std::convert::TryFrom<NSObject> for SWPersonIdentity {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<SWPersonIdentity, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"SWPersonIdentity").unwrap()) };
        if is_kind_of {
            Ok(SWPersonIdentity(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to SWPersonIdentity")
        }
    }
}
impl ISWPersonIdentity for SWPersonIdentity {}
pub trait ISWPersonIdentity: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn initWithRootHash_(&self, rootHash: NSData) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithRootHash : rootHash)
    }
    unsafe fn rootHash(&self) -> NSData
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, rootHash)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SWPersonIdentity").unwrap(), new)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct SWPersonIdentityProof(pub id);
impl std::ops::Deref for SWPersonIdentityProof {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for SWPersonIdentityProof {}
impl SWPersonIdentityProof {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"SWPersonIdentityProof").unwrap(), alloc) })
    }
}
impl PNSSecureCoding for SWPersonIdentityProof {}
impl PNSCopying for SWPersonIdentityProof {}
impl INSObject for SWPersonIdentityProof {}
impl PNSObject for SWPersonIdentityProof {}
impl std::convert::TryFrom<NSObject> for SWPersonIdentityProof {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<SWPersonIdentityProof, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"SWPersonIdentityProof").unwrap()) };
        if is_kind_of {
            Ok(SWPersonIdentityProof(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to SWPersonIdentityProof")
        }
    }
}
impl ISWPersonIdentityProof for SWPersonIdentityProof {}
pub trait ISWPersonIdentityProof: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn inclusionHashes(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inclusionHashes)
    }
    unsafe fn publicKey(&self) -> NSData
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, publicKey)
    }
    unsafe fn publicKeyIndex(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, publicKeyIndex)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SWPersonIdentityProof").unwrap(), new)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct SWSignedPersonIdentityProof(pub id);
impl std::ops::Deref for SWSignedPersonIdentityProof {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for SWSignedPersonIdentityProof {}
impl SWSignedPersonIdentityProof {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"SWSignedPersonIdentityProof").unwrap(), alloc) })
    }
}
impl ISWPersonIdentityProof for SWSignedPersonIdentityProof {}
impl PNSSecureCoding for SWSignedPersonIdentityProof {}
impl PNSCopying for SWSignedPersonIdentityProof {}
impl From<SWSignedPersonIdentityProof> for SWPersonIdentityProof {
    fn from(child: SWSignedPersonIdentityProof) -> SWPersonIdentityProof {
        SWPersonIdentityProof(child.0)
    }
}
impl std::convert::TryFrom<SWPersonIdentityProof> for SWSignedPersonIdentityProof {
    type Error = &'static str;
    fn try_from(parent: SWPersonIdentityProof) -> Result<SWSignedPersonIdentityProof, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"SWSignedPersonIdentityProof").unwrap()) };
        if is_kind_of {
            Ok(SWSignedPersonIdentityProof(parent.0))
        } else {
            Err("This SWPersonIdentityProof cannot be downcasted to SWSignedPersonIdentityProof")
        }
    }
}
impl INSObject for SWSignedPersonIdentityProof {}
impl PNSObject for SWSignedPersonIdentityProof {}
impl ISWSignedPersonIdentityProof for SWSignedPersonIdentityProof {}
pub trait ISWSignedPersonIdentityProof: Sized + std::ops::Deref {
    unsafe fn initWithPersonIdentityProof_signatureData_(
        &self,
        personIdentityProof: SWPersonIdentityProof,
        data: NSData,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithPersonIdentityProof : personIdentityProof, signatureData : data)
    }
    unsafe fn signatureData(&self) -> NSData
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, signatureData)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct SWStartCollaborationAction(pub id);
impl std::ops::Deref for SWStartCollaborationAction {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for SWStartCollaborationAction {}
impl SWStartCollaborationAction {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"SWStartCollaborationAction").unwrap(), alloc) })
    }
}
impl PNSSecureCoding for SWStartCollaborationAction {}
impl PNSCopying for SWStartCollaborationAction {}
impl ISWAction for SWStartCollaborationAction {}
impl From<SWStartCollaborationAction> for SWAction {
    fn from(child: SWStartCollaborationAction) -> SWAction {
        SWAction(child.0)
    }
}
impl std::convert::TryFrom<SWAction> for SWStartCollaborationAction {
    type Error = &'static str;
    fn try_from(parent: SWAction) -> Result<SWStartCollaborationAction, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"SWStartCollaborationAction").unwrap()) };
        if is_kind_of {
            Ok(SWStartCollaborationAction(parent.0))
        } else {
            Err("This SWAction cannot be downcasted to SWStartCollaborationAction")
        }
    }
}
impl INSObject for SWStartCollaborationAction {}
impl PNSObject for SWStartCollaborationAction {}
impl ISWStartCollaborationAction for SWStartCollaborationAction {}
pub trait ISWStartCollaborationAction: Sized + std::ops::Deref {
    unsafe fn fulfillUsingURL_collaborationIdentifier_(
        &self,
        url: NSURL,
        collaborationIdentifier: NSString,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, fulfillUsingURL : url, collaborationIdentifier : collaborationIdentifier)
    }
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn collaborationMetadata(&self) -> SWCollaborationMetadata
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, collaborationMetadata)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SWStartCollaborationAction").unwrap(), new)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct SWUpdateCollaborationParticipantsAction(pub id);
impl std::ops::Deref for SWUpdateCollaborationParticipantsAction {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for SWUpdateCollaborationParticipantsAction {}
impl SWUpdateCollaborationParticipantsAction {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"SWUpdateCollaborationParticipantsAction").unwrap(), alloc) })
    }
}
impl PNSSecureCoding for SWUpdateCollaborationParticipantsAction {}
impl PNSCopying for SWUpdateCollaborationParticipantsAction {}
impl ISWAction for SWUpdateCollaborationParticipantsAction {}
impl std::convert::TryFrom<SWAction> for SWUpdateCollaborationParticipantsAction {
    type Error = &'static str;
    fn try_from(parent: SWAction) -> Result<SWUpdateCollaborationParticipantsAction, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"SWUpdateCollaborationParticipantsAction").unwrap())
        };
        if is_kind_of {
            Ok(SWUpdateCollaborationParticipantsAction(parent.0))
        } else {
            Err("This SWAction cannot be downcasted to SWUpdateCollaborationParticipantsAction")
        }
    }
}
impl INSObject for SWUpdateCollaborationParticipantsAction {}
impl PNSObject for SWUpdateCollaborationParticipantsAction {}
impl ISWUpdateCollaborationParticipantsAction for SWUpdateCollaborationParticipantsAction {}
pub trait ISWUpdateCollaborationParticipantsAction: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn collaborationMetadata(&self) -> SWCollaborationMetadata
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, collaborationMetadata)
    }
    unsafe fn addedIdentities(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, addedIdentities)
    }
    unsafe fn removedIdentities(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, removedIdentities)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SWUpdateCollaborationParticipantsAction").unwrap(), new)
    }
}
unsafe extern "C" {
    pub static mut SharedWithYouCoreVersionNumber: f64;
}
unsafe extern "C" {
    pub static SharedWithYouCoreVersionString: [::std::os::raw::c_uchar; 0usize];
}
unsafe extern "C" {
    pub static UTCollaborationOptionsTypeIdentifier: NSString;
}

unsafe impl objc2::encode::RefEncode for SWAction {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for SWAction {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for SWCollaborationCoordinator {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for SWCollaborationCoordinator {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for SWCollaborationOption {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for SWCollaborationOption {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for SWCollaborationOptionsGroup {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for SWCollaborationOptionsGroup {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for SWCollaborationOptionsPickerGroup {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for SWCollaborationOptionsPickerGroup {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for SWCollaborationShareOptions {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for SWCollaborationShareOptions {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for SWCollaborationMetadata {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for SWCollaborationMetadata {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for SWPerson {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for SWPerson {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for SWPersonIdentity {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for SWPersonIdentity {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for SWPersonIdentityProof {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for SWPersonIdentityProof {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for SWSignedPersonIdentityProof {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for SWSignedPersonIdentityProof {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for SWStartCollaborationAction {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for SWStartCollaborationAction {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for SWUpdateCollaborationParticipantsAction {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for SWUpdateCollaborationParticipantsAction {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
