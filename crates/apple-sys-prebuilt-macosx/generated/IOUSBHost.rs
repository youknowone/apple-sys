#[allow(unused_imports)]
use crate::objc::*;
#[allow(unused_imports)]
use crate::CoreFoundation::*;
#[allow(unused_imports)]
use crate::Foundation::*;
#[allow(unused_imports)]
use crate::IOKit::*;
#[allow(unused_imports)]
use libc::{id_t, uid_t, uuid_t};

#[allow(unused_imports)]
use objc2::msg_send;
pub type tIOUSBDescriptorType = ::std::os::raw::c_uint;
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct IOUSBDescriptorHeader {
    pub bLength: u8,
    pub bDescriptorType: u8,
}
pub type IOUSBDescriptor = IOUSBDescriptorHeader;
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct IOUSBDeviceDescriptor {
    pub bLength: u8,
    pub bDescriptorType: u8,
    pub bcdUSB: u16,
    pub bDeviceClass: u8,
    pub bDeviceSubClass: u8,
    pub bDeviceProtocol: u8,
    pub bMaxPacketSize0: u8,
    pub idVendor: u16,
    pub idProduct: u16,
    pub bcdDevice: u16,
    pub iManufacturer: u8,
    pub iProduct: u8,
    pub iSerialNumber: u8,
    pub bNumConfigurations: u8,
}
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct IOUSBConfigurationDescriptor {
    pub bLength: u8,
    pub bDescriptorType: u8,
    pub wTotalLength: u16,
    pub bNumInterfaces: u8,
    pub bConfigurationValue: u8,
    pub iConfiguration: u8,
    pub bmAttributes: u8,
    pub MaxPower: u8,
}
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct IOUSBInterfaceDescriptor {
    pub bLength: u8,
    pub bDescriptorType: u8,
    pub bInterfaceNumber: u8,
    pub bAlternateSetting: u8,
    pub bNumEndpoints: u8,
    pub bInterfaceClass: u8,
    pub bInterfaceSubClass: u8,
    pub bInterfaceProtocol: u8,
    pub iInterface: u8,
}
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct IOUSBEndpointDescriptor {
    pub bLength: u8,
    pub bDescriptorType: u8,
    pub bEndpointAddress: u8,
    pub bmAttributes: u8,
    pub wMaxPacketSize: u16,
    pub bInterval: u8,
}
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct IOUSBBOSDescriptor {
    pub bLength: u8,
    pub bDescriptorType: u8,
    pub wTotalLength: u16,
    pub bNumDeviceCaps: u8,
}
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct IOUSBDeviceCapabilityDescriptorHeader {
    pub bLength: u8,
    pub bDescriptorType: u8,
    pub bDevCapabilityType: u8,
}
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct IOUSBDeviceCapabilityUSB2Extension {
    pub bLength: u8,
    pub bDescriptorType: u8,
    pub bDevCapabilityType: u8,
    pub bmAttributes: u32,
}
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct IOUSBDeviceCapabilitySuperSpeedUSB {
    pub bLength: u8,
    pub bDescriptorType: u8,
    pub bDevCapabilityType: u8,
    pub bmAttributes: u8,
    pub wSpeedsSupported: u16,
    pub bFunctionalitySupport: u8,
    pub bU1DevExitLat: u8,
    pub wU2DevExitLat: u16,
}
#[repr(C, packed)]
pub struct IOUSBDeviceCapabilitySuperSpeedPlusUSB {
    pub bLength: u8,
    pub bDescriptorType: u8,
    pub bDevCapabilityType: u8,
    pub bReserved: u8,
    pub bmAttributes: u32,
    pub wFunctionalitySupport: u16,
    pub wReserved: u16,
    pub bmSublinkSpeedAttr: __IncompleteArrayField<u32>,
}
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct IOUSBDeviceCapabilityContainerID {
    pub bLength: u8,
    pub bDescriptorType: u8,
    pub bDevCapabilityType: u8,
    pub bReservedID: u8,
    pub containerID: [u8; 16usize],
}
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct IOUSBPlatformCapabilityDescriptor {
    pub bLength: u8,
    pub bDescriptorType: u8,
    pub bDevCapabilityType: u8,
    pub bReserved: u8,
    pub PlatformCapabilityUUID: uuid_t,
}
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct IOUSBDeviceCapabilityBillboardAltConfig {
    pub wSVID: u16,
    pub bAltenateMode: u8,
    pub iAlternateModeString: u8,
}
#[repr(C, packed)]
pub struct IOUSBDeviceCapabilityBillboard {
    pub bLength: u8,
    pub bDescriptorType: u8,
    pub bDevCapabilityType: u8,
    pub iAdditionalInfoURL: u8,
    pub bNumberOfAlternateModes: u8,
    pub bPreferredAlternateMode: u8,
    pub vCONNPower: u16,
    pub bmConfigured: [u8; 32usize],
    pub bcdVersion: u16,
    pub bAdditionalFailureInfo: u8,
    pub bReserved: u8,
    pub pAltConfigurations: __IncompleteArrayField<IOUSBDeviceCapabilityBillboardAltConfig>,
}
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct IOUSBInterfaceAssociationDescriptor {
    pub bLength: u8,
    pub bDescriptorType: u8,
    pub bFirstInterface: u8,
    pub bInterfaceCount: u8,
    pub bFunctionClass: u8,
    pub bFunctionSubClass: u8,
    pub bFunctionProtocol: u8,
    pub iFunction: u8,
}
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct IOUSBSuperSpeedEndpointCompanionDescriptor {
    pub bLength: u8,
    pub bDescriptorType: u8,
    pub bMaxBurst: u8,
    pub bmAttributes: u8,
    pub wBytesPerInterval: u16,
}
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct IOUSBSuperSpeedPlusIsochronousEndpointCompanionDescriptor {
    pub bLength: u8,
    pub bDescriptorType: u8,
    pub wReserved: u16,
    pub dwBytesPerInterval: u32,
}
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct IOUSBDeviceRequest {
    pub bmRequestType: u8,
    pub bRequest: u8,
    pub wValue: u16,
    pub wIndex: u16,
    pub wLength: u16,
}
pub type tIOUSBDeviceRequestTypeValue = ::std::os::raw::c_uint;
pub type tIOUSBDeviceRequestRecipientValue = ::std::os::raw::c_uint;
pub type IOUSBHostCompletionHandler = *mut ::std::os::raw::c_void;
pub type IOUSBHostTime = u64;
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct IOUSBHostIsochronousFrame {
    pub status: IOReturn,
    pub requestCount: u32,
    pub completeCount: u32,
    pub reserved: u32,
    pub timeStamp: IOUSBHostTime,
}
pub type IOUSBHostIsochronousTransferOptions = UInt32;
pub type IOUSBHostIsochronousTransactionOptions = UInt32;
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct IOUSBHostIsochronousTransaction {
    pub status: IOReturn,
    pub requestCount: u32,
    pub offset: u32,
    pub completeCount: u32,
    pub timeStamp: IOUSBHostTime,
    pub options: IOUSBHostIsochronousTransactionOptions,
}
pub type IOUSBHostIsochronousCompletionHandler = *mut ::std::os::raw::c_void;
pub type IOUSBHostIsochronousTransactionCompletionHandler = *mut ::std::os::raw::c_void;
pub type IOUSBHostAbortOption = NSUInteger;
pub type IOUSBHostObjectInitOptions = NSUInteger;
pub type IOUSBHostObjectDestroyOptions = NSUInteger;
pub type IOUSBHostMatchingPropertyKey = NSString;
pub type IOUSBHostPropertyKey = NSString;
pub type IOUSBHostDevicePropertyKey = NSString;
pub type IOUSBHostInterfacePropertyKey = NSString;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct IOUSBHostObject(pub id);
impl std::ops::Deref for IOUSBHostObject {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for IOUSBHostObject {}
impl IOUSBHostObject {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"IOUSBHostObject").unwrap(), alloc) })
    }
}
impl INSObject for IOUSBHostObject {}
impl PNSObject for IOUSBHostObject {}
impl std::convert::TryFrom<NSObject> for IOUSBHostObject {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<IOUSBHostObject, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"IOUSBHostObject").unwrap()) };
        if is_kind_of {
            Ok(IOUSBHostObject(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to IOUSBHostObject")
        }
    }
}
impl IIOUSBHostObject for IOUSBHostObject {}
pub trait IIOUSBHostObject: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn initWithIOService_options_queue_error_interestHandler_(
        &self,
        ioService: io_service_t,
        options: IOUSBHostObjectInitOptions,
        queue: NSObject,
        error: *mut NSError,
        interestHandler: IOUSBHostInterestHandler,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithIOService : ioService, options : options, queue : queue, error : error, interestHandler : interestHandler)
    }
    unsafe fn initWithIOService_queue_error_interestHandler_(
        &self,
        ioService: io_service_t,
        queue: NSObject,
        error: *mut NSError,
        interestHandler: IOUSBHostInterestHandler,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithIOService : ioService, queue : queue, error : error, interestHandler : interestHandler)
    }
    unsafe fn destroy(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, destroy)
    }
    unsafe fn destroyWithOptions_(&self, options: IOUSBHostObjectDestroyOptions)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, destroyWithOptions : options)
    }
    unsafe fn sendDeviceRequest_data_bytesTransferred_completionTimeout_error_(
        &self,
        request: IOUSBDeviceRequest,
        data: NSMutableData,
        bytesTransferred: *mut NSUInteger,
        completionTimeout: NSTimeInterval,
        error: *mut NSError,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, sendDeviceRequest : request, data : data, bytesTransferred : bytesTransferred, completionTimeout : completionTimeout, error : error)
    }
    unsafe fn sendDeviceRequest_data_bytesTransferred_error_(
        &self,
        request: IOUSBDeviceRequest,
        data: NSMutableData,
        bytesTransferred: *mut NSUInteger,
        error: *mut NSError,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, sendDeviceRequest : request, data : data, bytesTransferred : bytesTransferred, error : error)
    }
    unsafe fn sendDeviceRequest_error_(
        &self,
        request: IOUSBDeviceRequest,
        error: *mut NSError,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, sendDeviceRequest : request, error : error)
    }
    unsafe fn enqueueDeviceRequest_data_completionTimeout_error_completionHandler_(
        &self,
        request: IOUSBDeviceRequest,
        data: NSMutableData,
        completionTimeout: NSTimeInterval,
        error: *mut NSError,
        completionHandler: IOUSBHostCompletionHandler,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, enqueueDeviceRequest : request, data : data, completionTimeout : completionTimeout, error : error, completionHandler : completionHandler)
    }
    unsafe fn enqueueDeviceRequest_data_error_completionHandler_(
        &self,
        request: IOUSBDeviceRequest,
        data: NSMutableData,
        error: *mut NSError,
        completionHandler: IOUSBHostCompletionHandler,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, enqueueDeviceRequest : request, data : data, error : error, completionHandler : completionHandler)
    }
    unsafe fn enqueueDeviceRequest_error_completionHandler_(
        &self,
        request: IOUSBDeviceRequest,
        error: *mut NSError,
        completionHandler: IOUSBHostCompletionHandler,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, enqueueDeviceRequest : request, error : error, completionHandler : completionHandler)
    }
    unsafe fn abortDeviceRequestsWithOption_error_(
        &self,
        option: IOUSBHostAbortOption,
        error: *mut NSError,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, abortDeviceRequestsWithOption : option, error : error)
    }
    unsafe fn abortDeviceRequestsWithError_(&self, error: *mut NSError) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, abortDeviceRequestsWithError : error)
    }
    unsafe fn descriptorWithType_length_index_languageID_requestType_requestRecipient_error_(
        &self,
        type_: tIOUSBDescriptorType,
        length: *mut NSUInteger,
        index: NSUInteger,
        languageID: NSUInteger,
        requestType: tIOUSBDeviceRequestTypeValue,
        requestRecipient: tIOUSBDeviceRequestRecipientValue,
        error: *mut NSError,
    ) -> *const IOUSBDescriptor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, descriptorWithType : type_, length : length, index : index, languageID : languageID, requestType : requestType, requestRecipient : requestRecipient, error : error)
    }
    unsafe fn descriptorWithType_length_index_languageID_error_(
        &self,
        type_: tIOUSBDescriptorType,
        length: *mut NSUInteger,
        index: NSUInteger,
        languageID: NSUInteger,
        error: *mut NSError,
    ) -> *const IOUSBDescriptor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, descriptorWithType : type_, length : length, index : index, languageID : languageID, error : error)
    }
    unsafe fn descriptorWithType_length_error_(
        &self,
        type_: tIOUSBDescriptorType,
        length: *mut NSUInteger,
        error: *mut NSError,
    ) -> *const IOUSBDescriptor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, descriptorWithType : type_, length : length, error : error)
    }
    unsafe fn configurationDescriptorWithIndex_error_(
        &self,
        index: NSUInteger,
        error: *mut NSError,
    ) -> *const IOUSBConfigurationDescriptor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, configurationDescriptorWithIndex : index, error : error)
    }
    unsafe fn configurationDescriptorWithConfigurationValue_error_(
        &self,
        configurationValue: NSUInteger,
        error: *mut NSError,
    ) -> *const IOUSBConfigurationDescriptor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, configurationDescriptorWithConfigurationValue : configurationValue, error : error)
    }
    unsafe fn stringWithIndex_languageID_error_(
        &self,
        index: NSUInteger,
        languageID: NSUInteger,
        error: *mut NSError,
    ) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, stringWithIndex : index, languageID : languageID, error : error)
    }
    unsafe fn stringWithIndex_error_(&self, index: NSUInteger, error: *mut NSError) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, stringWithIndex : index, error : error)
    }
    unsafe fn frameNumberWithTime_(&self, time: *mut IOUSBHostTime) -> u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, frameNumberWithTime : time)
    }
    unsafe fn currentMicroframeWithTime_error_(
        &self,
        time: *mut IOUSBHostTime,
        error: *mut NSError,
    ) -> u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, currentMicroframeWithTime : time, error : error)
    }
    unsafe fn referenceMicroframeWithTime_error_(
        &self,
        time: *mut IOUSBHostTime,
        error: *mut NSError,
    ) -> u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, referenceMicroframeWithTime : time, error : error)
    }
    unsafe fn ioDataWithCapacity_error_(
        &self,
        capacity: NSUInteger,
        error: *mut NSError,
    ) -> NSMutableData
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, ioDataWithCapacity : capacity, error : error)
    }
    unsafe fn ioService(&self) -> io_service_t
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, ioService)
    }
    unsafe fn queue(&self) -> dispatch_queue_t
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, queue)
    }
    unsafe fn deviceDescriptor(&self) -> *const IOUSBDeviceDescriptor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, deviceDescriptor)
    }
    unsafe fn capabilityDescriptors(&self) -> *const IOUSBBOSDescriptor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, capabilityDescriptors)
    }
    unsafe fn deviceAddress(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, deviceAddress)
    }
}
pub type IOUSBHostInterestHandler = *mut ::std::os::raw::c_void;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct IOUSBHostDevice(pub id);
impl std::ops::Deref for IOUSBHostDevice {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for IOUSBHostDevice {}
impl IOUSBHostDevice {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"IOUSBHostDevice").unwrap(), alloc) })
    }
}
impl IIOUSBHostObject for IOUSBHostDevice {}
impl From<IOUSBHostDevice> for IOUSBHostObject {
    fn from(child: IOUSBHostDevice) -> IOUSBHostObject {
        IOUSBHostObject(child.0)
    }
}
impl std::convert::TryFrom<IOUSBHostObject> for IOUSBHostDevice {
    type Error = &'static str;
    fn try_from(parent: IOUSBHostObject) -> Result<IOUSBHostDevice, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"IOUSBHostDevice").unwrap()) };
        if is_kind_of {
            Ok(IOUSBHostDevice(parent.0))
        } else {
            Err("This IOUSBHostObject cannot be downcasted to IOUSBHostDevice")
        }
    }
}
impl INSObject for IOUSBHostDevice {}
impl PNSObject for IOUSBHostDevice {}
impl IIOUSBHostDevice for IOUSBHostDevice {}
pub trait IIOUSBHostDevice: Sized + std::ops::Deref {
    unsafe fn configureWithValue_matchInterfaces_error_(
        &self,
        value: NSUInteger,
        matchInterfaces: BOOL,
        error: *mut NSError,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, configureWithValue : value, matchInterfaces : matchInterfaces, error : error)
    }
    unsafe fn configureWithValue_error_(&self, value: NSUInteger, error: *mut NSError) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, configureWithValue : value, error : error)
    }
    unsafe fn resetWithError_(&self, error: *mut NSError) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, resetWithError : error)
    }
    unsafe fn configurationDescriptor(&self) -> *const IOUSBConfigurationDescriptor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, configurationDescriptor)
    }
    unsafe fn createMatchingDictionaryWithVendorID_productID_bcdDevice_deviceClass_deviceSubclass_deviceProtocol_speed_productIDArray_(
        vendorID: NSNumber,
        productID: NSNumber,
        bcdDevice: NSNumber,
        deviceClass: NSNumber,
        deviceSubclass: NSNumber,
        deviceProtocol: NSNumber,
        speed: NSNumber,
        productIDArray: NSArray,
    ) -> CFMutableDictionaryRef
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"IOUSBHostDevice").unwrap(), createMatchingDictionaryWithVendorID : vendorID, productID : productID, bcdDevice : bcdDevice, deviceClass : deviceClass, deviceSubclass : deviceSubclass, deviceProtocol : deviceProtocol, speed : speed, productIDArray : productIDArray)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct IOUSBHostInterface(pub id);
impl std::ops::Deref for IOUSBHostInterface {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for IOUSBHostInterface {}
impl IOUSBHostInterface {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"IOUSBHostInterface").unwrap(), alloc) })
    }
}
impl IIOUSBHostObject for IOUSBHostInterface {}
impl std::convert::TryFrom<IOUSBHostObject> for IOUSBHostInterface {
    type Error = &'static str;
    fn try_from(parent: IOUSBHostObject) -> Result<IOUSBHostInterface, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"IOUSBHostInterface").unwrap()) };
        if is_kind_of {
            Ok(IOUSBHostInterface(parent.0))
        } else {
            Err("This IOUSBHostObject cannot be downcasted to IOUSBHostInterface")
        }
    }
}
impl INSObject for IOUSBHostInterface {}
impl PNSObject for IOUSBHostInterface {}
impl IIOUSBHostInterface for IOUSBHostInterface {}
pub trait IIOUSBHostInterface: Sized + std::ops::Deref {
    unsafe fn initWithIOService_options_queue_error_interestHandler_(
        &self,
        ioService: io_service_t,
        options: IOUSBHostObjectInitOptions,
        queue: NSObject,
        error: *mut NSError,
        interestHandler: IOUSBHostInterestHandler,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithIOService : ioService, options : options, queue : queue, error : error, interestHandler : interestHandler)
    }
    unsafe fn setIdleTimeout_error_(&self, idleTimeout: NSTimeInterval, error: *mut NSError) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setIdleTimeout : idleTimeout, error : error)
    }
    unsafe fn selectAlternateSetting_error_(
        &self,
        alternateSetting: NSUInteger,
        error: *mut NSError,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, selectAlternateSetting : alternateSetting, error : error)
    }
    unsafe fn copyPipeWithAddress_error_(
        &self,
        address: NSUInteger,
        error: *mut NSError,
    ) -> IOUSBHostPipe
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, copyPipeWithAddress : address, error : error)
    }
    unsafe fn idleTimeout(&self) -> NSTimeInterval
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, idleTimeout)
    }
    unsafe fn configurationDescriptor(&self) -> *const IOUSBConfigurationDescriptor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, configurationDescriptor)
    }
    unsafe fn interfaceDescriptor(&self) -> *const IOUSBInterfaceDescriptor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, interfaceDescriptor)
    }
    unsafe fn createMatchingDictionaryWithVendorID_productID_bcdDevice_interfaceNumber_configurationValue_interfaceClass_interfaceSubclass_interfaceProtocol_speed_productIDArray_(
        vendorID: NSNumber,
        productID: NSNumber,
        bcdDevice: NSNumber,
        interfaceNumber: NSNumber,
        configurationValue: NSNumber,
        interfaceClass: NSNumber,
        interfaceSubclass: NSNumber,
        interfaceProtocol: NSNumber,
        speed: NSNumber,
        productIDArray: NSArray,
    ) -> CFMutableDictionaryRef
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"IOUSBHostInterface").unwrap(), createMatchingDictionaryWithVendorID : vendorID, productID : productID, bcdDevice : bcdDevice, interfaceNumber : interfaceNumber, configurationValue : configurationValue, interfaceClass : interfaceClass, interfaceSubclass : interfaceSubclass, interfaceProtocol : interfaceProtocol, speed : speed, productIDArray : productIDArray)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct IOUSBHostIOSource(pub id);
impl std::ops::Deref for IOUSBHostIOSource {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for IOUSBHostIOSource {}
impl IOUSBHostIOSource {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"IOUSBHostIOSource").unwrap(), alloc) })
    }
}
impl INSObject for IOUSBHostIOSource {}
impl PNSObject for IOUSBHostIOSource {}
impl std::convert::TryFrom<NSObject> for IOUSBHostIOSource {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<IOUSBHostIOSource, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"IOUSBHostIOSource").unwrap()) };
        if is_kind_of {
            Ok(IOUSBHostIOSource(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to IOUSBHostIOSource")
        }
    }
}
impl IIOUSBHostIOSource for IOUSBHostIOSource {}
pub trait IIOUSBHostIOSource: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn hostInterface(&self) -> IOUSBHostInterface
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, hostInterface)
    }
    unsafe fn deviceAddress(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, deviceAddress)
    }
    unsafe fn endpointAddress(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, endpointAddress)
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct IOUSBHostIOSourceDescriptors {
    pub bcdUSB: u16,
    pub descriptor: IOUSBEndpointDescriptor,
    pub ssCompanionDescriptor: IOUSBSuperSpeedEndpointCompanionDescriptor,
    pub sspCompanionDescriptor: IOUSBSuperSpeedPlusIsochronousEndpointCompanionDescriptor,
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct IOUSBHostPipe(pub id);
impl std::ops::Deref for IOUSBHostPipe {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for IOUSBHostPipe {}
impl IOUSBHostPipe {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"IOUSBHostPipe").unwrap(), alloc) })
    }
}
impl IIOUSBHostIOSource for IOUSBHostPipe {}
impl From<IOUSBHostPipe> for IOUSBHostIOSource {
    fn from(child: IOUSBHostPipe) -> IOUSBHostIOSource {
        IOUSBHostIOSource(child.0)
    }
}
impl std::convert::TryFrom<IOUSBHostIOSource> for IOUSBHostPipe {
    type Error = &'static str;
    fn try_from(parent: IOUSBHostIOSource) -> Result<IOUSBHostPipe, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"IOUSBHostPipe").unwrap()) };
        if is_kind_of {
            Ok(IOUSBHostPipe(parent.0))
        } else {
            Err("This IOUSBHostIOSource cannot be downcasted to IOUSBHostPipe")
        }
    }
}
impl INSObject for IOUSBHostPipe {}
impl PNSObject for IOUSBHostPipe {}
impl IIOUSBHostPipe for IOUSBHostPipe {}
pub trait IIOUSBHostPipe: Sized + std::ops::Deref {
    unsafe fn adjustPipeWithDescriptors_error_(
        &self,
        descriptors: *const IOUSBHostIOSourceDescriptors,
        error: *mut NSError,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, adjustPipeWithDescriptors : descriptors, error : error)
    }
    unsafe fn setIdleTimeout_error_(&self, idleTimeout: NSTimeInterval, error: *mut NSError) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setIdleTimeout : idleTimeout, error : error)
    }
    unsafe fn clearStallWithError_(&self, error: *mut NSError) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, clearStallWithError : error)
    }
    unsafe fn sendControlRequest_data_bytesTransferred_completionTimeout_error_(
        &self,
        request: IOUSBDeviceRequest,
        data: NSMutableData,
        bytesTransferred: *mut NSUInteger,
        completionTimeout: NSTimeInterval,
        error: *mut NSError,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, sendControlRequest : request, data : data, bytesTransferred : bytesTransferred, completionTimeout : completionTimeout, error : error)
    }
    unsafe fn sendControlRequest_data_bytesTransferred_error_(
        &self,
        request: IOUSBDeviceRequest,
        data: NSMutableData,
        bytesTransferred: *mut NSUInteger,
        error: *mut NSError,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, sendControlRequest : request, data : data, bytesTransferred : bytesTransferred, error : error)
    }
    unsafe fn sendControlRequest_error_(
        &self,
        request: IOUSBDeviceRequest,
        error: *mut NSError,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, sendControlRequest : request, error : error)
    }
    unsafe fn enqueueControlRequest_data_completionTimeout_error_completionHandler_(
        &self,
        request: IOUSBDeviceRequest,
        data: NSMutableData,
        completionTimeout: NSTimeInterval,
        error: *mut NSError,
        completionHandler: IOUSBHostCompletionHandler,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, enqueueControlRequest : request, data : data, completionTimeout : completionTimeout, error : error, completionHandler : completionHandler)
    }
    unsafe fn enqueueControlRequest_data_error_completionHandler_(
        &self,
        request: IOUSBDeviceRequest,
        data: NSMutableData,
        error: *mut NSError,
        completionHandler: IOUSBHostCompletionHandler,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, enqueueControlRequest : request, data : data, error : error, completionHandler : completionHandler)
    }
    unsafe fn enqueueControlRequest_error_completionHandler_(
        &self,
        request: IOUSBDeviceRequest,
        error: *mut NSError,
        completionHandler: IOUSBHostCompletionHandler,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, enqueueControlRequest : request, error : error, completionHandler : completionHandler)
    }
    unsafe fn abortWithOption_error_(
        &self,
        option: IOUSBHostAbortOption,
        error: *mut NSError,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, abortWithOption : option, error : error)
    }
    unsafe fn abortWithError_(&self, error: *mut NSError) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, abortWithError : error)
    }
    unsafe fn sendIORequestWithData_bytesTransferred_completionTimeout_error_(
        &self,
        data: NSMutableData,
        bytesTransferred: *mut NSUInteger,
        completionTimeout: NSTimeInterval,
        error: *mut NSError,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, sendIORequestWithData : data, bytesTransferred : bytesTransferred, completionTimeout : completionTimeout, error : error)
    }
    unsafe fn enqueueIORequestWithData_completionTimeout_error_completionHandler_(
        &self,
        data: NSMutableData,
        completionTimeout: NSTimeInterval,
        error: *mut NSError,
        completionHandler: IOUSBHostCompletionHandler,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, enqueueIORequestWithData : data, completionTimeout : completionTimeout, error : error, completionHandler : completionHandler)
    }
    unsafe fn sendIORequestWithData_frameList_frameListCount_firstFrameNumber_error_(
        &self,
        data: NSMutableData,
        frameList: *mut IOUSBHostIsochronousFrame,
        frameListCount: NSUInteger,
        firstFrameNumber: u64,
        error: *mut NSError,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, sendIORequestWithData : data, frameList : frameList, frameListCount : frameListCount, firstFrameNumber : firstFrameNumber, error : error)
    }
    unsafe fn enqueueIORequestWithData_frameList_frameListCount_firstFrameNumber_error_completionHandler_(
        &self,
        data: NSMutableData,
        frameList: *mut IOUSBHostIsochronousFrame,
        frameListCount: NSUInteger,
        firstFrameNumber: u64,
        error: *mut NSError,
        completionHandler: IOUSBHostIsochronousCompletionHandler,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, enqueueIORequestWithData : data, frameList : frameList, frameListCount : frameListCount, firstFrameNumber : firstFrameNumber, error : error, completionHandler : completionHandler)
    }
    unsafe fn sendIORequestWithData_transactionList_transactionListCount_firstFrameNumber_options_error_(
        &self,
        data: NSMutableData,
        transactionList: *mut IOUSBHostIsochronousTransaction,
        transactionListCount: NSUInteger,
        firstFrameNumber: u64,
        options: IOUSBHostIsochronousTransferOptions,
        error: *mut NSError,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, sendIORequestWithData : data, transactionList : transactionList, transactionListCount : transactionListCount, firstFrameNumber : firstFrameNumber, options : options, error : error)
    }
    unsafe fn enqueueIORequestWithData_transactionList_transactionListCount_firstFrameNumber_options_error_completionHandler_(
        &self,
        data: NSMutableData,
        transactionList: *mut IOUSBHostIsochronousTransaction,
        transactionListCount: NSUInteger,
        firstFrameNumber: u64,
        options: IOUSBHostIsochronousTransferOptions,
        error: *mut NSError,
        completionHandler: IOUSBHostIsochronousTransactionCompletionHandler,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, enqueueIORequestWithData : data, transactionList : transactionList, transactionListCount : transactionListCount, firstFrameNumber : firstFrameNumber, options : options, error : error, completionHandler : completionHandler)
    }
    unsafe fn enableStreamsWithError_(&self, error: *mut NSError) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, enableStreamsWithError : error)
    }
    unsafe fn disableStreamsWithError_(&self, error: *mut NSError) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, disableStreamsWithError : error)
    }
    unsafe fn copyStreamWithStreamID_error_(
        &self,
        streamID: NSUInteger,
        error: *mut NSError,
    ) -> IOUSBHostStream
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, copyStreamWithStreamID : streamID, error : error)
    }
    unsafe fn originalDescriptors(&self) -> *const IOUSBHostIOSourceDescriptors
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, originalDescriptors)
    }
    unsafe fn descriptors(&self) -> *const IOUSBHostIOSourceDescriptors
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, descriptors)
    }
    unsafe fn idleTimeout(&self) -> NSTimeInterval
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, idleTimeout)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct IOUSBHostStream(pub id);
impl std::ops::Deref for IOUSBHostStream {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for IOUSBHostStream {}
impl IOUSBHostStream {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"IOUSBHostStream").unwrap(), alloc) })
    }
}
impl IIOUSBHostIOSource for IOUSBHostStream {}
impl std::convert::TryFrom<IOUSBHostIOSource> for IOUSBHostStream {
    type Error = &'static str;
    fn try_from(parent: IOUSBHostIOSource) -> Result<IOUSBHostStream, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"IOUSBHostStream").unwrap()) };
        if is_kind_of {
            Ok(IOUSBHostStream(parent.0))
        } else {
            Err("This IOUSBHostIOSource cannot be downcasted to IOUSBHostStream")
        }
    }
}
impl INSObject for IOUSBHostStream {}
impl PNSObject for IOUSBHostStream {}
impl IIOUSBHostStream for IOUSBHostStream {}
pub trait IIOUSBHostStream: Sized + std::ops::Deref {
    unsafe fn abortWithOption_error_(
        &self,
        option: IOUSBHostAbortOption,
        error: *mut NSError,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, abortWithOption : option, error : error)
    }
    unsafe fn abortWithError_(&self, error: *mut NSError) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, abortWithError : error)
    }
    unsafe fn sendIORequestWithData_bytesTransferred_error_(
        &self,
        data: NSMutableData,
        bytesTransferred: *mut NSUInteger,
        error: *mut NSError,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, sendIORequestWithData : data, bytesTransferred : bytesTransferred, error : error)
    }
    unsafe fn enqueueIORequestWithData_error_completionHandler_(
        &self,
        data: NSMutableData,
        error: *mut NSError,
        completionHandler: IOUSBHostCompletionHandler,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, enqueueIORequestWithData : data, error : error, completionHandler : completionHandler)
    }
    unsafe fn hostPipe(&self) -> IOUSBHostPipe
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, hostPipe)
    }
    unsafe fn streamID(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, streamID)
    }
}
pub type IOUSBHostCIUserClientVersion = ::std::os::raw::c_uint;
pub type IOUSBHostCIExceptionType = ::std::os::raw::c_uint;
pub type IOUSBHostCIMessageType = ::std::os::raw::c_uint;
pub type IOUSBHostCIMessageStatus = ::std::os::raw::c_uint;
pub type IOUSBHostCIDeviceSpeed = ::std::os::raw::c_uint;
pub type IOUSBHostCILinkState = ::std::os::raw::c_uint;
pub type IOUSBHostCIPortStatus = u32;
pub type IOUSBHostCIDoorbell = u32;
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct IOUSBHostCIMessage {
    pub control: u32,
    pub data0: u32,
    pub data1: u64,
}
pub type IOUSBHostCIControllerState = ::std::os::raw::c_uint;
pub type IOUSBHostCIPortState = ::std::os::raw::c_uint;
pub type IOUSBHostCIDeviceState = ::std::os::raw::c_uint;
pub type IOUSBHostCIEndpointState = ::std::os::raw::c_uint;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct IOUSBHostCIControllerStateMachine(pub id);
impl std::ops::Deref for IOUSBHostCIControllerStateMachine {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for IOUSBHostCIControllerStateMachine {}
impl IOUSBHostCIControllerStateMachine {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"IOUSBHostCIControllerStateMachine").unwrap(), alloc) })
    }
}
impl INSObject for IOUSBHostCIControllerStateMachine {}
impl PNSObject for IOUSBHostCIControllerStateMachine {}
impl std::convert::TryFrom<NSObject> for IOUSBHostCIControllerStateMachine {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<IOUSBHostCIControllerStateMachine, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"IOUSBHostCIControllerStateMachine").unwrap())
        };
        if is_kind_of {
            Ok(IOUSBHostCIControllerStateMachine(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to IOUSBHostCIControllerStateMachine")
        }
    }
}
impl IIOUSBHostCIControllerStateMachine for IOUSBHostCIControllerStateMachine {}
pub trait IIOUSBHostCIControllerStateMachine: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn initWithInterface_error_(
        &self,
        interface: IOUSBHostControllerInterface,
        error: *mut NSError,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithInterface : interface, error : error)
    }
    unsafe fn inspectCommand_error_(
        &self,
        command: *const IOUSBHostCIMessage,
        error: *mut NSError,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, inspectCommand : command, error : error)
    }
    unsafe fn respondToCommand_status_error_(
        &self,
        command: *const IOUSBHostCIMessage,
        status: IOUSBHostCIMessageStatus,
        error: *mut NSError,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, respondToCommand : command, status : status, error : error)
    }
    unsafe fn respondToCommand_status_frame_timestamp_error_(
        &self,
        command: *const IOUSBHostCIMessage,
        status: IOUSBHostCIMessageStatus,
        frame: u64,
        timestamp: u64,
        error: *mut NSError,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, respondToCommand : command, status : status, frame : frame, timestamp : timestamp, error : error)
    }
    unsafe fn enqueueUpdatedFrame_timestamp_error_(
        &self,
        frame: u64,
        timestamp: u64,
        error: *mut NSError,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, enqueueUpdatedFrame : frame, timestamp : timestamp, error : error)
    }
    unsafe fn controllerState(&self) -> IOUSBHostCIControllerState
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, controllerState)
    }
    unsafe fn controllerInterface(&self) -> IOUSBHostControllerInterface
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, controllerInterface)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct IOUSBHostCIPortStateMachine(pub id);
impl std::ops::Deref for IOUSBHostCIPortStateMachine {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for IOUSBHostCIPortStateMachine {}
impl IOUSBHostCIPortStateMachine {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"IOUSBHostCIPortStateMachine").unwrap(), alloc) })
    }
}
impl INSObject for IOUSBHostCIPortStateMachine {}
impl PNSObject for IOUSBHostCIPortStateMachine {}
impl std::convert::TryFrom<NSObject> for IOUSBHostCIPortStateMachine {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<IOUSBHostCIPortStateMachine, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"IOUSBHostCIPortStateMachine").unwrap()) };
        if is_kind_of {
            Ok(IOUSBHostCIPortStateMachine(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to IOUSBHostCIPortStateMachine")
        }
    }
}
impl IIOUSBHostCIPortStateMachine for IOUSBHostCIPortStateMachine {}
pub trait IIOUSBHostCIPortStateMachine: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn initWithInterface_portNumber_error_(
        &self,
        interface: IOUSBHostControllerInterface,
        portNumber: NSUInteger,
        error: *mut NSError,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithInterface : interface, portNumber : portNumber, error : error)
    }
    unsafe fn inspectCommand_error_(
        &self,
        command: *const IOUSBHostCIMessage,
        error: *mut NSError,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, inspectCommand : command, error : error)
    }
    unsafe fn respondToCommand_status_error_(
        &self,
        command: *const IOUSBHostCIMessage,
        status: IOUSBHostCIMessageStatus,
        error: *mut NSError,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, respondToCommand : command, status : status, error : error)
    }
    unsafe fn updateLinkState_speed_inhibitLinkStateChange_error_(
        &self,
        linkState: IOUSBHostCILinkState,
        speed: IOUSBHostCIDeviceSpeed,
        inhibitLinkStateChange: BOOL,
        error: *mut NSError,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, updateLinkState : linkState, speed : speed, inhibitLinkStateChange : inhibitLinkStateChange, error : error)
    }
    unsafe fn portNumber(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, portNumber)
    }
    unsafe fn portState(&self) -> IOUSBHostCIPortState
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, portState)
    }
    unsafe fn portStatus(&self) -> IOUSBHostCIPortStatus
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, portStatus)
    }
    unsafe fn controllerInterface(&self) -> IOUSBHostControllerInterface
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, controllerInterface)
    }
    unsafe fn powered(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, powered)
    }
    unsafe fn setPowered_(&self, powered: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPowered : powered)
    }
    unsafe fn connected(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, connected)
    }
    unsafe fn setConnected_(&self, connected: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setConnected : connected)
    }
    unsafe fn overcurrent(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, overcurrent)
    }
    unsafe fn setOvercurrent_(&self, overcurrent: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setOvercurrent : overcurrent)
    }
    unsafe fn linkState(&self) -> IOUSBHostCILinkState
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, linkState)
    }
    unsafe fn speed(&self) -> IOUSBHostCIDeviceSpeed
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, speed)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct IOUSBHostControllerInterface(pub id);
impl std::ops::Deref for IOUSBHostControllerInterface {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for IOUSBHostControllerInterface {}
impl IOUSBHostControllerInterface {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"IOUSBHostControllerInterface").unwrap(), alloc) })
    }
}
impl INSObject for IOUSBHostControllerInterface {}
impl PNSObject for IOUSBHostControllerInterface {}
impl std::convert::TryFrom<NSObject> for IOUSBHostControllerInterface {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<IOUSBHostControllerInterface, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"IOUSBHostControllerInterface").unwrap()) };
        if is_kind_of {
            Ok(IOUSBHostControllerInterface(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to IOUSBHostControllerInterface")
        }
    }
}
impl IIOUSBHostControllerInterface for IOUSBHostControllerInterface {}
pub trait IIOUSBHostControllerInterface: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn initWithCapabilities_queue_interruptRateHz_error_commandHandler_doorbellHandler_interestHandler_(
        &self,
        capabilities: NSData,
        queue: NSObject,
        interruptRateHz: NSUInteger,
        error: *mut NSError,
        commandHandler: IOUSBHostControllerInterfaceCommandHandler,
        doorbellHandler: IOUSBHostControllerInterfaceDoorbellHandler,
        interestHandler: IOServiceInterestCallback,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithCapabilities : capabilities, queue : queue, interruptRateHz : interruptRateHz, error : error, commandHandler : commandHandler, doorbellHandler : doorbellHandler, interestHandler : interestHandler)
    }
    unsafe fn destroy(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, destroy)
    }
    unsafe fn enqueueInterrupt_error_(
        &self,
        interrupt: *const IOUSBHostCIMessage,
        error: *mut NSError,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, enqueueInterrupt : interrupt, error : error)
    }
    unsafe fn enqueueInterrupt_expedite_error_(
        &self,
        interrupt: *const IOUSBHostCIMessage,
        expedite: BOOL,
        error: *mut NSError,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, enqueueInterrupt : interrupt, expedite : expedite, error : error)
    }
    unsafe fn enqueueInterrupts_count_error_(
        &self,
        interrupts: *const IOUSBHostCIMessage,
        count: NSUInteger,
        error: *mut NSError,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, enqueueInterrupts : interrupts, count : count, error : error)
    }
    unsafe fn enqueueInterrupts_count_expedite_error_(
        &self,
        interrupts: *const IOUSBHostCIMessage,
        count: NSUInteger,
        expedite: BOOL,
        error: *mut NSError,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, enqueueInterrupts : interrupts, count : count, expedite : expedite, error : error)
    }
    unsafe fn descriptionForMessage_(&self, message: *const IOUSBHostCIMessage) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, descriptionForMessage : message)
    }
    unsafe fn getPortStateMachineForCommand_error_(
        &self,
        command: *const IOUSBHostCIMessage,
        error: *mut NSError,
    ) -> IOUSBHostCIPortStateMachine
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, getPortStateMachineForCommand : command, error : error)
    }
    unsafe fn getPortStateMachineForPort_error_(
        &self,
        port: NSUInteger,
        error: *mut NSError,
    ) -> IOUSBHostCIPortStateMachine
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, getPortStateMachineForPort : port, error : error)
    }
    unsafe fn capabilitiesForPort_(&self, port: NSUInteger) -> *const IOUSBHostCIMessage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, capabilitiesForPort : port)
    }
    unsafe fn queue(&self) -> dispatch_queue_t
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, queue)
    }
    unsafe fn interruptRateHz(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, interruptRateHz)
    }
    unsafe fn setInterruptRateHz_(&self, interruptRateHz: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInterruptRateHz : interruptRateHz)
    }
    unsafe fn controllerStateMachine(&self) -> IOUSBHostCIControllerStateMachine
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, controllerStateMachine)
    }
    unsafe fn capabilities(&self) -> *const IOUSBHostCIMessage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, capabilities)
    }
    unsafe fn uuid(&self) -> NSUUID
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, uuid)
    }
}
pub type IOUSBHostControllerInterfaceCommandHandler = *mut ::std::os::raw::c_void;
pub type IOUSBHostControllerInterfaceDoorbellHandler = *mut ::std::os::raw::c_void;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct IOUSBHostCIDeviceStateMachine(pub id);
impl std::ops::Deref for IOUSBHostCIDeviceStateMachine {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for IOUSBHostCIDeviceStateMachine {}
impl IOUSBHostCIDeviceStateMachine {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"IOUSBHostCIDeviceStateMachine").unwrap(), alloc) })
    }
}
impl INSObject for IOUSBHostCIDeviceStateMachine {}
impl PNSObject for IOUSBHostCIDeviceStateMachine {}
impl std::convert::TryFrom<NSObject> for IOUSBHostCIDeviceStateMachine {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<IOUSBHostCIDeviceStateMachine, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"IOUSBHostCIDeviceStateMachine").unwrap())
        };
        if is_kind_of {
            Ok(IOUSBHostCIDeviceStateMachine(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to IOUSBHostCIDeviceStateMachine")
        }
    }
}
impl IIOUSBHostCIDeviceStateMachine for IOUSBHostCIDeviceStateMachine {}
pub trait IIOUSBHostCIDeviceStateMachine: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn initWithInterface_command_error_(
        &self,
        interface: IOUSBHostControllerInterface,
        command: *const IOUSBHostCIMessage,
        error: *mut NSError,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithInterface : interface, command : command, error : error)
    }
    unsafe fn inspectCommand_error_(
        &self,
        command: *const IOUSBHostCIMessage,
        error: *mut NSError,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, inspectCommand : command, error : error)
    }
    unsafe fn respondToCommand_status_error_(
        &self,
        command: *const IOUSBHostCIMessage,
        status: IOUSBHostCIMessageStatus,
        error: *mut NSError,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, respondToCommand : command, status : status, error : error)
    }
    unsafe fn respondToCommand_status_deviceAddress_error_(
        &self,
        command: *const IOUSBHostCIMessage,
        status: IOUSBHostCIMessageStatus,
        deviceAddress: NSUInteger,
        error: *mut NSError,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, respondToCommand : command, status : status, deviceAddress : deviceAddress, error : error)
    }
    unsafe fn deviceState(&self) -> IOUSBHostCIDeviceState
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, deviceState)
    }
    unsafe fn completeRoute(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, completeRoute)
    }
    unsafe fn deviceAddress(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, deviceAddress)
    }
    unsafe fn controllerInterface(&self) -> IOUSBHostControllerInterface
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, controllerInterface)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct IOUSBHostCIEndpointStateMachine(pub id);
impl std::ops::Deref for IOUSBHostCIEndpointStateMachine {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for IOUSBHostCIEndpointStateMachine {}
impl IOUSBHostCIEndpointStateMachine {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"IOUSBHostCIEndpointStateMachine").unwrap(), alloc) })
    }
}
impl INSObject for IOUSBHostCIEndpointStateMachine {}
impl PNSObject for IOUSBHostCIEndpointStateMachine {}
impl std::convert::TryFrom<NSObject> for IOUSBHostCIEndpointStateMachine {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<IOUSBHostCIEndpointStateMachine, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"IOUSBHostCIEndpointStateMachine").unwrap())
        };
        if is_kind_of {
            Ok(IOUSBHostCIEndpointStateMachine(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to IOUSBHostCIEndpointStateMachine")
        }
    }
}
impl IIOUSBHostCIEndpointStateMachine for IOUSBHostCIEndpointStateMachine {}
pub trait IIOUSBHostCIEndpointStateMachine: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn initWithInterface_command_error_(
        &self,
        interface: IOUSBHostControllerInterface,
        command: *const IOUSBHostCIMessage,
        error: *mut NSError,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithInterface : interface, command : command, error : error)
    }
    unsafe fn inspectCommand_error_(
        &self,
        command: *const IOUSBHostCIMessage,
        error: *mut NSError,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, inspectCommand : command, error : error)
    }
    unsafe fn respondToCommand_status_error_(
        &self,
        command: *const IOUSBHostCIMessage,
        status: IOUSBHostCIMessageStatus,
        error: *mut NSError,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, respondToCommand : command, status : status, error : error)
    }
    unsafe fn processDoorbell_error_(
        &self,
        doorbell: IOUSBHostCIDoorbell,
        error: *mut NSError,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, processDoorbell : doorbell, error : error)
    }
    unsafe fn enqueueTransferCompletionForMessage_status_transferLength_error_(
        &self,
        message: *const IOUSBHostCIMessage,
        status: IOUSBHostCIMessageStatus,
        transferLength: NSUInteger,
        error: *mut NSError,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, enqueueTransferCompletionForMessage : message, status : status, transferLength : transferLength, error : error)
    }
    unsafe fn endpointState(&self) -> IOUSBHostCIEndpointState
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, endpointState)
    }
    unsafe fn deviceAddress(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, deviceAddress)
    }
    unsafe fn endpointAddress(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, endpointAddress)
    }
    unsafe fn currentTransferMessage(&self) -> *const IOUSBHostCIMessage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, currentTransferMessage)
    }
    unsafe fn controllerInterface(&self) -> IOUSBHostControllerInterface
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, controllerInterface)
    }
}
unsafe extern "C" {
    pub static mut IOUSBHostVersionNumber: f64;
}
unsafe extern "C" {
    pub static IOUSBHostVersionString: [::std::os::raw::c_uchar; 0usize];
}
unsafe extern "C" {
    pub fn IOUSBGetNextDescriptor(
        configurationDescriptor: *const IOUSBConfigurationDescriptor,
        currentDescriptor: *const IOUSBDescriptorHeader,
    ) -> *const IOUSBDescriptorHeader;
}
unsafe extern "C" {
    pub fn IOUSBGetNextDescriptorWithType(
        configurationDescriptor: *const IOUSBConfigurationDescriptor,
        currentDescriptor: *const IOUSBDescriptorHeader,
        type_: u8,
    ) -> *const IOUSBDescriptorHeader;
}
unsafe extern "C" {
    pub fn IOUSBGetNextAssociatedDescriptor(
        configurationDescriptor: *const IOUSBConfigurationDescriptor,
        parentDescriptor: *const IOUSBDescriptorHeader,
        currentDescriptor: *const IOUSBDescriptorHeader,
    ) -> *const IOUSBDescriptorHeader;
}
unsafe extern "C" {
    pub fn IOUSBGetNextAssociatedDescriptorWithType(
        configurationDescriptor: *const IOUSBConfigurationDescriptor,
        parentDescriptor: *const IOUSBDescriptorHeader,
        currentDescriptor: *const IOUSBDescriptorHeader,
        type_: u8,
    ) -> *const IOUSBDescriptorHeader;
}
unsafe extern "C" {
    pub fn IOUSBGetNextInterfaceAssociationDescriptor(
        configurationDescriptor: *const IOUSBConfigurationDescriptor,
        currentDescriptor: *const IOUSBDescriptorHeader,
    ) -> *const IOUSBInterfaceAssociationDescriptor;
}
unsafe extern "C" {
    pub fn IOUSBGetNextInterfaceDescriptor(
        configurationDescriptor: *const IOUSBConfigurationDescriptor,
        currentDescriptor: *const IOUSBDescriptorHeader,
    ) -> *const IOUSBInterfaceDescriptor;
}
unsafe extern "C" {
    pub fn IOUSBGetNextEndpointDescriptor(
        configurationDescriptor: *const IOUSBConfigurationDescriptor,
        interfaceDescriptor: *const IOUSBInterfaceDescriptor,
        currentDescriptor: *const IOUSBDescriptorHeader,
    ) -> *const IOUSBEndpointDescriptor;
}
unsafe extern "C" {
    pub fn IOUSBGetNextCapabilityDescriptor(
        bosDescriptor: *const IOUSBBOSDescriptor,
        currentDescriptor: *const IOUSBDeviceCapabilityDescriptorHeader,
    ) -> *const IOUSBDeviceCapabilityDescriptorHeader;
}
unsafe extern "C" {
    pub fn IOUSBGetNextCapabilityDescriptorWithType(
        bosDescriptor: *const IOUSBBOSDescriptor,
        currentDescriptor: *const IOUSBDeviceCapabilityDescriptorHeader,
        type_: u8,
    ) -> *const IOUSBDeviceCapabilityDescriptorHeader;
}
unsafe extern "C" {
    pub fn IOUSBGetUSB20ExtensionDeviceCapabilityDescriptor(
        bosDescriptor: *const IOUSBBOSDescriptor,
    ) -> *const IOUSBDeviceCapabilityUSB2Extension;
}
unsafe extern "C" {
    pub fn IOUSBGetSuperSpeedDeviceCapabilityDescriptor(
        bosDescriptor: *const IOUSBBOSDescriptor,
    ) -> *const IOUSBDeviceCapabilitySuperSpeedUSB;
}
unsafe extern "C" {
    pub fn IOUSBGetSuperSpeedPlusDeviceCapabilityDescriptor(
        bosDescriptor: *const IOUSBBOSDescriptor,
    ) -> *const IOUSBDeviceCapabilitySuperSpeedPlusUSB;
}
unsafe extern "C" {
    pub fn IOUSBGetContainerIDDescriptor(
        bosDescriptor: *const IOUSBBOSDescriptor,
    ) -> *const IOUSBDeviceCapabilityContainerID;
}
unsafe extern "C" {
    pub fn IOUSBGetPlatformCapabilityDescriptor(
        bosDescriptor: *const IOUSBBOSDescriptor,
    ) -> *const IOUSBPlatformCapabilityDescriptor;
}
unsafe extern "C" {
    pub fn IOUSBGetPlatformCapabilityDescriptorWithUUID(
        bosDescriptor: *const IOUSBBOSDescriptor,
        uuid: *mut ::std::os::raw::c_uchar,
    ) -> *const IOUSBPlatformCapabilityDescriptor;
}
unsafe extern "C" {
    pub fn IOUSBGetBillboardDescriptor(
        bosDescriptor: *const IOUSBBOSDescriptor,
    ) -> *const IOUSBDeviceCapabilityBillboard;
}
unsafe extern "C" {
    pub fn IOUSBGetEndpointDirection(descriptor: *const IOUSBEndpointDescriptor) -> u8;
}
unsafe extern "C" {
    pub fn IOUSBGetEndpointAddress(descriptor: *const IOUSBEndpointDescriptor) -> u8;
}
unsafe extern "C" {
    pub fn IOUSBGetEndpointNumber(descriptor: *const IOUSBEndpointDescriptor) -> u8;
}
unsafe extern "C" {
    pub fn IOUSBGetEndpointType(descriptor: *const IOUSBEndpointDescriptor) -> u8;
}
unsafe extern "C" {
    pub fn IOUSBGetEndpointUsageType(descriptor: *const IOUSBEndpointDescriptor) -> u8;
}
unsafe extern "C" {
    pub fn IOUSBGetEndpointSynchronizationType(descriptor: *const IOUSBEndpointDescriptor) -> u8;
}
unsafe extern "C" {
    pub fn IOUSBGetEndpointMaxPacketSize(
        usbDeviceSpeed: u32,
        descriptor: *const IOUSBEndpointDescriptor,
    ) -> u16;
}
unsafe extern "C" {
    pub fn IOUSBGetEndpointBurstSize(
        usbDeviceSpeed: u32,
        descriptor: *const IOUSBEndpointDescriptor,
        companionDescriptor: *const IOUSBSuperSpeedEndpointCompanionDescriptor,
        sspCompanionDescriptor: *const IOUSBSuperSpeedPlusIsochronousEndpointCompanionDescriptor,
    ) -> u32;
}
unsafe extern "C" {
    pub fn IOUSBGetEndpointMult(
        usbDeviceSpeed: u32,
        descriptor: *const IOUSBEndpointDescriptor,
        companionDescriptor: *const IOUSBSuperSpeedEndpointCompanionDescriptor,
        sspCompanionDescriptor: *const IOUSBSuperSpeedPlusIsochronousEndpointCompanionDescriptor,
    ) -> u8;
}
unsafe extern "C" {
    pub fn IOUSBGetEndpointIntervalEncodedMicroframes(
        usbDeviceSpeed: u32,
        descriptor: *const IOUSBEndpointDescriptor,
    ) -> u32;
}
unsafe extern "C" {
    pub fn IOUSBGetEndpointIntervalMicroframes(
        usbDeviceSpeed: u32,
        descriptor: *const IOUSBEndpointDescriptor,
    ) -> u32;
}
unsafe extern "C" {
    pub fn IOUSBGetEndpointIntervalFrames(
        usbDeviceSpeed: u32,
        descriptor: *const IOUSBEndpointDescriptor,
    ) -> u32;
}
unsafe extern "C" {
    pub fn IOUSBGetEndpointMaxStreamsEncoded(
        usbDeviceSpeed: u32,
        descriptor: *const IOUSBEndpointDescriptor,
        companionDescriptor: *const IOUSBSuperSpeedEndpointCompanionDescriptor,
    ) -> u32;
}
unsafe extern "C" {
    pub fn IOUSBGetEndpointMaxStreams(
        usbDeviceSpeed: u32,
        descriptor: *const IOUSBEndpointDescriptor,
        companionDescriptor: *const IOUSBSuperSpeedEndpointCompanionDescriptor,
    ) -> u32;
}
unsafe extern "C" {
    pub fn IOUSBGetConfigurationMaxPowerMilliAmps(
        usbDeviceSpeed: u32,
        descriptor: *const IOUSBConfigurationDescriptor,
    ) -> u32;
}
unsafe extern "C" {
    pub static IOUSBHostErrorDomain: NSErrorDomain;
}
unsafe extern "C" {
    pub static mut IOUSBHostMatchingPropertyKeyVendorID: IOUSBHostMatchingPropertyKey;
}
unsafe extern "C" {
    pub static mut IOUSBHostMatchingPropertyKeyProductID: IOUSBHostMatchingPropertyKey;
}
unsafe extern "C" {
    pub static mut IOUSBHostMatchingPropertyKeyProductIDMask: IOUSBHostMatchingPropertyKey;
}
unsafe extern "C" {
    pub static mut IOUSBHostMatchingPropertyKeyProductIDArray: IOUSBHostMatchingPropertyKey;
}
unsafe extern "C" {
    pub static mut IOUSBHostMatchingPropertyKeyInterfaceNumber: IOUSBHostMatchingPropertyKey;
}
unsafe extern "C" {
    pub static mut IOUSBHostMatchingPropertyKeyConfigurationValue: IOUSBHostMatchingPropertyKey;
}
unsafe extern "C" {
    pub static mut IOUSBHostMatchingPropertyKeyDeviceReleaseNumber: IOUSBHostMatchingPropertyKey;
}
unsafe extern "C" {
    pub static mut IOUSBHostMatchingPropertyKeyInterfaceClass: IOUSBHostMatchingPropertyKey;
}
unsafe extern "C" {
    pub static mut IOUSBHostMatchingPropertyKeyInterfaceSubClass: IOUSBHostMatchingPropertyKey;
}
unsafe extern "C" {
    pub static mut IOUSBHostMatchingPropertyKeyInterfaceProtocol: IOUSBHostMatchingPropertyKey;
}
unsafe extern "C" {
    pub static mut IOUSBHostMatchingPropertyKeyDeviceClass: IOUSBHostMatchingPropertyKey;
}
unsafe extern "C" {
    pub static mut IOUSBHostMatchingPropertyKeyDeviceSubClass: IOUSBHostMatchingPropertyKey;
}
unsafe extern "C" {
    pub static mut IOUSBHostMatchingPropertyKeyDeviceProtocol: IOUSBHostMatchingPropertyKey;
}
unsafe extern "C" {
    pub static mut IOUSBHostMatchingPropertyKeySpeed: IOUSBHostMatchingPropertyKey;
}
unsafe extern "C" {
    pub static mut IOUSBHostPropertyKeyLocationID: IOUSBHostPropertyKey;
}
unsafe extern "C" {
    pub static mut IOUSBHostDevicePropertyKeyVendorString: IOUSBHostDevicePropertyKey;
}
unsafe extern "C" {
    pub static mut IOUSBHostDevicePropertyKeySerialNumberString: IOUSBHostDevicePropertyKey;
}
unsafe extern "C" {
    pub static mut IOUSBHostDevicePropertyKeyContainerID: IOUSBHostDevicePropertyKey;
}
unsafe extern "C" {
    pub static mut IOUSBHostDevicePropertyKeyCurrentConfiguration: IOUSBHostDevicePropertyKey;
}
unsafe extern "C" {
    pub static mut IOUSBHostInterfacePropertyKeyAlternateSetting: IOUSBHostInterfacePropertyKey;
}
unsafe extern "C" {
    pub static IOUSBHostDefaultControlCompletionTimeout: NSTimeInterval;
}
unsafe extern "C" {
    pub fn IOUSBHostCIMessageStatusToIOReturn(status: IOUSBHostCIMessageStatus) -> IOReturn;
}
unsafe extern "C" {
    pub fn IOUSBHostCIMessageStatusFromIOReturn(status: IOReturn) -> IOUSBHostCIMessageStatus;
}
unsafe extern "C" {
    pub fn IOUSBHostCILinkStateEnabled(linkState: IOUSBHostCILinkState) -> bool;
}
unsafe extern "C" {
    pub fn IOUSBHostCIMessageTypeToString(
        type_: IOUSBHostCIMessageType,
    ) -> *const ::std::os::raw::c_char;
}
unsafe extern "C" {
    pub fn IOUSBHostCIMessageStatusToString(
        status: IOUSBHostCIMessageStatus,
    ) -> *const ::std::os::raw::c_char;
}
unsafe extern "C" {
    pub fn IOUSBHostCILinkStateToString(
        linkState: IOUSBHostCILinkState,
    ) -> *const ::std::os::raw::c_char;
}
unsafe extern "C" {
    pub fn IOUSBHostCIDeviceSpeedToString(
        speed: IOUSBHostCIDeviceSpeed,
    ) -> *const ::std::os::raw::c_char;
}
unsafe extern "C" {
    pub fn IOUSBHostCIExceptionTypeToString(
        exceptionType: IOUSBHostCIExceptionType,
    ) -> *const ::std::os::raw::c_char;
}
unsafe extern "C" {
    pub fn IOUSBHostCIControllerStateToString(
        controllerState: IOUSBHostCIControllerState,
    ) -> *const ::std::os::raw::c_char;
}
unsafe extern "C" {
    pub fn IOUSBHostCIPortStateToString(
        portState: IOUSBHostCIPortState,
    ) -> *const ::std::os::raw::c_char;
}
unsafe extern "C" {
    pub fn IOUSBHostCIDeviceStateToString(
        deviceState: IOUSBHostCIDeviceState,
    ) -> *const ::std::os::raw::c_char;
}
unsafe extern "C" {
    pub fn IOUSBHostCIEndpointStateToString(
        endpointState: IOUSBHostCIEndpointState,
    ) -> *const ::std::os::raw::c_char;
}

unsafe impl objc2::encode::RefEncode for IOUSBDescriptorHeader {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for IOUSBDescriptorHeader {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("IOUSBDescriptorHeader", &[]);
}
unsafe impl objc2::encode::RefEncode for IOUSBDeviceDescriptor {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for IOUSBDeviceDescriptor {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("IOUSBDeviceDescriptor", &[]);
}
unsafe impl objc2::encode::RefEncode for IOUSBConfigurationDescriptor {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for IOUSBConfigurationDescriptor {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("IOUSBConfigurationDescriptor", &[]);
}
unsafe impl objc2::encode::RefEncode for IOUSBInterfaceDescriptor {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for IOUSBInterfaceDescriptor {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("IOUSBInterfaceDescriptor", &[]);
}
unsafe impl objc2::encode::RefEncode for IOUSBEndpointDescriptor {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for IOUSBEndpointDescriptor {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("IOUSBEndpointDescriptor", &[]);
}
unsafe impl objc2::encode::RefEncode for IOUSBBOSDescriptor {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for IOUSBBOSDescriptor {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("IOUSBBOSDescriptor", &[]);
}
unsafe impl objc2::encode::RefEncode for IOUSBDeviceCapabilityDescriptorHeader {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for IOUSBDeviceCapabilityDescriptorHeader {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("IOUSBDeviceCapabilityDescriptorHeader", &[]);
}
unsafe impl objc2::encode::RefEncode for IOUSBDeviceCapabilityUSB2Extension {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for IOUSBDeviceCapabilityUSB2Extension {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("IOUSBDeviceCapabilityUSB2Extension", &[]);
}
unsafe impl objc2::encode::RefEncode for IOUSBDeviceCapabilitySuperSpeedUSB {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for IOUSBDeviceCapabilitySuperSpeedUSB {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("IOUSBDeviceCapabilitySuperSpeedUSB", &[]);
}
unsafe impl objc2::encode::RefEncode for IOUSBDeviceCapabilitySuperSpeedPlusUSB {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for IOUSBDeviceCapabilitySuperSpeedPlusUSB {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("IOUSBDeviceCapabilitySuperSpeedPlusUSB", &[]);
}
unsafe impl objc2::encode::RefEncode for IOUSBDeviceCapabilityContainerID {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for IOUSBDeviceCapabilityContainerID {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("IOUSBDeviceCapabilityContainerID", &[]);
}
unsafe impl objc2::encode::RefEncode for IOUSBPlatformCapabilityDescriptor {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for IOUSBPlatformCapabilityDescriptor {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("IOUSBPlatformCapabilityDescriptor", &[]);
}
unsafe impl objc2::encode::RefEncode for IOUSBDeviceCapabilityBillboardAltConfig {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for IOUSBDeviceCapabilityBillboardAltConfig {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("IOUSBDeviceCapabilityBillboardAltConfig", &[]);
}
unsafe impl objc2::encode::RefEncode for IOUSBDeviceCapabilityBillboard {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for IOUSBDeviceCapabilityBillboard {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("IOUSBDeviceCapabilityBillboard", &[]);
}
unsafe impl objc2::encode::RefEncode for IOUSBInterfaceAssociationDescriptor {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for IOUSBInterfaceAssociationDescriptor {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("IOUSBInterfaceAssociationDescriptor", &[]);
}
unsafe impl objc2::encode::RefEncode for IOUSBSuperSpeedEndpointCompanionDescriptor {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for IOUSBSuperSpeedEndpointCompanionDescriptor {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("IOUSBSuperSpeedEndpointCompanionDescriptor", &[]);
}
unsafe impl objc2::encode::RefEncode for IOUSBSuperSpeedPlusIsochronousEndpointCompanionDescriptor {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for IOUSBSuperSpeedPlusIsochronousEndpointCompanionDescriptor {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("IOUSBSuperSpeedPlusIsochronousEndpointCompanionDescriptor", &[]);
}
unsafe impl objc2::encode::RefEncode for IOUSBDeviceRequest {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for IOUSBDeviceRequest {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("IOUSBDeviceRequest", &[]);
}
unsafe impl objc2::encode::RefEncode for IOUSBHostIsochronousFrame {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for IOUSBHostIsochronousFrame {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("IOUSBHostIsochronousFrame", &[]);
}
unsafe impl objc2::encode::RefEncode for IOUSBHostIsochronousTransaction {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for IOUSBHostIsochronousTransaction {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("IOUSBHostIsochronousTransaction", &[]);
}
unsafe impl objc2::encode::RefEncode for IOUSBHostObject {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for IOUSBHostObject {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for IOUSBHostDevice {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for IOUSBHostDevice {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for IOUSBHostInterface {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for IOUSBHostInterface {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for IOUSBHostIOSource {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for IOUSBHostIOSource {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for IOUSBHostIOSourceDescriptors {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for IOUSBHostIOSourceDescriptors {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("IOUSBHostIOSourceDescriptors", &[]);
}
unsafe impl objc2::encode::RefEncode for IOUSBHostPipe {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for IOUSBHostPipe {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for IOUSBHostStream {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for IOUSBHostStream {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for IOUSBHostCIMessage {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for IOUSBHostCIMessage {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("IOUSBHostCIMessage", &[]);
}
unsafe impl objc2::encode::RefEncode for IOUSBHostCIControllerStateMachine {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for IOUSBHostCIControllerStateMachine {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for IOUSBHostCIPortStateMachine {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for IOUSBHostCIPortStateMachine {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for IOUSBHostControllerInterface {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for IOUSBHostControllerInterface {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for IOUSBHostCIDeviceStateMachine {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for IOUSBHostCIDeviceStateMachine {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for IOUSBHostCIEndpointStateMachine {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for IOUSBHostCIEndpointStateMachine {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
