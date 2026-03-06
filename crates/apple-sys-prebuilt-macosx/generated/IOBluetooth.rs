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
pub type BluetoothHCIVersions = ::std::os::raw::c_uint;
pub type BluetoothLMPVersions = ::std::os::raw::c_uint;
pub type BluetoothConnectionHandle = u16;
pub type BluetoothLMPHandle = u8;
pub type BluetoothReasonCode = u8;
pub type BluetoothEncryptionEnable = u8;
pub type BluetoothKeyFlag = u8;
pub type BluetoothKeyType = u8;
pub type BluetoothPacketType = u16;
pub type BluetoothLAP = u32;
pub type BluetoothPageScanRepetitionMode = u8;
pub type BluetoothPageScanPeriodMode = u8;
pub type BluetoothPageScanMode = u8;
pub type BluetoothHCIPageScanType = u8;
pub type BluetoothHCIErroneousDataReporting = u8;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct BluetoothDeviceAddress {
    pub data: [u8; 6usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct BluetoothKey {
    pub data: [u8; 16usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct BluetoothIRK {
    pub data: [u8; 16usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct BluetoothPINCode {
    pub data: [u8; 16usize],
}
pub type BluetoothClassOfDevice = u32;
pub type BluetoothServiceClassMajor = u32;
pub type BluetoothDeviceClassMajor = u32;
pub type BluetoothDeviceClassMinor = u32;
pub type BluetoothDeviceName = [u8; 248usize];
pub type BluetoothClockOffset = u16;
pub type BluetoothRole = u8;
pub type BluetoothAllowRoleSwitch = u8;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct BluetoothSetEventMask {
    pub data: [u8; 8usize],
}
pub type BluetoothPINType = u8;
pub type BluetoothL2CAPChannelID = u16;
pub type BluetoothL2CAPGroupID = BluetoothL2CAPChannelID;
pub type BluetoothL2CAPPSM = u16;
pub type BluetoothL2CAPCommandCode = ::std::os::raw::c_uint;
pub type BluetoothL2CAPCommandRejectReason = ::std::os::raw::c_uint;
pub type BluetoothL2CAPMTU = u16;
pub type BluetoothL2CAPLinkTimeout = u16;
pub type BluetoothL2CAPFlushTimeout = u16;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct BluetoothL2CAPQualityOfServiceOptions {
    pub flags: u8,
    pub serviceType: u8,
    pub tokenRate: u32,
    pub tokenBucketSize: u32,
    pub peakBandwidth: u32,
    pub latency: u32,
    pub delayVariation: u32,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct BluetoothL2CAPRetransmissionAndFlowControlOptions {
    pub flags: u8,
    pub txWindowSize: u8,
    pub maxTransmit: u8,
    pub retransmissionTimeout: u16,
    pub monitorTimeout: u16,
    pub maxPDUPayloadSize: u16,
}
pub type BluetoothL2CAPSegmentationAndReassembly = ::std::os::raw::c_uint;
pub type BluetoothL2CAPByteCount = u16;
pub type BluetoothL2CAPCommandID = u8;
pub type BluetoothL2CAPCommandByteCount = u16;
pub type BluetoothL2CAPConnectionResult = ::std::os::raw::c_uint;
pub type BluetoothL2CAPConnectionStatus = ::std::os::raw::c_uint;
pub type BluetoothL2CAPConfigurationResult = ::std::os::raw::c_uint;
pub type BluetoothL2CAPConfigurationOption = ::std::os::raw::c_uint;
pub type BluetoothL2CAPConfigurationRetransmissionAndFlowControlFlags = ::std::os::raw::c_uint;
pub type BluetoothL2CAPInformationType = ::std::os::raw::c_uint;
pub type BluetoothL2CAPInformationResult = ::std::os::raw::c_uint;
pub type BluetoothL2CAPInformationExtendedFeaturesMask = ::std::os::raw::c_uint;
pub type BluetoothL2CAPQoSType = ::std::os::raw::c_uint;
pub type BluetoothL2CAPSupervisoryFuctionType = ::std::os::raw::c_uint;
pub type BluetoothLESecurityManagerCommandCode = ::std::os::raw::c_uint;
pub type BluetoothLESecurityManagerUserInputCapability = ::std::os::raw::c_uint;
pub type BluetoothLESecurityManagerUserOutputCapability = ::std::os::raw::c_uint;
pub type BluetoothLESecurityManagerIOCapability = ::std::os::raw::c_uint;
pub type BluetoothLESecurityManagerOOBData = ::std::os::raw::c_uint;
pub type BluetoothLESecurityManagerPairingFailedReasonCode = ::std::os::raw::c_uint;
pub type BluetoothLESecurityManagerKeypressNotificationType = ::std::os::raw::c_uint;
pub type BluetoothAMPManagerCode = ::std::os::raw::c_uint;
pub type BluetoothAMPCommandRejectReason = ::std::os::raw::c_uint;
pub type BluetoothAMPDiscoverResponseControllerStatus = ::std::os::raw::c_uint;
pub type BluetoothAMPGetInfoResponseStatus = ::std::os::raw::c_uint;
pub type BluetoothAMPGetAssocResponseStatus = ::std::os::raw::c_uint;
pub type BluetoothAMPCreatePhysicalLinkResponseStatus = ::std::os::raw::c_uint;
pub type BluetoothAMPDisconnectPhysicalLinkResponseStatus = ::std::os::raw::c_uint;
pub type BluetoothHCICommandOpCodeGroup = u8;
pub type BluetoothHCICommandOpCodeCommand = u16;
pub type BluetoothHCICommandOpCode = u16;
pub type BluetoothHCIVendorCommandSelector = u32;
pub type BluetoothHCIQoSFlags = u8;
pub type BluetoothHCIParamByteCount = u8;
pub type BluetoothHCIACLDataByteCount = u16;
pub type BluetoothHCISCODataByteCount = u8;
pub type BluetoothHCIInquiryLength = u8;
pub type BluetoothHCIResponseCount = u8;
pub type BluetoothHCICountryCode = u8;
pub type BluetoothHCIModeInterval = u16;
pub type BluetoothHCISniffAttemptCount = u16;
pub type BluetoothHCISniffTimeout = u16;
pub type BluetoothHCIParkModeBeaconInterval = u16;
pub type BluetoothMaxSlots = u8;
pub type BluetoothManufacturerName = u16;
pub type BluetoothLMPVersion = u8;
pub type BluetoothLMPSubversion = u16;
pub type BluetoothHCIConnectionMode = u8;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct BluetoothHCISupportedCommands {
    pub data: [u8; 64usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct BluetoothHCISupportedFeatures {
    pub data: [u8; 8usize],
}
pub type BluetoothHCILESupportedFeatures = BluetoothHCISupportedFeatures;
pub type BluetoothHCILEUsedFeatures = BluetoothHCISupportedFeatures;
pub type BluetoothHCIPageNumber = u8;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct BluetoothHCIExtendedFeaturesInfo {
    pub page: BluetoothHCIPageNumber,
    pub maxPage: BluetoothHCIPageNumber,
    pub data: [u8; 8usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct BluetoothEventFilterCondition {
    pub data: [u8; 7usize],
}
pub type BluetoothHCIFailedContactCount = u16;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct BluetoothHCIFailedContactInfo {
    pub count: BluetoothHCIFailedContactCount,
    pub handle: BluetoothConnectionHandle,
}
pub type BluetoothHCIRSSIValue = SInt8;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct BluetoothHCIRSSIInfo {
    pub handle: BluetoothConnectionHandle,
    pub RSSIValue: BluetoothHCIRSSIValue,
}
pub type BluetoothHCILinkQuality = u8;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct BluetoothHCILinkQualityInfo {
    pub handle: BluetoothConnectionHandle,
    pub qualityValue: BluetoothHCILinkQuality,
}
pub type BluetoothHCIEncryptionKeySize = u8;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct BluetoothHCIEncryptionKeySizeInfo {
    pub handle: BluetoothConnectionHandle,
    pub keySize: BluetoothHCIEncryptionKeySize,
}
pub type BluetoothHCIRole = u8;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct BluetoothHCIRoleInfo {
    pub role: u8,
    pub handle: BluetoothConnectionHandle,
}
pub type BluetoothHCILinkPolicySettings = u16;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct BluetoothHCILinkPolicySettingsInfo {
    pub settings: BluetoothHCILinkPolicySettings,
    pub handle: BluetoothConnectionHandle,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct BluetoothHCIQualityOfServiceSetupParams {
    pub flags: u8,
    pub serviceType: u8,
    pub tokenRate: u32,
    pub peakBandwidth: u32,
    pub latency: u32,
    pub delayVariation: u32,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct BluetoothHCISetupSynchronousConnectionParams {
    pub transmitBandwidth: u32,
    pub receiveBandwidth: u32,
    pub maxLatency: u16,
    pub voiceSetting: u16,
    pub retransmissionEffort: u8,
    pub packetType: u16,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct BluetoothHCIAcceptSynchronousConnectionRequestParams {
    pub transmitBandwidth: u32,
    pub receiveBandwidth: u32,
    pub maxLatency: u16,
    pub contentFormat: u16,
    pub retransmissionEffort: u8,
    pub packetType: u16,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct BluetoothHCIEnhancedSetupSynchronousConnectionParams {
    pub transmitBandwidth: u32,
    pub receiveBandwidth: u32,
    pub transmitCodingFormat: u64,
    pub receiveCodingFormat: u64,
    pub transmitCodecFrameSize: u16,
    pub receiveCodecFrameSize: u16,
    pub inputBandwidth: u32,
    pub outputBandwidth: u32,
    pub inputCodingFormat: u64,
    pub outputCodingFormat: u64,
    pub inputCodedDataSize: u16,
    pub outputCodedDataSize: u16,
    pub inputPCMDataFormat: u8,
    pub outputPCMDataFormat: u8,
    pub inputPCMSamplePayloadMSBPosition: u8,
    pub outputPCMSamplePayloadMSBPosition: u8,
    pub inputDataPath: u8,
    pub outputDataPath: u8,
    pub inputTransportUnitSize: u8,
    pub outputTransportUnitSize: u8,
    pub maxLatency: u16,
    pub packetType: u16,
    pub retransmissionEffort: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct BluetoothHCIEnhancedAcceptSynchronousConnectionRequestParams {
    pub transmitBandwidth: u32,
    pub receiveBandwidth: u32,
    pub transmitCodingFormat: u64,
    pub receiveCodingFormat: u64,
    pub transmitCodecFrameSize: u16,
    pub receiveCodecFrameSize: u16,
    pub inputBandwidth: u32,
    pub outputBandwidth: u32,
    pub inputCodingFormat: u64,
    pub outputCodingFormat: u64,
    pub inputCodedDataSize: u16,
    pub outputCodedDataSize: u16,
    pub inputPCMDataFormat: u8,
    pub outputPCMDataFormat: u8,
    pub inputPCMSamplePayloadMSBPosition: u8,
    pub outputPCMSamplePayloadMSBPosition: u8,
    pub inputDataPath: u8,
    pub outputDataPath: u8,
    pub inputTransportUnitSize: u8,
    pub outputTransportUnitSize: u8,
    pub maxLatency: u16,
    pub packetType: u16,
    pub retransmissionEffort: u8,
}
pub type BluetoothHCILoopbackMode = u8;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct BluetoothReadClockInfo {
    pub handle: BluetoothConnectionHandle,
    pub clock: u32,
    pub accuracy: u16,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct BluetoothHCIEventFlowSpecificationData {
    pub connectionHandle: BluetoothConnectionHandle,
    pub flags: u8,
    pub flowDirection: u8,
    pub serviceType: u8,
    pub tokenRate: u32,
    pub tokenBucketSize: u32,
    pub peakBandwidth: u32,
    pub accessLatency: u32,
}
pub type BluetoothHCIOperationID = u32;
pub type BluetoothHCIEventID = u32;
pub type BluetoothHCIDataID = u32;
pub type BluetoothHCISignalID = u32;
pub type BluetoothHCITransportID = u32;
pub type BluetoothHCITransportCommandID = u32;
pub type BluetoothHCIRequestID = u32;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct BluetoothHCIVersionInfo {
    pub manufacturerName: BluetoothManufacturerName,
    pub lmpVersion: BluetoothLMPVersion,
    pub lmpSubVersion: BluetoothLMPSubversion,
    pub hciVersion: u8,
    pub hciRevision: u16,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct BluetoothHCIBufferSize {
    pub ACLDataPacketLength: u16,
    pub SCODataPacketLength: u8,
    pub totalNumACLDataPackets: u16,
    pub totalNumSCODataPackets: u16,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct BluetoothHCILEBufferSize {
    pub ACLDataPacketLength: u16,
    pub totalNumACLDataPackets: u8,
}
pub type BluetoothHCIConnectionAcceptTimeout = u16;
pub type BluetoothHCIPageTimeout = u16;
pub type BluetoothHCINumLinkKeysDeleted = u16;
pub type BluetoothHCINumLinkKeysToWrite = u8;
pub type BluetoothHCIDeleteStoredLinkKeyFlag = u8;
pub type BluetoothHCIReadStoredLinkKeysFlag = u8;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct BluetoothHCIStoredLinkKeysInfo {
    pub numLinkKeysRead: u16,
    pub maxNumLinkKeysAllowedInDevice: u16,
}
pub type BluetoothHCIPageScanMode = u8;
pub type BluetoothHCIPageScanPeriodMode = u8;
pub type BluetoothHCIPageScanEnableState = u8;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct BluetoothHCIScanActivity {
    pub scanInterval: u16,
    pub scanWindow: u16,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct BluetoothHCIInquiryAccessCode {
    pub data: [u8; 3usize],
}
pub type BluetoothHCIInquiryAccessCodeCount = u8;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct BluetoothHCICurrentInquiryAccessCodes {
    pub count: BluetoothHCIInquiryAccessCodeCount,
    pub codes: *mut BluetoothHCIInquiryAccessCode,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct BluetoothHCICurrentInquiryAccessCodesForWrite {
    pub count: BluetoothHCIInquiryAccessCodeCount,
    pub codes: [u8; 192usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct BluetoothHCILinkSupervisionTimeout {
    pub handle: BluetoothConnectionHandle,
    pub timeout: u16,
}
pub type BluetoothHCIFlowControlState = u8;
pub type BluetoothHCITransmitPowerLevel = SInt8;
pub type BluetoothHCITransmitPowerLevelType = u8;
pub type BluetoothHCIAFHChannelAssessmentMode = u8;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct BluetoothHCITransmitPowerLevelInfo {
    pub handle: BluetoothConnectionHandle,
    pub level: BluetoothHCITransmitPowerLevel,
}
pub type BluetoothHCINumBroadcastRetransmissions = u8;
pub type BluetoothHCIHoldModeActivity = u8;
pub type BluetoothHCIAuthenticationEnable = u8;
pub type BluetoothHCIEncryptionMode = u8;
pub type BluetoothHCIAutomaticFlushTimeout = u16;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct BluetoothHCIAutomaticFlushTimeoutInfo {
    pub handle: BluetoothConnectionHandle,
    pub timeout: BluetoothHCIAutomaticFlushTimeout,
}
pub type BluetoothTransportInfoPtr = *mut BluetoothTransportInfo;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct BluetoothTransportInfo {
    pub productID: u32,
    pub vendorID: u32,
    pub type_: u32,
    pub productName: [::std::os::raw::c_char; 35usize],
    pub vendorName: [::std::os::raw::c_char; 35usize],
    pub totalDataBytesSent: u64,
    pub totalSCOBytesSent: u64,
    pub totalDataBytesReceived: u64,
    pub totalSCOBytesReceived: u64,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct BluetoothHCIInquiryResult {
    pub deviceAddress: BluetoothDeviceAddress,
    pub pageScanRepetitionMode: BluetoothPageScanRepetitionMode,
    pub pageScanPeriodMode: BluetoothHCIPageScanPeriodMode,
    pub pageScanMode: BluetoothHCIPageScanMode,
    pub classOfDevice: BluetoothClassOfDevice,
    pub clockOffset: BluetoothClockOffset,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct BluetoothHCIInquiryResults {
    pub results: [BluetoothHCIInquiryResult; 50usize],
    pub count: IOItemCount,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct BluetoothHCIInquiryWithRSSIResult {
    pub deviceAddress: BluetoothDeviceAddress,
    pub pageScanRepetitionMode: BluetoothPageScanRepetitionMode,
    pub reserved: u8,
    pub classOfDevice: BluetoothClassOfDevice,
    pub clockOffset: BluetoothClockOffset,
    pub RSSIValue: BluetoothHCIRSSIValue,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct BluetoothHCIInquiryWithRSSIResults {
    pub results: [BluetoothHCIInquiryWithRSSIResult; 50usize],
    pub count: IOItemCount,
}
pub type BluetoothHCIFECRequired = u8;
pub type BluetoothHCIInquiryMode = u8;
pub type BluetoothHCIInquiryScanType = u8;
pub type BluetoothHCIExtendedInquiryResponseDataType = u8;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct BluetoothHCIExtendedInquiryResponse {
    pub data: [u8; 240usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct BluetoothHCIReadExtendedInquiryResponseResults {
    pub outFECRequired: BluetoothHCIFECRequired,
    pub extendedInquiryResponse: BluetoothHCIExtendedInquiryResponse,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct BluetoothHCIExtendedInquiryResult {
    pub numberOfReponses: u8,
    pub deviceAddress: BluetoothDeviceAddress,
    pub pageScanRepetitionMode: BluetoothPageScanRepetitionMode,
    pub reserved: u8,
    pub classOfDevice: BluetoothClassOfDevice,
    pub clockOffset: BluetoothClockOffset,
    pub RSSIValue: BluetoothHCIRSSIValue,
    pub extendedInquiryResponse: BluetoothHCIExtendedInquiryResponse,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct BluetoothHCIReadLMPHandleResults {
    pub handle: BluetoothConnectionHandle,
    pub lmp_handle: BluetoothLMPHandle,
    pub reserved: u32,
}
pub type BluetoothHCISimplePairingMode = u8;
pub type BluetoothSimplePairingDebugMode = u8;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct BluetoothHCISimplePairingOOBData {
    pub data: [u8; 16usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct BluetoothHCIReadLocalOOBDataResults {
    pub hash: BluetoothHCISimplePairingOOBData,
    pub randomizer: BluetoothHCISimplePairingOOBData,
}
pub type BluetoothIOCapability = u8;
pub type BluetoothOOBDataPresence = u8;
pub type BluetoothAuthenticationRequirements = u8;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct BluetoothIOCapabilityResponse {
    pub deviceAddress: BluetoothDeviceAddress,
    pub ioCapability: BluetoothIOCapability,
    pub OOBDataPresence: BluetoothOOBDataPresence,
    pub authenticationRequirements: BluetoothAuthenticationRequirements,
}
pub type BluetoothPasskey = u32;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct BluetoothUserPasskeyNotification {
    pub deviceAddress: BluetoothDeviceAddress,
    pub passkey: BluetoothPasskey,
}
pub type BluetoothKeypressNotificationType = u8;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct BluetoothKeypressNotification {
    pub deviceAddress: BluetoothDeviceAddress,
    pub notificationType: BluetoothKeypressNotificationType,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct BluetoothRemoteHostSupportedFeaturesNotification {
    pub deviceAddress: BluetoothDeviceAddress,
    pub hostSupportedFeatures: BluetoothHCISupportedFeatures,
}
pub type TransmissionPower = SInt8;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct BluetoothAFHHostChannelClassification {
    pub data: [u8; 10usize],
}
pub type BluetoothAFHMode = u8;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct BluetoothAFHResults {
    pub handle: BluetoothConnectionHandle,
    pub mode: BluetoothAFHMode,
    pub afhMap: [u8; 10usize],
}
pub type BluetoothNumericValue = u32;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct BluetoothUserConfirmationRequest {
    pub deviceAddress: BluetoothDeviceAddress,
    pub numericValue: BluetoothNumericValue,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct BluetoothHCIEventSimplePairingCompleteResults {
    pub deviceAddress: BluetoothDeviceAddress,
}
pub type BluetoothHCIEventCode = u8;
pub type BluetoothLinkType = u8;
pub type BluetoothHCIContentFormat = u16;
pub type BluetoothHCIVoiceSetting = u16;
pub type BluetoothHCISupportedIAC = u8;
pub type BluetoothHCITransmitBandwidth = u32;
pub type BluetoothHCIReceiveBandwidth = u32;
pub type BluetoothHCITransmitCodingFormat = u64;
pub type BluetoothHCIReceiveCodingFormat = u64;
pub type BluetoothHCITransmitCodecFrameSize = u16;
pub type BluetoothHCIReceiveCodecFrameSize = u16;
pub type BluetoothHCIInputBandwidth = u32;
pub type BluetoothHCIOutputBandwidth = u32;
pub type BluetoothHCIInputCodingFormat = u64;
pub type BluetoothHCIOutputCodingFormat = u64;
pub type BluetoothHCIInputCodedDataSize = u16;
pub type BluetoothHCIOutputCodedDataSize = u16;
pub type BluetoothHCIInputPCMDataFormat = u8;
pub type BluetoothHCIOutputPCMDataFormat = u8;
pub type BluetoothHCIInputPCMSamplePayloadMSBPosition = u8;
pub type BluetoothHCIOutputPCMSamplePayloadMSBPosition = u8;
pub type BluetoothHCIInputDataPath = u8;
pub type BluetoothHCIOutputDataPath = u8;
pub type BluetoothHCIInputTransportUnitSize = u8;
pub type BluetoothHCIOutputTransportUnitSize = u8;
pub type BluetoothHCIMaxLatency = u16;
pub type BluetoothHCIRetransmissionEffort = u8;
pub type BluetoothAirMode = u8;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct BluetoothSynchronousConnectionInfo {
    pub transmitBandWidth: BluetoothHCITransmitBandwidth,
    pub receiveBandWidth: BluetoothHCIReceiveBandwidth,
    pub maxLatency: BluetoothHCIMaxLatency,
    pub voiceSetting: BluetoothHCIVoiceSetting,
    pub retransmissionEffort: BluetoothHCIRetransmissionEffort,
    pub packetType: BluetoothPacketType,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct BluetoothEnhancedSynchronousConnectionInfo {
    pub transmitBandWidth: BluetoothHCITransmitBandwidth,
    pub receiveBandWidth: BluetoothHCIReceiveBandwidth,
    pub transmitCodingFormat: BluetoothHCITransmitCodingFormat,
    pub receiveCodingFormat: BluetoothHCIReceiveCodingFormat,
    pub transmitCodecFrameSize: BluetoothHCITransmitCodecFrameSize,
    pub receiveCodecFrameSize: BluetoothHCIReceiveCodecFrameSize,
    pub inputBandwidth: BluetoothHCIInputBandwidth,
    pub outputBandwidth: BluetoothHCIOutputBandwidth,
    pub inputCodingFormat: BluetoothHCIInputCodingFormat,
    pub outputCodingFormat: BluetoothHCIOutputCodingFormat,
    pub inputCodedDataSize: BluetoothHCIInputCodedDataSize,
    pub outputCodedDataSize: BluetoothHCIOutputCodedDataSize,
    pub inputPCMDataFormat: BluetoothHCIInputPCMDataFormat,
    pub outputPCMDataFormat: BluetoothHCIOutputPCMDataFormat,
    pub inputPCMSampelPayloadMSBPosition: BluetoothHCIInputPCMSamplePayloadMSBPosition,
    pub outputPCMSampelPayloadMSBPosition: BluetoothHCIOutputPCMSamplePayloadMSBPosition,
    pub inputDataPath: BluetoothHCIInputDataPath,
    pub outputDataPath: BluetoothHCIOutputDataPath,
    pub inputTransportUnitSize: BluetoothHCIInputTransportUnitSize,
    pub outputTransportUnitSize: BluetoothHCIOutputTransportUnitSize,
    pub maxLatency: BluetoothHCIMaxLatency,
    pub voiceSetting: BluetoothHCIVoiceSetting,
    pub retransmissionEffort: BluetoothHCIRetransmissionEffort,
    pub packetType: BluetoothPacketType,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct BluetoothHCIEventSynchronousConnectionCompleteResults {
    pub connectionHandle: BluetoothConnectionHandle,
    pub deviceAddress: BluetoothDeviceAddress,
    pub linkType: BluetoothLinkType,
    pub transmissionInterval: u8,
    pub retransmissionWindow: u8,
    pub receivePacketLength: u16,
    pub transmitPacketLength: u16,
    pub airMode: BluetoothAirMode,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct BluetoothHCIEventSynchronousConnectionChangedResults {
    pub connectionHandle: BluetoothConnectionHandle,
    pub transmissionInterval: u8,
    pub retransmissionWindow: u8,
    pub receivePacketLength: u16,
    pub transmitPacketLength: u16,
}
pub type BluetoothHCIStatus = u8;
pub type BluetoothHCIEventStatus = u8;
pub type BluetoothHCIEventMask = u64;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct BluetoothHCIEventConnectionCompleteResults {
    pub connectionHandle: BluetoothConnectionHandle,
    pub deviceAddress: BluetoothDeviceAddress,
    pub linkType: BluetoothLinkType,
    pub encryptionMode: BluetoothHCIEncryptionMode,
}
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct BluetoothHCIEventLEConnectionCompleteResults {
    pub connectionHandle: BluetoothConnectionHandle,
    pub role: u8,
    pub peerAddressType: u8,
    pub peerAddress: BluetoothDeviceAddress,
    pub connInterval: u16,
    pub connLatency: u16,
    pub supervisionTimeout: u16,
    pub masterClockAccuracy: u8,
}
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct BluetoothHCIEventLEEnhancedConnectionCompleteResults {
    pub connectionHandle: BluetoothConnectionHandle,
    pub role: u8,
    pub peerAddressType: u8,
    pub peerAddress: BluetoothDeviceAddress,
    pub localResolvablePrivateAddress: BluetoothDeviceAddress,
    pub peerResolvablePrivateAddress: BluetoothDeviceAddress,
    pub connInterval: u16,
    pub connLatency: u16,
    pub supervisionTimeout: u16,
    pub masterClockAccuracy: u8,
}
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct BluetoothHCIEventLEConnectionUpdateCompleteResults {
    pub connectionHandle: BluetoothConnectionHandle,
    pub connInterval: u16,
    pub connLatency: u16,
    pub supervisionTimeout: u16,
}
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct BluetoothHCIEventLEReadRemoteUsedFeaturesCompleteResults {
    pub connectionHandle: BluetoothConnectionHandle,
    pub usedFeatures: BluetoothHCISupportedFeatures,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct BluetoothHCIEventDisconnectionCompleteResults {
    pub connectionHandle: BluetoothConnectionHandle,
    pub reason: BluetoothReasonCode,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct BluetoothHCIEventReadSupportedFeaturesResults {
    pub connectionHandle: BluetoothConnectionHandle,
    pub supportedFeatures: BluetoothHCISupportedFeatures,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct BluetoothHCIEventReadExtendedFeaturesResults {
    pub connectionHandle: BluetoothConnectionHandle,
    pub supportedFeaturesInfo: BluetoothHCIExtendedFeaturesInfo,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct BluetoothHCIEventReadRemoteVersionInfoResults {
    pub connectionHandle: BluetoothConnectionHandle,
    pub lmpVersion: BluetoothLMPVersion,
    pub manufacturerName: BluetoothManufacturerName,
    pub lmpSubversion: BluetoothLMPSubversion,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct BluetoothHCIEventRemoteNameRequestResults {
    pub deviceAddress: BluetoothDeviceAddress,
    pub deviceName: BluetoothDeviceName,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct BluetoothHCIEventReadClockOffsetResults {
    pub connectionHandle: BluetoothConnectionHandle,
    pub clockOffset: BluetoothClockOffset,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct BluetoothHCIEventConnectionRequestResults {
    pub deviceAddress: BluetoothDeviceAddress,
    pub classOfDevice: BluetoothClassOfDevice,
    pub linkType: BluetoothLinkType,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct BluetoothHCIEventLinkKeyNotificationResults {
    pub deviceAddress: BluetoothDeviceAddress,
    pub linkKey: BluetoothKey,
    pub keyType: BluetoothKeyType,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct BluetoothHCIEventMaxSlotsChangeResults {
    pub connectionHandle: BluetoothConnectionHandle,
    pub maxSlots: BluetoothMaxSlots,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct BluetoothHCIEventModeChangeResults {
    pub connectionHandle: BluetoothConnectionHandle,
    pub mode: BluetoothHCIConnectionMode,
    pub modeInterval: BluetoothHCIModeInterval,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct BluetoothHCIEventReturnLinkKeysResults {
    pub __bindgen_anon_1: BluetoothHCIEventReturnLinkKeysResults__bindgen_ty_1,
    pub numLinkKeys: u8,
    pub linkKeys: [BluetoothHCIEventReturnLinkKeysResults__bindgen_ty_1; 1usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct BluetoothHCIEventReturnLinkKeysResults__bindgen_ty_1 {
    pub deviceAddress: BluetoothDeviceAddress,
    pub linkKey: BluetoothKey,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct BluetoothHCIEventAuthenticationCompleteResults {
    pub connectionHandle: BluetoothConnectionHandle,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct BluetoothHCIEventEncryptionChangeResults {
    pub connectionHandle: BluetoothConnectionHandle,
    pub enable: BluetoothEncryptionEnable,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct BluetoothHCIEventChangeConnectionLinkKeyCompleteResults {
    pub connectionHandle: BluetoothConnectionHandle,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct BluetoothHCIEventMasterLinkKeyCompleteResults {
    pub connectionHandle: BluetoothConnectionHandle,
    pub keyFlag: BluetoothKeyFlag,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct BluetoothHCIEventQoSSetupCompleteResults {
    pub connectionHandle: BluetoothConnectionHandle,
    pub setupParams: BluetoothHCIQualityOfServiceSetupParams,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct BluetoothHCIEventHardwareErrorResults {
    pub error: BluetoothHCIStatus,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct BluetoothHCIEventFlushOccurredResults {
    pub connectionHandle: BluetoothConnectionHandle,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct BluetoothHCIEventRoleChangeResults {
    pub connectionHandle: BluetoothConnectionHandle,
    pub deviceAddress: BluetoothDeviceAddress,
    pub role: BluetoothRole,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct BluetoothHCIEventDataBufferOverflowResults {
    pub linkType: BluetoothLinkType,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct BluetoothHCIEventConnectionPacketTypeResults {
    pub connectionHandle: BluetoothConnectionHandle,
    pub packetType: BluetoothPacketType,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct BluetoothHCIEventReadRemoteSupportedFeaturesResults {
    pub error: BluetoothHCIStatus,
    pub connectionHandle: BluetoothConnectionHandle,
    pub lmpFeatures: BluetoothHCISupportedFeatures,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct BluetoothHCIEventReadRemoteExtendedFeaturesResults {
    pub error: BluetoothHCIStatus,
    pub connectionHandle: BluetoothConnectionHandle,
    pub page: BluetoothHCIPageNumber,
    pub maxPage: BluetoothHCIPageNumber,
    pub lmpFeatures: BluetoothHCISupportedFeatures,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct BluetoothHCIEventQoSViolationResults {
    pub connectionHandle: BluetoothConnectionHandle,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct BluetoothHCIEventPageScanModeChangeResults {
    pub deviceAddress: BluetoothDeviceAddress,
    pub pageScanMode: BluetoothPageScanMode,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct BluetoothHCIEventPageScanRepetitionModeChangeResults {
    pub deviceAddress: BluetoothDeviceAddress,
    pub pageScanRepetitionMode: BluetoothPageScanRepetitionMode,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct BluetoothHCIEventVendorSpecificResults {
    pub length: u8,
    pub data: [u8; 255usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct BluetoothHCIEventEncryptionKeyRefreshCompleteResults {
    pub connectionHandle: BluetoothConnectionHandle,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct BluetoothHCIEventSniffSubratingResults {
    pub connectionHandle: BluetoothConnectionHandle,
    pub maxTransmitLatency: u16,
    pub maxReceiveLatency: u16,
    pub minRemoteTimeout: u16,
    pub minLocalTimeout: u16,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct BluetoothHCIEventLEMetaResults {
    pub length: u8,
    pub data: [u8; 255usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct BluetoothHCIEventLELongTermKeyRequestResults {
    pub connectionHandle: BluetoothConnectionHandle,
    pub randomNumber: [u8; 8usize],
    pub ediv: u16,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct BluetoothHCIRequestCallbackInfo {
    pub userCallback: mach_vm_address_t,
    pub userRefCon: mach_vm_address_t,
    pub internalRefCon: mach_vm_address_t,
    pub asyncIDRefCon: mach_vm_address_t,
    pub reserved: mach_vm_address_t,
}
pub type BluetoothHCIPowerState = ::std::os::raw::c_uint;
pub type BluetoothRFCOMMChannelID = u8;
pub type BluetoothRFCOMMMTU = u16;
pub type BluetoothRFCOMMParityType = ::std::os::raw::c_uint;
pub type BluetoothRFCOMMLineStatus = ::std::os::raw::c_uint;
pub type BluetoothSDPPDUID = u8;
pub type BluetoothSDPErrorCode = u16;
pub type BluetoothSDPTransactionID = u16;
pub type BluetoothSDPServiceRecordHandle = u32;
pub type BluetoothSDPUUID16 = u16;
pub type BluetoothSDPUUID32 = u32;
pub type BluetoothSDPDataElementTypeDescriptor = u8;
pub type BluetoothSDPDataElementSizeDescriptor = u8;
pub type BluetoothSDPServiceAttributeID = u16;
pub type BluetoothLEScanType = ::std::os::raw::c_uint;
pub type BluetoothLEAddressType = ::std::os::raw::c_uint;
pub type BluetoothLEScanFilter = ::std::os::raw::c_uint;
pub type BluetoothLEScan = ::std::os::raw::c_uint;
pub type BluetoothLEConnectionInterval = ::std::os::raw::c_uint;
pub type BluetoothLEScanDuplicateFilter = ::std::os::raw::c_uint;
pub type BluetoothLEAdvertisingType = ::std::os::raw::c_uint;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OpaqueIOBluetoothObjectRef {
    _unused: [u8; 0],
}
pub type IOBluetoothObjectRef = *mut OpaqueIOBluetoothObjectRef;
pub type IOBluetoothDeviceRef = *mut OpaqueIOBluetoothObjectRef;
pub type IOBluetoothL2CAPChannelRef = *mut OpaqueIOBluetoothObjectRef;
pub type IOBluetoothRFCOMMChannelRef = *mut OpaqueIOBluetoothObjectRef;
pub type IOBluetoothSDPServiceRecordRef = *mut OpaqueIOBluetoothObjectRef;
pub type IOBluetoothSDPUUIDRef = *mut OpaqueIOBluetoothObjectRef;
pub type IOBluetoothSDPDataElementRef = *mut OpaqueIOBluetoothObjectRef;
pub type IOBluetoothUserNotificationRef = *mut OpaqueIOBluetoothObjectRef;
pub type IOBluetoothObjectID = ::std::os::raw::c_ulong;
pub type IOBluetoothDeviceSearchOptions = UInt32;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct IOBluetoothDeviceSearchDeviceAttributes {
    pub address: BluetoothDeviceAddress,
    pub name: BluetoothDeviceName,
    pub serviceClassMajor: BluetoothServiceClassMajor,
    pub deviceClassMajor: BluetoothDeviceClassMajor,
    pub deviceClassMinor: BluetoothDeviceClassMinor,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct IOBluetoothDeviceSearchAttributes {
    pub options: IOBluetoothDeviceSearchOptions,
    pub maxResults: IOItemCount,
    pub deviceAttributeCount: IOItemCount,
    pub attributeList: *mut IOBluetoothDeviceSearchDeviceAttributes,
}
pub type IOBluetoothDeviceSearchTypes = UInt32;
pub type IOBluetoothUserNotificationChannelDirection = ::std::os::raw::c_uint;
pub type IOBluetoothUserNotificationCallback = ::std::option::Option<
    unsafe extern "C" fn(
        userRefCon: *mut ::std::os::raw::c_void,
        inRef: IOBluetoothUserNotificationRef,
        objectRef: IOBluetoothObjectRef,
    ),
>;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct IOBluetoothObject(pub id);
impl std::ops::Deref for IOBluetoothObject {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for IOBluetoothObject {}
impl IOBluetoothObject {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"IOBluetoothObject").unwrap(), alloc) })
    }
}
impl PNSCopying for IOBluetoothObject {}
impl INSObject for IOBluetoothObject {}
impl PNSObject for IOBluetoothObject {}
impl std::convert::TryFrom<NSObject> for IOBluetoothObject {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<IOBluetoothObject, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"IOBluetoothObject").unwrap()) };
        if is_kind_of {
            Ok(IOBluetoothObject(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to IOBluetoothObject")
        }
    }
}
impl IIOBluetoothObject for IOBluetoothObject {}
pub trait IIOBluetoothObject: Sized + std::ops::Deref {}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct IOBluetoothUserNotification(pub id);
impl std::ops::Deref for IOBluetoothUserNotification {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for IOBluetoothUserNotification {}
impl IOBluetoothUserNotification {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"IOBluetoothUserNotification").unwrap(), alloc) })
    }
}
impl INSObject for IOBluetoothUserNotification {}
impl PNSObject for IOBluetoothUserNotification {}
impl std::convert::TryFrom<NSObject> for IOBluetoothUserNotification {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<IOBluetoothUserNotification, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"IOBluetoothUserNotification").unwrap()) };
        if is_kind_of {
            Ok(IOBluetoothUserNotification(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to IOBluetoothUserNotification")
        }
    }
}
impl IIOBluetoothUserNotification for IOBluetoothUserNotification {}
pub trait IIOBluetoothUserNotification: Sized + std::ops::Deref {
    unsafe fn unregister(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, unregister)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct IOBluetoothDevice(pub id);
impl std::ops::Deref for IOBluetoothDevice {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for IOBluetoothDevice {}
impl IOBluetoothDevice {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"IOBluetoothDevice").unwrap(), alloc) })
    }
}
impl PNSCoding for IOBluetoothDevice {}
impl PNSSecureCoding for IOBluetoothDevice {}
impl IIOBluetoothObject for IOBluetoothDevice {}
impl PNSCopying for IOBluetoothDevice {}
impl From<IOBluetoothDevice> for IOBluetoothObject {
    fn from(child: IOBluetoothDevice) -> IOBluetoothObject {
        IOBluetoothObject(child.0)
    }
}
impl std::convert::TryFrom<IOBluetoothObject> for IOBluetoothDevice {
    type Error = &'static str;
    fn try_from(parent: IOBluetoothObject) -> Result<IOBluetoothDevice, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"IOBluetoothDevice").unwrap()) };
        if is_kind_of {
            Ok(IOBluetoothDevice(parent.0))
        } else {
            Err("This IOBluetoothObject cannot be downcasted to IOBluetoothDevice")
        }
    }
}
impl INSObject for IOBluetoothDevice {}
impl PNSObject for IOBluetoothDevice {}
impl IIOBluetoothDevice for IOBluetoothDevice {}
pub trait IIOBluetoothDevice: Sized + std::ops::Deref {
    unsafe fn registerForDisconnectNotification_selector_(
        &self,
        observer: id,
        inSelector: objc2::runtime::Sel,
    ) -> IOBluetoothUserNotification
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, registerForDisconnectNotification : observer, selector : inSelector)
    }
    unsafe fn getDeviceRef(&self) -> IOBluetoothDeviceRef
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, getDeviceRef)
    }
    unsafe fn openL2CAPChannelSync_withPSM_delegate_(
        &self,
        newChannel: *mut IOBluetoothL2CAPChannel,
        psm: BluetoothL2CAPPSM,
        channelDelegate: id,
    ) -> IOReturn
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, openL2CAPChannelSync : newChannel, withPSM : psm, delegate : channelDelegate)
    }
    unsafe fn openL2CAPChannelAsync_withPSM_delegate_(
        &self,
        newChannel: *mut IOBluetoothL2CAPChannel,
        psm: BluetoothL2CAPPSM,
        channelDelegate: id,
    ) -> IOReturn
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, openL2CAPChannelAsync : newChannel, withPSM : psm, delegate : channelDelegate)
    }
    unsafe fn openL2CAPChannel_findExisting_newChannel_(
        &self,
        psm: BluetoothL2CAPPSM,
        findExisting: BOOL,
        newChannel: *mut IOBluetoothL2CAPChannel,
    ) -> IOReturn
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, openL2CAPChannel : psm, findExisting : findExisting, newChannel : newChannel)
    }
    unsafe fn sendL2CAPEchoRequest_length_(
        &self,
        data: *mut ::std::os::raw::c_void,
        length: UInt16,
    ) -> IOReturn
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, sendL2CAPEchoRequest : data, length : length)
    }
    unsafe fn openRFCOMMChannel_channel_(
        &self,
        channelID: BluetoothRFCOMMChannelID,
        rfcommChannel: *mut IOBluetoothRFCOMMChannel,
    ) -> IOReturn
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, openRFCOMMChannel : channelID, channel : rfcommChannel)
    }
    unsafe fn openRFCOMMChannelSync_withChannelID_delegate_(
        &self,
        rfcommChannel: *mut IOBluetoothRFCOMMChannel,
        channelID: BluetoothRFCOMMChannelID,
        channelDelegate: id,
    ) -> IOReturn
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, openRFCOMMChannelSync : rfcommChannel, withChannelID : channelID, delegate : channelDelegate)
    }
    unsafe fn openRFCOMMChannelAsync_withChannelID_delegate_(
        &self,
        rfcommChannel: *mut IOBluetoothRFCOMMChannel,
        channelID: BluetoothRFCOMMChannelID,
        channelDelegate: id,
    ) -> IOReturn
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, openRFCOMMChannelAsync : rfcommChannel, withChannelID : channelID, delegate : channelDelegate)
    }
    unsafe fn getClassOfDevice(&self) -> BluetoothClassOfDevice
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, getClassOfDevice)
    }
    unsafe fn getServiceClassMajor(&self) -> BluetoothServiceClassMajor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, getServiceClassMajor)
    }
    unsafe fn getDeviceClassMajor(&self) -> BluetoothDeviceClassMajor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, getDeviceClassMajor)
    }
    unsafe fn getDeviceClassMinor(&self) -> BluetoothDeviceClassMinor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, getDeviceClassMinor)
    }
    unsafe fn getName(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, getName)
    }
    unsafe fn getNameOrAddress(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, getNameOrAddress)
    }
    unsafe fn getLastNameUpdate(&self) -> NSDate
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, getLastNameUpdate)
    }
    unsafe fn getAddress(&self) -> *const BluetoothDeviceAddress
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, getAddress)
    }
    unsafe fn getAddressString(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, getAddressString)
    }
    unsafe fn getPageScanRepetitionMode(&self) -> BluetoothPageScanRepetitionMode
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, getPageScanRepetitionMode)
    }
    unsafe fn getPageScanPeriodMode(&self) -> BluetoothPageScanPeriodMode
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, getPageScanPeriodMode)
    }
    unsafe fn getPageScanMode(&self) -> BluetoothPageScanMode
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, getPageScanMode)
    }
    unsafe fn getClockOffset(&self) -> BluetoothClockOffset
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, getClockOffset)
    }
    unsafe fn getLastInquiryUpdate(&self) -> NSDate
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, getLastInquiryUpdate)
    }
    unsafe fn RSSI(&self) -> BluetoothHCIRSSIValue
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, RSSI)
    }
    unsafe fn rawRSSI(&self) -> BluetoothHCIRSSIValue
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, rawRSSI)
    }
    unsafe fn isConnected(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isConnected)
    }
    unsafe fn openConnection(&self) -> IOReturn
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, openConnection)
    }
    unsafe fn openConnection_(&self, target: id) -> IOReturn
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, openConnection : target)
    }
    unsafe fn openConnection_withPageTimeout_authenticationRequired_(
        &self,
        target: id,
        pageTimeoutValue: BluetoothHCIPageTimeout,
        authenticationRequired: BOOL,
    ) -> IOReturn
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, openConnection : target, withPageTimeout : pageTimeoutValue, authenticationRequired : authenticationRequired)
    }
    unsafe fn closeConnection(&self) -> IOReturn
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, closeConnection)
    }
    unsafe fn remoteNameRequest_(&self, target: id) -> IOReturn
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, remoteNameRequest : target)
    }
    unsafe fn remoteNameRequest_withPageTimeout_(
        &self,
        target: id,
        pageTimeoutValue: BluetoothHCIPageTimeout,
    ) -> IOReturn
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, remoteNameRequest : target, withPageTimeout : pageTimeoutValue)
    }
    unsafe fn requestAuthentication(&self) -> IOReturn
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, requestAuthentication)
    }
    unsafe fn getConnectionHandle(&self) -> BluetoothConnectionHandle
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, getConnectionHandle)
    }
    unsafe fn isIncoming(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isIncoming)
    }
    unsafe fn getLinkType(&self) -> BluetoothLinkType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, getLinkType)
    }
    unsafe fn getEncryptionMode(&self) -> BluetoothHCIEncryptionMode
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, getEncryptionMode)
    }
    unsafe fn performSDPQuery_(&self, target: id) -> IOReturn
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, performSDPQuery : target)
    }
    unsafe fn performSDPQuery_uuids_(&self, target: id, uuidArray: NSArray) -> IOReturn
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, performSDPQuery : target, uuids : uuidArray)
    }
    unsafe fn getServices(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, getServices)
    }
    unsafe fn getLastServicesUpdate(&self) -> NSDate
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, getLastServicesUpdate)
    }
    unsafe fn getServiceRecordForUUID_(
        &self,
        sdpUUID: IOBluetoothSDPUUID,
    ) -> IOBluetoothSDPServiceRecord
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, getServiceRecordForUUID : sdpUUID)
    }
    unsafe fn isFavorite(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isFavorite)
    }
    unsafe fn addToFavorites(&self) -> IOReturn
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, addToFavorites)
    }
    unsafe fn removeFromFavorites(&self) -> IOReturn
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, removeFromFavorites)
    }
    unsafe fn recentAccessDate(&self) -> NSDate
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, recentAccessDate)
    }
    unsafe fn isPaired(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isPaired)
    }
    unsafe fn setSupervisionTimeout_(&self, timeout: UInt16) -> IOReturn
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSupervisionTimeout : timeout)
    }
    unsafe fn openL2CAPChannelSync_withPSM_withConfiguration_delegate_(
        &self,
        newChannel: *mut IOBluetoothL2CAPChannel,
        psm: BluetoothL2CAPPSM,
        channelConfiguration: NSDictionary,
        channelDelegate: id,
    ) -> IOReturn
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, openL2CAPChannelSync : newChannel, withPSM : psm, withConfiguration : channelConfiguration, delegate : channelDelegate)
    }
    unsafe fn openL2CAPChannelAsync_withPSM_withConfiguration_delegate_(
        &self,
        newChannel: *mut IOBluetoothL2CAPChannel,
        psm: BluetoothL2CAPPSM,
        channelConfiguration: NSDictionary,
        channelDelegate: id,
    ) -> IOReturn
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, openL2CAPChannelAsync : newChannel, withPSM : psm, withConfiguration : channelConfiguration, delegate : channelDelegate)
    }
    unsafe fn awakeAfterUsingCoder_(&self, coder: NSCoder) -> id
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, awakeAfterUsingCoder : coder)
    }
    unsafe fn classOfDevice(&self) -> BluetoothClassOfDevice
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, classOfDevice)
    }
    unsafe fn serviceClassMajor(&self) -> BluetoothServiceClassMajor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, serviceClassMajor)
    }
    unsafe fn deviceClassMajor(&self) -> BluetoothDeviceClassMajor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, deviceClassMajor)
    }
    unsafe fn deviceClassMinor(&self) -> BluetoothDeviceClassMinor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, deviceClassMinor)
    }
    unsafe fn name(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, name)
    }
    unsafe fn nameOrAddress(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, nameOrAddress)
    }
    unsafe fn lastNameUpdate(&self) -> NSDate
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, lastNameUpdate)
    }
    unsafe fn addressString(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, addressString)
    }
    unsafe fn connectionHandle(&self) -> BluetoothConnectionHandle
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, connectionHandle)
    }
    unsafe fn services(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, services)
    }
    unsafe fn registerForConnectNotifications_selector_(
        observer: id,
        inSelector: objc2::runtime::Sel,
    ) -> IOBluetoothUserNotification
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"IOBluetoothDevice").unwrap(), registerForConnectNotifications : observer, selector : inSelector)
    }
    unsafe fn deviceWithAddress_(address: *const BluetoothDeviceAddress) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"IOBluetoothDevice").unwrap(), deviceWithAddress : address)
    }
    unsafe fn withAddress_(address: *const BluetoothDeviceAddress) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"IOBluetoothDevice").unwrap(), withAddress : address)
    }
    unsafe fn deviceWithAddressString_(address: NSString) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"IOBluetoothDevice").unwrap(), deviceWithAddressString : address)
    }
    unsafe fn withDeviceRef_(deviceRef: IOBluetoothDeviceRef) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"IOBluetoothDevice").unwrap(), withDeviceRef : deviceRef)
    }
    unsafe fn favoriteDevices() -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"IOBluetoothDevice").unwrap(), favoriteDevices)
    }
    unsafe fn recentDevices_(numDevices: ::std::os::raw::c_ulong) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"IOBluetoothDevice").unwrap(), recentDevices : numDevices)
    }
    unsafe fn pairedDevices() -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"IOBluetoothDevice").unwrap(), pairedDevices)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct IOBluetoothDeviceInquiry(pub id);
impl std::ops::Deref for IOBluetoothDeviceInquiry {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for IOBluetoothDeviceInquiry {}
impl IOBluetoothDeviceInquiry {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"IOBluetoothDeviceInquiry").unwrap(), alloc) })
    }
}
impl INSObject for IOBluetoothDeviceInquiry {}
impl PNSObject for IOBluetoothDeviceInquiry {}
impl std::convert::TryFrom<NSObject> for IOBluetoothDeviceInquiry {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<IOBluetoothDeviceInquiry, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"IOBluetoothDeviceInquiry").unwrap()) };
        if is_kind_of {
            Ok(IOBluetoothDeviceInquiry(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to IOBluetoothDeviceInquiry")
        }
    }
}
impl IIOBluetoothDeviceInquiry for IOBluetoothDeviceInquiry {}
pub trait IIOBluetoothDeviceInquiry: Sized + std::ops::Deref {
    unsafe fn initWithDelegate_(&self, delegate: id) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithDelegate : delegate)
    }
    unsafe fn start(&self) -> IOReturn
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, start)
    }
    unsafe fn stop(&self) -> IOReturn
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, stop)
    }
    unsafe fn foundDevices(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, foundDevices)
    }
    unsafe fn clearFoundDevices(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, clearFoundDevices)
    }
    unsafe fn setSearchCriteria_majorDeviceClass_minorDeviceClass_(
        &self,
        inServiceClassMajor: BluetoothServiceClassMajor,
        inMajorDeviceClass: BluetoothDeviceClassMajor,
        inMinorDeviceClass: BluetoothDeviceClassMinor,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSearchCriteria : inServiceClassMajor, majorDeviceClass : inMajorDeviceClass, minorDeviceClass : inMinorDeviceClass)
    }
    unsafe fn delegate(&self) -> id
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, delegate)
    }
    unsafe fn setDelegate_(&self, delegate: id)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDelegate : delegate)
    }
    unsafe fn inquiryLength(&self) -> u8
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inquiryLength)
    }
    unsafe fn setInquiryLength_(&self, inquiryLength: u8)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInquiryLength : inquiryLength)
    }
    unsafe fn searchType(&self) -> IOBluetoothDeviceSearchTypes
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, searchType)
    }
    unsafe fn setSearchType_(&self, searchType: IOBluetoothDeviceSearchTypes)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSearchType : searchType)
    }
    unsafe fn updateNewDeviceNames(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, updateNewDeviceNames)
    }
    unsafe fn setUpdateNewDeviceNames_(&self, updateNewDeviceNames: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setUpdateNewDeviceNames : updateNewDeviceNames)
    }
    unsafe fn inquiryWithDelegate_(delegate: id) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"IOBluetoothDeviceInquiry").unwrap(), inquiryWithDelegate : delegate)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct IOBluetoothDevicePair(pub id);
impl std::ops::Deref for IOBluetoothDevicePair {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for IOBluetoothDevicePair {}
impl IOBluetoothDevicePair {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"IOBluetoothDevicePair").unwrap(), alloc) })
    }
}
impl INSObject for IOBluetoothDevicePair {}
impl PNSObject for IOBluetoothDevicePair {}
impl std::convert::TryFrom<NSObject> for IOBluetoothDevicePair {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<IOBluetoothDevicePair, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"IOBluetoothDevicePair").unwrap()) };
        if is_kind_of {
            Ok(IOBluetoothDevicePair(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to IOBluetoothDevicePair")
        }
    }
}
impl IIOBluetoothDevicePair for IOBluetoothDevicePair {}
pub trait IIOBluetoothDevicePair: Sized + std::ops::Deref {
    unsafe fn start(&self) -> IOReturn
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
    unsafe fn device(&self) -> IOBluetoothDevice
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, device)
    }
    unsafe fn setDevice_(&self, inDevice: IOBluetoothDevice)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDevice : inDevice)
    }
    unsafe fn replyPINCode_PINCode_(&self, PINCodeSize: ByteCount, PINCode: *mut BluetoothPINCode)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, replyPINCode : PINCodeSize, PINCode : PINCode)
    }
    unsafe fn replyUserConfirmation_(&self, reply: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, replyUserConfirmation : reply)
    }
    unsafe fn delegate(&self) -> id
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, delegate)
    }
    unsafe fn setDelegate_(&self, delegate: id)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDelegate : delegate)
    }
    unsafe fn pairWithDevice_(device: IOBluetoothDevice) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"IOBluetoothDevicePair").unwrap(), pairWithDevice : device)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct IOBluetoothHostController(pub id);
impl std::ops::Deref for IOBluetoothHostController {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for IOBluetoothHostController {}
impl IOBluetoothHostController {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"IOBluetoothHostController").unwrap(), alloc) })
    }
}
impl INSObject for IOBluetoothHostController {}
impl PNSObject for IOBluetoothHostController {}
impl std::convert::TryFrom<NSObject> for IOBluetoothHostController {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<IOBluetoothHostController, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"IOBluetoothHostController").unwrap()) };
        if is_kind_of {
            Ok(IOBluetoothHostController(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to IOBluetoothHostController")
        }
    }
}
impl IIOBluetoothHostController for IOBluetoothHostController {}
pub trait IIOBluetoothHostController: Sized + std::ops::Deref {
    unsafe fn classOfDevice(&self) -> BluetoothClassOfDevice
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, classOfDevice)
    }
    unsafe fn setClassOfDevice_forTimeInterval_(
        &self,
        classOfDevice: BluetoothClassOfDevice,
        seconds: NSTimeInterval,
    ) -> IOReturn
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setClassOfDevice : classOfDevice, forTimeInterval : seconds)
    }
    unsafe fn addressAsString(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, addressAsString)
    }
    unsafe fn nameAsString(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, nameAsString)
    }
    unsafe fn delegate(&self) -> id
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, delegate)
    }
    unsafe fn setDelegate_(&self, delegate: id)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDelegate : delegate)
    }
    unsafe fn powerState(&self) -> BluetoothHCIPowerState
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, powerState)
    }
    unsafe fn defaultController() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"IOBluetoothHostController").unwrap(), defaultController)
    }
}
pub trait NSObject_IOBluetoothHostControllerDelegate: Sized + std::ops::Deref {
    unsafe fn readRSSIForDeviceComplete_device_info_error_(
        &self,
        controller: id,
        device: IOBluetoothDevice,
        info: *mut BluetoothHCIRSSIInfo,
        error: IOReturn,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, readRSSIForDeviceComplete : controller, device : device, info : info, error : error)
    }
    unsafe fn readLinkQualityForDeviceComplete_device_info_error_(
        &self,
        controller: id,
        device: IOBluetoothDevice,
        info: *mut BluetoothHCILinkQualityInfo,
        error: IOReturn,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, readLinkQualityForDeviceComplete : controller, device : device, info : info, error : error)
    }
}
pub type IOBluetoothL2CAPChannelIncomingDataListener = ::std::option::Option<
    unsafe extern "C" fn(
        l2capChannel: IOBluetoothL2CAPChannelRef,
        data: *mut ::std::os::raw::c_void,
        length: UInt16,
        refCon: *mut ::std::os::raw::c_void,
    ),
>;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct IOBluetoothL2CAPChannel(pub id);
impl std::ops::Deref for IOBluetoothL2CAPChannel {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for IOBluetoothL2CAPChannel {}
impl IOBluetoothL2CAPChannel {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"IOBluetoothL2CAPChannel").unwrap(), alloc) })
    }
}
impl PNSPortDelegate for IOBluetoothL2CAPChannel {}
impl IIOBluetoothObject for IOBluetoothL2CAPChannel {}
impl PNSCopying for IOBluetoothL2CAPChannel {}
impl std::convert::TryFrom<IOBluetoothObject> for IOBluetoothL2CAPChannel {
    type Error = &'static str;
    fn try_from(parent: IOBluetoothObject) -> Result<IOBluetoothL2CAPChannel, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"IOBluetoothL2CAPChannel").unwrap()) };
        if is_kind_of {
            Ok(IOBluetoothL2CAPChannel(parent.0))
        } else {
            Err("This IOBluetoothObject cannot be downcasted to IOBluetoothL2CAPChannel")
        }
    }
}
impl INSObject for IOBluetoothL2CAPChannel {}
impl PNSObject for IOBluetoothL2CAPChannel {}
impl IIOBluetoothL2CAPChannel for IOBluetoothL2CAPChannel {}
pub trait IIOBluetoothL2CAPChannel: Sized + std::ops::Deref {
    unsafe fn closeChannel(&self) -> IOReturn
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, closeChannel)
    }
    unsafe fn getOutgoingMTU(&self) -> BluetoothL2CAPMTU
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, getOutgoingMTU)
    }
    unsafe fn getIncomingMTU(&self) -> BluetoothL2CAPMTU
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, getIncomingMTU)
    }
    unsafe fn requestRemoteMTU_(&self, remoteMTU: BluetoothL2CAPMTU) -> IOReturn
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, requestRemoteMTU : remoteMTU)
    }
    unsafe fn writeAsyncTrap_length_refcon_(
        &self,
        data: *mut ::std::os::raw::c_void,
        length: UInt16,
        refcon: *mut ::std::os::raw::c_void,
    ) -> IOReturn
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, writeAsyncTrap : data, length : length, refcon : refcon)
    }
    unsafe fn writeAsync_length_refcon_(
        &self,
        data: *mut ::std::os::raw::c_void,
        length: UInt16,
        refcon: *mut ::std::os::raw::c_void,
    ) -> IOReturn
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, writeAsync : data, length : length, refcon : refcon)
    }
    unsafe fn writeSync_length_(
        &self,
        data: *mut ::std::os::raw::c_void,
        length: UInt16,
    ) -> IOReturn
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, writeSync : data, length : length)
    }
    unsafe fn setDelegate_(&self, channelDelegate: id) -> IOReturn
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDelegate : channelDelegate)
    }
    unsafe fn setDelegate_withConfiguration_(
        &self,
        channelDelegate: id,
        channelConfiguration: NSDictionary,
    ) -> IOReturn
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDelegate : channelDelegate, withConfiguration : channelConfiguration)
    }
    unsafe fn delegate(&self) -> id
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, delegate)
    }
    unsafe fn getDevice(&self) -> IOBluetoothDevice
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, getDevice)
    }
    unsafe fn getObjectID(&self) -> IOBluetoothObjectID
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, getObjectID)
    }
    unsafe fn getPSM(&self) -> BluetoothL2CAPPSM
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, getPSM)
    }
    unsafe fn getLocalChannelID(&self) -> BluetoothL2CAPChannelID
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, getLocalChannelID)
    }
    unsafe fn getRemoteChannelID(&self) -> BluetoothL2CAPChannelID
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, getRemoteChannelID)
    }
    unsafe fn isIncoming(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isIncoming)
    }
    unsafe fn registerForChannelCloseNotification_selector_(
        &self,
        observer: id,
        inSelector: objc2::runtime::Sel,
    ) -> IOBluetoothUserNotification
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, registerForChannelCloseNotification : observer, selector : inSelector)
    }
    unsafe fn outgoingMTU(&self) -> BluetoothL2CAPMTU
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, outgoingMTU)
    }
    unsafe fn incomingMTU(&self) -> BluetoothL2CAPMTU
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, incomingMTU)
    }
    unsafe fn device(&self) -> IOBluetoothDevice
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, device)
    }
    unsafe fn objectID(&self) -> IOBluetoothObjectID
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, objectID)
    }
    unsafe fn PSM(&self) -> BluetoothL2CAPPSM
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, PSM)
    }
    unsafe fn localChannelID(&self) -> BluetoothL2CAPChannelID
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, localChannelID)
    }
    unsafe fn remoteChannelID(&self) -> BluetoothL2CAPChannelID
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, remoteChannelID)
    }
    unsafe fn registerForChannelOpenNotifications_selector_(
        object: id,
        selector: objc2::runtime::Sel,
    ) -> IOBluetoothUserNotification
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"IOBluetoothL2CAPChannel").unwrap(), registerForChannelOpenNotifications : object, selector : selector)
    }
    unsafe fn registerForChannelOpenNotifications_selector_withPSM_direction_(
        object: id,
        selector: objc2::runtime::Sel,
        psm: BluetoothL2CAPPSM,
        inDirection: IOBluetoothUserNotificationChannelDirection,
    ) -> IOBluetoothUserNotification
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"IOBluetoothL2CAPChannel").unwrap(), registerForChannelOpenNotifications : object, selector : selector, withPSM : psm, direction : inDirection)
    }
    unsafe fn withObjectID_(objectID: IOBluetoothObjectID) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"IOBluetoothL2CAPChannel").unwrap(), withObjectID : objectID)
    }
}
pub trait NSObject_IOBluetoothL2CAPChannelDeprecated: Sized + std::ops::Deref {
    unsafe fn registerIncomingDataListener_refCon_(
        &self,
        listener: IOBluetoothL2CAPChannelIncomingDataListener,
        refCon: *mut ::std::os::raw::c_void,
    ) -> IOReturn
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, registerIncomingDataListener : listener, refCon : refCon)
    }
    unsafe fn write_length_(&self, data: *mut ::std::os::raw::c_void, length: UInt16) -> IOReturn
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, write : data, length : length)
    }
    unsafe fn getL2CAPChannelRef(&self) -> IOBluetoothL2CAPChannelRef
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, getL2CAPChannelRef)
    }
    unsafe fn withL2CAPChannelRef_(
        l2capChannelRef: IOBluetoothL2CAPChannelRef,
    ) -> IOBluetoothL2CAPChannel
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NSObject").unwrap(), withL2CAPChannelRef : l2capChannelRef)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct IOBluetoothRFCOMMChannel(pub id);
impl std::ops::Deref for IOBluetoothRFCOMMChannel {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for IOBluetoothRFCOMMChannel {}
impl IOBluetoothRFCOMMChannel {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"IOBluetoothRFCOMMChannel").unwrap(), alloc) })
    }
}
impl PNSPortDelegate for IOBluetoothRFCOMMChannel {}
impl PNSStreamDelegate for IOBluetoothRFCOMMChannel {}
impl IIOBluetoothObject for IOBluetoothRFCOMMChannel {}
impl PNSCopying for IOBluetoothRFCOMMChannel {}
impl std::convert::TryFrom<IOBluetoothObject> for IOBluetoothRFCOMMChannel {
    type Error = &'static str;
    fn try_from(parent: IOBluetoothObject) -> Result<IOBluetoothRFCOMMChannel, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"IOBluetoothRFCOMMChannel").unwrap()) };
        if is_kind_of {
            Ok(IOBluetoothRFCOMMChannel(parent.0))
        } else {
            Err("This IOBluetoothObject cannot be downcasted to IOBluetoothRFCOMMChannel")
        }
    }
}
impl INSObject for IOBluetoothRFCOMMChannel {}
impl PNSObject for IOBluetoothRFCOMMChannel {}
impl IIOBluetoothRFCOMMChannel for IOBluetoothRFCOMMChannel {}
pub trait IIOBluetoothRFCOMMChannel: Sized + std::ops::Deref {
    unsafe fn getRFCOMMChannelRef(&self) -> IOBluetoothRFCOMMChannelRef
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, getRFCOMMChannelRef)
    }
    unsafe fn closeChannel(&self) -> IOReturn
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, closeChannel)
    }
    unsafe fn isOpen(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isOpen)
    }
    unsafe fn getMTU(&self) -> BluetoothRFCOMMMTU
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, getMTU)
    }
    unsafe fn isTransmissionPaused(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isTransmissionPaused)
    }
    unsafe fn write_length_sleep_(
        &self,
        data: *mut ::std::os::raw::c_void,
        length: UInt16,
        sleep: BOOL,
    ) -> IOReturn
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, write : data, length : length, sleep : sleep)
    }
    unsafe fn writeAsync_length_refcon_(
        &self,
        data: *mut ::std::os::raw::c_void,
        length: UInt16,
        refcon: *mut ::std::os::raw::c_void,
    ) -> IOReturn
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, writeAsync : data, length : length, refcon : refcon)
    }
    unsafe fn writeSync_length_(
        &self,
        data: *mut ::std::os::raw::c_void,
        length: UInt16,
    ) -> IOReturn
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, writeSync : data, length : length)
    }
    unsafe fn writeSimple_length_sleep_bytesSent_(
        &self,
        data: *mut ::std::os::raw::c_void,
        length: UInt16,
        sleep: BOOL,
        numBytesSent: *mut UInt32,
    ) -> IOReturn
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, writeSimple : data, length : length, sleep : sleep, bytesSent : numBytesSent)
    }
    unsafe fn setSerialParameters_dataBits_parity_stopBits_(
        &self,
        speed: UInt32,
        nBits: UInt8,
        parity: BluetoothRFCOMMParityType,
        bitStop: UInt8,
    ) -> IOReturn
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSerialParameters : speed, dataBits : nBits, parity : parity, stopBits : bitStop)
    }
    unsafe fn sendRemoteLineStatus_(&self, lineStatus: BluetoothRFCOMMLineStatus) -> IOReturn
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, sendRemoteLineStatus : lineStatus)
    }
    unsafe fn setDelegate_(&self, delegate: id) -> IOReturn
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDelegate : delegate)
    }
    unsafe fn delegate(&self) -> id
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, delegate)
    }
    unsafe fn getChannelID(&self) -> BluetoothRFCOMMChannelID
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, getChannelID)
    }
    unsafe fn isIncoming(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isIncoming)
    }
    unsafe fn getDevice(&self) -> IOBluetoothDevice
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, getDevice)
    }
    unsafe fn getObjectID(&self) -> IOBluetoothObjectID
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, getObjectID)
    }
    unsafe fn registerForChannelCloseNotification_selector_(
        &self,
        observer: id,
        inSelector: objc2::runtime::Sel,
    ) -> IOBluetoothUserNotification
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, registerForChannelCloseNotification : observer, selector : inSelector)
    }
    unsafe fn registerForChannelOpenNotifications_selector_(
        object: id,
        selector: objc2::runtime::Sel,
    ) -> IOBluetoothUserNotification
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"IOBluetoothRFCOMMChannel").unwrap(), registerForChannelOpenNotifications : object, selector : selector)
    }
    unsafe fn registerForChannelOpenNotifications_selector_withChannelID_direction_(
        object: id,
        selector: objc2::runtime::Sel,
        channelID: BluetoothRFCOMMChannelID,
        inDirection: IOBluetoothUserNotificationChannelDirection,
    ) -> IOBluetoothUserNotification
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"IOBluetoothRFCOMMChannel").unwrap(), registerForChannelOpenNotifications : object, selector : selector, withChannelID : channelID, direction : inDirection)
    }
    unsafe fn withRFCOMMChannelRef_(rfcommChannelRef: IOBluetoothRFCOMMChannelRef) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"IOBluetoothRFCOMMChannel").unwrap(), withRFCOMMChannelRef : rfcommChannelRef)
    }
    unsafe fn withObjectID_(objectID: IOBluetoothObjectID) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"IOBluetoothRFCOMMChannel").unwrap(), withObjectID : objectID)
    }
}
pub trait PIOBluetoothRFCOMMChannelDelegate: Sized + std::ops::Deref {
    unsafe fn rfcommChannelData_data_length_(
        &self,
        rfcommChannel: IOBluetoothRFCOMMChannel,
        dataPointer: *mut ::std::os::raw::c_void,
        dataLength: usize,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, rfcommChannelData : rfcommChannel, data : dataPointer, length : dataLength)
    }
    unsafe fn rfcommChannelOpenComplete_status_(
        &self,
        rfcommChannel: IOBluetoothRFCOMMChannel,
        error: IOReturn,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, rfcommChannelOpenComplete : rfcommChannel, status : error)
    }
    unsafe fn rfcommChannelClosed_(&self, rfcommChannel: IOBluetoothRFCOMMChannel)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, rfcommChannelClosed : rfcommChannel)
    }
    unsafe fn rfcommChannelControlSignalsChanged_(&self, rfcommChannel: IOBluetoothRFCOMMChannel)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, rfcommChannelControlSignalsChanged : rfcommChannel)
    }
    unsafe fn rfcommChannelFlowControlChanged_(&self, rfcommChannel: IOBluetoothRFCOMMChannel)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, rfcommChannelFlowControlChanged : rfcommChannel)
    }
    unsafe fn rfcommChannelWriteComplete_refcon_status_(
        &self,
        rfcommChannel: IOBluetoothRFCOMMChannel,
        refcon: *mut ::std::os::raw::c_void,
        error: IOReturn,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, rfcommChannelWriteComplete : rfcommChannel, refcon : refcon, status : error)
    }
    unsafe fn rfcommChannelWriteComplete_refcon_status_bytesWritten_(
        &self,
        rfcommChannel: IOBluetoothRFCOMMChannel,
        refcon: *mut ::std::os::raw::c_void,
        error: IOReturn,
        length: usize,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, rfcommChannelWriteComplete : rfcommChannel, refcon : refcon, status : error, bytesWritten : length)
    }
    unsafe fn rfcommChannelQueueSpaceAvailable_(&self, rfcommChannel: IOBluetoothRFCOMMChannel)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, rfcommChannelQueueSpaceAvailable : rfcommChannel)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct IOBluetoothSDPDataElement(pub id);
impl std::ops::Deref for IOBluetoothSDPDataElement {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for IOBluetoothSDPDataElement {}
impl IOBluetoothSDPDataElement {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"IOBluetoothSDPDataElement").unwrap(), alloc) })
    }
}
impl PNSCoding for IOBluetoothSDPDataElement {}
impl PNSSecureCoding for IOBluetoothSDPDataElement {}
impl INSObject for IOBluetoothSDPDataElement {}
impl PNSObject for IOBluetoothSDPDataElement {}
impl std::convert::TryFrom<NSObject> for IOBluetoothSDPDataElement {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<IOBluetoothSDPDataElement, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"IOBluetoothSDPDataElement").unwrap()) };
        if is_kind_of {
            Ok(IOBluetoothSDPDataElement(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to IOBluetoothSDPDataElement")
        }
    }
}
impl IIOBluetoothSDPDataElement for IOBluetoothSDPDataElement {}
pub trait IIOBluetoothSDPDataElement: Sized + std::ops::Deref {
    unsafe fn initWithElementValue_(&self, element: NSObject) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithElementValue : element)
    }
    unsafe fn initWithType_sizeDescriptor_size_value_(
        &self,
        newType: BluetoothSDPDataElementTypeDescriptor,
        newSizeDescriptor: BluetoothSDPDataElementSizeDescriptor,
        newSize: u32,
        newValue: NSObject,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithType : newType, sizeDescriptor : newSizeDescriptor, size : newSize, value : newValue)
    }
    unsafe fn getSDPDataElementRef(&self) -> IOBluetoothSDPDataElementRef
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, getSDPDataElementRef)
    }
    unsafe fn getTypeDescriptor(&self) -> BluetoothSDPDataElementTypeDescriptor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, getTypeDescriptor)
    }
    unsafe fn getSizeDescriptor(&self) -> BluetoothSDPDataElementSizeDescriptor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, getSizeDescriptor)
    }
    unsafe fn getSize(&self) -> u32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, getSize)
    }
    unsafe fn getNumberValue(&self) -> NSNumber
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, getNumberValue)
    }
    unsafe fn getDataValue(&self) -> NSData
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, getDataValue)
    }
    unsafe fn getStringValue(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, getStringValue)
    }
    unsafe fn getArrayValue(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, getArrayValue)
    }
    unsafe fn getUUIDValue(&self) -> IOBluetoothSDPUUID
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, getUUIDValue)
    }
    unsafe fn getValue(&self) -> NSObject
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, getValue)
    }
    unsafe fn containsDataElement_(&self, dataElement: IOBluetoothSDPDataElement) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, containsDataElement : dataElement)
    }
    unsafe fn containsValue_(&self, cmpValue: NSObject) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, containsValue : cmpValue)
    }
    unsafe fn withElementValue_(element: NSObject) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"IOBluetoothSDPDataElement").unwrap(), withElementValue : element)
    }
    unsafe fn withType_sizeDescriptor_size_value_(
        type_: BluetoothSDPDataElementTypeDescriptor,
        newSizeDescriptor: BluetoothSDPDataElementSizeDescriptor,
        newSize: u32,
        newValue: NSObject,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"IOBluetoothSDPDataElement").unwrap(), withType : type_, sizeDescriptor : newSizeDescriptor, size : newSize, value : newValue)
    }
    unsafe fn withSDPDataElementRef_(
        sdpDataElementRef: IOBluetoothSDPDataElementRef,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"IOBluetoothSDPDataElement").unwrap(), withSDPDataElementRef : sdpDataElementRef)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct IOBluetoothSDPServiceAttribute(pub id);
impl std::ops::Deref for IOBluetoothSDPServiceAttribute {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for IOBluetoothSDPServiceAttribute {}
impl IOBluetoothSDPServiceAttribute {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"IOBluetoothSDPServiceAttribute").unwrap(), alloc) })
    }
}
impl PNSCoding for IOBluetoothSDPServiceAttribute {}
impl PNSSecureCoding for IOBluetoothSDPServiceAttribute {}
impl INSObject for IOBluetoothSDPServiceAttribute {}
impl PNSObject for IOBluetoothSDPServiceAttribute {}
impl std::convert::TryFrom<NSObject> for IOBluetoothSDPServiceAttribute {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<IOBluetoothSDPServiceAttribute, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"IOBluetoothSDPServiceAttribute").unwrap())
        };
        if is_kind_of {
            Ok(IOBluetoothSDPServiceAttribute(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to IOBluetoothSDPServiceAttribute")
        }
    }
}
impl IIOBluetoothSDPServiceAttribute for IOBluetoothSDPServiceAttribute {}
pub trait IIOBluetoothSDPServiceAttribute: Sized + std::ops::Deref {
    unsafe fn initWithID_attributeElementValue_(
        &self,
        newAttributeID: BluetoothSDPServiceAttributeID,
        attributeElementValue: NSObject,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithID : newAttributeID, attributeElementValue : attributeElementValue)
    }
    unsafe fn initWithID_attributeElement_(
        &self,
        newAttributeID: BluetoothSDPServiceAttributeID,
        attributeElement: IOBluetoothSDPDataElement,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithID : newAttributeID, attributeElement : attributeElement)
    }
    unsafe fn getAttributeID(&self) -> BluetoothSDPServiceAttributeID
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, getAttributeID)
    }
    unsafe fn getDataElement(&self) -> IOBluetoothSDPDataElement
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, getDataElement)
    }
    unsafe fn getIDDataElement(&self) -> IOBluetoothSDPDataElement
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, getIDDataElement)
    }
    unsafe fn withID_attributeElementValue_(
        newAttributeID: BluetoothSDPServiceAttributeID,
        attributeElementValue: NSObject,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"IOBluetoothSDPServiceAttribute").unwrap(), withID : newAttributeID, attributeElementValue : attributeElementValue)
    }
    unsafe fn withID_attributeElement_(
        newAttributeID: BluetoothSDPServiceAttributeID,
        attributeElement: IOBluetoothSDPDataElement,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"IOBluetoothSDPServiceAttribute").unwrap(), withID : newAttributeID, attributeElement : attributeElement)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct IOBluetoothSDPServiceRecord(pub id);
impl std::ops::Deref for IOBluetoothSDPServiceRecord {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for IOBluetoothSDPServiceRecord {}
impl IOBluetoothSDPServiceRecord {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"IOBluetoothSDPServiceRecord").unwrap(), alloc) })
    }
}
impl PNSCoding for IOBluetoothSDPServiceRecord {}
impl PNSSecureCoding for IOBluetoothSDPServiceRecord {}
impl INSObject for IOBluetoothSDPServiceRecord {}
impl PNSObject for IOBluetoothSDPServiceRecord {}
impl std::convert::TryFrom<NSObject> for IOBluetoothSDPServiceRecord {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<IOBluetoothSDPServiceRecord, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"IOBluetoothSDPServiceRecord").unwrap()) };
        if is_kind_of {
            Ok(IOBluetoothSDPServiceRecord(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to IOBluetoothSDPServiceRecord")
        }
    }
}
impl IIOBluetoothSDPServiceRecord for IOBluetoothSDPServiceRecord {}
pub trait IIOBluetoothSDPServiceRecord: Sized + std::ops::Deref {
    unsafe fn removeServiceRecord(&self) -> IOReturn
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, removeServiceRecord)
    }
    unsafe fn initWithServiceDictionary_device_(
        &self,
        serviceDict: NSDictionary,
        device: IOBluetoothDevice,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithServiceDictionary : serviceDict, device : device)
    }
    unsafe fn getSDPServiceRecordRef(&self) -> IOBluetoothSDPServiceRecordRef
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, getSDPServiceRecordRef)
    }
    unsafe fn getDevice(&self) -> IOBluetoothDevice
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, getDevice)
    }
    unsafe fn getAttributes(&self) -> NSDictionary
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, getAttributes)
    }
    unsafe fn getAttributeDataElement_(
        &self,
        attributeID: BluetoothSDPServiceAttributeID,
    ) -> IOBluetoothSDPDataElement
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, getAttributeDataElement : attributeID)
    }
    unsafe fn getServiceName(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, getServiceName)
    }
    unsafe fn getRFCOMMChannelID_(&self, rfcommChannelID: *mut BluetoothRFCOMMChannelID) -> IOReturn
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, getRFCOMMChannelID : rfcommChannelID)
    }
    unsafe fn getL2CAPPSM_(&self, outPSM: *mut BluetoothL2CAPPSM) -> IOReturn
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, getL2CAPPSM : outPSM)
    }
    unsafe fn getServiceRecordHandle_(
        &self,
        outServiceRecordHandle: *mut BluetoothSDPServiceRecordHandle,
    ) -> IOReturn
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, getServiceRecordHandle : outServiceRecordHandle)
    }
    unsafe fn matchesUUID16_(&self, uuid16: BluetoothSDPUUID16) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, matchesUUID16 : uuid16)
    }
    unsafe fn matchesUUIDArray_(&self, uuidArray: NSArray) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, matchesUUIDArray : uuidArray)
    }
    unsafe fn matchesSearchArray_(&self, searchArray: NSArray) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, matchesSearchArray : searchArray)
    }
    unsafe fn hasServiceFromArray_(&self, array: NSArray) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, hasServiceFromArray : array)
    }
    unsafe fn device(&self) -> IOBluetoothDevice
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, device)
    }
    unsafe fn attributes(&self) -> NSDictionary
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, attributes)
    }
    unsafe fn sortedAttributes(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, sortedAttributes)
    }
    unsafe fn publishedServiceRecordWithDictionary_(serviceDict: NSDictionary) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"IOBluetoothSDPServiceRecord").unwrap(), publishedServiceRecordWithDictionary : serviceDict)
    }
    unsafe fn withServiceDictionary_device_(
        serviceDict: NSDictionary,
        device: IOBluetoothDevice,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"IOBluetoothSDPServiceRecord").unwrap(), withServiceDictionary : serviceDict, device : device)
    }
    unsafe fn withSDPServiceRecordRef_(
        sdpServiceRecordRef: IOBluetoothSDPServiceRecordRef,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"IOBluetoothSDPServiceRecord").unwrap(), withSDPServiceRecordRef : sdpServiceRecordRef)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct IOBluetoothSDPUUID(pub id);
impl std::ops::Deref for IOBluetoothSDPUUID {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for IOBluetoothSDPUUID {}
impl IOBluetoothSDPUUID {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"IOBluetoothSDPUUID").unwrap(), alloc) })
    }
}
impl INSData for IOBluetoothSDPUUID {}
impl PNSCopying for IOBluetoothSDPUUID {}
impl PNSMutableCopying for IOBluetoothSDPUUID {}
impl PNSSecureCoding for IOBluetoothSDPUUID {}
impl std::convert::TryFrom<NSData> for IOBluetoothSDPUUID {
    type Error = &'static str;
    fn try_from(parent: NSData) -> Result<IOBluetoothSDPUUID, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"IOBluetoothSDPUUID").unwrap()) };
        if is_kind_of {
            Ok(IOBluetoothSDPUUID(parent.0))
        } else {
            Err("This NSData cannot be downcasted to IOBluetoothSDPUUID")
        }
    }
}
impl INSObject for IOBluetoothSDPUUID {}
impl PNSObject for IOBluetoothSDPUUID {}
impl IIOBluetoothSDPUUID for IOBluetoothSDPUUID {}
pub trait IIOBluetoothSDPUUID: Sized + std::ops::Deref {
    unsafe fn initWithUUID16_(&self, uuid16: BluetoothSDPUUID16) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithUUID16 : uuid16)
    }
    unsafe fn initWithUUID32_(&self, uuid32: BluetoothSDPUUID32) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithUUID32 : uuid32)
    }
    unsafe fn getSDPUUIDRef(&self) -> IOBluetoothSDPUUIDRef
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, getSDPUUIDRef)
    }
    unsafe fn getUUIDWithLength_(&self, newLength: ::std::os::raw::c_uint) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, getUUIDWithLength : newLength)
    }
    unsafe fn isEqualToUUID_(&self, otherUUID: IOBluetoothSDPUUID) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, isEqualToUUID : otherUUID)
    }
    unsafe fn classForCoder(&self) -> Class
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, classForCoder)
    }
    unsafe fn classForArchiver(&self) -> Class
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, classForArchiver)
    }
    unsafe fn classForPortCoder(&self) -> Class
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, classForPortCoder)
    }
    unsafe fn uuidWithBytes_length_(
        bytes: *const ::std::os::raw::c_void,
        length: ::std::os::raw::c_uint,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"IOBluetoothSDPUUID").unwrap(), uuidWithBytes : bytes, length : length)
    }
    unsafe fn uuidWithData_(data: NSData) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"IOBluetoothSDPUUID").unwrap(), uuidWithData : data)
    }
    unsafe fn uuid16_(uuid16: BluetoothSDPUUID16) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"IOBluetoothSDPUUID").unwrap(), uuid16 : uuid16)
    }
    unsafe fn uuid32_(uuid32: BluetoothSDPUUID32) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"IOBluetoothSDPUUID").unwrap(), uuid32 : uuid32)
    }
    unsafe fn withSDPUUIDRef_(sdpUUIDRef: IOBluetoothSDPUUIDRef) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"IOBluetoothSDPUUID").unwrap(), withSDPUUIDRef : sdpUUIDRef)
    }
}
pub type OBEXError = i32;
pub type OBEXHeaderIdentifier = u8;
pub type OBEXVersion = u8;
pub type OBEXFlags = u8;
pub type OBEXOpCode = u8;
pub type OBEXConstants = u8;
pub type OBEXMaxPacketLength = u16;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OpaqueOBEXSessionRef {
    _unused: [u8; 0],
}
pub type OBEXSessionRef = *mut OpaqueOBEXSessionRef;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OBEXConnectCommandResponseData {
    pub serverResponseOpCode: OBEXOpCode,
    pub headerDataPtr: *mut ::std::os::raw::c_void,
    pub headerDataLength: usize,
    pub maxPacketSize: OBEXMaxPacketLength,
    pub version: OBEXVersion,
    pub flags: OBEXFlags,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OBEXDisconnectCommandResponseData {
    pub serverResponseOpCode: OBEXOpCode,
    pub headerDataPtr: *mut ::std::os::raw::c_void,
    pub headerDataLength: usize,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OBEXPutCommandResponseData {
    pub serverResponseOpCode: OBEXOpCode,
    pub headerDataPtr: *mut ::std::os::raw::c_void,
    pub headerDataLength: usize,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OBEXGetCommandResponseData {
    pub serverResponseOpCode: OBEXOpCode,
    pub headerDataPtr: *mut ::std::os::raw::c_void,
    pub headerDataLength: usize,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OBEXSetPathCommandResponseData {
    pub serverResponseOpCode: OBEXOpCode,
    pub headerDataPtr: *mut ::std::os::raw::c_void,
    pub headerDataLength: usize,
    pub flags: OBEXFlags,
    pub constants: OBEXConstants,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OBEXAbortCommandResponseData {
    pub serverResponseOpCode: OBEXOpCode,
    pub headerDataPtr: *mut ::std::os::raw::c_void,
    pub headerDataLength: usize,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OBEXConnectCommandData {
    pub headerDataPtr: *mut ::std::os::raw::c_void,
    pub headerDataLength: usize,
    pub maxPacketSize: OBEXMaxPacketLength,
    pub version: OBEXVersion,
    pub flags: OBEXFlags,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OBEXDisconnectCommandData {
    pub headerDataPtr: *mut ::std::os::raw::c_void,
    pub headerDataLength: usize,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OBEXPutCommandData {
    pub headerDataPtr: *mut ::std::os::raw::c_void,
    pub headerDataLength: usize,
    pub bodyDataLeftToSend: usize,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OBEXGetCommandData {
    pub headerDataPtr: *mut ::std::os::raw::c_void,
    pub headerDataLength: usize,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OBEXSetPathCommandData {
    pub headerDataPtr: *mut ::std::os::raw::c_void,
    pub headerDataLength: usize,
    pub flags: OBEXFlags,
    pub constants: OBEXConstants,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OBEXAbortCommandData {
    pub headerDataPtr: *mut ::std::os::raw::c_void,
    pub headerDataLength: usize,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OBEXErrorData {
    pub error: OBEXError,
    pub dataPtr: *mut ::std::os::raw::c_void,
    pub dataLength: usize,
}
pub type OBEXSessionEventType = u32;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct OBEXSessionEvent {
    pub __bindgen_anon_1: OBEXSessionEvent__bindgen_ty_1,
    pub type_: OBEXSessionEventType,
    pub session: OBEXSessionRef,
    pub refCon: *mut ::std::os::raw::c_void,
    pub isEndOfEventData: Boolean,
    pub reserved1: *mut ::std::os::raw::c_void,
    pub reserved2: *mut ::std::os::raw::c_void,
    pub u: OBEXSessionEvent__bindgen_ty_1,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union OBEXSessionEvent__bindgen_ty_1 {
    pub connectCommandResponseData: OBEXConnectCommandResponseData,
    pub disconnectCommandResponseData: OBEXDisconnectCommandResponseData,
    pub putCommandResponseData: OBEXPutCommandResponseData,
    pub getCommandResponseData: OBEXGetCommandResponseData,
    pub setPathCommandResponseData: OBEXSetPathCommandResponseData,
    pub abortCommandResponseData: OBEXAbortCommandResponseData,
    pub connectCommandData: OBEXConnectCommandData,
    pub disconnectCommandData: OBEXDisconnectCommandData,
    pub putCommandData: OBEXPutCommandData,
    pub getCommandData: OBEXGetCommandData,
    pub setPathCommandData: OBEXSetPathCommandData,
    pub abortCommandData: OBEXAbortCommandData,
    pub errorData: OBEXErrorData,
}
pub type OBEXSessionEventCallback =
    ::std::option::Option<unsafe extern "C" fn(inEvent: *const OBEXSessionEvent)>;
pub type IOBluetoothOBEXSessionOpenConnectionCallback = ::std::option::Option<
    unsafe extern "C" fn(
        session: OBEXSessionRef,
        status: OBEXError,
        refCon: *mut ::std::os::raw::c_void,
    ),
>;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct OBEXSession(pub id);
impl std::ops::Deref for OBEXSession {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for OBEXSession {}
impl OBEXSession {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"OBEXSession").unwrap(), alloc) })
    }
}
impl INSObject for OBEXSession {}
impl PNSObject for OBEXSession {}
impl std::convert::TryFrom<NSObject> for OBEXSession {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<OBEXSession, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"OBEXSession").unwrap()) };
        if is_kind_of {
            Ok(OBEXSession(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to OBEXSession")
        }
    }
}
impl IOBEXSession for OBEXSession {}
pub trait IOBEXSession: Sized + std::ops::Deref {
    unsafe fn OBEXConnect_maxPacketLength_optionalHeaders_optionalHeadersLength_eventSelector_selectorTarget_refCon_(
        &self,
        inFlags: OBEXFlags,
        inMaxPacketLength: OBEXMaxPacketLength,
        inOptionalHeaders: *mut ::std::os::raw::c_void,
        inOptionalHeadersLength: usize,
        inSelector: objc2::runtime::Sel,
        inTarget: id,
        inUserRefCon: *mut ::std::os::raw::c_void,
    ) -> OBEXError
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, OBEXConnect : inFlags, maxPacketLength : inMaxPacketLength, optionalHeaders : inOptionalHeaders, optionalHeadersLength : inOptionalHeadersLength, eventSelector : inSelector, selectorTarget : inTarget, refCon : inUserRefCon)
    }
    unsafe fn OBEXDisconnect_optionalHeadersLength_eventSelector_selectorTarget_refCon_(
        &self,
        inOptionalHeaders: *mut ::std::os::raw::c_void,
        inOptionalHeadersLength: usize,
        inSelector: objc2::runtime::Sel,
        inTarget: id,
        inUserRefCon: *mut ::std::os::raw::c_void,
    ) -> OBEXError
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, OBEXDisconnect : inOptionalHeaders, optionalHeadersLength : inOptionalHeadersLength, eventSelector : inSelector, selectorTarget : inTarget, refCon : inUserRefCon)
    }
    unsafe fn OBEXPut_headersData_headersDataLength_bodyData_bodyDataLength_eventSelector_selectorTarget_refCon_(
        &self,
        isFinalChunk: Boolean,
        inHeadersData: *mut ::std::os::raw::c_void,
        inHeadersDataLength: usize,
        inBodyData: *mut ::std::os::raw::c_void,
        inBodyDataLength: usize,
        inSelector: objc2::runtime::Sel,
        inTarget: id,
        inUserRefCon: *mut ::std::os::raw::c_void,
    ) -> OBEXError
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, OBEXPut : isFinalChunk, headersData : inHeadersData, headersDataLength : inHeadersDataLength, bodyData : inBodyData, bodyDataLength : inBodyDataLength, eventSelector : inSelector, selectorTarget : inTarget, refCon : inUserRefCon)
    }
    unsafe fn OBEXGet_headers_headersLength_eventSelector_selectorTarget_refCon_(
        &self,
        isFinalChunk: Boolean,
        inHeaders: *mut ::std::os::raw::c_void,
        inHeadersLength: usize,
        inSelector: objc2::runtime::Sel,
        inTarget: id,
        inUserRefCon: *mut ::std::os::raw::c_void,
    ) -> OBEXError
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, OBEXGet : isFinalChunk, headers : inHeaders, headersLength : inHeadersLength, eventSelector : inSelector, selectorTarget : inTarget, refCon : inUserRefCon)
    }
    unsafe fn OBEXAbort_optionalHeadersLength_eventSelector_selectorTarget_refCon_(
        &self,
        inOptionalHeaders: *mut ::std::os::raw::c_void,
        inOptionalHeadersLength: usize,
        inSelector: objc2::runtime::Sel,
        inTarget: id,
        inUserRefCon: *mut ::std::os::raw::c_void,
    ) -> OBEXError
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, OBEXAbort : inOptionalHeaders, optionalHeadersLength : inOptionalHeadersLength, eventSelector : inSelector, selectorTarget : inTarget, refCon : inUserRefCon)
    }
    unsafe fn OBEXSetPath_constants_optionalHeaders_optionalHeadersLength_eventSelector_selectorTarget_refCon_(
        &self,
        inFlags: OBEXFlags,
        inConstants: OBEXConstants,
        inOptionalHeaders: *mut ::std::os::raw::c_void,
        inOptionalHeadersLength: usize,
        inSelector: objc2::runtime::Sel,
        inTarget: id,
        inUserRefCon: *mut ::std::os::raw::c_void,
    ) -> OBEXError
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, OBEXSetPath : inFlags, constants : inConstants, optionalHeaders : inOptionalHeaders, optionalHeadersLength : inOptionalHeadersLength, eventSelector : inSelector, selectorTarget : inTarget, refCon : inUserRefCon)
    }
    unsafe fn OBEXConnectResponse_flags_maxPacketLength_optionalHeaders_optionalHeadersLength_eventSelector_selectorTarget_refCon_(
        &self,
        inResponseOpCode: OBEXOpCode,
        inFlags: OBEXFlags,
        inMaxPacketLength: OBEXMaxPacketLength,
        inOptionalHeaders: *mut ::std::os::raw::c_void,
        inOptionalHeadersLength: usize,
        inSelector: objc2::runtime::Sel,
        inTarget: id,
        inUserRefCon: *mut ::std::os::raw::c_void,
    ) -> OBEXError
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, OBEXConnectResponse : inResponseOpCode, flags : inFlags, maxPacketLength : inMaxPacketLength, optionalHeaders : inOptionalHeaders, optionalHeadersLength : inOptionalHeadersLength, eventSelector : inSelector, selectorTarget : inTarget, refCon : inUserRefCon)
    }
    unsafe fn OBEXDisconnectResponse_optionalHeaders_optionalHeadersLength_eventSelector_selectorTarget_refCon_(
        &self,
        inResponseOpCode: OBEXOpCode,
        inOptionalHeaders: *mut ::std::os::raw::c_void,
        inOptionalHeadersLength: usize,
        inSelector: objc2::runtime::Sel,
        inTarget: id,
        inUserRefCon: *mut ::std::os::raw::c_void,
    ) -> OBEXError
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, OBEXDisconnectResponse : inResponseOpCode, optionalHeaders : inOptionalHeaders, optionalHeadersLength : inOptionalHeadersLength, eventSelector : inSelector, selectorTarget : inTarget, refCon : inUserRefCon)
    }
    unsafe fn OBEXPutResponse_optionalHeaders_optionalHeadersLength_eventSelector_selectorTarget_refCon_(
        &self,
        inResponseOpCode: OBEXOpCode,
        inOptionalHeaders: *mut ::std::os::raw::c_void,
        inOptionalHeadersLength: usize,
        inSelector: objc2::runtime::Sel,
        inTarget: id,
        inUserRefCon: *mut ::std::os::raw::c_void,
    ) -> OBEXError
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, OBEXPutResponse : inResponseOpCode, optionalHeaders : inOptionalHeaders, optionalHeadersLength : inOptionalHeadersLength, eventSelector : inSelector, selectorTarget : inTarget, refCon : inUserRefCon)
    }
    unsafe fn OBEXGetResponse_optionalHeaders_optionalHeadersLength_eventSelector_selectorTarget_refCon_(
        &self,
        inResponseOpCode: OBEXOpCode,
        inOptionalHeaders: *mut ::std::os::raw::c_void,
        inOptionalHeadersLength: usize,
        inSelector: objc2::runtime::Sel,
        inTarget: id,
        inUserRefCon: *mut ::std::os::raw::c_void,
    ) -> OBEXError
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, OBEXGetResponse : inResponseOpCode, optionalHeaders : inOptionalHeaders, optionalHeadersLength : inOptionalHeadersLength, eventSelector : inSelector, selectorTarget : inTarget, refCon : inUserRefCon)
    }
    unsafe fn OBEXAbortResponse_optionalHeaders_optionalHeadersLength_eventSelector_selectorTarget_refCon_(
        &self,
        inResponseOpCode: OBEXOpCode,
        inOptionalHeaders: *mut ::std::os::raw::c_void,
        inOptionalHeadersLength: usize,
        inSelector: objc2::runtime::Sel,
        inTarget: id,
        inUserRefCon: *mut ::std::os::raw::c_void,
    ) -> OBEXError
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, OBEXAbortResponse : inResponseOpCode, optionalHeaders : inOptionalHeaders, optionalHeadersLength : inOptionalHeadersLength, eventSelector : inSelector, selectorTarget : inTarget, refCon : inUserRefCon)
    }
    unsafe fn OBEXSetPathResponse_optionalHeaders_optionalHeadersLength_eventSelector_selectorTarget_refCon_(
        &self,
        inResponseOpCode: OBEXOpCode,
        inOptionalHeaders: *mut ::std::os::raw::c_void,
        inOptionalHeadersLength: usize,
        inSelector: objc2::runtime::Sel,
        inTarget: id,
        inUserRefCon: *mut ::std::os::raw::c_void,
    ) -> OBEXError
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, OBEXSetPathResponse : inResponseOpCode, optionalHeaders : inOptionalHeaders, optionalHeadersLength : inOptionalHeadersLength, eventSelector : inSelector, selectorTarget : inTarget, refCon : inUserRefCon)
    }
    unsafe fn getAvailableCommandPayloadLength_(&self, inOpCode: OBEXOpCode) -> OBEXMaxPacketLength
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, getAvailableCommandPayloadLength : inOpCode)
    }
    unsafe fn getAvailableCommandResponsePayloadLength_(
        &self,
        inOpCode: OBEXOpCode,
    ) -> OBEXMaxPacketLength
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, getAvailableCommandResponsePayloadLength : inOpCode)
    }
    unsafe fn getMaxPacketLength(&self) -> OBEXMaxPacketLength
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, getMaxPacketLength)
    }
    unsafe fn hasOpenOBEXConnection(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, hasOpenOBEXConnection)
    }
    unsafe fn setEventCallback_(&self, inEventCallback: OBEXSessionEventCallback)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setEventCallback : inEventCallback)
    }
    unsafe fn setEventRefCon_(&self, inRefCon: *mut ::std::os::raw::c_void)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setEventRefCon : inRefCon)
    }
    unsafe fn setEventSelector_target_refCon_(
        &self,
        inEventSelector: objc2::runtime::Sel,
        inEventSelectorTarget: id,
        inUserRefCon: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setEventSelector : inEventSelector, target : inEventSelectorTarget, refCon : inUserRefCon)
    }
    unsafe fn serverHandleIncomingData_(&self, event: *mut OBEXTransportEvent)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, serverHandleIncomingData : event)
    }
    unsafe fn clientHandleIncomingData_(&self, event: *mut OBEXTransportEvent)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, clientHandleIncomingData : event)
    }
    unsafe fn sendDataToTransport_dataLength_(
        &self,
        inDataToSend: *mut ::std::os::raw::c_void,
        inDataLength: usize,
    ) -> OBEXError
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, sendDataToTransport : inDataToSend, dataLength : inDataLength)
    }
    unsafe fn openTransportConnection_selectorTarget_refCon_(
        &self,
        inSelector: objc2::runtime::Sel,
        inTarget: id,
        inUserRefCon: *mut ::std::os::raw::c_void,
    ) -> OBEXError
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, openTransportConnection : inSelector, selectorTarget : inTarget, refCon : inUserRefCon)
    }
    unsafe fn hasOpenTransportConnection(&self) -> Boolean
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, hasOpenTransportConnection)
    }
    unsafe fn closeTransportConnection(&self) -> OBEXError
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, closeTransportConnection)
    }
}
pub type OBEXTransportEventType = u32;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OBEXTransportEvent {
    pub type_: OBEXTransportEventType,
    pub status: OBEXError,
    pub dataPtr: *mut ::std::os::raw::c_void,
    pub dataLength: usize,
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct IOBluetoothOBEXSession(pub id);
impl std::ops::Deref for IOBluetoothOBEXSession {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for IOBluetoothOBEXSession {}
impl IOBluetoothOBEXSession {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"IOBluetoothOBEXSession").unwrap(), alloc) })
    }
}
impl PIOBluetoothRFCOMMChannelDelegate for IOBluetoothOBEXSession {}
impl IOBEXSession for IOBluetoothOBEXSession {}
impl From<IOBluetoothOBEXSession> for OBEXSession {
    fn from(child: IOBluetoothOBEXSession) -> OBEXSession {
        OBEXSession(child.0)
    }
}
impl std::convert::TryFrom<OBEXSession> for IOBluetoothOBEXSession {
    type Error = &'static str;
    fn try_from(parent: OBEXSession) -> Result<IOBluetoothOBEXSession, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"IOBluetoothOBEXSession").unwrap()) };
        if is_kind_of {
            Ok(IOBluetoothOBEXSession(parent.0))
        } else {
            Err("This OBEXSession cannot be downcasted to IOBluetoothOBEXSession")
        }
    }
}
impl INSObject for IOBluetoothOBEXSession {}
impl PNSObject for IOBluetoothOBEXSession {}
impl IIOBluetoothOBEXSession for IOBluetoothOBEXSession {}
pub trait IIOBluetoothOBEXSession: Sized + std::ops::Deref {
    unsafe fn initWithSDPServiceRecord_(
        &self,
        inSDPServiceRecord: IOBluetoothSDPServiceRecord,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithSDPServiceRecord : inSDPServiceRecord)
    }
    unsafe fn initWithDevice_channelID_(
        &self,
        inDevice: IOBluetoothDevice,
        inChannelID: BluetoothRFCOMMChannelID,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithDevice : inDevice, channelID : inChannelID)
    }
    unsafe fn initWithIncomingRFCOMMChannel_eventSelector_selectorTarget_refCon_(
        &self,
        inChannel: IOBluetoothRFCOMMChannel,
        inEventSelector: objc2::runtime::Sel,
        inEventSelectorTarget: id,
        inUserRefCon: *mut ::std::os::raw::c_void,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithIncomingRFCOMMChannel : inChannel, eventSelector : inEventSelector, selectorTarget : inEventSelectorTarget, refCon : inUserRefCon)
    }
    unsafe fn getRFCOMMChannel(&self) -> IOBluetoothRFCOMMChannel
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, getRFCOMMChannel)
    }
    unsafe fn getDevice(&self) -> IOBluetoothDevice
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, getDevice)
    }
    unsafe fn sendBufferTroughChannel(&self) -> IOReturn
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, sendBufferTroughChannel)
    }
    unsafe fn restartTransmission(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, restartTransmission)
    }
    unsafe fn isSessionTargetAMac(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isSessionTargetAMac)
    }
    unsafe fn openTransportConnection_selectorTarget_refCon_(
        &self,
        inSelector: objc2::runtime::Sel,
        inTarget: id,
        inUserRefCon: *mut ::std::os::raw::c_void,
    ) -> OBEXError
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, openTransportConnection : inSelector, selectorTarget : inTarget, refCon : inUserRefCon)
    }
    unsafe fn hasOpenTransportConnection(&self) -> Boolean
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, hasOpenTransportConnection)
    }
    unsafe fn closeTransportConnection(&self) -> OBEXError
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, closeTransportConnection)
    }
    unsafe fn sendDataToTransport_dataLength_(
        &self,
        inDataToSend: *mut ::std::os::raw::c_void,
        inDataLength: usize,
    ) -> OBEXError
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, sendDataToTransport : inDataToSend, dataLength : inDataLength)
    }
    unsafe fn setOpenTransportConnectionAsyncSelector_target_refCon_(
        &self,
        inSelector: objc2::runtime::Sel,
        inSelectorTarget: id,
        inUserRefCon: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setOpenTransportConnectionAsyncSelector : inSelector, target : inSelectorTarget, refCon : inUserRefCon)
    }
    unsafe fn setOBEXSessionOpenConnectionCallback_refCon_(
        &self,
        inCallback: IOBluetoothOBEXSessionOpenConnectionCallback,
        inUserRefCon: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setOBEXSessionOpenConnectionCallback : inCallback, refCon : inUserRefCon)
    }
    unsafe fn withSDPServiceRecord_(inSDPServiceRecord: IOBluetoothSDPServiceRecord) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"IOBluetoothOBEXSession").unwrap(), withSDPServiceRecord : inSDPServiceRecord)
    }
    unsafe fn withDevice_channelID_(
        inDevice: IOBluetoothDevice,
        inRFCOMMChannelID: BluetoothRFCOMMChannelID,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"IOBluetoothOBEXSession").unwrap(), withDevice : inDevice, channelID : inRFCOMMChannelID)
    }
    unsafe fn withIncomingRFCOMMChannel_eventSelector_selectorTarget_refCon_(
        inChannel: IOBluetoothRFCOMMChannel,
        inEventSelector: objc2::runtime::Sel,
        inEventSelectorTarget: id,
        inUserRefCon: *mut ::std::os::raw::c_void,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"IOBluetoothOBEXSession").unwrap(), withIncomingRFCOMMChannel : inChannel, eventSelector : inEventSelector, selectorTarget : inEventSelectorTarget, refCon : inUserRefCon)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct OBEXFileTransferServices(pub id);
impl std::ops::Deref for OBEXFileTransferServices {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for OBEXFileTransferServices {}
impl OBEXFileTransferServices {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"OBEXFileTransferServices").unwrap(), alloc) })
    }
}
impl INSObject for OBEXFileTransferServices {}
impl PNSObject for OBEXFileTransferServices {}
impl std::convert::TryFrom<NSObject> for OBEXFileTransferServices {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<OBEXFileTransferServices, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"OBEXFileTransferServices").unwrap()) };
        if is_kind_of {
            Ok(OBEXFileTransferServices(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to OBEXFileTransferServices")
        }
    }
}
impl IOBEXFileTransferServices for OBEXFileTransferServices {}
pub trait IOBEXFileTransferServices: Sized + std::ops::Deref {
    unsafe fn initWithOBEXSession_(&self, inOBEXSession: IOBluetoothOBEXSession) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithOBEXSession : inOBEXSession)
    }
    unsafe fn currentPath(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, currentPath)
    }
    unsafe fn isBusy(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isBusy)
    }
    unsafe fn isConnected(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isConnected)
    }
    unsafe fn connectToFTPService(&self) -> OBEXError
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, connectToFTPService)
    }
    unsafe fn connectToObjectPushService(&self) -> OBEXError
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, connectToObjectPushService)
    }
    unsafe fn disconnect(&self) -> OBEXError
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, disconnect)
    }
    unsafe fn changeCurrentFolderToRoot(&self) -> OBEXError
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, changeCurrentFolderToRoot)
    }
    unsafe fn changeCurrentFolderBackward(&self) -> OBEXError
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, changeCurrentFolderBackward)
    }
    unsafe fn changeCurrentFolderForwardToPath_(&self, inDirName: NSString) -> OBEXError
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, changeCurrentFolderForwardToPath : inDirName)
    }
    unsafe fn createFolder_(&self, inDirName: NSString) -> OBEXError
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, createFolder : inDirName)
    }
    unsafe fn removeItem_(&self, inItemName: NSString) -> OBEXError
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, removeItem : inItemName)
    }
    unsafe fn retrieveFolderListing(&self) -> OBEXError
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, retrieveFolderListing)
    }
    unsafe fn sendFile_(&self, inLocalPathAndName: NSString) -> OBEXError
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, sendFile : inLocalPathAndName)
    }
    unsafe fn copyRemoteFile_toLocalPath_(
        &self,
        inRemoteFileName: NSString,
        inLocalPathAndName: NSString,
    ) -> OBEXError
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, copyRemoteFile : inRemoteFileName, toLocalPath : inLocalPathAndName)
    }
    unsafe fn sendData_type_name_(
        &self,
        inData: NSData,
        inType: NSString,
        inName: NSString,
    ) -> OBEXError
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, sendData : inData, r#type : inType, name : inName)
    }
    unsafe fn getDefaultVCard_(&self, inLocalPathAndName: NSString) -> OBEXError
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, getDefaultVCard : inLocalPathAndName)
    }
    unsafe fn abort(&self) -> OBEXError
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, abort)
    }
    unsafe fn delegate(&self) -> id
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, delegate)
    }
    unsafe fn setDelegate_(&self, delegate: id)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDelegate : delegate)
    }
    unsafe fn withOBEXSession_(inOBEXSession: IOBluetoothOBEXSession) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"OBEXFileTransferServices").unwrap(), withOBEXSession : inOBEXSession)
    }
}
pub trait NSObject_OBEXFileTransferServicesDelegate: Sized + std::ops::Deref {
    unsafe fn fileTransferServicesConnectionComplete_error_(
        &self,
        inServices: OBEXFileTransferServices,
        inError: OBEXError,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, fileTransferServicesConnectionComplete : inServices, error : inError)
    }
    unsafe fn fileTransferServicesDisconnectionComplete_error_(
        &self,
        inServices: OBEXFileTransferServices,
        inError: OBEXError,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, fileTransferServicesDisconnectionComplete : inServices, error : inError)
    }
    unsafe fn fileTransferServicesAbortComplete_error_(
        &self,
        inServices: OBEXFileTransferServices,
        inError: OBEXError,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, fileTransferServicesAbortComplete : inServices, error : inError)
    }
    unsafe fn fileTransferServicesRemoveItemComplete_error_removedItem_(
        &self,
        inServices: OBEXFileTransferServices,
        inError: OBEXError,
        inItemName: NSString,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, fileTransferServicesRemoveItemComplete : inServices, error : inError, removedItem : inItemName)
    }
    unsafe fn fileTransferServicesCreateFolderComplete_error_folder_(
        &self,
        inServices: OBEXFileTransferServices,
        inError: OBEXError,
        inFolderName: NSString,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, fileTransferServicesCreateFolderComplete : inServices, error : inError, folder : inFolderName)
    }
    unsafe fn fileTransferServicesPathChangeComplete_error_finalPath_(
        &self,
        inServices: OBEXFileTransferServices,
        inError: OBEXError,
        inPath: NSString,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, fileTransferServicesPathChangeComplete : inServices, error : inError, finalPath : inPath)
    }
    unsafe fn fileTransferServicesRetrieveFolderListingComplete_error_listing_(
        &self,
        inServices: OBEXFileTransferServices,
        inError: OBEXError,
        inListing: NSArray,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, fileTransferServicesRetrieveFolderListingComplete : inServices, error : inError, listing : inListing)
    }
    unsafe fn fileTransferServicesFilePreparationComplete_error_(
        &self,
        inServices: OBEXFileTransferServices,
        inError: OBEXError,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, fileTransferServicesFilePreparationComplete : inServices, error : inError)
    }
    unsafe fn fileTransferServicesSendFileProgress_transferProgress_(
        &self,
        inServices: OBEXFileTransferServices,
        inProgressDescription: NSDictionary,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, fileTransferServicesSendFileProgress : inServices, transferProgress : inProgressDescription)
    }
    unsafe fn fileTransferServicesSendFileComplete_error_(
        &self,
        inServices: OBEXFileTransferServices,
        inError: OBEXError,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, fileTransferServicesSendFileComplete : inServices, error : inError)
    }
    unsafe fn fileTransferServicesCopyRemoteFileProgress_transferProgress_(
        &self,
        inServices: OBEXFileTransferServices,
        inProgressDescription: NSDictionary,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, fileTransferServicesCopyRemoteFileProgress : inServices, transferProgress : inProgressDescription)
    }
    unsafe fn fileTransferServicesCopyRemoteFileComplete_error_(
        &self,
        inServices: OBEXFileTransferServices,
        inError: OBEXError,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, fileTransferServicesCopyRemoteFileComplete : inServices, error : inError)
    }
}
pub trait NSMutableDictionary_NSDictionaryOBEXExtensions: Sized + std::ops::Deref {
    unsafe fn getHeaderBytes(&self) -> NSMutableData
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, getHeaderBytes)
    }
    unsafe fn addTargetHeader_length_(
        &self,
        inHeaderData: *const ::std::os::raw::c_void,
        inHeaderDataLength: u32,
    ) -> OBEXError
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, addTargetHeader : inHeaderData, length : inHeaderDataLength)
    }
    unsafe fn addHTTPHeader_length_(
        &self,
        inHeaderData: *const ::std::os::raw::c_void,
        inHeaderDataLength: u32,
    ) -> OBEXError
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, addHTTPHeader : inHeaderData, length : inHeaderDataLength)
    }
    unsafe fn addBodyHeader_length_endOfBody_(
        &self,
        inHeaderData: *const ::std::os::raw::c_void,
        inHeaderDataLength: u32,
        isEndOfBody: BOOL,
    ) -> OBEXError
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, addBodyHeader : inHeaderData, length : inHeaderDataLength, endOfBody : isEndOfBody)
    }
    unsafe fn addWhoHeader_length_(
        &self,
        inHeaderData: *const ::std::os::raw::c_void,
        inHeaderDataLength: u32,
    ) -> OBEXError
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, addWhoHeader : inHeaderData, length : inHeaderDataLength)
    }
    unsafe fn addConnectionIDHeader_length_(
        &self,
        inHeaderData: *const ::std::os::raw::c_void,
        inHeaderDataLength: u32,
    ) -> OBEXError
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, addConnectionIDHeader : inHeaderData, length : inHeaderDataLength)
    }
    unsafe fn addApplicationParameterHeader_length_(
        &self,
        inHeaderData: *const ::std::os::raw::c_void,
        inHeaderDataLength: u32,
    ) -> OBEXError
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, addApplicationParameterHeader : inHeaderData, length : inHeaderDataLength)
    }
    unsafe fn addByteSequenceHeader_length_(
        &self,
        inHeaderData: *const ::std::os::raw::c_void,
        inHeaderDataLength: u32,
    ) -> OBEXError
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, addByteSequenceHeader : inHeaderData, length : inHeaderDataLength)
    }
    unsafe fn addObjectClassHeader_length_(
        &self,
        inHeaderData: *const ::std::os::raw::c_void,
        inHeaderDataLength: u32,
    ) -> OBEXError
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, addObjectClassHeader : inHeaderData, length : inHeaderDataLength)
    }
    unsafe fn addAuthorizationChallengeHeader_length_(
        &self,
        inHeaderData: *const ::std::os::raw::c_void,
        inHeaderDataLength: u32,
    ) -> OBEXError
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, addAuthorizationChallengeHeader : inHeaderData, length : inHeaderDataLength)
    }
    unsafe fn addAuthorizationResponseHeader_length_(
        &self,
        inHeaderData: *const ::std::os::raw::c_void,
        inHeaderDataLength: u32,
    ) -> OBEXError
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, addAuthorizationResponseHeader : inHeaderData, length : inHeaderDataLength)
    }
    unsafe fn addTimeISOHeader_length_(
        &self,
        inHeaderData: *const ::std::os::raw::c_void,
        inHeaderDataLength: u32,
    ) -> OBEXError
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, addTimeISOHeader : inHeaderData, length : inHeaderDataLength)
    }
    unsafe fn addTypeHeader_(&self, type_: NSString) -> OBEXError
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, addTypeHeader : type_)
    }
    unsafe fn addLengthHeader_(&self, length: u32) -> OBEXError
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, addLengthHeader : length)
    }
    unsafe fn addTime4ByteHeader_(&self, time4Byte: u32) -> OBEXError
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, addTime4ByteHeader : time4Byte)
    }
    unsafe fn addCountHeader_(&self, inCount: u32) -> OBEXError
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, addCountHeader : inCount)
    }
    unsafe fn addDescriptionHeader_(&self, inDescriptionString: NSString) -> OBEXError
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, addDescriptionHeader : inDescriptionString)
    }
    unsafe fn addNameHeader_(&self, inNameString: NSString) -> OBEXError
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, addNameHeader : inNameString)
    }
    unsafe fn addUserDefinedHeader_length_(
        &self,
        inHeaderData: *const ::std::os::raw::c_void,
        inHeaderDataLength: u32,
    ) -> OBEXError
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, addUserDefinedHeader : inHeaderData, length : inHeaderDataLength)
    }
    unsafe fn addImageHandleHeader_(&self, type_: NSString) -> OBEXError
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, addImageHandleHeader : type_)
    }
    unsafe fn addImageDescriptorHeader_length_(
        &self,
        inHeaderData: *const ::std::os::raw::c_void,
        inHeaderDataLength: u32,
    ) -> OBEXError
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, addImageDescriptorHeader : inHeaderData, length : inHeaderDataLength)
    }
    unsafe fn dictionaryWithOBEXHeadersData_headersDataSize_(
        inHeadersData: *const ::std::os::raw::c_void,
        inDataSize: usize,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NSMutableDictionary").unwrap(), dictionaryWithOBEXHeadersData : inHeadersData, headersDataSize : inDataSize)
    }
    unsafe fn dictionaryWithOBEXHeadersData_(inHeadersData: NSData) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NSMutableDictionary").unwrap(), dictionaryWithOBEXHeadersData : inHeadersData)
    }
    unsafe fn withOBEXHeadersData_headersDataSize_(
        inHeadersData: *const ::std::os::raw::c_void,
        inDataSize: usize,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NSMutableDictionary").unwrap(), withOBEXHeadersData : inHeadersData, headersDataSize : inDataSize)
    }
}
pub type IOBluetoothSMSMode = NSUInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct IOBluetoothHandsFree(pub id);
impl std::ops::Deref for IOBluetoothHandsFree {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for IOBluetoothHandsFree {}
impl IOBluetoothHandsFree {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"IOBluetoothHandsFree").unwrap(), alloc) })
    }
}
impl INSObject for IOBluetoothHandsFree {}
impl PNSObject for IOBluetoothHandsFree {}
impl std::convert::TryFrom<NSObject> for IOBluetoothHandsFree {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<IOBluetoothHandsFree, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"IOBluetoothHandsFree").unwrap()) };
        if is_kind_of {
            Ok(IOBluetoothHandsFree(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to IOBluetoothHandsFree")
        }
    }
}
impl IIOBluetoothHandsFree for IOBluetoothHandsFree {}
pub trait IIOBluetoothHandsFree: Sized + std::ops::Deref {
    unsafe fn indicator_(&self, indicatorName: NSString) -> ::std::os::raw::c_int
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, indicator : indicatorName)
    }
    unsafe fn setIndicator_value_(
        &self,
        indicatorName: NSString,
        indicatorValue: ::std::os::raw::c_int,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setIndicator : indicatorName, value : indicatorValue)
    }
    unsafe fn initWithDevice_delegate_(
        &self,
        device: IOBluetoothDevice,
        inDelegate: *mut u64,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithDevice : device, delegate : inDelegate)
    }
    unsafe fn connect(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, connect)
    }
    unsafe fn disconnect(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, disconnect)
    }
    unsafe fn connectSCO(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, connectSCO)
    }
    unsafe fn disconnectSCO(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, disconnectSCO)
    }
    unsafe fn isSCOConnected(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isSCOConnected)
    }
    unsafe fn supportedFeatures(&self) -> u32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, supportedFeatures)
    }
    unsafe fn setSupportedFeatures_(&self, supportedFeatures: u32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSupportedFeatures : supportedFeatures)
    }
    unsafe fn inputVolume(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inputVolume)
    }
    unsafe fn setInputVolume_(&self, inputVolume: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInputVolume : inputVolume)
    }
    unsafe fn isInputMuted(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isInputMuted)
    }
    unsafe fn setInputMuted_(&self, inputMuted: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInputMuted : inputMuted)
    }
    unsafe fn outputVolume(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, outputVolume)
    }
    unsafe fn setOutputVolume_(&self, outputVolume: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setOutputVolume : outputVolume)
    }
    unsafe fn isOutputMuted(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isOutputMuted)
    }
    unsafe fn setOutputMuted_(&self, outputMuted: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setOutputMuted : outputMuted)
    }
    unsafe fn device(&self) -> IOBluetoothDevice
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, device)
    }
    unsafe fn deviceSupportedFeatures(&self) -> u32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, deviceSupportedFeatures)
    }
    unsafe fn deviceSupportedSMSServices(&self) -> u32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, deviceSupportedSMSServices)
    }
    unsafe fn deviceCallHoldModes(&self) -> u32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, deviceCallHoldModes)
    }
    unsafe fn SMSMode(&self) -> IOBluetoothSMSMode
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, SMSMode)
    }
    unsafe fn isSMSEnabled(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isSMSEnabled)
    }
    unsafe fn delegate(&self) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, delegate)
    }
    unsafe fn setDelegate_(&self, delegate: *mut u64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDelegate : delegate)
    }
    unsafe fn isConnected(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isConnected)
    }
}
impl IOBluetoothDevice_HandsFreeDeviceAdditions for IOBluetoothDevice {}
pub trait IOBluetoothDevice_HandsFreeDeviceAdditions: Sized + std::ops::Deref {
    unsafe fn handsFreeAudioGatewayDriverID(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, handsFreeAudioGatewayDriverID)
    }
    unsafe fn handsFreeAudioGatewayServiceRecord(&self) -> IOBluetoothSDPServiceRecord
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, handsFreeAudioGatewayServiceRecord)
    }
    unsafe fn handsFreeDeviceDriverID(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, handsFreeDeviceDriverID)
    }
    unsafe fn handsFreeDeviceServiceRecord(&self) -> IOBluetoothSDPServiceRecord
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, handsFreeDeviceServiceRecord)
    }
    unsafe fn isHandsFreeAudioGateway(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isHandsFreeAudioGateway)
    }
    unsafe fn isHandsFreeDevice(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isHandsFreeDevice)
    }
}
impl IOBluetoothSDPServiceRecord_HandsFreeSDPServiceRecordAdditions
    for IOBluetoothSDPServiceRecord
{
}
pub trait IOBluetoothSDPServiceRecord_HandsFreeSDPServiceRecordAdditions:
    Sized + std::ops::Deref
{
    unsafe fn handsFreeSupportedFeatures(&self) -> u16
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, handsFreeSupportedFeatures)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct IOBluetoothHandsFreeAudioGateway(pub id);
impl std::ops::Deref for IOBluetoothHandsFreeAudioGateway {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for IOBluetoothHandsFreeAudioGateway {}
impl IOBluetoothHandsFreeAudioGateway {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"IOBluetoothHandsFreeAudioGateway").unwrap(), alloc) })
    }
}
impl IIOBluetoothHandsFree for IOBluetoothHandsFreeAudioGateway {}
impl From<IOBluetoothHandsFreeAudioGateway> for IOBluetoothHandsFree {
    fn from(child: IOBluetoothHandsFreeAudioGateway) -> IOBluetoothHandsFree {
        IOBluetoothHandsFree(child.0)
    }
}
impl std::convert::TryFrom<IOBluetoothHandsFree> for IOBluetoothHandsFreeAudioGateway {
    type Error = &'static str;
    fn try_from(
        parent: IOBluetoothHandsFree,
    ) -> Result<IOBluetoothHandsFreeAudioGateway, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"IOBluetoothHandsFreeAudioGateway").unwrap())
        };
        if is_kind_of {
            Ok(IOBluetoothHandsFreeAudioGateway(parent.0))
        } else {
            Err ("This IOBluetoothHandsFree cannot be downcasted to IOBluetoothHandsFreeAudioGateway" ,)
        }
    }
}
impl INSObject for IOBluetoothHandsFreeAudioGateway {}
impl PNSObject for IOBluetoothHandsFreeAudioGateway {}
impl IIOBluetoothHandsFreeAudioGateway for IOBluetoothHandsFreeAudioGateway {}
pub trait IIOBluetoothHandsFreeAudioGateway: Sized + std::ops::Deref {
    unsafe fn initWithDevice_delegate_(
        &self,
        device: IOBluetoothDevice,
        inDelegate: id,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithDevice : device, delegate : inDelegate)
    }
    unsafe fn createIndicator_min_max_currentValue_(
        &self,
        indicatorName: NSString,
        minValue: ::std::os::raw::c_int,
        maxValue: ::std::os::raw::c_int,
        currentValue: ::std::os::raw::c_int,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, createIndicator : indicatorName, min : minValue, max : maxValue, currentValue : currentValue)
    }
    unsafe fn processATCommand_(&self, atCommand: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, processATCommand : atCommand)
    }
    unsafe fn sendOKResponse(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, sendOKResponse)
    }
    unsafe fn sendResponse_(&self, response: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, sendResponse : response)
    }
    unsafe fn sendResponse_withOK_(&self, response: NSString, withOK: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, sendResponse : response, withOK : withOK)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct IOBluetoothHandsFreeDevice(pub id);
impl std::ops::Deref for IOBluetoothHandsFreeDevice {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for IOBluetoothHandsFreeDevice {}
impl IOBluetoothHandsFreeDevice {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"IOBluetoothHandsFreeDevice").unwrap(), alloc) })
    }
}
impl IIOBluetoothHandsFree for IOBluetoothHandsFreeDevice {}
impl std::convert::TryFrom<IOBluetoothHandsFree> for IOBluetoothHandsFreeDevice {
    type Error = &'static str;
    fn try_from(parent: IOBluetoothHandsFree) -> Result<IOBluetoothHandsFreeDevice, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"IOBluetoothHandsFreeDevice").unwrap()) };
        if is_kind_of {
            Ok(IOBluetoothHandsFreeDevice(parent.0))
        } else {
            Err("This IOBluetoothHandsFree cannot be downcasted to IOBluetoothHandsFreeDevice")
        }
    }
}
impl INSObject for IOBluetoothHandsFreeDevice {}
impl PNSObject for IOBluetoothHandsFreeDevice {}
impl IIOBluetoothHandsFreeDevice for IOBluetoothHandsFreeDevice {}
pub trait IIOBluetoothHandsFreeDevice: Sized + std::ops::Deref {
    unsafe fn initWithDevice_delegate_(
        &self,
        device: IOBluetoothDevice,
        delegate: id,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithDevice : device, delegate : delegate)
    }
    unsafe fn dialNumber_(&self, aNumber: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, dialNumber : aNumber)
    }
    unsafe fn memoryDial_(&self, memoryLocation: ::std::os::raw::c_int)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, memoryDial : memoryLocation)
    }
    unsafe fn redial(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, redial)
    }
    unsafe fn endCall(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, endCall)
    }
    unsafe fn acceptCall(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, acceptCall)
    }
    unsafe fn acceptCallOnPhone(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, acceptCallOnPhone)
    }
    unsafe fn sendDTMF_(&self, character: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, sendDTMF : character)
    }
    unsafe fn subscriberNumber(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, subscriberNumber)
    }
    unsafe fn currentCallList(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, currentCallList)
    }
    unsafe fn releaseHeldCalls(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, releaseHeldCalls)
    }
    unsafe fn releaseActiveCalls(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, releaseActiveCalls)
    }
    unsafe fn releaseCall_(&self, index: ::std::os::raw::c_int)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, releaseCall : index)
    }
    unsafe fn holdCall(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, holdCall)
    }
    unsafe fn placeAllOthersOnHold_(&self, index: ::std::os::raw::c_int)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, placeAllOthersOnHold : index)
    }
    unsafe fn addHeldCall(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, addHeldCall)
    }
    unsafe fn callTransfer(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, callTransfer)
    }
    unsafe fn transferAudioToComputer(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, transferAudioToComputer)
    }
    unsafe fn transferAudioToPhone(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, transferAudioToPhone)
    }
    unsafe fn sendSMS_message_(&self, aNumber: NSString, aMessage: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, sendSMS : aNumber, message : aMessage)
    }
    unsafe fn sendATCommand_(&self, atCommand: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, sendATCommand : atCommand)
    }
    unsafe fn sendATCommand_timeout_selector_target_(
        &self,
        atCommand: NSString,
        timeout: f32,
        selector: objc2::runtime::Sel,
        target: id,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, sendATCommand : atCommand, timeout : timeout, selector : selector, target : target)
    }
}
unsafe extern "C" {
    pub fn IOBluetoothIgnoreHIDDevice(device: IOBluetoothDeviceRef);
}
unsafe extern "C" {
    pub fn IOBluetoothRemoveIgnoredHIDDevice(device: IOBluetoothDeviceRef);
}
unsafe extern "C" {
    pub fn IOBluetoothUserNotificationUnregister(notificationRef: IOBluetoothUserNotificationRef);
}
unsafe extern "C" {
    pub fn IOBluetoothL2CAPChannelRegisterForChannelCloseNotification(
        channel: IOBluetoothL2CAPChannelRef,
        callback: IOBluetoothUserNotificationCallback,
        inRefCon: *mut ::std::os::raw::c_void,
    ) -> IOBluetoothUserNotificationRef;
}
unsafe extern "C" {
    pub fn IOBluetoothAddSCOAudioDevice(
        device: IOBluetoothDeviceRef,
        configDict: CFDictionaryRef,
    ) -> IOReturn;
}
unsafe extern "C" {
    pub fn IOBluetoothRemoveSCOAudioDevice(device: IOBluetoothDeviceRef) -> IOReturn;
}
unsafe extern "C" {
    pub fn IOBluetoothNSStringToDeviceAddress(
        inNameString: NSString,
        outDeviceAddress: *mut BluetoothDeviceAddress,
    ) -> IOReturn;
}
unsafe extern "C" {
    pub fn IOBluetoothNSStringFromDeviceAddress(
        deviceAddress: *const BluetoothDeviceAddress,
    ) -> NSString;
}
unsafe extern "C" {
    pub fn IOBluetoothNSStringFromDeviceAddressColon(
        deviceAddress: *const BluetoothDeviceAddress,
    ) -> NSString;
}
unsafe extern "C" {
    pub fn IOBluetoothIsFileAppleDesignatedPIMData(inFileName: NSString) -> Boolean;
}
unsafe extern "C" {
    pub fn IOBluetoothGetUniqueFileNameAndPath(inName: NSString, inPath: NSString) -> NSString;
}
unsafe extern "C" {
    pub fn IOBluetoothPackData(
        ioBuffer: *mut ::std::os::raw::c_void,
        inFormat: *const ::std::os::raw::c_char,
        ...
    ) -> ::std::os::raw::c_long;
}
unsafe extern "C" {
    pub fn IOBluetoothPackDataList(
        ioBuffer: *mut ::std::os::raw::c_void,
        inFormat: *const ::std::os::raw::c_char,
        inArgs: va_list,
    ) -> ::std::os::raw::c_long;
}
unsafe extern "C" {
    pub fn IOBluetoothUnpackData(
        inBufferSize: ByteCount,
        inBuffer: *const ::std::os::raw::c_void,
        inFormat: *const ::std::os::raw::c_char,
        ...
    ) -> ::std::os::raw::c_long;
}
unsafe extern "C" {
    pub fn IOBluetoothUnpackDataList(
        inBufferSize: ByteCount,
        inBuffer: *const ::std::os::raw::c_void,
        inFormat: *const ::std::os::raw::c_char,
        inArgs: va_list,
    ) -> ::std::os::raw::c_long;
}
unsafe extern "C" {
    pub fn IOBluetoothNumberOfAvailableHIDDevices() -> ::std::os::raw::c_long;
}
unsafe extern "C" {
    pub fn IOBluetoothNumberOfPointingHIDDevices() -> ::std::os::raw::c_long;
}
unsafe extern "C" {
    pub fn IOBluetoothNumberOfKeyboardHIDDevices() -> ::std::os::raw::c_long;
}
unsafe extern "C" {
    pub fn IOBluetoothNumberOfTabletHIDDevices() -> ::std::os::raw::c_long;
}
unsafe extern "C" {
    pub fn IOBluetoothFindNumberOfRegistryEntriesOfClassName(
        deviceType: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_long;
}
unsafe extern "C" {
    pub static IOBluetoothHostControllerPoweredOnNotification: NSString;
}
unsafe extern "C" {
    pub static IOBluetoothHostControllerPoweredOffNotification: NSString;
}
unsafe extern "C" {
    pub static IOBluetoothL2CAPChannelPublishedNotification: NSString;
}
unsafe extern "C" {
    pub static IOBluetoothL2CAPChannelTerminatedNotification: NSString;
}
unsafe extern "C" {
    pub fn OBEXSessionDelete(inSessionRef: OBEXSessionRef) -> OBEXError;
}
unsafe extern "C" {
    pub fn OBEXSessionHasOpenOBEXConnection(
        inSessionRef: OBEXSessionRef,
        outIsConnected: *mut Boolean,
    ) -> OBEXError;
}
unsafe extern "C" {
    pub fn OBEXSessionGetMaxPacketLength(
        inSessionRef: OBEXSessionRef,
        outLength: *mut OBEXMaxPacketLength,
    ) -> OBEXError;
}
unsafe extern "C" {
    pub fn OBEXSessionGetAvailableCommandPayloadLength(
        inSessionRef: OBEXSessionRef,
        inOpCode: OBEXOpCode,
        outLength: *mut OBEXMaxPacketLength,
    ) -> OBEXError;
}
unsafe extern "C" {
    pub fn OBEXSessionGetAvailableCommandResponsePayloadLength(
        inSessionRef: OBEXSessionRef,
        inOpCode: OBEXOpCode,
        outLength: *mut OBEXMaxPacketLength,
    ) -> OBEXError;
}
unsafe extern "C" {
    pub fn OBEXSessionConnect(
        inSessionRef: OBEXSessionRef,
        inFlags: OBEXFlags,
        inMaxPacketLength: OBEXMaxPacketLength,
        inOptionalHeaders: *mut ::std::os::raw::c_void,
        inOptionalHeadersLength: usize,
        inCallback: OBEXSessionEventCallback,
        inUserRefCon: *mut ::std::os::raw::c_void,
    ) -> OBEXError;
}
unsafe extern "C" {
    pub fn OBEXSessionDisconnect(
        inSessionRef: OBEXSessionRef,
        inOptionalHeaders: *mut ::std::os::raw::c_void,
        inOptionalHeadersLength: usize,
        inCallback: OBEXSessionEventCallback,
        inUserRefCon: *mut ::std::os::raw::c_void,
    ) -> OBEXError;
}
unsafe extern "C" {
    pub fn OBEXSessionPut(
        inSessionRef: OBEXSessionRef,
        inIsFinalChunk: Boolean,
        inHeadersData: *mut ::std::os::raw::c_void,
        inHeadersDataLength: usize,
        inBodyData: *mut ::std::os::raw::c_void,
        inBodyDataLength: usize,
        inCallback: OBEXSessionEventCallback,
        inUserRefCon: *mut ::std::os::raw::c_void,
    ) -> OBEXError;
}
unsafe extern "C" {
    pub fn OBEXSessionGet(
        inSessionRef: OBEXSessionRef,
        inIsFinalChunk: Boolean,
        inHeadersData: *mut ::std::os::raw::c_void,
        inHeadersDataLength: usize,
        inCallback: OBEXSessionEventCallback,
        inUserRefCon: *mut ::std::os::raw::c_void,
    ) -> OBEXError;
}
unsafe extern "C" {
    pub fn OBEXSessionAbort(
        inSessionRef: OBEXSessionRef,
        inOptionalHeaders: *mut ::std::os::raw::c_void,
        inOptionalHeadersLength: usize,
        inCallback: OBEXSessionEventCallback,
        inUserRefCon: *mut ::std::os::raw::c_void,
    ) -> OBEXError;
}
unsafe extern "C" {
    pub fn OBEXSessionSetPath(
        inSessionRef: OBEXSessionRef,
        inFlags: OBEXFlags,
        inConstants: OBEXConstants,
        inOptionalHeaders: *mut ::std::os::raw::c_void,
        inOptionalHeadersLength: usize,
        inCallback: OBEXSessionEventCallback,
        inUserRefCon: *mut ::std::os::raw::c_void,
    ) -> OBEXError;
}
unsafe extern "C" {
    pub fn OBEXSessionConnectResponse(
        inSessionRef: OBEXSessionRef,
        inResponseOpCode: OBEXOpCode,
        inFlags: OBEXFlags,
        inMaxPacketLength: OBEXMaxPacketLength,
        inOptionalHeaders: *mut ::std::os::raw::c_void,
        inOptionalHeadersLength: usize,
        inCallback: OBEXSessionEventCallback,
        inUserRefCon: *mut ::std::os::raw::c_void,
    ) -> OBEXError;
}
unsafe extern "C" {
    pub fn OBEXSessionDisconnectResponse(
        inSessionRef: OBEXSessionRef,
        inResponseOpCode: OBEXOpCode,
        inOptionalHeaders: *mut ::std::os::raw::c_void,
        inOptionalHeadersLength: usize,
        inCallback: OBEXSessionEventCallback,
        inUserRefCon: *mut ::std::os::raw::c_void,
    ) -> OBEXError;
}
unsafe extern "C" {
    pub fn OBEXSessionGetResponse(
        inSessionRef: OBEXSessionRef,
        inResponseOpCode: OBEXOpCode,
        inOptionalHeaders: *mut ::std::os::raw::c_void,
        inOptionalHeadersLength: usize,
        inCallback: OBEXSessionEventCallback,
        inUserRefCon: *mut ::std::os::raw::c_void,
    ) -> OBEXError;
}
unsafe extern "C" {
    pub fn OBEXSessionPutResponse(
        inSessionRef: OBEXSessionRef,
        inResponseOpCode: OBEXOpCode,
        inOptionalHeaders: *mut ::std::os::raw::c_void,
        inOptionalHeadersLength: usize,
        inCallback: OBEXSessionEventCallback,
        inUserRefCon: *mut ::std::os::raw::c_void,
    ) -> OBEXError;
}
unsafe extern "C" {
    pub fn OBEXSessionAbortResponse(
        inSessionRef: OBEXSessionRef,
        inResponseOpCode: OBEXOpCode,
        inOptionalHeaders: *mut ::std::os::raw::c_void,
        inOptionalHeadersLength: usize,
        inCallback: OBEXSessionEventCallback,
        inUserRefCon: *mut ::std::os::raw::c_void,
    ) -> OBEXError;
}
unsafe extern "C" {
    pub fn OBEXSessionSetPathResponse(
        inSessionRef: OBEXSessionRef,
        inResponseOpCode: OBEXOpCode,
        inOptionalHeaders: *mut ::std::os::raw::c_void,
        inOptionalHeadersLength: usize,
        inCallback: OBEXSessionEventCallback,
        inUserRefCon: *mut ::std::os::raw::c_void,
    ) -> OBEXError;
}
unsafe extern "C" {
    pub fn OBEXSessionSetServerCallback(
        inSessionRef: OBEXSessionRef,
        inCallback: OBEXSessionEventCallback,
        inUserRefCon: *mut ::std::os::raw::c_void,
    ) -> OBEXError;
}
unsafe extern "C" {
    pub fn OBEXCreateVCard(
        inFirstName: *const ::std::os::raw::c_void,
        inFirstNameLength: u32,
        inLastName: *const ::std::os::raw::c_void,
        inLastNameLength: u32,
        inFriendlyName: *const ::std::os::raw::c_void,
        inFriendlyNameLength: u32,
        inNameCharset: *const ::std::os::raw::c_void,
        inNameCharsetLength: u32,
        inHomePhone: *const ::std::os::raw::c_void,
        inHomePhoneLength: u32,
        inWorkPhone: *const ::std::os::raw::c_void,
        inWorkPhoneLength: u32,
        inCellPhone: *const ::std::os::raw::c_void,
        inCellPhoneLength: u32,
        inFaxPhone: *const ::std::os::raw::c_void,
        inFaxPhoneLength: u32,
        inEMailAddress: *const ::std::os::raw::c_void,
        inEMailAddressLength: u32,
        inEMailAddressCharset: *const ::std::os::raw::c_void,
        inEMailAddressCharsetLength: u32,
        inOrganization: *const ::std::os::raw::c_void,
        inOrganizationLength: u32,
        inOrganizationCharset: *const ::std::os::raw::c_void,
        inOrganizationCharsetLength: u32,
        inTitle: *const ::std::os::raw::c_void,
        inTitleLength: u32,
        inTitleCharset: *const ::std::os::raw::c_void,
        inTitleCharsetLength: u32,
    ) -> CFDataRef;
}
unsafe extern "C" {
    pub fn OBEXCreateVEvent(
        inCharset: *const ::std::os::raw::c_char,
        inCharsetLength: u32,
        inEncoding: *const ::std::os::raw::c_char,
        inEncodingLength: u32,
        inEventStartDate: *const ::std::os::raw::c_char,
        inEventStartDateLength: u32,
        inEventEndDate: *const ::std::os::raw::c_char,
        inEventEndDateLength: u32,
        inAlarmDate: *const ::std::os::raw::c_char,
        inAlarmDateLength: u32,
        inCategory: *const ::std::os::raw::c_char,
        inCategoryLength: u32,
        inSummary: *const ::std::os::raw::c_char,
        inSummaryLength: u32,
        inLocation: *const ::std::os::raw::c_char,
        inLocationLength: u32,
        inXIRMCLUID: *const ::std::os::raw::c_char,
        inXIRMCLUIDLength: u32,
    ) -> CFDataRef;
}
unsafe extern "C" {
    pub static mut kOBEXHeaderIDKeyName: CFStringRef;
}
unsafe extern "C" {
    pub static mut kOBEXHeaderIDKeyType: CFStringRef;
}
unsafe extern "C" {
    pub static mut kOBEXHeaderIDKeyDescription: CFStringRef;
}
unsafe extern "C" {
    pub static mut kOBEXHeaderIDKeyTimeISO: CFStringRef;
}
unsafe extern "C" {
    pub static mut kOBEXHeaderIDKeyTime4Byte: CFStringRef;
}
unsafe extern "C" {
    pub static mut kOBEXHeaderIDKeyTarget: CFStringRef;
}
unsafe extern "C" {
    pub static mut kOBEXHeaderIDKeyHTTP: CFStringRef;
}
unsafe extern "C" {
    pub static mut kOBEXHeaderIDKeyBody: CFStringRef;
}
unsafe extern "C" {
    pub static mut kOBEXHeaderIDKeyEndOfBody: CFStringRef;
}
unsafe extern "C" {
    pub static mut kOBEXHeaderIDKeyWho: CFStringRef;
}
unsafe extern "C" {
    pub static mut kOBEXHeaderIDKeyAppParameters: CFStringRef;
}
unsafe extern "C" {
    pub static mut kOBEXHeaderIDKeyAuthorizationChallenge: CFStringRef;
}
unsafe extern "C" {
    pub static mut kOBEXHeaderIDKeyAuthorizationResponse: CFStringRef;
}
unsafe extern "C" {
    pub static mut kOBEXHeaderIDKeyObjectClass: CFStringRef;
}
unsafe extern "C" {
    pub static mut kOBEXHeaderIDKeyCount: CFStringRef;
}
unsafe extern "C" {
    pub static mut kOBEXHeaderIDKeyLength: CFStringRef;
}
unsafe extern "C" {
    pub static mut kOBEXHeaderIDKeyConnectionID: CFStringRef;
}
unsafe extern "C" {
    pub static mut kOBEXHeaderIDKeyByteSequence: CFStringRef;
}
unsafe extern "C" {
    pub static mut kOBEXHeaderIDKeyUnknownUnicodeText: CFStringRef;
}
unsafe extern "C" {
    pub static mut kOBEXHeaderIDKeyUnknownByteSequence: CFStringRef;
}
unsafe extern "C" {
    pub static mut kOBEXHeaderIDKeyUnknown1ByteQuantity: CFStringRef;
}
unsafe extern "C" {
    pub static mut kOBEXHeaderIDKeyUnknown4ByteQuantity: CFStringRef;
}
unsafe extern "C" {
    pub static mut kOBEXHeaderIDKeyUserDefined: CFStringRef;
}
unsafe extern "C" {
    pub fn OBEXGetHeaders(
        inData: *const ::std::os::raw::c_void,
        inDataSize: usize,
    ) -> CFDictionaryRef;
}
unsafe extern "C" {
    pub fn OBEXHeadersToBytes(dictionaryOfHeaders: CFDictionaryRef) -> CFMutableDataRef;
}
unsafe extern "C" {
    pub fn OBEXAddNameHeader(name: CFStringRef, dictRef: CFMutableDictionaryRef) -> OBEXError;
}
unsafe extern "C" {
    pub fn OBEXAddDescriptionHeader(
        description: CFStringRef,
        dictRef: CFMutableDictionaryRef,
    ) -> OBEXError;
}
unsafe extern "C" {
    pub fn OBEXAddCountHeader(count: u32, dictRef: CFMutableDictionaryRef) -> OBEXError;
}
unsafe extern "C" {
    pub fn OBEXAddTime4ByteHeader(time4Byte: u32, dictRef: CFMutableDictionaryRef) -> OBEXError;
}
unsafe extern "C" {
    pub fn OBEXAddLengthHeader(length: u32, dictRef: CFMutableDictionaryRef) -> OBEXError;
}
unsafe extern "C" {
    pub fn OBEXAddTypeHeader(type_: CFStringRef, dictRef: CFMutableDictionaryRef) -> OBEXError;
}
unsafe extern "C" {
    pub fn OBEXAddTimeISOHeader(
        inHeaderData: *const ::std::os::raw::c_void,
        inHeaderDataLength: u32,
        dictRef: CFMutableDictionaryRef,
    ) -> OBEXError;
}
unsafe extern "C" {
    pub fn OBEXAddTargetHeader(
        inHeaderData: *const ::std::os::raw::c_void,
        inHeaderDataLength: u32,
        dictRef: CFMutableDictionaryRef,
    ) -> OBEXError;
}
unsafe extern "C" {
    pub fn OBEXAddHTTPHeader(
        inHeaderData: *const ::std::os::raw::c_void,
        inHeaderDataLength: u32,
        dictRef: CFMutableDictionaryRef,
    ) -> OBEXError;
}
unsafe extern "C" {
    pub fn OBEXAddBodyHeader(
        inHeaderData: *const ::std::os::raw::c_void,
        inHeaderDataLength: u32,
        isEndOfBody: Boolean,
        dictRef: CFMutableDictionaryRef,
    ) -> OBEXError;
}
unsafe extern "C" {
    pub fn OBEXAddWhoHeader(
        inHeaderData: *const ::std::os::raw::c_void,
        inHeaderDataLength: u32,
        dictRef: CFMutableDictionaryRef,
    ) -> OBEXError;
}
unsafe extern "C" {
    pub fn OBEXAddConnectionIDHeader(
        inHeaderData: *const ::std::os::raw::c_void,
        inHeaderDataLength: u32,
        dictRef: CFMutableDictionaryRef,
    ) -> OBEXError;
}
unsafe extern "C" {
    pub fn OBEXAddApplicationParameterHeader(
        inHeaderData: *const ::std::os::raw::c_void,
        inHeaderDataLength: u32,
        dictRef: CFMutableDictionaryRef,
    ) -> OBEXError;
}
unsafe extern "C" {
    pub fn OBEXAddByteSequenceHeader(
        inHeaderData: *const ::std::os::raw::c_void,
        inHeaderDataLength: u32,
        dictRef: CFMutableDictionaryRef,
    ) -> OBEXError;
}
unsafe extern "C" {
    pub fn OBEXAddObjectClassHeader(
        inHeaderData: *const ::std::os::raw::c_void,
        inHeaderDataLength: u32,
        dictRef: CFMutableDictionaryRef,
    ) -> OBEXError;
}
unsafe extern "C" {
    pub fn OBEXAddAuthorizationChallengeHeader(
        inHeaderData: *const ::std::os::raw::c_void,
        inHeaderDataLength: u32,
        dictRef: CFMutableDictionaryRef,
    ) -> OBEXError;
}
unsafe extern "C" {
    pub fn OBEXAddAuthorizationResponseHeader(
        inHeaderData: *const ::std::os::raw::c_void,
        inHeaderDataLength: u32,
        dictRef: CFMutableDictionaryRef,
    ) -> OBEXError;
}
unsafe extern "C" {
    pub fn OBEXAddUserDefinedHeader(
        inHeaderData: *const ::std::os::raw::c_void,
        inHeaderDataLength: u32,
        dictRef: CFMutableDictionaryRef,
    ) -> OBEXError;
}
unsafe extern "C" {
    pub fn IOBluetoothOBEXSessionCreateWithIOBluetoothSDPServiceRecordRef(
        inSDPServiceRef: IOBluetoothSDPServiceRecordRef,
        outSessionRef: *mut OBEXSessionRef,
    ) -> OBEXError;
}
unsafe extern "C" {
    pub fn IOBluetoothOBEXSessionCreateWithIOBluetoothDeviceRefAndChannelNumber(
        inDeviceRef: IOBluetoothDeviceRef,
        inChannelID: BluetoothRFCOMMChannelID,
        outSessionRef: *mut OBEXSessionRef,
    ) -> OBEXError;
}
unsafe extern "C" {
    pub fn IOBluetoothOBEXSessionCreateWithIncomingIOBluetoothRFCOMMChannel(
        inRFCOMMChannelRef: IOBluetoothRFCOMMChannelRef,
        inCallback: OBEXSessionEventCallback,
        inUserRefCon: *mut ::std::os::raw::c_void,
        outSessionRef: *mut OBEXSessionRef,
    ) -> OBEXError;
}
unsafe extern "C" {
    pub fn IOBluetoothOBEXSessionOpenTransportConnection(
        inSessionRef: OBEXSessionRef,
        inCallback: IOBluetoothOBEXSessionOpenConnectionCallback,
        inUserRefCon: *mut ::std::os::raw::c_void,
    ) -> OBEXError;
}
unsafe extern "C" {
    pub static mut kFTSProgressBytesTransferredKey: CFStringRef;
}
unsafe extern "C" {
    pub static mut kFTSProgressBytesTotalKey: CFStringRef;
}
unsafe extern "C" {
    pub static mut kFTSProgressPercentageKey: CFStringRef;
}
unsafe extern "C" {
    pub static mut kFTSProgressPrecentageKey: CFStringRef;
}
unsafe extern "C" {
    pub static mut kFTSProgressEstimatedTimeKey: CFStringRef;
}
unsafe extern "C" {
    pub static mut kFTSProgressTimeElapsedKey: CFStringRef;
}
unsafe extern "C" {
    pub static mut kFTSProgressTransferRateKey: CFStringRef;
}
unsafe extern "C" {
    pub static mut kFTSListingNameKey: CFStringRef;
}
unsafe extern "C" {
    pub static mut kFTSListingTypeKey: CFStringRef;
}
unsafe extern "C" {
    pub static mut kFTSListingSizeKey: CFStringRef;
}
unsafe extern "C" {
    pub static IOBluetoothHandsFreeIndicatorService: NSString;
}
unsafe extern "C" {
    pub static IOBluetoothHandsFreeIndicatorCall: NSString;
}
unsafe extern "C" {
    pub static IOBluetoothHandsFreeIndicatorCallSetup: NSString;
}
unsafe extern "C" {
    pub static IOBluetoothHandsFreeIndicatorCallHeld: NSString;
}
unsafe extern "C" {
    pub static IOBluetoothHandsFreeIndicatorSignal: NSString;
}
unsafe extern "C" {
    pub static IOBluetoothHandsFreeIndicatorRoam: NSString;
}
unsafe extern "C" {
    pub static IOBluetoothHandsFreeIndicatorBattChg: NSString;
}
unsafe extern "C" {
    pub static IOBluetoothHandsFreeCallIndex: NSString;
}
unsafe extern "C" {
    pub static IOBluetoothHandsFreeCallDirection: NSString;
}
unsafe extern "C" {
    pub static IOBluetoothHandsFreeCallStatus: NSString;
}
unsafe extern "C" {
    pub static IOBluetoothHandsFreeCallMode: NSString;
}
unsafe extern "C" {
    pub static IOBluetoothHandsFreeCallMultiparty: NSString;
}
unsafe extern "C" {
    pub static IOBluetoothHandsFreeCallNumber: NSString;
}
unsafe extern "C" {
    pub static IOBluetoothHandsFreeCallType: NSString;
}
unsafe extern "C" {
    pub static IOBluetoothHandsFreeCallName: NSString;
}
unsafe extern "C" {
    pub static IOBluetoothPDUServicCenterAddress: NSString;
}
unsafe extern "C" {
    pub static IOBluetoothPDUServiceCenterAddressType: NSString;
}
unsafe extern "C" {
    pub static IOBluetoothPDUType: NSString;
}
unsafe extern "C" {
    pub static IOBluetoothPDUOriginatingAddress: NSString;
}
unsafe extern "C" {
    pub static IOBluetoothPDUOriginatingAddressType: NSString;
}
unsafe extern "C" {
    pub static IOBluetoothPDUProtocolID: NSString;
}
unsafe extern "C" {
    pub static IOBluetoothPDUTimestamp: NSString;
}
unsafe extern "C" {
    pub static IOBluetoothPDUEncoding: NSString;
}
unsafe extern "C" {
    pub static IOBluetoothPDUUserData: NSString;
}

unsafe impl objc2::encode::RefEncode for BluetoothDeviceAddress {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for BluetoothDeviceAddress {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("BluetoothDeviceAddress", &[]);
}
unsafe impl objc2::encode::RefEncode for BluetoothKey {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for BluetoothKey {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("BluetoothKey", &[]);
}
unsafe impl objc2::encode::RefEncode for BluetoothIRK {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for BluetoothIRK {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("BluetoothIRK", &[]);
}
unsafe impl objc2::encode::RefEncode for BluetoothPINCode {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for BluetoothPINCode {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("BluetoothPINCode", &[]);
}
unsafe impl objc2::encode::RefEncode for BluetoothSetEventMask {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for BluetoothSetEventMask {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("BluetoothSetEventMask", &[]);
}
unsafe impl objc2::encode::RefEncode for BluetoothL2CAPQualityOfServiceOptions {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for BluetoothL2CAPQualityOfServiceOptions {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("BluetoothL2CAPQualityOfServiceOptions", &[]);
}
unsafe impl objc2::encode::RefEncode for BluetoothL2CAPRetransmissionAndFlowControlOptions {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for BluetoothL2CAPRetransmissionAndFlowControlOptions {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("BluetoothL2CAPRetransmissionAndFlowControlOptions", &[]);
}
unsafe impl objc2::encode::RefEncode for BluetoothHCISupportedCommands {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for BluetoothHCISupportedCommands {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("BluetoothHCISupportedCommands", &[]);
}
unsafe impl objc2::encode::RefEncode for BluetoothHCISupportedFeatures {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for BluetoothHCISupportedFeatures {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("BluetoothHCISupportedFeatures", &[]);
}
unsafe impl objc2::encode::RefEncode for BluetoothHCIExtendedFeaturesInfo {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for BluetoothHCIExtendedFeaturesInfo {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("BluetoothHCIExtendedFeaturesInfo", &[]);
}
unsafe impl objc2::encode::RefEncode for BluetoothEventFilterCondition {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for BluetoothEventFilterCondition {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("BluetoothEventFilterCondition", &[]);
}
unsafe impl objc2::encode::RefEncode for BluetoothHCIFailedContactInfo {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for BluetoothHCIFailedContactInfo {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("BluetoothHCIFailedContactInfo", &[]);
}
unsafe impl objc2::encode::RefEncode for BluetoothHCIRSSIInfo {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for BluetoothHCIRSSIInfo {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("BluetoothHCIRSSIInfo", &[]);
}
unsafe impl objc2::encode::RefEncode for BluetoothHCILinkQualityInfo {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for BluetoothHCILinkQualityInfo {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("BluetoothHCILinkQualityInfo", &[]);
}
unsafe impl objc2::encode::RefEncode for BluetoothHCIEncryptionKeySizeInfo {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for BluetoothHCIEncryptionKeySizeInfo {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("BluetoothHCIEncryptionKeySizeInfo", &[]);
}
unsafe impl objc2::encode::RefEncode for BluetoothHCIRoleInfo {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for BluetoothHCIRoleInfo {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("BluetoothHCIRoleInfo", &[]);
}
unsafe impl objc2::encode::RefEncode for BluetoothHCILinkPolicySettingsInfo {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for BluetoothHCILinkPolicySettingsInfo {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("BluetoothHCILinkPolicySettingsInfo", &[]);
}
unsafe impl objc2::encode::RefEncode for BluetoothHCIQualityOfServiceSetupParams {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for BluetoothHCIQualityOfServiceSetupParams {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("BluetoothHCIQualityOfServiceSetupParams", &[]);
}
unsafe impl objc2::encode::RefEncode for BluetoothHCISetupSynchronousConnectionParams {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for BluetoothHCISetupSynchronousConnectionParams {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("BluetoothHCISetupSynchronousConnectionParams", &[]);
}
unsafe impl objc2::encode::RefEncode for BluetoothHCIAcceptSynchronousConnectionRequestParams {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for BluetoothHCIAcceptSynchronousConnectionRequestParams {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("BluetoothHCIAcceptSynchronousConnectionRequestParams", &[]);
}
unsafe impl objc2::encode::RefEncode for BluetoothHCIEnhancedSetupSynchronousConnectionParams {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for BluetoothHCIEnhancedSetupSynchronousConnectionParams {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("BluetoothHCIEnhancedSetupSynchronousConnectionParams", &[]);
}
unsafe impl objc2::encode::RefEncode for BluetoothHCIEnhancedAcceptSynchronousConnectionRequestParams {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for BluetoothHCIEnhancedAcceptSynchronousConnectionRequestParams {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("BluetoothHCIEnhancedAcceptSynchronousConnectionRequestParams", &[]);
}
unsafe impl objc2::encode::RefEncode for BluetoothReadClockInfo {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for BluetoothReadClockInfo {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("BluetoothReadClockInfo", &[]);
}
unsafe impl objc2::encode::RefEncode for BluetoothHCIEventFlowSpecificationData {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for BluetoothHCIEventFlowSpecificationData {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("BluetoothHCIEventFlowSpecificationData", &[]);
}
unsafe impl objc2::encode::RefEncode for BluetoothHCIVersionInfo {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for BluetoothHCIVersionInfo {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("BluetoothHCIVersionInfo", &[]);
}
unsafe impl objc2::encode::RefEncode for BluetoothHCIBufferSize {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for BluetoothHCIBufferSize {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("BluetoothHCIBufferSize", &[]);
}
unsafe impl objc2::encode::RefEncode for BluetoothHCILEBufferSize {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for BluetoothHCILEBufferSize {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("BluetoothHCILEBufferSize", &[]);
}
unsafe impl objc2::encode::RefEncode for BluetoothHCIStoredLinkKeysInfo {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for BluetoothHCIStoredLinkKeysInfo {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("BluetoothHCIStoredLinkKeysInfo", &[]);
}
unsafe impl objc2::encode::RefEncode for BluetoothHCIScanActivity {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for BluetoothHCIScanActivity {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("BluetoothHCIScanActivity", &[]);
}
unsafe impl objc2::encode::RefEncode for BluetoothHCIInquiryAccessCode {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for BluetoothHCIInquiryAccessCode {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("BluetoothHCIInquiryAccessCode", &[]);
}
unsafe impl objc2::encode::RefEncode for BluetoothHCICurrentInquiryAccessCodes {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for BluetoothHCICurrentInquiryAccessCodes {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("BluetoothHCICurrentInquiryAccessCodes", &[]);
}
unsafe impl objc2::encode::RefEncode for BluetoothHCICurrentInquiryAccessCodesForWrite {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for BluetoothHCICurrentInquiryAccessCodesForWrite {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("BluetoothHCICurrentInquiryAccessCodesForWrite", &[]);
}
unsafe impl objc2::encode::RefEncode for BluetoothHCILinkSupervisionTimeout {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for BluetoothHCILinkSupervisionTimeout {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("BluetoothHCILinkSupervisionTimeout", &[]);
}
unsafe impl objc2::encode::RefEncode for BluetoothHCITransmitPowerLevelInfo {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for BluetoothHCITransmitPowerLevelInfo {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("BluetoothHCITransmitPowerLevelInfo", &[]);
}
unsafe impl objc2::encode::RefEncode for BluetoothHCIAutomaticFlushTimeoutInfo {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for BluetoothHCIAutomaticFlushTimeoutInfo {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("BluetoothHCIAutomaticFlushTimeoutInfo", &[]);
}
unsafe impl objc2::encode::RefEncode for BluetoothTransportInfo {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for BluetoothTransportInfo {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("BluetoothTransportInfo", &[]);
}
unsafe impl objc2::encode::RefEncode for BluetoothHCIInquiryResult {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for BluetoothHCIInquiryResult {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("BluetoothHCIInquiryResult", &[]);
}
unsafe impl objc2::encode::RefEncode for BluetoothHCIInquiryResults {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for BluetoothHCIInquiryResults {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("BluetoothHCIInquiryResults", &[]);
}
unsafe impl objc2::encode::RefEncode for BluetoothHCIInquiryWithRSSIResult {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for BluetoothHCIInquiryWithRSSIResult {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("BluetoothHCIInquiryWithRSSIResult", &[]);
}
unsafe impl objc2::encode::RefEncode for BluetoothHCIInquiryWithRSSIResults {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for BluetoothHCIInquiryWithRSSIResults {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("BluetoothHCIInquiryWithRSSIResults", &[]);
}
unsafe impl objc2::encode::RefEncode for BluetoothHCIExtendedInquiryResponse {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for BluetoothHCIExtendedInquiryResponse {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("BluetoothHCIExtendedInquiryResponse", &[]);
}
unsafe impl objc2::encode::RefEncode for BluetoothHCIReadExtendedInquiryResponseResults {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for BluetoothHCIReadExtendedInquiryResponseResults {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("BluetoothHCIReadExtendedInquiryResponseResults", &[]);
}
unsafe impl objc2::encode::RefEncode for BluetoothHCIExtendedInquiryResult {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for BluetoothHCIExtendedInquiryResult {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("BluetoothHCIExtendedInquiryResult", &[]);
}
unsafe impl objc2::encode::RefEncode for BluetoothHCIReadLMPHandleResults {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for BluetoothHCIReadLMPHandleResults {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("BluetoothHCIReadLMPHandleResults", &[]);
}
unsafe impl objc2::encode::RefEncode for BluetoothHCISimplePairingOOBData {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for BluetoothHCISimplePairingOOBData {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("BluetoothHCISimplePairingOOBData", &[]);
}
unsafe impl objc2::encode::RefEncode for BluetoothHCIReadLocalOOBDataResults {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for BluetoothHCIReadLocalOOBDataResults {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("BluetoothHCIReadLocalOOBDataResults", &[]);
}
unsafe impl objc2::encode::RefEncode for BluetoothIOCapabilityResponse {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for BluetoothIOCapabilityResponse {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("BluetoothIOCapabilityResponse", &[]);
}
unsafe impl objc2::encode::RefEncode for BluetoothUserPasskeyNotification {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for BluetoothUserPasskeyNotification {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("BluetoothUserPasskeyNotification", &[]);
}
unsafe impl objc2::encode::RefEncode for BluetoothKeypressNotification {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for BluetoothKeypressNotification {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("BluetoothKeypressNotification", &[]);
}
unsafe impl objc2::encode::RefEncode for BluetoothRemoteHostSupportedFeaturesNotification {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for BluetoothRemoteHostSupportedFeaturesNotification {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("BluetoothRemoteHostSupportedFeaturesNotification", &[]);
}
unsafe impl objc2::encode::RefEncode for BluetoothAFHHostChannelClassification {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for BluetoothAFHHostChannelClassification {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("BluetoothAFHHostChannelClassification", &[]);
}
unsafe impl objc2::encode::RefEncode for BluetoothAFHResults {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for BluetoothAFHResults {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("BluetoothAFHResults", &[]);
}
unsafe impl objc2::encode::RefEncode for BluetoothUserConfirmationRequest {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for BluetoothUserConfirmationRequest {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("BluetoothUserConfirmationRequest", &[]);
}
unsafe impl objc2::encode::RefEncode for BluetoothHCIEventSimplePairingCompleteResults {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for BluetoothHCIEventSimplePairingCompleteResults {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("BluetoothHCIEventSimplePairingCompleteResults", &[]);
}
unsafe impl objc2::encode::RefEncode for BluetoothSynchronousConnectionInfo {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for BluetoothSynchronousConnectionInfo {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("BluetoothSynchronousConnectionInfo", &[]);
}
unsafe impl objc2::encode::RefEncode for BluetoothEnhancedSynchronousConnectionInfo {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for BluetoothEnhancedSynchronousConnectionInfo {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("BluetoothEnhancedSynchronousConnectionInfo", &[]);
}
unsafe impl objc2::encode::RefEncode for BluetoothHCIEventSynchronousConnectionCompleteResults {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for BluetoothHCIEventSynchronousConnectionCompleteResults {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("BluetoothHCIEventSynchronousConnectionCompleteResults", &[]);
}
unsafe impl objc2::encode::RefEncode for BluetoothHCIEventSynchronousConnectionChangedResults {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for BluetoothHCIEventSynchronousConnectionChangedResults {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("BluetoothHCIEventSynchronousConnectionChangedResults", &[]);
}
unsafe impl objc2::encode::RefEncode for BluetoothHCIEventConnectionCompleteResults {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for BluetoothHCIEventConnectionCompleteResults {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("BluetoothHCIEventConnectionCompleteResults", &[]);
}
unsafe impl objc2::encode::RefEncode for BluetoothHCIEventLEConnectionCompleteResults {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for BluetoothHCIEventLEConnectionCompleteResults {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("BluetoothHCIEventLEConnectionCompleteResults", &[]);
}
unsafe impl objc2::encode::RefEncode for BluetoothHCIEventLEEnhancedConnectionCompleteResults {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for BluetoothHCIEventLEEnhancedConnectionCompleteResults {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("BluetoothHCIEventLEEnhancedConnectionCompleteResults", &[]);
}
unsafe impl objc2::encode::RefEncode for BluetoothHCIEventLEConnectionUpdateCompleteResults {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for BluetoothHCIEventLEConnectionUpdateCompleteResults {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("BluetoothHCIEventLEConnectionUpdateCompleteResults", &[]);
}
unsafe impl objc2::encode::RefEncode for BluetoothHCIEventLEReadRemoteUsedFeaturesCompleteResults {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for BluetoothHCIEventLEReadRemoteUsedFeaturesCompleteResults {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("BluetoothHCIEventLEReadRemoteUsedFeaturesCompleteResults", &[]);
}
unsafe impl objc2::encode::RefEncode for BluetoothHCIEventDisconnectionCompleteResults {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for BluetoothHCIEventDisconnectionCompleteResults {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("BluetoothHCIEventDisconnectionCompleteResults", &[]);
}
unsafe impl objc2::encode::RefEncode for BluetoothHCIEventReadSupportedFeaturesResults {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for BluetoothHCIEventReadSupportedFeaturesResults {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("BluetoothHCIEventReadSupportedFeaturesResults", &[]);
}
unsafe impl objc2::encode::RefEncode for BluetoothHCIEventReadExtendedFeaturesResults {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for BluetoothHCIEventReadExtendedFeaturesResults {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("BluetoothHCIEventReadExtendedFeaturesResults", &[]);
}
unsafe impl objc2::encode::RefEncode for BluetoothHCIEventReadRemoteVersionInfoResults {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for BluetoothHCIEventReadRemoteVersionInfoResults {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("BluetoothHCIEventReadRemoteVersionInfoResults", &[]);
}
unsafe impl objc2::encode::RefEncode for BluetoothHCIEventRemoteNameRequestResults {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for BluetoothHCIEventRemoteNameRequestResults {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("BluetoothHCIEventRemoteNameRequestResults", &[]);
}
unsafe impl objc2::encode::RefEncode for BluetoothHCIEventReadClockOffsetResults {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for BluetoothHCIEventReadClockOffsetResults {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("BluetoothHCIEventReadClockOffsetResults", &[]);
}
unsafe impl objc2::encode::RefEncode for BluetoothHCIEventConnectionRequestResults {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for BluetoothHCIEventConnectionRequestResults {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("BluetoothHCIEventConnectionRequestResults", &[]);
}
unsafe impl objc2::encode::RefEncode for BluetoothHCIEventLinkKeyNotificationResults {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for BluetoothHCIEventLinkKeyNotificationResults {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("BluetoothHCIEventLinkKeyNotificationResults", &[]);
}
unsafe impl objc2::encode::RefEncode for BluetoothHCIEventMaxSlotsChangeResults {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for BluetoothHCIEventMaxSlotsChangeResults {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("BluetoothHCIEventMaxSlotsChangeResults", &[]);
}
unsafe impl objc2::encode::RefEncode for BluetoothHCIEventModeChangeResults {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for BluetoothHCIEventModeChangeResults {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("BluetoothHCIEventModeChangeResults", &[]);
}
unsafe impl objc2::encode::RefEncode for BluetoothHCIEventReturnLinkKeysResults {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for BluetoothHCIEventReturnLinkKeysResults {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("BluetoothHCIEventReturnLinkKeysResults", &[]);
}
unsafe impl objc2::encode::RefEncode for BluetoothHCIEventReturnLinkKeysResults__bindgen_ty_1 {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for BluetoothHCIEventReturnLinkKeysResults__bindgen_ty_1 {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("BluetoothHCIEventReturnLinkKeysResults__bindgen_ty_1", &[]);
}
unsafe impl objc2::encode::RefEncode for BluetoothHCIEventAuthenticationCompleteResults {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for BluetoothHCIEventAuthenticationCompleteResults {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("BluetoothHCIEventAuthenticationCompleteResults", &[]);
}
unsafe impl objc2::encode::RefEncode for BluetoothHCIEventEncryptionChangeResults {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for BluetoothHCIEventEncryptionChangeResults {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("BluetoothHCIEventEncryptionChangeResults", &[]);
}
unsafe impl objc2::encode::RefEncode for BluetoothHCIEventChangeConnectionLinkKeyCompleteResults {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for BluetoothHCIEventChangeConnectionLinkKeyCompleteResults {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("BluetoothHCIEventChangeConnectionLinkKeyCompleteResults", &[]);
}
unsafe impl objc2::encode::RefEncode for BluetoothHCIEventMasterLinkKeyCompleteResults {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for BluetoothHCIEventMasterLinkKeyCompleteResults {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("BluetoothHCIEventMasterLinkKeyCompleteResults", &[]);
}
unsafe impl objc2::encode::RefEncode for BluetoothHCIEventQoSSetupCompleteResults {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for BluetoothHCIEventQoSSetupCompleteResults {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("BluetoothHCIEventQoSSetupCompleteResults", &[]);
}
unsafe impl objc2::encode::RefEncode for BluetoothHCIEventHardwareErrorResults {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for BluetoothHCIEventHardwareErrorResults {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("BluetoothHCIEventHardwareErrorResults", &[]);
}
unsafe impl objc2::encode::RefEncode for BluetoothHCIEventFlushOccurredResults {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for BluetoothHCIEventFlushOccurredResults {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("BluetoothHCIEventFlushOccurredResults", &[]);
}
unsafe impl objc2::encode::RefEncode for BluetoothHCIEventRoleChangeResults {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for BluetoothHCIEventRoleChangeResults {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("BluetoothHCIEventRoleChangeResults", &[]);
}
unsafe impl objc2::encode::RefEncode for BluetoothHCIEventDataBufferOverflowResults {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for BluetoothHCIEventDataBufferOverflowResults {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("BluetoothHCIEventDataBufferOverflowResults", &[]);
}
unsafe impl objc2::encode::RefEncode for BluetoothHCIEventConnectionPacketTypeResults {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for BluetoothHCIEventConnectionPacketTypeResults {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("BluetoothHCIEventConnectionPacketTypeResults", &[]);
}
unsafe impl objc2::encode::RefEncode for BluetoothHCIEventReadRemoteSupportedFeaturesResults {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for BluetoothHCIEventReadRemoteSupportedFeaturesResults {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("BluetoothHCIEventReadRemoteSupportedFeaturesResults", &[]);
}
unsafe impl objc2::encode::RefEncode for BluetoothHCIEventReadRemoteExtendedFeaturesResults {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for BluetoothHCIEventReadRemoteExtendedFeaturesResults {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("BluetoothHCIEventReadRemoteExtendedFeaturesResults", &[]);
}
unsafe impl objc2::encode::RefEncode for BluetoothHCIEventQoSViolationResults {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for BluetoothHCIEventQoSViolationResults {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("BluetoothHCIEventQoSViolationResults", &[]);
}
unsafe impl objc2::encode::RefEncode for BluetoothHCIEventPageScanModeChangeResults {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for BluetoothHCIEventPageScanModeChangeResults {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("BluetoothHCIEventPageScanModeChangeResults", &[]);
}
unsafe impl objc2::encode::RefEncode for BluetoothHCIEventPageScanRepetitionModeChangeResults {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for BluetoothHCIEventPageScanRepetitionModeChangeResults {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("BluetoothHCIEventPageScanRepetitionModeChangeResults", &[]);
}
unsafe impl objc2::encode::RefEncode for BluetoothHCIEventVendorSpecificResults {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for BluetoothHCIEventVendorSpecificResults {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("BluetoothHCIEventVendorSpecificResults", &[]);
}
unsafe impl objc2::encode::RefEncode for BluetoothHCIEventEncryptionKeyRefreshCompleteResults {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for BluetoothHCIEventEncryptionKeyRefreshCompleteResults {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("BluetoothHCIEventEncryptionKeyRefreshCompleteResults", &[]);
}
unsafe impl objc2::encode::RefEncode for BluetoothHCIEventSniffSubratingResults {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for BluetoothHCIEventSniffSubratingResults {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("BluetoothHCIEventSniffSubratingResults", &[]);
}
unsafe impl objc2::encode::RefEncode for BluetoothHCIEventLEMetaResults {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for BluetoothHCIEventLEMetaResults {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("BluetoothHCIEventLEMetaResults", &[]);
}
unsafe impl objc2::encode::RefEncode for BluetoothHCIEventLELongTermKeyRequestResults {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for BluetoothHCIEventLELongTermKeyRequestResults {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("BluetoothHCIEventLELongTermKeyRequestResults", &[]);
}
unsafe impl objc2::encode::RefEncode for BluetoothHCIRequestCallbackInfo {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for BluetoothHCIRequestCallbackInfo {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("BluetoothHCIRequestCallbackInfo", &[]);
}
unsafe impl objc2::encode::RefEncode for OpaqueIOBluetoothObjectRef {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for OpaqueIOBluetoothObjectRef {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("OpaqueIOBluetoothObjectRef", &[]);
}
unsafe impl objc2::encode::RefEncode for IOBluetoothDeviceSearchDeviceAttributes {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for IOBluetoothDeviceSearchDeviceAttributes {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("IOBluetoothDeviceSearchDeviceAttributes", &[]);
}
unsafe impl objc2::encode::RefEncode for IOBluetoothDeviceSearchAttributes {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for IOBluetoothDeviceSearchAttributes {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("IOBluetoothDeviceSearchAttributes", &[]);
}
unsafe impl objc2::encode::RefEncode for IOBluetoothObject {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for IOBluetoothObject {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for IOBluetoothUserNotification {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for IOBluetoothUserNotification {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for IOBluetoothDevice {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for IOBluetoothDevice {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for IOBluetoothDeviceInquiry {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for IOBluetoothDeviceInquiry {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for IOBluetoothDevicePair {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for IOBluetoothDevicePair {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for IOBluetoothHostController {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for IOBluetoothHostController {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for IOBluetoothL2CAPChannel {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for IOBluetoothL2CAPChannel {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for IOBluetoothRFCOMMChannel {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for IOBluetoothRFCOMMChannel {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for IOBluetoothSDPDataElement {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for IOBluetoothSDPDataElement {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for IOBluetoothSDPServiceAttribute {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for IOBluetoothSDPServiceAttribute {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for IOBluetoothSDPServiceRecord {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for IOBluetoothSDPServiceRecord {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for IOBluetoothSDPUUID {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for IOBluetoothSDPUUID {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for OpaqueOBEXSessionRef {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for OpaqueOBEXSessionRef {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("OpaqueOBEXSessionRef", &[]);
}
unsafe impl objc2::encode::RefEncode for OBEXConnectCommandResponseData {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for OBEXConnectCommandResponseData {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("OBEXConnectCommandResponseData", &[]);
}
unsafe impl objc2::encode::RefEncode for OBEXDisconnectCommandResponseData {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for OBEXDisconnectCommandResponseData {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("OBEXDisconnectCommandResponseData", &[]);
}
unsafe impl objc2::encode::RefEncode for OBEXPutCommandResponseData {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for OBEXPutCommandResponseData {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("OBEXPutCommandResponseData", &[]);
}
unsafe impl objc2::encode::RefEncode for OBEXGetCommandResponseData {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for OBEXGetCommandResponseData {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("OBEXGetCommandResponseData", &[]);
}
unsafe impl objc2::encode::RefEncode for OBEXSetPathCommandResponseData {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for OBEXSetPathCommandResponseData {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("OBEXSetPathCommandResponseData", &[]);
}
unsafe impl objc2::encode::RefEncode for OBEXAbortCommandResponseData {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for OBEXAbortCommandResponseData {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("OBEXAbortCommandResponseData", &[]);
}
unsafe impl objc2::encode::RefEncode for OBEXConnectCommandData {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for OBEXConnectCommandData {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("OBEXConnectCommandData", &[]);
}
unsafe impl objc2::encode::RefEncode for OBEXDisconnectCommandData {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for OBEXDisconnectCommandData {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("OBEXDisconnectCommandData", &[]);
}
unsafe impl objc2::encode::RefEncode for OBEXPutCommandData {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for OBEXPutCommandData {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("OBEXPutCommandData", &[]);
}
unsafe impl objc2::encode::RefEncode for OBEXGetCommandData {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for OBEXGetCommandData {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("OBEXGetCommandData", &[]);
}
unsafe impl objc2::encode::RefEncode for OBEXSetPathCommandData {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for OBEXSetPathCommandData {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("OBEXSetPathCommandData", &[]);
}
unsafe impl objc2::encode::RefEncode for OBEXAbortCommandData {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for OBEXAbortCommandData {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("OBEXAbortCommandData", &[]);
}
unsafe impl objc2::encode::RefEncode for OBEXErrorData {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for OBEXErrorData {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("OBEXErrorData", &[]);
}
unsafe impl objc2::encode::RefEncode for OBEXSessionEvent {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for OBEXSessionEvent {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("OBEXSessionEvent", &[]);
}
unsafe impl objc2::encode::RefEncode for OBEXSessionEvent__bindgen_ty_1 {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for OBEXSessionEvent__bindgen_ty_1 {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("OBEXSessionEvent__bindgen_ty_1", &[]);
}
unsafe impl objc2::encode::RefEncode for OBEXSession {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for OBEXSession {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for OBEXTransportEvent {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for OBEXTransportEvent {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("OBEXTransportEvent", &[]);
}
unsafe impl objc2::encode::RefEncode for IOBluetoothOBEXSession {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for IOBluetoothOBEXSession {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for OBEXFileTransferServices {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for OBEXFileTransferServices {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for IOBluetoothHandsFree {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for IOBluetoothHandsFree {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for IOBluetoothHandsFreeAudioGateway {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for IOBluetoothHandsFreeAudioGateway {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for IOBluetoothHandsFreeDevice {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for IOBluetoothHandsFreeDevice {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
