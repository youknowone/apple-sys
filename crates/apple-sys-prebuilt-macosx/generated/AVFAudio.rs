#[allow(unused_imports)]
use crate::objc::*;
#[allow(unused_imports)]
use crate::AppKit::*;
#[allow(unused_imports)]
use crate::AudioToolbox::*;
#[allow(unused_imports)]
use crate::CoreAudio::*;
#[allow(unused_imports)]
use crate::CoreFoundation::*;
#[allow(unused_imports)]
use crate::CoreMIDI::*;
#[allow(unused_imports)]
use crate::CoreMedia::*;
#[allow(unused_imports)]
use crate::Foundation::*;
#[allow(unused_imports)]
use crate::IOKit::*;

#[allow(unused_imports)]
use objc2::msg_send;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AudioStreamPacketDependencyDescription {
    pub mIsIndependentlyDecodable: UInt32,
    pub mPreRollCount: UInt32,
    pub mFlags: UInt32,
    pub mReserved: UInt32,
}
pub type AVAudioFramePosition = i64;
pub type AVAudioFrameCount = u32;
pub type AVAudioPacketCount = u32;
pub type AVAudioChannelCount = u32;
pub type AVAudioNodeCompletionHandler = *mut ::std::os::raw::c_void;
pub type AVAudioNodeBus = NSUInteger;
pub type AVMusicTimeStamp = f64;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AVAudio3DPoint {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}
pub type AVAudio3DVector = AVAudio3DPoint;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AVAudio3DVectorOrientation {
    pub forward: AVAudio3DVector,
    pub up: AVAudio3DVector,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AVAudio3DAngularOrientation {
    pub yaw: f32,
    pub pitch: f32,
    pub roll: f32,
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct AVAudioBuffer(pub id);
impl std::ops::Deref for AVAudioBuffer {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for AVAudioBuffer {}
impl AVAudioBuffer {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"AVAudioBuffer").unwrap(), alloc) })
    }
}
impl PNSCopying for AVAudioBuffer {}
impl PNSMutableCopying for AVAudioBuffer {}
impl INSObject for AVAudioBuffer {}
impl PNSObject for AVAudioBuffer {}
impl std::convert::TryFrom<NSObject> for AVAudioBuffer {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<AVAudioBuffer, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"AVAudioBuffer").unwrap()) };
        if is_kind_of {
            Ok(AVAudioBuffer(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to AVAudioBuffer")
        }
    }
}
impl IAVAudioBuffer for AVAudioBuffer {}
pub trait IAVAudioBuffer: Sized + std::ops::Deref {
    unsafe fn format(&self) -> AVAudioFormat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, format)
    }
    unsafe fn audioBufferList(&self) -> *const AudioBufferList
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, audioBufferList)
    }
    unsafe fn mutableAudioBufferList(&self) -> *mut AudioBufferList
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, mutableAudioBufferList)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct AVAudioPCMBuffer(pub id);
impl std::ops::Deref for AVAudioPCMBuffer {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for AVAudioPCMBuffer {}
impl AVAudioPCMBuffer {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"AVAudioPCMBuffer").unwrap(), alloc) })
    }
}
impl IAVAudioBuffer for AVAudioPCMBuffer {}
impl PNSCopying for AVAudioPCMBuffer {}
impl PNSMutableCopying for AVAudioPCMBuffer {}
impl From<AVAudioPCMBuffer> for AVAudioBuffer {
    fn from(child: AVAudioPCMBuffer) -> AVAudioBuffer {
        AVAudioBuffer(child.0)
    }
}
impl std::convert::TryFrom<AVAudioBuffer> for AVAudioPCMBuffer {
    type Error = &'static str;
    fn try_from(parent: AVAudioBuffer) -> Result<AVAudioPCMBuffer, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"AVAudioPCMBuffer").unwrap()) };
        if is_kind_of {
            Ok(AVAudioPCMBuffer(parent.0))
        } else {
            Err("This AVAudioBuffer cannot be downcasted to AVAudioPCMBuffer")
        }
    }
}
impl INSObject for AVAudioPCMBuffer {}
impl PNSObject for AVAudioPCMBuffer {}
impl IAVAudioPCMBuffer for AVAudioPCMBuffer {}
pub trait IAVAudioPCMBuffer: Sized + std::ops::Deref {
    unsafe fn initWithPCMFormat_frameCapacity_(
        &self,
        format: AVAudioFormat,
        frameCapacity: AVAudioFrameCount,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithPCMFormat : format, frameCapacity : frameCapacity)
    }
    unsafe fn initWithPCMFormat_bufferListNoCopy_deallocator_(
        &self,
        format: AVAudioFormat,
        bufferList: *const AudioBufferList,
        deallocator: *mut ::std::os::raw::c_void,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithPCMFormat : format, bufferListNoCopy : bufferList, deallocator : deallocator)
    }
    unsafe fn frameCapacity(&self) -> AVAudioFrameCount
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, frameCapacity)
    }
    unsafe fn frameLength(&self) -> AVAudioFrameCount
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, frameLength)
    }
    unsafe fn setFrameLength_(&self, frameLength: AVAudioFrameCount)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setFrameLength : frameLength)
    }
    unsafe fn stride(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, stride)
    }
    unsafe fn floatChannelData(&self) -> *const *mut f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, floatChannelData)
    }
    unsafe fn int16ChannelData(&self) -> *const *mut i16
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, int16ChannelData)
    }
    unsafe fn int32ChannelData(&self) -> *const *mut i32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, int32ChannelData)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct AVAudioCompressedBuffer(pub id);
impl std::ops::Deref for AVAudioCompressedBuffer {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for AVAudioCompressedBuffer {}
impl AVAudioCompressedBuffer {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"AVAudioCompressedBuffer").unwrap(), alloc) })
    }
}
impl IAVAudioBuffer for AVAudioCompressedBuffer {}
impl PNSCopying for AVAudioCompressedBuffer {}
impl PNSMutableCopying for AVAudioCompressedBuffer {}
impl std::convert::TryFrom<AVAudioBuffer> for AVAudioCompressedBuffer {
    type Error = &'static str;
    fn try_from(parent: AVAudioBuffer) -> Result<AVAudioCompressedBuffer, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"AVAudioCompressedBuffer").unwrap()) };
        if is_kind_of {
            Ok(AVAudioCompressedBuffer(parent.0))
        } else {
            Err("This AVAudioBuffer cannot be downcasted to AVAudioCompressedBuffer")
        }
    }
}
impl INSObject for AVAudioCompressedBuffer {}
impl PNSObject for AVAudioCompressedBuffer {}
impl IAVAudioCompressedBuffer for AVAudioCompressedBuffer {}
pub trait IAVAudioCompressedBuffer: Sized + std::ops::Deref {
    unsafe fn initWithFormat_packetCapacity_maximumPacketSize_(
        &self,
        format: AVAudioFormat,
        packetCapacity: AVAudioPacketCount,
        maximumPacketSize: NSInteger,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithFormat : format, packetCapacity : packetCapacity, maximumPacketSize : maximumPacketSize)
    }
    unsafe fn initWithFormat_packetCapacity_(
        &self,
        format: AVAudioFormat,
        packetCapacity: AVAudioPacketCount,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithFormat : format, packetCapacity : packetCapacity)
    }
    unsafe fn packetCapacity(&self) -> AVAudioPacketCount
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, packetCapacity)
    }
    unsafe fn packetCount(&self) -> AVAudioPacketCount
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, packetCount)
    }
    unsafe fn setPacketCount_(&self, packetCount: AVAudioPacketCount)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPacketCount : packetCount)
    }
    unsafe fn maximumPacketSize(&self) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, maximumPacketSize)
    }
    unsafe fn data(&self) -> *mut ::std::os::raw::c_void
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, data)
    }
    unsafe fn byteCapacity(&self) -> u32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, byteCapacity)
    }
    unsafe fn byteLength(&self) -> u32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, byteLength)
    }
    unsafe fn setByteLength_(&self, byteLength: u32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setByteLength : byteLength)
    }
    unsafe fn packetDescriptions(&self) -> *mut AudioStreamPacketDescription
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, packetDescriptions)
    }
    unsafe fn packetDependencies(&self) -> *mut AudioStreamPacketDependencyDescription
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, packetDependencies)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct AVAudioChannelLayout(pub id);
impl std::ops::Deref for AVAudioChannelLayout {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for AVAudioChannelLayout {}
impl AVAudioChannelLayout {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"AVAudioChannelLayout").unwrap(), alloc) })
    }
}
impl PNSSecureCoding for AVAudioChannelLayout {}
impl INSObject for AVAudioChannelLayout {}
impl PNSObject for AVAudioChannelLayout {}
impl std::convert::TryFrom<NSObject> for AVAudioChannelLayout {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<AVAudioChannelLayout, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"AVAudioChannelLayout").unwrap()) };
        if is_kind_of {
            Ok(AVAudioChannelLayout(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to AVAudioChannelLayout")
        }
    }
}
impl IAVAudioChannelLayout for AVAudioChannelLayout {}
pub trait IAVAudioChannelLayout: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn initWithLayoutTag_(&self, layoutTag: AudioChannelLayoutTag) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithLayoutTag : layoutTag)
    }
    unsafe fn initWithLayout_(&self, layout: *const AudioChannelLayout) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithLayout : layout)
    }
    unsafe fn isEqual_(&self, object: id) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, isEqual : object)
    }
    unsafe fn layoutTag(&self) -> AudioChannelLayoutTag
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, layoutTag)
    }
    unsafe fn layout(&self) -> *const AudioChannelLayout
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, layout)
    }
    unsafe fn channelCount(&self) -> AVAudioChannelCount
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, channelCount)
    }
    unsafe fn layoutWithLayoutTag_(layoutTag: AudioChannelLayoutTag) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"AVAudioChannelLayout").unwrap(), layoutWithLayoutTag : layoutTag)
    }
    unsafe fn layoutWithLayout_(layout: *const AudioChannelLayout) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"AVAudioChannelLayout").unwrap(), layoutWithLayout : layout)
    }
}
pub type AVAudioCommonFormat = NSUInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct AVAudioFormat(pub id);
impl std::ops::Deref for AVAudioFormat {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for AVAudioFormat {}
impl AVAudioFormat {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"AVAudioFormat").unwrap(), alloc) })
    }
}
impl PNSSecureCoding for AVAudioFormat {}
impl INSObject for AVAudioFormat {}
impl PNSObject for AVAudioFormat {}
impl std::convert::TryFrom<NSObject> for AVAudioFormat {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<AVAudioFormat, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"AVAudioFormat").unwrap()) };
        if is_kind_of {
            Ok(AVAudioFormat(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to AVAudioFormat")
        }
    }
}
impl IAVAudioFormat for AVAudioFormat {}
pub trait IAVAudioFormat: Sized + std::ops::Deref {
    unsafe fn initWithStreamDescription_(
        &self,
        asbd: *const AudioStreamBasicDescription,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithStreamDescription : asbd)
    }
    unsafe fn initWithStreamDescription_channelLayout_(
        &self,
        asbd: *const AudioStreamBasicDescription,
        layout: AVAudioChannelLayout,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithStreamDescription : asbd, channelLayout : layout)
    }
    unsafe fn initStandardFormatWithSampleRate_channels_(
        &self,
        sampleRate: f64,
        channels: AVAudioChannelCount,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initStandardFormatWithSampleRate : sampleRate, channels : channels)
    }
    unsafe fn initStandardFormatWithSampleRate_channelLayout_(
        &self,
        sampleRate: f64,
        layout: AVAudioChannelLayout,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initStandardFormatWithSampleRate : sampleRate, channelLayout : layout)
    }
    unsafe fn initWithCommonFormat_sampleRate_channels_interleaved_(
        &self,
        format: AVAudioCommonFormat,
        sampleRate: f64,
        channels: AVAudioChannelCount,
        interleaved: BOOL,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithCommonFormat : format, sampleRate : sampleRate, channels : channels, interleaved : interleaved)
    }
    unsafe fn initWithCommonFormat_sampleRate_interleaved_channelLayout_(
        &self,
        format: AVAudioCommonFormat,
        sampleRate: f64,
        interleaved: BOOL,
        layout: AVAudioChannelLayout,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithCommonFormat : format, sampleRate : sampleRate, interleaved : interleaved, channelLayout : layout)
    }
    unsafe fn initWithSettings_(&self, settings: NSDictionary) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithSettings : settings)
    }
    unsafe fn initWithCMAudioFormatDescription_(
        &self,
        formatDescription: CMAudioFormatDescriptionRef,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithCMAudioFormatDescription : formatDescription)
    }
    unsafe fn isEqual_(&self, object: id) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, isEqual : object)
    }
    unsafe fn isStandard(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isStandard)
    }
    unsafe fn commonFormat(&self) -> AVAudioCommonFormat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, commonFormat)
    }
    unsafe fn channelCount(&self) -> AVAudioChannelCount
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, channelCount)
    }
    unsafe fn sampleRate(&self) -> f64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, sampleRate)
    }
    unsafe fn isInterleaved(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isInterleaved)
    }
    unsafe fn streamDescription(&self) -> *const AudioStreamBasicDescription
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, streamDescription)
    }
    unsafe fn channelLayout(&self) -> AVAudioChannelLayout
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, channelLayout)
    }
    unsafe fn magicCookie(&self) -> NSData
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, magicCookie)
    }
    unsafe fn setMagicCookie_(&self, magicCookie: NSData)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMagicCookie : magicCookie)
    }
    unsafe fn settings(&self) -> NSDictionary
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, settings)
    }
    unsafe fn formatDescription(&self) -> CMAudioFormatDescriptionRef
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, formatDescription)
    }
}
pub type AVAudioQuality = NSInteger;
pub type AVAudioDynamicRangeControlConfiguration = NSInteger;
pub type AVAudioContentSource = NSInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct AVAudioTime(pub id);
impl std::ops::Deref for AVAudioTime {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for AVAudioTime {}
impl AVAudioTime {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"AVAudioTime").unwrap(), alloc) })
    }
}
impl INSObject for AVAudioTime {}
impl PNSObject for AVAudioTime {}
impl std::convert::TryFrom<NSObject> for AVAudioTime {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<AVAudioTime, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"AVAudioTime").unwrap()) };
        if is_kind_of {
            Ok(AVAudioTime(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to AVAudioTime")
        }
    }
}
impl IAVAudioTime for AVAudioTime {}
pub trait IAVAudioTime: Sized + std::ops::Deref {
    unsafe fn initWithAudioTimeStamp_sampleRate_(
        &self,
        ts: *const AudioTimeStamp,
        sampleRate: f64,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithAudioTimeStamp : ts, sampleRate : sampleRate)
    }
    unsafe fn initWithHostTime_(&self, hostTime: u64) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithHostTime : hostTime)
    }
    unsafe fn initWithSampleTime_atRate_(
        &self,
        sampleTime: AVAudioFramePosition,
        sampleRate: f64,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithSampleTime : sampleTime, atRate : sampleRate)
    }
    unsafe fn initWithHostTime_sampleTime_atRate_(
        &self,
        hostTime: u64,
        sampleTime: AVAudioFramePosition,
        sampleRate: f64,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithHostTime : hostTime, sampleTime : sampleTime, atRate : sampleRate)
    }
    unsafe fn extrapolateTimeFromAnchor_(&self, anchorTime: AVAudioTime) -> AVAudioTime
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, extrapolateTimeFromAnchor : anchorTime)
    }
    unsafe fn isHostTimeValid(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isHostTimeValid)
    }
    unsafe fn hostTime(&self) -> u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, hostTime)
    }
    unsafe fn isSampleTimeValid(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isSampleTimeValid)
    }
    unsafe fn sampleTime(&self) -> AVAudioFramePosition
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, sampleTime)
    }
    unsafe fn sampleRate(&self) -> f64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, sampleRate)
    }
    unsafe fn audioTimeStamp(&self) -> AudioTimeStamp
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, audioTimeStamp)
    }
    unsafe fn timeWithAudioTimeStamp_sampleRate_(
        ts: *const AudioTimeStamp,
        sampleRate: f64,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"AVAudioTime").unwrap(), timeWithAudioTimeStamp : ts, sampleRate : sampleRate)
    }
    unsafe fn timeWithHostTime_(hostTime: u64) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"AVAudioTime").unwrap(), timeWithHostTime : hostTime)
    }
    unsafe fn timeWithSampleTime_atRate_(
        sampleTime: AVAudioFramePosition,
        sampleRate: f64,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"AVAudioTime").unwrap(), timeWithSampleTime : sampleTime, atRate : sampleRate)
    }
    unsafe fn timeWithHostTime_sampleTime_atRate_(
        hostTime: u64,
        sampleTime: AVAudioFramePosition,
        sampleRate: f64,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"AVAudioTime").unwrap(), timeWithHostTime : hostTime, sampleTime : sampleTime, atRate : sampleRate)
    }
    unsafe fn hostTimeForSeconds_(seconds: NSTimeInterval) -> u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"AVAudioTime").unwrap(), hostTimeForSeconds : seconds)
    }
    unsafe fn secondsForHostTime_(hostTime: u64) -> NSTimeInterval
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"AVAudioTime").unwrap(), secondsForHostTime : hostTime)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct AVAudioConnectionPoint(pub id);
impl std::ops::Deref for AVAudioConnectionPoint {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for AVAudioConnectionPoint {}
impl AVAudioConnectionPoint {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"AVAudioConnectionPoint").unwrap(), alloc) })
    }
}
impl INSObject for AVAudioConnectionPoint {}
impl PNSObject for AVAudioConnectionPoint {}
impl std::convert::TryFrom<NSObject> for AVAudioConnectionPoint {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<AVAudioConnectionPoint, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"AVAudioConnectionPoint").unwrap()) };
        if is_kind_of {
            Ok(AVAudioConnectionPoint(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to AVAudioConnectionPoint")
        }
    }
}
impl IAVAudioConnectionPoint for AVAudioConnectionPoint {}
pub trait IAVAudioConnectionPoint: Sized + std::ops::Deref {
    unsafe fn initWithNode_bus_(&self, node: AVAudioNode, bus: AVAudioNodeBus) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithNode : node, bus : bus)
    }
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn node(&self) -> AVAudioNode
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, node)
    }
    unsafe fn bus(&self) -> AVAudioNodeBus
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, bus)
    }
}
pub type AVAudioConverterPrimeMethod = NSInteger;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AVAudioConverterPrimeInfo {
    pub leadingFrames: AVAudioFrameCount,
    pub trailingFrames: AVAudioFrameCount,
}
pub type AVAudioConverterInputStatus = NSInteger;
pub type AVAudioConverterOutputStatus = NSInteger;
pub type AVAudioConverterInputBlock = *mut ::std::os::raw::c_void;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct AVAudioConverter(pub id);
impl std::ops::Deref for AVAudioConverter {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for AVAudioConverter {}
impl AVAudioConverter {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"AVAudioConverter").unwrap(), alloc) })
    }
}
impl INSObject for AVAudioConverter {}
impl PNSObject for AVAudioConverter {}
impl std::convert::TryFrom<NSObject> for AVAudioConverter {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<AVAudioConverter, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"AVAudioConverter").unwrap()) };
        if is_kind_of {
            Ok(AVAudioConverter(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to AVAudioConverter")
        }
    }
}
impl IAVAudioConverter for AVAudioConverter {}
pub trait IAVAudioConverter: Sized + std::ops::Deref {
    unsafe fn initFromFormat_toFormat_(
        &self,
        fromFormat: AVAudioFormat,
        toFormat: AVAudioFormat,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initFromFormat : fromFormat, toFormat : toFormat)
    }
    unsafe fn reset(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, reset)
    }
    unsafe fn convertToBuffer_fromBuffer_error_(
        &self,
        outputBuffer: AVAudioPCMBuffer,
        inputBuffer: AVAudioPCMBuffer,
        outError: *mut NSError,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, convertToBuffer : outputBuffer, fromBuffer : inputBuffer, error : outError)
    }
    unsafe fn convertToBuffer_error_withInputFromBlock_(
        &self,
        outputBuffer: AVAudioBuffer,
        outError: *mut NSError,
        inputBlock: AVAudioConverterInputBlock,
    ) -> AVAudioConverterOutputStatus
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, convertToBuffer : outputBuffer, error : outError, withInputFromBlock : inputBlock)
    }
    unsafe fn inputFormat(&self) -> AVAudioFormat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inputFormat)
    }
    unsafe fn outputFormat(&self) -> AVAudioFormat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, outputFormat)
    }
    unsafe fn channelMap(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, channelMap)
    }
    unsafe fn setChannelMap_(&self, channelMap: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setChannelMap : channelMap)
    }
    unsafe fn magicCookie(&self) -> NSData
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, magicCookie)
    }
    unsafe fn setMagicCookie_(&self, magicCookie: NSData)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMagicCookie : magicCookie)
    }
    unsafe fn downmix(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, downmix)
    }
    unsafe fn setDownmix_(&self, downmix: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDownmix : downmix)
    }
    unsafe fn dither(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, dither)
    }
    unsafe fn setDither_(&self, dither: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDither : dither)
    }
    unsafe fn sampleRateConverterQuality(&self) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, sampleRateConverterQuality)
    }
    unsafe fn setSampleRateConverterQuality_(&self, sampleRateConverterQuality: NSInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSampleRateConverterQuality : sampleRateConverterQuality)
    }
    unsafe fn sampleRateConverterAlgorithm(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, sampleRateConverterAlgorithm)
    }
    unsafe fn setSampleRateConverterAlgorithm_(&self, sampleRateConverterAlgorithm: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSampleRateConverterAlgorithm : sampleRateConverterAlgorithm)
    }
    unsafe fn primeMethod(&self) -> AVAudioConverterPrimeMethod
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, primeMethod)
    }
    unsafe fn setPrimeMethod_(&self, primeMethod: AVAudioConverterPrimeMethod)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPrimeMethod : primeMethod)
    }
    unsafe fn primeInfo(&self) -> AVAudioConverterPrimeInfo
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, primeInfo)
    }
    unsafe fn setPrimeInfo_(&self, primeInfo: AVAudioConverterPrimeInfo)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPrimeInfo : primeInfo)
    }
    unsafe fn audioSyncPacketFrequency(&self) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, audioSyncPacketFrequency)
    }
    unsafe fn setAudioSyncPacketFrequency_(&self, audioSyncPacketFrequency: NSInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAudioSyncPacketFrequency : audioSyncPacketFrequency)
    }
    unsafe fn contentSource(&self) -> AVAudioContentSource
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, contentSource)
    }
    unsafe fn setContentSource_(&self, contentSource: AVAudioContentSource)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setContentSource : contentSource)
    }
    unsafe fn dynamicRangeControlConfiguration(&self) -> AVAudioDynamicRangeControlConfiguration
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, dynamicRangeControlConfiguration)
    }
    unsafe fn setDynamicRangeControlConfiguration_(
        &self,
        dynamicRangeControlConfiguration: AVAudioDynamicRangeControlConfiguration,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDynamicRangeControlConfiguration : dynamicRangeControlConfiguration)
    }
}
impl AVAudioConverter_Encoding for AVAudioConverter {}
pub trait AVAudioConverter_Encoding: Sized + std::ops::Deref {
    unsafe fn bitRate(&self) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, bitRate)
    }
    unsafe fn setBitRate_(&self, bitRate: NSInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setBitRate : bitRate)
    }
    unsafe fn bitRateStrategy(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, bitRateStrategy)
    }
    unsafe fn setBitRateStrategy_(&self, bitRateStrategy: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setBitRateStrategy : bitRateStrategy)
    }
    unsafe fn maximumOutputPacketSize(&self) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, maximumOutputPacketSize)
    }
    unsafe fn availableEncodeBitRates(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, availableEncodeBitRates)
    }
    unsafe fn applicableEncodeBitRates(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, applicableEncodeBitRates)
    }
    unsafe fn availableEncodeSampleRates(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, availableEncodeSampleRates)
    }
    unsafe fn applicableEncodeSampleRates(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, applicableEncodeSampleRates)
    }
    unsafe fn availableEncodeChannelLayoutTags(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, availableEncodeChannelLayoutTags)
    }
}
pub type AVAudioNodeTapBlock = *mut ::std::os::raw::c_void;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct AVAudioNode(pub id);
impl std::ops::Deref for AVAudioNode {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for AVAudioNode {}
impl AVAudioNode {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"AVAudioNode").unwrap(), alloc) })
    }
}
impl INSObject for AVAudioNode {}
impl PNSObject for AVAudioNode {}
impl std::convert::TryFrom<NSObject> for AVAudioNode {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<AVAudioNode, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"AVAudioNode").unwrap()) };
        if is_kind_of {
            Ok(AVAudioNode(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to AVAudioNode")
        }
    }
}
impl IAVAudioNode for AVAudioNode {}
pub trait IAVAudioNode: Sized + std::ops::Deref {
    unsafe fn reset(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, reset)
    }
    unsafe fn inputFormatForBus_(&self, bus: AVAudioNodeBus) -> AVAudioFormat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, inputFormatForBus : bus)
    }
    unsafe fn outputFormatForBus_(&self, bus: AVAudioNodeBus) -> AVAudioFormat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, outputFormatForBus : bus)
    }
    unsafe fn nameForInputBus_(&self, bus: AVAudioNodeBus) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, nameForInputBus : bus)
    }
    unsafe fn nameForOutputBus_(&self, bus: AVAudioNodeBus) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, nameForOutputBus : bus)
    }
    unsafe fn installTapOnBus_bufferSize_format_block_(
        &self,
        bus: AVAudioNodeBus,
        bufferSize: AVAudioFrameCount,
        format: AVAudioFormat,
        tapBlock: AVAudioNodeTapBlock,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, installTapOnBus : bus, bufferSize : bufferSize, format : format, block : tapBlock)
    }
    unsafe fn removeTapOnBus_(&self, bus: AVAudioNodeBus)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, removeTapOnBus : bus)
    }
    unsafe fn engine(&self) -> AVAudioEngine
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, engine)
    }
    unsafe fn numberOfInputs(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, numberOfInputs)
    }
    unsafe fn numberOfOutputs(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, numberOfOutputs)
    }
    unsafe fn lastRenderTime(&self) -> AVAudioTime
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, lastRenderTime)
    }
    unsafe fn AUAudioUnit(&self) -> AUAudioUnit
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, AUAudioUnit)
    }
    unsafe fn latency(&self) -> NSTimeInterval
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, latency)
    }
    unsafe fn outputPresentationLatency(&self) -> NSTimeInterval
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, outputPresentationLatency)
    }
}
pub trait PAVAudioMixing: Sized + std::ops::Deref {
    unsafe fn destinationForMixer_bus_(
        &self,
        mixer: AVAudioNode,
        bus: AVAudioNodeBus,
    ) -> AVAudioMixingDestination
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, destinationForMixer : mixer, bus : bus)
    }
    unsafe fn volume(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, volume)
    }
    unsafe fn setVolume_(&self, volume: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setVolume : volume)
    }
}
pub trait PAVAudioStereoMixing: Sized + std::ops::Deref {
    unsafe fn pan(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, pan)
    }
    unsafe fn setPan_(&self, pan: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPan : pan)
    }
}
pub type AVAudio3DMixingRenderingAlgorithm = NSInteger;
pub type AVAudio3DMixingSourceMode = NSInteger;
pub type AVAudio3DMixingPointSourceInHeadMode = NSInteger;
pub trait PAVAudio3DMixing: Sized + std::ops::Deref {
    unsafe fn renderingAlgorithm(&self) -> AVAudio3DMixingRenderingAlgorithm
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, renderingAlgorithm)
    }
    unsafe fn setRenderingAlgorithm_(&self, renderingAlgorithm: AVAudio3DMixingRenderingAlgorithm)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRenderingAlgorithm : renderingAlgorithm)
    }
    unsafe fn sourceMode(&self) -> AVAudio3DMixingSourceMode
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, sourceMode)
    }
    unsafe fn setSourceMode_(&self, sourceMode: AVAudio3DMixingSourceMode)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSourceMode : sourceMode)
    }
    unsafe fn pointSourceInHeadMode(&self) -> AVAudio3DMixingPointSourceInHeadMode
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, pointSourceInHeadMode)
    }
    unsafe fn setPointSourceInHeadMode_(
        &self,
        pointSourceInHeadMode: AVAudio3DMixingPointSourceInHeadMode,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPointSourceInHeadMode : pointSourceInHeadMode)
    }
    unsafe fn rate(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, rate)
    }
    unsafe fn setRate_(&self, rate: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRate : rate)
    }
    unsafe fn reverbBlend(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, reverbBlend)
    }
    unsafe fn setReverbBlend_(&self, reverbBlend: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setReverbBlend : reverbBlend)
    }
    unsafe fn obstruction(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, obstruction)
    }
    unsafe fn setObstruction_(&self, obstruction: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setObstruction : obstruction)
    }
    unsafe fn occlusion(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, occlusion)
    }
    unsafe fn setOcclusion_(&self, occlusion: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setOcclusion : occlusion)
    }
    unsafe fn position(&self) -> AVAudio3DPoint
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, position)
    }
    unsafe fn setPosition_(&self, position: AVAudio3DPoint)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPosition : position)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct AVAudioMixingDestination(pub id);
impl std::ops::Deref for AVAudioMixingDestination {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for AVAudioMixingDestination {}
impl AVAudioMixingDestination {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"AVAudioMixingDestination").unwrap(), alloc) })
    }
}
impl PAVAudioMixing for AVAudioMixingDestination {}
impl INSObject for AVAudioMixingDestination {}
impl PNSObject for AVAudioMixingDestination {}
impl std::convert::TryFrom<NSObject> for AVAudioMixingDestination {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<AVAudioMixingDestination, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"AVAudioMixingDestination").unwrap()) };
        if is_kind_of {
            Ok(AVAudioMixingDestination(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to AVAudioMixingDestination")
        }
    }
}
impl IAVAudioMixingDestination for AVAudioMixingDestination {}
pub trait IAVAudioMixingDestination: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn connectionPoint(&self) -> AVAudioConnectionPoint
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, connectionPoint)
    }
}
pub type AVAudioIONodeInputBlock = *mut ::std::os::raw::c_void;
pub type AVAudioVoiceProcessingSpeechActivityEvent = NSInteger;
pub type AVAudioVoiceProcessingOtherAudioDuckingLevel = NSInteger;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AVAudioVoiceProcessingOtherAudioDuckingConfiguration {
    pub enableAdvancedDucking: BOOL,
    pub duckingLevel: AVAudioVoiceProcessingOtherAudioDuckingLevel,
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct AVAudioIONode(pub id);
impl std::ops::Deref for AVAudioIONode {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for AVAudioIONode {}
impl AVAudioIONode {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"AVAudioIONode").unwrap(), alloc) })
    }
}
impl IAVAudioNode for AVAudioIONode {}
impl From<AVAudioIONode> for AVAudioNode {
    fn from(child: AVAudioIONode) -> AVAudioNode {
        AVAudioNode(child.0)
    }
}
impl std::convert::TryFrom<AVAudioNode> for AVAudioIONode {
    type Error = &'static str;
    fn try_from(parent: AVAudioNode) -> Result<AVAudioIONode, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"AVAudioIONode").unwrap()) };
        if is_kind_of {
            Ok(AVAudioIONode(parent.0))
        } else {
            Err("This AVAudioNode cannot be downcasted to AVAudioIONode")
        }
    }
}
impl INSObject for AVAudioIONode {}
impl PNSObject for AVAudioIONode {}
impl IAVAudioIONode for AVAudioIONode {}
pub trait IAVAudioIONode: Sized + std::ops::Deref {
    unsafe fn setVoiceProcessingEnabled_error_(&self, enabled: BOOL, outError: *mut NSError) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setVoiceProcessingEnabled : enabled, error : outError)
    }
    unsafe fn presentationLatency(&self) -> NSTimeInterval
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, presentationLatency)
    }
    unsafe fn audioUnit(&self) -> AudioUnit
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, audioUnit)
    }
    unsafe fn isVoiceProcessingEnabled(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isVoiceProcessingEnabled)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct AVAudioInputNode(pub id);
impl std::ops::Deref for AVAudioInputNode {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for AVAudioInputNode {}
impl AVAudioInputNode {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"AVAudioInputNode").unwrap(), alloc) })
    }
}
impl PAVAudioMixing for AVAudioInputNode {}
impl IAVAudioIONode for AVAudioInputNode {}
impl From<AVAudioInputNode> for AVAudioIONode {
    fn from(child: AVAudioInputNode) -> AVAudioIONode {
        AVAudioIONode(child.0)
    }
}
impl std::convert::TryFrom<AVAudioIONode> for AVAudioInputNode {
    type Error = &'static str;
    fn try_from(parent: AVAudioIONode) -> Result<AVAudioInputNode, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"AVAudioInputNode").unwrap()) };
        if is_kind_of {
            Ok(AVAudioInputNode(parent.0))
        } else {
            Err("This AVAudioIONode cannot be downcasted to AVAudioInputNode")
        }
    }
}
impl IAVAudioNode for AVAudioInputNode {}
impl INSObject for AVAudioInputNode {}
impl PNSObject for AVAudioInputNode {}
impl IAVAudioInputNode for AVAudioInputNode {}
pub trait IAVAudioInputNode: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn setManualRenderingInputPCMFormat_inputBlock_(
        &self,
        format: AVAudioFormat,
        block: AVAudioIONodeInputBlock,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setManualRenderingInputPCMFormat : format, inputBlock : block)
    }
    unsafe fn setMutedSpeechActivityEventListener_(
        &self,
        listenerBlock: *mut ::std::os::raw::c_void,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMutedSpeechActivityEventListener : listenerBlock)
    }
    unsafe fn isVoiceProcessingBypassed(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isVoiceProcessingBypassed)
    }
    unsafe fn setVoiceProcessingBypassed_(&self, voiceProcessingBypassed: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setVoiceProcessingBypassed : voiceProcessingBypassed)
    }
    unsafe fn isVoiceProcessingAGCEnabled(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isVoiceProcessingAGCEnabled)
    }
    unsafe fn setVoiceProcessingAGCEnabled_(&self, voiceProcessingAGCEnabled: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setVoiceProcessingAGCEnabled : voiceProcessingAGCEnabled)
    }
    unsafe fn isVoiceProcessingInputMuted(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isVoiceProcessingInputMuted)
    }
    unsafe fn setVoiceProcessingInputMuted_(&self, voiceProcessingInputMuted: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setVoiceProcessingInputMuted : voiceProcessingInputMuted)
    }
    unsafe fn voiceProcessingOtherAudioDuckingConfiguration(
        &self,
    ) -> AVAudioVoiceProcessingOtherAudioDuckingConfiguration
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, voiceProcessingOtherAudioDuckingConfiguration)
    }
    unsafe fn setVoiceProcessingOtherAudioDuckingConfiguration_(
        &self,
        voiceProcessingOtherAudioDuckingConfiguration : AVAudioVoiceProcessingOtherAudioDuckingConfiguration,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setVoiceProcessingOtherAudioDuckingConfiguration : voiceProcessingOtherAudioDuckingConfiguration)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct AVAudioOutputNode(pub id);
impl std::ops::Deref for AVAudioOutputNode {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for AVAudioOutputNode {}
impl AVAudioOutputNode {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"AVAudioOutputNode").unwrap(), alloc) })
    }
}
impl IAVAudioIONode for AVAudioOutputNode {}
impl std::convert::TryFrom<AVAudioIONode> for AVAudioOutputNode {
    type Error = &'static str;
    fn try_from(parent: AVAudioIONode) -> Result<AVAudioOutputNode, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"AVAudioOutputNode").unwrap()) };
        if is_kind_of {
            Ok(AVAudioOutputNode(parent.0))
        } else {
            Err("This AVAudioIONode cannot be downcasted to AVAudioOutputNode")
        }
    }
}
impl IAVAudioNode for AVAudioOutputNode {}
impl INSObject for AVAudioOutputNode {}
impl PNSObject for AVAudioOutputNode {}
impl IAVAudioOutputNode for AVAudioOutputNode {}
pub trait IAVAudioOutputNode: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn intendedSpatialExperience(&self) -> CASpatialAudioExperience
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, intendedSpatialExperience)
    }
    unsafe fn setIntendedSpatialExperience_(
        &self,
        intendedSpatialExperience: CASpatialAudioExperience,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setIntendedSpatialExperience : intendedSpatialExperience)
    }
}
pub type AVAudioEngineManualRenderingError = OSStatus;
pub type AVAudioEngineManualRenderingStatus = NSInteger;
pub type AVAudioEngineManualRenderingMode = NSInteger;
pub type AVAudioEngineManualRenderingBlock = *mut ::std::os::raw::c_void;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct AVAudioEngine(pub id);
impl std::ops::Deref for AVAudioEngine {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for AVAudioEngine {}
impl AVAudioEngine {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"AVAudioEngine").unwrap(), alloc) })
    }
}
impl INSObject for AVAudioEngine {}
impl PNSObject for AVAudioEngine {}
impl std::convert::TryFrom<NSObject> for AVAudioEngine {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<AVAudioEngine, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"AVAudioEngine").unwrap()) };
        if is_kind_of {
            Ok(AVAudioEngine(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to AVAudioEngine")
        }
    }
}
impl IAVAudioEngine for AVAudioEngine {}
pub trait IAVAudioEngine: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn attachNode_(&self, node: AVAudioNode)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, attachNode : node)
    }
    unsafe fn detachNode_(&self, node: AVAudioNode)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, detachNode : node)
    }
    unsafe fn connect_to_fromBus_toBus_format_(
        &self,
        node1: AVAudioNode,
        node2: AVAudioNode,
        bus1: AVAudioNodeBus,
        bus2: AVAudioNodeBus,
        format: AVAudioFormat,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, connect : node1, to : node2, fromBus : bus1, toBus : bus2, format : format)
    }
    unsafe fn connect_to_format_(
        &self,
        node1: AVAudioNode,
        node2: AVAudioNode,
        format: AVAudioFormat,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, connect : node1, to : node2, format : format)
    }
    unsafe fn connect_toConnectionPoints_fromBus_format_(
        &self,
        sourceNode: AVAudioNode,
        destNodes: NSArray,
        sourceBus: AVAudioNodeBus,
        format: AVAudioFormat,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, connect : sourceNode, toConnectionPoints : destNodes, fromBus : sourceBus, format : format)
    }
    unsafe fn disconnectNodeInput_bus_(&self, node: AVAudioNode, bus: AVAudioNodeBus)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, disconnectNodeInput : node, bus : bus)
    }
    unsafe fn disconnectNodeInput_(&self, node: AVAudioNode)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, disconnectNodeInput : node)
    }
    unsafe fn disconnectNodeOutput_bus_(&self, node: AVAudioNode, bus: AVAudioNodeBus)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, disconnectNodeOutput : node, bus : bus)
    }
    unsafe fn disconnectNodeOutput_(&self, node: AVAudioNode)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, disconnectNodeOutput : node)
    }
    unsafe fn prepare(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, prepare)
    }
    unsafe fn startAndReturnError_(&self, outError: *mut NSError) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, startAndReturnError : outError)
    }
    unsafe fn pause(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, pause)
    }
    unsafe fn reset(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, reset)
    }
    unsafe fn stop(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, stop)
    }
    unsafe fn inputConnectionPointForNode_inputBus_(
        &self,
        node: AVAudioNode,
        bus: AVAudioNodeBus,
    ) -> AVAudioConnectionPoint
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, inputConnectionPointForNode : node, inputBus : bus)
    }
    unsafe fn outputConnectionPointsForNode_outputBus_(
        &self,
        node: AVAudioNode,
        bus: AVAudioNodeBus,
    ) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, outputConnectionPointsForNode : node, outputBus : bus)
    }
    unsafe fn enableManualRenderingMode_format_maximumFrameCount_error_(
        &self,
        mode: AVAudioEngineManualRenderingMode,
        pcmFormat: AVAudioFormat,
        maximumFrameCount: AVAudioFrameCount,
        outError: *mut NSError,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, enableManualRenderingMode : mode, format : pcmFormat, maximumFrameCount : maximumFrameCount, error : outError)
    }
    unsafe fn disableManualRenderingMode(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, disableManualRenderingMode)
    }
    unsafe fn renderOffline_toBuffer_error_(
        &self,
        numberOfFrames: AVAudioFrameCount,
        buffer: AVAudioPCMBuffer,
        outError: *mut NSError,
    ) -> AVAudioEngineManualRenderingStatus
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, renderOffline : numberOfFrames, toBuffer : buffer, error : outError)
    }
    unsafe fn connectMIDI_to_format_block_(
        &self,
        sourceNode: AVAudioNode,
        destinationNode: AVAudioNode,
        format: AVAudioFormat,
        tapBlock: AUMIDIOutputEventBlock,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, connectMIDI : sourceNode, to : destinationNode, format : format, block : tapBlock)
    }
    unsafe fn connectMIDI_to_format_eventListBlock_(
        &self,
        sourceNode: AVAudioNode,
        destinationNode: AVAudioNode,
        format: AVAudioFormat,
        tapBlock: AUMIDIEventListBlock,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, connectMIDI : sourceNode, to : destinationNode, format : format, eventListBlock : tapBlock)
    }
    unsafe fn connectMIDI_toNodes_format_block_(
        &self,
        sourceNode: AVAudioNode,
        destinationNodes: NSArray,
        format: AVAudioFormat,
        tapBlock: AUMIDIOutputEventBlock,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, connectMIDI : sourceNode, toNodes : destinationNodes, format : format, block : tapBlock)
    }
    unsafe fn connectMIDI_toNodes_format_eventListBlock_(
        &self,
        sourceNode: AVAudioNode,
        destinationNodes: NSArray,
        format: AVAudioFormat,
        tapBlock: AUMIDIEventListBlock,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, connectMIDI : sourceNode, toNodes : destinationNodes, format : format, eventListBlock : tapBlock)
    }
    unsafe fn disconnectMIDI_from_(&self, sourceNode: AVAudioNode, destinationNode: AVAudioNode)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, disconnectMIDI : sourceNode, from : destinationNode)
    }
    unsafe fn disconnectMIDI_fromNodes_(&self, sourceNode: AVAudioNode, destinationNodes: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, disconnectMIDI : sourceNode, fromNodes : destinationNodes)
    }
    unsafe fn disconnectMIDIInput_(&self, node: AVAudioNode)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, disconnectMIDIInput : node)
    }
    unsafe fn disconnectMIDIOutput_(&self, node: AVAudioNode)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, disconnectMIDIOutput : node)
    }
    unsafe fn musicSequence(&self) -> MusicSequence
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, musicSequence)
    }
    unsafe fn setMusicSequence_(&self, musicSequence: MusicSequence)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMusicSequence : musicSequence)
    }
    unsafe fn outputNode(&self) -> AVAudioOutputNode
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, outputNode)
    }
    unsafe fn inputNode(&self) -> AVAudioInputNode
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inputNode)
    }
    unsafe fn mainMixerNode(&self) -> AVAudioMixerNode
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, mainMixerNode)
    }
    unsafe fn isRunning(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isRunning)
    }
    unsafe fn isAutoShutdownEnabled(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isAutoShutdownEnabled)
    }
    unsafe fn setAutoShutdownEnabled_(&self, autoShutdownEnabled: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAutoShutdownEnabled : autoShutdownEnabled)
    }
    unsafe fn attachedNodes(&self) -> NSSet
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, attachedNodes)
    }
    unsafe fn manualRenderingBlock(&self) -> AVAudioEngineManualRenderingBlock
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, manualRenderingBlock)
    }
    unsafe fn isInManualRenderingMode(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isInManualRenderingMode)
    }
    unsafe fn manualRenderingMode(&self) -> AVAudioEngineManualRenderingMode
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, manualRenderingMode)
    }
    unsafe fn manualRenderingFormat(&self) -> AVAudioFormat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, manualRenderingFormat)
    }
    unsafe fn manualRenderingMaximumFrameCount(&self) -> AVAudioFrameCount
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, manualRenderingMaximumFrameCount)
    }
    unsafe fn manualRenderingSampleTime(&self) -> AVAudioFramePosition
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, manualRenderingSampleTime)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct AVAudioUnit(pub id);
impl std::ops::Deref for AVAudioUnit {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for AVAudioUnit {}
impl AVAudioUnit {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"AVAudioUnit").unwrap(), alloc) })
    }
}
impl IAVAudioNode for AVAudioUnit {}
impl std::convert::TryFrom<AVAudioNode> for AVAudioUnit {
    type Error = &'static str;
    fn try_from(parent: AVAudioNode) -> Result<AVAudioUnit, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"AVAudioUnit").unwrap()) };
        if is_kind_of {
            Ok(AVAudioUnit(parent.0))
        } else {
            Err("This AVAudioNode cannot be downcasted to AVAudioUnit")
        }
    }
}
impl INSObject for AVAudioUnit {}
impl PNSObject for AVAudioUnit {}
impl IAVAudioUnit for AVAudioUnit {}
pub trait IAVAudioUnit: Sized + std::ops::Deref {
    unsafe fn loadAudioUnitPresetAtURL_error_(&self, url: NSURL, outError: *mut NSError) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, loadAudioUnitPresetAtURL : url, error : outError)
    }
    unsafe fn audioComponentDescription(&self) -> AudioComponentDescription
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, audioComponentDescription)
    }
    unsafe fn audioUnit(&self) -> AudioUnit
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, audioUnit)
    }
    unsafe fn AUAudioUnit(&self) -> AUAudioUnit
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, AUAudioUnit)
    }
    unsafe fn name(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, name)
    }
    unsafe fn manufacturerName(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, manufacturerName)
    }
    unsafe fn version(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, version)
    }
    unsafe fn instantiateWithComponentDescription_options_completionHandler_(
        audioComponentDescription: AudioComponentDescription,
        options: AudioComponentInstantiationOptions,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"AVAudioUnit").unwrap(), instantiateWithComponentDescription : audioComponentDescription, options : options, completionHandler : completionHandler)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct AVAudioUnitEffect(pub id);
impl std::ops::Deref for AVAudioUnitEffect {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for AVAudioUnitEffect {}
impl AVAudioUnitEffect {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"AVAudioUnitEffect").unwrap(), alloc) })
    }
}
impl IAVAudioUnit for AVAudioUnitEffect {}
impl From<AVAudioUnitEffect> for AVAudioUnit {
    fn from(child: AVAudioUnitEffect) -> AVAudioUnit {
        AVAudioUnit(child.0)
    }
}
impl std::convert::TryFrom<AVAudioUnit> for AVAudioUnitEffect {
    type Error = &'static str;
    fn try_from(parent: AVAudioUnit) -> Result<AVAudioUnitEffect, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"AVAudioUnitEffect").unwrap()) };
        if is_kind_of {
            Ok(AVAudioUnitEffect(parent.0))
        } else {
            Err("This AVAudioUnit cannot be downcasted to AVAudioUnitEffect")
        }
    }
}
impl IAVAudioNode for AVAudioUnitEffect {}
impl INSObject for AVAudioUnitEffect {}
impl PNSObject for AVAudioUnitEffect {}
impl IAVAudioUnitEffect for AVAudioUnitEffect {}
pub trait IAVAudioUnitEffect: Sized + std::ops::Deref {
    unsafe fn initWithAudioComponentDescription_(
        &self,
        audioComponentDescription: AudioComponentDescription,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithAudioComponentDescription : audioComponentDescription)
    }
    unsafe fn bypass(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, bypass)
    }
    unsafe fn setBypass_(&self, bypass: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setBypass : bypass)
    }
}
pub type AVAudioUnitReverbPreset = NSInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct AVAudioUnitReverb(pub id);
impl std::ops::Deref for AVAudioUnitReverb {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for AVAudioUnitReverb {}
impl AVAudioUnitReverb {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"AVAudioUnitReverb").unwrap(), alloc) })
    }
}
impl IAVAudioUnitEffect for AVAudioUnitReverb {}
impl From<AVAudioUnitReverb> for AVAudioUnitEffect {
    fn from(child: AVAudioUnitReverb) -> AVAudioUnitEffect {
        AVAudioUnitEffect(child.0)
    }
}
impl std::convert::TryFrom<AVAudioUnitEffect> for AVAudioUnitReverb {
    type Error = &'static str;
    fn try_from(parent: AVAudioUnitEffect) -> Result<AVAudioUnitReverb, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"AVAudioUnitReverb").unwrap()) };
        if is_kind_of {
            Ok(AVAudioUnitReverb(parent.0))
        } else {
            Err("This AVAudioUnitEffect cannot be downcasted to AVAudioUnitReverb")
        }
    }
}
impl IAVAudioUnit for AVAudioUnitReverb {}
impl IAVAudioNode for AVAudioUnitReverb {}
impl INSObject for AVAudioUnitReverb {}
impl PNSObject for AVAudioUnitReverb {}
impl IAVAudioUnitReverb for AVAudioUnitReverb {}
pub trait IAVAudioUnitReverb: Sized + std::ops::Deref {
    unsafe fn loadFactoryPreset_(&self, preset: AVAudioUnitReverbPreset)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, loadFactoryPreset : preset)
    }
    unsafe fn wetDryMix(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, wetDryMix)
    }
    unsafe fn setWetDryMix_(&self, wetDryMix: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setWetDryMix : wetDryMix)
    }
}
pub type AVAudioUnitEQFilterType = NSInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct AVAudioUnitEQFilterParameters(pub id);
impl std::ops::Deref for AVAudioUnitEQFilterParameters {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for AVAudioUnitEQFilterParameters {}
impl AVAudioUnitEQFilterParameters {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"AVAudioUnitEQFilterParameters").unwrap(), alloc) })
    }
}
impl INSObject for AVAudioUnitEQFilterParameters {}
impl PNSObject for AVAudioUnitEQFilterParameters {}
impl std::convert::TryFrom<NSObject> for AVAudioUnitEQFilterParameters {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<AVAudioUnitEQFilterParameters, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"AVAudioUnitEQFilterParameters").unwrap())
        };
        if is_kind_of {
            Ok(AVAudioUnitEQFilterParameters(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to AVAudioUnitEQFilterParameters")
        }
    }
}
impl IAVAudioUnitEQFilterParameters for AVAudioUnitEQFilterParameters {}
pub trait IAVAudioUnitEQFilterParameters: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn filterType(&self) -> AVAudioUnitEQFilterType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, filterType)
    }
    unsafe fn setFilterType_(&self, filterType: AVAudioUnitEQFilterType)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setFilterType : filterType)
    }
    unsafe fn frequency(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, frequency)
    }
    unsafe fn setFrequency_(&self, frequency: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setFrequency : frequency)
    }
    unsafe fn bandwidth(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, bandwidth)
    }
    unsafe fn setBandwidth_(&self, bandwidth: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setBandwidth : bandwidth)
    }
    unsafe fn gain(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, gain)
    }
    unsafe fn setGain_(&self, gain: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setGain : gain)
    }
    unsafe fn bypass(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, bypass)
    }
    unsafe fn setBypass_(&self, bypass: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setBypass : bypass)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct AVAudioUnitEQ(pub id);
impl std::ops::Deref for AVAudioUnitEQ {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for AVAudioUnitEQ {}
impl AVAudioUnitEQ {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"AVAudioUnitEQ").unwrap(), alloc) })
    }
}
impl IAVAudioUnitEffect for AVAudioUnitEQ {}
impl std::convert::TryFrom<AVAudioUnitEffect> for AVAudioUnitEQ {
    type Error = &'static str;
    fn try_from(parent: AVAudioUnitEffect) -> Result<AVAudioUnitEQ, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"AVAudioUnitEQ").unwrap()) };
        if is_kind_of {
            Ok(AVAudioUnitEQ(parent.0))
        } else {
            Err("This AVAudioUnitEffect cannot be downcasted to AVAudioUnitEQ")
        }
    }
}
impl IAVAudioUnit for AVAudioUnitEQ {}
impl IAVAudioNode for AVAudioUnitEQ {}
impl INSObject for AVAudioUnitEQ {}
impl PNSObject for AVAudioUnitEQ {}
impl IAVAudioUnitEQ for AVAudioUnitEQ {}
pub trait IAVAudioUnitEQ: Sized + std::ops::Deref {
    unsafe fn initWithNumberOfBands_(&self, numberOfBands: NSUInteger) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithNumberOfBands : numberOfBands)
    }
    unsafe fn bands(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, bands)
    }
    unsafe fn globalGain(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, globalGain)
    }
    unsafe fn setGlobalGain_(&self, globalGain: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setGlobalGain : globalGain)
    }
}
pub type AVAudioEnvironmentDistanceAttenuationModel = NSInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct AVAudioEnvironmentDistanceAttenuationParameters(pub id);
impl std::ops::Deref for AVAudioEnvironmentDistanceAttenuationParameters {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for AVAudioEnvironmentDistanceAttenuationParameters {}
impl AVAudioEnvironmentDistanceAttenuationParameters {
    pub fn alloc() -> Self {
        Self(unsafe {
            msg_send!(&*objc2::runtime::AnyClass::get(c"AVAudioEnvironmentDistanceAttenuationParameters").unwrap(), alloc)
        })
    }
}
impl INSObject for AVAudioEnvironmentDistanceAttenuationParameters {}
impl PNSObject for AVAudioEnvironmentDistanceAttenuationParameters {}
impl std::convert::TryFrom<NSObject> for AVAudioEnvironmentDistanceAttenuationParameters {
    type Error = &'static str;
    fn try_from(
        parent: NSObject,
    ) -> Result<AVAudioEnvironmentDistanceAttenuationParameters, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"AVAudioEnvironmentDistanceAttenuationParameters").unwrap())
        };
        if is_kind_of {
            Ok(AVAudioEnvironmentDistanceAttenuationParameters(parent.0))
        } else {
            Err ("This NSObject cannot be downcasted to AVAudioEnvironmentDistanceAttenuationParameters" ,)
        }
    }
}
impl IAVAudioEnvironmentDistanceAttenuationParameters
    for AVAudioEnvironmentDistanceAttenuationParameters
{
}
pub trait IAVAudioEnvironmentDistanceAttenuationParameters: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn distanceAttenuationModel(&self) -> AVAudioEnvironmentDistanceAttenuationModel
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, distanceAttenuationModel)
    }
    unsafe fn setDistanceAttenuationModel_(
        &self,
        distanceAttenuationModel: AVAudioEnvironmentDistanceAttenuationModel,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDistanceAttenuationModel : distanceAttenuationModel)
    }
    unsafe fn referenceDistance(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, referenceDistance)
    }
    unsafe fn setReferenceDistance_(&self, referenceDistance: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setReferenceDistance : referenceDistance)
    }
    unsafe fn maximumDistance(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, maximumDistance)
    }
    unsafe fn setMaximumDistance_(&self, maximumDistance: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMaximumDistance : maximumDistance)
    }
    unsafe fn rolloffFactor(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, rolloffFactor)
    }
    unsafe fn setRolloffFactor_(&self, rolloffFactor: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRolloffFactor : rolloffFactor)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct AVAudioEnvironmentReverbParameters(pub id);
impl std::ops::Deref for AVAudioEnvironmentReverbParameters {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for AVAudioEnvironmentReverbParameters {}
impl AVAudioEnvironmentReverbParameters {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"AVAudioEnvironmentReverbParameters").unwrap(), alloc) })
    }
}
impl INSObject for AVAudioEnvironmentReverbParameters {}
impl PNSObject for AVAudioEnvironmentReverbParameters {}
impl std::convert::TryFrom<NSObject> for AVAudioEnvironmentReverbParameters {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<AVAudioEnvironmentReverbParameters, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"AVAudioEnvironmentReverbParameters").unwrap())
        };
        if is_kind_of {
            Ok(AVAudioEnvironmentReverbParameters(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to AVAudioEnvironmentReverbParameters")
        }
    }
}
impl IAVAudioEnvironmentReverbParameters for AVAudioEnvironmentReverbParameters {}
pub trait IAVAudioEnvironmentReverbParameters: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn loadFactoryReverbPreset_(&self, preset: AVAudioUnitReverbPreset)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, loadFactoryReverbPreset : preset)
    }
    unsafe fn enable(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, enable)
    }
    unsafe fn setEnable_(&self, enable: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setEnable : enable)
    }
    unsafe fn level(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, level)
    }
    unsafe fn setLevel_(&self, level: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setLevel : level)
    }
    unsafe fn filterParameters(&self) -> AVAudioUnitEQFilterParameters
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, filterParameters)
    }
}
pub type AVAudioEnvironmentOutputType = NSInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct AVAudioEnvironmentNode(pub id);
impl std::ops::Deref for AVAudioEnvironmentNode {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for AVAudioEnvironmentNode {}
impl AVAudioEnvironmentNode {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"AVAudioEnvironmentNode").unwrap(), alloc) })
    }
}
impl PAVAudioMixing for AVAudioEnvironmentNode {}
impl IAVAudioNode for AVAudioEnvironmentNode {}
impl std::convert::TryFrom<AVAudioNode> for AVAudioEnvironmentNode {
    type Error = &'static str;
    fn try_from(parent: AVAudioNode) -> Result<AVAudioEnvironmentNode, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"AVAudioEnvironmentNode").unwrap()) };
        if is_kind_of {
            Ok(AVAudioEnvironmentNode(parent.0))
        } else {
            Err("This AVAudioNode cannot be downcasted to AVAudioEnvironmentNode")
        }
    }
}
impl INSObject for AVAudioEnvironmentNode {}
impl PNSObject for AVAudioEnvironmentNode {}
impl IAVAudioEnvironmentNode for AVAudioEnvironmentNode {}
pub trait IAVAudioEnvironmentNode: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn outputType(&self) -> AVAudioEnvironmentOutputType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, outputType)
    }
    unsafe fn setOutputType_(&self, outputType: AVAudioEnvironmentOutputType)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setOutputType : outputType)
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
    unsafe fn nextAvailableInputBus(&self) -> AVAudioNodeBus
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, nextAvailableInputBus)
    }
    unsafe fn listenerPosition(&self) -> AVAudio3DPoint
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, listenerPosition)
    }
    unsafe fn setListenerPosition_(&self, listenerPosition: AVAudio3DPoint)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setListenerPosition : listenerPosition)
    }
    unsafe fn listenerVectorOrientation(&self) -> AVAudio3DVectorOrientation
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, listenerVectorOrientation)
    }
    unsafe fn setListenerVectorOrientation_(
        &self,
        listenerVectorOrientation: AVAudio3DVectorOrientation,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setListenerVectorOrientation : listenerVectorOrientation)
    }
    unsafe fn listenerAngularOrientation(&self) -> AVAudio3DAngularOrientation
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, listenerAngularOrientation)
    }
    unsafe fn setListenerAngularOrientation_(
        &self,
        listenerAngularOrientation: AVAudio3DAngularOrientation,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setListenerAngularOrientation : listenerAngularOrientation)
    }
    unsafe fn distanceAttenuationParameters(
        &self,
    ) -> AVAudioEnvironmentDistanceAttenuationParameters
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, distanceAttenuationParameters)
    }
    unsafe fn reverbParameters(&self) -> AVAudioEnvironmentReverbParameters
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, reverbParameters)
    }
    unsafe fn applicableRenderingAlgorithms(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, applicableRenderingAlgorithms)
    }
    unsafe fn isListenerHeadTrackingEnabled(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isListenerHeadTrackingEnabled)
    }
    unsafe fn setListenerHeadTrackingEnabled_(&self, listenerHeadTrackingEnabled: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setListenerHeadTrackingEnabled : listenerHeadTrackingEnabled)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct AVAudioFile(pub id);
impl std::ops::Deref for AVAudioFile {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for AVAudioFile {}
impl AVAudioFile {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"AVAudioFile").unwrap(), alloc) })
    }
}
impl INSObject for AVAudioFile {}
impl PNSObject for AVAudioFile {}
impl std::convert::TryFrom<NSObject> for AVAudioFile {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<AVAudioFile, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"AVAudioFile").unwrap()) };
        if is_kind_of {
            Ok(AVAudioFile(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to AVAudioFile")
        }
    }
}
impl IAVAudioFile for AVAudioFile {}
pub trait IAVAudioFile: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn initForReading_error_(&self, fileURL: NSURL, outError: *mut NSError) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initForReading : fileURL, error : outError)
    }
    unsafe fn initForReading_commonFormat_interleaved_error_(
        &self,
        fileURL: NSURL,
        format: AVAudioCommonFormat,
        interleaved: BOOL,
        outError: *mut NSError,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initForReading : fileURL, commonFormat : format, interleaved : interleaved, error : outError)
    }
    unsafe fn initForWriting_settings_error_(
        &self,
        fileURL: NSURL,
        settings: NSDictionary,
        outError: *mut NSError,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initForWriting : fileURL, settings : settings, error : outError)
    }
    unsafe fn initForWriting_settings_commonFormat_interleaved_error_(
        &self,
        fileURL: NSURL,
        settings: NSDictionary,
        format: AVAudioCommonFormat,
        interleaved: BOOL,
        outError: *mut NSError,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initForWriting : fileURL, settings : settings, commonFormat : format, interleaved : interleaved, error : outError)
    }
    unsafe fn close(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, close)
    }
    unsafe fn readIntoBuffer_error_(&self, buffer: AVAudioPCMBuffer, outError: *mut NSError) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, readIntoBuffer : buffer, error : outError)
    }
    unsafe fn readIntoBuffer_frameCount_error_(
        &self,
        buffer: AVAudioPCMBuffer,
        frames: AVAudioFrameCount,
        outError: *mut NSError,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, readIntoBuffer : buffer, frameCount : frames, error : outError)
    }
    unsafe fn writeFromBuffer_error_(
        &self,
        buffer: AVAudioPCMBuffer,
        outError: *mut NSError,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, writeFromBuffer : buffer, error : outError)
    }
    unsafe fn isOpen(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isOpen)
    }
    unsafe fn url(&self) -> NSURL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, url)
    }
    unsafe fn fileFormat(&self) -> AVAudioFormat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, fileFormat)
    }
    unsafe fn processingFormat(&self) -> AVAudioFormat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, processingFormat)
    }
    unsafe fn length(&self) -> AVAudioFramePosition
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, length)
    }
    unsafe fn framePosition(&self) -> AVAudioFramePosition
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, framePosition)
    }
    unsafe fn setFramePosition_(&self, framePosition: AVAudioFramePosition)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setFramePosition : framePosition)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct AVAudioMixerNode(pub id);
impl std::ops::Deref for AVAudioMixerNode {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for AVAudioMixerNode {}
impl AVAudioMixerNode {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"AVAudioMixerNode").unwrap(), alloc) })
    }
}
impl PAVAudioMixing for AVAudioMixerNode {}
impl IAVAudioNode for AVAudioMixerNode {}
impl std::convert::TryFrom<AVAudioNode> for AVAudioMixerNode {
    type Error = &'static str;
    fn try_from(parent: AVAudioNode) -> Result<AVAudioMixerNode, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"AVAudioMixerNode").unwrap()) };
        if is_kind_of {
            Ok(AVAudioMixerNode(parent.0))
        } else {
            Err("This AVAudioNode cannot be downcasted to AVAudioMixerNode")
        }
    }
}
impl INSObject for AVAudioMixerNode {}
impl PNSObject for AVAudioMixerNode {}
impl IAVAudioMixerNode for AVAudioMixerNode {}
pub trait IAVAudioMixerNode: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
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
    unsafe fn nextAvailableInputBus(&self) -> AVAudioNodeBus
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, nextAvailableInputBus)
    }
}
pub type AVAudioSessionPort = NSString;
pub type AVAudioSessionCategory = NSString;
pub type AVAudioSessionMode = NSString;
pub type AVAudioSessionActivationOptions = NSUInteger;
pub type AVAudioSessionPortOverride = NSUInteger;
pub type AVAudioSessionRouteChangeReason = NSUInteger;
pub type AVAudioSessionCategoryOptions = NSUInteger;
pub type AVAudioSessionInterruptionType = NSUInteger;
pub type AVAudioSessionInterruptionOptions = NSUInteger;
pub type AVAudioSessionInterruptionReason = NSUInteger;
pub type AVAudioSessionSetActiveOptions = NSUInteger;
pub type AVAudioSessionSilenceSecondaryAudioHintType = NSUInteger;
pub type AVAudioSessionIOType = NSUInteger;
pub type AVAudioSessionRouteSharingPolicy = NSUInteger;
pub type AVAudioSessionPromptStyle = NSUInteger;
pub type AVAudioStereoOrientation = NSInteger;
pub type AVAudioSessionRecordPermission = NSUInteger;
pub type AVAudioSessionRenderingMode = NSInteger;
pub type AVAudioSessionMicrophoneInjectionMode = NSInteger;
pub type AVAudioSessionLocation = NSString;
pub type AVAudioSessionOrientation = NSString;
pub type AVAudioSessionPolarPattern = NSString;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct AVAudioSessionChannelDescription(pub id);
impl std::ops::Deref for AVAudioSessionChannelDescription {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for AVAudioSessionChannelDescription {}
impl AVAudioSessionChannelDescription {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"AVAudioSessionChannelDescription").unwrap(), alloc) })
    }
}
impl INSObject for AVAudioSessionChannelDescription {}
impl PNSObject for AVAudioSessionChannelDescription {}
impl std::convert::TryFrom<NSObject> for AVAudioSessionChannelDescription {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<AVAudioSessionChannelDescription, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"AVAudioSessionChannelDescription").unwrap())
        };
        if is_kind_of {
            Ok(AVAudioSessionChannelDescription(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to AVAudioSessionChannelDescription")
        }
    }
}
impl IAVAudioSessionChannelDescription for AVAudioSessionChannelDescription {}
pub trait IAVAudioSessionChannelDescription: Sized + std::ops::Deref {
    unsafe fn channelName(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, channelName)
    }
    unsafe fn owningPortUID(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, owningPortUID)
    }
    unsafe fn channelNumber(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, channelNumber)
    }
    unsafe fn channelLabel(&self) -> AudioChannelLabel
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, channelLabel)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct AVAudioSessionDataSourceDescription(pub id);
impl std::ops::Deref for AVAudioSessionDataSourceDescription {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for AVAudioSessionDataSourceDescription {}
impl AVAudioSessionDataSourceDescription {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"AVAudioSessionDataSourceDescription").unwrap(), alloc) })
    }
}
impl INSObject for AVAudioSessionDataSourceDescription {}
impl PNSObject for AVAudioSessionDataSourceDescription {}
impl std::convert::TryFrom<NSObject> for AVAudioSessionDataSourceDescription {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<AVAudioSessionDataSourceDescription, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"AVAudioSessionDataSourceDescription").unwrap())
        };
        if is_kind_of {
            Ok(AVAudioSessionDataSourceDescription(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to AVAudioSessionDataSourceDescription")
        }
    }
}
impl IAVAudioSessionDataSourceDescription for AVAudioSessionDataSourceDescription {}
pub trait IAVAudioSessionDataSourceDescription: Sized + std::ops::Deref {
    unsafe fn setPreferredPolarPattern_error_(
        &self,
        pattern: NSString,
        outError: *mut NSError,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPreferredPolarPattern : pattern, error : outError)
    }
    unsafe fn dataSourceID(&self) -> NSNumber
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, dataSourceID)
    }
    unsafe fn dataSourceName(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, dataSourceName)
    }
    unsafe fn location(&self) -> AVAudioSessionLocation
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, location)
    }
    unsafe fn orientation(&self) -> AVAudioSessionOrientation
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, orientation)
    }
    unsafe fn supportedPolarPatterns(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, supportedPolarPatterns)
    }
    unsafe fn selectedPolarPattern(&self) -> AVAudioSessionPolarPattern
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, selectedPolarPattern)
    }
    unsafe fn preferredPolarPattern(&self) -> AVAudioSessionPolarPattern
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, preferredPolarPattern)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct AVAudioSessionCapability(pub id);
impl std::ops::Deref for AVAudioSessionCapability {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for AVAudioSessionCapability {}
impl AVAudioSessionCapability {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"AVAudioSessionCapability").unwrap(), alloc) })
    }
}
impl INSObject for AVAudioSessionCapability {}
impl PNSObject for AVAudioSessionCapability {}
impl std::convert::TryFrom<NSObject> for AVAudioSessionCapability {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<AVAudioSessionCapability, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"AVAudioSessionCapability").unwrap()) };
        if is_kind_of {
            Ok(AVAudioSessionCapability(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to AVAudioSessionCapability")
        }
    }
}
impl IAVAudioSessionCapability for AVAudioSessionCapability {}
pub trait IAVAudioSessionCapability: Sized + std::ops::Deref {
    unsafe fn isSupported(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isSupported)
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
pub struct AVAudioSessionPortExtensionBluetoothMicrophone(pub id);
impl std::ops::Deref for AVAudioSessionPortExtensionBluetoothMicrophone {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for AVAudioSessionPortExtensionBluetoothMicrophone {}
impl AVAudioSessionPortExtensionBluetoothMicrophone {
    pub fn alloc() -> Self {
        Self(unsafe {
            msg_send!(&*objc2::runtime::AnyClass::get(c"AVAudioSessionPortExtensionBluetoothMicrophone").unwrap(), alloc)
        })
    }
}
impl INSObject for AVAudioSessionPortExtensionBluetoothMicrophone {}
impl PNSObject for AVAudioSessionPortExtensionBluetoothMicrophone {}
impl std::convert::TryFrom<NSObject> for AVAudioSessionPortExtensionBluetoothMicrophone {
    type Error = &'static str;
    fn try_from(
        parent: NSObject,
    ) -> Result<AVAudioSessionPortExtensionBluetoothMicrophone, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"AVAudioSessionPortExtensionBluetoothMicrophone").unwrap())
        };
        if is_kind_of {
            Ok(AVAudioSessionPortExtensionBluetoothMicrophone(parent.0))
        } else {
            Err ("This NSObject cannot be downcasted to AVAudioSessionPortExtensionBluetoothMicrophone" ,)
        }
    }
}
impl IAVAudioSessionPortExtensionBluetoothMicrophone
    for AVAudioSessionPortExtensionBluetoothMicrophone
{
}
pub trait IAVAudioSessionPortExtensionBluetoothMicrophone: Sized + std::ops::Deref {
    unsafe fn highQualityRecording(&self) -> AVAudioSessionCapability
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, highQualityRecording)
    }
    unsafe fn farFieldCapture(&self) -> AVAudioSessionCapability
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, farFieldCapture)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct AVAudioSessionPortDescription(pub id);
impl std::ops::Deref for AVAudioSessionPortDescription {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for AVAudioSessionPortDescription {}
impl AVAudioSessionPortDescription {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"AVAudioSessionPortDescription").unwrap(), alloc) })
    }
}
impl INSObject for AVAudioSessionPortDescription {}
impl PNSObject for AVAudioSessionPortDescription {}
impl std::convert::TryFrom<NSObject> for AVAudioSessionPortDescription {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<AVAudioSessionPortDescription, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"AVAudioSessionPortDescription").unwrap())
        };
        if is_kind_of {
            Ok(AVAudioSessionPortDescription(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to AVAudioSessionPortDescription")
        }
    }
}
impl IAVAudioSessionPortDescription for AVAudioSessionPortDescription {}
pub trait IAVAudioSessionPortDescription: Sized + std::ops::Deref {
    unsafe fn setPreferredDataSource_error_(
        &self,
        dataSource: AVAudioSessionDataSourceDescription,
        outError: *mut NSError,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPreferredDataSource : dataSource, error : outError)
    }
    unsafe fn portType(&self) -> AVAudioSessionPort
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, portType)
    }
    unsafe fn portName(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, portName)
    }
    unsafe fn UID(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, UID)
    }
    unsafe fn hasHardwareVoiceCallProcessing(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, hasHardwareVoiceCallProcessing)
    }
    unsafe fn isSpatialAudioEnabled(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isSpatialAudioEnabled)
    }
    unsafe fn channels(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, channels)
    }
    unsafe fn dataSources(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, dataSources)
    }
    unsafe fn selectedDataSource(&self) -> AVAudioSessionDataSourceDescription
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, selectedDataSource)
    }
    unsafe fn preferredDataSource(&self) -> AVAudioSessionDataSourceDescription
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, preferredDataSource)
    }
}
impl AVAudioSessionPortDescription_BluetoothMicrophoneExtension for AVAudioSessionPortDescription {}
pub trait AVAudioSessionPortDescription_BluetoothMicrophoneExtension:
    Sized + std::ops::Deref
{
    unsafe fn bluetoothMicrophoneExtension(&self) -> AVAudioSessionPortExtensionBluetoothMicrophone
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, bluetoothMicrophoneExtension)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct AVAudioSessionRouteDescription(pub id);
impl std::ops::Deref for AVAudioSessionRouteDescription {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for AVAudioSessionRouteDescription {}
impl AVAudioSessionRouteDescription {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"AVAudioSessionRouteDescription").unwrap(), alloc) })
    }
}
impl INSObject for AVAudioSessionRouteDescription {}
impl PNSObject for AVAudioSessionRouteDescription {}
impl std::convert::TryFrom<NSObject> for AVAudioSessionRouteDescription {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<AVAudioSessionRouteDescription, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"AVAudioSessionRouteDescription").unwrap())
        };
        if is_kind_of {
            Ok(AVAudioSessionRouteDescription(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to AVAudioSessionRouteDescription")
        }
    }
}
impl IAVAudioSessionRouteDescription for AVAudioSessionRouteDescription {}
pub trait IAVAudioSessionRouteDescription: Sized + std::ops::Deref {
    unsafe fn inputs(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inputs)
    }
    unsafe fn outputs(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, outputs)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct AVAudioSession(pub id);
impl std::ops::Deref for AVAudioSession {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for AVAudioSession {}
impl AVAudioSession {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"AVAudioSession").unwrap(), alloc) })
    }
}
impl INSObject for AVAudioSession {}
impl PNSObject for AVAudioSession {}
impl std::convert::TryFrom<NSObject> for AVAudioSession {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<AVAudioSession, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"AVAudioSession").unwrap()) };
        if is_kind_of {
            Ok(AVAudioSession(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to AVAudioSession")
        }
    }
}
impl IAVAudioSession for AVAudioSession {}
pub trait IAVAudioSession: Sized + std::ops::Deref {
    unsafe fn setCategory_error_(&self, category: NSString, outError: *mut NSError) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCategory : category, error : outError)
    }
    unsafe fn setCategory_withOptions_error_(
        &self,
        category: NSString,
        options: AVAudioSessionCategoryOptions,
        outError: *mut NSError,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCategory : category, withOptions : options, error : outError)
    }
    unsafe fn setCategory_mode_options_error_(
        &self,
        category: NSString,
        mode: NSString,
        options: AVAudioSessionCategoryOptions,
        outError: *mut NSError,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCategory : category, mode : mode, options : options, error : outError)
    }
    unsafe fn setCategory_mode_routeSharingPolicy_options_error_(
        &self,
        category: NSString,
        mode: NSString,
        policy: AVAudioSessionRouteSharingPolicy,
        options: AVAudioSessionCategoryOptions,
        outError: *mut NSError,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCategory : category, mode : mode, routeSharingPolicy : policy, options : options, error : outError)
    }
    unsafe fn setMode_error_(&self, mode: NSString, outError: *mut NSError) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMode : mode, error : outError)
    }
    unsafe fn setAllowHapticsAndSystemSoundsDuringRecording_error_(
        &self,
        inValue: BOOL,
        outError: *mut NSError,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAllowHapticsAndSystemSoundsDuringRecording : inValue, error : outError)
    }
    unsafe fn requestRecordPermission_(&self, response: *mut ::std::os::raw::c_void)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, requestRecordPermission : response)
    }
    unsafe fn overrideOutputAudioPort_error_(
        &self,
        portOverride: AVAudioSessionPortOverride,
        outError: *mut NSError,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, overrideOutputAudioPort : portOverride, error : outError)
    }
    unsafe fn setPreferredInput_error_(
        &self,
        inPort: AVAudioSessionPortDescription,
        outError: *mut NSError,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPreferredInput : inPort, error : outError)
    }
    unsafe fn setPrefersNoInterruptionsFromSystemAlerts_error_(
        &self,
        inValue: BOOL,
        outError: *mut NSError,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPrefersNoInterruptionsFromSystemAlerts : inValue, error : outError)
    }
    unsafe fn setPrefersEchoCancelledInput_error_(&self, value: BOOL, error: *mut NSError) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPrefersEchoCancelledInput : value, error : error)
    }
    unsafe fn setOutputMuted_error_(&self, muted: BOOL, outError: *mut NSError) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setOutputMuted : muted, error : outError)
    }
    unsafe fn availableCategories(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, availableCategories)
    }
    unsafe fn category(&self) -> AVAudioSessionCategory
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, category)
    }
    unsafe fn categoryOptions(&self) -> AVAudioSessionCategoryOptions
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, categoryOptions)
    }
    unsafe fn routeSharingPolicy(&self) -> AVAudioSessionRouteSharingPolicy
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, routeSharingPolicy)
    }
    unsafe fn availableModes(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, availableModes)
    }
    unsafe fn mode(&self) -> AVAudioSessionMode
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, mode)
    }
    unsafe fn allowHapticsAndSystemSoundsDuringRecording(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, allowHapticsAndSystemSoundsDuringRecording)
    }
    unsafe fn recordPermission(&self) -> AVAudioSessionRecordPermission
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, recordPermission)
    }
    unsafe fn preferredInput(&self) -> AVAudioSessionPortDescription
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, preferredInput)
    }
    unsafe fn prefersNoInterruptionsFromSystemAlerts(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, prefersNoInterruptionsFromSystemAlerts)
    }
    unsafe fn renderingMode(&self) -> AVAudioSessionRenderingMode
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, renderingMode)
    }
    unsafe fn prefersEchoCancelledInput(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, prefersEchoCancelledInput)
    }
    unsafe fn isEchoCancelledInputEnabled(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isEchoCancelledInputEnabled)
    }
    unsafe fn isEchoCancelledInputAvailable(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isEchoCancelledInputAvailable)
    }
    unsafe fn isOutputMuted(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isOutputMuted)
    }
    unsafe fn sharedInstance() -> AVAudioSession
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"AVAudioSession").unwrap(), sharedInstance)
    }
}
impl AVAudioSession_Activation for AVAudioSession {}
pub trait AVAudioSession_Activation: Sized + std::ops::Deref {
    unsafe fn setActive_error_(&self, active: BOOL, outError: *mut NSError) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setActive : active, error : outError)
    }
    unsafe fn setActive_withOptions_error_(
        &self,
        active: BOOL,
        options: AVAudioSessionSetActiveOptions,
        outError: *mut NSError,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setActive : active, withOptions : options, error : outError)
    }
    unsafe fn activateWithOptions_completionHandler_(
        &self,
        options: AVAudioSessionActivationOptions,
        handler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, activateWithOptions : options, completionHandler : handler)
    }
}
impl AVAudioSession_AVAudioSessionHardwareConfiguration for AVAudioSession {}
pub trait AVAudioSession_AVAudioSessionHardwareConfiguration: Sized + std::ops::Deref {
    unsafe fn setPreferredSampleRate_error_(&self, sampleRate: f64, outError: *mut NSError) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPreferredSampleRate : sampleRate, error : outError)
    }
    unsafe fn setPreferredIOBufferDuration_error_(
        &self,
        duration: NSTimeInterval,
        outError: *mut NSError,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPreferredIOBufferDuration : duration, error : outError)
    }
    unsafe fn setPreferredInputNumberOfChannels_error_(
        &self,
        count: NSInteger,
        outError: *mut NSError,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPreferredInputNumberOfChannels : count, error : outError)
    }
    unsafe fn setPreferredOutputNumberOfChannels_error_(
        &self,
        count: NSInteger,
        outError: *mut NSError,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPreferredOutputNumberOfChannels : count, error : outError)
    }
    unsafe fn setPreferredInputOrientation_error_(
        &self,
        orientation: AVAudioStereoOrientation,
        outError: *mut NSError,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPreferredInputOrientation : orientation, error : outError)
    }
    unsafe fn setInputGain_error_(&self, gain: f32, outError: *mut NSError) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInputGain : gain, error : outError)
    }
    unsafe fn setInputDataSource_error_(
        &self,
        dataSource: AVAudioSessionDataSourceDescription,
        outError: *mut NSError,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInputDataSource : dataSource, error : outError)
    }
    unsafe fn setOutputDataSource_error_(
        &self,
        dataSource: AVAudioSessionDataSourceDescription,
        outError: *mut NSError,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setOutputDataSource : dataSource, error : outError)
    }
    unsafe fn preferredSampleRate(&self) -> f64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, preferredSampleRate)
    }
    unsafe fn preferredIOBufferDuration(&self) -> NSTimeInterval
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, preferredIOBufferDuration)
    }
    unsafe fn preferredInputNumberOfChannels(&self) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, preferredInputNumberOfChannels)
    }
    unsafe fn preferredOutputNumberOfChannels(&self) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, preferredOutputNumberOfChannels)
    }
    unsafe fn preferredInputOrientation(&self) -> AVAudioStereoOrientation
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, preferredInputOrientation)
    }
    unsafe fn inputOrientation(&self) -> AVAudioStereoOrientation
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inputOrientation)
    }
    unsafe fn maximumInputNumberOfChannels(&self) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, maximumInputNumberOfChannels)
    }
    unsafe fn maximumOutputNumberOfChannels(&self) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, maximumOutputNumberOfChannels)
    }
    unsafe fn inputGain(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inputGain)
    }
    unsafe fn isInputGainSettable(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isInputGainSettable)
    }
    unsafe fn isInputAvailable(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isInputAvailable)
    }
    unsafe fn inputDataSources(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inputDataSources)
    }
    unsafe fn inputDataSource(&self) -> AVAudioSessionDataSourceDescription
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inputDataSource)
    }
    unsafe fn outputDataSources(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, outputDataSources)
    }
    unsafe fn outputDataSource(&self) -> AVAudioSessionDataSourceDescription
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, outputDataSource)
    }
    unsafe fn sampleRate(&self) -> f64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, sampleRate)
    }
    unsafe fn inputNumberOfChannels(&self) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inputNumberOfChannels)
    }
    unsafe fn outputNumberOfChannels(&self) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, outputNumberOfChannels)
    }
    unsafe fn inputLatency(&self) -> NSTimeInterval
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inputLatency)
    }
    unsafe fn outputLatency(&self) -> NSTimeInterval
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, outputLatency)
    }
    unsafe fn IOBufferDuration(&self) -> NSTimeInterval
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, IOBufferDuration)
    }
    unsafe fn supportedOutputChannelLayouts(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, supportedOutputChannelLayouts)
    }
}
impl AVAudioSession_Observation for AVAudioSession {}
pub trait AVAudioSession_Observation: Sized + std::ops::Deref {
    unsafe fn isOtherAudioPlaying(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isOtherAudioPlaying)
    }
    unsafe fn secondaryAudioShouldBeSilencedHint(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, secondaryAudioShouldBeSilencedHint)
    }
    unsafe fn outputVolume(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, outputVolume)
    }
    unsafe fn promptStyle(&self) -> AVAudioSessionPromptStyle
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, promptStyle)
    }
}
impl AVAudioSession_RoutingConfiguration for AVAudioSession {}
pub trait AVAudioSession_RoutingConfiguration: Sized + std::ops::Deref {
    unsafe fn setAggregatedIOPreference_error_(
        &self,
        inIOType: AVAudioSessionIOType,
        outError: *mut NSError,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAggregatedIOPreference : inIOType, error : outError)
    }
    unsafe fn setSupportsMultichannelContent_error_(
        &self,
        inValue: BOOL,
        outError: *mut NSError,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSupportsMultichannelContent : inValue, error : outError)
    }
    unsafe fn setPrefersInterruptionOnRouteDisconnect_error_(
        &self,
        inValue: BOOL,
        outError: *mut NSError,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPrefersInterruptionOnRouteDisconnect : inValue, error : outError)
    }
    unsafe fn availableInputs(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, availableInputs)
    }
    unsafe fn currentRoute(&self) -> AVAudioSessionRouteDescription
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, currentRoute)
    }
    unsafe fn supportsMultichannelContent(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, supportsMultichannelContent)
    }
    unsafe fn prefersInterruptionOnRouteDisconnect(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, prefersInterruptionOnRouteDisconnect)
    }
}
impl AVAudioSession_MicrophoneInjection for AVAudioSession {}
pub trait AVAudioSession_MicrophoneInjection: Sized + std::ops::Deref {
    unsafe fn setPreferredMicrophoneInjectionMode_error_(
        &self,
        inValue: AVAudioSessionMicrophoneInjectionMode,
        outError: *mut NSError,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPreferredMicrophoneInjectionMode : inValue, error : outError)
    }
    unsafe fn preferredMicrophoneInjectionMode(&self) -> AVAudioSessionMicrophoneInjectionMode
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, preferredMicrophoneInjectionMode)
    }
    unsafe fn isMicrophoneInjectionAvailable(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isMicrophoneInjectionAvailable)
    }
}
impl AVAudioSession_AVAudioSessionDeprecated for AVAudioSession {}
pub trait AVAudioSession_AVAudioSessionDeprecated: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn setActive_withFlags_error_(
        &self,
        active: BOOL,
        flags: NSInteger,
        outError: *mut NSError,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setActive : active, withFlags : flags, error : outError)
    }
    unsafe fn setPreferredHardwareSampleRate_error_(
        &self,
        sampleRate: f64,
        outError: *mut NSError,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPreferredHardwareSampleRate : sampleRate, error : outError)
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
    unsafe fn inputIsAvailable(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inputIsAvailable)
    }
    unsafe fn currentHardwareSampleRate(&self) -> f64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, currentHardwareSampleRate)
    }
    unsafe fn currentHardwareInputNumberOfChannels(&self) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, currentHardwareInputNumberOfChannels)
    }
    unsafe fn currentHardwareOutputNumberOfChannels(&self) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, currentHardwareOutputNumberOfChannels)
    }
    unsafe fn preferredHardwareSampleRate(&self) -> f64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, preferredHardwareSampleRate)
    }
}
pub trait PAVAudioSessionDelegate: Sized + std::ops::Deref {
    unsafe fn beginInterruption(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, beginInterruption)
    }
    unsafe fn endInterruptionWithFlags_(&self, flags: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, endInterruptionWithFlags : flags)
    }
    unsafe fn endInterruption(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, endInterruption)
    }
    unsafe fn inputIsAvailableChanged_(&self, isInputAvailable: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, inputIsAvailableChanged : isInputAvailable)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct AVAudioPlayer(pub id);
impl std::ops::Deref for AVAudioPlayer {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for AVAudioPlayer {}
impl AVAudioPlayer {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"AVAudioPlayer").unwrap(), alloc) })
    }
}
impl INSObject for AVAudioPlayer {}
impl PNSObject for AVAudioPlayer {}
impl std::convert::TryFrom<NSObject> for AVAudioPlayer {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<AVAudioPlayer, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"AVAudioPlayer").unwrap()) };
        if is_kind_of {
            Ok(AVAudioPlayer(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to AVAudioPlayer")
        }
    }
}
impl IAVAudioPlayer for AVAudioPlayer {}
pub trait IAVAudioPlayer: Sized + std::ops::Deref {
    unsafe fn initWithContentsOfURL_error_(
        &self,
        url: NSURL,
        outError: *mut NSError,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithContentsOfURL : url, error : outError)
    }
    unsafe fn initWithData_error_(&self, data: NSData, outError: *mut NSError) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithData : data, error : outError)
    }
    unsafe fn initWithContentsOfURL_fileTypeHint_error_(
        &self,
        url: NSURL,
        utiString: NSString,
        outError: *mut NSError,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithContentsOfURL : url, fileTypeHint : utiString, error : outError)
    }
    unsafe fn initWithData_fileTypeHint_error_(
        &self,
        data: NSData,
        utiString: NSString,
        outError: *mut NSError,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithData : data, fileTypeHint : utiString, error : outError)
    }
    unsafe fn prepareToPlay(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, prepareToPlay)
    }
    unsafe fn play(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, play)
    }
    unsafe fn playAtTime_(&self, time: NSTimeInterval) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, playAtTime : time)
    }
    unsafe fn pause(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, pause)
    }
    unsafe fn stop(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, stop)
    }
    unsafe fn setVolume_fadeDuration_(&self, volume: f32, duration: NSTimeInterval)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setVolume : volume, fadeDuration : duration)
    }
    unsafe fn updateMeters(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, updateMeters)
    }
    unsafe fn peakPowerForChannel_(&self, channelNumber: NSUInteger) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, peakPowerForChannel : channelNumber)
    }
    unsafe fn averagePowerForChannel_(&self, channelNumber: NSUInteger) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, averagePowerForChannel : channelNumber)
    }
    unsafe fn isPlaying(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isPlaying)
    }
    unsafe fn numberOfChannels(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, numberOfChannels)
    }
    unsafe fn duration(&self) -> NSTimeInterval
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, duration)
    }
    unsafe fn currentDevice(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, currentDevice)
    }
    unsafe fn setCurrentDevice_(&self, currentDevice: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCurrentDevice : currentDevice)
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
    unsafe fn url(&self) -> NSURL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, url)
    }
    unsafe fn data(&self) -> NSData
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, data)
    }
    unsafe fn pan(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, pan)
    }
    unsafe fn setPan_(&self, pan: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPan : pan)
    }
    unsafe fn volume(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, volume)
    }
    unsafe fn setVolume_(&self, volume: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setVolume : volume)
    }
    unsafe fn enableRate(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, enableRate)
    }
    unsafe fn setEnableRate_(&self, enableRate: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setEnableRate : enableRate)
    }
    unsafe fn rate(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, rate)
    }
    unsafe fn setRate_(&self, rate: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRate : rate)
    }
    unsafe fn currentTime(&self) -> NSTimeInterval
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, currentTime)
    }
    unsafe fn setCurrentTime_(&self, currentTime: NSTimeInterval)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCurrentTime : currentTime)
    }
    unsafe fn deviceCurrentTime(&self) -> NSTimeInterval
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, deviceCurrentTime)
    }
    unsafe fn numberOfLoops(&self) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, numberOfLoops)
    }
    unsafe fn setNumberOfLoops_(&self, numberOfLoops: NSInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setNumberOfLoops : numberOfLoops)
    }
    unsafe fn settings(&self) -> NSDictionary
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, settings)
    }
    unsafe fn format(&self) -> AVAudioFormat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, format)
    }
    unsafe fn isMeteringEnabled(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isMeteringEnabled)
    }
    unsafe fn setMeteringEnabled_(&self, meteringEnabled: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMeteringEnabled : meteringEnabled)
    }
    unsafe fn channelAssignments(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, channelAssignments)
    }
    unsafe fn setChannelAssignments_(&self, channelAssignments: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setChannelAssignments : channelAssignments)
    }
    unsafe fn intendedSpatialExperience(&self) -> CASpatialAudioExperience
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, intendedSpatialExperience)
    }
    unsafe fn setIntendedSpatialExperience_(
        &self,
        intendedSpatialExperience: CASpatialAudioExperience,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setIntendedSpatialExperience : intendedSpatialExperience)
    }
}
pub trait PAVAudioPlayerDelegate: Sized + std::ops::Deref {
    unsafe fn audioPlayerDidFinishPlaying_successfully_(&self, player: AVAudioPlayer, flag: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, audioPlayerDidFinishPlaying : player, successfully : flag)
    }
    unsafe fn audioPlayerDecodeErrorDidOccur_error_(&self, player: AVAudioPlayer, error: NSError)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, audioPlayerDecodeErrorDidOccur : player, error : error)
    }
}
pub type AVAudioPlayerNodeBufferOptions = NSUInteger;
pub type AVAudioPlayerNodeCompletionCallbackType = NSInteger;
pub type AVAudioPlayerNodeCompletionHandler = *mut ::std::os::raw::c_void;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct AVAudioPlayerNode(pub id);
impl std::ops::Deref for AVAudioPlayerNode {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for AVAudioPlayerNode {}
impl AVAudioPlayerNode {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"AVAudioPlayerNode").unwrap(), alloc) })
    }
}
impl PAVAudioMixing for AVAudioPlayerNode {}
impl IAVAudioNode for AVAudioPlayerNode {}
impl std::convert::TryFrom<AVAudioNode> for AVAudioPlayerNode {
    type Error = &'static str;
    fn try_from(parent: AVAudioNode) -> Result<AVAudioPlayerNode, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"AVAudioPlayerNode").unwrap()) };
        if is_kind_of {
            Ok(AVAudioPlayerNode(parent.0))
        } else {
            Err("This AVAudioNode cannot be downcasted to AVAudioPlayerNode")
        }
    }
}
impl INSObject for AVAudioPlayerNode {}
impl PNSObject for AVAudioPlayerNode {}
impl IAVAudioPlayerNode for AVAudioPlayerNode {}
pub trait IAVAudioPlayerNode: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn scheduleBuffer_completionHandler_(
        &self,
        buffer: AVAudioPCMBuffer,
        completionHandler: AVAudioNodeCompletionHandler,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, scheduleBuffer : buffer, completionHandler : completionHandler)
    }
    unsafe fn scheduleBuffer_completionCallbackType_completionHandler_(
        &self,
        buffer: AVAudioPCMBuffer,
        callbackType: AVAudioPlayerNodeCompletionCallbackType,
        completionHandler: AVAudioPlayerNodeCompletionHandler,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, scheduleBuffer : buffer, completionCallbackType : callbackType, completionHandler : completionHandler)
    }
    unsafe fn scheduleBuffer_atTime_options_completionHandler_(
        &self,
        buffer: AVAudioPCMBuffer,
        when: AVAudioTime,
        options: AVAudioPlayerNodeBufferOptions,
        completionHandler: AVAudioNodeCompletionHandler,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, scheduleBuffer : buffer, atTime : when, options : options, completionHandler : completionHandler)
    }
    unsafe fn scheduleBuffer_atTime_options_completionCallbackType_completionHandler_(
        &self,
        buffer: AVAudioPCMBuffer,
        when: AVAudioTime,
        options: AVAudioPlayerNodeBufferOptions,
        callbackType: AVAudioPlayerNodeCompletionCallbackType,
        completionHandler: AVAudioPlayerNodeCompletionHandler,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, scheduleBuffer : buffer, atTime : when, options : options, completionCallbackType : callbackType, completionHandler : completionHandler)
    }
    unsafe fn scheduleFile_atTime_completionHandler_(
        &self,
        file: AVAudioFile,
        when: AVAudioTime,
        completionHandler: AVAudioNodeCompletionHandler,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, scheduleFile : file, atTime : when, completionHandler : completionHandler)
    }
    unsafe fn scheduleFile_atTime_completionCallbackType_completionHandler_(
        &self,
        file: AVAudioFile,
        when: AVAudioTime,
        callbackType: AVAudioPlayerNodeCompletionCallbackType,
        completionHandler: AVAudioPlayerNodeCompletionHandler,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, scheduleFile : file, atTime : when, completionCallbackType : callbackType, completionHandler : completionHandler)
    }
    unsafe fn scheduleSegment_startingFrame_frameCount_atTime_completionHandler_(
        &self,
        file: AVAudioFile,
        startFrame: AVAudioFramePosition,
        numberFrames: AVAudioFrameCount,
        when: AVAudioTime,
        completionHandler: AVAudioNodeCompletionHandler,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, scheduleSegment : file, startingFrame : startFrame, frameCount : numberFrames, atTime : when, completionHandler : completionHandler)
    }
    unsafe fn scheduleSegment_startingFrame_frameCount_atTime_completionCallbackType_completionHandler_(
        &self,
        file: AVAudioFile,
        startFrame: AVAudioFramePosition,
        numberFrames: AVAudioFrameCount,
        when: AVAudioTime,
        callbackType: AVAudioPlayerNodeCompletionCallbackType,
        completionHandler: AVAudioPlayerNodeCompletionHandler,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, scheduleSegment : file, startingFrame : startFrame, frameCount : numberFrames, atTime : when, completionCallbackType : callbackType, completionHandler : completionHandler)
    }
    unsafe fn stop(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, stop)
    }
    unsafe fn prepareWithFrameCount_(&self, frameCount: AVAudioFrameCount)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, prepareWithFrameCount : frameCount)
    }
    unsafe fn play(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, play)
    }
    unsafe fn playAtTime_(&self, when: AVAudioTime)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, playAtTime : when)
    }
    unsafe fn pause(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, pause)
    }
    unsafe fn nodeTimeForPlayerTime_(&self, playerTime: AVAudioTime) -> AVAudioTime
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, nodeTimeForPlayerTime : playerTime)
    }
    unsafe fn playerTimeForNodeTime_(&self, nodeTime: AVAudioTime) -> AVAudioTime
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, playerTimeForNodeTime : nodeTime)
    }
    unsafe fn isPlaying(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isPlaying)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct AVAudioRecorder(pub id);
impl std::ops::Deref for AVAudioRecorder {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for AVAudioRecorder {}
impl AVAudioRecorder {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"AVAudioRecorder").unwrap(), alloc) })
    }
}
impl INSObject for AVAudioRecorder {}
impl PNSObject for AVAudioRecorder {}
impl std::convert::TryFrom<NSObject> for AVAudioRecorder {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<AVAudioRecorder, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"AVAudioRecorder").unwrap()) };
        if is_kind_of {
            Ok(AVAudioRecorder(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to AVAudioRecorder")
        }
    }
}
impl IAVAudioRecorder for AVAudioRecorder {}
pub trait IAVAudioRecorder: Sized + std::ops::Deref {
    unsafe fn initWithURL_settings_error_(
        &self,
        url: NSURL,
        settings: NSDictionary,
        outError: *mut NSError,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithURL : url, settings : settings, error : outError)
    }
    unsafe fn initWithURL_format_error_(
        &self,
        url: NSURL,
        format: AVAudioFormat,
        outError: *mut NSError,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithURL : url, format : format, error : outError)
    }
    unsafe fn prepareToRecord(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, prepareToRecord)
    }
    unsafe fn record(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, record)
    }
    unsafe fn recordAtTime_(&self, time: NSTimeInterval) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, recordAtTime : time)
    }
    unsafe fn recordForDuration_(&self, duration: NSTimeInterval) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, recordForDuration : duration)
    }
    unsafe fn recordAtTime_forDuration_(
        &self,
        time: NSTimeInterval,
        duration: NSTimeInterval,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, recordAtTime : time, forDuration : duration)
    }
    unsafe fn pause(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, pause)
    }
    unsafe fn stop(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, stop)
    }
    unsafe fn deleteRecording(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, deleteRecording)
    }
    unsafe fn updateMeters(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, updateMeters)
    }
    unsafe fn peakPowerForChannel_(&self, channelNumber: NSUInteger) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, peakPowerForChannel : channelNumber)
    }
    unsafe fn averagePowerForChannel_(&self, channelNumber: NSUInteger) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, averagePowerForChannel : channelNumber)
    }
    unsafe fn isRecording(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isRecording)
    }
    unsafe fn url(&self) -> NSURL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, url)
    }
    unsafe fn settings(&self) -> NSDictionary
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, settings)
    }
    unsafe fn format(&self) -> AVAudioFormat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, format)
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
    unsafe fn currentTime(&self) -> NSTimeInterval
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, currentTime)
    }
    unsafe fn deviceCurrentTime(&self) -> NSTimeInterval
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, deviceCurrentTime)
    }
    unsafe fn isMeteringEnabled(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isMeteringEnabled)
    }
    unsafe fn setMeteringEnabled_(&self, meteringEnabled: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMeteringEnabled : meteringEnabled)
    }
    unsafe fn channelAssignments(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, channelAssignments)
    }
    unsafe fn setChannelAssignments_(&self, channelAssignments: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setChannelAssignments : channelAssignments)
    }
}
pub trait PAVAudioRecorderDelegate: Sized + std::ops::Deref {
    unsafe fn audioRecorderDidFinishRecording_successfully_(
        &self,
        recorder: AVAudioRecorder,
        flag: BOOL,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, audioRecorderDidFinishRecording : recorder, successfully : flag)
    }
    unsafe fn audioRecorderEncodeErrorDidOccur_error_(
        &self,
        recorder: AVAudioRecorder,
        error: NSError,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, audioRecorderEncodeErrorDidOccur : recorder, error : error)
    }
}
pub type AVAudioRoutingArbitrationCategory = NSInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct AVAudioRoutingArbiter(pub id);
impl std::ops::Deref for AVAudioRoutingArbiter {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for AVAudioRoutingArbiter {}
impl AVAudioRoutingArbiter {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"AVAudioRoutingArbiter").unwrap(), alloc) })
    }
}
impl INSObject for AVAudioRoutingArbiter {}
impl PNSObject for AVAudioRoutingArbiter {}
impl std::convert::TryFrom<NSObject> for AVAudioRoutingArbiter {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<AVAudioRoutingArbiter, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"AVAudioRoutingArbiter").unwrap()) };
        if is_kind_of {
            Ok(AVAudioRoutingArbiter(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to AVAudioRoutingArbiter")
        }
    }
}
impl IAVAudioRoutingArbiter for AVAudioRoutingArbiter {}
pub trait IAVAudioRoutingArbiter: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn beginArbitrationWithCategory_completionHandler_(
        &self,
        category: AVAudioRoutingArbitrationCategory,
        handler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, beginArbitrationWithCategory : category, completionHandler : handler)
    }
    unsafe fn leaveArbitration(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, leaveArbitration)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"AVAudioRoutingArbiter").unwrap(), new)
    }
    unsafe fn sharedRoutingArbiter() -> AVAudioRoutingArbiter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"AVAudioRoutingArbiter").unwrap(), sharedRoutingArbiter)
    }
}
pub type AVMusicSequenceLoadOptions = NSUInteger;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _AVBeatRange {
    pub start: AVMusicTimeStamp,
    pub length: AVMusicTimeStamp,
}
pub type AVBeatRange = _AVBeatRange;
pub type AVAudioSequencerInfoDictionaryKey = NSString;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct AVAudioSequencer(pub id);
impl std::ops::Deref for AVAudioSequencer {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for AVAudioSequencer {}
impl AVAudioSequencer {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"AVAudioSequencer").unwrap(), alloc) })
    }
}
impl INSObject for AVAudioSequencer {}
impl PNSObject for AVAudioSequencer {}
impl std::convert::TryFrom<NSObject> for AVAudioSequencer {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<AVAudioSequencer, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"AVAudioSequencer").unwrap()) };
        if is_kind_of {
            Ok(AVAudioSequencer(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to AVAudioSequencer")
        }
    }
}
impl IAVAudioSequencer for AVAudioSequencer {}
pub trait IAVAudioSequencer: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn initWithAudioEngine_(&self, engine: AVAudioEngine) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithAudioEngine : engine)
    }
    unsafe fn loadFromURL_options_error_(
        &self,
        fileURL: NSURL,
        options: AVMusicSequenceLoadOptions,
        outError: *mut NSError,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, loadFromURL : fileURL, options : options, error : outError)
    }
    unsafe fn loadFromData_options_error_(
        &self,
        data: NSData,
        options: AVMusicSequenceLoadOptions,
        outError: *mut NSError,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, loadFromData : data, options : options, error : outError)
    }
    unsafe fn writeToURL_SMPTEResolution_replaceExisting_error_(
        &self,
        fileURL: NSURL,
        resolution: NSInteger,
        replace: BOOL,
        outError: *mut NSError,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, writeToURL : fileURL, SMPTEResolution : resolution, replaceExisting : replace, error : outError)
    }
    unsafe fn dataWithSMPTEResolution_error_(
        &self,
        SMPTEResolution: NSInteger,
        outError: *mut NSError,
    ) -> NSData
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, dataWithSMPTEResolution : SMPTEResolution, error : outError)
    }
    unsafe fn secondsForBeats_(&self, beats: AVMusicTimeStamp) -> NSTimeInterval
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, secondsForBeats : beats)
    }
    unsafe fn beatsForSeconds_(&self, seconds: NSTimeInterval) -> AVMusicTimeStamp
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, beatsForSeconds : seconds)
    }
    unsafe fn reverseEvents(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, reverseEvents)
    }
    unsafe fn createAndAppendTrack(&self) -> AVMusicTrack
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, createAndAppendTrack)
    }
    unsafe fn removeTrack_(&self, track: AVMusicTrack) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, removeTrack : track)
    }
    unsafe fn setUserCallback_(&self, userCallback: AVAudioSequencerUserCallback)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setUserCallback : userCallback)
    }
    unsafe fn tracks(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, tracks)
    }
    unsafe fn tempoTrack(&self) -> AVMusicTrack
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, tempoTrack)
    }
    unsafe fn userInfo(&self) -> NSDictionary
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, userInfo)
    }
}
pub type AVAudioSequencerUserCallback = *mut ::std::os::raw::c_void;
impl AVAudioSequencer_AVAudioSequencer_Player for AVAudioSequencer {}
pub trait AVAudioSequencer_AVAudioSequencer_Player: Sized + std::ops::Deref {
    unsafe fn hostTimeForBeats_error_(
        &self,
        inBeats: AVMusicTimeStamp,
        outError: *mut NSError,
    ) -> UInt64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, hostTimeForBeats : inBeats, error : outError)
    }
    unsafe fn beatsForHostTime_error_(
        &self,
        inHostTime: UInt64,
        outError: *mut NSError,
    ) -> AVMusicTimeStamp
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, beatsForHostTime : inHostTime, error : outError)
    }
    unsafe fn prepareToPlay(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, prepareToPlay)
    }
    unsafe fn startAndReturnError_(&self, outError: *mut NSError) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, startAndReturnError : outError)
    }
    unsafe fn stop(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, stop)
    }
    unsafe fn currentPositionInSeconds(&self) -> NSTimeInterval
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, currentPositionInSeconds)
    }
    unsafe fn setCurrentPositionInSeconds_(&self, currentPositionInSeconds: NSTimeInterval)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCurrentPositionInSeconds : currentPositionInSeconds)
    }
    unsafe fn currentPositionInBeats(&self) -> NSTimeInterval
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, currentPositionInBeats)
    }
    unsafe fn setCurrentPositionInBeats_(&self, currentPositionInBeats: NSTimeInterval)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCurrentPositionInBeats : currentPositionInBeats)
    }
    unsafe fn isPlaying(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isPlaying)
    }
    unsafe fn rate(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, rate)
    }
    unsafe fn setRate_(&self, rate: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRate : rate)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct AVMusicTrack(pub id);
impl std::ops::Deref for AVMusicTrack {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for AVMusicTrack {}
impl AVMusicTrack {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"AVMusicTrack").unwrap(), alloc) })
    }
}
impl INSObject for AVMusicTrack {}
impl PNSObject for AVMusicTrack {}
impl std::convert::TryFrom<NSObject> for AVMusicTrack {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<AVMusicTrack, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"AVMusicTrack").unwrap()) };
        if is_kind_of {
            Ok(AVMusicTrack(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to AVMusicTrack")
        }
    }
}
impl IAVMusicTrack for AVMusicTrack {}
pub trait IAVMusicTrack: Sized + std::ops::Deref {
    unsafe fn destinationAudioUnit(&self) -> AVAudioUnit
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, destinationAudioUnit)
    }
    unsafe fn setDestinationAudioUnit_(&self, destinationAudioUnit: AVAudioUnit)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDestinationAudioUnit : destinationAudioUnit)
    }
    unsafe fn destinationMIDIEndpoint(&self) -> MIDIEndpointRef
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, destinationMIDIEndpoint)
    }
    unsafe fn setDestinationMIDIEndpoint_(&self, destinationMIDIEndpoint: MIDIEndpointRef)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDestinationMIDIEndpoint : destinationMIDIEndpoint)
    }
    unsafe fn loopRange(&self) -> AVBeatRange
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, loopRange)
    }
    unsafe fn setLoopRange_(&self, loopRange: AVBeatRange)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setLoopRange : loopRange)
    }
    unsafe fn isLoopingEnabled(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isLoopingEnabled)
    }
    unsafe fn setLoopingEnabled_(&self, loopingEnabled: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setLoopingEnabled : loopingEnabled)
    }
    unsafe fn numberOfLoops(&self) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, numberOfLoops)
    }
    unsafe fn setNumberOfLoops_(&self, numberOfLoops: NSInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setNumberOfLoops : numberOfLoops)
    }
    unsafe fn offsetTime(&self) -> AVMusicTimeStamp
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, offsetTime)
    }
    unsafe fn setOffsetTime_(&self, offsetTime: AVMusicTimeStamp)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setOffsetTime : offsetTime)
    }
    unsafe fn isMuted(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isMuted)
    }
    unsafe fn setMuted_(&self, muted: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMuted : muted)
    }
    unsafe fn isSoloed(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isSoloed)
    }
    unsafe fn setSoloed_(&self, soloed: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSoloed : soloed)
    }
    unsafe fn lengthInBeats(&self) -> AVMusicTimeStamp
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, lengthInBeats)
    }
    unsafe fn setLengthInBeats_(&self, lengthInBeats: AVMusicTimeStamp)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setLengthInBeats : lengthInBeats)
    }
    unsafe fn lengthInSeconds(&self) -> NSTimeInterval
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, lengthInSeconds)
    }
    unsafe fn setLengthInSeconds_(&self, lengthInSeconds: NSTimeInterval)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setLengthInSeconds : lengthInSeconds)
    }
    unsafe fn timeResolution(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, timeResolution)
    }
}
pub type AVMusicTrackLoopCount = NSInteger;
impl AVMusicTrack_AVMusicTrackEditor for AVMusicTrack {}
pub trait AVMusicTrack_AVMusicTrackEditor: Sized + std::ops::Deref {
    unsafe fn addEvent_atBeat_(&self, event: AVMusicEvent, beat: AVMusicTimeStamp)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, addEvent : event, atBeat : beat)
    }
    unsafe fn moveEventsInRange_byAmount_(&self, range: AVBeatRange, beatAmount: AVMusicTimeStamp)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, moveEventsInRange : range, byAmount : beatAmount)
    }
    unsafe fn clearEventsInRange_(&self, range: AVBeatRange)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, clearEventsInRange : range)
    }
    unsafe fn cutEventsInRange_(&self, range: AVBeatRange)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, cutEventsInRange : range)
    }
    unsafe fn copyEventsInRange_fromTrack_insertAtBeat_(
        &self,
        range: AVBeatRange,
        sourceTrack: AVMusicTrack,
        insertStartBeat: AVMusicTimeStamp,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, copyEventsInRange : range, fromTrack : sourceTrack, insertAtBeat : insertStartBeat)
    }
    unsafe fn copyAndMergeEventsInRange_fromTrack_mergeAtBeat_(
        &self,
        range: AVBeatRange,
        sourceTrack: AVMusicTrack,
        mergeStartBeat: AVMusicTimeStamp,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, copyAndMergeEventsInRange : range, fromTrack : sourceTrack, mergeAtBeat : mergeStartBeat)
    }
    unsafe fn enumerateEventsInRange_usingBlock_(
        &self,
        range: AVBeatRange,
        block: AVMusicEventEnumerationBlock,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, enumerateEventsInRange : range, usingBlock : block)
    }
    unsafe fn usesAutomatedParameters(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, usesAutomatedParameters)
    }
    unsafe fn setUsesAutomatedParameters_(&self, usesAutomatedParameters: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setUsesAutomatedParameters : usesAutomatedParameters)
    }
}
pub type AVMusicEventEnumerationBlock = *mut ::std::os::raw::c_void;
pub type AVAudioSinkNodeReceiverBlock = *mut ::std::os::raw::c_void;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct AVAudioSinkNode(pub id);
impl std::ops::Deref for AVAudioSinkNode {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for AVAudioSinkNode {}
impl AVAudioSinkNode {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"AVAudioSinkNode").unwrap(), alloc) })
    }
}
impl IAVAudioNode for AVAudioSinkNode {}
impl std::convert::TryFrom<AVAudioNode> for AVAudioSinkNode {
    type Error = &'static str;
    fn try_from(parent: AVAudioNode) -> Result<AVAudioSinkNode, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"AVAudioSinkNode").unwrap()) };
        if is_kind_of {
            Ok(AVAudioSinkNode(parent.0))
        } else {
            Err("This AVAudioNode cannot be downcasted to AVAudioSinkNode")
        }
    }
}
impl INSObject for AVAudioSinkNode {}
impl PNSObject for AVAudioSinkNode {}
impl IAVAudioSinkNode for AVAudioSinkNode {}
pub trait IAVAudioSinkNode: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn initWithReceiverBlock_(&self, block: AVAudioSinkNodeReceiverBlock) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithReceiverBlock : block)
    }
}
pub type AVAudioSourceNodeRenderBlock = *mut ::std::os::raw::c_void;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct AVAudioSourceNode(pub id);
impl std::ops::Deref for AVAudioSourceNode {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for AVAudioSourceNode {}
impl AVAudioSourceNode {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"AVAudioSourceNode").unwrap(), alloc) })
    }
}
impl PAVAudioMixing for AVAudioSourceNode {}
impl IAVAudioNode for AVAudioSourceNode {}
impl std::convert::TryFrom<AVAudioNode> for AVAudioSourceNode {
    type Error = &'static str;
    fn try_from(parent: AVAudioNode) -> Result<AVAudioSourceNode, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"AVAudioSourceNode").unwrap()) };
        if is_kind_of {
            Ok(AVAudioSourceNode(parent.0))
        } else {
            Err("This AVAudioNode cannot be downcasted to AVAudioSourceNode")
        }
    }
}
impl INSObject for AVAudioSourceNode {}
impl PNSObject for AVAudioSourceNode {}
impl IAVAudioSourceNode for AVAudioSourceNode {}
pub trait IAVAudioSourceNode: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn initWithRenderBlock_(&self, block: AVAudioSourceNodeRenderBlock) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithRenderBlock : block)
    }
    unsafe fn initWithFormat_renderBlock_(
        &self,
        format: AVAudioFormat,
        block: AVAudioSourceNodeRenderBlock,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithFormat : format, renderBlock : block)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct AVAudioUnitComponent(pub id);
impl std::ops::Deref for AVAudioUnitComponent {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for AVAudioUnitComponent {}
impl AVAudioUnitComponent {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"AVAudioUnitComponent").unwrap(), alloc) })
    }
}
impl INSObject for AVAudioUnitComponent {}
impl PNSObject for AVAudioUnitComponent {}
impl std::convert::TryFrom<NSObject> for AVAudioUnitComponent {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<AVAudioUnitComponent, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"AVAudioUnitComponent").unwrap()) };
        if is_kind_of {
            Ok(AVAudioUnitComponent(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to AVAudioUnitComponent")
        }
    }
}
impl IAVAudioUnitComponent for AVAudioUnitComponent {}
pub trait IAVAudioUnitComponent: Sized + std::ops::Deref {
    unsafe fn supportsNumberInputChannels_outputChannels_(
        &self,
        numInputChannels: NSInteger,
        numOutputChannels: NSInteger,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, supportsNumberInputChannels : numInputChannels, outputChannels : numOutputChannels)
    }
    unsafe fn name(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, name)
    }
    unsafe fn typeName(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, typeName)
    }
    unsafe fn localizedTypeName(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, localizedTypeName)
    }
    unsafe fn manufacturerName(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, manufacturerName)
    }
    unsafe fn version(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, version)
    }
    unsafe fn versionString(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, versionString)
    }
    unsafe fn componentURL(&self) -> NSURL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, componentURL)
    }
    unsafe fn availableArchitectures(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, availableArchitectures)
    }
    unsafe fn isSandboxSafe(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isSandboxSafe)
    }
    unsafe fn hasMIDIInput(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, hasMIDIInput)
    }
    unsafe fn hasMIDIOutput(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, hasMIDIOutput)
    }
    unsafe fn audioComponent(&self) -> AudioComponent
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, audioComponent)
    }
    unsafe fn userTagNames(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, userTagNames)
    }
    unsafe fn setUserTagNames_(&self, userTagNames: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setUserTagNames : userTagNames)
    }
    unsafe fn allTagNames(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, allTagNames)
    }
    unsafe fn audioComponentDescription(&self) -> AudioComponentDescription
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, audioComponentDescription)
    }
    unsafe fn iconURL(&self) -> NSURL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, iconURL)
    }
    unsafe fn icon(&self) -> NSImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, icon)
    }
    unsafe fn passesAUVal(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, passesAUVal)
    }
    unsafe fn hasCustomView(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, hasCustomView)
    }
    unsafe fn configurationDictionary(&self) -> NSDictionary
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, configurationDictionary)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct AVAudioUnitComponentManager(pub id);
impl std::ops::Deref for AVAudioUnitComponentManager {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for AVAudioUnitComponentManager {}
impl AVAudioUnitComponentManager {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"AVAudioUnitComponentManager").unwrap(), alloc) })
    }
}
impl INSObject for AVAudioUnitComponentManager {}
impl PNSObject for AVAudioUnitComponentManager {}
impl std::convert::TryFrom<NSObject> for AVAudioUnitComponentManager {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<AVAudioUnitComponentManager, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"AVAudioUnitComponentManager").unwrap()) };
        if is_kind_of {
            Ok(AVAudioUnitComponentManager(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to AVAudioUnitComponentManager")
        }
    }
}
impl IAVAudioUnitComponentManager for AVAudioUnitComponentManager {}
pub trait IAVAudioUnitComponentManager: Sized + std::ops::Deref {
    unsafe fn componentsMatchingPredicate_(&self, predicate: NSPredicate) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, componentsMatchingPredicate : predicate)
    }
    unsafe fn componentsPassingTest_(&self, testHandler: *mut ::std::os::raw::c_void) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, componentsPassingTest : testHandler)
    }
    unsafe fn componentsMatchingDescription_(&self, desc: AudioComponentDescription) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, componentsMatchingDescription : desc)
    }
    unsafe fn tagNames(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, tagNames)
    }
    unsafe fn standardLocalizedTagNames(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, standardLocalizedTagNames)
    }
    unsafe fn sharedAudioUnitComponentManager() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"AVAudioUnitComponentManager").unwrap(), sharedAudioUnitComponentManager)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct AVAudioUnitDelay(pub id);
impl std::ops::Deref for AVAudioUnitDelay {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for AVAudioUnitDelay {}
impl AVAudioUnitDelay {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"AVAudioUnitDelay").unwrap(), alloc) })
    }
}
impl IAVAudioUnitEffect for AVAudioUnitDelay {}
impl std::convert::TryFrom<AVAudioUnitEffect> for AVAudioUnitDelay {
    type Error = &'static str;
    fn try_from(parent: AVAudioUnitEffect) -> Result<AVAudioUnitDelay, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"AVAudioUnitDelay").unwrap()) };
        if is_kind_of {
            Ok(AVAudioUnitDelay(parent.0))
        } else {
            Err("This AVAudioUnitEffect cannot be downcasted to AVAudioUnitDelay")
        }
    }
}
impl IAVAudioUnit for AVAudioUnitDelay {}
impl IAVAudioNode for AVAudioUnitDelay {}
impl INSObject for AVAudioUnitDelay {}
impl PNSObject for AVAudioUnitDelay {}
impl IAVAudioUnitDelay for AVAudioUnitDelay {}
pub trait IAVAudioUnitDelay: Sized + std::ops::Deref {
    unsafe fn delayTime(&self) -> NSTimeInterval
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, delayTime)
    }
    unsafe fn setDelayTime_(&self, delayTime: NSTimeInterval)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDelayTime : delayTime)
    }
    unsafe fn feedback(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, feedback)
    }
    unsafe fn setFeedback_(&self, feedback: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setFeedback : feedback)
    }
    unsafe fn lowPassCutoff(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, lowPassCutoff)
    }
    unsafe fn setLowPassCutoff_(&self, lowPassCutoff: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setLowPassCutoff : lowPassCutoff)
    }
    unsafe fn wetDryMix(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, wetDryMix)
    }
    unsafe fn setWetDryMix_(&self, wetDryMix: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setWetDryMix : wetDryMix)
    }
}
pub type AVAudioUnitDistortionPreset = NSInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct AVAudioUnitDistortion(pub id);
impl std::ops::Deref for AVAudioUnitDistortion {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for AVAudioUnitDistortion {}
impl AVAudioUnitDistortion {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"AVAudioUnitDistortion").unwrap(), alloc) })
    }
}
impl IAVAudioUnitEffect for AVAudioUnitDistortion {}
impl std::convert::TryFrom<AVAudioUnitEffect> for AVAudioUnitDistortion {
    type Error = &'static str;
    fn try_from(parent: AVAudioUnitEffect) -> Result<AVAudioUnitDistortion, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"AVAudioUnitDistortion").unwrap()) };
        if is_kind_of {
            Ok(AVAudioUnitDistortion(parent.0))
        } else {
            Err("This AVAudioUnitEffect cannot be downcasted to AVAudioUnitDistortion")
        }
    }
}
impl IAVAudioUnit for AVAudioUnitDistortion {}
impl IAVAudioNode for AVAudioUnitDistortion {}
impl INSObject for AVAudioUnitDistortion {}
impl PNSObject for AVAudioUnitDistortion {}
impl IAVAudioUnitDistortion for AVAudioUnitDistortion {}
pub trait IAVAudioUnitDistortion: Sized + std::ops::Deref {
    unsafe fn loadFactoryPreset_(&self, preset: AVAudioUnitDistortionPreset)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, loadFactoryPreset : preset)
    }
    unsafe fn preGain(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, preGain)
    }
    unsafe fn setPreGain_(&self, preGain: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPreGain : preGain)
    }
    unsafe fn wetDryMix(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, wetDryMix)
    }
    unsafe fn setWetDryMix_(&self, wetDryMix: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setWetDryMix : wetDryMix)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct AVAudioUnitGenerator(pub id);
impl std::ops::Deref for AVAudioUnitGenerator {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for AVAudioUnitGenerator {}
impl AVAudioUnitGenerator {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"AVAudioUnitGenerator").unwrap(), alloc) })
    }
}
impl PAVAudioMixing for AVAudioUnitGenerator {}
impl IAVAudioUnit for AVAudioUnitGenerator {}
impl std::convert::TryFrom<AVAudioUnit> for AVAudioUnitGenerator {
    type Error = &'static str;
    fn try_from(parent: AVAudioUnit) -> Result<AVAudioUnitGenerator, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"AVAudioUnitGenerator").unwrap()) };
        if is_kind_of {
            Ok(AVAudioUnitGenerator(parent.0))
        } else {
            Err("This AVAudioUnit cannot be downcasted to AVAudioUnitGenerator")
        }
    }
}
impl IAVAudioNode for AVAudioUnitGenerator {}
impl INSObject for AVAudioUnitGenerator {}
impl PNSObject for AVAudioUnitGenerator {}
impl IAVAudioUnitGenerator for AVAudioUnitGenerator {}
pub trait IAVAudioUnitGenerator: Sized + std::ops::Deref {
    unsafe fn initWithAudioComponentDescription_(
        &self,
        audioComponentDescription: AudioComponentDescription,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithAudioComponentDescription : audioComponentDescription)
    }
    unsafe fn bypass(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, bypass)
    }
    unsafe fn setBypass_(&self, bypass: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setBypass : bypass)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct AVAudioUnitMIDIInstrument(pub id);
impl std::ops::Deref for AVAudioUnitMIDIInstrument {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for AVAudioUnitMIDIInstrument {}
impl AVAudioUnitMIDIInstrument {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"AVAudioUnitMIDIInstrument").unwrap(), alloc) })
    }
}
impl PAVAudioMixing for AVAudioUnitMIDIInstrument {}
impl IAVAudioUnit for AVAudioUnitMIDIInstrument {}
impl std::convert::TryFrom<AVAudioUnit> for AVAudioUnitMIDIInstrument {
    type Error = &'static str;
    fn try_from(parent: AVAudioUnit) -> Result<AVAudioUnitMIDIInstrument, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"AVAudioUnitMIDIInstrument").unwrap()) };
        if is_kind_of {
            Ok(AVAudioUnitMIDIInstrument(parent.0))
        } else {
            Err("This AVAudioUnit cannot be downcasted to AVAudioUnitMIDIInstrument")
        }
    }
}
impl IAVAudioNode for AVAudioUnitMIDIInstrument {}
impl INSObject for AVAudioUnitMIDIInstrument {}
impl PNSObject for AVAudioUnitMIDIInstrument {}
impl IAVAudioUnitMIDIInstrument for AVAudioUnitMIDIInstrument {}
pub trait IAVAudioUnitMIDIInstrument: Sized + std::ops::Deref {
    unsafe fn initWithAudioComponentDescription_(
        &self,
        description: AudioComponentDescription,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithAudioComponentDescription : description)
    }
    unsafe fn startNote_withVelocity_onChannel_(&self, note: u8, velocity: u8, channel: u8)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, startNote : note, withVelocity : velocity, onChannel : channel)
    }
    unsafe fn stopNote_onChannel_(&self, note: u8, channel: u8)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, stopNote : note, onChannel : channel)
    }
    unsafe fn sendController_withValue_onChannel_(&self, controller: u8, value: u8, channel: u8)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, sendController : controller, withValue : value, onChannel : channel)
    }
    unsafe fn sendPitchBend_onChannel_(&self, pitchbend: u16, channel: u8)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, sendPitchBend : pitchbend, onChannel : channel)
    }
    unsafe fn sendPressure_onChannel_(&self, pressure: u8, channel: u8)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, sendPressure : pressure, onChannel : channel)
    }
    unsafe fn sendPressureForKey_withValue_onChannel_(&self, key: u8, value: u8, channel: u8)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, sendPressureForKey : key, withValue : value, onChannel : channel)
    }
    unsafe fn sendProgramChange_onChannel_(&self, program: u8, channel: u8)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, sendProgramChange : program, onChannel : channel)
    }
    unsafe fn sendProgramChange_bankMSB_bankLSB_onChannel_(
        &self,
        program: u8,
        bankMSB: u8,
        bankLSB: u8,
        channel: u8,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, sendProgramChange : program, bankMSB : bankMSB, bankLSB : bankLSB, onChannel : channel)
    }
    unsafe fn sendMIDIEvent_data1_data2_(&self, midiStatus: u8, data1: u8, data2: u8)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, sendMIDIEvent : midiStatus, data1 : data1, data2 : data2)
    }
    unsafe fn sendMIDIEvent_data1_(&self, midiStatus: u8, data1: u8)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, sendMIDIEvent : midiStatus, data1 : data1)
    }
    unsafe fn sendMIDISysExEvent_(&self, midiData: NSData)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, sendMIDISysExEvent : midiData)
    }
    unsafe fn sendMIDIEventList_(&self, eventList: *const MIDIEventList)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, sendMIDIEventList : eventList)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct AVAudioUnitSampler(pub id);
impl std::ops::Deref for AVAudioUnitSampler {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for AVAudioUnitSampler {}
impl AVAudioUnitSampler {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"AVAudioUnitSampler").unwrap(), alloc) })
    }
}
impl IAVAudioUnitMIDIInstrument for AVAudioUnitSampler {}
impl PAVAudioMixing for AVAudioUnitSampler {}
impl From<AVAudioUnitSampler> for AVAudioUnitMIDIInstrument {
    fn from(child: AVAudioUnitSampler) -> AVAudioUnitMIDIInstrument {
        AVAudioUnitMIDIInstrument(child.0)
    }
}
impl std::convert::TryFrom<AVAudioUnitMIDIInstrument> for AVAudioUnitSampler {
    type Error = &'static str;
    fn try_from(parent: AVAudioUnitMIDIInstrument) -> Result<AVAudioUnitSampler, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"AVAudioUnitSampler").unwrap()) };
        if is_kind_of {
            Ok(AVAudioUnitSampler(parent.0))
        } else {
            Err("This AVAudioUnitMIDIInstrument cannot be downcasted to AVAudioUnitSampler")
        }
    }
}
impl IAVAudioUnit for AVAudioUnitSampler {}
impl IAVAudioNode for AVAudioUnitSampler {}
impl INSObject for AVAudioUnitSampler {}
impl PNSObject for AVAudioUnitSampler {}
impl IAVAudioUnitSampler for AVAudioUnitSampler {}
pub trait IAVAudioUnitSampler: Sized + std::ops::Deref {
    unsafe fn loadSoundBankInstrumentAtURL_program_bankMSB_bankLSB_error_(
        &self,
        bankURL: NSURL,
        program: u8,
        bankMSB: u8,
        bankLSB: u8,
        outError: *mut NSError,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, loadSoundBankInstrumentAtURL : bankURL, program : program, bankMSB : bankMSB, bankLSB : bankLSB, error : outError)
    }
    unsafe fn loadInstrumentAtURL_error_(
        &self,
        instrumentURL: NSURL,
        outError: *mut NSError,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, loadInstrumentAtURL : instrumentURL, error : outError)
    }
    unsafe fn loadAudioFilesAtURLs_error_(
        &self,
        audioFiles: NSArray,
        outError: *mut NSError,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, loadAudioFilesAtURLs : audioFiles, error : outError)
    }
    unsafe fn stereoPan(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, stereoPan)
    }
    unsafe fn setStereoPan_(&self, stereoPan: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setStereoPan : stereoPan)
    }
    unsafe fn overallGain(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, overallGain)
    }
    unsafe fn setOverallGain_(&self, overallGain: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setOverallGain : overallGain)
    }
    unsafe fn masterGain(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, masterGain)
    }
    unsafe fn setMasterGain_(&self, masterGain: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMasterGain : masterGain)
    }
    unsafe fn globalTuning(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, globalTuning)
    }
    unsafe fn setGlobalTuning_(&self, globalTuning: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setGlobalTuning : globalTuning)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct AVAudioUnitTimeEffect(pub id);
impl std::ops::Deref for AVAudioUnitTimeEffect {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for AVAudioUnitTimeEffect {}
impl AVAudioUnitTimeEffect {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"AVAudioUnitTimeEffect").unwrap(), alloc) })
    }
}
impl IAVAudioUnit for AVAudioUnitTimeEffect {}
impl std::convert::TryFrom<AVAudioUnit> for AVAudioUnitTimeEffect {
    type Error = &'static str;
    fn try_from(parent: AVAudioUnit) -> Result<AVAudioUnitTimeEffect, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"AVAudioUnitTimeEffect").unwrap()) };
        if is_kind_of {
            Ok(AVAudioUnitTimeEffect(parent.0))
        } else {
            Err("This AVAudioUnit cannot be downcasted to AVAudioUnitTimeEffect")
        }
    }
}
impl IAVAudioNode for AVAudioUnitTimeEffect {}
impl INSObject for AVAudioUnitTimeEffect {}
impl PNSObject for AVAudioUnitTimeEffect {}
impl IAVAudioUnitTimeEffect for AVAudioUnitTimeEffect {}
pub trait IAVAudioUnitTimeEffect: Sized + std::ops::Deref {
    unsafe fn initWithAudioComponentDescription_(
        &self,
        audioComponentDescription: AudioComponentDescription,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithAudioComponentDescription : audioComponentDescription)
    }
    unsafe fn bypass(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, bypass)
    }
    unsafe fn setBypass_(&self, bypass: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setBypass : bypass)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct AVAudioUnitTimePitch(pub id);
impl std::ops::Deref for AVAudioUnitTimePitch {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for AVAudioUnitTimePitch {}
impl AVAudioUnitTimePitch {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"AVAudioUnitTimePitch").unwrap(), alloc) })
    }
}
impl IAVAudioUnitTimeEffect for AVAudioUnitTimePitch {}
impl From<AVAudioUnitTimePitch> for AVAudioUnitTimeEffect {
    fn from(child: AVAudioUnitTimePitch) -> AVAudioUnitTimeEffect {
        AVAudioUnitTimeEffect(child.0)
    }
}
impl std::convert::TryFrom<AVAudioUnitTimeEffect> for AVAudioUnitTimePitch {
    type Error = &'static str;
    fn try_from(parent: AVAudioUnitTimeEffect) -> Result<AVAudioUnitTimePitch, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"AVAudioUnitTimePitch").unwrap()) };
        if is_kind_of {
            Ok(AVAudioUnitTimePitch(parent.0))
        } else {
            Err("This AVAudioUnitTimeEffect cannot be downcasted to AVAudioUnitTimePitch")
        }
    }
}
impl IAVAudioUnit for AVAudioUnitTimePitch {}
impl IAVAudioNode for AVAudioUnitTimePitch {}
impl INSObject for AVAudioUnitTimePitch {}
impl PNSObject for AVAudioUnitTimePitch {}
impl IAVAudioUnitTimePitch for AVAudioUnitTimePitch {}
pub trait IAVAudioUnitTimePitch: Sized + std::ops::Deref {
    unsafe fn rate(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, rate)
    }
    unsafe fn setRate_(&self, rate: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRate : rate)
    }
    unsafe fn pitch(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, pitch)
    }
    unsafe fn setPitch_(&self, pitch: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPitch : pitch)
    }
    unsafe fn overlap(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, overlap)
    }
    unsafe fn setOverlap_(&self, overlap: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setOverlap : overlap)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct AVAudioUnitVarispeed(pub id);
impl std::ops::Deref for AVAudioUnitVarispeed {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for AVAudioUnitVarispeed {}
impl AVAudioUnitVarispeed {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"AVAudioUnitVarispeed").unwrap(), alloc) })
    }
}
impl IAVAudioUnitTimeEffect for AVAudioUnitVarispeed {}
impl std::convert::TryFrom<AVAudioUnitTimeEffect> for AVAudioUnitVarispeed {
    type Error = &'static str;
    fn try_from(parent: AVAudioUnitTimeEffect) -> Result<AVAudioUnitVarispeed, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"AVAudioUnitVarispeed").unwrap()) };
        if is_kind_of {
            Ok(AVAudioUnitVarispeed(parent.0))
        } else {
            Err("This AVAudioUnitTimeEffect cannot be downcasted to AVAudioUnitVarispeed")
        }
    }
}
impl IAVAudioUnit for AVAudioUnitVarispeed {}
impl IAVAudioNode for AVAudioUnitVarispeed {}
impl INSObject for AVAudioUnitVarispeed {}
impl PNSObject for AVAudioUnitVarispeed {}
impl IAVAudioUnitVarispeed for AVAudioUnitVarispeed {}
pub trait IAVAudioUnitVarispeed: Sized + std::ops::Deref {
    unsafe fn rate(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, rate)
    }
    unsafe fn setRate_(&self, rate: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRate : rate)
    }
}
pub type AVMIDIPlayerCompletionHandler = *mut ::std::os::raw::c_void;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct AVMIDIPlayer(pub id);
impl std::ops::Deref for AVMIDIPlayer {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for AVMIDIPlayer {}
impl AVMIDIPlayer {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"AVMIDIPlayer").unwrap(), alloc) })
    }
}
impl INSObject for AVMIDIPlayer {}
impl PNSObject for AVMIDIPlayer {}
impl std::convert::TryFrom<NSObject> for AVMIDIPlayer {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<AVMIDIPlayer, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"AVMIDIPlayer").unwrap()) };
        if is_kind_of {
            Ok(AVMIDIPlayer(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to AVMIDIPlayer")
        }
    }
}
impl IAVMIDIPlayer for AVMIDIPlayer {}
pub trait IAVMIDIPlayer: Sized + std::ops::Deref {
    unsafe fn initWithContentsOfURL_soundBankURL_error_(
        &self,
        inURL: NSURL,
        bankURL: NSURL,
        outError: *mut NSError,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithContentsOfURL : inURL, soundBankURL : bankURL, error : outError)
    }
    unsafe fn initWithData_soundBankURL_error_(
        &self,
        data: NSData,
        bankURL: NSURL,
        outError: *mut NSError,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithData : data, soundBankURL : bankURL, error : outError)
    }
    unsafe fn prepareToPlay(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, prepareToPlay)
    }
    unsafe fn play_(&self, completionHandler: AVMIDIPlayerCompletionHandler)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, play : completionHandler)
    }
    unsafe fn stop(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, stop)
    }
    unsafe fn duration(&self) -> NSTimeInterval
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, duration)
    }
    unsafe fn isPlaying(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isPlaying)
    }
    unsafe fn rate(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, rate)
    }
    unsafe fn setRate_(&self, rate: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRate : rate)
    }
    unsafe fn currentPosition(&self) -> NSTimeInterval
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, currentPosition)
    }
    unsafe fn setCurrentPosition_(&self, currentPosition: NSTimeInterval)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCurrentPosition : currentPosition)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct AVMusicEvent(pub id);
impl std::ops::Deref for AVMusicEvent {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for AVMusicEvent {}
impl AVMusicEvent {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"AVMusicEvent").unwrap(), alloc) })
    }
}
impl INSObject for AVMusicEvent {}
impl PNSObject for AVMusicEvent {}
impl std::convert::TryFrom<NSObject> for AVMusicEvent {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<AVMusicEvent, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"AVMusicEvent").unwrap()) };
        if is_kind_of {
            Ok(AVMusicEvent(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to AVMusicEvent")
        }
    }
}
impl IAVMusicEvent for AVMusicEvent {}
pub trait IAVMusicEvent: Sized + std::ops::Deref {}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct AVMIDINoteEvent(pub id);
impl std::ops::Deref for AVMIDINoteEvent {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for AVMIDINoteEvent {}
impl AVMIDINoteEvent {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"AVMIDINoteEvent").unwrap(), alloc) })
    }
}
impl IAVMusicEvent for AVMIDINoteEvent {}
impl From<AVMIDINoteEvent> for AVMusicEvent {
    fn from(child: AVMIDINoteEvent) -> AVMusicEvent {
        AVMusicEvent(child.0)
    }
}
impl std::convert::TryFrom<AVMusicEvent> for AVMIDINoteEvent {
    type Error = &'static str;
    fn try_from(parent: AVMusicEvent) -> Result<AVMIDINoteEvent, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"AVMIDINoteEvent").unwrap()) };
        if is_kind_of {
            Ok(AVMIDINoteEvent(parent.0))
        } else {
            Err("This AVMusicEvent cannot be downcasted to AVMIDINoteEvent")
        }
    }
}
impl INSObject for AVMIDINoteEvent {}
impl PNSObject for AVMIDINoteEvent {}
impl IAVMIDINoteEvent for AVMIDINoteEvent {}
pub trait IAVMIDINoteEvent: Sized + std::ops::Deref {
    unsafe fn initWithChannel_key_velocity_duration_(
        &self,
        channel: UInt32,
        keyNum: UInt32,
        velocity: UInt32,
        duration: AVMusicTimeStamp,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithChannel : channel, key : keyNum, velocity : velocity, duration : duration)
    }
    unsafe fn channel(&self) -> UInt32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, channel)
    }
    unsafe fn setChannel_(&self, channel: UInt32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setChannel : channel)
    }
    unsafe fn key(&self) -> UInt32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, key)
    }
    unsafe fn setKey_(&self, key: UInt32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setKey : key)
    }
    unsafe fn velocity(&self) -> UInt32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, velocity)
    }
    unsafe fn setVelocity_(&self, velocity: UInt32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setVelocity : velocity)
    }
    unsafe fn duration(&self) -> AVMusicTimeStamp
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, duration)
    }
    unsafe fn setDuration_(&self, duration: AVMusicTimeStamp)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDuration : duration)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct AVMIDIChannelEvent(pub id);
impl std::ops::Deref for AVMIDIChannelEvent {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for AVMIDIChannelEvent {}
impl AVMIDIChannelEvent {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"AVMIDIChannelEvent").unwrap(), alloc) })
    }
}
impl IAVMusicEvent for AVMIDIChannelEvent {}
impl std::convert::TryFrom<AVMusicEvent> for AVMIDIChannelEvent {
    type Error = &'static str;
    fn try_from(parent: AVMusicEvent) -> Result<AVMIDIChannelEvent, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"AVMIDIChannelEvent").unwrap()) };
        if is_kind_of {
            Ok(AVMIDIChannelEvent(parent.0))
        } else {
            Err("This AVMusicEvent cannot be downcasted to AVMIDIChannelEvent")
        }
    }
}
impl INSObject for AVMIDIChannelEvent {}
impl PNSObject for AVMIDIChannelEvent {}
impl IAVMIDIChannelEvent for AVMIDIChannelEvent {}
pub trait IAVMIDIChannelEvent: Sized + std::ops::Deref {
    unsafe fn channel(&self) -> UInt32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, channel)
    }
    unsafe fn setChannel_(&self, channel: UInt32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setChannel : channel)
    }
}
pub type AVMIDIControlChangeMessageType = NSInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct AVMIDIControlChangeEvent(pub id);
impl std::ops::Deref for AVMIDIControlChangeEvent {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for AVMIDIControlChangeEvent {}
impl AVMIDIControlChangeEvent {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"AVMIDIControlChangeEvent").unwrap(), alloc) })
    }
}
impl IAVMIDIChannelEvent for AVMIDIControlChangeEvent {}
impl From<AVMIDIControlChangeEvent> for AVMIDIChannelEvent {
    fn from(child: AVMIDIControlChangeEvent) -> AVMIDIChannelEvent {
        AVMIDIChannelEvent(child.0)
    }
}
impl std::convert::TryFrom<AVMIDIChannelEvent> for AVMIDIControlChangeEvent {
    type Error = &'static str;
    fn try_from(parent: AVMIDIChannelEvent) -> Result<AVMIDIControlChangeEvent, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"AVMIDIControlChangeEvent").unwrap()) };
        if is_kind_of {
            Ok(AVMIDIControlChangeEvent(parent.0))
        } else {
            Err("This AVMIDIChannelEvent cannot be downcasted to AVMIDIControlChangeEvent")
        }
    }
}
impl IAVMusicEvent for AVMIDIControlChangeEvent {}
impl INSObject for AVMIDIControlChangeEvent {}
impl PNSObject for AVMIDIControlChangeEvent {}
impl IAVMIDIControlChangeEvent for AVMIDIControlChangeEvent {}
pub trait IAVMIDIControlChangeEvent: Sized + std::ops::Deref {
    unsafe fn initWithChannel_messageType_value_(
        &self,
        channel: UInt32,
        messageType: AVMIDIControlChangeMessageType,
        value: UInt32,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithChannel : channel, messageType : messageType, value : value)
    }
    unsafe fn messageType(&self) -> AVMIDIControlChangeMessageType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, messageType)
    }
    unsafe fn value(&self) -> UInt32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, value)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct AVMIDIPolyPressureEvent(pub id);
impl std::ops::Deref for AVMIDIPolyPressureEvent {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for AVMIDIPolyPressureEvent {}
impl AVMIDIPolyPressureEvent {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"AVMIDIPolyPressureEvent").unwrap(), alloc) })
    }
}
impl IAVMIDIChannelEvent for AVMIDIPolyPressureEvent {}
impl std::convert::TryFrom<AVMIDIChannelEvent> for AVMIDIPolyPressureEvent {
    type Error = &'static str;
    fn try_from(parent: AVMIDIChannelEvent) -> Result<AVMIDIPolyPressureEvent, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"AVMIDIPolyPressureEvent").unwrap()) };
        if is_kind_of {
            Ok(AVMIDIPolyPressureEvent(parent.0))
        } else {
            Err("This AVMIDIChannelEvent cannot be downcasted to AVMIDIPolyPressureEvent")
        }
    }
}
impl IAVMusicEvent for AVMIDIPolyPressureEvent {}
impl INSObject for AVMIDIPolyPressureEvent {}
impl PNSObject for AVMIDIPolyPressureEvent {}
impl IAVMIDIPolyPressureEvent for AVMIDIPolyPressureEvent {}
pub trait IAVMIDIPolyPressureEvent: Sized + std::ops::Deref {
    unsafe fn initWithChannel_key_pressure_(
        &self,
        channel: UInt32,
        key: UInt32,
        pressure: UInt32,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithChannel : channel, key : key, pressure : pressure)
    }
    unsafe fn key(&self) -> UInt32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, key)
    }
    unsafe fn setKey_(&self, key: UInt32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setKey : key)
    }
    unsafe fn pressure(&self) -> UInt32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, pressure)
    }
    unsafe fn setPressure_(&self, pressure: UInt32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPressure : pressure)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct AVMIDIProgramChangeEvent(pub id);
impl std::ops::Deref for AVMIDIProgramChangeEvent {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for AVMIDIProgramChangeEvent {}
impl AVMIDIProgramChangeEvent {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"AVMIDIProgramChangeEvent").unwrap(), alloc) })
    }
}
impl IAVMIDIChannelEvent for AVMIDIProgramChangeEvent {}
impl std::convert::TryFrom<AVMIDIChannelEvent> for AVMIDIProgramChangeEvent {
    type Error = &'static str;
    fn try_from(parent: AVMIDIChannelEvent) -> Result<AVMIDIProgramChangeEvent, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"AVMIDIProgramChangeEvent").unwrap()) };
        if is_kind_of {
            Ok(AVMIDIProgramChangeEvent(parent.0))
        } else {
            Err("This AVMIDIChannelEvent cannot be downcasted to AVMIDIProgramChangeEvent")
        }
    }
}
impl IAVMusicEvent for AVMIDIProgramChangeEvent {}
impl INSObject for AVMIDIProgramChangeEvent {}
impl PNSObject for AVMIDIProgramChangeEvent {}
impl IAVMIDIProgramChangeEvent for AVMIDIProgramChangeEvent {}
pub trait IAVMIDIProgramChangeEvent: Sized + std::ops::Deref {
    unsafe fn initWithChannel_programNumber_(
        &self,
        channel: UInt32,
        programNumber: UInt32,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithChannel : channel, programNumber : programNumber)
    }
    unsafe fn programNumber(&self) -> UInt32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, programNumber)
    }
    unsafe fn setProgramNumber_(&self, programNumber: UInt32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setProgramNumber : programNumber)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct AVMIDIChannelPressureEvent(pub id);
impl std::ops::Deref for AVMIDIChannelPressureEvent {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for AVMIDIChannelPressureEvent {}
impl AVMIDIChannelPressureEvent {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"AVMIDIChannelPressureEvent").unwrap(), alloc) })
    }
}
impl IAVMIDIChannelEvent for AVMIDIChannelPressureEvent {}
impl std::convert::TryFrom<AVMIDIChannelEvent> for AVMIDIChannelPressureEvent {
    type Error = &'static str;
    fn try_from(parent: AVMIDIChannelEvent) -> Result<AVMIDIChannelPressureEvent, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"AVMIDIChannelPressureEvent").unwrap()) };
        if is_kind_of {
            Ok(AVMIDIChannelPressureEvent(parent.0))
        } else {
            Err("This AVMIDIChannelEvent cannot be downcasted to AVMIDIChannelPressureEvent")
        }
    }
}
impl IAVMusicEvent for AVMIDIChannelPressureEvent {}
impl INSObject for AVMIDIChannelPressureEvent {}
impl PNSObject for AVMIDIChannelPressureEvent {}
impl IAVMIDIChannelPressureEvent for AVMIDIChannelPressureEvent {}
pub trait IAVMIDIChannelPressureEvent: Sized + std::ops::Deref {
    unsafe fn initWithChannel_pressure_(&self, channel: UInt32, pressure: UInt32) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithChannel : channel, pressure : pressure)
    }
    unsafe fn pressure(&self) -> UInt32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, pressure)
    }
    unsafe fn setPressure_(&self, pressure: UInt32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPressure : pressure)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct AVMIDIPitchBendEvent(pub id);
impl std::ops::Deref for AVMIDIPitchBendEvent {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for AVMIDIPitchBendEvent {}
impl AVMIDIPitchBendEvent {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"AVMIDIPitchBendEvent").unwrap(), alloc) })
    }
}
impl IAVMIDIChannelEvent for AVMIDIPitchBendEvent {}
impl std::convert::TryFrom<AVMIDIChannelEvent> for AVMIDIPitchBendEvent {
    type Error = &'static str;
    fn try_from(parent: AVMIDIChannelEvent) -> Result<AVMIDIPitchBendEvent, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"AVMIDIPitchBendEvent").unwrap()) };
        if is_kind_of {
            Ok(AVMIDIPitchBendEvent(parent.0))
        } else {
            Err("This AVMIDIChannelEvent cannot be downcasted to AVMIDIPitchBendEvent")
        }
    }
}
impl IAVMusicEvent for AVMIDIPitchBendEvent {}
impl INSObject for AVMIDIPitchBendEvent {}
impl PNSObject for AVMIDIPitchBendEvent {}
impl IAVMIDIPitchBendEvent for AVMIDIPitchBendEvent {}
pub trait IAVMIDIPitchBendEvent: Sized + std::ops::Deref {
    unsafe fn initWithChannel_value_(&self, channel: UInt32, value: UInt32) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithChannel : channel, value : value)
    }
    unsafe fn value(&self) -> UInt32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, value)
    }
    unsafe fn setValue_(&self, value: UInt32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setValue : value)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct AVMIDISysexEvent(pub id);
impl std::ops::Deref for AVMIDISysexEvent {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for AVMIDISysexEvent {}
impl AVMIDISysexEvent {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"AVMIDISysexEvent").unwrap(), alloc) })
    }
}
impl IAVMusicEvent for AVMIDISysexEvent {}
impl std::convert::TryFrom<AVMusicEvent> for AVMIDISysexEvent {
    type Error = &'static str;
    fn try_from(parent: AVMusicEvent) -> Result<AVMIDISysexEvent, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"AVMIDISysexEvent").unwrap()) };
        if is_kind_of {
            Ok(AVMIDISysexEvent(parent.0))
        } else {
            Err("This AVMusicEvent cannot be downcasted to AVMIDISysexEvent")
        }
    }
}
impl INSObject for AVMIDISysexEvent {}
impl PNSObject for AVMIDISysexEvent {}
impl IAVMIDISysexEvent for AVMIDISysexEvent {}
pub trait IAVMIDISysexEvent: Sized + std::ops::Deref {
    unsafe fn initWithData_(&self, data: NSData) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithData : data)
    }
    unsafe fn sizeInBytes(&self) -> UInt32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, sizeInBytes)
    }
}
pub type AVMIDIMetaEventType = NSInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct AVMIDIMetaEvent(pub id);
impl std::ops::Deref for AVMIDIMetaEvent {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for AVMIDIMetaEvent {}
impl AVMIDIMetaEvent {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"AVMIDIMetaEvent").unwrap(), alloc) })
    }
}
impl IAVMusicEvent for AVMIDIMetaEvent {}
impl std::convert::TryFrom<AVMusicEvent> for AVMIDIMetaEvent {
    type Error = &'static str;
    fn try_from(parent: AVMusicEvent) -> Result<AVMIDIMetaEvent, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"AVMIDIMetaEvent").unwrap()) };
        if is_kind_of {
            Ok(AVMIDIMetaEvent(parent.0))
        } else {
            Err("This AVMusicEvent cannot be downcasted to AVMIDIMetaEvent")
        }
    }
}
impl INSObject for AVMIDIMetaEvent {}
impl PNSObject for AVMIDIMetaEvent {}
impl IAVMIDIMetaEvent for AVMIDIMetaEvent {}
pub trait IAVMIDIMetaEvent: Sized + std::ops::Deref {
    unsafe fn initWithType_data_(&self, type_: AVMIDIMetaEventType, data: NSData) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithType : type_, data : data)
    }
    unsafe fn type_(&self) -> AVMIDIMetaEventType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, type)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct AVMusicUserEvent(pub id);
impl std::ops::Deref for AVMusicUserEvent {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for AVMusicUserEvent {}
impl AVMusicUserEvent {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"AVMusicUserEvent").unwrap(), alloc) })
    }
}
impl IAVMusicEvent for AVMusicUserEvent {}
impl std::convert::TryFrom<AVMusicEvent> for AVMusicUserEvent {
    type Error = &'static str;
    fn try_from(parent: AVMusicEvent) -> Result<AVMusicUserEvent, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"AVMusicUserEvent").unwrap()) };
        if is_kind_of {
            Ok(AVMusicUserEvent(parent.0))
        } else {
            Err("This AVMusicEvent cannot be downcasted to AVMusicUserEvent")
        }
    }
}
impl INSObject for AVMusicUserEvent {}
impl PNSObject for AVMusicUserEvent {}
impl IAVMusicUserEvent for AVMusicUserEvent {}
pub trait IAVMusicUserEvent: Sized + std::ops::Deref {
    unsafe fn initWithData_(&self, data: NSData) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithData : data)
    }
    unsafe fn sizeInBytes(&self) -> UInt32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, sizeInBytes)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct AVExtendedNoteOnEvent(pub id);
impl std::ops::Deref for AVExtendedNoteOnEvent {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for AVExtendedNoteOnEvent {}
impl AVExtendedNoteOnEvent {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"AVExtendedNoteOnEvent").unwrap(), alloc) })
    }
}
impl IAVMusicEvent for AVExtendedNoteOnEvent {}
impl std::convert::TryFrom<AVMusicEvent> for AVExtendedNoteOnEvent {
    type Error = &'static str;
    fn try_from(parent: AVMusicEvent) -> Result<AVExtendedNoteOnEvent, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"AVExtendedNoteOnEvent").unwrap()) };
        if is_kind_of {
            Ok(AVExtendedNoteOnEvent(parent.0))
        } else {
            Err("This AVMusicEvent cannot be downcasted to AVExtendedNoteOnEvent")
        }
    }
}
impl INSObject for AVExtendedNoteOnEvent {}
impl PNSObject for AVExtendedNoteOnEvent {}
impl IAVExtendedNoteOnEvent for AVExtendedNoteOnEvent {}
pub trait IAVExtendedNoteOnEvent: Sized + std::ops::Deref {
    unsafe fn initWithMIDINote_velocity_groupID_duration_(
        &self,
        midiNote: f32,
        velocity: f32,
        groupID: UInt32,
        duration: AVMusicTimeStamp,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithMIDINote : midiNote, velocity : velocity, groupID : groupID, duration : duration)
    }
    unsafe fn initWithMIDINote_velocity_instrumentID_groupID_duration_(
        &self,
        midiNote: f32,
        velocity: f32,
        instrumentID: UInt32,
        groupID: UInt32,
        duration: AVMusicTimeStamp,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithMIDINote : midiNote, velocity : velocity, instrumentID : instrumentID, groupID : groupID, duration : duration)
    }
    unsafe fn midiNote(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, midiNote)
    }
    unsafe fn setMidiNote_(&self, midiNote: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMidiNote : midiNote)
    }
    unsafe fn velocity(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, velocity)
    }
    unsafe fn setVelocity_(&self, velocity: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setVelocity : velocity)
    }
    unsafe fn instrumentID(&self) -> UInt32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, instrumentID)
    }
    unsafe fn setInstrumentID_(&self, instrumentID: UInt32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInstrumentID : instrumentID)
    }
    unsafe fn groupID(&self) -> UInt32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, groupID)
    }
    unsafe fn setGroupID_(&self, groupID: UInt32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setGroupID : groupID)
    }
    unsafe fn duration(&self) -> AVMusicTimeStamp
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, duration)
    }
    unsafe fn setDuration_(&self, duration: AVMusicTimeStamp)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDuration : duration)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct AVParameterEvent(pub id);
impl std::ops::Deref for AVParameterEvent {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for AVParameterEvent {}
impl AVParameterEvent {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"AVParameterEvent").unwrap(), alloc) })
    }
}
impl IAVMusicEvent for AVParameterEvent {}
impl std::convert::TryFrom<AVMusicEvent> for AVParameterEvent {
    type Error = &'static str;
    fn try_from(parent: AVMusicEvent) -> Result<AVParameterEvent, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"AVParameterEvent").unwrap()) };
        if is_kind_of {
            Ok(AVParameterEvent(parent.0))
        } else {
            Err("This AVMusicEvent cannot be downcasted to AVParameterEvent")
        }
    }
}
impl INSObject for AVParameterEvent {}
impl PNSObject for AVParameterEvent {}
impl IAVParameterEvent for AVParameterEvent {}
pub trait IAVParameterEvent: Sized + std::ops::Deref {
    unsafe fn initWithParameterID_scope_element_value_(
        &self,
        parameterID: UInt32,
        scope: UInt32,
        element: UInt32,
        value: f32,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithParameterID : parameterID, scope : scope, element : element, value : value)
    }
    unsafe fn parameterID(&self) -> UInt32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, parameterID)
    }
    unsafe fn setParameterID_(&self, parameterID: UInt32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setParameterID : parameterID)
    }
    unsafe fn scope(&self) -> UInt32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, scope)
    }
    unsafe fn setScope_(&self, scope: UInt32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setScope : scope)
    }
    unsafe fn element(&self) -> UInt32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, element)
    }
    unsafe fn setElement_(&self, element: UInt32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setElement : element)
    }
    unsafe fn value(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, value)
    }
    unsafe fn setValue_(&self, value: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setValue : value)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct AVAUPresetEvent(pub id);
impl std::ops::Deref for AVAUPresetEvent {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for AVAUPresetEvent {}
impl AVAUPresetEvent {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"AVAUPresetEvent").unwrap(), alloc) })
    }
}
impl IAVMusicEvent for AVAUPresetEvent {}
impl std::convert::TryFrom<AVMusicEvent> for AVAUPresetEvent {
    type Error = &'static str;
    fn try_from(parent: AVMusicEvent) -> Result<AVAUPresetEvent, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"AVAUPresetEvent").unwrap()) };
        if is_kind_of {
            Ok(AVAUPresetEvent(parent.0))
        } else {
            Err("This AVMusicEvent cannot be downcasted to AVAUPresetEvent")
        }
    }
}
impl INSObject for AVAUPresetEvent {}
impl PNSObject for AVAUPresetEvent {}
impl IAVAUPresetEvent for AVAUPresetEvent {}
pub trait IAVAUPresetEvent: Sized + std::ops::Deref {
    unsafe fn initWithScope_element_dictionary_(
        &self,
        scope: UInt32,
        element: UInt32,
        presetDictionary: NSDictionary,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithScope : scope, element : element, dictionary : presetDictionary)
    }
    unsafe fn scope(&self) -> UInt32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, scope)
    }
    unsafe fn setScope_(&self, scope: UInt32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setScope : scope)
    }
    unsafe fn element(&self) -> UInt32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, element)
    }
    unsafe fn setElement_(&self, element: UInt32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setElement : element)
    }
    unsafe fn presetDictionary(&self) -> NSDictionary
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, presetDictionary)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct AVExtendedTempoEvent(pub id);
impl std::ops::Deref for AVExtendedTempoEvent {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for AVExtendedTempoEvent {}
impl AVExtendedTempoEvent {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"AVExtendedTempoEvent").unwrap(), alloc) })
    }
}
impl IAVMusicEvent for AVExtendedTempoEvent {}
impl std::convert::TryFrom<AVMusicEvent> for AVExtendedTempoEvent {
    type Error = &'static str;
    fn try_from(parent: AVMusicEvent) -> Result<AVExtendedTempoEvent, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"AVExtendedTempoEvent").unwrap()) };
        if is_kind_of {
            Ok(AVExtendedTempoEvent(parent.0))
        } else {
            Err("This AVMusicEvent cannot be downcasted to AVExtendedTempoEvent")
        }
    }
}
impl INSObject for AVExtendedTempoEvent {}
impl PNSObject for AVExtendedTempoEvent {}
impl IAVExtendedTempoEvent for AVExtendedTempoEvent {}
pub trait IAVExtendedTempoEvent: Sized + std::ops::Deref {
    unsafe fn initWithTempo_(&self, tempo: f64) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithTempo : tempo)
    }
    unsafe fn tempo(&self) -> f64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, tempo)
    }
    unsafe fn setTempo_(&self, tempo: f64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setTempo : tempo)
    }
}
pub type AVSpeechBoundary = NSInteger;
pub type AVSpeechSynthesisVoiceQuality = NSInteger;
pub type AVSpeechSynthesisVoiceGender = NSInteger;
pub type AVSpeechSynthesisMarkerMark = NSInteger;
pub type AVSpeechSynthesizerBufferCallback = *mut ::std::os::raw::c_void;
pub type AVSpeechSynthesizerMarkerCallback = *mut ::std::os::raw::c_void;
pub type AVSpeechSynthesisPersonalVoiceAuthorizationStatus = NSUInteger;
pub type AVSpeechSynthesisVoiceTraits = NSUInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct AVSpeechSynthesisVoice(pub id);
impl std::ops::Deref for AVSpeechSynthesisVoice {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for AVSpeechSynthesisVoice {}
impl AVSpeechSynthesisVoice {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"AVSpeechSynthesisVoice").unwrap(), alloc) })
    }
}
impl PNSSecureCoding for AVSpeechSynthesisVoice {}
impl INSObject for AVSpeechSynthesisVoice {}
impl PNSObject for AVSpeechSynthesisVoice {}
impl std::convert::TryFrom<NSObject> for AVSpeechSynthesisVoice {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<AVSpeechSynthesisVoice, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"AVSpeechSynthesisVoice").unwrap()) };
        if is_kind_of {
            Ok(AVSpeechSynthesisVoice(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to AVSpeechSynthesisVoice")
        }
    }
}
impl IAVSpeechSynthesisVoice for AVSpeechSynthesisVoice {}
pub trait IAVSpeechSynthesisVoice: Sized + std::ops::Deref {
    unsafe fn language(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, language)
    }
    unsafe fn identifier(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, identifier)
    }
    unsafe fn name(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, name)
    }
    unsafe fn quality(&self) -> AVSpeechSynthesisVoiceQuality
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, quality)
    }
    unsafe fn gender(&self) -> AVSpeechSynthesisVoiceGender
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, gender)
    }
    unsafe fn audioFileSettings(&self) -> NSDictionary
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, audioFileSettings)
    }
    unsafe fn voiceTraits(&self) -> AVSpeechSynthesisVoiceTraits
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, voiceTraits)
    }
    unsafe fn speechVoices() -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"AVSpeechSynthesisVoice").unwrap(), speechVoices)
    }
    unsafe fn currentLanguageCode() -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"AVSpeechSynthesisVoice").unwrap(), currentLanguageCode)
    }
    unsafe fn voiceWithLanguage_(languageCode: NSString) -> AVSpeechSynthesisVoice
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"AVSpeechSynthesisVoice").unwrap(), voiceWithLanguage : languageCode)
    }
    unsafe fn voiceWithIdentifier_(identifier: NSString) -> AVSpeechSynthesisVoice
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"AVSpeechSynthesisVoice").unwrap(), voiceWithIdentifier : identifier)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct AVSpeechUtterance(pub id);
impl std::ops::Deref for AVSpeechUtterance {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for AVSpeechUtterance {}
impl AVSpeechUtterance {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"AVSpeechUtterance").unwrap(), alloc) })
    }
}
impl PNSCopying for AVSpeechUtterance {}
impl PNSSecureCoding for AVSpeechUtterance {}
impl INSObject for AVSpeechUtterance {}
impl PNSObject for AVSpeechUtterance {}
impl std::convert::TryFrom<NSObject> for AVSpeechUtterance {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<AVSpeechUtterance, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"AVSpeechUtterance").unwrap()) };
        if is_kind_of {
            Ok(AVSpeechUtterance(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to AVSpeechUtterance")
        }
    }
}
impl IAVSpeechUtterance for AVSpeechUtterance {}
pub trait IAVSpeechUtterance: Sized + std::ops::Deref {
    unsafe fn initWithString_(&self, string: NSString) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithString : string)
    }
    unsafe fn initWithAttributedString_(&self, string: NSAttributedString) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithAttributedString : string)
    }
    unsafe fn initWithSSMLRepresentation_(&self, string: NSString) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithSSMLRepresentation : string)
    }
    unsafe fn voice(&self) -> AVSpeechSynthesisVoice
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, voice)
    }
    unsafe fn setVoice_(&self, voice: AVSpeechSynthesisVoice)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setVoice : voice)
    }
    unsafe fn speechString(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, speechString)
    }
    unsafe fn attributedSpeechString(&self) -> NSAttributedString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, attributedSpeechString)
    }
    unsafe fn rate(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, rate)
    }
    unsafe fn setRate_(&self, rate: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRate : rate)
    }
    unsafe fn pitchMultiplier(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, pitchMultiplier)
    }
    unsafe fn setPitchMultiplier_(&self, pitchMultiplier: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPitchMultiplier : pitchMultiplier)
    }
    unsafe fn volume(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, volume)
    }
    unsafe fn setVolume_(&self, volume: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setVolume : volume)
    }
    unsafe fn prefersAssistiveTechnologySettings(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, prefersAssistiveTechnologySettings)
    }
    unsafe fn setPrefersAssistiveTechnologySettings_(
        &self,
        prefersAssistiveTechnologySettings: BOOL,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPrefersAssistiveTechnologySettings : prefersAssistiveTechnologySettings)
    }
    unsafe fn preUtteranceDelay(&self) -> NSTimeInterval
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, preUtteranceDelay)
    }
    unsafe fn setPreUtteranceDelay_(&self, preUtteranceDelay: NSTimeInterval)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPreUtteranceDelay : preUtteranceDelay)
    }
    unsafe fn postUtteranceDelay(&self) -> NSTimeInterval
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, postUtteranceDelay)
    }
    unsafe fn setPostUtteranceDelay_(&self, postUtteranceDelay: NSTimeInterval)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPostUtteranceDelay : postUtteranceDelay)
    }
    unsafe fn speechUtteranceWithString_(string: NSString) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"AVSpeechUtterance").unwrap(), speechUtteranceWithString : string)
    }
    unsafe fn speechUtteranceWithAttributedString_(string: NSAttributedString) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"AVSpeechUtterance").unwrap(), speechUtteranceWithAttributedString : string)
    }
    unsafe fn speechUtteranceWithSSMLRepresentation_(string: NSString) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"AVSpeechUtterance").unwrap(), speechUtteranceWithSSMLRepresentation : string)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct AVSpeechSynthesizer(pub id);
impl std::ops::Deref for AVSpeechSynthesizer {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for AVSpeechSynthesizer {}
impl AVSpeechSynthesizer {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"AVSpeechSynthesizer").unwrap(), alloc) })
    }
}
impl INSObject for AVSpeechSynthesizer {}
impl PNSObject for AVSpeechSynthesizer {}
impl std::convert::TryFrom<NSObject> for AVSpeechSynthesizer {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<AVSpeechSynthesizer, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"AVSpeechSynthesizer").unwrap()) };
        if is_kind_of {
            Ok(AVSpeechSynthesizer(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to AVSpeechSynthesizer")
        }
    }
}
impl IAVSpeechSynthesizer for AVSpeechSynthesizer {}
pub trait IAVSpeechSynthesizer: Sized + std::ops::Deref {
    unsafe fn speakUtterance_(&self, utterance: AVSpeechUtterance)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, speakUtterance : utterance)
    }
    unsafe fn writeUtterance_toBufferCallback_(
        &self,
        utterance: AVSpeechUtterance,
        bufferCallback: AVSpeechSynthesizerBufferCallback,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, writeUtterance : utterance, toBufferCallback : bufferCallback)
    }
    unsafe fn writeUtterance_toBufferCallback_toMarkerCallback_(
        &self,
        utterance: AVSpeechUtterance,
        bufferCallback: AVSpeechSynthesizerBufferCallback,
        markerCallback: AVSpeechSynthesizerMarkerCallback,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, writeUtterance : utterance, toBufferCallback : bufferCallback, toMarkerCallback : markerCallback)
    }
    unsafe fn stopSpeakingAtBoundary_(&self, boundary: AVSpeechBoundary) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, stopSpeakingAtBoundary : boundary)
    }
    unsafe fn pauseSpeakingAtBoundary_(&self, boundary: AVSpeechBoundary) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, pauseSpeakingAtBoundary : boundary)
    }
    unsafe fn continueSpeaking(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, continueSpeaking)
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
    unsafe fn isSpeaking(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isSpeaking)
    }
    unsafe fn isPaused(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isPaused)
    }
    unsafe fn outputChannels(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, outputChannels)
    }
    unsafe fn setOutputChannels_(&self, outputChannels: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setOutputChannels : outputChannels)
    }
    unsafe fn usesApplicationAudioSession(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, usesApplicationAudioSession)
    }
    unsafe fn setUsesApplicationAudioSession_(&self, usesApplicationAudioSession: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setUsesApplicationAudioSession : usesApplicationAudioSession)
    }
    unsafe fn mixToTelephonyUplink(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, mixToTelephonyUplink)
    }
    unsafe fn setMixToTelephonyUplink_(&self, mixToTelephonyUplink: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMixToTelephonyUplink : mixToTelephonyUplink)
    }
    unsafe fn requestPersonalVoiceAuthorizationWithCompletionHandler_(
        handler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"AVSpeechSynthesizer").unwrap(), requestPersonalVoiceAuthorizationWithCompletionHandler : handler)
    }
    unsafe fn personalVoiceAuthorizationStatus() -> AVSpeechSynthesisPersonalVoiceAuthorizationStatus
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"AVSpeechSynthesizer").unwrap(), personalVoiceAuthorizationStatus)
    }
}
pub trait PAVSpeechSynthesizerDelegate: Sized + std::ops::Deref {
    unsafe fn speechSynthesizer_didStartSpeechUtterance_(
        &self,
        synthesizer: AVSpeechSynthesizer,
        utterance: AVSpeechUtterance,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, speechSynthesizer : synthesizer, didStartSpeechUtterance : utterance)
    }
    unsafe fn speechSynthesizer_didFinishSpeechUtterance_(
        &self,
        synthesizer: AVSpeechSynthesizer,
        utterance: AVSpeechUtterance,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, speechSynthesizer : synthesizer, didFinishSpeechUtterance : utterance)
    }
    unsafe fn speechSynthesizer_didPauseSpeechUtterance_(
        &self,
        synthesizer: AVSpeechSynthesizer,
        utterance: AVSpeechUtterance,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, speechSynthesizer : synthesizer, didPauseSpeechUtterance : utterance)
    }
    unsafe fn speechSynthesizer_didContinueSpeechUtterance_(
        &self,
        synthesizer: AVSpeechSynthesizer,
        utterance: AVSpeechUtterance,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, speechSynthesizer : synthesizer, didContinueSpeechUtterance : utterance)
    }
    unsafe fn speechSynthesizer_didCancelSpeechUtterance_(
        &self,
        synthesizer: AVSpeechSynthesizer,
        utterance: AVSpeechUtterance,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, speechSynthesizer : synthesizer, didCancelSpeechUtterance : utterance)
    }
    unsafe fn speechSynthesizer_willSpeakRangeOfSpeechString_utterance_(
        &self,
        synthesizer: AVSpeechSynthesizer,
        characterRange: NSRange,
        utterance: AVSpeechUtterance,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, speechSynthesizer : synthesizer, willSpeakRangeOfSpeechString : characterRange, utterance : utterance)
    }
    unsafe fn speechSynthesizer_willSpeakMarker_utterance_(
        &self,
        synthesizer: AVSpeechSynthesizer,
        marker: AVSpeechSynthesisMarker,
        utterance: AVSpeechUtterance,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, speechSynthesizer : synthesizer, willSpeakMarker : marker, utterance : utterance)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct AVSpeechSynthesisMarker(pub id);
impl std::ops::Deref for AVSpeechSynthesisMarker {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for AVSpeechSynthesisMarker {}
impl AVSpeechSynthesisMarker {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"AVSpeechSynthesisMarker").unwrap(), alloc) })
    }
}
impl PNSSecureCoding for AVSpeechSynthesisMarker {}
impl PNSCopying for AVSpeechSynthesisMarker {}
impl INSObject for AVSpeechSynthesisMarker {}
impl PNSObject for AVSpeechSynthesisMarker {}
impl std::convert::TryFrom<NSObject> for AVSpeechSynthesisMarker {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<AVSpeechSynthesisMarker, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"AVSpeechSynthesisMarker").unwrap()) };
        if is_kind_of {
            Ok(AVSpeechSynthesisMarker(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to AVSpeechSynthesisMarker")
        }
    }
}
impl IAVSpeechSynthesisMarker for AVSpeechSynthesisMarker {}
pub trait IAVSpeechSynthesisMarker: Sized + std::ops::Deref {
    unsafe fn initWithMarkerType_forTextRange_atByteSampleOffset_(
        &self,
        type_: AVSpeechSynthesisMarkerMark,
        range: NSRange,
        byteSampleOffset: NSUInteger,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithMarkerType : type_, forTextRange : range, atByteSampleOffset : byteSampleOffset)
    }
    unsafe fn initWithWordRange_atByteSampleOffset_(
        &self,
        range: NSRange,
        byteSampleOffset: NSInteger,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithWordRange : range, atByteSampleOffset : byteSampleOffset)
    }
    unsafe fn initWithSentenceRange_atByteSampleOffset_(
        &self,
        range: NSRange,
        byteSampleOffset: NSInteger,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithSentenceRange : range, atByteSampleOffset : byteSampleOffset)
    }
    unsafe fn initWithParagraphRange_atByteSampleOffset_(
        &self,
        range: NSRange,
        byteSampleOffset: NSInteger,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithParagraphRange : range, atByteSampleOffset : byteSampleOffset)
    }
    unsafe fn initWithPhonemeString_atByteSampleOffset_(
        &self,
        phoneme: NSString,
        byteSampleOffset: NSInteger,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithPhonemeString : phoneme, atByteSampleOffset : byteSampleOffset)
    }
    unsafe fn initWithBookmarkName_atByteSampleOffset_(
        &self,
        mark: NSString,
        byteSampleOffset: NSInteger,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithBookmarkName : mark, atByteSampleOffset : byteSampleOffset)
    }
    unsafe fn mark(&self) -> AVSpeechSynthesisMarkerMark
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, mark)
    }
    unsafe fn setMark_(&self, mark: AVSpeechSynthesisMarkerMark)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMark : mark)
    }
    unsafe fn byteSampleOffset(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, byteSampleOffset)
    }
    unsafe fn setByteSampleOffset_(&self, byteSampleOffset: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setByteSampleOffset : byteSampleOffset)
    }
    unsafe fn textRange(&self) -> NSRange
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, textRange)
    }
    unsafe fn setTextRange_(&self, textRange: NSRange)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setTextRange : textRange)
    }
    unsafe fn bookmarkName(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, bookmarkName)
    }
    unsafe fn setBookmarkName_(&self, bookmarkName: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setBookmarkName : bookmarkName)
    }
    unsafe fn phoneme(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, phoneme)
    }
    unsafe fn setPhoneme_(&self, phoneme: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPhoneme : phoneme)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct AVSpeechSynthesisProviderVoice(pub id);
impl std::ops::Deref for AVSpeechSynthesisProviderVoice {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for AVSpeechSynthesisProviderVoice {}
impl AVSpeechSynthesisProviderVoice {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"AVSpeechSynthesisProviderVoice").unwrap(), alloc) })
    }
}
impl PNSSecureCoding for AVSpeechSynthesisProviderVoice {}
impl PNSCopying for AVSpeechSynthesisProviderVoice {}
impl INSObject for AVSpeechSynthesisProviderVoice {}
impl PNSObject for AVSpeechSynthesisProviderVoice {}
impl std::convert::TryFrom<NSObject> for AVSpeechSynthesisProviderVoice {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<AVSpeechSynthesisProviderVoice, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"AVSpeechSynthesisProviderVoice").unwrap())
        };
        if is_kind_of {
            Ok(AVSpeechSynthesisProviderVoice(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to AVSpeechSynthesisProviderVoice")
        }
    }
}
impl IAVSpeechSynthesisProviderVoice for AVSpeechSynthesisProviderVoice {}
pub trait IAVSpeechSynthesisProviderVoice: Sized + std::ops::Deref {
    unsafe fn initWithName_identifier_primaryLanguages_supportedLanguages_(
        &self,
        name: NSString,
        identifier: NSString,
        primaryLanguages: NSArray,
        supportedLanguages: NSArray,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithName : name, identifier : identifier, primaryLanguages : primaryLanguages, supportedLanguages : supportedLanguages)
    }
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
    unsafe fn identifier(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, identifier)
    }
    unsafe fn primaryLanguages(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, primaryLanguages)
    }
    unsafe fn supportedLanguages(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, supportedLanguages)
    }
    unsafe fn voiceSize(&self) -> i64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, voiceSize)
    }
    unsafe fn setVoiceSize_(&self, voiceSize: i64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setVoiceSize : voiceSize)
    }
    unsafe fn version(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, version)
    }
    unsafe fn setVersion_(&self, version: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setVersion : version)
    }
    unsafe fn gender(&self) -> AVSpeechSynthesisVoiceGender
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, gender)
    }
    unsafe fn setGender_(&self, gender: AVSpeechSynthesisVoiceGender)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setGender : gender)
    }
    unsafe fn age(&self) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, age)
    }
    unsafe fn setAge_(&self, age: NSInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAge : age)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"AVSpeechSynthesisProviderVoice").unwrap(), new)
    }
    unsafe fn updateSpeechVoices()
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"AVSpeechSynthesisProviderVoice").unwrap(), updateSpeechVoices)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct AVSpeechSynthesisProviderRequest(pub id);
impl std::ops::Deref for AVSpeechSynthesisProviderRequest {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for AVSpeechSynthesisProviderRequest {}
impl AVSpeechSynthesisProviderRequest {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"AVSpeechSynthesisProviderRequest").unwrap(), alloc) })
    }
}
impl PNSSecureCoding for AVSpeechSynthesisProviderRequest {}
impl PNSCopying for AVSpeechSynthesisProviderRequest {}
impl INSObject for AVSpeechSynthesisProviderRequest {}
impl PNSObject for AVSpeechSynthesisProviderRequest {}
impl std::convert::TryFrom<NSObject> for AVSpeechSynthesisProviderRequest {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<AVSpeechSynthesisProviderRequest, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"AVSpeechSynthesisProviderRequest").unwrap())
        };
        if is_kind_of {
            Ok(AVSpeechSynthesisProviderRequest(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to AVSpeechSynthesisProviderRequest")
        }
    }
}
impl IAVSpeechSynthesisProviderRequest for AVSpeechSynthesisProviderRequest {}
pub trait IAVSpeechSynthesisProviderRequest: Sized + std::ops::Deref {
    unsafe fn initWithSSMLRepresentation_voice_(
        &self,
        text: NSString,
        voice: AVSpeechSynthesisProviderVoice,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithSSMLRepresentation : text, voice : voice)
    }
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn ssmlRepresentation(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, ssmlRepresentation)
    }
    unsafe fn voice(&self) -> AVSpeechSynthesisProviderVoice
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, voice)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"AVSpeechSynthesisProviderRequest").unwrap(), new)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct AVSpeechSynthesisProviderAudioUnit(pub id);
impl std::ops::Deref for AVSpeechSynthesisProviderAudioUnit {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for AVSpeechSynthesisProviderAudioUnit {}
impl AVSpeechSynthesisProviderAudioUnit {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"AVSpeechSynthesisProviderAudioUnit").unwrap(), alloc) })
    }
}
impl IAUAudioUnit for AVSpeechSynthesisProviderAudioUnit {}
impl std::convert::TryFrom<AUAudioUnit> for AVSpeechSynthesisProviderAudioUnit {
    type Error = &'static str;
    fn try_from(parent: AUAudioUnit) -> Result<AVSpeechSynthesisProviderAudioUnit, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"AVSpeechSynthesisProviderAudioUnit").unwrap())
        };
        if is_kind_of {
            Ok(AVSpeechSynthesisProviderAudioUnit(parent.0))
        } else {
            Err("This AUAudioUnit cannot be downcasted to AVSpeechSynthesisProviderAudioUnit")
        }
    }
}
impl INSObject for AVSpeechSynthesisProviderAudioUnit {}
impl PNSObject for AVSpeechSynthesisProviderAudioUnit {}
impl IAVSpeechSynthesisProviderAudioUnit for AVSpeechSynthesisProviderAudioUnit {}
pub trait IAVSpeechSynthesisProviderAudioUnit: Sized + std::ops::Deref {
    unsafe fn synthesizeSpeechRequest_(&self, speechRequest: AVSpeechSynthesisProviderRequest)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, synthesizeSpeechRequest : speechRequest)
    }
    unsafe fn cancelSpeechRequest(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, cancelSpeechRequest)
    }
    unsafe fn speechVoices(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, speechVoices)
    }
    unsafe fn setSpeechVoices_(&self, speechVoices: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSpeechVoices : speechVoices)
    }
    unsafe fn speechSynthesisOutputMetadataBlock(&self) -> AVSpeechSynthesisProviderOutputBlock
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, speechSynthesisOutputMetadataBlock)
    }
    unsafe fn setSpeechSynthesisOutputMetadataBlock_(
        &self,
        speechSynthesisOutputMetadataBlock: AVSpeechSynthesisProviderOutputBlock,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSpeechSynthesisOutputMetadataBlock : speechSynthesisOutputMetadataBlock)
    }
}
pub type AVSpeechSynthesisProviderOutputBlock = *mut ::std::os::raw::c_void;
pub type AVAudioApplicationRecordPermission = NSInteger;
pub type AVAudioApplicationMicrophoneInjectionPermission = NSInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct AVAudioApplication(pub id);
impl std::ops::Deref for AVAudioApplication {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for AVAudioApplication {}
impl AVAudioApplication {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"AVAudioApplication").unwrap(), alloc) })
    }
}
impl INSObject for AVAudioApplication {}
impl PNSObject for AVAudioApplication {}
impl std::convert::TryFrom<NSObject> for AVAudioApplication {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<AVAudioApplication, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"AVAudioApplication").unwrap()) };
        if is_kind_of {
            Ok(AVAudioApplication(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to AVAudioApplication")
        }
    }
}
impl IAVAudioApplication for AVAudioApplication {}
pub trait IAVAudioApplication: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn setInputMuted_error_(&self, muted: BOOL, outError: *mut NSError) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInputMuted : muted, error : outError)
    }
    unsafe fn setInputMuteStateChangeHandler_error_(
        &self,
        inputMuteHandler: *mut ::std::os::raw::c_void,
        outError: *mut NSError,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInputMuteStateChangeHandler : inputMuteHandler, error : outError)
    }
    unsafe fn isInputMuted(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isInputMuted)
    }
    unsafe fn recordPermission(&self) -> AVAudioApplicationRecordPermission
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, recordPermission)
    }
    unsafe fn microphoneInjectionPermission(
        &self,
    ) -> AVAudioApplicationMicrophoneInjectionPermission
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, microphoneInjectionPermission)
    }
    unsafe fn requestRecordPermissionWithCompletionHandler_(response: *mut ::std::os::raw::c_void)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"AVAudioApplication").unwrap(), requestRecordPermissionWithCompletionHandler : response)
    }
    unsafe fn requestMicrophoneInjectionPermissionWithCompletionHandler_(
        response: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"AVAudioApplication").unwrap(), requestMicrophoneInjectionPermissionWithCompletionHandler : response)
    }
    unsafe fn sharedInstance() -> AVAudioApplication
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"AVAudioApplication").unwrap(), sharedInstance)
    }
}
unsafe extern "C" {
    pub static AVFormatIDKey: NSString;
}
unsafe extern "C" {
    pub static AVSampleRateKey: NSString;
}
unsafe extern "C" {
    pub static AVNumberOfChannelsKey: NSString;
}
unsafe extern "C" {
    pub static AVLinearPCMBitDepthKey: NSString;
}
unsafe extern "C" {
    pub static AVLinearPCMIsBigEndianKey: NSString;
}
unsafe extern "C" {
    pub static AVLinearPCMIsFloatKey: NSString;
}
unsafe extern "C" {
    pub static AVLinearPCMIsNonInterleaved: NSString;
}
unsafe extern "C" {
    pub static AVAudioFileTypeKey: NSString;
}
unsafe extern "C" {
    pub static AVEncoderAudioQualityKey: NSString;
}
unsafe extern "C" {
    pub static AVEncoderAudioQualityForVBRKey: NSString;
}
unsafe extern "C" {
    pub static AVEncoderBitRateKey: NSString;
}
unsafe extern "C" {
    pub static AVEncoderBitRatePerChannelKey: NSString;
}
unsafe extern "C" {
    pub static AVEncoderBitRateStrategyKey: NSString;
}
unsafe extern "C" {
    pub static AVEncoderBitDepthHintKey: NSString;
}
unsafe extern "C" {
    pub static AVEncoderDynamicRangeControlConfigurationKey: NSString;
}
unsafe extern "C" {
    pub static AVEncoderContentSourceKey: NSString;
}
unsafe extern "C" {
    pub static AVEncoderASPFrequencyKey: NSString;
}
unsafe extern "C" {
    pub static AVSampleRateConverterAlgorithmKey: NSString;
}
unsafe extern "C" {
    pub static AVSampleRateConverterAudioQualityKey: NSString;
}
unsafe extern "C" {
    pub static AVChannelLayoutKey: NSString;
}
unsafe extern "C" {
    pub static AVAudioBitRateStrategy_Constant: NSString;
}
unsafe extern "C" {
    pub static AVAudioBitRateStrategy_LongTermAverage: NSString;
}
unsafe extern "C" {
    pub static AVAudioBitRateStrategy_VariableConstrained: NSString;
}
unsafe extern "C" {
    pub static AVAudioBitRateStrategy_Variable: NSString;
}
unsafe extern "C" {
    pub static AVSampleRateConverterAlgorithm_Normal: NSString;
}
unsafe extern "C" {
    pub static AVSampleRateConverterAlgorithm_Mastering: NSString;
}
unsafe extern "C" {
    pub static AVSampleRateConverterAlgorithm_MinimumPhase: NSString;
}
unsafe extern "C" {
    pub static AVAudioEngineConfigurationChangeNotification: NSString;
}
unsafe extern "C" {
    pub static AVAudioSessionPortContinuityMicrophone: AVAudioSessionPort;
}
unsafe extern "C" {
    pub static AVAudioSessionPortLineIn: AVAudioSessionPort;
}
unsafe extern "C" {
    pub static AVAudioSessionPortBuiltInMic: AVAudioSessionPort;
}
unsafe extern "C" {
    pub static AVAudioSessionPortHeadsetMic: AVAudioSessionPort;
}
unsafe extern "C" {
    pub static AVAudioSessionPortLineOut: AVAudioSessionPort;
}
unsafe extern "C" {
    pub static AVAudioSessionPortHeadphones: AVAudioSessionPort;
}
unsafe extern "C" {
    pub static AVAudioSessionPortBluetoothA2DP: AVAudioSessionPort;
}
unsafe extern "C" {
    pub static AVAudioSessionPortBuiltInReceiver: AVAudioSessionPort;
}
unsafe extern "C" {
    pub static AVAudioSessionPortBuiltInSpeaker: AVAudioSessionPort;
}
unsafe extern "C" {
    pub static AVAudioSessionPortHDMI: AVAudioSessionPort;
}
unsafe extern "C" {
    pub static AVAudioSessionPortAirPlay: AVAudioSessionPort;
}
unsafe extern "C" {
    pub static AVAudioSessionPortBluetoothLE: AVAudioSessionPort;
}
unsafe extern "C" {
    pub static AVAudioSessionPortBluetoothHFP: AVAudioSessionPort;
}
unsafe extern "C" {
    pub static AVAudioSessionPortUSBAudio: AVAudioSessionPort;
}
unsafe extern "C" {
    pub static AVAudioSessionPortCarAudio: AVAudioSessionPort;
}
unsafe extern "C" {
    pub static AVAudioSessionPortVirtual: AVAudioSessionPort;
}
unsafe extern "C" {
    pub static AVAudioSessionPortPCI: AVAudioSessionPort;
}
unsafe extern "C" {
    pub static AVAudioSessionPortFireWire: AVAudioSessionPort;
}
unsafe extern "C" {
    pub static AVAudioSessionPortDisplayPort: AVAudioSessionPort;
}
unsafe extern "C" {
    pub static AVAudioSessionPortAVB: AVAudioSessionPort;
}
unsafe extern "C" {
    pub static AVAudioSessionPortThunderbolt: AVAudioSessionPort;
}
unsafe extern "C" {
    pub static AVAudioSessionCategoryAmbient: AVAudioSessionCategory;
}
unsafe extern "C" {
    pub static AVAudioSessionCategorySoloAmbient: AVAudioSessionCategory;
}
unsafe extern "C" {
    pub static AVAudioSessionCategoryPlayback: AVAudioSessionCategory;
}
unsafe extern "C" {
    pub static AVAudioSessionCategoryRecord: AVAudioSessionCategory;
}
unsafe extern "C" {
    pub static AVAudioSessionCategoryPlayAndRecord: AVAudioSessionCategory;
}
unsafe extern "C" {
    pub static AVAudioSessionCategoryAudioProcessing: AVAudioSessionCategory;
}
unsafe extern "C" {
    pub static AVAudioSessionCategoryMultiRoute: AVAudioSessionCategory;
}
unsafe extern "C" {
    pub static AVAudioSessionModeDefault: AVAudioSessionMode;
}
unsafe extern "C" {
    pub static AVAudioSessionModeVoiceChat: AVAudioSessionMode;
}
unsafe extern "C" {
    pub static AVAudioSessionModeGameChat: AVAudioSessionMode;
}
unsafe extern "C" {
    pub static AVAudioSessionModeVideoRecording: AVAudioSessionMode;
}
unsafe extern "C" {
    pub static AVAudioSessionModeMeasurement: AVAudioSessionMode;
}
unsafe extern "C" {
    pub static AVAudioSessionModeMoviePlayback: AVAudioSessionMode;
}
unsafe extern "C" {
    pub static AVAudioSessionModeVideoChat: AVAudioSessionMode;
}
unsafe extern "C" {
    pub static AVAudioSessionModeSpokenAudio: AVAudioSessionMode;
}
unsafe extern "C" {
    pub static AVAudioSessionModeVoicePrompt: AVAudioSessionMode;
}
unsafe extern "C" {
    pub static AVAudioSessionModeShortFormVideo: AVAudioSessionMode;
}
unsafe extern "C" {
    pub static AVAudioSessionModeDualRoute: AVAudioSessionMode;
}
unsafe extern "C" {
    pub static AVAudioSessionInterruptionNotification: NSNotificationName;
}
unsafe extern "C" {
    pub static AVAudioSessionRouteChangeNotification: NSNotificationName;
}
unsafe extern "C" {
    pub static AVAudioSessionMediaServicesWereLostNotification: NSNotificationName;
}
unsafe extern "C" {
    pub static AVAudioSessionMediaServicesWereResetNotification: NSNotificationName;
}
unsafe extern "C" {
    pub static AVAudioSessionSilenceSecondaryAudioHintNotification: NSNotificationName;
}
unsafe extern "C" {
    pub static AVAudioSessionSpatialPlaybackCapabilitiesChangedNotification: NSNotificationName;
}
unsafe extern "C" {
    pub static AVAudioSessionRenderingModeChangeNotification: NSNotificationName;
}
unsafe extern "C" {
    pub static AVAudioSessionRenderingCapabilitiesChangeNotification: NSNotificationName;
}
unsafe extern "C" {
    pub static AVAudioSessionMicrophoneInjectionCapabilitiesChangeNotification: NSNotificationName;
}
unsafe extern "C" {
    pub static AVAudioSessionOutputMuteStateChangeNotification: NSNotificationName;
}
unsafe extern "C" {
    pub static AVAudioSessionMuteStateKey: NSString;
}
unsafe extern "C" {
    pub static AVAudioSessionUserIntentToUnmuteOutputNotification: NSNotificationName;
}
unsafe extern "C" {
    pub static AVAudioSessionSpatialAudioEnabledKey: NSString;
}
unsafe extern "C" {
    pub static AVAudioSessionInterruptionTypeKey: NSString;
}
unsafe extern "C" {
    pub static AVAudioSessionInterruptionOptionKey: NSString;
}
unsafe extern "C" {
    pub static AVAudioSessionInterruptionReasonKey: NSString;
}
unsafe extern "C" {
    pub static AVAudioSessionInterruptionWasSuspendedKey: NSString;
}
unsafe extern "C" {
    pub static AVAudioSessionRouteChangeReasonKey: NSString;
}
unsafe extern "C" {
    pub static AVAudioSessionRouteChangePreviousRouteKey: NSString;
}
unsafe extern "C" {
    pub static AVAudioSessionSilenceSecondaryAudioHintTypeKey: NSString;
}
unsafe extern "C" {
    pub static AVAudioSessionRenderingModeNewRenderingModeKey: NSString;
}
unsafe extern "C" {
    pub static AVAudioSessionMicrophoneInjectionIsAvailableKey: NSString;
}
unsafe extern "C" {
    pub static AVAudioSessionAvailableInputsChangeNotification: NSNotificationName;
}
unsafe extern "C" {
    pub static AVAudioSessionLocationUpper: AVAudioSessionLocation;
}
unsafe extern "C" {
    pub static AVAudioSessionLocationLower: AVAudioSessionLocation;
}
unsafe extern "C" {
    pub static AVAudioSessionOrientationTop: AVAudioSessionOrientation;
}
unsafe extern "C" {
    pub static AVAudioSessionOrientationBottom: AVAudioSessionOrientation;
}
unsafe extern "C" {
    pub static AVAudioSessionOrientationFront: AVAudioSessionOrientation;
}
unsafe extern "C" {
    pub static AVAudioSessionOrientationBack: AVAudioSessionOrientation;
}
unsafe extern "C" {
    pub static AVAudioSessionOrientationLeft: AVAudioSessionOrientation;
}
unsafe extern "C" {
    pub static AVAudioSessionOrientationRight: AVAudioSessionOrientation;
}
unsafe extern "C" {
    pub static AVAudioSessionPolarPatternOmnidirectional: AVAudioSessionPolarPattern;
}
unsafe extern "C" {
    pub static AVAudioSessionPolarPatternCardioid: AVAudioSessionPolarPattern;
}
unsafe extern "C" {
    pub static AVAudioSessionPolarPatternSubcardioid: AVAudioSessionPolarPattern;
}
unsafe extern "C" {
    pub static AVAudioSessionPolarPatternStereo: AVAudioSessionPolarPattern;
}
unsafe extern "C" {
    pub static mut AVAudioSequencerInfoDictionaryKeyAlbum: AVAudioSequencerInfoDictionaryKey;
}
unsafe extern "C" {
    pub static mut AVAudioSequencerInfoDictionaryKeyApproximateDurationInSeconds:
        AVAudioSequencerInfoDictionaryKey;
}
unsafe extern "C" {
    pub static mut AVAudioSequencerInfoDictionaryKeyArtist: AVAudioSequencerInfoDictionaryKey;
}
unsafe extern "C" {
    pub static mut AVAudioSequencerInfoDictionaryKeyChannelLayout:
        AVAudioSequencerInfoDictionaryKey;
}
unsafe extern "C" {
    pub static mut AVAudioSequencerInfoDictionaryKeyComments: AVAudioSequencerInfoDictionaryKey;
}
unsafe extern "C" {
    pub static mut AVAudioSequencerInfoDictionaryKeyComposer: AVAudioSequencerInfoDictionaryKey;
}
unsafe extern "C" {
    pub static mut AVAudioSequencerInfoDictionaryKeyCopyright: AVAudioSequencerInfoDictionaryKey;
}
unsafe extern "C" {
    pub static mut AVAudioSequencerInfoDictionaryKeyEncodingApplication:
        AVAudioSequencerInfoDictionaryKey;
}
unsafe extern "C" {
    pub static mut AVAudioSequencerInfoDictionaryKeyGenre: AVAudioSequencerInfoDictionaryKey;
}
unsafe extern "C" {
    pub static mut AVAudioSequencerInfoDictionaryKeyISRC: AVAudioSequencerInfoDictionaryKey;
}
unsafe extern "C" {
    pub static mut AVAudioSequencerInfoDictionaryKeyKeySignature: AVAudioSequencerInfoDictionaryKey;
}
unsafe extern "C" {
    pub static mut AVAudioSequencerInfoDictionaryKeyLyricist: AVAudioSequencerInfoDictionaryKey;
}
unsafe extern "C" {
    pub static mut AVAudioSequencerInfoDictionaryKeyNominalBitRate:
        AVAudioSequencerInfoDictionaryKey;
}
unsafe extern "C" {
    pub static mut AVAudioSequencerInfoDictionaryKeyRecordedDate: AVAudioSequencerInfoDictionaryKey;
}
unsafe extern "C" {
    pub static mut AVAudioSequencerInfoDictionaryKeySourceBitDepth:
        AVAudioSequencerInfoDictionaryKey;
}
unsafe extern "C" {
    pub static mut AVAudioSequencerInfoDictionaryKeySourceEncoder:
        AVAudioSequencerInfoDictionaryKey;
}
unsafe extern "C" {
    pub static mut AVAudioSequencerInfoDictionaryKeySubTitle: AVAudioSequencerInfoDictionaryKey;
}
unsafe extern "C" {
    pub static mut AVAudioSequencerInfoDictionaryKeyTempo: AVAudioSequencerInfoDictionaryKey;
}
unsafe extern "C" {
    pub static mut AVAudioSequencerInfoDictionaryKeyTimeSignature:
        AVAudioSequencerInfoDictionaryKey;
}
unsafe extern "C" {
    pub static mut AVAudioSequencerInfoDictionaryKeyTitle: AVAudioSequencerInfoDictionaryKey;
}
unsafe extern "C" {
    pub static mut AVAudioSequencerInfoDictionaryKeyTrackNumber: AVAudioSequencerInfoDictionaryKey;
}
unsafe extern "C" {
    pub static mut AVAudioSequencerInfoDictionaryKeyYear: AVAudioSequencerInfoDictionaryKey;
}
unsafe extern "C" {
    pub static AVAudioUnitTypeOutput: NSString;
}
unsafe extern "C" {
    pub static AVAudioUnitTypeMusicDevice: NSString;
}
unsafe extern "C" {
    pub static AVAudioUnitTypeMusicEffect: NSString;
}
unsafe extern "C" {
    pub static AVAudioUnitTypeFormatConverter: NSString;
}
unsafe extern "C" {
    pub static AVAudioUnitTypeEffect: NSString;
}
unsafe extern "C" {
    pub static AVAudioUnitTypeMixer: NSString;
}
unsafe extern "C" {
    pub static AVAudioUnitTypePanner: NSString;
}
unsafe extern "C" {
    pub static AVAudioUnitTypeGenerator: NSString;
}
unsafe extern "C" {
    pub static AVAudioUnitTypeOfflineEffect: NSString;
}
unsafe extern "C" {
    pub static AVAudioUnitTypeMIDIProcessor: NSString;
}
unsafe extern "C" {
    pub static AVAudioUnitManufacturerNameApple: NSString;
}
unsafe extern "C" {
    pub static AVAudioUnitComponentTagsDidChangeNotification: NSString;
}
unsafe extern "C" {
    pub static AVAudioUnitComponentManagerRegistrationsChangedNotification: NSNotificationName;
}
unsafe extern "C" {
    pub static AVExtendedNoteOnEventDefaultInstrument: UInt32;
}
unsafe extern "C" {
    pub static AVSpeechUtteranceMinimumSpeechRate: f32;
}
unsafe extern "C" {
    pub static AVSpeechUtteranceMaximumSpeechRate: f32;
}
unsafe extern "C" {
    pub static AVSpeechUtteranceDefaultSpeechRate: f32;
}
unsafe extern "C" {
    pub static AVSpeechSynthesisVoiceIdentifierAlex: NSString;
}
unsafe extern "C" {
    pub static AVSpeechSynthesisIPANotationAttribute: NSString;
}
unsafe extern "C" {
    pub static mut AVSpeechSynthesisAvailableVoicesDidChangeNotification: NSNotificationName;
}
unsafe extern "C" {
    pub static AVAudioApplicationInputMuteStateChangeNotification: NSNotificationName;
}
unsafe extern "C" {
    pub static AVAudioApplicationMuteStateKey: NSString;
}

unsafe impl objc2::encode::RefEncode for AudioStreamPacketDependencyDescription {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for AudioStreamPacketDependencyDescription {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("AudioStreamPacketDependencyDescription", &[]);
}
unsafe impl objc2::encode::RefEncode for AVAudio3DPoint {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for AVAudio3DPoint {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("AVAudio3DPoint", &[]);
}
unsafe impl objc2::encode::RefEncode for AVAudio3DVectorOrientation {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for AVAudio3DVectorOrientation {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("AVAudio3DVectorOrientation", &[]);
}
unsafe impl objc2::encode::RefEncode for AVAudio3DAngularOrientation {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for AVAudio3DAngularOrientation {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("AVAudio3DAngularOrientation", &[]);
}
unsafe impl objc2::encode::RefEncode for AVAudioBuffer {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for AVAudioBuffer {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for AVAudioPCMBuffer {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for AVAudioPCMBuffer {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for AVAudioCompressedBuffer {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for AVAudioCompressedBuffer {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for AVAudioChannelLayout {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for AVAudioChannelLayout {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for AVAudioFormat {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for AVAudioFormat {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for AVAudioTime {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for AVAudioTime {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for AVAudioConnectionPoint {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for AVAudioConnectionPoint {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for AVAudioConverterPrimeInfo {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for AVAudioConverterPrimeInfo {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("AVAudioConverterPrimeInfo", &[]);
}
unsafe impl objc2::encode::RefEncode for AVAudioConverter {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for AVAudioConverter {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for AVAudioNode {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for AVAudioNode {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for AVAudioMixingDestination {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for AVAudioMixingDestination {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for AVAudioVoiceProcessingOtherAudioDuckingConfiguration {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for AVAudioVoiceProcessingOtherAudioDuckingConfiguration {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("AVAudioVoiceProcessingOtherAudioDuckingConfiguration", &[]);
}
unsafe impl objc2::encode::RefEncode for AVAudioIONode {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for AVAudioIONode {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for AVAudioInputNode {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for AVAudioInputNode {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for AVAudioOutputNode {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for AVAudioOutputNode {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for AVAudioEngine {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for AVAudioEngine {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for AVAudioUnit {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for AVAudioUnit {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for AVAudioUnitEffect {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for AVAudioUnitEffect {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for AVAudioUnitReverb {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for AVAudioUnitReverb {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for AVAudioUnitEQFilterParameters {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for AVAudioUnitEQFilterParameters {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for AVAudioUnitEQ {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for AVAudioUnitEQ {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for AVAudioEnvironmentDistanceAttenuationParameters {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for AVAudioEnvironmentDistanceAttenuationParameters {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for AVAudioEnvironmentReverbParameters {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for AVAudioEnvironmentReverbParameters {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for AVAudioEnvironmentNode {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for AVAudioEnvironmentNode {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for AVAudioFile {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for AVAudioFile {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for AVAudioMixerNode {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for AVAudioMixerNode {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for AVAudioSessionChannelDescription {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for AVAudioSessionChannelDescription {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for AVAudioSessionDataSourceDescription {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for AVAudioSessionDataSourceDescription {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for AVAudioSessionCapability {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for AVAudioSessionCapability {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for AVAudioSessionPortExtensionBluetoothMicrophone {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for AVAudioSessionPortExtensionBluetoothMicrophone {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for AVAudioSessionPortDescription {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for AVAudioSessionPortDescription {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for AVAudioSessionRouteDescription {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for AVAudioSessionRouteDescription {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for AVAudioSession {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for AVAudioSession {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for AVAudioPlayer {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for AVAudioPlayer {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for AVAudioPlayerNode {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for AVAudioPlayerNode {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for AVAudioRecorder {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for AVAudioRecorder {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for AVAudioRoutingArbiter {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for AVAudioRoutingArbiter {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for _AVBeatRange {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for _AVBeatRange {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("_AVBeatRange", &[]);
}
unsafe impl objc2::encode::RefEncode for AVAudioSequencer {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for AVAudioSequencer {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for AVMusicTrack {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for AVMusicTrack {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for AVAudioSinkNode {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for AVAudioSinkNode {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for AVAudioSourceNode {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for AVAudioSourceNode {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for AVAudioUnitComponent {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for AVAudioUnitComponent {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for AVAudioUnitComponentManager {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for AVAudioUnitComponentManager {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for AVAudioUnitDelay {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for AVAudioUnitDelay {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for AVAudioUnitDistortion {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for AVAudioUnitDistortion {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for AVAudioUnitGenerator {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for AVAudioUnitGenerator {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for AVAudioUnitMIDIInstrument {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for AVAudioUnitMIDIInstrument {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for AVAudioUnitSampler {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for AVAudioUnitSampler {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for AVAudioUnitTimeEffect {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for AVAudioUnitTimeEffect {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for AVAudioUnitTimePitch {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for AVAudioUnitTimePitch {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for AVAudioUnitVarispeed {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for AVAudioUnitVarispeed {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for AVMIDIPlayer {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for AVMIDIPlayer {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for AVMusicEvent {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for AVMusicEvent {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for AVMIDINoteEvent {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for AVMIDINoteEvent {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for AVMIDIChannelEvent {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for AVMIDIChannelEvent {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for AVMIDIControlChangeEvent {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for AVMIDIControlChangeEvent {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for AVMIDIPolyPressureEvent {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for AVMIDIPolyPressureEvent {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for AVMIDIProgramChangeEvent {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for AVMIDIProgramChangeEvent {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for AVMIDIChannelPressureEvent {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for AVMIDIChannelPressureEvent {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for AVMIDIPitchBendEvent {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for AVMIDIPitchBendEvent {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for AVMIDISysexEvent {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for AVMIDISysexEvent {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for AVMIDIMetaEvent {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for AVMIDIMetaEvent {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for AVMusicUserEvent {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for AVMusicUserEvent {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for AVExtendedNoteOnEvent {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for AVExtendedNoteOnEvent {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for AVParameterEvent {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for AVParameterEvent {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for AVAUPresetEvent {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for AVAUPresetEvent {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for AVExtendedTempoEvent {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for AVExtendedTempoEvent {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for AVSpeechSynthesisVoice {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for AVSpeechSynthesisVoice {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for AVSpeechUtterance {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for AVSpeechUtterance {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for AVSpeechSynthesizer {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for AVSpeechSynthesizer {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for AVSpeechSynthesisMarker {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for AVSpeechSynthesisMarker {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for AVSpeechSynthesisProviderVoice {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for AVSpeechSynthesisProviderVoice {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for AVSpeechSynthesisProviderRequest {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for AVSpeechSynthesisProviderRequest {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for AVSpeechSynthesisProviderAudioUnit {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for AVSpeechSynthesisProviderAudioUnit {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for AVAudioApplication {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for AVAudioApplication {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
