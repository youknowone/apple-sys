#[allow(unused_imports)]
use crate::objc::*;
#[allow(unused_imports)]
use crate::CoreFoundation::*;
#[allow(unused_imports)]
use crate::Foundation::*;

#[allow(unused_imports)]
use objc2::msg_send;
pub type MLCGraphCompletionHandler = *mut ::std::os::raw::c_void;
pub type MLCDataType = i32;
pub type MLCRandomInitializerType = i32;
pub type MLCDeviceType = i32;
pub type MLCGraphCompilationOptions = u64;
pub type MLCExecutionOptions = u64;
pub type MLCArithmeticOperation = i32;
pub type MLCLossType = i32;
pub type MLCActivationType = i32;
pub type MLCConvolutionType = i32;
pub type MLCPaddingPolicy = i32;
pub type MLCPaddingType = i32;
pub type MLCPoolingType = i32;
pub type MLCReductionType = i32;
pub type MLCRegularizationType = i32;
pub type MLCSampleMode = i32;
pub type MLCSoftmaxOperation = i32;
pub type MLCLSTMResultMode = u64;
pub type MLCComparisonOperation = i32;
pub type MLCGradientClippingType = i32;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MLCDevice(pub id);
impl std::ops::Deref for MLCDevice {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MLCDevice {}
impl MLCDevice {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MLCDevice").unwrap(), alloc) })
    }
}
impl PNSCopying for MLCDevice {}
impl INSObject for MLCDevice {}
impl PNSObject for MLCDevice {}
impl std::convert::TryFrom<NSObject> for MLCDevice {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<MLCDevice, Self::Error> {
        let is_kind_of: bool = unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MLCDevice").unwrap()) };
        if is_kind_of {
            Ok(MLCDevice(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to MLCDevice")
        }
    }
}
impl IMLCDevice for MLCDevice {}
pub trait IMLCDevice: Sized + std::ops::Deref {
    unsafe fn type_(&self) -> MLCDeviceType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, type)
    }
    unsafe fn actualDeviceType(&self) -> MLCDeviceType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, actualDeviceType)
    }
    unsafe fn gpuDevices(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, gpuDevices)
    }
    unsafe fn cpuDevice() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MLCDevice").unwrap(), cpuDevice)
    }
    unsafe fn gpuDevice() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MLCDevice").unwrap(), gpuDevice)
    }
    unsafe fn aneDevice() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MLCDevice").unwrap(), aneDevice)
    }
    unsafe fn deviceWithType_(type_: MLCDeviceType) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MLCDevice").unwrap(), deviceWithType : type_)
    }
    unsafe fn deviceWithType_selectsMultipleComputeDevices_(
        type_: MLCDeviceType,
        selectsMultipleComputeDevices: BOOL,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MLCDevice").unwrap(), deviceWithType : type_, selectsMultipleComputeDevices : selectsMultipleComputeDevices)
    }
    unsafe fn deviceWithGPUDevices_(gpus: NSArray) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MLCDevice").unwrap(), deviceWithGPUDevices : gpus)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MLCTensor(pub id);
impl std::ops::Deref for MLCTensor {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MLCTensor {}
impl MLCTensor {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MLCTensor").unwrap(), alloc) })
    }
}
impl PNSCopying for MLCTensor {}
impl INSObject for MLCTensor {}
impl PNSObject for MLCTensor {}
impl std::convert::TryFrom<NSObject> for MLCTensor {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<MLCTensor, Self::Error> {
        let is_kind_of: bool = unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MLCTensor").unwrap()) };
        if is_kind_of {
            Ok(MLCTensor(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to MLCTensor")
        }
    }
}
impl IMLCTensor for MLCTensor {}
pub trait IMLCTensor: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn synchronizeData(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, synchronizeData)
    }
    unsafe fn synchronizeOptimizerData(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, synchronizeOptimizerData)
    }
    unsafe fn copyDataFromDeviceMemoryToBytes_length_synchronizeWithDevice_(
        &self,
        bytes: *mut ::std::os::raw::c_void,
        length: NSUInteger,
        synchronizeWithDevice: BOOL,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, copyDataFromDeviceMemoryToBytes : bytes, length : length, synchronizeWithDevice : synchronizeWithDevice)
    }
    unsafe fn bindAndWriteData_toDevice_(&self, data: MLCTensorData, device: MLCDevice) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, bindAndWriteData : data, toDevice : device)
    }
    unsafe fn bindOptimizerData_deviceData_(&self, data: NSArray, deviceData: NSArray) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, bindOptimizerData : data, deviceData : deviceData)
    }
    unsafe fn tensorByQuantizingToType_scale_bias_(
        &self,
        type_: MLCDataType,
        scale: f32,
        bias: NSInteger,
    ) -> MLCTensor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, tensorByQuantizingToType : type_, scale : scale, bias : bias)
    }
    unsafe fn tensorByQuantizingToType_scale_bias_axis_(
        &self,
        type_: MLCDataType,
        scale: MLCTensor,
        bias: MLCTensor,
        axis: NSInteger,
    ) -> MLCTensor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, tensorByQuantizingToType : type_, scale : scale, bias : bias, axis : axis)
    }
    unsafe fn tensorByDequantizingToType_scale_bias_(
        &self,
        type_: MLCDataType,
        scale: MLCTensor,
        bias: MLCTensor,
    ) -> MLCTensor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, tensorByDequantizingToType : type_, scale : scale, bias : bias)
    }
    unsafe fn tensorByDequantizingToType_scale_bias_axis_(
        &self,
        type_: MLCDataType,
        scale: MLCTensor,
        bias: MLCTensor,
        axis: NSInteger,
    ) -> MLCTensor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, tensorByDequantizingToType : type_, scale : scale, bias : bias, axis : axis)
    }
    unsafe fn tensorID(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, tensorID)
    }
    unsafe fn descriptor(&self) -> MLCTensorDescriptor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, descriptor)
    }
    unsafe fn data(&self) -> NSData
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, data)
    }
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
    unsafe fn device(&self) -> MLCDevice
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, device)
    }
    unsafe fn optimizerData(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, optimizerData)
    }
    unsafe fn optimizerDeviceData(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, optimizerDeviceData)
    }
    unsafe fn hasValidNumerics(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, hasValidNumerics)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MLCTensor").unwrap(), new)
    }
    unsafe fn tensorWithDescriptor_(tensorDescriptor: MLCTensorDescriptor) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MLCTensor").unwrap(), tensorWithDescriptor : tensorDescriptor)
    }
    unsafe fn tensorWithDescriptor_randomInitializerType_(
        tensorDescriptor: MLCTensorDescriptor,
        randomInitializerType: MLCRandomInitializerType,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MLCTensor").unwrap(), tensorWithDescriptor : tensorDescriptor, randomInitializerType : randomInitializerType)
    }
    unsafe fn tensorWithDescriptor_fillWithData_(
        tensorDescriptor: MLCTensorDescriptor,
        fillData: NSNumber,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MLCTensor").unwrap(), tensorWithDescriptor : tensorDescriptor, fillWithData : fillData)
    }
    unsafe fn tensorWithDescriptor_data_(
        tensorDescriptor: MLCTensorDescriptor,
        data: MLCTensorData,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MLCTensor").unwrap(), tensorWithDescriptor : tensorDescriptor, data : data)
    }
    unsafe fn tensorWithShape_(shape: NSArray) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MLCTensor").unwrap(), tensorWithShape : shape)
    }
    unsafe fn tensorWithShape_randomInitializerType_(
        shape: NSArray,
        randomInitializerType: MLCRandomInitializerType,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MLCTensor").unwrap(), tensorWithShape : shape, randomInitializerType : randomInitializerType)
    }
    unsafe fn tensorWithShape_randomInitializerType_dataType_(
        shape: NSArray,
        randomInitializerType: MLCRandomInitializerType,
        dataType: MLCDataType,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MLCTensor").unwrap(), tensorWithShape : shape, randomInitializerType : randomInitializerType, dataType : dataType)
    }
    unsafe fn tensorWithShape_dataType_(shape: NSArray, dataType: MLCDataType) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MLCTensor").unwrap(), tensorWithShape : shape, dataType : dataType)
    }
    unsafe fn tensorWithShape_data_dataType_(
        shape: NSArray,
        data: MLCTensorData,
        dataType: MLCDataType,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MLCTensor").unwrap(), tensorWithShape : shape, data : data, dataType : dataType)
    }
    unsafe fn tensorWithShape_fillWithData_dataType_(
        shape: NSArray,
        fillData: NSNumber,
        dataType: MLCDataType,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MLCTensor").unwrap(), tensorWithShape : shape, fillWithData : fillData, dataType : dataType)
    }
    unsafe fn tensorWithWidth_height_featureChannelCount_batchSize_(
        width: NSUInteger,
        height: NSUInteger,
        featureChannelCount: NSUInteger,
        batchSize: NSUInteger,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MLCTensor").unwrap(), tensorWithWidth : width, height : height, featureChannelCount : featureChannelCount, batchSize : batchSize)
    }
    unsafe fn tensorWithWidth_height_featureChannelCount_batchSize_fillWithData_dataType_(
        width: NSUInteger,
        height: NSUInteger,
        featureChannelCount: NSUInteger,
        batchSize: NSUInteger,
        fillData: f32,
        dataType: MLCDataType,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MLCTensor").unwrap(), tensorWithWidth : width, height : height, featureChannelCount : featureChannelCount, batchSize : batchSize, fillWithData : fillData, dataType : dataType)
    }
    unsafe fn tensorWithWidth_height_featureChannelCount_batchSize_randomInitializerType_(
        width: NSUInteger,
        height: NSUInteger,
        featureChannelCount: NSUInteger,
        batchSize: NSUInteger,
        randomInitializerType: MLCRandomInitializerType,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MLCTensor").unwrap(), tensorWithWidth : width, height : height, featureChannelCount : featureChannelCount, batchSize : batchSize, randomInitializerType : randomInitializerType)
    }
    unsafe fn tensorWithWidth_height_featureChannelCount_batchSize_data_(
        width: NSUInteger,
        height: NSUInteger,
        featureChannelCount: NSUInteger,
        batchSize: NSUInteger,
        data: MLCTensorData,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MLCTensor").unwrap(), tensorWithWidth : width, height : height, featureChannelCount : featureChannelCount, batchSize : batchSize, data : data)
    }
    unsafe fn tensorWithWidth_height_featureChannelCount_batchSize_data_dataType_(
        width: NSUInteger,
        height: NSUInteger,
        featureChannelCount: NSUInteger,
        batchSize: NSUInteger,
        data: MLCTensorData,
        dataType: MLCDataType,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MLCTensor").unwrap(), tensorWithWidth : width, height : height, featureChannelCount : featureChannelCount, batchSize : batchSize, data : data, dataType : dataType)
    }
    unsafe fn tensorWithSequenceLength_featureChannelCount_batchSize_(
        sequenceLength: NSUInteger,
        featureChannelCount: NSUInteger,
        batchSize: NSUInteger,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MLCTensor").unwrap(), tensorWithSequenceLength : sequenceLength, featureChannelCount : featureChannelCount, batchSize : batchSize)
    }
    unsafe fn tensorWithSequenceLength_featureChannelCount_batchSize_randomInitializerType_(
        sequenceLength: NSUInteger,
        featureChannelCount: NSUInteger,
        batchSize: NSUInteger,
        randomInitializerType: MLCRandomInitializerType,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MLCTensor").unwrap(), tensorWithSequenceLength : sequenceLength, featureChannelCount : featureChannelCount, batchSize : batchSize, randomInitializerType : randomInitializerType)
    }
    unsafe fn tensorWithSequenceLength_featureChannelCount_batchSize_data_(
        sequenceLength: NSUInteger,
        featureChannelCount: NSUInteger,
        batchSize: NSUInteger,
        data: MLCTensorData,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MLCTensor").unwrap(), tensorWithSequenceLength : sequenceLength, featureChannelCount : featureChannelCount, batchSize : batchSize, data : data)
    }
    unsafe fn tensorWithSequenceLengths_sortedSequences_featureChannelCount_batchSize_randomInitializerType_(
        sequenceLengths: NSArray,
        sortedSequences: BOOL,
        featureChannelCount: NSUInteger,
        batchSize: NSUInteger,
        randomInitializerType: MLCRandomInitializerType,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MLCTensor").unwrap(), tensorWithSequenceLengths : sequenceLengths, sortedSequences : sortedSequences, featureChannelCount : featureChannelCount, batchSize : batchSize, randomInitializerType : randomInitializerType)
    }
    unsafe fn tensorWithSequenceLengths_sortedSequences_featureChannelCount_batchSize_data_(
        sequenceLengths: NSArray,
        sortedSequences: BOOL,
        featureChannelCount: NSUInteger,
        batchSize: NSUInteger,
        data: MLCTensorData,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MLCTensor").unwrap(), tensorWithSequenceLengths : sequenceLengths, sortedSequences : sortedSequences, featureChannelCount : featureChannelCount, batchSize : batchSize, data : data)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MLCTensorData(pub id);
impl std::ops::Deref for MLCTensorData {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MLCTensorData {}
impl MLCTensorData {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MLCTensorData").unwrap(), alloc) })
    }
}
impl INSObject for MLCTensorData {}
impl PNSObject for MLCTensorData {}
impl std::convert::TryFrom<NSObject> for MLCTensorData {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<MLCTensorData, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MLCTensorData").unwrap()) };
        if is_kind_of {
            Ok(MLCTensorData(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to MLCTensorData")
        }
    }
}
impl IMLCTensorData for MLCTensorData {}
pub trait IMLCTensorData: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn bytes(&self) -> *mut ::std::os::raw::c_void
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, bytes)
    }
    unsafe fn length(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, length)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MLCTensorData").unwrap(), new)
    }
    unsafe fn dataWithBytesNoCopy_length_(
        bytes: *mut ::std::os::raw::c_void,
        length: NSUInteger,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MLCTensorData").unwrap(), dataWithBytesNoCopy : bytes, length : length)
    }
    unsafe fn dataWithImmutableBytesNoCopy_length_(
        bytes: *const ::std::os::raw::c_void,
        length: NSUInteger,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MLCTensorData").unwrap(), dataWithImmutableBytesNoCopy : bytes, length : length)
    }
    unsafe fn dataWithBytesNoCopy_length_deallocator_(
        bytes: *mut ::std::os::raw::c_void,
        length: NSUInteger,
        deallocator: *mut ::std::os::raw::c_void,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MLCTensorData").unwrap(), dataWithBytesNoCopy : bytes, length : length, deallocator : deallocator)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MLCTensorParameter(pub id);
impl std::ops::Deref for MLCTensorParameter {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MLCTensorParameter {}
impl MLCTensorParameter {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MLCTensorParameter").unwrap(), alloc) })
    }
}
impl INSObject for MLCTensorParameter {}
impl PNSObject for MLCTensorParameter {}
impl std::convert::TryFrom<NSObject> for MLCTensorParameter {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<MLCTensorParameter, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MLCTensorParameter").unwrap()) };
        if is_kind_of {
            Ok(MLCTensorParameter(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to MLCTensorParameter")
        }
    }
}
impl IMLCTensorParameter for MLCTensorParameter {}
pub trait IMLCTensorParameter: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn tensor(&self) -> MLCTensor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, tensor)
    }
    unsafe fn isUpdatable(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isUpdatable)
    }
    unsafe fn setIsUpdatable_(&self, isUpdatable: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setIsUpdatable : isUpdatable)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MLCTensorParameter").unwrap(), new)
    }
    unsafe fn parameterWithTensor_(tensor: MLCTensor) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MLCTensorParameter").unwrap(), parameterWithTensor : tensor)
    }
    unsafe fn parameterWithTensor_optimizerData_(
        tensor: MLCTensor,
        optimizerData: NSArray,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MLCTensorParameter").unwrap(), parameterWithTensor : tensor, optimizerData : optimizerData)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MLCTensorOptimizerDeviceData(pub id);
impl std::ops::Deref for MLCTensorOptimizerDeviceData {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MLCTensorOptimizerDeviceData {}
impl MLCTensorOptimizerDeviceData {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MLCTensorOptimizerDeviceData").unwrap(), alloc) })
    }
}
impl PNSCopying for MLCTensorOptimizerDeviceData {}
impl INSObject for MLCTensorOptimizerDeviceData {}
impl PNSObject for MLCTensorOptimizerDeviceData {}
impl std::convert::TryFrom<NSObject> for MLCTensorOptimizerDeviceData {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<MLCTensorOptimizerDeviceData, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MLCTensorOptimizerDeviceData").unwrap()) };
        if is_kind_of {
            Ok(MLCTensorOptimizerDeviceData(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to MLCTensorOptimizerDeviceData")
        }
    }
}
impl IMLCTensorOptimizerDeviceData for MLCTensorOptimizerDeviceData {}
pub trait IMLCTensorOptimizerDeviceData: Sized + std::ops::Deref {
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MLCTensorOptimizerDeviceData").unwrap(), new)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MLCGraph(pub id);
impl std::ops::Deref for MLCGraph {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MLCGraph {}
impl MLCGraph {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MLCGraph").unwrap(), alloc) })
    }
}
impl INSObject for MLCGraph {}
impl PNSObject for MLCGraph {}
impl std::convert::TryFrom<NSObject> for MLCGraph {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<MLCGraph, Self::Error> {
        let is_kind_of: bool = unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MLCGraph").unwrap()) };
        if is_kind_of {
            Ok(MLCGraph(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to MLCGraph")
        }
    }
}
impl IMLCGraph for MLCGraph {}
pub trait IMLCGraph: Sized + std::ops::Deref {
    unsafe fn nodeWithLayer_source_(&self, layer: MLCLayer, source: MLCTensor) -> MLCTensor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, nodeWithLayer : layer, source : source)
    }
    unsafe fn nodeWithLayer_sources_(&self, layer: MLCLayer, sources: NSArray) -> MLCTensor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, nodeWithLayer : layer, sources : sources)
    }
    unsafe fn nodeWithLayer_sources_disableUpdate_(
        &self,
        layer: MLCLayer,
        sources: NSArray,
        disableUpdate: BOOL,
    ) -> MLCTensor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, nodeWithLayer : layer, sources : sources, disableUpdate : disableUpdate)
    }
    unsafe fn nodeWithLayer_sources_lossLabels_(
        &self,
        layer: MLCLayer,
        sources: NSArray,
        lossLabels: NSArray,
    ) -> MLCTensor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, nodeWithLayer : layer, sources : sources, lossLabels : lossLabels)
    }
    unsafe fn splitWithSource_splitCount_dimension_(
        &self,
        source: MLCTensor,
        splitCount: NSUInteger,
        dimension: NSUInteger,
    ) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, splitWithSource : source, splitCount : splitCount, dimension : dimension)
    }
    unsafe fn splitWithSource_splitSectionLengths_dimension_(
        &self,
        source: MLCTensor,
        splitSectionLengths: NSArray,
        dimension: NSUInteger,
    ) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, splitWithSource : source, splitSectionLengths : splitSectionLengths, dimension : dimension)
    }
    unsafe fn concatenateWithSources_dimension_(
        &self,
        sources: NSArray,
        dimension: NSUInteger,
    ) -> MLCTensor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, concatenateWithSources : sources, dimension : dimension)
    }
    unsafe fn reshapeWithShape_source_(&self, shape: NSArray, source: MLCTensor) -> MLCTensor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, reshapeWithShape : shape, source : source)
    }
    unsafe fn transposeWithDimensions_source_(
        &self,
        dimensions: NSArray,
        source: MLCTensor,
    ) -> MLCTensor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, transposeWithDimensions : dimensions, source : source)
    }
    unsafe fn selectWithSources_condition_(
        &self,
        sources: NSArray,
        condition: MLCTensor,
    ) -> MLCTensor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, selectWithSources : sources, condition : condition)
    }
    unsafe fn scatterWithDimension_source_indices_copyFrom_reductionType_(
        &self,
        dimension: NSUInteger,
        source: MLCTensor,
        indices: MLCTensor,
        copyFrom: MLCTensor,
        reductionType: MLCReductionType,
    ) -> MLCTensor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, scatterWithDimension : dimension, source : source, indices : indices, copyFrom : copyFrom, reductionType : reductionType)
    }
    unsafe fn gatherWithDimension_source_indices_(
        &self,
        dimension: NSUInteger,
        source: MLCTensor,
        indices: MLCTensor,
    ) -> MLCTensor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, gatherWithDimension : dimension, source : source, indices : indices)
    }
    unsafe fn bindAndWriteData_forInputs_toDevice_batchSize_synchronous_(
        &self,
        inputsData: NSDictionary,
        inputTensors: NSDictionary,
        device: MLCDevice,
        batchSize: NSUInteger,
        synchronous: BOOL,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, bindAndWriteData : inputsData, forInputs : inputTensors, toDevice : device, batchSize : batchSize, synchronous : synchronous)
    }
    unsafe fn bindAndWriteData_forInputs_toDevice_synchronous_(
        &self,
        inputsData: NSDictionary,
        inputTensors: NSDictionary,
        device: MLCDevice,
        synchronous: BOOL,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, bindAndWriteData : inputsData, forInputs : inputTensors, toDevice : device, synchronous : synchronous)
    }
    unsafe fn sourceTensorsForLayer_(&self, layer: MLCLayer) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, sourceTensorsForLayer : layer)
    }
    unsafe fn resultTensorsForLayer_(&self, layer: MLCLayer) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, resultTensorsForLayer : layer)
    }
    unsafe fn device(&self) -> MLCDevice
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, device)
    }
    unsafe fn layers(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, layers)
    }
    unsafe fn summarizedDOTDescription(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, summarizedDOTDescription)
    }
    unsafe fn graph() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MLCGraph").unwrap(), graph)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MLCTrainingGraph(pub id);
impl std::ops::Deref for MLCTrainingGraph {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MLCTrainingGraph {}
impl MLCTrainingGraph {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MLCTrainingGraph").unwrap(), alloc) })
    }
}
impl IMLCGraph for MLCTrainingGraph {}
impl From<MLCTrainingGraph> for MLCGraph {
    fn from(child: MLCTrainingGraph) -> MLCGraph {
        MLCGraph(child.0)
    }
}
impl std::convert::TryFrom<MLCGraph> for MLCTrainingGraph {
    type Error = &'static str;
    fn try_from(parent: MLCGraph) -> Result<MLCTrainingGraph, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MLCTrainingGraph").unwrap()) };
        if is_kind_of {
            Ok(MLCTrainingGraph(parent.0))
        } else {
            Err("This MLCGraph cannot be downcasted to MLCTrainingGraph")
        }
    }
}
impl INSObject for MLCTrainingGraph {}
impl PNSObject for MLCTrainingGraph {}
impl IMLCTrainingGraph for MLCTrainingGraph {}
pub trait IMLCTrainingGraph: Sized + std::ops::Deref {
    unsafe fn addInputs_lossLabels_(&self, inputs: NSDictionary, lossLabels: NSDictionary) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, addInputs : inputs, lossLabels : lossLabels)
    }
    unsafe fn addInputs_lossLabels_lossLabelWeights_(
        &self,
        inputs: NSDictionary,
        lossLabels: NSDictionary,
        lossLabelWeights: NSDictionary,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, addInputs : inputs, lossLabels : lossLabels, lossLabelWeights : lossLabelWeights)
    }
    unsafe fn addOutputs_(&self, outputs: NSDictionary) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, addOutputs : outputs)
    }
    unsafe fn stopGradientForTensors_(&self, tensors: NSArray) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, stopGradientForTensors : tensors)
    }
    unsafe fn compileWithOptions_device_(
        &self,
        options: MLCGraphCompilationOptions,
        device: MLCDevice,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, compileWithOptions : options, device : device)
    }
    unsafe fn compileWithOptions_device_inputTensors_inputTensorsData_(
        &self,
        options: MLCGraphCompilationOptions,
        device: MLCDevice,
        inputTensors: NSDictionary,
        inputTensorsData: NSDictionary,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, compileWithOptions : options, device : device, inputTensors : inputTensors, inputTensorsData : inputTensorsData)
    }
    unsafe fn compileOptimizer_(&self, optimizer: MLCOptimizer) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, compileOptimizer : optimizer)
    }
    unsafe fn linkWithGraphs_(&self, graphs: NSArray) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, linkWithGraphs : graphs)
    }
    unsafe fn gradientTensorForInput_(&self, input: MLCTensor) -> MLCTensor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, gradientTensorForInput : input)
    }
    unsafe fn sourceGradientTensorsForLayer_(&self, layer: MLCLayer) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, sourceGradientTensorsForLayer : layer)
    }
    unsafe fn resultGradientTensorsForLayer_(&self, layer: MLCLayer) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, resultGradientTensorsForLayer : layer)
    }
    unsafe fn gradientDataForParameter_layer_(
        &self,
        parameter: MLCTensor,
        layer: MLCLayer,
    ) -> NSData
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, gradientDataForParameter : parameter, layer : layer)
    }
    unsafe fn allocateUserGradientForTensor_(&self, tensor: MLCTensor) -> MLCTensor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, allocateUserGradientForTensor : tensor)
    }
    unsafe fn executeWithInputsData_lossLabelsData_lossLabelWeightsData_batchSize_options_completionHandler_(
        &self,
        inputsData: NSDictionary,
        lossLabelsData: NSDictionary,
        lossLabelWeightsData: NSDictionary,
        batchSize: NSUInteger,
        options: MLCExecutionOptions,
        completionHandler: MLCGraphCompletionHandler,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, executeWithInputsData : inputsData, lossLabelsData : lossLabelsData, lossLabelWeightsData : lossLabelWeightsData, batchSize : batchSize, options : options, completionHandler : completionHandler)
    }
    unsafe fn executeWithInputsData_lossLabelsData_lossLabelWeightsData_outputsData_batchSize_options_completionHandler_(
        &self,
        inputsData: NSDictionary,
        lossLabelsData: NSDictionary,
        lossLabelWeightsData: NSDictionary,
        outputsData: NSDictionary,
        batchSize: NSUInteger,
        options: MLCExecutionOptions,
        completionHandler: MLCGraphCompletionHandler,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, executeWithInputsData : inputsData, lossLabelsData : lossLabelsData, lossLabelWeightsData : lossLabelWeightsData, outputsData : outputsData, batchSize : batchSize, options : options, completionHandler : completionHandler)
    }
    unsafe fn executeForwardWithBatchSize_options_completionHandler_(
        &self,
        batchSize: NSUInteger,
        options: MLCExecutionOptions,
        completionHandler: MLCGraphCompletionHandler,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, executeForwardWithBatchSize : batchSize, options : options, completionHandler : completionHandler)
    }
    unsafe fn executeForwardWithBatchSize_options_outputsData_completionHandler_(
        &self,
        batchSize: NSUInteger,
        options: MLCExecutionOptions,
        outputsData: NSDictionary,
        completionHandler: MLCGraphCompletionHandler,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, executeForwardWithBatchSize : batchSize, options : options, outputsData : outputsData, completionHandler : completionHandler)
    }
    unsafe fn executeGradientWithBatchSize_options_completionHandler_(
        &self,
        batchSize: NSUInteger,
        options: MLCExecutionOptions,
        completionHandler: MLCGraphCompletionHandler,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, executeGradientWithBatchSize : batchSize, options : options, completionHandler : completionHandler)
    }
    unsafe fn executeGradientWithBatchSize_options_outputsData_completionHandler_(
        &self,
        batchSize: NSUInteger,
        options: MLCExecutionOptions,
        outputsData: NSDictionary,
        completionHandler: MLCGraphCompletionHandler,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, executeGradientWithBatchSize : batchSize, options : options, outputsData : outputsData, completionHandler : completionHandler)
    }
    unsafe fn executeOptimizerUpdateWithOptions_completionHandler_(
        &self,
        options: MLCExecutionOptions,
        completionHandler: MLCGraphCompletionHandler,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, executeOptimizerUpdateWithOptions : options, completionHandler : completionHandler)
    }
    unsafe fn synchronizeUpdates(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, synchronizeUpdates)
    }
    unsafe fn setTrainingTensorParameters_(&self, parameters: NSArray) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setTrainingTensorParameters : parameters)
    }
    unsafe fn bindOptimizerData_deviceData_withTensor_(
        &self,
        data: NSArray,
        deviceData: NSArray,
        tensor: MLCTensor,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, bindOptimizerData : data, deviceData : deviceData, withTensor : tensor)
    }
    unsafe fn optimizer(&self) -> MLCOptimizer
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, optimizer)
    }
    unsafe fn deviceMemorySize(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, deviceMemorySize)
    }
    unsafe fn graphWithGraphObjects_lossLayer_optimizer_(
        graphObjects: NSArray,
        lossLayer: MLCLayer,
        optimizer: MLCOptimizer,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MLCTrainingGraph").unwrap(), graphWithGraphObjects : graphObjects, lossLayer : lossLayer, optimizer : optimizer)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MLCInferenceGraph(pub id);
impl std::ops::Deref for MLCInferenceGraph {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MLCInferenceGraph {}
impl MLCInferenceGraph {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MLCInferenceGraph").unwrap(), alloc) })
    }
}
impl IMLCGraph for MLCInferenceGraph {}
impl std::convert::TryFrom<MLCGraph> for MLCInferenceGraph {
    type Error = &'static str;
    fn try_from(parent: MLCGraph) -> Result<MLCInferenceGraph, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MLCInferenceGraph").unwrap()) };
        if is_kind_of {
            Ok(MLCInferenceGraph(parent.0))
        } else {
            Err("This MLCGraph cannot be downcasted to MLCInferenceGraph")
        }
    }
}
impl INSObject for MLCInferenceGraph {}
impl PNSObject for MLCInferenceGraph {}
impl IMLCInferenceGraph for MLCInferenceGraph {}
pub trait IMLCInferenceGraph: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn addInputs_(&self, inputs: NSDictionary) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, addInputs : inputs)
    }
    unsafe fn addInputs_lossLabels_lossLabelWeights_(
        &self,
        inputs: NSDictionary,
        lossLabels: NSDictionary,
        lossLabelWeights: NSDictionary,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, addInputs : inputs, lossLabels : lossLabels, lossLabelWeights : lossLabelWeights)
    }
    unsafe fn addOutputs_(&self, outputs: NSDictionary) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, addOutputs : outputs)
    }
    unsafe fn compileWithOptions_device_(
        &self,
        options: MLCGraphCompilationOptions,
        device: MLCDevice,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, compileWithOptions : options, device : device)
    }
    unsafe fn compileWithOptions_device_inputTensors_inputTensorsData_(
        &self,
        options: MLCGraphCompilationOptions,
        device: MLCDevice,
        inputTensors: NSDictionary,
        inputTensorsData: NSDictionary,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, compileWithOptions : options, device : device, inputTensors : inputTensors, inputTensorsData : inputTensorsData)
    }
    unsafe fn linkWithGraphs_(&self, graphs: NSArray) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, linkWithGraphs : graphs)
    }
    unsafe fn executeWithInputsData_batchSize_options_completionHandler_(
        &self,
        inputsData: NSDictionary,
        batchSize: NSUInteger,
        options: MLCExecutionOptions,
        completionHandler: MLCGraphCompletionHandler,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, executeWithInputsData : inputsData, batchSize : batchSize, options : options, completionHandler : completionHandler)
    }
    unsafe fn executeWithInputsData_outputsData_batchSize_options_completionHandler_(
        &self,
        inputsData: NSDictionary,
        outputsData: NSDictionary,
        batchSize: NSUInteger,
        options: MLCExecutionOptions,
        completionHandler: MLCGraphCompletionHandler,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, executeWithInputsData : inputsData, outputsData : outputsData, batchSize : batchSize, options : options, completionHandler : completionHandler)
    }
    unsafe fn executeWithInputsData_lossLabelsData_lossLabelWeightsData_batchSize_options_completionHandler_(
        &self,
        inputsData: NSDictionary,
        lossLabelsData: NSDictionary,
        lossLabelWeightsData: NSDictionary,
        batchSize: NSUInteger,
        options: MLCExecutionOptions,
        completionHandler: MLCGraphCompletionHandler,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, executeWithInputsData : inputsData, lossLabelsData : lossLabelsData, lossLabelWeightsData : lossLabelWeightsData, batchSize : batchSize, options : options, completionHandler : completionHandler)
    }
    unsafe fn executeWithInputsData_lossLabelsData_lossLabelWeightsData_outputsData_batchSize_options_completionHandler_(
        &self,
        inputsData: NSDictionary,
        lossLabelsData: NSDictionary,
        lossLabelWeightsData: NSDictionary,
        outputsData: NSDictionary,
        batchSize: NSUInteger,
        options: MLCExecutionOptions,
        completionHandler: MLCGraphCompletionHandler,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, executeWithInputsData : inputsData, lossLabelsData : lossLabelsData, lossLabelWeightsData : lossLabelWeightsData, outputsData : outputsData, batchSize : batchSize, options : options, completionHandler : completionHandler)
    }
    unsafe fn deviceMemorySize(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, deviceMemorySize)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MLCInferenceGraph").unwrap(), new)
    }
    unsafe fn graphWithGraphObjects_(graphObjects: NSArray) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MLCInferenceGraph").unwrap(), graphWithGraphObjects : graphObjects)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MLCPlatform(pub id);
impl std::ops::Deref for MLCPlatform {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MLCPlatform {}
impl MLCPlatform {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MLCPlatform").unwrap(), alloc) })
    }
}
impl INSObject for MLCPlatform {}
impl PNSObject for MLCPlatform {}
impl std::convert::TryFrom<NSObject> for MLCPlatform {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<MLCPlatform, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MLCPlatform").unwrap()) };
        if is_kind_of {
            Ok(MLCPlatform(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to MLCPlatform")
        }
    }
}
impl IMLCPlatform for MLCPlatform {}
pub trait IMLCPlatform: Sized + std::ops::Deref {
    unsafe fn setRNGSeedTo_(seed: NSNumber)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MLCPlatform").unwrap(), setRNGSeedTo : seed)
    }
    unsafe fn getRNGseed() -> NSNumber
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MLCPlatform").unwrap(), getRNGseed)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MLCActivationDescriptor(pub id);
impl std::ops::Deref for MLCActivationDescriptor {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MLCActivationDescriptor {}
impl MLCActivationDescriptor {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MLCActivationDescriptor").unwrap(), alloc) })
    }
}
impl PNSCopying for MLCActivationDescriptor {}
impl INSObject for MLCActivationDescriptor {}
impl PNSObject for MLCActivationDescriptor {}
impl std::convert::TryFrom<NSObject> for MLCActivationDescriptor {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<MLCActivationDescriptor, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MLCActivationDescriptor").unwrap()) };
        if is_kind_of {
            Ok(MLCActivationDescriptor(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to MLCActivationDescriptor")
        }
    }
}
impl IMLCActivationDescriptor for MLCActivationDescriptor {}
pub trait IMLCActivationDescriptor: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn activationType(&self) -> MLCActivationType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, activationType)
    }
    unsafe fn a(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, a)
    }
    unsafe fn b(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, b)
    }
    unsafe fn c(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, c)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MLCActivationDescriptor").unwrap(), new)
    }
    unsafe fn descriptorWithType_(activationType: MLCActivationType) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MLCActivationDescriptor").unwrap(), descriptorWithType : activationType)
    }
    unsafe fn descriptorWithType_a_(activationType: MLCActivationType, a: f32) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MLCActivationDescriptor").unwrap(), descriptorWithType : activationType, a : a)
    }
    unsafe fn descriptorWithType_a_b_(
        activationType: MLCActivationType,
        a: f32,
        b: f32,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MLCActivationDescriptor").unwrap(), descriptorWithType : activationType, a : a, b : b)
    }
    unsafe fn descriptorWithType_a_b_c_(
        activationType: MLCActivationType,
        a: f32,
        b: f32,
        c: f32,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MLCActivationDescriptor").unwrap(), descriptorWithType : activationType, a : a, b : b, c : c)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MLCPoolingDescriptor(pub id);
impl std::ops::Deref for MLCPoolingDescriptor {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MLCPoolingDescriptor {}
impl MLCPoolingDescriptor {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MLCPoolingDescriptor").unwrap(), alloc) })
    }
}
impl PNSCopying for MLCPoolingDescriptor {}
impl INSObject for MLCPoolingDescriptor {}
impl PNSObject for MLCPoolingDescriptor {}
impl std::convert::TryFrom<NSObject> for MLCPoolingDescriptor {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<MLCPoolingDescriptor, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MLCPoolingDescriptor").unwrap()) };
        if is_kind_of {
            Ok(MLCPoolingDescriptor(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to MLCPoolingDescriptor")
        }
    }
}
impl IMLCPoolingDescriptor for MLCPoolingDescriptor {}
pub trait IMLCPoolingDescriptor: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn poolingType(&self) -> MLCPoolingType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, poolingType)
    }
    unsafe fn kernelWidth(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, kernelWidth)
    }
    unsafe fn kernelHeight(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, kernelHeight)
    }
    unsafe fn strideInX(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, strideInX)
    }
    unsafe fn strideInY(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, strideInY)
    }
    unsafe fn dilationRateInX(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, dilationRateInX)
    }
    unsafe fn dilationRateInY(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, dilationRateInY)
    }
    unsafe fn paddingPolicy(&self) -> MLCPaddingPolicy
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, paddingPolicy)
    }
    unsafe fn paddingSizeInX(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, paddingSizeInX)
    }
    unsafe fn paddingSizeInY(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, paddingSizeInY)
    }
    unsafe fn countIncludesPadding(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, countIncludesPadding)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MLCPoolingDescriptor").unwrap(), new)
    }
    unsafe fn poolingDescriptorWithType_kernelSize_stride_(
        poolingType: MLCPoolingType,
        kernelSize: NSUInteger,
        stride: NSUInteger,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MLCPoolingDescriptor").unwrap(), poolingDescriptorWithType : poolingType, kernelSize : kernelSize, stride : stride)
    }
    unsafe fn maxPoolingDescriptorWithKernelSizes_strides_paddingPolicy_paddingSizes_(
        kernelSizes: NSArray,
        strides: NSArray,
        paddingPolicy: MLCPaddingPolicy,
        paddingSizes: NSArray,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MLCPoolingDescriptor").unwrap(), maxPoolingDescriptorWithKernelSizes : kernelSizes, strides : strides, paddingPolicy : paddingPolicy, paddingSizes : paddingSizes)
    }
    unsafe fn maxPoolingDescriptorWithKernelSizes_strides_dilationRates_paddingPolicy_paddingSizes_(
        kernelSizes: NSArray,
        strides: NSArray,
        dilationRates: NSArray,
        paddingPolicy: MLCPaddingPolicy,
        paddingSizes: NSArray,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MLCPoolingDescriptor").unwrap(), maxPoolingDescriptorWithKernelSizes : kernelSizes, strides : strides, dilationRates : dilationRates, paddingPolicy : paddingPolicy, paddingSizes : paddingSizes)
    }
    unsafe fn averagePoolingDescriptorWithKernelSizes_strides_paddingPolicy_paddingSizes_countIncludesPadding_(
        kernelSizes: NSArray,
        strides: NSArray,
        paddingPolicy: MLCPaddingPolicy,
        paddingSizes: NSArray,
        countIncludesPadding: BOOL,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MLCPoolingDescriptor").unwrap(), averagePoolingDescriptorWithKernelSizes : kernelSizes, strides : strides, paddingPolicy : paddingPolicy, paddingSizes : paddingSizes, countIncludesPadding : countIncludesPadding)
    }
    unsafe fn averagePoolingDescriptorWithKernelSizes_strides_dilationRates_paddingPolicy_paddingSizes_countIncludesPadding_(
        kernelSizes: NSArray,
        strides: NSArray,
        dilationRates: NSArray,
        paddingPolicy: MLCPaddingPolicy,
        paddingSizes: NSArray,
        countIncludesPadding: BOOL,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MLCPoolingDescriptor").unwrap(), averagePoolingDescriptorWithKernelSizes : kernelSizes, strides : strides, dilationRates : dilationRates, paddingPolicy : paddingPolicy, paddingSizes : paddingSizes, countIncludesPadding : countIncludesPadding)
    }
    unsafe fn l2NormPoolingDescriptorWithKernelSizes_strides_paddingPolicy_paddingSizes_(
        kernelSizes: NSArray,
        strides: NSArray,
        paddingPolicy: MLCPaddingPolicy,
        paddingSizes: NSArray,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MLCPoolingDescriptor").unwrap(), l2NormPoolingDescriptorWithKernelSizes : kernelSizes, strides : strides, paddingPolicy : paddingPolicy, paddingSizes : paddingSizes)
    }
    unsafe fn l2NormPoolingDescriptorWithKernelSizes_strides_dilationRates_paddingPolicy_paddingSizes_(
        kernelSizes: NSArray,
        strides: NSArray,
        dilationRates: NSArray,
        paddingPolicy: MLCPaddingPolicy,
        paddingSizes: NSArray,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MLCPoolingDescriptor").unwrap(), l2NormPoolingDescriptorWithKernelSizes : kernelSizes, strides : strides, dilationRates : dilationRates, paddingPolicy : paddingPolicy, paddingSizes : paddingSizes)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MLCConvolutionDescriptor(pub id);
impl std::ops::Deref for MLCConvolutionDescriptor {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MLCConvolutionDescriptor {}
impl MLCConvolutionDescriptor {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MLCConvolutionDescriptor").unwrap(), alloc) })
    }
}
impl PNSCopying for MLCConvolutionDescriptor {}
impl INSObject for MLCConvolutionDescriptor {}
impl PNSObject for MLCConvolutionDescriptor {}
impl std::convert::TryFrom<NSObject> for MLCConvolutionDescriptor {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<MLCConvolutionDescriptor, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MLCConvolutionDescriptor").unwrap()) };
        if is_kind_of {
            Ok(MLCConvolutionDescriptor(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to MLCConvolutionDescriptor")
        }
    }
}
impl IMLCConvolutionDescriptor for MLCConvolutionDescriptor {}
pub trait IMLCConvolutionDescriptor: Sized + std::ops::Deref {
    unsafe fn convolutionType(&self) -> MLCConvolutionType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, convolutionType)
    }
    unsafe fn kernelWidth(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, kernelWidth)
    }
    unsafe fn kernelHeight(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, kernelHeight)
    }
    unsafe fn inputFeatureChannelCount(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inputFeatureChannelCount)
    }
    unsafe fn outputFeatureChannelCount(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, outputFeatureChannelCount)
    }
    unsafe fn strideInX(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, strideInX)
    }
    unsafe fn strideInY(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, strideInY)
    }
    unsafe fn dilationRateInX(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, dilationRateInX)
    }
    unsafe fn dilationRateInY(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, dilationRateInY)
    }
    unsafe fn groupCount(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, groupCount)
    }
    unsafe fn paddingPolicy(&self) -> MLCPaddingPolicy
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, paddingPolicy)
    }
    unsafe fn paddingSizeInX(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, paddingSizeInX)
    }
    unsafe fn paddingSizeInY(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, paddingSizeInY)
    }
    unsafe fn isConvolutionTranspose(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isConvolutionTranspose)
    }
    unsafe fn usesDepthwiseConvolution(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, usesDepthwiseConvolution)
    }
    unsafe fn descriptorWithType_kernelSizes_inputFeatureChannelCount_outputFeatureChannelCount_groupCount_strides_dilationRates_paddingPolicy_paddingSizes_(
        convolutionType: MLCConvolutionType,
        kernelSizes: NSArray,
        inputFeatureChannelCount: NSUInteger,
        outputFeatureChannelCount: NSUInteger,
        groupCount: NSUInteger,
        strides: NSArray,
        dilationRates: NSArray,
        paddingPolicy: MLCPaddingPolicy,
        paddingSizes: NSArray,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MLCConvolutionDescriptor").unwrap(), descriptorWithType : convolutionType, kernelSizes : kernelSizes, inputFeatureChannelCount : inputFeatureChannelCount, outputFeatureChannelCount : outputFeatureChannelCount, groupCount : groupCount, strides : strides, dilationRates : dilationRates, paddingPolicy : paddingPolicy, paddingSizes : paddingSizes)
    }
    unsafe fn descriptorWithKernelWidth_kernelHeight_inputFeatureChannelCount_outputFeatureChannelCount_(
        kernelWidth: NSUInteger,
        kernelHeight: NSUInteger,
        inputFeatureChannelCount: NSUInteger,
        outputFeatureChannelCount: NSUInteger,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MLCConvolutionDescriptor").unwrap(), descriptorWithKernelWidth : kernelWidth, kernelHeight : kernelHeight, inputFeatureChannelCount : inputFeatureChannelCount, outputFeatureChannelCount : outputFeatureChannelCount)
    }
    unsafe fn descriptorWithKernelSizes_inputFeatureChannelCount_outputFeatureChannelCount_strides_paddingPolicy_paddingSizes_(
        kernelSizes: NSArray,
        inputFeatureChannelCount: NSUInteger,
        outputFeatureChannelCount: NSUInteger,
        strides: NSArray,
        paddingPolicy: MLCPaddingPolicy,
        paddingSizes: NSArray,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MLCConvolutionDescriptor").unwrap(), descriptorWithKernelSizes : kernelSizes, inputFeatureChannelCount : inputFeatureChannelCount, outputFeatureChannelCount : outputFeatureChannelCount, strides : strides, paddingPolicy : paddingPolicy, paddingSizes : paddingSizes)
    }
    unsafe fn descriptorWithKernelSizes_inputFeatureChannelCount_outputFeatureChannelCount_groupCount_strides_dilationRates_paddingPolicy_paddingSizes_(
        kernelSizes: NSArray,
        inputFeatureChannelCount: NSUInteger,
        outputFeatureChannelCount: NSUInteger,
        groupCount: NSUInteger,
        strides: NSArray,
        dilationRates: NSArray,
        paddingPolicy: MLCPaddingPolicy,
        paddingSizes: NSArray,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MLCConvolutionDescriptor").unwrap(), descriptorWithKernelSizes : kernelSizes, inputFeatureChannelCount : inputFeatureChannelCount, outputFeatureChannelCount : outputFeatureChannelCount, groupCount : groupCount, strides : strides, dilationRates : dilationRates, paddingPolicy : paddingPolicy, paddingSizes : paddingSizes)
    }
    unsafe fn convolutionTransposeDescriptorWithKernelWidth_kernelHeight_inputFeatureChannelCount_outputFeatureChannelCount_(
        kernelWidth: NSUInteger,
        kernelHeight: NSUInteger,
        inputFeatureChannelCount: NSUInteger,
        outputFeatureChannelCount: NSUInteger,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MLCConvolutionDescriptor").unwrap(), convolutionTransposeDescriptorWithKernelWidth : kernelWidth, kernelHeight : kernelHeight, inputFeatureChannelCount : inputFeatureChannelCount, outputFeatureChannelCount : outputFeatureChannelCount)
    }
    unsafe fn convolutionTransposeDescriptorWithKernelSizes_inputFeatureChannelCount_outputFeatureChannelCount_strides_paddingPolicy_paddingSizes_(
        kernelSizes: NSArray,
        inputFeatureChannelCount: NSUInteger,
        outputFeatureChannelCount: NSUInteger,
        strides: NSArray,
        paddingPolicy: MLCPaddingPolicy,
        paddingSizes: NSArray,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MLCConvolutionDescriptor").unwrap(), convolutionTransposeDescriptorWithKernelSizes : kernelSizes, inputFeatureChannelCount : inputFeatureChannelCount, outputFeatureChannelCount : outputFeatureChannelCount, strides : strides, paddingPolicy : paddingPolicy, paddingSizes : paddingSizes)
    }
    unsafe fn convolutionTransposeDescriptorWithKernelSizes_inputFeatureChannelCount_outputFeatureChannelCount_groupCount_strides_dilationRates_paddingPolicy_paddingSizes_(
        kernelSizes: NSArray,
        inputFeatureChannelCount: NSUInteger,
        outputFeatureChannelCount: NSUInteger,
        groupCount: NSUInteger,
        strides: NSArray,
        dilationRates: NSArray,
        paddingPolicy: MLCPaddingPolicy,
        paddingSizes: NSArray,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MLCConvolutionDescriptor").unwrap(), convolutionTransposeDescriptorWithKernelSizes : kernelSizes, inputFeatureChannelCount : inputFeatureChannelCount, outputFeatureChannelCount : outputFeatureChannelCount, groupCount : groupCount, strides : strides, dilationRates : dilationRates, paddingPolicy : paddingPolicy, paddingSizes : paddingSizes)
    }
    unsafe fn depthwiseConvolutionDescriptorWithKernelWidth_kernelHeight_inputFeatureChannelCount_channelMultiplier_(
        kernelWidth: NSUInteger,
        kernelHeight: NSUInteger,
        inputFeatureChannelCount: NSUInteger,
        channelMultiplier: NSUInteger,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MLCConvolutionDescriptor").unwrap(), depthwiseConvolutionDescriptorWithKernelWidth : kernelWidth, kernelHeight : kernelHeight, inputFeatureChannelCount : inputFeatureChannelCount, channelMultiplier : channelMultiplier)
    }
    unsafe fn depthwiseConvolutionDescriptorWithKernelSizes_inputFeatureChannelCount_channelMultiplier_strides_paddingPolicy_paddingSizes_(
        kernelSizes: NSArray,
        inputFeatureChannelCount: NSUInteger,
        channelMultiplier: NSUInteger,
        strides: NSArray,
        paddingPolicy: MLCPaddingPolicy,
        paddingSizes: NSArray,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MLCConvolutionDescriptor").unwrap(), depthwiseConvolutionDescriptorWithKernelSizes : kernelSizes, inputFeatureChannelCount : inputFeatureChannelCount, channelMultiplier : channelMultiplier, strides : strides, paddingPolicy : paddingPolicy, paddingSizes : paddingSizes)
    }
    unsafe fn depthwiseConvolutionDescriptorWithKernelSizes_inputFeatureChannelCount_channelMultiplier_strides_dilationRates_paddingPolicy_paddingSizes_(
        kernelSizes: NSArray,
        inputFeatureChannelCount: NSUInteger,
        channelMultiplier: NSUInteger,
        strides: NSArray,
        dilationRates: NSArray,
        paddingPolicy: MLCPaddingPolicy,
        paddingSizes: NSArray,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MLCConvolutionDescriptor").unwrap(), depthwiseConvolutionDescriptorWithKernelSizes : kernelSizes, inputFeatureChannelCount : inputFeatureChannelCount, channelMultiplier : channelMultiplier, strides : strides, dilationRates : dilationRates, paddingPolicy : paddingPolicy, paddingSizes : paddingSizes)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MLCLossDescriptor(pub id);
impl std::ops::Deref for MLCLossDescriptor {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MLCLossDescriptor {}
impl MLCLossDescriptor {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MLCLossDescriptor").unwrap(), alloc) })
    }
}
impl PNSCopying for MLCLossDescriptor {}
impl INSObject for MLCLossDescriptor {}
impl PNSObject for MLCLossDescriptor {}
impl std::convert::TryFrom<NSObject> for MLCLossDescriptor {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<MLCLossDescriptor, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MLCLossDescriptor").unwrap()) };
        if is_kind_of {
            Ok(MLCLossDescriptor(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to MLCLossDescriptor")
        }
    }
}
impl IMLCLossDescriptor for MLCLossDescriptor {}
pub trait IMLCLossDescriptor: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn lossType(&self) -> MLCLossType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, lossType)
    }
    unsafe fn reductionType(&self) -> MLCReductionType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, reductionType)
    }
    unsafe fn weight(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, weight)
    }
    unsafe fn labelSmoothing(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, labelSmoothing)
    }
    unsafe fn classCount(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, classCount)
    }
    unsafe fn epsilon(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, epsilon)
    }
    unsafe fn delta(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, delta)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MLCLossDescriptor").unwrap(), new)
    }
    unsafe fn descriptorWithType_reductionType_(
        lossType: MLCLossType,
        reductionType: MLCReductionType,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MLCLossDescriptor").unwrap(), descriptorWithType : lossType, reductionType : reductionType)
    }
    unsafe fn descriptorWithType_reductionType_weight_(
        lossType: MLCLossType,
        reductionType: MLCReductionType,
        weight: f32,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MLCLossDescriptor").unwrap(), descriptorWithType : lossType, reductionType : reductionType, weight : weight)
    }
    unsafe fn descriptorWithType_reductionType_weight_labelSmoothing_classCount_(
        lossType: MLCLossType,
        reductionType: MLCReductionType,
        weight: f32,
        labelSmoothing: f32,
        classCount: NSUInteger,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MLCLossDescriptor").unwrap(), descriptorWithType : lossType, reductionType : reductionType, weight : weight, labelSmoothing : labelSmoothing, classCount : classCount)
    }
    unsafe fn descriptorWithType_reductionType_weight_labelSmoothing_classCount_epsilon_delta_(
        lossType: MLCLossType,
        reductionType: MLCReductionType,
        weight: f32,
        labelSmoothing: f32,
        classCount: NSUInteger,
        epsilon: f32,
        delta: f32,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MLCLossDescriptor").unwrap(), descriptorWithType : lossType, reductionType : reductionType, weight : weight, labelSmoothing : labelSmoothing, classCount : classCount, epsilon : epsilon, delta : delta)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MLCYOLOLossDescriptor(pub id);
impl std::ops::Deref for MLCYOLOLossDescriptor {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MLCYOLOLossDescriptor {}
impl MLCYOLOLossDescriptor {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MLCYOLOLossDescriptor").unwrap(), alloc) })
    }
}
impl PNSCopying for MLCYOLOLossDescriptor {}
impl INSObject for MLCYOLOLossDescriptor {}
impl PNSObject for MLCYOLOLossDescriptor {}
impl std::convert::TryFrom<NSObject> for MLCYOLOLossDescriptor {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<MLCYOLOLossDescriptor, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MLCYOLOLossDescriptor").unwrap()) };
        if is_kind_of {
            Ok(MLCYOLOLossDescriptor(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to MLCYOLOLossDescriptor")
        }
    }
}
impl IMLCYOLOLossDescriptor for MLCYOLOLossDescriptor {}
pub trait IMLCYOLOLossDescriptor: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn anchorBoxCount(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, anchorBoxCount)
    }
    unsafe fn anchorBoxes(&self) -> NSData
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, anchorBoxes)
    }
    unsafe fn shouldRescore(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, shouldRescore)
    }
    unsafe fn setShouldRescore_(&self, shouldRescore: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setShouldRescore : shouldRescore)
    }
    unsafe fn scaleSpatialPositionLoss(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, scaleSpatialPositionLoss)
    }
    unsafe fn setScaleSpatialPositionLoss_(&self, scaleSpatialPositionLoss: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setScaleSpatialPositionLoss : scaleSpatialPositionLoss)
    }
    unsafe fn scaleSpatialSizeLoss(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, scaleSpatialSizeLoss)
    }
    unsafe fn setScaleSpatialSizeLoss_(&self, scaleSpatialSizeLoss: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setScaleSpatialSizeLoss : scaleSpatialSizeLoss)
    }
    unsafe fn scaleNoObjectConfidenceLoss(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, scaleNoObjectConfidenceLoss)
    }
    unsafe fn setScaleNoObjectConfidenceLoss_(&self, scaleNoObjectConfidenceLoss: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setScaleNoObjectConfidenceLoss : scaleNoObjectConfidenceLoss)
    }
    unsafe fn scaleObjectConfidenceLoss(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, scaleObjectConfidenceLoss)
    }
    unsafe fn setScaleObjectConfidenceLoss_(&self, scaleObjectConfidenceLoss: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setScaleObjectConfidenceLoss : scaleObjectConfidenceLoss)
    }
    unsafe fn scaleClassLoss(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, scaleClassLoss)
    }
    unsafe fn setScaleClassLoss_(&self, scaleClassLoss: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setScaleClassLoss : scaleClassLoss)
    }
    unsafe fn minimumIOUForObjectPresence(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, minimumIOUForObjectPresence)
    }
    unsafe fn setMinimumIOUForObjectPresence_(&self, minimumIOUForObjectPresence: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMinimumIOUForObjectPresence : minimumIOUForObjectPresence)
    }
    unsafe fn maximumIOUForObjectAbsence(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, maximumIOUForObjectAbsence)
    }
    unsafe fn setMaximumIOUForObjectAbsence_(&self, maximumIOUForObjectAbsence: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMaximumIOUForObjectAbsence : maximumIOUForObjectAbsence)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MLCYOLOLossDescriptor").unwrap(), new)
    }
    unsafe fn descriptorWithAnchorBoxes_anchorBoxCount_(
        anchorBoxes: NSData,
        anchorBoxCount: NSUInteger,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MLCYOLOLossDescriptor").unwrap(), descriptorWithAnchorBoxes : anchorBoxes, anchorBoxCount : anchorBoxCount)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MLCLSTMDescriptor(pub id);
impl std::ops::Deref for MLCLSTMDescriptor {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MLCLSTMDescriptor {}
impl MLCLSTMDescriptor {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MLCLSTMDescriptor").unwrap(), alloc) })
    }
}
impl PNSCopying for MLCLSTMDescriptor {}
impl INSObject for MLCLSTMDescriptor {}
impl PNSObject for MLCLSTMDescriptor {}
impl std::convert::TryFrom<NSObject> for MLCLSTMDescriptor {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<MLCLSTMDescriptor, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MLCLSTMDescriptor").unwrap()) };
        if is_kind_of {
            Ok(MLCLSTMDescriptor(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to MLCLSTMDescriptor")
        }
    }
}
impl IMLCLSTMDescriptor for MLCLSTMDescriptor {}
pub trait IMLCLSTMDescriptor: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn inputSize(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inputSize)
    }
    unsafe fn hiddenSize(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, hiddenSize)
    }
    unsafe fn layerCount(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, layerCount)
    }
    unsafe fn usesBiases(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, usesBiases)
    }
    unsafe fn batchFirst(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, batchFirst)
    }
    unsafe fn isBidirectional(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isBidirectional)
    }
    unsafe fn returnsSequences(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, returnsSequences)
    }
    unsafe fn dropout(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, dropout)
    }
    unsafe fn resultMode(&self) -> MLCLSTMResultMode
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, resultMode)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MLCLSTMDescriptor").unwrap(), new)
    }
    unsafe fn descriptorWithInputSize_hiddenSize_layerCount_(
        inputSize: NSUInteger,
        hiddenSize: NSUInteger,
        layerCount: NSUInteger,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MLCLSTMDescriptor").unwrap(), descriptorWithInputSize : inputSize, hiddenSize : hiddenSize, layerCount : layerCount)
    }
    unsafe fn descriptorWithInputSize_hiddenSize_layerCount_usesBiases_isBidirectional_dropout_(
        inputSize: NSUInteger,
        hiddenSize: NSUInteger,
        layerCount: NSUInteger,
        usesBiases: BOOL,
        isBidirectional: BOOL,
        dropout: f32,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MLCLSTMDescriptor").unwrap(), descriptorWithInputSize : inputSize, hiddenSize : hiddenSize, layerCount : layerCount, usesBiases : usesBiases, isBidirectional : isBidirectional, dropout : dropout)
    }
    unsafe fn descriptorWithInputSize_hiddenSize_layerCount_usesBiases_batchFirst_isBidirectional_dropout_(
        inputSize: NSUInteger,
        hiddenSize: NSUInteger,
        layerCount: NSUInteger,
        usesBiases: BOOL,
        batchFirst: BOOL,
        isBidirectional: BOOL,
        dropout: f32,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MLCLSTMDescriptor").unwrap(), descriptorWithInputSize : inputSize, hiddenSize : hiddenSize, layerCount : layerCount, usesBiases : usesBiases, batchFirst : batchFirst, isBidirectional : isBidirectional, dropout : dropout)
    }
    unsafe fn descriptorWithInputSize_hiddenSize_layerCount_usesBiases_batchFirst_isBidirectional_returnsSequences_dropout_(
        inputSize: NSUInteger,
        hiddenSize: NSUInteger,
        layerCount: NSUInteger,
        usesBiases: BOOL,
        batchFirst: BOOL,
        isBidirectional: BOOL,
        returnsSequences: BOOL,
        dropout: f32,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MLCLSTMDescriptor").unwrap(), descriptorWithInputSize : inputSize, hiddenSize : hiddenSize, layerCount : layerCount, usesBiases : usesBiases, batchFirst : batchFirst, isBidirectional : isBidirectional, returnsSequences : returnsSequences, dropout : dropout)
    }
    unsafe fn descriptorWithInputSize_hiddenSize_layerCount_usesBiases_batchFirst_isBidirectional_returnsSequences_dropout_resultMode_(
        inputSize: NSUInteger,
        hiddenSize: NSUInteger,
        layerCount: NSUInteger,
        usesBiases: BOOL,
        batchFirst: BOOL,
        isBidirectional: BOOL,
        returnsSequences: BOOL,
        dropout: f32,
        resultMode: MLCLSTMResultMode,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MLCLSTMDescriptor").unwrap(), descriptorWithInputSize : inputSize, hiddenSize : hiddenSize, layerCount : layerCount, usesBiases : usesBiases, batchFirst : batchFirst, isBidirectional : isBidirectional, returnsSequences : returnsSequences, dropout : dropout, resultMode : resultMode)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MLCTensorDescriptor(pub id);
impl std::ops::Deref for MLCTensorDescriptor {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MLCTensorDescriptor {}
impl MLCTensorDescriptor {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MLCTensorDescriptor").unwrap(), alloc) })
    }
}
impl PNSCopying for MLCTensorDescriptor {}
impl INSObject for MLCTensorDescriptor {}
impl PNSObject for MLCTensorDescriptor {}
impl std::convert::TryFrom<NSObject> for MLCTensorDescriptor {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<MLCTensorDescriptor, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MLCTensorDescriptor").unwrap()) };
        if is_kind_of {
            Ok(MLCTensorDescriptor(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to MLCTensorDescriptor")
        }
    }
}
impl IMLCTensorDescriptor for MLCTensorDescriptor {}
pub trait IMLCTensorDescriptor: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn dataType(&self) -> MLCDataType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, dataType)
    }
    unsafe fn dimensionCount(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, dimensionCount)
    }
    unsafe fn shape(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, shape)
    }
    unsafe fn stride(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, stride)
    }
    unsafe fn tensorAllocationSizeInBytes(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, tensorAllocationSizeInBytes)
    }
    unsafe fn sequenceLengths(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, sequenceLengths)
    }
    unsafe fn sortedSequences(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, sortedSequences)
    }
    unsafe fn batchSizePerSequenceStep(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, batchSizePerSequenceStep)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MLCTensorDescriptor").unwrap(), new)
    }
    unsafe fn descriptorWithShape_dataType_(shape: NSArray, dataType: MLCDataType) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MLCTensorDescriptor").unwrap(), descriptorWithShape : shape, dataType : dataType)
    }
    unsafe fn descriptorWithShape_sequenceLengths_sortedSequences_dataType_(
        shape: NSArray,
        sequenceLengths: NSArray,
        sortedSequences: BOOL,
        dataType: MLCDataType,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MLCTensorDescriptor").unwrap(), descriptorWithShape : shape, sequenceLengths : sequenceLengths, sortedSequences : sortedSequences, dataType : dataType)
    }
    unsafe fn descriptorWithWidth_height_featureChannelCount_batchSize_(
        width: NSUInteger,
        height: NSUInteger,
        featureChannels: NSUInteger,
        batchSize: NSUInteger,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MLCTensorDescriptor").unwrap(), descriptorWithWidth : width, height : height, featureChannelCount : featureChannels, batchSize : batchSize)
    }
    unsafe fn descriptorWithWidth_height_featureChannelCount_batchSize_dataType_(
        width: NSUInteger,
        height: NSUInteger,
        featureChannelCount: NSUInteger,
        batchSize: NSUInteger,
        dataType: MLCDataType,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MLCTensorDescriptor").unwrap(), descriptorWithWidth : width, height : height, featureChannelCount : featureChannelCount, batchSize : batchSize, dataType : dataType)
    }
    unsafe fn convolutionWeightsDescriptorWithWidth_height_inputFeatureChannelCount_outputFeatureChannelCount_dataType_(
        width: NSUInteger,
        height: NSUInteger,
        inputFeatureChannelCount: NSUInteger,
        outputFeatureChannelCount: NSUInteger,
        dataType: MLCDataType,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MLCTensorDescriptor").unwrap(), convolutionWeightsDescriptorWithWidth : width, height : height, inputFeatureChannelCount : inputFeatureChannelCount, outputFeatureChannelCount : outputFeatureChannelCount, dataType : dataType)
    }
    unsafe fn convolutionWeightsDescriptorWithInputFeatureChannelCount_outputFeatureChannelCount_dataType_(
        inputFeatureChannelCount: NSUInteger,
        outputFeatureChannelCount: NSUInteger,
        dataType: MLCDataType,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MLCTensorDescriptor").unwrap(), convolutionWeightsDescriptorWithInputFeatureChannelCount : inputFeatureChannelCount, outputFeatureChannelCount : outputFeatureChannelCount, dataType : dataType)
    }
    unsafe fn convolutionBiasesDescriptorWithFeatureChannelCount_dataType_(
        featureChannelCount: NSUInteger,
        dataType: MLCDataType,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MLCTensorDescriptor").unwrap(), convolutionBiasesDescriptorWithFeatureChannelCount : featureChannelCount, dataType : dataType)
    }
    unsafe fn maxTensorDimensions() -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MLCTensorDescriptor").unwrap(), maxTensorDimensions)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MLCOptimizerDescriptor(pub id);
impl std::ops::Deref for MLCOptimizerDescriptor {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MLCOptimizerDescriptor {}
impl MLCOptimizerDescriptor {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MLCOptimizerDescriptor").unwrap(), alloc) })
    }
}
impl PNSCopying for MLCOptimizerDescriptor {}
impl INSObject for MLCOptimizerDescriptor {}
impl PNSObject for MLCOptimizerDescriptor {}
impl std::convert::TryFrom<NSObject> for MLCOptimizerDescriptor {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<MLCOptimizerDescriptor, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MLCOptimizerDescriptor").unwrap()) };
        if is_kind_of {
            Ok(MLCOptimizerDescriptor(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to MLCOptimizerDescriptor")
        }
    }
}
impl IMLCOptimizerDescriptor for MLCOptimizerDescriptor {}
pub trait IMLCOptimizerDescriptor: Sized + std::ops::Deref {
    unsafe fn learningRate(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, learningRate)
    }
    unsafe fn gradientRescale(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, gradientRescale)
    }
    unsafe fn appliesGradientClipping(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, appliesGradientClipping)
    }
    unsafe fn gradientClipMax(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, gradientClipMax)
    }
    unsafe fn gradientClipMin(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, gradientClipMin)
    }
    unsafe fn regularizationScale(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, regularizationScale)
    }
    unsafe fn regularizationType(&self) -> MLCRegularizationType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, regularizationType)
    }
    unsafe fn gradientClippingType(&self) -> MLCGradientClippingType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, gradientClippingType)
    }
    unsafe fn maximumClippingNorm(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, maximumClippingNorm)
    }
    unsafe fn customGlobalNorm(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, customGlobalNorm)
    }
    unsafe fn descriptorWithLearningRate_gradientRescale_regularizationType_regularizationScale_(
        learningRate: f32,
        gradientRescale: f32,
        regularizationType: MLCRegularizationType,
        regularizationScale: f32,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MLCOptimizerDescriptor").unwrap(), descriptorWithLearningRate : learningRate, gradientRescale : gradientRescale, regularizationType : regularizationType, regularizationScale : regularizationScale)
    }
    unsafe fn descriptorWithLearningRate_gradientRescale_appliesGradientClipping_gradientClipMax_gradientClipMin_regularizationType_regularizationScale_(
        learningRate: f32,
        gradientRescale: f32,
        appliesGradientClipping: BOOL,
        gradientClipMax: f32,
        gradientClipMin: f32,
        regularizationType: MLCRegularizationType,
        regularizationScale: f32,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MLCOptimizerDescriptor").unwrap(), descriptorWithLearningRate : learningRate, gradientRescale : gradientRescale, appliesGradientClipping : appliesGradientClipping, gradientClipMax : gradientClipMax, gradientClipMin : gradientClipMin, regularizationType : regularizationType, regularizationScale : regularizationScale)
    }
    unsafe fn descriptorWithLearningRate_gradientRescale_appliesGradientClipping_gradientClippingType_gradientClipMax_gradientClipMin_maximumClippingNorm_customGlobalNorm_regularizationType_regularizationScale_(
        learningRate: f32,
        gradientRescale: f32,
        appliesGradientClipping: BOOL,
        gradientClippingType: MLCGradientClippingType,
        gradientClipMax: f32,
        gradientClipMin: f32,
        maximumClippingNorm: f32,
        customGlobalNorm: f32,
        regularizationType: MLCRegularizationType,
        regularizationScale: f32,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MLCOptimizerDescriptor").unwrap(), descriptorWithLearningRate : learningRate, gradientRescale : gradientRescale, appliesGradientClipping : appliesGradientClipping, gradientClippingType : gradientClippingType, gradientClipMax : gradientClipMax, gradientClipMin : gradientClipMin, maximumClippingNorm : maximumClippingNorm, customGlobalNorm : customGlobalNorm, regularizationType : regularizationType, regularizationScale : regularizationScale)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MLCMatMulDescriptor(pub id);
impl std::ops::Deref for MLCMatMulDescriptor {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MLCMatMulDescriptor {}
impl MLCMatMulDescriptor {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MLCMatMulDescriptor").unwrap(), alloc) })
    }
}
impl PNSCopying for MLCMatMulDescriptor {}
impl INSObject for MLCMatMulDescriptor {}
impl PNSObject for MLCMatMulDescriptor {}
impl std::convert::TryFrom<NSObject> for MLCMatMulDescriptor {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<MLCMatMulDescriptor, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MLCMatMulDescriptor").unwrap()) };
        if is_kind_of {
            Ok(MLCMatMulDescriptor(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to MLCMatMulDescriptor")
        }
    }
}
impl IMLCMatMulDescriptor for MLCMatMulDescriptor {}
pub trait IMLCMatMulDescriptor: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn alpha(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, alpha)
    }
    unsafe fn transposesX(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, transposesX)
    }
    unsafe fn transposesY(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, transposesY)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MLCMatMulDescriptor").unwrap(), new)
    }
    unsafe fn descriptorWithAlpha_transposesX_transposesY_(
        alpha: f32,
        transposesX: BOOL,
        transposesY: BOOL,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MLCMatMulDescriptor").unwrap(), descriptorWithAlpha : alpha, transposesX : transposesX, transposesY : transposesY)
    }
    unsafe fn descriptor() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MLCMatMulDescriptor").unwrap(), descriptor)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MLCMultiheadAttentionDescriptor(pub id);
impl std::ops::Deref for MLCMultiheadAttentionDescriptor {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MLCMultiheadAttentionDescriptor {}
impl MLCMultiheadAttentionDescriptor {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MLCMultiheadAttentionDescriptor").unwrap(), alloc) })
    }
}
impl PNSCopying for MLCMultiheadAttentionDescriptor {}
impl INSObject for MLCMultiheadAttentionDescriptor {}
impl PNSObject for MLCMultiheadAttentionDescriptor {}
impl std::convert::TryFrom<NSObject> for MLCMultiheadAttentionDescriptor {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<MLCMultiheadAttentionDescriptor, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MLCMultiheadAttentionDescriptor").unwrap())
        };
        if is_kind_of {
            Ok(MLCMultiheadAttentionDescriptor(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to MLCMultiheadAttentionDescriptor")
        }
    }
}
impl IMLCMultiheadAttentionDescriptor for MLCMultiheadAttentionDescriptor {}
pub trait IMLCMultiheadAttentionDescriptor: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn modelDimension(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, modelDimension)
    }
    unsafe fn keyDimension(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, keyDimension)
    }
    unsafe fn valueDimension(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, valueDimension)
    }
    unsafe fn headCount(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, headCount)
    }
    unsafe fn dropout(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, dropout)
    }
    unsafe fn hasBiases(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, hasBiases)
    }
    unsafe fn hasAttentionBiases(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, hasAttentionBiases)
    }
    unsafe fn addsZeroAttention(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, addsZeroAttention)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MLCMultiheadAttentionDescriptor").unwrap(), new)
    }
    unsafe fn descriptorWithModelDimension_keyDimension_valueDimension_headCount_dropout_hasBiases_hasAttentionBiases_addsZeroAttention_(
        modelDimension: NSUInteger,
        keyDimension: NSUInteger,
        valueDimension: NSUInteger,
        headCount: NSUInteger,
        dropout: f32,
        hasBiases: BOOL,
        hasAttentionBiases: BOOL,
        addsZeroAttention: BOOL,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MLCMultiheadAttentionDescriptor").unwrap(), descriptorWithModelDimension : modelDimension, keyDimension : keyDimension, valueDimension : valueDimension, headCount : headCount, dropout : dropout, hasBiases : hasBiases, hasAttentionBiases : hasAttentionBiases, addsZeroAttention : addsZeroAttention)
    }
    unsafe fn descriptorWithModelDimension_headCount_(
        modelDimension: NSUInteger,
        headCount: NSUInteger,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MLCMultiheadAttentionDescriptor").unwrap(), descriptorWithModelDimension : modelDimension, headCount : headCount)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MLCEmbeddingDescriptor(pub id);
impl std::ops::Deref for MLCEmbeddingDescriptor {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MLCEmbeddingDescriptor {}
impl MLCEmbeddingDescriptor {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MLCEmbeddingDescriptor").unwrap(), alloc) })
    }
}
impl PNSCopying for MLCEmbeddingDescriptor {}
impl INSObject for MLCEmbeddingDescriptor {}
impl PNSObject for MLCEmbeddingDescriptor {}
impl std::convert::TryFrom<NSObject> for MLCEmbeddingDescriptor {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<MLCEmbeddingDescriptor, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MLCEmbeddingDescriptor").unwrap()) };
        if is_kind_of {
            Ok(MLCEmbeddingDescriptor(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to MLCEmbeddingDescriptor")
        }
    }
}
impl IMLCEmbeddingDescriptor for MLCEmbeddingDescriptor {}
pub trait IMLCEmbeddingDescriptor: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn embeddingCount(&self) -> NSNumber
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, embeddingCount)
    }
    unsafe fn embeddingDimension(&self) -> NSNumber
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, embeddingDimension)
    }
    unsafe fn paddingIndex(&self) -> NSNumber
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, paddingIndex)
    }
    unsafe fn maximumNorm(&self) -> NSNumber
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, maximumNorm)
    }
    unsafe fn pNorm(&self) -> NSNumber
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, pNorm)
    }
    unsafe fn scalesGradientByFrequency(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, scalesGradientByFrequency)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MLCEmbeddingDescriptor").unwrap(), new)
    }
    unsafe fn descriptorWithEmbeddingCount_embeddingDimension_(
        embeddingCount: NSNumber,
        embeddingDimension: NSNumber,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MLCEmbeddingDescriptor").unwrap(), descriptorWithEmbeddingCount : embeddingCount, embeddingDimension : embeddingDimension)
    }
    unsafe fn descriptorWithEmbeddingCount_embeddingDimension_paddingIndex_maximumNorm_pNorm_scalesGradientByFrequency_(
        embeddingCount: NSNumber,
        embeddingDimension: NSNumber,
        paddingIndex: NSNumber,
        maximumNorm: NSNumber,
        pNorm: NSNumber,
        scalesGradientByFrequency: BOOL,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MLCEmbeddingDescriptor").unwrap(), descriptorWithEmbeddingCount : embeddingCount, embeddingDimension : embeddingDimension, paddingIndex : paddingIndex, maximumNorm : maximumNorm, pNorm : pNorm, scalesGradientByFrequency : scalesGradientByFrequency)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MLCLayer(pub id);
impl std::ops::Deref for MLCLayer {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MLCLayer {}
impl MLCLayer {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MLCLayer").unwrap(), alloc) })
    }
}
impl INSObject for MLCLayer {}
impl PNSObject for MLCLayer {}
impl std::convert::TryFrom<NSObject> for MLCLayer {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<MLCLayer, Self::Error> {
        let is_kind_of: bool = unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MLCLayer").unwrap()) };
        if is_kind_of {
            Ok(MLCLayer(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to MLCLayer")
        }
    }
}
impl IMLCLayer for MLCLayer {}
pub trait IMLCLayer: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn layerID(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, layerID)
    }
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
    unsafe fn isDebuggingEnabled(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isDebuggingEnabled)
    }
    unsafe fn setIsDebuggingEnabled_(&self, isDebuggingEnabled: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setIsDebuggingEnabled : isDebuggingEnabled)
    }
    unsafe fn deviceType(&self) -> MLCDeviceType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, deviceType)
    }
    unsafe fn supportsDataType_onDevice_(dataType: MLCDataType, device: MLCDevice) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MLCLayer").unwrap(), supportsDataType : dataType, onDevice : device)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MLCLayer").unwrap(), new)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MLCArithmeticLayer(pub id);
impl std::ops::Deref for MLCArithmeticLayer {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MLCArithmeticLayer {}
impl MLCArithmeticLayer {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MLCArithmeticLayer").unwrap(), alloc) })
    }
}
impl IMLCLayer for MLCArithmeticLayer {}
impl From<MLCArithmeticLayer> for MLCLayer {
    fn from(child: MLCArithmeticLayer) -> MLCLayer {
        MLCLayer(child.0)
    }
}
impl std::convert::TryFrom<MLCLayer> for MLCArithmeticLayer {
    type Error = &'static str;
    fn try_from(parent: MLCLayer) -> Result<MLCArithmeticLayer, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MLCArithmeticLayer").unwrap()) };
        if is_kind_of {
            Ok(MLCArithmeticLayer(parent.0))
        } else {
            Err("This MLCLayer cannot be downcasted to MLCArithmeticLayer")
        }
    }
}
impl INSObject for MLCArithmeticLayer {}
impl PNSObject for MLCArithmeticLayer {}
impl IMLCArithmeticLayer for MLCArithmeticLayer {}
pub trait IMLCArithmeticLayer: Sized + std::ops::Deref {
    unsafe fn operation(&self) -> MLCArithmeticOperation
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, operation)
    }
    unsafe fn layerWithOperation_(operation: MLCArithmeticOperation) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MLCArithmeticLayer").unwrap(), layerWithOperation : operation)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MLCComparisonLayer(pub id);
impl std::ops::Deref for MLCComparisonLayer {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MLCComparisonLayer {}
impl MLCComparisonLayer {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MLCComparisonLayer").unwrap(), alloc) })
    }
}
impl IMLCLayer for MLCComparisonLayer {}
impl std::convert::TryFrom<MLCLayer> for MLCComparisonLayer {
    type Error = &'static str;
    fn try_from(parent: MLCLayer) -> Result<MLCComparisonLayer, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MLCComparisonLayer").unwrap()) };
        if is_kind_of {
            Ok(MLCComparisonLayer(parent.0))
        } else {
            Err("This MLCLayer cannot be downcasted to MLCComparisonLayer")
        }
    }
}
impl INSObject for MLCComparisonLayer {}
impl PNSObject for MLCComparisonLayer {}
impl IMLCComparisonLayer for MLCComparisonLayer {}
pub trait IMLCComparisonLayer: Sized + std::ops::Deref {
    unsafe fn operation(&self) -> MLCComparisonOperation
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, operation)
    }
    unsafe fn layerWithOperation_(operation: MLCComparisonOperation) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MLCComparisonLayer").unwrap(), layerWithOperation : operation)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MLCConvolutionLayer(pub id);
impl std::ops::Deref for MLCConvolutionLayer {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MLCConvolutionLayer {}
impl MLCConvolutionLayer {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MLCConvolutionLayer").unwrap(), alloc) })
    }
}
impl IMLCLayer for MLCConvolutionLayer {}
impl std::convert::TryFrom<MLCLayer> for MLCConvolutionLayer {
    type Error = &'static str;
    fn try_from(parent: MLCLayer) -> Result<MLCConvolutionLayer, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MLCConvolutionLayer").unwrap()) };
        if is_kind_of {
            Ok(MLCConvolutionLayer(parent.0))
        } else {
            Err("This MLCLayer cannot be downcasted to MLCConvolutionLayer")
        }
    }
}
impl INSObject for MLCConvolutionLayer {}
impl PNSObject for MLCConvolutionLayer {}
impl IMLCConvolutionLayer for MLCConvolutionLayer {}
pub trait IMLCConvolutionLayer: Sized + std::ops::Deref {
    unsafe fn descriptor(&self) -> MLCConvolutionDescriptor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, descriptor)
    }
    unsafe fn weights(&self) -> MLCTensor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, weights)
    }
    unsafe fn biases(&self) -> MLCTensor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, biases)
    }
    unsafe fn weightsParameter(&self) -> MLCTensorParameter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, weightsParameter)
    }
    unsafe fn biasesParameter(&self) -> MLCTensorParameter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, biasesParameter)
    }
    unsafe fn layerWithWeights_biases_descriptor_(
        weights: MLCTensor,
        biases: MLCTensor,
        descriptor: MLCConvolutionDescriptor,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MLCConvolutionLayer").unwrap(), layerWithWeights : weights, biases : biases, descriptor : descriptor)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MLCFullyConnectedLayer(pub id);
impl std::ops::Deref for MLCFullyConnectedLayer {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MLCFullyConnectedLayer {}
impl MLCFullyConnectedLayer {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MLCFullyConnectedLayer").unwrap(), alloc) })
    }
}
impl IMLCLayer for MLCFullyConnectedLayer {}
impl std::convert::TryFrom<MLCLayer> for MLCFullyConnectedLayer {
    type Error = &'static str;
    fn try_from(parent: MLCLayer) -> Result<MLCFullyConnectedLayer, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MLCFullyConnectedLayer").unwrap()) };
        if is_kind_of {
            Ok(MLCFullyConnectedLayer(parent.0))
        } else {
            Err("This MLCLayer cannot be downcasted to MLCFullyConnectedLayer")
        }
    }
}
impl INSObject for MLCFullyConnectedLayer {}
impl PNSObject for MLCFullyConnectedLayer {}
impl IMLCFullyConnectedLayer for MLCFullyConnectedLayer {}
pub trait IMLCFullyConnectedLayer: Sized + std::ops::Deref {
    unsafe fn descriptor(&self) -> MLCConvolutionDescriptor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, descriptor)
    }
    unsafe fn weights(&self) -> MLCTensor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, weights)
    }
    unsafe fn biases(&self) -> MLCTensor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, biases)
    }
    unsafe fn weightsParameter(&self) -> MLCTensorParameter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, weightsParameter)
    }
    unsafe fn biasesParameter(&self) -> MLCTensorParameter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, biasesParameter)
    }
    unsafe fn layerWithWeights_biases_descriptor_(
        weights: MLCTensor,
        biases: MLCTensor,
        descriptor: MLCConvolutionDescriptor,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MLCFullyConnectedLayer").unwrap(), layerWithWeights : weights, biases : biases, descriptor : descriptor)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MLCDropoutLayer(pub id);
impl std::ops::Deref for MLCDropoutLayer {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MLCDropoutLayer {}
impl MLCDropoutLayer {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MLCDropoutLayer").unwrap(), alloc) })
    }
}
impl IMLCLayer for MLCDropoutLayer {}
impl std::convert::TryFrom<MLCLayer> for MLCDropoutLayer {
    type Error = &'static str;
    fn try_from(parent: MLCLayer) -> Result<MLCDropoutLayer, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MLCDropoutLayer").unwrap()) };
        if is_kind_of {
            Ok(MLCDropoutLayer(parent.0))
        } else {
            Err("This MLCLayer cannot be downcasted to MLCDropoutLayer")
        }
    }
}
impl INSObject for MLCDropoutLayer {}
impl PNSObject for MLCDropoutLayer {}
impl IMLCDropoutLayer for MLCDropoutLayer {}
pub trait IMLCDropoutLayer: Sized + std::ops::Deref {
    unsafe fn rate(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, rate)
    }
    unsafe fn seed(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, seed)
    }
    unsafe fn layerWithRate_seed_(rate: f32, seed: NSUInteger) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MLCDropoutLayer").unwrap(), layerWithRate : rate, seed : seed)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MLCGramMatrixLayer(pub id);
impl std::ops::Deref for MLCGramMatrixLayer {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MLCGramMatrixLayer {}
impl MLCGramMatrixLayer {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MLCGramMatrixLayer").unwrap(), alloc) })
    }
}
impl IMLCLayer for MLCGramMatrixLayer {}
impl std::convert::TryFrom<MLCLayer> for MLCGramMatrixLayer {
    type Error = &'static str;
    fn try_from(parent: MLCLayer) -> Result<MLCGramMatrixLayer, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MLCGramMatrixLayer").unwrap()) };
        if is_kind_of {
            Ok(MLCGramMatrixLayer(parent.0))
        } else {
            Err("This MLCLayer cannot be downcasted to MLCGramMatrixLayer")
        }
    }
}
impl INSObject for MLCGramMatrixLayer {}
impl PNSObject for MLCGramMatrixLayer {}
impl IMLCGramMatrixLayer for MLCGramMatrixLayer {}
pub trait IMLCGramMatrixLayer: Sized + std::ops::Deref {
    unsafe fn scale(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, scale)
    }
    unsafe fn layerWithScale_(scale: f32) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MLCGramMatrixLayer").unwrap(), layerWithScale : scale)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MLCActivationLayer(pub id);
impl std::ops::Deref for MLCActivationLayer {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MLCActivationLayer {}
impl MLCActivationLayer {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MLCActivationLayer").unwrap(), alloc) })
    }
}
impl IMLCLayer for MLCActivationLayer {}
impl std::convert::TryFrom<MLCLayer> for MLCActivationLayer {
    type Error = &'static str;
    fn try_from(parent: MLCLayer) -> Result<MLCActivationLayer, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MLCActivationLayer").unwrap()) };
        if is_kind_of {
            Ok(MLCActivationLayer(parent.0))
        } else {
            Err("This MLCLayer cannot be downcasted to MLCActivationLayer")
        }
    }
}
impl INSObject for MLCActivationLayer {}
impl PNSObject for MLCActivationLayer {}
impl IMLCActivationLayer for MLCActivationLayer {}
pub trait IMLCActivationLayer: Sized + std::ops::Deref {
    unsafe fn descriptor(&self) -> MLCActivationDescriptor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, descriptor)
    }
    unsafe fn layerWithDescriptor_(descriptor: MLCActivationDescriptor) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MLCActivationLayer").unwrap(), layerWithDescriptor : descriptor)
    }
    unsafe fn leakyReLULayerWithNegativeSlope_(negativeSlope: f32) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MLCActivationLayer").unwrap(), leakyReLULayerWithNegativeSlope : negativeSlope)
    }
    unsafe fn linearLayerWithScale_bias_(scale: f32, bias: f32) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MLCActivationLayer").unwrap(), linearLayerWithScale : scale, bias : bias)
    }
    unsafe fn softPlusLayerWithBeta_(beta: f32) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MLCActivationLayer").unwrap(), softPlusLayerWithBeta : beta)
    }
    unsafe fn eluLayerWithA_(a: f32) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MLCActivationLayer").unwrap(), eluLayerWithA : a)
    }
    unsafe fn relunLayerWithA_b_(a: f32, b: f32) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MLCActivationLayer").unwrap(), relunLayerWithA : a, b : b)
    }
    unsafe fn celuLayerWithA_(a: f32) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MLCActivationLayer").unwrap(), celuLayerWithA : a)
    }
    unsafe fn hardShrinkLayerWithA_(a: f32) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MLCActivationLayer").unwrap(), hardShrinkLayerWithA : a)
    }
    unsafe fn softShrinkLayerWithA_(a: f32) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MLCActivationLayer").unwrap(), softShrinkLayerWithA : a)
    }
    unsafe fn thresholdLayerWithThreshold_replacement_(
        threshold: f32,
        replacement: f32,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MLCActivationLayer").unwrap(), thresholdLayerWithThreshold : threshold, replacement : replacement)
    }
    unsafe fn clampLayerWithMinValue_maxValue_(minValue: f32, maxValue: f32) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MLCActivationLayer").unwrap(), clampLayerWithMinValue : minValue, maxValue : maxValue)
    }
    unsafe fn reluLayer() -> MLCActivationLayer
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MLCActivationLayer").unwrap(), reluLayer)
    }
    unsafe fn relu6Layer() -> MLCActivationLayer
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MLCActivationLayer").unwrap(), relu6Layer)
    }
    unsafe fn leakyReLULayer() -> MLCActivationLayer
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MLCActivationLayer").unwrap(), leakyReLULayer)
    }
    unsafe fn sigmoidLayer() -> MLCActivationLayer
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MLCActivationLayer").unwrap(), sigmoidLayer)
    }
    unsafe fn hardSigmoidLayer() -> MLCActivationLayer
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MLCActivationLayer").unwrap(), hardSigmoidLayer)
    }
    unsafe fn tanhLayer() -> MLCActivationLayer
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MLCActivationLayer").unwrap(), tanhLayer)
    }
    unsafe fn absoluteLayer() -> MLCActivationLayer
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MLCActivationLayer").unwrap(), absoluteLayer)
    }
    unsafe fn softPlusLayer() -> MLCActivationLayer
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MLCActivationLayer").unwrap(), softPlusLayer)
    }
    unsafe fn softSignLayer() -> MLCActivationLayer
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MLCActivationLayer").unwrap(), softSignLayer)
    }
    unsafe fn eluLayer() -> MLCActivationLayer
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MLCActivationLayer").unwrap(), eluLayer)
    }
    unsafe fn logSigmoidLayer() -> MLCActivationLayer
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MLCActivationLayer").unwrap(), logSigmoidLayer)
    }
    unsafe fn seluLayer() -> MLCActivationLayer
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MLCActivationLayer").unwrap(), seluLayer)
    }
    unsafe fn celuLayer() -> MLCActivationLayer
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MLCActivationLayer").unwrap(), celuLayer)
    }
    unsafe fn hardShrinkLayer() -> MLCActivationLayer
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MLCActivationLayer").unwrap(), hardShrinkLayer)
    }
    unsafe fn softShrinkLayer() -> MLCActivationLayer
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MLCActivationLayer").unwrap(), softShrinkLayer)
    }
    unsafe fn tanhShrinkLayer() -> MLCActivationLayer
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MLCActivationLayer").unwrap(), tanhShrinkLayer)
    }
    unsafe fn geluLayer() -> MLCActivationLayer
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MLCActivationLayer").unwrap(), geluLayer)
    }
    unsafe fn hardSwishLayer() -> MLCActivationLayer
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MLCActivationLayer").unwrap(), hardSwishLayer)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MLCPoolingLayer(pub id);
impl std::ops::Deref for MLCPoolingLayer {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MLCPoolingLayer {}
impl MLCPoolingLayer {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MLCPoolingLayer").unwrap(), alloc) })
    }
}
impl IMLCLayer for MLCPoolingLayer {}
impl std::convert::TryFrom<MLCLayer> for MLCPoolingLayer {
    type Error = &'static str;
    fn try_from(parent: MLCLayer) -> Result<MLCPoolingLayer, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MLCPoolingLayer").unwrap()) };
        if is_kind_of {
            Ok(MLCPoolingLayer(parent.0))
        } else {
            Err("This MLCLayer cannot be downcasted to MLCPoolingLayer")
        }
    }
}
impl INSObject for MLCPoolingLayer {}
impl PNSObject for MLCPoolingLayer {}
impl IMLCPoolingLayer for MLCPoolingLayer {}
pub trait IMLCPoolingLayer: Sized + std::ops::Deref {
    unsafe fn descriptor(&self) -> MLCPoolingDescriptor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, descriptor)
    }
    unsafe fn layerWithDescriptor_(descriptor: MLCPoolingDescriptor) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MLCPoolingLayer").unwrap(), layerWithDescriptor : descriptor)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MLCBatchNormalizationLayer(pub id);
impl std::ops::Deref for MLCBatchNormalizationLayer {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MLCBatchNormalizationLayer {}
impl MLCBatchNormalizationLayer {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MLCBatchNormalizationLayer").unwrap(), alloc) })
    }
}
impl IMLCLayer for MLCBatchNormalizationLayer {}
impl std::convert::TryFrom<MLCLayer> for MLCBatchNormalizationLayer {
    type Error = &'static str;
    fn try_from(parent: MLCLayer) -> Result<MLCBatchNormalizationLayer, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MLCBatchNormalizationLayer").unwrap()) };
        if is_kind_of {
            Ok(MLCBatchNormalizationLayer(parent.0))
        } else {
            Err("This MLCLayer cannot be downcasted to MLCBatchNormalizationLayer")
        }
    }
}
impl INSObject for MLCBatchNormalizationLayer {}
impl PNSObject for MLCBatchNormalizationLayer {}
impl IMLCBatchNormalizationLayer for MLCBatchNormalizationLayer {}
pub trait IMLCBatchNormalizationLayer: Sized + std::ops::Deref {
    unsafe fn featureChannelCount(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, featureChannelCount)
    }
    unsafe fn mean(&self) -> MLCTensor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, mean)
    }
    unsafe fn variance(&self) -> MLCTensor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, variance)
    }
    unsafe fn beta(&self) -> MLCTensor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, beta)
    }
    unsafe fn gamma(&self) -> MLCTensor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, gamma)
    }
    unsafe fn betaParameter(&self) -> MLCTensorParameter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, betaParameter)
    }
    unsafe fn gammaParameter(&self) -> MLCTensorParameter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, gammaParameter)
    }
    unsafe fn varianceEpsilon(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, varianceEpsilon)
    }
    unsafe fn momentum(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, momentum)
    }
    unsafe fn layerWithFeatureChannelCount_mean_variance_beta_gamma_varianceEpsilon_(
        featureChannelCount: NSUInteger,
        mean: MLCTensor,
        variance: MLCTensor,
        beta: MLCTensor,
        gamma: MLCTensor,
        varianceEpsilon: f32,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MLCBatchNormalizationLayer").unwrap(), layerWithFeatureChannelCount : featureChannelCount, mean : mean, variance : variance, beta : beta, gamma : gamma, varianceEpsilon : varianceEpsilon)
    }
    unsafe fn layerWithFeatureChannelCount_mean_variance_beta_gamma_varianceEpsilon_momentum_(
        featureChannelCount: NSUInteger,
        mean: MLCTensor,
        variance: MLCTensor,
        beta: MLCTensor,
        gamma: MLCTensor,
        varianceEpsilon: f32,
        momentum: f32,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MLCBatchNormalizationLayer").unwrap(), layerWithFeatureChannelCount : featureChannelCount, mean : mean, variance : variance, beta : beta, gamma : gamma, varianceEpsilon : varianceEpsilon, momentum : momentum)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MLCInstanceNormalizationLayer(pub id);
impl std::ops::Deref for MLCInstanceNormalizationLayer {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MLCInstanceNormalizationLayer {}
impl MLCInstanceNormalizationLayer {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MLCInstanceNormalizationLayer").unwrap(), alloc) })
    }
}
impl IMLCLayer for MLCInstanceNormalizationLayer {}
impl std::convert::TryFrom<MLCLayer> for MLCInstanceNormalizationLayer {
    type Error = &'static str;
    fn try_from(parent: MLCLayer) -> Result<MLCInstanceNormalizationLayer, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MLCInstanceNormalizationLayer").unwrap())
        };
        if is_kind_of {
            Ok(MLCInstanceNormalizationLayer(parent.0))
        } else {
            Err("This MLCLayer cannot be downcasted to MLCInstanceNormalizationLayer")
        }
    }
}
impl INSObject for MLCInstanceNormalizationLayer {}
impl PNSObject for MLCInstanceNormalizationLayer {}
impl IMLCInstanceNormalizationLayer for MLCInstanceNormalizationLayer {}
pub trait IMLCInstanceNormalizationLayer: Sized + std::ops::Deref {
    unsafe fn featureChannelCount(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, featureChannelCount)
    }
    unsafe fn mean(&self) -> MLCTensor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, mean)
    }
    unsafe fn variance(&self) -> MLCTensor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, variance)
    }
    unsafe fn beta(&self) -> MLCTensor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, beta)
    }
    unsafe fn gamma(&self) -> MLCTensor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, gamma)
    }
    unsafe fn betaParameter(&self) -> MLCTensorParameter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, betaParameter)
    }
    unsafe fn gammaParameter(&self) -> MLCTensorParameter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, gammaParameter)
    }
    unsafe fn varianceEpsilon(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, varianceEpsilon)
    }
    unsafe fn momentum(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, momentum)
    }
    unsafe fn layerWithFeatureChannelCount_beta_gamma_varianceEpsilon_(
        featureChannelCount: NSUInteger,
        beta: MLCTensor,
        gamma: MLCTensor,
        varianceEpsilon: f32,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MLCInstanceNormalizationLayer").unwrap(), layerWithFeatureChannelCount : featureChannelCount, beta : beta, gamma : gamma, varianceEpsilon : varianceEpsilon)
    }
    unsafe fn layerWithFeatureChannelCount_beta_gamma_varianceEpsilon_momentum_(
        featureChannelCount: NSUInteger,
        beta: MLCTensor,
        gamma: MLCTensor,
        varianceEpsilon: f32,
        momentum: f32,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MLCInstanceNormalizationLayer").unwrap(), layerWithFeatureChannelCount : featureChannelCount, beta : beta, gamma : gamma, varianceEpsilon : varianceEpsilon, momentum : momentum)
    }
    unsafe fn layerWithFeatureChannelCount_mean_variance_beta_gamma_varianceEpsilon_momentum_(
        featureChannelCount: NSUInteger,
        mean: MLCTensor,
        variance: MLCTensor,
        beta: MLCTensor,
        gamma: MLCTensor,
        varianceEpsilon: f32,
        momentum: f32,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MLCInstanceNormalizationLayer").unwrap(), layerWithFeatureChannelCount : featureChannelCount, mean : mean, variance : variance, beta : beta, gamma : gamma, varianceEpsilon : varianceEpsilon, momentum : momentum)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MLCConcatenationLayer(pub id);
impl std::ops::Deref for MLCConcatenationLayer {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MLCConcatenationLayer {}
impl MLCConcatenationLayer {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MLCConcatenationLayer").unwrap(), alloc) })
    }
}
impl IMLCLayer for MLCConcatenationLayer {}
impl std::convert::TryFrom<MLCLayer> for MLCConcatenationLayer {
    type Error = &'static str;
    fn try_from(parent: MLCLayer) -> Result<MLCConcatenationLayer, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MLCConcatenationLayer").unwrap()) };
        if is_kind_of {
            Ok(MLCConcatenationLayer(parent.0))
        } else {
            Err("This MLCLayer cannot be downcasted to MLCConcatenationLayer")
        }
    }
}
impl INSObject for MLCConcatenationLayer {}
impl PNSObject for MLCConcatenationLayer {}
impl IMLCConcatenationLayer for MLCConcatenationLayer {}
pub trait IMLCConcatenationLayer: Sized + std::ops::Deref {
    unsafe fn dimension(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, dimension)
    }
    unsafe fn layer() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MLCConcatenationLayer").unwrap(), layer)
    }
    unsafe fn layerWithDimension_(dimension: NSUInteger) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MLCConcatenationLayer").unwrap(), layerWithDimension : dimension)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MLCLayerNormalizationLayer(pub id);
impl std::ops::Deref for MLCLayerNormalizationLayer {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MLCLayerNormalizationLayer {}
impl MLCLayerNormalizationLayer {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MLCLayerNormalizationLayer").unwrap(), alloc) })
    }
}
impl IMLCLayer for MLCLayerNormalizationLayer {}
impl std::convert::TryFrom<MLCLayer> for MLCLayerNormalizationLayer {
    type Error = &'static str;
    fn try_from(parent: MLCLayer) -> Result<MLCLayerNormalizationLayer, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MLCLayerNormalizationLayer").unwrap()) };
        if is_kind_of {
            Ok(MLCLayerNormalizationLayer(parent.0))
        } else {
            Err("This MLCLayer cannot be downcasted to MLCLayerNormalizationLayer")
        }
    }
}
impl INSObject for MLCLayerNormalizationLayer {}
impl PNSObject for MLCLayerNormalizationLayer {}
impl IMLCLayerNormalizationLayer for MLCLayerNormalizationLayer {}
pub trait IMLCLayerNormalizationLayer: Sized + std::ops::Deref {
    unsafe fn normalizedShape(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, normalizedShape)
    }
    unsafe fn beta(&self) -> MLCTensor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, beta)
    }
    unsafe fn gamma(&self) -> MLCTensor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, gamma)
    }
    unsafe fn betaParameter(&self) -> MLCTensorParameter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, betaParameter)
    }
    unsafe fn gammaParameter(&self) -> MLCTensorParameter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, gammaParameter)
    }
    unsafe fn varianceEpsilon(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, varianceEpsilon)
    }
    unsafe fn layerWithNormalizedShape_beta_gamma_varianceEpsilon_(
        normalizedShape: NSArray,
        beta: MLCTensor,
        gamma: MLCTensor,
        varianceEpsilon: f32,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MLCLayerNormalizationLayer").unwrap(), layerWithNormalizedShape : normalizedShape, beta : beta, gamma : gamma, varianceEpsilon : varianceEpsilon)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MLCGroupNormalizationLayer(pub id);
impl std::ops::Deref for MLCGroupNormalizationLayer {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MLCGroupNormalizationLayer {}
impl MLCGroupNormalizationLayer {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MLCGroupNormalizationLayer").unwrap(), alloc) })
    }
}
impl IMLCLayer for MLCGroupNormalizationLayer {}
impl std::convert::TryFrom<MLCLayer> for MLCGroupNormalizationLayer {
    type Error = &'static str;
    fn try_from(parent: MLCLayer) -> Result<MLCGroupNormalizationLayer, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MLCGroupNormalizationLayer").unwrap()) };
        if is_kind_of {
            Ok(MLCGroupNormalizationLayer(parent.0))
        } else {
            Err("This MLCLayer cannot be downcasted to MLCGroupNormalizationLayer")
        }
    }
}
impl INSObject for MLCGroupNormalizationLayer {}
impl PNSObject for MLCGroupNormalizationLayer {}
impl IMLCGroupNormalizationLayer for MLCGroupNormalizationLayer {}
pub trait IMLCGroupNormalizationLayer: Sized + std::ops::Deref {
    unsafe fn featureChannelCount(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, featureChannelCount)
    }
    unsafe fn groupCount(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, groupCount)
    }
    unsafe fn beta(&self) -> MLCTensor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, beta)
    }
    unsafe fn gamma(&self) -> MLCTensor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, gamma)
    }
    unsafe fn betaParameter(&self) -> MLCTensorParameter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, betaParameter)
    }
    unsafe fn gammaParameter(&self) -> MLCTensorParameter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, gammaParameter)
    }
    unsafe fn varianceEpsilon(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, varianceEpsilon)
    }
    unsafe fn layerWithFeatureChannelCount_groupCount_beta_gamma_varianceEpsilon_(
        featureChannelCount: NSUInteger,
        groupCount: NSUInteger,
        beta: MLCTensor,
        gamma: MLCTensor,
        varianceEpsilon: f32,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MLCGroupNormalizationLayer").unwrap(), layerWithFeatureChannelCount : featureChannelCount, groupCount : groupCount, beta : beta, gamma : gamma, varianceEpsilon : varianceEpsilon)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MLCReshapeLayer(pub id);
impl std::ops::Deref for MLCReshapeLayer {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MLCReshapeLayer {}
impl MLCReshapeLayer {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MLCReshapeLayer").unwrap(), alloc) })
    }
}
impl IMLCLayer for MLCReshapeLayer {}
impl std::convert::TryFrom<MLCLayer> for MLCReshapeLayer {
    type Error = &'static str;
    fn try_from(parent: MLCLayer) -> Result<MLCReshapeLayer, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MLCReshapeLayer").unwrap()) };
        if is_kind_of {
            Ok(MLCReshapeLayer(parent.0))
        } else {
            Err("This MLCLayer cannot be downcasted to MLCReshapeLayer")
        }
    }
}
impl INSObject for MLCReshapeLayer {}
impl PNSObject for MLCReshapeLayer {}
impl IMLCReshapeLayer for MLCReshapeLayer {}
pub trait IMLCReshapeLayer: Sized + std::ops::Deref {
    unsafe fn shape(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, shape)
    }
    unsafe fn layerWithShape_(shape: NSArray) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MLCReshapeLayer").unwrap(), layerWithShape : shape)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MLCSoftmaxLayer(pub id);
impl std::ops::Deref for MLCSoftmaxLayer {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MLCSoftmaxLayer {}
impl MLCSoftmaxLayer {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MLCSoftmaxLayer").unwrap(), alloc) })
    }
}
impl IMLCLayer for MLCSoftmaxLayer {}
impl std::convert::TryFrom<MLCLayer> for MLCSoftmaxLayer {
    type Error = &'static str;
    fn try_from(parent: MLCLayer) -> Result<MLCSoftmaxLayer, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MLCSoftmaxLayer").unwrap()) };
        if is_kind_of {
            Ok(MLCSoftmaxLayer(parent.0))
        } else {
            Err("This MLCLayer cannot be downcasted to MLCSoftmaxLayer")
        }
    }
}
impl INSObject for MLCSoftmaxLayer {}
impl PNSObject for MLCSoftmaxLayer {}
impl IMLCSoftmaxLayer for MLCSoftmaxLayer {}
pub trait IMLCSoftmaxLayer: Sized + std::ops::Deref {
    unsafe fn operation(&self) -> MLCSoftmaxOperation
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, operation)
    }
    unsafe fn dimension(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, dimension)
    }
    unsafe fn layerWithOperation_(operation: MLCSoftmaxOperation) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MLCSoftmaxLayer").unwrap(), layerWithOperation : operation)
    }
    unsafe fn layerWithOperation_dimension_(
        operation: MLCSoftmaxOperation,
        dimension: NSUInteger,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MLCSoftmaxLayer").unwrap(), layerWithOperation : operation, dimension : dimension)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MLCLossLayer(pub id);
impl std::ops::Deref for MLCLossLayer {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MLCLossLayer {}
impl MLCLossLayer {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MLCLossLayer").unwrap(), alloc) })
    }
}
impl IMLCLayer for MLCLossLayer {}
impl std::convert::TryFrom<MLCLayer> for MLCLossLayer {
    type Error = &'static str;
    fn try_from(parent: MLCLayer) -> Result<MLCLossLayer, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MLCLossLayer").unwrap()) };
        if is_kind_of {
            Ok(MLCLossLayer(parent.0))
        } else {
            Err("This MLCLayer cannot be downcasted to MLCLossLayer")
        }
    }
}
impl INSObject for MLCLossLayer {}
impl PNSObject for MLCLossLayer {}
impl IMLCLossLayer for MLCLossLayer {}
pub trait IMLCLossLayer: Sized + std::ops::Deref {
    unsafe fn descriptor(&self) -> MLCLossDescriptor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, descriptor)
    }
    unsafe fn weights(&self) -> MLCTensor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, weights)
    }
    unsafe fn layerWithDescriptor_(lossDescriptor: MLCLossDescriptor) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MLCLossLayer").unwrap(), layerWithDescriptor : lossDescriptor)
    }
    unsafe fn layerWithDescriptor_weights_(
        lossDescriptor: MLCLossDescriptor,
        weights: MLCTensor,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MLCLossLayer").unwrap(), layerWithDescriptor : lossDescriptor, weights : weights)
    }
    unsafe fn softmaxCrossEntropyLossWithReductionType_labelSmoothing_classCount_weight_(
        reductionType: MLCReductionType,
        labelSmoothing: f32,
        classCount: NSUInteger,
        weight: f32,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MLCLossLayer").unwrap(), softmaxCrossEntropyLossWithReductionType : reductionType, labelSmoothing : labelSmoothing, classCount : classCount, weight : weight)
    }
    unsafe fn softmaxCrossEntropyLossWithReductionType_labelSmoothing_classCount_weights_(
        reductionType: MLCReductionType,
        labelSmoothing: f32,
        classCount: NSUInteger,
        weights: MLCTensor,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MLCLossLayer").unwrap(), softmaxCrossEntropyLossWithReductionType : reductionType, labelSmoothing : labelSmoothing, classCount : classCount, weights : weights)
    }
    unsafe fn categoricalCrossEntropyLossWithReductionType_labelSmoothing_classCount_weight_(
        reductionType: MLCReductionType,
        labelSmoothing: f32,
        classCount: NSUInteger,
        weight: f32,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MLCLossLayer").unwrap(), categoricalCrossEntropyLossWithReductionType : reductionType, labelSmoothing : labelSmoothing, classCount : classCount, weight : weight)
    }
    unsafe fn categoricalCrossEntropyLossWithReductionType_labelSmoothing_classCount_weights_(
        reductionType: MLCReductionType,
        labelSmoothing: f32,
        classCount: NSUInteger,
        weights: MLCTensor,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MLCLossLayer").unwrap(), categoricalCrossEntropyLossWithReductionType : reductionType, labelSmoothing : labelSmoothing, classCount : classCount, weights : weights)
    }
    unsafe fn sigmoidCrossEntropyLossWithReductionType_labelSmoothing_weight_(
        reductionType: MLCReductionType,
        labelSmoothing: f32,
        weight: f32,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MLCLossLayer").unwrap(), sigmoidCrossEntropyLossWithReductionType : reductionType, labelSmoothing : labelSmoothing, weight : weight)
    }
    unsafe fn sigmoidCrossEntropyLossWithReductionType_labelSmoothing_weights_(
        reductionType: MLCReductionType,
        labelSmoothing: f32,
        weights: MLCTensor,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MLCLossLayer").unwrap(), sigmoidCrossEntropyLossWithReductionType : reductionType, labelSmoothing : labelSmoothing, weights : weights)
    }
    unsafe fn logLossWithReductionType_epsilon_weight_(
        reductionType: MLCReductionType,
        epsilon: f32,
        weight: f32,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MLCLossLayer").unwrap(), logLossWithReductionType : reductionType, epsilon : epsilon, weight : weight)
    }
    unsafe fn logLossWithReductionType_epsilon_weights_(
        reductionType: MLCReductionType,
        epsilon: f32,
        weights: MLCTensor,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MLCLossLayer").unwrap(), logLossWithReductionType : reductionType, epsilon : epsilon, weights : weights)
    }
    unsafe fn huberLossWithReductionType_delta_weight_(
        reductionType: MLCReductionType,
        delta: f32,
        weight: f32,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MLCLossLayer").unwrap(), huberLossWithReductionType : reductionType, delta : delta, weight : weight)
    }
    unsafe fn huberLossWithReductionType_delta_weights_(
        reductionType: MLCReductionType,
        delta: f32,
        weights: MLCTensor,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MLCLossLayer").unwrap(), huberLossWithReductionType : reductionType, delta : delta, weights : weights)
    }
    unsafe fn meanAbsoluteErrorLossWithReductionType_weight_(
        reductionType: MLCReductionType,
        weight: f32,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MLCLossLayer").unwrap(), meanAbsoluteErrorLossWithReductionType : reductionType, weight : weight)
    }
    unsafe fn meanAbsoluteErrorLossWithReductionType_weights_(
        reductionType: MLCReductionType,
        weights: MLCTensor,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MLCLossLayer").unwrap(), meanAbsoluteErrorLossWithReductionType : reductionType, weights : weights)
    }
    unsafe fn meanSquaredErrorLossWithReductionType_weight_(
        reductionType: MLCReductionType,
        weight: f32,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MLCLossLayer").unwrap(), meanSquaredErrorLossWithReductionType : reductionType, weight : weight)
    }
    unsafe fn meanSquaredErrorLossWithReductionType_weights_(
        reductionType: MLCReductionType,
        weights: MLCTensor,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MLCLossLayer").unwrap(), meanSquaredErrorLossWithReductionType : reductionType, weights : weights)
    }
    unsafe fn hingeLossWithReductionType_weight_(
        reductionType: MLCReductionType,
        weight: f32,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MLCLossLayer").unwrap(), hingeLossWithReductionType : reductionType, weight : weight)
    }
    unsafe fn hingeLossWithReductionType_weights_(
        reductionType: MLCReductionType,
        weights: MLCTensor,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MLCLossLayer").unwrap(), hingeLossWithReductionType : reductionType, weights : weights)
    }
    unsafe fn cosineDistanceLossWithReductionType_weight_(
        reductionType: MLCReductionType,
        weight: f32,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MLCLossLayer").unwrap(), cosineDistanceLossWithReductionType : reductionType, weight : weight)
    }
    unsafe fn cosineDistanceLossWithReductionType_weights_(
        reductionType: MLCReductionType,
        weights: MLCTensor,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MLCLossLayer").unwrap(), cosineDistanceLossWithReductionType : reductionType, weights : weights)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MLCYOLOLossLayer(pub id);
impl std::ops::Deref for MLCYOLOLossLayer {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MLCYOLOLossLayer {}
impl MLCYOLOLossLayer {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MLCYOLOLossLayer").unwrap(), alloc) })
    }
}
impl IMLCLossLayer for MLCYOLOLossLayer {}
impl From<MLCYOLOLossLayer> for MLCLossLayer {
    fn from(child: MLCYOLOLossLayer) -> MLCLossLayer {
        MLCLossLayer(child.0)
    }
}
impl std::convert::TryFrom<MLCLossLayer> for MLCYOLOLossLayer {
    type Error = &'static str;
    fn try_from(parent: MLCLossLayer) -> Result<MLCYOLOLossLayer, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MLCYOLOLossLayer").unwrap()) };
        if is_kind_of {
            Ok(MLCYOLOLossLayer(parent.0))
        } else {
            Err("This MLCLossLayer cannot be downcasted to MLCYOLOLossLayer")
        }
    }
}
impl IMLCLayer for MLCYOLOLossLayer {}
impl INSObject for MLCYOLOLossLayer {}
impl PNSObject for MLCYOLOLossLayer {}
impl IMLCYOLOLossLayer for MLCYOLOLossLayer {}
pub trait IMLCYOLOLossLayer: Sized + std::ops::Deref {
    unsafe fn yoloLossDescriptor(&self) -> MLCYOLOLossDescriptor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, yoloLossDescriptor)
    }
    unsafe fn layerWithDescriptor_(lossDescriptor: MLCYOLOLossDescriptor) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MLCYOLOLossLayer").unwrap(), layerWithDescriptor : lossDescriptor)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MLCLSTMLayer(pub id);
impl std::ops::Deref for MLCLSTMLayer {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MLCLSTMLayer {}
impl MLCLSTMLayer {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MLCLSTMLayer").unwrap(), alloc) })
    }
}
impl IMLCLayer for MLCLSTMLayer {}
impl std::convert::TryFrom<MLCLayer> for MLCLSTMLayer {
    type Error = &'static str;
    fn try_from(parent: MLCLayer) -> Result<MLCLSTMLayer, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MLCLSTMLayer").unwrap()) };
        if is_kind_of {
            Ok(MLCLSTMLayer(parent.0))
        } else {
            Err("This MLCLayer cannot be downcasted to MLCLSTMLayer")
        }
    }
}
impl INSObject for MLCLSTMLayer {}
impl PNSObject for MLCLSTMLayer {}
impl IMLCLSTMLayer for MLCLSTMLayer {}
pub trait IMLCLSTMLayer: Sized + std::ops::Deref {
    unsafe fn descriptor(&self) -> MLCLSTMDescriptor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, descriptor)
    }
    unsafe fn gateActivations(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, gateActivations)
    }
    unsafe fn outputResultActivation(&self) -> MLCActivationDescriptor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, outputResultActivation)
    }
    unsafe fn inputWeights(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inputWeights)
    }
    unsafe fn hiddenWeights(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, hiddenWeights)
    }
    unsafe fn peepholeWeights(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, peepholeWeights)
    }
    unsafe fn biases(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, biases)
    }
    unsafe fn inputWeightsParameters(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inputWeightsParameters)
    }
    unsafe fn hiddenWeightsParameters(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, hiddenWeightsParameters)
    }
    unsafe fn peepholeWeightsParameters(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, peepholeWeightsParameters)
    }
    unsafe fn biasesParameters(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, biasesParameters)
    }
    unsafe fn layerWithDescriptor_inputWeights_hiddenWeights_biases_(
        descriptor: MLCLSTMDescriptor,
        inputWeights: NSArray,
        hiddenWeights: NSArray,
        biases: NSArray,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MLCLSTMLayer").unwrap(), layerWithDescriptor : descriptor, inputWeights : inputWeights, hiddenWeights : hiddenWeights, biases : biases)
    }
    unsafe fn layerWithDescriptor_inputWeights_hiddenWeights_peepholeWeights_biases_(
        descriptor: MLCLSTMDescriptor,
        inputWeights: NSArray,
        hiddenWeights: NSArray,
        peepholeWeights: NSArray,
        biases: NSArray,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MLCLSTMLayer").unwrap(), layerWithDescriptor : descriptor, inputWeights : inputWeights, hiddenWeights : hiddenWeights, peepholeWeights : peepholeWeights, biases : biases)
    }
    unsafe fn layerWithDescriptor_inputWeights_hiddenWeights_peepholeWeights_biases_gateActivations_outputResultActivation_(
        descriptor: MLCLSTMDescriptor,
        inputWeights: NSArray,
        hiddenWeights: NSArray,
        peepholeWeights: NSArray,
        biases: NSArray,
        gateActivations: NSArray,
        outputResultActivation: MLCActivationDescriptor,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MLCLSTMLayer").unwrap(), layerWithDescriptor : descriptor, inputWeights : inputWeights, hiddenWeights : hiddenWeights, peepholeWeights : peepholeWeights, biases : biases, gateActivations : gateActivations, outputResultActivation : outputResultActivation)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MLCUpsampleLayer(pub id);
impl std::ops::Deref for MLCUpsampleLayer {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MLCUpsampleLayer {}
impl MLCUpsampleLayer {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MLCUpsampleLayer").unwrap(), alloc) })
    }
}
impl IMLCLayer for MLCUpsampleLayer {}
impl std::convert::TryFrom<MLCLayer> for MLCUpsampleLayer {
    type Error = &'static str;
    fn try_from(parent: MLCLayer) -> Result<MLCUpsampleLayer, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MLCUpsampleLayer").unwrap()) };
        if is_kind_of {
            Ok(MLCUpsampleLayer(parent.0))
        } else {
            Err("This MLCLayer cannot be downcasted to MLCUpsampleLayer")
        }
    }
}
impl INSObject for MLCUpsampleLayer {}
impl PNSObject for MLCUpsampleLayer {}
impl IMLCUpsampleLayer for MLCUpsampleLayer {}
pub trait IMLCUpsampleLayer: Sized + std::ops::Deref {
    unsafe fn shape(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, shape)
    }
    unsafe fn sampleMode(&self) -> MLCSampleMode
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, sampleMode)
    }
    unsafe fn alignsCorners(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, alignsCorners)
    }
    unsafe fn layerWithShape_(shape: NSArray) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MLCUpsampleLayer").unwrap(), layerWithShape : shape)
    }
    unsafe fn layerWithShape_sampleMode_alignsCorners_(
        shape: NSArray,
        sampleMode: MLCSampleMode,
        alignsCorners: BOOL,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MLCUpsampleLayer").unwrap(), layerWithShape : shape, sampleMode : sampleMode, alignsCorners : alignsCorners)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MLCPaddingLayer(pub id);
impl std::ops::Deref for MLCPaddingLayer {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MLCPaddingLayer {}
impl MLCPaddingLayer {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MLCPaddingLayer").unwrap(), alloc) })
    }
}
impl PNSCopying for MLCPaddingLayer {}
impl IMLCLayer for MLCPaddingLayer {}
impl std::convert::TryFrom<MLCLayer> for MLCPaddingLayer {
    type Error = &'static str;
    fn try_from(parent: MLCLayer) -> Result<MLCPaddingLayer, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MLCPaddingLayer").unwrap()) };
        if is_kind_of {
            Ok(MLCPaddingLayer(parent.0))
        } else {
            Err("This MLCLayer cannot be downcasted to MLCPaddingLayer")
        }
    }
}
impl INSObject for MLCPaddingLayer {}
impl PNSObject for MLCPaddingLayer {}
impl IMLCPaddingLayer for MLCPaddingLayer {}
pub trait IMLCPaddingLayer: Sized + std::ops::Deref {
    unsafe fn paddingType(&self) -> MLCPaddingType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, paddingType)
    }
    unsafe fn paddingLeft(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, paddingLeft)
    }
    unsafe fn paddingRight(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, paddingRight)
    }
    unsafe fn paddingTop(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, paddingTop)
    }
    unsafe fn paddingBottom(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, paddingBottom)
    }
    unsafe fn constantValue(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, constantValue)
    }
    unsafe fn layerWithReflectionPadding_(padding: NSArray) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MLCPaddingLayer").unwrap(), layerWithReflectionPadding : padding)
    }
    unsafe fn layerWithSymmetricPadding_(padding: NSArray) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MLCPaddingLayer").unwrap(), layerWithSymmetricPadding : padding)
    }
    unsafe fn layerWithZeroPadding_(padding: NSArray) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MLCPaddingLayer").unwrap(), layerWithZeroPadding : padding)
    }
    unsafe fn layerWithConstantPadding_constantValue_(
        padding: NSArray,
        constantValue: f32,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MLCPaddingLayer").unwrap(), layerWithConstantPadding : padding, constantValue : constantValue)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MLCTransposeLayer(pub id);
impl std::ops::Deref for MLCTransposeLayer {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MLCTransposeLayer {}
impl MLCTransposeLayer {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MLCTransposeLayer").unwrap(), alloc) })
    }
}
impl IMLCLayer for MLCTransposeLayer {}
impl std::convert::TryFrom<MLCLayer> for MLCTransposeLayer {
    type Error = &'static str;
    fn try_from(parent: MLCLayer) -> Result<MLCTransposeLayer, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MLCTransposeLayer").unwrap()) };
        if is_kind_of {
            Ok(MLCTransposeLayer(parent.0))
        } else {
            Err("This MLCLayer cannot be downcasted to MLCTransposeLayer")
        }
    }
}
impl INSObject for MLCTransposeLayer {}
impl PNSObject for MLCTransposeLayer {}
impl IMLCTransposeLayer for MLCTransposeLayer {}
pub trait IMLCTransposeLayer: Sized + std::ops::Deref {
    unsafe fn dimensions(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, dimensions)
    }
    unsafe fn layerWithDimensions_(dimensions: NSArray) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MLCTransposeLayer").unwrap(), layerWithDimensions : dimensions)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MLCReductionLayer(pub id);
impl std::ops::Deref for MLCReductionLayer {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MLCReductionLayer {}
impl MLCReductionLayer {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MLCReductionLayer").unwrap(), alloc) })
    }
}
impl IMLCLayer for MLCReductionLayer {}
impl std::convert::TryFrom<MLCLayer> for MLCReductionLayer {
    type Error = &'static str;
    fn try_from(parent: MLCLayer) -> Result<MLCReductionLayer, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MLCReductionLayer").unwrap()) };
        if is_kind_of {
            Ok(MLCReductionLayer(parent.0))
        } else {
            Err("This MLCLayer cannot be downcasted to MLCReductionLayer")
        }
    }
}
impl INSObject for MLCReductionLayer {}
impl PNSObject for MLCReductionLayer {}
impl IMLCReductionLayer for MLCReductionLayer {}
pub trait IMLCReductionLayer: Sized + std::ops::Deref {
    unsafe fn reductionType(&self) -> MLCReductionType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, reductionType)
    }
    unsafe fn dimension(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, dimension)
    }
    unsafe fn dimensions(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, dimensions)
    }
    unsafe fn layerWithReductionType_dimension_(
        reductionType: MLCReductionType,
        dimension: NSUInteger,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MLCReductionLayer").unwrap(), layerWithReductionType : reductionType, dimension : dimension)
    }
    unsafe fn layerWithReductionType_dimensions_(
        reductionType: MLCReductionType,
        dimensions: NSArray,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MLCReductionLayer").unwrap(), layerWithReductionType : reductionType, dimensions : dimensions)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MLCMultiheadAttentionLayer(pub id);
impl std::ops::Deref for MLCMultiheadAttentionLayer {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MLCMultiheadAttentionLayer {}
impl MLCMultiheadAttentionLayer {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MLCMultiheadAttentionLayer").unwrap(), alloc) })
    }
}
impl IMLCLayer for MLCMultiheadAttentionLayer {}
impl std::convert::TryFrom<MLCLayer> for MLCMultiheadAttentionLayer {
    type Error = &'static str;
    fn try_from(parent: MLCLayer) -> Result<MLCMultiheadAttentionLayer, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MLCMultiheadAttentionLayer").unwrap()) };
        if is_kind_of {
            Ok(MLCMultiheadAttentionLayer(parent.0))
        } else {
            Err("This MLCLayer cannot be downcasted to MLCMultiheadAttentionLayer")
        }
    }
}
impl INSObject for MLCMultiheadAttentionLayer {}
impl PNSObject for MLCMultiheadAttentionLayer {}
impl IMLCMultiheadAttentionLayer for MLCMultiheadAttentionLayer {}
pub trait IMLCMultiheadAttentionLayer: Sized + std::ops::Deref {
    unsafe fn descriptor(&self) -> MLCMultiheadAttentionDescriptor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, descriptor)
    }
    unsafe fn weights(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, weights)
    }
    unsafe fn biases(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, biases)
    }
    unsafe fn attentionBiases(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, attentionBiases)
    }
    unsafe fn weightsParameters(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, weightsParameters)
    }
    unsafe fn biasesParameters(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, biasesParameters)
    }
    unsafe fn layerWithDescriptor_weights_biases_attentionBiases_(
        descriptor: MLCMultiheadAttentionDescriptor,
        weights: NSArray,
        biases: NSArray,
        attentionBiases: NSArray,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MLCMultiheadAttentionLayer").unwrap(), layerWithDescriptor : descriptor, weights : weights, biases : biases, attentionBiases : attentionBiases)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MLCSplitLayer(pub id);
impl std::ops::Deref for MLCSplitLayer {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MLCSplitLayer {}
impl MLCSplitLayer {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MLCSplitLayer").unwrap(), alloc) })
    }
}
impl IMLCLayer for MLCSplitLayer {}
impl std::convert::TryFrom<MLCLayer> for MLCSplitLayer {
    type Error = &'static str;
    fn try_from(parent: MLCLayer) -> Result<MLCSplitLayer, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MLCSplitLayer").unwrap()) };
        if is_kind_of {
            Ok(MLCSplitLayer(parent.0))
        } else {
            Err("This MLCLayer cannot be downcasted to MLCSplitLayer")
        }
    }
}
impl INSObject for MLCSplitLayer {}
impl PNSObject for MLCSplitLayer {}
impl IMLCSplitLayer for MLCSplitLayer {}
pub trait IMLCSplitLayer: Sized + std::ops::Deref {
    unsafe fn dimension(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, dimension)
    }
    unsafe fn splitCount(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, splitCount)
    }
    unsafe fn splitSectionLengths(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, splitSectionLengths)
    }
    unsafe fn layerWithSplitCount_dimension_(
        splitCount: NSUInteger,
        dimension: NSUInteger,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MLCSplitLayer").unwrap(), layerWithSplitCount : splitCount, dimension : dimension)
    }
    unsafe fn layerWithSplitSectionLengths_dimension_(
        splitSectionLengths: NSArray,
        dimension: NSUInteger,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MLCSplitLayer").unwrap(), layerWithSplitSectionLengths : splitSectionLengths, dimension : dimension)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MLCMatMulLayer(pub id);
impl std::ops::Deref for MLCMatMulLayer {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MLCMatMulLayer {}
impl MLCMatMulLayer {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MLCMatMulLayer").unwrap(), alloc) })
    }
}
impl IMLCLayer for MLCMatMulLayer {}
impl std::convert::TryFrom<MLCLayer> for MLCMatMulLayer {
    type Error = &'static str;
    fn try_from(parent: MLCLayer) -> Result<MLCMatMulLayer, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MLCMatMulLayer").unwrap()) };
        if is_kind_of {
            Ok(MLCMatMulLayer(parent.0))
        } else {
            Err("This MLCLayer cannot be downcasted to MLCMatMulLayer")
        }
    }
}
impl INSObject for MLCMatMulLayer {}
impl PNSObject for MLCMatMulLayer {}
impl IMLCMatMulLayer for MLCMatMulLayer {}
pub trait IMLCMatMulLayer: Sized + std::ops::Deref {
    unsafe fn descriptor(&self) -> MLCMatMulDescriptor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, descriptor)
    }
    unsafe fn layerWithDescriptor_(descriptor: MLCMatMulDescriptor) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MLCMatMulLayer").unwrap(), layerWithDescriptor : descriptor)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MLCSliceLayer(pub id);
impl std::ops::Deref for MLCSliceLayer {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MLCSliceLayer {}
impl MLCSliceLayer {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MLCSliceLayer").unwrap(), alloc) })
    }
}
impl IMLCLayer for MLCSliceLayer {}
impl std::convert::TryFrom<MLCLayer> for MLCSliceLayer {
    type Error = &'static str;
    fn try_from(parent: MLCLayer) -> Result<MLCSliceLayer, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MLCSliceLayer").unwrap()) };
        if is_kind_of {
            Ok(MLCSliceLayer(parent.0))
        } else {
            Err("This MLCLayer cannot be downcasted to MLCSliceLayer")
        }
    }
}
impl INSObject for MLCSliceLayer {}
impl PNSObject for MLCSliceLayer {}
impl IMLCSliceLayer for MLCSliceLayer {}
pub trait IMLCSliceLayer: Sized + std::ops::Deref {
    unsafe fn start(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, start)
    }
    unsafe fn end(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, end)
    }
    unsafe fn stride(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, stride)
    }
    unsafe fn sliceLayerWithStart_end_stride_(
        start: NSArray,
        end: NSArray,
        stride: NSArray,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MLCSliceLayer").unwrap(), sliceLayerWithStart : start, end : end, stride : stride)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MLCEmbeddingLayer(pub id);
impl std::ops::Deref for MLCEmbeddingLayer {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MLCEmbeddingLayer {}
impl MLCEmbeddingLayer {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MLCEmbeddingLayer").unwrap(), alloc) })
    }
}
impl IMLCLayer for MLCEmbeddingLayer {}
impl std::convert::TryFrom<MLCLayer> for MLCEmbeddingLayer {
    type Error = &'static str;
    fn try_from(parent: MLCLayer) -> Result<MLCEmbeddingLayer, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MLCEmbeddingLayer").unwrap()) };
        if is_kind_of {
            Ok(MLCEmbeddingLayer(parent.0))
        } else {
            Err("This MLCLayer cannot be downcasted to MLCEmbeddingLayer")
        }
    }
}
impl INSObject for MLCEmbeddingLayer {}
impl PNSObject for MLCEmbeddingLayer {}
impl IMLCEmbeddingLayer for MLCEmbeddingLayer {}
pub trait IMLCEmbeddingLayer: Sized + std::ops::Deref {
    unsafe fn descriptor(&self) -> MLCEmbeddingDescriptor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, descriptor)
    }
    unsafe fn weights(&self) -> MLCTensor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, weights)
    }
    unsafe fn weightsParameter(&self) -> MLCTensorParameter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, weightsParameter)
    }
    unsafe fn layerWithDescriptor_weights_(
        descriptor: MLCEmbeddingDescriptor,
        weights: MLCTensor,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MLCEmbeddingLayer").unwrap(), layerWithDescriptor : descriptor, weights : weights)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MLCSelectionLayer(pub id);
impl std::ops::Deref for MLCSelectionLayer {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MLCSelectionLayer {}
impl MLCSelectionLayer {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MLCSelectionLayer").unwrap(), alloc) })
    }
}
impl IMLCLayer for MLCSelectionLayer {}
impl std::convert::TryFrom<MLCLayer> for MLCSelectionLayer {
    type Error = &'static str;
    fn try_from(parent: MLCLayer) -> Result<MLCSelectionLayer, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MLCSelectionLayer").unwrap()) };
        if is_kind_of {
            Ok(MLCSelectionLayer(parent.0))
        } else {
            Err("This MLCLayer cannot be downcasted to MLCSelectionLayer")
        }
    }
}
impl INSObject for MLCSelectionLayer {}
impl PNSObject for MLCSelectionLayer {}
impl IMLCSelectionLayer for MLCSelectionLayer {}
pub trait IMLCSelectionLayer: Sized + std::ops::Deref {
    unsafe fn layer() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MLCSelectionLayer").unwrap(), layer)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MLCGatherLayer(pub id);
impl std::ops::Deref for MLCGatherLayer {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MLCGatherLayer {}
impl MLCGatherLayer {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MLCGatherLayer").unwrap(), alloc) })
    }
}
impl IMLCLayer for MLCGatherLayer {}
impl std::convert::TryFrom<MLCLayer> for MLCGatherLayer {
    type Error = &'static str;
    fn try_from(parent: MLCLayer) -> Result<MLCGatherLayer, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MLCGatherLayer").unwrap()) };
        if is_kind_of {
            Ok(MLCGatherLayer(parent.0))
        } else {
            Err("This MLCLayer cannot be downcasted to MLCGatherLayer")
        }
    }
}
impl INSObject for MLCGatherLayer {}
impl PNSObject for MLCGatherLayer {}
impl IMLCGatherLayer for MLCGatherLayer {}
pub trait IMLCGatherLayer: Sized + std::ops::Deref {
    unsafe fn dimension(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, dimension)
    }
    unsafe fn layerWithDimension_(dimension: NSUInteger) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MLCGatherLayer").unwrap(), layerWithDimension : dimension)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MLCScatterLayer(pub id);
impl std::ops::Deref for MLCScatterLayer {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MLCScatterLayer {}
impl MLCScatterLayer {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MLCScatterLayer").unwrap(), alloc) })
    }
}
impl IMLCLayer for MLCScatterLayer {}
impl std::convert::TryFrom<MLCLayer> for MLCScatterLayer {
    type Error = &'static str;
    fn try_from(parent: MLCLayer) -> Result<MLCScatterLayer, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MLCScatterLayer").unwrap()) };
        if is_kind_of {
            Ok(MLCScatterLayer(parent.0))
        } else {
            Err("This MLCLayer cannot be downcasted to MLCScatterLayer")
        }
    }
}
impl INSObject for MLCScatterLayer {}
impl PNSObject for MLCScatterLayer {}
impl IMLCScatterLayer for MLCScatterLayer {}
pub trait IMLCScatterLayer: Sized + std::ops::Deref {
    unsafe fn dimension(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, dimension)
    }
    unsafe fn reductionType(&self) -> MLCReductionType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, reductionType)
    }
    unsafe fn layerWithDimension_reductionType_(
        dimension: NSUInteger,
        reductionType: MLCReductionType,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MLCScatterLayer").unwrap(), layerWithDimension : dimension, reductionType : reductionType)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MLCOptimizer(pub id);
impl std::ops::Deref for MLCOptimizer {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MLCOptimizer {}
impl MLCOptimizer {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MLCOptimizer").unwrap(), alloc) })
    }
}
impl PNSCopying for MLCOptimizer {}
impl INSObject for MLCOptimizer {}
impl PNSObject for MLCOptimizer {}
impl std::convert::TryFrom<NSObject> for MLCOptimizer {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<MLCOptimizer, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MLCOptimizer").unwrap()) };
        if is_kind_of {
            Ok(MLCOptimizer(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to MLCOptimizer")
        }
    }
}
impl IMLCOptimizer for MLCOptimizer {}
pub trait IMLCOptimizer: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn learningRate(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, learningRate)
    }
    unsafe fn setLearningRate_(&self, learningRate: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setLearningRate : learningRate)
    }
    unsafe fn gradientRescale(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, gradientRescale)
    }
    unsafe fn appliesGradientClipping(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, appliesGradientClipping)
    }
    unsafe fn setAppliesGradientClipping_(&self, appliesGradientClipping: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAppliesGradientClipping : appliesGradientClipping)
    }
    unsafe fn gradientClipMax(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, gradientClipMax)
    }
    unsafe fn gradientClipMin(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, gradientClipMin)
    }
    unsafe fn regularizationScale(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, regularizationScale)
    }
    unsafe fn regularizationType(&self) -> MLCRegularizationType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, regularizationType)
    }
    unsafe fn gradientClippingType(&self) -> MLCGradientClippingType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, gradientClippingType)
    }
    unsafe fn maximumClippingNorm(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, maximumClippingNorm)
    }
    unsafe fn customGlobalNorm(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, customGlobalNorm)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MLCOptimizer").unwrap(), new)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MLCSGDOptimizer(pub id);
impl std::ops::Deref for MLCSGDOptimizer {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MLCSGDOptimizer {}
impl MLCSGDOptimizer {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MLCSGDOptimizer").unwrap(), alloc) })
    }
}
impl PNSCopying for MLCSGDOptimizer {}
impl IMLCOptimizer for MLCSGDOptimizer {}
impl From<MLCSGDOptimizer> for MLCOptimizer {
    fn from(child: MLCSGDOptimizer) -> MLCOptimizer {
        MLCOptimizer(child.0)
    }
}
impl std::convert::TryFrom<MLCOptimizer> for MLCSGDOptimizer {
    type Error = &'static str;
    fn try_from(parent: MLCOptimizer) -> Result<MLCSGDOptimizer, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MLCSGDOptimizer").unwrap()) };
        if is_kind_of {
            Ok(MLCSGDOptimizer(parent.0))
        } else {
            Err("This MLCOptimizer cannot be downcasted to MLCSGDOptimizer")
        }
    }
}
impl INSObject for MLCSGDOptimizer {}
impl PNSObject for MLCSGDOptimizer {}
impl IMLCSGDOptimizer for MLCSGDOptimizer {}
pub trait IMLCSGDOptimizer: Sized + std::ops::Deref {
    unsafe fn momentumScale(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, momentumScale)
    }
    unsafe fn usesNesterovMomentum(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, usesNesterovMomentum)
    }
    unsafe fn optimizerWithDescriptor_(optimizerDescriptor: MLCOptimizerDescriptor) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MLCSGDOptimizer").unwrap(), optimizerWithDescriptor : optimizerDescriptor)
    }
    unsafe fn optimizerWithDescriptor_momentumScale_usesNesterovMomentum_(
        optimizerDescriptor: MLCOptimizerDescriptor,
        momentumScale: f32,
        usesNesterovMomentum: BOOL,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MLCSGDOptimizer").unwrap(), optimizerWithDescriptor : optimizerDescriptor, momentumScale : momentumScale, usesNesterovMomentum : usesNesterovMomentum)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MLCAdamOptimizer(pub id);
impl std::ops::Deref for MLCAdamOptimizer {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MLCAdamOptimizer {}
impl MLCAdamOptimizer {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MLCAdamOptimizer").unwrap(), alloc) })
    }
}
impl PNSCopying for MLCAdamOptimizer {}
impl IMLCOptimizer for MLCAdamOptimizer {}
impl std::convert::TryFrom<MLCOptimizer> for MLCAdamOptimizer {
    type Error = &'static str;
    fn try_from(parent: MLCOptimizer) -> Result<MLCAdamOptimizer, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MLCAdamOptimizer").unwrap()) };
        if is_kind_of {
            Ok(MLCAdamOptimizer(parent.0))
        } else {
            Err("This MLCOptimizer cannot be downcasted to MLCAdamOptimizer")
        }
    }
}
impl INSObject for MLCAdamOptimizer {}
impl PNSObject for MLCAdamOptimizer {}
impl IMLCAdamOptimizer for MLCAdamOptimizer {}
pub trait IMLCAdamOptimizer: Sized + std::ops::Deref {
    unsafe fn beta1(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, beta1)
    }
    unsafe fn beta2(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, beta2)
    }
    unsafe fn epsilon(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, epsilon)
    }
    unsafe fn usesAMSGrad(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, usesAMSGrad)
    }
    unsafe fn timeStep(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, timeStep)
    }
    unsafe fn optimizerWithDescriptor_(optimizerDescriptor: MLCOptimizerDescriptor) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MLCAdamOptimizer").unwrap(), optimizerWithDescriptor : optimizerDescriptor)
    }
    unsafe fn optimizerWithDescriptor_beta1_beta2_epsilon_timeStep_(
        optimizerDescriptor: MLCOptimizerDescriptor,
        beta1: f32,
        beta2: f32,
        epsilon: f32,
        timeStep: NSUInteger,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MLCAdamOptimizer").unwrap(), optimizerWithDescriptor : optimizerDescriptor, beta1 : beta1, beta2 : beta2, epsilon : epsilon, timeStep : timeStep)
    }
    unsafe fn optimizerWithDescriptor_beta1_beta2_epsilon_usesAMSGrad_timeStep_(
        optimizerDescriptor: MLCOptimizerDescriptor,
        beta1: f32,
        beta2: f32,
        epsilon: f32,
        usesAMSGrad: BOOL,
        timeStep: NSUInteger,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MLCAdamOptimizer").unwrap(), optimizerWithDescriptor : optimizerDescriptor, beta1 : beta1, beta2 : beta2, epsilon : epsilon, usesAMSGrad : usesAMSGrad, timeStep : timeStep)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MLCAdamWOptimizer(pub id);
impl std::ops::Deref for MLCAdamWOptimizer {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MLCAdamWOptimizer {}
impl MLCAdamWOptimizer {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MLCAdamWOptimizer").unwrap(), alloc) })
    }
}
impl PNSCopying for MLCAdamWOptimizer {}
impl IMLCOptimizer for MLCAdamWOptimizer {}
impl std::convert::TryFrom<MLCOptimizer> for MLCAdamWOptimizer {
    type Error = &'static str;
    fn try_from(parent: MLCOptimizer) -> Result<MLCAdamWOptimizer, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MLCAdamWOptimizer").unwrap()) };
        if is_kind_of {
            Ok(MLCAdamWOptimizer(parent.0))
        } else {
            Err("This MLCOptimizer cannot be downcasted to MLCAdamWOptimizer")
        }
    }
}
impl INSObject for MLCAdamWOptimizer {}
impl PNSObject for MLCAdamWOptimizer {}
impl IMLCAdamWOptimizer for MLCAdamWOptimizer {}
pub trait IMLCAdamWOptimizer: Sized + std::ops::Deref {
    unsafe fn beta1(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, beta1)
    }
    unsafe fn beta2(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, beta2)
    }
    unsafe fn epsilon(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, epsilon)
    }
    unsafe fn usesAMSGrad(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, usesAMSGrad)
    }
    unsafe fn timeStep(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, timeStep)
    }
    unsafe fn optimizerWithDescriptor_(optimizerDescriptor: MLCOptimizerDescriptor) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MLCAdamWOptimizer").unwrap(), optimizerWithDescriptor : optimizerDescriptor)
    }
    unsafe fn optimizerWithDescriptor_beta1_beta2_epsilon_usesAMSGrad_timeStep_(
        optimizerDescriptor: MLCOptimizerDescriptor,
        beta1: f32,
        beta2: f32,
        epsilon: f32,
        usesAMSGrad: BOOL,
        timeStep: NSUInteger,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MLCAdamWOptimizer").unwrap(), optimizerWithDescriptor : optimizerDescriptor, beta1 : beta1, beta2 : beta2, epsilon : epsilon, usesAMSGrad : usesAMSGrad, timeStep : timeStep)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MLCRMSPropOptimizer(pub id);
impl std::ops::Deref for MLCRMSPropOptimizer {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MLCRMSPropOptimizer {}
impl MLCRMSPropOptimizer {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MLCRMSPropOptimizer").unwrap(), alloc) })
    }
}
impl PNSCopying for MLCRMSPropOptimizer {}
impl IMLCOptimizer for MLCRMSPropOptimizer {}
impl std::convert::TryFrom<MLCOptimizer> for MLCRMSPropOptimizer {
    type Error = &'static str;
    fn try_from(parent: MLCOptimizer) -> Result<MLCRMSPropOptimizer, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MLCRMSPropOptimizer").unwrap()) };
        if is_kind_of {
            Ok(MLCRMSPropOptimizer(parent.0))
        } else {
            Err("This MLCOptimizer cannot be downcasted to MLCRMSPropOptimizer")
        }
    }
}
impl INSObject for MLCRMSPropOptimizer {}
impl PNSObject for MLCRMSPropOptimizer {}
impl IMLCRMSPropOptimizer for MLCRMSPropOptimizer {}
pub trait IMLCRMSPropOptimizer: Sized + std::ops::Deref {
    unsafe fn momentumScale(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, momentumScale)
    }
    unsafe fn alpha(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, alpha)
    }
    unsafe fn epsilon(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, epsilon)
    }
    unsafe fn isCentered(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isCentered)
    }
    unsafe fn optimizerWithDescriptor_(optimizerDescriptor: MLCOptimizerDescriptor) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MLCRMSPropOptimizer").unwrap(), optimizerWithDescriptor : optimizerDescriptor)
    }
    unsafe fn optimizerWithDescriptor_momentumScale_alpha_epsilon_isCentered_(
        optimizerDescriptor: MLCOptimizerDescriptor,
        momentumScale: f32,
        alpha: f32,
        epsilon: f32,
        isCentered: BOOL,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MLCRMSPropOptimizer").unwrap(), optimizerWithDescriptor : optimizerDescriptor, momentumScale : momentumScale, alpha : alpha, epsilon : epsilon, isCentered : isCentered)
    }
}
unsafe extern "C" {
    pub fn MLCActivationTypeDebugDescription(activationType: MLCActivationType) -> NSString;
}
unsafe extern "C" {
    pub fn MLCArithmeticOperationDebugDescription(operation: MLCArithmeticOperation) -> NSString;
}
unsafe extern "C" {
    pub fn MLCPaddingPolicyDebugDescription(paddingPolicy: MLCPaddingPolicy) -> NSString;
}
unsafe extern "C" {
    pub fn MLCLossTypeDebugDescription(lossType: MLCLossType) -> NSString;
}
unsafe extern "C" {
    pub fn MLCReductionTypeDebugDescription(reductionType: MLCReductionType) -> NSString;
}
unsafe extern "C" {
    pub fn MLCPaddingTypeDebugDescription(paddingType: MLCPaddingType) -> NSString;
}
unsafe extern "C" {
    pub fn MLCConvolutionTypeDebugDescription(convolutionType: MLCConvolutionType) -> NSString;
}
unsafe extern "C" {
    pub fn MLCPoolingTypeDebugDescription(poolingType: MLCPoolingType) -> NSString;
}
unsafe extern "C" {
    pub fn MLCSoftmaxOperationDebugDescription(operation: MLCSoftmaxOperation) -> NSString;
}
unsafe extern "C" {
    pub fn MLCSampleModeDebugDescription(mode: MLCSampleMode) -> NSString;
}
unsafe extern "C" {
    pub fn MLCLSTMResultModeDebugDescription(mode: MLCLSTMResultMode) -> NSString;
}
unsafe extern "C" {
    pub fn MLCComparisonOperationDebugDescription(operation: MLCComparisonOperation) -> NSString;
}
unsafe extern "C" {
    pub fn MLCGradientClippingTypeDebugDescription(
        gradientClippingType: MLCGradientClippingType,
    ) -> NSString;
}

unsafe impl objc2::encode::RefEncode for MLCDevice {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MLCDevice {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MLCTensor {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MLCTensor {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MLCTensorData {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MLCTensorData {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MLCTensorParameter {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MLCTensorParameter {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MLCTensorOptimizerDeviceData {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MLCTensorOptimizerDeviceData {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MLCGraph {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MLCGraph {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MLCTrainingGraph {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MLCTrainingGraph {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MLCInferenceGraph {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MLCInferenceGraph {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MLCPlatform {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MLCPlatform {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MLCActivationDescriptor {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MLCActivationDescriptor {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MLCPoolingDescriptor {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MLCPoolingDescriptor {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MLCConvolutionDescriptor {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MLCConvolutionDescriptor {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MLCLossDescriptor {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MLCLossDescriptor {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MLCYOLOLossDescriptor {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MLCYOLOLossDescriptor {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MLCLSTMDescriptor {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MLCLSTMDescriptor {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MLCTensorDescriptor {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MLCTensorDescriptor {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MLCOptimizerDescriptor {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MLCOptimizerDescriptor {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MLCMatMulDescriptor {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MLCMatMulDescriptor {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MLCMultiheadAttentionDescriptor {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MLCMultiheadAttentionDescriptor {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MLCEmbeddingDescriptor {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MLCEmbeddingDescriptor {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MLCLayer {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MLCLayer {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MLCArithmeticLayer {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MLCArithmeticLayer {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MLCComparisonLayer {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MLCComparisonLayer {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MLCConvolutionLayer {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MLCConvolutionLayer {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MLCFullyConnectedLayer {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MLCFullyConnectedLayer {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MLCDropoutLayer {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MLCDropoutLayer {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MLCGramMatrixLayer {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MLCGramMatrixLayer {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MLCActivationLayer {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MLCActivationLayer {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MLCPoolingLayer {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MLCPoolingLayer {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MLCBatchNormalizationLayer {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MLCBatchNormalizationLayer {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MLCInstanceNormalizationLayer {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MLCInstanceNormalizationLayer {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MLCConcatenationLayer {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MLCConcatenationLayer {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MLCLayerNormalizationLayer {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MLCLayerNormalizationLayer {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MLCGroupNormalizationLayer {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MLCGroupNormalizationLayer {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MLCReshapeLayer {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MLCReshapeLayer {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MLCSoftmaxLayer {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MLCSoftmaxLayer {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MLCLossLayer {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MLCLossLayer {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MLCYOLOLossLayer {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MLCYOLOLossLayer {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MLCLSTMLayer {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MLCLSTMLayer {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MLCUpsampleLayer {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MLCUpsampleLayer {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MLCPaddingLayer {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MLCPaddingLayer {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MLCTransposeLayer {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MLCTransposeLayer {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MLCReductionLayer {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MLCReductionLayer {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MLCMultiheadAttentionLayer {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MLCMultiheadAttentionLayer {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MLCSplitLayer {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MLCSplitLayer {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MLCMatMulLayer {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MLCMatMulLayer {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MLCSliceLayer {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MLCSliceLayer {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MLCEmbeddingLayer {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MLCEmbeddingLayer {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MLCSelectionLayer {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MLCSelectionLayer {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MLCGatherLayer {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MLCGatherLayer {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MLCScatterLayer {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MLCScatterLayer {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MLCOptimizer {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MLCOptimizer {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MLCSGDOptimizer {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MLCSGDOptimizer {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MLCAdamOptimizer {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MLCAdamOptimizer {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MLCAdamWOptimizer {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MLCAdamWOptimizer {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MLCRMSPropOptimizer {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MLCRMSPropOptimizer {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
