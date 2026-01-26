#[allow(unused_imports)]
use crate::objc::*;
#[allow(unused_imports)]
use crate::vmnet::*;
#[allow(unused_imports)]
use crate::AppKit::*;
#[allow(unused_imports)]
use crate::CoreFoundation::*;
#[allow(unused_imports)]
use crate::Foundation::*;

#[allow(unused_imports)]
use objc2::msg_send;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct VZAudioDeviceConfiguration(pub id);
impl std::ops::Deref for VZAudioDeviceConfiguration {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for VZAudioDeviceConfiguration {}
impl VZAudioDeviceConfiguration {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"VZAudioDeviceConfiguration").unwrap(), alloc) })
    }
}
impl PNSCopying for VZAudioDeviceConfiguration {}
impl INSObject for VZAudioDeviceConfiguration {}
impl PNSObject for VZAudioDeviceConfiguration {}
impl std::convert::TryFrom<NSObject> for VZAudioDeviceConfiguration {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<VZAudioDeviceConfiguration, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"VZAudioDeviceConfiguration").unwrap()) };
        if is_kind_of {
            Ok(VZAudioDeviceConfiguration(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to VZAudioDeviceConfiguration")
        }
    }
}
impl IVZAudioDeviceConfiguration for VZAudioDeviceConfiguration {}
pub trait IVZAudioDeviceConfiguration: Sized + std::ops::Deref {
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
        msg_send!(&*objc2::runtime::AnyClass::get(c"VZAudioDeviceConfiguration").unwrap(), new)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct VZAudioInputStreamSource(pub id);
impl std::ops::Deref for VZAudioInputStreamSource {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for VZAudioInputStreamSource {}
impl VZAudioInputStreamSource {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"VZAudioInputStreamSource").unwrap(), alloc) })
    }
}
impl INSObject for VZAudioInputStreamSource {}
impl PNSObject for VZAudioInputStreamSource {}
impl std::convert::TryFrom<NSObject> for VZAudioInputStreamSource {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<VZAudioInputStreamSource, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"VZAudioInputStreamSource").unwrap()) };
        if is_kind_of {
            Ok(VZAudioInputStreamSource(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to VZAudioInputStreamSource")
        }
    }
}
impl IVZAudioInputStreamSource for VZAudioInputStreamSource {}
pub trait IVZAudioInputStreamSource: Sized + std::ops::Deref {
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
        msg_send!(&*objc2::runtime::AnyClass::get(c"VZAudioInputStreamSource").unwrap(), new)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct VZAudioOutputStreamSink(pub id);
impl std::ops::Deref for VZAudioOutputStreamSink {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for VZAudioOutputStreamSink {}
impl VZAudioOutputStreamSink {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"VZAudioOutputStreamSink").unwrap(), alloc) })
    }
}
impl INSObject for VZAudioOutputStreamSink {}
impl PNSObject for VZAudioOutputStreamSink {}
impl std::convert::TryFrom<NSObject> for VZAudioOutputStreamSink {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<VZAudioOutputStreamSink, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"VZAudioOutputStreamSink").unwrap()) };
        if is_kind_of {
            Ok(VZAudioOutputStreamSink(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to VZAudioOutputStreamSink")
        }
    }
}
impl IVZAudioOutputStreamSink for VZAudioOutputStreamSink {}
pub trait IVZAudioOutputStreamSink: Sized + std::ops::Deref {
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
        msg_send!(&*objc2::runtime::AnyClass::get(c"VZAudioOutputStreamSink").unwrap(), new)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct VZBootLoader(pub id);
impl std::ops::Deref for VZBootLoader {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for VZBootLoader {}
impl VZBootLoader {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"VZBootLoader").unwrap(), alloc) })
    }
}
impl PNSCopying for VZBootLoader {}
impl INSObject for VZBootLoader {}
impl PNSObject for VZBootLoader {}
impl std::convert::TryFrom<NSObject> for VZBootLoader {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<VZBootLoader, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"VZBootLoader").unwrap()) };
        if is_kind_of {
            Ok(VZBootLoader(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to VZBootLoader")
        }
    }
}
impl IVZBootLoader for VZBootLoader {}
pub trait IVZBootLoader: Sized + std::ops::Deref {
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
        msg_send!(&*objc2::runtime::AnyClass::get(c"VZBootLoader").unwrap(), new)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct VZNetworkDeviceAttachment(pub id);
impl std::ops::Deref for VZNetworkDeviceAttachment {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for VZNetworkDeviceAttachment {}
impl VZNetworkDeviceAttachment {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"VZNetworkDeviceAttachment").unwrap(), alloc) })
    }
}
impl INSObject for VZNetworkDeviceAttachment {}
impl PNSObject for VZNetworkDeviceAttachment {}
impl std::convert::TryFrom<NSObject> for VZNetworkDeviceAttachment {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<VZNetworkDeviceAttachment, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"VZNetworkDeviceAttachment").unwrap()) };
        if is_kind_of {
            Ok(VZNetworkDeviceAttachment(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to VZNetworkDeviceAttachment")
        }
    }
}
impl IVZNetworkDeviceAttachment for VZNetworkDeviceAttachment {}
pub trait IVZNetworkDeviceAttachment: Sized + std::ops::Deref {
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
        msg_send!(&*objc2::runtime::AnyClass::get(c"VZNetworkDeviceAttachment").unwrap(), new)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct VZBridgedNetworkDeviceAttachment(pub id);
impl std::ops::Deref for VZBridgedNetworkDeviceAttachment {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for VZBridgedNetworkDeviceAttachment {}
impl VZBridgedNetworkDeviceAttachment {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"VZBridgedNetworkDeviceAttachment").unwrap(), alloc) })
    }
}
impl IVZNetworkDeviceAttachment for VZBridgedNetworkDeviceAttachment {}
impl From<VZBridgedNetworkDeviceAttachment> for VZNetworkDeviceAttachment {
    fn from(child: VZBridgedNetworkDeviceAttachment) -> VZNetworkDeviceAttachment {
        VZNetworkDeviceAttachment(child.0)
    }
}
impl std::convert::TryFrom<VZNetworkDeviceAttachment> for VZBridgedNetworkDeviceAttachment {
    type Error = &'static str;
    fn try_from(
        parent: VZNetworkDeviceAttachment,
    ) -> Result<VZBridgedNetworkDeviceAttachment, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"VZBridgedNetworkDeviceAttachment").unwrap())
        };
        if is_kind_of {
            Ok(VZBridgedNetworkDeviceAttachment(parent.0))
        } else {
            Err ("This VZNetworkDeviceAttachment cannot be downcasted to VZBridgedNetworkDeviceAttachment" ,)
        }
    }
}
impl INSObject for VZBridgedNetworkDeviceAttachment {}
impl PNSObject for VZBridgedNetworkDeviceAttachment {}
impl IVZBridgedNetworkDeviceAttachment for VZBridgedNetworkDeviceAttachment {}
pub trait IVZBridgedNetworkDeviceAttachment: Sized + std::ops::Deref {
    unsafe fn initWithInterface_(&self, interface: VZBridgedNetworkInterface) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithInterface : interface)
    }
    unsafe fn interface(&self) -> VZBridgedNetworkInterface
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, interface)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct VZBridgedNetworkInterface(pub id);
impl std::ops::Deref for VZBridgedNetworkInterface {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for VZBridgedNetworkInterface {}
impl VZBridgedNetworkInterface {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"VZBridgedNetworkInterface").unwrap(), alloc) })
    }
}
impl INSObject for VZBridgedNetworkInterface {}
impl PNSObject for VZBridgedNetworkInterface {}
impl std::convert::TryFrom<NSObject> for VZBridgedNetworkInterface {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<VZBridgedNetworkInterface, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"VZBridgedNetworkInterface").unwrap()) };
        if is_kind_of {
            Ok(VZBridgedNetworkInterface(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to VZBridgedNetworkInterface")
        }
    }
}
impl IVZBridgedNetworkInterface for VZBridgedNetworkInterface {}
pub trait IVZBridgedNetworkInterface: Sized + std::ops::Deref {
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
    unsafe fn localizedDisplayName(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, localizedDisplayName)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"VZBridgedNetworkInterface").unwrap(), new)
    }
    unsafe fn networkInterfaces() -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"VZBridgedNetworkInterface").unwrap(), networkInterfaces)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct VZConsoleDevice(pub id);
impl std::ops::Deref for VZConsoleDevice {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for VZConsoleDevice {}
impl VZConsoleDevice {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"VZConsoleDevice").unwrap(), alloc) })
    }
}
impl INSObject for VZConsoleDevice {}
impl PNSObject for VZConsoleDevice {}
impl std::convert::TryFrom<NSObject> for VZConsoleDevice {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<VZConsoleDevice, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"VZConsoleDevice").unwrap()) };
        if is_kind_of {
            Ok(VZConsoleDevice(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to VZConsoleDevice")
        }
    }
}
impl IVZConsoleDevice for VZConsoleDevice {}
pub trait IVZConsoleDevice: Sized + std::ops::Deref {
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
        msg_send!(&*objc2::runtime::AnyClass::get(c"VZConsoleDevice").unwrap(), new)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct VZConsoleDeviceConfiguration(pub id);
impl std::ops::Deref for VZConsoleDeviceConfiguration {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for VZConsoleDeviceConfiguration {}
impl VZConsoleDeviceConfiguration {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"VZConsoleDeviceConfiguration").unwrap(), alloc) })
    }
}
impl PNSCopying for VZConsoleDeviceConfiguration {}
impl INSObject for VZConsoleDeviceConfiguration {}
impl PNSObject for VZConsoleDeviceConfiguration {}
impl std::convert::TryFrom<NSObject> for VZConsoleDeviceConfiguration {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<VZConsoleDeviceConfiguration, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"VZConsoleDeviceConfiguration").unwrap()) };
        if is_kind_of {
            Ok(VZConsoleDeviceConfiguration(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to VZConsoleDeviceConfiguration")
        }
    }
}
impl IVZConsoleDeviceConfiguration for VZConsoleDeviceConfiguration {}
pub trait IVZConsoleDeviceConfiguration: Sized + std::ops::Deref {
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
        msg_send!(&*objc2::runtime::AnyClass::get(c"VZConsoleDeviceConfiguration").unwrap(), new)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct VZConsolePortConfiguration(pub id);
impl std::ops::Deref for VZConsolePortConfiguration {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for VZConsolePortConfiguration {}
impl VZConsolePortConfiguration {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"VZConsolePortConfiguration").unwrap(), alloc) })
    }
}
impl PNSCopying for VZConsolePortConfiguration {}
impl INSObject for VZConsolePortConfiguration {}
impl PNSObject for VZConsolePortConfiguration {}
impl std::convert::TryFrom<NSObject> for VZConsolePortConfiguration {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<VZConsolePortConfiguration, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"VZConsolePortConfiguration").unwrap()) };
        if is_kind_of {
            Ok(VZConsolePortConfiguration(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to VZConsolePortConfiguration")
        }
    }
}
impl IVZConsolePortConfiguration for VZConsolePortConfiguration {}
pub trait IVZConsolePortConfiguration: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn attachment(&self) -> VZSerialPortAttachment
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, attachment)
    }
    unsafe fn setAttachment_(&self, attachment: VZSerialPortAttachment)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAttachment : attachment)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"VZConsolePortConfiguration").unwrap(), new)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct VZDirectoryShare(pub id);
impl std::ops::Deref for VZDirectoryShare {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for VZDirectoryShare {}
impl VZDirectoryShare {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"VZDirectoryShare").unwrap(), alloc) })
    }
}
impl INSObject for VZDirectoryShare {}
impl PNSObject for VZDirectoryShare {}
impl std::convert::TryFrom<NSObject> for VZDirectoryShare {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<VZDirectoryShare, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"VZDirectoryShare").unwrap()) };
        if is_kind_of {
            Ok(VZDirectoryShare(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to VZDirectoryShare")
        }
    }
}
impl IVZDirectoryShare for VZDirectoryShare {}
pub trait IVZDirectoryShare: Sized + std::ops::Deref {
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
        msg_send!(&*objc2::runtime::AnyClass::get(c"VZDirectoryShare").unwrap(), new)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct VZDirectorySharingDevice(pub id);
impl std::ops::Deref for VZDirectorySharingDevice {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for VZDirectorySharingDevice {}
impl VZDirectorySharingDevice {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"VZDirectorySharingDevice").unwrap(), alloc) })
    }
}
impl INSObject for VZDirectorySharingDevice {}
impl PNSObject for VZDirectorySharingDevice {}
impl std::convert::TryFrom<NSObject> for VZDirectorySharingDevice {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<VZDirectorySharingDevice, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"VZDirectorySharingDevice").unwrap()) };
        if is_kind_of {
            Ok(VZDirectorySharingDevice(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to VZDirectorySharingDevice")
        }
    }
}
impl IVZDirectorySharingDevice for VZDirectorySharingDevice {}
pub trait IVZDirectorySharingDevice: Sized + std::ops::Deref {
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
        msg_send!(&*objc2::runtime::AnyClass::get(c"VZDirectorySharingDevice").unwrap(), new)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct VZDirectorySharingDeviceConfiguration(pub id);
impl std::ops::Deref for VZDirectorySharingDeviceConfiguration {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for VZDirectorySharingDeviceConfiguration {}
impl VZDirectorySharingDeviceConfiguration {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"VZDirectorySharingDeviceConfiguration").unwrap(), alloc) })
    }
}
impl PNSCopying for VZDirectorySharingDeviceConfiguration {}
impl INSObject for VZDirectorySharingDeviceConfiguration {}
impl PNSObject for VZDirectorySharingDeviceConfiguration {}
impl std::convert::TryFrom<NSObject> for VZDirectorySharingDeviceConfiguration {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<VZDirectorySharingDeviceConfiguration, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"VZDirectorySharingDeviceConfiguration").unwrap())
        };
        if is_kind_of {
            Ok(VZDirectorySharingDeviceConfiguration(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to VZDirectorySharingDeviceConfiguration")
        }
    }
}
impl IVZDirectorySharingDeviceConfiguration for VZDirectorySharingDeviceConfiguration {}
pub trait IVZDirectorySharingDeviceConfiguration: Sized + std::ops::Deref {
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
        msg_send!(&*objc2::runtime::AnyClass::get(c"VZDirectorySharingDeviceConfiguration").unwrap(), new)
    }
}
pub type VZDiskSynchronizationMode = NSInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct VZStorageDeviceAttachment(pub id);
impl std::ops::Deref for VZStorageDeviceAttachment {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for VZStorageDeviceAttachment {}
impl VZStorageDeviceAttachment {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"VZStorageDeviceAttachment").unwrap(), alloc) })
    }
}
impl INSObject for VZStorageDeviceAttachment {}
impl PNSObject for VZStorageDeviceAttachment {}
impl std::convert::TryFrom<NSObject> for VZStorageDeviceAttachment {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<VZStorageDeviceAttachment, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"VZStorageDeviceAttachment").unwrap()) };
        if is_kind_of {
            Ok(VZStorageDeviceAttachment(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to VZStorageDeviceAttachment")
        }
    }
}
impl IVZStorageDeviceAttachment for VZStorageDeviceAttachment {}
pub trait IVZStorageDeviceAttachment: Sized + std::ops::Deref {
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
        msg_send!(&*objc2::runtime::AnyClass::get(c"VZStorageDeviceAttachment").unwrap(), new)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct VZDiskBlockDeviceStorageDeviceAttachment(pub id);
impl std::ops::Deref for VZDiskBlockDeviceStorageDeviceAttachment {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for VZDiskBlockDeviceStorageDeviceAttachment {}
impl VZDiskBlockDeviceStorageDeviceAttachment {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"VZDiskBlockDeviceStorageDeviceAttachment").unwrap(), alloc) })
    }
}
impl IVZStorageDeviceAttachment for VZDiskBlockDeviceStorageDeviceAttachment {}
impl From<VZDiskBlockDeviceStorageDeviceAttachment> for VZStorageDeviceAttachment {
    fn from(child: VZDiskBlockDeviceStorageDeviceAttachment) -> VZStorageDeviceAttachment {
        VZStorageDeviceAttachment(child.0)
    }
}
impl std::convert::TryFrom<VZStorageDeviceAttachment> for VZDiskBlockDeviceStorageDeviceAttachment {
    type Error = &'static str;
    fn try_from(
        parent: VZStorageDeviceAttachment,
    ) -> Result<VZDiskBlockDeviceStorageDeviceAttachment, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"VZDiskBlockDeviceStorageDeviceAttachment").unwrap())
        };
        if is_kind_of {
            Ok(VZDiskBlockDeviceStorageDeviceAttachment(parent.0))
        } else {
            Err ("This VZStorageDeviceAttachment cannot be downcasted to VZDiskBlockDeviceStorageDeviceAttachment" ,)
        }
    }
}
impl INSObject for VZDiskBlockDeviceStorageDeviceAttachment {}
impl PNSObject for VZDiskBlockDeviceStorageDeviceAttachment {}
impl IVZDiskBlockDeviceStorageDeviceAttachment for VZDiskBlockDeviceStorageDeviceAttachment {}
pub trait IVZDiskBlockDeviceStorageDeviceAttachment: Sized + std::ops::Deref {
    unsafe fn initWithFileHandle_readOnly_synchronizationMode_error_(
        &self,
        fileHandle: NSFileHandle,
        readOnly: BOOL,
        synchronizationMode: VZDiskSynchronizationMode,
        error: *mut NSError,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithFileHandle : fileHandle, readOnly : readOnly, synchronizationMode : synchronizationMode, error : error)
    }
    unsafe fn fileHandle(&self) -> NSFileHandle
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, fileHandle)
    }
    unsafe fn isReadOnly(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isReadOnly)
    }
    unsafe fn synchronizationMode(&self) -> VZDiskSynchronizationMode
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, synchronizationMode)
    }
}
pub type VZDiskImageCachingMode = NSInteger;
pub type VZDiskImageSynchronizationMode = NSInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct VZDiskImageStorageDeviceAttachment(pub id);
impl std::ops::Deref for VZDiskImageStorageDeviceAttachment {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for VZDiskImageStorageDeviceAttachment {}
impl VZDiskImageStorageDeviceAttachment {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"VZDiskImageStorageDeviceAttachment").unwrap(), alloc) })
    }
}
impl IVZStorageDeviceAttachment for VZDiskImageStorageDeviceAttachment {}
impl std::convert::TryFrom<VZStorageDeviceAttachment> for VZDiskImageStorageDeviceAttachment {
    type Error = &'static str;
    fn try_from(
        parent: VZStorageDeviceAttachment,
    ) -> Result<VZDiskImageStorageDeviceAttachment, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"VZDiskImageStorageDeviceAttachment").unwrap())
        };
        if is_kind_of {
            Ok(VZDiskImageStorageDeviceAttachment(parent.0))
        } else {
            Err ("This VZStorageDeviceAttachment cannot be downcasted to VZDiskImageStorageDeviceAttachment" ,)
        }
    }
}
impl INSObject for VZDiskImageStorageDeviceAttachment {}
impl PNSObject for VZDiskImageStorageDeviceAttachment {}
impl IVZDiskImageStorageDeviceAttachment for VZDiskImageStorageDeviceAttachment {}
pub trait IVZDiskImageStorageDeviceAttachment: Sized + std::ops::Deref {
    unsafe fn initWithURL_readOnly_error_(
        &self,
        url: NSURL,
        readOnly: BOOL,
        error: *mut NSError,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithURL : url, readOnly : readOnly, error : error)
    }
    unsafe fn initWithURL_readOnly_cachingMode_synchronizationMode_error_(
        &self,
        url: NSURL,
        readOnly: BOOL,
        cachingMode: VZDiskImageCachingMode,
        synchronizationMode: VZDiskImageSynchronizationMode,
        error: *mut NSError,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithURL : url, readOnly : readOnly, cachingMode : cachingMode, synchronizationMode : synchronizationMode, error : error)
    }
    unsafe fn URL(&self) -> NSURL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, URL)
    }
    unsafe fn isReadOnly(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isReadOnly)
    }
    unsafe fn cachingMode(&self) -> VZDiskImageCachingMode
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, cachingMode)
    }
    unsafe fn synchronizationMode(&self) -> VZDiskImageSynchronizationMode
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, synchronizationMode)
    }
}
pub type VZEFIVariableStoreInitializationOptions = NSUInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct VZEFIVariableStore(pub id);
impl std::ops::Deref for VZEFIVariableStore {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for VZEFIVariableStore {}
impl VZEFIVariableStore {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"VZEFIVariableStore").unwrap(), alloc) })
    }
}
impl INSObject for VZEFIVariableStore {}
impl PNSObject for VZEFIVariableStore {}
impl std::convert::TryFrom<NSObject> for VZEFIVariableStore {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<VZEFIVariableStore, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"VZEFIVariableStore").unwrap()) };
        if is_kind_of {
            Ok(VZEFIVariableStore(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to VZEFIVariableStore")
        }
    }
}
impl IVZEFIVariableStore for VZEFIVariableStore {}
pub trait IVZEFIVariableStore: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn initWithURL_(&self, URL: NSURL) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithURL : URL)
    }
    unsafe fn initCreatingVariableStoreAtURL_options_error_(
        &self,
        URL: NSURL,
        options: VZEFIVariableStoreInitializationOptions,
        error: *mut NSError,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initCreatingVariableStoreAtURL : URL, options : options, error : error)
    }
    unsafe fn URL(&self) -> NSURL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, URL)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"VZEFIVariableStore").unwrap(), new)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct VZEFIBootLoader(pub id);
impl std::ops::Deref for VZEFIBootLoader {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for VZEFIBootLoader {}
impl VZEFIBootLoader {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"VZEFIBootLoader").unwrap(), alloc) })
    }
}
impl IVZBootLoader for VZEFIBootLoader {}
impl PNSCopying for VZEFIBootLoader {}
impl From<VZEFIBootLoader> for VZBootLoader {
    fn from(child: VZEFIBootLoader) -> VZBootLoader {
        VZBootLoader(child.0)
    }
}
impl std::convert::TryFrom<VZBootLoader> for VZEFIBootLoader {
    type Error = &'static str;
    fn try_from(parent: VZBootLoader) -> Result<VZEFIBootLoader, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"VZEFIBootLoader").unwrap()) };
        if is_kind_of {
            Ok(VZEFIBootLoader(parent.0))
        } else {
            Err("This VZBootLoader cannot be downcasted to VZEFIBootLoader")
        }
    }
}
impl INSObject for VZEFIBootLoader {}
impl PNSObject for VZEFIBootLoader {}
impl IVZEFIBootLoader for VZEFIBootLoader {}
pub trait IVZEFIBootLoader: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn variableStore(&self) -> VZEFIVariableStore
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, variableStore)
    }
    unsafe fn setVariableStore_(&self, variableStore: VZEFIVariableStore)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setVariableStore : variableStore)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct VZEntropyDeviceConfiguration(pub id);
impl std::ops::Deref for VZEntropyDeviceConfiguration {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for VZEntropyDeviceConfiguration {}
impl VZEntropyDeviceConfiguration {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"VZEntropyDeviceConfiguration").unwrap(), alloc) })
    }
}
impl PNSCopying for VZEntropyDeviceConfiguration {}
impl INSObject for VZEntropyDeviceConfiguration {}
impl PNSObject for VZEntropyDeviceConfiguration {}
impl std::convert::TryFrom<NSObject> for VZEntropyDeviceConfiguration {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<VZEntropyDeviceConfiguration, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"VZEntropyDeviceConfiguration").unwrap()) };
        if is_kind_of {
            Ok(VZEntropyDeviceConfiguration(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to VZEntropyDeviceConfiguration")
        }
    }
}
impl IVZEntropyDeviceConfiguration for VZEntropyDeviceConfiguration {}
pub trait IVZEntropyDeviceConfiguration: Sized + std::ops::Deref {
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
        msg_send!(&*objc2::runtime::AnyClass::get(c"VZEntropyDeviceConfiguration").unwrap(), new)
    }
}
pub type VZErrorCode = NSInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct VZFileHandleNetworkDeviceAttachment(pub id);
impl std::ops::Deref for VZFileHandleNetworkDeviceAttachment {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for VZFileHandleNetworkDeviceAttachment {}
impl VZFileHandleNetworkDeviceAttachment {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"VZFileHandleNetworkDeviceAttachment").unwrap(), alloc) })
    }
}
impl IVZNetworkDeviceAttachment for VZFileHandleNetworkDeviceAttachment {}
impl std::convert::TryFrom<VZNetworkDeviceAttachment> for VZFileHandleNetworkDeviceAttachment {
    type Error = &'static str;
    fn try_from(
        parent: VZNetworkDeviceAttachment,
    ) -> Result<VZFileHandleNetworkDeviceAttachment, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"VZFileHandleNetworkDeviceAttachment").unwrap())
        };
        if is_kind_of {
            Ok(VZFileHandleNetworkDeviceAttachment(parent.0))
        } else {
            Err ("This VZNetworkDeviceAttachment cannot be downcasted to VZFileHandleNetworkDeviceAttachment" ,)
        }
    }
}
impl INSObject for VZFileHandleNetworkDeviceAttachment {}
impl PNSObject for VZFileHandleNetworkDeviceAttachment {}
impl IVZFileHandleNetworkDeviceAttachment for VZFileHandleNetworkDeviceAttachment {}
pub trait IVZFileHandleNetworkDeviceAttachment: Sized + std::ops::Deref {
    unsafe fn initWithFileHandle_(&self, fileHandle: NSFileHandle) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithFileHandle : fileHandle)
    }
    unsafe fn fileHandle(&self) -> NSFileHandle
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, fileHandle)
    }
    unsafe fn maximumTransmissionUnit(&self) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, maximumTransmissionUnit)
    }
    unsafe fn setMaximumTransmissionUnit_(&self, maximumTransmissionUnit: NSInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMaximumTransmissionUnit : maximumTransmissionUnit)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct VZSerialPortAttachment(pub id);
impl std::ops::Deref for VZSerialPortAttachment {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for VZSerialPortAttachment {}
impl VZSerialPortAttachment {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"VZSerialPortAttachment").unwrap(), alloc) })
    }
}
impl INSObject for VZSerialPortAttachment {}
impl PNSObject for VZSerialPortAttachment {}
impl std::convert::TryFrom<NSObject> for VZSerialPortAttachment {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<VZSerialPortAttachment, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"VZSerialPortAttachment").unwrap()) };
        if is_kind_of {
            Ok(VZSerialPortAttachment(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to VZSerialPortAttachment")
        }
    }
}
impl IVZSerialPortAttachment for VZSerialPortAttachment {}
pub trait IVZSerialPortAttachment: Sized + std::ops::Deref {
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
        msg_send!(&*objc2::runtime::AnyClass::get(c"VZSerialPortAttachment").unwrap(), new)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct VZFileHandleSerialPortAttachment(pub id);
impl std::ops::Deref for VZFileHandleSerialPortAttachment {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for VZFileHandleSerialPortAttachment {}
impl VZFileHandleSerialPortAttachment {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"VZFileHandleSerialPortAttachment").unwrap(), alloc) })
    }
}
impl IVZSerialPortAttachment for VZFileHandleSerialPortAttachment {}
impl From<VZFileHandleSerialPortAttachment> for VZSerialPortAttachment {
    fn from(child: VZFileHandleSerialPortAttachment) -> VZSerialPortAttachment {
        VZSerialPortAttachment(child.0)
    }
}
impl std::convert::TryFrom<VZSerialPortAttachment> for VZFileHandleSerialPortAttachment {
    type Error = &'static str;
    fn try_from(
        parent: VZSerialPortAttachment,
    ) -> Result<VZFileHandleSerialPortAttachment, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"VZFileHandleSerialPortAttachment").unwrap())
        };
        if is_kind_of {
            Ok(VZFileHandleSerialPortAttachment(parent.0))
        } else {
            Err ("This VZSerialPortAttachment cannot be downcasted to VZFileHandleSerialPortAttachment" ,)
        }
    }
}
impl INSObject for VZFileHandleSerialPortAttachment {}
impl PNSObject for VZFileHandleSerialPortAttachment {}
impl IVZFileHandleSerialPortAttachment for VZFileHandleSerialPortAttachment {}
pub trait IVZFileHandleSerialPortAttachment: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn initWithFileHandleForReading_fileHandleForWriting_(
        &self,
        fileHandleForReading: NSFileHandle,
        fileHandleForWriting: NSFileHandle,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithFileHandleForReading : fileHandleForReading, fileHandleForWriting : fileHandleForWriting)
    }
    unsafe fn fileHandleForReading(&self) -> NSFileHandle
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, fileHandleForReading)
    }
    unsafe fn fileHandleForWriting(&self) -> NSFileHandle
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, fileHandleForWriting)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"VZFileHandleSerialPortAttachment").unwrap(), new)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct VZFileSerialPortAttachment(pub id);
impl std::ops::Deref for VZFileSerialPortAttachment {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for VZFileSerialPortAttachment {}
impl VZFileSerialPortAttachment {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"VZFileSerialPortAttachment").unwrap(), alloc) })
    }
}
impl IVZSerialPortAttachment for VZFileSerialPortAttachment {}
impl std::convert::TryFrom<VZSerialPortAttachment> for VZFileSerialPortAttachment {
    type Error = &'static str;
    fn try_from(parent: VZSerialPortAttachment) -> Result<VZFileSerialPortAttachment, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"VZFileSerialPortAttachment").unwrap()) };
        if is_kind_of {
            Ok(VZFileSerialPortAttachment(parent.0))
        } else {
            Err("This VZSerialPortAttachment cannot be downcasted to VZFileSerialPortAttachment")
        }
    }
}
impl INSObject for VZFileSerialPortAttachment {}
impl PNSObject for VZFileSerialPortAttachment {}
impl IVZFileSerialPortAttachment for VZFileSerialPortAttachment {}
pub trait IVZFileSerialPortAttachment: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn initWithURL_append_error_(
        &self,
        url: NSURL,
        shouldAppend: BOOL,
        error: *mut NSError,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithURL : url, append : shouldAppend, error : error)
    }
    unsafe fn URL(&self) -> NSURL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, URL)
    }
    unsafe fn append(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, append)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"VZFileSerialPortAttachment").unwrap(), new)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct VZGenericMachineIdentifier(pub id);
impl std::ops::Deref for VZGenericMachineIdentifier {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for VZGenericMachineIdentifier {}
impl VZGenericMachineIdentifier {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"VZGenericMachineIdentifier").unwrap(), alloc) })
    }
}
impl PNSCopying for VZGenericMachineIdentifier {}
impl INSObject for VZGenericMachineIdentifier {}
impl PNSObject for VZGenericMachineIdentifier {}
impl std::convert::TryFrom<NSObject> for VZGenericMachineIdentifier {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<VZGenericMachineIdentifier, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"VZGenericMachineIdentifier").unwrap()) };
        if is_kind_of {
            Ok(VZGenericMachineIdentifier(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to VZGenericMachineIdentifier")
        }
    }
}
impl IVZGenericMachineIdentifier for VZGenericMachineIdentifier {}
pub trait IVZGenericMachineIdentifier: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn initWithDataRepresentation_(&self, dataRepresentation: NSData) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithDataRepresentation : dataRepresentation)
    }
    unsafe fn dataRepresentation(&self) -> NSData
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, dataRepresentation)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct VZPlatformConfiguration(pub id);
impl std::ops::Deref for VZPlatformConfiguration {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for VZPlatformConfiguration {}
impl VZPlatformConfiguration {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"VZPlatformConfiguration").unwrap(), alloc) })
    }
}
impl PNSCopying for VZPlatformConfiguration {}
impl INSObject for VZPlatformConfiguration {}
impl PNSObject for VZPlatformConfiguration {}
impl std::convert::TryFrom<NSObject> for VZPlatformConfiguration {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<VZPlatformConfiguration, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"VZPlatformConfiguration").unwrap()) };
        if is_kind_of {
            Ok(VZPlatformConfiguration(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to VZPlatformConfiguration")
        }
    }
}
impl IVZPlatformConfiguration for VZPlatformConfiguration {}
pub trait IVZPlatformConfiguration: Sized + std::ops::Deref {
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
        msg_send!(&*objc2::runtime::AnyClass::get(c"VZPlatformConfiguration").unwrap(), new)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct VZGenericPlatformConfiguration(pub id);
impl std::ops::Deref for VZGenericPlatformConfiguration {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for VZGenericPlatformConfiguration {}
impl VZGenericPlatformConfiguration {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"VZGenericPlatformConfiguration").unwrap(), alloc) })
    }
}
impl IVZPlatformConfiguration for VZGenericPlatformConfiguration {}
impl PNSCopying for VZGenericPlatformConfiguration {}
impl From<VZGenericPlatformConfiguration> for VZPlatformConfiguration {
    fn from(child: VZGenericPlatformConfiguration) -> VZPlatformConfiguration {
        VZPlatformConfiguration(child.0)
    }
}
impl std::convert::TryFrom<VZPlatformConfiguration> for VZGenericPlatformConfiguration {
    type Error = &'static str;
    fn try_from(
        parent: VZPlatformConfiguration,
    ) -> Result<VZGenericPlatformConfiguration, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"VZGenericPlatformConfiguration").unwrap())
        };
        if is_kind_of {
            Ok(VZGenericPlatformConfiguration(parent.0))
        } else {
            Err ("This VZPlatformConfiguration cannot be downcasted to VZGenericPlatformConfiguration" ,)
        }
    }
}
impl INSObject for VZGenericPlatformConfiguration {}
impl PNSObject for VZGenericPlatformConfiguration {}
impl IVZGenericPlatformConfiguration for VZGenericPlatformConfiguration {}
pub trait IVZGenericPlatformConfiguration: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn machineIdentifier(&self) -> VZGenericMachineIdentifier
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, machineIdentifier)
    }
    unsafe fn setMachineIdentifier_(&self, machineIdentifier: VZGenericMachineIdentifier)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMachineIdentifier : machineIdentifier)
    }
    unsafe fn isNestedVirtualizationEnabled(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isNestedVirtualizationEnabled)
    }
    unsafe fn setNestedVirtualizationEnabled_(&self, nestedVirtualizationEnabled: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setNestedVirtualizationEnabled : nestedVirtualizationEnabled)
    }
    unsafe fn isNestedVirtualizationSupported() -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"VZGenericPlatformConfiguration").unwrap(), isNestedVirtualizationSupported)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct VZGraphicsDevice(pub id);
impl std::ops::Deref for VZGraphicsDevice {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for VZGraphicsDevice {}
impl VZGraphicsDevice {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"VZGraphicsDevice").unwrap(), alloc) })
    }
}
impl INSObject for VZGraphicsDevice {}
impl PNSObject for VZGraphicsDevice {}
impl std::convert::TryFrom<NSObject> for VZGraphicsDevice {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<VZGraphicsDevice, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"VZGraphicsDevice").unwrap()) };
        if is_kind_of {
            Ok(VZGraphicsDevice(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to VZGraphicsDevice")
        }
    }
}
impl IVZGraphicsDevice for VZGraphicsDevice {}
pub trait IVZGraphicsDevice: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn displays(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, displays)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"VZGraphicsDevice").unwrap(), new)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct VZGraphicsDeviceConfiguration(pub id);
impl std::ops::Deref for VZGraphicsDeviceConfiguration {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for VZGraphicsDeviceConfiguration {}
impl VZGraphicsDeviceConfiguration {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"VZGraphicsDeviceConfiguration").unwrap(), alloc) })
    }
}
impl PNSCopying for VZGraphicsDeviceConfiguration {}
impl INSObject for VZGraphicsDeviceConfiguration {}
impl PNSObject for VZGraphicsDeviceConfiguration {}
impl std::convert::TryFrom<NSObject> for VZGraphicsDeviceConfiguration {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<VZGraphicsDeviceConfiguration, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"VZGraphicsDeviceConfiguration").unwrap())
        };
        if is_kind_of {
            Ok(VZGraphicsDeviceConfiguration(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to VZGraphicsDeviceConfiguration")
        }
    }
}
impl IVZGraphicsDeviceConfiguration for VZGraphicsDeviceConfiguration {}
pub trait IVZGraphicsDeviceConfiguration: Sized + std::ops::Deref {
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
        msg_send!(&*objc2::runtime::AnyClass::get(c"VZGraphicsDeviceConfiguration").unwrap(), new)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct VZGraphicsDisplay(pub id);
impl std::ops::Deref for VZGraphicsDisplay {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for VZGraphicsDisplay {}
impl VZGraphicsDisplay {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"VZGraphicsDisplay").unwrap(), alloc) })
    }
}
impl INSObject for VZGraphicsDisplay {}
impl PNSObject for VZGraphicsDisplay {}
impl std::convert::TryFrom<NSObject> for VZGraphicsDisplay {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<VZGraphicsDisplay, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"VZGraphicsDisplay").unwrap()) };
        if is_kind_of {
            Ok(VZGraphicsDisplay(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to VZGraphicsDisplay")
        }
    }
}
impl IVZGraphicsDisplay for VZGraphicsDisplay {}
pub trait IVZGraphicsDisplay: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn reconfigureWithSizeInPixels_error_(
        &self,
        sizeInPixels: CGSize,
        error: *mut NSError,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, reconfigureWithSizeInPixels : sizeInPixels, error : error)
    }
    unsafe fn reconfigureWithConfiguration_error_(
        &self,
        configuration: VZGraphicsDisplayConfiguration,
        error: *mut NSError,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, reconfigureWithConfiguration : configuration, error : error)
    }
    unsafe fn addObserver_(&self, observer: *mut u64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, addObserver : observer)
    }
    unsafe fn removeObserver_(&self, observer: *mut u64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, removeObserver : observer)
    }
    unsafe fn sizeInPixels(&self) -> CGSize
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, sizeInPixels)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"VZGraphicsDisplay").unwrap(), new)
    }
}
pub trait PVZGraphicsDisplayObserver: Sized + std::ops::Deref {
    unsafe fn displayDidBeginReconfiguration_(&self, display: VZGraphicsDisplay)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, displayDidBeginReconfiguration : display)
    }
    unsafe fn displayDidEndReconfiguration_(&self, display: VZGraphicsDisplay)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, displayDidEndReconfiguration : display)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct VZGraphicsDisplayConfiguration(pub id);
impl std::ops::Deref for VZGraphicsDisplayConfiguration {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for VZGraphicsDisplayConfiguration {}
impl VZGraphicsDisplayConfiguration {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"VZGraphicsDisplayConfiguration").unwrap(), alloc) })
    }
}
impl PNSCopying for VZGraphicsDisplayConfiguration {}
impl INSObject for VZGraphicsDisplayConfiguration {}
impl PNSObject for VZGraphicsDisplayConfiguration {}
impl std::convert::TryFrom<NSObject> for VZGraphicsDisplayConfiguration {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<VZGraphicsDisplayConfiguration, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"VZGraphicsDisplayConfiguration").unwrap())
        };
        if is_kind_of {
            Ok(VZGraphicsDisplayConfiguration(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to VZGraphicsDisplayConfiguration")
        }
    }
}
impl IVZGraphicsDisplayConfiguration for VZGraphicsDisplayConfiguration {}
pub trait IVZGraphicsDisplayConfiguration: Sized + std::ops::Deref {
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
        msg_send!(&*objc2::runtime::AnyClass::get(c"VZGraphicsDisplayConfiguration").unwrap(), new)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct VZHostAudioInputStreamSource(pub id);
impl std::ops::Deref for VZHostAudioInputStreamSource {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for VZHostAudioInputStreamSource {}
impl VZHostAudioInputStreamSource {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"VZHostAudioInputStreamSource").unwrap(), alloc) })
    }
}
impl IVZAudioInputStreamSource for VZHostAudioInputStreamSource {}
impl From<VZHostAudioInputStreamSource> for VZAudioInputStreamSource {
    fn from(child: VZHostAudioInputStreamSource) -> VZAudioInputStreamSource {
        VZAudioInputStreamSource(child.0)
    }
}
impl std::convert::TryFrom<VZAudioInputStreamSource> for VZHostAudioInputStreamSource {
    type Error = &'static str;
    fn try_from(
        parent: VZAudioInputStreamSource,
    ) -> Result<VZHostAudioInputStreamSource, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"VZHostAudioInputStreamSource").unwrap()) };
        if is_kind_of {
            Ok(VZHostAudioInputStreamSource(parent.0))
        } else {
            Err ("This VZAudioInputStreamSource cannot be downcasted to VZHostAudioInputStreamSource" ,)
        }
    }
}
impl INSObject for VZHostAudioInputStreamSource {}
impl PNSObject for VZHostAudioInputStreamSource {}
impl IVZHostAudioInputStreamSource for VZHostAudioInputStreamSource {}
pub trait IVZHostAudioInputStreamSource: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct VZHostAudioOutputStreamSink(pub id);
impl std::ops::Deref for VZHostAudioOutputStreamSink {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for VZHostAudioOutputStreamSink {}
impl VZHostAudioOutputStreamSink {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"VZHostAudioOutputStreamSink").unwrap(), alloc) })
    }
}
impl IVZAudioOutputStreamSink for VZHostAudioOutputStreamSink {}
impl From<VZHostAudioOutputStreamSink> for VZAudioOutputStreamSink {
    fn from(child: VZHostAudioOutputStreamSink) -> VZAudioOutputStreamSink {
        VZAudioOutputStreamSink(child.0)
    }
}
impl std::convert::TryFrom<VZAudioOutputStreamSink> for VZHostAudioOutputStreamSink {
    type Error = &'static str;
    fn try_from(
        parent: VZAudioOutputStreamSink,
    ) -> Result<VZHostAudioOutputStreamSink, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"VZHostAudioOutputStreamSink").unwrap()) };
        if is_kind_of {
            Ok(VZHostAudioOutputStreamSink(parent.0))
        } else {
            Err("This VZAudioOutputStreamSink cannot be downcasted to VZHostAudioOutputStreamSink")
        }
    }
}
impl INSObject for VZHostAudioOutputStreamSink {}
impl PNSObject for VZHostAudioOutputStreamSink {}
impl IVZHostAudioOutputStreamSink for VZHostAudioOutputStreamSink {}
pub trait IVZHostAudioOutputStreamSink: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct VZKeyboardConfiguration(pub id);
impl std::ops::Deref for VZKeyboardConfiguration {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for VZKeyboardConfiguration {}
impl VZKeyboardConfiguration {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"VZKeyboardConfiguration").unwrap(), alloc) })
    }
}
impl PNSCopying for VZKeyboardConfiguration {}
impl INSObject for VZKeyboardConfiguration {}
impl PNSObject for VZKeyboardConfiguration {}
impl std::convert::TryFrom<NSObject> for VZKeyboardConfiguration {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<VZKeyboardConfiguration, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"VZKeyboardConfiguration").unwrap()) };
        if is_kind_of {
            Ok(VZKeyboardConfiguration(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to VZKeyboardConfiguration")
        }
    }
}
impl IVZKeyboardConfiguration for VZKeyboardConfiguration {}
pub trait IVZKeyboardConfiguration: Sized + std::ops::Deref {
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
        msg_send!(&*objc2::runtime::AnyClass::get(c"VZKeyboardConfiguration").unwrap(), new)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct VZLinuxBootLoader(pub id);
impl std::ops::Deref for VZLinuxBootLoader {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for VZLinuxBootLoader {}
impl VZLinuxBootLoader {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"VZLinuxBootLoader").unwrap(), alloc) })
    }
}
impl IVZBootLoader for VZLinuxBootLoader {}
impl PNSCopying for VZLinuxBootLoader {}
impl std::convert::TryFrom<VZBootLoader> for VZLinuxBootLoader {
    type Error = &'static str;
    fn try_from(parent: VZBootLoader) -> Result<VZLinuxBootLoader, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"VZLinuxBootLoader").unwrap()) };
        if is_kind_of {
            Ok(VZLinuxBootLoader(parent.0))
        } else {
            Err("This VZBootLoader cannot be downcasted to VZLinuxBootLoader")
        }
    }
}
impl INSObject for VZLinuxBootLoader {}
impl PNSObject for VZLinuxBootLoader {}
impl IVZLinuxBootLoader for VZLinuxBootLoader {}
pub trait IVZLinuxBootLoader: Sized + std::ops::Deref {
    unsafe fn initWithKernelURL_(&self, kernelURL: NSURL) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithKernelURL : kernelURL)
    }
    unsafe fn kernelURL(&self) -> NSURL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, kernelURL)
    }
    unsafe fn setKernelURL_(&self, kernelURL: NSURL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setKernelURL : kernelURL)
    }
    unsafe fn commandLine(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, commandLine)
    }
    unsafe fn setCommandLine_(&self, commandLine: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCommandLine : commandLine)
    }
    unsafe fn initialRamdiskURL(&self) -> NSURL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, initialRamdiskURL)
    }
    unsafe fn setInitialRamdiskURL_(&self, initialRamdiskURL: NSURL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInitialRamdiskURL : initialRamdiskURL)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct VZLinuxRosettaCachingOptions(pub id);
impl std::ops::Deref for VZLinuxRosettaCachingOptions {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for VZLinuxRosettaCachingOptions {}
impl VZLinuxRosettaCachingOptions {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"VZLinuxRosettaCachingOptions").unwrap(), alloc) })
    }
}
impl INSObject for VZLinuxRosettaCachingOptions {}
impl PNSObject for VZLinuxRosettaCachingOptions {}
impl std::convert::TryFrom<NSObject> for VZLinuxRosettaCachingOptions {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<VZLinuxRosettaCachingOptions, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"VZLinuxRosettaCachingOptions").unwrap()) };
        if is_kind_of {
            Ok(VZLinuxRosettaCachingOptions(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to VZLinuxRosettaCachingOptions")
        }
    }
}
impl IVZLinuxRosettaCachingOptions for VZLinuxRosettaCachingOptions {}
pub trait IVZLinuxRosettaCachingOptions: Sized + std::ops::Deref {
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
        msg_send!(&*objc2::runtime::AnyClass::get(c"VZLinuxRosettaCachingOptions").unwrap(), new)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct VZLinuxRosettaAbstractSocketCachingOptions(pub id);
impl std::ops::Deref for VZLinuxRosettaAbstractSocketCachingOptions {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for VZLinuxRosettaAbstractSocketCachingOptions {}
impl VZLinuxRosettaAbstractSocketCachingOptions {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"VZLinuxRosettaAbstractSocketCachingOptions").unwrap(), alloc) })
    }
}
impl IVZLinuxRosettaCachingOptions for VZLinuxRosettaAbstractSocketCachingOptions {}
impl From<VZLinuxRosettaAbstractSocketCachingOptions> for VZLinuxRosettaCachingOptions {
    fn from(child: VZLinuxRosettaAbstractSocketCachingOptions) -> VZLinuxRosettaCachingOptions {
        VZLinuxRosettaCachingOptions(child.0)
    }
}
impl std::convert::TryFrom<VZLinuxRosettaCachingOptions>
    for VZLinuxRosettaAbstractSocketCachingOptions
{
    type Error = &'static str;
    fn try_from(
        parent: VZLinuxRosettaCachingOptions,
    ) -> Result<VZLinuxRosettaAbstractSocketCachingOptions, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"VZLinuxRosettaAbstractSocketCachingOptions").unwrap())
        };
        if is_kind_of {
            Ok(VZLinuxRosettaAbstractSocketCachingOptions(parent.0))
        } else {
            Err ("This VZLinuxRosettaCachingOptions cannot be downcasted to VZLinuxRosettaAbstractSocketCachingOptions" ,)
        }
    }
}
impl INSObject for VZLinuxRosettaAbstractSocketCachingOptions {}
impl PNSObject for VZLinuxRosettaAbstractSocketCachingOptions {}
impl IVZLinuxRosettaAbstractSocketCachingOptions for VZLinuxRosettaAbstractSocketCachingOptions {}
pub trait IVZLinuxRosettaAbstractSocketCachingOptions: Sized + std::ops::Deref {
    unsafe fn initWithName_error_(&self, name: NSString, error: *mut NSError) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithName : name, error : error)
    }
    unsafe fn name(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, name)
    }
    unsafe fn maximumNameLength() -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"VZLinuxRosettaAbstractSocketCachingOptions").unwrap(), maximumNameLength)
    }
}
pub type VZLinuxRosettaAvailability = NSInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct VZLinuxRosettaDirectoryShare(pub id);
impl std::ops::Deref for VZLinuxRosettaDirectoryShare {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for VZLinuxRosettaDirectoryShare {}
impl VZLinuxRosettaDirectoryShare {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"VZLinuxRosettaDirectoryShare").unwrap(), alloc) })
    }
}
impl IVZDirectoryShare for VZLinuxRosettaDirectoryShare {}
impl From<VZLinuxRosettaDirectoryShare> for VZDirectoryShare {
    fn from(child: VZLinuxRosettaDirectoryShare) -> VZDirectoryShare {
        VZDirectoryShare(child.0)
    }
}
impl std::convert::TryFrom<VZDirectoryShare> for VZLinuxRosettaDirectoryShare {
    type Error = &'static str;
    fn try_from(parent: VZDirectoryShare) -> Result<VZLinuxRosettaDirectoryShare, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"VZLinuxRosettaDirectoryShare").unwrap()) };
        if is_kind_of {
            Ok(VZLinuxRosettaDirectoryShare(parent.0))
        } else {
            Err("This VZDirectoryShare cannot be downcasted to VZLinuxRosettaDirectoryShare")
        }
    }
}
impl INSObject for VZLinuxRosettaDirectoryShare {}
impl PNSObject for VZLinuxRosettaDirectoryShare {}
impl IVZLinuxRosettaDirectoryShare for VZLinuxRosettaDirectoryShare {}
pub trait IVZLinuxRosettaDirectoryShare: Sized + std::ops::Deref {
    unsafe fn initWithError_(&self, error: *mut NSError) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithError : error)
    }
    unsafe fn options(&self) -> VZLinuxRosettaCachingOptions
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, options)
    }
    unsafe fn setOptions_(&self, options: VZLinuxRosettaCachingOptions)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setOptions : options)
    }
    unsafe fn installRosettaWithCompletionHandler_(completionHandler: *mut ::std::os::raw::c_void)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"VZLinuxRosettaDirectoryShare").unwrap(), installRosettaWithCompletionHandler : completionHandler)
    }
    unsafe fn availability() -> VZLinuxRosettaAvailability
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"VZLinuxRosettaDirectoryShare").unwrap(), availability)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct VZLinuxRosettaUnixSocketCachingOptions(pub id);
impl std::ops::Deref for VZLinuxRosettaUnixSocketCachingOptions {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for VZLinuxRosettaUnixSocketCachingOptions {}
impl VZLinuxRosettaUnixSocketCachingOptions {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"VZLinuxRosettaUnixSocketCachingOptions").unwrap(), alloc) })
    }
}
impl IVZLinuxRosettaCachingOptions for VZLinuxRosettaUnixSocketCachingOptions {}
impl std::convert::TryFrom<VZLinuxRosettaCachingOptions>
    for VZLinuxRosettaUnixSocketCachingOptions
{
    type Error = &'static str;
    fn try_from(
        parent: VZLinuxRosettaCachingOptions,
    ) -> Result<VZLinuxRosettaUnixSocketCachingOptions, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"VZLinuxRosettaUnixSocketCachingOptions").unwrap())
        };
        if is_kind_of {
            Ok(VZLinuxRosettaUnixSocketCachingOptions(parent.0))
        } else {
            Err ("This VZLinuxRosettaCachingOptions cannot be downcasted to VZLinuxRosettaUnixSocketCachingOptions" ,)
        }
    }
}
impl INSObject for VZLinuxRosettaUnixSocketCachingOptions {}
impl PNSObject for VZLinuxRosettaUnixSocketCachingOptions {}
impl IVZLinuxRosettaUnixSocketCachingOptions for VZLinuxRosettaUnixSocketCachingOptions {}
pub trait IVZLinuxRosettaUnixSocketCachingOptions: Sized + std::ops::Deref {
    unsafe fn initWithPath_error_(&self, path: NSString, error: *mut NSError) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithPath : path, error : error)
    }
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn path(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, path)
    }
    unsafe fn maximumPathLength() -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"VZLinuxRosettaUnixSocketCachingOptions").unwrap(), maximumPathLength)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct VZMACAddress(pub id);
impl std::ops::Deref for VZMACAddress {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for VZMACAddress {}
impl VZMACAddress {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"VZMACAddress").unwrap(), alloc) })
    }
}
impl PNSCopying for VZMACAddress {}
impl INSObject for VZMACAddress {}
impl PNSObject for VZMACAddress {}
impl std::convert::TryFrom<NSObject> for VZMACAddress {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<VZMACAddress, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"VZMACAddress").unwrap()) };
        if is_kind_of {
            Ok(VZMACAddress(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to VZMACAddress")
        }
    }
}
impl IVZMACAddress for VZMACAddress {}
pub trait IVZMACAddress: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn initWithEthernetAddress_(&self, ethernetAddress: ether_addr_t) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithEthernetAddress : ethernetAddress)
    }
    unsafe fn initWithString_(&self, string: NSString) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithString : string)
    }
    unsafe fn ethernetAddress(&self) -> ether_addr_t
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, ethernetAddress)
    }
    unsafe fn string(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, string)
    }
    unsafe fn isBroadcastAddress(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isBroadcastAddress)
    }
    unsafe fn isMulticastAddress(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isMulticastAddress)
    }
    unsafe fn isUnicastAddress(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isUnicastAddress)
    }
    unsafe fn isLocallyAdministeredAddress(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isLocallyAdministeredAddress)
    }
    unsafe fn isUniversallyAdministeredAddress(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isUniversallyAdministeredAddress)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"VZMACAddress").unwrap(), new)
    }
    unsafe fn randomLocallyAdministeredAddress() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"VZMACAddress").unwrap(), randomLocallyAdministeredAddress)
    }
}
pub type VZMacAuxiliaryStorageInitializationOptions = NSUInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct VZMacAuxiliaryStorage(pub id);
impl std::ops::Deref for VZMacAuxiliaryStorage {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for VZMacAuxiliaryStorage {}
impl VZMacAuxiliaryStorage {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"VZMacAuxiliaryStorage").unwrap(), alloc) })
    }
}
impl INSObject for VZMacAuxiliaryStorage {}
impl PNSObject for VZMacAuxiliaryStorage {}
impl std::convert::TryFrom<NSObject> for VZMacAuxiliaryStorage {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<VZMacAuxiliaryStorage, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"VZMacAuxiliaryStorage").unwrap()) };
        if is_kind_of {
            Ok(VZMacAuxiliaryStorage(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to VZMacAuxiliaryStorage")
        }
    }
}
impl IVZMacAuxiliaryStorage for VZMacAuxiliaryStorage {}
pub trait IVZMacAuxiliaryStorage: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn initWithURL_(&self, URL: NSURL) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithURL : URL)
    }
    unsafe fn initCreatingStorageAtURL_hardwareModel_options_error_(
        &self,
        URL: NSURL,
        hardwareModel: VZMacHardwareModel,
        options: VZMacAuxiliaryStorageInitializationOptions,
        error: *mut NSError,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initCreatingStorageAtURL : URL, hardwareModel : hardwareModel, options : options, error : error)
    }
    unsafe fn URL(&self) -> NSURL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, URL)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"VZMacAuxiliaryStorage").unwrap(), new)
    }
}
impl VZMacAuxiliaryStorage_VZDeprecated for VZMacAuxiliaryStorage {}
pub trait VZMacAuxiliaryStorage_VZDeprecated: Sized + std::ops::Deref {
    unsafe fn initWithContentsOfURL_(&self, URL: NSURL) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithContentsOfURL : URL)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct VZMacGraphicsDevice(pub id);
impl std::ops::Deref for VZMacGraphicsDevice {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for VZMacGraphicsDevice {}
impl VZMacGraphicsDevice {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"VZMacGraphicsDevice").unwrap(), alloc) })
    }
}
impl IVZGraphicsDevice for VZMacGraphicsDevice {}
impl From<VZMacGraphicsDevice> for VZGraphicsDevice {
    fn from(child: VZMacGraphicsDevice) -> VZGraphicsDevice {
        VZGraphicsDevice(child.0)
    }
}
impl std::convert::TryFrom<VZGraphicsDevice> for VZMacGraphicsDevice {
    type Error = &'static str;
    fn try_from(parent: VZGraphicsDevice) -> Result<VZMacGraphicsDevice, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"VZMacGraphicsDevice").unwrap()) };
        if is_kind_of {
            Ok(VZMacGraphicsDevice(parent.0))
        } else {
            Err("This VZGraphicsDevice cannot be downcasted to VZMacGraphicsDevice")
        }
    }
}
impl INSObject for VZMacGraphicsDevice {}
impl PNSObject for VZMacGraphicsDevice {}
impl IVZMacGraphicsDevice for VZMacGraphicsDevice {}
pub trait IVZMacGraphicsDevice: Sized + std::ops::Deref {}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct VZMacGraphicsDeviceConfiguration(pub id);
impl std::ops::Deref for VZMacGraphicsDeviceConfiguration {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for VZMacGraphicsDeviceConfiguration {}
impl VZMacGraphicsDeviceConfiguration {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"VZMacGraphicsDeviceConfiguration").unwrap(), alloc) })
    }
}
impl IVZGraphicsDeviceConfiguration for VZMacGraphicsDeviceConfiguration {}
impl PNSCopying for VZMacGraphicsDeviceConfiguration {}
impl From<VZMacGraphicsDeviceConfiguration> for VZGraphicsDeviceConfiguration {
    fn from(child: VZMacGraphicsDeviceConfiguration) -> VZGraphicsDeviceConfiguration {
        VZGraphicsDeviceConfiguration(child.0)
    }
}
impl std::convert::TryFrom<VZGraphicsDeviceConfiguration> for VZMacGraphicsDeviceConfiguration {
    type Error = &'static str;
    fn try_from(
        parent: VZGraphicsDeviceConfiguration,
    ) -> Result<VZMacGraphicsDeviceConfiguration, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"VZMacGraphicsDeviceConfiguration").unwrap())
        };
        if is_kind_of {
            Ok(VZMacGraphicsDeviceConfiguration(parent.0))
        } else {
            Err ("This VZGraphicsDeviceConfiguration cannot be downcasted to VZMacGraphicsDeviceConfiguration" ,)
        }
    }
}
impl INSObject for VZMacGraphicsDeviceConfiguration {}
impl PNSObject for VZMacGraphicsDeviceConfiguration {}
impl IVZMacGraphicsDeviceConfiguration for VZMacGraphicsDeviceConfiguration {}
pub trait IVZMacGraphicsDeviceConfiguration: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn displays(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, displays)
    }
    unsafe fn setDisplays_(&self, displays: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDisplays : displays)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct VZMacGraphicsDisplay(pub id);
impl std::ops::Deref for VZMacGraphicsDisplay {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for VZMacGraphicsDisplay {}
impl VZMacGraphicsDisplay {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"VZMacGraphicsDisplay").unwrap(), alloc) })
    }
}
impl IVZGraphicsDisplay for VZMacGraphicsDisplay {}
impl From<VZMacGraphicsDisplay> for VZGraphicsDisplay {
    fn from(child: VZMacGraphicsDisplay) -> VZGraphicsDisplay {
        VZGraphicsDisplay(child.0)
    }
}
impl std::convert::TryFrom<VZGraphicsDisplay> for VZMacGraphicsDisplay {
    type Error = &'static str;
    fn try_from(parent: VZGraphicsDisplay) -> Result<VZMacGraphicsDisplay, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"VZMacGraphicsDisplay").unwrap()) };
        if is_kind_of {
            Ok(VZMacGraphicsDisplay(parent.0))
        } else {
            Err("This VZGraphicsDisplay cannot be downcasted to VZMacGraphicsDisplay")
        }
    }
}
impl INSObject for VZMacGraphicsDisplay {}
impl PNSObject for VZMacGraphicsDisplay {}
impl IVZMacGraphicsDisplay for VZMacGraphicsDisplay {}
pub trait IVZMacGraphicsDisplay: Sized + std::ops::Deref {
    unsafe fn pixelsPerInch(&self) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, pixelsPerInch)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct VZMacGraphicsDisplayConfiguration(pub id);
impl std::ops::Deref for VZMacGraphicsDisplayConfiguration {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for VZMacGraphicsDisplayConfiguration {}
impl VZMacGraphicsDisplayConfiguration {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"VZMacGraphicsDisplayConfiguration").unwrap(), alloc) })
    }
}
impl IVZGraphicsDisplayConfiguration for VZMacGraphicsDisplayConfiguration {}
impl PNSCopying for VZMacGraphicsDisplayConfiguration {}
impl From<VZMacGraphicsDisplayConfiguration> for VZGraphicsDisplayConfiguration {
    fn from(child: VZMacGraphicsDisplayConfiguration) -> VZGraphicsDisplayConfiguration {
        VZGraphicsDisplayConfiguration(child.0)
    }
}
impl std::convert::TryFrom<VZGraphicsDisplayConfiguration> for VZMacGraphicsDisplayConfiguration {
    type Error = &'static str;
    fn try_from(
        parent: VZGraphicsDisplayConfiguration,
    ) -> Result<VZMacGraphicsDisplayConfiguration, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"VZMacGraphicsDisplayConfiguration").unwrap())
        };
        if is_kind_of {
            Ok(VZMacGraphicsDisplayConfiguration(parent.0))
        } else {
            Err ("This VZGraphicsDisplayConfiguration cannot be downcasted to VZMacGraphicsDisplayConfiguration" ,)
        }
    }
}
impl INSObject for VZMacGraphicsDisplayConfiguration {}
impl PNSObject for VZMacGraphicsDisplayConfiguration {}
impl IVZMacGraphicsDisplayConfiguration for VZMacGraphicsDisplayConfiguration {}
pub trait IVZMacGraphicsDisplayConfiguration: Sized + std::ops::Deref {
    unsafe fn initWithWidthInPixels_heightInPixels_pixelsPerInch_(
        &self,
        widthInPixels: NSInteger,
        heightInPixels: NSInteger,
        pixelsPerInch: NSInteger,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithWidthInPixels : widthInPixels, heightInPixels : heightInPixels, pixelsPerInch : pixelsPerInch)
    }
    unsafe fn initForScreen_sizeInPoints_(
        &self,
        screen: NSScreen,
        sizeInPoints: NSSize,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initForScreen : screen, sizeInPoints : sizeInPoints)
    }
    unsafe fn widthInPixels(&self) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, widthInPixels)
    }
    unsafe fn setWidthInPixels_(&self, widthInPixels: NSInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setWidthInPixels : widthInPixels)
    }
    unsafe fn heightInPixels(&self) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, heightInPixels)
    }
    unsafe fn setHeightInPixels_(&self, heightInPixels: NSInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setHeightInPixels : heightInPixels)
    }
    unsafe fn pixelsPerInch(&self) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, pixelsPerInch)
    }
    unsafe fn setPixelsPerInch_(&self, pixelsPerInch: NSInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPixelsPerInch : pixelsPerInch)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct VZMacHardwareModel(pub id);
impl std::ops::Deref for VZMacHardwareModel {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for VZMacHardwareModel {}
impl VZMacHardwareModel {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"VZMacHardwareModel").unwrap(), alloc) })
    }
}
impl PNSCopying for VZMacHardwareModel {}
impl INSObject for VZMacHardwareModel {}
impl PNSObject for VZMacHardwareModel {}
impl std::convert::TryFrom<NSObject> for VZMacHardwareModel {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<VZMacHardwareModel, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"VZMacHardwareModel").unwrap()) };
        if is_kind_of {
            Ok(VZMacHardwareModel(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to VZMacHardwareModel")
        }
    }
}
impl IVZMacHardwareModel for VZMacHardwareModel {}
pub trait IVZMacHardwareModel: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn initWithDataRepresentation_(&self, dataRepresentation: NSData) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithDataRepresentation : dataRepresentation)
    }
    unsafe fn dataRepresentation(&self) -> NSData
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, dataRepresentation)
    }
    unsafe fn isSupported(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isSupported)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"VZMacHardwareModel").unwrap(), new)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct VZMacKeyboardConfiguration(pub id);
impl std::ops::Deref for VZMacKeyboardConfiguration {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for VZMacKeyboardConfiguration {}
impl VZMacKeyboardConfiguration {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"VZMacKeyboardConfiguration").unwrap(), alloc) })
    }
}
impl IVZKeyboardConfiguration for VZMacKeyboardConfiguration {}
impl PNSCopying for VZMacKeyboardConfiguration {}
impl From<VZMacKeyboardConfiguration> for VZKeyboardConfiguration {
    fn from(child: VZMacKeyboardConfiguration) -> VZKeyboardConfiguration {
        VZKeyboardConfiguration(child.0)
    }
}
impl std::convert::TryFrom<VZKeyboardConfiguration> for VZMacKeyboardConfiguration {
    type Error = &'static str;
    fn try_from(
        parent: VZKeyboardConfiguration,
    ) -> Result<VZMacKeyboardConfiguration, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"VZMacKeyboardConfiguration").unwrap()) };
        if is_kind_of {
            Ok(VZMacKeyboardConfiguration(parent.0))
        } else {
            Err("This VZKeyboardConfiguration cannot be downcasted to VZMacKeyboardConfiguration")
        }
    }
}
impl INSObject for VZMacKeyboardConfiguration {}
impl PNSObject for VZMacKeyboardConfiguration {}
impl IVZMacKeyboardConfiguration for VZMacKeyboardConfiguration {}
pub trait IVZMacKeyboardConfiguration: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct VZMacMachineIdentifier(pub id);
impl std::ops::Deref for VZMacMachineIdentifier {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for VZMacMachineIdentifier {}
impl VZMacMachineIdentifier {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"VZMacMachineIdentifier").unwrap(), alloc) })
    }
}
impl PNSCopying for VZMacMachineIdentifier {}
impl INSObject for VZMacMachineIdentifier {}
impl PNSObject for VZMacMachineIdentifier {}
impl std::convert::TryFrom<NSObject> for VZMacMachineIdentifier {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<VZMacMachineIdentifier, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"VZMacMachineIdentifier").unwrap()) };
        if is_kind_of {
            Ok(VZMacMachineIdentifier(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to VZMacMachineIdentifier")
        }
    }
}
impl IVZMacMachineIdentifier for VZMacMachineIdentifier {}
pub trait IVZMacMachineIdentifier: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn initWithDataRepresentation_(&self, dataRepresentation: NSData) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithDataRepresentation : dataRepresentation)
    }
    unsafe fn dataRepresentation(&self) -> NSData
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, dataRepresentation)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct VZMacOSBootLoader(pub id);
impl std::ops::Deref for VZMacOSBootLoader {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for VZMacOSBootLoader {}
impl VZMacOSBootLoader {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"VZMacOSBootLoader").unwrap(), alloc) })
    }
}
impl IVZBootLoader for VZMacOSBootLoader {}
impl PNSCopying for VZMacOSBootLoader {}
impl std::convert::TryFrom<VZBootLoader> for VZMacOSBootLoader {
    type Error = &'static str;
    fn try_from(parent: VZBootLoader) -> Result<VZMacOSBootLoader, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"VZMacOSBootLoader").unwrap()) };
        if is_kind_of {
            Ok(VZMacOSBootLoader(parent.0))
        } else {
            Err("This VZBootLoader cannot be downcasted to VZMacOSBootLoader")
        }
    }
}
impl INSObject for VZMacOSBootLoader {}
impl PNSObject for VZMacOSBootLoader {}
impl IVZMacOSBootLoader for VZMacOSBootLoader {}
pub trait IVZMacOSBootLoader: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct VZMacOSConfigurationRequirements(pub id);
impl std::ops::Deref for VZMacOSConfigurationRequirements {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for VZMacOSConfigurationRequirements {}
impl VZMacOSConfigurationRequirements {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"VZMacOSConfigurationRequirements").unwrap(), alloc) })
    }
}
impl INSObject for VZMacOSConfigurationRequirements {}
impl PNSObject for VZMacOSConfigurationRequirements {}
impl std::convert::TryFrom<NSObject> for VZMacOSConfigurationRequirements {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<VZMacOSConfigurationRequirements, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"VZMacOSConfigurationRequirements").unwrap())
        };
        if is_kind_of {
            Ok(VZMacOSConfigurationRequirements(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to VZMacOSConfigurationRequirements")
        }
    }
}
impl IVZMacOSConfigurationRequirements for VZMacOSConfigurationRequirements {}
pub trait IVZMacOSConfigurationRequirements: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn hardwareModel(&self) -> VZMacHardwareModel
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, hardwareModel)
    }
    unsafe fn minimumSupportedCPUCount(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, minimumSupportedCPUCount)
    }
    unsafe fn minimumSupportedMemorySize(&self) -> u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, minimumSupportedMemorySize)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"VZMacOSConfigurationRequirements").unwrap(), new)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct VZMacOSInstaller(pub id);
impl std::ops::Deref for VZMacOSInstaller {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for VZMacOSInstaller {}
impl VZMacOSInstaller {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"VZMacOSInstaller").unwrap(), alloc) })
    }
}
impl INSObject for VZMacOSInstaller {}
impl PNSObject for VZMacOSInstaller {}
impl std::convert::TryFrom<NSObject> for VZMacOSInstaller {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<VZMacOSInstaller, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"VZMacOSInstaller").unwrap()) };
        if is_kind_of {
            Ok(VZMacOSInstaller(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to VZMacOSInstaller")
        }
    }
}
impl IVZMacOSInstaller for VZMacOSInstaller {}
pub trait IVZMacOSInstaller: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn initWithVirtualMachine_restoreImageURL_(
        &self,
        virtualMachine: VZVirtualMachine,
        restoreImageFileURL: NSURL,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithVirtualMachine : virtualMachine, restoreImageURL : restoreImageFileURL)
    }
    unsafe fn installWithCompletionHandler_(&self, completionHandler: *mut ::std::os::raw::c_void)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, installWithCompletionHandler : completionHandler)
    }
    unsafe fn progress(&self) -> NSProgress
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, progress)
    }
    unsafe fn virtualMachine(&self) -> VZVirtualMachine
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, virtualMachine)
    }
    unsafe fn restoreImageURL(&self) -> NSURL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, restoreImageURL)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"VZMacOSInstaller").unwrap(), new)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct VZMacOSRestoreImage(pub id);
impl std::ops::Deref for VZMacOSRestoreImage {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for VZMacOSRestoreImage {}
impl VZMacOSRestoreImage {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"VZMacOSRestoreImage").unwrap(), alloc) })
    }
}
impl INSObject for VZMacOSRestoreImage {}
impl PNSObject for VZMacOSRestoreImage {}
impl std::convert::TryFrom<NSObject> for VZMacOSRestoreImage {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<VZMacOSRestoreImage, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"VZMacOSRestoreImage").unwrap()) };
        if is_kind_of {
            Ok(VZMacOSRestoreImage(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to VZMacOSRestoreImage")
        }
    }
}
impl IVZMacOSRestoreImage for VZMacOSRestoreImage {}
pub trait IVZMacOSRestoreImage: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn isSupported(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isSupported)
    }
    unsafe fn URL(&self) -> NSURL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, URL)
    }
    unsafe fn buildVersion(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, buildVersion)
    }
    unsafe fn operatingSystemVersion(&self) -> NSOperatingSystemVersion
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, operatingSystemVersion)
    }
    unsafe fn mostFeaturefulSupportedConfiguration(&self) -> VZMacOSConfigurationRequirements
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, mostFeaturefulSupportedConfiguration)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"VZMacOSRestoreImage").unwrap(), new)
    }
    unsafe fn loadFileURL_completionHandler_(
        fileURL: NSURL,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"VZMacOSRestoreImage").unwrap(), loadFileURL : fileURL, completionHandler : completionHandler)
    }
    unsafe fn fetchLatestSupportedWithCompletionHandler_(
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"VZMacOSRestoreImage").unwrap(), fetchLatestSupportedWithCompletionHandler : completionHandler)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct VZVirtualMachineStartOptions(pub id);
impl std::ops::Deref for VZVirtualMachineStartOptions {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for VZVirtualMachineStartOptions {}
impl VZVirtualMachineStartOptions {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"VZVirtualMachineStartOptions").unwrap(), alloc) })
    }
}
impl INSObject for VZVirtualMachineStartOptions {}
impl PNSObject for VZVirtualMachineStartOptions {}
impl std::convert::TryFrom<NSObject> for VZVirtualMachineStartOptions {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<VZVirtualMachineStartOptions, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"VZVirtualMachineStartOptions").unwrap()) };
        if is_kind_of {
            Ok(VZVirtualMachineStartOptions(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to VZVirtualMachineStartOptions")
        }
    }
}
impl IVZVirtualMachineStartOptions for VZVirtualMachineStartOptions {}
pub trait IVZVirtualMachineStartOptions: Sized + std::ops::Deref {}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct VZMacOSVirtualMachineStartOptions(pub id);
impl std::ops::Deref for VZMacOSVirtualMachineStartOptions {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for VZMacOSVirtualMachineStartOptions {}
impl VZMacOSVirtualMachineStartOptions {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"VZMacOSVirtualMachineStartOptions").unwrap(), alloc) })
    }
}
impl IVZVirtualMachineStartOptions for VZMacOSVirtualMachineStartOptions {}
impl From<VZMacOSVirtualMachineStartOptions> for VZVirtualMachineStartOptions {
    fn from(child: VZMacOSVirtualMachineStartOptions) -> VZVirtualMachineStartOptions {
        VZVirtualMachineStartOptions(child.0)
    }
}
impl std::convert::TryFrom<VZVirtualMachineStartOptions> for VZMacOSVirtualMachineStartOptions {
    type Error = &'static str;
    fn try_from(
        parent: VZVirtualMachineStartOptions,
    ) -> Result<VZMacOSVirtualMachineStartOptions, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"VZMacOSVirtualMachineStartOptions").unwrap())
        };
        if is_kind_of {
            Ok(VZMacOSVirtualMachineStartOptions(parent.0))
        } else {
            Err ("This VZVirtualMachineStartOptions cannot be downcasted to VZMacOSVirtualMachineStartOptions" ,)
        }
    }
}
impl INSObject for VZMacOSVirtualMachineStartOptions {}
impl PNSObject for VZMacOSVirtualMachineStartOptions {}
impl IVZMacOSVirtualMachineStartOptions for VZMacOSVirtualMachineStartOptions {}
pub trait IVZMacOSVirtualMachineStartOptions: Sized + std::ops::Deref {
    unsafe fn startUpFromMacOSRecovery(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, startUpFromMacOSRecovery)
    }
    unsafe fn setStartUpFromMacOSRecovery_(&self, startUpFromMacOSRecovery: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setStartUpFromMacOSRecovery : startUpFromMacOSRecovery)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct VZMacPlatformConfiguration(pub id);
impl std::ops::Deref for VZMacPlatformConfiguration {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for VZMacPlatformConfiguration {}
impl VZMacPlatformConfiguration {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"VZMacPlatformConfiguration").unwrap(), alloc) })
    }
}
impl IVZPlatformConfiguration for VZMacPlatformConfiguration {}
impl PNSCopying for VZMacPlatformConfiguration {}
impl std::convert::TryFrom<VZPlatformConfiguration> for VZMacPlatformConfiguration {
    type Error = &'static str;
    fn try_from(
        parent: VZPlatformConfiguration,
    ) -> Result<VZMacPlatformConfiguration, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"VZMacPlatformConfiguration").unwrap()) };
        if is_kind_of {
            Ok(VZMacPlatformConfiguration(parent.0))
        } else {
            Err("This VZPlatformConfiguration cannot be downcasted to VZMacPlatformConfiguration")
        }
    }
}
impl INSObject for VZMacPlatformConfiguration {}
impl PNSObject for VZMacPlatformConfiguration {}
impl IVZMacPlatformConfiguration for VZMacPlatformConfiguration {}
pub trait IVZMacPlatformConfiguration: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn hardwareModel(&self) -> VZMacHardwareModel
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, hardwareModel)
    }
    unsafe fn setHardwareModel_(&self, hardwareModel: VZMacHardwareModel)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setHardwareModel : hardwareModel)
    }
    unsafe fn machineIdentifier(&self) -> VZMacMachineIdentifier
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, machineIdentifier)
    }
    unsafe fn setMachineIdentifier_(&self, machineIdentifier: VZMacMachineIdentifier)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMachineIdentifier : machineIdentifier)
    }
    unsafe fn auxiliaryStorage(&self) -> VZMacAuxiliaryStorage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, auxiliaryStorage)
    }
    unsafe fn setAuxiliaryStorage_(&self, auxiliaryStorage: VZMacAuxiliaryStorage)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAuxiliaryStorage : auxiliaryStorage)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct VZPointingDeviceConfiguration(pub id);
impl std::ops::Deref for VZPointingDeviceConfiguration {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for VZPointingDeviceConfiguration {}
impl VZPointingDeviceConfiguration {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"VZPointingDeviceConfiguration").unwrap(), alloc) })
    }
}
impl PNSCopying for VZPointingDeviceConfiguration {}
impl INSObject for VZPointingDeviceConfiguration {}
impl PNSObject for VZPointingDeviceConfiguration {}
impl std::convert::TryFrom<NSObject> for VZPointingDeviceConfiguration {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<VZPointingDeviceConfiguration, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"VZPointingDeviceConfiguration").unwrap())
        };
        if is_kind_of {
            Ok(VZPointingDeviceConfiguration(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to VZPointingDeviceConfiguration")
        }
    }
}
impl IVZPointingDeviceConfiguration for VZPointingDeviceConfiguration {}
pub trait IVZPointingDeviceConfiguration: Sized + std::ops::Deref {
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
        msg_send!(&*objc2::runtime::AnyClass::get(c"VZPointingDeviceConfiguration").unwrap(), new)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct VZMacTrackpadConfiguration(pub id);
impl std::ops::Deref for VZMacTrackpadConfiguration {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for VZMacTrackpadConfiguration {}
impl VZMacTrackpadConfiguration {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"VZMacTrackpadConfiguration").unwrap(), alloc) })
    }
}
impl IVZPointingDeviceConfiguration for VZMacTrackpadConfiguration {}
impl PNSCopying for VZMacTrackpadConfiguration {}
impl From<VZMacTrackpadConfiguration> for VZPointingDeviceConfiguration {
    fn from(child: VZMacTrackpadConfiguration) -> VZPointingDeviceConfiguration {
        VZPointingDeviceConfiguration(child.0)
    }
}
impl std::convert::TryFrom<VZPointingDeviceConfiguration> for VZMacTrackpadConfiguration {
    type Error = &'static str;
    fn try_from(
        parent: VZPointingDeviceConfiguration,
    ) -> Result<VZMacTrackpadConfiguration, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"VZMacTrackpadConfiguration").unwrap()) };
        if is_kind_of {
            Ok(VZMacTrackpadConfiguration(parent.0))
        } else {
            Err ("This VZPointingDeviceConfiguration cannot be downcasted to VZMacTrackpadConfiguration" ,)
        }
    }
}
impl INSObject for VZMacTrackpadConfiguration {}
impl PNSObject for VZMacTrackpadConfiguration {}
impl IVZMacTrackpadConfiguration for VZMacTrackpadConfiguration {}
pub trait IVZMacTrackpadConfiguration: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct VZMemoryBalloonDevice(pub id);
impl std::ops::Deref for VZMemoryBalloonDevice {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for VZMemoryBalloonDevice {}
impl VZMemoryBalloonDevice {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"VZMemoryBalloonDevice").unwrap(), alloc) })
    }
}
impl INSObject for VZMemoryBalloonDevice {}
impl PNSObject for VZMemoryBalloonDevice {}
impl std::convert::TryFrom<NSObject> for VZMemoryBalloonDevice {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<VZMemoryBalloonDevice, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"VZMemoryBalloonDevice").unwrap()) };
        if is_kind_of {
            Ok(VZMemoryBalloonDevice(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to VZMemoryBalloonDevice")
        }
    }
}
impl IVZMemoryBalloonDevice for VZMemoryBalloonDevice {}
pub trait IVZMemoryBalloonDevice: Sized + std::ops::Deref {
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
        msg_send!(&*objc2::runtime::AnyClass::get(c"VZMemoryBalloonDevice").unwrap(), new)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct VZMemoryBalloonDeviceConfiguration(pub id);
impl std::ops::Deref for VZMemoryBalloonDeviceConfiguration {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for VZMemoryBalloonDeviceConfiguration {}
impl VZMemoryBalloonDeviceConfiguration {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"VZMemoryBalloonDeviceConfiguration").unwrap(), alloc) })
    }
}
impl PNSCopying for VZMemoryBalloonDeviceConfiguration {}
impl INSObject for VZMemoryBalloonDeviceConfiguration {}
impl PNSObject for VZMemoryBalloonDeviceConfiguration {}
impl std::convert::TryFrom<NSObject> for VZMemoryBalloonDeviceConfiguration {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<VZMemoryBalloonDeviceConfiguration, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"VZMemoryBalloonDeviceConfiguration").unwrap())
        };
        if is_kind_of {
            Ok(VZMemoryBalloonDeviceConfiguration(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to VZMemoryBalloonDeviceConfiguration")
        }
    }
}
impl IVZMemoryBalloonDeviceConfiguration for VZMemoryBalloonDeviceConfiguration {}
pub trait IVZMemoryBalloonDeviceConfiguration: Sized + std::ops::Deref {
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
        msg_send!(&*objc2::runtime::AnyClass::get(c"VZMemoryBalloonDeviceConfiguration").unwrap(), new)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct VZMultipleDirectoryShare(pub id);
impl std::ops::Deref for VZMultipleDirectoryShare {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for VZMultipleDirectoryShare {}
impl VZMultipleDirectoryShare {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"VZMultipleDirectoryShare").unwrap(), alloc) })
    }
}
impl IVZDirectoryShare for VZMultipleDirectoryShare {}
impl std::convert::TryFrom<VZDirectoryShare> for VZMultipleDirectoryShare {
    type Error = &'static str;
    fn try_from(parent: VZDirectoryShare) -> Result<VZMultipleDirectoryShare, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"VZMultipleDirectoryShare").unwrap()) };
        if is_kind_of {
            Ok(VZMultipleDirectoryShare(parent.0))
        } else {
            Err("This VZDirectoryShare cannot be downcasted to VZMultipleDirectoryShare")
        }
    }
}
impl INSObject for VZMultipleDirectoryShare {}
impl PNSObject for VZMultipleDirectoryShare {}
impl IVZMultipleDirectoryShare for VZMultipleDirectoryShare {}
pub trait IVZMultipleDirectoryShare: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn initWithDirectories_(&self, directories: NSDictionary) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithDirectories : directories)
    }
    unsafe fn directories(&self) -> NSDictionary
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, directories)
    }
    unsafe fn validateName_error_(name: NSString, error: *mut NSError) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"VZMultipleDirectoryShare").unwrap(), validateName : name, error : error)
    }
    unsafe fn canonicalizedNameFromName_(name: NSString) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"VZMultipleDirectoryShare").unwrap(), canonicalizedNameFromName : name)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct VZNATNetworkDeviceAttachment(pub id);
impl std::ops::Deref for VZNATNetworkDeviceAttachment {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for VZNATNetworkDeviceAttachment {}
impl VZNATNetworkDeviceAttachment {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"VZNATNetworkDeviceAttachment").unwrap(), alloc) })
    }
}
impl IVZNetworkDeviceAttachment for VZNATNetworkDeviceAttachment {}
impl std::convert::TryFrom<VZNetworkDeviceAttachment> for VZNATNetworkDeviceAttachment {
    type Error = &'static str;
    fn try_from(
        parent: VZNetworkDeviceAttachment,
    ) -> Result<VZNATNetworkDeviceAttachment, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"VZNATNetworkDeviceAttachment").unwrap()) };
        if is_kind_of {
            Ok(VZNATNetworkDeviceAttachment(parent.0))
        } else {
            Err ("This VZNetworkDeviceAttachment cannot be downcasted to VZNATNetworkDeviceAttachment" ,)
        }
    }
}
impl INSObject for VZNATNetworkDeviceAttachment {}
impl PNSObject for VZNATNetworkDeviceAttachment {}
impl IVZNATNetworkDeviceAttachment for VZNATNetworkDeviceAttachment {}
pub trait IVZNATNetworkDeviceAttachment: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct VZStorageDeviceConfiguration(pub id);
impl std::ops::Deref for VZStorageDeviceConfiguration {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for VZStorageDeviceConfiguration {}
impl VZStorageDeviceConfiguration {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"VZStorageDeviceConfiguration").unwrap(), alloc) })
    }
}
impl PNSCopying for VZStorageDeviceConfiguration {}
impl INSObject for VZStorageDeviceConfiguration {}
impl PNSObject for VZStorageDeviceConfiguration {}
impl std::convert::TryFrom<NSObject> for VZStorageDeviceConfiguration {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<VZStorageDeviceConfiguration, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"VZStorageDeviceConfiguration").unwrap()) };
        if is_kind_of {
            Ok(VZStorageDeviceConfiguration(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to VZStorageDeviceConfiguration")
        }
    }
}
impl IVZStorageDeviceConfiguration for VZStorageDeviceConfiguration {}
pub trait IVZStorageDeviceConfiguration: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn attachment(&self) -> VZStorageDeviceAttachment
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, attachment)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"VZStorageDeviceConfiguration").unwrap(), new)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct VZNVMExpressControllerDeviceConfiguration(pub id);
impl std::ops::Deref for VZNVMExpressControllerDeviceConfiguration {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for VZNVMExpressControllerDeviceConfiguration {}
impl VZNVMExpressControllerDeviceConfiguration {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"VZNVMExpressControllerDeviceConfiguration").unwrap(), alloc) })
    }
}
impl IVZStorageDeviceConfiguration for VZNVMExpressControllerDeviceConfiguration {}
impl PNSCopying for VZNVMExpressControllerDeviceConfiguration {}
impl From<VZNVMExpressControllerDeviceConfiguration> for VZStorageDeviceConfiguration {
    fn from(child: VZNVMExpressControllerDeviceConfiguration) -> VZStorageDeviceConfiguration {
        VZStorageDeviceConfiguration(child.0)
    }
}
impl std::convert::TryFrom<VZStorageDeviceConfiguration>
    for VZNVMExpressControllerDeviceConfiguration
{
    type Error = &'static str;
    fn try_from(
        parent: VZStorageDeviceConfiguration,
    ) -> Result<VZNVMExpressControllerDeviceConfiguration, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"VZNVMExpressControllerDeviceConfiguration").unwrap())
        };
        if is_kind_of {
            Ok(VZNVMExpressControllerDeviceConfiguration(parent.0))
        } else {
            Err ("This VZStorageDeviceConfiguration cannot be downcasted to VZNVMExpressControllerDeviceConfiguration" ,)
        }
    }
}
impl INSObject for VZNVMExpressControllerDeviceConfiguration {}
impl PNSObject for VZNVMExpressControllerDeviceConfiguration {}
impl IVZNVMExpressControllerDeviceConfiguration for VZNVMExpressControllerDeviceConfiguration {}
pub trait IVZNVMExpressControllerDeviceConfiguration: Sized + std::ops::Deref {
    unsafe fn initWithAttachment_(&self, attachment: VZStorageDeviceAttachment) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithAttachment : attachment)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct VZNetworkBlockDeviceStorageDeviceAttachment(pub id);
impl std::ops::Deref for VZNetworkBlockDeviceStorageDeviceAttachment {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for VZNetworkBlockDeviceStorageDeviceAttachment {}
impl VZNetworkBlockDeviceStorageDeviceAttachment {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"VZNetworkBlockDeviceStorageDeviceAttachment").unwrap(), alloc) })
    }
}
impl IVZStorageDeviceAttachment for VZNetworkBlockDeviceStorageDeviceAttachment {}
impl std::convert::TryFrom<VZStorageDeviceAttachment>
    for VZNetworkBlockDeviceStorageDeviceAttachment
{
    type Error = &'static str;
    fn try_from(
        parent: VZStorageDeviceAttachment,
    ) -> Result<VZNetworkBlockDeviceStorageDeviceAttachment, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"VZNetworkBlockDeviceStorageDeviceAttachment").unwrap())
        };
        if is_kind_of {
            Ok(VZNetworkBlockDeviceStorageDeviceAttachment(parent.0))
        } else {
            Err ("This VZStorageDeviceAttachment cannot be downcasted to VZNetworkBlockDeviceStorageDeviceAttachment" ,)
        }
    }
}
impl INSObject for VZNetworkBlockDeviceStorageDeviceAttachment {}
impl PNSObject for VZNetworkBlockDeviceStorageDeviceAttachment {}
impl IVZNetworkBlockDeviceStorageDeviceAttachment for VZNetworkBlockDeviceStorageDeviceAttachment {}
pub trait IVZNetworkBlockDeviceStorageDeviceAttachment: Sized + std::ops::Deref {
    unsafe fn initWithURL_timeout_forcedReadOnly_synchronizationMode_error_(
        &self,
        URL: NSURL,
        timeout: NSTimeInterval,
        forcedReadOnly: BOOL,
        synchronizationMode: VZDiskSynchronizationMode,
        error: *mut NSError,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithURL : URL, timeout : timeout, forcedReadOnly : forcedReadOnly, synchronizationMode : synchronizationMode, error : error)
    }
    unsafe fn initWithURL_error_(&self, URL: NSURL, error: *mut NSError) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithURL : URL, error : error)
    }
    unsafe fn URL(&self) -> NSURL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, URL)
    }
    unsafe fn timeout(&self) -> NSTimeInterval
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, timeout)
    }
    unsafe fn isForcedReadOnly(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isForcedReadOnly)
    }
    unsafe fn synchronizationMode(&self) -> VZDiskSynchronizationMode
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, synchronizationMode)
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
    unsafe fn validateURL_error_(URL: NSURL, error: *mut NSError) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"VZNetworkBlockDeviceStorageDeviceAttachment").unwrap(), validateURL : URL, error : error)
    }
}
pub trait PVZNetworkBlockDeviceStorageDeviceAttachmentDelegate: Sized + std::ops::Deref {
    unsafe fn attachmentWasConnected_(
        &self,
        attachment: VZNetworkBlockDeviceStorageDeviceAttachment,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, attachmentWasConnected : attachment)
    }
    unsafe fn attachment_didEncounterError_(
        &self,
        attachment: VZNetworkBlockDeviceStorageDeviceAttachment,
        error: NSError,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, attachment : attachment, didEncounterError : error)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct VZNetworkDevice(pub id);
impl std::ops::Deref for VZNetworkDevice {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for VZNetworkDevice {}
impl VZNetworkDevice {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"VZNetworkDevice").unwrap(), alloc) })
    }
}
impl INSObject for VZNetworkDevice {}
impl PNSObject for VZNetworkDevice {}
impl std::convert::TryFrom<NSObject> for VZNetworkDevice {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<VZNetworkDevice, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"VZNetworkDevice").unwrap()) };
        if is_kind_of {
            Ok(VZNetworkDevice(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to VZNetworkDevice")
        }
    }
}
impl IVZNetworkDevice for VZNetworkDevice {}
pub trait IVZNetworkDevice: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn attachment(&self) -> VZNetworkDeviceAttachment
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, attachment)
    }
    unsafe fn setAttachment_(&self, attachment: VZNetworkDeviceAttachment)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAttachment : attachment)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"VZNetworkDevice").unwrap(), new)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct VZNetworkDeviceConfiguration(pub id);
impl std::ops::Deref for VZNetworkDeviceConfiguration {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for VZNetworkDeviceConfiguration {}
impl VZNetworkDeviceConfiguration {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"VZNetworkDeviceConfiguration").unwrap(), alloc) })
    }
}
impl PNSCopying for VZNetworkDeviceConfiguration {}
impl INSObject for VZNetworkDeviceConfiguration {}
impl PNSObject for VZNetworkDeviceConfiguration {}
impl std::convert::TryFrom<NSObject> for VZNetworkDeviceConfiguration {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<VZNetworkDeviceConfiguration, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"VZNetworkDeviceConfiguration").unwrap()) };
        if is_kind_of {
            Ok(VZNetworkDeviceConfiguration(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to VZNetworkDeviceConfiguration")
        }
    }
}
impl IVZNetworkDeviceConfiguration for VZNetworkDeviceConfiguration {}
pub trait IVZNetworkDeviceConfiguration: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn MACAddress(&self) -> VZMACAddress
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, MACAddress)
    }
    unsafe fn setMACAddress_(&self, MACAddress: VZMACAddress)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMACAddress : MACAddress)
    }
    unsafe fn attachment(&self) -> VZNetworkDeviceAttachment
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, attachment)
    }
    unsafe fn setAttachment_(&self, attachment: VZNetworkDeviceAttachment)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAttachment : attachment)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"VZNetworkDeviceConfiguration").unwrap(), new)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct VZSerialPortConfiguration(pub id);
impl std::ops::Deref for VZSerialPortConfiguration {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for VZSerialPortConfiguration {}
impl VZSerialPortConfiguration {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"VZSerialPortConfiguration").unwrap(), alloc) })
    }
}
impl PNSCopying for VZSerialPortConfiguration {}
impl INSObject for VZSerialPortConfiguration {}
impl PNSObject for VZSerialPortConfiguration {}
impl std::convert::TryFrom<NSObject> for VZSerialPortConfiguration {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<VZSerialPortConfiguration, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"VZSerialPortConfiguration").unwrap()) };
        if is_kind_of {
            Ok(VZSerialPortConfiguration(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to VZSerialPortConfiguration")
        }
    }
}
impl IVZSerialPortConfiguration for VZSerialPortConfiguration {}
pub trait IVZSerialPortConfiguration: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn attachment(&self) -> VZSerialPortAttachment
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, attachment)
    }
    unsafe fn setAttachment_(&self, attachment: VZSerialPortAttachment)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAttachment : attachment)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"VZSerialPortConfiguration").unwrap(), new)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct VZSharedDirectory(pub id);
impl std::ops::Deref for VZSharedDirectory {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for VZSharedDirectory {}
impl VZSharedDirectory {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"VZSharedDirectory").unwrap(), alloc) })
    }
}
impl INSObject for VZSharedDirectory {}
impl PNSObject for VZSharedDirectory {}
impl std::convert::TryFrom<NSObject> for VZSharedDirectory {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<VZSharedDirectory, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"VZSharedDirectory").unwrap()) };
        if is_kind_of {
            Ok(VZSharedDirectory(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to VZSharedDirectory")
        }
    }
}
impl IVZSharedDirectory for VZSharedDirectory {}
pub trait IVZSharedDirectory: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn initWithURL_readOnly_(&self, url: NSURL, readOnly: BOOL) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithURL : url, readOnly : readOnly)
    }
    unsafe fn URL(&self) -> NSURL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, URL)
    }
    unsafe fn isReadOnly(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isReadOnly)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"VZSharedDirectory").unwrap(), new)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct VZSingleDirectoryShare(pub id);
impl std::ops::Deref for VZSingleDirectoryShare {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for VZSingleDirectoryShare {}
impl VZSingleDirectoryShare {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"VZSingleDirectoryShare").unwrap(), alloc) })
    }
}
impl IVZDirectoryShare for VZSingleDirectoryShare {}
impl std::convert::TryFrom<VZDirectoryShare> for VZSingleDirectoryShare {
    type Error = &'static str;
    fn try_from(parent: VZDirectoryShare) -> Result<VZSingleDirectoryShare, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"VZSingleDirectoryShare").unwrap()) };
        if is_kind_of {
            Ok(VZSingleDirectoryShare(parent.0))
        } else {
            Err("This VZDirectoryShare cannot be downcasted to VZSingleDirectoryShare")
        }
    }
}
impl INSObject for VZSingleDirectoryShare {}
impl PNSObject for VZSingleDirectoryShare {}
impl IVZSingleDirectoryShare for VZSingleDirectoryShare {}
pub trait IVZSingleDirectoryShare: Sized + std::ops::Deref {
    unsafe fn initWithDirectory_(&self, directory: VZSharedDirectory) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithDirectory : directory)
    }
    unsafe fn directory(&self) -> VZSharedDirectory
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, directory)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct VZSocketDevice(pub id);
impl std::ops::Deref for VZSocketDevice {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for VZSocketDevice {}
impl VZSocketDevice {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"VZSocketDevice").unwrap(), alloc) })
    }
}
impl INSObject for VZSocketDevice {}
impl PNSObject for VZSocketDevice {}
impl std::convert::TryFrom<NSObject> for VZSocketDevice {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<VZSocketDevice, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"VZSocketDevice").unwrap()) };
        if is_kind_of {
            Ok(VZSocketDevice(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to VZSocketDevice")
        }
    }
}
impl IVZSocketDevice for VZSocketDevice {}
pub trait IVZSocketDevice: Sized + std::ops::Deref {
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
        msg_send!(&*objc2::runtime::AnyClass::get(c"VZSocketDevice").unwrap(), new)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct VZSocketDeviceConfiguration(pub id);
impl std::ops::Deref for VZSocketDeviceConfiguration {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for VZSocketDeviceConfiguration {}
impl VZSocketDeviceConfiguration {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"VZSocketDeviceConfiguration").unwrap(), alloc) })
    }
}
impl PNSCopying for VZSocketDeviceConfiguration {}
impl INSObject for VZSocketDeviceConfiguration {}
impl PNSObject for VZSocketDeviceConfiguration {}
impl std::convert::TryFrom<NSObject> for VZSocketDeviceConfiguration {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<VZSocketDeviceConfiguration, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"VZSocketDeviceConfiguration").unwrap()) };
        if is_kind_of {
            Ok(VZSocketDeviceConfiguration(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to VZSocketDeviceConfiguration")
        }
    }
}
impl IVZSocketDeviceConfiguration for VZSocketDeviceConfiguration {}
pub trait IVZSocketDeviceConfiguration: Sized + std::ops::Deref {
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
        msg_send!(&*objc2::runtime::AnyClass::get(c"VZSocketDeviceConfiguration").unwrap(), new)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct VZSpiceAgentPortAttachment(pub id);
impl std::ops::Deref for VZSpiceAgentPortAttachment {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for VZSpiceAgentPortAttachment {}
impl VZSpiceAgentPortAttachment {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"VZSpiceAgentPortAttachment").unwrap(), alloc) })
    }
}
impl IVZSerialPortAttachment for VZSpiceAgentPortAttachment {}
impl std::convert::TryFrom<VZSerialPortAttachment> for VZSpiceAgentPortAttachment {
    type Error = &'static str;
    fn try_from(parent: VZSerialPortAttachment) -> Result<VZSpiceAgentPortAttachment, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"VZSpiceAgentPortAttachment").unwrap()) };
        if is_kind_of {
            Ok(VZSpiceAgentPortAttachment(parent.0))
        } else {
            Err("This VZSerialPortAttachment cannot be downcasted to VZSpiceAgentPortAttachment")
        }
    }
}
impl INSObject for VZSpiceAgentPortAttachment {}
impl PNSObject for VZSpiceAgentPortAttachment {}
impl IVZSpiceAgentPortAttachment for VZSpiceAgentPortAttachment {}
pub trait IVZSpiceAgentPortAttachment: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn sharesClipboard(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, sharesClipboard)
    }
    unsafe fn setSharesClipboard_(&self, sharesClipboard: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSharesClipboard : sharesClipboard)
    }
    unsafe fn spiceAgentPortName() -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"VZSpiceAgentPortAttachment").unwrap(), spiceAgentPortName)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct VZStorageDevice(pub id);
impl std::ops::Deref for VZStorageDevice {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for VZStorageDevice {}
impl VZStorageDevice {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"VZStorageDevice").unwrap(), alloc) })
    }
}
impl INSObject for VZStorageDevice {}
impl PNSObject for VZStorageDevice {}
impl std::convert::TryFrom<NSObject> for VZStorageDevice {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<VZStorageDevice, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"VZStorageDevice").unwrap()) };
        if is_kind_of {
            Ok(VZStorageDevice(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to VZStorageDevice")
        }
    }
}
impl IVZStorageDevice for VZStorageDevice {}
pub trait IVZStorageDevice: Sized + std::ops::Deref {
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
        msg_send!(&*objc2::runtime::AnyClass::get(c"VZStorageDevice").unwrap(), new)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct VZUSBController(pub id);
impl std::ops::Deref for VZUSBController {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for VZUSBController {}
impl VZUSBController {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"VZUSBController").unwrap(), alloc) })
    }
}
impl INSObject for VZUSBController {}
impl PNSObject for VZUSBController {}
impl std::convert::TryFrom<NSObject> for VZUSBController {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<VZUSBController, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"VZUSBController").unwrap()) };
        if is_kind_of {
            Ok(VZUSBController(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to VZUSBController")
        }
    }
}
impl IVZUSBController for VZUSBController {}
pub trait IVZUSBController: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn attachDevice_completionHandler_(
        &self,
        device: *mut u64,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, attachDevice : device, completionHandler : completionHandler)
    }
    unsafe fn detachDevice_completionHandler_(
        &self,
        device: *mut u64,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, detachDevice : device, completionHandler : completionHandler)
    }
    unsafe fn usbDevices(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, usbDevices)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"VZUSBController").unwrap(), new)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct VZUSBControllerConfiguration(pub id);
impl std::ops::Deref for VZUSBControllerConfiguration {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for VZUSBControllerConfiguration {}
impl VZUSBControllerConfiguration {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"VZUSBControllerConfiguration").unwrap(), alloc) })
    }
}
impl PNSCopying for VZUSBControllerConfiguration {}
impl INSObject for VZUSBControllerConfiguration {}
impl PNSObject for VZUSBControllerConfiguration {}
impl std::convert::TryFrom<NSObject> for VZUSBControllerConfiguration {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<VZUSBControllerConfiguration, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"VZUSBControllerConfiguration").unwrap()) };
        if is_kind_of {
            Ok(VZUSBControllerConfiguration(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to VZUSBControllerConfiguration")
        }
    }
}
impl IVZUSBControllerConfiguration for VZUSBControllerConfiguration {}
pub trait IVZUSBControllerConfiguration: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn usbDevices(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, usbDevices)
    }
    unsafe fn setUsbDevices_(&self, usbDevices: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setUsbDevices : usbDevices)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"VZUSBControllerConfiguration").unwrap(), new)
    }
}
pub trait PVZUSBDevice: Sized + std::ops::Deref {
    unsafe fn usbController(&self) -> VZUSBController
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, usbController)
    }
    unsafe fn uuid(&self) -> NSUUID
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, uuid)
    }
}
pub trait PVZUSBDeviceConfiguration: Sized + std::ops::Deref {
    unsafe fn uuid(&self) -> NSUUID
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, uuid)
    }
    unsafe fn setUuid_(&self, uuid: NSUUID)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setUuid : uuid)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct VZUSBKeyboardConfiguration(pub id);
impl std::ops::Deref for VZUSBKeyboardConfiguration {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for VZUSBKeyboardConfiguration {}
impl VZUSBKeyboardConfiguration {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"VZUSBKeyboardConfiguration").unwrap(), alloc) })
    }
}
impl IVZKeyboardConfiguration for VZUSBKeyboardConfiguration {}
impl PNSCopying for VZUSBKeyboardConfiguration {}
impl std::convert::TryFrom<VZKeyboardConfiguration> for VZUSBKeyboardConfiguration {
    type Error = &'static str;
    fn try_from(
        parent: VZKeyboardConfiguration,
    ) -> Result<VZUSBKeyboardConfiguration, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"VZUSBKeyboardConfiguration").unwrap()) };
        if is_kind_of {
            Ok(VZUSBKeyboardConfiguration(parent.0))
        } else {
            Err("This VZKeyboardConfiguration cannot be downcasted to VZUSBKeyboardConfiguration")
        }
    }
}
impl INSObject for VZUSBKeyboardConfiguration {}
impl PNSObject for VZUSBKeyboardConfiguration {}
impl IVZUSBKeyboardConfiguration for VZUSBKeyboardConfiguration {}
pub trait IVZUSBKeyboardConfiguration: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct VZUSBMassStorageDevice(pub id);
impl std::ops::Deref for VZUSBMassStorageDevice {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for VZUSBMassStorageDevice {}
impl VZUSBMassStorageDevice {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"VZUSBMassStorageDevice").unwrap(), alloc) })
    }
}
impl PVZUSBDevice for VZUSBMassStorageDevice {}
impl IVZStorageDevice for VZUSBMassStorageDevice {}
impl From<VZUSBMassStorageDevice> for VZStorageDevice {
    fn from(child: VZUSBMassStorageDevice) -> VZStorageDevice {
        VZStorageDevice(child.0)
    }
}
impl std::convert::TryFrom<VZStorageDevice> for VZUSBMassStorageDevice {
    type Error = &'static str;
    fn try_from(parent: VZStorageDevice) -> Result<VZUSBMassStorageDevice, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"VZUSBMassStorageDevice").unwrap()) };
        if is_kind_of {
            Ok(VZUSBMassStorageDevice(parent.0))
        } else {
            Err("This VZStorageDevice cannot be downcasted to VZUSBMassStorageDevice")
        }
    }
}
impl INSObject for VZUSBMassStorageDevice {}
impl PNSObject for VZUSBMassStorageDevice {}
impl IVZUSBMassStorageDevice for VZUSBMassStorageDevice {}
pub trait IVZUSBMassStorageDevice: Sized + std::ops::Deref {
    unsafe fn initWithConfiguration_(
        &self,
        configuration: VZUSBMassStorageDeviceConfiguration,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithConfiguration : configuration)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct VZUSBMassStorageDeviceConfiguration(pub id);
impl std::ops::Deref for VZUSBMassStorageDeviceConfiguration {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for VZUSBMassStorageDeviceConfiguration {}
impl VZUSBMassStorageDeviceConfiguration {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"VZUSBMassStorageDeviceConfiguration").unwrap(), alloc) })
    }
}
impl PVZUSBDeviceConfiguration for VZUSBMassStorageDeviceConfiguration {}
impl IVZStorageDeviceConfiguration for VZUSBMassStorageDeviceConfiguration {}
impl PNSCopying for VZUSBMassStorageDeviceConfiguration {}
impl std::convert::TryFrom<VZStorageDeviceConfiguration> for VZUSBMassStorageDeviceConfiguration {
    type Error = &'static str;
    fn try_from(
        parent: VZStorageDeviceConfiguration,
    ) -> Result<VZUSBMassStorageDeviceConfiguration, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"VZUSBMassStorageDeviceConfiguration").unwrap())
        };
        if is_kind_of {
            Ok(VZUSBMassStorageDeviceConfiguration(parent.0))
        } else {
            Err ("This VZStorageDeviceConfiguration cannot be downcasted to VZUSBMassStorageDeviceConfiguration" ,)
        }
    }
}
impl INSObject for VZUSBMassStorageDeviceConfiguration {}
impl PNSObject for VZUSBMassStorageDeviceConfiguration {}
impl IVZUSBMassStorageDeviceConfiguration for VZUSBMassStorageDeviceConfiguration {}
pub trait IVZUSBMassStorageDeviceConfiguration: Sized + std::ops::Deref {
    unsafe fn initWithAttachment_(&self, attachment: VZStorageDeviceAttachment) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithAttachment : attachment)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct VZUSBScreenCoordinatePointingDeviceConfiguration(pub id);
impl std::ops::Deref for VZUSBScreenCoordinatePointingDeviceConfiguration {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for VZUSBScreenCoordinatePointingDeviceConfiguration {}
impl VZUSBScreenCoordinatePointingDeviceConfiguration {
    pub fn alloc() -> Self {
        Self(unsafe {
            msg_send!(&*objc2::runtime::AnyClass::get(c"VZUSBScreenCoordinatePointingDeviceConfiguration").unwrap(), alloc)
        })
    }
}
impl IVZPointingDeviceConfiguration for VZUSBScreenCoordinatePointingDeviceConfiguration {}
impl PNSCopying for VZUSBScreenCoordinatePointingDeviceConfiguration {}
impl std::convert::TryFrom<VZPointingDeviceConfiguration>
    for VZUSBScreenCoordinatePointingDeviceConfiguration
{
    type Error = &'static str;
    fn try_from(
        parent: VZPointingDeviceConfiguration,
    ) -> Result<VZUSBScreenCoordinatePointingDeviceConfiguration, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"VZUSBScreenCoordinatePointingDeviceConfiguration").unwrap())
        };
        if is_kind_of {
            Ok(VZUSBScreenCoordinatePointingDeviceConfiguration(parent.0))
        } else {
            Err ("This VZPointingDeviceConfiguration cannot be downcasted to VZUSBScreenCoordinatePointingDeviceConfiguration" ,)
        }
    }
}
impl INSObject for VZUSBScreenCoordinatePointingDeviceConfiguration {}
impl PNSObject for VZUSBScreenCoordinatePointingDeviceConfiguration {}
impl IVZUSBScreenCoordinatePointingDeviceConfiguration
    for VZUSBScreenCoordinatePointingDeviceConfiguration
{
}
pub trait IVZUSBScreenCoordinatePointingDeviceConfiguration: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct VZVirtioBlockDeviceConfiguration(pub id);
impl std::ops::Deref for VZVirtioBlockDeviceConfiguration {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for VZVirtioBlockDeviceConfiguration {}
impl VZVirtioBlockDeviceConfiguration {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"VZVirtioBlockDeviceConfiguration").unwrap(), alloc) })
    }
}
impl IVZStorageDeviceConfiguration for VZVirtioBlockDeviceConfiguration {}
impl PNSCopying for VZVirtioBlockDeviceConfiguration {}
impl std::convert::TryFrom<VZStorageDeviceConfiguration> for VZVirtioBlockDeviceConfiguration {
    type Error = &'static str;
    fn try_from(
        parent: VZStorageDeviceConfiguration,
    ) -> Result<VZVirtioBlockDeviceConfiguration, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"VZVirtioBlockDeviceConfiguration").unwrap())
        };
        if is_kind_of {
            Ok(VZVirtioBlockDeviceConfiguration(parent.0))
        } else {
            Err ("This VZStorageDeviceConfiguration cannot be downcasted to VZVirtioBlockDeviceConfiguration" ,)
        }
    }
}
impl INSObject for VZVirtioBlockDeviceConfiguration {}
impl PNSObject for VZVirtioBlockDeviceConfiguration {}
impl IVZVirtioBlockDeviceConfiguration for VZVirtioBlockDeviceConfiguration {}
pub trait IVZVirtioBlockDeviceConfiguration: Sized + std::ops::Deref {
    unsafe fn initWithAttachment_(&self, attachment: VZStorageDeviceAttachment) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithAttachment : attachment)
    }
    unsafe fn blockDeviceIdentifier(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, blockDeviceIdentifier)
    }
    unsafe fn setBlockDeviceIdentifier_(&self, blockDeviceIdentifier: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setBlockDeviceIdentifier : blockDeviceIdentifier)
    }
    unsafe fn validateBlockDeviceIdentifier_error_(
        blockDeviceIdentifier: NSString,
        error: *mut NSError,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"VZVirtioBlockDeviceConfiguration").unwrap(), validateBlockDeviceIdentifier : blockDeviceIdentifier, error : error)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct VZVirtioConsolePort(pub id);
impl std::ops::Deref for VZVirtioConsolePort {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for VZVirtioConsolePort {}
impl VZVirtioConsolePort {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"VZVirtioConsolePort").unwrap(), alloc) })
    }
}
impl INSObject for VZVirtioConsolePort {}
impl PNSObject for VZVirtioConsolePort {}
impl std::convert::TryFrom<NSObject> for VZVirtioConsolePort {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<VZVirtioConsolePort, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"VZVirtioConsolePort").unwrap()) };
        if is_kind_of {
            Ok(VZVirtioConsolePort(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to VZVirtioConsolePort")
        }
    }
}
impl IVZVirtioConsolePort for VZVirtioConsolePort {}
pub trait IVZVirtioConsolePort: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn name(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, name)
    }
    unsafe fn attachment(&self) -> VZSerialPortAttachment
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, attachment)
    }
    unsafe fn setAttachment_(&self, attachment: VZSerialPortAttachment)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAttachment : attachment)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"VZVirtioConsolePort").unwrap(), new)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct VZVirtioConsolePortArray(pub id);
impl std::ops::Deref for VZVirtioConsolePortArray {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for VZVirtioConsolePortArray {}
impl VZVirtioConsolePortArray {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"VZVirtioConsolePortArray").unwrap(), alloc) })
    }
}
impl INSObject for VZVirtioConsolePortArray {}
impl PNSObject for VZVirtioConsolePortArray {}
impl std::convert::TryFrom<NSObject> for VZVirtioConsolePortArray {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<VZVirtioConsolePortArray, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"VZVirtioConsolePortArray").unwrap()) };
        if is_kind_of {
            Ok(VZVirtioConsolePortArray(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to VZVirtioConsolePortArray")
        }
    }
}
impl IVZVirtioConsolePortArray for VZVirtioConsolePortArray {}
pub trait IVZVirtioConsolePortArray: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn objectAtIndexedSubscript_(&self, portIndex: NSUInteger) -> VZVirtioConsolePort
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, objectAtIndexedSubscript : portIndex)
    }
    unsafe fn maximumPortCount(&self) -> u32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, maximumPortCount)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"VZVirtioConsolePortArray").unwrap(), new)
    }
}
pub trait PVZVirtioConsoleDeviceDelegate: Sized + std::ops::Deref {
    unsafe fn consoleDevice_didOpenPort_(
        &self,
        consoleDevice: VZVirtioConsoleDevice,
        consolePort: VZVirtioConsolePort,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, consoleDevice : consoleDevice, didOpenPort : consolePort)
    }
    unsafe fn consoleDevice_didClosePort_(
        &self,
        consoleDevice: VZVirtioConsoleDevice,
        consolePort: VZVirtioConsolePort,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, consoleDevice : consoleDevice, didClosePort : consolePort)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct VZVirtioConsoleDevice(pub id);
impl std::ops::Deref for VZVirtioConsoleDevice {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for VZVirtioConsoleDevice {}
impl VZVirtioConsoleDevice {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"VZVirtioConsoleDevice").unwrap(), alloc) })
    }
}
impl IVZConsoleDevice for VZVirtioConsoleDevice {}
impl From<VZVirtioConsoleDevice> for VZConsoleDevice {
    fn from(child: VZVirtioConsoleDevice) -> VZConsoleDevice {
        VZConsoleDevice(child.0)
    }
}
impl std::convert::TryFrom<VZConsoleDevice> for VZVirtioConsoleDevice {
    type Error = &'static str;
    fn try_from(parent: VZConsoleDevice) -> Result<VZVirtioConsoleDevice, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"VZVirtioConsoleDevice").unwrap()) };
        if is_kind_of {
            Ok(VZVirtioConsoleDevice(parent.0))
        } else {
            Err("This VZConsoleDevice cannot be downcasted to VZVirtioConsoleDevice")
        }
    }
}
impl INSObject for VZVirtioConsoleDevice {}
impl PNSObject for VZVirtioConsoleDevice {}
impl IVZVirtioConsoleDevice for VZVirtioConsoleDevice {}
pub trait IVZVirtioConsoleDevice: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
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
    unsafe fn ports(&self) -> VZVirtioConsolePortArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, ports)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"VZVirtioConsoleDevice").unwrap(), new)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct VZVirtioConsolePortConfiguration(pub id);
impl std::ops::Deref for VZVirtioConsolePortConfiguration {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for VZVirtioConsolePortConfiguration {}
impl VZVirtioConsolePortConfiguration {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"VZVirtioConsolePortConfiguration").unwrap(), alloc) })
    }
}
impl IVZConsolePortConfiguration for VZVirtioConsolePortConfiguration {}
impl PNSCopying for VZVirtioConsolePortConfiguration {}
impl From<VZVirtioConsolePortConfiguration> for VZConsolePortConfiguration {
    fn from(child: VZVirtioConsolePortConfiguration) -> VZConsolePortConfiguration {
        VZConsolePortConfiguration(child.0)
    }
}
impl std::convert::TryFrom<VZConsolePortConfiguration> for VZVirtioConsolePortConfiguration {
    type Error = &'static str;
    fn try_from(
        parent: VZConsolePortConfiguration,
    ) -> Result<VZVirtioConsolePortConfiguration, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"VZVirtioConsolePortConfiguration").unwrap())
        };
        if is_kind_of {
            Ok(VZVirtioConsolePortConfiguration(parent.0))
        } else {
            Err ("This VZConsolePortConfiguration cannot be downcasted to VZVirtioConsolePortConfiguration" ,)
        }
    }
}
impl INSObject for VZVirtioConsolePortConfiguration {}
impl PNSObject for VZVirtioConsolePortConfiguration {}
impl IVZVirtioConsolePortConfiguration for VZVirtioConsolePortConfiguration {}
pub trait IVZVirtioConsolePortConfiguration: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
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
    unsafe fn isConsole(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isConsole)
    }
    unsafe fn setIsConsole_(&self, isConsole: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setIsConsole : isConsole)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct VZVirtioConsolePortConfigurationArray(pub id);
impl std::ops::Deref for VZVirtioConsolePortConfigurationArray {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for VZVirtioConsolePortConfigurationArray {}
impl VZVirtioConsolePortConfigurationArray {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"VZVirtioConsolePortConfigurationArray").unwrap(), alloc) })
    }
}
impl PNSCopying for VZVirtioConsolePortConfigurationArray {}
impl INSObject for VZVirtioConsolePortConfigurationArray {}
impl PNSObject for VZVirtioConsolePortConfigurationArray {}
impl std::convert::TryFrom<NSObject> for VZVirtioConsolePortConfigurationArray {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<VZVirtioConsolePortConfigurationArray, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"VZVirtioConsolePortConfigurationArray").unwrap())
        };
        if is_kind_of {
            Ok(VZVirtioConsolePortConfigurationArray(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to VZVirtioConsolePortConfigurationArray")
        }
    }
}
impl IVZVirtioConsolePortConfigurationArray for VZVirtioConsolePortConfigurationArray {}
pub trait IVZVirtioConsolePortConfigurationArray: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn objectAtIndexedSubscript_(
        &self,
        portIndex: NSUInteger,
    ) -> VZVirtioConsolePortConfiguration
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, objectAtIndexedSubscript : portIndex)
    }
    unsafe fn setObject_atIndexedSubscript_(
        &self,
        configuration: VZVirtioConsolePortConfiguration,
        portIndex: NSUInteger,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setObject : configuration, atIndexedSubscript : portIndex)
    }
    unsafe fn maximumPortCount(&self) -> u32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, maximumPortCount)
    }
    unsafe fn setMaximumPortCount_(&self, maximumPortCount: u32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMaximumPortCount : maximumPortCount)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"VZVirtioConsolePortConfigurationArray").unwrap(), new)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct VZVirtioConsoleDeviceConfiguration(pub id);
impl std::ops::Deref for VZVirtioConsoleDeviceConfiguration {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for VZVirtioConsoleDeviceConfiguration {}
impl VZVirtioConsoleDeviceConfiguration {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"VZVirtioConsoleDeviceConfiguration").unwrap(), alloc) })
    }
}
impl IVZConsoleDeviceConfiguration for VZVirtioConsoleDeviceConfiguration {}
impl PNSCopying for VZVirtioConsoleDeviceConfiguration {}
impl From<VZVirtioConsoleDeviceConfiguration> for VZConsoleDeviceConfiguration {
    fn from(child: VZVirtioConsoleDeviceConfiguration) -> VZConsoleDeviceConfiguration {
        VZConsoleDeviceConfiguration(child.0)
    }
}
impl std::convert::TryFrom<VZConsoleDeviceConfiguration> for VZVirtioConsoleDeviceConfiguration {
    type Error = &'static str;
    fn try_from(
        parent: VZConsoleDeviceConfiguration,
    ) -> Result<VZVirtioConsoleDeviceConfiguration, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"VZVirtioConsoleDeviceConfiguration").unwrap())
        };
        if is_kind_of {
            Ok(VZVirtioConsoleDeviceConfiguration(parent.0))
        } else {
            Err ("This VZConsoleDeviceConfiguration cannot be downcasted to VZVirtioConsoleDeviceConfiguration" ,)
        }
    }
}
impl INSObject for VZVirtioConsoleDeviceConfiguration {}
impl PNSObject for VZVirtioConsoleDeviceConfiguration {}
impl IVZVirtioConsoleDeviceConfiguration for VZVirtioConsoleDeviceConfiguration {}
pub trait IVZVirtioConsoleDeviceConfiguration: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn ports(&self) -> VZVirtioConsolePortConfigurationArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, ports)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct VZVirtioConsoleDeviceSerialPortConfiguration(pub id);
impl std::ops::Deref for VZVirtioConsoleDeviceSerialPortConfiguration {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for VZVirtioConsoleDeviceSerialPortConfiguration {}
impl VZVirtioConsoleDeviceSerialPortConfiguration {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"VZVirtioConsoleDeviceSerialPortConfiguration").unwrap(), alloc) })
    }
}
impl IVZSerialPortConfiguration for VZVirtioConsoleDeviceSerialPortConfiguration {}
impl PNSCopying for VZVirtioConsoleDeviceSerialPortConfiguration {}
impl From<VZVirtioConsoleDeviceSerialPortConfiguration> for VZSerialPortConfiguration {
    fn from(child: VZVirtioConsoleDeviceSerialPortConfiguration) -> VZSerialPortConfiguration {
        VZSerialPortConfiguration(child.0)
    }
}
impl std::convert::TryFrom<VZSerialPortConfiguration>
    for VZVirtioConsoleDeviceSerialPortConfiguration
{
    type Error = &'static str;
    fn try_from(
        parent: VZSerialPortConfiguration,
    ) -> Result<VZVirtioConsoleDeviceSerialPortConfiguration, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"VZVirtioConsoleDeviceSerialPortConfiguration").unwrap())
        };
        if is_kind_of {
            Ok(VZVirtioConsoleDeviceSerialPortConfiguration(parent.0))
        } else {
            Err ("This VZSerialPortConfiguration cannot be downcasted to VZVirtioConsoleDeviceSerialPortConfiguration" ,)
        }
    }
}
impl INSObject for VZVirtioConsoleDeviceSerialPortConfiguration {}
impl PNSObject for VZVirtioConsoleDeviceSerialPortConfiguration {}
impl IVZVirtioConsoleDeviceSerialPortConfiguration
    for VZVirtioConsoleDeviceSerialPortConfiguration
{
}
pub trait IVZVirtioConsoleDeviceSerialPortConfiguration: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct VZVirtioEntropyDeviceConfiguration(pub id);
impl std::ops::Deref for VZVirtioEntropyDeviceConfiguration {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for VZVirtioEntropyDeviceConfiguration {}
impl VZVirtioEntropyDeviceConfiguration {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"VZVirtioEntropyDeviceConfiguration").unwrap(), alloc) })
    }
}
impl IVZEntropyDeviceConfiguration for VZVirtioEntropyDeviceConfiguration {}
impl PNSCopying for VZVirtioEntropyDeviceConfiguration {}
impl From<VZVirtioEntropyDeviceConfiguration> for VZEntropyDeviceConfiguration {
    fn from(child: VZVirtioEntropyDeviceConfiguration) -> VZEntropyDeviceConfiguration {
        VZEntropyDeviceConfiguration(child.0)
    }
}
impl std::convert::TryFrom<VZEntropyDeviceConfiguration> for VZVirtioEntropyDeviceConfiguration {
    type Error = &'static str;
    fn try_from(
        parent: VZEntropyDeviceConfiguration,
    ) -> Result<VZVirtioEntropyDeviceConfiguration, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"VZVirtioEntropyDeviceConfiguration").unwrap())
        };
        if is_kind_of {
            Ok(VZVirtioEntropyDeviceConfiguration(parent.0))
        } else {
            Err ("This VZEntropyDeviceConfiguration cannot be downcasted to VZVirtioEntropyDeviceConfiguration" ,)
        }
    }
}
impl INSObject for VZVirtioEntropyDeviceConfiguration {}
impl PNSObject for VZVirtioEntropyDeviceConfiguration {}
impl IVZVirtioEntropyDeviceConfiguration for VZVirtioEntropyDeviceConfiguration {}
pub trait IVZVirtioEntropyDeviceConfiguration: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct VZVirtioFileSystemDevice(pub id);
impl std::ops::Deref for VZVirtioFileSystemDevice {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for VZVirtioFileSystemDevice {}
impl VZVirtioFileSystemDevice {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"VZVirtioFileSystemDevice").unwrap(), alloc) })
    }
}
impl IVZDirectorySharingDevice for VZVirtioFileSystemDevice {}
impl From<VZVirtioFileSystemDevice> for VZDirectorySharingDevice {
    fn from(child: VZVirtioFileSystemDevice) -> VZDirectorySharingDevice {
        VZDirectorySharingDevice(child.0)
    }
}
impl std::convert::TryFrom<VZDirectorySharingDevice> for VZVirtioFileSystemDevice {
    type Error = &'static str;
    fn try_from(parent: VZDirectorySharingDevice) -> Result<VZVirtioFileSystemDevice, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"VZVirtioFileSystemDevice").unwrap()) };
        if is_kind_of {
            Ok(VZVirtioFileSystemDevice(parent.0))
        } else {
            Err("This VZDirectorySharingDevice cannot be downcasted to VZVirtioFileSystemDevice")
        }
    }
}
impl INSObject for VZVirtioFileSystemDevice {}
impl PNSObject for VZVirtioFileSystemDevice {}
impl IVZVirtioFileSystemDevice for VZVirtioFileSystemDevice {}
pub trait IVZVirtioFileSystemDevice: Sized + std::ops::Deref {
    unsafe fn tag(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, tag)
    }
    unsafe fn share(&self) -> VZDirectoryShare
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, share)
    }
    unsafe fn setShare_(&self, share: VZDirectoryShare)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setShare : share)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct VZVirtioFileSystemDeviceConfiguration(pub id);
impl std::ops::Deref for VZVirtioFileSystemDeviceConfiguration {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for VZVirtioFileSystemDeviceConfiguration {}
impl VZVirtioFileSystemDeviceConfiguration {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"VZVirtioFileSystemDeviceConfiguration").unwrap(), alloc) })
    }
}
impl IVZDirectorySharingDeviceConfiguration for VZVirtioFileSystemDeviceConfiguration {}
impl PNSCopying for VZVirtioFileSystemDeviceConfiguration {}
impl From<VZVirtioFileSystemDeviceConfiguration> for VZDirectorySharingDeviceConfiguration {
    fn from(child: VZVirtioFileSystemDeviceConfiguration) -> VZDirectorySharingDeviceConfiguration {
        VZDirectorySharingDeviceConfiguration(child.0)
    }
}
impl std::convert::TryFrom<VZDirectorySharingDeviceConfiguration>
    for VZVirtioFileSystemDeviceConfiguration
{
    type Error = &'static str;
    fn try_from(
        parent: VZDirectorySharingDeviceConfiguration,
    ) -> Result<VZVirtioFileSystemDeviceConfiguration, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"VZVirtioFileSystemDeviceConfiguration").unwrap())
        };
        if is_kind_of {
            Ok(VZVirtioFileSystemDeviceConfiguration(parent.0))
        } else {
            Err ("This VZDirectorySharingDeviceConfiguration cannot be downcasted to VZVirtioFileSystemDeviceConfiguration" ,)
        }
    }
}
impl INSObject for VZVirtioFileSystemDeviceConfiguration {}
impl PNSObject for VZVirtioFileSystemDeviceConfiguration {}
impl IVZVirtioFileSystemDeviceConfiguration for VZVirtioFileSystemDeviceConfiguration {}
pub trait IVZVirtioFileSystemDeviceConfiguration: Sized + std::ops::Deref {
    unsafe fn initWithTag_(&self, tag: NSString) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithTag : tag)
    }
    unsafe fn tag(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, tag)
    }
    unsafe fn setTag_(&self, tag: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setTag : tag)
    }
    unsafe fn share(&self) -> VZDirectoryShare
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, share)
    }
    unsafe fn setShare_(&self, share: VZDirectoryShare)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setShare : share)
    }
    unsafe fn validateTag_error_(tag: NSString, error: *mut NSError) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"VZVirtioFileSystemDeviceConfiguration").unwrap(), validateTag : tag, error : error)
    }
    unsafe fn macOSGuestAutomountTag() -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"VZVirtioFileSystemDeviceConfiguration").unwrap(), macOSGuestAutomountTag)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct VZVirtioGraphicsDevice(pub id);
impl std::ops::Deref for VZVirtioGraphicsDevice {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for VZVirtioGraphicsDevice {}
impl VZVirtioGraphicsDevice {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"VZVirtioGraphicsDevice").unwrap(), alloc) })
    }
}
impl IVZGraphicsDevice for VZVirtioGraphicsDevice {}
impl std::convert::TryFrom<VZGraphicsDevice> for VZVirtioGraphicsDevice {
    type Error = &'static str;
    fn try_from(parent: VZGraphicsDevice) -> Result<VZVirtioGraphicsDevice, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"VZVirtioGraphicsDevice").unwrap()) };
        if is_kind_of {
            Ok(VZVirtioGraphicsDevice(parent.0))
        } else {
            Err("This VZGraphicsDevice cannot be downcasted to VZVirtioGraphicsDevice")
        }
    }
}
impl INSObject for VZVirtioGraphicsDevice {}
impl PNSObject for VZVirtioGraphicsDevice {}
impl IVZVirtioGraphicsDevice for VZVirtioGraphicsDevice {}
pub trait IVZVirtioGraphicsDevice: Sized + std::ops::Deref {}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct VZVirtioGraphicsDeviceConfiguration(pub id);
impl std::ops::Deref for VZVirtioGraphicsDeviceConfiguration {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for VZVirtioGraphicsDeviceConfiguration {}
impl VZVirtioGraphicsDeviceConfiguration {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"VZVirtioGraphicsDeviceConfiguration").unwrap(), alloc) })
    }
}
impl IVZGraphicsDeviceConfiguration for VZVirtioGraphicsDeviceConfiguration {}
impl PNSCopying for VZVirtioGraphicsDeviceConfiguration {}
impl std::convert::TryFrom<VZGraphicsDeviceConfiguration> for VZVirtioGraphicsDeviceConfiguration {
    type Error = &'static str;
    fn try_from(
        parent: VZGraphicsDeviceConfiguration,
    ) -> Result<VZVirtioGraphicsDeviceConfiguration, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"VZVirtioGraphicsDeviceConfiguration").unwrap())
        };
        if is_kind_of {
            Ok(VZVirtioGraphicsDeviceConfiguration(parent.0))
        } else {
            Err ("This VZGraphicsDeviceConfiguration cannot be downcasted to VZVirtioGraphicsDeviceConfiguration" ,)
        }
    }
}
impl INSObject for VZVirtioGraphicsDeviceConfiguration {}
impl PNSObject for VZVirtioGraphicsDeviceConfiguration {}
impl IVZVirtioGraphicsDeviceConfiguration for VZVirtioGraphicsDeviceConfiguration {}
pub trait IVZVirtioGraphicsDeviceConfiguration: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn scanouts(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, scanouts)
    }
    unsafe fn setScanouts_(&self, scanouts: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setScanouts : scanouts)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct VZVirtioGraphicsScanout(pub id);
impl std::ops::Deref for VZVirtioGraphicsScanout {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for VZVirtioGraphicsScanout {}
impl VZVirtioGraphicsScanout {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"VZVirtioGraphicsScanout").unwrap(), alloc) })
    }
}
impl IVZGraphicsDisplay for VZVirtioGraphicsScanout {}
impl std::convert::TryFrom<VZGraphicsDisplay> for VZVirtioGraphicsScanout {
    type Error = &'static str;
    fn try_from(parent: VZGraphicsDisplay) -> Result<VZVirtioGraphicsScanout, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"VZVirtioGraphicsScanout").unwrap()) };
        if is_kind_of {
            Ok(VZVirtioGraphicsScanout(parent.0))
        } else {
            Err("This VZGraphicsDisplay cannot be downcasted to VZVirtioGraphicsScanout")
        }
    }
}
impl INSObject for VZVirtioGraphicsScanout {}
impl PNSObject for VZVirtioGraphicsScanout {}
impl IVZVirtioGraphicsScanout for VZVirtioGraphicsScanout {}
pub trait IVZVirtioGraphicsScanout: Sized + std::ops::Deref {}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct VZVirtioGraphicsScanoutConfiguration(pub id);
impl std::ops::Deref for VZVirtioGraphicsScanoutConfiguration {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for VZVirtioGraphicsScanoutConfiguration {}
impl VZVirtioGraphicsScanoutConfiguration {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"VZVirtioGraphicsScanoutConfiguration").unwrap(), alloc) })
    }
}
impl IVZGraphicsDisplayConfiguration for VZVirtioGraphicsScanoutConfiguration {}
impl PNSCopying for VZVirtioGraphicsScanoutConfiguration {}
impl std::convert::TryFrom<VZGraphicsDisplayConfiguration>
    for VZVirtioGraphicsScanoutConfiguration
{
    type Error = &'static str;
    fn try_from(
        parent: VZGraphicsDisplayConfiguration,
    ) -> Result<VZVirtioGraphicsScanoutConfiguration, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"VZVirtioGraphicsScanoutConfiguration").unwrap())
        };
        if is_kind_of {
            Ok(VZVirtioGraphicsScanoutConfiguration(parent.0))
        } else {
            Err ("This VZGraphicsDisplayConfiguration cannot be downcasted to VZVirtioGraphicsScanoutConfiguration" ,)
        }
    }
}
impl INSObject for VZVirtioGraphicsScanoutConfiguration {}
impl PNSObject for VZVirtioGraphicsScanoutConfiguration {}
impl IVZVirtioGraphicsScanoutConfiguration for VZVirtioGraphicsScanoutConfiguration {}
pub trait IVZVirtioGraphicsScanoutConfiguration: Sized + std::ops::Deref {
    unsafe fn initWithWidthInPixels_heightInPixels_(
        &self,
        widthInPixels: NSInteger,
        heightInPixels: NSInteger,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithWidthInPixels : widthInPixels, heightInPixels : heightInPixels)
    }
    unsafe fn widthInPixels(&self) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, widthInPixels)
    }
    unsafe fn setWidthInPixels_(&self, widthInPixels: NSInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setWidthInPixels : widthInPixels)
    }
    unsafe fn heightInPixels(&self) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, heightInPixels)
    }
    unsafe fn setHeightInPixels_(&self, heightInPixels: NSInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setHeightInPixels : heightInPixels)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct VZVirtioNetworkDeviceConfiguration(pub id);
impl std::ops::Deref for VZVirtioNetworkDeviceConfiguration {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for VZVirtioNetworkDeviceConfiguration {}
impl VZVirtioNetworkDeviceConfiguration {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"VZVirtioNetworkDeviceConfiguration").unwrap(), alloc) })
    }
}
impl IVZNetworkDeviceConfiguration for VZVirtioNetworkDeviceConfiguration {}
impl PNSCopying for VZVirtioNetworkDeviceConfiguration {}
impl From<VZVirtioNetworkDeviceConfiguration> for VZNetworkDeviceConfiguration {
    fn from(child: VZVirtioNetworkDeviceConfiguration) -> VZNetworkDeviceConfiguration {
        VZNetworkDeviceConfiguration(child.0)
    }
}
impl std::convert::TryFrom<VZNetworkDeviceConfiguration> for VZVirtioNetworkDeviceConfiguration {
    type Error = &'static str;
    fn try_from(
        parent: VZNetworkDeviceConfiguration,
    ) -> Result<VZVirtioNetworkDeviceConfiguration, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"VZVirtioNetworkDeviceConfiguration").unwrap())
        };
        if is_kind_of {
            Ok(VZVirtioNetworkDeviceConfiguration(parent.0))
        } else {
            Err ("This VZNetworkDeviceConfiguration cannot be downcasted to VZVirtioNetworkDeviceConfiguration" ,)
        }
    }
}
impl INSObject for VZVirtioNetworkDeviceConfiguration {}
impl PNSObject for VZVirtioNetworkDeviceConfiguration {}
impl IVZVirtioNetworkDeviceConfiguration for VZVirtioNetworkDeviceConfiguration {}
pub trait IVZVirtioNetworkDeviceConfiguration: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct VZVirtioSocketConnection(pub id);
impl std::ops::Deref for VZVirtioSocketConnection {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for VZVirtioSocketConnection {}
impl VZVirtioSocketConnection {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"VZVirtioSocketConnection").unwrap(), alloc) })
    }
}
impl INSObject for VZVirtioSocketConnection {}
impl PNSObject for VZVirtioSocketConnection {}
impl std::convert::TryFrom<NSObject> for VZVirtioSocketConnection {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<VZVirtioSocketConnection, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"VZVirtioSocketConnection").unwrap()) };
        if is_kind_of {
            Ok(VZVirtioSocketConnection(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to VZVirtioSocketConnection")
        }
    }
}
impl IVZVirtioSocketConnection for VZVirtioSocketConnection {}
pub trait IVZVirtioSocketConnection: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn close(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, close)
    }
    unsafe fn destinationPort(&self) -> u32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, destinationPort)
    }
    unsafe fn sourcePort(&self) -> u32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, sourcePort)
    }
    unsafe fn fileDescriptor(&self) -> ::std::os::raw::c_int
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, fileDescriptor)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"VZVirtioSocketConnection").unwrap(), new)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct VZVirtioSocketDevice(pub id);
impl std::ops::Deref for VZVirtioSocketDevice {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for VZVirtioSocketDevice {}
impl VZVirtioSocketDevice {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"VZVirtioSocketDevice").unwrap(), alloc) })
    }
}
impl IVZSocketDevice for VZVirtioSocketDevice {}
impl From<VZVirtioSocketDevice> for VZSocketDevice {
    fn from(child: VZVirtioSocketDevice) -> VZSocketDevice {
        VZSocketDevice(child.0)
    }
}
impl std::convert::TryFrom<VZSocketDevice> for VZVirtioSocketDevice {
    type Error = &'static str;
    fn try_from(parent: VZSocketDevice) -> Result<VZVirtioSocketDevice, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"VZVirtioSocketDevice").unwrap()) };
        if is_kind_of {
            Ok(VZVirtioSocketDevice(parent.0))
        } else {
            Err("This VZSocketDevice cannot be downcasted to VZVirtioSocketDevice")
        }
    }
}
impl INSObject for VZVirtioSocketDevice {}
impl PNSObject for VZVirtioSocketDevice {}
impl IVZVirtioSocketDevice for VZVirtioSocketDevice {}
pub trait IVZVirtioSocketDevice: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn setSocketListener_forPort_(&self, listener: VZVirtioSocketListener, port: u32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSocketListener : listener, forPort : port)
    }
    unsafe fn removeSocketListenerForPort_(&self, port: u32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, removeSocketListenerForPort : port)
    }
    unsafe fn connectToPort_completionHandler_(
        &self,
        port: u32,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, connectToPort : port, completionHandler : completionHandler)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"VZVirtioSocketDevice").unwrap(), new)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct VZVirtioSocketDeviceConfiguration(pub id);
impl std::ops::Deref for VZVirtioSocketDeviceConfiguration {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for VZVirtioSocketDeviceConfiguration {}
impl VZVirtioSocketDeviceConfiguration {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"VZVirtioSocketDeviceConfiguration").unwrap(), alloc) })
    }
}
impl IVZSocketDeviceConfiguration for VZVirtioSocketDeviceConfiguration {}
impl PNSCopying for VZVirtioSocketDeviceConfiguration {}
impl From<VZVirtioSocketDeviceConfiguration> for VZSocketDeviceConfiguration {
    fn from(child: VZVirtioSocketDeviceConfiguration) -> VZSocketDeviceConfiguration {
        VZSocketDeviceConfiguration(child.0)
    }
}
impl std::convert::TryFrom<VZSocketDeviceConfiguration> for VZVirtioSocketDeviceConfiguration {
    type Error = &'static str;
    fn try_from(
        parent: VZSocketDeviceConfiguration,
    ) -> Result<VZVirtioSocketDeviceConfiguration, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"VZVirtioSocketDeviceConfiguration").unwrap())
        };
        if is_kind_of {
            Ok(VZVirtioSocketDeviceConfiguration(parent.0))
        } else {
            Err ("This VZSocketDeviceConfiguration cannot be downcasted to VZVirtioSocketDeviceConfiguration" ,)
        }
    }
}
impl INSObject for VZVirtioSocketDeviceConfiguration {}
impl PNSObject for VZVirtioSocketDeviceConfiguration {}
impl IVZVirtioSocketDeviceConfiguration for VZVirtioSocketDeviceConfiguration {}
pub trait IVZVirtioSocketDeviceConfiguration: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct VZVirtioSocketListener(pub id);
impl std::ops::Deref for VZVirtioSocketListener {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for VZVirtioSocketListener {}
impl VZVirtioSocketListener {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"VZVirtioSocketListener").unwrap(), alloc) })
    }
}
impl INSObject for VZVirtioSocketListener {}
impl PNSObject for VZVirtioSocketListener {}
impl std::convert::TryFrom<NSObject> for VZVirtioSocketListener {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<VZVirtioSocketListener, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"VZVirtioSocketListener").unwrap()) };
        if is_kind_of {
            Ok(VZVirtioSocketListener(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to VZVirtioSocketListener")
        }
    }
}
impl IVZVirtioSocketListener for VZVirtioSocketListener {}
pub trait IVZVirtioSocketListener: Sized + std::ops::Deref {
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
}
pub trait PVZVirtioSocketListenerDelegate: Sized + std::ops::Deref {
    unsafe fn listener_shouldAcceptNewConnection_fromSocketDevice_(
        &self,
        listener: VZVirtioSocketListener,
        connection: VZVirtioSocketConnection,
        socketDevice: VZVirtioSocketDevice,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, listener : listener, shouldAcceptNewConnection : connection, fromSocketDevice : socketDevice)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct VZVirtioSoundDeviceConfiguration(pub id);
impl std::ops::Deref for VZVirtioSoundDeviceConfiguration {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for VZVirtioSoundDeviceConfiguration {}
impl VZVirtioSoundDeviceConfiguration {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"VZVirtioSoundDeviceConfiguration").unwrap(), alloc) })
    }
}
impl IVZAudioDeviceConfiguration for VZVirtioSoundDeviceConfiguration {}
impl PNSCopying for VZVirtioSoundDeviceConfiguration {}
impl From<VZVirtioSoundDeviceConfiguration> for VZAudioDeviceConfiguration {
    fn from(child: VZVirtioSoundDeviceConfiguration) -> VZAudioDeviceConfiguration {
        VZAudioDeviceConfiguration(child.0)
    }
}
impl std::convert::TryFrom<VZAudioDeviceConfiguration> for VZVirtioSoundDeviceConfiguration {
    type Error = &'static str;
    fn try_from(
        parent: VZAudioDeviceConfiguration,
    ) -> Result<VZVirtioSoundDeviceConfiguration, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"VZVirtioSoundDeviceConfiguration").unwrap())
        };
        if is_kind_of {
            Ok(VZVirtioSoundDeviceConfiguration(parent.0))
        } else {
            Err ("This VZAudioDeviceConfiguration cannot be downcasted to VZVirtioSoundDeviceConfiguration" ,)
        }
    }
}
impl INSObject for VZVirtioSoundDeviceConfiguration {}
impl PNSObject for VZVirtioSoundDeviceConfiguration {}
impl IVZVirtioSoundDeviceConfiguration for VZVirtioSoundDeviceConfiguration {}
pub trait IVZVirtioSoundDeviceConfiguration: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn streams(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, streams)
    }
    unsafe fn setStreams_(&self, streams: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setStreams : streams)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct VZVirtioSoundDeviceStreamConfiguration(pub id);
impl std::ops::Deref for VZVirtioSoundDeviceStreamConfiguration {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for VZVirtioSoundDeviceStreamConfiguration {}
impl VZVirtioSoundDeviceStreamConfiguration {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"VZVirtioSoundDeviceStreamConfiguration").unwrap(), alloc) })
    }
}
impl PNSCopying for VZVirtioSoundDeviceStreamConfiguration {}
impl INSObject for VZVirtioSoundDeviceStreamConfiguration {}
impl PNSObject for VZVirtioSoundDeviceStreamConfiguration {}
impl std::convert::TryFrom<NSObject> for VZVirtioSoundDeviceStreamConfiguration {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<VZVirtioSoundDeviceStreamConfiguration, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"VZVirtioSoundDeviceStreamConfiguration").unwrap())
        };
        if is_kind_of {
            Ok(VZVirtioSoundDeviceStreamConfiguration(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to VZVirtioSoundDeviceStreamConfiguration")
        }
    }
}
impl IVZVirtioSoundDeviceStreamConfiguration for VZVirtioSoundDeviceStreamConfiguration {}
pub trait IVZVirtioSoundDeviceStreamConfiguration: Sized + std::ops::Deref {
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
        msg_send!(&*objc2::runtime::AnyClass::get(c"VZVirtioSoundDeviceStreamConfiguration").unwrap(), new)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct VZVirtioSoundDeviceInputStreamConfiguration(pub id);
impl std::ops::Deref for VZVirtioSoundDeviceInputStreamConfiguration {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for VZVirtioSoundDeviceInputStreamConfiguration {}
impl VZVirtioSoundDeviceInputStreamConfiguration {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"VZVirtioSoundDeviceInputStreamConfiguration").unwrap(), alloc) })
    }
}
impl IVZVirtioSoundDeviceStreamConfiguration for VZVirtioSoundDeviceInputStreamConfiguration {}
impl PNSCopying for VZVirtioSoundDeviceInputStreamConfiguration {}
impl From<VZVirtioSoundDeviceInputStreamConfiguration> for VZVirtioSoundDeviceStreamConfiguration {
    fn from(
        child: VZVirtioSoundDeviceInputStreamConfiguration,
    ) -> VZVirtioSoundDeviceStreamConfiguration {
        VZVirtioSoundDeviceStreamConfiguration(child.0)
    }
}
impl std::convert::TryFrom<VZVirtioSoundDeviceStreamConfiguration>
    for VZVirtioSoundDeviceInputStreamConfiguration
{
    type Error = &'static str;
    fn try_from(
        parent: VZVirtioSoundDeviceStreamConfiguration,
    ) -> Result<VZVirtioSoundDeviceInputStreamConfiguration, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"VZVirtioSoundDeviceInputStreamConfiguration").unwrap())
        };
        if is_kind_of {
            Ok(VZVirtioSoundDeviceInputStreamConfiguration(parent.0))
        } else {
            Err ("This VZVirtioSoundDeviceStreamConfiguration cannot be downcasted to VZVirtioSoundDeviceInputStreamConfiguration" ,)
        }
    }
}
impl INSObject for VZVirtioSoundDeviceInputStreamConfiguration {}
impl PNSObject for VZVirtioSoundDeviceInputStreamConfiguration {}
impl IVZVirtioSoundDeviceInputStreamConfiguration for VZVirtioSoundDeviceInputStreamConfiguration {}
pub trait IVZVirtioSoundDeviceInputStreamConfiguration: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn source(&self) -> VZAudioInputStreamSource
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, source)
    }
    unsafe fn setSource_(&self, source: VZAudioInputStreamSource)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSource : source)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct VZVirtioSoundDeviceOutputStreamConfiguration(pub id);
impl std::ops::Deref for VZVirtioSoundDeviceOutputStreamConfiguration {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for VZVirtioSoundDeviceOutputStreamConfiguration {}
impl VZVirtioSoundDeviceOutputStreamConfiguration {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"VZVirtioSoundDeviceOutputStreamConfiguration").unwrap(), alloc) })
    }
}
impl IVZVirtioSoundDeviceStreamConfiguration for VZVirtioSoundDeviceOutputStreamConfiguration {}
impl PNSCopying for VZVirtioSoundDeviceOutputStreamConfiguration {}
impl std::convert::TryFrom<VZVirtioSoundDeviceStreamConfiguration>
    for VZVirtioSoundDeviceOutputStreamConfiguration
{
    type Error = &'static str;
    fn try_from(
        parent: VZVirtioSoundDeviceStreamConfiguration,
    ) -> Result<VZVirtioSoundDeviceOutputStreamConfiguration, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"VZVirtioSoundDeviceOutputStreamConfiguration").unwrap())
        };
        if is_kind_of {
            Ok(VZVirtioSoundDeviceOutputStreamConfiguration(parent.0))
        } else {
            Err ("This VZVirtioSoundDeviceStreamConfiguration cannot be downcasted to VZVirtioSoundDeviceOutputStreamConfiguration" ,)
        }
    }
}
impl INSObject for VZVirtioSoundDeviceOutputStreamConfiguration {}
impl PNSObject for VZVirtioSoundDeviceOutputStreamConfiguration {}
impl IVZVirtioSoundDeviceOutputStreamConfiguration
    for VZVirtioSoundDeviceOutputStreamConfiguration
{
}
pub trait IVZVirtioSoundDeviceOutputStreamConfiguration: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn sink(&self) -> VZAudioOutputStreamSink
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, sink)
    }
    unsafe fn setSink_(&self, sink: VZAudioOutputStreamSink)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSink : sink)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct VZVirtioTraditionalMemoryBalloonDevice(pub id);
impl std::ops::Deref for VZVirtioTraditionalMemoryBalloonDevice {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for VZVirtioTraditionalMemoryBalloonDevice {}
impl VZVirtioTraditionalMemoryBalloonDevice {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"VZVirtioTraditionalMemoryBalloonDevice").unwrap(), alloc) })
    }
}
impl IVZMemoryBalloonDevice for VZVirtioTraditionalMemoryBalloonDevice {}
impl From<VZVirtioTraditionalMemoryBalloonDevice> for VZMemoryBalloonDevice {
    fn from(child: VZVirtioTraditionalMemoryBalloonDevice) -> VZMemoryBalloonDevice {
        VZMemoryBalloonDevice(child.0)
    }
}
impl std::convert::TryFrom<VZMemoryBalloonDevice> for VZVirtioTraditionalMemoryBalloonDevice {
    type Error = &'static str;
    fn try_from(
        parent: VZMemoryBalloonDevice,
    ) -> Result<VZVirtioTraditionalMemoryBalloonDevice, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"VZVirtioTraditionalMemoryBalloonDevice").unwrap())
        };
        if is_kind_of {
            Ok(VZVirtioTraditionalMemoryBalloonDevice(parent.0))
        } else {
            Err ("This VZMemoryBalloonDevice cannot be downcasted to VZVirtioTraditionalMemoryBalloonDevice" ,)
        }
    }
}
impl INSObject for VZVirtioTraditionalMemoryBalloonDevice {}
impl PNSObject for VZVirtioTraditionalMemoryBalloonDevice {}
impl IVZVirtioTraditionalMemoryBalloonDevice for VZVirtioTraditionalMemoryBalloonDevice {}
pub trait IVZVirtioTraditionalMemoryBalloonDevice: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn targetVirtualMachineMemorySize(&self) -> u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, targetVirtualMachineMemorySize)
    }
    unsafe fn setTargetVirtualMachineMemorySize_(&self, targetVirtualMachineMemorySize: u64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setTargetVirtualMachineMemorySize : targetVirtualMachineMemorySize)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"VZVirtioTraditionalMemoryBalloonDevice").unwrap(), new)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct VZVirtioTraditionalMemoryBalloonDeviceConfiguration(pub id);
impl std::ops::Deref for VZVirtioTraditionalMemoryBalloonDeviceConfiguration {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for VZVirtioTraditionalMemoryBalloonDeviceConfiguration {}
impl VZVirtioTraditionalMemoryBalloonDeviceConfiguration {
    pub fn alloc() -> Self {
        Self(unsafe {
            msg_send!(&*objc2::runtime::AnyClass::get(c"VZVirtioTraditionalMemoryBalloonDeviceConfiguration").unwrap(), alloc)
        })
    }
}
impl IVZMemoryBalloonDeviceConfiguration for VZVirtioTraditionalMemoryBalloonDeviceConfiguration {}
impl PNSCopying for VZVirtioTraditionalMemoryBalloonDeviceConfiguration {}
impl From<VZVirtioTraditionalMemoryBalloonDeviceConfiguration>
    for VZMemoryBalloonDeviceConfiguration
{
    fn from(
        child: VZVirtioTraditionalMemoryBalloonDeviceConfiguration,
    ) -> VZMemoryBalloonDeviceConfiguration {
        VZMemoryBalloonDeviceConfiguration(child.0)
    }
}
impl std::convert::TryFrom<VZMemoryBalloonDeviceConfiguration>
    for VZVirtioTraditionalMemoryBalloonDeviceConfiguration
{
    type Error = &'static str;
    fn try_from(
        parent: VZMemoryBalloonDeviceConfiguration,
    ) -> Result<VZVirtioTraditionalMemoryBalloonDeviceConfiguration, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"VZVirtioTraditionalMemoryBalloonDeviceConfiguration").unwrap())
        };
        if is_kind_of {
            Ok(VZVirtioTraditionalMemoryBalloonDeviceConfiguration(
                parent.0,
            ))
        } else {
            Err ("This VZMemoryBalloonDeviceConfiguration cannot be downcasted to VZVirtioTraditionalMemoryBalloonDeviceConfiguration" ,)
        }
    }
}
impl INSObject for VZVirtioTraditionalMemoryBalloonDeviceConfiguration {}
impl PNSObject for VZVirtioTraditionalMemoryBalloonDeviceConfiguration {}
impl IVZVirtioTraditionalMemoryBalloonDeviceConfiguration
    for VZVirtioTraditionalMemoryBalloonDeviceConfiguration
{
}
pub trait IVZVirtioTraditionalMemoryBalloonDeviceConfiguration: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
}
pub type VZVirtualMachineState = NSInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct VZVirtualMachine(pub id);
impl std::ops::Deref for VZVirtualMachine {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for VZVirtualMachine {}
impl VZVirtualMachine {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"VZVirtualMachine").unwrap(), alloc) })
    }
}
impl INSObject for VZVirtualMachine {}
impl PNSObject for VZVirtualMachine {}
impl std::convert::TryFrom<NSObject> for VZVirtualMachine {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<VZVirtualMachine, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"VZVirtualMachine").unwrap()) };
        if is_kind_of {
            Ok(VZVirtualMachine(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to VZVirtualMachine")
        }
    }
}
impl IVZVirtualMachine for VZVirtualMachine {}
pub trait IVZVirtualMachine: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn initWithConfiguration_(
        &self,
        configuration: VZVirtualMachineConfiguration,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithConfiguration : configuration)
    }
    unsafe fn initWithConfiguration_queue_(
        &self,
        configuration: VZVirtualMachineConfiguration,
        queue: NSObject,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithConfiguration : configuration, queue : queue)
    }
    unsafe fn startWithCompletionHandler_(&self, completionHandler: *mut ::std::os::raw::c_void)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, startWithCompletionHandler : completionHandler)
    }
    unsafe fn startWithOptions_completionHandler_(
        &self,
        options: VZVirtualMachineStartOptions,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, startWithOptions : options, completionHandler : completionHandler)
    }
    unsafe fn stopWithCompletionHandler_(&self, completionHandler: *mut ::std::os::raw::c_void)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, stopWithCompletionHandler : completionHandler)
    }
    unsafe fn pauseWithCompletionHandler_(&self, completionHandler: *mut ::std::os::raw::c_void)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, pauseWithCompletionHandler : completionHandler)
    }
    unsafe fn resumeWithCompletionHandler_(&self, completionHandler: *mut ::std::os::raw::c_void)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, resumeWithCompletionHandler : completionHandler)
    }
    unsafe fn restoreMachineStateFromURL_completionHandler_(
        &self,
        saveFileURL: NSURL,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, restoreMachineStateFromURL : saveFileURL, completionHandler : completionHandler)
    }
    unsafe fn saveMachineStateToURL_completionHandler_(
        &self,
        saveFileURL: NSURL,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, saveMachineStateToURL : saveFileURL, completionHandler : completionHandler)
    }
    unsafe fn requestStopWithError_(&self, error: *mut NSError) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, requestStopWithError : error)
    }
    unsafe fn queue(&self) -> dispatch_queue_t
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, queue)
    }
    unsafe fn state(&self) -> VZVirtualMachineState
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, state)
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
    unsafe fn canStart(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, canStart)
    }
    unsafe fn canStop(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, canStop)
    }
    unsafe fn canPause(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, canPause)
    }
    unsafe fn canResume(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, canResume)
    }
    unsafe fn canRequestStop(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, canRequestStop)
    }
    unsafe fn consoleDevices(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, consoleDevices)
    }
    unsafe fn directorySharingDevices(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, directorySharingDevices)
    }
    unsafe fn graphicsDevices(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, graphicsDevices)
    }
    unsafe fn memoryBalloonDevices(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, memoryBalloonDevices)
    }
    unsafe fn networkDevices(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, networkDevices)
    }
    unsafe fn socketDevices(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, socketDevices)
    }
    unsafe fn usbControllers(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, usbControllers)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"VZVirtualMachine").unwrap(), new)
    }
    unsafe fn isSupported() -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"VZVirtualMachine").unwrap(), isSupported)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct VZVirtualMachineConfiguration(pub id);
impl std::ops::Deref for VZVirtualMachineConfiguration {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for VZVirtualMachineConfiguration {}
impl VZVirtualMachineConfiguration {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"VZVirtualMachineConfiguration").unwrap(), alloc) })
    }
}
impl PNSCopying for VZVirtualMachineConfiguration {}
impl INSObject for VZVirtualMachineConfiguration {}
impl PNSObject for VZVirtualMachineConfiguration {}
impl std::convert::TryFrom<NSObject> for VZVirtualMachineConfiguration {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<VZVirtualMachineConfiguration, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"VZVirtualMachineConfiguration").unwrap())
        };
        if is_kind_of {
            Ok(VZVirtualMachineConfiguration(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to VZVirtualMachineConfiguration")
        }
    }
}
impl IVZVirtualMachineConfiguration for VZVirtualMachineConfiguration {}
pub trait IVZVirtualMachineConfiguration: Sized + std::ops::Deref {
    unsafe fn bootLoader(&self) -> VZBootLoader
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, bootLoader)
    }
    unsafe fn setBootLoader_(&self, bootLoader: VZBootLoader)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setBootLoader : bootLoader)
    }
    unsafe fn memorySize(&self) -> u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, memorySize)
    }
    unsafe fn setMemorySize_(&self, memorySize: u64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMemorySize : memorySize)
    }
    unsafe fn CPUCount(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, CPUCount)
    }
    unsafe fn setCPUCount_(&self, CPUCount: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCPUCount : CPUCount)
    }
    unsafe fn platform(&self) -> VZPlatformConfiguration
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, platform)
    }
    unsafe fn setPlatform_(&self, platform: VZPlatformConfiguration)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPlatform : platform)
    }
    unsafe fn audioDevices(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, audioDevices)
    }
    unsafe fn setAudioDevices_(&self, audioDevices: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAudioDevices : audioDevices)
    }
    unsafe fn consoleDevices(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, consoleDevices)
    }
    unsafe fn setConsoleDevices_(&self, consoleDevices: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setConsoleDevices : consoleDevices)
    }
    unsafe fn directorySharingDevices(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, directorySharingDevices)
    }
    unsafe fn setDirectorySharingDevices_(&self, directorySharingDevices: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDirectorySharingDevices : directorySharingDevices)
    }
    unsafe fn entropyDevices(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, entropyDevices)
    }
    unsafe fn setEntropyDevices_(&self, entropyDevices: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setEntropyDevices : entropyDevices)
    }
    unsafe fn memoryBalloonDevices(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, memoryBalloonDevices)
    }
    unsafe fn setMemoryBalloonDevices_(&self, memoryBalloonDevices: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMemoryBalloonDevices : memoryBalloonDevices)
    }
    unsafe fn networkDevices(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, networkDevices)
    }
    unsafe fn setNetworkDevices_(&self, networkDevices: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setNetworkDevices : networkDevices)
    }
    unsafe fn serialPorts(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, serialPorts)
    }
    unsafe fn setSerialPorts_(&self, serialPorts: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSerialPorts : serialPorts)
    }
    unsafe fn socketDevices(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, socketDevices)
    }
    unsafe fn setSocketDevices_(&self, socketDevices: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSocketDevices : socketDevices)
    }
    unsafe fn storageDevices(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, storageDevices)
    }
    unsafe fn setStorageDevices_(&self, storageDevices: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setStorageDevices : storageDevices)
    }
    unsafe fn keyboards(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, keyboards)
    }
    unsafe fn setKeyboards_(&self, keyboards: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setKeyboards : keyboards)
    }
    unsafe fn pointingDevices(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, pointingDevices)
    }
    unsafe fn setPointingDevices_(&self, pointingDevices: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPointingDevices : pointingDevices)
    }
    unsafe fn graphicsDevices(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, graphicsDevices)
    }
    unsafe fn setGraphicsDevices_(&self, graphicsDevices: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setGraphicsDevices : graphicsDevices)
    }
    unsafe fn usbControllers(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, usbControllers)
    }
    unsafe fn setUsbControllers_(&self, usbControllers: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setUsbControllers : usbControllers)
    }
}
impl VZVirtualMachineConfiguration_VZVirtualMachineConfigurationValidation
    for VZVirtualMachineConfiguration
{
}
pub trait VZVirtualMachineConfiguration_VZVirtualMachineConfigurationValidation:
    Sized + std::ops::Deref
{
    unsafe fn validateWithError_(&self, error: *mut NSError) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, validateWithError : error)
    }
    unsafe fn validateSaveRestoreSupportWithError_(&self, error: *mut NSError) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, validateSaveRestoreSupportWithError : error)
    }
    unsafe fn minimumAllowedMemorySize() -> u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"VZVirtualMachineConfiguration").unwrap(), minimumAllowedMemorySize)
    }
    unsafe fn maximumAllowedMemorySize() -> u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"VZVirtualMachineConfiguration").unwrap(), maximumAllowedMemorySize)
    }
    unsafe fn minimumAllowedCPUCount() -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"VZVirtualMachineConfiguration").unwrap(), minimumAllowedCPUCount)
    }
    unsafe fn maximumAllowedCPUCount() -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"VZVirtualMachineConfiguration").unwrap(), maximumAllowedCPUCount)
    }
}
pub trait PVZVirtualMachineDelegate: Sized + std::ops::Deref {
    unsafe fn guestDidStopVirtualMachine_(&self, virtualMachine: VZVirtualMachine)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, guestDidStopVirtualMachine : virtualMachine)
    }
    unsafe fn virtualMachine_didStopWithError_(
        &self,
        virtualMachine: VZVirtualMachine,
        error: NSError,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, virtualMachine : virtualMachine, didStopWithError : error)
    }
    unsafe fn virtualMachine_networkDevice_attachmentWasDisconnectedWithError_(
        &self,
        virtualMachine: VZVirtualMachine,
        networkDevice: VZNetworkDevice,
        error: NSError,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, virtualMachine : virtualMachine, networkDevice : networkDevice, attachmentWasDisconnectedWithError : error)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct VZVmnetNetworkDeviceAttachment(pub id);
impl std::ops::Deref for VZVmnetNetworkDeviceAttachment {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for VZVmnetNetworkDeviceAttachment {}
impl VZVmnetNetworkDeviceAttachment {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"VZVmnetNetworkDeviceAttachment").unwrap(), alloc) })
    }
}
impl IVZNetworkDeviceAttachment for VZVmnetNetworkDeviceAttachment {}
impl std::convert::TryFrom<VZNetworkDeviceAttachment> for VZVmnetNetworkDeviceAttachment {
    type Error = &'static str;
    fn try_from(
        parent: VZNetworkDeviceAttachment,
    ) -> Result<VZVmnetNetworkDeviceAttachment, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"VZVmnetNetworkDeviceAttachment").unwrap())
        };
        if is_kind_of {
            Ok(VZVmnetNetworkDeviceAttachment(parent.0))
        } else {
            Err ("This VZNetworkDeviceAttachment cannot be downcasted to VZVmnetNetworkDeviceAttachment" ,)
        }
    }
}
impl INSObject for VZVmnetNetworkDeviceAttachment {}
impl PNSObject for VZVmnetNetworkDeviceAttachment {}
impl IVZVmnetNetworkDeviceAttachment for VZVmnetNetworkDeviceAttachment {}
pub trait IVZVmnetNetworkDeviceAttachment: Sized + std::ops::Deref {
    unsafe fn initWithNetwork_(&self, network: vmnet_network_ref) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithNetwork : network)
    }
    unsafe fn network(&self) -> vmnet_network_ref
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, network)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct VZXHCIController(pub id);
impl std::ops::Deref for VZXHCIController {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for VZXHCIController {}
impl VZXHCIController {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"VZXHCIController").unwrap(), alloc) })
    }
}
impl IVZUSBController for VZXHCIController {}
impl From<VZXHCIController> for VZUSBController {
    fn from(child: VZXHCIController) -> VZUSBController {
        VZUSBController(child.0)
    }
}
impl std::convert::TryFrom<VZUSBController> for VZXHCIController {
    type Error = &'static str;
    fn try_from(parent: VZUSBController) -> Result<VZXHCIController, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"VZXHCIController").unwrap()) };
        if is_kind_of {
            Ok(VZXHCIController(parent.0))
        } else {
            Err("This VZUSBController cannot be downcasted to VZXHCIController")
        }
    }
}
impl INSObject for VZXHCIController {}
impl PNSObject for VZXHCIController {}
impl IVZXHCIController for VZXHCIController {}
pub trait IVZXHCIController: Sized + std::ops::Deref {}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct VZXHCIControllerConfiguration(pub id);
impl std::ops::Deref for VZXHCIControllerConfiguration {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for VZXHCIControllerConfiguration {}
impl VZXHCIControllerConfiguration {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"VZXHCIControllerConfiguration").unwrap(), alloc) })
    }
}
impl IVZUSBControllerConfiguration for VZXHCIControllerConfiguration {}
impl PNSCopying for VZXHCIControllerConfiguration {}
impl From<VZXHCIControllerConfiguration> for VZUSBControllerConfiguration {
    fn from(child: VZXHCIControllerConfiguration) -> VZUSBControllerConfiguration {
        VZUSBControllerConfiguration(child.0)
    }
}
impl std::convert::TryFrom<VZUSBControllerConfiguration> for VZXHCIControllerConfiguration {
    type Error = &'static str;
    fn try_from(
        parent: VZUSBControllerConfiguration,
    ) -> Result<VZXHCIControllerConfiguration, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"VZXHCIControllerConfiguration").unwrap())
        };
        if is_kind_of {
            Ok(VZXHCIControllerConfiguration(parent.0))
        } else {
            Err ("This VZUSBControllerConfiguration cannot be downcasted to VZXHCIControllerConfiguration" ,)
        }
    }
}
impl INSObject for VZXHCIControllerConfiguration {}
impl PNSObject for VZXHCIControllerConfiguration {}
impl IVZXHCIControllerConfiguration for VZXHCIControllerConfiguration {}
pub trait IVZXHCIControllerConfiguration: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct VZVirtualMachineView(pub id);
impl std::ops::Deref for VZVirtualMachineView {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for VZVirtualMachineView {}
impl VZVirtualMachineView {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"VZVirtualMachineView").unwrap(), alloc) })
    }
}
impl INSView for VZVirtualMachineView {}
impl PNSAnimatablePropertyContainer for VZVirtualMachineView {}
impl PNSUserInterfaceItemIdentification for VZVirtualMachineView {}
impl PNSDraggingDestination for VZVirtualMachineView {}
impl PNSAppearanceCustomization for VZVirtualMachineView {}
impl PNSAccessibilityElement for VZVirtualMachineView {}
impl PNSAccessibility for VZVirtualMachineView {}
impl std::convert::TryFrom<NSView> for VZVirtualMachineView {
    type Error = &'static str;
    fn try_from(parent: NSView) -> Result<VZVirtualMachineView, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"VZVirtualMachineView").unwrap()) };
        if is_kind_of {
            Ok(VZVirtualMachineView(parent.0))
        } else {
            Err("This NSView cannot be downcasted to VZVirtualMachineView")
        }
    }
}
impl INSResponder for VZVirtualMachineView {}
impl PNSCoding for VZVirtualMachineView {}
impl INSObject for VZVirtualMachineView {}
impl PNSObject for VZVirtualMachineView {}
impl IVZVirtualMachineView for VZVirtualMachineView {}
pub trait IVZVirtualMachineView: Sized + std::ops::Deref {
    unsafe fn virtualMachine(&self) -> VZVirtualMachine
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, virtualMachine)
    }
    unsafe fn setVirtualMachine_(&self, virtualMachine: VZVirtualMachine)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setVirtualMachine : virtualMachine)
    }
    unsafe fn capturesSystemKeys(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, capturesSystemKeys)
    }
    unsafe fn setCapturesSystemKeys_(&self, capturesSystemKeys: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCapturesSystemKeys : capturesSystemKeys)
    }
    unsafe fn automaticallyReconfiguresDisplay(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, automaticallyReconfiguresDisplay)
    }
    unsafe fn setAutomaticallyReconfiguresDisplay_(&self, automaticallyReconfiguresDisplay: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAutomaticallyReconfiguresDisplay : automaticallyReconfiguresDisplay)
    }
}
unsafe extern "C" {
    pub static VZErrorDomain: NSErrorDomain;
}

unsafe impl objc2::encode::RefEncode for VZAudioDeviceConfiguration {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for VZAudioDeviceConfiguration {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for VZAudioInputStreamSource {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for VZAudioInputStreamSource {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for VZAudioOutputStreamSink {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for VZAudioOutputStreamSink {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for VZBootLoader {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for VZBootLoader {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for VZNetworkDeviceAttachment {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for VZNetworkDeviceAttachment {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for VZBridgedNetworkDeviceAttachment {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for VZBridgedNetworkDeviceAttachment {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for VZBridgedNetworkInterface {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for VZBridgedNetworkInterface {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for VZConsoleDevice {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for VZConsoleDevice {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for VZConsoleDeviceConfiguration {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for VZConsoleDeviceConfiguration {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for VZConsolePortConfiguration {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for VZConsolePortConfiguration {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for VZDirectoryShare {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for VZDirectoryShare {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for VZDirectorySharingDevice {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for VZDirectorySharingDevice {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for VZDirectorySharingDeviceConfiguration {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for VZDirectorySharingDeviceConfiguration {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for VZStorageDeviceAttachment {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for VZStorageDeviceAttachment {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for VZDiskBlockDeviceStorageDeviceAttachment {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for VZDiskBlockDeviceStorageDeviceAttachment {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for VZDiskImageStorageDeviceAttachment {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for VZDiskImageStorageDeviceAttachment {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for VZEFIVariableStore {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for VZEFIVariableStore {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for VZEFIBootLoader {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for VZEFIBootLoader {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for VZEntropyDeviceConfiguration {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for VZEntropyDeviceConfiguration {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for VZFileHandleNetworkDeviceAttachment {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for VZFileHandleNetworkDeviceAttachment {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for VZSerialPortAttachment {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for VZSerialPortAttachment {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for VZFileHandleSerialPortAttachment {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for VZFileHandleSerialPortAttachment {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for VZFileSerialPortAttachment {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for VZFileSerialPortAttachment {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for VZGenericMachineIdentifier {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for VZGenericMachineIdentifier {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for VZPlatformConfiguration {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for VZPlatformConfiguration {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for VZGenericPlatformConfiguration {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for VZGenericPlatformConfiguration {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for VZGraphicsDevice {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for VZGraphicsDevice {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for VZGraphicsDeviceConfiguration {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for VZGraphicsDeviceConfiguration {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for VZGraphicsDisplay {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for VZGraphicsDisplay {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for VZGraphicsDisplayConfiguration {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for VZGraphicsDisplayConfiguration {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for VZHostAudioInputStreamSource {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for VZHostAudioInputStreamSource {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for VZHostAudioOutputStreamSink {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for VZHostAudioOutputStreamSink {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for VZKeyboardConfiguration {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for VZKeyboardConfiguration {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for VZLinuxBootLoader {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for VZLinuxBootLoader {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for VZLinuxRosettaCachingOptions {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for VZLinuxRosettaCachingOptions {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for VZLinuxRosettaAbstractSocketCachingOptions {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for VZLinuxRosettaAbstractSocketCachingOptions {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for VZLinuxRosettaDirectoryShare {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for VZLinuxRosettaDirectoryShare {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for VZLinuxRosettaUnixSocketCachingOptions {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for VZLinuxRosettaUnixSocketCachingOptions {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for VZMACAddress {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for VZMACAddress {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for VZMacAuxiliaryStorage {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for VZMacAuxiliaryStorage {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for VZMacGraphicsDevice {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for VZMacGraphicsDevice {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for VZMacGraphicsDeviceConfiguration {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for VZMacGraphicsDeviceConfiguration {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for VZMacGraphicsDisplay {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for VZMacGraphicsDisplay {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for VZMacGraphicsDisplayConfiguration {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for VZMacGraphicsDisplayConfiguration {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for VZMacHardwareModel {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for VZMacHardwareModel {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for VZMacKeyboardConfiguration {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for VZMacKeyboardConfiguration {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for VZMacMachineIdentifier {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for VZMacMachineIdentifier {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for VZMacOSBootLoader {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for VZMacOSBootLoader {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for VZMacOSConfigurationRequirements {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for VZMacOSConfigurationRequirements {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for VZMacOSInstaller {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for VZMacOSInstaller {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for VZMacOSRestoreImage {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for VZMacOSRestoreImage {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for VZVirtualMachineStartOptions {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for VZVirtualMachineStartOptions {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for VZMacOSVirtualMachineStartOptions {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for VZMacOSVirtualMachineStartOptions {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for VZMacPlatformConfiguration {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for VZMacPlatformConfiguration {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for VZPointingDeviceConfiguration {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for VZPointingDeviceConfiguration {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for VZMacTrackpadConfiguration {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for VZMacTrackpadConfiguration {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for VZMemoryBalloonDevice {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for VZMemoryBalloonDevice {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for VZMemoryBalloonDeviceConfiguration {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for VZMemoryBalloonDeviceConfiguration {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for VZMultipleDirectoryShare {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for VZMultipleDirectoryShare {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for VZNATNetworkDeviceAttachment {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for VZNATNetworkDeviceAttachment {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for VZStorageDeviceConfiguration {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for VZStorageDeviceConfiguration {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for VZNVMExpressControllerDeviceConfiguration {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for VZNVMExpressControllerDeviceConfiguration {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for VZNetworkBlockDeviceStorageDeviceAttachment {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for VZNetworkBlockDeviceStorageDeviceAttachment {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for VZNetworkDevice {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for VZNetworkDevice {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for VZNetworkDeviceConfiguration {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for VZNetworkDeviceConfiguration {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for VZSerialPortConfiguration {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for VZSerialPortConfiguration {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for VZSharedDirectory {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for VZSharedDirectory {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for VZSingleDirectoryShare {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for VZSingleDirectoryShare {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for VZSocketDevice {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for VZSocketDevice {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for VZSocketDeviceConfiguration {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for VZSocketDeviceConfiguration {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for VZSpiceAgentPortAttachment {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for VZSpiceAgentPortAttachment {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for VZStorageDevice {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for VZStorageDevice {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for VZUSBController {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for VZUSBController {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for VZUSBControllerConfiguration {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for VZUSBControllerConfiguration {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for VZUSBKeyboardConfiguration {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for VZUSBKeyboardConfiguration {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for VZUSBMassStorageDevice {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for VZUSBMassStorageDevice {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for VZUSBMassStorageDeviceConfiguration {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for VZUSBMassStorageDeviceConfiguration {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for VZUSBScreenCoordinatePointingDeviceConfiguration {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for VZUSBScreenCoordinatePointingDeviceConfiguration {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for VZVirtioBlockDeviceConfiguration {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for VZVirtioBlockDeviceConfiguration {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for VZVirtioConsolePort {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for VZVirtioConsolePort {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for VZVirtioConsolePortArray {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for VZVirtioConsolePortArray {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for VZVirtioConsoleDevice {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for VZVirtioConsoleDevice {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for VZVirtioConsolePortConfiguration {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for VZVirtioConsolePortConfiguration {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for VZVirtioConsolePortConfigurationArray {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for VZVirtioConsolePortConfigurationArray {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for VZVirtioConsoleDeviceConfiguration {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for VZVirtioConsoleDeviceConfiguration {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for VZVirtioConsoleDeviceSerialPortConfiguration {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for VZVirtioConsoleDeviceSerialPortConfiguration {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for VZVirtioEntropyDeviceConfiguration {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for VZVirtioEntropyDeviceConfiguration {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for VZVirtioFileSystemDevice {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for VZVirtioFileSystemDevice {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for VZVirtioFileSystemDeviceConfiguration {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for VZVirtioFileSystemDeviceConfiguration {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for VZVirtioGraphicsDevice {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for VZVirtioGraphicsDevice {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for VZVirtioGraphicsDeviceConfiguration {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for VZVirtioGraphicsDeviceConfiguration {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for VZVirtioGraphicsScanout {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for VZVirtioGraphicsScanout {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for VZVirtioGraphicsScanoutConfiguration {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for VZVirtioGraphicsScanoutConfiguration {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for VZVirtioNetworkDeviceConfiguration {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for VZVirtioNetworkDeviceConfiguration {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for VZVirtioSocketConnection {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for VZVirtioSocketConnection {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for VZVirtioSocketDevice {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for VZVirtioSocketDevice {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for VZVirtioSocketDeviceConfiguration {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for VZVirtioSocketDeviceConfiguration {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for VZVirtioSocketListener {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for VZVirtioSocketListener {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for VZVirtioSoundDeviceConfiguration {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for VZVirtioSoundDeviceConfiguration {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for VZVirtioSoundDeviceStreamConfiguration {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for VZVirtioSoundDeviceStreamConfiguration {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for VZVirtioSoundDeviceInputStreamConfiguration {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for VZVirtioSoundDeviceInputStreamConfiguration {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for VZVirtioSoundDeviceOutputStreamConfiguration {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for VZVirtioSoundDeviceOutputStreamConfiguration {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for VZVirtioTraditionalMemoryBalloonDevice {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for VZVirtioTraditionalMemoryBalloonDevice {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for VZVirtioTraditionalMemoryBalloonDeviceConfiguration {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for VZVirtioTraditionalMemoryBalloonDeviceConfiguration {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for VZVirtualMachine {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for VZVirtualMachine {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for VZVirtualMachineConfiguration {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for VZVirtualMachineConfiguration {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for VZVmnetNetworkDeviceAttachment {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for VZVmnetNetworkDeviceAttachment {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for VZXHCIController {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for VZXHCIController {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for VZXHCIControllerConfiguration {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for VZXHCIControllerConfiguration {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for VZVirtualMachineView {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for VZVirtualMachineView {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
