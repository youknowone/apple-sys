pub type TW_HANDLE = *mut *mut ::std::os::raw::c_char;
pub type TW_MEMREF = *mut ::std::os::raw::c_char;
pub type TW_STR32 = [::std::os::raw::c_uchar; 34usize];
pub type pTW_STR32 = *mut ::std::os::raw::c_uchar;
pub type pTW_STR64 = *mut ::std::os::raw::c_uchar;
pub type pTW_STR128 = *mut ::std::os::raw::c_uchar;
pub type TW_STR255 = [::std::os::raw::c_uchar; 256usize];
pub type pTW_STR255 = *mut ::std::os::raw::c_uchar;
pub type pTW_INT8 = *mut ::std::os::raw::c_char;
pub type TW_INT16 = ::std::os::raw::c_short;
pub type pTW_INT16 = *mut ::std::os::raw::c_short;
pub type TW_INT32 = ::std::os::raw::c_int;
pub type pTW_INT32 = *mut ::std::os::raw::c_int;
pub type TW_UINT8 = ::std::os::raw::c_uchar;
pub type pTW_UINT8 = *mut ::std::os::raw::c_uchar;
pub type TW_UINT16 = ::std::os::raw::c_ushort;
pub type pTW_UINT16 = *mut ::std::os::raw::c_ushort;
pub type TW_UINT32 = ::std::os::raw::c_uint;
pub type pTW_UINT32 = *mut ::std::os::raw::c_uint;
pub type TW_BOOL = ::std::os::raw::c_ushort;
pub type pTW_BOOL = *mut ::std::os::raw::c_ushort;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct TW_FIX32 {
    pub Whole: TW_INT16,
    pub Frac: TW_UINT16,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct TW_CIEPOINT {
    pub X: TW_FIX32,
    pub Y: TW_FIX32,
    pub Z: TW_FIX32,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct TW_DECODEFUNCTION {
    pub StartIn: TW_FIX32,
    pub BreakIn: TW_FIX32,
    pub EndIn: TW_FIX32,
    pub StartOut: TW_FIX32,
    pub BreakOut: TW_FIX32,
    pub EndOut: TW_FIX32,
    pub Gamma: TW_FIX32,
    pub SampleCount: TW_FIX32,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct TW_ELEMENT8 {
    pub Index: TW_UINT8,
    pub Channel1: TW_UINT8,
    pub Channel2: TW_UINT8,
    pub Channel3: TW_UINT8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct TW_FRAME {
    pub Left: TW_FIX32,
    pub Top: TW_FIX32,
    pub Right: TW_FIX32,
    pub Bottom: TW_FIX32,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct TW_MEMORY {
    pub Flags: TW_UINT32,
    pub Length: TW_UINT32,
    pub TheMem: TW_MEMREF,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct TW_TRANSFORMSTAGE {
    pub Decode: [TW_DECODEFUNCTION; 3usize],
    pub Mix: [[TW_FIX32; 3usize]; 3usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct TW_VERSION {
    pub MajorNum: TW_UINT16,
    pub MinorNum: TW_UINT16,
    pub Language: TW_UINT16,
    pub Country: TW_UINT16,
    pub Info: TW_STR32,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct TW_ARRAY {
    pub ItemType: TW_UINT16,
    pub NumItems: TW_UINT32,
    pub ItemList: [TW_UINT8; 1usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct TW_ENUMERATION {
    pub ItemType: TW_UINT16,
    pub NumItems: TW_UINT32,
    pub CurrentIndex: TW_UINT32,
    pub DefaultIndex: TW_UINT32,
    pub ItemList: [TW_UINT8; 1usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct TW_ONEVALUE {
    pub ItemType: TW_UINT16,
    pub Item: TW_UINT32,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct TW_RANGE {
    pub ItemType: TW_UINT16,
    pub MinValue: TW_UINT32,
    pub MaxValue: TW_UINT32,
    pub StepSize: TW_UINT32,
    pub DefaultValue: TW_UINT32,
    pub CurrentValue: TW_UINT32,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct TW_CAPABILITY {
    pub Cap: TW_UINT16,
    pub ConType: TW_UINT16,
    pub hContainer: TW_HANDLE,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct TW_CIECOLOR {
    pub ColorSpace: TW_UINT16,
    pub LowEndian: TW_INT16,
    pub DeviceDependent: TW_INT16,
    pub VersionNumber: TW_INT32,
    pub StageABC: TW_TRANSFORMSTAGE,
    pub StageLMN: TW_TRANSFORMSTAGE,
    pub WhitePoint: TW_CIEPOINT,
    pub BlackPoint: TW_CIEPOINT,
    pub WhitePaper: TW_CIEPOINT,
    pub BlackInk: TW_CIEPOINT,
    pub Samples: [TW_FIX32; 1usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct TW_EVENT {
    pub pEvent: TW_MEMREF,
    pub TWMessage: TW_UINT16,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct TW_GRAYRESPONSE {
    pub Response: [TW_ELEMENT8; 1usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct TW_IDENTITY {
    pub Id: TW_MEMREF,
    pub Version: TW_VERSION,
    pub ProtocolMajor: TW_UINT16,
    pub ProtocolMinor: TW_UINT16,
    pub SupportedGroups: TW_UINT32,
    pub Manufacturer: TW_STR32,
    pub ProductFamily: TW_STR32,
    pub ProductName: TW_STR32,
}
pub type pTW_IDENTITY = *mut TW_IDENTITY;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct TW_IMAGEINFO {
    pub XResolution: TW_FIX32,
    pub YResolution: TW_FIX32,
    pub ImageWidth: TW_INT32,
    pub ImageLength: TW_INT32,
    pub SamplesPerPixel: TW_INT16,
    pub BitsPerSample: [TW_INT16; 8usize],
    pub BitsPerPixel: TW_INT16,
    pub Planar: TW_BOOL,
    pub PixelType: TW_INT16,
    pub Compression: TW_UINT16,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct TW_IMAGELAYOUT {
    pub Frame: TW_FRAME,
    pub DocumentNumber: TW_UINT32,
    pub PageNumber: TW_UINT32,
    pub FrameNumber: TW_UINT32,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct TW_IMAGEMEMXFER {
    pub Compression: TW_UINT16,
    pub BytesPerRow: TW_UINT32,
    pub Columns: TW_UINT32,
    pub Rows: TW_UINT32,
    pub XOffset: TW_UINT32,
    pub YOffset: TW_UINT32,
    pub BytesWritten: TW_UINT32,
    pub Memory: TW_MEMORY,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct TW_JPEGCOMPRESSION {
    pub ColorSpace: TW_UINT16,
    pub SubSampling: TW_UINT32,
    pub NumComponents: TW_UINT16,
    pub RestartFrequency: TW_UINT16,
    pub QuantMap: [TW_UINT16; 4usize],
    pub QuantTable: [TW_MEMORY; 4usize],
    pub HuffmanMap: [TW_UINT16; 4usize],
    pub HuffmanDC: [TW_MEMORY; 2usize],
    pub HuffmanAC: [TW_MEMORY; 2usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct TW_PALETTE8 {
    pub NumColors: TW_UINT16,
    pub PaletteType: TW_UINT16,
    pub Colors: [TW_ELEMENT8; 256usize],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct TW_PENDINGXFERS {
    pub __bindgen_anon_1: TW_PENDINGXFERS__bindgen_ty_1,
    pub Count: TW_UINT16,
    pub TW_JOBCONTROL: TW_PENDINGXFERS__bindgen_ty_1,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union TW_PENDINGXFERS__bindgen_ty_1 {
    pub EOJ: TW_UINT32,
    pub Reserved: TW_UINT32,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct TW_RGBRESPONSE {
    pub Response: [TW_ELEMENT8; 1usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct TW_SETUPFILEXFER {
    pub FileName: TW_STR255,
    pub Format: TW_UINT16,
    pub VRefNum: TW_INT16,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct TW_SETUPFILEXFER2 {
    pub FileName: TW_MEMREF,
    pub FileNameType: TW_UINT16,
    pub Format: TW_UINT16,
    pub VRefNum: TW_INT16,
    pub ParID: TW_UINT32,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct TW_SETUPMEMXFER {
    pub MinBufSize: TW_UINT32,
    pub MaxBufSize: TW_UINT32,
    pub Preferred: TW_UINT32,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct TW_STATUS {
    pub ConditionCode: TW_UINT16,
    pub Reserved: TW_UINT16,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct TW_USERINTERFACE {
    pub ShowUI: TW_BOOL,
    pub ModalUI: TW_BOOL,
    pub hParent: TW_HANDLE,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct TW_TWUNKIDENTITY {
    pub identity: TW_IDENTITY,
    pub dsPath: TW_STR255,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct TW_CUSTOMDSDATA {
    pub InfoLength: TW_UINT32,
    pub hData: TW_HANDLE,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct TW_INFO {
    pub InfoID: TW_UINT16,
    pub ItemType: TW_UINT16,
    pub NumItems: TW_UINT16,
    pub CondCode: TW_UINT16,
    pub Item: TW_UINT32,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct TW_EXTIMAGEINFO {
    pub NumInfos: TW_UINT32,
    pub Info: [TW_INFO; 1usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct TW_AUDIOINFO {
    pub Name: TW_STR255,
    pub Reserved: TW_UINT32,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct TW_DEVICEEVENT {
    pub Event: TW_UINT32,
    pub DeviceName: TW_STR255,
    pub BatteryMinutes: TW_UINT32,
    pub BatteryPercentage: TW_INT16,
    pub PowerSupply: TW_INT32,
    pub XResolution: TW_FIX32,
    pub YResolution: TW_FIX32,
    pub FlashUsed2: TW_UINT32,
    pub AutomaticCapture: TW_UINT32,
    pub TimeBeforeFirstCapture: TW_UINT32,
    pub TimeBetweenCaptures: TW_UINT32,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct TW_FILESYSTEM {
    pub InputName: TW_STR255,
    pub OutputName: TW_STR255,
    pub Context: TW_MEMREF,
    pub Recursive: ::std::os::raw::c_int,
    pub FileType: TW_INT32,
    pub Size: TW_UINT32,
    pub CreateTimeDate: TW_STR32,
    pub ModifiedTimeDate: TW_STR32,
    pub FreeSpace: TW_UINT32,
    pub NewImageSize: TW_INT32,
    pub NumberOfFiles: TW_UINT32,
    pub NumberOfSnippets: TW_UINT32,
    pub DeviceGroupMask: TW_UINT32,
    pub Reserved: [::std::os::raw::c_char; 508usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct TW_PASSTHRU {
    pub pCommand: TW_MEMREF,
    pub CommandBytes: TW_UINT32,
    pub Direction: TW_INT32,
    pub pData: TW_MEMREF,
    pub DataBytes: TW_UINT32,
    pub DataBytesXfered: TW_UINT32,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct TW_SETUPAUDIOFILEXFER {
    pub FileName: TW_STR255,
    pub Format: TW_UINT16,
    pub VRefNum: TW_INT16,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct TW_CALLBACK {
    pub CallBackProc: TW_MEMREF,
    pub RefCon: TW_MEMREF,
    pub Message: TW_INT16,
}
unsafe extern "C" {
    pub fn DSM_Entry(
        pOrigin: pTW_IDENTITY,
        pDest: pTW_IDENTITY,
        DG: TW_UINT32,
        DAT: TW_UINT16,
        MSG: TW_UINT16,
        pData: TW_MEMREF,
    ) -> TW_UINT16;
}

unsafe impl objc2::encode::RefEncode for TW_FIX32 {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for TW_FIX32 {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("TW_FIX32", &[]);
}
unsafe impl objc2::encode::RefEncode for TW_CIEPOINT {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for TW_CIEPOINT {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("TW_CIEPOINT", &[]);
}
unsafe impl objc2::encode::RefEncode for TW_DECODEFUNCTION {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for TW_DECODEFUNCTION {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("TW_DECODEFUNCTION", &[]);
}
unsafe impl objc2::encode::RefEncode for TW_ELEMENT8 {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for TW_ELEMENT8 {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("TW_ELEMENT8", &[]);
}
unsafe impl objc2::encode::RefEncode for TW_FRAME {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for TW_FRAME {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("TW_FRAME", &[]);
}
unsafe impl objc2::encode::RefEncode for TW_MEMORY {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for TW_MEMORY {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("TW_MEMORY", &[]);
}
unsafe impl objc2::encode::RefEncode for TW_TRANSFORMSTAGE {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for TW_TRANSFORMSTAGE {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("TW_TRANSFORMSTAGE", &[]);
}
unsafe impl objc2::encode::RefEncode for TW_VERSION {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for TW_VERSION {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("TW_VERSION", &[]);
}
unsafe impl objc2::encode::RefEncode for TW_ARRAY {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for TW_ARRAY {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("TW_ARRAY", &[]);
}
unsafe impl objc2::encode::RefEncode for TW_ENUMERATION {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for TW_ENUMERATION {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("TW_ENUMERATION", &[]);
}
unsafe impl objc2::encode::RefEncode for TW_ONEVALUE {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for TW_ONEVALUE {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("TW_ONEVALUE", &[]);
}
unsafe impl objc2::encode::RefEncode for TW_RANGE {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for TW_RANGE {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("TW_RANGE", &[]);
}
unsafe impl objc2::encode::RefEncode for TW_CAPABILITY {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for TW_CAPABILITY {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("TW_CAPABILITY", &[]);
}
unsafe impl objc2::encode::RefEncode for TW_CIECOLOR {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for TW_CIECOLOR {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("TW_CIECOLOR", &[]);
}
unsafe impl objc2::encode::RefEncode for TW_EVENT {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for TW_EVENT {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("TW_EVENT", &[]);
}
unsafe impl objc2::encode::RefEncode for TW_GRAYRESPONSE {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for TW_GRAYRESPONSE {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("TW_GRAYRESPONSE", &[]);
}
unsafe impl objc2::encode::RefEncode for TW_IDENTITY {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for TW_IDENTITY {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("TW_IDENTITY", &[]);
}
unsafe impl objc2::encode::RefEncode for TW_IMAGEINFO {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for TW_IMAGEINFO {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("TW_IMAGEINFO", &[]);
}
unsafe impl objc2::encode::RefEncode for TW_IMAGELAYOUT {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for TW_IMAGELAYOUT {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("TW_IMAGELAYOUT", &[]);
}
unsafe impl objc2::encode::RefEncode for TW_IMAGEMEMXFER {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for TW_IMAGEMEMXFER {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("TW_IMAGEMEMXFER", &[]);
}
unsafe impl objc2::encode::RefEncode for TW_JPEGCOMPRESSION {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for TW_JPEGCOMPRESSION {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("TW_JPEGCOMPRESSION", &[]);
}
unsafe impl objc2::encode::RefEncode for TW_PALETTE8 {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for TW_PALETTE8 {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("TW_PALETTE8", &[]);
}
unsafe impl objc2::encode::RefEncode for TW_PENDINGXFERS {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for TW_PENDINGXFERS {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("TW_PENDINGXFERS", &[]);
}
unsafe impl objc2::encode::RefEncode for TW_PENDINGXFERS__bindgen_ty_1 {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for TW_PENDINGXFERS__bindgen_ty_1 {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("TW_PENDINGXFERS__bindgen_ty_1", &[]);
}
unsafe impl objc2::encode::RefEncode for TW_RGBRESPONSE {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for TW_RGBRESPONSE {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("TW_RGBRESPONSE", &[]);
}
unsafe impl objc2::encode::RefEncode for TW_SETUPFILEXFER {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for TW_SETUPFILEXFER {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("TW_SETUPFILEXFER", &[]);
}
unsafe impl objc2::encode::RefEncode for TW_SETUPFILEXFER2 {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for TW_SETUPFILEXFER2 {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("TW_SETUPFILEXFER2", &[]);
}
unsafe impl objc2::encode::RefEncode for TW_SETUPMEMXFER {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for TW_SETUPMEMXFER {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("TW_SETUPMEMXFER", &[]);
}
unsafe impl objc2::encode::RefEncode for TW_STATUS {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for TW_STATUS {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("TW_STATUS", &[]);
}
unsafe impl objc2::encode::RefEncode for TW_USERINTERFACE {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for TW_USERINTERFACE {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("TW_USERINTERFACE", &[]);
}
unsafe impl objc2::encode::RefEncode for TW_TWUNKIDENTITY {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for TW_TWUNKIDENTITY {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("TW_TWUNKIDENTITY", &[]);
}
unsafe impl objc2::encode::RefEncode for TW_CUSTOMDSDATA {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for TW_CUSTOMDSDATA {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("TW_CUSTOMDSDATA", &[]);
}
unsafe impl objc2::encode::RefEncode for TW_INFO {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for TW_INFO {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("TW_INFO", &[]);
}
unsafe impl objc2::encode::RefEncode for TW_EXTIMAGEINFO {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for TW_EXTIMAGEINFO {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("TW_EXTIMAGEINFO", &[]);
}
unsafe impl objc2::encode::RefEncode for TW_AUDIOINFO {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for TW_AUDIOINFO {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("TW_AUDIOINFO", &[]);
}
unsafe impl objc2::encode::RefEncode for TW_DEVICEEVENT {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for TW_DEVICEEVENT {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("TW_DEVICEEVENT", &[]);
}
unsafe impl objc2::encode::RefEncode for TW_FILESYSTEM {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for TW_FILESYSTEM {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("TW_FILESYSTEM", &[]);
}
unsafe impl objc2::encode::RefEncode for TW_PASSTHRU {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for TW_PASSTHRU {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("TW_PASSTHRU", &[]);
}
unsafe impl objc2::encode::RefEncode for TW_SETUPAUDIOFILEXFER {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for TW_SETUPAUDIOFILEXFER {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("TW_SETUPAUDIOFILEXFER", &[]);
}
unsafe impl objc2::encode::RefEncode for TW_CALLBACK {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for TW_CALLBACK {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("TW_CALLBACK", &[]);
}
