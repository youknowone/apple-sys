#[allow(unused_imports)]
use crate::objc::*;
#[allow(unused_imports)]
use crate::Accounts::*;
#[allow(unused_imports)]
use crate::AddressBook::*;
#[allow(unused_imports)]
use crate::Foundation::*;

#[allow(unused_imports)]
use objc2::msg_send;
pub type SLRequestMethod = NSInteger;
pub type SLRequestHandler = *mut ::std::os::raw::c_void;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct SLRequest(pub id);
impl std::ops::Deref for SLRequest {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for SLRequest {}
impl SLRequest {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"SLRequest").unwrap(), alloc) })
    }
}
impl std::convert::TryFrom<NSObject> for SLRequest {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<SLRequest, Self::Error> {
        let is_kind_of: bool = unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"SLRequest").unwrap()) };
        if is_kind_of {
            Ok(SLRequest(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to SLRequest")
        }
    }
}
impl ISLRequest for SLRequest {}
pub trait ISLRequest: Sized + std::ops::Deref {
    unsafe fn addMultipartData_withName_type_filename_(
        &self,
        data: NSData,
        name: NSString,
        type_: NSString,
        filename: NSString,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, addMultipartData : data, withName : name, r#type : type_, filename : filename)
    }
    unsafe fn addMultipartData_withName_type_(&self, data: NSData, name: NSString, type_: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, addMultipartData : data, withName : name, r#type : type_)
    }
    unsafe fn preparedURLRequest(&self) -> NSURLRequest
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, preparedURLRequest)
    }
    unsafe fn performRequestWithHandler_(&self, handler: SLRequestHandler)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, performRequestWithHandler : handler)
    }
    unsafe fn account(&self) -> ACAccount
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, account)
    }
    unsafe fn setAccount_(&self, account: ACAccount)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAccount : account)
    }
    unsafe fn requestMethod(&self) -> SLRequestMethod
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, requestMethod)
    }
    unsafe fn URL(&self) -> NSURL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, URL)
    }
    unsafe fn parameters(&self) -> NSDictionary
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, parameters)
    }
    unsafe fn requestForServiceType_requestMethod_URL_parameters_(
        serviceType: NSString,
        requestMethod: SLRequestMethod,
        url: NSURL,
        parameters: NSDictionary,
    ) -> SLRequest
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SLRequest").unwrap(), requestForServiceType : serviceType, requestMethod : requestMethod, URL : url, parameters : parameters)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct SLComposeServiceViewController(pub id);
impl std::ops::Deref for SLComposeServiceViewController {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for SLComposeServiceViewController {}
impl SLComposeServiceViewController {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"SLComposeServiceViewController").unwrap(), alloc) })
    }
}
impl PNSTextViewDelegate for SLComposeServiceViewController {}
impl INSViewController for SLComposeServiceViewController {}
impl PNSEditor for SLComposeServiceViewController {}
impl PNSSeguePerforming for SLComposeServiceViewController {}
impl PNSUserInterfaceItemIdentification for SLComposeServiceViewController {}
impl std::convert::TryFrom<NSViewController> for SLComposeServiceViewController {
    type Error = &'static str;
    fn try_from(parent: NSViewController) -> Result<SLComposeServiceViewController, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"SLComposeServiceViewController").unwrap())
        };
        if is_kind_of {
            Ok(SLComposeServiceViewController(parent.0))
        } else {
            Err("This NSViewController cannot be downcasted to SLComposeServiceViewController")
        }
    }
}
impl INSResponder for SLComposeServiceViewController {}
impl PNSCoding for SLComposeServiceViewController {}
impl ISLComposeServiceViewController for SLComposeServiceViewController {}
pub trait ISLComposeServiceViewController: Sized + std::ops::Deref {
    unsafe fn presentationAnimationDidFinish(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, presentationAnimationDidFinish)
    }
    unsafe fn didSelectPost(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, didSelectPost)
    }
    unsafe fn didSelectCancel(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, didSelectCancel)
    }
    unsafe fn cancel(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, cancel)
    }
    unsafe fn isContentValid(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isContentValid)
    }
    unsafe fn validateContent(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, validateContent)
    }
    unsafe fn textView(&self) -> NSTextView
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, textView)
    }
    unsafe fn contentText(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, contentText)
    }
    unsafe fn placeholder(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, placeholder)
    }
    unsafe fn setPlaceholder_(&self, placeholder: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPlaceholder : placeholder)
    }
    unsafe fn charactersRemaining(&self) -> NSNumber
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, charactersRemaining)
    }
    unsafe fn setCharactersRemaining_(&self, charactersRemaining: NSNumber)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCharactersRemaining : charactersRemaining)
    }
}
unsafe extern "C" {
    pub static SLServiceTypeTwitter: NSString;
}
unsafe extern "C" {
    pub static SLServiceTypeFacebook: NSString;
}
unsafe extern "C" {
    pub static SLServiceTypeSinaWeibo: NSString;
}
unsafe extern "C" {
    pub static SLServiceTypeTencentWeibo: NSString;
}
unsafe extern "C" {
    pub static SLServiceTypeLinkedIn: NSString;
}

unsafe impl objc2::encode::RefEncode for SLRequest {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for SLRequest {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for SLComposeServiceViewController {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for SLComposeServiceViewController {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
