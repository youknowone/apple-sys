#[allow(unused_imports)]
use crate::CoreFoundation::*;

#[allow(unused_imports)]
use objc2::msg_send;
pub type tDirStatus = ::std::os::raw::c_int;
pub type tDirPatternMatch = ::std::os::raw::c_uint;
pub type tDirReference = UInt32;
pub type tDirNodeReference = UInt32;
pub type tClientData = *mut ::std::os::raw::c_void;
pub type tBuffer = *mut ::std::os::raw::c_void;
pub type tContextData = UInt32;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct tDataBuffer {
    pub fBufferSize: UInt32,
    pub fBufferLength: UInt32,
    pub fBufferData: [::std::os::raw::c_char; 1usize],
}
pub type tDataBufferPtr = *mut tDataBuffer;
pub type tDataNode = tDataBuffer;
pub type tDataNodePtr = *mut tDataNode;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct tDataList {
    pub fDataNodeCount: UInt32,
    pub fDataListHead: tDataNodePtr,
}
pub type tDataListPtr = *mut tDataList;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct tAccessControlEntry {
    pub fGuestAccessFlags: UInt32,
    pub fDirMemberFlags: UInt32,
    pub fDirNodeMemberFlags: UInt32,
    pub fOwnerFlags: UInt32,
    pub fAdministratorFlags: UInt32,
}
pub type tAccessControlEntryPtr = *mut tAccessControlEntry;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct tAttributeValueEntry {
    pub fAttributeValueID: UInt32,
    pub fAttributeValueData: tDataNode,
}
pub type tAttributeValueEntryPtr = *mut tAttributeValueEntry;
pub type tAttributeValueListRef = UInt32;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct tAttributeEntry {
    pub fReserved1: UInt32,
    pub fReserved2: tAccessControlEntry,
    pub fAttributeValueCount: UInt32,
    pub fAttributeDataSize: UInt32,
    pub fAttributeValueMaxSize: UInt32,
    pub fAttributeSignature: tDataNode,
}
pub type tAttributeEntryPtr = *mut tAttributeEntry;
pub type tAttributeListRef = UInt32;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct tRecordEntry {
    pub fReserved1: UInt32,
    pub fReserved2: tAccessControlEntry,
    pub fRecordAttributeCount: UInt32,
    pub fRecordNameAndType: tDataNode,
}
pub type tRecordEntryPtr = *mut tRecordEntry;
pub type tRecordReference = UInt32;
pub type fpCustomAllocate = ::std::option::Option<
    unsafe extern "C" fn(
        inDirReference: tDirReference,
        inClientData: tClientData,
        inAllocationRequest: UInt32,
        outAllocationPtr: *mut tBuffer,
    ) -> tDirStatus,
>;
pub type fpCustomDeAllocate = ::std::option::Option<
    unsafe extern "C" fn(
        inDirReference: tDirReference,
        inClientData: tClientData,
        inAllocationPtr: tBuffer,
    ) -> tDirStatus,
>;
pub type fpCustomThreadBlock = ::std::option::Option<
    unsafe extern "C" fn(inDirReference: tDirReference, inClientData: tClientData) -> tDirStatus,
>;
pub type fpCustomThreadUnBlock = ::std::option::Option<
    unsafe extern "C" fn(inDirReference: tDirReference, inClientData: tClientData) -> tDirStatus,
>;
pub type fpCustomThreadYield = ::std::option::Option<
    unsafe extern "C" fn(inDirReference: tDirReference, inClientData: tClientData) -> tDirStatus,
>;
unsafe extern "C" {
    pub fn dsOpenDirService(outDirReference: *mut tDirReference) -> tDirStatus;
}
unsafe extern "C" {
    pub fn dsOpenDirServiceProxy(
        outDirRef: *mut tDirReference,
        inHostOrIPAddress: *const ::std::os::raw::c_char,
        inIPPort: UInt32,
        inAuthMethod: tDataNodePtr,
        inAuthStepData: tDataBufferPtr,
        outAuthStepDataResponse: tDataBufferPtr,
        ioContinueData: *mut tContextData,
    ) -> tDirStatus;
}
unsafe extern "C" {
    pub fn dsOpenDirServiceLocal(
        outDirRef: *mut tDirReference,
        inFilePath: *const ::std::os::raw::c_char,
    ) -> tDirStatus;
}
unsafe extern "C" {
    pub fn dsCloseDirService(inDirReference: tDirReference) -> tDirStatus;
}
unsafe extern "C" {
    pub fn dsAddChildPIDToReference(
        inDirRef: tDirReference,
        inValidChildPID: SInt32,
        inValidAPIReferenceToGrantChild: UInt32,
    ) -> tDirStatus;
}
unsafe extern "C" {
    pub fn dsGetDirNodeCount(
        inDirReference: tDirReference,
        outDirectoryNodeCount: *mut UInt32,
    ) -> tDirStatus;
}
unsafe extern "C" {
    pub fn dsGetDirNodeCountWithInfo(
        inDirReference: tDirReference,
        outDirectoryNodeCount: *mut UInt32,
        outDirectoryNodeChangeToken: *mut UInt32,
    ) -> tDirStatus;
}
unsafe extern "C" {
    pub fn dsGetDirNodeList(
        inDirReference: tDirReference,
        inOutDataBufferPtr: tDataBufferPtr,
        outDirNodeCount: *mut UInt32,
        inOutContinueData: *mut tContextData,
    ) -> tDirStatus;
}
unsafe extern "C" {
    pub fn dsReleaseContinueData(
        inDirReference: tDirReference,
        inContinueData: tContextData,
    ) -> tDirStatus;
}
unsafe extern "C" {
    pub fn dsFindDirNodes(
        inDirReference: tDirReference,
        inOutDataBufferPtr: tDataBufferPtr,
        inNodeNamePattern: tDataListPtr,
        inPatternMatchType: tDirPatternMatch,
        outDirNodeCount: *mut UInt32,
        inOutContinueData: *mut tContextData,
    ) -> tDirStatus;
}
unsafe extern "C" {
    pub fn dsGetDirNodeName(
        inDirReference: tDirReference,
        inOutDataBuffer: tDataBufferPtr,
        inDirNodeIndex: UInt32,
        inOutDataList: *mut tDataListPtr,
    ) -> tDirStatus;
}
unsafe extern "C" {
    pub fn dsOpenDirNode(
        inDirReference: tDirReference,
        inDirNodeName: tDataListPtr,
        outDirNodeReference: *mut tDirNodeReference,
    ) -> tDirStatus;
}
unsafe extern "C" {
    pub fn dsCloseDirNode(inDirNodeReference: tDirNodeReference) -> tDirStatus;
}
unsafe extern "C" {
    pub fn dsGetDirNodeInfo(
        inDirNodeReference: tDirNodeReference,
        inDirNodeInfoTypeList: tDataListPtr,
        inOutDataBuffer: tDataBufferPtr,
        inAttributeInfoOnly: ::std::os::raw::c_int,
        outAttributeInfoCount: *mut UInt32,
        outAttributeListRef: *mut tAttributeListRef,
        inOutContinueData: *mut tContextData,
    ) -> tDirStatus;
}
unsafe extern "C" {
    pub fn dsGetRecordList(
        inDirNodeReference: tDirNodeReference,
        inOutDataBuffer: tDataBufferPtr,
        inRecordNameList: tDataListPtr,
        inPatternMatchType: tDirPatternMatch,
        inRecordTypeList: tDataListPtr,
        inAttributeTypeList: tDataListPtr,
        inAttributeInfoOnly: ::std::os::raw::c_int,
        inOutRecordEntryCount: *mut UInt32,
        inOutContinueData: *mut tContextData,
    ) -> tDirStatus;
}
unsafe extern "C" {
    pub fn dsGetRecordEntry(
        inDirNodeReference: tDirNodeReference,
        inOutDataBuffer: tDataBufferPtr,
        inRecordEntryIndex: UInt32,
        outAttributeListRef: *mut tAttributeListRef,
        outRecordEntryPtr: *mut tRecordEntryPtr,
    ) -> tDirStatus;
}
unsafe extern "C" {
    pub fn dsGetAttributeEntry(
        inDirNodeReference: tDirNodeReference,
        inOutDataBuffer: tDataBufferPtr,
        inAttributeListRef: tAttributeListRef,
        inAttributeInfoIndex: UInt32,
        outAttributeValueListRef: *mut tAttributeValueListRef,
        outAttributeInfoPtr: *mut tAttributeEntryPtr,
    ) -> tDirStatus;
}
unsafe extern "C" {
    pub fn dsGetAttributeValue(
        inDirNodeReference: tDirNodeReference,
        inOutDataBuffer: tDataBufferPtr,
        inAttributeValueIndex: UInt32,
        inAttributeValueListRef: tAttributeValueListRef,
        outAttributeValue: *mut tAttributeValueEntryPtr,
    ) -> tDirStatus;
}
unsafe extern "C" {
    pub fn dsGetNextAttributeEntry(
        inDirNodeReference: tDirNodeReference,
        inOutDataBuffer: tDataBufferPtr,
        inAttributeListRef: tAttributeListRef,
        inAttributeInfoIndex: UInt32,
        inOutAttributeOffset: *mut SInt32,
        outAttributeValueListRef: *mut tAttributeValueListRef,
        outAttributeInfoPtr: *mut tAttributeEntryPtr,
    ) -> tDirStatus;
}
unsafe extern "C" {
    pub fn dsGetNextAttributeValue(
        inDirNodeReference: tDirNodeReference,
        inOutDataBuffer: tDataBufferPtr,
        inAttributeValueIndex: UInt32,
        inOutAttributeValueOffset: *mut SInt32,
        inAttributeValueListRef: tAttributeValueListRef,
        outAttributeValue: *mut tAttributeValueEntryPtr,
    ) -> tDirStatus;
}
unsafe extern "C" {
    pub fn dsCloseAttributeList(inAttributeListRef: tAttributeListRef) -> tDirStatus;
}
unsafe extern "C" {
    pub fn dsCloseAttributeValueList(inAttributeValueListRef: tAttributeValueListRef)
        -> tDirStatus;
}
unsafe extern "C" {
    pub fn dsOpenRecord(
        inDirNodeReference: tDirNodeReference,
        inRecordType: tDataNodePtr,
        inRecordName: tDataNodePtr,
        outRecordReference: *mut tRecordReference,
    ) -> tDirStatus;
}
unsafe extern "C" {
    pub fn dsGetRecordReferenceInfo(
        inRecordReference: tRecordReference,
        outRecordInfo: *mut tRecordEntryPtr,
    ) -> tDirStatus;
}
unsafe extern "C" {
    pub fn dsGetRecordAttributeInfo(
        inRecordReference: tRecordReference,
        inAttributeType: tDataNodePtr,
        outAttributeInfoPtr: *mut tAttributeEntryPtr,
    ) -> tDirStatus;
}
unsafe extern "C" {
    pub fn dsGetRecordAttributeValueByID(
        inRecordReference: tRecordReference,
        inAttributeType: tDataNodePtr,
        inValueID: UInt32,
        outEntryPtr: *mut tAttributeValueEntryPtr,
    ) -> tDirStatus;
}
unsafe extern "C" {
    pub fn dsGetRecordAttributeValueByIndex(
        inRecordReference: tRecordReference,
        inAttributeType: tDataNodePtr,
        inValueIndex: UInt32,
        outEntryPtr: *mut tAttributeValueEntryPtr,
    ) -> tDirStatus;
}
unsafe extern "C" {
    pub fn dsGetRecordAttributeValueByValue(
        inRecordReference: tRecordReference,
        inAttributeType: tDataNodePtr,
        inAttributeValue: tDataNodePtr,
        outEntryPtr: *mut tAttributeValueEntryPtr,
    ) -> tDirStatus;
}
unsafe extern "C" {
    pub fn dsFlushRecord(inRecordReference: tRecordReference) -> tDirStatus;
}
unsafe extern "C" {
    pub fn dsCloseRecord(inRecordReference: tRecordReference) -> tDirStatus;
}
unsafe extern "C" {
    pub fn dsSetRecordName(
        inRecordReference: tRecordReference,
        inNewRecordName: tDataNodePtr,
    ) -> tDirStatus;
}
unsafe extern "C" {
    pub fn dsSetRecordType(
        inRecordReference: tRecordReference,
        inNewRecordType: tDataNodePtr,
    ) -> tDirStatus;
}
unsafe extern "C" {
    pub fn dsDeleteRecord(inRecordReference: tRecordReference) -> tDirStatus;
}
unsafe extern "C" {
    pub fn dsCreateRecord(
        inDirNodeReference: tDirNodeReference,
        inRecordType: tDataNodePtr,
        inRecordName: tDataNodePtr,
    ) -> tDirStatus;
}
unsafe extern "C" {
    pub fn dsCreateRecordAndOpen(
        inDirNodeReference: tDirNodeReference,
        inRecordType: tDataNodePtr,
        inRecordName: tDataNodePtr,
        outRecordReference: *mut tRecordReference,
    ) -> tDirStatus;
}
unsafe extern "C" {
    pub fn dsAddAttribute(
        inRecordReference: tRecordReference,
        inNewAttribute: tDataNodePtr,
        inNewAttributeAccess: tAccessControlEntryPtr,
        inFirstAttributeValue: tDataNodePtr,
    ) -> tDirStatus;
}
unsafe extern "C" {
    pub fn dsRemoveAttribute(
        inRecordReference: tRecordReference,
        inAttribute: tDataNodePtr,
    ) -> tDirStatus;
}
unsafe extern "C" {
    pub fn dsAddAttributeValue(
        inRecordReference: tRecordReference,
        inAttributeType: tDataNodePtr,
        inAttributeValue: tDataNodePtr,
    ) -> tDirStatus;
}
unsafe extern "C" {
    pub fn dsRemoveAttributeValue(
        inRecordReference: tRecordReference,
        inAttributeType: tDataNodePtr,
        inAttributeValueID: UInt32,
    ) -> tDirStatus;
}
unsafe extern "C" {
    pub fn dsSetAttributeValue(
        inRecordReference: tRecordReference,
        inAttributeType: tDataNodePtr,
        inAttributeValuePtr: tAttributeValueEntryPtr,
    ) -> tDirStatus;
}
unsafe extern "C" {
    pub fn dsSetAttributeValues(
        inRecordReference: tRecordReference,
        inAttributeType: tDataNodePtr,
        inAttributeValuesPtr: tDataListPtr,
    ) -> tDirStatus;
}
unsafe extern "C" {
    pub fn dsDoDirNodeAuth(
        inDirNodeReference: tDirNodeReference,
        inDirNodeAuthName: tDataNodePtr,
        inDirNodeAuthOnlyFlag: ::std::os::raw::c_int,
        inAuthStepData: tDataBufferPtr,
        outAuthStepDataResponse: tDataBufferPtr,
        inOutContinueData: *mut tContextData,
    ) -> tDirStatus;
}
unsafe extern "C" {
    pub fn dsDoDirNodeAuthOnRecordType(
        inDirNodeReference: tDirNodeReference,
        inDirNodeAuthName: tDataNodePtr,
        inDirNodeAuthOnlyFlag: ::std::os::raw::c_int,
        inAuthStepData: tDataBufferPtr,
        outAuthStepDataResponse: tDataBufferPtr,
        inOutContinueData: *mut tContextData,
        inRecordType: tDataNodePtr,
    ) -> tDirStatus;
}
unsafe extern "C" {
    pub fn dsDoAttributeValueSearch(
        inDirNodeReference: tDirNodeReference,
        inOutDataBuffer: tDataBufferPtr,
        inRecordTypeList: tDataListPtr,
        inAttributeType: tDataNodePtr,
        inPatternMatchType: tDirPatternMatch,
        inPattern2Match: tDataNodePtr,
        inOutMatchRecordCount: *mut UInt32,
        inOutContinueData: *mut tContextData,
    ) -> tDirStatus;
}
unsafe extern "C" {
    pub fn dsDoMultipleAttributeValueSearch(
        inDirNodeReference: tDirNodeReference,
        inOutDataBuffer: tDataBufferPtr,
        inRecordTypeList: tDataListPtr,
        inAttributeType: tDataNodePtr,
        inPatternMatchType: tDirPatternMatch,
        inPatterns2Match: tDataListPtr,
        inOutMatchRecordCount: *mut UInt32,
        inOutContinueData: *mut tContextData,
    ) -> tDirStatus;
}
unsafe extern "C" {
    pub fn dsDoAttributeValueSearchWithData(
        inDirNodeReference: tDirNodeReference,
        inOutDataBuffer: tDataBufferPtr,
        inRecordTypeList: tDataListPtr,
        inAttributeMatchType: tDataNodePtr,
        inPatternMatchType: tDirPatternMatch,
        inPatternToMatch: tDataNodePtr,
        inAttributeTypeRequestList: tDataListPtr,
        inAttributeInfoOnly: ::std::os::raw::c_int,
        inOutMatchRecordCount: *mut UInt32,
        inOutContinueData: *mut tContextData,
    ) -> tDirStatus;
}
unsafe extern "C" {
    pub fn dsDoMultipleAttributeValueSearchWithData(
        inDirNodeReference: tDirNodeReference,
        inOutDataBuffer: tDataBufferPtr,
        inRecordTypeList: tDataListPtr,
        inAttributeMatchType: tDataNodePtr,
        inPatternMatchType: tDirPatternMatch,
        inPatternsToMatch: tDataListPtr,
        inAttributeTypeRequestList: tDataListPtr,
        inAttributeInfoOnly: ::std::os::raw::c_int,
        inOutMatchRecordCount: *mut UInt32,
        inOutContinueData: *mut tContextData,
    ) -> tDirStatus;
}
unsafe extern "C" {
    pub fn dsDoPlugInCustomCall(
        inDirNodeReference: tDirNodeReference,
        inCustomRequestCode: UInt32,
        inCustomRequestData: tDataBufferPtr,
        outCustomRequestResponse: tDataBufferPtr,
    ) -> tDirStatus;
}
unsafe extern "C" {
    pub fn dsVerifyDirRefNum(inDirReference: tDirReference) -> tDirStatus;
}
unsafe extern "C" {
    pub fn dsDataBufferAllocate(
        inDirReference: tDirReference,
        inBufferSize: UInt32,
    ) -> tDataBufferPtr;
}
unsafe extern "C" {
    pub fn dsDataBufferDeAllocate(
        inDirReference: tDirReference,
        inDataBufferPtr: tDataBufferPtr,
    ) -> tDirStatus;
}
unsafe extern "C" {
    pub fn dsDataNodeAllocateBlock(
        inDirReference: tDirReference,
        inDataNodeSize: UInt32,
        inDataNodeLength: UInt32,
        inDataNodeBuffer: tBuffer,
    ) -> tDataNodePtr;
}
unsafe extern "C" {
    pub fn dsDataNodeAllocateString(
        inDirReference: tDirReference,
        inCString: *const ::std::os::raw::c_char,
    ) -> tDataNodePtr;
}
unsafe extern "C" {
    pub fn dsDataNodeDeAllocate(
        inDirReference: tDirReference,
        inDataNodePtr: tDataNodePtr,
    ) -> tDirStatus;
}
unsafe extern "C" {
    pub fn dsDataNodeSetLength(inDataNodePtr: tDataNodePtr, inDataNodeLength: UInt32)
        -> tDirStatus;
}
unsafe extern "C" {
    pub fn dsDataNodeGetLength(inDataNodePtr: tDataNodePtr) -> UInt32;
}
unsafe extern "C" {
    pub fn dsDataNodeGetSize(inDataNodePtr: tDataNodePtr) -> UInt32;
}
unsafe extern "C" {
    pub fn dsDataListAllocate(inDirReference: tDirReference) -> tDataListPtr;
}
unsafe extern "C" {
    pub fn dsDataListDeallocate(
        inDirReference: tDirReference,
        inDataList: tDataListPtr,
    ) -> tDirStatus;
}
unsafe extern "C" {
    pub fn dsDataListDeAllocate(
        inDirReference: tDirReference,
        inDataList: tDataListPtr,
        inDeAllocateNodesFlag: ::std::os::raw::c_int,
    ) -> tDirStatus;
}
unsafe extern "C" {
    pub fn dsGetPathFromList(
        inDirReference: tDirReference,
        inDataList: *const tDataList,
        inDelimiter: *const ::std::os::raw::c_char,
    ) -> *mut ::std::os::raw::c_char;
}
unsafe extern "C" {
    pub fn dsBuildFromPath(
        inDirReference: tDirReference,
        inPathCString: *const ::std::os::raw::c_char,
        inPathSeparatorCString: *const ::std::os::raw::c_char,
    ) -> tDataListPtr;
}
unsafe extern "C" {
    pub fn dsBuildListFromPathAlloc(
        inDirReference: tDirReference,
        inDataList: tDataListPtr,
        inPathCString: *const ::std::os::raw::c_char,
        inPathSeparatorCString: *const ::std::os::raw::c_char,
    ) -> tDirStatus;
}
unsafe extern "C" {
    pub fn dsBuildListFromNodesAlloc(
        inDirReferences: tDirReference,
        inDataList: tDataListPtr,
        in1stDataNodePtr: tDataNodePtr,
        ...
    ) -> tDirStatus;
}
unsafe extern "C" {
    pub fn dsBuildListFromStrings(
        inDirReference: tDirReference,
        in1stCString: *const ::std::os::raw::c_char,
        ...
    ) -> tDataListPtr;
}
unsafe extern "C" {
    pub fn dsBuildListFromStringsAlloc(
        inDirReferences: tDirReference,
        inDataList: tDataListPtr,
        in1stCString: *const ::std::os::raw::c_char,
        ...
    ) -> tDirStatus;
}
unsafe extern "C" {
    pub fn dsBuildListFromStringsAllocV(
        inDirRef: tDirReference,
        inDataList: *mut tDataList,
        in1stCString: *const ::std::os::raw::c_char,
        args: va_list,
    ) -> tDirStatus;
}
unsafe extern "C" {
    pub fn dsAppendStringToListAlloc(
        inDirReferences: tDirReference,
        inDataList: tDataListPtr,
        inCString: *const ::std::os::raw::c_char,
    ) -> tDirStatus;
}
unsafe extern "C" {
    pub fn dsDataListGetNodeCount(inDataList: *const tDataList) -> UInt32;
}
unsafe extern "C" {
    pub fn dsAllocStringsFromList(
        inDirRef: tDirReference,
        inDataList: *const tDataList,
    ) -> *mut *mut ::std::os::raw::c_char;
}
unsafe extern "C" {
    pub fn dsGetDataLength(inDataList: *const tDataList) -> UInt32;
}
unsafe extern "C" {
    pub fn dsDataListInsertAfter(
        inDirReferences: tDirReference,
        inDataList: tDataListPtr,
        inInsertDataNode: tDataNodePtr,
        inNodeIndex: UInt32,
    ) -> tDirStatus;
}
unsafe extern "C" {
    pub fn dsDataListMergeListAfter(
        inTargetList: tDataListPtr,
        inSourceList: tDataListPtr,
        inNodeIndex: UInt32,
    ) -> tDirStatus;
}
unsafe extern "C" {
    pub fn dsDataListCopyList(
        inDirReference: tDirReference,
        inDataListSource: *const tDataList,
    ) -> tDataListPtr;
}
unsafe extern "C" {
    pub fn dsDataListDeleteThisNode(
        inDirReference: tDirReference,
        inDataList: tDataListPtr,
        inNodeIndex: UInt32,
    ) -> tDirStatus;
}
unsafe extern "C" {
    pub fn dsDataListGetNodeAlloc(
        inDirReference: tDirReference,
        inDataListPtr: *const tDataList,
        inNodeIndex: UInt32,
        outDataNode: *mut tDataNodePtr,
    ) -> tDirStatus;
}
unsafe extern "C" {
    pub fn dsAllocAttributeValueEntry(
        inDirRef: tDirReference,
        inAttrValueID: UInt32,
        inAttrValueData: *mut ::std::os::raw::c_void,
        inAttrValueDataLen: UInt32,
    ) -> tAttributeValueEntryPtr;
}
unsafe extern "C" {
    pub fn dsDeallocAttributeValueEntry(
        inDirRef: tDirReference,
        inAttrValueEntry: tAttributeValueEntryPtr,
    ) -> tDirStatus;
}
unsafe extern "C" {
    pub fn dsDeallocAttributeEntry(
        inDirRef: tDirReference,
        inAttrEntry: tAttributeEntryPtr,
    ) -> tDirStatus;
}
unsafe extern "C" {
    pub fn dsDeallocRecordEntry(inDirRef: tDirReference, inRecEntry: tRecordEntryPtr)
        -> tDirStatus;
}
unsafe extern "C" {
    pub fn dsGetRecordNameFromEntry(
        inRecEntryPtr: tRecordEntryPtr,
        outRecName: *mut *mut ::std::os::raw::c_char,
    ) -> tDirStatus;
}
unsafe extern "C" {
    pub fn dsGetRecordTypeFromEntry(
        inRecEntryPtr: tRecordEntryPtr,
        outRecType: *mut *mut ::std::os::raw::c_char,
    ) -> tDirStatus;
}
unsafe extern "C" {
    pub fn dsParseAuthAuthority(
        inAuthAuthority: *const ::std::os::raw::c_char,
        outVersion: *mut *mut ::std::os::raw::c_char,
        outAuthTag: *mut *mut ::std::os::raw::c_char,
        outAuthData: *mut *mut ::std::os::raw::c_char,
    ) -> tDirStatus;
}
unsafe extern "C" {
    pub fn dsCopyDirStatusName(inDirStatus: SInt32) -> *mut ::std::os::raw::c_char;
}
unsafe extern "C" {
    pub fn dsFillAuthBuffer(
        inOutAuthBuffer: tDataBufferPtr,
        inCount: UInt32,
        inLen: UInt32,
        inData: *const ::std::os::raw::c_void,
        ...
    ) -> tDirStatus;
}
unsafe extern "C" {
    pub fn dsAppendAuthBuffer(
        inOutAuthBuffer: tDataBufferPtr,
        inCount: UInt32,
        inLen: UInt32,
        inData: *const ::std::os::raw::c_void,
        ...
    ) -> tDirStatus;
}
unsafe extern "C" {
    pub fn dsAppendAuthBufferWithAuthorityAttribute(
        inNodeRef: tDirNodeReference,
        inRecordListBuffPtr: tDataBufferPtr,
        inAttributePtr: tAttributeEntryPtr,
        inValueRef: tAttributeValueListRef,
        inUserName: *const ::std::os::raw::c_char,
        inOutAuthBuffer: tDataBufferPtr,
    ) -> tDirStatus;
}
unsafe extern "C" {
    pub fn dsAppendAuthBufferWithAuthorityStrings(
        inUserName: *const ::std::os::raw::c_char,
        inAuthAuthority: *mut *const ::std::os::raw::c_char,
        inOutAuthBuffer: tDataBufferPtr,
    ) -> tDirStatus;
}
unsafe extern "C" {
    pub fn dsServiceInformationAllocate(
        inServiceInfo: CFDictionaryRef,
        inBufferSize: UInt32,
        outPackedServiceInfo: *mut tDataBufferPtr,
    ) -> tDirStatus;
}
unsafe extern "C" {
    pub fn dsBuildListFromNodes(
        inDirReferences: tDirReference,
        in1stDataNodePtr: tDataNodePtr,
        ...
    ) -> tDataListPtr;
}
unsafe extern "C" {
    pub fn dsAppendStringToList(
        inDataList: tDataListPtr,
        inCString: *const ::std::os::raw::c_char,
    ) -> tDirStatus;
}
unsafe extern "C" {
    pub fn dsDataListInsertNode(
        inDataList: tDataListPtr,
        inAfterDataNode: tDataNodePtr,
        inInsertDataNode: tDataNodePtr,
    ) -> tDirStatus;
}
unsafe extern "C" {
    pub fn dsDataListMergeList(
        inDataList: tDataListPtr,
        inAfterDataNode: tDataNodePtr,
        inMergeDataList: tDataListPtr,
    ) -> tDirStatus;
}
unsafe extern "C" {
    pub fn dsDataListRemoveThisNode(
        inDataList: tDataListPtr,
        inNodeIndex: UInt32,
        inDeleteCount: UInt32,
    ) -> tDirStatus;
}
unsafe extern "C" {
    pub fn dsDataListRemoveNodes(
        inDataList: tDataListPtr,
        in1stDataNode: tDataNodePtr,
        inDeleteCount: UInt32,
    ) -> tDirStatus;
}
unsafe extern "C" {
    pub fn dsDataListGetNode(
        inDataListPtr: tDataListPtr,
        inNodeIndex: UInt32,
        outDataNode: *mut tDataNodePtr,
    ) -> tDirStatus;
}
unsafe extern "C" {
    pub fn dsRegisterCustomMemory(
        inDirReference: tDirReference,
        inCustomAllocate: fpCustomAllocate,
        inCustomDeAllocate: fpCustomDeAllocate,
        inClientData: tClientData,
    ) -> tDirStatus;
}
unsafe extern "C" {
    pub fn dsGetCustomAllocate(
        inDirReference: tDirReference,
        outCustomAllocate: *mut fpCustomAllocate,
        outCustomDeAllocate: *mut fpCustomDeAllocate,
        outClientData: *mut tClientData,
    ) -> tDirStatus;
}
unsafe extern "C" {
    pub fn dsUnRegisterCustomMemory(
        inDirReference: tDirReference,
        inCustomAllocate: fpCustomAllocate,
        inCustomDeAllocate: fpCustomDeAllocate,
        inClientData: tClientData,
    ) -> tDirStatus;
}
unsafe extern "C" {
    pub fn dsRegisterCustomThread(
        inDirReference: tDirReference,
        inCustomBlock: fpCustomThreadBlock,
        inCustomUnBlock: fpCustomThreadUnBlock,
        inCustomYield: fpCustomThreadYield,
        inClientData: tClientData,
    ) -> tDirStatus;
}
unsafe extern "C" {
    pub fn dsGetCustomThread(
        inDirReference: tDirReference,
        outCustomBlock: *mut fpCustomThreadBlock,
        outCustomUnBlock: *mut fpCustomThreadUnBlock,
        outCustomYield: *mut fpCustomThreadYield,
        outClientData: *mut tClientData,
    ) -> tDirStatus;
}
unsafe extern "C" {
    pub fn dsUnRegisterCustomThread(
        inDirReference: tDirReference,
        inCustomBlock: fpCustomThreadBlock,
        inCustomUnBlock: fpCustomThreadUnBlock,
        inCustomYield: fpCustomThreadYield,
        inClientData: tClientData,
    ) -> tDirStatus;
}

unsafe impl objc2::encode::RefEncode for tDataBuffer {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for tDataBuffer {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("tDataBuffer", &[]);
}
unsafe impl objc2::encode::RefEncode for tDataList {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for tDataList {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("tDataList", &[]);
}
unsafe impl objc2::encode::RefEncode for tAccessControlEntry {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for tAccessControlEntry {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("tAccessControlEntry", &[]);
}
unsafe impl objc2::encode::RefEncode for tAttributeValueEntry {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for tAttributeValueEntry {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("tAttributeValueEntry", &[]);
}
unsafe impl objc2::encode::RefEncode for tAttributeEntry {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for tAttributeEntry {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("tAttributeEntry", &[]);
}
unsafe impl objc2::encode::RefEncode for tRecordEntry {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for tRecordEntry {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("tRecordEntry", &[]);
}
