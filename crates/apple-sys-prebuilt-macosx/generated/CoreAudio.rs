#[allow(unused_imports)]
use crate::objc::*;
#[allow(unused_imports)]
use crate::CoreFoundation::*;
#[allow(unused_imports)]
use crate::Foundation::*;
#[allow(unused_imports)]
use crate::IOKit::*;
#[allow(unused_imports)]
use libc::{id_t, pid_t};

#[allow(unused_imports)]
use objc2::msg_send;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AudioValueRange {
    pub mMinimum: Float64,
    pub mMaximum: Float64,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AudioBuffer {
    pub mNumberChannels: UInt32,
    pub mDataByteSize: UInt32,
    pub mData: *mut ::std::os::raw::c_void,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AudioBufferList {
    pub mNumberBuffers: UInt32,
    pub mBuffers: [AudioBuffer; 1usize],
}
pub type AudioFormatID = UInt32;
pub type AudioFormatFlags = UInt32;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AudioStreamBasicDescription {
    pub mSampleRate: Float64,
    pub mFormatID: AudioFormatID,
    pub mFormatFlags: AudioFormatFlags,
    pub mBytesPerPacket: UInt32,
    pub mFramesPerPacket: UInt32,
    pub mBytesPerFrame: UInt32,
    pub mChannelsPerFrame: UInt32,
    pub mBitsPerChannel: UInt32,
    pub mReserved: UInt32,
}
pub type SMPTETimeType = UInt32;
pub type SMPTETimeFlags = UInt32;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SMPTETime {
    pub mSubframes: SInt16,
    pub mSubframeDivisor: SInt16,
    pub mCounter: UInt32,
    pub mType: SMPTETimeType,
    pub mFlags: SMPTETimeFlags,
    pub mHours: SInt16,
    pub mMinutes: SInt16,
    pub mSeconds: SInt16,
    pub mFrames: SInt16,
}
pub type AudioTimeStampFlags = UInt32;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AudioTimeStamp {
    pub mSampleTime: Float64,
    pub mHostTime: UInt64,
    pub mRateScalar: Float64,
    pub mWordClockTime: UInt64,
    pub mSMPTETime: SMPTETime,
    pub mFlags: AudioTimeStampFlags,
    pub mReserved: UInt32,
}
pub type AudioObjectID = UInt32;
pub type AudioClassID = UInt32;
pub type AudioObjectPropertySelector = UInt32;
pub type AudioObjectPropertyScope = UInt32;
pub type AudioObjectPropertyElement = UInt32;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AudioObjectPropertyAddress {
    pub mSelector: AudioObjectPropertySelector,
    pub mScope: AudioObjectPropertyScope,
    pub mElement: AudioObjectPropertyElement,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AudioStreamRangedDescription {
    pub mFormat: AudioStreamBasicDescription,
    pub mSampleRateRange: AudioValueRange,
}
pub type AudioObjectPropertyListenerProc = ::std::option::Option<
    unsafe extern "C" fn(
        inObjectID: AudioObjectID,
        inNumberAddresses: UInt32,
        inAddresses: *const AudioObjectPropertyAddress,
        inClientData: *mut ::std::os::raw::c_void,
    ) -> OSStatus,
>;
pub type AudioObjectPropertyListenerBlock = *mut ::std::os::raw::c_void;
pub type AudioHardwarePowerHint = UInt32;
pub type AudioDeviceIOProc = ::std::option::Option<
    unsafe extern "C" fn(
        inDevice: AudioObjectID,
        inNow: *const AudioTimeStamp,
        inInputData: *const AudioBufferList,
        inInputTime: *const AudioTimeStamp,
        outOutputData: *mut AudioBufferList,
        inOutputTime: *const AudioTimeStamp,
        inClientData: *mut ::std::os::raw::c_void,
    ) -> OSStatus,
>;
pub type AudioDeviceIOBlock = *mut ::std::os::raw::c_void;
pub type AudioDeviceIOProcID = AudioDeviceIOProc;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AudioHardwareIOProcStreamUsage {
    pub mIOProc: *mut ::std::os::raw::c_void,
    pub mNumberStreams: UInt32,
    pub mStreamIsOn: [UInt32; 1usize],
}
pub type AudioLevelControlTransferFunction = UInt32;
pub type AudioHardwarePropertyID = AudioObjectPropertySelector;
pub type AudioHardwarePropertyListenerProc = ::std::option::Option<
    unsafe extern "C" fn(
        inPropertyID: AudioHardwarePropertyID,
        inClientData: *mut ::std::os::raw::c_void,
    ) -> OSStatus,
>;
pub type AudioDeviceID = AudioObjectID;
pub type AudioDevicePropertyID = AudioObjectPropertySelector;
pub type AudioDevicePropertyListenerProc = ::std::option::Option<
    unsafe extern "C" fn(
        inDevice: AudioDeviceID,
        inChannel: UInt32,
        isInput: Boolean,
        inPropertyID: AudioDevicePropertyID,
        inClientData: *mut ::std::os::raw::c_void,
    ) -> OSStatus,
>;
pub type AudioStreamID = AudioObjectID;
pub type AudioStreamPropertyListenerProc = ::std::option::Option<
    unsafe extern "C" fn(
        inStream: AudioStreamID,
        inChannel: UInt32,
        inPropertyID: AudioDevicePropertyID,
        inClientData: *mut ::std::os::raw::c_void,
    ) -> OSStatus,
>;
pub type AudioServerPlugInHostRef = *const AudioServerPlugInHostInterface;
pub type AudioServerPlugInDriverRef = *mut *mut AudioServerPlugInDriverInterface;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AudioServerPlugInCustomPropertyInfo {
    pub mSelector: AudioObjectPropertySelector,
    pub mPropertyDataType: UInt32,
    pub mQualifierDataType: UInt32,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AudioServerPlugInClientInfo {
    pub mClientID: UInt32,
    pub mProcessID: pid_t,
    pub mIsNativeEndian: Boolean,
    pub mBundleID: CFStringRef,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct AudioServerPlugInIOCycleInfo {
    pub __bindgen_anon_1: AudioServerPlugInIOCycleInfo__bindgen_ty_1,
    pub mIOCycleCounter: UInt64,
    pub mNominalIOBufferFrameSize: UInt32,
    pub mCurrentTime: AudioTimeStamp,
    pub mInputTime: AudioTimeStamp,
    pub mOutputTime: AudioTimeStamp,
    pub mDeviceHostTicksPerFrame: Float64,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union AudioServerPlugInIOCycleInfo__bindgen_ty_1 {
    pub mMainHostTicksPerFrame: Float64,
    pub mMasterHostTicksPerFrame: Float64,
}
pub type AudioServerPlugInCustomPropertyDataType = UInt32;
pub type AudioServerPlugInIOOperation = UInt32;
pub type AudioDeviceClockAlgorithmSelector = UInt32;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AudioServerPlugInHostInterface {
    pub PropertiesChanged: ::std::option::Option<
        unsafe extern "C" fn(
            inHost: AudioServerPlugInHostRef,
            inObjectID: AudioObjectID,
            inNumberAddresses: UInt32,
            inAddresses: *const AudioObjectPropertyAddress,
        ) -> OSStatus,
    >,
    pub CopyFromStorage: ::std::option::Option<
        unsafe extern "C" fn(
            inHost: AudioServerPlugInHostRef,
            inKey: CFStringRef,
            outData: *mut CFPropertyListRef,
        ) -> OSStatus,
    >,
    pub WriteToStorage: ::std::option::Option<
        unsafe extern "C" fn(
            inHost: AudioServerPlugInHostRef,
            inKey: CFStringRef,
            inData: CFPropertyListRef,
        ) -> OSStatus,
    >,
    pub DeleteFromStorage: ::std::option::Option<
        unsafe extern "C" fn(inHost: AudioServerPlugInHostRef, inKey: CFStringRef) -> OSStatus,
    >,
    pub RequestDeviceConfigurationChange: ::std::option::Option<
        unsafe extern "C" fn(
            inHost: AudioServerPlugInHostRef,
            inDeviceObjectID: AudioObjectID,
            inChangeAction: UInt64,
            inChangeInfo: *mut ::std::os::raw::c_void,
        ) -> OSStatus,
    >,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AudioServerPlugInDriverInterface {
    pub _reserved: *mut ::std::os::raw::c_void,
    pub QueryInterface: ::std::option::Option<
        unsafe extern "C" fn(
            inDriver: *mut ::std::os::raw::c_void,
            inUUID: REFIID,
            outInterface: *mut LPVOID,
        ) -> HRESULT,
    >,
    pub AddRef:
        ::std::option::Option<unsafe extern "C" fn(inDriver: *mut ::std::os::raw::c_void) -> ULONG>,
    pub Release:
        ::std::option::Option<unsafe extern "C" fn(inDriver: *mut ::std::os::raw::c_void) -> ULONG>,
    pub Initialize: ::std::option::Option<
        unsafe extern "C" fn(
            inDriver: AudioServerPlugInDriverRef,
            inHost: AudioServerPlugInHostRef,
        ) -> OSStatus,
    >,
    pub CreateDevice: ::std::option::Option<
        unsafe extern "C" fn(
            inDriver: AudioServerPlugInDriverRef,
            inDescription: CFDictionaryRef,
            inClientInfo: *const AudioServerPlugInClientInfo,
            outDeviceObjectID: *mut AudioObjectID,
        ) -> OSStatus,
    >,
    pub DestroyDevice: ::std::option::Option<
        unsafe extern "C" fn(
            inDriver: AudioServerPlugInDriverRef,
            inDeviceObjectID: AudioObjectID,
        ) -> OSStatus,
    >,
    pub AddDeviceClient: ::std::option::Option<
        unsafe extern "C" fn(
            inDriver: AudioServerPlugInDriverRef,
            inDeviceObjectID: AudioObjectID,
            inClientInfo: *const AudioServerPlugInClientInfo,
        ) -> OSStatus,
    >,
    pub RemoveDeviceClient: ::std::option::Option<
        unsafe extern "C" fn(
            inDriver: AudioServerPlugInDriverRef,
            inDeviceObjectID: AudioObjectID,
            inClientInfo: *const AudioServerPlugInClientInfo,
        ) -> OSStatus,
    >,
    pub PerformDeviceConfigurationChange: ::std::option::Option<
        unsafe extern "C" fn(
            inDriver: AudioServerPlugInDriverRef,
            inDeviceObjectID: AudioObjectID,
            inChangeAction: UInt64,
            inChangeInfo: *mut ::std::os::raw::c_void,
        ) -> OSStatus,
    >,
    pub AbortDeviceConfigurationChange: ::std::option::Option<
        unsafe extern "C" fn(
            inDriver: AudioServerPlugInDriverRef,
            inDeviceObjectID: AudioObjectID,
            inChangeAction: UInt64,
            inChangeInfo: *mut ::std::os::raw::c_void,
        ) -> OSStatus,
    >,
    pub HasProperty: ::std::option::Option<
        unsafe extern "C" fn(
            inDriver: AudioServerPlugInDriverRef,
            inObjectID: AudioObjectID,
            inClientProcessID: pid_t,
            inAddress: *const AudioObjectPropertyAddress,
        ) -> Boolean,
    >,
    pub IsPropertySettable: ::std::option::Option<
        unsafe extern "C" fn(
            inDriver: AudioServerPlugInDriverRef,
            inObjectID: AudioObjectID,
            inClientProcessID: pid_t,
            inAddress: *const AudioObjectPropertyAddress,
            outIsSettable: *mut Boolean,
        ) -> OSStatus,
    >,
    pub GetPropertyDataSize: ::std::option::Option<
        unsafe extern "C" fn(
            inDriver: AudioServerPlugInDriverRef,
            inObjectID: AudioObjectID,
            inClientProcessID: pid_t,
            inAddress: *const AudioObjectPropertyAddress,
            inQualifierDataSize: UInt32,
            inQualifierData: *const ::std::os::raw::c_void,
            outDataSize: *mut UInt32,
        ) -> OSStatus,
    >,
    pub GetPropertyData: ::std::option::Option<
        unsafe extern "C" fn(
            inDriver: AudioServerPlugInDriverRef,
            inObjectID: AudioObjectID,
            inClientProcessID: pid_t,
            inAddress: *const AudioObjectPropertyAddress,
            inQualifierDataSize: UInt32,
            inQualifierData: *const ::std::os::raw::c_void,
            inDataSize: UInt32,
            outDataSize: *mut UInt32,
            outData: *mut ::std::os::raw::c_void,
        ) -> OSStatus,
    >,
    pub SetPropertyData: ::std::option::Option<
        unsafe extern "C" fn(
            inDriver: AudioServerPlugInDriverRef,
            inObjectID: AudioObjectID,
            inClientProcessID: pid_t,
            inAddress: *const AudioObjectPropertyAddress,
            inQualifierDataSize: UInt32,
            inQualifierData: *const ::std::os::raw::c_void,
            inDataSize: UInt32,
            inData: *const ::std::os::raw::c_void,
        ) -> OSStatus,
    >,
    pub StartIO: ::std::option::Option<
        unsafe extern "C" fn(
            inDriver: AudioServerPlugInDriverRef,
            inDeviceObjectID: AudioObjectID,
            inClientID: UInt32,
        ) -> OSStatus,
    >,
    pub StopIO: ::std::option::Option<
        unsafe extern "C" fn(
            inDriver: AudioServerPlugInDriverRef,
            inDeviceObjectID: AudioObjectID,
            inClientID: UInt32,
        ) -> OSStatus,
    >,
    pub GetZeroTimeStamp: ::std::option::Option<
        unsafe extern "C" fn(
            inDriver: AudioServerPlugInDriverRef,
            inDeviceObjectID: AudioObjectID,
            inClientID: UInt32,
            outSampleTime: *mut Float64,
            outHostTime: *mut UInt64,
            outSeed: *mut UInt64,
        ) -> OSStatus,
    >,
    pub WillDoIOOperation: ::std::option::Option<
        unsafe extern "C" fn(
            inDriver: AudioServerPlugInDriverRef,
            inDeviceObjectID: AudioObjectID,
            inClientID: UInt32,
            inOperationID: UInt32,
            outWillDo: *mut Boolean,
            outWillDoInPlace: *mut Boolean,
        ) -> OSStatus,
    >,
    pub BeginIOOperation: ::std::option::Option<
        unsafe extern "C" fn(
            inDriver: AudioServerPlugInDriverRef,
            inDeviceObjectID: AudioObjectID,
            inClientID: UInt32,
            inOperationID: UInt32,
            inIOBufferFrameSize: UInt32,
            inIOCycleInfo: *const AudioServerPlugInIOCycleInfo,
        ) -> OSStatus,
    >,
    pub DoIOOperation: ::std::option::Option<
        unsafe extern "C" fn(
            inDriver: AudioServerPlugInDriverRef,
            inDeviceObjectID: AudioObjectID,
            inStreamObjectID: AudioObjectID,
            inClientID: UInt32,
            inOperationID: UInt32,
            inIOBufferFrameSize: UInt32,
            inIOCycleInfo: *const AudioServerPlugInIOCycleInfo,
            ioMainBuffer: *mut ::std::os::raw::c_void,
            ioSecondaryBuffer: *mut ::std::os::raw::c_void,
        ) -> OSStatus,
    >,
    pub EndIOOperation: ::std::option::Option<
        unsafe extern "C" fn(
            inDriver: AudioServerPlugInDriverRef,
            inDeviceObjectID: AudioObjectID,
            inClientID: UInt32,
            inOperationID: UInt32,
            inIOBufferFrameSize: UInt32,
            inIOCycleInfo: *const AudioServerPlugInIOCycleInfo,
        ) -> OSStatus,
    >,
}
pub type CATapMuteBehavior = NSInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CATapDescription(pub id);
impl std::ops::Deref for CATapDescription {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CATapDescription {}
impl CATapDescription {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CATapDescription").unwrap(), alloc) })
    }
}
impl INSObject for CATapDescription {}
impl PNSObject for CATapDescription {}
impl std::convert::TryFrom<NSObject> for CATapDescription {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<CATapDescription, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CATapDescription").unwrap()) };
        if is_kind_of {
            Ok(CATapDescription(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to CATapDescription")
        }
    }
}
impl ICATapDescription for CATapDescription {}
pub trait ICATapDescription: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn initStereoMixdownOfProcesses_(
        &self,
        processesObjectIDsToIncludeInTap: NSArray,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initStereoMixdownOfProcesses : processesObjectIDsToIncludeInTap)
    }
    unsafe fn initStereoGlobalTapButExcludeProcesses_(
        &self,
        processesObjectIDsToExcludeFromTap: NSArray,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initStereoGlobalTapButExcludeProcesses : processesObjectIDsToExcludeFromTap)
    }
    unsafe fn initMonoMixdownOfProcesses_(
        &self,
        processesObjectIDsToIncludeInTap: NSArray,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initMonoMixdownOfProcesses : processesObjectIDsToIncludeInTap)
    }
    unsafe fn initMonoGlobalTapButExcludeProcesses_(
        &self,
        processesObjectIDsToExcludeFromTap: NSArray,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initMonoGlobalTapButExcludeProcesses : processesObjectIDsToExcludeFromTap)
    }
    unsafe fn initWithProcesses_andDeviceUID_withStream_(
        &self,
        processesObjectIDsToIncludeInTap: NSArray,
        deviceUID: NSString,
        stream: NSInteger,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithProcesses : processesObjectIDsToIncludeInTap, andDeviceUID : deviceUID, withStream : stream)
    }
    unsafe fn initExcludingProcesses_andDeviceUID_withStream_(
        &self,
        processesObjectIDsToExcludeFromTap: NSArray,
        deviceUID: NSString,
        stream: NSInteger,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initExcludingProcesses : processesObjectIDsToExcludeFromTap, andDeviceUID : deviceUID, withStream : stream)
    }
    unsafe fn name(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, name)
    }
    unsafe fn setName_(&self, name: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setName : name)
    }
    unsafe fn UUID(&self) -> NSUUID
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, UUID)
    }
    unsafe fn setUUID_(&self, UUID: NSUUID)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setUUID : UUID)
    }
    unsafe fn processes(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, processes)
    }
    unsafe fn setProcesses_(&self, processes: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setProcesses : processes)
    }
    unsafe fn bundleIDs(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, bundleIDs)
    }
    unsafe fn setBundleIDs_(&self, bundleIDs: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setBundleIDs : bundleIDs)
    }
    unsafe fn isMono(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isMono)
    }
    unsafe fn setMono_(&self, mono: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMono : mono)
    }
    unsafe fn isExclusive(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isExclusive)
    }
    unsafe fn setExclusive_(&self, exclusive: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setExclusive : exclusive)
    }
    unsafe fn isMixdown(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isMixdown)
    }
    unsafe fn setMixdown_(&self, mixdown: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMixdown : mixdown)
    }
    unsafe fn isPrivate(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isPrivate)
    }
    unsafe fn setPrivate_(&self, privateTap: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPrivate : privateTap)
    }
    unsafe fn isProcessRestoreEnabled(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isProcessRestoreEnabled)
    }
    unsafe fn setProcessRestoreEnabled_(&self, processRestoreEnabled: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setProcessRestoreEnabled : processRestoreEnabled)
    }
    unsafe fn isMuted(&self) -> CATapMuteBehavior
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isMuted)
    }
    unsafe fn setMuteBehavior_(&self, muteBehavior: CATapMuteBehavior)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMuteBehavior : muteBehavior)
    }
    unsafe fn deviceUID(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, deviceUID)
    }
    unsafe fn setDeviceUID_(&self, deviceUID: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDeviceUID : deviceUID)
    }
    unsafe fn stream(&self) -> NSNumber
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, stream)
    }
    unsafe fn setStream_(&self, stream: NSNumber)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setStream : stream)
    }
}
unsafe extern "C" {
    pub fn AudioObjectShow(inObjectID: AudioObjectID);
}
unsafe extern "C" {
    pub fn AudioObjectHasProperty(
        inObjectID: AudioObjectID,
        inAddress: *const AudioObjectPropertyAddress,
    ) -> Boolean;
}
unsafe extern "C" {
    pub fn AudioObjectIsPropertySettable(
        inObjectID: AudioObjectID,
        inAddress: *const AudioObjectPropertyAddress,
        outIsSettable: *mut Boolean,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn AudioObjectGetPropertyDataSize(
        inObjectID: AudioObjectID,
        inAddress: *const AudioObjectPropertyAddress,
        inQualifierDataSize: UInt32,
        inQualifierData: *const ::std::os::raw::c_void,
        outDataSize: *mut UInt32,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn AudioObjectGetPropertyData(
        inObjectID: AudioObjectID,
        inAddress: *const AudioObjectPropertyAddress,
        inQualifierDataSize: UInt32,
        inQualifierData: *const ::std::os::raw::c_void,
        ioDataSize: *mut UInt32,
        outData: *mut ::std::os::raw::c_void,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn AudioObjectSetPropertyData(
        inObjectID: AudioObjectID,
        inAddress: *const AudioObjectPropertyAddress,
        inQualifierDataSize: UInt32,
        inQualifierData: *const ::std::os::raw::c_void,
        inDataSize: UInt32,
        inData: *const ::std::os::raw::c_void,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn AudioObjectAddPropertyListener(
        inObjectID: AudioObjectID,
        inAddress: *const AudioObjectPropertyAddress,
        inListener: AudioObjectPropertyListenerProc,
        inClientData: *mut ::std::os::raw::c_void,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn AudioObjectRemovePropertyListener(
        inObjectID: AudioObjectID,
        inAddress: *const AudioObjectPropertyAddress,
        inListener: AudioObjectPropertyListenerProc,
        inClientData: *mut ::std::os::raw::c_void,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn AudioObjectAddPropertyListenerBlock(
        inObjectID: AudioObjectID,
        inAddress: *const AudioObjectPropertyAddress,
        inDispatchQueue: NSObject,
        inListener: AudioObjectPropertyListenerBlock,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn AudioObjectRemovePropertyListenerBlock(
        inObjectID: AudioObjectID,
        inAddress: *const AudioObjectPropertyAddress,
        inDispatchQueue: NSObject,
        inListener: AudioObjectPropertyListenerBlock,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn AudioHardwareUnload() -> OSStatus;
}
unsafe extern "C" {
    pub fn AudioHardwareCreateAggregateDevice(
        inDescription: CFDictionaryRef,
        outDeviceID: *mut AudioObjectID,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn AudioHardwareDestroyAggregateDevice(inDeviceID: AudioObjectID) -> OSStatus;
}
unsafe extern "C" {
    pub fn AudioDeviceCreateIOProcID(
        inDevice: AudioObjectID,
        inProc: AudioDeviceIOProc,
        inClientData: *mut ::std::os::raw::c_void,
        outIOProcID: *mut AudioDeviceIOProcID,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn AudioDeviceCreateIOProcIDWithBlock(
        outIOProcID: *mut AudioDeviceIOProcID,
        inDevice: AudioObjectID,
        inDispatchQueue: NSObject,
        inIOBlock: AudioDeviceIOBlock,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn AudioDeviceDestroyIOProcID(
        inDevice: AudioObjectID,
        inIOProcID: AudioDeviceIOProcID,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn AudioDeviceStart(inDevice: AudioObjectID, inProcID: AudioDeviceIOProcID) -> OSStatus;
}
unsafe extern "C" {
    pub fn AudioDeviceStartAtTime(
        inDevice: AudioObjectID,
        inProcID: AudioDeviceIOProcID,
        ioRequestedStartTime: *mut AudioTimeStamp,
        inFlags: UInt32,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn AudioDeviceStop(inDevice: AudioObjectID, inProcID: AudioDeviceIOProcID) -> OSStatus;
}
unsafe extern "C" {
    pub fn AudioDeviceGetCurrentTime(
        inDevice: AudioObjectID,
        outTime: *mut AudioTimeStamp,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn AudioDeviceTranslateTime(
        inDevice: AudioObjectID,
        inTime: *const AudioTimeStamp,
        outTime: *mut AudioTimeStamp,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn AudioDeviceGetNearestStartTime(
        inDevice: AudioObjectID,
        ioRequestedStartTime: *mut AudioTimeStamp,
        inFlags: UInt32,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn AudioHardwareAddRunLoopSource(inRunLoopSource: CFRunLoopSourceRef) -> OSStatus;
}
unsafe extern "C" {
    pub fn AudioHardwareRemoveRunLoopSource(inRunLoopSource: CFRunLoopSourceRef) -> OSStatus;
}
unsafe extern "C" {
    pub fn AudioHardwareGetPropertyInfo(
        inPropertyID: AudioHardwarePropertyID,
        outSize: *mut UInt32,
        outWritable: *mut Boolean,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn AudioHardwareGetProperty(
        inPropertyID: AudioHardwarePropertyID,
        ioPropertyDataSize: *mut UInt32,
        outPropertyData: *mut ::std::os::raw::c_void,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn AudioHardwareSetProperty(
        inPropertyID: AudioHardwarePropertyID,
        inPropertyDataSize: UInt32,
        inPropertyData: *const ::std::os::raw::c_void,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn AudioHardwareAddPropertyListener(
        inPropertyID: AudioHardwarePropertyID,
        inProc: AudioHardwarePropertyListenerProc,
        inClientData: *mut ::std::os::raw::c_void,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn AudioHardwareRemovePropertyListener(
        inPropertyID: AudioHardwarePropertyID,
        inProc: AudioHardwarePropertyListenerProc,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn AudioDeviceAddIOProc(
        inDevice: AudioDeviceID,
        inProc: AudioDeviceIOProc,
        inClientData: *mut ::std::os::raw::c_void,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn AudioDeviceRemoveIOProc(inDevice: AudioDeviceID, inProc: AudioDeviceIOProc) -> OSStatus;
}
unsafe extern "C" {
    pub fn AudioDeviceRead(
        inDevice: AudioDeviceID,
        inStartTime: *const AudioTimeStamp,
        outData: *mut AudioBufferList,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn AudioDeviceGetPropertyInfo(
        inDevice: AudioDeviceID,
        inChannel: UInt32,
        isInput: Boolean,
        inPropertyID: AudioDevicePropertyID,
        outSize: *mut UInt32,
        outWritable: *mut Boolean,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn AudioDeviceGetProperty(
        inDevice: AudioDeviceID,
        inChannel: UInt32,
        isInput: Boolean,
        inPropertyID: AudioDevicePropertyID,
        ioPropertyDataSize: *mut UInt32,
        outPropertyData: *mut ::std::os::raw::c_void,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn AudioDeviceSetProperty(
        inDevice: AudioDeviceID,
        inWhen: *const AudioTimeStamp,
        inChannel: UInt32,
        isInput: Boolean,
        inPropertyID: AudioDevicePropertyID,
        inPropertyDataSize: UInt32,
        inPropertyData: *const ::std::os::raw::c_void,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn AudioDeviceAddPropertyListener(
        inDevice: AudioDeviceID,
        inChannel: UInt32,
        isInput: Boolean,
        inPropertyID: AudioDevicePropertyID,
        inProc: AudioDevicePropertyListenerProc,
        inClientData: *mut ::std::os::raw::c_void,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn AudioDeviceRemovePropertyListener(
        inDevice: AudioDeviceID,
        inChannel: UInt32,
        isInput: Boolean,
        inPropertyID: AudioDevicePropertyID,
        inProc: AudioDevicePropertyListenerProc,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn AudioStreamGetPropertyInfo(
        inStream: AudioStreamID,
        inChannel: UInt32,
        inPropertyID: AudioDevicePropertyID,
        outSize: *mut UInt32,
        outWritable: *mut Boolean,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn AudioStreamGetProperty(
        inStream: AudioStreamID,
        inChannel: UInt32,
        inPropertyID: AudioDevicePropertyID,
        ioPropertyDataSize: *mut UInt32,
        outPropertyData: *mut ::std::os::raw::c_void,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn AudioStreamSetProperty(
        inStream: AudioStreamID,
        inWhen: *const AudioTimeStamp,
        inChannel: UInt32,
        inPropertyID: AudioDevicePropertyID,
        inPropertyDataSize: UInt32,
        inPropertyData: *const ::std::os::raw::c_void,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn AudioStreamAddPropertyListener(
        inStream: AudioStreamID,
        inChannel: UInt32,
        inPropertyID: AudioDevicePropertyID,
        inProc: AudioStreamPropertyListenerProc,
        inClientData: *mut ::std::os::raw::c_void,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn AudioStreamRemovePropertyListener(
        inStream: AudioStreamID,
        inChannel: UInt32,
        inPropertyID: AudioDevicePropertyID,
        inProc: AudioStreamPropertyListenerProc,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn AudioGetCurrentHostTime() -> UInt64;
}
unsafe extern "C" {
    pub fn AudioGetHostClockFrequency() -> Float64;
}
unsafe extern "C" {
    pub fn AudioGetHostClockMinimumTimeDelta() -> UInt32;
}
unsafe extern "C" {
    pub fn AudioConvertHostTimeToNanos(inHostTime: UInt64) -> UInt64;
}
unsafe extern "C" {
    pub fn AudioConvertNanosToHostTime(inNanos: UInt64) -> UInt64;
}
unsafe extern "C" {
    pub fn AudioHardwareCreateProcessTap(
        inDescription: CATapDescription,
        outTapID: *mut AudioObjectID,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn AudioHardwareDestroyProcessTap(inTapID: AudioObjectID) -> OSStatus;
}

unsafe impl objc2::encode::RefEncode for AudioValueRange {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for AudioValueRange {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("AudioValueRange", &[]);
}
unsafe impl objc2::encode::RefEncode for AudioBuffer {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for AudioBuffer {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("AudioBuffer", &[]);
}
unsafe impl objc2::encode::RefEncode for AudioBufferList {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for AudioBufferList {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("AudioBufferList", &[]);
}
unsafe impl objc2::encode::RefEncode for AudioStreamBasicDescription {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for AudioStreamBasicDescription {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("AudioStreamBasicDescription", &[]);
}
unsafe impl objc2::encode::RefEncode for SMPTETime {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for SMPTETime {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("SMPTETime", &[]);
}
unsafe impl objc2::encode::RefEncode for AudioTimeStamp {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for AudioTimeStamp {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("AudioTimeStamp", &[]);
}
unsafe impl objc2::encode::RefEncode for AudioObjectPropertyAddress {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for AudioObjectPropertyAddress {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("AudioObjectPropertyAddress", &[]);
}
unsafe impl objc2::encode::RefEncode for AudioStreamRangedDescription {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for AudioStreamRangedDescription {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("AudioStreamRangedDescription", &[]);
}
unsafe impl objc2::encode::RefEncode for AudioHardwareIOProcStreamUsage {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for AudioHardwareIOProcStreamUsage {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("AudioHardwareIOProcStreamUsage", &[]);
}
unsafe impl objc2::encode::RefEncode for AudioServerPlugInCustomPropertyInfo {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for AudioServerPlugInCustomPropertyInfo {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("AudioServerPlugInCustomPropertyInfo", &[]);
}
unsafe impl objc2::encode::RefEncode for AudioServerPlugInClientInfo {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for AudioServerPlugInClientInfo {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("AudioServerPlugInClientInfo", &[]);
}
unsafe impl objc2::encode::RefEncode for AudioServerPlugInIOCycleInfo {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for AudioServerPlugInIOCycleInfo {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("AudioServerPlugInIOCycleInfo", &[]);
}
unsafe impl objc2::encode::RefEncode for AudioServerPlugInIOCycleInfo__bindgen_ty_1 {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for AudioServerPlugInIOCycleInfo__bindgen_ty_1 {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("AudioServerPlugInIOCycleInfo__bindgen_ty_1", &[]);
}
unsafe impl objc2::encode::RefEncode for AudioServerPlugInHostInterface {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for AudioServerPlugInHostInterface {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("AudioServerPlugInHostInterface", &[]);
}
unsafe impl objc2::encode::RefEncode for AudioServerPlugInDriverInterface {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for AudioServerPlugInDriverInterface {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("AudioServerPlugInDriverInterface", &[]);
}
unsafe impl objc2::encode::RefEncode for CATapDescription {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CATapDescription {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
