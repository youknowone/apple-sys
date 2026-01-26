pub type SCARDCONTEXT = i32;
pub type PSCARDCONTEXT = *mut SCARDCONTEXT;
pub type LPSCARDCONTEXT = *mut SCARDCONTEXT;
pub type SCARDHANDLE = i32;
pub type PSCARDHANDLE = *mut SCARDHANDLE;
pub type LPSCARDHANDLE = *mut SCARDHANDLE;
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct SCARD_READERSTATE_A {
    pub szReader: *const ::std::os::raw::c_char,
    pub pvUserData: *mut ::std::os::raw::c_void,
    pub dwCurrentState: u32,
    pub dwEventState: u32,
    pub cbAtr: u32,
    pub rgbAtr: [::std::os::raw::c_uchar; 33usize],
}
pub type LPSCARD_READERSTATE_A = *mut SCARD_READERSTATE_A;
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct _SCARD_IO_REQUEST {
    pub dwProtocol: u32,
    pub cbPciLength: u32,
}
pub type SCARD_IO_REQUEST = _SCARD_IO_REQUEST;
pub type LPSCARD_IO_REQUEST = *mut _SCARD_IO_REQUEST;
pub type LPCSCARD_IO_REQUEST = *const SCARD_IO_REQUEST;
pub type MSC_RV = u32;
pub type MSCChar8 = ::std::os::raw::c_char;
pub type MSCPUChar8 = *mut u8;
pub type MSCPCUChar8 = *const u8;
pub type MSCUChar8 = u8;
pub type MSCPUShort16 = *mut u16;
pub type MSCUShort16 = u16;
pub type MSCPShort16 = *mut i16;
pub type MSCShort16 = i16;
pub type MSCPULong32 = *mut u32;
pub type MSCULong32 = u32;
pub type MSCPLong32 = *mut i32;
pub type MSCLong32 = i32;
pub type MSCPCVoid32 = *const ::std::os::raw::c_void;
pub type MSCPVoid32 = *mut ::std::os::raw::c_void;
pub type MSCCString = *const ::std::os::raw::c_char;
pub type MSCString = *mut ::std::os::raw::c_char;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CFDyLibPointers {
    pub pvfWriteFramework: MSCPVoid32,
    pub pvfInitializePlugin: MSCPVoid32,
    pub pvfIdentifyToken: MSCPVoid32,
    pub pvfFinalizePlugin: MSCPVoid32,
    pub pvfGetStatus: MSCPVoid32,
    pub pvfGetCapabilities: MSCPVoid32,
    pub pvfExtendedFeature: MSCPVoid32,
    pub pvfGenerateKeys: MSCPVoid32,
    pub pvfImportKey: MSCPVoid32,
    pub pvfExportKey: MSCPVoid32,
    pub pvfComputeCrypt: MSCPVoid32,
    pub pvfExtAuthenticate: MSCPVoid32,
    pub pvfListKeys: MSCPVoid32,
    pub pvfCreatePIN: MSCPVoid32,
    pub pvfVerifyPIN: MSCPVoid32,
    pub pvfChangePIN: MSCPVoid32,
    pub pvfUnblockPIN: MSCPVoid32,
    pub pvfListPINs: MSCPVoid32,
    pub pvfCreateObject: MSCPVoid32,
    pub pvfDeleteObject: MSCPVoid32,
    pub pvfWriteObject: MSCPVoid32,
    pub pvfReadObject: MSCPVoid32,
    pub pvfListObjects: MSCPVoid32,
    pub pvfLogoutAll: MSCPVoid32,
    pub pvfGetChallenge: MSCPVoid32,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct MSCTokenInfo {
    pub tokenName: [MSCChar8; 150usize],
    pub slotName: [MSCChar8; 52usize],
    pub svProvider: [MSCChar8; 200usize],
    pub tokenId: [MSCUChar8; 33usize],
    pub tokenApp: [MSCUChar8; 64usize],
    pub tokenAppLen: MSCULong32,
    pub tokenIdLength: MSCULong32,
    pub tokenState: MSCULong32,
    pub tokenType: MSCULong32,
    pub addParams: MSCPVoid32,
    pub addParamsSize: MSCULong32,
}
pub type MSCLPTokenInfo = *mut MSCTokenInfo;
pub type LPRWEventCallback = ::std::option::Option<
    unsafe extern "C" fn(arg1: MSCPVoid32, arg2: ::std::os::raw::c_int) -> MSC_RV,
>;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct MSCTokenConnection {
    pub hContext: MSCLong32,
    pub hCard: MSCLong32,
    pub ioType: LPSCARD_IO_REQUEST,
    pub pMac: [MSCUChar8; 128usize],
    pub macSize: MSCULong32,
    pub tokenLibHandle: MSCPVoid32,
    pub libPointers: CFDyLibPointers,
    pub tokenInfo: MSCTokenInfo,
    pub loggedIDs: MSCUChar8,
    pub shareMode: MSCULong32,
    pub rwCallback: LPRWEventCallback,
}
pub type MSCLPTokenConnection = *mut MSCTokenConnection;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct MSCInitTokenParams {
    pub transportKey: [MSCUChar8; 264usize],
    pub transportKeyLen: MSCULong32,
    pub transportBehavior: MSCUChar8,
    pub objectMemory: MSCULong32,
    pub newTransportKey: [MSCUChar8; 264usize],
    pub newTransportKeyLen: MSCULong32,
    pub defaultCHV: [MSCUChar8; 264usize],
    pub defaultCHVLen: MSCULong32,
    pub defaultCHVTries: MSCUChar8,
    pub defaultCHVUnblock: [MSCUChar8; 264usize],
    pub defaultCHVUnblockSize: MSCULong32,
    pub defaultCHVUnblockTries: MSCUChar8,
    pub createObjectACL: MSCUShort16,
    pub createKeysACL: MSCUShort16,
    pub createPINsACL: MSCUShort16,
    pub maxNumberKeys: MSCUChar8,
    pub maxNumberPINs: MSCUChar8,
    pub maxNumberObjects: MSCUShort16,
}
pub type MSCLPInitTokenParams = *mut MSCInitTokenParams;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct MSCKeyPolicy {
    pub cipherMode: MSCUShort16,
    pub cipherDirection: MSCUShort16,
}
pub type MSCLPKeyPolicy = *mut MSCKeyPolicy;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct MSCKeyACL {
    pub readPermission: MSCUShort16,
    pub writePermission: MSCUShort16,
    pub usePermission: MSCUShort16,
}
pub type MSCLPKeyACL = *mut MSCKeyACL;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct MSCObjectACL {
    pub readPermission: MSCUShort16,
    pub writePermission: MSCUShort16,
    pub deletePermission: MSCUShort16,
}
pub type MSCLPObjectACL = *mut MSCObjectACL;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct MSCGenKeyParams {
    pub algoType: MSCUChar8,
    pub keySize: MSCUShort16,
    pub privateKeyACL: MSCKeyACL,
    pub publicKeyACL: MSCKeyACL,
    pub privateKeyPolicy: MSCKeyPolicy,
    pub publicKeyPolicy: MSCKeyPolicy,
    pub keyGenOptions: MSCUChar8,
    pub pOptParams: MSCPUChar8,
    pub optParamsSize: MSCULong32,
}
pub type MSCLPGenKeyParams = *mut MSCGenKeyParams;
pub type MSCLPKeyBlob = MSCPUChar8;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct MSCKeyInfo {
    pub keyNum: MSCUChar8,
    pub keyType: MSCUChar8,
    pub keyPartner: MSCUChar8,
    pub keyMapping: MSCUChar8,
    pub keySize: MSCUShort16,
    pub keyPolicy: MSCKeyPolicy,
    pub keyACL: MSCKeyACL,
}
pub type MSCLPKeyInfo = *mut MSCKeyInfo;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct MSCCryptInit {
    pub keyNum: MSCUChar8,
    pub cipherMode: MSCUChar8,
    pub cipherDirection: MSCUChar8,
    pub optParams: MSCPUChar8,
    pub optParamsSize: MSCUShort16,
}
pub type MSCLPCryptInit = *mut MSCCryptInit;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct MSCStatusInfo {
    pub appVersion: MSCUShort16,
    pub swVersion: MSCUShort16,
    pub freeMemory: MSCULong32,
    pub totalMemory: MSCULong32,
    pub usedPINs: MSCUChar8,
    pub usedKeys: MSCUChar8,
    pub loggedID: MSCUShort16,
}
pub type MSCLPStatusInfo = *mut MSCStatusInfo;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct MSCObjectInfo {
    pub objectID: [MSCChar8; 16usize],
    pub objectSize: MSCULong32,
    pub objectACL: MSCObjectACL,
}
pub type MSCLPObjectInfo = *mut MSCObjectInfo;
pub type BYTE = u8;
pub type UCHAR = u8;
pub type PUCHAR = *mut u8;
pub type USHORT = u16;
pub type PULONG = *mut u32;
pub type LPCVOID = *const ::std::os::raw::c_void;
pub type PDWORD = *mut u32;
pub type WORD = u16;
pub type RESPONSECODE = i32;
pub type LPCSTR = *const ::std::os::raw::c_char;
pub type LPCBYTE = *const BYTE;
pub type LPBYTE = *mut BYTE;
pub type LPSTR = *mut ::std::os::raw::c_char;
pub type LPTSTR = *mut ::std::os::raw::c_char;
pub type LPCTSTR = *const ::std::os::raw::c_char;
unsafe extern "C" {
    pub static mut g_rgSCardT0Pci: SCARD_IO_REQUEST;
}
unsafe extern "C" {
    pub static mut g_rgSCardT1Pci: SCARD_IO_REQUEST;
}
unsafe extern "C" {
    pub static mut g_rgSCardRawPci: SCARD_IO_REQUEST;
}
unsafe extern "C" {
    pub fn pcsc_stringify_error(err: i32) -> *mut ::std::os::raw::c_char;
}
unsafe extern "C" {
    pub fn SCardEstablishContext(
        dwScope: u32,
        pvReserved1: *const ::std::os::raw::c_void,
        pvReserved2: *const ::std::os::raw::c_void,
        phContext: LPSCARDCONTEXT,
    ) -> i32;
}
unsafe extern "C" {
    pub fn SCardReleaseContext(hContext: SCARDCONTEXT) -> i32;
}
unsafe extern "C" {
    pub fn SCardIsValidContext(hContext: SCARDCONTEXT) -> i32;
}
unsafe extern "C" {
    pub fn SCardSetTimeout(hContext: SCARDCONTEXT, dwTimeout: u32) -> i32;
}
unsafe extern "C" {
    pub fn SCardConnect(
        hContext: SCARDCONTEXT,
        szReader: *const ::std::os::raw::c_char,
        dwShareMode: u32,
        dwPreferredProtocols: u32,
        phCard: LPSCARDHANDLE,
        pdwActiveProtocol: *mut u32,
    ) -> i32;
}
unsafe extern "C" {
    pub fn SCardReconnect(
        hCard: SCARDHANDLE,
        dwShareMode: u32,
        dwPreferredProtocols: u32,
        dwInitialization: u32,
        pdwActiveProtocol: *mut u32,
    ) -> i32;
}
unsafe extern "C" {
    pub fn SCardDisconnect(hCard: SCARDHANDLE, dwDisposition: u32) -> i32;
}
unsafe extern "C" {
    pub fn SCardBeginTransaction(hCard: SCARDHANDLE) -> i32;
}
unsafe extern "C" {
    pub fn SCardEndTransaction(hCard: SCARDHANDLE, dwDisposition: u32) -> i32;
}
unsafe extern "C" {
    pub fn SCardCancelTransaction(hCard: SCARDHANDLE) -> i32;
}
unsafe extern "C" {
    pub fn SCardStatus(
        hCard: SCARDHANDLE,
        mszReaderNames: *mut ::std::os::raw::c_char,
        pcchReaderLen: *mut u32,
        pdwState: *mut u32,
        pdwProtocol: *mut u32,
        pbAtr: *mut ::std::os::raw::c_uchar,
        pcbAtrLen: *mut u32,
    ) -> i32;
}
unsafe extern "C" {
    pub fn SCardGetStatusChange(
        hContext: SCARDCONTEXT,
        dwTimeout: u32,
        rgReaderStates: LPSCARD_READERSTATE_A,
        cReaders: u32,
    ) -> i32;
}
unsafe extern "C" {
    pub fn SCardControl(
        hCard: SCARDHANDLE,
        pbSendBuffer: *const ::std::os::raw::c_void,
        cbSendLength: u32,
        pbRecvBuffer: *mut ::std::os::raw::c_void,
        pcbRecvLength: *mut u32,
    ) -> i32;
}
unsafe extern "C" {
    pub fn SCardControl132(
        hCard: SCARDHANDLE,
        dwControlCode: u32,
        pbSendBuffer: *const ::std::os::raw::c_void,
        cbSendLength: u32,
        pbRecvBuffer: *mut ::std::os::raw::c_void,
        cbRecvLength: u32,
        lpBytesReturned: *mut u32,
    ) -> i32;
}
unsafe extern "C" {
    pub fn SCardTransmit(
        hCard: SCARDHANDLE,
        pioSendPci: LPCSCARD_IO_REQUEST,
        pbSendBuffer: *const ::std::os::raw::c_uchar,
        cbSendLength: u32,
        pioRecvPci: LPSCARD_IO_REQUEST,
        pbRecvBuffer: *mut ::std::os::raw::c_uchar,
        pcbRecvLength: *mut u32,
    ) -> i32;
}
unsafe extern "C" {
    pub fn SCardListReaderGroups(
        hContext: SCARDCONTEXT,
        mszGroups: *mut ::std::os::raw::c_char,
        pcchGroups: *mut u32,
    ) -> i32;
}
unsafe extern "C" {
    pub fn SCardListReaders(
        hContext: SCARDCONTEXT,
        mszGroups: *const ::std::os::raw::c_char,
        mszReaders: *mut ::std::os::raw::c_char,
        pcchReaders: *mut u32,
    ) -> i32;
}
unsafe extern "C" {
    pub fn SCardCancel(hContext: SCARDCONTEXT) -> i32;
}
unsafe extern "C" {
    pub fn SCardGetAttrib(
        hCard: SCARDHANDLE,
        dwAttrId: u32,
        pbAttr: *mut u8,
        pcbAttrLen: *mut u32,
    ) -> i32;
}
unsafe extern "C" {
    pub fn SCardSetAttrib(
        hCard: SCARDHANDLE,
        dwAttrId: u32,
        pbAttr: *const u8,
        cbAttrLen: u32,
    ) -> i32;
}
unsafe extern "C" {
    pub fn SCardUnload();
}
unsafe extern "C" {
    pub fn MSCListTokens(
        listScope: MSCULong32,
        tokenArray: MSCLPTokenInfo,
        arrayLength: MSCPULong32,
    ) -> MSC_RV;
}
unsafe extern "C" {
    pub fn MSCEstablishConnection(
        tokenStruct: MSCLPTokenInfo,
        sharingMode: MSCULong32,
        applicationName: MSCPUChar8,
        nameSize: MSCULong32,
        pConnection: MSCLPTokenConnection,
    ) -> MSC_RV;
}
unsafe extern "C" {
    pub fn MSCReleaseConnection(pConnection: MSCLPTokenConnection, endAction: MSCULong32)
        -> MSC_RV;
}
unsafe extern "C" {
    pub fn MSCWaitForTokenEvent(
        tokenArray: MSCLPTokenInfo,
        arraySize: MSCULong32,
        timeoutValue: MSCULong32,
    ) -> MSC_RV;
}
unsafe extern "C" {
    pub fn MSCCancelEventWait() -> MSC_RV;
}
unsafe extern "C" {
    pub fn MSCBeginTransaction(pConnection: MSCLPTokenConnection) -> MSC_RV;
}
unsafe extern "C" {
    pub fn MSCEndTransaction(pConnection: MSCLPTokenConnection, endAction: MSCULong32) -> MSC_RV;
}
unsafe extern "C" {
    pub fn MSCWriteFramework(
        pConnection: MSCLPTokenConnection,
        pInitParams: MSCLPInitTokenParams,
    ) -> MSC_RV;
}
unsafe extern "C" {
    pub fn MSCGetStatus(pConnection: MSCLPTokenConnection, pStatusInfo: MSCLPStatusInfo) -> MSC_RV;
}
unsafe extern "C" {
    pub fn MSCGetCapabilities(
        pConnection: MSCLPTokenConnection,
        Tag: MSCULong32,
        Value: MSCPUChar8,
        Length: MSCPULong32,
    ) -> MSC_RV;
}
unsafe extern "C" {
    pub fn MSCExtendedFeature(
        pConnection: MSCLPTokenConnection,
        extFeature: MSCULong32,
        outData: MSCPUChar8,
        outLength: MSCULong32,
        inData: MSCPUChar8,
        inLength: MSCPULong32,
    ) -> MSC_RV;
}
unsafe extern "C" {
    pub fn MSCGenerateKeys(
        pConnection: MSCLPTokenConnection,
        prvKeyNum: MSCUChar8,
        pubKeyNum: MSCUChar8,
        pParams: MSCLPGenKeyParams,
    ) -> MSC_RV;
}
unsafe extern "C" {
    pub fn MSCImportKey(
        pConnection: MSCLPTokenConnection,
        keyNum: MSCUChar8,
        pKeyACL: MSCLPKeyACL,
        pKeyBlob: MSCPUChar8,
        keyBlobSize: MSCULong32,
        keyPolicy: MSCLPKeyPolicy,
        pAddParams: MSCPVoid32,
        addParamsSize: MSCUChar8,
    ) -> MSC_RV;
}
unsafe extern "C" {
    pub fn MSCExportKey(
        pConnection: MSCLPTokenConnection,
        keyNum: MSCUChar8,
        pKeyBlob: MSCPUChar8,
        keyBlobSize: MSCPULong32,
        pAddParams: MSCPVoid32,
        addParamsSize: MSCUChar8,
    ) -> MSC_RV;
}
unsafe extern "C" {
    pub fn MSCComputeCrypt(
        pConnection: MSCLPTokenConnection,
        cryptInit: MSCLPCryptInit,
        pInputData: MSCPUChar8,
        inputDataSize: MSCULong32,
        pOutputData: MSCPUChar8,
        outputDataSize: MSCPULong32,
    ) -> MSC_RV;
}
unsafe extern "C" {
    pub fn MSCExtAuthenticate(
        pConnection: MSCLPTokenConnection,
        keyNum: MSCUChar8,
        cipherMode: MSCUChar8,
        cipherDirection: MSCUChar8,
        pData: MSCPUChar8,
        dataSize: MSCULong32,
    ) -> MSC_RV;
}
unsafe extern "C" {
    pub fn MSCListKeys(
        pConnection: MSCLPTokenConnection,
        seqOption: MSCUChar8,
        pKeyInfo: MSCLPKeyInfo,
    ) -> MSC_RV;
}
unsafe extern "C" {
    pub fn MSCCreatePIN(
        pConnection: MSCLPTokenConnection,
        pinNum: MSCUChar8,
        pinAttempts: MSCUChar8,
        pPinCode: MSCPUChar8,
        pinCodeSize: MSCULong32,
        pUnblockCode: MSCPUChar8,
        unblockCodeSize: MSCUChar8,
    ) -> MSC_RV;
}
unsafe extern "C" {
    pub fn MSCVerifyPIN(
        pConnection: MSCLPTokenConnection,
        pinNum: MSCUChar8,
        pPinCode: MSCPUChar8,
        pinCodeSize: MSCULong32,
    ) -> MSC_RV;
}
unsafe extern "C" {
    pub fn MSCChangePIN(
        pConnection: MSCLPTokenConnection,
        pinNum: MSCUChar8,
        pOldPinCode: MSCPUChar8,
        oldPinCodeSize: MSCUChar8,
        pNewPinCode: MSCPUChar8,
        newPinCodeSize: MSCUChar8,
    ) -> MSC_RV;
}
unsafe extern "C" {
    pub fn MSCUnblockPIN(
        pConnection: MSCLPTokenConnection,
        pinNum: MSCUChar8,
        pUnblockCode: MSCPUChar8,
        unblockCodeSize: MSCULong32,
    ) -> MSC_RV;
}
unsafe extern "C" {
    pub fn MSCListPINs(pConnection: MSCLPTokenConnection, pPinBitMask: MSCPUShort16) -> MSC_RV;
}
unsafe extern "C" {
    pub fn MSCCreateObject(
        pConnection: MSCLPTokenConnection,
        objectID: MSCString,
        objectSize: MSCULong32,
        pObjectACL: MSCLPObjectACL,
    ) -> MSC_RV;
}
unsafe extern "C" {
    pub fn MSCDeleteObject(
        pConnection: MSCLPTokenConnection,
        objectID: MSCString,
        zeroFlag: MSCUChar8,
    ) -> MSC_RV;
}
unsafe extern "C" {
    pub fn MSCWriteObject(
        pConnection: MSCLPTokenConnection,
        objectID: MSCString,
        offset: MSCULong32,
        pInputData: MSCPUChar8,
        dataSize: MSCULong32,
        rwCallback: LPRWEventCallback,
        addParams: MSCPVoid32,
    ) -> MSC_RV;
}
unsafe extern "C" {
    pub fn MSCReadObject(
        pConnection: MSCLPTokenConnection,
        objectID: MSCString,
        offset: MSCULong32,
        pOutputData: MSCPUChar8,
        dataSize: MSCULong32,
        rwCallback: LPRWEventCallback,
        addParams: MSCPVoid32,
    ) -> MSC_RV;
}
unsafe extern "C" {
    pub fn MSCReadAllocateObject(
        pConnection: MSCLPTokenConnection,
        objectID: MSCString,
        pOutputData: *mut MSCPUChar8,
        dataSize: MSCPULong32,
        rwCallback: LPRWEventCallback,
        addParams: MSCPVoid32,
    ) -> MSC_RV;
}
unsafe extern "C" {
    pub fn MSCListObjects(
        pConnection: MSCLPTokenConnection,
        seqOption: MSCUChar8,
        pObjectInfo: MSCLPObjectInfo,
    ) -> MSC_RV;
}
unsafe extern "C" {
    pub fn MSCLogoutAll(pConnection: MSCLPTokenConnection) -> MSC_RV;
}
unsafe extern "C" {
    pub fn MSCGetChallenge(
        pConnection: MSCLPTokenConnection,
        pSeed: MSCPUChar8,
        seedSize: MSCUShort16,
        pRandomData: MSCPUChar8,
        randomDataSize: MSCUShort16,
    ) -> MSC_RV;
}
unsafe extern "C" {
    pub fn MSCGetKeyAttributes(
        pConnection: MSCLPTokenConnection,
        keyNumber: MSCUChar8,
        pKeyInfo: MSCLPKeyInfo,
    ) -> MSC_RV;
}
unsafe extern "C" {
    pub fn MSCGetObjectAttributes(
        pConnection: MSCLPTokenConnection,
        objectID: MSCString,
        pObjectInfo: MSCLPObjectInfo,
    ) -> MSC_RV;
}
unsafe extern "C" {
    pub fn msc_error(errorCode: ::std::os::raw::c_ulong) -> *mut ::std::os::raw::c_char;
}
unsafe extern "C" {
    pub fn MSCIsTokenReset(pConnection: MSCLPTokenConnection) -> MSCUChar8;
}
unsafe extern "C" {
    pub fn MSCClearReset(pConnection: MSCLPTokenConnection) -> MSCUChar8;
}
unsafe extern "C" {
    pub fn MSCIsTokenMoved(pConnection: MSCLPTokenConnection) -> MSCUChar8;
}
unsafe extern "C" {
    pub fn MSCIsTokenChanged(pConnection: MSCLPTokenConnection) -> MSCUChar8;
}
unsafe extern "C" {
    pub fn MSCIsTokenKnown(pConnection: MSCLPTokenConnection) -> MSCUChar8;
}

unsafe impl objc2::encode::RefEncode for SCARD_READERSTATE_A {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for SCARD_READERSTATE_A {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("SCARD_READERSTATE_A", &[]);
}
unsafe impl objc2::encode::RefEncode for _SCARD_IO_REQUEST {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for _SCARD_IO_REQUEST {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("_SCARD_IO_REQUEST", &[]);
}
unsafe impl objc2::encode::RefEncode for CFDyLibPointers {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CFDyLibPointers {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("CFDyLibPointers", &[]);
}
unsafe impl objc2::encode::RefEncode for MSCTokenInfo {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MSCTokenInfo {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("MSCTokenInfo", &[]);
}
unsafe impl objc2::encode::RefEncode for MSCTokenConnection {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MSCTokenConnection {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("MSCTokenConnection", &[]);
}
unsafe impl objc2::encode::RefEncode for MSCInitTokenParams {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MSCInitTokenParams {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("MSCInitTokenParams", &[]);
}
unsafe impl objc2::encode::RefEncode for MSCKeyPolicy {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MSCKeyPolicy {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("MSCKeyPolicy", &[]);
}
unsafe impl objc2::encode::RefEncode for MSCKeyACL {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MSCKeyACL {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("MSCKeyACL", &[]);
}
unsafe impl objc2::encode::RefEncode for MSCObjectACL {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MSCObjectACL {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("MSCObjectACL", &[]);
}
unsafe impl objc2::encode::RefEncode for MSCGenKeyParams {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MSCGenKeyParams {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("MSCGenKeyParams", &[]);
}
unsafe impl objc2::encode::RefEncode for MSCKeyInfo {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MSCKeyInfo {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("MSCKeyInfo", &[]);
}
unsafe impl objc2::encode::RefEncode for MSCCryptInit {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MSCCryptInit {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("MSCCryptInit", &[]);
}
unsafe impl objc2::encode::RefEncode for MSCStatusInfo {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MSCStatusInfo {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("MSCStatusInfo", &[]);
}
unsafe impl objc2::encode::RefEncode for MSCObjectInfo {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MSCObjectInfo {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("MSCObjectInfo", &[]);
}
