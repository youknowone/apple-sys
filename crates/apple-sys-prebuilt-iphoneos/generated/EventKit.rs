#[allow(unused_imports)]
use crate::objc::*;
#[allow(unused_imports)]
use crate::AddressBook::*;
#[allow(unused_imports)]
use crate::CoreData::*;
#[allow(unused_imports)]
use crate::CoreGraphics::*;
#[allow(unused_imports)]
use crate::Foundation::*;
#[allow(unused_imports)]
use crate::MapKit::*;
#[allow(unused_imports)]
use crate::Security::*;

#[allow(unused_imports)]
use objc2::msg_send;
pub type EKAuthorizationStatus = NSInteger;
pub type EKWeekday = NSInteger;
pub type EKRecurrenceFrequency = NSInteger;
pub type EKParticipantType = NSInteger;
pub type EKParticipantRole = NSInteger;
pub type EKParticipantScheduleStatus = NSInteger;
pub type EKParticipantStatus = NSInteger;
pub type EKCalendarType = NSInteger;
pub type EKCalendarEventAvailabilityMask = NSUInteger;
pub type EKSourceType = NSInteger;
pub type EKEntityType = NSUInteger;
pub type EKEntityMask = NSUInteger;
pub type EKAlarmProximity = NSInteger;
pub type EKAlarmType = NSInteger;
pub type EKReminderPriority = NSUInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct EKObject(pub id);
impl std::ops::Deref for EKObject {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for EKObject {}
impl EKObject {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"EKObject").unwrap(), alloc) })
    }
}
impl INSObject for EKObject {}
impl PNSObject for EKObject {}
impl std::convert::TryFrom<NSObject> for EKObject {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<EKObject, Self::Error> {
        let is_kind_of: bool = unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"EKObject").unwrap()) };
        if is_kind_of {
            Ok(EKObject(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to EKObject")
        }
    }
}
impl IEKObject for EKObject {}
pub trait IEKObject: Sized + std::ops::Deref {
    unsafe fn reset(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, reset)
    }
    unsafe fn rollback(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, rollback)
    }
    unsafe fn refresh(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, refresh)
    }
    unsafe fn hasChanges(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, hasChanges)
    }
    unsafe fn isNew(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isNew)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct EKAlarm(pub id);
impl std::ops::Deref for EKAlarm {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for EKAlarm {}
impl EKAlarm {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"EKAlarm").unwrap(), alloc) })
    }
}
impl PNSCopying for EKAlarm {}
impl IEKObject for EKAlarm {}
impl From<EKAlarm> for EKObject {
    fn from(child: EKAlarm) -> EKObject {
        EKObject(child.0)
    }
}
impl std::convert::TryFrom<EKObject> for EKAlarm {
    type Error = &'static str;
    fn try_from(parent: EKObject) -> Result<EKAlarm, Self::Error> {
        let is_kind_of: bool = unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"EKAlarm").unwrap()) };
        if is_kind_of {
            Ok(EKAlarm(parent.0))
        } else {
            Err("This EKObject cannot be downcasted to EKAlarm")
        }
    }
}
impl INSObject for EKAlarm {}
impl PNSObject for EKAlarm {}
impl IEKAlarm for EKAlarm {}
pub trait IEKAlarm: Sized + std::ops::Deref {
    unsafe fn relativeOffset(&self) -> NSTimeInterval
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, relativeOffset)
    }
    unsafe fn setRelativeOffset_(&self, relativeOffset: NSTimeInterval)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRelativeOffset : relativeOffset)
    }
    unsafe fn absoluteDate(&self) -> NSDate
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, absoluteDate)
    }
    unsafe fn setAbsoluteDate_(&self, absoluteDate: NSDate)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAbsoluteDate : absoluteDate)
    }
    unsafe fn structuredLocation(&self) -> EKStructuredLocation
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, structuredLocation)
    }
    unsafe fn setStructuredLocation_(&self, structuredLocation: EKStructuredLocation)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setStructuredLocation : structuredLocation)
    }
    unsafe fn proximity(&self) -> EKAlarmProximity
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, proximity)
    }
    unsafe fn setProximity_(&self, proximity: EKAlarmProximity)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setProximity : proximity)
    }
    unsafe fn type_(&self) -> EKAlarmType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, type)
    }
    unsafe fn emailAddress(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, emailAddress)
    }
    unsafe fn setEmailAddress_(&self, emailAddress: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setEmailAddress : emailAddress)
    }
    unsafe fn soundName(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, soundName)
    }
    unsafe fn setSoundName_(&self, soundName: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSoundName : soundName)
    }
    unsafe fn url(&self) -> NSURL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, url)
    }
    unsafe fn setUrl_(&self, url: NSURL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setUrl : url)
    }
    unsafe fn alarmWithAbsoluteDate_(date: NSDate) -> EKAlarm
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"EKAlarm").unwrap(), alarmWithAbsoluteDate : date)
    }
    unsafe fn alarmWithRelativeOffset_(offset: NSTimeInterval) -> EKAlarm
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"EKAlarm").unwrap(), alarmWithRelativeOffset : offset)
    }
}
pub type EKSpan = NSInteger;
pub type EKEventSearchCallback = *mut ::std::os::raw::c_void;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct EKEventStore(pub id);
impl std::ops::Deref for EKEventStore {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for EKEventStore {}
impl EKEventStore {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"EKEventStore").unwrap(), alloc) })
    }
}
impl INSObject for EKEventStore {}
impl PNSObject for EKEventStore {}
impl std::convert::TryFrom<NSObject> for EKEventStore {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<EKEventStore, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"EKEventStore").unwrap()) };
        if is_kind_of {
            Ok(EKEventStore(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to EKEventStore")
        }
    }
}
impl IEKEventStore for EKEventStore {}
pub trait IEKEventStore: Sized + std::ops::Deref {
    unsafe fn initWithAccessToEntityTypes_(&self, entityTypes: EKEntityMask) -> id
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithAccessToEntityTypes : entityTypes)
    }
    unsafe fn init(&self) -> id
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn initWithSources_(&self, sources: NSArray) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithSources : sources)
    }
    unsafe fn requestFullAccessToEventsWithCompletion_(
        &self,
        completion: EKEventStoreRequestAccessCompletionHandler,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, requestFullAccessToEventsWithCompletion : completion)
    }
    unsafe fn requestWriteOnlyAccessToEventsWithCompletion_(
        &self,
        completion: EKEventStoreRequestAccessCompletionHandler,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, requestWriteOnlyAccessToEventsWithCompletion : completion)
    }
    unsafe fn requestFullAccessToRemindersWithCompletion_(
        &self,
        completion: EKEventStoreRequestAccessCompletionHandler,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, requestFullAccessToRemindersWithCompletion : completion)
    }
    unsafe fn requestAccessToEntityType_completion_(
        &self,
        entityType: EKEntityType,
        completion: EKEventStoreRequestAccessCompletionHandler,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, requestAccessToEntityType : entityType, completion : completion)
    }
    unsafe fn sourceWithIdentifier_(&self, identifier: NSString) -> EKSource
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, sourceWithIdentifier : identifier)
    }
    unsafe fn calendarsForEntityType_(&self, entityType: EKEntityType) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, calendarsForEntityType : entityType)
    }
    unsafe fn defaultCalendarForNewReminders(&self) -> EKCalendar
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, defaultCalendarForNewReminders)
    }
    unsafe fn calendarWithIdentifier_(&self, identifier: NSString) -> EKCalendar
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, calendarWithIdentifier : identifier)
    }
    unsafe fn saveCalendar_commit_error_(
        &self,
        calendar: EKCalendar,
        commit: BOOL,
        error: *mut NSError,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, saveCalendar : calendar, commit : commit, error : error)
    }
    unsafe fn removeCalendar_commit_error_(
        &self,
        calendar: EKCalendar,
        commit: BOOL,
        error: *mut NSError,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, removeCalendar : calendar, commit : commit, error : error)
    }
    unsafe fn calendarItemWithIdentifier_(&self, identifier: NSString) -> EKCalendarItem
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, calendarItemWithIdentifier : identifier)
    }
    unsafe fn calendarItemsWithExternalIdentifier_(&self, externalIdentifier: NSString) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, calendarItemsWithExternalIdentifier : externalIdentifier)
    }
    unsafe fn saveEvent_span_error_(
        &self,
        event: EKEvent,
        span: EKSpan,
        error: *mut NSError,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, saveEvent : event, span : span, error : error)
    }
    unsafe fn removeEvent_span_error_(
        &self,
        event: EKEvent,
        span: EKSpan,
        error: *mut NSError,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, removeEvent : event, span : span, error : error)
    }
    unsafe fn saveEvent_span_commit_error_(
        &self,
        event: EKEvent,
        span: EKSpan,
        commit: BOOL,
        error: *mut NSError,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, saveEvent : event, span : span, commit : commit, error : error)
    }
    unsafe fn removeEvent_span_commit_error_(
        &self,
        event: EKEvent,
        span: EKSpan,
        commit: BOOL,
        error: *mut NSError,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, removeEvent : event, span : span, commit : commit, error : error)
    }
    unsafe fn eventWithIdentifier_(&self, identifier: NSString) -> EKEvent
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, eventWithIdentifier : identifier)
    }
    unsafe fn eventsMatchingPredicate_(&self, predicate: NSPredicate) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, eventsMatchingPredicate : predicate)
    }
    unsafe fn enumerateEventsMatchingPredicate_usingBlock_(
        &self,
        predicate: NSPredicate,
        block: EKEventSearchCallback,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, enumerateEventsMatchingPredicate : predicate, usingBlock : block)
    }
    unsafe fn predicateForEventsWithStartDate_endDate_calendars_(
        &self,
        startDate: NSDate,
        endDate: NSDate,
        calendars: NSArray,
    ) -> NSPredicate
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, predicateForEventsWithStartDate : startDate, endDate : endDate, calendars : calendars)
    }
    unsafe fn saveReminder_commit_error_(
        &self,
        reminder: EKReminder,
        commit: BOOL,
        error: *mut NSError,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, saveReminder : reminder, commit : commit, error : error)
    }
    unsafe fn removeReminder_commit_error_(
        &self,
        reminder: EKReminder,
        commit: BOOL,
        error: *mut NSError,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, removeReminder : reminder, commit : commit, error : error)
    }
    unsafe fn fetchRemindersMatchingPredicate_completion_(
        &self,
        predicate: NSPredicate,
        completion: *mut ::std::os::raw::c_void,
    ) -> id
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, fetchRemindersMatchingPredicate : predicate, completion : completion)
    }
    unsafe fn cancelFetchRequest_(&self, fetchIdentifier: id)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, cancelFetchRequest : fetchIdentifier)
    }
    unsafe fn predicateForRemindersInCalendars_(&self, calendars: NSArray) -> NSPredicate
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, predicateForRemindersInCalendars : calendars)
    }
    unsafe fn predicateForIncompleteRemindersWithDueDateStarting_ending_calendars_(
        &self,
        startDate: NSDate,
        endDate: NSDate,
        calendars: NSArray,
    ) -> NSPredicate
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, predicateForIncompleteRemindersWithDueDateStarting : startDate, ending : endDate, calendars : calendars)
    }
    unsafe fn predicateForCompletedRemindersWithCompletionDateStarting_ending_calendars_(
        &self,
        startDate: NSDate,
        endDate: NSDate,
        calendars: NSArray,
    ) -> NSPredicate
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, predicateForCompletedRemindersWithCompletionDateStarting : startDate, ending : endDate, calendars : calendars)
    }
    unsafe fn commit_(&self, error: *mut NSError) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, commit : error)
    }
    unsafe fn reset(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, reset)
    }
    unsafe fn refreshSourcesIfNecessary(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, refreshSourcesIfNecessary)
    }
    unsafe fn eventStoreIdentifier(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, eventStoreIdentifier)
    }
    unsafe fn delegateSources(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, delegateSources)
    }
    unsafe fn sources(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, sources)
    }
    unsafe fn calendars(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, calendars)
    }
    unsafe fn defaultCalendarForNewEvents(&self) -> EKCalendar
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, defaultCalendarForNewEvents)
    }
    unsafe fn authorizationStatusForEntityType_(entityType: EKEntityType) -> EKAuthorizationStatus
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"EKEventStore").unwrap(), authorizationStatusForEntityType : entityType)
    }
}
pub type EKEventStoreRequestAccessCompletionHandler = *mut ::std::os::raw::c_void;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct EKCalendar(pub id);
impl std::ops::Deref for EKCalendar {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for EKCalendar {}
impl EKCalendar {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"EKCalendar").unwrap(), alloc) })
    }
}
impl IEKObject for EKCalendar {}
impl std::convert::TryFrom<EKObject> for EKCalendar {
    type Error = &'static str;
    fn try_from(parent: EKObject) -> Result<EKCalendar, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"EKCalendar").unwrap()) };
        if is_kind_of {
            Ok(EKCalendar(parent.0))
        } else {
            Err("This EKObject cannot be downcasted to EKCalendar")
        }
    }
}
impl INSObject for EKCalendar {}
impl PNSObject for EKCalendar {}
impl IEKCalendar for EKCalendar {}
pub trait IEKCalendar: Sized + std::ops::Deref {
    unsafe fn source(&self) -> EKSource
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, source)
    }
    unsafe fn setSource_(&self, source: EKSource)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSource : source)
    }
    unsafe fn calendarIdentifier(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, calendarIdentifier)
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
    unsafe fn type_(&self) -> EKCalendarType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, type)
    }
    unsafe fn allowsContentModifications(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, allowsContentModifications)
    }
    unsafe fn isSubscribed(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isSubscribed)
    }
    unsafe fn isImmutable(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isImmutable)
    }
    unsafe fn CGColor(&self) -> CGColorRef
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, CGColor)
    }
    unsafe fn setCGColor_(&self, CGColor: CGColorRef)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCGColor : CGColor)
    }
    unsafe fn color(&self) -> NSColor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, color)
    }
    unsafe fn setColor_(&self, color: NSColor)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setColor : color)
    }
    unsafe fn supportedEventAvailabilities(&self) -> EKCalendarEventAvailabilityMask
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, supportedEventAvailabilities)
    }
    unsafe fn allowedEntityTypes(&self) -> EKEntityMask
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, allowedEntityTypes)
    }
    unsafe fn calendarWithEventStore_(eventStore: EKEventStore) -> EKCalendar
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"EKCalendar").unwrap(), calendarWithEventStore : eventStore)
    }
    unsafe fn calendarForEntityType_eventStore_(
        entityType: EKEntityType,
        eventStore: EKEventStore,
    ) -> EKCalendar
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"EKCalendar").unwrap(), calendarForEntityType : entityType, eventStore : eventStore)
    }
}
pub type EKErrorCode = NSInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct EKCalendarItem(pub id);
impl std::ops::Deref for EKCalendarItem {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for EKCalendarItem {}
impl EKCalendarItem {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"EKCalendarItem").unwrap(), alloc) })
    }
}
impl IEKObject for EKCalendarItem {}
impl std::convert::TryFrom<EKObject> for EKCalendarItem {
    type Error = &'static str;
    fn try_from(parent: EKObject) -> Result<EKCalendarItem, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"EKCalendarItem").unwrap()) };
        if is_kind_of {
            Ok(EKCalendarItem(parent.0))
        } else {
            Err("This EKObject cannot be downcasted to EKCalendarItem")
        }
    }
}
impl INSObject for EKCalendarItem {}
impl PNSObject for EKCalendarItem {}
impl IEKCalendarItem for EKCalendarItem {}
pub trait IEKCalendarItem: Sized + std::ops::Deref {
    unsafe fn addAlarm_(&self, alarm: EKAlarm)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, addAlarm : alarm)
    }
    unsafe fn removeAlarm_(&self, alarm: EKAlarm)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, removeAlarm : alarm)
    }
    unsafe fn addRecurrenceRule_(&self, rule: EKRecurrenceRule)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, addRecurrenceRule : rule)
    }
    unsafe fn removeRecurrenceRule_(&self, rule: EKRecurrenceRule)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, removeRecurrenceRule : rule)
    }
    unsafe fn UUID(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, UUID)
    }
    unsafe fn calendar(&self) -> EKCalendar
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, calendar)
    }
    unsafe fn setCalendar_(&self, calendar: EKCalendar)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCalendar : calendar)
    }
    unsafe fn calendarItemIdentifier(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, calendarItemIdentifier)
    }
    unsafe fn calendarItemExternalIdentifier(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, calendarItemExternalIdentifier)
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
    unsafe fn location(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, location)
    }
    unsafe fn setLocation_(&self, location: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setLocation : location)
    }
    unsafe fn notes(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, notes)
    }
    unsafe fn setNotes_(&self, notes: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setNotes : notes)
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
    unsafe fn lastModifiedDate(&self) -> NSDate
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, lastModifiedDate)
    }
    unsafe fn creationDate(&self) -> NSDate
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, creationDate)
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
    unsafe fn hasAlarms(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, hasAlarms)
    }
    unsafe fn hasRecurrenceRules(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, hasRecurrenceRules)
    }
    unsafe fn hasAttendees(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, hasAttendees)
    }
    unsafe fn hasNotes(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, hasNotes)
    }
    unsafe fn attendees(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, attendees)
    }
    unsafe fn alarms(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, alarms)
    }
    unsafe fn setAlarms_(&self, alarms: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAlarms : alarms)
    }
    unsafe fn recurrenceRules(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, recurrenceRules)
    }
    unsafe fn setRecurrenceRules_(&self, recurrenceRules: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRecurrenceRules : recurrenceRules)
    }
}
pub type EKEventAvailability = NSInteger;
pub type EKEventStatus = NSInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct EKEvent(pub id);
impl std::ops::Deref for EKEvent {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for EKEvent {}
impl EKEvent {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"EKEvent").unwrap(), alloc) })
    }
}
impl IEKCalendarItem for EKEvent {}
impl From<EKEvent> for EKCalendarItem {
    fn from(child: EKEvent) -> EKCalendarItem {
        EKCalendarItem(child.0)
    }
}
impl std::convert::TryFrom<EKCalendarItem> for EKEvent {
    type Error = &'static str;
    fn try_from(parent: EKCalendarItem) -> Result<EKEvent, Self::Error> {
        let is_kind_of: bool = unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"EKEvent").unwrap()) };
        if is_kind_of {
            Ok(EKEvent(parent.0))
        } else {
            Err("This EKCalendarItem cannot be downcasted to EKEvent")
        }
    }
}
impl IEKObject for EKEvent {}
impl INSObject for EKEvent {}
impl PNSObject for EKEvent {}
impl IEKEvent for EKEvent {}
pub trait IEKEvent: Sized + std::ops::Deref {
    unsafe fn compareStartDateWithEvent_(&self, other: EKEvent) -> NSComparisonResult
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, compareStartDateWithEvent : other)
    }
    unsafe fn refresh(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, refresh)
    }
    unsafe fn eventIdentifier(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, eventIdentifier)
    }
    unsafe fn isAllDay(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isAllDay)
    }
    unsafe fn setAllDay_(&self, allDay: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAllDay : allDay)
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
    unsafe fn structuredLocation(&self) -> EKStructuredLocation
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, structuredLocation)
    }
    unsafe fn setStructuredLocation_(&self, structuredLocation: EKStructuredLocation)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setStructuredLocation : structuredLocation)
    }
    unsafe fn organizer(&self) -> EKParticipant
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, organizer)
    }
    unsafe fn availability(&self) -> EKEventAvailability
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, availability)
    }
    unsafe fn setAvailability_(&self, availability: EKEventAvailability)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAvailability : availability)
    }
    unsafe fn status(&self) -> EKEventStatus
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, status)
    }
    unsafe fn isDetached(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isDetached)
    }
    unsafe fn occurrenceDate(&self) -> NSDate
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, occurrenceDate)
    }
    unsafe fn birthdayContactIdentifier(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, birthdayContactIdentifier)
    }
    unsafe fn birthdayPersonID(&self) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, birthdayPersonID)
    }
    unsafe fn birthdayPersonUniqueID(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, birthdayPersonUniqueID)
    }
    unsafe fn eventWithEventStore_(eventStore: EKEventStore) -> EKEvent
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"EKEvent").unwrap(), eventWithEventStore : eventStore)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct EKParticipant(pub id);
impl std::ops::Deref for EKParticipant {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for EKParticipant {}
impl EKParticipant {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"EKParticipant").unwrap(), alloc) })
    }
}
impl PNSCopying for EKParticipant {}
impl IEKObject for EKParticipant {}
impl std::convert::TryFrom<EKObject> for EKParticipant {
    type Error = &'static str;
    fn try_from(parent: EKObject) -> Result<EKParticipant, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"EKParticipant").unwrap()) };
        if is_kind_of {
            Ok(EKParticipant(parent.0))
        } else {
            Err("This EKObject cannot be downcasted to EKParticipant")
        }
    }
}
impl INSObject for EKParticipant {}
impl PNSObject for EKParticipant {}
impl IEKParticipant for EKParticipant {}
pub trait IEKParticipant: Sized + std::ops::Deref {
    unsafe fn ABPersonInAddressBook_(&self, addressBook: ABAddressBook) -> ABPerson
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, ABPersonInAddressBook : addressBook)
    }
    unsafe fn URL(&self) -> NSURL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, URL)
    }
    unsafe fn name(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, name)
    }
    unsafe fn participantStatus(&self) -> EKParticipantStatus
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, participantStatus)
    }
    unsafe fn participantRole(&self) -> EKParticipantRole
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, participantRole)
    }
    unsafe fn participantType(&self) -> EKParticipantType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, participantType)
    }
    unsafe fn isCurrentUser(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isCurrentUser)
    }
    unsafe fn contactPredicate(&self) -> NSPredicate
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, contactPredicate)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct EKRecurrenceEnd(pub id);
impl std::ops::Deref for EKRecurrenceEnd {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for EKRecurrenceEnd {}
impl EKRecurrenceEnd {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"EKRecurrenceEnd").unwrap(), alloc) })
    }
}
impl PNSCopying for EKRecurrenceEnd {}
impl PNSSecureCoding for EKRecurrenceEnd {}
impl INSObject for EKRecurrenceEnd {}
impl PNSObject for EKRecurrenceEnd {}
impl std::convert::TryFrom<NSObject> for EKRecurrenceEnd {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<EKRecurrenceEnd, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"EKRecurrenceEnd").unwrap()) };
        if is_kind_of {
            Ok(EKRecurrenceEnd(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to EKRecurrenceEnd")
        }
    }
}
impl IEKRecurrenceEnd for EKRecurrenceEnd {}
pub trait IEKRecurrenceEnd: Sized + std::ops::Deref {
    unsafe fn endDate(&self) -> NSDate
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, endDate)
    }
    unsafe fn occurrenceCount(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, occurrenceCount)
    }
    unsafe fn recurrenceEndWithEndDate_(endDate: NSDate) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"EKRecurrenceEnd").unwrap(), recurrenceEndWithEndDate : endDate)
    }
    unsafe fn recurrenceEndWithOccurrenceCount_(occurrenceCount: NSUInteger) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"EKRecurrenceEnd").unwrap(), recurrenceEndWithOccurrenceCount : occurrenceCount)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct EKRecurrenceDayOfWeek(pub id);
impl std::ops::Deref for EKRecurrenceDayOfWeek {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for EKRecurrenceDayOfWeek {}
impl EKRecurrenceDayOfWeek {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"EKRecurrenceDayOfWeek").unwrap(), alloc) })
    }
}
impl PNSCopying for EKRecurrenceDayOfWeek {}
impl PNSSecureCoding for EKRecurrenceDayOfWeek {}
impl INSObject for EKRecurrenceDayOfWeek {}
impl PNSObject for EKRecurrenceDayOfWeek {}
impl std::convert::TryFrom<NSObject> for EKRecurrenceDayOfWeek {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<EKRecurrenceDayOfWeek, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"EKRecurrenceDayOfWeek").unwrap()) };
        if is_kind_of {
            Ok(EKRecurrenceDayOfWeek(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to EKRecurrenceDayOfWeek")
        }
    }
}
impl IEKRecurrenceDayOfWeek for EKRecurrenceDayOfWeek {}
pub trait IEKRecurrenceDayOfWeek: Sized + std::ops::Deref {
    unsafe fn initWithDayOfTheWeek_weekNumber_(
        &self,
        dayOfTheWeek: EKWeekday,
        weekNumber: NSInteger,
    ) -> id
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithDayOfTheWeek : dayOfTheWeek, weekNumber : weekNumber)
    }
    unsafe fn dayOfTheWeek(&self) -> EKWeekday
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, dayOfTheWeek)
    }
    unsafe fn weekNumber(&self) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, weekNumber)
    }
    unsafe fn dayOfWeek_(dayOfTheWeek: EKWeekday) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"EKRecurrenceDayOfWeek").unwrap(), dayOfWeek : dayOfTheWeek)
    }
    unsafe fn dayOfWeek_weekNumber_(dayOfTheWeek: EKWeekday, weekNumber: NSInteger) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"EKRecurrenceDayOfWeek").unwrap(), dayOfWeek : dayOfTheWeek, weekNumber : weekNumber)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct EKRecurrenceRule(pub id);
impl std::ops::Deref for EKRecurrenceRule {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for EKRecurrenceRule {}
impl EKRecurrenceRule {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"EKRecurrenceRule").unwrap(), alloc) })
    }
}
impl PNSCopying for EKRecurrenceRule {}
impl IEKObject for EKRecurrenceRule {}
impl std::convert::TryFrom<EKObject> for EKRecurrenceRule {
    type Error = &'static str;
    fn try_from(parent: EKObject) -> Result<EKRecurrenceRule, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"EKRecurrenceRule").unwrap()) };
        if is_kind_of {
            Ok(EKRecurrenceRule(parent.0))
        } else {
            Err("This EKObject cannot be downcasted to EKRecurrenceRule")
        }
    }
}
impl INSObject for EKRecurrenceRule {}
impl PNSObject for EKRecurrenceRule {}
impl IEKRecurrenceRule for EKRecurrenceRule {}
pub trait IEKRecurrenceRule: Sized + std::ops::Deref {
    unsafe fn initRecurrenceWithFrequency_interval_end_(
        &self,
        type_: EKRecurrenceFrequency,
        interval: NSInteger,
        end: EKRecurrenceEnd,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initRecurrenceWithFrequency : type_, interval : interval, end : end)
    }
    unsafe fn initRecurrenceWithFrequency_interval_daysOfTheWeek_daysOfTheMonth_monthsOfTheYear_weeksOfTheYear_daysOfTheYear_setPositions_end_(
        &self,
        type_: EKRecurrenceFrequency,
        interval: NSInteger,
        days: NSArray,
        monthDays: NSArray,
        months: NSArray,
        weeksOfTheYear: NSArray,
        daysOfTheYear: NSArray,
        setPositions: NSArray,
        end: EKRecurrenceEnd,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initRecurrenceWithFrequency : type_, interval : interval, daysOfTheWeek : days, daysOfTheMonth : monthDays, monthsOfTheYear : months, weeksOfTheYear : weeksOfTheYear, daysOfTheYear : daysOfTheYear, setPositions : setPositions, end : end)
    }
    unsafe fn calendarIdentifier(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, calendarIdentifier)
    }
    unsafe fn recurrenceEnd(&self) -> EKRecurrenceEnd
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, recurrenceEnd)
    }
    unsafe fn setRecurrenceEnd_(&self, recurrenceEnd: EKRecurrenceEnd)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRecurrenceEnd : recurrenceEnd)
    }
    unsafe fn frequency(&self) -> EKRecurrenceFrequency
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, frequency)
    }
    unsafe fn interval(&self) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, interval)
    }
    unsafe fn firstDayOfTheWeek(&self) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, firstDayOfTheWeek)
    }
    unsafe fn daysOfTheWeek(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, daysOfTheWeek)
    }
    unsafe fn daysOfTheMonth(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, daysOfTheMonth)
    }
    unsafe fn daysOfTheYear(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, daysOfTheYear)
    }
    unsafe fn weeksOfTheYear(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, weeksOfTheYear)
    }
    unsafe fn monthsOfTheYear(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, monthsOfTheYear)
    }
    unsafe fn setPositions(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, setPositions)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct EKReminder(pub id);
impl std::ops::Deref for EKReminder {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for EKReminder {}
impl EKReminder {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"EKReminder").unwrap(), alloc) })
    }
}
impl IEKCalendarItem for EKReminder {}
impl std::convert::TryFrom<EKCalendarItem> for EKReminder {
    type Error = &'static str;
    fn try_from(parent: EKCalendarItem) -> Result<EKReminder, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"EKReminder").unwrap()) };
        if is_kind_of {
            Ok(EKReminder(parent.0))
        } else {
            Err("This EKCalendarItem cannot be downcasted to EKReminder")
        }
    }
}
impl IEKObject for EKReminder {}
impl INSObject for EKReminder {}
impl PNSObject for EKReminder {}
impl IEKReminder for EKReminder {}
pub trait IEKReminder: Sized + std::ops::Deref {
    unsafe fn startDateComponents(&self) -> NSDateComponents
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, startDateComponents)
    }
    unsafe fn setStartDateComponents_(&self, startDateComponents: NSDateComponents)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setStartDateComponents : startDateComponents)
    }
    unsafe fn dueDateComponents(&self) -> NSDateComponents
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, dueDateComponents)
    }
    unsafe fn setDueDateComponents_(&self, dueDateComponents: NSDateComponents)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDueDateComponents : dueDateComponents)
    }
    unsafe fn isCompleted(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isCompleted)
    }
    unsafe fn setCompleted_(&self, completed: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCompleted : completed)
    }
    unsafe fn completionDate(&self) -> NSDate
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, completionDate)
    }
    unsafe fn setCompletionDate_(&self, completionDate: NSDate)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCompletionDate : completionDate)
    }
    unsafe fn priority(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, priority)
    }
    unsafe fn setPriority_(&self, priority: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPriority : priority)
    }
    unsafe fn reminderWithEventStore_(eventStore: EKEventStore) -> EKReminder
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"EKReminder").unwrap(), reminderWithEventStore : eventStore)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct EKSource(pub id);
impl std::ops::Deref for EKSource {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for EKSource {}
impl EKSource {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"EKSource").unwrap(), alloc) })
    }
}
impl IEKObject for EKSource {}
impl std::convert::TryFrom<EKObject> for EKSource {
    type Error = &'static str;
    fn try_from(parent: EKObject) -> Result<EKSource, Self::Error> {
        let is_kind_of: bool = unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"EKSource").unwrap()) };
        if is_kind_of {
            Ok(EKSource(parent.0))
        } else {
            Err("This EKObject cannot be downcasted to EKSource")
        }
    }
}
impl INSObject for EKSource {}
impl PNSObject for EKSource {}
impl IEKSource for EKSource {}
pub trait IEKSource: Sized + std::ops::Deref {
    unsafe fn calendarsForEntityType_(&self, entityType: EKEntityType) -> NSSet
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, calendarsForEntityType : entityType)
    }
    unsafe fn sourceIdentifier(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, sourceIdentifier)
    }
    unsafe fn sourceType(&self) -> EKSourceType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, sourceType)
    }
    unsafe fn title(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, title)
    }
    unsafe fn calendars(&self) -> NSSet
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, calendars)
    }
    unsafe fn isDelegate(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isDelegate)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct EKStructuredLocation(pub id);
impl std::ops::Deref for EKStructuredLocation {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for EKStructuredLocation {}
impl EKStructuredLocation {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"EKStructuredLocation").unwrap(), alloc) })
    }
}
impl PNSCopying for EKStructuredLocation {}
impl IEKObject for EKStructuredLocation {}
impl std::convert::TryFrom<EKObject> for EKStructuredLocation {
    type Error = &'static str;
    fn try_from(parent: EKObject) -> Result<EKStructuredLocation, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"EKStructuredLocation").unwrap()) };
        if is_kind_of {
            Ok(EKStructuredLocation(parent.0))
        } else {
            Err("This EKObject cannot be downcasted to EKStructuredLocation")
        }
    }
}
impl INSObject for EKStructuredLocation {}
impl PNSObject for EKStructuredLocation {}
impl IEKStructuredLocation for EKStructuredLocation {}
pub trait IEKStructuredLocation: Sized + std::ops::Deref {
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
    unsafe fn geoLocation(&self) -> CLLocation
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, geoLocation)
    }
    unsafe fn setGeoLocation_(&self, geoLocation: CLLocation)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setGeoLocation : geoLocation)
    }
    unsafe fn radius(&self) -> f64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, radius)
    }
    unsafe fn setRadius_(&self, radius: f64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRadius : radius)
    }
    unsafe fn locationWithTitle_(title: NSString) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"EKStructuredLocation").unwrap(), locationWithTitle : title)
    }
    unsafe fn locationWithMapItem_(mapItem: MKMapItem) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"EKStructuredLocation").unwrap(), locationWithMapItem : mapItem)
    }
}
pub type EKVirtualConferenceRoomTypeIdentifier = NSString;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct EKVirtualConferenceRoomTypeDescriptor(pub id);
impl std::ops::Deref for EKVirtualConferenceRoomTypeDescriptor {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for EKVirtualConferenceRoomTypeDescriptor {}
impl EKVirtualConferenceRoomTypeDescriptor {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"EKVirtualConferenceRoomTypeDescriptor").unwrap(), alloc) })
    }
}
impl INSObject for EKVirtualConferenceRoomTypeDescriptor {}
impl PNSObject for EKVirtualConferenceRoomTypeDescriptor {}
impl std::convert::TryFrom<NSObject> for EKVirtualConferenceRoomTypeDescriptor {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<EKVirtualConferenceRoomTypeDescriptor, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"EKVirtualConferenceRoomTypeDescriptor").unwrap())
        };
        if is_kind_of {
            Ok(EKVirtualConferenceRoomTypeDescriptor(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to EKVirtualConferenceRoomTypeDescriptor")
        }
    }
}
impl IEKVirtualConferenceRoomTypeDescriptor for EKVirtualConferenceRoomTypeDescriptor {}
pub trait IEKVirtualConferenceRoomTypeDescriptor: Sized + std::ops::Deref {
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
    unsafe fn identifier(&self) -> EKVirtualConferenceRoomTypeIdentifier
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, identifier)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"EKVirtualConferenceRoomTypeDescriptor").unwrap(), new)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct EKVirtualConferenceURLDescriptor(pub id);
impl std::ops::Deref for EKVirtualConferenceURLDescriptor {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for EKVirtualConferenceURLDescriptor {}
impl EKVirtualConferenceURLDescriptor {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"EKVirtualConferenceURLDescriptor").unwrap(), alloc) })
    }
}
impl INSObject for EKVirtualConferenceURLDescriptor {}
impl PNSObject for EKVirtualConferenceURLDescriptor {}
impl std::convert::TryFrom<NSObject> for EKVirtualConferenceURLDescriptor {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<EKVirtualConferenceURLDescriptor, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"EKVirtualConferenceURLDescriptor").unwrap())
        };
        if is_kind_of {
            Ok(EKVirtualConferenceURLDescriptor(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to EKVirtualConferenceURLDescriptor")
        }
    }
}
impl IEKVirtualConferenceURLDescriptor for EKVirtualConferenceURLDescriptor {}
pub trait IEKVirtualConferenceURLDescriptor: Sized + std::ops::Deref {
    unsafe fn initWithTitle_URL_(&self, title: NSString, URL: NSURL) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithTitle : title, URL : URL)
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
        msg_send!(&*objc2::runtime::AnyClass::get(c"EKVirtualConferenceURLDescriptor").unwrap(), new)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct EKVirtualConferenceDescriptor(pub id);
impl std::ops::Deref for EKVirtualConferenceDescriptor {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for EKVirtualConferenceDescriptor {}
impl EKVirtualConferenceDescriptor {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"EKVirtualConferenceDescriptor").unwrap(), alloc) })
    }
}
impl INSObject for EKVirtualConferenceDescriptor {}
impl PNSObject for EKVirtualConferenceDescriptor {}
impl std::convert::TryFrom<NSObject> for EKVirtualConferenceDescriptor {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<EKVirtualConferenceDescriptor, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"EKVirtualConferenceDescriptor").unwrap())
        };
        if is_kind_of {
            Ok(EKVirtualConferenceDescriptor(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to EKVirtualConferenceDescriptor")
        }
    }
}
impl IEKVirtualConferenceDescriptor for EKVirtualConferenceDescriptor {}
pub trait IEKVirtualConferenceDescriptor: Sized + std::ops::Deref {
    unsafe fn initWithTitle_URLDescriptors_conferenceDetails_(
        &self,
        title: NSString,
        URLDescriptors: NSArray,
        conferenceDetails: NSString,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithTitle : title, URLDescriptors : URLDescriptors, conferenceDetails : conferenceDetails)
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
    unsafe fn URLDescriptors(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, URLDescriptors)
    }
    unsafe fn conferenceDetails(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, conferenceDetails)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"EKVirtualConferenceDescriptor").unwrap(), new)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct EKVirtualConferenceProvider(pub id);
impl std::ops::Deref for EKVirtualConferenceProvider {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for EKVirtualConferenceProvider {}
impl EKVirtualConferenceProvider {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"EKVirtualConferenceProvider").unwrap(), alloc) })
    }
}
impl PNSExtensionRequestHandling for EKVirtualConferenceProvider {}
impl INSObject for EKVirtualConferenceProvider {}
impl PNSObject for EKVirtualConferenceProvider {}
impl std::convert::TryFrom<NSObject> for EKVirtualConferenceProvider {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<EKVirtualConferenceProvider, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"EKVirtualConferenceProvider").unwrap()) };
        if is_kind_of {
            Ok(EKVirtualConferenceProvider(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to EKVirtualConferenceProvider")
        }
    }
}
impl IEKVirtualConferenceProvider for EKVirtualConferenceProvider {}
pub trait IEKVirtualConferenceProvider: Sized + std::ops::Deref {
    unsafe fn fetchAvailableRoomTypesWithCompletionHandler_(
        &self,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, fetchAvailableRoomTypesWithCompletionHandler : completionHandler)
    }
    unsafe fn fetchVirtualConferenceForIdentifier_completionHandler_(
        &self,
        identifier: NSString,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, fetchVirtualConferenceForIdentifier : identifier, completionHandler : completionHandler)
    }
}
unsafe extern "C" {
    pub static EKEventStoreChangedNotification: NSString;
}
unsafe extern "C" {
    pub static EKErrorDomain: NSString;
}

unsafe impl objc2::encode::RefEncode for EKObject {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for EKObject {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for EKAlarm {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for EKAlarm {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for EKEventStore {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for EKEventStore {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for EKCalendar {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for EKCalendar {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for EKCalendarItem {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for EKCalendarItem {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for EKEvent {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for EKEvent {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for EKParticipant {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for EKParticipant {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for EKRecurrenceEnd {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for EKRecurrenceEnd {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for EKRecurrenceDayOfWeek {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for EKRecurrenceDayOfWeek {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for EKRecurrenceRule {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for EKRecurrenceRule {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for EKReminder {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for EKReminder {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for EKSource {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for EKSource {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for EKStructuredLocation {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for EKStructuredLocation {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for EKVirtualConferenceRoomTypeDescriptor {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for EKVirtualConferenceRoomTypeDescriptor {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for EKVirtualConferenceURLDescriptor {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for EKVirtualConferenceURLDescriptor {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for EKVirtualConferenceDescriptor {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for EKVirtualConferenceDescriptor {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for EKVirtualConferenceProvider {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for EKVirtualConferenceProvider {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
