#[allow(unused_imports)]
use crate::objc::*;
#[allow(unused_imports)]
use crate::CoreGraphics::*;
#[allow(unused_imports)]
use crate::CoreServices::*;
#[allow(unused_imports)]
use crate::Foundation::*;
#[allow(unused_imports)]
use libc::{gid_t, id_t, mach_port_t, mode_t, size_t, uid_t};

#[allow(unused_imports)]
use objc2::msg_send;
pub type va_list = __builtin_va_list;
pub type __darwin_natural_t = ::std::os::raw::c_uint;
pub type __darwin_mach_port_name_t = __darwin_natural_t;
pub type malloc_type_id_t = ::std::os::raw::c_ulonglong;
pub type malloc_zone_t = _malloc_zone_t;
pub type integer_t = ::std::os::raw::c_int;
pub type vm_offset_t = usize;
pub type vm_size_t = usize;
pub type kern_return_t = ::std::os::raw::c_int;
pub type vm_address_t = vm_offset_t;
pub type cpu_type_t = integer_t;
pub type task_t = mach_port_t;
pub type UInt8 = ::std::os::raw::c_uchar;
pub type SInt8 = ::std::os::raw::c_schar;
pub type UInt16 = ::std::os::raw::c_ushort;
pub type SInt16 = ::std::os::raw::c_short;
pub type UInt32 = ::std::os::raw::c_uint;
pub type SInt32 = ::std::os::raw::c_int;
pub type Float32 = f32;
pub type Float64 = f64;
pub type OSErr = SInt16;
pub type OSStatus = SInt32;
pub type ScriptCode = SInt16;
pub type LangCode = SInt16;
pub type RegionCode = SInt16;
pub type FourCharCode = UInt32;
pub type OSType = FourCharCode;
pub type Boolean = ::std::os::raw::c_uchar;
pub type UTF32Char = UInt32;
pub type UniChar = UInt16;
pub type UTF16Char = UInt16;
pub type UTF8Char = UInt8;
pub type UniCharCount = ::std::os::raw::c_ulong;
pub type StringPtr = *mut ::std::os::raw::c_uchar;
pub type ConstStringPtr = *const ::std::os::raw::c_uchar;
pub type ConstStr255Param = *const ::std::os::raw::c_uchar;
pub type Byte = UInt8;
pub type SignedByte = SInt8;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _malloc_zone_t {
    pub reserved1: *mut ::std::os::raw::c_void,
    pub reserved2: *mut ::std::os::raw::c_void,
    pub size: ::std::option::Option<
        unsafe extern "C" fn(
            zone: *mut _malloc_zone_t,
            ptr: *const ::std::os::raw::c_void,
        ) -> usize,
    >,
    pub malloc: ::std::option::Option<
        unsafe extern "C" fn(zone: *mut _malloc_zone_t, size: usize) -> *mut ::std::os::raw::c_void,
    >,
    pub calloc: ::std::option::Option<
        unsafe extern "C" fn(
            zone: *mut _malloc_zone_t,
            num_items: usize,
            size: usize,
        ) -> *mut ::std::os::raw::c_void,
    >,
    pub valloc: ::std::option::Option<
        unsafe extern "C" fn(zone: *mut _malloc_zone_t, size: usize) -> *mut ::std::os::raw::c_void,
    >,
    pub free: ::std::option::Option<
        unsafe extern "C" fn(zone: *mut _malloc_zone_t, ptr: *mut ::std::os::raw::c_void),
    >,
    pub realloc: ::std::option::Option<
        unsafe extern "C" fn(
            zone: *mut _malloc_zone_t,
            ptr: *mut ::std::os::raw::c_void,
            size: usize,
        ) -> *mut ::std::os::raw::c_void,
    >,
    pub destroy: ::std::option::Option<unsafe extern "C" fn(zone: *mut _malloc_zone_t)>,
    pub zone_name: *const ::std::os::raw::c_char,
    pub batch_malloc: ::std::option::Option<
        unsafe extern "C" fn(
            zone: *mut _malloc_zone_t,
            size: usize,
            results: *mut *mut ::std::os::raw::c_void,
            num_requested: ::std::os::raw::c_uint,
        ) -> ::std::os::raw::c_uint,
    >,
    pub batch_free: ::std::option::Option<
        unsafe extern "C" fn(
            zone: *mut _malloc_zone_t,
            to_be_freed: *mut *mut ::std::os::raw::c_void,
            num_to_be_freed: ::std::os::raw::c_uint,
        ),
    >,
    pub introspect: *mut malloc_introspection_t,
    pub version: ::std::os::raw::c_uint,
    pub memalign: ::std::option::Option<
        unsafe extern "C" fn(
            zone: *mut _malloc_zone_t,
            alignment: usize,
            size: usize,
        ) -> *mut ::std::os::raw::c_void,
    >,
    pub free_definite_size: ::std::option::Option<
        unsafe extern "C" fn(
            zone: *mut _malloc_zone_t,
            ptr: *mut ::std::os::raw::c_void,
            size: usize,
        ),
    >,
    pub pressure_relief: ::std::option::Option<
        unsafe extern "C" fn(zone: *mut _malloc_zone_t, goal: usize) -> usize,
    >,
    pub claimed_address: ::std::option::Option<
        unsafe extern "C" fn(
            zone: *mut _malloc_zone_t,
            ptr: *mut ::std::os::raw::c_void,
        ) -> boolean_t,
    >,
    pub try_free_default: ::std::option::Option<
        unsafe extern "C" fn(zone: *mut _malloc_zone_t, ptr: *mut ::std::os::raw::c_void),
    >,
    pub malloc_with_options: ::std::option::Option<
        unsafe extern "C" fn(
            zone: *mut _malloc_zone_t,
            align: usize,
            size: usize,
            options: u64,
        ) -> *mut ::std::os::raw::c_void,
    >,
    pub malloc_type_malloc: ::std::option::Option<
        unsafe extern "C" fn(
            zone: *mut _malloc_zone_t,
            size: usize,
            type_id: malloc_type_id_t,
        ) -> *mut ::std::os::raw::c_void,
    >,
    pub malloc_type_calloc: ::std::option::Option<
        unsafe extern "C" fn(
            zone: *mut _malloc_zone_t,
            count: usize,
            size: usize,
            type_id: malloc_type_id_t,
        ) -> *mut ::std::os::raw::c_void,
    >,
    pub malloc_type_realloc: ::std::option::Option<
        unsafe extern "C" fn(
            zone: *mut _malloc_zone_t,
            ptr: *mut ::std::os::raw::c_void,
            size: usize,
            type_id: malloc_type_id_t,
        ) -> *mut ::std::os::raw::c_void,
    >,
    pub malloc_type_memalign: ::std::option::Option<
        unsafe extern "C" fn(
            zone: *mut _malloc_zone_t,
            alignment: usize,
            size: usize,
            type_id: malloc_type_id_t,
        ) -> *mut ::std::os::raw::c_void,
    >,
    pub malloc_type_malloc_with_options: ::std::option::Option<
        unsafe extern "C" fn(
            zone: *mut _malloc_zone_t,
            align: usize,
            size: usize,
            options: u64,
            type_id: malloc_type_id_t,
        ) -> *mut ::std::os::raw::c_void,
    >,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct vm_range_t {
    pub address: vm_address_t,
    pub size: vm_size_t,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct malloc_statistics_t {
    pub blocks_in_use: ::std::os::raw::c_uint,
    pub size_in_use: usize,
    pub max_size_in_use: usize,
    pub size_allocated: usize,
}
pub type memory_reader_t = ::std::option::Option<
    unsafe extern "C" fn(
        remote_task: task_t,
        remote_address: vm_address_t,
        size: vm_size_t,
        local_memory: *mut *mut ::std::os::raw::c_void,
    ) -> kern_return_t,
>;
pub type vm_range_recorder_t = ::std::option::Option<
    unsafe extern "C" fn(
        arg1: task_t,
        arg2: *mut ::std::os::raw::c_void,
        type_: ::std::os::raw::c_uint,
        arg3: *mut vm_range_t,
        arg4: ::std::os::raw::c_uint,
    ),
>;
pub type print_task_printer_t =
    ::std::option::Option<unsafe extern "C" fn(fmt: *const ::std::os::raw::c_char, ...)>;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct malloc_introspection_t {
    pub enumerator: ::std::option::Option<
        unsafe extern "C" fn(
            task: task_t,
            arg1: *mut ::std::os::raw::c_void,
            type_mask: ::std::os::raw::c_uint,
            zone_address: vm_address_t,
            reader: memory_reader_t,
            recorder: vm_range_recorder_t,
        ) -> kern_return_t,
    >,
    pub good_size:
        ::std::option::Option<unsafe extern "C" fn(zone: *mut malloc_zone_t, size: usize) -> usize>,
    pub check: ::std::option::Option<unsafe extern "C" fn(zone: *mut malloc_zone_t) -> boolean_t>,
    pub print:
        ::std::option::Option<unsafe extern "C" fn(zone: *mut malloc_zone_t, verbose: boolean_t)>,
    pub log: ::std::option::Option<
        unsafe extern "C" fn(zone: *mut malloc_zone_t, address: *mut ::std::os::raw::c_void),
    >,
    pub force_lock: ::std::option::Option<unsafe extern "C" fn(zone: *mut malloc_zone_t)>,
    pub force_unlock: ::std::option::Option<unsafe extern "C" fn(zone: *mut malloc_zone_t)>,
    pub statistics: ::std::option::Option<
        unsafe extern "C" fn(zone: *mut malloc_zone_t, stats: *mut malloc_statistics_t),
    >,
    pub zone_locked:
        ::std::option::Option<unsafe extern "C" fn(zone: *mut malloc_zone_t) -> boolean_t>,
    pub enable_discharge_checking:
        ::std::option::Option<unsafe extern "C" fn(zone: *mut malloc_zone_t) -> boolean_t>,
    pub disable_discharge_checking:
        ::std::option::Option<unsafe extern "C" fn(zone: *mut malloc_zone_t)>,
    pub discharge: ::std::option::Option<
        unsafe extern "C" fn(zone: *mut malloc_zone_t, memory: *mut ::std::os::raw::c_void),
    >,
    pub enumerate_discharged_pointers: ::std::option::Option<
        unsafe extern "C" fn(
            zone: *mut malloc_zone_t,
            report_discharged: *mut ::std::os::raw::c_void,
        ),
    >,
    pub reinit_lock: ::std::option::Option<unsafe extern "C" fn(zone: *mut malloc_zone_t)>,
    pub print_task: ::std::option::Option<
        unsafe extern "C" fn(
            task: task_t,
            level: ::std::os::raw::c_uint,
            zone_address: vm_address_t,
            reader: memory_reader_t,
            printer: print_task_printer_t,
        ),
    >,
    pub task_statistics: ::std::option::Option<
        unsafe extern "C" fn(
            task: task_t,
            zone_address: vm_address_t,
            reader: memory_reader_t,
            stats: *mut malloc_statistics_t,
        ),
    >,
    pub zone_type: ::std::os::raw::c_uint,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _acl {
    _unused: [u8; 0],
}
pub type acl_t = *mut _acl;
pub trait IClass: Sized + std::ops::Deref {}
pub type IMP = ::std::option::Option<unsafe extern "C" fn()>;
pub type NSUInteger = ::std::os::raw::c_ulong;
pub trait PNSObject: Sized + std::ops::Deref {
    unsafe fn isEqual_(&self, object: id) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, isEqual : object)
    }
    unsafe fn class(&self) -> Class
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, class)
    }
    unsafe fn self_(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, self_)
    }
    unsafe fn performSelector_(&self, aSelector: objc2::runtime::Sel) -> id
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, performSelector : aSelector)
    }
    unsafe fn performSelector_withObject_(&self, aSelector: objc2::runtime::Sel, object: id) -> id
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, performSelector : aSelector, withObject : object)
    }
    unsafe fn performSelector_withObject_withObject_(
        &self,
        aSelector: objc2::runtime::Sel,
        object1: id,
        object2: id,
    ) -> id
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, performSelector : aSelector, withObject : object1, withObject : object2)
    }
    unsafe fn isProxy(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isProxy)
    }
    unsafe fn isKindOfClass_(&self, aClass: Class) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, isKindOfClass : aClass)
    }
    unsafe fn isMemberOfClass_(&self, aClass: Class) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, isMemberOfClass : aClass)
    }
    unsafe fn conformsToProtocol_(&self, aProtocol: Protocol) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, conformsToProtocol : aProtocol)
    }
    unsafe fn respondsToSelector_(&self, aSelector: objc2::runtime::Sel) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, respondsToSelector : aSelector)
    }
    unsafe fn retain(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, retain)
    }
    unsafe fn release(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, release)
    }
    unsafe fn autorelease(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, autorelease)
    }
    unsafe fn retainCount(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, retainCount)
    }
    unsafe fn zone(&self) -> *mut _NSZone
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, zone)
    }
    unsafe fn hash(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, hash)
    }
    unsafe fn superclass(&self) -> Class
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, superclass)
    }
    unsafe fn description(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, description)
    }
    unsafe fn debugDescription(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, debugDescription)
    }
}
pub trait INSObject: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn dealloc(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, dealloc)
    }
    unsafe fn finalize(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, finalize)
    }
    unsafe fn copy(&self) -> id
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, copy)
    }
    unsafe fn mutableCopy(&self) -> id
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, mutableCopy)
    }
    unsafe fn methodForSelector_(&self, aSelector: objc2::runtime::Sel) -> IMP
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, methodForSelector : aSelector)
    }
    unsafe fn doesNotRecognizeSelector_(&self, aSelector: objc2::runtime::Sel)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, doesNotRecognizeSelector : aSelector)
    }
    unsafe fn forwardingTargetForSelector_(&self, aSelector: objc2::runtime::Sel) -> id
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, forwardingTargetForSelector : aSelector)
    }
    unsafe fn forwardInvocation_(&self, anInvocation: NSInvocation)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, forwardInvocation : anInvocation)
    }
    unsafe fn methodSignatureForSelector_(&self, aSelector: objc2::runtime::Sel) -> NSMethodSignature
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, methodSignatureForSelector : aSelector)
    }
    unsafe fn allowsWeakReference(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, allowsWeakReference)
    }
    unsafe fn retainWeakReference(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, retainWeakReference)
    }
    unsafe fn load()
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NSObject").unwrap(), load)
    }
    unsafe fn initialize()
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NSObject").unwrap(), initialize)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NSObject").unwrap(), new)
    }
    unsafe fn allocWithZone_(zone: *mut _NSZone) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NSObject").unwrap(), allocWithZone : zone)
    }
    unsafe fn alloc() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NSObject").unwrap(), alloc)
    }
    unsafe fn copyWithZone_(zone: *mut _NSZone) -> id
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NSObject").unwrap(), copyWithZone : zone)
    }
    unsafe fn mutableCopyWithZone_(zone: *mut _NSZone) -> id
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NSObject").unwrap(), mutableCopyWithZone : zone)
    }
    unsafe fn instancesRespondToSelector_(aSelector: objc2::runtime::Sel) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NSObject").unwrap(), instancesRespondToSelector : aSelector)
    }
    unsafe fn conformsToProtocol_(protocol: Protocol) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NSObject").unwrap(), conformsToProtocol : protocol)
    }
    unsafe fn instanceMethodForSelector_(aSelector: objc2::runtime::Sel) -> IMP
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NSObject").unwrap(), instanceMethodForSelector : aSelector)
    }
    unsafe fn instanceMethodSignatureForSelector_(
        aSelector: objc2::runtime::Sel,
    ) -> NSMethodSignature
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NSObject").unwrap(), instanceMethodSignatureForSelector : aSelector)
    }
    unsafe fn isSubclassOfClass_(aClass: Class) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NSObject").unwrap(), isSubclassOfClass : aClass)
    }
    unsafe fn resolveClassMethod_(sel: objc2::runtime::Sel) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NSObject").unwrap(), resolveClassMethod : sel)
    }
    unsafe fn resolveInstanceMethod_(sel: objc2::runtime::Sel) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NSObject").unwrap(), resolveInstanceMethod : sel)
    }
    unsafe fn hash() -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NSObject").unwrap(), hash)
    }
    unsafe fn superclass() -> Class
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NSObject").unwrap(), superclass)
    }
    unsafe fn class() -> Class
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NSObject").unwrap(), class)
    }
    unsafe fn description() -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NSObject").unwrap(), description)
    }
    unsafe fn debugDescription() -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NSObject").unwrap(), debugDescription)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct OS_object(pub id);
impl std::ops::Deref for OS_object {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for OS_object {}
impl OS_object {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"OS_object").unwrap(), alloc) })
    }
}
impl INSObject for OS_object {}
impl PNSObject for OS_object {}
impl std::convert::TryFrom<NSObject> for OS_object {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<OS_object, Self::Error> {
        let is_kind_of: bool = unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"OS_object").unwrap()) };
        if is_kind_of {
            Ok(OS_object(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to OS_object")
        }
    }
}
impl IOS_object for OS_object {}
pub trait IOS_object: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct OS_os_workgroup(pub id);
impl std::ops::Deref for OS_os_workgroup {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for OS_os_workgroup {}
impl OS_os_workgroup {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"OS_os_workgroup").unwrap(), alloc) })
    }
}
impl IOS_object for OS_os_workgroup {}
impl From<OS_os_workgroup> for OS_object {
    fn from(child: OS_os_workgroup) -> OS_object {
        OS_object(child.0)
    }
}
impl std::convert::TryFrom<OS_object> for OS_os_workgroup {
    type Error = &'static str;
    fn try_from(parent: OS_object) -> Result<OS_os_workgroup, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"OS_os_workgroup").unwrap()) };
        if is_kind_of {
            Ok(OS_os_workgroup(parent.0))
        } else {
            Err("This OS_object cannot be downcasted to OS_os_workgroup")
        }
    }
}
impl INSObject for OS_os_workgroup {}
impl PNSObject for OS_os_workgroup {}
impl IOS_os_workgroup for OS_os_workgroup {}
pub trait IOS_os_workgroup: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
}
pub trait POS_os_workgroup_interval: Sized + std::ops::Deref {}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct OS_os_workgroup_interval(pub id);
impl std::ops::Deref for OS_os_workgroup_interval {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for OS_os_workgroup_interval {}
impl OS_os_workgroup_interval {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"OS_os_workgroup_interval").unwrap(), alloc) })
    }
}
impl POS_os_workgroup_interval for OS_os_workgroup_interval {}
impl IOS_os_workgroup for OS_os_workgroup_interval {}
impl From<OS_os_workgroup_interval> for OS_os_workgroup {
    fn from(child: OS_os_workgroup_interval) -> OS_os_workgroup {
        OS_os_workgroup(child.0)
    }
}
impl std::convert::TryFrom<OS_os_workgroup> for OS_os_workgroup_interval {
    type Error = &'static str;
    fn try_from(parent: OS_os_workgroup) -> Result<OS_os_workgroup_interval, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"OS_os_workgroup_interval").unwrap()) };
        if is_kind_of {
            Ok(OS_os_workgroup_interval(parent.0))
        } else {
            Err("This OS_os_workgroup cannot be downcasted to OS_os_workgroup_interval")
        }
    }
}
impl IOS_object for OS_os_workgroup_interval {}
impl INSObject for OS_os_workgroup_interval {}
impl PNSObject for OS_os_workgroup_interval {}
impl IOS_os_workgroup_interval for OS_os_workgroup_interval {}
pub trait IOS_os_workgroup_interval: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
}
pub trait POS_os_workgroup_parallel: Sized + std::ops::Deref {}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct OS_os_workgroup_parallel(pub id);
impl std::ops::Deref for OS_os_workgroup_parallel {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for OS_os_workgroup_parallel {}
impl OS_os_workgroup_parallel {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"OS_os_workgroup_parallel").unwrap(), alloc) })
    }
}
impl POS_os_workgroup_parallel for OS_os_workgroup_parallel {}
impl IOS_os_workgroup for OS_os_workgroup_parallel {}
impl std::convert::TryFrom<OS_os_workgroup> for OS_os_workgroup_parallel {
    type Error = &'static str;
    fn try_from(parent: OS_os_workgroup) -> Result<OS_os_workgroup_parallel, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"OS_os_workgroup_parallel").unwrap()) };
        if is_kind_of {
            Ok(OS_os_workgroup_parallel(parent.0))
        } else {
            Err("This OS_os_workgroup cannot be downcasted to OS_os_workgroup_parallel")
        }
    }
}
impl IOS_object for OS_os_workgroup_parallel {}
impl INSObject for OS_os_workgroup_parallel {}
impl PNSObject for OS_os_workgroup_parallel {}
impl IOS_os_workgroup_parallel for OS_os_workgroup_parallel {}
pub trait IOS_os_workgroup_parallel: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
}
pub type dispatch_queue_t = NSObject;
pub type CFAllocatorTypeID = ::std::os::raw::c_ulonglong;
pub type CFTypeID = ::std::os::raw::c_ulong;
pub type CFOptionFlags = ::std::os::raw::c_ulong;
pub type CFHashCode = ::std::os::raw::c_ulong;
pub type CFIndex = ::std::os::raw::c_long;
pub type CFTypeRef = *const ::std::os::raw::c_void;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __CFString {
    _unused: [u8; 0],
}
pub type CFStringRef = *const __CFString;
pub type CFMutableStringRef = *mut __CFString;
pub type CFPropertyListRef = CFTypeRef;
pub type CFComparisonResult = CFIndex;
pub type CFComparatorFunction = ::std::option::Option<
    unsafe extern "C" fn(
        val1: *const ::std::os::raw::c_void,
        val2: *const ::std::os::raw::c_void,
        context: *mut ::std::os::raw::c_void,
    ) -> CFComparisonResult,
>;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CFRange {
    pub location: CFIndex,
    pub length: CFIndex,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __CFNull {
    _unused: [u8; 0],
}
pub type CFNullRef = *const __CFNull;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __CFAllocator {
    _unused: [u8; 0],
}
pub type CFAllocatorRef = *const __CFAllocator;
pub type CFAllocatorRetainCallBack = ::std::option::Option<
    unsafe extern "C" fn(info: *const ::std::os::raw::c_void) -> *const ::std::os::raw::c_void,
>;
pub type CFAllocatorReleaseCallBack =
    ::std::option::Option<unsafe extern "C" fn(info: *const ::std::os::raw::c_void)>;
pub type CFAllocatorCopyDescriptionCallBack =
    ::std::option::Option<unsafe extern "C" fn(info: *const ::std::os::raw::c_void) -> CFStringRef>;
pub type CFAllocatorAllocateCallBack = ::std::option::Option<
    unsafe extern "C" fn(
        allocSize: CFIndex,
        hint: CFOptionFlags,
        info: *mut ::std::os::raw::c_void,
    ) -> *mut ::std::os::raw::c_void,
>;
pub type CFAllocatorReallocateCallBack = ::std::option::Option<
    unsafe extern "C" fn(
        ptr: *mut ::std::os::raw::c_void,
        newsize: CFIndex,
        hint: CFOptionFlags,
        info: *mut ::std::os::raw::c_void,
    ) -> *mut ::std::os::raw::c_void,
>;
pub type CFAllocatorDeallocateCallBack = ::std::option::Option<
    unsafe extern "C" fn(ptr: *mut ::std::os::raw::c_void, info: *mut ::std::os::raw::c_void),
>;
pub type CFAllocatorPreferredSizeCallBack = ::std::option::Option<
    unsafe extern "C" fn(
        size: CFIndex,
        hint: CFOptionFlags,
        info: *mut ::std::os::raw::c_void,
    ) -> CFIndex,
>;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CFAllocatorContext {
    pub version: CFIndex,
    pub info: *mut ::std::os::raw::c_void,
    pub retain: CFAllocatorRetainCallBack,
    pub release: CFAllocatorReleaseCallBack,
    pub copyDescription: CFAllocatorCopyDescriptionCallBack,
    pub allocate: CFAllocatorAllocateCallBack,
    pub reallocate: CFAllocatorReallocateCallBack,
    pub deallocate: CFAllocatorDeallocateCallBack,
    pub preferredSize: CFAllocatorPreferredSizeCallBack,
}
pub type CFArrayRetainCallBack = ::std::option::Option<
    unsafe extern "C" fn(
        allocator: CFAllocatorRef,
        value: *const ::std::os::raw::c_void,
    ) -> *const ::std::os::raw::c_void,
>;
pub type CFArrayReleaseCallBack = ::std::option::Option<
    unsafe extern "C" fn(allocator: CFAllocatorRef, value: *const ::std::os::raw::c_void),
>;
pub type CFArrayCopyDescriptionCallBack = ::std::option::Option<
    unsafe extern "C" fn(value: *const ::std::os::raw::c_void) -> CFStringRef,
>;
pub type CFArrayEqualCallBack = ::std::option::Option<
    unsafe extern "C" fn(
        value1: *const ::std::os::raw::c_void,
        value2: *const ::std::os::raw::c_void,
    ) -> Boolean,
>;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CFArrayCallBacks {
    pub version: CFIndex,
    pub retain: CFArrayRetainCallBack,
    pub release: CFArrayReleaseCallBack,
    pub copyDescription: CFArrayCopyDescriptionCallBack,
    pub equal: CFArrayEqualCallBack,
}
pub type CFArrayApplierFunction = ::std::option::Option<
    unsafe extern "C" fn(
        value: *const ::std::os::raw::c_void,
        context: *mut ::std::os::raw::c_void,
    ),
>;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __CFArray {
    _unused: [u8; 0],
}
pub type CFArrayRef = *const __CFArray;
pub type CFMutableArrayRef = *mut __CFArray;
pub type CFBagRetainCallBack = ::std::option::Option<
    unsafe extern "C" fn(
        allocator: CFAllocatorRef,
        value: *const ::std::os::raw::c_void,
    ) -> *const ::std::os::raw::c_void,
>;
pub type CFBagReleaseCallBack = ::std::option::Option<
    unsafe extern "C" fn(allocator: CFAllocatorRef, value: *const ::std::os::raw::c_void),
>;
pub type CFBagCopyDescriptionCallBack = ::std::option::Option<
    unsafe extern "C" fn(value: *const ::std::os::raw::c_void) -> CFStringRef,
>;
pub type CFBagEqualCallBack = ::std::option::Option<
    unsafe extern "C" fn(
        value1: *const ::std::os::raw::c_void,
        value2: *const ::std::os::raw::c_void,
    ) -> Boolean,
>;
pub type CFBagHashCallBack =
    ::std::option::Option<unsafe extern "C" fn(value: *const ::std::os::raw::c_void) -> CFHashCode>;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CFBagCallBacks {
    pub version: CFIndex,
    pub retain: CFBagRetainCallBack,
    pub release: CFBagReleaseCallBack,
    pub copyDescription: CFBagCopyDescriptionCallBack,
    pub equal: CFBagEqualCallBack,
    pub hash: CFBagHashCallBack,
}
pub type CFBagApplierFunction = ::std::option::Option<
    unsafe extern "C" fn(
        value: *const ::std::os::raw::c_void,
        context: *mut ::std::os::raw::c_void,
    ),
>;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __CFBag {
    _unused: [u8; 0],
}
pub type CFBagRef = *const __CFBag;
pub type CFMutableBagRef = *mut __CFBag;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CFBinaryHeapCompareContext {
    pub version: CFIndex,
    pub info: *mut ::std::os::raw::c_void,
    pub retain: ::std::option::Option<
        unsafe extern "C" fn(info: *const ::std::os::raw::c_void) -> *const ::std::os::raw::c_void,
    >,
    pub release: ::std::option::Option<unsafe extern "C" fn(info: *const ::std::os::raw::c_void)>,
    pub copyDescription: ::std::option::Option<
        unsafe extern "C" fn(info: *const ::std::os::raw::c_void) -> CFStringRef,
    >,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CFBinaryHeapCallBacks {
    pub version: CFIndex,
    pub retain: ::std::option::Option<
        unsafe extern "C" fn(
            allocator: CFAllocatorRef,
            ptr: *const ::std::os::raw::c_void,
        ) -> *const ::std::os::raw::c_void,
    >,
    pub release: ::std::option::Option<
        unsafe extern "C" fn(allocator: CFAllocatorRef, ptr: *const ::std::os::raw::c_void),
    >,
    pub copyDescription: ::std::option::Option<
        unsafe extern "C" fn(ptr: *const ::std::os::raw::c_void) -> CFStringRef,
    >,
    pub compare: ::std::option::Option<
        unsafe extern "C" fn(
            ptr1: *const ::std::os::raw::c_void,
            ptr2: *const ::std::os::raw::c_void,
            context: *mut ::std::os::raw::c_void,
        ) -> CFComparisonResult,
    >,
}
pub type CFBinaryHeapApplierFunction = ::std::option::Option<
    unsafe extern "C" fn(val: *const ::std::os::raw::c_void, context: *mut ::std::os::raw::c_void),
>;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __CFBinaryHeap {
    _unused: [u8; 0],
}
pub type CFBinaryHeapRef = *mut __CFBinaryHeap;
pub type CFBit = UInt32;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __CFBitVector {
    _unused: [u8; 0],
}
pub type CFBitVectorRef = *const __CFBitVector;
pub type CFMutableBitVectorRef = *mut __CFBitVector;
pub type CFByteOrder = CFIndex;
pub type CFDictionaryRetainCallBack = ::std::option::Option<
    unsafe extern "C" fn(
        allocator: CFAllocatorRef,
        value: *const ::std::os::raw::c_void,
    ) -> *const ::std::os::raw::c_void,
>;
pub type CFDictionaryReleaseCallBack = ::std::option::Option<
    unsafe extern "C" fn(allocator: CFAllocatorRef, value: *const ::std::os::raw::c_void),
>;
pub type CFDictionaryCopyDescriptionCallBack = ::std::option::Option<
    unsafe extern "C" fn(value: *const ::std::os::raw::c_void) -> CFStringRef,
>;
pub type CFDictionaryEqualCallBack = ::std::option::Option<
    unsafe extern "C" fn(
        value1: *const ::std::os::raw::c_void,
        value2: *const ::std::os::raw::c_void,
    ) -> Boolean,
>;
pub type CFDictionaryHashCallBack =
    ::std::option::Option<unsafe extern "C" fn(value: *const ::std::os::raw::c_void) -> CFHashCode>;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CFDictionaryKeyCallBacks {
    pub version: CFIndex,
    pub retain: CFDictionaryRetainCallBack,
    pub release: CFDictionaryReleaseCallBack,
    pub copyDescription: CFDictionaryCopyDescriptionCallBack,
    pub equal: CFDictionaryEqualCallBack,
    pub hash: CFDictionaryHashCallBack,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CFDictionaryValueCallBacks {
    pub version: CFIndex,
    pub retain: CFDictionaryRetainCallBack,
    pub release: CFDictionaryReleaseCallBack,
    pub copyDescription: CFDictionaryCopyDescriptionCallBack,
    pub equal: CFDictionaryEqualCallBack,
}
pub type CFDictionaryApplierFunction = ::std::option::Option<
    unsafe extern "C" fn(
        key: *const ::std::os::raw::c_void,
        value: *const ::std::os::raw::c_void,
        context: *mut ::std::os::raw::c_void,
    ),
>;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __CFDictionary {
    _unused: [u8; 0],
}
pub type CFDictionaryRef = *const __CFDictionary;
pub type CFMutableDictionaryRef = *mut __CFDictionary;
pub type CFNotificationName = CFStringRef;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __CFNotificationCenter {
    _unused: [u8; 0],
}
pub type CFNotificationCenterRef = *mut __CFNotificationCenter;
pub type CFNotificationCallback = ::std::option::Option<
    unsafe extern "C" fn(
        center: CFNotificationCenterRef,
        observer: *mut ::std::os::raw::c_void,
        name: CFNotificationName,
        object: *const ::std::os::raw::c_void,
        userInfo: CFDictionaryRef,
    ),
>;
pub type CFNotificationSuspensionBehavior = CFIndex;
pub type CFLocaleIdentifier = CFStringRef;
pub type CFLocaleKey = CFStringRef;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __CFLocale {
    _unused: [u8; 0],
}
pub type CFLocaleRef = *const __CFLocale;
pub type CFLocaleLanguageDirection = CFIndex;
pub type CFCalendarIdentifier = CFStringRef;
pub type CFTimeInterval = f64;
pub type CFAbsoluteTime = CFTimeInterval;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __CFDate {
    _unused: [u8; 0],
}
pub type CFDateRef = *const __CFDate;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __CFTimeZone {
    _unused: [u8; 0],
}
pub type CFTimeZoneRef = *const __CFTimeZone;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CFGregorianDate {
    pub year: SInt32,
    pub month: SInt8,
    pub day: SInt8,
    pub hour: SInt8,
    pub minute: SInt8,
    pub second: f64,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CFGregorianUnits {
    pub years: SInt32,
    pub months: SInt32,
    pub days: SInt32,
    pub hours: SInt32,
    pub minutes: SInt32,
    pub seconds: f64,
}
pub type CFGregorianUnitFlags = CFOptionFlags;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __CFData {
    _unused: [u8; 0],
}
pub type CFDataRef = *const __CFData;
pub type CFMutableDataRef = *mut __CFData;
pub type CFDataSearchFlags = CFOptionFlags;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __CFCharacterSet {
    _unused: [u8; 0],
}
pub type CFCharacterSetRef = *const __CFCharacterSet;
pub type CFMutableCharacterSetRef = *mut __CFCharacterSet;
pub type CFCharacterSetPredefinedSet = CFIndex;
pub type CFErrorDomain = CFStringRef;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __CFError {
    _unused: [u8; 0],
}
pub type CFErrorRef = *mut __CFError;
pub type CFStringEncoding = UInt32;
pub type CFStringBuiltInEncodings = CFStringEncoding;
pub type CFStringCompareFlags = CFOptionFlags;
pub type CFStringNormalizationForm = CFIndex;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CFStringInlineBuffer {
    pub buffer: [UniChar; 64usize],
    pub theString: CFStringRef,
    pub directUniCharBuffer: *const UniChar,
    pub directCStringBuffer: *const ::std::os::raw::c_char,
    pub rangeToBuffer: CFRange,
    pub bufferedRangeStart: CFIndex,
    pub bufferedRangeEnd: CFIndex,
}
pub type CFTimeZoneNameStyle = CFIndex;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __CFCalendar {
    _unused: [u8; 0],
}
pub type CFCalendarRef = *mut __CFCalendar;
pub type CFCalendarUnit = CFOptionFlags;
pub type CGFloat = f64;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CGPoint {
    pub x: CGFloat,
    pub y: CGFloat,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CGSize {
    pub width: CGFloat,
    pub height: CGFloat,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CGVector {
    pub dx: CGFloat,
    pub dy: CGFloat,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CGRect {
    pub origin: CGPoint,
    pub size: CGSize,
}
pub type CGRectEdge = u32;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CGAffineTransform {
    pub a: CGFloat,
    pub b: CGFloat,
    pub c: CGFloat,
    pub d: CGFloat,
    pub tx: CGFloat,
    pub ty: CGFloat,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CGAffineTransformComponents {
    pub scale: CGSize,
    pub horizontalShear: CGFloat,
    pub rotation: CGFloat,
    pub translation: CGVector,
}
pub type CFDateFormatterKey = CFStringRef;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __CFDateFormatter {
    _unused: [u8; 0],
}
pub type CFDateFormatterRef = *mut __CFDateFormatter;
pub type CFDateFormatterStyle = CFIndex;
pub type CFISO8601DateFormatOptions = CFOptionFlags;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __CFBoolean {
    _unused: [u8; 0],
}
pub type CFBooleanRef = *const __CFBoolean;
pub type CFNumberType = CFIndex;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __CFNumber {
    _unused: [u8; 0],
}
pub type CFNumberRef = *const __CFNumber;
pub type CFNumberFormatterKey = CFStringRef;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __CFNumberFormatter {
    _unused: [u8; 0],
}
pub type CFNumberFormatterRef = *mut __CFNumberFormatter;
pub type CFNumberFormatterStyle = CFIndex;
pub type CFNumberFormatterOptionFlags = CFOptionFlags;
pub type CFNumberFormatterRoundingMode = CFIndex;
pub type CFNumberFormatterPadPosition = CFIndex;
pub type CFURLPathStyle = CFIndex;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __CFURL {
    _unused: [u8; 0],
}
pub type CFURLRef = *const __CFURL;
pub type CFURLComponentType = CFIndex;
pub type CFURLBookmarkCreationOptions = CFOptionFlags;
pub type CFURLBookmarkResolutionOptions = CFOptionFlags;
pub type CFURLBookmarkFileCreationOptions = CFOptionFlags;
pub type CFRunLoopMode = CFStringRef;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __CFRunLoop {
    _unused: [u8; 0],
}
pub type CFRunLoopRef = *mut __CFRunLoop;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __CFRunLoopSource {
    _unused: [u8; 0],
}
pub type CFRunLoopSourceRef = *mut __CFRunLoopSource;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __CFRunLoopObserver {
    _unused: [u8; 0],
}
pub type CFRunLoopObserverRef = *mut __CFRunLoopObserver;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __CFRunLoopTimer {
    _unused: [u8; 0],
}
pub type CFRunLoopTimerRef = *mut __CFRunLoopTimer;
pub type CFRunLoopRunResult = SInt32;
pub type CFRunLoopActivity = CFOptionFlags;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CFRunLoopSourceContext {
    pub version: CFIndex,
    pub info: *mut ::std::os::raw::c_void,
    pub retain: ::std::option::Option<
        unsafe extern "C" fn(info: *const ::std::os::raw::c_void) -> *const ::std::os::raw::c_void,
    >,
    pub release: ::std::option::Option<unsafe extern "C" fn(info: *const ::std::os::raw::c_void)>,
    pub copyDescription: ::std::option::Option<
        unsafe extern "C" fn(info: *const ::std::os::raw::c_void) -> CFStringRef,
    >,
    pub equal: ::std::option::Option<
        unsafe extern "C" fn(
            info1: *const ::std::os::raw::c_void,
            info2: *const ::std::os::raw::c_void,
        ) -> Boolean,
    >,
    pub hash: ::std::option::Option<
        unsafe extern "C" fn(info: *const ::std::os::raw::c_void) -> CFHashCode,
    >,
    pub schedule: ::std::option::Option<
        unsafe extern "C" fn(
            info: *mut ::std::os::raw::c_void,
            rl: CFRunLoopRef,
            mode: CFRunLoopMode,
        ),
    >,
    pub cancel: ::std::option::Option<
        unsafe extern "C" fn(
            info: *mut ::std::os::raw::c_void,
            rl: CFRunLoopRef,
            mode: CFRunLoopMode,
        ),
    >,
    pub perform: ::std::option::Option<unsafe extern "C" fn(info: *mut ::std::os::raw::c_void)>,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CFRunLoopSourceContext1 {
    pub version: CFIndex,
    pub info: *mut ::std::os::raw::c_void,
    pub retain: ::std::option::Option<
        unsafe extern "C" fn(info: *const ::std::os::raw::c_void) -> *const ::std::os::raw::c_void,
    >,
    pub release: ::std::option::Option<unsafe extern "C" fn(info: *const ::std::os::raw::c_void)>,
    pub copyDescription: ::std::option::Option<
        unsafe extern "C" fn(info: *const ::std::os::raw::c_void) -> CFStringRef,
    >,
    pub equal: ::std::option::Option<
        unsafe extern "C" fn(
            info1: *const ::std::os::raw::c_void,
            info2: *const ::std::os::raw::c_void,
        ) -> Boolean,
    >,
    pub hash: ::std::option::Option<
        unsafe extern "C" fn(info: *const ::std::os::raw::c_void) -> CFHashCode,
    >,
    pub getPort: ::std::option::Option<
        unsafe extern "C" fn(info: *mut ::std::os::raw::c_void) -> mach_port_t,
    >,
    pub perform: ::std::option::Option<
        unsafe extern "C" fn(
            msg: *mut ::std::os::raw::c_void,
            size: CFIndex,
            allocator: CFAllocatorRef,
            info: *mut ::std::os::raw::c_void,
        ) -> *mut ::std::os::raw::c_void,
    >,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CFRunLoopObserverContext {
    pub version: CFIndex,
    pub info: *mut ::std::os::raw::c_void,
    pub retain: ::std::option::Option<
        unsafe extern "C" fn(info: *const ::std::os::raw::c_void) -> *const ::std::os::raw::c_void,
    >,
    pub release: ::std::option::Option<unsafe extern "C" fn(info: *const ::std::os::raw::c_void)>,
    pub copyDescription: ::std::option::Option<
        unsafe extern "C" fn(info: *const ::std::os::raw::c_void) -> CFStringRef,
    >,
}
pub type CFRunLoopObserverCallBack = ::std::option::Option<
    unsafe extern "C" fn(
        observer: CFRunLoopObserverRef,
        activity: CFRunLoopActivity,
        info: *mut ::std::os::raw::c_void,
    ),
>;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CFRunLoopTimerContext {
    pub version: CFIndex,
    pub info: *mut ::std::os::raw::c_void,
    pub retain: ::std::option::Option<
        unsafe extern "C" fn(info: *const ::std::os::raw::c_void) -> *const ::std::os::raw::c_void,
    >,
    pub release: ::std::option::Option<unsafe extern "C" fn(info: *const ::std::os::raw::c_void)>,
    pub copyDescription: ::std::option::Option<
        unsafe extern "C" fn(info: *const ::std::os::raw::c_void) -> CFStringRef,
    >,
}
pub type CFRunLoopTimerCallBack = ::std::option::Option<
    unsafe extern "C" fn(timer: CFRunLoopTimerRef, info: *mut ::std::os::raw::c_void),
>;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __CFSocket {
    _unused: [u8; 0],
}
pub type CFSocketRef = *mut __CFSocket;
pub type CFSocketError = CFIndex;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CFSocketSignature {
    pub protocolFamily: SInt32,
    pub socketType: SInt32,
    pub protocol: SInt32,
    pub address: CFDataRef,
}
pub type CFSocketCallBackType = CFOptionFlags;
pub type CFSocketCallBack = ::std::option::Option<
    unsafe extern "C" fn(
        s: CFSocketRef,
        type_: CFSocketCallBackType,
        address: CFDataRef,
        data: *const ::std::os::raw::c_void,
        info: *mut ::std::os::raw::c_void,
    ),
>;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CFSocketContext {
    pub version: CFIndex,
    pub info: *mut ::std::os::raw::c_void,
    pub retain: ::std::option::Option<
        unsafe extern "C" fn(info: *const ::std::os::raw::c_void) -> *const ::std::os::raw::c_void,
    >,
    pub release: ::std::option::Option<unsafe extern "C" fn(info: *const ::std::os::raw::c_void)>,
    pub copyDescription: ::std::option::Option<
        unsafe extern "C" fn(info: *const ::std::os::raw::c_void) -> CFStringRef,
    >,
}
pub type CFSocketNativeHandle = ::std::os::raw::c_int;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CFStreamError {
    pub domain: CFIndex,
    pub error: SInt32,
}
pub type CFStreamPropertyKey = CFStringRef;
pub type CFStreamStatus = CFIndex;
pub type CFStreamEventType = CFOptionFlags;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CFStreamClientContext {
    pub version: CFIndex,
    pub info: *mut ::std::os::raw::c_void,
    pub retain: ::std::option::Option<
        unsafe extern "C" fn(info: *mut ::std::os::raw::c_void) -> *mut ::std::os::raw::c_void,
    >,
    pub release: ::std::option::Option<unsafe extern "C" fn(info: *mut ::std::os::raw::c_void)>,
    pub copyDescription: ::std::option::Option<
        unsafe extern "C" fn(info: *mut ::std::os::raw::c_void) -> CFStringRef,
    >,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __CFReadStream {
    _unused: [u8; 0],
}
pub type CFReadStreamRef = *mut __CFReadStream;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __CFWriteStream {
    _unused: [u8; 0],
}
pub type CFWriteStreamRef = *mut __CFWriteStream;
pub type CFReadStreamClientCallBack = ::std::option::Option<
    unsafe extern "C" fn(
        stream: CFReadStreamRef,
        type_: CFStreamEventType,
        clientCallBackInfo: *mut ::std::os::raw::c_void,
    ),
>;
pub type CFWriteStreamClientCallBack = ::std::option::Option<
    unsafe extern "C" fn(
        stream: CFWriteStreamRef,
        type_: CFStreamEventType,
        clientCallBackInfo: *mut ::std::os::raw::c_void,
    ),
>;
pub type CFStreamErrorDomain = CFIndex;
pub type CFPropertyListMutabilityOptions = CFOptionFlags;
pub type CFPropertyListFormat = CFIndex;
pub type CFSetRetainCallBack = ::std::option::Option<
    unsafe extern "C" fn(
        allocator: CFAllocatorRef,
        value: *const ::std::os::raw::c_void,
    ) -> *const ::std::os::raw::c_void,
>;
pub type CFSetReleaseCallBack = ::std::option::Option<
    unsafe extern "C" fn(allocator: CFAllocatorRef, value: *const ::std::os::raw::c_void),
>;
pub type CFSetCopyDescriptionCallBack = ::std::option::Option<
    unsafe extern "C" fn(value: *const ::std::os::raw::c_void) -> CFStringRef,
>;
pub type CFSetEqualCallBack = ::std::option::Option<
    unsafe extern "C" fn(
        value1: *const ::std::os::raw::c_void,
        value2: *const ::std::os::raw::c_void,
    ) -> Boolean,
>;
pub type CFSetHashCallBack =
    ::std::option::Option<unsafe extern "C" fn(value: *const ::std::os::raw::c_void) -> CFHashCode>;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CFSetCallBacks {
    pub version: CFIndex,
    pub retain: CFSetRetainCallBack,
    pub release: CFSetReleaseCallBack,
    pub copyDescription: CFSetCopyDescriptionCallBack,
    pub equal: CFSetEqualCallBack,
    pub hash: CFSetHashCallBack,
}
pub type CFSetApplierFunction = ::std::option::Option<
    unsafe extern "C" fn(
        value: *const ::std::os::raw::c_void,
        context: *mut ::std::os::raw::c_void,
    ),
>;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __CFSet {
    _unused: [u8; 0],
}
pub type CFSetRef = *const __CFSet;
pub type CFMutableSetRef = *mut __CFSet;
pub type CFStringEncodings = CFIndex;
pub type CFTreeRetainCallBack = ::std::option::Option<
    unsafe extern "C" fn(info: *const ::std::os::raw::c_void) -> *const ::std::os::raw::c_void,
>;
pub type CFTreeReleaseCallBack =
    ::std::option::Option<unsafe extern "C" fn(info: *const ::std::os::raw::c_void)>;
pub type CFTreeCopyDescriptionCallBack =
    ::std::option::Option<unsafe extern "C" fn(info: *const ::std::os::raw::c_void) -> CFStringRef>;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CFTreeContext {
    pub version: CFIndex,
    pub info: *mut ::std::os::raw::c_void,
    pub retain: CFTreeRetainCallBack,
    pub release: CFTreeReleaseCallBack,
    pub copyDescription: CFTreeCopyDescriptionCallBack,
}
pub type CFTreeApplierFunction = ::std::option::Option<
    unsafe extern "C" fn(
        value: *const ::std::os::raw::c_void,
        context: *mut ::std::os::raw::c_void,
    ),
>;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __CFTree {
    _unused: [u8; 0],
}
pub type CFTreeRef = *mut __CFTree;
pub type CFURLError = CFIndex;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __CFUUID {
    _unused: [u8; 0],
}
pub type CFUUIDRef = *const __CFUUID;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CFUUIDBytes {
    pub byte0: UInt8,
    pub byte1: UInt8,
    pub byte2: UInt8,
    pub byte3: UInt8,
    pub byte4: UInt8,
    pub byte5: UInt8,
    pub byte6: UInt8,
    pub byte7: UInt8,
    pub byte8: UInt8,
    pub byte9: UInt8,
    pub byte10: UInt8,
    pub byte11: UInt8,
    pub byte12: UInt8,
    pub byte13: UInt8,
    pub byte14: UInt8,
    pub byte15: UInt8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __CFBundle {
    _unused: [u8; 0],
}
pub type CFBundleRef = *mut __CFBundle;
pub type CFPlugInRef = *mut __CFBundle;
pub type CFBundleRefNum = ::std::os::raw::c_int;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __CFMessagePort {
    _unused: [u8; 0],
}
pub type CFMessagePortRef = *mut __CFMessagePort;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CFMessagePortContext {
    pub version: CFIndex,
    pub info: *mut ::std::os::raw::c_void,
    pub retain: ::std::option::Option<
        unsafe extern "C" fn(info: *const ::std::os::raw::c_void) -> *const ::std::os::raw::c_void,
    >,
    pub release: ::std::option::Option<unsafe extern "C" fn(info: *const ::std::os::raw::c_void)>,
    pub copyDescription: ::std::option::Option<
        unsafe extern "C" fn(info: *const ::std::os::raw::c_void) -> CFStringRef,
    >,
}
pub type CFMessagePortCallBack = ::std::option::Option<
    unsafe extern "C" fn(
        local: CFMessagePortRef,
        msgid: SInt32,
        data: CFDataRef,
        info: *mut ::std::os::raw::c_void,
    ) -> CFDataRef,
>;
pub type CFMessagePortInvalidationCallBack = ::std::option::Option<
    unsafe extern "C" fn(ms: CFMessagePortRef, info: *mut ::std::os::raw::c_void),
>;
pub type CFPlugInFactoryFunction = ::std::option::Option<
    unsafe extern "C" fn(
        allocator: CFAllocatorRef,
        typeUUID: CFUUIDRef,
    ) -> *mut ::std::os::raw::c_void,
>;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __CFPlugInInstance {
    _unused: [u8; 0],
}
pub type CFPlugInInstanceRef = *mut __CFPlugInInstance;
pub type CFPlugInInstanceGetInterfaceFunction = ::std::option::Option<
    unsafe extern "C" fn(
        instance: CFPlugInInstanceRef,
        interfaceName: CFStringRef,
        ftbl: *mut *mut ::std::os::raw::c_void,
    ) -> Boolean,
>;
pub type CFPlugInInstanceDeallocateInstanceDataFunction =
    ::std::option::Option<unsafe extern "C" fn(instanceData: *mut ::std::os::raw::c_void)>;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __CFMachPort {
    _unused: [u8; 0],
}
pub type CFMachPortRef = *mut __CFMachPort;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CFMachPortContext {
    pub version: CFIndex,
    pub info: *mut ::std::os::raw::c_void,
    pub retain: ::std::option::Option<
        unsafe extern "C" fn(info: *const ::std::os::raw::c_void) -> *const ::std::os::raw::c_void,
    >,
    pub release: ::std::option::Option<unsafe extern "C" fn(info: *const ::std::os::raw::c_void)>,
    pub copyDescription: ::std::option::Option<
        unsafe extern "C" fn(info: *const ::std::os::raw::c_void) -> CFStringRef,
    >,
}
pub type CFMachPortCallBack = ::std::option::Option<
    unsafe extern "C" fn(
        port: CFMachPortRef,
        msg: *mut ::std::os::raw::c_void,
        size: CFIndex,
        info: *mut ::std::os::raw::c_void,
    ),
>;
pub type CFMachPortInvalidationCallBack = ::std::option::Option<
    unsafe extern "C" fn(port: CFMachPortRef, info: *mut ::std::os::raw::c_void),
>;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __CFAttributedString {
    _unused: [u8; 0],
}
pub type CFAttributedStringRef = *const __CFAttributedString;
pub type CFMutableAttributedStringRef = *mut __CFAttributedString;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __CFURLEnumerator {
    _unused: [u8; 0],
}
pub type CFURLEnumeratorRef = *const __CFURLEnumerator;
pub type CFURLEnumeratorOptions = CFOptionFlags;
pub type CFURLEnumeratorResult = CFIndex;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __CFFileSecurity {
    _unused: [u8; 0],
}
pub type CFFileSecurityRef = *mut __CFFileSecurity;
pub type CFFileSecurityClearOptions = CFOptionFlags;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __CFStringTokenizer {
    _unused: [u8; 0],
}
pub type CFStringTokenizerRef = *mut __CFStringTokenizer;
pub type CFStringTokenizerTokenType = CFOptionFlags;
pub type CFFileDescriptorNativeDescriptor = ::std::os::raw::c_int;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __CFFileDescriptor {
    _unused: [u8; 0],
}
pub type CFFileDescriptorRef = *mut __CFFileDescriptor;
pub type CFFileDescriptorCallBack = ::std::option::Option<
    unsafe extern "C" fn(
        f: CFFileDescriptorRef,
        callBackTypes: CFOptionFlags,
        info: *mut ::std::os::raw::c_void,
    ),
>;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CFFileDescriptorContext {
    pub version: CFIndex,
    pub info: *mut ::std::os::raw::c_void,
    pub retain: ::std::option::Option<
        unsafe extern "C" fn(info: *mut ::std::os::raw::c_void) -> *mut ::std::os::raw::c_void,
    >,
    pub release: ::std::option::Option<unsafe extern "C" fn(info: *mut ::std::os::raw::c_void)>,
    pub copyDescription: ::std::option::Option<
        unsafe extern "C" fn(info: *mut ::std::os::raw::c_void) -> CFStringRef,
    >,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __CFUserNotification {
    _unused: [u8; 0],
}
pub type CFUserNotificationRef = *mut __CFUserNotification;
pub type CFUserNotificationCallBack = ::std::option::Option<
    unsafe extern "C" fn(userNotification: CFUserNotificationRef, responseFlags: CFOptionFlags),
>;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __CFXMLNode {
    _unused: [u8; 0],
}
pub type CFXMLNodeRef = *const __CFXMLNode;
pub type CFXMLTreeRef = CFTreeRef;
pub type CFXMLNodeTypeCode = CFIndex;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CFXMLElementInfo {
    pub attributes: CFDictionaryRef,
    pub attributeOrder: CFArrayRef,
    pub isEmpty: Boolean,
    pub _reserved: [::std::os::raw::c_char; 3usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CFXMLProcessingInstructionInfo {
    pub dataString: CFStringRef,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CFXMLDocumentInfo {
    pub sourceURL: CFURLRef,
    pub encoding: CFStringEncoding,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CFXMLExternalID {
    pub systemID: CFURLRef,
    pub publicID: CFStringRef,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CFXMLDocumentTypeInfo {
    pub externalID: CFXMLExternalID,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CFXMLNotationInfo {
    pub externalID: CFXMLExternalID,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CFXMLElementTypeDeclarationInfo {
    pub contentDescription: CFStringRef,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CFXMLAttributeDeclarationInfo {
    pub attributeName: CFStringRef,
    pub typeString: CFStringRef,
    pub defaultString: CFStringRef,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CFXMLAttributeListDeclarationInfo {
    pub numberOfAttributes: CFIndex,
    pub attributes: *mut CFXMLAttributeDeclarationInfo,
}
pub type CFXMLEntityTypeCode = CFIndex;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CFXMLEntityInfo {
    pub entityType: CFXMLEntityTypeCode,
    pub replacementText: CFStringRef,
    pub entityID: CFXMLExternalID,
    pub notationName: CFStringRef,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CFXMLEntityReferenceInfo {
    pub entityType: CFXMLEntityTypeCode,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __CFXMLParser {
    _unused: [u8; 0],
}
pub type CFXMLParserRef = *mut __CFXMLParser;
pub type CFXMLParserOptions = CFOptionFlags;
pub type CFXMLParserStatusCode = CFIndex;
pub type CFXMLParserCreateXMLStructureCallBack = ::std::option::Option<
    unsafe extern "C" fn(
        parser: CFXMLParserRef,
        nodeDesc: CFXMLNodeRef,
        info: *mut ::std::os::raw::c_void,
    ) -> *mut ::std::os::raw::c_void,
>;
pub type CFXMLParserAddChildCallBack = ::std::option::Option<
    unsafe extern "C" fn(
        parser: CFXMLParserRef,
        parent: *mut ::std::os::raw::c_void,
        child: *mut ::std::os::raw::c_void,
        info: *mut ::std::os::raw::c_void,
    ),
>;
pub type CFXMLParserEndXMLStructureCallBack = ::std::option::Option<
    unsafe extern "C" fn(
        parser: CFXMLParserRef,
        xmlType: *mut ::std::os::raw::c_void,
        info: *mut ::std::os::raw::c_void,
    ),
>;
pub type CFXMLParserResolveExternalEntityCallBack = ::std::option::Option<
    unsafe extern "C" fn(
        parser: CFXMLParserRef,
        extID: *mut CFXMLExternalID,
        info: *mut ::std::os::raw::c_void,
    ) -> CFDataRef,
>;
pub type CFXMLParserHandleErrorCallBack = ::std::option::Option<
    unsafe extern "C" fn(
        parser: CFXMLParserRef,
        error: CFXMLParserStatusCode,
        info: *mut ::std::os::raw::c_void,
    ) -> Boolean,
>;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CFXMLParserCallBacks {
    pub version: CFIndex,
    pub createXMLStructure: CFXMLParserCreateXMLStructureCallBack,
    pub addChild: CFXMLParserAddChildCallBack,
    pub endXMLStructure: CFXMLParserEndXMLStructureCallBack,
    pub resolveExternalEntity: CFXMLParserResolveExternalEntityCallBack,
    pub handleError: CFXMLParserHandleErrorCallBack,
}
pub type CFXMLParserRetainCallBack = ::std::option::Option<
    unsafe extern "C" fn(info: *const ::std::os::raw::c_void) -> *const ::std::os::raw::c_void,
>;
pub type CFXMLParserReleaseCallBack =
    ::std::option::Option<unsafe extern "C" fn(info: *const ::std::os::raw::c_void)>;
pub type CFXMLParserCopyDescriptionCallBack =
    ::std::option::Option<unsafe extern "C" fn(info: *const ::std::os::raw::c_void) -> CFStringRef>;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CFXMLParserContext {
    pub version: CFIndex,
    pub info: *mut ::std::os::raw::c_void,
    pub retain: CFXMLParserRetainCallBack,
    pub release: CFXMLParserReleaseCallBack,
    pub copyDescription: CFXMLParserCopyDescriptionCallBack,
}
pub type HRESULT = SInt32;
pub type ULONG = UInt32;
pub type LPVOID = *mut ::std::os::raw::c_void;
pub type REFIID = CFUUIDBytes;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct IUnknownVTbl {
    pub _reserved: *mut ::std::os::raw::c_void,
    pub QueryInterface: ::std::option::Option<
        unsafe extern "C" fn(
            thisPointer: *mut ::std::os::raw::c_void,
            iid: REFIID,
            ppv: *mut LPVOID,
        ) -> HRESULT,
    >,
    pub AddRef: ::std::option::Option<
        unsafe extern "C" fn(thisPointer: *mut ::std::os::raw::c_void) -> ULONG,
    >,
    pub Release: ::std::option::Option<
        unsafe extern "C" fn(thisPointer: *mut ::std::os::raw::c_void) -> ULONG,
    >,
}
pub type __builtin_va_list = *mut ::std::os::raw::c_char;
pub type instancetype = id;
unsafe extern "C" {
    pub static mut kCFCoreFoundationVersionNumber: f64;
}
unsafe extern "C" {
    pub fn __CFRangeMake(loc: CFIndex, len: CFIndex) -> CFRange;
}
unsafe extern "C" {
    pub fn CFNullGetTypeID() -> CFTypeID;
}
unsafe extern "C" {
    pub static kCFNull: CFNullRef;
}
unsafe extern "C" {
    pub static kCFAllocatorDefault: CFAllocatorRef;
}
unsafe extern "C" {
    pub static kCFAllocatorSystemDefault: CFAllocatorRef;
}
unsafe extern "C" {
    pub static kCFAllocatorMalloc: CFAllocatorRef;
}
unsafe extern "C" {
    pub static kCFAllocatorMallocZone: CFAllocatorRef;
}
unsafe extern "C" {
    pub static kCFAllocatorNull: CFAllocatorRef;
}
unsafe extern "C" {
    pub static kCFAllocatorUseContext: CFAllocatorRef;
}
unsafe extern "C" {
    pub fn CFAllocatorGetTypeID() -> CFTypeID;
}
unsafe extern "C" {
    pub fn CFAllocatorSetDefault(allocator: CFAllocatorRef);
}
unsafe extern "C" {
    pub fn CFAllocatorGetDefault() -> CFAllocatorRef;
}
unsafe extern "C" {
    pub fn CFAllocatorCreate(
        allocator: CFAllocatorRef,
        context: *mut CFAllocatorContext,
    ) -> CFAllocatorRef;
}
unsafe extern "C" {
    pub fn CFAllocatorCreateWithZone(
        allocator: CFAllocatorRef,
        zone: *mut _malloc_zone_t,
    ) -> CFAllocatorRef;
}
unsafe extern "C" {
    pub fn CFAllocatorAllocateTyped(
        allocator: CFAllocatorRef,
        size: CFIndex,
        descriptor: CFAllocatorTypeID,
        hint: CFOptionFlags,
    ) -> *mut ::std::os::raw::c_void;
}
unsafe extern "C" {
    pub fn CFAllocatorReallocateTyped(
        allocator: CFAllocatorRef,
        ptr: *mut ::std::os::raw::c_void,
        newsize: CFIndex,
        descriptor: CFAllocatorTypeID,
        hint: CFOptionFlags,
    ) -> *mut ::std::os::raw::c_void;
}
unsafe extern "C" {
    pub fn CFAllocatorAllocateBytes(
        allocator: CFAllocatorRef,
        size: CFIndex,
        hint: CFOptionFlags,
    ) -> *mut ::std::os::raw::c_void;
}
unsafe extern "C" {
    pub fn CFAllocatorReallocateBytes(
        allocator: CFAllocatorRef,
        ptr: *mut ::std::os::raw::c_void,
        newsize: CFIndex,
        hint: CFOptionFlags,
    ) -> *mut ::std::os::raw::c_void;
}
unsafe extern "C" {
    pub fn CFAllocatorAllocate(
        allocator: CFAllocatorRef,
        size: CFIndex,
        hint: CFOptionFlags,
    ) -> *mut ::std::os::raw::c_void;
}
unsafe extern "C" {
    pub fn CFAllocatorReallocate(
        allocator: CFAllocatorRef,
        ptr: *mut ::std::os::raw::c_void,
        newsize: CFIndex,
        hint: CFOptionFlags,
    ) -> *mut ::std::os::raw::c_void;
}
unsafe extern "C" {
    pub fn CFAllocatorDeallocate(allocator: CFAllocatorRef, ptr: *mut ::std::os::raw::c_void);
}
unsafe extern "C" {
    pub fn CFAllocatorGetPreferredSizeForSize(
        allocator: CFAllocatorRef,
        size: CFIndex,
        hint: CFOptionFlags,
    ) -> CFIndex;
}
unsafe extern "C" {
    pub fn CFAllocatorGetContext(allocator: CFAllocatorRef, context: *mut CFAllocatorContext);
}
unsafe extern "C" {
    pub fn CFGetTypeID(cf: CFTypeRef) -> CFTypeID;
}
unsafe extern "C" {
    pub fn CFCopyTypeIDDescription(type_id: CFTypeID) -> CFStringRef;
}
unsafe extern "C" {
    pub fn CFRetain(cf: CFTypeRef) -> CFTypeRef;
}
unsafe extern "C" {
    pub fn CFRelease(cf: CFTypeRef);
}
unsafe extern "C" {
    pub fn CFAutorelease(arg: CFTypeRef) -> CFTypeRef;
}
unsafe extern "C" {
    pub fn CFGetRetainCount(cf: CFTypeRef) -> CFIndex;
}
unsafe extern "C" {
    pub fn CFEqual(cf1: CFTypeRef, cf2: CFTypeRef) -> Boolean;
}
unsafe extern "C" {
    pub fn CFHash(cf: CFTypeRef) -> CFHashCode;
}
unsafe extern "C" {
    pub fn CFCopyDescription(cf: CFTypeRef) -> CFStringRef;
}
unsafe extern "C" {
    pub fn CFGetAllocator(cf: CFTypeRef) -> CFAllocatorRef;
}
unsafe extern "C" {
    pub fn CFMakeCollectable(cf: CFTypeRef) -> CFTypeRef;
}
unsafe extern "C" {
    pub static kCFTypeArrayCallBacks: CFArrayCallBacks;
}
unsafe extern "C" {
    pub fn CFArrayGetTypeID() -> CFTypeID;
}
unsafe extern "C" {
    pub fn CFArrayCreate(
        allocator: CFAllocatorRef,
        values: *mut *const ::std::os::raw::c_void,
        numValues: CFIndex,
        callBacks: *const CFArrayCallBacks,
    ) -> CFArrayRef;
}
unsafe extern "C" {
    pub fn CFArrayCreateCopy(allocator: CFAllocatorRef, theArray: CFArrayRef) -> CFArrayRef;
}
unsafe extern "C" {
    pub fn CFArrayCreateMutable(
        allocator: CFAllocatorRef,
        capacity: CFIndex,
        callBacks: *const CFArrayCallBacks,
    ) -> CFMutableArrayRef;
}
unsafe extern "C" {
    pub fn CFArrayCreateMutableCopy(
        allocator: CFAllocatorRef,
        capacity: CFIndex,
        theArray: CFArrayRef,
    ) -> CFMutableArrayRef;
}
unsafe extern "C" {
    pub fn CFArrayGetCount(theArray: CFArrayRef) -> CFIndex;
}
unsafe extern "C" {
    pub fn CFArrayGetCountOfValue(
        theArray: CFArrayRef,
        range: CFRange,
        value: *const ::std::os::raw::c_void,
    ) -> CFIndex;
}
unsafe extern "C" {
    pub fn CFArrayContainsValue(
        theArray: CFArrayRef,
        range: CFRange,
        value: *const ::std::os::raw::c_void,
    ) -> Boolean;
}
unsafe extern "C" {
    pub fn CFArrayGetValueAtIndex(
        theArray: CFArrayRef,
        idx: CFIndex,
    ) -> *const ::std::os::raw::c_void;
}
unsafe extern "C" {
    pub fn CFArrayGetValues(
        theArray: CFArrayRef,
        range: CFRange,
        values: *mut *const ::std::os::raw::c_void,
    );
}
unsafe extern "C" {
    pub fn CFArrayApplyFunction(
        theArray: CFArrayRef,
        range: CFRange,
        applier: CFArrayApplierFunction,
        context: *mut ::std::os::raw::c_void,
    );
}
unsafe extern "C" {
    pub fn CFArrayGetFirstIndexOfValue(
        theArray: CFArrayRef,
        range: CFRange,
        value: *const ::std::os::raw::c_void,
    ) -> CFIndex;
}
unsafe extern "C" {
    pub fn CFArrayGetLastIndexOfValue(
        theArray: CFArrayRef,
        range: CFRange,
        value: *const ::std::os::raw::c_void,
    ) -> CFIndex;
}
unsafe extern "C" {
    pub fn CFArrayBSearchValues(
        theArray: CFArrayRef,
        range: CFRange,
        value: *const ::std::os::raw::c_void,
        comparator: CFComparatorFunction,
        context: *mut ::std::os::raw::c_void,
    ) -> CFIndex;
}
unsafe extern "C" {
    pub fn CFArrayAppendValue(theArray: CFMutableArrayRef, value: *const ::std::os::raw::c_void);
}
unsafe extern "C" {
    pub fn CFArrayInsertValueAtIndex(
        theArray: CFMutableArrayRef,
        idx: CFIndex,
        value: *const ::std::os::raw::c_void,
    );
}
unsafe extern "C" {
    pub fn CFArraySetValueAtIndex(
        theArray: CFMutableArrayRef,
        idx: CFIndex,
        value: *const ::std::os::raw::c_void,
    );
}
unsafe extern "C" {
    pub fn CFArrayRemoveValueAtIndex(theArray: CFMutableArrayRef, idx: CFIndex);
}
unsafe extern "C" {
    pub fn CFArrayRemoveAllValues(theArray: CFMutableArrayRef);
}
unsafe extern "C" {
    pub fn CFArrayReplaceValues(
        theArray: CFMutableArrayRef,
        range: CFRange,
        newValues: *mut *const ::std::os::raw::c_void,
        newCount: CFIndex,
    );
}
unsafe extern "C" {
    pub fn CFArrayExchangeValuesAtIndices(
        theArray: CFMutableArrayRef,
        idx1: CFIndex,
        idx2: CFIndex,
    );
}
unsafe extern "C" {
    pub fn CFArraySortValues(
        theArray: CFMutableArrayRef,
        range: CFRange,
        comparator: CFComparatorFunction,
        context: *mut ::std::os::raw::c_void,
    );
}
unsafe extern "C" {
    pub fn CFArrayAppendArray(
        theArray: CFMutableArrayRef,
        otherArray: CFArrayRef,
        otherRange: CFRange,
    );
}
unsafe extern "C" {
    pub static kCFTypeBagCallBacks: CFBagCallBacks;
}
unsafe extern "C" {
    pub static kCFCopyStringBagCallBacks: CFBagCallBacks;
}
unsafe extern "C" {
    pub fn CFBagGetTypeID() -> CFTypeID;
}
unsafe extern "C" {
    pub fn CFBagCreate(
        allocator: CFAllocatorRef,
        values: *mut *const ::std::os::raw::c_void,
        numValues: CFIndex,
        callBacks: *const CFBagCallBacks,
    ) -> CFBagRef;
}
unsafe extern "C" {
    pub fn CFBagCreateCopy(allocator: CFAllocatorRef, theBag: CFBagRef) -> CFBagRef;
}
unsafe extern "C" {
    pub fn CFBagCreateMutable(
        allocator: CFAllocatorRef,
        capacity: CFIndex,
        callBacks: *const CFBagCallBacks,
    ) -> CFMutableBagRef;
}
unsafe extern "C" {
    pub fn CFBagCreateMutableCopy(
        allocator: CFAllocatorRef,
        capacity: CFIndex,
        theBag: CFBagRef,
    ) -> CFMutableBagRef;
}
unsafe extern "C" {
    pub fn CFBagGetCount(theBag: CFBagRef) -> CFIndex;
}
unsafe extern "C" {
    pub fn CFBagGetCountOfValue(theBag: CFBagRef, value: *const ::std::os::raw::c_void) -> CFIndex;
}
unsafe extern "C" {
    pub fn CFBagContainsValue(theBag: CFBagRef, value: *const ::std::os::raw::c_void) -> Boolean;
}
unsafe extern "C" {
    pub fn CFBagGetValue(
        theBag: CFBagRef,
        value: *const ::std::os::raw::c_void,
    ) -> *const ::std::os::raw::c_void;
}
unsafe extern "C" {
    pub fn CFBagGetValueIfPresent(
        theBag: CFBagRef,
        candidate: *const ::std::os::raw::c_void,
        value: *mut *const ::std::os::raw::c_void,
    ) -> Boolean;
}
unsafe extern "C" {
    pub fn CFBagGetValues(theBag: CFBagRef, values: *mut *const ::std::os::raw::c_void);
}
unsafe extern "C" {
    pub fn CFBagApplyFunction(
        theBag: CFBagRef,
        applier: CFBagApplierFunction,
        context: *mut ::std::os::raw::c_void,
    );
}
unsafe extern "C" {
    pub fn CFBagAddValue(theBag: CFMutableBagRef, value: *const ::std::os::raw::c_void);
}
unsafe extern "C" {
    pub fn CFBagReplaceValue(theBag: CFMutableBagRef, value: *const ::std::os::raw::c_void);
}
unsafe extern "C" {
    pub fn CFBagSetValue(theBag: CFMutableBagRef, value: *const ::std::os::raw::c_void);
}
unsafe extern "C" {
    pub fn CFBagRemoveValue(theBag: CFMutableBagRef, value: *const ::std::os::raw::c_void);
}
unsafe extern "C" {
    pub fn CFBagRemoveAllValues(theBag: CFMutableBagRef);
}
unsafe extern "C" {
    pub static kCFStringBinaryHeapCallBacks: CFBinaryHeapCallBacks;
}
unsafe extern "C" {
    pub fn CFBinaryHeapGetTypeID() -> CFTypeID;
}
unsafe extern "C" {
    pub fn CFBinaryHeapCreate(
        allocator: CFAllocatorRef,
        capacity: CFIndex,
        callBacks: *const CFBinaryHeapCallBacks,
        compareContext: *const CFBinaryHeapCompareContext,
    ) -> CFBinaryHeapRef;
}
unsafe extern "C" {
    pub fn CFBinaryHeapCreateCopy(
        allocator: CFAllocatorRef,
        capacity: CFIndex,
        heap: CFBinaryHeapRef,
    ) -> CFBinaryHeapRef;
}
unsafe extern "C" {
    pub fn CFBinaryHeapGetCount(heap: CFBinaryHeapRef) -> CFIndex;
}
unsafe extern "C" {
    pub fn CFBinaryHeapGetCountOfValue(
        heap: CFBinaryHeapRef,
        value: *const ::std::os::raw::c_void,
    ) -> CFIndex;
}
unsafe extern "C" {
    pub fn CFBinaryHeapContainsValue(
        heap: CFBinaryHeapRef,
        value: *const ::std::os::raw::c_void,
    ) -> Boolean;
}
unsafe extern "C" {
    pub fn CFBinaryHeapGetMinimum(heap: CFBinaryHeapRef) -> *const ::std::os::raw::c_void;
}
unsafe extern "C" {
    pub fn CFBinaryHeapGetMinimumIfPresent(
        heap: CFBinaryHeapRef,
        value: *mut *const ::std::os::raw::c_void,
    ) -> Boolean;
}
unsafe extern "C" {
    pub fn CFBinaryHeapGetValues(heap: CFBinaryHeapRef, values: *mut *const ::std::os::raw::c_void);
}
unsafe extern "C" {
    pub fn CFBinaryHeapApplyFunction(
        heap: CFBinaryHeapRef,
        applier: CFBinaryHeapApplierFunction,
        context: *mut ::std::os::raw::c_void,
    );
}
unsafe extern "C" {
    pub fn CFBinaryHeapAddValue(heap: CFBinaryHeapRef, value: *const ::std::os::raw::c_void);
}
unsafe extern "C" {
    pub fn CFBinaryHeapRemoveMinimumValue(heap: CFBinaryHeapRef);
}
unsafe extern "C" {
    pub fn CFBinaryHeapRemoveAllValues(heap: CFBinaryHeapRef);
}
unsafe extern "C" {
    pub fn CFBitVectorGetTypeID() -> CFTypeID;
}
unsafe extern "C" {
    pub fn CFBitVectorCreate(
        allocator: CFAllocatorRef,
        bytes: *const UInt8,
        numBits: CFIndex,
    ) -> CFBitVectorRef;
}
unsafe extern "C" {
    pub fn CFBitVectorCreateCopy(allocator: CFAllocatorRef, bv: CFBitVectorRef) -> CFBitVectorRef;
}
unsafe extern "C" {
    pub fn CFBitVectorCreateMutable(
        allocator: CFAllocatorRef,
        capacity: CFIndex,
    ) -> CFMutableBitVectorRef;
}
unsafe extern "C" {
    pub fn CFBitVectorCreateMutableCopy(
        allocator: CFAllocatorRef,
        capacity: CFIndex,
        bv: CFBitVectorRef,
    ) -> CFMutableBitVectorRef;
}
unsafe extern "C" {
    pub fn CFBitVectorGetCount(bv: CFBitVectorRef) -> CFIndex;
}
unsafe extern "C" {
    pub fn CFBitVectorGetCountOfBit(bv: CFBitVectorRef, range: CFRange, value: CFBit) -> CFIndex;
}
unsafe extern "C" {
    pub fn CFBitVectorContainsBit(bv: CFBitVectorRef, range: CFRange, value: CFBit) -> Boolean;
}
unsafe extern "C" {
    pub fn CFBitVectorGetBitAtIndex(bv: CFBitVectorRef, idx: CFIndex) -> CFBit;
}
unsafe extern "C" {
    pub fn CFBitVectorGetBits(bv: CFBitVectorRef, range: CFRange, bytes: *mut UInt8);
}
unsafe extern "C" {
    pub fn CFBitVectorGetFirstIndexOfBit(
        bv: CFBitVectorRef,
        range: CFRange,
        value: CFBit,
    ) -> CFIndex;
}
unsafe extern "C" {
    pub fn CFBitVectorGetLastIndexOfBit(
        bv: CFBitVectorRef,
        range: CFRange,
        value: CFBit,
    ) -> CFIndex;
}
unsafe extern "C" {
    pub fn CFBitVectorSetCount(bv: CFMutableBitVectorRef, count: CFIndex);
}
unsafe extern "C" {
    pub fn CFBitVectorFlipBitAtIndex(bv: CFMutableBitVectorRef, idx: CFIndex);
}
unsafe extern "C" {
    pub fn CFBitVectorFlipBits(bv: CFMutableBitVectorRef, range: CFRange);
}
unsafe extern "C" {
    pub fn CFBitVectorSetBitAtIndex(bv: CFMutableBitVectorRef, idx: CFIndex, value: CFBit);
}
unsafe extern "C" {
    pub fn CFBitVectorSetBits(bv: CFMutableBitVectorRef, range: CFRange, value: CFBit);
}
unsafe extern "C" {
    pub fn CFBitVectorSetAllBits(bv: CFMutableBitVectorRef, value: CFBit);
}
unsafe extern "C" {
    pub static kCFTypeDictionaryKeyCallBacks: CFDictionaryKeyCallBacks;
}
unsafe extern "C" {
    pub static kCFCopyStringDictionaryKeyCallBacks: CFDictionaryKeyCallBacks;
}
unsafe extern "C" {
    pub static kCFTypeDictionaryValueCallBacks: CFDictionaryValueCallBacks;
}
unsafe extern "C" {
    pub fn CFDictionaryGetTypeID() -> CFTypeID;
}
unsafe extern "C" {
    pub fn CFDictionaryCreate(
        allocator: CFAllocatorRef,
        keys: *mut *const ::std::os::raw::c_void,
        values: *mut *const ::std::os::raw::c_void,
        numValues: CFIndex,
        keyCallBacks: *const CFDictionaryKeyCallBacks,
        valueCallBacks: *const CFDictionaryValueCallBacks,
    ) -> CFDictionaryRef;
}
unsafe extern "C" {
    pub fn CFDictionaryCreateCopy(
        allocator: CFAllocatorRef,
        theDict: CFDictionaryRef,
    ) -> CFDictionaryRef;
}
unsafe extern "C" {
    pub fn CFDictionaryCreateMutable(
        allocator: CFAllocatorRef,
        capacity: CFIndex,
        keyCallBacks: *const CFDictionaryKeyCallBacks,
        valueCallBacks: *const CFDictionaryValueCallBacks,
    ) -> CFMutableDictionaryRef;
}
unsafe extern "C" {
    pub fn CFDictionaryCreateMutableCopy(
        allocator: CFAllocatorRef,
        capacity: CFIndex,
        theDict: CFDictionaryRef,
    ) -> CFMutableDictionaryRef;
}
unsafe extern "C" {
    pub fn CFDictionaryGetCount(theDict: CFDictionaryRef) -> CFIndex;
}
unsafe extern "C" {
    pub fn CFDictionaryGetCountOfKey(
        theDict: CFDictionaryRef,
        key: *const ::std::os::raw::c_void,
    ) -> CFIndex;
}
unsafe extern "C" {
    pub fn CFDictionaryGetCountOfValue(
        theDict: CFDictionaryRef,
        value: *const ::std::os::raw::c_void,
    ) -> CFIndex;
}
unsafe extern "C" {
    pub fn CFDictionaryContainsKey(
        theDict: CFDictionaryRef,
        key: *const ::std::os::raw::c_void,
    ) -> Boolean;
}
unsafe extern "C" {
    pub fn CFDictionaryContainsValue(
        theDict: CFDictionaryRef,
        value: *const ::std::os::raw::c_void,
    ) -> Boolean;
}
unsafe extern "C" {
    pub fn CFDictionaryGetValue(
        theDict: CFDictionaryRef,
        key: *const ::std::os::raw::c_void,
    ) -> *const ::std::os::raw::c_void;
}
unsafe extern "C" {
    pub fn CFDictionaryGetValueIfPresent(
        theDict: CFDictionaryRef,
        key: *const ::std::os::raw::c_void,
        value: *mut *const ::std::os::raw::c_void,
    ) -> Boolean;
}
unsafe extern "C" {
    pub fn CFDictionaryGetKeysAndValues(
        theDict: CFDictionaryRef,
        keys: *mut *const ::std::os::raw::c_void,
        values: *mut *const ::std::os::raw::c_void,
    );
}
unsafe extern "C" {
    pub fn CFDictionaryApplyFunction(
        theDict: CFDictionaryRef,
        applier: CFDictionaryApplierFunction,
        context: *mut ::std::os::raw::c_void,
    );
}
unsafe extern "C" {
    pub fn CFDictionaryAddValue(
        theDict: CFMutableDictionaryRef,
        key: *const ::std::os::raw::c_void,
        value: *const ::std::os::raw::c_void,
    );
}
unsafe extern "C" {
    pub fn CFDictionarySetValue(
        theDict: CFMutableDictionaryRef,
        key: *const ::std::os::raw::c_void,
        value: *const ::std::os::raw::c_void,
    );
}
unsafe extern "C" {
    pub fn CFDictionaryReplaceValue(
        theDict: CFMutableDictionaryRef,
        key: *const ::std::os::raw::c_void,
        value: *const ::std::os::raw::c_void,
    );
}
unsafe extern "C" {
    pub fn CFDictionaryRemoveValue(
        theDict: CFMutableDictionaryRef,
        key: *const ::std::os::raw::c_void,
    );
}
unsafe extern "C" {
    pub fn CFDictionaryRemoveAllValues(theDict: CFMutableDictionaryRef);
}
unsafe extern "C" {
    pub fn CFNotificationCenterGetTypeID() -> CFTypeID;
}
unsafe extern "C" {
    pub fn CFNotificationCenterGetLocalCenter() -> CFNotificationCenterRef;
}
unsafe extern "C" {
    pub fn CFNotificationCenterGetDistributedCenter() -> CFNotificationCenterRef;
}
unsafe extern "C" {
    pub fn CFNotificationCenterGetDarwinNotifyCenter() -> CFNotificationCenterRef;
}
unsafe extern "C" {
    pub fn CFNotificationCenterAddObserver(
        center: CFNotificationCenterRef,
        observer: *const ::std::os::raw::c_void,
        callBack: CFNotificationCallback,
        name: CFStringRef,
        object: *const ::std::os::raw::c_void,
        suspensionBehavior: CFNotificationSuspensionBehavior,
    );
}
unsafe extern "C" {
    pub fn CFNotificationCenterRemoveObserver(
        center: CFNotificationCenterRef,
        observer: *const ::std::os::raw::c_void,
        name: CFNotificationName,
        object: *const ::std::os::raw::c_void,
    );
}
unsafe extern "C" {
    pub fn CFNotificationCenterRemoveEveryObserver(
        center: CFNotificationCenterRef,
        observer: *const ::std::os::raw::c_void,
    );
}
unsafe extern "C" {
    pub fn CFNotificationCenterPostNotification(
        center: CFNotificationCenterRef,
        name: CFNotificationName,
        object: *const ::std::os::raw::c_void,
        userInfo: CFDictionaryRef,
        deliverImmediately: Boolean,
    );
}
unsafe extern "C" {
    pub fn CFNotificationCenterPostNotificationWithOptions(
        center: CFNotificationCenterRef,
        name: CFNotificationName,
        object: *const ::std::os::raw::c_void,
        userInfo: CFDictionaryRef,
        options: CFOptionFlags,
    );
}
unsafe extern "C" {
    pub fn CFLocaleGetTypeID() -> CFTypeID;
}
unsafe extern "C" {
    pub fn CFLocaleGetSystem() -> CFLocaleRef;
}
unsafe extern "C" {
    pub fn CFLocaleCopyCurrent() -> CFLocaleRef;
}
unsafe extern "C" {
    pub fn CFLocaleCopyAvailableLocaleIdentifiers() -> CFArrayRef;
}
unsafe extern "C" {
    pub fn CFLocaleCopyISOLanguageCodes() -> CFArrayRef;
}
unsafe extern "C" {
    pub fn CFLocaleCopyISOCountryCodes() -> CFArrayRef;
}
unsafe extern "C" {
    pub fn CFLocaleCopyISOCurrencyCodes() -> CFArrayRef;
}
unsafe extern "C" {
    pub fn CFLocaleCopyCommonISOCurrencyCodes() -> CFArrayRef;
}
unsafe extern "C" {
    pub fn CFLocaleCopyPreferredLanguages() -> CFArrayRef;
}
unsafe extern "C" {
    pub fn CFLocaleCreateCanonicalLanguageIdentifierFromString(
        allocator: CFAllocatorRef,
        localeIdentifier: CFStringRef,
    ) -> CFLocaleIdentifier;
}
unsafe extern "C" {
    pub fn CFLocaleCreateCanonicalLocaleIdentifierFromString(
        allocator: CFAllocatorRef,
        localeIdentifier: CFStringRef,
    ) -> CFLocaleIdentifier;
}
unsafe extern "C" {
    pub fn CFLocaleCreateCanonicalLocaleIdentifierFromScriptManagerCodes(
        allocator: CFAllocatorRef,
        lcode: LangCode,
        rcode: RegionCode,
    ) -> CFLocaleIdentifier;
}
unsafe extern "C" {
    pub fn CFLocaleCreateLocaleIdentifierFromWindowsLocaleCode(
        allocator: CFAllocatorRef,
        lcid: u32,
    ) -> CFLocaleIdentifier;
}
unsafe extern "C" {
    pub fn CFLocaleGetWindowsLocaleCodeFromLocaleIdentifier(
        localeIdentifier: CFLocaleIdentifier,
    ) -> u32;
}
unsafe extern "C" {
    pub fn CFLocaleGetLanguageCharacterDirection(
        isoLangCode: CFStringRef,
    ) -> CFLocaleLanguageDirection;
}
unsafe extern "C" {
    pub fn CFLocaleGetLanguageLineDirection(isoLangCode: CFStringRef) -> CFLocaleLanguageDirection;
}
unsafe extern "C" {
    pub fn CFLocaleCreateComponentsFromLocaleIdentifier(
        allocator: CFAllocatorRef,
        localeID: CFLocaleIdentifier,
    ) -> CFDictionaryRef;
}
unsafe extern "C" {
    pub fn CFLocaleCreateLocaleIdentifierFromComponents(
        allocator: CFAllocatorRef,
        dictionary: CFDictionaryRef,
    ) -> CFLocaleIdentifier;
}
unsafe extern "C" {
    pub fn CFLocaleCreate(
        allocator: CFAllocatorRef,
        localeIdentifier: CFLocaleIdentifier,
    ) -> CFLocaleRef;
}
unsafe extern "C" {
    pub fn CFLocaleCreateCopy(allocator: CFAllocatorRef, locale: CFLocaleRef) -> CFLocaleRef;
}
unsafe extern "C" {
    pub fn CFLocaleGetIdentifier(locale: CFLocaleRef) -> CFLocaleIdentifier;
}
unsafe extern "C" {
    pub fn CFLocaleGetValue(locale: CFLocaleRef, key: CFLocaleKey) -> CFTypeRef;
}
unsafe extern "C" {
    pub fn CFLocaleCopyDisplayNameForPropertyValue(
        displayLocale: CFLocaleRef,
        key: CFLocaleKey,
        value: CFStringRef,
    ) -> CFStringRef;
}
unsafe extern "C" {
    pub static kCFLocaleCurrentLocaleDidChangeNotification: CFNotificationName;
}
unsafe extern "C" {
    pub static kCFLocaleIdentifier: CFLocaleKey;
}
unsafe extern "C" {
    pub static kCFLocaleLanguageCode: CFLocaleKey;
}
unsafe extern "C" {
    pub static kCFLocaleCountryCode: CFLocaleKey;
}
unsafe extern "C" {
    pub static kCFLocaleScriptCode: CFLocaleKey;
}
unsafe extern "C" {
    pub static kCFLocaleVariantCode: CFLocaleKey;
}
unsafe extern "C" {
    pub static kCFLocaleExemplarCharacterSet: CFLocaleKey;
}
unsafe extern "C" {
    pub static kCFLocaleCalendarIdentifier: CFLocaleKey;
}
unsafe extern "C" {
    pub static kCFLocaleCalendar: CFLocaleKey;
}
unsafe extern "C" {
    pub static kCFLocaleCollationIdentifier: CFLocaleKey;
}
unsafe extern "C" {
    pub static kCFLocaleUsesMetricSystem: CFLocaleKey;
}
unsafe extern "C" {
    pub static kCFLocaleMeasurementSystem: CFLocaleKey;
}
unsafe extern "C" {
    pub static kCFLocaleDecimalSeparator: CFLocaleKey;
}
unsafe extern "C" {
    pub static kCFLocaleGroupingSeparator: CFLocaleKey;
}
unsafe extern "C" {
    pub static kCFLocaleCurrencySymbol: CFLocaleKey;
}
unsafe extern "C" {
    pub static kCFLocaleCurrencyCode: CFLocaleKey;
}
unsafe extern "C" {
    pub static kCFLocaleCollatorIdentifier: CFLocaleKey;
}
unsafe extern "C" {
    pub static kCFLocaleQuotationBeginDelimiterKey: CFLocaleKey;
}
unsafe extern "C" {
    pub static kCFLocaleQuotationEndDelimiterKey: CFLocaleKey;
}
unsafe extern "C" {
    pub static kCFLocaleAlternateQuotationBeginDelimiterKey: CFLocaleKey;
}
unsafe extern "C" {
    pub static kCFLocaleAlternateQuotationEndDelimiterKey: CFLocaleKey;
}
unsafe extern "C" {
    pub static kCFGregorianCalendar: CFCalendarIdentifier;
}
unsafe extern "C" {
    pub static kCFBuddhistCalendar: CFCalendarIdentifier;
}
unsafe extern "C" {
    pub static kCFChineseCalendar: CFCalendarIdentifier;
}
unsafe extern "C" {
    pub static kCFHebrewCalendar: CFCalendarIdentifier;
}
unsafe extern "C" {
    pub static kCFIslamicCalendar: CFCalendarIdentifier;
}
unsafe extern "C" {
    pub static kCFIslamicCivilCalendar: CFCalendarIdentifier;
}
unsafe extern "C" {
    pub static kCFJapaneseCalendar: CFCalendarIdentifier;
}
unsafe extern "C" {
    pub static kCFRepublicOfChinaCalendar: CFCalendarIdentifier;
}
unsafe extern "C" {
    pub static kCFPersianCalendar: CFCalendarIdentifier;
}
unsafe extern "C" {
    pub static kCFIndianCalendar: CFCalendarIdentifier;
}
unsafe extern "C" {
    pub static kCFISO8601Calendar: CFCalendarIdentifier;
}
unsafe extern "C" {
    pub static kCFIslamicTabularCalendar: CFCalendarIdentifier;
}
unsafe extern "C" {
    pub static kCFIslamicUmmAlQuraCalendar: CFCalendarIdentifier;
}
unsafe extern "C" {
    pub static kCFBanglaCalendar: CFCalendarIdentifier;
}
unsafe extern "C" {
    pub static kCFGujaratiCalendar: CFCalendarIdentifier;
}
unsafe extern "C" {
    pub static kCFKannadaCalendar: CFCalendarIdentifier;
}
unsafe extern "C" {
    pub static kCFMalayalamCalendar: CFCalendarIdentifier;
}
unsafe extern "C" {
    pub static kCFMarathiCalendar: CFCalendarIdentifier;
}
unsafe extern "C" {
    pub static kCFOdiaCalendar: CFCalendarIdentifier;
}
unsafe extern "C" {
    pub static kCFTamilCalendar: CFCalendarIdentifier;
}
unsafe extern "C" {
    pub static kCFTeluguCalendar: CFCalendarIdentifier;
}
unsafe extern "C" {
    pub static kCFVikramCalendar: CFCalendarIdentifier;
}
unsafe extern "C" {
    pub static kCFDangiCalendar: CFCalendarIdentifier;
}
unsafe extern "C" {
    pub static kCFVietnameseCalendar: CFCalendarIdentifier;
}
unsafe extern "C" {
    pub fn CFAbsoluteTimeGetCurrent() -> CFAbsoluteTime;
}
unsafe extern "C" {
    pub static kCFAbsoluteTimeIntervalSince1970: CFTimeInterval;
}
unsafe extern "C" {
    pub static kCFAbsoluteTimeIntervalSince1904: CFTimeInterval;
}
unsafe extern "C" {
    pub fn CFDateGetTypeID() -> CFTypeID;
}
unsafe extern "C" {
    pub fn CFDateCreate(allocator: CFAllocatorRef, at: CFAbsoluteTime) -> CFDateRef;
}
unsafe extern "C" {
    pub fn CFDateGetAbsoluteTime(theDate: CFDateRef) -> CFAbsoluteTime;
}
unsafe extern "C" {
    pub fn CFDateGetTimeIntervalSinceDate(
        theDate: CFDateRef,
        otherDate: CFDateRef,
    ) -> CFTimeInterval;
}
unsafe extern "C" {
    pub fn CFDateCompare(
        theDate: CFDateRef,
        otherDate: CFDateRef,
        context: *mut ::std::os::raw::c_void,
    ) -> CFComparisonResult;
}
unsafe extern "C" {
    pub fn CFGregorianDateIsValid(gdate: CFGregorianDate, unitFlags: CFOptionFlags) -> Boolean;
}
unsafe extern "C" {
    pub fn CFGregorianDateGetAbsoluteTime(
        gdate: CFGregorianDate,
        tz: CFTimeZoneRef,
    ) -> CFAbsoluteTime;
}
unsafe extern "C" {
    pub fn CFAbsoluteTimeGetGregorianDate(at: CFAbsoluteTime, tz: CFTimeZoneRef)
        -> CFGregorianDate;
}
unsafe extern "C" {
    pub fn CFAbsoluteTimeAddGregorianUnits(
        at: CFAbsoluteTime,
        tz: CFTimeZoneRef,
        units: CFGregorianUnits,
    ) -> CFAbsoluteTime;
}
unsafe extern "C" {
    pub fn CFAbsoluteTimeGetDifferenceAsGregorianUnits(
        at1: CFAbsoluteTime,
        at2: CFAbsoluteTime,
        tz: CFTimeZoneRef,
        unitFlags: CFOptionFlags,
    ) -> CFGregorianUnits;
}
unsafe extern "C" {
    pub fn CFAbsoluteTimeGetDayOfWeek(at: CFAbsoluteTime, tz: CFTimeZoneRef) -> SInt32;
}
unsafe extern "C" {
    pub fn CFAbsoluteTimeGetDayOfYear(at: CFAbsoluteTime, tz: CFTimeZoneRef) -> SInt32;
}
unsafe extern "C" {
    pub fn CFAbsoluteTimeGetWeekOfYear(at: CFAbsoluteTime, tz: CFTimeZoneRef) -> SInt32;
}
unsafe extern "C" {
    pub fn CFDataGetTypeID() -> CFTypeID;
}
unsafe extern "C" {
    pub fn CFDataCreate(
        allocator: CFAllocatorRef,
        bytes: *const UInt8,
        length: CFIndex,
    ) -> CFDataRef;
}
unsafe extern "C" {
    pub fn CFDataCreateWithBytesNoCopy(
        allocator: CFAllocatorRef,
        bytes: *const UInt8,
        length: CFIndex,
        bytesDeallocator: CFAllocatorRef,
    ) -> CFDataRef;
}
unsafe extern "C" {
    pub fn CFDataCreateCopy(allocator: CFAllocatorRef, theData: CFDataRef) -> CFDataRef;
}
unsafe extern "C" {
    pub fn CFDataCreateMutable(allocator: CFAllocatorRef, capacity: CFIndex) -> CFMutableDataRef;
}
unsafe extern "C" {
    pub fn CFDataCreateMutableCopy(
        allocator: CFAllocatorRef,
        capacity: CFIndex,
        theData: CFDataRef,
    ) -> CFMutableDataRef;
}
unsafe extern "C" {
    pub fn CFDataGetLength(theData: CFDataRef) -> CFIndex;
}
unsafe extern "C" {
    pub fn CFDataGetBytePtr(theData: CFDataRef) -> *const UInt8;
}
unsafe extern "C" {
    pub fn CFDataGetMutableBytePtr(theData: CFMutableDataRef) -> *mut UInt8;
}
unsafe extern "C" {
    pub fn CFDataGetBytes(theData: CFDataRef, range: CFRange, buffer: *mut UInt8);
}
unsafe extern "C" {
    pub fn CFDataSetLength(theData: CFMutableDataRef, length: CFIndex);
}
unsafe extern "C" {
    pub fn CFDataIncreaseLength(theData: CFMutableDataRef, extraLength: CFIndex);
}
unsafe extern "C" {
    pub fn CFDataAppendBytes(theData: CFMutableDataRef, bytes: *const UInt8, length: CFIndex);
}
unsafe extern "C" {
    pub fn CFDataReplaceBytes(
        theData: CFMutableDataRef,
        range: CFRange,
        newBytes: *const UInt8,
        newLength: CFIndex,
    );
}
unsafe extern "C" {
    pub fn CFDataDeleteBytes(theData: CFMutableDataRef, range: CFRange);
}
unsafe extern "C" {
    pub fn CFDataFind(
        theData: CFDataRef,
        dataToFind: CFDataRef,
        searchRange: CFRange,
        compareOptions: CFDataSearchFlags,
    ) -> CFRange;
}
unsafe extern "C" {
    pub fn CFCharacterSetGetTypeID() -> CFTypeID;
}
unsafe extern "C" {
    pub fn CFCharacterSetGetPredefined(
        theSetIdentifier: CFCharacterSetPredefinedSet,
    ) -> CFCharacterSetRef;
}
unsafe extern "C" {
    pub fn CFCharacterSetCreateWithCharactersInRange(
        alloc: CFAllocatorRef,
        theRange: CFRange,
    ) -> CFCharacterSetRef;
}
unsafe extern "C" {
    pub fn CFCharacterSetCreateWithCharactersInString(
        alloc: CFAllocatorRef,
        theString: CFStringRef,
    ) -> CFCharacterSetRef;
}
unsafe extern "C" {
    pub fn CFCharacterSetCreateWithBitmapRepresentation(
        alloc: CFAllocatorRef,
        theData: CFDataRef,
    ) -> CFCharacterSetRef;
}
unsafe extern "C" {
    pub fn CFCharacterSetCreateInvertedSet(
        alloc: CFAllocatorRef,
        theSet: CFCharacterSetRef,
    ) -> CFCharacterSetRef;
}
unsafe extern "C" {
    pub fn CFCharacterSetIsSupersetOfSet(
        theSet: CFCharacterSetRef,
        theOtherset: CFCharacterSetRef,
    ) -> Boolean;
}
unsafe extern "C" {
    pub fn CFCharacterSetHasMemberInPlane(theSet: CFCharacterSetRef, thePlane: CFIndex) -> Boolean;
}
unsafe extern "C" {
    pub fn CFCharacterSetCreateMutable(alloc: CFAllocatorRef) -> CFMutableCharacterSetRef;
}
unsafe extern "C" {
    pub fn CFCharacterSetCreateCopy(
        alloc: CFAllocatorRef,
        theSet: CFCharacterSetRef,
    ) -> CFCharacterSetRef;
}
unsafe extern "C" {
    pub fn CFCharacterSetCreateMutableCopy(
        alloc: CFAllocatorRef,
        theSet: CFCharacterSetRef,
    ) -> CFMutableCharacterSetRef;
}
unsafe extern "C" {
    pub fn CFCharacterSetIsCharacterMember(theSet: CFCharacterSetRef, theChar: UniChar) -> Boolean;
}
unsafe extern "C" {
    pub fn CFCharacterSetIsLongCharacterMember(
        theSet: CFCharacterSetRef,
        theChar: UTF32Char,
    ) -> Boolean;
}
unsafe extern "C" {
    pub fn CFCharacterSetCreateBitmapRepresentation(
        alloc: CFAllocatorRef,
        theSet: CFCharacterSetRef,
    ) -> CFDataRef;
}
unsafe extern "C" {
    pub fn CFCharacterSetAddCharactersInRange(theSet: CFMutableCharacterSetRef, theRange: CFRange);
}
unsafe extern "C" {
    pub fn CFCharacterSetRemoveCharactersInRange(
        theSet: CFMutableCharacterSetRef,
        theRange: CFRange,
    );
}
unsafe extern "C" {
    pub fn CFCharacterSetAddCharactersInString(
        theSet: CFMutableCharacterSetRef,
        theString: CFStringRef,
    );
}
unsafe extern "C" {
    pub fn CFCharacterSetRemoveCharactersInString(
        theSet: CFMutableCharacterSetRef,
        theString: CFStringRef,
    );
}
unsafe extern "C" {
    pub fn CFCharacterSetUnion(theSet: CFMutableCharacterSetRef, theOtherSet: CFCharacterSetRef);
}
unsafe extern "C" {
    pub fn CFCharacterSetIntersect(
        theSet: CFMutableCharacterSetRef,
        theOtherSet: CFCharacterSetRef,
    );
}
unsafe extern "C" {
    pub fn CFCharacterSetInvert(theSet: CFMutableCharacterSetRef);
}
unsafe extern "C" {
    pub fn CFErrorGetTypeID() -> CFTypeID;
}
unsafe extern "C" {
    pub static kCFErrorDomainPOSIX: CFErrorDomain;
}
unsafe extern "C" {
    pub static kCFErrorDomainOSStatus: CFErrorDomain;
}
unsafe extern "C" {
    pub static kCFErrorDomainMach: CFErrorDomain;
}
unsafe extern "C" {
    pub static kCFErrorDomainCocoa: CFErrorDomain;
}
unsafe extern "C" {
    pub static kCFErrorLocalizedDescriptionKey: CFStringRef;
}
unsafe extern "C" {
    pub static kCFErrorLocalizedFailureKey: CFStringRef;
}
unsafe extern "C" {
    pub static kCFErrorLocalizedFailureReasonKey: CFStringRef;
}
unsafe extern "C" {
    pub static kCFErrorLocalizedRecoverySuggestionKey: CFStringRef;
}
unsafe extern "C" {
    pub static kCFErrorDescriptionKey: CFStringRef;
}
unsafe extern "C" {
    pub static kCFErrorUnderlyingErrorKey: CFStringRef;
}
unsafe extern "C" {
    pub static kCFErrorURLKey: CFStringRef;
}
unsafe extern "C" {
    pub static kCFErrorFilePathKey: CFStringRef;
}
unsafe extern "C" {
    pub fn CFErrorCreate(
        allocator: CFAllocatorRef,
        domain: CFErrorDomain,
        code: CFIndex,
        userInfo: CFDictionaryRef,
    ) -> CFErrorRef;
}
unsafe extern "C" {
    pub fn CFErrorCreateWithUserInfoKeysAndValues(
        allocator: CFAllocatorRef,
        domain: CFErrorDomain,
        code: CFIndex,
        userInfoKeys: *const *const ::std::os::raw::c_void,
        userInfoValues: *const *const ::std::os::raw::c_void,
        numUserInfoValues: CFIndex,
    ) -> CFErrorRef;
}
unsafe extern "C" {
    pub fn CFErrorGetDomain(err: CFErrorRef) -> CFErrorDomain;
}
unsafe extern "C" {
    pub fn CFErrorGetCode(err: CFErrorRef) -> CFIndex;
}
unsafe extern "C" {
    pub fn CFErrorCopyUserInfo(err: CFErrorRef) -> CFDictionaryRef;
}
unsafe extern "C" {
    pub fn CFErrorCopyDescription(err: CFErrorRef) -> CFStringRef;
}
unsafe extern "C" {
    pub fn CFErrorCopyFailureReason(err: CFErrorRef) -> CFStringRef;
}
unsafe extern "C" {
    pub fn CFErrorCopyRecoverySuggestion(err: CFErrorRef) -> CFStringRef;
}
unsafe extern "C" {
    pub fn CFStringGetTypeID() -> CFTypeID;
}
unsafe extern "C" {
    pub fn CFStringCreateWithPascalString(
        alloc: CFAllocatorRef,
        pStr: ConstStr255Param,
        encoding: CFStringEncoding,
    ) -> CFStringRef;
}
unsafe extern "C" {
    pub fn CFStringCreateWithCString(
        alloc: CFAllocatorRef,
        cStr: *const ::std::os::raw::c_char,
        encoding: CFStringEncoding,
    ) -> CFStringRef;
}
unsafe extern "C" {
    pub fn CFStringCreateWithBytes(
        alloc: CFAllocatorRef,
        bytes: *const UInt8,
        numBytes: CFIndex,
        encoding: CFStringEncoding,
        isExternalRepresentation: Boolean,
    ) -> CFStringRef;
}
unsafe extern "C" {
    pub fn CFStringCreateWithCharacters(
        alloc: CFAllocatorRef,
        chars: *const UniChar,
        numChars: CFIndex,
    ) -> CFStringRef;
}
unsafe extern "C" {
    pub fn CFStringCreateWithPascalStringNoCopy(
        alloc: CFAllocatorRef,
        pStr: ConstStr255Param,
        encoding: CFStringEncoding,
        contentsDeallocator: CFAllocatorRef,
    ) -> CFStringRef;
}
unsafe extern "C" {
    pub fn CFStringCreateWithCStringNoCopy(
        alloc: CFAllocatorRef,
        cStr: *const ::std::os::raw::c_char,
        encoding: CFStringEncoding,
        contentsDeallocator: CFAllocatorRef,
    ) -> CFStringRef;
}
unsafe extern "C" {
    pub fn CFStringCreateWithBytesNoCopy(
        alloc: CFAllocatorRef,
        bytes: *const UInt8,
        numBytes: CFIndex,
        encoding: CFStringEncoding,
        isExternalRepresentation: Boolean,
        contentsDeallocator: CFAllocatorRef,
    ) -> CFStringRef;
}
unsafe extern "C" {
    pub fn CFStringCreateWithCharactersNoCopy(
        alloc: CFAllocatorRef,
        chars: *const UniChar,
        numChars: CFIndex,
        contentsDeallocator: CFAllocatorRef,
    ) -> CFStringRef;
}
unsafe extern "C" {
    pub fn CFStringCreateWithSubstring(
        alloc: CFAllocatorRef,
        str_: CFStringRef,
        range: CFRange,
    ) -> CFStringRef;
}
unsafe extern "C" {
    pub fn CFStringCreateCopy(alloc: CFAllocatorRef, theString: CFStringRef) -> CFStringRef;
}
unsafe extern "C" {
    pub fn CFStringCreateWithFormat(
        alloc: CFAllocatorRef,
        formatOptions: CFDictionaryRef,
        format: CFStringRef,
        ...
    ) -> CFStringRef;
}
unsafe extern "C" {
    pub fn CFStringCreateWithFormatAndArguments(
        alloc: CFAllocatorRef,
        formatOptions: CFDictionaryRef,
        format: CFStringRef,
        arguments: va_list,
    ) -> CFStringRef;
}
unsafe extern "C" {
    pub fn CFStringCreateStringWithValidatedFormat(
        alloc: CFAllocatorRef,
        formatOptions: CFDictionaryRef,
        validFormatSpecifiers: CFStringRef,
        format: CFStringRef,
        errorPtr: *mut CFErrorRef,
        ...
    ) -> CFStringRef;
}
unsafe extern "C" {
    pub fn CFStringCreateStringWithValidatedFormatAndArguments(
        alloc: CFAllocatorRef,
        formatOptions: CFDictionaryRef,
        validFormatSpecifiers: CFStringRef,
        format: CFStringRef,
        arguments: va_list,
        errorPtr: *mut CFErrorRef,
    ) -> CFStringRef;
}
unsafe extern "C" {
    pub fn CFStringCreateMutable(alloc: CFAllocatorRef, maxLength: CFIndex) -> CFMutableStringRef;
}
unsafe extern "C" {
    pub fn CFStringCreateMutableCopy(
        alloc: CFAllocatorRef,
        maxLength: CFIndex,
        theString: CFStringRef,
    ) -> CFMutableStringRef;
}
unsafe extern "C" {
    pub fn CFStringCreateMutableWithExternalCharactersNoCopy(
        alloc: CFAllocatorRef,
        chars: *mut UniChar,
        numChars: CFIndex,
        capacity: CFIndex,
        externalCharactersAllocator: CFAllocatorRef,
    ) -> CFMutableStringRef;
}
unsafe extern "C" {
    pub fn CFStringGetLength(theString: CFStringRef) -> CFIndex;
}
unsafe extern "C" {
    pub fn CFStringGetCharacterAtIndex(theString: CFStringRef, idx: CFIndex) -> UniChar;
}
unsafe extern "C" {
    pub fn CFStringGetCharacters(theString: CFStringRef, range: CFRange, buffer: *mut UniChar);
}
unsafe extern "C" {
    pub fn CFStringGetPascalString(
        theString: CFStringRef,
        buffer: StringPtr,
        bufferSize: CFIndex,
        encoding: CFStringEncoding,
    ) -> Boolean;
}
unsafe extern "C" {
    pub fn CFStringGetCString(
        theString: CFStringRef,
        buffer: *mut ::std::os::raw::c_char,
        bufferSize: CFIndex,
        encoding: CFStringEncoding,
    ) -> Boolean;
}
unsafe extern "C" {
    pub fn CFStringGetPascalStringPtr(
        theString: CFStringRef,
        encoding: CFStringEncoding,
    ) -> ConstStringPtr;
}
unsafe extern "C" {
    pub fn CFStringGetCStringPtr(
        theString: CFStringRef,
        encoding: CFStringEncoding,
    ) -> *const ::std::os::raw::c_char;
}
unsafe extern "C" {
    pub fn CFStringGetCharactersPtr(theString: CFStringRef) -> *const UniChar;
}
unsafe extern "C" {
    pub fn CFStringGetBytes(
        theString: CFStringRef,
        range: CFRange,
        encoding: CFStringEncoding,
        lossByte: UInt8,
        isExternalRepresentation: Boolean,
        buffer: *mut UInt8,
        maxBufLen: CFIndex,
        usedBufLen: *mut CFIndex,
    ) -> CFIndex;
}
unsafe extern "C" {
    pub fn CFStringCreateFromExternalRepresentation(
        alloc: CFAllocatorRef,
        data: CFDataRef,
        encoding: CFStringEncoding,
    ) -> CFStringRef;
}
unsafe extern "C" {
    pub fn CFStringCreateExternalRepresentation(
        alloc: CFAllocatorRef,
        theString: CFStringRef,
        encoding: CFStringEncoding,
        lossByte: UInt8,
    ) -> CFDataRef;
}
unsafe extern "C" {
    pub fn CFStringGetSmallestEncoding(theString: CFStringRef) -> CFStringEncoding;
}
unsafe extern "C" {
    pub fn CFStringGetFastestEncoding(theString: CFStringRef) -> CFStringEncoding;
}
unsafe extern "C" {
    pub fn CFStringGetSystemEncoding() -> CFStringEncoding;
}
unsafe extern "C" {
    pub fn CFStringGetMaximumSizeForEncoding(
        length: CFIndex,
        encoding: CFStringEncoding,
    ) -> CFIndex;
}
unsafe extern "C" {
    pub fn CFStringGetFileSystemRepresentation(
        string: CFStringRef,
        buffer: *mut ::std::os::raw::c_char,
        maxBufLen: CFIndex,
    ) -> Boolean;
}
unsafe extern "C" {
    pub fn CFStringGetMaximumSizeOfFileSystemRepresentation(string: CFStringRef) -> CFIndex;
}
unsafe extern "C" {
    pub fn CFStringCreateWithFileSystemRepresentation(
        alloc: CFAllocatorRef,
        buffer: *const ::std::os::raw::c_char,
    ) -> CFStringRef;
}
unsafe extern "C" {
    pub fn CFStringCompareWithOptionsAndLocale(
        theString1: CFStringRef,
        theString2: CFStringRef,
        rangeToCompare: CFRange,
        compareOptions: CFStringCompareFlags,
        locale: CFLocaleRef,
    ) -> CFComparisonResult;
}
unsafe extern "C" {
    pub fn CFStringCompareWithOptions(
        theString1: CFStringRef,
        theString2: CFStringRef,
        rangeToCompare: CFRange,
        compareOptions: CFStringCompareFlags,
    ) -> CFComparisonResult;
}
unsafe extern "C" {
    pub fn CFStringCompare(
        theString1: CFStringRef,
        theString2: CFStringRef,
        compareOptions: CFStringCompareFlags,
    ) -> CFComparisonResult;
}
unsafe extern "C" {
    pub fn CFStringFindWithOptionsAndLocale(
        theString: CFStringRef,
        stringToFind: CFStringRef,
        rangeToSearch: CFRange,
        searchOptions: CFStringCompareFlags,
        locale: CFLocaleRef,
        result: *mut CFRange,
    ) -> Boolean;
}
unsafe extern "C" {
    pub fn CFStringFindWithOptions(
        theString: CFStringRef,
        stringToFind: CFStringRef,
        rangeToSearch: CFRange,
        searchOptions: CFStringCompareFlags,
        result: *mut CFRange,
    ) -> Boolean;
}
unsafe extern "C" {
    pub fn CFStringCreateArrayWithFindResults(
        alloc: CFAllocatorRef,
        theString: CFStringRef,
        stringToFind: CFStringRef,
        rangeToSearch: CFRange,
        compareOptions: CFStringCompareFlags,
    ) -> CFArrayRef;
}
unsafe extern "C" {
    pub fn CFStringFind(
        theString: CFStringRef,
        stringToFind: CFStringRef,
        compareOptions: CFStringCompareFlags,
    ) -> CFRange;
}
unsafe extern "C" {
    pub fn CFStringHasPrefix(theString: CFStringRef, prefix: CFStringRef) -> Boolean;
}
unsafe extern "C" {
    pub fn CFStringHasSuffix(theString: CFStringRef, suffix: CFStringRef) -> Boolean;
}
unsafe extern "C" {
    pub fn CFStringGetRangeOfComposedCharactersAtIndex(
        theString: CFStringRef,
        theIndex: CFIndex,
    ) -> CFRange;
}
unsafe extern "C" {
    pub fn CFStringFindCharacterFromSet(
        theString: CFStringRef,
        theSet: CFCharacterSetRef,
        rangeToSearch: CFRange,
        searchOptions: CFStringCompareFlags,
        result: *mut CFRange,
    ) -> Boolean;
}
unsafe extern "C" {
    pub fn CFStringGetLineBounds(
        theString: CFStringRef,
        range: CFRange,
        lineBeginIndex: *mut CFIndex,
        lineEndIndex: *mut CFIndex,
        contentsEndIndex: *mut CFIndex,
    );
}
unsafe extern "C" {
    pub fn CFStringGetParagraphBounds(
        string: CFStringRef,
        range: CFRange,
        parBeginIndex: *mut CFIndex,
        parEndIndex: *mut CFIndex,
        contentsEndIndex: *mut CFIndex,
    );
}
unsafe extern "C" {
    pub fn CFStringGetHyphenationLocationBeforeIndex(
        string: CFStringRef,
        location: CFIndex,
        limitRange: CFRange,
        options: CFOptionFlags,
        locale: CFLocaleRef,
        character: *mut UTF32Char,
    ) -> CFIndex;
}
unsafe extern "C" {
    pub fn CFStringIsHyphenationAvailableForLocale(locale: CFLocaleRef) -> Boolean;
}
unsafe extern "C" {
    pub fn CFStringCreateByCombiningStrings(
        alloc: CFAllocatorRef,
        theArray: CFArrayRef,
        separatorString: CFStringRef,
    ) -> CFStringRef;
}
unsafe extern "C" {
    pub fn CFStringCreateArrayBySeparatingStrings(
        alloc: CFAllocatorRef,
        theString: CFStringRef,
        separatorString: CFStringRef,
    ) -> CFArrayRef;
}
unsafe extern "C" {
    pub fn CFStringGetIntValue(str_: CFStringRef) -> SInt32;
}
unsafe extern "C" {
    pub fn CFStringGetDoubleValue(str_: CFStringRef) -> f64;
}
unsafe extern "C" {
    pub fn CFStringAppend(theString: CFMutableStringRef, appendedString: CFStringRef);
}
unsafe extern "C" {
    pub fn CFStringAppendCharacters(
        theString: CFMutableStringRef,
        chars: *const UniChar,
        numChars: CFIndex,
    );
}
unsafe extern "C" {
    pub fn CFStringAppendPascalString(
        theString: CFMutableStringRef,
        pStr: ConstStr255Param,
        encoding: CFStringEncoding,
    );
}
unsafe extern "C" {
    pub fn CFStringAppendCString(
        theString: CFMutableStringRef,
        cStr: *const ::std::os::raw::c_char,
        encoding: CFStringEncoding,
    );
}
unsafe extern "C" {
    pub fn CFStringAppendFormat(
        theString: CFMutableStringRef,
        formatOptions: CFDictionaryRef,
        format: CFStringRef,
        ...
    );
}
unsafe extern "C" {
    pub fn CFStringAppendFormatAndArguments(
        theString: CFMutableStringRef,
        formatOptions: CFDictionaryRef,
        format: CFStringRef,
        arguments: va_list,
    );
}
unsafe extern "C" {
    pub fn CFStringInsert(str_: CFMutableStringRef, idx: CFIndex, insertedStr: CFStringRef);
}
unsafe extern "C" {
    pub fn CFStringDelete(theString: CFMutableStringRef, range: CFRange);
}
unsafe extern "C" {
    pub fn CFStringReplace(theString: CFMutableStringRef, range: CFRange, replacement: CFStringRef);
}
unsafe extern "C" {
    pub fn CFStringReplaceAll(theString: CFMutableStringRef, replacement: CFStringRef);
}
unsafe extern "C" {
    pub fn CFStringFindAndReplace(
        theString: CFMutableStringRef,
        stringToFind: CFStringRef,
        replacementString: CFStringRef,
        rangeToSearch: CFRange,
        compareOptions: CFStringCompareFlags,
    ) -> CFIndex;
}
unsafe extern "C" {
    pub fn CFStringSetExternalCharactersNoCopy(
        theString: CFMutableStringRef,
        chars: *mut UniChar,
        length: CFIndex,
        capacity: CFIndex,
    );
}
unsafe extern "C" {
    pub fn CFStringPad(
        theString: CFMutableStringRef,
        padString: CFStringRef,
        length: CFIndex,
        indexIntoPad: CFIndex,
    );
}
unsafe extern "C" {
    pub fn CFStringTrim(theString: CFMutableStringRef, trimString: CFStringRef);
}
unsafe extern "C" {
    pub fn CFStringTrimWhitespace(theString: CFMutableStringRef);
}
unsafe extern "C" {
    pub fn CFStringLowercase(theString: CFMutableStringRef, locale: CFLocaleRef);
}
unsafe extern "C" {
    pub fn CFStringUppercase(theString: CFMutableStringRef, locale: CFLocaleRef);
}
unsafe extern "C" {
    pub fn CFStringCapitalize(theString: CFMutableStringRef, locale: CFLocaleRef);
}
unsafe extern "C" {
    pub fn CFStringNormalize(theString: CFMutableStringRef, theForm: CFStringNormalizationForm);
}
unsafe extern "C" {
    pub fn CFStringFold(
        theString: CFMutableStringRef,
        theFlags: CFStringCompareFlags,
        theLocale: CFLocaleRef,
    );
}
unsafe extern "C" {
    pub fn CFStringTransform(
        string: CFMutableStringRef,
        range: *mut CFRange,
        transform: CFStringRef,
        reverse: Boolean,
    ) -> Boolean;
}
unsafe extern "C" {
    pub static kCFStringTransformStripCombiningMarks: CFStringRef;
}
unsafe extern "C" {
    pub static kCFStringTransformToLatin: CFStringRef;
}
unsafe extern "C" {
    pub static kCFStringTransformFullwidthHalfwidth: CFStringRef;
}
unsafe extern "C" {
    pub static kCFStringTransformLatinKatakana: CFStringRef;
}
unsafe extern "C" {
    pub static kCFStringTransformLatinHiragana: CFStringRef;
}
unsafe extern "C" {
    pub static kCFStringTransformHiraganaKatakana: CFStringRef;
}
unsafe extern "C" {
    pub static kCFStringTransformMandarinLatin: CFStringRef;
}
unsafe extern "C" {
    pub static kCFStringTransformLatinHangul: CFStringRef;
}
unsafe extern "C" {
    pub static kCFStringTransformLatinArabic: CFStringRef;
}
unsafe extern "C" {
    pub static kCFStringTransformLatinHebrew: CFStringRef;
}
unsafe extern "C" {
    pub static kCFStringTransformLatinThai: CFStringRef;
}
unsafe extern "C" {
    pub static kCFStringTransformLatinCyrillic: CFStringRef;
}
unsafe extern "C" {
    pub static kCFStringTransformLatinGreek: CFStringRef;
}
unsafe extern "C" {
    pub static kCFStringTransformToXMLHex: CFStringRef;
}
unsafe extern "C" {
    pub static kCFStringTransformToUnicodeName: CFStringRef;
}
unsafe extern "C" {
    pub static kCFStringTransformStripDiacritics: CFStringRef;
}
unsafe extern "C" {
    pub fn CFStringIsEncodingAvailable(encoding: CFStringEncoding) -> Boolean;
}
unsafe extern "C" {
    pub fn CFStringGetListOfAvailableEncodings() -> *const CFStringEncoding;
}
unsafe extern "C" {
    pub fn CFStringGetNameOfEncoding(encoding: CFStringEncoding) -> CFStringRef;
}
unsafe extern "C" {
    pub fn CFStringConvertEncodingToNSStringEncoding(
        encoding: CFStringEncoding,
    ) -> ::std::os::raw::c_ulong;
}
unsafe extern "C" {
    pub fn CFStringConvertNSStringEncodingToEncoding(
        encoding: ::std::os::raw::c_ulong,
    ) -> CFStringEncoding;
}
unsafe extern "C" {
    pub fn CFStringConvertEncodingToWindowsCodepage(encoding: CFStringEncoding) -> UInt32;
}
unsafe extern "C" {
    pub fn CFStringConvertWindowsCodepageToEncoding(codepage: UInt32) -> CFStringEncoding;
}
unsafe extern "C" {
    pub fn CFStringConvertIANACharSetNameToEncoding(theString: CFStringRef) -> CFStringEncoding;
}
unsafe extern "C" {
    pub fn CFStringConvertEncodingToIANACharSetName(encoding: CFStringEncoding) -> CFStringRef;
}
unsafe extern "C" {
    pub fn CFStringGetMostCompatibleMacStringEncoding(
        encoding: CFStringEncoding,
    ) -> CFStringEncoding;
}
unsafe extern "C" {
    pub fn CFShow(obj: CFTypeRef);
}
unsafe extern "C" {
    pub fn CFShowStr(str_: CFStringRef);
}
unsafe extern "C" {
    pub fn __CFStringMakeConstantString(cStr: *const ::std::os::raw::c_char) -> CFStringRef;
}
unsafe extern "C" {
    pub fn CFTimeZoneGetTypeID() -> CFTypeID;
}
unsafe extern "C" {
    pub fn CFTimeZoneCopySystem() -> CFTimeZoneRef;
}
unsafe extern "C" {
    pub fn CFTimeZoneResetSystem();
}
unsafe extern "C" {
    pub fn CFTimeZoneCopyDefault() -> CFTimeZoneRef;
}
unsafe extern "C" {
    pub fn CFTimeZoneSetDefault(tz: CFTimeZoneRef);
}
unsafe extern "C" {
    pub fn CFTimeZoneCopyKnownNames() -> CFArrayRef;
}
unsafe extern "C" {
    pub fn CFTimeZoneCopyAbbreviationDictionary() -> CFDictionaryRef;
}
unsafe extern "C" {
    pub fn CFTimeZoneSetAbbreviationDictionary(dict: CFDictionaryRef);
}
unsafe extern "C" {
    pub fn CFTimeZoneCreate(
        allocator: CFAllocatorRef,
        name: CFStringRef,
        data: CFDataRef,
    ) -> CFTimeZoneRef;
}
unsafe extern "C" {
    pub fn CFTimeZoneCreateWithTimeIntervalFromGMT(
        allocator: CFAllocatorRef,
        ti: CFTimeInterval,
    ) -> CFTimeZoneRef;
}
unsafe extern "C" {
    pub fn CFTimeZoneCreateWithName(
        allocator: CFAllocatorRef,
        name: CFStringRef,
        tryAbbrev: Boolean,
    ) -> CFTimeZoneRef;
}
unsafe extern "C" {
    pub fn CFTimeZoneGetName(tz: CFTimeZoneRef) -> CFStringRef;
}
unsafe extern "C" {
    pub fn CFTimeZoneGetData(tz: CFTimeZoneRef) -> CFDataRef;
}
unsafe extern "C" {
    pub fn CFTimeZoneGetSecondsFromGMT(tz: CFTimeZoneRef, at: CFAbsoluteTime) -> CFTimeInterval;
}
unsafe extern "C" {
    pub fn CFTimeZoneCopyAbbreviation(tz: CFTimeZoneRef, at: CFAbsoluteTime) -> CFStringRef;
}
unsafe extern "C" {
    pub fn CFTimeZoneIsDaylightSavingTime(tz: CFTimeZoneRef, at: CFAbsoluteTime) -> Boolean;
}
unsafe extern "C" {
    pub fn CFTimeZoneGetDaylightSavingTimeOffset(
        tz: CFTimeZoneRef,
        at: CFAbsoluteTime,
    ) -> CFTimeInterval;
}
unsafe extern "C" {
    pub fn CFTimeZoneGetNextDaylightSavingTimeTransition(
        tz: CFTimeZoneRef,
        at: CFAbsoluteTime,
    ) -> CFAbsoluteTime;
}
unsafe extern "C" {
    pub fn CFTimeZoneCopyLocalizedName(
        tz: CFTimeZoneRef,
        style: CFTimeZoneNameStyle,
        locale: CFLocaleRef,
    ) -> CFStringRef;
}
unsafe extern "C" {
    pub static kCFTimeZoneSystemTimeZoneDidChangeNotification: CFNotificationName;
}
unsafe extern "C" {
    pub fn CFCalendarGetTypeID() -> CFTypeID;
}
unsafe extern "C" {
    pub fn CFCalendarCopyCurrent() -> CFCalendarRef;
}
unsafe extern "C" {
    pub fn CFCalendarCreateWithIdentifier(
        allocator: CFAllocatorRef,
        identifier: CFCalendarIdentifier,
    ) -> CFCalendarRef;
}
unsafe extern "C" {
    pub fn CFCalendarGetIdentifier(calendar: CFCalendarRef) -> CFCalendarIdentifier;
}
unsafe extern "C" {
    pub fn CFCalendarCopyLocale(calendar: CFCalendarRef) -> CFLocaleRef;
}
unsafe extern "C" {
    pub fn CFCalendarSetLocale(calendar: CFCalendarRef, locale: CFLocaleRef);
}
unsafe extern "C" {
    pub fn CFCalendarCopyTimeZone(calendar: CFCalendarRef) -> CFTimeZoneRef;
}
unsafe extern "C" {
    pub fn CFCalendarSetTimeZone(calendar: CFCalendarRef, tz: CFTimeZoneRef);
}
unsafe extern "C" {
    pub fn CFCalendarGetFirstWeekday(calendar: CFCalendarRef) -> CFIndex;
}
unsafe extern "C" {
    pub fn CFCalendarSetFirstWeekday(calendar: CFCalendarRef, wkdy: CFIndex);
}
unsafe extern "C" {
    pub fn CFCalendarGetMinimumDaysInFirstWeek(calendar: CFCalendarRef) -> CFIndex;
}
unsafe extern "C" {
    pub fn CFCalendarSetMinimumDaysInFirstWeek(calendar: CFCalendarRef, mwd: CFIndex);
}
unsafe extern "C" {
    pub fn CFCalendarGetMinimumRangeOfUnit(
        calendar: CFCalendarRef,
        unit: CFCalendarUnit,
    ) -> CFRange;
}
unsafe extern "C" {
    pub fn CFCalendarGetMaximumRangeOfUnit(
        calendar: CFCalendarRef,
        unit: CFCalendarUnit,
    ) -> CFRange;
}
unsafe extern "C" {
    pub fn CFCalendarGetRangeOfUnit(
        calendar: CFCalendarRef,
        smallerUnit: CFCalendarUnit,
        biggerUnit: CFCalendarUnit,
        at: CFAbsoluteTime,
    ) -> CFRange;
}
unsafe extern "C" {
    pub fn CFCalendarGetOrdinalityOfUnit(
        calendar: CFCalendarRef,
        smallerUnit: CFCalendarUnit,
        biggerUnit: CFCalendarUnit,
        at: CFAbsoluteTime,
    ) -> CFIndex;
}
unsafe extern "C" {
    pub fn CFCalendarGetTimeRangeOfUnit(
        calendar: CFCalendarRef,
        unit: CFCalendarUnit,
        at: CFAbsoluteTime,
        startp: *mut CFAbsoluteTime,
        tip: *mut CFTimeInterval,
    ) -> Boolean;
}
unsafe extern "C" {
    pub fn CFCalendarComposeAbsoluteTime(
        calendar: CFCalendarRef,
        at: *mut CFAbsoluteTime,
        componentDesc: *const ::std::os::raw::c_char,
        ...
    ) -> Boolean;
}
unsafe extern "C" {
    pub fn CFCalendarDecomposeAbsoluteTime(
        calendar: CFCalendarRef,
        at: CFAbsoluteTime,
        componentDesc: *const ::std::os::raw::c_char,
        ...
    ) -> Boolean;
}
unsafe extern "C" {
    pub fn CFCalendarAddComponents(
        calendar: CFCalendarRef,
        at: *mut CFAbsoluteTime,
        options: CFOptionFlags,
        componentDesc: *const ::std::os::raw::c_char,
        ...
    ) -> Boolean;
}
unsafe extern "C" {
    pub fn CFCalendarGetComponentDifference(
        calendar: CFCalendarRef,
        startingAT: CFAbsoluteTime,
        resultAT: CFAbsoluteTime,
        options: CFOptionFlags,
        componentDesc: *const ::std::os::raw::c_char,
        ...
    ) -> Boolean;
}
unsafe extern "C" {
    pub fn CFDateFormatterCreateDateFormatFromTemplate(
        allocator: CFAllocatorRef,
        tmplate: CFStringRef,
        options: CFOptionFlags,
        locale: CFLocaleRef,
    ) -> CFStringRef;
}
unsafe extern "C" {
    pub fn CFDateFormatterGetTypeID() -> CFTypeID;
}
unsafe extern "C" {
    pub fn CFDateFormatterCreateISO8601Formatter(
        allocator: CFAllocatorRef,
        formatOptions: CFISO8601DateFormatOptions,
    ) -> CFDateFormatterRef;
}
unsafe extern "C" {
    pub fn CFDateFormatterCreate(
        allocator: CFAllocatorRef,
        locale: CFLocaleRef,
        dateStyle: CFDateFormatterStyle,
        timeStyle: CFDateFormatterStyle,
    ) -> CFDateFormatterRef;
}
unsafe extern "C" {
    pub fn CFDateFormatterGetLocale(formatter: CFDateFormatterRef) -> CFLocaleRef;
}
unsafe extern "C" {
    pub fn CFDateFormatterGetDateStyle(formatter: CFDateFormatterRef) -> CFDateFormatterStyle;
}
unsafe extern "C" {
    pub fn CFDateFormatterGetTimeStyle(formatter: CFDateFormatterRef) -> CFDateFormatterStyle;
}
unsafe extern "C" {
    pub fn CFDateFormatterGetFormat(formatter: CFDateFormatterRef) -> CFStringRef;
}
unsafe extern "C" {
    pub fn CFDateFormatterSetFormat(formatter: CFDateFormatterRef, formatString: CFStringRef);
}
unsafe extern "C" {
    pub fn CFDateFormatterCreateStringWithDate(
        allocator: CFAllocatorRef,
        formatter: CFDateFormatterRef,
        date: CFDateRef,
    ) -> CFStringRef;
}
unsafe extern "C" {
    pub fn CFDateFormatterCreateStringWithAbsoluteTime(
        allocator: CFAllocatorRef,
        formatter: CFDateFormatterRef,
        at: CFAbsoluteTime,
    ) -> CFStringRef;
}
unsafe extern "C" {
    pub fn CFDateFormatterCreateDateFromString(
        allocator: CFAllocatorRef,
        formatter: CFDateFormatterRef,
        string: CFStringRef,
        rangep: *mut CFRange,
    ) -> CFDateRef;
}
unsafe extern "C" {
    pub fn CFDateFormatterGetAbsoluteTimeFromString(
        formatter: CFDateFormatterRef,
        string: CFStringRef,
        rangep: *mut CFRange,
        atp: *mut CFAbsoluteTime,
    ) -> Boolean;
}
unsafe extern "C" {
    pub fn CFDateFormatterSetProperty(
        formatter: CFDateFormatterRef,
        key: CFStringRef,
        value: CFTypeRef,
    );
}
unsafe extern "C" {
    pub fn CFDateFormatterCopyProperty(
        formatter: CFDateFormatterRef,
        key: CFDateFormatterKey,
    ) -> CFTypeRef;
}
unsafe extern "C" {
    pub static kCFDateFormatterIsLenient: CFDateFormatterKey;
}
unsafe extern "C" {
    pub static kCFDateFormatterTimeZone: CFDateFormatterKey;
}
unsafe extern "C" {
    pub static kCFDateFormatterCalendarName: CFDateFormatterKey;
}
unsafe extern "C" {
    pub static kCFDateFormatterDefaultFormat: CFDateFormatterKey;
}
unsafe extern "C" {
    pub static kCFDateFormatterTwoDigitStartDate: CFDateFormatterKey;
}
unsafe extern "C" {
    pub static kCFDateFormatterDefaultDate: CFDateFormatterKey;
}
unsafe extern "C" {
    pub static kCFDateFormatterCalendar: CFDateFormatterKey;
}
unsafe extern "C" {
    pub static kCFDateFormatterEraSymbols: CFDateFormatterKey;
}
unsafe extern "C" {
    pub static kCFDateFormatterMonthSymbols: CFDateFormatterKey;
}
unsafe extern "C" {
    pub static kCFDateFormatterShortMonthSymbols: CFDateFormatterKey;
}
unsafe extern "C" {
    pub static kCFDateFormatterWeekdaySymbols: CFDateFormatterKey;
}
unsafe extern "C" {
    pub static kCFDateFormatterShortWeekdaySymbols: CFDateFormatterKey;
}
unsafe extern "C" {
    pub static kCFDateFormatterAMSymbol: CFDateFormatterKey;
}
unsafe extern "C" {
    pub static kCFDateFormatterPMSymbol: CFDateFormatterKey;
}
unsafe extern "C" {
    pub static kCFDateFormatterLongEraSymbols: CFDateFormatterKey;
}
unsafe extern "C" {
    pub static kCFDateFormatterVeryShortMonthSymbols: CFDateFormatterKey;
}
unsafe extern "C" {
    pub static kCFDateFormatterStandaloneMonthSymbols: CFDateFormatterKey;
}
unsafe extern "C" {
    pub static kCFDateFormatterShortStandaloneMonthSymbols: CFDateFormatterKey;
}
unsafe extern "C" {
    pub static kCFDateFormatterVeryShortStandaloneMonthSymbols: CFDateFormatterKey;
}
unsafe extern "C" {
    pub static kCFDateFormatterVeryShortWeekdaySymbols: CFDateFormatterKey;
}
unsafe extern "C" {
    pub static kCFDateFormatterStandaloneWeekdaySymbols: CFDateFormatterKey;
}
unsafe extern "C" {
    pub static kCFDateFormatterShortStandaloneWeekdaySymbols: CFDateFormatterKey;
}
unsafe extern "C" {
    pub static kCFDateFormatterVeryShortStandaloneWeekdaySymbols: CFDateFormatterKey;
}
unsafe extern "C" {
    pub static kCFDateFormatterQuarterSymbols: CFDateFormatterKey;
}
unsafe extern "C" {
    pub static kCFDateFormatterShortQuarterSymbols: CFDateFormatterKey;
}
unsafe extern "C" {
    pub static kCFDateFormatterStandaloneQuarterSymbols: CFDateFormatterKey;
}
unsafe extern "C" {
    pub static kCFDateFormatterShortStandaloneQuarterSymbols: CFDateFormatterKey;
}
unsafe extern "C" {
    pub static kCFDateFormatterGregorianStartDate: CFDateFormatterKey;
}
unsafe extern "C" {
    pub static kCFDateFormatterDoesRelativeDateFormattingKey: CFDateFormatterKey;
}
unsafe extern "C" {
    pub static kCFBooleanTrue: CFBooleanRef;
}
unsafe extern "C" {
    pub static kCFBooleanFalse: CFBooleanRef;
}
unsafe extern "C" {
    pub fn CFBooleanGetTypeID() -> CFTypeID;
}
unsafe extern "C" {
    pub fn CFBooleanGetValue(boolean: CFBooleanRef) -> Boolean;
}
unsafe extern "C" {
    pub static kCFNumberPositiveInfinity: CFNumberRef;
}
unsafe extern "C" {
    pub static kCFNumberNegativeInfinity: CFNumberRef;
}
unsafe extern "C" {
    pub static kCFNumberNaN: CFNumberRef;
}
unsafe extern "C" {
    pub fn CFNumberGetTypeID() -> CFTypeID;
}
unsafe extern "C" {
    pub fn CFNumberCreate(
        allocator: CFAllocatorRef,
        theType: CFNumberType,
        valuePtr: *const ::std::os::raw::c_void,
    ) -> CFNumberRef;
}
unsafe extern "C" {
    pub fn CFNumberGetType(number: CFNumberRef) -> CFNumberType;
}
unsafe extern "C" {
    pub fn CFNumberGetByteSize(number: CFNumberRef) -> CFIndex;
}
unsafe extern "C" {
    pub fn CFNumberIsFloatType(number: CFNumberRef) -> Boolean;
}
unsafe extern "C" {
    pub fn CFNumberGetValue(
        number: CFNumberRef,
        theType: CFNumberType,
        valuePtr: *mut ::std::os::raw::c_void,
    ) -> Boolean;
}
unsafe extern "C" {
    pub fn CFNumberCompare(
        number: CFNumberRef,
        otherNumber: CFNumberRef,
        context: *mut ::std::os::raw::c_void,
    ) -> CFComparisonResult;
}
unsafe extern "C" {
    pub fn CFNumberFormatterGetTypeID() -> CFTypeID;
}
unsafe extern "C" {
    pub fn CFNumberFormatterCreate(
        allocator: CFAllocatorRef,
        locale: CFLocaleRef,
        style: CFNumberFormatterStyle,
    ) -> CFNumberFormatterRef;
}
unsafe extern "C" {
    pub fn CFNumberFormatterGetLocale(formatter: CFNumberFormatterRef) -> CFLocaleRef;
}
unsafe extern "C" {
    pub fn CFNumberFormatterGetStyle(formatter: CFNumberFormatterRef) -> CFNumberFormatterStyle;
}
unsafe extern "C" {
    pub fn CFNumberFormatterGetFormat(formatter: CFNumberFormatterRef) -> CFStringRef;
}
unsafe extern "C" {
    pub fn CFNumberFormatterSetFormat(formatter: CFNumberFormatterRef, formatString: CFStringRef);
}
unsafe extern "C" {
    pub fn CFNumberFormatterCreateStringWithNumber(
        allocator: CFAllocatorRef,
        formatter: CFNumberFormatterRef,
        number: CFNumberRef,
    ) -> CFStringRef;
}
unsafe extern "C" {
    pub fn CFNumberFormatterCreateStringWithValue(
        allocator: CFAllocatorRef,
        formatter: CFNumberFormatterRef,
        numberType: CFNumberType,
        valuePtr: *const ::std::os::raw::c_void,
    ) -> CFStringRef;
}
unsafe extern "C" {
    pub fn CFNumberFormatterCreateNumberFromString(
        allocator: CFAllocatorRef,
        formatter: CFNumberFormatterRef,
        string: CFStringRef,
        rangep: *mut CFRange,
        options: CFOptionFlags,
    ) -> CFNumberRef;
}
unsafe extern "C" {
    pub fn CFNumberFormatterGetValueFromString(
        formatter: CFNumberFormatterRef,
        string: CFStringRef,
        rangep: *mut CFRange,
        numberType: CFNumberType,
        valuePtr: *mut ::std::os::raw::c_void,
    ) -> Boolean;
}
unsafe extern "C" {
    pub fn CFNumberFormatterSetProperty(
        formatter: CFNumberFormatterRef,
        key: CFNumberFormatterKey,
        value: CFTypeRef,
    );
}
unsafe extern "C" {
    pub fn CFNumberFormatterCopyProperty(
        formatter: CFNumberFormatterRef,
        key: CFNumberFormatterKey,
    ) -> CFTypeRef;
}
unsafe extern "C" {
    pub static kCFNumberFormatterCurrencyCode: CFNumberFormatterKey;
}
unsafe extern "C" {
    pub static kCFNumberFormatterDecimalSeparator: CFNumberFormatterKey;
}
unsafe extern "C" {
    pub static kCFNumberFormatterCurrencyDecimalSeparator: CFNumberFormatterKey;
}
unsafe extern "C" {
    pub static kCFNumberFormatterAlwaysShowDecimalSeparator: CFNumberFormatterKey;
}
unsafe extern "C" {
    pub static kCFNumberFormatterGroupingSeparator: CFNumberFormatterKey;
}
unsafe extern "C" {
    pub static kCFNumberFormatterUseGroupingSeparator: CFNumberFormatterKey;
}
unsafe extern "C" {
    pub static kCFNumberFormatterPercentSymbol: CFNumberFormatterKey;
}
unsafe extern "C" {
    pub static kCFNumberFormatterZeroSymbol: CFNumberFormatterKey;
}
unsafe extern "C" {
    pub static kCFNumberFormatterNaNSymbol: CFNumberFormatterKey;
}
unsafe extern "C" {
    pub static kCFNumberFormatterInfinitySymbol: CFNumberFormatterKey;
}
unsafe extern "C" {
    pub static kCFNumberFormatterMinusSign: CFNumberFormatterKey;
}
unsafe extern "C" {
    pub static kCFNumberFormatterPlusSign: CFNumberFormatterKey;
}
unsafe extern "C" {
    pub static kCFNumberFormatterCurrencySymbol: CFNumberFormatterKey;
}
unsafe extern "C" {
    pub static kCFNumberFormatterExponentSymbol: CFNumberFormatterKey;
}
unsafe extern "C" {
    pub static kCFNumberFormatterMinIntegerDigits: CFNumberFormatterKey;
}
unsafe extern "C" {
    pub static kCFNumberFormatterMaxIntegerDigits: CFNumberFormatterKey;
}
unsafe extern "C" {
    pub static kCFNumberFormatterMinFractionDigits: CFNumberFormatterKey;
}
unsafe extern "C" {
    pub static kCFNumberFormatterMaxFractionDigits: CFNumberFormatterKey;
}
unsafe extern "C" {
    pub static kCFNumberFormatterGroupingSize: CFNumberFormatterKey;
}
unsafe extern "C" {
    pub static kCFNumberFormatterSecondaryGroupingSize: CFNumberFormatterKey;
}
unsafe extern "C" {
    pub static kCFNumberFormatterRoundingMode: CFNumberFormatterKey;
}
unsafe extern "C" {
    pub static kCFNumberFormatterRoundingIncrement: CFNumberFormatterKey;
}
unsafe extern "C" {
    pub static kCFNumberFormatterFormatWidth: CFNumberFormatterKey;
}
unsafe extern "C" {
    pub static kCFNumberFormatterPaddingPosition: CFNumberFormatterKey;
}
unsafe extern "C" {
    pub static kCFNumberFormatterPaddingCharacter: CFNumberFormatterKey;
}
unsafe extern "C" {
    pub static kCFNumberFormatterDefaultFormat: CFNumberFormatterKey;
}
unsafe extern "C" {
    pub static kCFNumberFormatterMultiplier: CFNumberFormatterKey;
}
unsafe extern "C" {
    pub static kCFNumberFormatterPositivePrefix: CFNumberFormatterKey;
}
unsafe extern "C" {
    pub static kCFNumberFormatterPositiveSuffix: CFNumberFormatterKey;
}
unsafe extern "C" {
    pub static kCFNumberFormatterNegativePrefix: CFNumberFormatterKey;
}
unsafe extern "C" {
    pub static kCFNumberFormatterNegativeSuffix: CFNumberFormatterKey;
}
unsafe extern "C" {
    pub static kCFNumberFormatterPerMillSymbol: CFNumberFormatterKey;
}
unsafe extern "C" {
    pub static kCFNumberFormatterInternationalCurrencySymbol: CFNumberFormatterKey;
}
unsafe extern "C" {
    pub static kCFNumberFormatterCurrencyGroupingSeparator: CFNumberFormatterKey;
}
unsafe extern "C" {
    pub static kCFNumberFormatterIsLenient: CFNumberFormatterKey;
}
unsafe extern "C" {
    pub static kCFNumberFormatterUseSignificantDigits: CFNumberFormatterKey;
}
unsafe extern "C" {
    pub static kCFNumberFormatterMinSignificantDigits: CFNumberFormatterKey;
}
unsafe extern "C" {
    pub static kCFNumberFormatterMaxSignificantDigits: CFNumberFormatterKey;
}
unsafe extern "C" {
    pub static kCFNumberFormatterMinGroupingDigits: CFNumberFormatterKey;
}
unsafe extern "C" {
    pub fn CFNumberFormatterGetDecimalInfoForCurrencyCode(
        currencyCode: CFStringRef,
        defaultFractionDigits: *mut i32,
        roundingIncrement: *mut f64,
    ) -> Boolean;
}
unsafe extern "C" {
    pub static kCFPreferencesAnyApplication: CFStringRef;
}
unsafe extern "C" {
    pub static kCFPreferencesCurrentApplication: CFStringRef;
}
unsafe extern "C" {
    pub static kCFPreferencesAnyHost: CFStringRef;
}
unsafe extern "C" {
    pub static kCFPreferencesCurrentHost: CFStringRef;
}
unsafe extern "C" {
    pub static kCFPreferencesAnyUser: CFStringRef;
}
unsafe extern "C" {
    pub static kCFPreferencesCurrentUser: CFStringRef;
}
unsafe extern "C" {
    pub fn CFPreferencesCopyAppValue(
        key: CFStringRef,
        applicationID: CFStringRef,
    ) -> CFPropertyListRef;
}
unsafe extern "C" {
    pub fn CFPreferencesGetAppBooleanValue(
        key: CFStringRef,
        applicationID: CFStringRef,
        keyExistsAndHasValidFormat: *mut Boolean,
    ) -> Boolean;
}
unsafe extern "C" {
    pub fn CFPreferencesGetAppIntegerValue(
        key: CFStringRef,
        applicationID: CFStringRef,
        keyExistsAndHasValidFormat: *mut Boolean,
    ) -> CFIndex;
}
unsafe extern "C" {
    pub fn CFPreferencesSetAppValue(
        key: CFStringRef,
        value: CFPropertyListRef,
        applicationID: CFStringRef,
    );
}
unsafe extern "C" {
    pub fn CFPreferencesAddSuitePreferencesToApp(applicationID: CFStringRef, suiteID: CFStringRef);
}
unsafe extern "C" {
    pub fn CFPreferencesRemoveSuitePreferencesFromApp(
        applicationID: CFStringRef,
        suiteID: CFStringRef,
    );
}
unsafe extern "C" {
    pub fn CFPreferencesAppSynchronize(applicationID: CFStringRef) -> Boolean;
}
unsafe extern "C" {
    pub fn CFPreferencesCopyValue(
        key: CFStringRef,
        applicationID: CFStringRef,
        userName: CFStringRef,
        hostName: CFStringRef,
    ) -> CFPropertyListRef;
}
unsafe extern "C" {
    pub fn CFPreferencesCopyMultiple(
        keysToFetch: CFArrayRef,
        applicationID: CFStringRef,
        userName: CFStringRef,
        hostName: CFStringRef,
    ) -> CFDictionaryRef;
}
unsafe extern "C" {
    pub fn CFPreferencesSetValue(
        key: CFStringRef,
        value: CFPropertyListRef,
        applicationID: CFStringRef,
        userName: CFStringRef,
        hostName: CFStringRef,
    );
}
unsafe extern "C" {
    pub fn CFPreferencesSetMultiple(
        keysToSet: CFDictionaryRef,
        keysToRemove: CFArrayRef,
        applicationID: CFStringRef,
        userName: CFStringRef,
        hostName: CFStringRef,
    );
}
unsafe extern "C" {
    pub fn CFPreferencesSynchronize(
        applicationID: CFStringRef,
        userName: CFStringRef,
        hostName: CFStringRef,
    ) -> Boolean;
}
unsafe extern "C" {
    pub fn CFPreferencesCopyApplicationList(
        userName: CFStringRef,
        hostName: CFStringRef,
    ) -> CFArrayRef;
}
unsafe extern "C" {
    pub fn CFPreferencesCopyKeyList(
        applicationID: CFStringRef,
        userName: CFStringRef,
        hostName: CFStringRef,
    ) -> CFArrayRef;
}
unsafe extern "C" {
    pub fn CFPreferencesAppValueIsForced(key: CFStringRef, applicationID: CFStringRef) -> Boolean;
}
unsafe extern "C" {
    pub fn CFURLGetTypeID() -> CFTypeID;
}
unsafe extern "C" {
    pub fn CFURLCreateWithBytes(
        allocator: CFAllocatorRef,
        URLBytes: *const UInt8,
        length: CFIndex,
        encoding: CFStringEncoding,
        baseURL: CFURLRef,
    ) -> CFURLRef;
}
unsafe extern "C" {
    pub fn CFURLCreateData(
        allocator: CFAllocatorRef,
        url: CFURLRef,
        encoding: CFStringEncoding,
        escapeWhitespace: Boolean,
    ) -> CFDataRef;
}
unsafe extern "C" {
    pub fn CFURLCreateWithString(
        allocator: CFAllocatorRef,
        URLString: CFStringRef,
        baseURL: CFURLRef,
    ) -> CFURLRef;
}
unsafe extern "C" {
    pub fn CFURLCreateAbsoluteURLWithBytes(
        alloc: CFAllocatorRef,
        relativeURLBytes: *const UInt8,
        length: CFIndex,
        encoding: CFStringEncoding,
        baseURL: CFURLRef,
        useCompatibilityMode: Boolean,
    ) -> CFURLRef;
}
unsafe extern "C" {
    pub fn CFURLCreateWithFileSystemPath(
        allocator: CFAllocatorRef,
        filePath: CFStringRef,
        pathStyle: CFURLPathStyle,
        isDirectory: Boolean,
    ) -> CFURLRef;
}
unsafe extern "C" {
    pub fn CFURLCreateFromFileSystemRepresentation(
        allocator: CFAllocatorRef,
        buffer: *const UInt8,
        bufLen: CFIndex,
        isDirectory: Boolean,
    ) -> CFURLRef;
}
unsafe extern "C" {
    pub fn CFURLCreateWithFileSystemPathRelativeToBase(
        allocator: CFAllocatorRef,
        filePath: CFStringRef,
        pathStyle: CFURLPathStyle,
        isDirectory: Boolean,
        baseURL: CFURLRef,
    ) -> CFURLRef;
}
unsafe extern "C" {
    pub fn CFURLCreateFromFileSystemRepresentationRelativeToBase(
        allocator: CFAllocatorRef,
        buffer: *const UInt8,
        bufLen: CFIndex,
        isDirectory: Boolean,
        baseURL: CFURLRef,
    ) -> CFURLRef;
}
unsafe extern "C" {
    pub fn CFURLGetFileSystemRepresentation(
        url: CFURLRef,
        resolveAgainstBase: Boolean,
        buffer: *mut UInt8,
        maxBufLen: CFIndex,
    ) -> Boolean;
}
unsafe extern "C" {
    pub fn CFURLCopyAbsoluteURL(relativeURL: CFURLRef) -> CFURLRef;
}
unsafe extern "C" {
    pub fn CFURLGetString(anURL: CFURLRef) -> CFStringRef;
}
unsafe extern "C" {
    pub fn CFURLGetBaseURL(anURL: CFURLRef) -> CFURLRef;
}
unsafe extern "C" {
    pub fn CFURLCanBeDecomposed(anURL: CFURLRef) -> Boolean;
}
unsafe extern "C" {
    pub fn CFURLCopyScheme(anURL: CFURLRef) -> CFStringRef;
}
unsafe extern "C" {
    pub fn CFURLCopyNetLocation(anURL: CFURLRef) -> CFStringRef;
}
unsafe extern "C" {
    pub fn CFURLCopyPath(anURL: CFURLRef) -> CFStringRef;
}
unsafe extern "C" {
    pub fn CFURLCopyStrictPath(anURL: CFURLRef, isAbsolute: *mut Boolean) -> CFStringRef;
}
unsafe extern "C" {
    pub fn CFURLCopyFileSystemPath(anURL: CFURLRef, pathStyle: CFURLPathStyle) -> CFStringRef;
}
unsafe extern "C" {
    pub fn CFURLHasDirectoryPath(anURL: CFURLRef) -> Boolean;
}
unsafe extern "C" {
    pub fn CFURLCopyResourceSpecifier(anURL: CFURLRef) -> CFStringRef;
}
unsafe extern "C" {
    pub fn CFURLCopyHostName(anURL: CFURLRef) -> CFStringRef;
}
unsafe extern "C" {
    pub fn CFURLGetPortNumber(anURL: CFURLRef) -> SInt32;
}
unsafe extern "C" {
    pub fn CFURLCopyUserName(anURL: CFURLRef) -> CFStringRef;
}
unsafe extern "C" {
    pub fn CFURLCopyPassword(anURL: CFURLRef) -> CFStringRef;
}
unsafe extern "C" {
    pub fn CFURLCopyParameterString(
        anURL: CFURLRef,
        charactersToLeaveEscaped: CFStringRef,
    ) -> CFStringRef;
}
unsafe extern "C" {
    pub fn CFURLCopyQueryString(
        anURL: CFURLRef,
        charactersToLeaveEscaped: CFStringRef,
    ) -> CFStringRef;
}
unsafe extern "C" {
    pub fn CFURLCopyFragment(anURL: CFURLRef, charactersToLeaveEscaped: CFStringRef)
        -> CFStringRef;
}
unsafe extern "C" {
    pub fn CFURLCopyLastPathComponent(url: CFURLRef) -> CFStringRef;
}
unsafe extern "C" {
    pub fn CFURLCopyPathExtension(url: CFURLRef) -> CFStringRef;
}
unsafe extern "C" {
    pub fn CFURLCreateCopyAppendingPathComponent(
        allocator: CFAllocatorRef,
        url: CFURLRef,
        pathComponent: CFStringRef,
        isDirectory: Boolean,
    ) -> CFURLRef;
}
unsafe extern "C" {
    pub fn CFURLCreateCopyDeletingLastPathComponent(
        allocator: CFAllocatorRef,
        url: CFURLRef,
    ) -> CFURLRef;
}
unsafe extern "C" {
    pub fn CFURLCreateCopyAppendingPathExtension(
        allocator: CFAllocatorRef,
        url: CFURLRef,
        extension: CFStringRef,
    ) -> CFURLRef;
}
unsafe extern "C" {
    pub fn CFURLCreateCopyDeletingPathExtension(
        allocator: CFAllocatorRef,
        url: CFURLRef,
    ) -> CFURLRef;
}
unsafe extern "C" {
    pub fn CFURLGetBytes(url: CFURLRef, buffer: *mut UInt8, bufferLength: CFIndex) -> CFIndex;
}
unsafe extern "C" {
    pub fn CFURLGetByteRangeForComponent(
        url: CFURLRef,
        component: CFURLComponentType,
        rangeIncludingSeparators: *mut CFRange,
    ) -> CFRange;
}
unsafe extern "C" {
    pub fn CFURLCreateStringByReplacingPercentEscapes(
        allocator: CFAllocatorRef,
        originalString: CFStringRef,
        charactersToLeaveEscaped: CFStringRef,
    ) -> CFStringRef;
}
unsafe extern "C" {
    pub fn CFURLCreateStringByReplacingPercentEscapesUsingEncoding(
        allocator: CFAllocatorRef,
        origString: CFStringRef,
        charsToLeaveEscaped: CFStringRef,
        encoding: CFStringEncoding,
    ) -> CFStringRef;
}
unsafe extern "C" {
    pub fn CFURLCreateStringByAddingPercentEscapes(
        allocator: CFAllocatorRef,
        originalString: CFStringRef,
        charactersToLeaveUnescaped: CFStringRef,
        legalURLCharactersToBeEscaped: CFStringRef,
        encoding: CFStringEncoding,
    ) -> CFStringRef;
}
unsafe extern "C" {
    pub fn CFURLIsFileReferenceURL(url: CFURLRef) -> Boolean;
}
unsafe extern "C" {
    pub fn CFURLCreateFileReferenceURL(
        allocator: CFAllocatorRef,
        url: CFURLRef,
        error: *mut CFErrorRef,
    ) -> CFURLRef;
}
unsafe extern "C" {
    pub fn CFURLCreateFilePathURL(
        allocator: CFAllocatorRef,
        url: CFURLRef,
        error: *mut CFErrorRef,
    ) -> CFURLRef;
}
unsafe extern "C" {
    pub fn CFURLCreateFromFSRef(allocator: CFAllocatorRef, fsRef: *const FSRef) -> CFURLRef;
}
unsafe extern "C" {
    pub fn CFURLGetFSRef(url: CFURLRef, fsRef: *mut FSRef) -> Boolean;
}
unsafe extern "C" {
    pub fn CFURLCopyResourcePropertyForKey(
        url: CFURLRef,
        key: CFStringRef,
        propertyValueTypeRefPtr: *mut ::std::os::raw::c_void,
        error: *mut CFErrorRef,
    ) -> Boolean;
}
unsafe extern "C" {
    pub fn CFURLCopyResourcePropertiesForKeys(
        url: CFURLRef,
        keys: CFArrayRef,
        error: *mut CFErrorRef,
    ) -> CFDictionaryRef;
}
unsafe extern "C" {
    pub fn CFURLSetResourcePropertyForKey(
        url: CFURLRef,
        key: CFStringRef,
        propertyValue: CFTypeRef,
        error: *mut CFErrorRef,
    ) -> Boolean;
}
unsafe extern "C" {
    pub fn CFURLSetResourcePropertiesForKeys(
        url: CFURLRef,
        keyedPropertyValues: CFDictionaryRef,
        error: *mut CFErrorRef,
    ) -> Boolean;
}
unsafe extern "C" {
    pub static kCFURLKeysOfUnsetValuesKey: CFStringRef;
}
unsafe extern "C" {
    pub fn CFURLClearResourcePropertyCacheForKey(url: CFURLRef, key: CFStringRef);
}
unsafe extern "C" {
    pub fn CFURLClearResourcePropertyCache(url: CFURLRef);
}
unsafe extern "C" {
    pub fn CFURLSetTemporaryResourcePropertyForKey(
        url: CFURLRef,
        key: CFStringRef,
        propertyValue: CFTypeRef,
    );
}
unsafe extern "C" {
    pub fn CFURLResourceIsReachable(url: CFURLRef, error: *mut CFErrorRef) -> Boolean;
}
unsafe extern "C" {
    pub static kCFURLNameKey: CFStringRef;
}
unsafe extern "C" {
    pub static kCFURLLocalizedNameKey: CFStringRef;
}
unsafe extern "C" {
    pub static kCFURLIsRegularFileKey: CFStringRef;
}
unsafe extern "C" {
    pub static kCFURLIsDirectoryKey: CFStringRef;
}
unsafe extern "C" {
    pub static kCFURLIsSymbolicLinkKey: CFStringRef;
}
unsafe extern "C" {
    pub static kCFURLIsVolumeKey: CFStringRef;
}
unsafe extern "C" {
    pub static kCFURLIsPackageKey: CFStringRef;
}
unsafe extern "C" {
    pub static kCFURLIsApplicationKey: CFStringRef;
}
unsafe extern "C" {
    pub static kCFURLApplicationIsScriptableKey: CFStringRef;
}
unsafe extern "C" {
    pub static kCFURLIsSystemImmutableKey: CFStringRef;
}
unsafe extern "C" {
    pub static kCFURLIsUserImmutableKey: CFStringRef;
}
unsafe extern "C" {
    pub static kCFURLIsHiddenKey: CFStringRef;
}
unsafe extern "C" {
    pub static kCFURLHasHiddenExtensionKey: CFStringRef;
}
unsafe extern "C" {
    pub static kCFURLCreationDateKey: CFStringRef;
}
unsafe extern "C" {
    pub static kCFURLContentAccessDateKey: CFStringRef;
}
unsafe extern "C" {
    pub static kCFURLContentModificationDateKey: CFStringRef;
}
unsafe extern "C" {
    pub static kCFURLAttributeModificationDateKey: CFStringRef;
}
unsafe extern "C" {
    pub static kCFURLFileIdentifierKey: CFStringRef;
}
unsafe extern "C" {
    pub static kCFURLFileContentIdentifierKey: CFStringRef;
}
unsafe extern "C" {
    pub static kCFURLMayShareFileContentKey: CFStringRef;
}
unsafe extern "C" {
    pub static kCFURLMayHaveExtendedAttributesKey: CFStringRef;
}
unsafe extern "C" {
    pub static kCFURLIsPurgeableKey: CFStringRef;
}
unsafe extern "C" {
    pub static kCFURLIsSparseKey: CFStringRef;
}
unsafe extern "C" {
    pub static kCFURLLinkCountKey: CFStringRef;
}
unsafe extern "C" {
    pub static kCFURLParentDirectoryURLKey: CFStringRef;
}
unsafe extern "C" {
    pub static kCFURLVolumeURLKey: CFStringRef;
}
unsafe extern "C" {
    pub static kCFURLTypeIdentifierKey: CFStringRef;
}
unsafe extern "C" {
    pub static kCFURLLocalizedTypeDescriptionKey: CFStringRef;
}
unsafe extern "C" {
    pub static kCFURLLabelNumberKey: CFStringRef;
}
unsafe extern "C" {
    pub static kCFURLLabelColorKey: CFStringRef;
}
unsafe extern "C" {
    pub static kCFURLLocalizedLabelKey: CFStringRef;
}
unsafe extern "C" {
    pub static kCFURLEffectiveIconKey: CFStringRef;
}
unsafe extern "C" {
    pub static kCFURLCustomIconKey: CFStringRef;
}
unsafe extern "C" {
    pub static kCFURLFileResourceIdentifierKey: CFStringRef;
}
unsafe extern "C" {
    pub static kCFURLVolumeIdentifierKey: CFStringRef;
}
unsafe extern "C" {
    pub static kCFURLPreferredIOBlockSizeKey: CFStringRef;
}
unsafe extern "C" {
    pub static kCFURLIsReadableKey: CFStringRef;
}
unsafe extern "C" {
    pub static kCFURLIsWritableKey: CFStringRef;
}
unsafe extern "C" {
    pub static kCFURLIsExecutableKey: CFStringRef;
}
unsafe extern "C" {
    pub static kCFURLFileSecurityKey: CFStringRef;
}
unsafe extern "C" {
    pub static kCFURLIsExcludedFromBackupKey: CFStringRef;
}
unsafe extern "C" {
    pub static kCFURLTagNamesKey: CFStringRef;
}
unsafe extern "C" {
    pub static kCFURLPathKey: CFStringRef;
}
unsafe extern "C" {
    pub static kCFURLCanonicalPathKey: CFStringRef;
}
unsafe extern "C" {
    pub static kCFURLIsMountTriggerKey: CFStringRef;
}
unsafe extern "C" {
    pub static kCFURLGenerationIdentifierKey: CFStringRef;
}
unsafe extern "C" {
    pub static kCFURLDocumentIdentifierKey: CFStringRef;
}
unsafe extern "C" {
    pub static kCFURLAddedToDirectoryDateKey: CFStringRef;
}
unsafe extern "C" {
    pub static kCFURLQuarantinePropertiesKey: CFStringRef;
}
unsafe extern "C" {
    pub static kCFURLFileResourceTypeKey: CFStringRef;
}
unsafe extern "C" {
    pub static kCFURLFileResourceTypeNamedPipe: CFStringRef;
}
unsafe extern "C" {
    pub static kCFURLFileResourceTypeCharacterSpecial: CFStringRef;
}
unsafe extern "C" {
    pub static kCFURLFileResourceTypeDirectory: CFStringRef;
}
unsafe extern "C" {
    pub static kCFURLFileResourceTypeBlockSpecial: CFStringRef;
}
unsafe extern "C" {
    pub static kCFURLFileResourceTypeRegular: CFStringRef;
}
unsafe extern "C" {
    pub static kCFURLFileResourceTypeSymbolicLink: CFStringRef;
}
unsafe extern "C" {
    pub static kCFURLFileResourceTypeSocket: CFStringRef;
}
unsafe extern "C" {
    pub static kCFURLFileResourceTypeUnknown: CFStringRef;
}
unsafe extern "C" {
    pub static kCFURLFileSizeKey: CFStringRef;
}
unsafe extern "C" {
    pub static kCFURLFileAllocatedSizeKey: CFStringRef;
}
unsafe extern "C" {
    pub static kCFURLTotalFileSizeKey: CFStringRef;
}
unsafe extern "C" {
    pub static kCFURLTotalFileAllocatedSizeKey: CFStringRef;
}
unsafe extern "C" {
    pub static kCFURLIsAliasFileKey: CFStringRef;
}
unsafe extern "C" {
    pub static kCFURLFileProtectionKey: CFStringRef;
}
unsafe extern "C" {
    pub static kCFURLFileProtectionNone: CFStringRef;
}
unsafe extern "C" {
    pub static kCFURLFileProtectionComplete: CFStringRef;
}
unsafe extern "C" {
    pub static kCFURLFileProtectionCompleteUnlessOpen: CFStringRef;
}
unsafe extern "C" {
    pub static kCFURLFileProtectionCompleteUntilFirstUserAuthentication: CFStringRef;
}
unsafe extern "C" {
    pub static kCFURLFileProtectionCompleteWhenUserInactive: CFStringRef;
}
unsafe extern "C" {
    pub static kCFURLDirectoryEntryCountKey: CFStringRef;
}
unsafe extern "C" {
    pub static kCFURLVolumeLocalizedFormatDescriptionKey: CFStringRef;
}
unsafe extern "C" {
    pub static kCFURLVolumeTotalCapacityKey: CFStringRef;
}
unsafe extern "C" {
    pub static kCFURLVolumeAvailableCapacityKey: CFStringRef;
}
unsafe extern "C" {
    pub static kCFURLVolumeAvailableCapacityForImportantUsageKey: CFStringRef;
}
unsafe extern "C" {
    pub static kCFURLVolumeAvailableCapacityForOpportunisticUsageKey: CFStringRef;
}
unsafe extern "C" {
    pub static kCFURLVolumeResourceCountKey: CFStringRef;
}
unsafe extern "C" {
    pub static kCFURLVolumeSupportsPersistentIDsKey: CFStringRef;
}
unsafe extern "C" {
    pub static kCFURLVolumeSupportsSymbolicLinksKey: CFStringRef;
}
unsafe extern "C" {
    pub static kCFURLVolumeSupportsHardLinksKey: CFStringRef;
}
unsafe extern "C" {
    pub static kCFURLVolumeSupportsJournalingKey: CFStringRef;
}
unsafe extern "C" {
    pub static kCFURLVolumeIsJournalingKey: CFStringRef;
}
unsafe extern "C" {
    pub static kCFURLVolumeSupportsSparseFilesKey: CFStringRef;
}
unsafe extern "C" {
    pub static kCFURLVolumeSupportsZeroRunsKey: CFStringRef;
}
unsafe extern "C" {
    pub static kCFURLVolumeSupportsCaseSensitiveNamesKey: CFStringRef;
}
unsafe extern "C" {
    pub static kCFURLVolumeSupportsCasePreservedNamesKey: CFStringRef;
}
unsafe extern "C" {
    pub static kCFURLVolumeSupportsRootDirectoryDatesKey: CFStringRef;
}
unsafe extern "C" {
    pub static kCFURLVolumeSupportsVolumeSizesKey: CFStringRef;
}
unsafe extern "C" {
    pub static kCFURLVolumeSupportsRenamingKey: CFStringRef;
}
unsafe extern "C" {
    pub static kCFURLVolumeSupportsAdvisoryFileLockingKey: CFStringRef;
}
unsafe extern "C" {
    pub static kCFURLVolumeSupportsExtendedSecurityKey: CFStringRef;
}
unsafe extern "C" {
    pub static kCFURLVolumeIsBrowsableKey: CFStringRef;
}
unsafe extern "C" {
    pub static kCFURLVolumeMaximumFileSizeKey: CFStringRef;
}
unsafe extern "C" {
    pub static kCFURLVolumeIsEjectableKey: CFStringRef;
}
unsafe extern "C" {
    pub static kCFURLVolumeIsRemovableKey: CFStringRef;
}
unsafe extern "C" {
    pub static kCFURLVolumeIsInternalKey: CFStringRef;
}
unsafe extern "C" {
    pub static kCFURLVolumeIsAutomountedKey: CFStringRef;
}
unsafe extern "C" {
    pub static kCFURLVolumeIsLocalKey: CFStringRef;
}
unsafe extern "C" {
    pub static kCFURLVolumeIsReadOnlyKey: CFStringRef;
}
unsafe extern "C" {
    pub static kCFURLVolumeCreationDateKey: CFStringRef;
}
unsafe extern "C" {
    pub static kCFURLVolumeURLForRemountingKey: CFStringRef;
}
unsafe extern "C" {
    pub static kCFURLVolumeUUIDStringKey: CFStringRef;
}
unsafe extern "C" {
    pub static kCFURLVolumeNameKey: CFStringRef;
}
unsafe extern "C" {
    pub static kCFURLVolumeLocalizedNameKey: CFStringRef;
}
unsafe extern "C" {
    pub static kCFURLVolumeIsEncryptedKey: CFStringRef;
}
unsafe extern "C" {
    pub static kCFURLVolumeIsRootFileSystemKey: CFStringRef;
}
unsafe extern "C" {
    pub static kCFURLVolumeSupportsCompressionKey: CFStringRef;
}
unsafe extern "C" {
    pub static kCFURLVolumeSupportsFileCloningKey: CFStringRef;
}
unsafe extern "C" {
    pub static kCFURLVolumeSupportsSwapRenamingKey: CFStringRef;
}
unsafe extern "C" {
    pub static kCFURLVolumeSupportsExclusiveRenamingKey: CFStringRef;
}
unsafe extern "C" {
    pub static kCFURLVolumeSupportsImmutableFilesKey: CFStringRef;
}
unsafe extern "C" {
    pub static kCFURLVolumeSupportsAccessPermissionsKey: CFStringRef;
}
unsafe extern "C" {
    pub static kCFURLVolumeSupportsFileProtectionKey: CFStringRef;
}
unsafe extern "C" {
    pub static kCFURLVolumeTypeNameKey: CFStringRef;
}
unsafe extern "C" {
    pub static kCFURLVolumeSubtypeKey: CFStringRef;
}
unsafe extern "C" {
    pub static kCFURLVolumeMountFromLocationKey: CFStringRef;
}
unsafe extern "C" {
    pub static kCFURLIsUbiquitousItemKey: CFStringRef;
}
unsafe extern "C" {
    pub static kCFURLUbiquitousItemHasUnresolvedConflictsKey: CFStringRef;
}
unsafe extern "C" {
    pub static kCFURLUbiquitousItemIsDownloadedKey: CFStringRef;
}
unsafe extern "C" {
    pub static kCFURLUbiquitousItemIsDownloadingKey: CFStringRef;
}
unsafe extern "C" {
    pub static kCFURLUbiquitousItemIsUploadedKey: CFStringRef;
}
unsafe extern "C" {
    pub static kCFURLUbiquitousItemIsUploadingKey: CFStringRef;
}
unsafe extern "C" {
    pub static kCFURLUbiquitousItemPercentDownloadedKey: CFStringRef;
}
unsafe extern "C" {
    pub static kCFURLUbiquitousItemPercentUploadedKey: CFStringRef;
}
unsafe extern "C" {
    pub static kCFURLUbiquitousItemDownloadingStatusKey: CFStringRef;
}
unsafe extern "C" {
    pub static kCFURLUbiquitousItemDownloadingErrorKey: CFStringRef;
}
unsafe extern "C" {
    pub static kCFURLUbiquitousItemUploadingErrorKey: CFStringRef;
}
unsafe extern "C" {
    pub static kCFURLUbiquitousItemIsExcludedFromSyncKey: CFStringRef;
}
unsafe extern "C" {
    pub static kCFURLUbiquitousItemDownloadingStatusNotDownloaded: CFStringRef;
}
unsafe extern "C" {
    pub static kCFURLUbiquitousItemDownloadingStatusDownloaded: CFStringRef;
}
unsafe extern "C" {
    pub static kCFURLUbiquitousItemDownloadingStatusCurrent: CFStringRef;
}
unsafe extern "C" {
    pub static kCFURLUbiquitousItemSupportedSyncControlsKey: CFStringRef;
}
unsafe extern "C" {
    pub static kCFURLUbiquitousItemIsSyncPausedKey: CFStringRef;
}
unsafe extern "C" {
    pub fn CFURLCreateBookmarkData(
        allocator: CFAllocatorRef,
        url: CFURLRef,
        options: CFURLBookmarkCreationOptions,
        resourcePropertiesToInclude: CFArrayRef,
        relativeToURL: CFURLRef,
        error: *mut CFErrorRef,
    ) -> CFDataRef;
}
unsafe extern "C" {
    pub fn CFURLCreateByResolvingBookmarkData(
        allocator: CFAllocatorRef,
        bookmark: CFDataRef,
        options: CFURLBookmarkResolutionOptions,
        relativeToURL: CFURLRef,
        resourcePropertiesToInclude: CFArrayRef,
        isStale: *mut Boolean,
        error: *mut CFErrorRef,
    ) -> CFURLRef;
}
unsafe extern "C" {
    pub fn CFURLCreateResourcePropertiesForKeysFromBookmarkData(
        allocator: CFAllocatorRef,
        resourcePropertiesToReturn: CFArrayRef,
        bookmark: CFDataRef,
    ) -> CFDictionaryRef;
}
unsafe extern "C" {
    pub fn CFURLCreateResourcePropertyForKeyFromBookmarkData(
        allocator: CFAllocatorRef,
        resourcePropertyKey: CFStringRef,
        bookmark: CFDataRef,
    ) -> CFTypeRef;
}
unsafe extern "C" {
    pub fn CFURLCreateBookmarkDataFromFile(
        allocator: CFAllocatorRef,
        fileURL: CFURLRef,
        errorRef: *mut CFErrorRef,
    ) -> CFDataRef;
}
unsafe extern "C" {
    pub fn CFURLWriteBookmarkDataToFile(
        bookmarkRef: CFDataRef,
        fileURL: CFURLRef,
        options: CFURLBookmarkFileCreationOptions,
        errorRef: *mut CFErrorRef,
    ) -> Boolean;
}
unsafe extern "C" {
    pub fn CFURLCreateBookmarkDataFromAliasRecord(
        allocatorRef: CFAllocatorRef,
        aliasRecordDataRef: CFDataRef,
    ) -> CFDataRef;
}
unsafe extern "C" {
    pub fn CFURLStartAccessingSecurityScopedResource(url: CFURLRef) -> Boolean;
}
unsafe extern "C" {
    pub fn CFURLStopAccessingSecurityScopedResource(url: CFURLRef);
}
unsafe extern "C" {
    pub static kCFRunLoopDefaultMode: CFRunLoopMode;
}
unsafe extern "C" {
    pub static kCFRunLoopCommonModes: CFRunLoopMode;
}
unsafe extern "C" {
    pub fn CFRunLoopGetTypeID() -> CFTypeID;
}
unsafe extern "C" {
    pub fn CFRunLoopGetCurrent() -> CFRunLoopRef;
}
unsafe extern "C" {
    pub fn CFRunLoopGetMain() -> CFRunLoopRef;
}
unsafe extern "C" {
    pub fn CFRunLoopCopyCurrentMode(rl: CFRunLoopRef) -> CFRunLoopMode;
}
unsafe extern "C" {
    pub fn CFRunLoopCopyAllModes(rl: CFRunLoopRef) -> CFArrayRef;
}
unsafe extern "C" {
    pub fn CFRunLoopAddCommonMode(rl: CFRunLoopRef, mode: CFRunLoopMode);
}
unsafe extern "C" {
    pub fn CFRunLoopGetNextTimerFireDate(rl: CFRunLoopRef, mode: CFRunLoopMode) -> CFAbsoluteTime;
}
unsafe extern "C" {
    pub fn CFRunLoopRun();
}
unsafe extern "C" {
    pub fn CFRunLoopRunInMode(
        mode: CFRunLoopMode,
        seconds: CFTimeInterval,
        returnAfterSourceHandled: Boolean,
    ) -> CFRunLoopRunResult;
}
unsafe extern "C" {
    pub fn CFRunLoopIsWaiting(rl: CFRunLoopRef) -> Boolean;
}
unsafe extern "C" {
    pub fn CFRunLoopWakeUp(rl: CFRunLoopRef);
}
unsafe extern "C" {
    pub fn CFRunLoopStop(rl: CFRunLoopRef);
}
unsafe extern "C" {
    pub fn CFRunLoopPerformBlock(
        rl: CFRunLoopRef,
        mode: CFTypeRef,
        block: *mut ::std::os::raw::c_void,
    );
}
unsafe extern "C" {
    pub fn CFRunLoopContainsSource(
        rl: CFRunLoopRef,
        source: CFRunLoopSourceRef,
        mode: CFRunLoopMode,
    ) -> Boolean;
}
unsafe extern "C" {
    pub fn CFRunLoopAddSource(rl: CFRunLoopRef, source: CFRunLoopSourceRef, mode: CFRunLoopMode);
}
unsafe extern "C" {
    pub fn CFRunLoopRemoveSource(rl: CFRunLoopRef, source: CFRunLoopSourceRef, mode: CFRunLoopMode);
}
unsafe extern "C" {
    pub fn CFRunLoopContainsObserver(
        rl: CFRunLoopRef,
        observer: CFRunLoopObserverRef,
        mode: CFRunLoopMode,
    ) -> Boolean;
}
unsafe extern "C" {
    pub fn CFRunLoopAddObserver(
        rl: CFRunLoopRef,
        observer: CFRunLoopObserverRef,
        mode: CFRunLoopMode,
    );
}
unsafe extern "C" {
    pub fn CFRunLoopRemoveObserver(
        rl: CFRunLoopRef,
        observer: CFRunLoopObserverRef,
        mode: CFRunLoopMode,
    );
}
unsafe extern "C" {
    pub fn CFRunLoopContainsTimer(
        rl: CFRunLoopRef,
        timer: CFRunLoopTimerRef,
        mode: CFRunLoopMode,
    ) -> Boolean;
}
unsafe extern "C" {
    pub fn CFRunLoopAddTimer(rl: CFRunLoopRef, timer: CFRunLoopTimerRef, mode: CFRunLoopMode);
}
unsafe extern "C" {
    pub fn CFRunLoopRemoveTimer(rl: CFRunLoopRef, timer: CFRunLoopTimerRef, mode: CFRunLoopMode);
}
unsafe extern "C" {
    pub fn CFRunLoopSourceGetTypeID() -> CFTypeID;
}
unsafe extern "C" {
    pub fn CFRunLoopSourceCreate(
        allocator: CFAllocatorRef,
        order: CFIndex,
        context: *mut CFRunLoopSourceContext,
    ) -> CFRunLoopSourceRef;
}
unsafe extern "C" {
    pub fn CFRunLoopSourceGetOrder(source: CFRunLoopSourceRef) -> CFIndex;
}
unsafe extern "C" {
    pub fn CFRunLoopSourceInvalidate(source: CFRunLoopSourceRef);
}
unsafe extern "C" {
    pub fn CFRunLoopSourceIsValid(source: CFRunLoopSourceRef) -> Boolean;
}
unsafe extern "C" {
    pub fn CFRunLoopSourceGetContext(
        source: CFRunLoopSourceRef,
        context: *mut CFRunLoopSourceContext,
    );
}
unsafe extern "C" {
    pub fn CFRunLoopSourceSignal(source: CFRunLoopSourceRef);
}
unsafe extern "C" {
    pub fn CFRunLoopObserverGetTypeID() -> CFTypeID;
}
unsafe extern "C" {
    pub fn CFRunLoopObserverCreate(
        allocator: CFAllocatorRef,
        activities: CFOptionFlags,
        repeats: Boolean,
        order: CFIndex,
        callout: CFRunLoopObserverCallBack,
        context: *mut CFRunLoopObserverContext,
    ) -> CFRunLoopObserverRef;
}
unsafe extern "C" {
    pub fn CFRunLoopObserverCreateWithHandler(
        allocator: CFAllocatorRef,
        activities: CFOptionFlags,
        repeats: Boolean,
        order: CFIndex,
        block: *mut ::std::os::raw::c_void,
    ) -> CFRunLoopObserverRef;
}
unsafe extern "C" {
    pub fn CFRunLoopObserverGetActivities(observer: CFRunLoopObserverRef) -> CFOptionFlags;
}
unsafe extern "C" {
    pub fn CFRunLoopObserverDoesRepeat(observer: CFRunLoopObserverRef) -> Boolean;
}
unsafe extern "C" {
    pub fn CFRunLoopObserverGetOrder(observer: CFRunLoopObserverRef) -> CFIndex;
}
unsafe extern "C" {
    pub fn CFRunLoopObserverInvalidate(observer: CFRunLoopObserverRef);
}
unsafe extern "C" {
    pub fn CFRunLoopObserverIsValid(observer: CFRunLoopObserverRef) -> Boolean;
}
unsafe extern "C" {
    pub fn CFRunLoopObserverGetContext(
        observer: CFRunLoopObserverRef,
        context: *mut CFRunLoopObserverContext,
    );
}
unsafe extern "C" {
    pub fn CFRunLoopTimerGetTypeID() -> CFTypeID;
}
unsafe extern "C" {
    pub fn CFRunLoopTimerCreate(
        allocator: CFAllocatorRef,
        fireDate: CFAbsoluteTime,
        interval: CFTimeInterval,
        flags: CFOptionFlags,
        order: CFIndex,
        callout: CFRunLoopTimerCallBack,
        context: *mut CFRunLoopTimerContext,
    ) -> CFRunLoopTimerRef;
}
unsafe extern "C" {
    pub fn CFRunLoopTimerCreateWithHandler(
        allocator: CFAllocatorRef,
        fireDate: CFAbsoluteTime,
        interval: CFTimeInterval,
        flags: CFOptionFlags,
        order: CFIndex,
        block: *mut ::std::os::raw::c_void,
    ) -> CFRunLoopTimerRef;
}
unsafe extern "C" {
    pub fn CFRunLoopTimerGetNextFireDate(timer: CFRunLoopTimerRef) -> CFAbsoluteTime;
}
unsafe extern "C" {
    pub fn CFRunLoopTimerSetNextFireDate(timer: CFRunLoopTimerRef, fireDate: CFAbsoluteTime);
}
unsafe extern "C" {
    pub fn CFRunLoopTimerGetInterval(timer: CFRunLoopTimerRef) -> CFTimeInterval;
}
unsafe extern "C" {
    pub fn CFRunLoopTimerDoesRepeat(timer: CFRunLoopTimerRef) -> Boolean;
}
unsafe extern "C" {
    pub fn CFRunLoopTimerGetOrder(timer: CFRunLoopTimerRef) -> CFIndex;
}
unsafe extern "C" {
    pub fn CFRunLoopTimerInvalidate(timer: CFRunLoopTimerRef);
}
unsafe extern "C" {
    pub fn CFRunLoopTimerIsValid(timer: CFRunLoopTimerRef) -> Boolean;
}
unsafe extern "C" {
    pub fn CFRunLoopTimerGetContext(timer: CFRunLoopTimerRef, context: *mut CFRunLoopTimerContext);
}
unsafe extern "C" {
    pub fn CFRunLoopTimerGetTolerance(timer: CFRunLoopTimerRef) -> CFTimeInterval;
}
unsafe extern "C" {
    pub fn CFRunLoopTimerSetTolerance(timer: CFRunLoopTimerRef, tolerance: CFTimeInterval);
}
unsafe extern "C" {
    pub fn CFSocketGetTypeID() -> CFTypeID;
}
unsafe extern "C" {
    pub fn CFSocketCreate(
        allocator: CFAllocatorRef,
        protocolFamily: SInt32,
        socketType: SInt32,
        protocol: SInt32,
        callBackTypes: CFOptionFlags,
        callout: CFSocketCallBack,
        context: *const CFSocketContext,
    ) -> CFSocketRef;
}
unsafe extern "C" {
    pub fn CFSocketCreateWithNative(
        allocator: CFAllocatorRef,
        sock: CFSocketNativeHandle,
        callBackTypes: CFOptionFlags,
        callout: CFSocketCallBack,
        context: *const CFSocketContext,
    ) -> CFSocketRef;
}
unsafe extern "C" {
    pub fn CFSocketCreateWithSocketSignature(
        allocator: CFAllocatorRef,
        signature: *const CFSocketSignature,
        callBackTypes: CFOptionFlags,
        callout: CFSocketCallBack,
        context: *const CFSocketContext,
    ) -> CFSocketRef;
}
unsafe extern "C" {
    pub fn CFSocketCreateConnectedToSocketSignature(
        allocator: CFAllocatorRef,
        signature: *const CFSocketSignature,
        callBackTypes: CFOptionFlags,
        callout: CFSocketCallBack,
        context: *const CFSocketContext,
        timeout: CFTimeInterval,
    ) -> CFSocketRef;
}
unsafe extern "C" {
    pub fn CFSocketSetAddress(s: CFSocketRef, address: CFDataRef) -> CFSocketError;
}
unsafe extern "C" {
    pub fn CFSocketConnectToAddress(
        s: CFSocketRef,
        address: CFDataRef,
        timeout: CFTimeInterval,
    ) -> CFSocketError;
}
unsafe extern "C" {
    pub fn CFSocketInvalidate(s: CFSocketRef);
}
unsafe extern "C" {
    pub fn CFSocketIsValid(s: CFSocketRef) -> Boolean;
}
unsafe extern "C" {
    pub fn CFSocketCopyAddress(s: CFSocketRef) -> CFDataRef;
}
unsafe extern "C" {
    pub fn CFSocketCopyPeerAddress(s: CFSocketRef) -> CFDataRef;
}
unsafe extern "C" {
    pub fn CFSocketGetContext(s: CFSocketRef, context: *mut CFSocketContext);
}
unsafe extern "C" {
    pub fn CFSocketGetNative(s: CFSocketRef) -> CFSocketNativeHandle;
}
unsafe extern "C" {
    pub fn CFSocketCreateRunLoopSource(
        allocator: CFAllocatorRef,
        s: CFSocketRef,
        order: CFIndex,
    ) -> CFRunLoopSourceRef;
}
unsafe extern "C" {
    pub fn CFSocketGetSocketFlags(s: CFSocketRef) -> CFOptionFlags;
}
unsafe extern "C" {
    pub fn CFSocketSetSocketFlags(s: CFSocketRef, flags: CFOptionFlags);
}
unsafe extern "C" {
    pub fn CFSocketDisableCallBacks(s: CFSocketRef, callBackTypes: CFOptionFlags);
}
unsafe extern "C" {
    pub fn CFSocketEnableCallBacks(s: CFSocketRef, callBackTypes: CFOptionFlags);
}
unsafe extern "C" {
    pub fn CFSocketSendData(
        s: CFSocketRef,
        address: CFDataRef,
        data: CFDataRef,
        timeout: CFTimeInterval,
    ) -> CFSocketError;
}
unsafe extern "C" {
    pub fn CFSocketRegisterValue(
        nameServerSignature: *const CFSocketSignature,
        timeout: CFTimeInterval,
        name: CFStringRef,
        value: CFPropertyListRef,
    ) -> CFSocketError;
}
unsafe extern "C" {
    pub fn CFSocketCopyRegisteredValue(
        nameServerSignature: *const CFSocketSignature,
        timeout: CFTimeInterval,
        name: CFStringRef,
        value: *mut CFPropertyListRef,
        nameServerAddress: *mut CFDataRef,
    ) -> CFSocketError;
}
unsafe extern "C" {
    pub fn CFSocketRegisterSocketSignature(
        nameServerSignature: *const CFSocketSignature,
        timeout: CFTimeInterval,
        name: CFStringRef,
        signature: *const CFSocketSignature,
    ) -> CFSocketError;
}
unsafe extern "C" {
    pub fn CFSocketCopyRegisteredSocketSignature(
        nameServerSignature: *const CFSocketSignature,
        timeout: CFTimeInterval,
        name: CFStringRef,
        signature: *mut CFSocketSignature,
        nameServerAddress: *mut CFDataRef,
    ) -> CFSocketError;
}
unsafe extern "C" {
    pub fn CFSocketUnregister(
        nameServerSignature: *const CFSocketSignature,
        timeout: CFTimeInterval,
        name: CFStringRef,
    ) -> CFSocketError;
}
unsafe extern "C" {
    pub fn CFSocketSetDefaultNameRegistryPortNumber(port: UInt16);
}
unsafe extern "C" {
    pub fn CFSocketGetDefaultNameRegistryPortNumber() -> UInt16;
}
unsafe extern "C" {
    pub static kCFSocketCommandKey: CFStringRef;
}
unsafe extern "C" {
    pub static kCFSocketNameKey: CFStringRef;
}
unsafe extern "C" {
    pub static kCFSocketValueKey: CFStringRef;
}
unsafe extern "C" {
    pub static kCFSocketResultKey: CFStringRef;
}
unsafe extern "C" {
    pub static kCFSocketErrorKey: CFStringRef;
}
unsafe extern "C" {
    pub static kCFSocketRegisterCommand: CFStringRef;
}
unsafe extern "C" {
    pub static kCFSocketRetrieveCommand: CFStringRef;
}
unsafe extern "C" {
    pub fn CFReadStreamGetTypeID() -> CFTypeID;
}
unsafe extern "C" {
    pub fn CFWriteStreamGetTypeID() -> CFTypeID;
}
unsafe extern "C" {
    pub static mut kCFStreamPropertyDataWritten: CFStreamPropertyKey;
}
unsafe extern "C" {
    pub fn CFReadStreamCreateWithBytesNoCopy(
        alloc: CFAllocatorRef,
        bytes: *const UInt8,
        length: CFIndex,
        bytesDeallocator: CFAllocatorRef,
    ) -> CFReadStreamRef;
}
unsafe extern "C" {
    pub fn CFWriteStreamCreateWithBuffer(
        alloc: CFAllocatorRef,
        buffer: *mut UInt8,
        bufferCapacity: CFIndex,
    ) -> CFWriteStreamRef;
}
unsafe extern "C" {
    pub fn CFWriteStreamCreateWithAllocatedBuffers(
        alloc: CFAllocatorRef,
        bufferAllocator: CFAllocatorRef,
    ) -> CFWriteStreamRef;
}
unsafe extern "C" {
    pub fn CFReadStreamCreateWithFile(alloc: CFAllocatorRef, fileURL: CFURLRef) -> CFReadStreamRef;
}
unsafe extern "C" {
    pub fn CFWriteStreamCreateWithFile(
        alloc: CFAllocatorRef,
        fileURL: CFURLRef,
    ) -> CFWriteStreamRef;
}
unsafe extern "C" {
    pub fn CFStreamCreateBoundPair(
        alloc: CFAllocatorRef,
        readStream: *mut CFReadStreamRef,
        writeStream: *mut CFWriteStreamRef,
        transferBufferSize: CFIndex,
    );
}
unsafe extern "C" {
    pub static mut kCFStreamPropertyAppendToFile: CFStreamPropertyKey;
}
unsafe extern "C" {
    pub static mut kCFStreamPropertyFileCurrentOffset: CFStreamPropertyKey;
}
unsafe extern "C" {
    pub static mut kCFStreamPropertySocketNativeHandle: CFStreamPropertyKey;
}
unsafe extern "C" {
    pub static mut kCFStreamPropertySocketRemoteHostName: CFStreamPropertyKey;
}
unsafe extern "C" {
    pub static mut kCFStreamPropertySocketRemotePortNumber: CFStreamPropertyKey;
}
unsafe extern "C" {
    pub static kCFStreamErrorDomainSOCKS: ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub static mut kCFStreamPropertySOCKSProxy: CFStringRef;
}
unsafe extern "C" {
    pub static mut kCFStreamPropertySOCKSProxyHost: CFStringRef;
}
unsafe extern "C" {
    pub static mut kCFStreamPropertySOCKSProxyPort: CFStringRef;
}
unsafe extern "C" {
    pub static mut kCFStreamPropertySOCKSVersion: CFStringRef;
}
unsafe extern "C" {
    pub static mut kCFStreamSocketSOCKSVersion4: CFStringRef;
}
unsafe extern "C" {
    pub static mut kCFStreamSocketSOCKSVersion5: CFStringRef;
}
unsafe extern "C" {
    pub static mut kCFStreamPropertySOCKSUser: CFStringRef;
}
unsafe extern "C" {
    pub static mut kCFStreamPropertySOCKSPassword: CFStringRef;
}
unsafe extern "C" {
    pub static kCFStreamErrorDomainSSL: ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub static mut kCFStreamPropertySocketSecurityLevel: CFStringRef;
}
unsafe extern "C" {
    pub static mut kCFStreamSocketSecurityLevelNone: CFStringRef;
}
unsafe extern "C" {
    pub static mut kCFStreamSocketSecurityLevelSSLv2: CFStringRef;
}
unsafe extern "C" {
    pub static mut kCFStreamSocketSecurityLevelSSLv3: CFStringRef;
}
unsafe extern "C" {
    pub static mut kCFStreamSocketSecurityLevelTLSv1: CFStringRef;
}
unsafe extern "C" {
    pub static mut kCFStreamSocketSecurityLevelNegotiatedSSL: CFStringRef;
}
unsafe extern "C" {
    pub static mut kCFStreamPropertyShouldCloseNativeSocket: CFStringRef;
}
unsafe extern "C" {
    pub fn CFStreamCreatePairWithSocket(
        alloc: CFAllocatorRef,
        sock: CFSocketNativeHandle,
        readStream: *mut CFReadStreamRef,
        writeStream: *mut CFWriteStreamRef,
    );
}
unsafe extern "C" {
    pub fn CFStreamCreatePairWithSocketToHost(
        alloc: CFAllocatorRef,
        host: CFStringRef,
        port: UInt32,
        readStream: *mut CFReadStreamRef,
        writeStream: *mut CFWriteStreamRef,
    );
}
unsafe extern "C" {
    pub fn CFStreamCreatePairWithPeerSocketSignature(
        alloc: CFAllocatorRef,
        signature: *const CFSocketSignature,
        readStream: *mut CFReadStreamRef,
        writeStream: *mut CFWriteStreamRef,
    );
}
unsafe extern "C" {
    pub fn CFReadStreamGetStatus(stream: CFReadStreamRef) -> CFStreamStatus;
}
unsafe extern "C" {
    pub fn CFWriteStreamGetStatus(stream: CFWriteStreamRef) -> CFStreamStatus;
}
unsafe extern "C" {
    pub fn CFReadStreamCopyError(stream: CFReadStreamRef) -> CFErrorRef;
}
unsafe extern "C" {
    pub fn CFWriteStreamCopyError(stream: CFWriteStreamRef) -> CFErrorRef;
}
unsafe extern "C" {
    pub fn CFReadStreamOpen(stream: CFReadStreamRef) -> Boolean;
}
unsafe extern "C" {
    pub fn CFWriteStreamOpen(stream: CFWriteStreamRef) -> Boolean;
}
unsafe extern "C" {
    pub fn CFReadStreamClose(stream: CFReadStreamRef);
}
unsafe extern "C" {
    pub fn CFWriteStreamClose(stream: CFWriteStreamRef);
}
unsafe extern "C" {
    pub fn CFReadStreamHasBytesAvailable(stream: CFReadStreamRef) -> Boolean;
}
unsafe extern "C" {
    pub fn CFReadStreamRead(
        stream: CFReadStreamRef,
        buffer: *mut UInt8,
        bufferLength: CFIndex,
    ) -> CFIndex;
}
unsafe extern "C" {
    pub fn CFReadStreamGetBuffer(
        stream: CFReadStreamRef,
        maxBytesToRead: CFIndex,
        numBytesRead: *mut CFIndex,
    ) -> *const UInt8;
}
unsafe extern "C" {
    pub fn CFWriteStreamCanAcceptBytes(stream: CFWriteStreamRef) -> Boolean;
}
unsafe extern "C" {
    pub fn CFWriteStreamWrite(
        stream: CFWriteStreamRef,
        buffer: *const UInt8,
        bufferLength: CFIndex,
    ) -> CFIndex;
}
unsafe extern "C" {
    pub fn CFReadStreamCopyProperty(
        stream: CFReadStreamRef,
        propertyName: CFStreamPropertyKey,
    ) -> CFTypeRef;
}
unsafe extern "C" {
    pub fn CFWriteStreamCopyProperty(
        stream: CFWriteStreamRef,
        propertyName: CFStreamPropertyKey,
    ) -> CFTypeRef;
}
unsafe extern "C" {
    pub fn CFReadStreamSetProperty(
        stream: CFReadStreamRef,
        propertyName: CFStreamPropertyKey,
        propertyValue: CFTypeRef,
    ) -> Boolean;
}
unsafe extern "C" {
    pub fn CFWriteStreamSetProperty(
        stream: CFWriteStreamRef,
        propertyName: CFStreamPropertyKey,
        propertyValue: CFTypeRef,
    ) -> Boolean;
}
unsafe extern "C" {
    pub fn CFReadStreamSetClient(
        stream: CFReadStreamRef,
        streamEvents: CFOptionFlags,
        clientCB: CFReadStreamClientCallBack,
        clientContext: *mut CFStreamClientContext,
    ) -> Boolean;
}
unsafe extern "C" {
    pub fn CFWriteStreamSetClient(
        stream: CFWriteStreamRef,
        streamEvents: CFOptionFlags,
        clientCB: CFWriteStreamClientCallBack,
        clientContext: *mut CFStreamClientContext,
    ) -> Boolean;
}
unsafe extern "C" {
    pub fn CFReadStreamScheduleWithRunLoop(
        stream: CFReadStreamRef,
        runLoop: CFRunLoopRef,
        runLoopMode: CFRunLoopMode,
    );
}
unsafe extern "C" {
    pub fn CFWriteStreamScheduleWithRunLoop(
        stream: CFWriteStreamRef,
        runLoop: CFRunLoopRef,
        runLoopMode: CFRunLoopMode,
    );
}
unsafe extern "C" {
    pub fn CFReadStreamUnscheduleFromRunLoop(
        stream: CFReadStreamRef,
        runLoop: CFRunLoopRef,
        runLoopMode: CFRunLoopMode,
    );
}
unsafe extern "C" {
    pub fn CFWriteStreamUnscheduleFromRunLoop(
        stream: CFWriteStreamRef,
        runLoop: CFRunLoopRef,
        runLoopMode: CFRunLoopMode,
    );
}
unsafe extern "C" {
    pub fn CFReadStreamSetDispatchQueue(stream: CFReadStreamRef, q: NSObject);
}
unsafe extern "C" {
    pub fn CFWriteStreamSetDispatchQueue(stream: CFWriteStreamRef, q: NSObject);
}
unsafe extern "C" {
    pub fn CFReadStreamCopyDispatchQueue(stream: CFReadStreamRef) -> dispatch_queue_t;
}
unsafe extern "C" {
    pub fn CFWriteStreamCopyDispatchQueue(stream: CFWriteStreamRef) -> dispatch_queue_t;
}
unsafe extern "C" {
    pub fn CFReadStreamGetError(stream: CFReadStreamRef) -> CFStreamError;
}
unsafe extern "C" {
    pub fn CFWriteStreamGetError(stream: CFWriteStreamRef) -> CFStreamError;
}
unsafe extern "C" {
    pub fn CFPropertyListCreateFromXMLData(
        allocator: CFAllocatorRef,
        xmlData: CFDataRef,
        mutabilityOption: CFOptionFlags,
        errorString: *mut CFStringRef,
    ) -> CFPropertyListRef;
}
unsafe extern "C" {
    pub fn CFPropertyListCreateXMLData(
        allocator: CFAllocatorRef,
        propertyList: CFPropertyListRef,
    ) -> CFDataRef;
}
unsafe extern "C" {
    pub fn CFPropertyListCreateDeepCopy(
        allocator: CFAllocatorRef,
        propertyList: CFPropertyListRef,
        mutabilityOption: CFOptionFlags,
    ) -> CFPropertyListRef;
}
unsafe extern "C" {
    pub fn CFPropertyListIsValid(plist: CFPropertyListRef, format: CFPropertyListFormat)
        -> Boolean;
}
unsafe extern "C" {
    pub fn CFPropertyListWriteToStream(
        propertyList: CFPropertyListRef,
        stream: CFWriteStreamRef,
        format: CFPropertyListFormat,
        errorString: *mut CFStringRef,
    ) -> CFIndex;
}
unsafe extern "C" {
    pub fn CFPropertyListCreateFromStream(
        allocator: CFAllocatorRef,
        stream: CFReadStreamRef,
        streamLength: CFIndex,
        mutabilityOption: CFOptionFlags,
        format: *mut CFPropertyListFormat,
        errorString: *mut CFStringRef,
    ) -> CFPropertyListRef;
}
unsafe extern "C" {
    pub fn CFPropertyListCreateWithData(
        allocator: CFAllocatorRef,
        data: CFDataRef,
        options: CFOptionFlags,
        format: *mut CFPropertyListFormat,
        error: *mut CFErrorRef,
    ) -> CFPropertyListRef;
}
unsafe extern "C" {
    pub fn CFPropertyListCreateWithStream(
        allocator: CFAllocatorRef,
        stream: CFReadStreamRef,
        streamLength: CFIndex,
        options: CFOptionFlags,
        format: *mut CFPropertyListFormat,
        error: *mut CFErrorRef,
    ) -> CFPropertyListRef;
}
unsafe extern "C" {
    pub fn CFPropertyListWrite(
        propertyList: CFPropertyListRef,
        stream: CFWriteStreamRef,
        format: CFPropertyListFormat,
        options: CFOptionFlags,
        error: *mut CFErrorRef,
    ) -> CFIndex;
}
unsafe extern "C" {
    pub fn CFPropertyListCreateData(
        allocator: CFAllocatorRef,
        propertyList: CFPropertyListRef,
        format: CFPropertyListFormat,
        options: CFOptionFlags,
        error: *mut CFErrorRef,
    ) -> CFDataRef;
}
unsafe extern "C" {
    pub static kCFTypeSetCallBacks: CFSetCallBacks;
}
unsafe extern "C" {
    pub static kCFCopyStringSetCallBacks: CFSetCallBacks;
}
unsafe extern "C" {
    pub fn CFSetGetTypeID() -> CFTypeID;
}
unsafe extern "C" {
    pub fn CFSetCreate(
        allocator: CFAllocatorRef,
        values: *mut *const ::std::os::raw::c_void,
        numValues: CFIndex,
        callBacks: *const CFSetCallBacks,
    ) -> CFSetRef;
}
unsafe extern "C" {
    pub fn CFSetCreateCopy(allocator: CFAllocatorRef, theSet: CFSetRef) -> CFSetRef;
}
unsafe extern "C" {
    pub fn CFSetCreateMutable(
        allocator: CFAllocatorRef,
        capacity: CFIndex,
        callBacks: *const CFSetCallBacks,
    ) -> CFMutableSetRef;
}
unsafe extern "C" {
    pub fn CFSetCreateMutableCopy(
        allocator: CFAllocatorRef,
        capacity: CFIndex,
        theSet: CFSetRef,
    ) -> CFMutableSetRef;
}
unsafe extern "C" {
    pub fn CFSetGetCount(theSet: CFSetRef) -> CFIndex;
}
unsafe extern "C" {
    pub fn CFSetGetCountOfValue(theSet: CFSetRef, value: *const ::std::os::raw::c_void) -> CFIndex;
}
unsafe extern "C" {
    pub fn CFSetContainsValue(theSet: CFSetRef, value: *const ::std::os::raw::c_void) -> Boolean;
}
unsafe extern "C" {
    pub fn CFSetGetValue(
        theSet: CFSetRef,
        value: *const ::std::os::raw::c_void,
    ) -> *const ::std::os::raw::c_void;
}
unsafe extern "C" {
    pub fn CFSetGetValueIfPresent(
        theSet: CFSetRef,
        candidate: *const ::std::os::raw::c_void,
        value: *mut *const ::std::os::raw::c_void,
    ) -> Boolean;
}
unsafe extern "C" {
    pub fn CFSetGetValues(theSet: CFSetRef, values: *mut *const ::std::os::raw::c_void);
}
unsafe extern "C" {
    pub fn CFSetApplyFunction(
        theSet: CFSetRef,
        applier: CFSetApplierFunction,
        context: *mut ::std::os::raw::c_void,
    );
}
unsafe extern "C" {
    pub fn CFSetAddValue(theSet: CFMutableSetRef, value: *const ::std::os::raw::c_void);
}
unsafe extern "C" {
    pub fn CFSetReplaceValue(theSet: CFMutableSetRef, value: *const ::std::os::raw::c_void);
}
unsafe extern "C" {
    pub fn CFSetSetValue(theSet: CFMutableSetRef, value: *const ::std::os::raw::c_void);
}
unsafe extern "C" {
    pub fn CFSetRemoveValue(theSet: CFMutableSetRef, value: *const ::std::os::raw::c_void);
}
unsafe extern "C" {
    pub fn CFSetRemoveAllValues(theSet: CFMutableSetRef);
}
unsafe extern "C" {
    pub fn CFTreeGetTypeID() -> CFTypeID;
}
unsafe extern "C" {
    pub fn CFTreeCreate(allocator: CFAllocatorRef, context: *const CFTreeContext) -> CFTreeRef;
}
unsafe extern "C" {
    pub fn CFTreeGetParent(tree: CFTreeRef) -> CFTreeRef;
}
unsafe extern "C" {
    pub fn CFTreeGetNextSibling(tree: CFTreeRef) -> CFTreeRef;
}
unsafe extern "C" {
    pub fn CFTreeGetFirstChild(tree: CFTreeRef) -> CFTreeRef;
}
unsafe extern "C" {
    pub fn CFTreeGetContext(tree: CFTreeRef, context: *mut CFTreeContext);
}
unsafe extern "C" {
    pub fn CFTreeGetChildCount(tree: CFTreeRef) -> CFIndex;
}
unsafe extern "C" {
    pub fn CFTreeGetChildAtIndex(tree: CFTreeRef, idx: CFIndex) -> CFTreeRef;
}
unsafe extern "C" {
    pub fn CFTreeGetChildren(tree: CFTreeRef, children: *mut CFTreeRef);
}
unsafe extern "C" {
    pub fn CFTreeApplyFunctionToChildren(
        tree: CFTreeRef,
        applier: CFTreeApplierFunction,
        context: *mut ::std::os::raw::c_void,
    );
}
unsafe extern "C" {
    pub fn CFTreeFindRoot(tree: CFTreeRef) -> CFTreeRef;
}
unsafe extern "C" {
    pub fn CFTreeSetContext(tree: CFTreeRef, context: *const CFTreeContext);
}
unsafe extern "C" {
    pub fn CFTreePrependChild(tree: CFTreeRef, newChild: CFTreeRef);
}
unsafe extern "C" {
    pub fn CFTreeAppendChild(tree: CFTreeRef, newChild: CFTreeRef);
}
unsafe extern "C" {
    pub fn CFTreeInsertSibling(tree: CFTreeRef, newSibling: CFTreeRef);
}
unsafe extern "C" {
    pub fn CFTreeRemove(tree: CFTreeRef);
}
unsafe extern "C" {
    pub fn CFTreeRemoveAllChildren(tree: CFTreeRef);
}
unsafe extern "C" {
    pub fn CFTreeSortChildren(
        tree: CFTreeRef,
        comparator: CFComparatorFunction,
        context: *mut ::std::os::raw::c_void,
    );
}
unsafe extern "C" {
    pub fn CFURLCreateDataAndPropertiesFromResource(
        alloc: CFAllocatorRef,
        url: CFURLRef,
        resourceData: *mut CFDataRef,
        properties: *mut CFDictionaryRef,
        desiredProperties: CFArrayRef,
        errorCode: *mut SInt32,
    ) -> Boolean;
}
unsafe extern "C" {
    pub fn CFURLWriteDataAndPropertiesToResource(
        url: CFURLRef,
        dataToWrite: CFDataRef,
        propertiesToWrite: CFDictionaryRef,
        errorCode: *mut SInt32,
    ) -> Boolean;
}
unsafe extern "C" {
    pub fn CFURLDestroyResource(url: CFURLRef, errorCode: *mut SInt32) -> Boolean;
}
unsafe extern "C" {
    pub fn CFURLCreatePropertyFromResource(
        alloc: CFAllocatorRef,
        url: CFURLRef,
        property: CFStringRef,
        errorCode: *mut SInt32,
    ) -> CFTypeRef;
}
unsafe extern "C" {
    pub static kCFURLFileExists: CFStringRef;
}
unsafe extern "C" {
    pub static kCFURLFileDirectoryContents: CFStringRef;
}
unsafe extern "C" {
    pub static kCFURLFileLength: CFStringRef;
}
unsafe extern "C" {
    pub static kCFURLFileLastModificationTime: CFStringRef;
}
unsafe extern "C" {
    pub static kCFURLFilePOSIXMode: CFStringRef;
}
unsafe extern "C" {
    pub static kCFURLFileOwnerID: CFStringRef;
}
unsafe extern "C" {
    pub static kCFURLHTTPStatusCode: CFStringRef;
}
unsafe extern "C" {
    pub static kCFURLHTTPStatusLine: CFStringRef;
}
unsafe extern "C" {
    pub fn CFUUIDGetTypeID() -> CFTypeID;
}
unsafe extern "C" {
    pub fn CFUUIDCreate(alloc: CFAllocatorRef) -> CFUUIDRef;
}
unsafe extern "C" {
    pub fn CFUUIDCreateWithBytes(
        alloc: CFAllocatorRef,
        byte0: UInt8,
        byte1: UInt8,
        byte2: UInt8,
        byte3: UInt8,
        byte4: UInt8,
        byte5: UInt8,
        byte6: UInt8,
        byte7: UInt8,
        byte8: UInt8,
        byte9: UInt8,
        byte10: UInt8,
        byte11: UInt8,
        byte12: UInt8,
        byte13: UInt8,
        byte14: UInt8,
        byte15: UInt8,
    ) -> CFUUIDRef;
}
unsafe extern "C" {
    pub fn CFUUIDCreateFromString(alloc: CFAllocatorRef, uuidStr: CFStringRef) -> CFUUIDRef;
}
unsafe extern "C" {
    pub fn CFUUIDCreateString(alloc: CFAllocatorRef, uuid: CFUUIDRef) -> CFStringRef;
}
unsafe extern "C" {
    pub fn CFUUIDGetConstantUUIDWithBytes(
        alloc: CFAllocatorRef,
        byte0: UInt8,
        byte1: UInt8,
        byte2: UInt8,
        byte3: UInt8,
        byte4: UInt8,
        byte5: UInt8,
        byte6: UInt8,
        byte7: UInt8,
        byte8: UInt8,
        byte9: UInt8,
        byte10: UInt8,
        byte11: UInt8,
        byte12: UInt8,
        byte13: UInt8,
        byte14: UInt8,
        byte15: UInt8,
    ) -> CFUUIDRef;
}
unsafe extern "C" {
    pub fn CFUUIDGetUUIDBytes(uuid: CFUUIDRef) -> CFUUIDBytes;
}
unsafe extern "C" {
    pub fn CFUUIDCreateFromUUIDBytes(alloc: CFAllocatorRef, bytes: CFUUIDBytes) -> CFUUIDRef;
}
unsafe extern "C" {
    pub fn CFCopyHomeDirectoryURL() -> CFURLRef;
}
unsafe extern "C" {
    pub static kCFBundleInfoDictionaryVersionKey: CFStringRef;
}
unsafe extern "C" {
    pub static kCFBundleExecutableKey: CFStringRef;
}
unsafe extern "C" {
    pub static kCFBundleIdentifierKey: CFStringRef;
}
unsafe extern "C" {
    pub static kCFBundleVersionKey: CFStringRef;
}
unsafe extern "C" {
    pub static kCFBundleDevelopmentRegionKey: CFStringRef;
}
unsafe extern "C" {
    pub static kCFBundleNameKey: CFStringRef;
}
unsafe extern "C" {
    pub static kCFBundleLocalizationsKey: CFStringRef;
}
unsafe extern "C" {
    pub fn CFBundleGetMainBundle() -> CFBundleRef;
}
unsafe extern "C" {
    pub fn CFBundleGetBundleWithIdentifier(bundleID: CFStringRef) -> CFBundleRef;
}
unsafe extern "C" {
    pub fn CFBundleGetAllBundles() -> CFArrayRef;
}
unsafe extern "C" {
    pub fn CFBundleGetTypeID() -> CFTypeID;
}
unsafe extern "C" {
    pub fn CFBundleCreate(allocator: CFAllocatorRef, bundleURL: CFURLRef) -> CFBundleRef;
}
unsafe extern "C" {
    pub fn CFBundleCreateBundlesFromDirectory(
        allocator: CFAllocatorRef,
        directoryURL: CFURLRef,
        bundleType: CFStringRef,
    ) -> CFArrayRef;
}
unsafe extern "C" {
    pub fn CFBundleCopyBundleURL(bundle: CFBundleRef) -> CFURLRef;
}
unsafe extern "C" {
    pub fn CFBundleGetValueForInfoDictionaryKey(bundle: CFBundleRef, key: CFStringRef)
        -> CFTypeRef;
}
unsafe extern "C" {
    pub fn CFBundleGetInfoDictionary(bundle: CFBundleRef) -> CFDictionaryRef;
}
unsafe extern "C" {
    pub fn CFBundleGetLocalInfoDictionary(bundle: CFBundleRef) -> CFDictionaryRef;
}
unsafe extern "C" {
    pub fn CFBundleGetPackageInfo(
        bundle: CFBundleRef,
        packageType: *mut UInt32,
        packageCreator: *mut UInt32,
    );
}
unsafe extern "C" {
    pub fn CFBundleGetIdentifier(bundle: CFBundleRef) -> CFStringRef;
}
unsafe extern "C" {
    pub fn CFBundleGetVersionNumber(bundle: CFBundleRef) -> UInt32;
}
unsafe extern "C" {
    pub fn CFBundleGetDevelopmentRegion(bundle: CFBundleRef) -> CFStringRef;
}
unsafe extern "C" {
    pub fn CFBundleCopySupportFilesDirectoryURL(bundle: CFBundleRef) -> CFURLRef;
}
unsafe extern "C" {
    pub fn CFBundleCopyResourcesDirectoryURL(bundle: CFBundleRef) -> CFURLRef;
}
unsafe extern "C" {
    pub fn CFBundleCopyPrivateFrameworksURL(bundle: CFBundleRef) -> CFURLRef;
}
unsafe extern "C" {
    pub fn CFBundleCopySharedFrameworksURL(bundle: CFBundleRef) -> CFURLRef;
}
unsafe extern "C" {
    pub fn CFBundleCopySharedSupportURL(bundle: CFBundleRef) -> CFURLRef;
}
unsafe extern "C" {
    pub fn CFBundleCopyBuiltInPlugInsURL(bundle: CFBundleRef) -> CFURLRef;
}
unsafe extern "C" {
    pub fn CFBundleCopyInfoDictionaryInDirectory(bundleURL: CFURLRef) -> CFDictionaryRef;
}
unsafe extern "C" {
    pub fn CFBundleGetPackageInfoInDirectory(
        url: CFURLRef,
        packageType: *mut UInt32,
        packageCreator: *mut UInt32,
    ) -> Boolean;
}
unsafe extern "C" {
    pub fn CFBundleCopyResourceURL(
        bundle: CFBundleRef,
        resourceName: CFStringRef,
        resourceType: CFStringRef,
        subDirName: CFStringRef,
    ) -> CFURLRef;
}
unsafe extern "C" {
    pub fn CFBundleCopyResourceURLsOfType(
        bundle: CFBundleRef,
        resourceType: CFStringRef,
        subDirName: CFStringRef,
    ) -> CFArrayRef;
}
unsafe extern "C" {
    pub fn CFBundleCopyLocalizedString(
        bundle: CFBundleRef,
        key: CFStringRef,
        value: CFStringRef,
        tableName: CFStringRef,
    ) -> CFStringRef;
}
unsafe extern "C" {
    pub fn CFBundleCopyLocalizedStringForLocalizations(
        bundle: CFBundleRef,
        key: CFStringRef,
        value: CFStringRef,
        tableName: CFStringRef,
        localizations: CFArrayRef,
    ) -> CFStringRef;
}
unsafe extern "C" {
    pub fn CFBundleCopyResourceURLInDirectory(
        bundleURL: CFURLRef,
        resourceName: CFStringRef,
        resourceType: CFStringRef,
        subDirName: CFStringRef,
    ) -> CFURLRef;
}
unsafe extern "C" {
    pub fn CFBundleCopyResourceURLsOfTypeInDirectory(
        bundleURL: CFURLRef,
        resourceType: CFStringRef,
        subDirName: CFStringRef,
    ) -> CFArrayRef;
}
unsafe extern "C" {
    pub fn CFBundleCopyBundleLocalizations(bundle: CFBundleRef) -> CFArrayRef;
}
unsafe extern "C" {
    pub fn CFBundleCopyPreferredLocalizationsFromArray(locArray: CFArrayRef) -> CFArrayRef;
}
unsafe extern "C" {
    pub fn CFBundleCopyLocalizationsForPreferences(
        locArray: CFArrayRef,
        prefArray: CFArrayRef,
    ) -> CFArrayRef;
}
unsafe extern "C" {
    pub fn CFBundleCopyResourceURLForLocalization(
        bundle: CFBundleRef,
        resourceName: CFStringRef,
        resourceType: CFStringRef,
        subDirName: CFStringRef,
        localizationName: CFStringRef,
    ) -> CFURLRef;
}
unsafe extern "C" {
    pub fn CFBundleCopyResourceURLsOfTypeForLocalization(
        bundle: CFBundleRef,
        resourceType: CFStringRef,
        subDirName: CFStringRef,
        localizationName: CFStringRef,
    ) -> CFArrayRef;
}
unsafe extern "C" {
    pub fn CFBundleCopyInfoDictionaryForURL(url: CFURLRef) -> CFDictionaryRef;
}
unsafe extern "C" {
    pub fn CFBundleCopyLocalizationsForURL(url: CFURLRef) -> CFArrayRef;
}
unsafe extern "C" {
    pub fn CFBundleCopyExecutableArchitecturesForURL(url: CFURLRef) -> CFArrayRef;
}
unsafe extern "C" {
    pub fn CFBundleCopyExecutableURL(bundle: CFBundleRef) -> CFURLRef;
}
unsafe extern "C" {
    pub fn CFBundleCopyExecutableArchitectures(bundle: CFBundleRef) -> CFArrayRef;
}
unsafe extern "C" {
    pub fn CFBundlePreflightExecutable(bundle: CFBundleRef, error: *mut CFErrorRef) -> Boolean;
}
unsafe extern "C" {
    pub fn CFBundleLoadExecutableAndReturnError(
        bundle: CFBundleRef,
        error: *mut CFErrorRef,
    ) -> Boolean;
}
unsafe extern "C" {
    pub fn CFBundleLoadExecutable(bundle: CFBundleRef) -> Boolean;
}
unsafe extern "C" {
    pub fn CFBundleIsExecutableLoaded(bundle: CFBundleRef) -> Boolean;
}
unsafe extern "C" {
    pub fn CFBundleUnloadExecutable(bundle: CFBundleRef);
}
unsafe extern "C" {
    pub fn CFBundleGetFunctionPointerForName(
        bundle: CFBundleRef,
        functionName: CFStringRef,
    ) -> *mut ::std::os::raw::c_void;
}
unsafe extern "C" {
    pub fn CFBundleGetFunctionPointersForNames(
        bundle: CFBundleRef,
        functionNames: CFArrayRef,
        ftbl: *mut *mut ::std::os::raw::c_void,
    );
}
unsafe extern "C" {
    pub fn CFBundleGetDataPointerForName(
        bundle: CFBundleRef,
        symbolName: CFStringRef,
    ) -> *mut ::std::os::raw::c_void;
}
unsafe extern "C" {
    pub fn CFBundleGetDataPointersForNames(
        bundle: CFBundleRef,
        symbolNames: CFArrayRef,
        stbl: *mut *mut ::std::os::raw::c_void,
    );
}
unsafe extern "C" {
    pub fn CFBundleCopyAuxiliaryExecutableURL(
        bundle: CFBundleRef,
        executableName: CFStringRef,
    ) -> CFURLRef;
}
unsafe extern "C" {
    pub fn CFBundleIsExecutableLoadable(bundle: CFBundleRef) -> Boolean;
}
unsafe extern "C" {
    pub fn CFBundleIsExecutableLoadableForURL(url: CFURLRef) -> Boolean;
}
unsafe extern "C" {
    pub fn CFBundleIsArchitectureLoadable(arch: cpu_type_t) -> Boolean;
}
unsafe extern "C" {
    pub fn CFBundleGetPlugIn(bundle: CFBundleRef) -> CFPlugInRef;
}
unsafe extern "C" {
    pub fn CFBundleOpenBundleResourceMap(bundle: CFBundleRef) -> CFBundleRefNum;
}
unsafe extern "C" {
    pub fn CFBundleOpenBundleResourceFiles(
        bundle: CFBundleRef,
        refNum: *mut CFBundleRefNum,
        localizedRefNum: *mut CFBundleRefNum,
    ) -> SInt32;
}
unsafe extern "C" {
    pub fn CFBundleCloseBundleResourceMap(bundle: CFBundleRef, refNum: CFBundleRefNum);
}
unsafe extern "C" {
    pub fn CFMessagePortGetTypeID() -> CFTypeID;
}
unsafe extern "C" {
    pub fn CFMessagePortCreateLocal(
        allocator: CFAllocatorRef,
        name: CFStringRef,
        callout: CFMessagePortCallBack,
        context: *mut CFMessagePortContext,
        shouldFreeInfo: *mut Boolean,
    ) -> CFMessagePortRef;
}
unsafe extern "C" {
    pub fn CFMessagePortCreateRemote(
        allocator: CFAllocatorRef,
        name: CFStringRef,
    ) -> CFMessagePortRef;
}
unsafe extern "C" {
    pub fn CFMessagePortIsRemote(ms: CFMessagePortRef) -> Boolean;
}
unsafe extern "C" {
    pub fn CFMessagePortGetName(ms: CFMessagePortRef) -> CFStringRef;
}
unsafe extern "C" {
    pub fn CFMessagePortSetName(ms: CFMessagePortRef, newName: CFStringRef) -> Boolean;
}
unsafe extern "C" {
    pub fn CFMessagePortGetContext(ms: CFMessagePortRef, context: *mut CFMessagePortContext);
}
unsafe extern "C" {
    pub fn CFMessagePortInvalidate(ms: CFMessagePortRef);
}
unsafe extern "C" {
    pub fn CFMessagePortIsValid(ms: CFMessagePortRef) -> Boolean;
}
unsafe extern "C" {
    pub fn CFMessagePortGetInvalidationCallBack(
        ms: CFMessagePortRef,
    ) -> CFMessagePortInvalidationCallBack;
}
unsafe extern "C" {
    pub fn CFMessagePortSetInvalidationCallBack(
        ms: CFMessagePortRef,
        callout: CFMessagePortInvalidationCallBack,
    );
}
unsafe extern "C" {
    pub fn CFMessagePortSendRequest(
        remote: CFMessagePortRef,
        msgid: SInt32,
        data: CFDataRef,
        sendTimeout: CFTimeInterval,
        rcvTimeout: CFTimeInterval,
        replyMode: CFStringRef,
        returnData: *mut CFDataRef,
    ) -> SInt32;
}
unsafe extern "C" {
    pub fn CFMessagePortCreateRunLoopSource(
        allocator: CFAllocatorRef,
        local: CFMessagePortRef,
        order: CFIndex,
    ) -> CFRunLoopSourceRef;
}
unsafe extern "C" {
    pub fn CFMessagePortSetDispatchQueue(ms: CFMessagePortRef, queue: NSObject);
}
unsafe extern "C" {
    pub static kCFPlugInDynamicRegistrationKey: CFStringRef;
}
unsafe extern "C" {
    pub static kCFPlugInDynamicRegisterFunctionKey: CFStringRef;
}
unsafe extern "C" {
    pub static kCFPlugInUnloadFunctionKey: CFStringRef;
}
unsafe extern "C" {
    pub static kCFPlugInFactoriesKey: CFStringRef;
}
unsafe extern "C" {
    pub static kCFPlugInTypesKey: CFStringRef;
}
unsafe extern "C" {
    pub fn CFPlugInGetTypeID() -> CFTypeID;
}
unsafe extern "C" {
    pub fn CFPlugInCreate(allocator: CFAllocatorRef, plugInURL: CFURLRef) -> CFPlugInRef;
}
unsafe extern "C" {
    pub fn CFPlugInGetBundle(plugIn: CFPlugInRef) -> CFBundleRef;
}
unsafe extern "C" {
    pub fn CFPlugInSetLoadOnDemand(plugIn: CFPlugInRef, flag: Boolean);
}
unsafe extern "C" {
    pub fn CFPlugInIsLoadOnDemand(plugIn: CFPlugInRef) -> Boolean;
}
unsafe extern "C" {
    pub fn CFPlugInFindFactoriesForPlugInType(typeUUID: CFUUIDRef) -> CFArrayRef;
}
unsafe extern "C" {
    pub fn CFPlugInFindFactoriesForPlugInTypeInPlugIn(
        typeUUID: CFUUIDRef,
        plugIn: CFPlugInRef,
    ) -> CFArrayRef;
}
unsafe extern "C" {
    pub fn CFPlugInInstanceCreate(
        allocator: CFAllocatorRef,
        factoryUUID: CFUUIDRef,
        typeUUID: CFUUIDRef,
    ) -> *mut ::std::os::raw::c_void;
}
unsafe extern "C" {
    pub fn CFPlugInRegisterFactoryFunction(
        factoryUUID: CFUUIDRef,
        func: CFPlugInFactoryFunction,
    ) -> Boolean;
}
unsafe extern "C" {
    pub fn CFPlugInRegisterFactoryFunctionByName(
        factoryUUID: CFUUIDRef,
        plugIn: CFPlugInRef,
        functionName: CFStringRef,
    ) -> Boolean;
}
unsafe extern "C" {
    pub fn CFPlugInUnregisterFactory(factoryUUID: CFUUIDRef) -> Boolean;
}
unsafe extern "C" {
    pub fn CFPlugInRegisterPlugInType(factoryUUID: CFUUIDRef, typeUUID: CFUUIDRef) -> Boolean;
}
unsafe extern "C" {
    pub fn CFPlugInUnregisterPlugInType(factoryUUID: CFUUIDRef, typeUUID: CFUUIDRef) -> Boolean;
}
unsafe extern "C" {
    pub fn CFPlugInAddInstanceForFactory(factoryID: CFUUIDRef);
}
unsafe extern "C" {
    pub fn CFPlugInRemoveInstanceForFactory(factoryID: CFUUIDRef);
}
unsafe extern "C" {
    pub fn CFPlugInInstanceGetInterfaceFunctionTable(
        instance: CFPlugInInstanceRef,
        interfaceName: CFStringRef,
        ftbl: *mut *mut ::std::os::raw::c_void,
    ) -> Boolean;
}
unsafe extern "C" {
    pub fn CFPlugInInstanceGetFactoryName(instance: CFPlugInInstanceRef) -> CFStringRef;
}
unsafe extern "C" {
    pub fn CFPlugInInstanceGetInstanceData(
        instance: CFPlugInInstanceRef,
    ) -> *mut ::std::os::raw::c_void;
}
unsafe extern "C" {
    pub fn CFPlugInInstanceGetTypeID() -> CFTypeID;
}
unsafe extern "C" {
    pub fn CFPlugInInstanceCreateWithInstanceDataSize(
        allocator: CFAllocatorRef,
        instanceDataSize: CFIndex,
        deallocateInstanceFunction: CFPlugInInstanceDeallocateInstanceDataFunction,
        factoryName: CFStringRef,
        getInterfaceFunction: CFPlugInInstanceGetInterfaceFunction,
    ) -> CFPlugInInstanceRef;
}
unsafe extern "C" {
    pub fn CFMachPortGetTypeID() -> CFTypeID;
}
unsafe extern "C" {
    pub fn CFMachPortCreate(
        allocator: CFAllocatorRef,
        callout: CFMachPortCallBack,
        context: *mut CFMachPortContext,
        shouldFreeInfo: *mut Boolean,
    ) -> CFMachPortRef;
}
unsafe extern "C" {
    pub fn CFMachPortCreateWithPort(
        allocator: CFAllocatorRef,
        portNum: mach_port_t,
        callout: CFMachPortCallBack,
        context: *mut CFMachPortContext,
        shouldFreeInfo: *mut Boolean,
    ) -> CFMachPortRef;
}
unsafe extern "C" {
    pub fn CFMachPortGetPort(port: CFMachPortRef) -> mach_port_t;
}
unsafe extern "C" {
    pub fn CFMachPortGetContext(port: CFMachPortRef, context: *mut CFMachPortContext);
}
unsafe extern "C" {
    pub fn CFMachPortInvalidate(port: CFMachPortRef);
}
unsafe extern "C" {
    pub fn CFMachPortIsValid(port: CFMachPortRef) -> Boolean;
}
unsafe extern "C" {
    pub fn CFMachPortGetInvalidationCallBack(port: CFMachPortRef)
        -> CFMachPortInvalidationCallBack;
}
unsafe extern "C" {
    pub fn CFMachPortSetInvalidationCallBack(
        port: CFMachPortRef,
        callout: CFMachPortInvalidationCallBack,
    );
}
unsafe extern "C" {
    pub fn CFMachPortCreateRunLoopSource(
        allocator: CFAllocatorRef,
        port: CFMachPortRef,
        order: CFIndex,
    ) -> CFRunLoopSourceRef;
}
unsafe extern "C" {
    pub fn CFAttributedStringGetTypeID() -> CFTypeID;
}
unsafe extern "C" {
    pub fn CFAttributedStringCreate(
        alloc: CFAllocatorRef,
        str_: CFStringRef,
        attributes: CFDictionaryRef,
    ) -> CFAttributedStringRef;
}
unsafe extern "C" {
    pub fn CFAttributedStringCreateWithSubstring(
        alloc: CFAllocatorRef,
        aStr: CFAttributedStringRef,
        range: CFRange,
    ) -> CFAttributedStringRef;
}
unsafe extern "C" {
    pub fn CFAttributedStringCreateCopy(
        alloc: CFAllocatorRef,
        aStr: CFAttributedStringRef,
    ) -> CFAttributedStringRef;
}
unsafe extern "C" {
    pub fn CFAttributedStringGetString(aStr: CFAttributedStringRef) -> CFStringRef;
}
unsafe extern "C" {
    pub fn CFAttributedStringGetLength(aStr: CFAttributedStringRef) -> CFIndex;
}
unsafe extern "C" {
    pub fn CFAttributedStringGetAttributes(
        aStr: CFAttributedStringRef,
        loc: CFIndex,
        effectiveRange: *mut CFRange,
    ) -> CFDictionaryRef;
}
unsafe extern "C" {
    pub fn CFAttributedStringGetAttribute(
        aStr: CFAttributedStringRef,
        loc: CFIndex,
        attrName: CFStringRef,
        effectiveRange: *mut CFRange,
    ) -> CFTypeRef;
}
unsafe extern "C" {
    pub fn CFAttributedStringGetAttributesAndLongestEffectiveRange(
        aStr: CFAttributedStringRef,
        loc: CFIndex,
        inRange: CFRange,
        longestEffectiveRange: *mut CFRange,
    ) -> CFDictionaryRef;
}
unsafe extern "C" {
    pub fn CFAttributedStringGetAttributeAndLongestEffectiveRange(
        aStr: CFAttributedStringRef,
        loc: CFIndex,
        attrName: CFStringRef,
        inRange: CFRange,
        longestEffectiveRange: *mut CFRange,
    ) -> CFTypeRef;
}
unsafe extern "C" {
    pub fn CFAttributedStringCreateMutableCopy(
        alloc: CFAllocatorRef,
        maxLength: CFIndex,
        aStr: CFAttributedStringRef,
    ) -> CFMutableAttributedStringRef;
}
unsafe extern "C" {
    pub fn CFAttributedStringCreateMutable(
        alloc: CFAllocatorRef,
        maxLength: CFIndex,
    ) -> CFMutableAttributedStringRef;
}
unsafe extern "C" {
    pub fn CFAttributedStringReplaceString(
        aStr: CFMutableAttributedStringRef,
        range: CFRange,
        replacement: CFStringRef,
    );
}
unsafe extern "C" {
    pub fn CFAttributedStringGetMutableString(
        aStr: CFMutableAttributedStringRef,
    ) -> CFMutableStringRef;
}
unsafe extern "C" {
    pub fn CFAttributedStringSetAttributes(
        aStr: CFMutableAttributedStringRef,
        range: CFRange,
        replacement: CFDictionaryRef,
        clearOtherAttributes: Boolean,
    );
}
unsafe extern "C" {
    pub fn CFAttributedStringSetAttribute(
        aStr: CFMutableAttributedStringRef,
        range: CFRange,
        attrName: CFStringRef,
        value: CFTypeRef,
    );
}
unsafe extern "C" {
    pub fn CFAttributedStringRemoveAttribute(
        aStr: CFMutableAttributedStringRef,
        range: CFRange,
        attrName: CFStringRef,
    );
}
unsafe extern "C" {
    pub fn CFAttributedStringReplaceAttributedString(
        aStr: CFMutableAttributedStringRef,
        range: CFRange,
        replacement: CFAttributedStringRef,
    );
}
unsafe extern "C" {
    pub fn CFAttributedStringBeginEditing(aStr: CFMutableAttributedStringRef);
}
unsafe extern "C" {
    pub fn CFAttributedStringEndEditing(aStr: CFMutableAttributedStringRef);
}
unsafe extern "C" {
    pub fn CFAttributedStringGetBidiLevelsAndResolvedDirections(
        attributedString: CFAttributedStringRef,
        range: CFRange,
        baseDirection: i8,
        bidiLevels: *mut u8,
        baseDirections: *mut u8,
    ) -> bool;
}
unsafe extern "C" {
    pub fn CFAttributedStringGetStatisticalWritingDirections(
        attributedString: CFAttributedStringRef,
        range: CFRange,
        baseDirection: i8,
        bidiLevels: *mut u8,
        baseDirections: *mut u8,
    ) -> bool;
}
unsafe extern "C" {
    pub fn CFURLEnumeratorGetTypeID() -> CFTypeID;
}
unsafe extern "C" {
    pub fn CFURLEnumeratorCreateForDirectoryURL(
        alloc: CFAllocatorRef,
        directoryURL: CFURLRef,
        option: CFURLEnumeratorOptions,
        propertyKeys: CFArrayRef,
    ) -> CFURLEnumeratorRef;
}
unsafe extern "C" {
    pub fn CFURLEnumeratorCreateForMountedVolumes(
        alloc: CFAllocatorRef,
        option: CFURLEnumeratorOptions,
        propertyKeys: CFArrayRef,
    ) -> CFURLEnumeratorRef;
}
unsafe extern "C" {
    pub fn CFURLEnumeratorGetNextURL(
        enumerator: CFURLEnumeratorRef,
        url: *mut CFURLRef,
        error: *mut CFErrorRef,
    ) -> CFURLEnumeratorResult;
}
unsafe extern "C" {
    pub fn CFURLEnumeratorSkipDescendents(enumerator: CFURLEnumeratorRef);
}
unsafe extern "C" {
    pub fn CFURLEnumeratorGetDescendentLevel(enumerator: CFURLEnumeratorRef) -> CFIndex;
}
unsafe extern "C" {
    pub fn CFURLEnumeratorGetSourceDidChange(enumerator: CFURLEnumeratorRef) -> Boolean;
}
unsafe extern "C" {
    pub fn CFFileSecurityGetTypeID() -> CFTypeID;
}
unsafe extern "C" {
    pub fn CFFileSecurityCreate(allocator: CFAllocatorRef) -> CFFileSecurityRef;
}
unsafe extern "C" {
    pub fn CFFileSecurityCreateCopy(
        allocator: CFAllocatorRef,
        fileSec: CFFileSecurityRef,
    ) -> CFFileSecurityRef;
}
unsafe extern "C" {
    pub fn CFFileSecurityCopyOwnerUUID(
        fileSec: CFFileSecurityRef,
        ownerUUID: *mut CFUUIDRef,
    ) -> Boolean;
}
unsafe extern "C" {
    pub fn CFFileSecuritySetOwnerUUID(fileSec: CFFileSecurityRef, ownerUUID: CFUUIDRef) -> Boolean;
}
unsafe extern "C" {
    pub fn CFFileSecurityCopyGroupUUID(
        fileSec: CFFileSecurityRef,
        groupUUID: *mut CFUUIDRef,
    ) -> Boolean;
}
unsafe extern "C" {
    pub fn CFFileSecuritySetGroupUUID(fileSec: CFFileSecurityRef, groupUUID: CFUUIDRef) -> Boolean;
}
unsafe extern "C" {
    pub fn CFFileSecurityCopyAccessControlList(
        fileSec: CFFileSecurityRef,
        accessControlList: *mut acl_t,
    ) -> Boolean;
}
unsafe extern "C" {
    pub fn CFFileSecuritySetAccessControlList(
        fileSec: CFFileSecurityRef,
        accessControlList: acl_t,
    ) -> Boolean;
}
unsafe extern "C" {
    pub fn CFFileSecurityGetOwner(fileSec: CFFileSecurityRef, owner: *mut uid_t) -> Boolean;
}
unsafe extern "C" {
    pub fn CFFileSecuritySetOwner(fileSec: CFFileSecurityRef, owner: uid_t) -> Boolean;
}
unsafe extern "C" {
    pub fn CFFileSecurityGetGroup(fileSec: CFFileSecurityRef, group: *mut gid_t) -> Boolean;
}
unsafe extern "C" {
    pub fn CFFileSecuritySetGroup(fileSec: CFFileSecurityRef, group: gid_t) -> Boolean;
}
unsafe extern "C" {
    pub fn CFFileSecurityGetMode(fileSec: CFFileSecurityRef, mode: *mut mode_t) -> Boolean;
}
unsafe extern "C" {
    pub fn CFFileSecuritySetMode(fileSec: CFFileSecurityRef, mode: mode_t) -> Boolean;
}
unsafe extern "C" {
    pub fn CFFileSecurityClearProperties(
        fileSec: CFFileSecurityRef,
        clearPropertyMask: CFFileSecurityClearOptions,
    ) -> Boolean;
}
unsafe extern "C" {
    pub fn CFStringTokenizerCopyBestStringLanguage(
        string: CFStringRef,
        range: CFRange,
    ) -> CFStringRef;
}
unsafe extern "C" {
    pub fn CFStringTokenizerGetTypeID() -> CFTypeID;
}
unsafe extern "C" {
    pub fn CFStringTokenizerCreate(
        alloc: CFAllocatorRef,
        string: CFStringRef,
        range: CFRange,
        options: CFOptionFlags,
        locale: CFLocaleRef,
    ) -> CFStringTokenizerRef;
}
unsafe extern "C" {
    pub fn CFStringTokenizerSetString(
        tokenizer: CFStringTokenizerRef,
        string: CFStringRef,
        range: CFRange,
    );
}
unsafe extern "C" {
    pub fn CFStringTokenizerGoToTokenAtIndex(
        tokenizer: CFStringTokenizerRef,
        index: CFIndex,
    ) -> CFStringTokenizerTokenType;
}
unsafe extern "C" {
    pub fn CFStringTokenizerAdvanceToNextToken(
        tokenizer: CFStringTokenizerRef,
    ) -> CFStringTokenizerTokenType;
}
unsafe extern "C" {
    pub fn CFStringTokenizerGetCurrentTokenRange(tokenizer: CFStringTokenizerRef) -> CFRange;
}
unsafe extern "C" {
    pub fn CFStringTokenizerCopyCurrentTokenAttribute(
        tokenizer: CFStringTokenizerRef,
        attribute: CFOptionFlags,
    ) -> CFTypeRef;
}
unsafe extern "C" {
    pub fn CFStringTokenizerGetCurrentSubTokens(
        tokenizer: CFStringTokenizerRef,
        ranges: *mut CFRange,
        maxRangeLength: CFIndex,
        derivedSubTokens: CFMutableArrayRef,
    ) -> CFIndex;
}
unsafe extern "C" {
    pub fn CFFileDescriptorGetTypeID() -> CFTypeID;
}
unsafe extern "C" {
    pub fn CFFileDescriptorCreate(
        allocator: CFAllocatorRef,
        fd: CFFileDescriptorNativeDescriptor,
        closeOnInvalidate: Boolean,
        callout: CFFileDescriptorCallBack,
        context: *const CFFileDescriptorContext,
    ) -> CFFileDescriptorRef;
}
unsafe extern "C" {
    pub fn CFFileDescriptorGetNativeDescriptor(
        f: CFFileDescriptorRef,
    ) -> CFFileDescriptorNativeDescriptor;
}
unsafe extern "C" {
    pub fn CFFileDescriptorGetContext(
        f: CFFileDescriptorRef,
        context: *mut CFFileDescriptorContext,
    );
}
unsafe extern "C" {
    pub fn CFFileDescriptorEnableCallBacks(f: CFFileDescriptorRef, callBackTypes: CFOptionFlags);
}
unsafe extern "C" {
    pub fn CFFileDescriptorDisableCallBacks(f: CFFileDescriptorRef, callBackTypes: CFOptionFlags);
}
unsafe extern "C" {
    pub fn CFFileDescriptorInvalidate(f: CFFileDescriptorRef);
}
unsafe extern "C" {
    pub fn CFFileDescriptorIsValid(f: CFFileDescriptorRef) -> Boolean;
}
unsafe extern "C" {
    pub fn CFFileDescriptorCreateRunLoopSource(
        allocator: CFAllocatorRef,
        f: CFFileDescriptorRef,
        order: CFIndex,
    ) -> CFRunLoopSourceRef;
}
unsafe extern "C" {
    pub fn CFUserNotificationGetTypeID() -> CFTypeID;
}
unsafe extern "C" {
    pub fn CFUserNotificationCreate(
        allocator: CFAllocatorRef,
        timeout: CFTimeInterval,
        flags: CFOptionFlags,
        error: *mut SInt32,
        dictionary: CFDictionaryRef,
    ) -> CFUserNotificationRef;
}
unsafe extern "C" {
    pub fn CFUserNotificationReceiveResponse(
        userNotification: CFUserNotificationRef,
        timeout: CFTimeInterval,
        responseFlags: *mut CFOptionFlags,
    ) -> SInt32;
}
unsafe extern "C" {
    pub fn CFUserNotificationGetResponseValue(
        userNotification: CFUserNotificationRef,
        key: CFStringRef,
        idx: CFIndex,
    ) -> CFStringRef;
}
unsafe extern "C" {
    pub fn CFUserNotificationGetResponseDictionary(
        userNotification: CFUserNotificationRef,
    ) -> CFDictionaryRef;
}
unsafe extern "C" {
    pub fn CFUserNotificationUpdate(
        userNotification: CFUserNotificationRef,
        timeout: CFTimeInterval,
        flags: CFOptionFlags,
        dictionary: CFDictionaryRef,
    ) -> SInt32;
}
unsafe extern "C" {
    pub fn CFUserNotificationCancel(userNotification: CFUserNotificationRef) -> SInt32;
}
unsafe extern "C" {
    pub fn CFUserNotificationCreateRunLoopSource(
        allocator: CFAllocatorRef,
        userNotification: CFUserNotificationRef,
        callout: CFUserNotificationCallBack,
        order: CFIndex,
    ) -> CFRunLoopSourceRef;
}
unsafe extern "C" {
    pub fn CFUserNotificationDisplayNotice(
        timeout: CFTimeInterval,
        flags: CFOptionFlags,
        iconURL: CFURLRef,
        soundURL: CFURLRef,
        localizationURL: CFURLRef,
        alertHeader: CFStringRef,
        alertMessage: CFStringRef,
        defaultButtonTitle: CFStringRef,
    ) -> SInt32;
}
unsafe extern "C" {
    pub fn CFUserNotificationDisplayAlert(
        timeout: CFTimeInterval,
        flags: CFOptionFlags,
        iconURL: CFURLRef,
        soundURL: CFURLRef,
        localizationURL: CFURLRef,
        alertHeader: CFStringRef,
        alertMessage: CFStringRef,
        defaultButtonTitle: CFStringRef,
        alternateButtonTitle: CFStringRef,
        otherButtonTitle: CFStringRef,
        responseFlags: *mut CFOptionFlags,
    ) -> SInt32;
}
unsafe extern "C" {
    pub static kCFUserNotificationIconURLKey: CFStringRef;
}
unsafe extern "C" {
    pub static kCFUserNotificationSoundURLKey: CFStringRef;
}
unsafe extern "C" {
    pub static kCFUserNotificationLocalizationURLKey: CFStringRef;
}
unsafe extern "C" {
    pub static kCFUserNotificationAlertHeaderKey: CFStringRef;
}
unsafe extern "C" {
    pub static kCFUserNotificationAlertMessageKey: CFStringRef;
}
unsafe extern "C" {
    pub static kCFUserNotificationDefaultButtonTitleKey: CFStringRef;
}
unsafe extern "C" {
    pub static kCFUserNotificationAlternateButtonTitleKey: CFStringRef;
}
unsafe extern "C" {
    pub static kCFUserNotificationOtherButtonTitleKey: CFStringRef;
}
unsafe extern "C" {
    pub static kCFUserNotificationProgressIndicatorValueKey: CFStringRef;
}
unsafe extern "C" {
    pub static kCFUserNotificationPopUpTitlesKey: CFStringRef;
}
unsafe extern "C" {
    pub static kCFUserNotificationTextFieldTitlesKey: CFStringRef;
}
unsafe extern "C" {
    pub static kCFUserNotificationCheckBoxTitlesKey: CFStringRef;
}
unsafe extern "C" {
    pub static kCFUserNotificationTextFieldValuesKey: CFStringRef;
}
unsafe extern "C" {
    pub static kCFUserNotificationPopUpSelectionKey: CFStringRef;
}
unsafe extern "C" {
    pub static kCFUserNotificationAlertTopMostKey: CFStringRef;
}
unsafe extern "C" {
    pub static kCFUserNotificationKeyboardTypesKey: CFStringRef;
}
unsafe extern "C" {
    pub fn CFXMLNodeGetTypeID() -> CFTypeID;
}
unsafe extern "C" {
    pub fn CFXMLNodeCreate(
        alloc: CFAllocatorRef,
        xmlType: CFXMLNodeTypeCode,
        dataString: CFStringRef,
        additionalInfoPtr: *const ::std::os::raw::c_void,
        version: CFIndex,
    ) -> CFXMLNodeRef;
}
unsafe extern "C" {
    pub fn CFXMLNodeCreateCopy(alloc: CFAllocatorRef, origNode: CFXMLNodeRef) -> CFXMLNodeRef;
}
unsafe extern "C" {
    pub fn CFXMLNodeGetTypeCode(node: CFXMLNodeRef) -> CFXMLNodeTypeCode;
}
unsafe extern "C" {
    pub fn CFXMLNodeGetString(node: CFXMLNodeRef) -> CFStringRef;
}
unsafe extern "C" {
    pub fn CFXMLNodeGetInfoPtr(node: CFXMLNodeRef) -> *const ::std::os::raw::c_void;
}
unsafe extern "C" {
    pub fn CFXMLNodeGetVersion(node: CFXMLNodeRef) -> CFIndex;
}
unsafe extern "C" {
    pub fn CFXMLTreeCreateWithNode(allocator: CFAllocatorRef, node: CFXMLNodeRef) -> CFXMLTreeRef;
}
unsafe extern "C" {
    pub fn CFXMLTreeGetNode(xmlTree: CFXMLTreeRef) -> CFXMLNodeRef;
}
unsafe extern "C" {
    pub fn CFXMLParserGetTypeID() -> CFTypeID;
}
unsafe extern "C" {
    pub fn CFXMLParserCreate(
        allocator: CFAllocatorRef,
        xmlData: CFDataRef,
        dataSource: CFURLRef,
        parseOptions: CFOptionFlags,
        versionOfNodes: CFIndex,
        callBacks: *mut CFXMLParserCallBacks,
        context: *mut CFXMLParserContext,
    ) -> CFXMLParserRef;
}
unsafe extern "C" {
    pub fn CFXMLParserCreateWithDataFromURL(
        allocator: CFAllocatorRef,
        dataSource: CFURLRef,
        parseOptions: CFOptionFlags,
        versionOfNodes: CFIndex,
        callBacks: *mut CFXMLParserCallBacks,
        context: *mut CFXMLParserContext,
    ) -> CFXMLParserRef;
}
unsafe extern "C" {
    pub fn CFXMLParserGetContext(parser: CFXMLParserRef, context: *mut CFXMLParserContext);
}
unsafe extern "C" {
    pub fn CFXMLParserGetCallBacks(parser: CFXMLParserRef, callBacks: *mut CFXMLParserCallBacks);
}
unsafe extern "C" {
    pub fn CFXMLParserGetSourceURL(parser: CFXMLParserRef) -> CFURLRef;
}
unsafe extern "C" {
    pub fn CFXMLParserGetLocation(parser: CFXMLParserRef) -> CFIndex;
}
unsafe extern "C" {
    pub fn CFXMLParserGetLineNumber(parser: CFXMLParserRef) -> CFIndex;
}
unsafe extern "C" {
    pub fn CFXMLParserGetDocument(parser: CFXMLParserRef) -> *mut ::std::os::raw::c_void;
}
unsafe extern "C" {
    pub fn CFXMLParserGetStatusCode(parser: CFXMLParserRef) -> CFXMLParserStatusCode;
}
unsafe extern "C" {
    pub fn CFXMLParserCopyErrorDescription(parser: CFXMLParserRef) -> CFStringRef;
}
unsafe extern "C" {
    pub fn CFXMLParserAbort(
        parser: CFXMLParserRef,
        errorCode: CFXMLParserStatusCode,
        errorDescription: CFStringRef,
    );
}
unsafe extern "C" {
    pub fn CFXMLParserParse(parser: CFXMLParserRef) -> Boolean;
}
unsafe extern "C" {
    pub fn CFXMLTreeCreateFromData(
        allocator: CFAllocatorRef,
        xmlData: CFDataRef,
        dataSource: CFURLRef,
        parseOptions: CFOptionFlags,
        versionOfNodes: CFIndex,
    ) -> CFXMLTreeRef;
}
unsafe extern "C" {
    pub fn CFXMLTreeCreateFromDataWithError(
        allocator: CFAllocatorRef,
        xmlData: CFDataRef,
        dataSource: CFURLRef,
        parseOptions: CFOptionFlags,
        versionOfNodes: CFIndex,
        errorDict: *mut CFDictionaryRef,
    ) -> CFXMLTreeRef;
}
unsafe extern "C" {
    pub fn CFXMLTreeCreateWithDataFromURL(
        allocator: CFAllocatorRef,
        dataSource: CFURLRef,
        parseOptions: CFOptionFlags,
        versionOfNodes: CFIndex,
    ) -> CFXMLTreeRef;
}
unsafe extern "C" {
    pub fn CFXMLTreeCreateXMLData(allocator: CFAllocatorRef, xmlTree: CFXMLTreeRef) -> CFDataRef;
}
unsafe extern "C" {
    pub fn CFXMLCreateStringByEscapingEntities(
        allocator: CFAllocatorRef,
        string: CFStringRef,
        entitiesDictionary: CFDictionaryRef,
    ) -> CFStringRef;
}
unsafe extern "C" {
    pub fn CFXMLCreateStringByUnescapingEntities(
        allocator: CFAllocatorRef,
        string: CFStringRef,
        entitiesDictionary: CFDictionaryRef,
    ) -> CFStringRef;
}
unsafe extern "C" {
    pub static kCFXMLTreeErrorDescription: CFStringRef;
}
unsafe extern "C" {
    pub static kCFXMLTreeErrorLineNumber: CFStringRef;
}
unsafe extern "C" {
    pub static kCFXMLTreeErrorLocation: CFStringRef;
}
unsafe extern "C" {
    pub static kCFXMLTreeErrorStatusCode: CFStringRef;
}

unsafe impl objc2::encode::RefEncode for _malloc_zone_t {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for _malloc_zone_t {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("_malloc_zone_t", &[]);
}
unsafe impl objc2::encode::RefEncode for vm_range_t {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for vm_range_t {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("vm_range_t", &[]);
}
unsafe impl objc2::encode::RefEncode for malloc_statistics_t {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for malloc_statistics_t {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("malloc_statistics_t", &[]);
}
unsafe impl objc2::encode::RefEncode for malloc_introspection_t {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for malloc_introspection_t {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("malloc_introspection_t", &[]);
}
unsafe impl objc2::encode::RefEncode for _acl {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for _acl {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("_acl", &[]);
}
unsafe impl objc2::encode::RefEncode for OS_object {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for OS_object {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for OS_os_workgroup {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for OS_os_workgroup {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for OS_os_workgroup_interval {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for OS_os_workgroup_interval {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for OS_os_workgroup_parallel {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for OS_os_workgroup_parallel {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for __CFString {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for __CFString {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("__CFString", &[]);
}
unsafe impl objc2::encode::RefEncode for CFRange {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CFRange {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("CFRange", &[]);
}
unsafe impl objc2::encode::RefEncode for __CFNull {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for __CFNull {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("__CFNull", &[]);
}
unsafe impl objc2::encode::RefEncode for __CFAllocator {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for __CFAllocator {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("__CFAllocator", &[]);
}
unsafe impl objc2::encode::RefEncode for CFAllocatorContext {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CFAllocatorContext {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("CFAllocatorContext", &[]);
}
unsafe impl objc2::encode::RefEncode for CFArrayCallBacks {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CFArrayCallBacks {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("CFArrayCallBacks", &[]);
}
unsafe impl objc2::encode::RefEncode for __CFArray {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for __CFArray {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("__CFArray", &[]);
}
unsafe impl objc2::encode::RefEncode for CFBagCallBacks {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CFBagCallBacks {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("CFBagCallBacks", &[]);
}
unsafe impl objc2::encode::RefEncode for __CFBag {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for __CFBag {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("__CFBag", &[]);
}
unsafe impl objc2::encode::RefEncode for CFBinaryHeapCompareContext {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CFBinaryHeapCompareContext {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("CFBinaryHeapCompareContext", &[]);
}
unsafe impl objc2::encode::RefEncode for CFBinaryHeapCallBacks {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CFBinaryHeapCallBacks {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("CFBinaryHeapCallBacks", &[]);
}
unsafe impl objc2::encode::RefEncode for __CFBinaryHeap {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for __CFBinaryHeap {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("__CFBinaryHeap", &[]);
}
unsafe impl objc2::encode::RefEncode for __CFBitVector {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for __CFBitVector {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("__CFBitVector", &[]);
}
unsafe impl objc2::encode::RefEncode for CFDictionaryKeyCallBacks {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CFDictionaryKeyCallBacks {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("CFDictionaryKeyCallBacks", &[]);
}
unsafe impl objc2::encode::RefEncode for CFDictionaryValueCallBacks {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CFDictionaryValueCallBacks {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("CFDictionaryValueCallBacks", &[]);
}
unsafe impl objc2::encode::RefEncode for __CFDictionary {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for __CFDictionary {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("__CFDictionary", &[]);
}
unsafe impl objc2::encode::RefEncode for __CFNotificationCenter {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for __CFNotificationCenter {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("__CFNotificationCenter", &[]);
}
unsafe impl objc2::encode::RefEncode for __CFLocale {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for __CFLocale {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("__CFLocale", &[]);
}
unsafe impl objc2::encode::RefEncode for __CFDate {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for __CFDate {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("__CFDate", &[]);
}
unsafe impl objc2::encode::RefEncode for __CFTimeZone {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for __CFTimeZone {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("__CFTimeZone", &[]);
}
unsafe impl objc2::encode::RefEncode for CFGregorianDate {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CFGregorianDate {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("CFGregorianDate", &[]);
}
unsafe impl objc2::encode::RefEncode for CFGregorianUnits {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CFGregorianUnits {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("CFGregorianUnits", &[]);
}
unsafe impl objc2::encode::RefEncode for __CFData {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for __CFData {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("__CFData", &[]);
}
unsafe impl objc2::encode::RefEncode for __CFCharacterSet {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for __CFCharacterSet {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("__CFCharacterSet", &[]);
}
unsafe impl objc2::encode::RefEncode for __CFError {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for __CFError {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("__CFError", &[]);
}
unsafe impl objc2::encode::RefEncode for CFStringInlineBuffer {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CFStringInlineBuffer {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("CFStringInlineBuffer", &[]);
}
unsafe impl objc2::encode::RefEncode for __CFCalendar {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for __CFCalendar {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("__CFCalendar", &[]);
}
unsafe impl objc2::encode::RefEncode for CGPoint {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CGPoint {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("CGPoint", &[]);
}
unsafe impl objc2::encode::RefEncode for CGSize {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CGSize {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("CGSize", &[]);
}
unsafe impl objc2::encode::RefEncode for CGVector {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CGVector {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("CGVector", &[]);
}
unsafe impl objc2::encode::RefEncode for CGRect {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CGRect {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("CGRect", &[]);
}
unsafe impl objc2::encode::RefEncode for CGAffineTransform {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CGAffineTransform {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("CGAffineTransform", &[]);
}
unsafe impl objc2::encode::RefEncode for CGAffineTransformComponents {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CGAffineTransformComponents {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("CGAffineTransformComponents", &[]);
}
unsafe impl objc2::encode::RefEncode for __CFDateFormatter {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for __CFDateFormatter {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("__CFDateFormatter", &[]);
}
unsafe impl objc2::encode::RefEncode for __CFBoolean {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for __CFBoolean {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("__CFBoolean", &[]);
}
unsafe impl objc2::encode::RefEncode for __CFNumber {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for __CFNumber {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("__CFNumber", &[]);
}
unsafe impl objc2::encode::RefEncode for __CFNumberFormatter {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for __CFNumberFormatter {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("__CFNumberFormatter", &[]);
}
unsafe impl objc2::encode::RefEncode for __CFURL {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for __CFURL {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("__CFURL", &[]);
}
unsafe impl objc2::encode::RefEncode for __CFRunLoop {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for __CFRunLoop {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("__CFRunLoop", &[]);
}
unsafe impl objc2::encode::RefEncode for __CFRunLoopSource {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for __CFRunLoopSource {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("__CFRunLoopSource", &[]);
}
unsafe impl objc2::encode::RefEncode for __CFRunLoopObserver {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for __CFRunLoopObserver {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("__CFRunLoopObserver", &[]);
}
unsafe impl objc2::encode::RefEncode for __CFRunLoopTimer {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for __CFRunLoopTimer {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("__CFRunLoopTimer", &[]);
}
unsafe impl objc2::encode::RefEncode for CFRunLoopSourceContext {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CFRunLoopSourceContext {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("CFRunLoopSourceContext", &[]);
}
unsafe impl objc2::encode::RefEncode for CFRunLoopSourceContext1 {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CFRunLoopSourceContext1 {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("CFRunLoopSourceContext1", &[]);
}
unsafe impl objc2::encode::RefEncode for CFRunLoopObserverContext {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CFRunLoopObserverContext {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("CFRunLoopObserverContext", &[]);
}
unsafe impl objc2::encode::RefEncode for CFRunLoopTimerContext {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CFRunLoopTimerContext {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("CFRunLoopTimerContext", &[]);
}
unsafe impl objc2::encode::RefEncode for __CFSocket {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for __CFSocket {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("__CFSocket", &[]);
}
unsafe impl objc2::encode::RefEncode for CFSocketSignature {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CFSocketSignature {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("CFSocketSignature", &[]);
}
unsafe impl objc2::encode::RefEncode for CFSocketContext {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CFSocketContext {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("CFSocketContext", &[]);
}
unsafe impl objc2::encode::RefEncode for CFStreamError {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CFStreamError {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("CFStreamError", &[]);
}
unsafe impl objc2::encode::RefEncode for CFStreamClientContext {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CFStreamClientContext {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("CFStreamClientContext", &[]);
}
unsafe impl objc2::encode::RefEncode for __CFReadStream {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for __CFReadStream {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("__CFReadStream", &[]);
}
unsafe impl objc2::encode::RefEncode for __CFWriteStream {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for __CFWriteStream {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("__CFWriteStream", &[]);
}
unsafe impl objc2::encode::RefEncode for CFSetCallBacks {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CFSetCallBacks {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("CFSetCallBacks", &[]);
}
unsafe impl objc2::encode::RefEncode for __CFSet {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for __CFSet {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("__CFSet", &[]);
}
unsafe impl objc2::encode::RefEncode for CFTreeContext {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CFTreeContext {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("CFTreeContext", &[]);
}
unsafe impl objc2::encode::RefEncode for __CFTree {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for __CFTree {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("__CFTree", &[]);
}
unsafe impl objc2::encode::RefEncode for __CFUUID {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for __CFUUID {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("__CFUUID", &[]);
}
unsafe impl objc2::encode::RefEncode for CFUUIDBytes {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CFUUIDBytes {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("CFUUIDBytes", &[]);
}
unsafe impl objc2::encode::RefEncode for __CFBundle {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for __CFBundle {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("__CFBundle", &[]);
}
unsafe impl objc2::encode::RefEncode for __CFMessagePort {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for __CFMessagePort {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("__CFMessagePort", &[]);
}
unsafe impl objc2::encode::RefEncode for CFMessagePortContext {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CFMessagePortContext {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("CFMessagePortContext", &[]);
}
unsafe impl objc2::encode::RefEncode for __CFPlugInInstance {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for __CFPlugInInstance {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("__CFPlugInInstance", &[]);
}
unsafe impl objc2::encode::RefEncode for __CFMachPort {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for __CFMachPort {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("__CFMachPort", &[]);
}
unsafe impl objc2::encode::RefEncode for CFMachPortContext {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CFMachPortContext {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("CFMachPortContext", &[]);
}
unsafe impl objc2::encode::RefEncode for __CFAttributedString {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for __CFAttributedString {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("__CFAttributedString", &[]);
}
unsafe impl objc2::encode::RefEncode for __CFURLEnumerator {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for __CFURLEnumerator {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("__CFURLEnumerator", &[]);
}
unsafe impl objc2::encode::RefEncode for __CFFileSecurity {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for __CFFileSecurity {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("__CFFileSecurity", &[]);
}
unsafe impl objc2::encode::RefEncode for __CFStringTokenizer {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for __CFStringTokenizer {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("__CFStringTokenizer", &[]);
}
unsafe impl objc2::encode::RefEncode for __CFFileDescriptor {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for __CFFileDescriptor {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("__CFFileDescriptor", &[]);
}
unsafe impl objc2::encode::RefEncode for CFFileDescriptorContext {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CFFileDescriptorContext {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("CFFileDescriptorContext", &[]);
}
unsafe impl objc2::encode::RefEncode for __CFUserNotification {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for __CFUserNotification {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("__CFUserNotification", &[]);
}
unsafe impl objc2::encode::RefEncode for __CFXMLNode {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for __CFXMLNode {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("__CFXMLNode", &[]);
}
unsafe impl objc2::encode::RefEncode for CFXMLElementInfo {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CFXMLElementInfo {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("CFXMLElementInfo", &[]);
}
unsafe impl objc2::encode::RefEncode for CFXMLProcessingInstructionInfo {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CFXMLProcessingInstructionInfo {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("CFXMLProcessingInstructionInfo", &[]);
}
unsafe impl objc2::encode::RefEncode for CFXMLDocumentInfo {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CFXMLDocumentInfo {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("CFXMLDocumentInfo", &[]);
}
unsafe impl objc2::encode::RefEncode for CFXMLExternalID {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CFXMLExternalID {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("CFXMLExternalID", &[]);
}
unsafe impl objc2::encode::RefEncode for CFXMLDocumentTypeInfo {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CFXMLDocumentTypeInfo {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("CFXMLDocumentTypeInfo", &[]);
}
unsafe impl objc2::encode::RefEncode for CFXMLNotationInfo {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CFXMLNotationInfo {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("CFXMLNotationInfo", &[]);
}
unsafe impl objc2::encode::RefEncode for CFXMLElementTypeDeclarationInfo {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CFXMLElementTypeDeclarationInfo {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("CFXMLElementTypeDeclarationInfo", &[]);
}
unsafe impl objc2::encode::RefEncode for CFXMLAttributeDeclarationInfo {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CFXMLAttributeDeclarationInfo {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("CFXMLAttributeDeclarationInfo", &[]);
}
unsafe impl objc2::encode::RefEncode for CFXMLAttributeListDeclarationInfo {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CFXMLAttributeListDeclarationInfo {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("CFXMLAttributeListDeclarationInfo", &[]);
}
unsafe impl objc2::encode::RefEncode for CFXMLEntityInfo {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CFXMLEntityInfo {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("CFXMLEntityInfo", &[]);
}
unsafe impl objc2::encode::RefEncode for CFXMLEntityReferenceInfo {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CFXMLEntityReferenceInfo {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("CFXMLEntityReferenceInfo", &[]);
}
unsafe impl objc2::encode::RefEncode for __CFXMLParser {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for __CFXMLParser {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("__CFXMLParser", &[]);
}
unsafe impl objc2::encode::RefEncode for CFXMLParserCallBacks {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CFXMLParserCallBacks {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("CFXMLParserCallBacks", &[]);
}
unsafe impl objc2::encode::RefEncode for CFXMLParserContext {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CFXMLParserContext {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("CFXMLParserContext", &[]);
}
unsafe impl objc2::encode::RefEncode for IUnknownVTbl {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for IUnknownVTbl {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("IUnknownVTbl", &[]);
}
