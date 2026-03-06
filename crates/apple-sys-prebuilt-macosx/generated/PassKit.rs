pub type mount_t = *mut ::std::os::raw::c_void;
#[allow(unused_imports)]
use crate::objc::*;
#[allow(unused_imports)]
use crate::AppKit::*;
#[allow(unused_imports)]
use crate::Contacts::*;
#[allow(unused_imports)]
use crate::CoreFoundation::*;
#[allow(unused_imports)]
use crate::CoreGraphics::*;
#[allow(unused_imports)]
use crate::Foundation::*;

#[allow(unused_imports)]
use objc2::msg_send;
pub type PKEncryptionScheme = NSString;
pub type PKPaymentNetwork = NSString;
pub type PKPaymentAuthorizationStatus = NSInteger;
pub type PKPaymentButtonStyle = NSInteger;
pub type PKPaymentButtonType = NSInteger;
pub type PKRadioTechnology = NSUInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct PKObject(pub id);
impl std::ops::Deref for PKObject {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for PKObject {}
impl PKObject {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"PKObject").unwrap(), alloc) })
    }
}
impl INSObject for PKObject {}
impl PNSObject for PKObject {}
impl std::convert::TryFrom<NSObject> for PKObject {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<PKObject, Self::Error> {
        let is_kind_of: bool = unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"PKObject").unwrap()) };
        if is_kind_of {
            Ok(PKObject(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to PKObject")
        }
    }
}
impl IPKObject for PKObject {}
pub trait IPKObject: Sized + std::ops::Deref {}
pub type PKPassType = NSUInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct PKPass(pub id);
impl std::ops::Deref for PKPass {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for PKPass {}
impl PKPass {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"PKPass").unwrap(), alloc) })
    }
}
impl IPKObject for PKPass {}
impl From<PKPass> for PKObject {
    fn from(child: PKPass) -> PKObject {
        PKObject(child.0)
    }
}
impl std::convert::TryFrom<PKObject> for PKPass {
    type Error = &'static str;
    fn try_from(parent: PKObject) -> Result<PKPass, Self::Error> {
        let is_kind_of: bool = unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"PKPass").unwrap()) };
        if is_kind_of {
            Ok(PKPass(parent.0))
        } else {
            Err("This PKObject cannot be downcasted to PKPass")
        }
    }
}
impl INSObject for PKPass {}
impl PNSObject for PKPass {}
impl IPKPass for PKPass {}
pub trait IPKPass: Sized + std::ops::Deref {
    unsafe fn initWithData_error_(&self, data: NSData, error: *mut NSError) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithData : data, error : error)
    }
    unsafe fn localizedValueForFieldKey_(&self, key: NSString) -> id
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, localizedValueForFieldKey : key)
    }
    unsafe fn passType(&self) -> PKPassType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, passType)
    }
    unsafe fn paymentPass(&self) -> PKPaymentPass
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, paymentPass)
    }
    unsafe fn secureElementPass(&self) -> PKSecureElementPass
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, secureElementPass)
    }
    unsafe fn serialNumber(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, serialNumber)
    }
    unsafe fn passTypeIdentifier(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, passTypeIdentifier)
    }
    unsafe fn webServiceURL(&self) -> NSURL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, webServiceURL)
    }
    unsafe fn authenticationToken(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, authenticationToken)
    }
    unsafe fn localizedName(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, localizedName)
    }
    unsafe fn localizedDescription(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, localizedDescription)
    }
    unsafe fn organizationName(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, organizationName)
    }
    unsafe fn relevantDate(&self) -> NSDate
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, relevantDate)
    }
    unsafe fn relevantDates(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, relevantDates)
    }
    unsafe fn userInfo(&self) -> NSDictionary
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, userInfo)
    }
    unsafe fn passURL(&self) -> NSURL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, passURL)
    }
    unsafe fn isRemotePass(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isRemotePass)
    }
    unsafe fn deviceName(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, deviceName)
    }
}
pub type PKSecureElementPassActivationState = NSInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct PKSecureElementPass(pub id);
impl std::ops::Deref for PKSecureElementPass {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for PKSecureElementPass {}
impl PKSecureElementPass {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"PKSecureElementPass").unwrap(), alloc) })
    }
}
impl IPKPass for PKSecureElementPass {}
impl From<PKSecureElementPass> for PKPass {
    fn from(child: PKSecureElementPass) -> PKPass {
        PKPass(child.0)
    }
}
impl std::convert::TryFrom<PKPass> for PKSecureElementPass {
    type Error = &'static str;
    fn try_from(parent: PKPass) -> Result<PKSecureElementPass, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"PKSecureElementPass").unwrap()) };
        if is_kind_of {
            Ok(PKSecureElementPass(parent.0))
        } else {
            Err("This PKPass cannot be downcasted to PKSecureElementPass")
        }
    }
}
impl IPKObject for PKSecureElementPass {}
impl INSObject for PKSecureElementPass {}
impl PNSObject for PKSecureElementPass {}
impl IPKSecureElementPass for PKSecureElementPass {}
pub trait IPKSecureElementPass: Sized + std::ops::Deref {
    unsafe fn primaryAccountIdentifier(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, primaryAccountIdentifier)
    }
    unsafe fn primaryAccountNumberSuffix(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, primaryAccountNumberSuffix)
    }
    unsafe fn deviceAccountIdentifier(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, deviceAccountIdentifier)
    }
    unsafe fn deviceAccountNumberSuffix(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, deviceAccountNumberSuffix)
    }
    unsafe fn passActivationState(&self) -> PKSecureElementPassActivationState
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, passActivationState)
    }
    unsafe fn devicePassIdentifier(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, devicePassIdentifier)
    }
    unsafe fn pairedTerminalIdentifier(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, pairedTerminalIdentifier)
    }
}
pub type PKPaymentPassActivationState = NSUInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct PKPaymentPass(pub id);
impl std::ops::Deref for PKPaymentPass {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for PKPaymentPass {}
impl PKPaymentPass {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"PKPaymentPass").unwrap(), alloc) })
    }
}
impl IPKSecureElementPass for PKPaymentPass {}
impl From<PKPaymentPass> for PKSecureElementPass {
    fn from(child: PKPaymentPass) -> PKSecureElementPass {
        PKSecureElementPass(child.0)
    }
}
impl std::convert::TryFrom<PKSecureElementPass> for PKPaymentPass {
    type Error = &'static str;
    fn try_from(parent: PKSecureElementPass) -> Result<PKPaymentPass, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"PKPaymentPass").unwrap()) };
        if is_kind_of {
            Ok(PKPaymentPass(parent.0))
        } else {
            Err("This PKSecureElementPass cannot be downcasted to PKPaymentPass")
        }
    }
}
impl IPKPass for PKPaymentPass {}
impl IPKObject for PKPaymentPass {}
impl INSObject for PKPaymentPass {}
impl PNSObject for PKPaymentPass {}
impl IPKPaymentPass for PKPaymentPass {}
pub trait IPKPaymentPass: Sized + std::ops::Deref {
    unsafe fn activationState(&self) -> PKPaymentPassActivationState
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, activationState)
    }
}
pub type PKPassKitErrorCode = NSInteger;
pub type PKPaymentErrorCode = NSInteger;
pub type PKDisbursementErrorCode = NSInteger;
pub type PKAddPaymentPassError = NSInteger;
pub type PKAddSecureElementPassErrorCode = NSInteger;
pub type PKShareSecureElementPassErrorCode = NSInteger;
pub type PKPassLibraryAddPassesStatus = NSInteger;
pub type PKAutomaticPassPresentationSuppressionResult = NSUInteger;
pub type PKPassLibraryCapability = NSInteger;
pub type PKPassLibraryAuthorizationStatus = NSInteger;
pub type PKSuppressionRequestToken = NSUInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct PKPassLibrary(pub id);
impl std::ops::Deref for PKPassLibrary {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for PKPassLibrary {}
impl PKPassLibrary {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"PKPassLibrary").unwrap(), alloc) })
    }
}
impl INSObject for PKPassLibrary {}
impl PNSObject for PKPassLibrary {}
impl std::convert::TryFrom<NSObject> for PKPassLibrary {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<PKPassLibrary, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"PKPassLibrary").unwrap()) };
        if is_kind_of {
            Ok(PKPassLibrary(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to PKPassLibrary")
        }
    }
}
impl IPKPassLibrary for PKPassLibrary {}
pub trait IPKPassLibrary: Sized + std::ops::Deref {
    unsafe fn isPaymentPassActivationAvailable(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isPaymentPassActivationAvailable)
    }
    unsafe fn passes(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, passes)
    }
    unsafe fn passWithPassTypeIdentifier_serialNumber_(
        &self,
        identifier: NSString,
        serialNumber: NSString,
    ) -> PKPass
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, passWithPassTypeIdentifier : identifier, serialNumber : serialNumber)
    }
    unsafe fn passesWithReaderIdentifier_(&self, readerIdentifier: NSString) -> NSSet
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, passesWithReaderIdentifier : readerIdentifier)
    }
    unsafe fn passesOfType_(&self, passType: PKPassType) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, passesOfType : passType)
    }
    unsafe fn remotePaymentPasses(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, remotePaymentPasses)
    }
    unsafe fn removePass_(&self, pass: PKPass)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, removePass : pass)
    }
    unsafe fn containsPass_(&self, pass: PKPass) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, containsPass : pass)
    }
    unsafe fn replacePassWithPass_(&self, pass: PKPass) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, replacePassWithPass : pass)
    }
    unsafe fn addPasses_withCompletionHandler_(
        &self,
        passes: NSArray,
        completion: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, addPasses : passes, withCompletionHandler : completion)
    }
    unsafe fn openPaymentSetup(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, openPaymentSetup)
    }
    unsafe fn presentPaymentPass_(&self, pass: PKPaymentPass)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, presentPaymentPass : pass)
    }
    unsafe fn presentSecureElementPass_(&self, pass: PKSecureElementPass)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, presentSecureElementPass : pass)
    }
    unsafe fn canAddPaymentPassWithPrimaryAccountIdentifier_(
        &self,
        primaryAccountIdentifier: NSString,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, canAddPaymentPassWithPrimaryAccountIdentifier : primaryAccountIdentifier)
    }
    unsafe fn canAddSecureElementPassWithPrimaryAccountIdentifier_(
        &self,
        primaryAccountIdentifier: NSString,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, canAddSecureElementPassWithPrimaryAccountIdentifier : primaryAccountIdentifier)
    }
    unsafe fn canAddFelicaPass(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, canAddFelicaPass)
    }
    unsafe fn activatePaymentPass_withActivationData_completion_(
        &self,
        paymentPass: PKPaymentPass,
        activationData: NSData,
        completion: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, activatePaymentPass : paymentPass, withActivationData : activationData, completion : completion)
    }
    unsafe fn activatePaymentPass_withActivationCode_completion_(
        &self,
        paymentPass: PKPaymentPass,
        activationCode: NSString,
        completion: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, activatePaymentPass : paymentPass, withActivationCode : activationCode, completion : completion)
    }
    unsafe fn activateSecureElementPass_withActivationData_completion_(
        &self,
        secureElementPass: PKSecureElementPass,
        activationData: NSData,
        completion: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, activateSecureElementPass : secureElementPass, withActivationData : activationData, completion : completion)
    }
    unsafe fn signData_withSecureElementPass_completion_(
        &self,
        signData: NSData,
        secureElementPass: PKSecureElementPass,
        completion: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, signData : signData, withSecureElementPass : secureElementPass, completion : completion)
    }
    unsafe fn encryptedServiceProviderDataForSecureElementPass_completion_(
        &self,
        secureElementPass: PKSecureElementPass,
        completion: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, encryptedServiceProviderDataForSecureElementPass : secureElementPass, completion : completion)
    }
    unsafe fn serviceProviderDataForSecureElementPass_completion_(
        &self,
        secureElementPass: PKSecureElementPass,
        completion: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, serviceProviderDataForSecureElementPass : secureElementPass, completion : completion)
    }
    unsafe fn authorizationStatusForCapability_(
        &self,
        capability: PKPassLibraryCapability,
    ) -> PKPassLibraryAuthorizationStatus
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, authorizationStatusForCapability : capability)
    }
    unsafe fn requestAuthorizationForCapability_completion_(
        &self,
        capability: PKPassLibraryCapability,
        completion: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, requestAuthorizationForCapability : capability, completion : completion)
    }
    unsafe fn isSecureElementPassActivationAvailable(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isSecureElementPassActivationAvailable)
    }
    unsafe fn remoteSecureElementPasses(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, remoteSecureElementPasses)
    }
    unsafe fn isPassLibraryAvailable() -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"PKPassLibrary").unwrap(), isPassLibraryAvailable)
    }
    unsafe fn requestAutomaticPassPresentationSuppressionWithResponseHandler_(
        responseHandler: *mut ::std::os::raw::c_void,
    ) -> PKSuppressionRequestToken
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"PKPassLibrary").unwrap(), requestAutomaticPassPresentationSuppressionWithResponseHandler : responseHandler)
    }
    unsafe fn endAutomaticPassPresentationSuppressionWithRequestToken_(
        requestToken: PKSuppressionRequestToken,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"PKPassLibrary").unwrap(), endAutomaticPassPresentationSuppressionWithRequestToken : requestToken)
    }
    unsafe fn isSuppressingAutomaticPassPresentation() -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"PKPassLibrary").unwrap(), isSuppressingAutomaticPassPresentation)
    }
    unsafe fn class_isPaymentPassActivationAvailable() -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"PKPassLibrary").unwrap(), isPaymentPassActivationAvailable)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct PKLabeledValue(pub id);
impl std::ops::Deref for PKLabeledValue {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for PKLabeledValue {}
impl PKLabeledValue {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"PKLabeledValue").unwrap(), alloc) })
    }
}
impl INSObject for PKLabeledValue {}
impl PNSObject for PKLabeledValue {}
impl std::convert::TryFrom<NSObject> for PKLabeledValue {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<PKLabeledValue, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"PKLabeledValue").unwrap()) };
        if is_kind_of {
            Ok(PKLabeledValue(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to PKLabeledValue")
        }
    }
}
impl IPKLabeledValue for PKLabeledValue {}
pub trait IPKLabeledValue: Sized + std::ops::Deref {
    unsafe fn initWithLabel_value_(&self, label: NSString, value: NSString) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithLabel : label, value : value)
    }
    unsafe fn label(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, label)
    }
    unsafe fn value(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, value)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct PKContact(pub id);
impl std::ops::Deref for PKContact {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for PKContact {}
impl PKContact {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"PKContact").unwrap(), alloc) })
    }
}
impl INSObject for PKContact {}
impl PNSObject for PKContact {}
impl std::convert::TryFrom<NSObject> for PKContact {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<PKContact, Self::Error> {
        let is_kind_of: bool = unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"PKContact").unwrap()) };
        if is_kind_of {
            Ok(PKContact(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to PKContact")
        }
    }
}
impl IPKContact for PKContact {}
pub trait IPKContact: Sized + std::ops::Deref {
    unsafe fn name(&self) -> NSPersonNameComponents
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, name)
    }
    unsafe fn setName_(&self, name: NSPersonNameComponents)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setName : name)
    }
    unsafe fn postalAddress(&self) -> CNPostalAddress
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, postalAddress)
    }
    unsafe fn setPostalAddress_(&self, postalAddress: CNPostalAddress)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPostalAddress : postalAddress)
    }
    unsafe fn phoneNumber(&self) -> CNPhoneNumber
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, phoneNumber)
    }
    unsafe fn setPhoneNumber_(&self, phoneNumber: CNPhoneNumber)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPhoneNumber : phoneNumber)
    }
    unsafe fn emailAddress(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, emailAddress)
    }
    unsafe fn setEmailAddress_(&self, emailAddress: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setEmailAddress : emailAddress)
    }
    unsafe fn supplementarySubLocality(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, supplementarySubLocality)
    }
    unsafe fn setSupplementarySubLocality_(&self, supplementarySubLocality: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSupplementarySubLocality : supplementarySubLocality)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct PKStoredValuePassProperties(pub id);
impl std::ops::Deref for PKStoredValuePassProperties {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for PKStoredValuePassProperties {}
impl PKStoredValuePassProperties {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"PKStoredValuePassProperties").unwrap(), alloc) })
    }
}
impl INSObject for PKStoredValuePassProperties {}
impl PNSObject for PKStoredValuePassProperties {}
impl std::convert::TryFrom<NSObject> for PKStoredValuePassProperties {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<PKStoredValuePassProperties, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"PKStoredValuePassProperties").unwrap()) };
        if is_kind_of {
            Ok(PKStoredValuePassProperties(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to PKStoredValuePassProperties")
        }
    }
}
impl IPKStoredValuePassProperties for PKStoredValuePassProperties {}
pub trait IPKStoredValuePassProperties: Sized + std::ops::Deref {
    unsafe fn isBlacklisted(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isBlacklisted)
    }
    unsafe fn isBlocked(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isBlocked)
    }
    unsafe fn expirationDate(&self) -> NSDate
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, expirationDate)
    }
    unsafe fn balances(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, balances)
    }
    unsafe fn passPropertiesForPass_(pass: PKPass) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"PKStoredValuePassProperties").unwrap(), passPropertiesForPass : pass)
    }
}
pub type PKStoredValuePassBalanceType = NSString;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct PKStoredValuePassBalance(pub id);
impl std::ops::Deref for PKStoredValuePassBalance {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for PKStoredValuePassBalance {}
impl PKStoredValuePassBalance {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"PKStoredValuePassBalance").unwrap(), alloc) })
    }
}
impl INSObject for PKStoredValuePassBalance {}
impl PNSObject for PKStoredValuePassBalance {}
impl std::convert::TryFrom<NSObject> for PKStoredValuePassBalance {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<PKStoredValuePassBalance, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"PKStoredValuePassBalance").unwrap()) };
        if is_kind_of {
            Ok(PKStoredValuePassBalance(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to PKStoredValuePassBalance")
        }
    }
}
impl IPKStoredValuePassBalance for PKStoredValuePassBalance {}
pub trait IPKStoredValuePassBalance: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn isEqualToBalance_(&self, balance: PKStoredValuePassBalance) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, isEqualToBalance : balance)
    }
    unsafe fn amount(&self) -> NSDecimalNumber
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, amount)
    }
    unsafe fn currencyCode(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, currencyCode)
    }
    unsafe fn balanceType(&self) -> PKStoredValuePassBalanceType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, balanceType)
    }
    unsafe fn expiryDate(&self) -> NSDate
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, expiryDate)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"PKStoredValuePassBalance").unwrap(), new)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct PKTransitPassProperties(pub id);
impl std::ops::Deref for PKTransitPassProperties {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for PKTransitPassProperties {}
impl PKTransitPassProperties {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"PKTransitPassProperties").unwrap(), alloc) })
    }
}
impl IPKStoredValuePassProperties for PKTransitPassProperties {}
impl From<PKTransitPassProperties> for PKStoredValuePassProperties {
    fn from(child: PKTransitPassProperties) -> PKStoredValuePassProperties {
        PKStoredValuePassProperties(child.0)
    }
}
impl std::convert::TryFrom<PKStoredValuePassProperties> for PKTransitPassProperties {
    type Error = &'static str;
    fn try_from(
        parent: PKStoredValuePassProperties,
    ) -> Result<PKTransitPassProperties, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"PKTransitPassProperties").unwrap()) };
        if is_kind_of {
            Ok(PKTransitPassProperties(parent.0))
        } else {
            Err("This PKStoredValuePassProperties cannot be downcasted to PKTransitPassProperties")
        }
    }
}
impl INSObject for PKTransitPassProperties {}
impl PNSObject for PKTransitPassProperties {}
impl IPKTransitPassProperties for PKTransitPassProperties {}
pub trait IPKTransitPassProperties: Sized + std::ops::Deref {
    unsafe fn transitBalance(&self) -> NSDecimalNumber
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, transitBalance)
    }
    unsafe fn transitBalanceCurrencyCode(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, transitBalanceCurrencyCode)
    }
    unsafe fn isBlacklisted(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isBlacklisted)
    }
    unsafe fn expirationDate(&self) -> NSDate
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, expirationDate)
    }
    unsafe fn isBlocked(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isBlocked)
    }
    unsafe fn isInStation(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isInStation)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct PKSuicaPassProperties(pub id);
impl std::ops::Deref for PKSuicaPassProperties {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for PKSuicaPassProperties {}
impl PKSuicaPassProperties {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"PKSuicaPassProperties").unwrap(), alloc) })
    }
}
impl IPKTransitPassProperties for PKSuicaPassProperties {}
impl From<PKSuicaPassProperties> for PKTransitPassProperties {
    fn from(child: PKSuicaPassProperties) -> PKTransitPassProperties {
        PKTransitPassProperties(child.0)
    }
}
impl std::convert::TryFrom<PKTransitPassProperties> for PKSuicaPassProperties {
    type Error = &'static str;
    fn try_from(parent: PKTransitPassProperties) -> Result<PKSuicaPassProperties, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"PKSuicaPassProperties").unwrap()) };
        if is_kind_of {
            Ok(PKSuicaPassProperties(parent.0))
        } else {
            Err("This PKTransitPassProperties cannot be downcasted to PKSuicaPassProperties")
        }
    }
}
impl IPKStoredValuePassProperties for PKSuicaPassProperties {}
impl INSObject for PKSuicaPassProperties {}
impl PNSObject for PKSuicaPassProperties {}
impl IPKSuicaPassProperties for PKSuicaPassProperties {}
pub trait IPKSuicaPassProperties: Sized + std::ops::Deref {
    unsafe fn transitBalance(&self) -> NSDecimalNumber
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, transitBalance)
    }
    unsafe fn transitBalanceCurrencyCode(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, transitBalanceCurrencyCode)
    }
    unsafe fn isInStation(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isInStation)
    }
    unsafe fn isInShinkansenStation(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isInShinkansenStation)
    }
    unsafe fn isBalanceAllowedForCommute(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isBalanceAllowedForCommute)
    }
    unsafe fn isLowBalanceGateNotificationEnabled(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isLowBalanceGateNotificationEnabled)
    }
    unsafe fn isGreenCarTicketUsed(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isGreenCarTicketUsed)
    }
    unsafe fn isBlacklisted(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isBlacklisted)
    }
    unsafe fn passPropertiesForPass_(pass: PKPass) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"PKSuicaPassProperties").unwrap(), passPropertiesForPass : pass)
    }
}
pub type PKPaymentSummaryItemType = NSUInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct PKPaymentSummaryItem(pub id);
impl std::ops::Deref for PKPaymentSummaryItem {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for PKPaymentSummaryItem {}
impl PKPaymentSummaryItem {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"PKPaymentSummaryItem").unwrap(), alloc) })
    }
}
impl INSObject for PKPaymentSummaryItem {}
impl PNSObject for PKPaymentSummaryItem {}
impl std::convert::TryFrom<NSObject> for PKPaymentSummaryItem {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<PKPaymentSummaryItem, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"PKPaymentSummaryItem").unwrap()) };
        if is_kind_of {
            Ok(PKPaymentSummaryItem(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to PKPaymentSummaryItem")
        }
    }
}
impl IPKPaymentSummaryItem for PKPaymentSummaryItem {}
pub trait IPKPaymentSummaryItem: Sized + std::ops::Deref {
    unsafe fn label(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, label)
    }
    unsafe fn setLabel_(&self, label: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setLabel : label)
    }
    unsafe fn amount(&self) -> NSDecimalNumber
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, amount)
    }
    unsafe fn setAmount_(&self, amount: NSDecimalNumber)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAmount : amount)
    }
    unsafe fn type_(&self) -> PKPaymentSummaryItemType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, type)
    }
    unsafe fn setType_(&self, type_: PKPaymentSummaryItemType)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setType : type_)
    }
    unsafe fn summaryItemWithLabel_amount_(label: NSString, amount: NSDecimalNumber) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"PKPaymentSummaryItem").unwrap(), summaryItemWithLabel : label, amount : amount)
    }
    unsafe fn summaryItemWithLabel_amount_type_(
        label: NSString,
        amount: NSDecimalNumber,
        type_: PKPaymentSummaryItemType,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"PKPaymentSummaryItem").unwrap(), summaryItemWithLabel : label, amount : amount, r#type : type_)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct PKDeferredPaymentSummaryItem(pub id);
impl std::ops::Deref for PKDeferredPaymentSummaryItem {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for PKDeferredPaymentSummaryItem {}
impl PKDeferredPaymentSummaryItem {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"PKDeferredPaymentSummaryItem").unwrap(), alloc) })
    }
}
impl IPKPaymentSummaryItem for PKDeferredPaymentSummaryItem {}
impl From<PKDeferredPaymentSummaryItem> for PKPaymentSummaryItem {
    fn from(child: PKDeferredPaymentSummaryItem) -> PKPaymentSummaryItem {
        PKPaymentSummaryItem(child.0)
    }
}
impl std::convert::TryFrom<PKPaymentSummaryItem> for PKDeferredPaymentSummaryItem {
    type Error = &'static str;
    fn try_from(parent: PKPaymentSummaryItem) -> Result<PKDeferredPaymentSummaryItem, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"PKDeferredPaymentSummaryItem").unwrap()) };
        if is_kind_of {
            Ok(PKDeferredPaymentSummaryItem(parent.0))
        } else {
            Err("This PKPaymentSummaryItem cannot be downcasted to PKDeferredPaymentSummaryItem")
        }
    }
}
impl INSObject for PKDeferredPaymentSummaryItem {}
impl PNSObject for PKDeferredPaymentSummaryItem {}
impl IPKDeferredPaymentSummaryItem for PKDeferredPaymentSummaryItem {}
pub trait IPKDeferredPaymentSummaryItem: Sized + std::ops::Deref {
    unsafe fn deferredDate(&self) -> NSDate
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, deferredDate)
    }
    unsafe fn setDeferredDate_(&self, deferredDate: NSDate)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDeferredDate : deferredDate)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct PKRecurringPaymentSummaryItem(pub id);
impl std::ops::Deref for PKRecurringPaymentSummaryItem {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for PKRecurringPaymentSummaryItem {}
impl PKRecurringPaymentSummaryItem {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"PKRecurringPaymentSummaryItem").unwrap(), alloc) })
    }
}
impl IPKPaymentSummaryItem for PKRecurringPaymentSummaryItem {}
impl std::convert::TryFrom<PKPaymentSummaryItem> for PKRecurringPaymentSummaryItem {
    type Error = &'static str;
    fn try_from(
        parent: PKPaymentSummaryItem,
    ) -> Result<PKRecurringPaymentSummaryItem, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"PKRecurringPaymentSummaryItem").unwrap())
        };
        if is_kind_of {
            Ok(PKRecurringPaymentSummaryItem(parent.0))
        } else {
            Err("This PKPaymentSummaryItem cannot be downcasted to PKRecurringPaymentSummaryItem")
        }
    }
}
impl INSObject for PKRecurringPaymentSummaryItem {}
impl PNSObject for PKRecurringPaymentSummaryItem {}
impl IPKRecurringPaymentSummaryItem for PKRecurringPaymentSummaryItem {}
pub trait IPKRecurringPaymentSummaryItem: Sized + std::ops::Deref {
    unsafe fn startDate(&self) -> NSDate
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, startDate)
    }
    unsafe fn setStartDate_(&self, startDate: NSDate)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setStartDate : startDate)
    }
    unsafe fn intervalUnit(&self) -> NSCalendarUnit
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, intervalUnit)
    }
    unsafe fn setIntervalUnit_(&self, intervalUnit: NSCalendarUnit)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setIntervalUnit : intervalUnit)
    }
    unsafe fn intervalCount(&self) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, intervalCount)
    }
    unsafe fn setIntervalCount_(&self, intervalCount: NSInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setIntervalCount : intervalCount)
    }
    unsafe fn endDate(&self) -> NSDate
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, endDate)
    }
    unsafe fn setEndDate_(&self, endDate: NSDate)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setEndDate : endDate)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct PKDateComponentsRange(pub id);
impl std::ops::Deref for PKDateComponentsRange {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for PKDateComponentsRange {}
impl PKDateComponentsRange {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"PKDateComponentsRange").unwrap(), alloc) })
    }
}
impl PNSCopying for PKDateComponentsRange {}
impl PNSSecureCoding for PKDateComponentsRange {}
impl INSObject for PKDateComponentsRange {}
impl PNSObject for PKDateComponentsRange {}
impl std::convert::TryFrom<NSObject> for PKDateComponentsRange {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<PKDateComponentsRange, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"PKDateComponentsRange").unwrap()) };
        if is_kind_of {
            Ok(PKDateComponentsRange(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to PKDateComponentsRange")
        }
    }
}
impl IPKDateComponentsRange for PKDateComponentsRange {}
pub trait IPKDateComponentsRange: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn initWithStartDateComponents_endDateComponents_(
        &self,
        startDateComponents: NSDateComponents,
        endDateComponents: NSDateComponents,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithStartDateComponents : startDateComponents, endDateComponents : endDateComponents)
    }
    unsafe fn startDateComponents(&self) -> NSDateComponents
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, startDateComponents)
    }
    unsafe fn endDateComponents(&self) -> NSDateComponents
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, endDateComponents)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct PKShippingMethod(pub id);
impl std::ops::Deref for PKShippingMethod {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for PKShippingMethod {}
impl PKShippingMethod {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"PKShippingMethod").unwrap(), alloc) })
    }
}
impl IPKPaymentSummaryItem for PKShippingMethod {}
impl std::convert::TryFrom<PKPaymentSummaryItem> for PKShippingMethod {
    type Error = &'static str;
    fn try_from(parent: PKPaymentSummaryItem) -> Result<PKShippingMethod, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"PKShippingMethod").unwrap()) };
        if is_kind_of {
            Ok(PKShippingMethod(parent.0))
        } else {
            Err("This PKPaymentSummaryItem cannot be downcasted to PKShippingMethod")
        }
    }
}
impl INSObject for PKShippingMethod {}
impl PNSObject for PKShippingMethod {}
impl IPKShippingMethod for PKShippingMethod {}
pub trait IPKShippingMethod: Sized + std::ops::Deref {
    unsafe fn identifier(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, identifier)
    }
    unsafe fn setIdentifier_(&self, identifier: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setIdentifier : identifier)
    }
    unsafe fn detail(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, detail)
    }
    unsafe fn setDetail_(&self, detail: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDetail : detail)
    }
    unsafe fn dateComponentsRange(&self) -> PKDateComponentsRange
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, dateComponentsRange)
    }
    unsafe fn setDateComponentsRange_(&self, dateComponentsRange: PKDateComponentsRange)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDateComponentsRange : dateComponentsRange)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct PKAutomaticReloadPaymentRequest(pub id);
impl std::ops::Deref for PKAutomaticReloadPaymentRequest {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for PKAutomaticReloadPaymentRequest {}
impl PKAutomaticReloadPaymentRequest {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"PKAutomaticReloadPaymentRequest").unwrap(), alloc) })
    }
}
impl INSObject for PKAutomaticReloadPaymentRequest {}
impl PNSObject for PKAutomaticReloadPaymentRequest {}
impl std::convert::TryFrom<NSObject> for PKAutomaticReloadPaymentRequest {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<PKAutomaticReloadPaymentRequest, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"PKAutomaticReloadPaymentRequest").unwrap())
        };
        if is_kind_of {
            Ok(PKAutomaticReloadPaymentRequest(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to PKAutomaticReloadPaymentRequest")
        }
    }
}
impl IPKAutomaticReloadPaymentRequest for PKAutomaticReloadPaymentRequest {}
pub trait IPKAutomaticReloadPaymentRequest: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn initWithPaymentDescription_automaticReloadBilling_managementURL_(
        &self,
        paymentDescription: NSString,
        automaticReloadBilling: PKAutomaticReloadPaymentSummaryItem,
        managementURL: NSURL,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithPaymentDescription : paymentDescription, automaticReloadBilling : automaticReloadBilling, managementURL : managementURL)
    }
    unsafe fn paymentDescription(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, paymentDescription)
    }
    unsafe fn setPaymentDescription_(&self, paymentDescription: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPaymentDescription : paymentDescription)
    }
    unsafe fn automaticReloadBilling(&self) -> PKAutomaticReloadPaymentSummaryItem
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, automaticReloadBilling)
    }
    unsafe fn setAutomaticReloadBilling_(
        &self,
        automaticReloadBilling: PKAutomaticReloadPaymentSummaryItem,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAutomaticReloadBilling : automaticReloadBilling)
    }
    unsafe fn billingAgreement(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, billingAgreement)
    }
    unsafe fn setBillingAgreement_(&self, billingAgreement: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setBillingAgreement : billingAgreement)
    }
    unsafe fn managementURL(&self) -> NSURL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, managementURL)
    }
    unsafe fn setManagementURL_(&self, managementURL: NSURL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setManagementURL : managementURL)
    }
    unsafe fn tokenNotificationURL(&self) -> NSURL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, tokenNotificationURL)
    }
    unsafe fn setTokenNotificationURL_(&self, tokenNotificationURL: NSURL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setTokenNotificationURL : tokenNotificationURL)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct PKRecurringPaymentRequest(pub id);
impl std::ops::Deref for PKRecurringPaymentRequest {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for PKRecurringPaymentRequest {}
impl PKRecurringPaymentRequest {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"PKRecurringPaymentRequest").unwrap(), alloc) })
    }
}
impl INSObject for PKRecurringPaymentRequest {}
impl PNSObject for PKRecurringPaymentRequest {}
impl std::convert::TryFrom<NSObject> for PKRecurringPaymentRequest {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<PKRecurringPaymentRequest, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"PKRecurringPaymentRequest").unwrap()) };
        if is_kind_of {
            Ok(PKRecurringPaymentRequest(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to PKRecurringPaymentRequest")
        }
    }
}
impl IPKRecurringPaymentRequest for PKRecurringPaymentRequest {}
pub trait IPKRecurringPaymentRequest: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn initWithPaymentDescription_regularBilling_managementURL_(
        &self,
        paymentDescription: NSString,
        regularBilling: PKRecurringPaymentSummaryItem,
        managementURL: NSURL,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithPaymentDescription : paymentDescription, regularBilling : regularBilling, managementURL : managementURL)
    }
    unsafe fn paymentDescription(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, paymentDescription)
    }
    unsafe fn setPaymentDescription_(&self, paymentDescription: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPaymentDescription : paymentDescription)
    }
    unsafe fn regularBilling(&self) -> PKRecurringPaymentSummaryItem
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, regularBilling)
    }
    unsafe fn setRegularBilling_(&self, regularBilling: PKRecurringPaymentSummaryItem)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRegularBilling : regularBilling)
    }
    unsafe fn trialBilling(&self) -> PKRecurringPaymentSummaryItem
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, trialBilling)
    }
    unsafe fn setTrialBilling_(&self, trialBilling: PKRecurringPaymentSummaryItem)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setTrialBilling : trialBilling)
    }
    unsafe fn billingAgreement(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, billingAgreement)
    }
    unsafe fn setBillingAgreement_(&self, billingAgreement: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setBillingAgreement : billingAgreement)
    }
    unsafe fn managementURL(&self) -> NSURL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, managementURL)
    }
    unsafe fn setManagementURL_(&self, managementURL: NSURL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setManagementURL : managementURL)
    }
    unsafe fn tokenNotificationURL(&self) -> NSURL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, tokenNotificationURL)
    }
    unsafe fn setTokenNotificationURL_(&self, tokenNotificationURL: NSURL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setTokenNotificationURL : tokenNotificationURL)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct PKDeferredPaymentRequest(pub id);
impl std::ops::Deref for PKDeferredPaymentRequest {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for PKDeferredPaymentRequest {}
impl PKDeferredPaymentRequest {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"PKDeferredPaymentRequest").unwrap(), alloc) })
    }
}
impl INSObject for PKDeferredPaymentRequest {}
impl PNSObject for PKDeferredPaymentRequest {}
impl std::convert::TryFrom<NSObject> for PKDeferredPaymentRequest {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<PKDeferredPaymentRequest, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"PKDeferredPaymentRequest").unwrap()) };
        if is_kind_of {
            Ok(PKDeferredPaymentRequest(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to PKDeferredPaymentRequest")
        }
    }
}
impl IPKDeferredPaymentRequest for PKDeferredPaymentRequest {}
pub trait IPKDeferredPaymentRequest: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn initWithPaymentDescription_deferredBilling_managementURL_(
        &self,
        paymentDescription: NSString,
        deferredBilling: PKDeferredPaymentSummaryItem,
        managementURL: NSURL,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithPaymentDescription : paymentDescription, deferredBilling : deferredBilling, managementURL : managementURL)
    }
    unsafe fn paymentDescription(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, paymentDescription)
    }
    unsafe fn setPaymentDescription_(&self, paymentDescription: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPaymentDescription : paymentDescription)
    }
    unsafe fn deferredBilling(&self) -> PKDeferredPaymentSummaryItem
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, deferredBilling)
    }
    unsafe fn setDeferredBilling_(&self, deferredBilling: PKDeferredPaymentSummaryItem)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDeferredBilling : deferredBilling)
    }
    unsafe fn billingAgreement(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, billingAgreement)
    }
    unsafe fn setBillingAgreement_(&self, billingAgreement: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setBillingAgreement : billingAgreement)
    }
    unsafe fn managementURL(&self) -> NSURL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, managementURL)
    }
    unsafe fn setManagementURL_(&self, managementURL: NSURL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setManagementURL : managementURL)
    }
    unsafe fn tokenNotificationURL(&self) -> NSURL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, tokenNotificationURL)
    }
    unsafe fn setTokenNotificationURL_(&self, tokenNotificationURL: NSURL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setTokenNotificationURL : tokenNotificationURL)
    }
    unsafe fn freeCancellationDate(&self) -> NSDate
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, freeCancellationDate)
    }
    unsafe fn setFreeCancellationDate_(&self, freeCancellationDate: NSDate)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setFreeCancellationDate : freeCancellationDate)
    }
    unsafe fn freeCancellationDateTimeZone(&self) -> NSTimeZone
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, freeCancellationDateTimeZone)
    }
    unsafe fn setFreeCancellationDateTimeZone_(&self, freeCancellationDateTimeZone: NSTimeZone)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setFreeCancellationDateTimeZone : freeCancellationDateTimeZone)
    }
}
pub type PKMerchantCapability = NSUInteger;
pub type PKMerchantCategoryCode = SInt16;
pub type PKAddressField = NSUInteger;
pub type PKShippingType = NSUInteger;
pub type PKShippingContactEditingMode = NSUInteger;
pub type PKApplePayLaterAvailability = NSInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct PKPaymentRequest(pub id);
impl std::ops::Deref for PKPaymentRequest {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for PKPaymentRequest {}
impl PKPaymentRequest {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"PKPaymentRequest").unwrap(), alloc) })
    }
}
impl INSObject for PKPaymentRequest {}
impl PNSObject for PKPaymentRequest {}
impl std::convert::TryFrom<NSObject> for PKPaymentRequest {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<PKPaymentRequest, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"PKPaymentRequest").unwrap()) };
        if is_kind_of {
            Ok(PKPaymentRequest(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to PKPaymentRequest")
        }
    }
}
impl IPKPaymentRequest for PKPaymentRequest {}
pub trait IPKPaymentRequest: Sized + std::ops::Deref {
    unsafe fn merchantIdentifier(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, merchantIdentifier)
    }
    unsafe fn setMerchantIdentifier_(&self, merchantIdentifier: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMerchantIdentifier : merchantIdentifier)
    }
    unsafe fn attributionIdentifier(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, attributionIdentifier)
    }
    unsafe fn setAttributionIdentifier_(&self, attributionIdentifier: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAttributionIdentifier : attributionIdentifier)
    }
    unsafe fn countryCode(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, countryCode)
    }
    unsafe fn setCountryCode_(&self, countryCode: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCountryCode : countryCode)
    }
    unsafe fn supportedNetworks(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, supportedNetworks)
    }
    unsafe fn setSupportedNetworks_(&self, supportedNetworks: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSupportedNetworks : supportedNetworks)
    }
    unsafe fn merchantCapabilities(&self) -> PKMerchantCapability
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, merchantCapabilities)
    }
    unsafe fn setMerchantCapabilities_(&self, merchantCapabilities: PKMerchantCapability)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMerchantCapabilities : merchantCapabilities)
    }
    unsafe fn supportsCouponCode(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, supportsCouponCode)
    }
    unsafe fn setSupportsCouponCode_(&self, supportsCouponCode: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSupportsCouponCode : supportsCouponCode)
    }
    unsafe fn couponCode(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, couponCode)
    }
    unsafe fn setCouponCode_(&self, couponCode: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCouponCode : couponCode)
    }
    unsafe fn merchantCategoryCode(&self) -> PKMerchantCategoryCode
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, merchantCategoryCode)
    }
    unsafe fn setMerchantCategoryCode_(&self, merchantCategoryCode: PKMerchantCategoryCode)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMerchantCategoryCode : merchantCategoryCode)
    }
    unsafe fn paymentSummaryItems(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, paymentSummaryItems)
    }
    unsafe fn setPaymentSummaryItems_(&self, paymentSummaryItems: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPaymentSummaryItems : paymentSummaryItems)
    }
    unsafe fn currencyCode(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, currencyCode)
    }
    unsafe fn setCurrencyCode_(&self, currencyCode: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCurrencyCode : currencyCode)
    }
    unsafe fn requiredBillingContactFields(&self) -> NSSet
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, requiredBillingContactFields)
    }
    unsafe fn setRequiredBillingContactFields_(&self, requiredBillingContactFields: NSSet)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRequiredBillingContactFields : requiredBillingContactFields)
    }
    unsafe fn requiredBillingAddressFields(&self) -> PKAddressField
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, requiredBillingAddressFields)
    }
    unsafe fn setRequiredBillingAddressFields_(&self, requiredBillingAddressFields: PKAddressField)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRequiredBillingAddressFields : requiredBillingAddressFields)
    }
    unsafe fn billingContact(&self) -> PKContact
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, billingContact)
    }
    unsafe fn setBillingContact_(&self, billingContact: PKContact)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setBillingContact : billingContact)
    }
    unsafe fn requiredShippingContactFields(&self) -> NSSet
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, requiredShippingContactFields)
    }
    unsafe fn setRequiredShippingContactFields_(&self, requiredShippingContactFields: NSSet)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRequiredShippingContactFields : requiredShippingContactFields)
    }
    unsafe fn requiredShippingAddressFields(&self) -> PKAddressField
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, requiredShippingAddressFields)
    }
    unsafe fn setRequiredShippingAddressFields_(
        &self,
        requiredShippingAddressFields: PKAddressField,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRequiredShippingAddressFields : requiredShippingAddressFields)
    }
    unsafe fn shippingContact(&self) -> PKContact
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, shippingContact)
    }
    unsafe fn setShippingContact_(&self, shippingContact: PKContact)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setShippingContact : shippingContact)
    }
    unsafe fn shippingMethods(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, shippingMethods)
    }
    unsafe fn setShippingMethods_(&self, shippingMethods: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setShippingMethods : shippingMethods)
    }
    unsafe fn shippingType(&self) -> PKShippingType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, shippingType)
    }
    unsafe fn setShippingType_(&self, shippingType: PKShippingType)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setShippingType : shippingType)
    }
    unsafe fn shippingContactEditingMode(&self) -> PKShippingContactEditingMode
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, shippingContactEditingMode)
    }
    unsafe fn setShippingContactEditingMode_(
        &self,
        shippingContactEditingMode: PKShippingContactEditingMode,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setShippingContactEditingMode : shippingContactEditingMode)
    }
    unsafe fn applicationData(&self) -> NSData
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, applicationData)
    }
    unsafe fn setApplicationData_(&self, applicationData: NSData)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setApplicationData : applicationData)
    }
    unsafe fn supportedCountries(&self) -> NSSet
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, supportedCountries)
    }
    unsafe fn setSupportedCountries_(&self, supportedCountries: NSSet)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSupportedCountries : supportedCountries)
    }
    unsafe fn multiTokenContexts(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, multiTokenContexts)
    }
    unsafe fn setMultiTokenContexts_(&self, multiTokenContexts: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMultiTokenContexts : multiTokenContexts)
    }
    unsafe fn recurringPaymentRequest(&self) -> PKRecurringPaymentRequest
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, recurringPaymentRequest)
    }
    unsafe fn setRecurringPaymentRequest_(&self, recurringPaymentRequest: PKRecurringPaymentRequest)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRecurringPaymentRequest : recurringPaymentRequest)
    }
    unsafe fn automaticReloadPaymentRequest(&self) -> PKAutomaticReloadPaymentRequest
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, automaticReloadPaymentRequest)
    }
    unsafe fn setAutomaticReloadPaymentRequest_(
        &self,
        automaticReloadPaymentRequest: PKAutomaticReloadPaymentRequest,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAutomaticReloadPaymentRequest : automaticReloadPaymentRequest)
    }
    unsafe fn deferredPaymentRequest(&self) -> PKDeferredPaymentRequest
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, deferredPaymentRequest)
    }
    unsafe fn setDeferredPaymentRequest_(&self, deferredPaymentRequest: PKDeferredPaymentRequest)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDeferredPaymentRequest : deferredPaymentRequest)
    }
    unsafe fn applePayLaterAvailability(&self) -> PKApplePayLaterAvailability
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, applePayLaterAvailability)
    }
    unsafe fn setApplePayLaterAvailability_(
        &self,
        applePayLaterAvailability: PKApplePayLaterAvailability,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setApplePayLaterAvailability : applePayLaterAvailability)
    }
    unsafe fn availableNetworks() -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"PKPaymentRequest").unwrap(), availableNetworks)
    }
    unsafe fn paymentContactInvalidErrorWithContactField_localizedDescription_(
        field: NSString,
        localizedDescription: NSString,
    ) -> NSError
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"PKPaymentRequest").unwrap(), paymentContactInvalidErrorWithContactField : field, localizedDescription : localizedDescription)
    }
    unsafe fn paymentShippingAddressInvalidErrorWithKey_localizedDescription_(
        postalAddressKey: NSString,
        localizedDescription: NSString,
    ) -> NSError
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"PKPaymentRequest").unwrap(), paymentShippingAddressInvalidErrorWithKey : postalAddressKey, localizedDescription : localizedDescription)
    }
    unsafe fn paymentBillingAddressInvalidErrorWithKey_localizedDescription_(
        postalAddressKey: NSString,
        localizedDescription: NSString,
    ) -> NSError
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"PKPaymentRequest").unwrap(), paymentBillingAddressInvalidErrorWithKey : postalAddressKey, localizedDescription : localizedDescription)
    }
    unsafe fn paymentShippingAddressUnserviceableErrorWithLocalizedDescription_(
        localizedDescription: NSString,
    ) -> NSError
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"PKPaymentRequest").unwrap(), paymentShippingAddressUnserviceableErrorWithLocalizedDescription : localizedDescription)
    }
    unsafe fn paymentCouponCodeInvalidErrorWithLocalizedDescription_(
        localizedDescription: NSString,
    ) -> NSError
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"PKPaymentRequest").unwrap(), paymentCouponCodeInvalidErrorWithLocalizedDescription : localizedDescription)
    }
    unsafe fn paymentCouponCodeExpiredErrorWithLocalizedDescription_(
        localizedDescription: NSString,
    ) -> NSError
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"PKPaymentRequest").unwrap(), paymentCouponCodeExpiredErrorWithLocalizedDescription : localizedDescription)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct PKAutomaticReloadPaymentSummaryItem(pub id);
impl std::ops::Deref for PKAutomaticReloadPaymentSummaryItem {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for PKAutomaticReloadPaymentSummaryItem {}
impl PKAutomaticReloadPaymentSummaryItem {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"PKAutomaticReloadPaymentSummaryItem").unwrap(), alloc) })
    }
}
impl IPKPaymentSummaryItem for PKAutomaticReloadPaymentSummaryItem {}
impl std::convert::TryFrom<PKPaymentSummaryItem> for PKAutomaticReloadPaymentSummaryItem {
    type Error = &'static str;
    fn try_from(
        parent: PKPaymentSummaryItem,
    ) -> Result<PKAutomaticReloadPaymentSummaryItem, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"PKAutomaticReloadPaymentSummaryItem").unwrap())
        };
        if is_kind_of {
            Ok(PKAutomaticReloadPaymentSummaryItem(parent.0))
        } else {
            Err ("This PKPaymentSummaryItem cannot be downcasted to PKAutomaticReloadPaymentSummaryItem" ,)
        }
    }
}
impl INSObject for PKAutomaticReloadPaymentSummaryItem {}
impl PNSObject for PKAutomaticReloadPaymentSummaryItem {}
impl IPKAutomaticReloadPaymentSummaryItem for PKAutomaticReloadPaymentSummaryItem {}
pub trait IPKAutomaticReloadPaymentSummaryItem: Sized + std::ops::Deref {
    unsafe fn thresholdAmount(&self) -> NSDecimalNumber
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, thresholdAmount)
    }
    unsafe fn setThresholdAmount_(&self, thresholdAmount: NSDecimalNumber)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setThresholdAmount : thresholdAmount)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct PKPaymentTokenContext(pub id);
impl std::ops::Deref for PKPaymentTokenContext {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for PKPaymentTokenContext {}
impl PKPaymentTokenContext {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"PKPaymentTokenContext").unwrap(), alloc) })
    }
}
impl INSObject for PKPaymentTokenContext {}
impl PNSObject for PKPaymentTokenContext {}
impl std::convert::TryFrom<NSObject> for PKPaymentTokenContext {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<PKPaymentTokenContext, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"PKPaymentTokenContext").unwrap()) };
        if is_kind_of {
            Ok(PKPaymentTokenContext(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to PKPaymentTokenContext")
        }
    }
}
impl IPKPaymentTokenContext for PKPaymentTokenContext {}
pub trait IPKPaymentTokenContext: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn initWithMerchantIdentifier_externalIdentifier_merchantName_merchantDomain_amount_(
        &self,
        merchantIdentifier: NSString,
        externalIdentifier: NSString,
        merchantName: NSString,
        merchantDomain: NSString,
        amount: NSDecimalNumber,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithMerchantIdentifier : merchantIdentifier, externalIdentifier : externalIdentifier, merchantName : merchantName, merchantDomain : merchantDomain, amount : amount)
    }
    unsafe fn merchantIdentifier(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, merchantIdentifier)
    }
    unsafe fn setMerchantIdentifier_(&self, merchantIdentifier: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMerchantIdentifier : merchantIdentifier)
    }
    unsafe fn externalIdentifier(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, externalIdentifier)
    }
    unsafe fn setExternalIdentifier_(&self, externalIdentifier: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setExternalIdentifier : externalIdentifier)
    }
    unsafe fn merchantName(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, merchantName)
    }
    unsafe fn setMerchantName_(&self, merchantName: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMerchantName : merchantName)
    }
    unsafe fn merchantDomain(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, merchantDomain)
    }
    unsafe fn setMerchantDomain_(&self, merchantDomain: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMerchantDomain : merchantDomain)
    }
    unsafe fn amount(&self) -> NSDecimalNumber
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, amount)
    }
    unsafe fn setAmount_(&self, amount: NSDecimalNumber)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAmount : amount)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct PKPaymentAuthorizationResult(pub id);
impl std::ops::Deref for PKPaymentAuthorizationResult {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for PKPaymentAuthorizationResult {}
impl PKPaymentAuthorizationResult {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"PKPaymentAuthorizationResult").unwrap(), alloc) })
    }
}
impl INSObject for PKPaymentAuthorizationResult {}
impl PNSObject for PKPaymentAuthorizationResult {}
impl std::convert::TryFrom<NSObject> for PKPaymentAuthorizationResult {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<PKPaymentAuthorizationResult, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"PKPaymentAuthorizationResult").unwrap()) };
        if is_kind_of {
            Ok(PKPaymentAuthorizationResult(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to PKPaymentAuthorizationResult")
        }
    }
}
impl IPKPaymentAuthorizationResult for PKPaymentAuthorizationResult {}
pub trait IPKPaymentAuthorizationResult: Sized + std::ops::Deref {
    unsafe fn initWithStatus_errors_(
        &self,
        status: PKPaymentAuthorizationStatus,
        errors: NSArray,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithStatus : status, errors : errors)
    }
    unsafe fn status(&self) -> PKPaymentAuthorizationStatus
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, status)
    }
    unsafe fn setStatus_(&self, status: PKPaymentAuthorizationStatus)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setStatus : status)
    }
    unsafe fn errors(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, errors)
    }
    unsafe fn setErrors_(&self, errors: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setErrors : errors)
    }
    unsafe fn orderDetails(&self) -> PKPaymentOrderDetails
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, orderDetails)
    }
    unsafe fn setOrderDetails_(&self, orderDetails: PKPaymentOrderDetails)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setOrderDetails : orderDetails)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct PKPaymentRequestUpdate(pub id);
impl std::ops::Deref for PKPaymentRequestUpdate {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for PKPaymentRequestUpdate {}
impl PKPaymentRequestUpdate {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"PKPaymentRequestUpdate").unwrap(), alloc) })
    }
}
impl INSObject for PKPaymentRequestUpdate {}
impl PNSObject for PKPaymentRequestUpdate {}
impl std::convert::TryFrom<NSObject> for PKPaymentRequestUpdate {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<PKPaymentRequestUpdate, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"PKPaymentRequestUpdate").unwrap()) };
        if is_kind_of {
            Ok(PKPaymentRequestUpdate(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to PKPaymentRequestUpdate")
        }
    }
}
impl IPKPaymentRequestUpdate for PKPaymentRequestUpdate {}
pub trait IPKPaymentRequestUpdate: Sized + std::ops::Deref {
    unsafe fn initWithPaymentSummaryItems_(&self, paymentSummaryItems: NSArray) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithPaymentSummaryItems : paymentSummaryItems)
    }
    unsafe fn status(&self) -> PKPaymentAuthorizationStatus
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, status)
    }
    unsafe fn setStatus_(&self, status: PKPaymentAuthorizationStatus)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setStatus : status)
    }
    unsafe fn paymentSummaryItems(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, paymentSummaryItems)
    }
    unsafe fn setPaymentSummaryItems_(&self, paymentSummaryItems: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPaymentSummaryItems : paymentSummaryItems)
    }
    unsafe fn shippingMethods(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, shippingMethods)
    }
    unsafe fn setShippingMethods_(&self, shippingMethods: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setShippingMethods : shippingMethods)
    }
    unsafe fn multiTokenContexts(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, multiTokenContexts)
    }
    unsafe fn setMultiTokenContexts_(&self, multiTokenContexts: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMultiTokenContexts : multiTokenContexts)
    }
    unsafe fn recurringPaymentRequest(&self) -> PKRecurringPaymentRequest
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, recurringPaymentRequest)
    }
    unsafe fn setRecurringPaymentRequest_(&self, recurringPaymentRequest: PKRecurringPaymentRequest)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRecurringPaymentRequest : recurringPaymentRequest)
    }
    unsafe fn automaticReloadPaymentRequest(&self) -> PKAutomaticReloadPaymentRequest
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, automaticReloadPaymentRequest)
    }
    unsafe fn setAutomaticReloadPaymentRequest_(
        &self,
        automaticReloadPaymentRequest: PKAutomaticReloadPaymentRequest,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAutomaticReloadPaymentRequest : automaticReloadPaymentRequest)
    }
    unsafe fn deferredPaymentRequest(&self) -> PKDeferredPaymentRequest
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, deferredPaymentRequest)
    }
    unsafe fn setDeferredPaymentRequest_(&self, deferredPaymentRequest: PKDeferredPaymentRequest)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDeferredPaymentRequest : deferredPaymentRequest)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct PKPaymentRequestShippingContactUpdate(pub id);
impl std::ops::Deref for PKPaymentRequestShippingContactUpdate {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for PKPaymentRequestShippingContactUpdate {}
impl PKPaymentRequestShippingContactUpdate {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"PKPaymentRequestShippingContactUpdate").unwrap(), alloc) })
    }
}
impl IPKPaymentRequestUpdate for PKPaymentRequestShippingContactUpdate {}
impl From<PKPaymentRequestShippingContactUpdate> for PKPaymentRequestUpdate {
    fn from(child: PKPaymentRequestShippingContactUpdate) -> PKPaymentRequestUpdate {
        PKPaymentRequestUpdate(child.0)
    }
}
impl std::convert::TryFrom<PKPaymentRequestUpdate> for PKPaymentRequestShippingContactUpdate {
    type Error = &'static str;
    fn try_from(
        parent: PKPaymentRequestUpdate,
    ) -> Result<PKPaymentRequestShippingContactUpdate, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"PKPaymentRequestShippingContactUpdate").unwrap())
        };
        if is_kind_of {
            Ok(PKPaymentRequestShippingContactUpdate(parent.0))
        } else {
            Err ("This PKPaymentRequestUpdate cannot be downcasted to PKPaymentRequestShippingContactUpdate" ,)
        }
    }
}
impl INSObject for PKPaymentRequestShippingContactUpdate {}
impl PNSObject for PKPaymentRequestShippingContactUpdate {}
impl IPKPaymentRequestShippingContactUpdate for PKPaymentRequestShippingContactUpdate {}
pub trait IPKPaymentRequestShippingContactUpdate: Sized + std::ops::Deref {
    unsafe fn initWithErrors_paymentSummaryItems_shippingMethods_(
        &self,
        errors: NSArray,
        paymentSummaryItems: NSArray,
        shippingMethods: NSArray,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithErrors : errors, paymentSummaryItems : paymentSummaryItems, shippingMethods : shippingMethods)
    }
    unsafe fn shippingMethods(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, shippingMethods)
    }
    unsafe fn setShippingMethods_(&self, shippingMethods: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setShippingMethods : shippingMethods)
    }
    unsafe fn errors(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, errors)
    }
    unsafe fn setErrors_(&self, errors: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setErrors : errors)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct PKPaymentRequestShippingMethodUpdate(pub id);
impl std::ops::Deref for PKPaymentRequestShippingMethodUpdate {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for PKPaymentRequestShippingMethodUpdate {}
impl PKPaymentRequestShippingMethodUpdate {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"PKPaymentRequestShippingMethodUpdate").unwrap(), alloc) })
    }
}
impl IPKPaymentRequestUpdate for PKPaymentRequestShippingMethodUpdate {}
impl std::convert::TryFrom<PKPaymentRequestUpdate> for PKPaymentRequestShippingMethodUpdate {
    type Error = &'static str;
    fn try_from(
        parent: PKPaymentRequestUpdate,
    ) -> Result<PKPaymentRequestShippingMethodUpdate, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"PKPaymentRequestShippingMethodUpdate").unwrap())
        };
        if is_kind_of {
            Ok(PKPaymentRequestShippingMethodUpdate(parent.0))
        } else {
            Err ("This PKPaymentRequestUpdate cannot be downcasted to PKPaymentRequestShippingMethodUpdate" ,)
        }
    }
}
impl INSObject for PKPaymentRequestShippingMethodUpdate {}
impl PNSObject for PKPaymentRequestShippingMethodUpdate {}
impl IPKPaymentRequestShippingMethodUpdate for PKPaymentRequestShippingMethodUpdate {}
pub trait IPKPaymentRequestShippingMethodUpdate: Sized + std::ops::Deref {}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct PKPaymentRequestPaymentMethodUpdate(pub id);
impl std::ops::Deref for PKPaymentRequestPaymentMethodUpdate {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for PKPaymentRequestPaymentMethodUpdate {}
impl PKPaymentRequestPaymentMethodUpdate {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"PKPaymentRequestPaymentMethodUpdate").unwrap(), alloc) })
    }
}
impl IPKPaymentRequestUpdate for PKPaymentRequestPaymentMethodUpdate {}
impl std::convert::TryFrom<PKPaymentRequestUpdate> for PKPaymentRequestPaymentMethodUpdate {
    type Error = &'static str;
    fn try_from(
        parent: PKPaymentRequestUpdate,
    ) -> Result<PKPaymentRequestPaymentMethodUpdate, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"PKPaymentRequestPaymentMethodUpdate").unwrap())
        };
        if is_kind_of {
            Ok(PKPaymentRequestPaymentMethodUpdate(parent.0))
        } else {
            Err ("This PKPaymentRequestUpdate cannot be downcasted to PKPaymentRequestPaymentMethodUpdate" ,)
        }
    }
}
impl INSObject for PKPaymentRequestPaymentMethodUpdate {}
impl PNSObject for PKPaymentRequestPaymentMethodUpdate {}
impl IPKPaymentRequestPaymentMethodUpdate for PKPaymentRequestPaymentMethodUpdate {}
pub trait IPKPaymentRequestPaymentMethodUpdate: Sized + std::ops::Deref {
    unsafe fn initWithErrors_paymentSummaryItems_(
        &self,
        errors: NSArray,
        paymentSummaryItems: NSArray,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithErrors : errors, paymentSummaryItems : paymentSummaryItems)
    }
    unsafe fn errors(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, errors)
    }
    unsafe fn setErrors_(&self, errors: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setErrors : errors)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct PKPaymentRequestMerchantSessionUpdate(pub id);
impl std::ops::Deref for PKPaymentRequestMerchantSessionUpdate {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for PKPaymentRequestMerchantSessionUpdate {}
impl PKPaymentRequestMerchantSessionUpdate {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"PKPaymentRequestMerchantSessionUpdate").unwrap(), alloc) })
    }
}
impl INSObject for PKPaymentRequestMerchantSessionUpdate {}
impl PNSObject for PKPaymentRequestMerchantSessionUpdate {}
impl std::convert::TryFrom<NSObject> for PKPaymentRequestMerchantSessionUpdate {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<PKPaymentRequestMerchantSessionUpdate, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"PKPaymentRequestMerchantSessionUpdate").unwrap())
        };
        if is_kind_of {
            Ok(PKPaymentRequestMerchantSessionUpdate(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to PKPaymentRequestMerchantSessionUpdate")
        }
    }
}
impl IPKPaymentRequestMerchantSessionUpdate for PKPaymentRequestMerchantSessionUpdate {}
pub trait IPKPaymentRequestMerchantSessionUpdate: Sized + std::ops::Deref {
    unsafe fn initWithStatus_merchantSession_(
        &self,
        status: PKPaymentAuthorizationStatus,
        session: PKPaymentMerchantSession,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithStatus : status, merchantSession : session)
    }
    unsafe fn status(&self) -> PKPaymentAuthorizationStatus
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, status)
    }
    unsafe fn setStatus_(&self, status: PKPaymentAuthorizationStatus)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setStatus : status)
    }
    unsafe fn session(&self) -> PKPaymentMerchantSession
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, session)
    }
    unsafe fn setSession_(&self, session: PKPaymentMerchantSession)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSession : session)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct PKPaymentRequestCouponCodeUpdate(pub id);
impl std::ops::Deref for PKPaymentRequestCouponCodeUpdate {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for PKPaymentRequestCouponCodeUpdate {}
impl PKPaymentRequestCouponCodeUpdate {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"PKPaymentRequestCouponCodeUpdate").unwrap(), alloc) })
    }
}
impl IPKPaymentRequestUpdate for PKPaymentRequestCouponCodeUpdate {}
impl std::convert::TryFrom<PKPaymentRequestUpdate> for PKPaymentRequestCouponCodeUpdate {
    type Error = &'static str;
    fn try_from(
        parent: PKPaymentRequestUpdate,
    ) -> Result<PKPaymentRequestCouponCodeUpdate, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"PKPaymentRequestCouponCodeUpdate").unwrap())
        };
        if is_kind_of {
            Ok(PKPaymentRequestCouponCodeUpdate(parent.0))
        } else {
            Err ("This PKPaymentRequestUpdate cannot be downcasted to PKPaymentRequestCouponCodeUpdate" ,)
        }
    }
}
impl INSObject for PKPaymentRequestCouponCodeUpdate {}
impl PNSObject for PKPaymentRequestCouponCodeUpdate {}
impl IPKPaymentRequestCouponCodeUpdate for PKPaymentRequestCouponCodeUpdate {}
pub trait IPKPaymentRequestCouponCodeUpdate: Sized + std::ops::Deref {
    unsafe fn initWithErrors_paymentSummaryItems_shippingMethods_(
        &self,
        errors: NSArray,
        paymentSummaryItems: NSArray,
        shippingMethods: NSArray,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithErrors : errors, paymentSummaryItems : paymentSummaryItems, shippingMethods : shippingMethods)
    }
    unsafe fn errors(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, errors)
    }
    unsafe fn setErrors_(&self, errors: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setErrors : errors)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct PKPaymentOrderDetails(pub id);
impl std::ops::Deref for PKPaymentOrderDetails {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for PKPaymentOrderDetails {}
impl PKPaymentOrderDetails {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"PKPaymentOrderDetails").unwrap(), alloc) })
    }
}
impl INSObject for PKPaymentOrderDetails {}
impl PNSObject for PKPaymentOrderDetails {}
impl std::convert::TryFrom<NSObject> for PKPaymentOrderDetails {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<PKPaymentOrderDetails, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"PKPaymentOrderDetails").unwrap()) };
        if is_kind_of {
            Ok(PKPaymentOrderDetails(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to PKPaymentOrderDetails")
        }
    }
}
impl IPKPaymentOrderDetails for PKPaymentOrderDetails {}
pub trait IPKPaymentOrderDetails: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn initWithOrderTypeIdentifier_orderIdentifier_webServiceURL_authenticationToken_(
        &self,
        orderTypeIdentifier: NSString,
        orderIdentifier: NSString,
        webServiceURL: NSURL,
        authenticationToken: NSString,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithOrderTypeIdentifier : orderTypeIdentifier, orderIdentifier : orderIdentifier, webServiceURL : webServiceURL, authenticationToken : authenticationToken)
    }
    unsafe fn orderTypeIdentifier(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, orderTypeIdentifier)
    }
    unsafe fn setOrderTypeIdentifier_(&self, orderTypeIdentifier: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setOrderTypeIdentifier : orderTypeIdentifier)
    }
    unsafe fn orderIdentifier(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, orderIdentifier)
    }
    unsafe fn setOrderIdentifier_(&self, orderIdentifier: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setOrderIdentifier : orderIdentifier)
    }
    unsafe fn webServiceURL(&self) -> NSURL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, webServiceURL)
    }
    unsafe fn setWebServiceURL_(&self, webServiceURL: NSURL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setWebServiceURL : webServiceURL)
    }
    unsafe fn authenticationToken(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, authenticationToken)
    }
    unsafe fn setAuthenticationToken_(&self, authenticationToken: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAuthenticationToken : authenticationToken)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"PKPaymentOrderDetails").unwrap(), new)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct PKPaymentToken(pub id);
impl std::ops::Deref for PKPaymentToken {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for PKPaymentToken {}
impl PKPaymentToken {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"PKPaymentToken").unwrap(), alloc) })
    }
}
impl INSObject for PKPaymentToken {}
impl PNSObject for PKPaymentToken {}
impl std::convert::TryFrom<NSObject> for PKPaymentToken {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<PKPaymentToken, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"PKPaymentToken").unwrap()) };
        if is_kind_of {
            Ok(PKPaymentToken(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to PKPaymentToken")
        }
    }
}
impl IPKPaymentToken for PKPaymentToken {}
pub trait IPKPaymentToken: Sized + std::ops::Deref {
    unsafe fn paymentMethod(&self) -> PKPaymentMethod
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, paymentMethod)
    }
    unsafe fn paymentInstrumentName(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, paymentInstrumentName)
    }
    unsafe fn paymentNetwork(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, paymentNetwork)
    }
    unsafe fn transactionIdentifier(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, transactionIdentifier)
    }
    unsafe fn paymentData(&self) -> NSData
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, paymentData)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct PKPayment(pub id);
impl std::ops::Deref for PKPayment {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for PKPayment {}
impl PKPayment {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"PKPayment").unwrap(), alloc) })
    }
}
impl INSObject for PKPayment {}
impl PNSObject for PKPayment {}
impl std::convert::TryFrom<NSObject> for PKPayment {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<PKPayment, Self::Error> {
        let is_kind_of: bool = unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"PKPayment").unwrap()) };
        if is_kind_of {
            Ok(PKPayment(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to PKPayment")
        }
    }
}
impl IPKPayment for PKPayment {}
pub trait IPKPayment: Sized + std::ops::Deref {
    unsafe fn token(&self) -> PKPaymentToken
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, token)
    }
    unsafe fn billingContact(&self) -> PKContact
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, billingContact)
    }
    unsafe fn shippingContact(&self) -> PKContact
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, shippingContact)
    }
    unsafe fn shippingMethod(&self) -> PKShippingMethod
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, shippingMethod)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct PKPaymentMethod(pub id);
impl std::ops::Deref for PKPaymentMethod {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for PKPaymentMethod {}
impl PKPaymentMethod {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"PKPaymentMethod").unwrap(), alloc) })
    }
}
impl INSObject for PKPaymentMethod {}
impl PNSObject for PKPaymentMethod {}
impl std::convert::TryFrom<NSObject> for PKPaymentMethod {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<PKPaymentMethod, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"PKPaymentMethod").unwrap()) };
        if is_kind_of {
            Ok(PKPaymentMethod(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to PKPaymentMethod")
        }
    }
}
impl IPKPaymentMethod for PKPaymentMethod {}
pub trait IPKPaymentMethod: Sized + std::ops::Deref {
    unsafe fn displayName(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, displayName)
    }
    unsafe fn network(&self) -> PKPaymentNetwork
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, network)
    }
    unsafe fn type_(&self) -> PKPaymentMethodType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, type)
    }
    unsafe fn paymentPass(&self) -> PKPaymentPass
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, paymentPass)
    }
    unsafe fn secureElementPass(&self) -> PKSecureElementPass
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, secureElementPass)
    }
    unsafe fn billingAddress(&self) -> CNContact
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, billingAddress)
    }
}
pub type PKPaymentMethodType = NSUInteger;
pub trait PPKPaymentAuthorizationViewControllerDelegate: Sized + std::ops::Deref {
    unsafe fn paymentAuthorizationViewControllerDidFinish_(
        &self,
        controller: PKPaymentAuthorizationViewController,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, paymentAuthorizationViewControllerDidFinish : controller)
    }
    unsafe fn paymentAuthorizationViewController_didAuthorizePayment_handler_(
        &self,
        controller: PKPaymentAuthorizationViewController,
        payment: PKPayment,
        completion: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, paymentAuthorizationViewController : controller, didAuthorizePayment : payment, handler : completion)
    }
    unsafe fn paymentAuthorizationViewControllerWillAuthorizePayment_(
        &self,
        controller: PKPaymentAuthorizationViewController,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, paymentAuthorizationViewControllerWillAuthorizePayment : controller)
    }
    unsafe fn paymentAuthorizationViewController_didRequestMerchantSessionUpdate_(
        &self,
        controller: PKPaymentAuthorizationViewController,
        handler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, paymentAuthorizationViewController : controller, didRequestMerchantSessionUpdate : handler)
    }
    unsafe fn paymentAuthorizationViewController_didChangeCouponCode_handler_(
        &self,
        controller: PKPaymentAuthorizationViewController,
        couponCode: NSString,
        completion: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, paymentAuthorizationViewController : controller, didChangeCouponCode : couponCode, handler : completion)
    }
    unsafe fn paymentAuthorizationViewController_didSelectShippingMethod_handler_(
        &self,
        controller: PKPaymentAuthorizationViewController,
        shippingMethod: PKShippingMethod,
        completion: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, paymentAuthorizationViewController : controller, didSelectShippingMethod : shippingMethod, handler : completion)
    }
    unsafe fn paymentAuthorizationViewController_didSelectShippingContact_handler_(
        &self,
        controller: PKPaymentAuthorizationViewController,
        contact: PKContact,
        completion: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, paymentAuthorizationViewController : controller, didSelectShippingContact : contact, handler : completion)
    }
    unsafe fn paymentAuthorizationViewController_didSelectPaymentMethod_handler_(
        &self,
        controller: PKPaymentAuthorizationViewController,
        paymentMethod: PKPaymentMethod,
        completion: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, paymentAuthorizationViewController : controller, didSelectPaymentMethod : paymentMethod, handler : completion)
    }
    unsafe fn paymentAuthorizationViewController_didAuthorizePayment_completion_(
        &self,
        controller: PKPaymentAuthorizationViewController,
        payment: PKPayment,
        completion: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, paymentAuthorizationViewController : controller, didAuthorizePayment : payment, completion : completion)
    }
    unsafe fn paymentAuthorizationViewController_didSelectShippingMethod_completion_(
        &self,
        controller: PKPaymentAuthorizationViewController,
        shippingMethod: PKShippingMethod,
        completion: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, paymentAuthorizationViewController : controller, didSelectShippingMethod : shippingMethod, completion : completion)
    }
    unsafe fn paymentAuthorizationViewController_didSelectShippingContact_completion_(
        &self,
        controller: PKPaymentAuthorizationViewController,
        contact: PKContact,
        completion: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, paymentAuthorizationViewController : controller, didSelectShippingContact : contact, completion : completion)
    }
    unsafe fn paymentAuthorizationViewController_didSelectPaymentMethod_completion_(
        &self,
        controller: PKPaymentAuthorizationViewController,
        paymentMethod: PKPaymentMethod,
        completion: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, paymentAuthorizationViewController : controller, didSelectPaymentMethod : paymentMethod, completion : completion)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct PKPaymentAuthorizationViewController(pub id);
impl std::ops::Deref for PKPaymentAuthorizationViewController {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for PKPaymentAuthorizationViewController {}
impl PKPaymentAuthorizationViewController {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"PKPaymentAuthorizationViewController").unwrap(), alloc) })
    }
}
impl INSViewController for PKPaymentAuthorizationViewController {}
impl PNSEditor for PKPaymentAuthorizationViewController {}
impl PNSSeguePerforming for PKPaymentAuthorizationViewController {}
impl PNSUserInterfaceItemIdentification for PKPaymentAuthorizationViewController {}
impl std::convert::TryFrom<NSViewController> for PKPaymentAuthorizationViewController {
    type Error = &'static str;
    fn try_from(
        parent: NSViewController,
    ) -> Result<PKPaymentAuthorizationViewController, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"PKPaymentAuthorizationViewController").unwrap())
        };
        if is_kind_of {
            Ok(PKPaymentAuthorizationViewController(parent.0))
        } else {
            Err ("This NSViewController cannot be downcasted to PKPaymentAuthorizationViewController" ,)
        }
    }
}
impl INSResponder for PKPaymentAuthorizationViewController {}
impl PNSCoding for PKPaymentAuthorizationViewController {}
impl INSObject for PKPaymentAuthorizationViewController {}
impl PNSObject for PKPaymentAuthorizationViewController {}
impl IPKPaymentAuthorizationViewController for PKPaymentAuthorizationViewController {}
pub trait IPKPaymentAuthorizationViewController: Sized + std::ops::Deref {
    unsafe fn initWithPaymentRequest_(&self, request: PKPaymentRequest) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithPaymentRequest : request)
    }
    unsafe fn initWithDisbursementRequest_(&self, request: PKDisbursementRequest) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithDisbursementRequest : request)
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
    unsafe fn canMakePayments() -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"PKPaymentAuthorizationViewController").unwrap(), canMakePayments)
    }
    unsafe fn canMakePaymentsUsingNetworks_(supportedNetworks: NSArray) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"PKPaymentAuthorizationViewController").unwrap(), canMakePaymentsUsingNetworks : supportedNetworks)
    }
    unsafe fn canMakePaymentsUsingNetworks_capabilities_(
        supportedNetworks: NSArray,
        capabilties: PKMerchantCapability,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"PKPaymentAuthorizationViewController").unwrap(), canMakePaymentsUsingNetworks : supportedNetworks, capabilities : capabilties)
    }
    unsafe fn supportsDisbursements() -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"PKPaymentAuthorizationViewController").unwrap(), supportsDisbursements)
    }
    unsafe fn supportsDisbursementsUsingNetworks_(supportedNetworks: NSArray) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"PKPaymentAuthorizationViewController").unwrap(), supportsDisbursementsUsingNetworks : supportedNetworks)
    }
    unsafe fn supportsDisbursementsUsingNetworks_capabilities_(
        supportedNetworks: NSArray,
        capabilities: PKMerchantCapability,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"PKPaymentAuthorizationViewController").unwrap(), supportsDisbursementsUsingNetworks : supportedNetworks, capabilities : capabilities)
    }
}
pub trait PPKPaymentAuthorizationControllerDelegate: Sized + std::ops::Deref {
    unsafe fn paymentAuthorizationControllerDidFinish_(
        &self,
        controller: PKPaymentAuthorizationController,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, paymentAuthorizationControllerDidFinish : controller)
    }
    unsafe fn paymentAuthorizationController_didAuthorizePayment_handler_(
        &self,
        controller: PKPaymentAuthorizationController,
        payment: PKPayment,
        completion: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, paymentAuthorizationController : controller, didAuthorizePayment : payment, handler : completion)
    }
    unsafe fn paymentAuthorizationController_didAuthorizePayment_completion_(
        &self,
        controller: PKPaymentAuthorizationController,
        payment: PKPayment,
        completion: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, paymentAuthorizationController : controller, didAuthorizePayment : payment, completion : completion)
    }
    unsafe fn paymentAuthorizationControllerWillAuthorizePayment_(
        &self,
        controller: PKPaymentAuthorizationController,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, paymentAuthorizationControllerWillAuthorizePayment : controller)
    }
    unsafe fn paymentAuthorizationController_didRequestMerchantSessionUpdate_(
        &self,
        controller: PKPaymentAuthorizationController,
        handler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, paymentAuthorizationController : controller, didRequestMerchantSessionUpdate : handler)
    }
    unsafe fn paymentAuthorizationController_didChangeCouponCode_handler_(
        &self,
        controller: PKPaymentAuthorizationController,
        couponCode: NSString,
        completion: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, paymentAuthorizationController : controller, didChangeCouponCode : couponCode, handler : completion)
    }
    unsafe fn paymentAuthorizationController_didSelectShippingMethod_handler_(
        &self,
        controller: PKPaymentAuthorizationController,
        shippingMethod: PKShippingMethod,
        completion: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, paymentAuthorizationController : controller, didSelectShippingMethod : shippingMethod, handler : completion)
    }
    unsafe fn paymentAuthorizationController_didSelectShippingContact_handler_(
        &self,
        controller: PKPaymentAuthorizationController,
        contact: PKContact,
        completion: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, paymentAuthorizationController : controller, didSelectShippingContact : contact, handler : completion)
    }
    unsafe fn paymentAuthorizationController_didSelectPaymentMethod_handler_(
        &self,
        controller: PKPaymentAuthorizationController,
        paymentMethod: PKPaymentMethod,
        completion: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, paymentAuthorizationController : controller, didSelectPaymentMethod : paymentMethod, handler : completion)
    }
    unsafe fn paymentAuthorizationController_didSelectShippingMethod_completion_(
        &self,
        controller: PKPaymentAuthorizationController,
        shippingMethod: PKShippingMethod,
        completion: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, paymentAuthorizationController : controller, didSelectShippingMethod : shippingMethod, completion : completion)
    }
    unsafe fn paymentAuthorizationController_didSelectShippingContact_completion_(
        &self,
        controller: PKPaymentAuthorizationController,
        contact: PKContact,
        completion: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, paymentAuthorizationController : controller, didSelectShippingContact : contact, completion : completion)
    }
    unsafe fn paymentAuthorizationController_didSelectPaymentMethod_completion_(
        &self,
        controller: PKPaymentAuthorizationController,
        paymentMethod: PKPaymentMethod,
        completion: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, paymentAuthorizationController : controller, didSelectPaymentMethod : paymentMethod, completion : completion)
    }
    unsafe fn presentationWindowForPaymentAuthorizationController_(
        &self,
        controller: PKPaymentAuthorizationController,
    ) -> NSWindow
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, presentationWindowForPaymentAuthorizationController : controller)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct PKPaymentAuthorizationController(pub id);
impl std::ops::Deref for PKPaymentAuthorizationController {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for PKPaymentAuthorizationController {}
impl PKPaymentAuthorizationController {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"PKPaymentAuthorizationController").unwrap(), alloc) })
    }
}
impl INSObject for PKPaymentAuthorizationController {}
impl PNSObject for PKPaymentAuthorizationController {}
impl std::convert::TryFrom<NSObject> for PKPaymentAuthorizationController {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<PKPaymentAuthorizationController, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"PKPaymentAuthorizationController").unwrap())
        };
        if is_kind_of {
            Ok(PKPaymentAuthorizationController(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to PKPaymentAuthorizationController")
        }
    }
}
impl IPKPaymentAuthorizationController for PKPaymentAuthorizationController {}
pub trait IPKPaymentAuthorizationController: Sized + std::ops::Deref {
    unsafe fn initWithPaymentRequest_(&self, request: PKPaymentRequest) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithPaymentRequest : request)
    }
    unsafe fn presentWithCompletion_(&self, completion: *mut ::std::os::raw::c_void)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, presentWithCompletion : completion)
    }
    unsafe fn dismissWithCompletion_(&self, completion: *mut ::std::os::raw::c_void)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, dismissWithCompletion : completion)
    }
    unsafe fn initWithDisbursementRequest_(&self, request: PKDisbursementRequest) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithDisbursementRequest : request)
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
    unsafe fn canMakePayments() -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"PKPaymentAuthorizationController").unwrap(), canMakePayments)
    }
    unsafe fn canMakePaymentsUsingNetworks_(supportedNetworks: NSArray) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"PKPaymentAuthorizationController").unwrap(), canMakePaymentsUsingNetworks : supportedNetworks)
    }
    unsafe fn canMakePaymentsUsingNetworks_capabilities_(
        supportedNetworks: NSArray,
        capabilties: PKMerchantCapability,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"PKPaymentAuthorizationController").unwrap(), canMakePaymentsUsingNetworks : supportedNetworks, capabilities : capabilties)
    }
    unsafe fn supportsDisbursements() -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"PKPaymentAuthorizationController").unwrap(), supportsDisbursements)
    }
    unsafe fn supportsDisbursementsUsingNetworks_(supportedNetworks: NSArray) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"PKPaymentAuthorizationController").unwrap(), supportsDisbursementsUsingNetworks : supportedNetworks)
    }
    unsafe fn supportsDisbursementsUsingNetworks_capabilities_(
        supportedNetworks: NSArray,
        capabilties: PKMerchantCapability,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"PKPaymentAuthorizationController").unwrap(), supportsDisbursementsUsingNetworks : supportedNetworks, capabilities : capabilties)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct PKPaymentButton(pub id);
impl std::ops::Deref for PKPaymentButton {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for PKPaymentButton {}
impl PKPaymentButton {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"PKPaymentButton").unwrap(), alloc) })
    }
}
impl INSButton for PKPaymentButton {}
impl PNSUserInterfaceValidations for PKPaymentButton {}
impl PNSAccessibilityButton for PKPaymentButton {}
impl PNSUserInterfaceCompression for PKPaymentButton {}
impl std::convert::TryFrom<NSButton> for PKPaymentButton {
    type Error = &'static str;
    fn try_from(parent: NSButton) -> Result<PKPaymentButton, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"PKPaymentButton").unwrap()) };
        if is_kind_of {
            Ok(PKPaymentButton(parent.0))
        } else {
            Err("This NSButton cannot be downcasted to PKPaymentButton")
        }
    }
}
impl INSControl for PKPaymentButton {}
impl INSView for PKPaymentButton {}
impl PNSAnimatablePropertyContainer for PKPaymentButton {}
impl PNSUserInterfaceItemIdentification for PKPaymentButton {}
impl PNSDraggingDestination for PKPaymentButton {}
impl PNSAppearanceCustomization for PKPaymentButton {}
impl PNSAccessibilityElement for PKPaymentButton {}
impl PNSAccessibility for PKPaymentButton {}
impl INSResponder for PKPaymentButton {}
impl PNSCoding for PKPaymentButton {}
impl INSObject for PKPaymentButton {}
impl PNSObject for PKPaymentButton {}
impl IPKPaymentButton for PKPaymentButton {}
pub trait IPKPaymentButton: Sized + std::ops::Deref {
    unsafe fn initWithPaymentButtonType_paymentButtonStyle_(
        &self,
        type_: PKPaymentButtonType,
        style: PKPaymentButtonStyle,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithPaymentButtonType : type_, paymentButtonStyle : style)
    }
    unsafe fn initWithPaymentButtonType_paymentButtonStyle_disableCardArt_(
        &self,
        type_: PKPaymentButtonType,
        style: PKPaymentButtonStyle,
        disableCardArt: BOOL,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithPaymentButtonType : type_, paymentButtonStyle : style, disableCardArt : disableCardArt)
    }
    unsafe fn cornerRadius(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, cornerRadius)
    }
    unsafe fn setCornerRadius_(&self, cornerRadius: CGFloat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCornerRadius : cornerRadius)
    }
    unsafe fn buttonWithType_style_(
        buttonType: PKPaymentButtonType,
        buttonStyle: PKPaymentButtonStyle,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"PKPaymentButton").unwrap(), buttonWithType : buttonType, style : buttonStyle)
    }
    unsafe fn buttonWithType_style_disableCardArt_(
        buttonType: PKPaymentButtonType,
        buttonStyle: PKPaymentButtonStyle,
        disableCardArt: BOOL,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"PKPaymentButton").unwrap(), buttonWithType : buttonType, style : buttonStyle, disableCardArt : disableCardArt)
    }
}
pub type PKAddPaymentPassStyle = NSInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct PKAddPaymentPassRequestConfiguration(pub id);
impl std::ops::Deref for PKAddPaymentPassRequestConfiguration {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for PKAddPaymentPassRequestConfiguration {}
impl PKAddPaymentPassRequestConfiguration {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"PKAddPaymentPassRequestConfiguration").unwrap(), alloc) })
    }
}
impl INSObject for PKAddPaymentPassRequestConfiguration {}
impl PNSObject for PKAddPaymentPassRequestConfiguration {}
impl std::convert::TryFrom<NSObject> for PKAddPaymentPassRequestConfiguration {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<PKAddPaymentPassRequestConfiguration, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"PKAddPaymentPassRequestConfiguration").unwrap())
        };
        if is_kind_of {
            Ok(PKAddPaymentPassRequestConfiguration(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to PKAddPaymentPassRequestConfiguration")
        }
    }
}
impl IPKAddPaymentPassRequestConfiguration for PKAddPaymentPassRequestConfiguration {}
pub trait IPKAddPaymentPassRequestConfiguration: Sized + std::ops::Deref {
    unsafe fn initWithEncryptionScheme_(&self, encryptionScheme: NSString) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithEncryptionScheme : encryptionScheme)
    }
    unsafe fn encryptionScheme(&self) -> PKEncryptionScheme
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, encryptionScheme)
    }
    unsafe fn style(&self) -> PKAddPaymentPassStyle
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, style)
    }
    unsafe fn setStyle_(&self, style: PKAddPaymentPassStyle)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setStyle : style)
    }
    unsafe fn cardholderName(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, cardholderName)
    }
    unsafe fn setCardholderName_(&self, cardholderName: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCardholderName : cardholderName)
    }
    unsafe fn primaryAccountSuffix(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, primaryAccountSuffix)
    }
    unsafe fn setPrimaryAccountSuffix_(&self, primaryAccountSuffix: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPrimaryAccountSuffix : primaryAccountSuffix)
    }
    unsafe fn cardDetails(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, cardDetails)
    }
    unsafe fn setCardDetails_(&self, cardDetails: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCardDetails : cardDetails)
    }
    unsafe fn localizedDescription(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, localizedDescription)
    }
    unsafe fn setLocalizedDescription_(&self, localizedDescription: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setLocalizedDescription : localizedDescription)
    }
    unsafe fn primaryAccountIdentifier(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, primaryAccountIdentifier)
    }
    unsafe fn setPrimaryAccountIdentifier_(&self, primaryAccountIdentifier: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPrimaryAccountIdentifier : primaryAccountIdentifier)
    }
    unsafe fn paymentNetwork(&self) -> PKPaymentNetwork
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, paymentNetwork)
    }
    unsafe fn setPaymentNetwork_(&self, paymentNetwork: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPaymentNetwork : paymentNetwork)
    }
    unsafe fn productIdentifiers(&self) -> NSSet
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, productIdentifiers)
    }
    unsafe fn setProductIdentifiers_(&self, productIdentifiers: NSSet)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setProductIdentifiers : productIdentifiers)
    }
    unsafe fn requiresFelicaSecureElement(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, requiresFelicaSecureElement)
    }
    unsafe fn setRequiresFelicaSecureElement_(&self, requiresFelicaSecureElement: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRequiresFelicaSecureElement : requiresFelicaSecureElement)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct PKAddPaymentPassRequest(pub id);
impl std::ops::Deref for PKAddPaymentPassRequest {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for PKAddPaymentPassRequest {}
impl PKAddPaymentPassRequest {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"PKAddPaymentPassRequest").unwrap(), alloc) })
    }
}
impl INSObject for PKAddPaymentPassRequest {}
impl PNSObject for PKAddPaymentPassRequest {}
impl std::convert::TryFrom<NSObject> for PKAddPaymentPassRequest {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<PKAddPaymentPassRequest, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"PKAddPaymentPassRequest").unwrap()) };
        if is_kind_of {
            Ok(PKAddPaymentPassRequest(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to PKAddPaymentPassRequest")
        }
    }
}
impl IPKAddPaymentPassRequest for PKAddPaymentPassRequest {}
pub trait IPKAddPaymentPassRequest: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn encryptedPassData(&self) -> NSData
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, encryptedPassData)
    }
    unsafe fn setEncryptedPassData_(&self, encryptedPassData: NSData)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setEncryptedPassData : encryptedPassData)
    }
    unsafe fn activationData(&self) -> NSData
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, activationData)
    }
    unsafe fn setActivationData_(&self, activationData: NSData)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setActivationData : activationData)
    }
    unsafe fn ephemeralPublicKey(&self) -> NSData
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, ephemeralPublicKey)
    }
    unsafe fn setEphemeralPublicKey_(&self, ephemeralPublicKey: NSData)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setEphemeralPublicKey : ephemeralPublicKey)
    }
    unsafe fn wrappedKey(&self) -> NSData
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, wrappedKey)
    }
    unsafe fn setWrappedKey_(&self, wrappedKey: NSData)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setWrappedKey : wrappedKey)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct PKDisbursementRequest(pub id);
impl std::ops::Deref for PKDisbursementRequest {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for PKDisbursementRequest {}
impl PKDisbursementRequest {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"PKDisbursementRequest").unwrap(), alloc) })
    }
}
impl INSObject for PKDisbursementRequest {}
impl PNSObject for PKDisbursementRequest {}
impl std::convert::TryFrom<NSObject> for PKDisbursementRequest {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<PKDisbursementRequest, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"PKDisbursementRequest").unwrap()) };
        if is_kind_of {
            Ok(PKDisbursementRequest(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to PKDisbursementRequest")
        }
    }
}
impl IPKDisbursementRequest for PKDisbursementRequest {}
pub trait IPKDisbursementRequest: Sized + std::ops::Deref {
    unsafe fn initWithMerchantIdentifier_currencyCode_regionCode_supportedNetworks_merchantCapabilities_summaryItems_(
        &self,
        merchantIdentifier: NSString,
        currencyCode: NSString,
        regionCode: NSString,
        supportedNetworks: NSArray,
        merchantCapabilities: PKMerchantCapability,
        summaryItems: NSArray,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithMerchantIdentifier : merchantIdentifier, currencyCode : currencyCode, regionCode : regionCode, supportedNetworks : supportedNetworks, merchantCapabilities : merchantCapabilities, summaryItems : summaryItems)
    }
    unsafe fn merchantIdentifier(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, merchantIdentifier)
    }
    unsafe fn setMerchantIdentifier_(&self, merchantIdentifier: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMerchantIdentifier : merchantIdentifier)
    }
    unsafe fn regionCode(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, regionCode)
    }
    unsafe fn setRegionCode_(&self, regionCode: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRegionCode : regionCode)
    }
    unsafe fn supportedNetworks(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, supportedNetworks)
    }
    unsafe fn setSupportedNetworks_(&self, supportedNetworks: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSupportedNetworks : supportedNetworks)
    }
    unsafe fn merchantCapabilities(&self) -> PKMerchantCapability
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, merchantCapabilities)
    }
    unsafe fn setMerchantCapabilities_(&self, merchantCapabilities: PKMerchantCapability)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMerchantCapabilities : merchantCapabilities)
    }
    unsafe fn summaryItems(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, summaryItems)
    }
    unsafe fn setSummaryItems_(&self, summaryItems: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSummaryItems : summaryItems)
    }
    unsafe fn currencyCode(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, currencyCode)
    }
    unsafe fn setCurrencyCode_(&self, currencyCode: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCurrencyCode : currencyCode)
    }
    unsafe fn requiredRecipientContactFields(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, requiredRecipientContactFields)
    }
    unsafe fn setRequiredRecipientContactFields_(&self, requiredRecipientContactFields: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRequiredRecipientContactFields : requiredRecipientContactFields)
    }
    unsafe fn recipientContact(&self) -> PKContact
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, recipientContact)
    }
    unsafe fn setRecipientContact_(&self, recipientContact: PKContact)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRecipientContact : recipientContact)
    }
    unsafe fn supportedRegions(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, supportedRegions)
    }
    unsafe fn setSupportedRegions_(&self, supportedRegions: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSupportedRegions : supportedRegions)
    }
    unsafe fn applicationData(&self) -> NSData
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, applicationData)
    }
    unsafe fn setApplicationData_(&self, applicationData: NSData)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setApplicationData : applicationData)
    }
    unsafe fn disbursementContactInvalidErrorWithContactField_localizedDescription_(
        field: NSString,
        localizedDescription: NSString,
    ) -> NSError
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"PKDisbursementRequest").unwrap(), disbursementContactInvalidErrorWithContactField : field, localizedDescription : localizedDescription)
    }
    unsafe fn disbursementCardUnsupportedError() -> NSError
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"PKDisbursementRequest").unwrap(), disbursementCardUnsupportedError)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct PKInstantFundsOutFeeSummaryItem(pub id);
impl std::ops::Deref for PKInstantFundsOutFeeSummaryItem {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for PKInstantFundsOutFeeSummaryItem {}
impl PKInstantFundsOutFeeSummaryItem {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"PKInstantFundsOutFeeSummaryItem").unwrap(), alloc) })
    }
}
impl IPKPaymentSummaryItem for PKInstantFundsOutFeeSummaryItem {}
impl std::convert::TryFrom<PKPaymentSummaryItem> for PKInstantFundsOutFeeSummaryItem {
    type Error = &'static str;
    fn try_from(
        parent: PKPaymentSummaryItem,
    ) -> Result<PKInstantFundsOutFeeSummaryItem, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"PKInstantFundsOutFeeSummaryItem").unwrap())
        };
        if is_kind_of {
            Ok(PKInstantFundsOutFeeSummaryItem(parent.0))
        } else {
            Err("This PKPaymentSummaryItem cannot be downcasted to PKInstantFundsOutFeeSummaryItem")
        }
    }
}
impl INSObject for PKInstantFundsOutFeeSummaryItem {}
impl PNSObject for PKInstantFundsOutFeeSummaryItem {}
impl IPKInstantFundsOutFeeSummaryItem for PKInstantFundsOutFeeSummaryItem {}
pub trait IPKInstantFundsOutFeeSummaryItem: Sized + std::ops::Deref {}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct PKDisbursementSummaryItem(pub id);
impl std::ops::Deref for PKDisbursementSummaryItem {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for PKDisbursementSummaryItem {}
impl PKDisbursementSummaryItem {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"PKDisbursementSummaryItem").unwrap(), alloc) })
    }
}
impl IPKPaymentSummaryItem for PKDisbursementSummaryItem {}
impl std::convert::TryFrom<PKPaymentSummaryItem> for PKDisbursementSummaryItem {
    type Error = &'static str;
    fn try_from(parent: PKPaymentSummaryItem) -> Result<PKDisbursementSummaryItem, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"PKDisbursementSummaryItem").unwrap()) };
        if is_kind_of {
            Ok(PKDisbursementSummaryItem(parent.0))
        } else {
            Err("This PKPaymentSummaryItem cannot be downcasted to PKDisbursementSummaryItem")
        }
    }
}
impl INSObject for PKDisbursementSummaryItem {}
impl PNSObject for PKDisbursementSummaryItem {}
impl IPKDisbursementSummaryItem for PKDisbursementSummaryItem {}
pub trait IPKDisbursementSummaryItem: Sized + std::ops::Deref {}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct PKBarcodeEventMetadataRequest(pub id);
impl std::ops::Deref for PKBarcodeEventMetadataRequest {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for PKBarcodeEventMetadataRequest {}
impl PKBarcodeEventMetadataRequest {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"PKBarcodeEventMetadataRequest").unwrap(), alloc) })
    }
}
impl INSObject for PKBarcodeEventMetadataRequest {}
impl PNSObject for PKBarcodeEventMetadataRequest {}
impl std::convert::TryFrom<NSObject> for PKBarcodeEventMetadataRequest {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<PKBarcodeEventMetadataRequest, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"PKBarcodeEventMetadataRequest").unwrap())
        };
        if is_kind_of {
            Ok(PKBarcodeEventMetadataRequest(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to PKBarcodeEventMetadataRequest")
        }
    }
}
impl IPKBarcodeEventMetadataRequest for PKBarcodeEventMetadataRequest {}
pub trait IPKBarcodeEventMetadataRequest: Sized + std::ops::Deref {
    unsafe fn deviceAccountIdentifier(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, deviceAccountIdentifier)
    }
    unsafe fn lastUsedBarcodeIdentifier(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, lastUsedBarcodeIdentifier)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct PKBarcodeEventMetadataResponse(pub id);
impl std::ops::Deref for PKBarcodeEventMetadataResponse {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for PKBarcodeEventMetadataResponse {}
impl PKBarcodeEventMetadataResponse {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"PKBarcodeEventMetadataResponse").unwrap(), alloc) })
    }
}
impl INSObject for PKBarcodeEventMetadataResponse {}
impl PNSObject for PKBarcodeEventMetadataResponse {}
impl std::convert::TryFrom<NSObject> for PKBarcodeEventMetadataResponse {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<PKBarcodeEventMetadataResponse, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"PKBarcodeEventMetadataResponse").unwrap())
        };
        if is_kind_of {
            Ok(PKBarcodeEventMetadataResponse(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to PKBarcodeEventMetadataResponse")
        }
    }
}
impl IPKBarcodeEventMetadataResponse for PKBarcodeEventMetadataResponse {}
pub trait IPKBarcodeEventMetadataResponse: Sized + std::ops::Deref {
    unsafe fn initWithPaymentInformation_(&self, paymentInformation: NSData) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithPaymentInformation : paymentInformation)
    }
    unsafe fn paymentInformation(&self) -> NSData
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, paymentInformation)
    }
    unsafe fn setPaymentInformation_(&self, paymentInformation: NSData)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPaymentInformation : paymentInformation)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct PKBarcodeEventSignatureRequest(pub id);
impl std::ops::Deref for PKBarcodeEventSignatureRequest {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for PKBarcodeEventSignatureRequest {}
impl PKBarcodeEventSignatureRequest {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"PKBarcodeEventSignatureRequest").unwrap(), alloc) })
    }
}
impl INSObject for PKBarcodeEventSignatureRequest {}
impl PNSObject for PKBarcodeEventSignatureRequest {}
impl std::convert::TryFrom<NSObject> for PKBarcodeEventSignatureRequest {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<PKBarcodeEventSignatureRequest, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"PKBarcodeEventSignatureRequest").unwrap())
        };
        if is_kind_of {
            Ok(PKBarcodeEventSignatureRequest(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to PKBarcodeEventSignatureRequest")
        }
    }
}
impl IPKBarcodeEventSignatureRequest for PKBarcodeEventSignatureRequest {}
pub trait IPKBarcodeEventSignatureRequest: Sized + std::ops::Deref {
    unsafe fn deviceAccountIdentifier(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, deviceAccountIdentifier)
    }
    unsafe fn transactionIdentifier(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, transactionIdentifier)
    }
    unsafe fn barcodeIdentifier(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, barcodeIdentifier)
    }
    unsafe fn rawMerchantName(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, rawMerchantName)
    }
    unsafe fn merchantName(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, merchantName)
    }
    unsafe fn transactionDate(&self) -> NSDate
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, transactionDate)
    }
    unsafe fn currencyCode(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, currencyCode)
    }
    unsafe fn amount(&self) -> NSNumber
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, amount)
    }
    unsafe fn transactionStatus(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, transactionStatus)
    }
    unsafe fn partialSignature(&self) -> NSData
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, partialSignature)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct PKBarcodeEventSignatureResponse(pub id);
impl std::ops::Deref for PKBarcodeEventSignatureResponse {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for PKBarcodeEventSignatureResponse {}
impl PKBarcodeEventSignatureResponse {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"PKBarcodeEventSignatureResponse").unwrap(), alloc) })
    }
}
impl INSObject for PKBarcodeEventSignatureResponse {}
impl PNSObject for PKBarcodeEventSignatureResponse {}
impl std::convert::TryFrom<NSObject> for PKBarcodeEventSignatureResponse {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<PKBarcodeEventSignatureResponse, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"PKBarcodeEventSignatureResponse").unwrap())
        };
        if is_kind_of {
            Ok(PKBarcodeEventSignatureResponse(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to PKBarcodeEventSignatureResponse")
        }
    }
}
impl IPKBarcodeEventSignatureResponse for PKBarcodeEventSignatureResponse {}
pub trait IPKBarcodeEventSignatureResponse: Sized + std::ops::Deref {
    unsafe fn initWithSignedData_(&self, signedData: NSData) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithSignedData : signedData)
    }
    unsafe fn signedData(&self) -> NSData
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, signedData)
    }
    unsafe fn setSignedData_(&self, signedData: NSData)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSignedData : signedData)
    }
}
pub type PKBarcodeEventConfigurationDataType = NSInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct PKBarcodeEventConfigurationRequest(pub id);
impl std::ops::Deref for PKBarcodeEventConfigurationRequest {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for PKBarcodeEventConfigurationRequest {}
impl PKBarcodeEventConfigurationRequest {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"PKBarcodeEventConfigurationRequest").unwrap(), alloc) })
    }
}
impl INSObject for PKBarcodeEventConfigurationRequest {}
impl PNSObject for PKBarcodeEventConfigurationRequest {}
impl std::convert::TryFrom<NSObject> for PKBarcodeEventConfigurationRequest {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<PKBarcodeEventConfigurationRequest, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"PKBarcodeEventConfigurationRequest").unwrap())
        };
        if is_kind_of {
            Ok(PKBarcodeEventConfigurationRequest(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to PKBarcodeEventConfigurationRequest")
        }
    }
}
impl IPKBarcodeEventConfigurationRequest for PKBarcodeEventConfigurationRequest {}
pub trait IPKBarcodeEventConfigurationRequest: Sized + std::ops::Deref {
    unsafe fn deviceAccountIdentifier(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, deviceAccountIdentifier)
    }
    unsafe fn configurationData(&self) -> NSData
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, configurationData)
    }
    unsafe fn configurationDataType(&self) -> PKBarcodeEventConfigurationDataType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, configurationDataType)
    }
}
pub type PKInformationRequestCompletionBlock = *mut ::std::os::raw::c_void;
pub type PKSignatureRequestCompletionBlock = *mut ::std::os::raw::c_void;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct PKPaymentInformationEventExtension(pub id);
impl std::ops::Deref for PKPaymentInformationEventExtension {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for PKPaymentInformationEventExtension {}
impl PKPaymentInformationEventExtension {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"PKPaymentInformationEventExtension").unwrap(), alloc) })
    }
}
impl INSObject for PKPaymentInformationEventExtension {}
impl PNSObject for PKPaymentInformationEventExtension {}
impl std::convert::TryFrom<NSObject> for PKPaymentInformationEventExtension {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<PKPaymentInformationEventExtension, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"PKPaymentInformationEventExtension").unwrap())
        };
        if is_kind_of {
            Ok(PKPaymentInformationEventExtension(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to PKPaymentInformationEventExtension")
        }
    }
}
impl IPKPaymentInformationEventExtension for PKPaymentInformationEventExtension {}
pub trait IPKPaymentInformationEventExtension: Sized + std::ops::Deref {}
pub trait PPKPaymentInformationRequestHandling: Sized + std::ops::Deref {
    unsafe fn handleInformationRequest_completion_(
        &self,
        infoRequest: PKBarcodeEventMetadataRequest,
        completion: PKInformationRequestCompletionBlock,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, handleInformationRequest : infoRequest, completion : completion)
    }
    unsafe fn handleSignatureRequest_completion_(
        &self,
        signatureRequest: PKBarcodeEventSignatureRequest,
        completion: PKSignatureRequestCompletionBlock,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, handleSignatureRequest : signatureRequest, completion : completion)
    }
    unsafe fn handleConfigurationRequest_completion_(
        &self,
        configurationRequest: PKBarcodeEventConfigurationRequest,
        completion: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, handleConfigurationRequest : configurationRequest, completion : completion)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct PKAddPassMetadataPreview(pub id);
impl std::ops::Deref for PKAddPassMetadataPreview {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for PKAddPassMetadataPreview {}
impl PKAddPassMetadataPreview {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"PKAddPassMetadataPreview").unwrap(), alloc) })
    }
}
impl INSObject for PKAddPassMetadataPreview {}
impl PNSObject for PKAddPassMetadataPreview {}
impl std::convert::TryFrom<NSObject> for PKAddPassMetadataPreview {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<PKAddPassMetadataPreview, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"PKAddPassMetadataPreview").unwrap()) };
        if is_kind_of {
            Ok(PKAddPassMetadataPreview(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to PKAddPassMetadataPreview")
        }
    }
}
impl IPKAddPassMetadataPreview for PKAddPassMetadataPreview {}
pub trait IPKAddPassMetadataPreview: Sized + std::ops::Deref {
    unsafe fn initWithPassThumbnail_localizedDescription_(
        &self,
        passThumbnail: CGImageRef,
        description: NSString,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithPassThumbnail : passThumbnail, localizedDescription : description)
    }
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn passThumbnailImage(&self) -> CGImageRef
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, passThumbnailImage)
    }
    unsafe fn localizedDescription(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, localizedDescription)
    }
    unsafe fn previewWithPassThumbnail_localizedDescription_(
        passThumbnail: CGImageRef,
        description: NSString,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"PKAddPassMetadataPreview").unwrap(), previewWithPassThumbnail : passThumbnail, localizedDescription : description)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"PKAddPassMetadataPreview").unwrap(), new)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct PKAddSecureElementPassConfiguration(pub id);
impl std::ops::Deref for PKAddSecureElementPassConfiguration {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for PKAddSecureElementPassConfiguration {}
impl PKAddSecureElementPassConfiguration {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"PKAddSecureElementPassConfiguration").unwrap(), alloc) })
    }
}
impl INSObject for PKAddSecureElementPassConfiguration {}
impl PNSObject for PKAddSecureElementPassConfiguration {}
impl std::convert::TryFrom<NSObject> for PKAddSecureElementPassConfiguration {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<PKAddSecureElementPassConfiguration, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"PKAddSecureElementPassConfiguration").unwrap())
        };
        if is_kind_of {
            Ok(PKAddSecureElementPassConfiguration(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to PKAddSecureElementPassConfiguration")
        }
    }
}
impl IPKAddSecureElementPassConfiguration for PKAddSecureElementPassConfiguration {}
pub trait IPKAddSecureElementPassConfiguration: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn issuerIdentifier(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, issuerIdentifier)
    }
    unsafe fn setIssuerIdentifier_(&self, issuerIdentifier: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setIssuerIdentifier : issuerIdentifier)
    }
    unsafe fn localizedDescription(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, localizedDescription)
    }
    unsafe fn setLocalizedDescription_(&self, localizedDescription: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setLocalizedDescription : localizedDescription)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"PKAddSecureElementPassConfiguration").unwrap(), new)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct PKAddCarKeyPassConfiguration(pub id);
impl std::ops::Deref for PKAddCarKeyPassConfiguration {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for PKAddCarKeyPassConfiguration {}
impl PKAddCarKeyPassConfiguration {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"PKAddCarKeyPassConfiguration").unwrap(), alloc) })
    }
}
impl IPKAddSecureElementPassConfiguration for PKAddCarKeyPassConfiguration {}
impl From<PKAddCarKeyPassConfiguration> for PKAddSecureElementPassConfiguration {
    fn from(child: PKAddCarKeyPassConfiguration) -> PKAddSecureElementPassConfiguration {
        PKAddSecureElementPassConfiguration(child.0)
    }
}
impl std::convert::TryFrom<PKAddSecureElementPassConfiguration> for PKAddCarKeyPassConfiguration {
    type Error = &'static str;
    fn try_from(
        parent: PKAddSecureElementPassConfiguration,
    ) -> Result<PKAddCarKeyPassConfiguration, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"PKAddCarKeyPassConfiguration").unwrap()) };
        if is_kind_of {
            Ok(PKAddCarKeyPassConfiguration(parent.0))
        } else {
            Err ("This PKAddSecureElementPassConfiguration cannot be downcasted to PKAddCarKeyPassConfiguration" ,)
        }
    }
}
impl INSObject for PKAddCarKeyPassConfiguration {}
impl PNSObject for PKAddCarKeyPassConfiguration {}
impl IPKAddCarKeyPassConfiguration for PKAddCarKeyPassConfiguration {}
pub trait IPKAddCarKeyPassConfiguration: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn password(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, password)
    }
    unsafe fn setPassword_(&self, password: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPassword : password)
    }
    unsafe fn supportedRadioTechnologies(&self) -> PKRadioTechnology
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, supportedRadioTechnologies)
    }
    unsafe fn setSupportedRadioTechnologies_(&self, supportedRadioTechnologies: PKRadioTechnology)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSupportedRadioTechnologies : supportedRadioTechnologies)
    }
    unsafe fn manufacturerIdentifier(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, manufacturerIdentifier)
    }
    unsafe fn setManufacturerIdentifier_(&self, manufacturerIdentifier: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setManufacturerIdentifier : manufacturerIdentifier)
    }
    unsafe fn provisioningTemplateIdentifier(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, provisioningTemplateIdentifier)
    }
    unsafe fn setProvisioningTemplateIdentifier_(&self, provisioningTemplateIdentifier: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setProvisioningTemplateIdentifier : provisioningTemplateIdentifier)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct PKPaymentMerchantSession(pub id);
impl std::ops::Deref for PKPaymentMerchantSession {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for PKPaymentMerchantSession {}
impl PKPaymentMerchantSession {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"PKPaymentMerchantSession").unwrap(), alloc) })
    }
}
impl INSObject for PKPaymentMerchantSession {}
impl PNSObject for PKPaymentMerchantSession {}
impl std::convert::TryFrom<NSObject> for PKPaymentMerchantSession {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<PKPaymentMerchantSession, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"PKPaymentMerchantSession").unwrap()) };
        if is_kind_of {
            Ok(PKPaymentMerchantSession(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to PKPaymentMerchantSession")
        }
    }
}
impl IPKPaymentMerchantSession for PKPaymentMerchantSession {}
pub trait IPKPaymentMerchantSession: Sized + std::ops::Deref {
    unsafe fn initWithDictionary_(&self, dictionary: NSDictionary) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithDictionary : dictionary)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct PKShareablePassMetadataPreview(pub id);
impl std::ops::Deref for PKShareablePassMetadataPreview {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for PKShareablePassMetadataPreview {}
impl PKShareablePassMetadataPreview {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"PKShareablePassMetadataPreview").unwrap(), alloc) })
    }
}
impl IPKAddPassMetadataPreview for PKShareablePassMetadataPreview {}
impl From<PKShareablePassMetadataPreview> for PKAddPassMetadataPreview {
    fn from(child: PKShareablePassMetadataPreview) -> PKAddPassMetadataPreview {
        PKAddPassMetadataPreview(child.0)
    }
}
impl std::convert::TryFrom<PKAddPassMetadataPreview> for PKShareablePassMetadataPreview {
    type Error = &'static str;
    fn try_from(
        parent: PKAddPassMetadataPreview,
    ) -> Result<PKShareablePassMetadataPreview, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"PKShareablePassMetadataPreview").unwrap())
        };
        if is_kind_of {
            Ok(PKShareablePassMetadataPreview(parent.0))
        } else {
            Err ("This PKAddPassMetadataPreview cannot be downcasted to PKShareablePassMetadataPreview" ,)
        }
    }
}
impl INSObject for PKShareablePassMetadataPreview {}
impl PNSObject for PKShareablePassMetadataPreview {}
impl IPKShareablePassMetadataPreview for PKShareablePassMetadataPreview {}
pub trait IPKShareablePassMetadataPreview: Sized + std::ops::Deref {
    unsafe fn initWithTemplateIdentifier_(&self, templateIdentifier: NSString) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithTemplateIdentifier : templateIdentifier)
    }
    unsafe fn ownerDisplayName(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, ownerDisplayName)
    }
    unsafe fn setOwnerDisplayName_(&self, ownerDisplayName: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setOwnerDisplayName : ownerDisplayName)
    }
    unsafe fn provisioningTemplateIdentifier(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, provisioningTemplateIdentifier)
    }
    unsafe fn previewWithTemplateIdentifier_(templateIdentifier: NSString) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"PKShareablePassMetadataPreview").unwrap(), previewWithTemplateIdentifier : templateIdentifier)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct PKShareablePassMetadata(pub id);
impl std::ops::Deref for PKShareablePassMetadata {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for PKShareablePassMetadata {}
impl PKShareablePassMetadata {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"PKShareablePassMetadata").unwrap(), alloc) })
    }
}
impl INSObject for PKShareablePassMetadata {}
impl PNSObject for PKShareablePassMetadata {}
impl std::convert::TryFrom<NSObject> for PKShareablePassMetadata {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<PKShareablePassMetadata, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"PKShareablePassMetadata").unwrap()) };
        if is_kind_of {
            Ok(PKShareablePassMetadata(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to PKShareablePassMetadata")
        }
    }
}
impl IPKShareablePassMetadata for PKShareablePassMetadata {}
pub trait IPKShareablePassMetadata: Sized + std::ops::Deref {
    unsafe fn initWithProvisioningCredentialIdentifier_cardConfigurationIdentifier_sharingInstanceIdentifier_passThumbnailImage_ownerDisplayName_localizedDescription_(
        &self,
        credentialIdentifier: NSString,
        cardConfigurationIdentifier: NSString,
        sharingInstanceIdentifier: NSString,
        passThumbnailImage: CGImageRef,
        ownerDisplayName: NSString,
        localizedDescription: NSString,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithProvisioningCredentialIdentifier : credentialIdentifier, cardConfigurationIdentifier : cardConfigurationIdentifier, sharingInstanceIdentifier : sharingInstanceIdentifier, passThumbnailImage : passThumbnailImage, ownerDisplayName : ownerDisplayName, localizedDescription : localizedDescription)
    }
    unsafe fn initWithProvisioningCredentialIdentifier_sharingInstanceIdentifier_passThumbnailImage_ownerDisplayName_localizedDescription_accountHash_templateIdentifier_relyingPartyIdentifier_requiresUnifiedAccessCapableDevice_(
        &self,
        credentialIdentifier: NSString,
        sharingInstanceIdentifier: NSString,
        passThumbnailImage: CGImageRef,
        ownerDisplayName: NSString,
        localizedDescription: NSString,
        accountHash: NSString,
        templateIdentifier: NSString,
        relyingPartyIdentifier: NSString,
        requiresUnifiedAccessCapableDevice: BOOL,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithProvisioningCredentialIdentifier : credentialIdentifier, sharingInstanceIdentifier : sharingInstanceIdentifier, passThumbnailImage : passThumbnailImage, ownerDisplayName : ownerDisplayName, localizedDescription : localizedDescription, accountHash : accountHash, templateIdentifier : templateIdentifier, relyingPartyIdentifier : relyingPartyIdentifier, requiresUnifiedAccessCapableDevice : requiresUnifiedAccessCapableDevice)
    }
    unsafe fn initWithProvisioningCredentialIdentifier_sharingInstanceIdentifier_cardTemplateIdentifier_preview_(
        &self,
        credentialIdentifier: NSString,
        sharingInstanceIdentifier: NSString,
        templateIdentifier: NSString,
        preview: PKShareablePassMetadataPreview,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithProvisioningCredentialIdentifier : credentialIdentifier, sharingInstanceIdentifier : sharingInstanceIdentifier, cardTemplateIdentifier : templateIdentifier, preview : preview)
    }
    unsafe fn initWithProvisioningCredentialIdentifier_sharingInstanceIdentifier_cardConfigurationIdentifier_preview_(
        &self,
        credentialIdentifier: NSString,
        sharingInstanceIdentifier: NSString,
        templateIdentifier: NSString,
        preview: PKShareablePassMetadataPreview,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithProvisioningCredentialIdentifier : credentialIdentifier, sharingInstanceIdentifier : sharingInstanceIdentifier, cardConfigurationIdentifier : templateIdentifier, preview : preview)
    }
    unsafe fn credentialIdentifier(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, credentialIdentifier)
    }
    unsafe fn sharingInstanceIdentifier(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, sharingInstanceIdentifier)
    }
    unsafe fn templateIdentifier(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, templateIdentifier)
    }
    unsafe fn cardTemplateIdentifier(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, cardTemplateIdentifier)
    }
    unsafe fn cardConfigurationIdentifier(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, cardConfigurationIdentifier)
    }
    unsafe fn requiresUnifiedAccessCapableDevice(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, requiresUnifiedAccessCapableDevice)
    }
    unsafe fn setRequiresUnifiedAccessCapableDevice_(
        &self,
        requiresUnifiedAccessCapableDevice: BOOL,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRequiresUnifiedAccessCapableDevice : requiresUnifiedAccessCapableDevice)
    }
    unsafe fn serverEnvironmentIdentifier(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, serverEnvironmentIdentifier)
    }
    unsafe fn setServerEnvironmentIdentifier_(&self, serverEnvironmentIdentifier: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setServerEnvironmentIdentifier : serverEnvironmentIdentifier)
    }
    unsafe fn preview(&self) -> PKShareablePassMetadataPreview
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, preview)
    }
    unsafe fn passThumbnailImage(&self) -> CGImageRef
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, passThumbnailImage)
    }
    unsafe fn localizedDescription(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, localizedDescription)
    }
    unsafe fn ownerDisplayName(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, ownerDisplayName)
    }
    unsafe fn accountHash(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, accountHash)
    }
    unsafe fn setAccountHash_(&self, accountHash: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAccountHash : accountHash)
    }
    unsafe fn relyingPartyIdentifier(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, relyingPartyIdentifier)
    }
    unsafe fn setRelyingPartyIdentifier_(&self, relyingPartyIdentifier: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRelyingPartyIdentifier : relyingPartyIdentifier)
    }
}
pub type PKAddShareablePassConfigurationPrimaryAction = NSUInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct PKAddShareablePassConfiguration(pub id);
impl std::ops::Deref for PKAddShareablePassConfiguration {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for PKAddShareablePassConfiguration {}
impl PKAddShareablePassConfiguration {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"PKAddShareablePassConfiguration").unwrap(), alloc) })
    }
}
impl IPKAddSecureElementPassConfiguration for PKAddShareablePassConfiguration {}
impl std::convert::TryFrom<PKAddSecureElementPassConfiguration>
    for PKAddShareablePassConfiguration
{
    type Error = &'static str;
    fn try_from(
        parent: PKAddSecureElementPassConfiguration,
    ) -> Result<PKAddShareablePassConfiguration, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"PKAddShareablePassConfiguration").unwrap())
        };
        if is_kind_of {
            Ok(PKAddShareablePassConfiguration(parent.0))
        } else {
            Err ("This PKAddSecureElementPassConfiguration cannot be downcasted to PKAddShareablePassConfiguration" ,)
        }
    }
}
impl INSObject for PKAddShareablePassConfiguration {}
impl PNSObject for PKAddShareablePassConfiguration {}
impl IPKAddShareablePassConfiguration for PKAddShareablePassConfiguration {}
pub trait IPKAddShareablePassConfiguration: Sized + std::ops::Deref {
    unsafe fn primaryAction(&self) -> PKAddShareablePassConfigurationPrimaryAction
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, primaryAction)
    }
    unsafe fn credentialsMetadata(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, credentialsMetadata)
    }
    unsafe fn provisioningPolicyIdentifier(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, provisioningPolicyIdentifier)
    }
    unsafe fn configurationForPassMetadata_provisioningPolicyIdentifier_primaryAction_completion_(
        passMetadata: NSArray,
        provisioningPolicyIdentifier: NSString,
        action: PKAddShareablePassConfigurationPrimaryAction,
        completion: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"PKAddShareablePassConfiguration").unwrap(), configurationForPassMetadata : passMetadata, provisioningPolicyIdentifier : provisioningPolicyIdentifier, primaryAction : action, completion : completion)
    }
    unsafe fn configurationForPassMetadata_primaryAction_completion_(
        passMetadata: NSArray,
        action: PKAddShareablePassConfigurationPrimaryAction,
        completion: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"PKAddShareablePassConfiguration").unwrap(), configurationForPassMetadata : passMetadata, primaryAction : action, completion : completion)
    }
}
pub type PKAddIdentityDocumentType = NSInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct PKIdentityDocumentMetadata(pub id);
impl std::ops::Deref for PKIdentityDocumentMetadata {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for PKIdentityDocumentMetadata {}
impl PKIdentityDocumentMetadata {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"PKIdentityDocumentMetadata").unwrap(), alloc) })
    }
}
impl INSObject for PKIdentityDocumentMetadata {}
impl PNSObject for PKIdentityDocumentMetadata {}
impl std::convert::TryFrom<NSObject> for PKIdentityDocumentMetadata {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<PKIdentityDocumentMetadata, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"PKIdentityDocumentMetadata").unwrap()) };
        if is_kind_of {
            Ok(PKIdentityDocumentMetadata(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to PKIdentityDocumentMetadata")
        }
    }
}
impl IPKIdentityDocumentMetadata for PKIdentityDocumentMetadata {}
pub trait IPKIdentityDocumentMetadata: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn credentialIdentifier(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, credentialIdentifier)
    }
    unsafe fn sharingInstanceIdentifier(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, sharingInstanceIdentifier)
    }
    unsafe fn cardTemplateIdentifier(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, cardTemplateIdentifier)
    }
    unsafe fn cardConfigurationIdentifier(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, cardConfigurationIdentifier)
    }
    unsafe fn serverEnvironmentIdentifier(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, serverEnvironmentIdentifier)
    }
    unsafe fn setServerEnvironmentIdentifier_(&self, serverEnvironmentIdentifier: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setServerEnvironmentIdentifier : serverEnvironmentIdentifier)
    }
    unsafe fn issuingCountryCode(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, issuingCountryCode)
    }
    unsafe fn documentType(&self) -> PKAddIdentityDocumentType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, documentType)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"PKIdentityDocumentMetadata").unwrap(), new)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct PKAddIdentityDocumentMetadata(pub id);
impl std::ops::Deref for PKAddIdentityDocumentMetadata {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for PKAddIdentityDocumentMetadata {}
impl PKAddIdentityDocumentMetadata {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"PKAddIdentityDocumentMetadata").unwrap(), alloc) })
    }
}
impl IPKIdentityDocumentMetadata for PKAddIdentityDocumentMetadata {}
impl From<PKAddIdentityDocumentMetadata> for PKIdentityDocumentMetadata {
    fn from(child: PKAddIdentityDocumentMetadata) -> PKIdentityDocumentMetadata {
        PKIdentityDocumentMetadata(child.0)
    }
}
impl std::convert::TryFrom<PKIdentityDocumentMetadata> for PKAddIdentityDocumentMetadata {
    type Error = &'static str;
    fn try_from(
        parent: PKIdentityDocumentMetadata,
    ) -> Result<PKAddIdentityDocumentMetadata, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"PKAddIdentityDocumentMetadata").unwrap())
        };
        if is_kind_of {
            Ok(PKAddIdentityDocumentMetadata(parent.0))
        } else {
            Err ("This PKIdentityDocumentMetadata cannot be downcasted to PKAddIdentityDocumentMetadata" ,)
        }
    }
}
impl INSObject for PKAddIdentityDocumentMetadata {}
impl PNSObject for PKAddIdentityDocumentMetadata {}
impl IPKAddIdentityDocumentMetadata for PKAddIdentityDocumentMetadata {}
pub trait IPKAddIdentityDocumentMetadata: Sized + std::ops::Deref {
    unsafe fn initWithProvisioningCredentialIdentifier_sharingInstanceIdentifier_cardTemplateIdentifier_issuingCountryCode_documentType_preview_(
        &self,
        credentialIdentifier: NSString,
        sharingInstanceIdentifier: NSString,
        templateIdentifier: NSString,
        issuingCountryCode: NSString,
        documentType: PKAddIdentityDocumentType,
        preview: PKAddPassMetadataPreview,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithProvisioningCredentialIdentifier : credentialIdentifier, sharingInstanceIdentifier : sharingInstanceIdentifier, cardTemplateIdentifier : templateIdentifier, issuingCountryCode : issuingCountryCode, documentType : documentType, preview : preview)
    }
    unsafe fn preview(&self) -> PKAddPassMetadataPreview
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, preview)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct PKJapanIndividualNumberCardMetadata(pub id);
impl std::ops::Deref for PKJapanIndividualNumberCardMetadata {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for PKJapanIndividualNumberCardMetadata {}
impl PKJapanIndividualNumberCardMetadata {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"PKJapanIndividualNumberCardMetadata").unwrap(), alloc) })
    }
}
impl IPKIdentityDocumentMetadata for PKJapanIndividualNumberCardMetadata {}
impl std::convert::TryFrom<PKIdentityDocumentMetadata> for PKJapanIndividualNumberCardMetadata {
    type Error = &'static str;
    fn try_from(
        parent: PKIdentityDocumentMetadata,
    ) -> Result<PKJapanIndividualNumberCardMetadata, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"PKJapanIndividualNumberCardMetadata").unwrap())
        };
        if is_kind_of {
            Ok(PKJapanIndividualNumberCardMetadata(parent.0))
        } else {
            Err ("This PKIdentityDocumentMetadata cannot be downcasted to PKJapanIndividualNumberCardMetadata" ,)
        }
    }
}
impl INSObject for PKJapanIndividualNumberCardMetadata {}
impl PNSObject for PKJapanIndividualNumberCardMetadata {}
impl IPKJapanIndividualNumberCardMetadata for PKJapanIndividualNumberCardMetadata {}
pub trait IPKJapanIndividualNumberCardMetadata: Sized + std::ops::Deref {
    unsafe fn initWithProvisioningCredentialIdentifier_sharingInstanceIdentifier_cardTemplateIdentifier_preview_(
        &self,
        credentialIdentifier: NSString,
        sharingInstanceIdentifier: NSString,
        templateIdentifier: NSString,
        preview: PKAddPassMetadataPreview,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithProvisioningCredentialIdentifier : credentialIdentifier, sharingInstanceIdentifier : sharingInstanceIdentifier, cardTemplateIdentifier : templateIdentifier, preview : preview)
    }
    unsafe fn initWithProvisioningCredentialIdentifier_sharingInstanceIdentifier_cardConfigurationIdentifier_preview_(
        &self,
        credentialIdentifier: NSString,
        sharingInstanceIdentifier: NSString,
        cardConfigurationIdentifier: NSString,
        preview: PKAddPassMetadataPreview,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithProvisioningCredentialIdentifier : credentialIdentifier, sharingInstanceIdentifier : sharingInstanceIdentifier, cardConfigurationIdentifier : cardConfigurationIdentifier, preview : preview)
    }
    unsafe fn authenticationPassword(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, authenticationPassword)
    }
    unsafe fn setAuthenticationPassword_(&self, authenticationPassword: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAuthenticationPassword : authenticationPassword)
    }
    unsafe fn signingPassword(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, signingPassword)
    }
    unsafe fn setSigningPassword_(&self, signingPassword: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSigningPassword : signingPassword)
    }
    unsafe fn preview(&self) -> PKAddPassMetadataPreview
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, preview)
    }
    unsafe fn setPreview_(&self, preview: PKAddPassMetadataPreview)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPreview : preview)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct PKAddIdentityDocumentConfiguration(pub id);
impl std::ops::Deref for PKAddIdentityDocumentConfiguration {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for PKAddIdentityDocumentConfiguration {}
impl PKAddIdentityDocumentConfiguration {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"PKAddIdentityDocumentConfiguration").unwrap(), alloc) })
    }
}
impl IPKAddSecureElementPassConfiguration for PKAddIdentityDocumentConfiguration {}
impl std::convert::TryFrom<PKAddSecureElementPassConfiguration>
    for PKAddIdentityDocumentConfiguration
{
    type Error = &'static str;
    fn try_from(
        parent: PKAddSecureElementPassConfiguration,
    ) -> Result<PKAddIdentityDocumentConfiguration, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"PKAddIdentityDocumentConfiguration").unwrap())
        };
        if is_kind_of {
            Ok(PKAddIdentityDocumentConfiguration(parent.0))
        } else {
            Err ("This PKAddSecureElementPassConfiguration cannot be downcasted to PKAddIdentityDocumentConfiguration" ,)
        }
    }
}
impl INSObject for PKAddIdentityDocumentConfiguration {}
impl PNSObject for PKAddIdentityDocumentConfiguration {}
impl IPKAddIdentityDocumentConfiguration for PKAddIdentityDocumentConfiguration {}
pub trait IPKAddIdentityDocumentConfiguration: Sized + std::ops::Deref {
    unsafe fn metadata(&self) -> PKIdentityDocumentMetadata
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, metadata)
    }
    unsafe fn configurationForMetadata_completion_(
        metadata: PKIdentityDocumentMetadata,
        completion: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"PKAddIdentityDocumentConfiguration").unwrap(), configurationForMetadata : metadata, completion : completion)
    }
}
pub type PKIssuerProvisioningExtensionAuthorizationResult = NSInteger;
pub trait PPKIssuerProvisioningExtensionAuthorizationProviding: Sized + std::ops::Deref {
    unsafe fn completionHandler(&self) -> *mut ::std::os::raw::c_void
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, completionHandler)
    }
    unsafe fn setCompletionHandler_(&self, completionHandler: *mut ::std::os::raw::c_void)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCompletionHandler : completionHandler)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct PKIssuerProvisioningExtensionHandler(pub id);
impl std::ops::Deref for PKIssuerProvisioningExtensionHandler {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for PKIssuerProvisioningExtensionHandler {}
impl PKIssuerProvisioningExtensionHandler {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"PKIssuerProvisioningExtensionHandler").unwrap(), alloc) })
    }
}
impl INSObject for PKIssuerProvisioningExtensionHandler {}
impl PNSObject for PKIssuerProvisioningExtensionHandler {}
impl std::convert::TryFrom<NSObject> for PKIssuerProvisioningExtensionHandler {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<PKIssuerProvisioningExtensionHandler, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"PKIssuerProvisioningExtensionHandler").unwrap())
        };
        if is_kind_of {
            Ok(PKIssuerProvisioningExtensionHandler(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to PKIssuerProvisioningExtensionHandler")
        }
    }
}
impl IPKIssuerProvisioningExtensionHandler for PKIssuerProvisioningExtensionHandler {}
pub trait IPKIssuerProvisioningExtensionHandler: Sized + std::ops::Deref {
    unsafe fn statusWithCompletion_(&self, completion: *mut ::std::os::raw::c_void)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, statusWithCompletion : completion)
    }
    unsafe fn passEntriesWithCompletion_(&self, completion: *mut ::std::os::raw::c_void)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, passEntriesWithCompletion : completion)
    }
    unsafe fn remotePassEntriesWithCompletion_(&self, completion: *mut ::std::os::raw::c_void)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, remotePassEntriesWithCompletion : completion)
    }
    unsafe fn generateAddPaymentPassRequestForPassEntryWithIdentifier_configuration_certificateChain_nonce_nonceSignature_completionHandler_(
        &self,
        identifier: NSString,
        configuration: PKAddPaymentPassRequestConfiguration,
        certificates: NSArray,
        nonce: NSData,
        nonceSignature: NSData,
        completion: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, generateAddPaymentPassRequestForPassEntryWithIdentifier : identifier, configuration : configuration, certificateChain : certificates, nonce : nonce, nonceSignature : nonceSignature, completionHandler : completion)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct PKIssuerProvisioningExtensionStatus(pub id);
impl std::ops::Deref for PKIssuerProvisioningExtensionStatus {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for PKIssuerProvisioningExtensionStatus {}
impl PKIssuerProvisioningExtensionStatus {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"PKIssuerProvisioningExtensionStatus").unwrap(), alloc) })
    }
}
impl INSObject for PKIssuerProvisioningExtensionStatus {}
impl PNSObject for PKIssuerProvisioningExtensionStatus {}
impl std::convert::TryFrom<NSObject> for PKIssuerProvisioningExtensionStatus {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<PKIssuerProvisioningExtensionStatus, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"PKIssuerProvisioningExtensionStatus").unwrap())
        };
        if is_kind_of {
            Ok(PKIssuerProvisioningExtensionStatus(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to PKIssuerProvisioningExtensionStatus")
        }
    }
}
impl IPKIssuerProvisioningExtensionStatus for PKIssuerProvisioningExtensionStatus {}
pub trait IPKIssuerProvisioningExtensionStatus: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn requiresAuthentication(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, requiresAuthentication)
    }
    unsafe fn setRequiresAuthentication_(&self, requiresAuthentication: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRequiresAuthentication : requiresAuthentication)
    }
    unsafe fn passEntriesAvailable(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, passEntriesAvailable)
    }
    unsafe fn setPassEntriesAvailable_(&self, passEntriesAvailable: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPassEntriesAvailable : passEntriesAvailable)
    }
    unsafe fn remotePassEntriesAvailable(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, remotePassEntriesAvailable)
    }
    unsafe fn setRemotePassEntriesAvailable_(&self, remotePassEntriesAvailable: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRemotePassEntriesAvailable : remotePassEntriesAvailable)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct PKIssuerProvisioningExtensionPassEntry(pub id);
impl std::ops::Deref for PKIssuerProvisioningExtensionPassEntry {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for PKIssuerProvisioningExtensionPassEntry {}
impl PKIssuerProvisioningExtensionPassEntry {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"PKIssuerProvisioningExtensionPassEntry").unwrap(), alloc) })
    }
}
impl INSObject for PKIssuerProvisioningExtensionPassEntry {}
impl PNSObject for PKIssuerProvisioningExtensionPassEntry {}
impl std::convert::TryFrom<NSObject> for PKIssuerProvisioningExtensionPassEntry {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<PKIssuerProvisioningExtensionPassEntry, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"PKIssuerProvisioningExtensionPassEntry").unwrap())
        };
        if is_kind_of {
            Ok(PKIssuerProvisioningExtensionPassEntry(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to PKIssuerProvisioningExtensionPassEntry")
        }
    }
}
impl IPKIssuerProvisioningExtensionPassEntry for PKIssuerProvisioningExtensionPassEntry {}
pub trait IPKIssuerProvisioningExtensionPassEntry: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn identifier(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, identifier)
    }
    unsafe fn title(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, title)
    }
    unsafe fn art(&self) -> CGImageRef
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, art)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct PKIssuerProvisioningExtensionPaymentPassEntry(pub id);
impl std::ops::Deref for PKIssuerProvisioningExtensionPaymentPassEntry {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for PKIssuerProvisioningExtensionPaymentPassEntry {}
impl PKIssuerProvisioningExtensionPaymentPassEntry {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"PKIssuerProvisioningExtensionPaymentPassEntry").unwrap(), alloc) })
    }
}
impl IPKIssuerProvisioningExtensionPassEntry for PKIssuerProvisioningExtensionPaymentPassEntry {}
impl From<PKIssuerProvisioningExtensionPaymentPassEntry>
    for PKIssuerProvisioningExtensionPassEntry
{
    fn from(
        child: PKIssuerProvisioningExtensionPaymentPassEntry,
    ) -> PKIssuerProvisioningExtensionPassEntry {
        PKIssuerProvisioningExtensionPassEntry(child.0)
    }
}
impl std::convert::TryFrom<PKIssuerProvisioningExtensionPassEntry>
    for PKIssuerProvisioningExtensionPaymentPassEntry
{
    type Error = &'static str;
    fn try_from(
        parent: PKIssuerProvisioningExtensionPassEntry,
    ) -> Result<PKIssuerProvisioningExtensionPaymentPassEntry, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"PKIssuerProvisioningExtensionPaymentPassEntry").unwrap())
        };
        if is_kind_of {
            Ok(PKIssuerProvisioningExtensionPaymentPassEntry(parent.0))
        } else {
            Err ("This PKIssuerProvisioningExtensionPassEntry cannot be downcasted to PKIssuerProvisioningExtensionPaymentPassEntry" ,)
        }
    }
}
impl INSObject for PKIssuerProvisioningExtensionPaymentPassEntry {}
impl PNSObject for PKIssuerProvisioningExtensionPaymentPassEntry {}
impl IPKIssuerProvisioningExtensionPaymentPassEntry
    for PKIssuerProvisioningExtensionPaymentPassEntry
{
}
pub trait IPKIssuerProvisioningExtensionPaymentPassEntry: Sized + std::ops::Deref {
    unsafe fn initWithIdentifier_title_art_addRequestConfiguration_(
        &self,
        identifier: NSString,
        title: NSString,
        art: CGImageRef,
        configuration: PKAddPaymentPassRequestConfiguration,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithIdentifier : identifier, title : title, art : art, addRequestConfiguration : configuration)
    }
    unsafe fn addRequestConfiguration(&self) -> PKAddPaymentPassRequestConfiguration
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, addRequestConfiguration)
    }
}
pub type PKVehicleConnectionErrorCode = NSInteger;
pub type PKVehicleConnectionSessionConnectionState = NSInteger;
pub trait PPKVehicleConnectionDelegate: Sized + std::ops::Deref {
    unsafe fn sessionDidChangeConnectionState_(
        &self,
        newState: PKVehicleConnectionSessionConnectionState,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, sessionDidChangeConnectionState : newState)
    }
    unsafe fn sessionDidReceiveData_(&self, data: NSData)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, sessionDidReceiveData : data)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct PKVehicleConnectionSession(pub id);
impl std::ops::Deref for PKVehicleConnectionSession {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for PKVehicleConnectionSession {}
impl PKVehicleConnectionSession {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"PKVehicleConnectionSession").unwrap(), alloc) })
    }
}
impl INSObject for PKVehicleConnectionSession {}
impl PNSObject for PKVehicleConnectionSession {}
impl std::convert::TryFrom<NSObject> for PKVehicleConnectionSession {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<PKVehicleConnectionSession, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"PKVehicleConnectionSession").unwrap()) };
        if is_kind_of {
            Ok(PKVehicleConnectionSession(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to PKVehicleConnectionSession")
        }
    }
}
impl IPKVehicleConnectionSession for PKVehicleConnectionSession {}
pub trait IPKVehicleConnectionSession: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn sendData_error_(&self, message: NSData, error: *mut NSError) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, sendData : message, error : error)
    }
    unsafe fn invalidate(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, invalidate)
    }
    unsafe fn delegate(&self) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, delegate)
    }
    unsafe fn connectionStatus(&self) -> PKVehicleConnectionSessionConnectionState
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, connectionStatus)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"PKVehicleConnectionSession").unwrap(), new)
    }
    unsafe fn sessionForPass_delegate_completion_(
        pass: PKSecureElementPass,
        delegate: *mut u64,
        completion: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"PKVehicleConnectionSession").unwrap(), sessionForPass : pass, delegate : delegate, completion : completion)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct PKIdentityAuthorizationController(pub id);
impl std::ops::Deref for PKIdentityAuthorizationController {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for PKIdentityAuthorizationController {}
impl PKIdentityAuthorizationController {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"PKIdentityAuthorizationController").unwrap(), alloc) })
    }
}
impl INSObject for PKIdentityAuthorizationController {}
impl PNSObject for PKIdentityAuthorizationController {}
impl std::convert::TryFrom<NSObject> for PKIdentityAuthorizationController {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<PKIdentityAuthorizationController, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"PKIdentityAuthorizationController").unwrap())
        };
        if is_kind_of {
            Ok(PKIdentityAuthorizationController(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to PKIdentityAuthorizationController")
        }
    }
}
impl IPKIdentityAuthorizationController for PKIdentityAuthorizationController {}
pub trait IPKIdentityAuthorizationController: Sized + std::ops::Deref {
    unsafe fn checkCanRequestDocument_completion_(
        &self,
        descriptor: *mut u64,
        completion: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, checkCanRequestDocument : descriptor, completion : completion)
    }
    unsafe fn requestDocument_completion_(
        &self,
        request: PKIdentityRequest,
        completion: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, requestDocument : request, completion : completion)
    }
    unsafe fn cancelRequest(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, cancelRequest)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct PKIdentityDocument(pub id);
impl std::ops::Deref for PKIdentityDocument {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for PKIdentityDocument {}
impl PKIdentityDocument {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"PKIdentityDocument").unwrap(), alloc) })
    }
}
impl INSObject for PKIdentityDocument {}
impl PNSObject for PKIdentityDocument {}
impl std::convert::TryFrom<NSObject> for PKIdentityDocument {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<PKIdentityDocument, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"PKIdentityDocument").unwrap()) };
        if is_kind_of {
            Ok(PKIdentityDocument(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to PKIdentityDocument")
        }
    }
}
impl IPKIdentityDocument for PKIdentityDocument {}
pub trait IPKIdentityDocument: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn encryptedData(&self) -> NSData
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, encryptedData)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"PKIdentityDocument").unwrap(), new)
    }
}
pub trait PPKIdentityDocumentDescriptor: Sized + std::ops::Deref {
    unsafe fn intentToStoreForElement_(&self, element: PKIdentityElement) -> PKIdentityIntentToStore
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, intentToStoreForElement : element)
    }
    unsafe fn addElements_withIntentToStore_(
        &self,
        elements: NSArray,
        intentToStore: PKIdentityIntentToStore,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, addElements : elements, withIntentToStore : intentToStore)
    }
    unsafe fn elements(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, elements)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct PKIdentityDriversLicenseDescriptor(pub id);
impl std::ops::Deref for PKIdentityDriversLicenseDescriptor {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for PKIdentityDriversLicenseDescriptor {}
impl PKIdentityDriversLicenseDescriptor {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"PKIdentityDriversLicenseDescriptor").unwrap(), alloc) })
    }
}
impl PPKIdentityDocumentDescriptor for PKIdentityDriversLicenseDescriptor {}
impl INSObject for PKIdentityDriversLicenseDescriptor {}
impl PNSObject for PKIdentityDriversLicenseDescriptor {}
impl std::convert::TryFrom<NSObject> for PKIdentityDriversLicenseDescriptor {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<PKIdentityDriversLicenseDescriptor, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"PKIdentityDriversLicenseDescriptor").unwrap())
        };
        if is_kind_of {
            Ok(PKIdentityDriversLicenseDescriptor(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to PKIdentityDriversLicenseDescriptor")
        }
    }
}
impl IPKIdentityDriversLicenseDescriptor for PKIdentityDriversLicenseDescriptor {}
pub trait IPKIdentityDriversLicenseDescriptor: Sized + std::ops::Deref {}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct PKIdentityNationalIDCardDescriptor(pub id);
impl std::ops::Deref for PKIdentityNationalIDCardDescriptor {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for PKIdentityNationalIDCardDescriptor {}
impl PKIdentityNationalIDCardDescriptor {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"PKIdentityNationalIDCardDescriptor").unwrap(), alloc) })
    }
}
impl PPKIdentityDocumentDescriptor for PKIdentityNationalIDCardDescriptor {}
impl INSObject for PKIdentityNationalIDCardDescriptor {}
impl PNSObject for PKIdentityNationalIDCardDescriptor {}
impl std::convert::TryFrom<NSObject> for PKIdentityNationalIDCardDescriptor {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<PKIdentityNationalIDCardDescriptor, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"PKIdentityNationalIDCardDescriptor").unwrap())
        };
        if is_kind_of {
            Ok(PKIdentityNationalIDCardDescriptor(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to PKIdentityNationalIDCardDescriptor")
        }
    }
}
impl IPKIdentityNationalIDCardDescriptor for PKIdentityNationalIDCardDescriptor {}
pub trait IPKIdentityNationalIDCardDescriptor: Sized + std::ops::Deref {
    unsafe fn regionCode(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, regionCode)
    }
    unsafe fn setRegionCode_(&self, regionCode: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRegionCode : regionCode)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct PKIdentityPhotoIDDescriptor(pub id);
impl std::ops::Deref for PKIdentityPhotoIDDescriptor {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for PKIdentityPhotoIDDescriptor {}
impl PKIdentityPhotoIDDescriptor {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"PKIdentityPhotoIDDescriptor").unwrap(), alloc) })
    }
}
impl PPKIdentityDocumentDescriptor for PKIdentityPhotoIDDescriptor {}
impl INSObject for PKIdentityPhotoIDDescriptor {}
impl PNSObject for PKIdentityPhotoIDDescriptor {}
impl std::convert::TryFrom<NSObject> for PKIdentityPhotoIDDescriptor {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<PKIdentityPhotoIDDescriptor, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"PKIdentityPhotoIDDescriptor").unwrap()) };
        if is_kind_of {
            Ok(PKIdentityPhotoIDDescriptor(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to PKIdentityPhotoIDDescriptor")
        }
    }
}
impl IPKIdentityPhotoIDDescriptor for PKIdentityPhotoIDDescriptor {}
pub trait IPKIdentityPhotoIDDescriptor: Sized + std::ops::Deref {}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct PKIdentityAnyOfDescriptor(pub id);
impl std::ops::Deref for PKIdentityAnyOfDescriptor {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for PKIdentityAnyOfDescriptor {}
impl PKIdentityAnyOfDescriptor {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"PKIdentityAnyOfDescriptor").unwrap(), alloc) })
    }
}
impl PPKIdentityDocumentDescriptor for PKIdentityAnyOfDescriptor {}
impl INSObject for PKIdentityAnyOfDescriptor {}
impl PNSObject for PKIdentityAnyOfDescriptor {}
impl std::convert::TryFrom<NSObject> for PKIdentityAnyOfDescriptor {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<PKIdentityAnyOfDescriptor, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"PKIdentityAnyOfDescriptor").unwrap()) };
        if is_kind_of {
            Ok(PKIdentityAnyOfDescriptor(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to PKIdentityAnyOfDescriptor")
        }
    }
}
impl IPKIdentityAnyOfDescriptor for PKIdentityAnyOfDescriptor {}
pub trait IPKIdentityAnyOfDescriptor: Sized + std::ops::Deref {
    unsafe fn initWithDescriptors_(&self, descriptors: NSArray) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithDescriptors : descriptors)
    }
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn descriptors(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, descriptors)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"PKIdentityAnyOfDescriptor").unwrap(), new)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct PKIdentityElement(pub id);
impl std::ops::Deref for PKIdentityElement {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for PKIdentityElement {}
impl PKIdentityElement {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"PKIdentityElement").unwrap(), alloc) })
    }
}
impl PNSCopying for PKIdentityElement {}
impl INSObject for PKIdentityElement {}
impl PNSObject for PKIdentityElement {}
impl std::convert::TryFrom<NSObject> for PKIdentityElement {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<PKIdentityElement, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"PKIdentityElement").unwrap()) };
        if is_kind_of {
            Ok(PKIdentityElement(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to PKIdentityElement")
        }
    }
}
impl IPKIdentityElement for PKIdentityElement {}
pub trait IPKIdentityElement: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn ageThresholdElementWithAge_(age: NSInteger) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"PKIdentityElement").unwrap(), ageThresholdElementWithAge : age)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"PKIdentityElement").unwrap(), new)
    }
    unsafe fn givenNameElement() -> PKIdentityElement
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"PKIdentityElement").unwrap(), givenNameElement)
    }
    unsafe fn familyNameElement() -> PKIdentityElement
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"PKIdentityElement").unwrap(), familyNameElement)
    }
    unsafe fn portraitElement() -> PKIdentityElement
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"PKIdentityElement").unwrap(), portraitElement)
    }
    unsafe fn addressElement() -> PKIdentityElement
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"PKIdentityElement").unwrap(), addressElement)
    }
    unsafe fn heightElement() -> PKIdentityElement
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"PKIdentityElement").unwrap(), heightElement)
    }
    unsafe fn weightElement() -> PKIdentityElement
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"PKIdentityElement").unwrap(), weightElement)
    }
    unsafe fn eyeColorElement() -> PKIdentityElement
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"PKIdentityElement").unwrap(), eyeColorElement)
    }
    unsafe fn hairColorElement() -> PKIdentityElement
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"PKIdentityElement").unwrap(), hairColorElement)
    }
    unsafe fn organDonorStatusElement() -> PKIdentityElement
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"PKIdentityElement").unwrap(), organDonorStatusElement)
    }
    unsafe fn veteranStatusElement() -> PKIdentityElement
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"PKIdentityElement").unwrap(), veteranStatusElement)
    }
    unsafe fn issuingAuthorityElement() -> PKIdentityElement
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"PKIdentityElement").unwrap(), issuingAuthorityElement)
    }
    unsafe fn documentIssueDateElement() -> PKIdentityElement
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"PKIdentityElement").unwrap(), documentIssueDateElement)
    }
    unsafe fn documentExpirationDateElement() -> PKIdentityElement
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"PKIdentityElement").unwrap(), documentExpirationDateElement)
    }
    unsafe fn documentDHSComplianceStatusElement() -> PKIdentityElement
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"PKIdentityElement").unwrap(), documentDHSComplianceStatusElement)
    }
    unsafe fn documentNumberElement() -> PKIdentityElement
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"PKIdentityElement").unwrap(), documentNumberElement)
    }
    unsafe fn drivingPrivilegesElement() -> PKIdentityElement
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"PKIdentityElement").unwrap(), drivingPrivilegesElement)
    }
    unsafe fn ageElement() -> PKIdentityElement
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"PKIdentityElement").unwrap(), ageElement)
    }
    unsafe fn dateOfBirthElement() -> PKIdentityElement
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"PKIdentityElement").unwrap(), dateOfBirthElement)
    }
    unsafe fn sexElement() -> PKIdentityElement
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"PKIdentityElement").unwrap(), sexElement)
    }
}
pub type PKIdentityError = NSInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct PKIdentityIntentToStore(pub id);
impl std::ops::Deref for PKIdentityIntentToStore {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for PKIdentityIntentToStore {}
impl PKIdentityIntentToStore {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"PKIdentityIntentToStore").unwrap(), alloc) })
    }
}
impl PNSCopying for PKIdentityIntentToStore {}
impl INSObject for PKIdentityIntentToStore {}
impl PNSObject for PKIdentityIntentToStore {}
impl std::convert::TryFrom<NSObject> for PKIdentityIntentToStore {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<PKIdentityIntentToStore, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"PKIdentityIntentToStore").unwrap()) };
        if is_kind_of {
            Ok(PKIdentityIntentToStore(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to PKIdentityIntentToStore")
        }
    }
}
impl IPKIdentityIntentToStore for PKIdentityIntentToStore {}
pub trait IPKIdentityIntentToStore: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn mayStoreIntentForDays_(days: NSInteger) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"PKIdentityIntentToStore").unwrap(), mayStoreIntentForDays : days)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"PKIdentityIntentToStore").unwrap(), new)
    }
    unsafe fn willNotStoreIntent() -> PKIdentityIntentToStore
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"PKIdentityIntentToStore").unwrap(), willNotStoreIntent)
    }
    unsafe fn mayStoreIntent() -> PKIdentityIntentToStore
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"PKIdentityIntentToStore").unwrap(), mayStoreIntent)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct PKIdentityRequest(pub id);
impl std::ops::Deref for PKIdentityRequest {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for PKIdentityRequest {}
impl PKIdentityRequest {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"PKIdentityRequest").unwrap(), alloc) })
    }
}
impl INSObject for PKIdentityRequest {}
impl PNSObject for PKIdentityRequest {}
impl std::convert::TryFrom<NSObject> for PKIdentityRequest {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<PKIdentityRequest, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"PKIdentityRequest").unwrap()) };
        if is_kind_of {
            Ok(PKIdentityRequest(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to PKIdentityRequest")
        }
    }
}
impl IPKIdentityRequest for PKIdentityRequest {}
pub trait IPKIdentityRequest: Sized + std::ops::Deref {
    unsafe fn descriptor(&self) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, descriptor)
    }
    unsafe fn setDescriptor_(&self, descriptor: *mut u64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDescriptor : descriptor)
    }
    unsafe fn nonce(&self) -> NSData
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, nonce)
    }
    unsafe fn setNonce_(&self, nonce: NSData)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setNonce : nonce)
    }
    unsafe fn merchantIdentifier(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, merchantIdentifier)
    }
    unsafe fn setMerchantIdentifier_(&self, merchantIdentifier: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMerchantIdentifier : merchantIdentifier)
    }
    unsafe fn usageDescriptionKey(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, usageDescriptionKey)
    }
    unsafe fn setUsageDescriptionKey_(&self, usageDescriptionKey: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setUsageDescriptionKey : usageDescriptionKey)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct PKPassRelevantDate(pub id);
impl std::ops::Deref for PKPassRelevantDate {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for PKPassRelevantDate {}
impl PKPassRelevantDate {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"PKPassRelevantDate").unwrap(), alloc) })
    }
}
impl INSObject for PKPassRelevantDate {}
impl PNSObject for PKPassRelevantDate {}
impl std::convert::TryFrom<NSObject> for PKPassRelevantDate {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<PKPassRelevantDate, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"PKPassRelevantDate").unwrap()) };
        if is_kind_of {
            Ok(PKPassRelevantDate(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to PKPassRelevantDate")
        }
    }
}
impl IPKPassRelevantDate for PKPassRelevantDate {}
pub trait IPKPassRelevantDate: Sized + std::ops::Deref {
    unsafe fn interval(&self) -> NSDateInterval
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, interval)
    }
    unsafe fn date(&self) -> NSDate
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, date)
    }
}
impl PKPassRelevantDate_ for PKPassRelevantDate {}
pub trait PKPassRelevantDate_: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"PKPassRelevantDate").unwrap(), new)
    }
}
unsafe extern "C" {
    pub static PKPassKitErrorDomain: NSString;
}
unsafe extern "C" {
    pub static PKPaymentErrorDomain: NSString;
}
unsafe extern "C" {
    pub static PKDisbursementErrorDomain: NSString;
}
unsafe extern "C" {
    pub static PKAddSecureElementPassErrorDomain: NSString;
}
unsafe extern "C" {
    pub static PKShareSecureElementPassErrorDomain: NSString;
}
unsafe extern "C" {
    pub static PKIdentityErrorDomain: NSErrorDomain;
}

unsafe impl objc2::encode::RefEncode for PKObject {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for PKObject {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for PKPass {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for PKPass {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for PKSecureElementPass {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for PKSecureElementPass {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for PKPaymentPass {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for PKPaymentPass {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for PKPassLibrary {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for PKPassLibrary {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for PKLabeledValue {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for PKLabeledValue {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for PKContact {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for PKContact {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for PKStoredValuePassProperties {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for PKStoredValuePassProperties {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for PKStoredValuePassBalance {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for PKStoredValuePassBalance {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for PKTransitPassProperties {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for PKTransitPassProperties {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for PKSuicaPassProperties {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for PKSuicaPassProperties {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for PKPaymentSummaryItem {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for PKPaymentSummaryItem {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for PKDeferredPaymentSummaryItem {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for PKDeferredPaymentSummaryItem {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for PKRecurringPaymentSummaryItem {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for PKRecurringPaymentSummaryItem {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for PKDateComponentsRange {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for PKDateComponentsRange {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for PKShippingMethod {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for PKShippingMethod {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for PKAutomaticReloadPaymentRequest {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for PKAutomaticReloadPaymentRequest {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for PKRecurringPaymentRequest {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for PKRecurringPaymentRequest {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for PKDeferredPaymentRequest {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for PKDeferredPaymentRequest {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for PKPaymentRequest {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for PKPaymentRequest {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for PKAutomaticReloadPaymentSummaryItem {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for PKAutomaticReloadPaymentSummaryItem {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for PKPaymentTokenContext {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for PKPaymentTokenContext {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for PKPaymentAuthorizationResult {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for PKPaymentAuthorizationResult {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for PKPaymentRequestUpdate {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for PKPaymentRequestUpdate {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for PKPaymentRequestShippingContactUpdate {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for PKPaymentRequestShippingContactUpdate {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for PKPaymentRequestShippingMethodUpdate {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for PKPaymentRequestShippingMethodUpdate {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for PKPaymentRequestPaymentMethodUpdate {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for PKPaymentRequestPaymentMethodUpdate {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for PKPaymentRequestMerchantSessionUpdate {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for PKPaymentRequestMerchantSessionUpdate {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for PKPaymentRequestCouponCodeUpdate {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for PKPaymentRequestCouponCodeUpdate {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for PKPaymentOrderDetails {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for PKPaymentOrderDetails {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for PKPaymentToken {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for PKPaymentToken {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for PKPayment {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for PKPayment {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for PKPaymentMethod {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for PKPaymentMethod {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for PKPaymentAuthorizationViewController {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for PKPaymentAuthorizationViewController {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for PKPaymentAuthorizationController {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for PKPaymentAuthorizationController {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for PKPaymentButton {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for PKPaymentButton {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for PKAddPaymentPassRequestConfiguration {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for PKAddPaymentPassRequestConfiguration {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for PKAddPaymentPassRequest {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for PKAddPaymentPassRequest {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for PKDisbursementRequest {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for PKDisbursementRequest {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for PKInstantFundsOutFeeSummaryItem {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for PKInstantFundsOutFeeSummaryItem {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for PKDisbursementSummaryItem {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for PKDisbursementSummaryItem {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for PKBarcodeEventMetadataRequest {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for PKBarcodeEventMetadataRequest {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for PKBarcodeEventMetadataResponse {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for PKBarcodeEventMetadataResponse {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for PKBarcodeEventSignatureRequest {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for PKBarcodeEventSignatureRequest {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for PKBarcodeEventSignatureResponse {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for PKBarcodeEventSignatureResponse {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for PKBarcodeEventConfigurationRequest {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for PKBarcodeEventConfigurationRequest {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for PKPaymentInformationEventExtension {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for PKPaymentInformationEventExtension {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for PKAddPassMetadataPreview {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for PKAddPassMetadataPreview {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for PKAddSecureElementPassConfiguration {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for PKAddSecureElementPassConfiguration {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for PKAddCarKeyPassConfiguration {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for PKAddCarKeyPassConfiguration {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for PKPaymentMerchantSession {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for PKPaymentMerchantSession {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for PKShareablePassMetadataPreview {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for PKShareablePassMetadataPreview {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for PKShareablePassMetadata {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for PKShareablePassMetadata {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for PKAddShareablePassConfiguration {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for PKAddShareablePassConfiguration {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for PKIdentityDocumentMetadata {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for PKIdentityDocumentMetadata {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for PKAddIdentityDocumentMetadata {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for PKAddIdentityDocumentMetadata {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for PKJapanIndividualNumberCardMetadata {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for PKJapanIndividualNumberCardMetadata {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for PKAddIdentityDocumentConfiguration {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for PKAddIdentityDocumentConfiguration {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for PKIssuerProvisioningExtensionHandler {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for PKIssuerProvisioningExtensionHandler {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for PKIssuerProvisioningExtensionStatus {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for PKIssuerProvisioningExtensionStatus {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for PKIssuerProvisioningExtensionPassEntry {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for PKIssuerProvisioningExtensionPassEntry {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for PKIssuerProvisioningExtensionPaymentPassEntry {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for PKIssuerProvisioningExtensionPaymentPassEntry {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for PKVehicleConnectionSession {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for PKVehicleConnectionSession {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for PKIdentityAuthorizationController {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for PKIdentityAuthorizationController {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for PKIdentityDocument {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for PKIdentityDocument {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for PKIdentityDriversLicenseDescriptor {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for PKIdentityDriversLicenseDescriptor {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for PKIdentityNationalIDCardDescriptor {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for PKIdentityNationalIDCardDescriptor {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for PKIdentityPhotoIDDescriptor {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for PKIdentityPhotoIDDescriptor {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for PKIdentityAnyOfDescriptor {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for PKIdentityAnyOfDescriptor {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for PKIdentityElement {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for PKIdentityElement {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for PKIdentityIntentToStore {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for PKIdentityIntentToStore {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for PKIdentityRequest {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for PKIdentityRequest {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for PKPassRelevantDate {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for PKPassRelevantDate {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
