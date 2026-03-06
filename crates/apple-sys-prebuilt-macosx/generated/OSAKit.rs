#[allow(unused_imports)]
use crate::objc::*;
#[allow(unused_imports)]
use crate::AppKit::*;
#[allow(unused_imports)]
use crate::CoreFoundation::*;
#[allow(unused_imports)]
use crate::CoreServices::*;
#[allow(unused_imports)]
use crate::Foundation::*;

#[allow(unused_imports)]
use objc2::msg_send;
pub type OSALanguageFeatures = NSUInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct OSALanguage(pub id);
impl std::ops::Deref for OSALanguage {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for OSALanguage {}
impl OSALanguage {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"OSALanguage").unwrap(), alloc) })
    }
}
impl INSObject for OSALanguage {}
impl PNSObject for OSALanguage {}
impl std::convert::TryFrom<NSObject> for OSALanguage {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<OSALanguage, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"OSALanguage").unwrap()) };
        if is_kind_of {
            Ok(OSALanguage(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to OSALanguage")
        }
    }
}
impl IOSALanguage for OSALanguage {}
pub trait IOSALanguage: Sized + std::ops::Deref {
    unsafe fn initWithComponent_(&self, component: Component) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithComponent : component)
    }
    unsafe fn sharedLanguageInstance(&self) -> OSALanguageInstance
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, sharedLanguageInstance)
    }
    unsafe fn componentInstance(&self) -> ComponentInstance
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, componentInstance)
    }
    unsafe fn name(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, name)
    }
    unsafe fn info(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, info)
    }
    unsafe fn version(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, version)
    }
    unsafe fn type_(&self) -> OSType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, type)
    }
    unsafe fn subType(&self) -> OSType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, subType)
    }
    unsafe fn manufacturer(&self) -> OSType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, manufacturer)
    }
    unsafe fn features(&self) -> OSALanguageFeatures
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, features)
    }
    unsafe fn isThreadSafe(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isThreadSafe)
    }
    unsafe fn availableLanguages() -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"OSALanguage").unwrap(), availableLanguages)
    }
    unsafe fn languageForName_(name: NSString) -> OSALanguage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"OSALanguage").unwrap(), languageForName : name)
    }
    unsafe fn languageForScriptDataDescriptor_(descriptor: NSAppleEventDescriptor) -> OSALanguage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"OSALanguage").unwrap(), languageForScriptDataDescriptor : descriptor)
    }
    unsafe fn defaultLanguage() -> OSALanguage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"OSALanguage").unwrap(), defaultLanguage)
    }
    unsafe fn setDefaultLanguage_(defaultLanguage: OSALanguage)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"OSALanguage").unwrap(), setDefaultLanguage : defaultLanguage)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct OSALanguageInstance(pub id);
impl std::ops::Deref for OSALanguageInstance {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for OSALanguageInstance {}
impl OSALanguageInstance {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"OSALanguageInstance").unwrap(), alloc) })
    }
}
impl INSObject for OSALanguageInstance {}
impl PNSObject for OSALanguageInstance {}
impl std::convert::TryFrom<NSObject> for OSALanguageInstance {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<OSALanguageInstance, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"OSALanguageInstance").unwrap()) };
        if is_kind_of {
            Ok(OSALanguageInstance(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to OSALanguageInstance")
        }
    }
}
impl IOSALanguageInstance for OSALanguageInstance {}
pub trait IOSALanguageInstance: Sized + std::ops::Deref {
    unsafe fn initWithLanguage_(&self, language: OSALanguage) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithLanguage : language)
    }
    unsafe fn richTextFromDescriptor_(
        &self,
        descriptor: NSAppleEventDescriptor,
    ) -> NSAttributedString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, richTextFromDescriptor : descriptor)
    }
    unsafe fn language(&self) -> OSALanguage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, language)
    }
    unsafe fn componentInstance(&self) -> ComponentInstance
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, componentInstance)
    }
    unsafe fn defaultTarget(&self) -> NSAppleEventDescriptor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, defaultTarget)
    }
    unsafe fn setDefaultTarget_(&self, defaultTarget: NSAppleEventDescriptor)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDefaultTarget : defaultTarget)
    }
    unsafe fn languageInstanceWithLanguage_(language: OSALanguage) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"OSALanguageInstance").unwrap(), languageInstanceWithLanguage : language)
    }
}
pub type OSAStorageOptions = NSUInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct OSAScript(pub id);
impl std::ops::Deref for OSAScript {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for OSAScript {}
impl OSAScript {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"OSAScript").unwrap(), alloc) })
    }
}
impl PNSCopying for OSAScript {}
impl INSObject for OSAScript {}
impl PNSObject for OSAScript {}
impl std::convert::TryFrom<NSObject> for OSAScript {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<OSAScript, Self::Error> {
        let is_kind_of: bool = unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"OSAScript").unwrap()) };
        if is_kind_of {
            Ok(OSAScript(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to OSAScript")
        }
    }
}
impl IOSAScript for OSAScript {}
pub trait IOSAScript: Sized + std::ops::Deref {
    unsafe fn initWithSource_(&self, source: NSString) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithSource : source)
    }
    unsafe fn initWithSource_language_(
        &self,
        source: NSString,
        language: OSALanguage,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithSource : source, language : language)
    }
    unsafe fn initWithSource_fromURL_languageInstance_usingStorageOptions_(
        &self,
        source: NSString,
        url: NSURL,
        instance: OSALanguageInstance,
        storageOptions: OSAStorageOptions,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithSource : source, fromURL : url, languageInstance : instance, usingStorageOptions : storageOptions)
    }
    unsafe fn initWithContentsOfURL_error_(
        &self,
        url: NSURL,
        errorInfo: *mut NSDictionary,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithContentsOfURL : url, error : errorInfo)
    }
    unsafe fn initWithContentsOfURL_language_error_(
        &self,
        url: NSURL,
        language: OSALanguage,
        errorInfo: *mut NSDictionary,
    ) -> id
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithContentsOfURL : url, language : language, error : errorInfo)
    }
    unsafe fn initWithContentsOfURL_languageInstance_usingStorageOptions_error_(
        &self,
        url: NSURL,
        instance: OSALanguageInstance,
        storageOptions: OSAStorageOptions,
        errorInfo: *mut NSError,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithContentsOfURL : url, languageInstance : instance, usingStorageOptions : storageOptions, error : errorInfo)
    }
    unsafe fn initWithCompiledData_error_(&self, data: NSData, errorInfo: *mut NSDictionary) -> id
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithCompiledData : data, error : errorInfo)
    }
    unsafe fn initWithCompiledData_fromURL_usingStorageOptions_error_(
        &self,
        data: NSData,
        url: NSURL,
        storageOptions: OSAStorageOptions,
        errorInfo: *mut NSError,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithCompiledData : data, fromURL : url, usingStorageOptions : storageOptions, error : errorInfo)
    }
    unsafe fn initWithScriptDataDescriptor_fromURL_languageInstance_usingStorageOptions_error_(
        &self,
        data: NSAppleEventDescriptor,
        url: NSURL,
        instance: OSALanguageInstance,
        storageOptions: OSAStorageOptions,
        errorInfo: *mut NSError,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithScriptDataDescriptor : data, fromURL : url, languageInstance : instance, usingStorageOptions : storageOptions, error : errorInfo)
    }
    unsafe fn compileAndReturnError_(&self, errorInfo: *mut NSDictionary) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, compileAndReturnError : errorInfo)
    }
    unsafe fn executeAndReturnError_(&self, errorInfo: *mut NSDictionary) -> NSAppleEventDescriptor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, executeAndReturnError : errorInfo)
    }
    unsafe fn executeAppleEvent_error_(
        &self,
        event: NSAppleEventDescriptor,
        errorInfo: *mut NSDictionary,
    ) -> NSAppleEventDescriptor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, executeAppleEvent : event, error : errorInfo)
    }
    unsafe fn executeAndReturnDisplayValue_error_(
        &self,
        displayValue: *mut NSAttributedString,
        errorInfo: *mut NSDictionary,
    ) -> NSAppleEventDescriptor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, executeAndReturnDisplayValue : displayValue, error : errorInfo)
    }
    unsafe fn executeHandlerWithName_arguments_error_(
        &self,
        name: NSString,
        arguments: NSArray,
        errorInfo: *mut NSDictionary,
    ) -> NSAppleEventDescriptor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, executeHandlerWithName : name, arguments : arguments, error : errorInfo)
    }
    unsafe fn richTextFromDescriptor_(
        &self,
        descriptor: NSAppleEventDescriptor,
    ) -> NSAttributedString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, richTextFromDescriptor : descriptor)
    }
    unsafe fn writeToURL_ofType_error_(
        &self,
        url: NSURL,
        type_: NSString,
        errorInfo: *mut NSDictionary,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, writeToURL : url, ofType : type_, error : errorInfo)
    }
    unsafe fn writeToURL_ofType_usingStorageOptions_error_(
        &self,
        url: NSURL,
        type_: NSString,
        storageOptions: OSAStorageOptions,
        errorInfo: *mut NSDictionary,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, writeToURL : url, ofType : type_, usingStorageOptions : storageOptions, error : errorInfo)
    }
    unsafe fn compiledDataForType_usingStorageOptions_error_(
        &self,
        type_: NSString,
        storageOptions: OSAStorageOptions,
        errorInfo: *mut NSDictionary,
    ) -> NSData
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, compiledDataForType : type_, usingStorageOptions : storageOptions, error : errorInfo)
    }
    unsafe fn source(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, source)
    }
    unsafe fn url(&self) -> NSURL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, url)
    }
    unsafe fn language(&self) -> OSALanguage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, language)
    }
    unsafe fn setLanguage_(&self, language: OSALanguage)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setLanguage : language)
    }
    unsafe fn languageInstance(&self) -> OSALanguageInstance
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, languageInstance)
    }
    unsafe fn setLanguageInstance_(&self, languageInstance: OSALanguageInstance)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setLanguageInstance : languageInstance)
    }
    unsafe fn isCompiled(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isCompiled)
    }
    unsafe fn richTextSource(&self) -> NSAttributedString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, richTextSource)
    }
    unsafe fn scriptDataDescriptorWithContentsOfURL_(url: NSURL) -> NSAppleEventDescriptor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"OSAScript").unwrap(), scriptDataDescriptorWithContentsOfURL : url)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct OSAScriptView(pub id);
impl std::ops::Deref for OSAScriptView {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for OSAScriptView {}
impl OSAScriptView {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"OSAScriptView").unwrap(), alloc) })
    }
}
impl INSTextView for OSAScriptView {}
impl PNSColorChanging for OSAScriptView {}
impl PNSMenuItemValidation for OSAScriptView {}
impl PNSUserInterfaceValidations for OSAScriptView {}
impl PNSTextInputClient for OSAScriptView {}
impl PNSTextLayoutOrientationProvider for OSAScriptView {}
impl PNSDraggingSource for OSAScriptView {}
impl PNSStandardKeyBindingResponding for OSAScriptView {}
impl PNSTextInput for OSAScriptView {}
impl PNSAccessibilityNavigableStaticText for OSAScriptView {}
impl PNSTextContent for OSAScriptView {}
impl std::convert::TryFrom<NSTextView> for OSAScriptView {
    type Error = &'static str;
    fn try_from(parent: NSTextView) -> Result<OSAScriptView, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"OSAScriptView").unwrap()) };
        if is_kind_of {
            Ok(OSAScriptView(parent.0))
        } else {
            Err("This NSTextView cannot be downcasted to OSAScriptView")
        }
    }
}
impl INSText for OSAScriptView {}
impl PNSChangeSpelling for OSAScriptView {}
impl PNSIgnoreMisspelledWords for OSAScriptView {}
impl INSView for OSAScriptView {}
impl PNSAnimatablePropertyContainer for OSAScriptView {}
impl PNSUserInterfaceItemIdentification for OSAScriptView {}
impl PNSDraggingDestination for OSAScriptView {}
impl PNSAppearanceCustomization for OSAScriptView {}
impl PNSAccessibilityElement for OSAScriptView {}
impl PNSAccessibility for OSAScriptView {}
impl INSResponder for OSAScriptView {}
impl PNSCoding for OSAScriptView {}
impl INSObject for OSAScriptView {}
impl PNSObject for OSAScriptView {}
impl IOSAScriptView for OSAScriptView {}
pub trait IOSAScriptView: Sized + std::ops::Deref {
    unsafe fn source(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, source)
    }
    unsafe fn setSource_(&self, source: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSource : source)
    }
    unsafe fn usesScriptAssistant(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, usesScriptAssistant)
    }
    unsafe fn setUsesScriptAssistant_(&self, usesScriptAssistant: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setUsesScriptAssistant : usesScriptAssistant)
    }
    unsafe fn usesTabs(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, usesTabs)
    }
    unsafe fn setUsesTabs_(&self, usesTabs: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setUsesTabs : usesTabs)
    }
    unsafe fn tabWidth(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, tabWidth)
    }
    unsafe fn setTabWidth_(&self, tabWidth: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setTabWidth : tabWidth)
    }
    unsafe fn wrapsLines(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, wrapsLines)
    }
    unsafe fn setWrapsLines_(&self, wrapsLines: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setWrapsLines : wrapsLines)
    }
    unsafe fn indentsWrappedLines(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, indentsWrappedLines)
    }
    unsafe fn setIndentsWrappedLines_(&self, indentsWrappedLines: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setIndentsWrappedLines : indentsWrappedLines)
    }
    unsafe fn indentWidth(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, indentWidth)
    }
    unsafe fn setIndentWidth_(&self, indentWidth: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setIndentWidth : indentWidth)
    }
}
pub type OSAScriptState = NSInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct OSAScriptController(pub id);
impl std::ops::Deref for OSAScriptController {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for OSAScriptController {}
impl OSAScriptController {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"OSAScriptController").unwrap(), alloc) })
    }
}
impl INSController for OSAScriptController {}
impl PNSCoding for OSAScriptController {}
impl PNSEditor for OSAScriptController {}
impl PNSEditorRegistration for OSAScriptController {}
impl std::convert::TryFrom<NSController> for OSAScriptController {
    type Error = &'static str;
    fn try_from(parent: NSController) -> Result<OSAScriptController, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"OSAScriptController").unwrap()) };
        if is_kind_of {
            Ok(OSAScriptController(parent.0))
        } else {
            Err("This NSController cannot be downcasted to OSAScriptController")
        }
    }
}
impl INSObject for OSAScriptController {}
impl PNSObject for OSAScriptController {}
impl IOSAScriptController for OSAScriptController {}
pub trait IOSAScriptController: Sized + std::ops::Deref {
    unsafe fn compileScript_(&self, sender: id)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, compileScript : sender)
    }
    unsafe fn recordScript_(&self, sender: id)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, recordScript : sender)
    }
    unsafe fn runScript_(&self, sender: id)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, runScript : sender)
    }
    unsafe fn stopScript_(&self, sender: id)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, stopScript : sender)
    }
    unsafe fn scriptView(&self) -> OSAScriptView
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, scriptView)
    }
    unsafe fn setScriptView_(&self, scriptView: OSAScriptView)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setScriptView : scriptView)
    }
    unsafe fn resultView(&self) -> NSTextView
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, resultView)
    }
    unsafe fn setResultView_(&self, resultView: NSTextView)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setResultView : resultView)
    }
    unsafe fn script(&self) -> OSAScript
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, script)
    }
    unsafe fn setScript_(&self, script: OSAScript)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setScript : script)
    }
    unsafe fn language(&self) -> OSALanguage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, language)
    }
    unsafe fn setLanguage_(&self, language: OSALanguage)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setLanguage : language)
    }
    unsafe fn scriptState(&self) -> OSAScriptState
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, scriptState)
    }
    unsafe fn isCompiling(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isCompiling)
    }
}
unsafe extern "C" {
    pub static OSAScriptErrorMessageKey: NSString;
}
unsafe extern "C" {
    pub static OSAScriptErrorBriefMessageKey: NSString;
}
unsafe extern "C" {
    pub static OSAScriptErrorNumberKey: NSString;
}
unsafe extern "C" {
    pub static OSAScriptErrorPartialResultKey: NSString;
}
unsafe extern "C" {
    pub static OSAScriptErrorOffendingObjectKey: NSString;
}
unsafe extern "C" {
    pub static OSAScriptErrorExpectedTypeKey: NSString;
}
unsafe extern "C" {
    pub static OSAScriptErrorAppAddressKey: NSString;
}
unsafe extern "C" {
    pub static OSAScriptErrorAppNameKey: NSString;
}
unsafe extern "C" {
    pub static OSAScriptErrorRangeKey: NSString;
}
unsafe extern "C" {
    pub static OSAScriptErrorMessage: NSString;
}
unsafe extern "C" {
    pub static OSAScriptErrorNumber: NSString;
}
unsafe extern "C" {
    pub static OSAScriptErrorAppName: NSString;
}
unsafe extern "C" {
    pub static OSAScriptErrorBriefMessage: NSString;
}
unsafe extern "C" {
    pub static OSAScriptErrorRange: NSString;
}
unsafe extern "C" {
    pub static OSAStorageScriptType: NSString;
}
unsafe extern "C" {
    pub static OSAStorageScriptBundleType: NSString;
}
unsafe extern "C" {
    pub static OSAStorageApplicationType: NSString;
}
unsafe extern "C" {
    pub static OSAStorageApplicationBundleType: NSString;
}
unsafe extern "C" {
    pub static OSAStorageTextType: NSString;
}

unsafe impl objc2::encode::RefEncode for OSALanguage {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for OSALanguage {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for OSALanguageInstance {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for OSALanguageInstance {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for OSAScript {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for OSAScript {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for OSAScriptView {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for OSAScriptView {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for OSAScriptController {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for OSAScriptController {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
