#[allow(unused_imports)]
use crate::objc::*;
#[allow(unused_imports)]
use crate::Foundation::*;

#[allow(unused_imports)]
use objc2::msg_send;
pub type BEEligibilityContext = NSInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct BEAvailability(pub id);
impl std::ops::Deref for BEAvailability {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for BEAvailability {}
impl BEAvailability {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"BEAvailability").unwrap(), alloc) })
    }
}
impl std::convert::TryFrom<NSObject> for BEAvailability {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<BEAvailability, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"BEAvailability").unwrap()) };
        if is_kind_of {
            Ok(BEAvailability(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to BEAvailability")
        }
    }
}
impl IBEAvailability for BEAvailability {}
pub trait IBEAvailability: Sized + std::ops::Deref {
    unsafe fn isEligibleForContext_completionHandler_(
        context: BEEligibilityContext,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"BEAvailability").unwrap(), isEligibleForContext : context, completionHandler : completionHandler)
    }
}

unsafe impl objc2::encode::RefEncode for BEAvailability {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for BEAvailability {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
