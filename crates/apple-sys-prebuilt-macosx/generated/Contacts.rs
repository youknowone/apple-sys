#[allow(unused_imports)]
use crate::objc::*;
#[allow(unused_imports)]
use crate::CoreFoundation::*;
#[allow(unused_imports)]
use crate::Foundation::*;

#[allow(unused_imports)]
use objc2::msg_send;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CNLabeledValue(pub id);
impl std::ops::Deref for CNLabeledValue {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CNLabeledValue {}
impl CNLabeledValue {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CNLabeledValue").unwrap(), alloc) })
    }
}
impl PNSCopying for CNLabeledValue {}
impl PNSSecureCoding for CNLabeledValue {}
impl INSObject for CNLabeledValue {}
impl PNSObject for CNLabeledValue {}
impl std::convert::TryFrom<NSObject> for CNLabeledValue {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<CNLabeledValue, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CNLabeledValue").unwrap()) };
        if is_kind_of {
            Ok(CNLabeledValue(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to CNLabeledValue")
        }
    }
}
impl<ValueType: 'static> ICNLabeledValue<ValueType> for CNLabeledValue {}
pub trait ICNLabeledValue<ValueType: 'static>: Sized + std::ops::Deref {
    unsafe fn initWithLabel_value_(&self, label: NSString, value: id) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithLabel : label, value : value)
    }
    unsafe fn labeledValueBySettingLabel_(&self, label: NSString) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, labeledValueBySettingLabel : label)
    }
    unsafe fn labeledValueBySettingValue_(&self, value: id) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, labeledValueBySettingValue : value)
    }
    unsafe fn labeledValueBySettingLabel_value_(&self, label: NSString, value: id) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, labeledValueBySettingLabel : label, value : value)
    }
    unsafe fn identifier(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, identifier)
    }
    unsafe fn label(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, label)
    }
    unsafe fn value(&self) -> id
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, value)
    }
    unsafe fn labeledValueWithLabel_value_(label: NSString, value: id) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CNLabeledValue").unwrap(), labeledValueWithLabel : label, value : value)
    }
    unsafe fn localizedStringForLabel_(label: NSString) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CNLabeledValue").unwrap(), localizedStringForLabel : label)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CNPhoneNumber(pub id);
impl std::ops::Deref for CNPhoneNumber {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CNPhoneNumber {}
impl CNPhoneNumber {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CNPhoneNumber").unwrap(), alloc) })
    }
}
impl PNSCopying for CNPhoneNumber {}
impl PNSSecureCoding for CNPhoneNumber {}
impl INSObject for CNPhoneNumber {}
impl PNSObject for CNPhoneNumber {}
impl std::convert::TryFrom<NSObject> for CNPhoneNumber {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<CNPhoneNumber, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CNPhoneNumber").unwrap()) };
        if is_kind_of {
            Ok(CNPhoneNumber(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to CNPhoneNumber")
        }
    }
}
impl ICNPhoneNumber for CNPhoneNumber {}
pub trait ICNPhoneNumber: Sized + std::ops::Deref {
    unsafe fn initWithStringValue_(&self, string: NSString) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithStringValue : string)
    }
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn stringValue(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, stringValue)
    }
    unsafe fn phoneNumberWithStringValue_(stringValue: NSString) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CNPhoneNumber").unwrap(), phoneNumberWithStringValue : stringValue)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CNPhoneNumber").unwrap(), new)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CNPostalAddress(pub id);
impl std::ops::Deref for CNPostalAddress {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CNPostalAddress {}
impl CNPostalAddress {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CNPostalAddress").unwrap(), alloc) })
    }
}
impl PNSCopying for CNPostalAddress {}
impl PNSMutableCopying for CNPostalAddress {}
impl PNSSecureCoding for CNPostalAddress {}
impl INSObject for CNPostalAddress {}
impl PNSObject for CNPostalAddress {}
impl std::convert::TryFrom<NSObject> for CNPostalAddress {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<CNPostalAddress, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CNPostalAddress").unwrap()) };
        if is_kind_of {
            Ok(CNPostalAddress(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to CNPostalAddress")
        }
    }
}
impl ICNPostalAddress for CNPostalAddress {}
pub trait ICNPostalAddress: Sized + std::ops::Deref {
    unsafe fn street(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, street)
    }
    unsafe fn subLocality(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, subLocality)
    }
    unsafe fn city(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, city)
    }
    unsafe fn subAdministrativeArea(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, subAdministrativeArea)
    }
    unsafe fn state(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, state)
    }
    unsafe fn postalCode(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, postalCode)
    }
    unsafe fn country(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, country)
    }
    unsafe fn ISOCountryCode(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, ISOCountryCode)
    }
    unsafe fn localizedStringForKey_(key: NSString) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CNPostalAddress").unwrap(), localizedStringForKey : key)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CNContactRelation(pub id);
impl std::ops::Deref for CNContactRelation {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CNContactRelation {}
impl CNContactRelation {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CNContactRelation").unwrap(), alloc) })
    }
}
impl PNSCopying for CNContactRelation {}
impl PNSSecureCoding for CNContactRelation {}
impl INSObject for CNContactRelation {}
impl PNSObject for CNContactRelation {}
impl std::convert::TryFrom<NSObject> for CNContactRelation {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<CNContactRelation, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CNContactRelation").unwrap()) };
        if is_kind_of {
            Ok(CNContactRelation(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to CNContactRelation")
        }
    }
}
impl ICNContactRelation for CNContactRelation {}
pub trait ICNContactRelation: Sized + std::ops::Deref {
    unsafe fn initWithName_(&self, name: NSString) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithName : name)
    }
    unsafe fn name(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, name)
    }
    unsafe fn contactRelationWithName_(name: NSString) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CNContactRelation").unwrap(), contactRelationWithName : name)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CNSocialProfile(pub id);
impl std::ops::Deref for CNSocialProfile {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CNSocialProfile {}
impl CNSocialProfile {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CNSocialProfile").unwrap(), alloc) })
    }
}
impl PNSCopying for CNSocialProfile {}
impl PNSSecureCoding for CNSocialProfile {}
impl INSObject for CNSocialProfile {}
impl PNSObject for CNSocialProfile {}
impl std::convert::TryFrom<NSObject> for CNSocialProfile {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<CNSocialProfile, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CNSocialProfile").unwrap()) };
        if is_kind_of {
            Ok(CNSocialProfile(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to CNSocialProfile")
        }
    }
}
impl ICNSocialProfile for CNSocialProfile {}
pub trait ICNSocialProfile: Sized + std::ops::Deref {
    unsafe fn initWithUrlString_username_userIdentifier_service_(
        &self,
        urlString: NSString,
        username: NSString,
        userIdentifier: NSString,
        service: NSString,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithUrlString : urlString, username : username, userIdentifier : userIdentifier, service : service)
    }
    unsafe fn urlString(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, urlString)
    }
    unsafe fn username(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, username)
    }
    unsafe fn userIdentifier(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, userIdentifier)
    }
    unsafe fn service(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, service)
    }
    unsafe fn localizedStringForKey_(key: NSString) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CNSocialProfile").unwrap(), localizedStringForKey : key)
    }
    unsafe fn localizedStringForService_(service: NSString) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CNSocialProfile").unwrap(), localizedStringForService : service)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CNInstantMessageAddress(pub id);
impl std::ops::Deref for CNInstantMessageAddress {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CNInstantMessageAddress {}
impl CNInstantMessageAddress {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CNInstantMessageAddress").unwrap(), alloc) })
    }
}
impl PNSCopying for CNInstantMessageAddress {}
impl PNSSecureCoding for CNInstantMessageAddress {}
impl INSObject for CNInstantMessageAddress {}
impl PNSObject for CNInstantMessageAddress {}
impl std::convert::TryFrom<NSObject> for CNInstantMessageAddress {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<CNInstantMessageAddress, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CNInstantMessageAddress").unwrap()) };
        if is_kind_of {
            Ok(CNInstantMessageAddress(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to CNInstantMessageAddress")
        }
    }
}
impl ICNInstantMessageAddress for CNInstantMessageAddress {}
pub trait ICNInstantMessageAddress: Sized + std::ops::Deref {
    unsafe fn initWithUsername_service_(
        &self,
        username: NSString,
        service: NSString,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithUsername : username, service : service)
    }
    unsafe fn username(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, username)
    }
    unsafe fn service(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, service)
    }
    unsafe fn localizedStringForKey_(key: NSString) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CNInstantMessageAddress").unwrap(), localizedStringForKey : key)
    }
    unsafe fn localizedStringForService_(service: NSString) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CNInstantMessageAddress").unwrap(), localizedStringForService : service)
    }
}
pub type CNContactType = NSInteger;
pub type CNContactSortOrder = NSInteger;
pub trait PCNKeyDescriptor: Sized + std::ops::Deref {}
pub trait NSString_Contacts: Sized + std::ops::Deref {}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CNContact(pub id);
impl std::ops::Deref for CNContact {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CNContact {}
impl CNContact {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CNContact").unwrap(), alloc) })
    }
}
impl PNSCopying for CNContact {}
impl PNSMutableCopying for CNContact {}
impl PNSSecureCoding for CNContact {}
impl INSObject for CNContact {}
impl PNSObject for CNContact {}
impl std::convert::TryFrom<NSObject> for CNContact {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<CNContact, Self::Error> {
        let is_kind_of: bool = unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CNContact").unwrap()) };
        if is_kind_of {
            Ok(CNContact(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to CNContact")
        }
    }
}
impl ICNContact for CNContact {}
pub trait ICNContact: Sized + std::ops::Deref {
    unsafe fn isKeyAvailable_(&self, key: NSString) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, isKeyAvailable : key)
    }
    unsafe fn areKeysAvailable_(&self, keyDescriptors: NSArray) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, areKeysAvailable : keyDescriptors)
    }
    unsafe fn isUnifiedWithContactWithIdentifier_(&self, contactIdentifier: NSString) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, isUnifiedWithContactWithIdentifier : contactIdentifier)
    }
    unsafe fn identifier(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, identifier)
    }
    unsafe fn contactType(&self) -> CNContactType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, contactType)
    }
    unsafe fn namePrefix(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, namePrefix)
    }
    unsafe fn givenName(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, givenName)
    }
    unsafe fn middleName(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, middleName)
    }
    unsafe fn familyName(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, familyName)
    }
    unsafe fn previousFamilyName(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, previousFamilyName)
    }
    unsafe fn nameSuffix(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, nameSuffix)
    }
    unsafe fn nickname(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, nickname)
    }
    unsafe fn organizationName(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, organizationName)
    }
    unsafe fn departmentName(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, departmentName)
    }
    unsafe fn jobTitle(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, jobTitle)
    }
    unsafe fn phoneticGivenName(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, phoneticGivenName)
    }
    unsafe fn phoneticMiddleName(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, phoneticMiddleName)
    }
    unsafe fn phoneticFamilyName(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, phoneticFamilyName)
    }
    unsafe fn phoneticOrganizationName(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, phoneticOrganizationName)
    }
    unsafe fn note(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, note)
    }
    unsafe fn imageData(&self) -> NSData
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, imageData)
    }
    unsafe fn thumbnailImageData(&self) -> NSData
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, thumbnailImageData)
    }
    unsafe fn imageDataAvailable(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, imageDataAvailable)
    }
    unsafe fn phoneNumbers(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, phoneNumbers)
    }
    unsafe fn emailAddresses(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, emailAddresses)
    }
    unsafe fn postalAddresses(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, postalAddresses)
    }
    unsafe fn urlAddresses(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, urlAddresses)
    }
    unsafe fn contactRelations(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, contactRelations)
    }
    unsafe fn socialProfiles(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, socialProfiles)
    }
    unsafe fn instantMessageAddresses(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, instantMessageAddresses)
    }
    unsafe fn birthday(&self) -> NSDateComponents
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, birthday)
    }
    unsafe fn nonGregorianBirthday(&self) -> NSDateComponents
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, nonGregorianBirthday)
    }
    unsafe fn dates(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, dates)
    }
    unsafe fn localizedStringForKey_(key: NSString) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CNContact").unwrap(), localizedStringForKey : key)
    }
    unsafe fn comparatorForNameSortOrder_(sortOrder: CNContactSortOrder) -> NSComparator
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CNContact").unwrap(), comparatorForNameSortOrder : sortOrder)
    }
    unsafe fn descriptorForAllComparatorKeys() -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CNContact").unwrap(), descriptorForAllComparatorKeys)
    }
}
pub type CNEntityType = NSInteger;
pub type CNAuthorizationStatus = NSInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CNContactStore(pub id);
impl std::ops::Deref for CNContactStore {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CNContactStore {}
impl CNContactStore {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CNContactStore").unwrap(), alloc) })
    }
}
impl INSObject for CNContactStore {}
impl PNSObject for CNContactStore {}
impl std::convert::TryFrom<NSObject> for CNContactStore {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<CNContactStore, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CNContactStore").unwrap()) };
        if is_kind_of {
            Ok(CNContactStore(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to CNContactStore")
        }
    }
}
impl ICNContactStore for CNContactStore {}
pub trait ICNContactStore: Sized + std::ops::Deref {
    unsafe fn requestAccessForEntityType_completionHandler_(
        &self,
        entityType: CNEntityType,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, requestAccessForEntityType : entityType, completionHandler : completionHandler)
    }
    unsafe fn unifiedContactsMatchingPredicate_keysToFetch_error_(
        &self,
        predicate: NSPredicate,
        keys: NSArray,
        error: *mut NSError,
    ) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, unifiedContactsMatchingPredicate : predicate, keysToFetch : keys, error : error)
    }
    unsafe fn unifiedContactWithIdentifier_keysToFetch_error_(
        &self,
        identifier: NSString,
        keys: NSArray,
        error: *mut NSError,
    ) -> CNContact
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, unifiedContactWithIdentifier : identifier, keysToFetch : keys, error : error)
    }
    unsafe fn unifiedMeContactWithKeysToFetch_error_(
        &self,
        keys: NSArray,
        error: *mut NSError,
    ) -> CNContact
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, unifiedMeContactWithKeysToFetch : keys, error : error)
    }
    unsafe fn enumeratorForContactFetchRequest_error_(
        &self,
        request: CNContactFetchRequest,
        error: *mut NSError,
    ) -> CNFetchResult
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, enumeratorForContactFetchRequest : request, error : error)
    }
    unsafe fn enumeratorForChangeHistoryFetchRequest_error_(
        &self,
        request: CNChangeHistoryFetchRequest,
        error: *mut NSError,
    ) -> CNFetchResult
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, enumeratorForChangeHistoryFetchRequest : request, error : error)
    }
    unsafe fn enumerateContactsWithFetchRequest_error_usingBlock_(
        &self,
        fetchRequest: CNContactFetchRequest,
        error: *mut NSError,
        block: *mut ::std::os::raw::c_void,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, enumerateContactsWithFetchRequest : fetchRequest, error : error, usingBlock : block)
    }
    unsafe fn groupsMatchingPredicate_error_(
        &self,
        predicate: NSPredicate,
        error: *mut NSError,
    ) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, groupsMatchingPredicate : predicate, error : error)
    }
    unsafe fn containersMatchingPredicate_error_(
        &self,
        predicate: NSPredicate,
        error: *mut NSError,
    ) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, containersMatchingPredicate : predicate, error : error)
    }
    unsafe fn executeSaveRequest_error_(
        &self,
        saveRequest: CNSaveRequest,
        error: *mut NSError,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, executeSaveRequest : saveRequest, error : error)
    }
    unsafe fn defaultContainerIdentifier(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, defaultContainerIdentifier)
    }
    unsafe fn currentHistoryToken(&self) -> NSData
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, currentHistoryToken)
    }
    unsafe fn authorizationStatusForEntityType_(entityType: CNEntityType) -> CNAuthorizationStatus
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CNContactStore").unwrap(), authorizationStatusForEntityType : entityType)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CNFetchRequest(pub id);
impl std::ops::Deref for CNFetchRequest {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CNFetchRequest {}
impl CNFetchRequest {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CNFetchRequest").unwrap(), alloc) })
    }
}
impl INSObject for CNFetchRequest {}
impl PNSObject for CNFetchRequest {}
impl std::convert::TryFrom<NSObject> for CNFetchRequest {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<CNFetchRequest, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CNFetchRequest").unwrap()) };
        if is_kind_of {
            Ok(CNFetchRequest(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to CNFetchRequest")
        }
    }
}
impl ICNFetchRequest for CNFetchRequest {}
pub trait ICNFetchRequest: Sized + std::ops::Deref {}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CNContactFetchRequest(pub id);
impl std::ops::Deref for CNContactFetchRequest {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CNContactFetchRequest {}
impl CNContactFetchRequest {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CNContactFetchRequest").unwrap(), alloc) })
    }
}
impl PNSSecureCoding for CNContactFetchRequest {}
impl ICNFetchRequest for CNContactFetchRequest {}
impl From<CNContactFetchRequest> for CNFetchRequest {
    fn from(child: CNContactFetchRequest) -> CNFetchRequest {
        CNFetchRequest(child.0)
    }
}
impl std::convert::TryFrom<CNFetchRequest> for CNContactFetchRequest {
    type Error = &'static str;
    fn try_from(parent: CNFetchRequest) -> Result<CNContactFetchRequest, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CNContactFetchRequest").unwrap()) };
        if is_kind_of {
            Ok(CNContactFetchRequest(parent.0))
        } else {
            Err("This CNFetchRequest cannot be downcasted to CNContactFetchRequest")
        }
    }
}
impl INSObject for CNContactFetchRequest {}
impl PNSObject for CNContactFetchRequest {}
impl ICNContactFetchRequest for CNContactFetchRequest {}
pub trait ICNContactFetchRequest: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn initWithKeysToFetch_(&self, keysToFetch: NSArray) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithKeysToFetch : keysToFetch)
    }
    unsafe fn predicate(&self) -> NSPredicate
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, predicate)
    }
    unsafe fn setPredicate_(&self, predicate: NSPredicate)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPredicate : predicate)
    }
    unsafe fn keysToFetch(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, keysToFetch)
    }
    unsafe fn setKeysToFetch_(&self, keysToFetch: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setKeysToFetch : keysToFetch)
    }
    unsafe fn mutableObjects(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, mutableObjects)
    }
    unsafe fn setMutableObjects_(&self, mutableObjects: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMutableObjects : mutableObjects)
    }
    unsafe fn unifyResults(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, unifyResults)
    }
    unsafe fn setUnifyResults_(&self, unifyResults: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setUnifyResults : unifyResults)
    }
    unsafe fn sortOrder(&self) -> CNContactSortOrder
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, sortOrder)
    }
    unsafe fn setSortOrder_(&self, sortOrder: CNContactSortOrder)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSortOrder : sortOrder)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CNContactFetchRequest").unwrap(), new)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CNFetchResult(pub id);
impl std::ops::Deref for CNFetchResult {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CNFetchResult {}
impl CNFetchResult {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CNFetchResult").unwrap(), alloc) })
    }
}
impl INSObject for CNFetchResult {}
impl PNSObject for CNFetchResult {}
impl std::convert::TryFrom<NSObject> for CNFetchResult {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<CNFetchResult, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CNFetchResult").unwrap()) };
        if is_kind_of {
            Ok(CNFetchResult(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to CNFetchResult")
        }
    }
}
impl<ValueType: 'static> ICNFetchResult<ValueType> for CNFetchResult {}
pub trait ICNFetchResult<ValueType: 'static>: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn value(&self) -> id
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, value)
    }
    unsafe fn currentHistoryToken(&self) -> NSData
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, currentHistoryToken)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CNFetchResult").unwrap(), new)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CNSaveRequest(pub id);
impl std::ops::Deref for CNSaveRequest {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CNSaveRequest {}
impl CNSaveRequest {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CNSaveRequest").unwrap(), alloc) })
    }
}
impl INSObject for CNSaveRequest {}
impl PNSObject for CNSaveRequest {}
impl std::convert::TryFrom<NSObject> for CNSaveRequest {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<CNSaveRequest, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CNSaveRequest").unwrap()) };
        if is_kind_of {
            Ok(CNSaveRequest(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to CNSaveRequest")
        }
    }
}
impl ICNSaveRequest for CNSaveRequest {}
pub trait ICNSaveRequest: Sized + std::ops::Deref {
    unsafe fn addContact_toContainerWithIdentifier_(
        &self,
        contact: CNMutableContact,
        identifier: NSString,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, addContact : contact, toContainerWithIdentifier : identifier)
    }
    unsafe fn updateContact_(&self, contact: CNMutableContact)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, updateContact : contact)
    }
    unsafe fn deleteContact_(&self, contact: CNMutableContact)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, deleteContact : contact)
    }
    unsafe fn addGroup_toContainerWithIdentifier_(
        &self,
        group: CNMutableGroup,
        identifier: NSString,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, addGroup : group, toContainerWithIdentifier : identifier)
    }
    unsafe fn updateGroup_(&self, group: CNMutableGroup)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, updateGroup : group)
    }
    unsafe fn deleteGroup_(&self, group: CNMutableGroup)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, deleteGroup : group)
    }
    unsafe fn addSubgroup_toGroup_(&self, subgroup: CNGroup, group: CNGroup)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, addSubgroup : subgroup, toGroup : group)
    }
    unsafe fn removeSubgroup_fromGroup_(&self, subgroup: CNGroup, group: CNGroup)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, removeSubgroup : subgroup, fromGroup : group)
    }
    unsafe fn addMember_toGroup_(&self, contact: CNContact, group: CNGroup)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, addMember : contact, toGroup : group)
    }
    unsafe fn removeMember_fromGroup_(&self, contact: CNContact, group: CNGroup)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, removeMember : contact, fromGroup : group)
    }
    unsafe fn transactionAuthor(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, transactionAuthor)
    }
    unsafe fn setTransactionAuthor_(&self, transactionAuthor: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setTransactionAuthor : transactionAuthor)
    }
    unsafe fn shouldRefetchContacts(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, shouldRefetchContacts)
    }
    unsafe fn setShouldRefetchContacts_(&self, shouldRefetchContacts: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setShouldRefetchContacts : shouldRefetchContacts)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CNChangeHistoryFetchRequest(pub id);
impl std::ops::Deref for CNChangeHistoryFetchRequest {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CNChangeHistoryFetchRequest {}
impl CNChangeHistoryFetchRequest {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CNChangeHistoryFetchRequest").unwrap(), alloc) })
    }
}
impl PNSSecureCoding for CNChangeHistoryFetchRequest {}
impl ICNFetchRequest for CNChangeHistoryFetchRequest {}
impl std::convert::TryFrom<CNFetchRequest> for CNChangeHistoryFetchRequest {
    type Error = &'static str;
    fn try_from(parent: CNFetchRequest) -> Result<CNChangeHistoryFetchRequest, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CNChangeHistoryFetchRequest").unwrap()) };
        if is_kind_of {
            Ok(CNChangeHistoryFetchRequest(parent.0))
        } else {
            Err("This CNFetchRequest cannot be downcasted to CNChangeHistoryFetchRequest")
        }
    }
}
impl INSObject for CNChangeHistoryFetchRequest {}
impl PNSObject for CNChangeHistoryFetchRequest {}
impl ICNChangeHistoryFetchRequest for CNChangeHistoryFetchRequest {}
pub trait ICNChangeHistoryFetchRequest: Sized + std::ops::Deref {
    unsafe fn startingToken(&self) -> NSData
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, startingToken)
    }
    unsafe fn setStartingToken_(&self, startingToken: NSData)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setStartingToken : startingToken)
    }
    unsafe fn additionalContactKeyDescriptors(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, additionalContactKeyDescriptors)
    }
    unsafe fn setAdditionalContactKeyDescriptors_(&self, additionalContactKeyDescriptors: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAdditionalContactKeyDescriptors : additionalContactKeyDescriptors)
    }
    unsafe fn shouldUnifyResults(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, shouldUnifyResults)
    }
    unsafe fn setShouldUnifyResults_(&self, shouldUnifyResults: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setShouldUnifyResults : shouldUnifyResults)
    }
    unsafe fn mutableObjects(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, mutableObjects)
    }
    unsafe fn setMutableObjects_(&self, mutableObjects: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMutableObjects : mutableObjects)
    }
    unsafe fn includeGroupChanges(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, includeGroupChanges)
    }
    unsafe fn setIncludeGroupChanges_(&self, includeGroupChanges: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setIncludeGroupChanges : includeGroupChanges)
    }
    unsafe fn excludedTransactionAuthors(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, excludedTransactionAuthors)
    }
    unsafe fn setExcludedTransactionAuthors_(&self, excludedTransactionAuthors: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setExcludedTransactionAuthors : excludedTransactionAuthors)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CNChangeHistoryEvent(pub id);
impl std::ops::Deref for CNChangeHistoryEvent {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CNChangeHistoryEvent {}
impl CNChangeHistoryEvent {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CNChangeHistoryEvent").unwrap(), alloc) })
    }
}
impl PNSCopying for CNChangeHistoryEvent {}
impl PNSSecureCoding for CNChangeHistoryEvent {}
impl INSObject for CNChangeHistoryEvent {}
impl PNSObject for CNChangeHistoryEvent {}
impl std::convert::TryFrom<NSObject> for CNChangeHistoryEvent {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<CNChangeHistoryEvent, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CNChangeHistoryEvent").unwrap()) };
        if is_kind_of {
            Ok(CNChangeHistoryEvent(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to CNChangeHistoryEvent")
        }
    }
}
impl ICNChangeHistoryEvent for CNChangeHistoryEvent {}
pub trait ICNChangeHistoryEvent: Sized + std::ops::Deref {
    unsafe fn acceptEventVisitor_(&self, visitor: *mut u64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, acceptEventVisitor : visitor)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CNChangeHistoryDropEverythingEvent(pub id);
impl std::ops::Deref for CNChangeHistoryDropEverythingEvent {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CNChangeHistoryDropEverythingEvent {}
impl CNChangeHistoryDropEverythingEvent {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CNChangeHistoryDropEverythingEvent").unwrap(), alloc) })
    }
}
impl ICNChangeHistoryEvent for CNChangeHistoryDropEverythingEvent {}
impl PNSCopying for CNChangeHistoryDropEverythingEvent {}
impl PNSSecureCoding for CNChangeHistoryDropEverythingEvent {}
impl From<CNChangeHistoryDropEverythingEvent> for CNChangeHistoryEvent {
    fn from(child: CNChangeHistoryDropEverythingEvent) -> CNChangeHistoryEvent {
        CNChangeHistoryEvent(child.0)
    }
}
impl std::convert::TryFrom<CNChangeHistoryEvent> for CNChangeHistoryDropEverythingEvent {
    type Error = &'static str;
    fn try_from(
        parent: CNChangeHistoryEvent,
    ) -> Result<CNChangeHistoryDropEverythingEvent, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CNChangeHistoryDropEverythingEvent").unwrap())
        };
        if is_kind_of {
            Ok(CNChangeHistoryDropEverythingEvent(parent.0))
        } else {
            Err ("This CNChangeHistoryEvent cannot be downcasted to CNChangeHistoryDropEverythingEvent" ,)
        }
    }
}
impl INSObject for CNChangeHistoryDropEverythingEvent {}
impl PNSObject for CNChangeHistoryDropEverythingEvent {}
impl ICNChangeHistoryDropEverythingEvent for CNChangeHistoryDropEverythingEvent {}
pub trait ICNChangeHistoryDropEverythingEvent: Sized + std::ops::Deref {}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CNChangeHistoryAddContactEvent(pub id);
impl std::ops::Deref for CNChangeHistoryAddContactEvent {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CNChangeHistoryAddContactEvent {}
impl CNChangeHistoryAddContactEvent {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CNChangeHistoryAddContactEvent").unwrap(), alloc) })
    }
}
impl ICNChangeHistoryEvent for CNChangeHistoryAddContactEvent {}
impl PNSCopying for CNChangeHistoryAddContactEvent {}
impl PNSSecureCoding for CNChangeHistoryAddContactEvent {}
impl std::convert::TryFrom<CNChangeHistoryEvent> for CNChangeHistoryAddContactEvent {
    type Error = &'static str;
    fn try_from(
        parent: CNChangeHistoryEvent,
    ) -> Result<CNChangeHistoryAddContactEvent, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CNChangeHistoryAddContactEvent").unwrap())
        };
        if is_kind_of {
            Ok(CNChangeHistoryAddContactEvent(parent.0))
        } else {
            Err("This CNChangeHistoryEvent cannot be downcasted to CNChangeHistoryAddContactEvent")
        }
    }
}
impl INSObject for CNChangeHistoryAddContactEvent {}
impl PNSObject for CNChangeHistoryAddContactEvent {}
impl ICNChangeHistoryAddContactEvent for CNChangeHistoryAddContactEvent {}
pub trait ICNChangeHistoryAddContactEvent: Sized + std::ops::Deref {
    unsafe fn contact(&self) -> CNContact
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, contact)
    }
    unsafe fn containerIdentifier(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, containerIdentifier)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CNChangeHistoryUpdateContactEvent(pub id);
impl std::ops::Deref for CNChangeHistoryUpdateContactEvent {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CNChangeHistoryUpdateContactEvent {}
impl CNChangeHistoryUpdateContactEvent {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CNChangeHistoryUpdateContactEvent").unwrap(), alloc) })
    }
}
impl ICNChangeHistoryEvent for CNChangeHistoryUpdateContactEvent {}
impl PNSCopying for CNChangeHistoryUpdateContactEvent {}
impl PNSSecureCoding for CNChangeHistoryUpdateContactEvent {}
impl std::convert::TryFrom<CNChangeHistoryEvent> for CNChangeHistoryUpdateContactEvent {
    type Error = &'static str;
    fn try_from(
        parent: CNChangeHistoryEvent,
    ) -> Result<CNChangeHistoryUpdateContactEvent, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CNChangeHistoryUpdateContactEvent").unwrap())
        };
        if is_kind_of {
            Ok(CNChangeHistoryUpdateContactEvent(parent.0))
        } else {
            Err ("This CNChangeHistoryEvent cannot be downcasted to CNChangeHistoryUpdateContactEvent" ,)
        }
    }
}
impl INSObject for CNChangeHistoryUpdateContactEvent {}
impl PNSObject for CNChangeHistoryUpdateContactEvent {}
impl ICNChangeHistoryUpdateContactEvent for CNChangeHistoryUpdateContactEvent {}
pub trait ICNChangeHistoryUpdateContactEvent: Sized + std::ops::Deref {
    unsafe fn contact(&self) -> CNContact
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, contact)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CNChangeHistoryDeleteContactEvent(pub id);
impl std::ops::Deref for CNChangeHistoryDeleteContactEvent {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CNChangeHistoryDeleteContactEvent {}
impl CNChangeHistoryDeleteContactEvent {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CNChangeHistoryDeleteContactEvent").unwrap(), alloc) })
    }
}
impl ICNChangeHistoryEvent for CNChangeHistoryDeleteContactEvent {}
impl PNSCopying for CNChangeHistoryDeleteContactEvent {}
impl PNSSecureCoding for CNChangeHistoryDeleteContactEvent {}
impl std::convert::TryFrom<CNChangeHistoryEvent> for CNChangeHistoryDeleteContactEvent {
    type Error = &'static str;
    fn try_from(
        parent: CNChangeHistoryEvent,
    ) -> Result<CNChangeHistoryDeleteContactEvent, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CNChangeHistoryDeleteContactEvent").unwrap())
        };
        if is_kind_of {
            Ok(CNChangeHistoryDeleteContactEvent(parent.0))
        } else {
            Err ("This CNChangeHistoryEvent cannot be downcasted to CNChangeHistoryDeleteContactEvent" ,)
        }
    }
}
impl INSObject for CNChangeHistoryDeleteContactEvent {}
impl PNSObject for CNChangeHistoryDeleteContactEvent {}
impl ICNChangeHistoryDeleteContactEvent for CNChangeHistoryDeleteContactEvent {}
pub trait ICNChangeHistoryDeleteContactEvent: Sized + std::ops::Deref {
    unsafe fn contactIdentifier(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, contactIdentifier)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CNChangeHistoryAddGroupEvent(pub id);
impl std::ops::Deref for CNChangeHistoryAddGroupEvent {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CNChangeHistoryAddGroupEvent {}
impl CNChangeHistoryAddGroupEvent {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CNChangeHistoryAddGroupEvent").unwrap(), alloc) })
    }
}
impl ICNChangeHistoryEvent for CNChangeHistoryAddGroupEvent {}
impl PNSCopying for CNChangeHistoryAddGroupEvent {}
impl PNSSecureCoding for CNChangeHistoryAddGroupEvent {}
impl std::convert::TryFrom<CNChangeHistoryEvent> for CNChangeHistoryAddGroupEvent {
    type Error = &'static str;
    fn try_from(parent: CNChangeHistoryEvent) -> Result<CNChangeHistoryAddGroupEvent, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CNChangeHistoryAddGroupEvent").unwrap()) };
        if is_kind_of {
            Ok(CNChangeHistoryAddGroupEvent(parent.0))
        } else {
            Err("This CNChangeHistoryEvent cannot be downcasted to CNChangeHistoryAddGroupEvent")
        }
    }
}
impl INSObject for CNChangeHistoryAddGroupEvent {}
impl PNSObject for CNChangeHistoryAddGroupEvent {}
impl ICNChangeHistoryAddGroupEvent for CNChangeHistoryAddGroupEvent {}
pub trait ICNChangeHistoryAddGroupEvent: Sized + std::ops::Deref {
    unsafe fn group(&self) -> CNGroup
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, group)
    }
    unsafe fn containerIdentifier(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, containerIdentifier)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CNChangeHistoryUpdateGroupEvent(pub id);
impl std::ops::Deref for CNChangeHistoryUpdateGroupEvent {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CNChangeHistoryUpdateGroupEvent {}
impl CNChangeHistoryUpdateGroupEvent {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CNChangeHistoryUpdateGroupEvent").unwrap(), alloc) })
    }
}
impl ICNChangeHistoryEvent for CNChangeHistoryUpdateGroupEvent {}
impl PNSCopying for CNChangeHistoryUpdateGroupEvent {}
impl PNSSecureCoding for CNChangeHistoryUpdateGroupEvent {}
impl std::convert::TryFrom<CNChangeHistoryEvent> for CNChangeHistoryUpdateGroupEvent {
    type Error = &'static str;
    fn try_from(
        parent: CNChangeHistoryEvent,
    ) -> Result<CNChangeHistoryUpdateGroupEvent, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CNChangeHistoryUpdateGroupEvent").unwrap())
        };
        if is_kind_of {
            Ok(CNChangeHistoryUpdateGroupEvent(parent.0))
        } else {
            Err("This CNChangeHistoryEvent cannot be downcasted to CNChangeHistoryUpdateGroupEvent")
        }
    }
}
impl INSObject for CNChangeHistoryUpdateGroupEvent {}
impl PNSObject for CNChangeHistoryUpdateGroupEvent {}
impl ICNChangeHistoryUpdateGroupEvent for CNChangeHistoryUpdateGroupEvent {}
pub trait ICNChangeHistoryUpdateGroupEvent: Sized + std::ops::Deref {
    unsafe fn group(&self) -> CNGroup
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, group)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CNChangeHistoryDeleteGroupEvent(pub id);
impl std::ops::Deref for CNChangeHistoryDeleteGroupEvent {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CNChangeHistoryDeleteGroupEvent {}
impl CNChangeHistoryDeleteGroupEvent {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CNChangeHistoryDeleteGroupEvent").unwrap(), alloc) })
    }
}
impl ICNChangeHistoryEvent for CNChangeHistoryDeleteGroupEvent {}
impl PNSCopying for CNChangeHistoryDeleteGroupEvent {}
impl PNSSecureCoding for CNChangeHistoryDeleteGroupEvent {}
impl std::convert::TryFrom<CNChangeHistoryEvent> for CNChangeHistoryDeleteGroupEvent {
    type Error = &'static str;
    fn try_from(
        parent: CNChangeHistoryEvent,
    ) -> Result<CNChangeHistoryDeleteGroupEvent, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CNChangeHistoryDeleteGroupEvent").unwrap())
        };
        if is_kind_of {
            Ok(CNChangeHistoryDeleteGroupEvent(parent.0))
        } else {
            Err("This CNChangeHistoryEvent cannot be downcasted to CNChangeHistoryDeleteGroupEvent")
        }
    }
}
impl INSObject for CNChangeHistoryDeleteGroupEvent {}
impl PNSObject for CNChangeHistoryDeleteGroupEvent {}
impl ICNChangeHistoryDeleteGroupEvent for CNChangeHistoryDeleteGroupEvent {}
pub trait ICNChangeHistoryDeleteGroupEvent: Sized + std::ops::Deref {
    unsafe fn groupIdentifier(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, groupIdentifier)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CNChangeHistoryAddMemberToGroupEvent(pub id);
impl std::ops::Deref for CNChangeHistoryAddMemberToGroupEvent {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CNChangeHistoryAddMemberToGroupEvent {}
impl CNChangeHistoryAddMemberToGroupEvent {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CNChangeHistoryAddMemberToGroupEvent").unwrap(), alloc) })
    }
}
impl ICNChangeHistoryEvent for CNChangeHistoryAddMemberToGroupEvent {}
impl PNSCopying for CNChangeHistoryAddMemberToGroupEvent {}
impl PNSSecureCoding for CNChangeHistoryAddMemberToGroupEvent {}
impl std::convert::TryFrom<CNChangeHistoryEvent> for CNChangeHistoryAddMemberToGroupEvent {
    type Error = &'static str;
    fn try_from(
        parent: CNChangeHistoryEvent,
    ) -> Result<CNChangeHistoryAddMemberToGroupEvent, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CNChangeHistoryAddMemberToGroupEvent").unwrap())
        };
        if is_kind_of {
            Ok(CNChangeHistoryAddMemberToGroupEvent(parent.0))
        } else {
            Err ("This CNChangeHistoryEvent cannot be downcasted to CNChangeHistoryAddMemberToGroupEvent" ,)
        }
    }
}
impl INSObject for CNChangeHistoryAddMemberToGroupEvent {}
impl PNSObject for CNChangeHistoryAddMemberToGroupEvent {}
impl ICNChangeHistoryAddMemberToGroupEvent for CNChangeHistoryAddMemberToGroupEvent {}
pub trait ICNChangeHistoryAddMemberToGroupEvent: Sized + std::ops::Deref {
    unsafe fn member(&self) -> CNContact
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, member)
    }
    unsafe fn group(&self) -> CNGroup
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, group)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CNChangeHistoryRemoveMemberFromGroupEvent(pub id);
impl std::ops::Deref for CNChangeHistoryRemoveMemberFromGroupEvent {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CNChangeHistoryRemoveMemberFromGroupEvent {}
impl CNChangeHistoryRemoveMemberFromGroupEvent {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CNChangeHistoryRemoveMemberFromGroupEvent").unwrap(), alloc) })
    }
}
impl ICNChangeHistoryEvent for CNChangeHistoryRemoveMemberFromGroupEvent {}
impl PNSCopying for CNChangeHistoryRemoveMemberFromGroupEvent {}
impl PNSSecureCoding for CNChangeHistoryRemoveMemberFromGroupEvent {}
impl std::convert::TryFrom<CNChangeHistoryEvent> for CNChangeHistoryRemoveMemberFromGroupEvent {
    type Error = &'static str;
    fn try_from(
        parent: CNChangeHistoryEvent,
    ) -> Result<CNChangeHistoryRemoveMemberFromGroupEvent, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CNChangeHistoryRemoveMemberFromGroupEvent").unwrap())
        };
        if is_kind_of {
            Ok(CNChangeHistoryRemoveMemberFromGroupEvent(parent.0))
        } else {
            Err ("This CNChangeHistoryEvent cannot be downcasted to CNChangeHistoryRemoveMemberFromGroupEvent" ,)
        }
    }
}
impl INSObject for CNChangeHistoryRemoveMemberFromGroupEvent {}
impl PNSObject for CNChangeHistoryRemoveMemberFromGroupEvent {}
impl ICNChangeHistoryRemoveMemberFromGroupEvent for CNChangeHistoryRemoveMemberFromGroupEvent {}
pub trait ICNChangeHistoryRemoveMemberFromGroupEvent: Sized + std::ops::Deref {
    unsafe fn member(&self) -> CNContact
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, member)
    }
    unsafe fn group(&self) -> CNGroup
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, group)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CNChangeHistoryAddSubgroupToGroupEvent(pub id);
impl std::ops::Deref for CNChangeHistoryAddSubgroupToGroupEvent {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CNChangeHistoryAddSubgroupToGroupEvent {}
impl CNChangeHistoryAddSubgroupToGroupEvent {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CNChangeHistoryAddSubgroupToGroupEvent").unwrap(), alloc) })
    }
}
impl ICNChangeHistoryEvent for CNChangeHistoryAddSubgroupToGroupEvent {}
impl PNSCopying for CNChangeHistoryAddSubgroupToGroupEvent {}
impl PNSSecureCoding for CNChangeHistoryAddSubgroupToGroupEvent {}
impl std::convert::TryFrom<CNChangeHistoryEvent> for CNChangeHistoryAddSubgroupToGroupEvent {
    type Error = &'static str;
    fn try_from(
        parent: CNChangeHistoryEvent,
    ) -> Result<CNChangeHistoryAddSubgroupToGroupEvent, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CNChangeHistoryAddSubgroupToGroupEvent").unwrap())
        };
        if is_kind_of {
            Ok(CNChangeHistoryAddSubgroupToGroupEvent(parent.0))
        } else {
            Err ("This CNChangeHistoryEvent cannot be downcasted to CNChangeHistoryAddSubgroupToGroupEvent" ,)
        }
    }
}
impl INSObject for CNChangeHistoryAddSubgroupToGroupEvent {}
impl PNSObject for CNChangeHistoryAddSubgroupToGroupEvent {}
impl ICNChangeHistoryAddSubgroupToGroupEvent for CNChangeHistoryAddSubgroupToGroupEvent {}
pub trait ICNChangeHistoryAddSubgroupToGroupEvent: Sized + std::ops::Deref {
    unsafe fn subgroup(&self) -> CNGroup
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, subgroup)
    }
    unsafe fn group(&self) -> CNGroup
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, group)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CNChangeHistoryRemoveSubgroupFromGroupEvent(pub id);
impl std::ops::Deref for CNChangeHistoryRemoveSubgroupFromGroupEvent {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CNChangeHistoryRemoveSubgroupFromGroupEvent {}
impl CNChangeHistoryRemoveSubgroupFromGroupEvent {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CNChangeHistoryRemoveSubgroupFromGroupEvent").unwrap(), alloc) })
    }
}
impl ICNChangeHistoryEvent for CNChangeHistoryRemoveSubgroupFromGroupEvent {}
impl PNSCopying for CNChangeHistoryRemoveSubgroupFromGroupEvent {}
impl PNSSecureCoding for CNChangeHistoryRemoveSubgroupFromGroupEvent {}
impl std::convert::TryFrom<CNChangeHistoryEvent> for CNChangeHistoryRemoveSubgroupFromGroupEvent {
    type Error = &'static str;
    fn try_from(
        parent: CNChangeHistoryEvent,
    ) -> Result<CNChangeHistoryRemoveSubgroupFromGroupEvent, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CNChangeHistoryRemoveSubgroupFromGroupEvent").unwrap())
        };
        if is_kind_of {
            Ok(CNChangeHistoryRemoveSubgroupFromGroupEvent(parent.0))
        } else {
            Err ("This CNChangeHistoryEvent cannot be downcasted to CNChangeHistoryRemoveSubgroupFromGroupEvent" ,)
        }
    }
}
impl INSObject for CNChangeHistoryRemoveSubgroupFromGroupEvent {}
impl PNSObject for CNChangeHistoryRemoveSubgroupFromGroupEvent {}
impl ICNChangeHistoryRemoveSubgroupFromGroupEvent for CNChangeHistoryRemoveSubgroupFromGroupEvent {}
pub trait ICNChangeHistoryRemoveSubgroupFromGroupEvent: Sized + std::ops::Deref {
    unsafe fn subgroup(&self) -> CNGroup
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, subgroup)
    }
    unsafe fn group(&self) -> CNGroup
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, group)
    }
}
pub trait PCNChangeHistoryEventVisitor: Sized + std::ops::Deref {
    unsafe fn visitDropEverythingEvent_(&self, event: CNChangeHistoryDropEverythingEvent)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, visitDropEverythingEvent : event)
    }
    unsafe fn visitAddContactEvent_(&self, event: CNChangeHistoryAddContactEvent)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, visitAddContactEvent : event)
    }
    unsafe fn visitUpdateContactEvent_(&self, event: CNChangeHistoryUpdateContactEvent)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, visitUpdateContactEvent : event)
    }
    unsafe fn visitDeleteContactEvent_(&self, event: CNChangeHistoryDeleteContactEvent)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, visitDeleteContactEvent : event)
    }
    unsafe fn visitAddGroupEvent_(&self, event: CNChangeHistoryAddGroupEvent)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, visitAddGroupEvent : event)
    }
    unsafe fn visitUpdateGroupEvent_(&self, event: CNChangeHistoryUpdateGroupEvent)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, visitUpdateGroupEvent : event)
    }
    unsafe fn visitDeleteGroupEvent_(&self, event: CNChangeHistoryDeleteGroupEvent)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, visitDeleteGroupEvent : event)
    }
    unsafe fn visitAddMemberToGroupEvent_(&self, event: CNChangeHistoryAddMemberToGroupEvent)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, visitAddMemberToGroupEvent : event)
    }
    unsafe fn visitRemoveMemberFromGroupEvent_(
        &self,
        event: CNChangeHistoryRemoveMemberFromGroupEvent,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, visitRemoveMemberFromGroupEvent : event)
    }
    unsafe fn visitAddSubgroupToGroupEvent_(&self, event: CNChangeHistoryAddSubgroupToGroupEvent)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, visitAddSubgroupToGroupEvent : event)
    }
    unsafe fn visitRemoveSubgroupFromGroupEvent_(
        &self,
        event: CNChangeHistoryRemoveSubgroupFromGroupEvent,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, visitRemoveSubgroupFromGroupEvent : event)
    }
}
impl CNContact_Predicates for CNContact {}
pub trait CNContact_Predicates: Sized + std::ops::Deref {
    unsafe fn predicateForContactsMatchingName_(name: NSString) -> NSPredicate
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CNContact").unwrap(), predicateForContactsMatchingName : name)
    }
    unsafe fn predicateForContactsMatchingEmailAddress_(emailAddress: NSString) -> NSPredicate
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CNContact").unwrap(), predicateForContactsMatchingEmailAddress : emailAddress)
    }
    unsafe fn predicateForContactsMatchingPhoneNumber_(phoneNumber: CNPhoneNumber) -> NSPredicate
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CNContact").unwrap(), predicateForContactsMatchingPhoneNumber : phoneNumber)
    }
    unsafe fn predicateForContactsWithIdentifiers_(identifiers: NSArray) -> NSPredicate
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CNContact").unwrap(), predicateForContactsWithIdentifiers : identifiers)
    }
    unsafe fn predicateForContactsInGroupWithIdentifier_(groupIdentifier: NSString) -> NSPredicate
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CNContact").unwrap(), predicateForContactsInGroupWithIdentifier : groupIdentifier)
    }
    unsafe fn predicateForContactsInContainerWithIdentifier_(
        containerIdentifier: NSString,
    ) -> NSPredicate
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CNContact").unwrap(), predicateForContactsInContainerWithIdentifier : containerIdentifier)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CNMutableContact(pub id);
impl std::ops::Deref for CNMutableContact {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CNMutableContact {}
impl CNMutableContact {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CNMutableContact").unwrap(), alloc) })
    }
}
impl ICNContact for CNMutableContact {}
impl PNSCopying for CNMutableContact {}
impl PNSMutableCopying for CNMutableContact {}
impl PNSSecureCoding for CNMutableContact {}
impl From<CNMutableContact> for CNContact {
    fn from(child: CNMutableContact) -> CNContact {
        CNContact(child.0)
    }
}
impl std::convert::TryFrom<CNContact> for CNMutableContact {
    type Error = &'static str;
    fn try_from(parent: CNContact) -> Result<CNMutableContact, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CNMutableContact").unwrap()) };
        if is_kind_of {
            Ok(CNMutableContact(parent.0))
        } else {
            Err("This CNContact cannot be downcasted to CNMutableContact")
        }
    }
}
impl INSObject for CNMutableContact {}
impl PNSObject for CNMutableContact {}
impl ICNMutableContact for CNMutableContact {}
pub trait ICNMutableContact: Sized + std::ops::Deref {
    unsafe fn contactType(&self) -> CNContactType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, contactType)
    }
    unsafe fn setContactType_(&self, contactType: CNContactType)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setContactType : contactType)
    }
    unsafe fn namePrefix(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, namePrefix)
    }
    unsafe fn setNamePrefix_(&self, namePrefix: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setNamePrefix : namePrefix)
    }
    unsafe fn givenName(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, givenName)
    }
    unsafe fn setGivenName_(&self, givenName: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setGivenName : givenName)
    }
    unsafe fn middleName(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, middleName)
    }
    unsafe fn setMiddleName_(&self, middleName: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMiddleName : middleName)
    }
    unsafe fn familyName(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, familyName)
    }
    unsafe fn setFamilyName_(&self, familyName: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setFamilyName : familyName)
    }
    unsafe fn previousFamilyName(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, previousFamilyName)
    }
    unsafe fn setPreviousFamilyName_(&self, previousFamilyName: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPreviousFamilyName : previousFamilyName)
    }
    unsafe fn nameSuffix(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, nameSuffix)
    }
    unsafe fn setNameSuffix_(&self, nameSuffix: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setNameSuffix : nameSuffix)
    }
    unsafe fn nickname(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, nickname)
    }
    unsafe fn setNickname_(&self, nickname: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setNickname : nickname)
    }
    unsafe fn organizationName(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, organizationName)
    }
    unsafe fn setOrganizationName_(&self, organizationName: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setOrganizationName : organizationName)
    }
    unsafe fn departmentName(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, departmentName)
    }
    unsafe fn setDepartmentName_(&self, departmentName: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDepartmentName : departmentName)
    }
    unsafe fn jobTitle(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, jobTitle)
    }
    unsafe fn setJobTitle_(&self, jobTitle: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setJobTitle : jobTitle)
    }
    unsafe fn phoneticGivenName(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, phoneticGivenName)
    }
    unsafe fn setPhoneticGivenName_(&self, phoneticGivenName: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPhoneticGivenName : phoneticGivenName)
    }
    unsafe fn phoneticMiddleName(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, phoneticMiddleName)
    }
    unsafe fn setPhoneticMiddleName_(&self, phoneticMiddleName: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPhoneticMiddleName : phoneticMiddleName)
    }
    unsafe fn phoneticFamilyName(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, phoneticFamilyName)
    }
    unsafe fn setPhoneticFamilyName_(&self, phoneticFamilyName: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPhoneticFamilyName : phoneticFamilyName)
    }
    unsafe fn phoneticOrganizationName(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, phoneticOrganizationName)
    }
    unsafe fn setPhoneticOrganizationName_(&self, phoneticOrganizationName: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPhoneticOrganizationName : phoneticOrganizationName)
    }
    unsafe fn note(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, note)
    }
    unsafe fn setNote_(&self, note: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setNote : note)
    }
    unsafe fn imageData(&self) -> NSData
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, imageData)
    }
    unsafe fn setImageData_(&self, imageData: NSData)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setImageData : imageData)
    }
    unsafe fn phoneNumbers(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, phoneNumbers)
    }
    unsafe fn setPhoneNumbers_(&self, phoneNumbers: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPhoneNumbers : phoneNumbers)
    }
    unsafe fn emailAddresses(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, emailAddresses)
    }
    unsafe fn setEmailAddresses_(&self, emailAddresses: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setEmailAddresses : emailAddresses)
    }
    unsafe fn postalAddresses(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, postalAddresses)
    }
    unsafe fn setPostalAddresses_(&self, postalAddresses: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPostalAddresses : postalAddresses)
    }
    unsafe fn urlAddresses(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, urlAddresses)
    }
    unsafe fn setUrlAddresses_(&self, urlAddresses: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setUrlAddresses : urlAddresses)
    }
    unsafe fn contactRelations(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, contactRelations)
    }
    unsafe fn setContactRelations_(&self, contactRelations: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setContactRelations : contactRelations)
    }
    unsafe fn socialProfiles(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, socialProfiles)
    }
    unsafe fn setSocialProfiles_(&self, socialProfiles: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSocialProfiles : socialProfiles)
    }
    unsafe fn instantMessageAddresses(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, instantMessageAddresses)
    }
    unsafe fn setInstantMessageAddresses_(&self, instantMessageAddresses: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInstantMessageAddresses : instantMessageAddresses)
    }
    unsafe fn birthday(&self) -> NSDateComponents
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, birthday)
    }
    unsafe fn setBirthday_(&self, birthday: NSDateComponents)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setBirthday : birthday)
    }
    unsafe fn nonGregorianBirthday(&self) -> NSDateComponents
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, nonGregorianBirthday)
    }
    unsafe fn setNonGregorianBirthday_(&self, nonGregorianBirthday: NSDateComponents)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setNonGregorianBirthday : nonGregorianBirthday)
    }
    unsafe fn dates(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, dates)
    }
    unsafe fn setDates_(&self, dates: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDates : dates)
    }
}
impl CNContact_NSItemProvider for CNContact {}
pub trait CNContact_NSItemProvider: Sized + std::ops::Deref {}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CNMutablePostalAddress(pub id);
impl std::ops::Deref for CNMutablePostalAddress {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CNMutablePostalAddress {}
impl CNMutablePostalAddress {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CNMutablePostalAddress").unwrap(), alloc) })
    }
}
impl ICNPostalAddress for CNMutablePostalAddress {}
impl PNSCopying for CNMutablePostalAddress {}
impl PNSMutableCopying for CNMutablePostalAddress {}
impl PNSSecureCoding for CNMutablePostalAddress {}
impl From<CNMutablePostalAddress> for CNPostalAddress {
    fn from(child: CNMutablePostalAddress) -> CNPostalAddress {
        CNPostalAddress(child.0)
    }
}
impl std::convert::TryFrom<CNPostalAddress> for CNMutablePostalAddress {
    type Error = &'static str;
    fn try_from(parent: CNPostalAddress) -> Result<CNMutablePostalAddress, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CNMutablePostalAddress").unwrap()) };
        if is_kind_of {
            Ok(CNMutablePostalAddress(parent.0))
        } else {
            Err("This CNPostalAddress cannot be downcasted to CNMutablePostalAddress")
        }
    }
}
impl INSObject for CNMutablePostalAddress {}
impl PNSObject for CNMutablePostalAddress {}
impl ICNMutablePostalAddress for CNMutablePostalAddress {}
pub trait ICNMutablePostalAddress: Sized + std::ops::Deref {
    unsafe fn street(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, street)
    }
    unsafe fn setStreet_(&self, street: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setStreet : street)
    }
    unsafe fn subLocality(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, subLocality)
    }
    unsafe fn setSubLocality_(&self, subLocality: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSubLocality : subLocality)
    }
    unsafe fn city(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, city)
    }
    unsafe fn setCity_(&self, city: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCity : city)
    }
    unsafe fn subAdministrativeArea(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, subAdministrativeArea)
    }
    unsafe fn setSubAdministrativeArea_(&self, subAdministrativeArea: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSubAdministrativeArea : subAdministrativeArea)
    }
    unsafe fn state(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, state)
    }
    unsafe fn setState_(&self, state: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setState : state)
    }
    unsafe fn postalCode(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, postalCode)
    }
    unsafe fn setPostalCode_(&self, postalCode: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPostalCode : postalCode)
    }
    unsafe fn country(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, country)
    }
    unsafe fn setCountry_(&self, country: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCountry : country)
    }
    unsafe fn ISOCountryCode(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, ISOCountryCode)
    }
    unsafe fn setISOCountryCode_(&self, ISOCountryCode: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setISOCountryCode : ISOCountryCode)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CNGroup(pub id);
impl std::ops::Deref for CNGroup {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CNGroup {}
impl CNGroup {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CNGroup").unwrap(), alloc) })
    }
}
impl PNSCopying for CNGroup {}
impl PNSMutableCopying for CNGroup {}
impl PNSSecureCoding for CNGroup {}
impl INSObject for CNGroup {}
impl PNSObject for CNGroup {}
impl std::convert::TryFrom<NSObject> for CNGroup {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<CNGroup, Self::Error> {
        let is_kind_of: bool = unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CNGroup").unwrap()) };
        if is_kind_of {
            Ok(CNGroup(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to CNGroup")
        }
    }
}
impl ICNGroup for CNGroup {}
pub trait ICNGroup: Sized + std::ops::Deref {
    unsafe fn identifier(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, identifier)
    }
    unsafe fn name(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, name)
    }
}
impl CNGroup_Predicates for CNGroup {}
pub trait CNGroup_Predicates: Sized + std::ops::Deref {
    unsafe fn predicateForGroupsWithIdentifiers_(identifiers: NSArray) -> NSPredicate
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CNGroup").unwrap(), predicateForGroupsWithIdentifiers : identifiers)
    }
    unsafe fn predicateForSubgroupsInGroupWithIdentifier_(
        parentGroupIdentifier: NSString,
    ) -> NSPredicate
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CNGroup").unwrap(), predicateForSubgroupsInGroupWithIdentifier : parentGroupIdentifier)
    }
    unsafe fn predicateForGroupsInContainerWithIdentifier_(
        containerIdentifier: NSString,
    ) -> NSPredicate
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CNGroup").unwrap(), predicateForGroupsInContainerWithIdentifier : containerIdentifier)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CNMutableGroup(pub id);
impl std::ops::Deref for CNMutableGroup {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CNMutableGroup {}
impl CNMutableGroup {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CNMutableGroup").unwrap(), alloc) })
    }
}
impl ICNGroup for CNMutableGroup {}
impl PNSCopying for CNMutableGroup {}
impl PNSMutableCopying for CNMutableGroup {}
impl PNSSecureCoding for CNMutableGroup {}
impl From<CNMutableGroup> for CNGroup {
    fn from(child: CNMutableGroup) -> CNGroup {
        CNGroup(child.0)
    }
}
impl std::convert::TryFrom<CNGroup> for CNMutableGroup {
    type Error = &'static str;
    fn try_from(parent: CNGroup) -> Result<CNMutableGroup, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CNMutableGroup").unwrap()) };
        if is_kind_of {
            Ok(CNMutableGroup(parent.0))
        } else {
            Err("This CNGroup cannot be downcasted to CNMutableGroup")
        }
    }
}
impl INSObject for CNMutableGroup {}
impl PNSObject for CNMutableGroup {}
impl ICNMutableGroup for CNMutableGroup {}
pub trait ICNMutableGroup: Sized + std::ops::Deref {
    unsafe fn name(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, name)
    }
    unsafe fn setName_(&self, name: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setName : name)
    }
}
pub type CNContainerType = NSInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CNContainer(pub id);
impl std::ops::Deref for CNContainer {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CNContainer {}
impl CNContainer {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CNContainer").unwrap(), alloc) })
    }
}
impl PNSCopying for CNContainer {}
impl PNSSecureCoding for CNContainer {}
impl INSObject for CNContainer {}
impl PNSObject for CNContainer {}
impl std::convert::TryFrom<NSObject> for CNContainer {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<CNContainer, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CNContainer").unwrap()) };
        if is_kind_of {
            Ok(CNContainer(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to CNContainer")
        }
    }
}
impl ICNContainer for CNContainer {}
pub trait ICNContainer: Sized + std::ops::Deref {
    unsafe fn identifier(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, identifier)
    }
    unsafe fn name(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, name)
    }
    unsafe fn type_(&self) -> CNContainerType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, type)
    }
}
impl CNContainer_Predicates for CNContainer {}
pub trait CNContainer_Predicates: Sized + std::ops::Deref {
    unsafe fn predicateForContainersWithIdentifiers_(identifiers: NSArray) -> NSPredicate
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CNContainer").unwrap(), predicateForContainersWithIdentifiers : identifiers)
    }
    unsafe fn predicateForContainerOfContactWithIdentifier_(
        contactIdentifier: NSString,
    ) -> NSPredicate
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CNContainer").unwrap(), predicateForContainerOfContactWithIdentifier : contactIdentifier)
    }
    unsafe fn predicateForContainerOfGroupWithIdentifier_(groupIdentifier: NSString) -> NSPredicate
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CNContainer").unwrap(), predicateForContainerOfGroupWithIdentifier : groupIdentifier)
    }
}
pub type CNContactFormatterStyle = NSInteger;
pub type CNContactDisplayNameOrder = NSInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CNContactFormatter(pub id);
impl std::ops::Deref for CNContactFormatter {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CNContactFormatter {}
impl CNContactFormatter {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CNContactFormatter").unwrap(), alloc) })
    }
}
impl PNSSecureCoding for CNContactFormatter {}
impl INSFormatter for CNContactFormatter {}
impl PNSCopying for CNContactFormatter {}
impl PNSCoding for CNContactFormatter {}
impl std::convert::TryFrom<NSFormatter> for CNContactFormatter {
    type Error = &'static str;
    fn try_from(parent: NSFormatter) -> Result<CNContactFormatter, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CNContactFormatter").unwrap()) };
        if is_kind_of {
            Ok(CNContactFormatter(parent.0))
        } else {
            Err("This NSFormatter cannot be downcasted to CNContactFormatter")
        }
    }
}
impl INSObject for CNContactFormatter {}
impl PNSObject for CNContactFormatter {}
impl ICNContactFormatter for CNContactFormatter {}
pub trait ICNContactFormatter: Sized + std::ops::Deref {
    unsafe fn stringFromContact_(&self, contact: CNContact) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, stringFromContact : contact)
    }
    unsafe fn attributedStringFromContact_defaultAttributes_(
        &self,
        contact: CNContact,
        attributes: NSDictionary,
    ) -> NSAttributedString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, attributedStringFromContact : contact, defaultAttributes : attributes)
    }
    unsafe fn style(&self) -> CNContactFormatterStyle
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, style)
    }
    unsafe fn setStyle_(&self, style: CNContactFormatterStyle)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setStyle : style)
    }
    unsafe fn descriptorForRequiredKeysForStyle_(style: CNContactFormatterStyle) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CNContactFormatter").unwrap(), descriptorForRequiredKeysForStyle : style)
    }
    unsafe fn stringFromContact_style_(
        contact: CNContact,
        style: CNContactFormatterStyle,
    ) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CNContactFormatter").unwrap(), stringFromContact : contact, style : style)
    }
    unsafe fn attributedStringFromContact_style_defaultAttributes_(
        contact: CNContact,
        style: CNContactFormatterStyle,
        attributes: NSDictionary,
    ) -> NSAttributedString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CNContactFormatter").unwrap(), attributedStringFromContact : contact, style : style, defaultAttributes : attributes)
    }
    unsafe fn nameOrderForContact_(contact: CNContact) -> CNContactDisplayNameOrder
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CNContactFormatter").unwrap(), nameOrderForContact : contact)
    }
    unsafe fn delimiterForContact_(contact: CNContact) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CNContactFormatter").unwrap(), delimiterForContact : contact)
    }
    unsafe fn descriptorForRequiredKeysForNameOrder() -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CNContactFormatter").unwrap(), descriptorForRequiredKeysForNameOrder)
    }
    unsafe fn descriptorForRequiredKeysForDelimiter() -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CNContactFormatter").unwrap(), descriptorForRequiredKeysForDelimiter)
    }
}
pub type CNPostalAddressFormatterStyle = NSInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CNPostalAddressFormatter(pub id);
impl std::ops::Deref for CNPostalAddressFormatter {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CNPostalAddressFormatter {}
impl CNPostalAddressFormatter {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CNPostalAddressFormatter").unwrap(), alloc) })
    }
}
impl INSFormatter for CNPostalAddressFormatter {}
impl PNSCopying for CNPostalAddressFormatter {}
impl PNSCoding for CNPostalAddressFormatter {}
impl std::convert::TryFrom<NSFormatter> for CNPostalAddressFormatter {
    type Error = &'static str;
    fn try_from(parent: NSFormatter) -> Result<CNPostalAddressFormatter, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CNPostalAddressFormatter").unwrap()) };
        if is_kind_of {
            Ok(CNPostalAddressFormatter(parent.0))
        } else {
            Err("This NSFormatter cannot be downcasted to CNPostalAddressFormatter")
        }
    }
}
impl INSObject for CNPostalAddressFormatter {}
impl PNSObject for CNPostalAddressFormatter {}
impl ICNPostalAddressFormatter for CNPostalAddressFormatter {}
pub trait ICNPostalAddressFormatter: Sized + std::ops::Deref {
    unsafe fn stringFromPostalAddress_(&self, postalAddress: CNPostalAddress) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, stringFromPostalAddress : postalAddress)
    }
    unsafe fn attributedStringFromPostalAddress_withDefaultAttributes_(
        &self,
        postalAddress: CNPostalAddress,
        attributes: NSDictionary,
    ) -> NSAttributedString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, attributedStringFromPostalAddress : postalAddress, withDefaultAttributes : attributes)
    }
    unsafe fn style(&self) -> CNPostalAddressFormatterStyle
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, style)
    }
    unsafe fn setStyle_(&self, style: CNPostalAddressFormatterStyle)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setStyle : style)
    }
    unsafe fn stringFromPostalAddress_style_(
        postalAddress: CNPostalAddress,
        style: CNPostalAddressFormatterStyle,
    ) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CNPostalAddressFormatter").unwrap(), stringFromPostalAddress : postalAddress, style : style)
    }
    unsafe fn attributedStringFromPostalAddress_style_withDefaultAttributes_(
        postalAddress: CNPostalAddress,
        style: CNPostalAddressFormatterStyle,
        attributes: NSDictionary,
    ) -> NSAttributedString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CNPostalAddressFormatter").unwrap(), attributedStringFromPostalAddress : postalAddress, style : style, withDefaultAttributes : attributes)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CNContactVCardSerialization(pub id);
impl std::ops::Deref for CNContactVCardSerialization {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CNContactVCardSerialization {}
impl CNContactVCardSerialization {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CNContactVCardSerialization").unwrap(), alloc) })
    }
}
impl INSObject for CNContactVCardSerialization {}
impl PNSObject for CNContactVCardSerialization {}
impl std::convert::TryFrom<NSObject> for CNContactVCardSerialization {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<CNContactVCardSerialization, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CNContactVCardSerialization").unwrap()) };
        if is_kind_of {
            Ok(CNContactVCardSerialization(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to CNContactVCardSerialization")
        }
    }
}
impl ICNContactVCardSerialization for CNContactVCardSerialization {}
pub trait ICNContactVCardSerialization: Sized + std::ops::Deref {
    unsafe fn descriptorForRequiredKeys() -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CNContactVCardSerialization").unwrap(), descriptorForRequiredKeys)
    }
    unsafe fn dataWithContacts_error_(contacts: NSArray, error: *mut NSError) -> NSData
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CNContactVCardSerialization").unwrap(), dataWithContacts : contacts, error : error)
    }
    unsafe fn contactsWithData_error_(data: NSData, error: *mut NSError) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CNContactVCardSerialization").unwrap(), contactsWithData : data, error : error)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CNContactsUserDefaults(pub id);
impl std::ops::Deref for CNContactsUserDefaults {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CNContactsUserDefaults {}
impl CNContactsUserDefaults {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CNContactsUserDefaults").unwrap(), alloc) })
    }
}
impl INSObject for CNContactsUserDefaults {}
impl PNSObject for CNContactsUserDefaults {}
impl std::convert::TryFrom<NSObject> for CNContactsUserDefaults {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<CNContactsUserDefaults, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CNContactsUserDefaults").unwrap()) };
        if is_kind_of {
            Ok(CNContactsUserDefaults(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to CNContactsUserDefaults")
        }
    }
}
impl ICNContactsUserDefaults for CNContactsUserDefaults {}
pub trait ICNContactsUserDefaults: Sized + std::ops::Deref {
    unsafe fn sortOrder(&self) -> CNContactSortOrder
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, sortOrder)
    }
    unsafe fn countryCode(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, countryCode)
    }
    unsafe fn sharedDefaults() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CNContactsUserDefaults").unwrap(), sharedDefaults)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CNContactProperty(pub id);
impl std::ops::Deref for CNContactProperty {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CNContactProperty {}
impl CNContactProperty {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CNContactProperty").unwrap(), alloc) })
    }
}
impl PNSCopying for CNContactProperty {}
impl PNSSecureCoding for CNContactProperty {}
impl INSObject for CNContactProperty {}
impl PNSObject for CNContactProperty {}
impl std::convert::TryFrom<NSObject> for CNContactProperty {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<CNContactProperty, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CNContactProperty").unwrap()) };
        if is_kind_of {
            Ok(CNContactProperty(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to CNContactProperty")
        }
    }
}
impl ICNContactProperty for CNContactProperty {}
pub trait ICNContactProperty: Sized + std::ops::Deref {
    unsafe fn contact(&self) -> CNContact
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, contact)
    }
    unsafe fn key(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, key)
    }
    unsafe fn value(&self) -> id
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, value)
    }
    unsafe fn identifier(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, identifier)
    }
    unsafe fn label(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, label)
    }
}
pub type CNErrorCode = NSInteger;
unsafe extern "C" {
    pub static CNLabelHome: NSString;
}
unsafe extern "C" {
    pub static CNLabelWork: NSString;
}
unsafe extern "C" {
    pub static CNLabelSchool: NSString;
}
unsafe extern "C" {
    pub static CNLabelOther: NSString;
}
unsafe extern "C" {
    pub static CNLabelEmailiCloud: NSString;
}
unsafe extern "C" {
    pub static CNLabelURLAddressHomePage: NSString;
}
unsafe extern "C" {
    pub static CNLabelDateAnniversary: NSString;
}
unsafe extern "C" {
    pub static CNLabelPhoneNumberiPhone: NSString;
}
unsafe extern "C" {
    pub static CNLabelPhoneNumberAppleWatch: NSString;
}
unsafe extern "C" {
    pub static CNLabelPhoneNumberMobile: NSString;
}
unsafe extern "C" {
    pub static CNLabelPhoneNumberMain: NSString;
}
unsafe extern "C" {
    pub static CNLabelPhoneNumberHomeFax: NSString;
}
unsafe extern "C" {
    pub static CNLabelPhoneNumberWorkFax: NSString;
}
unsafe extern "C" {
    pub static CNLabelPhoneNumberOtherFax: NSString;
}
unsafe extern "C" {
    pub static CNLabelPhoneNumberPager: NSString;
}
unsafe extern "C" {
    pub static CNPostalAddressStreetKey: NSString;
}
unsafe extern "C" {
    pub static CNPostalAddressSubLocalityKey: NSString;
}
unsafe extern "C" {
    pub static CNPostalAddressCityKey: NSString;
}
unsafe extern "C" {
    pub static CNPostalAddressSubAdministrativeAreaKey: NSString;
}
unsafe extern "C" {
    pub static CNPostalAddressStateKey: NSString;
}
unsafe extern "C" {
    pub static CNPostalAddressPostalCodeKey: NSString;
}
unsafe extern "C" {
    pub static CNPostalAddressCountryKey: NSString;
}
unsafe extern "C" {
    pub static CNPostalAddressISOCountryCodeKey: NSString;
}
unsafe extern "C" {
    pub static CNLabelContactRelationAssistant: NSString;
}
unsafe extern "C" {
    pub static CNLabelContactRelationManager: NSString;
}
unsafe extern "C" {
    pub static CNLabelContactRelationColleague: NSString;
}
unsafe extern "C" {
    pub static CNLabelContactRelationTeacher: NSString;
}
unsafe extern "C" {
    pub static CNLabelContactRelationSibling: NSString;
}
unsafe extern "C" {
    pub static CNLabelContactRelationYoungerSibling: NSString;
}
unsafe extern "C" {
    pub static CNLabelContactRelationElderSibling: NSString;
}
unsafe extern "C" {
    pub static CNLabelContactRelationSister: NSString;
}
unsafe extern "C" {
    pub static CNLabelContactRelationYoungerSister: NSString;
}
unsafe extern "C" {
    pub static CNLabelContactRelationYoungestSister: NSString;
}
unsafe extern "C" {
    pub static CNLabelContactRelationElderSister: NSString;
}
unsafe extern "C" {
    pub static CNLabelContactRelationEldestSister: NSString;
}
unsafe extern "C" {
    pub static CNLabelContactRelationBrother: NSString;
}
unsafe extern "C" {
    pub static CNLabelContactRelationYoungerBrother: NSString;
}
unsafe extern "C" {
    pub static CNLabelContactRelationYoungestBrother: NSString;
}
unsafe extern "C" {
    pub static CNLabelContactRelationElderBrother: NSString;
}
unsafe extern "C" {
    pub static CNLabelContactRelationEldestBrother: NSString;
}
unsafe extern "C" {
    pub static CNLabelContactRelationFriend: NSString;
}
unsafe extern "C" {
    pub static CNLabelContactRelationMaleFriend: NSString;
}
unsafe extern "C" {
    pub static CNLabelContactRelationFemaleFriend: NSString;
}
unsafe extern "C" {
    pub static CNLabelContactRelationSpouse: NSString;
}
unsafe extern "C" {
    pub static CNLabelContactRelationWife: NSString;
}
unsafe extern "C" {
    pub static CNLabelContactRelationHusband: NSString;
}
unsafe extern "C" {
    pub static CNLabelContactRelationPartner: NSString;
}
unsafe extern "C" {
    pub static CNLabelContactRelationMalePartner: NSString;
}
unsafe extern "C" {
    pub static CNLabelContactRelationFemalePartner: NSString;
}
unsafe extern "C" {
    pub static CNLabelContactRelationGirlfriendOrBoyfriend: NSString;
}
unsafe extern "C" {
    pub static CNLabelContactRelationGirlfriend: NSString;
}
unsafe extern "C" {
    pub static CNLabelContactRelationBoyfriend: NSString;
}
unsafe extern "C" {
    pub static CNLabelContactRelationParent: NSString;
}
unsafe extern "C" {
    pub static CNLabelContactRelationMother: NSString;
}
unsafe extern "C" {
    pub static CNLabelContactRelationFather: NSString;
}
unsafe extern "C" {
    pub static CNLabelContactRelationChild: NSString;
}
unsafe extern "C" {
    pub static CNLabelContactRelationDaughter: NSString;
}
unsafe extern "C" {
    pub static CNLabelContactRelationSon: NSString;
}
unsafe extern "C" {
    pub static CNLabelContactRelationGrandparent: NSString;
}
unsafe extern "C" {
    pub static CNLabelContactRelationGrandmother: NSString;
}
unsafe extern "C" {
    pub static CNLabelContactRelationGrandmotherMothersMother: NSString;
}
unsafe extern "C" {
    pub static CNLabelContactRelationGrandmotherFathersMother: NSString;
}
unsafe extern "C" {
    pub static CNLabelContactRelationGrandfather: NSString;
}
unsafe extern "C" {
    pub static CNLabelContactRelationGrandfatherMothersFather: NSString;
}
unsafe extern "C" {
    pub static CNLabelContactRelationGrandfatherFathersFather: NSString;
}
unsafe extern "C" {
    pub static CNLabelContactRelationGreatGrandparent: NSString;
}
unsafe extern "C" {
    pub static CNLabelContactRelationGreatGrandmother: NSString;
}
unsafe extern "C" {
    pub static CNLabelContactRelationGreatGrandfather: NSString;
}
unsafe extern "C" {
    pub static CNLabelContactRelationGrandchild: NSString;
}
unsafe extern "C" {
    pub static CNLabelContactRelationGranddaughter: NSString;
}
unsafe extern "C" {
    pub static CNLabelContactRelationGranddaughterDaughtersDaughter: NSString;
}
unsafe extern "C" {
    pub static CNLabelContactRelationGranddaughterSonsDaughter: NSString;
}
unsafe extern "C" {
    pub static CNLabelContactRelationGrandson: NSString;
}
unsafe extern "C" {
    pub static CNLabelContactRelationGrandsonDaughtersSon: NSString;
}
unsafe extern "C" {
    pub static CNLabelContactRelationGrandsonSonsSon: NSString;
}
unsafe extern "C" {
    pub static CNLabelContactRelationGreatGrandchild: NSString;
}
unsafe extern "C" {
    pub static CNLabelContactRelationGreatGranddaughter: NSString;
}
unsafe extern "C" {
    pub static CNLabelContactRelationGreatGrandson: NSString;
}
unsafe extern "C" {
    pub static CNLabelContactRelationParentInLaw: NSString;
}
unsafe extern "C" {
    pub static CNLabelContactRelationMotherInLaw: NSString;
}
unsafe extern "C" {
    pub static CNLabelContactRelationMotherInLawWifesMother: NSString;
}
unsafe extern "C" {
    pub static CNLabelContactRelationMotherInLawHusbandsMother: NSString;
}
unsafe extern "C" {
    pub static CNLabelContactRelationFatherInLaw: NSString;
}
unsafe extern "C" {
    pub static CNLabelContactRelationFatherInLawWifesFather: NSString;
}
unsafe extern "C" {
    pub static CNLabelContactRelationFatherInLawHusbandsFather: NSString;
}
unsafe extern "C" {
    pub static CNLabelContactRelationCoParentInLaw: NSString;
}
unsafe extern "C" {
    pub static CNLabelContactRelationCoMotherInLaw: NSString;
}
unsafe extern "C" {
    pub static CNLabelContactRelationCoFatherInLaw: NSString;
}
unsafe extern "C" {
    pub static CNLabelContactRelationSiblingInLaw: NSString;
}
unsafe extern "C" {
    pub static CNLabelContactRelationYoungerSiblingInLaw: NSString;
}
unsafe extern "C" {
    pub static CNLabelContactRelationElderSiblingInLaw: NSString;
}
unsafe extern "C" {
    pub static CNLabelContactRelationSisterInLaw: NSString;
}
unsafe extern "C" {
    pub static CNLabelContactRelationYoungerSisterInLaw: NSString;
}
unsafe extern "C" {
    pub static CNLabelContactRelationElderSisterInLaw: NSString;
}
unsafe extern "C" {
    pub static CNLabelContactRelationSisterInLawSpousesSister: NSString;
}
unsafe extern "C" {
    pub static CNLabelContactRelationSisterInLawWifesSister: NSString;
}
unsafe extern "C" {
    pub static CNLabelContactRelationSisterInLawHusbandsSister: NSString;
}
unsafe extern "C" {
    pub static CNLabelContactRelationSisterInLawBrothersWife: NSString;
}
unsafe extern "C" {
    pub static CNLabelContactRelationSisterInLawYoungerBrothersWife: NSString;
}
unsafe extern "C" {
    pub static CNLabelContactRelationSisterInLawElderBrothersWife: NSString;
}
unsafe extern "C" {
    pub static CNLabelContactRelationBrotherInLaw: NSString;
}
unsafe extern "C" {
    pub static CNLabelContactRelationYoungerBrotherInLaw: NSString;
}
unsafe extern "C" {
    pub static CNLabelContactRelationElderBrotherInLaw: NSString;
}
unsafe extern "C" {
    pub static CNLabelContactRelationBrotherInLawSpousesBrother: NSString;
}
unsafe extern "C" {
    pub static CNLabelContactRelationBrotherInLawHusbandsBrother: NSString;
}
unsafe extern "C" {
    pub static CNLabelContactRelationBrotherInLawWifesBrother: NSString;
}
unsafe extern "C" {
    pub static CNLabelContactRelationBrotherInLawSistersHusband: NSString;
}
unsafe extern "C" {
    pub static CNLabelContactRelationBrotherInLawYoungerSistersHusband: NSString;
}
unsafe extern "C" {
    pub static CNLabelContactRelationBrotherInLawElderSistersHusband: NSString;
}
unsafe extern "C" {
    pub static CNLabelContactRelationSisterInLawWifesBrothersWife: NSString;
}
unsafe extern "C" {
    pub static CNLabelContactRelationSisterInLawHusbandsBrothersWife: NSString;
}
unsafe extern "C" {
    pub static CNLabelContactRelationBrotherInLawWifesSistersHusband: NSString;
}
unsafe extern "C" {
    pub static CNLabelContactRelationBrotherInLawHusbandsSistersHusband: NSString;
}
unsafe extern "C" {
    pub static CNLabelContactRelationCoSiblingInLaw: NSString;
}
unsafe extern "C" {
    pub static CNLabelContactRelationCoSisterInLaw: NSString;
}
unsafe extern "C" {
    pub static CNLabelContactRelationCoBrotherInLaw: NSString;
}
unsafe extern "C" {
    pub static CNLabelContactRelationChildInLaw: NSString;
}
unsafe extern "C" {
    pub static CNLabelContactRelationDaughterInLaw: NSString;
}
unsafe extern "C" {
    pub static CNLabelContactRelationSonInLaw: NSString;
}
unsafe extern "C" {
    pub static CNLabelContactRelationCousin: NSString;
}
unsafe extern "C" {
    pub static CNLabelContactRelationYoungerCousin: NSString;
}
unsafe extern "C" {
    pub static CNLabelContactRelationElderCousin: NSString;
}
unsafe extern "C" {
    pub static CNLabelContactRelationMaleCousin: NSString;
}
unsafe extern "C" {
    pub static CNLabelContactRelationFemaleCousin: NSString;
}
unsafe extern "C" {
    pub static CNLabelContactRelationCousinParentsSiblingsChild: NSString;
}
unsafe extern "C" {
    pub static CNLabelContactRelationCousinParentsSiblingsSon: NSString;
}
unsafe extern "C" {
    pub static CNLabelContactRelationYoungerCousinParentsSiblingsSon: NSString;
}
unsafe extern "C" {
    pub static CNLabelContactRelationElderCousinParentsSiblingsSon: NSString;
}
unsafe extern "C" {
    pub static CNLabelContactRelationCousinParentsSiblingsDaughter: NSString;
}
unsafe extern "C" {
    pub static CNLabelContactRelationYoungerCousinParentsSiblingsDaughter: NSString;
}
unsafe extern "C" {
    pub static CNLabelContactRelationElderCousinParentsSiblingsDaughter: NSString;
}
unsafe extern "C" {
    pub static CNLabelContactRelationCousinMothersSistersDaughter: NSString;
}
unsafe extern "C" {
    pub static CNLabelContactRelationYoungerCousinMothersSistersDaughter: NSString;
}
unsafe extern "C" {
    pub static CNLabelContactRelationElderCousinMothersSistersDaughter: NSString;
}
unsafe extern "C" {
    pub static CNLabelContactRelationCousinMothersSistersSon: NSString;
}
unsafe extern "C" {
    pub static CNLabelContactRelationYoungerCousinMothersSistersSon: NSString;
}
unsafe extern "C" {
    pub static CNLabelContactRelationElderCousinMothersSistersSon: NSString;
}
unsafe extern "C" {
    pub static CNLabelContactRelationCousinMothersBrothersDaughter: NSString;
}
unsafe extern "C" {
    pub static CNLabelContactRelationYoungerCousinMothersBrothersDaughter: NSString;
}
unsafe extern "C" {
    pub static CNLabelContactRelationElderCousinMothersBrothersDaughter: NSString;
}
unsafe extern "C" {
    pub static CNLabelContactRelationCousinMothersBrothersSon: NSString;
}
unsafe extern "C" {
    pub static CNLabelContactRelationYoungerCousinMothersBrothersSon: NSString;
}
unsafe extern "C" {
    pub static CNLabelContactRelationElderCousinMothersBrothersSon: NSString;
}
unsafe extern "C" {
    pub static CNLabelContactRelationCousinFathersSistersDaughter: NSString;
}
unsafe extern "C" {
    pub static CNLabelContactRelationYoungerCousinFathersSistersDaughter: NSString;
}
unsafe extern "C" {
    pub static CNLabelContactRelationElderCousinFathersSistersDaughter: NSString;
}
unsafe extern "C" {
    pub static CNLabelContactRelationCousinFathersSistersSon: NSString;
}
unsafe extern "C" {
    pub static CNLabelContactRelationYoungerCousinFathersSistersSon: NSString;
}
unsafe extern "C" {
    pub static CNLabelContactRelationElderCousinFathersSistersSon: NSString;
}
unsafe extern "C" {
    pub static CNLabelContactRelationCousinFathersBrothersDaughter: NSString;
}
unsafe extern "C" {
    pub static CNLabelContactRelationYoungerCousinFathersBrothersDaughter: NSString;
}
unsafe extern "C" {
    pub static CNLabelContactRelationElderCousinFathersBrothersDaughter: NSString;
}
unsafe extern "C" {
    pub static CNLabelContactRelationCousinFathersBrothersSon: NSString;
}
unsafe extern "C" {
    pub static CNLabelContactRelationYoungerCousinFathersBrothersSon: NSString;
}
unsafe extern "C" {
    pub static CNLabelContactRelationElderCousinFathersBrothersSon: NSString;
}
unsafe extern "C" {
    pub static CNLabelContactRelationCousinGrandparentsSiblingsChild: NSString;
}
unsafe extern "C" {
    pub static CNLabelContactRelationCousinGrandparentsSiblingsDaughter: NSString;
}
unsafe extern "C" {
    pub static CNLabelContactRelationCousinGrandparentsSiblingsSon: NSString;
}
unsafe extern "C" {
    pub static CNLabelContactRelationYoungerCousinMothersSiblingsSonOrFathersSistersSon: NSString;
}
unsafe extern "C" {
    pub static CNLabelContactRelationElderCousinMothersSiblingsSonOrFathersSistersSon: NSString;
}
unsafe extern "C" {
    pub static CNLabelContactRelationYoungerCousinMothersSiblingsDaughterOrFathersSistersDaughter:
        NSString;
}
unsafe extern "C" {
    pub static CNLabelContactRelationElderCousinMothersSiblingsDaughterOrFathersSistersDaughter:
        NSString;
}
unsafe extern "C" {
    pub static CNLabelContactRelationParentsSibling: NSString;
}
unsafe extern "C" {
    pub static CNLabelContactRelationParentsYoungerSibling: NSString;
}
unsafe extern "C" {
    pub static CNLabelContactRelationParentsElderSibling: NSString;
}
unsafe extern "C" {
    pub static CNLabelContactRelationParentsSiblingMothersSibling: NSString;
}
unsafe extern "C" {
    pub static CNLabelContactRelationParentsSiblingMothersYoungerSibling: NSString;
}
unsafe extern "C" {
    pub static CNLabelContactRelationParentsSiblingMothersElderSibling: NSString;
}
unsafe extern "C" {
    pub static CNLabelContactRelationParentsSiblingFathersSibling: NSString;
}
unsafe extern "C" {
    pub static CNLabelContactRelationParentsSiblingFathersYoungerSibling: NSString;
}
unsafe extern "C" {
    pub static CNLabelContactRelationParentsSiblingFathersElderSibling: NSString;
}
unsafe extern "C" {
    pub static CNLabelContactRelationAunt: NSString;
}
unsafe extern "C" {
    pub static CNLabelContactRelationAuntParentsSister: NSString;
}
unsafe extern "C" {
    pub static CNLabelContactRelationAuntParentsYoungerSister: NSString;
}
unsafe extern "C" {
    pub static CNLabelContactRelationAuntParentsElderSister: NSString;
}
unsafe extern "C" {
    pub static CNLabelContactRelationAuntFathersSister: NSString;
}
unsafe extern "C" {
    pub static CNLabelContactRelationAuntFathersYoungerSister: NSString;
}
unsafe extern "C" {
    pub static CNLabelContactRelationAuntFathersElderSister: NSString;
}
unsafe extern "C" {
    pub static CNLabelContactRelationAuntFathersBrothersWife: NSString;
}
unsafe extern "C" {
    pub static CNLabelContactRelationAuntFathersYoungerBrothersWife: NSString;
}
unsafe extern "C" {
    pub static CNLabelContactRelationAuntFathersElderBrothersWife: NSString;
}
unsafe extern "C" {
    pub static CNLabelContactRelationAuntMothersSister: NSString;
}
unsafe extern "C" {
    pub static CNLabelContactRelationAuntMothersYoungerSister: NSString;
}
unsafe extern "C" {
    pub static CNLabelContactRelationAuntMothersElderSister: NSString;
}
unsafe extern "C" {
    pub static CNLabelContactRelationAuntMothersBrothersWife: NSString;
}
unsafe extern "C" {
    pub static CNLabelContactRelationGrandaunt: NSString;
}
unsafe extern "C" {
    pub static CNLabelContactRelationUncle: NSString;
}
unsafe extern "C" {
    pub static CNLabelContactRelationUncleParentsBrother: NSString;
}
unsafe extern "C" {
    pub static CNLabelContactRelationUncleParentsYoungerBrother: NSString;
}
unsafe extern "C" {
    pub static CNLabelContactRelationUncleParentsElderBrother: NSString;
}
unsafe extern "C" {
    pub static CNLabelContactRelationUncleMothersBrother: NSString;
}
unsafe extern "C" {
    pub static CNLabelContactRelationUncleMothersYoungerBrother: NSString;
}
unsafe extern "C" {
    pub static CNLabelContactRelationUncleMothersElderBrother: NSString;
}
unsafe extern "C" {
    pub static CNLabelContactRelationUncleMothersSistersHusband: NSString;
}
unsafe extern "C" {
    pub static CNLabelContactRelationUncleFathersBrother: NSString;
}
unsafe extern "C" {
    pub static CNLabelContactRelationUncleFathersYoungerBrother: NSString;
}
unsafe extern "C" {
    pub static CNLabelContactRelationUncleFathersElderBrother: NSString;
}
unsafe extern "C" {
    pub static CNLabelContactRelationUncleFathersSistersHusband: NSString;
}
unsafe extern "C" {
    pub static CNLabelContactRelationUncleFathersYoungerSistersHusband: NSString;
}
unsafe extern "C" {
    pub static CNLabelContactRelationUncleFathersElderSistersHusband: NSString;
}
unsafe extern "C" {
    pub static CNLabelContactRelationGranduncle: NSString;
}
unsafe extern "C" {
    pub static CNLabelContactRelationSiblingsChild: NSString;
}
unsafe extern "C" {
    pub static CNLabelContactRelationNiece: NSString;
}
unsafe extern "C" {
    pub static CNLabelContactRelationNieceSistersDaughter: NSString;
}
unsafe extern "C" {
    pub static CNLabelContactRelationNieceBrothersDaughter: NSString;
}
unsafe extern "C" {
    pub static CNLabelContactRelationNieceSistersDaughterOrWifesSiblingsDaughter: NSString;
}
unsafe extern "C" {
    pub static CNLabelContactRelationNieceBrothersDaughterOrHusbandsSiblingsDaughter: NSString;
}
unsafe extern "C" {
    pub static CNLabelContactRelationNephew: NSString;
}
unsafe extern "C" {
    pub static CNLabelContactRelationNephewSistersSon: NSString;
}
unsafe extern "C" {
    pub static CNLabelContactRelationNephewBrothersSon: NSString;
}
unsafe extern "C" {
    pub static CNLabelContactRelationNephewBrothersSonOrHusbandsSiblingsSon: NSString;
}
unsafe extern "C" {
    pub static CNLabelContactRelationNephewSistersSonOrWifesSiblingsSon: NSString;
}
unsafe extern "C" {
    pub static CNLabelContactRelationGrandniece: NSString;
}
unsafe extern "C" {
    pub static CNLabelContactRelationGrandnieceSistersGranddaughter: NSString;
}
unsafe extern "C" {
    pub static CNLabelContactRelationGrandnieceBrothersGranddaughter: NSString;
}
unsafe extern "C" {
    pub static CNLabelContactRelationGrandnephew: NSString;
}
unsafe extern "C" {
    pub static CNLabelContactRelationGrandnephewSistersGrandson: NSString;
}
unsafe extern "C" {
    pub static CNLabelContactRelationGrandnephewBrothersGrandson: NSString;
}
unsafe extern "C" {
    pub static CNLabelContactRelationStepparent: NSString;
}
unsafe extern "C" {
    pub static CNLabelContactRelationStepmother: NSString;
}
unsafe extern "C" {
    pub static CNLabelContactRelationStepfather: NSString;
}
unsafe extern "C" {
    pub static CNLabelContactRelationStepchild: NSString;
}
unsafe extern "C" {
    pub static CNLabelContactRelationStepdaughter: NSString;
}
unsafe extern "C" {
    pub static CNLabelContactRelationStepson: NSString;
}
unsafe extern "C" {
    pub static CNLabelContactRelationStepsister: NSString;
}
unsafe extern "C" {
    pub static CNLabelContactRelationStepbrother: NSString;
}
unsafe extern "C" {
    pub static CNLabelContactRelationMotherInLawOrStepmother: NSString;
}
unsafe extern "C" {
    pub static CNLabelContactRelationFatherInLawOrStepfather: NSString;
}
unsafe extern "C" {
    pub static CNLabelContactRelationDaughterInLawOrStepdaughter: NSString;
}
unsafe extern "C" {
    pub static CNLabelContactRelationSonInLawOrStepson: NSString;
}
unsafe extern "C" {
    pub static CNLabelContactRelationCousinOrSiblingsChild: NSString;
}
unsafe extern "C" {
    pub static CNLabelContactRelationNieceOrCousin: NSString;
}
unsafe extern "C" {
    pub static CNLabelContactRelationNephewOrCousin: NSString;
}
unsafe extern "C" {
    pub static CNLabelContactRelationGrandchildOrSiblingsChild: NSString;
}
unsafe extern "C" {
    pub static CNLabelContactRelationGranddaughterOrNiece: NSString;
}
unsafe extern "C" {
    pub static CNLabelContactRelationGrandsonOrNephew: NSString;
}
unsafe extern "C" {
    pub static CNLabelContactRelationGreatGrandchildOrSiblingsGrandchild: NSString;
}
unsafe extern "C" {
    pub static CNLabelContactRelationDaughterInLawOrSisterInLaw: NSString;
}
unsafe extern "C" {
    pub static CNLabelContactRelationSonInLawOrBrotherInLaw: NSString;
}
unsafe extern "C" {
    pub static CNSocialProfileURLStringKey: NSString;
}
unsafe extern "C" {
    pub static CNSocialProfileUsernameKey: NSString;
}
unsafe extern "C" {
    pub static CNSocialProfileUserIdentifierKey: NSString;
}
unsafe extern "C" {
    pub static CNSocialProfileServiceKey: NSString;
}
unsafe extern "C" {
    pub static CNSocialProfileServiceFacebook: NSString;
}
unsafe extern "C" {
    pub static CNSocialProfileServiceFlickr: NSString;
}
unsafe extern "C" {
    pub static CNSocialProfileServiceLinkedIn: NSString;
}
unsafe extern "C" {
    pub static CNSocialProfileServiceMySpace: NSString;
}
unsafe extern "C" {
    pub static CNSocialProfileServiceSinaWeibo: NSString;
}
unsafe extern "C" {
    pub static CNSocialProfileServiceTencentWeibo: NSString;
}
unsafe extern "C" {
    pub static CNSocialProfileServiceTwitter: NSString;
}
unsafe extern "C" {
    pub static CNSocialProfileServiceYelp: NSString;
}
unsafe extern "C" {
    pub static CNSocialProfileServiceGameCenter: NSString;
}
unsafe extern "C" {
    pub static CNInstantMessageAddressUsernameKey: NSString;
}
unsafe extern "C" {
    pub static CNInstantMessageAddressServiceKey: NSString;
}
unsafe extern "C" {
    pub static CNInstantMessageServiceAIM: NSString;
}
unsafe extern "C" {
    pub static CNInstantMessageServiceFacebook: NSString;
}
unsafe extern "C" {
    pub static CNInstantMessageServiceGaduGadu: NSString;
}
unsafe extern "C" {
    pub static CNInstantMessageServiceGoogleTalk: NSString;
}
unsafe extern "C" {
    pub static CNInstantMessageServiceICQ: NSString;
}
unsafe extern "C" {
    pub static CNInstantMessageServiceJabber: NSString;
}
unsafe extern "C" {
    pub static CNInstantMessageServiceMSN: NSString;
}
unsafe extern "C" {
    pub static CNInstantMessageServiceQQ: NSString;
}
unsafe extern "C" {
    pub static CNInstantMessageServiceSkype: NSString;
}
unsafe extern "C" {
    pub static CNInstantMessageServiceYahoo: NSString;
}
unsafe extern "C" {
    pub static CNContactPropertyNotFetchedExceptionName: NSString;
}
unsafe extern "C" {
    pub static CNContactIdentifierKey: NSString;
}
unsafe extern "C" {
    pub static CNContactNamePrefixKey: NSString;
}
unsafe extern "C" {
    pub static CNContactGivenNameKey: NSString;
}
unsafe extern "C" {
    pub static CNContactMiddleNameKey: NSString;
}
unsafe extern "C" {
    pub static CNContactFamilyNameKey: NSString;
}
unsafe extern "C" {
    pub static CNContactPreviousFamilyNameKey: NSString;
}
unsafe extern "C" {
    pub static CNContactNameSuffixKey: NSString;
}
unsafe extern "C" {
    pub static CNContactNicknameKey: NSString;
}
unsafe extern "C" {
    pub static CNContactOrganizationNameKey: NSString;
}
unsafe extern "C" {
    pub static CNContactDepartmentNameKey: NSString;
}
unsafe extern "C" {
    pub static CNContactJobTitleKey: NSString;
}
unsafe extern "C" {
    pub static CNContactPhoneticGivenNameKey: NSString;
}
unsafe extern "C" {
    pub static CNContactPhoneticMiddleNameKey: NSString;
}
unsafe extern "C" {
    pub static CNContactPhoneticFamilyNameKey: NSString;
}
unsafe extern "C" {
    pub static CNContactPhoneticOrganizationNameKey: NSString;
}
unsafe extern "C" {
    pub static CNContactBirthdayKey: NSString;
}
unsafe extern "C" {
    pub static CNContactNonGregorianBirthdayKey: NSString;
}
unsafe extern "C" {
    pub static CNContactNoteKey: NSString;
}
unsafe extern "C" {
    pub static CNContactImageDataKey: NSString;
}
unsafe extern "C" {
    pub static CNContactThumbnailImageDataKey: NSString;
}
unsafe extern "C" {
    pub static CNContactImageDataAvailableKey: NSString;
}
unsafe extern "C" {
    pub static CNContactTypeKey: NSString;
}
unsafe extern "C" {
    pub static CNContactPhoneNumbersKey: NSString;
}
unsafe extern "C" {
    pub static CNContactEmailAddressesKey: NSString;
}
unsafe extern "C" {
    pub static CNContactPostalAddressesKey: NSString;
}
unsafe extern "C" {
    pub static CNContactDatesKey: NSString;
}
unsafe extern "C" {
    pub static CNContactUrlAddressesKey: NSString;
}
unsafe extern "C" {
    pub static CNContactRelationsKey: NSString;
}
unsafe extern "C" {
    pub static CNContactSocialProfilesKey: NSString;
}
unsafe extern "C" {
    pub static CNContactInstantMessageAddressesKey: NSString;
}
unsafe extern "C" {
    pub static CNContactStoreDidChangeNotification: NSString;
}
unsafe extern "C" {
    pub static CNGroupIdentifierKey: NSString;
}
unsafe extern "C" {
    pub static CNGroupNameKey: NSString;
}
unsafe extern "C" {
    pub static CNContainerIdentifierKey: NSString;
}
unsafe extern "C" {
    pub static CNContainerNameKey: NSString;
}
unsafe extern "C" {
    pub static CNContainerTypeKey: NSString;
}
unsafe extern "C" {
    pub static CNContactPropertyAttribute: NSString;
}
unsafe extern "C" {
    pub static CNPostalAddressPropertyAttribute: NSString;
}
unsafe extern "C" {
    pub static CNPostalAddressLocalizedPropertyNameAttribute: NSString;
}
unsafe extern "C" {
    pub static CNErrorDomain: NSString;
}
unsafe extern "C" {
    pub static CNErrorUserInfoAffectedRecordsKey: NSString;
}
unsafe extern "C" {
    pub static CNErrorUserInfoAffectedRecordIdentifiersKey: NSString;
}
unsafe extern "C" {
    pub static CNErrorUserInfoValidationErrorsKey: NSString;
}
unsafe extern "C" {
    pub static CNErrorUserInfoKeyPathsKey: NSString;
}

unsafe impl objc2::encode::RefEncode for CNLabeledValue {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CNLabeledValue {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CNPhoneNumber {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CNPhoneNumber {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CNPostalAddress {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CNPostalAddress {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CNContactRelation {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CNContactRelation {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CNSocialProfile {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CNSocialProfile {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CNInstantMessageAddress {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CNInstantMessageAddress {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CNContact {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CNContact {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CNContactStore {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CNContactStore {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CNFetchRequest {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CNFetchRequest {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CNContactFetchRequest {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CNContactFetchRequest {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CNFetchResult {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CNFetchResult {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CNSaveRequest {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CNSaveRequest {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CNChangeHistoryFetchRequest {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CNChangeHistoryFetchRequest {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CNChangeHistoryEvent {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CNChangeHistoryEvent {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CNChangeHistoryDropEverythingEvent {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CNChangeHistoryDropEverythingEvent {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CNChangeHistoryAddContactEvent {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CNChangeHistoryAddContactEvent {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CNChangeHistoryUpdateContactEvent {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CNChangeHistoryUpdateContactEvent {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CNChangeHistoryDeleteContactEvent {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CNChangeHistoryDeleteContactEvent {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CNChangeHistoryAddGroupEvent {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CNChangeHistoryAddGroupEvent {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CNChangeHistoryUpdateGroupEvent {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CNChangeHistoryUpdateGroupEvent {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CNChangeHistoryDeleteGroupEvent {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CNChangeHistoryDeleteGroupEvent {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CNChangeHistoryAddMemberToGroupEvent {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CNChangeHistoryAddMemberToGroupEvent {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CNChangeHistoryRemoveMemberFromGroupEvent {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CNChangeHistoryRemoveMemberFromGroupEvent {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CNChangeHistoryAddSubgroupToGroupEvent {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CNChangeHistoryAddSubgroupToGroupEvent {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CNChangeHistoryRemoveSubgroupFromGroupEvent {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CNChangeHistoryRemoveSubgroupFromGroupEvent {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CNMutableContact {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CNMutableContact {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CNMutablePostalAddress {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CNMutablePostalAddress {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CNGroup {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CNGroup {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CNMutableGroup {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CNMutableGroup {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CNContainer {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CNContainer {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CNContactFormatter {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CNContactFormatter {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CNPostalAddressFormatter {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CNPostalAddressFormatter {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CNContactVCardSerialization {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CNContactVCardSerialization {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CNContactsUserDefaults {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CNContactsUserDefaults {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CNContactProperty {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CNContactProperty {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
