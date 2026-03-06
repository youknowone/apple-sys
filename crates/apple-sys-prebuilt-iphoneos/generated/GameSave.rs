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
pub type GSSyncState = NSInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct GSSyncedDirectoryVersion(pub id);
impl std::ops::Deref for GSSyncedDirectoryVersion {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for GSSyncedDirectoryVersion {}
impl GSSyncedDirectoryVersion {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"GSSyncedDirectoryVersion").unwrap(), alloc) })
    }
}
impl INSObject for GSSyncedDirectoryVersion {}
impl PNSObject for GSSyncedDirectoryVersion {}
impl std::convert::TryFrom<NSObject> for GSSyncedDirectoryVersion {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<GSSyncedDirectoryVersion, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"GSSyncedDirectoryVersion").unwrap()) };
        if is_kind_of {
            Ok(GSSyncedDirectoryVersion(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to GSSyncedDirectoryVersion")
        }
    }
}
impl IGSSyncedDirectoryVersion for GSSyncedDirectoryVersion {}
pub trait IGSSyncedDirectoryVersion: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn new(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, new)
    }
    unsafe fn isLocal(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isLocal)
    }
    unsafe fn localizedNameOfSavingComputer(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, localizedNameOfSavingComputer)
    }
    unsafe fn modifiedDate(&self) -> NSDate
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, modifiedDate)
    }
    unsafe fn url(&self) -> NSURL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, url)
    }
    unsafe fn description(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, description)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct GSSyncedDirectoryState(pub id);
impl std::ops::Deref for GSSyncedDirectoryState {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for GSSyncedDirectoryState {}
impl GSSyncedDirectoryState {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"GSSyncedDirectoryState").unwrap(), alloc) })
    }
}
impl INSObject for GSSyncedDirectoryState {}
impl PNSObject for GSSyncedDirectoryState {}
impl std::convert::TryFrom<NSObject> for GSSyncedDirectoryState {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<GSSyncedDirectoryState, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"GSSyncedDirectoryState").unwrap()) };
        if is_kind_of {
            Ok(GSSyncedDirectoryState(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to GSSyncedDirectoryState")
        }
    }
}
impl IGSSyncedDirectoryState for GSSyncedDirectoryState {}
pub trait IGSSyncedDirectoryState: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn new(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, new)
    }
    unsafe fn state(&self) -> GSSyncState
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, state)
    }
    unsafe fn url(&self) -> NSURL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, url)
    }
    unsafe fn conflictedVersions(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, conflictedVersions)
    }
    unsafe fn error(&self) -> NSError
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, error)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct GSSyncedDirectory(pub id);
impl std::ops::Deref for GSSyncedDirectory {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for GSSyncedDirectory {}
impl GSSyncedDirectory {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"GSSyncedDirectory").unwrap(), alloc) })
    }
}
impl INSObject for GSSyncedDirectory {}
impl PNSObject for GSSyncedDirectory {}
impl std::convert::TryFrom<NSObject> for GSSyncedDirectory {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<GSSyncedDirectory, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"GSSyncedDirectory").unwrap()) };
        if is_kind_of {
            Ok(GSSyncedDirectory(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to GSSyncedDirectory")
        }
    }
}
impl IGSSyncedDirectory for GSSyncedDirectory {}
pub trait IGSSyncedDirectory: Sized + std::ops::Deref {
    unsafe fn close(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, close)
    }
    unsafe fn triggerPendingUploadWithCompletionHandler_(
        &self,
        completion: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, triggerPendingUploadWithCompletionHandler : completion)
    }
    unsafe fn resolveConflictsWithVersion_(&self, version: GSSyncedDirectoryVersion)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, resolveConflictsWithVersion : version)
    }
    unsafe fn finishSyncingWithCompletionHandler_(&self, completion: *mut ::std::os::raw::c_void)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, finishSyncingWithCompletionHandler : completion)
    }
    unsafe fn finishSyncing_completionHandler_(
        &self,
        statusDisplay: NSWindow,
        completion: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, finishSyncing : statusDisplay, completionHandler : completion)
    }
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn new(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, new)
    }
    unsafe fn directoryState(&self) -> GSSyncedDirectoryState
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, directoryState)
    }
    unsafe fn openDirectoryForContainerIdentifier_(
        containerIdentifier: NSString,
    ) -> GSSyncedDirectory
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"GSSyncedDirectory").unwrap(), openDirectoryForContainerIdentifier : containerIdentifier)
    }
}
unsafe extern "C" {
    pub static mut GameSaveVersionNumber: f64;
}
unsafe extern "C" {
    pub static GameSaveVersionString: [::std::os::raw::c_uchar; 0usize];
}

unsafe impl objc2::encode::RefEncode for GSSyncedDirectoryVersion {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for GSSyncedDirectoryVersion {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for GSSyncedDirectoryState {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for GSSyncedDirectoryState {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for GSSyncedDirectory {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for GSSyncedDirectory {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
