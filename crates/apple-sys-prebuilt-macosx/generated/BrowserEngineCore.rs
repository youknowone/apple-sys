#[allow(unused_imports)]
use crate::objc::*;
#[allow(unused_imports)]
use crate::AVFAudio::*;
#[allow(unused_imports)]
use crate::Foundation::*;
#[allow(unused_imports)]
use crate::IOKit::*;

#[allow(unused_imports)]
use objc2::msg_send;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct BEAudioSession(pub id);
impl std::ops::Deref for BEAudioSession {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for BEAudioSession {}
impl BEAudioSession {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"BEAudioSession").unwrap(), alloc) })
    }
}
impl INSObject for BEAudioSession {}
impl PNSObject for BEAudioSession {}
impl std::convert::TryFrom<NSObject> for BEAudioSession {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<BEAudioSession, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"BEAudioSession").unwrap()) };
        if is_kind_of {
            Ok(BEAudioSession(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to BEAudioSession")
        }
    }
}
impl IBEAudioSession for BEAudioSession {}
pub trait IBEAudioSession: Sized + std::ops::Deref {
    unsafe fn initWithAudioSession_(&self, audioSession: AVAudioSession) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithAudioSession : audioSession)
    }
    unsafe fn setPreferredOutput_error_(
        &self,
        outPort: AVAudioSessionPortDescription,
        outError: *mut NSError,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPreferredOutput : outPort, error : outError)
    }
    unsafe fn availableOutputs(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, availableOutputs)
    }
    unsafe fn preferredOutput(&self) -> AVAudioSessionPortDescription
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, preferredOutput)
    }
}
unsafe extern "C" {
    pub fn be_memory_inline_jit_restrict_with_witness_supported() -> ::std::os::raw::c_int;
}

unsafe impl objc2::encode::RefEncode for BEAudioSession {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for BEAudioSession {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
