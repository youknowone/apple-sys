#[allow(unused_imports)]
use crate::objc::*;
#[allow(unused_imports)]
use crate::AppKit::*;
#[allow(unused_imports)]
use crate::CoreFoundation::*;
#[allow(unused_imports)]
use crate::Foundation::*;

#[allow(unused_imports)]
use objc2::msg_send;
pub type CalSpan = ::std::os::raw::c_uint;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CalCalendarStore(pub id);
impl std::ops::Deref for CalCalendarStore {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CalCalendarStore {}
impl CalCalendarStore {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CalCalendarStore").unwrap(), alloc) })
    }
}
impl INSObject for CalCalendarStore {}
impl PNSObject for CalCalendarStore {}
impl std::convert::TryFrom<NSObject> for CalCalendarStore {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<CalCalendarStore, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CalCalendarStore").unwrap()) };
        if is_kind_of {
            Ok(CalCalendarStore(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to CalCalendarStore")
        }
    }
}
impl ICalCalendarStore for CalCalendarStore {}
pub trait ICalCalendarStore: Sized + std::ops::Deref {
    unsafe fn calendars(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, calendars)
    }
    unsafe fn calendarWithUID_(&self, UID: NSString) -> CalCalendar
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, calendarWithUID : UID)
    }
    unsafe fn saveCalendar_error_(&self, calendar: CalCalendar, error: *mut NSError) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, saveCalendar : calendar, error : error)
    }
    unsafe fn removeCalendar_error_(&self, calendar: CalCalendar, error: *mut NSError) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, removeCalendar : calendar, error : error)
    }
    unsafe fn eventsWithPredicate_(&self, predicate: NSPredicate) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, eventsWithPredicate : predicate)
    }
    unsafe fn eventWithUID_occurrence_(&self, uid: NSString, date: NSDate) -> CalEvent
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, eventWithUID : uid, occurrence : date)
    }
    unsafe fn tasksWithPredicate_(&self, predicate: NSPredicate) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, tasksWithPredicate : predicate)
    }
    unsafe fn taskWithUID_(&self, uid: NSString) -> CalTask
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, taskWithUID : uid)
    }
    unsafe fn saveEvent_span_error_(
        &self,
        event: CalEvent,
        span: CalSpan,
        error: *mut NSError,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, saveEvent : event, span : span, error : error)
    }
    unsafe fn removeEvent_span_error_(
        &self,
        event: CalEvent,
        span: CalSpan,
        error: *mut NSError,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, removeEvent : event, span : span, error : error)
    }
    unsafe fn saveTask_error_(&self, task: CalTask, error: *mut NSError) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, saveTask : task, error : error)
    }
    unsafe fn removeTask_error_(&self, task: CalTask, error: *mut NSError) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, removeTask : task, error : error)
    }
    unsafe fn defaultCalendarStore() -> CalCalendarStore
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CalCalendarStore").unwrap(), defaultCalendarStore)
    }
    unsafe fn eventPredicateWithStartDate_endDate_calendars_(
        startDate: NSDate,
        endDate: NSDate,
        calendars: NSArray,
    ) -> NSPredicate
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CalCalendarStore").unwrap(), eventPredicateWithStartDate : startDate, endDate : endDate, calendars : calendars)
    }
    unsafe fn eventPredicateWithStartDate_endDate_UID_calendars_(
        startDate: NSDate,
        endDate: NSDate,
        UID: NSString,
        calendars: NSArray,
    ) -> NSPredicate
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CalCalendarStore").unwrap(), eventPredicateWithStartDate : startDate, endDate : endDate, UID : UID, calendars : calendars)
    }
    unsafe fn taskPredicateWithCalendars_(calendars: NSArray) -> NSPredicate
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CalCalendarStore").unwrap(), taskPredicateWithCalendars : calendars)
    }
    unsafe fn taskPredicateWithUncompletedTasks_(calendars: NSArray) -> NSPredicate
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CalCalendarStore").unwrap(), taskPredicateWithUncompletedTasks : calendars)
    }
    unsafe fn taskPredicateWithUncompletedTasksDueBefore_calendars_(
        dueDate: NSDate,
        calendars: NSArray,
    ) -> NSPredicate
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CalCalendarStore").unwrap(), taskPredicateWithUncompletedTasksDueBefore : dueDate, calendars : calendars)
    }
    unsafe fn taskPredicateWithTasksCompletedSince_calendars_(
        completedSince: NSDate,
        calendars: NSArray,
    ) -> NSPredicate
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CalCalendarStore").unwrap(), taskPredicateWithTasksCompletedSince : completedSince, calendars : calendars)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CalCalendar(pub id);
impl std::ops::Deref for CalCalendar {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CalCalendar {}
impl CalCalendar {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CalCalendar").unwrap(), alloc) })
    }
}
impl PNSCopying for CalCalendar {}
impl INSObject for CalCalendar {}
impl PNSObject for CalCalendar {}
impl std::convert::TryFrom<NSObject> for CalCalendar {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<CalCalendar, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CalCalendar").unwrap()) };
        if is_kind_of {
            Ok(CalCalendar(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to CalCalendar")
        }
    }
}
impl ICalCalendar for CalCalendar {}
pub trait ICalCalendar: Sized + std::ops::Deref {
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
    unsafe fn type_(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, type)
    }
    unsafe fn uid(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, uid)
    }
    unsafe fn isEditable(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isEditable)
    }
    unsafe fn calendar() -> id
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CalCalendar").unwrap(), calendar)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CalCalendarItem(pub id);
impl std::ops::Deref for CalCalendarItem {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CalCalendarItem {}
impl CalCalendarItem {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CalCalendarItem").unwrap(), alloc) })
    }
}
impl PNSCopying for CalCalendarItem {}
impl INSObject for CalCalendarItem {}
impl PNSObject for CalCalendarItem {}
impl std::convert::TryFrom<NSObject> for CalCalendarItem {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<CalCalendarItem, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CalCalendarItem").unwrap()) };
        if is_kind_of {
            Ok(CalCalendarItem(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to CalCalendarItem")
        }
    }
}
impl ICalCalendarItem for CalCalendarItem {}
pub trait ICalCalendarItem: Sized + std::ops::Deref {
    unsafe fn hasAlarm(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, hasAlarm)
    }
    unsafe fn nextAlarmDate(&self) -> NSDate
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, nextAlarmDate)
    }
    unsafe fn addAlarm_(&self, alarm: CalAlarm)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, addAlarm : alarm)
    }
    unsafe fn addAlarms_(&self, alarms: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, addAlarms : alarms)
    }
    unsafe fn removeAlarm_(&self, alarm: CalAlarm)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, removeAlarm : alarm)
    }
    unsafe fn removeAlarms_(&self, alarms: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, removeAlarms : alarms)
    }
    unsafe fn calendar(&self) -> CalCalendar
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, calendar)
    }
    unsafe fn setCalendar_(&self, calendar: CalCalendar)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCalendar : calendar)
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
    unsafe fn uid(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, uid)
    }
    unsafe fn dateStamp(&self) -> NSDate
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, dateStamp)
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
}
pub type CalPriority = NSUInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CalTask(pub id);
impl std::ops::Deref for CalTask {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CalTask {}
impl CalTask {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CalTask").unwrap(), alloc) })
    }
}
impl ICalCalendarItem for CalTask {}
impl PNSCopying for CalTask {}
impl From<CalTask> for CalCalendarItem {
    fn from(child: CalTask) -> CalCalendarItem {
        CalCalendarItem(child.0)
    }
}
impl std::convert::TryFrom<CalCalendarItem> for CalTask {
    type Error = &'static str;
    fn try_from(parent: CalCalendarItem) -> Result<CalTask, Self::Error> {
        let is_kind_of: bool = unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CalTask").unwrap()) };
        if is_kind_of {
            Ok(CalTask(parent.0))
        } else {
            Err("This CalCalendarItem cannot be downcasted to CalTask")
        }
    }
}
impl INSObject for CalTask {}
impl PNSObject for CalTask {}
impl ICalTask for CalTask {}
pub trait ICalTask: Sized + std::ops::Deref {
    unsafe fn dueDate(&self) -> NSDate
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, dueDate)
    }
    unsafe fn setDueDate_(&self, dueDate: NSDate)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDueDate : dueDate)
    }
    unsafe fn priority(&self) -> CalPriority
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, priority)
    }
    unsafe fn setPriority_(&self, priority: CalPriority)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPriority : priority)
    }
    unsafe fn isCompleted(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isCompleted)
    }
    unsafe fn setIsCompleted_(&self, isCompleted: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setIsCompleted : isCompleted)
    }
    unsafe fn completedDate(&self) -> NSDate
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, completedDate)
    }
    unsafe fn setCompletedDate_(&self, completedDate: NSDate)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCompletedDate : completedDate)
    }
    unsafe fn task() -> id
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CalTask").unwrap(), task)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CalEvent(pub id);
impl std::ops::Deref for CalEvent {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CalEvent {}
impl CalEvent {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CalEvent").unwrap(), alloc) })
    }
}
impl ICalCalendarItem for CalEvent {}
impl PNSCopying for CalEvent {}
impl std::convert::TryFrom<CalCalendarItem> for CalEvent {
    type Error = &'static str;
    fn try_from(parent: CalCalendarItem) -> Result<CalEvent, Self::Error> {
        let is_kind_of: bool = unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CalEvent").unwrap()) };
        if is_kind_of {
            Ok(CalEvent(parent.0))
        } else {
            Err("This CalCalendarItem cannot be downcasted to CalEvent")
        }
    }
}
impl INSObject for CalEvent {}
impl PNSObject for CalEvent {}
impl ICalEvent for CalEvent {}
pub trait ICalEvent: Sized + std::ops::Deref {
    unsafe fn isAllDay(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isAllDay)
    }
    unsafe fn setIsAllDay_(&self, isAllDay: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setIsAllDay : isAllDay)
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
    unsafe fn recurrenceRule(&self) -> CalRecurrenceRule
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, recurrenceRule)
    }
    unsafe fn setRecurrenceRule_(&self, recurrenceRule: CalRecurrenceRule)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRecurrenceRule : recurrenceRule)
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
    unsafe fn attendees(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, attendees)
    }
    unsafe fn isDetached(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isDetached)
    }
    unsafe fn occurrence(&self) -> NSDate
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, occurrence)
    }
    unsafe fn event() -> id
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CalEvent").unwrap(), event)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CalRecurrenceEnd(pub id);
impl std::ops::Deref for CalRecurrenceEnd {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CalRecurrenceEnd {}
impl CalRecurrenceEnd {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CalRecurrenceEnd").unwrap(), alloc) })
    }
}
impl PNSCopying for CalRecurrenceEnd {}
impl INSObject for CalRecurrenceEnd {}
impl PNSObject for CalRecurrenceEnd {}
impl std::convert::TryFrom<NSObject> for CalRecurrenceEnd {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<CalRecurrenceEnd, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CalRecurrenceEnd").unwrap()) };
        if is_kind_of {
            Ok(CalRecurrenceEnd(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to CalRecurrenceEnd")
        }
    }
}
impl ICalRecurrenceEnd for CalRecurrenceEnd {}
pub trait ICalRecurrenceEnd: Sized + std::ops::Deref {
    unsafe fn usesEndDate(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, usesEndDate)
    }
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
    unsafe fn recurrenceEndWithEndDate_(endDate: NSDate) -> id
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CalRecurrenceEnd").unwrap(), recurrenceEndWithEndDate : endDate)
    }
    unsafe fn recurrenceEndWithOccurrenceCount_(occurrenceCount: NSUInteger) -> id
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CalRecurrenceEnd").unwrap(), recurrenceEndWithOccurrenceCount : occurrenceCount)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CalNthWeekDay(pub id);
impl std::ops::Deref for CalNthWeekDay {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CalNthWeekDay {}
impl CalNthWeekDay {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CalNthWeekDay").unwrap(), alloc) })
    }
}
impl PNSCopying for CalNthWeekDay {}
impl INSObject for CalNthWeekDay {}
impl PNSObject for CalNthWeekDay {}
impl std::convert::TryFrom<NSObject> for CalNthWeekDay {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<CalNthWeekDay, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CalNthWeekDay").unwrap()) };
        if is_kind_of {
            Ok(CalNthWeekDay(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to CalNthWeekDay")
        }
    }
}
impl ICalNthWeekDay for CalNthWeekDay {}
pub trait ICalNthWeekDay: Sized + std::ops::Deref {
    unsafe fn dayOfTheWeek(&self) -> NSUInteger
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
}
pub type CalRecurrenceType = ::std::os::raw::c_uint;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CalRecurrenceRule(pub id);
impl std::ops::Deref for CalRecurrenceRule {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CalRecurrenceRule {}
impl CalRecurrenceRule {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CalRecurrenceRule").unwrap(), alloc) })
    }
}
impl PNSCopying for CalRecurrenceRule {}
impl INSObject for CalRecurrenceRule {}
impl PNSObject for CalRecurrenceRule {}
impl std::convert::TryFrom<NSObject> for CalRecurrenceRule {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<CalRecurrenceRule, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CalRecurrenceRule").unwrap()) };
        if is_kind_of {
            Ok(CalRecurrenceRule(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to CalRecurrenceRule")
        }
    }
}
impl ICalRecurrenceRule for CalRecurrenceRule {}
pub trait ICalRecurrenceRule: Sized + std::ops::Deref {
    unsafe fn initDailyRecurrenceWithInterval_end_(
        &self,
        interval: NSUInteger,
        end: CalRecurrenceEnd,
    ) -> id
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initDailyRecurrenceWithInterval : interval, end : end)
    }
    unsafe fn initWeeklyRecurrenceWithInterval_end_(
        &self,
        interval: NSUInteger,
        end: CalRecurrenceEnd,
    ) -> id
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWeeklyRecurrenceWithInterval : interval, end : end)
    }
    unsafe fn initWeeklyRecurrenceWithInterval_forDaysOfTheWeek_end_(
        &self,
        interval: NSUInteger,
        days: NSArray,
        end: CalRecurrenceEnd,
    ) -> id
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWeeklyRecurrenceWithInterval : interval, forDaysOfTheWeek : days, end : end)
    }
    unsafe fn initMonthlyRecurrenceWithInterval_end_(
        &self,
        interval: NSUInteger,
        end: CalRecurrenceEnd,
    ) -> id
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initMonthlyRecurrenceWithInterval : interval, end : end)
    }
    unsafe fn initMonthlyRecurrenceWithInterval_forDaysOfTheMonth_end_(
        &self,
        interval: NSUInteger,
        monthDays: NSArray,
        end: CalRecurrenceEnd,
    ) -> id
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initMonthlyRecurrenceWithInterval : interval, forDaysOfTheMonth : monthDays, end : end)
    }
    unsafe fn initMonthlyRecurrenceWithInterval_forDayOfTheWeek_forWeekOfTheMonth_end_(
        &self,
        interval: NSUInteger,
        weekDay: NSUInteger,
        monthWeek: NSInteger,
        end: CalRecurrenceEnd,
    ) -> id
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initMonthlyRecurrenceWithInterval : interval, forDayOfTheWeek : weekDay, forWeekOfTheMonth : monthWeek, end : end)
    }
    unsafe fn initYearlyRecurrenceWithInterval_end_(
        &self,
        interval: NSUInteger,
        end: CalRecurrenceEnd,
    ) -> id
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initYearlyRecurrenceWithInterval : interval, end : end)
    }
    unsafe fn initYearlyRecurrenceWithInterval_forMonthsOfTheYear_end_(
        &self,
        interval: NSUInteger,
        months: NSArray,
        end: CalRecurrenceEnd,
    ) -> id
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initYearlyRecurrenceWithInterval : interval, forMonthsOfTheYear : months, end : end)
    }
    unsafe fn initYearlyRecurrenceWithInterval_forDayOfTheWeek_forWeekOfTheMonth_forMonthsOfTheYear_end_(
        &self,
        interval: NSUInteger,
        weekDay: NSUInteger,
        monthWeek: NSInteger,
        months: NSArray,
        end: CalRecurrenceEnd,
    ) -> id
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initYearlyRecurrenceWithInterval : interval, forDayOfTheWeek : weekDay, forWeekOfTheMonth : monthWeek, forMonthsOfTheYear : months, end : end)
    }
    unsafe fn recurrenceEnd(&self) -> CalRecurrenceEnd
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, recurrenceEnd)
    }
    unsafe fn recurrenceType(&self) -> CalRecurrenceType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, recurrenceType)
    }
    unsafe fn recurrenceInterval(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, recurrenceInterval)
    }
    unsafe fn firstDayOfTheWeek(&self) -> NSUInteger
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
    unsafe fn nthWeekDaysOfTheMonth(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, nthWeekDaysOfTheMonth)
    }
    unsafe fn monthsOfTheYear(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, monthsOfTheYear)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CalAttendee(pub id);
impl std::ops::Deref for CalAttendee {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CalAttendee {}
impl CalAttendee {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CalAttendee").unwrap(), alloc) })
    }
}
impl PNSCopying for CalAttendee {}
impl INSObject for CalAttendee {}
impl PNSObject for CalAttendee {}
impl std::convert::TryFrom<NSObject> for CalAttendee {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<CalAttendee, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CalAttendee").unwrap()) };
        if is_kind_of {
            Ok(CalAttendee(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to CalAttendee")
        }
    }
}
impl ICalAttendee for CalAttendee {}
pub trait ICalAttendee: Sized + std::ops::Deref {
    unsafe fn address(&self) -> NSURL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, address)
    }
    unsafe fn commonName(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, commonName)
    }
    unsafe fn status(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, status)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CalAlarm(pub id);
impl std::ops::Deref for CalAlarm {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CalAlarm {}
impl CalAlarm {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CalAlarm").unwrap(), alloc) })
    }
}
impl PNSCopying for CalAlarm {}
impl INSObject for CalAlarm {}
impl PNSObject for CalAlarm {}
impl std::convert::TryFrom<NSObject> for CalAlarm {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<CalAlarm, Self::Error> {
        let is_kind_of: bool = unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CalAlarm").unwrap()) };
        if is_kind_of {
            Ok(CalAlarm(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to CalAlarm")
        }
    }
}
impl ICalAlarm for CalAlarm {}
pub trait ICalAlarm: Sized + std::ops::Deref {
    unsafe fn setAcknowledged_(&self, date: NSDate)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAcknowledged : date)
    }
    unsafe fn acknowledged(&self) -> NSDate
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, acknowledged)
    }
    unsafe fn setRelatedTo_(&self, relatedTo: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRelatedTo : relatedTo)
    }
    unsafe fn relatedTo(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, relatedTo)
    }
    unsafe fn triggerDateRelativeTo_(&self, date: NSDate) -> NSDate
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, triggerDateRelativeTo : date)
    }
    unsafe fn action(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, action)
    }
    unsafe fn setAction_(&self, action: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAction : action)
    }
    unsafe fn sound(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, sound)
    }
    unsafe fn setSound_(&self, sound: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSound : sound)
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
    unsafe fn relativeTrigger(&self) -> NSTimeInterval
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, relativeTrigger)
    }
    unsafe fn setRelativeTrigger_(&self, relativeTrigger: NSTimeInterval)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRelativeTrigger : relativeTrigger)
    }
    unsafe fn absoluteTrigger(&self) -> NSDate
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, absoluteTrigger)
    }
    unsafe fn setAbsoluteTrigger_(&self, absoluteTrigger: NSDate)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAbsoluteTrigger : absoluteTrigger)
    }
    unsafe fn alarm() -> id
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CalAlarm").unwrap(), alarm)
    }
}
unsafe extern "C" {
    pub static CalCalendarsChangedNotification: NSString;
}
unsafe extern "C" {
    pub static CalEventsChangedNotification: NSString;
}
unsafe extern "C" {
    pub static CalTasksChangedNotification: NSString;
}
unsafe extern "C" {
    pub static CalCalendarsChangedExternallyNotification: NSString;
}
unsafe extern "C" {
    pub static CalEventsChangedExternallyNotification: NSString;
}
unsafe extern "C" {
    pub static CalTasksChangedExternallyNotification: NSString;
}
unsafe extern "C" {
    pub static CalInsertedRecordsKey: NSString;
}
unsafe extern "C" {
    pub static CalUpdatedRecordsKey: NSString;
}
unsafe extern "C" {
    pub static CalDeletedRecordsKey: NSString;
}
unsafe extern "C" {
    pub static CalSenderProcessIDKey: NSString;
}
unsafe extern "C" {
    pub static CalUserUIDKey: NSString;
}
unsafe extern "C" {
    pub static CalCalendarTypeBirthday: NSString;
}
unsafe extern "C" {
    pub static CalCalendarTypeCalDAV: NSString;
}
unsafe extern "C" {
    pub static CalCalendarTypeLocal: NSString;
}
unsafe extern "C" {
    pub static CalCalendarTypeSubscription: NSString;
}
unsafe extern "C" {
    pub static CalCalendarTypeIMAP: NSString;
}
unsafe extern "C" {
    pub static CalCalendarTypeExchange: NSString;
}
unsafe extern "C" {
    pub static CalDefaultRecurrenceInterval: NSUInteger;
}
unsafe extern "C" {
    pub static CalAttendeeStatusNeedsAction: NSString;
}
unsafe extern "C" {
    pub static CalAttendeeStatusAccepted: NSString;
}
unsafe extern "C" {
    pub static CalAttendeeStatusDeclined: NSString;
}
unsafe extern "C" {
    pub static CalAttendeeStatusTentative: NSString;
}
unsafe extern "C" {
    pub static CalAlarmActionDisplay: NSString;
}
unsafe extern "C" {
    pub static CalAlarmActionEmail: NSString;
}
unsafe extern "C" {
    pub static CalAlarmActionProcedure: NSString;
}
unsafe extern "C" {
    pub static CalAlarmActionSound: NSString;
}
unsafe extern "C" {
    pub static CalCalendarStoreErrorDomain: NSString;
}

unsafe impl objc2::encode::RefEncode for CalCalendarStore {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CalCalendarStore {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CalCalendar {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CalCalendar {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CalCalendarItem {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CalCalendarItem {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CalTask {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CalTask {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CalEvent {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CalEvent {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CalRecurrenceEnd {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CalRecurrenceEnd {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CalNthWeekDay {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CalNthWeekDay {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CalRecurrenceRule {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CalRecurrenceRule {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CalAttendee {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CalAttendee {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CalAlarm {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CalAlarm {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
