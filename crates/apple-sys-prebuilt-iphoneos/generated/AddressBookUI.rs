#[allow(unused_imports)]
use crate::objc::*;
#[allow(unused_imports)]
use crate::AddressBook::*;
#[allow(unused_imports)]
use crate::CoreFoundation::*;
#[allow(unused_imports)]
use crate::Foundation::*;

#[allow(unused_imports)]
use objc2::msg_send;
pub type ABPropertyID = i32;
pub type ABAddressBookRef = CFTypeRef;
pub type ABMultiValueIdentifier = i32;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct ABPeoplePickerNavigationController(pub id);
impl std::ops::Deref for ABPeoplePickerNavigationController {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for ABPeoplePickerNavigationController {}
impl ABPeoplePickerNavigationController {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"ABPeoplePickerNavigationController").unwrap(), alloc) })
    }
}
impl PNSCoding for ABPeoplePickerNavigationController {}
impl std::convert::TryFrom<NSObject> for ABPeoplePickerNavigationController {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<ABPeoplePickerNavigationController, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"ABPeoplePickerNavigationController").unwrap())
        };
        if is_kind_of {
            Ok(ABPeoplePickerNavigationController(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to ABPeoplePickerNavigationController")
        }
    }
}
impl IABPeoplePickerNavigationController for ABPeoplePickerNavigationController {}
pub trait IABPeoplePickerNavigationController: Sized + std::ops::Deref {
    unsafe fn peoplePickerDelegate(&self) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, peoplePickerDelegate)
    }
    unsafe fn setPeoplePickerDelegate_(&self, peoplePickerDelegate: *mut u64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPeoplePickerDelegate : peoplePickerDelegate)
    }
    unsafe fn displayedProperties(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, displayedProperties)
    }
    unsafe fn setDisplayedProperties_(&self, displayedProperties: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDisplayedProperties : displayedProperties)
    }
    unsafe fn addressBook(&self) -> ABAddressBookRef
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, addressBook)
    }
    unsafe fn setAddressBook_(&self, addressBook: ABAddressBookRef)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAddressBook : addressBook)
    }
    unsafe fn predicateForEnablingPerson(&self) -> NSPredicate
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, predicateForEnablingPerson)
    }
    unsafe fn setPredicateForEnablingPerson_(&self, predicateForEnablingPerson: NSPredicate)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPredicateForEnablingPerson : predicateForEnablingPerson)
    }
    unsafe fn predicateForSelectionOfPerson(&self) -> NSPredicate
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, predicateForSelectionOfPerson)
    }
    unsafe fn setPredicateForSelectionOfPerson_(&self, predicateForSelectionOfPerson: NSPredicate)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPredicateForSelectionOfPerson : predicateForSelectionOfPerson)
    }
    unsafe fn predicateForSelectionOfProperty(&self) -> NSPredicate
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, predicateForSelectionOfProperty)
    }
    unsafe fn setPredicateForSelectionOfProperty_(
        &self,
        predicateForSelectionOfProperty: NSPredicate,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPredicateForSelectionOfProperty : predicateForSelectionOfProperty)
    }
}
pub trait PABPeoplePickerNavigationControllerDelegate: Sized + std::ops::Deref {
    unsafe fn peoplePickerNavigationController_didSelectPerson_(
        &self,
        peoplePicker: ABPeoplePickerNavigationController,
        person: ABRecordRef,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, peoplePickerNavigationController : peoplePicker, didSelectPerson : person)
    }
    unsafe fn peoplePickerNavigationController_didSelectPerson_property_identifier_(
        &self,
        peoplePicker: ABPeoplePickerNavigationController,
        person: ABRecordRef,
        property: ABPropertyID,
        identifier: ABMultiValueIdentifier,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, peoplePickerNavigationController : peoplePicker, didSelectPerson : person, property : property, identifier : identifier)
    }
    unsafe fn peoplePickerNavigationControllerDidCancel_(
        &self,
        peoplePicker: ABPeoplePickerNavigationController,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, peoplePickerNavigationControllerDidCancel : peoplePicker)
    }
    unsafe fn peoplePickerNavigationController_shouldContinueAfterSelectingPerson_(
        &self,
        peoplePicker: ABPeoplePickerNavigationController,
        person: ABRecordRef,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, peoplePickerNavigationController : peoplePicker, shouldContinueAfterSelectingPerson : person)
    }
    unsafe fn peoplePickerNavigationController_shouldContinueAfterSelectingPerson_property_identifier_(
        &self,
        peoplePicker: ABPeoplePickerNavigationController,
        person: ABRecordRef,
        property: ABPropertyID,
        identifier: ABMultiValueIdentifier,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, peoplePickerNavigationController : peoplePicker, shouldContinueAfterSelectingPerson : person, property : property, identifier : identifier)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct ABPersonViewController(pub id);
impl std::ops::Deref for ABPersonViewController {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for ABPersonViewController {}
impl ABPersonViewController {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"ABPersonViewController").unwrap(), alloc) })
    }
}
impl PNSCoding for ABPersonViewController {}
impl std::convert::TryFrom<NSObject> for ABPersonViewController {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<ABPersonViewController, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"ABPersonViewController").unwrap()) };
        if is_kind_of {
            Ok(ABPersonViewController(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to ABPersonViewController")
        }
    }
}
impl IABPersonViewController for ABPersonViewController {}
pub trait IABPersonViewController: Sized + std::ops::Deref {
    unsafe fn setHighlightedItemForProperty_withIdentifier_(
        &self,
        property: ABPropertyID,
        identifier: ABMultiValueIdentifier,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setHighlightedItemForProperty : property, withIdentifier : identifier)
    }
    unsafe fn personViewDelegate(&self) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, personViewDelegate)
    }
    unsafe fn setPersonViewDelegate_(&self, personViewDelegate: *mut u64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPersonViewDelegate : personViewDelegate)
    }
    unsafe fn addressBook(&self) -> ABAddressBookRef
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, addressBook)
    }
    unsafe fn setAddressBook_(&self, addressBook: ABAddressBookRef)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAddressBook : addressBook)
    }
    unsafe fn displayedPerson(&self) -> ABRecordRef
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, displayedPerson)
    }
    unsafe fn setDisplayedPerson_(&self, displayedPerson: ABRecordRef)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDisplayedPerson : displayedPerson)
    }
    unsafe fn displayedProperties(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, displayedProperties)
    }
    unsafe fn setDisplayedProperties_(&self, displayedProperties: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDisplayedProperties : displayedProperties)
    }
    unsafe fn allowsEditing(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, allowsEditing)
    }
    unsafe fn setAllowsEditing_(&self, allowsEditing: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAllowsEditing : allowsEditing)
    }
    unsafe fn allowsActions(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, allowsActions)
    }
    unsafe fn setAllowsActions_(&self, allowsActions: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAllowsActions : allowsActions)
    }
    unsafe fn shouldShowLinkedPeople(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, shouldShowLinkedPeople)
    }
    unsafe fn setShouldShowLinkedPeople_(&self, shouldShowLinkedPeople: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setShouldShowLinkedPeople : shouldShowLinkedPeople)
    }
}
pub trait PABPersonViewControllerDelegate: Sized + std::ops::Deref {
    unsafe fn personViewController_shouldPerformDefaultActionForPerson_property_identifier_(
        &self,
        personViewController: ABPersonViewController,
        person: ABRecordRef,
        property: ABPropertyID,
        identifier: ABMultiValueIdentifier,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, personViewController : personViewController, shouldPerformDefaultActionForPerson : person, property : property, identifier : identifier)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct ABNewPersonViewController(pub id);
impl std::ops::Deref for ABNewPersonViewController {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for ABNewPersonViewController {}
impl ABNewPersonViewController {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"ABNewPersonViewController").unwrap(), alloc) })
    }
}
impl PNSCoding for ABNewPersonViewController {}
impl std::convert::TryFrom<NSObject> for ABNewPersonViewController {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<ABNewPersonViewController, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"ABNewPersonViewController").unwrap()) };
        if is_kind_of {
            Ok(ABNewPersonViewController(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to ABNewPersonViewController")
        }
    }
}
impl IABNewPersonViewController for ABNewPersonViewController {}
pub trait IABNewPersonViewController: Sized + std::ops::Deref {
    unsafe fn newPersonViewDelegate(&self) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, newPersonViewDelegate)
    }
    unsafe fn setNewPersonViewDelegate_(&self, newPersonViewDelegate: *mut u64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setNewPersonViewDelegate : newPersonViewDelegate)
    }
    unsafe fn addressBook(&self) -> ABAddressBookRef
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, addressBook)
    }
    unsafe fn setAddressBook_(&self, addressBook: ABAddressBookRef)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAddressBook : addressBook)
    }
    unsafe fn displayedPerson(&self) -> ABRecordRef
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, displayedPerson)
    }
    unsafe fn setDisplayedPerson_(&self, displayedPerson: ABRecordRef)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDisplayedPerson : displayedPerson)
    }
    unsafe fn parentGroup(&self) -> ABRecordRef
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, parentGroup)
    }
    unsafe fn setParentGroup_(&self, parentGroup: ABRecordRef)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setParentGroup : parentGroup)
    }
}
pub trait PABNewPersonViewControllerDelegate: Sized + std::ops::Deref {
    unsafe fn newPersonViewController_didCompleteWithNewPerson_(
        &self,
        newPersonView: ABNewPersonViewController,
        person: ABRecordRef,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, newPersonViewController : newPersonView, didCompleteWithNewPerson : person)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct ABUnknownPersonViewController(pub id);
impl std::ops::Deref for ABUnknownPersonViewController {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for ABUnknownPersonViewController {}
impl ABUnknownPersonViewController {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"ABUnknownPersonViewController").unwrap(), alloc) })
    }
}
impl PNSCoding for ABUnknownPersonViewController {}
impl std::convert::TryFrom<NSObject> for ABUnknownPersonViewController {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<ABUnknownPersonViewController, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"ABUnknownPersonViewController").unwrap())
        };
        if is_kind_of {
            Ok(ABUnknownPersonViewController(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to ABUnknownPersonViewController")
        }
    }
}
impl IABUnknownPersonViewController for ABUnknownPersonViewController {}
pub trait IABUnknownPersonViewController: Sized + std::ops::Deref {
    unsafe fn unknownPersonViewDelegate(&self) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, unknownPersonViewDelegate)
    }
    unsafe fn setUnknownPersonViewDelegate_(&self, unknownPersonViewDelegate: *mut u64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setUnknownPersonViewDelegate : unknownPersonViewDelegate)
    }
    unsafe fn addressBook(&self) -> ABAddressBookRef
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, addressBook)
    }
    unsafe fn setAddressBook_(&self, addressBook: ABAddressBookRef)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAddressBook : addressBook)
    }
    unsafe fn displayedPerson(&self) -> ABRecordRef
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, displayedPerson)
    }
    unsafe fn setDisplayedPerson_(&self, displayedPerson: ABRecordRef)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDisplayedPerson : displayedPerson)
    }
    unsafe fn alternateName(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, alternateName)
    }
    unsafe fn setAlternateName_(&self, alternateName: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAlternateName : alternateName)
    }
    unsafe fn message(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, message)
    }
    unsafe fn setMessage_(&self, message: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMessage : message)
    }
    unsafe fn allowsActions(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, allowsActions)
    }
    unsafe fn setAllowsActions_(&self, allowsActions: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAllowsActions : allowsActions)
    }
    unsafe fn allowsAddingToAddressBook(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, allowsAddingToAddressBook)
    }
    unsafe fn setAllowsAddingToAddressBook_(&self, allowsAddingToAddressBook: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAllowsAddingToAddressBook : allowsAddingToAddressBook)
    }
}
pub trait PABUnknownPersonViewControllerDelegate: Sized + std::ops::Deref {
    unsafe fn unknownPersonViewController_didResolveToPerson_(
        &self,
        unknownCardViewController: ABUnknownPersonViewController,
        person: ABRecordRef,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, unknownPersonViewController : unknownCardViewController, didResolveToPerson : person)
    }
    unsafe fn unknownPersonViewController_shouldPerformDefaultActionForPerson_property_identifier_(
        &self,
        personViewController: ABUnknownPersonViewController,
        person: ABRecordRef,
        property: ABPropertyID,
        identifier: ABMultiValueIdentifier,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, unknownPersonViewController : personViewController, shouldPerformDefaultActionForPerson : person, property : property, identifier : identifier)
    }
}
pub trait PUIAppearanceContainer: Sized + std::ops::Deref {}
impl PUIAppearanceContainer for ABPeoplePickerNavigationController {}
impl PUIAppearanceContainer for ABPersonViewController {}
impl PUIAppearanceContainer for ABNewPersonViewController {}
impl PUIAppearanceContainer for ABUnknownPersonViewController {}
unsafe extern "C" {
    pub static ABPersonNamePrefixProperty: NSString;
}
unsafe extern "C" {
    pub static ABPersonGivenNameProperty: NSString;
}
unsafe extern "C" {
    pub static ABPersonMiddleNameProperty: NSString;
}
unsafe extern "C" {
    pub static ABPersonFamilyNameProperty: NSString;
}
unsafe extern "C" {
    pub static ABPersonNameSuffixProperty: NSString;
}
unsafe extern "C" {
    pub static ABPersonPreviousFamilyNameProperty: NSString;
}
unsafe extern "C" {
    pub static ABPersonNicknameProperty: NSString;
}
unsafe extern "C" {
    pub static ABPersonPhoneticGivenNameProperty: NSString;
}
unsafe extern "C" {
    pub static ABPersonPhoneticMiddleNameProperty: NSString;
}
unsafe extern "C" {
    pub static ABPersonPhoneticFamilyNameProperty: NSString;
}
unsafe extern "C" {
    pub static ABPersonOrganizationNameProperty: NSString;
}
unsafe extern "C" {
    pub static ABPersonDepartmentNameProperty: NSString;
}
unsafe extern "C" {
    pub static ABPersonJobTitleProperty: NSString;
}
unsafe extern "C" {
    pub static ABPersonBirthdayProperty: NSString;
}
unsafe extern "C" {
    pub static ABPersonNoteProperty: NSString;
}
unsafe extern "C" {
    pub static ABPersonPhoneNumbersProperty: NSString;
}
unsafe extern "C" {
    pub static ABPersonEmailAddressesProperty: NSString;
}
unsafe extern "C" {
    pub static ABPersonUrlAddressesProperty: NSString;
}
unsafe extern "C" {
    pub static ABPersonDatesProperty: NSString;
}
unsafe extern "C" {
    pub static ABPersonInstantMessageAddressesProperty: NSString;
}
unsafe extern "C" {
    pub static ABPersonRelatedNamesProperty: NSString;
}
unsafe extern "C" {
    pub static ABPersonSocialProfilesProperty: NSString;
}
unsafe extern "C" {
    pub static ABPersonPostalAddressesProperty: NSString;
}
unsafe extern "C" {
    pub fn ABCreateStringWithAddressDictionary(
        address: NSDictionary,
        addCountryName: BOOL,
    ) -> NSString;
}

unsafe impl objc2::encode::RefEncode for ABPeoplePickerNavigationController {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for ABPeoplePickerNavigationController {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for ABPersonViewController {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for ABPersonViewController {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for ABNewPersonViewController {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for ABNewPersonViewController {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for ABUnknownPersonViewController {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for ABUnknownPersonViewController {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
