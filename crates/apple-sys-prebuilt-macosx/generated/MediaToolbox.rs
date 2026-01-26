#[allow(unused_imports)]
use crate::CoreAudio::*;
#[allow(unused_imports)]
use crate::CoreFoundation::*;
#[allow(unused_imports)]
use crate::CoreMedia::*;

#[allow(unused_imports)]
use objc2::msg_send;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct opaqueMTAudioProcessingTap {
    _unused: [u8; 0],
}
pub type MTAudioProcessingTapRef = *const opaqueMTAudioProcessingTap;
pub type MTAudioProcessingTapCreationFlags = u32;
pub type MTAudioProcessingTapFlags = u32;
pub type MTAudioProcessingTapInitCallback = ::std::option::Option<
    unsafe extern "C" fn(
        tap: MTAudioProcessingTapRef,
        clientInfo: *mut ::std::os::raw::c_void,
        tapStorageOut: *mut *mut ::std::os::raw::c_void,
    ),
>;
pub type MTAudioProcessingTapFinalizeCallback =
    ::std::option::Option<unsafe extern "C" fn(tap: MTAudioProcessingTapRef)>;
pub type MTAudioProcessingTapPrepareCallback = ::std::option::Option<
    unsafe extern "C" fn(
        tap: MTAudioProcessingTapRef,
        maxFrames: CMItemCount,
        processingFormat: *const AudioStreamBasicDescription,
    ),
>;
pub type MTAudioProcessingTapUnprepareCallback =
    ::std::option::Option<unsafe extern "C" fn(tap: MTAudioProcessingTapRef)>;
pub type MTAudioProcessingTapProcessCallback = ::std::option::Option<
    unsafe extern "C" fn(
        tap: MTAudioProcessingTapRef,
        numberFrames: CMItemCount,
        flags: MTAudioProcessingTapFlags,
        bufferListInOut: *mut AudioBufferList,
        numberFramesOut: *mut CMItemCount,
        flagsOut: *mut MTAudioProcessingTapFlags,
    ),
>;
#[repr(C, packed(4))]
#[derive(Debug, Copy, Clone)]
pub struct MTAudioProcessingTapCallbacks {
    pub version: ::std::os::raw::c_int,
    pub clientInfo: *mut ::std::os::raw::c_void,
    pub init: MTAudioProcessingTapInitCallback,
    pub finalize: MTAudioProcessingTapFinalizeCallback,
    pub prepare: MTAudioProcessingTapPrepareCallback,
    pub unprepare: MTAudioProcessingTapUnprepareCallback,
    pub process: MTAudioProcessingTapProcessCallback,
}
unsafe extern "C" {
    pub fn MTAudioProcessingTapGetTypeID() -> CFTypeID;
}
unsafe extern "C" {
    pub fn MTAudioProcessingTapCreate(
        allocator: CFAllocatorRef,
        callbacks: *const MTAudioProcessingTapCallbacks,
        flags: MTAudioProcessingTapCreationFlags,
        tapOut: *mut MTAudioProcessingTapRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn MTAudioProcessingTapGetStorage(
        tap: MTAudioProcessingTapRef,
    ) -> *mut ::std::os::raw::c_void;
}
unsafe extern "C" {
    pub fn MTAudioProcessingTapGetSourceAudio(
        tap: MTAudioProcessingTapRef,
        numberFrames: CMItemCount,
        bufferListInOut: *mut AudioBufferList,
        flagsOut: *mut MTAudioProcessingTapFlags,
        timeRangeOut: *mut CMTimeRange,
        numberFramesOut: *mut CMItemCount,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn MTRegisterProfessionalVideoWorkflowFormatReaders();
}
unsafe extern "C" {
    pub fn MTCopyLocalizedNameForMediaType(mediaType: CMMediaType) -> CFStringRef;
}
unsafe extern "C" {
    pub fn MTCopyLocalizedNameForMediaSubType(
        mediaType: CMMediaType,
        mediaSubType: FourCharCode,
    ) -> CFStringRef;
}

unsafe impl objc2::encode::RefEncode for opaqueMTAudioProcessingTap {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for opaqueMTAudioProcessingTap {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("opaqueMTAudioProcessingTap", &[]);
}
unsafe impl objc2::encode::RefEncode for MTAudioProcessingTapCallbacks {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MTAudioProcessingTapCallbacks {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("MTAudioProcessingTapCallbacks", &[]);
}
