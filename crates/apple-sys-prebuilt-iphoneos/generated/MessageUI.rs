#[allow(unused_imports)]
use crate::objc::*;
#[allow(unused_imports)]
use crate::Foundation::*;
#[allow(unused_imports)]
use crate::Messages::*;

#[allow(unused_imports)]
use objc2::msg_send;
pub type MFMailComposeControllerDeferredAction = NSInteger;
pub type MFMailComposeResult = NSInteger;
pub type MFMailComposeErrorCode = NSInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MFMailComposeViewController(pub id);
impl std::ops::Deref for MFMailComposeViewController {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MFMailComposeViewController {}
impl MFMailComposeViewController {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MFMailComposeViewController").unwrap(), alloc) })
    }
}
impl PNSCoding for MFMailComposeViewController {}
impl std::convert::TryFrom<NSObject> for MFMailComposeViewController {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<MFMailComposeViewController, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MFMailComposeViewController").unwrap()) };
        if is_kind_of {
            Ok(MFMailComposeViewController(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to MFMailComposeViewController")
        }
    }
}
impl IMFMailComposeViewController for MFMailComposeViewController {}
pub trait IMFMailComposeViewController: Sized + std::ops::Deref {
    unsafe fn setSubject_(&self, subject: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSubject : subject)
    }
    unsafe fn setToRecipients_(&self, toRecipients: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setToRecipients : toRecipients)
    }
    unsafe fn setCcRecipients_(&self, ccRecipients: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCcRecipients : ccRecipients)
    }
    unsafe fn setBccRecipients_(&self, bccRecipients: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setBccRecipients : bccRecipients)
    }
    unsafe fn setMessageBody_isHTML_(&self, body: NSString, isHTML: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMessageBody : body, isHTML : isHTML)
    }
    unsafe fn addAttachmentData_mimeType_fileName_(
        &self,
        attachment: NSData,
        mimeType: NSString,
        filename: NSString,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, addAttachmentData : attachment, mimeType : mimeType, fileName : filename)
    }
    unsafe fn setPreferredSendingEmailAddress_(&self, emailAddress: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPreferredSendingEmailAddress : emailAddress)
    }
    unsafe fn insertCollaborationItemProvider_completionHandler_(
        &self,
        itemProvider: NSItemProvider,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, insertCollaborationItemProvider : itemProvider, completionHandler : completionHandler)
    }
    unsafe fn mailComposeDelegate(&self) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, mailComposeDelegate)
    }
    unsafe fn setMailComposeDelegate_(&self, mailComposeDelegate: *mut u64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMailComposeDelegate : mailComposeDelegate)
    }
    unsafe fn canSendMail() -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MFMailComposeViewController").unwrap(), canSendMail)
    }
}
pub trait PMFMailComposeViewControllerDelegate: Sized + std::ops::Deref {
    unsafe fn mailComposeController_didFinishWithResult_error_(
        &self,
        controller: MFMailComposeViewController,
        result: MFMailComposeResult,
        error: NSError,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, mailComposeController : controller, didFinishWithResult : result, error : error)
    }
}
pub type MessageComposeResult = NSInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MFMessageComposeViewController(pub id);
impl std::ops::Deref for MFMessageComposeViewController {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MFMessageComposeViewController {}
impl MFMessageComposeViewController {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MFMessageComposeViewController").unwrap(), alloc) })
    }
}
impl PNSCoding for MFMessageComposeViewController {}
impl std::convert::TryFrom<NSObject> for MFMessageComposeViewController {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<MFMessageComposeViewController, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MFMessageComposeViewController").unwrap())
        };
        if is_kind_of {
            Ok(MFMessageComposeViewController(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to MFMessageComposeViewController")
        }
    }
}
impl IMFMessageComposeViewController for MFMessageComposeViewController {}
pub trait IMFMessageComposeViewController: Sized + std::ops::Deref {
    unsafe fn disableUserAttachments(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, disableUserAttachments)
    }
    unsafe fn addAttachmentURL_withAlternateFilename_(
        &self,
        attachmentURL: NSURL,
        alternateFilename: NSString,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, addAttachmentURL : attachmentURL, withAlternateFilename : alternateFilename)
    }
    unsafe fn addAttachmentData_typeIdentifier_filename_(
        &self,
        attachmentData: NSData,
        uti: NSString,
        filename: NSString,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, addAttachmentData : attachmentData, typeIdentifier : uti, filename : filename)
    }
    unsafe fn insertCollaborationItemProvider_(&self, itemProvider: NSItemProvider) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, insertCollaborationItemProvider : itemProvider)
    }
    unsafe fn messageComposeDelegate(&self) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, messageComposeDelegate)
    }
    unsafe fn setMessageComposeDelegate_(&self, messageComposeDelegate: *mut u64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMessageComposeDelegate : messageComposeDelegate)
    }
    unsafe fn recipients(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, recipients)
    }
    unsafe fn setRecipients_(&self, recipients: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRecipients : recipients)
    }
    unsafe fn body(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, body)
    }
    unsafe fn setBody_(&self, body: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setBody : body)
    }
    unsafe fn subject(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, subject)
    }
    unsafe fn setSubject_(&self, subject: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSubject : subject)
    }
    unsafe fn attachments(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, attachments)
    }
    unsafe fn message(&self) -> MSMessage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, message)
    }
    unsafe fn setMessage_(&self, message: MSMessage)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMessage : message)
    }
    unsafe fn canSendText() -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MFMessageComposeViewController").unwrap(), canSendText)
    }
    unsafe fn canSendSubject() -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MFMessageComposeViewController").unwrap(), canSendSubject)
    }
    unsafe fn canSendAttachments() -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MFMessageComposeViewController").unwrap(), canSendAttachments)
    }
    unsafe fn isSupportedAttachmentUTI_(uti: NSString) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MFMessageComposeViewController").unwrap(), isSupportedAttachmentUTI : uti)
    }
}
pub trait PMFMessageComposeViewControllerDelegate: Sized + std::ops::Deref {
    unsafe fn messageComposeViewController_didFinishWithResult_(
        &self,
        controller: MFMessageComposeViewController,
        result: MessageComposeResult,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, messageComposeViewController : controller, didFinishWithResult : result)
    }
}
impl MFMessageComposeViewController_UPI for MFMessageComposeViewController {}
pub trait MFMessageComposeViewController_UPI: Sized + std::ops::Deref {
    unsafe fn setUPIVerificationCodeSendCompletion_(&self, completion: *mut ::std::os::raw::c_void)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setUPIVerificationCodeSendCompletion : completion)
    }
}
pub trait PUIAppearanceContainer: Sized + std::ops::Deref {}
impl PUIAppearanceContainer for MFMailComposeViewController {}
impl PUIAppearanceContainer for MFMessageComposeViewController {}
unsafe extern "C" {
    pub static MFMailComposeErrorDomain: NSErrorDomain;
}
unsafe extern "C" {
    pub static MFMessageComposeViewControllerAttachmentURL: NSString;
}
unsafe extern "C" {
    pub static MFMessageComposeViewControllerAttachmentAlternateFilename: NSString;
}
unsafe extern "C" {
    pub static MFMessageComposeViewControllerTextMessageAvailabilityDidChangeNotification: NSString;
}
unsafe extern "C" {
    pub static MFMessageComposeViewControllerTextMessageAvailabilityKey: NSString;
}

unsafe impl objc2::encode::RefEncode for MFMailComposeViewController {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MFMailComposeViewController {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MFMessageComposeViewController {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MFMessageComposeViewController {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
