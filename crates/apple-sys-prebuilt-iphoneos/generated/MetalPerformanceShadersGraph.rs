#[allow(unused_imports)]
use crate::objc::*;
#[allow(unused_imports)]
use crate::Foundation::*;
#[allow(unused_imports)]
use crate::MetalPerformanceShaders::*;
#[allow(unused_imports)]
use crate::Security::*;

#[allow(unused_imports)]
use objc2::msg_send;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MPSGraphObject(pub id);
impl std::ops::Deref for MPSGraphObject {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MPSGraphObject {}
impl MPSGraphObject {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MPSGraphObject").unwrap(), alloc) })
    }
}
impl INSObject for MPSGraphObject {}
impl PNSObject for MPSGraphObject {}
impl std::convert::TryFrom<NSObject> for MPSGraphObject {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<MPSGraphObject, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MPSGraphObject").unwrap()) };
        if is_kind_of {
            Ok(MPSGraphObject(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to MPSGraphObject")
        }
    }
}
impl IMPSGraphObject for MPSGraphObject {}
pub trait IMPSGraphObject: Sized + std::ops::Deref {}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MPSGraphType(pub id);
impl std::ops::Deref for MPSGraphType {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MPSGraphType {}
impl MPSGraphType {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MPSGraphType").unwrap(), alloc) })
    }
}
impl PNSCopying for MPSGraphType {}
impl IMPSGraphObject for MPSGraphType {}
impl From<MPSGraphType> for MPSGraphObject {
    fn from(child: MPSGraphType) -> MPSGraphObject {
        MPSGraphObject(child.0)
    }
}
impl std::convert::TryFrom<MPSGraphObject> for MPSGraphType {
    type Error = &'static str;
    fn try_from(parent: MPSGraphObject) -> Result<MPSGraphType, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MPSGraphType").unwrap()) };
        if is_kind_of {
            Ok(MPSGraphType(parent.0))
        } else {
            Err("This MPSGraphObject cannot be downcasted to MPSGraphType")
        }
    }
}
impl INSObject for MPSGraphType {}
impl PNSObject for MPSGraphType {}
impl IMPSGraphType for MPSGraphType {}
pub trait IMPSGraphType: Sized + std::ops::Deref {}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MPSGraphShapedType(pub id);
impl std::ops::Deref for MPSGraphShapedType {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MPSGraphShapedType {}
impl MPSGraphShapedType {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MPSGraphShapedType").unwrap(), alloc) })
    }
}
impl IMPSGraphType for MPSGraphShapedType {}
impl PNSCopying for MPSGraphShapedType {}
impl From<MPSGraphShapedType> for MPSGraphType {
    fn from(child: MPSGraphShapedType) -> MPSGraphType {
        MPSGraphType(child.0)
    }
}
impl std::convert::TryFrom<MPSGraphType> for MPSGraphShapedType {
    type Error = &'static str;
    fn try_from(parent: MPSGraphType) -> Result<MPSGraphShapedType, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MPSGraphShapedType").unwrap()) };
        if is_kind_of {
            Ok(MPSGraphShapedType(parent.0))
        } else {
            Err("This MPSGraphType cannot be downcasted to MPSGraphShapedType")
        }
    }
}
impl IMPSGraphObject for MPSGraphShapedType {}
impl INSObject for MPSGraphShapedType {}
impl PNSObject for MPSGraphShapedType {}
impl IMPSGraphShapedType for MPSGraphShapedType {}
pub trait IMPSGraphShapedType: Sized + std::ops::Deref {
    unsafe fn initWithShape_dataType_(&self, shape: NSArray, dataType: MPSDataType) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithShape : shape, dataType : dataType)
    }
    unsafe fn isEqualTo_(&self, object: MPSGraphShapedType) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, isEqualTo : object)
    }
    unsafe fn shape(&self) -> *mut MPSShape
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, shape)
    }
    unsafe fn setShape_(&self, shape: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setShape : shape)
    }
    unsafe fn dataType(&self) -> MPSDataType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, dataType)
    }
    unsafe fn setDataType_(&self, dataType: MPSDataType)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDataType : dataType)
    }
}
pub type MPSGraphTensorNamedDataLayout = NSUInteger;
pub type MPSGraphPaddingStyle = NSUInteger;
pub type MPSGraphPaddingMode = NSInteger;
pub type MPSGraphReductionMode = NSUInteger;
pub type MPSGraphDeviceType = u32;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MPSGraphDevice(pub id);
impl std::ops::Deref for MPSGraphDevice {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MPSGraphDevice {}
impl MPSGraphDevice {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MPSGraphDevice").unwrap(), alloc) })
    }
}
impl IMPSGraphObject for MPSGraphDevice {}
impl std::convert::TryFrom<MPSGraphObject> for MPSGraphDevice {
    type Error = &'static str;
    fn try_from(parent: MPSGraphObject) -> Result<MPSGraphDevice, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MPSGraphDevice").unwrap()) };
        if is_kind_of {
            Ok(MPSGraphDevice(parent.0))
        } else {
            Err("This MPSGraphObject cannot be downcasted to MPSGraphDevice")
        }
    }
}
impl INSObject for MPSGraphDevice {}
impl PNSObject for MPSGraphDevice {}
impl IMPSGraphDevice for MPSGraphDevice {}
pub trait IMPSGraphDevice: Sized + std::ops::Deref {
    unsafe fn type_(&self) -> MPSGraphDeviceType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, type)
    }
    unsafe fn metalDevice(&self) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, metalDevice)
    }
    unsafe fn deviceWithMTLDevice_(metalDevice: *mut u64) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MPSGraphDevice").unwrap(), deviceWithMTLDevice : metalDevice)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MPSGraphTensor(pub id);
impl std::ops::Deref for MPSGraphTensor {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MPSGraphTensor {}
impl MPSGraphTensor {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MPSGraphTensor").unwrap(), alloc) })
    }
}
impl PNSCopying for MPSGraphTensor {}
impl IMPSGraphObject for MPSGraphTensor {}
impl std::convert::TryFrom<MPSGraphObject> for MPSGraphTensor {
    type Error = &'static str;
    fn try_from(parent: MPSGraphObject) -> Result<MPSGraphTensor, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MPSGraphTensor").unwrap()) };
        if is_kind_of {
            Ok(MPSGraphTensor(parent.0))
        } else {
            Err("This MPSGraphObject cannot be downcasted to MPSGraphTensor")
        }
    }
}
impl INSObject for MPSGraphTensor {}
impl PNSObject for MPSGraphTensor {}
impl IMPSGraphTensor for MPSGraphTensor {}
pub trait IMPSGraphTensor: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn shape(&self) -> *mut MPSShape
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, shape)
    }
    unsafe fn dataType(&self) -> MPSDataType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, dataType)
    }
    unsafe fn operation(&self) -> MPSGraphOperation
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, operation)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MPSGraphTensorData(pub id);
impl std::ops::Deref for MPSGraphTensorData {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MPSGraphTensorData {}
impl MPSGraphTensorData {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MPSGraphTensorData").unwrap(), alloc) })
    }
}
impl IMPSGraphObject for MPSGraphTensorData {}
impl std::convert::TryFrom<MPSGraphObject> for MPSGraphTensorData {
    type Error = &'static str;
    fn try_from(parent: MPSGraphObject) -> Result<MPSGraphTensorData, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MPSGraphTensorData").unwrap()) };
        if is_kind_of {
            Ok(MPSGraphTensorData(parent.0))
        } else {
            Err("This MPSGraphObject cannot be downcasted to MPSGraphTensorData")
        }
    }
}
impl INSObject for MPSGraphTensorData {}
impl PNSObject for MPSGraphTensorData {}
impl IMPSGraphTensorData for MPSGraphTensorData {}
pub trait IMPSGraphTensorData: Sized + std::ops::Deref {
    unsafe fn initWithDevice_data_shape_dataType_(
        &self,
        device: MPSGraphDevice,
        data: NSData,
        shape: NSArray,
        dataType: MPSDataType,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithDevice : device, data : data, shape : shape, dataType : dataType)
    }
    unsafe fn initWithMTLBuffer_shape_dataType_(
        &self,
        buffer: *mut u64,
        shape: NSArray,
        dataType: MPSDataType,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithMTLBuffer : buffer, shape : shape, dataType : dataType)
    }
    unsafe fn initWithMTLBuffer_shape_dataType_rowBytes_(
        &self,
        buffer: *mut u64,
        shape: NSArray,
        dataType: MPSDataType,
        rowBytes: NSUInteger,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithMTLBuffer : buffer, shape : shape, dataType : dataType, rowBytes : rowBytes)
    }
    unsafe fn initWithMPSMatrix_(&self, matrix: MPSMatrix) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithMPSMatrix : matrix)
    }
    unsafe fn initWithMPSMatrix_rank_(&self, matrix: MPSMatrix, rank: NSUInteger) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithMPSMatrix : matrix, rank : rank)
    }
    unsafe fn initWithMPSVector_(&self, vector: MPSVector) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithMPSVector : vector)
    }
    unsafe fn initWithMPSVector_rank_(&self, vector: MPSVector, rank: NSUInteger) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithMPSVector : vector, rank : rank)
    }
    unsafe fn initWithMPSNDArray_(&self, ndarray: MPSNDArray) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithMPSNDArray : ndarray)
    }
    unsafe fn initWithMPSImageBatch_(&self, imageBatch: NSArray) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithMPSImageBatch : imageBatch)
    }
    unsafe fn initWithMTLTensor_(&self, tensor: *mut u64) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithMTLTensor : tensor)
    }
    unsafe fn mpsndarray(&self) -> MPSNDArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, mpsndarray)
    }
    unsafe fn shape(&self) -> *mut MPSShape
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, shape)
    }
    unsafe fn dataType(&self) -> MPSDataType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, dataType)
    }
    unsafe fn device(&self) -> MPSGraphDevice
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, device)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MPSGraphOperation(pub id);
impl std::ops::Deref for MPSGraphOperation {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MPSGraphOperation {}
impl MPSGraphOperation {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MPSGraphOperation").unwrap(), alloc) })
    }
}
impl PNSCopying for MPSGraphOperation {}
impl IMPSGraphObject for MPSGraphOperation {}
impl std::convert::TryFrom<MPSGraphObject> for MPSGraphOperation {
    type Error = &'static str;
    fn try_from(parent: MPSGraphObject) -> Result<MPSGraphOperation, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MPSGraphOperation").unwrap()) };
        if is_kind_of {
            Ok(MPSGraphOperation(parent.0))
        } else {
            Err("This MPSGraphObject cannot be downcasted to MPSGraphOperation")
        }
    }
}
impl INSObject for MPSGraphOperation {}
impl PNSObject for MPSGraphOperation {}
impl IMPSGraphOperation for MPSGraphOperation {}
pub trait IMPSGraphOperation: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn inputTensors(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inputTensors)
    }
    unsafe fn outputTensors(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, outputTensors)
    }
    unsafe fn controlDependencies(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, controlDependencies)
    }
    unsafe fn graph(&self) -> MPSGraph
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, graph)
    }
    unsafe fn name(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, name)
    }
}
pub type MPSGraphOptions = u64;
pub type MPSGraphOptimization = u64;
pub type MPSGraphOptimizationProfile = u64;
pub type MPSGraphExecutionStage = u64;
pub type MPSGraphReducedPrecisionFastMath = NSUInteger;
pub type MPSGraphTensorDataDictionary = NSDictionary;
pub type MPSGraphTensorShapedTypeDictionary = NSDictionary;
pub type MPSGraphCompletionHandler = *mut ::std::os::raw::c_void;
pub type MPSGraphScheduledHandler = *mut ::std::os::raw::c_void;
pub type MPSGraphCompilationCompletionHandler = *mut ::std::os::raw::c_void;
pub type MPSGraphCallableMap = NSDictionary;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MPSGraphCompilationDescriptor(pub id);
impl std::ops::Deref for MPSGraphCompilationDescriptor {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MPSGraphCompilationDescriptor {}
impl MPSGraphCompilationDescriptor {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MPSGraphCompilationDescriptor").unwrap(), alloc) })
    }
}
impl PNSCopying for MPSGraphCompilationDescriptor {}
impl IMPSGraphObject for MPSGraphCompilationDescriptor {}
impl std::convert::TryFrom<MPSGraphObject> for MPSGraphCompilationDescriptor {
    type Error = &'static str;
    fn try_from(parent: MPSGraphObject) -> Result<MPSGraphCompilationDescriptor, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MPSGraphCompilationDescriptor").unwrap())
        };
        if is_kind_of {
            Ok(MPSGraphCompilationDescriptor(parent.0))
        } else {
            Err("This MPSGraphObject cannot be downcasted to MPSGraphCompilationDescriptor")
        }
    }
}
impl INSObject for MPSGraphCompilationDescriptor {}
impl PNSObject for MPSGraphCompilationDescriptor {}
impl IMPSGraphCompilationDescriptor for MPSGraphCompilationDescriptor {}
pub trait IMPSGraphCompilationDescriptor: Sized + std::ops::Deref {
    unsafe fn disableTypeInference(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, disableTypeInference)
    }
    unsafe fn optimizationLevel(&self) -> MPSGraphOptimization
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, optimizationLevel)
    }
    unsafe fn setOptimizationLevel_(&self, optimizationLevel: MPSGraphOptimization)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setOptimizationLevel : optimizationLevel)
    }
    unsafe fn waitForCompilationCompletion(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, waitForCompilationCompletion)
    }
    unsafe fn setWaitForCompilationCompletion_(&self, waitForCompilationCompletion: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setWaitForCompilationCompletion : waitForCompilationCompletion)
    }
    unsafe fn compilationCompletionHandler(&self) -> MPSGraphCompilationCompletionHandler
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, compilationCompletionHandler)
    }
    unsafe fn setCompilationCompletionHandler_(
        &self,
        compilationCompletionHandler: MPSGraphCompilationCompletionHandler,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCompilationCompletionHandler : compilationCompletionHandler)
    }
    unsafe fn dispatchQueue(&self) -> dispatch_queue_t
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, dispatchQueue)
    }
    unsafe fn setDispatchQueue_(&self, dispatchQueue: NSObject)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDispatchQueue : dispatchQueue)
    }
    unsafe fn optimizationProfile(&self) -> MPSGraphOptimizationProfile
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, optimizationProfile)
    }
    unsafe fn setOptimizationProfile_(&self, optimizationProfile: MPSGraphOptimizationProfile)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setOptimizationProfile : optimizationProfile)
    }
    unsafe fn callables(&self) -> *mut MPSGraphCallableMap
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, callables)
    }
    unsafe fn setCallables_(&self, callables: NSDictionary)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCallables : callables)
    }
    unsafe fn reducedPrecisionFastMath(&self) -> MPSGraphReducedPrecisionFastMath
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, reducedPrecisionFastMath)
    }
    unsafe fn setReducedPrecisionFastMath_(
        &self,
        reducedPrecisionFastMath: MPSGraphReducedPrecisionFastMath,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setReducedPrecisionFastMath : reducedPrecisionFastMath)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MPSGraphExecutionDescriptor(pub id);
impl std::ops::Deref for MPSGraphExecutionDescriptor {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MPSGraphExecutionDescriptor {}
impl MPSGraphExecutionDescriptor {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MPSGraphExecutionDescriptor").unwrap(), alloc) })
    }
}
impl IMPSGraphObject for MPSGraphExecutionDescriptor {}
impl std::convert::TryFrom<MPSGraphObject> for MPSGraphExecutionDescriptor {
    type Error = &'static str;
    fn try_from(parent: MPSGraphObject) -> Result<MPSGraphExecutionDescriptor, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MPSGraphExecutionDescriptor").unwrap()) };
        if is_kind_of {
            Ok(MPSGraphExecutionDescriptor(parent.0))
        } else {
            Err("This MPSGraphObject cannot be downcasted to MPSGraphExecutionDescriptor")
        }
    }
}
impl INSObject for MPSGraphExecutionDescriptor {}
impl PNSObject for MPSGraphExecutionDescriptor {}
impl IMPSGraphExecutionDescriptor for MPSGraphExecutionDescriptor {}
pub trait IMPSGraphExecutionDescriptor: Sized + std::ops::Deref {
    unsafe fn waitForEvent_value_(&self, event: *mut u64, value: u64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, waitForEvent : event, value : value)
    }
    unsafe fn signalEvent_atExecutionEvent_value_(
        &self,
        event: *mut u64,
        executionStage: MPSGraphExecutionStage,
        value: u64,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, signalEvent : event, atExecutionEvent : executionStage, value : value)
    }
    unsafe fn scheduledHandler(&self) -> MPSGraphScheduledHandler
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, scheduledHandler)
    }
    unsafe fn setScheduledHandler_(&self, scheduledHandler: MPSGraphScheduledHandler)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setScheduledHandler : scheduledHandler)
    }
    unsafe fn completionHandler(&self) -> MPSGraphCompletionHandler
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, completionHandler)
    }
    unsafe fn setCompletionHandler_(&self, completionHandler: MPSGraphCompletionHandler)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCompletionHandler : completionHandler)
    }
    unsafe fn waitUntilCompleted(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, waitUntilCompleted)
    }
    unsafe fn setWaitUntilCompleted_(&self, waitUntilCompleted: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setWaitUntilCompleted : waitUntilCompleted)
    }
    unsafe fn compilationDescriptor(&self) -> MPSGraphCompilationDescriptor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, compilationDescriptor)
    }
    unsafe fn setCompilationDescriptor_(&self, compilationDescriptor: MPSGraphCompilationDescriptor)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCompilationDescriptor : compilationDescriptor)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MPSGraph(pub id);
impl std::ops::Deref for MPSGraph {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MPSGraph {}
impl MPSGraph {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MPSGraph").unwrap(), alloc) })
    }
}
impl IMPSGraphObject for MPSGraph {}
impl std::convert::TryFrom<MPSGraphObject> for MPSGraph {
    type Error = &'static str;
    fn try_from(parent: MPSGraphObject) -> Result<MPSGraph, Self::Error> {
        let is_kind_of: bool = unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MPSGraph").unwrap()) };
        if is_kind_of {
            Ok(MPSGraph(parent.0))
        } else {
            Err("This MPSGraphObject cannot be downcasted to MPSGraph")
        }
    }
}
impl INSObject for MPSGraph {}
impl PNSObject for MPSGraph {}
impl IMPSGraph for MPSGraph {}
pub trait IMPSGraph: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn compileWithDevice_feeds_targetTensors_targetOperations_compilationDescriptor_(
        &self,
        device: MPSGraphDevice,
        feeds: NSDictionary,
        targetTensors: NSArray,
        targetOperations: NSArray,
        compilationDescriptor: MPSGraphCompilationDescriptor,
    ) -> MPSGraphExecutable
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, compileWithDevice : device, feeds : feeds, targetTensors : targetTensors, targetOperations : targetOperations, compilationDescriptor : compilationDescriptor)
    }
    unsafe fn runWithFeeds_targetTensors_targetOperations_(
        &self,
        feeds: NSDictionary,
        targetTensors: NSArray,
        targetOperations: NSArray,
    ) -> *mut MPSGraphTensorDataDictionary
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, runWithFeeds : feeds, targetTensors : targetTensors, targetOperations : targetOperations)
    }
    unsafe fn runWithMTLCommandQueue_feeds_targetTensors_targetOperations_(
        &self,
        commandQueue: *mut u64,
        feeds: NSDictionary,
        targetTensors: NSArray,
        targetOperations: NSArray,
    ) -> *mut MPSGraphTensorDataDictionary
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, runWithMTLCommandQueue : commandQueue, feeds : feeds, targetTensors : targetTensors, targetOperations : targetOperations)
    }
    unsafe fn runWithMTLCommandQueue_feeds_targetOperations_resultsDictionary_(
        &self,
        commandQueue: *mut u64,
        feeds: NSDictionary,
        targetOperations: NSArray,
        resultsDictionary: NSDictionary,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, runWithMTLCommandQueue : commandQueue, feeds : feeds, targetOperations : targetOperations, resultsDictionary : resultsDictionary)
    }
    unsafe fn runAsyncWithFeeds_targetTensors_targetOperations_executionDescriptor_(
        &self,
        feeds: NSDictionary,
        targetTensors: NSArray,
        targetOperations: NSArray,
        executionDescriptor: MPSGraphExecutionDescriptor,
    ) -> *mut MPSGraphTensorDataDictionary
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, runAsyncWithFeeds : feeds, targetTensors : targetTensors, targetOperations : targetOperations, executionDescriptor : executionDescriptor)
    }
    unsafe fn runAsyncWithMTLCommandQueue_feeds_targetTensors_targetOperations_executionDescriptor_(
        &self,
        commandQueue: *mut u64,
        feeds: NSDictionary,
        targetTensors: NSArray,
        targetOperations: NSArray,
        executionDescriptor: MPSGraphExecutionDescriptor,
    ) -> *mut MPSGraphTensorDataDictionary
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, runAsyncWithMTLCommandQueue : commandQueue, feeds : feeds, targetTensors : targetTensors, targetOperations : targetOperations, executionDescriptor : executionDescriptor)
    }
    unsafe fn runAsyncWithMTLCommandQueue_feeds_targetOperations_resultsDictionary_executionDescriptor_(
        &self,
        commandQueue: *mut u64,
        feeds: NSDictionary,
        targetOperations: NSArray,
        resultsDictionary: NSDictionary,
        executionDescriptor: MPSGraphExecutionDescriptor,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, runAsyncWithMTLCommandQueue : commandQueue, feeds : feeds, targetOperations : targetOperations, resultsDictionary : resultsDictionary, executionDescriptor : executionDescriptor)
    }
    unsafe fn encodeToCommandBuffer_feeds_targetTensors_targetOperations_executionDescriptor_(
        &self,
        commandBuffer: MPSCommandBuffer,
        feeds: NSDictionary,
        targetTensors: NSArray,
        targetOperations: NSArray,
        executionDescriptor: MPSGraphExecutionDescriptor,
    ) -> *mut MPSGraphTensorDataDictionary
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, encodeToCommandBuffer : commandBuffer, feeds : feeds, targetTensors : targetTensors, targetOperations : targetOperations, executionDescriptor : executionDescriptor)
    }
    unsafe fn encodeToCommandBuffer_feeds_targetOperations_resultsDictionary_executionDescriptor_(
        &self,
        commandBuffer: MPSCommandBuffer,
        feeds: NSDictionary,
        targetOperations: NSArray,
        resultsDictionary: NSDictionary,
        executionDescriptor: MPSGraphExecutionDescriptor,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, encodeToCommandBuffer : commandBuffer, feeds : feeds, targetOperations : targetOperations, resultsDictionary : resultsDictionary, executionDescriptor : executionDescriptor)
    }
    unsafe fn options(&self) -> MPSGraphOptions
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, options)
    }
    unsafe fn setOptions_(&self, options: MPSGraphOptions)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setOptions : options)
    }
    unsafe fn placeholderTensors(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, placeholderTensors)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MPSGraph").unwrap(), new)
    }
}
pub type MPSGraphExecutableCompletionHandler = *mut ::std::os::raw::c_void;
pub type MPSGraphExecutableScheduledHandler = *mut ::std::os::raw::c_void;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MPSGraphExecutableExecutionDescriptor(pub id);
impl std::ops::Deref for MPSGraphExecutableExecutionDescriptor {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MPSGraphExecutableExecutionDescriptor {}
impl MPSGraphExecutableExecutionDescriptor {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MPSGraphExecutableExecutionDescriptor").unwrap(), alloc) })
    }
}
impl PNSCopying for MPSGraphExecutableExecutionDescriptor {}
impl IMPSGraphObject for MPSGraphExecutableExecutionDescriptor {}
impl std::convert::TryFrom<MPSGraphObject> for MPSGraphExecutableExecutionDescriptor {
    type Error = &'static str;
    fn try_from(
        parent: MPSGraphObject,
    ) -> Result<MPSGraphExecutableExecutionDescriptor, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MPSGraphExecutableExecutionDescriptor").unwrap())
        };
        if is_kind_of {
            Ok(MPSGraphExecutableExecutionDescriptor(parent.0))
        } else {
            Err("This MPSGraphObject cannot be downcasted to MPSGraphExecutableExecutionDescriptor")
        }
    }
}
impl INSObject for MPSGraphExecutableExecutionDescriptor {}
impl PNSObject for MPSGraphExecutableExecutionDescriptor {}
impl IMPSGraphExecutableExecutionDescriptor for MPSGraphExecutableExecutionDescriptor {}
pub trait IMPSGraphExecutableExecutionDescriptor: Sized + std::ops::Deref {
    unsafe fn waitForEvent_value_(&self, event: *mut u64, value: u64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, waitForEvent : event, value : value)
    }
    unsafe fn signalEvent_atExecutionEvent_value_(
        &self,
        event: *mut u64,
        executionStage: MPSGraphExecutionStage,
        value: u64,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, signalEvent : event, atExecutionEvent : executionStage, value : value)
    }
    unsafe fn scheduledHandler(&self) -> MPSGraphExecutableScheduledHandler
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, scheduledHandler)
    }
    unsafe fn setScheduledHandler_(&self, scheduledHandler: MPSGraphExecutableScheduledHandler)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setScheduledHandler : scheduledHandler)
    }
    unsafe fn completionHandler(&self) -> MPSGraphExecutableCompletionHandler
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, completionHandler)
    }
    unsafe fn setCompletionHandler_(&self, completionHandler: MPSGraphExecutableCompletionHandler)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCompletionHandler : completionHandler)
    }
    unsafe fn waitUntilCompleted(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, waitUntilCompleted)
    }
    unsafe fn setWaitUntilCompleted_(&self, waitUntilCompleted: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setWaitUntilCompleted : waitUntilCompleted)
    }
}
pub type MPSGraphDeploymentPlatform = u64;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MPSGraphExecutableSerializationDescriptor(pub id);
impl std::ops::Deref for MPSGraphExecutableSerializationDescriptor {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MPSGraphExecutableSerializationDescriptor {}
impl MPSGraphExecutableSerializationDescriptor {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MPSGraphExecutableSerializationDescriptor").unwrap(), alloc) })
    }
}
impl IMPSGraphObject for MPSGraphExecutableSerializationDescriptor {}
impl std::convert::TryFrom<MPSGraphObject> for MPSGraphExecutableSerializationDescriptor {
    type Error = &'static str;
    fn try_from(
        parent: MPSGraphObject,
    ) -> Result<MPSGraphExecutableSerializationDescriptor, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MPSGraphExecutableSerializationDescriptor").unwrap())
        };
        if is_kind_of {
            Ok(MPSGraphExecutableSerializationDescriptor(parent.0))
        } else {
            Err ("This MPSGraphObject cannot be downcasted to MPSGraphExecutableSerializationDescriptor" ,)
        }
    }
}
impl INSObject for MPSGraphExecutableSerializationDescriptor {}
impl PNSObject for MPSGraphExecutableSerializationDescriptor {}
impl IMPSGraphExecutableSerializationDescriptor for MPSGraphExecutableSerializationDescriptor {}
pub trait IMPSGraphExecutableSerializationDescriptor: Sized + std::ops::Deref {
    unsafe fn append(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, append)
    }
    unsafe fn setAppend_(&self, append: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAppend : append)
    }
    unsafe fn deploymentPlatform(&self) -> MPSGraphDeploymentPlatform
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, deploymentPlatform)
    }
    unsafe fn setDeploymentPlatform_(&self, deploymentPlatform: MPSGraphDeploymentPlatform)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDeploymentPlatform : deploymentPlatform)
    }
    unsafe fn minimumDeploymentTarget(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, minimumDeploymentTarget)
    }
    unsafe fn setMinimumDeploymentTarget_(&self, minimumDeploymentTarget: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMinimumDeploymentTarget : minimumDeploymentTarget)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MPSGraphExecutable(pub id);
impl std::ops::Deref for MPSGraphExecutable {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MPSGraphExecutable {}
impl MPSGraphExecutable {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MPSGraphExecutable").unwrap(), alloc) })
    }
}
impl IMPSGraphObject for MPSGraphExecutable {}
impl std::convert::TryFrom<MPSGraphObject> for MPSGraphExecutable {
    type Error = &'static str;
    fn try_from(parent: MPSGraphObject) -> Result<MPSGraphExecutable, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MPSGraphExecutable").unwrap()) };
        if is_kind_of {
            Ok(MPSGraphExecutable(parent.0))
        } else {
            Err("This MPSGraphObject cannot be downcasted to MPSGraphExecutable")
        }
    }
}
impl INSObject for MPSGraphExecutable {}
impl PNSObject for MPSGraphExecutable {}
impl IMPSGraphExecutable for MPSGraphExecutable {}
pub trait IMPSGraphExecutable: Sized + std::ops::Deref {
    unsafe fn specializeWithDevice_inputTypes_compilationDescriptor_(
        &self,
        device: MPSGraphDevice,
        inputTypes: NSArray,
        compilationDescriptor: MPSGraphCompilationDescriptor,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, specializeWithDevice : device, inputTypes : inputTypes, compilationDescriptor : compilationDescriptor)
    }
    unsafe fn getOutputTypesWithDevice_inputTypes_compilationDescriptor_(
        &self,
        device: MPSGraphDevice,
        inputTypes: NSArray,
        compilationDescriptor: MPSGraphCompilationDescriptor,
    ) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, getOutputTypesWithDevice : device, inputTypes : inputTypes, compilationDescriptor : compilationDescriptor)
    }
    unsafe fn runWithMTLCommandQueue_inputsArray_resultsArray_executionDescriptor_(
        &self,
        commandQueue: *mut u64,
        inputsArray: NSArray,
        resultsArray: NSArray,
        executionDescriptor: MPSGraphExecutableExecutionDescriptor,
    ) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, runWithMTLCommandQueue : commandQueue, inputsArray : inputsArray, resultsArray : resultsArray, executionDescriptor : executionDescriptor)
    }
    unsafe fn runAsyncWithMTLCommandQueue_inputsArray_resultsArray_executionDescriptor_(
        &self,
        commandQueue: *mut u64,
        inputsArray: NSArray,
        resultsArray: NSArray,
        executionDescriptor: MPSGraphExecutableExecutionDescriptor,
    ) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, runAsyncWithMTLCommandQueue : commandQueue, inputsArray : inputsArray, resultsArray : resultsArray, executionDescriptor : executionDescriptor)
    }
    unsafe fn encodeToCommandBuffer_inputsArray_resultsArray_executionDescriptor_(
        &self,
        commandBuffer: MPSCommandBuffer,
        inputsArray: NSArray,
        resultsArray: NSArray,
        executionDescriptor: MPSGraphExecutableExecutionDescriptor,
    ) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, encodeToCommandBuffer : commandBuffer, inputsArray : inputsArray, resultsArray : resultsArray, executionDescriptor : executionDescriptor)
    }
    unsafe fn serializeToMPSGraphPackageAtURL_descriptor_(
        &self,
        url: NSURL,
        descriptor: MPSGraphExecutableSerializationDescriptor,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, serializeToMPSGraphPackageAtURL : url, descriptor : descriptor)
    }
    unsafe fn initWithMPSGraphPackageAtURL_compilationDescriptor_(
        &self,
        mpsgraphPackageURL: NSURL,
        compilationDescriptor: MPSGraphCompilationDescriptor,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithMPSGraphPackageAtURL : mpsgraphPackageURL, compilationDescriptor : compilationDescriptor)
    }
    unsafe fn initWithCoreMLPackageAtURL_compilationDescriptor_(
        &self,
        coreMLPackageURL: NSURL,
        compilationDescriptor: MPSGraphCompilationDescriptor,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithCoreMLPackageAtURL : coreMLPackageURL, compilationDescriptor : compilationDescriptor)
    }
    unsafe fn options(&self) -> MPSGraphOptions
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, options)
    }
    unsafe fn setOptions_(&self, options: MPSGraphOptions)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setOptions : options)
    }
    unsafe fn feedTensors(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, feedTensors)
    }
    unsafe fn targetTensors(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, targetTensors)
    }
}
impl MPSGraph_MPSGraphGradientOps for MPSGraph {}
pub trait MPSGraph_MPSGraphGradientOps: Sized + std::ops::Deref {
    unsafe fn gradientForPrimaryTensor_withTensors_name_(
        &self,
        primaryTensor: MPSGraphTensor,
        tensors: NSArray,
        name: NSString,
    ) -> NSDictionary
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, gradientForPrimaryTensor : primaryTensor, withTensors : tensors, name : name)
    }
}
impl MPSGraph_MPSGraphActivationOps for MPSGraph {}
pub trait MPSGraph_MPSGraphActivationOps: Sized + std::ops::Deref {
    unsafe fn reLUWithTensor_name_(&self, tensor: MPSGraphTensor, name: NSString) -> MPSGraphTensor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, reLUWithTensor : tensor, name : name)
    }
    unsafe fn reLUGradientWithIncomingGradient_sourceTensor_name_(
        &self,
        gradient: MPSGraphTensor,
        source: MPSGraphTensor,
        name: NSString,
    ) -> MPSGraphTensor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, reLUGradientWithIncomingGradient : gradient, sourceTensor : source, name : name)
    }
    unsafe fn sigmoidWithTensor_name_(
        &self,
        tensor: MPSGraphTensor,
        name: NSString,
    ) -> MPSGraphTensor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, sigmoidWithTensor : tensor, name : name)
    }
    unsafe fn sigmoidGradientWithIncomingGradient_sourceTensor_name_(
        &self,
        gradient: MPSGraphTensor,
        source: MPSGraphTensor,
        name: NSString,
    ) -> MPSGraphTensor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, sigmoidGradientWithIncomingGradient : gradient, sourceTensor : source, name : name)
    }
    unsafe fn softMaxWithTensor_axis_name_(
        &self,
        tensor: MPSGraphTensor,
        axis: NSInteger,
        name: NSString,
    ) -> MPSGraphTensor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, softMaxWithTensor : tensor, axis : axis, name : name)
    }
    unsafe fn softMaxGradientWithIncomingGradient_sourceTensor_axis_name_(
        &self,
        gradient: MPSGraphTensor,
        source: MPSGraphTensor,
        axis: NSInteger,
        name: NSString,
    ) -> MPSGraphTensor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, softMaxGradientWithIncomingGradient : gradient, sourceTensor : source, axis : axis, name : name)
    }
    unsafe fn leakyReLUWithTensor_alpha_name_(
        &self,
        tensor: MPSGraphTensor,
        alpha: f64,
        name: NSString,
    ) -> MPSGraphTensor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, leakyReLUWithTensor : tensor, alpha : alpha, name : name)
    }
    unsafe fn leakyReLUWithTensor_alphaTensor_name_(
        &self,
        tensor: MPSGraphTensor,
        alphaTensor: MPSGraphTensor,
        name: NSString,
    ) -> MPSGraphTensor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, leakyReLUWithTensor : tensor, alphaTensor : alphaTensor, name : name)
    }
    unsafe fn leakyReLUGradientWithIncomingGradient_sourceTensor_alphaTensor_name_(
        &self,
        gradient: MPSGraphTensor,
        source: MPSGraphTensor,
        alphaTensor: MPSGraphTensor,
        name: NSString,
    ) -> MPSGraphTensor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, leakyReLUGradientWithIncomingGradient : gradient, sourceTensor : source, alphaTensor : alphaTensor, name : name)
    }
}
impl MPSGraph_MPSGraphArithmeticOps for MPSGraph {}
pub trait MPSGraph_MPSGraphArithmeticOps: Sized + std::ops::Deref {
    unsafe fn identityWithTensor_name_(
        &self,
        tensor: MPSGraphTensor,
        name: NSString,
    ) -> MPSGraphTensor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, identityWithTensor : tensor, name : name)
    }
    unsafe fn exponentWithTensor_name_(
        &self,
        tensor: MPSGraphTensor,
        name: NSString,
    ) -> MPSGraphTensor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, exponentWithTensor : tensor, name : name)
    }
    unsafe fn exponentBase2WithTensor_name_(
        &self,
        tensor: MPSGraphTensor,
        name: NSString,
    ) -> MPSGraphTensor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, exponentBase2WithTensor : tensor, name : name)
    }
    unsafe fn exponentBase10WithTensor_name_(
        &self,
        tensor: MPSGraphTensor,
        name: NSString,
    ) -> MPSGraphTensor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, exponentBase10WithTensor : tensor, name : name)
    }
    unsafe fn logarithmWithTensor_name_(
        &self,
        tensor: MPSGraphTensor,
        name: NSString,
    ) -> MPSGraphTensor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, logarithmWithTensor : tensor, name : name)
    }
    unsafe fn logarithmBase2WithTensor_name_(
        &self,
        tensor: MPSGraphTensor,
        name: NSString,
    ) -> MPSGraphTensor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, logarithmBase2WithTensor : tensor, name : name)
    }
    unsafe fn logarithmBase10WithTensor_name_(
        &self,
        tensor: MPSGraphTensor,
        name: NSString,
    ) -> MPSGraphTensor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, logarithmBase10WithTensor : tensor, name : name)
    }
    unsafe fn squareWithTensor_name_(
        &self,
        tensor: MPSGraphTensor,
        name: NSString,
    ) -> MPSGraphTensor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, squareWithTensor : tensor, name : name)
    }
    unsafe fn squareRootWithTensor_name_(
        &self,
        tensor: MPSGraphTensor,
        name: NSString,
    ) -> MPSGraphTensor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, squareRootWithTensor : tensor, name : name)
    }
    unsafe fn reciprocalSquareRootWithTensor_name_(
        &self,
        tensor: MPSGraphTensor,
        name: NSString,
    ) -> MPSGraphTensor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, reciprocalSquareRootWithTensor : tensor, name : name)
    }
    unsafe fn reverseSquareRootWithTensor_name_(
        &self,
        tensor: MPSGraphTensor,
        name: NSString,
    ) -> MPSGraphTensor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, reverseSquareRootWithTensor : tensor, name : name)
    }
    unsafe fn reciprocalWithTensor_name_(
        &self,
        tensor: MPSGraphTensor,
        name: NSString,
    ) -> MPSGraphTensor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, reciprocalWithTensor : tensor, name : name)
    }
    unsafe fn absoluteWithTensor_name_(
        &self,
        tensor: MPSGraphTensor,
        name: NSString,
    ) -> MPSGraphTensor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, absoluteWithTensor : tensor, name : name)
    }
    unsafe fn absoluteSquareWithTensor_name_(
        &self,
        tensor: MPSGraphTensor,
        name: NSString,
    ) -> MPSGraphTensor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, absoluteSquareWithTensor : tensor, name : name)
    }
    unsafe fn negativeWithTensor_name_(
        &self,
        tensor: MPSGraphTensor,
        name: NSString,
    ) -> MPSGraphTensor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, negativeWithTensor : tensor, name : name)
    }
    unsafe fn signWithTensor_name_(&self, tensor: MPSGraphTensor, name: NSString) -> MPSGraphTensor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, signWithTensor : tensor, name : name)
    }
    unsafe fn signbitWithTensor_name_(
        &self,
        tensor: MPSGraphTensor,
        name: NSString,
    ) -> MPSGraphTensor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, signbitWithTensor : tensor, name : name)
    }
    unsafe fn ceilWithTensor_name_(&self, tensor: MPSGraphTensor, name: NSString) -> MPSGraphTensor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, ceilWithTensor : tensor, name : name)
    }
    unsafe fn floorWithTensor_name_(&self, tensor: MPSGraphTensor, name: NSString) -> MPSGraphTensor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, floorWithTensor : tensor, name : name)
    }
    unsafe fn roundWithTensor_name_(&self, tensor: MPSGraphTensor, name: NSString) -> MPSGraphTensor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, roundWithTensor : tensor, name : name)
    }
    unsafe fn rintWithTensor_name_(&self, tensor: MPSGraphTensor, name: NSString) -> MPSGraphTensor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, rintWithTensor : tensor, name : name)
    }
    unsafe fn sinWithTensor_name_(&self, tensor: MPSGraphTensor, name: NSString) -> MPSGraphTensor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, sinWithTensor : tensor, name : name)
    }
    unsafe fn cosWithTensor_name_(&self, tensor: MPSGraphTensor, name: NSString) -> MPSGraphTensor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, cosWithTensor : tensor, name : name)
    }
    unsafe fn tanWithTensor_name_(&self, tensor: MPSGraphTensor, name: NSString) -> MPSGraphTensor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, tanWithTensor : tensor, name : name)
    }
    unsafe fn sinhWithTensor_name_(&self, tensor: MPSGraphTensor, name: NSString) -> MPSGraphTensor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, sinhWithTensor : tensor, name : name)
    }
    unsafe fn coshWithTensor_name_(&self, tensor: MPSGraphTensor, name: NSString) -> MPSGraphTensor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, coshWithTensor : tensor, name : name)
    }
    unsafe fn tanhWithTensor_name_(&self, tensor: MPSGraphTensor, name: NSString) -> MPSGraphTensor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, tanhWithTensor : tensor, name : name)
    }
    unsafe fn asinWithTensor_name_(&self, tensor: MPSGraphTensor, name: NSString) -> MPSGraphTensor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, asinWithTensor : tensor, name : name)
    }
    unsafe fn acosWithTensor_name_(&self, tensor: MPSGraphTensor, name: NSString) -> MPSGraphTensor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, acosWithTensor : tensor, name : name)
    }
    unsafe fn atanWithTensor_name_(&self, tensor: MPSGraphTensor, name: NSString) -> MPSGraphTensor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, atanWithTensor : tensor, name : name)
    }
    unsafe fn asinhWithTensor_name_(&self, tensor: MPSGraphTensor, name: NSString) -> MPSGraphTensor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, asinhWithTensor : tensor, name : name)
    }
    unsafe fn acoshWithTensor_name_(&self, tensor: MPSGraphTensor, name: NSString) -> MPSGraphTensor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, acoshWithTensor : tensor, name : name)
    }
    unsafe fn atanhWithTensor_name_(&self, tensor: MPSGraphTensor, name: NSString) -> MPSGraphTensor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, atanhWithTensor : tensor, name : name)
    }
    unsafe fn notWithTensor_name_(&self, tensor: MPSGraphTensor, name: NSString) -> MPSGraphTensor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, notWithTensor : tensor, name : name)
    }
    unsafe fn isInfiniteWithTensor_name_(
        &self,
        tensor: MPSGraphTensor,
        name: NSString,
    ) -> MPSGraphTensor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, isInfiniteWithTensor : tensor, name : name)
    }
    unsafe fn isFiniteWithTensor_name_(
        &self,
        tensor: MPSGraphTensor,
        name: NSString,
    ) -> MPSGraphTensor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, isFiniteWithTensor : tensor, name : name)
    }
    unsafe fn isNaNWithTensor_name_(&self, tensor: MPSGraphTensor, name: NSString) -> MPSGraphTensor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, isNaNWithTensor : tensor, name : name)
    }
    unsafe fn erfWithTensor_name_(&self, tensor: MPSGraphTensor, name: NSString) -> MPSGraphTensor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, erfWithTensor : tensor, name : name)
    }
    unsafe fn truncateWithTensor_name_(
        &self,
        tensor: MPSGraphTensor,
        name: NSString,
    ) -> MPSGraphTensor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, truncateWithTensor : tensor, name : name)
    }
    unsafe fn bitwiseNOTWithTensor_name_(
        &self,
        tensor: MPSGraphTensor,
        name: NSString,
    ) -> MPSGraphTensor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, bitwiseNOTWithTensor : tensor, name : name)
    }
    unsafe fn bitwisePopulationCountWithTensor_name_(
        &self,
        tensor: MPSGraphTensor,
        name: NSString,
    ) -> MPSGraphTensor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, bitwisePopulationCountWithTensor : tensor, name : name)
    }
    unsafe fn conjugateWithTensor_name_(
        &self,
        tensor: MPSGraphTensor,
        name: NSString,
    ) -> MPSGraphTensor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, conjugateWithTensor : tensor, name : name)
    }
    unsafe fn additionWithPrimaryTensor_secondaryTensor_name_(
        &self,
        primaryTensor: MPSGraphTensor,
        secondaryTensor: MPSGraphTensor,
        name: NSString,
    ) -> MPSGraphTensor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, additionWithPrimaryTensor : primaryTensor, secondaryTensor : secondaryTensor, name : name)
    }
    unsafe fn subtractionWithPrimaryTensor_secondaryTensor_name_(
        &self,
        primaryTensor: MPSGraphTensor,
        secondaryTensor: MPSGraphTensor,
        name: NSString,
    ) -> MPSGraphTensor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, subtractionWithPrimaryTensor : primaryTensor, secondaryTensor : secondaryTensor, name : name)
    }
    unsafe fn multiplicationWithPrimaryTensor_secondaryTensor_name_(
        &self,
        primaryTensor: MPSGraphTensor,
        secondaryTensor: MPSGraphTensor,
        name: NSString,
    ) -> MPSGraphTensor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, multiplicationWithPrimaryTensor : primaryTensor, secondaryTensor : secondaryTensor, name : name)
    }
    unsafe fn divisionWithPrimaryTensor_secondaryTensor_name_(
        &self,
        primaryTensor: MPSGraphTensor,
        secondaryTensor: MPSGraphTensor,
        name: NSString,
    ) -> MPSGraphTensor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, divisionWithPrimaryTensor : primaryTensor, secondaryTensor : secondaryTensor, name : name)
    }
    unsafe fn moduloWithPrimaryTensor_secondaryTensor_name_(
        &self,
        primaryTensor: MPSGraphTensor,
        secondaryTensor: MPSGraphTensor,
        name: NSString,
    ) -> MPSGraphTensor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, moduloWithPrimaryTensor : primaryTensor, secondaryTensor : secondaryTensor, name : name)
    }
    unsafe fn powerWithPrimaryTensor_secondaryTensor_name_(
        &self,
        primaryTensor: MPSGraphTensor,
        secondaryTensor: MPSGraphTensor,
        name: NSString,
    ) -> MPSGraphTensor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, powerWithPrimaryTensor : primaryTensor, secondaryTensor : secondaryTensor, name : name)
    }
    unsafe fn minimumWithPrimaryTensor_secondaryTensor_name_(
        &self,
        primaryTensor: MPSGraphTensor,
        secondaryTensor: MPSGraphTensor,
        name: NSString,
    ) -> MPSGraphTensor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, minimumWithPrimaryTensor : primaryTensor, secondaryTensor : secondaryTensor, name : name)
    }
    unsafe fn maximumWithPrimaryTensor_secondaryTensor_name_(
        &self,
        primaryTensor: MPSGraphTensor,
        secondaryTensor: MPSGraphTensor,
        name: NSString,
    ) -> MPSGraphTensor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, maximumWithPrimaryTensor : primaryTensor, secondaryTensor : secondaryTensor, name : name)
    }
    unsafe fn minimumWithNaNPropagationWithPrimaryTensor_secondaryTensor_name_(
        &self,
        primaryTensor: MPSGraphTensor,
        secondaryTensor: MPSGraphTensor,
        name: NSString,
    ) -> MPSGraphTensor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, minimumWithNaNPropagationWithPrimaryTensor : primaryTensor, secondaryTensor : secondaryTensor, name : name)
    }
    unsafe fn maximumWithNaNPropagationWithPrimaryTensor_secondaryTensor_name_(
        &self,
        primaryTensor: MPSGraphTensor,
        secondaryTensor: MPSGraphTensor,
        name: NSString,
    ) -> MPSGraphTensor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, maximumWithNaNPropagationWithPrimaryTensor : primaryTensor, secondaryTensor : secondaryTensor, name : name)
    }
    unsafe fn equalWithPrimaryTensor_secondaryTensor_name_(
        &self,
        primaryTensor: MPSGraphTensor,
        secondaryTensor: MPSGraphTensor,
        name: NSString,
    ) -> MPSGraphTensor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, equalWithPrimaryTensor : primaryTensor, secondaryTensor : secondaryTensor, name : name)
    }
    unsafe fn notEqualWithPrimaryTensor_secondaryTensor_name_(
        &self,
        primaryTensor: MPSGraphTensor,
        secondaryTensor: MPSGraphTensor,
        name: NSString,
    ) -> MPSGraphTensor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, notEqualWithPrimaryTensor : primaryTensor, secondaryTensor : secondaryTensor, name : name)
    }
    unsafe fn lessThanWithPrimaryTensor_secondaryTensor_name_(
        &self,
        primaryTensor: MPSGraphTensor,
        secondaryTensor: MPSGraphTensor,
        name: NSString,
    ) -> MPSGraphTensor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, lessThanWithPrimaryTensor : primaryTensor, secondaryTensor : secondaryTensor, name : name)
    }
    unsafe fn lessThanOrEqualToWithPrimaryTensor_secondaryTensor_name_(
        &self,
        primaryTensor: MPSGraphTensor,
        secondaryTensor: MPSGraphTensor,
        name: NSString,
    ) -> MPSGraphTensor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, lessThanOrEqualToWithPrimaryTensor : primaryTensor, secondaryTensor : secondaryTensor, name : name)
    }
    unsafe fn greaterThanWithPrimaryTensor_secondaryTensor_name_(
        &self,
        primaryTensor: MPSGraphTensor,
        secondaryTensor: MPSGraphTensor,
        name: NSString,
    ) -> MPSGraphTensor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, greaterThanWithPrimaryTensor : primaryTensor, secondaryTensor : secondaryTensor, name : name)
    }
    unsafe fn greaterThanOrEqualToWithPrimaryTensor_secondaryTensor_name_(
        &self,
        primaryTensor: MPSGraphTensor,
        secondaryTensor: MPSGraphTensor,
        name: NSString,
    ) -> MPSGraphTensor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, greaterThanOrEqualToWithPrimaryTensor : primaryTensor, secondaryTensor : secondaryTensor, name : name)
    }
    unsafe fn logicalANDWithPrimaryTensor_secondaryTensor_name_(
        &self,
        primaryTensor: MPSGraphTensor,
        secondaryTensor: MPSGraphTensor,
        name: NSString,
    ) -> MPSGraphTensor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, logicalANDWithPrimaryTensor : primaryTensor, secondaryTensor : secondaryTensor, name : name)
    }
    unsafe fn logicalORWithPrimaryTensor_secondaryTensor_name_(
        &self,
        primaryTensor: MPSGraphTensor,
        secondaryTensor: MPSGraphTensor,
        name: NSString,
    ) -> MPSGraphTensor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, logicalORWithPrimaryTensor : primaryTensor, secondaryTensor : secondaryTensor, name : name)
    }
    unsafe fn logicalNANDWithPrimaryTensor_secondaryTensor_name_(
        &self,
        primaryTensor: MPSGraphTensor,
        secondaryTensor: MPSGraphTensor,
        name: NSString,
    ) -> MPSGraphTensor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, logicalNANDWithPrimaryTensor : primaryTensor, secondaryTensor : secondaryTensor, name : name)
    }
    unsafe fn logicalNORWithPrimaryTensor_secondaryTensor_name_(
        &self,
        primaryTensor: MPSGraphTensor,
        secondaryTensor: MPSGraphTensor,
        name: NSString,
    ) -> MPSGraphTensor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, logicalNORWithPrimaryTensor : primaryTensor, secondaryTensor : secondaryTensor, name : name)
    }
    unsafe fn logicalXORWithPrimaryTensor_secondaryTensor_name_(
        &self,
        primaryTensor: MPSGraphTensor,
        secondaryTensor: MPSGraphTensor,
        name: NSString,
    ) -> MPSGraphTensor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, logicalXORWithPrimaryTensor : primaryTensor, secondaryTensor : secondaryTensor, name : name)
    }
    unsafe fn logicalXNORWithPrimaryTensor_secondaryTensor_name_(
        &self,
        primaryTensor: MPSGraphTensor,
        secondaryTensor: MPSGraphTensor,
        name: NSString,
    ) -> MPSGraphTensor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, logicalXNORWithPrimaryTensor : primaryTensor, secondaryTensor : secondaryTensor, name : name)
    }
    unsafe fn atan2WithPrimaryTensor_secondaryTensor_name_(
        &self,
        primaryTensor: MPSGraphTensor,
        secondaryTensor: MPSGraphTensor,
        name: NSString,
    ) -> MPSGraphTensor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, atan2WithPrimaryTensor : primaryTensor, secondaryTensor : secondaryTensor, name : name)
    }
    unsafe fn bitwiseANDWithPrimaryTensor_secondaryTensor_name_(
        &self,
        primaryTensor: MPSGraphTensor,
        secondaryTensor: MPSGraphTensor,
        name: NSString,
    ) -> MPSGraphTensor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, bitwiseANDWithPrimaryTensor : primaryTensor, secondaryTensor : secondaryTensor, name : name)
    }
    unsafe fn bitwiseORWithPrimaryTensor_secondaryTensor_name_(
        &self,
        primaryTensor: MPSGraphTensor,
        secondaryTensor: MPSGraphTensor,
        name: NSString,
    ) -> MPSGraphTensor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, bitwiseORWithPrimaryTensor : primaryTensor, secondaryTensor : secondaryTensor, name : name)
    }
    unsafe fn bitwiseXORWithPrimaryTensor_secondaryTensor_name_(
        &self,
        primaryTensor: MPSGraphTensor,
        secondaryTensor: MPSGraphTensor,
        name: NSString,
    ) -> MPSGraphTensor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, bitwiseXORWithPrimaryTensor : primaryTensor, secondaryTensor : secondaryTensor, name : name)
    }
    unsafe fn bitwiseLeftShiftWithPrimaryTensor_secondaryTensor_name_(
        &self,
        primaryTensor: MPSGraphTensor,
        secondaryTensor: MPSGraphTensor,
        name: NSString,
    ) -> MPSGraphTensor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, bitwiseLeftShiftWithPrimaryTensor : primaryTensor, secondaryTensor : secondaryTensor, name : name)
    }
    unsafe fn bitwiseRightShiftWithPrimaryTensor_secondaryTensor_name_(
        &self,
        primaryTensor: MPSGraphTensor,
        secondaryTensor: MPSGraphTensor,
        name: NSString,
    ) -> MPSGraphTensor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, bitwiseRightShiftWithPrimaryTensor : primaryTensor, secondaryTensor : secondaryTensor, name : name)
    }
    unsafe fn selectWithPredicateTensor_truePredicateTensor_falsePredicateTensor_name_(
        &self,
        predicateTensor: MPSGraphTensor,
        truePredicateTensor: MPSGraphTensor,
        falseSelectTensor: MPSGraphTensor,
        name: NSString,
    ) -> MPSGraphTensor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, selectWithPredicateTensor : predicateTensor, truePredicateTensor : truePredicateTensor, falsePredicateTensor : falseSelectTensor, name : name)
    }
    unsafe fn clampWithTensor_minValueTensor_maxValueTensor_name_(
        &self,
        tensor: MPSGraphTensor,
        minValueTensor: MPSGraphTensor,
        maxValueTensor: MPSGraphTensor,
        name: NSString,
    ) -> MPSGraphTensor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, clampWithTensor : tensor, minValueTensor : minValueTensor, maxValueTensor : maxValueTensor, name : name)
    }
    unsafe fn divisionNoNaNWithPrimaryTensor_secondaryTensor_name_(
        &self,
        primaryTensor: MPSGraphTensor,
        secondaryTensor: MPSGraphTensor,
        name: NSString,
    ) -> MPSGraphTensor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, divisionNoNaNWithPrimaryTensor : primaryTensor, secondaryTensor : secondaryTensor, name : name)
    }
    unsafe fn floorModuloWithPrimaryTensor_secondaryTensor_name_(
        &self,
        primaryTensor: MPSGraphTensor,
        secondaryTensor: MPSGraphTensor,
        name: NSString,
    ) -> MPSGraphTensor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, floorModuloWithPrimaryTensor : primaryTensor, secondaryTensor : secondaryTensor, name : name)
    }
    unsafe fn realPartOfTensor_name_(
        &self,
        tensor: MPSGraphTensor,
        name: NSString,
    ) -> MPSGraphTensor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, realPartOfTensor : tensor, name : name)
    }
    unsafe fn imaginaryPartOfTensor_name_(
        &self,
        tensor: MPSGraphTensor,
        name: NSString,
    ) -> MPSGraphTensor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, imaginaryPartOfTensor : tensor, name : name)
    }
    unsafe fn complexTensorWithRealTensor_imaginaryTensor_name_(
        &self,
        realTensor: MPSGraphTensor,
        imaginaryTensor: MPSGraphTensor,
        name: NSString,
    ) -> MPSGraphTensor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, complexTensorWithRealTensor : realTensor, imaginaryTensor : imaginaryTensor, name : name)
    }
}
impl MPSGraph_CallOp for MPSGraph {}
pub trait MPSGraph_CallOp: Sized + std::ops::Deref {
    unsafe fn callSymbolName_inputTensors_outputTypes_name_(
        &self,
        symbolName: NSString,
        inputTensors: NSArray,
        outputTypes: NSArray,
        name: NSString,
    ) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, callSymbolName : symbolName, inputTensors : inputTensors, outputTypes : outputTypes, name : name)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MPSGraphConvolution2DOpDescriptor(pub id);
impl std::ops::Deref for MPSGraphConvolution2DOpDescriptor {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MPSGraphConvolution2DOpDescriptor {}
impl MPSGraphConvolution2DOpDescriptor {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MPSGraphConvolution2DOpDescriptor").unwrap(), alloc) })
    }
}
impl PNSCopying for MPSGraphConvolution2DOpDescriptor {}
impl IMPSGraphObject for MPSGraphConvolution2DOpDescriptor {}
impl std::convert::TryFrom<MPSGraphObject> for MPSGraphConvolution2DOpDescriptor {
    type Error = &'static str;
    fn try_from(parent: MPSGraphObject) -> Result<MPSGraphConvolution2DOpDescriptor, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MPSGraphConvolution2DOpDescriptor").unwrap())
        };
        if is_kind_of {
            Ok(MPSGraphConvolution2DOpDescriptor(parent.0))
        } else {
            Err("This MPSGraphObject cannot be downcasted to MPSGraphConvolution2DOpDescriptor")
        }
    }
}
impl INSObject for MPSGraphConvolution2DOpDescriptor {}
impl PNSObject for MPSGraphConvolution2DOpDescriptor {}
impl IMPSGraphConvolution2DOpDescriptor for MPSGraphConvolution2DOpDescriptor {}
pub trait IMPSGraphConvolution2DOpDescriptor: Sized + std::ops::Deref {
    unsafe fn setExplicitPaddingWithPaddingLeft_paddingRight_paddingTop_paddingBottom_(
        &self,
        paddingLeft: NSUInteger,
        paddingRight: NSUInteger,
        paddingTop: NSUInteger,
        paddingBottom: NSUInteger,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setExplicitPaddingWithPaddingLeft : paddingLeft, paddingRight : paddingRight, paddingTop : paddingTop, paddingBottom : paddingBottom)
    }
    unsafe fn strideInX(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, strideInX)
    }
    unsafe fn setStrideInX_(&self, strideInX: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setStrideInX : strideInX)
    }
    unsafe fn strideInY(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, strideInY)
    }
    unsafe fn setStrideInY_(&self, strideInY: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setStrideInY : strideInY)
    }
    unsafe fn dilationRateInX(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, dilationRateInX)
    }
    unsafe fn setDilationRateInX_(&self, dilationRateInX: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDilationRateInX : dilationRateInX)
    }
    unsafe fn dilationRateInY(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, dilationRateInY)
    }
    unsafe fn setDilationRateInY_(&self, dilationRateInY: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDilationRateInY : dilationRateInY)
    }
    unsafe fn paddingLeft(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, paddingLeft)
    }
    unsafe fn setPaddingLeft_(&self, paddingLeft: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPaddingLeft : paddingLeft)
    }
    unsafe fn paddingRight(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, paddingRight)
    }
    unsafe fn setPaddingRight_(&self, paddingRight: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPaddingRight : paddingRight)
    }
    unsafe fn paddingTop(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, paddingTop)
    }
    unsafe fn setPaddingTop_(&self, paddingTop: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPaddingTop : paddingTop)
    }
    unsafe fn paddingBottom(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, paddingBottom)
    }
    unsafe fn setPaddingBottom_(&self, paddingBottom: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPaddingBottom : paddingBottom)
    }
    unsafe fn paddingStyle(&self) -> MPSGraphPaddingStyle
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, paddingStyle)
    }
    unsafe fn setPaddingStyle_(&self, paddingStyle: MPSGraphPaddingStyle)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPaddingStyle : paddingStyle)
    }
    unsafe fn dataLayout(&self) -> MPSGraphTensorNamedDataLayout
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, dataLayout)
    }
    unsafe fn setDataLayout_(&self, dataLayout: MPSGraphTensorNamedDataLayout)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDataLayout : dataLayout)
    }
    unsafe fn weightsLayout(&self) -> MPSGraphTensorNamedDataLayout
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, weightsLayout)
    }
    unsafe fn setWeightsLayout_(&self, weightsLayout: MPSGraphTensorNamedDataLayout)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setWeightsLayout : weightsLayout)
    }
    unsafe fn groups(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, groups)
    }
    unsafe fn setGroups_(&self, groups: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setGroups : groups)
    }
    unsafe fn descriptorWithStrideInX_strideInY_dilationRateInX_dilationRateInY_groups_paddingLeft_paddingRight_paddingTop_paddingBottom_paddingStyle_dataLayout_weightsLayout_(
        strideInX: NSUInteger,
        strideInY: NSUInteger,
        dilationRateInX: NSUInteger,
        dilationRateInY: NSUInteger,
        groups: NSUInteger,
        paddingLeft: NSUInteger,
        paddingRight: NSUInteger,
        paddingTop: NSUInteger,
        paddingBottom: NSUInteger,
        paddingStyle: MPSGraphPaddingStyle,
        dataLayout: MPSGraphTensorNamedDataLayout,
        weightsLayout: MPSGraphTensorNamedDataLayout,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MPSGraphConvolution2DOpDescriptor").unwrap(), descriptorWithStrideInX : strideInX, strideInY : strideInY, dilationRateInX : dilationRateInX, dilationRateInY : dilationRateInY, groups : groups, paddingLeft : paddingLeft, paddingRight : paddingRight, paddingTop : paddingTop, paddingBottom : paddingBottom, paddingStyle : paddingStyle, dataLayout : dataLayout, weightsLayout : weightsLayout)
    }
    unsafe fn descriptorWithStrideInX_strideInY_dilationRateInX_dilationRateInY_groups_paddingStyle_dataLayout_weightsLayout_(
        strideInX: NSUInteger,
        strideInY: NSUInteger,
        dilationRateInX: NSUInteger,
        dilationRateInY: NSUInteger,
        groups: NSUInteger,
        paddingStyle: MPSGraphPaddingStyle,
        dataLayout: MPSGraphTensorNamedDataLayout,
        weightsLayout: MPSGraphTensorNamedDataLayout,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MPSGraphConvolution2DOpDescriptor").unwrap(), descriptorWithStrideInX : strideInX, strideInY : strideInY, dilationRateInX : dilationRateInX, dilationRateInY : dilationRateInY, groups : groups, paddingStyle : paddingStyle, dataLayout : dataLayout, weightsLayout : weightsLayout)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MPSGraphConvolution3DOpDescriptor(pub id);
impl std::ops::Deref for MPSGraphConvolution3DOpDescriptor {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MPSGraphConvolution3DOpDescriptor {}
impl MPSGraphConvolution3DOpDescriptor {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MPSGraphConvolution3DOpDescriptor").unwrap(), alloc) })
    }
}
impl PNSCopying for MPSGraphConvolution3DOpDescriptor {}
impl IMPSGraphObject for MPSGraphConvolution3DOpDescriptor {}
impl std::convert::TryFrom<MPSGraphObject> for MPSGraphConvolution3DOpDescriptor {
    type Error = &'static str;
    fn try_from(parent: MPSGraphObject) -> Result<MPSGraphConvolution3DOpDescriptor, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MPSGraphConvolution3DOpDescriptor").unwrap())
        };
        if is_kind_of {
            Ok(MPSGraphConvolution3DOpDescriptor(parent.0))
        } else {
            Err("This MPSGraphObject cannot be downcasted to MPSGraphConvolution3DOpDescriptor")
        }
    }
}
impl INSObject for MPSGraphConvolution3DOpDescriptor {}
impl PNSObject for MPSGraphConvolution3DOpDescriptor {}
impl IMPSGraphConvolution3DOpDescriptor for MPSGraphConvolution3DOpDescriptor {}
pub trait IMPSGraphConvolution3DOpDescriptor: Sized + std::ops::Deref {
    unsafe fn setExplicitPaddingWithPaddingLeft_paddingRight_paddingTop_paddingBottom_paddingFront_paddingBack_(
        &self,
        paddingLeft: NSUInteger,
        paddingRight: NSUInteger,
        paddingTop: NSUInteger,
        paddingBottom: NSUInteger,
        paddingFront: NSUInteger,
        paddingBack: NSUInteger,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setExplicitPaddingWithPaddingLeft : paddingLeft, paddingRight : paddingRight, paddingTop : paddingTop, paddingBottom : paddingBottom, paddingFront : paddingFront, paddingBack : paddingBack)
    }
    unsafe fn strideInX(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, strideInX)
    }
    unsafe fn setStrideInX_(&self, strideInX: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setStrideInX : strideInX)
    }
    unsafe fn strideInY(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, strideInY)
    }
    unsafe fn setStrideInY_(&self, strideInY: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setStrideInY : strideInY)
    }
    unsafe fn strideInZ(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, strideInZ)
    }
    unsafe fn setStrideInZ_(&self, strideInZ: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setStrideInZ : strideInZ)
    }
    unsafe fn dilationRateInX(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, dilationRateInX)
    }
    unsafe fn setDilationRateInX_(&self, dilationRateInX: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDilationRateInX : dilationRateInX)
    }
    unsafe fn dilationRateInY(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, dilationRateInY)
    }
    unsafe fn setDilationRateInY_(&self, dilationRateInY: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDilationRateInY : dilationRateInY)
    }
    unsafe fn dilationRateInZ(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, dilationRateInZ)
    }
    unsafe fn setDilationRateInZ_(&self, dilationRateInZ: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDilationRateInZ : dilationRateInZ)
    }
    unsafe fn paddingLeft(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, paddingLeft)
    }
    unsafe fn setPaddingLeft_(&self, paddingLeft: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPaddingLeft : paddingLeft)
    }
    unsafe fn paddingRight(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, paddingRight)
    }
    unsafe fn setPaddingRight_(&self, paddingRight: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPaddingRight : paddingRight)
    }
    unsafe fn paddingTop(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, paddingTop)
    }
    unsafe fn setPaddingTop_(&self, paddingTop: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPaddingTop : paddingTop)
    }
    unsafe fn paddingBottom(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, paddingBottom)
    }
    unsafe fn setPaddingBottom_(&self, paddingBottom: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPaddingBottom : paddingBottom)
    }
    unsafe fn paddingFront(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, paddingFront)
    }
    unsafe fn setPaddingFront_(&self, paddingFront: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPaddingFront : paddingFront)
    }
    unsafe fn paddingBack(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, paddingBack)
    }
    unsafe fn setPaddingBack_(&self, paddingBack: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPaddingBack : paddingBack)
    }
    unsafe fn paddingStyle(&self) -> MPSGraphPaddingStyle
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, paddingStyle)
    }
    unsafe fn setPaddingStyle_(&self, paddingStyle: MPSGraphPaddingStyle)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPaddingStyle : paddingStyle)
    }
    unsafe fn dataLayout(&self) -> MPSGraphTensorNamedDataLayout
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, dataLayout)
    }
    unsafe fn setDataLayout_(&self, dataLayout: MPSGraphTensorNamedDataLayout)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDataLayout : dataLayout)
    }
    unsafe fn weightsLayout(&self) -> MPSGraphTensorNamedDataLayout
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, weightsLayout)
    }
    unsafe fn setWeightsLayout_(&self, weightsLayout: MPSGraphTensorNamedDataLayout)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setWeightsLayout : weightsLayout)
    }
    unsafe fn groups(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, groups)
    }
    unsafe fn setGroups_(&self, groups: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setGroups : groups)
    }
    unsafe fn descriptorWithStrideInX_strideInY_strideInZ_dilationRateInX_dilationRateInY_dilationRateInZ_groups_paddingStyle_dataLayout_weightsLayout_(
        strideInX: NSUInteger,
        strideInY: NSUInteger,
        strideInZ: NSUInteger,
        dilationRateInX: NSUInteger,
        dilationRateInY: NSUInteger,
        dilationRateInZ: NSUInteger,
        groups: NSUInteger,
        paddingStyle: MPSGraphPaddingStyle,
        dataLayout: MPSGraphTensorNamedDataLayout,
        weightsLayout: MPSGraphTensorNamedDataLayout,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MPSGraphConvolution3DOpDescriptor").unwrap(), descriptorWithStrideInX : strideInX, strideInY : strideInY, strideInZ : strideInZ, dilationRateInX : dilationRateInX, dilationRateInY : dilationRateInY, dilationRateInZ : dilationRateInZ, groups : groups, paddingStyle : paddingStyle, dataLayout : dataLayout, weightsLayout : weightsLayout)
    }
}
impl MPSGraph_MPSGraphConvolutionOps for MPSGraph {}
pub trait MPSGraph_MPSGraphConvolutionOps: Sized + std::ops::Deref {
    unsafe fn convolution2DWithSourceTensor_weightsTensor_descriptor_name_(
        &self,
        source: MPSGraphTensor,
        weights: MPSGraphTensor,
        descriptor: MPSGraphConvolution2DOpDescriptor,
        name: NSString,
    ) -> MPSGraphTensor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, convolution2DWithSourceTensor : source, weightsTensor : weights, descriptor : descriptor, name : name)
    }
    unsafe fn convolution2DDataGradientWithIncomingGradientTensor_weightsTensor_outputShape_forwardConvolutionDescriptor_name_(
        &self,
        incomingGradient: MPSGraphTensor,
        weights: MPSGraphTensor,
        outputShape: NSArray,
        forwardConvolutionDescriptor: MPSGraphConvolution2DOpDescriptor,
        name: NSString,
    ) -> MPSGraphTensor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, convolution2DDataGradientWithIncomingGradientTensor : incomingGradient, weightsTensor : weights, outputShape : outputShape, forwardConvolutionDescriptor : forwardConvolutionDescriptor, name : name)
    }
    unsafe fn convolution2DDataGradientWithIncomingGradientTensor_weightsTensor_outputShapeTensor_forwardConvolutionDescriptor_name_(
        &self,
        gradient: MPSGraphTensor,
        weights: MPSGraphTensor,
        outputShapeTensor: MPSGraphTensor,
        forwardConvolutionDescriptor: MPSGraphConvolution2DOpDescriptor,
        name: NSString,
    ) -> MPSGraphTensor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, convolution2DDataGradientWithIncomingGradientTensor : gradient, weightsTensor : weights, outputShapeTensor : outputShapeTensor, forwardConvolutionDescriptor : forwardConvolutionDescriptor, name : name)
    }
    unsafe fn convolution2DWeightsGradientWithIncomingGradientTensor_sourceTensor_outputShape_forwardConvolutionDescriptor_name_(
        &self,
        incomingGradient: MPSGraphTensor,
        source: MPSGraphTensor,
        outputShape: NSArray,
        forwardConvolutionDescriptor: MPSGraphConvolution2DOpDescriptor,
        name: NSString,
    ) -> MPSGraphTensor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, convolution2DWeightsGradientWithIncomingGradientTensor : incomingGradient, sourceTensor : source, outputShape : outputShape, forwardConvolutionDescriptor : forwardConvolutionDescriptor, name : name)
    }
    unsafe fn convolution2DWeightsGradientWithIncomingGradientTensor_sourceTensor_outputShapeTensor_forwardConvolutionDescriptor_name_(
        &self,
        gradient: MPSGraphTensor,
        source: MPSGraphTensor,
        outputShapeTensor: MPSGraphTensor,
        forwardConvolutionDescriptor: MPSGraphConvolution2DOpDescriptor,
        name: NSString,
    ) -> MPSGraphTensor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, convolution2DWeightsGradientWithIncomingGradientTensor : gradient, sourceTensor : source, outputShapeTensor : outputShapeTensor, forwardConvolutionDescriptor : forwardConvolutionDescriptor, name : name)
    }
    unsafe fn convolution3DWithSourceTensor_weightsTensor_descriptor_name_(
        &self,
        source: MPSGraphTensor,
        weights: MPSGraphTensor,
        descriptor: MPSGraphConvolution3DOpDescriptor,
        name: NSString,
    ) -> MPSGraphTensor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, convolution3DWithSourceTensor : source, weightsTensor : weights, descriptor : descriptor, name : name)
    }
    unsafe fn convolution3DDataGradientWithIncomingGradientTensor_weightsTensor_outputShape_forwardConvolutionDescriptor_name_(
        &self,
        incomingGradient: MPSGraphTensor,
        weights: MPSGraphTensor,
        outputShape: NSArray,
        forwardConvolutionDescriptor: MPSGraphConvolution3DOpDescriptor,
        name: NSString,
    ) -> MPSGraphTensor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, convolution3DDataGradientWithIncomingGradientTensor : incomingGradient, weightsTensor : weights, outputShape : outputShape, forwardConvolutionDescriptor : forwardConvolutionDescriptor, name : name)
    }
    unsafe fn convolution3DDataGradientWithIncomingGradientTensor_weightsTensor_outputShapeTensor_forwardConvolutionDescriptor_name_(
        &self,
        gradient: MPSGraphTensor,
        weights: MPSGraphTensor,
        outputShapeTensor: MPSGraphTensor,
        forwardConvolutionDescriptor: MPSGraphConvolution3DOpDescriptor,
        name: NSString,
    ) -> MPSGraphTensor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, convolution3DDataGradientWithIncomingGradientTensor : gradient, weightsTensor : weights, outputShapeTensor : outputShapeTensor, forwardConvolutionDescriptor : forwardConvolutionDescriptor, name : name)
    }
    unsafe fn convolution3DWeightsGradientWithIncomingGradientTensor_sourceTensor_outputShape_forwardConvolutionDescriptor_name_(
        &self,
        incomingGradient: MPSGraphTensor,
        source: MPSGraphTensor,
        outputShape: NSArray,
        forwardConvolutionDescriptor: MPSGraphConvolution3DOpDescriptor,
        name: NSString,
    ) -> MPSGraphTensor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, convolution3DWeightsGradientWithIncomingGradientTensor : incomingGradient, sourceTensor : source, outputShape : outputShape, forwardConvolutionDescriptor : forwardConvolutionDescriptor, name : name)
    }
    unsafe fn convolution3DWeightsGradientWithIncomingGradientTensor_sourceTensor_outputShapeTensor_forwardConvolutionDescriptor_name_(
        &self,
        gradient: MPSGraphTensor,
        source: MPSGraphTensor,
        outputShapeTensor: MPSGraphTensor,
        forwardConvolutionDescriptor: MPSGraphConvolution3DOpDescriptor,
        name: NSString,
    ) -> MPSGraphTensor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, convolution3DWeightsGradientWithIncomingGradientTensor : gradient, sourceTensor : source, outputShapeTensor : outputShapeTensor, forwardConvolutionDescriptor : forwardConvolutionDescriptor, name : name)
    }
}
impl MPSGraph_MPSGraphConvolutionTransposeOps for MPSGraph {}
pub trait MPSGraph_MPSGraphConvolutionTransposeOps: Sized + std::ops::Deref {
    unsafe fn convolutionTranspose2DWithSourceTensor_weightsTensor_outputShape_descriptor_name_(
        &self,
        source: MPSGraphTensor,
        weights: MPSGraphTensor,
        outputShape: NSArray,
        descriptor: MPSGraphConvolution2DOpDescriptor,
        name: NSString,
    ) -> MPSGraphTensor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, convolutionTranspose2DWithSourceTensor : source, weightsTensor : weights, outputShape : outputShape, descriptor : descriptor, name : name)
    }
    unsafe fn convolutionTranspose2DWithSourceTensor_weightsTensor_outputShapeTensor_descriptor_name_(
        &self,
        source: MPSGraphTensor,
        weights: MPSGraphTensor,
        outputShape: MPSGraphTensor,
        descriptor: MPSGraphConvolution2DOpDescriptor,
        name: NSString,
    ) -> MPSGraphTensor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, convolutionTranspose2DWithSourceTensor : source, weightsTensor : weights, outputShapeTensor : outputShape, descriptor : descriptor, name : name)
    }
    unsafe fn convolutionTranspose2DDataGradientWithIncomingGradientTensor_weightsTensor_outputShape_forwardConvolutionDescriptor_name_(
        &self,
        incomingGradient: MPSGraphTensor,
        weights: MPSGraphTensor,
        outputShape: NSArray,
        forwardConvolutionDescriptor: MPSGraphConvolution2DOpDescriptor,
        name: NSString,
    ) -> MPSGraphTensor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, convolutionTranspose2DDataGradientWithIncomingGradientTensor : incomingGradient, weightsTensor : weights, outputShape : outputShape, forwardConvolutionDescriptor : forwardConvolutionDescriptor, name : name)
    }
    unsafe fn convolutionTranspose2DDataGradientWithIncomingGradientTensor_weightsTensor_outputShapeTensor_forwardConvolutionDescriptor_name_(
        &self,
        incomingGradient: MPSGraphTensor,
        weights: MPSGraphTensor,
        outputShape: MPSGraphTensor,
        forwardConvolutionDescriptor: MPSGraphConvolution2DOpDescriptor,
        name: NSString,
    ) -> MPSGraphTensor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, convolutionTranspose2DDataGradientWithIncomingGradientTensor : incomingGradient, weightsTensor : weights, outputShapeTensor : outputShape, forwardConvolutionDescriptor : forwardConvolutionDescriptor, name : name)
    }
    unsafe fn convolutionTranspose2DWeightsGradientWithIncomingGradientTensor_sourceTensor_outputShape_forwardConvolutionDescriptor_name_(
        &self,
        incomingGradientTensor: MPSGraphTensor,
        source: MPSGraphTensor,
        outputShape: NSArray,
        forwardConvolutionDescriptor: MPSGraphConvolution2DOpDescriptor,
        name: NSString,
    ) -> MPSGraphTensor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, convolutionTranspose2DWeightsGradientWithIncomingGradientTensor : incomingGradientTensor, sourceTensor : source, outputShape : outputShape, forwardConvolutionDescriptor : forwardConvolutionDescriptor, name : name)
    }
    unsafe fn convolutionTranspose2DWeightsGradientWithIncomingGradientTensor_sourceTensor_outputShapeTensor_forwardConvolutionDescriptor_name_(
        &self,
        incomingGradientTensor: MPSGraphTensor,
        source: MPSGraphTensor,
        outputShape: MPSGraphTensor,
        forwardConvolutionDescriptor: MPSGraphConvolution2DOpDescriptor,
        name: NSString,
    ) -> MPSGraphTensor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, convolutionTranspose2DWeightsGradientWithIncomingGradientTensor : incomingGradientTensor, sourceTensor : source, outputShapeTensor : outputShape, forwardConvolutionDescriptor : forwardConvolutionDescriptor, name : name)
    }
}
impl MPSGraph_MPSGraphControlFlowOps for MPSGraph {}
pub trait MPSGraph_MPSGraphControlFlowOps: Sized + std::ops::Deref {
    unsafe fn controlDependencyWithOperations_dependentBlock_name_(
        &self,
        operations: NSArray,
        dependentBlock: MPSGraphControlFlowDependencyBlock,
        name: NSString,
    ) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, controlDependencyWithOperations : operations, dependentBlock : dependentBlock, name : name)
    }
    unsafe fn ifWithPredicateTensor_thenBlock_elseBlock_name_(
        &self,
        predicateTensor: MPSGraphTensor,
        thenBlock: MPSGraphIfThenElseBlock,
        elseBlock: MPSGraphIfThenElseBlock,
        name: NSString,
    ) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, ifWithPredicateTensor : predicateTensor, thenBlock : thenBlock, elseBlock : elseBlock, name : name)
    }
    unsafe fn whileWithInitialInputs_before_after_name_(
        &self,
        initialInputs: NSArray,
        before: MPSGraphWhileBeforeBlock,
        after: MPSGraphWhileAfterBlock,
        name: NSString,
    ) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, whileWithInitialInputs : initialInputs, before : before, after : after, name : name)
    }
    unsafe fn forLoopWithLowerBound_upperBound_step_initialBodyArguments_body_name_(
        &self,
        lowerBound: MPSGraphTensor,
        upperBound: MPSGraphTensor,
        step: MPSGraphTensor,
        initialBodyArguments: NSArray,
        body: MPSGraphForLoopBodyBlock,
        name: NSString,
    ) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, forLoopWithLowerBound : lowerBound, upperBound : upperBound, step : step, initialBodyArguments : initialBodyArguments, body : body, name : name)
    }
    unsafe fn forLoopWithNumberOfIterations_initialBodyArguments_body_name_(
        &self,
        numberOfIterations: MPSGraphTensor,
        initialBodyArguments: NSArray,
        body: MPSGraphForLoopBodyBlock,
        name: NSString,
    ) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, forLoopWithNumberOfIterations : numberOfIterations, initialBodyArguments : initialBodyArguments, body : body, name : name)
    }
}
pub type MPSGraphControlFlowDependencyBlock = *mut ::std::os::raw::c_void;
pub type MPSGraphIfThenElseBlock = *mut ::std::os::raw::c_void;
pub type MPSGraphWhileBeforeBlock = *mut ::std::os::raw::c_void;
pub type MPSGraphWhileAfterBlock = *mut ::std::os::raw::c_void;
pub type MPSGraphForLoopBodyBlock = *mut ::std::os::raw::c_void;
impl MPSGraph_MPSGraphCumulativeOps for MPSGraph {}
pub trait MPSGraph_MPSGraphCumulativeOps: Sized + std::ops::Deref {
    unsafe fn cumulativeSumWithTensor_axis_exclusive_reverse_name_(
        &self,
        tensor: MPSGraphTensor,
        axis: NSInteger,
        exclusive: BOOL,
        reverse: BOOL,
        name: NSString,
    ) -> MPSGraphTensor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, cumulativeSumWithTensor : tensor, axis : axis, exclusive : exclusive, reverse : reverse, name : name)
    }
    unsafe fn cumulativeSumWithTensor_axisTensor_exclusive_reverse_name_(
        &self,
        tensor: MPSGraphTensor,
        axisTensor: MPSGraphTensor,
        exclusive: BOOL,
        reverse: BOOL,
        name: NSString,
    ) -> MPSGraphTensor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, cumulativeSumWithTensor : tensor, axisTensor : axisTensor, exclusive : exclusive, reverse : reverse, name : name)
    }
    unsafe fn cumulativeSumWithTensor_axis_name_(
        &self,
        tensor: MPSGraphTensor,
        axis: NSInteger,
        name: NSString,
    ) -> MPSGraphTensor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, cumulativeSumWithTensor : tensor, axis : axis, name : name)
    }
    unsafe fn cumulativeSumWithTensor_axisTensor_name_(
        &self,
        tensor: MPSGraphTensor,
        axisTensor: MPSGraphTensor,
        name: NSString,
    ) -> MPSGraphTensor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, cumulativeSumWithTensor : tensor, axisTensor : axisTensor, name : name)
    }
    unsafe fn cumulativeProductWithTensor_axis_exclusive_reverse_name_(
        &self,
        tensor: MPSGraphTensor,
        axis: NSInteger,
        exclusive: BOOL,
        reverse: BOOL,
        name: NSString,
    ) -> MPSGraphTensor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, cumulativeProductWithTensor : tensor, axis : axis, exclusive : exclusive, reverse : reverse, name : name)
    }
    unsafe fn cumulativeProductWithTensor_axisTensor_exclusive_reverse_name_(
        &self,
        tensor: MPSGraphTensor,
        axisTensor: MPSGraphTensor,
        exclusive: BOOL,
        reverse: BOOL,
        name: NSString,
    ) -> MPSGraphTensor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, cumulativeProductWithTensor : tensor, axisTensor : axisTensor, exclusive : exclusive, reverse : reverse, name : name)
    }
    unsafe fn cumulativeProductWithTensor_axis_name_(
        &self,
        tensor: MPSGraphTensor,
        axis: NSInteger,
        name: NSString,
    ) -> MPSGraphTensor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, cumulativeProductWithTensor : tensor, axis : axis, name : name)
    }
    unsafe fn cumulativeProductWithTensor_axisTensor_name_(
        &self,
        tensor: MPSGraphTensor,
        axisTensor: MPSGraphTensor,
        name: NSString,
    ) -> MPSGraphTensor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, cumulativeProductWithTensor : tensor, axisTensor : axisTensor, name : name)
    }
    unsafe fn cumulativeMinimumWithTensor_axis_exclusive_reverse_name_(
        &self,
        tensor: MPSGraphTensor,
        axis: NSInteger,
        exclusive: BOOL,
        reverse: BOOL,
        name: NSString,
    ) -> MPSGraphTensor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, cumulativeMinimumWithTensor : tensor, axis : axis, exclusive : exclusive, reverse : reverse, name : name)
    }
    unsafe fn cumulativeMinimumWithTensor_axisTensor_exclusive_reverse_name_(
        &self,
        tensor: MPSGraphTensor,
        axisTensor: MPSGraphTensor,
        exclusive: BOOL,
        reverse: BOOL,
        name: NSString,
    ) -> MPSGraphTensor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, cumulativeMinimumWithTensor : tensor, axisTensor : axisTensor, exclusive : exclusive, reverse : reverse, name : name)
    }
    unsafe fn cumulativeMinimumWithTensor_axis_name_(
        &self,
        tensor: MPSGraphTensor,
        axis: NSInteger,
        name: NSString,
    ) -> MPSGraphTensor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, cumulativeMinimumWithTensor : tensor, axis : axis, name : name)
    }
    unsafe fn cumulativeMinimumWithTensor_axisTensor_name_(
        &self,
        tensor: MPSGraphTensor,
        axisTensor: MPSGraphTensor,
        name: NSString,
    ) -> MPSGraphTensor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, cumulativeMinimumWithTensor : tensor, axisTensor : axisTensor, name : name)
    }
    unsafe fn cumulativeMaximumWithTensor_axis_exclusive_reverse_name_(
        &self,
        tensor: MPSGraphTensor,
        axis: NSInteger,
        exclusive: BOOL,
        reverse: BOOL,
        name: NSString,
    ) -> MPSGraphTensor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, cumulativeMaximumWithTensor : tensor, axis : axis, exclusive : exclusive, reverse : reverse, name : name)
    }
    unsafe fn cumulativeMaximumWithTensor_axisTensor_exclusive_reverse_name_(
        &self,
        tensor: MPSGraphTensor,
        axisTensor: MPSGraphTensor,
        exclusive: BOOL,
        reverse: BOOL,
        name: NSString,
    ) -> MPSGraphTensor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, cumulativeMaximumWithTensor : tensor, axisTensor : axisTensor, exclusive : exclusive, reverse : reverse, name : name)
    }
    unsafe fn cumulativeMaximumWithTensor_axis_name_(
        &self,
        tensor: MPSGraphTensor,
        axis: NSInteger,
        name: NSString,
    ) -> MPSGraphTensor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, cumulativeMaximumWithTensor : tensor, axis : axis, name : name)
    }
    unsafe fn cumulativeMaximumWithTensor_axisTensor_name_(
        &self,
        tensor: MPSGraphTensor,
        axisTensor: MPSGraphTensor,
        name: NSString,
    ) -> MPSGraphTensor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, cumulativeMaximumWithTensor : tensor, axisTensor : axisTensor, name : name)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MPSGraphDepthwiseConvolution2DOpDescriptor(pub id);
impl std::ops::Deref for MPSGraphDepthwiseConvolution2DOpDescriptor {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MPSGraphDepthwiseConvolution2DOpDescriptor {}
impl MPSGraphDepthwiseConvolution2DOpDescriptor {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MPSGraphDepthwiseConvolution2DOpDescriptor").unwrap(), alloc) })
    }
}
impl PNSCopying for MPSGraphDepthwiseConvolution2DOpDescriptor {}
impl IMPSGraphObject for MPSGraphDepthwiseConvolution2DOpDescriptor {}
impl std::convert::TryFrom<MPSGraphObject> for MPSGraphDepthwiseConvolution2DOpDescriptor {
    type Error = &'static str;
    fn try_from(
        parent: MPSGraphObject,
    ) -> Result<MPSGraphDepthwiseConvolution2DOpDescriptor, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MPSGraphDepthwiseConvolution2DOpDescriptor").unwrap())
        };
        if is_kind_of {
            Ok(MPSGraphDepthwiseConvolution2DOpDescriptor(parent.0))
        } else {
            Err ("This MPSGraphObject cannot be downcasted to MPSGraphDepthwiseConvolution2DOpDescriptor" ,)
        }
    }
}
impl INSObject for MPSGraphDepthwiseConvolution2DOpDescriptor {}
impl PNSObject for MPSGraphDepthwiseConvolution2DOpDescriptor {}
impl IMPSGraphDepthwiseConvolution2DOpDescriptor for MPSGraphDepthwiseConvolution2DOpDescriptor {}
pub trait IMPSGraphDepthwiseConvolution2DOpDescriptor: Sized + std::ops::Deref {
    unsafe fn setExplicitPaddingWithPaddingLeft_paddingRight_paddingTop_paddingBottom_(
        &self,
        paddingLeft: NSUInteger,
        paddingRight: NSUInteger,
        paddingTop: NSUInteger,
        paddingBottom: NSUInteger,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setExplicitPaddingWithPaddingLeft : paddingLeft, paddingRight : paddingRight, paddingTop : paddingTop, paddingBottom : paddingBottom)
    }
    unsafe fn strideInX(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, strideInX)
    }
    unsafe fn setStrideInX_(&self, strideInX: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setStrideInX : strideInX)
    }
    unsafe fn strideInY(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, strideInY)
    }
    unsafe fn setStrideInY_(&self, strideInY: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setStrideInY : strideInY)
    }
    unsafe fn dilationRateInX(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, dilationRateInX)
    }
    unsafe fn setDilationRateInX_(&self, dilationRateInX: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDilationRateInX : dilationRateInX)
    }
    unsafe fn dilationRateInY(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, dilationRateInY)
    }
    unsafe fn setDilationRateInY_(&self, dilationRateInY: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDilationRateInY : dilationRateInY)
    }
    unsafe fn paddingLeft(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, paddingLeft)
    }
    unsafe fn setPaddingLeft_(&self, paddingLeft: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPaddingLeft : paddingLeft)
    }
    unsafe fn paddingRight(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, paddingRight)
    }
    unsafe fn setPaddingRight_(&self, paddingRight: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPaddingRight : paddingRight)
    }
    unsafe fn paddingTop(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, paddingTop)
    }
    unsafe fn setPaddingTop_(&self, paddingTop: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPaddingTop : paddingTop)
    }
    unsafe fn paddingBottom(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, paddingBottom)
    }
    unsafe fn setPaddingBottom_(&self, paddingBottom: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPaddingBottom : paddingBottom)
    }
    unsafe fn paddingStyle(&self) -> MPSGraphPaddingStyle
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, paddingStyle)
    }
    unsafe fn setPaddingStyle_(&self, paddingStyle: MPSGraphPaddingStyle)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPaddingStyle : paddingStyle)
    }
    unsafe fn dataLayout(&self) -> MPSGraphTensorNamedDataLayout
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, dataLayout)
    }
    unsafe fn setDataLayout_(&self, dataLayout: MPSGraphTensorNamedDataLayout)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDataLayout : dataLayout)
    }
    unsafe fn weightsLayout(&self) -> MPSGraphTensorNamedDataLayout
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, weightsLayout)
    }
    unsafe fn setWeightsLayout_(&self, weightsLayout: MPSGraphTensorNamedDataLayout)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setWeightsLayout : weightsLayout)
    }
    unsafe fn descriptorWithStrideInX_strideInY_dilationRateInX_dilationRateInY_paddingLeft_paddingRight_paddingTop_paddingBottom_paddingStyle_dataLayout_weightsLayout_(
        strideInX: NSUInteger,
        strideInY: NSUInteger,
        dilationRateInX: NSUInteger,
        dilationRateInY: NSUInteger,
        paddingLeft: NSUInteger,
        paddingRight: NSUInteger,
        paddingTop: NSUInteger,
        paddingBottom: NSUInteger,
        paddingStyle: MPSGraphPaddingStyle,
        dataLayout: MPSGraphTensorNamedDataLayout,
        weightsLayout: MPSGraphTensorNamedDataLayout,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MPSGraphDepthwiseConvolution2DOpDescriptor").unwrap(), descriptorWithStrideInX : strideInX, strideInY : strideInY, dilationRateInX : dilationRateInX, dilationRateInY : dilationRateInY, paddingLeft : paddingLeft, paddingRight : paddingRight, paddingTop : paddingTop, paddingBottom : paddingBottom, paddingStyle : paddingStyle, dataLayout : dataLayout, weightsLayout : weightsLayout)
    }
    unsafe fn descriptorWithDataLayout_weightsLayout_(
        dataLayout: MPSGraphTensorNamedDataLayout,
        weightsLayout: MPSGraphTensorNamedDataLayout,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MPSGraphDepthwiseConvolution2DOpDescriptor").unwrap(), descriptorWithDataLayout : dataLayout, weightsLayout : weightsLayout)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MPSGraphDepthwiseConvolution3DOpDescriptor(pub id);
impl std::ops::Deref for MPSGraphDepthwiseConvolution3DOpDescriptor {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MPSGraphDepthwiseConvolution3DOpDescriptor {}
impl MPSGraphDepthwiseConvolution3DOpDescriptor {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MPSGraphDepthwiseConvolution3DOpDescriptor").unwrap(), alloc) })
    }
}
impl PNSCopying for MPSGraphDepthwiseConvolution3DOpDescriptor {}
impl IMPSGraphObject for MPSGraphDepthwiseConvolution3DOpDescriptor {}
impl std::convert::TryFrom<MPSGraphObject> for MPSGraphDepthwiseConvolution3DOpDescriptor {
    type Error = &'static str;
    fn try_from(
        parent: MPSGraphObject,
    ) -> Result<MPSGraphDepthwiseConvolution3DOpDescriptor, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MPSGraphDepthwiseConvolution3DOpDescriptor").unwrap())
        };
        if is_kind_of {
            Ok(MPSGraphDepthwiseConvolution3DOpDescriptor(parent.0))
        } else {
            Err ("This MPSGraphObject cannot be downcasted to MPSGraphDepthwiseConvolution3DOpDescriptor" ,)
        }
    }
}
impl INSObject for MPSGraphDepthwiseConvolution3DOpDescriptor {}
impl PNSObject for MPSGraphDepthwiseConvolution3DOpDescriptor {}
impl IMPSGraphDepthwiseConvolution3DOpDescriptor for MPSGraphDepthwiseConvolution3DOpDescriptor {}
pub trait IMPSGraphDepthwiseConvolution3DOpDescriptor: Sized + std::ops::Deref {
    unsafe fn strides(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, strides)
    }
    unsafe fn setStrides_(&self, strides: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setStrides : strides)
    }
    unsafe fn dilationRates(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, dilationRates)
    }
    unsafe fn setDilationRates_(&self, dilationRates: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDilationRates : dilationRates)
    }
    unsafe fn paddingValues(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, paddingValues)
    }
    unsafe fn setPaddingValues_(&self, paddingValues: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPaddingValues : paddingValues)
    }
    unsafe fn paddingStyle(&self) -> MPSGraphPaddingStyle
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, paddingStyle)
    }
    unsafe fn setPaddingStyle_(&self, paddingStyle: MPSGraphPaddingStyle)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPaddingStyle : paddingStyle)
    }
    unsafe fn channelDimensionIndex(&self) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, channelDimensionIndex)
    }
    unsafe fn setChannelDimensionIndex_(&self, channelDimensionIndex: NSInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setChannelDimensionIndex : channelDimensionIndex)
    }
    unsafe fn descriptorWithStrides_dilationRates_paddingValues_paddingStyle_(
        strides: NSArray,
        dilationRates: NSArray,
        paddingValues: NSArray,
        paddingStyle: MPSGraphPaddingStyle,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MPSGraphDepthwiseConvolution3DOpDescriptor").unwrap(), descriptorWithStrides : strides, dilationRates : dilationRates, paddingValues : paddingValues, paddingStyle : paddingStyle)
    }
    unsafe fn descriptorWithPaddingStyle_(paddingStyle: MPSGraphPaddingStyle) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MPSGraphDepthwiseConvolution3DOpDescriptor").unwrap(), descriptorWithPaddingStyle : paddingStyle)
    }
}
impl MPSGraph_MPSGraphDepthwiseConvolutionOps for MPSGraph {}
pub trait MPSGraph_MPSGraphDepthwiseConvolutionOps: Sized + std::ops::Deref {
    unsafe fn depthwiseConvolution2DWithSourceTensor_weightsTensor_descriptor_name_(
        &self,
        source: MPSGraphTensor,
        weights: MPSGraphTensor,
        descriptor: MPSGraphDepthwiseConvolution2DOpDescriptor,
        name: NSString,
    ) -> MPSGraphTensor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, depthwiseConvolution2DWithSourceTensor : source, weightsTensor : weights, descriptor : descriptor, name : name)
    }
    unsafe fn depthwiseConvolution2DDataGradientWithIncomingGradientTensor_weightsTensor_outputShape_descriptor_name_(
        &self,
        incomingGradient: MPSGraphTensor,
        weights: MPSGraphTensor,
        outputShape: NSArray,
        descriptor: MPSGraphDepthwiseConvolution2DOpDescriptor,
        name: NSString,
    ) -> MPSGraphTensor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, depthwiseConvolution2DDataGradientWithIncomingGradientTensor : incomingGradient, weightsTensor : weights, outputShape : outputShape, descriptor : descriptor, name : name)
    }
    unsafe fn depthwiseConvolution2DWeightsGradientWithIncomingGradientTensor_sourceTensor_outputShape_descriptor_name_(
        &self,
        incomingGradient: MPSGraphTensor,
        source: MPSGraphTensor,
        outputShape: NSArray,
        descriptor: MPSGraphDepthwiseConvolution2DOpDescriptor,
        name: NSString,
    ) -> MPSGraphTensor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, depthwiseConvolution2DWeightsGradientWithIncomingGradientTensor : incomingGradient, sourceTensor : source, outputShape : outputShape, descriptor : descriptor, name : name)
    }
    unsafe fn depthwiseConvolution3DWithSourceTensor_weightsTensor_descriptor_name_(
        &self,
        source: MPSGraphTensor,
        weights: MPSGraphTensor,
        descriptor: MPSGraphDepthwiseConvolution3DOpDescriptor,
        name: NSString,
    ) -> MPSGraphTensor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, depthwiseConvolution3DWithSourceTensor : source, weightsTensor : weights, descriptor : descriptor, name : name)
    }
    unsafe fn depthwiseConvolution3DDataGradientWithIncomingGradientTensor_weightsTensor_outputShape_descriptor_name_(
        &self,
        incomingGradient: MPSGraphTensor,
        weights: MPSGraphTensor,
        outputShape: NSArray,
        descriptor: MPSGraphDepthwiseConvolution3DOpDescriptor,
        name: NSString,
    ) -> MPSGraphTensor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, depthwiseConvolution3DDataGradientWithIncomingGradientTensor : incomingGradient, weightsTensor : weights, outputShape : outputShape, descriptor : descriptor, name : name)
    }
    unsafe fn depthwiseConvolution3DWeightsGradientWithIncomingGradientTensor_sourceTensor_outputShape_descriptor_name_(
        &self,
        incomingGradient: MPSGraphTensor,
        source: MPSGraphTensor,
        outputShape: NSArray,
        descriptor: MPSGraphDepthwiseConvolution3DOpDescriptor,
        name: NSString,
    ) -> MPSGraphTensor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, depthwiseConvolution3DWeightsGradientWithIncomingGradientTensor : incomingGradient, sourceTensor : source, outputShape : outputShape, descriptor : descriptor, name : name)
    }
}
pub type MPSGraphFFTScalingMode = NSUInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MPSGraphFFTDescriptor(pub id);
impl std::ops::Deref for MPSGraphFFTDescriptor {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MPSGraphFFTDescriptor {}
impl MPSGraphFFTDescriptor {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MPSGraphFFTDescriptor").unwrap(), alloc) })
    }
}
impl PNSCopying for MPSGraphFFTDescriptor {}
impl IMPSGraphObject for MPSGraphFFTDescriptor {}
impl std::convert::TryFrom<MPSGraphObject> for MPSGraphFFTDescriptor {
    type Error = &'static str;
    fn try_from(parent: MPSGraphObject) -> Result<MPSGraphFFTDescriptor, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MPSGraphFFTDescriptor").unwrap()) };
        if is_kind_of {
            Ok(MPSGraphFFTDescriptor(parent.0))
        } else {
            Err("This MPSGraphObject cannot be downcasted to MPSGraphFFTDescriptor")
        }
    }
}
impl INSObject for MPSGraphFFTDescriptor {}
impl PNSObject for MPSGraphFFTDescriptor {}
impl IMPSGraphFFTDescriptor for MPSGraphFFTDescriptor {}
pub trait IMPSGraphFFTDescriptor: Sized + std::ops::Deref {
    unsafe fn inverse(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inverse)
    }
    unsafe fn setInverse_(&self, inverse: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInverse : inverse)
    }
    unsafe fn scalingMode(&self) -> MPSGraphFFTScalingMode
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, scalingMode)
    }
    unsafe fn setScalingMode_(&self, scalingMode: MPSGraphFFTScalingMode)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setScalingMode : scalingMode)
    }
    unsafe fn roundToOddHermitean(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, roundToOddHermitean)
    }
    unsafe fn setRoundToOddHermitean_(&self, roundToOddHermitean: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRoundToOddHermitean : roundToOddHermitean)
    }
    unsafe fn descriptor() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MPSGraphFFTDescriptor").unwrap(), descriptor)
    }
}
impl MPSGraph_MPSGraphFourierTransformOps for MPSGraph {}
pub trait MPSGraph_MPSGraphFourierTransformOps: Sized + std::ops::Deref {
    unsafe fn fastFourierTransformWithTensor_axes_descriptor_name_(
        &self,
        tensor: MPSGraphTensor,
        axes: NSArray,
        descriptor: MPSGraphFFTDescriptor,
        name: NSString,
    ) -> MPSGraphTensor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, fastFourierTransformWithTensor : tensor, axes : axes, descriptor : descriptor, name : name)
    }
    unsafe fn fastFourierTransformWithTensor_axesTensor_descriptor_name_(
        &self,
        tensor: MPSGraphTensor,
        axesTensor: MPSGraphTensor,
        descriptor: MPSGraphFFTDescriptor,
        name: NSString,
    ) -> MPSGraphTensor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, fastFourierTransformWithTensor : tensor, axesTensor : axesTensor, descriptor : descriptor, name : name)
    }
    unsafe fn realToHermiteanFFTWithTensor_axes_descriptor_name_(
        &self,
        tensor: MPSGraphTensor,
        axes: NSArray,
        descriptor: MPSGraphFFTDescriptor,
        name: NSString,
    ) -> MPSGraphTensor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, realToHermiteanFFTWithTensor : tensor, axes : axes, descriptor : descriptor, name : name)
    }
    unsafe fn realToHermiteanFFTWithTensor_axesTensor_descriptor_name_(
        &self,
        tensor: MPSGraphTensor,
        axesTensor: MPSGraphTensor,
        descriptor: MPSGraphFFTDescriptor,
        name: NSString,
    ) -> MPSGraphTensor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, realToHermiteanFFTWithTensor : tensor, axesTensor : axesTensor, descriptor : descriptor, name : name)
    }
    unsafe fn HermiteanToRealFFTWithTensor_axes_descriptor_name_(
        &self,
        tensor: MPSGraphTensor,
        axes: NSArray,
        descriptor: MPSGraphFFTDescriptor,
        name: NSString,
    ) -> MPSGraphTensor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, HermiteanToRealFFTWithTensor : tensor, axes : axes, descriptor : descriptor, name : name)
    }
    unsafe fn HermiteanToRealFFTWithTensor_axesTensor_descriptor_name_(
        &self,
        tensor: MPSGraphTensor,
        axesTensor: MPSGraphTensor,
        descriptor: MPSGraphFFTDescriptor,
        name: NSString,
    ) -> MPSGraphTensor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, HermiteanToRealFFTWithTensor : tensor, axesTensor : axesTensor, descriptor : descriptor, name : name)
    }
}
impl MPSGraph_GatherNDOps for MPSGraph {}
pub trait MPSGraph_GatherNDOps: Sized + std::ops::Deref {
    unsafe fn gatherNDWithUpdatesTensor_indicesTensor_batchDimensions_name_(
        &self,
        updatesTensor: MPSGraphTensor,
        indicesTensor: MPSGraphTensor,
        batchDimensions: NSUInteger,
        name: NSString,
    ) -> MPSGraphTensor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, gatherNDWithUpdatesTensor : updatesTensor, indicesTensor : indicesTensor, batchDimensions : batchDimensions, name : name)
    }
}
impl MPSGraph_GatherOps for MPSGraph {}
pub trait MPSGraph_GatherOps: Sized + std::ops::Deref {
    unsafe fn gatherWithUpdatesTensor_indicesTensor_axis_batchDimensions_name_(
        &self,
        updatesTensor: MPSGraphTensor,
        indicesTensor: MPSGraphTensor,
        axis: NSUInteger,
        batchDimensions: NSUInteger,
        name: NSString,
    ) -> MPSGraphTensor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, gatherWithUpdatesTensor : updatesTensor, indicesTensor : indicesTensor, axis : axis, batchDimensions : batchDimensions, name : name)
    }
}
impl MPSGraph_MPSGraphGatherAlongAxisOps for MPSGraph {}
pub trait MPSGraph_MPSGraphGatherAlongAxisOps: Sized + std::ops::Deref {
    unsafe fn gatherAlongAxis_withUpdatesTensor_indicesTensor_name_(
        &self,
        axis: NSInteger,
        updatesTensor: MPSGraphTensor,
        indicesTensor: MPSGraphTensor,
        name: NSString,
    ) -> MPSGraphTensor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, gatherAlongAxis : axis, withUpdatesTensor : updatesTensor, indicesTensor : indicesTensor, name : name)
    }
    unsafe fn gatherAlongAxisTensor_withUpdatesTensor_indicesTensor_name_(
        &self,
        axisTensor: MPSGraphTensor,
        updatesTensor: MPSGraphTensor,
        indicesTensor: MPSGraphTensor,
        name: NSString,
    ) -> MPSGraphTensor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, gatherAlongAxisTensor : axisTensor, withUpdatesTensor : updatesTensor, indicesTensor : indicesTensor, name : name)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MPSGraphImToColOpDescriptor(pub id);
impl std::ops::Deref for MPSGraphImToColOpDescriptor {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MPSGraphImToColOpDescriptor {}
impl MPSGraphImToColOpDescriptor {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MPSGraphImToColOpDescriptor").unwrap(), alloc) })
    }
}
impl PNSCopying for MPSGraphImToColOpDescriptor {}
impl IMPSGraphObject for MPSGraphImToColOpDescriptor {}
impl std::convert::TryFrom<MPSGraphObject> for MPSGraphImToColOpDescriptor {
    type Error = &'static str;
    fn try_from(parent: MPSGraphObject) -> Result<MPSGraphImToColOpDescriptor, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MPSGraphImToColOpDescriptor").unwrap()) };
        if is_kind_of {
            Ok(MPSGraphImToColOpDescriptor(parent.0))
        } else {
            Err("This MPSGraphObject cannot be downcasted to MPSGraphImToColOpDescriptor")
        }
    }
}
impl INSObject for MPSGraphImToColOpDescriptor {}
impl PNSObject for MPSGraphImToColOpDescriptor {}
impl IMPSGraphImToColOpDescriptor for MPSGraphImToColOpDescriptor {}
pub trait IMPSGraphImToColOpDescriptor: Sized + std::ops::Deref {
    unsafe fn setExplicitPaddingWithPaddingLeft_paddingRight_paddingTop_paddingBottom_(
        &self,
        paddingLeft: NSUInteger,
        paddingRight: NSUInteger,
        paddingTop: NSUInteger,
        paddingBottom: NSUInteger,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setExplicitPaddingWithPaddingLeft : paddingLeft, paddingRight : paddingRight, paddingTop : paddingTop, paddingBottom : paddingBottom)
    }
    unsafe fn kernelWidth(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, kernelWidth)
    }
    unsafe fn setKernelWidth_(&self, kernelWidth: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setKernelWidth : kernelWidth)
    }
    unsafe fn kernelHeight(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, kernelHeight)
    }
    unsafe fn setKernelHeight_(&self, kernelHeight: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setKernelHeight : kernelHeight)
    }
    unsafe fn strideInX(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, strideInX)
    }
    unsafe fn setStrideInX_(&self, strideInX: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setStrideInX : strideInX)
    }
    unsafe fn strideInY(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, strideInY)
    }
    unsafe fn setStrideInY_(&self, strideInY: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setStrideInY : strideInY)
    }
    unsafe fn dilationRateInX(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, dilationRateInX)
    }
    unsafe fn setDilationRateInX_(&self, dilationRateInX: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDilationRateInX : dilationRateInX)
    }
    unsafe fn dilationRateInY(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, dilationRateInY)
    }
    unsafe fn setDilationRateInY_(&self, dilationRateInY: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDilationRateInY : dilationRateInY)
    }
    unsafe fn paddingLeft(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, paddingLeft)
    }
    unsafe fn setPaddingLeft_(&self, paddingLeft: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPaddingLeft : paddingLeft)
    }
    unsafe fn paddingRight(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, paddingRight)
    }
    unsafe fn setPaddingRight_(&self, paddingRight: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPaddingRight : paddingRight)
    }
    unsafe fn paddingTop(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, paddingTop)
    }
    unsafe fn setPaddingTop_(&self, paddingTop: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPaddingTop : paddingTop)
    }
    unsafe fn paddingBottom(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, paddingBottom)
    }
    unsafe fn setPaddingBottom_(&self, paddingBottom: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPaddingBottom : paddingBottom)
    }
    unsafe fn dataLayout(&self) -> MPSGraphTensorNamedDataLayout
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, dataLayout)
    }
    unsafe fn setDataLayout_(&self, dataLayout: MPSGraphTensorNamedDataLayout)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDataLayout : dataLayout)
    }
    unsafe fn descriptorWithKernelWidth_kernelHeight_strideInX_strideInY_dilationRateInX_dilationRateInY_paddingLeft_paddingRight_paddingTop_paddingBottom_dataLayout_(
        kernelWidth: NSUInteger,
        kernelHeight: NSUInteger,
        strideInX: NSUInteger,
        strideInY: NSUInteger,
        dilationRateInX: NSUInteger,
        dilationRateInY: NSUInteger,
        paddingLeft: NSUInteger,
        paddingRight: NSUInteger,
        paddingTop: NSUInteger,
        paddingBottom: NSUInteger,
        dataLayout: MPSGraphTensorNamedDataLayout,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MPSGraphImToColOpDescriptor").unwrap(), descriptorWithKernelWidth : kernelWidth, kernelHeight : kernelHeight, strideInX : strideInX, strideInY : strideInY, dilationRateInX : dilationRateInX, dilationRateInY : dilationRateInY, paddingLeft : paddingLeft, paddingRight : paddingRight, paddingTop : paddingTop, paddingBottom : paddingBottom, dataLayout : dataLayout)
    }
    unsafe fn descriptorWithKernelWidth_kernelHeight_strideInX_strideInY_dilationRateInX_dilationRateInY_dataLayout_(
        kernelWidth: NSUInteger,
        kernelHeight: NSUInteger,
        strideInX: NSUInteger,
        strideInY: NSUInteger,
        dilationRateInX: NSUInteger,
        dilationRateInY: NSUInteger,
        dataLayout: MPSGraphTensorNamedDataLayout,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MPSGraphImToColOpDescriptor").unwrap(), descriptorWithKernelWidth : kernelWidth, kernelHeight : kernelHeight, strideInX : strideInX, strideInY : strideInY, dilationRateInX : dilationRateInX, dilationRateInY : dilationRateInY, dataLayout : dataLayout)
    }
}
impl MPSGraph_MPSGraphImToColOps for MPSGraph {}
pub trait MPSGraph_MPSGraphImToColOps: Sized + std::ops::Deref {
    unsafe fn imToColWithSourceTensor_descriptor_name_(
        &self,
        source: MPSGraphTensor,
        descriptor: MPSGraphImToColOpDescriptor,
        name: NSString,
    ) -> MPSGraphTensor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, imToColWithSourceTensor : source, descriptor : descriptor, name : name)
    }
    unsafe fn colToImWithSourceTensor_outputShape_descriptor_name_(
        &self,
        source: MPSGraphTensor,
        outputShape: NSArray,
        descriptor: MPSGraphImToColOpDescriptor,
        name: NSString,
    ) -> MPSGraphTensor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, colToImWithSourceTensor : source, outputShape : outputShape, descriptor : descriptor, name : name)
    }
}
impl MPSGraph_MPSGraphLinearAlgebraOps for MPSGraph {}
pub trait MPSGraph_MPSGraphLinearAlgebraOps: Sized + std::ops::Deref {
    unsafe fn bandPartWithTensor_numLower_numUpper_name_(
        &self,
        inputTensor: MPSGraphTensor,
        numLower: NSInteger,
        numUpper: NSInteger,
        name: NSString,
    ) -> MPSGraphTensor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, bandPartWithTensor : inputTensor, numLower : numLower, numUpper : numUpper, name : name)
    }
    unsafe fn bandPartWithTensor_numLowerTensor_numUpperTensor_name_(
        &self,
        inputTensor: MPSGraphTensor,
        numLowerTensor: MPSGraphTensor,
        numUpperTensor: MPSGraphTensor,
        name: NSString,
    ) -> MPSGraphTensor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, bandPartWithTensor : inputTensor, numLowerTensor : numLowerTensor, numUpperTensor : numUpperTensor, name : name)
    }
}
pub type MPSGraphLossReductionType = u64;
impl MPSGraph_MPSGraphLossOps for MPSGraph {}
pub trait MPSGraph_MPSGraphLossOps: Sized + std::ops::Deref {
    unsafe fn softMaxCrossEntropyWithSourceTensor_labelsTensor_axis_reductionType_name_(
        &self,
        sourceTensor: MPSGraphTensor,
        labelsTensor: MPSGraphTensor,
        axis: NSInteger,
        reductionType: MPSGraphLossReductionType,
        name: NSString,
    ) -> MPSGraphTensor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, softMaxCrossEntropyWithSourceTensor : sourceTensor, labelsTensor : labelsTensor, axis : axis, reductionType : reductionType, name : name)
    }
    unsafe fn softMaxCrossEntropyGradientWithIncomingGradientTensor_sourceTensor_labelsTensor_axis_reductionType_name_(
        &self,
        gradientTensor: MPSGraphTensor,
        sourceTensor: MPSGraphTensor,
        labelsTensor: MPSGraphTensor,
        axis: NSInteger,
        reductionType: MPSGraphLossReductionType,
        name: NSString,
    ) -> MPSGraphTensor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, softMaxCrossEntropyGradientWithIncomingGradientTensor : gradientTensor, sourceTensor : sourceTensor, labelsTensor : labelsTensor, axis : axis, reductionType : reductionType, name : name)
    }
}
impl MPSGraph_MPSGraphMatrixInverseOps for MPSGraph {}
pub trait MPSGraph_MPSGraphMatrixInverseOps: Sized + std::ops::Deref {
    unsafe fn inverseOfTensor_name_(
        &self,
        inputTensor: MPSGraphTensor,
        name: NSString,
    ) -> MPSGraphTensor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, inverseOfTensor : inputTensor, name : name)
    }
}
impl MPSGraph_MPSGraphMatrixMultiplicationOps for MPSGraph {}
pub trait MPSGraph_MPSGraphMatrixMultiplicationOps: Sized + std::ops::Deref {
    unsafe fn matrixMultiplicationWithPrimaryTensor_secondaryTensor_name_(
        &self,
        primaryTensor: MPSGraphTensor,
        secondaryTensor: MPSGraphTensor,
        name: NSString,
    ) -> MPSGraphTensor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, matrixMultiplicationWithPrimaryTensor : primaryTensor, secondaryTensor : secondaryTensor, name : name)
    }
    unsafe fn HammingDistanceWithPrimaryTensor_secondaryTensor_resultDataType_name_(
        &self,
        primaryTensor: MPSGraphTensor,
        secondaryTensor: MPSGraphTensor,
        resultDataType: MPSDataType,
        name: NSString,
    ) -> MPSGraphTensor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, HammingDistanceWithPrimaryTensor : primaryTensor, secondaryTensor : secondaryTensor, resultDataType : resultDataType, name : name)
    }
    unsafe fn scaledDotProductAttentionWithQueryTensor_keyTensor_valueTensor_maskTensor_scale_name_(
        &self,
        queryTensor: MPSGraphTensor,
        keyTensor: MPSGraphTensor,
        valueTensor: MPSGraphTensor,
        maskTensor: MPSGraphTensor,
        scale: f32,
        name: NSString,
    ) -> MPSGraphTensor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, scaledDotProductAttentionWithQueryTensor : queryTensor, keyTensor : keyTensor, valueTensor : valueTensor, maskTensor : maskTensor, scale : scale, name : name)
    }
    unsafe fn scaledDotProductAttentionWithQueryTensor_keyTensor_valueTensor_scale_name_(
        &self,
        queryTensor: MPSGraphTensor,
        keyTensor: MPSGraphTensor,
        valueTensor: MPSGraphTensor,
        scale: f32,
        name: NSString,
    ) -> MPSGraphTensor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, scaledDotProductAttentionWithQueryTensor : queryTensor, keyTensor : keyTensor, valueTensor : valueTensor, scale : scale, name : name)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MPSGraphVariableOp(pub id);
impl std::ops::Deref for MPSGraphVariableOp {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MPSGraphVariableOp {}
impl MPSGraphVariableOp {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MPSGraphVariableOp").unwrap(), alloc) })
    }
}
impl IMPSGraphOperation for MPSGraphVariableOp {}
impl PNSCopying for MPSGraphVariableOp {}
impl From<MPSGraphVariableOp> for MPSGraphOperation {
    fn from(child: MPSGraphVariableOp) -> MPSGraphOperation {
        MPSGraphOperation(child.0)
    }
}
impl std::convert::TryFrom<MPSGraphOperation> for MPSGraphVariableOp {
    type Error = &'static str;
    fn try_from(parent: MPSGraphOperation) -> Result<MPSGraphVariableOp, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MPSGraphVariableOp").unwrap()) };
        if is_kind_of {
            Ok(MPSGraphVariableOp(parent.0))
        } else {
            Err("This MPSGraphOperation cannot be downcasted to MPSGraphVariableOp")
        }
    }
}
impl IMPSGraphObject for MPSGraphVariableOp {}
impl INSObject for MPSGraphVariableOp {}
impl PNSObject for MPSGraphVariableOp {}
impl IMPSGraphVariableOp for MPSGraphVariableOp {}
pub trait IMPSGraphVariableOp: Sized + std::ops::Deref {
    unsafe fn shape(&self) -> *mut MPSShape
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, shape)
    }
    unsafe fn dataType(&self) -> MPSDataType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, dataType)
    }
}
impl MPSGraph_MemoryOps for MPSGraph {}
pub trait MPSGraph_MemoryOps: Sized + std::ops::Deref {
    unsafe fn placeholderWithShape_dataType_name_(
        &self,
        shape: NSArray,
        dataType: MPSDataType,
        name: NSString,
    ) -> MPSGraphTensor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, placeholderWithShape : shape, dataType : dataType, name : name)
    }
    unsafe fn placeholderWithShape_name_(&self, shape: NSArray, name: NSString) -> MPSGraphTensor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, placeholderWithShape : shape, name : name)
    }
    unsafe fn constantWithData_shape_dataType_(
        &self,
        data: NSData,
        shape: NSArray,
        dataType: MPSDataType,
    ) -> MPSGraphTensor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, constantWithData : data, shape : shape, dataType : dataType)
    }
    unsafe fn constantWithScalar_dataType_(
        &self,
        scalar: f64,
        dataType: MPSDataType,
    ) -> MPSGraphTensor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, constantWithScalar : scalar, dataType : dataType)
    }
    unsafe fn constantWithScalar_shape_dataType_(
        &self,
        scalar: f64,
        shape: NSArray,
        dataType: MPSDataType,
    ) -> MPSGraphTensor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, constantWithScalar : scalar, shape : shape, dataType : dataType)
    }
    unsafe fn constantWithRealPart_imaginaryPart_(
        &self,
        realPart: f64,
        imaginaryPart: f64,
    ) -> MPSGraphTensor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, constantWithRealPart : realPart, imaginaryPart : imaginaryPart)
    }
    unsafe fn constantWithRealPart_imaginaryPart_dataType_(
        &self,
        realPart: f64,
        imaginaryPart: f64,
        dataType: MPSDataType,
    ) -> MPSGraphTensor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, constantWithRealPart : realPart, imaginaryPart : imaginaryPart, dataType : dataType)
    }
    unsafe fn constantWithRealPart_imaginaryPart_shape_dataType_(
        &self,
        realPart: f64,
        imaginaryPart: f64,
        shape: NSArray,
        dataType: MPSDataType,
    ) -> MPSGraphTensor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, constantWithRealPart : realPart, imaginaryPart : imaginaryPart, shape : shape, dataType : dataType)
    }
    unsafe fn variableWithData_shape_dataType_name_(
        &self,
        data: NSData,
        shape: NSArray,
        dataType: MPSDataType,
        name: NSString,
    ) -> MPSGraphTensor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, variableWithData : data, shape : shape, dataType : dataType, name : name)
    }
    unsafe fn variableFromTensorWithTensor_name_(
        &self,
        tensor: MPSGraphTensor,
        name: NSString,
    ) -> MPSGraphTensor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, variableFromTensorWithTensor : tensor, name : name)
    }
    unsafe fn readVariable_name_(&self, variable: MPSGraphTensor, name: NSString) -> MPSGraphTensor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, readVariable : variable, name : name)
    }
    unsafe fn assignVariable_withValueOfTensor_name_(
        &self,
        variable: MPSGraphTensor,
        tensor: MPSGraphTensor,
        name: NSString,
    ) -> MPSGraphOperation
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, assignVariable : variable, withValueOfTensor : tensor, name : name)
    }
}
pub type MPSGraphNonMaximumSuppressionCoordinateMode = NSUInteger;
impl MPSGraph_MPSGraphNonMaximumSuppressionOps for MPSGraph {}
pub trait MPSGraph_MPSGraphNonMaximumSuppressionOps: Sized + std::ops::Deref {
    unsafe fn nonMaximumSuppressionWithBoxesTensor_scoresTensor_IOUThreshold_scoreThreshold_perClassSuppression_coordinateMode_name_(
        &self,
        boxesTensor: MPSGraphTensor,
        scoresTensor: MPSGraphTensor,
        IOUThreshold: f32,
        scoreThreshold: f32,
        perClassSuppression: BOOL,
        coordinateMode: MPSGraphNonMaximumSuppressionCoordinateMode,
        name: NSString,
    ) -> MPSGraphTensor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, nonMaximumSuppressionWithBoxesTensor : boxesTensor, scoresTensor : scoresTensor, IOUThreshold : IOUThreshold, scoreThreshold : scoreThreshold, perClassSuppression : perClassSuppression, coordinateMode : coordinateMode, name : name)
    }
    unsafe fn nonMaximumSuppressionWithBoxesTensor_scoresTensor_classIndicesTensor_IOUThreshold_scoreThreshold_perClassSuppression_coordinateMode_name_(
        &self,
        boxesTensor: MPSGraphTensor,
        scoresTensor: MPSGraphTensor,
        classIndicesTensor: MPSGraphTensor,
        IOUThreshold: f32,
        scoreThreshold: f32,
        perClassSuppression: BOOL,
        coordinateMode: MPSGraphNonMaximumSuppressionCoordinateMode,
        name: NSString,
    ) -> MPSGraphTensor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, nonMaximumSuppressionWithBoxesTensor : boxesTensor, scoresTensor : scoresTensor, classIndicesTensor : classIndicesTensor, IOUThreshold : IOUThreshold, scoreThreshold : scoreThreshold, perClassSuppression : perClassSuppression, coordinateMode : coordinateMode, name : name)
    }
}
impl MPSGraph_NonZeroOps for MPSGraph {}
pub trait MPSGraph_NonZeroOps: Sized + std::ops::Deref {
    unsafe fn nonZeroIndicesOfTensor_name_(
        &self,
        tensor: MPSGraphTensor,
        name: NSString,
    ) -> MPSGraphTensor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, nonZeroIndicesOfTensor : tensor, name : name)
    }
}
impl MPSGraph_MPSGraphNormalizationOps for MPSGraph {}
pub trait MPSGraph_MPSGraphNormalizationOps: Sized + std::ops::Deref {
    unsafe fn meanOfTensor_axes_name_(
        &self,
        tensor: MPSGraphTensor,
        axes: NSArray,
        name: NSString,
    ) -> MPSGraphTensor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, meanOfTensor : tensor, axes : axes, name : name)
    }
    unsafe fn varianceOfTensor_meanTensor_axes_name_(
        &self,
        tensor: MPSGraphTensor,
        meanTensor: MPSGraphTensor,
        axes: NSArray,
        name: NSString,
    ) -> MPSGraphTensor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, varianceOfTensor : tensor, meanTensor : meanTensor, axes : axes, name : name)
    }
    unsafe fn varianceOfTensor_axes_name_(
        &self,
        tensor: MPSGraphTensor,
        axes: NSArray,
        name: NSString,
    ) -> MPSGraphTensor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, varianceOfTensor : tensor, axes : axes, name : name)
    }
    unsafe fn normalizationWithTensor_meanTensor_varianceTensor_gammaTensor_betaTensor_epsilon_name_(
        &self,
        tensor: MPSGraphTensor,
        mean: MPSGraphTensor,
        variance: MPSGraphTensor,
        gamma: MPSGraphTensor,
        beta: MPSGraphTensor,
        epsilon: f32,
        name: NSString,
    ) -> MPSGraphTensor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, normalizationWithTensor : tensor, meanTensor : mean, varianceTensor : variance, gammaTensor : gamma, betaTensor : beta, epsilon : epsilon, name : name)
    }
    unsafe fn normalizationGammaGradientWithIncomingGradientTensor_sourceTensor_meanTensor_varianceTensor_reductionAxes_epsilon_name_(
        &self,
        incomingGradientTensor: MPSGraphTensor,
        sourceTensor: MPSGraphTensor,
        meanTensor: MPSGraphTensor,
        varianceTensor: MPSGraphTensor,
        axes: NSArray,
        epsilon: f32,
        name: NSString,
    ) -> MPSGraphTensor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, normalizationGammaGradientWithIncomingGradientTensor : incomingGradientTensor, sourceTensor : sourceTensor, meanTensor : meanTensor, varianceTensor : varianceTensor, reductionAxes : axes, epsilon : epsilon, name : name)
    }
    unsafe fn normalizationBetaGradientWithIncomingGradientTensor_sourceTensor_reductionAxes_name_(
        &self,
        incomingGradientTensor: MPSGraphTensor,
        sourceTensor: MPSGraphTensor,
        axes: NSArray,
        name: NSString,
    ) -> MPSGraphTensor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, normalizationBetaGradientWithIncomingGradientTensor : incomingGradientTensor, sourceTensor : sourceTensor, reductionAxes : axes, name : name)
    }
    unsafe fn normalizationGradientWithIncomingGradientTensor_sourceTensor_meanTensor_varianceTensor_gammaTensor_gammaGradientTensor_betaGradientTensor_reductionAxes_epsilon_name_(
        &self,
        incomingGradientTensor: MPSGraphTensor,
        sourceTensor: MPSGraphTensor,
        meanTensor: MPSGraphTensor,
        varianceTensor: MPSGraphTensor,
        gamma: MPSGraphTensor,
        gammaGradient: MPSGraphTensor,
        betaGradient: MPSGraphTensor,
        axes: NSArray,
        epsilon: f32,
        name: NSString,
    ) -> MPSGraphTensor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, normalizationGradientWithIncomingGradientTensor : incomingGradientTensor, sourceTensor : sourceTensor, meanTensor : meanTensor, varianceTensor : varianceTensor, gammaTensor : gamma, gammaGradientTensor : gammaGradient, betaGradientTensor : betaGradient, reductionAxes : axes, epsilon : epsilon, name : name)
    }
}
impl MPSGraph_MPSGraphOneHotOps for MPSGraph {}
pub trait MPSGraph_MPSGraphOneHotOps: Sized + std::ops::Deref {
    unsafe fn oneHotWithIndicesTensor_depth_axis_dataType_onValue_offValue_name_(
        &self,
        indicesTensor: MPSGraphTensor,
        depth: NSUInteger,
        axis: NSUInteger,
        dataType: MPSDataType,
        onValue: f64,
        offValue: f64,
        name: NSString,
    ) -> MPSGraphTensor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, oneHotWithIndicesTensor : indicesTensor, depth : depth, axis : axis, dataType : dataType, onValue : onValue, offValue : offValue, name : name)
    }
    unsafe fn oneHotWithIndicesTensor_depth_dataType_onValue_offValue_name_(
        &self,
        indicesTensor: MPSGraphTensor,
        depth: NSUInteger,
        dataType: MPSDataType,
        onValue: f64,
        offValue: f64,
        name: NSString,
    ) -> MPSGraphTensor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, oneHotWithIndicesTensor : indicesTensor, depth : depth, dataType : dataType, onValue : onValue, offValue : offValue, name : name)
    }
    unsafe fn oneHotWithIndicesTensor_depth_axis_dataType_name_(
        &self,
        indicesTensor: MPSGraphTensor,
        depth: NSUInteger,
        axis: NSUInteger,
        dataType: MPSDataType,
        name: NSString,
    ) -> MPSGraphTensor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, oneHotWithIndicesTensor : indicesTensor, depth : depth, axis : axis, dataType : dataType, name : name)
    }
    unsafe fn oneHotWithIndicesTensor_depth_axis_name_(
        &self,
        indicesTensor: MPSGraphTensor,
        depth: NSUInteger,
        axis: NSUInteger,
        name: NSString,
    ) -> MPSGraphTensor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, oneHotWithIndicesTensor : indicesTensor, depth : depth, axis : axis, name : name)
    }
    unsafe fn oneHotWithIndicesTensor_depth_dataType_name_(
        &self,
        indicesTensor: MPSGraphTensor,
        depth: NSUInteger,
        dataType: MPSDataType,
        name: NSString,
    ) -> MPSGraphTensor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, oneHotWithIndicesTensor : indicesTensor, depth : depth, dataType : dataType, name : name)
    }
    unsafe fn oneHotWithIndicesTensor_depth_name_(
        &self,
        indicesTensor: MPSGraphTensor,
        depth: NSUInteger,
        name: NSString,
    ) -> MPSGraphTensor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, oneHotWithIndicesTensor : indicesTensor, depth : depth, name : name)
    }
}
impl MPSGraph_MPSGraphOptimizerOps for MPSGraph {}
pub trait MPSGraph_MPSGraphOptimizerOps: Sized + std::ops::Deref {
    unsafe fn stochasticGradientDescentWithLearningRateTensor_valuesTensor_gradientTensor_name_(
        &self,
        learningRateTensor: MPSGraphTensor,
        valuesTensor: MPSGraphTensor,
        gradientTensor: MPSGraphTensor,
        name: NSString,
    ) -> MPSGraphTensor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, stochasticGradientDescentWithLearningRateTensor : learningRateTensor, valuesTensor : valuesTensor, gradientTensor : gradientTensor, name : name)
    }
    unsafe fn applyStochasticGradientDescentWithLearningRateTensor_variable_gradientTensor_name_(
        &self,
        learningRateTensor: MPSGraphTensor,
        variable: MPSGraphVariableOp,
        gradientTensor: MPSGraphTensor,
        name: NSString,
    ) -> MPSGraphOperation
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, applyStochasticGradientDescentWithLearningRateTensor : learningRateTensor, variable : variable, gradientTensor : gradientTensor, name : name)
    }
    unsafe fn adamWithLearningRateTensor_beta1Tensor_beta2Tensor_epsilonTensor_beta1PowerTensor_beta2PowerTensor_valuesTensor_momentumTensor_velocityTensor_maximumVelocityTensor_gradientTensor_name_(
        &self,
        learningRateTensor: MPSGraphTensor,
        beta1Tensor: MPSGraphTensor,
        beta2Tensor: MPSGraphTensor,
        epsilonTensor: MPSGraphTensor,
        beta1PowerTensor: MPSGraphTensor,
        beta2PowerTensor: MPSGraphTensor,
        valuesTensor: MPSGraphTensor,
        momentumTensor: MPSGraphTensor,
        velocityTensor: MPSGraphTensor,
        maximumVelocityTensor: MPSGraphTensor,
        gradientTensor: MPSGraphTensor,
        name: NSString,
    ) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, adamWithLearningRateTensor : learningRateTensor, beta1Tensor : beta1Tensor, beta2Tensor : beta2Tensor, epsilonTensor : epsilonTensor, beta1PowerTensor : beta1PowerTensor, beta2PowerTensor : beta2PowerTensor, valuesTensor : valuesTensor, momentumTensor : momentumTensor, velocityTensor : velocityTensor, maximumVelocityTensor : maximumVelocityTensor, gradientTensor : gradientTensor, name : name)
    }
    unsafe fn adamWithCurrentLearningRateTensor_beta1Tensor_beta2Tensor_epsilonTensor_valuesTensor_momentumTensor_velocityTensor_maximumVelocityTensor_gradientTensor_name_(
        &self,
        currentLearningRateTensor: MPSGraphTensor,
        beta1Tensor: MPSGraphTensor,
        beta2Tensor: MPSGraphTensor,
        epsilonTensor: MPSGraphTensor,
        valuesTensor: MPSGraphTensor,
        momentumTensor: MPSGraphTensor,
        velocityTensor: MPSGraphTensor,
        maximumVelocityTensor: MPSGraphTensor,
        gradientTensor: MPSGraphTensor,
        name: NSString,
    ) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, adamWithCurrentLearningRateTensor : currentLearningRateTensor, beta1Tensor : beta1Tensor, beta2Tensor : beta2Tensor, epsilonTensor : epsilonTensor, valuesTensor : valuesTensor, momentumTensor : momentumTensor, velocityTensor : velocityTensor, maximumVelocityTensor : maximumVelocityTensor, gradientTensor : gradientTensor, name : name)
    }
}
pub type MPSGraphPoolingReturnIndicesMode = NSUInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MPSGraphPooling2DOpDescriptor(pub id);
impl std::ops::Deref for MPSGraphPooling2DOpDescriptor {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MPSGraphPooling2DOpDescriptor {}
impl MPSGraphPooling2DOpDescriptor {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MPSGraphPooling2DOpDescriptor").unwrap(), alloc) })
    }
}
impl PNSCopying for MPSGraphPooling2DOpDescriptor {}
impl IMPSGraphObject for MPSGraphPooling2DOpDescriptor {}
impl std::convert::TryFrom<MPSGraphObject> for MPSGraphPooling2DOpDescriptor {
    type Error = &'static str;
    fn try_from(parent: MPSGraphObject) -> Result<MPSGraphPooling2DOpDescriptor, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MPSGraphPooling2DOpDescriptor").unwrap())
        };
        if is_kind_of {
            Ok(MPSGraphPooling2DOpDescriptor(parent.0))
        } else {
            Err("This MPSGraphObject cannot be downcasted to MPSGraphPooling2DOpDescriptor")
        }
    }
}
impl INSObject for MPSGraphPooling2DOpDescriptor {}
impl PNSObject for MPSGraphPooling2DOpDescriptor {}
impl IMPSGraphPooling2DOpDescriptor for MPSGraphPooling2DOpDescriptor {}
pub trait IMPSGraphPooling2DOpDescriptor: Sized + std::ops::Deref {
    unsafe fn setExplicitPaddingWithPaddingLeft_paddingRight_paddingTop_paddingBottom_(
        &self,
        paddingLeft: NSUInteger,
        paddingRight: NSUInteger,
        paddingTop: NSUInteger,
        paddingBottom: NSUInteger,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setExplicitPaddingWithPaddingLeft : paddingLeft, paddingRight : paddingRight, paddingTop : paddingTop, paddingBottom : paddingBottom)
    }
    unsafe fn kernelWidth(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, kernelWidth)
    }
    unsafe fn setKernelWidth_(&self, kernelWidth: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setKernelWidth : kernelWidth)
    }
    unsafe fn kernelHeight(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, kernelHeight)
    }
    unsafe fn setKernelHeight_(&self, kernelHeight: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setKernelHeight : kernelHeight)
    }
    unsafe fn strideInX(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, strideInX)
    }
    unsafe fn setStrideInX_(&self, strideInX: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setStrideInX : strideInX)
    }
    unsafe fn strideInY(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, strideInY)
    }
    unsafe fn setStrideInY_(&self, strideInY: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setStrideInY : strideInY)
    }
    unsafe fn dilationRateInX(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, dilationRateInX)
    }
    unsafe fn setDilationRateInX_(&self, dilationRateInX: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDilationRateInX : dilationRateInX)
    }
    unsafe fn dilationRateInY(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, dilationRateInY)
    }
    unsafe fn setDilationRateInY_(&self, dilationRateInY: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDilationRateInY : dilationRateInY)
    }
    unsafe fn paddingLeft(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, paddingLeft)
    }
    unsafe fn setPaddingLeft_(&self, paddingLeft: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPaddingLeft : paddingLeft)
    }
    unsafe fn paddingRight(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, paddingRight)
    }
    unsafe fn setPaddingRight_(&self, paddingRight: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPaddingRight : paddingRight)
    }
    unsafe fn paddingTop(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, paddingTop)
    }
    unsafe fn setPaddingTop_(&self, paddingTop: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPaddingTop : paddingTop)
    }
    unsafe fn paddingBottom(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, paddingBottom)
    }
    unsafe fn setPaddingBottom_(&self, paddingBottom: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPaddingBottom : paddingBottom)
    }
    unsafe fn paddingStyle(&self) -> MPSGraphPaddingStyle
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, paddingStyle)
    }
    unsafe fn setPaddingStyle_(&self, paddingStyle: MPSGraphPaddingStyle)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPaddingStyle : paddingStyle)
    }
    unsafe fn dataLayout(&self) -> MPSGraphTensorNamedDataLayout
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, dataLayout)
    }
    unsafe fn setDataLayout_(&self, dataLayout: MPSGraphTensorNamedDataLayout)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDataLayout : dataLayout)
    }
    unsafe fn returnIndicesMode(&self) -> MPSGraphPoolingReturnIndicesMode
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, returnIndicesMode)
    }
    unsafe fn setReturnIndicesMode_(&self, returnIndicesMode: MPSGraphPoolingReturnIndicesMode)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setReturnIndicesMode : returnIndicesMode)
    }
    unsafe fn returnIndicesDataType(&self) -> MPSDataType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, returnIndicesDataType)
    }
    unsafe fn setReturnIndicesDataType_(&self, returnIndicesDataType: MPSDataType)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setReturnIndicesDataType : returnIndicesDataType)
    }
    unsafe fn ceilMode(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, ceilMode)
    }
    unsafe fn setCeilMode_(&self, ceilMode: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCeilMode : ceilMode)
    }
    unsafe fn includeZeroPadToAverage(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, includeZeroPadToAverage)
    }
    unsafe fn setIncludeZeroPadToAverage_(&self, includeZeroPadToAverage: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setIncludeZeroPadToAverage : includeZeroPadToAverage)
    }
    unsafe fn descriptorWithKernelWidth_kernelHeight_strideInX_strideInY_dilationRateInX_dilationRateInY_paddingLeft_paddingRight_paddingTop_paddingBottom_paddingStyle_dataLayout_(
        kernelWidth: NSUInteger,
        kernelHeight: NSUInteger,
        strideInX: NSUInteger,
        strideInY: NSUInteger,
        dilationRateInX: NSUInteger,
        dilationRateInY: NSUInteger,
        paddingLeft: NSUInteger,
        paddingRight: NSUInteger,
        paddingTop: NSUInteger,
        paddingBottom: NSUInteger,
        paddingStyle: MPSGraphPaddingStyle,
        dataLayout: MPSGraphTensorNamedDataLayout,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MPSGraphPooling2DOpDescriptor").unwrap(), descriptorWithKernelWidth : kernelWidth, kernelHeight : kernelHeight, strideInX : strideInX, strideInY : strideInY, dilationRateInX : dilationRateInX, dilationRateInY : dilationRateInY, paddingLeft : paddingLeft, paddingRight : paddingRight, paddingTop : paddingTop, paddingBottom : paddingBottom, paddingStyle : paddingStyle, dataLayout : dataLayout)
    }
    unsafe fn descriptorWithKernelWidth_kernelHeight_strideInX_strideInY_paddingStyle_dataLayout_(
        kernelWidth: NSUInteger,
        kernelHeight: NSUInteger,
        strideInX: NSUInteger,
        strideInY: NSUInteger,
        paddingStyle: MPSGraphPaddingStyle,
        dataLayout: MPSGraphTensorNamedDataLayout,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MPSGraphPooling2DOpDescriptor").unwrap(), descriptorWithKernelWidth : kernelWidth, kernelHeight : kernelHeight, strideInX : strideInX, strideInY : strideInY, paddingStyle : paddingStyle, dataLayout : dataLayout)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MPSGraphPooling4DOpDescriptor(pub id);
impl std::ops::Deref for MPSGraphPooling4DOpDescriptor {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MPSGraphPooling4DOpDescriptor {}
impl MPSGraphPooling4DOpDescriptor {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MPSGraphPooling4DOpDescriptor").unwrap(), alloc) })
    }
}
impl PNSCopying for MPSGraphPooling4DOpDescriptor {}
impl IMPSGraphObject for MPSGraphPooling4DOpDescriptor {}
impl std::convert::TryFrom<MPSGraphObject> for MPSGraphPooling4DOpDescriptor {
    type Error = &'static str;
    fn try_from(parent: MPSGraphObject) -> Result<MPSGraphPooling4DOpDescriptor, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MPSGraphPooling4DOpDescriptor").unwrap())
        };
        if is_kind_of {
            Ok(MPSGraphPooling4DOpDescriptor(parent.0))
        } else {
            Err("This MPSGraphObject cannot be downcasted to MPSGraphPooling4DOpDescriptor")
        }
    }
}
impl INSObject for MPSGraphPooling4DOpDescriptor {}
impl PNSObject for MPSGraphPooling4DOpDescriptor {}
impl IMPSGraphPooling4DOpDescriptor for MPSGraphPooling4DOpDescriptor {}
pub trait IMPSGraphPooling4DOpDescriptor: Sized + std::ops::Deref {
    unsafe fn kernelSizes(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, kernelSizes)
    }
    unsafe fn setKernelSizes_(&self, kernelSizes: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setKernelSizes : kernelSizes)
    }
    unsafe fn strides(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, strides)
    }
    unsafe fn setStrides_(&self, strides: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setStrides : strides)
    }
    unsafe fn dilationRates(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, dilationRates)
    }
    unsafe fn setDilationRates_(&self, dilationRates: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDilationRates : dilationRates)
    }
    unsafe fn paddingValues(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, paddingValues)
    }
    unsafe fn setPaddingValues_(&self, paddingValues: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPaddingValues : paddingValues)
    }
    unsafe fn paddingStyle(&self) -> MPSGraphPaddingStyle
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, paddingStyle)
    }
    unsafe fn setPaddingStyle_(&self, paddingStyle: MPSGraphPaddingStyle)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPaddingStyle : paddingStyle)
    }
    unsafe fn ceilMode(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, ceilMode)
    }
    unsafe fn setCeilMode_(&self, ceilMode: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCeilMode : ceilMode)
    }
    unsafe fn includeZeroPadToAverage(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, includeZeroPadToAverage)
    }
    unsafe fn setIncludeZeroPadToAverage_(&self, includeZeroPadToAverage: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setIncludeZeroPadToAverage : includeZeroPadToAverage)
    }
    unsafe fn returnIndicesMode(&self) -> MPSGraphPoolingReturnIndicesMode
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, returnIndicesMode)
    }
    unsafe fn setReturnIndicesMode_(&self, returnIndicesMode: MPSGraphPoolingReturnIndicesMode)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setReturnIndicesMode : returnIndicesMode)
    }
    unsafe fn returnIndicesDataType(&self) -> MPSDataType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, returnIndicesDataType)
    }
    unsafe fn setReturnIndicesDataType_(&self, returnIndicesDataType: MPSDataType)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setReturnIndicesDataType : returnIndicesDataType)
    }
    unsafe fn descriptorWithKernelSizes_strides_dilationRates_paddingValues_paddingStyle_(
        kernelSizes: NSArray,
        strides: NSArray,
        dilationRates: NSArray,
        paddingValues: NSArray,
        paddingStyle: MPSGraphPaddingStyle,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MPSGraphPooling4DOpDescriptor").unwrap(), descriptorWithKernelSizes : kernelSizes, strides : strides, dilationRates : dilationRates, paddingValues : paddingValues, paddingStyle : paddingStyle)
    }
    unsafe fn descriptorWithKernelSizes_paddingStyle_(
        kernelSizes: NSArray,
        paddingStyle: MPSGraphPaddingStyle,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MPSGraphPooling4DOpDescriptor").unwrap(), descriptorWithKernelSizes : kernelSizes, paddingStyle : paddingStyle)
    }
}
impl MPSGraph_MPSGraphPoolingOps for MPSGraph {}
pub trait MPSGraph_MPSGraphPoolingOps: Sized + std::ops::Deref {
    unsafe fn maxPooling2DWithSourceTensor_descriptor_name_(
        &self,
        source: MPSGraphTensor,
        descriptor: MPSGraphPooling2DOpDescriptor,
        name: NSString,
    ) -> MPSGraphTensor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, maxPooling2DWithSourceTensor : source, descriptor : descriptor, name : name)
    }
    unsafe fn maxPooling2DReturnIndicesWithSourceTensor_descriptor_name_(
        &self,
        source: MPSGraphTensor,
        descriptor: MPSGraphPooling2DOpDescriptor,
        name: NSString,
    ) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, maxPooling2DReturnIndicesWithSourceTensor : source, descriptor : descriptor, name : name)
    }
    unsafe fn maxPooling2DGradientWithGradientTensor_sourceTensor_descriptor_name_(
        &self,
        gradient: MPSGraphTensor,
        source: MPSGraphTensor,
        descriptor: MPSGraphPooling2DOpDescriptor,
        name: NSString,
    ) -> MPSGraphTensor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, maxPooling2DGradientWithGradientTensor : gradient, sourceTensor : source, descriptor : descriptor, name : name)
    }
    unsafe fn maxPooling2DGradientWithGradientTensor_indicesTensor_outputShape_descriptor_name_(
        &self,
        gradient: MPSGraphTensor,
        indices: MPSGraphTensor,
        outputShape: NSArray,
        descriptor: MPSGraphPooling2DOpDescriptor,
        name: NSString,
    ) -> MPSGraphTensor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, maxPooling2DGradientWithGradientTensor : gradient, indicesTensor : indices, outputShape : outputShape, descriptor : descriptor, name : name)
    }
    unsafe fn maxPooling2DGradientWithGradientTensor_indicesTensor_outputShapeTensor_descriptor_name_(
        &self,
        gradient: MPSGraphTensor,
        indices: MPSGraphTensor,
        outputShape: MPSGraphTensor,
        descriptor: MPSGraphPooling2DOpDescriptor,
        name: NSString,
    ) -> MPSGraphTensor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, maxPooling2DGradientWithGradientTensor : gradient, indicesTensor : indices, outputShapeTensor : outputShape, descriptor : descriptor, name : name)
    }
    unsafe fn avgPooling2DWithSourceTensor_descriptor_name_(
        &self,
        source: MPSGraphTensor,
        descriptor: MPSGraphPooling2DOpDescriptor,
        name: NSString,
    ) -> MPSGraphTensor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, avgPooling2DWithSourceTensor : source, descriptor : descriptor, name : name)
    }
    unsafe fn avgPooling2DGradientWithGradientTensor_sourceTensor_descriptor_name_(
        &self,
        gradient: MPSGraphTensor,
        source: MPSGraphTensor,
        descriptor: MPSGraphPooling2DOpDescriptor,
        name: NSString,
    ) -> MPSGraphTensor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, avgPooling2DGradientWithGradientTensor : gradient, sourceTensor : source, descriptor : descriptor, name : name)
    }
    unsafe fn maxPooling4DWithSourceTensor_descriptor_name_(
        &self,
        source: MPSGraphTensor,
        descriptor: MPSGraphPooling4DOpDescriptor,
        name: NSString,
    ) -> MPSGraphTensor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, maxPooling4DWithSourceTensor : source, descriptor : descriptor, name : name)
    }
    unsafe fn maxPooling4DReturnIndicesWithSourceTensor_descriptor_name_(
        &self,
        source: MPSGraphTensor,
        descriptor: MPSGraphPooling4DOpDescriptor,
        name: NSString,
    ) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, maxPooling4DReturnIndicesWithSourceTensor : source, descriptor : descriptor, name : name)
    }
    unsafe fn maxPooling4DGradientWithGradientTensor_sourceTensor_descriptor_name_(
        &self,
        gradient: MPSGraphTensor,
        source: MPSGraphTensor,
        descriptor: MPSGraphPooling4DOpDescriptor,
        name: NSString,
    ) -> MPSGraphTensor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, maxPooling4DGradientWithGradientTensor : gradient, sourceTensor : source, descriptor : descriptor, name : name)
    }
    unsafe fn maxPooling4DGradientWithGradientTensor_indicesTensor_outputShape_descriptor_name_(
        &self,
        gradient: MPSGraphTensor,
        indices: MPSGraphTensor,
        outputShape: NSArray,
        descriptor: MPSGraphPooling4DOpDescriptor,
        name: NSString,
    ) -> MPSGraphTensor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, maxPooling4DGradientWithGradientTensor : gradient, indicesTensor : indices, outputShape : outputShape, descriptor : descriptor, name : name)
    }
    unsafe fn maxPooling4DGradientWithGradientTensor_indicesTensor_outputShapeTensor_descriptor_name_(
        &self,
        gradient: MPSGraphTensor,
        indices: MPSGraphTensor,
        outputShape: MPSGraphTensor,
        descriptor: MPSGraphPooling4DOpDescriptor,
        name: NSString,
    ) -> MPSGraphTensor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, maxPooling4DGradientWithGradientTensor : gradient, indicesTensor : indices, outputShapeTensor : outputShape, descriptor : descriptor, name : name)
    }
    unsafe fn avgPooling4DWithSourceTensor_descriptor_name_(
        &self,
        source: MPSGraphTensor,
        descriptor: MPSGraphPooling4DOpDescriptor,
        name: NSString,
    ) -> MPSGraphTensor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, avgPooling4DWithSourceTensor : source, descriptor : descriptor, name : name)
    }
    unsafe fn avgPooling4DGradientWithGradientTensor_sourceTensor_descriptor_name_(
        &self,
        gradient: MPSGraphTensor,
        source: MPSGraphTensor,
        descriptor: MPSGraphPooling4DOpDescriptor,
        name: NSString,
    ) -> MPSGraphTensor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, avgPooling4DGradientWithGradientTensor : gradient, sourceTensor : source, descriptor : descriptor, name : name)
    }
    unsafe fn L2NormPooling4DWithSourceTensor_descriptor_name_(
        &self,
        source: MPSGraphTensor,
        descriptor: MPSGraphPooling4DOpDescriptor,
        name: NSString,
    ) -> MPSGraphTensor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, L2NormPooling4DWithSourceTensor : source, descriptor : descriptor, name : name)
    }
    unsafe fn L2NormPooling4DGradientWithGradientTensor_sourceTensor_descriptor_name_(
        &self,
        gradient: MPSGraphTensor,
        source: MPSGraphTensor,
        descriptor: MPSGraphPooling4DOpDescriptor,
        name: NSString,
    ) -> MPSGraphTensor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, L2NormPooling4DGradientWithGradientTensor : gradient, sourceTensor : source, descriptor : descriptor, name : name)
    }
}
impl MPSGraph_MPSGraphQuantizationOps for MPSGraph {}
pub trait MPSGraph_MPSGraphQuantizationOps: Sized + std::ops::Deref {
    unsafe fn quantizeTensor_scale_zeroPoint_dataType_name_(
        &self,
        tensor: MPSGraphTensor,
        scale: f64,
        zeroPoint: f64,
        dataType: MPSDataType,
        name: NSString,
    ) -> MPSGraphTensor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, quantizeTensor : tensor, scale : scale, zeroPoint : zeroPoint, dataType : dataType, name : name)
    }
    unsafe fn dequantizeTensor_scale_zeroPoint_dataType_name_(
        &self,
        tensor: MPSGraphTensor,
        scale: f64,
        zeroPoint: f64,
        dataType: MPSDataType,
        name: NSString,
    ) -> MPSGraphTensor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, dequantizeTensor : tensor, scale : scale, zeroPoint : zeroPoint, dataType : dataType, name : name)
    }
    unsafe fn quantizeTensor_scaleTensor_zeroPoint_dataType_axis_name_(
        &self,
        tensor: MPSGraphTensor,
        scaleTensor: MPSGraphTensor,
        zeroPoint: f64,
        dataType: MPSDataType,
        axis: NSInteger,
        name: NSString,
    ) -> MPSGraphTensor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, quantizeTensor : tensor, scaleTensor : scaleTensor, zeroPoint : zeroPoint, dataType : dataType, axis : axis, name : name)
    }
    unsafe fn dequantizeTensor_scaleTensor_zeroPoint_dataType_axis_name_(
        &self,
        tensor: MPSGraphTensor,
        scaleTensor: MPSGraphTensor,
        zeroPoint: f64,
        dataType: MPSDataType,
        axis: NSInteger,
        name: NSString,
    ) -> MPSGraphTensor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, dequantizeTensor : tensor, scaleTensor : scaleTensor, zeroPoint : zeroPoint, dataType : dataType, axis : axis, name : name)
    }
    unsafe fn quantizeTensor_scaleTensor_zeroPointTensor_dataType_axis_name_(
        &self,
        tensor: MPSGraphTensor,
        scaleTensor: MPSGraphTensor,
        zeroPointTensor: MPSGraphTensor,
        dataType: MPSDataType,
        axis: NSInteger,
        name: NSString,
    ) -> MPSGraphTensor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, quantizeTensor : tensor, scaleTensor : scaleTensor, zeroPointTensor : zeroPointTensor, dataType : dataType, axis : axis, name : name)
    }
    unsafe fn dequantizeTensor_scaleTensor_zeroPointTensor_dataType_axis_name_(
        &self,
        tensor: MPSGraphTensor,
        scaleTensor: MPSGraphTensor,
        zeroPointTensor: MPSGraphTensor,
        dataType: MPSDataType,
        axis: NSInteger,
        name: NSString,
    ) -> MPSGraphTensor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, dequantizeTensor : tensor, scaleTensor : scaleTensor, zeroPointTensor : zeroPointTensor, dataType : dataType, axis : axis, name : name)
    }
    unsafe fn dequantizeTensor_scaleTensor_zeroPointTensor_dataType_name_(
        &self,
        tensor: MPSGraphTensor,
        scaleTensor: MPSGraphTensor,
        zeroPointTensor: MPSGraphTensor,
        dataType: MPSDataType,
        name: NSString,
    ) -> MPSGraphTensor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, dequantizeTensor : tensor, scaleTensor : scaleTensor, zeroPointTensor : zeroPointTensor, dataType : dataType, name : name)
    }
    unsafe fn dequantizeTensor_scaleTensor_dataType_name_(
        &self,
        tensor: MPSGraphTensor,
        scaleTensor: MPSGraphTensor,
        dataType: MPSDataType,
        name: NSString,
    ) -> MPSGraphTensor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, dequantizeTensor : tensor, scaleTensor : scaleTensor, dataType : dataType, name : name)
    }
    unsafe fn dequantizeTensor_LUTTensor_name_(
        &self,
        tensor: MPSGraphTensor,
        LUTTensor: MPSGraphTensor,
        name: NSString,
    ) -> MPSGraphTensor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, dequantizeTensor : tensor, LUTTensor : LUTTensor, name : name)
    }
    unsafe fn dequantizeTensor_LUTTensor_axis_name_(
        &self,
        tensor: MPSGraphTensor,
        LUTTensor: MPSGraphTensor,
        axis: NSInteger,
        name: NSString,
    ) -> MPSGraphTensor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, dequantizeTensor : tensor, LUTTensor : LUTTensor, axis : axis, name : name)
    }
}
pub type MPSGraphRandomDistribution = u64;
pub type MPSGraphRandomNormalSamplingMethod = u64;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MPSGraphRandomOpDescriptor(pub id);
impl std::ops::Deref for MPSGraphRandomOpDescriptor {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MPSGraphRandomOpDescriptor {}
impl MPSGraphRandomOpDescriptor {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MPSGraphRandomOpDescriptor").unwrap(), alloc) })
    }
}
impl PNSCopying for MPSGraphRandomOpDescriptor {}
impl IMPSGraphObject for MPSGraphRandomOpDescriptor {}
impl std::convert::TryFrom<MPSGraphObject> for MPSGraphRandomOpDescriptor {
    type Error = &'static str;
    fn try_from(parent: MPSGraphObject) -> Result<MPSGraphRandomOpDescriptor, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MPSGraphRandomOpDescriptor").unwrap()) };
        if is_kind_of {
            Ok(MPSGraphRandomOpDescriptor(parent.0))
        } else {
            Err("This MPSGraphObject cannot be downcasted to MPSGraphRandomOpDescriptor")
        }
    }
}
impl INSObject for MPSGraphRandomOpDescriptor {}
impl PNSObject for MPSGraphRandomOpDescriptor {}
impl IMPSGraphRandomOpDescriptor for MPSGraphRandomOpDescriptor {}
pub trait IMPSGraphRandomOpDescriptor: Sized + std::ops::Deref {
    unsafe fn distribution(&self) -> MPSGraphRandomDistribution
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, distribution)
    }
    unsafe fn setDistribution_(&self, distribution: MPSGraphRandomDistribution)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDistribution : distribution)
    }
    unsafe fn dataType(&self) -> MPSDataType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, dataType)
    }
    unsafe fn setDataType_(&self, dataType: MPSDataType)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDataType : dataType)
    }
    unsafe fn min(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, min)
    }
    unsafe fn setMin_(&self, min: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMin : min)
    }
    unsafe fn max(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, max)
    }
    unsafe fn setMax_(&self, max: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMax : max)
    }
    unsafe fn minInteger(&self) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, minInteger)
    }
    unsafe fn setMinInteger_(&self, minInteger: NSInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMinInteger : minInteger)
    }
    unsafe fn maxInteger(&self) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, maxInteger)
    }
    unsafe fn setMaxInteger_(&self, maxInteger: NSInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMaxInteger : maxInteger)
    }
    unsafe fn mean(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, mean)
    }
    unsafe fn setMean_(&self, mean: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMean : mean)
    }
    unsafe fn standardDeviation(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, standardDeviation)
    }
    unsafe fn setStandardDeviation_(&self, standardDeviation: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setStandardDeviation : standardDeviation)
    }
    unsafe fn samplingMethod(&self) -> MPSGraphRandomNormalSamplingMethod
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, samplingMethod)
    }
    unsafe fn setSamplingMethod_(&self, samplingMethod: MPSGraphRandomNormalSamplingMethod)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSamplingMethod : samplingMethod)
    }
    unsafe fn descriptorWithDistribution_dataType_(
        distribution: MPSGraphRandomDistribution,
        dataType: MPSDataType,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MPSGraphRandomOpDescriptor").unwrap(), descriptorWithDistribution : distribution, dataType : dataType)
    }
}
impl MPSGraph_MPSGraphRandomOps for MPSGraph {}
pub trait MPSGraph_MPSGraphRandomOps: Sized + std::ops::Deref {
    unsafe fn randomPhiloxStateTensorWithSeed_name_(
        &self,
        seed: NSUInteger,
        name: NSString,
    ) -> MPSGraphTensor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, randomPhiloxStateTensorWithSeed : seed, name : name)
    }
    unsafe fn randomPhiloxStateTensorWithCounterLow_counterHigh_key_name_(
        &self,
        counterLow: NSUInteger,
        counterHigh: NSUInteger,
        key: NSUInteger,
        name: NSString,
    ) -> MPSGraphTensor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, randomPhiloxStateTensorWithCounterLow : counterLow, counterHigh : counterHigh, key : key, name : name)
    }
    unsafe fn randomTensorWithShape_descriptor_name_(
        &self,
        shape: NSArray,
        descriptor: MPSGraphRandomOpDescriptor,
        name: NSString,
    ) -> MPSGraphTensor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, randomTensorWithShape : shape, descriptor : descriptor, name : name)
    }
    unsafe fn randomTensorWithShapeTensor_descriptor_name_(
        &self,
        shapeTensor: MPSGraphTensor,
        descriptor: MPSGraphRandomOpDescriptor,
        name: NSString,
    ) -> MPSGraphTensor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, randomTensorWithShapeTensor : shapeTensor, descriptor : descriptor, name : name)
    }
    unsafe fn randomTensorWithShape_descriptor_seed_name_(
        &self,
        shape: NSArray,
        descriptor: MPSGraphRandomOpDescriptor,
        seed: NSUInteger,
        name: NSString,
    ) -> MPSGraphTensor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, randomTensorWithShape : shape, descriptor : descriptor, seed : seed, name : name)
    }
    unsafe fn randomTensorWithShapeTensor_descriptor_seed_name_(
        &self,
        shapeTensor: MPSGraphTensor,
        descriptor: MPSGraphRandomOpDescriptor,
        seed: NSUInteger,
        name: NSString,
    ) -> MPSGraphTensor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, randomTensorWithShapeTensor : shapeTensor, descriptor : descriptor, seed : seed, name : name)
    }
    unsafe fn randomTensorWithShape_descriptor_stateTensor_name_(
        &self,
        shape: NSArray,
        descriptor: MPSGraphRandomOpDescriptor,
        state: MPSGraphTensor,
        name: NSString,
    ) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, randomTensorWithShape : shape, descriptor : descriptor, stateTensor : state, name : name)
    }
    unsafe fn randomTensorWithShapeTensor_descriptor_stateTensor_name_(
        &self,
        shapeTensor: MPSGraphTensor,
        descriptor: MPSGraphRandomOpDescriptor,
        state: MPSGraphTensor,
        name: NSString,
    ) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, randomTensorWithShapeTensor : shapeTensor, descriptor : descriptor, stateTensor : state, name : name)
    }
    unsafe fn randomUniformTensorWithShape_name_(
        &self,
        shape: NSArray,
        name: NSString,
    ) -> MPSGraphTensor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, randomUniformTensorWithShape : shape, name : name)
    }
    unsafe fn randomUniformTensorWithShapeTensor_name_(
        &self,
        shapeTensor: MPSGraphTensor,
        name: NSString,
    ) -> MPSGraphTensor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, randomUniformTensorWithShapeTensor : shapeTensor, name : name)
    }
    unsafe fn randomUniformTensorWithShape_seed_name_(
        &self,
        shape: NSArray,
        seed: NSUInteger,
        name: NSString,
    ) -> MPSGraphTensor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, randomUniformTensorWithShape : shape, seed : seed, name : name)
    }
    unsafe fn randomUniformTensorWithShapeTensor_seed_name_(
        &self,
        shapeTensor: MPSGraphTensor,
        seed: NSUInteger,
        name: NSString,
    ) -> MPSGraphTensor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, randomUniformTensorWithShapeTensor : shapeTensor, seed : seed, name : name)
    }
    unsafe fn randomUniformTensorWithShape_stateTensor_name_(
        &self,
        shape: NSArray,
        state: MPSGraphTensor,
        name: NSString,
    ) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, randomUniformTensorWithShape : shape, stateTensor : state, name : name)
    }
    unsafe fn randomUniformTensorWithShapeTensor_stateTensor_name_(
        &self,
        shapeTensor: MPSGraphTensor,
        state: MPSGraphTensor,
        name: NSString,
    ) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, randomUniformTensorWithShapeTensor : shapeTensor, stateTensor : state, name : name)
    }
    unsafe fn dropoutTensor_rate_name_(
        &self,
        tensor: MPSGraphTensor,
        rate: f64,
        name: NSString,
    ) -> MPSGraphTensor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, dropoutTensor : tensor, rate : rate, name : name)
    }
    unsafe fn dropoutTensor_rateTensor_name_(
        &self,
        tensor: MPSGraphTensor,
        rate: MPSGraphTensor,
        name: NSString,
    ) -> MPSGraphTensor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, dropoutTensor : tensor, rateTensor : rate, name : name)
    }
}
impl MPSGraph_MPSGraphReductionOps for MPSGraph {}
pub trait MPSGraph_MPSGraphReductionOps: Sized + std::ops::Deref {
    unsafe fn reductionSumWithTensor_axis_name_(
        &self,
        tensor: MPSGraphTensor,
        axis: NSInteger,
        name: NSString,
    ) -> MPSGraphTensor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, reductionSumWithTensor : tensor, axis : axis, name : name)
    }
    unsafe fn reductionSumWithTensor_axes_name_(
        &self,
        tensor: MPSGraphTensor,
        axes: NSArray,
        name: NSString,
    ) -> MPSGraphTensor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, reductionSumWithTensor : tensor, axes : axes, name : name)
    }
    unsafe fn reductionMaximumWithTensor_axis_name_(
        &self,
        tensor: MPSGraphTensor,
        axis: NSInteger,
        name: NSString,
    ) -> MPSGraphTensor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, reductionMaximumWithTensor : tensor, axis : axis, name : name)
    }
    unsafe fn reductionMaximumWithTensor_axes_name_(
        &self,
        tensor: MPSGraphTensor,
        axes: NSArray,
        name: NSString,
    ) -> MPSGraphTensor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, reductionMaximumWithTensor : tensor, axes : axes, name : name)
    }
    unsafe fn reductionMinimumWithTensor_axis_name_(
        &self,
        tensor: MPSGraphTensor,
        axis: NSInteger,
        name: NSString,
    ) -> MPSGraphTensor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, reductionMinimumWithTensor : tensor, axis : axis, name : name)
    }
    unsafe fn reductionMinimumWithTensor_axes_name_(
        &self,
        tensor: MPSGraphTensor,
        axes: NSArray,
        name: NSString,
    ) -> MPSGraphTensor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, reductionMinimumWithTensor : tensor, axes : axes, name : name)
    }
    unsafe fn reductionMaximumPropagateNaNWithTensor_axis_name_(
        &self,
        tensor: MPSGraphTensor,
        axis: NSInteger,
        name: NSString,
    ) -> MPSGraphTensor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, reductionMaximumPropagateNaNWithTensor : tensor, axis : axis, name : name)
    }
    unsafe fn reductionMaximumPropagateNaNWithTensor_axes_name_(
        &self,
        tensor: MPSGraphTensor,
        axes: NSArray,
        name: NSString,
    ) -> MPSGraphTensor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, reductionMaximumPropagateNaNWithTensor : tensor, axes : axes, name : name)
    }
    unsafe fn reductionMinimumPropagateNaNWithTensor_axis_name_(
        &self,
        tensor: MPSGraphTensor,
        axis: NSInteger,
        name: NSString,
    ) -> MPSGraphTensor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, reductionMinimumPropagateNaNWithTensor : tensor, axis : axis, name : name)
    }
    unsafe fn reductionMinimumPropagateNaNWithTensor_axes_name_(
        &self,
        tensor: MPSGraphTensor,
        axes: NSArray,
        name: NSString,
    ) -> MPSGraphTensor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, reductionMinimumPropagateNaNWithTensor : tensor, axes : axes, name : name)
    }
    unsafe fn reductionProductWithTensor_axis_name_(
        &self,
        tensor: MPSGraphTensor,
        axis: NSInteger,
        name: NSString,
    ) -> MPSGraphTensor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, reductionProductWithTensor : tensor, axis : axis, name : name)
    }
    unsafe fn reductionProductWithTensor_axes_name_(
        &self,
        tensor: MPSGraphTensor,
        axes: NSArray,
        name: NSString,
    ) -> MPSGraphTensor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, reductionProductWithTensor : tensor, axes : axes, name : name)
    }
    unsafe fn reductionArgMaximumWithTensor_axis_name_(
        &self,
        tensor: MPSGraphTensor,
        axis: NSInteger,
        name: NSString,
    ) -> MPSGraphTensor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, reductionArgMaximumWithTensor : tensor, axis : axis, name : name)
    }
    unsafe fn reductionArgMinimumWithTensor_axis_name_(
        &self,
        tensor: MPSGraphTensor,
        axis: NSInteger,
        name: NSString,
    ) -> MPSGraphTensor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, reductionArgMinimumWithTensor : tensor, axis : axis, name : name)
    }
    unsafe fn reductionAndWithTensor_axis_name_(
        &self,
        tensor: MPSGraphTensor,
        axis: NSInteger,
        name: NSString,
    ) -> MPSGraphTensor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, reductionAndWithTensor : tensor, axis : axis, name : name)
    }
    unsafe fn reductionAndWithTensor_axes_name_(
        &self,
        tensor: MPSGraphTensor,
        axes: NSArray,
        name: NSString,
    ) -> MPSGraphTensor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, reductionAndWithTensor : tensor, axes : axes, name : name)
    }
    unsafe fn reductionOrWithTensor_axis_name_(
        &self,
        tensor: MPSGraphTensor,
        axis: NSInteger,
        name: NSString,
    ) -> MPSGraphTensor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, reductionOrWithTensor : tensor, axis : axis, name : name)
    }
    unsafe fn reductionOrWithTensor_axes_name_(
        &self,
        tensor: MPSGraphTensor,
        axes: NSArray,
        name: NSString,
    ) -> MPSGraphTensor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, reductionOrWithTensor : tensor, axes : axes, name : name)
    }
}
pub type MPSGraphResizeMode = NSUInteger;
pub type MPSGraphResizeNearestRoundingMode = NSUInteger;
impl MPSGraph_MPSGraphResizeOps for MPSGraph {}
pub trait MPSGraph_MPSGraphResizeOps: Sized + std::ops::Deref {
    unsafe fn resizeTensor_size_mode_centerResult_alignCorners_layout_name_(
        &self,
        imagesTensor: MPSGraphTensor,
        size: NSArray,
        mode: MPSGraphResizeMode,
        centerResult: BOOL,
        alignCorners: BOOL,
        layout: MPSGraphTensorNamedDataLayout,
        name: NSString,
    ) -> MPSGraphTensor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, resizeTensor : imagesTensor, size : size, mode : mode, centerResult : centerResult, alignCorners : alignCorners, layout : layout, name : name)
    }
    unsafe fn resizeTensor_sizeTensor_mode_centerResult_alignCorners_layout_name_(
        &self,
        imagesTensor: MPSGraphTensor,
        size: MPSGraphTensor,
        mode: MPSGraphResizeMode,
        centerResult: BOOL,
        alignCorners: BOOL,
        layout: MPSGraphTensorNamedDataLayout,
        name: NSString,
    ) -> MPSGraphTensor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, resizeTensor : imagesTensor, sizeTensor : size, mode : mode, centerResult : centerResult, alignCorners : alignCorners, layout : layout, name : name)
    }
    unsafe fn resizeTensor_sizeTensor_mode_centerResult_alignCorners_name_(
        &self,
        imagesTensor: MPSGraphTensor,
        size: MPSGraphTensor,
        mode: MPSGraphResizeMode,
        centerResult: BOOL,
        alignCorners: BOOL,
        name: NSString,
    ) -> MPSGraphTensor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, resizeTensor : imagesTensor, sizeTensor : size, mode : mode, centerResult : centerResult, alignCorners : alignCorners, name : name)
    }
    unsafe fn resizeNearestWithTensor_sizeTensor_nearestRoundingMode_centerResult_alignCorners_layout_name_(
        &self,
        imagesTensor: MPSGraphTensor,
        size: MPSGraphTensor,
        nearestRoundingMode: MPSGraphResizeNearestRoundingMode,
        centerResult: BOOL,
        alignCorners: BOOL,
        layout: MPSGraphTensorNamedDataLayout,
        name: NSString,
    ) -> MPSGraphTensor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, resizeNearestWithTensor : imagesTensor, sizeTensor : size, nearestRoundingMode : nearestRoundingMode, centerResult : centerResult, alignCorners : alignCorners, layout : layout, name : name)
    }
    unsafe fn resizeNearestWithTensor_sizeTensor_nearestRoundingMode_centerResult_alignCorners_name_(
        &self,
        imagesTensor: MPSGraphTensor,
        size: MPSGraphTensor,
        nearestRoundingMode: MPSGraphResizeNearestRoundingMode,
        centerResult: BOOL,
        alignCorners: BOOL,
        name: NSString,
    ) -> MPSGraphTensor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, resizeNearestWithTensor : imagesTensor, sizeTensor : size, nearestRoundingMode : nearestRoundingMode, centerResult : centerResult, alignCorners : alignCorners, name : name)
    }
    unsafe fn resizeBilinearWithTensor_sizeTensor_centerResult_alignCorners_layout_name_(
        &self,
        imagesTensor: MPSGraphTensor,
        size: MPSGraphTensor,
        centerResult: BOOL,
        alignCorners: BOOL,
        layout: MPSGraphTensorNamedDataLayout,
        name: NSString,
    ) -> MPSGraphTensor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, resizeBilinearWithTensor : imagesTensor, sizeTensor : size, centerResult : centerResult, alignCorners : alignCorners, layout : layout, name : name)
    }
    unsafe fn resizeBilinearWithTensor_sizeTensor_centerResult_alignCorners_name_(
        &self,
        imagesTensor: MPSGraphTensor,
        size: MPSGraphTensor,
        centerResult: BOOL,
        alignCorners: BOOL,
        name: NSString,
    ) -> MPSGraphTensor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, resizeBilinearWithTensor : imagesTensor, sizeTensor : size, centerResult : centerResult, alignCorners : alignCorners, name : name)
    }
    unsafe fn resizeTensor_sizeTensor_scaleOffsetTensor_mode_layout_name_(
        &self,
        imagesTensor: MPSGraphTensor,
        size: MPSGraphTensor,
        scaleOffset: MPSGraphTensor,
        mode: MPSGraphResizeMode,
        layout: MPSGraphTensorNamedDataLayout,
        name: NSString,
    ) -> MPSGraphTensor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, resizeTensor : imagesTensor, sizeTensor : size, scaleOffsetTensor : scaleOffset, mode : mode, layout : layout, name : name)
    }
    unsafe fn resizeTensor_sizeTensor_scaleTensor_offsetTensor_mode_name_(
        &self,
        imagesTensor: MPSGraphTensor,
        size: MPSGraphTensor,
        scale: MPSGraphTensor,
        offset: MPSGraphTensor,
        mode: MPSGraphResizeMode,
        name: NSString,
    ) -> MPSGraphTensor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, resizeTensor : imagesTensor, sizeTensor : size, scaleTensor : scale, offsetTensor : offset, mode : mode, name : name)
    }
    unsafe fn resizeNearestWithTensor_sizeTensor_scaleOffsetTensor_nearestRoundingMode_layout_name_(
        &self,
        imagesTensor: MPSGraphTensor,
        size: MPSGraphTensor,
        scaleOffset: MPSGraphTensor,
        nearestRoundingMode: MPSGraphResizeNearestRoundingMode,
        layout: MPSGraphTensorNamedDataLayout,
        name: NSString,
    ) -> MPSGraphTensor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, resizeNearestWithTensor : imagesTensor, sizeTensor : size, scaleOffsetTensor : scaleOffset, nearestRoundingMode : nearestRoundingMode, layout : layout, name : name)
    }
    unsafe fn resizeNearestWithTensor_sizeTensor_scaleTensor_offsetTensor_nearestRoundingMode_name_(
        &self,
        imagesTensor: MPSGraphTensor,
        size: MPSGraphTensor,
        scale: MPSGraphTensor,
        offset: MPSGraphTensor,
        nearestRoundingMode: MPSGraphResizeNearestRoundingMode,
        name: NSString,
    ) -> MPSGraphTensor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, resizeNearestWithTensor : imagesTensor, sizeTensor : size, scaleTensor : scale, offsetTensor : offset, nearestRoundingMode : nearestRoundingMode, name : name)
    }
    unsafe fn resizeBilinearWithTensor_sizeTensor_scaleOffsetTensor_layout_name_(
        &self,
        imagesTensor: MPSGraphTensor,
        size: MPSGraphTensor,
        scaleOffset: MPSGraphTensor,
        layout: MPSGraphTensorNamedDataLayout,
        name: NSString,
    ) -> MPSGraphTensor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, resizeBilinearWithTensor : imagesTensor, sizeTensor : size, scaleOffsetTensor : scaleOffset, layout : layout, name : name)
    }
    unsafe fn resizeBilinearWithTensor_sizeTensor_scaleTensor_offsetTensor_name_(
        &self,
        imagesTensor: MPSGraphTensor,
        size: MPSGraphTensor,
        scale: MPSGraphTensor,
        offset: MPSGraphTensor,
        name: NSString,
    ) -> MPSGraphTensor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, resizeBilinearWithTensor : imagesTensor, sizeTensor : size, scaleTensor : scale, offsetTensor : offset, name : name)
    }
    unsafe fn resizeWithGradientTensor_input_mode_centerResult_alignCorners_layout_name_(
        &self,
        gradient: MPSGraphTensor,
        input: MPSGraphTensor,
        mode: MPSGraphResizeMode,
        centerResult: BOOL,
        alignCorners: BOOL,
        layout: MPSGraphTensorNamedDataLayout,
        name: NSString,
    ) -> MPSGraphTensor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, resizeWithGradientTensor : gradient, input : input, mode : mode, centerResult : centerResult, alignCorners : alignCorners, layout : layout, name : name)
    }
    unsafe fn resizeNearestWithGradientTensor_input_nearestRoundingMode_centerResult_alignCorners_layout_name_(
        &self,
        gradient: MPSGraphTensor,
        input: MPSGraphTensor,
        nearestRoundingMode: MPSGraphResizeNearestRoundingMode,
        centerResult: BOOL,
        alignCorners: BOOL,
        layout: MPSGraphTensorNamedDataLayout,
        name: NSString,
    ) -> MPSGraphTensor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, resizeNearestWithGradientTensor : gradient, input : input, nearestRoundingMode : nearestRoundingMode, centerResult : centerResult, alignCorners : alignCorners, layout : layout, name : name)
    }
    unsafe fn resizeBilinearWithGradientTensor_input_centerResult_alignCorners_layout_name_(
        &self,
        gradient: MPSGraphTensor,
        input: MPSGraphTensor,
        centerResult: BOOL,
        alignCorners: BOOL,
        layout: MPSGraphTensorNamedDataLayout,
        name: NSString,
    ) -> MPSGraphTensor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, resizeBilinearWithGradientTensor : gradient, input : input, centerResult : centerResult, alignCorners : alignCorners, layout : layout, name : name)
    }
    unsafe fn resizeWithGradientTensor_input_scaleOffsetTensor_mode_layout_name_(
        &self,
        gradient: MPSGraphTensor,
        input: MPSGraphTensor,
        scaleOffset: MPSGraphTensor,
        mode: MPSGraphResizeMode,
        layout: MPSGraphTensorNamedDataLayout,
        name: NSString,
    ) -> MPSGraphTensor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, resizeWithGradientTensor : gradient, input : input, scaleOffsetTensor : scaleOffset, mode : mode, layout : layout, name : name)
    }
    unsafe fn resizeWithGradientTensor_input_scaleTensor_offsetTensor_mode_name_(
        &self,
        gradient: MPSGraphTensor,
        input: MPSGraphTensor,
        scale: MPSGraphTensor,
        offset: MPSGraphTensor,
        mode: MPSGraphResizeMode,
        name: NSString,
    ) -> MPSGraphTensor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, resizeWithGradientTensor : gradient, input : input, scaleTensor : scale, offsetTensor : offset, mode : mode, name : name)
    }
    unsafe fn resizeNearestWithGradientTensor_input_scaleOffsetTensor_nearestRoundingMode_layout_name_(
        &self,
        gradient: MPSGraphTensor,
        input: MPSGraphTensor,
        scaleOffset: MPSGraphTensor,
        nearestRoundingMode: MPSGraphResizeNearestRoundingMode,
        layout: MPSGraphTensorNamedDataLayout,
        name: NSString,
    ) -> MPSGraphTensor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, resizeNearestWithGradientTensor : gradient, input : input, scaleOffsetTensor : scaleOffset, nearestRoundingMode : nearestRoundingMode, layout : layout, name : name)
    }
    unsafe fn resizeNearestWithGradientTensor_input_scaleTensor_offsetTensor_nearestRoundingMode_name_(
        &self,
        gradient: MPSGraphTensor,
        input: MPSGraphTensor,
        scale: MPSGraphTensor,
        offset: MPSGraphTensor,
        nearestRoundingMode: MPSGraphResizeNearestRoundingMode,
        name: NSString,
    ) -> MPSGraphTensor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, resizeNearestWithGradientTensor : gradient, input : input, scaleTensor : scale, offsetTensor : offset, nearestRoundingMode : nearestRoundingMode, name : name)
    }
    unsafe fn resizeBilinearWithGradientTensor_input_scaleOffsetTensor_layout_name_(
        &self,
        gradient: MPSGraphTensor,
        input: MPSGraphTensor,
        scaleOffset: MPSGraphTensor,
        layout: MPSGraphTensorNamedDataLayout,
        name: NSString,
    ) -> MPSGraphTensor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, resizeBilinearWithGradientTensor : gradient, input : input, scaleOffsetTensor : scaleOffset, layout : layout, name : name)
    }
    unsafe fn resizeBilinearWithGradientTensor_input_scaleTensor_offsetTensor_name_(
        &self,
        gradient: MPSGraphTensor,
        input: MPSGraphTensor,
        scale: MPSGraphTensor,
        offset: MPSGraphTensor,
        name: NSString,
    ) -> MPSGraphTensor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, resizeBilinearWithGradientTensor : gradient, input : input, scaleTensor : scale, offsetTensor : offset, name : name)
    }
}
pub type MPSGraphRNNActivation = NSUInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MPSGraphSingleGateRNNDescriptor(pub id);
impl std::ops::Deref for MPSGraphSingleGateRNNDescriptor {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MPSGraphSingleGateRNNDescriptor {}
impl MPSGraphSingleGateRNNDescriptor {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MPSGraphSingleGateRNNDescriptor").unwrap(), alloc) })
    }
}
impl PNSCopying for MPSGraphSingleGateRNNDescriptor {}
impl IMPSGraphObject for MPSGraphSingleGateRNNDescriptor {}
impl std::convert::TryFrom<MPSGraphObject> for MPSGraphSingleGateRNNDescriptor {
    type Error = &'static str;
    fn try_from(parent: MPSGraphObject) -> Result<MPSGraphSingleGateRNNDescriptor, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MPSGraphSingleGateRNNDescriptor").unwrap())
        };
        if is_kind_of {
            Ok(MPSGraphSingleGateRNNDescriptor(parent.0))
        } else {
            Err("This MPSGraphObject cannot be downcasted to MPSGraphSingleGateRNNDescriptor")
        }
    }
}
impl INSObject for MPSGraphSingleGateRNNDescriptor {}
impl PNSObject for MPSGraphSingleGateRNNDescriptor {}
impl IMPSGraphSingleGateRNNDescriptor for MPSGraphSingleGateRNNDescriptor {}
pub trait IMPSGraphSingleGateRNNDescriptor: Sized + std::ops::Deref {
    unsafe fn reverse(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, reverse)
    }
    unsafe fn setReverse_(&self, reverse: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setReverse : reverse)
    }
    unsafe fn bidirectional(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, bidirectional)
    }
    unsafe fn setBidirectional_(&self, bidirectional: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setBidirectional : bidirectional)
    }
    unsafe fn training(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, training)
    }
    unsafe fn setTraining_(&self, training: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setTraining : training)
    }
    unsafe fn activation(&self) -> MPSGraphRNNActivation
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, activation)
    }
    unsafe fn setActivation_(&self, activation: MPSGraphRNNActivation)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setActivation : activation)
    }
    unsafe fn descriptor() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MPSGraphSingleGateRNNDescriptor").unwrap(), descriptor)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MPSGraphLSTMDescriptor(pub id);
impl std::ops::Deref for MPSGraphLSTMDescriptor {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MPSGraphLSTMDescriptor {}
impl MPSGraphLSTMDescriptor {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MPSGraphLSTMDescriptor").unwrap(), alloc) })
    }
}
impl PNSCopying for MPSGraphLSTMDescriptor {}
impl IMPSGraphObject for MPSGraphLSTMDescriptor {}
impl std::convert::TryFrom<MPSGraphObject> for MPSGraphLSTMDescriptor {
    type Error = &'static str;
    fn try_from(parent: MPSGraphObject) -> Result<MPSGraphLSTMDescriptor, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MPSGraphLSTMDescriptor").unwrap()) };
        if is_kind_of {
            Ok(MPSGraphLSTMDescriptor(parent.0))
        } else {
            Err("This MPSGraphObject cannot be downcasted to MPSGraphLSTMDescriptor")
        }
    }
}
impl INSObject for MPSGraphLSTMDescriptor {}
impl PNSObject for MPSGraphLSTMDescriptor {}
impl IMPSGraphLSTMDescriptor for MPSGraphLSTMDescriptor {}
pub trait IMPSGraphLSTMDescriptor: Sized + std::ops::Deref {
    unsafe fn reverse(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, reverse)
    }
    unsafe fn setReverse_(&self, reverse: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setReverse : reverse)
    }
    unsafe fn bidirectional(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, bidirectional)
    }
    unsafe fn setBidirectional_(&self, bidirectional: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setBidirectional : bidirectional)
    }
    unsafe fn produceCell(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, produceCell)
    }
    unsafe fn setProduceCell_(&self, produceCell: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setProduceCell : produceCell)
    }
    unsafe fn training(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, training)
    }
    unsafe fn setTraining_(&self, training: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setTraining : training)
    }
    unsafe fn forgetGateLast(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, forgetGateLast)
    }
    unsafe fn setForgetGateLast_(&self, forgetGateLast: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setForgetGateLast : forgetGateLast)
    }
    unsafe fn inputGateActivation(&self) -> MPSGraphRNNActivation
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inputGateActivation)
    }
    unsafe fn setInputGateActivation_(&self, inputGateActivation: MPSGraphRNNActivation)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInputGateActivation : inputGateActivation)
    }
    unsafe fn forgetGateActivation(&self) -> MPSGraphRNNActivation
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, forgetGateActivation)
    }
    unsafe fn setForgetGateActivation_(&self, forgetGateActivation: MPSGraphRNNActivation)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setForgetGateActivation : forgetGateActivation)
    }
    unsafe fn cellGateActivation(&self) -> MPSGraphRNNActivation
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, cellGateActivation)
    }
    unsafe fn setCellGateActivation_(&self, cellGateActivation: MPSGraphRNNActivation)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCellGateActivation : cellGateActivation)
    }
    unsafe fn outputGateActivation(&self) -> MPSGraphRNNActivation
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, outputGateActivation)
    }
    unsafe fn setOutputGateActivation_(&self, outputGateActivation: MPSGraphRNNActivation)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setOutputGateActivation : outputGateActivation)
    }
    unsafe fn activation(&self) -> MPSGraphRNNActivation
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, activation)
    }
    unsafe fn setActivation_(&self, activation: MPSGraphRNNActivation)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setActivation : activation)
    }
    unsafe fn descriptor() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MPSGraphLSTMDescriptor").unwrap(), descriptor)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MPSGraphGRUDescriptor(pub id);
impl std::ops::Deref for MPSGraphGRUDescriptor {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MPSGraphGRUDescriptor {}
impl MPSGraphGRUDescriptor {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MPSGraphGRUDescriptor").unwrap(), alloc) })
    }
}
impl PNSCopying for MPSGraphGRUDescriptor {}
impl IMPSGraphObject for MPSGraphGRUDescriptor {}
impl std::convert::TryFrom<MPSGraphObject> for MPSGraphGRUDescriptor {
    type Error = &'static str;
    fn try_from(parent: MPSGraphObject) -> Result<MPSGraphGRUDescriptor, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MPSGraphGRUDescriptor").unwrap()) };
        if is_kind_of {
            Ok(MPSGraphGRUDescriptor(parent.0))
        } else {
            Err("This MPSGraphObject cannot be downcasted to MPSGraphGRUDescriptor")
        }
    }
}
impl INSObject for MPSGraphGRUDescriptor {}
impl PNSObject for MPSGraphGRUDescriptor {}
impl IMPSGraphGRUDescriptor for MPSGraphGRUDescriptor {}
pub trait IMPSGraphGRUDescriptor: Sized + std::ops::Deref {
    unsafe fn reverse(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, reverse)
    }
    unsafe fn setReverse_(&self, reverse: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setReverse : reverse)
    }
    unsafe fn bidirectional(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, bidirectional)
    }
    unsafe fn setBidirectional_(&self, bidirectional: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setBidirectional : bidirectional)
    }
    unsafe fn training(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, training)
    }
    unsafe fn setTraining_(&self, training: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setTraining : training)
    }
    unsafe fn resetGateFirst(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, resetGateFirst)
    }
    unsafe fn setResetGateFirst_(&self, resetGateFirst: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setResetGateFirst : resetGateFirst)
    }
    unsafe fn resetAfter(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, resetAfter)
    }
    unsafe fn setResetAfter_(&self, resetAfter: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setResetAfter : resetAfter)
    }
    unsafe fn flipZ(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, flipZ)
    }
    unsafe fn setFlipZ_(&self, flipZ: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setFlipZ : flipZ)
    }
    unsafe fn updateGateActivation(&self) -> MPSGraphRNNActivation
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, updateGateActivation)
    }
    unsafe fn setUpdateGateActivation_(&self, updateGateActivation: MPSGraphRNNActivation)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setUpdateGateActivation : updateGateActivation)
    }
    unsafe fn resetGateActivation(&self) -> MPSGraphRNNActivation
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, resetGateActivation)
    }
    unsafe fn setResetGateActivation_(&self, resetGateActivation: MPSGraphRNNActivation)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setResetGateActivation : resetGateActivation)
    }
    unsafe fn outputGateActivation(&self) -> MPSGraphRNNActivation
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, outputGateActivation)
    }
    unsafe fn setOutputGateActivation_(&self, outputGateActivation: MPSGraphRNNActivation)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setOutputGateActivation : outputGateActivation)
    }
    unsafe fn descriptor() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MPSGraphGRUDescriptor").unwrap(), descriptor)
    }
}
impl MPSGraph_MPSGraphRNNOps for MPSGraph {}
pub trait MPSGraph_MPSGraphRNNOps: Sized + std::ops::Deref {
    unsafe fn singleGateRNNWithSourceTensor_recurrentWeight_inputWeight_bias_initState_mask_descriptor_name_(
        &self,
        source: MPSGraphTensor,
        recurrentWeight: MPSGraphTensor,
        inputWeight: MPSGraphTensor,
        bias: MPSGraphTensor,
        initState: MPSGraphTensor,
        mask: MPSGraphTensor,
        descriptor: MPSGraphSingleGateRNNDescriptor,
        name: NSString,
    ) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, singleGateRNNWithSourceTensor : source, recurrentWeight : recurrentWeight, inputWeight : inputWeight, bias : bias, initState : initState, mask : mask, descriptor : descriptor, name : name)
    }
    unsafe fn singleGateRNNWithSourceTensor_recurrentWeight_inputWeight_bias_initState_descriptor_name_(
        &self,
        source: MPSGraphTensor,
        recurrentWeight: MPSGraphTensor,
        inputWeight: MPSGraphTensor,
        bias: MPSGraphTensor,
        initState: MPSGraphTensor,
        descriptor: MPSGraphSingleGateRNNDescriptor,
        name: NSString,
    ) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, singleGateRNNWithSourceTensor : source, recurrentWeight : recurrentWeight, inputWeight : inputWeight, bias : bias, initState : initState, descriptor : descriptor, name : name)
    }
    unsafe fn singleGateRNNWithSourceTensor_recurrentWeight_initState_descriptor_name_(
        &self,
        source: MPSGraphTensor,
        recurrentWeight: MPSGraphTensor,
        initState: MPSGraphTensor,
        descriptor: MPSGraphSingleGateRNNDescriptor,
        name: NSString,
    ) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, singleGateRNNWithSourceTensor : source, recurrentWeight : recurrentWeight, initState : initState, descriptor : descriptor, name : name)
    }
    unsafe fn singleGateRNNGradientsWithSourceTensor_recurrentWeight_sourceGradient_zState_stateGradient_inputWeight_bias_initState_mask_descriptor_name_(
        &self,
        source: MPSGraphTensor,
        recurrentWeight: MPSGraphTensor,
        sourceGradient: MPSGraphTensor,
        zState: MPSGraphTensor,
        stateGradient: MPSGraphTensor,
        inputWeight: MPSGraphTensor,
        bias: MPSGraphTensor,
        initState: MPSGraphTensor,
        mask: MPSGraphTensor,
        descriptor: MPSGraphSingleGateRNNDescriptor,
        name: NSString,
    ) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, singleGateRNNGradientsWithSourceTensor : source, recurrentWeight : recurrentWeight, sourceGradient : sourceGradient, zState : zState, stateGradient : stateGradient, inputWeight : inputWeight, bias : bias, initState : initState, mask : mask, descriptor : descriptor, name : name)
    }
    unsafe fn singleGateRNNGradientsWithSourceTensor_recurrentWeight_sourceGradient_zState_inputWeight_bias_initState_mask_descriptor_name_(
        &self,
        source: MPSGraphTensor,
        recurrentWeight: MPSGraphTensor,
        sourceGradient: MPSGraphTensor,
        zState: MPSGraphTensor,
        inputWeight: MPSGraphTensor,
        bias: MPSGraphTensor,
        initState: MPSGraphTensor,
        mask: MPSGraphTensor,
        descriptor: MPSGraphSingleGateRNNDescriptor,
        name: NSString,
    ) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, singleGateRNNGradientsWithSourceTensor : source, recurrentWeight : recurrentWeight, sourceGradient : sourceGradient, zState : zState, inputWeight : inputWeight, bias : bias, initState : initState, mask : mask, descriptor : descriptor, name : name)
    }
    unsafe fn singleGateRNNGradientsWithSourceTensor_recurrentWeight_sourceGradient_zState_inputWeight_bias_initState_descriptor_name_(
        &self,
        source: MPSGraphTensor,
        recurrentWeight: MPSGraphTensor,
        sourceGradient: MPSGraphTensor,
        zState: MPSGraphTensor,
        inputWeight: MPSGraphTensor,
        bias: MPSGraphTensor,
        initState: MPSGraphTensor,
        descriptor: MPSGraphSingleGateRNNDescriptor,
        name: NSString,
    ) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, singleGateRNNGradientsWithSourceTensor : source, recurrentWeight : recurrentWeight, sourceGradient : sourceGradient, zState : zState, inputWeight : inputWeight, bias : bias, initState : initState, descriptor : descriptor, name : name)
    }
    unsafe fn singleGateRNNGradientsWithSourceTensor_recurrentWeight_sourceGradient_zState_initState_descriptor_name_(
        &self,
        source: MPSGraphTensor,
        recurrentWeight: MPSGraphTensor,
        sourceGradient: MPSGraphTensor,
        zState: MPSGraphTensor,
        initState: MPSGraphTensor,
        descriptor: MPSGraphSingleGateRNNDescriptor,
        name: NSString,
    ) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, singleGateRNNGradientsWithSourceTensor : source, recurrentWeight : recurrentWeight, sourceGradient : sourceGradient, zState : zState, initState : initState, descriptor : descriptor, name : name)
    }
    unsafe fn LSTMWithSourceTensor_recurrentWeight_inputWeight_bias_initState_initCell_mask_peephole_descriptor_name_(
        &self,
        source: MPSGraphTensor,
        recurrentWeight: MPSGraphTensor,
        inputWeight: MPSGraphTensor,
        bias: MPSGraphTensor,
        initState: MPSGraphTensor,
        initCell: MPSGraphTensor,
        mask: MPSGraphTensor,
        peephole: MPSGraphTensor,
        descriptor: MPSGraphLSTMDescriptor,
        name: NSString,
    ) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, LSTMWithSourceTensor : source, recurrentWeight : recurrentWeight, inputWeight : inputWeight, bias : bias, initState : initState, initCell : initCell, mask : mask, peephole : peephole, descriptor : descriptor, name : name)
    }
    unsafe fn LSTMWithSourceTensor_recurrentWeight_inputWeight_bias_initState_initCell_descriptor_name_(
        &self,
        source: MPSGraphTensor,
        recurrentWeight: MPSGraphTensor,
        inputWeight: MPSGraphTensor,
        bias: MPSGraphTensor,
        initState: MPSGraphTensor,
        initCell: MPSGraphTensor,
        descriptor: MPSGraphLSTMDescriptor,
        name: NSString,
    ) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, LSTMWithSourceTensor : source, recurrentWeight : recurrentWeight, inputWeight : inputWeight, bias : bias, initState : initState, initCell : initCell, descriptor : descriptor, name : name)
    }
    unsafe fn LSTMWithSourceTensor_recurrentWeight_initState_initCell_descriptor_name_(
        &self,
        source: MPSGraphTensor,
        recurrentWeight: MPSGraphTensor,
        initState: MPSGraphTensor,
        initCell: MPSGraphTensor,
        descriptor: MPSGraphLSTMDescriptor,
        name: NSString,
    ) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, LSTMWithSourceTensor : source, recurrentWeight : recurrentWeight, initState : initState, initCell : initCell, descriptor : descriptor, name : name)
    }
    unsafe fn LSTMGradientsWithSourceTensor_recurrentWeight_sourceGradient_zState_cellOutputFwd_inputWeight_bias_initState_initCell_mask_descriptor_name_(
        &self,
        source: MPSGraphTensor,
        recurrentWeight: MPSGraphTensor,
        sourceGradient: MPSGraphTensor,
        zState: MPSGraphTensor,
        cellOutputFwd: MPSGraphTensor,
        inputWeight: MPSGraphTensor,
        bias: MPSGraphTensor,
        initState: MPSGraphTensor,
        initCell: MPSGraphTensor,
        mask: MPSGraphTensor,
        descriptor: MPSGraphLSTMDescriptor,
        name: NSString,
    ) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, LSTMGradientsWithSourceTensor : source, recurrentWeight : recurrentWeight, sourceGradient : sourceGradient, zState : zState, cellOutputFwd : cellOutputFwd, inputWeight : inputWeight, bias : bias, initState : initState, initCell : initCell, mask : mask, descriptor : descriptor, name : name)
    }
    unsafe fn LSTMGradientsWithSourceTensor_recurrentWeight_sourceGradient_zState_cellOutputFwd_inputWeight_bias_initState_initCell_descriptor_name_(
        &self,
        source: MPSGraphTensor,
        recurrentWeight: MPSGraphTensor,
        sourceGradient: MPSGraphTensor,
        zState: MPSGraphTensor,
        cellOutputFwd: MPSGraphTensor,
        inputWeight: MPSGraphTensor,
        bias: MPSGraphTensor,
        initState: MPSGraphTensor,
        initCell: MPSGraphTensor,
        descriptor: MPSGraphLSTMDescriptor,
        name: NSString,
    ) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, LSTMGradientsWithSourceTensor : source, recurrentWeight : recurrentWeight, sourceGradient : sourceGradient, zState : zState, cellOutputFwd : cellOutputFwd, inputWeight : inputWeight, bias : bias, initState : initState, initCell : initCell, descriptor : descriptor, name : name)
    }
    unsafe fn LSTMGradientsWithSourceTensor_recurrentWeight_sourceGradient_zState_cellOutputFwd_descriptor_name_(
        &self,
        source: MPSGraphTensor,
        recurrentWeight: MPSGraphTensor,
        sourceGradient: MPSGraphTensor,
        zState: MPSGraphTensor,
        cellOutputFwd: MPSGraphTensor,
        descriptor: MPSGraphLSTMDescriptor,
        name: NSString,
    ) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, LSTMGradientsWithSourceTensor : source, recurrentWeight : recurrentWeight, sourceGradient : sourceGradient, zState : zState, cellOutputFwd : cellOutputFwd, descriptor : descriptor, name : name)
    }
    unsafe fn GRUWithSourceTensor_recurrentWeight_inputWeight_bias_initState_mask_secondaryBias_descriptor_name_(
        &self,
        source: MPSGraphTensor,
        recurrentWeight: MPSGraphTensor,
        inputWeight: MPSGraphTensor,
        bias: MPSGraphTensor,
        initState: MPSGraphTensor,
        mask: MPSGraphTensor,
        secondaryBias: MPSGraphTensor,
        descriptor: MPSGraphGRUDescriptor,
        name: NSString,
    ) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, GRUWithSourceTensor : source, recurrentWeight : recurrentWeight, inputWeight : inputWeight, bias : bias, initState : initState, mask : mask, secondaryBias : secondaryBias, descriptor : descriptor, name : name)
    }
    unsafe fn GRUWithSourceTensor_recurrentWeight_inputWeight_bias_initState_descriptor_name_(
        &self,
        source: MPSGraphTensor,
        recurrentWeight: MPSGraphTensor,
        inputWeight: MPSGraphTensor,
        bias: MPSGraphTensor,
        initState: MPSGraphTensor,
        descriptor: MPSGraphGRUDescriptor,
        name: NSString,
    ) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, GRUWithSourceTensor : source, recurrentWeight : recurrentWeight, inputWeight : inputWeight, bias : bias, initState : initState, descriptor : descriptor, name : name)
    }
    unsafe fn GRUWithSourceTensor_recurrentWeight_inputWeight_bias_descriptor_name_(
        &self,
        source: MPSGraphTensor,
        recurrentWeight: MPSGraphTensor,
        inputWeight: MPSGraphTensor,
        bias: MPSGraphTensor,
        descriptor: MPSGraphGRUDescriptor,
        name: NSString,
    ) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, GRUWithSourceTensor : source, recurrentWeight : recurrentWeight, inputWeight : inputWeight, bias : bias, descriptor : descriptor, name : name)
    }
    unsafe fn GRUGradientsWithSourceTensor_recurrentWeight_sourceGradient_zState_outputFwd_inputWeight_bias_initState_descriptor_name_(
        &self,
        source: MPSGraphTensor,
        recurrentWeight: MPSGraphTensor,
        sourceGradient: MPSGraphTensor,
        zState: MPSGraphTensor,
        outputFwd: MPSGraphTensor,
        inputWeight: MPSGraphTensor,
        bias: MPSGraphTensor,
        initState: MPSGraphTensor,
        descriptor: MPSGraphGRUDescriptor,
        name: NSString,
    ) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, GRUGradientsWithSourceTensor : source, recurrentWeight : recurrentWeight, sourceGradient : sourceGradient, zState : zState, outputFwd : outputFwd, inputWeight : inputWeight, bias : bias, initState : initState, descriptor : descriptor, name : name)
    }
    unsafe fn GRUGradientsWithSourceTensor_recurrentWeight_sourceGradient_zState_outputFwd_inputWeight_bias_descriptor_name_(
        &self,
        source: MPSGraphTensor,
        recurrentWeight: MPSGraphTensor,
        sourceGradient: MPSGraphTensor,
        zState: MPSGraphTensor,
        outputFwd: MPSGraphTensor,
        inputWeight: MPSGraphTensor,
        bias: MPSGraphTensor,
        descriptor: MPSGraphGRUDescriptor,
        name: NSString,
    ) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, GRUGradientsWithSourceTensor : source, recurrentWeight : recurrentWeight, sourceGradient : sourceGradient, zState : zState, outputFwd : outputFwd, inputWeight : inputWeight, bias : bias, descriptor : descriptor, name : name)
    }
}
impl MPSGraph_MPSGraphSampleGrid for MPSGraph {}
pub trait MPSGraph_MPSGraphSampleGrid: Sized + std::ops::Deref {
    unsafe fn sampleGridWithSourceTensor_coordinateTensor_layout_normalizeCoordinates_relativeCoordinates_alignCorners_paddingMode_samplingMode_constantValue_name_(
        &self,
        source: MPSGraphTensor,
        coordinates: MPSGraphTensor,
        layout: MPSGraphTensorNamedDataLayout,
        normalizeCoordinates: BOOL,
        relativeCoordinates: BOOL,
        alignCorners: BOOL,
        paddingMode: MPSGraphPaddingMode,
        samplingMode: MPSGraphResizeMode,
        constantValue: f64,
        name: NSString,
    ) -> MPSGraphTensor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, sampleGridWithSourceTensor : source, coordinateTensor : coordinates, layout : layout, normalizeCoordinates : normalizeCoordinates, relativeCoordinates : relativeCoordinates, alignCorners : alignCorners, paddingMode : paddingMode, samplingMode : samplingMode, constantValue : constantValue, name : name)
    }
    unsafe fn sampleGridWithSourceTensor_coordinateTensor_layout_normalizeCoordinates_relativeCoordinates_alignCorners_paddingMode_nearestRoundingMode_constantValue_name_(
        &self,
        source: MPSGraphTensor,
        coordinates: MPSGraphTensor,
        layout: MPSGraphTensorNamedDataLayout,
        normalizeCoordinates: BOOL,
        relativeCoordinates: BOOL,
        alignCorners: BOOL,
        paddingMode: MPSGraphPaddingMode,
        nearestRoundingMode: MPSGraphResizeNearestRoundingMode,
        constantValue: f64,
        name: NSString,
    ) -> MPSGraphTensor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, sampleGridWithSourceTensor : source, coordinateTensor : coordinates, layout : layout, normalizeCoordinates : normalizeCoordinates, relativeCoordinates : relativeCoordinates, alignCorners : alignCorners, paddingMode : paddingMode, nearestRoundingMode : nearestRoundingMode, constantValue : constantValue, name : name)
    }
}
pub type MPSGraphScatterMode = NSInteger;
impl MPSGraph_ScatterNDOps for MPSGraph {}
pub trait MPSGraph_ScatterNDOps: Sized + std::ops::Deref {
    unsafe fn scatterNDWithUpdatesTensor_indicesTensor_shape_batchDimensions_mode_name_(
        &self,
        updatesTensor: MPSGraphTensor,
        indicesTensor: MPSGraphTensor,
        shape: NSArray,
        batchDimensions: NSUInteger,
        mode: MPSGraphScatterMode,
        name: NSString,
    ) -> MPSGraphTensor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, scatterNDWithUpdatesTensor : updatesTensor, indicesTensor : indicesTensor, shape : shape, batchDimensions : batchDimensions, mode : mode, name : name)
    }
    unsafe fn scatterNDWithUpdatesTensor_indicesTensor_shape_batchDimensions_name_(
        &self,
        updatesTensor: MPSGraphTensor,
        indicesTensor: MPSGraphTensor,
        shape: NSArray,
        batchDimensions: NSUInteger,
        name: NSString,
    ) -> MPSGraphTensor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, scatterNDWithUpdatesTensor : updatesTensor, indicesTensor : indicesTensor, shape : shape, batchDimensions : batchDimensions, name : name)
    }
    unsafe fn scatterNDWithDataTensor_updatesTensor_indicesTensor_batchDimensions_mode_name_(
        &self,
        dataTensor: MPSGraphTensor,
        updatesTensor: MPSGraphTensor,
        indicesTensor: MPSGraphTensor,
        batchDimensions: NSUInteger,
        mode: MPSGraphScatterMode,
        name: NSString,
    ) -> MPSGraphTensor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, scatterNDWithDataTensor : dataTensor, updatesTensor : updatesTensor, indicesTensor : indicesTensor, batchDimensions : batchDimensions, mode : mode, name : name)
    }
}
impl MPSGraph_MPSGraphScatterOps for MPSGraph {}
pub trait MPSGraph_MPSGraphScatterOps: Sized + std::ops::Deref {
    unsafe fn scatterWithUpdatesTensor_indicesTensor_shape_axis_mode_name_(
        &self,
        updatesTensor: MPSGraphTensor,
        indicesTensor: MPSGraphTensor,
        shape: NSArray,
        axis: NSInteger,
        mode: MPSGraphScatterMode,
        name: NSString,
    ) -> MPSGraphTensor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, scatterWithUpdatesTensor : updatesTensor, indicesTensor : indicesTensor, shape : shape, axis : axis, mode : mode, name : name)
    }
    unsafe fn scatterWithDataTensor_updatesTensor_indicesTensor_axis_mode_name_(
        &self,
        dataTensor: MPSGraphTensor,
        updatesTensor: MPSGraphTensor,
        indicesTensor: MPSGraphTensor,
        axis: NSInteger,
        mode: MPSGraphScatterMode,
        name: NSString,
    ) -> MPSGraphTensor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, scatterWithDataTensor : dataTensor, updatesTensor : updatesTensor, indicesTensor : indicesTensor, axis : axis, mode : mode, name : name)
    }
}
impl MPSGraph_MPSGraphScatterAlongAxisOps for MPSGraph {}
pub trait MPSGraph_MPSGraphScatterAlongAxisOps: Sized + std::ops::Deref {
    unsafe fn scatterAlongAxis_withUpdatesTensor_indicesTensor_shape_mode_name_(
        &self,
        axis: NSInteger,
        updatesTensor: MPSGraphTensor,
        indicesTensor: MPSGraphTensor,
        shape: NSArray,
        mode: MPSGraphScatterMode,
        name: NSString,
    ) -> MPSGraphTensor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, scatterAlongAxis : axis, withUpdatesTensor : updatesTensor, indicesTensor : indicesTensor, shape : shape, mode : mode, name : name)
    }
    unsafe fn scatterAlongAxisTensor_withUpdatesTensor_indicesTensor_shape_mode_name_(
        &self,
        axisTensor: MPSGraphTensor,
        updatesTensor: MPSGraphTensor,
        indicesTensor: MPSGraphTensor,
        shape: NSArray,
        mode: MPSGraphScatterMode,
        name: NSString,
    ) -> MPSGraphTensor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, scatterAlongAxisTensor : axisTensor, withUpdatesTensor : updatesTensor, indicesTensor : indicesTensor, shape : shape, mode : mode, name : name)
    }
    unsafe fn scatterAlongAxis_withDataTensor_updatesTensor_indicesTensor_mode_name_(
        &self,
        axis: NSInteger,
        dataTensor: MPSGraphTensor,
        updatesTensor: MPSGraphTensor,
        indicesTensor: MPSGraphTensor,
        mode: MPSGraphScatterMode,
        name: NSString,
    ) -> MPSGraphTensor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, scatterAlongAxis : axis, withDataTensor : dataTensor, updatesTensor : updatesTensor, indicesTensor : indicesTensor, mode : mode, name : name)
    }
    unsafe fn scatterAlongAxisTensor_withDataTensor_updatesTensor_indicesTensor_mode_name_(
        &self,
        axisTensor: MPSGraphTensor,
        dataTensor: MPSGraphTensor,
        updatesTensor: MPSGraphTensor,
        indicesTensor: MPSGraphTensor,
        mode: MPSGraphScatterMode,
        name: NSString,
    ) -> MPSGraphTensor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, scatterAlongAxisTensor : axisTensor, withDataTensor : dataTensor, updatesTensor : updatesTensor, indicesTensor : indicesTensor, mode : mode, name : name)
    }
}
impl MPSGraph_MPSGraphSortOps for MPSGraph {}
pub trait MPSGraph_MPSGraphSortOps: Sized + std::ops::Deref {
    unsafe fn sortWithTensor_axis_descending_name_(
        &self,
        tensor: MPSGraphTensor,
        axis: NSInteger,
        descending: BOOL,
        name: NSString,
    ) -> MPSGraphTensor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, sortWithTensor : tensor, axis : axis, descending : descending, name : name)
    }
    unsafe fn sortWithTensor_axisTensor_descending_name_(
        &self,
        tensor: MPSGraphTensor,
        axisTensor: MPSGraphTensor,
        descending: BOOL,
        name: NSString,
    ) -> MPSGraphTensor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, sortWithTensor : tensor, axisTensor : axisTensor, descending : descending, name : name)
    }
    unsafe fn sortWithTensor_axis_name_(
        &self,
        tensor: MPSGraphTensor,
        axis: NSInteger,
        name: NSString,
    ) -> MPSGraphTensor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, sortWithTensor : tensor, axis : axis, name : name)
    }
    unsafe fn sortWithTensor_axisTensor_name_(
        &self,
        tensor: MPSGraphTensor,
        axisTensor: MPSGraphTensor,
        name: NSString,
    ) -> MPSGraphTensor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, sortWithTensor : tensor, axisTensor : axisTensor, name : name)
    }
    unsafe fn argSortWithTensor_axis_descending_name_(
        &self,
        tensor: MPSGraphTensor,
        axis: NSInteger,
        descending: BOOL,
        name: NSString,
    ) -> MPSGraphTensor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, argSortWithTensor : tensor, axis : axis, descending : descending, name : name)
    }
    unsafe fn argSortWithTensor_axisTensor_descending_name_(
        &self,
        tensor: MPSGraphTensor,
        axisTensor: MPSGraphTensor,
        descending: BOOL,
        name: NSString,
    ) -> MPSGraphTensor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, argSortWithTensor : tensor, axisTensor : axisTensor, descending : descending, name : name)
    }
    unsafe fn argSortWithTensor_axis_name_(
        &self,
        tensor: MPSGraphTensor,
        axis: NSInteger,
        name: NSString,
    ) -> MPSGraphTensor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, argSortWithTensor : tensor, axis : axis, name : name)
    }
    unsafe fn argSortWithTensor_axisTensor_name_(
        &self,
        tensor: MPSGraphTensor,
        axisTensor: MPSGraphTensor,
        name: NSString,
    ) -> MPSGraphTensor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, argSortWithTensor : tensor, axisTensor : axisTensor, name : name)
    }
}
pub type MPSGraphSparseStorageType = u64;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MPSGraphCreateSparseOpDescriptor(pub id);
impl std::ops::Deref for MPSGraphCreateSparseOpDescriptor {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MPSGraphCreateSparseOpDescriptor {}
impl MPSGraphCreateSparseOpDescriptor {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MPSGraphCreateSparseOpDescriptor").unwrap(), alloc) })
    }
}
impl PNSCopying for MPSGraphCreateSparseOpDescriptor {}
impl IMPSGraphObject for MPSGraphCreateSparseOpDescriptor {}
impl std::convert::TryFrom<MPSGraphObject> for MPSGraphCreateSparseOpDescriptor {
    type Error = &'static str;
    fn try_from(parent: MPSGraphObject) -> Result<MPSGraphCreateSparseOpDescriptor, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MPSGraphCreateSparseOpDescriptor").unwrap())
        };
        if is_kind_of {
            Ok(MPSGraphCreateSparseOpDescriptor(parent.0))
        } else {
            Err("This MPSGraphObject cannot be downcasted to MPSGraphCreateSparseOpDescriptor")
        }
    }
}
impl INSObject for MPSGraphCreateSparseOpDescriptor {}
impl PNSObject for MPSGraphCreateSparseOpDescriptor {}
impl IMPSGraphCreateSparseOpDescriptor for MPSGraphCreateSparseOpDescriptor {}
pub trait IMPSGraphCreateSparseOpDescriptor: Sized + std::ops::Deref {
    unsafe fn sparseStorageType(&self) -> MPSGraphSparseStorageType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, sparseStorageType)
    }
    unsafe fn setSparseStorageType_(&self, sparseStorageType: MPSGraphSparseStorageType)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSparseStorageType : sparseStorageType)
    }
    unsafe fn dataType(&self) -> MPSDataType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, dataType)
    }
    unsafe fn setDataType_(&self, dataType: MPSDataType)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDataType : dataType)
    }
    unsafe fn descriptorWithStorageType_dataType_(
        sparseStorageType: MPSGraphSparseStorageType,
        dataType: MPSDataType,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MPSGraphCreateSparseOpDescriptor").unwrap(), descriptorWithStorageType : sparseStorageType, dataType : dataType)
    }
}
impl MPSGraph_MPSGraphSparseOps for MPSGraph {}
pub trait MPSGraph_MPSGraphSparseOps: Sized + std::ops::Deref {
    unsafe fn sparseTensorWithType_tensors_shape_dataType_name_(
        &self,
        sparseStorageType: MPSGraphSparseStorageType,
        inputTensorArray: NSArray,
        shape: NSArray,
        dataType: MPSDataType,
        name: NSString,
    ) -> MPSGraphTensor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, sparseTensorWithType : sparseStorageType, tensors : inputTensorArray, shape : shape, dataType : dataType, name : name)
    }
    unsafe fn sparseTensorWithDescriptor_tensors_shape_name_(
        &self,
        sparseDescriptor: MPSGraphCreateSparseOpDescriptor,
        inputTensorArray: NSArray,
        shape: NSArray,
        name: NSString,
    ) -> MPSGraphTensor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, sparseTensorWithDescriptor : sparseDescriptor, tensors : inputTensorArray, shape : shape, name : name)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MPSGraphStencilOpDescriptor(pub id);
impl std::ops::Deref for MPSGraphStencilOpDescriptor {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MPSGraphStencilOpDescriptor {}
impl MPSGraphStencilOpDescriptor {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MPSGraphStencilOpDescriptor").unwrap(), alloc) })
    }
}
impl PNSCopying for MPSGraphStencilOpDescriptor {}
impl IMPSGraphObject for MPSGraphStencilOpDescriptor {}
impl std::convert::TryFrom<MPSGraphObject> for MPSGraphStencilOpDescriptor {
    type Error = &'static str;
    fn try_from(parent: MPSGraphObject) -> Result<MPSGraphStencilOpDescriptor, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MPSGraphStencilOpDescriptor").unwrap()) };
        if is_kind_of {
            Ok(MPSGraphStencilOpDescriptor(parent.0))
        } else {
            Err("This MPSGraphObject cannot be downcasted to MPSGraphStencilOpDescriptor")
        }
    }
}
impl INSObject for MPSGraphStencilOpDescriptor {}
impl PNSObject for MPSGraphStencilOpDescriptor {}
impl IMPSGraphStencilOpDescriptor for MPSGraphStencilOpDescriptor {}
pub trait IMPSGraphStencilOpDescriptor: Sized + std::ops::Deref {
    unsafe fn reductionMode(&self) -> MPSGraphReductionMode
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, reductionMode)
    }
    unsafe fn setReductionMode_(&self, reductionMode: MPSGraphReductionMode)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setReductionMode : reductionMode)
    }
    unsafe fn offsets(&self) -> *mut MPSShape
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, offsets)
    }
    unsafe fn setOffsets_(&self, offsets: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setOffsets : offsets)
    }
    unsafe fn strides(&self) -> *mut MPSShape
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, strides)
    }
    unsafe fn setStrides_(&self, strides: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setStrides : strides)
    }
    unsafe fn dilationRates(&self) -> *mut MPSShape
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, dilationRates)
    }
    unsafe fn setDilationRates_(&self, dilationRates: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDilationRates : dilationRates)
    }
    unsafe fn explicitPadding(&self) -> *mut MPSShape
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, explicitPadding)
    }
    unsafe fn setExplicitPadding_(&self, explicitPadding: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setExplicitPadding : explicitPadding)
    }
    unsafe fn boundaryMode(&self) -> MPSGraphPaddingMode
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, boundaryMode)
    }
    unsafe fn setBoundaryMode_(&self, boundaryMode: MPSGraphPaddingMode)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setBoundaryMode : boundaryMode)
    }
    unsafe fn paddingStyle(&self) -> MPSGraphPaddingStyle
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, paddingStyle)
    }
    unsafe fn setPaddingStyle_(&self, paddingStyle: MPSGraphPaddingStyle)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPaddingStyle : paddingStyle)
    }
    unsafe fn paddingConstant(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, paddingConstant)
    }
    unsafe fn setPaddingConstant_(&self, paddingConstant: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPaddingConstant : paddingConstant)
    }
    unsafe fn descriptorWithReductionMode_offsets_strides_dilationRates_explicitPadding_boundaryMode_paddingStyle_paddingConstant_(
        reductionMode: MPSGraphReductionMode,
        offsets: NSArray,
        strides: NSArray,
        dilationRates: NSArray,
        explicitPadding: NSArray,
        boundaryMode: MPSGraphPaddingMode,
        paddingStyle: MPSGraphPaddingStyle,
        paddingConstant: f32,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MPSGraphStencilOpDescriptor").unwrap(), descriptorWithReductionMode : reductionMode, offsets : offsets, strides : strides, dilationRates : dilationRates, explicitPadding : explicitPadding, boundaryMode : boundaryMode, paddingStyle : paddingStyle, paddingConstant : paddingConstant)
    }
    unsafe fn descriptorWithOffsets_explicitPadding_(
        offsets: NSArray,
        explicitPadding: NSArray,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MPSGraphStencilOpDescriptor").unwrap(), descriptorWithOffsets : offsets, explicitPadding : explicitPadding)
    }
    unsafe fn descriptorWithExplicitPadding_(explicitPadding: NSArray) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MPSGraphStencilOpDescriptor").unwrap(), descriptorWithExplicitPadding : explicitPadding)
    }
    unsafe fn descriptorWithPaddingStyle_(paddingStyle: MPSGraphPaddingStyle) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MPSGraphStencilOpDescriptor").unwrap(), descriptorWithPaddingStyle : paddingStyle)
    }
}
impl MPSGraph_MPSGraphStencilOps for MPSGraph {}
pub trait MPSGraph_MPSGraphStencilOps: Sized + std::ops::Deref {
    unsafe fn stencilWithSourceTensor_weightsTensor_descriptor_name_(
        &self,
        source: MPSGraphTensor,
        weights: MPSGraphTensor,
        descriptor: MPSGraphStencilOpDescriptor,
        name: NSString,
    ) -> MPSGraphTensor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, stencilWithSourceTensor : source, weightsTensor : weights, descriptor : descriptor, name : name)
    }
}
impl MPSGraph_MPSGraphTensorShapeOps for MPSGraph {}
pub trait MPSGraph_MPSGraphTensorShapeOps: Sized + std::ops::Deref {
    unsafe fn reshapeTensor_withShape_name_(
        &self,
        tensor: MPSGraphTensor,
        shape: NSArray,
        name: NSString,
    ) -> MPSGraphTensor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, reshapeTensor : tensor, withShape : shape, name : name)
    }
    unsafe fn reshapeTensor_withShapeTensor_name_(
        &self,
        tensor: MPSGraphTensor,
        shapeTensor: MPSGraphTensor,
        name: NSString,
    ) -> MPSGraphTensor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, reshapeTensor : tensor, withShapeTensor : shapeTensor, name : name)
    }
    unsafe fn transposeTensor_dimension_withDimension_name_(
        &self,
        tensor: MPSGraphTensor,
        dimensionIndex: NSUInteger,
        dimensionIndex2: NSUInteger,
        name: NSString,
    ) -> MPSGraphTensor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, transposeTensor : tensor, dimension : dimensionIndex, withDimension : dimensionIndex2, name : name)
    }
    unsafe fn transposeTensor_permutation_name_(
        &self,
        tensor: MPSGraphTensor,
        permutation: NSArray,
        name: NSString,
    ) -> MPSGraphTensor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, transposeTensor : tensor, permutation : permutation, name : name)
    }
    unsafe fn sliceTensor_dimension_start_length_name_(
        &self,
        tensor: MPSGraphTensor,
        dimensionIndex: NSUInteger,
        start: NSInteger,
        length: NSInteger,
        name: NSString,
    ) -> MPSGraphTensor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, sliceTensor : tensor, dimension : dimensionIndex, start : start, length : length, name : name)
    }
    unsafe fn sliceTensor_starts_ends_strides_name_(
        &self,
        tensor: MPSGraphTensor,
        starts: NSArray,
        ends: NSArray,
        strides: NSArray,
        name: NSString,
    ) -> MPSGraphTensor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, sliceTensor : tensor, starts : starts, ends : ends, strides : strides, name : name)
    }
    unsafe fn sliceTensor_starts_ends_strides_startMask_endMask_squeezeMask_name_(
        &self,
        tensor: MPSGraphTensor,
        starts: NSArray,
        ends: NSArray,
        strides: NSArray,
        startMask: u32,
        endMask: u32,
        squeezeMask: u32,
        name: NSString,
    ) -> MPSGraphTensor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, sliceTensor : tensor, starts : starts, ends : ends, strides : strides, startMask : startMask, endMask : endMask, squeezeMask : squeezeMask, name : name)
    }
    unsafe fn sliceTensor_startTensor_endTensor_strideTensor_startMask_endMask_squeezeMask_name_(
        &self,
        tensor: MPSGraphTensor,
        startTensor: MPSGraphTensor,
        endTensor: MPSGraphTensor,
        strideTensor: MPSGraphTensor,
        startMask: u32,
        endMask: u32,
        squeezeMask: u32,
        name: NSString,
    ) -> MPSGraphTensor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, sliceTensor : tensor, startTensor : startTensor, endTensor : endTensor, strideTensor : strideTensor, startMask : startMask, endMask : endMask, squeezeMask : squeezeMask, name : name)
    }
    unsafe fn sliceTensor_startTensor_sizeTensor_squeezeMask_name_(
        &self,
        tensor: MPSGraphTensor,
        startTensor: MPSGraphTensor,
        sizeTensor: MPSGraphTensor,
        squeezeMask: u32,
        name: NSString,
    ) -> MPSGraphTensor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, sliceTensor : tensor, startTensor : startTensor, sizeTensor : sizeTensor, squeezeMask : squeezeMask, name : name)
    }
    unsafe fn sliceGradientTensor_fwdInShapeTensor_starts_ends_strides_name_(
        &self,
        inputGradientTensor: MPSGraphTensor,
        fwdInShapeTensor: MPSGraphTensor,
        starts: NSArray,
        ends: NSArray,
        strides: NSArray,
        name: NSString,
    ) -> MPSGraphTensor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, sliceGradientTensor : inputGradientTensor, fwdInShapeTensor : fwdInShapeTensor, starts : starts, ends : ends, strides : strides, name : name)
    }
    unsafe fn sliceGradientTensor_fwdInShapeTensor_startTensor_endTensor_strideTensor_startMask_endMask_squeezeMask_name_(
        &self,
        inputGradientTensor: MPSGraphTensor,
        fwdInShapeTensor: MPSGraphTensor,
        startTensor: MPSGraphTensor,
        endTensor: MPSGraphTensor,
        strideTensor: MPSGraphTensor,
        startMask: u32,
        endMask: u32,
        squeezeMask: u32,
        name: NSString,
    ) -> MPSGraphTensor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, sliceGradientTensor : inputGradientTensor, fwdInShapeTensor : fwdInShapeTensor, startTensor : startTensor, endTensor : endTensor, strideTensor : strideTensor, startMask : startMask, endMask : endMask, squeezeMask : squeezeMask, name : name)
    }
    unsafe fn sliceGradientTensor_fwdInShapeTensor_startTensor_sizeTensor_squeezeMask_name_(
        &self,
        inputGradientTensor: MPSGraphTensor,
        fwdInShapeTensor: MPSGraphTensor,
        startTensor: MPSGraphTensor,
        sizeTensor: MPSGraphTensor,
        squeezeMask: u32,
        name: NSString,
    ) -> MPSGraphTensor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, sliceGradientTensor : inputGradientTensor, fwdInShapeTensor : fwdInShapeTensor, startTensor : startTensor, sizeTensor : sizeTensor, squeezeMask : squeezeMask, name : name)
    }
    unsafe fn sliceGradientTensor_fwdInShapeTensor_starts_ends_strides_startMask_endMask_squeezeMask_name_(
        &self,
        inputGradientTensor: MPSGraphTensor,
        fwdInShapeTensor: MPSGraphTensor,
        starts: NSArray,
        ends: NSArray,
        strides: NSArray,
        startMask: u32,
        endMask: u32,
        squeezeMask: u32,
        name: NSString,
    ) -> MPSGraphTensor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, sliceGradientTensor : inputGradientTensor, fwdInShapeTensor : fwdInShapeTensor, starts : starts, ends : ends, strides : strides, startMask : startMask, endMask : endMask, squeezeMask : squeezeMask, name : name)
    }
    unsafe fn sliceUpdateDataTensor_updateTensor_startsTensor_endsTensor_stridesTensor_startMask_endMask_squeezeMask_name_(
        &self,
        dataTensor: MPSGraphTensor,
        updateTensor: MPSGraphTensor,
        startsTensor: MPSGraphTensor,
        endsTensor: MPSGraphTensor,
        stridesTensor: MPSGraphTensor,
        startMask: u32,
        endMask: u32,
        squeezeMask: u32,
        name: NSString,
    ) -> MPSGraphTensor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, sliceUpdateDataTensor : dataTensor, updateTensor : updateTensor, startsTensor : startsTensor, endsTensor : endsTensor, stridesTensor : stridesTensor, startMask : startMask, endMask : endMask, squeezeMask : squeezeMask, name : name)
    }
    unsafe fn sliceUpdateDataTensor_updateTensor_starts_ends_strides_startMask_endMask_squeezeMask_name_(
        &self,
        dataTensor: MPSGraphTensor,
        updateTensor: MPSGraphTensor,
        starts: NSArray,
        ends: NSArray,
        strides: NSArray,
        startMask: u32,
        endMask: u32,
        squeezeMask: u32,
        name: NSString,
    ) -> MPSGraphTensor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, sliceUpdateDataTensor : dataTensor, updateTensor : updateTensor, starts : starts, ends : ends, strides : strides, startMask : startMask, endMask : endMask, squeezeMask : squeezeMask, name : name)
    }
    unsafe fn sliceUpdateDataTensor_updateTensor_startsTensor_endsTensor_stridesTensor_name_(
        &self,
        dataTensor: MPSGraphTensor,
        updateTensor: MPSGraphTensor,
        startsTensor: MPSGraphTensor,
        endsTensor: MPSGraphTensor,
        stridesTensor: MPSGraphTensor,
        name: NSString,
    ) -> MPSGraphTensor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, sliceUpdateDataTensor : dataTensor, updateTensor : updateTensor, startsTensor : startsTensor, endsTensor : endsTensor, stridesTensor : stridesTensor, name : name)
    }
    unsafe fn sliceUpdateDataTensor_updateTensor_starts_ends_strides_name_(
        &self,
        dataTensor: MPSGraphTensor,
        updateTensor: MPSGraphTensor,
        starts: NSArray,
        ends: NSArray,
        strides: NSArray,
        name: NSString,
    ) -> MPSGraphTensor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, sliceUpdateDataTensor : dataTensor, updateTensor : updateTensor, starts : starts, ends : ends, strides : strides, name : name)
    }
    unsafe fn concatTensor_withTensor_dimension_name_(
        &self,
        tensor: MPSGraphTensor,
        tensor2: MPSGraphTensor,
        dimensionIndex: NSInteger,
        name: NSString,
    ) -> MPSGraphTensor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, concatTensor : tensor, withTensor : tensor2, dimension : dimensionIndex, name : name)
    }
    unsafe fn concatTensors_dimension_name_(
        &self,
        tensors: NSArray,
        dimensionIndex: NSInteger,
        name: NSString,
    ) -> MPSGraphTensor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, concatTensors : tensors, dimension : dimensionIndex, name : name)
    }
    unsafe fn concatTensors_dimension_interleave_name_(
        &self,
        tensors: NSArray,
        dimensionIndex: NSInteger,
        interleave: BOOL,
        name: NSString,
    ) -> MPSGraphTensor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, concatTensors : tensors, dimension : dimensionIndex, interleave : interleave, name : name)
    }
    unsafe fn tileTensor_withMultiplier_name_(
        &self,
        tensor: MPSGraphTensor,
        multiplier: NSArray,
        name: NSString,
    ) -> MPSGraphTensor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, tileTensor : tensor, withMultiplier : multiplier, name : name)
    }
    unsafe fn tileGradientWithIncomingGradientTensor_sourceTensor_withMultiplier_name_(
        &self,
        incomingGradientTensor: MPSGraphTensor,
        sourceTensor: MPSGraphTensor,
        multiplier: NSArray,
        name: NSString,
    ) -> MPSGraphTensor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, tileGradientWithIncomingGradientTensor : incomingGradientTensor, sourceTensor : sourceTensor, withMultiplier : multiplier, name : name)
    }
    unsafe fn padTensor_withPaddingMode_leftPadding_rightPadding_constantValue_name_(
        &self,
        tensor: MPSGraphTensor,
        paddingMode: MPSGraphPaddingMode,
        leftPadding: NSArray,
        rightPadding: NSArray,
        constantValue: f64,
        name: NSString,
    ) -> MPSGraphTensor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, padTensor : tensor, withPaddingMode : paddingMode, leftPadding : leftPadding, rightPadding : rightPadding, constantValue : constantValue, name : name)
    }
    unsafe fn padGradientWithIncomingGradientTensor_sourceTensor_paddingMode_leftPadding_rightPadding_name_(
        &self,
        incomingGradientTensor: MPSGraphTensor,
        sourceTensor: MPSGraphTensor,
        paddingMode: MPSGraphPaddingMode,
        leftPadding: NSArray,
        rightPadding: NSArray,
        name: NSString,
    ) -> MPSGraphTensor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, padGradientWithIncomingGradientTensor : incomingGradientTensor, sourceTensor : sourceTensor, paddingMode : paddingMode, leftPadding : leftPadding, rightPadding : rightPadding, name : name)
    }
    unsafe fn spaceToDepth2DTensor_widthAxis_heightAxis_depthAxis_blockSize_usePixelShuffleOrder_name_(
        &self,
        tensor: MPSGraphTensor,
        widthAxis: NSUInteger,
        heightAxis: NSUInteger,
        depthAxis: NSUInteger,
        blockSize: NSUInteger,
        usePixelShuffleOrder: BOOL,
        name: NSString,
    ) -> MPSGraphTensor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, spaceToDepth2DTensor : tensor, widthAxis : widthAxis, heightAxis : heightAxis, depthAxis : depthAxis, blockSize : blockSize, usePixelShuffleOrder : usePixelShuffleOrder, name : name)
    }
    unsafe fn spaceToDepth2DTensor_widthAxisTensor_heightAxisTensor_depthAxisTensor_blockSize_usePixelShuffleOrder_name_(
        &self,
        tensor: MPSGraphTensor,
        widthAxisTensor: MPSGraphTensor,
        heightAxisTensor: MPSGraphTensor,
        depthAxisTensor: MPSGraphTensor,
        blockSize: NSUInteger,
        usePixelShuffleOrder: BOOL,
        name: NSString,
    ) -> MPSGraphTensor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, spaceToDepth2DTensor : tensor, widthAxisTensor : widthAxisTensor, heightAxisTensor : heightAxisTensor, depthAxisTensor : depthAxisTensor, blockSize : blockSize, usePixelShuffleOrder : usePixelShuffleOrder, name : name)
    }
    unsafe fn depthToSpace2DTensor_widthAxis_heightAxis_depthAxis_blockSize_usePixelShuffleOrder_name_(
        &self,
        tensor: MPSGraphTensor,
        widthAxis: NSUInteger,
        heightAxis: NSUInteger,
        depthAxis: NSUInteger,
        blockSize: NSUInteger,
        usePixelShuffleOrder: BOOL,
        name: NSString,
    ) -> MPSGraphTensor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, depthToSpace2DTensor : tensor, widthAxis : widthAxis, heightAxis : heightAxis, depthAxis : depthAxis, blockSize : blockSize, usePixelShuffleOrder : usePixelShuffleOrder, name : name)
    }
    unsafe fn depthToSpace2DTensor_widthAxisTensor_heightAxisTensor_depthAxisTensor_blockSize_usePixelShuffleOrder_name_(
        &self,
        tensor: MPSGraphTensor,
        widthAxisTensor: MPSGraphTensor,
        heightAxisTensor: MPSGraphTensor,
        depthAxisTensor: MPSGraphTensor,
        blockSize: NSUInteger,
        usePixelShuffleOrder: BOOL,
        name: NSString,
    ) -> MPSGraphTensor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, depthToSpace2DTensor : tensor, widthAxisTensor : widthAxisTensor, heightAxisTensor : heightAxisTensor, depthAxisTensor : depthAxisTensor, blockSize : blockSize, usePixelShuffleOrder : usePixelShuffleOrder, name : name)
    }
    unsafe fn spaceToBatchTensor_spatialAxes_batchAxis_blockDimensions_usePixelShuffleOrder_name_(
        &self,
        tensor: MPSGraphTensor,
        spatialAxes: NSArray,
        batchAxis: NSInteger,
        blockDimensions: NSArray,
        usePixelShuffleOrder: BOOL,
        name: NSString,
    ) -> MPSGraphTensor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, spaceToBatchTensor : tensor, spatialAxes : spatialAxes, batchAxis : batchAxis, blockDimensions : blockDimensions, usePixelShuffleOrder : usePixelShuffleOrder, name : name)
    }
    unsafe fn spaceToBatchTensor_spatialAxesTensor_batchAxisTensor_blockDimensionsTensor_usePixelShuffleOrder_name_(
        &self,
        tensor: MPSGraphTensor,
        spatialAxesTensor: MPSGraphTensor,
        batchAxisTensor: MPSGraphTensor,
        blockDimensionsTensor: MPSGraphTensor,
        usePixelShuffleOrder: BOOL,
        name: NSString,
    ) -> MPSGraphTensor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, spaceToBatchTensor : tensor, spatialAxesTensor : spatialAxesTensor, batchAxisTensor : batchAxisTensor, blockDimensionsTensor : blockDimensionsTensor, usePixelShuffleOrder : usePixelShuffleOrder, name : name)
    }
    unsafe fn batchToSpaceTensor_spatialAxes_batchAxis_blockDimensions_usePixelShuffleOrder_name_(
        &self,
        tensor: MPSGraphTensor,
        spatialAxes: NSArray,
        batchAxis: NSInteger,
        blockDimensions: NSArray,
        usePixelShuffleOrder: BOOL,
        name: NSString,
    ) -> MPSGraphTensor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, batchToSpaceTensor : tensor, spatialAxes : spatialAxes, batchAxis : batchAxis, blockDimensions : blockDimensions, usePixelShuffleOrder : usePixelShuffleOrder, name : name)
    }
    unsafe fn batchToSpaceTensor_spatialAxesTensor_batchAxisTensor_blockDimensionsTensor_usePixelShuffleOrder_name_(
        &self,
        tensor: MPSGraphTensor,
        spatialAxesTensor: MPSGraphTensor,
        batchAxisTensor: MPSGraphTensor,
        blockDimensionsTensor: MPSGraphTensor,
        usePixelShuffleOrder: BOOL,
        name: NSString,
    ) -> MPSGraphTensor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, batchToSpaceTensor : tensor, spatialAxesTensor : spatialAxesTensor, batchAxisTensor : batchAxisTensor, blockDimensionsTensor : blockDimensionsTensor, usePixelShuffleOrder : usePixelShuffleOrder, name : name)
    }
    unsafe fn reverseTensor_axesTensor_name_(
        &self,
        tensor: MPSGraphTensor,
        axesTensor: MPSGraphTensor,
        name: NSString,
    ) -> MPSGraphTensor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, reverseTensor : tensor, axesTensor : axesTensor, name : name)
    }
    unsafe fn reverseTensor_axes_name_(
        &self,
        tensor: MPSGraphTensor,
        axes: NSArray,
        name: NSString,
    ) -> MPSGraphTensor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, reverseTensor : tensor, axes : axes, name : name)
    }
    unsafe fn reverseTensor_name_(&self, tensor: MPSGraphTensor, name: NSString) -> MPSGraphTensor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, reverseTensor : tensor, name : name)
    }
    unsafe fn flatten2DTensor_axis_name_(
        &self,
        tensor: MPSGraphTensor,
        axis: NSInteger,
        name: NSString,
    ) -> MPSGraphTensor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, flatten2DTensor : tensor, axis : axis, name : name)
    }
    unsafe fn flatten2DTensor_axisTensor_name_(
        &self,
        tensor: MPSGraphTensor,
        axisTensor: MPSGraphTensor,
        name: NSString,
    ) -> MPSGraphTensor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, flatten2DTensor : tensor, axisTensor : axisTensor, name : name)
    }
    unsafe fn broadcastTensor_toShape_name_(
        &self,
        tensor: MPSGraphTensor,
        shape: NSArray,
        name: NSString,
    ) -> MPSGraphTensor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, broadcastTensor : tensor, toShape : shape, name : name)
    }
    unsafe fn broadcastTensor_toShapeTensor_name_(
        &self,
        tensor: MPSGraphTensor,
        shapeTensor: MPSGraphTensor,
        name: NSString,
    ) -> MPSGraphTensor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, broadcastTensor : tensor, toShapeTensor : shapeTensor, name : name)
    }
    unsafe fn shapeOfTensor_name_(&self, tensor: MPSGraphTensor, name: NSString) -> MPSGraphTensor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, shapeOfTensor : tensor, name : name)
    }
    unsafe fn castTensor_toType_name_(
        &self,
        tensor: MPSGraphTensor,
        type_: MPSDataType,
        name: NSString,
    ) -> MPSGraphTensor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, castTensor : tensor, toType : type_, name : name)
    }
    unsafe fn reinterpretCastTensor_toType_name_(
        &self,
        tensor: MPSGraphTensor,
        type_: MPSDataType,
        name: NSString,
    ) -> MPSGraphTensor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, reinterpretCastTensor : tensor, toType : type_, name : name)
    }
    unsafe fn stackTensors_axis_name_(
        &self,
        inputTensors: NSArray,
        axis: NSInteger,
        name: NSString,
    ) -> MPSGraphTensor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, stackTensors : inputTensors, axis : axis, name : name)
    }
    unsafe fn splitTensor_splitSizes_axis_name_(
        &self,
        tensor: MPSGraphTensor,
        splitSizes: NSArray,
        axis: NSInteger,
        name: NSString,
    ) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, splitTensor : tensor, splitSizes : splitSizes, axis : axis, name : name)
    }
    unsafe fn splitTensor_splitSizesTensor_axis_name_(
        &self,
        tensor: MPSGraphTensor,
        splitSizesTensor: MPSGraphTensor,
        axis: NSInteger,
        name: NSString,
    ) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, splitTensor : tensor, splitSizesTensor : splitSizesTensor, axis : axis, name : name)
    }
    unsafe fn splitTensor_numSplits_axis_name_(
        &self,
        tensor: MPSGraphTensor,
        numSplits: NSUInteger,
        axis: NSInteger,
        name: NSString,
    ) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, splitTensor : tensor, numSplits : numSplits, axis : axis, name : name)
    }
    unsafe fn squeezeTensor_name_(&self, tensor: MPSGraphTensor, name: NSString) -> MPSGraphTensor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, squeezeTensor : tensor, name : name)
    }
    unsafe fn squeezeTensor_axis_name_(
        &self,
        tensor: MPSGraphTensor,
        axis: NSInteger,
        name: NSString,
    ) -> MPSGraphTensor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, squeezeTensor : tensor, axis : axis, name : name)
    }
    unsafe fn squeezeTensor_axes_name_(
        &self,
        tensor: MPSGraphTensor,
        axes: NSArray,
        name: NSString,
    ) -> MPSGraphTensor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, squeezeTensor : tensor, axes : axes, name : name)
    }
    unsafe fn squeezeTensor_axesTensor_name_(
        &self,
        tensor: MPSGraphTensor,
        axesTensor: MPSGraphTensor,
        name: NSString,
    ) -> MPSGraphTensor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, squeezeTensor : tensor, axesTensor : axesTensor, name : name)
    }
    unsafe fn expandDimsOfTensor_axis_name_(
        &self,
        tensor: MPSGraphTensor,
        axis: NSInteger,
        name: NSString,
    ) -> MPSGraphTensor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, expandDimsOfTensor : tensor, axis : axis, name : name)
    }
    unsafe fn expandDimsOfTensor_axes_name_(
        &self,
        tensor: MPSGraphTensor,
        axes: NSArray,
        name: NSString,
    ) -> MPSGraphTensor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, expandDimsOfTensor : tensor, axes : axes, name : name)
    }
    unsafe fn expandDimsOfTensor_axesTensor_name_(
        &self,
        tensor: MPSGraphTensor,
        axesTensor: MPSGraphTensor,
        name: NSString,
    ) -> MPSGraphTensor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, expandDimsOfTensor : tensor, axesTensor : axesTensor, name : name)
    }
    unsafe fn coordinateAlongAxis_withShape_name_(
        &self,
        axis: NSInteger,
        shape: NSArray,
        name: NSString,
    ) -> MPSGraphTensor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, coordinateAlongAxis : axis, withShape : shape, name : name)
    }
    unsafe fn coordinateAlongAxisTensor_withShape_name_(
        &self,
        axisTensor: MPSGraphTensor,
        shape: NSArray,
        name: NSString,
    ) -> MPSGraphTensor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, coordinateAlongAxisTensor : axisTensor, withShape : shape, name : name)
    }
    unsafe fn coordinateAlongAxis_withShapeTensor_name_(
        &self,
        axis: NSInteger,
        shapeTensor: MPSGraphTensor,
        name: NSString,
    ) -> MPSGraphTensor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, coordinateAlongAxis : axis, withShapeTensor : shapeTensor, name : name)
    }
    unsafe fn coordinateAlongAxisTensor_withShapeTensor_name_(
        &self,
        axisTensor: MPSGraphTensor,
        shapeTensor: MPSGraphTensor,
        name: NSString,
    ) -> MPSGraphTensor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, coordinateAlongAxisTensor : axisTensor, withShapeTensor : shapeTensor, name : name)
    }
}
impl MPSGraph_MPSGraphTopKOps for MPSGraph {}
pub trait MPSGraph_MPSGraphTopKOps: Sized + std::ops::Deref {
    unsafe fn topKWithSourceTensor_k_name_(
        &self,
        source: MPSGraphTensor,
        k: NSUInteger,
        name: NSString,
    ) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, topKWithSourceTensor : source, k : k, name : name)
    }
    unsafe fn topKWithSourceTensor_kTensor_name_(
        &self,
        source: MPSGraphTensor,
        kTensor: MPSGraphTensor,
        name: NSString,
    ) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, topKWithSourceTensor : source, kTensor : kTensor, name : name)
    }
    unsafe fn topKWithSourceTensor_axis_k_name_(
        &self,
        source: MPSGraphTensor,
        axis: NSInteger,
        k: NSUInteger,
        name: NSString,
    ) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, topKWithSourceTensor : source, axis : axis, k : k, name : name)
    }
    unsafe fn bottomKWithSourceTensor_axis_k_name_(
        &self,
        source: MPSGraphTensor,
        axis: NSInteger,
        k: NSUInteger,
        name: NSString,
    ) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, bottomKWithSourceTensor : source, axis : axis, k : k, name : name)
    }
    unsafe fn topKWithSourceTensor_axisTensor_kTensor_name_(
        &self,
        source: MPSGraphTensor,
        axisTensor: MPSGraphTensor,
        kTensor: MPSGraphTensor,
        name: NSString,
    ) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, topKWithSourceTensor : source, axisTensor : axisTensor, kTensor : kTensor, name : name)
    }
    unsafe fn bottomKWithSourceTensor_axisTensor_kTensor_name_(
        &self,
        source: MPSGraphTensor,
        axisTensor: MPSGraphTensor,
        kTensor: MPSGraphTensor,
        name: NSString,
    ) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, bottomKWithSourceTensor : source, axisTensor : axisTensor, kTensor : kTensor, name : name)
    }
}
impl MPSGraph_MPSGraphTopKGradientOps for MPSGraph {}
pub trait MPSGraph_MPSGraphTopKGradientOps: Sized + std::ops::Deref {
    unsafe fn topKWithGradientTensor_source_k_name_(
        &self,
        gradient: MPSGraphTensor,
        source: MPSGraphTensor,
        k: NSUInteger,
        name: NSString,
    ) -> MPSGraphTensor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, topKWithGradientTensor : gradient, source : source, k : k, name : name)
    }
    unsafe fn topKWithGradientTensor_source_axis_k_name_(
        &self,
        gradient: MPSGraphTensor,
        source: MPSGraphTensor,
        axis: NSInteger,
        k: NSUInteger,
        name: NSString,
    ) -> MPSGraphTensor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, topKWithGradientTensor : gradient, source : source, axis : axis, k : k, name : name)
    }
    unsafe fn bottomKWithGradientTensor_source_axis_k_name_(
        &self,
        gradient: MPSGraphTensor,
        source: MPSGraphTensor,
        axis: NSInteger,
        k: NSUInteger,
        name: NSString,
    ) -> MPSGraphTensor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, bottomKWithGradientTensor : gradient, source : source, axis : axis, k : k, name : name)
    }
    unsafe fn topKWithGradientTensor_source_kTensor_name_(
        &self,
        gradient: MPSGraphTensor,
        source: MPSGraphTensor,
        kTensor: MPSGraphTensor,
        name: NSString,
    ) -> MPSGraphTensor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, topKWithGradientTensor : gradient, source : source, kTensor : kTensor, name : name)
    }
    unsafe fn topKWithGradientTensor_source_axisTensor_kTensor_name_(
        &self,
        gradient: MPSGraphTensor,
        source: MPSGraphTensor,
        axisTensor: MPSGraphTensor,
        kTensor: MPSGraphTensor,
        name: NSString,
    ) -> MPSGraphTensor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, topKWithGradientTensor : gradient, source : source, axisTensor : axisTensor, kTensor : kTensor, name : name)
    }
    unsafe fn bottomKWithGradientTensor_source_axisTensor_kTensor_name_(
        &self,
        gradient: MPSGraphTensor,
        source: MPSGraphTensor,
        axisTensor: MPSGraphTensor,
        kTensor: MPSGraphTensor,
        name: NSString,
    ) -> MPSGraphTensor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, bottomKWithGradientTensor : gradient, source : source, axisTensor : axisTensor, kTensor : kTensor, name : name)
    }
}

unsafe impl objc2::encode::RefEncode for MPSGraphObject {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MPSGraphObject {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MPSGraphType {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MPSGraphType {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MPSGraphShapedType {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MPSGraphShapedType {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MPSGraphDevice {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MPSGraphDevice {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MPSGraphTensor {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MPSGraphTensor {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MPSGraphTensorData {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MPSGraphTensorData {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MPSGraphOperation {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MPSGraphOperation {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MPSGraphCompilationDescriptor {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MPSGraphCompilationDescriptor {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MPSGraphExecutionDescriptor {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MPSGraphExecutionDescriptor {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MPSGraph {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MPSGraph {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MPSGraphExecutableExecutionDescriptor {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MPSGraphExecutableExecutionDescriptor {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MPSGraphExecutableSerializationDescriptor {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MPSGraphExecutableSerializationDescriptor {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MPSGraphExecutable {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MPSGraphExecutable {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MPSGraphConvolution2DOpDescriptor {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MPSGraphConvolution2DOpDescriptor {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MPSGraphConvolution3DOpDescriptor {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MPSGraphConvolution3DOpDescriptor {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MPSGraphDepthwiseConvolution2DOpDescriptor {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MPSGraphDepthwiseConvolution2DOpDescriptor {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MPSGraphDepthwiseConvolution3DOpDescriptor {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MPSGraphDepthwiseConvolution3DOpDescriptor {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MPSGraphFFTDescriptor {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MPSGraphFFTDescriptor {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MPSGraphImToColOpDescriptor {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MPSGraphImToColOpDescriptor {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MPSGraphVariableOp {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MPSGraphVariableOp {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MPSGraphPooling2DOpDescriptor {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MPSGraphPooling2DOpDescriptor {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MPSGraphPooling4DOpDescriptor {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MPSGraphPooling4DOpDescriptor {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MPSGraphRandomOpDescriptor {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MPSGraphRandomOpDescriptor {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MPSGraphSingleGateRNNDescriptor {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MPSGraphSingleGateRNNDescriptor {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MPSGraphLSTMDescriptor {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MPSGraphLSTMDescriptor {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MPSGraphGRUDescriptor {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MPSGraphGRUDescriptor {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MPSGraphCreateSparseOpDescriptor {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MPSGraphCreateSparseOpDescriptor {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MPSGraphStencilOpDescriptor {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MPSGraphStencilOpDescriptor {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
