#[allow(unused_imports)]
use crate::objc::*;
#[allow(unused_imports)]
use crate::CoreGraphics::*;
#[allow(unused_imports)]
use crate::Foundation::*;
#[allow(unused_imports)]
use crate::Security::*;
#[allow(unused_imports)]
use crate::UIKit::*;

#[allow(unused_imports)]
use objc2::msg_send;
pub type MSMessagesAppPresentationStyle = NSUInteger;
pub type MSMessagesAppPresentationContext = NSUInteger;
pub trait PMSMessagesAppTranscriptPresentation: Sized + std::ops::Deref {
    unsafe fn contentSizeThatFits_(&self, size: CGSize) -> CGSize
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, contentSizeThatFits : size)
    }
    unsafe fn invalidateMessageTintColor(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, invalidateMessageTintColor)
    }
    unsafe fn messageTintColor(&self) -> UIColor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, messageTintColor)
    }
    unsafe fn messageCornerRadius(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, messageCornerRadius)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MSMessagesAppViewController(pub id);
impl std::ops::Deref for MSMessagesAppViewController {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MSMessagesAppViewController {}
impl MSMessagesAppViewController {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MSMessagesAppViewController").unwrap(), alloc) })
    }
}
impl PMSMessagesAppTranscriptPresentation for MSMessagesAppViewController {}
impl IUIViewController for MSMessagesAppViewController {}
impl PNSCoding for MSMessagesAppViewController {}
impl PUIAppearanceContainer for MSMessagesAppViewController {}
impl PUITraitEnvironment for MSMessagesAppViewController {}
impl PUIContentContainer for MSMessagesAppViewController {}
impl PUIFocusEnvironment for MSMessagesAppViewController {}
impl std::convert::TryFrom<UIViewController> for MSMessagesAppViewController {
    type Error = &'static str;
    fn try_from(parent: UIViewController) -> Result<MSMessagesAppViewController, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MSMessagesAppViewController").unwrap()) };
        if is_kind_of {
            Ok(MSMessagesAppViewController(parent.0))
        } else {
            Err("This UIViewController cannot be downcasted to MSMessagesAppViewController")
        }
    }
}
impl IUIResponder for MSMessagesAppViewController {}
impl PUIResponderStandardEditActions for MSMessagesAppViewController {}
impl INSObject for MSMessagesAppViewController {}
impl PNSObject for MSMessagesAppViewController {}
impl IMSMessagesAppViewController for MSMessagesAppViewController {}
pub trait IMSMessagesAppViewController: Sized + std::ops::Deref {
    unsafe fn requestPresentationStyle_(&self, presentationStyle: MSMessagesAppPresentationStyle)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, requestPresentationStyle : presentationStyle)
    }
    unsafe fn willBecomeActiveWithConversation_(&self, conversation: MSConversation)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, willBecomeActiveWithConversation : conversation)
    }
    unsafe fn didBecomeActiveWithConversation_(&self, conversation: MSConversation)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, didBecomeActiveWithConversation : conversation)
    }
    unsafe fn willResignActiveWithConversation_(&self, conversation: MSConversation)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, willResignActiveWithConversation : conversation)
    }
    unsafe fn didResignActiveWithConversation_(&self, conversation: MSConversation)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, didResignActiveWithConversation : conversation)
    }
    unsafe fn activeConversation(&self) -> MSConversation
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, activeConversation)
    }
    unsafe fn presentationStyle(&self) -> MSMessagesAppPresentationStyle
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, presentationStyle)
    }
    unsafe fn presentationContext(&self) -> MSMessagesAppPresentationContext
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, presentationContext)
    }
}
impl MSMessagesAppViewController_CompactOrExpandedPresentation for MSMessagesAppViewController {}
pub trait MSMessagesAppViewController_CompactOrExpandedPresentation:
    Sized + std::ops::Deref
{
    unsafe fn dismiss(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, dismiss)
    }
    unsafe fn willSelectMessage_conversation_(
        &self,
        message: MSMessage,
        conversation: MSConversation,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, willSelectMessage : message, conversation : conversation)
    }
    unsafe fn didSelectMessage_conversation_(
        &self,
        message: MSMessage,
        conversation: MSConversation,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, didSelectMessage : message, conversation : conversation)
    }
    unsafe fn didReceiveMessage_conversation_(
        &self,
        message: MSMessage,
        conversation: MSConversation,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, didReceiveMessage : message, conversation : conversation)
    }
    unsafe fn didStartSendingMessage_conversation_(
        &self,
        message: MSMessage,
        conversation: MSConversation,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, didStartSendingMessage : message, conversation : conversation)
    }
    unsafe fn didCancelSendingMessage_conversation_(
        &self,
        message: MSMessage,
        conversation: MSConversation,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, didCancelSendingMessage : message, conversation : conversation)
    }
    unsafe fn willTransitionToPresentationStyle_(
        &self,
        presentationStyle: MSMessagesAppPresentationStyle,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, willTransitionToPresentationStyle : presentationStyle)
    }
    unsafe fn didTransitionToPresentationStyle_(
        &self,
        presentationStyle: MSMessagesAppPresentationStyle,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, didTransitionToPresentationStyle : presentationStyle)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MSConversation(pub id);
impl std::ops::Deref for MSConversation {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MSConversation {}
impl MSConversation {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MSConversation").unwrap(), alloc) })
    }
}
impl INSObject for MSConversation {}
impl PNSObject for MSConversation {}
impl std::convert::TryFrom<NSObject> for MSConversation {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<MSConversation, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MSConversation").unwrap()) };
        if is_kind_of {
            Ok(MSConversation(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to MSConversation")
        }
    }
}
impl IMSConversation for MSConversation {}
pub trait IMSConversation: Sized + std::ops::Deref {
    unsafe fn insertMessage_completionHandler_(
        &self,
        message: MSMessage,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, insertMessage : message, completionHandler : completionHandler)
    }
    unsafe fn insertSticker_completionHandler_(
        &self,
        sticker: MSSticker,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, insertSticker : sticker, completionHandler : completionHandler)
    }
    unsafe fn insertText_completionHandler_(
        &self,
        text: NSString,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, insertText : text, completionHandler : completionHandler)
    }
    unsafe fn insertAttachment_withAlternateFilename_completionHandler_(
        &self,
        URL: NSURL,
        filename: NSString,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, insertAttachment : URL, withAlternateFilename : filename, completionHandler : completionHandler)
    }
    unsafe fn sendMessage_completionHandler_(
        &self,
        message: MSMessage,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, sendMessage : message, completionHandler : completionHandler)
    }
    unsafe fn sendSticker_completionHandler_(
        &self,
        sticker: MSSticker,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, sendSticker : sticker, completionHandler : completionHandler)
    }
    unsafe fn sendText_completionHandler_(
        &self,
        text: NSString,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, sendText : text, completionHandler : completionHandler)
    }
    unsafe fn sendAttachment_withAlternateFilename_completionHandler_(
        &self,
        URL: NSURL,
        filename: NSString,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, sendAttachment : URL, withAlternateFilename : filename, completionHandler : completionHandler)
    }
    unsafe fn localParticipantIdentifier(&self) -> NSUUID
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, localParticipantIdentifier)
    }
    unsafe fn remoteParticipantIdentifiers(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, remoteParticipantIdentifiers)
    }
    unsafe fn selectedMessage(&self) -> MSMessage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, selectedMessage)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MSMessage(pub id);
impl std::ops::Deref for MSMessage {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MSMessage {}
impl MSMessage {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MSMessage").unwrap(), alloc) })
    }
}
impl PNSCopying for MSMessage {}
impl PNSSecureCoding for MSMessage {}
impl INSObject for MSMessage {}
impl PNSObject for MSMessage {}
impl std::convert::TryFrom<NSObject> for MSMessage {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<MSMessage, Self::Error> {
        let is_kind_of: bool = unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MSMessage").unwrap()) };
        if is_kind_of {
            Ok(MSMessage(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to MSMessage")
        }
    }
}
impl IMSMessage for MSMessage {}
pub trait IMSMessage: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn initWithSession_(&self, session: MSSession) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithSession : session)
    }
    unsafe fn session(&self) -> MSSession
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, session)
    }
    unsafe fn isPending(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isPending)
    }
    unsafe fn senderParticipantIdentifier(&self) -> NSUUID
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, senderParticipantIdentifier)
    }
    unsafe fn layout(&self) -> MSMessageLayout
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, layout)
    }
    unsafe fn setLayout_(&self, layout: MSMessageLayout)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setLayout : layout)
    }
    unsafe fn URL(&self) -> NSURL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, URL)
    }
    unsafe fn setURL_(&self, URL: NSURL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setURL : URL)
    }
    unsafe fn shouldExpire(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, shouldExpire)
    }
    unsafe fn setShouldExpire_(&self, shouldExpire: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setShouldExpire : shouldExpire)
    }
    unsafe fn accessibilityLabel(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, accessibilityLabel)
    }
    unsafe fn setAccessibilityLabel_(&self, accessibilityLabel: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAccessibilityLabel : accessibilityLabel)
    }
    unsafe fn summaryText(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, summaryText)
    }
    unsafe fn setSummaryText_(&self, summaryText: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSummaryText : summaryText)
    }
    unsafe fn error(&self) -> NSError
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, error)
    }
    unsafe fn setError_(&self, error: NSError)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setError : error)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MSMessageLayout(pub id);
impl std::ops::Deref for MSMessageLayout {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MSMessageLayout {}
impl MSMessageLayout {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MSMessageLayout").unwrap(), alloc) })
    }
}
impl PNSCopying for MSMessageLayout {}
impl INSObject for MSMessageLayout {}
impl PNSObject for MSMessageLayout {}
impl std::convert::TryFrom<NSObject> for MSMessageLayout {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<MSMessageLayout, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MSMessageLayout").unwrap()) };
        if is_kind_of {
            Ok(MSMessageLayout(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to MSMessageLayout")
        }
    }
}
impl IMSMessageLayout for MSMessageLayout {}
pub trait IMSMessageLayout: Sized + std::ops::Deref {}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MSMessageTemplateLayout(pub id);
impl std::ops::Deref for MSMessageTemplateLayout {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MSMessageTemplateLayout {}
impl MSMessageTemplateLayout {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MSMessageTemplateLayout").unwrap(), alloc) })
    }
}
impl IMSMessageLayout for MSMessageTemplateLayout {}
impl PNSCopying for MSMessageTemplateLayout {}
impl From<MSMessageTemplateLayout> for MSMessageLayout {
    fn from(child: MSMessageTemplateLayout) -> MSMessageLayout {
        MSMessageLayout(child.0)
    }
}
impl std::convert::TryFrom<MSMessageLayout> for MSMessageTemplateLayout {
    type Error = &'static str;
    fn try_from(parent: MSMessageLayout) -> Result<MSMessageTemplateLayout, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MSMessageTemplateLayout").unwrap()) };
        if is_kind_of {
            Ok(MSMessageTemplateLayout(parent.0))
        } else {
            Err("This MSMessageLayout cannot be downcasted to MSMessageTemplateLayout")
        }
    }
}
impl INSObject for MSMessageTemplateLayout {}
impl PNSObject for MSMessageTemplateLayout {}
impl IMSMessageTemplateLayout for MSMessageTemplateLayout {}
pub trait IMSMessageTemplateLayout: Sized + std::ops::Deref {
    unsafe fn caption(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, caption)
    }
    unsafe fn setCaption_(&self, caption: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCaption : caption)
    }
    unsafe fn subcaption(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, subcaption)
    }
    unsafe fn setSubcaption_(&self, subcaption: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSubcaption : subcaption)
    }
    unsafe fn trailingCaption(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, trailingCaption)
    }
    unsafe fn setTrailingCaption_(&self, trailingCaption: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setTrailingCaption : trailingCaption)
    }
    unsafe fn trailingSubcaption(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, trailingSubcaption)
    }
    unsafe fn setTrailingSubcaption_(&self, trailingSubcaption: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setTrailingSubcaption : trailingSubcaption)
    }
    unsafe fn image(&self) -> UIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, image)
    }
    unsafe fn setImage_(&self, image: UIImage)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setImage : image)
    }
    unsafe fn mediaFileURL(&self) -> NSURL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, mediaFileURL)
    }
    unsafe fn setMediaFileURL_(&self, mediaFileURL: NSURL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMediaFileURL : mediaFileURL)
    }
    unsafe fn imageTitle(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, imageTitle)
    }
    unsafe fn setImageTitle_(&self, imageTitle: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setImageTitle : imageTitle)
    }
    unsafe fn imageSubtitle(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, imageSubtitle)
    }
    unsafe fn setImageSubtitle_(&self, imageSubtitle: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setImageSubtitle : imageSubtitle)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MSMessageLiveLayout(pub id);
impl std::ops::Deref for MSMessageLiveLayout {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MSMessageLiveLayout {}
impl MSMessageLiveLayout {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MSMessageLiveLayout").unwrap(), alloc) })
    }
}
impl IMSMessageLayout for MSMessageLiveLayout {}
impl PNSCopying for MSMessageLiveLayout {}
impl std::convert::TryFrom<MSMessageLayout> for MSMessageLiveLayout {
    type Error = &'static str;
    fn try_from(parent: MSMessageLayout) -> Result<MSMessageLiveLayout, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MSMessageLiveLayout").unwrap()) };
        if is_kind_of {
            Ok(MSMessageLiveLayout(parent.0))
        } else {
            Err("This MSMessageLayout cannot be downcasted to MSMessageLiveLayout")
        }
    }
}
impl INSObject for MSMessageLiveLayout {}
impl PNSObject for MSMessageLiveLayout {}
impl IMSMessageLiveLayout for MSMessageLiveLayout {}
pub trait IMSMessageLiveLayout: Sized + std::ops::Deref {
    unsafe fn initWithAlternateLayout_(
        &self,
        alternateLayout: MSMessageTemplateLayout,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithAlternateLayout : alternateLayout)
    }
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn alternateLayout(&self) -> MSMessageTemplateLayout
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, alternateLayout)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MSSession(pub id);
impl std::ops::Deref for MSSession {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MSSession {}
impl MSSession {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MSSession").unwrap(), alloc) })
    }
}
impl PNSSecureCoding for MSSession {}
impl INSObject for MSSession {}
impl PNSObject for MSSession {}
impl std::convert::TryFrom<NSObject> for MSSession {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<MSSession, Self::Error> {
        let is_kind_of: bool = unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MSSession").unwrap()) };
        if is_kind_of {
            Ok(MSSession(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to MSSession")
        }
    }
}
impl IMSSession for MSSession {}
pub trait IMSSession: Sized + std::ops::Deref {}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MSSticker(pub id);
impl std::ops::Deref for MSSticker {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MSSticker {}
impl MSSticker {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MSSticker").unwrap(), alloc) })
    }
}
impl INSObject for MSSticker {}
impl PNSObject for MSSticker {}
impl std::convert::TryFrom<NSObject> for MSSticker {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<MSSticker, Self::Error> {
        let is_kind_of: bool = unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MSSticker").unwrap()) };
        if is_kind_of {
            Ok(MSSticker(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to MSSticker")
        }
    }
}
impl IMSSticker for MSSticker {}
pub trait IMSSticker: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn initWithContentsOfFileURL_localizedDescription_error_(
        &self,
        fileURL: NSURL,
        localizedDescription: NSString,
        error: *mut NSError,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithContentsOfFileURL : fileURL, localizedDescription : localizedDescription, error : error)
    }
    unsafe fn imageFileURL(&self) -> NSURL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, imageFileURL)
    }
    unsafe fn localizedDescription(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, localizedDescription)
    }
}
pub trait PMSStickerBrowserViewDataSource: Sized + std::ops::Deref {
    unsafe fn numberOfStickersInStickerBrowserView_(
        &self,
        stickerBrowserView: MSStickerBrowserView,
    ) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, numberOfStickersInStickerBrowserView : stickerBrowserView)
    }
    unsafe fn stickerBrowserView_stickerAtIndex_(
        &self,
        stickerBrowserView: MSStickerBrowserView,
        index: NSInteger,
    ) -> MSSticker
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, stickerBrowserView : stickerBrowserView, stickerAtIndex : index)
    }
}
pub type MSStickerSize = NSInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MSStickerBrowserView(pub id);
impl std::ops::Deref for MSStickerBrowserView {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MSStickerBrowserView {}
impl MSStickerBrowserView {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MSStickerBrowserView").unwrap(), alloc) })
    }
}
impl IUIView for MSStickerBrowserView {}
impl PNSCoding for MSStickerBrowserView {}
impl PUIAppearance for MSStickerBrowserView {}
impl PUIAppearanceContainer for MSStickerBrowserView {}
impl PUIDynamicItem for MSStickerBrowserView {}
impl PUITraitEnvironment for MSStickerBrowserView {}
impl PUICoordinateSpace for MSStickerBrowserView {}
impl PUIFocusItem for MSStickerBrowserView {}
impl PUIFocusItemContainer for MSStickerBrowserView {}
impl std::convert::TryFrom<UIView> for MSStickerBrowserView {
    type Error = &'static str;
    fn try_from(parent: UIView) -> Result<MSStickerBrowserView, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MSStickerBrowserView").unwrap()) };
        if is_kind_of {
            Ok(MSStickerBrowserView(parent.0))
        } else {
            Err("This UIView cannot be downcasted to MSStickerBrowserView")
        }
    }
}
impl IUIResponder for MSStickerBrowserView {}
impl PUIResponderStandardEditActions for MSStickerBrowserView {}
impl INSObject for MSStickerBrowserView {}
impl PNSObject for MSStickerBrowserView {}
impl IMSStickerBrowserView for MSStickerBrowserView {}
pub trait IMSStickerBrowserView: Sized + std::ops::Deref {
    unsafe fn initWithFrame_(&self, frame: CGRect) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithFrame : frame)
    }
    unsafe fn initWithFrame_stickerSize_(
        &self,
        frame: CGRect,
        stickerSize: MSStickerSize,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithFrame : frame, stickerSize : stickerSize)
    }
    unsafe fn setContentOffset_animated_(&self, contentOffset: CGPoint, animated: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setContentOffset : contentOffset, animated : animated)
    }
    unsafe fn reloadData(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, reloadData)
    }
    unsafe fn stickerSize(&self) -> MSStickerSize
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, stickerSize)
    }
    unsafe fn dataSource(&self) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, dataSource)
    }
    unsafe fn setDataSource_(&self, dataSource: *mut u64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDataSource : dataSource)
    }
    unsafe fn contentOffset(&self) -> CGPoint
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, contentOffset)
    }
    unsafe fn setContentOffset_(&self, contentOffset: CGPoint)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setContentOffset : contentOffset)
    }
    unsafe fn contentInset(&self) -> UIEdgeInsets
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, contentInset)
    }
    unsafe fn setContentInset_(&self, contentInset: UIEdgeInsets)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setContentInset : contentInset)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MSStickerBrowserViewController(pub id);
impl std::ops::Deref for MSStickerBrowserViewController {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MSStickerBrowserViewController {}
impl MSStickerBrowserViewController {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MSStickerBrowserViewController").unwrap(), alloc) })
    }
}
impl PMSStickerBrowserViewDataSource for MSStickerBrowserViewController {}
impl IUIViewController for MSStickerBrowserViewController {}
impl PNSCoding for MSStickerBrowserViewController {}
impl PUIAppearanceContainer for MSStickerBrowserViewController {}
impl PUITraitEnvironment for MSStickerBrowserViewController {}
impl PUIContentContainer for MSStickerBrowserViewController {}
impl PUIFocusEnvironment for MSStickerBrowserViewController {}
impl std::convert::TryFrom<UIViewController> for MSStickerBrowserViewController {
    type Error = &'static str;
    fn try_from(parent: UIViewController) -> Result<MSStickerBrowserViewController, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MSStickerBrowserViewController").unwrap())
        };
        if is_kind_of {
            Ok(MSStickerBrowserViewController(parent.0))
        } else {
            Err("This UIViewController cannot be downcasted to MSStickerBrowserViewController")
        }
    }
}
impl IUIResponder for MSStickerBrowserViewController {}
impl PUIResponderStandardEditActions for MSStickerBrowserViewController {}
impl INSObject for MSStickerBrowserViewController {}
impl PNSObject for MSStickerBrowserViewController {}
impl IMSStickerBrowserViewController for MSStickerBrowserViewController {}
pub trait IMSStickerBrowserViewController: Sized + std::ops::Deref {
    unsafe fn initWithStickerSize_(&self, stickerSize: MSStickerSize) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithStickerSize : stickerSize)
    }
    unsafe fn stickerBrowserView(&self) -> MSStickerBrowserView
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, stickerBrowserView)
    }
    unsafe fn stickerSize(&self) -> MSStickerSize
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, stickerSize)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MSStickerView(pub id);
impl std::ops::Deref for MSStickerView {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MSStickerView {}
impl MSStickerView {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MSStickerView").unwrap(), alloc) })
    }
}
impl IUIView for MSStickerView {}
impl PNSCoding for MSStickerView {}
impl PUIAppearance for MSStickerView {}
impl PUIAppearanceContainer for MSStickerView {}
impl PUIDynamicItem for MSStickerView {}
impl PUITraitEnvironment for MSStickerView {}
impl PUICoordinateSpace for MSStickerView {}
impl PUIFocusItem for MSStickerView {}
impl PUIFocusItemContainer for MSStickerView {}
impl std::convert::TryFrom<UIView> for MSStickerView {
    type Error = &'static str;
    fn try_from(parent: UIView) -> Result<MSStickerView, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MSStickerView").unwrap()) };
        if is_kind_of {
            Ok(MSStickerView(parent.0))
        } else {
            Err("This UIView cannot be downcasted to MSStickerView")
        }
    }
}
impl IUIResponder for MSStickerView {}
impl PUIResponderStandardEditActions for MSStickerView {}
impl INSObject for MSStickerView {}
impl PNSObject for MSStickerView {}
impl IMSStickerView for MSStickerView {}
pub trait IMSStickerView: Sized + std::ops::Deref {
    unsafe fn initWithFrame_sticker_(&self, frame: CGRect, sticker: MSSticker) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithFrame : frame, sticker : sticker)
    }
    unsafe fn startAnimating(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, startAnimating)
    }
    unsafe fn stopAnimating(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, stopAnimating)
    }
    unsafe fn isAnimating(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isAnimating)
    }
    unsafe fn sticker(&self) -> MSSticker
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, sticker)
    }
    unsafe fn setSticker_(&self, sticker: MSSticker)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSticker : sticker)
    }
    unsafe fn animationDuration(&self) -> NSTimeInterval
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, animationDuration)
    }
}
pub type MSMessageErrorCode = NSInteger;
unsafe extern "C" {
    pub static MSStickersErrorDomain: NSString;
}
unsafe extern "C" {
    pub static MSMessagesErrorDomain: NSString;
}

unsafe impl objc2::encode::RefEncode for MSMessagesAppViewController {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MSMessagesAppViewController {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MSConversation {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MSConversation {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MSMessage {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MSMessage {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MSMessageLayout {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MSMessageLayout {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MSMessageTemplateLayout {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MSMessageTemplateLayout {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MSMessageLiveLayout {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MSMessageLiveLayout {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MSSession {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MSSession {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MSSticker {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MSSticker {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MSStickerBrowserView {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MSStickerBrowserView {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MSStickerBrowserViewController {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MSStickerBrowserViewController {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MSStickerView {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MSStickerView {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
