#[allow(unused_imports)]
use crate::objc::*;
#[allow(unused_imports)]
use crate::CoreFoundation::*;
#[allow(unused_imports)]
use crate::IOKit::*;

#[allow(unused_imports)]
use objc2::msg_send;
pub type mach_error_t = kern_return_t;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __DASession {
    _unused: [u8; 0],
}
pub type DASessionRef = *mut __DASession;
pub type DAApprovalSessionRef = *mut __DASession;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __DADisk {
    _unused: [u8; 0],
}
pub type DADiskRef = *mut __DADisk;
pub type DAReturn = mach_error_t;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __DADissenter {
    _unused: [u8; 0],
}
pub type DADissenterRef = *const __DADissenter;
pub type DADiskMountOptions = UInt32;
pub type DADiskRenameOptions = UInt32;
pub type DADiskUnmountOptions = UInt32;
pub type DADiskEjectOptions = UInt32;
pub type DADiskClaimOptions = UInt32;
pub type DADiskOptions = UInt32;
pub type DADiskAppearedCallback = ::std::option::Option<
    unsafe extern "C" fn(disk: DADiskRef, context: *mut ::std::os::raw::c_void),
>;
pub type DADiskDescriptionChangedCallback = ::std::option::Option<
    unsafe extern "C" fn(disk: DADiskRef, keys: CFArrayRef, context: *mut ::std::os::raw::c_void),
>;
pub type DADiskDisappearedCallback = ::std::option::Option<
    unsafe extern "C" fn(disk: DADiskRef, context: *mut ::std::os::raw::c_void),
>;
pub type DADiskMountCallback = ::std::option::Option<
    unsafe extern "C" fn(
        disk: DADiskRef,
        dissenter: DADissenterRef,
        context: *mut ::std::os::raw::c_void,
    ),
>;
pub type DADiskMountApprovalCallback = ::std::option::Option<
    unsafe extern "C" fn(disk: DADiskRef, context: *mut ::std::os::raw::c_void) -> DADissenterRef,
>;
pub type DADiskRenameCallback = ::std::option::Option<
    unsafe extern "C" fn(
        disk: DADiskRef,
        dissenter: DADissenterRef,
        context: *mut ::std::os::raw::c_void,
    ),
>;
pub type DADiskUnmountCallback = ::std::option::Option<
    unsafe extern "C" fn(
        disk: DADiskRef,
        dissenter: DADissenterRef,
        context: *mut ::std::os::raw::c_void,
    ),
>;
pub type DADiskUnmountApprovalCallback = ::std::option::Option<
    unsafe extern "C" fn(disk: DADiskRef, context: *mut ::std::os::raw::c_void) -> DADissenterRef,
>;
pub type DADiskEjectCallback = ::std::option::Option<
    unsafe extern "C" fn(
        disk: DADiskRef,
        dissenter: DADissenterRef,
        context: *mut ::std::os::raw::c_void,
    ),
>;
pub type DADiskEjectApprovalCallback = ::std::option::Option<
    unsafe extern "C" fn(disk: DADiskRef, context: *mut ::std::os::raw::c_void) -> DADissenterRef,
>;
pub type DADiskClaimCallback = ::std::option::Option<
    unsafe extern "C" fn(
        disk: DADiskRef,
        dissenter: DADissenterRef,
        context: *mut ::std::os::raw::c_void,
    ),
>;
pub type DADiskClaimReleaseCallback = ::std::option::Option<
    unsafe extern "C" fn(disk: DADiskRef, context: *mut ::std::os::raw::c_void) -> DADissenterRef,
>;
pub type DADiskPeekCallback = ::std::option::Option<
    unsafe extern "C" fn(disk: DADiskRef, context: *mut ::std::os::raw::c_void),
>;
unsafe extern "C" {
    pub fn DASessionGetTypeID() -> CFTypeID;
}
unsafe extern "C" {
    pub fn DASessionCreate(allocator: CFAllocatorRef) -> DASessionRef;
}
unsafe extern "C" {
    pub fn DASessionScheduleWithRunLoop(
        session: DASessionRef,
        runLoop: CFRunLoopRef,
        runLoopMode: CFStringRef,
    );
}
unsafe extern "C" {
    pub fn DASessionUnscheduleFromRunLoop(
        session: DASessionRef,
        runLoop: CFRunLoopRef,
        runLoopMode: CFStringRef,
    );
}
unsafe extern "C" {
    pub fn DASessionSetDispatchQueue(session: DASessionRef, queue: NSObject);
}
unsafe extern "C" {
    pub fn DAApprovalSessionGetTypeID() -> CFTypeID;
}
unsafe extern "C" {
    pub fn DAApprovalSessionCreate(allocator: CFAllocatorRef) -> DAApprovalSessionRef;
}
unsafe extern "C" {
    pub fn DAApprovalSessionScheduleWithRunLoop(
        session: DAApprovalSessionRef,
        runLoop: CFRunLoopRef,
        runLoopMode: CFStringRef,
    );
}
unsafe extern "C" {
    pub fn DAApprovalSessionUnscheduleFromRunLoop(
        session: DAApprovalSessionRef,
        runLoop: CFRunLoopRef,
        runLoopMode: CFStringRef,
    );
}
unsafe extern "C" {
    pub static kDADiskDescriptionVolumeKindKey: CFStringRef;
}
unsafe extern "C" {
    pub static kDADiskDescriptionVolumeMountableKey: CFStringRef;
}
unsafe extern "C" {
    pub static kDADiskDescriptionVolumeNameKey: CFStringRef;
}
unsafe extern "C" {
    pub static kDADiskDescriptionVolumeNetworkKey: CFStringRef;
}
unsafe extern "C" {
    pub static kDADiskDescriptionVolumePathKey: CFStringRef;
}
unsafe extern "C" {
    pub static kDADiskDescriptionVolumeTypeKey: CFStringRef;
}
unsafe extern "C" {
    pub static kDADiskDescriptionVolumeUUIDKey: CFStringRef;
}
unsafe extern "C" {
    pub static kDADiskDescriptionMediaBlockSizeKey: CFStringRef;
}
unsafe extern "C" {
    pub static kDADiskDescriptionMediaBSDMajorKey: CFStringRef;
}
unsafe extern "C" {
    pub static kDADiskDescriptionMediaBSDMinorKey: CFStringRef;
}
unsafe extern "C" {
    pub static kDADiskDescriptionMediaBSDNameKey: CFStringRef;
}
unsafe extern "C" {
    pub static kDADiskDescriptionMediaBSDUnitKey: CFStringRef;
}
unsafe extern "C" {
    pub static kDADiskDescriptionMediaContentKey: CFStringRef;
}
unsafe extern "C" {
    pub static kDADiskDescriptionMediaEjectableKey: CFStringRef;
}
unsafe extern "C" {
    pub static kDADiskDescriptionMediaIconKey: CFStringRef;
}
unsafe extern "C" {
    pub static kDADiskDescriptionMediaKindKey: CFStringRef;
}
unsafe extern "C" {
    pub static kDADiskDescriptionMediaLeafKey: CFStringRef;
}
unsafe extern "C" {
    pub static kDADiskDescriptionMediaNameKey: CFStringRef;
}
unsafe extern "C" {
    pub static kDADiskDescriptionMediaPathKey: CFStringRef;
}
unsafe extern "C" {
    pub static kDADiskDescriptionMediaRemovableKey: CFStringRef;
}
unsafe extern "C" {
    pub static kDADiskDescriptionMediaSizeKey: CFStringRef;
}
unsafe extern "C" {
    pub static kDADiskDescriptionMediaTypeKey: CFStringRef;
}
unsafe extern "C" {
    pub static kDADiskDescriptionMediaUUIDKey: CFStringRef;
}
unsafe extern "C" {
    pub static kDADiskDescriptionMediaWholeKey: CFStringRef;
}
unsafe extern "C" {
    pub static kDADiskDescriptionMediaWritableKey: CFStringRef;
}
unsafe extern "C" {
    pub static kDADiskDescriptionMediaEncryptedKey: CFStringRef;
}
unsafe extern "C" {
    pub static kDADiskDescriptionMediaEncryptionDetailKey: CFStringRef;
}
unsafe extern "C" {
    pub static kDADiskDescriptionDeviceGUIDKey: CFStringRef;
}
unsafe extern "C" {
    pub static kDADiskDescriptionDeviceInternalKey: CFStringRef;
}
unsafe extern "C" {
    pub static kDADiskDescriptionDeviceModelKey: CFStringRef;
}
unsafe extern "C" {
    pub static kDADiskDescriptionDevicePathKey: CFStringRef;
}
unsafe extern "C" {
    pub static kDADiskDescriptionDeviceProtocolKey: CFStringRef;
}
unsafe extern "C" {
    pub static kDADiskDescriptionDeviceRevisionKey: CFStringRef;
}
unsafe extern "C" {
    pub static kDADiskDescriptionDeviceUnitKey: CFStringRef;
}
unsafe extern "C" {
    pub static kDADiskDescriptionDeviceVendorKey: CFStringRef;
}
unsafe extern "C" {
    pub static kDADiskDescriptionDeviceTDMLockedKey: CFStringRef;
}
unsafe extern "C" {
    pub static kDADiskDescriptionBusNameKey: CFStringRef;
}
unsafe extern "C" {
    pub static kDADiskDescriptionBusPathKey: CFStringRef;
}
unsafe extern "C" {
    pub static kDADiskDescriptionFSKitPrefix: CFStringRef;
}
unsafe extern "C" {
    pub static kDADiskDescriptionRepairRunningKey: CFStringRef;
}
unsafe extern "C" {
    pub fn DADiskGetTypeID() -> CFTypeID;
}
unsafe extern "C" {
    pub fn DADiskCreateFromBSDName(
        allocator: CFAllocatorRef,
        session: DASessionRef,
        name: *const ::std::os::raw::c_char,
    ) -> DADiskRef;
}
unsafe extern "C" {
    pub fn DADiskCreateFromIOMedia(
        allocator: CFAllocatorRef,
        session: DASessionRef,
        media: io_service_t,
    ) -> DADiskRef;
}
unsafe extern "C" {
    pub fn DADiskCreateFromVolumePath(
        allocator: CFAllocatorRef,
        session: DASessionRef,
        path: CFURLRef,
    ) -> DADiskRef;
}
unsafe extern "C" {
    pub fn DADiskGetBSDName(disk: DADiskRef) -> *const ::std::os::raw::c_char;
}
unsafe extern "C" {
    pub fn DADiskCopyIOMedia(disk: DADiskRef) -> io_service_t;
}
unsafe extern "C" {
    pub fn DADiskCopyDescription(disk: DADiskRef) -> CFDictionaryRef;
}
unsafe extern "C" {
    pub fn DADiskCopyWholeDisk(disk: DADiskRef) -> DADiskRef;
}
unsafe extern "C" {
    pub fn DADissenterCreate(
        allocator: CFAllocatorRef,
        status: DAReturn,
        string: CFStringRef,
    ) -> DADissenterRef;
}
unsafe extern "C" {
    pub fn DADissenterGetStatus(dissenter: DADissenterRef) -> DAReturn;
}
unsafe extern "C" {
    pub fn DADissenterGetStatusString(dissenter: DADissenterRef) -> CFStringRef;
}
unsafe extern "C" {
    pub static mut kDADiskDescriptionMatchMediaUnformatted: CFDictionaryRef;
}
unsafe extern "C" {
    pub static mut kDADiskDescriptionMatchMediaWhole: CFDictionaryRef;
}
unsafe extern "C" {
    pub static mut kDADiskDescriptionMatchVolumeMountable: CFDictionaryRef;
}
unsafe extern "C" {
    pub static mut kDADiskDescriptionMatchVolumeUnrecognized: CFDictionaryRef;
}
unsafe extern "C" {
    pub static mut kDADiskDescriptionWatchVolumeName: CFArrayRef;
}
unsafe extern "C" {
    pub static mut kDADiskDescriptionWatchVolumePath: CFArrayRef;
}
unsafe extern "C" {
    pub fn DARegisterDiskAppearedCallback(
        session: DASessionRef,
        match_: CFDictionaryRef,
        callback: DADiskAppearedCallback,
        context: *mut ::std::os::raw::c_void,
    );
}
unsafe extern "C" {
    pub fn DARegisterDiskDescriptionChangedCallback(
        session: DASessionRef,
        match_: CFDictionaryRef,
        watch: CFArrayRef,
        callback: DADiskDescriptionChangedCallback,
        context: *mut ::std::os::raw::c_void,
    );
}
unsafe extern "C" {
    pub fn DARegisterDiskDisappearedCallback(
        session: DASessionRef,
        match_: CFDictionaryRef,
        callback: DADiskDisappearedCallback,
        context: *mut ::std::os::raw::c_void,
    );
}
unsafe extern "C" {
    pub fn DADiskMount(
        disk: DADiskRef,
        path: CFURLRef,
        options: DADiskMountOptions,
        callback: DADiskMountCallback,
        context: *mut ::std::os::raw::c_void,
    );
}
unsafe extern "C" {
    pub fn DADiskMountWithArguments(
        disk: DADiskRef,
        path: CFURLRef,
        options: DADiskMountOptions,
        callback: DADiskMountCallback,
        context: *mut ::std::os::raw::c_void,
        arguments: *mut CFStringRef,
    );
}
unsafe extern "C" {
    pub fn DARegisterDiskMountApprovalCallback(
        session: DASessionRef,
        match_: CFDictionaryRef,
        callback: DADiskMountApprovalCallback,
        context: *mut ::std::os::raw::c_void,
    );
}
unsafe extern "C" {
    pub fn DADiskRename(
        disk: DADiskRef,
        name: CFStringRef,
        options: DADiskRenameOptions,
        callback: DADiskRenameCallback,
        context: *mut ::std::os::raw::c_void,
    );
}
unsafe extern "C" {
    pub fn DADiskUnmount(
        disk: DADiskRef,
        options: DADiskUnmountOptions,
        callback: DADiskUnmountCallback,
        context: *mut ::std::os::raw::c_void,
    );
}
unsafe extern "C" {
    pub fn DARegisterDiskUnmountApprovalCallback(
        session: DASessionRef,
        match_: CFDictionaryRef,
        callback: DADiskUnmountApprovalCallback,
        context: *mut ::std::os::raw::c_void,
    );
}
unsafe extern "C" {
    pub fn DADiskEject(
        disk: DADiskRef,
        options: DADiskEjectOptions,
        callback: DADiskEjectCallback,
        context: *mut ::std::os::raw::c_void,
    );
}
unsafe extern "C" {
    pub fn DARegisterDiskEjectApprovalCallback(
        session: DASessionRef,
        match_: CFDictionaryRef,
        callback: DADiskEjectApprovalCallback,
        context: *mut ::std::os::raw::c_void,
    );
}
unsafe extern "C" {
    pub fn DADiskClaim(
        disk: DADiskRef,
        options: DADiskClaimOptions,
        release: DADiskClaimReleaseCallback,
        releaseContext: *mut ::std::os::raw::c_void,
        callback: DADiskClaimCallback,
        callbackContext: *mut ::std::os::raw::c_void,
    );
}
unsafe extern "C" {
    pub fn DADiskIsClaimed(disk: DADiskRef) -> Boolean;
}
unsafe extern "C" {
    pub fn DADiskUnclaim(disk: DADiskRef);
}
unsafe extern "C" {
    pub fn DARegisterDiskPeekCallback(
        session: DASessionRef,
        match_: CFDictionaryRef,
        order: CFIndex,
        callback: DADiskPeekCallback,
        context: *mut ::std::os::raw::c_void,
    );
}
unsafe extern "C" {
    pub fn DADiskGetOptions(disk: DADiskRef) -> DADiskOptions;
}
unsafe extern "C" {
    pub fn DADiskSetOptions(disk: DADiskRef, options: DADiskOptions, value: Boolean) -> DAReturn;
}
unsafe extern "C" {
    pub fn DAUnregisterCallback(
        session: DASessionRef,
        callback: *mut ::std::os::raw::c_void,
        context: *mut ::std::os::raw::c_void,
    );
}
unsafe extern "C" {
    pub fn DAUnregisterApprovalCallback(
        session: DASessionRef,
        callback: *mut ::std::os::raw::c_void,
        context: *mut ::std::os::raw::c_void,
    );
}

unsafe impl objc2::encode::RefEncode for __DASession {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for __DASession {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("__DASession", &[]);
}
unsafe impl objc2::encode::RefEncode for __DADisk {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for __DADisk {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("__DADisk", &[]);
}
unsafe impl objc2::encode::RefEncode for __DADissenter {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for __DADissenter {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("__DADissenter", &[]);
}
