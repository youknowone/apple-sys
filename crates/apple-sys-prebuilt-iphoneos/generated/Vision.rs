#[allow(unused_imports)]
use crate::objc::*;
#[allow(unused_imports)]
use crate::AVFoundation::*;
#[allow(unused_imports)]
use crate::CoreFoundation::*;
#[allow(unused_imports)]
use crate::CoreGraphics::*;
#[allow(unused_imports)]
use crate::CoreImage::*;
#[allow(unused_imports)]
use crate::CoreML::*;
#[allow(unused_imports)]
use crate::CoreMedia::*;
#[allow(unused_imports)]
use crate::CoreVideo::*;
#[allow(unused_imports)]
use crate::Foundation::*;
#[allow(unused_imports)]
use crate::ImageIO::*;
#[allow(unused_imports)]
use crate::ModelIO::*;
#[allow(unused_imports)]
use crate::Security::*;

#[allow(unused_imports)]
use objc2::msg_send;
pub type VNConfidence = f32;
pub type VNAspectRatio = f32;
pub type VNDegrees = f32;
pub type VNImageCropAndScaleOption = NSUInteger;
pub type VNComputeStage = NSString;
pub type VNBarcodeSymbology = NSString;
pub type VNElementType = NSUInteger;
pub type VNVideoProcessingOption = NSString;
pub type VNChirality = NSInteger;
pub type VNPointsClassification = NSInteger;
pub type VNBarcodeCompositeType = NSInteger;
pub type VNRecognizedPointKey = NSString;
pub type VNRecognizedPointGroupKey = NSString;
pub type VNAnimalBodyPoseObservationJointName = VNRecognizedPointKey;
pub type VNAnimalBodyPoseObservationJointsGroupName = VNRecognizedPointGroupKey;
pub type VNHumanBodyPose3DObservationJointName = VNRecognizedPointKey;
pub type VNHumanBodyPose3DObservationJointsGroupName = VNRecognizedPointGroupKey;
pub type VNErrorCode = NSInteger;
pub trait PVNRequestRevisionProviding: Sized + std::ops::Deref {
    unsafe fn requestRevision(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, requestRevision)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct VNFaceLandmarkRegion(pub id);
impl std::ops::Deref for VNFaceLandmarkRegion {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for VNFaceLandmarkRegion {}
impl VNFaceLandmarkRegion {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"VNFaceLandmarkRegion").unwrap(), alloc) })
    }
}
impl PNSCopying for VNFaceLandmarkRegion {}
impl PNSSecureCoding for VNFaceLandmarkRegion {}
impl PVNRequestRevisionProviding for VNFaceLandmarkRegion {}
impl INSObject for VNFaceLandmarkRegion {}
impl PNSObject for VNFaceLandmarkRegion {}
impl std::convert::TryFrom<NSObject> for VNFaceLandmarkRegion {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<VNFaceLandmarkRegion, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"VNFaceLandmarkRegion").unwrap()) };
        if is_kind_of {
            Ok(VNFaceLandmarkRegion(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to VNFaceLandmarkRegion")
        }
    }
}
impl IVNFaceLandmarkRegion for VNFaceLandmarkRegion {}
pub trait IVNFaceLandmarkRegion: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn pointCount(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, pointCount)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"VNFaceLandmarkRegion").unwrap(), new)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct VNFaceLandmarkRegion2D(pub id);
impl std::ops::Deref for VNFaceLandmarkRegion2D {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for VNFaceLandmarkRegion2D {}
impl VNFaceLandmarkRegion2D {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"VNFaceLandmarkRegion2D").unwrap(), alloc) })
    }
}
impl IVNFaceLandmarkRegion for VNFaceLandmarkRegion2D {}
impl PNSCopying for VNFaceLandmarkRegion2D {}
impl PNSSecureCoding for VNFaceLandmarkRegion2D {}
impl PVNRequestRevisionProviding for VNFaceLandmarkRegion2D {}
impl From<VNFaceLandmarkRegion2D> for VNFaceLandmarkRegion {
    fn from(child: VNFaceLandmarkRegion2D) -> VNFaceLandmarkRegion {
        VNFaceLandmarkRegion(child.0)
    }
}
impl std::convert::TryFrom<VNFaceLandmarkRegion> for VNFaceLandmarkRegion2D {
    type Error = &'static str;
    fn try_from(parent: VNFaceLandmarkRegion) -> Result<VNFaceLandmarkRegion2D, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"VNFaceLandmarkRegion2D").unwrap()) };
        if is_kind_of {
            Ok(VNFaceLandmarkRegion2D(parent.0))
        } else {
            Err("This VNFaceLandmarkRegion cannot be downcasted to VNFaceLandmarkRegion2D")
        }
    }
}
impl INSObject for VNFaceLandmarkRegion2D {}
impl PNSObject for VNFaceLandmarkRegion2D {}
impl IVNFaceLandmarkRegion2D for VNFaceLandmarkRegion2D {}
pub trait IVNFaceLandmarkRegion2D: Sized + std::ops::Deref {
    unsafe fn pointsInImageOfSize_(&self, imageSize: CGSize) -> *const CGPoint
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, pointsInImageOfSize : imageSize)
    }
    unsafe fn normalizedPoints(&self) -> *const CGPoint
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, normalizedPoints)
    }
    unsafe fn precisionEstimatesPerPoint(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, precisionEstimatesPerPoint)
    }
    unsafe fn pointsClassification(&self) -> VNPointsClassification
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, pointsClassification)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct VNFaceLandmarks(pub id);
impl std::ops::Deref for VNFaceLandmarks {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for VNFaceLandmarks {}
impl VNFaceLandmarks {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"VNFaceLandmarks").unwrap(), alloc) })
    }
}
impl PNSCopying for VNFaceLandmarks {}
impl PNSSecureCoding for VNFaceLandmarks {}
impl PVNRequestRevisionProviding for VNFaceLandmarks {}
impl INSObject for VNFaceLandmarks {}
impl PNSObject for VNFaceLandmarks {}
impl std::convert::TryFrom<NSObject> for VNFaceLandmarks {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<VNFaceLandmarks, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"VNFaceLandmarks").unwrap()) };
        if is_kind_of {
            Ok(VNFaceLandmarks(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to VNFaceLandmarks")
        }
    }
}
impl IVNFaceLandmarks for VNFaceLandmarks {}
pub trait IVNFaceLandmarks: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn confidence(&self) -> VNConfidence
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, confidence)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct VNFaceLandmarks2D(pub id);
impl std::ops::Deref for VNFaceLandmarks2D {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for VNFaceLandmarks2D {}
impl VNFaceLandmarks2D {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"VNFaceLandmarks2D").unwrap(), alloc) })
    }
}
impl IVNFaceLandmarks for VNFaceLandmarks2D {}
impl PNSCopying for VNFaceLandmarks2D {}
impl PNSSecureCoding for VNFaceLandmarks2D {}
impl PVNRequestRevisionProviding for VNFaceLandmarks2D {}
impl From<VNFaceLandmarks2D> for VNFaceLandmarks {
    fn from(child: VNFaceLandmarks2D) -> VNFaceLandmarks {
        VNFaceLandmarks(child.0)
    }
}
impl std::convert::TryFrom<VNFaceLandmarks> for VNFaceLandmarks2D {
    type Error = &'static str;
    fn try_from(parent: VNFaceLandmarks) -> Result<VNFaceLandmarks2D, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"VNFaceLandmarks2D").unwrap()) };
        if is_kind_of {
            Ok(VNFaceLandmarks2D(parent.0))
        } else {
            Err("This VNFaceLandmarks cannot be downcasted to VNFaceLandmarks2D")
        }
    }
}
impl INSObject for VNFaceLandmarks2D {}
impl PNSObject for VNFaceLandmarks2D {}
impl IVNFaceLandmarks2D for VNFaceLandmarks2D {}
pub trait IVNFaceLandmarks2D: Sized + std::ops::Deref {
    unsafe fn allPoints(&self) -> VNFaceLandmarkRegion2D
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, allPoints)
    }
    unsafe fn faceContour(&self) -> VNFaceLandmarkRegion2D
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, faceContour)
    }
    unsafe fn leftEye(&self) -> VNFaceLandmarkRegion2D
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, leftEye)
    }
    unsafe fn rightEye(&self) -> VNFaceLandmarkRegion2D
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, rightEye)
    }
    unsafe fn leftEyebrow(&self) -> VNFaceLandmarkRegion2D
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, leftEyebrow)
    }
    unsafe fn rightEyebrow(&self) -> VNFaceLandmarkRegion2D
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, rightEyebrow)
    }
    unsafe fn nose(&self) -> VNFaceLandmarkRegion2D
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, nose)
    }
    unsafe fn noseCrest(&self) -> VNFaceLandmarkRegion2D
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, noseCrest)
    }
    unsafe fn medianLine(&self) -> VNFaceLandmarkRegion2D
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, medianLine)
    }
    unsafe fn outerLips(&self) -> VNFaceLandmarkRegion2D
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, outerLips)
    }
    unsafe fn innerLips(&self) -> VNFaceLandmarkRegion2D
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, innerLips)
    }
    unsafe fn leftPupil(&self) -> VNFaceLandmarkRegion2D
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, leftPupil)
    }
    unsafe fn rightPupil(&self) -> VNFaceLandmarkRegion2D
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, rightPupil)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct VNPoint(pub id);
impl std::ops::Deref for VNPoint {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for VNPoint {}
impl VNPoint {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"VNPoint").unwrap(), alloc) })
    }
}
impl PNSCopying for VNPoint {}
impl PNSSecureCoding for VNPoint {}
impl INSObject for VNPoint {}
impl PNSObject for VNPoint {}
impl std::convert::TryFrom<NSObject> for VNPoint {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<VNPoint, Self::Error> {
        let is_kind_of: bool = unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"VNPoint").unwrap()) };
        if is_kind_of {
            Ok(VNPoint(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to VNPoint")
        }
    }
}
impl IVNPoint for VNPoint {}
pub trait IVNPoint: Sized + std::ops::Deref {
    unsafe fn distanceToPoint_(&self, point: VNPoint) -> f64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, distanceToPoint : point)
    }
    unsafe fn initWithX_y_(&self, x: f64, y: f64) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithX : x, y : y)
    }
    unsafe fn initWithLocation_(&self, location: CGPoint) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithLocation : location)
    }
    unsafe fn location(&self) -> CGPoint
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, location)
    }
    unsafe fn x(&self) -> f64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, x)
    }
    unsafe fn y(&self) -> f64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, y)
    }
    unsafe fn pointByApplyingVector_toPoint_(vector: VNVector, point: VNPoint) -> VNPoint
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"VNPoint").unwrap(), pointByApplyingVector : vector, toPoint : point)
    }
    unsafe fn distanceBetweenPoint_point_(point1: VNPoint, point2: VNPoint) -> f64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"VNPoint").unwrap(), distanceBetweenPoint : point1, point : point2)
    }
    unsafe fn zeroPoint() -> VNPoint
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"VNPoint").unwrap(), zeroPoint)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct VNPoint3D(pub id);
impl std::ops::Deref for VNPoint3D {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for VNPoint3D {}
impl VNPoint3D {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"VNPoint3D").unwrap(), alloc) })
    }
}
impl PNSCopying for VNPoint3D {}
impl PNSSecureCoding for VNPoint3D {}
impl INSObject for VNPoint3D {}
impl PNSObject for VNPoint3D {}
impl std::convert::TryFrom<NSObject> for VNPoint3D {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<VNPoint3D, Self::Error> {
        let is_kind_of: bool = unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"VNPoint3D").unwrap()) };
        if is_kind_of {
            Ok(VNPoint3D(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to VNPoint3D")
        }
    }
}
impl IVNPoint3D for VNPoint3D {}
pub trait IVNPoint3D: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct VNVector(pub id);
impl std::ops::Deref for VNVector {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for VNVector {}
impl VNVector {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"VNVector").unwrap(), alloc) })
    }
}
impl PNSCopying for VNVector {}
impl PNSSecureCoding for VNVector {}
impl INSObject for VNVector {}
impl PNSObject for VNVector {}
impl std::convert::TryFrom<NSObject> for VNVector {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<VNVector, Self::Error> {
        let is_kind_of: bool = unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"VNVector").unwrap()) };
        if is_kind_of {
            Ok(VNVector(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to VNVector")
        }
    }
}
impl IVNVector for VNVector {}
pub trait IVNVector: Sized + std::ops::Deref {
    unsafe fn initWithXComponent_yComponent_(&self, x: f64, y: f64) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithXComponent : x, yComponent : y)
    }
    unsafe fn initWithR_theta_(&self, r: f64, theta: f64) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithR : r, theta : theta)
    }
    unsafe fn initWithVectorHead_tail_(&self, head: VNPoint, tail: VNPoint) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithVectorHead : head, tail : tail)
    }
    unsafe fn x(&self) -> f64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, x)
    }
    unsafe fn y(&self) -> f64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, y)
    }
    unsafe fn r(&self) -> f64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, r)
    }
    unsafe fn theta(&self) -> f64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, theta)
    }
    unsafe fn length(&self) -> f64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, length)
    }
    unsafe fn squaredLength(&self) -> f64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, squaredLength)
    }
    unsafe fn unitVectorForVector_(vector: VNVector) -> VNVector
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"VNVector").unwrap(), unitVectorForVector : vector)
    }
    unsafe fn vectorByMultiplyingVector_byScalar_(vector: VNVector, scalar: f64) -> VNVector
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"VNVector").unwrap(), vectorByMultiplyingVector : vector, byScalar : scalar)
    }
    unsafe fn vectorByAddingVector_toVector_(v1: VNVector, v2: VNVector) -> VNVector
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"VNVector").unwrap(), vectorByAddingVector : v1, toVector : v2)
    }
    unsafe fn vectorBySubtractingVector_fromVector_(v1: VNVector, v2: VNVector) -> VNVector
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"VNVector").unwrap(), vectorBySubtractingVector : v1, fromVector : v2)
    }
    unsafe fn dotProductOfVector_vector_(v1: VNVector, v2: VNVector) -> f64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"VNVector").unwrap(), dotProductOfVector : v1, vector : v2)
    }
    unsafe fn zeroVector() -> VNVector
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"VNVector").unwrap(), zeroVector)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct VNCircle(pub id);
impl std::ops::Deref for VNCircle {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for VNCircle {}
impl VNCircle {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"VNCircle").unwrap(), alloc) })
    }
}
impl PNSCopying for VNCircle {}
impl PNSSecureCoding for VNCircle {}
impl INSObject for VNCircle {}
impl PNSObject for VNCircle {}
impl std::convert::TryFrom<NSObject> for VNCircle {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<VNCircle, Self::Error> {
        let is_kind_of: bool = unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"VNCircle").unwrap()) };
        if is_kind_of {
            Ok(VNCircle(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to VNCircle")
        }
    }
}
impl IVNCircle for VNCircle {}
pub trait IVNCircle: Sized + std::ops::Deref {
    unsafe fn initWithCenter_radius_(&self, center: VNPoint, radius: f64) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithCenter : center, radius : radius)
    }
    unsafe fn initWithCenter_diameter_(&self, center: VNPoint, diameter: f64) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithCenter : center, diameter : diameter)
    }
    unsafe fn containsPoint_(&self, point: VNPoint) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, containsPoint : point)
    }
    unsafe fn containsPoint_inCircumferentialRingOfWidth_(
        &self,
        point: VNPoint,
        ringWidth: f64,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, containsPoint : point, inCircumferentialRingOfWidth : ringWidth)
    }
    unsafe fn center(&self) -> VNPoint
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, center)
    }
    unsafe fn radius(&self) -> f64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, radius)
    }
    unsafe fn diameter(&self) -> f64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, diameter)
    }
    unsafe fn zeroCircle() -> VNCircle
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"VNCircle").unwrap(), zeroCircle)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct VNContour(pub id);
impl std::ops::Deref for VNContour {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for VNContour {}
impl VNContour {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"VNContour").unwrap(), alloc) })
    }
}
impl PNSCopying for VNContour {}
impl PVNRequestRevisionProviding for VNContour {}
impl INSObject for VNContour {}
impl PNSObject for VNContour {}
impl std::convert::TryFrom<NSObject> for VNContour {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<VNContour, Self::Error> {
        let is_kind_of: bool = unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"VNContour").unwrap()) };
        if is_kind_of {
            Ok(VNContour(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to VNContour")
        }
    }
}
impl IVNContour for VNContour {}
pub trait IVNContour: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn childContourAtIndex_error_(
        &self,
        childContourIndex: NSUInteger,
        error: *mut NSError,
    ) -> VNContour
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, childContourAtIndex : childContourIndex, error : error)
    }
    unsafe fn polygonApproximationWithEpsilon_error_(
        &self,
        epsilon: f32,
        error: *mut NSError,
    ) -> VNContour
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, polygonApproximationWithEpsilon : epsilon, error : error)
    }
    unsafe fn indexPath(&self) -> NSIndexPath
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, indexPath)
    }
    unsafe fn childContourCount(&self) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, childContourCount)
    }
    unsafe fn childContours(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, childContours)
    }
    unsafe fn pointCount(&self) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, pointCount)
    }
    unsafe fn normalizedPoints(&self) -> *const simd_float2
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, normalizedPoints)
    }
    unsafe fn normalizedPath(&self) -> CGPathRef
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, normalizedPath)
    }
    unsafe fn aspectRatio(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, aspectRatio)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"VNContour").unwrap(), new)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct VNDetectedPoint(pub id);
impl std::ops::Deref for VNDetectedPoint {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for VNDetectedPoint {}
impl VNDetectedPoint {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"VNDetectedPoint").unwrap(), alloc) })
    }
}
impl IVNPoint for VNDetectedPoint {}
impl PNSCopying for VNDetectedPoint {}
impl PNSSecureCoding for VNDetectedPoint {}
impl From<VNDetectedPoint> for VNPoint {
    fn from(child: VNDetectedPoint) -> VNPoint {
        VNPoint(child.0)
    }
}
impl std::convert::TryFrom<VNPoint> for VNDetectedPoint {
    type Error = &'static str;
    fn try_from(parent: VNPoint) -> Result<VNDetectedPoint, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"VNDetectedPoint").unwrap()) };
        if is_kind_of {
            Ok(VNDetectedPoint(parent.0))
        } else {
            Err("This VNPoint cannot be downcasted to VNDetectedPoint")
        }
    }
}
impl INSObject for VNDetectedPoint {}
impl PNSObject for VNDetectedPoint {}
impl IVNDetectedPoint for VNDetectedPoint {}
pub trait IVNDetectedPoint: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn initWithX_y_(&self, x: f64, y: f64) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithX : x, y : y)
    }
    unsafe fn initWithLocation_(&self, location: CGPoint) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithLocation : location)
    }
    unsafe fn confidence(&self) -> VNConfidence
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, confidence)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"VNDetectedPoint").unwrap(), new)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct VNRecognizedPoint(pub id);
impl std::ops::Deref for VNRecognizedPoint {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for VNRecognizedPoint {}
impl VNRecognizedPoint {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"VNRecognizedPoint").unwrap(), alloc) })
    }
}
impl IVNDetectedPoint for VNRecognizedPoint {}
impl From<VNRecognizedPoint> for VNDetectedPoint {
    fn from(child: VNRecognizedPoint) -> VNDetectedPoint {
        VNDetectedPoint(child.0)
    }
}
impl std::convert::TryFrom<VNDetectedPoint> for VNRecognizedPoint {
    type Error = &'static str;
    fn try_from(parent: VNDetectedPoint) -> Result<VNRecognizedPoint, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"VNRecognizedPoint").unwrap()) };
        if is_kind_of {
            Ok(VNRecognizedPoint(parent.0))
        } else {
            Err("This VNDetectedPoint cannot be downcasted to VNRecognizedPoint")
        }
    }
}
impl IVNPoint for VNRecognizedPoint {}
impl PNSCopying for VNRecognizedPoint {}
impl PNSSecureCoding for VNRecognizedPoint {}
impl INSObject for VNRecognizedPoint {}
impl PNSObject for VNRecognizedPoint {}
impl IVNRecognizedPoint for VNRecognizedPoint {}
pub trait IVNRecognizedPoint: Sized + std::ops::Deref {
    unsafe fn identifier(&self) -> VNRecognizedPointKey
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, identifier)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct VNObservation(pub id);
impl std::ops::Deref for VNObservation {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for VNObservation {}
impl VNObservation {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"VNObservation").unwrap(), alloc) })
    }
}
impl PNSCopying for VNObservation {}
impl PNSSecureCoding for VNObservation {}
impl PVNRequestRevisionProviding for VNObservation {}
impl INSObject for VNObservation {}
impl PNSObject for VNObservation {}
impl std::convert::TryFrom<NSObject> for VNObservation {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<VNObservation, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"VNObservation").unwrap()) };
        if is_kind_of {
            Ok(VNObservation(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to VNObservation")
        }
    }
}
impl IVNObservation for VNObservation {}
pub trait IVNObservation: Sized + std::ops::Deref {
    unsafe fn uuid(&self) -> NSUUID
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, uuid)
    }
    unsafe fn confidence(&self) -> VNConfidence
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, confidence)
    }
    unsafe fn timeRange(&self) -> CMTimeRange
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, timeRange)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct VNDetectedObjectObservation(pub id);
impl std::ops::Deref for VNDetectedObjectObservation {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for VNDetectedObjectObservation {}
impl VNDetectedObjectObservation {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"VNDetectedObjectObservation").unwrap(), alloc) })
    }
}
impl IVNObservation for VNDetectedObjectObservation {}
impl PNSCopying for VNDetectedObjectObservation {}
impl PNSSecureCoding for VNDetectedObjectObservation {}
impl PVNRequestRevisionProviding for VNDetectedObjectObservation {}
impl From<VNDetectedObjectObservation> for VNObservation {
    fn from(child: VNDetectedObjectObservation) -> VNObservation {
        VNObservation(child.0)
    }
}
impl std::convert::TryFrom<VNObservation> for VNDetectedObjectObservation {
    type Error = &'static str;
    fn try_from(parent: VNObservation) -> Result<VNDetectedObjectObservation, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"VNDetectedObjectObservation").unwrap()) };
        if is_kind_of {
            Ok(VNDetectedObjectObservation(parent.0))
        } else {
            Err("This VNObservation cannot be downcasted to VNDetectedObjectObservation")
        }
    }
}
impl INSObject for VNDetectedObjectObservation {}
impl PNSObject for VNDetectedObjectObservation {}
impl IVNDetectedObjectObservation for VNDetectedObjectObservation {}
pub trait IVNDetectedObjectObservation: Sized + std::ops::Deref {
    unsafe fn boundingBox(&self) -> CGRect
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, boundingBox)
    }
    unsafe fn globalSegmentationMask(&self) -> VNPixelBufferObservation
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, globalSegmentationMask)
    }
    unsafe fn observationWithBoundingBox_(boundingBox: CGRect) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"VNDetectedObjectObservation").unwrap(), observationWithBoundingBox : boundingBox)
    }
    unsafe fn observationWithRequestRevision_boundingBox_(
        requestRevision: NSUInteger,
        boundingBox: CGRect,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"VNDetectedObjectObservation").unwrap(), observationWithRequestRevision : requestRevision, boundingBox : boundingBox)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct VNFaceObservation(pub id);
impl std::ops::Deref for VNFaceObservation {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for VNFaceObservation {}
impl VNFaceObservation {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"VNFaceObservation").unwrap(), alloc) })
    }
}
impl IVNDetectedObjectObservation for VNFaceObservation {}
impl From<VNFaceObservation> for VNDetectedObjectObservation {
    fn from(child: VNFaceObservation) -> VNDetectedObjectObservation {
        VNDetectedObjectObservation(child.0)
    }
}
impl std::convert::TryFrom<VNDetectedObjectObservation> for VNFaceObservation {
    type Error = &'static str;
    fn try_from(parent: VNDetectedObjectObservation) -> Result<VNFaceObservation, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"VNFaceObservation").unwrap()) };
        if is_kind_of {
            Ok(VNFaceObservation(parent.0))
        } else {
            Err("This VNDetectedObjectObservation cannot be downcasted to VNFaceObservation")
        }
    }
}
impl IVNObservation for VNFaceObservation {}
impl PNSCopying for VNFaceObservation {}
impl PNSSecureCoding for VNFaceObservation {}
impl PVNRequestRevisionProviding for VNFaceObservation {}
impl INSObject for VNFaceObservation {}
impl PNSObject for VNFaceObservation {}
impl IVNFaceObservation for VNFaceObservation {}
pub trait IVNFaceObservation: Sized + std::ops::Deref {
    unsafe fn landmarks(&self) -> VNFaceLandmarks2D
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, landmarks)
    }
    unsafe fn faceCaptureQuality(&self) -> NSNumber
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, faceCaptureQuality)
    }
    unsafe fn roll(&self) -> NSNumber
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, roll)
    }
    unsafe fn yaw(&self) -> NSNumber
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, yaw)
    }
    unsafe fn pitch(&self) -> NSNumber
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, pitch)
    }
    unsafe fn faceObservationWithRequestRevision_boundingBox_roll_yaw_(
        requestRevision: NSUInteger,
        boundingBox: CGRect,
        roll: NSNumber,
        yaw: NSNumber,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"VNFaceObservation").unwrap(), faceObservationWithRequestRevision : requestRevision, boundingBox : boundingBox, roll : roll, yaw : yaw)
    }
    unsafe fn faceObservationWithRequestRevision_boundingBox_roll_yaw_pitch_(
        requestRevision: NSUInteger,
        boundingBox: CGRect,
        roll: NSNumber,
        yaw: NSNumber,
        pitch: NSNumber,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"VNFaceObservation").unwrap(), faceObservationWithRequestRevision : requestRevision, boundingBox : boundingBox, roll : roll, yaw : yaw, pitch : pitch)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct VNClassificationObservation(pub id);
impl std::ops::Deref for VNClassificationObservation {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for VNClassificationObservation {}
impl VNClassificationObservation {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"VNClassificationObservation").unwrap(), alloc) })
    }
}
impl IVNObservation for VNClassificationObservation {}
impl PNSCopying for VNClassificationObservation {}
impl PNSSecureCoding for VNClassificationObservation {}
impl PVNRequestRevisionProviding for VNClassificationObservation {}
impl std::convert::TryFrom<VNObservation> for VNClassificationObservation {
    type Error = &'static str;
    fn try_from(parent: VNObservation) -> Result<VNClassificationObservation, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"VNClassificationObservation").unwrap()) };
        if is_kind_of {
            Ok(VNClassificationObservation(parent.0))
        } else {
            Err("This VNObservation cannot be downcasted to VNClassificationObservation")
        }
    }
}
impl INSObject for VNClassificationObservation {}
impl PNSObject for VNClassificationObservation {}
impl IVNClassificationObservation for VNClassificationObservation {}
pub trait IVNClassificationObservation: Sized + std::ops::Deref {
    unsafe fn identifier(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, identifier)
    }
}
impl VNClassificationObservation_PrecisionRecallAdditions for VNClassificationObservation {}
pub trait VNClassificationObservation_PrecisionRecallAdditions: Sized + std::ops::Deref {
    unsafe fn hasMinimumRecall_forPrecision_(&self, minimumRecall: f32, precision: f32) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, hasMinimumRecall : minimumRecall, forPrecision : precision)
    }
    unsafe fn hasMinimumPrecision_forRecall_(&self, minimumPrecision: f32, recall: f32) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, hasMinimumPrecision : minimumPrecision, forRecall : recall)
    }
    unsafe fn hasPrecisionRecallCurve(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, hasPrecisionRecallCurve)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct VNRecognizedObjectObservation(pub id);
impl std::ops::Deref for VNRecognizedObjectObservation {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for VNRecognizedObjectObservation {}
impl VNRecognizedObjectObservation {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"VNRecognizedObjectObservation").unwrap(), alloc) })
    }
}
impl IVNDetectedObjectObservation for VNRecognizedObjectObservation {}
impl std::convert::TryFrom<VNDetectedObjectObservation> for VNRecognizedObjectObservation {
    type Error = &'static str;
    fn try_from(
        parent: VNDetectedObjectObservation,
    ) -> Result<VNRecognizedObjectObservation, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"VNRecognizedObjectObservation").unwrap())
        };
        if is_kind_of {
            Ok(VNRecognizedObjectObservation(parent.0))
        } else {
            Err ("This VNDetectedObjectObservation cannot be downcasted to VNRecognizedObjectObservation" ,)
        }
    }
}
impl IVNObservation for VNRecognizedObjectObservation {}
impl PNSCopying for VNRecognizedObjectObservation {}
impl PNSSecureCoding for VNRecognizedObjectObservation {}
impl PVNRequestRevisionProviding for VNRecognizedObjectObservation {}
impl INSObject for VNRecognizedObjectObservation {}
impl PNSObject for VNRecognizedObjectObservation {}
impl IVNRecognizedObjectObservation for VNRecognizedObjectObservation {}
pub trait IVNRecognizedObjectObservation: Sized + std::ops::Deref {
    unsafe fn labels(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, labels)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct VNCoreMLFeatureValueObservation(pub id);
impl std::ops::Deref for VNCoreMLFeatureValueObservation {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for VNCoreMLFeatureValueObservation {}
impl VNCoreMLFeatureValueObservation {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"VNCoreMLFeatureValueObservation").unwrap(), alloc) })
    }
}
impl IVNObservation for VNCoreMLFeatureValueObservation {}
impl PNSCopying for VNCoreMLFeatureValueObservation {}
impl PNSSecureCoding for VNCoreMLFeatureValueObservation {}
impl PVNRequestRevisionProviding for VNCoreMLFeatureValueObservation {}
impl std::convert::TryFrom<VNObservation> for VNCoreMLFeatureValueObservation {
    type Error = &'static str;
    fn try_from(parent: VNObservation) -> Result<VNCoreMLFeatureValueObservation, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"VNCoreMLFeatureValueObservation").unwrap())
        };
        if is_kind_of {
            Ok(VNCoreMLFeatureValueObservation(parent.0))
        } else {
            Err("This VNObservation cannot be downcasted to VNCoreMLFeatureValueObservation")
        }
    }
}
impl INSObject for VNCoreMLFeatureValueObservation {}
impl PNSObject for VNCoreMLFeatureValueObservation {}
impl IVNCoreMLFeatureValueObservation for VNCoreMLFeatureValueObservation {}
pub trait IVNCoreMLFeatureValueObservation: Sized + std::ops::Deref {
    unsafe fn featureValue(&self) -> MLFeatureValue
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, featureValue)
    }
    unsafe fn featureName(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, featureName)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct VNPixelBufferObservation(pub id);
impl std::ops::Deref for VNPixelBufferObservation {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for VNPixelBufferObservation {}
impl VNPixelBufferObservation {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"VNPixelBufferObservation").unwrap(), alloc) })
    }
}
impl IVNObservation for VNPixelBufferObservation {}
impl PNSCopying for VNPixelBufferObservation {}
impl PNSSecureCoding for VNPixelBufferObservation {}
impl PVNRequestRevisionProviding for VNPixelBufferObservation {}
impl std::convert::TryFrom<VNObservation> for VNPixelBufferObservation {
    type Error = &'static str;
    fn try_from(parent: VNObservation) -> Result<VNPixelBufferObservation, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"VNPixelBufferObservation").unwrap()) };
        if is_kind_of {
            Ok(VNPixelBufferObservation(parent.0))
        } else {
            Err("This VNObservation cannot be downcasted to VNPixelBufferObservation")
        }
    }
}
impl INSObject for VNPixelBufferObservation {}
impl PNSObject for VNPixelBufferObservation {}
impl IVNPixelBufferObservation for VNPixelBufferObservation {}
pub trait IVNPixelBufferObservation: Sized + std::ops::Deref {
    unsafe fn pixelBuffer(&self) -> CVPixelBufferRef
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, pixelBuffer)
    }
    unsafe fn featureName(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, featureName)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct VNRectangleObservation(pub id);
impl std::ops::Deref for VNRectangleObservation {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for VNRectangleObservation {}
impl VNRectangleObservation {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"VNRectangleObservation").unwrap(), alloc) })
    }
}
impl IVNDetectedObjectObservation for VNRectangleObservation {}
impl std::convert::TryFrom<VNDetectedObjectObservation> for VNRectangleObservation {
    type Error = &'static str;
    fn try_from(
        parent: VNDetectedObjectObservation,
    ) -> Result<VNRectangleObservation, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"VNRectangleObservation").unwrap()) };
        if is_kind_of {
            Ok(VNRectangleObservation(parent.0))
        } else {
            Err("This VNDetectedObjectObservation cannot be downcasted to VNRectangleObservation")
        }
    }
}
impl IVNObservation for VNRectangleObservation {}
impl PNSCopying for VNRectangleObservation {}
impl PNSSecureCoding for VNRectangleObservation {}
impl PVNRequestRevisionProviding for VNRectangleObservation {}
impl INSObject for VNRectangleObservation {}
impl PNSObject for VNRectangleObservation {}
impl IVNRectangleObservation for VNRectangleObservation {}
pub trait IVNRectangleObservation: Sized + std::ops::Deref {
    unsafe fn topLeft(&self) -> CGPoint
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, topLeft)
    }
    unsafe fn topRight(&self) -> CGPoint
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, topRight)
    }
    unsafe fn bottomLeft(&self) -> CGPoint
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, bottomLeft)
    }
    unsafe fn bottomRight(&self) -> CGPoint
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, bottomRight)
    }
    unsafe fn rectangleObservationWithRequestRevision_topLeft_bottomLeft_bottomRight_topRight_(
        requestRevision: NSUInteger,
        topLeft: CGPoint,
        bottomLeft: CGPoint,
        bottomRight: CGPoint,
        topRight: CGPoint,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"VNRectangleObservation").unwrap(), rectangleObservationWithRequestRevision : requestRevision, topLeft : topLeft, bottomLeft : bottomLeft, bottomRight : bottomRight, topRight : topRight)
    }
    unsafe fn rectangleObservationWithRequestRevision_topLeft_topRight_bottomRight_bottomLeft_(
        requestRevision: NSUInteger,
        topLeft: CGPoint,
        topRight: CGPoint,
        bottomRight: CGPoint,
        bottomLeft: CGPoint,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"VNRectangleObservation").unwrap(), rectangleObservationWithRequestRevision : requestRevision, topLeft : topLeft, topRight : topRight, bottomRight : bottomRight, bottomLeft : bottomLeft)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct VNTrajectoryObservation(pub id);
impl std::ops::Deref for VNTrajectoryObservation {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for VNTrajectoryObservation {}
impl VNTrajectoryObservation {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"VNTrajectoryObservation").unwrap(), alloc) })
    }
}
impl IVNObservation for VNTrajectoryObservation {}
impl PNSCopying for VNTrajectoryObservation {}
impl PNSSecureCoding for VNTrajectoryObservation {}
impl PVNRequestRevisionProviding for VNTrajectoryObservation {}
impl std::convert::TryFrom<VNObservation> for VNTrajectoryObservation {
    type Error = &'static str;
    fn try_from(parent: VNObservation) -> Result<VNTrajectoryObservation, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"VNTrajectoryObservation").unwrap()) };
        if is_kind_of {
            Ok(VNTrajectoryObservation(parent.0))
        } else {
            Err("This VNObservation cannot be downcasted to VNTrajectoryObservation")
        }
    }
}
impl INSObject for VNTrajectoryObservation {}
impl PNSObject for VNTrajectoryObservation {}
impl IVNTrajectoryObservation for VNTrajectoryObservation {}
pub trait IVNTrajectoryObservation: Sized + std::ops::Deref {
    unsafe fn detectedPoints(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, detectedPoints)
    }
    unsafe fn projectedPoints(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, projectedPoints)
    }
    unsafe fn equationCoefficients(&self) -> simd_float3
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, equationCoefficients)
    }
    unsafe fn movingAverageRadius(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, movingAverageRadius)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct VNTextObservation(pub id);
impl std::ops::Deref for VNTextObservation {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for VNTextObservation {}
impl VNTextObservation {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"VNTextObservation").unwrap(), alloc) })
    }
}
impl IVNRectangleObservation for VNTextObservation {}
impl From<VNTextObservation> for VNRectangleObservation {
    fn from(child: VNTextObservation) -> VNRectangleObservation {
        VNRectangleObservation(child.0)
    }
}
impl std::convert::TryFrom<VNRectangleObservation> for VNTextObservation {
    type Error = &'static str;
    fn try_from(parent: VNRectangleObservation) -> Result<VNTextObservation, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"VNTextObservation").unwrap()) };
        if is_kind_of {
            Ok(VNTextObservation(parent.0))
        } else {
            Err("This VNRectangleObservation cannot be downcasted to VNTextObservation")
        }
    }
}
impl IVNDetectedObjectObservation for VNTextObservation {}
impl IVNObservation for VNTextObservation {}
impl PNSCopying for VNTextObservation {}
impl PNSSecureCoding for VNTextObservation {}
impl PVNRequestRevisionProviding for VNTextObservation {}
impl INSObject for VNTextObservation {}
impl PNSObject for VNTextObservation {}
impl IVNTextObservation for VNTextObservation {}
pub trait IVNTextObservation: Sized + std::ops::Deref {
    unsafe fn characterBoxes(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, characterBoxes)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct VNRecognizedText(pub id);
impl std::ops::Deref for VNRecognizedText {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for VNRecognizedText {}
impl VNRecognizedText {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"VNRecognizedText").unwrap(), alloc) })
    }
}
impl PNSCopying for VNRecognizedText {}
impl PNSSecureCoding for VNRecognizedText {}
impl PVNRequestRevisionProviding for VNRecognizedText {}
impl INSObject for VNRecognizedText {}
impl PNSObject for VNRecognizedText {}
impl std::convert::TryFrom<NSObject> for VNRecognizedText {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<VNRecognizedText, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"VNRecognizedText").unwrap()) };
        if is_kind_of {
            Ok(VNRecognizedText(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to VNRecognizedText")
        }
    }
}
impl IVNRecognizedText for VNRecognizedText {}
pub trait IVNRecognizedText: Sized + std::ops::Deref {
    unsafe fn boundingBoxForRange_error_(
        &self,
        range: NSRange,
        error: *mut NSError,
    ) -> VNRectangleObservation
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, boundingBoxForRange : range, error : error)
    }
    unsafe fn string(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, string)
    }
    unsafe fn confidence(&self) -> VNConfidence
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, confidence)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct VNRecognizedTextObservation(pub id);
impl std::ops::Deref for VNRecognizedTextObservation {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for VNRecognizedTextObservation {}
impl VNRecognizedTextObservation {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"VNRecognizedTextObservation").unwrap(), alloc) })
    }
}
impl IVNRectangleObservation for VNRecognizedTextObservation {}
impl std::convert::TryFrom<VNRectangleObservation> for VNRecognizedTextObservation {
    type Error = &'static str;
    fn try_from(
        parent: VNRectangleObservation,
    ) -> Result<VNRecognizedTextObservation, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"VNRecognizedTextObservation").unwrap()) };
        if is_kind_of {
            Ok(VNRecognizedTextObservation(parent.0))
        } else {
            Err("This VNRectangleObservation cannot be downcasted to VNRecognizedTextObservation")
        }
    }
}
impl IVNDetectedObjectObservation for VNRecognizedTextObservation {}
impl IVNObservation for VNRecognizedTextObservation {}
impl PNSCopying for VNRecognizedTextObservation {}
impl PNSSecureCoding for VNRecognizedTextObservation {}
impl PVNRequestRevisionProviding for VNRecognizedTextObservation {}
impl INSObject for VNRecognizedTextObservation {}
impl PNSObject for VNRecognizedTextObservation {}
impl IVNRecognizedTextObservation for VNRecognizedTextObservation {}
pub trait IVNRecognizedTextObservation: Sized + std::ops::Deref {
    unsafe fn topCandidates_(&self, maxCandidateCount: NSUInteger) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, topCandidates : maxCandidateCount)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct VNBarcodeObservation(pub id);
impl std::ops::Deref for VNBarcodeObservation {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for VNBarcodeObservation {}
impl VNBarcodeObservation {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"VNBarcodeObservation").unwrap(), alloc) })
    }
}
impl IVNRectangleObservation for VNBarcodeObservation {}
impl std::convert::TryFrom<VNRectangleObservation> for VNBarcodeObservation {
    type Error = &'static str;
    fn try_from(parent: VNRectangleObservation) -> Result<VNBarcodeObservation, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"VNBarcodeObservation").unwrap()) };
        if is_kind_of {
            Ok(VNBarcodeObservation(parent.0))
        } else {
            Err("This VNRectangleObservation cannot be downcasted to VNBarcodeObservation")
        }
    }
}
impl IVNDetectedObjectObservation for VNBarcodeObservation {}
impl IVNObservation for VNBarcodeObservation {}
impl PNSCopying for VNBarcodeObservation {}
impl PNSSecureCoding for VNBarcodeObservation {}
impl PVNRequestRevisionProviding for VNBarcodeObservation {}
impl INSObject for VNBarcodeObservation {}
impl PNSObject for VNBarcodeObservation {}
impl IVNBarcodeObservation for VNBarcodeObservation {}
pub trait IVNBarcodeObservation: Sized + std::ops::Deref {
    unsafe fn symbology(&self) -> VNBarcodeSymbology
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, symbology)
    }
    unsafe fn barcodeDescriptor(&self) -> CIBarcodeDescriptor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, barcodeDescriptor)
    }
    unsafe fn payloadStringValue(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, payloadStringValue)
    }
    unsafe fn payloadData(&self) -> NSData
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, payloadData)
    }
    unsafe fn isGS1DataCarrier(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isGS1DataCarrier)
    }
    unsafe fn isColorInverted(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isColorInverted)
    }
    unsafe fn supplementalCompositeType(&self) -> VNBarcodeCompositeType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, supplementalCompositeType)
    }
    unsafe fn supplementalPayloadString(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, supplementalPayloadString)
    }
    unsafe fn supplementalPayloadData(&self) -> NSData
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, supplementalPayloadData)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct VNHorizonObservation(pub id);
impl std::ops::Deref for VNHorizonObservation {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for VNHorizonObservation {}
impl VNHorizonObservation {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"VNHorizonObservation").unwrap(), alloc) })
    }
}
impl IVNObservation for VNHorizonObservation {}
impl PNSCopying for VNHorizonObservation {}
impl PNSSecureCoding for VNHorizonObservation {}
impl PVNRequestRevisionProviding for VNHorizonObservation {}
impl std::convert::TryFrom<VNObservation> for VNHorizonObservation {
    type Error = &'static str;
    fn try_from(parent: VNObservation) -> Result<VNHorizonObservation, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"VNHorizonObservation").unwrap()) };
        if is_kind_of {
            Ok(VNHorizonObservation(parent.0))
        } else {
            Err("This VNObservation cannot be downcasted to VNHorizonObservation")
        }
    }
}
impl INSObject for VNHorizonObservation {}
impl PNSObject for VNHorizonObservation {}
impl IVNHorizonObservation for VNHorizonObservation {}
pub trait IVNHorizonObservation: Sized + std::ops::Deref {
    unsafe fn transformForImageWidth_height_(
        &self,
        width: usize,
        height: usize,
    ) -> CGAffineTransform
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, transformForImageWidth : width, height : height)
    }
    unsafe fn transform(&self) -> CGAffineTransform
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, transform)
    }
    unsafe fn angle(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, angle)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct VNImageAlignmentObservation(pub id);
impl std::ops::Deref for VNImageAlignmentObservation {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for VNImageAlignmentObservation {}
impl VNImageAlignmentObservation {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"VNImageAlignmentObservation").unwrap(), alloc) })
    }
}
impl IVNObservation for VNImageAlignmentObservation {}
impl PNSCopying for VNImageAlignmentObservation {}
impl PNSSecureCoding for VNImageAlignmentObservation {}
impl PVNRequestRevisionProviding for VNImageAlignmentObservation {}
impl std::convert::TryFrom<VNObservation> for VNImageAlignmentObservation {
    type Error = &'static str;
    fn try_from(parent: VNObservation) -> Result<VNImageAlignmentObservation, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"VNImageAlignmentObservation").unwrap()) };
        if is_kind_of {
            Ok(VNImageAlignmentObservation(parent.0))
        } else {
            Err("This VNObservation cannot be downcasted to VNImageAlignmentObservation")
        }
    }
}
impl INSObject for VNImageAlignmentObservation {}
impl PNSObject for VNImageAlignmentObservation {}
impl IVNImageAlignmentObservation for VNImageAlignmentObservation {}
pub trait IVNImageAlignmentObservation: Sized + std::ops::Deref {}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct VNImageTranslationAlignmentObservation(pub id);
impl std::ops::Deref for VNImageTranslationAlignmentObservation {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for VNImageTranslationAlignmentObservation {}
impl VNImageTranslationAlignmentObservation {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"VNImageTranslationAlignmentObservation").unwrap(), alloc) })
    }
}
impl IVNImageAlignmentObservation for VNImageTranslationAlignmentObservation {}
impl From<VNImageTranslationAlignmentObservation> for VNImageAlignmentObservation {
    fn from(child: VNImageTranslationAlignmentObservation) -> VNImageAlignmentObservation {
        VNImageAlignmentObservation(child.0)
    }
}
impl std::convert::TryFrom<VNImageAlignmentObservation> for VNImageTranslationAlignmentObservation {
    type Error = &'static str;
    fn try_from(
        parent: VNImageAlignmentObservation,
    ) -> Result<VNImageTranslationAlignmentObservation, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"VNImageTranslationAlignmentObservation").unwrap())
        };
        if is_kind_of {
            Ok(VNImageTranslationAlignmentObservation(parent.0))
        } else {
            Err ("This VNImageAlignmentObservation cannot be downcasted to VNImageTranslationAlignmentObservation" ,)
        }
    }
}
impl IVNObservation for VNImageTranslationAlignmentObservation {}
impl PNSCopying for VNImageTranslationAlignmentObservation {}
impl PNSSecureCoding for VNImageTranslationAlignmentObservation {}
impl PVNRequestRevisionProviding for VNImageTranslationAlignmentObservation {}
impl INSObject for VNImageTranslationAlignmentObservation {}
impl PNSObject for VNImageTranslationAlignmentObservation {}
impl IVNImageTranslationAlignmentObservation for VNImageTranslationAlignmentObservation {}
pub trait IVNImageTranslationAlignmentObservation: Sized + std::ops::Deref {
    unsafe fn alignmentTransform(&self) -> CGAffineTransform
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, alignmentTransform)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct VNImageHomographicAlignmentObservation(pub id);
impl std::ops::Deref for VNImageHomographicAlignmentObservation {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for VNImageHomographicAlignmentObservation {}
impl VNImageHomographicAlignmentObservation {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"VNImageHomographicAlignmentObservation").unwrap(), alloc) })
    }
}
impl IVNImageAlignmentObservation for VNImageHomographicAlignmentObservation {}
impl std::convert::TryFrom<VNImageAlignmentObservation> for VNImageHomographicAlignmentObservation {
    type Error = &'static str;
    fn try_from(
        parent: VNImageAlignmentObservation,
    ) -> Result<VNImageHomographicAlignmentObservation, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"VNImageHomographicAlignmentObservation").unwrap())
        };
        if is_kind_of {
            Ok(VNImageHomographicAlignmentObservation(parent.0))
        } else {
            Err ("This VNImageAlignmentObservation cannot be downcasted to VNImageHomographicAlignmentObservation" ,)
        }
    }
}
impl IVNObservation for VNImageHomographicAlignmentObservation {}
impl PNSCopying for VNImageHomographicAlignmentObservation {}
impl PNSSecureCoding for VNImageHomographicAlignmentObservation {}
impl PVNRequestRevisionProviding for VNImageHomographicAlignmentObservation {}
impl INSObject for VNImageHomographicAlignmentObservation {}
impl PNSObject for VNImageHomographicAlignmentObservation {}
impl IVNImageHomographicAlignmentObservation for VNImageHomographicAlignmentObservation {}
pub trait IVNImageHomographicAlignmentObservation: Sized + std::ops::Deref {
    unsafe fn warpTransform(&self) -> matrix_float3x3
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, warpTransform)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct VNSaliencyImageObservation(pub id);
impl std::ops::Deref for VNSaliencyImageObservation {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for VNSaliencyImageObservation {}
impl VNSaliencyImageObservation {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"VNSaliencyImageObservation").unwrap(), alloc) })
    }
}
impl IVNPixelBufferObservation for VNSaliencyImageObservation {}
impl From<VNSaliencyImageObservation> for VNPixelBufferObservation {
    fn from(child: VNSaliencyImageObservation) -> VNPixelBufferObservation {
        VNPixelBufferObservation(child.0)
    }
}
impl std::convert::TryFrom<VNPixelBufferObservation> for VNSaliencyImageObservation {
    type Error = &'static str;
    fn try_from(
        parent: VNPixelBufferObservation,
    ) -> Result<VNSaliencyImageObservation, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"VNSaliencyImageObservation").unwrap()) };
        if is_kind_of {
            Ok(VNSaliencyImageObservation(parent.0))
        } else {
            Err("This VNPixelBufferObservation cannot be downcasted to VNSaliencyImageObservation")
        }
    }
}
impl IVNObservation for VNSaliencyImageObservation {}
impl PNSCopying for VNSaliencyImageObservation {}
impl PNSSecureCoding for VNSaliencyImageObservation {}
impl PVNRequestRevisionProviding for VNSaliencyImageObservation {}
impl INSObject for VNSaliencyImageObservation {}
impl PNSObject for VNSaliencyImageObservation {}
impl IVNSaliencyImageObservation for VNSaliencyImageObservation {}
pub trait IVNSaliencyImageObservation: Sized + std::ops::Deref {
    unsafe fn salientObjects(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, salientObjects)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct VNFeaturePrintObservation(pub id);
impl std::ops::Deref for VNFeaturePrintObservation {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for VNFeaturePrintObservation {}
impl VNFeaturePrintObservation {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"VNFeaturePrintObservation").unwrap(), alloc) })
    }
}
impl IVNObservation for VNFeaturePrintObservation {}
impl PNSCopying for VNFeaturePrintObservation {}
impl PNSSecureCoding for VNFeaturePrintObservation {}
impl PVNRequestRevisionProviding for VNFeaturePrintObservation {}
impl std::convert::TryFrom<VNObservation> for VNFeaturePrintObservation {
    type Error = &'static str;
    fn try_from(parent: VNObservation) -> Result<VNFeaturePrintObservation, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"VNFeaturePrintObservation").unwrap()) };
        if is_kind_of {
            Ok(VNFeaturePrintObservation(parent.0))
        } else {
            Err("This VNObservation cannot be downcasted to VNFeaturePrintObservation")
        }
    }
}
impl INSObject for VNFeaturePrintObservation {}
impl PNSObject for VNFeaturePrintObservation {}
impl IVNFeaturePrintObservation for VNFeaturePrintObservation {}
pub trait IVNFeaturePrintObservation: Sized + std::ops::Deref {
    unsafe fn computeDistance_toFeaturePrintObservation_error_(
        &self,
        outDistance: *mut f32,
        featurePrint: VNFeaturePrintObservation,
        error: *mut NSError,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, computeDistance : outDistance, toFeaturePrintObservation : featurePrint, error : error)
    }
    unsafe fn elementType(&self) -> VNElementType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, elementType)
    }
    unsafe fn elementCount(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, elementCount)
    }
    unsafe fn data(&self) -> NSData
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, data)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct VNContoursObservation(pub id);
impl std::ops::Deref for VNContoursObservation {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for VNContoursObservation {}
impl VNContoursObservation {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"VNContoursObservation").unwrap(), alloc) })
    }
}
impl IVNObservation for VNContoursObservation {}
impl PNSCopying for VNContoursObservation {}
impl PNSSecureCoding for VNContoursObservation {}
impl PVNRequestRevisionProviding for VNContoursObservation {}
impl std::convert::TryFrom<VNObservation> for VNContoursObservation {
    type Error = &'static str;
    fn try_from(parent: VNObservation) -> Result<VNContoursObservation, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"VNContoursObservation").unwrap()) };
        if is_kind_of {
            Ok(VNContoursObservation(parent.0))
        } else {
            Err("This VNObservation cannot be downcasted to VNContoursObservation")
        }
    }
}
impl INSObject for VNContoursObservation {}
impl PNSObject for VNContoursObservation {}
impl IVNContoursObservation for VNContoursObservation {}
pub trait IVNContoursObservation: Sized + std::ops::Deref {
    unsafe fn contourAtIndex_error_(
        &self,
        contourIndex: NSInteger,
        error: *mut NSError,
    ) -> VNContour
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, contourAtIndex : contourIndex, error : error)
    }
    unsafe fn contourAtIndexPath_error_(
        &self,
        indexPath: NSIndexPath,
        error: *mut NSError,
    ) -> VNContour
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, contourAtIndexPath : indexPath, error : error)
    }
    unsafe fn contourCount(&self) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, contourCount)
    }
    unsafe fn topLevelContourCount(&self) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, topLevelContourCount)
    }
    unsafe fn topLevelContours(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, topLevelContours)
    }
    unsafe fn normalizedPath(&self) -> CGPathRef
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, normalizedPath)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct VNRecognizedPointsObservation(pub id);
impl std::ops::Deref for VNRecognizedPointsObservation {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for VNRecognizedPointsObservation {}
impl VNRecognizedPointsObservation {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"VNRecognizedPointsObservation").unwrap(), alloc) })
    }
}
impl IVNObservation for VNRecognizedPointsObservation {}
impl PNSCopying for VNRecognizedPointsObservation {}
impl PNSSecureCoding for VNRecognizedPointsObservation {}
impl PVNRequestRevisionProviding for VNRecognizedPointsObservation {}
impl std::convert::TryFrom<VNObservation> for VNRecognizedPointsObservation {
    type Error = &'static str;
    fn try_from(parent: VNObservation) -> Result<VNRecognizedPointsObservation, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"VNRecognizedPointsObservation").unwrap())
        };
        if is_kind_of {
            Ok(VNRecognizedPointsObservation(parent.0))
        } else {
            Err("This VNObservation cannot be downcasted to VNRecognizedPointsObservation")
        }
    }
}
impl INSObject for VNRecognizedPointsObservation {}
impl PNSObject for VNRecognizedPointsObservation {}
impl IVNRecognizedPointsObservation for VNRecognizedPointsObservation {}
pub trait IVNRecognizedPointsObservation: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn recognizedPointForKey_error_(
        &self,
        pointKey: NSString,
        error: *mut NSError,
    ) -> VNRecognizedPoint
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, recognizedPointForKey : pointKey, error : error)
    }
    unsafe fn recognizedPointsForGroupKey_error_(
        &self,
        groupKey: NSString,
        error: *mut NSError,
    ) -> NSDictionary
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, recognizedPointsForGroupKey : groupKey, error : error)
    }
    unsafe fn keypointsMultiArrayAndReturnError_(&self, error: *mut NSError) -> MLMultiArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, keypointsMultiArrayAndReturnError : error)
    }
    unsafe fn availableKeys(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, availableKeys)
    }
    unsafe fn availableGroupKeys(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, availableGroupKeys)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"VNRecognizedPointsObservation").unwrap(), new)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct VNHumanObservation(pub id);
impl std::ops::Deref for VNHumanObservation {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for VNHumanObservation {}
impl VNHumanObservation {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"VNHumanObservation").unwrap(), alloc) })
    }
}
impl IVNDetectedObjectObservation for VNHumanObservation {}
impl std::convert::TryFrom<VNDetectedObjectObservation> for VNHumanObservation {
    type Error = &'static str;
    fn try_from(parent: VNDetectedObjectObservation) -> Result<VNHumanObservation, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"VNHumanObservation").unwrap()) };
        if is_kind_of {
            Ok(VNHumanObservation(parent.0))
        } else {
            Err("This VNDetectedObjectObservation cannot be downcasted to VNHumanObservation")
        }
    }
}
impl IVNObservation for VNHumanObservation {}
impl PNSCopying for VNHumanObservation {}
impl PNSSecureCoding for VNHumanObservation {}
impl PVNRequestRevisionProviding for VNHumanObservation {}
impl INSObject for VNHumanObservation {}
impl PNSObject for VNHumanObservation {}
impl IVNHumanObservation for VNHumanObservation {}
pub trait IVNHumanObservation: Sized + std::ops::Deref {
    unsafe fn upperBodyOnly(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, upperBodyOnly)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct VNInstanceMaskObservation(pub id);
impl std::ops::Deref for VNInstanceMaskObservation {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for VNInstanceMaskObservation {}
impl VNInstanceMaskObservation {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"VNInstanceMaskObservation").unwrap(), alloc) })
    }
}
impl IVNObservation for VNInstanceMaskObservation {}
impl PNSCopying for VNInstanceMaskObservation {}
impl PNSSecureCoding for VNInstanceMaskObservation {}
impl PVNRequestRevisionProviding for VNInstanceMaskObservation {}
impl std::convert::TryFrom<VNObservation> for VNInstanceMaskObservation {
    type Error = &'static str;
    fn try_from(parent: VNObservation) -> Result<VNInstanceMaskObservation, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"VNInstanceMaskObservation").unwrap()) };
        if is_kind_of {
            Ok(VNInstanceMaskObservation(parent.0))
        } else {
            Err("This VNObservation cannot be downcasted to VNInstanceMaskObservation")
        }
    }
}
impl INSObject for VNInstanceMaskObservation {}
impl PNSObject for VNInstanceMaskObservation {}
impl IVNInstanceMaskObservation for VNInstanceMaskObservation {}
pub trait IVNInstanceMaskObservation: Sized + std::ops::Deref {
    unsafe fn generateMaskForInstances_error_(
        &self,
        instances: NSIndexSet,
        error: *mut NSError,
    ) -> CVPixelBufferRef
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, generateMaskForInstances : instances, error : error)
    }
    unsafe fn generateMaskedImageOfInstances_fromRequestHandler_croppedToInstancesExtent_error_(
        &self,
        instances: NSIndexSet,
        requestHandler: VNImageRequestHandler,
        cropResult: BOOL,
        error: *mut NSError,
    ) -> CVPixelBufferRef
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, generateMaskedImageOfInstances : instances, fromRequestHandler : requestHandler, croppedToInstancesExtent : cropResult, error : error)
    }
    unsafe fn generateScaledMaskForImageForInstances_fromRequestHandler_error_(
        &self,
        instances: NSIndexSet,
        requestHandler: VNImageRequestHandler,
        error: *mut NSError,
    ) -> CVPixelBufferRef
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, generateScaledMaskForImageForInstances : instances, fromRequestHandler : requestHandler, error : error)
    }
    unsafe fn instanceMask(&self) -> CVPixelBufferRef
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, instanceMask)
    }
    unsafe fn allInstances(&self) -> NSIndexSet
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, allInstances)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct VNAnimalBodyPoseObservation(pub id);
impl std::ops::Deref for VNAnimalBodyPoseObservation {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for VNAnimalBodyPoseObservation {}
impl VNAnimalBodyPoseObservation {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"VNAnimalBodyPoseObservation").unwrap(), alloc) })
    }
}
impl IVNRecognizedPointsObservation for VNAnimalBodyPoseObservation {}
impl From<VNAnimalBodyPoseObservation> for VNRecognizedPointsObservation {
    fn from(child: VNAnimalBodyPoseObservation) -> VNRecognizedPointsObservation {
        VNRecognizedPointsObservation(child.0)
    }
}
impl std::convert::TryFrom<VNRecognizedPointsObservation> for VNAnimalBodyPoseObservation {
    type Error = &'static str;
    fn try_from(
        parent: VNRecognizedPointsObservation,
    ) -> Result<VNAnimalBodyPoseObservation, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"VNAnimalBodyPoseObservation").unwrap()) };
        if is_kind_of {
            Ok(VNAnimalBodyPoseObservation(parent.0))
        } else {
            Err ("This VNRecognizedPointsObservation cannot be downcasted to VNAnimalBodyPoseObservation" ,)
        }
    }
}
impl IVNObservation for VNAnimalBodyPoseObservation {}
impl PNSCopying for VNAnimalBodyPoseObservation {}
impl PNSSecureCoding for VNAnimalBodyPoseObservation {}
impl PVNRequestRevisionProviding for VNAnimalBodyPoseObservation {}
impl INSObject for VNAnimalBodyPoseObservation {}
impl PNSObject for VNAnimalBodyPoseObservation {}
impl IVNAnimalBodyPoseObservation for VNAnimalBodyPoseObservation {}
pub trait IVNAnimalBodyPoseObservation: Sized + std::ops::Deref {
    unsafe fn recognizedPointForJointName_error_(
        &self,
        jointName: NSString,
        error: *mut NSError,
    ) -> VNRecognizedPoint
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, recognizedPointForJointName : jointName, error : error)
    }
    unsafe fn recognizedPointsForJointsGroupName_error_(
        &self,
        jointsGroupName: NSString,
        error: *mut NSError,
    ) -> NSDictionary
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, recognizedPointsForJointsGroupName : jointsGroupName, error : error)
    }
    unsafe fn availableJointNames(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, availableJointNames)
    }
    unsafe fn availableJointGroupNames(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, availableJointGroupNames)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct VNRecognizedPoints3DObservation(pub id);
impl std::ops::Deref for VNRecognizedPoints3DObservation {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for VNRecognizedPoints3DObservation {}
impl VNRecognizedPoints3DObservation {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"VNRecognizedPoints3DObservation").unwrap(), alloc) })
    }
}
impl IVNObservation for VNRecognizedPoints3DObservation {}
impl PNSCopying for VNRecognizedPoints3DObservation {}
impl PNSSecureCoding for VNRecognizedPoints3DObservation {}
impl PVNRequestRevisionProviding for VNRecognizedPoints3DObservation {}
impl std::convert::TryFrom<VNObservation> for VNRecognizedPoints3DObservation {
    type Error = &'static str;
    fn try_from(parent: VNObservation) -> Result<VNRecognizedPoints3DObservation, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"VNRecognizedPoints3DObservation").unwrap())
        };
        if is_kind_of {
            Ok(VNRecognizedPoints3DObservation(parent.0))
        } else {
            Err("This VNObservation cannot be downcasted to VNRecognizedPoints3DObservation")
        }
    }
}
impl INSObject for VNRecognizedPoints3DObservation {}
impl PNSObject for VNRecognizedPoints3DObservation {}
impl IVNRecognizedPoints3DObservation for VNRecognizedPoints3DObservation {}
pub trait IVNRecognizedPoints3DObservation: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn recognizedPointForKey_error_(
        &self,
        pointKey: NSString,
        error: *mut NSError,
    ) -> VNRecognizedPoint3D
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, recognizedPointForKey : pointKey, error : error)
    }
    unsafe fn recognizedPointsForGroupKey_error_(
        &self,
        groupKey: NSString,
        error: *mut NSError,
    ) -> NSDictionary
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, recognizedPointsForGroupKey : groupKey, error : error)
    }
    unsafe fn availableKeys(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, availableKeys)
    }
    unsafe fn availableGroupKeys(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, availableGroupKeys)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"VNRecognizedPoints3DObservation").unwrap(), new)
    }
}
pub type VNHumanBodyPose3DObservationHeightEstimation = NSInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct VNHumanBodyPose3DObservation(pub id);
impl std::ops::Deref for VNHumanBodyPose3DObservation {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for VNHumanBodyPose3DObservation {}
impl VNHumanBodyPose3DObservation {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"VNHumanBodyPose3DObservation").unwrap(), alloc) })
    }
}
impl IVNRecognizedPoints3DObservation for VNHumanBodyPose3DObservation {}
impl From<VNHumanBodyPose3DObservation> for VNRecognizedPoints3DObservation {
    fn from(child: VNHumanBodyPose3DObservation) -> VNRecognizedPoints3DObservation {
        VNRecognizedPoints3DObservation(child.0)
    }
}
impl std::convert::TryFrom<VNRecognizedPoints3DObservation> for VNHumanBodyPose3DObservation {
    type Error = &'static str;
    fn try_from(
        parent: VNRecognizedPoints3DObservation,
    ) -> Result<VNHumanBodyPose3DObservation, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"VNHumanBodyPose3DObservation").unwrap()) };
        if is_kind_of {
            Ok(VNHumanBodyPose3DObservation(parent.0))
        } else {
            Err ("This VNRecognizedPoints3DObservation cannot be downcasted to VNHumanBodyPose3DObservation" ,)
        }
    }
}
impl IVNObservation for VNHumanBodyPose3DObservation {}
impl PNSCopying for VNHumanBodyPose3DObservation {}
impl PNSSecureCoding for VNHumanBodyPose3DObservation {}
impl PVNRequestRevisionProviding for VNHumanBodyPose3DObservation {}
impl INSObject for VNHumanBodyPose3DObservation {}
impl PNSObject for VNHumanBodyPose3DObservation {}
impl IVNHumanBodyPose3DObservation for VNHumanBodyPose3DObservation {}
pub trait IVNHumanBodyPose3DObservation: Sized + std::ops::Deref {
    unsafe fn recognizedPointsForJointsGroupName_error_(
        &self,
        jointsGroupName: NSString,
        error: *mut NSError,
    ) -> NSDictionary
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, recognizedPointsForJointsGroupName : jointsGroupName, error : error)
    }
    unsafe fn recognizedPointForJointName_error_(
        &self,
        jointName: NSString,
        error: *mut NSError,
    ) -> VNHumanBodyRecognizedPoint3D
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, recognizedPointForJointName : jointName, error : error)
    }
    unsafe fn pointInImageForJointName_error_(
        &self,
        jointName: NSString,
        error: *mut NSError,
    ) -> VNPoint
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, pointInImageForJointName : jointName, error : error)
    }
    unsafe fn parentJointNameForJointName_(
        &self,
        jointName: NSString,
    ) -> VNHumanBodyPose3DObservationJointName
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, parentJointNameForJointName : jointName)
    }
    unsafe fn heightEstimation(&self) -> VNHumanBodyPose3DObservationHeightEstimation
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, heightEstimation)
    }
    unsafe fn availableJointsGroupNames(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, availableJointsGroupNames)
    }
    unsafe fn availableJointNames(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, availableJointNames)
    }
    unsafe fn bodyHeight(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, bodyHeight)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct VNImageAestheticsScoresObservation(pub id);
impl std::ops::Deref for VNImageAestheticsScoresObservation {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for VNImageAestheticsScoresObservation {}
impl VNImageAestheticsScoresObservation {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"VNImageAestheticsScoresObservation").unwrap(), alloc) })
    }
}
impl IVNObservation for VNImageAestheticsScoresObservation {}
impl PNSCopying for VNImageAestheticsScoresObservation {}
impl PNSSecureCoding for VNImageAestheticsScoresObservation {}
impl PVNRequestRevisionProviding for VNImageAestheticsScoresObservation {}
impl std::convert::TryFrom<VNObservation> for VNImageAestheticsScoresObservation {
    type Error = &'static str;
    fn try_from(parent: VNObservation) -> Result<VNImageAestheticsScoresObservation, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"VNImageAestheticsScoresObservation").unwrap())
        };
        if is_kind_of {
            Ok(VNImageAestheticsScoresObservation(parent.0))
        } else {
            Err("This VNObservation cannot be downcasted to VNImageAestheticsScoresObservation")
        }
    }
}
impl INSObject for VNImageAestheticsScoresObservation {}
impl PNSObject for VNImageAestheticsScoresObservation {}
impl IVNImageAestheticsScoresObservation for VNImageAestheticsScoresObservation {}
pub trait IVNImageAestheticsScoresObservation: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn isUtility(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isUtility)
    }
    unsafe fn overallScore(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, overallScore)
    }
}
pub type VNRequestCompletionHandler = *mut ::std::os::raw::c_void;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct VNRequest(pub id);
impl std::ops::Deref for VNRequest {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for VNRequest {}
impl VNRequest {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"VNRequest").unwrap(), alloc) })
    }
}
impl PNSCopying for VNRequest {}
impl INSObject for VNRequest {}
impl PNSObject for VNRequest {}
impl std::convert::TryFrom<NSObject> for VNRequest {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<VNRequest, Self::Error> {
        let is_kind_of: bool = unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"VNRequest").unwrap()) };
        if is_kind_of {
            Ok(VNRequest(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to VNRequest")
        }
    }
}
impl IVNRequest for VNRequest {}
pub trait IVNRequest: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn initWithCompletionHandler_(
        &self,
        completionHandler: VNRequestCompletionHandler,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithCompletionHandler : completionHandler)
    }
    unsafe fn cancel(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, cancel)
    }
    unsafe fn preferBackgroundProcessing(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, preferBackgroundProcessing)
    }
    unsafe fn setPreferBackgroundProcessing_(&self, preferBackgroundProcessing: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPreferBackgroundProcessing : preferBackgroundProcessing)
    }
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
    unsafe fn results(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, results)
    }
    unsafe fn completionHandler(&self) -> VNRequestCompletionHandler
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, completionHandler)
    }
    unsafe fn revision(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, revision)
    }
    unsafe fn setRevision_(&self, revision: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRevision : revision)
    }
    unsafe fn supportedRevisions() -> NSIndexSet
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"VNRequest").unwrap(), supportedRevisions)
    }
    unsafe fn defaultRevision() -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"VNRequest").unwrap(), defaultRevision)
    }
    unsafe fn currentRevision() -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"VNRequest").unwrap(), currentRevision)
    }
}
impl VNRequest_ for VNRequest {}
pub trait VNRequest_: Sized + std::ops::Deref {
    unsafe fn supportedComputeStageDevicesAndReturnError_(
        &self,
        error: *mut NSError,
    ) -> NSDictionary
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, supportedComputeStageDevicesAndReturnError : error)
    }
    unsafe fn computeDeviceForComputeStage_(&self, computeStage: NSString) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, computeDeviceForComputeStage : computeStage)
    }
    unsafe fn setComputeDevice_forComputeStage_(
        &self,
        computeDevice: *mut u64,
        computeStage: NSString,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setComputeDevice : computeDevice, forComputeStage : computeStage)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct VNImageBasedRequest(pub id);
impl std::ops::Deref for VNImageBasedRequest {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for VNImageBasedRequest {}
impl VNImageBasedRequest {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"VNImageBasedRequest").unwrap(), alloc) })
    }
}
impl IVNRequest for VNImageBasedRequest {}
impl PNSCopying for VNImageBasedRequest {}
impl From<VNImageBasedRequest> for VNRequest {
    fn from(child: VNImageBasedRequest) -> VNRequest {
        VNRequest(child.0)
    }
}
impl std::convert::TryFrom<VNRequest> for VNImageBasedRequest {
    type Error = &'static str;
    fn try_from(parent: VNRequest) -> Result<VNImageBasedRequest, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"VNImageBasedRequest").unwrap()) };
        if is_kind_of {
            Ok(VNImageBasedRequest(parent.0))
        } else {
            Err("This VNRequest cannot be downcasted to VNImageBasedRequest")
        }
    }
}
impl INSObject for VNImageBasedRequest {}
impl PNSObject for VNImageBasedRequest {}
impl IVNImageBasedRequest for VNImageBasedRequest {}
pub trait IVNImageBasedRequest: Sized + std::ops::Deref {
    unsafe fn regionOfInterest(&self) -> CGRect
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, regionOfInterest)
    }
    unsafe fn setRegionOfInterest_(&self, regionOfInterest: CGRect)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRegionOfInterest : regionOfInterest)
    }
}
pub type VNRequestProgressHandler = *mut ::std::os::raw::c_void;
pub trait PVNRequestProgressProviding: Sized + std::ops::Deref {
    unsafe fn progressHandler(&self) -> VNRequestProgressHandler
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, progressHandler)
    }
    unsafe fn setProgressHandler_(&self, progressHandler: VNRequestProgressHandler)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setProgressHandler : progressHandler)
    }
    unsafe fn indeterminate(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, indeterminate)
    }
}
pub trait PVNFaceObservationAccepting: Sized + std::ops::Deref {
    unsafe fn inputFaceObservations(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inputFaceObservations)
    }
    unsafe fn setInputFaceObservations_(&self, inputFaceObservations: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInputFaceObservations : inputFaceObservations)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct VNClassifyImageRequest(pub id);
impl std::ops::Deref for VNClassifyImageRequest {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for VNClassifyImageRequest {}
impl VNClassifyImageRequest {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"VNClassifyImageRequest").unwrap(), alloc) })
    }
}
impl IVNImageBasedRequest for VNClassifyImageRequest {}
impl From<VNClassifyImageRequest> for VNImageBasedRequest {
    fn from(child: VNClassifyImageRequest) -> VNImageBasedRequest {
        VNImageBasedRequest(child.0)
    }
}
impl std::convert::TryFrom<VNImageBasedRequest> for VNClassifyImageRequest {
    type Error = &'static str;
    fn try_from(parent: VNImageBasedRequest) -> Result<VNClassifyImageRequest, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"VNClassifyImageRequest").unwrap()) };
        if is_kind_of {
            Ok(VNClassifyImageRequest(parent.0))
        } else {
            Err("This VNImageBasedRequest cannot be downcasted to VNClassifyImageRequest")
        }
    }
}
impl IVNRequest for VNClassifyImageRequest {}
impl PNSCopying for VNClassifyImageRequest {}
impl INSObject for VNClassifyImageRequest {}
impl PNSObject for VNClassifyImageRequest {}
impl IVNClassifyImageRequest for VNClassifyImageRequest {}
pub trait IVNClassifyImageRequest: Sized + std::ops::Deref {
    unsafe fn supportedIdentifiersAndReturnError_(&self, error: *mut NSError) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, supportedIdentifiersAndReturnError : error)
    }
    unsafe fn results(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, results)
    }
    unsafe fn knownClassificationsForRevision_error_(
        requestRevision: NSUInteger,
        error: *mut NSError,
    ) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"VNClassifyImageRequest").unwrap(), knownClassificationsForRevision : requestRevision, error : error)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct VNDetectBarcodesRequest(pub id);
impl std::ops::Deref for VNDetectBarcodesRequest {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for VNDetectBarcodesRequest {}
impl VNDetectBarcodesRequest {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"VNDetectBarcodesRequest").unwrap(), alloc) })
    }
}
impl IVNImageBasedRequest for VNDetectBarcodesRequest {}
impl std::convert::TryFrom<VNImageBasedRequest> for VNDetectBarcodesRequest {
    type Error = &'static str;
    fn try_from(parent: VNImageBasedRequest) -> Result<VNDetectBarcodesRequest, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"VNDetectBarcodesRequest").unwrap()) };
        if is_kind_of {
            Ok(VNDetectBarcodesRequest(parent.0))
        } else {
            Err("This VNImageBasedRequest cannot be downcasted to VNDetectBarcodesRequest")
        }
    }
}
impl IVNRequest for VNDetectBarcodesRequest {}
impl PNSCopying for VNDetectBarcodesRequest {}
impl INSObject for VNDetectBarcodesRequest {}
impl PNSObject for VNDetectBarcodesRequest {}
impl IVNDetectBarcodesRequest for VNDetectBarcodesRequest {}
pub trait IVNDetectBarcodesRequest: Sized + std::ops::Deref {
    unsafe fn supportedSymbologiesAndReturnError_(&self, error: *mut NSError) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, supportedSymbologiesAndReturnError : error)
    }
    unsafe fn symbologies(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, symbologies)
    }
    unsafe fn setSymbologies_(&self, symbologies: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSymbologies : symbologies)
    }
    unsafe fn coalesceCompositeSymbologies(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, coalesceCompositeSymbologies)
    }
    unsafe fn setCoalesceCompositeSymbologies_(&self, coalesceCompositeSymbologies: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCoalesceCompositeSymbologies : coalesceCompositeSymbologies)
    }
    unsafe fn results(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, results)
    }
    unsafe fn supportedSymbologies() -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"VNDetectBarcodesRequest").unwrap(), supportedSymbologies)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct VNDetectFaceRectanglesRequest(pub id);
impl std::ops::Deref for VNDetectFaceRectanglesRequest {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for VNDetectFaceRectanglesRequest {}
impl VNDetectFaceRectanglesRequest {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"VNDetectFaceRectanglesRequest").unwrap(), alloc) })
    }
}
impl IVNImageBasedRequest for VNDetectFaceRectanglesRequest {}
impl std::convert::TryFrom<VNImageBasedRequest> for VNDetectFaceRectanglesRequest {
    type Error = &'static str;
    fn try_from(parent: VNImageBasedRequest) -> Result<VNDetectFaceRectanglesRequest, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"VNDetectFaceRectanglesRequest").unwrap())
        };
        if is_kind_of {
            Ok(VNDetectFaceRectanglesRequest(parent.0))
        } else {
            Err("This VNImageBasedRequest cannot be downcasted to VNDetectFaceRectanglesRequest")
        }
    }
}
impl IVNRequest for VNDetectFaceRectanglesRequest {}
impl PNSCopying for VNDetectFaceRectanglesRequest {}
impl INSObject for VNDetectFaceRectanglesRequest {}
impl PNSObject for VNDetectFaceRectanglesRequest {}
impl IVNDetectFaceRectanglesRequest for VNDetectFaceRectanglesRequest {}
pub trait IVNDetectFaceRectanglesRequest: Sized + std::ops::Deref {
    unsafe fn results(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, results)
    }
}
pub type VNRequestFaceLandmarksConstellation = NSUInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct VNDetectFaceLandmarksRequest(pub id);
impl std::ops::Deref for VNDetectFaceLandmarksRequest {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for VNDetectFaceLandmarksRequest {}
impl VNDetectFaceLandmarksRequest {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"VNDetectFaceLandmarksRequest").unwrap(), alloc) })
    }
}
impl PVNFaceObservationAccepting for VNDetectFaceLandmarksRequest {}
impl IVNImageBasedRequest for VNDetectFaceLandmarksRequest {}
impl std::convert::TryFrom<VNImageBasedRequest> for VNDetectFaceLandmarksRequest {
    type Error = &'static str;
    fn try_from(parent: VNImageBasedRequest) -> Result<VNDetectFaceLandmarksRequest, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"VNDetectFaceLandmarksRequest").unwrap()) };
        if is_kind_of {
            Ok(VNDetectFaceLandmarksRequest(parent.0))
        } else {
            Err("This VNImageBasedRequest cannot be downcasted to VNDetectFaceLandmarksRequest")
        }
    }
}
impl IVNRequest for VNDetectFaceLandmarksRequest {}
impl PNSCopying for VNDetectFaceLandmarksRequest {}
impl INSObject for VNDetectFaceLandmarksRequest {}
impl PNSObject for VNDetectFaceLandmarksRequest {}
impl IVNDetectFaceLandmarksRequest for VNDetectFaceLandmarksRequest {}
pub trait IVNDetectFaceLandmarksRequest: Sized + std::ops::Deref {
    unsafe fn constellation(&self) -> VNRequestFaceLandmarksConstellation
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, constellation)
    }
    unsafe fn setConstellation_(&self, constellation: VNRequestFaceLandmarksConstellation)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setConstellation : constellation)
    }
    unsafe fn results(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, results)
    }
    unsafe fn revision_supportsConstellation_(
        requestRevision: NSUInteger,
        constellation: VNRequestFaceLandmarksConstellation,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"VNDetectFaceLandmarksRequest").unwrap(), revision : requestRevision, supportsConstellation : constellation)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct VNDetectFaceCaptureQualityRequest(pub id);
impl std::ops::Deref for VNDetectFaceCaptureQualityRequest {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for VNDetectFaceCaptureQualityRequest {}
impl VNDetectFaceCaptureQualityRequest {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"VNDetectFaceCaptureQualityRequest").unwrap(), alloc) })
    }
}
impl PVNFaceObservationAccepting for VNDetectFaceCaptureQualityRequest {}
impl IVNImageBasedRequest for VNDetectFaceCaptureQualityRequest {}
impl std::convert::TryFrom<VNImageBasedRequest> for VNDetectFaceCaptureQualityRequest {
    type Error = &'static str;
    fn try_from(
        parent: VNImageBasedRequest,
    ) -> Result<VNDetectFaceCaptureQualityRequest, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"VNDetectFaceCaptureQualityRequest").unwrap())
        };
        if is_kind_of {
            Ok(VNDetectFaceCaptureQualityRequest(parent.0))
        } else {
            Err ("This VNImageBasedRequest cannot be downcasted to VNDetectFaceCaptureQualityRequest" ,)
        }
    }
}
impl IVNRequest for VNDetectFaceCaptureQualityRequest {}
impl PNSCopying for VNDetectFaceCaptureQualityRequest {}
impl INSObject for VNDetectFaceCaptureQualityRequest {}
impl PNSObject for VNDetectFaceCaptureQualityRequest {}
impl IVNDetectFaceCaptureQualityRequest for VNDetectFaceCaptureQualityRequest {}
pub trait IVNDetectFaceCaptureQualityRequest: Sized + std::ops::Deref {
    unsafe fn results(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, results)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct VNDetectHorizonRequest(pub id);
impl std::ops::Deref for VNDetectHorizonRequest {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for VNDetectHorizonRequest {}
impl VNDetectHorizonRequest {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"VNDetectHorizonRequest").unwrap(), alloc) })
    }
}
impl IVNImageBasedRequest for VNDetectHorizonRequest {}
impl std::convert::TryFrom<VNImageBasedRequest> for VNDetectHorizonRequest {
    type Error = &'static str;
    fn try_from(parent: VNImageBasedRequest) -> Result<VNDetectHorizonRequest, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"VNDetectHorizonRequest").unwrap()) };
        if is_kind_of {
            Ok(VNDetectHorizonRequest(parent.0))
        } else {
            Err("This VNImageBasedRequest cannot be downcasted to VNDetectHorizonRequest")
        }
    }
}
impl IVNRequest for VNDetectHorizonRequest {}
impl PNSCopying for VNDetectHorizonRequest {}
impl INSObject for VNDetectHorizonRequest {}
impl PNSObject for VNDetectHorizonRequest {}
impl IVNDetectHorizonRequest for VNDetectHorizonRequest {}
pub trait IVNDetectHorizonRequest: Sized + std::ops::Deref {
    unsafe fn results(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, results)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct VNDetectRectanglesRequest(pub id);
impl std::ops::Deref for VNDetectRectanglesRequest {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for VNDetectRectanglesRequest {}
impl VNDetectRectanglesRequest {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"VNDetectRectanglesRequest").unwrap(), alloc) })
    }
}
impl IVNImageBasedRequest for VNDetectRectanglesRequest {}
impl std::convert::TryFrom<VNImageBasedRequest> for VNDetectRectanglesRequest {
    type Error = &'static str;
    fn try_from(parent: VNImageBasedRequest) -> Result<VNDetectRectanglesRequest, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"VNDetectRectanglesRequest").unwrap()) };
        if is_kind_of {
            Ok(VNDetectRectanglesRequest(parent.0))
        } else {
            Err("This VNImageBasedRequest cannot be downcasted to VNDetectRectanglesRequest")
        }
    }
}
impl IVNRequest for VNDetectRectanglesRequest {}
impl PNSCopying for VNDetectRectanglesRequest {}
impl INSObject for VNDetectRectanglesRequest {}
impl PNSObject for VNDetectRectanglesRequest {}
impl IVNDetectRectanglesRequest for VNDetectRectanglesRequest {}
pub trait IVNDetectRectanglesRequest: Sized + std::ops::Deref {
    unsafe fn minimumAspectRatio(&self) -> VNAspectRatio
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, minimumAspectRatio)
    }
    unsafe fn setMinimumAspectRatio_(&self, minimumAspectRatio: VNAspectRatio)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMinimumAspectRatio : minimumAspectRatio)
    }
    unsafe fn maximumAspectRatio(&self) -> VNAspectRatio
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, maximumAspectRatio)
    }
    unsafe fn setMaximumAspectRatio_(&self, maximumAspectRatio: VNAspectRatio)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMaximumAspectRatio : maximumAspectRatio)
    }
    unsafe fn quadratureTolerance(&self) -> VNDegrees
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, quadratureTolerance)
    }
    unsafe fn setQuadratureTolerance_(&self, quadratureTolerance: VNDegrees)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setQuadratureTolerance : quadratureTolerance)
    }
    unsafe fn minimumSize(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, minimumSize)
    }
    unsafe fn setMinimumSize_(&self, minimumSize: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMinimumSize : minimumSize)
    }
    unsafe fn minimumConfidence(&self) -> VNConfidence
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, minimumConfidence)
    }
    unsafe fn setMinimumConfidence_(&self, minimumConfidence: VNConfidence)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMinimumConfidence : minimumConfidence)
    }
    unsafe fn maximumObservations(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, maximumObservations)
    }
    unsafe fn setMaximumObservations_(&self, maximumObservations: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMaximumObservations : maximumObservations)
    }
    unsafe fn results(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, results)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct VNDetectTextRectanglesRequest(pub id);
impl std::ops::Deref for VNDetectTextRectanglesRequest {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for VNDetectTextRectanglesRequest {}
impl VNDetectTextRectanglesRequest {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"VNDetectTextRectanglesRequest").unwrap(), alloc) })
    }
}
impl IVNImageBasedRequest for VNDetectTextRectanglesRequest {}
impl std::convert::TryFrom<VNImageBasedRequest> for VNDetectTextRectanglesRequest {
    type Error = &'static str;
    fn try_from(parent: VNImageBasedRequest) -> Result<VNDetectTextRectanglesRequest, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"VNDetectTextRectanglesRequest").unwrap())
        };
        if is_kind_of {
            Ok(VNDetectTextRectanglesRequest(parent.0))
        } else {
            Err("This VNImageBasedRequest cannot be downcasted to VNDetectTextRectanglesRequest")
        }
    }
}
impl IVNRequest for VNDetectTextRectanglesRequest {}
impl PNSCopying for VNDetectTextRectanglesRequest {}
impl INSObject for VNDetectTextRectanglesRequest {}
impl PNSObject for VNDetectTextRectanglesRequest {}
impl IVNDetectTextRectanglesRequest for VNDetectTextRectanglesRequest {}
pub trait IVNDetectTextRectanglesRequest: Sized + std::ops::Deref {
    unsafe fn reportCharacterBoxes(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, reportCharacterBoxes)
    }
    unsafe fn setReportCharacterBoxes_(&self, reportCharacterBoxes: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setReportCharacterBoxes : reportCharacterBoxes)
    }
    unsafe fn results(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, results)
    }
}
pub type VNRequestTextRecognitionLevel = NSInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct VNRecognizeTextRequest(pub id);
impl std::ops::Deref for VNRecognizeTextRequest {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for VNRecognizeTextRequest {}
impl VNRecognizeTextRequest {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"VNRecognizeTextRequest").unwrap(), alloc) })
    }
}
impl PVNRequestProgressProviding for VNRecognizeTextRequest {}
impl IVNImageBasedRequest for VNRecognizeTextRequest {}
impl std::convert::TryFrom<VNImageBasedRequest> for VNRecognizeTextRequest {
    type Error = &'static str;
    fn try_from(parent: VNImageBasedRequest) -> Result<VNRecognizeTextRequest, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"VNRecognizeTextRequest").unwrap()) };
        if is_kind_of {
            Ok(VNRecognizeTextRequest(parent.0))
        } else {
            Err("This VNImageBasedRequest cannot be downcasted to VNRecognizeTextRequest")
        }
    }
}
impl IVNRequest for VNRecognizeTextRequest {}
impl PNSCopying for VNRecognizeTextRequest {}
impl INSObject for VNRecognizeTextRequest {}
impl PNSObject for VNRecognizeTextRequest {}
impl IVNRecognizeTextRequest for VNRecognizeTextRequest {}
pub trait IVNRecognizeTextRequest: Sized + std::ops::Deref {
    unsafe fn supportedRecognitionLanguagesAndReturnError_(&self, error: *mut NSError) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, supportedRecognitionLanguagesAndReturnError : error)
    }
    unsafe fn recognitionLanguages(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, recognitionLanguages)
    }
    unsafe fn setRecognitionLanguages_(&self, recognitionLanguages: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRecognitionLanguages : recognitionLanguages)
    }
    unsafe fn customWords(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, customWords)
    }
    unsafe fn setCustomWords_(&self, customWords: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCustomWords : customWords)
    }
    unsafe fn recognitionLevel(&self) -> VNRequestTextRecognitionLevel
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, recognitionLevel)
    }
    unsafe fn setRecognitionLevel_(&self, recognitionLevel: VNRequestTextRecognitionLevel)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRecognitionLevel : recognitionLevel)
    }
    unsafe fn usesLanguageCorrection(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, usesLanguageCorrection)
    }
    unsafe fn setUsesLanguageCorrection_(&self, usesLanguageCorrection: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setUsesLanguageCorrection : usesLanguageCorrection)
    }
    unsafe fn automaticallyDetectsLanguage(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, automaticallyDetectsLanguage)
    }
    unsafe fn setAutomaticallyDetectsLanguage_(&self, automaticallyDetectsLanguage: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAutomaticallyDetectsLanguage : automaticallyDetectsLanguage)
    }
    unsafe fn minimumTextHeight(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, minimumTextHeight)
    }
    unsafe fn setMinimumTextHeight_(&self, minimumTextHeight: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMinimumTextHeight : minimumTextHeight)
    }
    unsafe fn results(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, results)
    }
    unsafe fn supportedRecognitionLanguagesForTextRecognitionLevel_revision_error_(
        recognitionLevel: VNRequestTextRecognitionLevel,
        requestRevision: NSUInteger,
        error: *mut NSError,
    ) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"VNRecognizeTextRequest").unwrap(), supportedRecognitionLanguagesForTextRecognitionLevel : recognitionLevel, revision : requestRevision, error : error)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct VNGenerateAttentionBasedSaliencyImageRequest(pub id);
impl std::ops::Deref for VNGenerateAttentionBasedSaliencyImageRequest {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for VNGenerateAttentionBasedSaliencyImageRequest {}
impl VNGenerateAttentionBasedSaliencyImageRequest {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"VNGenerateAttentionBasedSaliencyImageRequest").unwrap(), alloc) })
    }
}
impl IVNImageBasedRequest for VNGenerateAttentionBasedSaliencyImageRequest {}
impl std::convert::TryFrom<VNImageBasedRequest> for VNGenerateAttentionBasedSaliencyImageRequest {
    type Error = &'static str;
    fn try_from(
        parent: VNImageBasedRequest,
    ) -> Result<VNGenerateAttentionBasedSaliencyImageRequest, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"VNGenerateAttentionBasedSaliencyImageRequest").unwrap())
        };
        if is_kind_of {
            Ok(VNGenerateAttentionBasedSaliencyImageRequest(parent.0))
        } else {
            Err ("This VNImageBasedRequest cannot be downcasted to VNGenerateAttentionBasedSaliencyImageRequest" ,)
        }
    }
}
impl IVNRequest for VNGenerateAttentionBasedSaliencyImageRequest {}
impl PNSCopying for VNGenerateAttentionBasedSaliencyImageRequest {}
impl INSObject for VNGenerateAttentionBasedSaliencyImageRequest {}
impl PNSObject for VNGenerateAttentionBasedSaliencyImageRequest {}
impl IVNGenerateAttentionBasedSaliencyImageRequest
    for VNGenerateAttentionBasedSaliencyImageRequest
{
}
pub trait IVNGenerateAttentionBasedSaliencyImageRequest: Sized + std::ops::Deref {
    unsafe fn results(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, results)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct VNGenerateObjectnessBasedSaliencyImageRequest(pub id);
impl std::ops::Deref for VNGenerateObjectnessBasedSaliencyImageRequest {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for VNGenerateObjectnessBasedSaliencyImageRequest {}
impl VNGenerateObjectnessBasedSaliencyImageRequest {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"VNGenerateObjectnessBasedSaliencyImageRequest").unwrap(), alloc) })
    }
}
impl IVNImageBasedRequest for VNGenerateObjectnessBasedSaliencyImageRequest {}
impl std::convert::TryFrom<VNImageBasedRequest> for VNGenerateObjectnessBasedSaliencyImageRequest {
    type Error = &'static str;
    fn try_from(
        parent: VNImageBasedRequest,
    ) -> Result<VNGenerateObjectnessBasedSaliencyImageRequest, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"VNGenerateObjectnessBasedSaliencyImageRequest").unwrap())
        };
        if is_kind_of {
            Ok(VNGenerateObjectnessBasedSaliencyImageRequest(parent.0))
        } else {
            Err ("This VNImageBasedRequest cannot be downcasted to VNGenerateObjectnessBasedSaliencyImageRequest" ,)
        }
    }
}
impl IVNRequest for VNGenerateObjectnessBasedSaliencyImageRequest {}
impl PNSCopying for VNGenerateObjectnessBasedSaliencyImageRequest {}
impl INSObject for VNGenerateObjectnessBasedSaliencyImageRequest {}
impl PNSObject for VNGenerateObjectnessBasedSaliencyImageRequest {}
impl IVNGenerateObjectnessBasedSaliencyImageRequest
    for VNGenerateObjectnessBasedSaliencyImageRequest
{
}
pub trait IVNGenerateObjectnessBasedSaliencyImageRequest: Sized + std::ops::Deref {
    unsafe fn results(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, results)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct VNGenerateImageFeaturePrintRequest(pub id);
impl std::ops::Deref for VNGenerateImageFeaturePrintRequest {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for VNGenerateImageFeaturePrintRequest {}
impl VNGenerateImageFeaturePrintRequest {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"VNGenerateImageFeaturePrintRequest").unwrap(), alloc) })
    }
}
impl IVNImageBasedRequest for VNGenerateImageFeaturePrintRequest {}
impl std::convert::TryFrom<VNImageBasedRequest> for VNGenerateImageFeaturePrintRequest {
    type Error = &'static str;
    fn try_from(
        parent: VNImageBasedRequest,
    ) -> Result<VNGenerateImageFeaturePrintRequest, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"VNGenerateImageFeaturePrintRequest").unwrap())
        };
        if is_kind_of {
            Ok(VNGenerateImageFeaturePrintRequest(parent.0))
        } else {
            Err ("This VNImageBasedRequest cannot be downcasted to VNGenerateImageFeaturePrintRequest" ,)
        }
    }
}
impl IVNRequest for VNGenerateImageFeaturePrintRequest {}
impl PNSCopying for VNGenerateImageFeaturePrintRequest {}
impl INSObject for VNGenerateImageFeaturePrintRequest {}
impl PNSObject for VNGenerateImageFeaturePrintRequest {}
impl IVNGenerateImageFeaturePrintRequest for VNGenerateImageFeaturePrintRequest {}
pub trait IVNGenerateImageFeaturePrintRequest: Sized + std::ops::Deref {
    unsafe fn imageCropAndScaleOption(&self) -> VNImageCropAndScaleOption
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, imageCropAndScaleOption)
    }
    unsafe fn setImageCropAndScaleOption_(&self, imageCropAndScaleOption: VNImageCropAndScaleOption)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setImageCropAndScaleOption : imageCropAndScaleOption)
    }
    unsafe fn results(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, results)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct VNCoreMLModel(pub id);
impl std::ops::Deref for VNCoreMLModel {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for VNCoreMLModel {}
impl VNCoreMLModel {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"VNCoreMLModel").unwrap(), alloc) })
    }
}
impl INSObject for VNCoreMLModel {}
impl PNSObject for VNCoreMLModel {}
impl std::convert::TryFrom<NSObject> for VNCoreMLModel {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<VNCoreMLModel, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"VNCoreMLModel").unwrap()) };
        if is_kind_of {
            Ok(VNCoreMLModel(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to VNCoreMLModel")
        }
    }
}
impl IVNCoreMLModel for VNCoreMLModel {}
pub trait IVNCoreMLModel: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn inputImageFeatureName(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inputImageFeatureName)
    }
    unsafe fn setInputImageFeatureName_(&self, inputImageFeatureName: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInputImageFeatureName : inputImageFeatureName)
    }
    unsafe fn featureProvider(&self) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, featureProvider)
    }
    unsafe fn setFeatureProvider_(&self, featureProvider: *mut u64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setFeatureProvider : featureProvider)
    }
    unsafe fn modelForMLModel_error_(model: MLModel, error: *mut NSError) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"VNCoreMLModel").unwrap(), modelForMLModel : model, error : error)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct VNCoreMLRequest(pub id);
impl std::ops::Deref for VNCoreMLRequest {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for VNCoreMLRequest {}
impl VNCoreMLRequest {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"VNCoreMLRequest").unwrap(), alloc) })
    }
}
impl IVNImageBasedRequest for VNCoreMLRequest {}
impl std::convert::TryFrom<VNImageBasedRequest> for VNCoreMLRequest {
    type Error = &'static str;
    fn try_from(parent: VNImageBasedRequest) -> Result<VNCoreMLRequest, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"VNCoreMLRequest").unwrap()) };
        if is_kind_of {
            Ok(VNCoreMLRequest(parent.0))
        } else {
            Err("This VNImageBasedRequest cannot be downcasted to VNCoreMLRequest")
        }
    }
}
impl IVNRequest for VNCoreMLRequest {}
impl PNSCopying for VNCoreMLRequest {}
impl INSObject for VNCoreMLRequest {}
impl PNSObject for VNCoreMLRequest {}
impl IVNCoreMLRequest for VNCoreMLRequest {}
pub trait IVNCoreMLRequest: Sized + std::ops::Deref {
    unsafe fn initWithModel_(&self, model: VNCoreMLModel) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithModel : model)
    }
    unsafe fn initWithModel_completionHandler_(
        &self,
        model: VNCoreMLModel,
        completionHandler: VNRequestCompletionHandler,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithModel : model, completionHandler : completionHandler)
    }
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn initWithCompletionHandler_(
        &self,
        completionHandler: VNRequestCompletionHandler,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithCompletionHandler : completionHandler)
    }
    unsafe fn model(&self) -> VNCoreMLModel
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, model)
    }
    unsafe fn imageCropAndScaleOption(&self) -> VNImageCropAndScaleOption
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, imageCropAndScaleOption)
    }
    unsafe fn setImageCropAndScaleOption_(&self, imageCropAndScaleOption: VNImageCropAndScaleOption)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setImageCropAndScaleOption : imageCropAndScaleOption)
    }
}
pub type VNImageOption = NSString;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct VNImageRequestHandler(pub id);
impl std::ops::Deref for VNImageRequestHandler {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for VNImageRequestHandler {}
impl VNImageRequestHandler {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"VNImageRequestHandler").unwrap(), alloc) })
    }
}
impl INSObject for VNImageRequestHandler {}
impl PNSObject for VNImageRequestHandler {}
impl std::convert::TryFrom<NSObject> for VNImageRequestHandler {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<VNImageRequestHandler, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"VNImageRequestHandler").unwrap()) };
        if is_kind_of {
            Ok(VNImageRequestHandler(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to VNImageRequestHandler")
        }
    }
}
impl IVNImageRequestHandler for VNImageRequestHandler {}
pub trait IVNImageRequestHandler: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn initWithCVPixelBuffer_options_(
        &self,
        pixelBuffer: CVPixelBufferRef,
        options: NSDictionary,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithCVPixelBuffer : pixelBuffer, options : options)
    }
    unsafe fn initWithCVPixelBuffer_orientation_options_(
        &self,
        pixelBuffer: CVPixelBufferRef,
        orientation: CGImagePropertyOrientation,
        options: NSDictionary,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithCVPixelBuffer : pixelBuffer, orientation : orientation, options : options)
    }
    unsafe fn initWithCVPixelBuffer_depthData_orientation_options_(
        &self,
        pixelBuffer: CVPixelBufferRef,
        depthData: AVDepthData,
        orientation: CGImagePropertyOrientation,
        options: NSDictionary,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithCVPixelBuffer : pixelBuffer, depthData : depthData, orientation : orientation, options : options)
    }
    unsafe fn initWithCGImage_options_(
        &self,
        image: CGImageRef,
        options: NSDictionary,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithCGImage : image, options : options)
    }
    unsafe fn initWithCGImage_orientation_options_(
        &self,
        image: CGImageRef,
        orientation: CGImagePropertyOrientation,
        options: NSDictionary,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithCGImage : image, orientation : orientation, options : options)
    }
    unsafe fn initWithCIImage_options_(&self, image: CIImage, options: NSDictionary) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithCIImage : image, options : options)
    }
    unsafe fn initWithCIImage_orientation_options_(
        &self,
        image: CIImage,
        orientation: CGImagePropertyOrientation,
        options: NSDictionary,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithCIImage : image, orientation : orientation, options : options)
    }
    unsafe fn initWithURL_options_(&self, imageURL: NSURL, options: NSDictionary) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithURL : imageURL, options : options)
    }
    unsafe fn initWithURL_orientation_options_(
        &self,
        imageURL: NSURL,
        orientation: CGImagePropertyOrientation,
        options: NSDictionary,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithURL : imageURL, orientation : orientation, options : options)
    }
    unsafe fn initWithData_options_(&self, imageData: NSData, options: NSDictionary) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithData : imageData, options : options)
    }
    unsafe fn initWithData_orientation_options_(
        &self,
        imageData: NSData,
        orientation: CGImagePropertyOrientation,
        options: NSDictionary,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithData : imageData, orientation : orientation, options : options)
    }
    unsafe fn initWithCMSampleBuffer_options_(
        &self,
        sampleBuffer: CMSampleBufferRef,
        options: NSDictionary,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithCMSampleBuffer : sampleBuffer, options : options)
    }
    unsafe fn initWithCMSampleBuffer_orientation_options_(
        &self,
        sampleBuffer: CMSampleBufferRef,
        orientation: CGImagePropertyOrientation,
        options: NSDictionary,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithCMSampleBuffer : sampleBuffer, orientation : orientation, options : options)
    }
    unsafe fn initWithCMSampleBuffer_depthData_orientation_options_(
        &self,
        sampleBuffer: CMSampleBufferRef,
        depthData: AVDepthData,
        orientation: CGImagePropertyOrientation,
        options: NSDictionary,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithCMSampleBuffer : sampleBuffer, depthData : depthData, orientation : orientation, options : options)
    }
    unsafe fn performRequests_error_(&self, requests: NSArray, error: *mut NSError) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, performRequests : requests, error : error)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct VNSequenceRequestHandler(pub id);
impl std::ops::Deref for VNSequenceRequestHandler {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for VNSequenceRequestHandler {}
impl VNSequenceRequestHandler {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"VNSequenceRequestHandler").unwrap(), alloc) })
    }
}
impl INSObject for VNSequenceRequestHandler {}
impl PNSObject for VNSequenceRequestHandler {}
impl std::convert::TryFrom<NSObject> for VNSequenceRequestHandler {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<VNSequenceRequestHandler, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"VNSequenceRequestHandler").unwrap()) };
        if is_kind_of {
            Ok(VNSequenceRequestHandler(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to VNSequenceRequestHandler")
        }
    }
}
impl IVNSequenceRequestHandler for VNSequenceRequestHandler {}
pub trait IVNSequenceRequestHandler: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn performRequests_onCVPixelBuffer_error_(
        &self,
        requests: NSArray,
        pixelBuffer: CVPixelBufferRef,
        error: *mut NSError,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, performRequests : requests, onCVPixelBuffer : pixelBuffer, error : error)
    }
    unsafe fn performRequests_onCVPixelBuffer_orientation_error_(
        &self,
        requests: NSArray,
        pixelBuffer: CVPixelBufferRef,
        orientation: CGImagePropertyOrientation,
        error: *mut NSError,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, performRequests : requests, onCVPixelBuffer : pixelBuffer, orientation : orientation, error : error)
    }
    unsafe fn performRequests_onCGImage_error_(
        &self,
        requests: NSArray,
        image: CGImageRef,
        error: *mut NSError,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, performRequests : requests, onCGImage : image, error : error)
    }
    unsafe fn performRequests_onCGImage_orientation_error_(
        &self,
        requests: NSArray,
        image: CGImageRef,
        orientation: CGImagePropertyOrientation,
        error: *mut NSError,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, performRequests : requests, onCGImage : image, orientation : orientation, error : error)
    }
    unsafe fn performRequests_onCIImage_error_(
        &self,
        requests: NSArray,
        image: CIImage,
        error: *mut NSError,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, performRequests : requests, onCIImage : image, error : error)
    }
    unsafe fn performRequests_onCIImage_orientation_error_(
        &self,
        requests: NSArray,
        image: CIImage,
        orientation: CGImagePropertyOrientation,
        error: *mut NSError,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, performRequests : requests, onCIImage : image, orientation : orientation, error : error)
    }
    unsafe fn performRequests_onImageURL_error_(
        &self,
        requests: NSArray,
        imageURL: NSURL,
        error: *mut NSError,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, performRequests : requests, onImageURL : imageURL, error : error)
    }
    unsafe fn performRequests_onImageURL_orientation_error_(
        &self,
        requests: NSArray,
        imageURL: NSURL,
        orientation: CGImagePropertyOrientation,
        error: *mut NSError,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, performRequests : requests, onImageURL : imageURL, orientation : orientation, error : error)
    }
    unsafe fn performRequests_onImageData_error_(
        &self,
        requests: NSArray,
        imageData: NSData,
        error: *mut NSError,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, performRequests : requests, onImageData : imageData, error : error)
    }
    unsafe fn performRequests_onImageData_orientation_error_(
        &self,
        requests: NSArray,
        imageData: NSData,
        orientation: CGImagePropertyOrientation,
        error: *mut NSError,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, performRequests : requests, onImageData : imageData, orientation : orientation, error : error)
    }
    unsafe fn performRequests_onCMSampleBuffer_error_(
        &self,
        requests: NSArray,
        sampleBuffer: CMSampleBufferRef,
        error: *mut NSError,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, performRequests : requests, onCMSampleBuffer : sampleBuffer, error : error)
    }
    unsafe fn performRequests_onCMSampleBuffer_orientation_error_(
        &self,
        requests: NSArray,
        sampleBuffer: CMSampleBufferRef,
        orientation: CGImagePropertyOrientation,
        error: *mut NSError,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, performRequests : requests, onCMSampleBuffer : sampleBuffer, orientation : orientation, error : error)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct VNTargetedImageRequest(pub id);
impl std::ops::Deref for VNTargetedImageRequest {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for VNTargetedImageRequest {}
impl VNTargetedImageRequest {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"VNTargetedImageRequest").unwrap(), alloc) })
    }
}
impl IVNImageBasedRequest for VNTargetedImageRequest {}
impl std::convert::TryFrom<VNImageBasedRequest> for VNTargetedImageRequest {
    type Error = &'static str;
    fn try_from(parent: VNImageBasedRequest) -> Result<VNTargetedImageRequest, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"VNTargetedImageRequest").unwrap()) };
        if is_kind_of {
            Ok(VNTargetedImageRequest(parent.0))
        } else {
            Err("This VNImageBasedRequest cannot be downcasted to VNTargetedImageRequest")
        }
    }
}
impl IVNRequest for VNTargetedImageRequest {}
impl PNSCopying for VNTargetedImageRequest {}
impl INSObject for VNTargetedImageRequest {}
impl PNSObject for VNTargetedImageRequest {}
impl IVNTargetedImageRequest for VNTargetedImageRequest {}
pub trait IVNTargetedImageRequest: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn initWithCompletionHandler_(
        &self,
        completionHandler: VNRequestCompletionHandler,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithCompletionHandler : completionHandler)
    }
    unsafe fn initWithTargetedCVPixelBuffer_options_(
        &self,
        pixelBuffer: CVPixelBufferRef,
        options: NSDictionary,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithTargetedCVPixelBuffer : pixelBuffer, options : options)
    }
    unsafe fn initWithTargetedCVPixelBuffer_options_completionHandler_(
        &self,
        pixelBuffer: CVPixelBufferRef,
        options: NSDictionary,
        completionHandler: VNRequestCompletionHandler,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithTargetedCVPixelBuffer : pixelBuffer, options : options, completionHandler : completionHandler)
    }
    unsafe fn initWithTargetedCVPixelBuffer_orientation_options_(
        &self,
        pixelBuffer: CVPixelBufferRef,
        orientation: CGImagePropertyOrientation,
        options: NSDictionary,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithTargetedCVPixelBuffer : pixelBuffer, orientation : orientation, options : options)
    }
    unsafe fn initWithTargetedCVPixelBuffer_orientation_options_completionHandler_(
        &self,
        pixelBuffer: CVPixelBufferRef,
        orientation: CGImagePropertyOrientation,
        options: NSDictionary,
        completionHandler: VNRequestCompletionHandler,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithTargetedCVPixelBuffer : pixelBuffer, orientation : orientation, options : options, completionHandler : completionHandler)
    }
    unsafe fn initWithTargetedCGImage_options_(
        &self,
        cgImage: CGImageRef,
        options: NSDictionary,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithTargetedCGImage : cgImage, options : options)
    }
    unsafe fn initWithTargetedCGImage_options_completionHandler_(
        &self,
        cgImage: CGImageRef,
        options: NSDictionary,
        completionHandler: VNRequestCompletionHandler,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithTargetedCGImage : cgImage, options : options, completionHandler : completionHandler)
    }
    unsafe fn initWithTargetedCGImage_orientation_options_(
        &self,
        cgImage: CGImageRef,
        orientation: CGImagePropertyOrientation,
        options: NSDictionary,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithTargetedCGImage : cgImage, orientation : orientation, options : options)
    }
    unsafe fn initWithTargetedCGImage_orientation_options_completionHandler_(
        &self,
        cgImage: CGImageRef,
        orientation: CGImagePropertyOrientation,
        options: NSDictionary,
        completionHandler: VNRequestCompletionHandler,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithTargetedCGImage : cgImage, orientation : orientation, options : options, completionHandler : completionHandler)
    }
    unsafe fn initWithTargetedCIImage_options_(
        &self,
        ciImage: CIImage,
        options: NSDictionary,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithTargetedCIImage : ciImage, options : options)
    }
    unsafe fn initWithTargetedCIImage_options_completionHandler_(
        &self,
        ciImage: CIImage,
        options: NSDictionary,
        completionHandler: VNRequestCompletionHandler,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithTargetedCIImage : ciImage, options : options, completionHandler : completionHandler)
    }
    unsafe fn initWithTargetedCIImage_orientation_options_(
        &self,
        ciImage: CIImage,
        orientation: CGImagePropertyOrientation,
        options: NSDictionary,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithTargetedCIImage : ciImage, orientation : orientation, options : options)
    }
    unsafe fn initWithTargetedCIImage_orientation_options_completionHandler_(
        &self,
        ciImage: CIImage,
        orientation: CGImagePropertyOrientation,
        options: NSDictionary,
        completionHandler: VNRequestCompletionHandler,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithTargetedCIImage : ciImage, orientation : orientation, options : options, completionHandler : completionHandler)
    }
    unsafe fn initWithTargetedImageURL_options_(
        &self,
        imageURL: NSURL,
        options: NSDictionary,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithTargetedImageURL : imageURL, options : options)
    }
    unsafe fn initWithTargetedImageURL_options_completionHandler_(
        &self,
        imageURL: NSURL,
        options: NSDictionary,
        completionHandler: VNRequestCompletionHandler,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithTargetedImageURL : imageURL, options : options, completionHandler : completionHandler)
    }
    unsafe fn initWithTargetedImageURL_orientation_options_(
        &self,
        imageURL: NSURL,
        orientation: CGImagePropertyOrientation,
        options: NSDictionary,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithTargetedImageURL : imageURL, orientation : orientation, options : options)
    }
    unsafe fn initWithTargetedImageURL_orientation_options_completionHandler_(
        &self,
        imageURL: NSURL,
        orientation: CGImagePropertyOrientation,
        options: NSDictionary,
        completionHandler: VNRequestCompletionHandler,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithTargetedImageURL : imageURL, orientation : orientation, options : options, completionHandler : completionHandler)
    }
    unsafe fn initWithTargetedImageData_options_(
        &self,
        imageData: NSData,
        options: NSDictionary,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithTargetedImageData : imageData, options : options)
    }
    unsafe fn initWithTargetedImageData_options_completionHandler_(
        &self,
        imageData: NSData,
        options: NSDictionary,
        completionHandler: VNRequestCompletionHandler,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithTargetedImageData : imageData, options : options, completionHandler : completionHandler)
    }
    unsafe fn initWithTargetedImageData_orientation_options_(
        &self,
        imageData: NSData,
        orientation: CGImagePropertyOrientation,
        options: NSDictionary,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithTargetedImageData : imageData, orientation : orientation, options : options)
    }
    unsafe fn initWithTargetedImageData_orientation_options_completionHandler_(
        &self,
        imageData: NSData,
        orientation: CGImagePropertyOrientation,
        options: NSDictionary,
        completionHandler: VNRequestCompletionHandler,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithTargetedImageData : imageData, orientation : orientation, options : options, completionHandler : completionHandler)
    }
    unsafe fn initWithTargetedCMSampleBuffer_options_(
        &self,
        sampleBuffer: CMSampleBufferRef,
        options: NSDictionary,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithTargetedCMSampleBuffer : sampleBuffer, options : options)
    }
    unsafe fn initWithTargetedCMSampleBuffer_options_completionHandler_(
        &self,
        sampleBuffer: CMSampleBufferRef,
        options: NSDictionary,
        completionHandler: VNRequestCompletionHandler,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithTargetedCMSampleBuffer : sampleBuffer, options : options, completionHandler : completionHandler)
    }
    unsafe fn initWithTargetedCMSampleBuffer_orientation_options_(
        &self,
        sampleBuffer: CMSampleBufferRef,
        orientation: CGImagePropertyOrientation,
        options: NSDictionary,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithTargetedCMSampleBuffer : sampleBuffer, orientation : orientation, options : options)
    }
    unsafe fn initWithTargetedCMSampleBuffer_orientation_options_completionHandler_(
        &self,
        sampleBuffer: CMSampleBufferRef,
        orientation: CGImagePropertyOrientation,
        options: NSDictionary,
        completionHandler: VNRequestCompletionHandler,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithTargetedCMSampleBuffer : sampleBuffer, orientation : orientation, options : options, completionHandler : completionHandler)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct VNImageRegistrationRequest(pub id);
impl std::ops::Deref for VNImageRegistrationRequest {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for VNImageRegistrationRequest {}
impl VNImageRegistrationRequest {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"VNImageRegistrationRequest").unwrap(), alloc) })
    }
}
impl IVNTargetedImageRequest for VNImageRegistrationRequest {}
impl From<VNImageRegistrationRequest> for VNTargetedImageRequest {
    fn from(child: VNImageRegistrationRequest) -> VNTargetedImageRequest {
        VNTargetedImageRequest(child.0)
    }
}
impl std::convert::TryFrom<VNTargetedImageRequest> for VNImageRegistrationRequest {
    type Error = &'static str;
    fn try_from(parent: VNTargetedImageRequest) -> Result<VNImageRegistrationRequest, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"VNImageRegistrationRequest").unwrap()) };
        if is_kind_of {
            Ok(VNImageRegistrationRequest(parent.0))
        } else {
            Err("This VNTargetedImageRequest cannot be downcasted to VNImageRegistrationRequest")
        }
    }
}
impl IVNImageBasedRequest for VNImageRegistrationRequest {}
impl IVNRequest for VNImageRegistrationRequest {}
impl PNSCopying for VNImageRegistrationRequest {}
impl INSObject for VNImageRegistrationRequest {}
impl PNSObject for VNImageRegistrationRequest {}
impl IVNImageRegistrationRequest for VNImageRegistrationRequest {}
pub trait IVNImageRegistrationRequest: Sized + std::ops::Deref {}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct VNTranslationalImageRegistrationRequest(pub id);
impl std::ops::Deref for VNTranslationalImageRegistrationRequest {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for VNTranslationalImageRegistrationRequest {}
impl VNTranslationalImageRegistrationRequest {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"VNTranslationalImageRegistrationRequest").unwrap(), alloc) })
    }
}
impl IVNImageRegistrationRequest for VNTranslationalImageRegistrationRequest {}
impl From<VNTranslationalImageRegistrationRequest> for VNImageRegistrationRequest {
    fn from(child: VNTranslationalImageRegistrationRequest) -> VNImageRegistrationRequest {
        VNImageRegistrationRequest(child.0)
    }
}
impl std::convert::TryFrom<VNImageRegistrationRequest> for VNTranslationalImageRegistrationRequest {
    type Error = &'static str;
    fn try_from(
        parent: VNImageRegistrationRequest,
    ) -> Result<VNTranslationalImageRegistrationRequest, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"VNTranslationalImageRegistrationRequest").unwrap())
        };
        if is_kind_of {
            Ok(VNTranslationalImageRegistrationRequest(parent.0))
        } else {
            Err ("This VNImageRegistrationRequest cannot be downcasted to VNTranslationalImageRegistrationRequest" ,)
        }
    }
}
impl IVNTargetedImageRequest for VNTranslationalImageRegistrationRequest {}
impl IVNImageBasedRequest for VNTranslationalImageRegistrationRequest {}
impl IVNRequest for VNTranslationalImageRegistrationRequest {}
impl PNSCopying for VNTranslationalImageRegistrationRequest {}
impl INSObject for VNTranslationalImageRegistrationRequest {}
impl PNSObject for VNTranslationalImageRegistrationRequest {}
impl IVNTranslationalImageRegistrationRequest for VNTranslationalImageRegistrationRequest {}
pub trait IVNTranslationalImageRegistrationRequest: Sized + std::ops::Deref {
    unsafe fn results(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, results)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct VNHomographicImageRegistrationRequest(pub id);
impl std::ops::Deref for VNHomographicImageRegistrationRequest {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for VNHomographicImageRegistrationRequest {}
impl VNHomographicImageRegistrationRequest {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"VNHomographicImageRegistrationRequest").unwrap(), alloc) })
    }
}
impl IVNImageRegistrationRequest for VNHomographicImageRegistrationRequest {}
impl std::convert::TryFrom<VNImageRegistrationRequest> for VNHomographicImageRegistrationRequest {
    type Error = &'static str;
    fn try_from(
        parent: VNImageRegistrationRequest,
    ) -> Result<VNHomographicImageRegistrationRequest, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"VNHomographicImageRegistrationRequest").unwrap())
        };
        if is_kind_of {
            Ok(VNHomographicImageRegistrationRequest(parent.0))
        } else {
            Err ("This VNImageRegistrationRequest cannot be downcasted to VNHomographicImageRegistrationRequest" ,)
        }
    }
}
impl IVNTargetedImageRequest for VNHomographicImageRegistrationRequest {}
impl IVNImageBasedRequest for VNHomographicImageRegistrationRequest {}
impl IVNRequest for VNHomographicImageRegistrationRequest {}
impl PNSCopying for VNHomographicImageRegistrationRequest {}
impl INSObject for VNHomographicImageRegistrationRequest {}
impl PNSObject for VNHomographicImageRegistrationRequest {}
impl IVNHomographicImageRegistrationRequest for VNHomographicImageRegistrationRequest {}
pub trait IVNHomographicImageRegistrationRequest: Sized + std::ops::Deref {
    unsafe fn results(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, results)
    }
}
pub type VNRequestTrackingLevel = NSUInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct VNTrackingRequest(pub id);
impl std::ops::Deref for VNTrackingRequest {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for VNTrackingRequest {}
impl VNTrackingRequest {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"VNTrackingRequest").unwrap(), alloc) })
    }
}
impl IVNImageBasedRequest for VNTrackingRequest {}
impl std::convert::TryFrom<VNImageBasedRequest> for VNTrackingRequest {
    type Error = &'static str;
    fn try_from(parent: VNImageBasedRequest) -> Result<VNTrackingRequest, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"VNTrackingRequest").unwrap()) };
        if is_kind_of {
            Ok(VNTrackingRequest(parent.0))
        } else {
            Err("This VNImageBasedRequest cannot be downcasted to VNTrackingRequest")
        }
    }
}
impl IVNRequest for VNTrackingRequest {}
impl PNSCopying for VNTrackingRequest {}
impl INSObject for VNTrackingRequest {}
impl PNSObject for VNTrackingRequest {}
impl IVNTrackingRequest for VNTrackingRequest {}
pub trait IVNTrackingRequest: Sized + std::ops::Deref {
    unsafe fn supportedNumberOfTrackersAndReturnError_(&self, error: *mut NSError) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, supportedNumberOfTrackersAndReturnError : error)
    }
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn initWithCompletionHandler_(
        &self,
        completionHandler: VNRequestCompletionHandler,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithCompletionHandler : completionHandler)
    }
    unsafe fn inputObservation(&self) -> VNDetectedObjectObservation
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inputObservation)
    }
    unsafe fn setInputObservation_(&self, inputObservation: VNDetectedObjectObservation)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInputObservation : inputObservation)
    }
    unsafe fn trackingLevel(&self) -> VNRequestTrackingLevel
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, trackingLevel)
    }
    unsafe fn setTrackingLevel_(&self, trackingLevel: VNRequestTrackingLevel)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setTrackingLevel : trackingLevel)
    }
    unsafe fn isLastFrame(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isLastFrame)
    }
    unsafe fn setLastFrame_(&self, lastFrame: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setLastFrame : lastFrame)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct VNTrackObjectRequest(pub id);
impl std::ops::Deref for VNTrackObjectRequest {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for VNTrackObjectRequest {}
impl VNTrackObjectRequest {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"VNTrackObjectRequest").unwrap(), alloc) })
    }
}
impl IVNTrackingRequest for VNTrackObjectRequest {}
impl From<VNTrackObjectRequest> for VNTrackingRequest {
    fn from(child: VNTrackObjectRequest) -> VNTrackingRequest {
        VNTrackingRequest(child.0)
    }
}
impl std::convert::TryFrom<VNTrackingRequest> for VNTrackObjectRequest {
    type Error = &'static str;
    fn try_from(parent: VNTrackingRequest) -> Result<VNTrackObjectRequest, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"VNTrackObjectRequest").unwrap()) };
        if is_kind_of {
            Ok(VNTrackObjectRequest(parent.0))
        } else {
            Err("This VNTrackingRequest cannot be downcasted to VNTrackObjectRequest")
        }
    }
}
impl IVNImageBasedRequest for VNTrackObjectRequest {}
impl IVNRequest for VNTrackObjectRequest {}
impl PNSCopying for VNTrackObjectRequest {}
impl INSObject for VNTrackObjectRequest {}
impl PNSObject for VNTrackObjectRequest {}
impl IVNTrackObjectRequest for VNTrackObjectRequest {}
pub trait IVNTrackObjectRequest: Sized + std::ops::Deref {
    unsafe fn initWithDetectedObjectObservation_(
        &self,
        observation: VNDetectedObjectObservation,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithDetectedObjectObservation : observation)
    }
    unsafe fn initWithDetectedObjectObservation_completionHandler_(
        &self,
        observation: VNDetectedObjectObservation,
        completionHandler: VNRequestCompletionHandler,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithDetectedObjectObservation : observation, completionHandler : completionHandler)
    }
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn initWithCompletionHandler_(
        &self,
        completionHandler: VNRequestCompletionHandler,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithCompletionHandler : completionHandler)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct VNTrackRectangleRequest(pub id);
impl std::ops::Deref for VNTrackRectangleRequest {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for VNTrackRectangleRequest {}
impl VNTrackRectangleRequest {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"VNTrackRectangleRequest").unwrap(), alloc) })
    }
}
impl IVNTrackingRequest for VNTrackRectangleRequest {}
impl std::convert::TryFrom<VNTrackingRequest> for VNTrackRectangleRequest {
    type Error = &'static str;
    fn try_from(parent: VNTrackingRequest) -> Result<VNTrackRectangleRequest, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"VNTrackRectangleRequest").unwrap()) };
        if is_kind_of {
            Ok(VNTrackRectangleRequest(parent.0))
        } else {
            Err("This VNTrackingRequest cannot be downcasted to VNTrackRectangleRequest")
        }
    }
}
impl IVNImageBasedRequest for VNTrackRectangleRequest {}
impl IVNRequest for VNTrackRectangleRequest {}
impl PNSCopying for VNTrackRectangleRequest {}
impl INSObject for VNTrackRectangleRequest {}
impl PNSObject for VNTrackRectangleRequest {}
impl IVNTrackRectangleRequest for VNTrackRectangleRequest {}
pub trait IVNTrackRectangleRequest: Sized + std::ops::Deref {
    unsafe fn initWithRectangleObservation_(
        &self,
        observation: VNRectangleObservation,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithRectangleObservation : observation)
    }
    unsafe fn initWithRectangleObservation_completionHandler_(
        &self,
        observation: VNRectangleObservation,
        completionHandler: VNRequestCompletionHandler,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithRectangleObservation : observation, completionHandler : completionHandler)
    }
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn initWithCompletionHandler_(
        &self,
        completionHandler: VNRequestCompletionHandler,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithCompletionHandler : completionHandler)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct VNDetectHumanRectanglesRequest(pub id);
impl std::ops::Deref for VNDetectHumanRectanglesRequest {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for VNDetectHumanRectanglesRequest {}
impl VNDetectHumanRectanglesRequest {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"VNDetectHumanRectanglesRequest").unwrap(), alloc) })
    }
}
impl IVNImageBasedRequest for VNDetectHumanRectanglesRequest {}
impl std::convert::TryFrom<VNImageBasedRequest> for VNDetectHumanRectanglesRequest {
    type Error = &'static str;
    fn try_from(
        parent: VNImageBasedRequest,
    ) -> Result<VNDetectHumanRectanglesRequest, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"VNDetectHumanRectanglesRequest").unwrap())
        };
        if is_kind_of {
            Ok(VNDetectHumanRectanglesRequest(parent.0))
        } else {
            Err("This VNImageBasedRequest cannot be downcasted to VNDetectHumanRectanglesRequest")
        }
    }
}
impl IVNRequest for VNDetectHumanRectanglesRequest {}
impl PNSCopying for VNDetectHumanRectanglesRequest {}
impl INSObject for VNDetectHumanRectanglesRequest {}
impl PNSObject for VNDetectHumanRectanglesRequest {}
impl IVNDetectHumanRectanglesRequest for VNDetectHumanRectanglesRequest {}
pub trait IVNDetectHumanRectanglesRequest: Sized + std::ops::Deref {
    unsafe fn upperBodyOnly(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, upperBodyOnly)
    }
    unsafe fn setUpperBodyOnly_(&self, upperBodyOnly: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setUpperBodyOnly : upperBodyOnly)
    }
    unsafe fn results(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, results)
    }
}
pub type VNAnimalIdentifier = NSString;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct VNRecognizeAnimalsRequest(pub id);
impl std::ops::Deref for VNRecognizeAnimalsRequest {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for VNRecognizeAnimalsRequest {}
impl VNRecognizeAnimalsRequest {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"VNRecognizeAnimalsRequest").unwrap(), alloc) })
    }
}
impl IVNImageBasedRequest for VNRecognizeAnimalsRequest {}
impl std::convert::TryFrom<VNImageBasedRequest> for VNRecognizeAnimalsRequest {
    type Error = &'static str;
    fn try_from(parent: VNImageBasedRequest) -> Result<VNRecognizeAnimalsRequest, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"VNRecognizeAnimalsRequest").unwrap()) };
        if is_kind_of {
            Ok(VNRecognizeAnimalsRequest(parent.0))
        } else {
            Err("This VNImageBasedRequest cannot be downcasted to VNRecognizeAnimalsRequest")
        }
    }
}
impl IVNRequest for VNRecognizeAnimalsRequest {}
impl PNSCopying for VNRecognizeAnimalsRequest {}
impl INSObject for VNRecognizeAnimalsRequest {}
impl PNSObject for VNRecognizeAnimalsRequest {}
impl IVNRecognizeAnimalsRequest for VNRecognizeAnimalsRequest {}
pub trait IVNRecognizeAnimalsRequest: Sized + std::ops::Deref {
    unsafe fn supportedIdentifiersAndReturnError_(&self, error: *mut NSError) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, supportedIdentifiersAndReturnError : error)
    }
    unsafe fn results(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, results)
    }
    unsafe fn knownAnimalIdentifiersForRevision_error_(
        requestRevision: NSUInteger,
        error: *mut NSError,
    ) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"VNRecognizeAnimalsRequest").unwrap(), knownAnimalIdentifiersForRevision : requestRevision, error : error)
    }
}
pub type VNGenerateOpticalFlowRequestComputationAccuracy = NSUInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct VNGenerateOpticalFlowRequest(pub id);
impl std::ops::Deref for VNGenerateOpticalFlowRequest {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for VNGenerateOpticalFlowRequest {}
impl VNGenerateOpticalFlowRequest {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"VNGenerateOpticalFlowRequest").unwrap(), alloc) })
    }
}
impl IVNTargetedImageRequest for VNGenerateOpticalFlowRequest {}
impl std::convert::TryFrom<VNTargetedImageRequest> for VNGenerateOpticalFlowRequest {
    type Error = &'static str;
    fn try_from(
        parent: VNTargetedImageRequest,
    ) -> Result<VNGenerateOpticalFlowRequest, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"VNGenerateOpticalFlowRequest").unwrap()) };
        if is_kind_of {
            Ok(VNGenerateOpticalFlowRequest(parent.0))
        } else {
            Err("This VNTargetedImageRequest cannot be downcasted to VNGenerateOpticalFlowRequest")
        }
    }
}
impl IVNImageBasedRequest for VNGenerateOpticalFlowRequest {}
impl IVNRequest for VNGenerateOpticalFlowRequest {}
impl PNSCopying for VNGenerateOpticalFlowRequest {}
impl INSObject for VNGenerateOpticalFlowRequest {}
impl PNSObject for VNGenerateOpticalFlowRequest {}
impl IVNGenerateOpticalFlowRequest for VNGenerateOpticalFlowRequest {}
pub trait IVNGenerateOpticalFlowRequest: Sized + std::ops::Deref {
    unsafe fn computationAccuracy(&self) -> VNGenerateOpticalFlowRequestComputationAccuracy
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, computationAccuracy)
    }
    unsafe fn setComputationAccuracy_(
        &self,
        computationAccuracy: VNGenerateOpticalFlowRequestComputationAccuracy,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setComputationAccuracy : computationAccuracy)
    }
    unsafe fn outputPixelFormat(&self) -> OSType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, outputPixelFormat)
    }
    unsafe fn setOutputPixelFormat_(&self, outputPixelFormat: OSType)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setOutputPixelFormat : outputPixelFormat)
    }
    unsafe fn keepNetworkOutput(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, keepNetworkOutput)
    }
    unsafe fn setKeepNetworkOutput_(&self, keepNetworkOutput: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setKeepNetworkOutput : keepNetworkOutput)
    }
    unsafe fn results(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, results)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct VNVideoProcessorCadence(pub id);
impl std::ops::Deref for VNVideoProcessorCadence {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for VNVideoProcessorCadence {}
impl VNVideoProcessorCadence {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"VNVideoProcessorCadence").unwrap(), alloc) })
    }
}
impl PNSCopying for VNVideoProcessorCadence {}
impl INSObject for VNVideoProcessorCadence {}
impl PNSObject for VNVideoProcessorCadence {}
impl std::convert::TryFrom<NSObject> for VNVideoProcessorCadence {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<VNVideoProcessorCadence, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"VNVideoProcessorCadence").unwrap()) };
        if is_kind_of {
            Ok(VNVideoProcessorCadence(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to VNVideoProcessorCadence")
        }
    }
}
impl IVNVideoProcessorCadence for VNVideoProcessorCadence {}
pub trait IVNVideoProcessorCadence: Sized + std::ops::Deref {}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct VNVideoProcessorFrameRateCadence(pub id);
impl std::ops::Deref for VNVideoProcessorFrameRateCadence {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for VNVideoProcessorFrameRateCadence {}
impl VNVideoProcessorFrameRateCadence {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"VNVideoProcessorFrameRateCadence").unwrap(), alloc) })
    }
}
impl IVNVideoProcessorCadence for VNVideoProcessorFrameRateCadence {}
impl PNSCopying for VNVideoProcessorFrameRateCadence {}
impl From<VNVideoProcessorFrameRateCadence> for VNVideoProcessorCadence {
    fn from(child: VNVideoProcessorFrameRateCadence) -> VNVideoProcessorCadence {
        VNVideoProcessorCadence(child.0)
    }
}
impl std::convert::TryFrom<VNVideoProcessorCadence> for VNVideoProcessorFrameRateCadence {
    type Error = &'static str;
    fn try_from(
        parent: VNVideoProcessorCadence,
    ) -> Result<VNVideoProcessorFrameRateCadence, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"VNVideoProcessorFrameRateCadence").unwrap())
        };
        if is_kind_of {
            Ok(VNVideoProcessorFrameRateCadence(parent.0))
        } else {
            Err ("This VNVideoProcessorCadence cannot be downcasted to VNVideoProcessorFrameRateCadence" ,)
        }
    }
}
impl INSObject for VNVideoProcessorFrameRateCadence {}
impl PNSObject for VNVideoProcessorFrameRateCadence {}
impl IVNVideoProcessorFrameRateCadence for VNVideoProcessorFrameRateCadence {}
pub trait IVNVideoProcessorFrameRateCadence: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn initWithFrameRate_(&self, frameRate: NSInteger) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithFrameRate : frameRate)
    }
    unsafe fn frameRate(&self) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, frameRate)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct VNVideoProcessorTimeIntervalCadence(pub id);
impl std::ops::Deref for VNVideoProcessorTimeIntervalCadence {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for VNVideoProcessorTimeIntervalCadence {}
impl VNVideoProcessorTimeIntervalCadence {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"VNVideoProcessorTimeIntervalCadence").unwrap(), alloc) })
    }
}
impl IVNVideoProcessorCadence for VNVideoProcessorTimeIntervalCadence {}
impl PNSCopying for VNVideoProcessorTimeIntervalCadence {}
impl std::convert::TryFrom<VNVideoProcessorCadence> for VNVideoProcessorTimeIntervalCadence {
    type Error = &'static str;
    fn try_from(
        parent: VNVideoProcessorCadence,
    ) -> Result<VNVideoProcessorTimeIntervalCadence, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"VNVideoProcessorTimeIntervalCadence").unwrap())
        };
        if is_kind_of {
            Ok(VNVideoProcessorTimeIntervalCadence(parent.0))
        } else {
            Err ("This VNVideoProcessorCadence cannot be downcasted to VNVideoProcessorTimeIntervalCadence" ,)
        }
    }
}
impl INSObject for VNVideoProcessorTimeIntervalCadence {}
impl PNSObject for VNVideoProcessorTimeIntervalCadence {}
impl IVNVideoProcessorTimeIntervalCadence for VNVideoProcessorTimeIntervalCadence {}
pub trait IVNVideoProcessorTimeIntervalCadence: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn initWithTimeInterval_(&self, timeInterval: CFTimeInterval) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithTimeInterval : timeInterval)
    }
    unsafe fn timeInterval(&self) -> CFTimeInterval
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, timeInterval)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct VNVideoProcessorRequestProcessingOptions(pub id);
impl std::ops::Deref for VNVideoProcessorRequestProcessingOptions {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for VNVideoProcessorRequestProcessingOptions {}
impl VNVideoProcessorRequestProcessingOptions {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"VNVideoProcessorRequestProcessingOptions").unwrap(), alloc) })
    }
}
impl PNSCopying for VNVideoProcessorRequestProcessingOptions {}
impl INSObject for VNVideoProcessorRequestProcessingOptions {}
impl PNSObject for VNVideoProcessorRequestProcessingOptions {}
impl std::convert::TryFrom<NSObject> for VNVideoProcessorRequestProcessingOptions {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<VNVideoProcessorRequestProcessingOptions, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"VNVideoProcessorRequestProcessingOptions").unwrap())
        };
        if is_kind_of {
            Ok(VNVideoProcessorRequestProcessingOptions(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to VNVideoProcessorRequestProcessingOptions")
        }
    }
}
impl IVNVideoProcessorRequestProcessingOptions for VNVideoProcessorRequestProcessingOptions {}
pub trait IVNVideoProcessorRequestProcessingOptions: Sized + std::ops::Deref {
    unsafe fn cadence(&self) -> VNVideoProcessorCadence
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, cadence)
    }
    unsafe fn setCadence_(&self, cadence: VNVideoProcessorCadence)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCadence : cadence)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct VNVideoProcessor(pub id);
impl std::ops::Deref for VNVideoProcessor {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for VNVideoProcessor {}
impl VNVideoProcessor {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"VNVideoProcessor").unwrap(), alloc) })
    }
}
impl INSObject for VNVideoProcessor {}
impl PNSObject for VNVideoProcessor {}
impl std::convert::TryFrom<NSObject> for VNVideoProcessor {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<VNVideoProcessor, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"VNVideoProcessor").unwrap()) };
        if is_kind_of {
            Ok(VNVideoProcessor(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to VNVideoProcessor")
        }
    }
}
impl IVNVideoProcessor for VNVideoProcessor {}
pub trait IVNVideoProcessor: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn initWithURL_(&self, videoURL: NSURL) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithURL : videoURL)
    }
    unsafe fn addRequest_processingOptions_error_(
        &self,
        request: VNRequest,
        processingOptions: VNVideoProcessorRequestProcessingOptions,
        error: *mut NSError,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, addRequest : request, processingOptions : processingOptions, error : error)
    }
    unsafe fn addRequest_withProcessingOptions_error_(
        &self,
        request: VNRequest,
        processingOptions: NSDictionary,
        error: *mut NSError,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, addRequest : request, withProcessingOptions : processingOptions, error : error)
    }
    unsafe fn removeRequest_error_(&self, request: VNRequest, error: *mut NSError) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, removeRequest : request, error : error)
    }
    unsafe fn analyzeTimeRange_error_(&self, timeRange: CMTimeRange, error: *mut NSError) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, analyzeTimeRange : timeRange, error : error)
    }
    unsafe fn analyzeWithTimeRange_error_(
        &self,
        timeRange: CMTimeRange,
        error: *mut NSError,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, analyzeWithTimeRange : timeRange, error : error)
    }
    unsafe fn cancel(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, cancel)
    }
}
pub type VNHumanHandPoseObservationJointName = VNRecognizedPointKey;
pub type VNHumanHandPoseObservationJointsGroupName = VNRecognizedPointGroupKey;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct VNHumanHandPoseObservation(pub id);
impl std::ops::Deref for VNHumanHandPoseObservation {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for VNHumanHandPoseObservation {}
impl VNHumanHandPoseObservation {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"VNHumanHandPoseObservation").unwrap(), alloc) })
    }
}
impl IVNRecognizedPointsObservation for VNHumanHandPoseObservation {}
impl std::convert::TryFrom<VNRecognizedPointsObservation> for VNHumanHandPoseObservation {
    type Error = &'static str;
    fn try_from(
        parent: VNRecognizedPointsObservation,
    ) -> Result<VNHumanHandPoseObservation, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"VNHumanHandPoseObservation").unwrap()) };
        if is_kind_of {
            Ok(VNHumanHandPoseObservation(parent.0))
        } else {
            Err ("This VNRecognizedPointsObservation cannot be downcasted to VNHumanHandPoseObservation" ,)
        }
    }
}
impl IVNObservation for VNHumanHandPoseObservation {}
impl PNSCopying for VNHumanHandPoseObservation {}
impl PNSSecureCoding for VNHumanHandPoseObservation {}
impl PVNRequestRevisionProviding for VNHumanHandPoseObservation {}
impl INSObject for VNHumanHandPoseObservation {}
impl PNSObject for VNHumanHandPoseObservation {}
impl IVNHumanHandPoseObservation for VNHumanHandPoseObservation {}
pub trait IVNHumanHandPoseObservation: Sized + std::ops::Deref {
    unsafe fn recognizedPointForJointName_error_(
        &self,
        jointName: NSString,
        error: *mut NSError,
    ) -> VNRecognizedPoint
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, recognizedPointForJointName : jointName, error : error)
    }
    unsafe fn recognizedPointsForJointsGroupName_error_(
        &self,
        jointsGroupName: NSString,
        error: *mut NSError,
    ) -> NSDictionary
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, recognizedPointsForJointsGroupName : jointsGroupName, error : error)
    }
    unsafe fn availableJointNames(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, availableJointNames)
    }
    unsafe fn availableJointsGroupNames(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, availableJointsGroupNames)
    }
    unsafe fn chirality(&self) -> VNChirality
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, chirality)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct VNDetectHumanHandPoseRequest(pub id);
impl std::ops::Deref for VNDetectHumanHandPoseRequest {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for VNDetectHumanHandPoseRequest {}
impl VNDetectHumanHandPoseRequest {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"VNDetectHumanHandPoseRequest").unwrap(), alloc) })
    }
}
impl IVNImageBasedRequest for VNDetectHumanHandPoseRequest {}
impl std::convert::TryFrom<VNImageBasedRequest> for VNDetectHumanHandPoseRequest {
    type Error = &'static str;
    fn try_from(parent: VNImageBasedRequest) -> Result<VNDetectHumanHandPoseRequest, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"VNDetectHumanHandPoseRequest").unwrap()) };
        if is_kind_of {
            Ok(VNDetectHumanHandPoseRequest(parent.0))
        } else {
            Err("This VNImageBasedRequest cannot be downcasted to VNDetectHumanHandPoseRequest")
        }
    }
}
impl IVNRequest for VNDetectHumanHandPoseRequest {}
impl PNSCopying for VNDetectHumanHandPoseRequest {}
impl INSObject for VNDetectHumanHandPoseRequest {}
impl PNSObject for VNDetectHumanHandPoseRequest {}
impl IVNDetectHumanHandPoseRequest for VNDetectHumanHandPoseRequest {}
pub trait IVNDetectHumanHandPoseRequest: Sized + std::ops::Deref {
    unsafe fn supportedJointNamesAndReturnError_(&self, error: *mut NSError) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, supportedJointNamesAndReturnError : error)
    }
    unsafe fn supportedJointsGroupNamesAndReturnError_(&self, error: *mut NSError) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, supportedJointsGroupNamesAndReturnError : error)
    }
    unsafe fn maximumHandCount(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, maximumHandCount)
    }
    unsafe fn setMaximumHandCount_(&self, maximumHandCount: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMaximumHandCount : maximumHandCount)
    }
    unsafe fn results(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, results)
    }
    unsafe fn supportedJointNamesForRevision_error_(
        revision: NSUInteger,
        error: *mut NSError,
    ) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"VNDetectHumanHandPoseRequest").unwrap(), supportedJointNamesForRevision : revision, error : error)
    }
    unsafe fn supportedJointsGroupNamesForRevision_error_(
        revision: NSUInteger,
        error: *mut NSError,
    ) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"VNDetectHumanHandPoseRequest").unwrap(), supportedJointsGroupNamesForRevision : revision, error : error)
    }
}
pub type VNHumanBodyPoseObservationJointName = VNRecognizedPointKey;
pub type VNHumanBodyPoseObservationJointsGroupName = VNRecognizedPointGroupKey;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct VNHumanBodyPoseObservation(pub id);
impl std::ops::Deref for VNHumanBodyPoseObservation {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for VNHumanBodyPoseObservation {}
impl VNHumanBodyPoseObservation {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"VNHumanBodyPoseObservation").unwrap(), alloc) })
    }
}
impl IVNRecognizedPointsObservation for VNHumanBodyPoseObservation {}
impl std::convert::TryFrom<VNRecognizedPointsObservation> for VNHumanBodyPoseObservation {
    type Error = &'static str;
    fn try_from(
        parent: VNRecognizedPointsObservation,
    ) -> Result<VNHumanBodyPoseObservation, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"VNHumanBodyPoseObservation").unwrap()) };
        if is_kind_of {
            Ok(VNHumanBodyPoseObservation(parent.0))
        } else {
            Err ("This VNRecognizedPointsObservation cannot be downcasted to VNHumanBodyPoseObservation" ,)
        }
    }
}
impl IVNObservation for VNHumanBodyPoseObservation {}
impl PNSCopying for VNHumanBodyPoseObservation {}
impl PNSSecureCoding for VNHumanBodyPoseObservation {}
impl PVNRequestRevisionProviding for VNHumanBodyPoseObservation {}
impl INSObject for VNHumanBodyPoseObservation {}
impl PNSObject for VNHumanBodyPoseObservation {}
impl IVNHumanBodyPoseObservation for VNHumanBodyPoseObservation {}
pub trait IVNHumanBodyPoseObservation: Sized + std::ops::Deref {
    unsafe fn recognizedPointForJointName_error_(
        &self,
        jointName: NSString,
        error: *mut NSError,
    ) -> VNRecognizedPoint
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, recognizedPointForJointName : jointName, error : error)
    }
    unsafe fn recognizedPointsForJointsGroupName_error_(
        &self,
        jointsGroupName: NSString,
        error: *mut NSError,
    ) -> NSDictionary
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, recognizedPointsForJointsGroupName : jointsGroupName, error : error)
    }
    unsafe fn availableJointNames(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, availableJointNames)
    }
    unsafe fn availableJointsGroupNames(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, availableJointsGroupNames)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct VNDetectHumanBodyPoseRequest(pub id);
impl std::ops::Deref for VNDetectHumanBodyPoseRequest {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for VNDetectHumanBodyPoseRequest {}
impl VNDetectHumanBodyPoseRequest {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"VNDetectHumanBodyPoseRequest").unwrap(), alloc) })
    }
}
impl IVNImageBasedRequest for VNDetectHumanBodyPoseRequest {}
impl std::convert::TryFrom<VNImageBasedRequest> for VNDetectHumanBodyPoseRequest {
    type Error = &'static str;
    fn try_from(parent: VNImageBasedRequest) -> Result<VNDetectHumanBodyPoseRequest, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"VNDetectHumanBodyPoseRequest").unwrap()) };
        if is_kind_of {
            Ok(VNDetectHumanBodyPoseRequest(parent.0))
        } else {
            Err("This VNImageBasedRequest cannot be downcasted to VNDetectHumanBodyPoseRequest")
        }
    }
}
impl IVNRequest for VNDetectHumanBodyPoseRequest {}
impl PNSCopying for VNDetectHumanBodyPoseRequest {}
impl INSObject for VNDetectHumanBodyPoseRequest {}
impl PNSObject for VNDetectHumanBodyPoseRequest {}
impl IVNDetectHumanBodyPoseRequest for VNDetectHumanBodyPoseRequest {}
pub trait IVNDetectHumanBodyPoseRequest: Sized + std::ops::Deref {
    unsafe fn supportedJointNamesAndReturnError_(&self, error: *mut NSError) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, supportedJointNamesAndReturnError : error)
    }
    unsafe fn supportedJointsGroupNamesAndReturnError_(&self, error: *mut NSError) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, supportedJointsGroupNamesAndReturnError : error)
    }
    unsafe fn results(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, results)
    }
    unsafe fn supportedJointNamesForRevision_error_(
        revision: NSUInteger,
        error: *mut NSError,
    ) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"VNDetectHumanBodyPoseRequest").unwrap(), supportedJointNamesForRevision : revision, error : error)
    }
    unsafe fn supportedJointsGroupNamesForRevision_error_(
        revision: NSUInteger,
        error: *mut NSError,
    ) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"VNDetectHumanBodyPoseRequest").unwrap(), supportedJointsGroupNamesForRevision : revision, error : error)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct VNDetectContoursRequest(pub id);
impl std::ops::Deref for VNDetectContoursRequest {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for VNDetectContoursRequest {}
impl VNDetectContoursRequest {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"VNDetectContoursRequest").unwrap(), alloc) })
    }
}
impl IVNImageBasedRequest for VNDetectContoursRequest {}
impl std::convert::TryFrom<VNImageBasedRequest> for VNDetectContoursRequest {
    type Error = &'static str;
    fn try_from(parent: VNImageBasedRequest) -> Result<VNDetectContoursRequest, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"VNDetectContoursRequest").unwrap()) };
        if is_kind_of {
            Ok(VNDetectContoursRequest(parent.0))
        } else {
            Err("This VNImageBasedRequest cannot be downcasted to VNDetectContoursRequest")
        }
    }
}
impl IVNRequest for VNDetectContoursRequest {}
impl PNSCopying for VNDetectContoursRequest {}
impl INSObject for VNDetectContoursRequest {}
impl PNSObject for VNDetectContoursRequest {}
impl IVNDetectContoursRequest for VNDetectContoursRequest {}
pub trait IVNDetectContoursRequest: Sized + std::ops::Deref {
    unsafe fn contrastAdjustment(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, contrastAdjustment)
    }
    unsafe fn setContrastAdjustment_(&self, contrastAdjustment: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setContrastAdjustment : contrastAdjustment)
    }
    unsafe fn contrastPivot(&self) -> NSNumber
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, contrastPivot)
    }
    unsafe fn setContrastPivot_(&self, contrastPivot: NSNumber)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setContrastPivot : contrastPivot)
    }
    unsafe fn detectsDarkOnLight(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, detectsDarkOnLight)
    }
    unsafe fn setDetectsDarkOnLight_(&self, detectsDarkOnLight: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDetectsDarkOnLight : detectsDarkOnLight)
    }
    unsafe fn detectDarkOnLight(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, detectDarkOnLight)
    }
    unsafe fn setDetectDarkOnLight_(&self, detectDarkOnLight: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDetectDarkOnLight : detectDarkOnLight)
    }
    unsafe fn maximumImageDimension(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, maximumImageDimension)
    }
    unsafe fn setMaximumImageDimension_(&self, maximumImageDimension: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMaximumImageDimension : maximumImageDimension)
    }
    unsafe fn results(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, results)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct VNGeometryUtils(pub id);
impl std::ops::Deref for VNGeometryUtils {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for VNGeometryUtils {}
impl VNGeometryUtils {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"VNGeometryUtils").unwrap(), alloc) })
    }
}
impl INSObject for VNGeometryUtils {}
impl PNSObject for VNGeometryUtils {}
impl std::convert::TryFrom<NSObject> for VNGeometryUtils {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<VNGeometryUtils, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"VNGeometryUtils").unwrap()) };
        if is_kind_of {
            Ok(VNGeometryUtils(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to VNGeometryUtils")
        }
    }
}
impl IVNGeometryUtils for VNGeometryUtils {}
pub trait IVNGeometryUtils: Sized + std::ops::Deref {
    unsafe fn boundingCircleForContour_error_(contour: VNContour, error: *mut NSError) -> VNCircle
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"VNGeometryUtils").unwrap(), boundingCircleForContour : contour, error : error)
    }
    unsafe fn boundingCircleForPoints_error_(points: NSArray, error: *mut NSError) -> VNCircle
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"VNGeometryUtils").unwrap(), boundingCircleForPoints : points, error : error)
    }
    unsafe fn boundingCircleForSIMDPoints_pointCount_error_(
        points: *const simd_float2,
        pointCount: NSInteger,
        error: *mut NSError,
    ) -> VNCircle
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"VNGeometryUtils").unwrap(), boundingCircleForSIMDPoints : points, pointCount : pointCount, error : error)
    }
    unsafe fn calculateArea_forContour_orientedArea_error_(
        area: *mut f64,
        contour: VNContour,
        orientedArea: BOOL,
        error: *mut NSError,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"VNGeometryUtils").unwrap(), calculateArea : area, forContour : contour, orientedArea : orientedArea, error : error)
    }
    unsafe fn calculatePerimeter_forContour_error_(
        perimeter: *mut f64,
        contour: VNContour,
        error: *mut NSError,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"VNGeometryUtils").unwrap(), calculatePerimeter : perimeter, forContour : contour, error : error)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct VNStatefulRequest(pub id);
impl std::ops::Deref for VNStatefulRequest {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for VNStatefulRequest {}
impl VNStatefulRequest {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"VNStatefulRequest").unwrap(), alloc) })
    }
}
impl IVNImageBasedRequest for VNStatefulRequest {}
impl std::convert::TryFrom<VNImageBasedRequest> for VNStatefulRequest {
    type Error = &'static str;
    fn try_from(parent: VNImageBasedRequest) -> Result<VNStatefulRequest, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"VNStatefulRequest").unwrap()) };
        if is_kind_of {
            Ok(VNStatefulRequest(parent.0))
        } else {
            Err("This VNImageBasedRequest cannot be downcasted to VNStatefulRequest")
        }
    }
}
impl IVNRequest for VNStatefulRequest {}
impl PNSCopying for VNStatefulRequest {}
impl INSObject for VNStatefulRequest {}
impl PNSObject for VNStatefulRequest {}
impl IVNStatefulRequest for VNStatefulRequest {}
pub trait IVNStatefulRequest: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn initWithCompletionHandler_(
        &self,
        completionHandler: VNRequestCompletionHandler,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithCompletionHandler : completionHandler)
    }
    unsafe fn initWithFrameAnalysisSpacing_completionHandler_(
        &self,
        frameAnalysisSpacing: CMTime,
        completionHandler: VNRequestCompletionHandler,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithFrameAnalysisSpacing : frameAnalysisSpacing, completionHandler : completionHandler)
    }
    unsafe fn minimumLatencyFrameCount(&self) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, minimumLatencyFrameCount)
    }
    unsafe fn frameAnalysisSpacing(&self) -> CMTime
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, frameAnalysisSpacing)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"VNStatefulRequest").unwrap(), new)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct VNDetectTrajectoriesRequest(pub id);
impl std::ops::Deref for VNDetectTrajectoriesRequest {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for VNDetectTrajectoriesRequest {}
impl VNDetectTrajectoriesRequest {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"VNDetectTrajectoriesRequest").unwrap(), alloc) })
    }
}
impl IVNStatefulRequest for VNDetectTrajectoriesRequest {}
impl From<VNDetectTrajectoriesRequest> for VNStatefulRequest {
    fn from(child: VNDetectTrajectoriesRequest) -> VNStatefulRequest {
        VNStatefulRequest(child.0)
    }
}
impl std::convert::TryFrom<VNStatefulRequest> for VNDetectTrajectoriesRequest {
    type Error = &'static str;
    fn try_from(parent: VNStatefulRequest) -> Result<VNDetectTrajectoriesRequest, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"VNDetectTrajectoriesRequest").unwrap()) };
        if is_kind_of {
            Ok(VNDetectTrajectoriesRequest(parent.0))
        } else {
            Err("This VNStatefulRequest cannot be downcasted to VNDetectTrajectoriesRequest")
        }
    }
}
impl IVNImageBasedRequest for VNDetectTrajectoriesRequest {}
impl IVNRequest for VNDetectTrajectoriesRequest {}
impl PNSCopying for VNDetectTrajectoriesRequest {}
impl INSObject for VNDetectTrajectoriesRequest {}
impl PNSObject for VNDetectTrajectoriesRequest {}
impl IVNDetectTrajectoriesRequest for VNDetectTrajectoriesRequest {}
pub trait IVNDetectTrajectoriesRequest: Sized + std::ops::Deref {
    unsafe fn initWithFrameAnalysisSpacing_completionHandler_(
        &self,
        frameAnalysisSpacing: CMTime,
        completionHandler: VNRequestCompletionHandler,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithFrameAnalysisSpacing : frameAnalysisSpacing, completionHandler : completionHandler)
    }
    unsafe fn initWithFrameAnalysisSpacing_trajectoryLength_completionHandler_(
        &self,
        frameAnalysisSpacing: CMTime,
        trajectoryLength: NSInteger,
        completionHandler: VNRequestCompletionHandler,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithFrameAnalysisSpacing : frameAnalysisSpacing, trajectoryLength : trajectoryLength, completionHandler : completionHandler)
    }
    unsafe fn trajectoryLength(&self) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, trajectoryLength)
    }
    unsafe fn objectMinimumNormalizedRadius(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, objectMinimumNormalizedRadius)
    }
    unsafe fn setObjectMinimumNormalizedRadius_(&self, objectMinimumNormalizedRadius: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setObjectMinimumNormalizedRadius : objectMinimumNormalizedRadius)
    }
    unsafe fn minimumObjectSize(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, minimumObjectSize)
    }
    unsafe fn setMinimumObjectSize_(&self, minimumObjectSize: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMinimumObjectSize : minimumObjectSize)
    }
    unsafe fn objectMaximumNormalizedRadius(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, objectMaximumNormalizedRadius)
    }
    unsafe fn setObjectMaximumNormalizedRadius_(&self, objectMaximumNormalizedRadius: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setObjectMaximumNormalizedRadius : objectMaximumNormalizedRadius)
    }
    unsafe fn maximumObjectSize(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, maximumObjectSize)
    }
    unsafe fn setMaximumObjectSize_(&self, maximumObjectSize: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMaximumObjectSize : maximumObjectSize)
    }
    unsafe fn targetFrameTime(&self) -> CMTime
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, targetFrameTime)
    }
    unsafe fn setTargetFrameTime_(&self, targetFrameTime: CMTime)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setTargetFrameTime : targetFrameTime)
    }
    unsafe fn results(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, results)
    }
}
pub type VNGeneratePersonSegmentationRequestQualityLevel = NSUInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct VNGeneratePersonSegmentationRequest(pub id);
impl std::ops::Deref for VNGeneratePersonSegmentationRequest {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for VNGeneratePersonSegmentationRequest {}
impl VNGeneratePersonSegmentationRequest {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"VNGeneratePersonSegmentationRequest").unwrap(), alloc) })
    }
}
impl IVNStatefulRequest for VNGeneratePersonSegmentationRequest {}
impl std::convert::TryFrom<VNStatefulRequest> for VNGeneratePersonSegmentationRequest {
    type Error = &'static str;
    fn try_from(
        parent: VNStatefulRequest,
    ) -> Result<VNGeneratePersonSegmentationRequest, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"VNGeneratePersonSegmentationRequest").unwrap())
        };
        if is_kind_of {
            Ok(VNGeneratePersonSegmentationRequest(parent.0))
        } else {
            Err ("This VNStatefulRequest cannot be downcasted to VNGeneratePersonSegmentationRequest" ,)
        }
    }
}
impl IVNImageBasedRequest for VNGeneratePersonSegmentationRequest {}
impl IVNRequest for VNGeneratePersonSegmentationRequest {}
impl PNSCopying for VNGeneratePersonSegmentationRequest {}
impl INSObject for VNGeneratePersonSegmentationRequest {}
impl PNSObject for VNGeneratePersonSegmentationRequest {}
impl IVNGeneratePersonSegmentationRequest for VNGeneratePersonSegmentationRequest {}
pub trait IVNGeneratePersonSegmentationRequest: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn initWithCompletionHandler_(
        &self,
        completionHandler: VNRequestCompletionHandler,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithCompletionHandler : completionHandler)
    }
    unsafe fn initWithFrameAnalysisSpacing_completionHandler_(
        &self,
        frameAnalysisSpacing: CMTime,
        completionHandler: VNRequestCompletionHandler,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithFrameAnalysisSpacing : frameAnalysisSpacing, completionHandler : completionHandler)
    }
    unsafe fn supportedOutputPixelFormatsAndReturnError_(&self, error: *mut NSError) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, supportedOutputPixelFormatsAndReturnError : error)
    }
    unsafe fn qualityLevel(&self) -> VNGeneratePersonSegmentationRequestQualityLevel
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, qualityLevel)
    }
    unsafe fn setQualityLevel_(&self, qualityLevel: VNGeneratePersonSegmentationRequestQualityLevel)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setQualityLevel : qualityLevel)
    }
    unsafe fn outputPixelFormat(&self) -> OSType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, outputPixelFormat)
    }
    unsafe fn setOutputPixelFormat_(&self, outputPixelFormat: OSType)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setOutputPixelFormat : outputPixelFormat)
    }
    unsafe fn results(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, results)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"VNGeneratePersonSegmentationRequest").unwrap(), new)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct VNDetectDocumentSegmentationRequest(pub id);
impl std::ops::Deref for VNDetectDocumentSegmentationRequest {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for VNDetectDocumentSegmentationRequest {}
impl VNDetectDocumentSegmentationRequest {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"VNDetectDocumentSegmentationRequest").unwrap(), alloc) })
    }
}
impl IVNImageBasedRequest for VNDetectDocumentSegmentationRequest {}
impl std::convert::TryFrom<VNImageBasedRequest> for VNDetectDocumentSegmentationRequest {
    type Error = &'static str;
    fn try_from(
        parent: VNImageBasedRequest,
    ) -> Result<VNDetectDocumentSegmentationRequest, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"VNDetectDocumentSegmentationRequest").unwrap())
        };
        if is_kind_of {
            Ok(VNDetectDocumentSegmentationRequest(parent.0))
        } else {
            Err ("This VNImageBasedRequest cannot be downcasted to VNDetectDocumentSegmentationRequest" ,)
        }
    }
}
impl IVNRequest for VNDetectDocumentSegmentationRequest {}
impl PNSCopying for VNDetectDocumentSegmentationRequest {}
impl INSObject for VNDetectDocumentSegmentationRequest {}
impl PNSObject for VNDetectDocumentSegmentationRequest {}
impl IVNDetectDocumentSegmentationRequest for VNDetectDocumentSegmentationRequest {}
pub trait IVNDetectDocumentSegmentationRequest: Sized + std::ops::Deref {
    unsafe fn results(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, results)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct VNGenerateForegroundInstanceMaskRequest(pub id);
impl std::ops::Deref for VNGenerateForegroundInstanceMaskRequest {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for VNGenerateForegroundInstanceMaskRequest {}
impl VNGenerateForegroundInstanceMaskRequest {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"VNGenerateForegroundInstanceMaskRequest").unwrap(), alloc) })
    }
}
impl IVNImageBasedRequest for VNGenerateForegroundInstanceMaskRequest {}
impl std::convert::TryFrom<VNImageBasedRequest> for VNGenerateForegroundInstanceMaskRequest {
    type Error = &'static str;
    fn try_from(
        parent: VNImageBasedRequest,
    ) -> Result<VNGenerateForegroundInstanceMaskRequest, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"VNGenerateForegroundInstanceMaskRequest").unwrap())
        };
        if is_kind_of {
            Ok(VNGenerateForegroundInstanceMaskRequest(parent.0))
        } else {
            Err ("This VNImageBasedRequest cannot be downcasted to VNGenerateForegroundInstanceMaskRequest" ,)
        }
    }
}
impl IVNRequest for VNGenerateForegroundInstanceMaskRequest {}
impl PNSCopying for VNGenerateForegroundInstanceMaskRequest {}
impl INSObject for VNGenerateForegroundInstanceMaskRequest {}
impl PNSObject for VNGenerateForegroundInstanceMaskRequest {}
impl IVNGenerateForegroundInstanceMaskRequest for VNGenerateForegroundInstanceMaskRequest {}
pub trait IVNGenerateForegroundInstanceMaskRequest: Sized + std::ops::Deref {
    unsafe fn results(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, results)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct VNGeneratePersonInstanceMaskRequest(pub id);
impl std::ops::Deref for VNGeneratePersonInstanceMaskRequest {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for VNGeneratePersonInstanceMaskRequest {}
impl VNGeneratePersonInstanceMaskRequest {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"VNGeneratePersonInstanceMaskRequest").unwrap(), alloc) })
    }
}
impl IVNImageBasedRequest for VNGeneratePersonInstanceMaskRequest {}
impl std::convert::TryFrom<VNImageBasedRequest> for VNGeneratePersonInstanceMaskRequest {
    type Error = &'static str;
    fn try_from(
        parent: VNImageBasedRequest,
    ) -> Result<VNGeneratePersonInstanceMaskRequest, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"VNGeneratePersonInstanceMaskRequest").unwrap())
        };
        if is_kind_of {
            Ok(VNGeneratePersonInstanceMaskRequest(parent.0))
        } else {
            Err ("This VNImageBasedRequest cannot be downcasted to VNGeneratePersonInstanceMaskRequest" ,)
        }
    }
}
impl IVNRequest for VNGeneratePersonInstanceMaskRequest {}
impl PNSCopying for VNGeneratePersonInstanceMaskRequest {}
impl INSObject for VNGeneratePersonInstanceMaskRequest {}
impl PNSObject for VNGeneratePersonInstanceMaskRequest {}
impl IVNGeneratePersonInstanceMaskRequest for VNGeneratePersonInstanceMaskRequest {}
pub trait IVNGeneratePersonInstanceMaskRequest: Sized + std::ops::Deref {
    unsafe fn results(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, results)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct VNTrackTranslationalImageRegistrationRequest(pub id);
impl std::ops::Deref for VNTrackTranslationalImageRegistrationRequest {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for VNTrackTranslationalImageRegistrationRequest {}
impl VNTrackTranslationalImageRegistrationRequest {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"VNTrackTranslationalImageRegistrationRequest").unwrap(), alloc) })
    }
}
impl IVNStatefulRequest for VNTrackTranslationalImageRegistrationRequest {}
impl std::convert::TryFrom<VNStatefulRequest> for VNTrackTranslationalImageRegistrationRequest {
    type Error = &'static str;
    fn try_from(
        parent: VNStatefulRequest,
    ) -> Result<VNTrackTranslationalImageRegistrationRequest, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"VNTrackTranslationalImageRegistrationRequest").unwrap())
        };
        if is_kind_of {
            Ok(VNTrackTranslationalImageRegistrationRequest(parent.0))
        } else {
            Err ("This VNStatefulRequest cannot be downcasted to VNTrackTranslationalImageRegistrationRequest" ,)
        }
    }
}
impl IVNImageBasedRequest for VNTrackTranslationalImageRegistrationRequest {}
impl IVNRequest for VNTrackTranslationalImageRegistrationRequest {}
impl PNSCopying for VNTrackTranslationalImageRegistrationRequest {}
impl INSObject for VNTrackTranslationalImageRegistrationRequest {}
impl PNSObject for VNTrackTranslationalImageRegistrationRequest {}
impl IVNTrackTranslationalImageRegistrationRequest
    for VNTrackTranslationalImageRegistrationRequest
{
}
pub trait IVNTrackTranslationalImageRegistrationRequest: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn initWithCompletionHandler_(
        &self,
        completionHandler: VNRequestCompletionHandler,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithCompletionHandler : completionHandler)
    }
    unsafe fn results(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, results)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct VNTrackHomographicImageRegistrationRequest(pub id);
impl std::ops::Deref for VNTrackHomographicImageRegistrationRequest {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for VNTrackHomographicImageRegistrationRequest {}
impl VNTrackHomographicImageRegistrationRequest {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"VNTrackHomographicImageRegistrationRequest").unwrap(), alloc) })
    }
}
impl IVNStatefulRequest for VNTrackHomographicImageRegistrationRequest {}
impl std::convert::TryFrom<VNStatefulRequest> for VNTrackHomographicImageRegistrationRequest {
    type Error = &'static str;
    fn try_from(
        parent: VNStatefulRequest,
    ) -> Result<VNTrackHomographicImageRegistrationRequest, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"VNTrackHomographicImageRegistrationRequest").unwrap())
        };
        if is_kind_of {
            Ok(VNTrackHomographicImageRegistrationRequest(parent.0))
        } else {
            Err ("This VNStatefulRequest cannot be downcasted to VNTrackHomographicImageRegistrationRequest" ,)
        }
    }
}
impl IVNImageBasedRequest for VNTrackHomographicImageRegistrationRequest {}
impl IVNRequest for VNTrackHomographicImageRegistrationRequest {}
impl PNSCopying for VNTrackHomographicImageRegistrationRequest {}
impl INSObject for VNTrackHomographicImageRegistrationRequest {}
impl PNSObject for VNTrackHomographicImageRegistrationRequest {}
impl IVNTrackHomographicImageRegistrationRequest for VNTrackHomographicImageRegistrationRequest {}
pub trait IVNTrackHomographicImageRegistrationRequest: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn initWithCompletionHandler_(
        &self,
        completionHandler: VNRequestCompletionHandler,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithCompletionHandler : completionHandler)
    }
    unsafe fn results(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, results)
    }
}
pub type VNTrackOpticalFlowRequestComputationAccuracy = NSUInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct VNTrackOpticalFlowRequest(pub id);
impl std::ops::Deref for VNTrackOpticalFlowRequest {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for VNTrackOpticalFlowRequest {}
impl VNTrackOpticalFlowRequest {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"VNTrackOpticalFlowRequest").unwrap(), alloc) })
    }
}
impl IVNStatefulRequest for VNTrackOpticalFlowRequest {}
impl std::convert::TryFrom<VNStatefulRequest> for VNTrackOpticalFlowRequest {
    type Error = &'static str;
    fn try_from(parent: VNStatefulRequest) -> Result<VNTrackOpticalFlowRequest, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"VNTrackOpticalFlowRequest").unwrap()) };
        if is_kind_of {
            Ok(VNTrackOpticalFlowRequest(parent.0))
        } else {
            Err("This VNStatefulRequest cannot be downcasted to VNTrackOpticalFlowRequest")
        }
    }
}
impl IVNImageBasedRequest for VNTrackOpticalFlowRequest {}
impl IVNRequest for VNTrackOpticalFlowRequest {}
impl PNSCopying for VNTrackOpticalFlowRequest {}
impl INSObject for VNTrackOpticalFlowRequest {}
impl PNSObject for VNTrackOpticalFlowRequest {}
impl IVNTrackOpticalFlowRequest for VNTrackOpticalFlowRequest {}
pub trait IVNTrackOpticalFlowRequest: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn initWithCompletionHandler_(
        &self,
        completionHandler: VNRequestCompletionHandler,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithCompletionHandler : completionHandler)
    }
    unsafe fn computationAccuracy(&self) -> VNTrackOpticalFlowRequestComputationAccuracy
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, computationAccuracy)
    }
    unsafe fn setComputationAccuracy_(
        &self,
        computationAccuracy: VNTrackOpticalFlowRequestComputationAccuracy,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setComputationAccuracy : computationAccuracy)
    }
    unsafe fn outputPixelFormat(&self) -> OSType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, outputPixelFormat)
    }
    unsafe fn setOutputPixelFormat_(&self, outputPixelFormat: OSType)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setOutputPixelFormat : outputPixelFormat)
    }
    unsafe fn keepNetworkOutput(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, keepNetworkOutput)
    }
    unsafe fn setKeepNetworkOutput_(&self, keepNetworkOutput: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setKeepNetworkOutput : keepNetworkOutput)
    }
    unsafe fn results(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, results)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct VNDetectAnimalBodyPoseRequest(pub id);
impl std::ops::Deref for VNDetectAnimalBodyPoseRequest {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for VNDetectAnimalBodyPoseRequest {}
impl VNDetectAnimalBodyPoseRequest {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"VNDetectAnimalBodyPoseRequest").unwrap(), alloc) })
    }
}
impl IVNImageBasedRequest for VNDetectAnimalBodyPoseRequest {}
impl std::convert::TryFrom<VNImageBasedRequest> for VNDetectAnimalBodyPoseRequest {
    type Error = &'static str;
    fn try_from(parent: VNImageBasedRequest) -> Result<VNDetectAnimalBodyPoseRequest, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"VNDetectAnimalBodyPoseRequest").unwrap())
        };
        if is_kind_of {
            Ok(VNDetectAnimalBodyPoseRequest(parent.0))
        } else {
            Err("This VNImageBasedRequest cannot be downcasted to VNDetectAnimalBodyPoseRequest")
        }
    }
}
impl IVNRequest for VNDetectAnimalBodyPoseRequest {}
impl PNSCopying for VNDetectAnimalBodyPoseRequest {}
impl INSObject for VNDetectAnimalBodyPoseRequest {}
impl PNSObject for VNDetectAnimalBodyPoseRequest {}
impl IVNDetectAnimalBodyPoseRequest for VNDetectAnimalBodyPoseRequest {}
pub trait IVNDetectAnimalBodyPoseRequest: Sized + std::ops::Deref {
    unsafe fn supportedJointNamesAndReturnError_(&self, error: *mut NSError) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, supportedJointNamesAndReturnError : error)
    }
    unsafe fn supportedJointsGroupNamesAndReturnError_(&self, error: *mut NSError) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, supportedJointsGroupNamesAndReturnError : error)
    }
    unsafe fn results(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, results)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct VNDetectHumanBodyPose3DRequest(pub id);
impl std::ops::Deref for VNDetectHumanBodyPose3DRequest {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for VNDetectHumanBodyPose3DRequest {}
impl VNDetectHumanBodyPose3DRequest {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"VNDetectHumanBodyPose3DRequest").unwrap(), alloc) })
    }
}
impl IVNStatefulRequest for VNDetectHumanBodyPose3DRequest {}
impl std::convert::TryFrom<VNStatefulRequest> for VNDetectHumanBodyPose3DRequest {
    type Error = &'static str;
    fn try_from(parent: VNStatefulRequest) -> Result<VNDetectHumanBodyPose3DRequest, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"VNDetectHumanBodyPose3DRequest").unwrap())
        };
        if is_kind_of {
            Ok(VNDetectHumanBodyPose3DRequest(parent.0))
        } else {
            Err("This VNStatefulRequest cannot be downcasted to VNDetectHumanBodyPose3DRequest")
        }
    }
}
impl IVNImageBasedRequest for VNDetectHumanBodyPose3DRequest {}
impl IVNRequest for VNDetectHumanBodyPose3DRequest {}
impl PNSCopying for VNDetectHumanBodyPose3DRequest {}
impl INSObject for VNDetectHumanBodyPose3DRequest {}
impl PNSObject for VNDetectHumanBodyPose3DRequest {}
impl IVNDetectHumanBodyPose3DRequest for VNDetectHumanBodyPose3DRequest {}
pub trait IVNDetectHumanBodyPose3DRequest: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn initWithCompletionHandler_(
        &self,
        completionHandler: VNRequestCompletionHandler,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithCompletionHandler : completionHandler)
    }
    unsafe fn supportedJointNamesAndReturnError_(&self, error: *mut NSError) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, supportedJointNamesAndReturnError : error)
    }
    unsafe fn supportedJointsGroupNamesAndReturnError_(&self, error: *mut NSError) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, supportedJointsGroupNamesAndReturnError : error)
    }
    unsafe fn results(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, results)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct VNRecognizedPoint3D(pub id);
impl std::ops::Deref for VNRecognizedPoint3D {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for VNRecognizedPoint3D {}
impl VNRecognizedPoint3D {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"VNRecognizedPoint3D").unwrap(), alloc) })
    }
}
impl IVNPoint3D for VNRecognizedPoint3D {}
impl PNSCopying for VNRecognizedPoint3D {}
impl PNSSecureCoding for VNRecognizedPoint3D {}
impl From<VNRecognizedPoint3D> for VNPoint3D {
    fn from(child: VNRecognizedPoint3D) -> VNPoint3D {
        VNPoint3D(child.0)
    }
}
impl std::convert::TryFrom<VNPoint3D> for VNRecognizedPoint3D {
    type Error = &'static str;
    fn try_from(parent: VNPoint3D) -> Result<VNRecognizedPoint3D, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"VNRecognizedPoint3D").unwrap()) };
        if is_kind_of {
            Ok(VNRecognizedPoint3D(parent.0))
        } else {
            Err("This VNPoint3D cannot be downcasted to VNRecognizedPoint3D")
        }
    }
}
impl INSObject for VNRecognizedPoint3D {}
impl PNSObject for VNRecognizedPoint3D {}
impl IVNRecognizedPoint3D for VNRecognizedPoint3D {}
pub trait IVNRecognizedPoint3D: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn identifier(&self) -> VNRecognizedPointKey
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, identifier)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"VNRecognizedPoint3D").unwrap(), new)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct VNHumanBodyRecognizedPoint3D(pub id);
impl std::ops::Deref for VNHumanBodyRecognizedPoint3D {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for VNHumanBodyRecognizedPoint3D {}
impl VNHumanBodyRecognizedPoint3D {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"VNHumanBodyRecognizedPoint3D").unwrap(), alloc) })
    }
}
impl IVNRecognizedPoint3D for VNHumanBodyRecognizedPoint3D {}
impl From<VNHumanBodyRecognizedPoint3D> for VNRecognizedPoint3D {
    fn from(child: VNHumanBodyRecognizedPoint3D) -> VNRecognizedPoint3D {
        VNRecognizedPoint3D(child.0)
    }
}
impl std::convert::TryFrom<VNRecognizedPoint3D> for VNHumanBodyRecognizedPoint3D {
    type Error = &'static str;
    fn try_from(parent: VNRecognizedPoint3D) -> Result<VNHumanBodyRecognizedPoint3D, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"VNHumanBodyRecognizedPoint3D").unwrap()) };
        if is_kind_of {
            Ok(VNHumanBodyRecognizedPoint3D(parent.0))
        } else {
            Err("This VNRecognizedPoint3D cannot be downcasted to VNHumanBodyRecognizedPoint3D")
        }
    }
}
impl IVNPoint3D for VNHumanBodyRecognizedPoint3D {}
impl PNSCopying for VNHumanBodyRecognizedPoint3D {}
impl PNSSecureCoding for VNHumanBodyRecognizedPoint3D {}
impl INSObject for VNHumanBodyRecognizedPoint3D {}
impl PNSObject for VNHumanBodyRecognizedPoint3D {}
impl IVNHumanBodyRecognizedPoint3D for VNHumanBodyRecognizedPoint3D {}
pub trait IVNHumanBodyRecognizedPoint3D: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn parentJoint(&self) -> VNHumanBodyPose3DObservationJointName
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, parentJoint)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"VNHumanBodyRecognizedPoint3D").unwrap(), new)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct VNCalculateImageAestheticsScoresRequest(pub id);
impl std::ops::Deref for VNCalculateImageAestheticsScoresRequest {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for VNCalculateImageAestheticsScoresRequest {}
impl VNCalculateImageAestheticsScoresRequest {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"VNCalculateImageAestheticsScoresRequest").unwrap(), alloc) })
    }
}
impl IVNImageBasedRequest for VNCalculateImageAestheticsScoresRequest {}
impl std::convert::TryFrom<VNImageBasedRequest> for VNCalculateImageAestheticsScoresRequest {
    type Error = &'static str;
    fn try_from(
        parent: VNImageBasedRequest,
    ) -> Result<VNCalculateImageAestheticsScoresRequest, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"VNCalculateImageAestheticsScoresRequest").unwrap())
        };
        if is_kind_of {
            Ok(VNCalculateImageAestheticsScoresRequest(parent.0))
        } else {
            Err ("This VNImageBasedRequest cannot be downcasted to VNCalculateImageAestheticsScoresRequest" ,)
        }
    }
}
impl IVNRequest for VNCalculateImageAestheticsScoresRequest {}
impl PNSCopying for VNCalculateImageAestheticsScoresRequest {}
impl INSObject for VNCalculateImageAestheticsScoresRequest {}
impl PNSObject for VNCalculateImageAestheticsScoresRequest {}
impl IVNCalculateImageAestheticsScoresRequest for VNCalculateImageAestheticsScoresRequest {}
pub trait IVNCalculateImageAestheticsScoresRequest: Sized + std::ops::Deref {
    unsafe fn results(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, results)
    }
}
unsafe extern "C" {
    pub static VNComputeStageMain: VNComputeStage;
}
unsafe extern "C" {
    pub static VNComputeStagePostProcessing: VNComputeStage;
}
unsafe extern "C" {
    pub static VNBarcodeSymbologyAztec: VNBarcodeSymbology;
}
unsafe extern "C" {
    pub static VNBarcodeSymbologyCode39: VNBarcodeSymbology;
}
unsafe extern "C" {
    pub static VNBarcodeSymbologyCode39Checksum: VNBarcodeSymbology;
}
unsafe extern "C" {
    pub static VNBarcodeSymbologyCode39FullASCII: VNBarcodeSymbology;
}
unsafe extern "C" {
    pub static VNBarcodeSymbologyCode39FullASCIIChecksum: VNBarcodeSymbology;
}
unsafe extern "C" {
    pub static VNBarcodeSymbologyCode93: VNBarcodeSymbology;
}
unsafe extern "C" {
    pub static VNBarcodeSymbologyCode93i: VNBarcodeSymbology;
}
unsafe extern "C" {
    pub static VNBarcodeSymbologyCode128: VNBarcodeSymbology;
}
unsafe extern "C" {
    pub static VNBarcodeSymbologyDataMatrix: VNBarcodeSymbology;
}
unsafe extern "C" {
    pub static VNBarcodeSymbologyEAN8: VNBarcodeSymbology;
}
unsafe extern "C" {
    pub static VNBarcodeSymbologyEAN13: VNBarcodeSymbology;
}
unsafe extern "C" {
    pub static VNBarcodeSymbologyI2of5: VNBarcodeSymbology;
}
unsafe extern "C" {
    pub static VNBarcodeSymbologyI2of5Checksum: VNBarcodeSymbology;
}
unsafe extern "C" {
    pub static VNBarcodeSymbologyITF14: VNBarcodeSymbology;
}
unsafe extern "C" {
    pub static VNBarcodeSymbologyPDF417: VNBarcodeSymbology;
}
unsafe extern "C" {
    pub static VNBarcodeSymbologyQR: VNBarcodeSymbology;
}
unsafe extern "C" {
    pub static VNBarcodeSymbologyUPCE: VNBarcodeSymbology;
}
unsafe extern "C" {
    pub static VNBarcodeSymbologyCodabar: VNBarcodeSymbology;
}
unsafe extern "C" {
    pub static VNBarcodeSymbologyGS1DataBar: VNBarcodeSymbology;
}
unsafe extern "C" {
    pub static VNBarcodeSymbologyGS1DataBarExpanded: VNBarcodeSymbology;
}
unsafe extern "C" {
    pub static VNBarcodeSymbologyGS1DataBarLimited: VNBarcodeSymbology;
}
unsafe extern "C" {
    pub static VNBarcodeSymbologyMicroPDF417: VNBarcodeSymbology;
}
unsafe extern "C" {
    pub static VNBarcodeSymbologyMicroQR: VNBarcodeSymbology;
}
unsafe extern "C" {
    pub static VNBarcodeSymbologyMSIPlessey: VNBarcodeSymbology;
}
unsafe extern "C" {
    pub static VNVideoProcessingOptionFrameCadence: VNVideoProcessingOption;
}
unsafe extern "C" {
    pub static VNVideoProcessingOptionTimeInterval: VNVideoProcessingOption;
}
unsafe extern "C" {
    pub static VNAnimalBodyPoseObservationJointNameLeftEarTop: VNAnimalBodyPoseObservationJointName;
}
unsafe extern "C" {
    pub static VNAnimalBodyPoseObservationJointNameRightEarTop:
        VNAnimalBodyPoseObservationJointName;
}
unsafe extern "C" {
    pub static VNAnimalBodyPoseObservationJointNameLeftEarMiddle:
        VNAnimalBodyPoseObservationJointName;
}
unsafe extern "C" {
    pub static VNAnimalBodyPoseObservationJointNameRightEarMiddle:
        VNAnimalBodyPoseObservationJointName;
}
unsafe extern "C" {
    pub static VNAnimalBodyPoseObservationJointNameLeftEarBottom:
        VNAnimalBodyPoseObservationJointName;
}
unsafe extern "C" {
    pub static VNAnimalBodyPoseObservationJointNameRightEarBottom:
        VNAnimalBodyPoseObservationJointName;
}
unsafe extern "C" {
    pub static VNAnimalBodyPoseObservationJointNameLeftEye: VNAnimalBodyPoseObservationJointName;
}
unsafe extern "C" {
    pub static VNAnimalBodyPoseObservationJointNameRightEye: VNAnimalBodyPoseObservationJointName;
}
unsafe extern "C" {
    pub static VNAnimalBodyPoseObservationJointNameNose: VNAnimalBodyPoseObservationJointName;
}
unsafe extern "C" {
    pub static VNAnimalBodyPoseObservationJointNameNeck: VNAnimalBodyPoseObservationJointName;
}
unsafe extern "C" {
    pub static VNAnimalBodyPoseObservationJointNameLeftFrontElbow:
        VNAnimalBodyPoseObservationJointName;
}
unsafe extern "C" {
    pub static VNAnimalBodyPoseObservationJointNameRightFrontElbow:
        VNAnimalBodyPoseObservationJointName;
}
unsafe extern "C" {
    pub static VNAnimalBodyPoseObservationJointNameLeftFrontKnee:
        VNAnimalBodyPoseObservationJointName;
}
unsafe extern "C" {
    pub static VNAnimalBodyPoseObservationJointNameRightFrontKnee:
        VNAnimalBodyPoseObservationJointName;
}
unsafe extern "C" {
    pub static VNAnimalBodyPoseObservationJointNameLeftFrontPaw:
        VNAnimalBodyPoseObservationJointName;
}
unsafe extern "C" {
    pub static VNAnimalBodyPoseObservationJointNameRightFrontPaw:
        VNAnimalBodyPoseObservationJointName;
}
unsafe extern "C" {
    pub static VNAnimalBodyPoseObservationJointNameLeftBackElbow:
        VNAnimalBodyPoseObservationJointName;
}
unsafe extern "C" {
    pub static VNAnimalBodyPoseObservationJointNameRightBackElbow:
        VNAnimalBodyPoseObservationJointName;
}
unsafe extern "C" {
    pub static VNAnimalBodyPoseObservationJointNameLeftBackKnee:
        VNAnimalBodyPoseObservationJointName;
}
unsafe extern "C" {
    pub static VNAnimalBodyPoseObservationJointNameRightBackKnee:
        VNAnimalBodyPoseObservationJointName;
}
unsafe extern "C" {
    pub static VNAnimalBodyPoseObservationJointNameLeftBackPaw:
        VNAnimalBodyPoseObservationJointName;
}
unsafe extern "C" {
    pub static VNAnimalBodyPoseObservationJointNameRightBackPaw:
        VNAnimalBodyPoseObservationJointName;
}
unsafe extern "C" {
    pub static VNAnimalBodyPoseObservationJointNameTailTop: VNAnimalBodyPoseObservationJointName;
}
unsafe extern "C" {
    pub static VNAnimalBodyPoseObservationJointNameTailMiddle: VNAnimalBodyPoseObservationJointName;
}
unsafe extern "C" {
    pub static VNAnimalBodyPoseObservationJointNameTailBottom: VNAnimalBodyPoseObservationJointName;
}
unsafe extern "C" {
    pub static VNAnimalBodyPoseObservationJointsGroupNameHead:
        VNAnimalBodyPoseObservationJointsGroupName;
}
unsafe extern "C" {
    pub static VNAnimalBodyPoseObservationJointsGroupNameTrunk:
        VNAnimalBodyPoseObservationJointsGroupName;
}
unsafe extern "C" {
    pub static VNAnimalBodyPoseObservationJointsGroupNameForelegs:
        VNAnimalBodyPoseObservationJointsGroupName;
}
unsafe extern "C" {
    pub static VNAnimalBodyPoseObservationJointsGroupNameHindlegs:
        VNAnimalBodyPoseObservationJointsGroupName;
}
unsafe extern "C" {
    pub static VNAnimalBodyPoseObservationJointsGroupNameTail:
        VNAnimalBodyPoseObservationJointsGroupName;
}
unsafe extern "C" {
    pub static VNAnimalBodyPoseObservationJointsGroupNameAll:
        VNAnimalBodyPoseObservationJointsGroupName;
}
unsafe extern "C" {
    pub static VNHumanBodyPose3DObservationJointNameRoot: VNHumanBodyPose3DObservationJointName;
}
unsafe extern "C" {
    pub static VNHumanBodyPose3DObservationJointNameRightHip: VNHumanBodyPose3DObservationJointName;
}
unsafe extern "C" {
    pub static VNHumanBodyPose3DObservationJointNameRightKnee:
        VNHumanBodyPose3DObservationJointName;
}
unsafe extern "C" {
    pub static VNHumanBodyPose3DObservationJointNameRightAnkle:
        VNHumanBodyPose3DObservationJointName;
}
unsafe extern "C" {
    pub static VNHumanBodyPose3DObservationJointNameLeftHip: VNHumanBodyPose3DObservationJointName;
}
unsafe extern "C" {
    pub static VNHumanBodyPose3DObservationJointNameLeftKnee: VNHumanBodyPose3DObservationJointName;
}
unsafe extern "C" {
    pub static VNHumanBodyPose3DObservationJointNameLeftAnkle:
        VNHumanBodyPose3DObservationJointName;
}
unsafe extern "C" {
    pub static VNHumanBodyPose3DObservationJointNameSpine: VNHumanBodyPose3DObservationJointName;
}
unsafe extern "C" {
    pub static VNHumanBodyPose3DObservationJointNameCenterShoulder:
        VNHumanBodyPose3DObservationJointName;
}
unsafe extern "C" {
    pub static VNHumanBodyPose3DObservationJointNameCenterHead:
        VNHumanBodyPose3DObservationJointName;
}
unsafe extern "C" {
    pub static VNHumanBodyPose3DObservationJointNameTopHead: VNHumanBodyPose3DObservationJointName;
}
unsafe extern "C" {
    pub static VNHumanBodyPose3DObservationJointNameLeftShoulder:
        VNHumanBodyPose3DObservationJointName;
}
unsafe extern "C" {
    pub static VNHumanBodyPose3DObservationJointNameLeftElbow:
        VNHumanBodyPose3DObservationJointName;
}
unsafe extern "C" {
    pub static VNHumanBodyPose3DObservationJointNameLeftWrist:
        VNHumanBodyPose3DObservationJointName;
}
unsafe extern "C" {
    pub static VNHumanBodyPose3DObservationJointNameRightShoulder:
        VNHumanBodyPose3DObservationJointName;
}
unsafe extern "C" {
    pub static VNHumanBodyPose3DObservationJointNameRightElbow:
        VNHumanBodyPose3DObservationJointName;
}
unsafe extern "C" {
    pub static VNHumanBodyPose3DObservationJointNameRightWrist:
        VNHumanBodyPose3DObservationJointName;
}
unsafe extern "C" {
    pub static VNHumanBodyPose3DObservationJointsGroupNameHead:
        VNHumanBodyPose3DObservationJointsGroupName;
}
unsafe extern "C" {
    pub static VNHumanBodyPose3DObservationJointsGroupNameTorso:
        VNHumanBodyPose3DObservationJointsGroupName;
}
unsafe extern "C" {
    pub static VNHumanBodyPose3DObservationJointsGroupNameLeftArm:
        VNHumanBodyPose3DObservationJointsGroupName;
}
unsafe extern "C" {
    pub static VNHumanBodyPose3DObservationJointsGroupNameRightArm:
        VNHumanBodyPose3DObservationJointsGroupName;
}
unsafe extern "C" {
    pub static VNHumanBodyPose3DObservationJointsGroupNameLeftLeg:
        VNHumanBodyPose3DObservationJointsGroupName;
}
unsafe extern "C" {
    pub static VNHumanBodyPose3DObservationJointsGroupNameRightLeg:
        VNHumanBodyPose3DObservationJointsGroupName;
}
unsafe extern "C" {
    pub static VNHumanBodyPose3DObservationJointsGroupNameAll:
        VNHumanBodyPose3DObservationJointsGroupName;
}
unsafe extern "C" {
    pub static VNErrorDomain: NSString;
}
unsafe extern "C" {
    pub static VNNormalizedIdentityRect: CGRect;
}
unsafe extern "C" {
    pub fn VNNormalizedRectIsIdentityRect(normalizedRect: CGRect) -> bool;
}
unsafe extern "C" {
    pub fn VNImagePointForNormalizedPoint(
        normalizedPoint: CGPoint,
        imageWidth: usize,
        imageHeight: usize,
    ) -> CGPoint;
}
unsafe extern "C" {
    pub fn VNImagePointForNormalizedPointUsingRegionOfInterest(
        normalizedPoint: CGPoint,
        imageWidth: usize,
        imageHeight: usize,
        roi: CGRect,
    ) -> CGPoint;
}
unsafe extern "C" {
    pub fn VNNormalizedPointForImagePoint(
        imagePoint: CGPoint,
        imageWidth: usize,
        imageHeight: usize,
    ) -> CGPoint;
}
unsafe extern "C" {
    pub fn VNNormalizedPointForImagePointUsingRegionOfInterest(
        imagePoint: CGPoint,
        imageWidth: usize,
        imageHeight: usize,
        roi: CGRect,
    ) -> CGPoint;
}
unsafe extern "C" {
    pub fn VNImageRectForNormalizedRect(
        normalizedRect: CGRect,
        imageWidth: usize,
        imageHeight: usize,
    ) -> CGRect;
}
unsafe extern "C" {
    pub fn VNImageRectForNormalizedRectUsingRegionOfInterest(
        normalizedRect: CGRect,
        imageWidth: usize,
        imageHeight: usize,
        roi: CGRect,
    ) -> CGRect;
}
unsafe extern "C" {
    pub fn VNNormalizedRectForImageRect(
        imageRect: CGRect,
        imageWidth: usize,
        imageHeight: usize,
    ) -> CGRect;
}
unsafe extern "C" {
    pub fn VNNormalizedRectForImageRectUsingRegionOfInterest(
        imageRect: CGRect,
        imageWidth: usize,
        imageHeight: usize,
        roi: CGRect,
    ) -> CGRect;
}
unsafe extern "C" {
    pub fn VNNormalizedFaceBoundingBoxPointForLandmarkPoint(
        faceLandmarkPoint: vector_float2,
        faceBoundingBox: CGRect,
        imageWidth: usize,
        imageHeight: usize,
    ) -> CGPoint;
}
unsafe extern "C" {
    pub fn VNImagePointForFaceLandmarkPoint(
        faceLandmarkPoint: vector_float2,
        faceBoundingBox: CGRect,
        imageWidth: usize,
        imageHeight: usize,
    ) -> CGPoint;
}
unsafe extern "C" {
    pub fn VNElementTypeSize(elementType: VNElementType) -> NSUInteger;
}
unsafe extern "C" {
    pub static VNRecognizedPointGroupKeyAll: VNRecognizedPointGroupKey;
}
unsafe extern "C" {
    pub static VNRecognizedPoint3DGroupKeyAll: VNRecognizedPointGroupKey;
}
unsafe extern "C" {
    pub static VNImageOptionProperties: VNImageOption;
}
unsafe extern "C" {
    pub static VNImageOptionCameraIntrinsics: VNImageOption;
}
unsafe extern "C" {
    pub static VNImageOptionCIContext: VNImageOption;
}
unsafe extern "C" {
    pub static VNAnimalIdentifierDog: VNAnimalIdentifier;
}
unsafe extern "C" {
    pub static VNAnimalIdentifierCat: VNAnimalIdentifier;
}
unsafe extern "C" {
    pub static VNHumanHandPoseObservationJointNameWrist: VNHumanHandPoseObservationJointName;
}
unsafe extern "C" {
    pub static VNHumanHandPoseObservationJointNameThumbCMC: VNHumanHandPoseObservationJointName;
}
unsafe extern "C" {
    pub static VNHumanHandPoseObservationJointNameThumbMP: VNHumanHandPoseObservationJointName;
}
unsafe extern "C" {
    pub static VNHumanHandPoseObservationJointNameThumbIP: VNHumanHandPoseObservationJointName;
}
unsafe extern "C" {
    pub static VNHumanHandPoseObservationJointNameThumbTip: VNHumanHandPoseObservationJointName;
}
unsafe extern "C" {
    pub static VNHumanHandPoseObservationJointNameIndexMCP: VNHumanHandPoseObservationJointName;
}
unsafe extern "C" {
    pub static VNHumanHandPoseObservationJointNameIndexPIP: VNHumanHandPoseObservationJointName;
}
unsafe extern "C" {
    pub static VNHumanHandPoseObservationJointNameIndexDIP: VNHumanHandPoseObservationJointName;
}
unsafe extern "C" {
    pub static VNHumanHandPoseObservationJointNameIndexTip: VNHumanHandPoseObservationJointName;
}
unsafe extern "C" {
    pub static VNHumanHandPoseObservationJointNameMiddleMCP: VNHumanHandPoseObservationJointName;
}
unsafe extern "C" {
    pub static VNHumanHandPoseObservationJointNameMiddlePIP: VNHumanHandPoseObservationJointName;
}
unsafe extern "C" {
    pub static VNHumanHandPoseObservationJointNameMiddleDIP: VNHumanHandPoseObservationJointName;
}
unsafe extern "C" {
    pub static VNHumanHandPoseObservationJointNameMiddleTip: VNHumanHandPoseObservationJointName;
}
unsafe extern "C" {
    pub static VNHumanHandPoseObservationJointNameRingMCP: VNHumanHandPoseObservationJointName;
}
unsafe extern "C" {
    pub static VNHumanHandPoseObservationJointNameRingPIP: VNHumanHandPoseObservationJointName;
}
unsafe extern "C" {
    pub static VNHumanHandPoseObservationJointNameRingDIP: VNHumanHandPoseObservationJointName;
}
unsafe extern "C" {
    pub static VNHumanHandPoseObservationJointNameRingTip: VNHumanHandPoseObservationJointName;
}
unsafe extern "C" {
    pub static VNHumanHandPoseObservationJointNameLittleMCP: VNHumanHandPoseObservationJointName;
}
unsafe extern "C" {
    pub static VNHumanHandPoseObservationJointNameLittlePIP: VNHumanHandPoseObservationJointName;
}
unsafe extern "C" {
    pub static VNHumanHandPoseObservationJointNameLittleDIP: VNHumanHandPoseObservationJointName;
}
unsafe extern "C" {
    pub static VNHumanHandPoseObservationJointNameLittleTip: VNHumanHandPoseObservationJointName;
}
unsafe extern "C" {
    pub static VNHumanHandPoseObservationJointsGroupNameThumb:
        VNHumanHandPoseObservationJointsGroupName;
}
unsafe extern "C" {
    pub static VNHumanHandPoseObservationJointsGroupNameIndexFinger:
        VNHumanHandPoseObservationJointsGroupName;
}
unsafe extern "C" {
    pub static VNHumanHandPoseObservationJointsGroupNameMiddleFinger:
        VNHumanHandPoseObservationJointsGroupName;
}
unsafe extern "C" {
    pub static VNHumanHandPoseObservationJointsGroupNameRingFinger:
        VNHumanHandPoseObservationJointsGroupName;
}
unsafe extern "C" {
    pub static VNHumanHandPoseObservationJointsGroupNameLittleFinger:
        VNHumanHandPoseObservationJointsGroupName;
}
unsafe extern "C" {
    pub static VNHumanHandPoseObservationJointsGroupNameAll:
        VNHumanHandPoseObservationJointsGroupName;
}
unsafe extern "C" {
    pub static VNBodyLandmarkKeyNose: VNRecognizedPointKey;
}
unsafe extern "C" {
    pub static VNBodyLandmarkKeyLeftEye: VNRecognizedPointKey;
}
unsafe extern "C" {
    pub static VNBodyLandmarkKeyRightEye: VNRecognizedPointKey;
}
unsafe extern "C" {
    pub static VNBodyLandmarkKeyLeftEar: VNRecognizedPointKey;
}
unsafe extern "C" {
    pub static VNBodyLandmarkKeyRightEar: VNRecognizedPointKey;
}
unsafe extern "C" {
    pub static VNBodyLandmarkKeyLeftShoulder: VNRecognizedPointKey;
}
unsafe extern "C" {
    pub static VNBodyLandmarkKeyRightShoulder: VNRecognizedPointKey;
}
unsafe extern "C" {
    pub static VNBodyLandmarkKeyNeck: VNRecognizedPointKey;
}
unsafe extern "C" {
    pub static VNBodyLandmarkKeyLeftElbow: VNRecognizedPointKey;
}
unsafe extern "C" {
    pub static VNBodyLandmarkKeyRightElbow: VNRecognizedPointKey;
}
unsafe extern "C" {
    pub static VNBodyLandmarkKeyLeftWrist: VNRecognizedPointKey;
}
unsafe extern "C" {
    pub static VNBodyLandmarkKeyRightWrist: VNRecognizedPointKey;
}
unsafe extern "C" {
    pub static VNBodyLandmarkKeyLeftHip: VNRecognizedPointKey;
}
unsafe extern "C" {
    pub static VNBodyLandmarkKeyRightHip: VNRecognizedPointKey;
}
unsafe extern "C" {
    pub static VNBodyLandmarkKeyRoot: VNRecognizedPointKey;
}
unsafe extern "C" {
    pub static VNBodyLandmarkKeyLeftKnee: VNRecognizedPointKey;
}
unsafe extern "C" {
    pub static VNBodyLandmarkKeyRightKnee: VNRecognizedPointKey;
}
unsafe extern "C" {
    pub static VNBodyLandmarkKeyLeftAnkle: VNRecognizedPointKey;
}
unsafe extern "C" {
    pub static VNBodyLandmarkKeyRightAnkle: VNRecognizedPointKey;
}
unsafe extern "C" {
    pub static VNBodyLandmarkRegionKeyFace: VNRecognizedPointGroupKey;
}
unsafe extern "C" {
    pub static VNBodyLandmarkRegionKeyTorso: VNRecognizedPointGroupKey;
}
unsafe extern "C" {
    pub static VNBodyLandmarkRegionKeyLeftArm: VNRecognizedPointGroupKey;
}
unsafe extern "C" {
    pub static VNBodyLandmarkRegionKeyRightArm: VNRecognizedPointGroupKey;
}
unsafe extern "C" {
    pub static VNBodyLandmarkRegionKeyLeftLeg: VNRecognizedPointGroupKey;
}
unsafe extern "C" {
    pub static VNBodyLandmarkRegionKeyRightLeg: VNRecognizedPointGroupKey;
}
unsafe extern "C" {
    pub static VNHumanBodyPoseObservationJointNameNose: VNHumanBodyPoseObservationJointName;
}
unsafe extern "C" {
    pub static VNHumanBodyPoseObservationJointNameLeftEye: VNHumanBodyPoseObservationJointName;
}
unsafe extern "C" {
    pub static VNHumanBodyPoseObservationJointNameRightEye: VNHumanBodyPoseObservationJointName;
}
unsafe extern "C" {
    pub static VNHumanBodyPoseObservationJointNameLeftEar: VNHumanBodyPoseObservationJointName;
}
unsafe extern "C" {
    pub static VNHumanBodyPoseObservationJointNameRightEar: VNHumanBodyPoseObservationJointName;
}
unsafe extern "C" {
    pub static VNHumanBodyPoseObservationJointNameLeftShoulder: VNHumanBodyPoseObservationJointName;
}
unsafe extern "C" {
    pub static VNHumanBodyPoseObservationJointNameRightShoulder:
        VNHumanBodyPoseObservationJointName;
}
unsafe extern "C" {
    pub static VNHumanBodyPoseObservationJointNameNeck: VNHumanBodyPoseObservationJointName;
}
unsafe extern "C" {
    pub static VNHumanBodyPoseObservationJointNameLeftElbow: VNHumanBodyPoseObservationJointName;
}
unsafe extern "C" {
    pub static VNHumanBodyPoseObservationJointNameRightElbow: VNHumanBodyPoseObservationJointName;
}
unsafe extern "C" {
    pub static VNHumanBodyPoseObservationJointNameLeftWrist: VNHumanBodyPoseObservationJointName;
}
unsafe extern "C" {
    pub static VNHumanBodyPoseObservationJointNameRightWrist: VNHumanBodyPoseObservationJointName;
}
unsafe extern "C" {
    pub static VNHumanBodyPoseObservationJointNameLeftHip: VNHumanBodyPoseObservationJointName;
}
unsafe extern "C" {
    pub static VNHumanBodyPoseObservationJointNameRightHip: VNHumanBodyPoseObservationJointName;
}
unsafe extern "C" {
    pub static VNHumanBodyPoseObservationJointNameRoot: VNHumanBodyPoseObservationJointName;
}
unsafe extern "C" {
    pub static VNHumanBodyPoseObservationJointNameLeftKnee: VNHumanBodyPoseObservationJointName;
}
unsafe extern "C" {
    pub static VNHumanBodyPoseObservationJointNameRightKnee: VNHumanBodyPoseObservationJointName;
}
unsafe extern "C" {
    pub static VNHumanBodyPoseObservationJointNameLeftAnkle: VNHumanBodyPoseObservationJointName;
}
unsafe extern "C" {
    pub static VNHumanBodyPoseObservationJointNameRightAnkle: VNHumanBodyPoseObservationJointName;
}
unsafe extern "C" {
    pub static VNHumanBodyPoseObservationJointsGroupNameFace:
        VNHumanBodyPoseObservationJointsGroupName;
}
unsafe extern "C" {
    pub static VNHumanBodyPoseObservationJointsGroupNameTorso:
        VNHumanBodyPoseObservationJointsGroupName;
}
unsafe extern "C" {
    pub static VNHumanBodyPoseObservationJointsGroupNameLeftArm:
        VNHumanBodyPoseObservationJointsGroupName;
}
unsafe extern "C" {
    pub static VNHumanBodyPoseObservationJointsGroupNameRightArm:
        VNHumanBodyPoseObservationJointsGroupName;
}
unsafe extern "C" {
    pub static VNHumanBodyPoseObservationJointsGroupNameLeftLeg:
        VNHumanBodyPoseObservationJointsGroupName;
}
unsafe extern "C" {
    pub static VNHumanBodyPoseObservationJointsGroupNameRightLeg:
        VNHumanBodyPoseObservationJointsGroupName;
}
unsafe extern "C" {
    pub static VNHumanBodyPoseObservationJointsGroupNameAll:
        VNHumanBodyPoseObservationJointsGroupName;
}
unsafe extern "C" {
    pub static mut VNVisionVersionNumber: f64;
}

unsafe impl objc2::encode::RefEncode for VNFaceLandmarkRegion {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for VNFaceLandmarkRegion {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for VNFaceLandmarkRegion2D {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for VNFaceLandmarkRegion2D {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for VNFaceLandmarks {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for VNFaceLandmarks {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for VNFaceLandmarks2D {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for VNFaceLandmarks2D {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for VNPoint {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for VNPoint {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for VNPoint3D {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for VNPoint3D {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for VNVector {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for VNVector {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for VNCircle {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for VNCircle {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for VNContour {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for VNContour {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for VNDetectedPoint {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for VNDetectedPoint {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for VNRecognizedPoint {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for VNRecognizedPoint {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for VNObservation {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for VNObservation {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for VNDetectedObjectObservation {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for VNDetectedObjectObservation {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for VNFaceObservation {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for VNFaceObservation {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for VNClassificationObservation {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for VNClassificationObservation {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for VNRecognizedObjectObservation {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for VNRecognizedObjectObservation {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for VNCoreMLFeatureValueObservation {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for VNCoreMLFeatureValueObservation {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for VNPixelBufferObservation {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for VNPixelBufferObservation {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for VNRectangleObservation {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for VNRectangleObservation {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for VNTrajectoryObservation {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for VNTrajectoryObservation {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for VNTextObservation {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for VNTextObservation {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for VNRecognizedText {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for VNRecognizedText {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for VNRecognizedTextObservation {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for VNRecognizedTextObservation {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for VNBarcodeObservation {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for VNBarcodeObservation {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for VNHorizonObservation {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for VNHorizonObservation {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for VNImageAlignmentObservation {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for VNImageAlignmentObservation {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for VNImageTranslationAlignmentObservation {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for VNImageTranslationAlignmentObservation {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for VNImageHomographicAlignmentObservation {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for VNImageHomographicAlignmentObservation {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for VNSaliencyImageObservation {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for VNSaliencyImageObservation {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for VNFeaturePrintObservation {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for VNFeaturePrintObservation {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for VNContoursObservation {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for VNContoursObservation {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for VNRecognizedPointsObservation {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for VNRecognizedPointsObservation {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for VNHumanObservation {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for VNHumanObservation {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for VNInstanceMaskObservation {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for VNInstanceMaskObservation {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for VNAnimalBodyPoseObservation {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for VNAnimalBodyPoseObservation {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for VNRecognizedPoints3DObservation {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for VNRecognizedPoints3DObservation {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for VNHumanBodyPose3DObservation {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for VNHumanBodyPose3DObservation {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for VNImageAestheticsScoresObservation {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for VNImageAestheticsScoresObservation {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for VNRequest {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for VNRequest {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for VNImageBasedRequest {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for VNImageBasedRequest {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for VNClassifyImageRequest {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for VNClassifyImageRequest {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for VNDetectBarcodesRequest {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for VNDetectBarcodesRequest {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for VNDetectFaceRectanglesRequest {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for VNDetectFaceRectanglesRequest {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for VNDetectFaceLandmarksRequest {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for VNDetectFaceLandmarksRequest {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for VNDetectFaceCaptureQualityRequest {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for VNDetectFaceCaptureQualityRequest {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for VNDetectHorizonRequest {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for VNDetectHorizonRequest {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for VNDetectRectanglesRequest {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for VNDetectRectanglesRequest {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for VNDetectTextRectanglesRequest {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for VNDetectTextRectanglesRequest {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for VNRecognizeTextRequest {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for VNRecognizeTextRequest {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for VNGenerateAttentionBasedSaliencyImageRequest {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for VNGenerateAttentionBasedSaliencyImageRequest {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for VNGenerateObjectnessBasedSaliencyImageRequest {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for VNGenerateObjectnessBasedSaliencyImageRequest {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for VNGenerateImageFeaturePrintRequest {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for VNGenerateImageFeaturePrintRequest {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for VNCoreMLModel {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for VNCoreMLModel {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for VNCoreMLRequest {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for VNCoreMLRequest {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for VNImageRequestHandler {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for VNImageRequestHandler {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for VNSequenceRequestHandler {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for VNSequenceRequestHandler {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for VNTargetedImageRequest {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for VNTargetedImageRequest {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for VNImageRegistrationRequest {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for VNImageRegistrationRequest {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for VNTranslationalImageRegistrationRequest {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for VNTranslationalImageRegistrationRequest {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for VNHomographicImageRegistrationRequest {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for VNHomographicImageRegistrationRequest {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for VNTrackingRequest {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for VNTrackingRequest {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for VNTrackObjectRequest {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for VNTrackObjectRequest {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for VNTrackRectangleRequest {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for VNTrackRectangleRequest {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for VNDetectHumanRectanglesRequest {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for VNDetectHumanRectanglesRequest {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for VNRecognizeAnimalsRequest {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for VNRecognizeAnimalsRequest {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for VNGenerateOpticalFlowRequest {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for VNGenerateOpticalFlowRequest {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for VNVideoProcessorCadence {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for VNVideoProcessorCadence {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for VNVideoProcessorFrameRateCadence {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for VNVideoProcessorFrameRateCadence {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for VNVideoProcessorTimeIntervalCadence {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for VNVideoProcessorTimeIntervalCadence {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for VNVideoProcessorRequestProcessingOptions {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for VNVideoProcessorRequestProcessingOptions {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for VNVideoProcessor {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for VNVideoProcessor {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for VNHumanHandPoseObservation {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for VNHumanHandPoseObservation {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for VNDetectHumanHandPoseRequest {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for VNDetectHumanHandPoseRequest {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for VNHumanBodyPoseObservation {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for VNHumanBodyPoseObservation {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for VNDetectHumanBodyPoseRequest {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for VNDetectHumanBodyPoseRequest {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for VNDetectContoursRequest {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for VNDetectContoursRequest {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for VNGeometryUtils {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for VNGeometryUtils {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for VNStatefulRequest {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for VNStatefulRequest {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for VNDetectTrajectoriesRequest {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for VNDetectTrajectoriesRequest {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for VNGeneratePersonSegmentationRequest {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for VNGeneratePersonSegmentationRequest {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for VNDetectDocumentSegmentationRequest {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for VNDetectDocumentSegmentationRequest {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for VNGenerateForegroundInstanceMaskRequest {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for VNGenerateForegroundInstanceMaskRequest {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for VNGeneratePersonInstanceMaskRequest {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for VNGeneratePersonInstanceMaskRequest {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for VNTrackTranslationalImageRegistrationRequest {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for VNTrackTranslationalImageRegistrationRequest {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for VNTrackHomographicImageRegistrationRequest {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for VNTrackHomographicImageRegistrationRequest {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for VNTrackOpticalFlowRequest {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for VNTrackOpticalFlowRequest {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for VNDetectAnimalBodyPoseRequest {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for VNDetectAnimalBodyPoseRequest {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for VNDetectHumanBodyPose3DRequest {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for VNDetectHumanBodyPose3DRequest {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for VNRecognizedPoint3D {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for VNRecognizedPoint3D {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for VNHumanBodyRecognizedPoint3D {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for VNHumanBodyRecognizedPoint3D {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for VNCalculateImageAestheticsScoresRequest {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for VNCalculateImageAestheticsScoresRequest {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
