#[allow(unused_imports)]
use crate::objc::*;
#[allow(unused_imports)]
use crate::AddressBook::*;
#[allow(unused_imports)]
use crate::Foundation::*;
#[allow(unused_imports)]
use crate::Security::*;

#[allow(unused_imports)]
use objc2::msg_send;
pub type NCUpdateResult = NSUInteger;
pub trait PNCWidgetProviding: Sized + std::ops::Deref {
    unsafe fn widgetPerformUpdateWithCompletionHandler_(
        &self,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, widgetPerformUpdateWithCompletionHandler : completionHandler)
    }
    unsafe fn widgetMarginInsetsForProposedMarginInsets_(
        &self,
        defaultMarginInset: NSEdgeInsets,
    ) -> NSEdgeInsets
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, widgetMarginInsetsForProposedMarginInsets : defaultMarginInset)
    }
    unsafe fn widgetDidBeginEditing(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, widgetDidBeginEditing)
    }
    unsafe fn widgetDidEndEditing(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, widgetDidEndEditing)
    }
    unsafe fn widgetAllowsEditing(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, widgetAllowsEditing)
    }
}
pub trait NSViewController_NCWidgetProvidingPresentationStyles: Sized + std::ops::Deref {
    unsafe fn presentViewControllerInWidget_(&self, viewController: NSViewController)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, presentViewControllerInWidget : viewController)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct NCWidgetController(pub id);
impl std::ops::Deref for NCWidgetController {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for NCWidgetController {}
impl NCWidgetController {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"NCWidgetController").unwrap(), alloc) })
    }
}
impl INSObject for NCWidgetController {}
impl PNSObject for NCWidgetController {}
impl std::convert::TryFrom<NSObject> for NCWidgetController {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<NCWidgetController, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"NCWidgetController").unwrap()) };
        if is_kind_of {
            Ok(NCWidgetController(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to NCWidgetController")
        }
    }
}
impl INCWidgetController for NCWidgetController {}
pub trait INCWidgetController: Sized + std::ops::Deref {
    unsafe fn setHasContent_forWidgetWithBundleIdentifier_(&self, flag: BOOL, bundleID: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setHasContent : flag, forWidgetWithBundleIdentifier : bundleID)
    }
    unsafe fn widgetController() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NCWidgetController").unwrap(), widgetController)
    }
    unsafe fn defaultWidgetController() -> NCWidgetController
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NCWidgetController").unwrap(), defaultWidgetController)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct NCWidgetListViewController(pub id);
impl std::ops::Deref for NCWidgetListViewController {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for NCWidgetListViewController {}
impl NCWidgetListViewController {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"NCWidgetListViewController").unwrap(), alloc) })
    }
}
impl INSViewController for NCWidgetListViewController {}
impl PNSEditor for NCWidgetListViewController {}
impl PNSSeguePerforming for NCWidgetListViewController {}
impl PNSUserInterfaceItemIdentification for NCWidgetListViewController {}
impl std::convert::TryFrom<NSViewController> for NCWidgetListViewController {
    type Error = &'static str;
    fn try_from(parent: NSViewController) -> Result<NCWidgetListViewController, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"NCWidgetListViewController").unwrap()) };
        if is_kind_of {
            Ok(NCWidgetListViewController(parent.0))
        } else {
            Err("This NSViewController cannot be downcasted to NCWidgetListViewController")
        }
    }
}
impl INSResponder for NCWidgetListViewController {}
impl PNSCoding for NCWidgetListViewController {}
impl INSObject for NCWidgetListViewController {}
impl PNSObject for NCWidgetListViewController {}
impl INCWidgetListViewController for NCWidgetListViewController {}
pub trait INCWidgetListViewController: Sized + std::ops::Deref {
    unsafe fn viewControllerAtRow_makeIfNecessary_(
        &self,
        row: NSUInteger,
        makeIfNecesary: BOOL,
    ) -> NSViewController
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, viewControllerAtRow : row, makeIfNecessary : makeIfNecesary)
    }
    unsafe fn rowForViewController_(&self, viewController: NSViewController) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, rowForViewController : viewController)
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
    unsafe fn contents(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, contents)
    }
    unsafe fn setContents_(&self, contents: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setContents : contents)
    }
    unsafe fn minimumVisibleRowCount(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, minimumVisibleRowCount)
    }
    unsafe fn setMinimumVisibleRowCount_(&self, minimumVisibleRowCount: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMinimumVisibleRowCount : minimumVisibleRowCount)
    }
    unsafe fn hasDividerLines(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, hasDividerLines)
    }
    unsafe fn setHasDividerLines_(&self, hasDividerLines: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setHasDividerLines : hasDividerLines)
    }
    unsafe fn editing(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, editing)
    }
    unsafe fn setEditing_(&self, editing: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setEditing : editing)
    }
    unsafe fn showsAddButtonWhenEditing(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, showsAddButtonWhenEditing)
    }
    unsafe fn setShowsAddButtonWhenEditing_(&self, showsAddButtonWhenEditing: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setShowsAddButtonWhenEditing : showsAddButtonWhenEditing)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct NCWidgetSearchViewController(pub id);
impl std::ops::Deref for NCWidgetSearchViewController {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for NCWidgetSearchViewController {}
impl NCWidgetSearchViewController {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"NCWidgetSearchViewController").unwrap(), alloc) })
    }
}
impl INSViewController for NCWidgetSearchViewController {}
impl PNSEditor for NCWidgetSearchViewController {}
impl PNSSeguePerforming for NCWidgetSearchViewController {}
impl PNSUserInterfaceItemIdentification for NCWidgetSearchViewController {}
impl std::convert::TryFrom<NSViewController> for NCWidgetSearchViewController {
    type Error = &'static str;
    fn try_from(parent: NSViewController) -> Result<NCWidgetSearchViewController, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"NCWidgetSearchViewController").unwrap()) };
        if is_kind_of {
            Ok(NCWidgetSearchViewController(parent.0))
        } else {
            Err("This NSViewController cannot be downcasted to NCWidgetSearchViewController")
        }
    }
}
impl INSResponder for NCWidgetSearchViewController {}
impl PNSCoding for NCWidgetSearchViewController {}
impl INSObject for NCWidgetSearchViewController {}
impl PNSObject for NCWidgetSearchViewController {}
impl INCWidgetSearchViewController for NCWidgetSearchViewController {}
pub trait INCWidgetSearchViewController: Sized + std::ops::Deref {
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
    unsafe fn searchResults(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, searchResults)
    }
    unsafe fn setSearchResults_(&self, searchResults: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSearchResults : searchResults)
    }
    unsafe fn searchDescription(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, searchDescription)
    }
    unsafe fn setSearchDescription_(&self, searchDescription: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSearchDescription : searchDescription)
    }
    unsafe fn searchResultsPlaceholderString(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, searchResultsPlaceholderString)
    }
    unsafe fn setSearchResultsPlaceholderString_(&self, searchResultsPlaceholderString: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSearchResultsPlaceholderString : searchResultsPlaceholderString)
    }
    unsafe fn searchResultKeyPath(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, searchResultKeyPath)
    }
    unsafe fn setSearchResultKeyPath_(&self, searchResultKeyPath: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSearchResultKeyPath : searchResultKeyPath)
    }
}

unsafe impl objc2::encode::RefEncode for NCWidgetController {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for NCWidgetController {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for NCWidgetListViewController {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for NCWidgetListViewController {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for NCWidgetSearchViewController {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for NCWidgetSearchViewController {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
