#[allow(unused_imports)]
use crate::objc::*;
#[allow(unused_imports)]
use crate::CoreFoundation::*;

#[allow(unused_imports)]
use objc2::msg_send;
pub type netfsError = SInt32;
pub type AsyncRequestID = *mut ::std::os::raw::c_void;
pub type NetFSMountURLBlock = *mut ::std::os::raw::c_void;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct NetFSMountInterface_V1 {
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
    pub CreateSessionRef: ::std::option::Option<
        unsafe extern "C" fn(out_SessionRef: *mut *mut ::std::os::raw::c_void) -> netfsError,
    >,
    pub GetServerInfo: ::std::option::Option<
        unsafe extern "C" fn(
            in_URL: CFURLRef,
            in_SessionRef: *mut ::std::os::raw::c_void,
            in_GetInfoOptions: CFDictionaryRef,
            out_ServerParms: *mut CFDictionaryRef,
        ) -> netfsError,
    >,
    pub ParseURL: ::std::option::Option<
        unsafe extern "C" fn(in_URL: CFURLRef, out_URLParms: *mut CFDictionaryRef) -> netfsError,
    >,
    pub CreateURL: ::std::option::Option<
        unsafe extern "C" fn(in_URLParms: CFDictionaryRef, out_URL: *mut CFURLRef) -> netfsError,
    >,
    pub OpenSession: ::std::option::Option<
        unsafe extern "C" fn(
            in_URL: CFURLRef,
            in_SessionRef: *mut ::std::os::raw::c_void,
            in_OpenOptions: CFDictionaryRef,
            out_SessionInfo: *mut CFDictionaryRef,
        ) -> netfsError,
    >,
    pub EnumerateShares: ::std::option::Option<
        unsafe extern "C" fn(
            in_SessionRef: *mut ::std::os::raw::c_void,
            in_EnumerateOptions: CFDictionaryRef,
            out_Sharepoints: *mut CFDictionaryRef,
        ) -> netfsError,
    >,
    pub Mount: ::std::option::Option<
        unsafe extern "C" fn(
            in_SessionRef: *mut ::std::os::raw::c_void,
            in_URL: CFURLRef,
            in_Mountpoint: CFStringRef,
            in_MountOptions: CFDictionaryRef,
            out_MountInfo: *mut CFDictionaryRef,
        ) -> netfsError,
    >,
    pub Cancel: ::std::option::Option<
        unsafe extern "C" fn(in_SessionRef: *mut ::std::os::raw::c_void) -> netfsError,
    >,
    pub CloseSession: ::std::option::Option<
        unsafe extern "C" fn(in_SessionRef: *mut ::std::os::raw::c_void) -> netfsError,
    >,
    pub GetMountInfo: ::std::option::Option<
        unsafe extern "C" fn(
            in_mountpoint: CFStringRef,
            out_MountInfo: *mut CFDictionaryRef,
        ) -> netfsError,
    >,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct NetFSInterface {
    pub _interface: *mut ::std::os::raw::c_void,
    pub _factoryID: CFUUIDRef,
    pub _refCount: UInt32,
}
unsafe extern "C" {
    pub fn NetFSMountURLSync(
        url: CFURLRef,
        mountpath: CFURLRef,
        user: CFStringRef,
        passwd: CFStringRef,
        open_options: CFMutableDictionaryRef,
        mount_options: CFMutableDictionaryRef,
        mountpoints: *mut CFArrayRef,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn NetFSMountURLAsync(
        url: CFURLRef,
        mountpath: CFURLRef,
        user: CFStringRef,
        passwd: CFStringRef,
        open_options: CFMutableDictionaryRef,
        mount_options: CFMutableDictionaryRef,
        requestID: *mut AsyncRequestID,
        dispatchq: NSObject,
        mount_report: NetFSMountURLBlock,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn NetFSMountURLCancel(requestID: AsyncRequestID) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn NetFSMountURLProbe(hostname: CFStringRef) -> CFStringRef;
}
unsafe extern "C" {
    pub fn NetFSCopyURLForRemountingVolume(localPathURL: CFURLRef) -> CFURLRef;
}
unsafe extern "C" {
    pub fn NetFSInterface_AddRef(arg1: *mut ::std::os::raw::c_void) -> ULONG;
}
unsafe extern "C" {
    pub fn NetFSInterface_Release(arg1: *mut ::std::os::raw::c_void) -> ULONG;
}
unsafe extern "C" {
    pub fn NetFS_CreateInterface(
        factoryID: CFUUIDRef,
        interfaceFTbl: *mut ::std::os::raw::c_void,
    ) -> *mut NetFSInterface;
}
unsafe extern "C" {
    pub fn NetFSQueryInterface(
        arg1: *mut ::std::os::raw::c_void,
        iid: REFIID,
        ppv: *mut LPVOID,
    ) -> HRESULT;
}
unsafe extern "C" {
    pub fn NetFSCFStringtoCString(arg1: CFStringRef) -> *mut ::std::os::raw::c_char;
}

unsafe impl objc2::encode::RefEncode for NetFSMountInterface_V1 {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for NetFSMountInterface_V1 {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("NetFSMountInterface_V1", &[]);
}
unsafe impl objc2::encode::RefEncode for NetFSInterface {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for NetFSInterface {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("NetFSInterface", &[]);
}
