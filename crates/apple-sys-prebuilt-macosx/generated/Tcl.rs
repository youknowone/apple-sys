#[allow(unused_imports)]
use crate::CoreFoundation::*;

pub type ClientData = *mut ::std::os::raw::c_void;
pub type Tcl_WideInt = ::std::os::raw::c_long;
pub type Tcl_WideUInt = ::std::os::raw::c_ulong;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct stat {
    _unused: [u8; 0],
}
pub type Tcl_StatBuf = stat;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Tcl_Interp {
    pub result: *mut ::std::os::raw::c_char,
    pub freeProc:
        ::std::option::Option<unsafe extern "C" fn(blockPtr: *mut ::std::os::raw::c_char)>,
    pub errorLine: ::std::os::raw::c_int,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Tcl_AsyncHandler_ {
    _unused: [u8; 0],
}
pub type Tcl_AsyncHandler = *mut Tcl_AsyncHandler_;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Tcl_Channel_ {
    _unused: [u8; 0],
}
pub type Tcl_Channel = *mut Tcl_Channel_;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Tcl_ChannelTypeVersion_ {
    _unused: [u8; 0],
}
pub type Tcl_ChannelTypeVersion = *mut Tcl_ChannelTypeVersion_;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Tcl_Command_ {
    _unused: [u8; 0],
}
pub type Tcl_Command = *mut Tcl_Command_;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Tcl_Condition_ {
    _unused: [u8; 0],
}
pub type Tcl_Condition = *mut Tcl_Condition_;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Tcl_Dict_ {
    _unused: [u8; 0],
}
pub type Tcl_Dict = *mut Tcl_Dict_;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Tcl_EncodingState_ {
    _unused: [u8; 0],
}
pub type Tcl_EncodingState = *mut Tcl_EncodingState_;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Tcl_Encoding_ {
    _unused: [u8; 0],
}
pub type Tcl_Encoding = *mut Tcl_Encoding_;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Tcl_InterpState_ {
    _unused: [u8; 0],
}
pub type Tcl_InterpState = *mut Tcl_InterpState_;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Tcl_LoadHandle_ {
    _unused: [u8; 0],
}
pub type Tcl_LoadHandle = *mut Tcl_LoadHandle_;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Tcl_Mutex_ {
    _unused: [u8; 0],
}
pub type Tcl_Mutex = *mut Tcl_Mutex_;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Tcl_Pid_ {
    _unused: [u8; 0],
}
pub type Tcl_Pid = *mut Tcl_Pid_;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Tcl_RegExp_ {
    _unused: [u8; 0],
}
pub type Tcl_RegExp = *mut Tcl_RegExp_;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Tcl_ThreadDataKey_ {
    _unused: [u8; 0],
}
pub type Tcl_ThreadDataKey = *mut Tcl_ThreadDataKey_;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Tcl_ThreadId_ {
    _unused: [u8; 0],
}
pub type Tcl_ThreadId = *mut Tcl_ThreadId_;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Tcl_TimerToken_ {
    _unused: [u8; 0],
}
pub type Tcl_TimerToken = *mut Tcl_TimerToken_;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Tcl_Trace_ {
    _unused: [u8; 0],
}
pub type Tcl_Trace = *mut Tcl_Trace_;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Tcl_Var_ {
    _unused: [u8; 0],
}
pub type Tcl_Var = *mut Tcl_Var_;
pub type Tcl_ThreadCreateProc = ::std::option::Option<unsafe extern "C" fn(clientData: ClientData)>;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Tcl_RegExpIndices {
    pub start: ::std::os::raw::c_long,
    pub end: ::std::os::raw::c_long,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Tcl_RegExpInfo {
    pub nsubs: ::std::os::raw::c_int,
    pub matches: *mut Tcl_RegExpIndices,
    pub extendStart: ::std::os::raw::c_long,
    pub reserved: ::std::os::raw::c_long,
}
pub type Tcl_Stat_ = *mut Tcl_StatBuf;
pub type Tcl_OldStat_ = *mut stat;
pub type Tcl_ValueType = ::std::os::raw::c_uint;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Tcl_Value {
    pub type_: Tcl_ValueType,
    pub intValue: ::std::os::raw::c_long,
    pub doubleValue: f64,
    pub wideValue: Tcl_WideInt,
}
pub type Tcl_AppInitProc =
    ::std::option::Option<unsafe extern "C" fn(interp: *mut Tcl_Interp) -> ::std::os::raw::c_int>;
pub type Tcl_AsyncProc = ::std::option::Option<
    unsafe extern "C" fn(
        clientData: ClientData,
        interp: *mut Tcl_Interp,
        code: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int,
>;
pub type Tcl_ChannelProc = ::std::option::Option<
    unsafe extern "C" fn(clientData: ClientData, mask: ::std::os::raw::c_int),
>;
pub type Tcl_CloseProc = ::std::option::Option<unsafe extern "C" fn(data: ClientData)>;
pub type Tcl_CmdDeleteProc = ::std::option::Option<unsafe extern "C" fn(clientData: ClientData)>;
pub type Tcl_CmdProc = ::std::option::Option<
    unsafe extern "C" fn(
        clientData: ClientData,
        interp: *mut Tcl_Interp,
        argc: ::std::os::raw::c_int,
        argv: *mut *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int,
>;
pub type Tcl_CmdTraceProc = ::std::option::Option<
    unsafe extern "C" fn(
        clientData: ClientData,
        interp: *mut Tcl_Interp,
        level: ::std::os::raw::c_int,
        command: *mut ::std::os::raw::c_char,
        proc_: Tcl_CmdProc,
        cmdClientData: ClientData,
        argc: ::std::os::raw::c_int,
        argv: *mut *const ::std::os::raw::c_char,
    ),
>;
pub type Tcl_CmdObjTraceProc = ::std::option::Option<
    unsafe extern "C" fn(
        clientData: ClientData,
        interp: *mut Tcl_Interp,
        level: ::std::os::raw::c_int,
        command: *const ::std::os::raw::c_char,
        commandInfo: Tcl_Command,
        objc: ::std::os::raw::c_int,
        objv: *const *mut Tcl_Obj,
    ) -> ::std::os::raw::c_int,
>;
pub type Tcl_CmdObjTraceDeleteProc =
    ::std::option::Option<unsafe extern "C" fn(clientData: ClientData)>;
pub type Tcl_DupInternalRepProc =
    ::std::option::Option<unsafe extern "C" fn(srcPtr: *mut Tcl_Obj, dupPtr: *mut Tcl_Obj)>;
pub type Tcl_EncodingConvertProc = ::std::option::Option<
    unsafe extern "C" fn(
        clientData: ClientData,
        src: *const ::std::os::raw::c_char,
        srcLen: ::std::os::raw::c_int,
        flags: ::std::os::raw::c_int,
        statePtr: *mut Tcl_EncodingState,
        dst: *mut ::std::os::raw::c_char,
        dstLen: ::std::os::raw::c_int,
        srcReadPtr: *mut ::std::os::raw::c_int,
        dstWrotePtr: *mut ::std::os::raw::c_int,
        dstCharsPtr: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int,
>;
pub type Tcl_EncodingFreeProc = ::std::option::Option<unsafe extern "C" fn(clientData: ClientData)>;
pub type Tcl_EventProc = ::std::option::Option<
    unsafe extern "C" fn(
        evPtr: *mut Tcl_Event,
        flags: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int,
>;
pub type Tcl_EventCheckProc = ::std::option::Option<
    unsafe extern "C" fn(clientData: ClientData, flags: ::std::os::raw::c_int),
>;
pub type Tcl_EventDeleteProc = ::std::option::Option<
    unsafe extern "C" fn(evPtr: *mut Tcl_Event, clientData: ClientData) -> ::std::os::raw::c_int,
>;
pub type Tcl_EventSetupProc = ::std::option::Option<
    unsafe extern "C" fn(clientData: ClientData, flags: ::std::os::raw::c_int),
>;
pub type Tcl_ExitProc = ::std::option::Option<unsafe extern "C" fn(clientData: ClientData)>;
pub type Tcl_FileProc = ::std::option::Option<
    unsafe extern "C" fn(clientData: ClientData, mask: ::std::os::raw::c_int),
>;
pub type Tcl_FreeInternalRepProc =
    ::std::option::Option<unsafe extern "C" fn(objPtr: *mut Tcl_Obj)>;
pub type Tcl_FreeProc =
    ::std::option::Option<unsafe extern "C" fn(blockPtr: *mut ::std::os::raw::c_char)>;
pub type Tcl_IdleProc = ::std::option::Option<unsafe extern "C" fn(clientData: ClientData)>;
pub type Tcl_InterpDeleteProc =
    ::std::option::Option<unsafe extern "C" fn(clientData: ClientData, interp: *mut Tcl_Interp)>;
pub type Tcl_MathProc = ::std::option::Option<
    unsafe extern "C" fn(
        clientData: ClientData,
        interp: *mut Tcl_Interp,
        args: *mut Tcl_Value,
        resultPtr: *mut Tcl_Value,
    ) -> ::std::os::raw::c_int,
>;
pub type Tcl_NamespaceDeleteProc =
    ::std::option::Option<unsafe extern "C" fn(clientData: ClientData)>;
pub type Tcl_ObjCmdProc = ::std::option::Option<
    unsafe extern "C" fn(
        clientData: ClientData,
        interp: *mut Tcl_Interp,
        objc: ::std::os::raw::c_int,
        objv: *const *mut Tcl_Obj,
    ) -> ::std::os::raw::c_int,
>;
pub type Tcl_PackageInitProc =
    ::std::option::Option<unsafe extern "C" fn(interp: *mut Tcl_Interp) -> ::std::os::raw::c_int>;
pub type Tcl_PanicProc =
    ::std::option::Option<unsafe extern "C" fn(format: *const ::std::os::raw::c_char, ...)>;
pub type Tcl_TcpAcceptProc = ::std::option::Option<
    unsafe extern "C" fn(
        callbackData: ClientData,
        chan: Tcl_Channel,
        address: *mut ::std::os::raw::c_char,
        port: ::std::os::raw::c_int,
    ),
>;
pub type Tcl_TimerProc = ::std::option::Option<unsafe extern "C" fn(clientData: ClientData)>;
pub type Tcl_SetFromAnyProc = ::std::option::Option<
    unsafe extern "C" fn(interp: *mut Tcl_Interp, objPtr: *mut Tcl_Obj) -> ::std::os::raw::c_int,
>;
pub type Tcl_UpdateStringProc = ::std::option::Option<unsafe extern "C" fn(objPtr: *mut Tcl_Obj)>;
pub type Tcl_VarTraceProc = ::std::option::Option<
    unsafe extern "C" fn(
        clientData: ClientData,
        interp: *mut Tcl_Interp,
        part1: *const ::std::os::raw::c_char,
        part2: *const ::std::os::raw::c_char,
        flags: ::std::os::raw::c_int,
    ) -> *mut ::std::os::raw::c_char,
>;
pub type Tcl_CommandTraceProc = ::std::option::Option<
    unsafe extern "C" fn(
        clientData: ClientData,
        interp: *mut Tcl_Interp,
        oldName: *const ::std::os::raw::c_char,
        newName: *const ::std::os::raw::c_char,
        flags: ::std::os::raw::c_int,
    ),
>;
pub type Tcl_CreateFileHandlerProc = ::std::option::Option<
    unsafe extern "C" fn(
        fd: ::std::os::raw::c_int,
        mask: ::std::os::raw::c_int,
        proc_: Tcl_FileProc,
        clientData: ClientData,
    ),
>;
pub type Tcl_DeleteFileHandlerProc =
    ::std::option::Option<unsafe extern "C" fn(fd: ::std::os::raw::c_int)>;
pub type Tcl_AlertNotifierProc =
    ::std::option::Option<unsafe extern "C" fn(clientData: ClientData)>;
pub type Tcl_ServiceModeHookProc =
    ::std::option::Option<unsafe extern "C" fn(mode: ::std::os::raw::c_int)>;
pub type Tcl_InitNotifierProc = ::std::option::Option<unsafe extern "C" fn() -> ClientData>;
pub type Tcl_FinalizeNotifierProc =
    ::std::option::Option<unsafe extern "C" fn(clientData: ClientData)>;
pub type Tcl_MainLoopProc = ::std::option::Option<unsafe extern "C" fn()>;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Tcl_ObjType {
    pub name: *mut ::std::os::raw::c_char,
    pub freeIntRepProc: Tcl_FreeInternalRepProc,
    pub dupIntRepProc: Tcl_DupInternalRepProc,
    pub updateStringProc: Tcl_UpdateStringProc,
    pub setFromAnyProc: Tcl_SetFromAnyProc,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Tcl_Obj {
    pub __bindgen_anon_1: Tcl_Obj__bindgen_ty_1,
    pub refCount: ::std::os::raw::c_int,
    pub bytes: *mut ::std::os::raw::c_char,
    pub length: ::std::os::raw::c_int,
    pub typePtr: *mut Tcl_ObjType,
    pub internalRep: Tcl_Obj__bindgen_ty_1,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union Tcl_Obj__bindgen_ty_1 {
    pub __bindgen_anon_1: Tcl_Obj__bindgen_ty_1__bindgen_ty_1,
    pub __bindgen_anon_2: Tcl_Obj__bindgen_ty_1__bindgen_ty_2,
    pub longValue: ::std::os::raw::c_long,
    pub doubleValue: f64,
    pub otherValuePtr: *mut ::std::os::raw::c_void,
    pub wideValue: Tcl_WideInt,
    pub twoPtrValue: Tcl_Obj__bindgen_ty_1__bindgen_ty_1,
    pub ptrAndLongRep: Tcl_Obj__bindgen_ty_1__bindgen_ty_2,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Tcl_Obj__bindgen_ty_1__bindgen_ty_1 {
    pub ptr1: *mut ::std::os::raw::c_void,
    pub ptr2: *mut ::std::os::raw::c_void,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Tcl_Obj__bindgen_ty_1__bindgen_ty_2 {
    pub ptr: *mut ::std::os::raw::c_void,
    pub value: ::std::os::raw::c_ulong,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Tcl_SavedResult {
    pub result: *mut ::std::os::raw::c_char,
    pub freeProc: Tcl_FreeProc,
    pub objResultPtr: *mut Tcl_Obj,
    pub appendResult: *mut ::std::os::raw::c_char,
    pub appendAvl: ::std::os::raw::c_int,
    pub appendUsed: ::std::os::raw::c_int,
    pub resultSpace: [::std::os::raw::c_char; 201usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Tcl_Namespace {
    pub name: *mut ::std::os::raw::c_char,
    pub fullName: *mut ::std::os::raw::c_char,
    pub clientData: ClientData,
    pub deleteProc: Tcl_NamespaceDeleteProc,
    pub parentPtr: *mut Tcl_Namespace,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Tcl_CallFrame {
    pub nsPtr: *mut Tcl_Namespace,
    pub dummy1: ::std::os::raw::c_int,
    pub dummy2: ::std::os::raw::c_int,
    pub dummy3: *mut ::std::os::raw::c_void,
    pub dummy4: *mut ::std::os::raw::c_void,
    pub dummy5: *mut ::std::os::raw::c_void,
    pub dummy6: ::std::os::raw::c_int,
    pub dummy7: *mut ::std::os::raw::c_void,
    pub dummy8: *mut ::std::os::raw::c_void,
    pub dummy9: ::std::os::raw::c_int,
    pub dummy10: *mut ::std::os::raw::c_void,
    pub dummy11: *mut ::std::os::raw::c_void,
    pub dummy12: *mut ::std::os::raw::c_void,
    pub dummy13: *mut ::std::os::raw::c_void,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Tcl_CmdInfo {
    pub isNativeObjectProc: ::std::os::raw::c_int,
    pub objProc: Tcl_ObjCmdProc,
    pub objClientData: ClientData,
    pub proc_: Tcl_CmdProc,
    pub clientData: ClientData,
    pub deleteProc: Tcl_CmdDeleteProc,
    pub deleteData: ClientData,
    pub namespacePtr: *mut Tcl_Namespace,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Tcl_DString {
    pub string: *mut ::std::os::raw::c_char,
    pub length: ::std::os::raw::c_int,
    pub spaceAvl: ::std::os::raw::c_int,
    pub staticSpace: [::std::os::raw::c_char; 200usize],
}
pub type Tcl_HashKeyProc = ::std::option::Option<
    unsafe extern "C" fn(
        tablePtr: *mut Tcl_HashTable,
        keyPtr: *mut ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_uint,
>;
pub type Tcl_CompareHashKeysProc = ::std::option::Option<
    unsafe extern "C" fn(
        keyPtr: *mut ::std::os::raw::c_void,
        hPtr: *mut Tcl_HashEntry,
    ) -> ::std::os::raw::c_int,
>;
pub type Tcl_AllocHashEntryProc = ::std::option::Option<
    unsafe extern "C" fn(
        tablePtr: *mut Tcl_HashTable,
        keyPtr: *mut ::std::os::raw::c_void,
    ) -> *mut Tcl_HashEntry,
>;
pub type Tcl_FreeHashEntryProc =
    ::std::option::Option<unsafe extern "C" fn(hPtr: *mut Tcl_HashEntry)>;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Tcl_HashEntry {
    pub __bindgen_anon_1: Tcl_HashEntry__bindgen_ty_1,
    pub nextPtr: *mut Tcl_HashEntry,
    pub tablePtr: *mut Tcl_HashTable,
    pub hash: *mut ::std::os::raw::c_void,
    pub clientData: ClientData,
    pub key: Tcl_HashEntry__bindgen_ty_1,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union Tcl_HashEntry__bindgen_ty_1 {
    pub oneWordValue: *mut ::std::os::raw::c_char,
    pub objPtr: *mut Tcl_Obj,
    pub words: [::std::os::raw::c_int; 1usize],
    pub string: [::std::os::raw::c_char; 1usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Tcl_HashKeyType {
    pub version: ::std::os::raw::c_int,
    pub flags: ::std::os::raw::c_int,
    pub hashKeyProc: Tcl_HashKeyProc,
    pub compareKeysProc: Tcl_CompareHashKeysProc,
    pub allocEntryProc: Tcl_AllocHashEntryProc,
    pub freeEntryProc: Tcl_FreeHashEntryProc,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Tcl_HashTable {
    pub buckets: *mut *mut Tcl_HashEntry,
    pub staticBuckets: [*mut Tcl_HashEntry; 4usize],
    pub numBuckets: ::std::os::raw::c_int,
    pub numEntries: ::std::os::raw::c_int,
    pub rebuildSize: ::std::os::raw::c_int,
    pub downShift: ::std::os::raw::c_int,
    pub mask: ::std::os::raw::c_int,
    pub keyType: ::std::os::raw::c_int,
    pub findProc: ::std::option::Option<
        unsafe extern "C" fn(
            tablePtr: *mut Tcl_HashTable,
            key: *const ::std::os::raw::c_char,
        ) -> *mut Tcl_HashEntry,
    >,
    pub createProc: ::std::option::Option<
        unsafe extern "C" fn(
            tablePtr: *mut Tcl_HashTable,
            key: *const ::std::os::raw::c_char,
            newPtr: *mut ::std::os::raw::c_int,
        ) -> *mut Tcl_HashEntry,
    >,
    pub typePtr: *mut Tcl_HashKeyType,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Tcl_HashSearch {
    pub tablePtr: *mut Tcl_HashTable,
    pub nextIndex: ::std::os::raw::c_int,
    pub nextEntryPtr: *mut Tcl_HashEntry,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Tcl_DictSearch {
    pub next: *mut ::std::os::raw::c_void,
    pub epoch: ::std::os::raw::c_int,
    pub dictionaryPtr: Tcl_Dict,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Tcl_Event {
    pub proc_: Tcl_EventProc,
    pub nextPtr: *mut Tcl_Event,
}
pub type Tcl_QueuePosition = ::std::os::raw::c_uint;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Tcl_Time {
    pub sec: ::std::os::raw::c_long,
    pub usec: ::std::os::raw::c_long,
}
pub type Tcl_SetTimerProc = ::std::option::Option<unsafe extern "C" fn(timePtr: *mut Tcl_Time)>;
pub type Tcl_WaitForEventProc =
    ::std::option::Option<unsafe extern "C" fn(timePtr: *mut Tcl_Time) -> ::std::os::raw::c_int>;
pub type Tcl_GetTimeProc =
    ::std::option::Option<unsafe extern "C" fn(timebuf: *mut Tcl_Time, clientData: ClientData)>;
pub type Tcl_ScaleTimeProc =
    ::std::option::Option<unsafe extern "C" fn(timebuf: *mut Tcl_Time, clientData: ClientData)>;
pub type Tcl_DriverBlockModeProc = ::std::option::Option<
    unsafe extern "C" fn(
        instanceData: ClientData,
        mode: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int,
>;
pub type Tcl_DriverCloseProc = ::std::option::Option<
    unsafe extern "C" fn(
        instanceData: ClientData,
        interp: *mut Tcl_Interp,
    ) -> ::std::os::raw::c_int,
>;
pub type Tcl_DriverClose2Proc = ::std::option::Option<
    unsafe extern "C" fn(
        instanceData: ClientData,
        interp: *mut Tcl_Interp,
        flags: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int,
>;
pub type Tcl_DriverInputProc = ::std::option::Option<
    unsafe extern "C" fn(
        instanceData: ClientData,
        buf: *mut ::std::os::raw::c_char,
        toRead: ::std::os::raw::c_int,
        errorCodePtr: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int,
>;
pub type Tcl_DriverOutputProc = ::std::option::Option<
    unsafe extern "C" fn(
        instanceData: ClientData,
        buf: *const ::std::os::raw::c_char,
        toWrite: ::std::os::raw::c_int,
        errorCodePtr: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int,
>;
pub type Tcl_DriverSeekProc = ::std::option::Option<
    unsafe extern "C" fn(
        instanceData: ClientData,
        offset: ::std::os::raw::c_long,
        mode: ::std::os::raw::c_int,
        errorCodePtr: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int,
>;
pub type Tcl_DriverSetOptionProc = ::std::option::Option<
    unsafe extern "C" fn(
        instanceData: ClientData,
        interp: *mut Tcl_Interp,
        optionName: *const ::std::os::raw::c_char,
        value: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int,
>;
pub type Tcl_DriverGetOptionProc = ::std::option::Option<
    unsafe extern "C" fn(
        instanceData: ClientData,
        interp: *mut Tcl_Interp,
        optionName: *const ::std::os::raw::c_char,
        dsPtr: *mut Tcl_DString,
    ) -> ::std::os::raw::c_int,
>;
pub type Tcl_DriverWatchProc = ::std::option::Option<
    unsafe extern "C" fn(instanceData: ClientData, mask: ::std::os::raw::c_int),
>;
pub type Tcl_DriverGetHandleProc = ::std::option::Option<
    unsafe extern "C" fn(
        instanceData: ClientData,
        direction: ::std::os::raw::c_int,
        handlePtr: *mut ClientData,
    ) -> ::std::os::raw::c_int,
>;
pub type Tcl_DriverFlushProc =
    ::std::option::Option<unsafe extern "C" fn(instanceData: ClientData) -> ::std::os::raw::c_int>;
pub type Tcl_DriverHandlerProc = ::std::option::Option<
    unsafe extern "C" fn(
        instanceData: ClientData,
        interestMask: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int,
>;
pub type Tcl_DriverWideSeekProc = ::std::option::Option<
    unsafe extern "C" fn(
        instanceData: ClientData,
        offset: Tcl_WideInt,
        mode: ::std::os::raw::c_int,
        errorCodePtr: *mut ::std::os::raw::c_int,
    ) -> Tcl_WideInt,
>;
pub type Tcl_DriverThreadActionProc = ::std::option::Option<
    unsafe extern "C" fn(instanceData: ClientData, action: ::std::os::raw::c_int),
>;
pub type Tcl_DriverTruncateProc = ::std::option::Option<
    unsafe extern "C" fn(instanceData: ClientData, length: Tcl_WideInt) -> ::std::os::raw::c_int,
>;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Tcl_ChannelType {
    pub typeName: *mut ::std::os::raw::c_char,
    pub version: Tcl_ChannelTypeVersion,
    pub closeProc: Tcl_DriverCloseProc,
    pub inputProc: Tcl_DriverInputProc,
    pub outputProc: Tcl_DriverOutputProc,
    pub seekProc: Tcl_DriverSeekProc,
    pub setOptionProc: Tcl_DriverSetOptionProc,
    pub getOptionProc: Tcl_DriverGetOptionProc,
    pub watchProc: Tcl_DriverWatchProc,
    pub getHandleProc: Tcl_DriverGetHandleProc,
    pub close2Proc: Tcl_DriverClose2Proc,
    pub blockModeProc: Tcl_DriverBlockModeProc,
    pub flushProc: Tcl_DriverFlushProc,
    pub handlerProc: Tcl_DriverHandlerProc,
    pub wideSeekProc: Tcl_DriverWideSeekProc,
    pub threadActionProc: Tcl_DriverThreadActionProc,
    pub truncateProc: Tcl_DriverTruncateProc,
}
pub type Tcl_PathType = ::std::os::raw::c_uint;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Tcl_GlobTypeData {
    pub type_: ::std::os::raw::c_int,
    pub perm: ::std::os::raw::c_int,
    pub macType: *mut Tcl_Obj,
    pub macCreator: *mut Tcl_Obj,
}
pub type Tcl_FSStatProc = ::std::option::Option<
    unsafe extern "C" fn(pathPtr: *mut Tcl_Obj, buf: *mut Tcl_StatBuf) -> ::std::os::raw::c_int,
>;
pub type Tcl_FSAccessProc = ::std::option::Option<
    unsafe extern "C" fn(
        pathPtr: *mut Tcl_Obj,
        mode: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int,
>;
pub type Tcl_FSOpenFileChannelProc = ::std::option::Option<
    unsafe extern "C" fn(
        interp: *mut Tcl_Interp,
        pathPtr: *mut Tcl_Obj,
        mode: ::std::os::raw::c_int,
        permissions: ::std::os::raw::c_int,
    ) -> Tcl_Channel,
>;
pub type Tcl_FSMatchInDirectoryProc = ::std::option::Option<
    unsafe extern "C" fn(
        interp: *mut Tcl_Interp,
        result: *mut Tcl_Obj,
        pathPtr: *mut Tcl_Obj,
        pattern: *const ::std::os::raw::c_char,
        types: *mut Tcl_GlobTypeData,
    ) -> ::std::os::raw::c_int,
>;
pub type Tcl_FSGetCwdProc =
    ::std::option::Option<unsafe extern "C" fn(interp: *mut Tcl_Interp) -> *mut Tcl_Obj>;
pub type Tcl_FSChdirProc =
    ::std::option::Option<unsafe extern "C" fn(pathPtr: *mut Tcl_Obj) -> ::std::os::raw::c_int>;
pub type Tcl_FSLstatProc = ::std::option::Option<
    unsafe extern "C" fn(pathPtr: *mut Tcl_Obj, buf: *mut Tcl_StatBuf) -> ::std::os::raw::c_int,
>;
pub type Tcl_FSCreateDirectoryProc =
    ::std::option::Option<unsafe extern "C" fn(pathPtr: *mut Tcl_Obj) -> ::std::os::raw::c_int>;
pub type Tcl_FSDeleteFileProc =
    ::std::option::Option<unsafe extern "C" fn(pathPtr: *mut Tcl_Obj) -> ::std::os::raw::c_int>;
pub type Tcl_FSCopyDirectoryProc = ::std::option::Option<
    unsafe extern "C" fn(
        srcPathPtr: *mut Tcl_Obj,
        destPathPtr: *mut Tcl_Obj,
        errorPtr: *mut *mut Tcl_Obj,
    ) -> ::std::os::raw::c_int,
>;
pub type Tcl_FSCopyFileProc = ::std::option::Option<
    unsafe extern "C" fn(
        srcPathPtr: *mut Tcl_Obj,
        destPathPtr: *mut Tcl_Obj,
    ) -> ::std::os::raw::c_int,
>;
pub type Tcl_FSRemoveDirectoryProc = ::std::option::Option<
    unsafe extern "C" fn(
        pathPtr: *mut Tcl_Obj,
        recursive: ::std::os::raw::c_int,
        errorPtr: *mut *mut Tcl_Obj,
    ) -> ::std::os::raw::c_int,
>;
pub type Tcl_FSRenameFileProc = ::std::option::Option<
    unsafe extern "C" fn(
        srcPathPtr: *mut Tcl_Obj,
        destPathPtr: *mut Tcl_Obj,
    ) -> ::std::os::raw::c_int,
>;
pub type Tcl_FSUnloadFileProc =
    ::std::option::Option<unsafe extern "C" fn(loadHandle: Tcl_LoadHandle)>;
pub type Tcl_FSListVolumesProc = ::std::option::Option<unsafe extern "C" fn() -> *mut Tcl_Obj>;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct utimbuf {
    _unused: [u8; 0],
}
pub type Tcl_FSUtimeProc = ::std::option::Option<
    unsafe extern "C" fn(pathPtr: *mut Tcl_Obj, tval: *mut utimbuf) -> ::std::os::raw::c_int,
>;
pub type Tcl_FSNormalizePathProc = ::std::option::Option<
    unsafe extern "C" fn(
        interp: *mut Tcl_Interp,
        pathPtr: *mut Tcl_Obj,
        nextCheckpoint: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int,
>;
pub type Tcl_FSFileAttrsGetProc = ::std::option::Option<
    unsafe extern "C" fn(
        interp: *mut Tcl_Interp,
        index: ::std::os::raw::c_int,
        pathPtr: *mut Tcl_Obj,
        objPtrRef: *mut *mut Tcl_Obj,
    ) -> ::std::os::raw::c_int,
>;
pub type Tcl_FSFileAttrStringsProc = ::std::option::Option<
    unsafe extern "C" fn(
        pathPtr: *mut Tcl_Obj,
        objPtrRef: *mut *mut Tcl_Obj,
    ) -> *mut *const ::std::os::raw::c_char,
>;
pub type Tcl_FSFileAttrsSetProc = ::std::option::Option<
    unsafe extern "C" fn(
        interp: *mut Tcl_Interp,
        index: ::std::os::raw::c_int,
        pathPtr: *mut Tcl_Obj,
        objPtr: *mut Tcl_Obj,
    ) -> ::std::os::raw::c_int,
>;
pub type Tcl_FSLinkProc = ::std::option::Option<
    unsafe extern "C" fn(
        pathPtr: *mut Tcl_Obj,
        toPtr: *mut Tcl_Obj,
        linkType: ::std::os::raw::c_int,
    ) -> *mut Tcl_Obj,
>;
pub type Tcl_FSLoadFileProc = ::std::option::Option<
    unsafe extern "C" fn(
        interp: *mut Tcl_Interp,
        pathPtr: *mut Tcl_Obj,
        handlePtr: *mut Tcl_LoadHandle,
        unloadProcPtr: *mut Tcl_FSUnloadFileProc,
    ) -> ::std::os::raw::c_int,
>;
pub type Tcl_FSPathInFilesystemProc = ::std::option::Option<
    unsafe extern "C" fn(
        pathPtr: *mut Tcl_Obj,
        clientDataPtr: *mut ClientData,
    ) -> ::std::os::raw::c_int,
>;
pub type Tcl_FSFilesystemPathTypeProc =
    ::std::option::Option<unsafe extern "C" fn(pathPtr: *mut Tcl_Obj) -> *mut Tcl_Obj>;
pub type Tcl_FSFilesystemSeparatorProc =
    ::std::option::Option<unsafe extern "C" fn(pathPtr: *mut Tcl_Obj) -> *mut Tcl_Obj>;
pub type Tcl_FSFreeInternalRepProc =
    ::std::option::Option<unsafe extern "C" fn(clientData: ClientData)>;
pub type Tcl_FSDupInternalRepProc =
    ::std::option::Option<unsafe extern "C" fn(clientData: ClientData) -> ClientData>;
pub type Tcl_FSInternalToNormalizedProc =
    ::std::option::Option<unsafe extern "C" fn(clientData: ClientData) -> *mut Tcl_Obj>;
pub type Tcl_FSCreateInternalRepProc =
    ::std::option::Option<unsafe extern "C" fn(pathPtr: *mut Tcl_Obj) -> ClientData>;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Tcl_FSVersion_ {
    _unused: [u8; 0],
}
pub type Tcl_FSVersion = *mut Tcl_FSVersion_;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Tcl_Filesystem {
    pub typeName: *const ::std::os::raw::c_char,
    pub structureLength: ::std::os::raw::c_int,
    pub version: Tcl_FSVersion,
    pub pathInFilesystemProc: Tcl_FSPathInFilesystemProc,
    pub dupInternalRepProc: Tcl_FSDupInternalRepProc,
    pub freeInternalRepProc: Tcl_FSFreeInternalRepProc,
    pub internalToNormalizedProc: Tcl_FSInternalToNormalizedProc,
    pub createInternalRepProc: Tcl_FSCreateInternalRepProc,
    pub normalizePathProc: Tcl_FSNormalizePathProc,
    pub filesystemPathTypeProc: Tcl_FSFilesystemPathTypeProc,
    pub filesystemSeparatorProc: Tcl_FSFilesystemSeparatorProc,
    pub statProc: Tcl_FSStatProc,
    pub accessProc: Tcl_FSAccessProc,
    pub openFileChannelProc: Tcl_FSOpenFileChannelProc,
    pub matchInDirectoryProc: Tcl_FSMatchInDirectoryProc,
    pub utimeProc: Tcl_FSUtimeProc,
    pub linkProc: Tcl_FSLinkProc,
    pub listVolumesProc: Tcl_FSListVolumesProc,
    pub fileAttrStringsProc: Tcl_FSFileAttrStringsProc,
    pub fileAttrsGetProc: Tcl_FSFileAttrsGetProc,
    pub fileAttrsSetProc: Tcl_FSFileAttrsSetProc,
    pub createDirectoryProc: Tcl_FSCreateDirectoryProc,
    pub removeDirectoryProc: Tcl_FSRemoveDirectoryProc,
    pub deleteFileProc: Tcl_FSDeleteFileProc,
    pub copyFileProc: Tcl_FSCopyFileProc,
    pub renameFileProc: Tcl_FSRenameFileProc,
    pub copyDirectoryProc: Tcl_FSCopyDirectoryProc,
    pub lstatProc: Tcl_FSLstatProc,
    pub loadFileProc: Tcl_FSLoadFileProc,
    pub getCwdProc: Tcl_FSGetCwdProc,
    pub chdirProc: Tcl_FSChdirProc,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Tcl_NotifierProcs {
    pub setTimerProc: Tcl_SetTimerProc,
    pub waitForEventProc: Tcl_WaitForEventProc,
    pub createFileHandlerProc: Tcl_CreateFileHandlerProc,
    pub deleteFileHandlerProc: Tcl_DeleteFileHandlerProc,
    pub initNotifierProc: Tcl_InitNotifierProc,
    pub finalizeNotifierProc: Tcl_FinalizeNotifierProc,
    pub alertNotifierProc: Tcl_AlertNotifierProc,
    pub serviceModeHookProc: Tcl_ServiceModeHookProc,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Tcl_EncodingType {
    pub encodingName: *const ::std::os::raw::c_char,
    pub toUtfProc: Tcl_EncodingConvertProc,
    pub fromUtfProc: Tcl_EncodingConvertProc,
    pub freeProc: Tcl_EncodingFreeProc,
    pub clientData: ClientData,
    pub nullSize: ::std::os::raw::c_int,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Tcl_Token {
    pub type_: ::std::os::raw::c_int,
    pub start: *const ::std::os::raw::c_char,
    pub size: ::std::os::raw::c_int,
    pub numComponents: ::std::os::raw::c_int,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Tcl_Parse {
    pub commentStart: *const ::std::os::raw::c_char,
    pub commentSize: ::std::os::raw::c_int,
    pub commandStart: *const ::std::os::raw::c_char,
    pub commandSize: ::std::os::raw::c_int,
    pub numWords: ::std::os::raw::c_int,
    pub tokenPtr: *mut Tcl_Token,
    pub numTokens: ::std::os::raw::c_int,
    pub tokensAvailable: ::std::os::raw::c_int,
    pub errorType: ::std::os::raw::c_int,
    pub string: *const ::std::os::raw::c_char,
    pub end: *const ::std::os::raw::c_char,
    pub interp: *mut Tcl_Interp,
    pub term: *const ::std::os::raw::c_char,
    pub incomplete: ::std::os::raw::c_int,
    pub staticTokens: [Tcl_Token; 20usize],
}
pub type Tcl_UniChar = ::std::os::raw::c_ushort;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Tcl_Config {
    pub key: *const ::std::os::raw::c_char,
    pub value: *const ::std::os::raw::c_char,
}
pub type Tcl_LimitHandlerProc =
    ::std::option::Option<unsafe extern "C" fn(clientData: ClientData, interp: *mut Tcl_Interp)>;
pub type Tcl_LimitHandlerDeleteProc =
    ::std::option::Option<unsafe extern "C" fn(clientData: ClientData)>;
pub type mp_digit = ::std::os::raw::c_uint;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct TclStubHooks {
    pub tclPlatStubs: *mut TclPlatStubs,
    pub tclIntStubs: *mut TclIntStubs,
    pub tclIntPlatStubs: *mut TclIntPlatStubs,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct TclStubs {
    pub magic: ::std::os::raw::c_int,
    pub hooks: *mut TclStubHooks,
    pub tcl_PkgProvideEx: ::std::option::Option<
        unsafe extern "C" fn(
            interp: *mut Tcl_Interp,
            name: *const ::std::os::raw::c_char,
            version: *const ::std::os::raw::c_char,
            clientData: ClientData,
        ) -> ::std::os::raw::c_int,
    >,
    pub tcl_PkgRequireEx: ::std::option::Option<
        unsafe extern "C" fn(
            interp: *mut Tcl_Interp,
            name: *const ::std::os::raw::c_char,
            version: *const ::std::os::raw::c_char,
            exact: ::std::os::raw::c_int,
            clientDataPtr: *mut ClientData,
        ) -> *const ::std::os::raw::c_char,
    >,
    pub tcl_Panic:
        ::std::option::Option<unsafe extern "C" fn(format: *const ::std::os::raw::c_char, ...)>,
    pub tcl_Alloc: ::std::option::Option<
        unsafe extern "C" fn(size: ::std::os::raw::c_uint) -> *mut ::std::os::raw::c_char,
    >,
    pub tcl_Free: ::std::option::Option<unsafe extern "C" fn(ptr: *mut ::std::os::raw::c_char)>,
    pub tcl_Realloc: ::std::option::Option<
        unsafe extern "C" fn(
            ptr: *mut ::std::os::raw::c_char,
            size: ::std::os::raw::c_uint,
        ) -> *mut ::std::os::raw::c_char,
    >,
    pub tcl_DbCkalloc: ::std::option::Option<
        unsafe extern "C" fn(
            size: ::std::os::raw::c_uint,
            file: *const ::std::os::raw::c_char,
            line: ::std::os::raw::c_int,
        ) -> *mut ::std::os::raw::c_char,
    >,
    pub tcl_DbCkfree: ::std::option::Option<
        unsafe extern "C" fn(
            ptr: *mut ::std::os::raw::c_char,
            file: *const ::std::os::raw::c_char,
            line: ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int,
    >,
    pub tcl_DbCkrealloc: ::std::option::Option<
        unsafe extern "C" fn(
            ptr: *mut ::std::os::raw::c_char,
            size: ::std::os::raw::c_uint,
            file: *const ::std::os::raw::c_char,
            line: ::std::os::raw::c_int,
        ) -> *mut ::std::os::raw::c_char,
    >,
    pub tcl_CreateFileHandler: ::std::option::Option<
        unsafe extern "C" fn(
            fd: ::std::os::raw::c_int,
            mask: ::std::os::raw::c_int,
            proc_: Tcl_FileProc,
            clientData: ClientData,
        ),
    >,
    pub tcl_DeleteFileHandler:
        ::std::option::Option<unsafe extern "C" fn(fd: ::std::os::raw::c_int)>,
    pub tcl_SetTimer: ::std::option::Option<unsafe extern "C" fn(timePtr: *mut Tcl_Time)>,
    pub tcl_Sleep: ::std::option::Option<unsafe extern "C" fn(ms: ::std::os::raw::c_int)>,
    pub tcl_WaitForEvent: ::std::option::Option<
        unsafe extern "C" fn(timePtr: *mut Tcl_Time) -> ::std::os::raw::c_int,
    >,
    pub tcl_AppendAllObjTypes: ::std::option::Option<
        unsafe extern "C" fn(
            interp: *mut Tcl_Interp,
            objPtr: *mut Tcl_Obj,
        ) -> ::std::os::raw::c_int,
    >,
    pub tcl_AppendStringsToObj:
        ::std::option::Option<unsafe extern "C" fn(objPtr: *mut Tcl_Obj, ...)>,
    pub tcl_AppendToObj: ::std::option::Option<
        unsafe extern "C" fn(
            objPtr: *mut Tcl_Obj,
            bytes: *const ::std::os::raw::c_char,
            length: ::std::os::raw::c_int,
        ),
    >,
    pub tcl_ConcatObj: ::std::option::Option<
        unsafe extern "C" fn(
            objc: ::std::os::raw::c_int,
            objv: *const *mut Tcl_Obj,
        ) -> *mut Tcl_Obj,
    >,
    pub tcl_ConvertToType: ::std::option::Option<
        unsafe extern "C" fn(
            interp: *mut Tcl_Interp,
            objPtr: *mut Tcl_Obj,
            typePtr: *mut Tcl_ObjType,
        ) -> ::std::os::raw::c_int,
    >,
    pub tcl_DbDecrRefCount: ::std::option::Option<
        unsafe extern "C" fn(
            objPtr: *mut Tcl_Obj,
            file: *const ::std::os::raw::c_char,
            line: ::std::os::raw::c_int,
        ),
    >,
    pub tcl_DbIncrRefCount: ::std::option::Option<
        unsafe extern "C" fn(
            objPtr: *mut Tcl_Obj,
            file: *const ::std::os::raw::c_char,
            line: ::std::os::raw::c_int,
        ),
    >,
    pub tcl_DbIsShared: ::std::option::Option<
        unsafe extern "C" fn(
            objPtr: *mut Tcl_Obj,
            file: *const ::std::os::raw::c_char,
            line: ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int,
    >,
    pub tcl_DbNewBooleanObj: ::std::option::Option<
        unsafe extern "C" fn(
            boolValue: ::std::os::raw::c_int,
            file: *const ::std::os::raw::c_char,
            line: ::std::os::raw::c_int,
        ) -> *mut Tcl_Obj,
    >,
    pub tcl_DbNewByteArrayObj: ::std::option::Option<
        unsafe extern "C" fn(
            bytes: *const ::std::os::raw::c_uchar,
            length: ::std::os::raw::c_int,
            file: *const ::std::os::raw::c_char,
            line: ::std::os::raw::c_int,
        ) -> *mut Tcl_Obj,
    >,
    pub tcl_DbNewDoubleObj: ::std::option::Option<
        unsafe extern "C" fn(
            doubleValue: f64,
            file: *const ::std::os::raw::c_char,
            line: ::std::os::raw::c_int,
        ) -> *mut Tcl_Obj,
    >,
    pub tcl_DbNewListObj: ::std::option::Option<
        unsafe extern "C" fn(
            objc: ::std::os::raw::c_int,
            objv: *const *mut Tcl_Obj,
            file: *const ::std::os::raw::c_char,
            line: ::std::os::raw::c_int,
        ) -> *mut Tcl_Obj,
    >,
    pub tcl_DbNewLongObj: ::std::option::Option<
        unsafe extern "C" fn(
            longValue: ::std::os::raw::c_long,
            file: *const ::std::os::raw::c_char,
            line: ::std::os::raw::c_int,
        ) -> *mut Tcl_Obj,
    >,
    pub tcl_DbNewObj: ::std::option::Option<
        unsafe extern "C" fn(
            file: *const ::std::os::raw::c_char,
            line: ::std::os::raw::c_int,
        ) -> *mut Tcl_Obj,
    >,
    pub tcl_DbNewStringObj: ::std::option::Option<
        unsafe extern "C" fn(
            bytes: *const ::std::os::raw::c_char,
            length: ::std::os::raw::c_int,
            file: *const ::std::os::raw::c_char,
            line: ::std::os::raw::c_int,
        ) -> *mut Tcl_Obj,
    >,
    pub tcl_DuplicateObj:
        ::std::option::Option<unsafe extern "C" fn(objPtr: *mut Tcl_Obj) -> *mut Tcl_Obj>,
    pub tclFreeObj: ::std::option::Option<unsafe extern "C" fn(objPtr: *mut Tcl_Obj)>,
    pub tcl_GetBoolean: ::std::option::Option<
        unsafe extern "C" fn(
            interp: *mut Tcl_Interp,
            src: *const ::std::os::raw::c_char,
            boolPtr: *mut ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int,
    >,
    pub tcl_GetBooleanFromObj: ::std::option::Option<
        unsafe extern "C" fn(
            interp: *mut Tcl_Interp,
            objPtr: *mut Tcl_Obj,
            boolPtr: *mut ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int,
    >,
    pub tcl_GetByteArrayFromObj: ::std::option::Option<
        unsafe extern "C" fn(
            objPtr: *mut Tcl_Obj,
            lengthPtr: *mut ::std::os::raw::c_int,
        ) -> *mut ::std::os::raw::c_uchar,
    >,
    pub tcl_GetDouble: ::std::option::Option<
        unsafe extern "C" fn(
            interp: *mut Tcl_Interp,
            src: *const ::std::os::raw::c_char,
            doublePtr: *mut f64,
        ) -> ::std::os::raw::c_int,
    >,
    pub tcl_GetDoubleFromObj: ::std::option::Option<
        unsafe extern "C" fn(
            interp: *mut Tcl_Interp,
            objPtr: *mut Tcl_Obj,
            doublePtr: *mut f64,
        ) -> ::std::os::raw::c_int,
    >,
    pub tcl_GetIndexFromObj: ::std::option::Option<
        unsafe extern "C" fn(
            interp: *mut Tcl_Interp,
            objPtr: *mut Tcl_Obj,
            tablePtr: *mut *const ::std::os::raw::c_char,
            msg: *const ::std::os::raw::c_char,
            flags: ::std::os::raw::c_int,
            indexPtr: *mut ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int,
    >,
    pub tcl_GetInt: ::std::option::Option<
        unsafe extern "C" fn(
            interp: *mut Tcl_Interp,
            src: *const ::std::os::raw::c_char,
            intPtr: *mut ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int,
    >,
    pub tcl_GetIntFromObj: ::std::option::Option<
        unsafe extern "C" fn(
            interp: *mut Tcl_Interp,
            objPtr: *mut Tcl_Obj,
            intPtr: *mut ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int,
    >,
    pub tcl_GetLongFromObj: ::std::option::Option<
        unsafe extern "C" fn(
            interp: *mut Tcl_Interp,
            objPtr: *mut Tcl_Obj,
            longPtr: *mut ::std::os::raw::c_long,
        ) -> ::std::os::raw::c_int,
    >,
    pub tcl_GetObjType: ::std::option::Option<
        unsafe extern "C" fn(typeName: *const ::std::os::raw::c_char) -> *mut Tcl_ObjType,
    >,
    pub tcl_GetStringFromObj: ::std::option::Option<
        unsafe extern "C" fn(
            objPtr: *mut Tcl_Obj,
            lengthPtr: *mut ::std::os::raw::c_int,
        ) -> *mut ::std::os::raw::c_char,
    >,
    pub tcl_InvalidateStringRep: ::std::option::Option<unsafe extern "C" fn(objPtr: *mut Tcl_Obj)>,
    pub tcl_ListObjAppendList: ::std::option::Option<
        unsafe extern "C" fn(
            interp: *mut Tcl_Interp,
            listPtr: *mut Tcl_Obj,
            elemListPtr: *mut Tcl_Obj,
        ) -> ::std::os::raw::c_int,
    >,
    pub tcl_ListObjAppendElement: ::std::option::Option<
        unsafe extern "C" fn(
            interp: *mut Tcl_Interp,
            listPtr: *mut Tcl_Obj,
            objPtr: *mut Tcl_Obj,
        ) -> ::std::os::raw::c_int,
    >,
    pub tcl_ListObjGetElements: ::std::option::Option<
        unsafe extern "C" fn(
            interp: *mut Tcl_Interp,
            listPtr: *mut Tcl_Obj,
            objcPtr: *mut ::std::os::raw::c_int,
            objvPtr: *mut *mut *mut Tcl_Obj,
        ) -> ::std::os::raw::c_int,
    >,
    pub tcl_ListObjIndex: ::std::option::Option<
        unsafe extern "C" fn(
            interp: *mut Tcl_Interp,
            listPtr: *mut Tcl_Obj,
            index: ::std::os::raw::c_int,
            objPtrPtr: *mut *mut Tcl_Obj,
        ) -> ::std::os::raw::c_int,
    >,
    pub tcl_ListObjLength: ::std::option::Option<
        unsafe extern "C" fn(
            interp: *mut Tcl_Interp,
            listPtr: *mut Tcl_Obj,
            lengthPtr: *mut ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int,
    >,
    pub tcl_ListObjReplace: ::std::option::Option<
        unsafe extern "C" fn(
            interp: *mut Tcl_Interp,
            listPtr: *mut Tcl_Obj,
            first: ::std::os::raw::c_int,
            count: ::std::os::raw::c_int,
            objc: ::std::os::raw::c_int,
            objv: *const *mut Tcl_Obj,
        ) -> ::std::os::raw::c_int,
    >,
    pub tcl_NewBooleanObj: ::std::option::Option<
        unsafe extern "C" fn(boolValue: ::std::os::raw::c_int) -> *mut Tcl_Obj,
    >,
    pub tcl_NewByteArrayObj: ::std::option::Option<
        unsafe extern "C" fn(
            bytes: *const ::std::os::raw::c_uchar,
            length: ::std::os::raw::c_int,
        ) -> *mut Tcl_Obj,
    >,
    pub tcl_NewDoubleObj:
        ::std::option::Option<unsafe extern "C" fn(doubleValue: f64) -> *mut Tcl_Obj>,
    pub tcl_NewIntObj: ::std::option::Option<
        unsafe extern "C" fn(intValue: ::std::os::raw::c_int) -> *mut Tcl_Obj,
    >,
    pub tcl_NewListObj: ::std::option::Option<
        unsafe extern "C" fn(
            objc: ::std::os::raw::c_int,
            objv: *const *mut Tcl_Obj,
        ) -> *mut Tcl_Obj,
    >,
    pub tcl_NewLongObj: ::std::option::Option<
        unsafe extern "C" fn(longValue: ::std::os::raw::c_long) -> *mut Tcl_Obj,
    >,
    pub tcl_NewObj: ::std::option::Option<unsafe extern "C" fn() -> *mut Tcl_Obj>,
    pub tcl_NewStringObj: ::std::option::Option<
        unsafe extern "C" fn(
            bytes: *const ::std::os::raw::c_char,
            length: ::std::os::raw::c_int,
        ) -> *mut Tcl_Obj,
    >,
    pub tcl_SetBooleanObj: ::std::option::Option<
        unsafe extern "C" fn(objPtr: *mut Tcl_Obj, boolValue: ::std::os::raw::c_int),
    >,
    pub tcl_SetByteArrayLength: ::std::option::Option<
        unsafe extern "C" fn(
            objPtr: *mut Tcl_Obj,
            length: ::std::os::raw::c_int,
        ) -> *mut ::std::os::raw::c_uchar,
    >,
    pub tcl_SetByteArrayObj: ::std::option::Option<
        unsafe extern "C" fn(
            objPtr: *mut Tcl_Obj,
            bytes: *const ::std::os::raw::c_uchar,
            length: ::std::os::raw::c_int,
        ),
    >,
    pub tcl_SetDoubleObj:
        ::std::option::Option<unsafe extern "C" fn(objPtr: *mut Tcl_Obj, doubleValue: f64)>,
    pub tcl_SetIntObj: ::std::option::Option<
        unsafe extern "C" fn(objPtr: *mut Tcl_Obj, intValue: ::std::os::raw::c_int),
    >,
    pub tcl_SetListObj: ::std::option::Option<
        unsafe extern "C" fn(
            objPtr: *mut Tcl_Obj,
            objc: ::std::os::raw::c_int,
            objv: *const *mut Tcl_Obj,
        ),
    >,
    pub tcl_SetLongObj: ::std::option::Option<
        unsafe extern "C" fn(objPtr: *mut Tcl_Obj, longValue: ::std::os::raw::c_long),
    >,
    pub tcl_SetObjLength: ::std::option::Option<
        unsafe extern "C" fn(objPtr: *mut Tcl_Obj, length: ::std::os::raw::c_int),
    >,
    pub tcl_SetStringObj: ::std::option::Option<
        unsafe extern "C" fn(
            objPtr: *mut Tcl_Obj,
            bytes: *const ::std::os::raw::c_char,
            length: ::std::os::raw::c_int,
        ),
    >,
    pub tcl_AddErrorInfo: ::std::option::Option<
        unsafe extern "C" fn(interp: *mut Tcl_Interp, message: *const ::std::os::raw::c_char),
    >,
    pub tcl_AddObjErrorInfo: ::std::option::Option<
        unsafe extern "C" fn(
            interp: *mut Tcl_Interp,
            message: *const ::std::os::raw::c_char,
            length: ::std::os::raw::c_int,
        ),
    >,
    pub tcl_AllowExceptions: ::std::option::Option<unsafe extern "C" fn(interp: *mut Tcl_Interp)>,
    pub tcl_AppendElement: ::std::option::Option<
        unsafe extern "C" fn(interp: *mut Tcl_Interp, element: *const ::std::os::raw::c_char),
    >,
    pub tcl_AppendResult: ::std::option::Option<unsafe extern "C" fn(interp: *mut Tcl_Interp, ...)>,
    pub tcl_AsyncCreate: ::std::option::Option<
        unsafe extern "C" fn(proc_: Tcl_AsyncProc, clientData: ClientData) -> Tcl_AsyncHandler,
    >,
    pub tcl_AsyncDelete: ::std::option::Option<unsafe extern "C" fn(async_: Tcl_AsyncHandler)>,
    pub tcl_AsyncInvoke: ::std::option::Option<
        unsafe extern "C" fn(
            interp: *mut Tcl_Interp,
            code: ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int,
    >,
    pub tcl_AsyncMark: ::std::option::Option<unsafe extern "C" fn(async_: Tcl_AsyncHandler)>,
    pub tcl_AsyncReady: ::std::option::Option<unsafe extern "C" fn() -> ::std::os::raw::c_int>,
    pub tcl_BackgroundError: ::std::option::Option<unsafe extern "C" fn(interp: *mut Tcl_Interp)>,
    pub tcl_Backslash: ::std::option::Option<
        unsafe extern "C" fn(
            src: *const ::std::os::raw::c_char,
            readPtr: *mut ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_char,
    >,
    pub tcl_BadChannelOption: ::std::option::Option<
        unsafe extern "C" fn(
            interp: *mut Tcl_Interp,
            optionName: *const ::std::os::raw::c_char,
            optionList: *const ::std::os::raw::c_char,
        ) -> ::std::os::raw::c_int,
    >,
    pub tcl_CallWhenDeleted: ::std::option::Option<
        unsafe extern "C" fn(
            interp: *mut Tcl_Interp,
            proc_: Tcl_InterpDeleteProc,
            clientData: ClientData,
        ),
    >,
    pub tcl_CancelIdleCall:
        ::std::option::Option<unsafe extern "C" fn(idleProc: Tcl_IdleProc, clientData: ClientData)>,
    pub tcl_Close: ::std::option::Option<
        unsafe extern "C" fn(interp: *mut Tcl_Interp, chan: Tcl_Channel) -> ::std::os::raw::c_int,
    >,
    pub tcl_CommandComplete: ::std::option::Option<
        unsafe extern "C" fn(cmd: *const ::std::os::raw::c_char) -> ::std::os::raw::c_int,
    >,
    pub tcl_Concat: ::std::option::Option<
        unsafe extern "C" fn(
            argc: ::std::os::raw::c_int,
            argv: *const *const ::std::os::raw::c_char,
        ) -> *mut ::std::os::raw::c_char,
    >,
    pub tcl_ConvertElement: ::std::option::Option<
        unsafe extern "C" fn(
            src: *const ::std::os::raw::c_char,
            dst: *mut ::std::os::raw::c_char,
            flags: ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int,
    >,
    pub tcl_ConvertCountedElement: ::std::option::Option<
        unsafe extern "C" fn(
            src: *const ::std::os::raw::c_char,
            length: ::std::os::raw::c_int,
            dst: *mut ::std::os::raw::c_char,
            flags: ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int,
    >,
    pub tcl_CreateAlias: ::std::option::Option<
        unsafe extern "C" fn(
            slave: *mut Tcl_Interp,
            slaveCmd: *const ::std::os::raw::c_char,
            target: *mut Tcl_Interp,
            targetCmd: *const ::std::os::raw::c_char,
            argc: ::std::os::raw::c_int,
            argv: *const *const ::std::os::raw::c_char,
        ) -> ::std::os::raw::c_int,
    >,
    pub tcl_CreateAliasObj: ::std::option::Option<
        unsafe extern "C" fn(
            slave: *mut Tcl_Interp,
            slaveCmd: *const ::std::os::raw::c_char,
            target: *mut Tcl_Interp,
            targetCmd: *const ::std::os::raw::c_char,
            objc: ::std::os::raw::c_int,
            objv: *const *mut Tcl_Obj,
        ) -> ::std::os::raw::c_int,
    >,
    pub tcl_CreateChannel: ::std::option::Option<
        unsafe extern "C" fn(
            typePtr: *mut Tcl_ChannelType,
            chanName: *const ::std::os::raw::c_char,
            instanceData: ClientData,
            mask: ::std::os::raw::c_int,
        ) -> Tcl_Channel,
    >,
    pub tcl_CreateChannelHandler: ::std::option::Option<
        unsafe extern "C" fn(
            chan: Tcl_Channel,
            mask: ::std::os::raw::c_int,
            proc_: Tcl_ChannelProc,
            clientData: ClientData,
        ),
    >,
    pub tcl_CreateCloseHandler: ::std::option::Option<
        unsafe extern "C" fn(chan: Tcl_Channel, proc_: Tcl_CloseProc, clientData: ClientData),
    >,
    pub tcl_CreateCommand: ::std::option::Option<
        unsafe extern "C" fn(
            interp: *mut Tcl_Interp,
            cmdName: *const ::std::os::raw::c_char,
            proc_: Tcl_CmdProc,
            clientData: ClientData,
            deleteProc: Tcl_CmdDeleteProc,
        ) -> Tcl_Command,
    >,
    pub tcl_CreateEventSource: ::std::option::Option<
        unsafe extern "C" fn(
            setupProc: Tcl_EventSetupProc,
            checkProc: Tcl_EventCheckProc,
            clientData: ClientData,
        ),
    >,
    pub tcl_CreateExitHandler:
        ::std::option::Option<unsafe extern "C" fn(proc_: Tcl_ExitProc, clientData: ClientData)>,
    pub tcl_CreateInterp: ::std::option::Option<unsafe extern "C" fn() -> *mut Tcl_Interp>,
    pub tcl_CreateMathFunc: ::std::option::Option<
        unsafe extern "C" fn(
            interp: *mut Tcl_Interp,
            name: *const ::std::os::raw::c_char,
            numArgs: ::std::os::raw::c_int,
            argTypes: *mut Tcl_ValueType,
            proc_: Tcl_MathProc,
            clientData: ClientData,
        ),
    >,
    pub tcl_CreateObjCommand: ::std::option::Option<
        unsafe extern "C" fn(
            interp: *mut Tcl_Interp,
            cmdName: *const ::std::os::raw::c_char,
            proc_: Tcl_ObjCmdProc,
            clientData: ClientData,
            deleteProc: Tcl_CmdDeleteProc,
        ) -> Tcl_Command,
    >,
    pub tcl_CreateSlave: ::std::option::Option<
        unsafe extern "C" fn(
            interp: *mut Tcl_Interp,
            slaveName: *const ::std::os::raw::c_char,
            isSafe: ::std::os::raw::c_int,
        ) -> *mut Tcl_Interp,
    >,
    pub tcl_CreateTimerHandler: ::std::option::Option<
        unsafe extern "C" fn(
            milliseconds: ::std::os::raw::c_int,
            proc_: Tcl_TimerProc,
            clientData: ClientData,
        ) -> Tcl_TimerToken,
    >,
    pub tcl_CreateTrace: ::std::option::Option<
        unsafe extern "C" fn(
            interp: *mut Tcl_Interp,
            level: ::std::os::raw::c_int,
            proc_: Tcl_CmdTraceProc,
            clientData: ClientData,
        ) -> Tcl_Trace,
    >,
    pub tcl_DeleteAssocData: ::std::option::Option<
        unsafe extern "C" fn(interp: *mut Tcl_Interp, name: *const ::std::os::raw::c_char),
    >,
    pub tcl_DeleteChannelHandler: ::std::option::Option<
        unsafe extern "C" fn(chan: Tcl_Channel, proc_: Tcl_ChannelProc, clientData: ClientData),
    >,
    pub tcl_DeleteCloseHandler: ::std::option::Option<
        unsafe extern "C" fn(chan: Tcl_Channel, proc_: Tcl_CloseProc, clientData: ClientData),
    >,
    pub tcl_DeleteCommand: ::std::option::Option<
        unsafe extern "C" fn(
            interp: *mut Tcl_Interp,
            cmdName: *const ::std::os::raw::c_char,
        ) -> ::std::os::raw::c_int,
    >,
    pub tcl_DeleteCommandFromToken: ::std::option::Option<
        unsafe extern "C" fn(
            interp: *mut Tcl_Interp,
            command: Tcl_Command,
        ) -> ::std::os::raw::c_int,
    >,
    pub tcl_DeleteEvents: ::std::option::Option<
        unsafe extern "C" fn(proc_: Tcl_EventDeleteProc, clientData: ClientData),
    >,
    pub tcl_DeleteEventSource: ::std::option::Option<
        unsafe extern "C" fn(
            setupProc: Tcl_EventSetupProc,
            checkProc: Tcl_EventCheckProc,
            clientData: ClientData,
        ),
    >,
    pub tcl_DeleteExitHandler:
        ::std::option::Option<unsafe extern "C" fn(proc_: Tcl_ExitProc, clientData: ClientData)>,
    pub tcl_DeleteHashEntry:
        ::std::option::Option<unsafe extern "C" fn(entryPtr: *mut Tcl_HashEntry)>,
    pub tcl_DeleteHashTable:
        ::std::option::Option<unsafe extern "C" fn(tablePtr: *mut Tcl_HashTable)>,
    pub tcl_DeleteInterp: ::std::option::Option<unsafe extern "C" fn(interp: *mut Tcl_Interp)>,
    pub tcl_DetachPids: ::std::option::Option<
        unsafe extern "C" fn(numPids: ::std::os::raw::c_int, pidPtr: *mut Tcl_Pid),
    >,
    pub tcl_DeleteTimerHandler: ::std::option::Option<unsafe extern "C" fn(token: Tcl_TimerToken)>,
    pub tcl_DeleteTrace:
        ::std::option::Option<unsafe extern "C" fn(interp: *mut Tcl_Interp, trace: Tcl_Trace)>,
    pub tcl_DontCallWhenDeleted: ::std::option::Option<
        unsafe extern "C" fn(
            interp: *mut Tcl_Interp,
            proc_: Tcl_InterpDeleteProc,
            clientData: ClientData,
        ),
    >,
    pub tcl_DoOneEvent: ::std::option::Option<
        unsafe extern "C" fn(flags: ::std::os::raw::c_int) -> ::std::os::raw::c_int,
    >,
    pub tcl_DoWhenIdle:
        ::std::option::Option<unsafe extern "C" fn(proc_: Tcl_IdleProc, clientData: ClientData)>,
    pub tcl_DStringAppend: ::std::option::Option<
        unsafe extern "C" fn(
            dsPtr: *mut Tcl_DString,
            bytes: *const ::std::os::raw::c_char,
            length: ::std::os::raw::c_int,
        ) -> *mut ::std::os::raw::c_char,
    >,
    pub tcl_DStringAppendElement: ::std::option::Option<
        unsafe extern "C" fn(
            dsPtr: *mut Tcl_DString,
            element: *const ::std::os::raw::c_char,
        ) -> *mut ::std::os::raw::c_char,
    >,
    pub tcl_DStringEndSublist: ::std::option::Option<unsafe extern "C" fn(dsPtr: *mut Tcl_DString)>,
    pub tcl_DStringFree: ::std::option::Option<unsafe extern "C" fn(dsPtr: *mut Tcl_DString)>,
    pub tcl_DStringGetResult: ::std::option::Option<
        unsafe extern "C" fn(interp: *mut Tcl_Interp, dsPtr: *mut Tcl_DString),
    >,
    pub tcl_DStringInit: ::std::option::Option<unsafe extern "C" fn(dsPtr: *mut Tcl_DString)>,
    pub tcl_DStringResult: ::std::option::Option<
        unsafe extern "C" fn(interp: *mut Tcl_Interp, dsPtr: *mut Tcl_DString),
    >,
    pub tcl_DStringSetLength: ::std::option::Option<
        unsafe extern "C" fn(dsPtr: *mut Tcl_DString, length: ::std::os::raw::c_int),
    >,
    pub tcl_DStringStartSublist:
        ::std::option::Option<unsafe extern "C" fn(dsPtr: *mut Tcl_DString)>,
    pub tcl_Eof:
        ::std::option::Option<unsafe extern "C" fn(chan: Tcl_Channel) -> ::std::os::raw::c_int>,
    pub tcl_ErrnoId: ::std::option::Option<unsafe extern "C" fn() -> *const ::std::os::raw::c_char>,
    pub tcl_ErrnoMsg: ::std::option::Option<
        unsafe extern "C" fn(err: ::std::os::raw::c_int) -> *const ::std::os::raw::c_char,
    >,
    pub tcl_Eval: ::std::option::Option<
        unsafe extern "C" fn(
            interp: *mut Tcl_Interp,
            script: *const ::std::os::raw::c_char,
        ) -> ::std::os::raw::c_int,
    >,
    pub tcl_EvalFile: ::std::option::Option<
        unsafe extern "C" fn(
            interp: *mut Tcl_Interp,
            fileName: *const ::std::os::raw::c_char,
        ) -> ::std::os::raw::c_int,
    >,
    pub tcl_EvalObj: ::std::option::Option<
        unsafe extern "C" fn(
            interp: *mut Tcl_Interp,
            objPtr: *mut Tcl_Obj,
        ) -> ::std::os::raw::c_int,
    >,
    pub tcl_EventuallyFree:
        ::std::option::Option<unsafe extern "C" fn(clientData: ClientData, freeProc: Tcl_FreeProc)>,
    pub tcl_Exit: ::std::option::Option<unsafe extern "C" fn(status: ::std::os::raw::c_int)>,
    pub tcl_ExposeCommand: ::std::option::Option<
        unsafe extern "C" fn(
            interp: *mut Tcl_Interp,
            hiddenCmdToken: *const ::std::os::raw::c_char,
            cmdName: *const ::std::os::raw::c_char,
        ) -> ::std::os::raw::c_int,
    >,
    pub tcl_ExprBoolean: ::std::option::Option<
        unsafe extern "C" fn(
            interp: *mut Tcl_Interp,
            expr: *const ::std::os::raw::c_char,
            ptr: *mut ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int,
    >,
    pub tcl_ExprBooleanObj: ::std::option::Option<
        unsafe extern "C" fn(
            interp: *mut Tcl_Interp,
            objPtr: *mut Tcl_Obj,
            ptr: *mut ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int,
    >,
    pub tcl_ExprDouble: ::std::option::Option<
        unsafe extern "C" fn(
            interp: *mut Tcl_Interp,
            expr: *const ::std::os::raw::c_char,
            ptr: *mut f64,
        ) -> ::std::os::raw::c_int,
    >,
    pub tcl_ExprDoubleObj: ::std::option::Option<
        unsafe extern "C" fn(
            interp: *mut Tcl_Interp,
            objPtr: *mut Tcl_Obj,
            ptr: *mut f64,
        ) -> ::std::os::raw::c_int,
    >,
    pub tcl_ExprLong: ::std::option::Option<
        unsafe extern "C" fn(
            interp: *mut Tcl_Interp,
            expr: *const ::std::os::raw::c_char,
            ptr: *mut ::std::os::raw::c_long,
        ) -> ::std::os::raw::c_int,
    >,
    pub tcl_ExprLongObj: ::std::option::Option<
        unsafe extern "C" fn(
            interp: *mut Tcl_Interp,
            objPtr: *mut Tcl_Obj,
            ptr: *mut ::std::os::raw::c_long,
        ) -> ::std::os::raw::c_int,
    >,
    pub tcl_ExprObj: ::std::option::Option<
        unsafe extern "C" fn(
            interp: *mut Tcl_Interp,
            objPtr: *mut Tcl_Obj,
            resultPtrPtr: *mut *mut Tcl_Obj,
        ) -> ::std::os::raw::c_int,
    >,
    pub tcl_ExprString: ::std::option::Option<
        unsafe extern "C" fn(
            interp: *mut Tcl_Interp,
            expr: *const ::std::os::raw::c_char,
        ) -> ::std::os::raw::c_int,
    >,
    pub tcl_Finalize: ::std::option::Option<unsafe extern "C" fn()>,
    pub tcl_FindExecutable:
        ::std::option::Option<unsafe extern "C" fn(argv0: *const ::std::os::raw::c_char)>,
    pub tcl_FirstHashEntry: ::std::option::Option<
        unsafe extern "C" fn(
            tablePtr: *mut Tcl_HashTable,
            searchPtr: *mut Tcl_HashSearch,
        ) -> *mut Tcl_HashEntry,
    >,
    pub tcl_Flush:
        ::std::option::Option<unsafe extern "C" fn(chan: Tcl_Channel) -> ::std::os::raw::c_int>,
    pub tcl_FreeResult: ::std::option::Option<unsafe extern "C" fn(interp: *mut Tcl_Interp)>,
    pub tcl_GetAlias: ::std::option::Option<
        unsafe extern "C" fn(
            interp: *mut Tcl_Interp,
            slaveCmd: *const ::std::os::raw::c_char,
            targetInterpPtr: *mut *mut Tcl_Interp,
            targetCmdPtr: *mut *const ::std::os::raw::c_char,
            argcPtr: *mut ::std::os::raw::c_int,
            argvPtr: *mut *mut *const ::std::os::raw::c_char,
        ) -> ::std::os::raw::c_int,
    >,
    pub tcl_GetAliasObj: ::std::option::Option<
        unsafe extern "C" fn(
            interp: *mut Tcl_Interp,
            slaveCmd: *const ::std::os::raw::c_char,
            targetInterpPtr: *mut *mut Tcl_Interp,
            targetCmdPtr: *mut *const ::std::os::raw::c_char,
            objcPtr: *mut ::std::os::raw::c_int,
            objv: *mut *mut *mut Tcl_Obj,
        ) -> ::std::os::raw::c_int,
    >,
    pub tcl_GetAssocData: ::std::option::Option<
        unsafe extern "C" fn(
            interp: *mut Tcl_Interp,
            name: *const ::std::os::raw::c_char,
            procPtr: *mut Tcl_InterpDeleteProc,
        ) -> ClientData,
    >,
    pub tcl_GetChannel: ::std::option::Option<
        unsafe extern "C" fn(
            interp: *mut Tcl_Interp,
            chanName: *const ::std::os::raw::c_char,
            modePtr: *mut ::std::os::raw::c_int,
        ) -> Tcl_Channel,
    >,
    pub tcl_GetChannelBufferSize:
        ::std::option::Option<unsafe extern "C" fn(chan: Tcl_Channel) -> ::std::os::raw::c_int>,
    pub tcl_GetChannelHandle: ::std::option::Option<
        unsafe extern "C" fn(
            chan: Tcl_Channel,
            direction: ::std::os::raw::c_int,
            handlePtr: *mut ClientData,
        ) -> ::std::os::raw::c_int,
    >,
    pub tcl_GetChannelInstanceData:
        ::std::option::Option<unsafe extern "C" fn(chan: Tcl_Channel) -> ClientData>,
    pub tcl_GetChannelMode:
        ::std::option::Option<unsafe extern "C" fn(chan: Tcl_Channel) -> ::std::os::raw::c_int>,
    pub tcl_GetChannelName: ::std::option::Option<
        unsafe extern "C" fn(chan: Tcl_Channel) -> *const ::std::os::raw::c_char,
    >,
    pub tcl_GetChannelOption: ::std::option::Option<
        unsafe extern "C" fn(
            interp: *mut Tcl_Interp,
            chan: Tcl_Channel,
            optionName: *const ::std::os::raw::c_char,
            dsPtr: *mut Tcl_DString,
        ) -> ::std::os::raw::c_int,
    >,
    pub tcl_GetChannelType:
        ::std::option::Option<unsafe extern "C" fn(chan: Tcl_Channel) -> *mut Tcl_ChannelType>,
    pub tcl_GetCommandInfo: ::std::option::Option<
        unsafe extern "C" fn(
            interp: *mut Tcl_Interp,
            cmdName: *const ::std::os::raw::c_char,
            infoPtr: *mut Tcl_CmdInfo,
        ) -> ::std::os::raw::c_int,
    >,
    pub tcl_GetCommandName: ::std::option::Option<
        unsafe extern "C" fn(
            interp: *mut Tcl_Interp,
            command: Tcl_Command,
        ) -> *const ::std::os::raw::c_char,
    >,
    pub tcl_GetErrno: ::std::option::Option<unsafe extern "C" fn() -> ::std::os::raw::c_int>,
    pub tcl_GetHostName:
        ::std::option::Option<unsafe extern "C" fn() -> *const ::std::os::raw::c_char>,
    pub tcl_GetInterpPath: ::std::option::Option<
        unsafe extern "C" fn(
            askInterp: *mut Tcl_Interp,
            slaveInterp: *mut Tcl_Interp,
        ) -> ::std::os::raw::c_int,
    >,
    pub tcl_GetMaster:
        ::std::option::Option<unsafe extern "C" fn(interp: *mut Tcl_Interp) -> *mut Tcl_Interp>,
    pub tcl_GetNameOfExecutable:
        ::std::option::Option<unsafe extern "C" fn() -> *const ::std::os::raw::c_char>,
    pub tcl_GetObjResult:
        ::std::option::Option<unsafe extern "C" fn(interp: *mut Tcl_Interp) -> *mut Tcl_Obj>,
    pub tcl_GetOpenFile: ::std::option::Option<
        unsafe extern "C" fn(
            interp: *mut Tcl_Interp,
            chanID: *const ::std::os::raw::c_char,
            forWriting: ::std::os::raw::c_int,
            checkUsage: ::std::os::raw::c_int,
            filePtr: *mut ClientData,
        ) -> ::std::os::raw::c_int,
    >,
    pub tcl_GetPathType: ::std::option::Option<
        unsafe extern "C" fn(path: *const ::std::os::raw::c_char) -> Tcl_PathType,
    >,
    pub tcl_Gets: ::std::option::Option<
        unsafe extern "C" fn(chan: Tcl_Channel, dsPtr: *mut Tcl_DString) -> ::std::os::raw::c_int,
    >,
    pub tcl_GetsObj: ::std::option::Option<
        unsafe extern "C" fn(chan: Tcl_Channel, objPtr: *mut Tcl_Obj) -> ::std::os::raw::c_int,
    >,
    pub tcl_GetServiceMode: ::std::option::Option<unsafe extern "C" fn() -> ::std::os::raw::c_int>,
    pub tcl_GetSlave: ::std::option::Option<
        unsafe extern "C" fn(
            interp: *mut Tcl_Interp,
            slaveName: *const ::std::os::raw::c_char,
        ) -> *mut Tcl_Interp,
    >,
    pub tcl_GetStdChannel:
        ::std::option::Option<unsafe extern "C" fn(type_: ::std::os::raw::c_int) -> Tcl_Channel>,
    pub tcl_GetStringResult: ::std::option::Option<
        unsafe extern "C" fn(interp: *mut Tcl_Interp) -> *const ::std::os::raw::c_char,
    >,
    pub tcl_GetVar: ::std::option::Option<
        unsafe extern "C" fn(
            interp: *mut Tcl_Interp,
            varName: *const ::std::os::raw::c_char,
            flags: ::std::os::raw::c_int,
        ) -> *const ::std::os::raw::c_char,
    >,
    pub tcl_GetVar2: ::std::option::Option<
        unsafe extern "C" fn(
            interp: *mut Tcl_Interp,
            part1: *const ::std::os::raw::c_char,
            part2: *const ::std::os::raw::c_char,
            flags: ::std::os::raw::c_int,
        ) -> *const ::std::os::raw::c_char,
    >,
    pub tcl_GlobalEval: ::std::option::Option<
        unsafe extern "C" fn(
            interp: *mut Tcl_Interp,
            command: *const ::std::os::raw::c_char,
        ) -> ::std::os::raw::c_int,
    >,
    pub tcl_GlobalEvalObj: ::std::option::Option<
        unsafe extern "C" fn(
            interp: *mut Tcl_Interp,
            objPtr: *mut Tcl_Obj,
        ) -> ::std::os::raw::c_int,
    >,
    pub tcl_HideCommand: ::std::option::Option<
        unsafe extern "C" fn(
            interp: *mut Tcl_Interp,
            cmdName: *const ::std::os::raw::c_char,
            hiddenCmdToken: *const ::std::os::raw::c_char,
        ) -> ::std::os::raw::c_int,
    >,
    pub tcl_Init: ::std::option::Option<
        unsafe extern "C" fn(interp: *mut Tcl_Interp) -> ::std::os::raw::c_int,
    >,
    pub tcl_InitHashTable: ::std::option::Option<
        unsafe extern "C" fn(tablePtr: *mut Tcl_HashTable, keyType: ::std::os::raw::c_int),
    >,
    pub tcl_InputBlocked:
        ::std::option::Option<unsafe extern "C" fn(chan: Tcl_Channel) -> ::std::os::raw::c_int>,
    pub tcl_InputBuffered:
        ::std::option::Option<unsafe extern "C" fn(chan: Tcl_Channel) -> ::std::os::raw::c_int>,
    pub tcl_InterpDeleted: ::std::option::Option<
        unsafe extern "C" fn(interp: *mut Tcl_Interp) -> ::std::os::raw::c_int,
    >,
    pub tcl_IsSafe: ::std::option::Option<
        unsafe extern "C" fn(interp: *mut Tcl_Interp) -> ::std::os::raw::c_int,
    >,
    pub tcl_JoinPath: ::std::option::Option<
        unsafe extern "C" fn(
            argc: ::std::os::raw::c_int,
            argv: *const *const ::std::os::raw::c_char,
            resultPtr: *mut Tcl_DString,
        ) -> *mut ::std::os::raw::c_char,
    >,
    pub tcl_LinkVar: ::std::option::Option<
        unsafe extern "C" fn(
            interp: *mut Tcl_Interp,
            varName: *const ::std::os::raw::c_char,
            addr: *mut ::std::os::raw::c_char,
            type_: ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int,
    >,
    pub reserved188: *mut ::std::os::raw::c_void,
    pub tcl_MakeFileChannel: ::std::option::Option<
        unsafe extern "C" fn(handle: ClientData, mode: ::std::os::raw::c_int) -> Tcl_Channel,
    >,
    pub tcl_MakeSafe: ::std::option::Option<
        unsafe extern "C" fn(interp: *mut Tcl_Interp) -> ::std::os::raw::c_int,
    >,
    pub tcl_MakeTcpClientChannel:
        ::std::option::Option<unsafe extern "C" fn(tcpSocket: ClientData) -> Tcl_Channel>,
    pub tcl_Merge: ::std::option::Option<
        unsafe extern "C" fn(
            argc: ::std::os::raw::c_int,
            argv: *const *const ::std::os::raw::c_char,
        ) -> *mut ::std::os::raw::c_char,
    >,
    pub tcl_NextHashEntry: ::std::option::Option<
        unsafe extern "C" fn(searchPtr: *mut Tcl_HashSearch) -> *mut Tcl_HashEntry,
    >,
    pub tcl_NotifyChannel: ::std::option::Option<
        unsafe extern "C" fn(channel: Tcl_Channel, mask: ::std::os::raw::c_int),
    >,
    pub tcl_ObjGetVar2: ::std::option::Option<
        unsafe extern "C" fn(
            interp: *mut Tcl_Interp,
            part1Ptr: *mut Tcl_Obj,
            part2Ptr: *mut Tcl_Obj,
            flags: ::std::os::raw::c_int,
        ) -> *mut Tcl_Obj,
    >,
    pub tcl_ObjSetVar2: ::std::option::Option<
        unsafe extern "C" fn(
            interp: *mut Tcl_Interp,
            part1Ptr: *mut Tcl_Obj,
            part2Ptr: *mut Tcl_Obj,
            newValuePtr: *mut Tcl_Obj,
            flags: ::std::os::raw::c_int,
        ) -> *mut Tcl_Obj,
    >,
    pub tcl_OpenCommandChannel: ::std::option::Option<
        unsafe extern "C" fn(
            interp: *mut Tcl_Interp,
            argc: ::std::os::raw::c_int,
            argv: *mut *const ::std::os::raw::c_char,
            flags: ::std::os::raw::c_int,
        ) -> Tcl_Channel,
    >,
    pub tcl_OpenFileChannel: ::std::option::Option<
        unsafe extern "C" fn(
            interp: *mut Tcl_Interp,
            fileName: *const ::std::os::raw::c_char,
            modeString: *const ::std::os::raw::c_char,
            permissions: ::std::os::raw::c_int,
        ) -> Tcl_Channel,
    >,
    pub tcl_OpenTcpClient: ::std::option::Option<
        unsafe extern "C" fn(
            interp: *mut Tcl_Interp,
            port: ::std::os::raw::c_int,
            address: *const ::std::os::raw::c_char,
            myaddr: *const ::std::os::raw::c_char,
            myport: ::std::os::raw::c_int,
            async_: ::std::os::raw::c_int,
        ) -> Tcl_Channel,
    >,
    pub tcl_OpenTcpServer: ::std::option::Option<
        unsafe extern "C" fn(
            interp: *mut Tcl_Interp,
            port: ::std::os::raw::c_int,
            host: *const ::std::os::raw::c_char,
            acceptProc: Tcl_TcpAcceptProc,
            callbackData: ClientData,
        ) -> Tcl_Channel,
    >,
    pub tcl_Preserve: ::std::option::Option<unsafe extern "C" fn(data: ClientData)>,
    pub tcl_PrintDouble: ::std::option::Option<
        unsafe extern "C" fn(interp: *mut Tcl_Interp, value: f64, dst: *mut ::std::os::raw::c_char),
    >,
    pub tcl_PutEnv: ::std::option::Option<
        unsafe extern "C" fn(assignment: *const ::std::os::raw::c_char) -> ::std::os::raw::c_int,
    >,
    pub tcl_PosixError: ::std::option::Option<
        unsafe extern "C" fn(interp: *mut Tcl_Interp) -> *const ::std::os::raw::c_char,
    >,
    pub tcl_QueueEvent: ::std::option::Option<
        unsafe extern "C" fn(evPtr: *mut Tcl_Event, position: Tcl_QueuePosition),
    >,
    pub tcl_Read: ::std::option::Option<
        unsafe extern "C" fn(
            chan: Tcl_Channel,
            bufPtr: *mut ::std::os::raw::c_char,
            toRead: ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int,
    >,
    pub tcl_ReapDetachedProcs: ::std::option::Option<unsafe extern "C" fn()>,
    pub tcl_RecordAndEval: ::std::option::Option<
        unsafe extern "C" fn(
            interp: *mut Tcl_Interp,
            cmd: *const ::std::os::raw::c_char,
            flags: ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int,
    >,
    pub tcl_RecordAndEvalObj: ::std::option::Option<
        unsafe extern "C" fn(
            interp: *mut Tcl_Interp,
            cmdPtr: *mut Tcl_Obj,
            flags: ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int,
    >,
    pub tcl_RegisterChannel:
        ::std::option::Option<unsafe extern "C" fn(interp: *mut Tcl_Interp, chan: Tcl_Channel)>,
    pub tcl_RegisterObjType: ::std::option::Option<unsafe extern "C" fn(typePtr: *mut Tcl_ObjType)>,
    pub tcl_RegExpCompile: ::std::option::Option<
        unsafe extern "C" fn(
            interp: *mut Tcl_Interp,
            pattern: *const ::std::os::raw::c_char,
        ) -> Tcl_RegExp,
    >,
    pub tcl_RegExpExec: ::std::option::Option<
        unsafe extern "C" fn(
            interp: *mut Tcl_Interp,
            regexp: Tcl_RegExp,
            text: *const ::std::os::raw::c_char,
            start: *const ::std::os::raw::c_char,
        ) -> ::std::os::raw::c_int,
    >,
    pub tcl_RegExpMatch: ::std::option::Option<
        unsafe extern "C" fn(
            interp: *mut Tcl_Interp,
            text: *const ::std::os::raw::c_char,
            pattern: *const ::std::os::raw::c_char,
        ) -> ::std::os::raw::c_int,
    >,
    pub tcl_RegExpRange: ::std::option::Option<
        unsafe extern "C" fn(
            regexp: Tcl_RegExp,
            index: ::std::os::raw::c_int,
            startPtr: *mut *const ::std::os::raw::c_char,
            endPtr: *mut *const ::std::os::raw::c_char,
        ),
    >,
    pub tcl_Release: ::std::option::Option<unsafe extern "C" fn(clientData: ClientData)>,
    pub tcl_ResetResult: ::std::option::Option<unsafe extern "C" fn(interp: *mut Tcl_Interp)>,
    pub tcl_ScanElement: ::std::option::Option<
        unsafe extern "C" fn(
            str_: *const ::std::os::raw::c_char,
            flagPtr: *mut ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int,
    >,
    pub tcl_ScanCountedElement: ::std::option::Option<
        unsafe extern "C" fn(
            str_: *const ::std::os::raw::c_char,
            length: ::std::os::raw::c_int,
            flagPtr: *mut ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int,
    >,
    pub tcl_SeekOld: ::std::option::Option<
        unsafe extern "C" fn(
            chan: Tcl_Channel,
            offset: ::std::os::raw::c_int,
            mode: ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int,
    >,
    pub tcl_ServiceAll: ::std::option::Option<unsafe extern "C" fn() -> ::std::os::raw::c_int>,
    pub tcl_ServiceEvent: ::std::option::Option<
        unsafe extern "C" fn(flags: ::std::os::raw::c_int) -> ::std::os::raw::c_int,
    >,
    pub tcl_SetAssocData: ::std::option::Option<
        unsafe extern "C" fn(
            interp: *mut Tcl_Interp,
            name: *const ::std::os::raw::c_char,
            proc_: Tcl_InterpDeleteProc,
            clientData: ClientData,
        ),
    >,
    pub tcl_SetChannelBufferSize:
        ::std::option::Option<unsafe extern "C" fn(chan: Tcl_Channel, sz: ::std::os::raw::c_int)>,
    pub tcl_SetChannelOption: ::std::option::Option<
        unsafe extern "C" fn(
            interp: *mut Tcl_Interp,
            chan: Tcl_Channel,
            optionName: *const ::std::os::raw::c_char,
            newValue: *const ::std::os::raw::c_char,
        ) -> ::std::os::raw::c_int,
    >,
    pub tcl_SetCommandInfo: ::std::option::Option<
        unsafe extern "C" fn(
            interp: *mut Tcl_Interp,
            cmdName: *const ::std::os::raw::c_char,
            infoPtr: *const Tcl_CmdInfo,
        ) -> ::std::os::raw::c_int,
    >,
    pub tcl_SetErrno: ::std::option::Option<unsafe extern "C" fn(err: ::std::os::raw::c_int)>,
    pub tcl_SetErrorCode: ::std::option::Option<unsafe extern "C" fn(interp: *mut Tcl_Interp, ...)>,
    pub tcl_SetMaxBlockTime: ::std::option::Option<unsafe extern "C" fn(timePtr: *mut Tcl_Time)>,
    pub tcl_SetPanicProc: ::std::option::Option<unsafe extern "C" fn(panicProc: Tcl_PanicProc)>,
    pub tcl_SetRecursionLimit: ::std::option::Option<
        unsafe extern "C" fn(
            interp: *mut Tcl_Interp,
            depth: ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int,
    >,
    pub tcl_SetResult: ::std::option::Option<
        unsafe extern "C" fn(
            interp: *mut Tcl_Interp,
            result: *mut ::std::os::raw::c_char,
            freeProc: Tcl_FreeProc,
        ),
    >,
    pub tcl_SetServiceMode: ::std::option::Option<
        unsafe extern "C" fn(mode: ::std::os::raw::c_int) -> ::std::os::raw::c_int,
    >,
    pub tcl_SetObjErrorCode: ::std::option::Option<
        unsafe extern "C" fn(interp: *mut Tcl_Interp, errorObjPtr: *mut Tcl_Obj),
    >,
    pub tcl_SetObjResult: ::std::option::Option<
        unsafe extern "C" fn(interp: *mut Tcl_Interp, resultObjPtr: *mut Tcl_Obj),
    >,
    pub tcl_SetStdChannel: ::std::option::Option<
        unsafe extern "C" fn(channel: Tcl_Channel, type_: ::std::os::raw::c_int),
    >,
    pub tcl_SetVar: ::std::option::Option<
        unsafe extern "C" fn(
            interp: *mut Tcl_Interp,
            varName: *const ::std::os::raw::c_char,
            newValue: *const ::std::os::raw::c_char,
            flags: ::std::os::raw::c_int,
        ) -> *const ::std::os::raw::c_char,
    >,
    pub tcl_SetVar2: ::std::option::Option<
        unsafe extern "C" fn(
            interp: *mut Tcl_Interp,
            part1: *const ::std::os::raw::c_char,
            part2: *const ::std::os::raw::c_char,
            newValue: *const ::std::os::raw::c_char,
            flags: ::std::os::raw::c_int,
        ) -> *const ::std::os::raw::c_char,
    >,
    pub tcl_SignalId: ::std::option::Option<
        unsafe extern "C" fn(sig: ::std::os::raw::c_int) -> *const ::std::os::raw::c_char,
    >,
    pub tcl_SignalMsg: ::std::option::Option<
        unsafe extern "C" fn(sig: ::std::os::raw::c_int) -> *const ::std::os::raw::c_char,
    >,
    pub tcl_SourceRCFile: ::std::option::Option<unsafe extern "C" fn(interp: *mut Tcl_Interp)>,
    pub tcl_SplitList: ::std::option::Option<
        unsafe extern "C" fn(
            interp: *mut Tcl_Interp,
            listStr: *const ::std::os::raw::c_char,
            argcPtr: *mut ::std::os::raw::c_int,
            argvPtr: *mut *mut *const ::std::os::raw::c_char,
        ) -> ::std::os::raw::c_int,
    >,
    pub tcl_SplitPath: ::std::option::Option<
        unsafe extern "C" fn(
            path: *const ::std::os::raw::c_char,
            argcPtr: *mut ::std::os::raw::c_int,
            argvPtr: *mut *mut *const ::std::os::raw::c_char,
        ),
    >,
    pub tcl_StaticPackage: ::std::option::Option<
        unsafe extern "C" fn(
            interp: *mut Tcl_Interp,
            pkgName: *const ::std::os::raw::c_char,
            initProc: Tcl_PackageInitProc,
            safeInitProc: Tcl_PackageInitProc,
        ),
    >,
    pub tcl_StringMatch: ::std::option::Option<
        unsafe extern "C" fn(
            str_: *const ::std::os::raw::c_char,
            pattern: *const ::std::os::raw::c_char,
        ) -> ::std::os::raw::c_int,
    >,
    pub tcl_TellOld:
        ::std::option::Option<unsafe extern "C" fn(chan: Tcl_Channel) -> ::std::os::raw::c_int>,
    pub tcl_TraceVar: ::std::option::Option<
        unsafe extern "C" fn(
            interp: *mut Tcl_Interp,
            varName: *const ::std::os::raw::c_char,
            flags: ::std::os::raw::c_int,
            proc_: Tcl_VarTraceProc,
            clientData: ClientData,
        ) -> ::std::os::raw::c_int,
    >,
    pub tcl_TraceVar2: ::std::option::Option<
        unsafe extern "C" fn(
            interp: *mut Tcl_Interp,
            part1: *const ::std::os::raw::c_char,
            part2: *const ::std::os::raw::c_char,
            flags: ::std::os::raw::c_int,
            proc_: Tcl_VarTraceProc,
            clientData: ClientData,
        ) -> ::std::os::raw::c_int,
    >,
    pub tcl_TranslateFileName: ::std::option::Option<
        unsafe extern "C" fn(
            interp: *mut Tcl_Interp,
            name: *const ::std::os::raw::c_char,
            bufferPtr: *mut Tcl_DString,
        ) -> *mut ::std::os::raw::c_char,
    >,
    pub tcl_Ungets: ::std::option::Option<
        unsafe extern "C" fn(
            chan: Tcl_Channel,
            str_: *const ::std::os::raw::c_char,
            len: ::std::os::raw::c_int,
            atHead: ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int,
    >,
    pub tcl_UnlinkVar: ::std::option::Option<
        unsafe extern "C" fn(interp: *mut Tcl_Interp, varName: *const ::std::os::raw::c_char),
    >,
    pub tcl_UnregisterChannel: ::std::option::Option<
        unsafe extern "C" fn(interp: *mut Tcl_Interp, chan: Tcl_Channel) -> ::std::os::raw::c_int,
    >,
    pub tcl_UnsetVar: ::std::option::Option<
        unsafe extern "C" fn(
            interp: *mut Tcl_Interp,
            varName: *const ::std::os::raw::c_char,
            flags: ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int,
    >,
    pub tcl_UnsetVar2: ::std::option::Option<
        unsafe extern "C" fn(
            interp: *mut Tcl_Interp,
            part1: *const ::std::os::raw::c_char,
            part2: *const ::std::os::raw::c_char,
            flags: ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int,
    >,
    pub tcl_UntraceVar: ::std::option::Option<
        unsafe extern "C" fn(
            interp: *mut Tcl_Interp,
            varName: *const ::std::os::raw::c_char,
            flags: ::std::os::raw::c_int,
            proc_: Tcl_VarTraceProc,
            clientData: ClientData,
        ),
    >,
    pub tcl_UntraceVar2: ::std::option::Option<
        unsafe extern "C" fn(
            interp: *mut Tcl_Interp,
            part1: *const ::std::os::raw::c_char,
            part2: *const ::std::os::raw::c_char,
            flags: ::std::os::raw::c_int,
            proc_: Tcl_VarTraceProc,
            clientData: ClientData,
        ),
    >,
    pub tcl_UpdateLinkedVar: ::std::option::Option<
        unsafe extern "C" fn(interp: *mut Tcl_Interp, varName: *const ::std::os::raw::c_char),
    >,
    pub tcl_UpVar: ::std::option::Option<
        unsafe extern "C" fn(
            interp: *mut Tcl_Interp,
            frameName: *const ::std::os::raw::c_char,
            varName: *const ::std::os::raw::c_char,
            localName: *const ::std::os::raw::c_char,
            flags: ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int,
    >,
    pub tcl_UpVar2: ::std::option::Option<
        unsafe extern "C" fn(
            interp: *mut Tcl_Interp,
            frameName: *const ::std::os::raw::c_char,
            part1: *const ::std::os::raw::c_char,
            part2: *const ::std::os::raw::c_char,
            localName: *const ::std::os::raw::c_char,
            flags: ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int,
    >,
    pub tcl_VarEval: ::std::option::Option<
        unsafe extern "C" fn(interp: *mut Tcl_Interp, ...) -> ::std::os::raw::c_int,
    >,
    pub tcl_VarTraceInfo: ::std::option::Option<
        unsafe extern "C" fn(
            interp: *mut Tcl_Interp,
            varName: *const ::std::os::raw::c_char,
            flags: ::std::os::raw::c_int,
            procPtr: Tcl_VarTraceProc,
            prevClientData: ClientData,
        ) -> ClientData,
    >,
    pub tcl_VarTraceInfo2: ::std::option::Option<
        unsafe extern "C" fn(
            interp: *mut Tcl_Interp,
            part1: *const ::std::os::raw::c_char,
            part2: *const ::std::os::raw::c_char,
            flags: ::std::os::raw::c_int,
            procPtr: Tcl_VarTraceProc,
            prevClientData: ClientData,
        ) -> ClientData,
    >,
    pub tcl_Write: ::std::option::Option<
        unsafe extern "C" fn(
            chan: Tcl_Channel,
            s: *const ::std::os::raw::c_char,
            slen: ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int,
    >,
    pub tcl_WrongNumArgs: ::std::option::Option<
        unsafe extern "C" fn(
            interp: *mut Tcl_Interp,
            objc: ::std::os::raw::c_int,
            objv: *const *mut Tcl_Obj,
            message: *const ::std::os::raw::c_char,
        ),
    >,
    pub tcl_DumpActiveMemory: ::std::option::Option<
        unsafe extern "C" fn(fileName: *const ::std::os::raw::c_char) -> ::std::os::raw::c_int,
    >,
    pub tcl_ValidateAllMemory: ::std::option::Option<
        unsafe extern "C" fn(file: *const ::std::os::raw::c_char, line: ::std::os::raw::c_int),
    >,
    pub tcl_AppendResultVA:
        ::std::option::Option<unsafe extern "C" fn(interp: *mut Tcl_Interp, argList: va_list)>,
    pub tcl_AppendStringsToObjVA:
        ::std::option::Option<unsafe extern "C" fn(objPtr: *mut Tcl_Obj, argList: va_list)>,
    pub tcl_HashStats: ::std::option::Option<
        unsafe extern "C" fn(tablePtr: *mut Tcl_HashTable) -> *mut ::std::os::raw::c_char,
    >,
    pub tcl_ParseVar: ::std::option::Option<
        unsafe extern "C" fn(
            interp: *mut Tcl_Interp,
            start: *const ::std::os::raw::c_char,
            termPtr: *mut *const ::std::os::raw::c_char,
        ) -> *const ::std::os::raw::c_char,
    >,
    pub tcl_PkgPresent: ::std::option::Option<
        unsafe extern "C" fn(
            interp: *mut Tcl_Interp,
            name: *const ::std::os::raw::c_char,
            version: *const ::std::os::raw::c_char,
            exact: ::std::os::raw::c_int,
        ) -> *const ::std::os::raw::c_char,
    >,
    pub tcl_PkgPresentEx: ::std::option::Option<
        unsafe extern "C" fn(
            interp: *mut Tcl_Interp,
            name: *const ::std::os::raw::c_char,
            version: *const ::std::os::raw::c_char,
            exact: ::std::os::raw::c_int,
            clientDataPtr: *mut ClientData,
        ) -> *const ::std::os::raw::c_char,
    >,
    pub tcl_PkgProvide: ::std::option::Option<
        unsafe extern "C" fn(
            interp: *mut Tcl_Interp,
            name: *const ::std::os::raw::c_char,
            version: *const ::std::os::raw::c_char,
        ) -> ::std::os::raw::c_int,
    >,
    pub tcl_PkgRequire: ::std::option::Option<
        unsafe extern "C" fn(
            interp: *mut Tcl_Interp,
            name: *const ::std::os::raw::c_char,
            version: *const ::std::os::raw::c_char,
            exact: ::std::os::raw::c_int,
        ) -> *const ::std::os::raw::c_char,
    >,
    pub tcl_SetErrorCodeVA:
        ::std::option::Option<unsafe extern "C" fn(interp: *mut Tcl_Interp, argList: va_list)>,
    pub tcl_VarEvalVA: ::std::option::Option<
        unsafe extern "C" fn(interp: *mut Tcl_Interp, argList: va_list) -> ::std::os::raw::c_int,
    >,
    pub tcl_WaitPid: ::std::option::Option<
        unsafe extern "C" fn(
            pid: Tcl_Pid,
            statPtr: *mut ::std::os::raw::c_int,
            options: ::std::os::raw::c_int,
        ) -> Tcl_Pid,
    >,
    pub tcl_PanicVA: ::std::option::Option<
        unsafe extern "C" fn(format: *const ::std::os::raw::c_char, argList: va_list),
    >,
    pub tcl_GetVersion: ::std::option::Option<
        unsafe extern "C" fn(
            major: *mut ::std::os::raw::c_int,
            minor: *mut ::std::os::raw::c_int,
            patchLevel: *mut ::std::os::raw::c_int,
            type_: *mut ::std::os::raw::c_int,
        ),
    >,
    pub tcl_InitMemory: ::std::option::Option<unsafe extern "C" fn(interp: *mut Tcl_Interp)>,
    pub tcl_StackChannel: ::std::option::Option<
        unsafe extern "C" fn(
            interp: *mut Tcl_Interp,
            typePtr: *mut Tcl_ChannelType,
            instanceData: ClientData,
            mask: ::std::os::raw::c_int,
            prevChan: Tcl_Channel,
        ) -> Tcl_Channel,
    >,
    pub tcl_UnstackChannel: ::std::option::Option<
        unsafe extern "C" fn(interp: *mut Tcl_Interp, chan: Tcl_Channel) -> ::std::os::raw::c_int,
    >,
    pub tcl_GetStackedChannel:
        ::std::option::Option<unsafe extern "C" fn(chan: Tcl_Channel) -> Tcl_Channel>,
    pub tcl_SetMainLoop: ::std::option::Option<unsafe extern "C" fn(proc_: Tcl_MainLoopProc)>,
    pub reserved285: *mut ::std::os::raw::c_void,
    pub tcl_AppendObjToObj: ::std::option::Option<
        unsafe extern "C" fn(objPtr: *mut Tcl_Obj, appendObjPtr: *mut Tcl_Obj),
    >,
    pub tcl_CreateEncoding: ::std::option::Option<
        unsafe extern "C" fn(typePtr: *const Tcl_EncodingType) -> Tcl_Encoding,
    >,
    pub tcl_CreateThreadExitHandler:
        ::std::option::Option<unsafe extern "C" fn(proc_: Tcl_ExitProc, clientData: ClientData)>,
    pub tcl_DeleteThreadExitHandler:
        ::std::option::Option<unsafe extern "C" fn(proc_: Tcl_ExitProc, clientData: ClientData)>,
    pub tcl_DiscardResult:
        ::std::option::Option<unsafe extern "C" fn(statePtr: *mut Tcl_SavedResult)>,
    pub tcl_EvalEx: ::std::option::Option<
        unsafe extern "C" fn(
            interp: *mut Tcl_Interp,
            script: *const ::std::os::raw::c_char,
            numBytes: ::std::os::raw::c_int,
            flags: ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int,
    >,
    pub tcl_EvalObjv: ::std::option::Option<
        unsafe extern "C" fn(
            interp: *mut Tcl_Interp,
            objc: ::std::os::raw::c_int,
            objv: *const *mut Tcl_Obj,
            flags: ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int,
    >,
    pub tcl_EvalObjEx: ::std::option::Option<
        unsafe extern "C" fn(
            interp: *mut Tcl_Interp,
            objPtr: *mut Tcl_Obj,
            flags: ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int,
    >,
    pub tcl_ExitThread: ::std::option::Option<unsafe extern "C" fn(status: ::std::os::raw::c_int)>,
    pub tcl_ExternalToUtf: ::std::option::Option<
        unsafe extern "C" fn(
            interp: *mut Tcl_Interp,
            encoding: Tcl_Encoding,
            src: *const ::std::os::raw::c_char,
            srcLen: ::std::os::raw::c_int,
            flags: ::std::os::raw::c_int,
            statePtr: *mut Tcl_EncodingState,
            dst: *mut ::std::os::raw::c_char,
            dstLen: ::std::os::raw::c_int,
            srcReadPtr: *mut ::std::os::raw::c_int,
            dstWrotePtr: *mut ::std::os::raw::c_int,
            dstCharsPtr: *mut ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int,
    >,
    pub tcl_ExternalToUtfDString: ::std::option::Option<
        unsafe extern "C" fn(
            encoding: Tcl_Encoding,
            src: *const ::std::os::raw::c_char,
            srcLen: ::std::os::raw::c_int,
            dsPtr: *mut Tcl_DString,
        ) -> *mut ::std::os::raw::c_char,
    >,
    pub tcl_FinalizeThread: ::std::option::Option<unsafe extern "C" fn()>,
    pub tcl_FinalizeNotifier: ::std::option::Option<unsafe extern "C" fn(clientData: ClientData)>,
    pub tcl_FreeEncoding: ::std::option::Option<unsafe extern "C" fn(encoding: Tcl_Encoding)>,
    pub tcl_GetCurrentThread: ::std::option::Option<unsafe extern "C" fn() -> Tcl_ThreadId>,
    pub tcl_GetEncoding: ::std::option::Option<
        unsafe extern "C" fn(
            interp: *mut Tcl_Interp,
            name: *const ::std::os::raw::c_char,
        ) -> Tcl_Encoding,
    >,
    pub tcl_GetEncodingName: ::std::option::Option<
        unsafe extern "C" fn(encoding: Tcl_Encoding) -> *const ::std::os::raw::c_char,
    >,
    pub tcl_GetEncodingNames: ::std::option::Option<unsafe extern "C" fn(interp: *mut Tcl_Interp)>,
    pub tcl_GetIndexFromObjStruct: ::std::option::Option<
        unsafe extern "C" fn(
            interp: *mut Tcl_Interp,
            objPtr: *mut Tcl_Obj,
            tablePtr: *const ::std::os::raw::c_void,
            offset: ::std::os::raw::c_int,
            msg: *const ::std::os::raw::c_char,
            flags: ::std::os::raw::c_int,
            indexPtr: *mut ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int,
    >,
    pub tcl_GetThreadData: ::std::option::Option<
        unsafe extern "C" fn(
            keyPtr: *mut Tcl_ThreadDataKey,
            size: ::std::os::raw::c_int,
        ) -> *mut ::std::os::raw::c_void,
    >,
    pub tcl_GetVar2Ex: ::std::option::Option<
        unsafe extern "C" fn(
            interp: *mut Tcl_Interp,
            part1: *const ::std::os::raw::c_char,
            part2: *const ::std::os::raw::c_char,
            flags: ::std::os::raw::c_int,
        ) -> *mut Tcl_Obj,
    >,
    pub tcl_InitNotifier: ::std::option::Option<unsafe extern "C" fn() -> ClientData>,
    pub tcl_MutexLock: ::std::option::Option<unsafe extern "C" fn(mutexPtr: *mut Tcl_Mutex)>,
    pub tcl_MutexUnlock: ::std::option::Option<unsafe extern "C" fn(mutexPtr: *mut Tcl_Mutex)>,
    pub tcl_ConditionNotify:
        ::std::option::Option<unsafe extern "C" fn(condPtr: *mut Tcl_Condition)>,
    pub tcl_ConditionWait: ::std::option::Option<
        unsafe extern "C" fn(
            condPtr: *mut Tcl_Condition,
            mutexPtr: *mut Tcl_Mutex,
            timePtr: *mut Tcl_Time,
        ),
    >,
    pub tcl_NumUtfChars: ::std::option::Option<
        unsafe extern "C" fn(
            src: *const ::std::os::raw::c_char,
            length: ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int,
    >,
    pub tcl_ReadChars: ::std::option::Option<
        unsafe extern "C" fn(
            channel: Tcl_Channel,
            objPtr: *mut Tcl_Obj,
            charsToRead: ::std::os::raw::c_int,
            appendFlag: ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int,
    >,
    pub tcl_RestoreResult: ::std::option::Option<
        unsafe extern "C" fn(interp: *mut Tcl_Interp, statePtr: *mut Tcl_SavedResult),
    >,
    pub tcl_SaveResult: ::std::option::Option<
        unsafe extern "C" fn(interp: *mut Tcl_Interp, statePtr: *mut Tcl_SavedResult),
    >,
    pub tcl_SetSystemEncoding: ::std::option::Option<
        unsafe extern "C" fn(
            interp: *mut Tcl_Interp,
            name: *const ::std::os::raw::c_char,
        ) -> ::std::os::raw::c_int,
    >,
    pub tcl_SetVar2Ex: ::std::option::Option<
        unsafe extern "C" fn(
            interp: *mut Tcl_Interp,
            part1: *const ::std::os::raw::c_char,
            part2: *const ::std::os::raw::c_char,
            newValuePtr: *mut Tcl_Obj,
            flags: ::std::os::raw::c_int,
        ) -> *mut Tcl_Obj,
    >,
    pub tcl_ThreadAlert: ::std::option::Option<unsafe extern "C" fn(threadId: Tcl_ThreadId)>,
    pub tcl_ThreadQueueEvent: ::std::option::Option<
        unsafe extern "C" fn(
            threadId: Tcl_ThreadId,
            evPtr: *mut Tcl_Event,
            position: Tcl_QueuePosition,
        ),
    >,
    pub tcl_UniCharAtIndex: ::std::option::Option<
        unsafe extern "C" fn(
            src: *const ::std::os::raw::c_char,
            index: ::std::os::raw::c_int,
        ) -> Tcl_UniChar,
    >,
    pub tcl_UniCharToLower:
        ::std::option::Option<unsafe extern "C" fn(ch: ::std::os::raw::c_int) -> Tcl_UniChar>,
    pub tcl_UniCharToTitle:
        ::std::option::Option<unsafe extern "C" fn(ch: ::std::os::raw::c_int) -> Tcl_UniChar>,
    pub tcl_UniCharToUpper:
        ::std::option::Option<unsafe extern "C" fn(ch: ::std::os::raw::c_int) -> Tcl_UniChar>,
    pub tcl_UniCharToUtf: ::std::option::Option<
        unsafe extern "C" fn(
            ch: ::std::os::raw::c_int,
            buf: *mut ::std::os::raw::c_char,
        ) -> ::std::os::raw::c_int,
    >,
    pub tcl_UtfAtIndex: ::std::option::Option<
        unsafe extern "C" fn(
            src: *const ::std::os::raw::c_char,
            index: ::std::os::raw::c_int,
        ) -> *const ::std::os::raw::c_char,
    >,
    pub tcl_UtfCharComplete: ::std::option::Option<
        unsafe extern "C" fn(
            src: *const ::std::os::raw::c_char,
            length: ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int,
    >,
    pub tcl_UtfBackslash: ::std::option::Option<
        unsafe extern "C" fn(
            src: *const ::std::os::raw::c_char,
            readPtr: *mut ::std::os::raw::c_int,
            dst: *mut ::std::os::raw::c_char,
        ) -> ::std::os::raw::c_int,
    >,
    pub tcl_UtfFindFirst: ::std::option::Option<
        unsafe extern "C" fn(
            src: *const ::std::os::raw::c_char,
            ch: ::std::os::raw::c_int,
        ) -> *const ::std::os::raw::c_char,
    >,
    pub tcl_UtfFindLast: ::std::option::Option<
        unsafe extern "C" fn(
            src: *const ::std::os::raw::c_char,
            ch: ::std::os::raw::c_int,
        ) -> *const ::std::os::raw::c_char,
    >,
    pub tcl_UtfNext: ::std::option::Option<
        unsafe extern "C" fn(src: *const ::std::os::raw::c_char) -> *const ::std::os::raw::c_char,
    >,
    pub tcl_UtfPrev: ::std::option::Option<
        unsafe extern "C" fn(
            src: *const ::std::os::raw::c_char,
            start: *const ::std::os::raw::c_char,
        ) -> *const ::std::os::raw::c_char,
    >,
    pub tcl_UtfToExternal: ::std::option::Option<
        unsafe extern "C" fn(
            interp: *mut Tcl_Interp,
            encoding: Tcl_Encoding,
            src: *const ::std::os::raw::c_char,
            srcLen: ::std::os::raw::c_int,
            flags: ::std::os::raw::c_int,
            statePtr: *mut Tcl_EncodingState,
            dst: *mut ::std::os::raw::c_char,
            dstLen: ::std::os::raw::c_int,
            srcReadPtr: *mut ::std::os::raw::c_int,
            dstWrotePtr: *mut ::std::os::raw::c_int,
            dstCharsPtr: *mut ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int,
    >,
    pub tcl_UtfToExternalDString: ::std::option::Option<
        unsafe extern "C" fn(
            encoding: Tcl_Encoding,
            src: *const ::std::os::raw::c_char,
            srcLen: ::std::os::raw::c_int,
            dsPtr: *mut Tcl_DString,
        ) -> *mut ::std::os::raw::c_char,
    >,
    pub tcl_UtfToLower: ::std::option::Option<
        unsafe extern "C" fn(src: *mut ::std::os::raw::c_char) -> ::std::os::raw::c_int,
    >,
    pub tcl_UtfToTitle: ::std::option::Option<
        unsafe extern "C" fn(src: *mut ::std::os::raw::c_char) -> ::std::os::raw::c_int,
    >,
    pub tcl_UtfToUniChar: ::std::option::Option<
        unsafe extern "C" fn(
            src: *const ::std::os::raw::c_char,
            chPtr: *mut Tcl_UniChar,
        ) -> ::std::os::raw::c_int,
    >,
    pub tcl_UtfToUpper: ::std::option::Option<
        unsafe extern "C" fn(src: *mut ::std::os::raw::c_char) -> ::std::os::raw::c_int,
    >,
    pub tcl_WriteChars: ::std::option::Option<
        unsafe extern "C" fn(
            chan: Tcl_Channel,
            src: *const ::std::os::raw::c_char,
            srcLen: ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int,
    >,
    pub tcl_WriteObj: ::std::option::Option<
        unsafe extern "C" fn(chan: Tcl_Channel, objPtr: *mut Tcl_Obj) -> ::std::os::raw::c_int,
    >,
    pub tcl_GetString: ::std::option::Option<
        unsafe extern "C" fn(objPtr: *mut Tcl_Obj) -> *mut ::std::os::raw::c_char,
    >,
    pub tcl_GetDefaultEncodingDir:
        ::std::option::Option<unsafe extern "C" fn() -> *const ::std::os::raw::c_char>,
    pub tcl_SetDefaultEncodingDir:
        ::std::option::Option<unsafe extern "C" fn(path: *const ::std::os::raw::c_char)>,
    pub tcl_AlertNotifier: ::std::option::Option<unsafe extern "C" fn(clientData: ClientData)>,
    pub tcl_ServiceModeHook:
        ::std::option::Option<unsafe extern "C" fn(mode: ::std::os::raw::c_int)>,
    pub tcl_UniCharIsAlnum: ::std::option::Option<
        unsafe extern "C" fn(ch: ::std::os::raw::c_int) -> ::std::os::raw::c_int,
    >,
    pub tcl_UniCharIsAlpha: ::std::option::Option<
        unsafe extern "C" fn(ch: ::std::os::raw::c_int) -> ::std::os::raw::c_int,
    >,
    pub tcl_UniCharIsDigit: ::std::option::Option<
        unsafe extern "C" fn(ch: ::std::os::raw::c_int) -> ::std::os::raw::c_int,
    >,
    pub tcl_UniCharIsLower: ::std::option::Option<
        unsafe extern "C" fn(ch: ::std::os::raw::c_int) -> ::std::os::raw::c_int,
    >,
    pub tcl_UniCharIsSpace: ::std::option::Option<
        unsafe extern "C" fn(ch: ::std::os::raw::c_int) -> ::std::os::raw::c_int,
    >,
    pub tcl_UniCharIsUpper: ::std::option::Option<
        unsafe extern "C" fn(ch: ::std::os::raw::c_int) -> ::std::os::raw::c_int,
    >,
    pub tcl_UniCharIsWordChar: ::std::option::Option<
        unsafe extern "C" fn(ch: ::std::os::raw::c_int) -> ::std::os::raw::c_int,
    >,
    pub tcl_UniCharLen: ::std::option::Option<
        unsafe extern "C" fn(uniStr: *const Tcl_UniChar) -> ::std::os::raw::c_int,
    >,
    pub tcl_UniCharNcmp: ::std::option::Option<
        unsafe extern "C" fn(
            ucs: *const Tcl_UniChar,
            uct: *const Tcl_UniChar,
            numChars: ::std::os::raw::c_ulong,
        ) -> ::std::os::raw::c_int,
    >,
    pub tcl_UniCharToUtfDString: ::std::option::Option<
        unsafe extern "C" fn(
            uniStr: *const Tcl_UniChar,
            uniLength: ::std::os::raw::c_int,
            dsPtr: *mut Tcl_DString,
        ) -> *mut ::std::os::raw::c_char,
    >,
    pub tcl_UtfToUniCharDString: ::std::option::Option<
        unsafe extern "C" fn(
            src: *const ::std::os::raw::c_char,
            length: ::std::os::raw::c_int,
            dsPtr: *mut Tcl_DString,
        ) -> *mut Tcl_UniChar,
    >,
    pub tcl_GetRegExpFromObj: ::std::option::Option<
        unsafe extern "C" fn(
            interp: *mut Tcl_Interp,
            patObj: *mut Tcl_Obj,
            flags: ::std::os::raw::c_int,
        ) -> Tcl_RegExp,
    >,
    pub tcl_EvalTokens: ::std::option::Option<
        unsafe extern "C" fn(
            interp: *mut Tcl_Interp,
            tokenPtr: *mut Tcl_Token,
            count: ::std::os::raw::c_int,
        ) -> *mut Tcl_Obj,
    >,
    pub tcl_FreeParse: ::std::option::Option<unsafe extern "C" fn(parsePtr: *mut Tcl_Parse)>,
    pub tcl_LogCommandInfo: ::std::option::Option<
        unsafe extern "C" fn(
            interp: *mut Tcl_Interp,
            script: *const ::std::os::raw::c_char,
            command: *const ::std::os::raw::c_char,
            length: ::std::os::raw::c_int,
        ),
    >,
    pub tcl_ParseBraces: ::std::option::Option<
        unsafe extern "C" fn(
            interp: *mut Tcl_Interp,
            start: *const ::std::os::raw::c_char,
            numBytes: ::std::os::raw::c_int,
            parsePtr: *mut Tcl_Parse,
            append: ::std::os::raw::c_int,
            termPtr: *mut *const ::std::os::raw::c_char,
        ) -> ::std::os::raw::c_int,
    >,
    pub tcl_ParseCommand: ::std::option::Option<
        unsafe extern "C" fn(
            interp: *mut Tcl_Interp,
            start: *const ::std::os::raw::c_char,
            numBytes: ::std::os::raw::c_int,
            nested: ::std::os::raw::c_int,
            parsePtr: *mut Tcl_Parse,
        ) -> ::std::os::raw::c_int,
    >,
    pub tcl_ParseExpr: ::std::option::Option<
        unsafe extern "C" fn(
            interp: *mut Tcl_Interp,
            start: *const ::std::os::raw::c_char,
            numBytes: ::std::os::raw::c_int,
            parsePtr: *mut Tcl_Parse,
        ) -> ::std::os::raw::c_int,
    >,
    pub tcl_ParseQuotedString: ::std::option::Option<
        unsafe extern "C" fn(
            interp: *mut Tcl_Interp,
            start: *const ::std::os::raw::c_char,
            numBytes: ::std::os::raw::c_int,
            parsePtr: *mut Tcl_Parse,
            append: ::std::os::raw::c_int,
            termPtr: *mut *const ::std::os::raw::c_char,
        ) -> ::std::os::raw::c_int,
    >,
    pub tcl_ParseVarName: ::std::option::Option<
        unsafe extern "C" fn(
            interp: *mut Tcl_Interp,
            start: *const ::std::os::raw::c_char,
            numBytes: ::std::os::raw::c_int,
            parsePtr: *mut Tcl_Parse,
            append: ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int,
    >,
    pub tcl_GetCwd: ::std::option::Option<
        unsafe extern "C" fn(
            interp: *mut Tcl_Interp,
            cwdPtr: *mut Tcl_DString,
        ) -> *mut ::std::os::raw::c_char,
    >,
    pub tcl_Chdir: ::std::option::Option<
        unsafe extern "C" fn(dirName: *const ::std::os::raw::c_char) -> ::std::os::raw::c_int,
    >,
    pub tcl_Access: ::std::option::Option<
        unsafe extern "C" fn(
            path: *const ::std::os::raw::c_char,
            mode: ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int,
    >,
    pub tcl_Stat: ::std::option::Option<
        unsafe extern "C" fn(
            path: *const ::std::os::raw::c_char,
            bufPtr: *mut stat,
        ) -> ::std::os::raw::c_int,
    >,
    pub tcl_UtfNcmp: ::std::option::Option<
        unsafe extern "C" fn(
            s1: *const ::std::os::raw::c_char,
            s2: *const ::std::os::raw::c_char,
            n: ::std::os::raw::c_ulong,
        ) -> ::std::os::raw::c_int,
    >,
    pub tcl_UtfNcasecmp: ::std::option::Option<
        unsafe extern "C" fn(
            s1: *const ::std::os::raw::c_char,
            s2: *const ::std::os::raw::c_char,
            n: ::std::os::raw::c_ulong,
        ) -> ::std::os::raw::c_int,
    >,
    pub tcl_StringCaseMatch: ::std::option::Option<
        unsafe extern "C" fn(
            str_: *const ::std::os::raw::c_char,
            pattern: *const ::std::os::raw::c_char,
            nocase: ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int,
    >,
    pub tcl_UniCharIsControl: ::std::option::Option<
        unsafe extern "C" fn(ch: ::std::os::raw::c_int) -> ::std::os::raw::c_int,
    >,
    pub tcl_UniCharIsGraph: ::std::option::Option<
        unsafe extern "C" fn(ch: ::std::os::raw::c_int) -> ::std::os::raw::c_int,
    >,
    pub tcl_UniCharIsPrint: ::std::option::Option<
        unsafe extern "C" fn(ch: ::std::os::raw::c_int) -> ::std::os::raw::c_int,
    >,
    pub tcl_UniCharIsPunct: ::std::option::Option<
        unsafe extern "C" fn(ch: ::std::os::raw::c_int) -> ::std::os::raw::c_int,
    >,
    pub tcl_RegExpExecObj: ::std::option::Option<
        unsafe extern "C" fn(
            interp: *mut Tcl_Interp,
            regexp: Tcl_RegExp,
            textObj: *mut Tcl_Obj,
            offset: ::std::os::raw::c_int,
            nmatches: ::std::os::raw::c_int,
            flags: ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int,
    >,
    pub tcl_RegExpGetInfo: ::std::option::Option<
        unsafe extern "C" fn(regexp: Tcl_RegExp, infoPtr: *mut Tcl_RegExpInfo),
    >,
    pub tcl_NewUnicodeObj: ::std::option::Option<
        unsafe extern "C" fn(
            unicode: *const Tcl_UniChar,
            numChars: ::std::os::raw::c_int,
        ) -> *mut Tcl_Obj,
    >,
    pub tcl_SetUnicodeObj: ::std::option::Option<
        unsafe extern "C" fn(
            objPtr: *mut Tcl_Obj,
            unicode: *const Tcl_UniChar,
            numChars: ::std::os::raw::c_int,
        ),
    >,
    pub tcl_GetCharLength:
        ::std::option::Option<unsafe extern "C" fn(objPtr: *mut Tcl_Obj) -> ::std::os::raw::c_int>,
    pub tcl_GetUniChar: ::std::option::Option<
        unsafe extern "C" fn(objPtr: *mut Tcl_Obj, index: ::std::os::raw::c_int) -> Tcl_UniChar,
    >,
    pub tcl_GetUnicode:
        ::std::option::Option<unsafe extern "C" fn(objPtr: *mut Tcl_Obj) -> *mut Tcl_UniChar>,
    pub tcl_GetRange: ::std::option::Option<
        unsafe extern "C" fn(
            objPtr: *mut Tcl_Obj,
            first: ::std::os::raw::c_int,
            last: ::std::os::raw::c_int,
        ) -> *mut Tcl_Obj,
    >,
    pub tcl_AppendUnicodeToObj: ::std::option::Option<
        unsafe extern "C" fn(
            objPtr: *mut Tcl_Obj,
            unicode: *const Tcl_UniChar,
            length: ::std::os::raw::c_int,
        ),
    >,
    pub tcl_RegExpMatchObj: ::std::option::Option<
        unsafe extern "C" fn(
            interp: *mut Tcl_Interp,
            textObj: *mut Tcl_Obj,
            patternObj: *mut Tcl_Obj,
        ) -> ::std::os::raw::c_int,
    >,
    pub tcl_SetNotifier:
        ::std::option::Option<unsafe extern "C" fn(notifierProcPtr: *mut Tcl_NotifierProcs)>,
    pub tcl_GetAllocMutex: ::std::option::Option<unsafe extern "C" fn() -> *mut Tcl_Mutex>,
    pub tcl_GetChannelNames: ::std::option::Option<
        unsafe extern "C" fn(interp: *mut Tcl_Interp) -> ::std::os::raw::c_int,
    >,
    pub tcl_GetChannelNamesEx: ::std::option::Option<
        unsafe extern "C" fn(
            interp: *mut Tcl_Interp,
            pattern: *const ::std::os::raw::c_char,
        ) -> ::std::os::raw::c_int,
    >,
    pub tcl_ProcObjCmd: ::std::option::Option<
        unsafe extern "C" fn(
            clientData: ClientData,
            interp: *mut Tcl_Interp,
            objc: ::std::os::raw::c_int,
            objv: *const *mut Tcl_Obj,
        ) -> ::std::os::raw::c_int,
    >,
    pub tcl_ConditionFinalize:
        ::std::option::Option<unsafe extern "C" fn(condPtr: *mut Tcl_Condition)>,
    pub tcl_MutexFinalize: ::std::option::Option<unsafe extern "C" fn(mutex: *mut Tcl_Mutex)>,
    pub tcl_CreateThread: ::std::option::Option<
        unsafe extern "C" fn(
            idPtr: *mut Tcl_ThreadId,
            proc_: Tcl_ThreadCreateProc,
            clientData: ClientData,
            stackSize: ::std::os::raw::c_int,
            flags: ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int,
    >,
    pub tcl_ReadRaw: ::std::option::Option<
        unsafe extern "C" fn(
            chan: Tcl_Channel,
            dst: *mut ::std::os::raw::c_char,
            bytesToRead: ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int,
    >,
    pub tcl_WriteRaw: ::std::option::Option<
        unsafe extern "C" fn(
            chan: Tcl_Channel,
            src: *const ::std::os::raw::c_char,
            srcLen: ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int,
    >,
    pub tcl_GetTopChannel:
        ::std::option::Option<unsafe extern "C" fn(chan: Tcl_Channel) -> Tcl_Channel>,
    pub tcl_ChannelBuffered:
        ::std::option::Option<unsafe extern "C" fn(chan: Tcl_Channel) -> ::std::os::raw::c_int>,
    pub tcl_ChannelName: ::std::option::Option<
        unsafe extern "C" fn(chanTypePtr: *const Tcl_ChannelType) -> *const ::std::os::raw::c_char,
    >,
    pub tcl_ChannelVersion: ::std::option::Option<
        unsafe extern "C" fn(chanTypePtr: *const Tcl_ChannelType) -> Tcl_ChannelTypeVersion,
    >,
    pub tcl_ChannelBlockModeProc: ::std::option::Option<
        unsafe extern "C" fn(chanTypePtr: *const Tcl_ChannelType) -> Tcl_DriverBlockModeProc,
    >,
    pub tcl_ChannelCloseProc: ::std::option::Option<
        unsafe extern "C" fn(chanTypePtr: *const Tcl_ChannelType) -> Tcl_DriverCloseProc,
    >,
    pub tcl_ChannelClose2Proc: ::std::option::Option<
        unsafe extern "C" fn(chanTypePtr: *const Tcl_ChannelType) -> Tcl_DriverClose2Proc,
    >,
    pub tcl_ChannelInputProc: ::std::option::Option<
        unsafe extern "C" fn(chanTypePtr: *const Tcl_ChannelType) -> Tcl_DriverInputProc,
    >,
    pub tcl_ChannelOutputProc: ::std::option::Option<
        unsafe extern "C" fn(chanTypePtr: *const Tcl_ChannelType) -> Tcl_DriverOutputProc,
    >,
    pub tcl_ChannelSeekProc: ::std::option::Option<
        unsafe extern "C" fn(chanTypePtr: *const Tcl_ChannelType) -> Tcl_DriverSeekProc,
    >,
    pub tcl_ChannelSetOptionProc: ::std::option::Option<
        unsafe extern "C" fn(chanTypePtr: *const Tcl_ChannelType) -> Tcl_DriverSetOptionProc,
    >,
    pub tcl_ChannelGetOptionProc: ::std::option::Option<
        unsafe extern "C" fn(chanTypePtr: *const Tcl_ChannelType) -> Tcl_DriverGetOptionProc,
    >,
    pub tcl_ChannelWatchProc: ::std::option::Option<
        unsafe extern "C" fn(chanTypePtr: *const Tcl_ChannelType) -> Tcl_DriverWatchProc,
    >,
    pub tcl_ChannelGetHandleProc: ::std::option::Option<
        unsafe extern "C" fn(chanTypePtr: *const Tcl_ChannelType) -> Tcl_DriverGetHandleProc,
    >,
    pub tcl_ChannelFlushProc: ::std::option::Option<
        unsafe extern "C" fn(chanTypePtr: *const Tcl_ChannelType) -> Tcl_DriverFlushProc,
    >,
    pub tcl_ChannelHandlerProc: ::std::option::Option<
        unsafe extern "C" fn(chanTypePtr: *const Tcl_ChannelType) -> Tcl_DriverHandlerProc,
    >,
    pub tcl_JoinThread: ::std::option::Option<
        unsafe extern "C" fn(
            threadId: Tcl_ThreadId,
            result: *mut ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int,
    >,
    pub tcl_IsChannelShared:
        ::std::option::Option<unsafe extern "C" fn(channel: Tcl_Channel) -> ::std::os::raw::c_int>,
    pub tcl_IsChannelRegistered: ::std::option::Option<
        unsafe extern "C" fn(
            interp: *mut Tcl_Interp,
            channel: Tcl_Channel,
        ) -> ::std::os::raw::c_int,
    >,
    pub tcl_CutChannel: ::std::option::Option<unsafe extern "C" fn(channel: Tcl_Channel)>,
    pub tcl_SpliceChannel: ::std::option::Option<unsafe extern "C" fn(channel: Tcl_Channel)>,
    pub tcl_ClearChannelHandlers: ::std::option::Option<unsafe extern "C" fn(channel: Tcl_Channel)>,
    pub tcl_IsChannelExisting: ::std::option::Option<
        unsafe extern "C" fn(channelName: *const ::std::os::raw::c_char) -> ::std::os::raw::c_int,
    >,
    pub tcl_UniCharNcasecmp: ::std::option::Option<
        unsafe extern "C" fn(
            ucs: *const Tcl_UniChar,
            uct: *const Tcl_UniChar,
            numChars: ::std::os::raw::c_ulong,
        ) -> ::std::os::raw::c_int,
    >,
    pub tcl_UniCharCaseMatch: ::std::option::Option<
        unsafe extern "C" fn(
            uniStr: *const Tcl_UniChar,
            uniPattern: *const Tcl_UniChar,
            nocase: ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int,
    >,
    pub tcl_FindHashEntry: ::std::option::Option<
        unsafe extern "C" fn(
            tablePtr: *mut Tcl_HashTable,
            key: *const ::std::os::raw::c_char,
        ) -> *mut Tcl_HashEntry,
    >,
    pub tcl_CreateHashEntry: ::std::option::Option<
        unsafe extern "C" fn(
            tablePtr: *mut Tcl_HashTable,
            key: *const ::std::os::raw::c_char,
            newPtr: *mut ::std::os::raw::c_int,
        ) -> *mut Tcl_HashEntry,
    >,
    pub tcl_InitCustomHashTable: ::std::option::Option<
        unsafe extern "C" fn(
            tablePtr: *mut Tcl_HashTable,
            keyType: ::std::os::raw::c_int,
            typePtr: *mut Tcl_HashKeyType,
        ),
    >,
    pub tcl_InitObjHashTable:
        ::std::option::Option<unsafe extern "C" fn(tablePtr: *mut Tcl_HashTable)>,
    pub tcl_CommandTraceInfo: ::std::option::Option<
        unsafe extern "C" fn(
            interp: *mut Tcl_Interp,
            varName: *const ::std::os::raw::c_char,
            flags: ::std::os::raw::c_int,
            procPtr: Tcl_CommandTraceProc,
            prevClientData: ClientData,
        ) -> ClientData,
    >,
    pub tcl_TraceCommand: ::std::option::Option<
        unsafe extern "C" fn(
            interp: *mut Tcl_Interp,
            varName: *const ::std::os::raw::c_char,
            flags: ::std::os::raw::c_int,
            proc_: Tcl_CommandTraceProc,
            clientData: ClientData,
        ) -> ::std::os::raw::c_int,
    >,
    pub tcl_UntraceCommand: ::std::option::Option<
        unsafe extern "C" fn(
            interp: *mut Tcl_Interp,
            varName: *const ::std::os::raw::c_char,
            flags: ::std::os::raw::c_int,
            proc_: Tcl_CommandTraceProc,
            clientData: ClientData,
        ),
    >,
    pub tcl_AttemptAlloc: ::std::option::Option<
        unsafe extern "C" fn(size: ::std::os::raw::c_uint) -> *mut ::std::os::raw::c_char,
    >,
    pub tcl_AttemptDbCkalloc: ::std::option::Option<
        unsafe extern "C" fn(
            size: ::std::os::raw::c_uint,
            file: *const ::std::os::raw::c_char,
            line: ::std::os::raw::c_int,
        ) -> *mut ::std::os::raw::c_char,
    >,
    pub tcl_AttemptRealloc: ::std::option::Option<
        unsafe extern "C" fn(
            ptr: *mut ::std::os::raw::c_char,
            size: ::std::os::raw::c_uint,
        ) -> *mut ::std::os::raw::c_char,
    >,
    pub tcl_AttemptDbCkrealloc: ::std::option::Option<
        unsafe extern "C" fn(
            ptr: *mut ::std::os::raw::c_char,
            size: ::std::os::raw::c_uint,
            file: *const ::std::os::raw::c_char,
            line: ::std::os::raw::c_int,
        ) -> *mut ::std::os::raw::c_char,
    >,
    pub tcl_AttemptSetObjLength: ::std::option::Option<
        unsafe extern "C" fn(
            objPtr: *mut Tcl_Obj,
            length: ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int,
    >,
    pub tcl_GetChannelThread:
        ::std::option::Option<unsafe extern "C" fn(channel: Tcl_Channel) -> Tcl_ThreadId>,
    pub tcl_GetUnicodeFromObj: ::std::option::Option<
        unsafe extern "C" fn(
            objPtr: *mut Tcl_Obj,
            lengthPtr: *mut ::std::os::raw::c_int,
        ) -> *mut Tcl_UniChar,
    >,
    pub tcl_GetMathFuncInfo: ::std::option::Option<
        unsafe extern "C" fn(
            interp: *mut Tcl_Interp,
            name: *const ::std::os::raw::c_char,
            numArgsPtr: *mut ::std::os::raw::c_int,
            argTypesPtr: *mut *mut Tcl_ValueType,
            procPtr: *mut Tcl_MathProc,
            clientDataPtr: *mut ClientData,
        ) -> ::std::os::raw::c_int,
    >,
    pub tcl_ListMathFuncs: ::std::option::Option<
        unsafe extern "C" fn(
            interp: *mut Tcl_Interp,
            pattern: *const ::std::os::raw::c_char,
        ) -> *mut Tcl_Obj,
    >,
    pub tcl_SubstObj: ::std::option::Option<
        unsafe extern "C" fn(
            interp: *mut Tcl_Interp,
            objPtr: *mut Tcl_Obj,
            flags: ::std::os::raw::c_int,
        ) -> *mut Tcl_Obj,
    >,
    pub tcl_DetachChannel: ::std::option::Option<
        unsafe extern "C" fn(
            interp: *mut Tcl_Interp,
            channel: Tcl_Channel,
        ) -> ::std::os::raw::c_int,
    >,
    pub tcl_IsStandardChannel:
        ::std::option::Option<unsafe extern "C" fn(channel: Tcl_Channel) -> ::std::os::raw::c_int>,
    pub tcl_FSCopyFile: ::std::option::Option<
        unsafe extern "C" fn(
            srcPathPtr: *mut Tcl_Obj,
            destPathPtr: *mut Tcl_Obj,
        ) -> ::std::os::raw::c_int,
    >,
    pub tcl_FSCopyDirectory: ::std::option::Option<
        unsafe extern "C" fn(
            srcPathPtr: *mut Tcl_Obj,
            destPathPtr: *mut Tcl_Obj,
            errorPtr: *mut *mut Tcl_Obj,
        ) -> ::std::os::raw::c_int,
    >,
    pub tcl_FSCreateDirectory:
        ::std::option::Option<unsafe extern "C" fn(pathPtr: *mut Tcl_Obj) -> ::std::os::raw::c_int>,
    pub tcl_FSDeleteFile:
        ::std::option::Option<unsafe extern "C" fn(pathPtr: *mut Tcl_Obj) -> ::std::os::raw::c_int>,
    pub tcl_FSLoadFile: ::std::option::Option<
        unsafe extern "C" fn(
            interp: *mut Tcl_Interp,
            pathPtr: *mut Tcl_Obj,
            sym1: *const ::std::os::raw::c_char,
            sym2: *const ::std::os::raw::c_char,
            proc1Ptr: *mut Tcl_PackageInitProc,
            proc2Ptr: *mut Tcl_PackageInitProc,
            handlePtr: *mut Tcl_LoadHandle,
            unloadProcPtr: *mut Tcl_FSUnloadFileProc,
        ) -> ::std::os::raw::c_int,
    >,
    pub tcl_FSMatchInDirectory: ::std::option::Option<
        unsafe extern "C" fn(
            interp: *mut Tcl_Interp,
            result: *mut Tcl_Obj,
            pathPtr: *mut Tcl_Obj,
            pattern: *const ::std::os::raw::c_char,
            types: *mut Tcl_GlobTypeData,
        ) -> ::std::os::raw::c_int,
    >,
    pub tcl_FSLink: ::std::option::Option<
        unsafe extern "C" fn(
            pathPtr: *mut Tcl_Obj,
            toPtr: *mut Tcl_Obj,
            linkAction: ::std::os::raw::c_int,
        ) -> *mut Tcl_Obj,
    >,
    pub tcl_FSRemoveDirectory: ::std::option::Option<
        unsafe extern "C" fn(
            pathPtr: *mut Tcl_Obj,
            recursive: ::std::os::raw::c_int,
            errorPtr: *mut *mut Tcl_Obj,
        ) -> ::std::os::raw::c_int,
    >,
    pub tcl_FSRenameFile: ::std::option::Option<
        unsafe extern "C" fn(
            srcPathPtr: *mut Tcl_Obj,
            destPathPtr: *mut Tcl_Obj,
        ) -> ::std::os::raw::c_int,
    >,
    pub tcl_FSLstat: ::std::option::Option<
        unsafe extern "C" fn(pathPtr: *mut Tcl_Obj, buf: *mut Tcl_StatBuf) -> ::std::os::raw::c_int,
    >,
    pub tcl_FSUtime: ::std::option::Option<
        unsafe extern "C" fn(pathPtr: *mut Tcl_Obj, tval: *mut utimbuf) -> ::std::os::raw::c_int,
    >,
    pub tcl_FSFileAttrsGet: ::std::option::Option<
        unsafe extern "C" fn(
            interp: *mut Tcl_Interp,
            index: ::std::os::raw::c_int,
            pathPtr: *mut Tcl_Obj,
            objPtrRef: *mut *mut Tcl_Obj,
        ) -> ::std::os::raw::c_int,
    >,
    pub tcl_FSFileAttrsSet: ::std::option::Option<
        unsafe extern "C" fn(
            interp: *mut Tcl_Interp,
            index: ::std::os::raw::c_int,
            pathPtr: *mut Tcl_Obj,
            objPtr: *mut Tcl_Obj,
        ) -> ::std::os::raw::c_int,
    >,
    pub tcl_FSFileAttrStrings: ::std::option::Option<
        unsafe extern "C" fn(
            pathPtr: *mut Tcl_Obj,
            objPtrRef: *mut *mut Tcl_Obj,
        ) -> *mut *const ::std::os::raw::c_char,
    >,
    pub tcl_FSStat: ::std::option::Option<
        unsafe extern "C" fn(pathPtr: *mut Tcl_Obj, buf: *mut Tcl_StatBuf) -> ::std::os::raw::c_int,
    >,
    pub tcl_FSAccess: ::std::option::Option<
        unsafe extern "C" fn(
            pathPtr: *mut Tcl_Obj,
            mode: ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int,
    >,
    pub tcl_FSOpenFileChannel: ::std::option::Option<
        unsafe extern "C" fn(
            interp: *mut Tcl_Interp,
            pathPtr: *mut Tcl_Obj,
            modeString: *const ::std::os::raw::c_char,
            permissions: ::std::os::raw::c_int,
        ) -> Tcl_Channel,
    >,
    pub tcl_FSGetCwd:
        ::std::option::Option<unsafe extern "C" fn(interp: *mut Tcl_Interp) -> *mut Tcl_Obj>,
    pub tcl_FSChdir:
        ::std::option::Option<unsafe extern "C" fn(pathPtr: *mut Tcl_Obj) -> ::std::os::raw::c_int>,
    pub tcl_FSConvertToPathType: ::std::option::Option<
        unsafe extern "C" fn(
            interp: *mut Tcl_Interp,
            pathPtr: *mut Tcl_Obj,
        ) -> ::std::os::raw::c_int,
    >,
    pub tcl_FSJoinPath: ::std::option::Option<
        unsafe extern "C" fn(
            listObj: *mut Tcl_Obj,
            elements: ::std::os::raw::c_int,
        ) -> *mut Tcl_Obj,
    >,
    pub tcl_FSSplitPath: ::std::option::Option<
        unsafe extern "C" fn(
            pathPtr: *mut Tcl_Obj,
            lenPtr: *mut ::std::os::raw::c_int,
        ) -> *mut Tcl_Obj,
    >,
    pub tcl_FSEqualPaths: ::std::option::Option<
        unsafe extern "C" fn(
            firstPtr: *mut Tcl_Obj,
            secondPtr: *mut Tcl_Obj,
        ) -> ::std::os::raw::c_int,
    >,
    pub tcl_FSGetNormalizedPath: ::std::option::Option<
        unsafe extern "C" fn(interp: *mut Tcl_Interp, pathPtr: *mut Tcl_Obj) -> *mut Tcl_Obj,
    >,
    pub tcl_FSJoinToPath: ::std::option::Option<
        unsafe extern "C" fn(
            pathPtr: *mut Tcl_Obj,
            objc: ::std::os::raw::c_int,
            objv: *const *mut Tcl_Obj,
        ) -> *mut Tcl_Obj,
    >,
    pub tcl_FSGetInternalRep: ::std::option::Option<
        unsafe extern "C" fn(pathPtr: *mut Tcl_Obj, fsPtr: *mut Tcl_Filesystem) -> ClientData,
    >,
    pub tcl_FSGetTranslatedPath: ::std::option::Option<
        unsafe extern "C" fn(interp: *mut Tcl_Interp, pathPtr: *mut Tcl_Obj) -> *mut Tcl_Obj,
    >,
    pub tcl_FSEvalFile: ::std::option::Option<
        unsafe extern "C" fn(
            interp: *mut Tcl_Interp,
            fileName: *mut Tcl_Obj,
        ) -> ::std::os::raw::c_int,
    >,
    pub tcl_FSNewNativePath: ::std::option::Option<
        unsafe extern "C" fn(
            fromFilesystem: *mut Tcl_Filesystem,
            clientData: ClientData,
        ) -> *mut Tcl_Obj,
    >,
    pub tcl_FSGetNativePath: ::std::option::Option<
        unsafe extern "C" fn(pathPtr: *mut Tcl_Obj) -> *const ::std::os::raw::c_char,
    >,
    pub tcl_FSFileSystemInfo:
        ::std::option::Option<unsafe extern "C" fn(pathPtr: *mut Tcl_Obj) -> *mut Tcl_Obj>,
    pub tcl_FSPathSeparator:
        ::std::option::Option<unsafe extern "C" fn(pathPtr: *mut Tcl_Obj) -> *mut Tcl_Obj>,
    pub tcl_FSListVolumes: ::std::option::Option<unsafe extern "C" fn() -> *mut Tcl_Obj>,
    pub tcl_FSRegister: ::std::option::Option<
        unsafe extern "C" fn(
            clientData: ClientData,
            fsPtr: *mut Tcl_Filesystem,
        ) -> ::std::os::raw::c_int,
    >,
    pub tcl_FSUnregister: ::std::option::Option<
        unsafe extern "C" fn(fsPtr: *mut Tcl_Filesystem) -> ::std::os::raw::c_int,
    >,
    pub tcl_FSData:
        ::std::option::Option<unsafe extern "C" fn(fsPtr: *mut Tcl_Filesystem) -> ClientData>,
    pub tcl_FSGetTranslatedStringPath: ::std::option::Option<
        unsafe extern "C" fn(
            interp: *mut Tcl_Interp,
            pathPtr: *mut Tcl_Obj,
        ) -> *const ::std::os::raw::c_char,
    >,
    pub tcl_FSGetFileSystemForPath:
        ::std::option::Option<unsafe extern "C" fn(pathPtr: *mut Tcl_Obj) -> *mut Tcl_Filesystem>,
    pub tcl_FSGetPathType:
        ::std::option::Option<unsafe extern "C" fn(pathPtr: *mut Tcl_Obj) -> Tcl_PathType>,
    pub tcl_OutputBuffered:
        ::std::option::Option<unsafe extern "C" fn(chan: Tcl_Channel) -> ::std::os::raw::c_int>,
    pub tcl_FSMountsChanged:
        ::std::option::Option<unsafe extern "C" fn(fsPtr: *mut Tcl_Filesystem)>,
    pub tcl_EvalTokensStandard: ::std::option::Option<
        unsafe extern "C" fn(
            interp: *mut Tcl_Interp,
            tokenPtr: *mut Tcl_Token,
            count: ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int,
    >,
    pub tcl_GetTime: ::std::option::Option<unsafe extern "C" fn(timeBuf: *mut Tcl_Time)>,
    pub tcl_CreateObjTrace: ::std::option::Option<
        unsafe extern "C" fn(
            interp: *mut Tcl_Interp,
            level: ::std::os::raw::c_int,
            flags: ::std::os::raw::c_int,
            objProc: Tcl_CmdObjTraceProc,
            clientData: ClientData,
            delProc: Tcl_CmdObjTraceDeleteProc,
        ) -> Tcl_Trace,
    >,
    pub tcl_GetCommandInfoFromToken: ::std::option::Option<
        unsafe extern "C" fn(
            token: Tcl_Command,
            infoPtr: *mut Tcl_CmdInfo,
        ) -> ::std::os::raw::c_int,
    >,
    pub tcl_SetCommandInfoFromToken: ::std::option::Option<
        unsafe extern "C" fn(
            token: Tcl_Command,
            infoPtr: *const Tcl_CmdInfo,
        ) -> ::std::os::raw::c_int,
    >,
    pub tcl_DbNewWideIntObj: ::std::option::Option<
        unsafe extern "C" fn(
            wideValue: Tcl_WideInt,
            file: *const ::std::os::raw::c_char,
            line: ::std::os::raw::c_int,
        ) -> *mut Tcl_Obj,
    >,
    pub tcl_GetWideIntFromObj: ::std::option::Option<
        unsafe extern "C" fn(
            interp: *mut Tcl_Interp,
            objPtr: *mut Tcl_Obj,
            widePtr: *mut Tcl_WideInt,
        ) -> ::std::os::raw::c_int,
    >,
    pub tcl_NewWideIntObj:
        ::std::option::Option<unsafe extern "C" fn(wideValue: Tcl_WideInt) -> *mut Tcl_Obj>,
    pub tcl_SetWideIntObj:
        ::std::option::Option<unsafe extern "C" fn(objPtr: *mut Tcl_Obj, wideValue: Tcl_WideInt)>,
    pub tcl_AllocStatBuf: ::std::option::Option<unsafe extern "C" fn() -> *mut Tcl_StatBuf>,
    pub tcl_Seek: ::std::option::Option<
        unsafe extern "C" fn(
            chan: Tcl_Channel,
            offset: Tcl_WideInt,
            mode: ::std::os::raw::c_int,
        ) -> Tcl_WideInt,
    >,
    pub tcl_Tell: ::std::option::Option<unsafe extern "C" fn(chan: Tcl_Channel) -> Tcl_WideInt>,
    pub tcl_ChannelWideSeekProc: ::std::option::Option<
        unsafe extern "C" fn(chanTypePtr: *const Tcl_ChannelType) -> Tcl_DriverWideSeekProc,
    >,
    pub tcl_DictObjPut: ::std::option::Option<
        unsafe extern "C" fn(
            interp: *mut Tcl_Interp,
            dictPtr: *mut Tcl_Obj,
            keyPtr: *mut Tcl_Obj,
            valuePtr: *mut Tcl_Obj,
        ) -> ::std::os::raw::c_int,
    >,
    pub tcl_DictObjGet: ::std::option::Option<
        unsafe extern "C" fn(
            interp: *mut Tcl_Interp,
            dictPtr: *mut Tcl_Obj,
            keyPtr: *mut Tcl_Obj,
            valuePtrPtr: *mut *mut Tcl_Obj,
        ) -> ::std::os::raw::c_int,
    >,
    pub tcl_DictObjRemove: ::std::option::Option<
        unsafe extern "C" fn(
            interp: *mut Tcl_Interp,
            dictPtr: *mut Tcl_Obj,
            keyPtr: *mut Tcl_Obj,
        ) -> ::std::os::raw::c_int,
    >,
    pub tcl_DictObjSize: ::std::option::Option<
        unsafe extern "C" fn(
            interp: *mut Tcl_Interp,
            dictPtr: *mut Tcl_Obj,
            sizePtr: *mut ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int,
    >,
    pub tcl_DictObjFirst: ::std::option::Option<
        unsafe extern "C" fn(
            interp: *mut Tcl_Interp,
            dictPtr: *mut Tcl_Obj,
            searchPtr: *mut Tcl_DictSearch,
            keyPtrPtr: *mut *mut Tcl_Obj,
            valuePtrPtr: *mut *mut Tcl_Obj,
            donePtr: *mut ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int,
    >,
    pub tcl_DictObjNext: ::std::option::Option<
        unsafe extern "C" fn(
            searchPtr: *mut Tcl_DictSearch,
            keyPtrPtr: *mut *mut Tcl_Obj,
            valuePtrPtr: *mut *mut Tcl_Obj,
            donePtr: *mut ::std::os::raw::c_int,
        ),
    >,
    pub tcl_DictObjDone:
        ::std::option::Option<unsafe extern "C" fn(searchPtr: *mut Tcl_DictSearch)>,
    pub tcl_DictObjPutKeyList: ::std::option::Option<
        unsafe extern "C" fn(
            interp: *mut Tcl_Interp,
            dictPtr: *mut Tcl_Obj,
            keyc: ::std::os::raw::c_int,
            keyv: *const *mut Tcl_Obj,
            valuePtr: *mut Tcl_Obj,
        ) -> ::std::os::raw::c_int,
    >,
    pub tcl_DictObjRemoveKeyList: ::std::option::Option<
        unsafe extern "C" fn(
            interp: *mut Tcl_Interp,
            dictPtr: *mut Tcl_Obj,
            keyc: ::std::os::raw::c_int,
            keyv: *const *mut Tcl_Obj,
        ) -> ::std::os::raw::c_int,
    >,
    pub tcl_NewDictObj: ::std::option::Option<unsafe extern "C" fn() -> *mut Tcl_Obj>,
    pub tcl_DbNewDictObj: ::std::option::Option<
        unsafe extern "C" fn(
            file: *const ::std::os::raw::c_char,
            line: ::std::os::raw::c_int,
        ) -> *mut Tcl_Obj,
    >,
    pub tcl_RegisterConfig: ::std::option::Option<
        unsafe extern "C" fn(
            interp: *mut Tcl_Interp,
            pkgName: *const ::std::os::raw::c_char,
            configuration: *mut Tcl_Config,
            valEncoding: *const ::std::os::raw::c_char,
        ),
    >,
    pub tcl_CreateNamespace: ::std::option::Option<
        unsafe extern "C" fn(
            interp: *mut Tcl_Interp,
            name: *const ::std::os::raw::c_char,
            clientData: ClientData,
            deleteProc: Tcl_NamespaceDeleteProc,
        ) -> *mut Tcl_Namespace,
    >,
    pub tcl_DeleteNamespace: ::std::option::Option<unsafe extern "C" fn(nsPtr: *mut Tcl_Namespace)>,
    pub tcl_AppendExportList: ::std::option::Option<
        unsafe extern "C" fn(
            interp: *mut Tcl_Interp,
            nsPtr: *mut Tcl_Namespace,
            objPtr: *mut Tcl_Obj,
        ) -> ::std::os::raw::c_int,
    >,
    pub tcl_Export: ::std::option::Option<
        unsafe extern "C" fn(
            interp: *mut Tcl_Interp,
            nsPtr: *mut Tcl_Namespace,
            pattern: *const ::std::os::raw::c_char,
            resetListFirst: ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int,
    >,
    pub tcl_Import: ::std::option::Option<
        unsafe extern "C" fn(
            interp: *mut Tcl_Interp,
            nsPtr: *mut Tcl_Namespace,
            pattern: *const ::std::os::raw::c_char,
            allowOverwrite: ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int,
    >,
    pub tcl_ForgetImport: ::std::option::Option<
        unsafe extern "C" fn(
            interp: *mut Tcl_Interp,
            nsPtr: *mut Tcl_Namespace,
            pattern: *const ::std::os::raw::c_char,
        ) -> ::std::os::raw::c_int,
    >,
    pub tcl_GetCurrentNamespace:
        ::std::option::Option<unsafe extern "C" fn(interp: *mut Tcl_Interp) -> *mut Tcl_Namespace>,
    pub tcl_GetGlobalNamespace:
        ::std::option::Option<unsafe extern "C" fn(interp: *mut Tcl_Interp) -> *mut Tcl_Namespace>,
    pub tcl_FindNamespace: ::std::option::Option<
        unsafe extern "C" fn(
            interp: *mut Tcl_Interp,
            name: *const ::std::os::raw::c_char,
            contextNsPtr: *mut Tcl_Namespace,
            flags: ::std::os::raw::c_int,
        ) -> *mut Tcl_Namespace,
    >,
    pub tcl_FindCommand: ::std::option::Option<
        unsafe extern "C" fn(
            interp: *mut Tcl_Interp,
            name: *const ::std::os::raw::c_char,
            contextNsPtr: *mut Tcl_Namespace,
            flags: ::std::os::raw::c_int,
        ) -> Tcl_Command,
    >,
    pub tcl_GetCommandFromObj: ::std::option::Option<
        unsafe extern "C" fn(interp: *mut Tcl_Interp, objPtr: *mut Tcl_Obj) -> Tcl_Command,
    >,
    pub tcl_GetCommandFullName: ::std::option::Option<
        unsafe extern "C" fn(interp: *mut Tcl_Interp, command: Tcl_Command, objPtr: *mut Tcl_Obj),
    >,
    pub tcl_FSEvalFileEx: ::std::option::Option<
        unsafe extern "C" fn(
            interp: *mut Tcl_Interp,
            fileName: *mut Tcl_Obj,
            encodingName: *const ::std::os::raw::c_char,
        ) -> ::std::os::raw::c_int,
    >,
    pub tcl_SetExitProc:
        ::std::option::Option<unsafe extern "C" fn(proc_: Tcl_ExitProc) -> Tcl_ExitProc>,
    pub tcl_LimitAddHandler: ::std::option::Option<
        unsafe extern "C" fn(
            interp: *mut Tcl_Interp,
            type_: ::std::os::raw::c_int,
            handlerProc: Tcl_LimitHandlerProc,
            clientData: ClientData,
            deleteProc: Tcl_LimitHandlerDeleteProc,
        ),
    >,
    pub tcl_LimitRemoveHandler: ::std::option::Option<
        unsafe extern "C" fn(
            interp: *mut Tcl_Interp,
            type_: ::std::os::raw::c_int,
            handlerProc: Tcl_LimitHandlerProc,
            clientData: ClientData,
        ),
    >,
    pub tcl_LimitReady: ::std::option::Option<
        unsafe extern "C" fn(interp: *mut Tcl_Interp) -> ::std::os::raw::c_int,
    >,
    pub tcl_LimitCheck: ::std::option::Option<
        unsafe extern "C" fn(interp: *mut Tcl_Interp) -> ::std::os::raw::c_int,
    >,
    pub tcl_LimitExceeded: ::std::option::Option<
        unsafe extern "C" fn(interp: *mut Tcl_Interp) -> ::std::os::raw::c_int,
    >,
    pub tcl_LimitSetCommands: ::std::option::Option<
        unsafe extern "C" fn(interp: *mut Tcl_Interp, commandLimit: ::std::os::raw::c_int),
    >,
    pub tcl_LimitSetTime: ::std::option::Option<
        unsafe extern "C" fn(interp: *mut Tcl_Interp, timeLimitPtr: *mut Tcl_Time),
    >,
    pub tcl_LimitSetGranularity: ::std::option::Option<
        unsafe extern "C" fn(
            interp: *mut Tcl_Interp,
            type_: ::std::os::raw::c_int,
            granularity: ::std::os::raw::c_int,
        ),
    >,
    pub tcl_LimitTypeEnabled: ::std::option::Option<
        unsafe extern "C" fn(
            interp: *mut Tcl_Interp,
            type_: ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int,
    >,
    pub tcl_LimitTypeExceeded: ::std::option::Option<
        unsafe extern "C" fn(
            interp: *mut Tcl_Interp,
            type_: ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int,
    >,
    pub tcl_LimitTypeSet: ::std::option::Option<
        unsafe extern "C" fn(interp: *mut Tcl_Interp, type_: ::std::os::raw::c_int),
    >,
    pub tcl_LimitTypeReset: ::std::option::Option<
        unsafe extern "C" fn(interp: *mut Tcl_Interp, type_: ::std::os::raw::c_int),
    >,
    pub tcl_LimitGetCommands: ::std::option::Option<
        unsafe extern "C" fn(interp: *mut Tcl_Interp) -> ::std::os::raw::c_int,
    >,
    pub tcl_LimitGetTime: ::std::option::Option<
        unsafe extern "C" fn(interp: *mut Tcl_Interp, timeLimitPtr: *mut Tcl_Time),
    >,
    pub tcl_LimitGetGranularity: ::std::option::Option<
        unsafe extern "C" fn(
            interp: *mut Tcl_Interp,
            type_: ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int,
    >,
    pub tcl_SaveInterpState: ::std::option::Option<
        unsafe extern "C" fn(
            interp: *mut Tcl_Interp,
            status: ::std::os::raw::c_int,
        ) -> Tcl_InterpState,
    >,
    pub tcl_RestoreInterpState: ::std::option::Option<
        unsafe extern "C" fn(
            interp: *mut Tcl_Interp,
            state: Tcl_InterpState,
        ) -> ::std::os::raw::c_int,
    >,
    pub tcl_DiscardInterpState: ::std::option::Option<unsafe extern "C" fn(state: Tcl_InterpState)>,
    pub tcl_SetReturnOptions: ::std::option::Option<
        unsafe extern "C" fn(
            interp: *mut Tcl_Interp,
            options: *mut Tcl_Obj,
        ) -> ::std::os::raw::c_int,
    >,
    pub tcl_GetReturnOptions: ::std::option::Option<
        unsafe extern "C" fn(
            interp: *mut Tcl_Interp,
            result: ::std::os::raw::c_int,
        ) -> *mut Tcl_Obj,
    >,
    pub tcl_IsEnsemble:
        ::std::option::Option<unsafe extern "C" fn(token: Tcl_Command) -> ::std::os::raw::c_int>,
    pub tcl_CreateEnsemble: ::std::option::Option<
        unsafe extern "C" fn(
            interp: *mut Tcl_Interp,
            name: *const ::std::os::raw::c_char,
            namespacePtr: *mut Tcl_Namespace,
            flags: ::std::os::raw::c_int,
        ) -> Tcl_Command,
    >,
    pub tcl_FindEnsemble: ::std::option::Option<
        unsafe extern "C" fn(
            interp: *mut Tcl_Interp,
            cmdNameObj: *mut Tcl_Obj,
            flags: ::std::os::raw::c_int,
        ) -> Tcl_Command,
    >,
    pub tcl_SetEnsembleSubcommandList: ::std::option::Option<
        unsafe extern "C" fn(
            interp: *mut Tcl_Interp,
            token: Tcl_Command,
            subcmdList: *mut Tcl_Obj,
        ) -> ::std::os::raw::c_int,
    >,
    pub tcl_SetEnsembleMappingDict: ::std::option::Option<
        unsafe extern "C" fn(
            interp: *mut Tcl_Interp,
            token: Tcl_Command,
            mapDict: *mut Tcl_Obj,
        ) -> ::std::os::raw::c_int,
    >,
    pub tcl_SetEnsembleUnknownHandler: ::std::option::Option<
        unsafe extern "C" fn(
            interp: *mut Tcl_Interp,
            token: Tcl_Command,
            unknownList: *mut Tcl_Obj,
        ) -> ::std::os::raw::c_int,
    >,
    pub tcl_SetEnsembleFlags: ::std::option::Option<
        unsafe extern "C" fn(
            interp: *mut Tcl_Interp,
            token: Tcl_Command,
            flags: ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int,
    >,
    pub tcl_GetEnsembleSubcommandList: ::std::option::Option<
        unsafe extern "C" fn(
            interp: *mut Tcl_Interp,
            token: Tcl_Command,
            subcmdListPtr: *mut *mut Tcl_Obj,
        ) -> ::std::os::raw::c_int,
    >,
    pub tcl_GetEnsembleMappingDict: ::std::option::Option<
        unsafe extern "C" fn(
            interp: *mut Tcl_Interp,
            token: Tcl_Command,
            mapDictPtr: *mut *mut Tcl_Obj,
        ) -> ::std::os::raw::c_int,
    >,
    pub tcl_GetEnsembleUnknownHandler: ::std::option::Option<
        unsafe extern "C" fn(
            interp: *mut Tcl_Interp,
            token: Tcl_Command,
            unknownListPtr: *mut *mut Tcl_Obj,
        ) -> ::std::os::raw::c_int,
    >,
    pub tcl_GetEnsembleFlags: ::std::option::Option<
        unsafe extern "C" fn(
            interp: *mut Tcl_Interp,
            token: Tcl_Command,
            flagsPtr: *mut ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int,
    >,
    pub tcl_GetEnsembleNamespace: ::std::option::Option<
        unsafe extern "C" fn(
            interp: *mut Tcl_Interp,
            token: Tcl_Command,
            namespacePtrPtr: *mut *mut Tcl_Namespace,
        ) -> ::std::os::raw::c_int,
    >,
    pub tcl_SetTimeProc: ::std::option::Option<
        unsafe extern "C" fn(
            getProc: Tcl_GetTimeProc,
            scaleProc: Tcl_ScaleTimeProc,
            clientData: ClientData,
        ),
    >,
    pub tcl_QueryTimeProc: ::std::option::Option<
        unsafe extern "C" fn(
            getProc: *mut Tcl_GetTimeProc,
            scaleProc: *mut Tcl_ScaleTimeProc,
            clientData: *mut ClientData,
        ),
    >,
    pub tcl_ChannelThreadActionProc: ::std::option::Option<
        unsafe extern "C" fn(chanTypePtr: *const Tcl_ChannelType) -> Tcl_DriverThreadActionProc,
    >,
    pub tcl_NewBignumObj:
        ::std::option::Option<unsafe extern "C" fn(value: *mut mp_int) -> *mut Tcl_Obj>,
    pub tcl_DbNewBignumObj: ::std::option::Option<
        unsafe extern "C" fn(
            value: *mut mp_int,
            file: *const ::std::os::raw::c_char,
            line: ::std::os::raw::c_int,
        ) -> *mut Tcl_Obj,
    >,
    pub tcl_SetBignumObj:
        ::std::option::Option<unsafe extern "C" fn(obj: *mut Tcl_Obj, value: *mut mp_int)>,
    pub tcl_GetBignumFromObj: ::std::option::Option<
        unsafe extern "C" fn(
            interp: *mut Tcl_Interp,
            obj: *mut Tcl_Obj,
            value: *mut mp_int,
        ) -> ::std::os::raw::c_int,
    >,
    pub tcl_TakeBignumFromObj: ::std::option::Option<
        unsafe extern "C" fn(
            interp: *mut Tcl_Interp,
            obj: *mut Tcl_Obj,
            value: *mut mp_int,
        ) -> ::std::os::raw::c_int,
    >,
    pub tcl_TruncateChannel: ::std::option::Option<
        unsafe extern "C" fn(chan: Tcl_Channel, length: Tcl_WideInt) -> ::std::os::raw::c_int,
    >,
    pub tcl_ChannelTruncateProc: ::std::option::Option<
        unsafe extern "C" fn(chanTypePtr: *const Tcl_ChannelType) -> Tcl_DriverTruncateProc,
    >,
    pub tcl_SetChannelErrorInterp:
        ::std::option::Option<unsafe extern "C" fn(interp: *mut Tcl_Interp, msg: *mut Tcl_Obj)>,
    pub tcl_GetChannelErrorInterp: ::std::option::Option<
        unsafe extern "C" fn(interp: *mut Tcl_Interp, msg: *mut *mut Tcl_Obj),
    >,
    pub tcl_SetChannelError:
        ::std::option::Option<unsafe extern "C" fn(chan: Tcl_Channel, msg: *mut Tcl_Obj)>,
    pub tcl_GetChannelError:
        ::std::option::Option<unsafe extern "C" fn(chan: Tcl_Channel, msg: *mut *mut Tcl_Obj)>,
    pub tcl_InitBignumFromDouble: ::std::option::Option<
        unsafe extern "C" fn(
            interp: *mut Tcl_Interp,
            initval: f64,
            toInit: *mut mp_int,
        ) -> ::std::os::raw::c_int,
    >,
    pub tcl_GetNamespaceUnknownHandler: ::std::option::Option<
        unsafe extern "C" fn(interp: *mut Tcl_Interp, nsPtr: *mut Tcl_Namespace) -> *mut Tcl_Obj,
    >,
    pub tcl_SetNamespaceUnknownHandler: ::std::option::Option<
        unsafe extern "C" fn(
            interp: *mut Tcl_Interp,
            nsPtr: *mut Tcl_Namespace,
            handlerPtr: *mut Tcl_Obj,
        ) -> ::std::os::raw::c_int,
    >,
    pub tcl_GetEncodingFromObj: ::std::option::Option<
        unsafe extern "C" fn(
            interp: *mut Tcl_Interp,
            objPtr: *mut Tcl_Obj,
            encodingPtr: *mut Tcl_Encoding,
        ) -> ::std::os::raw::c_int,
    >,
    pub tcl_GetEncodingSearchPath: ::std::option::Option<unsafe extern "C" fn() -> *mut Tcl_Obj>,
    pub tcl_SetEncodingSearchPath: ::std::option::Option<
        unsafe extern "C" fn(searchPath: *mut Tcl_Obj) -> ::std::os::raw::c_int,
    >,
    pub tcl_GetEncodingNameFromEnvironment: ::std::option::Option<
        unsafe extern "C" fn(bufPtr: *mut Tcl_DString) -> *const ::std::os::raw::c_char,
    >,
    pub tcl_PkgRequireProc: ::std::option::Option<
        unsafe extern "C" fn(
            interp: *mut Tcl_Interp,
            name: *const ::std::os::raw::c_char,
            objc: ::std::os::raw::c_int,
            objv: *const *mut Tcl_Obj,
            clientDataPtr: *mut ClientData,
        ) -> ::std::os::raw::c_int,
    >,
    pub tcl_AppendObjToErrorInfo:
        ::std::option::Option<unsafe extern "C" fn(interp: *mut Tcl_Interp, objPtr: *mut Tcl_Obj)>,
    pub tcl_AppendLimitedToObj: ::std::option::Option<
        unsafe extern "C" fn(
            objPtr: *mut Tcl_Obj,
            bytes: *const ::std::os::raw::c_char,
            length: ::std::os::raw::c_int,
            limit: ::std::os::raw::c_int,
            ellipsis: *const ::std::os::raw::c_char,
        ),
    >,
    pub tcl_Format: ::std::option::Option<
        unsafe extern "C" fn(
            interp: *mut Tcl_Interp,
            format: *const ::std::os::raw::c_char,
            objc: ::std::os::raw::c_int,
            objv: *const *mut Tcl_Obj,
        ) -> *mut Tcl_Obj,
    >,
    pub tcl_AppendFormatToObj: ::std::option::Option<
        unsafe extern "C" fn(
            interp: *mut Tcl_Interp,
            objPtr: *mut Tcl_Obj,
            format: *const ::std::os::raw::c_char,
            objc: ::std::os::raw::c_int,
            objv: *const *mut Tcl_Obj,
        ) -> ::std::os::raw::c_int,
    >,
    pub tcl_ObjPrintf: ::std::option::Option<
        unsafe extern "C" fn(format: *const ::std::os::raw::c_char, ...) -> *mut Tcl_Obj,
    >,
    pub tcl_AppendPrintfToObj: ::std::option::Option<
        unsafe extern "C" fn(objPtr: *mut Tcl_Obj, format: *const ::std::os::raw::c_char, ...),
    >,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct TclPlatStubs {
    pub magic: ::std::os::raw::c_int,
    pub hooks: *mut TclPlatStubHooks,
}
pub type ulong64 = ::std::os::raw::c_ulonglong;
pub type long64 = ::std::os::raw::c_longlong;
pub type mp_word = ulong64;
pub type mp_err = ::std::os::raw::c_int;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct mp_int {
    pub used: ::std::os::raw::c_int,
    pub alloc: ::std::os::raw::c_int,
    pub sign: ::std::os::raw::c_int,
    pub dp: *mut mp_digit,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct TclIntStubs {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct TclIntPlatStubs {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct TclPlatStubHooks {
    pub _address: u8,
}
unsafe extern "C" {
    pub fn Tcl_InitStubs(
        interp: *mut Tcl_Interp,
        version: *const ::std::os::raw::c_char,
        exact: ::std::os::raw::c_int,
    ) -> *const ::std::os::raw::c_char;
}
unsafe extern "C" {
    pub fn TclTomMathInitializeStubs(
        interp: *mut Tcl_Interp,
        version: *const ::std::os::raw::c_char,
        epoch: ::std::os::raw::c_int,
        revision: ::std::os::raw::c_int,
    ) -> *const ::std::os::raw::c_char;
}
unsafe extern "C" {
    pub fn Tcl_Main(
        argc: ::std::os::raw::c_int,
        argv: *mut *mut ::std::os::raw::c_char,
        appInitProc: Tcl_AppInitProc,
    );
}
unsafe extern "C" {
    pub fn Tcl_PkgInitStubsCheck(
        interp: *mut Tcl_Interp,
        version: *const ::std::os::raw::c_char,
        exact: ::std::os::raw::c_int,
    ) -> *const ::std::os::raw::c_char;
}
unsafe extern "C" {
    pub fn Tcl_PkgProvideEx(
        interp: *mut Tcl_Interp,
        name: *const ::std::os::raw::c_char,
        version: *const ::std::os::raw::c_char,
        clientData: ClientData,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn Tcl_PkgRequireEx(
        interp: *mut Tcl_Interp,
        name: *const ::std::os::raw::c_char,
        version: *const ::std::os::raw::c_char,
        exact: ::std::os::raw::c_int,
        clientDataPtr: *mut ClientData,
    ) -> *const ::std::os::raw::c_char;
}
unsafe extern "C" {
    pub fn Tcl_Panic(format: *const ::std::os::raw::c_char, ...);
}
unsafe extern "C" {
    pub fn Tcl_Alloc(size: ::std::os::raw::c_uint) -> *mut ::std::os::raw::c_char;
}
unsafe extern "C" {
    pub fn Tcl_Free(ptr: *mut ::std::os::raw::c_char);
}
unsafe extern "C" {
    pub fn Tcl_Realloc(
        ptr: *mut ::std::os::raw::c_char,
        size: ::std::os::raw::c_uint,
    ) -> *mut ::std::os::raw::c_char;
}
unsafe extern "C" {
    pub fn Tcl_DbCkalloc(
        size: ::std::os::raw::c_uint,
        file: *const ::std::os::raw::c_char,
        line: ::std::os::raw::c_int,
    ) -> *mut ::std::os::raw::c_char;
}
unsafe extern "C" {
    pub fn Tcl_DbCkfree(
        ptr: *mut ::std::os::raw::c_char,
        file: *const ::std::os::raw::c_char,
        line: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn Tcl_DbCkrealloc(
        ptr: *mut ::std::os::raw::c_char,
        size: ::std::os::raw::c_uint,
        file: *const ::std::os::raw::c_char,
        line: ::std::os::raw::c_int,
    ) -> *mut ::std::os::raw::c_char;
}
unsafe extern "C" {
    pub fn Tcl_CreateFileHandler(
        fd: ::std::os::raw::c_int,
        mask: ::std::os::raw::c_int,
        proc_: Tcl_FileProc,
        clientData: ClientData,
    );
}
unsafe extern "C" {
    pub fn Tcl_DeleteFileHandler(fd: ::std::os::raw::c_int);
}
unsafe extern "C" {
    pub fn Tcl_SetTimer(timePtr: *mut Tcl_Time);
}
unsafe extern "C" {
    pub fn Tcl_Sleep(ms: ::std::os::raw::c_int);
}
unsafe extern "C" {
    pub fn Tcl_WaitForEvent(timePtr: *mut Tcl_Time) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn Tcl_AppendAllObjTypes(
        interp: *mut Tcl_Interp,
        objPtr: *mut Tcl_Obj,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn Tcl_AppendStringsToObj(objPtr: *mut Tcl_Obj, ...);
}
unsafe extern "C" {
    pub fn Tcl_AppendToObj(
        objPtr: *mut Tcl_Obj,
        bytes: *const ::std::os::raw::c_char,
        length: ::std::os::raw::c_int,
    );
}
unsafe extern "C" {
    pub fn Tcl_ConcatObj(objc: ::std::os::raw::c_int, objv: *const *mut Tcl_Obj) -> *mut Tcl_Obj;
}
unsafe extern "C" {
    pub fn Tcl_ConvertToType(
        interp: *mut Tcl_Interp,
        objPtr: *mut Tcl_Obj,
        typePtr: *mut Tcl_ObjType,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn Tcl_DbDecrRefCount(
        objPtr: *mut Tcl_Obj,
        file: *const ::std::os::raw::c_char,
        line: ::std::os::raw::c_int,
    );
}
unsafe extern "C" {
    pub fn Tcl_DbIncrRefCount(
        objPtr: *mut Tcl_Obj,
        file: *const ::std::os::raw::c_char,
        line: ::std::os::raw::c_int,
    );
}
unsafe extern "C" {
    pub fn Tcl_DbIsShared(
        objPtr: *mut Tcl_Obj,
        file: *const ::std::os::raw::c_char,
        line: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn Tcl_DbNewBooleanObj(
        boolValue: ::std::os::raw::c_int,
        file: *const ::std::os::raw::c_char,
        line: ::std::os::raw::c_int,
    ) -> *mut Tcl_Obj;
}
unsafe extern "C" {
    pub fn Tcl_DbNewByteArrayObj(
        bytes: *const ::std::os::raw::c_uchar,
        length: ::std::os::raw::c_int,
        file: *const ::std::os::raw::c_char,
        line: ::std::os::raw::c_int,
    ) -> *mut Tcl_Obj;
}
unsafe extern "C" {
    pub fn Tcl_DbNewDoubleObj(
        doubleValue: f64,
        file: *const ::std::os::raw::c_char,
        line: ::std::os::raw::c_int,
    ) -> *mut Tcl_Obj;
}
unsafe extern "C" {
    pub fn Tcl_DbNewListObj(
        objc: ::std::os::raw::c_int,
        objv: *const *mut Tcl_Obj,
        file: *const ::std::os::raw::c_char,
        line: ::std::os::raw::c_int,
    ) -> *mut Tcl_Obj;
}
unsafe extern "C" {
    pub fn Tcl_DbNewLongObj(
        longValue: ::std::os::raw::c_long,
        file: *const ::std::os::raw::c_char,
        line: ::std::os::raw::c_int,
    ) -> *mut Tcl_Obj;
}
unsafe extern "C" {
    pub fn Tcl_DbNewObj(
        file: *const ::std::os::raw::c_char,
        line: ::std::os::raw::c_int,
    ) -> *mut Tcl_Obj;
}
unsafe extern "C" {
    pub fn Tcl_DbNewStringObj(
        bytes: *const ::std::os::raw::c_char,
        length: ::std::os::raw::c_int,
        file: *const ::std::os::raw::c_char,
        line: ::std::os::raw::c_int,
    ) -> *mut Tcl_Obj;
}
unsafe extern "C" {
    pub fn Tcl_DuplicateObj(objPtr: *mut Tcl_Obj) -> *mut Tcl_Obj;
}
unsafe extern "C" {
    pub fn TclFreeObj(objPtr: *mut Tcl_Obj);
}
unsafe extern "C" {
    pub fn Tcl_GetBoolean(
        interp: *mut Tcl_Interp,
        src: *const ::std::os::raw::c_char,
        boolPtr: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn Tcl_GetBooleanFromObj(
        interp: *mut Tcl_Interp,
        objPtr: *mut Tcl_Obj,
        boolPtr: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn Tcl_GetByteArrayFromObj(
        objPtr: *mut Tcl_Obj,
        lengthPtr: *mut ::std::os::raw::c_int,
    ) -> *mut ::std::os::raw::c_uchar;
}
unsafe extern "C" {
    pub fn Tcl_GetDouble(
        interp: *mut Tcl_Interp,
        src: *const ::std::os::raw::c_char,
        doublePtr: *mut f64,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn Tcl_GetDoubleFromObj(
        interp: *mut Tcl_Interp,
        objPtr: *mut Tcl_Obj,
        doublePtr: *mut f64,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn Tcl_GetIndexFromObj(
        interp: *mut Tcl_Interp,
        objPtr: *mut Tcl_Obj,
        tablePtr: *mut *const ::std::os::raw::c_char,
        msg: *const ::std::os::raw::c_char,
        flags: ::std::os::raw::c_int,
        indexPtr: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn Tcl_GetInt(
        interp: *mut Tcl_Interp,
        src: *const ::std::os::raw::c_char,
        intPtr: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn Tcl_GetIntFromObj(
        interp: *mut Tcl_Interp,
        objPtr: *mut Tcl_Obj,
        intPtr: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn Tcl_GetLongFromObj(
        interp: *mut Tcl_Interp,
        objPtr: *mut Tcl_Obj,
        longPtr: *mut ::std::os::raw::c_long,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn Tcl_GetObjType(typeName: *const ::std::os::raw::c_char) -> *mut Tcl_ObjType;
}
unsafe extern "C" {
    pub fn Tcl_GetStringFromObj(
        objPtr: *mut Tcl_Obj,
        lengthPtr: *mut ::std::os::raw::c_int,
    ) -> *mut ::std::os::raw::c_char;
}
unsafe extern "C" {
    pub fn Tcl_InvalidateStringRep(objPtr: *mut Tcl_Obj);
}
unsafe extern "C" {
    pub fn Tcl_ListObjAppendList(
        interp: *mut Tcl_Interp,
        listPtr: *mut Tcl_Obj,
        elemListPtr: *mut Tcl_Obj,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn Tcl_ListObjAppendElement(
        interp: *mut Tcl_Interp,
        listPtr: *mut Tcl_Obj,
        objPtr: *mut Tcl_Obj,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn Tcl_ListObjGetElements(
        interp: *mut Tcl_Interp,
        listPtr: *mut Tcl_Obj,
        objcPtr: *mut ::std::os::raw::c_int,
        objvPtr: *mut *mut *mut Tcl_Obj,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn Tcl_ListObjIndex(
        interp: *mut Tcl_Interp,
        listPtr: *mut Tcl_Obj,
        index: ::std::os::raw::c_int,
        objPtrPtr: *mut *mut Tcl_Obj,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn Tcl_ListObjLength(
        interp: *mut Tcl_Interp,
        listPtr: *mut Tcl_Obj,
        lengthPtr: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn Tcl_ListObjReplace(
        interp: *mut Tcl_Interp,
        listPtr: *mut Tcl_Obj,
        first: ::std::os::raw::c_int,
        count: ::std::os::raw::c_int,
        objc: ::std::os::raw::c_int,
        objv: *const *mut Tcl_Obj,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn Tcl_NewBooleanObj(boolValue: ::std::os::raw::c_int) -> *mut Tcl_Obj;
}
unsafe extern "C" {
    pub fn Tcl_NewByteArrayObj(
        bytes: *const ::std::os::raw::c_uchar,
        length: ::std::os::raw::c_int,
    ) -> *mut Tcl_Obj;
}
unsafe extern "C" {
    pub fn Tcl_NewDoubleObj(doubleValue: f64) -> *mut Tcl_Obj;
}
unsafe extern "C" {
    pub fn Tcl_NewIntObj(intValue: ::std::os::raw::c_int) -> *mut Tcl_Obj;
}
unsafe extern "C" {
    pub fn Tcl_NewListObj(objc: ::std::os::raw::c_int, objv: *const *mut Tcl_Obj) -> *mut Tcl_Obj;
}
unsafe extern "C" {
    pub fn Tcl_NewLongObj(longValue: ::std::os::raw::c_long) -> *mut Tcl_Obj;
}
unsafe extern "C" {
    pub fn Tcl_NewObj() -> *mut Tcl_Obj;
}
unsafe extern "C" {
    pub fn Tcl_NewStringObj(
        bytes: *const ::std::os::raw::c_char,
        length: ::std::os::raw::c_int,
    ) -> *mut Tcl_Obj;
}
unsafe extern "C" {
    pub fn Tcl_SetBooleanObj(objPtr: *mut Tcl_Obj, boolValue: ::std::os::raw::c_int);
}
unsafe extern "C" {
    pub fn Tcl_SetByteArrayLength(
        objPtr: *mut Tcl_Obj,
        length: ::std::os::raw::c_int,
    ) -> *mut ::std::os::raw::c_uchar;
}
unsafe extern "C" {
    pub fn Tcl_SetByteArrayObj(
        objPtr: *mut Tcl_Obj,
        bytes: *const ::std::os::raw::c_uchar,
        length: ::std::os::raw::c_int,
    );
}
unsafe extern "C" {
    pub fn Tcl_SetDoubleObj(objPtr: *mut Tcl_Obj, doubleValue: f64);
}
unsafe extern "C" {
    pub fn Tcl_SetIntObj(objPtr: *mut Tcl_Obj, intValue: ::std::os::raw::c_int);
}
unsafe extern "C" {
    pub fn Tcl_SetListObj(
        objPtr: *mut Tcl_Obj,
        objc: ::std::os::raw::c_int,
        objv: *const *mut Tcl_Obj,
    );
}
unsafe extern "C" {
    pub fn Tcl_SetLongObj(objPtr: *mut Tcl_Obj, longValue: ::std::os::raw::c_long);
}
unsafe extern "C" {
    pub fn Tcl_SetObjLength(objPtr: *mut Tcl_Obj, length: ::std::os::raw::c_int);
}
unsafe extern "C" {
    pub fn Tcl_SetStringObj(
        objPtr: *mut Tcl_Obj,
        bytes: *const ::std::os::raw::c_char,
        length: ::std::os::raw::c_int,
    );
}
unsafe extern "C" {
    pub fn Tcl_AddErrorInfo(interp: *mut Tcl_Interp, message: *const ::std::os::raw::c_char);
}
unsafe extern "C" {
    pub fn Tcl_AddObjErrorInfo(
        interp: *mut Tcl_Interp,
        message: *const ::std::os::raw::c_char,
        length: ::std::os::raw::c_int,
    );
}
unsafe extern "C" {
    pub fn Tcl_AllowExceptions(interp: *mut Tcl_Interp);
}
unsafe extern "C" {
    pub fn Tcl_AppendElement(interp: *mut Tcl_Interp, element: *const ::std::os::raw::c_char);
}
unsafe extern "C" {
    pub fn Tcl_AppendResult(interp: *mut Tcl_Interp, ...);
}
unsafe extern "C" {
    pub fn Tcl_AsyncCreate(proc_: Tcl_AsyncProc, clientData: ClientData) -> Tcl_AsyncHandler;
}
unsafe extern "C" {
    pub fn Tcl_AsyncDelete(async_: Tcl_AsyncHandler);
}
unsafe extern "C" {
    pub fn Tcl_AsyncInvoke(
        interp: *mut Tcl_Interp,
        code: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn Tcl_AsyncMark(async_: Tcl_AsyncHandler);
}
unsafe extern "C" {
    pub fn Tcl_AsyncReady() -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn Tcl_BackgroundError(interp: *mut Tcl_Interp);
}
unsafe extern "C" {
    pub fn Tcl_Backslash(
        src: *const ::std::os::raw::c_char,
        readPtr: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_char;
}
unsafe extern "C" {
    pub fn Tcl_BadChannelOption(
        interp: *mut Tcl_Interp,
        optionName: *const ::std::os::raw::c_char,
        optionList: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn Tcl_CallWhenDeleted(
        interp: *mut Tcl_Interp,
        proc_: Tcl_InterpDeleteProc,
        clientData: ClientData,
    );
}
unsafe extern "C" {
    pub fn Tcl_CancelIdleCall(idleProc: Tcl_IdleProc, clientData: ClientData);
}
unsafe extern "C" {
    pub fn Tcl_Close(interp: *mut Tcl_Interp, chan: Tcl_Channel) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn Tcl_CommandComplete(cmd: *const ::std::os::raw::c_char) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn Tcl_Concat(
        argc: ::std::os::raw::c_int,
        argv: *const *const ::std::os::raw::c_char,
    ) -> *mut ::std::os::raw::c_char;
}
unsafe extern "C" {
    pub fn Tcl_ConvertElement(
        src: *const ::std::os::raw::c_char,
        dst: *mut ::std::os::raw::c_char,
        flags: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn Tcl_ConvertCountedElement(
        src: *const ::std::os::raw::c_char,
        length: ::std::os::raw::c_int,
        dst: *mut ::std::os::raw::c_char,
        flags: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn Tcl_CreateAlias(
        slave: *mut Tcl_Interp,
        slaveCmd: *const ::std::os::raw::c_char,
        target: *mut Tcl_Interp,
        targetCmd: *const ::std::os::raw::c_char,
        argc: ::std::os::raw::c_int,
        argv: *const *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn Tcl_CreateAliasObj(
        slave: *mut Tcl_Interp,
        slaveCmd: *const ::std::os::raw::c_char,
        target: *mut Tcl_Interp,
        targetCmd: *const ::std::os::raw::c_char,
        objc: ::std::os::raw::c_int,
        objv: *const *mut Tcl_Obj,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn Tcl_CreateChannel(
        typePtr: *mut Tcl_ChannelType,
        chanName: *const ::std::os::raw::c_char,
        instanceData: ClientData,
        mask: ::std::os::raw::c_int,
    ) -> Tcl_Channel;
}
unsafe extern "C" {
    pub fn Tcl_CreateChannelHandler(
        chan: Tcl_Channel,
        mask: ::std::os::raw::c_int,
        proc_: Tcl_ChannelProc,
        clientData: ClientData,
    );
}
unsafe extern "C" {
    pub fn Tcl_CreateCloseHandler(chan: Tcl_Channel, proc_: Tcl_CloseProc, clientData: ClientData);
}
unsafe extern "C" {
    pub fn Tcl_CreateCommand(
        interp: *mut Tcl_Interp,
        cmdName: *const ::std::os::raw::c_char,
        proc_: Tcl_CmdProc,
        clientData: ClientData,
        deleteProc: Tcl_CmdDeleteProc,
    ) -> Tcl_Command;
}
unsafe extern "C" {
    pub fn Tcl_CreateEventSource(
        setupProc: Tcl_EventSetupProc,
        checkProc: Tcl_EventCheckProc,
        clientData: ClientData,
    );
}
unsafe extern "C" {
    pub fn Tcl_CreateExitHandler(proc_: Tcl_ExitProc, clientData: ClientData);
}
unsafe extern "C" {
    pub fn Tcl_CreateInterp() -> *mut Tcl_Interp;
}
unsafe extern "C" {
    pub fn Tcl_CreateMathFunc(
        interp: *mut Tcl_Interp,
        name: *const ::std::os::raw::c_char,
        numArgs: ::std::os::raw::c_int,
        argTypes: *mut Tcl_ValueType,
        proc_: Tcl_MathProc,
        clientData: ClientData,
    );
}
unsafe extern "C" {
    pub fn Tcl_CreateObjCommand(
        interp: *mut Tcl_Interp,
        cmdName: *const ::std::os::raw::c_char,
        proc_: Tcl_ObjCmdProc,
        clientData: ClientData,
        deleteProc: Tcl_CmdDeleteProc,
    ) -> Tcl_Command;
}
unsafe extern "C" {
    pub fn Tcl_CreateSlave(
        interp: *mut Tcl_Interp,
        slaveName: *const ::std::os::raw::c_char,
        isSafe: ::std::os::raw::c_int,
    ) -> *mut Tcl_Interp;
}
unsafe extern "C" {
    pub fn Tcl_CreateTimerHandler(
        milliseconds: ::std::os::raw::c_int,
        proc_: Tcl_TimerProc,
        clientData: ClientData,
    ) -> Tcl_TimerToken;
}
unsafe extern "C" {
    pub fn Tcl_CreateTrace(
        interp: *mut Tcl_Interp,
        level: ::std::os::raw::c_int,
        proc_: Tcl_CmdTraceProc,
        clientData: ClientData,
    ) -> Tcl_Trace;
}
unsafe extern "C" {
    pub fn Tcl_DeleteAssocData(interp: *mut Tcl_Interp, name: *const ::std::os::raw::c_char);
}
unsafe extern "C" {
    pub fn Tcl_DeleteChannelHandler(
        chan: Tcl_Channel,
        proc_: Tcl_ChannelProc,
        clientData: ClientData,
    );
}
unsafe extern "C" {
    pub fn Tcl_DeleteCloseHandler(chan: Tcl_Channel, proc_: Tcl_CloseProc, clientData: ClientData);
}
unsafe extern "C" {
    pub fn Tcl_DeleteCommand(
        interp: *mut Tcl_Interp,
        cmdName: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn Tcl_DeleteCommandFromToken(
        interp: *mut Tcl_Interp,
        command: Tcl_Command,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn Tcl_DeleteEvents(proc_: Tcl_EventDeleteProc, clientData: ClientData);
}
unsafe extern "C" {
    pub fn Tcl_DeleteEventSource(
        setupProc: Tcl_EventSetupProc,
        checkProc: Tcl_EventCheckProc,
        clientData: ClientData,
    );
}
unsafe extern "C" {
    pub fn Tcl_DeleteExitHandler(proc_: Tcl_ExitProc, clientData: ClientData);
}
unsafe extern "C" {
    pub fn Tcl_DeleteHashEntry(entryPtr: *mut Tcl_HashEntry);
}
unsafe extern "C" {
    pub fn Tcl_DeleteHashTable(tablePtr: *mut Tcl_HashTable);
}
unsafe extern "C" {
    pub fn Tcl_DeleteInterp(interp: *mut Tcl_Interp);
}
unsafe extern "C" {
    pub fn Tcl_DetachPids(numPids: ::std::os::raw::c_int, pidPtr: *mut Tcl_Pid);
}
unsafe extern "C" {
    pub fn Tcl_DeleteTimerHandler(token: Tcl_TimerToken);
}
unsafe extern "C" {
    pub fn Tcl_DeleteTrace(interp: *mut Tcl_Interp, trace: Tcl_Trace);
}
unsafe extern "C" {
    pub fn Tcl_DontCallWhenDeleted(
        interp: *mut Tcl_Interp,
        proc_: Tcl_InterpDeleteProc,
        clientData: ClientData,
    );
}
unsafe extern "C" {
    pub fn Tcl_DoOneEvent(flags: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn Tcl_DoWhenIdle(proc_: Tcl_IdleProc, clientData: ClientData);
}
unsafe extern "C" {
    pub fn Tcl_DStringAppend(
        dsPtr: *mut Tcl_DString,
        bytes: *const ::std::os::raw::c_char,
        length: ::std::os::raw::c_int,
    ) -> *mut ::std::os::raw::c_char;
}
unsafe extern "C" {
    pub fn Tcl_DStringAppendElement(
        dsPtr: *mut Tcl_DString,
        element: *const ::std::os::raw::c_char,
    ) -> *mut ::std::os::raw::c_char;
}
unsafe extern "C" {
    pub fn Tcl_DStringEndSublist(dsPtr: *mut Tcl_DString);
}
unsafe extern "C" {
    pub fn Tcl_DStringFree(dsPtr: *mut Tcl_DString);
}
unsafe extern "C" {
    pub fn Tcl_DStringGetResult(interp: *mut Tcl_Interp, dsPtr: *mut Tcl_DString);
}
unsafe extern "C" {
    pub fn Tcl_DStringInit(dsPtr: *mut Tcl_DString);
}
unsafe extern "C" {
    pub fn Tcl_DStringResult(interp: *mut Tcl_Interp, dsPtr: *mut Tcl_DString);
}
unsafe extern "C" {
    pub fn Tcl_DStringSetLength(dsPtr: *mut Tcl_DString, length: ::std::os::raw::c_int);
}
unsafe extern "C" {
    pub fn Tcl_DStringStartSublist(dsPtr: *mut Tcl_DString);
}
unsafe extern "C" {
    pub fn Tcl_Eof(chan: Tcl_Channel) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn Tcl_ErrnoId() -> *const ::std::os::raw::c_char;
}
unsafe extern "C" {
    pub fn Tcl_ErrnoMsg(err: ::std::os::raw::c_int) -> *const ::std::os::raw::c_char;
}
unsafe extern "C" {
    pub fn Tcl_Eval(
        interp: *mut Tcl_Interp,
        script: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn Tcl_EvalFile(
        interp: *mut Tcl_Interp,
        fileName: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn Tcl_EvalObj(interp: *mut Tcl_Interp, objPtr: *mut Tcl_Obj) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn Tcl_EventuallyFree(clientData: ClientData, freeProc: Tcl_FreeProc);
}
unsafe extern "C" {
    pub fn Tcl_Exit(status: ::std::os::raw::c_int);
}
unsafe extern "C" {
    pub fn Tcl_ExposeCommand(
        interp: *mut Tcl_Interp,
        hiddenCmdToken: *const ::std::os::raw::c_char,
        cmdName: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn Tcl_ExprBoolean(
        interp: *mut Tcl_Interp,
        expr: *const ::std::os::raw::c_char,
        ptr: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn Tcl_ExprBooleanObj(
        interp: *mut Tcl_Interp,
        objPtr: *mut Tcl_Obj,
        ptr: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn Tcl_ExprDouble(
        interp: *mut Tcl_Interp,
        expr: *const ::std::os::raw::c_char,
        ptr: *mut f64,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn Tcl_ExprDoubleObj(
        interp: *mut Tcl_Interp,
        objPtr: *mut Tcl_Obj,
        ptr: *mut f64,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn Tcl_ExprLong(
        interp: *mut Tcl_Interp,
        expr: *const ::std::os::raw::c_char,
        ptr: *mut ::std::os::raw::c_long,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn Tcl_ExprLongObj(
        interp: *mut Tcl_Interp,
        objPtr: *mut Tcl_Obj,
        ptr: *mut ::std::os::raw::c_long,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn Tcl_ExprObj(
        interp: *mut Tcl_Interp,
        objPtr: *mut Tcl_Obj,
        resultPtrPtr: *mut *mut Tcl_Obj,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn Tcl_ExprString(
        interp: *mut Tcl_Interp,
        expr: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn Tcl_Finalize();
}
unsafe extern "C" {
    pub fn Tcl_FindExecutable(argv0: *const ::std::os::raw::c_char);
}
unsafe extern "C" {
    pub fn Tcl_FirstHashEntry(
        tablePtr: *mut Tcl_HashTable,
        searchPtr: *mut Tcl_HashSearch,
    ) -> *mut Tcl_HashEntry;
}
unsafe extern "C" {
    pub fn Tcl_Flush(chan: Tcl_Channel) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn Tcl_FreeResult(interp: *mut Tcl_Interp);
}
unsafe extern "C" {
    pub fn Tcl_GetAlias(
        interp: *mut Tcl_Interp,
        slaveCmd: *const ::std::os::raw::c_char,
        targetInterpPtr: *mut *mut Tcl_Interp,
        targetCmdPtr: *mut *const ::std::os::raw::c_char,
        argcPtr: *mut ::std::os::raw::c_int,
        argvPtr: *mut *mut *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn Tcl_GetAliasObj(
        interp: *mut Tcl_Interp,
        slaveCmd: *const ::std::os::raw::c_char,
        targetInterpPtr: *mut *mut Tcl_Interp,
        targetCmdPtr: *mut *const ::std::os::raw::c_char,
        objcPtr: *mut ::std::os::raw::c_int,
        objv: *mut *mut *mut Tcl_Obj,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn Tcl_GetAssocData(
        interp: *mut Tcl_Interp,
        name: *const ::std::os::raw::c_char,
        procPtr: *mut Tcl_InterpDeleteProc,
    ) -> ClientData;
}
unsafe extern "C" {
    pub fn Tcl_GetChannel(
        interp: *mut Tcl_Interp,
        chanName: *const ::std::os::raw::c_char,
        modePtr: *mut ::std::os::raw::c_int,
    ) -> Tcl_Channel;
}
unsafe extern "C" {
    pub fn Tcl_GetChannelBufferSize(chan: Tcl_Channel) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn Tcl_GetChannelHandle(
        chan: Tcl_Channel,
        direction: ::std::os::raw::c_int,
        handlePtr: *mut ClientData,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn Tcl_GetChannelInstanceData(chan: Tcl_Channel) -> ClientData;
}
unsafe extern "C" {
    pub fn Tcl_GetChannelMode(chan: Tcl_Channel) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn Tcl_GetChannelName(chan: Tcl_Channel) -> *const ::std::os::raw::c_char;
}
unsafe extern "C" {
    pub fn Tcl_GetChannelOption(
        interp: *mut Tcl_Interp,
        chan: Tcl_Channel,
        optionName: *const ::std::os::raw::c_char,
        dsPtr: *mut Tcl_DString,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn Tcl_GetChannelType(chan: Tcl_Channel) -> *mut Tcl_ChannelType;
}
unsafe extern "C" {
    pub fn Tcl_GetCommandInfo(
        interp: *mut Tcl_Interp,
        cmdName: *const ::std::os::raw::c_char,
        infoPtr: *mut Tcl_CmdInfo,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn Tcl_GetCommandName(
        interp: *mut Tcl_Interp,
        command: Tcl_Command,
    ) -> *const ::std::os::raw::c_char;
}
unsafe extern "C" {
    pub fn Tcl_GetErrno() -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn Tcl_GetHostName() -> *const ::std::os::raw::c_char;
}
unsafe extern "C" {
    pub fn Tcl_GetInterpPath(
        askInterp: *mut Tcl_Interp,
        slaveInterp: *mut Tcl_Interp,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn Tcl_GetMaster(interp: *mut Tcl_Interp) -> *mut Tcl_Interp;
}
unsafe extern "C" {
    pub fn Tcl_GetNameOfExecutable() -> *const ::std::os::raw::c_char;
}
unsafe extern "C" {
    pub fn Tcl_GetObjResult(interp: *mut Tcl_Interp) -> *mut Tcl_Obj;
}
unsafe extern "C" {
    pub fn Tcl_GetOpenFile(
        interp: *mut Tcl_Interp,
        chanID: *const ::std::os::raw::c_char,
        forWriting: ::std::os::raw::c_int,
        checkUsage: ::std::os::raw::c_int,
        filePtr: *mut ClientData,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn Tcl_GetPathType(path: *const ::std::os::raw::c_char) -> Tcl_PathType;
}
unsafe extern "C" {
    pub fn Tcl_Gets(chan: Tcl_Channel, dsPtr: *mut Tcl_DString) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn Tcl_GetsObj(chan: Tcl_Channel, objPtr: *mut Tcl_Obj) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn Tcl_GetServiceMode() -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn Tcl_GetSlave(
        interp: *mut Tcl_Interp,
        slaveName: *const ::std::os::raw::c_char,
    ) -> *mut Tcl_Interp;
}
unsafe extern "C" {
    pub fn Tcl_GetStdChannel(type_: ::std::os::raw::c_int) -> Tcl_Channel;
}
unsafe extern "C" {
    pub fn Tcl_GetStringResult(interp: *mut Tcl_Interp) -> *const ::std::os::raw::c_char;
}
unsafe extern "C" {
    pub fn Tcl_GetVar(
        interp: *mut Tcl_Interp,
        varName: *const ::std::os::raw::c_char,
        flags: ::std::os::raw::c_int,
    ) -> *const ::std::os::raw::c_char;
}
unsafe extern "C" {
    pub fn Tcl_GetVar2(
        interp: *mut Tcl_Interp,
        part1: *const ::std::os::raw::c_char,
        part2: *const ::std::os::raw::c_char,
        flags: ::std::os::raw::c_int,
    ) -> *const ::std::os::raw::c_char;
}
unsafe extern "C" {
    pub fn Tcl_GlobalEval(
        interp: *mut Tcl_Interp,
        command: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn Tcl_GlobalEvalObj(
        interp: *mut Tcl_Interp,
        objPtr: *mut Tcl_Obj,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn Tcl_HideCommand(
        interp: *mut Tcl_Interp,
        cmdName: *const ::std::os::raw::c_char,
        hiddenCmdToken: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn Tcl_Init(interp: *mut Tcl_Interp) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn Tcl_InitHashTable(tablePtr: *mut Tcl_HashTable, keyType: ::std::os::raw::c_int);
}
unsafe extern "C" {
    pub fn Tcl_InputBlocked(chan: Tcl_Channel) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn Tcl_InputBuffered(chan: Tcl_Channel) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn Tcl_InterpDeleted(interp: *mut Tcl_Interp) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn Tcl_IsSafe(interp: *mut Tcl_Interp) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn Tcl_JoinPath(
        argc: ::std::os::raw::c_int,
        argv: *const *const ::std::os::raw::c_char,
        resultPtr: *mut Tcl_DString,
    ) -> *mut ::std::os::raw::c_char;
}
unsafe extern "C" {
    pub fn Tcl_LinkVar(
        interp: *mut Tcl_Interp,
        varName: *const ::std::os::raw::c_char,
        addr: *mut ::std::os::raw::c_char,
        type_: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn Tcl_MakeFileChannel(handle: ClientData, mode: ::std::os::raw::c_int) -> Tcl_Channel;
}
unsafe extern "C" {
    pub fn Tcl_MakeSafe(interp: *mut Tcl_Interp) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn Tcl_MakeTcpClientChannel(tcpSocket: ClientData) -> Tcl_Channel;
}
unsafe extern "C" {
    pub fn Tcl_Merge(
        argc: ::std::os::raw::c_int,
        argv: *const *const ::std::os::raw::c_char,
    ) -> *mut ::std::os::raw::c_char;
}
unsafe extern "C" {
    pub fn Tcl_NextHashEntry(searchPtr: *mut Tcl_HashSearch) -> *mut Tcl_HashEntry;
}
unsafe extern "C" {
    pub fn Tcl_NotifyChannel(channel: Tcl_Channel, mask: ::std::os::raw::c_int);
}
unsafe extern "C" {
    pub fn Tcl_ObjGetVar2(
        interp: *mut Tcl_Interp,
        part1Ptr: *mut Tcl_Obj,
        part2Ptr: *mut Tcl_Obj,
        flags: ::std::os::raw::c_int,
    ) -> *mut Tcl_Obj;
}
unsafe extern "C" {
    pub fn Tcl_ObjSetVar2(
        interp: *mut Tcl_Interp,
        part1Ptr: *mut Tcl_Obj,
        part2Ptr: *mut Tcl_Obj,
        newValuePtr: *mut Tcl_Obj,
        flags: ::std::os::raw::c_int,
    ) -> *mut Tcl_Obj;
}
unsafe extern "C" {
    pub fn Tcl_OpenCommandChannel(
        interp: *mut Tcl_Interp,
        argc: ::std::os::raw::c_int,
        argv: *mut *const ::std::os::raw::c_char,
        flags: ::std::os::raw::c_int,
    ) -> Tcl_Channel;
}
unsafe extern "C" {
    pub fn Tcl_OpenFileChannel(
        interp: *mut Tcl_Interp,
        fileName: *const ::std::os::raw::c_char,
        modeString: *const ::std::os::raw::c_char,
        permissions: ::std::os::raw::c_int,
    ) -> Tcl_Channel;
}
unsafe extern "C" {
    pub fn Tcl_OpenTcpClient(
        interp: *mut Tcl_Interp,
        port: ::std::os::raw::c_int,
        address: *const ::std::os::raw::c_char,
        myaddr: *const ::std::os::raw::c_char,
        myport: ::std::os::raw::c_int,
        async_: ::std::os::raw::c_int,
    ) -> Tcl_Channel;
}
unsafe extern "C" {
    pub fn Tcl_OpenTcpServer(
        interp: *mut Tcl_Interp,
        port: ::std::os::raw::c_int,
        host: *const ::std::os::raw::c_char,
        acceptProc: Tcl_TcpAcceptProc,
        callbackData: ClientData,
    ) -> Tcl_Channel;
}
unsafe extern "C" {
    pub fn Tcl_Preserve(data: ClientData);
}
unsafe extern "C" {
    pub fn Tcl_PrintDouble(interp: *mut Tcl_Interp, value: f64, dst: *mut ::std::os::raw::c_char);
}
unsafe extern "C" {
    pub fn Tcl_PutEnv(assignment: *const ::std::os::raw::c_char) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn Tcl_PosixError(interp: *mut Tcl_Interp) -> *const ::std::os::raw::c_char;
}
unsafe extern "C" {
    pub fn Tcl_QueueEvent(evPtr: *mut Tcl_Event, position: Tcl_QueuePosition);
}
unsafe extern "C" {
    pub fn Tcl_Read(
        chan: Tcl_Channel,
        bufPtr: *mut ::std::os::raw::c_char,
        toRead: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn Tcl_ReapDetachedProcs();
}
unsafe extern "C" {
    pub fn Tcl_RecordAndEval(
        interp: *mut Tcl_Interp,
        cmd: *const ::std::os::raw::c_char,
        flags: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn Tcl_RecordAndEvalObj(
        interp: *mut Tcl_Interp,
        cmdPtr: *mut Tcl_Obj,
        flags: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn Tcl_RegisterChannel(interp: *mut Tcl_Interp, chan: Tcl_Channel);
}
unsafe extern "C" {
    pub fn Tcl_RegisterObjType(typePtr: *mut Tcl_ObjType);
}
unsafe extern "C" {
    pub fn Tcl_RegExpCompile(
        interp: *mut Tcl_Interp,
        pattern: *const ::std::os::raw::c_char,
    ) -> Tcl_RegExp;
}
unsafe extern "C" {
    pub fn Tcl_RegExpExec(
        interp: *mut Tcl_Interp,
        regexp: Tcl_RegExp,
        text: *const ::std::os::raw::c_char,
        start: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn Tcl_RegExpMatch(
        interp: *mut Tcl_Interp,
        text: *const ::std::os::raw::c_char,
        pattern: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn Tcl_RegExpRange(
        regexp: Tcl_RegExp,
        index: ::std::os::raw::c_int,
        startPtr: *mut *const ::std::os::raw::c_char,
        endPtr: *mut *const ::std::os::raw::c_char,
    );
}
unsafe extern "C" {
    pub fn Tcl_Release(clientData: ClientData);
}
unsafe extern "C" {
    pub fn Tcl_ResetResult(interp: *mut Tcl_Interp);
}
unsafe extern "C" {
    pub fn Tcl_ScanElement(
        str_: *const ::std::os::raw::c_char,
        flagPtr: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn Tcl_ScanCountedElement(
        str_: *const ::std::os::raw::c_char,
        length: ::std::os::raw::c_int,
        flagPtr: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn Tcl_SeekOld(
        chan: Tcl_Channel,
        offset: ::std::os::raw::c_int,
        mode: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn Tcl_ServiceAll() -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn Tcl_ServiceEvent(flags: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn Tcl_SetAssocData(
        interp: *mut Tcl_Interp,
        name: *const ::std::os::raw::c_char,
        proc_: Tcl_InterpDeleteProc,
        clientData: ClientData,
    );
}
unsafe extern "C" {
    pub fn Tcl_SetChannelBufferSize(chan: Tcl_Channel, sz: ::std::os::raw::c_int);
}
unsafe extern "C" {
    pub fn Tcl_SetChannelOption(
        interp: *mut Tcl_Interp,
        chan: Tcl_Channel,
        optionName: *const ::std::os::raw::c_char,
        newValue: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn Tcl_SetCommandInfo(
        interp: *mut Tcl_Interp,
        cmdName: *const ::std::os::raw::c_char,
        infoPtr: *const Tcl_CmdInfo,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn Tcl_SetErrno(err: ::std::os::raw::c_int);
}
unsafe extern "C" {
    pub fn Tcl_SetErrorCode(interp: *mut Tcl_Interp, ...);
}
unsafe extern "C" {
    pub fn Tcl_SetMaxBlockTime(timePtr: *mut Tcl_Time);
}
unsafe extern "C" {
    pub fn Tcl_SetPanicProc(panicProc: Tcl_PanicProc);
}
unsafe extern "C" {
    pub fn Tcl_SetRecursionLimit(
        interp: *mut Tcl_Interp,
        depth: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn Tcl_SetResult(
        interp: *mut Tcl_Interp,
        result: *mut ::std::os::raw::c_char,
        freeProc: Tcl_FreeProc,
    );
}
unsafe extern "C" {
    pub fn Tcl_SetServiceMode(mode: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn Tcl_SetObjErrorCode(interp: *mut Tcl_Interp, errorObjPtr: *mut Tcl_Obj);
}
unsafe extern "C" {
    pub fn Tcl_SetObjResult(interp: *mut Tcl_Interp, resultObjPtr: *mut Tcl_Obj);
}
unsafe extern "C" {
    pub fn Tcl_SetStdChannel(channel: Tcl_Channel, type_: ::std::os::raw::c_int);
}
unsafe extern "C" {
    pub fn Tcl_SetVar(
        interp: *mut Tcl_Interp,
        varName: *const ::std::os::raw::c_char,
        newValue: *const ::std::os::raw::c_char,
        flags: ::std::os::raw::c_int,
    ) -> *const ::std::os::raw::c_char;
}
unsafe extern "C" {
    pub fn Tcl_SetVar2(
        interp: *mut Tcl_Interp,
        part1: *const ::std::os::raw::c_char,
        part2: *const ::std::os::raw::c_char,
        newValue: *const ::std::os::raw::c_char,
        flags: ::std::os::raw::c_int,
    ) -> *const ::std::os::raw::c_char;
}
unsafe extern "C" {
    pub fn Tcl_SignalId(sig: ::std::os::raw::c_int) -> *const ::std::os::raw::c_char;
}
unsafe extern "C" {
    pub fn Tcl_SignalMsg(sig: ::std::os::raw::c_int) -> *const ::std::os::raw::c_char;
}
unsafe extern "C" {
    pub fn Tcl_SourceRCFile(interp: *mut Tcl_Interp);
}
unsafe extern "C" {
    pub fn Tcl_SplitList(
        interp: *mut Tcl_Interp,
        listStr: *const ::std::os::raw::c_char,
        argcPtr: *mut ::std::os::raw::c_int,
        argvPtr: *mut *mut *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn Tcl_SplitPath(
        path: *const ::std::os::raw::c_char,
        argcPtr: *mut ::std::os::raw::c_int,
        argvPtr: *mut *mut *const ::std::os::raw::c_char,
    );
}
unsafe extern "C" {
    pub fn Tcl_StaticPackage(
        interp: *mut Tcl_Interp,
        pkgName: *const ::std::os::raw::c_char,
        initProc: Tcl_PackageInitProc,
        safeInitProc: Tcl_PackageInitProc,
    );
}
unsafe extern "C" {
    pub fn Tcl_StringMatch(
        str_: *const ::std::os::raw::c_char,
        pattern: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn Tcl_TellOld(chan: Tcl_Channel) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn Tcl_TraceVar(
        interp: *mut Tcl_Interp,
        varName: *const ::std::os::raw::c_char,
        flags: ::std::os::raw::c_int,
        proc_: Tcl_VarTraceProc,
        clientData: ClientData,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn Tcl_TraceVar2(
        interp: *mut Tcl_Interp,
        part1: *const ::std::os::raw::c_char,
        part2: *const ::std::os::raw::c_char,
        flags: ::std::os::raw::c_int,
        proc_: Tcl_VarTraceProc,
        clientData: ClientData,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn Tcl_TranslateFileName(
        interp: *mut Tcl_Interp,
        name: *const ::std::os::raw::c_char,
        bufferPtr: *mut Tcl_DString,
    ) -> *mut ::std::os::raw::c_char;
}
unsafe extern "C" {
    pub fn Tcl_Ungets(
        chan: Tcl_Channel,
        str_: *const ::std::os::raw::c_char,
        len: ::std::os::raw::c_int,
        atHead: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn Tcl_UnlinkVar(interp: *mut Tcl_Interp, varName: *const ::std::os::raw::c_char);
}
unsafe extern "C" {
    pub fn Tcl_UnregisterChannel(
        interp: *mut Tcl_Interp,
        chan: Tcl_Channel,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn Tcl_UnsetVar(
        interp: *mut Tcl_Interp,
        varName: *const ::std::os::raw::c_char,
        flags: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn Tcl_UnsetVar2(
        interp: *mut Tcl_Interp,
        part1: *const ::std::os::raw::c_char,
        part2: *const ::std::os::raw::c_char,
        flags: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn Tcl_UntraceVar(
        interp: *mut Tcl_Interp,
        varName: *const ::std::os::raw::c_char,
        flags: ::std::os::raw::c_int,
        proc_: Tcl_VarTraceProc,
        clientData: ClientData,
    );
}
unsafe extern "C" {
    pub fn Tcl_UntraceVar2(
        interp: *mut Tcl_Interp,
        part1: *const ::std::os::raw::c_char,
        part2: *const ::std::os::raw::c_char,
        flags: ::std::os::raw::c_int,
        proc_: Tcl_VarTraceProc,
        clientData: ClientData,
    );
}
unsafe extern "C" {
    pub fn Tcl_UpdateLinkedVar(interp: *mut Tcl_Interp, varName: *const ::std::os::raw::c_char);
}
unsafe extern "C" {
    pub fn Tcl_UpVar(
        interp: *mut Tcl_Interp,
        frameName: *const ::std::os::raw::c_char,
        varName: *const ::std::os::raw::c_char,
        localName: *const ::std::os::raw::c_char,
        flags: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn Tcl_UpVar2(
        interp: *mut Tcl_Interp,
        frameName: *const ::std::os::raw::c_char,
        part1: *const ::std::os::raw::c_char,
        part2: *const ::std::os::raw::c_char,
        localName: *const ::std::os::raw::c_char,
        flags: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn Tcl_VarEval(interp: *mut Tcl_Interp, ...) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn Tcl_VarTraceInfo(
        interp: *mut Tcl_Interp,
        varName: *const ::std::os::raw::c_char,
        flags: ::std::os::raw::c_int,
        procPtr: Tcl_VarTraceProc,
        prevClientData: ClientData,
    ) -> ClientData;
}
unsafe extern "C" {
    pub fn Tcl_VarTraceInfo2(
        interp: *mut Tcl_Interp,
        part1: *const ::std::os::raw::c_char,
        part2: *const ::std::os::raw::c_char,
        flags: ::std::os::raw::c_int,
        procPtr: Tcl_VarTraceProc,
        prevClientData: ClientData,
    ) -> ClientData;
}
unsafe extern "C" {
    pub fn Tcl_Write(
        chan: Tcl_Channel,
        s: *const ::std::os::raw::c_char,
        slen: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn Tcl_WrongNumArgs(
        interp: *mut Tcl_Interp,
        objc: ::std::os::raw::c_int,
        objv: *const *mut Tcl_Obj,
        message: *const ::std::os::raw::c_char,
    );
}
unsafe extern "C" {
    pub fn Tcl_DumpActiveMemory(fileName: *const ::std::os::raw::c_char) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn Tcl_ValidateAllMemory(file: *const ::std::os::raw::c_char, line: ::std::os::raw::c_int);
}
unsafe extern "C" {
    pub fn Tcl_AppendResultVA(interp: *mut Tcl_Interp, argList: va_list);
}
unsafe extern "C" {
    pub fn Tcl_AppendStringsToObjVA(objPtr: *mut Tcl_Obj, argList: va_list);
}
unsafe extern "C" {
    pub fn Tcl_HashStats(tablePtr: *mut Tcl_HashTable) -> *mut ::std::os::raw::c_char;
}
unsafe extern "C" {
    pub fn Tcl_ParseVar(
        interp: *mut Tcl_Interp,
        start: *const ::std::os::raw::c_char,
        termPtr: *mut *const ::std::os::raw::c_char,
    ) -> *const ::std::os::raw::c_char;
}
unsafe extern "C" {
    pub fn Tcl_PkgPresent(
        interp: *mut Tcl_Interp,
        name: *const ::std::os::raw::c_char,
        version: *const ::std::os::raw::c_char,
        exact: ::std::os::raw::c_int,
    ) -> *const ::std::os::raw::c_char;
}
unsafe extern "C" {
    pub fn Tcl_PkgPresentEx(
        interp: *mut Tcl_Interp,
        name: *const ::std::os::raw::c_char,
        version: *const ::std::os::raw::c_char,
        exact: ::std::os::raw::c_int,
        clientDataPtr: *mut ClientData,
    ) -> *const ::std::os::raw::c_char;
}
unsafe extern "C" {
    pub fn Tcl_PkgProvide(
        interp: *mut Tcl_Interp,
        name: *const ::std::os::raw::c_char,
        version: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn Tcl_PkgRequire(
        interp: *mut Tcl_Interp,
        name: *const ::std::os::raw::c_char,
        version: *const ::std::os::raw::c_char,
        exact: ::std::os::raw::c_int,
    ) -> *const ::std::os::raw::c_char;
}
unsafe extern "C" {
    pub fn Tcl_SetErrorCodeVA(interp: *mut Tcl_Interp, argList: va_list);
}
unsafe extern "C" {
    pub fn Tcl_VarEvalVA(interp: *mut Tcl_Interp, argList: va_list) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn Tcl_WaitPid(
        pid: Tcl_Pid,
        statPtr: *mut ::std::os::raw::c_int,
        options: ::std::os::raw::c_int,
    ) -> Tcl_Pid;
}
unsafe extern "C" {
    pub fn Tcl_PanicVA(format: *const ::std::os::raw::c_char, argList: va_list);
}
unsafe extern "C" {
    pub fn Tcl_GetVersion(
        major: *mut ::std::os::raw::c_int,
        minor: *mut ::std::os::raw::c_int,
        patchLevel: *mut ::std::os::raw::c_int,
        type_: *mut ::std::os::raw::c_int,
    );
}
unsafe extern "C" {
    pub fn Tcl_InitMemory(interp: *mut Tcl_Interp);
}
unsafe extern "C" {
    pub fn Tcl_StackChannel(
        interp: *mut Tcl_Interp,
        typePtr: *mut Tcl_ChannelType,
        instanceData: ClientData,
        mask: ::std::os::raw::c_int,
        prevChan: Tcl_Channel,
    ) -> Tcl_Channel;
}
unsafe extern "C" {
    pub fn Tcl_UnstackChannel(interp: *mut Tcl_Interp, chan: Tcl_Channel) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn Tcl_GetStackedChannel(chan: Tcl_Channel) -> Tcl_Channel;
}
unsafe extern "C" {
    pub fn Tcl_SetMainLoop(proc_: Tcl_MainLoopProc);
}
unsafe extern "C" {
    pub fn Tcl_AppendObjToObj(objPtr: *mut Tcl_Obj, appendObjPtr: *mut Tcl_Obj);
}
unsafe extern "C" {
    pub fn Tcl_CreateEncoding(typePtr: *const Tcl_EncodingType) -> Tcl_Encoding;
}
unsafe extern "C" {
    pub fn Tcl_CreateThreadExitHandler(proc_: Tcl_ExitProc, clientData: ClientData);
}
unsafe extern "C" {
    pub fn Tcl_DeleteThreadExitHandler(proc_: Tcl_ExitProc, clientData: ClientData);
}
unsafe extern "C" {
    pub fn Tcl_DiscardResult(statePtr: *mut Tcl_SavedResult);
}
unsafe extern "C" {
    pub fn Tcl_EvalEx(
        interp: *mut Tcl_Interp,
        script: *const ::std::os::raw::c_char,
        numBytes: ::std::os::raw::c_int,
        flags: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn Tcl_EvalObjv(
        interp: *mut Tcl_Interp,
        objc: ::std::os::raw::c_int,
        objv: *const *mut Tcl_Obj,
        flags: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn Tcl_EvalObjEx(
        interp: *mut Tcl_Interp,
        objPtr: *mut Tcl_Obj,
        flags: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn Tcl_ExitThread(status: ::std::os::raw::c_int);
}
unsafe extern "C" {
    pub fn Tcl_ExternalToUtf(
        interp: *mut Tcl_Interp,
        encoding: Tcl_Encoding,
        src: *const ::std::os::raw::c_char,
        srcLen: ::std::os::raw::c_int,
        flags: ::std::os::raw::c_int,
        statePtr: *mut Tcl_EncodingState,
        dst: *mut ::std::os::raw::c_char,
        dstLen: ::std::os::raw::c_int,
        srcReadPtr: *mut ::std::os::raw::c_int,
        dstWrotePtr: *mut ::std::os::raw::c_int,
        dstCharsPtr: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn Tcl_ExternalToUtfDString(
        encoding: Tcl_Encoding,
        src: *const ::std::os::raw::c_char,
        srcLen: ::std::os::raw::c_int,
        dsPtr: *mut Tcl_DString,
    ) -> *mut ::std::os::raw::c_char;
}
unsafe extern "C" {
    pub fn Tcl_FinalizeThread();
}
unsafe extern "C" {
    pub fn Tcl_FinalizeNotifier(clientData: ClientData);
}
unsafe extern "C" {
    pub fn Tcl_FreeEncoding(encoding: Tcl_Encoding);
}
unsafe extern "C" {
    pub fn Tcl_GetCurrentThread() -> Tcl_ThreadId;
}
unsafe extern "C" {
    pub fn Tcl_GetEncoding(
        interp: *mut Tcl_Interp,
        name: *const ::std::os::raw::c_char,
    ) -> Tcl_Encoding;
}
unsafe extern "C" {
    pub fn Tcl_GetEncodingName(encoding: Tcl_Encoding) -> *const ::std::os::raw::c_char;
}
unsafe extern "C" {
    pub fn Tcl_GetEncodingNames(interp: *mut Tcl_Interp);
}
unsafe extern "C" {
    pub fn Tcl_GetIndexFromObjStruct(
        interp: *mut Tcl_Interp,
        objPtr: *mut Tcl_Obj,
        tablePtr: *const ::std::os::raw::c_void,
        offset: ::std::os::raw::c_int,
        msg: *const ::std::os::raw::c_char,
        flags: ::std::os::raw::c_int,
        indexPtr: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn Tcl_GetThreadData(
        keyPtr: *mut Tcl_ThreadDataKey,
        size: ::std::os::raw::c_int,
    ) -> *mut ::std::os::raw::c_void;
}
unsafe extern "C" {
    pub fn Tcl_GetVar2Ex(
        interp: *mut Tcl_Interp,
        part1: *const ::std::os::raw::c_char,
        part2: *const ::std::os::raw::c_char,
        flags: ::std::os::raw::c_int,
    ) -> *mut Tcl_Obj;
}
unsafe extern "C" {
    pub fn Tcl_InitNotifier() -> ClientData;
}
unsafe extern "C" {
    pub fn Tcl_MutexLock(mutexPtr: *mut Tcl_Mutex);
}
unsafe extern "C" {
    pub fn Tcl_MutexUnlock(mutexPtr: *mut Tcl_Mutex);
}
unsafe extern "C" {
    pub fn Tcl_ConditionNotify(condPtr: *mut Tcl_Condition);
}
unsafe extern "C" {
    pub fn Tcl_ConditionWait(
        condPtr: *mut Tcl_Condition,
        mutexPtr: *mut Tcl_Mutex,
        timePtr: *mut Tcl_Time,
    );
}
unsafe extern "C" {
    pub fn Tcl_NumUtfChars(
        src: *const ::std::os::raw::c_char,
        length: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn Tcl_ReadChars(
        channel: Tcl_Channel,
        objPtr: *mut Tcl_Obj,
        charsToRead: ::std::os::raw::c_int,
        appendFlag: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn Tcl_RestoreResult(interp: *mut Tcl_Interp, statePtr: *mut Tcl_SavedResult);
}
unsafe extern "C" {
    pub fn Tcl_SaveResult(interp: *mut Tcl_Interp, statePtr: *mut Tcl_SavedResult);
}
unsafe extern "C" {
    pub fn Tcl_SetSystemEncoding(
        interp: *mut Tcl_Interp,
        name: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn Tcl_SetVar2Ex(
        interp: *mut Tcl_Interp,
        part1: *const ::std::os::raw::c_char,
        part2: *const ::std::os::raw::c_char,
        newValuePtr: *mut Tcl_Obj,
        flags: ::std::os::raw::c_int,
    ) -> *mut Tcl_Obj;
}
unsafe extern "C" {
    pub fn Tcl_ThreadAlert(threadId: Tcl_ThreadId);
}
unsafe extern "C" {
    pub fn Tcl_ThreadQueueEvent(
        threadId: Tcl_ThreadId,
        evPtr: *mut Tcl_Event,
        position: Tcl_QueuePosition,
    );
}
unsafe extern "C" {
    pub fn Tcl_UniCharAtIndex(
        src: *const ::std::os::raw::c_char,
        index: ::std::os::raw::c_int,
    ) -> Tcl_UniChar;
}
unsafe extern "C" {
    pub fn Tcl_UniCharToLower(ch: ::std::os::raw::c_int) -> Tcl_UniChar;
}
unsafe extern "C" {
    pub fn Tcl_UniCharToTitle(ch: ::std::os::raw::c_int) -> Tcl_UniChar;
}
unsafe extern "C" {
    pub fn Tcl_UniCharToUpper(ch: ::std::os::raw::c_int) -> Tcl_UniChar;
}
unsafe extern "C" {
    pub fn Tcl_UniCharToUtf(
        ch: ::std::os::raw::c_int,
        buf: *mut ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn Tcl_UtfAtIndex(
        src: *const ::std::os::raw::c_char,
        index: ::std::os::raw::c_int,
    ) -> *const ::std::os::raw::c_char;
}
unsafe extern "C" {
    pub fn Tcl_UtfCharComplete(
        src: *const ::std::os::raw::c_char,
        length: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn Tcl_UtfBackslash(
        src: *const ::std::os::raw::c_char,
        readPtr: *mut ::std::os::raw::c_int,
        dst: *mut ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn Tcl_UtfFindFirst(
        src: *const ::std::os::raw::c_char,
        ch: ::std::os::raw::c_int,
    ) -> *const ::std::os::raw::c_char;
}
unsafe extern "C" {
    pub fn Tcl_UtfFindLast(
        src: *const ::std::os::raw::c_char,
        ch: ::std::os::raw::c_int,
    ) -> *const ::std::os::raw::c_char;
}
unsafe extern "C" {
    pub fn Tcl_UtfNext(src: *const ::std::os::raw::c_char) -> *const ::std::os::raw::c_char;
}
unsafe extern "C" {
    pub fn Tcl_UtfPrev(
        src: *const ::std::os::raw::c_char,
        start: *const ::std::os::raw::c_char,
    ) -> *const ::std::os::raw::c_char;
}
unsafe extern "C" {
    pub fn Tcl_UtfToExternal(
        interp: *mut Tcl_Interp,
        encoding: Tcl_Encoding,
        src: *const ::std::os::raw::c_char,
        srcLen: ::std::os::raw::c_int,
        flags: ::std::os::raw::c_int,
        statePtr: *mut Tcl_EncodingState,
        dst: *mut ::std::os::raw::c_char,
        dstLen: ::std::os::raw::c_int,
        srcReadPtr: *mut ::std::os::raw::c_int,
        dstWrotePtr: *mut ::std::os::raw::c_int,
        dstCharsPtr: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn Tcl_UtfToExternalDString(
        encoding: Tcl_Encoding,
        src: *const ::std::os::raw::c_char,
        srcLen: ::std::os::raw::c_int,
        dsPtr: *mut Tcl_DString,
    ) -> *mut ::std::os::raw::c_char;
}
unsafe extern "C" {
    pub fn Tcl_UtfToLower(src: *mut ::std::os::raw::c_char) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn Tcl_UtfToTitle(src: *mut ::std::os::raw::c_char) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn Tcl_UtfToUniChar(
        src: *const ::std::os::raw::c_char,
        chPtr: *mut Tcl_UniChar,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn Tcl_UtfToUpper(src: *mut ::std::os::raw::c_char) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn Tcl_WriteChars(
        chan: Tcl_Channel,
        src: *const ::std::os::raw::c_char,
        srcLen: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn Tcl_WriteObj(chan: Tcl_Channel, objPtr: *mut Tcl_Obj) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn Tcl_GetString(objPtr: *mut Tcl_Obj) -> *mut ::std::os::raw::c_char;
}
unsafe extern "C" {
    pub fn Tcl_GetDefaultEncodingDir() -> *const ::std::os::raw::c_char;
}
unsafe extern "C" {
    pub fn Tcl_SetDefaultEncodingDir(path: *const ::std::os::raw::c_char);
}
unsafe extern "C" {
    pub fn Tcl_AlertNotifier(clientData: ClientData);
}
unsafe extern "C" {
    pub fn Tcl_ServiceModeHook(mode: ::std::os::raw::c_int);
}
unsafe extern "C" {
    pub fn Tcl_UniCharIsAlnum(ch: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn Tcl_UniCharIsAlpha(ch: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn Tcl_UniCharIsDigit(ch: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn Tcl_UniCharIsLower(ch: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn Tcl_UniCharIsSpace(ch: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn Tcl_UniCharIsUpper(ch: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn Tcl_UniCharIsWordChar(ch: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn Tcl_UniCharLen(uniStr: *const Tcl_UniChar) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn Tcl_UniCharNcmp(
        ucs: *const Tcl_UniChar,
        uct: *const Tcl_UniChar,
        numChars: ::std::os::raw::c_ulong,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn Tcl_UniCharToUtfDString(
        uniStr: *const Tcl_UniChar,
        uniLength: ::std::os::raw::c_int,
        dsPtr: *mut Tcl_DString,
    ) -> *mut ::std::os::raw::c_char;
}
unsafe extern "C" {
    pub fn Tcl_UtfToUniCharDString(
        src: *const ::std::os::raw::c_char,
        length: ::std::os::raw::c_int,
        dsPtr: *mut Tcl_DString,
    ) -> *mut Tcl_UniChar;
}
unsafe extern "C" {
    pub fn Tcl_GetRegExpFromObj(
        interp: *mut Tcl_Interp,
        patObj: *mut Tcl_Obj,
        flags: ::std::os::raw::c_int,
    ) -> Tcl_RegExp;
}
unsafe extern "C" {
    pub fn Tcl_EvalTokens(
        interp: *mut Tcl_Interp,
        tokenPtr: *mut Tcl_Token,
        count: ::std::os::raw::c_int,
    ) -> *mut Tcl_Obj;
}
unsafe extern "C" {
    pub fn Tcl_FreeParse(parsePtr: *mut Tcl_Parse);
}
unsafe extern "C" {
    pub fn Tcl_LogCommandInfo(
        interp: *mut Tcl_Interp,
        script: *const ::std::os::raw::c_char,
        command: *const ::std::os::raw::c_char,
        length: ::std::os::raw::c_int,
    );
}
unsafe extern "C" {
    pub fn Tcl_ParseBraces(
        interp: *mut Tcl_Interp,
        start: *const ::std::os::raw::c_char,
        numBytes: ::std::os::raw::c_int,
        parsePtr: *mut Tcl_Parse,
        append: ::std::os::raw::c_int,
        termPtr: *mut *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn Tcl_ParseCommand(
        interp: *mut Tcl_Interp,
        start: *const ::std::os::raw::c_char,
        numBytes: ::std::os::raw::c_int,
        nested: ::std::os::raw::c_int,
        parsePtr: *mut Tcl_Parse,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn Tcl_ParseExpr(
        interp: *mut Tcl_Interp,
        start: *const ::std::os::raw::c_char,
        numBytes: ::std::os::raw::c_int,
        parsePtr: *mut Tcl_Parse,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn Tcl_ParseQuotedString(
        interp: *mut Tcl_Interp,
        start: *const ::std::os::raw::c_char,
        numBytes: ::std::os::raw::c_int,
        parsePtr: *mut Tcl_Parse,
        append: ::std::os::raw::c_int,
        termPtr: *mut *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn Tcl_ParseVarName(
        interp: *mut Tcl_Interp,
        start: *const ::std::os::raw::c_char,
        numBytes: ::std::os::raw::c_int,
        parsePtr: *mut Tcl_Parse,
        append: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn Tcl_GetCwd(
        interp: *mut Tcl_Interp,
        cwdPtr: *mut Tcl_DString,
    ) -> *mut ::std::os::raw::c_char;
}
unsafe extern "C" {
    pub fn Tcl_Chdir(dirName: *const ::std::os::raw::c_char) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn Tcl_Access(
        path: *const ::std::os::raw::c_char,
        mode: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn Tcl_Stat(
        path: *const ::std::os::raw::c_char,
        bufPtr: *mut stat,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn Tcl_UtfNcmp(
        s1: *const ::std::os::raw::c_char,
        s2: *const ::std::os::raw::c_char,
        n: ::std::os::raw::c_ulong,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn Tcl_UtfNcasecmp(
        s1: *const ::std::os::raw::c_char,
        s2: *const ::std::os::raw::c_char,
        n: ::std::os::raw::c_ulong,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn Tcl_StringCaseMatch(
        str_: *const ::std::os::raw::c_char,
        pattern: *const ::std::os::raw::c_char,
        nocase: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn Tcl_UniCharIsControl(ch: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn Tcl_UniCharIsGraph(ch: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn Tcl_UniCharIsPrint(ch: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn Tcl_UniCharIsPunct(ch: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn Tcl_RegExpExecObj(
        interp: *mut Tcl_Interp,
        regexp: Tcl_RegExp,
        textObj: *mut Tcl_Obj,
        offset: ::std::os::raw::c_int,
        nmatches: ::std::os::raw::c_int,
        flags: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn Tcl_RegExpGetInfo(regexp: Tcl_RegExp, infoPtr: *mut Tcl_RegExpInfo);
}
unsafe extern "C" {
    pub fn Tcl_NewUnicodeObj(
        unicode: *const Tcl_UniChar,
        numChars: ::std::os::raw::c_int,
    ) -> *mut Tcl_Obj;
}
unsafe extern "C" {
    pub fn Tcl_SetUnicodeObj(
        objPtr: *mut Tcl_Obj,
        unicode: *const Tcl_UniChar,
        numChars: ::std::os::raw::c_int,
    );
}
unsafe extern "C" {
    pub fn Tcl_GetCharLength(objPtr: *mut Tcl_Obj) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn Tcl_GetUniChar(objPtr: *mut Tcl_Obj, index: ::std::os::raw::c_int) -> Tcl_UniChar;
}
unsafe extern "C" {
    pub fn Tcl_GetUnicode(objPtr: *mut Tcl_Obj) -> *mut Tcl_UniChar;
}
unsafe extern "C" {
    pub fn Tcl_GetRange(
        objPtr: *mut Tcl_Obj,
        first: ::std::os::raw::c_int,
        last: ::std::os::raw::c_int,
    ) -> *mut Tcl_Obj;
}
unsafe extern "C" {
    pub fn Tcl_AppendUnicodeToObj(
        objPtr: *mut Tcl_Obj,
        unicode: *const Tcl_UniChar,
        length: ::std::os::raw::c_int,
    );
}
unsafe extern "C" {
    pub fn Tcl_RegExpMatchObj(
        interp: *mut Tcl_Interp,
        textObj: *mut Tcl_Obj,
        patternObj: *mut Tcl_Obj,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn Tcl_SetNotifier(notifierProcPtr: *mut Tcl_NotifierProcs);
}
unsafe extern "C" {
    pub fn Tcl_GetAllocMutex() -> *mut Tcl_Mutex;
}
unsafe extern "C" {
    pub fn Tcl_GetChannelNames(interp: *mut Tcl_Interp) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn Tcl_GetChannelNamesEx(
        interp: *mut Tcl_Interp,
        pattern: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn Tcl_ProcObjCmd(
        clientData: ClientData,
        interp: *mut Tcl_Interp,
        objc: ::std::os::raw::c_int,
        objv: *const *mut Tcl_Obj,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn Tcl_ConditionFinalize(condPtr: *mut Tcl_Condition);
}
unsafe extern "C" {
    pub fn Tcl_MutexFinalize(mutex: *mut Tcl_Mutex);
}
unsafe extern "C" {
    pub fn Tcl_CreateThread(
        idPtr: *mut Tcl_ThreadId,
        proc_: Tcl_ThreadCreateProc,
        clientData: ClientData,
        stackSize: ::std::os::raw::c_int,
        flags: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn Tcl_ReadRaw(
        chan: Tcl_Channel,
        dst: *mut ::std::os::raw::c_char,
        bytesToRead: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn Tcl_WriteRaw(
        chan: Tcl_Channel,
        src: *const ::std::os::raw::c_char,
        srcLen: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn Tcl_GetTopChannel(chan: Tcl_Channel) -> Tcl_Channel;
}
unsafe extern "C" {
    pub fn Tcl_ChannelBuffered(chan: Tcl_Channel) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn Tcl_ChannelName(chanTypePtr: *const Tcl_ChannelType) -> *const ::std::os::raw::c_char;
}
unsafe extern "C" {
    pub fn Tcl_ChannelVersion(chanTypePtr: *const Tcl_ChannelType) -> Tcl_ChannelTypeVersion;
}
unsafe extern "C" {
    pub fn Tcl_ChannelBlockModeProc(chanTypePtr: *const Tcl_ChannelType)
        -> Tcl_DriverBlockModeProc;
}
unsafe extern "C" {
    pub fn Tcl_ChannelCloseProc(chanTypePtr: *const Tcl_ChannelType) -> Tcl_DriverCloseProc;
}
unsafe extern "C" {
    pub fn Tcl_ChannelClose2Proc(chanTypePtr: *const Tcl_ChannelType) -> Tcl_DriverClose2Proc;
}
unsafe extern "C" {
    pub fn Tcl_ChannelInputProc(chanTypePtr: *const Tcl_ChannelType) -> Tcl_DriverInputProc;
}
unsafe extern "C" {
    pub fn Tcl_ChannelOutputProc(chanTypePtr: *const Tcl_ChannelType) -> Tcl_DriverOutputProc;
}
unsafe extern "C" {
    pub fn Tcl_ChannelSeekProc(chanTypePtr: *const Tcl_ChannelType) -> Tcl_DriverSeekProc;
}
unsafe extern "C" {
    pub fn Tcl_ChannelSetOptionProc(chanTypePtr: *const Tcl_ChannelType)
        -> Tcl_DriverSetOptionProc;
}
unsafe extern "C" {
    pub fn Tcl_ChannelGetOptionProc(chanTypePtr: *const Tcl_ChannelType)
        -> Tcl_DriverGetOptionProc;
}
unsafe extern "C" {
    pub fn Tcl_ChannelWatchProc(chanTypePtr: *const Tcl_ChannelType) -> Tcl_DriverWatchProc;
}
unsafe extern "C" {
    pub fn Tcl_ChannelGetHandleProc(chanTypePtr: *const Tcl_ChannelType)
        -> Tcl_DriverGetHandleProc;
}
unsafe extern "C" {
    pub fn Tcl_ChannelFlushProc(chanTypePtr: *const Tcl_ChannelType) -> Tcl_DriverFlushProc;
}
unsafe extern "C" {
    pub fn Tcl_ChannelHandlerProc(chanTypePtr: *const Tcl_ChannelType) -> Tcl_DriverHandlerProc;
}
unsafe extern "C" {
    pub fn Tcl_JoinThread(
        threadId: Tcl_ThreadId,
        result: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn Tcl_IsChannelShared(channel: Tcl_Channel) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn Tcl_IsChannelRegistered(
        interp: *mut Tcl_Interp,
        channel: Tcl_Channel,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn Tcl_CutChannel(channel: Tcl_Channel);
}
unsafe extern "C" {
    pub fn Tcl_SpliceChannel(channel: Tcl_Channel);
}
unsafe extern "C" {
    pub fn Tcl_ClearChannelHandlers(channel: Tcl_Channel);
}
unsafe extern "C" {
    pub fn Tcl_IsChannelExisting(
        channelName: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn Tcl_UniCharNcasecmp(
        ucs: *const Tcl_UniChar,
        uct: *const Tcl_UniChar,
        numChars: ::std::os::raw::c_ulong,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn Tcl_UniCharCaseMatch(
        uniStr: *const Tcl_UniChar,
        uniPattern: *const Tcl_UniChar,
        nocase: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn Tcl_FindHashEntry(
        tablePtr: *mut Tcl_HashTable,
        key: *const ::std::os::raw::c_char,
    ) -> *mut Tcl_HashEntry;
}
unsafe extern "C" {
    pub fn Tcl_CreateHashEntry(
        tablePtr: *mut Tcl_HashTable,
        key: *const ::std::os::raw::c_char,
        newPtr: *mut ::std::os::raw::c_int,
    ) -> *mut Tcl_HashEntry;
}
unsafe extern "C" {
    pub fn Tcl_InitCustomHashTable(
        tablePtr: *mut Tcl_HashTable,
        keyType: ::std::os::raw::c_int,
        typePtr: *mut Tcl_HashKeyType,
    );
}
unsafe extern "C" {
    pub fn Tcl_InitObjHashTable(tablePtr: *mut Tcl_HashTable);
}
unsafe extern "C" {
    pub fn Tcl_CommandTraceInfo(
        interp: *mut Tcl_Interp,
        varName: *const ::std::os::raw::c_char,
        flags: ::std::os::raw::c_int,
        procPtr: Tcl_CommandTraceProc,
        prevClientData: ClientData,
    ) -> ClientData;
}
unsafe extern "C" {
    pub fn Tcl_TraceCommand(
        interp: *mut Tcl_Interp,
        varName: *const ::std::os::raw::c_char,
        flags: ::std::os::raw::c_int,
        proc_: Tcl_CommandTraceProc,
        clientData: ClientData,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn Tcl_UntraceCommand(
        interp: *mut Tcl_Interp,
        varName: *const ::std::os::raw::c_char,
        flags: ::std::os::raw::c_int,
        proc_: Tcl_CommandTraceProc,
        clientData: ClientData,
    );
}
unsafe extern "C" {
    pub fn Tcl_AttemptAlloc(size: ::std::os::raw::c_uint) -> *mut ::std::os::raw::c_char;
}
unsafe extern "C" {
    pub fn Tcl_AttemptDbCkalloc(
        size: ::std::os::raw::c_uint,
        file: *const ::std::os::raw::c_char,
        line: ::std::os::raw::c_int,
    ) -> *mut ::std::os::raw::c_char;
}
unsafe extern "C" {
    pub fn Tcl_AttemptRealloc(
        ptr: *mut ::std::os::raw::c_char,
        size: ::std::os::raw::c_uint,
    ) -> *mut ::std::os::raw::c_char;
}
unsafe extern "C" {
    pub fn Tcl_AttemptDbCkrealloc(
        ptr: *mut ::std::os::raw::c_char,
        size: ::std::os::raw::c_uint,
        file: *const ::std::os::raw::c_char,
        line: ::std::os::raw::c_int,
    ) -> *mut ::std::os::raw::c_char;
}
unsafe extern "C" {
    pub fn Tcl_AttemptSetObjLength(
        objPtr: *mut Tcl_Obj,
        length: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn Tcl_GetChannelThread(channel: Tcl_Channel) -> Tcl_ThreadId;
}
unsafe extern "C" {
    pub fn Tcl_GetUnicodeFromObj(
        objPtr: *mut Tcl_Obj,
        lengthPtr: *mut ::std::os::raw::c_int,
    ) -> *mut Tcl_UniChar;
}
unsafe extern "C" {
    pub fn Tcl_GetMathFuncInfo(
        interp: *mut Tcl_Interp,
        name: *const ::std::os::raw::c_char,
        numArgsPtr: *mut ::std::os::raw::c_int,
        argTypesPtr: *mut *mut Tcl_ValueType,
        procPtr: *mut Tcl_MathProc,
        clientDataPtr: *mut ClientData,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn Tcl_ListMathFuncs(
        interp: *mut Tcl_Interp,
        pattern: *const ::std::os::raw::c_char,
    ) -> *mut Tcl_Obj;
}
unsafe extern "C" {
    pub fn Tcl_SubstObj(
        interp: *mut Tcl_Interp,
        objPtr: *mut Tcl_Obj,
        flags: ::std::os::raw::c_int,
    ) -> *mut Tcl_Obj;
}
unsafe extern "C" {
    pub fn Tcl_DetachChannel(
        interp: *mut Tcl_Interp,
        channel: Tcl_Channel,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn Tcl_IsStandardChannel(channel: Tcl_Channel) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn Tcl_FSCopyFile(
        srcPathPtr: *mut Tcl_Obj,
        destPathPtr: *mut Tcl_Obj,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn Tcl_FSCopyDirectory(
        srcPathPtr: *mut Tcl_Obj,
        destPathPtr: *mut Tcl_Obj,
        errorPtr: *mut *mut Tcl_Obj,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn Tcl_FSCreateDirectory(pathPtr: *mut Tcl_Obj) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn Tcl_FSDeleteFile(pathPtr: *mut Tcl_Obj) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn Tcl_FSLoadFile(
        interp: *mut Tcl_Interp,
        pathPtr: *mut Tcl_Obj,
        sym1: *const ::std::os::raw::c_char,
        sym2: *const ::std::os::raw::c_char,
        proc1Ptr: *mut Tcl_PackageInitProc,
        proc2Ptr: *mut Tcl_PackageInitProc,
        handlePtr: *mut Tcl_LoadHandle,
        unloadProcPtr: *mut Tcl_FSUnloadFileProc,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn Tcl_FSMatchInDirectory(
        interp: *mut Tcl_Interp,
        result: *mut Tcl_Obj,
        pathPtr: *mut Tcl_Obj,
        pattern: *const ::std::os::raw::c_char,
        types: *mut Tcl_GlobTypeData,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn Tcl_FSLink(
        pathPtr: *mut Tcl_Obj,
        toPtr: *mut Tcl_Obj,
        linkAction: ::std::os::raw::c_int,
    ) -> *mut Tcl_Obj;
}
unsafe extern "C" {
    pub fn Tcl_FSRemoveDirectory(
        pathPtr: *mut Tcl_Obj,
        recursive: ::std::os::raw::c_int,
        errorPtr: *mut *mut Tcl_Obj,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn Tcl_FSRenameFile(
        srcPathPtr: *mut Tcl_Obj,
        destPathPtr: *mut Tcl_Obj,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn Tcl_FSLstat(pathPtr: *mut Tcl_Obj, buf: *mut Tcl_StatBuf) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn Tcl_FSUtime(pathPtr: *mut Tcl_Obj, tval: *mut utimbuf) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn Tcl_FSFileAttrsGet(
        interp: *mut Tcl_Interp,
        index: ::std::os::raw::c_int,
        pathPtr: *mut Tcl_Obj,
        objPtrRef: *mut *mut Tcl_Obj,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn Tcl_FSFileAttrsSet(
        interp: *mut Tcl_Interp,
        index: ::std::os::raw::c_int,
        pathPtr: *mut Tcl_Obj,
        objPtr: *mut Tcl_Obj,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn Tcl_FSFileAttrStrings(
        pathPtr: *mut Tcl_Obj,
        objPtrRef: *mut *mut Tcl_Obj,
    ) -> *mut *const ::std::os::raw::c_char;
}
unsafe extern "C" {
    pub fn Tcl_FSStat(pathPtr: *mut Tcl_Obj, buf: *mut Tcl_StatBuf) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn Tcl_FSAccess(
        pathPtr: *mut Tcl_Obj,
        mode: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn Tcl_FSOpenFileChannel(
        interp: *mut Tcl_Interp,
        pathPtr: *mut Tcl_Obj,
        modeString: *const ::std::os::raw::c_char,
        permissions: ::std::os::raw::c_int,
    ) -> Tcl_Channel;
}
unsafe extern "C" {
    pub fn Tcl_FSGetCwd(interp: *mut Tcl_Interp) -> *mut Tcl_Obj;
}
unsafe extern "C" {
    pub fn Tcl_FSChdir(pathPtr: *mut Tcl_Obj) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn Tcl_FSConvertToPathType(
        interp: *mut Tcl_Interp,
        pathPtr: *mut Tcl_Obj,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn Tcl_FSJoinPath(listObj: *mut Tcl_Obj, elements: ::std::os::raw::c_int) -> *mut Tcl_Obj;
}
unsafe extern "C" {
    pub fn Tcl_FSSplitPath(
        pathPtr: *mut Tcl_Obj,
        lenPtr: *mut ::std::os::raw::c_int,
    ) -> *mut Tcl_Obj;
}
unsafe extern "C" {
    pub fn Tcl_FSEqualPaths(
        firstPtr: *mut Tcl_Obj,
        secondPtr: *mut Tcl_Obj,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn Tcl_FSGetNormalizedPath(interp: *mut Tcl_Interp, pathPtr: *mut Tcl_Obj) -> *mut Tcl_Obj;
}
unsafe extern "C" {
    pub fn Tcl_FSJoinToPath(
        pathPtr: *mut Tcl_Obj,
        objc: ::std::os::raw::c_int,
        objv: *const *mut Tcl_Obj,
    ) -> *mut Tcl_Obj;
}
unsafe extern "C" {
    pub fn Tcl_FSGetInternalRep(pathPtr: *mut Tcl_Obj, fsPtr: *mut Tcl_Filesystem) -> ClientData;
}
unsafe extern "C" {
    pub fn Tcl_FSGetTranslatedPath(interp: *mut Tcl_Interp, pathPtr: *mut Tcl_Obj) -> *mut Tcl_Obj;
}
unsafe extern "C" {
    pub fn Tcl_FSEvalFile(interp: *mut Tcl_Interp, fileName: *mut Tcl_Obj)
        -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn Tcl_FSNewNativePath(
        fromFilesystem: *mut Tcl_Filesystem,
        clientData: ClientData,
    ) -> *mut Tcl_Obj;
}
unsafe extern "C" {
    pub fn Tcl_FSGetNativePath(pathPtr: *mut Tcl_Obj) -> *const ::std::os::raw::c_char;
}
unsafe extern "C" {
    pub fn Tcl_FSFileSystemInfo(pathPtr: *mut Tcl_Obj) -> *mut Tcl_Obj;
}
unsafe extern "C" {
    pub fn Tcl_FSPathSeparator(pathPtr: *mut Tcl_Obj) -> *mut Tcl_Obj;
}
unsafe extern "C" {
    pub fn Tcl_FSListVolumes() -> *mut Tcl_Obj;
}
unsafe extern "C" {
    pub fn Tcl_FSRegister(
        clientData: ClientData,
        fsPtr: *mut Tcl_Filesystem,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn Tcl_FSUnregister(fsPtr: *mut Tcl_Filesystem) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn Tcl_FSData(fsPtr: *mut Tcl_Filesystem) -> ClientData;
}
unsafe extern "C" {
    pub fn Tcl_FSGetTranslatedStringPath(
        interp: *mut Tcl_Interp,
        pathPtr: *mut Tcl_Obj,
    ) -> *const ::std::os::raw::c_char;
}
unsafe extern "C" {
    pub fn Tcl_FSGetFileSystemForPath(pathPtr: *mut Tcl_Obj) -> *mut Tcl_Filesystem;
}
unsafe extern "C" {
    pub fn Tcl_FSGetPathType(pathPtr: *mut Tcl_Obj) -> Tcl_PathType;
}
unsafe extern "C" {
    pub fn Tcl_OutputBuffered(chan: Tcl_Channel) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn Tcl_FSMountsChanged(fsPtr: *mut Tcl_Filesystem);
}
unsafe extern "C" {
    pub fn Tcl_EvalTokensStandard(
        interp: *mut Tcl_Interp,
        tokenPtr: *mut Tcl_Token,
        count: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn Tcl_GetTime(timeBuf: *mut Tcl_Time);
}
unsafe extern "C" {
    pub fn Tcl_CreateObjTrace(
        interp: *mut Tcl_Interp,
        level: ::std::os::raw::c_int,
        flags: ::std::os::raw::c_int,
        objProc: Tcl_CmdObjTraceProc,
        clientData: ClientData,
        delProc: Tcl_CmdObjTraceDeleteProc,
    ) -> Tcl_Trace;
}
unsafe extern "C" {
    pub fn Tcl_GetCommandInfoFromToken(
        token: Tcl_Command,
        infoPtr: *mut Tcl_CmdInfo,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn Tcl_SetCommandInfoFromToken(
        token: Tcl_Command,
        infoPtr: *const Tcl_CmdInfo,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn Tcl_DbNewWideIntObj(
        wideValue: Tcl_WideInt,
        file: *const ::std::os::raw::c_char,
        line: ::std::os::raw::c_int,
    ) -> *mut Tcl_Obj;
}
unsafe extern "C" {
    pub fn Tcl_GetWideIntFromObj(
        interp: *mut Tcl_Interp,
        objPtr: *mut Tcl_Obj,
        widePtr: *mut Tcl_WideInt,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn Tcl_NewWideIntObj(wideValue: Tcl_WideInt) -> *mut Tcl_Obj;
}
unsafe extern "C" {
    pub fn Tcl_SetWideIntObj(objPtr: *mut Tcl_Obj, wideValue: Tcl_WideInt);
}
unsafe extern "C" {
    pub fn Tcl_AllocStatBuf() -> *mut Tcl_StatBuf;
}
unsafe extern "C" {
    pub fn Tcl_Seek(
        chan: Tcl_Channel,
        offset: Tcl_WideInt,
        mode: ::std::os::raw::c_int,
    ) -> Tcl_WideInt;
}
unsafe extern "C" {
    pub fn Tcl_Tell(chan: Tcl_Channel) -> Tcl_WideInt;
}
unsafe extern "C" {
    pub fn Tcl_ChannelWideSeekProc(chanTypePtr: *const Tcl_ChannelType) -> Tcl_DriverWideSeekProc;
}
unsafe extern "C" {
    pub fn Tcl_DictObjPut(
        interp: *mut Tcl_Interp,
        dictPtr: *mut Tcl_Obj,
        keyPtr: *mut Tcl_Obj,
        valuePtr: *mut Tcl_Obj,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn Tcl_DictObjGet(
        interp: *mut Tcl_Interp,
        dictPtr: *mut Tcl_Obj,
        keyPtr: *mut Tcl_Obj,
        valuePtrPtr: *mut *mut Tcl_Obj,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn Tcl_DictObjRemove(
        interp: *mut Tcl_Interp,
        dictPtr: *mut Tcl_Obj,
        keyPtr: *mut Tcl_Obj,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn Tcl_DictObjSize(
        interp: *mut Tcl_Interp,
        dictPtr: *mut Tcl_Obj,
        sizePtr: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn Tcl_DictObjFirst(
        interp: *mut Tcl_Interp,
        dictPtr: *mut Tcl_Obj,
        searchPtr: *mut Tcl_DictSearch,
        keyPtrPtr: *mut *mut Tcl_Obj,
        valuePtrPtr: *mut *mut Tcl_Obj,
        donePtr: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn Tcl_DictObjNext(
        searchPtr: *mut Tcl_DictSearch,
        keyPtrPtr: *mut *mut Tcl_Obj,
        valuePtrPtr: *mut *mut Tcl_Obj,
        donePtr: *mut ::std::os::raw::c_int,
    );
}
unsafe extern "C" {
    pub fn Tcl_DictObjDone(searchPtr: *mut Tcl_DictSearch);
}
unsafe extern "C" {
    pub fn Tcl_DictObjPutKeyList(
        interp: *mut Tcl_Interp,
        dictPtr: *mut Tcl_Obj,
        keyc: ::std::os::raw::c_int,
        keyv: *const *mut Tcl_Obj,
        valuePtr: *mut Tcl_Obj,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn Tcl_DictObjRemoveKeyList(
        interp: *mut Tcl_Interp,
        dictPtr: *mut Tcl_Obj,
        keyc: ::std::os::raw::c_int,
        keyv: *const *mut Tcl_Obj,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn Tcl_NewDictObj() -> *mut Tcl_Obj;
}
unsafe extern "C" {
    pub fn Tcl_DbNewDictObj(
        file: *const ::std::os::raw::c_char,
        line: ::std::os::raw::c_int,
    ) -> *mut Tcl_Obj;
}
unsafe extern "C" {
    pub fn Tcl_RegisterConfig(
        interp: *mut Tcl_Interp,
        pkgName: *const ::std::os::raw::c_char,
        configuration: *mut Tcl_Config,
        valEncoding: *const ::std::os::raw::c_char,
    );
}
unsafe extern "C" {
    pub fn Tcl_CreateNamespace(
        interp: *mut Tcl_Interp,
        name: *const ::std::os::raw::c_char,
        clientData: ClientData,
        deleteProc: Tcl_NamespaceDeleteProc,
    ) -> *mut Tcl_Namespace;
}
unsafe extern "C" {
    pub fn Tcl_DeleteNamespace(nsPtr: *mut Tcl_Namespace);
}
unsafe extern "C" {
    pub fn Tcl_AppendExportList(
        interp: *mut Tcl_Interp,
        nsPtr: *mut Tcl_Namespace,
        objPtr: *mut Tcl_Obj,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn Tcl_Export(
        interp: *mut Tcl_Interp,
        nsPtr: *mut Tcl_Namespace,
        pattern: *const ::std::os::raw::c_char,
        resetListFirst: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn Tcl_Import(
        interp: *mut Tcl_Interp,
        nsPtr: *mut Tcl_Namespace,
        pattern: *const ::std::os::raw::c_char,
        allowOverwrite: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn Tcl_ForgetImport(
        interp: *mut Tcl_Interp,
        nsPtr: *mut Tcl_Namespace,
        pattern: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn Tcl_GetCurrentNamespace(interp: *mut Tcl_Interp) -> *mut Tcl_Namespace;
}
unsafe extern "C" {
    pub fn Tcl_GetGlobalNamespace(interp: *mut Tcl_Interp) -> *mut Tcl_Namespace;
}
unsafe extern "C" {
    pub fn Tcl_FindNamespace(
        interp: *mut Tcl_Interp,
        name: *const ::std::os::raw::c_char,
        contextNsPtr: *mut Tcl_Namespace,
        flags: ::std::os::raw::c_int,
    ) -> *mut Tcl_Namespace;
}
unsafe extern "C" {
    pub fn Tcl_FindCommand(
        interp: *mut Tcl_Interp,
        name: *const ::std::os::raw::c_char,
        contextNsPtr: *mut Tcl_Namespace,
        flags: ::std::os::raw::c_int,
    ) -> Tcl_Command;
}
unsafe extern "C" {
    pub fn Tcl_GetCommandFromObj(interp: *mut Tcl_Interp, objPtr: *mut Tcl_Obj) -> Tcl_Command;
}
unsafe extern "C" {
    pub fn Tcl_GetCommandFullName(
        interp: *mut Tcl_Interp,
        command: Tcl_Command,
        objPtr: *mut Tcl_Obj,
    );
}
unsafe extern "C" {
    pub fn Tcl_FSEvalFileEx(
        interp: *mut Tcl_Interp,
        fileName: *mut Tcl_Obj,
        encodingName: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn Tcl_SetExitProc(proc_: Tcl_ExitProc) -> Tcl_ExitProc;
}
unsafe extern "C" {
    pub fn Tcl_LimitAddHandler(
        interp: *mut Tcl_Interp,
        type_: ::std::os::raw::c_int,
        handlerProc: Tcl_LimitHandlerProc,
        clientData: ClientData,
        deleteProc: Tcl_LimitHandlerDeleteProc,
    );
}
unsafe extern "C" {
    pub fn Tcl_LimitRemoveHandler(
        interp: *mut Tcl_Interp,
        type_: ::std::os::raw::c_int,
        handlerProc: Tcl_LimitHandlerProc,
        clientData: ClientData,
    );
}
unsafe extern "C" {
    pub fn Tcl_LimitReady(interp: *mut Tcl_Interp) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn Tcl_LimitCheck(interp: *mut Tcl_Interp) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn Tcl_LimitExceeded(interp: *mut Tcl_Interp) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn Tcl_LimitSetCommands(interp: *mut Tcl_Interp, commandLimit: ::std::os::raw::c_int);
}
unsafe extern "C" {
    pub fn Tcl_LimitSetTime(interp: *mut Tcl_Interp, timeLimitPtr: *mut Tcl_Time);
}
unsafe extern "C" {
    pub fn Tcl_LimitSetGranularity(
        interp: *mut Tcl_Interp,
        type_: ::std::os::raw::c_int,
        granularity: ::std::os::raw::c_int,
    );
}
unsafe extern "C" {
    pub fn Tcl_LimitTypeEnabled(
        interp: *mut Tcl_Interp,
        type_: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn Tcl_LimitTypeExceeded(
        interp: *mut Tcl_Interp,
        type_: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn Tcl_LimitTypeSet(interp: *mut Tcl_Interp, type_: ::std::os::raw::c_int);
}
unsafe extern "C" {
    pub fn Tcl_LimitTypeReset(interp: *mut Tcl_Interp, type_: ::std::os::raw::c_int);
}
unsafe extern "C" {
    pub fn Tcl_LimitGetCommands(interp: *mut Tcl_Interp) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn Tcl_LimitGetTime(interp: *mut Tcl_Interp, timeLimitPtr: *mut Tcl_Time);
}
unsafe extern "C" {
    pub fn Tcl_LimitGetGranularity(
        interp: *mut Tcl_Interp,
        type_: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn Tcl_SaveInterpState(
        interp: *mut Tcl_Interp,
        status: ::std::os::raw::c_int,
    ) -> Tcl_InterpState;
}
unsafe extern "C" {
    pub fn Tcl_RestoreInterpState(
        interp: *mut Tcl_Interp,
        state: Tcl_InterpState,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn Tcl_DiscardInterpState(state: Tcl_InterpState);
}
unsafe extern "C" {
    pub fn Tcl_SetReturnOptions(
        interp: *mut Tcl_Interp,
        options: *mut Tcl_Obj,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn Tcl_GetReturnOptions(
        interp: *mut Tcl_Interp,
        result: ::std::os::raw::c_int,
    ) -> *mut Tcl_Obj;
}
unsafe extern "C" {
    pub fn Tcl_IsEnsemble(token: Tcl_Command) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn Tcl_CreateEnsemble(
        interp: *mut Tcl_Interp,
        name: *const ::std::os::raw::c_char,
        namespacePtr: *mut Tcl_Namespace,
        flags: ::std::os::raw::c_int,
    ) -> Tcl_Command;
}
unsafe extern "C" {
    pub fn Tcl_FindEnsemble(
        interp: *mut Tcl_Interp,
        cmdNameObj: *mut Tcl_Obj,
        flags: ::std::os::raw::c_int,
    ) -> Tcl_Command;
}
unsafe extern "C" {
    pub fn Tcl_SetEnsembleSubcommandList(
        interp: *mut Tcl_Interp,
        token: Tcl_Command,
        subcmdList: *mut Tcl_Obj,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn Tcl_SetEnsembleMappingDict(
        interp: *mut Tcl_Interp,
        token: Tcl_Command,
        mapDict: *mut Tcl_Obj,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn Tcl_SetEnsembleUnknownHandler(
        interp: *mut Tcl_Interp,
        token: Tcl_Command,
        unknownList: *mut Tcl_Obj,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn Tcl_SetEnsembleFlags(
        interp: *mut Tcl_Interp,
        token: Tcl_Command,
        flags: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn Tcl_GetEnsembleSubcommandList(
        interp: *mut Tcl_Interp,
        token: Tcl_Command,
        subcmdListPtr: *mut *mut Tcl_Obj,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn Tcl_GetEnsembleMappingDict(
        interp: *mut Tcl_Interp,
        token: Tcl_Command,
        mapDictPtr: *mut *mut Tcl_Obj,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn Tcl_GetEnsembleUnknownHandler(
        interp: *mut Tcl_Interp,
        token: Tcl_Command,
        unknownListPtr: *mut *mut Tcl_Obj,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn Tcl_GetEnsembleFlags(
        interp: *mut Tcl_Interp,
        token: Tcl_Command,
        flagsPtr: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn Tcl_GetEnsembleNamespace(
        interp: *mut Tcl_Interp,
        token: Tcl_Command,
        namespacePtrPtr: *mut *mut Tcl_Namespace,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn Tcl_SetTimeProc(
        getProc: Tcl_GetTimeProc,
        scaleProc: Tcl_ScaleTimeProc,
        clientData: ClientData,
    );
}
unsafe extern "C" {
    pub fn Tcl_QueryTimeProc(
        getProc: *mut Tcl_GetTimeProc,
        scaleProc: *mut Tcl_ScaleTimeProc,
        clientData: *mut ClientData,
    );
}
unsafe extern "C" {
    pub fn Tcl_ChannelThreadActionProc(
        chanTypePtr: *const Tcl_ChannelType,
    ) -> Tcl_DriverThreadActionProc;
}
unsafe extern "C" {
    pub fn Tcl_NewBignumObj(value: *mut mp_int) -> *mut Tcl_Obj;
}
unsafe extern "C" {
    pub fn Tcl_DbNewBignumObj(
        value: *mut mp_int,
        file: *const ::std::os::raw::c_char,
        line: ::std::os::raw::c_int,
    ) -> *mut Tcl_Obj;
}
unsafe extern "C" {
    pub fn Tcl_SetBignumObj(obj: *mut Tcl_Obj, value: *mut mp_int);
}
unsafe extern "C" {
    pub fn Tcl_GetBignumFromObj(
        interp: *mut Tcl_Interp,
        obj: *mut Tcl_Obj,
        value: *mut mp_int,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn Tcl_TakeBignumFromObj(
        interp: *mut Tcl_Interp,
        obj: *mut Tcl_Obj,
        value: *mut mp_int,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn Tcl_TruncateChannel(chan: Tcl_Channel, length: Tcl_WideInt) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn Tcl_ChannelTruncateProc(chanTypePtr: *const Tcl_ChannelType) -> Tcl_DriverTruncateProc;
}
unsafe extern "C" {
    pub fn Tcl_SetChannelErrorInterp(interp: *mut Tcl_Interp, msg: *mut Tcl_Obj);
}
unsafe extern "C" {
    pub fn Tcl_GetChannelErrorInterp(interp: *mut Tcl_Interp, msg: *mut *mut Tcl_Obj);
}
unsafe extern "C" {
    pub fn Tcl_SetChannelError(chan: Tcl_Channel, msg: *mut Tcl_Obj);
}
unsafe extern "C" {
    pub fn Tcl_GetChannelError(chan: Tcl_Channel, msg: *mut *mut Tcl_Obj);
}
unsafe extern "C" {
    pub fn Tcl_InitBignumFromDouble(
        interp: *mut Tcl_Interp,
        initval: f64,
        toInit: *mut mp_int,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn Tcl_GetNamespaceUnknownHandler(
        interp: *mut Tcl_Interp,
        nsPtr: *mut Tcl_Namespace,
    ) -> *mut Tcl_Obj;
}
unsafe extern "C" {
    pub fn Tcl_SetNamespaceUnknownHandler(
        interp: *mut Tcl_Interp,
        nsPtr: *mut Tcl_Namespace,
        handlerPtr: *mut Tcl_Obj,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn Tcl_GetEncodingFromObj(
        interp: *mut Tcl_Interp,
        objPtr: *mut Tcl_Obj,
        encodingPtr: *mut Tcl_Encoding,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn Tcl_GetEncodingSearchPath() -> *mut Tcl_Obj;
}
unsafe extern "C" {
    pub fn Tcl_SetEncodingSearchPath(searchPath: *mut Tcl_Obj) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn Tcl_GetEncodingNameFromEnvironment(
        bufPtr: *mut Tcl_DString,
    ) -> *const ::std::os::raw::c_char;
}
unsafe extern "C" {
    pub fn Tcl_PkgRequireProc(
        interp: *mut Tcl_Interp,
        name: *const ::std::os::raw::c_char,
        objc: ::std::os::raw::c_int,
        objv: *const *mut Tcl_Obj,
        clientDataPtr: *mut ClientData,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn Tcl_AppendObjToErrorInfo(interp: *mut Tcl_Interp, objPtr: *mut Tcl_Obj);
}
unsafe extern "C" {
    pub fn Tcl_AppendLimitedToObj(
        objPtr: *mut Tcl_Obj,
        bytes: *const ::std::os::raw::c_char,
        length: ::std::os::raw::c_int,
        limit: ::std::os::raw::c_int,
        ellipsis: *const ::std::os::raw::c_char,
    );
}
unsafe extern "C" {
    pub fn Tcl_Format(
        interp: *mut Tcl_Interp,
        format: *const ::std::os::raw::c_char,
        objc: ::std::os::raw::c_int,
        objv: *const *mut Tcl_Obj,
    ) -> *mut Tcl_Obj;
}
unsafe extern "C" {
    pub fn Tcl_AppendFormatToObj(
        interp: *mut Tcl_Interp,
        objPtr: *mut Tcl_Obj,
        format: *const ::std::os::raw::c_char,
        objc: ::std::os::raw::c_int,
        objv: *const *mut Tcl_Obj,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn Tcl_ObjPrintf(format: *const ::std::os::raw::c_char, ...) -> *mut Tcl_Obj;
}
unsafe extern "C" {
    pub fn Tcl_AppendPrintfToObj(objPtr: *mut Tcl_Obj, format: *const ::std::os::raw::c_char, ...);
}
unsafe extern "C" {
    pub static mut tclStubsPtr: *mut TclStubs;
}
unsafe extern "C" {
    pub static mut tclPlatStubsPtr: *mut TclPlatStubs;
}

unsafe impl objc2::encode::RefEncode for stat {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for stat {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("stat", &[]);
}
unsafe impl objc2::encode::RefEncode for Tcl_Interp {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for Tcl_Interp {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("Tcl_Interp", &[]);
}
unsafe impl objc2::encode::RefEncode for Tcl_AsyncHandler_ {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for Tcl_AsyncHandler_ {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("Tcl_AsyncHandler_", &[]);
}
unsafe impl objc2::encode::RefEncode for Tcl_Channel_ {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for Tcl_Channel_ {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("Tcl_Channel_", &[]);
}
unsafe impl objc2::encode::RefEncode for Tcl_ChannelTypeVersion_ {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for Tcl_ChannelTypeVersion_ {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("Tcl_ChannelTypeVersion_", &[]);
}
unsafe impl objc2::encode::RefEncode for Tcl_Command_ {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for Tcl_Command_ {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("Tcl_Command_", &[]);
}
unsafe impl objc2::encode::RefEncode for Tcl_Condition_ {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for Tcl_Condition_ {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("Tcl_Condition_", &[]);
}
unsafe impl objc2::encode::RefEncode for Tcl_Dict_ {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for Tcl_Dict_ {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("Tcl_Dict_", &[]);
}
unsafe impl objc2::encode::RefEncode for Tcl_EncodingState_ {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for Tcl_EncodingState_ {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("Tcl_EncodingState_", &[]);
}
unsafe impl objc2::encode::RefEncode for Tcl_Encoding_ {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for Tcl_Encoding_ {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("Tcl_Encoding_", &[]);
}
unsafe impl objc2::encode::RefEncode for Tcl_InterpState_ {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for Tcl_InterpState_ {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("Tcl_InterpState_", &[]);
}
unsafe impl objc2::encode::RefEncode for Tcl_LoadHandle_ {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for Tcl_LoadHandle_ {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("Tcl_LoadHandle_", &[]);
}
unsafe impl objc2::encode::RefEncode for Tcl_Mutex_ {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for Tcl_Mutex_ {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("Tcl_Mutex_", &[]);
}
unsafe impl objc2::encode::RefEncode for Tcl_Pid_ {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for Tcl_Pid_ {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("Tcl_Pid_", &[]);
}
unsafe impl objc2::encode::RefEncode for Tcl_RegExp_ {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for Tcl_RegExp_ {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("Tcl_RegExp_", &[]);
}
unsafe impl objc2::encode::RefEncode for Tcl_ThreadDataKey_ {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for Tcl_ThreadDataKey_ {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("Tcl_ThreadDataKey_", &[]);
}
unsafe impl objc2::encode::RefEncode for Tcl_ThreadId_ {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for Tcl_ThreadId_ {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("Tcl_ThreadId_", &[]);
}
unsafe impl objc2::encode::RefEncode for Tcl_TimerToken_ {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for Tcl_TimerToken_ {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("Tcl_TimerToken_", &[]);
}
unsafe impl objc2::encode::RefEncode for Tcl_Trace_ {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for Tcl_Trace_ {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("Tcl_Trace_", &[]);
}
unsafe impl objc2::encode::RefEncode for Tcl_Var_ {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for Tcl_Var_ {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("Tcl_Var_", &[]);
}
unsafe impl objc2::encode::RefEncode for Tcl_RegExpIndices {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for Tcl_RegExpIndices {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("Tcl_RegExpIndices", &[]);
}
unsafe impl objc2::encode::RefEncode for Tcl_RegExpInfo {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for Tcl_RegExpInfo {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("Tcl_RegExpInfo", &[]);
}
unsafe impl objc2::encode::RefEncode for Tcl_Value {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for Tcl_Value {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("Tcl_Value", &[]);
}
unsafe impl objc2::encode::RefEncode for Tcl_ObjType {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for Tcl_ObjType {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("Tcl_ObjType", &[]);
}
unsafe impl objc2::encode::RefEncode for Tcl_Obj {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for Tcl_Obj {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("Tcl_Obj", &[]);
}
unsafe impl objc2::encode::RefEncode for Tcl_Obj__bindgen_ty_1 {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for Tcl_Obj__bindgen_ty_1 {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("Tcl_Obj__bindgen_ty_1", &[]);
}
unsafe impl objc2::encode::RefEncode for Tcl_Obj__bindgen_ty_1__bindgen_ty_1 {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for Tcl_Obj__bindgen_ty_1__bindgen_ty_1 {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("Tcl_Obj__bindgen_ty_1__bindgen_ty_1", &[]);
}
unsafe impl objc2::encode::RefEncode for Tcl_Obj__bindgen_ty_1__bindgen_ty_2 {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for Tcl_Obj__bindgen_ty_1__bindgen_ty_2 {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("Tcl_Obj__bindgen_ty_1__bindgen_ty_2", &[]);
}
unsafe impl objc2::encode::RefEncode for Tcl_SavedResult {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for Tcl_SavedResult {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("Tcl_SavedResult", &[]);
}
unsafe impl objc2::encode::RefEncode for Tcl_Namespace {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for Tcl_Namespace {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("Tcl_Namespace", &[]);
}
unsafe impl objc2::encode::RefEncode for Tcl_CallFrame {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for Tcl_CallFrame {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("Tcl_CallFrame", &[]);
}
unsafe impl objc2::encode::RefEncode for Tcl_CmdInfo {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for Tcl_CmdInfo {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("Tcl_CmdInfo", &[]);
}
unsafe impl objc2::encode::RefEncode for Tcl_DString {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for Tcl_DString {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("Tcl_DString", &[]);
}
unsafe impl objc2::encode::RefEncode for Tcl_HashEntry {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for Tcl_HashEntry {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("Tcl_HashEntry", &[]);
}
unsafe impl objc2::encode::RefEncode for Tcl_HashEntry__bindgen_ty_1 {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for Tcl_HashEntry__bindgen_ty_1 {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("Tcl_HashEntry__bindgen_ty_1", &[]);
}
unsafe impl objc2::encode::RefEncode for Tcl_HashKeyType {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for Tcl_HashKeyType {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("Tcl_HashKeyType", &[]);
}
unsafe impl objc2::encode::RefEncode for Tcl_HashTable {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for Tcl_HashTable {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("Tcl_HashTable", &[]);
}
unsafe impl objc2::encode::RefEncode for Tcl_HashSearch {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for Tcl_HashSearch {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("Tcl_HashSearch", &[]);
}
unsafe impl objc2::encode::RefEncode for Tcl_DictSearch {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for Tcl_DictSearch {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("Tcl_DictSearch", &[]);
}
unsafe impl objc2::encode::RefEncode for Tcl_Event {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for Tcl_Event {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("Tcl_Event", &[]);
}
unsafe impl objc2::encode::RefEncode for Tcl_Time {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for Tcl_Time {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("Tcl_Time", &[]);
}
unsafe impl objc2::encode::RefEncode for Tcl_ChannelType {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for Tcl_ChannelType {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("Tcl_ChannelType", &[]);
}
unsafe impl objc2::encode::RefEncode for Tcl_GlobTypeData {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for Tcl_GlobTypeData {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("Tcl_GlobTypeData", &[]);
}
unsafe impl objc2::encode::RefEncode for utimbuf {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for utimbuf {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("utimbuf", &[]);
}
unsafe impl objc2::encode::RefEncode for Tcl_FSVersion_ {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for Tcl_FSVersion_ {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("Tcl_FSVersion_", &[]);
}
unsafe impl objc2::encode::RefEncode for Tcl_Filesystem {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for Tcl_Filesystem {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("Tcl_Filesystem", &[]);
}
unsafe impl objc2::encode::RefEncode for Tcl_NotifierProcs {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for Tcl_NotifierProcs {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("Tcl_NotifierProcs", &[]);
}
unsafe impl objc2::encode::RefEncode for Tcl_EncodingType {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for Tcl_EncodingType {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("Tcl_EncodingType", &[]);
}
unsafe impl objc2::encode::RefEncode for Tcl_Token {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for Tcl_Token {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("Tcl_Token", &[]);
}
unsafe impl objc2::encode::RefEncode for Tcl_Parse {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for Tcl_Parse {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("Tcl_Parse", &[]);
}
unsafe impl objc2::encode::RefEncode for Tcl_Config {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for Tcl_Config {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("Tcl_Config", &[]);
}
unsafe impl objc2::encode::RefEncode for TclStubHooks {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for TclStubHooks {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("TclStubHooks", &[]);
}
unsafe impl objc2::encode::RefEncode for TclStubs {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for TclStubs {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("TclStubs", &[]);
}
unsafe impl objc2::encode::RefEncode for TclPlatStubs {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for TclPlatStubs {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("TclPlatStubs", &[]);
}
unsafe impl objc2::encode::RefEncode for mp_int {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for mp_int {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("mp_int", &[]);
}
unsafe impl objc2::encode::RefEncode for TclIntStubs {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for TclIntStubs {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("TclIntStubs", &[]);
}
unsafe impl objc2::encode::RefEncode for TclIntPlatStubs {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for TclIntPlatStubs {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("TclIntPlatStubs", &[]);
}
unsafe impl objc2::encode::RefEncode for TclPlatStubHooks {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for TclPlatStubHooks {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("TclPlatStubHooks", &[]);
}
