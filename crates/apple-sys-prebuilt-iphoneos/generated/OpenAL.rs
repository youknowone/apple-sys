pub type ALboolean = ::std::os::raw::c_char;
pub type ALchar = ::std::os::raw::c_char;
pub type ALbyte = ::std::os::raw::c_char;
pub type ALubyte = ::std::os::raw::c_uchar;
pub type ALshort = ::std::os::raw::c_short;
pub type ALushort = ::std::os::raw::c_ushort;
pub type ALint = ::std::os::raw::c_int;
pub type ALuint = ::std::os::raw::c_uint;
pub type ALsizei = ::std::os::raw::c_int;
pub type ALenum = ::std::os::raw::c_int;
pub type ALfloat = f32;
pub type ALdouble = f64;
pub type ALvoid = ::std::os::raw::c_void;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ALCdevice_struct {
    _unused: [u8; 0],
}
pub type ALCdevice = ALCdevice_struct;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ALCcontext_struct {
    _unused: [u8; 0],
}
pub type ALCcontext = ALCcontext_struct;
pub type ALCboolean = ::std::os::raw::c_char;
pub type ALCchar = ::std::os::raw::c_char;
pub type ALCbyte = ::std::os::raw::c_char;
pub type ALCubyte = ::std::os::raw::c_uchar;
pub type ALCshort = ::std::os::raw::c_short;
pub type ALCushort = ::std::os::raw::c_ushort;
pub type ALCint = ::std::os::raw::c_int;
pub type ALCuint = ::std::os::raw::c_uint;
pub type ALCsizei = ::std::os::raw::c_int;
pub type ALCenum = ::std::os::raw::c_int;
pub type ALCfloat = f32;
pub type ALCdouble = f64;
pub type ALCvoid = ::std::os::raw::c_void;
unsafe extern "C" {
    pub fn alEnable(capability: ALenum);
}
unsafe extern "C" {
    pub fn alDisable(capability: ALenum);
}
unsafe extern "C" {
    pub fn alIsEnabled(capability: ALenum) -> ALboolean;
}
unsafe extern "C" {
    pub fn alGetString(param: ALenum) -> *const ALchar;
}
unsafe extern "C" {
    pub fn alGetBooleanv(param: ALenum, data: *mut ALboolean);
}
unsafe extern "C" {
    pub fn alGetIntegerv(param: ALenum, data: *mut ALint);
}
unsafe extern "C" {
    pub fn alGetFloatv(param: ALenum, data: *mut ALfloat);
}
unsafe extern "C" {
    pub fn alGetDoublev(param: ALenum, data: *mut ALdouble);
}
unsafe extern "C" {
    pub fn alGetBoolean(param: ALenum) -> ALboolean;
}
unsafe extern "C" {
    pub fn alGetInteger(param: ALenum) -> ALint;
}
unsafe extern "C" {
    pub fn alGetFloat(param: ALenum) -> ALfloat;
}
unsafe extern "C" {
    pub fn alGetDouble(param: ALenum) -> ALdouble;
}
unsafe extern "C" {
    pub fn alGetError() -> ALenum;
}
unsafe extern "C" {
    pub fn alIsExtensionPresent(extname: *const ALchar) -> ALboolean;
}
unsafe extern "C" {
    pub fn alGetProcAddress(fname: *const ALchar) -> *mut ::std::os::raw::c_void;
}
unsafe extern "C" {
    pub fn alGetEnumValue(ename: *const ALchar) -> ALenum;
}
unsafe extern "C" {
    pub fn alListenerf(param: ALenum, value: ALfloat);
}
unsafe extern "C" {
    pub fn alListener3f(param: ALenum, value1: ALfloat, value2: ALfloat, value3: ALfloat);
}
unsafe extern "C" {
    pub fn alListenerfv(param: ALenum, values: *const ALfloat);
}
unsafe extern "C" {
    pub fn alListeneri(param: ALenum, value: ALint);
}
unsafe extern "C" {
    pub fn alListener3i(param: ALenum, value1: ALint, value2: ALint, value3: ALint);
}
unsafe extern "C" {
    pub fn alListeneriv(param: ALenum, values: *const ALint);
}
unsafe extern "C" {
    pub fn alGetListenerf(param: ALenum, value: *mut ALfloat);
}
unsafe extern "C" {
    pub fn alGetListener3f(
        param: ALenum,
        value1: *mut ALfloat,
        value2: *mut ALfloat,
        value3: *mut ALfloat,
    );
}
unsafe extern "C" {
    pub fn alGetListenerfv(param: ALenum, values: *mut ALfloat);
}
unsafe extern "C" {
    pub fn alGetListeneri(param: ALenum, value: *mut ALint);
}
unsafe extern "C" {
    pub fn alGetListener3i(
        param: ALenum,
        value1: *mut ALint,
        value2: *mut ALint,
        value3: *mut ALint,
    );
}
unsafe extern "C" {
    pub fn alGetListeneriv(param: ALenum, values: *mut ALint);
}
unsafe extern "C" {
    pub fn alGenSources(n: ALsizei, sources: *mut ALuint);
}
unsafe extern "C" {
    pub fn alDeleteSources(n: ALsizei, sources: *const ALuint);
}
unsafe extern "C" {
    pub fn alIsSource(sid: ALuint) -> ALboolean;
}
unsafe extern "C" {
    pub fn alSourcef(sid: ALuint, param: ALenum, value: ALfloat);
}
unsafe extern "C" {
    pub fn alSource3f(
        sid: ALuint,
        param: ALenum,
        value1: ALfloat,
        value2: ALfloat,
        value3: ALfloat,
    );
}
unsafe extern "C" {
    pub fn alSourcefv(sid: ALuint, param: ALenum, values: *const ALfloat);
}
unsafe extern "C" {
    pub fn alSourcei(sid: ALuint, param: ALenum, value: ALint);
}
unsafe extern "C" {
    pub fn alSource3i(sid: ALuint, param: ALenum, value1: ALint, value2: ALint, value3: ALint);
}
unsafe extern "C" {
    pub fn alSourceiv(sid: ALuint, param: ALenum, values: *const ALint);
}
unsafe extern "C" {
    pub fn alGetSourcef(sid: ALuint, param: ALenum, value: *mut ALfloat);
}
unsafe extern "C" {
    pub fn alGetSource3f(
        sid: ALuint,
        param: ALenum,
        value1: *mut ALfloat,
        value2: *mut ALfloat,
        value3: *mut ALfloat,
    );
}
unsafe extern "C" {
    pub fn alGetSourcefv(sid: ALuint, param: ALenum, values: *mut ALfloat);
}
unsafe extern "C" {
    pub fn alGetSourcei(sid: ALuint, param: ALenum, value: *mut ALint);
}
unsafe extern "C" {
    pub fn alGetSource3i(
        sid: ALuint,
        param: ALenum,
        value1: *mut ALint,
        value2: *mut ALint,
        value3: *mut ALint,
    );
}
unsafe extern "C" {
    pub fn alGetSourceiv(sid: ALuint, param: ALenum, values: *mut ALint);
}
unsafe extern "C" {
    pub fn alSourcePlayv(ns: ALsizei, sids: *const ALuint);
}
unsafe extern "C" {
    pub fn alSourceStopv(ns: ALsizei, sids: *const ALuint);
}
unsafe extern "C" {
    pub fn alSourceRewindv(ns: ALsizei, sids: *const ALuint);
}
unsafe extern "C" {
    pub fn alSourcePausev(ns: ALsizei, sids: *const ALuint);
}
unsafe extern "C" {
    pub fn alSourcePlay(sid: ALuint);
}
unsafe extern "C" {
    pub fn alSourceStop(sid: ALuint);
}
unsafe extern "C" {
    pub fn alSourceRewind(sid: ALuint);
}
unsafe extern "C" {
    pub fn alSourcePause(sid: ALuint);
}
unsafe extern "C" {
    pub fn alSourceQueueBuffers(sid: ALuint, numEntries: ALsizei, bids: *const ALuint);
}
unsafe extern "C" {
    pub fn alSourceUnqueueBuffers(sid: ALuint, numEntries: ALsizei, bids: *mut ALuint);
}
unsafe extern "C" {
    pub fn alGenBuffers(n: ALsizei, buffers: *mut ALuint);
}
unsafe extern "C" {
    pub fn alDeleteBuffers(n: ALsizei, buffers: *const ALuint);
}
unsafe extern "C" {
    pub fn alIsBuffer(bid: ALuint) -> ALboolean;
}
unsafe extern "C" {
    pub fn alBufferData(
        bid: ALuint,
        format: ALenum,
        data: *const ALvoid,
        size: ALsizei,
        freq: ALsizei,
    );
}
unsafe extern "C" {
    pub fn alBufferf(bid: ALuint, param: ALenum, value: ALfloat);
}
unsafe extern "C" {
    pub fn alBuffer3f(
        bid: ALuint,
        param: ALenum,
        value1: ALfloat,
        value2: ALfloat,
        value3: ALfloat,
    );
}
unsafe extern "C" {
    pub fn alBufferfv(bid: ALuint, param: ALenum, values: *const ALfloat);
}
unsafe extern "C" {
    pub fn alBufferi(bid: ALuint, param: ALenum, value: ALint);
}
unsafe extern "C" {
    pub fn alBuffer3i(bid: ALuint, param: ALenum, value1: ALint, value2: ALint, value3: ALint);
}
unsafe extern "C" {
    pub fn alBufferiv(bid: ALuint, param: ALenum, values: *const ALint);
}
unsafe extern "C" {
    pub fn alGetBufferf(bid: ALuint, param: ALenum, value: *mut ALfloat);
}
unsafe extern "C" {
    pub fn alGetBuffer3f(
        bid: ALuint,
        param: ALenum,
        value1: *mut ALfloat,
        value2: *mut ALfloat,
        value3: *mut ALfloat,
    );
}
unsafe extern "C" {
    pub fn alGetBufferfv(bid: ALuint, param: ALenum, values: *mut ALfloat);
}
unsafe extern "C" {
    pub fn alGetBufferi(bid: ALuint, param: ALenum, value: *mut ALint);
}
unsafe extern "C" {
    pub fn alGetBuffer3i(
        bid: ALuint,
        param: ALenum,
        value1: *mut ALint,
        value2: *mut ALint,
        value3: *mut ALint,
    );
}
unsafe extern "C" {
    pub fn alGetBufferiv(bid: ALuint, param: ALenum, values: *mut ALint);
}
unsafe extern "C" {
    pub fn alDopplerFactor(value: ALfloat);
}
unsafe extern "C" {
    pub fn alDopplerVelocity(value: ALfloat);
}
unsafe extern "C" {
    pub fn alSpeedOfSound(value: ALfloat);
}
unsafe extern "C" {
    pub fn alDistanceModel(distanceModel: ALenum);
}
unsafe extern "C" {
    pub fn alcCreateContext(device: *mut ALCdevice, attrlist: *const ALCint) -> *mut ALCcontext;
}
unsafe extern "C" {
    pub fn alcMakeContextCurrent(context: *mut ALCcontext) -> ALCboolean;
}
unsafe extern "C" {
    pub fn alcProcessContext(context: *mut ALCcontext);
}
unsafe extern "C" {
    pub fn alcSuspendContext(context: *mut ALCcontext);
}
unsafe extern "C" {
    pub fn alcDestroyContext(context: *mut ALCcontext);
}
unsafe extern "C" {
    pub fn alcGetCurrentContext() -> *mut ALCcontext;
}
unsafe extern "C" {
    pub fn alcGetContextsDevice(context: *mut ALCcontext) -> *mut ALCdevice;
}
unsafe extern "C" {
    pub fn alcOpenDevice(devicename: *const ALCchar) -> *mut ALCdevice;
}
unsafe extern "C" {
    pub fn alcCloseDevice(device: *mut ALCdevice) -> ALCboolean;
}
unsafe extern "C" {
    pub fn alcGetError(device: *mut ALCdevice) -> ALCenum;
}
unsafe extern "C" {
    pub fn alcIsExtensionPresent(device: *mut ALCdevice, extname: *const ALCchar) -> ALCboolean;
}
unsafe extern "C" {
    pub fn alcGetProcAddress(
        device: *mut ALCdevice,
        funcname: *const ALCchar,
    ) -> *mut ::std::os::raw::c_void;
}
unsafe extern "C" {
    pub fn alcGetEnumValue(device: *mut ALCdevice, enumname: *const ALCchar) -> ALCenum;
}
unsafe extern "C" {
    pub fn alcGetString(device: *mut ALCdevice, param: ALCenum) -> *const ALCchar;
}
unsafe extern "C" {
    pub fn alcGetIntegerv(
        device: *mut ALCdevice,
        param: ALCenum,
        size: ALCsizei,
        data: *mut ALCint,
    );
}
unsafe extern "C" {
    pub fn alcCaptureOpenDevice(
        devicename: *const ALCchar,
        frequency: ALCuint,
        format: ALCenum,
        buffersize: ALCsizei,
    ) -> *mut ALCdevice;
}
unsafe extern "C" {
    pub fn alcCaptureCloseDevice(device: *mut ALCdevice) -> ALCboolean;
}
unsafe extern "C" {
    pub fn alcCaptureStart(device: *mut ALCdevice);
}
unsafe extern "C" {
    pub fn alcCaptureStop(device: *mut ALCdevice);
}
unsafe extern "C" {
    pub fn alcCaptureSamples(device: *mut ALCdevice, buffer: *mut ALCvoid, samples: ALCsizei);
}

unsafe impl objc2::encode::RefEncode for ALCdevice_struct {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for ALCdevice_struct {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("ALCdevice_struct", &[]);
}
unsafe impl objc2::encode::RefEncode for ALCcontext_struct {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for ALCcontext_struct {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("ALCcontext_struct", &[]);
}
