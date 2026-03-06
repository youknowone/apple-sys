#[allow(unused_imports)]
use crate::objc::*;
#[allow(unused_imports)]
use crate::AppKit::*;
#[allow(unused_imports)]
use crate::CoreServices::*;
#[allow(unused_imports)]
use crate::Foundation::*;
#[allow(unused_imports)]
use crate::Security::*;
#[allow(unused_imports)]
use libc::{gid_t, id_t, uid_t};

#[allow(unused_imports)]
use objc2::msg_send;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CBIdentity(pub id);
impl std::ops::Deref for CBIdentity {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CBIdentity {}
impl CBIdentity {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CBIdentity").unwrap(), alloc) })
    }
}
impl PNSCoding for CBIdentity {}
impl PNSCopying for CBIdentity {}
impl std::convert::TryFrom<NSObject> for CBIdentity {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<CBIdentity, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CBIdentity").unwrap()) };
        if is_kind_of {
            Ok(CBIdentity(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to CBIdentity")
        }
    }
}
impl ICBIdentity for CBIdentity {}
pub trait ICBIdentity: Sized + std::ops::Deref {
    unsafe fn isMemberOfGroup_(&self, group: CBGroupIdentity) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, isMemberOfGroup : group)
    }
    unsafe fn authority(&self) -> CBIdentityAuthority
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, authority)
    }
    unsafe fn uniqueIdentifier(&self) -> NSUUID
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, uniqueIdentifier)
    }
    unsafe fn UUIDString(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, UUIDString)
    }
    unsafe fn fullName(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, fullName)
    }
    unsafe fn posixName(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, posixName)
    }
    unsafe fn aliases(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, aliases)
    }
    unsafe fn emailAddress(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, emailAddress)
    }
    unsafe fn image(&self) -> NSImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, image)
    }
    unsafe fn persistentReference(&self) -> NSData
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, persistentReference)
    }
    unsafe fn isHidden(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isHidden)
    }
    unsafe fn CSIdentity(&self) -> CSIdentityRef
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, CSIdentity)
    }
    unsafe fn identityWithName_authority_(
        name: NSString,
        authority: CBIdentityAuthority,
    ) -> CBIdentity
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CBIdentity").unwrap(), identityWithName : name, authority : authority)
    }
    unsafe fn identityWithUniqueIdentifier_authority_(
        uuid: NSUUID,
        authority: CBIdentityAuthority,
    ) -> CBIdentity
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CBIdentity").unwrap(), identityWithUniqueIdentifier : uuid, authority : authority)
    }
    unsafe fn identityWithUUIDString_authority_(
        uuid: NSString,
        authority: CBIdentityAuthority,
    ) -> CBIdentity
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CBIdentity").unwrap(), identityWithUUIDString : uuid, authority : authority)
    }
    unsafe fn identityWithPersistentReference_(data: NSData) -> CBIdentity
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CBIdentity").unwrap(), identityWithPersistentReference : data)
    }
    unsafe fn identityWithCSIdentity_(csIdentity: CSIdentityRef) -> CBIdentity
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CBIdentity").unwrap(), identityWithCSIdentity : csIdentity)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CBUserIdentity(pub id);
impl std::ops::Deref for CBUserIdentity {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CBUserIdentity {}
impl CBUserIdentity {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CBUserIdentity").unwrap(), alloc) })
    }
}
impl PNSCoding for CBUserIdentity {}
impl PNSCopying for CBUserIdentity {}
impl ICBIdentity for CBUserIdentity {}
impl From<CBUserIdentity> for CBIdentity {
    fn from(child: CBUserIdentity) -> CBIdentity {
        CBIdentity(child.0)
    }
}
impl std::convert::TryFrom<CBIdentity> for CBUserIdentity {
    type Error = &'static str;
    fn try_from(parent: CBIdentity) -> Result<CBUserIdentity, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CBUserIdentity").unwrap()) };
        if is_kind_of {
            Ok(CBUserIdentity(parent.0))
        } else {
            Err("This CBIdentity cannot be downcasted to CBUserIdentity")
        }
    }
}
impl ICBUserIdentity for CBUserIdentity {}
pub trait ICBUserIdentity: Sized + std::ops::Deref {
    unsafe fn authenticateWithPassword_(&self, password: NSString) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, authenticateWithPassword : password)
    }
    unsafe fn posixUID(&self) -> uid_t
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, posixUID)
    }
    unsafe fn certificate(&self) -> SecCertificateRef
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, certificate)
    }
    unsafe fn isEnabled(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isEnabled)
    }
    unsafe fn userIdentityWithPosixUID_authority_(
        uid: uid_t,
        authority: CBIdentityAuthority,
    ) -> CBUserIdentity
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CBUserIdentity").unwrap(), userIdentityWithPosixUID : uid, authority : authority)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CBGroupIdentity(pub id);
impl std::ops::Deref for CBGroupIdentity {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CBGroupIdentity {}
impl CBGroupIdentity {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CBGroupIdentity").unwrap(), alloc) })
    }
}
impl ICBIdentity for CBGroupIdentity {}
impl PNSCoding for CBGroupIdentity {}
impl PNSCopying for CBGroupIdentity {}
impl std::convert::TryFrom<CBIdentity> for CBGroupIdentity {
    type Error = &'static str;
    fn try_from(parent: CBIdentity) -> Result<CBGroupIdentity, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CBGroupIdentity").unwrap()) };
        if is_kind_of {
            Ok(CBGroupIdentity(parent.0))
        } else {
            Err("This CBIdentity cannot be downcasted to CBGroupIdentity")
        }
    }
}
impl ICBGroupIdentity for CBGroupIdentity {}
pub trait ICBGroupIdentity: Sized + std::ops::Deref {
    unsafe fn posixGID(&self) -> gid_t
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, posixGID)
    }
    unsafe fn members(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, members)
    }
    unsafe fn memberIdentities(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, memberIdentities)
    }
    unsafe fn groupIdentityWithPosixGID_authority_(
        gid: gid_t,
        authority: CBIdentityAuthority,
    ) -> CBGroupIdentity
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CBGroupIdentity").unwrap(), groupIdentityWithPosixGID : gid, authority : authority)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CBIdentityPicker(pub id);
impl std::ops::Deref for CBIdentityPicker {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CBIdentityPicker {}
impl CBIdentityPicker {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CBIdentityPicker").unwrap(), alloc) })
    }
}
impl std::convert::TryFrom<NSObject> for CBIdentityPicker {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<CBIdentityPicker, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CBIdentityPicker").unwrap()) };
        if is_kind_of {
            Ok(CBIdentityPicker(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to CBIdentityPicker")
        }
    }
}
impl ICBIdentityPicker for CBIdentityPicker {}
pub trait ICBIdentityPicker: Sized + std::ops::Deref {
    unsafe fn runModal(&self) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, runModal)
    }
    unsafe fn runModalForWindow_modalDelegate_didEndSelector_contextInfo_(
        &self,
        window: NSWindow,
        delegate: id,
        didEndSelector: objc2::runtime::Sel,
        contextInfo: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, runModalForWindow : window, modalDelegate : delegate, didEndSelector : didEndSelector, contextInfo : contextInfo)
    }
    unsafe fn runModalForWindow_completionHandler_(
        &self,
        window: NSWindow,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, runModalForWindow : window, completionHandler : completionHandler)
    }
    unsafe fn title(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, title)
    }
    unsafe fn setTitle_(&self, title: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setTitle : title)
    }
    unsafe fn allowsMultipleSelection(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, allowsMultipleSelection)
    }
    unsafe fn setAllowsMultipleSelection_(&self, allowsMultipleSelection: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAllowsMultipleSelection : allowsMultipleSelection)
    }
    unsafe fn identities(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, identities)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CBIdentityAuthority(pub id);
impl std::ops::Deref for CBIdentityAuthority {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CBIdentityAuthority {}
impl CBIdentityAuthority {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CBIdentityAuthority").unwrap(), alloc) })
    }
}
impl std::convert::TryFrom<NSObject> for CBIdentityAuthority {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<CBIdentityAuthority, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CBIdentityAuthority").unwrap()) };
        if is_kind_of {
            Ok(CBIdentityAuthority(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to CBIdentityAuthority")
        }
    }
}
impl ICBIdentityAuthority for CBIdentityAuthority {}
pub trait ICBIdentityAuthority: Sized + std::ops::Deref {
    unsafe fn CSIdentityAuthority(&self) -> CSIdentityAuthorityRef
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, CSIdentityAuthority)
    }
    unsafe fn localizedName(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, localizedName)
    }
    unsafe fn localIdentityAuthority() -> CBIdentityAuthority
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CBIdentityAuthority").unwrap(), localIdentityAuthority)
    }
    unsafe fn managedIdentityAuthority() -> CBIdentityAuthority
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CBIdentityAuthority").unwrap(), managedIdentityAuthority)
    }
    unsafe fn defaultIdentityAuthority() -> CBIdentityAuthority
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CBIdentityAuthority").unwrap(), defaultIdentityAuthority)
    }
    unsafe fn identityAuthorityWithCSIdentityAuthority_(
        CSIdentityAuthority: CSIdentityAuthorityRef,
    ) -> CBIdentityAuthority
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CBIdentityAuthority").unwrap(), identityAuthorityWithCSIdentityAuthority : CSIdentityAuthority)
    }
}

unsafe impl objc2::encode::RefEncode for CBIdentity {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CBIdentity {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CBUserIdentity {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CBUserIdentity {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CBGroupIdentity {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CBGroupIdentity {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CBIdentityPicker {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CBIdentityPicker {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CBIdentityAuthority {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CBIdentityAuthority {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
