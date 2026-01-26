#[allow(unused_imports)]
use crate::AudioToolbox::*;
#[allow(unused_imports)]
use crate::CoreAudio::*;
#[allow(unused_imports)]
use crate::CoreFoundation::*;

#[allow(unused_imports)]
use objc2::msg_send;
pub type AudioComponentFactoryFunction = ::std::option::Option<
    unsafe extern "C" fn(
        inDesc: *const AudioComponentDescription,
    ) -> *mut AudioComponentPlugInInterface,
>;
pub type AudioUnitPropertyListenerProc = ::std::option::Option<
    unsafe extern "C" fn(
        inRefCon: *mut ::std::os::raw::c_void,
        inUnit: AudioUnit,
        inID: AudioUnitPropertyID,
        inScope: AudioUnitScope,
        inElement: AudioUnitElement,
    ),
>;
unsafe extern "C" {
    pub fn AudioComponentFindNext(
        inComponent: AudioComponent,
        inDesc: *const AudioComponentDescription,
    ) -> AudioComponent;
}
unsafe extern "C" {
    pub fn AudioComponentCount(inDesc: *const AudioComponentDescription) -> UInt32;
}
unsafe extern "C" {
    pub fn AudioComponentCopyName(
        inComponent: AudioComponent,
        outName: *mut CFStringRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn AudioComponentGetDescription(
        inComponent: AudioComponent,
        outDesc: *mut AudioComponentDescription,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn AudioComponentGetVersion(
        inComponent: AudioComponent,
        outVersion: *mut UInt32,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn AudioComponentInstanceNew(
        inComponent: AudioComponent,
        outInstance: *mut AudioComponentInstance,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn AudioComponentInstanceDispose(inInstance: AudioComponentInstance) -> OSStatus;
}
unsafe extern "C" {
    pub fn AudioComponentInstanceGetComponent(inInstance: AudioComponentInstance)
        -> AudioComponent;
}
unsafe extern "C" {
    pub fn AudioComponentInstanceCanDo(
        inInstance: AudioComponentInstance,
        inSelectorID: SInt16,
    ) -> Boolean;
}
unsafe extern "C" {
    pub fn AudioComponentRegister(
        inDesc: *const AudioComponentDescription,
        inName: CFStringRef,
        inVersion: UInt32,
        inFactory: AudioComponentFactoryFunction,
    ) -> AudioComponent;
}
unsafe extern "C" {
    pub fn AudioCodecGetPropertyInfo(
        inCodec: AudioCodec,
        inPropertyID: AudioCodecPropertyID,
        outSize: *mut UInt32,
        outWritable: *mut Boolean,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn AudioCodecGetProperty(
        inCodec: AudioCodec,
        inPropertyID: AudioCodecPropertyID,
        ioPropertyDataSize: *mut UInt32,
        outPropertyData: *mut ::std::os::raw::c_void,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn AudioCodecSetProperty(
        inCodec: AudioCodec,
        inPropertyID: AudioCodecPropertyID,
        inPropertyDataSize: UInt32,
        inPropertyData: *const ::std::os::raw::c_void,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn AudioCodecInitialize(
        inCodec: AudioCodec,
        inInputFormat: *const AudioStreamBasicDescription,
        inOutputFormat: *const AudioStreamBasicDescription,
        inMagicCookie: *const ::std::os::raw::c_void,
        inMagicCookieByteSize: UInt32,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn AudioCodecUninitialize(inCodec: AudioCodec) -> OSStatus;
}
unsafe extern "C" {
    pub fn AudioCodecAppendInputData(
        inCodec: AudioCodec,
        inInputData: *const ::std::os::raw::c_void,
        ioInputDataByteSize: *mut UInt32,
        ioNumberPackets: *mut UInt32,
        inPacketDescription: *const AudioStreamPacketDescription,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn AudioCodecProduceOutputPackets(
        inCodec: AudioCodec,
        outOutputData: *mut ::std::os::raw::c_void,
        ioOutputDataByteSize: *mut UInt32,
        ioNumberPackets: *mut UInt32,
        outPacketDescription: *mut AudioStreamPacketDescription,
        outStatus: *mut UInt32,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn AudioCodecAppendInputBufferList(
        inCodec: AudioCodec,
        inBufferList: *const AudioBufferList,
        ioNumberPackets: *mut UInt32,
        inPacketDescription: *const AudioStreamPacketDescription,
        outBytesConsumed: *mut UInt32,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn AudioCodecProduceOutputBufferList(
        inCodec: AudioCodec,
        ioBufferList: *mut AudioBufferList,
        ioNumberPackets: *mut UInt32,
        outPacketDescription: *mut AudioStreamPacketDescription,
        outStatus: *mut UInt32,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn AudioCodecReset(inCodec: AudioCodec) -> OSStatus;
}
unsafe extern "C" {
    pub fn AudioUnitInitialize(inUnit: AudioUnit) -> OSStatus;
}
unsafe extern "C" {
    pub fn AudioUnitUninitialize(inUnit: AudioUnit) -> OSStatus;
}
unsafe extern "C" {
    pub fn AudioUnitGetPropertyInfo(
        inUnit: AudioUnit,
        inID: AudioUnitPropertyID,
        inScope: AudioUnitScope,
        inElement: AudioUnitElement,
        outDataSize: *mut UInt32,
        outWritable: *mut Boolean,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn AudioUnitGetProperty(
        inUnit: AudioUnit,
        inID: AudioUnitPropertyID,
        inScope: AudioUnitScope,
        inElement: AudioUnitElement,
        outData: *mut ::std::os::raw::c_void,
        ioDataSize: *mut UInt32,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn AudioUnitSetProperty(
        inUnit: AudioUnit,
        inID: AudioUnitPropertyID,
        inScope: AudioUnitScope,
        inElement: AudioUnitElement,
        inData: *const ::std::os::raw::c_void,
        inDataSize: UInt32,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn AudioUnitAddPropertyListener(
        inUnit: AudioUnit,
        inID: AudioUnitPropertyID,
        inProc: AudioUnitPropertyListenerProc,
        inProcUserData: *mut ::std::os::raw::c_void,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn AudioUnitRemovePropertyListenerWithUserData(
        inUnit: AudioUnit,
        inID: AudioUnitPropertyID,
        inProc: AudioUnitPropertyListenerProc,
        inProcUserData: *mut ::std::os::raw::c_void,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn AudioUnitAddRenderNotify(
        inUnit: AudioUnit,
        inProc: AURenderCallback,
        inProcUserData: *mut ::std::os::raw::c_void,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn AudioUnitRemoveRenderNotify(
        inUnit: AudioUnit,
        inProc: AURenderCallback,
        inProcUserData: *mut ::std::os::raw::c_void,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn AudioUnitGetParameter(
        inUnit: AudioUnit,
        inID: AudioUnitParameterID,
        inScope: AudioUnitScope,
        inElement: AudioUnitElement,
        outValue: *mut AudioUnitParameterValue,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn AudioUnitSetParameter(
        inUnit: AudioUnit,
        inID: AudioUnitParameterID,
        inScope: AudioUnitScope,
        inElement: AudioUnitElement,
        inValue: AudioUnitParameterValue,
        inBufferOffsetInFrames: UInt32,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn AudioUnitScheduleParameters(
        inUnit: AudioUnit,
        inParameterEvent: *const AudioUnitParameterEvent,
        inNumParamEvents: UInt32,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn AudioUnitRender(
        inUnit: AudioUnit,
        ioActionFlags: *mut AudioUnitRenderActionFlags,
        inTimeStamp: *const AudioTimeStamp,
        inOutputBusNumber: UInt32,
        inNumberFrames: UInt32,
        ioData: *mut AudioBufferList,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn AudioUnitProcess(
        inUnit: AudioUnit,
        ioActionFlags: *mut AudioUnitRenderActionFlags,
        inTimeStamp: *const AudioTimeStamp,
        inNumberFrames: UInt32,
        ioData: *mut AudioBufferList,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn AudioUnitProcessMultiple(
        inUnit: AudioUnit,
        ioActionFlags: *mut AudioUnitRenderActionFlags,
        inTimeStamp: *const AudioTimeStamp,
        inNumberFrames: UInt32,
        inNumberInputBufferLists: UInt32,
        inInputBufferLists: *mut *const AudioBufferList,
        inNumberOutputBufferLists: UInt32,
        ioOutputBufferLists: *mut *mut AudioBufferList,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn AudioUnitReset(
        inUnit: AudioUnit,
        inScope: AudioUnitScope,
        inElement: AudioUnitElement,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn AudioOutputUnitStart(ci: AudioUnit) -> OSStatus;
}
unsafe extern "C" {
    pub fn AudioOutputUnitStop(ci: AudioUnit) -> OSStatus;
}
unsafe extern "C" {
    pub fn MusicDeviceMIDIEvent(
        inUnit: MusicDeviceComponent,
        inStatus: UInt32,
        inData1: UInt32,
        inData2: UInt32,
        inOffsetSampleFrame: UInt32,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn MusicDeviceSysEx(
        inUnit: MusicDeviceComponent,
        inData: *const UInt8,
        inLength: UInt32,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn MusicDeviceStartNote(
        inUnit: MusicDeviceComponent,
        inInstrument: MusicDeviceInstrumentID,
        inGroupID: MusicDeviceGroupID,
        outNoteInstanceID: *mut NoteInstanceID,
        inOffsetSampleFrame: UInt32,
        inParams: *const MusicDeviceNoteParams,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn MusicDeviceStopNote(
        inUnit: MusicDeviceComponent,
        inGroupID: MusicDeviceGroupID,
        inNoteInstanceID: NoteInstanceID,
        inOffsetSampleFrame: UInt32,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn MusicDevicePrepareInstrument(
        inUnit: MusicDeviceComponent,
        inInstrument: MusicDeviceInstrumentID,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn MusicDeviceReleaseInstrument(
        inUnit: MusicDeviceComponent,
        inInstrument: MusicDeviceInstrumentID,
    ) -> OSStatus;
}
