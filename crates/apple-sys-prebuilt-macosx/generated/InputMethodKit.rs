#[allow(unused_imports)]
use crate::objc::*;
#[allow(unused_imports)]
use crate::AppKit::*;
#[allow(unused_imports)]
use crate::Carbon::*;
#[allow(unused_imports)]
use crate::CoreFoundation::*;
#[allow(unused_imports)]
use crate::Foundation::*;

#[allow(unused_imports)]
use objc2::msg_send;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct IMKServer(pub id);
impl std::ops::Deref for IMKServer {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for IMKServer {}
impl IMKServer {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"IMKServer").unwrap(), alloc) })
    }
}
impl INSObject for IMKServer {}
impl PNSObject for IMKServer {}
impl std::convert::TryFrom<NSObject> for IMKServer {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<IMKServer, Self::Error> {
        let is_kind_of: bool = unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"IMKServer").unwrap()) };
        if is_kind_of {
            Ok(IMKServer(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to IMKServer")
        }
    }
}
impl IIMKServer for IMKServer {}
pub trait IIMKServer: Sized + std::ops::Deref {
    unsafe fn initWithName_bundleIdentifier_(
        &self,
        name: NSString,
        bundleIdentifier: NSString,
    ) -> id
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithName : name, bundleIdentifier : bundleIdentifier)
    }
    unsafe fn initWithName_controllerClass_delegateClass_(
        &self,
        name: NSString,
        controllerClassID: Class,
        delegateClassID: Class,
    ) -> id
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithName : name, controllerClass : controllerClassID, delegateClass : delegateClassID)
    }
    unsafe fn bundle(&self) -> NSBundle
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, bundle)
    }
    unsafe fn paletteWillTerminate(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, paletteWillTerminate)
    }
    unsafe fn lastKeyEventWasDeadKey(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, lastKeyEventWasDeadKey)
    }
}
pub trait NSObject_IMKServerInput: Sized + std::ops::Deref {
    unsafe fn inputText_key_modifiers_client_(
        &self,
        string: NSString,
        keyCode: NSInteger,
        flags: NSUInteger,
        sender: id,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, inputText : string, key : keyCode, modifiers : flags, client : sender)
    }
    unsafe fn inputText_client_(&self, string: NSString, sender: id) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, inputText : string, client : sender)
    }
    unsafe fn handleEvent_client_(&self, event: NSEvent, sender: id) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, handleEvent : event, client : sender)
    }
    unsafe fn didCommandBySelector_client_(&self, aSelector: objc2::runtime::Sel, sender: id) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, didCommandBySelector : aSelector, client : sender)
    }
    unsafe fn composedString_(&self, sender: id) -> id
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, composedString : sender)
    }
    unsafe fn originalString_(&self, sender: id) -> NSAttributedString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, originalString : sender)
    }
    unsafe fn commitComposition_(&self, sender: id)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, commitComposition : sender)
    }
    unsafe fn candidates_(&self, sender: id) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, candidates : sender)
    }
}
pub trait PIMKStateSetting: Sized + std::ops::Deref {
    unsafe fn activateServer_(&self, sender: id)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, activateServer : sender)
    }
    unsafe fn deactivateServer_(&self, sender: id)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, deactivateServer : sender)
    }
    unsafe fn valueForTag_client_(&self, tag: ::std::os::raw::c_long, sender: id) -> id
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, valueForTag : tag, client : sender)
    }
    unsafe fn setValue_forTag_client_(&self, value: id, tag: ::std::os::raw::c_long, sender: id)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setValue : value, forTag : tag, client : sender)
    }
    unsafe fn modes_(&self, sender: id) -> NSDictionary
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, modes : sender)
    }
    unsafe fn recognizedEvents_(&self, sender: id) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, recognizedEvents : sender)
    }
    unsafe fn showPreferences_(&self, sender: id)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, showPreferences : sender)
    }
}
pub trait PIMKMouseHandling: Sized + std::ops::Deref {
    unsafe fn mouseDownOnCharacterIndex_coordinate_withModifier_continueTracking_client_(
        &self,
        index: NSUInteger,
        point: NSPoint,
        flags: NSUInteger,
        keepTracking: *mut BOOL,
        sender: id,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, mouseDownOnCharacterIndex : index, coordinate : point, withModifier : flags, continueTracking : keepTracking, client : sender)
    }
    unsafe fn mouseUpOnCharacterIndex_coordinate_withModifier_client_(
        &self,
        index: NSUInteger,
        point: NSPoint,
        flags: NSUInteger,
        sender: id,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, mouseUpOnCharacterIndex : index, coordinate : point, withModifier : flags, client : sender)
    }
    unsafe fn mouseMovedOnCharacterIndex_coordinate_withModifier_client_(
        &self,
        index: NSUInteger,
        point: NSPoint,
        flags: NSUInteger,
        sender: id,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, mouseMovedOnCharacterIndex : index, coordinate : point, withModifier : flags, client : sender)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct IMKInputController(pub id);
impl std::ops::Deref for IMKInputController {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for IMKInputController {}
impl IMKInputController {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"IMKInputController").unwrap(), alloc) })
    }
}
impl PIMKStateSetting for IMKInputController {}
impl PIMKMouseHandling for IMKInputController {}
impl INSObject for IMKInputController {}
impl PNSObject for IMKInputController {}
impl std::convert::TryFrom<NSObject> for IMKInputController {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<IMKInputController, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"IMKInputController").unwrap()) };
        if is_kind_of {
            Ok(IMKInputController(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to IMKInputController")
        }
    }
}
impl IIMKInputController for IMKInputController {}
pub trait IIMKInputController: Sized + std::ops::Deref {
    unsafe fn initWithServer_delegate_client_(
        &self,
        server: IMKServer,
        delegate: id,
        inputClient: id,
    ) -> id
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithServer : server, delegate : delegate, client : inputClient)
    }
    unsafe fn updateComposition(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, updateComposition)
    }
    unsafe fn cancelComposition(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, cancelComposition)
    }
    unsafe fn compositionAttributesAtRange_(&self, range: NSRange) -> NSMutableDictionary
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, compositionAttributesAtRange : range)
    }
    unsafe fn selectionRange(&self) -> NSRange
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, selectionRange)
    }
    unsafe fn replacementRange(&self) -> NSRange
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, replacementRange)
    }
    unsafe fn markForStyle_atRange_(&self, style: NSInteger, range: NSRange) -> NSDictionary
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, markForStyle : style, atRange : range)
    }
    unsafe fn doCommandBySelector_commandDictionary_(
        &self,
        aSelector: objc2::runtime::Sel,
        infoDictionary: NSDictionary,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, doCommandBySelector : aSelector, commandDictionary : infoDictionary)
    }
    unsafe fn hidePalettes(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, hidePalettes)
    }
    unsafe fn menu(&self) -> NSMenu
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, menu)
    }
    unsafe fn delegate(&self) -> id
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, delegate)
    }
    unsafe fn setDelegate_(&self, newDelegate: id)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDelegate : newDelegate)
    }
    unsafe fn server(&self) -> IMKServer
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, server)
    }
    unsafe fn client(&self) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, client)
    }
    unsafe fn inputControllerWillClose(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inputControllerWillClose)
    }
    unsafe fn annotationSelected_forCandidate_(
        &self,
        annotationString: NSAttributedString,
        candidateString: NSAttributedString,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, annotationSelected : annotationString, forCandidate : candidateString)
    }
    unsafe fn candidateSelectionChanged_(&self, candidateString: NSAttributedString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, candidateSelectionChanged : candidateString)
    }
    unsafe fn candidateSelected_(&self, candidateString: NSAttributedString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, candidateSelected : candidateString)
    }
}
pub type IMKCandidatePanelType = NSUInteger;
pub type IMKStyleType = NSUInteger;
pub type IMKCandidatesLocationHint = NSUInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct IMKCandidates(pub id);
impl std::ops::Deref for IMKCandidates {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for IMKCandidates {}
impl IMKCandidates {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"IMKCandidates").unwrap(), alloc) })
    }
}
impl INSResponder for IMKCandidates {}
impl PNSCoding for IMKCandidates {}
impl std::convert::TryFrom<NSResponder> for IMKCandidates {
    type Error = &'static str;
    fn try_from(parent: NSResponder) -> Result<IMKCandidates, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"IMKCandidates").unwrap()) };
        if is_kind_of {
            Ok(IMKCandidates(parent.0))
        } else {
            Err("This NSResponder cannot be downcasted to IMKCandidates")
        }
    }
}
impl INSObject for IMKCandidates {}
impl PNSObject for IMKCandidates {}
impl IIMKCandidates for IMKCandidates {}
pub trait IIMKCandidates: Sized + std::ops::Deref {
    unsafe fn initWithServer_panelType_(
        &self,
        server: IMKServer,
        panelType: IMKCandidatePanelType,
    ) -> id
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithServer : server, panelType : panelType)
    }
    unsafe fn initWithServer_panelType_styleType_(
        &self,
        server: IMKServer,
        panelType: IMKCandidatePanelType,
        style: IMKStyleType,
    ) -> id
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithServer : server, panelType : panelType, styleType : style)
    }
    unsafe fn panelType(&self) -> IMKCandidatePanelType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, panelType)
    }
    unsafe fn setPanelType_(&self, panelType: IMKCandidatePanelType)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPanelType : panelType)
    }
    unsafe fn show_(&self, locationHint: IMKCandidatesLocationHint)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, show : locationHint)
    }
    unsafe fn hide(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, hide)
    }
    unsafe fn isVisible(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isVisible)
    }
    unsafe fn updateCandidates(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, updateCandidates)
    }
    unsafe fn showAnnotation_(&self, annotationString: NSAttributedString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, showAnnotation : annotationString)
    }
    unsafe fn showSublist_subListDelegate_(&self, candidates: NSArray, delegate: id)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, showSublist : candidates, subListDelegate : delegate)
    }
    unsafe fn candidateFrame(&self) -> NSRect
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, candidateFrame)
    }
    unsafe fn setSelectionKeys_(&self, keyCodes: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSelectionKeys : keyCodes)
    }
    unsafe fn selectionKeys(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, selectionKeys)
    }
    unsafe fn setSelectionKeysKeylayout_(&self, layout: TISInputSourceRef)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSelectionKeysKeylayout : layout)
    }
    unsafe fn selectionKeysKeylayout(&self) -> TISInputSourceRef
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, selectionKeysKeylayout)
    }
    unsafe fn setAttributes_(&self, attributes: NSDictionary)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAttributes : attributes)
    }
    unsafe fn attributes(&self) -> NSDictionary
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, attributes)
    }
    unsafe fn setDismissesAutomatically_(&self, flag: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDismissesAutomatically : flag)
    }
    unsafe fn dismissesAutomatically(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, dismissesAutomatically)
    }
    unsafe fn selectedCandidate(&self) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, selectedCandidate)
    }
    unsafe fn setCandidateFrameTopLeft_(&self, point: NSPoint)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCandidateFrameTopLeft : point)
    }
    unsafe fn showChild(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, showChild)
    }
    unsafe fn hideChild(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, hideChild)
    }
    unsafe fn attachChild_toCandidate_type_(
        &self,
        child: IMKCandidates,
        candidateIdentifier: NSInteger,
        theType: IMKStyleType,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, attachChild : child, toCandidate : candidateIdentifier, r#type : theType)
    }
    unsafe fn detachChild_(&self, candidateIdentifier: NSInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, detachChild : candidateIdentifier)
    }
    unsafe fn setCandidateData_(&self, candidatesArray: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCandidateData : candidatesArray)
    }
    unsafe fn selectCandidateWithIdentifier_(&self, candidateIdentifier: NSInteger) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, selectCandidateWithIdentifier : candidateIdentifier)
    }
    unsafe fn selectCandidate_(&self, candidateIdentifier: NSInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, selectCandidate : candidateIdentifier)
    }
    unsafe fn showCandidates(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, showCandidates)
    }
    unsafe fn candidateStringIdentifier_(&self, candidateString: id) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, candidateStringIdentifier : candidateString)
    }
    unsafe fn selectedCandidateString(&self) -> NSAttributedString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, selectedCandidateString)
    }
    unsafe fn candidateIdentifierAtLineNumber_(&self, lineNumber: NSInteger) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, candidateIdentifierAtLineNumber : lineNumber)
    }
    unsafe fn lineNumberForCandidateWithIdentifier_(
        &self,
        candidateIdentifier: NSInteger,
    ) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, lineNumberForCandidateWithIdentifier : candidateIdentifier)
    }
    unsafe fn clearSelection(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, clearSelection)
    }
}
unsafe extern "C" {
    pub static mut IMKModeDictionary: NSString;
}
unsafe extern "C" {
    pub static mut IMKControllerClass: NSString;
}
unsafe extern "C" {
    pub static mut IMKDelegateClass: NSString;
}
unsafe extern "C" {
    pub static mut kIMKCommandMenuItemName: NSString;
}
unsafe extern "C" {
    pub static mut kIMKCommandClientName: NSString;
}
unsafe extern "C" {
    pub static mut IMKCandidatesOpacityAttributeName: NSString;
}
unsafe extern "C" {
    pub static mut IMKCandidatesSendServerKeyEventFirst: NSString;
}

unsafe impl objc2::encode::RefEncode for IMKServer {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for IMKServer {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for IMKInputController {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for IMKInputController {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for IMKCandidates {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for IMKCandidates {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
