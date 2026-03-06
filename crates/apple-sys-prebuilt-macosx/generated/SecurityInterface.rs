#[allow(unused_imports)]
use crate::objc::*;
#[allow(unused_imports)]
use crate::AppKit::*;
#[allow(unused_imports)]
use crate::Foundation::*;
#[allow(unused_imports)]
use crate::Security::*;
#[allow(unused_imports)]
use crate::SecurityFoundation::*;

#[allow(unused_imports)]
use objc2::msg_send;
pub type SFButtonType = ::std::os::raw::c_uint;
pub type SFViewType = ::std::os::raw::c_uint;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct SFAuthorizationPluginView(pub id);
impl std::ops::Deref for SFAuthorizationPluginView {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for SFAuthorizationPluginView {}
impl SFAuthorizationPluginView {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"SFAuthorizationPluginView").unwrap(), alloc) })
    }
}
impl std::convert::TryFrom<NSObject> for SFAuthorizationPluginView {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<SFAuthorizationPluginView, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"SFAuthorizationPluginView").unwrap()) };
        if is_kind_of {
            Ok(SFAuthorizationPluginView(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to SFAuthorizationPluginView")
        }
    }
}
impl ISFAuthorizationPluginView for SFAuthorizationPluginView {}
pub trait ISFAuthorizationPluginView: Sized + std::ops::Deref {
    unsafe fn initWithCallbacks_andEngineRef_(
        &self,
        callbacks: *const AuthorizationCallbacks,
        engineRef: AuthorizationEngineRef,
    ) -> id
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithCallbacks : callbacks, andEngineRef : engineRef)
    }
    unsafe fn engineRef(&self) -> AuthorizationEngineRef
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, engineRef)
    }
    unsafe fn callbacks(&self) -> *const AuthorizationCallbacks
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, callbacks)
    }
    unsafe fn buttonPressed_(&self, inButtonType: SFButtonType)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, buttonPressed : inButtonType)
    }
    unsafe fn lastError(&self) -> NSError
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, lastError)
    }
    unsafe fn didActivate(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, didActivate)
    }
    unsafe fn willActivateWithUser_(&self, inUserInformation: NSDictionary)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, willActivateWithUser : inUserInformation)
    }
    unsafe fn didDeactivate(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, didDeactivate)
    }
    unsafe fn firstKeyView(&self) -> NSView
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, firstKeyView)
    }
    unsafe fn firstResponder(&self) -> NSResponder
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, firstResponder)
    }
    unsafe fn lastKeyView(&self) -> NSView
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, lastKeyView)
    }
    unsafe fn setEnabled_(&self, inEnabled: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setEnabled : inEnabled)
    }
    unsafe fn viewForType_(&self, inType: SFViewType) -> NSView
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, viewForType : inType)
    }
}
impl SFAuthorizationPluginView_SFHostControl for SFAuthorizationPluginView {}
pub trait SFAuthorizationPluginView_SFHostControl: Sized + std::ops::Deref {
    unsafe fn displayView(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, displayView)
    }
    unsafe fn setButton_enabled_(&self, inButtonType: SFButtonType, inEnabled: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setButton : inButtonType, enabled : inEnabled)
    }
    unsafe fn updateView(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, updateView)
    }
}
pub type SFAuthorizationViewState = ::std::os::raw::c_uint;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct SFAuthorizationView(pub id);
impl std::ops::Deref for SFAuthorizationView {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for SFAuthorizationView {}
impl SFAuthorizationView {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"SFAuthorizationView").unwrap(), alloc) })
    }
}
impl INSView for SFAuthorizationView {}
impl PNSAnimatablePropertyContainer for SFAuthorizationView {}
impl PNSUserInterfaceItemIdentification for SFAuthorizationView {}
impl PNSDraggingDestination for SFAuthorizationView {}
impl PNSAppearanceCustomization for SFAuthorizationView {}
impl PNSAccessibilityElement for SFAuthorizationView {}
impl PNSAccessibility for SFAuthorizationView {}
impl std::convert::TryFrom<NSView> for SFAuthorizationView {
    type Error = &'static str;
    fn try_from(parent: NSView) -> Result<SFAuthorizationView, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"SFAuthorizationView").unwrap()) };
        if is_kind_of {
            Ok(SFAuthorizationView(parent.0))
        } else {
            Err("This NSView cannot be downcasted to SFAuthorizationView")
        }
    }
}
impl INSResponder for SFAuthorizationView {}
impl PNSCoding for SFAuthorizationView {}
impl ISFAuthorizationView for SFAuthorizationView {}
pub trait ISFAuthorizationView: Sized + std::ops::Deref {
    unsafe fn setString_(&self, authorizationString: AuthorizationString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setString : authorizationString)
    }
    unsafe fn setAuthorizationRights_(&self, authorizationRights: *const AuthorizationRights)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAuthorizationRights : authorizationRights)
    }
    unsafe fn authorizationRights(&self) -> *mut AuthorizationRights
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, authorizationRights)
    }
    unsafe fn authorization(&self) -> SFAuthorization
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, authorization)
    }
    unsafe fn updateStatus_(&self, inSender: id) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, updateStatus : inSender)
    }
    unsafe fn setAutoupdate_(&self, autoupdate: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAutoupdate : autoupdate)
    }
    unsafe fn setAutoupdate_interval_(&self, autoupdate: BOOL, interval: NSTimeInterval)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAutoupdate : autoupdate, interval : interval)
    }
    unsafe fn authorizationState(&self) -> SFAuthorizationViewState
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, authorizationState)
    }
    unsafe fn setEnabled_(&self, enabled: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setEnabled : enabled)
    }
    unsafe fn isEnabled(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isEnabled)
    }
    unsafe fn setFlags_(&self, flags: AuthorizationFlags)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setFlags : flags)
    }
    unsafe fn setDelegate_(&self, delegate: id)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDelegate : delegate)
    }
    unsafe fn delegate(&self) -> id
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, delegate)
    }
    unsafe fn authorize_(&self, inSender: id) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, authorize : inSender)
    }
    unsafe fn deauthorize_(&self, inSender: id) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, deauthorize : inSender)
    }
}
pub trait NSObject_SFAuthorizationViewDelegate: Sized + std::ops::Deref {
    unsafe fn authorizationViewDidAuthorize_(&self, view: SFAuthorizationView)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, authorizationViewDidAuthorize : view)
    }
    unsafe fn authorizationViewDidDeauthorize_(&self, view: SFAuthorizationView)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, authorizationViewDidDeauthorize : view)
    }
    unsafe fn authorizationViewShouldDeauthorize_(&self, view: SFAuthorizationView) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, authorizationViewShouldDeauthorize : view)
    }
    unsafe fn authorizationViewCreatedAuthorization_(&self, view: SFAuthorizationView)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, authorizationViewCreatedAuthorization : view)
    }
    unsafe fn authorizationViewReleasedAuthorization_(&self, view: SFAuthorizationView)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, authorizationViewReleasedAuthorization : view)
    }
    unsafe fn authorizationViewDidHide_(&self, view: SFAuthorizationView)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, authorizationViewDidHide : view)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct SFCertificatePanel(pub id);
impl std::ops::Deref for SFCertificatePanel {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for SFCertificatePanel {}
impl SFCertificatePanel {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"SFCertificatePanel").unwrap(), alloc) })
    }
}
impl INSPanel for SFCertificatePanel {}
impl std::convert::TryFrom<NSPanel> for SFCertificatePanel {
    type Error = &'static str;
    fn try_from(parent: NSPanel) -> Result<SFCertificatePanel, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"SFCertificatePanel").unwrap()) };
        if is_kind_of {
            Ok(SFCertificatePanel(parent.0))
        } else {
            Err("This NSPanel cannot be downcasted to SFCertificatePanel")
        }
    }
}
impl INSWindow for SFCertificatePanel {}
impl PNSAnimatablePropertyContainer for SFCertificatePanel {}
impl PNSMenuItemValidation for SFCertificatePanel {}
impl PNSUserInterfaceValidations for SFCertificatePanel {}
impl PNSUserInterfaceItemIdentification for SFCertificatePanel {}
impl PNSAppearanceCustomization for SFCertificatePanel {}
impl PNSAccessibilityElement for SFCertificatePanel {}
impl PNSAccessibility for SFCertificatePanel {}
impl INSResponder for SFCertificatePanel {}
impl PNSCoding for SFCertificatePanel {}
impl ISFCertificatePanel for SFCertificatePanel {}
pub trait ISFCertificatePanel: Sized + std::ops::Deref {
    unsafe fn runModalForTrust_showGroup_(&self, trust: SecTrustRef, showGroup: BOOL) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, runModalForTrust : trust, showGroup : showGroup)
    }
    unsafe fn runModalForCertificates_showGroup_(
        &self,
        certificates: NSArray,
        showGroup: BOOL,
    ) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, runModalForCertificates : certificates, showGroup : showGroup)
    }
    unsafe fn beginSheetForWindow_modalDelegate_didEndSelector_contextInfo_trust_showGroup_(
        &self,
        docWindow: NSWindow,
        delegate: id,
        didEndSelector: objc2::runtime::Sel,
        contextInfo: *mut ::std::os::raw::c_void,
        trust: SecTrustRef,
        showGroup: BOOL,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, beginSheetForWindow : docWindow, modalDelegate : delegate, didEndSelector : didEndSelector, contextInfo : contextInfo, trust : trust, showGroup : showGroup)
    }
    unsafe fn beginSheetForWindow_modalDelegate_didEndSelector_contextInfo_certificates_showGroup_(
        &self,
        docWindow: NSWindow,
        delegate: id,
        didEndSelector: objc2::runtime::Sel,
        contextInfo: *mut ::std::os::raw::c_void,
        certificates: NSArray,
        showGroup: BOOL,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, beginSheetForWindow : docWindow, modalDelegate : delegate, didEndSelector : didEndSelector, contextInfo : contextInfo, certificates : certificates, showGroup : showGroup)
    }
    unsafe fn setPolicies_(&self, policies: id)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPolicies : policies)
    }
    unsafe fn policies(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, policies)
    }
    unsafe fn setDefaultButtonTitle_(&self, title: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDefaultButtonTitle : title)
    }
    unsafe fn setAlternateButtonTitle_(&self, title: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAlternateButtonTitle : title)
    }
    unsafe fn setShowsHelp_(&self, showsHelp: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setShowsHelp : showsHelp)
    }
    unsafe fn showsHelp(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, showsHelp)
    }
    unsafe fn setHelpAnchor_(&self, anchor: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setHelpAnchor : anchor)
    }
    unsafe fn helpAnchor(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, helpAnchor)
    }
    unsafe fn certificateView(&self) -> SFCertificateView
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, certificateView)
    }
    unsafe fn sharedCertificatePanel() -> SFCertificatePanel
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SFCertificatePanel").unwrap(), sharedCertificatePanel)
    }
}
pub trait NSObject_SFCertificatePanelDelegate: Sized + std::ops::Deref {
    unsafe fn certificatePanelShowHelp_(&self, sender: SFCertificatePanel) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, certificatePanelShowHelp : sender)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct SFCertificateTrustPanel(pub id);
impl std::ops::Deref for SFCertificateTrustPanel {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for SFCertificateTrustPanel {}
impl SFCertificateTrustPanel {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"SFCertificateTrustPanel").unwrap(), alloc) })
    }
}
impl ISFCertificatePanel for SFCertificateTrustPanel {}
impl From<SFCertificateTrustPanel> for SFCertificatePanel {
    fn from(child: SFCertificateTrustPanel) -> SFCertificatePanel {
        SFCertificatePanel(child.0)
    }
}
impl std::convert::TryFrom<SFCertificatePanel> for SFCertificateTrustPanel {
    type Error = &'static str;
    fn try_from(parent: SFCertificatePanel) -> Result<SFCertificateTrustPanel, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"SFCertificateTrustPanel").unwrap()) };
        if is_kind_of {
            Ok(SFCertificateTrustPanel(parent.0))
        } else {
            Err("This SFCertificatePanel cannot be downcasted to SFCertificateTrustPanel")
        }
    }
}
impl INSPanel for SFCertificateTrustPanel {}
impl INSWindow for SFCertificateTrustPanel {}
impl PNSAnimatablePropertyContainer for SFCertificateTrustPanel {}
impl PNSMenuItemValidation for SFCertificateTrustPanel {}
impl PNSUserInterfaceValidations for SFCertificateTrustPanel {}
impl PNSUserInterfaceItemIdentification for SFCertificateTrustPanel {}
impl PNSAppearanceCustomization for SFCertificateTrustPanel {}
impl PNSAccessibilityElement for SFCertificateTrustPanel {}
impl PNSAccessibility for SFCertificateTrustPanel {}
impl INSResponder for SFCertificateTrustPanel {}
impl PNSCoding for SFCertificateTrustPanel {}
impl ISFCertificateTrustPanel for SFCertificateTrustPanel {}
pub trait ISFCertificateTrustPanel: Sized + std::ops::Deref {
    unsafe fn runModalForTrust_message_(&self, trust: SecTrustRef, message: NSString) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, runModalForTrust : trust, message : message)
    }
    unsafe fn beginSheetForWindow_modalDelegate_didEndSelector_contextInfo_trust_message_(
        &self,
        docWindow: NSWindow,
        delegate: id,
        didEndSelector: objc2::runtime::Sel,
        contextInfo: *mut ::std::os::raw::c_void,
        trust: SecTrustRef,
        message: NSString,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, beginSheetForWindow : docWindow, modalDelegate : delegate, didEndSelector : didEndSelector, contextInfo : contextInfo, trust : trust, message : message)
    }
    unsafe fn setInformativeText_(&self, informativeText: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInformativeText : informativeText)
    }
    unsafe fn informativeText(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, informativeText)
    }
    unsafe fn sharedCertificateTrustPanel() -> SFCertificateTrustPanel
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SFCertificateTrustPanel").unwrap(), sharedCertificateTrustPanel)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct SFCertificateView(pub id);
impl std::ops::Deref for SFCertificateView {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for SFCertificateView {}
impl SFCertificateView {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"SFCertificateView").unwrap(), alloc) })
    }
}
impl INSVisualEffectView for SFCertificateView {}
impl std::convert::TryFrom<NSVisualEffectView> for SFCertificateView {
    type Error = &'static str;
    fn try_from(parent: NSVisualEffectView) -> Result<SFCertificateView, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"SFCertificateView").unwrap()) };
        if is_kind_of {
            Ok(SFCertificateView(parent.0))
        } else {
            Err("This NSVisualEffectView cannot be downcasted to SFCertificateView")
        }
    }
}
impl INSView for SFCertificateView {}
impl PNSAnimatablePropertyContainer for SFCertificateView {}
impl PNSUserInterfaceItemIdentification for SFCertificateView {}
impl PNSDraggingDestination for SFCertificateView {}
impl PNSAppearanceCustomization for SFCertificateView {}
impl PNSAccessibilityElement for SFCertificateView {}
impl PNSAccessibility for SFCertificateView {}
impl INSResponder for SFCertificateView {}
impl PNSCoding for SFCertificateView {}
impl ISFCertificateView for SFCertificateView {}
pub trait ISFCertificateView: Sized + std::ops::Deref {
    unsafe fn setCertificate_(&self, certificate: SecCertificateRef)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCertificate : certificate)
    }
    unsafe fn certificate(&self) -> SecCertificateRef
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, certificate)
    }
    unsafe fn setPolicies_(&self, policies: id)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPolicies : policies)
    }
    unsafe fn policies(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, policies)
    }
    unsafe fn setEditableTrust_(&self, editable: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setEditableTrust : editable)
    }
    unsafe fn isEditable(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isEditable)
    }
    unsafe fn setDisplayTrust_(&self, display: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDisplayTrust : display)
    }
    unsafe fn isTrustDisplayed(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isTrustDisplayed)
    }
    unsafe fn saveTrustSettings(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, saveTrustSettings)
    }
    unsafe fn setDisplayDetails_(&self, display: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDisplayDetails : display)
    }
    unsafe fn detailsDisplayed(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, detailsDisplayed)
    }
    unsafe fn setDetailsDisclosed_(&self, disclosed: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDetailsDisclosed : disclosed)
    }
    unsafe fn detailsDisclosed(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, detailsDisclosed)
    }
    unsafe fn setPoliciesDisclosed_(&self, disclosed: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPoliciesDisclosed : disclosed)
    }
    unsafe fn policiesDisclosed(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, policiesDisclosed)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct SFChooseIdentityPanel(pub id);
impl std::ops::Deref for SFChooseIdentityPanel {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for SFChooseIdentityPanel {}
impl SFChooseIdentityPanel {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"SFChooseIdentityPanel").unwrap(), alloc) })
    }
}
impl INSPanel for SFChooseIdentityPanel {}
impl std::convert::TryFrom<NSPanel> for SFChooseIdentityPanel {
    type Error = &'static str;
    fn try_from(parent: NSPanel) -> Result<SFChooseIdentityPanel, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"SFChooseIdentityPanel").unwrap()) };
        if is_kind_of {
            Ok(SFChooseIdentityPanel(parent.0))
        } else {
            Err("This NSPanel cannot be downcasted to SFChooseIdentityPanel")
        }
    }
}
impl INSWindow for SFChooseIdentityPanel {}
impl PNSAnimatablePropertyContainer for SFChooseIdentityPanel {}
impl PNSMenuItemValidation for SFChooseIdentityPanel {}
impl PNSUserInterfaceValidations for SFChooseIdentityPanel {}
impl PNSUserInterfaceItemIdentification for SFChooseIdentityPanel {}
impl PNSAppearanceCustomization for SFChooseIdentityPanel {}
impl PNSAccessibilityElement for SFChooseIdentityPanel {}
impl PNSAccessibility for SFChooseIdentityPanel {}
impl INSResponder for SFChooseIdentityPanel {}
impl PNSCoding for SFChooseIdentityPanel {}
impl ISFChooseIdentityPanel for SFChooseIdentityPanel {}
pub trait ISFChooseIdentityPanel: Sized + std::ops::Deref {
    unsafe fn runModalForIdentities_message_(
        &self,
        identities: NSArray,
        message: NSString,
    ) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, runModalForIdentities : identities, message : message)
    }
    unsafe fn beginSheetForWindow_modalDelegate_didEndSelector_contextInfo_identities_message_(
        &self,
        docWindow: NSWindow,
        delegate: id,
        didEndSelector: objc2::runtime::Sel,
        contextInfo: *mut ::std::os::raw::c_void,
        identities: NSArray,
        message: NSString,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, beginSheetForWindow : docWindow, modalDelegate : delegate, didEndSelector : didEndSelector, contextInfo : contextInfo, identities : identities, message : message)
    }
    unsafe fn identity(&self) -> SecIdentityRef
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, identity)
    }
    unsafe fn setPolicies_(&self, policies: id)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPolicies : policies)
    }
    unsafe fn policies(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, policies)
    }
    unsafe fn setDefaultButtonTitle_(&self, title: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDefaultButtonTitle : title)
    }
    unsafe fn setAlternateButtonTitle_(&self, title: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAlternateButtonTitle : title)
    }
    unsafe fn setShowsHelp_(&self, showsHelp: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setShowsHelp : showsHelp)
    }
    unsafe fn showsHelp(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, showsHelp)
    }
    unsafe fn setHelpAnchor_(&self, anchor: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setHelpAnchor : anchor)
    }
    unsafe fn helpAnchor(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, helpAnchor)
    }
    unsafe fn setInformativeText_(&self, informativeText: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInformativeText : informativeText)
    }
    unsafe fn informativeText(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, informativeText)
    }
    unsafe fn setDomain_(&self, domainString: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDomain : domainString)
    }
    unsafe fn domain(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, domain)
    }
    unsafe fn sharedChooseIdentityPanel() -> SFChooseIdentityPanel
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SFChooseIdentityPanel").unwrap(), sharedChooseIdentityPanel)
    }
}
pub trait NSObject_SFChooseIdentityPanelDelegate: Sized + std::ops::Deref {
    unsafe fn chooseIdentityPanelShowHelp_(&self, sender: SFChooseIdentityPanel) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, chooseIdentityPanelShowHelp : sender)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct SFChooseIdentityTableCellView(pub id);
impl std::ops::Deref for SFChooseIdentityTableCellView {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for SFChooseIdentityTableCellView {}
impl SFChooseIdentityTableCellView {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"SFChooseIdentityTableCellView").unwrap(), alloc) })
    }
}
impl INSTableCellView for SFChooseIdentityTableCellView {}
impl std::convert::TryFrom<NSTableCellView> for SFChooseIdentityTableCellView {
    type Error = &'static str;
    fn try_from(parent: NSTableCellView) -> Result<SFChooseIdentityTableCellView, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"SFChooseIdentityTableCellView").unwrap())
        };
        if is_kind_of {
            Ok(SFChooseIdentityTableCellView(parent.0))
        } else {
            Err("This NSTableCellView cannot be downcasted to SFChooseIdentityTableCellView")
        }
    }
}
impl INSView for SFChooseIdentityTableCellView {}
impl PNSAnimatablePropertyContainer for SFChooseIdentityTableCellView {}
impl PNSUserInterfaceItemIdentification for SFChooseIdentityTableCellView {}
impl PNSDraggingDestination for SFChooseIdentityTableCellView {}
impl PNSAppearanceCustomization for SFChooseIdentityTableCellView {}
impl PNSAccessibilityElement for SFChooseIdentityTableCellView {}
impl PNSAccessibility for SFChooseIdentityTableCellView {}
impl INSResponder for SFChooseIdentityTableCellView {}
impl PNSCoding for SFChooseIdentityTableCellView {}
impl ISFChooseIdentityTableCellView for SFChooseIdentityTableCellView {}
pub trait ISFChooseIdentityTableCellView: Sized + std::ops::Deref {
    unsafe fn issuerTextField(&self) -> NSTextField
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, issuerTextField)
    }
    unsafe fn setIssuerTextField_(&self, issuerTextField: NSTextField)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setIssuerTextField : issuerTextField)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct SFKeychainSavePanel(pub id);
impl std::ops::Deref for SFKeychainSavePanel {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for SFKeychainSavePanel {}
impl SFKeychainSavePanel {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"SFKeychainSavePanel").unwrap(), alloc) })
    }
}
impl INSSavePanel for SFKeychainSavePanel {}
impl std::convert::TryFrom<NSSavePanel> for SFKeychainSavePanel {
    type Error = &'static str;
    fn try_from(parent: NSSavePanel) -> Result<SFKeychainSavePanel, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"SFKeychainSavePanel").unwrap()) };
        if is_kind_of {
            Ok(SFKeychainSavePanel(parent.0))
        } else {
            Err("This NSSavePanel cannot be downcasted to SFKeychainSavePanel")
        }
    }
}
impl INSPanel for SFKeychainSavePanel {}
impl INSWindow for SFKeychainSavePanel {}
impl PNSAnimatablePropertyContainer for SFKeychainSavePanel {}
impl PNSMenuItemValidation for SFKeychainSavePanel {}
impl PNSUserInterfaceValidations for SFKeychainSavePanel {}
impl PNSUserInterfaceItemIdentification for SFKeychainSavePanel {}
impl PNSAppearanceCustomization for SFKeychainSavePanel {}
impl PNSAccessibilityElement for SFKeychainSavePanel {}
impl PNSAccessibility for SFKeychainSavePanel {}
impl INSResponder for SFKeychainSavePanel {}
impl PNSCoding for SFKeychainSavePanel {}
impl ISFKeychainSavePanel for SFKeychainSavePanel {}
pub trait ISFKeychainSavePanel: Sized + std::ops::Deref {
    unsafe fn runModalForDirectory_file_(&self, path: NSString, name: NSString) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, runModalForDirectory : path, file : name)
    }
    unsafe fn setPassword_(&self, password: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPassword : password)
    }
    unsafe fn keychain(&self) -> SecKeychainRef
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, keychain)
    }
    unsafe fn error(&self) -> NSError
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, error)
    }
    unsafe fn beginSheetForDirectory_file_modalForWindow_modalDelegate_didEndSelector_contextInfo_(
        &self,
        path: NSString,
        name: NSString,
        docWindow: NSWindow,
        delegate: id,
        didEndSelector: objc2::runtime::Sel,
        contextInfo: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, beginSheetForDirectory : path, file : name, modalForWindow : docWindow, modalDelegate : delegate, didEndSelector : didEndSelector, contextInfo : contextInfo)
    }
    unsafe fn sharedKeychainSavePanel() -> SFKeychainSavePanel
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SFKeychainSavePanel").unwrap(), sharedKeychainSavePanel)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct SFKeychainSettingsPanel(pub id);
impl std::ops::Deref for SFKeychainSettingsPanel {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for SFKeychainSettingsPanel {}
impl SFKeychainSettingsPanel {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"SFKeychainSettingsPanel").unwrap(), alloc) })
    }
}
impl INSPanel for SFKeychainSettingsPanel {}
impl std::convert::TryFrom<NSPanel> for SFKeychainSettingsPanel {
    type Error = &'static str;
    fn try_from(parent: NSPanel) -> Result<SFKeychainSettingsPanel, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"SFKeychainSettingsPanel").unwrap()) };
        if is_kind_of {
            Ok(SFKeychainSettingsPanel(parent.0))
        } else {
            Err("This NSPanel cannot be downcasted to SFKeychainSettingsPanel")
        }
    }
}
impl INSWindow for SFKeychainSettingsPanel {}
impl PNSAnimatablePropertyContainer for SFKeychainSettingsPanel {}
impl PNSMenuItemValidation for SFKeychainSettingsPanel {}
impl PNSUserInterfaceValidations for SFKeychainSettingsPanel {}
impl PNSUserInterfaceItemIdentification for SFKeychainSettingsPanel {}
impl PNSAppearanceCustomization for SFKeychainSettingsPanel {}
impl PNSAccessibilityElement for SFKeychainSettingsPanel {}
impl PNSAccessibility for SFKeychainSettingsPanel {}
impl INSResponder for SFKeychainSettingsPanel {}
impl PNSCoding for SFKeychainSettingsPanel {}
impl ISFKeychainSettingsPanel for SFKeychainSettingsPanel {}
pub trait ISFKeychainSettingsPanel: Sized + std::ops::Deref {
    unsafe fn runModalForSettings_keychain_(
        &self,
        settings: *mut SecKeychainSettings,
        keychain: SecKeychainRef,
    ) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, runModalForSettings : settings, keychain : keychain)
    }
    unsafe fn beginSheetForWindow_modalDelegate_didEndSelector_contextInfo_settings_keychain_(
        &self,
        docWindow: NSWindow,
        delegate: id,
        didEndSelector: objc2::runtime::Sel,
        contextInfo: *mut ::std::os::raw::c_void,
        settings: *mut SecKeychainSettings,
        keychain: SecKeychainRef,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, beginSheetForWindow : docWindow, modalDelegate : delegate, didEndSelector : didEndSelector, contextInfo : contextInfo, settings : settings, keychain : keychain)
    }
    unsafe fn sharedKeychainSettingsPanel() -> SFKeychainSettingsPanel
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SFKeychainSettingsPanel").unwrap(), sharedKeychainSettingsPanel)
    }
}
unsafe extern "C" {
    pub static SFAuthorizationPluginViewUserNameKey: NSString;
}
unsafe extern "C" {
    pub static SFAuthorizationPluginViewUserShortNameKey: NSString;
}
unsafe extern "C" {
    pub static SFDisplayViewException: NSString;
}
unsafe extern "C" {
    pub static mut SFCertificateViewDisclosureStateDidChange: NSString;
}

unsafe impl objc2::encode::RefEncode for SFAuthorizationPluginView {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for SFAuthorizationPluginView {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for SFAuthorizationView {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for SFAuthorizationView {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for SFCertificatePanel {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for SFCertificatePanel {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for SFCertificateTrustPanel {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for SFCertificateTrustPanel {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for SFCertificateView {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for SFCertificateView {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for SFChooseIdentityPanel {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for SFChooseIdentityPanel {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for SFChooseIdentityTableCellView {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for SFChooseIdentityTableCellView {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for SFKeychainSavePanel {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for SFKeychainSavePanel {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for SFKeychainSettingsPanel {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for SFKeychainSettingsPanel {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
