#[allow(unused_imports)]
use crate::objc::*;
#[allow(unused_imports)]
use crate::Accounts::*;
#[allow(unused_imports)]
use crate::Foundation::*;
#[allow(unused_imports)]
use crate::Security::*;
#[allow(unused_imports)]
use crate::Social::*;
#[allow(unused_imports)]
use crate::UIKit::*;

#[allow(unused_imports)]
use objc2::msg_send;
pub type SLComposeViewControllerResult = NSInteger;
pub type SLComposeViewControllerCompletionHandler = *mut ::std::os::raw::c_void;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct SLComposeViewController(pub id);
impl std::ops::Deref for SLComposeViewController {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for SLComposeViewController {}
impl SLComposeViewController {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"SLComposeViewController").unwrap(), alloc) })
    }
}
impl IUIViewController for SLComposeViewController {}
impl PNSCoding for SLComposeViewController {}
impl PUIAppearanceContainer for SLComposeViewController {}
impl PUITraitEnvironment for SLComposeViewController {}
impl PUIContentContainer for SLComposeViewController {}
impl PUIFocusEnvironment for SLComposeViewController {}
impl std::convert::TryFrom<UIViewController> for SLComposeViewController {
    type Error = &'static str;
    fn try_from(parent: UIViewController) -> Result<SLComposeViewController, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"SLComposeViewController").unwrap()) };
        if is_kind_of {
            Ok(SLComposeViewController(parent.0))
        } else {
            Err("This UIViewController cannot be downcasted to SLComposeViewController")
        }
    }
}
impl IUIResponder for SLComposeViewController {}
impl PUIResponderStandardEditActions for SLComposeViewController {}
impl INSObject for SLComposeViewController {}
impl PNSObject for SLComposeViewController {}
impl ISLComposeViewController for SLComposeViewController {}
pub trait ISLComposeViewController: Sized + std::ops::Deref {
    unsafe fn setInitialText_(&self, text: NSString) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInitialText : text)
    }
    unsafe fn addImage_(&self, image: UIImage) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, addImage : image)
    }
    unsafe fn removeAllImages(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, removeAllImages)
    }
    unsafe fn addURL_(&self, url: NSURL) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, addURL : url)
    }
    unsafe fn removeAllURLs(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, removeAllURLs)
    }
    unsafe fn serviceType(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, serviceType)
    }
    unsafe fn completionHandler(&self) -> SLComposeViewControllerCompletionHandler
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, completionHandler)
    }
    unsafe fn setCompletionHandler_(
        &self,
        completionHandler: SLComposeViewControllerCompletionHandler,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCompletionHandler : completionHandler)
    }
    unsafe fn isAvailableForServiceType_(serviceType: NSString) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SLComposeViewController").unwrap(), isAvailableForServiceType : serviceType)
    }
    unsafe fn composeViewControllerForServiceType_(serviceType: NSString) -> SLComposeViewController
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SLComposeViewController").unwrap(), composeViewControllerForServiceType : serviceType)
    }
}
pub type SLComposeSheetConfigurationItemTapHandler = *mut ::std::os::raw::c_void;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct SLComposeSheetConfigurationItem(pub id);
impl std::ops::Deref for SLComposeSheetConfigurationItem {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for SLComposeSheetConfigurationItem {}
impl SLComposeSheetConfigurationItem {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"SLComposeSheetConfigurationItem").unwrap(), alloc) })
    }
}
impl INSObject for SLComposeSheetConfigurationItem {}
impl PNSObject for SLComposeSheetConfigurationItem {}
impl std::convert::TryFrom<NSObject> for SLComposeSheetConfigurationItem {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<SLComposeSheetConfigurationItem, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"SLComposeSheetConfigurationItem").unwrap())
        };
        if is_kind_of {
            Ok(SLComposeSheetConfigurationItem(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to SLComposeSheetConfigurationItem")
        }
    }
}
impl ISLComposeSheetConfigurationItem for SLComposeSheetConfigurationItem {}
pub trait ISLComposeSheetConfigurationItem: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn title(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, title)
    }
    unsafe fn setTitle_(&self, title: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setTitle : title)
    }
    unsafe fn value(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, value)
    }
    unsafe fn setValue_(&self, value: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setValue : value)
    }
    unsafe fn valuePending(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, valuePending)
    }
    unsafe fn setValuePending_(&self, valuePending: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setValuePending : valuePending)
    }
    unsafe fn tapHandler(&self) -> SLComposeSheetConfigurationItemTapHandler
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, tapHandler)
    }
    unsafe fn setTapHandler_(&self, tapHandler: SLComposeSheetConfigurationItemTapHandler)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setTapHandler : tapHandler)
    }
}
pub type TWRequestHandler = SLRequestHandler;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct TWRequest(pub id);
impl std::ops::Deref for TWRequest {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for TWRequest {}
impl TWRequest {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"TWRequest").unwrap(), alloc) })
    }
}
impl INSObject for TWRequest {}
impl PNSObject for TWRequest {}
impl std::convert::TryFrom<NSObject> for TWRequest {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<TWRequest, Self::Error> {
        let is_kind_of: bool = unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"TWRequest").unwrap()) };
        if is_kind_of {
            Ok(TWRequest(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to TWRequest")
        }
    }
}
impl ITWRequest for TWRequest {}
pub trait ITWRequest: Sized + std::ops::Deref {
    unsafe fn initWithURL_parameters_requestMethod_(
        &self,
        url: NSURL,
        parameters: NSDictionary,
        requestMethod: TWRequestMethod,
    ) -> id
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithURL : url, parameters : parameters, requestMethod : requestMethod)
    }
    unsafe fn addMultiPartData_withName_type_(&self, data: NSData, name: NSString, type_: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, addMultiPartData : data, withName : name, r#type : type_)
    }
    unsafe fn signedURLRequest(&self) -> NSURLRequest
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, signedURLRequest)
    }
    unsafe fn performRequestWithHandler_(&self, handler: TWRequestHandler)
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
    unsafe fn requestMethod(&self) -> TWRequestMethod
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
}
pub use self::SLComposeViewControllerResult as TWTweetComposeViewControllerResult;
pub type TWTweetComposeViewControllerCompletionHandler = SLComposeViewControllerCompletionHandler;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct TWTweetComposeViewController(pub id);
impl std::ops::Deref for TWTweetComposeViewController {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for TWTweetComposeViewController {}
impl TWTweetComposeViewController {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"TWTweetComposeViewController").unwrap(), alloc) })
    }
}
impl IUIViewController for TWTweetComposeViewController {}
impl PNSCoding for TWTweetComposeViewController {}
impl PUIAppearanceContainer for TWTweetComposeViewController {}
impl PUITraitEnvironment for TWTweetComposeViewController {}
impl PUIContentContainer for TWTweetComposeViewController {}
impl PUIFocusEnvironment for TWTweetComposeViewController {}
impl std::convert::TryFrom<UIViewController> for TWTweetComposeViewController {
    type Error = &'static str;
    fn try_from(parent: UIViewController) -> Result<TWTweetComposeViewController, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"TWTweetComposeViewController").unwrap()) };
        if is_kind_of {
            Ok(TWTweetComposeViewController(parent.0))
        } else {
            Err("This UIViewController cannot be downcasted to TWTweetComposeViewController")
        }
    }
}
impl IUIResponder for TWTweetComposeViewController {}
impl PUIResponderStandardEditActions for TWTweetComposeViewController {}
impl INSObject for TWTweetComposeViewController {}
impl PNSObject for TWTweetComposeViewController {}
impl ITWTweetComposeViewController for TWTweetComposeViewController {}
pub trait ITWTweetComposeViewController: Sized + std::ops::Deref {
    unsafe fn setInitialText_(&self, text: NSString) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInitialText : text)
    }
    unsafe fn addImage_(&self, image: UIImage) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, addImage : image)
    }
    unsafe fn removeAllImages(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, removeAllImages)
    }
    unsafe fn addURL_(&self, url: NSURL) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, addURL : url)
    }
    unsafe fn removeAllURLs(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, removeAllURLs)
    }
    unsafe fn completionHandler(&self) -> TWTweetComposeViewControllerCompletionHandler
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, completionHandler)
    }
    unsafe fn setCompletionHandler_(
        &self,
        completionHandler: TWTweetComposeViewControllerCompletionHandler,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCompletionHandler : completionHandler)
    }
    unsafe fn canSendTweet() -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"TWTweetComposeViewController").unwrap(), canSendTweet)
    }
}

unsafe impl objc2::encode::RefEncode for SLComposeViewController {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for SLComposeViewController {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for SLComposeSheetConfigurationItem {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for SLComposeSheetConfigurationItem {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for TWRequest {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for TWRequest {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for TWTweetComposeViewController {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for TWTweetComposeViewController {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
