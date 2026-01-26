#[allow(unused_imports)]
use crate::objc::*;
#[allow(unused_imports)]
use crate::CoreFoundation::*;
#[allow(unused_imports)]
use crate::CoreGraphics::*;
#[allow(unused_imports)]
use crate::CoreServices::*;
#[allow(unused_imports)]
use libc::{id_t, mach_port_t, size_t, time_t};

#[repr(C)]
#[derive(Copy, Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct __BindgenBitfieldUnit<Storage> {
    storage: Storage,
}
impl<Storage> __BindgenBitfieldUnit<Storage> {
    #[inline]
    pub const fn new(storage: Storage) -> Self {
        Self { storage }
    }
}
impl<Storage> __BindgenBitfieldUnit<Storage>
where
    Storage: AsRef<[u8]> + AsMut<[u8]>,
{
    #[inline]
    fn extract_bit(byte: u8, index: usize) -> bool {
        let bit_index = if cfg!(target_endian = "big") {
            7 - (index % 8)
        } else {
            index % 8
        };
        let mask = 1 << bit_index;
        byte & mask == mask
    }
    #[inline]
    pub fn get_bit(&self, index: usize) -> bool {
        debug_assert!(index / 8 < self.storage.as_ref().len());
        let byte_index = index / 8;
        let byte = self.storage.as_ref()[byte_index];
        Self::extract_bit(byte, index)
    }
    #[inline]
    pub unsafe fn raw_get_bit(this: *const Self, index: usize) -> bool {
        debug_assert!(index / 8 < core::mem::size_of::<Storage>());
        let byte_index = index / 8;
        let byte = unsafe {
            *(core::ptr::addr_of!((*this).storage) as *const u8).offset(byte_index as isize)
        };
        Self::extract_bit(byte, index)
    }
    #[inline]
    fn change_bit(byte: u8, index: usize, val: bool) -> u8 {
        let bit_index = if cfg!(target_endian = "big") {
            7 - (index % 8)
        } else {
            index % 8
        };
        let mask = 1 << bit_index;
        if val {
            byte | mask
        } else {
            byte & !mask
        }
    }
    #[inline]
    pub fn set_bit(&mut self, index: usize, val: bool) {
        debug_assert!(index / 8 < self.storage.as_ref().len());
        let byte_index = index / 8;
        let byte = &mut self.storage.as_mut()[byte_index];
        *byte = Self::change_bit(*byte, index, val);
    }
    #[inline]
    pub unsafe fn raw_set_bit(this: *mut Self, index: usize, val: bool) {
        debug_assert!(index / 8 < core::mem::size_of::<Storage>());
        let byte_index = index / 8;
        let byte = unsafe {
            (core::ptr::addr_of_mut!((*this).storage) as *mut u8).offset(byte_index as isize)
        };
        unsafe { *byte = Self::change_bit(*byte, index, val) };
    }
    #[inline]
    pub fn get(&self, bit_offset: usize, bit_width: u8) -> u64 {
        debug_assert!(bit_width <= 64);
        debug_assert!(bit_offset / 8 < self.storage.as_ref().len());
        debug_assert!((bit_offset + (bit_width as usize)) / 8 <= self.storage.as_ref().len(),);
        let mut val = 0;
        for i in 0..(bit_width as usize) {
            if self.get_bit(i + bit_offset) {
                let index = if cfg!(target_endian = "big") {
                    bit_width as usize - 1 - i
                } else {
                    i
                };
                val |= 1 << index;
            }
        }
        val
    }
    #[inline]
    pub unsafe fn raw_get(this: *const Self, bit_offset: usize, bit_width: u8) -> u64 {
        debug_assert!(bit_width <= 64);
        debug_assert!(bit_offset / 8 < core::mem::size_of::<Storage>());
        debug_assert!((bit_offset + (bit_width as usize)) / 8 <= core::mem::size_of::<Storage>(),);
        let mut val = 0;
        for i in 0..(bit_width as usize) {
            if unsafe { Self::raw_get_bit(this, i + bit_offset) } {
                let index = if cfg!(target_endian = "big") {
                    bit_width as usize - 1 - i
                } else {
                    i
                };
                val |= 1 << index;
            }
        }
        val
    }
    #[inline]
    pub fn set(&mut self, bit_offset: usize, bit_width: u8, val: u64) {
        debug_assert!(bit_width <= 64);
        debug_assert!(bit_offset / 8 < self.storage.as_ref().len());
        debug_assert!((bit_offset + (bit_width as usize)) / 8 <= self.storage.as_ref().len(),);
        for i in 0..(bit_width as usize) {
            let mask = 1 << i;
            let val_bit_is_set = val & mask == mask;
            let index = if cfg!(target_endian = "big") {
                bit_width as usize - 1 - i
            } else {
                i
            };
            self.set_bit(index + bit_offset, val_bit_is_set);
        }
    }
    #[inline]
    pub unsafe fn raw_set(this: *mut Self, bit_offset: usize, bit_width: u8, val: u64) {
        debug_assert!(bit_width <= 64);
        debug_assert!(bit_offset / 8 < core::mem::size_of::<Storage>());
        debug_assert!((bit_offset + (bit_width as usize)) / 8 <= core::mem::size_of::<Storage>(),);
        for i in 0..(bit_width as usize) {
            let mask = 1 << i;
            let val_bit_is_set = val & mask == mask;
            let index = if cfg!(target_endian = "big") {
                bit_width as usize - 1 - i
            } else {
                i
            };
            unsafe { Self::raw_set_bit(this, index + bit_offset, val_bit_is_set) };
        }
    }
}
#[allow(unused_imports)]
use objc2::msg_send;
#[repr(C)]
#[derive(Default)]
pub struct __IncompleteArrayField<T>(::std::marker::PhantomData<T>, [T; 0]);
impl<T> __IncompleteArrayField<T> {
    #[inline]
    pub const fn new() -> Self {
        __IncompleteArrayField(::std::marker::PhantomData, [])
    }
    #[inline]
    pub fn as_ptr(&self) -> *const T {
        self as *const _ as *const T
    }
    #[inline]
    pub fn as_mut_ptr(&mut self) -> *mut T {
        self as *mut _ as *mut T
    }
    #[inline]
    pub unsafe fn as_slice(&self, len: usize) -> &[T] {
        ::std::slice::from_raw_parts(self.as_ptr(), len)
    }
    #[inline]
    pub unsafe fn as_mut_slice(&mut self, len: usize) -> &mut [T] {
        ::std::slice::from_raw_parts_mut(self.as_mut_ptr(), len)
    }
}
impl<T> ::std::fmt::Debug for __IncompleteArrayField<T> {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.write_str("__IncompleteArrayField")
    }
}
pub type natural_t = __darwin_natural_t;
pub type mach_vm_address_t = u64;
pub type mach_vm_size_t = u64;
pub type mach_port_name_t = natural_t;
pub type mach_msg_bits_t = ::std::os::raw::c_uint;
pub type mach_msg_size_t = natural_t;
pub type mach_msg_id_t = integer_t;
pub type mach_msg_type_name_t = ::std::os::raw::c_uint;
pub type mach_msg_descriptor_type_t = ::std::os::raw::c_uint;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct mach_msg_port_descriptor_t {
    pub name: mach_port_t,
    pub pad1: mach_msg_size_t,
    pub _bitfield_align_1: [u16; 0],
    pub _bitfield_1: __BindgenBitfieldUnit<[u8; 4usize]>,
}
impl mach_msg_port_descriptor_t {
    #[inline]
    pub fn pad2(&self) -> ::std::os::raw::c_uint {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(0usize, 16u8) as u32) }
    }
    #[inline]
    pub fn set_pad2(&mut self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(0usize, 16u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn pad2_raw(this: *const Self) -> ::std::os::raw::c_uint {
        unsafe {
            ::std::mem::transmute(<__BindgenBitfieldUnit<[u8; 4usize]>>::raw_get(
                ::std::ptr::addr_of!((*this)._bitfield_1),
                0usize,
                16u8,
            ) as u32)
        }
    }
    #[inline]
    pub unsafe fn set_pad2_raw(this: *mut Self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<[u8; 4usize]>>::raw_set(
                ::std::ptr::addr_of_mut!((*this)._bitfield_1),
                0usize,
                16u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn disposition(&self) -> mach_msg_type_name_t {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(16usize, 8u8) as u32) }
    }
    #[inline]
    pub fn set_disposition(&mut self, val: mach_msg_type_name_t) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(16usize, 8u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn disposition_raw(this: *const Self) -> mach_msg_type_name_t {
        unsafe {
            ::std::mem::transmute(<__BindgenBitfieldUnit<[u8; 4usize]>>::raw_get(
                ::std::ptr::addr_of!((*this)._bitfield_1),
                16usize,
                8u8,
            ) as u32)
        }
    }
    #[inline]
    pub unsafe fn set_disposition_raw(this: *mut Self, val: mach_msg_type_name_t) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<[u8; 4usize]>>::raw_set(
                ::std::ptr::addr_of_mut!((*this)._bitfield_1),
                16usize,
                8u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn type_(&self) -> mach_msg_descriptor_type_t {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(24usize, 8u8) as u32) }
    }
    #[inline]
    pub fn set_type(&mut self, val: mach_msg_descriptor_type_t) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(24usize, 8u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn type__raw(this: *const Self) -> mach_msg_descriptor_type_t {
        unsafe {
            ::std::mem::transmute(<__BindgenBitfieldUnit<[u8; 4usize]>>::raw_get(
                ::std::ptr::addr_of!((*this)._bitfield_1),
                24usize,
                8u8,
            ) as u32)
        }
    }
    #[inline]
    pub unsafe fn set_type_raw(this: *mut Self, val: mach_msg_descriptor_type_t) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<[u8; 4usize]>>::raw_set(
                ::std::ptr::addr_of_mut!((*this)._bitfield_1),
                24usize,
                8u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn new_bitfield_1(
        pad2: ::std::os::raw::c_uint,
        disposition: mach_msg_type_name_t,
        type_: mach_msg_descriptor_type_t,
    ) -> __BindgenBitfieldUnit<[u8; 4usize]> {
        let mut __bindgen_bitfield_unit: __BindgenBitfieldUnit<[u8; 4usize]> = Default::default();
        __bindgen_bitfield_unit.set(0usize, 16u8, {
            let pad2: u32 = unsafe { ::std::mem::transmute(pad2) };
            pad2 as u64
        });
        __bindgen_bitfield_unit.set(16usize, 8u8, {
            let disposition: u32 = unsafe { ::std::mem::transmute(disposition) };
            disposition as u64
        });
        __bindgen_bitfield_unit.set(24usize, 8u8, {
            let type_: u32 = unsafe { ::std::mem::transmute(type_) };
            type_ as u64
        });
        __bindgen_bitfield_unit
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct mach_msg_body_t {
    pub msgh_descriptor_count: mach_msg_size_t,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct mach_msg_header_t {
    pub msgh_bits: mach_msg_bits_t,
    pub msgh_size: mach_msg_size_t,
    pub msgh_remote_port: mach_port_t,
    pub msgh_local_port: mach_port_t,
    pub msgh_voucher_port: mach_port_name_t,
    pub msgh_id: mach_msg_id_t,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct audit_token_t {
    pub val: [::std::os::raw::c_uint; 8usize],
}
pub type clock_res_t = ::std::os::raw::c_int;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct mach_timespec {
    pub tv_sec: ::std::os::raw::c_uint,
    pub tv_nsec: clock_res_t,
}
pub type mach_timespec_t = mach_timespec;
pub type task_port_t = task_t;
pub type UInt64 = ::std::os::raw::c_ulonglong;
pub type io_object_t = mach_port_t;
pub type OSReturn = kern_return_t;
pub type dispatch_block_t = *mut ::std::os::raw::c_void;
pub type IOReturn = kern_return_t;
pub type IOOptionBits = UInt32;
pub type IOFixed = SInt32;
pub type IOVersion = UInt32;
pub type IOItemCount = UInt32;
pub type IOCacheMode = UInt32;
pub type IOByteCount32 = UInt32;
pub type IOByteCount64 = UInt64;
pub type IOPhysicalAddress32 = UInt32;
pub type IOPhysicalAddress64 = UInt64;
pub type IOPhysicalLength32 = UInt32;
pub type IOPhysicalLength64 = UInt64;
pub type IOVirtualAddress = mach_vm_address_t;
pub type IOByteCount = IOByteCount64;
pub type IOLogicalAddress = IOVirtualAddress;
pub type IOPhysicalAddress = IOPhysicalAddress64;
pub type IOPhysicalLength = IOPhysicalLength64;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct IOPhysicalRange {
    pub address: IOPhysicalAddress,
    pub length: IOByteCount,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct IOVirtualRange {
    pub address: IOVirtualAddress,
    pub length: IOByteCount,
}
pub type IOAddressRange = IOVirtualRange;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct IONamedValue {
    pub value: ::std::os::raw::c_int,
    pub name: *const ::std::os::raw::c_char,
}
pub type IOAlignment = ::std::os::raw::c_uint;
pub type io_connect_t = io_object_t;
pub type io_enumerator_t = io_object_t;
pub type io_ident_t = io_object_t;
pub type io_iterator_t = io_object_t;
pub type io_registry_entry_t = io_object_t;
pub type io_service_t = io_object_t;
pub type uext_object_t = io_object_t;
pub type IODeviceNumber = ::std::os::raw::c_uint;
pub type OSAsyncReference = [natural_t; 8usize];
#[repr(C)]
#[derive(Debug)]
pub struct OSNotificationHeader {
    pub size: mach_msg_size_t,
    pub type_: natural_t,
    pub reference: OSAsyncReference,
    pub content: __IncompleteArrayField<::std::os::raw::c_uchar>,
}
#[repr(C, packed(4))]
#[derive(Debug, Copy, Clone)]
pub struct IOServiceInterestContent {
    pub messageType: natural_t,
    pub messageArgument: [*mut ::std::os::raw::c_void; 1usize],
}
#[repr(C, packed(4))]
pub struct IOAsyncCompletionContent {
    pub result: IOReturn,
    pub args: __IncompleteArrayField<*mut ::std::os::raw::c_void>,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct IONotificationPort {
    _unused: [u8; 0],
}
pub type IONotificationPortRef = *mut IONotificationPort;
pub type IOServiceMatchingCallback = ::std::option::Option<
    unsafe extern "C" fn(refcon: *mut ::std::os::raw::c_void, iterator: io_iterator_t),
>;
pub type IOServiceInterestCallback = ::std::option::Option<
    unsafe extern "C" fn(
        refcon: *mut ::std::os::raw::c_void,
        service: io_service_t,
        messageType: u32,
        messageArgument: *mut ::std::os::raw::c_void,
    ),
>;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _IODataQueueEntry {
    pub size: UInt32,
    pub data: [UInt8; 4usize],
}
pub type IODataQueueEntry = _IODataQueueEntry;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _IODataQueueMemory {
    pub queueSize: UInt32,
    pub head: UInt32,
    pub tail: UInt32,
    pub queue: [IODataQueueEntry; 1usize],
}
pub type IODataQueueMemory = _IODataQueueMemory;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _IODataQueueAppendix {
    pub version: UInt32,
    pub msgh: mach_msg_header_t,
}
pub type IODataQueueAppendix = _IODataQueueAppendix;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct IOCFPlugInInterfaceStruct {
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
    pub version: UInt16,
    pub revision: UInt16,
    pub Probe: ::std::option::Option<
        unsafe extern "C" fn(
            thisPointer: *mut ::std::os::raw::c_void,
            propertyTable: CFDictionaryRef,
            service: io_service_t,
            order: *mut SInt32,
        ) -> IOReturn,
    >,
    pub Start: ::std::option::Option<
        unsafe extern "C" fn(
            thisPointer: *mut ::std::os::raw::c_void,
            propertyTable: CFDictionaryRef,
            service: io_service_t,
        ) -> IOReturn,
    >,
    pub Stop: ::std::option::Option<
        unsafe extern "C" fn(thisPointer: *mut ::std::os::raw::c_void) -> IOReturn,
    >,
}
pub type IOCFPlugInInterface = IOCFPlugInInterfaceStruct;
pub type IOURLError = ::std::os::raw::c_int;
pub type IOMessage = UInt32;
pub type OSObjectRef = u64;
#[repr(C)]
#[derive(Debug)]
pub struct IORPCMessageMach {
    pub msgh: mach_msg_header_t,
    pub msgh_body: mach_msg_body_t,
    pub objects: __IncompleteArrayField<mach_msg_port_descriptor_t>,
}
#[repr(C, packed(4))]
pub struct IORPCMessage {
    pub msgid: u64,
    pub flags: u64,
    pub objectRefs: u64,
    pub objects: __IncompleteArrayField<OSObjectRef>,
}
#[repr(C)]
#[derive(Debug)]
pub struct IORPCMessageErrorReturnContent {
    pub hdr: IORPCMessage,
    pub result: kern_return_t,
    pub pad: u32,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct IORPC {
    pub message: *mut IORPCMessageMach,
    pub reply: *mut IORPCMessageMach,
    pub sendSize: u32,
    pub replySize: u32,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct IOGPoint {
    pub x: SInt16,
    pub y: SInt16,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct IOGBounds {
    pub minx: SInt16,
    pub maxx: SInt16,
    pub miny: SInt16,
    pub maxy: SInt16,
}
pub type NXMouseButton = ::std::os::raw::c_uint;
pub type NXEventSystemInfoType = *mut ::std::os::raw::c_int;
pub type NXCoord = f32;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _NXSize {
    pub width: NXCoord,
    pub height: NXCoord,
}
pub type NXSize = _NXSize;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _NXTabletPointData {
    pub __bindgen_anon_1: _NXTabletPointData__bindgen_ty_1,
    pub x: SInt32,
    pub y: SInt32,
    pub z: SInt32,
    pub buttons: UInt16,
    pub pressure: UInt16,
    pub tilt: _NXTabletPointData__bindgen_ty_1,
    pub rotation: UInt16,
    pub tangentialPressure: SInt16,
    pub deviceID: UInt16,
    pub vendor1: SInt16,
    pub vendor2: SInt16,
    pub vendor3: SInt16,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _NXTabletPointData__bindgen_ty_1 {
    pub x: SInt16,
    pub y: SInt16,
}
pub type NXTabletPointData = _NXTabletPointData;
#[repr(C, packed(4))]
#[derive(Debug, Copy, Clone)]
pub struct _NXTabletProximityData {
    pub vendorID: UInt16,
    pub tabletID: UInt16,
    pub pointerID: UInt16,
    pub deviceID: UInt16,
    pub systemTabletID: UInt16,
    pub vendorPointerType: UInt16,
    pub pointerSerialNumber: UInt32,
    pub uniqueID: UInt64,
    pub capabilityMask: UInt32,
    pub pointerType: UInt8,
    pub enterProximity: UInt8,
    pub reserved1: SInt16,
}
pub type NXTabletProximityData = _NXTabletProximityData;
#[repr(C)]
#[derive(Copy, Clone)]
pub union NXEventData {
    pub __bindgen_anon_1: NXEventData__bindgen_ty_1,
    pub __bindgen_anon_2: NXEventData__bindgen_ty_2,
    pub __bindgen_anon_3: NXEventData__bindgen_ty_3,
    pub __bindgen_anon_4: NXEventData__bindgen_ty_4,
    pub __bindgen_anon_5: NXEventData__bindgen_ty_5,
    pub __bindgen_anon_6: NXEventData__bindgen_ty_6,
    pub __bindgen_anon_7: NXEventData__bindgen_ty_7,
    pub __bindgen_anon_8: NXEventData__bindgen_ty_8,
    pub mouse: NXEventData__bindgen_ty_1,
    pub mouseMove: NXEventData__bindgen_ty_2,
    pub key: NXEventData__bindgen_ty_3,
    pub tracking: NXEventData__bindgen_ty_4,
    pub scrollWheel: NXEventData__bindgen_ty_5,
    pub zoom: NXEventData__bindgen_ty_5,
    pub compound: NXEventData__bindgen_ty_6,
    pub tablet: NXEventData__bindgen_ty_7,
    pub proximity: NXEventData__bindgen_ty_8,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct NXEventData__bindgen_ty_1 {
    pub __bindgen_anon_1: NXEventData__bindgen_ty_1__bindgen_ty_1,
    pub subx: UInt8,
    pub suby: UInt8,
    pub eventNum: SInt16,
    pub click: SInt32,
    pub pressure: UInt8,
    pub buttonNumber: UInt8,
    pub subType: UInt8,
    pub reserved2: UInt8,
    pub reserved3: SInt32,
    pub tablet: NXEventData__bindgen_ty_1__bindgen_ty_1,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union NXEventData__bindgen_ty_1__bindgen_ty_1 {
    pub point: NXTabletPointData,
    pub proximity: NXTabletProximityData,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct NXEventData__bindgen_ty_2 {
    pub __bindgen_anon_1: NXEventData__bindgen_ty_2__bindgen_ty_1,
    pub dx: SInt32,
    pub dy: SInt32,
    pub subx: UInt8,
    pub suby: UInt8,
    pub subType: UInt8,
    pub reserved1: UInt8,
    pub reserved2: SInt32,
    pub tablet: NXEventData__bindgen_ty_2__bindgen_ty_1,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union NXEventData__bindgen_ty_2__bindgen_ty_1 {
    pub point: NXTabletPointData,
    pub proximity: NXTabletProximityData,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct NXEventData__bindgen_ty_3 {
    pub origCharSet: UInt16,
    pub repeat: SInt16,
    pub charSet: UInt16,
    pub charCode: UInt16,
    pub keyCode: UInt16,
    pub origCharCode: UInt16,
    pub reserved1: SInt32,
    pub keyboardType: UInt32,
    pub reserved2: SInt32,
    pub reserved3: SInt32,
    pub reserved4: SInt32,
    pub reserved5: [SInt32; 4usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct NXEventData__bindgen_ty_4 {
    pub reserved: SInt16,
    pub eventNum: SInt16,
    pub trackingNum: SInt32,
    pub userData: SInt32,
    pub reserved1: SInt32,
    pub reserved2: SInt32,
    pub reserved3: SInt32,
    pub reserved4: SInt32,
    pub reserved5: SInt32,
    pub reserved6: [SInt32; 4usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct NXEventData__bindgen_ty_5 {
    pub deltaAxis1: SInt16,
    pub deltaAxis2: SInt16,
    pub deltaAxis3: SInt16,
    pub reserved1: SInt16,
    pub fixedDeltaAxis1: SInt32,
    pub fixedDeltaAxis2: SInt32,
    pub fixedDeltaAxis3: SInt32,
    pub pointDeltaAxis1: SInt32,
    pub pointDeltaAxis2: SInt32,
    pub pointDeltaAxis3: SInt32,
    pub reserved8: [SInt32; 4usize],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct NXEventData__bindgen_ty_6 {
    pub __bindgen_anon_1: NXEventData__bindgen_ty_6__bindgen_ty_1,
    pub reserved: SInt16,
    pub subType: SInt16,
    pub misc: NXEventData__bindgen_ty_6__bindgen_ty_1,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union NXEventData__bindgen_ty_6__bindgen_ty_1 {
    pub F: [f32; 11usize],
    pub L: [SInt32; 11usize],
    pub S: [SInt16; 22usize],
    pub C: [::std::os::raw::c_char; 44usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct NXEventData__bindgen_ty_7 {
    pub __bindgen_anon_1: NXEventData__bindgen_ty_7__bindgen_ty_1,
    pub x: SInt32,
    pub y: SInt32,
    pub z: SInt32,
    pub buttons: UInt16,
    pub pressure: UInt16,
    pub tilt: NXEventData__bindgen_ty_7__bindgen_ty_1,
    pub rotation: UInt16,
    pub tangentialPressure: SInt16,
    pub deviceID: UInt16,
    pub vendor1: SInt16,
    pub vendor2: SInt16,
    pub vendor3: SInt16,
    pub reserved: [SInt32; 4usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct NXEventData__bindgen_ty_7__bindgen_ty_1 {
    pub x: SInt16,
    pub y: SInt16,
}
#[repr(C, packed(4))]
#[derive(Debug, Copy, Clone)]
pub struct NXEventData__bindgen_ty_8 {
    pub vendorID: UInt16,
    pub tabletID: UInt16,
    pub pointerID: UInt16,
    pub deviceID: UInt16,
    pub systemTabletID: UInt16,
    pub vendorPointerType: UInt16,
    pub pointerSerialNumber: UInt32,
    pub uniqueID: UInt64,
    pub capabilityMask: UInt32,
    pub pointerType: UInt8,
    pub enterProximity: UInt8,
    pub reserved1: SInt16,
    pub reserved2: [SInt32; 4usize],
}
pub type IOHIDReportType = ::std::os::raw::c_uint;
pub type IOHIDElementCookie = u32;
pub type IOHIDElementType = ::std::os::raw::c_uint;
pub type IOHIDElementCollectionType = ::std::os::raw::c_uint;
pub type IOHIDValueScaleType = u32;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __IOHIDDevice {
    _unused: [u8; 0],
}
pub type IOHIDDeviceRef = *mut __IOHIDDevice;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __IOHIDElement {
    _unused: [u8; 0],
}
pub type IOHIDElementRef = *mut __IOHIDElement;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __IOHIDValue {
    _unused: [u8; 0],
}
pub type IOHIDValueRef = *mut __IOHIDValue;
pub type IOHIDTransactionDirectionType = u32;
pub type IOHIDCallback = ::std::option::Option<
    unsafe extern "C" fn(
        context: *mut ::std::os::raw::c_void,
        result: IOReturn,
        sender: *mut ::std::os::raw::c_void,
    ),
>;
pub type IOHIDReportCallback = ::std::option::Option<
    unsafe extern "C" fn(
        context: *mut ::std::os::raw::c_void,
        result: IOReturn,
        sender: *mut ::std::os::raw::c_void,
        type_: IOHIDReportType,
        reportID: u32,
        report: *mut u8,
        reportLength: CFIndex,
    ),
>;
pub type IOHIDReportWithTimeStampCallback = ::std::option::Option<
    unsafe extern "C" fn(
        context: *mut ::std::os::raw::c_void,
        result: IOReturn,
        sender: *mut ::std::os::raw::c_void,
        type_: IOHIDReportType,
        reportID: u32,
        report: *mut u8,
        reportLength: CFIndex,
        timeStamp: u64,
    ),
>;
pub type IOHIDValueCallback = ::std::option::Option<
    unsafe extern "C" fn(
        context: *mut ::std::os::raw::c_void,
        result: IOReturn,
        sender: *mut ::std::os::raw::c_void,
        value: IOHIDValueRef,
    ),
>;
pub type IOHIDValueMultipleCallback = ::std::option::Option<
    unsafe extern "C" fn(
        context: *mut ::std::os::raw::c_void,
        result: IOReturn,
        sender: *mut ::std::os::raw::c_void,
        multiple: CFDictionaryRef,
    ),
>;
pub type IOHIDDeviceCallback = ::std::option::Option<
    unsafe extern "C" fn(
        context: *mut ::std::os::raw::c_void,
        result: IOReturn,
        sender: *mut ::std::os::raw::c_void,
        device: IOHIDDeviceRef,
    ),
>;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __IOHIDQueue {
    _unused: [u8; 0],
}
pub type IOHIDQueueRef = *mut __IOHIDQueue;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __IOHIDManager {
    _unused: [u8; 0],
}
pub type IOHIDManagerRef = *mut __IOHIDManager;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __IOHIDTransaction {
    _unused: [u8; 0],
}
pub type IOHIDTransactionRef = *mut __IOHIDTransaction;
pub type IOHIDRequestType = ::std::os::raw::c_uint;
pub type IOHIDAccessType = ::std::os::raw::c_uint;
pub type NXEventHandle = mach_port_t;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __IOHIDEventSystemClient {
    _unused: [u8; 0],
}
pub type IOHIDEventSystemClientRef = *mut __IOHIDEventSystemClient;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __IOHIDServiceClient {
    _unused: [u8; 0],
}
pub type IOHIDServiceClientRef = *mut __IOHIDServiceClient;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __IOHIDUserDevice {
    _unused: [u8; 0],
}
pub type IOHIDUserDeviceRef = *mut __IOHIDUserDevice;
pub type IOHIDUserDeviceSetReportBlock = *mut ::std::os::raw::c_void;
pub type IOHIDUserDeviceGetReportBlock = *mut ::std::os::raw::c_void;
pub type IOI2CRequestCompletion =
    ::std::option::Option<unsafe extern "C" fn(request: *mut IOI2CRequest)>;
#[repr(C, packed(4))]
#[derive(Debug, Copy, Clone)]
pub struct IOI2CRequest {
    pub sendTransactionType: IOOptionBits,
    pub replyTransactionType: IOOptionBits,
    pub sendAddress: u32,
    pub replyAddress: u32,
    pub sendSubAddress: u8,
    pub replySubAddress: u8,
    pub __reservedA: [u8; 2usize],
    pub minReplyDelay: u64,
    pub result: IOReturn,
    pub commFlags: IOOptionBits,
    pub __padA: u32,
    pub sendBytes: u32,
    pub __reservedB: [u32; 2usize],
    pub __padB: u32,
    pub replyBytes: u32,
    pub completion: IOI2CRequestCompletion,
    pub sendBuffer: vm_address_t,
    pub replyBuffer: vm_address_t,
    pub __reservedC: [u32; 10usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct IOI2CConnect {
    _unused: [u8; 0],
}
pub type IOI2CConnectRef = *mut IOI2CConnect;
pub type IONDHandle = UInt32;
pub type IOPSLowBatteryWarningLevel = ::std::os::raw::c_uint;
pub type IOPowerSourceCallbackType =
    ::std::option::Option<unsafe extern "C" fn(context: *mut ::std::os::raw::c_void)>;
pub type IOPMAssertionID = u32;
pub type IOPMAssertionLevel = u32;
pub type IOPMUserActiveType = ::std::os::raw::c_uint;
pub type IOSystemLoadAdvisoryLevel = ::std::os::raw::c_int;
impl ::std::fmt::Debug for IORPCMessage {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(stringify ! (# ty)).finish()
    }
}
unsafe extern "C" {
    pub static kIOMainPortDefault: mach_port_t;
}
unsafe extern "C" {
    pub fn IOMainPort(bootstrapPort: mach_port_t, mainPort: *mut mach_port_t) -> kern_return_t;
}
unsafe extern "C" {
    pub static kIOMasterPortDefault: mach_port_t;
}
unsafe extern "C" {
    pub fn IOMasterPort(bootstrapPort: mach_port_t, mainPort: *mut mach_port_t) -> kern_return_t;
}
unsafe extern "C" {
    pub fn IONotificationPortCreate(mainPort: mach_port_t) -> IONotificationPortRef;
}
unsafe extern "C" {
    pub fn IONotificationPortDestroy(notify: IONotificationPortRef);
}
unsafe extern "C" {
    pub fn IONotificationPortGetRunLoopSource(notify: IONotificationPortRef) -> CFRunLoopSourceRef;
}
unsafe extern "C" {
    pub fn IONotificationPortGetMachPort(notify: IONotificationPortRef) -> mach_port_t;
}
unsafe extern "C" {
    pub fn IONotificationPortSetImportanceReceiver(notify: IONotificationPortRef) -> kern_return_t;
}
unsafe extern "C" {
    pub fn IONotificationPortSetDispatchQueue(notify: IONotificationPortRef, queue: NSObject);
}
unsafe extern "C" {
    pub fn IODispatchCalloutFromMessage(
        unused: *mut ::std::os::raw::c_void,
        msg: *mut mach_msg_header_t,
        reference: *mut ::std::os::raw::c_void,
    );
}
unsafe extern "C" {
    pub fn IOCreateReceivePort(msgType: u32, recvPort: *mut mach_port_t) -> kern_return_t;
}
unsafe extern "C" {
    pub fn IOObjectRelease(object: io_object_t) -> kern_return_t;
}
unsafe extern "C" {
    pub fn IOObjectRetain(object: io_object_t) -> kern_return_t;
}
unsafe extern "C" {
    pub fn IOObjectGetClass(
        object: io_object_t,
        className: *mut ::std::os::raw::c_char,
    ) -> kern_return_t;
}
unsafe extern "C" {
    pub fn IOObjectCopyClass(object: io_object_t) -> CFStringRef;
}
unsafe extern "C" {
    pub fn IOObjectCopySuperclassForClass(classname: CFStringRef) -> CFStringRef;
}
unsafe extern "C" {
    pub fn IOObjectCopyBundleIdentifierForClass(classname: CFStringRef) -> CFStringRef;
}
unsafe extern "C" {
    pub fn IOObjectConformsTo(
        object: io_object_t,
        className: *const ::std::os::raw::c_char,
    ) -> boolean_t;
}
unsafe extern "C" {
    pub fn IOObjectIsEqualTo(object: io_object_t, anObject: io_object_t) -> boolean_t;
}
unsafe extern "C" {
    pub fn IOObjectGetKernelRetainCount(object: io_object_t) -> u32;
}
unsafe extern "C" {
    pub fn IOObjectGetUserRetainCount(object: io_object_t) -> u32;
}
unsafe extern "C" {
    pub fn IOObjectGetRetainCount(object: io_object_t) -> u32;
}
unsafe extern "C" {
    pub fn IOIteratorNext(iterator: io_iterator_t) -> io_object_t;
}
unsafe extern "C" {
    pub fn IOIteratorReset(iterator: io_iterator_t);
}
unsafe extern "C" {
    pub fn IOIteratorIsValid(iterator: io_iterator_t) -> boolean_t;
}
unsafe extern "C" {
    pub fn IOServiceGetMatchingService(
        mainPort: mach_port_t,
        matching: CFDictionaryRef,
    ) -> io_service_t;
}
unsafe extern "C" {
    pub fn IOServiceGetMatchingServices(
        mainPort: mach_port_t,
        matching: CFDictionaryRef,
        existing: *mut io_iterator_t,
    ) -> kern_return_t;
}
unsafe extern "C" {
    pub fn IOServiceAddNotification(
        mainPort: mach_port_t,
        notificationType: *const ::std::os::raw::c_char,
        matching: CFDictionaryRef,
        wakePort: mach_port_t,
        reference: usize,
        notification: *mut io_iterator_t,
    ) -> kern_return_t;
}
unsafe extern "C" {
    pub fn IOServiceAddMatchingNotification(
        notifyPort: IONotificationPortRef,
        notificationType: *const ::std::os::raw::c_char,
        matching: CFDictionaryRef,
        callback: IOServiceMatchingCallback,
        refCon: *mut ::std::os::raw::c_void,
        notification: *mut io_iterator_t,
    ) -> kern_return_t;
}
unsafe extern "C" {
    pub fn IOServiceAddInterestNotification(
        notifyPort: IONotificationPortRef,
        service: io_service_t,
        interestType: *const ::std::os::raw::c_char,
        callback: IOServiceInterestCallback,
        refCon: *mut ::std::os::raw::c_void,
        notification: *mut io_object_t,
    ) -> kern_return_t;
}
unsafe extern "C" {
    pub fn IOServiceMatchPropertyTable(
        service: io_service_t,
        matching: CFDictionaryRef,
        matches: *mut boolean_t,
    ) -> kern_return_t;
}
unsafe extern "C" {
    pub fn IOServiceGetBusyState(service: io_service_t, busyState: *mut u32) -> kern_return_t;
}
unsafe extern "C" {
    pub fn IOServiceWaitQuiet(
        service: io_service_t,
        waitTime: *mut mach_timespec_t,
    ) -> kern_return_t;
}
unsafe extern "C" {
    pub fn IOKitGetBusyState(mainPort: mach_port_t, busyState: *mut u32) -> kern_return_t;
}
unsafe extern "C" {
    pub fn IOKitWaitQuietWithOptions(
        mainPort: mach_port_t,
        waitTime: *mut mach_timespec_t,
        options: IOOptionBits,
    ) -> kern_return_t;
}
unsafe extern "C" {
    pub fn IOKitWaitQuiet(mainPort: mach_port_t, waitTime: *mut mach_timespec_t) -> kern_return_t;
}
unsafe extern "C" {
    pub fn IOServiceOpen(
        service: io_service_t,
        owningTask: task_port_t,
        type_: u32,
        connect: *mut io_connect_t,
    ) -> kern_return_t;
}
unsafe extern "C" {
    pub fn IOServiceRequestProbe(service: io_service_t, options: u32) -> kern_return_t;
}
unsafe extern "C" {
    pub fn IOServiceAuthorize(service: io_service_t, options: u32) -> kern_return_t;
}
unsafe extern "C" {
    pub fn IOServiceOpenAsFileDescriptor(
        service: io_service_t,
        oflag: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn IOServiceClose(connect: io_connect_t) -> kern_return_t;
}
unsafe extern "C" {
    pub fn IOConnectAddRef(connect: io_connect_t) -> kern_return_t;
}
unsafe extern "C" {
    pub fn IOConnectRelease(connect: io_connect_t) -> kern_return_t;
}
unsafe extern "C" {
    pub fn IOConnectGetService(connect: io_connect_t, service: *mut io_service_t) -> kern_return_t;
}
unsafe extern "C" {
    pub fn IOConnectSetNotificationPort(
        connect: io_connect_t,
        type_: u32,
        port: mach_port_t,
        reference: usize,
    ) -> kern_return_t;
}
unsafe extern "C" {
    pub fn IOConnectMapMemory(
        connect: io_connect_t,
        memoryType: u32,
        intoTask: task_port_t,
        atAddress: *mut mach_vm_address_t,
        ofSize: *mut mach_vm_size_t,
        options: IOOptionBits,
    ) -> kern_return_t;
}
unsafe extern "C" {
    pub fn IOConnectMapMemory64(
        connect: io_connect_t,
        memoryType: u32,
        intoTask: task_port_t,
        atAddress: *mut mach_vm_address_t,
        ofSize: *mut mach_vm_size_t,
        options: IOOptionBits,
    ) -> kern_return_t;
}
unsafe extern "C" {
    pub fn IOConnectUnmapMemory(
        connect: io_connect_t,
        memoryType: u32,
        fromTask: task_port_t,
        atAddress: mach_vm_address_t,
    ) -> kern_return_t;
}
unsafe extern "C" {
    pub fn IOConnectUnmapMemory64(
        connect: io_connect_t,
        memoryType: u32,
        fromTask: task_port_t,
        atAddress: mach_vm_address_t,
    ) -> kern_return_t;
}
unsafe extern "C" {
    pub fn IOConnectSetCFProperties(connect: io_connect_t, properties: CFTypeRef) -> kern_return_t;
}
unsafe extern "C" {
    pub fn IOConnectSetCFProperty(
        connect: io_connect_t,
        propertyName: CFStringRef,
        property: CFTypeRef,
    ) -> kern_return_t;
}
unsafe extern "C" {
    pub fn IOConnectCallMethod(
        connection: mach_port_t,
        selector: u32,
        input: *const u64,
        inputCnt: u32,
        inputStruct: *const ::std::os::raw::c_void,
        inputStructCnt: usize,
        output: *mut u64,
        outputCnt: *mut u32,
        outputStruct: *mut ::std::os::raw::c_void,
        outputStructCnt: *mut usize,
    ) -> kern_return_t;
}
unsafe extern "C" {
    pub fn IOConnectCallAsyncMethod(
        connection: mach_port_t,
        selector: u32,
        wake_port: mach_port_t,
        reference: *mut u64,
        referenceCnt: u32,
        input: *const u64,
        inputCnt: u32,
        inputStruct: *const ::std::os::raw::c_void,
        inputStructCnt: usize,
        output: *mut u64,
        outputCnt: *mut u32,
        outputStruct: *mut ::std::os::raw::c_void,
        outputStructCnt: *mut usize,
    ) -> kern_return_t;
}
unsafe extern "C" {
    pub fn IOConnectCallStructMethod(
        connection: mach_port_t,
        selector: u32,
        inputStruct: *const ::std::os::raw::c_void,
        inputStructCnt: usize,
        outputStruct: *mut ::std::os::raw::c_void,
        outputStructCnt: *mut usize,
    ) -> kern_return_t;
}
unsafe extern "C" {
    pub fn IOConnectCallAsyncStructMethod(
        connection: mach_port_t,
        selector: u32,
        wake_port: mach_port_t,
        reference: *mut u64,
        referenceCnt: u32,
        inputStruct: *const ::std::os::raw::c_void,
        inputStructCnt: usize,
        outputStruct: *mut ::std::os::raw::c_void,
        outputStructCnt: *mut usize,
    ) -> kern_return_t;
}
unsafe extern "C" {
    pub fn IOConnectCallScalarMethod(
        connection: mach_port_t,
        selector: u32,
        input: *const u64,
        inputCnt: u32,
        output: *mut u64,
        outputCnt: *mut u32,
    ) -> kern_return_t;
}
unsafe extern "C" {
    pub fn IOConnectCallAsyncScalarMethod(
        connection: mach_port_t,
        selector: u32,
        wake_port: mach_port_t,
        reference: *mut u64,
        referenceCnt: u32,
        input: *const u64,
        inputCnt: u32,
        output: *mut u64,
        outputCnt: *mut u32,
    ) -> kern_return_t;
}
unsafe extern "C" {
    pub fn IOConnectTrap0(connect: io_connect_t, index: u32) -> kern_return_t;
}
unsafe extern "C" {
    pub fn IOConnectTrap1(connect: io_connect_t, index: u32, p1: usize) -> kern_return_t;
}
unsafe extern "C" {
    pub fn IOConnectTrap2(connect: io_connect_t, index: u32, p1: usize, p2: usize)
        -> kern_return_t;
}
unsafe extern "C" {
    pub fn IOConnectTrap3(
        connect: io_connect_t,
        index: u32,
        p1: usize,
        p2: usize,
        p3: usize,
    ) -> kern_return_t;
}
unsafe extern "C" {
    pub fn IOConnectTrap4(
        connect: io_connect_t,
        index: u32,
        p1: usize,
        p2: usize,
        p3: usize,
        p4: usize,
    ) -> kern_return_t;
}
unsafe extern "C" {
    pub fn IOConnectTrap5(
        connect: io_connect_t,
        index: u32,
        p1: usize,
        p2: usize,
        p3: usize,
        p4: usize,
        p5: usize,
    ) -> kern_return_t;
}
unsafe extern "C" {
    pub fn IOConnectTrap6(
        connect: io_connect_t,
        index: u32,
        p1: usize,
        p2: usize,
        p3: usize,
        p4: usize,
        p5: usize,
        p6: usize,
    ) -> kern_return_t;
}
unsafe extern "C" {
    pub fn IOConnectAddClient(connect: io_connect_t, client: io_connect_t) -> kern_return_t;
}
unsafe extern "C" {
    pub fn IORegistryGetRootEntry(mainPort: mach_port_t) -> io_registry_entry_t;
}
unsafe extern "C" {
    pub fn IORegistryEntryFromPath(
        mainPort: mach_port_t,
        path: *const ::std::os::raw::c_char,
    ) -> io_registry_entry_t;
}
unsafe extern "C" {
    pub fn IORegistryEntryCopyFromPath(
        mainPort: mach_port_t,
        path: CFStringRef,
    ) -> io_registry_entry_t;
}
unsafe extern "C" {
    pub fn IORegistryCreateIterator(
        mainPort: mach_port_t,
        plane: *const ::std::os::raw::c_char,
        options: IOOptionBits,
        iterator: *mut io_iterator_t,
    ) -> kern_return_t;
}
unsafe extern "C" {
    pub fn IORegistryEntryCreateIterator(
        entry: io_registry_entry_t,
        plane: *const ::std::os::raw::c_char,
        options: IOOptionBits,
        iterator: *mut io_iterator_t,
    ) -> kern_return_t;
}
unsafe extern "C" {
    pub fn IORegistryIteratorEnterEntry(iterator: io_iterator_t) -> kern_return_t;
}
unsafe extern "C" {
    pub fn IORegistryIteratorExitEntry(iterator: io_iterator_t) -> kern_return_t;
}
unsafe extern "C" {
    pub fn IORegistryEntryGetName(
        entry: io_registry_entry_t,
        name: *mut ::std::os::raw::c_char,
    ) -> kern_return_t;
}
unsafe extern "C" {
    pub fn IORegistryEntryGetNameInPlane(
        entry: io_registry_entry_t,
        plane: *const ::std::os::raw::c_char,
        name: *mut ::std::os::raw::c_char,
    ) -> kern_return_t;
}
unsafe extern "C" {
    pub fn IORegistryEntryGetLocationInPlane(
        entry: io_registry_entry_t,
        plane: *const ::std::os::raw::c_char,
        location: *mut ::std::os::raw::c_char,
    ) -> kern_return_t;
}
unsafe extern "C" {
    pub fn IORegistryEntryGetPath(
        entry: io_registry_entry_t,
        plane: *const ::std::os::raw::c_char,
        path: *mut ::std::os::raw::c_char,
    ) -> kern_return_t;
}
unsafe extern "C" {
    pub fn IORegistryEntryCopyPath(
        entry: io_registry_entry_t,
        plane: *const ::std::os::raw::c_char,
    ) -> CFStringRef;
}
unsafe extern "C" {
    pub fn IORegistryEntryGetRegistryEntryID(
        entry: io_registry_entry_t,
        entryID: *mut u64,
    ) -> kern_return_t;
}
unsafe extern "C" {
    pub fn IORegistryEntryCreateCFProperties(
        entry: io_registry_entry_t,
        properties: *mut CFMutableDictionaryRef,
        allocator: CFAllocatorRef,
        options: IOOptionBits,
    ) -> kern_return_t;
}
unsafe extern "C" {
    pub fn IORegistryEntryCreateCFProperty(
        entry: io_registry_entry_t,
        key: CFStringRef,
        allocator: CFAllocatorRef,
        options: IOOptionBits,
    ) -> CFTypeRef;
}
unsafe extern "C" {
    pub fn IORegistryEntrySearchCFProperty(
        entry: io_registry_entry_t,
        plane: *const ::std::os::raw::c_char,
        key: CFStringRef,
        allocator: CFAllocatorRef,
        options: IOOptionBits,
    ) -> CFTypeRef;
}
unsafe extern "C" {
    pub fn IORegistryEntryGetProperty(
        entry: io_registry_entry_t,
        propertyName: *const ::std::os::raw::c_char,
        buffer: *mut ::std::os::raw::c_char,
        size: *mut u32,
    ) -> kern_return_t;
}
unsafe extern "C" {
    pub fn IORegistryEntrySetCFProperties(
        entry: io_registry_entry_t,
        properties: CFTypeRef,
    ) -> kern_return_t;
}
unsafe extern "C" {
    pub fn IORegistryEntrySetCFProperty(
        entry: io_registry_entry_t,
        propertyName: CFStringRef,
        property: CFTypeRef,
    ) -> kern_return_t;
}
unsafe extern "C" {
    pub fn IORegistryEntryGetChildIterator(
        entry: io_registry_entry_t,
        plane: *const ::std::os::raw::c_char,
        iterator: *mut io_iterator_t,
    ) -> kern_return_t;
}
unsafe extern "C" {
    pub fn IORegistryEntryGetChildEntry(
        entry: io_registry_entry_t,
        plane: *const ::std::os::raw::c_char,
        child: *mut io_registry_entry_t,
    ) -> kern_return_t;
}
unsafe extern "C" {
    pub fn IORegistryEntryGetParentIterator(
        entry: io_registry_entry_t,
        plane: *const ::std::os::raw::c_char,
        iterator: *mut io_iterator_t,
    ) -> kern_return_t;
}
unsafe extern "C" {
    pub fn IORegistryEntryGetParentEntry(
        entry: io_registry_entry_t,
        plane: *const ::std::os::raw::c_char,
        parent: *mut io_registry_entry_t,
    ) -> kern_return_t;
}
unsafe extern "C" {
    pub fn IORegistryEntryInPlane(
        entry: io_registry_entry_t,
        plane: *const ::std::os::raw::c_char,
    ) -> boolean_t;
}
unsafe extern "C" {
    pub fn IOServiceMatching(name: *const ::std::os::raw::c_char) -> CFMutableDictionaryRef;
}
unsafe extern "C" {
    pub fn IOServiceNameMatching(name: *const ::std::os::raw::c_char) -> CFMutableDictionaryRef;
}
unsafe extern "C" {
    pub fn IOBSDNameMatching(
        mainPort: mach_port_t,
        options: u32,
        bsdName: *const ::std::os::raw::c_char,
    ) -> CFMutableDictionaryRef;
}
unsafe extern "C" {
    pub fn IOOpenFirmwarePathMatching(
        mainPort: mach_port_t,
        options: u32,
        path: *const ::std::os::raw::c_char,
    ) -> CFMutableDictionaryRef;
}
unsafe extern "C" {
    pub fn IORegistryEntryIDMatching(entryID: u64) -> CFMutableDictionaryRef;
}
unsafe extern "C" {
    pub fn IOServiceOFPathToBSDName(
        mainPort: mach_port_t,
        openFirmwarePath: *const ::std::os::raw::c_char,
        bsdName: *mut ::std::os::raw::c_char,
    ) -> kern_return_t;
}
unsafe extern "C" {
    pub fn OSGetNotificationFromMessage(
        msg: *mut mach_msg_header_t,
        index: u32,
        type_: *mut u32,
        reference: *mut usize,
        content: *mut *mut ::std::os::raw::c_void,
        size: *mut vm_size_t,
    ) -> kern_return_t;
}
unsafe extern "C" {
    pub fn IOCatalogueSendData(
        mainPort: mach_port_t,
        flag: u32,
        buffer: *const ::std::os::raw::c_char,
        size: u32,
    ) -> kern_return_t;
}
unsafe extern "C" {
    pub fn IOCatalogueTerminate(
        mainPort: mach_port_t,
        flag: u32,
        description: *mut ::std::os::raw::c_char,
    ) -> kern_return_t;
}
unsafe extern "C" {
    pub fn IOCatalogueGetData(
        mainPort: mach_port_t,
        flag: u32,
        buffer: *mut *mut ::std::os::raw::c_char,
        size: *mut u32,
    ) -> kern_return_t;
}
unsafe extern "C" {
    pub fn IOCatalogueModuleLoaded(
        mainPort: mach_port_t,
        name: *mut ::std::os::raw::c_char,
    ) -> kern_return_t;
}
unsafe extern "C" {
    pub fn IOCatalogueReset(mainPort: mach_port_t, flag: u32) -> kern_return_t;
}
unsafe extern "C" {
    pub fn IODataQueueDataAvailable(dataQueue: *mut IODataQueueMemory) -> Boolean;
}
unsafe extern "C" {
    pub fn IODataQueuePeek(dataQueue: *mut IODataQueueMemory) -> *mut IODataQueueEntry;
}
unsafe extern "C" {
    pub fn IODataQueueDequeue(
        dataQueue: *mut IODataQueueMemory,
        data: *mut ::std::os::raw::c_void,
        dataSize: *mut u32,
    ) -> IOReturn;
}
unsafe extern "C" {
    pub fn IODataQueueWaitForAvailableData(
        dataQueue: *mut IODataQueueMemory,
        notificationPort: mach_port_t,
    ) -> IOReturn;
}
unsafe extern "C" {
    pub fn IODataQueueAllocateNotificationPort() -> mach_port_t;
}
unsafe extern "C" {
    pub fn IODataQueueEnqueue(
        dataQueue: *mut IODataQueueMemory,
        data: *mut ::std::os::raw::c_void,
        dataSize: u32,
    ) -> IOReturn;
}
unsafe extern "C" {
    pub fn IODataQueueSetNotificationPort(
        dataQueue: *mut IODataQueueMemory,
        notifyPort: mach_port_t,
    ) -> IOReturn;
}
unsafe extern "C" {
    pub fn IOCreatePlugInInterfaceForService(
        service: io_service_t,
        pluginType: CFUUIDRef,
        interfaceType: CFUUIDRef,
        theInterface: *mut *mut *mut IOCFPlugInInterface,
        theScore: *mut SInt32,
    ) -> kern_return_t;
}
unsafe extern "C" {
    pub fn IODestroyPlugInInterface(interface: *mut *mut IOCFPlugInInterface) -> kern_return_t;
}
unsafe extern "C" {
    pub fn IOCFSerialize(object: CFTypeRef, options: CFOptionFlags) -> CFDataRef;
}
unsafe extern "C" {
    pub fn IOURLCreatePropertyFromResource(
        alloc: CFAllocatorRef,
        url: CFURLRef,
        property: CFStringRef,
        errorCode: *mut SInt32,
    ) -> CFTypeRef;
}
unsafe extern "C" {
    pub fn IOURLCreateDataAndPropertiesFromResource(
        alloc: CFAllocatorRef,
        url: CFURLRef,
        resourceData: *mut CFDataRef,
        properties: *mut CFDictionaryRef,
        desiredProperties: CFArrayRef,
        errorCode: *mut SInt32,
    ) -> Boolean;
}
unsafe extern "C" {
    pub fn IOCFUnserialize(
        buffer: *const ::std::os::raw::c_char,
        allocator: CFAllocatorRef,
        options: CFOptionFlags,
        errorString: *mut CFStringRef,
    ) -> CFTypeRef;
}
unsafe extern "C" {
    pub fn IOCFUnserializeBinary(
        buffer: *const ::std::os::raw::c_char,
        bufferSize: usize,
        allocator: CFAllocatorRef,
        options: CFOptionFlags,
        errorString: *mut CFStringRef,
    ) -> CFTypeRef;
}
unsafe extern "C" {
    pub fn IOCFUnserializeWithSize(
        buffer: *const ::std::os::raw::c_char,
        bufferSize: usize,
        allocator: CFAllocatorRef,
        options: CFOptionFlags,
        errorString: *mut CFStringRef,
    ) -> CFTypeRef;
}
unsafe extern "C" {
    pub fn IOAccelFindAccelerator(
        framebuffer: io_service_t,
        pAccelerator: *mut io_service_t,
        pFramebufferIndex: *mut UInt32,
    ) -> IOReturn;
}
unsafe extern "C" {
    pub fn IOFramebufferOpen(
        service: io_service_t,
        owningTask: task_port_t,
        type_: ::std::os::raw::c_uint,
        connect: *mut io_connect_t,
    ) -> kern_return_t;
}
unsafe extern "C" {
    pub fn IODisplayCreateInfoDictionary(
        framebuffer: io_service_t,
        options: IOOptionBits,
    ) -> CFDictionaryRef;
}
unsafe extern "C" {
    pub fn IODisplayMatchDictionaries(
        matching1: CFDictionaryRef,
        matching2: CFDictionaryRef,
        options: IOOptionBits,
    ) -> SInt32;
}
unsafe extern "C" {
    pub fn IODisplayForFramebuffer(
        framebuffer: io_service_t,
        options: IOOptionBits,
    ) -> io_service_t;
}
unsafe extern "C" {
    pub fn IODisplaySetParameters(
        service: io_service_t,
        options: IOOptionBits,
        params: CFDictionaryRef,
    ) -> IOReturn;
}
unsafe extern "C" {
    pub fn IODisplaySetFloatParameter(
        service: io_service_t,
        options: IOOptionBits,
        parameterName: CFStringRef,
        value: f32,
    ) -> IOReturn;
}
unsafe extern "C" {
    pub fn IODisplaySetIntegerParameter(
        service: io_service_t,
        options: IOOptionBits,
        parameterName: CFStringRef,
        value: SInt32,
    ) -> IOReturn;
}
unsafe extern "C" {
    pub fn IODisplayCopyParameters(
        service: io_service_t,
        options: IOOptionBits,
        params: *mut CFDictionaryRef,
    ) -> IOReturn;
}
unsafe extern "C" {
    pub fn IODisplayCopyFloatParameters(
        service: io_service_t,
        options: IOOptionBits,
        params: *mut CFDictionaryRef,
    ) -> IOReturn;
}
unsafe extern "C" {
    pub fn IODisplayGetFloatParameter(
        service: io_service_t,
        options: IOOptionBits,
        parameterName: CFStringRef,
        value: *mut f32,
    ) -> IOReturn;
}
unsafe extern "C" {
    pub fn IODisplayGetIntegerRangeParameter(
        service: io_service_t,
        options: IOOptionBits,
        parameterName: CFStringRef,
        value: *mut SInt32,
        min: *mut SInt32,
        max: *mut SInt32,
    ) -> IOReturn;
}
unsafe extern "C" {
    pub fn IODisplayCommitParameters(service: io_service_t, options: IOOptionBits) -> IOReturn;
}
unsafe extern "C" {
    pub fn IOHIDQueueGetTypeID() -> CFTypeID;
}
unsafe extern "C" {
    pub fn IOHIDQueueCreate(
        allocator: CFAllocatorRef,
        device: IOHIDDeviceRef,
        depth: CFIndex,
        options: IOOptionBits,
    ) -> IOHIDQueueRef;
}
unsafe extern "C" {
    pub fn IOHIDQueueGetDevice(queue: IOHIDQueueRef) -> IOHIDDeviceRef;
}
unsafe extern "C" {
    pub fn IOHIDQueueGetDepth(queue: IOHIDQueueRef) -> CFIndex;
}
unsafe extern "C" {
    pub fn IOHIDQueueSetDepth(queue: IOHIDQueueRef, depth: CFIndex);
}
unsafe extern "C" {
    pub fn IOHIDQueueAddElement(queue: IOHIDQueueRef, element: IOHIDElementRef);
}
unsafe extern "C" {
    pub fn IOHIDQueueRemoveElement(queue: IOHIDQueueRef, element: IOHIDElementRef);
}
unsafe extern "C" {
    pub fn IOHIDQueueContainsElement(queue: IOHIDQueueRef, element: IOHIDElementRef) -> Boolean;
}
unsafe extern "C" {
    pub fn IOHIDQueueStart(queue: IOHIDQueueRef);
}
unsafe extern "C" {
    pub fn IOHIDQueueStop(queue: IOHIDQueueRef);
}
unsafe extern "C" {
    pub fn IOHIDQueueScheduleWithRunLoop(
        queue: IOHIDQueueRef,
        runLoop: CFRunLoopRef,
        runLoopMode: CFStringRef,
    );
}
unsafe extern "C" {
    pub fn IOHIDQueueUnscheduleFromRunLoop(
        queue: IOHIDQueueRef,
        runLoop: CFRunLoopRef,
        runLoopMode: CFStringRef,
    );
}
unsafe extern "C" {
    pub fn IOHIDQueueSetDispatchQueue(queue: IOHIDQueueRef, dispatchQueue: NSObject);
}
unsafe extern "C" {
    pub fn IOHIDQueueSetCancelHandler(queue: IOHIDQueueRef, handler: dispatch_block_t);
}
unsafe extern "C" {
    pub fn IOHIDQueueActivate(queue: IOHIDQueueRef);
}
unsafe extern "C" {
    pub fn IOHIDQueueCancel(queue: IOHIDQueueRef);
}
unsafe extern "C" {
    pub fn IOHIDQueueRegisterValueAvailableCallback(
        queue: IOHIDQueueRef,
        callback: IOHIDCallback,
        context: *mut ::std::os::raw::c_void,
    );
}
unsafe extern "C" {
    pub fn IOHIDQueueCopyNextValue(queue: IOHIDQueueRef) -> IOHIDValueRef;
}
unsafe extern "C" {
    pub fn IOHIDQueueCopyNextValueWithTimeout(
        queue: IOHIDQueueRef,
        timeout: CFTimeInterval,
    ) -> IOHIDValueRef;
}
unsafe extern "C" {
    pub fn IOHIDDeviceGetTypeID() -> CFTypeID;
}
unsafe extern "C" {
    pub fn IOHIDDeviceCreate(allocator: CFAllocatorRef, service: io_service_t) -> IOHIDDeviceRef;
}
unsafe extern "C" {
    pub fn IOHIDDeviceGetService(device: IOHIDDeviceRef) -> io_service_t;
}
unsafe extern "C" {
    pub fn IOHIDDeviceOpen(device: IOHIDDeviceRef, options: IOOptionBits) -> IOReturn;
}
unsafe extern "C" {
    pub fn IOHIDDeviceClose(device: IOHIDDeviceRef, options: IOOptionBits) -> IOReturn;
}
unsafe extern "C" {
    pub fn IOHIDDeviceConformsTo(device: IOHIDDeviceRef, usagePage: u32, usage: u32) -> Boolean;
}
unsafe extern "C" {
    pub fn IOHIDDeviceGetProperty(device: IOHIDDeviceRef, key: CFStringRef) -> CFTypeRef;
}
unsafe extern "C" {
    pub fn IOHIDDeviceSetProperty(
        device: IOHIDDeviceRef,
        key: CFStringRef,
        property: CFTypeRef,
    ) -> Boolean;
}
unsafe extern "C" {
    pub fn IOHIDDeviceCopyMatchingElements(
        device: IOHIDDeviceRef,
        matching: CFDictionaryRef,
        options: IOOptionBits,
    ) -> CFArrayRef;
}
unsafe extern "C" {
    pub fn IOHIDDeviceScheduleWithRunLoop(
        device: IOHIDDeviceRef,
        runLoop: CFRunLoopRef,
        runLoopMode: CFStringRef,
    );
}
unsafe extern "C" {
    pub fn IOHIDDeviceUnscheduleFromRunLoop(
        device: IOHIDDeviceRef,
        runLoop: CFRunLoopRef,
        runLoopMode: CFStringRef,
    );
}
unsafe extern "C" {
    pub fn IOHIDDeviceSetDispatchQueue(device: IOHIDDeviceRef, queue: NSObject);
}
unsafe extern "C" {
    pub fn IOHIDDeviceSetCancelHandler(device: IOHIDDeviceRef, handler: dispatch_block_t);
}
unsafe extern "C" {
    pub fn IOHIDDeviceActivate(device: IOHIDDeviceRef);
}
unsafe extern "C" {
    pub fn IOHIDDeviceCancel(device: IOHIDDeviceRef);
}
unsafe extern "C" {
    pub fn IOHIDDeviceRegisterRemovalCallback(
        device: IOHIDDeviceRef,
        callback: IOHIDCallback,
        context: *mut ::std::os::raw::c_void,
    );
}
unsafe extern "C" {
    pub fn IOHIDDeviceRegisterInputValueCallback(
        device: IOHIDDeviceRef,
        callback: IOHIDValueCallback,
        context: *mut ::std::os::raw::c_void,
    );
}
unsafe extern "C" {
    pub fn IOHIDDeviceRegisterInputReportCallback(
        device: IOHIDDeviceRef,
        report: *mut u8,
        reportLength: CFIndex,
        callback: IOHIDReportCallback,
        context: *mut ::std::os::raw::c_void,
    );
}
unsafe extern "C" {
    pub fn IOHIDDeviceRegisterInputReportWithTimeStampCallback(
        device: IOHIDDeviceRef,
        report: *mut u8,
        reportLength: CFIndex,
        callback: IOHIDReportWithTimeStampCallback,
        context: *mut ::std::os::raw::c_void,
    );
}
unsafe extern "C" {
    pub fn IOHIDDeviceSetInputValueMatching(device: IOHIDDeviceRef, matching: CFDictionaryRef);
}
unsafe extern "C" {
    pub fn IOHIDDeviceSetInputValueMatchingMultiple(device: IOHIDDeviceRef, multiple: CFArrayRef);
}
unsafe extern "C" {
    pub fn IOHIDDeviceSetValue(
        device: IOHIDDeviceRef,
        element: IOHIDElementRef,
        value: IOHIDValueRef,
    ) -> IOReturn;
}
unsafe extern "C" {
    pub fn IOHIDDeviceSetValueMultiple(
        device: IOHIDDeviceRef,
        multiple: CFDictionaryRef,
    ) -> IOReturn;
}
unsafe extern "C" {
    pub fn IOHIDDeviceSetValueWithCallback(
        device: IOHIDDeviceRef,
        element: IOHIDElementRef,
        value: IOHIDValueRef,
        timeout: CFTimeInterval,
        callback: IOHIDValueCallback,
        context: *mut ::std::os::raw::c_void,
    ) -> IOReturn;
}
unsafe extern "C" {
    pub fn IOHIDDeviceSetValueMultipleWithCallback(
        device: IOHIDDeviceRef,
        multiple: CFDictionaryRef,
        timeout: CFTimeInterval,
        callback: IOHIDValueMultipleCallback,
        context: *mut ::std::os::raw::c_void,
    ) -> IOReturn;
}
unsafe extern "C" {
    pub fn IOHIDDeviceGetValue(
        device: IOHIDDeviceRef,
        element: IOHIDElementRef,
        pValue: *mut IOHIDValueRef,
    ) -> IOReturn;
}
unsafe extern "C" {
    pub fn IOHIDDeviceGetValueWithOptions(
        device: IOHIDDeviceRef,
        element: IOHIDElementRef,
        pValue: *mut IOHIDValueRef,
        options: u32,
    ) -> IOReturn;
}
unsafe extern "C" {
    pub fn IOHIDDeviceCopyValueMultiple(
        device: IOHIDDeviceRef,
        elements: CFArrayRef,
        pMultiple: *mut CFDictionaryRef,
    ) -> IOReturn;
}
unsafe extern "C" {
    pub fn IOHIDDeviceGetValueWithCallback(
        device: IOHIDDeviceRef,
        element: IOHIDElementRef,
        pValue: *mut IOHIDValueRef,
        timeout: CFTimeInterval,
        callback: IOHIDValueCallback,
        context: *mut ::std::os::raw::c_void,
    ) -> IOReturn;
}
unsafe extern "C" {
    pub fn IOHIDDeviceCopyValueMultipleWithCallback(
        device: IOHIDDeviceRef,
        elements: CFArrayRef,
        pMultiple: *mut CFDictionaryRef,
        timeout: CFTimeInterval,
        callback: IOHIDValueMultipleCallback,
        context: *mut ::std::os::raw::c_void,
    ) -> IOReturn;
}
unsafe extern "C" {
    pub fn IOHIDDeviceSetReport(
        device: IOHIDDeviceRef,
        reportType: IOHIDReportType,
        reportID: CFIndex,
        report: *const u8,
        reportLength: CFIndex,
    ) -> IOReturn;
}
unsafe extern "C" {
    pub fn IOHIDDeviceSetReportWithCallback(
        device: IOHIDDeviceRef,
        reportType: IOHIDReportType,
        reportID: CFIndex,
        report: *const u8,
        reportLength: CFIndex,
        timeout: CFTimeInterval,
        callback: IOHIDReportCallback,
        context: *mut ::std::os::raw::c_void,
    ) -> IOReturn;
}
unsafe extern "C" {
    pub fn IOHIDDeviceGetReport(
        device: IOHIDDeviceRef,
        reportType: IOHIDReportType,
        reportID: CFIndex,
        report: *mut u8,
        pReportLength: *mut CFIndex,
    ) -> IOReturn;
}
unsafe extern "C" {
    pub fn IOHIDDeviceGetReportWithCallback(
        device: IOHIDDeviceRef,
        reportType: IOHIDReportType,
        reportID: CFIndex,
        report: *mut u8,
        pReportLength: *mut CFIndex,
        timeout: CFTimeInterval,
        callback: IOHIDReportCallback,
        context: *mut ::std::os::raw::c_void,
    ) -> IOReturn;
}
unsafe extern "C" {
    pub fn IOHIDElementGetTypeID() -> CFTypeID;
}
unsafe extern "C" {
    pub fn IOHIDElementCreateWithDictionary(
        allocator: CFAllocatorRef,
        dictionary: CFDictionaryRef,
    ) -> IOHIDElementRef;
}
unsafe extern "C" {
    pub fn IOHIDElementGetDevice(element: IOHIDElementRef) -> IOHIDDeviceRef;
}
unsafe extern "C" {
    pub fn IOHIDElementGetParent(element: IOHIDElementRef) -> IOHIDElementRef;
}
unsafe extern "C" {
    pub fn IOHIDElementGetChildren(element: IOHIDElementRef) -> CFArrayRef;
}
unsafe extern "C" {
    pub fn IOHIDElementAttach(element: IOHIDElementRef, toAttach: IOHIDElementRef);
}
unsafe extern "C" {
    pub fn IOHIDElementDetach(element: IOHIDElementRef, toDetach: IOHIDElementRef);
}
unsafe extern "C" {
    pub fn IOHIDElementCopyAttached(element: IOHIDElementRef) -> CFArrayRef;
}
unsafe extern "C" {
    pub fn IOHIDElementGetCookie(element: IOHIDElementRef) -> IOHIDElementCookie;
}
unsafe extern "C" {
    pub fn IOHIDElementGetType(element: IOHIDElementRef) -> IOHIDElementType;
}
unsafe extern "C" {
    pub fn IOHIDElementGetCollectionType(element: IOHIDElementRef) -> IOHIDElementCollectionType;
}
unsafe extern "C" {
    pub fn IOHIDElementGetUsagePage(element: IOHIDElementRef) -> u32;
}
unsafe extern "C" {
    pub fn IOHIDElementGetUsage(element: IOHIDElementRef) -> u32;
}
unsafe extern "C" {
    pub fn IOHIDElementIsVirtual(element: IOHIDElementRef) -> Boolean;
}
unsafe extern "C" {
    pub fn IOHIDElementIsRelative(element: IOHIDElementRef) -> Boolean;
}
unsafe extern "C" {
    pub fn IOHIDElementIsWrapping(element: IOHIDElementRef) -> Boolean;
}
unsafe extern "C" {
    pub fn IOHIDElementIsArray(element: IOHIDElementRef) -> Boolean;
}
unsafe extern "C" {
    pub fn IOHIDElementIsNonLinear(element: IOHIDElementRef) -> Boolean;
}
unsafe extern "C" {
    pub fn IOHIDElementHasPreferredState(element: IOHIDElementRef) -> Boolean;
}
unsafe extern "C" {
    pub fn IOHIDElementHasNullState(element: IOHIDElementRef) -> Boolean;
}
unsafe extern "C" {
    pub fn IOHIDElementGetName(element: IOHIDElementRef) -> CFStringRef;
}
unsafe extern "C" {
    pub fn IOHIDElementGetReportID(element: IOHIDElementRef) -> u32;
}
unsafe extern "C" {
    pub fn IOHIDElementGetReportSize(element: IOHIDElementRef) -> u32;
}
unsafe extern "C" {
    pub fn IOHIDElementGetReportCount(element: IOHIDElementRef) -> u32;
}
unsafe extern "C" {
    pub fn IOHIDElementGetUnit(element: IOHIDElementRef) -> u32;
}
unsafe extern "C" {
    pub fn IOHIDElementGetUnitExponent(element: IOHIDElementRef) -> u32;
}
unsafe extern "C" {
    pub fn IOHIDElementGetLogicalMin(element: IOHIDElementRef) -> CFIndex;
}
unsafe extern "C" {
    pub fn IOHIDElementGetLogicalMax(element: IOHIDElementRef) -> CFIndex;
}
unsafe extern "C" {
    pub fn IOHIDElementGetPhysicalMin(element: IOHIDElementRef) -> CFIndex;
}
unsafe extern "C" {
    pub fn IOHIDElementGetPhysicalMax(element: IOHIDElementRef) -> CFIndex;
}
unsafe extern "C" {
    pub fn IOHIDElementGetProperty(element: IOHIDElementRef, key: CFStringRef) -> CFTypeRef;
}
unsafe extern "C" {
    pub fn IOHIDElementSetProperty(
        element: IOHIDElementRef,
        key: CFStringRef,
        property: CFTypeRef,
    ) -> Boolean;
}
unsafe extern "C" {
    pub fn IOHIDManagerGetTypeID() -> CFTypeID;
}
unsafe extern "C" {
    pub fn IOHIDManagerCreate(allocator: CFAllocatorRef, options: IOOptionBits) -> IOHIDManagerRef;
}
unsafe extern "C" {
    pub fn IOHIDManagerOpen(manager: IOHIDManagerRef, options: IOOptionBits) -> IOReturn;
}
unsafe extern "C" {
    pub fn IOHIDManagerClose(manager: IOHIDManagerRef, options: IOOptionBits) -> IOReturn;
}
unsafe extern "C" {
    pub fn IOHIDManagerGetProperty(manager: IOHIDManagerRef, key: CFStringRef) -> CFTypeRef;
}
unsafe extern "C" {
    pub fn IOHIDManagerSetProperty(
        manager: IOHIDManagerRef,
        key: CFStringRef,
        value: CFTypeRef,
    ) -> Boolean;
}
unsafe extern "C" {
    pub fn IOHIDManagerScheduleWithRunLoop(
        manager: IOHIDManagerRef,
        runLoop: CFRunLoopRef,
        runLoopMode: CFStringRef,
    );
}
unsafe extern "C" {
    pub fn IOHIDManagerUnscheduleFromRunLoop(
        manager: IOHIDManagerRef,
        runLoop: CFRunLoopRef,
        runLoopMode: CFStringRef,
    );
}
unsafe extern "C" {
    pub fn IOHIDManagerSetDispatchQueue(manager: IOHIDManagerRef, queue: NSObject);
}
unsafe extern "C" {
    pub fn IOHIDManagerSetCancelHandler(manager: IOHIDManagerRef, handler: dispatch_block_t);
}
unsafe extern "C" {
    pub fn IOHIDManagerActivate(manager: IOHIDManagerRef);
}
unsafe extern "C" {
    pub fn IOHIDManagerCancel(manager: IOHIDManagerRef);
}
unsafe extern "C" {
    pub fn IOHIDManagerSetDeviceMatching(manager: IOHIDManagerRef, matching: CFDictionaryRef);
}
unsafe extern "C" {
    pub fn IOHIDManagerSetDeviceMatchingMultiple(manager: IOHIDManagerRef, multiple: CFArrayRef);
}
unsafe extern "C" {
    pub fn IOHIDManagerCopyDevices(manager: IOHIDManagerRef) -> CFSetRef;
}
unsafe extern "C" {
    pub fn IOHIDManagerRegisterDeviceMatchingCallback(
        manager: IOHIDManagerRef,
        callback: IOHIDDeviceCallback,
        context: *mut ::std::os::raw::c_void,
    );
}
unsafe extern "C" {
    pub fn IOHIDManagerRegisterDeviceRemovalCallback(
        manager: IOHIDManagerRef,
        callback: IOHIDDeviceCallback,
        context: *mut ::std::os::raw::c_void,
    );
}
unsafe extern "C" {
    pub fn IOHIDManagerRegisterInputReportCallback(
        manager: IOHIDManagerRef,
        callback: IOHIDReportCallback,
        context: *mut ::std::os::raw::c_void,
    );
}
unsafe extern "C" {
    pub fn IOHIDManagerRegisterInputReportWithTimeStampCallback(
        manager: IOHIDManagerRef,
        callback: IOHIDReportWithTimeStampCallback,
        context: *mut ::std::os::raw::c_void,
    );
}
unsafe extern "C" {
    pub fn IOHIDManagerRegisterInputValueCallback(
        manager: IOHIDManagerRef,
        callback: IOHIDValueCallback,
        context: *mut ::std::os::raw::c_void,
    );
}
unsafe extern "C" {
    pub fn IOHIDManagerSetInputValueMatching(manager: IOHIDManagerRef, matching: CFDictionaryRef);
}
unsafe extern "C" {
    pub fn IOHIDManagerSetInputValueMatchingMultiple(
        manager: IOHIDManagerRef,
        multiple: CFArrayRef,
    );
}
unsafe extern "C" {
    pub fn IOHIDManagerSaveToPropertyDomain(
        manager: IOHIDManagerRef,
        applicationID: CFStringRef,
        userName: CFStringRef,
        hostName: CFStringRef,
        options: IOOptionBits,
    );
}
unsafe extern "C" {
    pub fn IOHIDValueGetTypeID() -> CFTypeID;
}
unsafe extern "C" {
    pub fn IOHIDValueCreateWithIntegerValue(
        allocator: CFAllocatorRef,
        element: IOHIDElementRef,
        timeStamp: u64,
        value: CFIndex,
    ) -> IOHIDValueRef;
}
unsafe extern "C" {
    pub fn IOHIDValueCreateWithBytes(
        allocator: CFAllocatorRef,
        element: IOHIDElementRef,
        timeStamp: u64,
        bytes: *const u8,
        length: CFIndex,
    ) -> IOHIDValueRef;
}
unsafe extern "C" {
    pub fn IOHIDValueCreateWithBytesNoCopy(
        allocator: CFAllocatorRef,
        element: IOHIDElementRef,
        timeStamp: u64,
        bytes: *const u8,
        length: CFIndex,
    ) -> IOHIDValueRef;
}
unsafe extern "C" {
    pub fn IOHIDValueGetElement(value: IOHIDValueRef) -> IOHIDElementRef;
}
unsafe extern "C" {
    pub fn IOHIDValueGetTimeStamp(value: IOHIDValueRef) -> u64;
}
unsafe extern "C" {
    pub fn IOHIDValueGetLength(value: IOHIDValueRef) -> CFIndex;
}
unsafe extern "C" {
    pub fn IOHIDValueGetBytePtr(value: IOHIDValueRef) -> *const u8;
}
unsafe extern "C" {
    pub fn IOHIDValueGetIntegerValue(value: IOHIDValueRef) -> CFIndex;
}
unsafe extern "C" {
    pub fn IOHIDValueGetScaledValue(value: IOHIDValueRef, type_: IOHIDValueScaleType) -> double_t;
}
unsafe extern "C" {
    pub fn IOHIDTransactionGetTypeID() -> CFTypeID;
}
unsafe extern "C" {
    pub fn IOHIDTransactionCreate(
        allocator: CFAllocatorRef,
        device: IOHIDDeviceRef,
        direction: IOHIDTransactionDirectionType,
        options: IOOptionBits,
    ) -> IOHIDTransactionRef;
}
unsafe extern "C" {
    pub fn IOHIDTransactionGetDevice(transaction: IOHIDTransactionRef) -> IOHIDDeviceRef;
}
unsafe extern "C" {
    pub fn IOHIDTransactionGetDirection(
        transaction: IOHIDTransactionRef,
    ) -> IOHIDTransactionDirectionType;
}
unsafe extern "C" {
    pub fn IOHIDTransactionSetDirection(
        transaction: IOHIDTransactionRef,
        direction: IOHIDTransactionDirectionType,
    );
}
unsafe extern "C" {
    pub fn IOHIDTransactionAddElement(transaction: IOHIDTransactionRef, element: IOHIDElementRef);
}
unsafe extern "C" {
    pub fn IOHIDTransactionRemoveElement(
        transaction: IOHIDTransactionRef,
        element: IOHIDElementRef,
    );
}
unsafe extern "C" {
    pub fn IOHIDTransactionContainsElement(
        transaction: IOHIDTransactionRef,
        element: IOHIDElementRef,
    ) -> Boolean;
}
unsafe extern "C" {
    pub fn IOHIDTransactionScheduleWithRunLoop(
        transaction: IOHIDTransactionRef,
        runLoop: CFRunLoopRef,
        runLoopMode: CFStringRef,
    );
}
unsafe extern "C" {
    pub fn IOHIDTransactionUnscheduleFromRunLoop(
        transaction: IOHIDTransactionRef,
        runLoop: CFRunLoopRef,
        runLoopMode: CFStringRef,
    );
}
unsafe extern "C" {
    pub fn IOHIDTransactionSetValue(
        transaction: IOHIDTransactionRef,
        element: IOHIDElementRef,
        value: IOHIDValueRef,
        options: IOOptionBits,
    );
}
unsafe extern "C" {
    pub fn IOHIDTransactionGetValue(
        transaction: IOHIDTransactionRef,
        element: IOHIDElementRef,
        options: IOOptionBits,
    ) -> IOHIDValueRef;
}
unsafe extern "C" {
    pub fn IOHIDTransactionCommit(transaction: IOHIDTransactionRef) -> IOReturn;
}
unsafe extern "C" {
    pub fn IOHIDTransactionCommitWithCallback(
        transaction: IOHIDTransactionRef,
        timeout: CFTimeInterval,
        callback: IOHIDCallback,
        context: *mut ::std::os::raw::c_void,
    ) -> IOReturn;
}
unsafe extern "C" {
    pub fn IOHIDTransactionClear(transaction: IOHIDTransactionRef);
}
unsafe extern "C" {
    pub fn IOHIDCreateSharedMemory(
        connect: io_connect_t,
        version: ::std::os::raw::c_uint,
    ) -> kern_return_t;
}
unsafe extern "C" {
    pub fn IOHIDSetEventsEnable(connect: io_connect_t, enable: boolean_t) -> kern_return_t;
}
unsafe extern "C" {
    pub fn IOHIDSetCursorEnable(connect: io_connect_t, enable: boolean_t) -> kern_return_t;
}
unsafe extern "C" {
    pub fn IOHIDPostEvent(
        connect: io_connect_t,
        eventType: UInt32,
        location: IOGPoint,
        eventData: *const NXEventData,
        eventDataVersion: UInt32,
        eventFlags: IOOptionBits,
        options: IOOptionBits,
    ) -> kern_return_t;
}
unsafe extern "C" {
    pub fn IOHIDSetMouseLocation(
        connect: io_connect_t,
        x: ::std::os::raw::c_int,
        y: ::std::os::raw::c_int,
    ) -> kern_return_t;
}
unsafe extern "C" {
    pub fn IOHIDGetButtonEventNum(
        connect: io_connect_t,
        button: NXMouseButton,
        eventNum: *mut ::std::os::raw::c_int,
    ) -> kern_return_t;
}
unsafe extern "C" {
    pub fn IOHIDGetScrollAcceleration(
        handle: io_connect_t,
        acceleration: *mut f64,
    ) -> kern_return_t;
}
unsafe extern "C" {
    pub fn IOHIDSetScrollAcceleration(handle: io_connect_t, acceleration: f64) -> kern_return_t;
}
unsafe extern "C" {
    pub fn IOHIDGetMouseAcceleration(handle: io_connect_t, acceleration: *mut f64)
        -> kern_return_t;
}
unsafe extern "C" {
    pub fn IOHIDSetMouseAcceleration(handle: io_connect_t, acceleration: f64) -> kern_return_t;
}
unsafe extern "C" {
    pub fn IOHIDGetMouseButtonMode(
        handle: io_connect_t,
        mode: *mut ::std::os::raw::c_int,
    ) -> kern_return_t;
}
unsafe extern "C" {
    pub fn IOHIDSetMouseButtonMode(
        handle: io_connect_t,
        mode: ::std::os::raw::c_int,
    ) -> kern_return_t;
}
unsafe extern "C" {
    pub fn IOHIDGetAccelerationWithKey(
        handle: io_connect_t,
        key: CFStringRef,
        acceleration: *mut f64,
    ) -> kern_return_t;
}
unsafe extern "C" {
    pub fn IOHIDSetAccelerationWithKey(
        handle: io_connect_t,
        key: CFStringRef,
        acceleration: f64,
    ) -> kern_return_t;
}
unsafe extern "C" {
    pub fn IOHIDGetParameter(
        handle: io_connect_t,
        key: CFStringRef,
        maxSize: IOByteCount,
        bytes: *mut ::std::os::raw::c_void,
        actualSize: *mut IOByteCount,
    ) -> kern_return_t;
}
unsafe extern "C" {
    pub fn IOHIDSetParameter(
        handle: io_connect_t,
        key: CFStringRef,
        bytes: *const ::std::os::raw::c_void,
        size: IOByteCount,
    ) -> kern_return_t;
}
unsafe extern "C" {
    pub fn IOHIDCopyCFTypeParameter(
        handle: io_connect_t,
        key: CFStringRef,
        parameter: *mut CFTypeRef,
    ) -> kern_return_t;
}
unsafe extern "C" {
    pub fn IOHIDSetCFTypeParameter(
        handle: io_connect_t,
        key: CFStringRef,
        parameter: CFTypeRef,
    ) -> kern_return_t;
}
unsafe extern "C" {
    pub fn IOHIDGetStateForSelector(
        handle: io_connect_t,
        selector: ::std::os::raw::c_int,
        state: *mut UInt32,
    ) -> kern_return_t;
}
unsafe extern "C" {
    pub fn IOHIDSetStateForSelector(
        handle: io_connect_t,
        selector: ::std::os::raw::c_int,
        state: UInt32,
    ) -> kern_return_t;
}
unsafe extern "C" {
    pub fn IOHIDGetModifierLockState(
        handle: io_connect_t,
        selector: ::std::os::raw::c_int,
        state: *mut bool,
    ) -> kern_return_t;
}
unsafe extern "C" {
    pub fn IOHIDSetModifierLockState(
        handle: io_connect_t,
        selector: ::std::os::raw::c_int,
        state: bool,
    ) -> kern_return_t;
}
unsafe extern "C" {
    pub fn IOHIDRegisterVirtualDisplay(
        handle: io_connect_t,
        display_token: *mut UInt32,
    ) -> kern_return_t;
}
unsafe extern "C" {
    pub fn IOHIDUnregisterVirtualDisplay(
        handle: io_connect_t,
        display_token: UInt32,
    ) -> kern_return_t;
}
unsafe extern "C" {
    pub fn IOHIDSetVirtualDisplayBounds(
        handle: io_connect_t,
        display_token: UInt32,
        bounds: *const IOGBounds,
    ) -> kern_return_t;
}
unsafe extern "C" {
    pub fn IOHIDGetActivityState(handle: io_connect_t, hidActivityIdle: *mut bool)
        -> kern_return_t;
}
unsafe extern "C" {
    pub fn IOHIDCheckAccess(requestType: IOHIDRequestType) -> IOHIDAccessType;
}
unsafe extern "C" {
    pub fn IOHIDRequestAccess(requestType: IOHIDRequestType) -> bool;
}
unsafe extern "C" {
    pub fn IOHIDAccessCheckAuditToken(requestType: IOHIDRequestType, token: audit_token_t) -> bool;
}
unsafe extern "C" {
    pub fn NXOpenEventStatus() -> NXEventHandle;
}
unsafe extern "C" {
    pub fn NXCloseEventStatus(handle: NXEventHandle);
}
unsafe extern "C" {
    pub fn NXEventSystemInfo(
        handle: NXEventHandle,
        flavor: *mut ::std::os::raw::c_char,
        evs_info: *mut ::std::os::raw::c_int,
        evs_info_cnt: *mut ::std::os::raw::c_uint,
    ) -> NXEventSystemInfoType;
}
unsafe extern "C" {
    pub fn NXSetKeyRepeatInterval(handle: NXEventHandle, seconds: f64);
}
unsafe extern "C" {
    pub fn NXKeyRepeatInterval(handle: NXEventHandle) -> f64;
}
unsafe extern "C" {
    pub fn NXSetKeyRepeatThreshold(handle: NXEventHandle, threshold: f64);
}
unsafe extern "C" {
    pub fn NXKeyRepeatThreshold(handle: NXEventHandle) -> f64;
}
unsafe extern "C" {
    pub fn NXResetKeyboard(handle: NXEventHandle);
}
unsafe extern "C" {
    pub fn NXSetClickTime(handle: NXEventHandle, seconds: f64);
}
unsafe extern "C" {
    pub fn NXClickTime(handle: NXEventHandle) -> f64;
}
unsafe extern "C" {
    pub fn NXSetClickSpace(handle: NXEventHandle, area: *mut NXSize);
}
unsafe extern "C" {
    pub fn NXGetClickSpace(handle: NXEventHandle, area: *mut NXSize);
}
unsafe extern "C" {
    pub fn NXResetMouse(handle: NXEventHandle);
}
unsafe extern "C" {
    pub fn IOHIDEventSystemClientCreateSimpleClient(
        allocator: CFAllocatorRef,
    ) -> IOHIDEventSystemClientRef;
}
unsafe extern "C" {
    pub fn IOHIDEventSystemClientSetProperty(
        client: IOHIDEventSystemClientRef,
        key: CFStringRef,
        property: CFTypeRef,
    ) -> Boolean;
}
unsafe extern "C" {
    pub fn IOHIDEventSystemClientCopyProperty(
        client: IOHIDEventSystemClientRef,
        key: CFStringRef,
    ) -> CFTypeRef;
}
unsafe extern "C" {
    pub fn IOHIDEventSystemClientGetTypeID() -> CFTypeID;
}
unsafe extern "C" {
    pub fn IOHIDEventSystemClientCopyServices(client: IOHIDEventSystemClientRef) -> CFArrayRef;
}
unsafe extern "C" {
    pub fn IOHIDServiceClientSetProperty(
        service: IOHIDServiceClientRef,
        key: CFStringRef,
        property: CFTypeRef,
    ) -> Boolean;
}
unsafe extern "C" {
    pub fn IOHIDServiceClientCopyProperty(
        service: IOHIDServiceClientRef,
        key: CFStringRef,
    ) -> CFTypeRef;
}
unsafe extern "C" {
    pub fn IOHIDServiceClientGetTypeID() -> CFTypeID;
}
unsafe extern "C" {
    pub fn IOHIDServiceClientGetRegistryID(service: IOHIDServiceClientRef) -> CFTypeRef;
}
unsafe extern "C" {
    pub fn IOHIDServiceClientConformsTo(
        service: IOHIDServiceClientRef,
        usagePage: u32,
        usage: u32,
    ) -> boolean_t;
}
unsafe extern "C" {
    pub fn IOHIDUserDeviceGetTypeID() -> CFTypeID;
}
unsafe extern "C" {
    pub fn IOHIDUserDeviceCreateWithProperties(
        allocator: CFAllocatorRef,
        properties: CFDictionaryRef,
        options: IOOptionBits,
    ) -> IOHIDUserDeviceRef;
}
unsafe extern "C" {
    pub fn IOHIDUserDeviceRegisterGetReportBlock(
        device: IOHIDUserDeviceRef,
        block: IOHIDUserDeviceGetReportBlock,
    );
}
unsafe extern "C" {
    pub fn IOHIDUserDeviceRegisterSetReportBlock(
        device: IOHIDUserDeviceRef,
        block: IOHIDUserDeviceSetReportBlock,
    );
}
unsafe extern "C" {
    pub fn IOHIDUserDeviceSetDispatchQueue(device: IOHIDUserDeviceRef, queue: NSObject);
}
unsafe extern "C" {
    pub fn IOHIDUserDeviceSetCancelHandler(device: IOHIDUserDeviceRef, handler: dispatch_block_t);
}
unsafe extern "C" {
    pub fn IOHIDUserDeviceActivate(device: IOHIDUserDeviceRef);
}
unsafe extern "C" {
    pub fn IOHIDUserDeviceCancel(device: IOHIDUserDeviceRef);
}
unsafe extern "C" {
    pub fn IOHIDUserDeviceCopyProperty(device: IOHIDUserDeviceRef, key: CFStringRef) -> CFTypeRef;
}
unsafe extern "C" {
    pub fn IOHIDUserDeviceSetProperty(
        device: IOHIDUserDeviceRef,
        key: CFStringRef,
        property: CFTypeRef,
    ) -> Boolean;
}
unsafe extern "C" {
    pub fn IOHIDUserDeviceHandleReportWithTimeStamp(
        device: IOHIDUserDeviceRef,
        timestamp: u64,
        report: *const u8,
        reportLength: CFIndex,
    ) -> IOReturn;
}
unsafe extern "C" {
    pub fn IOFBGetI2CInterfaceCount(framebuffer: io_service_t, count: *mut IOItemCount)
        -> IOReturn;
}
unsafe extern "C" {
    pub fn IOFBCopyI2CInterfaceForBus(
        framebuffer: io_service_t,
        bus: IOOptionBits,
        interface: *mut io_service_t,
    ) -> IOReturn;
}
unsafe extern "C" {
    pub fn IOI2CCopyInterfaceForID(identifier: CFTypeRef, interface: *mut io_service_t)
        -> IOReturn;
}
unsafe extern "C" {
    pub fn IOI2CInterfaceOpen(
        interface: io_service_t,
        options: IOOptionBits,
        connect: *mut IOI2CConnectRef,
    ) -> IOReturn;
}
unsafe extern "C" {
    pub fn IOI2CInterfaceClose(connect: IOI2CConnectRef, options: IOOptionBits) -> IOReturn;
}
unsafe extern "C" {
    pub fn IOI2CSendRequest(
        connect: IOI2CConnectRef,
        options: IOOptionBits,
        request: *mut IOI2CRequest,
    ) -> IOReturn;
}
unsafe extern "C" {
    pub fn KextManagerCreateURLForBundleIdentifier(
        allocator: CFAllocatorRef,
        kextIdentifier: CFStringRef,
    ) -> CFURLRef;
}
unsafe extern "C" {
    pub fn KextManagerLoadKextWithIdentifier(
        kextIdentifier: CFStringRef,
        dependencyKextAndFolderURLs: CFArrayRef,
    ) -> OSReturn;
}
unsafe extern "C" {
    pub fn KextManagerLoadKextWithURL(
        kextURL: CFURLRef,
        dependencyKextAndFolderURLs: CFArrayRef,
    ) -> OSReturn;
}
unsafe extern "C" {
    pub fn KextManagerUnloadKextWithIdentifier(kextIdentifier: CFStringRef) -> OSReturn;
}
unsafe extern "C" {
    pub fn KextManagerCopyLoadedKextInfo(
        kextIdentifiers: CFArrayRef,
        infoKeys: CFArrayRef,
    ) -> CFDictionaryRef;
}
unsafe extern "C" {
    pub fn IONetworkOpen(obj: io_object_t, con: *mut io_connect_t) -> IOReturn;
}
unsafe extern "C" {
    pub fn IONetworkClose(con: io_connect_t) -> IOReturn;
}
unsafe extern "C" {
    pub fn IONetworkWriteData(
        conObj: io_connect_t,
        dataHandle: IONDHandle,
        srcBuf: *mut UInt8,
        inSize: UInt32,
    ) -> IOReturn;
}
unsafe extern "C" {
    pub fn IONetworkReadData(
        conObj: io_connect_t,
        dataHandle: IONDHandle,
        destBuf: *mut UInt8,
        inOutSizeP: *mut UInt32,
    ) -> IOReturn;
}
unsafe extern "C" {
    pub fn IONetworkResetData(conObject: io_connect_t, dataHandle: IONDHandle) -> IOReturn;
}
unsafe extern "C" {
    pub fn IONetworkGetDataCapacity(
        conObject: io_connect_t,
        dataHandle: IONDHandle,
        capacityP: *mut UInt32,
    ) -> IOReturn;
}
unsafe extern "C" {
    pub fn IONetworkGetDataHandle(
        conObject: io_connect_t,
        dataName: *const ::std::os::raw::c_char,
        dataHandleP: *mut IONDHandle,
    ) -> IOReturn;
}
unsafe extern "C" {
    pub fn IONetworkSetPacketFiltersMask(
        connect: io_connect_t,
        filterGroup: *const ::std::os::raw::c_char,
        filtersMask: UInt32,
        options: IOOptionBits,
    ) -> IOReturn;
}
unsafe extern "C" {
    pub fn IONetworkGetPacketFiltersMask(
        connect: io_connect_t,
        filterGroup: *const ::std::os::raw::c_char,
        filtersMask: *mut UInt32,
        options: IOOptionBits,
    ) -> IOReturn;
}
unsafe extern "C" {
    pub fn IOPSGetBatteryWarningLevel() -> IOPSLowBatteryWarningLevel;
}
unsafe extern "C" {
    pub fn IOPSGetTimeRemainingEstimate() -> CFTimeInterval;
}
unsafe extern "C" {
    pub fn IOPSCopyPowerSourcesInfo() -> CFTypeRef;
}
unsafe extern "C" {
    pub fn IOPSCopyPowerSourcesList(blob: CFTypeRef) -> CFArrayRef;
}
unsafe extern "C" {
    pub fn IOPSGetPowerSourceDescription(blob: CFTypeRef, ps: CFTypeRef) -> CFDictionaryRef;
}
unsafe extern "C" {
    pub fn IOPSGetProvidingPowerSourceType(snapshot: CFTypeRef) -> CFStringRef;
}
unsafe extern "C" {
    pub fn IOPSNotificationCreateRunLoopSource(
        callback: IOPowerSourceCallbackType,
        context: *mut ::std::os::raw::c_void,
    ) -> CFRunLoopSourceRef;
}
unsafe extern "C" {
    pub fn IOPSCreateLimitedPowerNotification(
        callback: IOPowerSourceCallbackType,
        context: *mut ::std::os::raw::c_void,
    ) -> CFRunLoopSourceRef;
}
unsafe extern "C" {
    pub fn IOPSCopyExternalPowerAdapterDetails() -> CFDictionaryRef;
}
unsafe extern "C" {
    pub fn IOPMFindPowerManagement(master_device_port: mach_port_t) -> io_connect_t;
}
unsafe extern "C" {
    pub fn IOPMSetAggressiveness(
        fb: io_connect_t,
        type_: ::std::os::raw::c_ulong,
        aggressiveness: ::std::os::raw::c_ulong,
    ) -> IOReturn;
}
unsafe extern "C" {
    pub fn IOPMGetAggressiveness(
        fb: io_connect_t,
        type_: ::std::os::raw::c_ulong,
        aggressiveness: *mut ::std::os::raw::c_ulong,
    ) -> IOReturn;
}
unsafe extern "C" {
    pub fn IOPMSleepEnabled() -> boolean_t;
}
unsafe extern "C" {
    pub fn IOPMSleepSystem(fb: io_connect_t) -> IOReturn;
}
unsafe extern "C" {
    pub fn IOPMCopyBatteryInfo(masterPort: mach_port_t, info: *mut CFArrayRef) -> IOReturn;
}
unsafe extern "C" {
    pub fn IORegisterApp(
        refcon: *mut ::std::os::raw::c_void,
        theDriver: io_service_t,
        thePortRef: *mut IONotificationPortRef,
        callback: IOServiceInterestCallback,
        notifier: *mut io_object_t,
    ) -> io_connect_t;
}
unsafe extern "C" {
    pub fn IORegisterForSystemPower(
        refcon: *mut ::std::os::raw::c_void,
        thePortRef: *mut IONotificationPortRef,
        callback: IOServiceInterestCallback,
        notifier: *mut io_object_t,
    ) -> io_connect_t;
}
unsafe extern "C" {
    pub fn IODeregisterApp(notifier: *mut io_object_t) -> IOReturn;
}
unsafe extern "C" {
    pub fn IODeregisterForSystemPower(notifier: *mut io_object_t) -> IOReturn;
}
unsafe extern "C" {
    pub fn IOAllowPowerChange(kernelPort: io_connect_t, notificationID: isize) -> IOReturn;
}
unsafe extern "C" {
    pub fn IOCancelPowerChange(kernelPort: io_connect_t, notificationID: isize) -> IOReturn;
}
unsafe extern "C" {
    pub fn IOPMSchedulePowerEvent(
        time_to_wake: CFDateRef,
        my_id: CFStringRef,
        type_: CFStringRef,
    ) -> IOReturn;
}
unsafe extern "C" {
    pub fn IOPMCancelScheduledPowerEvent(
        time_to_wake: CFDateRef,
        my_id: CFStringRef,
        type_: CFStringRef,
    ) -> IOReturn;
}
unsafe extern "C" {
    pub fn IOPMCopyScheduledPowerEvents() -> CFArrayRef;
}
unsafe extern "C" {
    pub fn IOPMAssertionCreateWithDescription(
        AssertionType: CFStringRef,
        Name: CFStringRef,
        Details: CFStringRef,
        HumanReadableReason: CFStringRef,
        LocalizationBundlePath: CFStringRef,
        Timeout: CFTimeInterval,
        TimeoutAction: CFStringRef,
        AssertionID: *mut IOPMAssertionID,
    ) -> IOReturn;
}
unsafe extern "C" {
    pub fn IOPMAssertionCreateWithProperties(
        AssertionProperties: CFDictionaryRef,
        AssertionID: *mut IOPMAssertionID,
    ) -> IOReturn;
}
unsafe extern "C" {
    pub fn IOPMAssertionDeclareUserActivity(
        AssertionName: CFStringRef,
        userType: IOPMUserActiveType,
        AssertionID: *mut IOPMAssertionID,
    ) -> IOReturn;
}
unsafe extern "C" {
    pub fn IOPMDeclareNetworkClientActivity(
        AssertionName: CFStringRef,
        AssertionID: *mut IOPMAssertionID,
    ) -> IOReturn;
}
unsafe extern "C" {
    pub fn IOPMAssertionRetain(theAssertion: IOPMAssertionID);
}
unsafe extern "C" {
    pub fn IOPMAssertionRelease(AssertionID: IOPMAssertionID) -> IOReturn;
}
unsafe extern "C" {
    pub fn IOPMAssertionCopyProperties(theAssertion: IOPMAssertionID) -> CFDictionaryRef;
}
unsafe extern "C" {
    pub fn IOPMAssertionSetProperty(
        theAssertion: IOPMAssertionID,
        theProperty: CFStringRef,
        theValue: CFTypeRef,
    ) -> IOReturn;
}
unsafe extern "C" {
    pub fn IOPMCopyAssertionsByProcess(AssertionsByPID: *mut CFDictionaryRef) -> IOReturn;
}
unsafe extern "C" {
    pub fn IOPMCopyAssertionsStatus(AssertionsStatus: *mut CFDictionaryRef) -> IOReturn;
}
unsafe extern "C" {
    pub fn IOPMAssertionCreate(
        AssertionType: CFStringRef,
        AssertionLevel: IOPMAssertionLevel,
        AssertionID: *mut IOPMAssertionID,
    ) -> IOReturn;
}
unsafe extern "C" {
    pub fn IOPMAssertionCreateWithName(
        AssertionType: CFStringRef,
        AssertionLevel: IOPMAssertionLevel,
        AssertionName: CFStringRef,
        AssertionID: *mut IOPMAssertionID,
    ) -> IOReturn;
}
unsafe extern "C" {
    pub fn IOGetSystemLoadAdvisory() -> IOSystemLoadAdvisoryLevel;
}
unsafe extern "C" {
    pub fn IOCopySystemLoadAdvisoryDetailed() -> CFDictionaryRef;
}
unsafe extern "C" {
    pub fn IOPMCopyCPUPowerStatus(cpuPowerStatus: *mut CFDictionaryRef) -> IOReturn;
}
unsafe extern "C" {
    pub fn IOPMGetThermalWarningLevel(thermalLevel: *mut u32) -> IOReturn;
}

unsafe impl objc2::encode::RefEncode for mach_msg_port_descriptor_t {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for mach_msg_port_descriptor_t {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("mach_msg_port_descriptor_t", &[]);
}
unsafe impl objc2::encode::RefEncode for mach_msg_body_t {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for mach_msg_body_t {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("mach_msg_body_t", &[]);
}
unsafe impl objc2::encode::RefEncode for mach_msg_header_t {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for mach_msg_header_t {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("mach_msg_header_t", &[]);
}
unsafe impl objc2::encode::RefEncode for audit_token_t {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for audit_token_t {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("audit_token_t", &[]);
}
unsafe impl objc2::encode::RefEncode for mach_timespec {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for mach_timespec {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("mach_timespec", &[]);
}
unsafe impl objc2::encode::RefEncode for IOPhysicalRange {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for IOPhysicalRange {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("IOPhysicalRange", &[]);
}
unsafe impl objc2::encode::RefEncode for IOVirtualRange {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for IOVirtualRange {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("IOVirtualRange", &[]);
}
unsafe impl objc2::encode::RefEncode for IONamedValue {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for IONamedValue {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("IONamedValue", &[]);
}
unsafe impl objc2::encode::RefEncode for OSNotificationHeader {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for OSNotificationHeader {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("OSNotificationHeader", &[]);
}
unsafe impl objc2::encode::RefEncode for IOServiceInterestContent {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for IOServiceInterestContent {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("IOServiceInterestContent", &[]);
}
unsafe impl objc2::encode::RefEncode for IOAsyncCompletionContent {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for IOAsyncCompletionContent {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("IOAsyncCompletionContent", &[]);
}
unsafe impl objc2::encode::RefEncode for IONotificationPort {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for IONotificationPort {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("IONotificationPort", &[]);
}
unsafe impl objc2::encode::RefEncode for _IODataQueueEntry {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for _IODataQueueEntry {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("_IODataQueueEntry", &[]);
}
unsafe impl objc2::encode::RefEncode for _IODataQueueMemory {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for _IODataQueueMemory {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("_IODataQueueMemory", &[]);
}
unsafe impl objc2::encode::RefEncode for _IODataQueueAppendix {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for _IODataQueueAppendix {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("_IODataQueueAppendix", &[]);
}
unsafe impl objc2::encode::RefEncode for IOCFPlugInInterfaceStruct {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for IOCFPlugInInterfaceStruct {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("IOCFPlugInInterfaceStruct", &[]);
}
unsafe impl objc2::encode::RefEncode for IORPCMessageMach {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for IORPCMessageMach {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("IORPCMessageMach", &[]);
}
unsafe impl objc2::encode::RefEncode for IORPCMessage {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for IORPCMessage {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("IORPCMessage", &[]);
}
unsafe impl objc2::encode::RefEncode for IORPCMessageErrorReturnContent {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for IORPCMessageErrorReturnContent {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("IORPCMessageErrorReturnContent", &[]);
}
unsafe impl objc2::encode::RefEncode for IORPC {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for IORPC {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("IORPC", &[]);
}
unsafe impl objc2::encode::RefEncode for IOGPoint {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for IOGPoint {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("IOGPoint", &[]);
}
unsafe impl objc2::encode::RefEncode for IOGBounds {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for IOGBounds {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("IOGBounds", &[]);
}
unsafe impl objc2::encode::RefEncode for _NXSize {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for _NXSize {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("_NXSize", &[]);
}
unsafe impl objc2::encode::RefEncode for _NXTabletPointData {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for _NXTabletPointData {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("_NXTabletPointData", &[]);
}
unsafe impl objc2::encode::RefEncode for _NXTabletPointData__bindgen_ty_1 {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for _NXTabletPointData__bindgen_ty_1 {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("_NXTabletPointData__bindgen_ty_1", &[]);
}
unsafe impl objc2::encode::RefEncode for _NXTabletProximityData {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for _NXTabletProximityData {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("_NXTabletProximityData", &[]);
}
unsafe impl objc2::encode::RefEncode for NXEventData {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for NXEventData {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("NXEventData", &[]);
}
unsafe impl objc2::encode::RefEncode for NXEventData__bindgen_ty_1 {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for NXEventData__bindgen_ty_1 {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("NXEventData__bindgen_ty_1", &[]);
}
unsafe impl objc2::encode::RefEncode for NXEventData__bindgen_ty_1__bindgen_ty_1 {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for NXEventData__bindgen_ty_1__bindgen_ty_1 {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("NXEventData__bindgen_ty_1__bindgen_ty_1", &[]);
}
unsafe impl objc2::encode::RefEncode for NXEventData__bindgen_ty_2 {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for NXEventData__bindgen_ty_2 {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("NXEventData__bindgen_ty_2", &[]);
}
unsafe impl objc2::encode::RefEncode for NXEventData__bindgen_ty_2__bindgen_ty_1 {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for NXEventData__bindgen_ty_2__bindgen_ty_1 {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("NXEventData__bindgen_ty_2__bindgen_ty_1", &[]);
}
unsafe impl objc2::encode::RefEncode for NXEventData__bindgen_ty_3 {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for NXEventData__bindgen_ty_3 {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("NXEventData__bindgen_ty_3", &[]);
}
unsafe impl objc2::encode::RefEncode for NXEventData__bindgen_ty_4 {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for NXEventData__bindgen_ty_4 {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("NXEventData__bindgen_ty_4", &[]);
}
unsafe impl objc2::encode::RefEncode for NXEventData__bindgen_ty_5 {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for NXEventData__bindgen_ty_5 {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("NXEventData__bindgen_ty_5", &[]);
}
unsafe impl objc2::encode::RefEncode for NXEventData__bindgen_ty_6 {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for NXEventData__bindgen_ty_6 {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("NXEventData__bindgen_ty_6", &[]);
}
unsafe impl objc2::encode::RefEncode for NXEventData__bindgen_ty_6__bindgen_ty_1 {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for NXEventData__bindgen_ty_6__bindgen_ty_1 {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("NXEventData__bindgen_ty_6__bindgen_ty_1", &[]);
}
unsafe impl objc2::encode::RefEncode for NXEventData__bindgen_ty_7 {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for NXEventData__bindgen_ty_7 {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("NXEventData__bindgen_ty_7", &[]);
}
unsafe impl objc2::encode::RefEncode for NXEventData__bindgen_ty_7__bindgen_ty_1 {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for NXEventData__bindgen_ty_7__bindgen_ty_1 {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("NXEventData__bindgen_ty_7__bindgen_ty_1", &[]);
}
unsafe impl objc2::encode::RefEncode for NXEventData__bindgen_ty_8 {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for NXEventData__bindgen_ty_8 {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("NXEventData__bindgen_ty_8", &[]);
}
unsafe impl objc2::encode::RefEncode for __IOHIDDevice {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for __IOHIDDevice {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("__IOHIDDevice", &[]);
}
unsafe impl objc2::encode::RefEncode for __IOHIDElement {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for __IOHIDElement {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("__IOHIDElement", &[]);
}
unsafe impl objc2::encode::RefEncode for __IOHIDValue {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for __IOHIDValue {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("__IOHIDValue", &[]);
}
unsafe impl objc2::encode::RefEncode for __IOHIDQueue {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for __IOHIDQueue {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("__IOHIDQueue", &[]);
}
unsafe impl objc2::encode::RefEncode for __IOHIDManager {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for __IOHIDManager {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("__IOHIDManager", &[]);
}
unsafe impl objc2::encode::RefEncode for __IOHIDTransaction {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for __IOHIDTransaction {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("__IOHIDTransaction", &[]);
}
unsafe impl objc2::encode::RefEncode for __IOHIDEventSystemClient {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for __IOHIDEventSystemClient {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("__IOHIDEventSystemClient", &[]);
}
unsafe impl objc2::encode::RefEncode for __IOHIDServiceClient {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for __IOHIDServiceClient {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("__IOHIDServiceClient", &[]);
}
unsafe impl objc2::encode::RefEncode for __IOHIDUserDevice {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for __IOHIDUserDevice {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("__IOHIDUserDevice", &[]);
}
unsafe impl objc2::encode::RefEncode for IOI2CRequest {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for IOI2CRequest {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("IOI2CRequest", &[]);
}
unsafe impl objc2::encode::RefEncode for IOI2CConnect {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for IOI2CConnect {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("IOI2CConnect", &[]);
}
