#[allow(unused_imports)]
use crate::objc::*;
#[allow(unused_imports)]
use crate::AVFAudio::*;
#[allow(unused_imports)]
use crate::CFNetwork::*;
#[allow(unused_imports)]
use crate::CoreAudio::*;
#[allow(unused_imports)]
use crate::CoreFoundation::*;
#[allow(unused_imports)]
use crate::CoreMIDI::*;
#[allow(unused_imports)]
use crate::CoreServices::*;
#[allow(unused_imports)]
use crate::CoreText::*;
#[allow(unused_imports)]
use crate::Foundation::*;
#[allow(unused_imports)]
use crate::Security::*;

#[allow(unused_imports)]
use objc2::msg_send;
pub type SInt64 = ::std::os::raw::c_longlong;
pub type os_workgroup_t = OS_os_workgroup;
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
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AudioStreamPacketDescription {
    pub mStartOffset: SInt64,
    pub mVariableFramesInPacket: UInt32,
    pub mDataByteSize: UInt32,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AudioClassDescription {
    pub mType: OSType,
    pub mSubType: OSType,
    pub mManufacturer: OSType,
}
pub type AudioChannelLabel = UInt32;
pub type AudioChannelLayoutTag = UInt32;
pub type AudioChannelBitmap = UInt32;
pub type AudioChannelFlags = UInt32;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AudioChannelDescription {
    pub mChannelLabel: AudioChannelLabel,
    pub mChannelFlags: AudioChannelFlags,
    pub mCoordinates: [Float32; 3usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AudioChannelLayout {
    pub mChannelLayoutTag: AudioChannelLayoutTag,
    pub mChannelBitmap: AudioChannelBitmap,
    pub mNumberChannelDescriptions: UInt32,
    pub mChannelDescriptions: [AudioChannelDescription; 1usize],
}
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct ComponentInstanceRecord {
    pub data: [::std::os::raw::c_long; 1usize],
}
pub type AudioComponentFlags = UInt32;
pub type AudioComponentInstantiationOptions = UInt32;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AudioComponentDescription {
    pub componentType: OSType,
    pub componentSubType: OSType,
    pub componentManufacturer: OSType,
    pub componentFlags: UInt32,
    pub componentFlagsMask: UInt32,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OpaqueAudioComponent {
    _unused: [u8; 0],
}
pub type AudioComponent = *mut OpaqueAudioComponent;
pub type AudioComponentInstance = *mut ComponentInstanceRecord;
pub type AudioComponentMethod = ::std::option::Option<
    unsafe extern "C" fn(self_: *mut ::std::os::raw::c_void, ...) -> OSStatus,
>;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AudioComponentPlugInInterface {
    pub Open: ::std::option::Option<
        unsafe extern "C" fn(
            self_: *mut ::std::os::raw::c_void,
            mInstance: AudioComponentInstance,
        ) -> OSStatus,
    >,
    pub Close:
        ::std::option::Option<unsafe extern "C" fn(self_: *mut ::std::os::raw::c_void) -> OSStatus>,
    pub Lookup:
        ::std::option::Option<unsafe extern "C" fn(selector: SInt16) -> AudioComponentMethod>,
    pub reserved: *mut ::std::os::raw::c_void,
}
pub type AudioComponentValidationResult = UInt32;
pub type AudioCodec = AudioComponentInstance;
pub type AudioCodecPropertyID = UInt32;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AudioCodecMagicCookieInfo {
    pub mMagicCookieSize: UInt32,
    pub mMagicCookie: *const ::std::os::raw::c_void,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AudioCodecPrimeInfo {
    pub leadingFrames: UInt32,
    pub trailingFrames: UInt32,
}
pub type AudioSettingsFlags = UInt32;
pub type AudioUnit = AudioComponentInstance;
pub type AudioUnitRenderActionFlags = UInt32;
pub type AudioUnitPropertyID = UInt32;
pub type AudioUnitScope = UInt32;
pub type AudioUnitElement = UInt32;
pub type AudioUnitParameterID = UInt32;
pub type AudioUnitParameterValue = Float32;
pub type AUParameterEventType = UInt32;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct AudioUnitParameterEvent {
    pub __bindgen_anon_1: AudioUnitParameterEvent__bindgen_ty_1,
    pub scope: AudioUnitScope,
    pub element: AudioUnitElement,
    pub parameter: AudioUnitParameterID,
    pub eventType: AUParameterEventType,
    pub eventValues: AudioUnitParameterEvent__bindgen_ty_1,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union AudioUnitParameterEvent__bindgen_ty_1 {
    pub __bindgen_anon_1: AudioUnitParameterEvent__bindgen_ty_1__bindgen_ty_1,
    pub __bindgen_anon_2: AudioUnitParameterEvent__bindgen_ty_1__bindgen_ty_2,
    pub ramp: AudioUnitParameterEvent__bindgen_ty_1__bindgen_ty_1,
    pub immediate: AudioUnitParameterEvent__bindgen_ty_1__bindgen_ty_2,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AudioUnitParameterEvent__bindgen_ty_1__bindgen_ty_1 {
    pub startBufferOffset: SInt32,
    pub durationInFrames: UInt32,
    pub startValue: AudioUnitParameterValue,
    pub endValue: AudioUnitParameterValue,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AudioUnitParameterEvent__bindgen_ty_1__bindgen_ty_2 {
    pub bufferOffset: UInt32,
    pub value: AudioUnitParameterValue,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AudioUnitParameter {
    pub mAudioUnit: AudioUnit,
    pub mParameterID: AudioUnitParameterID,
    pub mScope: AudioUnitScope,
    pub mElement: AudioUnitElement,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AudioUnitProperty {
    pub mAudioUnit: AudioUnit,
    pub mPropertyID: AudioUnitPropertyID,
    pub mScope: AudioUnitScope,
    pub mElement: AudioUnitElement,
}
pub type AURenderCallback = ::std::option::Option<
    unsafe extern "C" fn(
        inRefCon: *mut ::std::os::raw::c_void,
        ioActionFlags: *mut AudioUnitRenderActionFlags,
        inTimeStamp: *const AudioTimeStamp,
        inBusNumber: UInt32,
        inNumberFrames: UInt32,
        ioData: *mut AudioBufferList,
    ) -> OSStatus,
>;
pub type AUInputSamplesInOutputCallback = ::std::option::Option<
    unsafe extern "C" fn(
        inRefCon: *mut ::std::os::raw::c_void,
        inOutputTimeStamp: *const AudioTimeStamp,
        inInputSample: Float64,
        inNumberInputSamples: Float64,
    ),
>;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AudioUnitConnection {
    pub sourceAudioUnit: AudioUnit,
    pub sourceOutputNumber: UInt32,
    pub destInputNumber: UInt32,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AUChannelInfo {
    pub inChannels: SInt16,
    pub outChannels: SInt16,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AudioUnitExternalBuffer {
    pub buffer: *mut Byte,
    pub size: UInt32,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AURenderCallbackStruct {
    pub inputProc: AURenderCallback,
    pub inputProcRefCon: *mut ::std::os::raw::c_void,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AUPreset {
    pub presetNumber: SInt32,
    pub presetName: CFStringRef,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AudioUnitFrequencyResponseBin {
    pub mFrequency: Float64,
    pub mMagnitude: Float64,
}
pub type HostCallback_GetBeatAndTempo = ::std::option::Option<
    unsafe extern "C" fn(
        inHostUserData: *mut ::std::os::raw::c_void,
        outCurrentBeat: *mut Float64,
        outCurrentTempo: *mut Float64,
    ) -> OSStatus,
>;
pub type HostCallback_GetMusicalTimeLocation = ::std::option::Option<
    unsafe extern "C" fn(
        inHostUserData: *mut ::std::os::raw::c_void,
        outDeltaSampleOffsetToNextBeat: *mut UInt32,
        outTimeSig_Numerator: *mut Float32,
        outTimeSig_Denominator: *mut UInt32,
        outCurrentMeasureDownBeat: *mut Float64,
    ) -> OSStatus,
>;
pub type HostCallback_GetTransportState = ::std::option::Option<
    unsafe extern "C" fn(
        inHostUserData: *mut ::std::os::raw::c_void,
        outIsPlaying: *mut Boolean,
        outTransportStateChanged: *mut Boolean,
        outCurrentSampleInTimeLine: *mut Float64,
        outIsCycling: *mut Boolean,
        outCycleStartBeat: *mut Float64,
        outCycleEndBeat: *mut Float64,
    ) -> OSStatus,
>;
pub type HostCallback_GetTransportState2 = ::std::option::Option<
    unsafe extern "C" fn(
        inHostUserData: *mut ::std::os::raw::c_void,
        outIsPlaying: *mut Boolean,
        outIsRecording: *mut Boolean,
        outTransportStateChanged: *mut Boolean,
        outCurrentSampleInTimeLine: *mut Float64,
        outIsCycling: *mut Boolean,
        outCycleStartBeat: *mut Float64,
        outCycleEndBeat: *mut Float64,
    ) -> OSStatus,
>;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct HostCallbackInfo {
    pub hostUserData: *mut ::std::os::raw::c_void,
    pub beatAndTempoProc: HostCallback_GetBeatAndTempo,
    pub musicalTimeLocationProc: HostCallback_GetMusicalTimeLocation,
    pub transportStateProc: HostCallback_GetTransportState,
    pub transportStateProc2: HostCallback_GetTransportState2,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AUDependentParameter {
    pub mScope: AudioUnitScope,
    pub mParameterID: AudioUnitParameterID,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AudioUnitCocoaViewInfo {
    pub mCocoaAUViewBundleLocation: CFURLRef,
    pub mCocoaAUViewClass: [CFStringRef; 1usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AUHostVersionIdentifier {
    pub hostName: CFStringRef,
    pub hostVersion: UInt32,
}
pub type AUMIDIOutputCallback = ::std::option::Option<
    unsafe extern "C" fn(
        userData: *mut ::std::os::raw::c_void,
        timeStamp: *const AudioTimeStamp,
        midiOutNum: UInt32,
        pktlist: *const MIDIPacketList,
    ) -> OSStatus,
>;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AUMIDIOutputCallbackStruct {
    pub midiOutputCallback: AUMIDIOutputCallback,
    pub userData: *mut ::std::os::raw::c_void,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AUInputSamplesInOutputCallbackStruct {
    pub inputToOutputCallback: AUInputSamplesInOutputCallback,
    pub userData: *mut ::std::os::raw::c_void,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AudioUnitParameterHistoryInfo {
    pub updatesPerSecond: Float32,
    pub historyDurationInSeconds: Float32,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AudioUnitRenderContext {
    pub workgroup: os_workgroup_t,
    pub reserved: [u32; 6usize],
}
pub type AURenderContextObserver = *mut ::std::os::raw::c_void;
pub type AUEventSampleTime = i64;
pub type AUMIDIEventListBlock = *mut ::std::os::raw::c_void;
pub type AudioUnitParameterUnit = UInt32;
pub type AudioUnitParameterOptions = UInt32;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AudioUnitParameterInfo {
    pub name: [::std::os::raw::c_char; 52usize],
    pub unitName: CFStringRef,
    pub clumpID: UInt32,
    pub cfNameString: CFStringRef,
    pub unit: AudioUnitParameterUnit,
    pub minValue: AudioUnitParameterValue,
    pub maxValue: AudioUnitParameterValue,
    pub defaultValue: AudioUnitParameterValue,
    pub flags: AudioUnitParameterOptions,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AudioUnitParameterNameInfo {
    pub inID: AudioUnitParameterID,
    pub inDesiredLength: SInt32,
    pub outName: CFStringRef,
}
pub type AudioUnitParameterIDName = AudioUnitParameterNameInfo;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AudioUnitParameterStringFromValue {
    pub inParamID: AudioUnitParameterID,
    pub inValue: *const AudioUnitParameterValue,
    pub outString: CFStringRef,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AudioUnitParameterValueFromString {
    pub inParamID: AudioUnitParameterID,
    pub inString: CFStringRef,
    pub outValue: AudioUnitParameterValue,
}
pub type AudioUnitRemoteControlEvent = UInt32;
pub type AUParameterMIDIMappingFlags = UInt32;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AUParameterMIDIMapping {
    pub mScope: AudioUnitScope,
    pub mElement: AudioUnitElement,
    pub mParameterID: AudioUnitParameterID,
    pub mFlags: AUParameterMIDIMappingFlags,
    pub mSubRangeMin: AudioUnitParameterValue,
    pub mSubRangeMax: AudioUnitParameterValue,
    pub mStatus: UInt8,
    pub mData1: UInt8,
    pub reserved1: UInt8,
    pub reserved2: UInt8,
    pub reserved3: UInt32,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AUDistanceAttenuationData {
    pub __bindgen_anon_1: AUDistanceAttenuationData__bindgen_ty_1,
    pub inNumberOfPairs: UInt32,
    pub pairs: [AUDistanceAttenuationData__bindgen_ty_1; 1usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AUDistanceAttenuationData__bindgen_ty_1 {
    pub inDistance: Float32,
    pub outGain: Float32,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AudioUnitOtherPluginDesc {
    pub format: UInt32,
    pub plugin: AudioClassDescription,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AudioUnitParameterValueTranslation {
    pub otherDesc: AudioUnitOtherPluginDesc,
    pub otherParamID: UInt32,
    pub otherValue: Float32,
    pub auParamID: AudioUnitParameterID,
    pub auValue: AudioUnitParameterValue,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AudioUnitPresetMAS_SettingData {
    pub isStockSetting: UInt32,
    pub settingID: UInt32,
    pub dataLen: UInt32,
    pub data: [UInt8; 1usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AudioUnitPresetMAS_Settings {
    pub manufacturerID: UInt32,
    pub effectID: UInt32,
    pub variantID: UInt32,
    pub settingsVersion: UInt32,
    pub numberOfSettings: UInt32,
    pub settings: [AudioUnitPresetMAS_SettingData; 1usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AudioOutputUnitMIDICallbacks {
    pub userData: *mut ::std::os::raw::c_void,
    pub MIDIEventProc: ::std::option::Option<
        unsafe extern "C" fn(
            userData: *mut ::std::os::raw::c_void,
            inStatus: UInt32,
            inData1: UInt32,
            inData2: UInt32,
            inOffsetSampleFrame: UInt32,
        ),
    >,
    pub MIDISysExProc: ::std::option::Option<
        unsafe extern "C" fn(
            userData: *mut ::std::os::raw::c_void,
            inData: *const UInt8,
            inLength: UInt32,
        ),
    >,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AudioOutputUnitStartAtTimeParams {
    pub mTimestamp: AudioTimeStamp,
    pub mFlags: UInt32,
}
pub type AUVoiceIOSpeechActivityEvent = UInt32;
pub type AUVoiceIOOtherAudioDuckingLevel = UInt32;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AUVoiceIOOtherAudioDuckingConfiguration {
    pub mEnableAdvancedDucking: Boolean,
    pub mDuckingLevel: AUVoiceIOOtherAudioDuckingLevel,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AudioUnitMeterClipping {
    pub peakValueSinceLastCall: Float32,
    pub sawInfinity: Boolean,
    pub sawNotANumber: Boolean,
}
pub type AUSpatializationAlgorithm = UInt32;
pub type AUSpatialMixerSourceMode = UInt32;
pub type AUReverbRoomType = UInt32;
pub type AUSpatialMixerAttenuationCurve = UInt32;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct MixerDistanceParams {
    pub mReferenceDistance: Float32,
    pub mMaxDistance: Float32,
    pub mMaxAttenuation: Float32,
}
pub type AUSpatialMixerRenderingFlags = UInt32;
pub type AUSpatialMixerPersonalizedHRTFMode = UInt32;
pub type AUSpatialMixerOutputType = UInt32;
pub type AUSpatialMixerPointSourceInHeadMode = UInt32;
pub type AU3DMixerRenderingFlags = UInt32;
pub type AU3DMixerAttenuationCurve = UInt32;
pub type AUScheduledAudioSliceFlags = UInt32;
pub type ScheduledAudioSliceCompletionProc = ::std::option::Option<
    unsafe extern "C" fn(
        userData: *mut ::std::os::raw::c_void,
        bufferList: *mut ScheduledAudioSlice,
    ),
>;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ScheduledAudioSlice {
    pub mTimeStamp: AudioTimeStamp,
    pub mCompletionProc: ScheduledAudioSliceCompletionProc,
    pub mCompletionProcUserData: *mut ::std::os::raw::c_void,
    pub mFlags: AUScheduledAudioSliceFlags,
    pub mReserved: UInt32,
    pub mReserved2: *mut ::std::os::raw::c_void,
    pub mNumberFrames: UInt32,
    pub mBufferList: *mut AudioBufferList,
}
pub type ScheduledAudioFileRegionCompletionProc = ::std::option::Option<
    unsafe extern "C" fn(
        userData: *mut ::std::os::raw::c_void,
        fileRegion: *mut ScheduledAudioFileRegion,
        result: OSStatus,
    ),
>;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ScheduledAudioFileRegion {
    pub mTimeStamp: AudioTimeStamp,
    pub mCompletionProc: ScheduledAudioFileRegionCompletionProc,
    pub mCompletionProcUserData: *mut ::std::os::raw::c_void,
    pub mAudioFile: *mut OpaqueAudioFileID,
    pub mLoopCount: UInt32,
    pub mStartFrame: SInt64,
    pub mFramesToPlay: UInt32,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AUSamplerInstrumentData {
    pub fileURL: CFURLRef,
    pub instrumentType: UInt8,
    pub bankMSB: UInt8,
    pub bankLSB: UInt8,
    pub presetID: UInt8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AUNumVersion {
    pub nonRelRev: UInt8,
    pub stage: UInt8,
    pub minorAndBugRev: UInt8,
    pub majorRev: UInt8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AUHostIdentifier {
    pub hostName: CFStringRef,
    pub hostVersion: AUNumVersion,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AudioUnitMIDIControlMapping {
    pub midiNRPN: UInt16,
    pub midiControl: UInt8,
    pub scope: UInt8,
    pub element: AudioUnitElement,
    pub parameter: AudioUnitParameterID,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AudioUnitParameterValueName {
    pub inParamID: AudioUnitParameterID,
    pub inValue: *const Float32,
    pub outName: CFStringRef,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AUSamplerBankPresetData {
    pub bankURL: CFURLRef,
    pub bankMSB: UInt8,
    pub bankLSB: UInt8,
    pub presetID: UInt8,
    pub reserved: UInt8,
}
pub type AUValue = f32;
pub type AUParameterAddress = u64;
pub type AUParameterAutomationEventType = u32;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AURecordedParameterEvent {
    pub hostTime: u64,
    pub address: AUParameterAddress,
    pub value: AUValue,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AUParameterAutomationEvent {
    pub hostTime: u64,
    pub address: AUParameterAddress,
    pub value: AUValue,
    pub eventType: AUParameterAutomationEventType,
    pub reserved: u64,
}
pub type AUParameterObserver = *mut ::std::os::raw::c_void;
pub type AUParameterRecordingObserver = *mut ::std::os::raw::c_void;
pub type AUParameterAutomationObserver = *mut ::std::os::raw::c_void;
pub type AUParameterObserverToken = *mut ::std::os::raw::c_void;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct AUParameterNode(pub id);
impl std::ops::Deref for AUParameterNode {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for AUParameterNode {}
impl AUParameterNode {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"AUParameterNode").unwrap(), alloc) })
    }
}
impl INSObject for AUParameterNode {}
impl PNSObject for AUParameterNode {}
impl std::convert::TryFrom<NSObject> for AUParameterNode {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<AUParameterNode, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"AUParameterNode").unwrap()) };
        if is_kind_of {
            Ok(AUParameterNode(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to AUParameterNode")
        }
    }
}
impl IAUParameterNode for AUParameterNode {}
pub trait IAUParameterNode: Sized + std::ops::Deref {
    unsafe fn displayNameWithLength_(&self, maximumLength: NSInteger) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, displayNameWithLength : maximumLength)
    }
    unsafe fn tokenByAddingParameterObserver_(
        &self,
        observer: AUParameterObserver,
    ) -> AUParameterObserverToken
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, tokenByAddingParameterObserver : observer)
    }
    unsafe fn tokenByAddingParameterRecordingObserver_(
        &self,
        observer: AUParameterRecordingObserver,
    ) -> AUParameterObserverToken
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, tokenByAddingParameterRecordingObserver : observer)
    }
    unsafe fn tokenByAddingParameterAutomationObserver_(
        &self,
        observer: AUParameterAutomationObserver,
    ) -> AUParameterObserverToken
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, tokenByAddingParameterAutomationObserver : observer)
    }
    unsafe fn removeParameterObserver_(&self, token: AUParameterObserverToken)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, removeParameterObserver : token)
    }
    unsafe fn identifier(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, identifier)
    }
    unsafe fn keyPath(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, keyPath)
    }
    unsafe fn displayName(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, displayName)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct AUParameterGroup(pub id);
impl std::ops::Deref for AUParameterGroup {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for AUParameterGroup {}
impl AUParameterGroup {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"AUParameterGroup").unwrap(), alloc) })
    }
}
impl PNSSecureCoding for AUParameterGroup {}
impl IAUParameterNode for AUParameterGroup {}
impl From<AUParameterGroup> for AUParameterNode {
    fn from(child: AUParameterGroup) -> AUParameterNode {
        AUParameterNode(child.0)
    }
}
impl std::convert::TryFrom<AUParameterNode> for AUParameterGroup {
    type Error = &'static str;
    fn try_from(parent: AUParameterNode) -> Result<AUParameterGroup, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"AUParameterGroup").unwrap()) };
        if is_kind_of {
            Ok(AUParameterGroup(parent.0))
        } else {
            Err("This AUParameterNode cannot be downcasted to AUParameterGroup")
        }
    }
}
impl INSObject for AUParameterGroup {}
impl PNSObject for AUParameterGroup {}
impl IAUParameterGroup for AUParameterGroup {}
pub trait IAUParameterGroup: Sized + std::ops::Deref {
    unsafe fn children(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, children)
    }
    unsafe fn allParameters(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, allParameters)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct AUParameterTree(pub id);
impl std::ops::Deref for AUParameterTree {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for AUParameterTree {}
impl AUParameterTree {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"AUParameterTree").unwrap(), alloc) })
    }
}
impl PNSSecureCoding for AUParameterTree {}
impl IAUParameterGroup for AUParameterTree {}
impl From<AUParameterTree> for AUParameterGroup {
    fn from(child: AUParameterTree) -> AUParameterGroup {
        AUParameterGroup(child.0)
    }
}
impl std::convert::TryFrom<AUParameterGroup> for AUParameterTree {
    type Error = &'static str;
    fn try_from(parent: AUParameterGroup) -> Result<AUParameterTree, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"AUParameterTree").unwrap()) };
        if is_kind_of {
            Ok(AUParameterTree(parent.0))
        } else {
            Err("This AUParameterGroup cannot be downcasted to AUParameterTree")
        }
    }
}
impl IAUParameterNode for AUParameterTree {}
impl INSObject for AUParameterTree {}
impl PNSObject for AUParameterTree {}
impl IAUParameterTree for AUParameterTree {}
pub trait IAUParameterTree: Sized + std::ops::Deref {
    unsafe fn parameterWithAddress_(&self, address: AUParameterAddress) -> AUParameter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, parameterWithAddress : address)
    }
    unsafe fn parameterWithID_scope_element_(
        &self,
        paramID: AudioUnitParameterID,
        scope: AudioUnitScope,
        element: AudioUnitElement,
    ) -> AUParameter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, parameterWithID : paramID, scope : scope, element : element)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct AUParameter(pub id);
impl std::ops::Deref for AUParameter {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for AUParameter {}
impl AUParameter {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"AUParameter").unwrap(), alloc) })
    }
}
impl PNSSecureCoding for AUParameter {}
impl IAUParameterNode for AUParameter {}
impl std::convert::TryFrom<AUParameterNode> for AUParameter {
    type Error = &'static str;
    fn try_from(parent: AUParameterNode) -> Result<AUParameter, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"AUParameter").unwrap()) };
        if is_kind_of {
            Ok(AUParameter(parent.0))
        } else {
            Err("This AUParameterNode cannot be downcasted to AUParameter")
        }
    }
}
impl INSObject for AUParameter {}
impl PNSObject for AUParameter {}
impl IAUParameter for AUParameter {}
pub trait IAUParameter: Sized + std::ops::Deref {
    unsafe fn setValue_originator_(&self, value: AUValue, originator: AUParameterObserverToken)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setValue : value, originator : originator)
    }
    unsafe fn setValue_originator_atHostTime_(
        &self,
        value: AUValue,
        originator: AUParameterObserverToken,
        hostTime: u64,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setValue : value, originator : originator, atHostTime : hostTime)
    }
    unsafe fn setValue_originator_atHostTime_eventType_(
        &self,
        value: AUValue,
        originator: AUParameterObserverToken,
        hostTime: u64,
        eventType: AUParameterAutomationEventType,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setValue : value, originator : originator, atHostTime : hostTime, eventType : eventType)
    }
    unsafe fn stringFromValue_(&self, value: *const AUValue) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, stringFromValue : value)
    }
    unsafe fn valueFromString_(&self, string: NSString) -> AUValue
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, valueFromString : string)
    }
    unsafe fn minValue(&self) -> AUValue
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, minValue)
    }
    unsafe fn maxValue(&self) -> AUValue
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, maxValue)
    }
    unsafe fn unit(&self) -> AudioUnitParameterUnit
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, unit)
    }
    unsafe fn unitName(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, unitName)
    }
    unsafe fn flags(&self) -> AudioUnitParameterOptions
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, flags)
    }
    unsafe fn address(&self) -> AUParameterAddress
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, address)
    }
    unsafe fn valueStrings(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, valueStrings)
    }
    unsafe fn dependentParameters(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, dependentParameters)
    }
    unsafe fn value(&self) -> AUValue
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, value)
    }
    unsafe fn setValue_(&self, value: AUValue)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setValue : value)
    }
}
pub type AUAudioObjectID = UInt32;
pub type AUAudioUnitStatus = OSStatus;
pub type AUAudioFrameCount = u32;
pub type AUAudioChannelCount = u32;
pub type AUAudioUnitBusType = NSInteger;
pub type AURenderPullInputBlock = *mut ::std::os::raw::c_void;
pub type AURenderBlock = *mut ::std::os::raw::c_void;
pub type AURenderObserver = *mut ::std::os::raw::c_void;
pub type AUScheduleParameterBlock = *mut ::std::os::raw::c_void;
pub type AUScheduleMIDIEventBlock = *mut ::std::os::raw::c_void;
pub type AUMIDIOutputEventBlock = *mut ::std::os::raw::c_void;
pub type AUHostMusicalContextBlock = *mut ::std::os::raw::c_void;
pub type AUMIDICIProfileChangedBlock = *mut ::std::os::raw::c_void;
pub type AUHostTransportStateFlags = NSUInteger;
pub type AUHostTransportStateBlock = *mut ::std::os::raw::c_void;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct AUAudioUnit(pub id);
impl std::ops::Deref for AUAudioUnit {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for AUAudioUnit {}
impl AUAudioUnit {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"AUAudioUnit").unwrap(), alloc) })
    }
}
impl INSObject for AUAudioUnit {}
impl PNSObject for AUAudioUnit {}
impl std::convert::TryFrom<NSObject> for AUAudioUnit {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<AUAudioUnit, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"AUAudioUnit").unwrap()) };
        if is_kind_of {
            Ok(AUAudioUnit(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to AUAudioUnit")
        }
    }
}
impl IAUAudioUnit for AUAudioUnit {}
pub trait IAUAudioUnit: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn initWithComponentDescription_options_error_(
        &self,
        componentDescription: AudioComponentDescription,
        options: AudioComponentInstantiationOptions,
        outError: *mut NSError,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithComponentDescription : componentDescription, options : options, error : outError)
    }
    unsafe fn initWithComponentDescription_error_(
        &self,
        componentDescription: AudioComponentDescription,
        outError: *mut NSError,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithComponentDescription : componentDescription, error : outError)
    }
    unsafe fn allocateRenderResourcesAndReturnError_(&self, outError: *mut NSError) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, allocateRenderResourcesAndReturnError : outError)
    }
    unsafe fn deallocateRenderResources(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, deallocateRenderResources)
    }
    unsafe fn reset(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, reset)
    }
    unsafe fn tokenByAddingRenderObserver_(&self, observer: AURenderObserver) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, tokenByAddingRenderObserver : observer)
    }
    unsafe fn removeRenderObserver_(&self, token: NSInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, removeRenderObserver : token)
    }
    unsafe fn parametersForOverviewWithCount_(&self, count: NSInteger) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, parametersForOverviewWithCount : count)
    }
    unsafe fn saveUserPreset_error_(
        &self,
        userPreset: AUAudioUnitPreset,
        outError: *mut NSError,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, saveUserPreset : userPreset, error : outError)
    }
    unsafe fn deleteUserPreset_error_(
        &self,
        userPreset: AUAudioUnitPreset,
        outError: *mut NSError,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, deleteUserPreset : userPreset, error : outError)
    }
    unsafe fn presetStateFor_error_(
        &self,
        userPreset: AUAudioUnitPreset,
        outError: *mut NSError,
    ) -> NSDictionary
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, presetStateFor : userPreset, error : outError)
    }
    unsafe fn profileStateForCable_channel_(
        &self,
        cable: u8,
        channel: MIDIChannelNumber,
    ) -> MIDICIProfileState
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, profileStateForCable : cable, channel : channel)
    }
    unsafe fn enableProfile_cable_onChannel_error_(
        &self,
        profile: MIDICIProfile,
        cable: u8,
        channel: MIDIChannelNumber,
        outError: *mut NSError,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, enableProfile : profile, cable : cable, onChannel : channel, error : outError)
    }
    unsafe fn disableProfile_cable_onChannel_error_(
        &self,
        profile: MIDICIProfile,
        cable: u8,
        channel: MIDIChannelNumber,
        outError: *mut NSError,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, disableProfile : profile, cable : cable, onChannel : channel, error : outError)
    }
    unsafe fn messageChannelFor_(&self, channelName: NSString) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, messageChannelFor : channelName)
    }
    unsafe fn componentDescription(&self) -> AudioComponentDescription
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, componentDescription)
    }
    unsafe fn component(&self) -> AudioComponent
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, component)
    }
    unsafe fn componentName(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, componentName)
    }
    unsafe fn audioUnitName(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, audioUnitName)
    }
    unsafe fn manufacturerName(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, manufacturerName)
    }
    unsafe fn audioUnitShortName(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, audioUnitShortName)
    }
    unsafe fn componentVersion(&self) -> u32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, componentVersion)
    }
    unsafe fn renderResourcesAllocated(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, renderResourcesAllocated)
    }
    unsafe fn inputBusses(&self) -> AUAudioUnitBusArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inputBusses)
    }
    unsafe fn outputBusses(&self) -> AUAudioUnitBusArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, outputBusses)
    }
    unsafe fn renderBlock(&self) -> AURenderBlock
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, renderBlock)
    }
    unsafe fn scheduleParameterBlock(&self) -> AUScheduleParameterBlock
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, scheduleParameterBlock)
    }
    unsafe fn maximumFramesToRender(&self) -> AUAudioFrameCount
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, maximumFramesToRender)
    }
    unsafe fn setMaximumFramesToRender_(&self, maximumFramesToRender: AUAudioFrameCount)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMaximumFramesToRender : maximumFramesToRender)
    }
    unsafe fn parameterTree(&self) -> AUParameterTree
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, parameterTree)
    }
    unsafe fn setParameterTree_(&self, parameterTree: AUParameterTree)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setParameterTree : parameterTree)
    }
    unsafe fn allParameterValues(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, allParameterValues)
    }
    unsafe fn isMusicDeviceOrEffect(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isMusicDeviceOrEffect)
    }
    unsafe fn virtualMIDICableCount(&self) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, virtualMIDICableCount)
    }
    unsafe fn scheduleMIDIEventBlock(&self) -> AUScheduleMIDIEventBlock
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, scheduleMIDIEventBlock)
    }
    unsafe fn scheduleMIDIEventListBlock(&self) -> AUMIDIEventListBlock
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, scheduleMIDIEventListBlock)
    }
    unsafe fn MIDIOutputNames(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, MIDIOutputNames)
    }
    unsafe fn providesUserInterface(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, providesUserInterface)
    }
    unsafe fn MIDIOutputEventBlock(&self) -> AUMIDIOutputEventBlock
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, MIDIOutputEventBlock)
    }
    unsafe fn setMIDIOutputEventBlock_(&self, MIDIOutputEventBlock: AUMIDIOutputEventBlock)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMIDIOutputEventBlock : MIDIOutputEventBlock)
    }
    unsafe fn MIDIOutputEventListBlock(&self) -> AUMIDIEventListBlock
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, MIDIOutputEventListBlock)
    }
    unsafe fn setMIDIOutputEventListBlock_(&self, MIDIOutputEventListBlock: AUMIDIEventListBlock)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMIDIOutputEventListBlock : MIDIOutputEventListBlock)
    }
    unsafe fn AudioUnitMIDIProtocol(&self) -> MIDIProtocolID
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, AudioUnitMIDIProtocol)
    }
    unsafe fn hostMIDIProtocol(&self) -> MIDIProtocolID
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, hostMIDIProtocol)
    }
    unsafe fn setHostMIDIProtocol_(&self, hostMIDIProtocol: MIDIProtocolID)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setHostMIDIProtocol : hostMIDIProtocol)
    }
    unsafe fn fullState(&self) -> NSDictionary
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, fullState)
    }
    unsafe fn setFullState_(&self, fullState: NSDictionary)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setFullState : fullState)
    }
    unsafe fn fullStateForDocument(&self) -> NSDictionary
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, fullStateForDocument)
    }
    unsafe fn setFullStateForDocument_(&self, fullStateForDocument: NSDictionary)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setFullStateForDocument : fullStateForDocument)
    }
    unsafe fn factoryPresets(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, factoryPresets)
    }
    unsafe fn userPresets(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, userPresets)
    }
    unsafe fn supportsUserPresets(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, supportsUserPresets)
    }
    unsafe fn isLoadedInProcess(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isLoadedInProcess)
    }
    unsafe fn currentPreset(&self) -> AUAudioUnitPreset
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, currentPreset)
    }
    unsafe fn setCurrentPreset_(&self, currentPreset: AUAudioUnitPreset)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCurrentPreset : currentPreset)
    }
    unsafe fn latency(&self) -> NSTimeInterval
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, latency)
    }
    unsafe fn tailTime(&self) -> NSTimeInterval
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, tailTime)
    }
    unsafe fn renderQuality(&self) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, renderQuality)
    }
    unsafe fn setRenderQuality_(&self, renderQuality: NSInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRenderQuality : renderQuality)
    }
    unsafe fn shouldBypassEffect(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, shouldBypassEffect)
    }
    unsafe fn setShouldBypassEffect_(&self, shouldBypassEffect: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setShouldBypassEffect : shouldBypassEffect)
    }
    unsafe fn canProcessInPlace(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, canProcessInPlace)
    }
    unsafe fn isRenderingOffline(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isRenderingOffline)
    }
    unsafe fn setRenderingOffline_(&self, renderingOffline: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRenderingOffline : renderingOffline)
    }
    unsafe fn channelCapabilities(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, channelCapabilities)
    }
    unsafe fn musicalContextBlock(&self) -> AUHostMusicalContextBlock
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, musicalContextBlock)
    }
    unsafe fn setMusicalContextBlock_(&self, musicalContextBlock: AUHostMusicalContextBlock)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMusicalContextBlock : musicalContextBlock)
    }
    unsafe fn transportStateBlock(&self) -> AUHostTransportStateBlock
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, transportStateBlock)
    }
    unsafe fn setTransportStateBlock_(&self, transportStateBlock: AUHostTransportStateBlock)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setTransportStateBlock : transportStateBlock)
    }
    unsafe fn contextName(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, contextName)
    }
    unsafe fn setContextName_(&self, contextName: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setContextName : contextName)
    }
    unsafe fn migrateFromPlugin(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, migrateFromPlugin)
    }
    unsafe fn supportsMPE(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, supportsMPE)
    }
    unsafe fn channelMap(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, channelMap)
    }
    unsafe fn setChannelMap_(&self, channelMap: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setChannelMap : channelMap)
    }
    unsafe fn profileChangedBlock(&self) -> AUMIDICIProfileChangedBlock
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, profileChangedBlock)
    }
    unsafe fn setProfileChangedBlock_(&self, profileChangedBlock: AUMIDICIProfileChangedBlock)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setProfileChangedBlock : profileChangedBlock)
    }
    unsafe fn instantiateWithComponentDescription_options_completionHandler_(
        componentDescription: AudioComponentDescription,
        options: AudioComponentInstantiationOptions,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"AUAudioUnit").unwrap(), instantiateWithComponentDescription : componentDescription, options : options, completionHandler : completionHandler)
    }
}
pub type AUInputHandler = *mut ::std::os::raw::c_void;
impl AUAudioUnit_AUAudioInputOutputUnit for AUAudioUnit {}
pub trait AUAudioUnit_AUAudioInputOutputUnit: Sized + std::ops::Deref {
    unsafe fn setDeviceID_error_(&self, deviceID: AUAudioObjectID, outError: *mut NSError) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDeviceID : deviceID, error : outError)
    }
    unsafe fn startHardwareAndReturnError_(&self, outError: *mut NSError) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, startHardwareAndReturnError : outError)
    }
    unsafe fn stopHardware(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, stopHardware)
    }
    unsafe fn canPerformInput(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, canPerformInput)
    }
    unsafe fn canPerformOutput(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, canPerformOutput)
    }
    unsafe fn isInputEnabled(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isInputEnabled)
    }
    unsafe fn setInputEnabled_(&self, inputEnabled: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInputEnabled : inputEnabled)
    }
    unsafe fn isOutputEnabled(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isOutputEnabled)
    }
    unsafe fn setOutputEnabled_(&self, outputEnabled: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setOutputEnabled : outputEnabled)
    }
    unsafe fn outputProvider(&self) -> AURenderPullInputBlock
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, outputProvider)
    }
    unsafe fn setOutputProvider_(&self, outputProvider: AURenderPullInputBlock)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setOutputProvider : outputProvider)
    }
    unsafe fn inputHandler(&self) -> AUInputHandler
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inputHandler)
    }
    unsafe fn setInputHandler_(&self, inputHandler: AUInputHandler)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInputHandler : inputHandler)
    }
    unsafe fn deviceID(&self) -> AUAudioObjectID
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, deviceID)
    }
    unsafe fn deviceInputLatency(&self) -> NSTimeInterval
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, deviceInputLatency)
    }
    unsafe fn deviceOutputLatency(&self) -> NSTimeInterval
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, deviceOutputLatency)
    }
    unsafe fn isRunning(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isRunning)
    }
    unsafe fn osWorkgroup(&self) -> os_workgroup_t
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, osWorkgroup)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct AUAudioUnitBusArray(pub id);
impl std::ops::Deref for AUAudioUnitBusArray {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for AUAudioUnitBusArray {}
impl AUAudioUnitBusArray {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"AUAudioUnitBusArray").unwrap(), alloc) })
    }
}
impl PNSFastEnumeration for AUAudioUnitBusArray {}
impl INSObject for AUAudioUnitBusArray {}
impl PNSObject for AUAudioUnitBusArray {}
impl std::convert::TryFrom<NSObject> for AUAudioUnitBusArray {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<AUAudioUnitBusArray, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"AUAudioUnitBusArray").unwrap()) };
        if is_kind_of {
            Ok(AUAudioUnitBusArray(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to AUAudioUnitBusArray")
        }
    }
}
impl IAUAudioUnitBusArray for AUAudioUnitBusArray {}
pub trait IAUAudioUnitBusArray: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn initWithAudioUnit_busType_busses_(
        &self,
        owner: AUAudioUnit,
        busType: AUAudioUnitBusType,
        busArray: NSArray,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithAudioUnit : owner, busType : busType, busses : busArray)
    }
    unsafe fn initWithAudioUnit_busType_(
        &self,
        owner: AUAudioUnit,
        busType: AUAudioUnitBusType,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithAudioUnit : owner, busType : busType)
    }
    unsafe fn objectAtIndexedSubscript_(&self, index: NSUInteger) -> AUAudioUnitBus
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, objectAtIndexedSubscript : index)
    }
    unsafe fn setBusCount_error_(&self, count: NSUInteger, outError: *mut NSError) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setBusCount : count, error : outError)
    }
    unsafe fn addObserverToAllBusses_forKeyPath_options_context_(
        &self,
        observer: NSObject,
        keyPath: NSString,
        options: NSKeyValueObservingOptions,
        context: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, addObserverToAllBusses : observer, forKeyPath : keyPath, options : options, context : context)
    }
    unsafe fn removeObserverFromAllBusses_forKeyPath_context_(
        &self,
        observer: NSObject,
        keyPath: NSString,
        context: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, removeObserverFromAllBusses : observer, forKeyPath : keyPath, context : context)
    }
    unsafe fn count(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, count)
    }
    unsafe fn isCountChangeable(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isCountChangeable)
    }
    unsafe fn ownerAudioUnit(&self) -> AUAudioUnit
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, ownerAudioUnit)
    }
    unsafe fn busType(&self) -> AUAudioUnitBusType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, busType)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct AUAudioUnitBus(pub id);
impl std::ops::Deref for AUAudioUnitBus {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for AUAudioUnitBus {}
impl AUAudioUnitBus {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"AUAudioUnitBus").unwrap(), alloc) })
    }
}
impl INSObject for AUAudioUnitBus {}
impl PNSObject for AUAudioUnitBus {}
impl std::convert::TryFrom<NSObject> for AUAudioUnitBus {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<AUAudioUnitBus, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"AUAudioUnitBus").unwrap()) };
        if is_kind_of {
            Ok(AUAudioUnitBus(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to AUAudioUnitBus")
        }
    }
}
impl IAUAudioUnitBus for AUAudioUnitBus {}
pub trait IAUAudioUnitBus: Sized + std::ops::Deref {
    unsafe fn setFormat_error_(&self, format: AVAudioFormat, outError: *mut NSError) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setFormat : format, error : outError)
    }
    unsafe fn format(&self) -> AVAudioFormat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, format)
    }
    unsafe fn shouldAllocateBuffer(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, shouldAllocateBuffer)
    }
    unsafe fn setShouldAllocateBuffer_(&self, shouldAllocateBuffer: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setShouldAllocateBuffer : shouldAllocateBuffer)
    }
    unsafe fn isEnabled(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isEnabled)
    }
    unsafe fn setEnabled_(&self, enabled: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setEnabled : enabled)
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
    unsafe fn index(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, index)
    }
    unsafe fn busType(&self) -> AUAudioUnitBusType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, busType)
    }
    unsafe fn ownerAudioUnit(&self) -> AUAudioUnit
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, ownerAudioUnit)
    }
    unsafe fn supportedChannelLayoutTags(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, supportedChannelLayoutTags)
    }
    unsafe fn contextPresentationLatency(&self) -> NSTimeInterval
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, contextPresentationLatency)
    }
    unsafe fn setContextPresentationLatency_(&self, contextPresentationLatency: NSTimeInterval)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setContextPresentationLatency : contextPresentationLatency)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct AUAudioUnitPreset(pub id);
impl std::ops::Deref for AUAudioUnitPreset {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for AUAudioUnitPreset {}
impl AUAudioUnitPreset {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"AUAudioUnitPreset").unwrap(), alloc) })
    }
}
impl PNSSecureCoding for AUAudioUnitPreset {}
impl INSObject for AUAudioUnitPreset {}
impl PNSObject for AUAudioUnitPreset {}
impl std::convert::TryFrom<NSObject> for AUAudioUnitPreset {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<AUAudioUnitPreset, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"AUAudioUnitPreset").unwrap()) };
        if is_kind_of {
            Ok(AUAudioUnitPreset(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to AUAudioUnitPreset")
        }
    }
}
impl IAUAudioUnitPreset for AUAudioUnitPreset {}
pub trait IAUAudioUnitPreset: Sized + std::ops::Deref {
    unsafe fn number(&self) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, number)
    }
    unsafe fn setNumber_(&self, number: NSInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setNumber : number)
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
}
pub type CallHostBlock = *mut ::std::os::raw::c_void;
pub trait PAUMessageChannel: Sized + std::ops::Deref {
    unsafe fn callAudioUnit_(&self, message: NSDictionary) -> NSDictionary
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, callAudioUnit : message)
    }
    unsafe fn callHostBlock(&self) -> CallHostBlock
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, callHostBlock)
    }
    unsafe fn setCallHostBlock_(&self, callHostBlock: CallHostBlock)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCallHostBlock : callHostBlock)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CASpatialAudioExperience(pub id);
impl std::ops::Deref for CASpatialAudioExperience {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CASpatialAudioExperience {}
impl CASpatialAudioExperience {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CASpatialAudioExperience").unwrap(), alloc) })
    }
}
impl ICASpatialAudioExperience for CASpatialAudioExperience {}
pub trait ICASpatialAudioExperience: Sized + std::ops::Deref {}
impl AUAudioUnit_IntendedSpatialExperience for AUAudioUnit {}
pub trait AUAudioUnit_IntendedSpatialExperience: Sized + std::ops::Deref {
    unsafe fn intendedSpatialExperience(&self) -> CASpatialAudioExperience
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, intendedSpatialExperience)
    }
    unsafe fn setIntendedSpatialExperience_(
        &self,
        intendedSpatialExperience: CASpatialAudioExperience,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setIntendedSpatialExperience : intendedSpatialExperience)
    }
}
pub type AURenderEventType = u8;
#[repr(C, packed(4))]
#[derive(Debug, Copy, Clone)]
pub struct AURenderEventHeader {
    pub next: *mut AURenderEvent,
    pub eventSampleTime: AUEventSampleTime,
    pub eventType: AURenderEventType,
    pub reserved: u8,
}
#[repr(C, packed(4))]
#[derive(Debug, Copy, Clone)]
pub struct AUParameterEvent {
    pub next: *mut AURenderEvent,
    pub eventSampleTime: AUEventSampleTime,
    pub eventType: AURenderEventType,
    pub reserved: [u8; 3usize],
    pub rampDurationSampleFrames: AUAudioFrameCount,
    pub parameterAddress: AUParameterAddress,
    pub value: AUValue,
}
#[repr(C, packed(4))]
#[derive(Debug, Copy, Clone)]
pub struct AUMIDIEvent {
    pub next: *mut AURenderEvent,
    pub eventSampleTime: AUEventSampleTime,
    pub eventType: AURenderEventType,
    pub reserved: u8,
    pub length: u16,
    pub cable: u8,
    pub data: [u8; 3usize],
}
#[repr(C, packed(4))]
#[derive(Debug, Copy, Clone)]
pub struct AUMIDIEventList {
    pub next: *mut AURenderEvent,
    pub eventSampleTime: AUEventSampleTime,
    pub eventType: AURenderEventType,
    pub reserved: u8,
    pub cable: u8,
    pub eventList: MIDIEventList,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union AURenderEvent {
    pub head: AURenderEventHeader,
    pub parameter: AUParameterEvent,
    pub MIDI: AUMIDIEvent,
    pub MIDIEventsList: AUMIDIEventList,
}
pub type AUInternalRenderBlock = *mut ::std::os::raw::c_void;
impl AUAudioUnit_AUAudioUnitImplementation for AUAudioUnit {}
pub trait AUAudioUnit_AUAudioUnitImplementation: Sized + std::ops::Deref {
    unsafe fn shouldChangeToFormat_forBus_(
        &self,
        format: AVAudioFormat,
        bus: AUAudioUnitBus,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, shouldChangeToFormat : format, forBus : bus)
    }
    unsafe fn setRenderResourcesAllocated_(&self, flag: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRenderResourcesAllocated : flag)
    }
    unsafe fn internalRenderBlock(&self) -> AUInternalRenderBlock
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, internalRenderBlock)
    }
    unsafe fn renderContextObserver(&self) -> AURenderContextObserver
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, renderContextObserver)
    }
    unsafe fn MIDIOutputBufferSizeHint(&self) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, MIDIOutputBufferSizeHint)
    }
    unsafe fn setMIDIOutputBufferSizeHint_(&self, MIDIOutputBufferSizeHint: NSInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMIDIOutputBufferSizeHint : MIDIOutputBufferSizeHint)
    }
    unsafe fn registerSubclass_asComponentDescription_name_version_(
        cls: Class,
        componentDescription: AudioComponentDescription,
        name: NSString,
        version: UInt32,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"AUAudioUnit").unwrap(), registerSubclass : cls, asComponentDescription : componentDescription, name : name, version : version)
    }
}
impl AUAudioUnitBus_AUAudioUnitImplementation for AUAudioUnitBus {}
pub trait AUAudioUnitBus_AUAudioUnitImplementation: Sized + std::ops::Deref {
    unsafe fn initWithFormat_error_(
        &self,
        format: AVAudioFormat,
        outError: *mut NSError,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithFormat : format, error : outError)
    }
    unsafe fn supportedChannelCounts(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, supportedChannelCounts)
    }
    unsafe fn setSupportedChannelCounts_(&self, supportedChannelCounts: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSupportedChannelCounts : supportedChannelCounts)
    }
    unsafe fn maximumChannelCount(&self) -> AUAudioChannelCount
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, maximumChannelCount)
    }
    unsafe fn setMaximumChannelCount_(&self, maximumChannelCount: AUAudioChannelCount)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMaximumChannelCount : maximumChannelCount)
    }
}
impl AUAudioUnitBusArray_AUAudioUnitBusImplementation for AUAudioUnitBusArray {}
pub trait AUAudioUnitBusArray_AUAudioUnitBusImplementation: Sized + std::ops::Deref {
    unsafe fn replaceBusses_(&self, busArray: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, replaceBusses : busArray)
    }
}
impl AUParameterTree_Factory for AUParameterTree {}
pub trait AUParameterTree_Factory: Sized + std::ops::Deref {
    unsafe fn createParameterWithIdentifier_name_address_min_max_unit_unitName_flags_valueStrings_dependentParameters_(
        identifier: NSString,
        name: NSString,
        address: AUParameterAddress,
        min: AUValue,
        max: AUValue,
        unit: AudioUnitParameterUnit,
        unitName: NSString,
        flags: AudioUnitParameterOptions,
        valueStrings: NSArray,
        dependentParameters: NSArray,
    ) -> AUParameter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"AUParameterTree").unwrap(), createParameterWithIdentifier : identifier, name : name, address : address, min : min, max : max, unit : unit, unitName : unitName, flags : flags, valueStrings : valueStrings, dependentParameters : dependentParameters)
    }
    unsafe fn createGroupWithIdentifier_name_children_(
        identifier: NSString,
        name: NSString,
        children: NSArray,
    ) -> AUParameterGroup
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"AUParameterTree").unwrap(), createGroupWithIdentifier : identifier, name : name, children : children)
    }
    unsafe fn createGroupTemplate_(children: NSArray) -> AUParameterGroup
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"AUParameterTree").unwrap(), createGroupTemplate : children)
    }
    unsafe fn createGroupFromTemplate_identifier_name_addressOffset_(
        templateGroup: AUParameterGroup,
        identifier: NSString,
        name: NSString,
        addressOffset: AUParameterAddress,
    ) -> AUParameterGroup
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"AUParameterTree").unwrap(), createGroupFromTemplate : templateGroup, identifier : identifier, name : name, addressOffset : addressOffset)
    }
    unsafe fn createTreeWithChildren_(children: NSArray) -> AUParameterTree
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"AUParameterTree").unwrap(), createTreeWithChildren : children)
    }
}
pub type AUImplementorValueObserver = *mut ::std::os::raw::c_void;
pub type AUImplementorValueProvider = *mut ::std::os::raw::c_void;
pub type AUImplementorStringFromValueCallback = *mut ::std::os::raw::c_void;
pub type AUImplementorValueFromStringCallback = *mut ::std::os::raw::c_void;
pub type AUImplementorDisplayNameWithLengthCallback = *mut ::std::os::raw::c_void;
impl AUParameterNode_AUParameterNodeImplementation for AUParameterNode {}
pub trait AUParameterNode_AUParameterNodeImplementation: Sized + std::ops::Deref {
    unsafe fn implementorValueObserver(&self) -> AUImplementorValueObserver
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, implementorValueObserver)
    }
    unsafe fn setImplementorValueObserver_(
        &self,
        implementorValueObserver: AUImplementorValueObserver,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setImplementorValueObserver : implementorValueObserver)
    }
    unsafe fn implementorValueProvider(&self) -> AUImplementorValueProvider
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, implementorValueProvider)
    }
    unsafe fn setImplementorValueProvider_(
        &self,
        implementorValueProvider: AUImplementorValueProvider,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setImplementorValueProvider : implementorValueProvider)
    }
    unsafe fn implementorStringFromValueCallback(&self) -> AUImplementorStringFromValueCallback
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, implementorStringFromValueCallback)
    }
    unsafe fn setImplementorStringFromValueCallback_(
        &self,
        implementorStringFromValueCallback: AUImplementorStringFromValueCallback,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setImplementorStringFromValueCallback : implementorStringFromValueCallback)
    }
    unsafe fn implementorValueFromStringCallback(&self) -> AUImplementorValueFromStringCallback
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, implementorValueFromStringCallback)
    }
    unsafe fn setImplementorValueFromStringCallback_(
        &self,
        implementorValueFromStringCallback: AUImplementorValueFromStringCallback,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setImplementorValueFromStringCallback : implementorValueFromStringCallback)
    }
    unsafe fn implementorDisplayNameWithLengthCallback(
        &self,
    ) -> AUImplementorDisplayNameWithLengthCallback
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, implementorDisplayNameWithLengthCallback)
    }
    unsafe fn setImplementorDisplayNameWithLengthCallback_(
        &self,
        implementorDisplayNameWithLengthCallback: AUImplementorDisplayNameWithLengthCallback,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setImplementorDisplayNameWithLengthCallback : implementorDisplayNameWithLengthCallback)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct AUAudioUnitV2Bridge(pub id);
impl std::ops::Deref for AUAudioUnitV2Bridge {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for AUAudioUnitV2Bridge {}
impl AUAudioUnitV2Bridge {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"AUAudioUnitV2Bridge").unwrap(), alloc) })
    }
}
impl IAUAudioUnit for AUAudioUnitV2Bridge {}
impl From<AUAudioUnitV2Bridge> for AUAudioUnit {
    fn from(child: AUAudioUnitV2Bridge) -> AUAudioUnit {
        AUAudioUnit(child.0)
    }
}
impl std::convert::TryFrom<AUAudioUnit> for AUAudioUnitV2Bridge {
    type Error = &'static str;
    fn try_from(parent: AUAudioUnit) -> Result<AUAudioUnitV2Bridge, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"AUAudioUnitV2Bridge").unwrap()) };
        if is_kind_of {
            Ok(AUAudioUnitV2Bridge(parent.0))
        } else {
            Err("This AUAudioUnit cannot be downcasted to AUAudioUnitV2Bridge")
        }
    }
}
impl INSObject for AUAudioUnitV2Bridge {}
impl PNSObject for AUAudioUnitV2Bridge {}
impl IAUAudioUnitV2Bridge for AUAudioUnitV2Bridge {}
pub trait IAUAudioUnitV2Bridge: Sized + std::ops::Deref {
    unsafe fn audioUnit(&self) -> AudioUnit
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, audioUnit)
    }
}
pub trait PAUAudioUnitFactory: Sized + std::ops::Deref {
    unsafe fn createAudioUnitWithComponentDescription_error_(
        &self,
        desc: AudioComponentDescription,
        error: *mut NSError,
    ) -> AUAudioUnit
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, createAudioUnitWithComponentDescription : desc, error : error)
    }
}
pub type AUAudioMixRenderingStyle = UInt32;
pub type MusicDeviceInstrumentID = UInt32;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct MusicDeviceStdNoteParams {
    pub argCount: UInt32,
    pub mPitch: Float32,
    pub mVelocity: Float32,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct NoteParamsControlValue {
    pub mID: AudioUnitParameterID,
    pub mValue: AudioUnitParameterValue,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct MusicDeviceNoteParams {
    pub argCount: UInt32,
    pub mPitch: Float32,
    pub mVelocity: Float32,
    pub mControls: [NoteParamsControlValue; 1usize],
}
pub type MusicDeviceGroupID = UInt32;
pub type NoteInstanceID = UInt32;
pub type MusicDeviceComponent = AudioComponentInstance;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OpaqueAUGraph {
    _unused: [u8; 0],
}
pub type AUGraph = *mut OpaqueAUGraph;
pub type AUNode = SInt32;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AudioUnitNodeConnection {
    pub sourceNode: AUNode,
    pub sourceOutputNumber: UInt32,
    pub destNode: AUNode,
    pub destInputNumber: UInt32,
}
pub type AUNodeConnection = AudioUnitNodeConnection;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AUNodeRenderCallback {
    pub destNode: AUNode,
    pub destInputNumber: AudioUnitElement,
    pub cback: AURenderCallbackStruct,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct AUNodeInteraction {
    pub __bindgen_anon_1: AUNodeInteraction__bindgen_ty_1,
    pub nodeInteractionType: UInt32,
    pub nodeInteraction: AUNodeInteraction__bindgen_ty_1,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union AUNodeInteraction__bindgen_ty_1 {
    pub connection: AUNodeConnection,
    pub inputCallback: AUNodeRenderCallback,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OpaqueAudioConverter {
    _unused: [u8; 0],
}
pub type AudioConverterRef = *mut OpaqueAudioConverter;
pub type AudioConverterPropertyID = UInt32;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AudioConverterPrimeInfo {
    pub leadingFrames: UInt32,
    pub trailingFrames: UInt32,
}
pub type AudioConverterOptions = UInt32;
pub type AudioFileTypeID = UInt32;
pub type AudioFileFlags = UInt32;
pub type AudioFilePermissions = SInt8;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OpaqueAudioFileID {
    _unused: [u8; 0],
}
pub type AudioFileID = *mut OpaqueAudioFileID;
pub type AudioFilePropertyID = UInt32;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AudioFile_SMPTE_Time {
    pub mHours: SInt8,
    pub mMinutes: UInt8,
    pub mSeconds: UInt8,
    pub mFrames: UInt8,
    pub mSubFrameSampleOffset: UInt32,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AudioFileMarker {
    pub mFramePosition: Float64,
    pub mName: CFStringRef,
    pub mMarkerID: SInt32,
    pub mSMPTETime: AudioFile_SMPTE_Time,
    pub mType: UInt32,
    pub mReserved: UInt16,
    pub mChannel: UInt16,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AudioFileMarkerList {
    pub mSMPTE_TimeType: UInt32,
    pub mNumberMarkers: UInt32,
    pub mMarkers: [AudioFileMarker; 1usize],
}
pub type AudioFileRegionFlags = UInt32;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AudioFileRegion {
    pub mRegionID: UInt32,
    pub mName: CFStringRef,
    pub mFlags: AudioFileRegionFlags,
    pub mNumberMarkers: UInt32,
    pub mMarkers: [AudioFileMarker; 1usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AudioFileRegionList {
    pub mSMPTE_TimeType: UInt32,
    pub mNumberRegions: UInt32,
    pub mRegions: [AudioFileRegion; 1usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AudioFramePacketTranslation {
    pub mFrame: SInt64,
    pub mPacket: SInt64,
    pub mFrameOffsetInPacket: UInt32,
}
pub type AudioBytePacketTranslationFlags = UInt32;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AudioBytePacketTranslation {
    pub mByte: SInt64,
    pub mPacket: SInt64,
    pub mByteOffsetInPacket: UInt32,
    pub mFlags: AudioBytePacketTranslationFlags,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AudioFilePacketTableInfo {
    pub mNumberValidFrames: SInt64,
    pub mPrimingFrames: SInt32,
    pub mRemainderFrames: SInt32,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AudioPacketRangeByteCountTranslation {
    pub mPacket: SInt64,
    pub mPacketCount: SInt64,
    pub mByteCountUpperBound: SInt64,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AudioPacketRollDistanceTranslation {
    pub mPacket: SInt64,
    pub mRollDistance: SInt64,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AudioIndependentPacketTranslation {
    pub mPacket: SInt64,
    pub mIndependentlyDecodablePacket: SInt64,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AudioPacketDependencyInfoTranslation {
    pub mPacket: SInt64,
    pub mIsIndependentlyDecodable: UInt32,
    pub mNumberPrerollPackets: UInt32,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AudioFileTypeAndFormatID {
    pub mFileType: AudioFileTypeID,
    pub mFormatID: UInt32,
}
pub type AudioFileStreamPropertyFlags = UInt32;
pub type AudioFileStreamParseFlags = UInt32;
pub type AudioFileStreamSeekFlags = UInt32;
pub type AudioFileStreamPropertyID = UInt32;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OpaqueAudioFileStreamID {
    _unused: [u8; 0],
}
pub type AudioFileStreamID = *mut OpaqueAudioFileStreamID;
pub type AudioFormatPropertyID = UInt32;
pub type AudioPanningMode = UInt32;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AudioPanningInfo {
    pub mPanningMode: AudioPanningMode,
    pub mCoordinateFlags: UInt32,
    pub mCoordinates: [Float32; 3usize],
    pub mGainScale: Float32,
    pub mOutputChannelMap: *const AudioChannelLayout,
}
pub type AudioBalanceFadeType = UInt32;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AudioBalanceFade {
    pub mLeftRightBalance: Float32,
    pub mBackFrontFade: Float32,
    pub mType: AudioBalanceFadeType,
    pub mChannelLayout: *const AudioChannelLayout,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AudioFormatInfo {
    pub mASBD: AudioStreamBasicDescription,
    pub mMagicCookie: *const ::std::os::raw::c_void,
    pub mMagicCookieSize: UInt32,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ExtendedAudioFormatInfo {
    pub mASBD: AudioStreamBasicDescription,
    pub mMagicCookie: *const ::std::os::raw::c_void,
    pub mMagicCookieSize: UInt32,
    pub mClassDescription: AudioClassDescription,
}
pub type AudioQueuePropertyID = UInt32;
pub type AudioQueueParameterID = UInt32;
pub type AudioQueueParameterValue = Float32;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OpaqueAudioQueue {
    _unused: [u8; 0],
}
pub type AudioQueueRef = *mut OpaqueAudioQueue;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OpaqueAudioQueueTimeline {
    _unused: [u8; 0],
}
pub type AudioQueueTimelineRef = *mut OpaqueAudioQueueTimeline;
pub type AudioQueueProcessingTapFlags = UInt32;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AudioQueueBuffer {
    pub mAudioDataBytesCapacity: UInt32,
    pub mAudioData: *mut ::std::os::raw::c_void,
    pub mAudioDataByteSize: UInt32,
    pub mUserData: *mut ::std::os::raw::c_void,
    pub mPacketDescriptionCapacity: UInt32,
    pub mPacketDescriptions: *mut AudioStreamPacketDescription,
    pub mPacketDescriptionCount: UInt32,
}
pub type AudioQueueBufferRef = *mut AudioQueueBuffer;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AudioQueueParameterEvent {
    pub mID: AudioQueueParameterID,
    pub mValue: AudioQueueParameterValue,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AudioQueueLevelMeterState {
    pub mAveragePower: Float32,
    pub mPeakPower: Float32,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OpaqueAudioQueueProcessingTap {
    _unused: [u8; 0],
}
pub type AudioQueueProcessingTapRef = *mut OpaqueAudioQueueProcessingTap;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AudioQueueChannelAssignment {
    pub mDeviceUID: CFStringRef,
    pub mChannelNumber: UInt32,
}
pub type AudioQueueOutputCallbackBlock = *mut ::std::os::raw::c_void;
pub type AudioQueueInputCallbackBlock = *mut ::std::os::raw::c_void;
pub type AudioQueueOutputCallback = ::std::option::Option<
    unsafe extern "C" fn(
        inUserData: *mut ::std::os::raw::c_void,
        inAQ: AudioQueueRef,
        inBuffer: AudioQueueBufferRef,
    ),
>;
pub type AudioQueueInputCallback = ::std::option::Option<
    unsafe extern "C" fn(
        inUserData: *mut ::std::os::raw::c_void,
        inAQ: AudioQueueRef,
        inBuffer: AudioQueueBufferRef,
        inStartTime: *const AudioTimeStamp,
        inNumberPacketDescriptions: UInt32,
        inPacketDescs: *const AudioStreamPacketDescription,
    ),
>;
pub type AudioQueuePropertyListenerProc = ::std::option::Option<
    unsafe extern "C" fn(
        inUserData: *mut ::std::os::raw::c_void,
        inAQ: AudioQueueRef,
        inID: AudioQueuePropertyID,
    ),
>;
pub type AudioQueueProcessingTapCallback = ::std::option::Option<
    unsafe extern "C" fn(
        inClientData: *mut ::std::os::raw::c_void,
        inAQTap: AudioQueueProcessingTapRef,
        inNumberFrames: UInt32,
        ioTimeStamp: *mut AudioTimeStamp,
        ioFlags: *mut AudioQueueProcessingTapFlags,
        outNumberFrames: *mut UInt32,
        ioData: *mut AudioBufferList,
    ),
>;
pub type SystemSoundID = UInt32;
pub type AudioServicesPropertyID = UInt32;
pub type AudioServicesSystemSoundCompletionProc = ::std::option::Option<
    unsafe extern "C" fn(ssID: SystemSoundID, clientData: *mut ::std::os::raw::c_void),
>;
pub type AudioUnitEventType = UInt32;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AUListenerBase {
    _unused: [u8; 0],
}
pub type AUParameterListenerRef = *mut AUListenerBase;
pub type AUEventListenerRef = AUParameterListenerRef;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct AudioUnitEvent {
    pub __bindgen_anon_1: AudioUnitEvent__bindgen_ty_1,
    pub mEventType: AudioUnitEventType,
    pub mArgument: AudioUnitEvent__bindgen_ty_1,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union AudioUnitEvent__bindgen_ty_1 {
    pub mParameter: AudioUnitParameter,
    pub mProperty: AudioUnitProperty,
}
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct CAFFileHeader {
    pub mFileType: UInt32,
    pub mFileVersion: UInt16,
    pub mFileFlags: UInt16,
}
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct CAFChunkHeader {
    pub mChunkType: UInt32,
    pub mChunkSize: SInt64,
}
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct CAF_UUID_ChunkHeader {
    pub mHeader: CAFChunkHeader,
    pub mUUID: [UInt8; 16usize],
}
pub type CAFFormatFlags = UInt32;
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct CAFAudioDescription {
    pub mSampleRate: Float64,
    pub mFormatID: UInt32,
    pub mFormatFlags: CAFFormatFlags,
    pub mBytesPerPacket: UInt32,
    pub mFramesPerPacket: UInt32,
    pub mChannelsPerFrame: UInt32,
    pub mBitsPerChannel: UInt32,
}
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct CAFPacketTableHeader {
    pub mNumberPackets: SInt64,
    pub mNumberValidFrames: SInt64,
    pub mPrimingFrames: SInt32,
    pub mRemainderFrames: SInt32,
    pub mPacketDescriptions: [UInt8; 1usize],
}
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct CAFDataChunk {
    pub mEditCount: UInt32,
    pub mData: [UInt8; 1usize],
}
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct CAF_SMPTE_Time {
    pub mHours: SInt8,
    pub mMinutes: SInt8,
    pub mSeconds: SInt8,
    pub mFrames: SInt8,
    pub mSubFrameSampleOffset: UInt32,
}
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct CAFMarker {
    pub mType: UInt32,
    pub mFramePosition: Float64,
    pub mMarkerID: UInt32,
    pub mSMPTETime: CAF_SMPTE_Time,
    pub mChannel: UInt32,
}
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct CAFMarkerChunk {
    pub mSMPTE_TimeType: UInt32,
    pub mNumberMarkers: UInt32,
    pub mMarkers: [CAFMarker; 1usize],
}
pub type CAFRegionFlags = UInt32;
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct CAFRegion {
    pub mRegionID: UInt32,
    pub mFlags: CAFRegionFlags,
    pub mNumberMarkers: UInt32,
    pub mMarkers: [CAFMarker; 1usize],
}
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct CAFRegionChunk {
    pub mSMPTE_TimeType: UInt32,
    pub mNumberRegions: UInt32,
    pub mRegions: [CAFRegion; 1usize],
}
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct CAFInstrumentChunk {
    pub mBaseNote: Float32,
    pub mMIDILowNote: UInt8,
    pub mMIDIHighNote: UInt8,
    pub mMIDILowVelocity: UInt8,
    pub mMIDIHighVelocity: UInt8,
    pub mdBGain: Float32,
    pub mStartRegionID: UInt32,
    pub mSustainRegionID: UInt32,
    pub mReleaseRegionID: UInt32,
    pub mInstrumentID: UInt32,
}
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct CAFStringID {
    pub mStringID: UInt32,
    pub mStringStartByteOffset: SInt64,
}
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct CAFStrings {
    pub mNumEntries: UInt32,
    pub mStringsIDs: [CAFStringID; 1usize],
}
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct CAFInfoStrings {
    pub mNumEntries: UInt32,
}
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct CAFPositionPeak {
    pub mValue: Float32,
    pub mFrameNumber: UInt64,
}
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct CAFPeakChunk {
    pub mEditCount: UInt32,
    pub mPeaks: [CAFPositionPeak; 1usize],
}
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct CAFOverviewSample {
    pub mMinValue: SInt16,
    pub mMaxValue: SInt16,
}
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct CAFOverviewChunk {
    pub mEditCount: UInt32,
    pub mNumFramesPerOVWSample: UInt32,
    pub mData: [CAFOverviewSample; 1usize],
}
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct CAFUMIDChunk {
    pub mBytes: [UInt8; 64usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OpaqueExtAudioFile {
    _unused: [u8; 0],
}
pub type ExtAudioFileRef = *mut OpaqueExtAudioFile;
pub type ExtAudioFilePacketTableInfoOverride = SInt32;
pub type ExtAudioFilePropertyID = UInt32;
pub type MusicEventType = UInt32;
pub type MusicSequenceLoadFlags = UInt32;
pub type MusicSequenceType = UInt32;
pub type MusicSequenceFileTypeID = UInt32;
pub type MusicSequenceFileFlags = UInt32;
pub type MusicTimeStamp = Float64;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct MIDINoteMessage {
    pub channel: UInt8,
    pub note: UInt8,
    pub velocity: UInt8,
    pub releaseVelocity: UInt8,
    pub duration: Float32,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct MIDIChannelMessage {
    pub status: UInt8,
    pub data1: UInt8,
    pub data2: UInt8,
    pub reserved: UInt8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct MIDIRawData {
    pub length: UInt32,
    pub data: [UInt8; 1usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct MIDIMetaEvent {
    pub metaEventType: UInt8,
    pub unused1: UInt8,
    pub unused2: UInt8,
    pub unused3: UInt8,
    pub dataLength: UInt32,
    pub data: [UInt8; 1usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct MusicEventUserData {
    pub length: UInt32,
    pub data: [UInt8; 1usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ExtendedNoteOnEvent {
    pub instrumentID: MusicDeviceInstrumentID,
    pub groupID: MusicDeviceGroupID,
    pub duration: Float32,
    pub extendedParams: MusicDeviceNoteParams,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ParameterEvent {
    pub parameterID: AudioUnitParameterID,
    pub scope: AudioUnitScope,
    pub element: AudioUnitElement,
    pub value: AudioUnitParameterValue,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ExtendedTempoEvent {
    pub bpm: Float64,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AUPresetEvent {
    pub scope: AudioUnitScope,
    pub element: AudioUnitElement,
    pub preset: CFPropertyListRef,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CABarBeatTime {
    pub bar: SInt32,
    pub beat: UInt16,
    pub subbeat: UInt16,
    pub subbeatDivisor: UInt16,
    pub reserved: UInt16,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OpaqueMusicPlayer {
    _unused: [u8; 0],
}
pub type MusicPlayer = *mut OpaqueMusicPlayer;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OpaqueMusicSequence {
    _unused: [u8; 0],
}
pub type MusicSequence = *mut OpaqueMusicSequence;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OpaqueMusicTrack {
    _unused: [u8; 0],
}
pub type MusicTrack = *mut OpaqueMusicTrack;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OpaqueMusicEventIterator {
    _unused: [u8; 0],
}
pub type MusicEventIterator = *mut OpaqueMusicEventIterator;
pub type MusicSequenceUserCallback = ::std::option::Option<
    unsafe extern "C" fn(
        inClientData: *mut ::std::os::raw::c_void,
        inSequence: MusicSequence,
        inTrack: MusicTrack,
        inEventTime: MusicTimeStamp,
        inEventData: *const MusicEventUserData,
        inStartSliceBeat: MusicTimeStamp,
        inEndSliceBeat: MusicTimeStamp,
    ),
>;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct MusicTrackLoopInfo {
    pub loopDuration: MusicTimeStamp,
    pub numberOfLoops: SInt32,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ExtendedControlEvent {
    pub groupID: MusicDeviceGroupID,
    pub controlID: AudioUnitParameterID,
    pub value: AudioUnitParameterValue,
}
pub type AudioSessionPropertyID = UInt32;
pub type AudioSessionInterruptionType = UInt32;
pub type AudioSessionInterruptionListener = ::std::option::Option<
    unsafe extern "C" fn(inClientData: *mut ::std::os::raw::c_void, inInterruptionState: UInt32),
>;
pub type AudioSessionPropertyListener = ::std::option::Option<
    unsafe extern "C" fn(
        inClientData: *mut ::std::os::raw::c_void,
        inID: AudioSessionPropertyID,
        inDataSize: UInt32,
        inData: *const ::std::os::raw::c_void,
    ),
>;
unsafe extern "C" {
    pub fn NewAUGraph(outGraph: *mut AUGraph) -> OSStatus;
}
unsafe extern "C" {
    pub fn DisposeAUGraph(inGraph: AUGraph) -> OSStatus;
}
unsafe extern "C" {
    pub fn AUGraphAddNode(
        inGraph: AUGraph,
        inDescription: *const AudioComponentDescription,
        outNode: *mut AUNode,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn AUGraphRemoveNode(inGraph: AUGraph, inNode: AUNode) -> OSStatus;
}
unsafe extern "C" {
    pub fn AUGraphGetNodeCount(inGraph: AUGraph, outNumberOfNodes: *mut UInt32) -> OSStatus;
}
unsafe extern "C" {
    pub fn AUGraphGetIndNode(inGraph: AUGraph, inIndex: UInt32, outNode: *mut AUNode) -> OSStatus;
}
unsafe extern "C" {
    pub fn AUGraphNodeInfo(
        inGraph: AUGraph,
        inNode: AUNode,
        outDescription: *mut AudioComponentDescription,
        outAudioUnit: *mut AudioUnit,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn AUGraphConnectNodeInput(
        inGraph: AUGraph,
        inSourceNode: AUNode,
        inSourceOutputNumber: UInt32,
        inDestNode: AUNode,
        inDestInputNumber: UInt32,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn AUGraphSetNodeInputCallback(
        inGraph: AUGraph,
        inDestNode: AUNode,
        inDestInputNumber: UInt32,
        inInputCallback: *const AURenderCallbackStruct,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn AUGraphDisconnectNodeInput(
        inGraph: AUGraph,
        inDestNode: AUNode,
        inDestInputNumber: UInt32,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn AUGraphClearConnections(inGraph: AUGraph) -> OSStatus;
}
unsafe extern "C" {
    pub fn AUGraphGetNumberOfInteractions(
        inGraph: AUGraph,
        outNumInteractions: *mut UInt32,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn AUGraphGetInteractionInfo(
        inGraph: AUGraph,
        inInteractionIndex: UInt32,
        outInteraction: *mut AUNodeInteraction,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn AUGraphCountNodeInteractions(
        inGraph: AUGraph,
        inNode: AUNode,
        outNumInteractions: *mut UInt32,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn AUGraphGetNodeInteractions(
        inGraph: AUGraph,
        inNode: AUNode,
        ioNumInteractions: *mut UInt32,
        outInteractions: *mut AUNodeInteraction,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn AUGraphUpdate(inGraph: AUGraph, outIsUpdated: *mut Boolean) -> OSStatus;
}
unsafe extern "C" {
    pub fn AUGraphOpen(inGraph: AUGraph) -> OSStatus;
}
unsafe extern "C" {
    pub fn AUGraphClose(inGraph: AUGraph) -> OSStatus;
}
unsafe extern "C" {
    pub fn AUGraphInitialize(inGraph: AUGraph) -> OSStatus;
}
unsafe extern "C" {
    pub fn AUGraphUninitialize(inGraph: AUGraph) -> OSStatus;
}
unsafe extern "C" {
    pub fn AUGraphStart(inGraph: AUGraph) -> OSStatus;
}
unsafe extern "C" {
    pub fn AUGraphStop(inGraph: AUGraph) -> OSStatus;
}
unsafe extern "C" {
    pub fn AUGraphIsOpen(inGraph: AUGraph, outIsOpen: *mut Boolean) -> OSStatus;
}
unsafe extern "C" {
    pub fn AUGraphIsInitialized(inGraph: AUGraph, outIsInitialized: *mut Boolean) -> OSStatus;
}
unsafe extern "C" {
    pub fn AUGraphIsRunning(inGraph: AUGraph, outIsRunning: *mut Boolean) -> OSStatus;
}
unsafe extern "C" {
    pub fn AUGraphGetCPULoad(inGraph: AUGraph, outAverageCPULoad: *mut Float32) -> OSStatus;
}
unsafe extern "C" {
    pub fn AUGraphGetMaxCPULoad(inGraph: AUGraph, outMaxLoad: *mut Float32) -> OSStatus;
}
unsafe extern "C" {
    pub fn AUGraphAddRenderNotify(
        inGraph: AUGraph,
        inCallback: AURenderCallback,
        inRefCon: *mut ::std::os::raw::c_void,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn AUGraphRemoveRenderNotify(
        inGraph: AUGraph,
        inCallback: AURenderCallback,
        inRefCon: *mut ::std::os::raw::c_void,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn AudioQueueNewOutput(
        inFormat: *const AudioStreamBasicDescription,
        inCallbackProc: AudioQueueOutputCallback,
        inUserData: *mut ::std::os::raw::c_void,
        inCallbackRunLoop: CFRunLoopRef,
        inCallbackRunLoopMode: CFStringRef,
        inFlags: UInt32,
        outAQ: *mut AudioQueueRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn AudioQueueNewInput(
        inFormat: *const AudioStreamBasicDescription,
        inCallbackProc: AudioQueueInputCallback,
        inUserData: *mut ::std::os::raw::c_void,
        inCallbackRunLoop: CFRunLoopRef,
        inCallbackRunLoopMode: CFStringRef,
        inFlags: UInt32,
        outAQ: *mut AudioQueueRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn AudioQueueNewOutputWithDispatchQueue(
        outAQ: *mut AudioQueueRef,
        inFormat: *const AudioStreamBasicDescription,
        inFlags: UInt32,
        inCallbackDispatchQueue: NSObject,
        inCallbackBlock: AudioQueueOutputCallbackBlock,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn AudioQueueNewInputWithDispatchQueue(
        outAQ: *mut AudioQueueRef,
        inFormat: *const AudioStreamBasicDescription,
        inFlags: UInt32,
        inCallbackDispatchQueue: NSObject,
        inCallbackBlock: AudioQueueInputCallbackBlock,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn AudioQueueDispose(inAQ: AudioQueueRef, inImmediate: Boolean) -> OSStatus;
}
unsafe extern "C" {
    pub fn AudioQueueAllocateBuffer(
        inAQ: AudioQueueRef,
        inBufferByteSize: UInt32,
        outBuffer: *mut AudioQueueBufferRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn AudioQueueAllocateBufferWithPacketDescriptions(
        inAQ: AudioQueueRef,
        inBufferByteSize: UInt32,
        inNumberPacketDescriptions: UInt32,
        outBuffer: *mut AudioQueueBufferRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn AudioQueueFreeBuffer(inAQ: AudioQueueRef, inBuffer: AudioQueueBufferRef) -> OSStatus;
}
unsafe extern "C" {
    pub fn AudioQueueEnqueueBuffer(
        inAQ: AudioQueueRef,
        inBuffer: AudioQueueBufferRef,
        inNumPacketDescs: UInt32,
        inPacketDescs: *const AudioStreamPacketDescription,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn AudioQueueEnqueueBufferWithParameters(
        inAQ: AudioQueueRef,
        inBuffer: AudioQueueBufferRef,
        inNumPacketDescs: UInt32,
        inPacketDescs: *const AudioStreamPacketDescription,
        inTrimFramesAtStart: UInt32,
        inTrimFramesAtEnd: UInt32,
        inNumParamValues: UInt32,
        inParamValues: *const AudioQueueParameterEvent,
        inStartTime: *const AudioTimeStamp,
        outActualStartTime: *mut AudioTimeStamp,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn AudioQueueStart(inAQ: AudioQueueRef, inStartTime: *const AudioTimeStamp) -> OSStatus;
}
unsafe extern "C" {
    pub fn AudioQueuePrime(
        inAQ: AudioQueueRef,
        inNumberOfFramesToPrepare: UInt32,
        outNumberOfFramesPrepared: *mut UInt32,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn AudioQueueStop(inAQ: AudioQueueRef, inImmediate: Boolean) -> OSStatus;
}
unsafe extern "C" {
    pub fn AudioQueuePause(inAQ: AudioQueueRef) -> OSStatus;
}
unsafe extern "C" {
    pub fn AudioQueueFlush(inAQ: AudioQueueRef) -> OSStatus;
}
unsafe extern "C" {
    pub fn AudioQueueReset(inAQ: AudioQueueRef) -> OSStatus;
}
unsafe extern "C" {
    pub fn AudioQueueGetParameter(
        inAQ: AudioQueueRef,
        inParamID: AudioQueueParameterID,
        outValue: *mut AudioQueueParameterValue,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn AudioQueueSetParameter(
        inAQ: AudioQueueRef,
        inParamID: AudioQueueParameterID,
        inValue: AudioQueueParameterValue,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn AudioQueueGetProperty(
        inAQ: AudioQueueRef,
        inID: AudioQueuePropertyID,
        outData: *mut ::std::os::raw::c_void,
        ioDataSize: *mut UInt32,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn AudioQueueSetProperty(
        inAQ: AudioQueueRef,
        inID: AudioQueuePropertyID,
        inData: *const ::std::os::raw::c_void,
        inDataSize: UInt32,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn AudioQueueGetPropertySize(
        inAQ: AudioQueueRef,
        inID: AudioQueuePropertyID,
        outDataSize: *mut UInt32,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn AudioQueueAddPropertyListener(
        inAQ: AudioQueueRef,
        inID: AudioQueuePropertyID,
        inProc: AudioQueuePropertyListenerProc,
        inUserData: *mut ::std::os::raw::c_void,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn AudioQueueRemovePropertyListener(
        inAQ: AudioQueueRef,
        inID: AudioQueuePropertyID,
        inProc: AudioQueuePropertyListenerProc,
        inUserData: *mut ::std::os::raw::c_void,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn AudioQueueCreateTimeline(
        inAQ: AudioQueueRef,
        outTimeline: *mut AudioQueueTimelineRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn AudioQueueDisposeTimeline(
        inAQ: AudioQueueRef,
        inTimeline: AudioQueueTimelineRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn AudioQueueGetCurrentTime(
        inAQ: AudioQueueRef,
        inTimeline: AudioQueueTimelineRef,
        outTimeStamp: *mut AudioTimeStamp,
        outTimelineDiscontinuity: *mut Boolean,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn AudioQueueDeviceGetCurrentTime(
        inAQ: AudioQueueRef,
        outTimeStamp: *mut AudioTimeStamp,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn AudioQueueDeviceTranslateTime(
        inAQ: AudioQueueRef,
        inTime: *const AudioTimeStamp,
        outTime: *mut AudioTimeStamp,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn AudioQueueDeviceGetNearestStartTime(
        inAQ: AudioQueueRef,
        ioRequestedStartTime: *mut AudioTimeStamp,
        inFlags: UInt32,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn AudioQueueSetOfflineRenderFormat(
        inAQ: AudioQueueRef,
        inFormat: *const AudioStreamBasicDescription,
        inLayout: *const AudioChannelLayout,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn AudioQueueOfflineRender(
        inAQ: AudioQueueRef,
        inTimestamp: *const AudioTimeStamp,
        ioBuffer: AudioQueueBufferRef,
        inNumberFrames: UInt32,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn AudioQueueProcessingTapNew(
        inAQ: AudioQueueRef,
        inCallback: AudioQueueProcessingTapCallback,
        inClientData: *mut ::std::os::raw::c_void,
        inFlags: AudioQueueProcessingTapFlags,
        outMaxFrames: *mut UInt32,
        outProcessingFormat: *mut AudioStreamBasicDescription,
        outAQTap: *mut AudioQueueProcessingTapRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn AudioQueueProcessingTapDispose(inAQTap: AudioQueueProcessingTapRef) -> OSStatus;
}
unsafe extern "C" {
    pub fn AudioQueueProcessingTapGetSourceAudio(
        inAQTap: AudioQueueProcessingTapRef,
        inNumberFrames: UInt32,
        ioTimeStamp: *mut AudioTimeStamp,
        outFlags: *mut AudioQueueProcessingTapFlags,
        outNumberFrames: *mut UInt32,
        ioData: *mut AudioBufferList,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn AudioQueueProcessingTapGetQueueTime(
        inAQTap: AudioQueueProcessingTapRef,
        outQueueSampleTime: *mut Float64,
        outQueueFrameCount: *mut UInt32,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn AudioServicesCreateSystemSoundID(
        inFileURL: CFURLRef,
        outSystemSoundID: *mut SystemSoundID,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn AudioServicesDisposeSystemSoundID(inSystemSoundID: SystemSoundID) -> OSStatus;
}
unsafe extern "C" {
    pub fn AudioServicesPlayAlertSoundWithCompletion(
        inSystemSoundID: SystemSoundID,
        inCompletionBlock: *mut ::std::os::raw::c_void,
    );
}
unsafe extern "C" {
    pub fn AudioServicesPlaySystemSoundWithCompletion(
        inSystemSoundID: SystemSoundID,
        inCompletionBlock: *mut ::std::os::raw::c_void,
    );
}
unsafe extern "C" {
    pub fn AudioServicesGetPropertyInfo(
        inPropertyID: AudioServicesPropertyID,
        inSpecifierSize: UInt32,
        inSpecifier: *const ::std::os::raw::c_void,
        outPropertyDataSize: *mut UInt32,
        outWritable: *mut Boolean,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn AudioServicesGetProperty(
        inPropertyID: AudioServicesPropertyID,
        inSpecifierSize: UInt32,
        inSpecifier: *const ::std::os::raw::c_void,
        ioPropertyDataSize: *mut UInt32,
        outPropertyData: *mut ::std::os::raw::c_void,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn AudioServicesSetProperty(
        inPropertyID: AudioServicesPropertyID,
        inSpecifierSize: UInt32,
        inSpecifier: *const ::std::os::raw::c_void,
        inPropertyDataSize: UInt32,
        inPropertyData: *const ::std::os::raw::c_void,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn AudioServicesPlayAlertSound(inSystemSoundID: SystemSoundID);
}
unsafe extern "C" {
    pub fn AudioServicesPlaySystemSound(inSystemSoundID: SystemSoundID);
}
unsafe extern "C" {
    pub fn AudioServicesAddSystemSoundCompletion(
        inSystemSoundID: SystemSoundID,
        inRunLoop: CFRunLoopRef,
        inRunLoopMode: CFStringRef,
        inCompletionRoutine: AudioServicesSystemSoundCompletionProc,
        inClientData: *mut ::std::os::raw::c_void,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn AudioServicesRemoveSystemSoundCompletion(inSystemSoundID: SystemSoundID);
}
unsafe extern "C" {
    pub fn NewMusicPlayer(outPlayer: *mut MusicPlayer) -> OSStatus;
}
unsafe extern "C" {
    pub fn DisposeMusicPlayer(inPlayer: MusicPlayer) -> OSStatus;
}
unsafe extern "C" {
    pub fn MusicPlayerSetSequence(inPlayer: MusicPlayer, inSequence: MusicSequence) -> OSStatus;
}
unsafe extern "C" {
    pub fn MusicPlayerGetSequence(
        inPlayer: MusicPlayer,
        outSequence: *mut MusicSequence,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn MusicPlayerSetTime(inPlayer: MusicPlayer, inTime: MusicTimeStamp) -> OSStatus;
}
unsafe extern "C" {
    pub fn MusicPlayerGetTime(inPlayer: MusicPlayer, outTime: *mut MusicTimeStamp) -> OSStatus;
}
unsafe extern "C" {
    pub fn MusicPlayerGetHostTimeForBeats(
        inPlayer: MusicPlayer,
        inBeats: MusicTimeStamp,
        outHostTime: *mut UInt64,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn MusicPlayerGetBeatsForHostTime(
        inPlayer: MusicPlayer,
        inHostTime: UInt64,
        outBeats: *mut MusicTimeStamp,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn MusicPlayerPreroll(inPlayer: MusicPlayer) -> OSStatus;
}
unsafe extern "C" {
    pub fn MusicPlayerStart(inPlayer: MusicPlayer) -> OSStatus;
}
unsafe extern "C" {
    pub fn MusicPlayerStop(inPlayer: MusicPlayer) -> OSStatus;
}
unsafe extern "C" {
    pub fn MusicPlayerIsPlaying(inPlayer: MusicPlayer, outIsPlaying: *mut Boolean) -> OSStatus;
}
unsafe extern "C" {
    pub fn MusicPlayerSetPlayRateScalar(inPlayer: MusicPlayer, inScaleRate: Float64) -> OSStatus;
}
unsafe extern "C" {
    pub fn MusicPlayerGetPlayRateScalar(
        inPlayer: MusicPlayer,
        outScaleRate: *mut Float64,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn NewMusicSequence(outSequence: *mut MusicSequence) -> OSStatus;
}
unsafe extern "C" {
    pub fn DisposeMusicSequence(inSequence: MusicSequence) -> OSStatus;
}
unsafe extern "C" {
    pub fn MusicSequenceNewTrack(inSequence: MusicSequence, outTrack: *mut MusicTrack) -> OSStatus;
}
unsafe extern "C" {
    pub fn MusicSequenceDisposeTrack(inSequence: MusicSequence, inTrack: MusicTrack) -> OSStatus;
}
unsafe extern "C" {
    pub fn MusicSequenceGetTrackCount(
        inSequence: MusicSequence,
        outNumberOfTracks: *mut UInt32,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn MusicSequenceGetIndTrack(
        inSequence: MusicSequence,
        inTrackIndex: UInt32,
        outTrack: *mut MusicTrack,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn MusicSequenceGetTrackIndex(
        inSequence: MusicSequence,
        inTrack: MusicTrack,
        outTrackIndex: *mut UInt32,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn MusicSequenceGetTempoTrack(
        inSequence: MusicSequence,
        outTrack: *mut MusicTrack,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn MusicSequenceSetAUGraph(inSequence: MusicSequence, inGraph: AUGraph) -> OSStatus;
}
unsafe extern "C" {
    pub fn MusicSequenceGetAUGraph(inSequence: MusicSequence, outGraph: *mut AUGraph) -> OSStatus;
}
unsafe extern "C" {
    pub fn MusicSequenceSetMIDIEndpoint(
        inSequence: MusicSequence,
        inEndpoint: MIDIEndpointRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn MusicSequenceSetSequenceType(
        inSequence: MusicSequence,
        inType: MusicSequenceType,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn MusicSequenceGetSequenceType(
        inSequence: MusicSequence,
        outType: *mut MusicSequenceType,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn MusicSequenceFileLoad(
        inSequence: MusicSequence,
        inFileRef: CFURLRef,
        inFileTypeHint: MusicSequenceFileTypeID,
        inFlags: MusicSequenceLoadFlags,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn MusicSequenceFileLoadData(
        inSequence: MusicSequence,
        inData: CFDataRef,
        inFileTypeHint: MusicSequenceFileTypeID,
        inFlags: MusicSequenceLoadFlags,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn MusicSequenceFileCreate(
        inSequence: MusicSequence,
        inFileRef: CFURLRef,
        inFileType: MusicSequenceFileTypeID,
        inFlags: MusicSequenceFileFlags,
        inResolution: SInt16,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn MusicSequenceFileCreateData(
        inSequence: MusicSequence,
        inFileType: MusicSequenceFileTypeID,
        inFlags: MusicSequenceFileFlags,
        inResolution: SInt16,
        outData: *mut CFDataRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn MusicSequenceReverse(inSequence: MusicSequence) -> OSStatus;
}
unsafe extern "C" {
    pub fn MusicSequenceGetSecondsForBeats(
        inSequence: MusicSequence,
        inBeats: MusicTimeStamp,
        outSeconds: *mut Float64,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn MusicSequenceGetBeatsForSeconds(
        inSequence: MusicSequence,
        inSeconds: Float64,
        outBeats: *mut MusicTimeStamp,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn MusicSequenceSetUserCallback(
        inSequence: MusicSequence,
        inCallback: MusicSequenceUserCallback,
        inClientData: *mut ::std::os::raw::c_void,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn MusicSequenceBeatsToBarBeatTime(
        inSequence: MusicSequence,
        inBeats: MusicTimeStamp,
        inSubbeatDivisor: UInt32,
        outBarBeatTime: *mut CABarBeatTime,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn MusicSequenceBarBeatTimeToBeats(
        inSequence: MusicSequence,
        inBarBeatTime: *const CABarBeatTime,
        outBeats: *mut MusicTimeStamp,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn MusicSequenceGetInfoDictionary(inSequence: MusicSequence) -> CFDictionaryRef;
}
unsafe extern "C" {
    pub fn MusicTrackGetSequence(inTrack: MusicTrack, outSequence: *mut MusicSequence) -> OSStatus;
}
unsafe extern "C" {
    pub fn MusicTrackSetDestNode(inTrack: MusicTrack, inNode: AUNode) -> OSStatus;
}
unsafe extern "C" {
    pub fn MusicTrackSetDestMIDIEndpoint(
        inTrack: MusicTrack,
        inEndpoint: MIDIEndpointRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn MusicTrackGetDestNode(inTrack: MusicTrack, outNode: *mut AUNode) -> OSStatus;
}
unsafe extern "C" {
    pub fn MusicTrackGetDestMIDIEndpoint(
        inTrack: MusicTrack,
        outEndpoint: *mut MIDIEndpointRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn MusicTrackSetProperty(
        inTrack: MusicTrack,
        inPropertyID: UInt32,
        inData: *mut ::std::os::raw::c_void,
        inLength: UInt32,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn MusicTrackGetProperty(
        inTrack: MusicTrack,
        inPropertyID: UInt32,
        outData: *mut ::std::os::raw::c_void,
        ioLength: *mut UInt32,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn MusicTrackMoveEvents(
        inTrack: MusicTrack,
        inStartTime: MusicTimeStamp,
        inEndTime: MusicTimeStamp,
        inMoveTime: MusicTimeStamp,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn MusicTrackClear(
        inTrack: MusicTrack,
        inStartTime: MusicTimeStamp,
        inEndTime: MusicTimeStamp,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn MusicTrackCut(
        inTrack: MusicTrack,
        inStartTime: MusicTimeStamp,
        inEndTime: MusicTimeStamp,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn MusicTrackCopyInsert(
        inSourceTrack: MusicTrack,
        inSourceStartTime: MusicTimeStamp,
        inSourceEndTime: MusicTimeStamp,
        inDestTrack: MusicTrack,
        inDestInsertTime: MusicTimeStamp,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn MusicTrackMerge(
        inSourceTrack: MusicTrack,
        inSourceStartTime: MusicTimeStamp,
        inSourceEndTime: MusicTimeStamp,
        inDestTrack: MusicTrack,
        inDestInsertTime: MusicTimeStamp,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn MusicTrackNewMIDINoteEvent(
        inTrack: MusicTrack,
        inTimeStamp: MusicTimeStamp,
        inMessage: *const MIDINoteMessage,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn MusicTrackNewMIDIChannelEvent(
        inTrack: MusicTrack,
        inTimeStamp: MusicTimeStamp,
        inMessage: *const MIDIChannelMessage,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn MusicTrackNewMIDIRawDataEvent(
        inTrack: MusicTrack,
        inTimeStamp: MusicTimeStamp,
        inRawData: *const MIDIRawData,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn MusicTrackNewExtendedNoteEvent(
        inTrack: MusicTrack,
        inTimeStamp: MusicTimeStamp,
        inInfo: *const ExtendedNoteOnEvent,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn MusicTrackNewParameterEvent(
        inTrack: MusicTrack,
        inTimeStamp: MusicTimeStamp,
        inInfo: *const ParameterEvent,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn MusicTrackNewExtendedTempoEvent(
        inTrack: MusicTrack,
        inTimeStamp: MusicTimeStamp,
        inBPM: Float64,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn MusicTrackNewMetaEvent(
        inTrack: MusicTrack,
        inTimeStamp: MusicTimeStamp,
        inMetaEvent: *const MIDIMetaEvent,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn MusicTrackNewUserEvent(
        inTrack: MusicTrack,
        inTimeStamp: MusicTimeStamp,
        inUserData: *const MusicEventUserData,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn MusicTrackNewAUPresetEvent(
        inTrack: MusicTrack,
        inTimeStamp: MusicTimeStamp,
        inPresetEvent: *const AUPresetEvent,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn NewMusicEventIterator(
        inTrack: MusicTrack,
        outIterator: *mut MusicEventIterator,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn DisposeMusicEventIterator(inIterator: MusicEventIterator) -> OSStatus;
}
unsafe extern "C" {
    pub fn MusicEventIteratorSeek(
        inIterator: MusicEventIterator,
        inTimeStamp: MusicTimeStamp,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn MusicEventIteratorNextEvent(inIterator: MusicEventIterator) -> OSStatus;
}
unsafe extern "C" {
    pub fn MusicEventIteratorPreviousEvent(inIterator: MusicEventIterator) -> OSStatus;
}
unsafe extern "C" {
    pub fn MusicEventIteratorGetEventInfo(
        inIterator: MusicEventIterator,
        outTimeStamp: *mut MusicTimeStamp,
        outEventType: *mut MusicEventType,
        outEventData: *mut *const ::std::os::raw::c_void,
        outEventDataSize: *mut UInt32,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn MusicEventIteratorSetEventInfo(
        inIterator: MusicEventIterator,
        inEventType: MusicEventType,
        inEventData: *const ::std::os::raw::c_void,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn MusicEventIteratorSetEventTime(
        inIterator: MusicEventIterator,
        inTimeStamp: MusicTimeStamp,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn MusicEventIteratorDeleteEvent(inIterator: MusicEventIterator) -> OSStatus;
}
unsafe extern "C" {
    pub fn MusicEventIteratorHasPreviousEvent(
        inIterator: MusicEventIterator,
        outHasPrevEvent: *mut Boolean,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn MusicEventIteratorHasNextEvent(
        inIterator: MusicEventIterator,
        outHasNextEvent: *mut Boolean,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn MusicEventIteratorHasCurrentEvent(
        inIterator: MusicEventIterator,
        outHasCurEvent: *mut Boolean,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub static kAudioSession_RouteChangeKey_Reason: CFStringRef;
}
unsafe extern "C" {
    pub static kAudioSession_AudioRouteChangeKey_PreviousRouteDescription: CFStringRef;
}
unsafe extern "C" {
    pub static kAudioSession_AudioRouteChangeKey_CurrentRouteDescription: CFStringRef;
}
unsafe extern "C" {
    pub static kAudioSession_AudioRouteKey_Inputs: CFStringRef;
}
unsafe extern "C" {
    pub static kAudioSession_AudioRouteKey_Outputs: CFStringRef;
}
unsafe extern "C" {
    pub static kAudioSession_AudioRouteKey_Type: CFStringRef;
}
unsafe extern "C" {
    pub static kAudioSessionInputRoute_LineIn: CFStringRef;
}
unsafe extern "C" {
    pub static kAudioSessionInputRoute_BuiltInMic: CFStringRef;
}
unsafe extern "C" {
    pub static kAudioSessionInputRoute_HeadsetMic: CFStringRef;
}
unsafe extern "C" {
    pub static kAudioSessionInputRoute_BluetoothHFP: CFStringRef;
}
unsafe extern "C" {
    pub static kAudioSessionInputRoute_USBAudio: CFStringRef;
}
unsafe extern "C" {
    pub static kAudioSessionOutputRoute_LineOut: CFStringRef;
}
unsafe extern "C" {
    pub static kAudioSessionOutputRoute_Headphones: CFStringRef;
}
unsafe extern "C" {
    pub static kAudioSessionOutputRoute_BluetoothHFP: CFStringRef;
}
unsafe extern "C" {
    pub static kAudioSessionOutputRoute_BluetoothA2DP: CFStringRef;
}
unsafe extern "C" {
    pub static kAudioSessionOutputRoute_BuiltInReceiver: CFStringRef;
}
unsafe extern "C" {
    pub static kAudioSessionOutputRoute_BuiltInSpeaker: CFStringRef;
}
unsafe extern "C" {
    pub static kAudioSessionOutputRoute_USBAudio: CFStringRef;
}
unsafe extern "C" {
    pub static kAudioSessionOutputRoute_HDMI: CFStringRef;
}
unsafe extern "C" {
    pub static kAudioSessionOutputRoute_AirPlay: CFStringRef;
}
unsafe extern "C" {
    pub static kAudioSession_InputSourceKey_ID: CFStringRef;
}
unsafe extern "C" {
    pub static kAudioSession_InputSourceKey_Description: CFStringRef;
}
unsafe extern "C" {
    pub static kAudioSession_OutputDestinationKey_ID: CFStringRef;
}
unsafe extern "C" {
    pub static kAudioSession_OutputDestinationKey_Description: CFStringRef;
}
unsafe extern "C" {
    pub fn AudioSessionInitialize(
        inRunLoop: CFRunLoopRef,
        inRunLoopMode: CFStringRef,
        inInterruptionListener: AudioSessionInterruptionListener,
        inClientData: *mut ::std::os::raw::c_void,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn AudioSessionSetActive(active: Boolean) -> OSStatus;
}
unsafe extern "C" {
    pub fn AudioSessionSetActiveWithFlags(active: Boolean, inFlags: UInt32) -> OSStatus;
}
unsafe extern "C" {
    pub fn AudioSessionGetProperty(
        inID: AudioSessionPropertyID,
        ioDataSize: *mut UInt32,
        outData: *mut ::std::os::raw::c_void,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn AudioSessionSetProperty(
        inID: AudioSessionPropertyID,
        inDataSize: UInt32,
        inData: *const ::std::os::raw::c_void,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn AudioSessionGetPropertySize(
        inID: AudioSessionPropertyID,
        outDataSize: *mut UInt32,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn AudioSessionAddPropertyListener(
        inID: AudioSessionPropertyID,
        inProc: AudioSessionPropertyListener,
        inClientData: *mut ::std::os::raw::c_void,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn AudioSessionRemovePropertyListener(inID: AudioSessionPropertyID) -> OSStatus;
}
unsafe extern "C" {
    pub fn AudioSessionRemovePropertyListenerWithUserData(
        inID: AudioSessionPropertyID,
        inProc: AudioSessionPropertyListener,
        inClientData: *mut ::std::os::raw::c_void,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn CopyNameFromSoundBank(inURL: CFURLRef, outName: *mut CFStringRef) -> OSStatus;
}
unsafe extern "C" {
    pub fn CopyInstrumentInfoFromSoundBank(
        inURL: CFURLRef,
        outInstrumentInfo: *mut CFArrayRef,
    ) -> OSStatus;
}

unsafe impl objc2::encode::RefEncode for AudioStreamBasicDescription {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for AudioStreamBasicDescription {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("AudioStreamBasicDescription", &[]);
}
unsafe impl objc2::encode::RefEncode for AudioStreamPacketDescription {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for AudioStreamPacketDescription {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("AudioStreamPacketDescription", &[]);
}
unsafe impl objc2::encode::RefEncode for AudioClassDescription {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for AudioClassDescription {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("AudioClassDescription", &[]);
}
unsafe impl objc2::encode::RefEncode for AudioChannelDescription {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for AudioChannelDescription {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("AudioChannelDescription", &[]);
}
unsafe impl objc2::encode::RefEncode for AudioChannelLayout {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for AudioChannelLayout {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("AudioChannelLayout", &[]);
}
unsafe impl objc2::encode::RefEncode for ComponentInstanceRecord {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for ComponentInstanceRecord {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("ComponentInstanceRecord", &[]);
}
unsafe impl objc2::encode::RefEncode for AudioComponentDescription {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for AudioComponentDescription {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("AudioComponentDescription", &[]);
}
unsafe impl objc2::encode::RefEncode for OpaqueAudioComponent {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for OpaqueAudioComponent {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("OpaqueAudioComponent", &[]);
}
unsafe impl objc2::encode::RefEncode for AudioComponentPlugInInterface {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for AudioComponentPlugInInterface {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("AudioComponentPlugInInterface", &[]);
}
unsafe impl objc2::encode::RefEncode for AudioCodecMagicCookieInfo {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for AudioCodecMagicCookieInfo {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("AudioCodecMagicCookieInfo", &[]);
}
unsafe impl objc2::encode::RefEncode for AudioCodecPrimeInfo {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for AudioCodecPrimeInfo {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("AudioCodecPrimeInfo", &[]);
}
unsafe impl objc2::encode::RefEncode for AudioUnitParameterEvent {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for AudioUnitParameterEvent {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("AudioUnitParameterEvent", &[]);
}
unsafe impl objc2::encode::RefEncode for AudioUnitParameterEvent__bindgen_ty_1 {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for AudioUnitParameterEvent__bindgen_ty_1 {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("AudioUnitParameterEvent__bindgen_ty_1", &[]);
}
unsafe impl objc2::encode::RefEncode for AudioUnitParameterEvent__bindgen_ty_1__bindgen_ty_1 {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for AudioUnitParameterEvent__bindgen_ty_1__bindgen_ty_1 {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("AudioUnitParameterEvent__bindgen_ty_1__bindgen_ty_1", &[]);
}
unsafe impl objc2::encode::RefEncode for AudioUnitParameterEvent__bindgen_ty_1__bindgen_ty_2 {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for AudioUnitParameterEvent__bindgen_ty_1__bindgen_ty_2 {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("AudioUnitParameterEvent__bindgen_ty_1__bindgen_ty_2", &[]);
}
unsafe impl objc2::encode::RefEncode for AudioUnitParameter {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for AudioUnitParameter {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("AudioUnitParameter", &[]);
}
unsafe impl objc2::encode::RefEncode for AudioUnitProperty {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for AudioUnitProperty {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("AudioUnitProperty", &[]);
}
unsafe impl objc2::encode::RefEncode for AudioUnitConnection {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for AudioUnitConnection {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("AudioUnitConnection", &[]);
}
unsafe impl objc2::encode::RefEncode for AUChannelInfo {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for AUChannelInfo {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("AUChannelInfo", &[]);
}
unsafe impl objc2::encode::RefEncode for AudioUnitExternalBuffer {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for AudioUnitExternalBuffer {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("AudioUnitExternalBuffer", &[]);
}
unsafe impl objc2::encode::RefEncode for AURenderCallbackStruct {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for AURenderCallbackStruct {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("AURenderCallbackStruct", &[]);
}
unsafe impl objc2::encode::RefEncode for AUPreset {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for AUPreset {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("AUPreset", &[]);
}
unsafe impl objc2::encode::RefEncode for AudioUnitFrequencyResponseBin {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for AudioUnitFrequencyResponseBin {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("AudioUnitFrequencyResponseBin", &[]);
}
unsafe impl objc2::encode::RefEncode for HostCallbackInfo {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for HostCallbackInfo {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("HostCallbackInfo", &[]);
}
unsafe impl objc2::encode::RefEncode for AUDependentParameter {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for AUDependentParameter {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("AUDependentParameter", &[]);
}
unsafe impl objc2::encode::RefEncode for AudioUnitCocoaViewInfo {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for AudioUnitCocoaViewInfo {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("AudioUnitCocoaViewInfo", &[]);
}
unsafe impl objc2::encode::RefEncode for AUHostVersionIdentifier {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for AUHostVersionIdentifier {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("AUHostVersionIdentifier", &[]);
}
unsafe impl objc2::encode::RefEncode for AUMIDIOutputCallbackStruct {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for AUMIDIOutputCallbackStruct {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("AUMIDIOutputCallbackStruct", &[]);
}
unsafe impl objc2::encode::RefEncode for AUInputSamplesInOutputCallbackStruct {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for AUInputSamplesInOutputCallbackStruct {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("AUInputSamplesInOutputCallbackStruct", &[]);
}
unsafe impl objc2::encode::RefEncode for AudioUnitParameterHistoryInfo {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for AudioUnitParameterHistoryInfo {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("AudioUnitParameterHistoryInfo", &[]);
}
unsafe impl objc2::encode::RefEncode for AudioUnitRenderContext {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for AudioUnitRenderContext {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("AudioUnitRenderContext", &[]);
}
unsafe impl objc2::encode::RefEncode for AudioUnitParameterInfo {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for AudioUnitParameterInfo {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("AudioUnitParameterInfo", &[]);
}
unsafe impl objc2::encode::RefEncode for AudioUnitParameterNameInfo {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for AudioUnitParameterNameInfo {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("AudioUnitParameterNameInfo", &[]);
}
unsafe impl objc2::encode::RefEncode for AudioUnitParameterStringFromValue {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for AudioUnitParameterStringFromValue {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("AudioUnitParameterStringFromValue", &[]);
}
unsafe impl objc2::encode::RefEncode for AudioUnitParameterValueFromString {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for AudioUnitParameterValueFromString {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("AudioUnitParameterValueFromString", &[]);
}
unsafe impl objc2::encode::RefEncode for AUParameterMIDIMapping {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for AUParameterMIDIMapping {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("AUParameterMIDIMapping", &[]);
}
unsafe impl objc2::encode::RefEncode for AUDistanceAttenuationData {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for AUDistanceAttenuationData {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("AUDistanceAttenuationData", &[]);
}
unsafe impl objc2::encode::RefEncode for AUDistanceAttenuationData__bindgen_ty_1 {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for AUDistanceAttenuationData__bindgen_ty_1 {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("AUDistanceAttenuationData__bindgen_ty_1", &[]);
}
unsafe impl objc2::encode::RefEncode for AudioUnitOtherPluginDesc {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for AudioUnitOtherPluginDesc {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("AudioUnitOtherPluginDesc", &[]);
}
unsafe impl objc2::encode::RefEncode for AudioUnitParameterValueTranslation {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for AudioUnitParameterValueTranslation {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("AudioUnitParameterValueTranslation", &[]);
}
unsafe impl objc2::encode::RefEncode for AudioUnitPresetMAS_SettingData {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for AudioUnitPresetMAS_SettingData {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("AudioUnitPresetMAS_SettingData", &[]);
}
unsafe impl objc2::encode::RefEncode for AudioUnitPresetMAS_Settings {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for AudioUnitPresetMAS_Settings {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("AudioUnitPresetMAS_Settings", &[]);
}
unsafe impl objc2::encode::RefEncode for AudioOutputUnitMIDICallbacks {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for AudioOutputUnitMIDICallbacks {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("AudioOutputUnitMIDICallbacks", &[]);
}
unsafe impl objc2::encode::RefEncode for AudioOutputUnitStartAtTimeParams {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for AudioOutputUnitStartAtTimeParams {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("AudioOutputUnitStartAtTimeParams", &[]);
}
unsafe impl objc2::encode::RefEncode for AUVoiceIOOtherAudioDuckingConfiguration {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for AUVoiceIOOtherAudioDuckingConfiguration {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("AUVoiceIOOtherAudioDuckingConfiguration", &[]);
}
unsafe impl objc2::encode::RefEncode for AudioUnitMeterClipping {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for AudioUnitMeterClipping {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("AudioUnitMeterClipping", &[]);
}
unsafe impl objc2::encode::RefEncode for MixerDistanceParams {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MixerDistanceParams {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("MixerDistanceParams", &[]);
}
unsafe impl objc2::encode::RefEncode for ScheduledAudioSlice {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for ScheduledAudioSlice {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("ScheduledAudioSlice", &[]);
}
unsafe impl objc2::encode::RefEncode for ScheduledAudioFileRegion {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for ScheduledAudioFileRegion {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("ScheduledAudioFileRegion", &[]);
}
unsafe impl objc2::encode::RefEncode for AUSamplerInstrumentData {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for AUSamplerInstrumentData {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("AUSamplerInstrumentData", &[]);
}
unsafe impl objc2::encode::RefEncode for AUNumVersion {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for AUNumVersion {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("AUNumVersion", &[]);
}
unsafe impl objc2::encode::RefEncode for AUHostIdentifier {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for AUHostIdentifier {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("AUHostIdentifier", &[]);
}
unsafe impl objc2::encode::RefEncode for AudioUnitMIDIControlMapping {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for AudioUnitMIDIControlMapping {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("AudioUnitMIDIControlMapping", &[]);
}
unsafe impl objc2::encode::RefEncode for AudioUnitParameterValueName {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for AudioUnitParameterValueName {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("AudioUnitParameterValueName", &[]);
}
unsafe impl objc2::encode::RefEncode for AUSamplerBankPresetData {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for AUSamplerBankPresetData {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("AUSamplerBankPresetData", &[]);
}
unsafe impl objc2::encode::RefEncode for AURecordedParameterEvent {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for AURecordedParameterEvent {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("AURecordedParameterEvent", &[]);
}
unsafe impl objc2::encode::RefEncode for AUParameterAutomationEvent {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for AUParameterAutomationEvent {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("AUParameterAutomationEvent", &[]);
}
unsafe impl objc2::encode::RefEncode for AUParameterNode {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for AUParameterNode {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for AUParameterGroup {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for AUParameterGroup {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for AUParameterTree {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for AUParameterTree {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for AUParameter {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for AUParameter {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for AUAudioUnit {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for AUAudioUnit {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for AUAudioUnitBusArray {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for AUAudioUnitBusArray {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for AUAudioUnitBus {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for AUAudioUnitBus {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for AUAudioUnitPreset {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for AUAudioUnitPreset {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CASpatialAudioExperience {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CASpatialAudioExperience {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for AURenderEventHeader {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for AURenderEventHeader {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("AURenderEventHeader", &[]);
}
unsafe impl objc2::encode::RefEncode for AUParameterEvent {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for AUParameterEvent {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("AUParameterEvent", &[]);
}
unsafe impl objc2::encode::RefEncode for AUMIDIEvent {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for AUMIDIEvent {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("AUMIDIEvent", &[]);
}
unsafe impl objc2::encode::RefEncode for AUMIDIEventList {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for AUMIDIEventList {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("AUMIDIEventList", &[]);
}
unsafe impl objc2::encode::RefEncode for AURenderEvent {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for AURenderEvent {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("AURenderEvent", &[]);
}
unsafe impl objc2::encode::RefEncode for AUAudioUnitV2Bridge {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for AUAudioUnitV2Bridge {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MusicDeviceStdNoteParams {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MusicDeviceStdNoteParams {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("MusicDeviceStdNoteParams", &[]);
}
unsafe impl objc2::encode::RefEncode for NoteParamsControlValue {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for NoteParamsControlValue {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("NoteParamsControlValue", &[]);
}
unsafe impl objc2::encode::RefEncode for MusicDeviceNoteParams {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MusicDeviceNoteParams {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("MusicDeviceNoteParams", &[]);
}
unsafe impl objc2::encode::RefEncode for OpaqueAUGraph {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for OpaqueAUGraph {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("OpaqueAUGraph", &[]);
}
unsafe impl objc2::encode::RefEncode for AudioUnitNodeConnection {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for AudioUnitNodeConnection {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("AudioUnitNodeConnection", &[]);
}
unsafe impl objc2::encode::RefEncode for AUNodeRenderCallback {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for AUNodeRenderCallback {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("AUNodeRenderCallback", &[]);
}
unsafe impl objc2::encode::RefEncode for AUNodeInteraction {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for AUNodeInteraction {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("AUNodeInteraction", &[]);
}
unsafe impl objc2::encode::RefEncode for AUNodeInteraction__bindgen_ty_1 {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for AUNodeInteraction__bindgen_ty_1 {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("AUNodeInteraction__bindgen_ty_1", &[]);
}
unsafe impl objc2::encode::RefEncode for OpaqueAudioConverter {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for OpaqueAudioConverter {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("OpaqueAudioConverter", &[]);
}
unsafe impl objc2::encode::RefEncode for AudioConverterPrimeInfo {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for AudioConverterPrimeInfo {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("AudioConverterPrimeInfo", &[]);
}
unsafe impl objc2::encode::RefEncode for OpaqueAudioFileID {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for OpaqueAudioFileID {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("OpaqueAudioFileID", &[]);
}
unsafe impl objc2::encode::RefEncode for AudioFile_SMPTE_Time {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for AudioFile_SMPTE_Time {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("AudioFile_SMPTE_Time", &[]);
}
unsafe impl objc2::encode::RefEncode for AudioFileMarker {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for AudioFileMarker {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("AudioFileMarker", &[]);
}
unsafe impl objc2::encode::RefEncode for AudioFileMarkerList {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for AudioFileMarkerList {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("AudioFileMarkerList", &[]);
}
unsafe impl objc2::encode::RefEncode for AudioFileRegion {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for AudioFileRegion {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("AudioFileRegion", &[]);
}
unsafe impl objc2::encode::RefEncode for AudioFileRegionList {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for AudioFileRegionList {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("AudioFileRegionList", &[]);
}
unsafe impl objc2::encode::RefEncode for AudioFramePacketTranslation {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for AudioFramePacketTranslation {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("AudioFramePacketTranslation", &[]);
}
unsafe impl objc2::encode::RefEncode for AudioBytePacketTranslation {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for AudioBytePacketTranslation {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("AudioBytePacketTranslation", &[]);
}
unsafe impl objc2::encode::RefEncode for AudioFilePacketTableInfo {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for AudioFilePacketTableInfo {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("AudioFilePacketTableInfo", &[]);
}
unsafe impl objc2::encode::RefEncode for AudioPacketRangeByteCountTranslation {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for AudioPacketRangeByteCountTranslation {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("AudioPacketRangeByteCountTranslation", &[]);
}
unsafe impl objc2::encode::RefEncode for AudioPacketRollDistanceTranslation {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for AudioPacketRollDistanceTranslation {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("AudioPacketRollDistanceTranslation", &[]);
}
unsafe impl objc2::encode::RefEncode for AudioIndependentPacketTranslation {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for AudioIndependentPacketTranslation {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("AudioIndependentPacketTranslation", &[]);
}
unsafe impl objc2::encode::RefEncode for AudioPacketDependencyInfoTranslation {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for AudioPacketDependencyInfoTranslation {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("AudioPacketDependencyInfoTranslation", &[]);
}
unsafe impl objc2::encode::RefEncode for AudioFileTypeAndFormatID {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for AudioFileTypeAndFormatID {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("AudioFileTypeAndFormatID", &[]);
}
unsafe impl objc2::encode::RefEncode for OpaqueAudioFileStreamID {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for OpaqueAudioFileStreamID {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("OpaqueAudioFileStreamID", &[]);
}
unsafe impl objc2::encode::RefEncode for AudioPanningInfo {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for AudioPanningInfo {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("AudioPanningInfo", &[]);
}
unsafe impl objc2::encode::RefEncode for AudioBalanceFade {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for AudioBalanceFade {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("AudioBalanceFade", &[]);
}
unsafe impl objc2::encode::RefEncode for AudioFormatInfo {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for AudioFormatInfo {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("AudioFormatInfo", &[]);
}
unsafe impl objc2::encode::RefEncode for ExtendedAudioFormatInfo {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for ExtendedAudioFormatInfo {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("ExtendedAudioFormatInfo", &[]);
}
unsafe impl objc2::encode::RefEncode for OpaqueAudioQueue {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for OpaqueAudioQueue {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("OpaqueAudioQueue", &[]);
}
unsafe impl objc2::encode::RefEncode for OpaqueAudioQueueTimeline {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for OpaqueAudioQueueTimeline {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("OpaqueAudioQueueTimeline", &[]);
}
unsafe impl objc2::encode::RefEncode for AudioQueueBuffer {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for AudioQueueBuffer {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("AudioQueueBuffer", &[]);
}
unsafe impl objc2::encode::RefEncode for AudioQueueParameterEvent {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for AudioQueueParameterEvent {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("AudioQueueParameterEvent", &[]);
}
unsafe impl objc2::encode::RefEncode for AudioQueueLevelMeterState {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for AudioQueueLevelMeterState {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("AudioQueueLevelMeterState", &[]);
}
unsafe impl objc2::encode::RefEncode for OpaqueAudioQueueProcessingTap {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for OpaqueAudioQueueProcessingTap {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("OpaqueAudioQueueProcessingTap", &[]);
}
unsafe impl objc2::encode::RefEncode for AudioQueueChannelAssignment {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for AudioQueueChannelAssignment {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("AudioQueueChannelAssignment", &[]);
}
unsafe impl objc2::encode::RefEncode for AUListenerBase {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for AUListenerBase {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("AUListenerBase", &[]);
}
unsafe impl objc2::encode::RefEncode for AudioUnitEvent {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for AudioUnitEvent {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("AudioUnitEvent", &[]);
}
unsafe impl objc2::encode::RefEncode for AudioUnitEvent__bindgen_ty_1 {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for AudioUnitEvent__bindgen_ty_1 {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("AudioUnitEvent__bindgen_ty_1", &[]);
}
unsafe impl objc2::encode::RefEncode for CAFFileHeader {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CAFFileHeader {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("CAFFileHeader", &[]);
}
unsafe impl objc2::encode::RefEncode for CAFChunkHeader {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CAFChunkHeader {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("CAFChunkHeader", &[]);
}
unsafe impl objc2::encode::RefEncode for CAF_UUID_ChunkHeader {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CAF_UUID_ChunkHeader {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("CAF_UUID_ChunkHeader", &[]);
}
unsafe impl objc2::encode::RefEncode for CAFAudioDescription {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CAFAudioDescription {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("CAFAudioDescription", &[]);
}
unsafe impl objc2::encode::RefEncode for CAFPacketTableHeader {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CAFPacketTableHeader {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("CAFPacketTableHeader", &[]);
}
unsafe impl objc2::encode::RefEncode for CAFDataChunk {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CAFDataChunk {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("CAFDataChunk", &[]);
}
unsafe impl objc2::encode::RefEncode for CAF_SMPTE_Time {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CAF_SMPTE_Time {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("CAF_SMPTE_Time", &[]);
}
unsafe impl objc2::encode::RefEncode for CAFMarker {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CAFMarker {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("CAFMarker", &[]);
}
unsafe impl objc2::encode::RefEncode for CAFMarkerChunk {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CAFMarkerChunk {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("CAFMarkerChunk", &[]);
}
unsafe impl objc2::encode::RefEncode for CAFRegion {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CAFRegion {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("CAFRegion", &[]);
}
unsafe impl objc2::encode::RefEncode for CAFRegionChunk {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CAFRegionChunk {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("CAFRegionChunk", &[]);
}
unsafe impl objc2::encode::RefEncode for CAFInstrumentChunk {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CAFInstrumentChunk {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("CAFInstrumentChunk", &[]);
}
unsafe impl objc2::encode::RefEncode for CAFStringID {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CAFStringID {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("CAFStringID", &[]);
}
unsafe impl objc2::encode::RefEncode for CAFStrings {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CAFStrings {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("CAFStrings", &[]);
}
unsafe impl objc2::encode::RefEncode for CAFInfoStrings {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CAFInfoStrings {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("CAFInfoStrings", &[]);
}
unsafe impl objc2::encode::RefEncode for CAFPositionPeak {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CAFPositionPeak {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("CAFPositionPeak", &[]);
}
unsafe impl objc2::encode::RefEncode for CAFPeakChunk {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CAFPeakChunk {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("CAFPeakChunk", &[]);
}
unsafe impl objc2::encode::RefEncode for CAFOverviewSample {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CAFOverviewSample {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("CAFOverviewSample", &[]);
}
unsafe impl objc2::encode::RefEncode for CAFOverviewChunk {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CAFOverviewChunk {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("CAFOverviewChunk", &[]);
}
unsafe impl objc2::encode::RefEncode for CAFUMIDChunk {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CAFUMIDChunk {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("CAFUMIDChunk", &[]);
}
unsafe impl objc2::encode::RefEncode for OpaqueExtAudioFile {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for OpaqueExtAudioFile {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("OpaqueExtAudioFile", &[]);
}
unsafe impl objc2::encode::RefEncode for MIDINoteMessage {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MIDINoteMessage {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("MIDINoteMessage", &[]);
}
unsafe impl objc2::encode::RefEncode for MIDIChannelMessage {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MIDIChannelMessage {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("MIDIChannelMessage", &[]);
}
unsafe impl objc2::encode::RefEncode for MIDIRawData {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MIDIRawData {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("MIDIRawData", &[]);
}
unsafe impl objc2::encode::RefEncode for MIDIMetaEvent {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MIDIMetaEvent {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("MIDIMetaEvent", &[]);
}
unsafe impl objc2::encode::RefEncode for MusicEventUserData {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MusicEventUserData {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("MusicEventUserData", &[]);
}
unsafe impl objc2::encode::RefEncode for ExtendedNoteOnEvent {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for ExtendedNoteOnEvent {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("ExtendedNoteOnEvent", &[]);
}
unsafe impl objc2::encode::RefEncode for ParameterEvent {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for ParameterEvent {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("ParameterEvent", &[]);
}
unsafe impl objc2::encode::RefEncode for ExtendedTempoEvent {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for ExtendedTempoEvent {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("ExtendedTempoEvent", &[]);
}
unsafe impl objc2::encode::RefEncode for AUPresetEvent {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for AUPresetEvent {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("AUPresetEvent", &[]);
}
unsafe impl objc2::encode::RefEncode for CABarBeatTime {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CABarBeatTime {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("CABarBeatTime", &[]);
}
unsafe impl objc2::encode::RefEncode for OpaqueMusicPlayer {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for OpaqueMusicPlayer {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("OpaqueMusicPlayer", &[]);
}
unsafe impl objc2::encode::RefEncode for OpaqueMusicSequence {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for OpaqueMusicSequence {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("OpaqueMusicSequence", &[]);
}
unsafe impl objc2::encode::RefEncode for OpaqueMusicTrack {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for OpaqueMusicTrack {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("OpaqueMusicTrack", &[]);
}
unsafe impl objc2::encode::RefEncode for OpaqueMusicEventIterator {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for OpaqueMusicEventIterator {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("OpaqueMusicEventIterator", &[]);
}
unsafe impl objc2::encode::RefEncode for MusicTrackLoopInfo {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MusicTrackLoopInfo {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("MusicTrackLoopInfo", &[]);
}
unsafe impl objc2::encode::RefEncode for ExtendedControlEvent {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for ExtendedControlEvent {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("ExtendedControlEvent", &[]);
}
