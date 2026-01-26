#[allow(unused_imports)]
use crate::objc::*;
#[allow(unused_imports)]
use crate::AppKit::*;
#[allow(unused_imports)]
use crate::CoreFoundation::*;
#[allow(unused_imports)]
use crate::Foundation::*;
#[allow(unused_imports)]
use crate::OSAKit::*;

#[allow(unused_imports)]
use objc2::msg_send;
pub type AMLogLevel = NSUInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct AMAction(pub id);
impl std::ops::Deref for AMAction {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for AMAction {}
impl AMAction {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"AMAction").unwrap(), alloc) })
    }
}
impl INSObject for AMAction {}
impl PNSObject for AMAction {}
impl std::convert::TryFrom<NSObject> for AMAction {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<AMAction, Self::Error> {
        let is_kind_of: bool = unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"AMAction").unwrap()) };
        if is_kind_of {
            Ok(AMAction(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to AMAction")
        }
    }
}
impl IAMAction for AMAction {}
pub trait IAMAction: Sized + std::ops::Deref {
    unsafe fn initWithDefinition_fromArchive_(
        &self,
        dict: NSDictionary,
        archived: BOOL,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithDefinition : dict, fromArchive : archived)
    }
    unsafe fn initWithContentsOfURL_error_(
        &self,
        fileURL: NSURL,
        outError: *mut NSError,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithContentsOfURL : fileURL, error : outError)
    }
    unsafe fn runWithInput_fromAction_error_(
        &self,
        input: id,
        anAction: AMAction,
        errorInfo: *mut NSDictionary,
    ) -> id
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, runWithInput : input, fromAction : anAction, error : errorInfo)
    }
    unsafe fn runWithInput_error_(&self, input: id, error: *mut NSError) -> id
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, runWithInput : input, error : error)
    }
    unsafe fn runAsynchronouslyWithInput_(&self, input: id)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, runAsynchronouslyWithInput : input)
    }
    unsafe fn willFinishRunning(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, willFinishRunning)
    }
    unsafe fn didFinishRunningWithError_(&self, errorInfo: NSDictionary)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, didFinishRunningWithError : errorInfo)
    }
    unsafe fn finishRunningWithError_(&self, error: NSError)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, finishRunningWithError : error)
    }
    unsafe fn stop(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, stop)
    }
    unsafe fn reset(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, reset)
    }
    unsafe fn writeToDictionary_(&self, dictionary: NSMutableDictionary)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, writeToDictionary : dictionary)
    }
    unsafe fn opened(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, opened)
    }
    unsafe fn activated(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, activated)
    }
    unsafe fn closed(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, closed)
    }
    unsafe fn updateParameters(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, updateParameters)
    }
    unsafe fn parametersUpdated(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, parametersUpdated)
    }
    unsafe fn logMessageWithLevel_format_(&self, level: AMLogLevel, format: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, logMessageWithLevel : level, format : format)
    }
    unsafe fn name(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, name)
    }
    unsafe fn ignoresInput(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, ignoresInput)
    }
    unsafe fn selectedInputType(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, selectedInputType)
    }
    unsafe fn setSelectedInputType_(&self, selectedInputType: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSelectedInputType : selectedInputType)
    }
    unsafe fn selectedOutputType(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, selectedOutputType)
    }
    unsafe fn setSelectedOutputType_(&self, selectedOutputType: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSelectedOutputType : selectedOutputType)
    }
    unsafe fn progressValue(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, progressValue)
    }
    unsafe fn setProgressValue_(&self, progressValue: CGFloat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setProgressValue : progressValue)
    }
    unsafe fn output(&self) -> id
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, output)
    }
    unsafe fn setOutput_(&self, output: id)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setOutput : output)
    }
    unsafe fn isStopped(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isStopped)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct AMBundleAction(pub id);
impl std::ops::Deref for AMBundleAction {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for AMBundleAction {}
impl AMBundleAction {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"AMBundleAction").unwrap(), alloc) })
    }
}
impl PNSCopying for AMBundleAction {}
impl PNSSecureCoding for AMBundleAction {}
impl IAMAction for AMBundleAction {}
impl From<AMBundleAction> for AMAction {
    fn from(child: AMBundleAction) -> AMAction {
        AMAction(child.0)
    }
}
impl std::convert::TryFrom<AMAction> for AMBundleAction {
    type Error = &'static str;
    fn try_from(parent: AMAction) -> Result<AMBundleAction, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"AMBundleAction").unwrap()) };
        if is_kind_of {
            Ok(AMBundleAction(parent.0))
        } else {
            Err("This AMAction cannot be downcasted to AMBundleAction")
        }
    }
}
impl INSObject for AMBundleAction {}
impl PNSObject for AMBundleAction {}
impl IAMBundleAction for AMBundleAction {}
pub trait IAMBundleAction: Sized + std::ops::Deref {
    unsafe fn awakeFromBundle(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, awakeFromBundle)
    }
    unsafe fn hasView(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, hasView)
    }
    unsafe fn view(&self) -> NSView
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, view)
    }
    unsafe fn bundle(&self) -> NSBundle
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, bundle)
    }
    unsafe fn parameters(&self) -> NSMutableDictionary
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, parameters)
    }
    unsafe fn setParameters_(&self, parameters: NSMutableDictionary)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setParameters : parameters)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct AMAppleScriptAction(pub id);
impl std::ops::Deref for AMAppleScriptAction {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for AMAppleScriptAction {}
impl AMAppleScriptAction {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"AMAppleScriptAction").unwrap(), alloc) })
    }
}
impl IAMBundleAction for AMAppleScriptAction {}
impl PNSCopying for AMAppleScriptAction {}
impl PNSSecureCoding for AMAppleScriptAction {}
impl From<AMAppleScriptAction> for AMBundleAction {
    fn from(child: AMAppleScriptAction) -> AMBundleAction {
        AMBundleAction(child.0)
    }
}
impl std::convert::TryFrom<AMBundleAction> for AMAppleScriptAction {
    type Error = &'static str;
    fn try_from(parent: AMBundleAction) -> Result<AMAppleScriptAction, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"AMAppleScriptAction").unwrap()) };
        if is_kind_of {
            Ok(AMAppleScriptAction(parent.0))
        } else {
            Err("This AMBundleAction cannot be downcasted to AMAppleScriptAction")
        }
    }
}
impl IAMAction for AMAppleScriptAction {}
impl INSObject for AMAppleScriptAction {}
impl PNSObject for AMAppleScriptAction {}
impl IAMAppleScriptAction for AMAppleScriptAction {}
pub trait IAMAppleScriptAction: Sized + std::ops::Deref {
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
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct AMShellScriptAction(pub id);
impl std::ops::Deref for AMShellScriptAction {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for AMShellScriptAction {}
impl AMShellScriptAction {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"AMShellScriptAction").unwrap(), alloc) })
    }
}
impl IAMBundleAction for AMShellScriptAction {}
impl PNSCopying for AMShellScriptAction {}
impl PNSSecureCoding for AMShellScriptAction {}
impl std::convert::TryFrom<AMBundleAction> for AMShellScriptAction {
    type Error = &'static str;
    fn try_from(parent: AMBundleAction) -> Result<AMShellScriptAction, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"AMShellScriptAction").unwrap()) };
        if is_kind_of {
            Ok(AMShellScriptAction(parent.0))
        } else {
            Err("This AMBundleAction cannot be downcasted to AMShellScriptAction")
        }
    }
}
impl IAMAction for AMShellScriptAction {}
impl INSObject for AMShellScriptAction {}
impl PNSObject for AMShellScriptAction {}
impl IAMShellScriptAction for AMShellScriptAction {}
pub trait IAMShellScriptAction: Sized + std::ops::Deref {
    unsafe fn remapLineEndings(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, remapLineEndings)
    }
    unsafe fn inputFieldSeparator(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inputFieldSeparator)
    }
    unsafe fn outputFieldSeparator(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, outputFieldSeparator)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct AMWorkflow(pub id);
impl std::ops::Deref for AMWorkflow {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for AMWorkflow {}
impl AMWorkflow {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"AMWorkflow").unwrap(), alloc) })
    }
}
impl PNSCopying for AMWorkflow {}
impl INSObject for AMWorkflow {}
impl PNSObject for AMWorkflow {}
impl std::convert::TryFrom<NSObject> for AMWorkflow {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<AMWorkflow, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"AMWorkflow").unwrap()) };
        if is_kind_of {
            Ok(AMWorkflow(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to AMWorkflow")
        }
    }
}
impl IAMWorkflow for AMWorkflow {}
pub trait IAMWorkflow: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn initWithContentsOfURL_error_(
        &self,
        fileURL: NSURL,
        outError: *mut NSError,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithContentsOfURL : fileURL, error : outError)
    }
    unsafe fn writeToURL_error_(&self, fileURL: NSURL, outError: *mut NSError) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, writeToURL : fileURL, error : outError)
    }
    unsafe fn setValue_forVariableWithName_(&self, value: id, variableName: NSString) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setValue : value, forVariableWithName : variableName)
    }
    unsafe fn valueForVariableWithName_(&self, variableName: NSString) -> id
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, valueForVariableWithName : variableName)
    }
    unsafe fn addAction_(&self, action: AMAction)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, addAction : action)
    }
    unsafe fn removeAction_(&self, action: AMAction)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, removeAction : action)
    }
    unsafe fn insertAction_atIndex_(&self, action: AMAction, index: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, insertAction : action, atIndex : index)
    }
    unsafe fn moveActionAtIndex_toIndex_(&self, startIndex: NSUInteger, endIndex: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, moveActionAtIndex : startIndex, toIndex : endIndex)
    }
    unsafe fn fileURL(&self) -> NSURL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, fileURL)
    }
    unsafe fn actions(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, actions)
    }
    unsafe fn input(&self) -> id
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, input)
    }
    unsafe fn setInput_(&self, input: id)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInput : input)
    }
    unsafe fn output(&self) -> id
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, output)
    }
    unsafe fn runWorkflowAtURL_withInput_error_(
        fileURL: NSURL,
        input: id,
        error: *mut NSError,
    ) -> id
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"AMWorkflow").unwrap(), runWorkflowAtURL : fileURL, withInput : input, error : error)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct AMWorkflowController(pub id);
impl std::ops::Deref for AMWorkflowController {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for AMWorkflowController {}
impl AMWorkflowController {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"AMWorkflowController").unwrap(), alloc) })
    }
}
impl INSController for AMWorkflowController {}
impl PNSCoding for AMWorkflowController {}
impl PNSEditor for AMWorkflowController {}
impl PNSEditorRegistration for AMWorkflowController {}
impl std::convert::TryFrom<NSController> for AMWorkflowController {
    type Error = &'static str;
    fn try_from(parent: NSController) -> Result<AMWorkflowController, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"AMWorkflowController").unwrap()) };
        if is_kind_of {
            Ok(AMWorkflowController(parent.0))
        } else {
            Err("This NSController cannot be downcasted to AMWorkflowController")
        }
    }
}
impl INSObject for AMWorkflowController {}
impl PNSObject for AMWorkflowController {}
impl IAMWorkflowController for AMWorkflowController {}
pub trait IAMWorkflowController: Sized + std::ops::Deref {
    unsafe fn run_(&self, sender: id)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, run : sender)
    }
    unsafe fn stop_(&self, sender: id)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, stop : sender)
    }
    unsafe fn pause_(&self, sender: id)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, pause : sender)
    }
    unsafe fn step_(&self, sender: id)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, step : sender)
    }
    unsafe fn reset_(&self, sender: id)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, reset : sender)
    }
    unsafe fn workflow(&self) -> AMWorkflow
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, workflow)
    }
    unsafe fn setWorkflow_(&self, workflow: AMWorkflow)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setWorkflow : workflow)
    }
    unsafe fn workflowView(&self) -> AMWorkflowView
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, workflowView)
    }
    unsafe fn setWorkflowView_(&self, workflowView: AMWorkflowView)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setWorkflowView : workflowView)
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
    unsafe fn canRun(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, canRun)
    }
    unsafe fn isRunning(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isRunning)
    }
    unsafe fn isPaused(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isPaused)
    }
}
pub trait PAMWorkflowControllerDelegate: Sized + std::ops::Deref {
    unsafe fn workflowControllerWillRun_(&self, controller: AMWorkflowController)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, workflowControllerWillRun : controller)
    }
    unsafe fn workflowControllerWillStop_(&self, controller: AMWorkflowController)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, workflowControllerWillStop : controller)
    }
    unsafe fn workflowControllerDidRun_(&self, controller: AMWorkflowController)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, workflowControllerDidRun : controller)
    }
    unsafe fn workflowControllerDidStop_(&self, controller: AMWorkflowController)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, workflowControllerDidStop : controller)
    }
    unsafe fn workflowController_willRunAction_(
        &self,
        controller: AMWorkflowController,
        action: AMAction,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, workflowController : controller, willRunAction : action)
    }
    unsafe fn workflowController_didRunAction_(
        &self,
        controller: AMWorkflowController,
        action: AMAction,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, workflowController : controller, didRunAction : action)
    }
    unsafe fn workflowController_didError_(&self, controller: AMWorkflowController, error: NSError)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, workflowController : controller, didError : error)
    }
}
pub trait NSObject_AMWorkflowControllerDelegate: Sized + std::ops::Deref {
    unsafe fn workflowControllerWillRun_(&self, controller: AMWorkflowController)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, workflowControllerWillRun : controller)
    }
    unsafe fn workflowControllerWillStop_(&self, controller: AMWorkflowController)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, workflowControllerWillStop : controller)
    }
    unsafe fn workflowControllerDidRun_(&self, controller: AMWorkflowController)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, workflowControllerDidRun : controller)
    }
    unsafe fn workflowControllerDidStop_(&self, controller: AMWorkflowController)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, workflowControllerDidStop : controller)
    }
    unsafe fn workflowController_willRunAction_(
        &self,
        controller: AMWorkflowController,
        action: AMAction,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, workflowController : controller, willRunAction : action)
    }
    unsafe fn workflowController_didRunAction_(
        &self,
        controller: AMWorkflowController,
        action: AMAction,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, workflowController : controller, didRunAction : action)
    }
    unsafe fn workflowController_didError_(&self, controller: AMWorkflowController, error: NSError)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, workflowController : controller, didError : error)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct AMWorkflowView(pub id);
impl std::ops::Deref for AMWorkflowView {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for AMWorkflowView {}
impl AMWorkflowView {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"AMWorkflowView").unwrap(), alloc) })
    }
}
impl INSView for AMWorkflowView {}
impl PNSAnimatablePropertyContainer for AMWorkflowView {}
impl PNSUserInterfaceItemIdentification for AMWorkflowView {}
impl PNSDraggingDestination for AMWorkflowView {}
impl PNSAppearanceCustomization for AMWorkflowView {}
impl PNSAccessibilityElement for AMWorkflowView {}
impl PNSAccessibility for AMWorkflowView {}
impl std::convert::TryFrom<NSView> for AMWorkflowView {
    type Error = &'static str;
    fn try_from(parent: NSView) -> Result<AMWorkflowView, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"AMWorkflowView").unwrap()) };
        if is_kind_of {
            Ok(AMWorkflowView(parent.0))
        } else {
            Err("This NSView cannot be downcasted to AMWorkflowView")
        }
    }
}
impl INSResponder for AMWorkflowView {}
impl PNSCoding for AMWorkflowView {}
impl INSObject for AMWorkflowView {}
impl PNSObject for AMWorkflowView {}
impl IAMWorkflowView for AMWorkflowView {}
pub trait IAMWorkflowView: Sized + std::ops::Deref {
    unsafe fn isEditable(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isEditable)
    }
    unsafe fn setEditable_(&self, editable: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setEditable : editable)
    }
    unsafe fn workflowController(&self) -> AMWorkflowController
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, workflowController)
    }
    unsafe fn setWorkflowController_(&self, workflowController: AMWorkflowController)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setWorkflowController : workflowController)
    }
}
pub type AMErrorCode = NSInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct AMWorkspace(pub id);
impl std::ops::Deref for AMWorkspace {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for AMWorkspace {}
impl AMWorkspace {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"AMWorkspace").unwrap(), alloc) })
    }
}
impl INSObject for AMWorkspace {}
impl PNSObject for AMWorkspace {}
impl std::convert::TryFrom<NSObject> for AMWorkspace {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<AMWorkspace, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"AMWorkspace").unwrap()) };
        if is_kind_of {
            Ok(AMWorkspace(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to AMWorkspace")
        }
    }
}
impl IAMWorkspace for AMWorkspace {}
pub trait IAMWorkspace: Sized + std::ops::Deref {
    unsafe fn runWorkflowAtPath_withInput_error_(
        &self,
        path: NSString,
        input: id,
        error: *mut NSError,
    ) -> id
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, runWorkflowAtPath : path, withInput : input, error : error)
    }
    unsafe fn sharedWorkspace() -> AMWorkspace
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"AMWorkspace").unwrap(), sharedWorkspace)
    }
}

unsafe impl objc2::encode::RefEncode for AMAction {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for AMAction {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for AMBundleAction {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for AMBundleAction {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for AMAppleScriptAction {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for AMAppleScriptAction {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for AMShellScriptAction {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for AMShellScriptAction {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for AMWorkflow {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for AMWorkflow {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for AMWorkflowController {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for AMWorkflowController {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for AMWorkflowView {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for AMWorkflowView {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for AMWorkspace {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for AMWorkspace {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
