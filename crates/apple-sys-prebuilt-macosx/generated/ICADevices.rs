#[allow(unused_imports)]
use crate::CoreFoundation::*;
#[allow(unused_imports)]
use crate::CoreGraphics::*;
#[allow(unused_imports)]
use crate::CoreServices::*;
#[allow(unused_imports)]
use crate::CoreText::*;
#[allow(unused_imports)]
use crate::DiskArbitration::*;
#[allow(unused_imports)]
use crate::IOKit::*;
#[allow(unused_imports)]
use crate::Security::*;

#[allow(unused_imports)]
use objc2::msg_send;
pub type ICAError = OSErr;
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct ICAPTPPassThroughPB {
    pub commandCode: UInt32,
    pub resultCode: UInt32,
    pub numOfInputParams: UInt32,
    pub numOfOutputParams: UInt32,
    pub params: [UInt32; 4usize],
    pub dataUsageMode: UInt32,
    pub flags: UInt32,
    pub dataSize: UInt32,
    pub data: [UInt8; 1usize],
}
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct ICAPTPEventDataset {
    pub dataLength: UInt32,
    pub containerType: UInt16,
    pub eventCode: UInt16,
    pub transactionID: UInt32,
    pub params: [UInt32; 3usize],
}
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct ICAHeader {
    pub err: ICAError,
    pub refcon: ::std::os::raw::c_ulong,
}
pub type ICACompletion = ::std::option::Option<unsafe extern "C" fn(pb: *mut ICAHeader)>;
pub type ICAImportFilterProc = ::std::option::Option<
    unsafe extern "C" fn(imageInfo: CFDictionaryRef, refcon: ::std::os::raw::c_ulong) -> Boolean,
>;
pub type ICAObject = UInt32;
pub type ICAProperty = UInt32;
pub type ICAConnectionID = UInt32;
pub type ICASessionID = UInt32;
pub type ICAScannerSessionID = ICASessionID;
pub type ICAEventDataCookie = UInt32;
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct ICAObjectInfo {
    pub objectType: OSType,
    pub objectSubtype: OSType,
}
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct ICAImportImagePB {
    pub header: ICAHeader,
    pub deviceObject: ICAObject,
    pub flags: UInt32,
    pub supportedFileTypes: CFArrayRef,
    pub filterProc: ICAImportFilterProc,
    pub importedImages: *mut CFArrayRef,
}
pub type ICANotification = ::std::option::Option<
    unsafe extern "C" fn(notificationType: CFStringRef, notificationDictionary: CFDictionaryRef),
>;
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct ICARegisterForEventNotificationPB {
    pub header: ICAHeader,
    pub objectOfInterest: ICAObject,
    pub eventsOfInterest: CFArrayRef,
    pub notificationProc: ICANotification,
    pub options: CFDictionaryRef,
}
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct ICASendNotificationPB {
    pub header: ICAHeader,
    pub notificationDictionary: CFMutableDictionaryRef,
    pub replyCode: UInt32,
}
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct ICAGetDeviceListPB {
    pub header: ICAHeader,
    pub object: ICAObject,
}
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct ICACopyObjectPropertyDictionaryPB {
    pub header: ICAHeader,
    pub object: ICAObject,
    pub theDict: *mut CFDictionaryRef,
}
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct ICACopyObjectThumbnailPB {
    pub header: ICAHeader,
    pub object: ICAObject,
    pub thumbnailFormat: OSType,
    pub thumbnailData: *mut CFDataRef,
}
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct ICACopyObjectDataPB {
    pub header: ICAHeader,
    pub object: ICAObject,
    pub startByte: usize,
    pub requestedSize: usize,
    pub data: *mut CFDataRef,
}
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct ICAMessage {
    pub messageType: OSType,
    pub startByte: UInt32,
    pub dataPtr: *mut ::std::os::raw::c_void,
    pub dataSize: UInt32,
    pub dataType: OSType,
}
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct ICAObjectSendMessagePB {
    pub header: ICAHeader,
    pub object: ICAObject,
    pub message: ICAMessage,
    pub result: UInt32,
}
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct ICADownloadFilePB {
    pub header: ICAHeader,
    pub object: ICAObject,
    pub dirFSRef: *mut FSRef,
    pub flags: UInt32,
    pub fileType: OSType,
    pub fileCreator: OSType,
    pub rotationAngle: Fixed,
    pub fileFSRef: *mut FSRef,
}
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct ICAUploadFilePB {
    pub header: ICAHeader,
    pub parentObject: ICAObject,
    pub fileFSRef: *mut FSRef,
    pub flags: UInt32,
}
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct ICALoadDeviceModulePB {
    pub header: ICAHeader,
    pub paramDictionary: CFDictionaryRef,
}
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct ICAUnloadDeviceModulePB {
    pub header: ICAHeader,
    pub deviceObject: ICAObject,
}
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct ICAOpenSessionPB {
    pub header: ICAHeader,
    pub deviceObject: ICAObject,
    pub sessionID: ICASessionID,
}
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct ICACloseSessionPB {
    pub header: ICAHeader,
    pub sessionID: ICASessionID,
}
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct ICAScannerOpenSessionPB {
    pub header: ICAHeader,
    pub object: ICAObject,
    pub sessionID: ICAScannerSessionID,
}
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct ICAScannerCloseSessionPB {
    pub header: ICAHeader,
    pub sessionID: ICAScannerSessionID,
}
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct ICAScannerInitializePB {
    pub header: ICAHeader,
    pub sessionID: ICAScannerSessionID,
}
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct ICAScannerGetParametersPB {
    pub header: ICAHeader,
    pub sessionID: ICAScannerSessionID,
    pub theDict: CFMutableDictionaryRef,
}
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct ICAScannerSetParametersPB {
    pub header: ICAHeader,
    pub sessionID: ICAScannerSessionID,
    pub theDict: CFMutableDictionaryRef,
}
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct ICAScannerStatusPB {
    pub header: ICAHeader,
    pub sessionID: ICAScannerSessionID,
    pub status: UInt32,
}
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct ICAScannerStartPB {
    pub header: ICAHeader,
    pub sessionID: ICAScannerSessionID,
}
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct ICDHeader {
    pub err: ICAError,
    pub refcon: ::std::os::raw::c_ulong,
}
pub type ICDCompletion = ::std::option::Option<unsafe extern "C" fn(pb: *mut ICDHeader)>;
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct ICD_NewObjectPB {
    pub header: ICDHeader,
    pub parentObject: ICAObject,
    pub objectInfo: ICAObjectInfo,
    pub object: ICAObject,
}
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct ICD_DisposeObjectPB {
    pub header: ICDHeader,
    pub object: ICAObject,
}
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct ObjectInfo {
    pub icaObject: ICAObject,
    pub reserved: ::std::os::raw::c_ulong,
    pub icaObjectInfo: ICAObjectInfo,
    pub uniqueID: UInt32,
    pub thumbnailSize: UInt32,
    pub dataSize: UInt32,
    pub dataWidth: UInt32,
    pub dataHeight: UInt32,
    pub name: [UInt8; 32usize],
    pub creationDate: [UInt8; 20usize],
    pub flags: UInt32,
    pub privateData: Ptr,
    pub uniqueIDFireWire: UInt64,
    pub tag: UInt32,
    pub dataSize64: UInt64,
}
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct ICD_ObjectSendMessagePB {
    pub header: ICDHeader,
    pub object: ICAObject,
    pub objectInfo: ICAObjectInfo,
    pub connectionID: ICAConnectionID,
    pub message: ICAMessage,
    pub totalDataSize: UInt32,
    pub result: UInt32,
}
pub type __ICD_OpenUSBDevice = ::std::option::Option<
    unsafe extern "C" fn(locationID: UInt32, objectInfo: *mut ObjectInfo) -> ICAError,
>;
pub type __ICD_OpenUSBDeviceWithIORegPath = ::std::option::Option<
    unsafe extern "C" fn(
        locationID: UInt32,
        ioregPath: *mut ::std::os::raw::c_char,
        objectInfo: *mut ObjectInfo,
    ) -> ICAError,
>;
pub type __ICD_OpenFireWireDevice = ::std::option::Option<
    unsafe extern "C" fn(guid: UInt64, objectInfo: *mut ObjectInfo) -> ICAError,
>;
pub type __ICD_OpenFireWireDeviceWithIORegPath = ::std::option::Option<
    unsafe extern "C" fn(
        guid: UInt64,
        ioregPath: *mut ::std::os::raw::c_char,
        objectInfo: *mut ObjectInfo,
    ) -> ICAError,
>;
pub type __ICD_OpenBluetoothDevice = ::std::option::Option<
    unsafe extern "C" fn(params: CFDictionaryRef, objectInfo: *mut ObjectInfo) -> ICAError,
>;
pub type __ICD_OpenTCPIPDevice = ::std::option::Option<
    unsafe extern "C" fn(params: CFDictionaryRef, objectInfo: *mut ObjectInfo) -> ICAError,
>;
pub type __ICD_OpenMassStorageDevice = ::std::option::Option<
    unsafe extern "C" fn(
        diskBSDName: CFStringRef,
        daSession: DASessionRef,
        objectInfo: *mut ObjectInfo,
    ) -> ICAError,
>;
pub type __ICD_CloseDevice =
    ::std::option::Option<unsafe extern "C" fn(objectInfo: *mut ObjectInfo) -> ICAError>;
pub type __ICD_PeriodicTask =
    ::std::option::Option<unsafe extern "C" fn(objectInfo: *mut ObjectInfo) -> ICAError>;
pub type __ICD_GetObjectInfo = ::std::option::Option<
    unsafe extern "C" fn(
        parentInfo: *const ObjectInfo,
        index: UInt32,
        newInfo: *mut ObjectInfo,
    ) -> ICAError,
>;
pub type __ICD_Cleanup =
    ::std::option::Option<unsafe extern "C" fn(objectInfo: *mut ObjectInfo) -> ICAError>;
pub type __ICD_GetPropertyData = ::std::option::Option<
    unsafe extern "C" fn(
        objectInfo: *const ObjectInfo,
        pb: *mut ::std::os::raw::c_void,
    ) -> ICAError,
>;
pub type __ICD_SetPropertyData = ::std::option::Option<
    unsafe extern "C" fn(
        objectInfo: *const ObjectInfo,
        pb: *const ::std::os::raw::c_void,
    ) -> ICAError,
>;
pub type __ICD_ReadFileData = ::std::option::Option<
    unsafe extern "C" fn(
        objectInfo: *const ObjectInfo,
        dataType: UInt32,
        buffer: Ptr,
        offset: UInt32,
        length: *mut UInt32,
    ) -> ICAError,
>;
pub type __ICD_WriteFileData = ::std::option::Option<
    unsafe extern "C" fn(
        parentInfo: *const ObjectInfo,
        filename: *const ::std::os::raw::c_char,
        dataType: UInt32,
        buffer: Ptr,
        offset: UInt32,
        length: *mut UInt32,
    ) -> ICAError,
>;
pub type __ICD_SendMessage = ::std::option::Option<
    unsafe extern "C" fn(
        objectInfo: *const ObjectInfo,
        pb: *mut ICD_ObjectSendMessagePB,
        completion: ICDCompletion,
    ) -> ICAError,
>;
pub type __ICD_AddPropertiesToCFDictionary = ::std::option::Option<
    unsafe extern "C" fn(objectInfo: *mut ObjectInfo, dict: CFMutableDictionaryRef) -> ICAError,
>;
pub type __ICD_WriteDataToFile = ::std::option::Option<
    unsafe extern "C" fn(
        objectInfo: *const ObjectInfo,
        file: *mut FILE,
        offset: UInt32,
        length: *mut ::std::os::raw::c_long,
    ) -> ICAError,
>;
pub type __ICD_WriteDataToFileDescriptor = ::std::option::Option<
    unsafe extern "C" fn(
        objectInfo: *const ObjectInfo,
        fd: ::std::os::raw::c_int,
        offset: UInt32,
        length: *mut ::std::os::raw::c_long,
    ) -> ICAError,
>;
pub type __ICD_WriteDataToFileDescriptor64 = ::std::option::Option<
    unsafe extern "C" fn(objectInfo: *const ObjectInfo, fd: ::std::os::raw::c_int) -> ICAError,
>;
pub type __ICD_ReadFileData64 = ::std::option::Option<
    unsafe extern "C" fn(
        objectInfo: *const ObjectInfo,
        dataType: UInt32,
        buffer: Ptr,
        offset: UInt64,
        length: *mut UInt64,
    ) -> ICAError,
>;
pub type ICDNewObjectCreatedCompletion =
    ::std::option::Option<unsafe extern "C" fn(info: *const ObjectInfo)>;
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct ICD_callback_functions {
    pub f_ICD_OpenUSBDevice: __ICD_OpenUSBDevice,
    pub f_ICD_CloseDevice: __ICD_CloseDevice,
    pub f_ICD_PeriodicTask: __ICD_PeriodicTask,
    pub f_ICD_GetObjectInfo: __ICD_GetObjectInfo,
    pub f_ICD_Cleanup: __ICD_Cleanup,
    pub f_ICD_GetPropertyData: __ICD_GetPropertyData,
    pub f_ICD_SetPropertyData: __ICD_SetPropertyData,
    pub f_ICD_ReadFileData: __ICD_ReadFileData,
    pub f_ICD_WriteFileData: __ICD_WriteFileData,
    pub f_ICD_SendMessage: __ICD_SendMessage,
    pub f_ICD_AddPropertiesToCFDictionary: __ICD_AddPropertiesToCFDictionary,
    pub f_ICD_OpenFireWireDevice: __ICD_OpenFireWireDevice,
    pub f_ICD_OpenUSBDeviceWithIORegPath: __ICD_OpenUSBDeviceWithIORegPath,
    pub f_ICD_OpenFireWireDeviceWithIORegPath: __ICD_OpenFireWireDeviceWithIORegPath,
    pub f_ICD_OpenBluetoothDevice: __ICD_OpenBluetoothDevice,
    pub f_ICD_OpenTCPIPDevice: __ICD_OpenTCPIPDevice,
    pub f_ICD_WriteDataToFile: __ICD_WriteDataToFile,
    pub f_ICD_OpenMassStorageDevice: __ICD_OpenMassStorageDevice,
    pub f_ICD_WriteDataToFileDescriptor: __ICD_WriteDataToFileDescriptor,
    pub f_ICD_WriteDataToFileDescriptor64: __ICD_WriteDataToFileDescriptor64,
    pub f_ICD_ReadFileData64: __ICD_ReadFileData64,
}
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct ScannerObjectInfo {
    pub icaObject: ICAObject,
    pub reserved: ::std::os::raw::c_ulong,
    pub icaObjectInfo: ICAObjectInfo,
    pub uniqueID: UInt32,
    pub uniqueIDFireWire: UInt64,
    pub thumbnailSize: UInt32,
    pub dataSize: UInt32,
    pub dataWidth: UInt32,
    pub dataHeight: UInt32,
    pub name: [UInt8; 32usize],
    pub creationDate: [UInt8; 20usize],
    pub flags: UInt32,
    pub privateData: Ptr,
    pub tag: UInt32,
}
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct ICD_ScannerObjectSendMessagePB {
    pub header: ICDHeader,
    pub object: ICAObject,
    pub objectInfo: ICAObjectInfo,
    pub connectionID: ICAConnectionID,
    pub message: ICAMessage,
    pub totalDataSize: UInt32,
    pub result: UInt32,
}
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct ICD_ScannerOpenSessionPB {
    pub header: ICDHeader,
    pub object: ICAObject,
    pub objectInfo: ICAObjectInfo,
    pub connectionID: ICAConnectionID,
    pub sessionID: ICAScannerSessionID,
}
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct ICD_ScannerCloseSessionPB {
    pub header: ICDHeader,
    pub object: ICAObject,
    pub objectInfo: ICAObjectInfo,
    pub connectionID: ICAConnectionID,
    pub sessionID: ICAScannerSessionID,
}
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct ICD_ScannerInitializePB {
    pub header: ICDHeader,
    pub object: ICAObject,
    pub objectInfo: ICAObjectInfo,
    pub connectionID: ICAConnectionID,
    pub sessionID: ICAScannerSessionID,
}
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct ICD_ScannerGetParametersPB {
    pub header: ICDHeader,
    pub object: ICAObject,
    pub objectInfo: ICAObjectInfo,
    pub connectionID: ICAConnectionID,
    pub sessionID: ICAScannerSessionID,
    pub theDict: CFMutableDictionaryRef,
}
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct ICD_ScannerSetParametersPB {
    pub header: ICDHeader,
    pub object: ICAObject,
    pub objectInfo: ICAObjectInfo,
    pub connectionID: ICAConnectionID,
    pub sessionID: ICAScannerSessionID,
    pub theDict: CFMutableDictionaryRef,
}
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct ICD_ScannerStatusPB {
    pub header: ICDHeader,
    pub object: ICAObject,
    pub objectInfo: ICAObjectInfo,
    pub connectionID: ICAConnectionID,
    pub sessionID: ICAScannerSessionID,
    pub status: UInt32,
}
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct ICD_ScannerStartPB {
    pub header: ICDHeader,
    pub object: ICAObject,
    pub objectInfo: ICAObjectInfo,
    pub connectionID: ICAConnectionID,
    pub sessionID: ICAScannerSessionID,
}
pub type __ICD_ScannerOpenUSBDevice = ::std::option::Option<
    unsafe extern "C" fn(locationID: UInt32, objectInfo: *mut ScannerObjectInfo) -> ICAError,
>;
pub type __ICD_ScannerOpenUSBDeviceWithIORegPath = ::std::option::Option<
    unsafe extern "C" fn(
        locationID: UInt32,
        ioregPath: *mut ::std::os::raw::c_char,
        objectInfo: *mut ScannerObjectInfo,
    ) -> ICAError,
>;
pub type __ICD_ScannerOpenFireWireDevice = ::std::option::Option<
    unsafe extern "C" fn(guid: UInt64, objectInfo: *mut ScannerObjectInfo) -> ICAError,
>;
pub type __ICD_ScannerOpenFireWireDeviceWithIORegPath = ::std::option::Option<
    unsafe extern "C" fn(
        guid: UInt64,
        ioregPath: *mut ::std::os::raw::c_char,
        objectInfo: *mut ScannerObjectInfo,
    ) -> ICAError,
>;
pub type __ICD_ScannerOpenBluetoothDevice = ::std::option::Option<
    unsafe extern "C" fn(params: CFDictionaryRef, objectInfo: *mut ScannerObjectInfo) -> ICAError,
>;
pub type __ICD_ScannerOpenTCPIPDevice = ::std::option::Option<
    unsafe extern "C" fn(params: CFDictionaryRef, objectInfo: *mut ScannerObjectInfo) -> ICAError,
>;
pub type __ICD_ScannerOpenMassStorageDevice = ::std::option::Option<
    unsafe extern "C" fn(
        diskBSDName: CFStringRef,
        daSession: DASessionRef,
        objectInfo: *mut ScannerObjectInfo,
    ) -> ICAError,
>;
pub type __ICD_ScannerCloseDevice =
    ::std::option::Option<unsafe extern "C" fn(objectInfo: *mut ScannerObjectInfo) -> ICAError>;
pub type __ICD_ScannerPeriodicTask =
    ::std::option::Option<unsafe extern "C" fn(objectInfo: *mut ScannerObjectInfo) -> ICAError>;
pub type __ICD_ScannerGetObjectInfo = ::std::option::Option<
    unsafe extern "C" fn(
        parentInfo: *const ScannerObjectInfo,
        index: UInt32,
        newInfo: *mut ScannerObjectInfo,
    ) -> ICAError,
>;
pub type __ICD_ScannerCleanup =
    ::std::option::Option<unsafe extern "C" fn(objectInfo: *mut ScannerObjectInfo) -> ICAError>;
pub type __ICD_ScannerGetPropertyData = ::std::option::Option<
    unsafe extern "C" fn(
        objectInfo: *const ScannerObjectInfo,
        pb: *mut ::std::os::raw::c_void,
    ) -> ICAError,
>;
pub type __ICD_ScannerSetPropertyData = ::std::option::Option<
    unsafe extern "C" fn(
        objectInfo: *const ScannerObjectInfo,
        pb: *const ::std::os::raw::c_void,
    ) -> ICAError,
>;
pub type __ICD_ScannerReadFileData = ::std::option::Option<
    unsafe extern "C" fn(
        objectInfo: *const ScannerObjectInfo,
        dataType: UInt32,
        buffer: Ptr,
        offset: UInt32,
        length: *mut UInt32,
    ) -> ICAError,
>;
pub type __ICD_ScannerWriteFileData = ::std::option::Option<
    unsafe extern "C" fn(
        parentInfo: *const ScannerObjectInfo,
        filename: *const ::std::os::raw::c_char,
        dataType: UInt32,
        buffer: Ptr,
        offset: UInt32,
        length: *mut UInt32,
    ) -> ICAError,
>;
pub type __ICD_ScannerSendMessage = ::std::option::Option<
    unsafe extern "C" fn(
        objectInfo: *const ScannerObjectInfo,
        pb: *mut ICD_ScannerObjectSendMessagePB,
        completion: ICDCompletion,
    ) -> ICAError,
>;
pub type __ICD_ScannerAddPropertiesToCFDictionary = ::std::option::Option<
    unsafe extern "C" fn(
        objectInfo: *mut ScannerObjectInfo,
        dict: CFMutableDictionaryRef,
    ) -> ICAError,
>;
pub type __ICD_ScannerOpenSession = ::std::option::Option<
    unsafe extern "C" fn(
        deviceObjectInfo: *const ScannerObjectInfo,
        pb: *mut ICD_ScannerOpenSessionPB,
    ) -> ICAError,
>;
pub type __ICD_ScannerCloseSession = ::std::option::Option<
    unsafe extern "C" fn(
        deviceObjectInfo: *const ScannerObjectInfo,
        pb: *mut ICD_ScannerCloseSessionPB,
    ) -> ICAError,
>;
pub type __ICD_ScannerInitialize = ::std::option::Option<
    unsafe extern "C" fn(
        deviceObjectInfo: *const ScannerObjectInfo,
        pb: *mut ICD_ScannerInitializePB,
    ) -> ICAError,
>;
pub type __ICD_ScannerGetParameters = ::std::option::Option<
    unsafe extern "C" fn(
        deviceObjectInfo: *const ScannerObjectInfo,
        pb: *mut ICD_ScannerGetParametersPB,
    ) -> ICAError,
>;
pub type __ICD_ScannerSetParameters = ::std::option::Option<
    unsafe extern "C" fn(
        deviceObjectInfo: *const ScannerObjectInfo,
        pb: *mut ICD_ScannerSetParametersPB,
    ) -> ICAError,
>;
pub type __ICD_ScannerStatus = ::std::option::Option<
    unsafe extern "C" fn(
        deviceObjectInfo: *const ScannerObjectInfo,
        pb: *mut ICD_ScannerStatusPB,
    ) -> ICAError,
>;
pub type __ICD_ScannerStart = ::std::option::Option<
    unsafe extern "C" fn(
        deviceObjectInfo: *const ScannerObjectInfo,
        pb: *mut ICD_ScannerStartPB,
    ) -> ICAError,
>;
pub type __ICD_ScannerWriteDataToFile = ::std::option::Option<
    unsafe extern "C" fn(
        objectInfo: *const ScannerObjectInfo,
        file: *mut FILE,
        offset: UInt32,
        length: *mut ::std::os::raw::c_long,
    ) -> ICAError,
>;
pub type __ICD_ScannerWriteDataToFileDescriptor = ::std::option::Option<
    unsafe extern "C" fn(
        objectInfo: *const ScannerObjectInfo,
        fd: ::std::os::raw::c_int,
        offset: UInt32,
        length: *mut ::std::os::raw::c_long,
    ) -> ICAError,
>;
pub type __ICD_ScannerWriteDataToFileDescriptor64 = ::std::option::Option<
    unsafe extern "C" fn(
        objectInfo: *const ScannerObjectInfo,
        fd: ::std::os::raw::c_int,
    ) -> ICAError,
>;
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct ICD_Scannerscanner_callback_functions {
    pub f_ICD_ScannerOpenUSBDevice: __ICD_ScannerOpenUSBDevice,
    pub f_ICD_ScannerOpenUSBDeviceWithIORegPath: __ICD_ScannerOpenUSBDeviceWithIORegPath,
    pub f_ICD_ScannerCloseDevice: __ICD_ScannerCloseDevice,
    pub f_ICD_ScannerPeriodicTask: __ICD_ScannerPeriodicTask,
    pub f_ICD_ScannerGetObjectInfo: __ICD_ScannerGetObjectInfo,
    pub f_ICD_ScannerCleanup: __ICD_ScannerCleanup,
    pub f_ICD_ScannerGetPropertyData: __ICD_ScannerGetPropertyData,
    pub f_ICD_ScannerSetPropertyData: __ICD_ScannerSetPropertyData,
    pub f_ICD_ScannerReadFileData: __ICD_ScannerReadFileData,
    pub f_ICD_ScannerWriteFileData: __ICD_ScannerWriteFileData,
    pub f_ICD_ScannerSendMessage: __ICD_ScannerSendMessage,
    pub f_ICD_ScannerAddPropertiesToCFDictionary: __ICD_ScannerAddPropertiesToCFDictionary,
    pub f_ICD_ScannerOpenFireWireDevice: __ICD_ScannerOpenFireWireDevice,
    pub f_ICD_ScannerOpenFireWireDeviceWithIORegPath: __ICD_ScannerOpenFireWireDeviceWithIORegPath,
    pub f_ICD_ScannerOpenSession: __ICD_ScannerOpenSession,
    pub f_ICD_ScannerCloseSession: __ICD_ScannerCloseSession,
    pub f_ICD_ScannerInitialize: __ICD_ScannerInitialize,
    pub f_ICD_ScannerGetParameters: __ICD_ScannerGetParameters,
    pub f_ICD_ScannerSetParameters: __ICD_ScannerSetParameters,
    pub f_ICD_ScannerStatus: __ICD_ScannerStatus,
    pub f_ICD_ScannerStart: __ICD_ScannerStart,
    pub f_ICD_ScannerOpenBluetoothDevice: __ICD_ScannerOpenBluetoothDevice,
    pub f_ICD_ScannerOpenTCPIPDevice: __ICD_ScannerOpenTCPIPDevice,
    pub f_ICD_ScannerWriteDataToFile: __ICD_ScannerWriteDataToFile,
    pub f_ICD_ScannerOpenMassStorageDevice: __ICD_ScannerOpenMassStorageDevice,
    pub f_ICD_ScannerWriteDataToFileDescriptor: __ICD_ScannerWriteDataToFileDescriptor,
    pub f_ICD_ScannerWriteDataToFileDescriptor64: __ICD_ScannerWriteDataToFileDescriptor64,
}
pub type ICD_scanner_callback_functions = ICD_Scannerscanner_callback_functions;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ICARawFileHeader {
    pub imageDataOffset: UInt32,
    pub version: UInt32,
    pub imageWidth: UInt32,
    pub imageHeight: UInt32,
    pub bytesPerRow: UInt32,
    pub numberOfComponents: UInt32,
    pub bitsPerComponent: UInt32,
    pub bitsPerPixel: UInt32,
    pub cgColorSpaceModel: UInt32,
    pub bitmapInfo: UInt32,
    pub orientation: UInt32,
    pub dpi: UInt32,
    pub colorSyncModeStr: [::std::os::raw::c_char; 64usize],
}
unsafe extern "C" {
    pub static kICAUSBVendorIDKey: CFStringRef;
}
unsafe extern "C" {
    pub static kICAUSBProductIDKey: CFStringRef;
}
unsafe extern "C" {
    pub static kICADeviceTypeKey: CFStringRef;
}
unsafe extern "C" {
    pub static kICAExecutableArchitectureKey: CFStringRef;
}
unsafe extern "C" {
    pub static kICARemoteDeviceKey: CFStringRef;
}
unsafe extern "C" {
    pub static kICADeviceSharedKey: CFStringRef;
}
unsafe extern "C" {
    pub static kICADeviceWebSharedKey: CFStringRef;
}
unsafe extern "C" {
    pub static kICADeviceUsedKey: CFStringRef;
}
unsafe extern "C" {
    pub static kICADeviceCapabilitiesKey: CFStringRef;
}
unsafe extern "C" {
    pub static kICALockStatusKey: CFStringRef;
}
unsafe extern "C" {
    pub static kICADataPropertyKey: CFStringRef;
}
unsafe extern "C" {
    pub static kICADataTypeKey: CFStringRef;
}
unsafe extern "C" {
    pub static kICADataSizeKey: CFStringRef;
}
unsafe extern "C" {
    pub static kICAThumbnailPropertyKey: CFStringRef;
}
unsafe extern "C" {
    pub static kICAThumbnailSizeKey: CFStringRef;
}
unsafe extern "C" {
    pub static kICARawKey: CFStringRef;
}
unsafe extern "C" {
    pub static kICACreationDateStringKey: CFStringRef;
}
unsafe extern "C" {
    pub static kICAModificationDateStringKey: CFStringRef;
}
unsafe extern "C" {
    pub static kICAMediaDurationInSecondsKey: CFStringRef;
}
unsafe extern "C" {
    pub static kICADeviceTypeCamera: CFStringRef;
}
unsafe extern "C" {
    pub static kICADeviceTypeScanner: CFStringRef;
}
unsafe extern "C" {
    pub static kICAUSBTransportType: CFStringRef;
}
unsafe extern "C" {
    pub static kICAFireWireTransportType: CFStringRef;
}
unsafe extern "C" {
    pub static kICABluetoothTransportType: CFStringRef;
}
unsafe extern "C" {
    pub static kICATCPIPTransportType: CFStringRef;
}
unsafe extern "C" {
    pub static kICASCSITransportType: CFStringRef;
}
unsafe extern "C" {
    pub static kICATWAINTransportType: CFStringRef;
}
unsafe extern "C" {
    pub static kICADeviceBrowserDeviceRefKey: CFStringRef;
}
unsafe extern "C" {
    pub static kICADeviceModulePathKey: CFStringRef;
}
unsafe extern "C" {
    pub static kICATransportTypeKey: CFStringRef;
}
unsafe extern "C" {
    pub static kICABluetoothAddressKey: CFStringRef;
}
unsafe extern "C" {
    pub static kICAIPNameKey: CFStringRef;
}
unsafe extern "C" {
    pub static kICAIPGUIDKey: CFStringRef;
}
unsafe extern "C" {
    pub static kICATWAINDSPathKey: CFStringRef;
}
unsafe extern "C" {
    pub fn ICAImportImage(pb: *mut ICAImportImagePB, completion: ICACompletion) -> ICAError;
}
unsafe extern "C" {
    pub fn ICAShowDeviceBrowser(options: CFDictionaryRef) -> ICAError;
}
unsafe extern "C" {
    pub static kICANotificationTypeObjectInfoChanged: CFStringRef;
}
unsafe extern "C" {
    pub static kICANotificationTypeStoreAdded: CFStringRef;
}
unsafe extern "C" {
    pub static kICANotificationTypeStoreRemoved: CFStringRef;
}
unsafe extern "C" {
    pub static kICANotificationTypeStoreFull: CFStringRef;
}
unsafe extern "C" {
    pub static kICANotificationTypeStoreInfoChanged: CFStringRef;
}
unsafe extern "C" {
    pub static kICANotificationTypeDeviceAdded: CFStringRef;
}
unsafe extern "C" {
    pub static kICANotificationTypeDeviceInfoChanged: CFStringRef;
}
unsafe extern "C" {
    pub static kICANotificationTypeDevicePropertyChanged: CFStringRef;
}
unsafe extern "C" {
    pub static kICANotificationTypeDeviceWasReset: CFStringRef;
}
unsafe extern "C" {
    pub static kICANotificationTypeCaptureComplete: CFStringRef;
}
unsafe extern "C" {
    pub static kICANotificationTypeRequestObjectTransfer: CFStringRef;
}
unsafe extern "C" {
    pub static kICANotificationTypeUnreportedStatus: CFStringRef;
}
unsafe extern "C" {
    pub static kICANotificationTypeProprietary: CFStringRef;
}
unsafe extern "C" {
    pub static kICANotificationTypeDeviceConnectionProgress: CFStringRef;
}
unsafe extern "C" {
    pub static kICANotificationTypeDownloadProgressStatus: CFStringRef;
}
unsafe extern "C" {
    pub static kICARefconKey: CFStringRef;
}
unsafe extern "C" {
    pub static kICANotificationDeviceICAObjectKey: CFStringRef;
}
unsafe extern "C" {
    pub static kICANotificationDeviceListICAObjectKey: CFStringRef;
}
unsafe extern "C" {
    pub static kICANotificationClassKey: CFStringRef;
}
unsafe extern "C" {
    pub static kICANotificationRawEventKey: CFStringRef;
}
unsafe extern "C" {
    pub static kICANotificationDataSizeKey: CFStringRef;
}
unsafe extern "C" {
    pub static kICANotificationDataCookieKey: CFStringRef;
}
unsafe extern "C" {
    pub static kICANotificationImageKey: CFStringRef;
}
unsafe extern "C" {
    pub static kICANotificationImageWidthKey: CFStringRef;
}
unsafe extern "C" {
    pub static kICANotificationImageHeightKey: CFStringRef;
}
unsafe extern "C" {
    pub static kICANotificationImageBytesPerRowKey: CFStringRef;
}
unsafe extern "C" {
    pub static kICANotificationImageStartRowKey: CFStringRef;
}
unsafe extern "C" {
    pub static kICANotificationImageNumberOfRowsKey: CFStringRef;
}
unsafe extern "C" {
    pub static kICANotificationImageDataKey: CFStringRef;
}
unsafe extern "C" {
    pub static kICANotificationImageDataSizeKey: CFStringRef;
}
unsafe extern "C" {
    pub static kICANotificationDataIsBigEndianKey: CFStringRef;
}
unsafe extern "C" {
    pub static kICANotificationNumerOfImagesRemainingKey: CFStringRef;
}
unsafe extern "C" {
    pub static kICANotificationPercentDownloadedKey: CFStringRef;
}
unsafe extern "C" {
    pub static kICANotificationVendorErrorCodeKey: CFStringRef;
}
unsafe extern "C" {
    pub static kICANotificationClassPTPStandard: CFStringRef;
}
unsafe extern "C" {
    pub static kICANotificationClassPTPVendor: CFStringRef;
}
unsafe extern "C" {
    pub static kICANotificationClassProprietary: CFStringRef;
}
unsafe extern "C" {
    pub static kICADevicePropUndefined: CFStringRef;
}
unsafe extern "C" {
    pub static kICADevicePropBatteryLevel: CFStringRef;
}
unsafe extern "C" {
    pub static kICADevicePropFunctionalMode: CFStringRef;
}
unsafe extern "C" {
    pub static kICADevicePropImageSize: CFStringRef;
}
unsafe extern "C" {
    pub static kICADevicePropCompressionSetting: CFStringRef;
}
unsafe extern "C" {
    pub static kICADevicePropWhiteBalance: CFStringRef;
}
unsafe extern "C" {
    pub static kICADevicePropRGBGain: CFStringRef;
}
unsafe extern "C" {
    pub static kICADevicePropFNumber: CFStringRef;
}
unsafe extern "C" {
    pub static kICADevicePropFocalLength: CFStringRef;
}
unsafe extern "C" {
    pub static kICADevicePropFocusDistance: CFStringRef;
}
unsafe extern "C" {
    pub static kICADevicePropFocusMode: CFStringRef;
}
unsafe extern "C" {
    pub static kICADevicePropExposureMeteringMode: CFStringRef;
}
unsafe extern "C" {
    pub static kICADevicePropFlashMode: CFStringRef;
}
unsafe extern "C" {
    pub static kICADevicePropExposureTime: CFStringRef;
}
unsafe extern "C" {
    pub static kICADevicePropExposureProgramMode: CFStringRef;
}
unsafe extern "C" {
    pub static kICADevicePropExposureIndex: CFStringRef;
}
unsafe extern "C" {
    pub static kICADevicePropExposureBiasCompensation: CFStringRef;
}
unsafe extern "C" {
    pub static kICADevicePropDateTime: CFStringRef;
}
unsafe extern "C" {
    pub static kICADevicePropCaptureDelay: CFStringRef;
}
unsafe extern "C" {
    pub static kICADevicePropStillCaptureMode: CFStringRef;
}
unsafe extern "C" {
    pub static kICADevicePropContrast: CFStringRef;
}
unsafe extern "C" {
    pub static kICADevicePropSharpness: CFStringRef;
}
unsafe extern "C" {
    pub static kICADevicePropDigitalZoom: CFStringRef;
}
unsafe extern "C" {
    pub static kICADevicePropEffectMode: CFStringRef;
}
unsafe extern "C" {
    pub static kICADevicePropBurstNumber: CFStringRef;
}
unsafe extern "C" {
    pub static kICADevicePropBurstInterval: CFStringRef;
}
unsafe extern "C" {
    pub static kICADevicePropTimelapseNumber: CFStringRef;
}
unsafe extern "C" {
    pub static kICADevicePropTimelapseInterval: CFStringRef;
}
unsafe extern "C" {
    pub static kICADevicePropFocusMeteringMode: CFStringRef;
}
unsafe extern "C" {
    pub static kICADevicePropUploadURL: CFStringRef;
}
unsafe extern "C" {
    pub static kICADevicePropArtist: CFStringRef;
}
unsafe extern "C" {
    pub static kICADevicePropCopyrightInfo: CFStringRef;
}
unsafe extern "C" {
    pub fn ICARegisterForEventNotification(
        params: *mut ICARegisterForEventNotificationPB,
        completionProc: ICACompletion,
    ) -> ICAError;
}
unsafe extern "C" {
    pub fn ICASendNotification(pb: *mut ICASendNotificationPB) -> ICAError;
}
unsafe extern "C" {
    pub fn ICASendNotificationAndWaitForReply(pb: *mut ICASendNotificationPB) -> ICAError;
}
unsafe extern "C" {
    pub fn ICACopyObjectData(
        params: *mut ICACopyObjectDataPB,
        completionProc: ICACompletion,
    ) -> ICAError;
}
unsafe extern "C" {
    pub fn ICAObjectSendMessage(
        pb: *mut ICAObjectSendMessagePB,
        completion: ICACompletion,
    ) -> ICAError;
}
unsafe extern "C" {
    pub fn ICAUploadFile(pb: *mut ICAUploadFilePB, completion: ICACompletion) -> ICAError;
}
unsafe extern "C" {
    pub fn ICALoadDeviceModule(
        pb: *mut ICALoadDeviceModulePB,
        completion: ICACompletion,
    ) -> ICAError;
}
unsafe extern "C" {
    pub fn ICAUnloadDeviceModule(
        pb: *mut ICAUnloadDeviceModulePB,
        completion: ICACompletion,
    ) -> ICAError;
}
unsafe extern "C" {
    pub fn ICAOpenSession(pb: *mut ICAOpenSessionPB, completion: ICACompletion) -> ICAError;
}
unsafe extern "C" {
    pub fn ICACloseSession(pb: *mut ICACloseSessionPB, completion: ICACompletion) -> ICAError;
}
unsafe extern "C" {
    pub fn ICAScannerOpenSession(
        pb: *mut ICAScannerOpenSessionPB,
        completion: ICACompletion,
    ) -> ICAError;
}
unsafe extern "C" {
    pub fn ICAScannerCloseSession(
        pb: *mut ICAScannerCloseSessionPB,
        completion: ICACompletion,
    ) -> ICAError;
}
unsafe extern "C" {
    pub fn ICAScannerInitialize(
        pb: *mut ICAScannerInitializePB,
        completion: ICACompletion,
    ) -> ICAError;
}
unsafe extern "C" {
    pub fn ICAScannerGetParameters(
        pb: *mut ICAScannerGetParametersPB,
        completion: ICACompletion,
    ) -> ICAError;
}
unsafe extern "C" {
    pub fn ICAScannerSetParameters(
        pb: *mut ICAScannerSetParametersPB,
        completion: ICACompletion,
    ) -> ICAError;
}
unsafe extern "C" {
    pub fn ICAScannerStatus(pb: *mut ICAScannerStatusPB, completion: ICACompletion) -> ICAError;
}
unsafe extern "C" {
    pub fn ICAScannerStart(pb: *mut ICAScannerStartPB, completion: ICACompletion) -> ICAError;
}
unsafe extern "C" {
    pub fn ICDNewObject(pb: *mut ICD_NewObjectPB, completion: ICDCompletion) -> ICAError;
}
unsafe extern "C" {
    pub fn ICDDisposeObject(pb: *mut ICD_DisposeObjectPB, completion: ICDCompletion) -> ICAError;
}
unsafe extern "C" {
    pub fn ICD_main(
        argc: ::std::os::raw::c_int,
        argv: *mut *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn ICDGetStandardPropertyData(
        objectInfo: *const ObjectInfo,
        pb: *mut ::std::os::raw::c_void,
    ) -> ICAError;
}
unsafe extern "C" {
    pub fn ICDNewObjectInfoCreated(
        parentInfo: *const ObjectInfo,
        index: UInt32,
        newICAObject: *mut ICAObject,
    ) -> ICAError;
}
unsafe extern "C" {
    pub fn ICDNewObjectCreated(
        parentInfo: *const ObjectInfo,
        objectInfo: *const ObjectInfo,
        completion: ICDNewObjectCreatedCompletion,
    ) -> ICAError;
}
unsafe extern "C" {
    pub fn ICDCopyDeviceInfoDictionary(
        deviceName: *const ::std::os::raw::c_char,
        theDict: *mut CFDictionaryRef,
    ) -> ICAError;
}
unsafe extern "C" {
    pub fn ICDCreateICAThumbnailFromICNS(
        fileName: *const ::std::os::raw::c_char,
        thumbnail: *mut ::std::os::raw::c_void,
    ) -> ICAError;
}
unsafe extern "C" {
    pub fn ICDCreateICAThumbnailFromIconRef(
        iconRef: IconRef,
        thumbnail: *mut ::std::os::raw::c_void,
    ) -> ICAError;
}
unsafe extern "C" {
    pub fn ICDInitiateNotificationCallback(pb: *const ::std::os::raw::c_void) -> ICAError;
}
unsafe extern "C" {
    pub fn ICDCreateEventDataCookie(object: ICAObject, cookie: *mut ICAEventDataCookie)
        -> ICAError;
}
unsafe extern "C" {
    pub fn ICDConnectUSBDevice(locationID: UInt32) -> ICAError;
}
unsafe extern "C" {
    pub fn ICDConnectUSBDeviceWithIORegPath(
        locationID: UInt32,
        ioregPath: *mut ::std::os::raw::c_char,
    ) -> ICAError;
}
unsafe extern "C" {
    pub fn ICDDisconnectUSBDevice(locationID: UInt32) -> ICAError;
}
unsafe extern "C" {
    pub fn ICDDisconnectUSBDeviceWithIORegPath(
        locationID: UInt32,
        ioregPath: *mut ::std::os::raw::c_char,
    ) -> ICAError;
}
unsafe extern "C" {
    pub fn ICDConnectFWDevice(guid: UInt64) -> ICAError;
}
unsafe extern "C" {
    pub fn ICDConnectFWDeviceWithIORegPath(
        guid: UInt64,
        ioregPath: *mut ::std::os::raw::c_char,
    ) -> ICAError;
}
unsafe extern "C" {
    pub fn ICDDisconnectFWDevice(guid: UInt64) -> ICAError;
}
unsafe extern "C" {
    pub fn ICDDisconnectFWDeviceWithIORegPath(
        guid: UInt64,
        ioregPath: *mut ::std::os::raw::c_char,
    ) -> ICAError;
}
unsafe extern "C" {
    pub fn ICDConnectBluetoothDevice(params: CFDictionaryRef) -> ICAError;
}
unsafe extern "C" {
    pub fn ICDDisconnectBluetoothDevice(params: CFDictionaryRef) -> ICAError;
}
unsafe extern "C" {
    pub fn ICDConnectTCPIPDevice(params: CFDictionaryRef) -> ICAError;
}
unsafe extern "C" {
    pub fn ICDDisconnectTCPIPDevice(params: CFDictionaryRef) -> ICAError;
}
unsafe extern "C" {
    pub static mut gICDCallbackFunctions: ICD_callback_functions;
}
unsafe extern "C" {
    pub static mut gICDScannerCallbackFunctions: ICD_scanner_callback_functions;
}
unsafe extern "C" {
    pub fn ICD_ScannerMain(
        argc: ::std::os::raw::c_int,
        argv: *mut *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn ICDScannerGetStandardPropertyData(
        objectInfo: *const ScannerObjectInfo,
        pb: *mut ::std::os::raw::c_void,
    ) -> ICAError;
}
unsafe extern "C" {
    pub fn ICDScannerNewObjectInfoCreated(
        parentInfo: *const ScannerObjectInfo,
        index: UInt32,
        newICAObject: *mut ICAObject,
    ) -> ICAError;
}
unsafe extern "C" {
    pub fn ICDScannerCopyDeviceInfoDictionary(
        deviceName: *const ::std::os::raw::c_char,
        theDict: *mut CFDictionaryRef,
    ) -> ICAError;
}
unsafe extern "C" {
    pub fn ICDScannerCreateICAThumbnailFromICNS(
        fileName: *const ::std::os::raw::c_char,
        thumbnail: *mut ::std::os::raw::c_void,
    ) -> ICAError;
}
unsafe extern "C" {
    pub fn ICDScannerInitiateNotificationCallback(pb: *const ::std::os::raw::c_void) -> ICAError;
}
unsafe extern "C" {
    pub fn ICDScannerCreateEventDataCookie(
        object: ICAObject,
        cookie: *mut ICAEventDataCookie,
    ) -> ICAError;
}
unsafe extern "C" {
    pub fn ICDScannerConnectUSBDevice(locationID: UInt32) -> ICAError;
}
unsafe extern "C" {
    pub fn ICDScannerConnectUSBDeviceWithIORegPath(
        locationID: UInt32,
        ioregPath: *mut ::std::os::raw::c_char,
    ) -> ICAError;
}
unsafe extern "C" {
    pub fn ICDScannerDisconnectUSBDevice(locationID: UInt32) -> ICAError;
}
unsafe extern "C" {
    pub fn ICDScannerDisconnectUSBDeviceWithIORegPath(
        locationID: UInt32,
        ioregPath: *mut ::std::os::raw::c_char,
    ) -> ICAError;
}
unsafe extern "C" {
    pub fn ICDScannerConnectFWDevice(guid: UInt64) -> ICAError;
}
unsafe extern "C" {
    pub fn ICDScannerConnectFWDeviceWithIORegPath(
        guid: UInt64,
        ioregPath: *mut ::std::os::raw::c_char,
    ) -> ICAError;
}
unsafe extern "C" {
    pub fn ICDScannerDisconnectFWDevice(guid: UInt64) -> ICAError;
}
unsafe extern "C" {
    pub fn ICDScannerDisconnectFWDeviceWithIORegPath(
        guid: UInt64,
        ioregPath: *mut ::std::os::raw::c_char,
    ) -> ICAError;
}
unsafe extern "C" {
    pub fn ICDScannerConnectBluetoothDevice(params: CFDictionaryRef) -> ICAError;
}
unsafe extern "C" {
    pub fn ICDScannerDisconnectBluetoothDevice(params: CFDictionaryRef) -> ICAError;
}
unsafe extern "C" {
    pub fn ICDScannerConnectTCPIPDevice(params: CFDictionaryRef) -> ICAError;
}
unsafe extern "C" {
    pub fn ICDScannerDisconnectTCPIPDevice(params: CFDictionaryRef) -> ICAError;
}
unsafe extern "C" {
    pub static kICUTTypeRaw: CFStringRef;
}
unsafe extern "C" {
    pub fn ICDCreateColorSpace(
        bitsPerPixel: UInt32,
        samplesPerPixel: UInt32,
        icaObject: ICAObject,
        colorSyncMode: CFStringRef,
        abstractProfile: CFDataRef,
        tmpProfilePath: *mut ::std::os::raw::c_char,
    ) -> CGColorSpaceRef;
}
unsafe extern "C" {
    pub fn ICDAddImageInfoToNotificationDictionary(
        dict: CFMutableDictionaryRef,
        width: UInt32,
        height: UInt32,
        bytesPerRow: UInt32,
        dataStartRow: UInt32,
        dataNumberOfRows: UInt32,
        dataSize: UInt32,
        dataBuffer: *mut ::std::os::raw::c_void,
    ) -> ICAError;
}
unsafe extern "C" {
    pub fn ICDAddBandInfoToNotificationDictionary(
        dict: CFMutableDictionaryRef,
        width: UInt32,
        height: UInt32,
        bitsPerPixel: UInt32,
        bitsPerComponent: UInt32,
        numComponents: UInt32,
        endianness: UInt32,
        pixelDataType: UInt32,
        bytesPerRow: UInt32,
        dataStartRow: UInt32,
        dataNumberOfRows: UInt32,
        dataSize: UInt32,
        dataBuffer: *mut ::std::os::raw::c_void,
    ) -> ICAError;
}
unsafe extern "C" {
    pub fn ICDSendNotification(pb: *mut ICASendNotificationPB) -> ICAError;
}
unsafe extern "C" {
    pub fn ICDSendNotificationAndWaitForReply(pb: *mut ICASendNotificationPB) -> ICAError;
}

unsafe impl objc2::encode::RefEncode for ICAPTPPassThroughPB {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for ICAPTPPassThroughPB {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("ICAPTPPassThroughPB", &[]);
}
unsafe impl objc2::encode::RefEncode for ICAPTPEventDataset {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for ICAPTPEventDataset {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("ICAPTPEventDataset", &[]);
}
unsafe impl objc2::encode::RefEncode for ICAHeader {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for ICAHeader {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("ICAHeader", &[]);
}
unsafe impl objc2::encode::RefEncode for ICAObjectInfo {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for ICAObjectInfo {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("ICAObjectInfo", &[]);
}
unsafe impl objc2::encode::RefEncode for ICAImportImagePB {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for ICAImportImagePB {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("ICAImportImagePB", &[]);
}
unsafe impl objc2::encode::RefEncode for ICARegisterForEventNotificationPB {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for ICARegisterForEventNotificationPB {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("ICARegisterForEventNotificationPB", &[]);
}
unsafe impl objc2::encode::RefEncode for ICASendNotificationPB {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for ICASendNotificationPB {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("ICASendNotificationPB", &[]);
}
unsafe impl objc2::encode::RefEncode for ICAGetDeviceListPB {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for ICAGetDeviceListPB {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("ICAGetDeviceListPB", &[]);
}
unsafe impl objc2::encode::RefEncode for ICACopyObjectPropertyDictionaryPB {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for ICACopyObjectPropertyDictionaryPB {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("ICACopyObjectPropertyDictionaryPB", &[]);
}
unsafe impl objc2::encode::RefEncode for ICACopyObjectThumbnailPB {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for ICACopyObjectThumbnailPB {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("ICACopyObjectThumbnailPB", &[]);
}
unsafe impl objc2::encode::RefEncode for ICACopyObjectDataPB {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for ICACopyObjectDataPB {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("ICACopyObjectDataPB", &[]);
}
unsafe impl objc2::encode::RefEncode for ICAMessage {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for ICAMessage {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("ICAMessage", &[]);
}
unsafe impl objc2::encode::RefEncode for ICAObjectSendMessagePB {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for ICAObjectSendMessagePB {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("ICAObjectSendMessagePB", &[]);
}
unsafe impl objc2::encode::RefEncode for ICADownloadFilePB {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for ICADownloadFilePB {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("ICADownloadFilePB", &[]);
}
unsafe impl objc2::encode::RefEncode for ICAUploadFilePB {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for ICAUploadFilePB {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("ICAUploadFilePB", &[]);
}
unsafe impl objc2::encode::RefEncode for ICALoadDeviceModulePB {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for ICALoadDeviceModulePB {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("ICALoadDeviceModulePB", &[]);
}
unsafe impl objc2::encode::RefEncode for ICAUnloadDeviceModulePB {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for ICAUnloadDeviceModulePB {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("ICAUnloadDeviceModulePB", &[]);
}
unsafe impl objc2::encode::RefEncode for ICAOpenSessionPB {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for ICAOpenSessionPB {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("ICAOpenSessionPB", &[]);
}
unsafe impl objc2::encode::RefEncode for ICACloseSessionPB {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for ICACloseSessionPB {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("ICACloseSessionPB", &[]);
}
unsafe impl objc2::encode::RefEncode for ICAScannerOpenSessionPB {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for ICAScannerOpenSessionPB {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("ICAScannerOpenSessionPB", &[]);
}
unsafe impl objc2::encode::RefEncode for ICAScannerCloseSessionPB {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for ICAScannerCloseSessionPB {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("ICAScannerCloseSessionPB", &[]);
}
unsafe impl objc2::encode::RefEncode for ICAScannerInitializePB {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for ICAScannerInitializePB {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("ICAScannerInitializePB", &[]);
}
unsafe impl objc2::encode::RefEncode for ICAScannerGetParametersPB {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for ICAScannerGetParametersPB {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("ICAScannerGetParametersPB", &[]);
}
unsafe impl objc2::encode::RefEncode for ICAScannerSetParametersPB {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for ICAScannerSetParametersPB {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("ICAScannerSetParametersPB", &[]);
}
unsafe impl objc2::encode::RefEncode for ICAScannerStatusPB {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for ICAScannerStatusPB {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("ICAScannerStatusPB", &[]);
}
unsafe impl objc2::encode::RefEncode for ICAScannerStartPB {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for ICAScannerStartPB {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("ICAScannerStartPB", &[]);
}
unsafe impl objc2::encode::RefEncode for ICDHeader {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for ICDHeader {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("ICDHeader", &[]);
}
unsafe impl objc2::encode::RefEncode for ICD_NewObjectPB {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for ICD_NewObjectPB {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("ICD_NewObjectPB", &[]);
}
unsafe impl objc2::encode::RefEncode for ICD_DisposeObjectPB {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for ICD_DisposeObjectPB {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("ICD_DisposeObjectPB", &[]);
}
unsafe impl objc2::encode::RefEncode for ObjectInfo {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for ObjectInfo {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("ObjectInfo", &[]);
}
unsafe impl objc2::encode::RefEncode for ICD_ObjectSendMessagePB {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for ICD_ObjectSendMessagePB {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("ICD_ObjectSendMessagePB", &[]);
}
unsafe impl objc2::encode::RefEncode for ICD_callback_functions {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for ICD_callback_functions {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("ICD_callback_functions", &[]);
}
unsafe impl objc2::encode::RefEncode for ScannerObjectInfo {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for ScannerObjectInfo {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("ScannerObjectInfo", &[]);
}
unsafe impl objc2::encode::RefEncode for ICD_ScannerObjectSendMessagePB {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for ICD_ScannerObjectSendMessagePB {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("ICD_ScannerObjectSendMessagePB", &[]);
}
unsafe impl objc2::encode::RefEncode for ICD_ScannerOpenSessionPB {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for ICD_ScannerOpenSessionPB {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("ICD_ScannerOpenSessionPB", &[]);
}
unsafe impl objc2::encode::RefEncode for ICD_ScannerCloseSessionPB {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for ICD_ScannerCloseSessionPB {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("ICD_ScannerCloseSessionPB", &[]);
}
unsafe impl objc2::encode::RefEncode for ICD_ScannerInitializePB {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for ICD_ScannerInitializePB {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("ICD_ScannerInitializePB", &[]);
}
unsafe impl objc2::encode::RefEncode for ICD_ScannerGetParametersPB {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for ICD_ScannerGetParametersPB {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("ICD_ScannerGetParametersPB", &[]);
}
unsafe impl objc2::encode::RefEncode for ICD_ScannerSetParametersPB {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for ICD_ScannerSetParametersPB {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("ICD_ScannerSetParametersPB", &[]);
}
unsafe impl objc2::encode::RefEncode for ICD_ScannerStatusPB {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for ICD_ScannerStatusPB {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("ICD_ScannerStatusPB", &[]);
}
unsafe impl objc2::encode::RefEncode for ICD_ScannerStartPB {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for ICD_ScannerStartPB {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("ICD_ScannerStartPB", &[]);
}
unsafe impl objc2::encode::RefEncode for ICD_Scannerscanner_callback_functions {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for ICD_Scannerscanner_callback_functions {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("ICD_Scannerscanner_callback_functions", &[]);
}
unsafe impl objc2::encode::RefEncode for ICARawFileHeader {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for ICARawFileHeader {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("ICARawFileHeader", &[]);
}
