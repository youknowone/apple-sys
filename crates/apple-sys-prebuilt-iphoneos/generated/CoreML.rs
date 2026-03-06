#[allow(unused_imports)]
use crate::objc::*;
#[allow(unused_imports)]
use crate::CoreFoundation::*;
#[allow(unused_imports)]
use crate::CoreGraphics::*;
#[allow(unused_imports)]
use crate::CoreVideo::*;
#[allow(unused_imports)]
use crate::Foundation::*;
#[allow(unused_imports)]
use crate::ImageIO::*;
#[allow(unused_imports)]
use crate::Metal::*;
#[allow(unused_imports)]
use crate::Security::*;

#[allow(unused_imports)]
use objc2::msg_send;
pub type MTLCommandBufferHandler = *mut ::std::os::raw::c_void;
pub trait PMTLCommandBuffer: Sized + std::ops::Deref {
    unsafe fn enqueue(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, enqueue)
    }
    unsafe fn commit(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, commit)
    }
    unsafe fn addScheduledHandler_(&self, block: MTLCommandBufferHandler)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, addScheduledHandler : block)
    }
    unsafe fn presentDrawable_(&self, drawable: *mut u64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, presentDrawable : drawable)
    }
    unsafe fn presentDrawable_atTime_(&self, drawable: *mut u64, presentationTime: CFTimeInterval)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, presentDrawable : drawable, atTime : presentationTime)
    }
    unsafe fn presentDrawable_afterMinimumDuration_(
        &self,
        drawable: *mut u64,
        duration: CFTimeInterval,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, presentDrawable : drawable, afterMinimumDuration : duration)
    }
    unsafe fn waitUntilScheduled(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, waitUntilScheduled)
    }
    unsafe fn addCompletedHandler_(&self, block: MTLCommandBufferHandler)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, addCompletedHandler : block)
    }
    unsafe fn waitUntilCompleted(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, waitUntilCompleted)
    }
    unsafe fn blitCommandEncoder(&self) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, blitCommandEncoder)
    }
    unsafe fn renderCommandEncoderWithDescriptor_(
        &self,
        renderPassDescriptor: MTLRenderPassDescriptor,
    ) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, renderCommandEncoderWithDescriptor : renderPassDescriptor)
    }
    unsafe fn computeCommandEncoderWithDescriptor_(
        &self,
        computePassDescriptor: MTLComputePassDescriptor,
    ) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, computeCommandEncoderWithDescriptor : computePassDescriptor)
    }
    unsafe fn blitCommandEncoderWithDescriptor_(
        &self,
        blitPassDescriptor: MTLBlitPassDescriptor,
    ) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, blitCommandEncoderWithDescriptor : blitPassDescriptor)
    }
    unsafe fn computeCommandEncoder(&self) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, computeCommandEncoder)
    }
    unsafe fn computeCommandEncoderWithDispatchType_(
        &self,
        dispatchType: MTLDispatchType,
    ) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, computeCommandEncoderWithDispatchType : dispatchType)
    }
    unsafe fn encodeWaitForEvent_value_(&self, event: *mut u64, value: u64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, encodeWaitForEvent : event, value : value)
    }
    unsafe fn encodeSignalEvent_value_(&self, event: *mut u64, value: u64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, encodeSignalEvent : event, value : value)
    }
    unsafe fn parallelRenderCommandEncoderWithDescriptor_(
        &self,
        renderPassDescriptor: MTLRenderPassDescriptor,
    ) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, parallelRenderCommandEncoderWithDescriptor : renderPassDescriptor)
    }
    unsafe fn resourceStateCommandEncoder(&self) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, resourceStateCommandEncoder)
    }
    unsafe fn resourceStateCommandEncoderWithDescriptor_(
        &self,
        resourceStatePassDescriptor: MTLResourceStatePassDescriptor,
    ) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, resourceStateCommandEncoderWithDescriptor : resourceStatePassDescriptor)
    }
    unsafe fn accelerationStructureCommandEncoder(&self) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, accelerationStructureCommandEncoder)
    }
    unsafe fn accelerationStructureCommandEncoderWithDescriptor_(
        &self,
        descriptor: MTLAccelerationStructurePassDescriptor,
    ) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, accelerationStructureCommandEncoderWithDescriptor : descriptor)
    }
    unsafe fn pushDebugGroup_(&self, string: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, pushDebugGroup : string)
    }
    unsafe fn popDebugGroup(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, popDebugGroup)
    }
    unsafe fn useResidencySet_(&self, residencySet: *mut u64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, useResidencySet : residencySet)
    }
    unsafe fn useResidencySets_count_(&self, residencySets: *const *mut u64, count: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, useResidencySets : residencySets, count : count)
    }
    unsafe fn device(&self) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, device)
    }
    unsafe fn commandQueue(&self) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, commandQueue)
    }
    unsafe fn retainedReferences(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, retainedReferences)
    }
    unsafe fn errorOptions(&self) -> MTLCommandBufferErrorOption
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, errorOptions)
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
    unsafe fn kernelStartTime(&self) -> CFTimeInterval
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, kernelStartTime)
    }
    unsafe fn kernelEndTime(&self) -> CFTimeInterval
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, kernelEndTime)
    }
    unsafe fn logs(&self) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, logs)
    }
    unsafe fn GPUStartTime(&self) -> CFTimeInterval
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, GPUStartTime)
    }
    unsafe fn GPUEndTime(&self) -> CFTimeInterval
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, GPUEndTime)
    }
    unsafe fn status(&self) -> MTLCommandBufferStatus
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, status)
    }
    unsafe fn error(&self) -> NSError
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, error)
    }
}
pub type MLFeatureType = NSInteger;
pub type MLMultiArrayDataType = NSInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MLMultiArray(pub id);
impl std::ops::Deref for MLMultiArray {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MLMultiArray {}
impl MLMultiArray {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MLMultiArray").unwrap(), alloc) })
    }
}
impl PNSSecureCoding for MLMultiArray {}
impl INSObject for MLMultiArray {}
impl PNSObject for MLMultiArray {}
impl std::convert::TryFrom<NSObject> for MLMultiArray {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<MLMultiArray, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MLMultiArray").unwrap()) };
        if is_kind_of {
            Ok(MLMultiArray(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to MLMultiArray")
        }
    }
}
impl IMLMultiArray for MLMultiArray {}
pub trait IMLMultiArray: Sized + std::ops::Deref {
    unsafe fn dataPointer(&self) -> *mut ::std::os::raw::c_void
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, dataPointer)
    }
    unsafe fn dataType(&self) -> MLMultiArrayDataType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, dataType)
    }
    unsafe fn shape(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, shape)
    }
    unsafe fn strides(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, strides)
    }
    unsafe fn count(&self) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, count)
    }
    unsafe fn pixelBuffer(&self) -> CVPixelBufferRef
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, pixelBuffer)
    }
}
impl MLMultiArray_Creation for MLMultiArray {}
pub trait MLMultiArray_Creation: Sized + std::ops::Deref {
    unsafe fn initWithShape_dataType_error_(
        &self,
        shape: NSArray,
        dataType: MLMultiArrayDataType,
        error: *mut NSError,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithShape : shape, dataType : dataType, error : error)
    }
    unsafe fn initWithShape_dataType_strides_(
        &self,
        shape: NSArray,
        dataType: MLMultiArrayDataType,
        strides: NSArray,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithShape : shape, dataType : dataType, strides : strides)
    }
    unsafe fn initWithDataPointer_shape_dataType_strides_deallocator_error_(
        &self,
        dataPointer: *mut ::std::os::raw::c_void,
        shape: NSArray,
        dataType: MLMultiArrayDataType,
        strides: NSArray,
        deallocator: *mut ::std::os::raw::c_void,
        error: *mut NSError,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithDataPointer : dataPointer, shape : shape, dataType : dataType, strides : strides, deallocator : deallocator, error : error)
    }
    unsafe fn initWithPixelBuffer_shape_(
        &self,
        pixelBuffer: CVPixelBufferRef,
        shape: NSArray,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithPixelBuffer : pixelBuffer, shape : shape)
    }
}
impl MLMultiArray_ScopedBufferAccess for MLMultiArray {}
pub trait MLMultiArray_ScopedBufferAccess: Sized + std::ops::Deref {
    unsafe fn getBytesWithHandler_(&self, handler: *mut ::std::os::raw::c_void)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, getBytesWithHandler : handler)
    }
    unsafe fn getMutableBytesWithHandler_(&self, handler: *mut ::std::os::raw::c_void)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, getMutableBytesWithHandler : handler)
    }
}
impl MLMultiArray_Concatenating for MLMultiArray {}
pub trait MLMultiArray_Concatenating: Sized + std::ops::Deref {
    unsafe fn multiArrayByConcatenatingMultiArrays_alongAxis_dataType_(
        multiArrays: NSArray,
        axis: NSInteger,
        dataType: MLMultiArrayDataType,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MLMultiArray").unwrap(), multiArrayByConcatenatingMultiArrays : multiArrays, alongAxis : axis, dataType : dataType)
    }
}
impl MLMultiArray_NSNumberDataAccess for MLMultiArray {}
pub trait MLMultiArray_NSNumberDataAccess: Sized + std::ops::Deref {
    unsafe fn objectAtIndexedSubscript_(&self, idx: NSInteger) -> NSNumber
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, objectAtIndexedSubscript : idx)
    }
    unsafe fn objectForKeyedSubscript_(&self, key: NSArray) -> NSNumber
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, objectForKeyedSubscript : key)
    }
    unsafe fn setObject_atIndexedSubscript_(&self, obj: NSNumber, idx: NSInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setObject : obj, atIndexedSubscript : idx)
    }
    unsafe fn setObject_forKeyedSubscript_(&self, obj: NSNumber, key: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setObject : obj, forKeyedSubscript : key)
    }
}
impl MLMultiArray_Transferring for MLMultiArray {}
pub trait MLMultiArray_Transferring: Sized + std::ops::Deref {
    unsafe fn transferToMultiArray_(&self, destinationMultiArray: MLMultiArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, transferToMultiArray : destinationMultiArray)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MLSequence(pub id);
impl std::ops::Deref for MLSequence {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MLSequence {}
impl MLSequence {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MLSequence").unwrap(), alloc) })
    }
}
impl PNSSecureCoding for MLSequence {}
impl INSObject for MLSequence {}
impl PNSObject for MLSequence {}
impl std::convert::TryFrom<NSObject> for MLSequence {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<MLSequence, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MLSequence").unwrap()) };
        if is_kind_of {
            Ok(MLSequence(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to MLSequence")
        }
    }
}
impl IMLSequence for MLSequence {}
pub trait IMLSequence: Sized + std::ops::Deref {
    unsafe fn type_(&self) -> MLFeatureType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, type)
    }
    unsafe fn stringValues(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, stringValues)
    }
    unsafe fn int64Values(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, int64Values)
    }
    unsafe fn emptySequenceWithType_(type_: MLFeatureType) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MLSequence").unwrap(), emptySequenceWithType : type_)
    }
    unsafe fn sequenceWithStringArray_(stringValues: NSArray) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MLSequence").unwrap(), sequenceWithStringArray : stringValues)
    }
    unsafe fn sequenceWithInt64Array_(int64Values: NSArray) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MLSequence").unwrap(), sequenceWithInt64Array : int64Values)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MLState(pub id);
impl std::ops::Deref for MLState {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MLState {}
impl MLState {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MLState").unwrap(), alloc) })
    }
}
impl INSObject for MLState {}
impl PNSObject for MLState {}
impl std::convert::TryFrom<NSObject> for MLState {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<MLState, Self::Error> {
        let is_kind_of: bool = unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MLState").unwrap()) };
        if is_kind_of {
            Ok(MLState(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to MLState")
        }
    }
}
impl IMLState for MLState {}
pub trait IMLState: Sized + std::ops::Deref {
    unsafe fn getMultiArrayForStateNamed_handler_(
        &self,
        stateName: NSString,
        handler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, getMultiArrayForStateNamed : stateName, handler : handler)
    }
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
        msg_send!(&*objc2::runtime::AnyClass::get(c"MLState").unwrap(), new)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MLFeatureValue(pub id);
impl std::ops::Deref for MLFeatureValue {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MLFeatureValue {}
impl MLFeatureValue {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MLFeatureValue").unwrap(), alloc) })
    }
}
impl PNSCopying for MLFeatureValue {}
impl PNSSecureCoding for MLFeatureValue {}
impl INSObject for MLFeatureValue {}
impl PNSObject for MLFeatureValue {}
impl std::convert::TryFrom<NSObject> for MLFeatureValue {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<MLFeatureValue, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MLFeatureValue").unwrap()) };
        if is_kind_of {
            Ok(MLFeatureValue(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to MLFeatureValue")
        }
    }
}
impl IMLFeatureValue for MLFeatureValue {}
pub trait IMLFeatureValue: Sized + std::ops::Deref {
    unsafe fn isEqualToFeatureValue_(&self, value: MLFeatureValue) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, isEqualToFeatureValue : value)
    }
    unsafe fn type_(&self) -> MLFeatureType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, type)
    }
    unsafe fn isUndefined(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isUndefined)
    }
    unsafe fn int64Value(&self) -> i64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, int64Value)
    }
    unsafe fn doubleValue(&self) -> f64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, doubleValue)
    }
    unsafe fn stringValue(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, stringValue)
    }
    unsafe fn multiArrayValue(&self) -> MLMultiArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, multiArrayValue)
    }
    unsafe fn dictionaryValue(&self) -> NSDictionary
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, dictionaryValue)
    }
    unsafe fn imageBufferValue(&self) -> CVPixelBufferRef
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, imageBufferValue)
    }
    unsafe fn sequenceValue(&self) -> MLSequence
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, sequenceValue)
    }
    unsafe fn featureValueWithInt64_(value: i64) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MLFeatureValue").unwrap(), featureValueWithInt64 : value)
    }
    unsafe fn featureValueWithDouble_(value: f64) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MLFeatureValue").unwrap(), featureValueWithDouble : value)
    }
    unsafe fn featureValueWithString_(value: NSString) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MLFeatureValue").unwrap(), featureValueWithString : value)
    }
    unsafe fn featureValueWithMultiArray_(value: MLMultiArray) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MLFeatureValue").unwrap(), featureValueWithMultiArray : value)
    }
    unsafe fn featureValueWithPixelBuffer_(value: CVPixelBufferRef) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MLFeatureValue").unwrap(), featureValueWithPixelBuffer : value)
    }
    unsafe fn featureValueWithSequence_(sequence: MLSequence) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MLFeatureValue").unwrap(), featureValueWithSequence : sequence)
    }
    unsafe fn undefinedFeatureValueWithType_(type_: MLFeatureType) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MLFeatureValue").unwrap(), undefinedFeatureValueWithType : type_)
    }
    unsafe fn featureValueWithDictionary_error_(
        value: NSDictionary,
        error: *mut NSError,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MLFeatureValue").unwrap(), featureValueWithDictionary : value, error : error)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MLImageSize(pub id);
impl std::ops::Deref for MLImageSize {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MLImageSize {}
impl MLImageSize {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MLImageSize").unwrap(), alloc) })
    }
}
impl PNSSecureCoding for MLImageSize {}
impl INSObject for MLImageSize {}
impl PNSObject for MLImageSize {}
impl std::convert::TryFrom<NSObject> for MLImageSize {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<MLImageSize, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MLImageSize").unwrap()) };
        if is_kind_of {
            Ok(MLImageSize(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to MLImageSize")
        }
    }
}
impl IMLImageSize for MLImageSize {}
pub trait IMLImageSize: Sized + std::ops::Deref {
    unsafe fn pixelsWide(&self) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, pixelsWide)
    }
    unsafe fn pixelsHigh(&self) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, pixelsHigh)
    }
}
pub type MLImageSizeConstraintType = NSInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MLImageSizeConstraint(pub id);
impl std::ops::Deref for MLImageSizeConstraint {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MLImageSizeConstraint {}
impl MLImageSizeConstraint {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MLImageSizeConstraint").unwrap(), alloc) })
    }
}
impl PNSSecureCoding for MLImageSizeConstraint {}
impl INSObject for MLImageSizeConstraint {}
impl PNSObject for MLImageSizeConstraint {}
impl std::convert::TryFrom<NSObject> for MLImageSizeConstraint {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<MLImageSizeConstraint, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MLImageSizeConstraint").unwrap()) };
        if is_kind_of {
            Ok(MLImageSizeConstraint(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to MLImageSizeConstraint")
        }
    }
}
impl IMLImageSizeConstraint for MLImageSizeConstraint {}
pub trait IMLImageSizeConstraint: Sized + std::ops::Deref {
    unsafe fn type_(&self) -> MLImageSizeConstraintType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, type)
    }
    unsafe fn pixelsWideRange(&self) -> NSRange
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, pixelsWideRange)
    }
    unsafe fn pixelsHighRange(&self) -> NSRange
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, pixelsHighRange)
    }
    unsafe fn enumeratedImageSizes(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, enumeratedImageSizes)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MLImageConstraint(pub id);
impl std::ops::Deref for MLImageConstraint {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MLImageConstraint {}
impl MLImageConstraint {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MLImageConstraint").unwrap(), alloc) })
    }
}
impl PNSSecureCoding for MLImageConstraint {}
impl INSObject for MLImageConstraint {}
impl PNSObject for MLImageConstraint {}
impl std::convert::TryFrom<NSObject> for MLImageConstraint {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<MLImageConstraint, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MLImageConstraint").unwrap()) };
        if is_kind_of {
            Ok(MLImageConstraint(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to MLImageConstraint")
        }
    }
}
impl IMLImageConstraint for MLImageConstraint {}
pub trait IMLImageConstraint: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn pixelsHigh(&self) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, pixelsHigh)
    }
    unsafe fn pixelsWide(&self) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, pixelsWide)
    }
    unsafe fn pixelFormatType(&self) -> OSType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, pixelFormatType)
    }
    unsafe fn sizeConstraint(&self) -> MLImageSizeConstraint
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, sizeConstraint)
    }
}
pub type MLFeatureValueImageOption = NSString;
impl MLFeatureValue_MLImageConversion for MLFeatureValue {}
pub trait MLFeatureValue_MLImageConversion: Sized + std::ops::Deref {
    unsafe fn featureValueWithImageAtURL_pixelsWide_pixelsHigh_pixelFormatType_options_error_(
        url: NSURL,
        pixelsWide: NSInteger,
        pixelsHigh: NSInteger,
        pixelFormatType: OSType,
        options: NSDictionary,
        error: *mut NSError,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MLFeatureValue").unwrap(), featureValueWithImageAtURL : url, pixelsWide : pixelsWide, pixelsHigh : pixelsHigh, pixelFormatType : pixelFormatType, options : options, error : error)
    }
    unsafe fn featureValueWithImageAtURL_constraint_options_error_(
        url: NSURL,
        constraint: MLImageConstraint,
        options: NSDictionary,
        error: *mut NSError,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MLFeatureValue").unwrap(), featureValueWithImageAtURL : url, constraint : constraint, options : options, error : error)
    }
    unsafe fn featureValueWithCGImage_pixelsWide_pixelsHigh_pixelFormatType_options_error_(
        cgImage: CGImageRef,
        pixelsWide: NSInteger,
        pixelsHigh: NSInteger,
        pixelFormatType: OSType,
        options: NSDictionary,
        error: *mut NSError,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MLFeatureValue").unwrap(), featureValueWithCGImage : cgImage, pixelsWide : pixelsWide, pixelsHigh : pixelsHigh, pixelFormatType : pixelFormatType, options : options, error : error)
    }
    unsafe fn featureValueWithCGImage_constraint_options_error_(
        cgImage: CGImageRef,
        constraint: MLImageConstraint,
        options: NSDictionary,
        error: *mut NSError,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MLFeatureValue").unwrap(), featureValueWithCGImage : cgImage, constraint : constraint, options : options, error : error)
    }
    unsafe fn featureValueWithImageAtURL_orientation_pixelsWide_pixelsHigh_pixelFormatType_options_error_(
        url: NSURL,
        orientation: CGImagePropertyOrientation,
        pixelsWide: NSInteger,
        pixelsHigh: NSInteger,
        pixelFormatType: OSType,
        options: NSDictionary,
        error: *mut NSError,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MLFeatureValue").unwrap(), featureValueWithImageAtURL : url, orientation : orientation, pixelsWide : pixelsWide, pixelsHigh : pixelsHigh, pixelFormatType : pixelFormatType, options : options, error : error)
    }
    unsafe fn featureValueWithImageAtURL_orientation_constraint_options_error_(
        url: NSURL,
        orientation: CGImagePropertyOrientation,
        constraint: MLImageConstraint,
        options: NSDictionary,
        error: *mut NSError,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MLFeatureValue").unwrap(), featureValueWithImageAtURL : url, orientation : orientation, constraint : constraint, options : options, error : error)
    }
    unsafe fn featureValueWithCGImage_orientation_pixelsWide_pixelsHigh_pixelFormatType_options_error_(
        cgImage: CGImageRef,
        orientation: CGImagePropertyOrientation,
        pixelsWide: NSInteger,
        pixelsHigh: NSInteger,
        pixelFormatType: OSType,
        options: NSDictionary,
        error: *mut NSError,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MLFeatureValue").unwrap(), featureValueWithCGImage : cgImage, orientation : orientation, pixelsWide : pixelsWide, pixelsHigh : pixelsHigh, pixelFormatType : pixelFormatType, options : options, error : error)
    }
    unsafe fn featureValueWithCGImage_orientation_constraint_options_error_(
        cgImage: CGImageRef,
        orientation: CGImagePropertyOrientation,
        constraint: MLImageConstraint,
        options: NSDictionary,
        error: *mut NSError,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MLFeatureValue").unwrap(), featureValueWithCGImage : cgImage, orientation : orientation, constraint : constraint, options : options, error : error)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MLDictionaryConstraint(pub id);
impl std::ops::Deref for MLDictionaryConstraint {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MLDictionaryConstraint {}
impl MLDictionaryConstraint {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MLDictionaryConstraint").unwrap(), alloc) })
    }
}
impl PNSSecureCoding for MLDictionaryConstraint {}
impl INSObject for MLDictionaryConstraint {}
impl PNSObject for MLDictionaryConstraint {}
impl std::convert::TryFrom<NSObject> for MLDictionaryConstraint {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<MLDictionaryConstraint, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MLDictionaryConstraint").unwrap()) };
        if is_kind_of {
            Ok(MLDictionaryConstraint(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to MLDictionaryConstraint")
        }
    }
}
impl IMLDictionaryConstraint for MLDictionaryConstraint {}
pub trait IMLDictionaryConstraint: Sized + std::ops::Deref {
    unsafe fn keyType(&self) -> MLFeatureType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, keyType)
    }
}
pub type MLMultiArrayShapeConstraintType = NSInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MLMultiArrayShapeConstraint(pub id);
impl std::ops::Deref for MLMultiArrayShapeConstraint {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MLMultiArrayShapeConstraint {}
impl MLMultiArrayShapeConstraint {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MLMultiArrayShapeConstraint").unwrap(), alloc) })
    }
}
impl PNSSecureCoding for MLMultiArrayShapeConstraint {}
impl INSObject for MLMultiArrayShapeConstraint {}
impl PNSObject for MLMultiArrayShapeConstraint {}
impl std::convert::TryFrom<NSObject> for MLMultiArrayShapeConstraint {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<MLMultiArrayShapeConstraint, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MLMultiArrayShapeConstraint").unwrap()) };
        if is_kind_of {
            Ok(MLMultiArrayShapeConstraint(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to MLMultiArrayShapeConstraint")
        }
    }
}
impl IMLMultiArrayShapeConstraint for MLMultiArrayShapeConstraint {}
pub trait IMLMultiArrayShapeConstraint: Sized + std::ops::Deref {
    unsafe fn type_(&self) -> MLMultiArrayShapeConstraintType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, type)
    }
    unsafe fn sizeRangeForDimension(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, sizeRangeForDimension)
    }
    unsafe fn enumeratedShapes(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, enumeratedShapes)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MLMultiArrayConstraint(pub id);
impl std::ops::Deref for MLMultiArrayConstraint {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MLMultiArrayConstraint {}
impl MLMultiArrayConstraint {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MLMultiArrayConstraint").unwrap(), alloc) })
    }
}
impl PNSSecureCoding for MLMultiArrayConstraint {}
impl INSObject for MLMultiArrayConstraint {}
impl PNSObject for MLMultiArrayConstraint {}
impl std::convert::TryFrom<NSObject> for MLMultiArrayConstraint {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<MLMultiArrayConstraint, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MLMultiArrayConstraint").unwrap()) };
        if is_kind_of {
            Ok(MLMultiArrayConstraint(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to MLMultiArrayConstraint")
        }
    }
}
impl IMLMultiArrayConstraint for MLMultiArrayConstraint {}
pub trait IMLMultiArrayConstraint: Sized + std::ops::Deref {
    unsafe fn shape(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, shape)
    }
    unsafe fn dataType(&self) -> MLMultiArrayDataType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, dataType)
    }
    unsafe fn shapeConstraint(&self) -> MLMultiArrayShapeConstraint
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, shapeConstraint)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MLSequenceConstraint(pub id);
impl std::ops::Deref for MLSequenceConstraint {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MLSequenceConstraint {}
impl MLSequenceConstraint {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MLSequenceConstraint").unwrap(), alloc) })
    }
}
impl PNSSecureCoding for MLSequenceConstraint {}
impl INSObject for MLSequenceConstraint {}
impl PNSObject for MLSequenceConstraint {}
impl std::convert::TryFrom<NSObject> for MLSequenceConstraint {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<MLSequenceConstraint, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MLSequenceConstraint").unwrap()) };
        if is_kind_of {
            Ok(MLSequenceConstraint(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to MLSequenceConstraint")
        }
    }
}
impl IMLSequenceConstraint for MLSequenceConstraint {}
pub trait IMLSequenceConstraint: Sized + std::ops::Deref {
    unsafe fn valueDescription(&self) -> MLFeatureDescription
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, valueDescription)
    }
    unsafe fn countRange(&self) -> NSRange
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, countRange)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MLStateConstraint(pub id);
impl std::ops::Deref for MLStateConstraint {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MLStateConstraint {}
impl MLStateConstraint {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MLStateConstraint").unwrap(), alloc) })
    }
}
impl PNSSecureCoding for MLStateConstraint {}
impl INSObject for MLStateConstraint {}
impl PNSObject for MLStateConstraint {}
impl std::convert::TryFrom<NSObject> for MLStateConstraint {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<MLStateConstraint, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MLStateConstraint").unwrap()) };
        if is_kind_of {
            Ok(MLStateConstraint(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to MLStateConstraint")
        }
    }
}
impl IMLStateConstraint for MLStateConstraint {}
pub trait IMLStateConstraint: Sized + std::ops::Deref {
    unsafe fn bufferShape(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, bufferShape)
    }
    unsafe fn dataType(&self) -> MLMultiArrayDataType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, dataType)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MLFeatureDescription(pub id);
impl std::ops::Deref for MLFeatureDescription {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MLFeatureDescription {}
impl MLFeatureDescription {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MLFeatureDescription").unwrap(), alloc) })
    }
}
impl PNSCopying for MLFeatureDescription {}
impl PNSSecureCoding for MLFeatureDescription {}
impl INSObject for MLFeatureDescription {}
impl PNSObject for MLFeatureDescription {}
impl std::convert::TryFrom<NSObject> for MLFeatureDescription {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<MLFeatureDescription, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MLFeatureDescription").unwrap()) };
        if is_kind_of {
            Ok(MLFeatureDescription(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to MLFeatureDescription")
        }
    }
}
impl IMLFeatureDescription for MLFeatureDescription {}
pub trait IMLFeatureDescription: Sized + std::ops::Deref {
    unsafe fn isAllowedValue_(&self, value: MLFeatureValue) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, isAllowedValue : value)
    }
    unsafe fn name(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, name)
    }
    unsafe fn type_(&self) -> MLFeatureType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, type)
    }
    unsafe fn isOptional(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isOptional)
    }
}
impl MLFeatureDescription_MLFeatureValueConstraints for MLFeatureDescription {}
pub trait MLFeatureDescription_MLFeatureValueConstraints: Sized + std::ops::Deref {
    unsafe fn multiArrayConstraint(&self) -> MLMultiArrayConstraint
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, multiArrayConstraint)
    }
    unsafe fn imageConstraint(&self) -> MLImageConstraint
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, imageConstraint)
    }
    unsafe fn dictionaryConstraint(&self) -> MLDictionaryConstraint
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, dictionaryConstraint)
    }
    unsafe fn sequenceConstraint(&self) -> MLSequenceConstraint
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, sequenceConstraint)
    }
    unsafe fn stateConstraint(&self) -> MLStateConstraint
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, stateConstraint)
    }
}
pub trait PMLFeatureProvider: Sized + std::ops::Deref {
    unsafe fn featureValueForName_(&self, featureName: NSString) -> MLFeatureValue
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, featureValueForName : featureName)
    }
    unsafe fn featureNames(&self) -> NSSet
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, featureNames)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MLDictionaryFeatureProvider(pub id);
impl std::ops::Deref for MLDictionaryFeatureProvider {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MLDictionaryFeatureProvider {}
impl MLDictionaryFeatureProvider {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MLDictionaryFeatureProvider").unwrap(), alloc) })
    }
}
impl PMLFeatureProvider for MLDictionaryFeatureProvider {}
impl PNSFastEnumeration for MLDictionaryFeatureProvider {}
impl PNSSecureCoding for MLDictionaryFeatureProvider {}
impl INSObject for MLDictionaryFeatureProvider {}
impl PNSObject for MLDictionaryFeatureProvider {}
impl std::convert::TryFrom<NSObject> for MLDictionaryFeatureProvider {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<MLDictionaryFeatureProvider, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MLDictionaryFeatureProvider").unwrap()) };
        if is_kind_of {
            Ok(MLDictionaryFeatureProvider(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to MLDictionaryFeatureProvider")
        }
    }
}
impl IMLDictionaryFeatureProvider for MLDictionaryFeatureProvider {}
pub trait IMLDictionaryFeatureProvider: Sized + std::ops::Deref {
    unsafe fn initWithDictionary_error_(
        &self,
        dictionary: NSDictionary,
        error: *mut NSError,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithDictionary : dictionary, error : error)
    }
    unsafe fn objectForKeyedSubscript_(&self, featureName: NSString) -> MLFeatureValue
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, objectForKeyedSubscript : featureName)
    }
    unsafe fn dictionary(&self) -> NSDictionary
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, dictionary)
    }
}
pub trait PMLBatchProvider: Sized + std::ops::Deref {
    unsafe fn featuresAtIndex_(&self, index: NSInteger) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, featuresAtIndex : index)
    }
    unsafe fn count(&self) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, count)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MLArrayBatchProvider(pub id);
impl std::ops::Deref for MLArrayBatchProvider {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MLArrayBatchProvider {}
impl MLArrayBatchProvider {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MLArrayBatchProvider").unwrap(), alloc) })
    }
}
impl PMLBatchProvider for MLArrayBatchProvider {}
impl INSObject for MLArrayBatchProvider {}
impl PNSObject for MLArrayBatchProvider {}
impl std::convert::TryFrom<NSObject> for MLArrayBatchProvider {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<MLArrayBatchProvider, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MLArrayBatchProvider").unwrap()) };
        if is_kind_of {
            Ok(MLArrayBatchProvider(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to MLArrayBatchProvider")
        }
    }
}
impl IMLArrayBatchProvider for MLArrayBatchProvider {}
pub trait IMLArrayBatchProvider: Sized + std::ops::Deref {
    unsafe fn initWithFeatureProviderArray_(&self, array: NSArray) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithFeatureProviderArray : array)
    }
    unsafe fn initWithDictionary_error_(
        &self,
        dictionary: NSDictionary,
        error: *mut NSError,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithDictionary : dictionary, error : error)
    }
    unsafe fn array(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, array)
    }
}
pub type MLModelMetadataKey = NSString;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MLModelDescription(pub id);
impl std::ops::Deref for MLModelDescription {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MLModelDescription {}
impl MLModelDescription {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MLModelDescription").unwrap(), alloc) })
    }
}
impl PNSSecureCoding for MLModelDescription {}
impl INSObject for MLModelDescription {}
impl PNSObject for MLModelDescription {}
impl std::convert::TryFrom<NSObject> for MLModelDescription {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<MLModelDescription, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MLModelDescription").unwrap()) };
        if is_kind_of {
            Ok(MLModelDescription(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to MLModelDescription")
        }
    }
}
impl IMLModelDescription for MLModelDescription {}
pub trait IMLModelDescription: Sized + std::ops::Deref {
    unsafe fn inputDescriptionsByName(&self) -> NSDictionary
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inputDescriptionsByName)
    }
    unsafe fn outputDescriptionsByName(&self) -> NSDictionary
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, outputDescriptionsByName)
    }
    unsafe fn stateDescriptionsByName(&self) -> NSDictionary
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, stateDescriptionsByName)
    }
    unsafe fn predictedFeatureName(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, predictedFeatureName)
    }
    unsafe fn predictedProbabilitiesName(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, predictedProbabilitiesName)
    }
    unsafe fn metadata(&self) -> NSDictionary
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, metadata)
    }
    unsafe fn classLabels(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, classLabels)
    }
}
impl MLModelDescription_MLUpdateAdditions for MLModelDescription {}
pub trait MLModelDescription_MLUpdateAdditions: Sized + std::ops::Deref {
    unsafe fn isUpdatable(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isUpdatable)
    }
    unsafe fn trainingInputDescriptionsByName(&self) -> NSDictionary
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, trainingInputDescriptionsByName)
    }
}
impl MLModelDescription_MLParameters for MLModelDescription {}
pub trait MLModelDescription_MLParameters: Sized + std::ops::Deref {
    unsafe fn parameterDescriptionsByKey(&self) -> NSDictionary
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, parameterDescriptionsByKey)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MLPredictionOptions(pub id);
impl std::ops::Deref for MLPredictionOptions {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MLPredictionOptions {}
impl MLPredictionOptions {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MLPredictionOptions").unwrap(), alloc) })
    }
}
impl INSObject for MLPredictionOptions {}
impl PNSObject for MLPredictionOptions {}
impl std::convert::TryFrom<NSObject> for MLPredictionOptions {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<MLPredictionOptions, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MLPredictionOptions").unwrap()) };
        if is_kind_of {
            Ok(MLPredictionOptions(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to MLPredictionOptions")
        }
    }
}
impl IMLPredictionOptions for MLPredictionOptions {}
pub trait IMLPredictionOptions: Sized + std::ops::Deref {
    unsafe fn usesCPUOnly(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, usesCPUOnly)
    }
    unsafe fn setUsesCPUOnly_(&self, usesCPUOnly: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setUsesCPUOnly : usesCPUOnly)
    }
    unsafe fn outputBackings(&self) -> NSDictionary
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, outputBackings)
    }
    unsafe fn setOutputBackings_(&self, outputBackings: NSDictionary)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setOutputBackings : outputBackings)
    }
}
pub type MLReshapeFrequencyHint = NSInteger;
pub type MLSpecializationStrategy = NSInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MLOptimizationHints(pub id);
impl std::ops::Deref for MLOptimizationHints {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MLOptimizationHints {}
impl MLOptimizationHints {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MLOptimizationHints").unwrap(), alloc) })
    }
}
impl PNSCopying for MLOptimizationHints {}
impl PNSSecureCoding for MLOptimizationHints {}
impl INSObject for MLOptimizationHints {}
impl PNSObject for MLOptimizationHints {}
impl std::convert::TryFrom<NSObject> for MLOptimizationHints {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<MLOptimizationHints, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MLOptimizationHints").unwrap()) };
        if is_kind_of {
            Ok(MLOptimizationHints(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to MLOptimizationHints")
        }
    }
}
impl IMLOptimizationHints for MLOptimizationHints {}
pub trait IMLOptimizationHints: Sized + std::ops::Deref {
    unsafe fn reshapeFrequency(&self) -> MLReshapeFrequencyHint
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, reshapeFrequency)
    }
    unsafe fn setReshapeFrequency_(&self, reshapeFrequency: MLReshapeFrequencyHint)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setReshapeFrequency : reshapeFrequency)
    }
    unsafe fn specializationStrategy(&self) -> MLSpecializationStrategy
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, specializationStrategy)
    }
    unsafe fn setSpecializationStrategy_(&self, specializationStrategy: MLSpecializationStrategy)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSpecializationStrategy : specializationStrategy)
    }
}
pub type MLComputeUnits = NSInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MLModelConfiguration(pub id);
impl std::ops::Deref for MLModelConfiguration {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MLModelConfiguration {}
impl MLModelConfiguration {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MLModelConfiguration").unwrap(), alloc) })
    }
}
impl PNSCopying for MLModelConfiguration {}
impl PNSSecureCoding for MLModelConfiguration {}
impl INSObject for MLModelConfiguration {}
impl PNSObject for MLModelConfiguration {}
impl std::convert::TryFrom<NSObject> for MLModelConfiguration {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<MLModelConfiguration, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MLModelConfiguration").unwrap()) };
        if is_kind_of {
            Ok(MLModelConfiguration(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to MLModelConfiguration")
        }
    }
}
impl IMLModelConfiguration for MLModelConfiguration {}
pub trait IMLModelConfiguration: Sized + std::ops::Deref {
    unsafe fn modelDisplayName(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, modelDisplayName)
    }
    unsafe fn setModelDisplayName_(&self, modelDisplayName: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setModelDisplayName : modelDisplayName)
    }
    unsafe fn computeUnits(&self) -> MLComputeUnits
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, computeUnits)
    }
    unsafe fn setComputeUnits_(&self, computeUnits: MLComputeUnits)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setComputeUnits : computeUnits)
    }
    unsafe fn optimizationHints(&self) -> MLOptimizationHints
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, optimizationHints)
    }
    unsafe fn setOptimizationHints_(&self, optimizationHints: MLOptimizationHints)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setOptimizationHints : optimizationHints)
    }
}
impl MLModelConfiguration_MLGPUConfigurationOptions for MLModelConfiguration {}
pub trait MLModelConfiguration_MLGPUConfigurationOptions: Sized + std::ops::Deref {
    unsafe fn allowLowPrecisionAccumulationOnGPU(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, allowLowPrecisionAccumulationOnGPU)
    }
    unsafe fn setAllowLowPrecisionAccumulationOnGPU_(
        &self,
        allowLowPrecisionAccumulationOnGPU: BOOL,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAllowLowPrecisionAccumulationOnGPU : allowLowPrecisionAccumulationOnGPU)
    }
    unsafe fn preferredMetalDevice(&self) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, preferredMetalDevice)
    }
    unsafe fn setPreferredMetalDevice_(&self, preferredMetalDevice: *mut u64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPreferredMetalDevice : preferredMetalDevice)
    }
}
impl MLModelConfiguration_MLModelParameterAdditions for MLModelConfiguration {}
pub trait MLModelConfiguration_MLModelParameterAdditions: Sized + std::ops::Deref {
    unsafe fn parameters(&self) -> NSDictionary
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, parameters)
    }
    unsafe fn setParameters_(&self, parameters: NSDictionary)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setParameters : parameters)
    }
}
impl MLModelConfiguration_MultiFunctions for MLModelConfiguration {}
pub trait MLModelConfiguration_MultiFunctions: Sized + std::ops::Deref {
    unsafe fn functionName(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, functionName)
    }
    unsafe fn setFunctionName_(&self, functionName: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setFunctionName : functionName)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MLModelAsset(pub id);
impl std::ops::Deref for MLModelAsset {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MLModelAsset {}
impl MLModelAsset {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MLModelAsset").unwrap(), alloc) })
    }
}
impl INSObject for MLModelAsset {}
impl PNSObject for MLModelAsset {}
impl std::convert::TryFrom<NSObject> for MLModelAsset {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<MLModelAsset, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MLModelAsset").unwrap()) };
        if is_kind_of {
            Ok(MLModelAsset(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to MLModelAsset")
        }
    }
}
impl IMLModelAsset for MLModelAsset {}
pub trait IMLModelAsset: Sized + std::ops::Deref {
    unsafe fn modelDescriptionWithCompletionHandler_(&self, handler: *mut ::std::os::raw::c_void)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, modelDescriptionWithCompletionHandler : handler)
    }
    unsafe fn modelDescriptionOfFunctionNamed_completionHandler_(
        &self,
        functionName: NSString,
        handler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, modelDescriptionOfFunctionNamed : functionName, completionHandler : handler)
    }
    unsafe fn functionNamesWithCompletionHandler_(&self, handler: *mut ::std::os::raw::c_void)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, functionNamesWithCompletionHandler : handler)
    }
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn modelAssetWithSpecificationData_error_(
        specificationData: NSData,
        error: *mut NSError,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MLModelAsset").unwrap(), modelAssetWithSpecificationData : specificationData, error : error)
    }
    unsafe fn modelAssetWithSpecificationData_blobMapping_error_(
        specificationData: NSData,
        blobMapping: NSDictionary,
        error: *mut NSError,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MLModelAsset").unwrap(), modelAssetWithSpecificationData : specificationData, blobMapping : blobMapping, error : error)
    }
    unsafe fn modelAssetWithURL_error_(compiledModelURL: NSURL, error: *mut NSError) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MLModelAsset").unwrap(), modelAssetWithURL : compiledModelURL, error : error)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MLModelAsset").unwrap(), new)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MLModel(pub id);
impl std::ops::Deref for MLModel {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MLModel {}
impl MLModel {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MLModel").unwrap(), alloc) })
    }
}
impl INSObject for MLModel {}
impl PNSObject for MLModel {}
impl std::convert::TryFrom<NSObject> for MLModel {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<MLModel, Self::Error> {
        let is_kind_of: bool = unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MLModel").unwrap()) };
        if is_kind_of {
            Ok(MLModel(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to MLModel")
        }
    }
}
impl IMLModel for MLModel {}
pub trait IMLModel: Sized + std::ops::Deref {
    unsafe fn predictionFromFeatures_error_(&self, input: *mut u64, error: *mut NSError) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, predictionFromFeatures : input, error : error)
    }
    unsafe fn predictionFromFeatures_options_error_(
        &self,
        input: *mut u64,
        options: MLPredictionOptions,
        error: *mut NSError,
    ) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, predictionFromFeatures : input, options : options, error : error)
    }
    unsafe fn predictionFromFeatures_completionHandler_(
        &self,
        input: *mut u64,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, predictionFromFeatures : input, completionHandler : completionHandler)
    }
    unsafe fn predictionFromFeatures_options_completionHandler_(
        &self,
        input: *mut u64,
        options: MLPredictionOptions,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, predictionFromFeatures : input, options : options, completionHandler : completionHandler)
    }
    unsafe fn predictionsFromBatch_error_(
        &self,
        inputBatch: *mut u64,
        error: *mut NSError,
    ) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, predictionsFromBatch : inputBatch, error : error)
    }
    unsafe fn predictionsFromBatch_options_error_(
        &self,
        inputBatch: *mut u64,
        options: MLPredictionOptions,
        error: *mut NSError,
    ) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, predictionsFromBatch : inputBatch, options : options, error : error)
    }
    unsafe fn parameterValueForKey_error_(&self, key: MLParameterKey, error: *mut NSError) -> id
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, parameterValueForKey : key, error : error)
    }
    unsafe fn modelDescription(&self) -> MLModelDescription
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, modelDescription)
    }
    unsafe fn configuration(&self) -> MLModelConfiguration
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, configuration)
    }
    unsafe fn modelWithContentsOfURL_error_(url: NSURL, error: *mut NSError) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MLModel").unwrap(), modelWithContentsOfURL : url, error : error)
    }
    unsafe fn modelWithContentsOfURL_configuration_error_(
        url: NSURL,
        configuration: MLModelConfiguration,
        error: *mut NSError,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MLModel").unwrap(), modelWithContentsOfURL : url, configuration : configuration, error : error)
    }
    unsafe fn loadContentsOfURL_configuration_completionHandler_(
        url: NSURL,
        configuration: MLModelConfiguration,
        handler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MLModel").unwrap(), loadContentsOfURL : url, configuration : configuration, completionHandler : handler)
    }
    unsafe fn loadModelAsset_configuration_completionHandler_(
        asset: MLModelAsset,
        configuration: MLModelConfiguration,
        handler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MLModel").unwrap(), loadModelAsset : asset, configuration : configuration, completionHandler : handler)
    }
}
impl MLModel_MLModelCompilation for MLModel {}
pub trait MLModel_MLModelCompilation: Sized + std::ops::Deref {
    unsafe fn compileModelAtURL_error_(modelURL: NSURL, error: *mut NSError) -> NSURL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MLModel").unwrap(), compileModelAtURL : modelURL, error : error)
    }
    unsafe fn compileModelAtURL_completionHandler_(
        modelURL: NSURL,
        handler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MLModel").unwrap(), compileModelAtURL : modelURL, completionHandler : handler)
    }
}
impl MLModel_MLComputeDevice for MLModel {}
pub trait MLModel_MLComputeDevice: Sized + std::ops::Deref {
    unsafe fn availableComputeDevices() -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MLModel").unwrap(), availableComputeDevices)
    }
}
impl MLModel_MLState for MLModel {}
pub trait MLModel_MLState: Sized + std::ops::Deref {
    unsafe fn newState(&self) -> MLState
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, newState)
    }
    unsafe fn predictionFromFeatures_usingState_error_(
        &self,
        inputFeatures: *mut u64,
        state: MLState,
        error: *mut NSError,
    ) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, predictionFromFeatures : inputFeatures, usingState : state, error : error)
    }
    unsafe fn predictionFromFeatures_usingState_options_error_(
        &self,
        inputFeatures: *mut u64,
        state: MLState,
        options: MLPredictionOptions,
        error: *mut NSError,
    ) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, predictionFromFeatures : inputFeatures, usingState : state, options : options, error : error)
    }
    unsafe fn predictionFromFeatures_usingState_options_completionHandler_(
        &self,
        inputFeatures: *mut u64,
        state: MLState,
        options: MLPredictionOptions,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, predictionFromFeatures : inputFeatures, usingState : state, options : options, completionHandler : completionHandler)
    }
}
pub type MLModelError = NSInteger;
pub trait PMLCustomLayer: Sized + std::ops::Deref {
    unsafe fn initWithParameterDictionary_error_(
        &self,
        parameters: NSDictionary,
        error: *mut NSError,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithParameterDictionary : parameters, error : error)
    }
    unsafe fn setWeightData_error_(&self, weights: NSArray, error: *mut NSError) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setWeightData : weights, error : error)
    }
    unsafe fn outputShapesForInputShapes_error_(
        &self,
        inputShapes: NSArray,
        error: *mut NSError,
    ) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, outputShapesForInputShapes : inputShapes, error : error)
    }
    unsafe fn evaluateOnCPUWithInputs_outputs_error_(
        &self,
        inputs: NSArray,
        outputs: NSArray,
        error: *mut NSError,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, evaluateOnCPUWithInputs : inputs, outputs : outputs, error : error)
    }
    unsafe fn encodeToCommandBuffer_inputs_outputs_error_(
        &self,
        commandBuffer: *mut u64,
        inputs: NSArray,
        outputs: NSArray,
        error: *mut NSError,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, encodeToCommandBuffer : commandBuffer, inputs : inputs, outputs : outputs, error : error)
    }
}
pub trait PMLCustomModel: Sized + std::ops::Deref {
    unsafe fn initWithModelDescription_parameterDictionary_error_(
        &self,
        modelDescription: MLModelDescription,
        parameters: NSDictionary,
        error: *mut NSError,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithModelDescription : modelDescription, parameterDictionary : parameters, error : error)
    }
    unsafe fn predictionFromFeatures_options_error_(
        &self,
        input: *mut u64,
        options: MLPredictionOptions,
        error: *mut NSError,
    ) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, predictionFromFeatures : input, options : options, error : error)
    }
    unsafe fn predictionsFromBatch_options_error_(
        &self,
        inputBatch: *mut u64,
        options: MLPredictionOptions,
        error: *mut NSError,
    ) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, predictionsFromBatch : inputBatch, options : options, error : error)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MLKey(pub id);
impl std::ops::Deref for MLKey {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MLKey {}
impl MLKey {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MLKey").unwrap(), alloc) })
    }
}
impl PNSCopying for MLKey {}
impl PNSSecureCoding for MLKey {}
impl INSObject for MLKey {}
impl PNSObject for MLKey {}
impl std::convert::TryFrom<NSObject> for MLKey {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<MLKey, Self::Error> {
        let is_kind_of: bool = unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MLKey").unwrap()) };
        if is_kind_of {
            Ok(MLKey(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to MLKey")
        }
    }
}
impl IMLKey for MLKey {}
pub trait IMLKey: Sized + std::ops::Deref {
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
    unsafe fn scope(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, scope)
    }
    unsafe fn new() -> id
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MLKey").unwrap(), new)
    }
}
pub type MLTaskState = NSInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MLTask(pub id);
impl std::ops::Deref for MLTask {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MLTask {}
impl MLTask {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MLTask").unwrap(), alloc) })
    }
}
impl INSObject for MLTask {}
impl PNSObject for MLTask {}
impl std::convert::TryFrom<NSObject> for MLTask {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<MLTask, Self::Error> {
        let is_kind_of: bool = unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MLTask").unwrap()) };
        if is_kind_of {
            Ok(MLTask(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to MLTask")
        }
    }
}
impl IMLTask for MLTask {}
pub trait IMLTask: Sized + std::ops::Deref {
    unsafe fn resume(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, resume)
    }
    unsafe fn cancel(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, cancel)
    }
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn taskIdentifier(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, taskIdentifier)
    }
    unsafe fn state(&self) -> MLTaskState
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, state)
    }
    unsafe fn error(&self) -> NSError
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, error)
    }
    unsafe fn new() -> id
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MLTask").unwrap(), new)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MLUpdateTask(pub id);
impl std::ops::Deref for MLUpdateTask {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MLUpdateTask {}
impl MLUpdateTask {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MLUpdateTask").unwrap(), alloc) })
    }
}
impl IMLTask for MLUpdateTask {}
impl From<MLUpdateTask> for MLTask {
    fn from(child: MLUpdateTask) -> MLTask {
        MLTask(child.0)
    }
}
impl std::convert::TryFrom<MLTask> for MLUpdateTask {
    type Error = &'static str;
    fn try_from(parent: MLTask) -> Result<MLUpdateTask, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MLUpdateTask").unwrap()) };
        if is_kind_of {
            Ok(MLUpdateTask(parent.0))
        } else {
            Err("This MLTask cannot be downcasted to MLUpdateTask")
        }
    }
}
impl INSObject for MLUpdateTask {}
impl PNSObject for MLUpdateTask {}
impl IMLUpdateTask for MLUpdateTask {}
pub trait IMLUpdateTask: Sized + std::ops::Deref {
    unsafe fn resumeWithParameters_(&self, updateParameters: NSDictionary)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, resumeWithParameters : updateParameters)
    }
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn updateTaskForModelAtURL_trainingData_configuration_completionHandler_error_(
        modelURL: NSURL,
        trainingData: *mut u64,
        configuration: MLModelConfiguration,
        completionHandler: *mut ::std::os::raw::c_void,
        error: *mut NSError,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MLUpdateTask").unwrap(), updateTaskForModelAtURL : modelURL, trainingData : trainingData, configuration : configuration, completionHandler : completionHandler, error : error)
    }
    unsafe fn updateTaskForModelAtURL_trainingData_completionHandler_error_(
        modelURL: NSURL,
        trainingData: *mut u64,
        completionHandler: *mut ::std::os::raw::c_void,
        error: *mut NSError,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MLUpdateTask").unwrap(), updateTaskForModelAtURL : modelURL, trainingData : trainingData, completionHandler : completionHandler, error : error)
    }
    unsafe fn updateTaskForModelAtURL_trainingData_configuration_progressHandlers_error_(
        modelURL: NSURL,
        trainingData: *mut u64,
        configuration: MLModelConfiguration,
        progressHandlers: MLUpdateProgressHandlers,
        error: *mut NSError,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MLUpdateTask").unwrap(), updateTaskForModelAtURL : modelURL, trainingData : trainingData, configuration : configuration, progressHandlers : progressHandlers, error : error)
    }
    unsafe fn updateTaskForModelAtURL_trainingData_progressHandlers_error_(
        modelURL: NSURL,
        trainingData: *mut u64,
        progressHandlers: MLUpdateProgressHandlers,
        error: *mut NSError,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MLUpdateTask").unwrap(), updateTaskForModelAtURL : modelURL, trainingData : trainingData, progressHandlers : progressHandlers, error : error)
    }
    unsafe fn new() -> id
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MLUpdateTask").unwrap(), new)
    }
}
pub trait PMLWritable: Sized + std::ops::Deref {
    unsafe fn writeToURL_error_(&self, url: NSURL, error: *mut NSError) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, writeToURL : url, error : error)
    }
}
pub type MLUpdateProgressEvent = NSInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MLUpdateContext(pub id);
impl std::ops::Deref for MLUpdateContext {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MLUpdateContext {}
impl MLUpdateContext {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MLUpdateContext").unwrap(), alloc) })
    }
}
impl INSObject for MLUpdateContext {}
impl PNSObject for MLUpdateContext {}
impl std::convert::TryFrom<NSObject> for MLUpdateContext {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<MLUpdateContext, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MLUpdateContext").unwrap()) };
        if is_kind_of {
            Ok(MLUpdateContext(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to MLUpdateContext")
        }
    }
}
impl IMLUpdateContext for MLUpdateContext {}
pub trait IMLUpdateContext: Sized + std::ops::Deref {
    unsafe fn task(&self) -> MLUpdateTask
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, task)
    }
    unsafe fn model(&self) -> MLModel
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, model)
    }
    unsafe fn event(&self) -> MLUpdateProgressEvent
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, event)
    }
    unsafe fn metrics(&self) -> NSDictionary
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, metrics)
    }
    unsafe fn parameters(&self) -> NSDictionary
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, parameters)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MLUpdateProgressHandlers(pub id);
impl std::ops::Deref for MLUpdateProgressHandlers {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MLUpdateProgressHandlers {}
impl MLUpdateProgressHandlers {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MLUpdateProgressHandlers").unwrap(), alloc) })
    }
}
impl INSObject for MLUpdateProgressHandlers {}
impl PNSObject for MLUpdateProgressHandlers {}
impl std::convert::TryFrom<NSObject> for MLUpdateProgressHandlers {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<MLUpdateProgressHandlers, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MLUpdateProgressHandlers").unwrap()) };
        if is_kind_of {
            Ok(MLUpdateProgressHandlers(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to MLUpdateProgressHandlers")
        }
    }
}
impl IMLUpdateProgressHandlers for MLUpdateProgressHandlers {}
pub trait IMLUpdateProgressHandlers: Sized + std::ops::Deref {
    unsafe fn initForEvents_progressHandler_completionHandler_(
        &self,
        interestedEvents: MLUpdateProgressEvent,
        progressHandler: *mut ::std::os::raw::c_void,
        completionHandler: *mut ::std::os::raw::c_void,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initForEvents : interestedEvents, progressHandler : progressHandler, completionHandler : completionHandler)
    }
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn new() -> id
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MLUpdateProgressHandlers").unwrap(), new)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MLMetricKey(pub id);
impl std::ops::Deref for MLMetricKey {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MLMetricKey {}
impl MLMetricKey {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MLMetricKey").unwrap(), alloc) })
    }
}
impl IMLKey for MLMetricKey {}
impl PNSCopying for MLMetricKey {}
impl PNSSecureCoding for MLMetricKey {}
impl From<MLMetricKey> for MLKey {
    fn from(child: MLMetricKey) -> MLKey {
        MLKey(child.0)
    }
}
impl std::convert::TryFrom<MLKey> for MLMetricKey {
    type Error = &'static str;
    fn try_from(parent: MLKey) -> Result<MLMetricKey, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MLMetricKey").unwrap()) };
        if is_kind_of {
            Ok(MLMetricKey(parent.0))
        } else {
            Err("This MLKey cannot be downcasted to MLMetricKey")
        }
    }
}
impl INSObject for MLMetricKey {}
impl PNSObject for MLMetricKey {}
impl IMLMetricKey for MLMetricKey {}
pub trait IMLMetricKey: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn new() -> id
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MLMetricKey").unwrap(), new)
    }
    unsafe fn lossValue() -> MLMetricKey
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MLMetricKey").unwrap(), lossValue)
    }
    unsafe fn epochIndex() -> MLMetricKey
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MLMetricKey").unwrap(), epochIndex)
    }
    unsafe fn miniBatchIndex() -> MLMetricKey
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MLMetricKey").unwrap(), miniBatchIndex)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MLNumericConstraint(pub id);
impl std::ops::Deref for MLNumericConstraint {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MLNumericConstraint {}
impl MLNumericConstraint {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MLNumericConstraint").unwrap(), alloc) })
    }
}
impl PNSSecureCoding for MLNumericConstraint {}
impl INSObject for MLNumericConstraint {}
impl PNSObject for MLNumericConstraint {}
impl std::convert::TryFrom<NSObject> for MLNumericConstraint {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<MLNumericConstraint, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MLNumericConstraint").unwrap()) };
        if is_kind_of {
            Ok(MLNumericConstraint(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to MLNumericConstraint")
        }
    }
}
impl IMLNumericConstraint for MLNumericConstraint {}
pub trait IMLNumericConstraint: Sized + std::ops::Deref {
    unsafe fn minNumber(&self) -> NSNumber
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, minNumber)
    }
    unsafe fn maxNumber(&self) -> NSNumber
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, maxNumber)
    }
    unsafe fn enumeratedNumbers(&self) -> NSSet
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, enumeratedNumbers)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MLParameterDescription(pub id);
impl std::ops::Deref for MLParameterDescription {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MLParameterDescription {}
impl MLParameterDescription {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MLParameterDescription").unwrap(), alloc) })
    }
}
impl PNSSecureCoding for MLParameterDescription {}
impl INSObject for MLParameterDescription {}
impl PNSObject for MLParameterDescription {}
impl std::convert::TryFrom<NSObject> for MLParameterDescription {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<MLParameterDescription, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MLParameterDescription").unwrap()) };
        if is_kind_of {
            Ok(MLParameterDescription(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to MLParameterDescription")
        }
    }
}
impl IMLParameterDescription for MLParameterDescription {}
pub trait IMLParameterDescription: Sized + std::ops::Deref {
    unsafe fn key(&self) -> MLParameterKey
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, key)
    }
    unsafe fn defaultValue(&self) -> id
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, defaultValue)
    }
    unsafe fn numericConstraint(&self) -> MLNumericConstraint
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, numericConstraint)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MLParameterKey(pub id);
impl std::ops::Deref for MLParameterKey {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MLParameterKey {}
impl MLParameterKey {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MLParameterKey").unwrap(), alloc) })
    }
}
impl IMLKey for MLParameterKey {}
impl PNSCopying for MLParameterKey {}
impl PNSSecureCoding for MLParameterKey {}
impl std::convert::TryFrom<MLKey> for MLParameterKey {
    type Error = &'static str;
    fn try_from(parent: MLKey) -> Result<MLParameterKey, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MLParameterKey").unwrap()) };
        if is_kind_of {
            Ok(MLParameterKey(parent.0))
        } else {
            Err("This MLKey cannot be downcasted to MLParameterKey")
        }
    }
}
impl INSObject for MLParameterKey {}
impl PNSObject for MLParameterKey {}
impl IMLParameterKey for MLParameterKey {}
pub trait IMLParameterKey: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn new() -> id
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MLParameterKey").unwrap(), new)
    }
    unsafe fn learningRate() -> MLParameterKey
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MLParameterKey").unwrap(), learningRate)
    }
    unsafe fn momentum() -> MLParameterKey
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MLParameterKey").unwrap(), momentum)
    }
    unsafe fn miniBatchSize() -> MLParameterKey
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MLParameterKey").unwrap(), miniBatchSize)
    }
    unsafe fn beta1() -> MLParameterKey
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MLParameterKey").unwrap(), beta1)
    }
    unsafe fn beta2() -> MLParameterKey
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MLParameterKey").unwrap(), beta2)
    }
    unsafe fn eps() -> MLParameterKey
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MLParameterKey").unwrap(), eps)
    }
    unsafe fn epochs() -> MLParameterKey
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MLParameterKey").unwrap(), epochs)
    }
    unsafe fn shuffle() -> MLParameterKey
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MLParameterKey").unwrap(), shuffle)
    }
    unsafe fn seed() -> MLParameterKey
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MLParameterKey").unwrap(), seed)
    }
    unsafe fn numberOfNeighbors() -> MLParameterKey
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MLParameterKey").unwrap(), numberOfNeighbors)
    }
}
impl MLParameterKey_MLLinkedModelParameters for MLParameterKey {}
pub trait MLParameterKey_MLLinkedModelParameters: Sized + std::ops::Deref {
    unsafe fn linkedModelFileName() -> MLParameterKey
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MLParameterKey").unwrap(), linkedModelFileName)
    }
    unsafe fn linkedModelSearchPath() -> MLParameterKey
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MLParameterKey").unwrap(), linkedModelSearchPath)
    }
}
impl MLParameterKey_MLNeuralNetworkParameters for MLParameterKey {}
pub trait MLParameterKey_MLNeuralNetworkParameters: Sized + std::ops::Deref {
    unsafe fn weights() -> MLParameterKey
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MLParameterKey").unwrap(), weights)
    }
    unsafe fn biases() -> MLParameterKey
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MLParameterKey").unwrap(), biases)
    }
}
impl MLParameterKey_MLScopedParameters for MLParameterKey {}
pub trait MLParameterKey_MLScopedParameters: Sized + std::ops::Deref {
    unsafe fn scopedTo_(&self, scope: NSString) -> MLParameterKey
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, scopedTo : scope)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MLModelCollectionEntry(pub id);
impl std::ops::Deref for MLModelCollectionEntry {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MLModelCollectionEntry {}
impl MLModelCollectionEntry {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MLModelCollectionEntry").unwrap(), alloc) })
    }
}
impl INSObject for MLModelCollectionEntry {}
impl PNSObject for MLModelCollectionEntry {}
impl std::convert::TryFrom<NSObject> for MLModelCollectionEntry {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<MLModelCollectionEntry, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MLModelCollectionEntry").unwrap()) };
        if is_kind_of {
            Ok(MLModelCollectionEntry(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to MLModelCollectionEntry")
        }
    }
}
impl IMLModelCollectionEntry for MLModelCollectionEntry {}
pub trait IMLModelCollectionEntry: Sized + std::ops::Deref {
    unsafe fn isEqualToModelCollectionEntry_(&self, entry: MLModelCollectionEntry) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, isEqualToModelCollectionEntry : entry)
    }
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn modelIdentifier(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, modelIdentifier)
    }
    unsafe fn modelURL(&self) -> NSURL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, modelURL)
    }
    unsafe fn new() -> id
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MLModelCollectionEntry").unwrap(), new)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MLModelCollection(pub id);
impl std::ops::Deref for MLModelCollection {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MLModelCollection {}
impl MLModelCollection {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MLModelCollection").unwrap(), alloc) })
    }
}
impl INSObject for MLModelCollection {}
impl PNSObject for MLModelCollection {}
impl std::convert::TryFrom<NSObject> for MLModelCollection {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<MLModelCollection, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MLModelCollection").unwrap()) };
        if is_kind_of {
            Ok(MLModelCollection(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to MLModelCollection")
        }
    }
}
impl IMLModelCollection for MLModelCollection {}
pub trait IMLModelCollection: Sized + std::ops::Deref {
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
    unsafe fn entries(&self) -> NSDictionary
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, entries)
    }
    unsafe fn deploymentID(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, deploymentID)
    }
    unsafe fn beginAccessingModelCollectionWithIdentifier_completionHandler_(
        identifier: NSString,
        completionHandler: *mut ::std::os::raw::c_void,
    ) -> NSProgress
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MLModelCollection").unwrap(), beginAccessingModelCollectionWithIdentifier : identifier, completionHandler : completionHandler)
    }
    unsafe fn endAccessingModelCollectionWithIdentifier_completionHandler_(
        identifier: NSString,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MLModelCollection").unwrap(), endAccessingModelCollectionWithIdentifier : identifier, completionHandler : completionHandler)
    }
    unsafe fn new() -> id
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MLModelCollection").unwrap(), new)
    }
}
pub trait PMLComputeDeviceProtocol: Sized + std::ops::Deref {}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MLCPUComputeDevice(pub id);
impl std::ops::Deref for MLCPUComputeDevice {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MLCPUComputeDevice {}
impl MLCPUComputeDevice {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MLCPUComputeDevice").unwrap(), alloc) })
    }
}
impl PMLComputeDeviceProtocol for MLCPUComputeDevice {}
impl INSObject for MLCPUComputeDevice {}
impl PNSObject for MLCPUComputeDevice {}
impl std::convert::TryFrom<NSObject> for MLCPUComputeDevice {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<MLCPUComputeDevice, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MLCPUComputeDevice").unwrap()) };
        if is_kind_of {
            Ok(MLCPUComputeDevice(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to MLCPUComputeDevice")
        }
    }
}
impl IMLCPUComputeDevice for MLCPUComputeDevice {}
pub trait IMLCPUComputeDevice: Sized + std::ops::Deref {
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
        msg_send!(&*objc2::runtime::AnyClass::get(c"MLCPUComputeDevice").unwrap(), new)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MLGPUComputeDevice(pub id);
impl std::ops::Deref for MLGPUComputeDevice {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MLGPUComputeDevice {}
impl MLGPUComputeDevice {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MLGPUComputeDevice").unwrap(), alloc) })
    }
}
impl PMLComputeDeviceProtocol for MLGPUComputeDevice {}
impl INSObject for MLGPUComputeDevice {}
impl PNSObject for MLGPUComputeDevice {}
impl std::convert::TryFrom<NSObject> for MLGPUComputeDevice {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<MLGPUComputeDevice, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MLGPUComputeDevice").unwrap()) };
        if is_kind_of {
            Ok(MLGPUComputeDevice(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to MLGPUComputeDevice")
        }
    }
}
impl IMLGPUComputeDevice for MLGPUComputeDevice {}
pub trait IMLGPUComputeDevice: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn metalDevice(&self) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, metalDevice)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MLGPUComputeDevice").unwrap(), new)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MLNeuralEngineComputeDevice(pub id);
impl std::ops::Deref for MLNeuralEngineComputeDevice {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MLNeuralEngineComputeDevice {}
impl MLNeuralEngineComputeDevice {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MLNeuralEngineComputeDevice").unwrap(), alloc) })
    }
}
impl PMLComputeDeviceProtocol for MLNeuralEngineComputeDevice {}
impl INSObject for MLNeuralEngineComputeDevice {}
impl PNSObject for MLNeuralEngineComputeDevice {}
impl std::convert::TryFrom<NSObject> for MLNeuralEngineComputeDevice {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<MLNeuralEngineComputeDevice, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MLNeuralEngineComputeDevice").unwrap()) };
        if is_kind_of {
            Ok(MLNeuralEngineComputeDevice(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to MLNeuralEngineComputeDevice")
        }
    }
}
impl IMLNeuralEngineComputeDevice for MLNeuralEngineComputeDevice {}
pub trait IMLNeuralEngineComputeDevice: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn totalCoreCount(&self) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, totalCoreCount)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MLNeuralEngineComputeDevice").unwrap(), new)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MLModelStructure(pub id);
impl std::ops::Deref for MLModelStructure {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MLModelStructure {}
impl MLModelStructure {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MLModelStructure").unwrap(), alloc) })
    }
}
impl INSObject for MLModelStructure {}
impl PNSObject for MLModelStructure {}
impl std::convert::TryFrom<NSObject> for MLModelStructure {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<MLModelStructure, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MLModelStructure").unwrap()) };
        if is_kind_of {
            Ok(MLModelStructure(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to MLModelStructure")
        }
    }
}
impl IMLModelStructure for MLModelStructure {}
pub trait IMLModelStructure: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn neuralNetwork(&self) -> MLModelStructureNeuralNetwork
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, neuralNetwork)
    }
    unsafe fn program(&self) -> MLModelStructureProgram
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, program)
    }
    unsafe fn pipeline(&self) -> MLModelStructurePipeline
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, pipeline)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MLModelStructure").unwrap(), new)
    }
    unsafe fn loadContentsOfURL_completionHandler_(url: NSURL, handler: *mut ::std::os::raw::c_void)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MLModelStructure").unwrap(), loadContentsOfURL : url, completionHandler : handler)
    }
    unsafe fn loadModelAsset_completionHandler_(
        asset: MLModelAsset,
        handler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MLModelStructure").unwrap(), loadModelAsset : asset, completionHandler : handler)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MLModelStructureNeuralNetwork(pub id);
impl std::ops::Deref for MLModelStructureNeuralNetwork {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MLModelStructureNeuralNetwork {}
impl MLModelStructureNeuralNetwork {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MLModelStructureNeuralNetwork").unwrap(), alloc) })
    }
}
impl INSObject for MLModelStructureNeuralNetwork {}
impl PNSObject for MLModelStructureNeuralNetwork {}
impl std::convert::TryFrom<NSObject> for MLModelStructureNeuralNetwork {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<MLModelStructureNeuralNetwork, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MLModelStructureNeuralNetwork").unwrap())
        };
        if is_kind_of {
            Ok(MLModelStructureNeuralNetwork(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to MLModelStructureNeuralNetwork")
        }
    }
}
impl IMLModelStructureNeuralNetwork for MLModelStructureNeuralNetwork {}
pub trait IMLModelStructureNeuralNetwork: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn layers(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, layers)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MLModelStructureNeuralNetwork").unwrap(), new)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MLModelStructureNeuralNetworkLayer(pub id);
impl std::ops::Deref for MLModelStructureNeuralNetworkLayer {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MLModelStructureNeuralNetworkLayer {}
impl MLModelStructureNeuralNetworkLayer {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MLModelStructureNeuralNetworkLayer").unwrap(), alloc) })
    }
}
impl INSObject for MLModelStructureNeuralNetworkLayer {}
impl PNSObject for MLModelStructureNeuralNetworkLayer {}
impl std::convert::TryFrom<NSObject> for MLModelStructureNeuralNetworkLayer {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<MLModelStructureNeuralNetworkLayer, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MLModelStructureNeuralNetworkLayer").unwrap())
        };
        if is_kind_of {
            Ok(MLModelStructureNeuralNetworkLayer(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to MLModelStructureNeuralNetworkLayer")
        }
    }
}
impl IMLModelStructureNeuralNetworkLayer for MLModelStructureNeuralNetworkLayer {}
pub trait IMLModelStructureNeuralNetworkLayer: Sized + std::ops::Deref {
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
    unsafe fn type_(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, type)
    }
    unsafe fn inputNames(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inputNames)
    }
    unsafe fn outputNames(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, outputNames)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MLModelStructureNeuralNetworkLayer").unwrap(), new)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MLModelStructurePipeline(pub id);
impl std::ops::Deref for MLModelStructurePipeline {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MLModelStructurePipeline {}
impl MLModelStructurePipeline {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MLModelStructurePipeline").unwrap(), alloc) })
    }
}
impl INSObject for MLModelStructurePipeline {}
impl PNSObject for MLModelStructurePipeline {}
impl std::convert::TryFrom<NSObject> for MLModelStructurePipeline {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<MLModelStructurePipeline, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MLModelStructurePipeline").unwrap()) };
        if is_kind_of {
            Ok(MLModelStructurePipeline(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to MLModelStructurePipeline")
        }
    }
}
impl IMLModelStructurePipeline for MLModelStructurePipeline {}
pub trait IMLModelStructurePipeline: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn subModelNames(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, subModelNames)
    }
    unsafe fn subModels(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, subModels)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MLModelStructurePipeline").unwrap(), new)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MLModelStructureProgram(pub id);
impl std::ops::Deref for MLModelStructureProgram {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MLModelStructureProgram {}
impl MLModelStructureProgram {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MLModelStructureProgram").unwrap(), alloc) })
    }
}
impl INSObject for MLModelStructureProgram {}
impl PNSObject for MLModelStructureProgram {}
impl std::convert::TryFrom<NSObject> for MLModelStructureProgram {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<MLModelStructureProgram, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MLModelStructureProgram").unwrap()) };
        if is_kind_of {
            Ok(MLModelStructureProgram(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to MLModelStructureProgram")
        }
    }
}
impl IMLModelStructureProgram for MLModelStructureProgram {}
pub trait IMLModelStructureProgram: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn functions(&self) -> NSDictionary
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, functions)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MLModelStructureProgram").unwrap(), new)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MLModelStructureProgramArgument(pub id);
impl std::ops::Deref for MLModelStructureProgramArgument {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MLModelStructureProgramArgument {}
impl MLModelStructureProgramArgument {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MLModelStructureProgramArgument").unwrap(), alloc) })
    }
}
impl INSObject for MLModelStructureProgramArgument {}
impl PNSObject for MLModelStructureProgramArgument {}
impl std::convert::TryFrom<NSObject> for MLModelStructureProgramArgument {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<MLModelStructureProgramArgument, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MLModelStructureProgramArgument").unwrap())
        };
        if is_kind_of {
            Ok(MLModelStructureProgramArgument(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to MLModelStructureProgramArgument")
        }
    }
}
impl IMLModelStructureProgramArgument for MLModelStructureProgramArgument {}
pub trait IMLModelStructureProgramArgument: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn bindings(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, bindings)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MLModelStructureProgramArgument").unwrap(), new)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MLModelStructureProgramBinding(pub id);
impl std::ops::Deref for MLModelStructureProgramBinding {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MLModelStructureProgramBinding {}
impl MLModelStructureProgramBinding {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MLModelStructureProgramBinding").unwrap(), alloc) })
    }
}
impl INSObject for MLModelStructureProgramBinding {}
impl PNSObject for MLModelStructureProgramBinding {}
impl std::convert::TryFrom<NSObject> for MLModelStructureProgramBinding {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<MLModelStructureProgramBinding, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MLModelStructureProgramBinding").unwrap())
        };
        if is_kind_of {
            Ok(MLModelStructureProgramBinding(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to MLModelStructureProgramBinding")
        }
    }
}
impl IMLModelStructureProgramBinding for MLModelStructureProgramBinding {}
pub trait IMLModelStructureProgramBinding: Sized + std::ops::Deref {
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
    unsafe fn value(&self) -> MLModelStructureProgramValue
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, value)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MLModelStructureProgramBinding").unwrap(), new)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MLModelStructureProgramBlock(pub id);
impl std::ops::Deref for MLModelStructureProgramBlock {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MLModelStructureProgramBlock {}
impl MLModelStructureProgramBlock {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MLModelStructureProgramBlock").unwrap(), alloc) })
    }
}
impl INSObject for MLModelStructureProgramBlock {}
impl PNSObject for MLModelStructureProgramBlock {}
impl std::convert::TryFrom<NSObject> for MLModelStructureProgramBlock {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<MLModelStructureProgramBlock, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MLModelStructureProgramBlock").unwrap()) };
        if is_kind_of {
            Ok(MLModelStructureProgramBlock(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to MLModelStructureProgramBlock")
        }
    }
}
impl IMLModelStructureProgramBlock for MLModelStructureProgramBlock {}
pub trait IMLModelStructureProgramBlock: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn inputs(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inputs)
    }
    unsafe fn outputNames(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, outputNames)
    }
    unsafe fn operations(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, operations)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MLModelStructureProgramBlock").unwrap(), new)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MLModelStructureProgramFunction(pub id);
impl std::ops::Deref for MLModelStructureProgramFunction {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MLModelStructureProgramFunction {}
impl MLModelStructureProgramFunction {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MLModelStructureProgramFunction").unwrap(), alloc) })
    }
}
impl INSObject for MLModelStructureProgramFunction {}
impl PNSObject for MLModelStructureProgramFunction {}
impl std::convert::TryFrom<NSObject> for MLModelStructureProgramFunction {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<MLModelStructureProgramFunction, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MLModelStructureProgramFunction").unwrap())
        };
        if is_kind_of {
            Ok(MLModelStructureProgramFunction(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to MLModelStructureProgramFunction")
        }
    }
}
impl IMLModelStructureProgramFunction for MLModelStructureProgramFunction {}
pub trait IMLModelStructureProgramFunction: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn inputs(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inputs)
    }
    unsafe fn block(&self) -> MLModelStructureProgramBlock
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, block)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MLModelStructureProgramFunction").unwrap(), new)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MLModelStructureProgramNamedValueType(pub id);
impl std::ops::Deref for MLModelStructureProgramNamedValueType {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MLModelStructureProgramNamedValueType {}
impl MLModelStructureProgramNamedValueType {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MLModelStructureProgramNamedValueType").unwrap(), alloc) })
    }
}
impl INSObject for MLModelStructureProgramNamedValueType {}
impl PNSObject for MLModelStructureProgramNamedValueType {}
impl std::convert::TryFrom<NSObject> for MLModelStructureProgramNamedValueType {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<MLModelStructureProgramNamedValueType, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MLModelStructureProgramNamedValueType").unwrap())
        };
        if is_kind_of {
            Ok(MLModelStructureProgramNamedValueType(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to MLModelStructureProgramNamedValueType")
        }
    }
}
impl IMLModelStructureProgramNamedValueType for MLModelStructureProgramNamedValueType {}
pub trait IMLModelStructureProgramNamedValueType: Sized + std::ops::Deref {
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
    unsafe fn type_(&self) -> MLModelStructureProgramValueType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, type)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MLModelStructureProgramNamedValueType").unwrap(), new)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MLModelStructureProgramOperation(pub id);
impl std::ops::Deref for MLModelStructureProgramOperation {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MLModelStructureProgramOperation {}
impl MLModelStructureProgramOperation {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MLModelStructureProgramOperation").unwrap(), alloc) })
    }
}
impl INSObject for MLModelStructureProgramOperation {}
impl PNSObject for MLModelStructureProgramOperation {}
impl std::convert::TryFrom<NSObject> for MLModelStructureProgramOperation {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<MLModelStructureProgramOperation, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MLModelStructureProgramOperation").unwrap())
        };
        if is_kind_of {
            Ok(MLModelStructureProgramOperation(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to MLModelStructureProgramOperation")
        }
    }
}
impl IMLModelStructureProgramOperation for MLModelStructureProgramOperation {}
pub trait IMLModelStructureProgramOperation: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn operatorName(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, operatorName)
    }
    unsafe fn inputs(&self) -> NSDictionary
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inputs)
    }
    unsafe fn outputs(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, outputs)
    }
    unsafe fn blocks(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, blocks)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MLModelStructureProgramOperation").unwrap(), new)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MLModelStructureProgramValue(pub id);
impl std::ops::Deref for MLModelStructureProgramValue {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MLModelStructureProgramValue {}
impl MLModelStructureProgramValue {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MLModelStructureProgramValue").unwrap(), alloc) })
    }
}
impl INSObject for MLModelStructureProgramValue {}
impl PNSObject for MLModelStructureProgramValue {}
impl std::convert::TryFrom<NSObject> for MLModelStructureProgramValue {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<MLModelStructureProgramValue, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MLModelStructureProgramValue").unwrap()) };
        if is_kind_of {
            Ok(MLModelStructureProgramValue(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to MLModelStructureProgramValue")
        }
    }
}
impl IMLModelStructureProgramValue for MLModelStructureProgramValue {}
pub trait IMLModelStructureProgramValue: Sized + std::ops::Deref {
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
        msg_send!(&*objc2::runtime::AnyClass::get(c"MLModelStructureProgramValue").unwrap(), new)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MLModelStructureProgramValueType(pub id);
impl std::ops::Deref for MLModelStructureProgramValueType {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MLModelStructureProgramValueType {}
impl MLModelStructureProgramValueType {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MLModelStructureProgramValueType").unwrap(), alloc) })
    }
}
impl INSObject for MLModelStructureProgramValueType {}
impl PNSObject for MLModelStructureProgramValueType {}
impl std::convert::TryFrom<NSObject> for MLModelStructureProgramValueType {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<MLModelStructureProgramValueType, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MLModelStructureProgramValueType").unwrap())
        };
        if is_kind_of {
            Ok(MLModelStructureProgramValueType(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to MLModelStructureProgramValueType")
        }
    }
}
impl IMLModelStructureProgramValueType for MLModelStructureProgramValueType {}
pub trait IMLModelStructureProgramValueType: Sized + std::ops::Deref {
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
        msg_send!(&*objc2::runtime::AnyClass::get(c"MLModelStructureProgramValueType").unwrap(), new)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MLComputePlan(pub id);
impl std::ops::Deref for MLComputePlan {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MLComputePlan {}
impl MLComputePlan {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MLComputePlan").unwrap(), alloc) })
    }
}
impl INSObject for MLComputePlan {}
impl PNSObject for MLComputePlan {}
impl std::convert::TryFrom<NSObject> for MLComputePlan {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<MLComputePlan, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MLComputePlan").unwrap()) };
        if is_kind_of {
            Ok(MLComputePlan(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to MLComputePlan")
        }
    }
}
impl IMLComputePlan for MLComputePlan {}
pub trait IMLComputePlan: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn estimatedCostOfMLProgramOperation_(
        &self,
        operation: MLModelStructureProgramOperation,
    ) -> MLComputePlanCost
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, estimatedCostOfMLProgramOperation : operation)
    }
    unsafe fn computeDeviceUsageForNeuralNetworkLayer_(
        &self,
        layer: MLModelStructureNeuralNetworkLayer,
    ) -> MLComputePlanDeviceUsage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, computeDeviceUsageForNeuralNetworkLayer : layer)
    }
    unsafe fn computeDeviceUsageForMLProgramOperation_(
        &self,
        operation: MLModelStructureProgramOperation,
    ) -> MLComputePlanDeviceUsage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, computeDeviceUsageForMLProgramOperation : operation)
    }
    unsafe fn modelStructure(&self) -> MLModelStructure
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, modelStructure)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MLComputePlan").unwrap(), new)
    }
    unsafe fn loadContentsOfURL_configuration_completionHandler_(
        url: NSURL,
        configuration: MLModelConfiguration,
        handler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MLComputePlan").unwrap(), loadContentsOfURL : url, configuration : configuration, completionHandler : handler)
    }
    unsafe fn loadModelAsset_configuration_completionHandler_(
        asset: MLModelAsset,
        configuration: MLModelConfiguration,
        handler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MLComputePlan").unwrap(), loadModelAsset : asset, configuration : configuration, completionHandler : handler)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MLComputePlanCost(pub id);
impl std::ops::Deref for MLComputePlanCost {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MLComputePlanCost {}
impl MLComputePlanCost {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MLComputePlanCost").unwrap(), alloc) })
    }
}
impl INSObject for MLComputePlanCost {}
impl PNSObject for MLComputePlanCost {}
impl std::convert::TryFrom<NSObject> for MLComputePlanCost {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<MLComputePlanCost, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MLComputePlanCost").unwrap()) };
        if is_kind_of {
            Ok(MLComputePlanCost(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to MLComputePlanCost")
        }
    }
}
impl IMLComputePlanCost for MLComputePlanCost {}
pub trait IMLComputePlanCost: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn weight(&self) -> f64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, weight)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MLComputePlanCost").unwrap(), new)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MLComputePlanDeviceUsage(pub id);
impl std::ops::Deref for MLComputePlanDeviceUsage {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MLComputePlanDeviceUsage {}
impl MLComputePlanDeviceUsage {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MLComputePlanDeviceUsage").unwrap(), alloc) })
    }
}
impl INSObject for MLComputePlanDeviceUsage {}
impl PNSObject for MLComputePlanDeviceUsage {}
impl std::convert::TryFrom<NSObject> for MLComputePlanDeviceUsage {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<MLComputePlanDeviceUsage, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MLComputePlanDeviceUsage").unwrap()) };
        if is_kind_of {
            Ok(MLComputePlanDeviceUsage(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to MLComputePlanDeviceUsage")
        }
    }
}
impl IMLComputePlanDeviceUsage for MLComputePlanDeviceUsage {}
pub trait IMLComputePlanDeviceUsage: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn supportedComputeDevices(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, supportedComputeDevices)
    }
    unsafe fn preferredComputeDevice(&self) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, preferredComputeDevice)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MLComputePlanDeviceUsage").unwrap(), new)
    }
}
unsafe extern "C" {
    pub static MLFeatureValueImageOptionCropRect: MLFeatureValueImageOption;
}
unsafe extern "C" {
    pub static MLFeatureValueImageOptionCropAndScale: MLFeatureValueImageOption;
}
unsafe extern "C" {
    pub static MLModelDescriptionKey: MLModelMetadataKey;
}
unsafe extern "C" {
    pub static MLModelVersionStringKey: MLModelMetadataKey;
}
unsafe extern "C" {
    pub static MLModelAuthorKey: MLModelMetadataKey;
}
unsafe extern "C" {
    pub static MLModelLicenseKey: MLModelMetadataKey;
}
unsafe extern "C" {
    pub static MLModelCreatorDefinedKey: MLModelMetadataKey;
}
unsafe extern "C" {
    pub static MLModelErrorDomain: NSString;
}
unsafe extern "C" {
    pub fn MLAllComputeDevices() -> NSArray;
}

unsafe impl objc2::encode::RefEncode for MLMultiArray {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MLMultiArray {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MLSequence {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MLSequence {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MLState {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MLState {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MLFeatureValue {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MLFeatureValue {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MLImageSize {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MLImageSize {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MLImageSizeConstraint {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MLImageSizeConstraint {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MLImageConstraint {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MLImageConstraint {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MLDictionaryConstraint {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MLDictionaryConstraint {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MLMultiArrayShapeConstraint {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MLMultiArrayShapeConstraint {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MLMultiArrayConstraint {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MLMultiArrayConstraint {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MLSequenceConstraint {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MLSequenceConstraint {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MLStateConstraint {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MLStateConstraint {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MLFeatureDescription {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MLFeatureDescription {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MLDictionaryFeatureProvider {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MLDictionaryFeatureProvider {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MLArrayBatchProvider {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MLArrayBatchProvider {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MLModelDescription {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MLModelDescription {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MLPredictionOptions {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MLPredictionOptions {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MLOptimizationHints {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MLOptimizationHints {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MLModelConfiguration {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MLModelConfiguration {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MLModelAsset {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MLModelAsset {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MLModel {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MLModel {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MLKey {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MLKey {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MLTask {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MLTask {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MLUpdateTask {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MLUpdateTask {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MLUpdateContext {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MLUpdateContext {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MLUpdateProgressHandlers {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MLUpdateProgressHandlers {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MLMetricKey {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MLMetricKey {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MLNumericConstraint {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MLNumericConstraint {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MLParameterDescription {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MLParameterDescription {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MLParameterKey {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MLParameterKey {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MLModelCollectionEntry {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MLModelCollectionEntry {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MLModelCollection {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MLModelCollection {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MLCPUComputeDevice {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MLCPUComputeDevice {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MLGPUComputeDevice {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MLGPUComputeDevice {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MLNeuralEngineComputeDevice {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MLNeuralEngineComputeDevice {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MLModelStructure {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MLModelStructure {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MLModelStructureNeuralNetwork {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MLModelStructureNeuralNetwork {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MLModelStructureNeuralNetworkLayer {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MLModelStructureNeuralNetworkLayer {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MLModelStructurePipeline {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MLModelStructurePipeline {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MLModelStructureProgram {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MLModelStructureProgram {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MLModelStructureProgramArgument {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MLModelStructureProgramArgument {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MLModelStructureProgramBinding {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MLModelStructureProgramBinding {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MLModelStructureProgramBlock {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MLModelStructureProgramBlock {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MLModelStructureProgramFunction {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MLModelStructureProgramFunction {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MLModelStructureProgramNamedValueType {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MLModelStructureProgramNamedValueType {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MLModelStructureProgramOperation {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MLModelStructureProgramOperation {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MLModelStructureProgramValue {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MLModelStructureProgramValue {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MLModelStructureProgramValueType {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MLModelStructureProgramValueType {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MLComputePlan {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MLComputePlan {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MLComputePlanCost {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MLComputePlanCost {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MLComputePlanDeviceUsage {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MLComputePlanDeviceUsage {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
