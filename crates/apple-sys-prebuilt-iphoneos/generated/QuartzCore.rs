#[allow(unused_imports)]
use crate::objc::*;
#[allow(unused_imports)]
use crate::CoreFoundation::*;
#[allow(unused_imports)]
use crate::CoreGraphics::*;
#[allow(unused_imports)]
use crate::CoreImage::*;
#[allow(unused_imports)]
use crate::CoreVideo::*;
#[allow(unused_imports)]
use crate::Foundation::*;
#[allow(unused_imports)]
use crate::Metal::*;
#[allow(unused_imports)]
use crate::Security::*;
#[allow(unused_imports)]
use libc::mach_port_t;

#[allow(unused_imports)]
use objc2::msg_send;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CVTimeStamp {
    pub version: u32,
    pub videoTimeScale: i32,
    pub videoTime: i64,
    pub hostTime: u64,
    pub rateScalar: f64,
    pub videoRefreshPeriod: i64,
    pub smpteTime: CVSMPTETime,
    pub flags: u64,
    pub reserved: u64,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CATransform3D {
    pub m11: CGFloat,
    pub m12: CGFloat,
    pub m13: CGFloat,
    pub m14: CGFloat,
    pub m21: CGFloat,
    pub m22: CGFloat,
    pub m23: CGFloat,
    pub m24: CGFloat,
    pub m31: CGFloat,
    pub m32: CGFloat,
    pub m33: CGFloat,
    pub m34: CGFloat,
    pub m41: CGFloat,
    pub m42: CGFloat,
    pub m43: CGFloat,
    pub m44: CGFloat,
}
pub trait NSValue_CATransform3DAdditions: Sized + std::ops::Deref {
    unsafe fn CATransform3DValue(&self) -> CATransform3D
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, CATransform3DValue)
    }
    unsafe fn valueWithCATransform3D_(t: CATransform3D) -> NSValue
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NSValue").unwrap(), valueWithCATransform3D : t)
    }
}
pub type CAMediaTimingFillMode = NSString;
pub trait PCAMediaTiming: Sized + std::ops::Deref {
    unsafe fn beginTime(&self) -> CFTimeInterval
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, beginTime)
    }
    unsafe fn setBeginTime_(&self, beginTime: CFTimeInterval)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setBeginTime : beginTime)
    }
    unsafe fn duration(&self) -> CFTimeInterval
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, duration)
    }
    unsafe fn setDuration_(&self, duration: CFTimeInterval)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDuration : duration)
    }
    unsafe fn speed(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, speed)
    }
    unsafe fn setSpeed_(&self, speed: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSpeed : speed)
    }
    unsafe fn timeOffset(&self) -> CFTimeInterval
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, timeOffset)
    }
    unsafe fn setTimeOffset_(&self, timeOffset: CFTimeInterval)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setTimeOffset : timeOffset)
    }
    unsafe fn repeatCount(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, repeatCount)
    }
    unsafe fn setRepeatCount_(&self, repeatCount: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRepeatCount : repeatCount)
    }
    unsafe fn repeatDuration(&self) -> CFTimeInterval
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, repeatDuration)
    }
    unsafe fn setRepeatDuration_(&self, repeatDuration: CFTimeInterval)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRepeatDuration : repeatDuration)
    }
    unsafe fn autoreverses(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, autoreverses)
    }
    unsafe fn setAutoreverses_(&self, autoreverses: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAutoreverses : autoreverses)
    }
    unsafe fn fillMode(&self) -> CAMediaTimingFillMode
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, fillMode)
    }
    unsafe fn setFillMode_(&self, fillMode: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setFillMode : fillMode)
    }
}
pub type CALayerContentsGravity = NSString;
pub type CALayerContentsFormat = NSString;
pub type CALayerContentsFilter = NSString;
pub type CALayerCornerCurve = NSString;
pub type CAAutoresizingMask = ::std::os::raw::c_uint;
pub type CAToneMapMode = NSString;
pub type CADynamicRange = NSString;
pub type CAEdgeAntialiasingMask = ::std::os::raw::c_uint;
pub type CACornerMask = NSUInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CALayer(pub id);
impl std::ops::Deref for CALayer {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CALayer {}
impl CALayer {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CALayer").unwrap(), alloc) })
    }
}
impl PNSSecureCoding for CALayer {}
impl PCAMediaTiming for CALayer {}
impl INSObject for CALayer {}
impl PNSObject for CALayer {}
impl std::convert::TryFrom<NSObject> for CALayer {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<CALayer, Self::Error> {
        let is_kind_of: bool = unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CALayer").unwrap()) };
        if is_kind_of {
            Ok(CALayer(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to CALayer")
        }
    }
}
impl ICALayer for CALayer {}
pub trait ICALayer: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn initWithLayer_(&self, layer: id) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithLayer : layer)
    }
    unsafe fn presentationLayer(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, presentationLayer)
    }
    unsafe fn modelLayer(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, modelLayer)
    }
    unsafe fn shouldArchiveValueForKey_(&self, key: NSString) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, shouldArchiveValueForKey : key)
    }
    unsafe fn affineTransform(&self) -> CGAffineTransform
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, affineTransform)
    }
    unsafe fn setAffineTransform_(&self, m: CGAffineTransform)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAffineTransform : m)
    }
    unsafe fn contentsAreFlipped(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, contentsAreFlipped)
    }
    unsafe fn removeFromSuperlayer(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, removeFromSuperlayer)
    }
    unsafe fn addSublayer_(&self, layer: CALayer)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, addSublayer : layer)
    }
    unsafe fn insertSublayer_atIndex_(&self, layer: CALayer, idx: ::std::os::raw::c_uint)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, insertSublayer : layer, atIndex : idx)
    }
    unsafe fn insertSublayer_below_(&self, layer: CALayer, sibling: CALayer)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, insertSublayer : layer, below : sibling)
    }
    unsafe fn insertSublayer_above_(&self, layer: CALayer, sibling: CALayer)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, insertSublayer : layer, above : sibling)
    }
    unsafe fn replaceSublayer_with_(&self, oldLayer: CALayer, newLayer: CALayer)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, replaceSublayer : oldLayer, with : newLayer)
    }
    unsafe fn convertPoint_fromLayer_(&self, p: CGPoint, l: CALayer) -> CGPoint
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, convertPoint : p, fromLayer : l)
    }
    unsafe fn convertPoint_toLayer_(&self, p: CGPoint, l: CALayer) -> CGPoint
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, convertPoint : p, toLayer : l)
    }
    unsafe fn convertRect_fromLayer_(&self, r: CGRect, l: CALayer) -> CGRect
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, convertRect : r, fromLayer : l)
    }
    unsafe fn convertRect_toLayer_(&self, r: CGRect, l: CALayer) -> CGRect
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, convertRect : r, toLayer : l)
    }
    unsafe fn convertTime_fromLayer_(&self, t: CFTimeInterval, l: CALayer) -> CFTimeInterval
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, convertTime : t, fromLayer : l)
    }
    unsafe fn convertTime_toLayer_(&self, t: CFTimeInterval, l: CALayer) -> CFTimeInterval
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, convertTime : t, toLayer : l)
    }
    unsafe fn hitTest_(&self, p: CGPoint) -> CALayer
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, hitTest : p)
    }
    unsafe fn containsPoint_(&self, p: CGPoint) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, containsPoint : p)
    }
    unsafe fn display(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, display)
    }
    unsafe fn setNeedsDisplay(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, setNeedsDisplay)
    }
    unsafe fn setNeedsDisplayInRect_(&self, r: CGRect)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setNeedsDisplayInRect : r)
    }
    unsafe fn needsDisplay(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, needsDisplay)
    }
    unsafe fn displayIfNeeded(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, displayIfNeeded)
    }
    unsafe fn drawInContext_(&self, ctx: CGContextRef)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, drawInContext : ctx)
    }
    unsafe fn renderInContext_(&self, ctx: CGContextRef)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, renderInContext : ctx)
    }
    unsafe fn preferredFrameSize(&self) -> CGSize
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, preferredFrameSize)
    }
    unsafe fn setNeedsLayout(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, setNeedsLayout)
    }
    unsafe fn needsLayout(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, needsLayout)
    }
    unsafe fn layoutIfNeeded(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, layoutIfNeeded)
    }
    unsafe fn layoutSublayers(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, layoutSublayers)
    }
    unsafe fn resizeSublayersWithOldSize_(&self, size: CGSize)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, resizeSublayersWithOldSize : size)
    }
    unsafe fn resizeWithOldSuperlayerSize_(&self, size: CGSize)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, resizeWithOldSuperlayerSize : size)
    }
    unsafe fn actionForKey_(&self, event: NSString) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, actionForKey : event)
    }
    unsafe fn addAnimation_forKey_(&self, anim: CAAnimation, key: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, addAnimation : anim, forKey : key)
    }
    unsafe fn removeAllAnimations(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, removeAllAnimations)
    }
    unsafe fn removeAnimationForKey_(&self, key: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, removeAnimationForKey : key)
    }
    unsafe fn animationKeys(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, animationKeys)
    }
    unsafe fn animationForKey_(&self, key: NSString) -> CAAnimation
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, animationForKey : key)
    }
    unsafe fn bounds(&self) -> CGRect
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, bounds)
    }
    unsafe fn setBounds_(&self, bounds: CGRect)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setBounds : bounds)
    }
    unsafe fn position(&self) -> CGPoint
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, position)
    }
    unsafe fn setPosition_(&self, position: CGPoint)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPosition : position)
    }
    unsafe fn zPosition(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, zPosition)
    }
    unsafe fn setZPosition_(&self, zPosition: CGFloat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setZPosition : zPosition)
    }
    unsafe fn anchorPoint(&self) -> CGPoint
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, anchorPoint)
    }
    unsafe fn setAnchorPoint_(&self, anchorPoint: CGPoint)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAnchorPoint : anchorPoint)
    }
    unsafe fn anchorPointZ(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, anchorPointZ)
    }
    unsafe fn setAnchorPointZ_(&self, anchorPointZ: CGFloat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAnchorPointZ : anchorPointZ)
    }
    unsafe fn transform(&self) -> CATransform3D
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, transform)
    }
    unsafe fn setTransform_(&self, transform: CATransform3D)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setTransform : transform)
    }
    unsafe fn frame(&self) -> CGRect
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, frame)
    }
    unsafe fn setFrame_(&self, frame: CGRect)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setFrame : frame)
    }
    unsafe fn isHidden(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isHidden)
    }
    unsafe fn setHidden_(&self, hidden: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setHidden : hidden)
    }
    unsafe fn isDoubleSided(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isDoubleSided)
    }
    unsafe fn setDoubleSided_(&self, doubleSided: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDoubleSided : doubleSided)
    }
    unsafe fn isGeometryFlipped(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isGeometryFlipped)
    }
    unsafe fn setGeometryFlipped_(&self, geometryFlipped: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setGeometryFlipped : geometryFlipped)
    }
    unsafe fn superlayer(&self) -> CALayer
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, superlayer)
    }
    unsafe fn sublayers(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, sublayers)
    }
    unsafe fn setSublayers_(&self, sublayers: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSublayers : sublayers)
    }
    unsafe fn sublayerTransform(&self) -> CATransform3D
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, sublayerTransform)
    }
    unsafe fn setSublayerTransform_(&self, sublayerTransform: CATransform3D)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSublayerTransform : sublayerTransform)
    }
    unsafe fn mask(&self) -> CALayer
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, mask)
    }
    unsafe fn setMask_(&self, mask: CALayer)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMask : mask)
    }
    unsafe fn masksToBounds(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, masksToBounds)
    }
    unsafe fn setMasksToBounds_(&self, masksToBounds: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMasksToBounds : masksToBounds)
    }
    unsafe fn contents(&self) -> id
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, contents)
    }
    unsafe fn setContents_(&self, contents: id)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setContents : contents)
    }
    unsafe fn contentsRect(&self) -> CGRect
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, contentsRect)
    }
    unsafe fn setContentsRect_(&self, contentsRect: CGRect)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setContentsRect : contentsRect)
    }
    unsafe fn contentsGravity(&self) -> CALayerContentsGravity
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, contentsGravity)
    }
    unsafe fn setContentsGravity_(&self, contentsGravity: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setContentsGravity : contentsGravity)
    }
    unsafe fn contentsScale(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, contentsScale)
    }
    unsafe fn setContentsScale_(&self, contentsScale: CGFloat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setContentsScale : contentsScale)
    }
    unsafe fn contentsCenter(&self) -> CGRect
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, contentsCenter)
    }
    unsafe fn setContentsCenter_(&self, contentsCenter: CGRect)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setContentsCenter : contentsCenter)
    }
    unsafe fn contentsFormat(&self) -> CALayerContentsFormat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, contentsFormat)
    }
    unsafe fn setContentsFormat_(&self, contentsFormat: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setContentsFormat : contentsFormat)
    }
    unsafe fn wantsExtendedDynamicRangeContent(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, wantsExtendedDynamicRangeContent)
    }
    unsafe fn setWantsExtendedDynamicRangeContent_(&self, wantsExtendedDynamicRangeContent: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setWantsExtendedDynamicRangeContent : wantsExtendedDynamicRangeContent)
    }
    unsafe fn toneMapMode(&self) -> CAToneMapMode
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, toneMapMode)
    }
    unsafe fn setToneMapMode_(&self, toneMapMode: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setToneMapMode : toneMapMode)
    }
    unsafe fn preferredDynamicRange(&self) -> CADynamicRange
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, preferredDynamicRange)
    }
    unsafe fn setPreferredDynamicRange_(&self, preferredDynamicRange: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPreferredDynamicRange : preferredDynamicRange)
    }
    unsafe fn contentsHeadroom(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, contentsHeadroom)
    }
    unsafe fn setContentsHeadroom_(&self, contentsHeadroom: CGFloat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setContentsHeadroom : contentsHeadroom)
    }
    unsafe fn wantsDynamicContentScaling(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, wantsDynamicContentScaling)
    }
    unsafe fn setWantsDynamicContentScaling_(&self, wantsDynamicContentScaling: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setWantsDynamicContentScaling : wantsDynamicContentScaling)
    }
    unsafe fn minificationFilter(&self) -> CALayerContentsFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, minificationFilter)
    }
    unsafe fn setMinificationFilter_(&self, minificationFilter: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMinificationFilter : minificationFilter)
    }
    unsafe fn magnificationFilter(&self) -> CALayerContentsFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, magnificationFilter)
    }
    unsafe fn setMagnificationFilter_(&self, magnificationFilter: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMagnificationFilter : magnificationFilter)
    }
    unsafe fn minificationFilterBias(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, minificationFilterBias)
    }
    unsafe fn setMinificationFilterBias_(&self, minificationFilterBias: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMinificationFilterBias : minificationFilterBias)
    }
    unsafe fn isOpaque(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isOpaque)
    }
    unsafe fn setOpaque_(&self, opaque: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setOpaque : opaque)
    }
    unsafe fn needsDisplayOnBoundsChange(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, needsDisplayOnBoundsChange)
    }
    unsafe fn setNeedsDisplayOnBoundsChange_(&self, needsDisplayOnBoundsChange: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setNeedsDisplayOnBoundsChange : needsDisplayOnBoundsChange)
    }
    unsafe fn drawsAsynchronously(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, drawsAsynchronously)
    }
    unsafe fn setDrawsAsynchronously_(&self, drawsAsynchronously: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDrawsAsynchronously : drawsAsynchronously)
    }
    unsafe fn edgeAntialiasingMask(&self) -> CAEdgeAntialiasingMask
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, edgeAntialiasingMask)
    }
    unsafe fn setEdgeAntialiasingMask_(&self, edgeAntialiasingMask: CAEdgeAntialiasingMask)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setEdgeAntialiasingMask : edgeAntialiasingMask)
    }
    unsafe fn allowsEdgeAntialiasing(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, allowsEdgeAntialiasing)
    }
    unsafe fn setAllowsEdgeAntialiasing_(&self, allowsEdgeAntialiasing: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAllowsEdgeAntialiasing : allowsEdgeAntialiasing)
    }
    unsafe fn backgroundColor(&self) -> CGColorRef
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, backgroundColor)
    }
    unsafe fn setBackgroundColor_(&self, backgroundColor: CGColorRef)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setBackgroundColor : backgroundColor)
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
    unsafe fn maskedCorners(&self) -> CACornerMask
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, maskedCorners)
    }
    unsafe fn setMaskedCorners_(&self, maskedCorners: CACornerMask)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMaskedCorners : maskedCorners)
    }
    unsafe fn cornerCurve(&self) -> CALayerCornerCurve
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, cornerCurve)
    }
    unsafe fn setCornerCurve_(&self, cornerCurve: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCornerCurve : cornerCurve)
    }
    unsafe fn borderWidth(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, borderWidth)
    }
    unsafe fn setBorderWidth_(&self, borderWidth: CGFloat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setBorderWidth : borderWidth)
    }
    unsafe fn borderColor(&self) -> CGColorRef
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, borderColor)
    }
    unsafe fn setBorderColor_(&self, borderColor: CGColorRef)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setBorderColor : borderColor)
    }
    unsafe fn opacity(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, opacity)
    }
    unsafe fn setOpacity_(&self, opacity: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setOpacity : opacity)
    }
    unsafe fn allowsGroupOpacity(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, allowsGroupOpacity)
    }
    unsafe fn setAllowsGroupOpacity_(&self, allowsGroupOpacity: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAllowsGroupOpacity : allowsGroupOpacity)
    }
    unsafe fn compositingFilter(&self) -> id
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, compositingFilter)
    }
    unsafe fn setCompositingFilter_(&self, compositingFilter: id)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCompositingFilter : compositingFilter)
    }
    unsafe fn filters(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, filters)
    }
    unsafe fn setFilters_(&self, filters: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setFilters : filters)
    }
    unsafe fn backgroundFilters(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, backgroundFilters)
    }
    unsafe fn setBackgroundFilters_(&self, backgroundFilters: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setBackgroundFilters : backgroundFilters)
    }
    unsafe fn shouldRasterize(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, shouldRasterize)
    }
    unsafe fn setShouldRasterize_(&self, shouldRasterize: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setShouldRasterize : shouldRasterize)
    }
    unsafe fn rasterizationScale(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, rasterizationScale)
    }
    unsafe fn setRasterizationScale_(&self, rasterizationScale: CGFloat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRasterizationScale : rasterizationScale)
    }
    unsafe fn shadowColor(&self) -> CGColorRef
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, shadowColor)
    }
    unsafe fn setShadowColor_(&self, shadowColor: CGColorRef)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setShadowColor : shadowColor)
    }
    unsafe fn shadowOpacity(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, shadowOpacity)
    }
    unsafe fn setShadowOpacity_(&self, shadowOpacity: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setShadowOpacity : shadowOpacity)
    }
    unsafe fn shadowOffset(&self) -> CGSize
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, shadowOffset)
    }
    unsafe fn setShadowOffset_(&self, shadowOffset: CGSize)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setShadowOffset : shadowOffset)
    }
    unsafe fn shadowRadius(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, shadowRadius)
    }
    unsafe fn setShadowRadius_(&self, shadowRadius: CGFloat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setShadowRadius : shadowRadius)
    }
    unsafe fn shadowPath(&self) -> CGPathRef
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, shadowPath)
    }
    unsafe fn setShadowPath_(&self, shadowPath: CGPathRef)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setShadowPath : shadowPath)
    }
    unsafe fn autoresizingMask(&self) -> CAAutoresizingMask
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, autoresizingMask)
    }
    unsafe fn setAutoresizingMask_(&self, autoresizingMask: CAAutoresizingMask)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAutoresizingMask : autoresizingMask)
    }
    unsafe fn layoutManager(&self) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, layoutManager)
    }
    unsafe fn setLayoutManager_(&self, layoutManager: *mut u64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setLayoutManager : layoutManager)
    }
    unsafe fn actions(&self) -> NSDictionary
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, actions)
    }
    unsafe fn setActions_(&self, actions: NSDictionary)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setActions : actions)
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
    unsafe fn style(&self) -> NSDictionary
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, style)
    }
    unsafe fn setStyle_(&self, style: NSDictionary)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setStyle : style)
    }
    unsafe fn layer() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CALayer").unwrap(), layer)
    }
    unsafe fn defaultValueForKey_(key: NSString) -> id
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CALayer").unwrap(), defaultValueForKey : key)
    }
    unsafe fn needsDisplayForKey_(key: NSString) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CALayer").unwrap(), needsDisplayForKey : key)
    }
    unsafe fn cornerCurveExpansionFactor_(curve: NSString) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CALayer").unwrap(), cornerCurveExpansionFactor : curve)
    }
    unsafe fn defaultActionForKey_(event: NSString) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CALayer").unwrap(), defaultActionForKey : event)
    }
}
pub trait PCALayoutManager: Sized + std::ops::Deref {
    unsafe fn preferredSizeOfLayer_(&self, layer: CALayer) -> CGSize
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, preferredSizeOfLayer : layer)
    }
    unsafe fn invalidateLayoutOfLayer_(&self, layer: CALayer)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, invalidateLayoutOfLayer : layer)
    }
    unsafe fn layoutSublayersOfLayer_(&self, layer: CALayer)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, layoutSublayersOfLayer : layer)
    }
}
pub trait PCAAction: Sized + std::ops::Deref {
    unsafe fn runActionForKey_object_arguments_(
        &self,
        event: NSString,
        anObject: id,
        dict: NSDictionary,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, runActionForKey : event, object : anObject, arguments : dict)
    }
}
pub trait NSNull_CAActionAdditions: Sized + std::ops::Deref {}
pub trait PCALayerDelegate: Sized + std::ops::Deref {
    unsafe fn displayLayer_(&self, layer: CALayer)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, displayLayer : layer)
    }
    unsafe fn drawLayer_inContext_(&self, layer: CALayer, ctx: CGContextRef)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, drawLayer : layer, inContext : ctx)
    }
    unsafe fn layerWillDraw_(&self, layer: CALayer)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, layerWillDraw : layer)
    }
    unsafe fn layoutSublayersOfLayer_(&self, layer: CALayer)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, layoutSublayersOfLayer : layer)
    }
    unsafe fn actionForLayer_forKey_(&self, layer: CALayer, event: NSString) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, actionForLayer : layer, forKey : event)
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CAFrameRateRange {
    pub minimum: f32,
    pub maximum: f32,
    pub preferred: f32,
}
pub type CAAnimationCalculationMode = NSString;
pub type CAAnimationRotationMode = NSString;
pub type CATransitionType = NSString;
pub type CATransitionSubtype = NSString;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CAAnimation(pub id);
impl std::ops::Deref for CAAnimation {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CAAnimation {}
impl CAAnimation {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CAAnimation").unwrap(), alloc) })
    }
}
impl PNSSecureCoding for CAAnimation {}
impl PNSCopying for CAAnimation {}
impl PCAMediaTiming for CAAnimation {}
impl PCAAction for CAAnimation {}
impl INSObject for CAAnimation {}
impl PNSObject for CAAnimation {}
impl std::convert::TryFrom<NSObject> for CAAnimation {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<CAAnimation, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CAAnimation").unwrap()) };
        if is_kind_of {
            Ok(CAAnimation(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to CAAnimation")
        }
    }
}
impl ICAAnimation for CAAnimation {}
pub trait ICAAnimation: Sized + std::ops::Deref {
    unsafe fn shouldArchiveValueForKey_(&self, key: NSString) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, shouldArchiveValueForKey : key)
    }
    unsafe fn timingFunction(&self) -> CAMediaTimingFunction
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, timingFunction)
    }
    unsafe fn setTimingFunction_(&self, timingFunction: CAMediaTimingFunction)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setTimingFunction : timingFunction)
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
    unsafe fn isRemovedOnCompletion(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isRemovedOnCompletion)
    }
    unsafe fn setRemovedOnCompletion_(&self, removedOnCompletion: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRemovedOnCompletion : removedOnCompletion)
    }
    unsafe fn preferredFrameRateRange(&self) -> CAFrameRateRange
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, preferredFrameRateRange)
    }
    unsafe fn setPreferredFrameRateRange_(&self, preferredFrameRateRange: CAFrameRateRange)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPreferredFrameRateRange : preferredFrameRateRange)
    }
    unsafe fn animation() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CAAnimation").unwrap(), animation)
    }
    unsafe fn defaultValueForKey_(key: NSString) -> id
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CAAnimation").unwrap(), defaultValueForKey : key)
    }
}
pub trait PCAAnimationDelegate: Sized + std::ops::Deref {
    unsafe fn animationDidStart_(&self, anim: CAAnimation)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, animationDidStart : anim)
    }
    unsafe fn animationDidStop_finished_(&self, anim: CAAnimation, flag: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, animationDidStop : anim, finished : flag)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CAPropertyAnimation(pub id);
impl std::ops::Deref for CAPropertyAnimation {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CAPropertyAnimation {}
impl CAPropertyAnimation {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CAPropertyAnimation").unwrap(), alloc) })
    }
}
impl ICAAnimation for CAPropertyAnimation {}
impl PNSSecureCoding for CAPropertyAnimation {}
impl PNSCopying for CAPropertyAnimation {}
impl PCAMediaTiming for CAPropertyAnimation {}
impl PCAAction for CAPropertyAnimation {}
impl From<CAPropertyAnimation> for CAAnimation {
    fn from(child: CAPropertyAnimation) -> CAAnimation {
        CAAnimation(child.0)
    }
}
impl std::convert::TryFrom<CAAnimation> for CAPropertyAnimation {
    type Error = &'static str;
    fn try_from(parent: CAAnimation) -> Result<CAPropertyAnimation, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CAPropertyAnimation").unwrap()) };
        if is_kind_of {
            Ok(CAPropertyAnimation(parent.0))
        } else {
            Err("This CAAnimation cannot be downcasted to CAPropertyAnimation")
        }
    }
}
impl INSObject for CAPropertyAnimation {}
impl PNSObject for CAPropertyAnimation {}
impl ICAPropertyAnimation for CAPropertyAnimation {}
pub trait ICAPropertyAnimation: Sized + std::ops::Deref {
    unsafe fn keyPath(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, keyPath)
    }
    unsafe fn setKeyPath_(&self, keyPath: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setKeyPath : keyPath)
    }
    unsafe fn isAdditive(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isAdditive)
    }
    unsafe fn setAdditive_(&self, additive: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAdditive : additive)
    }
    unsafe fn isCumulative(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isCumulative)
    }
    unsafe fn setCumulative_(&self, cumulative: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCumulative : cumulative)
    }
    unsafe fn valueFunction(&self) -> CAValueFunction
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, valueFunction)
    }
    unsafe fn setValueFunction_(&self, valueFunction: CAValueFunction)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setValueFunction : valueFunction)
    }
    unsafe fn animationWithKeyPath_(path: NSString) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CAPropertyAnimation").unwrap(), animationWithKeyPath : path)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CABasicAnimation(pub id);
impl std::ops::Deref for CABasicAnimation {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CABasicAnimation {}
impl CABasicAnimation {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CABasicAnimation").unwrap(), alloc) })
    }
}
impl ICAPropertyAnimation for CABasicAnimation {}
impl From<CABasicAnimation> for CAPropertyAnimation {
    fn from(child: CABasicAnimation) -> CAPropertyAnimation {
        CAPropertyAnimation(child.0)
    }
}
impl std::convert::TryFrom<CAPropertyAnimation> for CABasicAnimation {
    type Error = &'static str;
    fn try_from(parent: CAPropertyAnimation) -> Result<CABasicAnimation, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CABasicAnimation").unwrap()) };
        if is_kind_of {
            Ok(CABasicAnimation(parent.0))
        } else {
            Err("This CAPropertyAnimation cannot be downcasted to CABasicAnimation")
        }
    }
}
impl ICAAnimation for CABasicAnimation {}
impl PNSSecureCoding for CABasicAnimation {}
impl PNSCopying for CABasicAnimation {}
impl PCAMediaTiming for CABasicAnimation {}
impl PCAAction for CABasicAnimation {}
impl INSObject for CABasicAnimation {}
impl PNSObject for CABasicAnimation {}
impl ICABasicAnimation for CABasicAnimation {}
pub trait ICABasicAnimation: Sized + std::ops::Deref {
    unsafe fn fromValue(&self) -> id
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, fromValue)
    }
    unsafe fn setFromValue_(&self, fromValue: id)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setFromValue : fromValue)
    }
    unsafe fn toValue(&self) -> id
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, toValue)
    }
    unsafe fn setToValue_(&self, toValue: id)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setToValue : toValue)
    }
    unsafe fn byValue(&self) -> id
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, byValue)
    }
    unsafe fn setByValue_(&self, byValue: id)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setByValue : byValue)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CAKeyframeAnimation(pub id);
impl std::ops::Deref for CAKeyframeAnimation {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CAKeyframeAnimation {}
impl CAKeyframeAnimation {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CAKeyframeAnimation").unwrap(), alloc) })
    }
}
impl ICAPropertyAnimation for CAKeyframeAnimation {}
impl std::convert::TryFrom<CAPropertyAnimation> for CAKeyframeAnimation {
    type Error = &'static str;
    fn try_from(parent: CAPropertyAnimation) -> Result<CAKeyframeAnimation, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CAKeyframeAnimation").unwrap()) };
        if is_kind_of {
            Ok(CAKeyframeAnimation(parent.0))
        } else {
            Err("This CAPropertyAnimation cannot be downcasted to CAKeyframeAnimation")
        }
    }
}
impl ICAAnimation for CAKeyframeAnimation {}
impl PNSSecureCoding for CAKeyframeAnimation {}
impl PNSCopying for CAKeyframeAnimation {}
impl PCAMediaTiming for CAKeyframeAnimation {}
impl PCAAction for CAKeyframeAnimation {}
impl INSObject for CAKeyframeAnimation {}
impl PNSObject for CAKeyframeAnimation {}
impl ICAKeyframeAnimation for CAKeyframeAnimation {}
pub trait ICAKeyframeAnimation: Sized + std::ops::Deref {
    unsafe fn values(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, values)
    }
    unsafe fn setValues_(&self, values: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setValues : values)
    }
    unsafe fn path(&self) -> CGPathRef
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, path)
    }
    unsafe fn setPath_(&self, path: CGPathRef)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPath : path)
    }
    unsafe fn keyTimes(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, keyTimes)
    }
    unsafe fn setKeyTimes_(&self, keyTimes: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setKeyTimes : keyTimes)
    }
    unsafe fn timingFunctions(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, timingFunctions)
    }
    unsafe fn setTimingFunctions_(&self, timingFunctions: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setTimingFunctions : timingFunctions)
    }
    unsafe fn calculationMode(&self) -> CAAnimationCalculationMode
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, calculationMode)
    }
    unsafe fn setCalculationMode_(&self, calculationMode: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCalculationMode : calculationMode)
    }
    unsafe fn tensionValues(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, tensionValues)
    }
    unsafe fn setTensionValues_(&self, tensionValues: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setTensionValues : tensionValues)
    }
    unsafe fn continuityValues(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, continuityValues)
    }
    unsafe fn setContinuityValues_(&self, continuityValues: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setContinuityValues : continuityValues)
    }
    unsafe fn biasValues(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, biasValues)
    }
    unsafe fn setBiasValues_(&self, biasValues: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setBiasValues : biasValues)
    }
    unsafe fn rotationMode(&self) -> CAAnimationRotationMode
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, rotationMode)
    }
    unsafe fn setRotationMode_(&self, rotationMode: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRotationMode : rotationMode)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CASpringAnimation(pub id);
impl std::ops::Deref for CASpringAnimation {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CASpringAnimation {}
impl CASpringAnimation {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CASpringAnimation").unwrap(), alloc) })
    }
}
impl ICABasicAnimation for CASpringAnimation {}
impl From<CASpringAnimation> for CABasicAnimation {
    fn from(child: CASpringAnimation) -> CABasicAnimation {
        CABasicAnimation(child.0)
    }
}
impl std::convert::TryFrom<CABasicAnimation> for CASpringAnimation {
    type Error = &'static str;
    fn try_from(parent: CABasicAnimation) -> Result<CASpringAnimation, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CASpringAnimation").unwrap()) };
        if is_kind_of {
            Ok(CASpringAnimation(parent.0))
        } else {
            Err("This CABasicAnimation cannot be downcasted to CASpringAnimation")
        }
    }
}
impl ICAPropertyAnimation for CASpringAnimation {}
impl ICAAnimation for CASpringAnimation {}
impl PNSSecureCoding for CASpringAnimation {}
impl PNSCopying for CASpringAnimation {}
impl PCAMediaTiming for CASpringAnimation {}
impl PCAAction for CASpringAnimation {}
impl INSObject for CASpringAnimation {}
impl PNSObject for CASpringAnimation {}
impl ICASpringAnimation for CASpringAnimation {}
pub trait ICASpringAnimation: Sized + std::ops::Deref {
    unsafe fn initWithPerceptualDuration_bounce_(
        &self,
        perceptualDuration: CFTimeInterval,
        bounce: CGFloat,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithPerceptualDuration : perceptualDuration, bounce : bounce)
    }
    unsafe fn mass(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, mass)
    }
    unsafe fn setMass_(&self, mass: CGFloat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMass : mass)
    }
    unsafe fn stiffness(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, stiffness)
    }
    unsafe fn setStiffness_(&self, stiffness: CGFloat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setStiffness : stiffness)
    }
    unsafe fn damping(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, damping)
    }
    unsafe fn setDamping_(&self, damping: CGFloat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDamping : damping)
    }
    unsafe fn initialVelocity(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, initialVelocity)
    }
    unsafe fn setInitialVelocity_(&self, initialVelocity: CGFloat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInitialVelocity : initialVelocity)
    }
    unsafe fn allowsOverdamping(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, allowsOverdamping)
    }
    unsafe fn setAllowsOverdamping_(&self, allowsOverdamping: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAllowsOverdamping : allowsOverdamping)
    }
    unsafe fn settlingDuration(&self) -> CFTimeInterval
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, settlingDuration)
    }
    unsafe fn perceptualDuration(&self) -> CFTimeInterval
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, perceptualDuration)
    }
    unsafe fn bounce(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, bounce)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CATransition(pub id);
impl std::ops::Deref for CATransition {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CATransition {}
impl CATransition {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CATransition").unwrap(), alloc) })
    }
}
impl ICAAnimation for CATransition {}
impl PNSSecureCoding for CATransition {}
impl PNSCopying for CATransition {}
impl PCAMediaTiming for CATransition {}
impl PCAAction for CATransition {}
impl std::convert::TryFrom<CAAnimation> for CATransition {
    type Error = &'static str;
    fn try_from(parent: CAAnimation) -> Result<CATransition, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CATransition").unwrap()) };
        if is_kind_of {
            Ok(CATransition(parent.0))
        } else {
            Err("This CAAnimation cannot be downcasted to CATransition")
        }
    }
}
impl INSObject for CATransition {}
impl PNSObject for CATransition {}
impl ICATransition for CATransition {}
pub trait ICATransition: Sized + std::ops::Deref {
    unsafe fn type_(&self) -> CATransitionType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, type)
    }
    unsafe fn setType_(&self, type_: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setType : type_)
    }
    unsafe fn subtype(&self) -> CATransitionSubtype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, subtype)
    }
    unsafe fn setSubtype_(&self, subtype: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSubtype : subtype)
    }
    unsafe fn startProgress(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, startProgress)
    }
    unsafe fn setStartProgress_(&self, startProgress: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setStartProgress : startProgress)
    }
    unsafe fn endProgress(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, endProgress)
    }
    unsafe fn setEndProgress_(&self, endProgress: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setEndProgress : endProgress)
    }
    unsafe fn filter(&self) -> id
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, filter)
    }
    unsafe fn setFilter_(&self, filter: id)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setFilter : filter)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CAAnimationGroup(pub id);
impl std::ops::Deref for CAAnimationGroup {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CAAnimationGroup {}
impl CAAnimationGroup {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CAAnimationGroup").unwrap(), alloc) })
    }
}
impl ICAAnimation for CAAnimationGroup {}
impl PNSSecureCoding for CAAnimationGroup {}
impl PNSCopying for CAAnimationGroup {}
impl PCAMediaTiming for CAAnimationGroup {}
impl PCAAction for CAAnimationGroup {}
impl std::convert::TryFrom<CAAnimation> for CAAnimationGroup {
    type Error = &'static str;
    fn try_from(parent: CAAnimation) -> Result<CAAnimationGroup, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CAAnimationGroup").unwrap()) };
        if is_kind_of {
            Ok(CAAnimationGroup(parent.0))
        } else {
            Err("This CAAnimation cannot be downcasted to CAAnimationGroup")
        }
    }
}
impl INSObject for CAAnimationGroup {}
impl PNSObject for CAAnimationGroup {}
impl ICAAnimationGroup for CAAnimationGroup {}
pub trait ICAAnimationGroup: Sized + std::ops::Deref {
    unsafe fn animations(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, animations)
    }
    unsafe fn setAnimations_(&self, animations: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAnimations : animations)
    }
}
pub type CAConstraintAttribute = ::std::os::raw::c_int;
impl CALayer_CAConstraintLayoutManager for CALayer {}
pub trait CALayer_CAConstraintLayoutManager: Sized + std::ops::Deref {
    unsafe fn addConstraint_(&self, c: CAConstraint)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, addConstraint : c)
    }
    unsafe fn constraints(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, constraints)
    }
    unsafe fn setConstraints_(&self, constraints: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setConstraints : constraints)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CAConstraintLayoutManager(pub id);
impl std::ops::Deref for CAConstraintLayoutManager {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CAConstraintLayoutManager {}
impl CAConstraintLayoutManager {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CAConstraintLayoutManager").unwrap(), alloc) })
    }
}
impl PCALayoutManager for CAConstraintLayoutManager {}
impl INSObject for CAConstraintLayoutManager {}
impl PNSObject for CAConstraintLayoutManager {}
impl std::convert::TryFrom<NSObject> for CAConstraintLayoutManager {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<CAConstraintLayoutManager, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CAConstraintLayoutManager").unwrap()) };
        if is_kind_of {
            Ok(CAConstraintLayoutManager(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to CAConstraintLayoutManager")
        }
    }
}
impl ICAConstraintLayoutManager for CAConstraintLayoutManager {}
pub trait ICAConstraintLayoutManager: Sized + std::ops::Deref {
    unsafe fn layoutManager() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CAConstraintLayoutManager").unwrap(), layoutManager)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CAConstraint(pub id);
impl std::ops::Deref for CAConstraint {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CAConstraint {}
impl CAConstraint {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CAConstraint").unwrap(), alloc) })
    }
}
impl PNSSecureCoding for CAConstraint {}
impl INSObject for CAConstraint {}
impl PNSObject for CAConstraint {}
impl std::convert::TryFrom<NSObject> for CAConstraint {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<CAConstraint, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CAConstraint").unwrap()) };
        if is_kind_of {
            Ok(CAConstraint(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to CAConstraint")
        }
    }
}
impl ICAConstraint for CAConstraint {}
pub trait ICAConstraint: Sized + std::ops::Deref {
    unsafe fn initWithAttribute_relativeTo_attribute_scale_offset_(
        &self,
        attr: CAConstraintAttribute,
        srcId: NSString,
        srcAttr: CAConstraintAttribute,
        m: CGFloat,
        c: CGFloat,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithAttribute : attr, relativeTo : srcId, attribute : srcAttr, scale : m, offset : c)
    }
    unsafe fn attribute(&self) -> CAConstraintAttribute
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, attribute)
    }
    unsafe fn sourceName(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, sourceName)
    }
    unsafe fn sourceAttribute(&self) -> CAConstraintAttribute
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, sourceAttribute)
    }
    unsafe fn scale(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, scale)
    }
    unsafe fn offset(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, offset)
    }
    unsafe fn constraintWithAttribute_relativeTo_attribute_scale_offset_(
        attr: CAConstraintAttribute,
        srcId: NSString,
        srcAttr: CAConstraintAttribute,
        m: CGFloat,
        c: CGFloat,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CAConstraint").unwrap(), constraintWithAttribute : attr, relativeTo : srcId, attribute : srcAttr, scale : m, offset : c)
    }
    unsafe fn constraintWithAttribute_relativeTo_attribute_offset_(
        attr: CAConstraintAttribute,
        srcId: NSString,
        srcAttr: CAConstraintAttribute,
        c: CGFloat,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CAConstraint").unwrap(), constraintWithAttribute : attr, relativeTo : srcId, attribute : srcAttr, offset : c)
    }
    unsafe fn constraintWithAttribute_relativeTo_attribute_(
        attr: CAConstraintAttribute,
        srcId: NSString,
        srcAttr: CAConstraintAttribute,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CAConstraint").unwrap(), constraintWithAttribute : attr, relativeTo : srcId, attribute : srcAttr)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CADisplayLink(pub id);
impl std::ops::Deref for CADisplayLink {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CADisplayLink {}
impl CADisplayLink {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CADisplayLink").unwrap(), alloc) })
    }
}
impl INSObject for CADisplayLink {}
impl PNSObject for CADisplayLink {}
impl std::convert::TryFrom<NSObject> for CADisplayLink {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<CADisplayLink, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CADisplayLink").unwrap()) };
        if is_kind_of {
            Ok(CADisplayLink(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to CADisplayLink")
        }
    }
}
impl ICADisplayLink for CADisplayLink {}
pub trait ICADisplayLink: Sized + std::ops::Deref {
    unsafe fn addToRunLoop_forMode_(&self, runloop: NSRunLoop, mode: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, addToRunLoop : runloop, forMode : mode)
    }
    unsafe fn removeFromRunLoop_forMode_(&self, runloop: NSRunLoop, mode: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, removeFromRunLoop : runloop, forMode : mode)
    }
    unsafe fn invalidate(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, invalidate)
    }
    unsafe fn timestamp(&self) -> CFTimeInterval
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, timestamp)
    }
    unsafe fn duration(&self) -> CFTimeInterval
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, duration)
    }
    unsafe fn targetTimestamp(&self) -> CFTimeInterval
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, targetTimestamp)
    }
    unsafe fn isPaused(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isPaused)
    }
    unsafe fn setPaused_(&self, paused: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPaused : paused)
    }
    unsafe fn frameInterval(&self) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, frameInterval)
    }
    unsafe fn setFrameInterval_(&self, frameInterval: NSInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setFrameInterval : frameInterval)
    }
    unsafe fn preferredFramesPerSecond(&self) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, preferredFramesPerSecond)
    }
    unsafe fn setPreferredFramesPerSecond_(&self, preferredFramesPerSecond: NSInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPreferredFramesPerSecond : preferredFramesPerSecond)
    }
    unsafe fn preferredFrameRateRange(&self) -> CAFrameRateRange
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, preferredFrameRateRange)
    }
    unsafe fn setPreferredFrameRateRange_(&self, preferredFrameRateRange: CAFrameRateRange)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPreferredFrameRateRange : preferredFrameRateRange)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CAEDRMetadata(pub id);
impl std::ops::Deref for CAEDRMetadata {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CAEDRMetadata {}
impl CAEDRMetadata {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CAEDRMetadata").unwrap(), alloc) })
    }
}
impl PNSCopying for CAEDRMetadata {}
impl PNSSecureCoding for CAEDRMetadata {}
impl INSObject for CAEDRMetadata {}
impl PNSObject for CAEDRMetadata {}
impl std::convert::TryFrom<NSObject> for CAEDRMetadata {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<CAEDRMetadata, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CAEDRMetadata").unwrap()) };
        if is_kind_of {
            Ok(CAEDRMetadata(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to CAEDRMetadata")
        }
    }
}
impl ICAEDRMetadata for CAEDRMetadata {}
pub trait ICAEDRMetadata: Sized + std::ops::Deref {
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
        msg_send!(&*objc2::runtime::AnyClass::get(c"CAEDRMetadata").unwrap(), new)
    }
    unsafe fn HDR10MetadataWithDisplayInfo_contentInfo_opticalOutputScale_(
        displayData: NSData,
        contentData: NSData,
        scale: f32,
    ) -> CAEDRMetadata
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CAEDRMetadata").unwrap(), HDR10MetadataWithDisplayInfo : displayData, contentInfo : contentData, opticalOutputScale : scale)
    }
    unsafe fn HDR10MetadataWithMinLuminance_maxLuminance_opticalOutputScale_(
        minNits: f32,
        maxNits: f32,
        scale: f32,
    ) -> CAEDRMetadata
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CAEDRMetadata").unwrap(), HDR10MetadataWithMinLuminance : minNits, maxLuminance : maxNits, opticalOutputScale : scale)
    }
    unsafe fn HLGMetadataWithAmbientViewingEnvironment_(data: NSData) -> CAEDRMetadata
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CAEDRMetadata").unwrap(), HLGMetadataWithAmbientViewingEnvironment : data)
    }
    unsafe fn HLGMetadata() -> CAEDRMetadata
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CAEDRMetadata").unwrap(), HLGMetadata)
    }
    unsafe fn isAvailable() -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CAEDRMetadata").unwrap(), isAvailable)
    }
}
pub trait PCAMetalDrawable: Sized + std::ops::Deref {
    unsafe fn texture(&self) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, texture)
    }
    unsafe fn layer(&self) -> CAMetalLayer
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, layer)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CAMetalLayer(pub id);
impl std::ops::Deref for CAMetalLayer {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CAMetalLayer {}
impl CAMetalLayer {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CAMetalLayer").unwrap(), alloc) })
    }
}
impl ICALayer for CAMetalLayer {}
impl PNSSecureCoding for CAMetalLayer {}
impl PCAMediaTiming for CAMetalLayer {}
impl From<CAMetalLayer> for CALayer {
    fn from(child: CAMetalLayer) -> CALayer {
        CALayer(child.0)
    }
}
impl std::convert::TryFrom<CALayer> for CAMetalLayer {
    type Error = &'static str;
    fn try_from(parent: CALayer) -> Result<CAMetalLayer, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CAMetalLayer").unwrap()) };
        if is_kind_of {
            Ok(CAMetalLayer(parent.0))
        } else {
            Err("This CALayer cannot be downcasted to CAMetalLayer")
        }
    }
}
impl INSObject for CAMetalLayer {}
impl PNSObject for CAMetalLayer {}
impl ICAMetalLayer for CAMetalLayer {}
pub trait ICAMetalLayer: Sized + std::ops::Deref {
    unsafe fn nextDrawable(&self) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, nextDrawable)
    }
    unsafe fn device(&self) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, device)
    }
    unsafe fn setDevice_(&self, device: *mut u64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDevice : device)
    }
    unsafe fn preferredDevice(&self) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, preferredDevice)
    }
    unsafe fn pixelFormat(&self) -> MTLPixelFormat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, pixelFormat)
    }
    unsafe fn setPixelFormat_(&self, pixelFormat: MTLPixelFormat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPixelFormat : pixelFormat)
    }
    unsafe fn framebufferOnly(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, framebufferOnly)
    }
    unsafe fn setFramebufferOnly_(&self, framebufferOnly: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setFramebufferOnly : framebufferOnly)
    }
    unsafe fn drawableSize(&self) -> CGSize
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, drawableSize)
    }
    unsafe fn setDrawableSize_(&self, drawableSize: CGSize)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDrawableSize : drawableSize)
    }
    unsafe fn maximumDrawableCount(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, maximumDrawableCount)
    }
    unsafe fn setMaximumDrawableCount_(&self, maximumDrawableCount: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMaximumDrawableCount : maximumDrawableCount)
    }
    unsafe fn presentsWithTransaction(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, presentsWithTransaction)
    }
    unsafe fn setPresentsWithTransaction_(&self, presentsWithTransaction: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPresentsWithTransaction : presentsWithTransaction)
    }
    unsafe fn colorspace(&self) -> CGColorSpaceRef
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, colorspace)
    }
    unsafe fn setColorspace_(&self, colorspace: CGColorSpaceRef)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setColorspace : colorspace)
    }
    unsafe fn wantsExtendedDynamicRangeContent(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, wantsExtendedDynamicRangeContent)
    }
    unsafe fn setWantsExtendedDynamicRangeContent_(&self, wantsExtendedDynamicRangeContent: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setWantsExtendedDynamicRangeContent : wantsExtendedDynamicRangeContent)
    }
    unsafe fn EDRMetadata(&self) -> CAEDRMetadata
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, EDRMetadata)
    }
    unsafe fn setEDRMetadata_(&self, EDRMetadata: CAEDRMetadata)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setEDRMetadata : EDRMetadata)
    }
    unsafe fn displaySyncEnabled(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, displaySyncEnabled)
    }
    unsafe fn setDisplaySyncEnabled_(&self, displaySyncEnabled: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDisplaySyncEnabled : displaySyncEnabled)
    }
    unsafe fn allowsNextDrawableTimeout(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, allowsNextDrawableTimeout)
    }
    unsafe fn setAllowsNextDrawableTimeout_(&self, allowsNextDrawableTimeout: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAllowsNextDrawableTimeout : allowsNextDrawableTimeout)
    }
    unsafe fn developerHUDProperties(&self) -> NSDictionary
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, developerHUDProperties)
    }
    unsafe fn setDeveloperHUDProperties_(&self, developerHUDProperties: NSDictionary)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDeveloperHUDProperties : developerHUDProperties)
    }
    unsafe fn residencySet(&self) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, residencySet)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CAMetalDisplayLinkUpdate(pub id);
impl std::ops::Deref for CAMetalDisplayLinkUpdate {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CAMetalDisplayLinkUpdate {}
impl CAMetalDisplayLinkUpdate {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CAMetalDisplayLinkUpdate").unwrap(), alloc) })
    }
}
impl INSObject for CAMetalDisplayLinkUpdate {}
impl PNSObject for CAMetalDisplayLinkUpdate {}
impl std::convert::TryFrom<NSObject> for CAMetalDisplayLinkUpdate {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<CAMetalDisplayLinkUpdate, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CAMetalDisplayLinkUpdate").unwrap()) };
        if is_kind_of {
            Ok(CAMetalDisplayLinkUpdate(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to CAMetalDisplayLinkUpdate")
        }
    }
}
impl ICAMetalDisplayLinkUpdate for CAMetalDisplayLinkUpdate {}
pub trait ICAMetalDisplayLinkUpdate: Sized + std::ops::Deref {
    unsafe fn drawable(&self) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, drawable)
    }
    unsafe fn targetTimestamp(&self) -> CFTimeInterval
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, targetTimestamp)
    }
    unsafe fn targetPresentationTimestamp(&self) -> CFTimeInterval
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, targetPresentationTimestamp)
    }
}
pub trait PCAMetalDisplayLinkDelegate: Sized + std::ops::Deref {
    unsafe fn metalDisplayLink_needsUpdate_(
        &self,
        link: CAMetalDisplayLink,
        update: CAMetalDisplayLinkUpdate,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, metalDisplayLink : link, needsUpdate : update)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CAMetalDisplayLink(pub id);
impl std::ops::Deref for CAMetalDisplayLink {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CAMetalDisplayLink {}
impl CAMetalDisplayLink {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CAMetalDisplayLink").unwrap(), alloc) })
    }
}
impl INSObject for CAMetalDisplayLink {}
impl PNSObject for CAMetalDisplayLink {}
impl std::convert::TryFrom<NSObject> for CAMetalDisplayLink {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<CAMetalDisplayLink, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CAMetalDisplayLink").unwrap()) };
        if is_kind_of {
            Ok(CAMetalDisplayLink(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to CAMetalDisplayLink")
        }
    }
}
impl ICAMetalDisplayLink for CAMetalDisplayLink {}
pub trait ICAMetalDisplayLink: Sized + std::ops::Deref {
    unsafe fn initWithMetalLayer_(&self, layer: CAMetalLayer) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithMetalLayer : layer)
    }
    unsafe fn addToRunLoop_forMode_(&self, runloop: NSRunLoop, mode: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, addToRunLoop : runloop, forMode : mode)
    }
    unsafe fn removeFromRunLoop_forMode_(&self, runloop: NSRunLoop, mode: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, removeFromRunLoop : runloop, forMode : mode)
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
    unsafe fn setDelegate_(&self, delegate: *mut u64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDelegate : delegate)
    }
    unsafe fn preferredFrameLatency(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, preferredFrameLatency)
    }
    unsafe fn setPreferredFrameLatency_(&self, preferredFrameLatency: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPreferredFrameLatency : preferredFrameLatency)
    }
    unsafe fn preferredFrameRateRange(&self) -> CAFrameRateRange
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, preferredFrameRateRange)
    }
    unsafe fn setPreferredFrameRateRange_(&self, preferredFrameRateRange: CAFrameRateRange)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPreferredFrameRateRange : preferredFrameRateRange)
    }
    unsafe fn isPaused(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isPaused)
    }
    unsafe fn setPaused_(&self, paused: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPaused : paused)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CAEmitterCell(pub id);
impl std::ops::Deref for CAEmitterCell {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CAEmitterCell {}
impl CAEmitterCell {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CAEmitterCell").unwrap(), alloc) })
    }
}
impl PNSSecureCoding for CAEmitterCell {}
impl PCAMediaTiming for CAEmitterCell {}
impl INSObject for CAEmitterCell {}
impl PNSObject for CAEmitterCell {}
impl std::convert::TryFrom<NSObject> for CAEmitterCell {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<CAEmitterCell, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CAEmitterCell").unwrap()) };
        if is_kind_of {
            Ok(CAEmitterCell(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to CAEmitterCell")
        }
    }
}
impl ICAEmitterCell for CAEmitterCell {}
pub trait ICAEmitterCell: Sized + std::ops::Deref {
    unsafe fn shouldArchiveValueForKey_(&self, key: NSString) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, shouldArchiveValueForKey : key)
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
    unsafe fn isEnabled(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isEnabled)
    }
    unsafe fn setEnabled_(&self, enabled: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setEnabled : enabled)
    }
    unsafe fn birthRate(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, birthRate)
    }
    unsafe fn setBirthRate_(&self, birthRate: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setBirthRate : birthRate)
    }
    unsafe fn lifetime(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, lifetime)
    }
    unsafe fn setLifetime_(&self, lifetime: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setLifetime : lifetime)
    }
    unsafe fn lifetimeRange(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, lifetimeRange)
    }
    unsafe fn setLifetimeRange_(&self, lifetimeRange: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setLifetimeRange : lifetimeRange)
    }
    unsafe fn emissionLatitude(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, emissionLatitude)
    }
    unsafe fn setEmissionLatitude_(&self, emissionLatitude: CGFloat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setEmissionLatitude : emissionLatitude)
    }
    unsafe fn emissionLongitude(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, emissionLongitude)
    }
    unsafe fn setEmissionLongitude_(&self, emissionLongitude: CGFloat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setEmissionLongitude : emissionLongitude)
    }
    unsafe fn emissionRange(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, emissionRange)
    }
    unsafe fn setEmissionRange_(&self, emissionRange: CGFloat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setEmissionRange : emissionRange)
    }
    unsafe fn velocity(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, velocity)
    }
    unsafe fn setVelocity_(&self, velocity: CGFloat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setVelocity : velocity)
    }
    unsafe fn velocityRange(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, velocityRange)
    }
    unsafe fn setVelocityRange_(&self, velocityRange: CGFloat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setVelocityRange : velocityRange)
    }
    unsafe fn xAcceleration(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, xAcceleration)
    }
    unsafe fn setXAcceleration_(&self, xAcceleration: CGFloat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setXAcceleration : xAcceleration)
    }
    unsafe fn yAcceleration(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, yAcceleration)
    }
    unsafe fn setYAcceleration_(&self, yAcceleration: CGFloat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setYAcceleration : yAcceleration)
    }
    unsafe fn zAcceleration(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, zAcceleration)
    }
    unsafe fn setZAcceleration_(&self, zAcceleration: CGFloat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setZAcceleration : zAcceleration)
    }
    unsafe fn scale(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, scale)
    }
    unsafe fn setScale_(&self, scale: CGFloat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setScale : scale)
    }
    unsafe fn scaleRange(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, scaleRange)
    }
    unsafe fn setScaleRange_(&self, scaleRange: CGFloat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setScaleRange : scaleRange)
    }
    unsafe fn scaleSpeed(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, scaleSpeed)
    }
    unsafe fn setScaleSpeed_(&self, scaleSpeed: CGFloat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setScaleSpeed : scaleSpeed)
    }
    unsafe fn spin(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, spin)
    }
    unsafe fn setSpin_(&self, spin: CGFloat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSpin : spin)
    }
    unsafe fn spinRange(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, spinRange)
    }
    unsafe fn setSpinRange_(&self, spinRange: CGFloat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSpinRange : spinRange)
    }
    unsafe fn color(&self) -> CGColorRef
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, color)
    }
    unsafe fn setColor_(&self, color: CGColorRef)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setColor : color)
    }
    unsafe fn redRange(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, redRange)
    }
    unsafe fn setRedRange_(&self, redRange: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRedRange : redRange)
    }
    unsafe fn greenRange(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, greenRange)
    }
    unsafe fn setGreenRange_(&self, greenRange: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setGreenRange : greenRange)
    }
    unsafe fn blueRange(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, blueRange)
    }
    unsafe fn setBlueRange_(&self, blueRange: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setBlueRange : blueRange)
    }
    unsafe fn alphaRange(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, alphaRange)
    }
    unsafe fn setAlphaRange_(&self, alphaRange: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAlphaRange : alphaRange)
    }
    unsafe fn redSpeed(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, redSpeed)
    }
    unsafe fn setRedSpeed_(&self, redSpeed: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRedSpeed : redSpeed)
    }
    unsafe fn greenSpeed(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, greenSpeed)
    }
    unsafe fn setGreenSpeed_(&self, greenSpeed: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setGreenSpeed : greenSpeed)
    }
    unsafe fn blueSpeed(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, blueSpeed)
    }
    unsafe fn setBlueSpeed_(&self, blueSpeed: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setBlueSpeed : blueSpeed)
    }
    unsafe fn alphaSpeed(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, alphaSpeed)
    }
    unsafe fn setAlphaSpeed_(&self, alphaSpeed: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAlphaSpeed : alphaSpeed)
    }
    unsafe fn contents(&self) -> id
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, contents)
    }
    unsafe fn setContents_(&self, contents: id)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setContents : contents)
    }
    unsafe fn contentsRect(&self) -> CGRect
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, contentsRect)
    }
    unsafe fn setContentsRect_(&self, contentsRect: CGRect)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setContentsRect : contentsRect)
    }
    unsafe fn contentsScale(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, contentsScale)
    }
    unsafe fn setContentsScale_(&self, contentsScale: CGFloat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setContentsScale : contentsScale)
    }
    unsafe fn minificationFilter(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, minificationFilter)
    }
    unsafe fn setMinificationFilter_(&self, minificationFilter: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMinificationFilter : minificationFilter)
    }
    unsafe fn magnificationFilter(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, magnificationFilter)
    }
    unsafe fn setMagnificationFilter_(&self, magnificationFilter: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMagnificationFilter : magnificationFilter)
    }
    unsafe fn minificationFilterBias(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, minificationFilterBias)
    }
    unsafe fn setMinificationFilterBias_(&self, minificationFilterBias: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMinificationFilterBias : minificationFilterBias)
    }
    unsafe fn emitterCells(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, emitterCells)
    }
    unsafe fn setEmitterCells_(&self, emitterCells: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setEmitterCells : emitterCells)
    }
    unsafe fn style(&self) -> NSDictionary
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, style)
    }
    unsafe fn setStyle_(&self, style: NSDictionary)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setStyle : style)
    }
    unsafe fn emitterCell() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CAEmitterCell").unwrap(), emitterCell)
    }
    unsafe fn defaultValueForKey_(key: NSString) -> id
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CAEmitterCell").unwrap(), defaultValueForKey : key)
    }
}
pub type CAEmitterLayerEmitterShape = NSString;
pub type CAEmitterLayerEmitterMode = NSString;
pub type CAEmitterLayerRenderMode = NSString;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CAEmitterLayer(pub id);
impl std::ops::Deref for CAEmitterLayer {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CAEmitterLayer {}
impl CAEmitterLayer {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CAEmitterLayer").unwrap(), alloc) })
    }
}
impl ICALayer for CAEmitterLayer {}
impl PNSSecureCoding for CAEmitterLayer {}
impl PCAMediaTiming for CAEmitterLayer {}
impl std::convert::TryFrom<CALayer> for CAEmitterLayer {
    type Error = &'static str;
    fn try_from(parent: CALayer) -> Result<CAEmitterLayer, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CAEmitterLayer").unwrap()) };
        if is_kind_of {
            Ok(CAEmitterLayer(parent.0))
        } else {
            Err("This CALayer cannot be downcasted to CAEmitterLayer")
        }
    }
}
impl INSObject for CAEmitterLayer {}
impl PNSObject for CAEmitterLayer {}
impl ICAEmitterLayer for CAEmitterLayer {}
pub trait ICAEmitterLayer: Sized + std::ops::Deref {
    unsafe fn emitterCells(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, emitterCells)
    }
    unsafe fn setEmitterCells_(&self, emitterCells: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setEmitterCells : emitterCells)
    }
    unsafe fn birthRate(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, birthRate)
    }
    unsafe fn setBirthRate_(&self, birthRate: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setBirthRate : birthRate)
    }
    unsafe fn lifetime(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, lifetime)
    }
    unsafe fn setLifetime_(&self, lifetime: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setLifetime : lifetime)
    }
    unsafe fn emitterPosition(&self) -> CGPoint
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, emitterPosition)
    }
    unsafe fn setEmitterPosition_(&self, emitterPosition: CGPoint)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setEmitterPosition : emitterPosition)
    }
    unsafe fn emitterZPosition(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, emitterZPosition)
    }
    unsafe fn setEmitterZPosition_(&self, emitterZPosition: CGFloat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setEmitterZPosition : emitterZPosition)
    }
    unsafe fn emitterSize(&self) -> CGSize
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, emitterSize)
    }
    unsafe fn setEmitterSize_(&self, emitterSize: CGSize)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setEmitterSize : emitterSize)
    }
    unsafe fn emitterDepth(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, emitterDepth)
    }
    unsafe fn setEmitterDepth_(&self, emitterDepth: CGFloat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setEmitterDepth : emitterDepth)
    }
    unsafe fn emitterShape(&self) -> CAEmitterLayerEmitterShape
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, emitterShape)
    }
    unsafe fn setEmitterShape_(&self, emitterShape: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setEmitterShape : emitterShape)
    }
    unsafe fn emitterMode(&self) -> CAEmitterLayerEmitterMode
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, emitterMode)
    }
    unsafe fn setEmitterMode_(&self, emitterMode: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setEmitterMode : emitterMode)
    }
    unsafe fn renderMode(&self) -> CAEmitterLayerRenderMode
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, renderMode)
    }
    unsafe fn setRenderMode_(&self, renderMode: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRenderMode : renderMode)
    }
    unsafe fn preservesDepth(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, preservesDepth)
    }
    unsafe fn setPreservesDepth_(&self, preservesDepth: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPreservesDepth : preservesDepth)
    }
    unsafe fn velocity(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, velocity)
    }
    unsafe fn setVelocity_(&self, velocity: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setVelocity : velocity)
    }
    unsafe fn scale(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, scale)
    }
    unsafe fn setScale_(&self, scale: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setScale : scale)
    }
    unsafe fn spin(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, spin)
    }
    unsafe fn setSpin_(&self, spin: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSpin : spin)
    }
    unsafe fn seed(&self) -> ::std::os::raw::c_uint
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, seed)
    }
    unsafe fn setSeed_(&self, seed: ::std::os::raw::c_uint)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSeed : seed)
    }
}
pub type CAMediaTimingFunctionName = NSString;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CAMediaTimingFunction(pub id);
impl std::ops::Deref for CAMediaTimingFunction {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CAMediaTimingFunction {}
impl CAMediaTimingFunction {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CAMediaTimingFunction").unwrap(), alloc) })
    }
}
impl PNSSecureCoding for CAMediaTimingFunction {}
impl INSObject for CAMediaTimingFunction {}
impl PNSObject for CAMediaTimingFunction {}
impl std::convert::TryFrom<NSObject> for CAMediaTimingFunction {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<CAMediaTimingFunction, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CAMediaTimingFunction").unwrap()) };
        if is_kind_of {
            Ok(CAMediaTimingFunction(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to CAMediaTimingFunction")
        }
    }
}
impl ICAMediaTimingFunction for CAMediaTimingFunction {}
pub trait ICAMediaTimingFunction: Sized + std::ops::Deref {
    unsafe fn initWithControlPoints____(
        &self,
        c1x: f32,
        c1y: f32,
        c2x: f32,
        c2y: f32,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithControlPoints : c1x, c1y : c1y, c2x : c2x, c2y : c2y)
    }
    unsafe fn getControlPointAtIndex_values_(&self, idx: usize, ptr: *mut f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, getControlPointAtIndex : idx, values : ptr)
    }
    unsafe fn functionWithName_(name: NSString) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CAMediaTimingFunction").unwrap(), functionWithName : name)
    }
    unsafe fn functionWithControlPoints____(c1x: f32, c1y: f32, c2x: f32, c2y: f32) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CAMediaTimingFunction").unwrap(), functionWithControlPoints : c1x, c1y : c1y, c2x : c2x, c2y : c2y)
    }
}
pub type CAGradientLayerType = NSString;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CAGradientLayer(pub id);
impl std::ops::Deref for CAGradientLayer {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CAGradientLayer {}
impl CAGradientLayer {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CAGradientLayer").unwrap(), alloc) })
    }
}
impl ICALayer for CAGradientLayer {}
impl PNSSecureCoding for CAGradientLayer {}
impl PCAMediaTiming for CAGradientLayer {}
impl std::convert::TryFrom<CALayer> for CAGradientLayer {
    type Error = &'static str;
    fn try_from(parent: CALayer) -> Result<CAGradientLayer, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CAGradientLayer").unwrap()) };
        if is_kind_of {
            Ok(CAGradientLayer(parent.0))
        } else {
            Err("This CALayer cannot be downcasted to CAGradientLayer")
        }
    }
}
impl INSObject for CAGradientLayer {}
impl PNSObject for CAGradientLayer {}
impl ICAGradientLayer for CAGradientLayer {}
pub trait ICAGradientLayer: Sized + std::ops::Deref {
    unsafe fn colors(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, colors)
    }
    unsafe fn setColors_(&self, colors: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setColors : colors)
    }
    unsafe fn locations(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, locations)
    }
    unsafe fn setLocations_(&self, locations: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setLocations : locations)
    }
    unsafe fn startPoint(&self) -> CGPoint
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, startPoint)
    }
    unsafe fn setStartPoint_(&self, startPoint: CGPoint)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setStartPoint : startPoint)
    }
    unsafe fn endPoint(&self) -> CGPoint
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, endPoint)
    }
    unsafe fn setEndPoint_(&self, endPoint: CGPoint)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setEndPoint : endPoint)
    }
    unsafe fn type_(&self) -> CAGradientLayerType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, type)
    }
    unsafe fn setType_(&self, type_: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setType : type_)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CAOpenGLLayer(pub id);
impl std::ops::Deref for CAOpenGLLayer {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CAOpenGLLayer {}
impl CAOpenGLLayer {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CAOpenGLLayer").unwrap(), alloc) })
    }
}
impl ICALayer for CAOpenGLLayer {}
impl PNSSecureCoding for CAOpenGLLayer {}
impl PCAMediaTiming for CAOpenGLLayer {}
impl std::convert::TryFrom<CALayer> for CAOpenGLLayer {
    type Error = &'static str;
    fn try_from(parent: CALayer) -> Result<CAOpenGLLayer, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CAOpenGLLayer").unwrap()) };
        if is_kind_of {
            Ok(CAOpenGLLayer(parent.0))
        } else {
            Err("This CALayer cannot be downcasted to CAOpenGLLayer")
        }
    }
}
impl INSObject for CAOpenGLLayer {}
impl PNSObject for CAOpenGLLayer {}
impl ICAOpenGLLayer for CAOpenGLLayer {}
pub trait ICAOpenGLLayer: Sized + std::ops::Deref {
    unsafe fn canDrawInCGLContext_pixelFormat_forLayerTime_displayTime_(
        &self,
        ctx: CGLContextObj,
        pf: CGLPixelFormatObj,
        t: CFTimeInterval,
        ts: *const CVTimeStamp,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, canDrawInCGLContext : ctx, pixelFormat : pf, forLayerTime : t, displayTime : ts)
    }
    unsafe fn drawInCGLContext_pixelFormat_forLayerTime_displayTime_(
        &self,
        ctx: CGLContextObj,
        pf: CGLPixelFormatObj,
        t: CFTimeInterval,
        ts: *const CVTimeStamp,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, drawInCGLContext : ctx, pixelFormat : pf, forLayerTime : t, displayTime : ts)
    }
    unsafe fn copyCGLPixelFormatForDisplayMask_(&self, mask: u32) -> CGLPixelFormatObj
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, copyCGLPixelFormatForDisplayMask : mask)
    }
    unsafe fn releaseCGLPixelFormat_(&self, pf: CGLPixelFormatObj)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, releaseCGLPixelFormat : pf)
    }
    unsafe fn copyCGLContextForPixelFormat_(&self, pf: CGLPixelFormatObj) -> CGLContextObj
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, copyCGLContextForPixelFormat : pf)
    }
    unsafe fn releaseCGLContext_(&self, ctx: CGLContextObj)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, releaseCGLContext : ctx)
    }
    unsafe fn isAsynchronous(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isAsynchronous)
    }
    unsafe fn setAsynchronous_(&self, asynchronous: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAsynchronous : asynchronous)
    }
    unsafe fn colorspace(&self) -> CGColorSpaceRef
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, colorspace)
    }
    unsafe fn setColorspace_(&self, colorspace: CGColorSpaceRef)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setColorspace : colorspace)
    }
    unsafe fn wantsExtendedDynamicRangeContent(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, wantsExtendedDynamicRangeContent)
    }
    unsafe fn setWantsExtendedDynamicRangeContent_(&self, wantsExtendedDynamicRangeContent: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setWantsExtendedDynamicRangeContent : wantsExtendedDynamicRangeContent)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CARemoteLayerClient(pub id);
impl std::ops::Deref for CARemoteLayerClient {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CARemoteLayerClient {}
impl CARemoteLayerClient {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CARemoteLayerClient").unwrap(), alloc) })
    }
}
impl INSObject for CARemoteLayerClient {}
impl PNSObject for CARemoteLayerClient {}
impl std::convert::TryFrom<NSObject> for CARemoteLayerClient {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<CARemoteLayerClient, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CARemoteLayerClient").unwrap()) };
        if is_kind_of {
            Ok(CARemoteLayerClient(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to CARemoteLayerClient")
        }
    }
}
impl ICARemoteLayerClient for CARemoteLayerClient {}
pub trait ICARemoteLayerClient: Sized + std::ops::Deref {
    unsafe fn initWithServerPort_(&self, port: mach_port_t) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithServerPort : port)
    }
    unsafe fn invalidate(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, invalidate)
    }
    unsafe fn clientId(&self) -> u32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, clientId)
    }
    unsafe fn layer(&self) -> CALayer
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, layer)
    }
    unsafe fn setLayer_(&self, layer: CALayer)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setLayer : layer)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CARemoteLayerServer(pub id);
impl std::ops::Deref for CARemoteLayerServer {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CARemoteLayerServer {}
impl CARemoteLayerServer {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CARemoteLayerServer").unwrap(), alloc) })
    }
}
impl INSObject for CARemoteLayerServer {}
impl PNSObject for CARemoteLayerServer {}
impl std::convert::TryFrom<NSObject> for CARemoteLayerServer {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<CARemoteLayerServer, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CARemoteLayerServer").unwrap()) };
        if is_kind_of {
            Ok(CARemoteLayerServer(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to CARemoteLayerServer")
        }
    }
}
impl ICARemoteLayerServer for CARemoteLayerServer {}
pub trait ICARemoteLayerServer: Sized + std::ops::Deref {
    unsafe fn serverPort(&self) -> mach_port_t
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, serverPort)
    }
    unsafe fn sharedServer() -> CARemoteLayerServer
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CARemoteLayerServer").unwrap(), sharedServer)
    }
}
impl CALayer_CARemoteLayerServer for CALayer {}
pub trait CALayer_CARemoteLayerServer: Sized + std::ops::Deref {
    unsafe fn layerWithRemoteClientId_(client_id: u32) -> CALayer
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CALayer").unwrap(), layerWithRemoteClientId : client_id)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CARenderer(pub id);
impl std::ops::Deref for CARenderer {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CARenderer {}
impl CARenderer {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CARenderer").unwrap(), alloc) })
    }
}
impl INSObject for CARenderer {}
impl PNSObject for CARenderer {}
impl std::convert::TryFrom<NSObject> for CARenderer {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<CARenderer, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CARenderer").unwrap()) };
        if is_kind_of {
            Ok(CARenderer(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to CARenderer")
        }
    }
}
impl ICARenderer for CARenderer {}
pub trait ICARenderer: Sized + std::ops::Deref {
    unsafe fn beginFrameAtTime_timeStamp_(&self, t: CFTimeInterval, ts: *mut CVTimeStamp)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, beginFrameAtTime : t, timeStamp : ts)
    }
    unsafe fn updateBounds(&self) -> CGRect
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, updateBounds)
    }
    unsafe fn addUpdateRect_(&self, r: CGRect)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, addUpdateRect : r)
    }
    unsafe fn render(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, render)
    }
    unsafe fn nextFrameTime(&self) -> CFTimeInterval
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, nextFrameTime)
    }
    unsafe fn endFrame(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, endFrame)
    }
    unsafe fn setDestination_(&self, tex: *mut u64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDestination : tex)
    }
    unsafe fn layer(&self) -> CALayer
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, layer)
    }
    unsafe fn setLayer_(&self, layer: CALayer)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setLayer : layer)
    }
    unsafe fn bounds(&self) -> CGRect
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, bounds)
    }
    unsafe fn setBounds_(&self, bounds: CGRect)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setBounds : bounds)
    }
    unsafe fn rendererWithCGLContext_options_(
        ctx: *mut ::std::os::raw::c_void,
        dict: NSDictionary,
    ) -> CARenderer
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CARenderer").unwrap(), rendererWithCGLContext : ctx, options : dict)
    }
    unsafe fn rendererWithMTLTexture_options_(tex: *mut u64, dict: NSDictionary) -> CARenderer
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CARenderer").unwrap(), rendererWithMTLTexture : tex, options : dict)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CAReplicatorLayer(pub id);
impl std::ops::Deref for CAReplicatorLayer {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CAReplicatorLayer {}
impl CAReplicatorLayer {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CAReplicatorLayer").unwrap(), alloc) })
    }
}
impl ICALayer for CAReplicatorLayer {}
impl PNSSecureCoding for CAReplicatorLayer {}
impl PCAMediaTiming for CAReplicatorLayer {}
impl std::convert::TryFrom<CALayer> for CAReplicatorLayer {
    type Error = &'static str;
    fn try_from(parent: CALayer) -> Result<CAReplicatorLayer, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CAReplicatorLayer").unwrap()) };
        if is_kind_of {
            Ok(CAReplicatorLayer(parent.0))
        } else {
            Err("This CALayer cannot be downcasted to CAReplicatorLayer")
        }
    }
}
impl INSObject for CAReplicatorLayer {}
impl PNSObject for CAReplicatorLayer {}
impl ICAReplicatorLayer for CAReplicatorLayer {}
pub trait ICAReplicatorLayer: Sized + std::ops::Deref {
    unsafe fn instanceCount(&self) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, instanceCount)
    }
    unsafe fn setInstanceCount_(&self, instanceCount: NSInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInstanceCount : instanceCount)
    }
    unsafe fn preservesDepth(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, preservesDepth)
    }
    unsafe fn setPreservesDepth_(&self, preservesDepth: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPreservesDepth : preservesDepth)
    }
    unsafe fn instanceDelay(&self) -> CFTimeInterval
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, instanceDelay)
    }
    unsafe fn setInstanceDelay_(&self, instanceDelay: CFTimeInterval)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInstanceDelay : instanceDelay)
    }
    unsafe fn instanceTransform(&self) -> CATransform3D
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, instanceTransform)
    }
    unsafe fn setInstanceTransform_(&self, instanceTransform: CATransform3D)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInstanceTransform : instanceTransform)
    }
    unsafe fn instanceColor(&self) -> CGColorRef
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, instanceColor)
    }
    unsafe fn setInstanceColor_(&self, instanceColor: CGColorRef)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInstanceColor : instanceColor)
    }
    unsafe fn instanceRedOffset(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, instanceRedOffset)
    }
    unsafe fn setInstanceRedOffset_(&self, instanceRedOffset: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInstanceRedOffset : instanceRedOffset)
    }
    unsafe fn instanceGreenOffset(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, instanceGreenOffset)
    }
    unsafe fn setInstanceGreenOffset_(&self, instanceGreenOffset: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInstanceGreenOffset : instanceGreenOffset)
    }
    unsafe fn instanceBlueOffset(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, instanceBlueOffset)
    }
    unsafe fn setInstanceBlueOffset_(&self, instanceBlueOffset: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInstanceBlueOffset : instanceBlueOffset)
    }
    unsafe fn instanceAlphaOffset(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, instanceAlphaOffset)
    }
    unsafe fn setInstanceAlphaOffset_(&self, instanceAlphaOffset: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInstanceAlphaOffset : instanceAlphaOffset)
    }
}
pub type CAScrollLayerScrollMode = NSString;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CAScrollLayer(pub id);
impl std::ops::Deref for CAScrollLayer {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CAScrollLayer {}
impl CAScrollLayer {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CAScrollLayer").unwrap(), alloc) })
    }
}
impl ICALayer for CAScrollLayer {}
impl PNSSecureCoding for CAScrollLayer {}
impl PCAMediaTiming for CAScrollLayer {}
impl std::convert::TryFrom<CALayer> for CAScrollLayer {
    type Error = &'static str;
    fn try_from(parent: CALayer) -> Result<CAScrollLayer, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CAScrollLayer").unwrap()) };
        if is_kind_of {
            Ok(CAScrollLayer(parent.0))
        } else {
            Err("This CALayer cannot be downcasted to CAScrollLayer")
        }
    }
}
impl INSObject for CAScrollLayer {}
impl PNSObject for CAScrollLayer {}
impl ICAScrollLayer for CAScrollLayer {}
pub trait ICAScrollLayer: Sized + std::ops::Deref {
    unsafe fn scrollToPoint_(&self, p: CGPoint)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, scrollToPoint : p)
    }
    unsafe fn scrollToRect_(&self, r: CGRect)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, scrollToRect : r)
    }
    unsafe fn scrollMode(&self) -> CAScrollLayerScrollMode
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, scrollMode)
    }
    unsafe fn setScrollMode_(&self, scrollMode: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setScrollMode : scrollMode)
    }
}
impl CALayer_CALayerScrolling for CALayer {}
pub trait CALayer_CALayerScrolling: Sized + std::ops::Deref {
    unsafe fn scrollPoint_(&self, p: CGPoint)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, scrollPoint : p)
    }
    unsafe fn scrollRectToVisible_(&self, r: CGRect)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, scrollRectToVisible : r)
    }
    unsafe fn visibleRect(&self) -> CGRect
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, visibleRect)
    }
}
pub type CAShapeLayerFillRule = NSString;
pub type CAShapeLayerLineJoin = NSString;
pub type CAShapeLayerLineCap = NSString;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CAShapeLayer(pub id);
impl std::ops::Deref for CAShapeLayer {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CAShapeLayer {}
impl CAShapeLayer {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CAShapeLayer").unwrap(), alloc) })
    }
}
impl ICALayer for CAShapeLayer {}
impl PNSSecureCoding for CAShapeLayer {}
impl PCAMediaTiming for CAShapeLayer {}
impl std::convert::TryFrom<CALayer> for CAShapeLayer {
    type Error = &'static str;
    fn try_from(parent: CALayer) -> Result<CAShapeLayer, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CAShapeLayer").unwrap()) };
        if is_kind_of {
            Ok(CAShapeLayer(parent.0))
        } else {
            Err("This CALayer cannot be downcasted to CAShapeLayer")
        }
    }
}
impl INSObject for CAShapeLayer {}
impl PNSObject for CAShapeLayer {}
impl ICAShapeLayer for CAShapeLayer {}
pub trait ICAShapeLayer: Sized + std::ops::Deref {
    unsafe fn path(&self) -> CGPathRef
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, path)
    }
    unsafe fn setPath_(&self, path: CGPathRef)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPath : path)
    }
    unsafe fn fillColor(&self) -> CGColorRef
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, fillColor)
    }
    unsafe fn setFillColor_(&self, fillColor: CGColorRef)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setFillColor : fillColor)
    }
    unsafe fn fillRule(&self) -> CAShapeLayerFillRule
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, fillRule)
    }
    unsafe fn setFillRule_(&self, fillRule: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setFillRule : fillRule)
    }
    unsafe fn strokeColor(&self) -> CGColorRef
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, strokeColor)
    }
    unsafe fn setStrokeColor_(&self, strokeColor: CGColorRef)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setStrokeColor : strokeColor)
    }
    unsafe fn strokeStart(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, strokeStart)
    }
    unsafe fn setStrokeStart_(&self, strokeStart: CGFloat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setStrokeStart : strokeStart)
    }
    unsafe fn strokeEnd(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, strokeEnd)
    }
    unsafe fn setStrokeEnd_(&self, strokeEnd: CGFloat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setStrokeEnd : strokeEnd)
    }
    unsafe fn lineWidth(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, lineWidth)
    }
    unsafe fn setLineWidth_(&self, lineWidth: CGFloat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setLineWidth : lineWidth)
    }
    unsafe fn miterLimit(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, miterLimit)
    }
    unsafe fn setMiterLimit_(&self, miterLimit: CGFloat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMiterLimit : miterLimit)
    }
    unsafe fn lineCap(&self) -> CAShapeLayerLineCap
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, lineCap)
    }
    unsafe fn setLineCap_(&self, lineCap: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setLineCap : lineCap)
    }
    unsafe fn lineJoin(&self) -> CAShapeLayerLineJoin
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, lineJoin)
    }
    unsafe fn setLineJoin_(&self, lineJoin: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setLineJoin : lineJoin)
    }
    unsafe fn lineDashPhase(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, lineDashPhase)
    }
    unsafe fn setLineDashPhase_(&self, lineDashPhase: CGFloat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setLineDashPhase : lineDashPhase)
    }
    unsafe fn lineDashPattern(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, lineDashPattern)
    }
    unsafe fn setLineDashPattern_(&self, lineDashPattern: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setLineDashPattern : lineDashPattern)
    }
}
pub type CATextLayerTruncationMode = NSString;
pub type CATextLayerAlignmentMode = NSString;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CATextLayer(pub id);
impl std::ops::Deref for CATextLayer {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CATextLayer {}
impl CATextLayer {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CATextLayer").unwrap(), alloc) })
    }
}
impl ICALayer for CATextLayer {}
impl PNSSecureCoding for CATextLayer {}
impl PCAMediaTiming for CATextLayer {}
impl std::convert::TryFrom<CALayer> for CATextLayer {
    type Error = &'static str;
    fn try_from(parent: CALayer) -> Result<CATextLayer, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CATextLayer").unwrap()) };
        if is_kind_of {
            Ok(CATextLayer(parent.0))
        } else {
            Err("This CALayer cannot be downcasted to CATextLayer")
        }
    }
}
impl INSObject for CATextLayer {}
impl PNSObject for CATextLayer {}
impl ICATextLayer for CATextLayer {}
pub trait ICATextLayer: Sized + std::ops::Deref {
    unsafe fn string(&self) -> id
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, string)
    }
    unsafe fn setString_(&self, string: id)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setString : string)
    }
    unsafe fn font(&self) -> CFTypeRef
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, font)
    }
    unsafe fn setFont_(&self, font: CFTypeRef)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setFont : font)
    }
    unsafe fn fontSize(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, fontSize)
    }
    unsafe fn setFontSize_(&self, fontSize: CGFloat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setFontSize : fontSize)
    }
    unsafe fn foregroundColor(&self) -> CGColorRef
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, foregroundColor)
    }
    unsafe fn setForegroundColor_(&self, foregroundColor: CGColorRef)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setForegroundColor : foregroundColor)
    }
    unsafe fn isWrapped(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isWrapped)
    }
    unsafe fn setWrapped_(&self, wrapped: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setWrapped : wrapped)
    }
    unsafe fn truncationMode(&self) -> CATextLayerTruncationMode
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, truncationMode)
    }
    unsafe fn setTruncationMode_(&self, truncationMode: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setTruncationMode : truncationMode)
    }
    unsafe fn alignmentMode(&self) -> CATextLayerAlignmentMode
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, alignmentMode)
    }
    unsafe fn setAlignmentMode_(&self, alignmentMode: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAlignmentMode : alignmentMode)
    }
    unsafe fn allowsFontSubpixelQuantization(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, allowsFontSubpixelQuantization)
    }
    unsafe fn setAllowsFontSubpixelQuantization_(&self, allowsFontSubpixelQuantization: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAllowsFontSubpixelQuantization : allowsFontSubpixelQuantization)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CATiledLayer(pub id);
impl std::ops::Deref for CATiledLayer {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CATiledLayer {}
impl CATiledLayer {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CATiledLayer").unwrap(), alloc) })
    }
}
impl ICALayer for CATiledLayer {}
impl PNSSecureCoding for CATiledLayer {}
impl PCAMediaTiming for CATiledLayer {}
impl std::convert::TryFrom<CALayer> for CATiledLayer {
    type Error = &'static str;
    fn try_from(parent: CALayer) -> Result<CATiledLayer, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CATiledLayer").unwrap()) };
        if is_kind_of {
            Ok(CATiledLayer(parent.0))
        } else {
            Err("This CALayer cannot be downcasted to CATiledLayer")
        }
    }
}
impl INSObject for CATiledLayer {}
impl PNSObject for CATiledLayer {}
impl ICATiledLayer for CATiledLayer {}
pub trait ICATiledLayer: Sized + std::ops::Deref {
    unsafe fn levelsOfDetail(&self) -> usize
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, levelsOfDetail)
    }
    unsafe fn setLevelsOfDetail_(&self, levelsOfDetail: usize)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setLevelsOfDetail : levelsOfDetail)
    }
    unsafe fn levelsOfDetailBias(&self) -> usize
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, levelsOfDetailBias)
    }
    unsafe fn setLevelsOfDetailBias_(&self, levelsOfDetailBias: usize)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setLevelsOfDetailBias : levelsOfDetailBias)
    }
    unsafe fn tileSize(&self) -> CGSize
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, tileSize)
    }
    unsafe fn setTileSize_(&self, tileSize: CGSize)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setTileSize : tileSize)
    }
    unsafe fn fadeDuration() -> CFTimeInterval
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CATiledLayer").unwrap(), fadeDuration)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CATransaction(pub id);
impl std::ops::Deref for CATransaction {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CATransaction {}
impl CATransaction {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CATransaction").unwrap(), alloc) })
    }
}
impl INSObject for CATransaction {}
impl PNSObject for CATransaction {}
impl std::convert::TryFrom<NSObject> for CATransaction {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<CATransaction, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CATransaction").unwrap()) };
        if is_kind_of {
            Ok(CATransaction(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to CATransaction")
        }
    }
}
impl ICATransaction for CATransaction {}
pub trait ICATransaction: Sized + std::ops::Deref {
    unsafe fn begin()
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CATransaction").unwrap(), begin)
    }
    unsafe fn commit()
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CATransaction").unwrap(), commit)
    }
    unsafe fn flush()
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CATransaction").unwrap(), flush)
    }
    unsafe fn lock()
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CATransaction").unwrap(), lock)
    }
    unsafe fn unlock()
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CATransaction").unwrap(), unlock)
    }
    unsafe fn animationDuration() -> CFTimeInterval
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CATransaction").unwrap(), animationDuration)
    }
    unsafe fn setAnimationDuration_(dur: CFTimeInterval)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CATransaction").unwrap(), setAnimationDuration : dur)
    }
    unsafe fn animationTimingFunction() -> CAMediaTimingFunction
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CATransaction").unwrap(), animationTimingFunction)
    }
    unsafe fn setAnimationTimingFunction_(function: CAMediaTimingFunction)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CATransaction").unwrap(), setAnimationTimingFunction : function)
    }
    unsafe fn disableActions() -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CATransaction").unwrap(), disableActions)
    }
    unsafe fn setDisableActions_(flag: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CATransaction").unwrap(), setDisableActions : flag)
    }
    unsafe fn completionBlock() -> *mut ::std::os::raw::c_void
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CATransaction").unwrap(), completionBlock)
    }
    unsafe fn setCompletionBlock_(block: *mut ::std::os::raw::c_void)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CATransaction").unwrap(), setCompletionBlock : block)
    }
    unsafe fn valueForKey_(key: NSString) -> id
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CATransaction").unwrap(), valueForKey : key)
    }
    unsafe fn setValue_forKey_(anObject: id, key: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CATransaction").unwrap(), setValue : anObject, forKey : key)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CATransformLayer(pub id);
impl std::ops::Deref for CATransformLayer {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CATransformLayer {}
impl CATransformLayer {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CATransformLayer").unwrap(), alloc) })
    }
}
impl ICALayer for CATransformLayer {}
impl PNSSecureCoding for CATransformLayer {}
impl PCAMediaTiming for CATransformLayer {}
impl std::convert::TryFrom<CALayer> for CATransformLayer {
    type Error = &'static str;
    fn try_from(parent: CALayer) -> Result<CATransformLayer, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CATransformLayer").unwrap()) };
        if is_kind_of {
            Ok(CATransformLayer(parent.0))
        } else {
            Err("This CALayer cannot be downcasted to CATransformLayer")
        }
    }
}
impl INSObject for CATransformLayer {}
impl PNSObject for CATransformLayer {}
impl ICATransformLayer for CATransformLayer {}
pub trait ICATransformLayer: Sized + std::ops::Deref {}
pub type CAValueFunctionName = NSString;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CAValueFunction(pub id);
impl std::ops::Deref for CAValueFunction {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CAValueFunction {}
impl CAValueFunction {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CAValueFunction").unwrap(), alloc) })
    }
}
impl PNSSecureCoding for CAValueFunction {}
impl INSObject for CAValueFunction {}
impl PNSObject for CAValueFunction {}
impl std::convert::TryFrom<NSObject> for CAValueFunction {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<CAValueFunction, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CAValueFunction").unwrap()) };
        if is_kind_of {
            Ok(CAValueFunction(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to CAValueFunction")
        }
    }
}
impl ICAValueFunction for CAValueFunction {}
pub trait ICAValueFunction: Sized + std::ops::Deref {
    unsafe fn name(&self) -> CAValueFunctionName
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, name)
    }
    unsafe fn functionWithName_(name: NSString) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CAValueFunction").unwrap(), functionWithName : name)
    }
}
unsafe extern "C" {
    pub fn CACurrentMediaTime() -> CFTimeInterval;
}
unsafe extern "C" {
    pub static CATransform3DIdentity: CATransform3D;
}
unsafe extern "C" {
    pub fn CATransform3DIsIdentity(t: CATransform3D) -> bool;
}
unsafe extern "C" {
    pub fn CATransform3DEqualToTransform(a: CATransform3D, b: CATransform3D) -> bool;
}
unsafe extern "C" {
    pub fn CATransform3DMakeTranslation(tx: CGFloat, ty: CGFloat, tz: CGFloat) -> CATransform3D;
}
unsafe extern "C" {
    pub fn CATransform3DMakeScale(sx: CGFloat, sy: CGFloat, sz: CGFloat) -> CATransform3D;
}
unsafe extern "C" {
    pub fn CATransform3DMakeRotation(
        angle: CGFloat,
        x: CGFloat,
        y: CGFloat,
        z: CGFloat,
    ) -> CATransform3D;
}
unsafe extern "C" {
    pub fn CATransform3DTranslate(
        t: CATransform3D,
        tx: CGFloat,
        ty: CGFloat,
        tz: CGFloat,
    ) -> CATransform3D;
}
unsafe extern "C" {
    pub fn CATransform3DScale(
        t: CATransform3D,
        sx: CGFloat,
        sy: CGFloat,
        sz: CGFloat,
    ) -> CATransform3D;
}
unsafe extern "C" {
    pub fn CATransform3DRotate(
        t: CATransform3D,
        angle: CGFloat,
        x: CGFloat,
        y: CGFloat,
        z: CGFloat,
    ) -> CATransform3D;
}
unsafe extern "C" {
    pub fn CATransform3DConcat(a: CATransform3D, b: CATransform3D) -> CATransform3D;
}
unsafe extern "C" {
    pub fn CATransform3DInvert(t: CATransform3D) -> CATransform3D;
}
unsafe extern "C" {
    pub fn CATransform3DMakeAffineTransform(m: CGAffineTransform) -> CATransform3D;
}
unsafe extern "C" {
    pub fn CATransform3DIsAffine(t: CATransform3D) -> bool;
}
unsafe extern "C" {
    pub fn CATransform3DGetAffineTransform(t: CATransform3D) -> CGAffineTransform;
}
unsafe extern "C" {
    pub static kCAFillModeForwards: CAMediaTimingFillMode;
}
unsafe extern "C" {
    pub static kCAFillModeBackwards: CAMediaTimingFillMode;
}
unsafe extern "C" {
    pub static kCAFillModeBoth: CAMediaTimingFillMode;
}
unsafe extern "C" {
    pub static kCAFillModeRemoved: CAMediaTimingFillMode;
}
unsafe extern "C" {
    pub static CAToneMapModeAutomatic: CAToneMapMode;
}
unsafe extern "C" {
    pub static CAToneMapModeNever: CAToneMapMode;
}
unsafe extern "C" {
    pub static CAToneMapModeIfSupported: CAToneMapMode;
}
unsafe extern "C" {
    pub static CADynamicRangeAutomatic: CADynamicRange;
}
unsafe extern "C" {
    pub static CADynamicRangeStandard: CADynamicRange;
}
unsafe extern "C" {
    pub static CADynamicRangeConstrainedHigh: CADynamicRange;
}
unsafe extern "C" {
    pub static CADynamicRangeHigh: CADynamicRange;
}
unsafe extern "C" {
    pub static kCAGravityCenter: CALayerContentsGravity;
}
unsafe extern "C" {
    pub static kCAGravityTop: CALayerContentsGravity;
}
unsafe extern "C" {
    pub static kCAGravityBottom: CALayerContentsGravity;
}
unsafe extern "C" {
    pub static kCAGravityLeft: CALayerContentsGravity;
}
unsafe extern "C" {
    pub static kCAGravityRight: CALayerContentsGravity;
}
unsafe extern "C" {
    pub static kCAGravityTopLeft: CALayerContentsGravity;
}
unsafe extern "C" {
    pub static kCAGravityTopRight: CALayerContentsGravity;
}
unsafe extern "C" {
    pub static kCAGravityBottomLeft: CALayerContentsGravity;
}
unsafe extern "C" {
    pub static kCAGravityBottomRight: CALayerContentsGravity;
}
unsafe extern "C" {
    pub static kCAGravityResize: CALayerContentsGravity;
}
unsafe extern "C" {
    pub static kCAGravityResizeAspect: CALayerContentsGravity;
}
unsafe extern "C" {
    pub static kCAGravityResizeAspectFill: CALayerContentsGravity;
}
unsafe extern "C" {
    pub static kCAContentsFormatRGBA8Uint: CALayerContentsFormat;
}
unsafe extern "C" {
    pub static kCAContentsFormatRGBA16Float: CALayerContentsFormat;
}
unsafe extern "C" {
    pub static kCAContentsFormatGray8Uint: CALayerContentsFormat;
}
unsafe extern "C" {
    pub static kCAContentsFormatAutomatic: CALayerContentsFormat;
}
unsafe extern "C" {
    pub static kCAFilterNearest: CALayerContentsFilter;
}
unsafe extern "C" {
    pub static kCAFilterLinear: CALayerContentsFilter;
}
unsafe extern "C" {
    pub static kCAFilterTrilinear: CALayerContentsFilter;
}
unsafe extern "C" {
    pub static kCACornerCurveCircular: CALayerCornerCurve;
}
unsafe extern "C" {
    pub static kCACornerCurveContinuous: CALayerCornerCurve;
}
unsafe extern "C" {
    pub static kCAOnOrderIn: NSString;
}
unsafe extern "C" {
    pub static kCAOnOrderOut: NSString;
}
unsafe extern "C" {
    pub static kCATransition: NSString;
}
unsafe extern "C" {
    pub static CAFrameRateRangeDefault: CAFrameRateRange;
}
unsafe extern "C" {
    pub fn CAFrameRateRangeMake(minimum: f32, maximum: f32, preferred: f32) -> CAFrameRateRange;
}
unsafe extern "C" {
    pub fn CAFrameRateRangeIsEqualToRange(range: CAFrameRateRange, other: CAFrameRateRange)
        -> bool;
}
unsafe extern "C" {
    pub static kCAAnimationLinear: CAAnimationCalculationMode;
}
unsafe extern "C" {
    pub static kCAAnimationDiscrete: CAAnimationCalculationMode;
}
unsafe extern "C" {
    pub static kCAAnimationPaced: CAAnimationCalculationMode;
}
unsafe extern "C" {
    pub static kCAAnimationCubic: CAAnimationCalculationMode;
}
unsafe extern "C" {
    pub static kCAAnimationCubicPaced: CAAnimationCalculationMode;
}
unsafe extern "C" {
    pub static kCAAnimationRotateAuto: CAAnimationRotationMode;
}
unsafe extern "C" {
    pub static kCAAnimationRotateAutoReverse: CAAnimationRotationMode;
}
unsafe extern "C" {
    pub static kCATransitionFade: CATransitionType;
}
unsafe extern "C" {
    pub static kCATransitionMoveIn: CATransitionType;
}
unsafe extern "C" {
    pub static kCATransitionPush: CATransitionType;
}
unsafe extern "C" {
    pub static kCATransitionReveal: CATransitionType;
}
unsafe extern "C" {
    pub static kCATransitionFromRight: CATransitionSubtype;
}
unsafe extern "C" {
    pub static kCATransitionFromLeft: CATransitionSubtype;
}
unsafe extern "C" {
    pub static kCATransitionFromTop: CATransitionSubtype;
}
unsafe extern "C" {
    pub static kCATransitionFromBottom: CATransitionSubtype;
}
unsafe extern "C" {
    pub static kCAEmitterLayerPoint: CAEmitterLayerEmitterShape;
}
unsafe extern "C" {
    pub static kCAEmitterLayerLine: CAEmitterLayerEmitterShape;
}
unsafe extern "C" {
    pub static kCAEmitterLayerRectangle: CAEmitterLayerEmitterShape;
}
unsafe extern "C" {
    pub static kCAEmitterLayerCuboid: CAEmitterLayerEmitterShape;
}
unsafe extern "C" {
    pub static kCAEmitterLayerCircle: CAEmitterLayerEmitterShape;
}
unsafe extern "C" {
    pub static kCAEmitterLayerSphere: CAEmitterLayerEmitterShape;
}
unsafe extern "C" {
    pub static kCAEmitterLayerPoints: CAEmitterLayerEmitterMode;
}
unsafe extern "C" {
    pub static kCAEmitterLayerOutline: CAEmitterLayerEmitterMode;
}
unsafe extern "C" {
    pub static kCAEmitterLayerSurface: CAEmitterLayerEmitterMode;
}
unsafe extern "C" {
    pub static kCAEmitterLayerVolume: CAEmitterLayerEmitterMode;
}
unsafe extern "C" {
    pub static kCAEmitterLayerUnordered: CAEmitterLayerRenderMode;
}
unsafe extern "C" {
    pub static kCAEmitterLayerOldestFirst: CAEmitterLayerRenderMode;
}
unsafe extern "C" {
    pub static kCAEmitterLayerOldestLast: CAEmitterLayerRenderMode;
}
unsafe extern "C" {
    pub static kCAEmitterLayerBackToFront: CAEmitterLayerRenderMode;
}
unsafe extern "C" {
    pub static kCAEmitterLayerAdditive: CAEmitterLayerRenderMode;
}
unsafe extern "C" {
    pub static kCAMediaTimingFunctionLinear: CAMediaTimingFunctionName;
}
unsafe extern "C" {
    pub static kCAMediaTimingFunctionEaseIn: CAMediaTimingFunctionName;
}
unsafe extern "C" {
    pub static kCAMediaTimingFunctionEaseOut: CAMediaTimingFunctionName;
}
unsafe extern "C" {
    pub static kCAMediaTimingFunctionEaseInEaseOut: CAMediaTimingFunctionName;
}
unsafe extern "C" {
    pub static kCAMediaTimingFunctionDefault: CAMediaTimingFunctionName;
}
unsafe extern "C" {
    pub static kCAGradientLayerAxial: CAGradientLayerType;
}
unsafe extern "C" {
    pub static kCAGradientLayerRadial: CAGradientLayerType;
}
unsafe extern "C" {
    pub static kCAGradientLayerConic: CAGradientLayerType;
}
unsafe extern "C" {
    pub static kCARendererColorSpace: NSString;
}
unsafe extern "C" {
    pub static kCARendererMetalCommandQueue: NSString;
}
unsafe extern "C" {
    pub static kCAScrollNone: CAScrollLayerScrollMode;
}
unsafe extern "C" {
    pub static kCAScrollVertically: CAScrollLayerScrollMode;
}
unsafe extern "C" {
    pub static kCAScrollHorizontally: CAScrollLayerScrollMode;
}
unsafe extern "C" {
    pub static kCAScrollBoth: CAScrollLayerScrollMode;
}
unsafe extern "C" {
    pub static kCAFillRuleNonZero: CAShapeLayerFillRule;
}
unsafe extern "C" {
    pub static kCAFillRuleEvenOdd: CAShapeLayerFillRule;
}
unsafe extern "C" {
    pub static kCALineJoinMiter: CAShapeLayerLineJoin;
}
unsafe extern "C" {
    pub static kCALineJoinRound: CAShapeLayerLineJoin;
}
unsafe extern "C" {
    pub static kCALineJoinBevel: CAShapeLayerLineJoin;
}
unsafe extern "C" {
    pub static kCALineCapButt: CAShapeLayerLineCap;
}
unsafe extern "C" {
    pub static kCALineCapRound: CAShapeLayerLineCap;
}
unsafe extern "C" {
    pub static kCALineCapSquare: CAShapeLayerLineCap;
}
unsafe extern "C" {
    pub static kCATruncationNone: CATextLayerTruncationMode;
}
unsafe extern "C" {
    pub static kCATruncationStart: CATextLayerTruncationMode;
}
unsafe extern "C" {
    pub static kCATruncationEnd: CATextLayerTruncationMode;
}
unsafe extern "C" {
    pub static kCATruncationMiddle: CATextLayerTruncationMode;
}
unsafe extern "C" {
    pub static kCAAlignmentNatural: CATextLayerAlignmentMode;
}
unsafe extern "C" {
    pub static kCAAlignmentLeft: CATextLayerAlignmentMode;
}
unsafe extern "C" {
    pub static kCAAlignmentRight: CATextLayerAlignmentMode;
}
unsafe extern "C" {
    pub static kCAAlignmentCenter: CATextLayerAlignmentMode;
}
unsafe extern "C" {
    pub static kCAAlignmentJustified: CATextLayerAlignmentMode;
}
unsafe extern "C" {
    pub static kCATransactionAnimationDuration: NSString;
}
unsafe extern "C" {
    pub static kCATransactionDisableActions: NSString;
}
unsafe extern "C" {
    pub static kCATransactionAnimationTimingFunction: NSString;
}
unsafe extern "C" {
    pub static kCATransactionCompletionBlock: NSString;
}
unsafe extern "C" {
    pub static kCAValueFunctionRotateX: CAValueFunctionName;
}
unsafe extern "C" {
    pub static kCAValueFunctionRotateY: CAValueFunctionName;
}
unsafe extern "C" {
    pub static kCAValueFunctionRotateZ: CAValueFunctionName;
}
unsafe extern "C" {
    pub static kCAValueFunctionScale: CAValueFunctionName;
}
unsafe extern "C" {
    pub static kCAValueFunctionScaleX: CAValueFunctionName;
}
unsafe extern "C" {
    pub static kCAValueFunctionScaleY: CAValueFunctionName;
}
unsafe extern "C" {
    pub static kCAValueFunctionScaleZ: CAValueFunctionName;
}
unsafe extern "C" {
    pub static kCAValueFunctionTranslate: CAValueFunctionName;
}
unsafe extern "C" {
    pub static kCAValueFunctionTranslateX: CAValueFunctionName;
}
unsafe extern "C" {
    pub static kCAValueFunctionTranslateY: CAValueFunctionName;
}
unsafe extern "C" {
    pub static kCAValueFunctionTranslateZ: CAValueFunctionName;
}

unsafe impl objc2::encode::RefEncode for CVTimeStamp {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CVTimeStamp {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("CVTimeStamp", &[]);
}
unsafe impl objc2::encode::RefEncode for CATransform3D {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CATransform3D {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("CATransform3D", &[]);
}
unsafe impl objc2::encode::RefEncode for CALayer {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CALayer {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CAFrameRateRange {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CAFrameRateRange {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("CAFrameRateRange", &[]);
}
unsafe impl objc2::encode::RefEncode for CAAnimation {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CAAnimation {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CAPropertyAnimation {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CAPropertyAnimation {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CABasicAnimation {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CABasicAnimation {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CAKeyframeAnimation {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CAKeyframeAnimation {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CASpringAnimation {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CASpringAnimation {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CATransition {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CATransition {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CAAnimationGroup {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CAAnimationGroup {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CAConstraintLayoutManager {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CAConstraintLayoutManager {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CAConstraint {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CAConstraint {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CADisplayLink {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CADisplayLink {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CAEDRMetadata {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CAEDRMetadata {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CAMetalLayer {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CAMetalLayer {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CAMetalDisplayLinkUpdate {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CAMetalDisplayLinkUpdate {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CAMetalDisplayLink {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CAMetalDisplayLink {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CAEmitterCell {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CAEmitterCell {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CAEmitterLayer {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CAEmitterLayer {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CAMediaTimingFunction {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CAMediaTimingFunction {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CAGradientLayer {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CAGradientLayer {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CAOpenGLLayer {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CAOpenGLLayer {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CARemoteLayerClient {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CARemoteLayerClient {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CARemoteLayerServer {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CARemoteLayerServer {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CARenderer {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CARenderer {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CAReplicatorLayer {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CAReplicatorLayer {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CAScrollLayer {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CAScrollLayer {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CAShapeLayer {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CAShapeLayer {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CATextLayer {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CATextLayer {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CATiledLayer {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CATiledLayer {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CATransaction {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CATransaction {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CATransformLayer {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CATransformLayer {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CAValueFunction {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CAValueFunction {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
