#[allow(unused_imports)]
use crate::ApplicationServices::*;
#[allow(unused_imports)]
use crate::CoreFoundation::*;
#[allow(unused_imports)]
use crate::CoreGraphics::*;
#[allow(unused_imports)]
use crate::CoreServices::*;
#[allow(unused_imports)]
use crate::Security::*;

#[allow(unused_imports)]
use objc2::msg_send;
pub type DVDErrorCode = OSStatus;
pub type DVDState = OSStatus;
pub type DVDMenu = UInt32;
pub type DVDButtonIndex = SInt32;
pub type DVDUserNavigation = UInt32;
pub type DVDTimePosition = UInt32;
pub type DVDTimeCode = SInt16;
pub type DVDScanDirection = SInt8;
pub type DVDScanRate = SInt16;
pub type DVDAspectRatio = SInt16;
pub type DVDFormat = SInt16;
pub type DVDAudioMode = SInt32;
pub type DVDAudioFormat = SInt16;
pub type DVDLanguageCode = OSType;
pub type DVDAudioExtensionCode = OSType;
pub type DVDSubpictureExtensionCode = OSType;
pub type DVDRegionCode = UInt32;
pub type DVDDomainCode = UInt32;
pub type DVDUOPCode = UInt32;
pub type DVDEventCode = UInt32;
pub type DVDEventValue = ::std::os::raw::c_ulong;
pub type DVDEventCallBackRef = *mut ::std::os::raw::c_void;
pub type DVDFatalErrCallBackFunctionPtr = ::std::option::Option<
    unsafe extern "C" fn(inError: DVDErrorCode, inRefCon: *mut ::std::os::raw::c_void),
>;
pub type DVDEventCallBackFunctionPtr = ::std::option::Option<
    unsafe extern "C" fn(
        inEventCode: DVDEventCode,
        inEventValue1: DVDEventValue,
        inEventValue2: DVDEventValue,
        inRefCon: *mut ::std::os::raw::c_void,
    ),
>;
unsafe extern "C" {
    pub fn DVDInitialize() -> OSStatus;
}
unsafe extern "C" {
    pub fn DVDDispose() -> OSStatus;
}
unsafe extern "C" {
    pub fn DVDIsValidMediaRef(inRef: *mut FSRef, outIsValid: *mut Boolean) -> OSStatus;
}
unsafe extern "C" {
    pub fn DVDIsValidMediaURL(inRef: CFURLRef, outIsValid: *mut Boolean) -> OSStatus;
}
unsafe extern "C" {
    pub fn DVDHasMedia(outHasMedia: *mut Boolean) -> OSStatus;
}
unsafe extern "C" {
    pub fn DVDOpenMediaFile(inFile: *mut FSRef) -> OSStatus;
}
unsafe extern "C" {
    pub fn DVDOpenMediaFileWithURL(inFile: CFURLRef) -> OSStatus;
}
unsafe extern "C" {
    pub fn DVDCloseMediaFile() -> OSStatus;
}
unsafe extern "C" {
    pub fn DVDOpenMediaVolume(inVolume: *mut FSRef) -> OSStatus;
}
unsafe extern "C" {
    pub fn DVDOpenMediaVolumeWithURL(inVolume: CFURLRef) -> OSStatus;
}
unsafe extern "C" {
    pub fn DVDCloseMediaVolume() -> OSStatus;
}
unsafe extern "C" {
    pub fn DVDIsSupportedDisplay(
        inDisplay: CGDirectDisplayID,
        outSupported: *mut Boolean,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn DVDSwitchToDisplay(
        newDisplay: CGDirectDisplayID,
        outSupported: *mut Boolean,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn DVDSetVideoDisplay(inDisplay: CGDirectDisplayID) -> OSStatus;
}
unsafe extern "C" {
    pub fn DVDGetVideoDisplay(outDisplay: *mut CGDirectDisplayID) -> OSStatus;
}
unsafe extern "C" {
    pub fn DVDSetVideoWindowID(inVidWindowID: UInt32) -> OSStatus;
}
unsafe extern "C" {
    pub fn DVDGetVideoWindowID(outVidWindowID: *mut UInt32) -> OSStatus;
}
unsafe extern "C" {
    pub fn DVDGetNativeVideoSize(outWidth: *mut UInt16, outHeight: *mut UInt16) -> OSStatus;
}
unsafe extern "C" {
    pub fn DVDGetAspectRatio(outRatio: *mut DVDAspectRatio) -> OSStatus;
}
unsafe extern "C" {
    pub fn DVDSetAspectRatio(inRatio: DVDAspectRatio) -> OSStatus;
}
unsafe extern "C" {
    pub fn DVDGetFormatStandard(outFormat: *mut DVDFormat) -> OSStatus;
}
unsafe extern "C" {
    pub fn DVDSetVideoWindowRef(inWindowRef: WindowRef) -> OSStatus;
}
unsafe extern "C" {
    pub fn DVDGetVideoWindowRef(outWindowRef: *mut WindowRef) -> OSStatus;
}
unsafe extern "C" {
    pub fn DVDSetVideoCGBounds(inRect: *mut CGRect) -> OSStatus;
}
unsafe extern "C" {
    pub fn DVDGetVideoCGBounds(outRect: *mut CGRect) -> OSStatus;
}
unsafe extern "C" {
    pub fn DVDGetAudioStreamFormat(
        outFormat: *mut DVDAudioFormat,
        outBitsPerSample: *mut UInt32,
        outSamplesPerSecond: *mut UInt32,
        outChannels: *mut UInt32,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn DVDGetAudioStreamFormatByStream(
        inStreamNum: UInt32,
        outFormat: *mut DVDAudioFormat,
        outBitsPerSample: *mut UInt32,
        outSamplesPerSecond: *mut UInt32,
        outChannels: *mut UInt32,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn DVDGetAudioOutputModeCapabilities(outModes: *mut DVDAudioMode) -> OSStatus;
}
unsafe extern "C" {
    pub fn DVDSetAudioOutputMode(inMode: DVDAudioMode) -> OSStatus;
}
unsafe extern "C" {
    pub fn DVDGetAudioOutputMode(outMode: *mut DVDAudioMode) -> OSStatus;
}
unsafe extern "C" {
    pub fn DVDGetSPDIFDataOutDeviceCount(outCount: *mut UInt32) -> OSStatus;
}
unsafe extern "C" {
    pub fn DVDGetSPDIFDataOutDeviceCFName(inIndex: UInt32, outName: *mut CFStringRef) -> OSStatus;
}
unsafe extern "C" {
    pub fn DVDSetSPDIFDataOutDevice(inIndex: UInt32) -> OSStatus;
}
unsafe extern "C" {
    pub fn DVDGetSPDIFDataOutDevice(outIndex: *mut UInt32) -> OSStatus;
}
unsafe extern "C" {
    pub fn DVDSetTime(
        inTimeCode: DVDTimeCode,
        inTime: DVDTimePosition,
        inFrames: UInt16,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn DVDGetTime(
        inTimeCode: DVDTimeCode,
        outTime: *mut DVDTimePosition,
        outFrames: *mut UInt16,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn DVDGetState(outState: *mut DVDState) -> OSStatus;
}
unsafe extern "C" {
    pub fn DVDIdle() -> OSStatus;
}
unsafe extern "C" {
    pub fn DVDUpdateVideo() -> OSStatus;
}
unsafe extern "C" {
    pub fn DVDIsPlaying(outIsPlaying: *mut Boolean) -> OSStatus;
}
unsafe extern "C" {
    pub fn DVDIsPaused(outIsPaused: *mut Boolean) -> OSStatus;
}
unsafe extern "C" {
    pub fn DVDPlay() -> OSStatus;
}
unsafe extern "C" {
    pub fn DVDPause() -> OSStatus;
}
unsafe extern "C" {
    pub fn DVDResume() -> OSStatus;
}
unsafe extern "C" {
    pub fn DVDStop() -> OSStatus;
}
unsafe extern "C" {
    pub fn DVDScan(inRate: DVDScanRate, inDirection: DVDScanDirection) -> OSStatus;
}
unsafe extern "C" {
    pub fn DVDGetScanRate(
        outRate: *mut DVDScanRate,
        outDirection: *mut DVDScanDirection,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn DVDStepFrame(inDirection: DVDScanDirection) -> OSStatus;
}
unsafe extern "C" {
    pub fn DVDIsMuted(outIsMuted: *mut Boolean) -> OSStatus;
}
unsafe extern "C" {
    pub fn DVDMute(inMute: Boolean) -> OSStatus;
}
unsafe extern "C" {
    pub fn DVDSetAudioVolume(inVolume: UInt16) -> OSStatus;
}
unsafe extern "C" {
    pub fn DVDGetAudioVolume(outVolume: *mut UInt16) -> OSStatus;
}
unsafe extern "C" {
    pub fn DVDGetAudioVolumeInfo(
        outMinVolume: *mut UInt16,
        outCurVolume: *mut UInt16,
        outMaxVolume: *mut UInt16,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn DVDHasMenu(inMenu: DVDMenu, outHasMenu: *mut Boolean) -> OSStatus;
}
unsafe extern "C" {
    pub fn DVDIsOnMenu(outOnMenu: *mut Boolean, outMenu: *mut DVDMenu) -> OSStatus;
}
unsafe extern "C" {
    pub fn DVDGoToMenu(inMenu: DVDMenu) -> OSStatus;
}
unsafe extern "C" {
    pub fn DVDReturnToTitle() -> OSStatus;
}
unsafe extern "C" {
    pub fn DVDGoBackOneLevel() -> OSStatus;
}
unsafe extern "C" {
    pub fn DVDDoUserNavigation(inNavigation: DVDUserNavigation) -> OSStatus;
}
unsafe extern "C" {
    pub fn DVDDoButtonActivate(inIndex: SInt32) -> OSStatus;
}
unsafe extern "C" {
    pub fn DVDGetButtoninfo(
        numberOfButtons: *mut UInt32,
        selectedButton: *mut UInt32,
        forcedActivateButton: *mut UInt32,
        userButtonOffset: *mut UInt32,
        numberOfUserButtons: *mut UInt32,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn DVDGetButtonPosition(
        index: UInt32,
        outRect: *mut CGRect,
        autoAction: *mut UInt32,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn DVDDoMenuCGClick(inPt: *mut CGPoint, outIndex: *mut SInt32) -> OSStatus;
}
unsafe extern "C" {
    pub fn DVDDoMenuCGMouseOver(inPt: *mut CGPoint, outIndex: *mut SInt32) -> OSStatus;
}
unsafe extern "C" {
    pub fn DVDGetMediaUniqueID(outDiscID: *mut UInt8) -> OSStatus;
}
unsafe extern "C" {
    pub fn DVDGetMediaVolumeName(outDiscVolumeName: *mut *mut ::std::os::raw::c_char) -> OSStatus;
}
unsafe extern "C" {
    pub fn DVDGetMediaVolumeCFName(outDiscVolumeCFName: *mut CFStringRef) -> OSStatus;
}
unsafe extern "C" {
    pub fn DVDSetTitle(inTitleNum: UInt16) -> OSStatus;
}
unsafe extern "C" {
    pub fn DVDGetTitle(outTitleNum: *mut UInt16) -> OSStatus;
}
unsafe extern "C" {
    pub fn DVDGetNumTitles(outNumTitles: *mut UInt16) -> OSStatus;
}
unsafe extern "C" {
    pub fn DVDHasPreviousChapter(outHasChapter: *mut Boolean) -> OSStatus;
}
unsafe extern "C" {
    pub fn DVDHasNextChapter(outHasChapter: *mut Boolean) -> OSStatus;
}
unsafe extern "C" {
    pub fn DVDSetChapter(inChapterNum: UInt16) -> OSStatus;
}
unsafe extern "C" {
    pub fn DVDGetChapter(outChapterNum: *mut UInt16) -> OSStatus;
}
unsafe extern "C" {
    pub fn DVDGetNumChapters(inTitleNum: UInt16, outNumChapters: *mut UInt16) -> OSStatus;
}
unsafe extern "C" {
    pub fn DVDPreviousChapter() -> OSStatus;
}
unsafe extern "C" {
    pub fn DVDNextChapter() -> OSStatus;
}
unsafe extern "C" {
    pub fn DVDSetAngle(inAngleNum: UInt16) -> OSStatus;
}
unsafe extern "C" {
    pub fn DVDGetAngle(outAngleNum: *mut UInt16) -> OSStatus;
}
unsafe extern "C" {
    pub fn DVDGetNumAngles(outNumAngles: *mut UInt16) -> OSStatus;
}
unsafe extern "C" {
    pub fn DVDDisplaySubPicture(inDisplay: Boolean) -> OSStatus;
}
unsafe extern "C" {
    pub fn DVDIsDisplayingSubPicture(outDisplayingSubPicture: *mut Boolean) -> OSStatus;
}
unsafe extern "C" {
    pub fn DVDSetSubPictureStream(inStreamNum: UInt16) -> OSStatus;
}
unsafe extern "C" {
    pub fn DVDGetSubPictureStream(outStreamNum: *mut UInt16) -> OSStatus;
}
unsafe extern "C" {
    pub fn DVDGetNumSubPictureStreams(outNumStreams: *mut UInt16) -> OSStatus;
}
unsafe extern "C" {
    pub fn DVDSetAudioStream(inStreamNum: UInt16) -> OSStatus;
}
unsafe extern "C" {
    pub fn DVDGetAudioStream(outStreamNum: *mut UInt16) -> OSStatus;
}
unsafe extern "C" {
    pub fn DVDGetNumAudioStreams(outNumStreams: *mut UInt16) -> OSStatus;
}
unsafe extern "C" {
    pub fn DVDSetDefaultSubPictureLanguageCode(
        inCode: DVDLanguageCode,
        inExtension: DVDSubpictureExtensionCode,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn DVDGetSubPictureLanguageCode(
        outCode: *mut DVDLanguageCode,
        outExtension: *mut DVDSubpictureExtensionCode,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn DVDGetSubPictureLanguageCodeByStream(
        inStreamNum: UInt16,
        outCode: *mut DVDLanguageCode,
        outExtension: *mut DVDSubpictureExtensionCode,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn DVDSetDefaultAudioLanguageCode(
        inCode: DVDLanguageCode,
        inExtension: DVDAudioExtensionCode,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn DVDGetAudioLanguageCode(
        outCode: *mut DVDLanguageCode,
        outExtension: *mut DVDAudioExtensionCode,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn DVDGetAudioLanguageCodeByStream(
        inStreamNum: UInt16,
        outCode: *mut DVDLanguageCode,
        outExtension: *mut DVDAudioExtensionCode,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn DVDSetDefaultMenuLanguageCode(inCode: DVDLanguageCode) -> OSStatus;
}
unsafe extern "C" {
    pub fn DVDGetMenuLanguageCode(outCode: *mut DVDLanguageCode) -> OSStatus;
}
unsafe extern "C" {
    pub fn DVDGetBookmark(
        outBookMarkData: *mut ::std::os::raw::c_void,
        ioBookMarkDataSize: *mut UInt32,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn DVDGotoBookmark(
        inBookMarkData: *mut ::std::os::raw::c_void,
        inBookMarkDataSize: UInt32,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn DVDGetLastPlayBookmark(
        outBookMarkData: *mut ::std::os::raw::c_void,
        ioBookMarkDataSize: *mut UInt32,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn DVDSetLastPlayBookmark(
        inBookMarkData: *mut ::std::os::raw::c_void,
        inBookMarkDataSize: UInt32,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn DVDClearLastPlayBookmark() -> OSStatus;
}
unsafe extern "C" {
    pub fn DVDGetDiscRegionCode(outCode: *mut DVDRegionCode) -> OSStatus;
}
unsafe extern "C" {
    pub fn DVDGetDriveRegionCode(
        outCode: *mut DVDRegionCode,
        outNumberChangesLeft: *mut SInt16,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn DVDSetDriveRegionCode(
        inCode: DVDRegionCode,
        inAuthorization: AuthorizationRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn DVDEnableWebAccess(inEnable: Boolean) -> OSStatus;
}
unsafe extern "C" {
    pub fn DVDGetGPRMValue(index: UInt32, value: *mut UInt32) -> OSStatus;
}
unsafe extern "C" {
    pub fn DVDSetFatalErrorCallBack(
        inCallBackProc: DVDFatalErrCallBackFunctionPtr,
        inRefCon: *mut ::std::os::raw::c_void,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn DVDRegisterEventCallBack(
        inCallBackProc: DVDEventCallBackFunctionPtr,
        inCode: *mut DVDEventCode,
        inCodeCount: UInt32,
        inRefCon: *mut ::std::os::raw::c_void,
        outCallBackID: *mut DVDEventCallBackRef,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn DVDUnregisterEventCallBack(inCallBackID: DVDEventCallBackRef) -> OSStatus;
}
unsafe extern "C" {
    pub fn DVDIsRegisteredEventCallBack(inCallBackID: DVDEventCallBackRef) -> Boolean;
}
unsafe extern "C" {
    pub fn DVDSetTimeEventRate(inMilliseconds: UInt32) -> OSStatus;
}
unsafe extern "C" {
    pub fn DVDGetTimeEventRate(outMilliseconds: *mut UInt32) -> OSStatus;
}
unsafe extern "C" {
    pub fn DVDSleep() -> OSStatus;
}
unsafe extern "C" {
    pub fn DVDWakeUp() -> OSStatus;
}
