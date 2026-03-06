#[allow(unused_imports)]
use crate::objc::*;
#[allow(unused_imports)]
use crate::CoreFoundation::*;
#[allow(unused_imports)]
use crate::CoreServices::*;
#[allow(unused_imports)]
use crate::Foundation::*;
#[allow(unused_imports)]
use crate::IOKit::*;

#[allow(unused_imports)]
use objc2::msg_send;
pub type MIDIObjectRef = UInt32;
pub type MIDIClientRef = MIDIObjectRef;
pub type MIDIPortRef = MIDIObjectRef;
pub type MIDIDeviceRef = MIDIObjectRef;
pub type MIDIEntityRef = MIDIObjectRef;
pub type MIDIEndpointRef = MIDIObjectRef;
pub type MIDITimeStamp = UInt64;
pub type MIDIObjectType = SInt32;
pub type MIDIUniqueID = SInt32;
pub type MIDIProtocolID = SInt32;
pub type MIDINotifyProc = ::std::option::Option<
    unsafe extern "C" fn(message: *const MIDINotification, refCon: *mut ::std::os::raw::c_void),
>;
pub type MIDINotifyBlock = *mut ::std::os::raw::c_void;
pub type MIDIReceiveBlock = *mut ::std::os::raw::c_void;
pub type MIDIReadProc = ::std::option::Option<
    unsafe extern "C" fn(
        pktlist: *const MIDIPacketList,
        readProcRefCon: *mut ::std::os::raw::c_void,
        srcConnRefCon: *mut ::std::os::raw::c_void,
    ),
>;
pub type MIDIReadBlock = *mut ::std::os::raw::c_void;
pub type MIDICompletionProc =
    ::std::option::Option<unsafe extern "C" fn(request: *mut MIDISysexSendRequest)>;
pub type MIDICompletionProcUMP =
    ::std::option::Option<unsafe extern "C" fn(request: *mut MIDISysexSendRequestUMP)>;
#[repr(C, packed(4))]
#[derive(Debug, Copy, Clone)]
pub struct MIDIEventPacket {
    pub timeStamp: MIDITimeStamp,
    pub wordCount: UInt32,
    pub words: [UInt32; 64usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct MIDIEventList {
    pub protocol: MIDIProtocolID,
    pub numPackets: UInt32,
    pub packet: [MIDIEventPacket; 1usize],
}
#[repr(C, packed(4))]
#[derive(Debug, Copy, Clone)]
pub struct MIDIPacket {
    pub timeStamp: MIDITimeStamp,
    pub length: UInt16,
    pub data: [Byte; 256usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct MIDIPacketList {
    pub numPackets: UInt32,
    pub packet: [MIDIPacket; 1usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct MIDISysexSendRequest {
    pub destination: MIDIEndpointRef,
    pub data: *const Byte,
    pub bytesToSend: UInt32,
    pub complete: Boolean,
    pub reserved: [Byte; 3usize],
    pub completionProc: MIDICompletionProc,
    pub completionRefCon: *mut ::std::os::raw::c_void,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct MIDISysexSendRequestUMP {
    pub destination: MIDIEndpointRef,
    pub words: *mut UInt32,
    pub wordsToSend: UInt32,
    pub complete: Boolean,
    pub completionProc: MIDICompletionProcUMP,
    pub completionRefCon: *mut ::std::os::raw::c_void,
}
pub type MIDINotificationMessageID = SInt32;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct MIDINotification {
    pub messageID: MIDINotificationMessageID,
    pub messageSize: UInt32,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct MIDIObjectAddRemoveNotification {
    pub messageID: MIDINotificationMessageID,
    pub messageSize: UInt32,
    pub parent: MIDIObjectRef,
    pub parentType: MIDIObjectType,
    pub child: MIDIObjectRef,
    pub childType: MIDIObjectType,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct MIDIObjectPropertyChangeNotification {
    pub messageID: MIDINotificationMessageID,
    pub messageSize: UInt32,
    pub object: MIDIObjectRef,
    pub objectType: MIDIObjectType,
    pub propertyName: CFStringRef,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct MIDIIOErrorNotification {
    pub messageID: MIDINotificationMessageID,
    pub messageSize: UInt32,
    pub driverDevice: MIDIDeviceRef,
    pub errorCode: OSStatus,
}
pub type MIDISetupRef = MIDIObjectRef;
pub type MIDIThruConnectionRef = MIDIObjectRef;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct MIDIValueMap {
    pub value: [UInt8; 128usize],
}
pub type MIDITransformType = UInt16;
pub type MIDITransformControlType = UInt8;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct MIDITransform {
    pub transform: MIDITransformType,
    pub param: SInt16,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct MIDIControlTransform {
    pub controlType: MIDITransformControlType,
    pub remappedControlType: MIDITransformControlType,
    pub controlNumber: UInt16,
    pub transform: MIDITransformType,
    pub param: SInt16,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct MIDIThruConnectionEndpoint {
    pub endpointRef: MIDIEndpointRef,
    pub uniqueID: MIDIUniqueID,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct MIDIThruConnectionParams {
    pub version: UInt32,
    pub numSources: UInt32,
    pub sources: [MIDIThruConnectionEndpoint; 8usize],
    pub numDestinations: UInt32,
    pub destinations: [MIDIThruConnectionEndpoint; 8usize],
    pub channelMap: [UInt8; 16usize],
    pub lowVelocity: UInt8,
    pub highVelocity: UInt8,
    pub lowNote: UInt8,
    pub highNote: UInt8,
    pub noteNumber: MIDITransform,
    pub velocity: MIDITransform,
    pub keyPressure: MIDITransform,
    pub channelPressure: MIDITransform,
    pub programChange: MIDITransform,
    pub pitchBend: MIDITransform,
    pub filterOutSysEx: UInt8,
    pub filterOutMTC: UInt8,
    pub filterOutBeatClock: UInt8,
    pub filterOutTuneRequest: UInt8,
    pub reserved2: [UInt8; 3usize],
    pub filterOutAllControls: UInt8,
    pub numControlTransforms: UInt16,
    pub numMaps: UInt16,
    pub reserved3: [UInt16; 4usize],
}
pub type MIDIDriverRef = *mut *mut MIDIDriverInterface;
pub type MIDIDeviceListRef = MIDIObjectRef;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct MIDIDriverInterface {
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
    pub FindDevices: ::std::option::Option<
        unsafe extern "C" fn(self_: MIDIDriverRef, devList: MIDIDeviceListRef) -> OSStatus,
    >,
    pub Start: ::std::option::Option<
        unsafe extern "C" fn(self_: MIDIDriverRef, devList: MIDIDeviceListRef) -> OSStatus,
    >,
    pub Stop: ::std::option::Option<unsafe extern "C" fn(self_: MIDIDriverRef) -> OSStatus>,
    pub Configure: ::std::option::Option<
        unsafe extern "C" fn(self_: MIDIDriverRef, device: MIDIDeviceRef) -> OSStatus,
    >,
    pub Send: ::std::option::Option<
        unsafe extern "C" fn(
            self_: MIDIDriverRef,
            pktlist: *const MIDIPacketList,
            destRefCon1: *mut ::std::os::raw::c_void,
            destRefCon2: *mut ::std::os::raw::c_void,
        ) -> OSStatus,
    >,
    pub EnableSource: ::std::option::Option<
        unsafe extern "C" fn(
            self_: MIDIDriverRef,
            src: MIDIEndpointRef,
            enabled: Boolean,
        ) -> OSStatus,
    >,
    pub Flush: ::std::option::Option<
        unsafe extern "C" fn(
            self_: MIDIDriverRef,
            dest: MIDIEndpointRef,
            destRefCon1: *mut ::std::os::raw::c_void,
            destRefCon2: *mut ::std::os::raw::c_void,
        ) -> OSStatus,
    >,
    pub Monitor: ::std::option::Option<
        unsafe extern "C" fn(
            self_: MIDIDriverRef,
            dest: MIDIEndpointRef,
            pktlist: *const MIDIPacketList,
        ) -> OSStatus,
    >,
    pub SendPackets: ::std::option::Option<
        unsafe extern "C" fn(
            self_: MIDIDriverRef,
            pktlist: *const MIDIEventList,
            destRefCon1: *mut ::std::os::raw::c_void,
            destRefCon2: *mut ::std::os::raw::c_void,
        ) -> OSStatus,
    >,
    pub MonitorEvents: ::std::option::Option<
        unsafe extern "C" fn(
            self_: MIDIDriverRef,
            dest: MIDIEndpointRef,
            pktlist: *const MIDIEventList,
        ) -> OSStatus,
    >,
}
pub type MIDIMessageType = ::std::os::raw::c_uint;
pub type MIDICVStatus = ::std::os::raw::c_uint;
pub type MIDISystemStatus = ::std::os::raw::c_uint;
pub type MIDISysExStatus = ::std::os::raw::c_uint;
pub type MIDIUtilityStatus = ::std::os::raw::c_uint;
pub type MIDINoteAttribute = UInt8;
pub type MIDIProgramChangeOptions = UInt8;
pub type MIDIPerNoteManagementOptions = UInt8;
pub type MIDIUMPFunctionBlockMIDI1Info = SInt32;
pub type MIDIUMPFunctionBlockUIHint = SInt32;
pub type MIDIUMPFunctionBlockDirection = SInt32;
pub type UMPStreamMessageFormat = UInt8;
pub type MIDIUInteger2 = UInt8;
pub type MIDIUInteger4 = UInt8;
pub type MIDIUInteger7 = UInt8;
pub type MIDIUInteger14 = UInt16;
pub type MIDIUInteger28 = UInt32;
pub type MIDIUMPGroupNumber = MIDIUInteger4;
pub type MIDIChannelNumber = MIDIUInteger4;
pub type MIDICIDeviceID = MIDIUInteger7;
pub type MIDICIMUID = MIDIUInteger28;
pub type MIDIMessage_32 = UInt32;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct MIDIMessage_64 {
    pub word0: UInt32,
    pub word1: UInt32,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct MIDIMessage_96 {
    pub word0: UInt32,
    pub word1: UInt32,
    pub word2: UInt32,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct MIDIMessage_128 {
    pub word0: UInt32,
    pub word1: UInt32,
    pub word2: UInt32,
    pub word3: UInt32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct MIDIUniversalMessage {
    pub __bindgen_anon_1: MIDIUniversalMessage__bindgen_ty_1,
    pub type_: MIDIMessageType,
    pub group: UInt8,
    pub reserved: [UInt8; 3usize],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union MIDIUniversalMessage__bindgen_ty_1 {
    pub __bindgen_anon_1: MIDIUniversalMessage__bindgen_ty_1__bindgen_ty_1,
    pub __bindgen_anon_2: MIDIUniversalMessage__bindgen_ty_1__bindgen_ty_2,
    pub __bindgen_anon_3: MIDIUniversalMessage__bindgen_ty_1__bindgen_ty_3,
    pub __bindgen_anon_4: MIDIUniversalMessage__bindgen_ty_1__bindgen_ty_4,
    pub __bindgen_anon_5: MIDIUniversalMessage__bindgen_ty_1__bindgen_ty_5,
    pub __bindgen_anon_6: MIDIUniversalMessage__bindgen_ty_1__bindgen_ty_6,
    pub __bindgen_anon_7: MIDIUniversalMessage__bindgen_ty_1__bindgen_ty_7,
    pub utility: MIDIUniversalMessage__bindgen_ty_1__bindgen_ty_1,
    pub system: MIDIUniversalMessage__bindgen_ty_1__bindgen_ty_2,
    pub channelVoice1: MIDIUniversalMessage__bindgen_ty_1__bindgen_ty_3,
    pub sysEx: MIDIUniversalMessage__bindgen_ty_1__bindgen_ty_4,
    pub channelVoice2: MIDIUniversalMessage__bindgen_ty_1__bindgen_ty_5,
    pub data128: MIDIUniversalMessage__bindgen_ty_1__bindgen_ty_6,
    pub unknown: MIDIUniversalMessage__bindgen_ty_1__bindgen_ty_7,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct MIDIUniversalMessage__bindgen_ty_1__bindgen_ty_1 {
    pub __bindgen_anon_1: MIDIUniversalMessage__bindgen_ty_1__bindgen_ty_1__bindgen_ty_1,
    pub status: MIDIUtilityStatus,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union MIDIUniversalMessage__bindgen_ty_1__bindgen_ty_1__bindgen_ty_1 {
    pub jitterReductionClock: UInt16,
    pub jitterReductionTimestamp: UInt16,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct MIDIUniversalMessage__bindgen_ty_1__bindgen_ty_2 {
    pub __bindgen_anon_1: MIDIUniversalMessage__bindgen_ty_1__bindgen_ty_2__bindgen_ty_1,
    pub status: MIDISystemStatus,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union MIDIUniversalMessage__bindgen_ty_1__bindgen_ty_2__bindgen_ty_1 {
    pub timeCode: UInt8,
    pub songPositionPointer: UInt16,
    pub songSelect: UInt8,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct MIDIUniversalMessage__bindgen_ty_1__bindgen_ty_3 {
    pub __bindgen_anon_1: MIDIUniversalMessage__bindgen_ty_1__bindgen_ty_3__bindgen_ty_1,
    pub status: MIDICVStatus,
    pub channel: UInt8,
    pub reserved: [UInt8; 3usize],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union MIDIUniversalMessage__bindgen_ty_1__bindgen_ty_3__bindgen_ty_1 {
    pub __bindgen_anon_1:
        MIDIUniversalMessage__bindgen_ty_1__bindgen_ty_3__bindgen_ty_1__bindgen_ty_1,
    pub __bindgen_anon_2:
        MIDIUniversalMessage__bindgen_ty_1__bindgen_ty_3__bindgen_ty_1__bindgen_ty_2,
    pub __bindgen_anon_3:
        MIDIUniversalMessage__bindgen_ty_1__bindgen_ty_3__bindgen_ty_1__bindgen_ty_3,
    pub note: MIDIUniversalMessage__bindgen_ty_1__bindgen_ty_3__bindgen_ty_1__bindgen_ty_1,
    pub polyPressure: MIDIUniversalMessage__bindgen_ty_1__bindgen_ty_3__bindgen_ty_1__bindgen_ty_2,
    pub controlChange: MIDIUniversalMessage__bindgen_ty_1__bindgen_ty_3__bindgen_ty_1__bindgen_ty_3,
    pub program: UInt8,
    pub channelPressure: UInt8,
    pub pitchBend: UInt16,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct MIDIUniversalMessage__bindgen_ty_1__bindgen_ty_3__bindgen_ty_1__bindgen_ty_1 {
    pub number: UInt8,
    pub velocity: UInt8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct MIDIUniversalMessage__bindgen_ty_1__bindgen_ty_3__bindgen_ty_1__bindgen_ty_2 {
    pub noteNumber: UInt8,
    pub pressure: UInt8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct MIDIUniversalMessage__bindgen_ty_1__bindgen_ty_3__bindgen_ty_1__bindgen_ty_3 {
    pub index: UInt8,
    pub data: UInt8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct MIDIUniversalMessage__bindgen_ty_1__bindgen_ty_4 {
    pub status: MIDISysExStatus,
    pub channel: UInt8,
    pub data: [UInt8; 6usize],
    pub reserved: UInt8,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct MIDIUniversalMessage__bindgen_ty_1__bindgen_ty_5 {
    pub __bindgen_anon_1: MIDIUniversalMessage__bindgen_ty_1__bindgen_ty_5__bindgen_ty_1,
    pub status: MIDICVStatus,
    pub channel: UInt8,
    pub reserved: [UInt8; 3usize],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union MIDIUniversalMessage__bindgen_ty_1__bindgen_ty_5__bindgen_ty_1 {
    pub __bindgen_anon_1:
        MIDIUniversalMessage__bindgen_ty_1__bindgen_ty_5__bindgen_ty_1__bindgen_ty_1,
    pub __bindgen_anon_2:
        MIDIUniversalMessage__bindgen_ty_1__bindgen_ty_5__bindgen_ty_1__bindgen_ty_2,
    pub __bindgen_anon_3:
        MIDIUniversalMessage__bindgen_ty_1__bindgen_ty_5__bindgen_ty_1__bindgen_ty_3,
    pub __bindgen_anon_4:
        MIDIUniversalMessage__bindgen_ty_1__bindgen_ty_5__bindgen_ty_1__bindgen_ty_4,
    pub __bindgen_anon_5:
        MIDIUniversalMessage__bindgen_ty_1__bindgen_ty_5__bindgen_ty_1__bindgen_ty_5,
    pub __bindgen_anon_6:
        MIDIUniversalMessage__bindgen_ty_1__bindgen_ty_5__bindgen_ty_1__bindgen_ty_6,
    pub __bindgen_anon_7:
        MIDIUniversalMessage__bindgen_ty_1__bindgen_ty_5__bindgen_ty_1__bindgen_ty_7,
    pub __bindgen_anon_8:
        MIDIUniversalMessage__bindgen_ty_1__bindgen_ty_5__bindgen_ty_1__bindgen_ty_8,
    pub __bindgen_anon_9:
        MIDIUniversalMessage__bindgen_ty_1__bindgen_ty_5__bindgen_ty_1__bindgen_ty_9,
    pub __bindgen_anon_10:
        MIDIUniversalMessage__bindgen_ty_1__bindgen_ty_5__bindgen_ty_1__bindgen_ty_10,
    pub note: MIDIUniversalMessage__bindgen_ty_1__bindgen_ty_5__bindgen_ty_1__bindgen_ty_1,
    pub polyPressure: MIDIUniversalMessage__bindgen_ty_1__bindgen_ty_5__bindgen_ty_1__bindgen_ty_2,
    pub controlChange: MIDIUniversalMessage__bindgen_ty_1__bindgen_ty_5__bindgen_ty_1__bindgen_ty_3,
    pub programChange: MIDIUniversalMessage__bindgen_ty_1__bindgen_ty_5__bindgen_ty_1__bindgen_ty_4,
    pub channelPressure:
        MIDIUniversalMessage__bindgen_ty_1__bindgen_ty_5__bindgen_ty_1__bindgen_ty_5,
    pub pitchBend: MIDIUniversalMessage__bindgen_ty_1__bindgen_ty_5__bindgen_ty_1__bindgen_ty_6,
    pub perNoteController:
        MIDIUniversalMessage__bindgen_ty_1__bindgen_ty_5__bindgen_ty_1__bindgen_ty_7,
    pub controller: MIDIUniversalMessage__bindgen_ty_1__bindgen_ty_5__bindgen_ty_1__bindgen_ty_8,
    pub perNotePitchBend:
        MIDIUniversalMessage__bindgen_ty_1__bindgen_ty_5__bindgen_ty_1__bindgen_ty_9,
    pub perNoteManagement:
        MIDIUniversalMessage__bindgen_ty_1__bindgen_ty_5__bindgen_ty_1__bindgen_ty_10,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct MIDIUniversalMessage__bindgen_ty_1__bindgen_ty_5__bindgen_ty_1__bindgen_ty_1 {
    pub number: UInt8,
    pub attributeType: MIDINoteAttribute,
    pub velocity: UInt16,
    pub attribute: UInt16,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct MIDIUniversalMessage__bindgen_ty_1__bindgen_ty_5__bindgen_ty_1__bindgen_ty_2 {
    pub noteNumber: UInt8,
    pub reserved: UInt8,
    pub pressure: UInt32,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct MIDIUniversalMessage__bindgen_ty_1__bindgen_ty_5__bindgen_ty_1__bindgen_ty_3 {
    pub index: UInt8,
    pub reserved: UInt8,
    pub data: UInt32,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct MIDIUniversalMessage__bindgen_ty_1__bindgen_ty_5__bindgen_ty_1__bindgen_ty_4 {
    pub options: MIDIProgramChangeOptions,
    pub program: UInt8,
    pub reserved: [UInt8; 2usize],
    pub bank: UInt16,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct MIDIUniversalMessage__bindgen_ty_1__bindgen_ty_5__bindgen_ty_1__bindgen_ty_5 {
    pub data: UInt32,
    pub reserved: [UInt8; 2usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct MIDIUniversalMessage__bindgen_ty_1__bindgen_ty_5__bindgen_ty_1__bindgen_ty_6 {
    pub data: UInt32,
    pub reserved: [UInt8; 2usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct MIDIUniversalMessage__bindgen_ty_1__bindgen_ty_5__bindgen_ty_1__bindgen_ty_7 {
    pub noteNumber: UInt8,
    pub index: UInt8,
    pub data: UInt32,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct MIDIUniversalMessage__bindgen_ty_1__bindgen_ty_5__bindgen_ty_1__bindgen_ty_8 {
    pub bank: UInt8,
    pub index: UInt8,
    pub data: UInt32,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct MIDIUniversalMessage__bindgen_ty_1__bindgen_ty_5__bindgen_ty_1__bindgen_ty_9 {
    pub noteNumber: UInt8,
    pub reserved: UInt8,
    pub bend: UInt32,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct MIDIUniversalMessage__bindgen_ty_1__bindgen_ty_5__bindgen_ty_1__bindgen_ty_10 {
    pub note: UInt8,
    pub options: MIDIPerNoteManagementOptions,
    pub reserved: [UInt8; 4usize],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct MIDIUniversalMessage__bindgen_ty_1__bindgen_ty_6 {
    pub __bindgen_anon_1: MIDIUniversalMessage__bindgen_ty_1__bindgen_ty_6__bindgen_ty_1,
    pub status: MIDISysExStatus,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union MIDIUniversalMessage__bindgen_ty_1__bindgen_ty_6__bindgen_ty_1 {
    pub __bindgen_anon_1:
        MIDIUniversalMessage__bindgen_ty_1__bindgen_ty_6__bindgen_ty_1__bindgen_ty_1,
    pub __bindgen_anon_2:
        MIDIUniversalMessage__bindgen_ty_1__bindgen_ty_6__bindgen_ty_1__bindgen_ty_2,
    pub sysex8: MIDIUniversalMessage__bindgen_ty_1__bindgen_ty_6__bindgen_ty_1__bindgen_ty_1,
    pub mixedDataSet: MIDIUniversalMessage__bindgen_ty_1__bindgen_ty_6__bindgen_ty_1__bindgen_ty_2,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct MIDIUniversalMessage__bindgen_ty_1__bindgen_ty_6__bindgen_ty_1__bindgen_ty_1 {
    pub byteCount: UInt8,
    pub streamID: UInt8,
    pub data: [UInt8; 13usize],
    pub reserved: UInt8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct MIDIUniversalMessage__bindgen_ty_1__bindgen_ty_6__bindgen_ty_1__bindgen_ty_2 {
    pub mdsID: UInt8,
    pub data: [UInt8; 14usize],
    pub reserved: UInt8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct MIDIUniversalMessage__bindgen_ty_1__bindgen_ty_7 {
    pub words: [UInt32; 4usize],
}
pub type MIDIEventVisitor = ::std::option::Option<
    unsafe extern "C" fn(
        context: *mut ::std::os::raw::c_void,
        timeStamp: MIDITimeStamp,
        message: MIDIUniversalMessage,
    ),
>;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct MIDI2DeviceManufacturer {
    pub sysExIDByte: [Byte; 3usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct MIDI2DeviceRevisionLevel {
    pub revisionLevel: [Byte; 4usize],
}
pub type MIDICICategoryOptions = MIDIUInteger7;
pub type MIDIUMPFunctionBlockID = MIDIUInteger7;
pub type MIDICIDeviceType = UInt8;
pub type MIDICIProfileMessageType = MIDIUInteger7;
pub type MIDICIPropertyExchangeMessageType = MIDIUInteger7;
pub type MIDICIProcessInquiryMessageType = MIDIUInteger7;
pub type MIDICIManagementMessageType = MIDIUInteger7;
pub type MIDICIProfileType = UInt8;
pub type MIDIUMPCIObjectBackingType = UInt8;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct MIDICIProfileIDStandard {
    pub profileIDByte1: MIDIUInteger7,
    pub profileBank: MIDIUInteger7,
    pub profileNumber: MIDIUInteger7,
    pub profileVersion: MIDIUInteger7,
    pub profileLevel: MIDIUInteger7,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct MIDICIProfileIDManufacturerSpecific {
    pub sysExID1: MIDIUInteger7,
    pub sysExID2: MIDIUInteger7,
    pub sysExID3: MIDIUInteger7,
    pub info1: MIDIUInteger7,
    pub info2: MIDIUInteger7,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union MIDICIProfileID {
    pub standard: MIDICIProfileIDStandard,
    pub manufacturerSpecific: MIDICIProfileIDManufacturerSpecific,
}
pub type MIDINetworkConnectionPolicy = NSUInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MIDINetworkHost(pub id);
impl std::ops::Deref for MIDINetworkHost {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MIDINetworkHost {}
impl MIDINetworkHost {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MIDINetworkHost").unwrap(), alloc) })
    }
}
impl INSObject for MIDINetworkHost {}
impl PNSObject for MIDINetworkHost {}
impl std::convert::TryFrom<NSObject> for MIDINetworkHost {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<MIDINetworkHost, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MIDINetworkHost").unwrap()) };
        if is_kind_of {
            Ok(MIDINetworkHost(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to MIDINetworkHost")
        }
    }
}
impl IMIDINetworkHost for MIDINetworkHost {}
pub trait IMIDINetworkHost: Sized + std::ops::Deref {
    unsafe fn hasSameAddressAs_(&self, other: MIDINetworkHost) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, hasSameAddressAs : other)
    }
    unsafe fn name(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, name)
    }
    unsafe fn address(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, address)
    }
    unsafe fn port(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, port)
    }
    unsafe fn netServiceName(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, netServiceName)
    }
    unsafe fn netServiceDomain(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, netServiceDomain)
    }
    unsafe fn hostWithName_address_port_(
        name: NSString,
        address: NSString,
        port: NSUInteger,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MIDINetworkHost").unwrap(), hostWithName : name, address : address, port : port)
    }
    unsafe fn hostWithName_netService_(name: NSString, netService: NSNetService) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MIDINetworkHost").unwrap(), hostWithName : name, netService : netService)
    }
    unsafe fn hostWithName_netServiceName_netServiceDomain_(
        name: NSString,
        netServiceName: NSString,
        netServiceDomain: NSString,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MIDINetworkHost").unwrap(), hostWithName : name, netServiceName : netServiceName, netServiceDomain : netServiceDomain)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MIDINetworkConnection(pub id);
impl std::ops::Deref for MIDINetworkConnection {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MIDINetworkConnection {}
impl MIDINetworkConnection {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MIDINetworkConnection").unwrap(), alloc) })
    }
}
impl INSObject for MIDINetworkConnection {}
impl PNSObject for MIDINetworkConnection {}
impl std::convert::TryFrom<NSObject> for MIDINetworkConnection {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<MIDINetworkConnection, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MIDINetworkConnection").unwrap()) };
        if is_kind_of {
            Ok(MIDINetworkConnection(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to MIDINetworkConnection")
        }
    }
}
impl IMIDINetworkConnection for MIDINetworkConnection {}
pub trait IMIDINetworkConnection: Sized + std::ops::Deref {
    unsafe fn host(&self) -> MIDINetworkHost
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, host)
    }
    unsafe fn connectionWithHost_(host: MIDINetworkHost) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MIDINetworkConnection").unwrap(), connectionWithHost : host)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MIDINetworkSession(pub id);
impl std::ops::Deref for MIDINetworkSession {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MIDINetworkSession {}
impl MIDINetworkSession {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MIDINetworkSession").unwrap(), alloc) })
    }
}
impl INSObject for MIDINetworkSession {}
impl PNSObject for MIDINetworkSession {}
impl std::convert::TryFrom<NSObject> for MIDINetworkSession {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<MIDINetworkSession, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MIDINetworkSession").unwrap()) };
        if is_kind_of {
            Ok(MIDINetworkSession(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to MIDINetworkSession")
        }
    }
}
impl IMIDINetworkSession for MIDINetworkSession {}
pub trait IMIDINetworkSession: Sized + std::ops::Deref {
    unsafe fn contacts(&self) -> NSSet
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, contacts)
    }
    unsafe fn addContact_(&self, contact: MIDINetworkHost) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, addContact : contact)
    }
    unsafe fn removeContact_(&self, contact: MIDINetworkHost) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, removeContact : contact)
    }
    unsafe fn connections(&self) -> NSSet
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, connections)
    }
    unsafe fn addConnection_(&self, connection: MIDINetworkConnection) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, addConnection : connection)
    }
    unsafe fn removeConnection_(&self, connection: MIDINetworkConnection) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, removeConnection : connection)
    }
    unsafe fn sourceEndpoint(&self) -> MIDIEndpointRef
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, sourceEndpoint)
    }
    unsafe fn destinationEndpoint(&self) -> MIDIEndpointRef
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, destinationEndpoint)
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
    unsafe fn networkPort(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, networkPort)
    }
    unsafe fn networkName(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, networkName)
    }
    unsafe fn localName(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, localName)
    }
    unsafe fn connectionPolicy(&self) -> MIDINetworkConnectionPolicy
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, connectionPolicy)
    }
    unsafe fn setConnectionPolicy_(&self, connectionPolicy: MIDINetworkConnectionPolicy)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setConnectionPolicy : connectionPolicy)
    }
    unsafe fn defaultSession() -> MIDINetworkSession
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MIDINetworkSession").unwrap(), defaultSession)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MIDI2DeviceInfo(pub id);
impl std::ops::Deref for MIDI2DeviceInfo {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MIDI2DeviceInfo {}
impl MIDI2DeviceInfo {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MIDI2DeviceInfo").unwrap(), alloc) })
    }
}
impl INSObject for MIDI2DeviceInfo {}
impl PNSObject for MIDI2DeviceInfo {}
impl std::convert::TryFrom<NSObject> for MIDI2DeviceInfo {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<MIDI2DeviceInfo, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MIDI2DeviceInfo").unwrap()) };
        if is_kind_of {
            Ok(MIDI2DeviceInfo(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to MIDI2DeviceInfo")
        }
    }
}
impl IMIDI2DeviceInfo for MIDI2DeviceInfo {}
pub trait IMIDI2DeviceInfo: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn initWithManufacturerID_family_modelNumber_revisionLevel_(
        &self,
        manufacturerID: MIDI2DeviceManufacturer,
        family: MIDIUInteger14,
        modelNumber: MIDIUInteger14,
        revisionLevel: MIDI2DeviceRevisionLevel,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithManufacturerID : manufacturerID, family : family, modelNumber : modelNumber, revisionLevel : revisionLevel)
    }
    unsafe fn manufacturerID(&self) -> MIDI2DeviceManufacturer
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, manufacturerID)
    }
    unsafe fn family(&self) -> MIDIUInteger14
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, family)
    }
    unsafe fn modelNumber(&self) -> MIDIUInteger14
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, modelNumber)
    }
    unsafe fn revisionLevel(&self) -> MIDI2DeviceRevisionLevel
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, revisionLevel)
    }
}
pub type MIDIUMPProtocolOptions = MIDIUInteger4;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MIDIUMPEndpoint(pub id);
impl std::ops::Deref for MIDIUMPEndpoint {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MIDIUMPEndpoint {}
impl MIDIUMPEndpoint {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MIDIUMPEndpoint").unwrap(), alloc) })
    }
}
impl INSObject for MIDIUMPEndpoint {}
impl PNSObject for MIDIUMPEndpoint {}
impl std::convert::TryFrom<NSObject> for MIDIUMPEndpoint {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<MIDIUMPEndpoint, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MIDIUMPEndpoint").unwrap()) };
        if is_kind_of {
            Ok(MIDIUMPEndpoint(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to MIDIUMPEndpoint")
        }
    }
}
impl IMIDIUMPEndpoint for MIDIUMPEndpoint {}
pub trait IMIDIUMPEndpoint: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn name(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, name)
    }
    unsafe fn MIDIProtocol(&self) -> MIDIProtocolID
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, MIDIProtocol)
    }
    unsafe fn supportedMIDIProtocols(&self) -> MIDIUMPProtocolOptions
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, supportedMIDIProtocols)
    }
    unsafe fn MIDIDestination(&self) -> MIDIEndpointRef
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, MIDIDestination)
    }
    unsafe fn MIDISource(&self) -> MIDIEndpointRef
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, MIDISource)
    }
    unsafe fn deviceInfo(&self) -> MIDI2DeviceInfo
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, deviceInfo)
    }
    unsafe fn productInstanceID(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, productInstanceID)
    }
    unsafe fn hasStaticFunctionBlocks(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, hasStaticFunctionBlocks)
    }
    unsafe fn hasJRTSReceiveCapability(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, hasJRTSReceiveCapability)
    }
    unsafe fn hasJRTSTransmitCapability(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, hasJRTSTransmitCapability)
    }
    unsafe fn endpointType(&self) -> MIDIUMPCIObjectBackingType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, endpointType)
    }
    unsafe fn functionBlocks(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, functionBlocks)
    }
    unsafe fn setFunctionBlocks_(&self, functionBlocks: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setFunctionBlocks : functionBlocks)
    }
}
pub type MIDIUMPEndpointManagerDictionaryKey = NSString;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MIDIUMPEndpointManager(pub id);
impl std::ops::Deref for MIDIUMPEndpointManager {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MIDIUMPEndpointManager {}
impl MIDIUMPEndpointManager {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MIDIUMPEndpointManager").unwrap(), alloc) })
    }
}
impl INSObject for MIDIUMPEndpointManager {}
impl PNSObject for MIDIUMPEndpointManager {}
impl std::convert::TryFrom<NSObject> for MIDIUMPEndpointManager {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<MIDIUMPEndpointManager, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MIDIUMPEndpointManager").unwrap()) };
        if is_kind_of {
            Ok(MIDIUMPEndpointManager(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to MIDIUMPEndpointManager")
        }
    }
}
impl IMIDIUMPEndpointManager for MIDIUMPEndpointManager {}
pub trait IMIDIUMPEndpointManager: Sized + std::ops::Deref {
    unsafe fn UMPEndpoints(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, UMPEndpoints)
    }
    unsafe fn sharedInstance() -> MIDIUMPEndpointManager
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MIDIUMPEndpointManager").unwrap(), sharedInstance)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MIDIUMPFunctionBlock(pub id);
impl std::ops::Deref for MIDIUMPFunctionBlock {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MIDIUMPFunctionBlock {}
impl MIDIUMPFunctionBlock {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MIDIUMPFunctionBlock").unwrap(), alloc) })
    }
}
impl INSObject for MIDIUMPFunctionBlock {}
impl PNSObject for MIDIUMPFunctionBlock {}
impl std::convert::TryFrom<NSObject> for MIDIUMPFunctionBlock {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<MIDIUMPFunctionBlock, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MIDIUMPFunctionBlock").unwrap()) };
        if is_kind_of {
            Ok(MIDIUMPFunctionBlock(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to MIDIUMPFunctionBlock")
        }
    }
}
impl IMIDIUMPFunctionBlock for MIDIUMPFunctionBlock {}
pub trait IMIDIUMPFunctionBlock: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn name(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, name)
    }
    unsafe fn functionBlockID(&self) -> MIDIUMPFunctionBlockID
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, functionBlockID)
    }
    unsafe fn direction(&self) -> MIDIUMPFunctionBlockDirection
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, direction)
    }
    unsafe fn firstGroup(&self) -> MIDIUMPGroupNumber
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, firstGroup)
    }
    unsafe fn totalGroupsSpanned(&self) -> MIDIUInteger7
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, totalGroupsSpanned)
    }
    unsafe fn maxSysEx8Streams(&self) -> UInt8
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, maxSysEx8Streams)
    }
    unsafe fn MIDI1Info(&self) -> MIDIUMPFunctionBlockMIDI1Info
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, MIDI1Info)
    }
    unsafe fn UIHint(&self) -> MIDIUMPFunctionBlockUIHint
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, UIHint)
    }
    unsafe fn UMPEndpoint(&self) -> MIDIUMPEndpoint
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, UMPEndpoint)
    }
    unsafe fn midiCIDevice(&self) -> MIDICIDevice
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, midiCIDevice)
    }
    unsafe fn isEnabled(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isEnabled)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MIDIUMPMutableEndpoint(pub id);
impl std::ops::Deref for MIDIUMPMutableEndpoint {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MIDIUMPMutableEndpoint {}
impl MIDIUMPMutableEndpoint {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MIDIUMPMutableEndpoint").unwrap(), alloc) })
    }
}
impl IMIDIUMPEndpoint for MIDIUMPMutableEndpoint {}
impl From<MIDIUMPMutableEndpoint> for MIDIUMPEndpoint {
    fn from(child: MIDIUMPMutableEndpoint) -> MIDIUMPEndpoint {
        MIDIUMPEndpoint(child.0)
    }
}
impl std::convert::TryFrom<MIDIUMPEndpoint> for MIDIUMPMutableEndpoint {
    type Error = &'static str;
    fn try_from(parent: MIDIUMPEndpoint) -> Result<MIDIUMPMutableEndpoint, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MIDIUMPMutableEndpoint").unwrap()) };
        if is_kind_of {
            Ok(MIDIUMPMutableEndpoint(parent.0))
        } else {
            Err("This MIDIUMPEndpoint cannot be downcasted to MIDIUMPMutableEndpoint")
        }
    }
}
impl INSObject for MIDIUMPMutableEndpoint {}
impl PNSObject for MIDIUMPMutableEndpoint {}
impl IMIDIUMPMutableEndpoint for MIDIUMPMutableEndpoint {}
pub trait IMIDIUMPMutableEndpoint: Sized + std::ops::Deref {
    unsafe fn initWithName_deviceInfo_productInstanceID_MIDIProtocol_destinationCallback_(
        &self,
        name: NSString,
        deviceInfo: MIDI2DeviceInfo,
        productInstanceID: NSString,
        MIDIProtocol: MIDIProtocolID,
        destinationCallback: MIDIReceiveBlock,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithName : name, deviceInfo : deviceInfo, productInstanceID : productInstanceID, MIDIProtocol : MIDIProtocol, destinationCallback : destinationCallback)
    }
    unsafe fn setName_error_(&self, name: NSString, error: *mut NSError) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setName : name, error : error)
    }
    unsafe fn registerFunctionBlocks_markAsStatic_error_(
        &self,
        functionBlocks: NSArray,
        markAsStatic: BOOL,
        error: *mut NSError,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, registerFunctionBlocks : functionBlocks, markAsStatic : markAsStatic, error : error)
    }
    unsafe fn setEnabled_error_(&self, isEnabled: BOOL, error: *mut NSError) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setEnabled : isEnabled, error : error)
    }
    unsafe fn mutableFunctionBlocks(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, mutableFunctionBlocks)
    }
    unsafe fn setMutableFunctionBlocks_(&self, mutableFunctionBlocks: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMutableFunctionBlocks : mutableFunctionBlocks)
    }
    unsafe fn isEnabled(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isEnabled)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MIDIUMPMutableFunctionBlock(pub id);
impl std::ops::Deref for MIDIUMPMutableFunctionBlock {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MIDIUMPMutableFunctionBlock {}
impl MIDIUMPMutableFunctionBlock {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MIDIUMPMutableFunctionBlock").unwrap(), alloc) })
    }
}
impl IMIDIUMPFunctionBlock for MIDIUMPMutableFunctionBlock {}
impl From<MIDIUMPMutableFunctionBlock> for MIDIUMPFunctionBlock {
    fn from(child: MIDIUMPMutableFunctionBlock) -> MIDIUMPFunctionBlock {
        MIDIUMPFunctionBlock(child.0)
    }
}
impl std::convert::TryFrom<MIDIUMPFunctionBlock> for MIDIUMPMutableFunctionBlock {
    type Error = &'static str;
    fn try_from(parent: MIDIUMPFunctionBlock) -> Result<MIDIUMPMutableFunctionBlock, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MIDIUMPMutableFunctionBlock").unwrap()) };
        if is_kind_of {
            Ok(MIDIUMPMutableFunctionBlock(parent.0))
        } else {
            Err("This MIDIUMPFunctionBlock cannot be downcasted to MIDIUMPMutableFunctionBlock")
        }
    }
}
impl INSObject for MIDIUMPMutableFunctionBlock {}
impl PNSObject for MIDIUMPMutableFunctionBlock {}
impl IMIDIUMPMutableFunctionBlock for MIDIUMPMutableFunctionBlock {}
pub trait IMIDIUMPMutableFunctionBlock: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn initWithName_direction_firstGroup_totalGroupsSpanned_maxSysEx8Streams_MIDI1Info_UIHint_isEnabled_(
        &self,
        name: NSString,
        direction: MIDIUMPFunctionBlockDirection,
        firstGroup: MIDIUMPGroupNumber,
        totalGroupsSpanned: MIDIUInteger7,
        maxSysEx8Streams: MIDIUInteger7,
        MIDI1Info: MIDIUMPFunctionBlockMIDI1Info,
        UIHint: MIDIUMPFunctionBlockUIHint,
        isEnabled: BOOL,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithName : name, direction : direction, firstGroup : firstGroup, totalGroupsSpanned : totalGroupsSpanned, maxSysEx8Streams : maxSysEx8Streams, MIDI1Info : MIDI1Info, UIHint : UIHint, isEnabled : isEnabled)
    }
    unsafe fn setEnabled_error_(&self, isEnabled: BOOL, error: *mut NSError) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setEnabled : isEnabled, error : error)
    }
    unsafe fn setName_error_(&self, name: NSString, error: *mut NSError) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setName : name, error : error)
    }
    unsafe fn reconfigureWithFirstGroup_direction_MIDI1Info_UIHint_error_(
        &self,
        firstGroup: MIDIUMPGroupNumber,
        direction: MIDIUMPFunctionBlockDirection,
        MIDI1Info: MIDIUMPFunctionBlockMIDI1Info,
        UIHint: MIDIUMPFunctionBlockUIHint,
        error: *mut NSError,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, reconfigureWithFirstGroup : firstGroup, direction : direction, MIDI1Info : MIDI1Info, UIHint : UIHint, error : error)
    }
    unsafe fn UMPEndpoint(&self) -> MIDIUMPMutableEndpoint
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, UMPEndpoint)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MIDICIDevice(pub id);
impl std::ops::Deref for MIDICIDevice {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MIDICIDevice {}
impl MIDICIDevice {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MIDICIDevice").unwrap(), alloc) })
    }
}
impl INSObject for MIDICIDevice {}
impl PNSObject for MIDICIDevice {}
impl std::convert::TryFrom<NSObject> for MIDICIDevice {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<MIDICIDevice, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MIDICIDevice").unwrap()) };
        if is_kind_of {
            Ok(MIDICIDevice(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to MIDICIDevice")
        }
    }
}
impl IMIDICIDevice for MIDICIDevice {}
pub trait IMIDICIDevice: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn deviceInfo(&self) -> MIDI2DeviceInfo
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, deviceInfo)
    }
    unsafe fn MUID(&self) -> MIDICIMUID
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, MUID)
    }
    unsafe fn supportsProtocolNegotiation(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, supportsProtocolNegotiation)
    }
    unsafe fn supportsProfileConfiguration(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, supportsProfileConfiguration)
    }
    unsafe fn supportsPropertyExchange(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, supportsPropertyExchange)
    }
    unsafe fn supportsProcessInquiry(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, supportsProcessInquiry)
    }
    unsafe fn maxSysExSize(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, maxSysExSize)
    }
    unsafe fn maxPropertyExchangeRequests(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, maxPropertyExchangeRequests)
    }
    unsafe fn deviceType(&self) -> MIDICIDeviceType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, deviceType)
    }
    unsafe fn profiles(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, profiles)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MIDIUMPCIProfile(pub id);
impl std::ops::Deref for MIDIUMPCIProfile {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MIDIUMPCIProfile {}
impl MIDIUMPCIProfile {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MIDIUMPCIProfile").unwrap(), alloc) })
    }
}
impl INSObject for MIDIUMPCIProfile {}
impl PNSObject for MIDIUMPCIProfile {}
impl std::convert::TryFrom<NSObject> for MIDIUMPCIProfile {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<MIDIUMPCIProfile, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MIDIUMPCIProfile").unwrap()) };
        if is_kind_of {
            Ok(MIDIUMPCIProfile(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to MIDIUMPCIProfile")
        }
    }
}
impl IMIDIUMPCIProfile for MIDIUMPCIProfile {}
pub trait IMIDIUMPCIProfile: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn setProfileState_enabledChannelCount_error_(
        &self,
        isEnabled: BOOL,
        enabledChannelCount: MIDIUInteger14,
        error: *mut NSError,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setProfileState : isEnabled, enabledChannelCount : enabledChannelCount, error : error)
    }
    unsafe fn name(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, name)
    }
    unsafe fn profileID(&self) -> MIDICIProfileID
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, profileID)
    }
    unsafe fn profileType(&self) -> MIDICIProfileType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, profileType)
    }
    unsafe fn groupOffset(&self) -> MIDIUMPGroupNumber
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, groupOffset)
    }
    unsafe fn firstChannel(&self) -> MIDIChannelNumber
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, firstChannel)
    }
    unsafe fn enabledChannelCount(&self) -> MIDIUInteger14
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, enabledChannelCount)
    }
    unsafe fn totalChannelCount(&self) -> MIDIUInteger14
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, totalChannelCount)
    }
    unsafe fn isEnabled(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isEnabled)
    }
}
pub type MIDICIDeviceManagerDictionaryKey = NSString;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MIDICIDeviceManager(pub id);
impl std::ops::Deref for MIDICIDeviceManager {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MIDICIDeviceManager {}
impl MIDICIDeviceManager {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MIDICIDeviceManager").unwrap(), alloc) })
    }
}
impl INSObject for MIDICIDeviceManager {}
impl PNSObject for MIDICIDeviceManager {}
impl std::convert::TryFrom<NSObject> for MIDICIDeviceManager {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<MIDICIDeviceManager, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MIDICIDeviceManager").unwrap()) };
        if is_kind_of {
            Ok(MIDICIDeviceManager(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to MIDICIDeviceManager")
        }
    }
}
impl IMIDICIDeviceManager for MIDICIDeviceManager {}
pub trait IMIDICIDeviceManager: Sized + std::ops::Deref {
    unsafe fn discoveredCIDevices(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, discoveredCIDevices)
    }
    unsafe fn sharedInstance() -> MIDICIDeviceManager
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MIDICIDeviceManager").unwrap(), sharedInstance)
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct MIDICIDeviceIdentification {
    pub manufacturer: [u8; 3usize],
    pub family: [u8; 2usize],
    pub modelNumber: [u8; 2usize],
    pub revisionLevel: [u8; 4usize],
    pub reserved: [u8; 5usize],
}
pub type MIDICIInitiatiorMUID = NSNumber;
pub type MIDICIProfileStateList = NSArray;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MIDICIDeviceInfo(pub id);
impl std::ops::Deref for MIDICIDeviceInfo {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MIDICIDeviceInfo {}
impl MIDICIDeviceInfo {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MIDICIDeviceInfo").unwrap(), alloc) })
    }
}
impl PNSSecureCoding for MIDICIDeviceInfo {}
impl INSObject for MIDICIDeviceInfo {}
impl PNSObject for MIDICIDeviceInfo {}
impl std::convert::TryFrom<NSObject> for MIDICIDeviceInfo {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<MIDICIDeviceInfo, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MIDICIDeviceInfo").unwrap()) };
        if is_kind_of {
            Ok(MIDICIDeviceInfo(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to MIDICIDeviceInfo")
        }
    }
}
impl IMIDICIDeviceInfo for MIDICIDeviceInfo {}
pub trait IMIDICIDeviceInfo: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn initWithDestination_manufacturer_family_model_revision_(
        &self,
        midiDestination: MIDIEntityRef,
        manufacturer: NSData,
        family: NSData,
        modelNumber: NSData,
        revisionLevel: NSData,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithDestination : midiDestination, manufacturer : manufacturer, family : family, model : modelNumber, revision : revisionLevel)
    }
    unsafe fn manufacturerID(&self) -> NSData
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, manufacturerID)
    }
    unsafe fn family(&self) -> NSData
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, family)
    }
    unsafe fn modelNumber(&self) -> NSData
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, modelNumber)
    }
    unsafe fn revisionLevel(&self) -> NSData
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, revisionLevel)
    }
    unsafe fn midiDestination(&self) -> MIDIEndpointRef
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, midiDestination)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MIDICIDiscoveredNode(pub id);
impl std::ops::Deref for MIDICIDiscoveredNode {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MIDICIDiscoveredNode {}
impl MIDICIDiscoveredNode {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MIDICIDiscoveredNode").unwrap(), alloc) })
    }
}
impl PNSSecureCoding for MIDICIDiscoveredNode {}
impl INSObject for MIDICIDiscoveredNode {}
impl PNSObject for MIDICIDiscoveredNode {}
impl std::convert::TryFrom<NSObject> for MIDICIDiscoveredNode {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<MIDICIDiscoveredNode, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MIDICIDiscoveredNode").unwrap()) };
        if is_kind_of {
            Ok(MIDICIDiscoveredNode(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to MIDICIDiscoveredNode")
        }
    }
}
impl IMIDICIDiscoveredNode for MIDICIDiscoveredNode {}
pub trait IMIDICIDiscoveredNode: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn destination(&self) -> MIDIEntityRef
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, destination)
    }
    unsafe fn deviceInfo(&self) -> MIDICIDeviceInfo
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, deviceInfo)
    }
    unsafe fn supportsProfiles(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, supportsProfiles)
    }
    unsafe fn supportsProperties(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, supportsProperties)
    }
    unsafe fn maximumSysExSize(&self) -> NSNumber
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, maximumSysExSize)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MIDICIProfile(pub id);
impl std::ops::Deref for MIDICIProfile {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MIDICIProfile {}
impl MIDICIProfile {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MIDICIProfile").unwrap(), alloc) })
    }
}
impl PNSSecureCoding for MIDICIProfile {}
impl INSObject for MIDICIProfile {}
impl PNSObject for MIDICIProfile {}
impl std::convert::TryFrom<NSObject> for MIDICIProfile {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<MIDICIProfile, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MIDICIProfile").unwrap()) };
        if is_kind_of {
            Ok(MIDICIProfile(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to MIDICIProfile")
        }
    }
}
impl IMIDICIProfile for MIDICIProfile {}
pub trait IMIDICIProfile: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn initWithData_(&self, data: NSData) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithData : data)
    }
    unsafe fn initWithData_name_(&self, data: NSData, inName: NSString) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithData : data, name : inName)
    }
    unsafe fn name(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, name)
    }
    unsafe fn profileID(&self) -> NSData
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, profileID)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MIDICIProfileState(pub id);
impl std::ops::Deref for MIDICIProfileState {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MIDICIProfileState {}
impl MIDICIProfileState {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MIDICIProfileState").unwrap(), alloc) })
    }
}
impl PNSSecureCoding for MIDICIProfileState {}
impl INSObject for MIDICIProfileState {}
impl PNSObject for MIDICIProfileState {}
impl std::convert::TryFrom<NSObject> for MIDICIProfileState {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<MIDICIProfileState, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MIDICIProfileState").unwrap()) };
        if is_kind_of {
            Ok(MIDICIProfileState(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to MIDICIProfileState")
        }
    }
}
impl IMIDICIProfileState for MIDICIProfileState {}
pub trait IMIDICIProfileState: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn initWithChannel_enabledProfiles_disabledProfiles_(
        &self,
        midiChannelNum: MIDIChannelNumber,
        enabled: NSArray,
        disabled: NSArray,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithChannel : midiChannelNum, enabledProfiles : enabled, disabledProfiles : disabled)
    }
    unsafe fn initWithEnabledProfiles_disabledProfiles_(
        &self,
        enabled: NSArray,
        disabled: NSArray,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithEnabledProfiles : enabled, disabledProfiles : disabled)
    }
    unsafe fn midiChannel(&self) -> MIDIChannelNumber
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, midiChannel)
    }
    unsafe fn enabledProfiles(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, enabledProfiles)
    }
    unsafe fn disabledProfiles(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, disabledProfiles)
    }
}
pub type MIDICIProfileChangedBlock = *mut ::std::os::raw::c_void;
pub type MIDICISessionDisconnectBlock = *mut ::std::os::raw::c_void;
pub type MIDICIProfileSpecificDataBlock = *mut ::std::os::raw::c_void;
pub type MIDICIDiscoveryResponseBlock = *mut ::std::os::raw::c_void;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MIDICISession(pub id);
impl std::ops::Deref for MIDICISession {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MIDICISession {}
impl MIDICISession {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MIDICISession").unwrap(), alloc) })
    }
}
impl INSObject for MIDICISession {}
impl PNSObject for MIDICISession {}
impl std::convert::TryFrom<NSObject> for MIDICISession {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<MIDICISession, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MIDICISession").unwrap()) };
        if is_kind_of {
            Ok(MIDICISession(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to MIDICISession")
        }
    }
}
impl IMIDICISession for MIDICISession {}
pub trait IMIDICISession: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn initWithDiscoveredNode_dataReadyHandler_disconnectHandler_(
        &self,
        discoveredNode: MIDICIDiscoveredNode,
        handler: *mut ::std::os::raw::c_void,
        disconnectHandler: MIDICISessionDisconnectBlock,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithDiscoveredNode : discoveredNode, dataReadyHandler : handler, disconnectHandler : disconnectHandler)
    }
    unsafe fn profileStateForChannel_(&self, channel: MIDIChannelNumber) -> MIDICIProfileState
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, profileStateForChannel : channel)
    }
    unsafe fn enableProfile_onChannel_error_(
        &self,
        profile: MIDICIProfile,
        channel: MIDIChannelNumber,
        outError: *mut NSError,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, enableProfile : profile, onChannel : channel, error : outError)
    }
    unsafe fn disableProfile_onChannel_error_(
        &self,
        profile: MIDICIProfile,
        channel: MIDIChannelNumber,
        outError: *mut NSError,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, disableProfile : profile, onChannel : channel, error : outError)
    }
    unsafe fn sendProfile_onChannel_profileData_(
        &self,
        profile: MIDICIProfile,
        channel: MIDIChannelNumber,
        profileSpecificData: NSData,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, sendProfile : profile, onChannel : channel, profileData : profileSpecificData)
    }
    unsafe fn midiDestination(&self) -> MIDIEntityRef
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, midiDestination)
    }
    unsafe fn supportsProfileCapability(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, supportsProfileCapability)
    }
    unsafe fn supportsPropertyCapability(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, supportsPropertyCapability)
    }
    unsafe fn deviceInfo(&self) -> MIDICIDeviceInfo
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, deviceInfo)
    }
    unsafe fn maxSysExSize(&self) -> NSNumber
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, maxSysExSize)
    }
    unsafe fn maxPropertyRequests(&self) -> NSNumber
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, maxPropertyRequests)
    }
    unsafe fn profileChangedCallback(&self) -> MIDICIProfileChangedBlock
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, profileChangedCallback)
    }
    unsafe fn setProfileChangedCallback_(&self, profileChangedCallback: MIDICIProfileChangedBlock)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setProfileChangedCallback : profileChangedCallback)
    }
    unsafe fn profileSpecificDataHandler(&self) -> MIDICIProfileSpecificDataBlock
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, profileSpecificDataHandler)
    }
    unsafe fn setProfileSpecificDataHandler_(
        &self,
        profileSpecificDataHandler: MIDICIProfileSpecificDataBlock,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setProfileSpecificDataHandler : profileSpecificDataHandler)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MIDICIDiscoveryManager(pub id);
impl std::ops::Deref for MIDICIDiscoveryManager {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MIDICIDiscoveryManager {}
impl MIDICIDiscoveryManager {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MIDICIDiscoveryManager").unwrap(), alloc) })
    }
}
impl INSObject for MIDICIDiscoveryManager {}
impl PNSObject for MIDICIDiscoveryManager {}
impl std::convert::TryFrom<NSObject> for MIDICIDiscoveryManager {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<MIDICIDiscoveryManager, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MIDICIDiscoveryManager").unwrap()) };
        if is_kind_of {
            Ok(MIDICIDiscoveryManager(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to MIDICIDiscoveryManager")
        }
    }
}
impl IMIDICIDiscoveryManager for MIDICIDiscoveryManager {}
pub trait IMIDICIDiscoveryManager: Sized + std::ops::Deref {
    unsafe fn discoverWithHandler_(&self, completedHandler: MIDICIDiscoveryResponseBlock)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, discoverWithHandler : completedHandler)
    }
    unsafe fn sharedInstance() -> MIDICIDiscoveryManager
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MIDICIDiscoveryManager").unwrap(), sharedInstance)
    }
}
pub trait PMIDICIProfileResponderDelegate: Sized + std::ops::Deref {
    unsafe fn connectInitiator_withDeviceInfo_(
        &self,
        initiatorMUID: NSNumber,
        deviceInfo: MIDICIDeviceInfo,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, connectInitiator : initiatorMUID, withDeviceInfo : deviceInfo)
    }
    unsafe fn initiatorDisconnected_(&self, initiatorMUID: NSNumber)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initiatorDisconnected : initiatorMUID)
    }
    unsafe fn willSetProfile_onChannel_enabled_(
        &self,
        aProfile: MIDICIProfile,
        channel: MIDIChannelNumber,
        shouldEnable: BOOL,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, willSetProfile : aProfile, onChannel : channel, enabled : shouldEnable)
    }
    unsafe fn handleDataForProfile_onChannel_data_(
        &self,
        aProfile: MIDICIProfile,
        channel: MIDIChannelNumber,
        inData: NSData,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, handleDataForProfile : aProfile, onChannel : channel, data : inData)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MIDICIResponder(pub id);
impl std::ops::Deref for MIDICIResponder {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MIDICIResponder {}
impl MIDICIResponder {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MIDICIResponder").unwrap(), alloc) })
    }
}
impl INSObject for MIDICIResponder {}
impl PNSObject for MIDICIResponder {}
impl std::convert::TryFrom<NSObject> for MIDICIResponder {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<MIDICIResponder, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MIDICIResponder").unwrap()) };
        if is_kind_of {
            Ok(MIDICIResponder(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to MIDICIResponder")
        }
    }
}
impl IMIDICIResponder for MIDICIResponder {}
pub trait IMIDICIResponder: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn initWithDeviceInfo_profileDelegate_profileStates_supportProperties_(
        &self,
        deviceInfo: MIDICIDeviceInfo,
        delegate: *mut u64,
        profileList: NSArray,
        propertiesSupported: BOOL,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithDeviceInfo : deviceInfo, profileDelegate : delegate, profileStates : profileList, supportProperties : propertiesSupported)
    }
    unsafe fn notifyProfile_onChannel_isEnabled_(
        &self,
        aProfile: MIDICIProfile,
        channel: MIDIChannelNumber,
        enabledState: BOOL,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, notifyProfile : aProfile, onChannel : channel, isEnabled : enabledState)
    }
    unsafe fn sendProfile_onChannel_profileData_(
        &self,
        aProfile: MIDICIProfile,
        channel: MIDIChannelNumber,
        profileSpecificData: NSData,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, sendProfile : aProfile, onChannel : channel, profileData : profileSpecificData)
    }
    unsafe fn start(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, start)
    }
    unsafe fn stop(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, stop)
    }
    unsafe fn initiators(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, initiators)
    }
    unsafe fn profileDelegate(&self) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, profileDelegate)
    }
    unsafe fn deviceInfo(&self) -> MIDICIDeviceInfo
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, deviceInfo)
    }
}
unsafe extern "C" {
    pub static kMIDIPropertyName: CFStringRef;
}
unsafe extern "C" {
    pub static kMIDIPropertyManufacturer: CFStringRef;
}
unsafe extern "C" {
    pub static kMIDIPropertyModel: CFStringRef;
}
unsafe extern "C" {
    pub static kMIDIPropertyUniqueID: CFStringRef;
}
unsafe extern "C" {
    pub static kMIDIPropertyDeviceID: CFStringRef;
}
unsafe extern "C" {
    pub static kMIDIPropertyReceiveChannels: CFStringRef;
}
unsafe extern "C" {
    pub static kMIDIPropertyTransmitChannels: CFStringRef;
}
unsafe extern "C" {
    pub static kMIDIPropertyMaxSysExSpeed: CFStringRef;
}
unsafe extern "C" {
    pub static kMIDIPropertyAdvanceScheduleTimeMuSec: CFStringRef;
}
unsafe extern "C" {
    pub static kMIDIPropertyIsEmbeddedEntity: CFStringRef;
}
unsafe extern "C" {
    pub static kMIDIPropertyIsBroadcast: CFStringRef;
}
unsafe extern "C" {
    pub static kMIDIPropertySingleRealtimeEntity: CFStringRef;
}
unsafe extern "C" {
    pub static kMIDIPropertyConnectionUniqueID: CFStringRef;
}
unsafe extern "C" {
    pub static kMIDIPropertyOffline: CFStringRef;
}
unsafe extern "C" {
    pub static kMIDIPropertyPrivate: CFStringRef;
}
unsafe extern "C" {
    pub static kMIDIPropertyDriverOwner: CFStringRef;
}
unsafe extern "C" {
    pub static kMIDIPropertyFactoryPatchNameFile: CFStringRef;
}
unsafe extern "C" {
    pub static kMIDIPropertyUserPatchNameFile: CFStringRef;
}
unsafe extern "C" {
    pub static kMIDIPropertyNameConfiguration: CFStringRef;
}
unsafe extern "C" {
    pub static kMIDIPropertyNameConfigurationDictionary: CFStringRef;
}
unsafe extern "C" {
    pub static kMIDIPropertyImage: CFStringRef;
}
unsafe extern "C" {
    pub static kMIDIPropertyDriverVersion: CFStringRef;
}
unsafe extern "C" {
    pub static kMIDIPropertySupportsGeneralMIDI: CFStringRef;
}
unsafe extern "C" {
    pub static kMIDIPropertySupportsMMC: CFStringRef;
}
unsafe extern "C" {
    pub static kMIDIPropertyCanRoute: CFStringRef;
}
unsafe extern "C" {
    pub static kMIDIPropertyReceivesClock: CFStringRef;
}
unsafe extern "C" {
    pub static kMIDIPropertyReceivesMTC: CFStringRef;
}
unsafe extern "C" {
    pub static kMIDIPropertyReceivesNotes: CFStringRef;
}
unsafe extern "C" {
    pub static kMIDIPropertyReceivesProgramChanges: CFStringRef;
}
unsafe extern "C" {
    pub static kMIDIPropertyReceivesBankSelectMSB: CFStringRef;
}
unsafe extern "C" {
    pub static kMIDIPropertyReceivesBankSelectLSB: CFStringRef;
}
unsafe extern "C" {
    pub static kMIDIPropertyTransmitsClock: CFStringRef;
}
unsafe extern "C" {
    pub static kMIDIPropertyTransmitsMTC: CFStringRef;
}
unsafe extern "C" {
    pub static kMIDIPropertyTransmitsNotes: CFStringRef;
}
unsafe extern "C" {
    pub static kMIDIPropertyTransmitsProgramChanges: CFStringRef;
}
unsafe extern "C" {
    pub static kMIDIPropertyTransmitsBankSelectMSB: CFStringRef;
}
unsafe extern "C" {
    pub static kMIDIPropertyTransmitsBankSelectLSB: CFStringRef;
}
unsafe extern "C" {
    pub static kMIDIPropertyPanDisruptsStereo: CFStringRef;
}
unsafe extern "C" {
    pub static kMIDIPropertyIsSampler: CFStringRef;
}
unsafe extern "C" {
    pub static kMIDIPropertyIsDrumMachine: CFStringRef;
}
unsafe extern "C" {
    pub static kMIDIPropertyIsMixer: CFStringRef;
}
unsafe extern "C" {
    pub static kMIDIPropertyIsEffectUnit: CFStringRef;
}
unsafe extern "C" {
    pub static kMIDIPropertyMaxReceiveChannels: CFStringRef;
}
unsafe extern "C" {
    pub static kMIDIPropertyMaxTransmitChannels: CFStringRef;
}
unsafe extern "C" {
    pub static kMIDIPropertyDriverDeviceEditorApp: CFStringRef;
}
unsafe extern "C" {
    pub static kMIDIPropertySupportsShowControl: CFStringRef;
}
unsafe extern "C" {
    pub static kMIDIPropertyDisplayName: CFStringRef;
}
unsafe extern "C" {
    pub static kMIDIPropertyProtocolID: CFStringRef;
}
unsafe extern "C" {
    pub static kMIDIPropertyUMPActiveGroupBitmap: CFStringRef;
}
unsafe extern "C" {
    pub static kMIDIPropertyUMPCanTransmitGroupless: CFStringRef;
}
unsafe extern "C" {
    pub static kMIDIPropertyAssociatedEndpoint: CFStringRef;
}
unsafe extern "C" {
    pub fn MIDIClientCreate(
        name: CFStringRef,
        notifyProc: MIDINotifyProc,
        notifyRefCon: *mut ::std::os::raw::c_void,
        outClient: *mut MIDIClientRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn MIDIClientCreateWithBlock(
        name: CFStringRef,
        outClient: *mut MIDIClientRef,
        notifyBlock: MIDINotifyBlock,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn MIDIClientDispose(client: MIDIClientRef) -> OSStatus;
}
unsafe extern "C" {
    pub fn MIDIInputPortCreateWithProtocol(
        client: MIDIClientRef,
        portName: CFStringRef,
        protocol: MIDIProtocolID,
        outPort: *mut MIDIPortRef,
        receiveBlock: MIDIReceiveBlock,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn MIDIInputPortCreate(
        client: MIDIClientRef,
        portName: CFStringRef,
        readProc: MIDIReadProc,
        refCon: *mut ::std::os::raw::c_void,
        outPort: *mut MIDIPortRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn MIDIInputPortCreateWithBlock(
        client: MIDIClientRef,
        portName: CFStringRef,
        outPort: *mut MIDIPortRef,
        readBlock: MIDIReadBlock,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn MIDIOutputPortCreate(
        client: MIDIClientRef,
        portName: CFStringRef,
        outPort: *mut MIDIPortRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn MIDIPortDispose(port: MIDIPortRef) -> OSStatus;
}
unsafe extern "C" {
    pub fn MIDIPortConnectSource(
        port: MIDIPortRef,
        source: MIDIEndpointRef,
        connRefCon: *mut ::std::os::raw::c_void,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn MIDIPortDisconnectSource(port: MIDIPortRef, source: MIDIEndpointRef) -> OSStatus;
}
unsafe extern "C" {
    pub fn MIDIGetNumberOfDevices() -> ItemCount;
}
unsafe extern "C" {
    pub fn MIDIGetDevice(deviceIndex0: ItemCount) -> MIDIDeviceRef;
}
unsafe extern "C" {
    pub fn MIDIDeviceGetNumberOfEntities(device: MIDIDeviceRef) -> ItemCount;
}
unsafe extern "C" {
    pub fn MIDIDeviceGetEntity(device: MIDIDeviceRef, entityIndex0: ItemCount) -> MIDIEntityRef;
}
unsafe extern "C" {
    pub fn MIDIEntityGetNumberOfSources(entity: MIDIEntityRef) -> ItemCount;
}
unsafe extern "C" {
    pub fn MIDIEntityGetSource(entity: MIDIEntityRef, sourceIndex0: ItemCount) -> MIDIEndpointRef;
}
unsafe extern "C" {
    pub fn MIDIEntityGetNumberOfDestinations(entity: MIDIEntityRef) -> ItemCount;
}
unsafe extern "C" {
    pub fn MIDIEntityGetDestination(
        entity: MIDIEntityRef,
        destIndex0: ItemCount,
    ) -> MIDIEndpointRef;
}
unsafe extern "C" {
    pub fn MIDIEntityGetDevice(inEntity: MIDIEntityRef, outDevice: *mut MIDIDeviceRef) -> OSStatus;
}
unsafe extern "C" {
    pub fn MIDIGetNumberOfSources() -> ItemCount;
}
unsafe extern "C" {
    pub fn MIDIGetSource(sourceIndex0: ItemCount) -> MIDIEndpointRef;
}
unsafe extern "C" {
    pub fn MIDIGetNumberOfDestinations() -> ItemCount;
}
unsafe extern "C" {
    pub fn MIDIGetDestination(destIndex0: ItemCount) -> MIDIEndpointRef;
}
unsafe extern "C" {
    pub fn MIDIEndpointGetEntity(
        inEndpoint: MIDIEndpointRef,
        outEntity: *mut MIDIEntityRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn MIDIDestinationCreateWithProtocol(
        client: MIDIClientRef,
        name: CFStringRef,
        protocol: MIDIProtocolID,
        outDest: *mut MIDIEndpointRef,
        readBlock: MIDIReceiveBlock,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn MIDIDestinationCreate(
        client: MIDIClientRef,
        name: CFStringRef,
        readProc: MIDIReadProc,
        refCon: *mut ::std::os::raw::c_void,
        outDest: *mut MIDIEndpointRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn MIDIDestinationCreateWithBlock(
        client: MIDIClientRef,
        name: CFStringRef,
        outDest: *mut MIDIEndpointRef,
        readBlock: MIDIReadBlock,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn MIDISourceCreateWithProtocol(
        client: MIDIClientRef,
        name: CFStringRef,
        protocol: MIDIProtocolID,
        outSrc: *mut MIDIEndpointRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn MIDISourceCreate(
        client: MIDIClientRef,
        name: CFStringRef,
        outSrc: *mut MIDIEndpointRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn MIDIEndpointDispose(endpt: MIDIEndpointRef) -> OSStatus;
}
unsafe extern "C" {
    pub fn MIDIGetNumberOfExternalDevices() -> ItemCount;
}
unsafe extern "C" {
    pub fn MIDIGetExternalDevice(deviceIndex0: ItemCount) -> MIDIDeviceRef;
}
unsafe extern "C" {
    pub fn MIDIObjectGetIntegerProperty(
        obj: MIDIObjectRef,
        propertyID: CFStringRef,
        outValue: *mut SInt32,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn MIDIObjectSetIntegerProperty(
        obj: MIDIObjectRef,
        propertyID: CFStringRef,
        value: SInt32,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn MIDIObjectGetStringProperty(
        obj: MIDIObjectRef,
        propertyID: CFStringRef,
        str_: *mut CFStringRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn MIDIObjectSetStringProperty(
        obj: MIDIObjectRef,
        propertyID: CFStringRef,
        str_: CFStringRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn MIDIObjectGetDataProperty(
        obj: MIDIObjectRef,
        propertyID: CFStringRef,
        outData: *mut CFDataRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn MIDIObjectSetDataProperty(
        obj: MIDIObjectRef,
        propertyID: CFStringRef,
        data: CFDataRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn MIDIObjectGetDictionaryProperty(
        obj: MIDIObjectRef,
        propertyID: CFStringRef,
        outDict: *mut CFDictionaryRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn MIDIObjectSetDictionaryProperty(
        obj: MIDIObjectRef,
        propertyID: CFStringRef,
        dict: CFDictionaryRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn MIDIObjectGetProperties(
        obj: MIDIObjectRef,
        outProperties: *mut CFPropertyListRef,
        deep: Boolean,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn MIDIObjectRemoveProperty(obj: MIDIObjectRef, propertyID: CFStringRef) -> OSStatus;
}
unsafe extern "C" {
    pub fn MIDIObjectFindByUniqueID(
        inUniqueID: MIDIUniqueID,
        outObject: *mut MIDIObjectRef,
        outObjectType: *mut MIDIObjectType,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn MIDISendEventList(
        port: MIDIPortRef,
        dest: MIDIEndpointRef,
        evtlist: *const MIDIEventList,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn MIDISend(
        port: MIDIPortRef,
        dest: MIDIEndpointRef,
        pktlist: *const MIDIPacketList,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn MIDISendSysex(request: *mut MIDISysexSendRequest) -> OSStatus;
}
unsafe extern "C" {
    pub fn MIDISendUMPSysex(umpRequest: *mut MIDISysexSendRequestUMP) -> OSStatus;
}
unsafe extern "C" {
    pub fn MIDISendUMPSysex8(umpRequest: *mut MIDISysexSendRequestUMP) -> OSStatus;
}
unsafe extern "C" {
    pub fn MIDIEventPacketSysexBytesForGroup(
        pkt: *const MIDIEventPacket,
        groupIndex: UInt8,
        outData: *mut CFDataRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn MIDIReceivedEventList(src: MIDIEndpointRef, evtlist: *const MIDIEventList) -> OSStatus;
}
unsafe extern "C" {
    pub fn MIDIReceived(src: MIDIEndpointRef, pktlist: *const MIDIPacketList) -> OSStatus;
}
unsafe extern "C" {
    pub fn MIDIFlushOutput(dest: MIDIEndpointRef) -> OSStatus;
}
unsafe extern "C" {
    pub fn MIDIRestart() -> OSStatus;
}
unsafe extern "C" {
    pub fn MIDIEventListInit(
        evtlist: *mut MIDIEventList,
        protocol: MIDIProtocolID,
    ) -> *mut MIDIEventPacket;
}
unsafe extern "C" {
    pub fn MIDIEventListAdd(
        evtlist: *mut MIDIEventList,
        listSize: ByteCount,
        curPacket: *mut MIDIEventPacket,
        time: MIDITimeStamp,
        wordCount: ByteCount,
        words: *const UInt32,
    ) -> *mut MIDIEventPacket;
}
unsafe extern "C" {
    pub fn MIDIPacketListInit(pktlist: *mut MIDIPacketList) -> *mut MIDIPacket;
}
unsafe extern "C" {
    pub fn MIDIPacketListAdd(
        pktlist: *mut MIDIPacketList,
        listSize: ByteCount,
        curPacket: *mut MIDIPacket,
        time: MIDITimeStamp,
        nData: ByteCount,
        data: *const Byte,
    ) -> *mut MIDIPacket;
}
unsafe extern "C" {
    pub fn MIDISetupCreate(outSetup: *mut MIDISetupRef) -> OSStatus;
}
unsafe extern "C" {
    pub fn MIDISetupDispose(setup: MIDISetupRef) -> OSStatus;
}
unsafe extern "C" {
    pub fn MIDISetupInstall(setup: MIDISetupRef) -> OSStatus;
}
unsafe extern "C" {
    pub fn MIDISetupGetCurrent(outSetup: *mut MIDISetupRef) -> OSStatus;
}
unsafe extern "C" {
    pub fn MIDISetupToData(setup: MIDISetupRef, outData: *mut CFDataRef) -> OSStatus;
}
unsafe extern "C" {
    pub fn MIDISetupFromData(data: CFDataRef, outSetup: *mut MIDISetupRef) -> OSStatus;
}
unsafe extern "C" {
    pub fn MIDIDeviceNewEntity(
        device: MIDIDeviceRef,
        name: CFStringRef,
        protocol: MIDIProtocolID,
        embedded: Boolean,
        numSourceEndpoints: ItemCount,
        numDestinationEndpoints: ItemCount,
        newEntity: *mut MIDIEntityRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn MIDIDeviceAddEntity(
        device: MIDIDeviceRef,
        name: CFStringRef,
        embedded: Boolean,
        numSourceEndpoints: ItemCount,
        numDestinationEndpoints: ItemCount,
        newEntity: *mut MIDIEntityRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn MIDIDeviceRemoveEntity(device: MIDIDeviceRef, entity: MIDIEntityRef) -> OSStatus;
}
unsafe extern "C" {
    pub fn MIDIEntityAddOrRemoveEndpoints(
        entity: MIDIEntityRef,
        numSourceEndpoints: ItemCount,
        numDestinationEndpoints: ItemCount,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn MIDISetupAddDevice(device: MIDIDeviceRef) -> OSStatus;
}
unsafe extern "C" {
    pub fn MIDISetupRemoveDevice(device: MIDIDeviceRef) -> OSStatus;
}
unsafe extern "C" {
    pub fn MIDISetupAddExternalDevice(device: MIDIDeviceRef) -> OSStatus;
}
unsafe extern "C" {
    pub fn MIDISetupRemoveExternalDevice(device: MIDIDeviceRef) -> OSStatus;
}
unsafe extern "C" {
    pub fn MIDIGetSerialPortOwner(
        portName: CFStringRef,
        outDriverName: *mut CFStringRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn MIDISetSerialPortOwner(portName: CFStringRef, driverName: CFStringRef) -> OSStatus;
}
unsafe extern "C" {
    pub fn MIDIGetSerialPortDrivers(outDriverNames: *mut CFArrayRef) -> OSStatus;
}
unsafe extern "C" {
    pub fn MIDIExternalDeviceCreate(
        name: CFStringRef,
        manufacturer: CFStringRef,
        model: CFStringRef,
        outDevice: *mut MIDIDeviceRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn MIDIThruConnectionParamsInitialize(inConnectionParams: *mut MIDIThruConnectionParams);
}
unsafe extern "C" {
    pub fn MIDIThruConnectionCreate(
        inPersistentOwnerID: CFStringRef,
        inConnectionParams: CFDataRef,
        outConnection: *mut MIDIThruConnectionRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn MIDIThruConnectionDispose(connection: MIDIThruConnectionRef) -> OSStatus;
}
unsafe extern "C" {
    pub fn MIDIThruConnectionGetParams(
        connection: MIDIThruConnectionRef,
        outConnectionParams: *mut CFDataRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn MIDIThruConnectionSetParams(
        connection: MIDIThruConnectionRef,
        inConnectionParams: CFDataRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn MIDIThruConnectionFind(
        inPersistentOwnerID: CFStringRef,
        outConnectionList: *mut CFDataRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub static kMIDIDriverPropertyUsesSerial: CFStringRef;
}
unsafe extern "C" {
    pub fn MIDIDeviceCreate(
        owner: MIDIDriverRef,
        name: CFStringRef,
        manufacturer: CFStringRef,
        model: CFStringRef,
        outDevice: *mut MIDIDeviceRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn MIDIDeviceDispose(device: MIDIDeviceRef) -> OSStatus;
}
unsafe extern "C" {
    pub fn MIDIDeviceListGetNumberOfDevices(devList: MIDIDeviceListRef) -> ItemCount;
}
unsafe extern "C" {
    pub fn MIDIDeviceListGetDevice(devList: MIDIDeviceListRef, index0: ItemCount) -> MIDIDeviceRef;
}
unsafe extern "C" {
    pub fn MIDIDeviceListAddDevice(devList: MIDIDeviceListRef, dev: MIDIDeviceRef) -> OSStatus;
}
unsafe extern "C" {
    pub fn MIDIDeviceListDispose(devList: MIDIDeviceListRef) -> OSStatus;
}
unsafe extern "C" {
    pub fn MIDIEndpointSetRefCons(
        endpt: MIDIEndpointRef,
        ref1: *mut ::std::os::raw::c_void,
        ref2: *mut ::std::os::raw::c_void,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn MIDIEndpointGetRefCons(
        endpt: MIDIEndpointRef,
        ref1: *mut *mut ::std::os::raw::c_void,
        ref2: *mut *mut ::std::os::raw::c_void,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn MIDIGetDriverIORunLoop() -> CFRunLoopRef;
}
unsafe extern "C" {
    pub fn MIDIGetDriverDeviceList(driver: MIDIDriverRef) -> MIDIDeviceListRef;
}
unsafe extern "C" {
    pub fn MIDIDriverEnableMonitoring(driver: MIDIDriverRef, enabled: Boolean) -> OSStatus;
}
unsafe extern "C" {
    pub fn MIDIEventListForEachEvent(
        evtlist: *const MIDIEventList,
        visitor: MIDIEventVisitor,
        visitorContext: *mut ::std::os::raw::c_void,
    );
}
unsafe extern "C" {
    pub fn MIDIBluetoothDriverActivateAllConnections() -> OSStatus;
}
unsafe extern "C" {
    pub fn MIDIBluetoothDriverDisconnect(uuid: CFStringRef) -> OSStatus;
}
unsafe extern "C" {
    pub static MIDINetworkBonjourServiceType: NSString;
}
unsafe extern "C" {
    pub static MIDINetworkNotificationContactsDidChange: NSString;
}
unsafe extern "C" {
    pub static MIDINetworkNotificationSessionDidChange: NSString;
}
unsafe extern "C" {
    pub static MIDIUMPEndpointWasAddedNotification: NSNotificationName;
}
unsafe extern "C" {
    pub static MIDIUMPEndpointWasRemovedNotification: NSNotificationName;
}
unsafe extern "C" {
    pub static MIDIUMPEndpointWasUpdatedNotification: NSNotificationName;
}
unsafe extern "C" {
    pub static MIDIUMPFunctionBlockWasUpdatedNotification: NSNotificationName;
}
unsafe extern "C" {
    pub static mut MIDIUMPEndpointObjectKey: MIDIUMPEndpointManagerDictionaryKey;
}
unsafe extern "C" {
    pub static mut MIDIUMPFunctionBlockObjectKey: MIDIUMPEndpointManagerDictionaryKey;
}
unsafe extern "C" {
    pub static MIDICIDeviceWasAddedNotification: NSNotificationName;
}
unsafe extern "C" {
    pub static MIDICIDeviceWasRemovedNotification: NSNotificationName;
}
unsafe extern "C" {
    pub static MIDICIProfileWasUpdatedNotification: NSNotificationName;
}
unsafe extern "C" {
    pub static MIDICIProfileWasRemovedNotification: NSNotificationName;
}
unsafe extern "C" {
    pub static mut MIDICIDeviceObjectKey: MIDICIDeviceManagerDictionaryKey;
}
unsafe extern "C" {
    pub static mut MIDICIProfileObjectKey: MIDICIDeviceManagerDictionaryKey;
}

unsafe impl objc2::encode::RefEncode for MIDIEventPacket {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MIDIEventPacket {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("MIDIEventPacket", &[]);
}
unsafe impl objc2::encode::RefEncode for MIDIEventList {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MIDIEventList {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("MIDIEventList", &[]);
}
unsafe impl objc2::encode::RefEncode for MIDIPacket {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MIDIPacket {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("MIDIPacket", &[]);
}
unsafe impl objc2::encode::RefEncode for MIDIPacketList {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MIDIPacketList {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("MIDIPacketList", &[]);
}
unsafe impl objc2::encode::RefEncode for MIDISysexSendRequest {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MIDISysexSendRequest {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("MIDISysexSendRequest", &[]);
}
unsafe impl objc2::encode::RefEncode for MIDISysexSendRequestUMP {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MIDISysexSendRequestUMP {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("MIDISysexSendRequestUMP", &[]);
}
unsafe impl objc2::encode::RefEncode for MIDINotification {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MIDINotification {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("MIDINotification", &[]);
}
unsafe impl objc2::encode::RefEncode for MIDIObjectAddRemoveNotification {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MIDIObjectAddRemoveNotification {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("MIDIObjectAddRemoveNotification", &[]);
}
unsafe impl objc2::encode::RefEncode for MIDIObjectPropertyChangeNotification {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MIDIObjectPropertyChangeNotification {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("MIDIObjectPropertyChangeNotification", &[]);
}
unsafe impl objc2::encode::RefEncode for MIDIIOErrorNotification {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MIDIIOErrorNotification {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("MIDIIOErrorNotification", &[]);
}
unsafe impl objc2::encode::RefEncode for MIDIValueMap {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MIDIValueMap {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("MIDIValueMap", &[]);
}
unsafe impl objc2::encode::RefEncode for MIDITransform {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MIDITransform {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("MIDITransform", &[]);
}
unsafe impl objc2::encode::RefEncode for MIDIControlTransform {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MIDIControlTransform {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("MIDIControlTransform", &[]);
}
unsafe impl objc2::encode::RefEncode for MIDIThruConnectionEndpoint {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MIDIThruConnectionEndpoint {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("MIDIThruConnectionEndpoint", &[]);
}
unsafe impl objc2::encode::RefEncode for MIDIThruConnectionParams {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MIDIThruConnectionParams {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("MIDIThruConnectionParams", &[]);
}
unsafe impl objc2::encode::RefEncode for MIDIDriverInterface {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MIDIDriverInterface {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("MIDIDriverInterface", &[]);
}
unsafe impl objc2::encode::RefEncode for MIDIMessage_64 {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MIDIMessage_64 {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("MIDIMessage_64", &[]);
}
unsafe impl objc2::encode::RefEncode for MIDIMessage_96 {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MIDIMessage_96 {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("MIDIMessage_96", &[]);
}
unsafe impl objc2::encode::RefEncode for MIDIMessage_128 {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MIDIMessage_128 {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("MIDIMessage_128", &[]);
}
unsafe impl objc2::encode::RefEncode for MIDIUniversalMessage {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MIDIUniversalMessage {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("MIDIUniversalMessage", &[]);
}
unsafe impl objc2::encode::RefEncode for MIDIUniversalMessage__bindgen_ty_1 {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MIDIUniversalMessage__bindgen_ty_1 {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("MIDIUniversalMessage__bindgen_ty_1", &[]);
}
unsafe impl objc2::encode::RefEncode for MIDIUniversalMessage__bindgen_ty_1__bindgen_ty_1 {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MIDIUniversalMessage__bindgen_ty_1__bindgen_ty_1 {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("MIDIUniversalMessage__bindgen_ty_1__bindgen_ty_1", &[]);
}
unsafe impl objc2::encode::RefEncode for MIDIUniversalMessage__bindgen_ty_1__bindgen_ty_1__bindgen_ty_1 {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MIDIUniversalMessage__bindgen_ty_1__bindgen_ty_1__bindgen_ty_1 {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("MIDIUniversalMessage__bindgen_ty_1__bindgen_ty_1__bindgen_ty_1", &[]);
}
unsafe impl objc2::encode::RefEncode for MIDIUniversalMessage__bindgen_ty_1__bindgen_ty_2 {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MIDIUniversalMessage__bindgen_ty_1__bindgen_ty_2 {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("MIDIUniversalMessage__bindgen_ty_1__bindgen_ty_2", &[]);
}
unsafe impl objc2::encode::RefEncode for MIDIUniversalMessage__bindgen_ty_1__bindgen_ty_2__bindgen_ty_1 {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MIDIUniversalMessage__bindgen_ty_1__bindgen_ty_2__bindgen_ty_1 {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("MIDIUniversalMessage__bindgen_ty_1__bindgen_ty_2__bindgen_ty_1", &[]);
}
unsafe impl objc2::encode::RefEncode for MIDIUniversalMessage__bindgen_ty_1__bindgen_ty_3 {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MIDIUniversalMessage__bindgen_ty_1__bindgen_ty_3 {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("MIDIUniversalMessage__bindgen_ty_1__bindgen_ty_3", &[]);
}
unsafe impl objc2::encode::RefEncode for MIDIUniversalMessage__bindgen_ty_1__bindgen_ty_3__bindgen_ty_1 {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MIDIUniversalMessage__bindgen_ty_1__bindgen_ty_3__bindgen_ty_1 {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("MIDIUniversalMessage__bindgen_ty_1__bindgen_ty_3__bindgen_ty_1", &[]);
}
unsafe impl objc2::encode::RefEncode for MIDIUniversalMessage__bindgen_ty_1__bindgen_ty_3__bindgen_ty_1__bindgen_ty_1 {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MIDIUniversalMessage__bindgen_ty_1__bindgen_ty_3__bindgen_ty_1__bindgen_ty_1 {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("MIDIUniversalMessage__bindgen_ty_1__bindgen_ty_3__bindgen_ty_1__bindgen_ty_1", &[]);
}
unsafe impl objc2::encode::RefEncode for MIDIUniversalMessage__bindgen_ty_1__bindgen_ty_3__bindgen_ty_1__bindgen_ty_2 {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MIDIUniversalMessage__bindgen_ty_1__bindgen_ty_3__bindgen_ty_1__bindgen_ty_2 {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("MIDIUniversalMessage__bindgen_ty_1__bindgen_ty_3__bindgen_ty_1__bindgen_ty_2", &[]);
}
unsafe impl objc2::encode::RefEncode for MIDIUniversalMessage__bindgen_ty_1__bindgen_ty_3__bindgen_ty_1__bindgen_ty_3 {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MIDIUniversalMessage__bindgen_ty_1__bindgen_ty_3__bindgen_ty_1__bindgen_ty_3 {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("MIDIUniversalMessage__bindgen_ty_1__bindgen_ty_3__bindgen_ty_1__bindgen_ty_3", &[]);
}
unsafe impl objc2::encode::RefEncode for MIDIUniversalMessage__bindgen_ty_1__bindgen_ty_4 {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MIDIUniversalMessage__bindgen_ty_1__bindgen_ty_4 {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("MIDIUniversalMessage__bindgen_ty_1__bindgen_ty_4", &[]);
}
unsafe impl objc2::encode::RefEncode for MIDIUniversalMessage__bindgen_ty_1__bindgen_ty_5 {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MIDIUniversalMessage__bindgen_ty_1__bindgen_ty_5 {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("MIDIUniversalMessage__bindgen_ty_1__bindgen_ty_5", &[]);
}
unsafe impl objc2::encode::RefEncode for MIDIUniversalMessage__bindgen_ty_1__bindgen_ty_5__bindgen_ty_1 {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MIDIUniversalMessage__bindgen_ty_1__bindgen_ty_5__bindgen_ty_1 {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("MIDIUniversalMessage__bindgen_ty_1__bindgen_ty_5__bindgen_ty_1", &[]);
}
unsafe impl objc2::encode::RefEncode for MIDIUniversalMessage__bindgen_ty_1__bindgen_ty_5__bindgen_ty_1__bindgen_ty_1 {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MIDIUniversalMessage__bindgen_ty_1__bindgen_ty_5__bindgen_ty_1__bindgen_ty_1 {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("MIDIUniversalMessage__bindgen_ty_1__bindgen_ty_5__bindgen_ty_1__bindgen_ty_1", &[]);
}
unsafe impl objc2::encode::RefEncode for MIDIUniversalMessage__bindgen_ty_1__bindgen_ty_5__bindgen_ty_1__bindgen_ty_2 {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MIDIUniversalMessage__bindgen_ty_1__bindgen_ty_5__bindgen_ty_1__bindgen_ty_2 {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("MIDIUniversalMessage__bindgen_ty_1__bindgen_ty_5__bindgen_ty_1__bindgen_ty_2", &[]);
}
unsafe impl objc2::encode::RefEncode for MIDIUniversalMessage__bindgen_ty_1__bindgen_ty_5__bindgen_ty_1__bindgen_ty_3 {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MIDIUniversalMessage__bindgen_ty_1__bindgen_ty_5__bindgen_ty_1__bindgen_ty_3 {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("MIDIUniversalMessage__bindgen_ty_1__bindgen_ty_5__bindgen_ty_1__bindgen_ty_3", &[]);
}
unsafe impl objc2::encode::RefEncode for MIDIUniversalMessage__bindgen_ty_1__bindgen_ty_5__bindgen_ty_1__bindgen_ty_4 {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MIDIUniversalMessage__bindgen_ty_1__bindgen_ty_5__bindgen_ty_1__bindgen_ty_4 {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("MIDIUniversalMessage__bindgen_ty_1__bindgen_ty_5__bindgen_ty_1__bindgen_ty_4", &[]);
}
unsafe impl objc2::encode::RefEncode for MIDIUniversalMessage__bindgen_ty_1__bindgen_ty_5__bindgen_ty_1__bindgen_ty_5 {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MIDIUniversalMessage__bindgen_ty_1__bindgen_ty_5__bindgen_ty_1__bindgen_ty_5 {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("MIDIUniversalMessage__bindgen_ty_1__bindgen_ty_5__bindgen_ty_1__bindgen_ty_5", &[]);
}
unsafe impl objc2::encode::RefEncode for MIDIUniversalMessage__bindgen_ty_1__bindgen_ty_5__bindgen_ty_1__bindgen_ty_6 {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MIDIUniversalMessage__bindgen_ty_1__bindgen_ty_5__bindgen_ty_1__bindgen_ty_6 {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("MIDIUniversalMessage__bindgen_ty_1__bindgen_ty_5__bindgen_ty_1__bindgen_ty_6", &[]);
}
unsafe impl objc2::encode::RefEncode for MIDIUniversalMessage__bindgen_ty_1__bindgen_ty_5__bindgen_ty_1__bindgen_ty_7 {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MIDIUniversalMessage__bindgen_ty_1__bindgen_ty_5__bindgen_ty_1__bindgen_ty_7 {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("MIDIUniversalMessage__bindgen_ty_1__bindgen_ty_5__bindgen_ty_1__bindgen_ty_7", &[]);
}
unsafe impl objc2::encode::RefEncode for MIDIUniversalMessage__bindgen_ty_1__bindgen_ty_5__bindgen_ty_1__bindgen_ty_8 {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MIDIUniversalMessage__bindgen_ty_1__bindgen_ty_5__bindgen_ty_1__bindgen_ty_8 {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("MIDIUniversalMessage__bindgen_ty_1__bindgen_ty_5__bindgen_ty_1__bindgen_ty_8", &[]);
}
unsafe impl objc2::encode::RefEncode for MIDIUniversalMessage__bindgen_ty_1__bindgen_ty_5__bindgen_ty_1__bindgen_ty_9 {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MIDIUniversalMessage__bindgen_ty_1__bindgen_ty_5__bindgen_ty_1__bindgen_ty_9 {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("MIDIUniversalMessage__bindgen_ty_1__bindgen_ty_5__bindgen_ty_1__bindgen_ty_9", &[]);
}
unsafe impl objc2::encode::RefEncode for MIDIUniversalMessage__bindgen_ty_1__bindgen_ty_5__bindgen_ty_1__bindgen_ty_10 {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MIDIUniversalMessage__bindgen_ty_1__bindgen_ty_5__bindgen_ty_1__bindgen_ty_10 {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("MIDIUniversalMessage__bindgen_ty_1__bindgen_ty_5__bindgen_ty_1__bindgen_ty_10", &[]);
}
unsafe impl objc2::encode::RefEncode for MIDIUniversalMessage__bindgen_ty_1__bindgen_ty_6 {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MIDIUniversalMessage__bindgen_ty_1__bindgen_ty_6 {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("MIDIUniversalMessage__bindgen_ty_1__bindgen_ty_6", &[]);
}
unsafe impl objc2::encode::RefEncode for MIDIUniversalMessage__bindgen_ty_1__bindgen_ty_6__bindgen_ty_1 {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MIDIUniversalMessage__bindgen_ty_1__bindgen_ty_6__bindgen_ty_1 {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("MIDIUniversalMessage__bindgen_ty_1__bindgen_ty_6__bindgen_ty_1", &[]);
}
unsafe impl objc2::encode::RefEncode for MIDIUniversalMessage__bindgen_ty_1__bindgen_ty_6__bindgen_ty_1__bindgen_ty_1 {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MIDIUniversalMessage__bindgen_ty_1__bindgen_ty_6__bindgen_ty_1__bindgen_ty_1 {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("MIDIUniversalMessage__bindgen_ty_1__bindgen_ty_6__bindgen_ty_1__bindgen_ty_1", &[]);
}
unsafe impl objc2::encode::RefEncode for MIDIUniversalMessage__bindgen_ty_1__bindgen_ty_6__bindgen_ty_1__bindgen_ty_2 {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MIDIUniversalMessage__bindgen_ty_1__bindgen_ty_6__bindgen_ty_1__bindgen_ty_2 {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("MIDIUniversalMessage__bindgen_ty_1__bindgen_ty_6__bindgen_ty_1__bindgen_ty_2", &[]);
}
unsafe impl objc2::encode::RefEncode for MIDIUniversalMessage__bindgen_ty_1__bindgen_ty_7 {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MIDIUniversalMessage__bindgen_ty_1__bindgen_ty_7 {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("MIDIUniversalMessage__bindgen_ty_1__bindgen_ty_7", &[]);
}
unsafe impl objc2::encode::RefEncode for MIDI2DeviceManufacturer {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MIDI2DeviceManufacturer {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("MIDI2DeviceManufacturer", &[]);
}
unsafe impl objc2::encode::RefEncode for MIDI2DeviceRevisionLevel {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MIDI2DeviceRevisionLevel {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("MIDI2DeviceRevisionLevel", &[]);
}
unsafe impl objc2::encode::RefEncode for MIDICIProfileIDStandard {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MIDICIProfileIDStandard {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("MIDICIProfileIDStandard", &[]);
}
unsafe impl objc2::encode::RefEncode for MIDICIProfileIDManufacturerSpecific {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MIDICIProfileIDManufacturerSpecific {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("MIDICIProfileIDManufacturerSpecific", &[]);
}
unsafe impl objc2::encode::RefEncode for MIDICIProfileID {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MIDICIProfileID {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("MIDICIProfileID", &[]);
}
unsafe impl objc2::encode::RefEncode for MIDINetworkHost {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MIDINetworkHost {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MIDINetworkConnection {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MIDINetworkConnection {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MIDINetworkSession {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MIDINetworkSession {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MIDI2DeviceInfo {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MIDI2DeviceInfo {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MIDIUMPEndpoint {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MIDIUMPEndpoint {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MIDIUMPEndpointManager {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MIDIUMPEndpointManager {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MIDIUMPFunctionBlock {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MIDIUMPFunctionBlock {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MIDIUMPMutableEndpoint {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MIDIUMPMutableEndpoint {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MIDIUMPMutableFunctionBlock {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MIDIUMPMutableFunctionBlock {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MIDICIDevice {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MIDICIDevice {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MIDIUMPCIProfile {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MIDIUMPCIProfile {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MIDICIDeviceManager {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MIDICIDeviceManager {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MIDICIDeviceIdentification {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MIDICIDeviceIdentification {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("MIDICIDeviceIdentification", &[]);
}
unsafe impl objc2::encode::RefEncode for MIDICIDeviceInfo {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MIDICIDeviceInfo {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MIDICIDiscoveredNode {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MIDICIDiscoveredNode {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MIDICIProfile {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MIDICIProfile {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MIDICIProfileState {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MIDICIProfileState {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MIDICISession {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MIDICISession {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MIDICIDiscoveryManager {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MIDICIDiscoveryManager {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MIDICIResponder {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MIDICIResponder {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
