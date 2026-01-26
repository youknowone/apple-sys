#[allow(unused_imports)]
use crate::objc::*;
#[allow(unused_imports)]
use crate::AppKit::*;
#[allow(unused_imports)]
use crate::AudioToolbox::*;
#[allow(unused_imports)]
use crate::CoreFoundation::*;
#[allow(unused_imports)]
use crate::Foundation::*;

#[allow(unused_imports)]
use objc2::msg_send;
pub trait PAUCustomViewPersistentData: Sized + std::ops::Deref {
    unsafe fn customViewPersistentData(&self) -> NSDictionary
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, customViewPersistentData)
    }
    unsafe fn setCustomViewPersistentData_(&self, customViewPersistentData: NSDictionary)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCustomViewPersistentData : customViewPersistentData)
    }
}
pub type AUGenericViewDisplayFlags = UInt32;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct AUGenericView(pub id);
impl std::ops::Deref for AUGenericView {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for AUGenericView {}
impl AUGenericView {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"AUGenericView").unwrap(), alloc) })
    }
}
impl PAUCustomViewPersistentData for AUGenericView {}
impl INSView for AUGenericView {}
impl PNSAnimatablePropertyContainer for AUGenericView {}
impl PNSUserInterfaceItemIdentification for AUGenericView {}
impl PNSDraggingDestination for AUGenericView {}
impl PNSAppearanceCustomization for AUGenericView {}
impl PNSAccessibilityElement for AUGenericView {}
impl PNSAccessibility for AUGenericView {}
impl std::convert::TryFrom<NSView> for AUGenericView {
    type Error = &'static str;
    fn try_from(parent: NSView) -> Result<AUGenericView, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"AUGenericView").unwrap()) };
        if is_kind_of {
            Ok(AUGenericView(parent.0))
        } else {
            Err("This NSView cannot be downcasted to AUGenericView")
        }
    }
}
impl INSResponder for AUGenericView {}
impl PNSCoding for AUGenericView {}
impl INSObject for AUGenericView {}
impl PNSObject for AUGenericView {}
impl IAUGenericView for AUGenericView {}
pub trait IAUGenericView: Sized + std::ops::Deref {
    unsafe fn initWithAudioUnit_(&self, au: AudioUnit) -> AUGenericView
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithAudioUnit : au)
    }
    unsafe fn initWithAudioUnit_displayFlags_(
        &self,
        inAudioUnit: AudioUnit,
        inFlags: AUGenericViewDisplayFlags,
    ) -> AUGenericView
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithAudioUnit : inAudioUnit, displayFlags : inFlags)
    }
    unsafe fn audioUnit(&self) -> AudioUnit
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, audioUnit)
    }
    unsafe fn showsExpertParameters(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, showsExpertParameters)
    }
    unsafe fn setShowsExpertParameters_(&self, showsExpertParameters: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setShowsExpertParameters : showsExpertParameters)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct AUPannerView(pub id);
impl std::ops::Deref for AUPannerView {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for AUPannerView {}
impl AUPannerView {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"AUPannerView").unwrap(), alloc) })
    }
}
impl INSView for AUPannerView {}
impl PNSAnimatablePropertyContainer for AUPannerView {}
impl PNSUserInterfaceItemIdentification for AUPannerView {}
impl PNSDraggingDestination for AUPannerView {}
impl PNSAppearanceCustomization for AUPannerView {}
impl PNSAccessibilityElement for AUPannerView {}
impl PNSAccessibility for AUPannerView {}
impl std::convert::TryFrom<NSView> for AUPannerView {
    type Error = &'static str;
    fn try_from(parent: NSView) -> Result<AUPannerView, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"AUPannerView").unwrap()) };
        if is_kind_of {
            Ok(AUPannerView(parent.0))
        } else {
            Err("This NSView cannot be downcasted to AUPannerView")
        }
    }
}
impl INSResponder for AUPannerView {}
impl PNSCoding for AUPannerView {}
impl INSObject for AUPannerView {}
impl PNSObject for AUPannerView {}
impl IAUPannerView for AUPannerView {}
pub trait IAUPannerView: Sized + std::ops::Deref {
    unsafe fn audioUnit(&self) -> AudioUnit
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, audioUnit)
    }
    unsafe fn AUPannerViewWithAudioUnit_(au: AudioUnit) -> AUPannerView
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"AUPannerView").unwrap(), AUPannerViewWithAudioUnit : au)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CANetworkBrowserWindowController(pub id);
impl std::ops::Deref for CANetworkBrowserWindowController {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CANetworkBrowserWindowController {}
impl CANetworkBrowserWindowController {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CANetworkBrowserWindowController").unwrap(), alloc) })
    }
}
impl INSWindowController for CANetworkBrowserWindowController {}
impl PNSSeguePerforming for CANetworkBrowserWindowController {}
impl std::convert::TryFrom<NSWindowController> for CANetworkBrowserWindowController {
    type Error = &'static str;
    fn try_from(
        parent: NSWindowController,
    ) -> Result<CANetworkBrowserWindowController, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CANetworkBrowserWindowController").unwrap())
        };
        if is_kind_of {
            Ok(CANetworkBrowserWindowController(parent.0))
        } else {
            Err("This NSWindowController cannot be downcasted to CANetworkBrowserWindowController")
        }
    }
}
impl INSResponder for CANetworkBrowserWindowController {}
impl PNSCoding for CANetworkBrowserWindowController {}
impl INSObject for CANetworkBrowserWindowController {}
impl PNSObject for CANetworkBrowserWindowController {}
impl ICANetworkBrowserWindowController for CANetworkBrowserWindowController {}
pub trait ICANetworkBrowserWindowController: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn isAVBSupported() -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CANetworkBrowserWindowController").unwrap(), isAVBSupported)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CABTLEMIDIWindowController(pub id);
impl std::ops::Deref for CABTLEMIDIWindowController {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CABTLEMIDIWindowController {}
impl CABTLEMIDIWindowController {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CABTLEMIDIWindowController").unwrap(), alloc) })
    }
}
impl INSWindowController for CABTLEMIDIWindowController {}
impl PNSSeguePerforming for CABTLEMIDIWindowController {}
impl std::convert::TryFrom<NSWindowController> for CABTLEMIDIWindowController {
    type Error = &'static str;
    fn try_from(parent: NSWindowController) -> Result<CABTLEMIDIWindowController, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CABTLEMIDIWindowController").unwrap()) };
        if is_kind_of {
            Ok(CABTLEMIDIWindowController(parent.0))
        } else {
            Err("This NSWindowController cannot be downcasted to CABTLEMIDIWindowController")
        }
    }
}
impl INSResponder for CABTLEMIDIWindowController {}
impl PNSCoding for CABTLEMIDIWindowController {}
impl INSObject for CABTLEMIDIWindowController {}
impl PNSObject for CABTLEMIDIWindowController {}
impl ICABTLEMIDIWindowController for CABTLEMIDIWindowController {}
pub trait ICABTLEMIDIWindowController: Sized + std::ops::Deref {}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CAInterDeviceAudioViewController(pub id);
impl std::ops::Deref for CAInterDeviceAudioViewController {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CAInterDeviceAudioViewController {}
impl CAInterDeviceAudioViewController {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CAInterDeviceAudioViewController").unwrap(), alloc) })
    }
}
impl INSViewController for CAInterDeviceAudioViewController {}
impl PNSEditor for CAInterDeviceAudioViewController {}
impl PNSSeguePerforming for CAInterDeviceAudioViewController {}
impl PNSUserInterfaceItemIdentification for CAInterDeviceAudioViewController {}
impl std::convert::TryFrom<NSViewController> for CAInterDeviceAudioViewController {
    type Error = &'static str;
    fn try_from(parent: NSViewController) -> Result<CAInterDeviceAudioViewController, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CAInterDeviceAudioViewController").unwrap())
        };
        if is_kind_of {
            Ok(CAInterDeviceAudioViewController(parent.0))
        } else {
            Err("This NSViewController cannot be downcasted to CAInterDeviceAudioViewController")
        }
    }
}
impl INSResponder for CAInterDeviceAudioViewController {}
impl PNSCoding for CAInterDeviceAudioViewController {}
impl INSObject for CAInterDeviceAudioViewController {}
impl PNSObject for CAInterDeviceAudioViewController {}
impl ICAInterDeviceAudioViewController for CAInterDeviceAudioViewController {}
pub trait ICAInterDeviceAudioViewController: Sized + std::ops::Deref {}
pub type AUViewControllerBase = NSViewController;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct AUViewController(pub id);
impl std::ops::Deref for AUViewController {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for AUViewController {}
impl AUViewController {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"AUViewController").unwrap(), alloc) })
    }
}
impl PNSExtensionRequestHandling for AUViewController {}
impl INSViewController for AUViewController {}
impl PNSEditor for AUViewController {}
impl PNSSeguePerforming for AUViewController {}
impl PNSUserInterfaceItemIdentification for AUViewController {}
impl std::convert::TryFrom<NSViewController> for AUViewController {
    type Error = &'static str;
    fn try_from(parent: NSViewController) -> Result<AUViewController, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"AUViewController").unwrap()) };
        if is_kind_of {
            Ok(AUViewController(parent.0))
        } else {
            Err("This NSViewController cannot be downcasted to AUViewController")
        }
    }
}
impl INSResponder for AUViewController {}
impl PNSCoding for AUViewController {}
impl INSObject for AUViewController {}
impl PNSObject for AUViewController {}
impl IAUViewController for AUViewController {}
pub trait IAUViewController: Sized + std::ops::Deref {}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct AUAudioUnitViewConfiguration(pub id);
impl std::ops::Deref for AUAudioUnitViewConfiguration {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for AUAudioUnitViewConfiguration {}
impl AUAudioUnitViewConfiguration {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"AUAudioUnitViewConfiguration").unwrap(), alloc) })
    }
}
impl PNSSecureCoding for AUAudioUnitViewConfiguration {}
impl INSObject for AUAudioUnitViewConfiguration {}
impl PNSObject for AUAudioUnitViewConfiguration {}
impl std::convert::TryFrom<NSObject> for AUAudioUnitViewConfiguration {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<AUAudioUnitViewConfiguration, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"AUAudioUnitViewConfiguration").unwrap()) };
        if is_kind_of {
            Ok(AUAudioUnitViewConfiguration(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to AUAudioUnitViewConfiguration")
        }
    }
}
impl IAUAudioUnitViewConfiguration for AUAudioUnitViewConfiguration {}
pub trait IAUAudioUnitViewConfiguration: Sized + std::ops::Deref {
    unsafe fn initWithWidth_height_hostHasController_(
        &self,
        width: CGFloat,
        height: CGFloat,
        hostHasController: BOOL,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithWidth : width, height : height, hostHasController : hostHasController)
    }
    unsafe fn width(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, width)
    }
    unsafe fn height(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, height)
    }
    unsafe fn hostHasController(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, hostHasController)
    }
}
pub trait AUAudioUnit_AUAudioUnit_ViewController: Sized + std::ops::Deref {
    unsafe fn requestViewControllerWithCompletionHandler_(
        &self,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, requestViewControllerWithCompletionHandler : completionHandler)
    }
    unsafe fn supportedViewConfigurations_(
        &self,
        availableViewConfigurations: NSArray,
    ) -> NSIndexSet
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, supportedViewConfigurations : availableViewConfigurations)
    }
    unsafe fn selectViewConfiguration_(&self, viewConfiguration: AUAudioUnitViewConfiguration)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, selectViewConfiguration : viewConfiguration)
    }
}
pub type AUViewColor = NSColor;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct AUGenericViewController(pub id);
impl std::ops::Deref for AUGenericViewController {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for AUGenericViewController {}
impl AUGenericViewController {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"AUGenericViewController").unwrap(), alloc) })
    }
}
impl INSViewController for AUGenericViewController {}
impl PNSEditor for AUGenericViewController {}
impl PNSSeguePerforming for AUGenericViewController {}
impl PNSUserInterfaceItemIdentification for AUGenericViewController {}
impl std::convert::TryFrom<NSViewController> for AUGenericViewController {
    type Error = &'static str;
    fn try_from(parent: NSViewController) -> Result<AUGenericViewController, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"AUGenericViewController").unwrap()) };
        if is_kind_of {
            Ok(AUGenericViewController(parent.0))
        } else {
            Err("This NSViewController cannot be downcasted to AUGenericViewController")
        }
    }
}
impl INSResponder for AUGenericViewController {}
impl PNSCoding for AUGenericViewController {}
impl INSObject for AUGenericViewController {}
impl PNSObject for AUGenericViewController {}
impl IAUGenericViewController for AUGenericViewController {}
pub trait IAUGenericViewController: Sized + std::ops::Deref {
    unsafe fn auAudioUnit(&self) -> AUAudioUnit
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, auAudioUnit)
    }
    unsafe fn setAuAudioUnit_(&self, auAudioUnit: AUAudioUnit)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAuAudioUnit : auAudioUnit)
    }
}

unsafe impl objc2::encode::RefEncode for AUGenericView {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for AUGenericView {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for AUPannerView {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for AUPannerView {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CANetworkBrowserWindowController {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CANetworkBrowserWindowController {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CABTLEMIDIWindowController {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CABTLEMIDIWindowController {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CAInterDeviceAudioViewController {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CAInterDeviceAudioViewController {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for AUViewController {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for AUViewController {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for AUAudioUnitViewConfiguration {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for AUAudioUnitViewConfiguration {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for AUGenericViewController {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for AUGenericViewController {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
