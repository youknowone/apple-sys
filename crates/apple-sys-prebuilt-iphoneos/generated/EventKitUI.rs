#[allow(unused_imports)]
use crate::objc::*;
#[allow(unused_imports)]
use crate::EventKit::*;
#[allow(unused_imports)]
use crate::Foundation::*;

#[allow(unused_imports)]
use objc2::msg_send;
pub type EKEventEditViewAction = NSInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct EKEventEditViewController(pub id);
impl std::ops::Deref for EKEventEditViewController {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for EKEventEditViewController {}
impl EKEventEditViewController {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"EKEventEditViewController").unwrap(), alloc) })
    }
}
impl PNSCoding for EKEventEditViewController {}
impl std::convert::TryFrom<NSObject> for EKEventEditViewController {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<EKEventEditViewController, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"EKEventEditViewController").unwrap()) };
        if is_kind_of {
            Ok(EKEventEditViewController(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to EKEventEditViewController")
        }
    }
}
impl IEKEventEditViewController for EKEventEditViewController {}
pub trait IEKEventEditViewController: Sized + std::ops::Deref {
    unsafe fn cancelEditing(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, cancelEditing)
    }
    unsafe fn editViewDelegate(&self) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, editViewDelegate)
    }
    unsafe fn setEditViewDelegate_(&self, editViewDelegate: *mut u64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setEditViewDelegate : editViewDelegate)
    }
    unsafe fn eventStore(&self) -> EKEventStore
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, eventStore)
    }
    unsafe fn setEventStore_(&self, eventStore: EKEventStore)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setEventStore : eventStore)
    }
    unsafe fn event(&self) -> EKEvent
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, event)
    }
    unsafe fn setEvent_(&self, event: EKEvent)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setEvent : event)
    }
}
pub trait PEKEventEditViewDelegate: Sized + std::ops::Deref {
    unsafe fn eventEditViewController_didCompleteWithAction_(
        &self,
        controller: EKEventEditViewController,
        action: EKEventEditViewAction,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, eventEditViewController : controller, didCompleteWithAction : action)
    }
    unsafe fn eventEditViewControllerDefaultCalendarForNewEvents_(
        &self,
        controller: EKEventEditViewController,
    ) -> EKCalendar
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, eventEditViewControllerDefaultCalendarForNewEvents : controller)
    }
}
pub type EKEventViewAction = NSInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct EKEventViewController(pub id);
impl std::ops::Deref for EKEventViewController {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for EKEventViewController {}
impl EKEventViewController {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"EKEventViewController").unwrap(), alloc) })
    }
}
impl PNSCoding for EKEventViewController {}
impl std::convert::TryFrom<NSObject> for EKEventViewController {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<EKEventViewController, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"EKEventViewController").unwrap()) };
        if is_kind_of {
            Ok(EKEventViewController(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to EKEventViewController")
        }
    }
}
impl IEKEventViewController for EKEventViewController {}
pub trait IEKEventViewController: Sized + std::ops::Deref {
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
    unsafe fn event(&self) -> EKEvent
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, event)
    }
    unsafe fn setEvent_(&self, event: EKEvent)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setEvent : event)
    }
    unsafe fn allowsEditing(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, allowsEditing)
    }
    unsafe fn setAllowsEditing_(&self, allowsEditing: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAllowsEditing : allowsEditing)
    }
    unsafe fn allowsCalendarPreview(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, allowsCalendarPreview)
    }
    unsafe fn setAllowsCalendarPreview_(&self, allowsCalendarPreview: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAllowsCalendarPreview : allowsCalendarPreview)
    }
}
pub trait PEKEventViewDelegate: Sized + std::ops::Deref {
    unsafe fn eventViewController_didCompleteWithAction_(
        &self,
        controller: EKEventViewController,
        action: EKEventViewAction,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, eventViewController : controller, didCompleteWithAction : action)
    }
}
pub type EKCalendarChooserSelectionStyle = NSInteger;
pub type EKCalendarChooserDisplayStyle = NSInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct EKCalendarChooser(pub id);
impl std::ops::Deref for EKCalendarChooser {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for EKCalendarChooser {}
impl EKCalendarChooser {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"EKCalendarChooser").unwrap(), alloc) })
    }
}
impl PNSCoding for EKCalendarChooser {}
impl std::convert::TryFrom<NSObject> for EKCalendarChooser {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<EKCalendarChooser, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"EKCalendarChooser").unwrap()) };
        if is_kind_of {
            Ok(EKCalendarChooser(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to EKCalendarChooser")
        }
    }
}
impl IEKCalendarChooser for EKCalendarChooser {}
pub trait IEKCalendarChooser: Sized + std::ops::Deref {
    unsafe fn initWithSelectionStyle_displayStyle_eventStore_(
        &self,
        selectionStyle: EKCalendarChooserSelectionStyle,
        displayStyle: EKCalendarChooserDisplayStyle,
        eventStore: EKEventStore,
    ) -> id
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithSelectionStyle : selectionStyle, displayStyle : displayStyle, eventStore : eventStore)
    }
    unsafe fn initWithSelectionStyle_displayStyle_entityType_eventStore_(
        &self,
        style: EKCalendarChooserSelectionStyle,
        displayStyle: EKCalendarChooserDisplayStyle,
        entityType: EKEntityType,
        eventStore: EKEventStore,
    ) -> id
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithSelectionStyle : style, displayStyle : displayStyle, entityType : entityType, eventStore : eventStore)
    }
    unsafe fn selectionStyle(&self) -> EKCalendarChooserSelectionStyle
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, selectionStyle)
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
    unsafe fn showsDoneButton(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, showsDoneButton)
    }
    unsafe fn setShowsDoneButton_(&self, showsDoneButton: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setShowsDoneButton : showsDoneButton)
    }
    unsafe fn showsCancelButton(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, showsCancelButton)
    }
    unsafe fn setShowsCancelButton_(&self, showsCancelButton: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setShowsCancelButton : showsCancelButton)
    }
    unsafe fn selectedCalendars(&self) -> NSSet
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, selectedCalendars)
    }
    unsafe fn setSelectedCalendars_(&self, selectedCalendars: NSSet)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSelectedCalendars : selectedCalendars)
    }
}
pub trait PEKCalendarChooserDelegate: Sized + std::ops::Deref {
    unsafe fn calendarChooserSelectionDidChange_(&self, calendarChooser: EKCalendarChooser)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, calendarChooserSelectionDidChange : calendarChooser)
    }
    unsafe fn calendarChooserDidFinish_(&self, calendarChooser: EKCalendarChooser)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, calendarChooserDidFinish : calendarChooser)
    }
    unsafe fn calendarChooserDidCancel_(&self, calendarChooser: EKCalendarChooser)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, calendarChooserDidCancel : calendarChooser)
    }
}
pub trait PUIAppearanceContainer: Sized + std::ops::Deref {}
impl PUIAppearanceContainer for EKEventEditViewController {}
impl PUIAppearanceContainer for EKEventViewController {}
impl PUIAppearanceContainer for EKCalendarChooser {}
unsafe extern "C" {
    pub fn EventKitUIBundle() -> NSBundle;
}

unsafe impl objc2::encode::RefEncode for EKEventEditViewController {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for EKEventEditViewController {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for EKEventViewController {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for EKEventViewController {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for EKCalendarChooser {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for EKCalendarChooser {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
