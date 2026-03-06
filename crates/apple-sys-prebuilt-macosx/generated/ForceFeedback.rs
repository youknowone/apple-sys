#[allow(unused_imports)]
use crate::ApplicationServices::*;
#[allow(unused_imports)]
use crate::CoreFoundation::*;
#[allow(unused_imports)]
use crate::CoreGraphics::*;
#[allow(unused_imports)]
use crate::IOKit::*;

#[allow(unused_imports)]
use objc2::msg_send;
pub type DWORD = UInt32;
pub type LPDWORD = *mut DWORD;
pub type LONG = SInt32;
pub type LPLONG = *mut LONG;
pub type FFCoordinateSystemFlag = UInt32;
pub type FFEffectParameterFlag = UInt32;
pub type FFEffectStartFlag = UInt32;
pub type FFEffectStatusFlag = UInt32;
pub type FFCommandFlag = UInt32;
pub type FFState = UInt32;
pub type FFProperty = UInt32;
pub type FFCooperativeLevelFlag = UInt32;
pub type FFCapabilitiesEffectType = UInt32;
pub type FFCapabilitiesEffectSubType = UInt32;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct FFCONSTANTFORCE {
    pub lMagnitude: LONG,
}
pub type PFFCONSTANTFORCE = *mut FFCONSTANTFORCE;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct FFRAMPFORCE {
    pub lStart: LONG,
    pub lEnd: LONG,
}
pub type PFFRAMPFORCE = *mut FFRAMPFORCE;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct FFPERIODIC {
    pub dwMagnitude: DWORD,
    pub lOffset: LONG,
    pub dwPhase: DWORD,
    pub dwPeriod: DWORD,
}
pub type PFFPERIODIC = *mut FFPERIODIC;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct FFCONDITION {
    pub lOffset: LONG,
    pub lPositiveCoefficient: LONG,
    pub lNegativeCoefficient: LONG,
    pub dwPositiveSaturation: DWORD,
    pub dwNegativeSaturation: DWORD,
    pub lDeadBand: LONG,
}
pub type PFFCONDITION = *mut FFCONDITION;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct FFCUSTOMFORCE {
    pub cChannels: DWORD,
    pub dwSamplePeriod: DWORD,
    pub cSamples: DWORD,
    pub rglForceData: LPLONG,
}
pub type PFFCUSTOMFORCE = *mut FFCUSTOMFORCE;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct FFENVELOPE {
    pub dwSize: DWORD,
    pub dwAttackLevel: DWORD,
    pub dwAttackTime: DWORD,
    pub dwFadeLevel: DWORD,
    pub dwFadeTime: DWORD,
}
pub type PFFENVELOPE = *mut FFENVELOPE;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct FFEFFECT {
    pub dwSize: DWORD,
    pub dwFlags: DWORD,
    pub dwDuration: DWORD,
    pub dwSamplePeriod: DWORD,
    pub dwGain: DWORD,
    pub dwTriggerButton: DWORD,
    pub dwTriggerRepeatInterval: DWORD,
    pub cAxes: DWORD,
    pub rgdwAxes: LPDWORD,
    pub rglDirection: LPLONG,
    pub lpEnvelope: PFFENVELOPE,
    pub cbTypeSpecificParams: DWORD,
    pub lpvTypeSpecificParams: *mut ::std::os::raw::c_void,
    pub dwStartDelay: DWORD,
}
pub type PFFEFFECT = *mut FFEFFECT;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct FFEFFESCAPE {
    pub dwSize: DWORD,
    pub dwCommand: DWORD,
    pub lpvInBuffer: *mut ::std::os::raw::c_void,
    pub cbInBuffer: DWORD,
    pub lpvOutBuffer: *mut ::std::os::raw::c_void,
    pub cbOutBuffer: DWORD,
}
pub type PFFEFFESCAPE = *mut FFEFFESCAPE;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct FFCAPABILITIES {
    pub ffSpecVer: NumVersion,
    pub supportedEffects: UInt32,
    pub emulatedEffects: UInt32,
    pub subType: UInt32,
    pub numFfAxes: UInt32,
    pub ffAxes: [UInt8; 32usize],
    pub storageCapacity: UInt32,
    pub playbackCapacity: UInt32,
    pub firmwareVer: NumVersion,
    pub hardwareVer: NumVersion,
    pub driverVer: NumVersion,
}
pub type PFFCAPABILITIES = *mut FFCAPABILITIES;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __FFDHIDDEN {
    _unused: [u8; 0],
}
pub type FFDeviceObjectReference = *mut __FFDHIDDEN;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __FFEHIDDEN {
    _unused: [u8; 0],
}
pub type FFEffectObjectReference = *mut __FFEHIDDEN;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ForceFeedbackDeviceState {
    pub dwSize: UInt32,
    pub dwState: UInt32,
    pub dwLoad: UInt32,
}
pub type ForceFeedbackDeviceStatePtr = *mut ForceFeedbackDeviceState;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ForceFeedbackVersion {
    pub apiVersion: NumVersion,
    pub plugInVersion: NumVersion,
}
pub type ForceFeedbackVersionPtr = *mut ForceFeedbackVersion;
pub type FFEffectDownloadID = UInt32;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct IOForceFeedbackDeviceInterface {
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
    pub ForceFeedbackGetVersion: ::std::option::Option<
        unsafe extern "C" fn(
            self_: *mut ::std::os::raw::c_void,
            version: *mut ForceFeedbackVersion,
        ) -> HRESULT,
    >,
    pub InitializeTerminate: ::std::option::Option<
        unsafe extern "C" fn(
            self_: *mut ::std::os::raw::c_void,
            forceFeedbackAPIVersion: NumVersion,
            hidDevice: io_object_t,
            begin: boolean_t,
        ) -> HRESULT,
    >,
    pub DestroyEffect: ::std::option::Option<
        unsafe extern "C" fn(
            self_: *mut ::std::os::raw::c_void,
            downloadID: FFEffectDownloadID,
        ) -> HRESULT,
    >,
    pub DownloadEffect: ::std::option::Option<
        unsafe extern "C" fn(
            self_: *mut ::std::os::raw::c_void,
            effectType: CFUUIDRef,
            pDownloadID: *mut FFEffectDownloadID,
            pEffect: *mut FFEFFECT,
            flags: FFEffectParameterFlag,
        ) -> HRESULT,
    >,
    pub Escape: ::std::option::Option<
        unsafe extern "C" fn(
            self_: *mut ::std::os::raw::c_void,
            downloadID: FFEffectDownloadID,
            pEscape: *mut FFEFFESCAPE,
        ) -> HRESULT,
    >,
    pub GetEffectStatus: ::std::option::Option<
        unsafe extern "C" fn(
            self_: *mut ::std::os::raw::c_void,
            downloadID: FFEffectDownloadID,
            pStatusCode: *mut FFEffectStatusFlag,
        ) -> HRESULT,
    >,
    pub GetForceFeedbackCapabilities: ::std::option::Option<
        unsafe extern "C" fn(
            self_: *mut ::std::os::raw::c_void,
            pCapabilities: *mut FFCAPABILITIES,
        ) -> HRESULT,
    >,
    pub GetForceFeedbackState: ::std::option::Option<
        unsafe extern "C" fn(
            self_: *mut ::std::os::raw::c_void,
            pDeviceState: *mut ForceFeedbackDeviceState,
        ) -> HRESULT,
    >,
    pub SendForceFeedbackCommand: ::std::option::Option<
        unsafe extern "C" fn(self_: *mut ::std::os::raw::c_void, state: FFCommandFlag) -> HRESULT,
    >,
    pub SetProperty: ::std::option::Option<
        unsafe extern "C" fn(
            self_: *mut ::std::os::raw::c_void,
            property: FFProperty,
            pValue: *mut ::std::os::raw::c_void,
        ) -> HRESULT,
    >,
    pub StartEffect: ::std::option::Option<
        unsafe extern "C" fn(
            self_: *mut ::std::os::raw::c_void,
            downloadID: FFEffectDownloadID,
            mode: FFEffectStartFlag,
            iterations: UInt32,
        ) -> HRESULT,
    >,
    pub StopEffect: ::std::option::Option<
        unsafe extern "C" fn(
            self_: *mut ::std::os::raw::c_void,
            downloadID: FFEffectDownloadID,
        ) -> HRESULT,
    >,
}
pub type IOForceFeedbackDeviceInterfacePtr = *mut IOForceFeedbackDeviceInterface;
unsafe extern "C" {
    pub fn FFCreateDevice(
        hidDevice: io_service_t,
        pDeviceReference: *mut FFDeviceObjectReference,
    ) -> HRESULT;
}
unsafe extern "C" {
    pub fn FFReleaseDevice(deviceReference: FFDeviceObjectReference) -> HRESULT;
}
unsafe extern "C" {
    pub fn FFIsForceFeedback(hidDevice: io_service_t) -> HRESULT;
}
unsafe extern "C" {
    pub fn FFDeviceCreateEffect(
        deviceReference: FFDeviceObjectReference,
        uuidRef: CFUUIDRef,
        pEffectDefinition: *mut FFEFFECT,
        pEffectReference: *mut FFEffectObjectReference,
    ) -> HRESULT;
}
unsafe extern "C" {
    pub fn FFDeviceReleaseEffect(
        deviceReference: FFDeviceObjectReference,
        effectReference: FFEffectObjectReference,
    ) -> HRESULT;
}
unsafe extern "C" {
    pub fn FFDeviceEscape(
        deviceReference: FFDeviceObjectReference,
        pFFEffectEscape: *mut FFEFFESCAPE,
    ) -> HRESULT;
}
unsafe extern "C" {
    pub fn FFDeviceGetForceFeedbackState(
        deviceReference: FFDeviceObjectReference,
        pFFState: *mut FFState,
    ) -> HRESULT;
}
unsafe extern "C" {
    pub fn FFDeviceSendForceFeedbackCommand(
        deviceReference: FFDeviceObjectReference,
        flags: FFCommandFlag,
    ) -> HRESULT;
}
unsafe extern "C" {
    pub fn FFDeviceSetForceFeedbackProperty(
        deviceReference: FFDeviceObjectReference,
        property: FFProperty,
        pValue: *mut ::std::os::raw::c_void,
    ) -> HRESULT;
}
unsafe extern "C" {
    pub fn FFDeviceGetForceFeedbackProperty(
        deviceReference: FFDeviceObjectReference,
        property: FFProperty,
        pValue: *mut ::std::os::raw::c_void,
        valueSize: IOByteCount,
    ) -> HRESULT;
}
unsafe extern "C" {
    pub fn FFDeviceSetCooperativeLevel(
        deviceReference: FFDeviceObjectReference,
        taskIdentifier: *mut ::std::os::raw::c_void,
        flags: FFCooperativeLevelFlag,
    ) -> HRESULT;
}
unsafe extern "C" {
    pub fn FFDeviceGetForceFeedbackCapabilities(
        deviceReference: FFDeviceObjectReference,
        pFFCapabilities: *mut FFCAPABILITIES,
    ) -> HRESULT;
}
unsafe extern "C" {
    pub fn FFEffectDownload(effectReference: FFEffectObjectReference) -> HRESULT;
}
unsafe extern "C" {
    pub fn FFEffectEscape(
        effectReference: FFEffectObjectReference,
        pFFEffectEscape: *mut FFEFFESCAPE,
    ) -> HRESULT;
}
unsafe extern "C" {
    pub fn FFEffectGetEffectStatus(
        effectReference: FFEffectObjectReference,
        pFlags: *mut FFEffectStatusFlag,
    ) -> HRESULT;
}
unsafe extern "C" {
    pub fn FFEffectGetParameters(
        effectReference: FFEffectObjectReference,
        pFFEffect: *mut FFEFFECT,
        flags: FFEffectParameterFlag,
    ) -> HRESULT;
}
unsafe extern "C" {
    pub fn FFEffectSetParameters(
        effectReference: FFEffectObjectReference,
        pFFEffect: *mut FFEFFECT,
        flags: FFEffectParameterFlag,
    ) -> HRESULT;
}
unsafe extern "C" {
    pub fn FFEffectStart(
        effectReference: FFEffectObjectReference,
        iterations: UInt32,
        flags: FFEffectStartFlag,
    ) -> HRESULT;
}
unsafe extern "C" {
    pub fn FFEffectStop(effectReference: FFEffectObjectReference) -> HRESULT;
}
unsafe extern "C" {
    pub fn FFEffectUnload(effectReference: FFEffectObjectReference) -> HRESULT;
}

unsafe impl objc2::encode::RefEncode for FFCONSTANTFORCE {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for FFCONSTANTFORCE {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("FFCONSTANTFORCE", &[]);
}
unsafe impl objc2::encode::RefEncode for FFRAMPFORCE {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for FFRAMPFORCE {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("FFRAMPFORCE", &[]);
}
unsafe impl objc2::encode::RefEncode for FFPERIODIC {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for FFPERIODIC {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("FFPERIODIC", &[]);
}
unsafe impl objc2::encode::RefEncode for FFCONDITION {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for FFCONDITION {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("FFCONDITION", &[]);
}
unsafe impl objc2::encode::RefEncode for FFCUSTOMFORCE {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for FFCUSTOMFORCE {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("FFCUSTOMFORCE", &[]);
}
unsafe impl objc2::encode::RefEncode for FFENVELOPE {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for FFENVELOPE {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("FFENVELOPE", &[]);
}
unsafe impl objc2::encode::RefEncode for FFEFFECT {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for FFEFFECT {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("FFEFFECT", &[]);
}
unsafe impl objc2::encode::RefEncode for FFEFFESCAPE {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for FFEFFESCAPE {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("FFEFFESCAPE", &[]);
}
unsafe impl objc2::encode::RefEncode for FFCAPABILITIES {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for FFCAPABILITIES {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("FFCAPABILITIES", &[]);
}
unsafe impl objc2::encode::RefEncode for __FFDHIDDEN {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for __FFDHIDDEN {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("__FFDHIDDEN", &[]);
}
unsafe impl objc2::encode::RefEncode for __FFEHIDDEN {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for __FFEHIDDEN {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("__FFEHIDDEN", &[]);
}
unsafe impl objc2::encode::RefEncode for ForceFeedbackDeviceState {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for ForceFeedbackDeviceState {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("ForceFeedbackDeviceState", &[]);
}
unsafe impl objc2::encode::RefEncode for ForceFeedbackVersion {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for ForceFeedbackVersion {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("ForceFeedbackVersion", &[]);
}
unsafe impl objc2::encode::RefEncode for IOForceFeedbackDeviceInterface {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for IOForceFeedbackDeviceInterface {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("IOForceFeedbackDeviceInterface", &[]);
}
