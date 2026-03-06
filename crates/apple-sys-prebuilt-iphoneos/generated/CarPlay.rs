#[allow(unused_imports)]
use crate::objc::*;
#[allow(unused_imports)]
use crate::CoreData::*;
#[allow(unused_imports)]
use crate::CoreGraphics::*;
#[allow(unused_imports)]
use crate::Foundation::*;
#[allow(unused_imports)]
use crate::MapKit::*;
#[allow(unused_imports)]
use crate::Security::*;
#[allow(unused_imports)]
use crate::UIKit::*;

#[allow(unused_imports)]
use objc2::msg_send;
pub trait MKMapItem_MKMapItemDragAndDropSupport: Sized + std::ops::Deref {}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MKOverlayView(pub id);
impl std::ops::Deref for MKOverlayView {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MKOverlayView {}
impl MKOverlayView {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MKOverlayView").unwrap(), alloc) })
    }
}
impl IUIView for MKOverlayView {}
impl PNSCoding for MKOverlayView {}
impl PUIAppearance for MKOverlayView {}
impl PUIAppearanceContainer for MKOverlayView {}
impl PUIDynamicItem for MKOverlayView {}
impl PUITraitEnvironment for MKOverlayView {}
impl PUICoordinateSpace for MKOverlayView {}
impl PUIFocusItem for MKOverlayView {}
impl PUIFocusItemContainer for MKOverlayView {}
impl std::convert::TryFrom<UIView> for MKOverlayView {
    type Error = &'static str;
    fn try_from(parent: UIView) -> Result<MKOverlayView, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MKOverlayView").unwrap()) };
        if is_kind_of {
            Ok(MKOverlayView(parent.0))
        } else {
            Err("This UIView cannot be downcasted to MKOverlayView")
        }
    }
}
impl IUIResponder for MKOverlayView {}
impl PUIResponderStandardEditActions for MKOverlayView {}
impl INSObject for MKOverlayView {}
impl PNSObject for MKOverlayView {}
impl IMKOverlayView for MKOverlayView {}
pub trait IMKOverlayView: Sized + std::ops::Deref {
    unsafe fn initWithOverlay_(&self, overlay: *mut u64) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithOverlay : overlay)
    }
    unsafe fn pointForMapPoint_(&self, mapPoint: MKMapPoint) -> CGPoint
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, pointForMapPoint : mapPoint)
    }
    unsafe fn mapPointForPoint_(&self, point: CGPoint) -> MKMapPoint
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, mapPointForPoint : point)
    }
    unsafe fn rectForMapRect_(&self, mapRect: MKMapRect) -> CGRect
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, rectForMapRect : mapRect)
    }
    unsafe fn mapRectForRect_(&self, rect: CGRect) -> MKMapRect
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, mapRectForRect : rect)
    }
    unsafe fn canDrawMapRect_zoomScale_(&self, mapRect: MKMapRect, zoomScale: MKZoomScale) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, canDrawMapRect : mapRect, zoomScale : zoomScale)
    }
    unsafe fn drawMapRect_zoomScale_inContext_(
        &self,
        mapRect: MKMapRect,
        zoomScale: MKZoomScale,
        context: CGContextRef,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, drawMapRect : mapRect, zoomScale : zoomScale, inContext : context)
    }
    unsafe fn setNeedsDisplayInMapRect_(&self, mapRect: MKMapRect)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setNeedsDisplayInMapRect : mapRect)
    }
    unsafe fn setNeedsDisplayInMapRect_zoomScale_(&self, mapRect: MKMapRect, zoomScale: MKZoomScale)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setNeedsDisplayInMapRect : mapRect, zoomScale : zoomScale)
    }
    unsafe fn overlay(&self) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, overlay)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MKIconStyle(pub id);
impl std::ops::Deref for MKIconStyle {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MKIconStyle {}
impl MKIconStyle {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MKIconStyle").unwrap(), alloc) })
    }
}
impl INSObject for MKIconStyle {}
impl PNSObject for MKIconStyle {}
impl std::convert::TryFrom<NSObject> for MKIconStyle {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<MKIconStyle, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MKIconStyle").unwrap()) };
        if is_kind_of {
            Ok(MKIconStyle(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to MKIconStyle")
        }
    }
}
impl IMKIconStyle for MKIconStyle {}
pub trait IMKIconStyle: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn backgroundColor(&self) -> UIColor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, backgroundColor)
    }
    unsafe fn image(&self) -> UIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, image)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MKIconStyle").unwrap(), new)
    }
}
pub type MKMapFeatureType = NSInteger;
pub type MKMapFeatureOptions = NSInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MKReverseGeocoder(pub id);
impl std::ops::Deref for MKReverseGeocoder {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MKReverseGeocoder {}
impl MKReverseGeocoder {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MKReverseGeocoder").unwrap(), alloc) })
    }
}
impl INSObject for MKReverseGeocoder {}
impl PNSObject for MKReverseGeocoder {}
impl std::convert::TryFrom<NSObject> for MKReverseGeocoder {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<MKReverseGeocoder, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MKReverseGeocoder").unwrap()) };
        if is_kind_of {
            Ok(MKReverseGeocoder(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to MKReverseGeocoder")
        }
    }
}
impl IMKReverseGeocoder for MKReverseGeocoder {}
pub trait IMKReverseGeocoder: Sized + std::ops::Deref {
    unsafe fn initWithCoordinate_(&self, coordinate: CLLocationCoordinate2D) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithCoordinate : coordinate)
    }
    unsafe fn start(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, start)
    }
    unsafe fn cancel(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, cancel)
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
    unsafe fn coordinate(&self) -> CLLocationCoordinate2D
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, coordinate)
    }
    unsafe fn placemark(&self) -> MKPlacemark
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, placemark)
    }
    unsafe fn isQuerying(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isQuerying)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MKOverlayPathView(pub id);
impl std::ops::Deref for MKOverlayPathView {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MKOverlayPathView {}
impl MKOverlayPathView {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MKOverlayPathView").unwrap(), alloc) })
    }
}
impl IMKOverlayView for MKOverlayPathView {}
impl From<MKOverlayPathView> for MKOverlayView {
    fn from(child: MKOverlayPathView) -> MKOverlayView {
        MKOverlayView(child.0)
    }
}
impl std::convert::TryFrom<MKOverlayView> for MKOverlayPathView {
    type Error = &'static str;
    fn try_from(parent: MKOverlayView) -> Result<MKOverlayPathView, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MKOverlayPathView").unwrap()) };
        if is_kind_of {
            Ok(MKOverlayPathView(parent.0))
        } else {
            Err("This MKOverlayView cannot be downcasted to MKOverlayPathView")
        }
    }
}
impl IUIView for MKOverlayPathView {}
impl PNSCoding for MKOverlayPathView {}
impl PUIAppearance for MKOverlayPathView {}
impl PUIAppearanceContainer for MKOverlayPathView {}
impl PUIDynamicItem for MKOverlayPathView {}
impl PUITraitEnvironment for MKOverlayPathView {}
impl PUICoordinateSpace for MKOverlayPathView {}
impl PUIFocusItem for MKOverlayPathView {}
impl PUIFocusItemContainer for MKOverlayPathView {}
impl IUIResponder for MKOverlayPathView {}
impl PUIResponderStandardEditActions for MKOverlayPathView {}
impl INSObject for MKOverlayPathView {}
impl PNSObject for MKOverlayPathView {}
impl IMKOverlayPathView for MKOverlayPathView {}
pub trait IMKOverlayPathView: Sized + std::ops::Deref {
    unsafe fn createPath(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, createPath)
    }
    unsafe fn invalidatePath(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, invalidatePath)
    }
    unsafe fn applyStrokePropertiesToContext_atZoomScale_(
        &self,
        context: CGContextRef,
        zoomScale: MKZoomScale,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, applyStrokePropertiesToContext : context, atZoomScale : zoomScale)
    }
    unsafe fn applyFillPropertiesToContext_atZoomScale_(
        &self,
        context: CGContextRef,
        zoomScale: MKZoomScale,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, applyFillPropertiesToContext : context, atZoomScale : zoomScale)
    }
    unsafe fn strokePath_inContext_(&self, path: CGPathRef, context: CGContextRef)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, strokePath : path, inContext : context)
    }
    unsafe fn fillPath_inContext_(&self, path: CGPathRef, context: CGContextRef)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, fillPath : path, inContext : context)
    }
    unsafe fn fillColor(&self) -> UIColor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, fillColor)
    }
    unsafe fn setFillColor_(&self, fillColor: UIColor)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setFillColor : fillColor)
    }
    unsafe fn strokeColor(&self) -> UIColor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, strokeColor)
    }
    unsafe fn setStrokeColor_(&self, strokeColor: UIColor)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setStrokeColor : strokeColor)
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
    unsafe fn lineJoin(&self) -> CGLineJoin
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, lineJoin)
    }
    unsafe fn setLineJoin_(&self, lineJoin: CGLineJoin)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setLineJoin : lineJoin)
    }
    unsafe fn lineCap(&self) -> CGLineCap
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, lineCap)
    }
    unsafe fn setLineCap_(&self, lineCap: CGLineCap)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setLineCap : lineCap)
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
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MKPolygonView(pub id);
impl std::ops::Deref for MKPolygonView {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MKPolygonView {}
impl MKPolygonView {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MKPolygonView").unwrap(), alloc) })
    }
}
impl IMKOverlayPathView for MKPolygonView {}
impl From<MKPolygonView> for MKOverlayPathView {
    fn from(child: MKPolygonView) -> MKOverlayPathView {
        MKOverlayPathView(child.0)
    }
}
impl std::convert::TryFrom<MKOverlayPathView> for MKPolygonView {
    type Error = &'static str;
    fn try_from(parent: MKOverlayPathView) -> Result<MKPolygonView, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MKPolygonView").unwrap()) };
        if is_kind_of {
            Ok(MKPolygonView(parent.0))
        } else {
            Err("This MKOverlayPathView cannot be downcasted to MKPolygonView")
        }
    }
}
impl IMKOverlayView for MKPolygonView {}
impl IUIView for MKPolygonView {}
impl PNSCoding for MKPolygonView {}
impl PUIAppearance for MKPolygonView {}
impl PUIAppearanceContainer for MKPolygonView {}
impl PUIDynamicItem for MKPolygonView {}
impl PUITraitEnvironment for MKPolygonView {}
impl PUICoordinateSpace for MKPolygonView {}
impl PUIFocusItem for MKPolygonView {}
impl PUIFocusItemContainer for MKPolygonView {}
impl IUIResponder for MKPolygonView {}
impl PUIResponderStandardEditActions for MKPolygonView {}
impl INSObject for MKPolygonView {}
impl PNSObject for MKPolygonView {}
impl IMKPolygonView for MKPolygonView {}
pub trait IMKPolygonView: Sized + std::ops::Deref {
    unsafe fn initWithPolygon_(&self, polygon: MKPolygon) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithPolygon : polygon)
    }
    unsafe fn polygon(&self) -> MKPolygon
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, polygon)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MKPolylineView(pub id);
impl std::ops::Deref for MKPolylineView {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MKPolylineView {}
impl MKPolylineView {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MKPolylineView").unwrap(), alloc) })
    }
}
impl IMKOverlayPathView for MKPolylineView {}
impl std::convert::TryFrom<MKOverlayPathView> for MKPolylineView {
    type Error = &'static str;
    fn try_from(parent: MKOverlayPathView) -> Result<MKPolylineView, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MKPolylineView").unwrap()) };
        if is_kind_of {
            Ok(MKPolylineView(parent.0))
        } else {
            Err("This MKOverlayPathView cannot be downcasted to MKPolylineView")
        }
    }
}
impl IMKOverlayView for MKPolylineView {}
impl IUIView for MKPolylineView {}
impl PNSCoding for MKPolylineView {}
impl PUIAppearance for MKPolylineView {}
impl PUIAppearanceContainer for MKPolylineView {}
impl PUIDynamicItem for MKPolylineView {}
impl PUITraitEnvironment for MKPolylineView {}
impl PUICoordinateSpace for MKPolylineView {}
impl PUIFocusItem for MKPolylineView {}
impl PUIFocusItemContainer for MKPolylineView {}
impl IUIResponder for MKPolylineView {}
impl PUIResponderStandardEditActions for MKPolylineView {}
impl INSObject for MKPolylineView {}
impl PNSObject for MKPolylineView {}
impl IMKPolylineView for MKPolylineView {}
pub trait IMKPolylineView: Sized + std::ops::Deref {
    unsafe fn initWithPolyline_(&self, polyline: MKPolyline) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithPolyline : polyline)
    }
    unsafe fn polyline(&self) -> MKPolyline
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, polyline)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MKCircleView(pub id);
impl std::ops::Deref for MKCircleView {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MKCircleView {}
impl MKCircleView {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MKCircleView").unwrap(), alloc) })
    }
}
impl IMKOverlayPathView for MKCircleView {}
impl std::convert::TryFrom<MKOverlayPathView> for MKCircleView {
    type Error = &'static str;
    fn try_from(parent: MKOverlayPathView) -> Result<MKCircleView, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MKCircleView").unwrap()) };
        if is_kind_of {
            Ok(MKCircleView(parent.0))
        } else {
            Err("This MKOverlayPathView cannot be downcasted to MKCircleView")
        }
    }
}
impl IMKOverlayView for MKCircleView {}
impl IUIView for MKCircleView {}
impl PNSCoding for MKCircleView {}
impl PUIAppearance for MKCircleView {}
impl PUIAppearanceContainer for MKCircleView {}
impl PUIDynamicItem for MKCircleView {}
impl PUITraitEnvironment for MKCircleView {}
impl PUICoordinateSpace for MKCircleView {}
impl PUIFocusItem for MKCircleView {}
impl PUIFocusItemContainer for MKCircleView {}
impl IUIResponder for MKCircleView {}
impl PUIResponderStandardEditActions for MKCircleView {}
impl INSObject for MKCircleView {}
impl PNSObject for MKCircleView {}
impl IMKCircleView for MKCircleView {}
pub trait IMKCircleView: Sized + std::ops::Deref {
    unsafe fn initWithCircle_(&self, circle: MKCircle) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithCircle : circle)
    }
    unsafe fn circle(&self) -> MKCircle
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, circle)
    }
}
pub type MKScaleViewAlignment = NSInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MKScaleView(pub id);
impl std::ops::Deref for MKScaleView {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MKScaleView {}
impl MKScaleView {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MKScaleView").unwrap(), alloc) })
    }
}
impl IUIView for MKScaleView {}
impl PNSCoding for MKScaleView {}
impl PUIAppearance for MKScaleView {}
impl PUIAppearanceContainer for MKScaleView {}
impl PUIDynamicItem for MKScaleView {}
impl PUITraitEnvironment for MKScaleView {}
impl PUICoordinateSpace for MKScaleView {}
impl PUIFocusItem for MKScaleView {}
impl PUIFocusItemContainer for MKScaleView {}
impl std::convert::TryFrom<UIView> for MKScaleView {
    type Error = &'static str;
    fn try_from(parent: UIView) -> Result<MKScaleView, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MKScaleView").unwrap()) };
        if is_kind_of {
            Ok(MKScaleView(parent.0))
        } else {
            Err("This UIView cannot be downcasted to MKScaleView")
        }
    }
}
impl IUIResponder for MKScaleView {}
impl PUIResponderStandardEditActions for MKScaleView {}
impl INSObject for MKScaleView {}
impl PNSObject for MKScaleView {}
impl IMKScaleView for MKScaleView {}
pub trait IMKScaleView: Sized + std::ops::Deref {
    unsafe fn mapView(&self) -> MKMapView
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, mapView)
    }
    unsafe fn setMapView_(&self, mapView: MKMapView)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMapView : mapView)
    }
    unsafe fn scaleVisibility(&self) -> MKFeatureVisibility
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, scaleVisibility)
    }
    unsafe fn setScaleVisibility_(&self, scaleVisibility: MKFeatureVisibility)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setScaleVisibility : scaleVisibility)
    }
    unsafe fn legendAlignment(&self) -> MKScaleViewAlignment
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, legendAlignment)
    }
    unsafe fn setLegendAlignment_(&self, legendAlignment: MKScaleViewAlignment)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setLegendAlignment : legendAlignment)
    }
    unsafe fn scaleViewWithMapView_(mapView: MKMapView) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MKScaleView").unwrap(), scaleViewWithMapView : mapView)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MKUserTrackingBarButtonItem(pub id);
impl std::ops::Deref for MKUserTrackingBarButtonItem {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MKUserTrackingBarButtonItem {}
impl MKUserTrackingBarButtonItem {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MKUserTrackingBarButtonItem").unwrap(), alloc) })
    }
}
impl IUIBarButtonItem for MKUserTrackingBarButtonItem {}
impl PNSCoding for MKUserTrackingBarButtonItem {}
impl std::convert::TryFrom<UIBarButtonItem> for MKUserTrackingBarButtonItem {
    type Error = &'static str;
    fn try_from(parent: UIBarButtonItem) -> Result<MKUserTrackingBarButtonItem, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MKUserTrackingBarButtonItem").unwrap()) };
        if is_kind_of {
            Ok(MKUserTrackingBarButtonItem(parent.0))
        } else {
            Err("This UIBarButtonItem cannot be downcasted to MKUserTrackingBarButtonItem")
        }
    }
}
impl IUIBarItem for MKUserTrackingBarButtonItem {}
impl PUIAppearance for MKUserTrackingBarButtonItem {}
impl INSObject for MKUserTrackingBarButtonItem {}
impl PNSObject for MKUserTrackingBarButtonItem {}
impl IMKUserTrackingBarButtonItem for MKUserTrackingBarButtonItem {}
pub trait IMKUserTrackingBarButtonItem: Sized + std::ops::Deref {
    unsafe fn initWithMapView_(&self, mapView: MKMapView) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithMapView : mapView)
    }
    unsafe fn mapView(&self) -> MKMapView
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, mapView)
    }
    unsafe fn setMapView_(&self, mapView: MKMapView)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMapView : mapView)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MKUserTrackingButton(pub id);
impl std::ops::Deref for MKUserTrackingButton {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MKUserTrackingButton {}
impl MKUserTrackingButton {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MKUserTrackingButton").unwrap(), alloc) })
    }
}
impl IUIView for MKUserTrackingButton {}
impl PNSCoding for MKUserTrackingButton {}
impl PUIAppearance for MKUserTrackingButton {}
impl PUIAppearanceContainer for MKUserTrackingButton {}
impl PUIDynamicItem for MKUserTrackingButton {}
impl PUITraitEnvironment for MKUserTrackingButton {}
impl PUICoordinateSpace for MKUserTrackingButton {}
impl PUIFocusItem for MKUserTrackingButton {}
impl PUIFocusItemContainer for MKUserTrackingButton {}
impl std::convert::TryFrom<UIView> for MKUserTrackingButton {
    type Error = &'static str;
    fn try_from(parent: UIView) -> Result<MKUserTrackingButton, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MKUserTrackingButton").unwrap()) };
        if is_kind_of {
            Ok(MKUserTrackingButton(parent.0))
        } else {
            Err("This UIView cannot be downcasted to MKUserTrackingButton")
        }
    }
}
impl IUIResponder for MKUserTrackingButton {}
impl PUIResponderStandardEditActions for MKUserTrackingButton {}
impl INSObject for MKUserTrackingButton {}
impl PNSObject for MKUserTrackingButton {}
impl IMKUserTrackingButton for MKUserTrackingButton {}
pub trait IMKUserTrackingButton: Sized + std::ops::Deref {
    unsafe fn mapView(&self) -> MKMapView
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, mapView)
    }
    unsafe fn setMapView_(&self, mapView: MKMapView)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMapView : mapView)
    }
    unsafe fn userTrackingButtonWithMapView_(mapView: MKMapView) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MKUserTrackingButton").unwrap(), userTrackingButtonWithMapView : mapView)
    }
}
pub type CPAlertActionStyle = NSUInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CPAlertAction(pub id);
impl std::ops::Deref for CPAlertAction {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CPAlertAction {}
impl CPAlertAction {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CPAlertAction").unwrap(), alloc) })
    }
}
impl PNSSecureCoding for CPAlertAction {}
impl INSObject for CPAlertAction {}
impl PNSObject for CPAlertAction {}
impl std::convert::TryFrom<NSObject> for CPAlertAction {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<CPAlertAction, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CPAlertAction").unwrap()) };
        if is_kind_of {
            Ok(CPAlertAction(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to CPAlertAction")
        }
    }
}
impl ICPAlertAction for CPAlertAction {}
pub trait ICPAlertAction: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn initWithTitle_style_handler_(
        &self,
        title: NSString,
        style: CPAlertActionStyle,
        handler: CPAlertActionHandler,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithTitle : title, style : style, handler : handler)
    }
    unsafe fn initWithTitle_color_handler_(
        &self,
        title: NSString,
        color: UIColor,
        handler: CPAlertActionHandler,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithTitle : title, color : color, handler : handler)
    }
    unsafe fn title(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, title)
    }
    unsafe fn style(&self) -> CPAlertActionStyle
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, style)
    }
    unsafe fn handler(&self) -> CPAlertActionHandler
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, handler)
    }
    unsafe fn color(&self) -> UIColor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, color)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CPAlertAction").unwrap(), new)
    }
}
pub type CPAlertActionHandler = *mut ::std::os::raw::c_void;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CPTemplate(pub id);
impl std::ops::Deref for CPTemplate {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CPTemplate {}
impl CPTemplate {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CPTemplate").unwrap(), alloc) })
    }
}
impl PNSSecureCoding for CPTemplate {}
impl INSObject for CPTemplate {}
impl PNSObject for CPTemplate {}
impl std::convert::TryFrom<NSObject> for CPTemplate {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<CPTemplate, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CPTemplate").unwrap()) };
        if is_kind_of {
            Ok(CPTemplate(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to CPTemplate")
        }
    }
}
impl ICPTemplate for CPTemplate {}
pub trait ICPTemplate: Sized + std::ops::Deref {
    unsafe fn userInfo(&self) -> id
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, userInfo)
    }
    unsafe fn setUserInfo_(&self, userInfo: id)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setUserInfo : userInfo)
    }
    unsafe fn tabTitle(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, tabTitle)
    }
    unsafe fn setTabTitle_(&self, tabTitle: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setTabTitle : tabTitle)
    }
    unsafe fn tabImage(&self) -> UIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, tabImage)
    }
    unsafe fn setTabImage_(&self, tabImage: UIImage)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setTabImage : tabImage)
    }
    unsafe fn tabSystemItem(&self) -> UITabBarSystemItem
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, tabSystemItem)
    }
    unsafe fn setTabSystemItem_(&self, tabSystemItem: UITabBarSystemItem)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setTabSystemItem : tabSystemItem)
    }
    unsafe fn showsTabBadge(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, showsTabBadge)
    }
    unsafe fn setShowsTabBadge_(&self, showsTabBadge: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setShowsTabBadge : showsTabBadge)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CPActionSheetTemplate(pub id);
impl std::ops::Deref for CPActionSheetTemplate {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CPActionSheetTemplate {}
impl CPActionSheetTemplate {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CPActionSheetTemplate").unwrap(), alloc) })
    }
}
impl ICPTemplate for CPActionSheetTemplate {}
impl PNSSecureCoding for CPActionSheetTemplate {}
impl From<CPActionSheetTemplate> for CPTemplate {
    fn from(child: CPActionSheetTemplate) -> CPTemplate {
        CPTemplate(child.0)
    }
}
impl std::convert::TryFrom<CPTemplate> for CPActionSheetTemplate {
    type Error = &'static str;
    fn try_from(parent: CPTemplate) -> Result<CPActionSheetTemplate, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CPActionSheetTemplate").unwrap()) };
        if is_kind_of {
            Ok(CPActionSheetTemplate(parent.0))
        } else {
            Err("This CPTemplate cannot be downcasted to CPActionSheetTemplate")
        }
    }
}
impl INSObject for CPActionSheetTemplate {}
impl PNSObject for CPActionSheetTemplate {}
impl ICPActionSheetTemplate for CPActionSheetTemplate {}
pub trait ICPActionSheetTemplate: Sized + std::ops::Deref {
    unsafe fn initWithTitle_message_actions_(
        &self,
        title: NSString,
        message: NSString,
        actions: NSArray,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithTitle : title, message : message, actions : actions)
    }
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn title(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, title)
    }
    unsafe fn message(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, message)
    }
    unsafe fn actions(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, actions)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CPActionSheetTemplate").unwrap(), new)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CPAlertTemplate(pub id);
impl std::ops::Deref for CPAlertTemplate {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CPAlertTemplate {}
impl CPAlertTemplate {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CPAlertTemplate").unwrap(), alloc) })
    }
}
impl ICPTemplate for CPAlertTemplate {}
impl PNSSecureCoding for CPAlertTemplate {}
impl std::convert::TryFrom<CPTemplate> for CPAlertTemplate {
    type Error = &'static str;
    fn try_from(parent: CPTemplate) -> Result<CPAlertTemplate, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CPAlertTemplate").unwrap()) };
        if is_kind_of {
            Ok(CPAlertTemplate(parent.0))
        } else {
            Err("This CPTemplate cannot be downcasted to CPAlertTemplate")
        }
    }
}
impl INSObject for CPAlertTemplate {}
impl PNSObject for CPAlertTemplate {}
impl ICPAlertTemplate for CPAlertTemplate {}
pub trait ICPAlertTemplate: Sized + std::ops::Deref {
    unsafe fn initWithTitleVariants_actions_(
        &self,
        titleVariants: NSArray,
        actions: NSArray,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithTitleVariants : titleVariants, actions : actions)
    }
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn titleVariants(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, titleVariants)
    }
    unsafe fn actions(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, actions)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CPAlertTemplate").unwrap(), new)
    }
    unsafe fn maximumActionCount() -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CPAlertTemplate").unwrap(), maximumActionCount)
    }
}
pub type CPBarButtonStyle = NSInteger;
pub type CPBarButtonType = NSUInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CPBarButton(pub id);
impl std::ops::Deref for CPBarButton {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CPBarButton {}
impl CPBarButton {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CPBarButton").unwrap(), alloc) })
    }
}
impl PNSSecureCoding for CPBarButton {}
impl INSObject for CPBarButton {}
impl PNSObject for CPBarButton {}
impl std::convert::TryFrom<NSObject> for CPBarButton {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<CPBarButton, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CPBarButton").unwrap()) };
        if is_kind_of {
            Ok(CPBarButton(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to CPBarButton")
        }
    }
}
impl ICPBarButton for CPBarButton {}
pub trait ICPBarButton: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn initWithImage_handler_(
        &self,
        image: UIImage,
        handler: CPBarButtonHandler,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithImage : image, handler : handler)
    }
    unsafe fn initWithTitle_handler_(
        &self,
        title: NSString,
        handler: CPBarButtonHandler,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithTitle : title, handler : handler)
    }
    unsafe fn initWithType_handler_(
        &self,
        type_: CPBarButtonType,
        handler: CPBarButtonHandler,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithType : type_, handler : handler)
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
    unsafe fn buttonStyle(&self) -> CPBarButtonStyle
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, buttonStyle)
    }
    unsafe fn setButtonStyle_(&self, buttonStyle: CPBarButtonStyle)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setButtonStyle : buttonStyle)
    }
    unsafe fn image(&self) -> UIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, image)
    }
    unsafe fn setImage_(&self, image: UIImage)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setImage : image)
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
    unsafe fn buttonType(&self) -> CPBarButtonType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, buttonType)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CPBarButton").unwrap(), new)
    }
}
pub type CPBarButtonHandler = *mut ::std::os::raw::c_void;
pub trait PCPBarButtonProviding: Sized + std::ops::Deref {
    unsafe fn leadingNavigationBarButtons(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, leadingNavigationBarButtons)
    }
    unsafe fn setLeadingNavigationBarButtons_(&self, leadingNavigationBarButtons: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setLeadingNavigationBarButtons : leadingNavigationBarButtons)
    }
    unsafe fn trailingNavigationBarButtons(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, trailingNavigationBarButtons)
    }
    unsafe fn setTrailingNavigationBarButtons_(&self, trailingNavigationBarButtons: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setTrailingNavigationBarButtons : trailingNavigationBarButtons)
    }
    unsafe fn backButton(&self) -> CPBarButton
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, backButton)
    }
    unsafe fn setBackButton_(&self, backButton: CPBarButton)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setBackButton : backButton)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CPButton(pub id);
impl std::ops::Deref for CPButton {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CPButton {}
impl CPButton {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CPButton").unwrap(), alloc) })
    }
}
impl INSObject for CPButton {}
impl PNSObject for CPButton {}
impl std::convert::TryFrom<NSObject> for CPButton {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<CPButton, Self::Error> {
        let is_kind_of: bool = unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CPButton").unwrap()) };
        if is_kind_of {
            Ok(CPButton(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to CPButton")
        }
    }
}
impl ICPButton for CPButton {}
pub trait ICPButton: Sized + std::ops::Deref {
    unsafe fn initWithImage_handler_(
        &self,
        image: UIImage,
        handler: *mut ::std::os::raw::c_void,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithImage : image, handler : handler)
    }
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn image(&self) -> UIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, image)
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
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CPButton").unwrap(), new)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CPContact(pub id);
impl std::ops::Deref for CPContact {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CPContact {}
impl CPContact {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CPContact").unwrap(), alloc) })
    }
}
impl PNSSecureCoding for CPContact {}
impl INSObject for CPContact {}
impl PNSObject for CPContact {}
impl std::convert::TryFrom<NSObject> for CPContact {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<CPContact, Self::Error> {
        let is_kind_of: bool = unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CPContact").unwrap()) };
        if is_kind_of {
            Ok(CPContact(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to CPContact")
        }
    }
}
impl ICPContact for CPContact {}
pub trait ICPContact: Sized + std::ops::Deref {
    unsafe fn initWithName_image_(&self, name: NSString, image: UIImage) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithName : name, image : image)
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
    unsafe fn image(&self) -> UIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, image)
    }
    unsafe fn setImage_(&self, image: UIImage)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setImage : image)
    }
    unsafe fn actions(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, actions)
    }
    unsafe fn setActions_(&self, actions: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setActions : actions)
    }
    unsafe fn subtitle(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, subtitle)
    }
    unsafe fn setSubtitle_(&self, subtitle: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSubtitle : subtitle)
    }
    unsafe fn informativeText(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, informativeText)
    }
    unsafe fn setInformativeText_(&self, informativeText: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInformativeText : informativeText)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CPContactCallButton(pub id);
impl std::ops::Deref for CPContactCallButton {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CPContactCallButton {}
impl CPContactCallButton {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CPContactCallButton").unwrap(), alloc) })
    }
}
impl ICPButton for CPContactCallButton {}
impl From<CPContactCallButton> for CPButton {
    fn from(child: CPContactCallButton) -> CPButton {
        CPButton(child.0)
    }
}
impl std::convert::TryFrom<CPButton> for CPContactCallButton {
    type Error = &'static str;
    fn try_from(parent: CPButton) -> Result<CPContactCallButton, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CPContactCallButton").unwrap()) };
        if is_kind_of {
            Ok(CPContactCallButton(parent.0))
        } else {
            Err("This CPButton cannot be downcasted to CPContactCallButton")
        }
    }
}
impl INSObject for CPContactCallButton {}
impl PNSObject for CPContactCallButton {}
impl ICPContactCallButton for CPContactCallButton {}
pub trait ICPContactCallButton: Sized + std::ops::Deref {
    unsafe fn initWithHandler_(&self, handler: *mut ::std::os::raw::c_void) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithHandler : handler)
    }
    unsafe fn initWithImage_handler_(
        &self,
        image: UIImage,
        handler: *mut ::std::os::raw::c_void,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithImage : image, handler : handler)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CPContactMessageButton(pub id);
impl std::ops::Deref for CPContactMessageButton {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CPContactMessageButton {}
impl CPContactMessageButton {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CPContactMessageButton").unwrap(), alloc) })
    }
}
impl ICPButton for CPContactMessageButton {}
impl std::convert::TryFrom<CPButton> for CPContactMessageButton {
    type Error = &'static str;
    fn try_from(parent: CPButton) -> Result<CPContactMessageButton, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CPContactMessageButton").unwrap()) };
        if is_kind_of {
            Ok(CPContactMessageButton(parent.0))
        } else {
            Err("This CPButton cannot be downcasted to CPContactMessageButton")
        }
    }
}
impl INSObject for CPContactMessageButton {}
impl PNSObject for CPContactMessageButton {}
impl ICPContactMessageButton for CPContactMessageButton {}
pub trait ICPContactMessageButton: Sized + std::ops::Deref {
    unsafe fn initWithPhoneOrEmail_(&self, phoneOrEmail: NSString) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithPhoneOrEmail : phoneOrEmail)
    }
    unsafe fn phoneOrEmail(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, phoneOrEmail)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CPContactDirectionsButton(pub id);
impl std::ops::Deref for CPContactDirectionsButton {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CPContactDirectionsButton {}
impl CPContactDirectionsButton {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CPContactDirectionsButton").unwrap(), alloc) })
    }
}
impl ICPButton for CPContactDirectionsButton {}
impl std::convert::TryFrom<CPButton> for CPContactDirectionsButton {
    type Error = &'static str;
    fn try_from(parent: CPButton) -> Result<CPContactDirectionsButton, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CPContactDirectionsButton").unwrap()) };
        if is_kind_of {
            Ok(CPContactDirectionsButton(parent.0))
        } else {
            Err("This CPButton cannot be downcasted to CPContactDirectionsButton")
        }
    }
}
impl INSObject for CPContactDirectionsButton {}
impl PNSObject for CPContactDirectionsButton {}
impl ICPContactDirectionsButton for CPContactDirectionsButton {}
pub trait ICPContactDirectionsButton: Sized + std::ops::Deref {
    unsafe fn initWithHandler_(&self, handler: *mut ::std::os::raw::c_void) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithHandler : handler)
    }
    unsafe fn initWithImage_handler_(
        &self,
        image: UIImage,
        handler: *mut ::std::os::raw::c_void,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithImage : image, handler : handler)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CPContactTemplate(pub id);
impl std::ops::Deref for CPContactTemplate {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CPContactTemplate {}
impl CPContactTemplate {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CPContactTemplate").unwrap(), alloc) })
    }
}
impl PCPBarButtonProviding for CPContactTemplate {}
impl ICPTemplate for CPContactTemplate {}
impl PNSSecureCoding for CPContactTemplate {}
impl std::convert::TryFrom<CPTemplate> for CPContactTemplate {
    type Error = &'static str;
    fn try_from(parent: CPTemplate) -> Result<CPContactTemplate, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CPContactTemplate").unwrap()) };
        if is_kind_of {
            Ok(CPContactTemplate(parent.0))
        } else {
            Err("This CPTemplate cannot be downcasted to CPContactTemplate")
        }
    }
}
impl INSObject for CPContactTemplate {}
impl PNSObject for CPContactTemplate {}
impl ICPContactTemplate for CPContactTemplate {}
pub trait ICPContactTemplate: Sized + std::ops::Deref {
    unsafe fn initWithContact_(&self, contact: CPContact) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithContact : contact)
    }
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn contact(&self) -> CPContact
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, contact)
    }
    unsafe fn setContact_(&self, contact: CPContact)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setContact : contact)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CPContactTemplate").unwrap(), new)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CPDashboardButton(pub id);
impl std::ops::Deref for CPDashboardButton {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CPDashboardButton {}
impl CPDashboardButton {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CPDashboardButton").unwrap(), alloc) })
    }
}
impl PNSSecureCoding for CPDashboardButton {}
impl INSObject for CPDashboardButton {}
impl PNSObject for CPDashboardButton {}
impl std::convert::TryFrom<NSObject> for CPDashboardButton {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<CPDashboardButton, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CPDashboardButton").unwrap()) };
        if is_kind_of {
            Ok(CPDashboardButton(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to CPDashboardButton")
        }
    }
}
impl ICPDashboardButton for CPDashboardButton {}
pub trait ICPDashboardButton: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn initWithTitleVariants_subtitleVariants_image_handler_(
        &self,
        titleVariants: NSArray,
        subtitleVariants: NSArray,
        image: UIImage,
        handler: *mut ::std::os::raw::c_void,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithTitleVariants : titleVariants, subtitleVariants : subtitleVariants, image : image, handler : handler)
    }
    unsafe fn image(&self) -> UIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, image)
    }
    unsafe fn titleVariants(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, titleVariants)
    }
    unsafe fn subtitleVariants(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, subtitleVariants)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CPDashboardButton").unwrap(), new)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CPDashboardController(pub id);
impl std::ops::Deref for CPDashboardController {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CPDashboardController {}
impl CPDashboardController {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CPDashboardController").unwrap(), alloc) })
    }
}
impl INSObject for CPDashboardController {}
impl PNSObject for CPDashboardController {}
impl std::convert::TryFrom<NSObject> for CPDashboardController {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<CPDashboardController, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CPDashboardController").unwrap()) };
        if is_kind_of {
            Ok(CPDashboardController(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to CPDashboardController")
        }
    }
}
impl ICPDashboardController for CPDashboardController {}
pub trait ICPDashboardController: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn shortcutButtons(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, shortcutButtons)
    }
    unsafe fn setShortcutButtons_(&self, shortcutButtons: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setShortcutButtons : shortcutButtons)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CPDashboardController").unwrap(), new)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CPMessageGridItemConfiguration(pub id);
impl std::ops::Deref for CPMessageGridItemConfiguration {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CPMessageGridItemConfiguration {}
impl CPMessageGridItemConfiguration {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CPMessageGridItemConfiguration").unwrap(), alloc) })
    }
}
impl INSObject for CPMessageGridItemConfiguration {}
impl PNSObject for CPMessageGridItemConfiguration {}
impl std::convert::TryFrom<NSObject> for CPMessageGridItemConfiguration {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<CPMessageGridItemConfiguration, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CPMessageGridItemConfiguration").unwrap())
        };
        if is_kind_of {
            Ok(CPMessageGridItemConfiguration(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to CPMessageGridItemConfiguration")
        }
    }
}
impl ICPMessageGridItemConfiguration for CPMessageGridItemConfiguration {}
pub trait ICPMessageGridItemConfiguration: Sized + std::ops::Deref {
    unsafe fn initWithConversationIdentifier_unread_(
        &self,
        conversationIdentifier: NSString,
        unread: BOOL,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithConversationIdentifier : conversationIdentifier, unread : unread)
    }
    unsafe fn isUnread(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isUnread)
    }
    unsafe fn setUnread_(&self, unread: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setUnread : unread)
    }
    unsafe fn conversationIdentifier(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, conversationIdentifier)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CPGridButton(pub id);
impl std::ops::Deref for CPGridButton {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CPGridButton {}
impl CPGridButton {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CPGridButton").unwrap(), alloc) })
    }
}
impl PNSSecureCoding for CPGridButton {}
impl INSObject for CPGridButton {}
impl PNSObject for CPGridButton {}
impl std::convert::TryFrom<NSObject> for CPGridButton {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<CPGridButton, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CPGridButton").unwrap()) };
        if is_kind_of {
            Ok(CPGridButton(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to CPGridButton")
        }
    }
}
impl ICPGridButton for CPGridButton {}
pub trait ICPGridButton: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn initWithTitleVariants_image_handler_(
        &self,
        titleVariants: NSArray,
        image: UIImage,
        handler: *mut ::std::os::raw::c_void,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithTitleVariants : titleVariants, image : image, handler : handler)
    }
    unsafe fn initWithTitleVariants_image_messageConfiguration_handler_(
        &self,
        titleVariants: NSArray,
        image: UIImage,
        messageConfiguration: CPMessageGridItemConfiguration,
        handler: *mut ::std::os::raw::c_void,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithTitleVariants : titleVariants, image : image, messageConfiguration : messageConfiguration, handler : handler)
    }
    unsafe fn updateImage_(&self, image: UIImage)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, updateImage : image)
    }
    unsafe fn updateTitleVariants_(&self, titleVariants: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, updateTitleVariants : titleVariants)
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
    unsafe fn messageConfiguration(&self) -> CPMessageGridItemConfiguration
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, messageConfiguration)
    }
    unsafe fn image(&self) -> UIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, image)
    }
    unsafe fn titleVariants(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, titleVariants)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CPGridButton").unwrap(), new)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CPGridTemplate(pub id);
impl std::ops::Deref for CPGridTemplate {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CPGridTemplate {}
impl CPGridTemplate {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CPGridTemplate").unwrap(), alloc) })
    }
}
impl PCPBarButtonProviding for CPGridTemplate {}
impl ICPTemplate for CPGridTemplate {}
impl PNSSecureCoding for CPGridTemplate {}
impl std::convert::TryFrom<CPTemplate> for CPGridTemplate {
    type Error = &'static str;
    fn try_from(parent: CPTemplate) -> Result<CPGridTemplate, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CPGridTemplate").unwrap()) };
        if is_kind_of {
            Ok(CPGridTemplate(parent.0))
        } else {
            Err("This CPTemplate cannot be downcasted to CPGridTemplate")
        }
    }
}
impl INSObject for CPGridTemplate {}
impl PNSObject for CPGridTemplate {}
impl ICPGridTemplate for CPGridTemplate {}
pub trait ICPGridTemplate: Sized + std::ops::Deref {
    unsafe fn initWithTitle_gridButtons_(
        &self,
        title: NSString,
        gridButtons: NSArray,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithTitle : title, gridButtons : gridButtons)
    }
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn updateGridButtons_(&self, gridButtons: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, updateGridButtons : gridButtons)
    }
    unsafe fn updateTitle_(&self, title: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, updateTitle : title)
    }
    unsafe fn gridButtons(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, gridButtons)
    }
    unsafe fn title(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, title)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CPGridTemplate").unwrap(), new)
    }
    unsafe fn maximumGridButtonImageSize() -> CGSize
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CPGridTemplate").unwrap(), maximumGridButtonImageSize)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CPImageSet(pub id);
impl std::ops::Deref for CPImageSet {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CPImageSet {}
impl CPImageSet {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CPImageSet").unwrap(), alloc) })
    }
}
impl PNSSecureCoding for CPImageSet {}
impl INSObject for CPImageSet {}
impl PNSObject for CPImageSet {}
impl std::convert::TryFrom<NSObject> for CPImageSet {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<CPImageSet, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CPImageSet").unwrap()) };
        if is_kind_of {
            Ok(CPImageSet(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to CPImageSet")
        }
    }
}
impl ICPImageSet for CPImageSet {}
pub trait ICPImageSet: Sized + std::ops::Deref {
    unsafe fn initWithLightContentImage_darkContentImage_(
        &self,
        lightImage: UIImage,
        darkImage: UIImage,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithLightContentImage : lightImage, darkContentImage : darkImage)
    }
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn lightContentImage(&self) -> UIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, lightContentImage)
    }
    unsafe fn darkContentImage(&self) -> UIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, darkContentImage)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CPImageSet").unwrap(), new)
    }
}
pub type CPTextButtonStyle = NSInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CPTextButton(pub id);
impl std::ops::Deref for CPTextButton {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CPTextButton {}
impl CPTextButton {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CPTextButton").unwrap(), alloc) })
    }
}
impl INSObject for CPTextButton {}
impl PNSObject for CPTextButton {}
impl std::convert::TryFrom<NSObject> for CPTextButton {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<CPTextButton, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CPTextButton").unwrap()) };
        if is_kind_of {
            Ok(CPTextButton(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to CPTextButton")
        }
    }
}
impl ICPTextButton for CPTextButton {}
pub trait ICPTextButton: Sized + std::ops::Deref {
    unsafe fn initWithTitle_textStyle_handler_(
        &self,
        title: NSString,
        textStyle: CPTextButtonStyle,
        handler: *mut ::std::os::raw::c_void,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithTitle : title, textStyle : textStyle, handler : handler)
    }
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
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
    unsafe fn textStyle(&self) -> CPTextButtonStyle
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, textStyle)
    }
    unsafe fn setTextStyle_(&self, textStyle: CPTextButtonStyle)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setTextStyle : textStyle)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CPTextButton").unwrap(), new)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CPInformationItem(pub id);
impl std::ops::Deref for CPInformationItem {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CPInformationItem {}
impl CPInformationItem {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CPInformationItem").unwrap(), alloc) })
    }
}
impl PNSSecureCoding for CPInformationItem {}
impl INSObject for CPInformationItem {}
impl PNSObject for CPInformationItem {}
impl std::convert::TryFrom<NSObject> for CPInformationItem {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<CPInformationItem, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CPInformationItem").unwrap()) };
        if is_kind_of {
            Ok(CPInformationItem(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to CPInformationItem")
        }
    }
}
impl ICPInformationItem for CPInformationItem {}
pub trait ICPInformationItem: Sized + std::ops::Deref {
    unsafe fn initWithTitle_detail_(&self, title: NSString, detail: NSString) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithTitle : title, detail : detail)
    }
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn title(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, title)
    }
    unsafe fn detail(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, detail)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CPInformationItem").unwrap(), new)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CPInformationRatingItem(pub id);
impl std::ops::Deref for CPInformationRatingItem {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CPInformationRatingItem {}
impl CPInformationRatingItem {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CPInformationRatingItem").unwrap(), alloc) })
    }
}
impl ICPInformationItem for CPInformationRatingItem {}
impl PNSSecureCoding for CPInformationRatingItem {}
impl From<CPInformationRatingItem> for CPInformationItem {
    fn from(child: CPInformationRatingItem) -> CPInformationItem {
        CPInformationItem(child.0)
    }
}
impl std::convert::TryFrom<CPInformationItem> for CPInformationRatingItem {
    type Error = &'static str;
    fn try_from(parent: CPInformationItem) -> Result<CPInformationRatingItem, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CPInformationRatingItem").unwrap()) };
        if is_kind_of {
            Ok(CPInformationRatingItem(parent.0))
        } else {
            Err("This CPInformationItem cannot be downcasted to CPInformationRatingItem")
        }
    }
}
impl INSObject for CPInformationRatingItem {}
impl PNSObject for CPInformationRatingItem {}
impl ICPInformationRatingItem for CPInformationRatingItem {}
pub trait ICPInformationRatingItem: Sized + std::ops::Deref {
    unsafe fn initWithRating_maximumRating_title_detail_(
        &self,
        rating: NSNumber,
        maximumRating: NSNumber,
        title: NSString,
        detail: NSString,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithRating : rating, maximumRating : maximumRating, title : title, detail : detail)
    }
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn rating(&self) -> NSNumber
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, rating)
    }
    unsafe fn maximumRating(&self) -> NSNumber
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, maximumRating)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CPInformationRatingItem").unwrap(), new)
    }
}
pub type CPInformationTemplateLayout = NSInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CPInformationTemplate(pub id);
impl std::ops::Deref for CPInformationTemplate {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CPInformationTemplate {}
impl CPInformationTemplate {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CPInformationTemplate").unwrap(), alloc) })
    }
}
impl PCPBarButtonProviding for CPInformationTemplate {}
impl ICPTemplate for CPInformationTemplate {}
impl PNSSecureCoding for CPInformationTemplate {}
impl std::convert::TryFrom<CPTemplate> for CPInformationTemplate {
    type Error = &'static str;
    fn try_from(parent: CPTemplate) -> Result<CPInformationTemplate, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CPInformationTemplate").unwrap()) };
        if is_kind_of {
            Ok(CPInformationTemplate(parent.0))
        } else {
            Err("This CPTemplate cannot be downcasted to CPInformationTemplate")
        }
    }
}
impl INSObject for CPInformationTemplate {}
impl PNSObject for CPInformationTemplate {}
impl ICPInformationTemplate for CPInformationTemplate {}
pub trait ICPInformationTemplate: Sized + std::ops::Deref {
    unsafe fn initWithTitle_layout_items_actions_(
        &self,
        title: NSString,
        layout: CPInformationTemplateLayout,
        items: NSArray,
        actions: NSArray,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithTitle : title, layout : layout, items : items, actions : actions)
    }
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn layout(&self) -> CPInformationTemplateLayout
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, layout)
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
    unsafe fn items(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, items)
    }
    unsafe fn setItems_(&self, items: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setItems : items)
    }
    unsafe fn actions(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, actions)
    }
    unsafe fn setActions_(&self, actions: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setActions : actions)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CPInformationTemplate").unwrap(), new)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CPWindow(pub id);
impl std::ops::Deref for CPWindow {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CPWindow {}
impl CPWindow {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CPWindow").unwrap(), alloc) })
    }
}
impl IUIWindow for CPWindow {}
impl std::convert::TryFrom<UIWindow> for CPWindow {
    type Error = &'static str;
    fn try_from(parent: UIWindow) -> Result<CPWindow, Self::Error> {
        let is_kind_of: bool = unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CPWindow").unwrap()) };
        if is_kind_of {
            Ok(CPWindow(parent.0))
        } else {
            Err("This UIWindow cannot be downcasted to CPWindow")
        }
    }
}
impl IUIView for CPWindow {}
impl PNSCoding for CPWindow {}
impl PUIAppearance for CPWindow {}
impl PUIAppearanceContainer for CPWindow {}
impl PUIDynamicItem for CPWindow {}
impl PUITraitEnvironment for CPWindow {}
impl PUICoordinateSpace for CPWindow {}
impl PUIFocusItem for CPWindow {}
impl PUIFocusItemContainer for CPWindow {}
impl IUIResponder for CPWindow {}
impl PUIResponderStandardEditActions for CPWindow {}
impl INSObject for CPWindow {}
impl PNSObject for CPWindow {}
impl ICPWindow for CPWindow {}
pub trait ICPWindow: Sized + std::ops::Deref {
    unsafe fn mapButtonSafeAreaLayoutGuide(&self) -> UILayoutGuide
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, mapButtonSafeAreaLayoutGuide)
    }
    unsafe fn windowScene(&self) -> UIWindowScene
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, windowScene)
    }
    unsafe fn setWindowScene_(&self, windowScene: UIWindowScene)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setWindowScene : windowScene)
    }
    unsafe fn templateApplicationScene(&self) -> CPTemplateApplicationScene
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, templateApplicationScene)
    }
    unsafe fn setTemplateApplicationScene_(
        &self,
        templateApplicationScene: CPTemplateApplicationScene,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setTemplateApplicationScene : templateApplicationScene)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CPInterfaceController(pub id);
impl std::ops::Deref for CPInterfaceController {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CPInterfaceController {}
impl CPInterfaceController {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CPInterfaceController").unwrap(), alloc) })
    }
}
impl INSObject for CPInterfaceController {}
impl PNSObject for CPInterfaceController {}
impl std::convert::TryFrom<NSObject> for CPInterfaceController {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<CPInterfaceController, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CPInterfaceController").unwrap()) };
        if is_kind_of {
            Ok(CPInterfaceController(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to CPInterfaceController")
        }
    }
}
impl ICPInterfaceController for CPInterfaceController {}
pub trait ICPInterfaceController: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn setRootTemplate_animated_completion_(
        &self,
        rootTemplate: CPTemplate,
        animated: BOOL,
        completion: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRootTemplate : rootTemplate, animated : animated, completion : completion)
    }
    unsafe fn pushTemplate_animated_completion_(
        &self,
        templateToPush: CPTemplate,
        animated: BOOL,
        completion: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, pushTemplate : templateToPush, animated : animated, completion : completion)
    }
    unsafe fn popTemplateAnimated_completion_(
        &self,
        animated: BOOL,
        completion: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, popTemplateAnimated : animated, completion : completion)
    }
    unsafe fn popToRootTemplateAnimated_completion_(
        &self,
        animated: BOOL,
        completion: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, popToRootTemplateAnimated : animated, completion : completion)
    }
    unsafe fn popToTemplate_animated_completion_(
        &self,
        targetTemplate: CPTemplate,
        animated: BOOL,
        completion: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, popToTemplate : targetTemplate, animated : animated, completion : completion)
    }
    unsafe fn presentTemplate_animated_completion_(
        &self,
        templateToPresent: CPTemplate,
        animated: BOOL,
        completion: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, presentTemplate : templateToPresent, animated : animated, completion : completion)
    }
    unsafe fn dismissTemplateAnimated_completion_(
        &self,
        animated: BOOL,
        completion: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, dismissTemplateAnimated : animated, completion : completion)
    }
    unsafe fn setRootTemplate_animated_(&self, rootTemplate: CPTemplate, animated: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRootTemplate : rootTemplate, animated : animated)
    }
    unsafe fn pushTemplate_animated_(&self, templateToPush: CPTemplate, animated: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, pushTemplate : templateToPush, animated : animated)
    }
    unsafe fn popTemplateAnimated_(&self, animated: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, popTemplateAnimated : animated)
    }
    unsafe fn popToRootTemplateAnimated_(&self, animated: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, popToRootTemplateAnimated : animated)
    }
    unsafe fn popToTemplate_animated_(&self, targetTemplate: CPTemplate, animated: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, popToTemplate : targetTemplate, animated : animated)
    }
    unsafe fn presentTemplate_animated_(&self, templateToPresent: CPTemplate, animated: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, presentTemplate : templateToPresent, animated : animated)
    }
    unsafe fn dismissTemplateAnimated_(&self, animated: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, dismissTemplateAnimated : animated)
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
    unsafe fn prefersDarkUserInterfaceStyle(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, prefersDarkUserInterfaceStyle)
    }
    unsafe fn setPrefersDarkUserInterfaceStyle_(&self, prefersDarkUserInterfaceStyle: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPrefersDarkUserInterfaceStyle : prefersDarkUserInterfaceStyle)
    }
    unsafe fn presentedTemplate(&self) -> CPTemplate
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, presentedTemplate)
    }
    unsafe fn rootTemplate(&self) -> CPTemplate
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, rootTemplate)
    }
    unsafe fn topTemplate(&self) -> CPTemplate
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, topTemplate)
    }
    unsafe fn templates(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, templates)
    }
    unsafe fn carTraitCollection(&self) -> UITraitCollection
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, carTraitCollection)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CPInterfaceController").unwrap(), new)
    }
}
pub trait PCPInterfaceControllerDelegate: Sized + std::ops::Deref {
    unsafe fn templateWillAppear_animated_(&self, aTemplate: CPTemplate, animated: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, templateWillAppear : aTemplate, animated : animated)
    }
    unsafe fn templateDidAppear_animated_(&self, aTemplate: CPTemplate, animated: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, templateDidAppear : aTemplate, animated : animated)
    }
    unsafe fn templateWillDisappear_animated_(&self, aTemplate: CPTemplate, animated: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, templateWillDisappear : aTemplate, animated : animated)
    }
    unsafe fn templateDidDisappear_animated_(&self, aTemplate: CPTemplate, animated: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, templateDidDisappear : aTemplate, animated : animated)
    }
}
pub trait PCPApplicationDelegate: Sized + std::ops::Deref {
    unsafe fn application_didConnectCarInterfaceController_toWindow_(
        &self,
        application: UIApplication,
        interfaceController: CPInterfaceController,
        window: CPWindow,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, application : application, didConnectCarInterfaceController : interfaceController, toWindow : window)
    }
    unsafe fn application_didDisconnectCarInterfaceController_fromWindow_(
        &self,
        application: UIApplication,
        interfaceController: CPInterfaceController,
        window: CPWindow,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, application : application, didDisconnectCarInterfaceController : interfaceController, fromWindow : window)
    }
    unsafe fn application_didSelectNavigationAlert_(
        &self,
        application: UIApplication,
        navigationAlert: CPNavigationAlert,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, application : application, didSelectNavigationAlert : navigationAlert)
    }
    unsafe fn application_didSelectManeuver_(
        &self,
        application: UIApplication,
        maneuver: CPManeuver,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, application : application, didSelectManeuver : maneuver)
    }
}
pub type CPInstrumentClusterSetting = NSUInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CPInstrumentClusterController(pub id);
impl std::ops::Deref for CPInstrumentClusterController {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CPInstrumentClusterController {}
impl CPInstrumentClusterController {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CPInstrumentClusterController").unwrap(), alloc) })
    }
}
impl INSObject for CPInstrumentClusterController {}
impl PNSObject for CPInstrumentClusterController {}
impl std::convert::TryFrom<NSObject> for CPInstrumentClusterController {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<CPInstrumentClusterController, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CPInstrumentClusterController").unwrap())
        };
        if is_kind_of {
            Ok(CPInstrumentClusterController(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to CPInstrumentClusterController")
        }
    }
}
impl ICPInstrumentClusterController for CPInstrumentClusterController {}
pub trait ICPInstrumentClusterController: Sized + std::ops::Deref {
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
    unsafe fn instrumentClusterWindow(&self) -> UIWindow
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, instrumentClusterWindow)
    }
    unsafe fn speedLimitSetting(&self) -> CPInstrumentClusterSetting
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, speedLimitSetting)
    }
    unsafe fn compassSetting(&self) -> CPInstrumentClusterSetting
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, compassSetting)
    }
    unsafe fn inactiveDescriptionVariants(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inactiveDescriptionVariants)
    }
    unsafe fn setInactiveDescriptionVariants_(&self, inactiveDescriptionVariants: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInactiveDescriptionVariants : inactiveDescriptionVariants)
    }
    unsafe fn attributedInactiveDescriptionVariants(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, attributedInactiveDescriptionVariants)
    }
    unsafe fn setAttributedInactiveDescriptionVariants_(
        &self,
        attributedInactiveDescriptionVariants: NSArray,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAttributedInactiveDescriptionVariants : attributedInactiveDescriptionVariants)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CPInstrumentClusterController").unwrap(), new)
    }
}
pub trait PCPInstrumentClusterControllerDelegate: Sized + std::ops::Deref {
    unsafe fn instrumentClusterControllerDidConnectWindow_(&self, instrumentClusterWindow: UIWindow)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, instrumentClusterControllerDidConnectWindow : instrumentClusterWindow)
    }
    unsafe fn instrumentClusterControllerDidDisconnectWindow_(
        &self,
        instrumentClusterWindow: UIWindow,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, instrumentClusterControllerDidDisconnectWindow : instrumentClusterWindow)
    }
    unsafe fn instrumentClusterControllerDidZoomIn_(
        &self,
        instrumentClusterController: CPInstrumentClusterController,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, instrumentClusterControllerDidZoomIn : instrumentClusterController)
    }
    unsafe fn instrumentClusterControllerDidZoomOut_(
        &self,
        instrumentClusterController: CPInstrumentClusterController,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, instrumentClusterControllerDidZoomOut : instrumentClusterController)
    }
    unsafe fn instrumentClusterController_didChangeCompassSetting_(
        &self,
        instrumentClusterController: CPInstrumentClusterController,
        compassSetting: CPInstrumentClusterSetting,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, instrumentClusterController : instrumentClusterController, didChangeCompassSetting : compassSetting)
    }
    unsafe fn instrumentClusterController_didChangeSpeedLimitSetting_(
        &self,
        instrumentClusterController: CPInstrumentClusterController,
        speedLimitSetting: CPInstrumentClusterSetting,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, instrumentClusterController : instrumentClusterController, didChangeSpeedLimitSetting : speedLimitSetting)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CPListImageRowItemElement(pub id);
impl std::ops::Deref for CPListImageRowItemElement {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CPListImageRowItemElement {}
impl CPListImageRowItemElement {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CPListImageRowItemElement").unwrap(), alloc) })
    }
}
impl INSObject for CPListImageRowItemElement {}
impl PNSObject for CPListImageRowItemElement {}
impl std::convert::TryFrom<NSObject> for CPListImageRowItemElement {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<CPListImageRowItemElement, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CPListImageRowItemElement").unwrap()) };
        if is_kind_of {
            Ok(CPListImageRowItemElement(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to CPListImageRowItemElement")
        }
    }
}
impl ICPListImageRowItemElement for CPListImageRowItemElement {}
pub trait ICPListImageRowItemElement: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn image(&self) -> UIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, image)
    }
    unsafe fn setImage_(&self, image: UIImage)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setImage : image)
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
    unsafe fn maximumImageSize() -> CGSize
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CPListImageRowItemElement").unwrap(), maximumImageSize)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CPListImageRowItemCardElement(pub id);
impl std::ops::Deref for CPListImageRowItemCardElement {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CPListImageRowItemCardElement {}
impl CPListImageRowItemCardElement {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CPListImageRowItemCardElement").unwrap(), alloc) })
    }
}
impl ICPListImageRowItemElement for CPListImageRowItemCardElement {}
impl From<CPListImageRowItemCardElement> for CPListImageRowItemElement {
    fn from(child: CPListImageRowItemCardElement) -> CPListImageRowItemElement {
        CPListImageRowItemElement(child.0)
    }
}
impl std::convert::TryFrom<CPListImageRowItemElement> for CPListImageRowItemCardElement {
    type Error = &'static str;
    fn try_from(
        parent: CPListImageRowItemElement,
    ) -> Result<CPListImageRowItemCardElement, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CPListImageRowItemCardElement").unwrap())
        };
        if is_kind_of {
            Ok(CPListImageRowItemCardElement(parent.0))
        } else {
            Err ("This CPListImageRowItemElement cannot be downcasted to CPListImageRowItemCardElement" ,)
        }
    }
}
impl INSObject for CPListImageRowItemCardElement {}
impl PNSObject for CPListImageRowItemCardElement {}
impl ICPListImageRowItemCardElement for CPListImageRowItemCardElement {}
pub trait ICPListImageRowItemCardElement: Sized + std::ops::Deref {
    unsafe fn initWithImage_showsImageFullHeight_title_subtitle_tintColor_(
        &self,
        image: UIImage,
        showsImageFullHeight: BOOL,
        title: NSString,
        subtitle: NSString,
        tintColor: UIColor,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithImage : image, showsImageFullHeight : showsImageFullHeight, title : title, subtitle : subtitle, tintColor : tintColor)
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
    unsafe fn subtitle(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, subtitle)
    }
    unsafe fn setSubtitle_(&self, subtitle: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSubtitle : subtitle)
    }
    unsafe fn showsImageFullHeight(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, showsImageFullHeight)
    }
    unsafe fn tintColor(&self) -> UIColor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, tintColor)
    }
    unsafe fn setTintColor_(&self, tintColor: UIColor)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setTintColor : tintColor)
    }
    unsafe fn maximumImageSize() -> CGSize
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CPListImageRowItemCardElement").unwrap(), maximumImageSize)
    }
    unsafe fn maximumFullHeightImageSize() -> CGSize
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CPListImageRowItemCardElement").unwrap(), maximumFullHeightImageSize)
    }
}
pub type CPListImageRowItemCondensedElementShape = NSInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CPListImageRowItemCondensedElement(pub id);
impl std::ops::Deref for CPListImageRowItemCondensedElement {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CPListImageRowItemCondensedElement {}
impl CPListImageRowItemCondensedElement {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CPListImageRowItemCondensedElement").unwrap(), alloc) })
    }
}
impl ICPListImageRowItemElement for CPListImageRowItemCondensedElement {}
impl std::convert::TryFrom<CPListImageRowItemElement> for CPListImageRowItemCondensedElement {
    type Error = &'static str;
    fn try_from(
        parent: CPListImageRowItemElement,
    ) -> Result<CPListImageRowItemCondensedElement, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CPListImageRowItemCondensedElement").unwrap())
        };
        if is_kind_of {
            Ok(CPListImageRowItemCondensedElement(parent.0))
        } else {
            Err ("This CPListImageRowItemElement cannot be downcasted to CPListImageRowItemCondensedElement" ,)
        }
    }
}
impl INSObject for CPListImageRowItemCondensedElement {}
impl PNSObject for CPListImageRowItemCondensedElement {}
impl ICPListImageRowItemCondensedElement for CPListImageRowItemCondensedElement {}
pub trait ICPListImageRowItemCondensedElement: Sized + std::ops::Deref {
    unsafe fn initWithImage_imageShape_title_subtitle_accessorySymbolName_(
        &self,
        image: UIImage,
        imageShape: CPListImageRowItemCondensedElementShape,
        title: NSString,
        subtitle: NSString,
        accessorySymbolName: NSString,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithImage : image, imageShape : imageShape, title : title, subtitle : subtitle, accessorySymbolName : accessorySymbolName)
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
    unsafe fn subtitle(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, subtitle)
    }
    unsafe fn setSubtitle_(&self, subtitle: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSubtitle : subtitle)
    }
    unsafe fn accessorySymbolName(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, accessorySymbolName)
    }
    unsafe fn setAccessorySymbolName_(&self, accessorySymbolName: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAccessorySymbolName : accessorySymbolName)
    }
    unsafe fn imageShape(&self) -> CPListImageRowItemCondensedElementShape
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, imageShape)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CPListImageRowItemGridElement(pub id);
impl std::ops::Deref for CPListImageRowItemGridElement {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CPListImageRowItemGridElement {}
impl CPListImageRowItemGridElement {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CPListImageRowItemGridElement").unwrap(), alloc) })
    }
}
impl ICPListImageRowItemElement for CPListImageRowItemGridElement {}
impl std::convert::TryFrom<CPListImageRowItemElement> for CPListImageRowItemGridElement {
    type Error = &'static str;
    fn try_from(
        parent: CPListImageRowItemElement,
    ) -> Result<CPListImageRowItemGridElement, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CPListImageRowItemGridElement").unwrap())
        };
        if is_kind_of {
            Ok(CPListImageRowItemGridElement(parent.0))
        } else {
            Err ("This CPListImageRowItemElement cannot be downcasted to CPListImageRowItemGridElement" ,)
        }
    }
}
impl INSObject for CPListImageRowItemGridElement {}
impl PNSObject for CPListImageRowItemGridElement {}
impl ICPListImageRowItemGridElement for CPListImageRowItemGridElement {}
pub trait ICPListImageRowItemGridElement: Sized + std::ops::Deref {
    unsafe fn initWithImage_(&self, image: UIImage) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithImage : image)
    }
}
pub type CPListImageRowItemImageGridElementShape = NSInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CPListImageRowItemImageGridElement(pub id);
impl std::ops::Deref for CPListImageRowItemImageGridElement {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CPListImageRowItemImageGridElement {}
impl CPListImageRowItemImageGridElement {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CPListImageRowItemImageGridElement").unwrap(), alloc) })
    }
}
impl ICPListImageRowItemElement for CPListImageRowItemImageGridElement {}
impl std::convert::TryFrom<CPListImageRowItemElement> for CPListImageRowItemImageGridElement {
    type Error = &'static str;
    fn try_from(
        parent: CPListImageRowItemElement,
    ) -> Result<CPListImageRowItemImageGridElement, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CPListImageRowItemImageGridElement").unwrap())
        };
        if is_kind_of {
            Ok(CPListImageRowItemImageGridElement(parent.0))
        } else {
            Err ("This CPListImageRowItemElement cannot be downcasted to CPListImageRowItemImageGridElement" ,)
        }
    }
}
impl INSObject for CPListImageRowItemImageGridElement {}
impl PNSObject for CPListImageRowItemImageGridElement {}
impl ICPListImageRowItemImageGridElement for CPListImageRowItemImageGridElement {}
pub trait ICPListImageRowItemImageGridElement: Sized + std::ops::Deref {
    unsafe fn initWithImage_imageShape_title_accessorySymbolName_(
        &self,
        image: UIImage,
        imageShape: CPListImageRowItemImageGridElementShape,
        title: NSString,
        accessorySymbolName: NSString,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithImage : image, imageShape : imageShape, title : title, accessorySymbolName : accessorySymbolName)
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
    unsafe fn accessorySymbolName(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, accessorySymbolName)
    }
    unsafe fn setAccessorySymbolName_(&self, accessorySymbolName: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAccessorySymbolName : accessorySymbolName)
    }
    unsafe fn imageShape(&self) -> CPListImageRowItemImageGridElementShape
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, imageShape)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CPListImageRowItemRowElement(pub id);
impl std::ops::Deref for CPListImageRowItemRowElement {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CPListImageRowItemRowElement {}
impl CPListImageRowItemRowElement {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CPListImageRowItemRowElement").unwrap(), alloc) })
    }
}
impl ICPListImageRowItemElement for CPListImageRowItemRowElement {}
impl std::convert::TryFrom<CPListImageRowItemElement> for CPListImageRowItemRowElement {
    type Error = &'static str;
    fn try_from(
        parent: CPListImageRowItemElement,
    ) -> Result<CPListImageRowItemRowElement, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CPListImageRowItemRowElement").unwrap()) };
        if is_kind_of {
            Ok(CPListImageRowItemRowElement(parent.0))
        } else {
            Err ("This CPListImageRowItemElement cannot be downcasted to CPListImageRowItemRowElement" ,)
        }
    }
}
impl INSObject for CPListImageRowItemRowElement {}
impl PNSObject for CPListImageRowItemRowElement {}
impl ICPListImageRowItemRowElement for CPListImageRowItemRowElement {}
pub trait ICPListImageRowItemRowElement: Sized + std::ops::Deref {
    unsafe fn initWithImage_title_subtitle_(
        &self,
        image: UIImage,
        title: NSString,
        subtitle: NSString,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithImage : image, title : title, subtitle : subtitle)
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
    unsafe fn subtitle(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, subtitle)
    }
    unsafe fn setSubtitle_(&self, subtitle: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSubtitle : subtitle)
    }
}
pub trait PCPListTemplateItem: Sized + std::ops::Deref {
    unsafe fn text(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, text)
    }
    unsafe fn userInfo(&self) -> id
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, userInfo)
    }
    unsafe fn setUserInfo_(&self, userInfo: id)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setUserInfo : userInfo)
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
}
pub trait PCPSelectableListItem: Sized + std::ops::Deref {
    unsafe fn handler(&self) -> *mut ::std::os::raw::c_void
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, handler)
    }
    unsafe fn setHandler_(&self, handler: *mut ::std::os::raw::c_void)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setHandler : handler)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CPListSection(pub id);
impl std::ops::Deref for CPListSection {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CPListSection {}
impl CPListSection {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CPListSection").unwrap(), alloc) })
    }
}
impl PNSSecureCoding for CPListSection {}
impl INSObject for CPListSection {}
impl PNSObject for CPListSection {}
impl std::convert::TryFrom<NSObject> for CPListSection {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<CPListSection, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CPListSection").unwrap()) };
        if is_kind_of {
            Ok(CPListSection(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to CPListSection")
        }
    }
}
impl ICPListSection for CPListSection {}
pub trait ICPListSection: Sized + std::ops::Deref {
    unsafe fn initWithItems_header_sectionIndexTitle_(
        &self,
        items: NSArray,
        header: NSString,
        sectionIndexTitle: NSString,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithItems : items, header : header, sectionIndexTitle : sectionIndexTitle)
    }
    unsafe fn initWithItems_header_headerSubtitle_headerImage_headerButton_sectionIndexTitle_(
        &self,
        items: NSArray,
        header: NSString,
        headerSubtitle: NSString,
        headerImage: UIImage,
        headerButton: CPButton,
        sectionIndexTitle: NSString,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithItems : items, header : header, headerSubtitle : headerSubtitle, headerImage : headerImage, headerButton : headerButton, sectionIndexTitle : sectionIndexTitle)
    }
    unsafe fn initWithItems_(&self, items: NSArray) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithItems : items)
    }
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn indexOfItem_(&self, item: *mut u64) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, indexOfItem : item)
    }
    unsafe fn itemAtIndex_(&self, index: NSUInteger) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, itemAtIndex : index)
    }
    unsafe fn header(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, header)
    }
    unsafe fn headerSubtitle(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, headerSubtitle)
    }
    unsafe fn headerImage(&self) -> UIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, headerImage)
    }
    unsafe fn setHeaderImage_(&self, headerImage: UIImage)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setHeaderImage : headerImage)
    }
    unsafe fn headerButton(&self) -> CPButton
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, headerButton)
    }
    unsafe fn sectionIndexTitle(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, sectionIndexTitle)
    }
    unsafe fn items(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, items)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CPListSection").unwrap(), new)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CPListImageRowItem(pub id);
impl std::ops::Deref for CPListImageRowItem {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CPListImageRowItem {}
impl CPListImageRowItem {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CPListImageRowItem").unwrap(), alloc) })
    }
}
impl PCPSelectableListItem for CPListImageRowItem {}
impl INSObject for CPListImageRowItem {}
impl PNSObject for CPListImageRowItem {}
impl std::convert::TryFrom<NSObject> for CPListImageRowItem {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<CPListImageRowItem, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CPListImageRowItem").unwrap()) };
        if is_kind_of {
            Ok(CPListImageRowItem(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to CPListImageRowItem")
        }
    }
}
impl ICPListImageRowItem for CPListImageRowItem {}
pub trait ICPListImageRowItem: Sized + std::ops::Deref {
    unsafe fn initWithText_images_(&self, text: NSString, images: NSArray) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithText : text, images : images)
    }
    unsafe fn initWithText_images_imageTitles_(
        &self,
        text: NSString,
        images: NSArray,
        imageTitles: NSArray,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithText : text, images : images, imageTitles : imageTitles)
    }
    unsafe fn initWithText_elements_allowsMultipleLines_(
        &self,
        text: NSString,
        elements: NSArray,
        allowsMultipleLines: BOOL,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithText : text, elements : elements, allowsMultipleLines : allowsMultipleLines)
    }
    unsafe fn initWithText_cardElements_allowsMultipleLines_(
        &self,
        text: NSString,
        elements: NSArray,
        allowsMultipleLines: BOOL,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithText : text, cardElements : elements, allowsMultipleLines : allowsMultipleLines)
    }
    unsafe fn initWithText_condensedElements_allowsMultipleLines_(
        &self,
        text: NSString,
        elements: NSArray,
        allowsMultipleLines: BOOL,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithText : text, condensedElements : elements, allowsMultipleLines : allowsMultipleLines)
    }
    unsafe fn initWithText_gridElements_allowsMultipleLines_(
        &self,
        text: NSString,
        elements: NSArray,
        allowsMultipleLines: BOOL,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithText : text, gridElements : elements, allowsMultipleLines : allowsMultipleLines)
    }
    unsafe fn initWithText_imageGridElements_allowsMultipleLines_(
        &self,
        text: NSString,
        elements: NSArray,
        allowsMultipleLines: BOOL,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithText : text, imageGridElements : elements, allowsMultipleLines : allowsMultipleLines)
    }
    unsafe fn updateImages_(&self, gridImages: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, updateImages : gridImages)
    }
    unsafe fn text(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, text)
    }
    unsafe fn setText_(&self, text: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setText : text)
    }
    unsafe fn userInfo(&self) -> id
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, userInfo)
    }
    unsafe fn setUserInfo_(&self, userInfo: id)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setUserInfo : userInfo)
    }
    unsafe fn handler(&self) -> *mut ::std::os::raw::c_void
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, handler)
    }
    unsafe fn setHandler_(&self, handler: *mut ::std::os::raw::c_void)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setHandler : handler)
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
    unsafe fn gridImages(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, gridImages)
    }
    unsafe fn imageTitles(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, imageTitles)
    }
    unsafe fn elements(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, elements)
    }
    unsafe fn setElements_(&self, elements: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setElements : elements)
    }
    unsafe fn allowsMultipleLines(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, allowsMultipleLines)
    }
    unsafe fn listImageRowHandler(&self) -> *mut ::std::os::raw::c_void
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, listImageRowHandler)
    }
    unsafe fn setListImageRowHandler_(&self, listImageRowHandler: *mut ::std::os::raw::c_void)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setListImageRowHandler : listImageRowHandler)
    }
    unsafe fn maximumImageSize() -> CGSize
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CPListImageRowItem").unwrap(), maximumImageSize)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CPListItem(pub id);
impl std::ops::Deref for CPListItem {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CPListItem {}
impl CPListItem {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CPListItem").unwrap(), alloc) })
    }
}
impl PCPSelectableListItem for CPListItem {}
impl INSObject for CPListItem {}
impl PNSObject for CPListItem {}
impl std::convert::TryFrom<NSObject> for CPListItem {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<CPListItem, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CPListItem").unwrap()) };
        if is_kind_of {
            Ok(CPListItem(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to CPListItem")
        }
    }
}
impl ICPListItem for CPListItem {}
pub trait ICPListItem: Sized + std::ops::Deref {
    unsafe fn initWithText_detailText_image_accessoryImage_accessoryType_(
        &self,
        text: NSString,
        detailText: NSString,
        image: UIImage,
        accessoryImage: UIImage,
        accessoryType: CPListItemAccessoryType,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithText : text, detailText : detailText, image : image, accessoryImage : accessoryImage, accessoryType : accessoryType)
    }
    unsafe fn initWithText_detailText_image_(
        &self,
        text: NSString,
        detailText: NSString,
        image: UIImage,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithText : text, detailText : detailText, image : image)
    }
    unsafe fn initWithText_detailText_(&self, text: NSString, detailText: NSString) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithText : text, detailText : detailText)
    }
    unsafe fn setDetailText_(&self, detailText: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDetailText : detailText)
    }
    unsafe fn setImage_(&self, image: UIImage)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setImage : image)
    }
    unsafe fn setAccessoryImage_(&self, accessoryImage: UIImage)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAccessoryImage : accessoryImage)
    }
    unsafe fn setText_(&self, text: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setText : text)
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
    unsafe fn accessoryType(&self) -> CPListItemAccessoryType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, accessoryType)
    }
    unsafe fn setAccessoryType_(&self, accessoryType: CPListItemAccessoryType)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAccessoryType : accessoryType)
    }
    unsafe fn isExplicitContent(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isExplicitContent)
    }
    unsafe fn setExplicitContent_(&self, explicitContent: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setExplicitContent : explicitContent)
    }
    unsafe fn playbackProgress(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, playbackProgress)
    }
    unsafe fn setPlaybackProgress_(&self, playbackProgress: CGFloat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPlaybackProgress : playbackProgress)
    }
    unsafe fn isPlaying(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isPlaying)
    }
    unsafe fn setPlaying_(&self, playing: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPlaying : playing)
    }
    unsafe fn playingIndicatorLocation(&self) -> CPListItemPlayingIndicatorLocation
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, playingIndicatorLocation)
    }
    unsafe fn setPlayingIndicatorLocation_(
        &self,
        playingIndicatorLocation: CPListItemPlayingIndicatorLocation,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPlayingIndicatorLocation : playingIndicatorLocation)
    }
    unsafe fn handler(&self) -> *mut ::std::os::raw::c_void
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, handler)
    }
    unsafe fn setHandler_(&self, handler: *mut ::std::os::raw::c_void)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setHandler : handler)
    }
    unsafe fn text(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, text)
    }
    unsafe fn userInfo(&self) -> id
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, userInfo)
    }
    unsafe fn setUserInfo_(&self, userInfo: id)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setUserInfo : userInfo)
    }
    unsafe fn detailText(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, detailText)
    }
    unsafe fn image(&self) -> UIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, image)
    }
    unsafe fn accessoryImage(&self) -> UIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, accessoryImage)
    }
    unsafe fn showsDisclosureIndicator(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, showsDisclosureIndicator)
    }
    unsafe fn showsExplicitLabel(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, showsExplicitLabel)
    }
    unsafe fn setShowsExplicitLabel_(&self, showsExplicitLabel: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setShowsExplicitLabel : showsExplicitLabel)
    }
    unsafe fn maximumImageSize() -> CGSize
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CPListItem").unwrap(), maximumImageSize)
    }
}
pub type CPListItemAccessoryType = NSInteger;
pub type CPListItemPlayingIndicatorLocation = NSInteger;
impl CPListItem_Deprecated for CPListItem {}
pub trait CPListItem_Deprecated: Sized + std::ops::Deref {
    unsafe fn initWithText_detailText_image_showsDisclosureIndicator_(
        &self,
        text: NSString,
        detailText: NSString,
        image: UIImage,
        showsDisclosureIndicator: BOOL,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithText : text, detailText : detailText, image : image, showsDisclosureIndicator : showsDisclosureIndicator)
    }
}
pub type CPAssistantCellActionType = NSInteger;
pub type CPAssistantCellVisibility = NSInteger;
pub type CPAssistantCellPosition = NSInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CPAssistantCellConfiguration(pub id);
impl std::ops::Deref for CPAssistantCellConfiguration {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CPAssistantCellConfiguration {}
impl CPAssistantCellConfiguration {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CPAssistantCellConfiguration").unwrap(), alloc) })
    }
}
impl PNSSecureCoding for CPAssistantCellConfiguration {}
impl INSObject for CPAssistantCellConfiguration {}
impl PNSObject for CPAssistantCellConfiguration {}
impl std::convert::TryFrom<NSObject> for CPAssistantCellConfiguration {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<CPAssistantCellConfiguration, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CPAssistantCellConfiguration").unwrap()) };
        if is_kind_of {
            Ok(CPAssistantCellConfiguration(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to CPAssistantCellConfiguration")
        }
    }
}
impl ICPAssistantCellConfiguration for CPAssistantCellConfiguration {}
pub trait ICPAssistantCellConfiguration: Sized + std::ops::Deref {
    unsafe fn initWithPosition_visibility_assistantAction_(
        &self,
        position: CPAssistantCellPosition,
        visibility: CPAssistantCellVisibility,
        assistantAction: CPAssistantCellActionType,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithPosition : position, visibility : visibility, assistantAction : assistantAction)
    }
    unsafe fn position(&self) -> CPAssistantCellPosition
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, position)
    }
    unsafe fn visibility(&self) -> CPAssistantCellVisibility
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, visibility)
    }
    unsafe fn assistantAction(&self) -> CPAssistantCellActionType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, assistantAction)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CPListTemplate(pub id);
impl std::ops::Deref for CPListTemplate {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CPListTemplate {}
impl CPListTemplate {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CPListTemplate").unwrap(), alloc) })
    }
}
impl PCPBarButtonProviding for CPListTemplate {}
impl ICPTemplate for CPListTemplate {}
impl PNSSecureCoding for CPListTemplate {}
impl std::convert::TryFrom<CPTemplate> for CPListTemplate {
    type Error = &'static str;
    fn try_from(parent: CPTemplate) -> Result<CPListTemplate, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CPListTemplate").unwrap()) };
        if is_kind_of {
            Ok(CPListTemplate(parent.0))
        } else {
            Err("This CPTemplate cannot be downcasted to CPListTemplate")
        }
    }
}
impl INSObject for CPListTemplate {}
impl PNSObject for CPListTemplate {}
impl ICPListTemplate for CPListTemplate {}
pub trait ICPListTemplate: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn initWithTitle_sections_(&self, title: NSString, sections: NSArray) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithTitle : title, sections : sections)
    }
    unsafe fn initWithTitle_sections_assistantCellConfiguration_(
        &self,
        title: NSString,
        sections: NSArray,
        assistantCellConfiguration: CPAssistantCellConfiguration,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithTitle : title, sections : sections, assistantCellConfiguration : assistantCellConfiguration)
    }
    unsafe fn initWithTitle_sections_assistantCellConfiguration_headerGridButtons_(
        &self,
        title: NSString,
        sections: NSArray,
        assistantCellConfiguration: CPAssistantCellConfiguration,
        headerGridButtons: NSArray,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithTitle : title, sections : sections, assistantCellConfiguration : assistantCellConfiguration, headerGridButtons : headerGridButtons)
    }
    unsafe fn updateSections_(&self, sections: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, updateSections : sections)
    }
    unsafe fn indexPathForItem_(&self, item: *mut u64) -> NSIndexPath
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, indexPathForItem : item)
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
    unsafe fn sections(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, sections)
    }
    unsafe fn title(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, title)
    }
    unsafe fn sectionCount(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, sectionCount)
    }
    unsafe fn itemCount(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, itemCount)
    }
    unsafe fn emptyViewTitleVariants(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, emptyViewTitleVariants)
    }
    unsafe fn setEmptyViewTitleVariants_(&self, emptyViewTitleVariants: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setEmptyViewTitleVariants : emptyViewTitleVariants)
    }
    unsafe fn emptyViewSubtitleVariants(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, emptyViewSubtitleVariants)
    }
    unsafe fn setEmptyViewSubtitleVariants_(&self, emptyViewSubtitleVariants: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setEmptyViewSubtitleVariants : emptyViewSubtitleVariants)
    }
    unsafe fn showsSpinnerWhileEmpty(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, showsSpinnerWhileEmpty)
    }
    unsafe fn setShowsSpinnerWhileEmpty_(&self, showsSpinnerWhileEmpty: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setShowsSpinnerWhileEmpty : showsSpinnerWhileEmpty)
    }
    unsafe fn assistantCellConfiguration(&self) -> CPAssistantCellConfiguration
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, assistantCellConfiguration)
    }
    unsafe fn setAssistantCellConfiguration_(
        &self,
        assistantCellConfiguration: CPAssistantCellConfiguration,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAssistantCellConfiguration : assistantCellConfiguration)
    }
    unsafe fn headerGridButtons(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, headerGridButtons)
    }
    unsafe fn setHeaderGridButtons_(&self, headerGridButtons: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setHeaderGridButtons : headerGridButtons)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CPListTemplate").unwrap(), new)
    }
    unsafe fn maximumItemCount() -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CPListTemplate").unwrap(), maximumItemCount)
    }
    unsafe fn maximumSectionCount() -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CPListTemplate").unwrap(), maximumSectionCount)
    }
    unsafe fn maximumHeaderGridButtonCount() -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CPListTemplate").unwrap(), maximumHeaderGridButtonCount)
    }
    unsafe fn maximumGridButtonImageSize() -> CGSize
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CPListTemplate").unwrap(), maximumGridButtonImageSize)
    }
}
pub trait PCPListTemplateDelegate: Sized + std::ops::Deref {
    unsafe fn listTemplate_didSelectListItem_completionHandler_(
        &self,
        listTemplate: CPListTemplate,
        item: CPListItem,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, listTemplate : listTemplate, didSelectListItem : item, completionHandler : completionHandler)
    }
}
pub type CPLaneStatus = NSInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CPLane(pub id);
impl std::ops::Deref for CPLane {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CPLane {}
impl CPLane {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CPLane").unwrap(), alloc) })
    }
}
impl PNSCopying for CPLane {}
impl PNSSecureCoding for CPLane {}
impl INSObject for CPLane {}
impl PNSObject for CPLane {}
impl std::convert::TryFrom<NSObject> for CPLane {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<CPLane, Self::Error> {
        let is_kind_of: bool = unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CPLane").unwrap()) };
        if is_kind_of {
            Ok(CPLane(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to CPLane")
        }
    }
}
impl ICPLane for CPLane {}
pub trait ICPLane: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn initWithAngles_(&self, angles: NSArray) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithAngles : angles)
    }
    unsafe fn initWithAngles_highlightedAngle_isPreferred_(
        &self,
        angles: NSArray,
        highlightedAngle: NSMeasurement,
        preferred: BOOL,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithAngles : angles, highlightedAngle : highlightedAngle, isPreferred : preferred)
    }
    unsafe fn setStatus_(&self, status: CPLaneStatus)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setStatus : status)
    }
    unsafe fn status(&self) -> CPLaneStatus
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, status)
    }
    unsafe fn primaryAngle(&self) -> NSMeasurement
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, primaryAngle)
    }
    unsafe fn setPrimaryAngle_(&self, primaryAngle: NSMeasurement)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPrimaryAngle : primaryAngle)
    }
    unsafe fn highlightedAngle(&self) -> NSMeasurement
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, highlightedAngle)
    }
    unsafe fn secondaryAngles(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, secondaryAngles)
    }
    unsafe fn setSecondaryAngles_(&self, secondaryAngles: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSecondaryAngles : secondaryAngles)
    }
    unsafe fn angles(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, angles)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CPLaneGuidance(pub id);
impl std::ops::Deref for CPLaneGuidance {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CPLaneGuidance {}
impl CPLaneGuidance {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CPLaneGuidance").unwrap(), alloc) })
    }
}
impl PNSCopying for CPLaneGuidance {}
impl PNSSecureCoding for CPLaneGuidance {}
impl INSObject for CPLaneGuidance {}
impl PNSObject for CPLaneGuidance {}
impl std::convert::TryFrom<NSObject> for CPLaneGuidance {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<CPLaneGuidance, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CPLaneGuidance").unwrap()) };
        if is_kind_of {
            Ok(CPLaneGuidance(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to CPLaneGuidance")
        }
    }
}
impl ICPLaneGuidance for CPLaneGuidance {}
pub trait ICPLaneGuidance: Sized + std::ops::Deref {
    unsafe fn lanes(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, lanes)
    }
    unsafe fn setLanes_(&self, lanes: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setLanes : lanes)
    }
    unsafe fn instructionVariants(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, instructionVariants)
    }
    unsafe fn setInstructionVariants_(&self, instructionVariants: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInstructionVariants : instructionVariants)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CPTravelEstimates(pub id);
impl std::ops::Deref for CPTravelEstimates {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CPTravelEstimates {}
impl CPTravelEstimates {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CPTravelEstimates").unwrap(), alloc) })
    }
}
impl PNSSecureCoding for CPTravelEstimates {}
impl INSObject for CPTravelEstimates {}
impl PNSObject for CPTravelEstimates {}
impl std::convert::TryFrom<NSObject> for CPTravelEstimates {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<CPTravelEstimates, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CPTravelEstimates").unwrap()) };
        if is_kind_of {
            Ok(CPTravelEstimates(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to CPTravelEstimates")
        }
    }
}
impl ICPTravelEstimates for CPTravelEstimates {}
pub trait ICPTravelEstimates: Sized + std::ops::Deref {
    unsafe fn initWithDistanceRemaining_timeRemaining_(
        &self,
        distance: NSMeasurement,
        time: NSTimeInterval,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithDistanceRemaining : distance, timeRemaining : time)
    }
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn initWithDistanceRemaining_distanceRemainingToDisplay_timeRemaining_(
        &self,
        distanceRemaining: NSMeasurement,
        distanceRemainingToDisplay: NSMeasurement,
        time: NSTimeInterval,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithDistanceRemaining : distanceRemaining, distanceRemainingToDisplay : distanceRemainingToDisplay, timeRemaining : time)
    }
    unsafe fn distanceRemainingToDisplay(&self) -> NSMeasurement
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, distanceRemainingToDisplay)
    }
    unsafe fn distanceRemaining(&self) -> NSMeasurement
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, distanceRemaining)
    }
    unsafe fn timeRemaining(&self) -> NSTimeInterval
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, timeRemaining)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CPTravelEstimates").unwrap(), new)
    }
}
pub type CPManeuverType = NSUInteger;
pub type CPJunctionType = NSUInteger;
pub type CPTrafficSide = NSUInteger;
pub type CPManeuverState = NSInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CPManeuver(pub id);
impl std::ops::Deref for CPManeuver {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CPManeuver {}
impl CPManeuver {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CPManeuver").unwrap(), alloc) })
    }
}
impl PNSCopying for CPManeuver {}
impl PNSSecureCoding for CPManeuver {}
impl INSObject for CPManeuver {}
impl PNSObject for CPManeuver {}
impl std::convert::TryFrom<NSObject> for CPManeuver {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<CPManeuver, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CPManeuver").unwrap()) };
        if is_kind_of {
            Ok(CPManeuver(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to CPManeuver")
        }
    }
}
impl ICPManeuver for CPManeuver {}
pub trait ICPManeuver: Sized + std::ops::Deref {
    unsafe fn symbolSet(&self) -> CPImageSet
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, symbolSet)
    }
    unsafe fn setSymbolSet_(&self, symbolSet: CPImageSet)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSymbolSet : symbolSet)
    }
    unsafe fn cardBackgroundColor(&self) -> UIColor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, cardBackgroundColor)
    }
    unsafe fn setCardBackgroundColor_(&self, cardBackgroundColor: UIColor)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCardBackgroundColor : cardBackgroundColor)
    }
    unsafe fn symbolImage(&self) -> UIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, symbolImage)
    }
    unsafe fn setSymbolImage_(&self, symbolImage: UIImage)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSymbolImage : symbolImage)
    }
    unsafe fn junctionImage(&self) -> UIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, junctionImage)
    }
    unsafe fn setJunctionImage_(&self, junctionImage: UIImage)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setJunctionImage : junctionImage)
    }
    unsafe fn instructionVariants(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, instructionVariants)
    }
    unsafe fn setInstructionVariants_(&self, instructionVariants: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInstructionVariants : instructionVariants)
    }
    unsafe fn initialTravelEstimates(&self) -> CPTravelEstimates
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, initialTravelEstimates)
    }
    unsafe fn setInitialTravelEstimates_(&self, initialTravelEstimates: CPTravelEstimates)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInitialTravelEstimates : initialTravelEstimates)
    }
    unsafe fn attributedInstructionVariants(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, attributedInstructionVariants)
    }
    unsafe fn setAttributedInstructionVariants_(&self, attributedInstructionVariants: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAttributedInstructionVariants : attributedInstructionVariants)
    }
    unsafe fn dashboardSymbolImage(&self) -> UIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, dashboardSymbolImage)
    }
    unsafe fn setDashboardSymbolImage_(&self, dashboardSymbolImage: UIImage)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDashboardSymbolImage : dashboardSymbolImage)
    }
    unsafe fn dashboardJunctionImage(&self) -> UIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, dashboardJunctionImage)
    }
    unsafe fn setDashboardJunctionImage_(&self, dashboardJunctionImage: UIImage)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDashboardJunctionImage : dashboardJunctionImage)
    }
    unsafe fn dashboardInstructionVariants(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, dashboardInstructionVariants)
    }
    unsafe fn setDashboardInstructionVariants_(&self, dashboardInstructionVariants: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDashboardInstructionVariants : dashboardInstructionVariants)
    }
    unsafe fn dashboardAttributedInstructionVariants(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, dashboardAttributedInstructionVariants)
    }
    unsafe fn setDashboardAttributedInstructionVariants_(
        &self,
        dashboardAttributedInstructionVariants: NSArray,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDashboardAttributedInstructionVariants : dashboardAttributedInstructionVariants)
    }
    unsafe fn notificationSymbolImage(&self) -> UIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, notificationSymbolImage)
    }
    unsafe fn setNotificationSymbolImage_(&self, notificationSymbolImage: UIImage)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setNotificationSymbolImage : notificationSymbolImage)
    }
    unsafe fn notificationInstructionVariants(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, notificationInstructionVariants)
    }
    unsafe fn setNotificationInstructionVariants_(&self, notificationInstructionVariants: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setNotificationInstructionVariants : notificationInstructionVariants)
    }
    unsafe fn notificationAttributedInstructionVariants(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, notificationAttributedInstructionVariants)
    }
    unsafe fn setNotificationAttributedInstructionVariants_(
        &self,
        notificationAttributedInstructionVariants: NSArray,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setNotificationAttributedInstructionVariants : notificationAttributedInstructionVariants)
    }
    unsafe fn maneuverType(&self) -> CPManeuverType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, maneuverType)
    }
    unsafe fn setManeuverType_(&self, maneuverType: CPManeuverType)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setManeuverType : maneuverType)
    }
    unsafe fn roadFollowingManeuverVariants(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, roadFollowingManeuverVariants)
    }
    unsafe fn setRoadFollowingManeuverVariants_(&self, roadFollowingManeuverVariants: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRoadFollowingManeuverVariants : roadFollowingManeuverVariants)
    }
    unsafe fn trafficSide(&self) -> CPTrafficSide
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, trafficSide)
    }
    unsafe fn setTrafficSide_(&self, trafficSide: CPTrafficSide)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setTrafficSide : trafficSide)
    }
    unsafe fn junctionType(&self) -> CPJunctionType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, junctionType)
    }
    unsafe fn setJunctionType_(&self, junctionType: CPJunctionType)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setJunctionType : junctionType)
    }
    unsafe fn junctionExitAngle(&self) -> NSMeasurement
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, junctionExitAngle)
    }
    unsafe fn setJunctionExitAngle_(&self, junctionExitAngle: NSMeasurement)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setJunctionExitAngle : junctionExitAngle)
    }
    unsafe fn junctionElementAngles(&self) -> NSSet
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, junctionElementAngles)
    }
    unsafe fn setJunctionElementAngles_(&self, junctionElementAngles: NSSet)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setJunctionElementAngles : junctionElementAngles)
    }
    unsafe fn linkedLaneGuidance(&self) -> CPLaneGuidance
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, linkedLaneGuidance)
    }
    unsafe fn setLinkedLaneGuidance_(&self, linkedLaneGuidance: CPLaneGuidance)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setLinkedLaneGuidance : linkedLaneGuidance)
    }
    unsafe fn highwayExitLabel(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, highwayExitLabel)
    }
    unsafe fn setHighwayExitLabel_(&self, highwayExitLabel: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setHighwayExitLabel : highwayExitLabel)
    }
    unsafe fn userInfo(&self) -> id
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, userInfo)
    }
    unsafe fn setUserInfo_(&self, userInfo: id)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setUserInfo : userInfo)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CPMapButton(pub id);
impl std::ops::Deref for CPMapButton {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CPMapButton {}
impl CPMapButton {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CPMapButton").unwrap(), alloc) })
    }
}
impl PNSSecureCoding for CPMapButton {}
impl INSObject for CPMapButton {}
impl PNSObject for CPMapButton {}
impl std::convert::TryFrom<NSObject> for CPMapButton {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<CPMapButton, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CPMapButton").unwrap()) };
        if is_kind_of {
            Ok(CPMapButton(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to CPMapButton")
        }
    }
}
impl ICPMapButton for CPMapButton {}
pub trait ICPMapButton: Sized + std::ops::Deref {
    unsafe fn initWithHandler_(&self, handler: *mut ::std::os::raw::c_void) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithHandler : handler)
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
    unsafe fn image(&self) -> UIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, image)
    }
    unsafe fn setImage_(&self, image: UIImage)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setImage : image)
    }
    unsafe fn focusedImage(&self) -> UIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, focusedImage)
    }
    unsafe fn setFocusedImage_(&self, focusedImage: UIImage)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setFocusedImage : focusedImage)
    }
}
pub type CPNavigationAlertDismissalContext = NSUInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CPNavigationAlert(pub id);
impl std::ops::Deref for CPNavigationAlert {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CPNavigationAlert {}
impl CPNavigationAlert {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CPNavigationAlert").unwrap(), alloc) })
    }
}
impl PNSSecureCoding for CPNavigationAlert {}
impl INSObject for CPNavigationAlert {}
impl PNSObject for CPNavigationAlert {}
impl std::convert::TryFrom<NSObject> for CPNavigationAlert {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<CPNavigationAlert, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CPNavigationAlert").unwrap()) };
        if is_kind_of {
            Ok(CPNavigationAlert(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to CPNavigationAlert")
        }
    }
}
impl ICPNavigationAlert for CPNavigationAlert {}
pub trait ICPNavigationAlert: Sized + std::ops::Deref {
    unsafe fn initWithTitleVariants_subtitleVariants_imageSet_primaryAction_secondaryAction_duration_(
        &self,
        titleVariants: NSArray,
        subtitleVariants: NSArray,
        imageSet: CPImageSet,
        primaryAction: CPAlertAction,
        secondaryAction: CPAlertAction,
        duration: NSTimeInterval,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithTitleVariants : titleVariants, subtitleVariants : subtitleVariants, imageSet : imageSet, primaryAction : primaryAction, secondaryAction : secondaryAction, duration : duration)
    }
    unsafe fn initWithTitleVariants_subtitleVariants_image_primaryAction_secondaryAction_duration_(
        &self,
        titleVariants: NSArray,
        subtitleVariants: NSArray,
        image: UIImage,
        primaryAction: CPAlertAction,
        secondaryAction: CPAlertAction,
        duration: NSTimeInterval,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithTitleVariants : titleVariants, subtitleVariants : subtitleVariants, image : image, primaryAction : primaryAction, secondaryAction : secondaryAction, duration : duration)
    }
    unsafe fn updateTitleVariants_subtitleVariants_(
        &self,
        newTitleVariants: NSArray,
        newSubtitleVariants: NSArray,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, updateTitleVariants : newTitleVariants, subtitleVariants : newSubtitleVariants)
    }
    unsafe fn titleVariants(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, titleVariants)
    }
    unsafe fn subtitleVariants(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, subtitleVariants)
    }
    unsafe fn imageSet(&self) -> CPImageSet
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, imageSet)
    }
    unsafe fn image(&self) -> UIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, image)
    }
    unsafe fn primaryAction(&self) -> CPAlertAction
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, primaryAction)
    }
    unsafe fn secondaryAction(&self) -> CPAlertAction
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, secondaryAction)
    }
    unsafe fn duration(&self) -> NSTimeInterval
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, duration)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CPRouteInformation(pub id);
impl std::ops::Deref for CPRouteInformation {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CPRouteInformation {}
impl CPRouteInformation {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CPRouteInformation").unwrap(), alloc) })
    }
}
impl INSObject for CPRouteInformation {}
impl PNSObject for CPRouteInformation {}
impl std::convert::TryFrom<NSObject> for CPRouteInformation {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<CPRouteInformation, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CPRouteInformation").unwrap()) };
        if is_kind_of {
            Ok(CPRouteInformation(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to CPRouteInformation")
        }
    }
}
impl ICPRouteInformation for CPRouteInformation {}
pub trait ICPRouteInformation: Sized + std::ops::Deref {
    unsafe fn initWithManeuvers_laneGuidances_currentManeuvers_currentLaneGuidance_tripTravelEstimates_maneuverTravelEstimates_(
        &self,
        maneuvers: NSArray,
        laneGuidances: NSArray,
        currentManeuvers: NSArray,
        currentLaneGuidance: CPLaneGuidance,
        tripTravelEstimates: CPTravelEstimates,
        maneuverTravelEstimates: CPTravelEstimates,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithManeuvers : maneuvers, laneGuidances : laneGuidances, currentManeuvers : currentManeuvers, currentLaneGuidance : currentLaneGuidance, tripTravelEstimates : tripTravelEstimates, maneuverTravelEstimates : maneuverTravelEstimates)
    }
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn maneuvers(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, maneuvers)
    }
    unsafe fn laneGuidances(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, laneGuidances)
    }
    unsafe fn currentManeuvers(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, currentManeuvers)
    }
    unsafe fn currentLaneGuidance(&self) -> CPLaneGuidance
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, currentLaneGuidance)
    }
    unsafe fn tripTravelEstimates(&self) -> CPTravelEstimates
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, tripTravelEstimates)
    }
    unsafe fn maneuverTravelEstimates(&self) -> CPTravelEstimates
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, maneuverTravelEstimates)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CPRouteChoice(pub id);
impl std::ops::Deref for CPRouteChoice {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CPRouteChoice {}
impl CPRouteChoice {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CPRouteChoice").unwrap(), alloc) })
    }
}
impl PNSCopying for CPRouteChoice {}
impl PNSSecureCoding for CPRouteChoice {}
impl INSObject for CPRouteChoice {}
impl PNSObject for CPRouteChoice {}
impl std::convert::TryFrom<NSObject> for CPRouteChoice {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<CPRouteChoice, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CPRouteChoice").unwrap()) };
        if is_kind_of {
            Ok(CPRouteChoice(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to CPRouteChoice")
        }
    }
}
impl ICPRouteChoice for CPRouteChoice {}
pub trait ICPRouteChoice: Sized + std::ops::Deref {
    unsafe fn initWithSummaryVariants_additionalInformationVariants_selectionSummaryVariants_(
        &self,
        summaryVariants: NSArray,
        additionalInformationVariants: NSArray,
        selectionSummaryVariants: NSArray,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithSummaryVariants : summaryVariants, additionalInformationVariants : additionalInformationVariants, selectionSummaryVariants : selectionSummaryVariants)
    }
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn summaryVariants(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, summaryVariants)
    }
    unsafe fn selectionSummaryVariants(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, selectionSummaryVariants)
    }
    unsafe fn additionalInformationVariants(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, additionalInformationVariants)
    }
    unsafe fn userInfo(&self) -> id
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, userInfo)
    }
    unsafe fn setUserInfo_(&self, userInfo: id)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setUserInfo : userInfo)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CPRouteChoice").unwrap(), new)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CPTrip(pub id);
impl std::ops::Deref for CPTrip {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CPTrip {}
impl CPTrip {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CPTrip").unwrap(), alloc) })
    }
}
impl PNSSecureCoding for CPTrip {}
impl INSObject for CPTrip {}
impl PNSObject for CPTrip {}
impl std::convert::TryFrom<NSObject> for CPTrip {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<CPTrip, Self::Error> {
        let is_kind_of: bool = unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CPTrip").unwrap()) };
        if is_kind_of {
            Ok(CPTrip(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to CPTrip")
        }
    }
}
impl ICPTrip for CPTrip {}
pub trait ICPTrip: Sized + std::ops::Deref {
    unsafe fn initWithOrigin_destination_routeChoices_(
        &self,
        origin: MKMapItem,
        destination: MKMapItem,
        routeChoices: NSArray,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithOrigin : origin, destination : destination, routeChoices : routeChoices)
    }
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn origin(&self) -> MKMapItem
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, origin)
    }
    unsafe fn destination(&self) -> MKMapItem
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, destination)
    }
    unsafe fn routeChoices(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, routeChoices)
    }
    unsafe fn userInfo(&self) -> id
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, userInfo)
    }
    unsafe fn setUserInfo_(&self, userInfo: id)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setUserInfo : userInfo)
    }
    unsafe fn destinationNameVariants(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, destinationNameVariants)
    }
    unsafe fn setDestinationNameVariants_(&self, destinationNameVariants: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDestinationNameVariants : destinationNameVariants)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CPTrip").unwrap(), new)
    }
}
pub type CPTripPauseReason = NSUInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CPNavigationSession(pub id);
impl std::ops::Deref for CPNavigationSession {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CPNavigationSession {}
impl CPNavigationSession {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CPNavigationSession").unwrap(), alloc) })
    }
}
impl INSObject for CPNavigationSession {}
impl PNSObject for CPNavigationSession {}
impl std::convert::TryFrom<NSObject> for CPNavigationSession {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<CPNavigationSession, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CPNavigationSession").unwrap()) };
        if is_kind_of {
            Ok(CPNavigationSession(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to CPNavigationSession")
        }
    }
}
impl ICPNavigationSession for CPNavigationSession {}
pub trait ICPNavigationSession: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn pauseTripForReason_description_(
        &self,
        reason: CPTripPauseReason,
        description: NSString,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, pauseTripForReason : reason, description : description)
    }
    unsafe fn pauseTripForReason_description_turnCardColor_(
        &self,
        reason: CPTripPauseReason,
        description: NSString,
        turnCardColor: UIColor,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, pauseTripForReason : reason, description : description, turnCardColor : turnCardColor)
    }
    unsafe fn resumeTripWithUpdatedRouteInformation_(&self, routeInformation: CPRouteInformation)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, resumeTripWithUpdatedRouteInformation : routeInformation)
    }
    unsafe fn finishTrip(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, finishTrip)
    }
    unsafe fn cancelTrip(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, cancelTrip)
    }
    unsafe fn addManeuvers_(&self, maneuvers: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, addManeuvers : maneuvers)
    }
    unsafe fn addLaneGuidances_(&self, laneGuidances: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, addLaneGuidances : laneGuidances)
    }
    unsafe fn updateTravelEstimates_forManeuver_(
        &self,
        estimates: CPTravelEstimates,
        maneuver: CPManeuver,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, updateTravelEstimates : estimates, forManeuver : maneuver)
    }
    unsafe fn upcomingManeuvers(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, upcomingManeuvers)
    }
    unsafe fn setUpcomingManeuvers_(&self, upcomingManeuvers: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setUpcomingManeuvers : upcomingManeuvers)
    }
    unsafe fn currentLaneGuidance(&self) -> CPLaneGuidance
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, currentLaneGuidance)
    }
    unsafe fn setCurrentLaneGuidance_(&self, currentLaneGuidance: CPLaneGuidance)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCurrentLaneGuidance : currentLaneGuidance)
    }
    unsafe fn currentRoadNameVariants(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, currentRoadNameVariants)
    }
    unsafe fn setCurrentRoadNameVariants_(&self, currentRoadNameVariants: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCurrentRoadNameVariants : currentRoadNameVariants)
    }
    unsafe fn maneuverState(&self) -> CPManeuverState
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, maneuverState)
    }
    unsafe fn setManeuverState_(&self, maneuverState: CPManeuverState)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setManeuverState : maneuverState)
    }
    unsafe fn trip(&self) -> CPTrip
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, trip)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CPNavigationSession").unwrap(), new)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CPTripPreviewTextConfiguration(pub id);
impl std::ops::Deref for CPTripPreviewTextConfiguration {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CPTripPreviewTextConfiguration {}
impl CPTripPreviewTextConfiguration {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CPTripPreviewTextConfiguration").unwrap(), alloc) })
    }
}
impl PNSSecureCoding for CPTripPreviewTextConfiguration {}
impl INSObject for CPTripPreviewTextConfiguration {}
impl PNSObject for CPTripPreviewTextConfiguration {}
impl std::convert::TryFrom<NSObject> for CPTripPreviewTextConfiguration {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<CPTripPreviewTextConfiguration, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CPTripPreviewTextConfiguration").unwrap())
        };
        if is_kind_of {
            Ok(CPTripPreviewTextConfiguration(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to CPTripPreviewTextConfiguration")
        }
    }
}
impl ICPTripPreviewTextConfiguration for CPTripPreviewTextConfiguration {}
pub trait ICPTripPreviewTextConfiguration: Sized + std::ops::Deref {
    unsafe fn initWithStartButtonTitle_additionalRoutesButtonTitle_overviewButtonTitle_(
        &self,
        startButtonTitle: NSString,
        additionalRoutesButtonTitle: NSString,
        overviewButtonTitle: NSString,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithStartButtonTitle : startButtonTitle, additionalRoutesButtonTitle : additionalRoutesButtonTitle, overviewButtonTitle : overviewButtonTitle)
    }
    unsafe fn startButtonTitle(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, startButtonTitle)
    }
    unsafe fn additionalRoutesButtonTitle(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, additionalRoutesButtonTitle)
    }
    unsafe fn overviewButtonTitle(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, overviewButtonTitle)
    }
}
pub type CPPanDirection = NSInteger;
pub type CPManeuverDisplayStyle = NSInteger;
pub type CPTimeRemainingColor = NSUInteger;
pub type CPTripEstimateStyle = NSUInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CPMapTemplate(pub id);
impl std::ops::Deref for CPMapTemplate {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CPMapTemplate {}
impl CPMapTemplate {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CPMapTemplate").unwrap(), alloc) })
    }
}
impl PCPBarButtonProviding for CPMapTemplate {}
impl ICPTemplate for CPMapTemplate {}
impl PNSSecureCoding for CPMapTemplate {}
impl std::convert::TryFrom<CPTemplate> for CPMapTemplate {
    type Error = &'static str;
    fn try_from(parent: CPTemplate) -> Result<CPMapTemplate, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CPMapTemplate").unwrap()) };
        if is_kind_of {
            Ok(CPMapTemplate(parent.0))
        } else {
            Err("This CPTemplate cannot be downcasted to CPMapTemplate")
        }
    }
}
impl INSObject for CPMapTemplate {}
impl PNSObject for CPMapTemplate {}
impl ICPMapTemplate for CPMapTemplate {}
pub trait ICPMapTemplate: Sized + std::ops::Deref {
    unsafe fn showTripPreviews_textConfiguration_(
        &self,
        tripPreviews: NSArray,
        textConfiguration: CPTripPreviewTextConfiguration,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, showTripPreviews : tripPreviews, textConfiguration : textConfiguration)
    }
    unsafe fn showTripPreviews_selectedTrip_textConfiguration_(
        &self,
        tripPreviews: NSArray,
        selectedTrip: CPTrip,
        textConfiguration: CPTripPreviewTextConfiguration,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, showTripPreviews : tripPreviews, selectedTrip : selectedTrip, textConfiguration : textConfiguration)
    }
    unsafe fn showRouteChoicesPreviewForTrip_textConfiguration_(
        &self,
        tripPreview: CPTrip,
        textConfiguration: CPTripPreviewTextConfiguration,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, showRouteChoicesPreviewForTrip : tripPreview, textConfiguration : textConfiguration)
    }
    unsafe fn hideTripPreviews(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, hideTripPreviews)
    }
    unsafe fn updateTravelEstimates_forTrip_(&self, estimates: CPTravelEstimates, trip: CPTrip)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, updateTravelEstimates : estimates, forTrip : trip)
    }
    unsafe fn updateTravelEstimates_forTrip_withTimeRemainingColor_(
        &self,
        estimates: CPTravelEstimates,
        trip: CPTrip,
        timeRemainingColor: CPTimeRemainingColor,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, updateTravelEstimates : estimates, forTrip : trip, withTimeRemainingColor : timeRemainingColor)
    }
    unsafe fn startNavigationSessionForTrip_(&self, trip: CPTrip) -> CPNavigationSession
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, startNavigationSessionForTrip : trip)
    }
    unsafe fn showPanningInterfaceAnimated_(&self, animated: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, showPanningInterfaceAnimated : animated)
    }
    unsafe fn dismissPanningInterfaceAnimated_(&self, animated: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, dismissPanningInterfaceAnimated : animated)
    }
    unsafe fn presentNavigationAlert_animated_(
        &self,
        navigationAlert: CPNavigationAlert,
        animated: BOOL,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, presentNavigationAlert : navigationAlert, animated : animated)
    }
    unsafe fn dismissNavigationAlertAnimated_completion_(
        &self,
        animated: BOOL,
        completion: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, dismissNavigationAlertAnimated : animated, completion : completion)
    }
    unsafe fn guidanceBackgroundColor(&self) -> UIColor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, guidanceBackgroundColor)
    }
    unsafe fn setGuidanceBackgroundColor_(&self, guidanceBackgroundColor: UIColor)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setGuidanceBackgroundColor : guidanceBackgroundColor)
    }
    unsafe fn tripEstimateStyle(&self) -> CPTripEstimateStyle
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, tripEstimateStyle)
    }
    unsafe fn setTripEstimateStyle_(&self, tripEstimateStyle: CPTripEstimateStyle)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setTripEstimateStyle : tripEstimateStyle)
    }
    unsafe fn mapButtons(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, mapButtons)
    }
    unsafe fn setMapButtons_(&self, mapButtons: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMapButtons : mapButtons)
    }
    unsafe fn automaticallyHidesNavigationBar(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, automaticallyHidesNavigationBar)
    }
    unsafe fn setAutomaticallyHidesNavigationBar_(&self, automaticallyHidesNavigationBar: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAutomaticallyHidesNavigationBar : automaticallyHidesNavigationBar)
    }
    unsafe fn hidesButtonsWithNavigationBar(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, hidesButtonsWithNavigationBar)
    }
    unsafe fn setHidesButtonsWithNavigationBar_(&self, hidesButtonsWithNavigationBar: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setHidesButtonsWithNavigationBar : hidesButtonsWithNavigationBar)
    }
    unsafe fn mapDelegate(&self) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, mapDelegate)
    }
    unsafe fn setMapDelegate_(&self, mapDelegate: *mut u64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMapDelegate : mapDelegate)
    }
    unsafe fn isPanningInterfaceVisible(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isPanningInterfaceVisible)
    }
    unsafe fn currentNavigationAlert(&self) -> CPNavigationAlert
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, currentNavigationAlert)
    }
}
pub trait PCPMapTemplateDelegate: Sized + std::ops::Deref {
    unsafe fn mapTemplateShouldProvideNavigationMetadata_(&self, mapTemplate: CPMapTemplate) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, mapTemplateShouldProvideNavigationMetadata : mapTemplate)
    }
    unsafe fn mapTemplate_shouldShowNotificationForManeuver_(
        &self,
        mapTemplate: CPMapTemplate,
        maneuver: CPManeuver,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, mapTemplate : mapTemplate, shouldShowNotificationForManeuver : maneuver)
    }
    unsafe fn mapTemplate_shouldUpdateNotificationForManeuver_withTravelEstimates_(
        &self,
        mapTemplate: CPMapTemplate,
        maneuver: CPManeuver,
        travelEstimates: CPTravelEstimates,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, mapTemplate : mapTemplate, shouldUpdateNotificationForManeuver : maneuver, withTravelEstimates : travelEstimates)
    }
    unsafe fn mapTemplate_shouldShowNotificationForNavigationAlert_(
        &self,
        mapTemplate: CPMapTemplate,
        navigationAlert: CPNavigationAlert,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, mapTemplate : mapTemplate, shouldShowNotificationForNavigationAlert : navigationAlert)
    }
    unsafe fn mapTemplateDidShowPanningInterface_(&self, mapTemplate: CPMapTemplate)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, mapTemplateDidShowPanningInterface : mapTemplate)
    }
    unsafe fn mapTemplateWillDismissPanningInterface_(&self, mapTemplate: CPMapTemplate)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, mapTemplateWillDismissPanningInterface : mapTemplate)
    }
    unsafe fn mapTemplateDidDismissPanningInterface_(&self, mapTemplate: CPMapTemplate)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, mapTemplateDidDismissPanningInterface : mapTemplate)
    }
    unsafe fn mapTemplate_panBeganWithDirection_(
        &self,
        mapTemplate: CPMapTemplate,
        direction: CPPanDirection,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, mapTemplate : mapTemplate, panBeganWithDirection : direction)
    }
    unsafe fn mapTemplate_panEndedWithDirection_(
        &self,
        mapTemplate: CPMapTemplate,
        direction: CPPanDirection,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, mapTemplate : mapTemplate, panEndedWithDirection : direction)
    }
    unsafe fn mapTemplate_panWithDirection_(
        &self,
        mapTemplate: CPMapTemplate,
        direction: CPPanDirection,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, mapTemplate : mapTemplate, panWithDirection : direction)
    }
    unsafe fn mapTemplateDidBeginPanGesture_(&self, mapTemplate: CPMapTemplate)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, mapTemplateDidBeginPanGesture : mapTemplate)
    }
    unsafe fn mapTemplate_didUpdatePanGestureWithTranslation_velocity_(
        &self,
        mapTemplate: CPMapTemplate,
        translation: CGPoint,
        velocity: CGPoint,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, mapTemplate : mapTemplate, didUpdatePanGestureWithTranslation : translation, velocity : velocity)
    }
    unsafe fn mapTemplate_didEndPanGestureWithVelocity_(
        &self,
        mapTemplate: CPMapTemplate,
        velocity: CGPoint,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, mapTemplate : mapTemplate, didEndPanGestureWithVelocity : velocity)
    }
    unsafe fn mapTemplateDidBeginZoomGesture_(&self, mapTemplate: CPMapTemplate)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, mapTemplateDidBeginZoomGesture : mapTemplate)
    }
    unsafe fn mapTemplate_didUpdateZoomGestureWithCenter_scale_velocity_(
        &self,
        mapTemplate: CPMapTemplate,
        center: CGPoint,
        scale: CGFloat,
        velocity: CGFloat,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, mapTemplate : mapTemplate, didUpdateZoomGestureWithCenter : center, scale : scale, velocity : velocity)
    }
    unsafe fn mapTemplate_didEndZoomGestureWithVelocity_(
        &self,
        mapTemplate: CPMapTemplate,
        velocity: CGFloat,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, mapTemplate : mapTemplate, didEndZoomGestureWithVelocity : velocity)
    }
    unsafe fn mapTemplateDidBeginRotationGesture_(&self, mapTemplate: CPMapTemplate)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, mapTemplateDidBeginRotationGesture : mapTemplate)
    }
    unsafe fn mapTemplate_didRotateWithCenter_rotation_velocity_(
        &self,
        mapTemplate: CPMapTemplate,
        center: CGPoint,
        rotation: CGFloat,
        velocity: CGFloat,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, mapTemplate : mapTemplate, didRotateWithCenter : center, rotation : rotation, velocity : velocity)
    }
    unsafe fn mapTemplate_rotationDidEndWithVelocity_(
        &self,
        mapTemplate: CPMapTemplate,
        velocity: CGFloat,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, mapTemplate : mapTemplate, rotationDidEndWithVelocity : velocity)
    }
    unsafe fn mapTemplateDidBeginPitchGesture_(&self, mapTemplate: CPMapTemplate)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, mapTemplateDidBeginPitchGesture : mapTemplate)
    }
    unsafe fn mapTemplate_pitchWithCenter_(&self, mapTemplate: CPMapTemplate, center: CGPoint)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, mapTemplate : mapTemplate, pitchWithCenter : center)
    }
    unsafe fn mapTemplate_pitchEndedWithCenter_(&self, mapTemplate: CPMapTemplate, center: CGPoint)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, mapTemplate : mapTemplate, pitchEndedWithCenter : center)
    }
    unsafe fn mapTemplate_willShowNavigationAlert_(
        &self,
        mapTemplate: CPMapTemplate,
        navigationAlert: CPNavigationAlert,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, mapTemplate : mapTemplate, willShowNavigationAlert : navigationAlert)
    }
    unsafe fn mapTemplate_didShowNavigationAlert_(
        &self,
        mapTemplate: CPMapTemplate,
        navigationAlert: CPNavigationAlert,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, mapTemplate : mapTemplate, didShowNavigationAlert : navigationAlert)
    }
    unsafe fn mapTemplate_willDismissNavigationAlert_dismissalContext_(
        &self,
        mapTemplate: CPMapTemplate,
        navigationAlert: CPNavigationAlert,
        dismissalContext: CPNavigationAlertDismissalContext,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, mapTemplate : mapTemplate, willDismissNavigationAlert : navigationAlert, dismissalContext : dismissalContext)
    }
    unsafe fn mapTemplate_didDismissNavigationAlert_dismissalContext_(
        &self,
        mapTemplate: CPMapTemplate,
        navigationAlert: CPNavigationAlert,
        dismissalContext: CPNavigationAlertDismissalContext,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, mapTemplate : mapTemplate, didDismissNavigationAlert : navigationAlert, dismissalContext : dismissalContext)
    }
    unsafe fn mapTemplate_selectedPreviewForTrip_usingRouteChoice_(
        &self,
        mapTemplate: CPMapTemplate,
        trip: CPTrip,
        routeChoice: CPRouteChoice,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, mapTemplate : mapTemplate, selectedPreviewForTrip : trip, usingRouteChoice : routeChoice)
    }
    unsafe fn mapTemplate_startedTrip_usingRouteChoice_(
        &self,
        mapTemplate: CPMapTemplate,
        trip: CPTrip,
        routeChoice: CPRouteChoice,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, mapTemplate : mapTemplate, startedTrip : trip, usingRouteChoice : routeChoice)
    }
    unsafe fn mapTemplateDidCancelNavigation_(&self, mapTemplate: CPMapTemplate)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, mapTemplateDidCancelNavigation : mapTemplate)
    }
    unsafe fn mapTemplate_displayStyleForManeuver_(
        &self,
        mapTemplate: CPMapTemplate,
        maneuver: CPManeuver,
    ) -> CPManeuverDisplayStyle
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, mapTemplate : mapTemplate, displayStyleForManeuver : maneuver)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CPMessageComposeBarButton(pub id);
impl std::ops::Deref for CPMessageComposeBarButton {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CPMessageComposeBarButton {}
impl CPMessageComposeBarButton {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CPMessageComposeBarButton").unwrap(), alloc) })
    }
}
impl ICPBarButton for CPMessageComposeBarButton {}
impl PNSSecureCoding for CPMessageComposeBarButton {}
impl From<CPMessageComposeBarButton> for CPBarButton {
    fn from(child: CPMessageComposeBarButton) -> CPBarButton {
        CPBarButton(child.0)
    }
}
impl std::convert::TryFrom<CPBarButton> for CPMessageComposeBarButton {
    type Error = &'static str;
    fn try_from(parent: CPBarButton) -> Result<CPMessageComposeBarButton, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CPMessageComposeBarButton").unwrap()) };
        if is_kind_of {
            Ok(CPMessageComposeBarButton(parent.0))
        } else {
            Err("This CPBarButton cannot be downcasted to CPMessageComposeBarButton")
        }
    }
}
impl INSObject for CPMessageComposeBarButton {}
impl PNSObject for CPMessageComposeBarButton {}
impl ICPMessageComposeBarButton for CPMessageComposeBarButton {}
pub trait ICPMessageComposeBarButton: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn initWithImage_(&self, image: UIImage) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithImage : image)
    }
    unsafe fn initWithImage_handler_(
        &self,
        image: UIImage,
        handler: CPBarButtonHandler,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithImage : image, handler : handler)
    }
    unsafe fn initWithTitle_handler_(
        &self,
        title: NSString,
        handler: CPBarButtonHandler,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithTitle : title, handler : handler)
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
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CPMessageComposeBarButton").unwrap(), new)
    }
}
pub type CPMessageLeadingItem = NSInteger;
pub type CPMessageTrailingItem = NSInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CPMessageListItemLeadingConfiguration(pub id);
impl std::ops::Deref for CPMessageListItemLeadingConfiguration {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CPMessageListItemLeadingConfiguration {}
impl CPMessageListItemLeadingConfiguration {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CPMessageListItemLeadingConfiguration").unwrap(), alloc) })
    }
}
impl INSObject for CPMessageListItemLeadingConfiguration {}
impl PNSObject for CPMessageListItemLeadingConfiguration {}
impl std::convert::TryFrom<NSObject> for CPMessageListItemLeadingConfiguration {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<CPMessageListItemLeadingConfiguration, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CPMessageListItemLeadingConfiguration").unwrap())
        };
        if is_kind_of {
            Ok(CPMessageListItemLeadingConfiguration(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to CPMessageListItemLeadingConfiguration")
        }
    }
}
impl ICPMessageListItemLeadingConfiguration for CPMessageListItemLeadingConfiguration {}
pub trait ICPMessageListItemLeadingConfiguration: Sized + std::ops::Deref {
    unsafe fn initWithLeadingItem_leadingImage_unread_(
        &self,
        leadingItem: CPMessageLeadingItem,
        leadingImage: UIImage,
        unread: BOOL,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithLeadingItem : leadingItem, leadingImage : leadingImage, unread : unread)
    }
    unsafe fn isUnread(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isUnread)
    }
    unsafe fn leadingItem(&self) -> CPMessageLeadingItem
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, leadingItem)
    }
    unsafe fn leadingImage(&self) -> UIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, leadingImage)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CPMessageListItemTrailingConfiguration(pub id);
impl std::ops::Deref for CPMessageListItemTrailingConfiguration {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CPMessageListItemTrailingConfiguration {}
impl CPMessageListItemTrailingConfiguration {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CPMessageListItemTrailingConfiguration").unwrap(), alloc) })
    }
}
impl INSObject for CPMessageListItemTrailingConfiguration {}
impl PNSObject for CPMessageListItemTrailingConfiguration {}
impl std::convert::TryFrom<NSObject> for CPMessageListItemTrailingConfiguration {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<CPMessageListItemTrailingConfiguration, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CPMessageListItemTrailingConfiguration").unwrap())
        };
        if is_kind_of {
            Ok(CPMessageListItemTrailingConfiguration(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to CPMessageListItemTrailingConfiguration")
        }
    }
}
impl ICPMessageListItemTrailingConfiguration for CPMessageListItemTrailingConfiguration {}
pub trait ICPMessageListItemTrailingConfiguration: Sized + std::ops::Deref {
    unsafe fn initWithTrailingItem_trailingImage_(
        &self,
        trailingItem: CPMessageTrailingItem,
        trailingImage: UIImage,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithTrailingItem : trailingItem, trailingImage : trailingImage)
    }
    unsafe fn trailingItem(&self) -> CPMessageTrailingItem
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, trailingItem)
    }
    unsafe fn trailingImage(&self) -> UIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, trailingImage)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CPMessageListItem(pub id);
impl std::ops::Deref for CPMessageListItem {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CPMessageListItem {}
impl CPMessageListItem {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CPMessageListItem").unwrap(), alloc) })
    }
}
impl PCPListTemplateItem for CPMessageListItem {}
impl INSObject for CPMessageListItem {}
impl PNSObject for CPMessageListItem {}
impl std::convert::TryFrom<NSObject> for CPMessageListItem {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<CPMessageListItem, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CPMessageListItem").unwrap()) };
        if is_kind_of {
            Ok(CPMessageListItem(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to CPMessageListItem")
        }
    }
}
impl ICPMessageListItem for CPMessageListItem {}
pub trait ICPMessageListItem: Sized + std::ops::Deref {
    unsafe fn initWithConversationIdentifier_text_leadingConfiguration_trailingConfiguration_detailText_trailingText_(
        &self,
        conversationIdentifier: NSString,
        text: NSString,
        leadingConfiguration: CPMessageListItemLeadingConfiguration,
        trailingConfiguration: CPMessageListItemTrailingConfiguration,
        detailText: NSString,
        trailingText: NSString,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithConversationIdentifier : conversationIdentifier, text : text, leadingConfiguration : leadingConfiguration, trailingConfiguration : trailingConfiguration, detailText : detailText, trailingText : trailingText)
    }
    unsafe fn initWithFullName_phoneOrEmailAddress_leadingConfiguration_trailingConfiguration_detailText_trailingText_(
        &self,
        fullName: NSString,
        phoneOrEmailAddress: NSString,
        leadingConfiguration: CPMessageListItemLeadingConfiguration,
        trailingConfiguration: CPMessageListItemTrailingConfiguration,
        detailText: NSString,
        trailingText: NSString,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithFullName : fullName, phoneOrEmailAddress : phoneOrEmailAddress, leadingConfiguration : leadingConfiguration, trailingConfiguration : trailingConfiguration, detailText : detailText, trailingText : trailingText)
    }
    unsafe fn text(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, text)
    }
    unsafe fn setText_(&self, text: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setText : text)
    }
    unsafe fn conversationIdentifier(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, conversationIdentifier)
    }
    unsafe fn setConversationIdentifier_(&self, conversationIdentifier: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setConversationIdentifier : conversationIdentifier)
    }
    unsafe fn phoneOrEmailAddress(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, phoneOrEmailAddress)
    }
    unsafe fn setPhoneOrEmailAddress_(&self, phoneOrEmailAddress: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPhoneOrEmailAddress : phoneOrEmailAddress)
    }
    unsafe fn leadingConfiguration(&self) -> CPMessageListItemLeadingConfiguration
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, leadingConfiguration)
    }
    unsafe fn setLeadingConfiguration_(
        &self,
        leadingConfiguration: CPMessageListItemLeadingConfiguration,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setLeadingConfiguration : leadingConfiguration)
    }
    unsafe fn trailingConfiguration(&self) -> CPMessageListItemTrailingConfiguration
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, trailingConfiguration)
    }
    unsafe fn setTrailingConfiguration_(
        &self,
        trailingConfiguration: CPMessageListItemTrailingConfiguration,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setTrailingConfiguration : trailingConfiguration)
    }
    unsafe fn detailText(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, detailText)
    }
    unsafe fn setDetailText_(&self, detailText: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDetailText : detailText)
    }
    unsafe fn trailingText(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, trailingText)
    }
    unsafe fn setTrailingText_(&self, trailingText: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setTrailingText : trailingText)
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
    unsafe fn userInfo(&self) -> id
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, userInfo)
    }
    unsafe fn setUserInfo_(&self, userInfo: id)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setUserInfo : userInfo)
    }
    unsafe fn leadingDetailTextImage(&self) -> UIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, leadingDetailTextImage)
    }
    unsafe fn setLeadingDetailTextImage_(&self, leadingDetailTextImage: UIImage)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setLeadingDetailTextImage : leadingDetailTextImage)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CPNowPlayingButton(pub id);
impl std::ops::Deref for CPNowPlayingButton {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CPNowPlayingButton {}
impl CPNowPlayingButton {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CPNowPlayingButton").unwrap(), alloc) })
    }
}
impl PNSSecureCoding for CPNowPlayingButton {}
impl INSObject for CPNowPlayingButton {}
impl PNSObject for CPNowPlayingButton {}
impl std::convert::TryFrom<NSObject> for CPNowPlayingButton {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<CPNowPlayingButton, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CPNowPlayingButton").unwrap()) };
        if is_kind_of {
            Ok(CPNowPlayingButton(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to CPNowPlayingButton")
        }
    }
}
impl ICPNowPlayingButton for CPNowPlayingButton {}
pub trait ICPNowPlayingButton: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn initWithHandler_(&self, handler: *mut ::std::os::raw::c_void) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithHandler : handler)
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
    unsafe fn isSelected(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isSelected)
    }
    unsafe fn setSelected_(&self, selected: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSelected : selected)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CPNowPlayingButton").unwrap(), new)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CPNowPlayingShuffleButton(pub id);
impl std::ops::Deref for CPNowPlayingShuffleButton {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CPNowPlayingShuffleButton {}
impl CPNowPlayingShuffleButton {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CPNowPlayingShuffleButton").unwrap(), alloc) })
    }
}
impl ICPNowPlayingButton for CPNowPlayingShuffleButton {}
impl PNSSecureCoding for CPNowPlayingShuffleButton {}
impl From<CPNowPlayingShuffleButton> for CPNowPlayingButton {
    fn from(child: CPNowPlayingShuffleButton) -> CPNowPlayingButton {
        CPNowPlayingButton(child.0)
    }
}
impl std::convert::TryFrom<CPNowPlayingButton> for CPNowPlayingShuffleButton {
    type Error = &'static str;
    fn try_from(parent: CPNowPlayingButton) -> Result<CPNowPlayingShuffleButton, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CPNowPlayingShuffleButton").unwrap()) };
        if is_kind_of {
            Ok(CPNowPlayingShuffleButton(parent.0))
        } else {
            Err("This CPNowPlayingButton cannot be downcasted to CPNowPlayingShuffleButton")
        }
    }
}
impl INSObject for CPNowPlayingShuffleButton {}
impl PNSObject for CPNowPlayingShuffleButton {}
impl ICPNowPlayingShuffleButton for CPNowPlayingShuffleButton {}
pub trait ICPNowPlayingShuffleButton: Sized + std::ops::Deref {}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CPNowPlayingAddToLibraryButton(pub id);
impl std::ops::Deref for CPNowPlayingAddToLibraryButton {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CPNowPlayingAddToLibraryButton {}
impl CPNowPlayingAddToLibraryButton {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CPNowPlayingAddToLibraryButton").unwrap(), alloc) })
    }
}
impl ICPNowPlayingButton for CPNowPlayingAddToLibraryButton {}
impl PNSSecureCoding for CPNowPlayingAddToLibraryButton {}
impl std::convert::TryFrom<CPNowPlayingButton> for CPNowPlayingAddToLibraryButton {
    type Error = &'static str;
    fn try_from(parent: CPNowPlayingButton) -> Result<CPNowPlayingAddToLibraryButton, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CPNowPlayingAddToLibraryButton").unwrap())
        };
        if is_kind_of {
            Ok(CPNowPlayingAddToLibraryButton(parent.0))
        } else {
            Err("This CPNowPlayingButton cannot be downcasted to CPNowPlayingAddToLibraryButton")
        }
    }
}
impl INSObject for CPNowPlayingAddToLibraryButton {}
impl PNSObject for CPNowPlayingAddToLibraryButton {}
impl ICPNowPlayingAddToLibraryButton for CPNowPlayingAddToLibraryButton {}
pub trait ICPNowPlayingAddToLibraryButton: Sized + std::ops::Deref {}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CPNowPlayingMoreButton(pub id);
impl std::ops::Deref for CPNowPlayingMoreButton {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CPNowPlayingMoreButton {}
impl CPNowPlayingMoreButton {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CPNowPlayingMoreButton").unwrap(), alloc) })
    }
}
impl ICPNowPlayingButton for CPNowPlayingMoreButton {}
impl PNSSecureCoding for CPNowPlayingMoreButton {}
impl std::convert::TryFrom<CPNowPlayingButton> for CPNowPlayingMoreButton {
    type Error = &'static str;
    fn try_from(parent: CPNowPlayingButton) -> Result<CPNowPlayingMoreButton, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CPNowPlayingMoreButton").unwrap()) };
        if is_kind_of {
            Ok(CPNowPlayingMoreButton(parent.0))
        } else {
            Err("This CPNowPlayingButton cannot be downcasted to CPNowPlayingMoreButton")
        }
    }
}
impl INSObject for CPNowPlayingMoreButton {}
impl PNSObject for CPNowPlayingMoreButton {}
impl ICPNowPlayingMoreButton for CPNowPlayingMoreButton {}
pub trait ICPNowPlayingMoreButton: Sized + std::ops::Deref {}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CPNowPlayingPlaybackRateButton(pub id);
impl std::ops::Deref for CPNowPlayingPlaybackRateButton {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CPNowPlayingPlaybackRateButton {}
impl CPNowPlayingPlaybackRateButton {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CPNowPlayingPlaybackRateButton").unwrap(), alloc) })
    }
}
impl ICPNowPlayingButton for CPNowPlayingPlaybackRateButton {}
impl PNSSecureCoding for CPNowPlayingPlaybackRateButton {}
impl std::convert::TryFrom<CPNowPlayingButton> for CPNowPlayingPlaybackRateButton {
    type Error = &'static str;
    fn try_from(parent: CPNowPlayingButton) -> Result<CPNowPlayingPlaybackRateButton, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CPNowPlayingPlaybackRateButton").unwrap())
        };
        if is_kind_of {
            Ok(CPNowPlayingPlaybackRateButton(parent.0))
        } else {
            Err("This CPNowPlayingButton cannot be downcasted to CPNowPlayingPlaybackRateButton")
        }
    }
}
impl INSObject for CPNowPlayingPlaybackRateButton {}
impl PNSObject for CPNowPlayingPlaybackRateButton {}
impl ICPNowPlayingPlaybackRateButton for CPNowPlayingPlaybackRateButton {}
pub trait ICPNowPlayingPlaybackRateButton: Sized + std::ops::Deref {}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CPNowPlayingRepeatButton(pub id);
impl std::ops::Deref for CPNowPlayingRepeatButton {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CPNowPlayingRepeatButton {}
impl CPNowPlayingRepeatButton {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CPNowPlayingRepeatButton").unwrap(), alloc) })
    }
}
impl ICPNowPlayingButton for CPNowPlayingRepeatButton {}
impl PNSSecureCoding for CPNowPlayingRepeatButton {}
impl std::convert::TryFrom<CPNowPlayingButton> for CPNowPlayingRepeatButton {
    type Error = &'static str;
    fn try_from(parent: CPNowPlayingButton) -> Result<CPNowPlayingRepeatButton, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CPNowPlayingRepeatButton").unwrap()) };
        if is_kind_of {
            Ok(CPNowPlayingRepeatButton(parent.0))
        } else {
            Err("This CPNowPlayingButton cannot be downcasted to CPNowPlayingRepeatButton")
        }
    }
}
impl INSObject for CPNowPlayingRepeatButton {}
impl PNSObject for CPNowPlayingRepeatButton {}
impl ICPNowPlayingRepeatButton for CPNowPlayingRepeatButton {}
pub trait ICPNowPlayingRepeatButton: Sized + std::ops::Deref {}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CPNowPlayingImageButton(pub id);
impl std::ops::Deref for CPNowPlayingImageButton {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CPNowPlayingImageButton {}
impl CPNowPlayingImageButton {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CPNowPlayingImageButton").unwrap(), alloc) })
    }
}
impl ICPNowPlayingButton for CPNowPlayingImageButton {}
impl PNSSecureCoding for CPNowPlayingImageButton {}
impl std::convert::TryFrom<CPNowPlayingButton> for CPNowPlayingImageButton {
    type Error = &'static str;
    fn try_from(parent: CPNowPlayingButton) -> Result<CPNowPlayingImageButton, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CPNowPlayingImageButton").unwrap()) };
        if is_kind_of {
            Ok(CPNowPlayingImageButton(parent.0))
        } else {
            Err("This CPNowPlayingButton cannot be downcasted to CPNowPlayingImageButton")
        }
    }
}
impl INSObject for CPNowPlayingImageButton {}
impl PNSObject for CPNowPlayingImageButton {}
impl ICPNowPlayingImageButton for CPNowPlayingImageButton {}
pub trait ICPNowPlayingImageButton: Sized + std::ops::Deref {
    unsafe fn initWithImage_handler_(
        &self,
        image: UIImage,
        handler: *mut ::std::os::raw::c_void,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithImage : image, handler : handler)
    }
    unsafe fn image(&self) -> UIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, image)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CPNowPlayingMode(pub id);
impl std::ops::Deref for CPNowPlayingMode {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CPNowPlayingMode {}
impl CPNowPlayingMode {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CPNowPlayingMode").unwrap(), alloc) })
    }
}
impl PNSSecureCoding for CPNowPlayingMode {}
impl INSObject for CPNowPlayingMode {}
impl PNSObject for CPNowPlayingMode {}
impl std::convert::TryFrom<NSObject> for CPNowPlayingMode {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<CPNowPlayingMode, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CPNowPlayingMode").unwrap()) };
        if is_kind_of {
            Ok(CPNowPlayingMode(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to CPNowPlayingMode")
        }
    }
}
impl ICPNowPlayingMode for CPNowPlayingMode {}
pub trait ICPNowPlayingMode: Sized + std::ops::Deref {
    unsafe fn defaultNowPlayingMode() -> CPNowPlayingMode
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CPNowPlayingMode").unwrap(), defaultNowPlayingMode)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CPNowPlayingModeSports(pub id);
impl std::ops::Deref for CPNowPlayingModeSports {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CPNowPlayingModeSports {}
impl CPNowPlayingModeSports {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CPNowPlayingModeSports").unwrap(), alloc) })
    }
}
impl PNSSecureCoding for CPNowPlayingModeSports {}
impl ICPNowPlayingMode for CPNowPlayingModeSports {}
impl From<CPNowPlayingModeSports> for CPNowPlayingMode {
    fn from(child: CPNowPlayingModeSports) -> CPNowPlayingMode {
        CPNowPlayingMode(child.0)
    }
}
impl std::convert::TryFrom<CPNowPlayingMode> for CPNowPlayingModeSports {
    type Error = &'static str;
    fn try_from(parent: CPNowPlayingMode) -> Result<CPNowPlayingModeSports, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CPNowPlayingModeSports").unwrap()) };
        if is_kind_of {
            Ok(CPNowPlayingModeSports(parent.0))
        } else {
            Err("This CPNowPlayingMode cannot be downcasted to CPNowPlayingModeSports")
        }
    }
}
impl INSObject for CPNowPlayingModeSports {}
impl PNSObject for CPNowPlayingModeSports {}
impl ICPNowPlayingModeSports for CPNowPlayingModeSports {}
pub trait ICPNowPlayingModeSports: Sized + std::ops::Deref {
    unsafe fn initWithLeftTeam_rightTeam_eventStatus_backgroundArtwork_(
        &self,
        leftTeam: CPNowPlayingSportsTeam,
        rightTeam: CPNowPlayingSportsTeam,
        eventStatus: CPNowPlayingSportsEventStatus,
        backgroundArtwork: UIImage,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithLeftTeam : leftTeam, rightTeam : rightTeam, eventStatus : eventStatus, backgroundArtwork : backgroundArtwork)
    }
    unsafe fn leftTeam(&self) -> CPNowPlayingSportsTeam
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, leftTeam)
    }
    unsafe fn rightTeam(&self) -> CPNowPlayingSportsTeam
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, rightTeam)
    }
    unsafe fn eventStatus(&self) -> CPNowPlayingSportsEventStatus
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, eventStatus)
    }
    unsafe fn backgroundArtwork(&self) -> UIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, backgroundArtwork)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CPNowPlayingSportsTeam(pub id);
impl std::ops::Deref for CPNowPlayingSportsTeam {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CPNowPlayingSportsTeam {}
impl CPNowPlayingSportsTeam {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CPNowPlayingSportsTeam").unwrap(), alloc) })
    }
}
impl PNSSecureCoding for CPNowPlayingSportsTeam {}
impl INSObject for CPNowPlayingSportsTeam {}
impl PNSObject for CPNowPlayingSportsTeam {}
impl std::convert::TryFrom<NSObject> for CPNowPlayingSportsTeam {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<CPNowPlayingSportsTeam, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CPNowPlayingSportsTeam").unwrap()) };
        if is_kind_of {
            Ok(CPNowPlayingSportsTeam(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to CPNowPlayingSportsTeam")
        }
    }
}
impl ICPNowPlayingSportsTeam for CPNowPlayingSportsTeam {}
pub trait ICPNowPlayingSportsTeam: Sized + std::ops::Deref {
    unsafe fn initWithName_logo_teamStandings_eventScore_possessionIndicator_favorite_(
        &self,
        name: NSString,
        logo: CPNowPlayingSportsTeamLogo,
        teamStandings: NSString,
        eventScore: NSString,
        possessionIndicator: UIImage,
        favorite: BOOL,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithName : name, logo : logo, teamStandings : teamStandings, eventScore : eventScore, possessionIndicator : possessionIndicator, favorite : favorite)
    }
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
    unsafe fn logo(&self) -> CPNowPlayingSportsTeamLogo
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, logo)
    }
    unsafe fn teamStandings(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, teamStandings)
    }
    unsafe fn eventScore(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, eventScore)
    }
    unsafe fn possessionIndicator(&self) -> UIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, possessionIndicator)
    }
    unsafe fn isFavorite(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isFavorite)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CPNowPlayingSportsTeam").unwrap(), new)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CPNowPlayingSportsEventStatus(pub id);
impl std::ops::Deref for CPNowPlayingSportsEventStatus {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CPNowPlayingSportsEventStatus {}
impl CPNowPlayingSportsEventStatus {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CPNowPlayingSportsEventStatus").unwrap(), alloc) })
    }
}
impl PNSSecureCoding for CPNowPlayingSportsEventStatus {}
impl INSObject for CPNowPlayingSportsEventStatus {}
impl PNSObject for CPNowPlayingSportsEventStatus {}
impl std::convert::TryFrom<NSObject> for CPNowPlayingSportsEventStatus {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<CPNowPlayingSportsEventStatus, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CPNowPlayingSportsEventStatus").unwrap())
        };
        if is_kind_of {
            Ok(CPNowPlayingSportsEventStatus(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to CPNowPlayingSportsEventStatus")
        }
    }
}
impl ICPNowPlayingSportsEventStatus for CPNowPlayingSportsEventStatus {}
pub trait ICPNowPlayingSportsEventStatus: Sized + std::ops::Deref {
    unsafe fn initWithEventStatusText_eventStatusImage_eventClock_(
        &self,
        eventStatusText: NSArray,
        eventStatusImage: UIImage,
        eventClock: CPNowPlayingSportsClock,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithEventStatusText : eventStatusText, eventStatusImage : eventStatusImage, eventClock : eventClock)
    }
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn eventStatusText(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, eventStatusText)
    }
    unsafe fn eventClock(&self) -> CPNowPlayingSportsClock
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, eventClock)
    }
    unsafe fn eventStatusImage(&self) -> UIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, eventStatusImage)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CPNowPlayingSportsEventStatus").unwrap(), new)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CPNowPlayingSportsClock(pub id);
impl std::ops::Deref for CPNowPlayingSportsClock {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CPNowPlayingSportsClock {}
impl CPNowPlayingSportsClock {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CPNowPlayingSportsClock").unwrap(), alloc) })
    }
}
impl PNSSecureCoding for CPNowPlayingSportsClock {}
impl INSObject for CPNowPlayingSportsClock {}
impl PNSObject for CPNowPlayingSportsClock {}
impl std::convert::TryFrom<NSObject> for CPNowPlayingSportsClock {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<CPNowPlayingSportsClock, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CPNowPlayingSportsClock").unwrap()) };
        if is_kind_of {
            Ok(CPNowPlayingSportsClock(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to CPNowPlayingSportsClock")
        }
    }
}
impl ICPNowPlayingSportsClock for CPNowPlayingSportsClock {}
pub trait ICPNowPlayingSportsClock: Sized + std::ops::Deref {
    unsafe fn initWithElapsedTime_paused_(
        &self,
        elapsedTime: NSTimeInterval,
        paused: BOOL,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithElapsedTime : elapsedTime, paused : paused)
    }
    unsafe fn initWithTimeRemaining_paused_(
        &self,
        timeRemaining: NSTimeInterval,
        paused: BOOL,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithTimeRemaining : timeRemaining, paused : paused)
    }
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn timeValue(&self) -> NSTimeInterval
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, timeValue)
    }
    unsafe fn isPaused(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isPaused)
    }
    unsafe fn countsUp(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, countsUp)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CPNowPlayingSportsClock").unwrap(), new)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CPNowPlayingSportsTeamLogo(pub id);
impl std::ops::Deref for CPNowPlayingSportsTeamLogo {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CPNowPlayingSportsTeamLogo {}
impl CPNowPlayingSportsTeamLogo {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CPNowPlayingSportsTeamLogo").unwrap(), alloc) })
    }
}
impl PNSSecureCoding for CPNowPlayingSportsTeamLogo {}
impl INSObject for CPNowPlayingSportsTeamLogo {}
impl PNSObject for CPNowPlayingSportsTeamLogo {}
impl std::convert::TryFrom<NSObject> for CPNowPlayingSportsTeamLogo {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<CPNowPlayingSportsTeamLogo, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CPNowPlayingSportsTeamLogo").unwrap()) };
        if is_kind_of {
            Ok(CPNowPlayingSportsTeamLogo(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to CPNowPlayingSportsTeamLogo")
        }
    }
}
impl ICPNowPlayingSportsTeamLogo for CPNowPlayingSportsTeamLogo {}
pub trait ICPNowPlayingSportsTeamLogo: Sized + std::ops::Deref {
    unsafe fn initWithTeamLogo_(&self, teamLogo: UIImage) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithTeamLogo : teamLogo)
    }
    unsafe fn initWithTeamInitials_(&self, teamInitials: NSString) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithTeamInitials : teamInitials)
    }
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn logo(&self) -> UIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, logo)
    }
    unsafe fn initials(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, initials)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CPNowPlayingSportsTeamLogo").unwrap(), new)
    }
}
pub trait PCPNowPlayingTemplateObserver: Sized + std::ops::Deref {
    unsafe fn nowPlayingTemplateUpNextButtonTapped_(&self, nowPlayingTemplate: CPNowPlayingTemplate)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, nowPlayingTemplateUpNextButtonTapped : nowPlayingTemplate)
    }
    unsafe fn nowPlayingTemplateAlbumArtistButtonTapped_(
        &self,
        nowPlayingTemplate: CPNowPlayingTemplate,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, nowPlayingTemplateAlbumArtistButtonTapped : nowPlayingTemplate)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CPNowPlayingTemplate(pub id);
impl std::ops::Deref for CPNowPlayingTemplate {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CPNowPlayingTemplate {}
impl CPNowPlayingTemplate {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CPNowPlayingTemplate").unwrap(), alloc) })
    }
}
impl ICPTemplate for CPNowPlayingTemplate {}
impl PNSSecureCoding for CPNowPlayingTemplate {}
impl std::convert::TryFrom<CPTemplate> for CPNowPlayingTemplate {
    type Error = &'static str;
    fn try_from(parent: CPTemplate) -> Result<CPNowPlayingTemplate, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CPNowPlayingTemplate").unwrap()) };
        if is_kind_of {
            Ok(CPNowPlayingTemplate(parent.0))
        } else {
            Err("This CPTemplate cannot be downcasted to CPNowPlayingTemplate")
        }
    }
}
impl INSObject for CPNowPlayingTemplate {}
impl PNSObject for CPNowPlayingTemplate {}
impl ICPNowPlayingTemplate for CPNowPlayingTemplate {}
pub trait ICPNowPlayingTemplate: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
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
    unsafe fn updateNowPlayingButtons_(&self, nowPlayingButtons: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, updateNowPlayingButtons : nowPlayingButtons)
    }
    unsafe fn nowPlayingButtons(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, nowPlayingButtons)
    }
    unsafe fn isUpNextButtonEnabled(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isUpNextButtonEnabled)
    }
    unsafe fn setUpNextButtonEnabled_(&self, upNextButtonEnabled: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setUpNextButtonEnabled : upNextButtonEnabled)
    }
    unsafe fn upNextTitle(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, upNextTitle)
    }
    unsafe fn setUpNextTitle_(&self, upNextTitle: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setUpNextTitle : upNextTitle)
    }
    unsafe fn isAlbumArtistButtonEnabled(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isAlbumArtistButtonEnabled)
    }
    unsafe fn setAlbumArtistButtonEnabled_(&self, albumArtistButtonEnabled: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAlbumArtistButtonEnabled : albumArtistButtonEnabled)
    }
    unsafe fn nowPlayingMode(&self) -> CPNowPlayingMode
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, nowPlayingMode)
    }
    unsafe fn setNowPlayingMode_(&self, nowPlayingMode: CPNowPlayingMode)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setNowPlayingMode : nowPlayingMode)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CPNowPlayingTemplate").unwrap(), new)
    }
    unsafe fn sharedTemplate() -> CPNowPlayingTemplate
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CPNowPlayingTemplate").unwrap(), sharedTemplate)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CPPointOfInterest(pub id);
impl std::ops::Deref for CPPointOfInterest {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CPPointOfInterest {}
impl CPPointOfInterest {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CPPointOfInterest").unwrap(), alloc) })
    }
}
impl PNSSecureCoding for CPPointOfInterest {}
impl INSObject for CPPointOfInterest {}
impl PNSObject for CPPointOfInterest {}
impl std::convert::TryFrom<NSObject> for CPPointOfInterest {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<CPPointOfInterest, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CPPointOfInterest").unwrap()) };
        if is_kind_of {
            Ok(CPPointOfInterest(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to CPPointOfInterest")
        }
    }
}
impl ICPPointOfInterest for CPPointOfInterest {}
pub trait ICPPointOfInterest: Sized + std::ops::Deref {
    unsafe fn initWithLocation_title_subtitle_summary_detailTitle_detailSubtitle_detailSummary_pinImage_selectedPinImage_(
        &self,
        location: MKMapItem,
        title: NSString,
        subtitle: NSString,
        summary: NSString,
        detailTitle: NSString,
        detailSubtitle: NSString,
        detailSummary: NSString,
        pinImage: UIImage,
        selectedPinImage: UIImage,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithLocation : location, title : title, subtitle : subtitle, summary : summary, detailTitle : detailTitle, detailSubtitle : detailSubtitle, detailSummary : detailSummary, pinImage : pinImage, selectedPinImage : selectedPinImage)
    }
    unsafe fn initWithLocation_title_subtitle_summary_detailTitle_detailSubtitle_detailSummary_pinImage_(
        &self,
        location: MKMapItem,
        title: NSString,
        subtitle: NSString,
        summary: NSString,
        detailTitle: NSString,
        detailSubtitle: NSString,
        detailSummary: NSString,
        pinImage: UIImage,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithLocation : location, title : title, subtitle : subtitle, summary : summary, detailTitle : detailTitle, detailSubtitle : detailSubtitle, detailSummary : detailSummary, pinImage : pinImage)
    }
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn location(&self) -> MKMapItem
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, location)
    }
    unsafe fn setLocation_(&self, location: MKMapItem)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setLocation : location)
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
    unsafe fn subtitle(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, subtitle)
    }
    unsafe fn setSubtitle_(&self, subtitle: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSubtitle : subtitle)
    }
    unsafe fn summary(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, summary)
    }
    unsafe fn setSummary_(&self, summary: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSummary : summary)
    }
    unsafe fn detailTitle(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, detailTitle)
    }
    unsafe fn setDetailTitle_(&self, detailTitle: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDetailTitle : detailTitle)
    }
    unsafe fn detailSubtitle(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, detailSubtitle)
    }
    unsafe fn setDetailSubtitle_(&self, detailSubtitle: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDetailSubtitle : detailSubtitle)
    }
    unsafe fn detailSummary(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, detailSummary)
    }
    unsafe fn setDetailSummary_(&self, detailSummary: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDetailSummary : detailSummary)
    }
    unsafe fn pinImage(&self) -> UIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, pinImage)
    }
    unsafe fn setPinImage_(&self, pinImage: UIImage)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPinImage : pinImage)
    }
    unsafe fn selectedPinImage(&self) -> UIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, selectedPinImage)
    }
    unsafe fn setSelectedPinImage_(&self, selectedPinImage: UIImage)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSelectedPinImage : selectedPinImage)
    }
    unsafe fn primaryButton(&self) -> CPTextButton
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, primaryButton)
    }
    unsafe fn setPrimaryButton_(&self, primaryButton: CPTextButton)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPrimaryButton : primaryButton)
    }
    unsafe fn secondaryButton(&self) -> CPTextButton
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, secondaryButton)
    }
    unsafe fn setSecondaryButton_(&self, secondaryButton: CPTextButton)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSecondaryButton : secondaryButton)
    }
    unsafe fn userInfo(&self) -> id
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, userInfo)
    }
    unsafe fn setUserInfo_(&self, userInfo: id)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setUserInfo : userInfo)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CPPointOfInterest").unwrap(), new)
    }
    unsafe fn pinImageSize() -> CGSize
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CPPointOfInterest").unwrap(), pinImageSize)
    }
    unsafe fn selectedPinImageSize() -> CGSize
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CPPointOfInterest").unwrap(), selectedPinImageSize)
    }
}
pub trait PCPPointOfInterestTemplateDelegate: Sized + std::ops::Deref {
    unsafe fn pointOfInterestTemplate_didChangeMapRegion_(
        &self,
        pointOfInterestTemplate: CPPointOfInterestTemplate,
        region: MKCoordinateRegion,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, pointOfInterestTemplate : pointOfInterestTemplate, didChangeMapRegion : region)
    }
    unsafe fn pointOfInterestTemplate_didSelectPointOfInterest_(
        &self,
        pointOfInterestTemplate: CPPointOfInterestTemplate,
        pointOfInterest: CPPointOfInterest,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, pointOfInterestTemplate : pointOfInterestTemplate, didSelectPointOfInterest : pointOfInterest)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CPPointOfInterestTemplate(pub id);
impl std::ops::Deref for CPPointOfInterestTemplate {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CPPointOfInterestTemplate {}
impl CPPointOfInterestTemplate {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CPPointOfInterestTemplate").unwrap(), alloc) })
    }
}
impl PCPBarButtonProviding for CPPointOfInterestTemplate {}
impl ICPTemplate for CPPointOfInterestTemplate {}
impl PNSSecureCoding for CPPointOfInterestTemplate {}
impl std::convert::TryFrom<CPTemplate> for CPPointOfInterestTemplate {
    type Error = &'static str;
    fn try_from(parent: CPTemplate) -> Result<CPPointOfInterestTemplate, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CPPointOfInterestTemplate").unwrap()) };
        if is_kind_of {
            Ok(CPPointOfInterestTemplate(parent.0))
        } else {
            Err("This CPTemplate cannot be downcasted to CPPointOfInterestTemplate")
        }
    }
}
impl INSObject for CPPointOfInterestTemplate {}
impl PNSObject for CPPointOfInterestTemplate {}
impl ICPPointOfInterestTemplate for CPPointOfInterestTemplate {}
pub trait ICPPointOfInterestTemplate: Sized + std::ops::Deref {
    unsafe fn initWithTitle_pointsOfInterest_selectedIndex_(
        &self,
        title: NSString,
        pointsOfInterest: NSArray,
        selectedIndex: NSUInteger,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithTitle : title, pointsOfInterest : pointsOfInterest, selectedIndex : selectedIndex)
    }
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn setPointsOfInterest_selectedIndex_(
        &self,
        pointsOfInterest: NSArray,
        selectedIndex: NSUInteger,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPointsOfInterest : pointsOfInterest, selectedIndex : selectedIndex)
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
    unsafe fn pointsOfInterest(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, pointsOfInterest)
    }
    unsafe fn selectedIndex(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, selectedIndex)
    }
    unsafe fn setSelectedIndex_(&self, selectedIndex: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSelectedIndex : selectedIndex)
    }
    unsafe fn pointOfInterestDelegate(&self) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, pointOfInterestDelegate)
    }
    unsafe fn setPointOfInterestDelegate_(&self, pointOfInterestDelegate: *mut u64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPointOfInterestDelegate : pointOfInterestDelegate)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CPPointOfInterestTemplate").unwrap(), new)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CPSearchTemplate(pub id);
impl std::ops::Deref for CPSearchTemplate {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CPSearchTemplate {}
impl CPSearchTemplate {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CPSearchTemplate").unwrap(), alloc) })
    }
}
impl ICPTemplate for CPSearchTemplate {}
impl PNSSecureCoding for CPSearchTemplate {}
impl std::convert::TryFrom<CPTemplate> for CPSearchTemplate {
    type Error = &'static str;
    fn try_from(parent: CPTemplate) -> Result<CPSearchTemplate, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CPSearchTemplate").unwrap()) };
        if is_kind_of {
            Ok(CPSearchTemplate(parent.0))
        } else {
            Err("This CPTemplate cannot be downcasted to CPSearchTemplate")
        }
    }
}
impl INSObject for CPSearchTemplate {}
impl PNSObject for CPSearchTemplate {}
impl ICPSearchTemplate for CPSearchTemplate {}
pub trait ICPSearchTemplate: Sized + std::ops::Deref {
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
pub trait PCPSearchTemplateDelegate: Sized + std::ops::Deref {
    unsafe fn searchTemplate_updatedSearchText_completionHandler_(
        &self,
        searchTemplate: CPSearchTemplate,
        searchText: NSString,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, searchTemplate : searchTemplate, updatedSearchText : searchText, completionHandler : completionHandler)
    }
    unsafe fn searchTemplate_selectedResult_completionHandler_(
        &self,
        searchTemplate: CPSearchTemplate,
        item: CPListItem,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, searchTemplate : searchTemplate, selectedResult : item, completionHandler : completionHandler)
    }
    unsafe fn searchTemplateSearchButtonPressed_(&self, searchTemplate: CPSearchTemplate)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, searchTemplateSearchButtonPressed : searchTemplate)
    }
}
pub type CPLimitableUserInterface = NSUInteger;
pub type CPContentStyle = NSUInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CPSessionConfiguration(pub id);
impl std::ops::Deref for CPSessionConfiguration {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CPSessionConfiguration {}
impl CPSessionConfiguration {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CPSessionConfiguration").unwrap(), alloc) })
    }
}
impl INSObject for CPSessionConfiguration {}
impl PNSObject for CPSessionConfiguration {}
impl std::convert::TryFrom<NSObject> for CPSessionConfiguration {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<CPSessionConfiguration, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CPSessionConfiguration").unwrap()) };
        if is_kind_of {
            Ok(CPSessionConfiguration(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to CPSessionConfiguration")
        }
    }
}
impl ICPSessionConfiguration for CPSessionConfiguration {}
pub trait ICPSessionConfiguration: Sized + std::ops::Deref {
    unsafe fn initWithDelegate_(&self, delegate: *mut u64) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithDelegate : delegate)
    }
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn limitedUserInterfaces(&self) -> CPLimitableUserInterface
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, limitedUserInterfaces)
    }
    unsafe fn contentStyle(&self) -> CPContentStyle
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, contentStyle)
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
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CPSessionConfiguration").unwrap(), new)
    }
}
pub trait PCPSessionConfigurationDelegate: Sized + std::ops::Deref {
    unsafe fn sessionConfiguration_limitedUserInterfacesChanged_(
        &self,
        sessionConfiguration: CPSessionConfiguration,
        limitedUserInterfaces: CPLimitableUserInterface,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, sessionConfiguration : sessionConfiguration, limitedUserInterfacesChanged : limitedUserInterfaces)
    }
    unsafe fn sessionConfiguration_contentStyleChanged_(
        &self,
        sessionConfiguration: CPSessionConfiguration,
        contentStyle: CPContentStyle,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, sessionConfiguration : sessionConfiguration, contentStyleChanged : contentStyle)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CPTabBarTemplate(pub id);
impl std::ops::Deref for CPTabBarTemplate {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CPTabBarTemplate {}
impl CPTabBarTemplate {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CPTabBarTemplate").unwrap(), alloc) })
    }
}
impl ICPTemplate for CPTabBarTemplate {}
impl PNSSecureCoding for CPTabBarTemplate {}
impl std::convert::TryFrom<CPTemplate> for CPTabBarTemplate {
    type Error = &'static str;
    fn try_from(parent: CPTemplate) -> Result<CPTabBarTemplate, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CPTabBarTemplate").unwrap()) };
        if is_kind_of {
            Ok(CPTabBarTemplate(parent.0))
        } else {
            Err("This CPTemplate cannot be downcasted to CPTabBarTemplate")
        }
    }
}
impl INSObject for CPTabBarTemplate {}
impl PNSObject for CPTabBarTemplate {}
impl ICPTabBarTemplate for CPTabBarTemplate {}
pub trait ICPTabBarTemplate: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn initWithTemplates_(&self, templates: NSArray) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithTemplates : templates)
    }
    unsafe fn updateTemplates_(&self, newTemplates: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, updateTemplates : newTemplates)
    }
    unsafe fn selectTemplate_(&self, newTemplate: CPTemplate)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, selectTemplate : newTemplate)
    }
    unsafe fn selectTemplateAtIndex_(&self, index: NSInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, selectTemplateAtIndex : index)
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
    unsafe fn templates(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, templates)
    }
    unsafe fn selectedTemplate(&self) -> CPTemplate
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, selectedTemplate)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CPTabBarTemplate").unwrap(), new)
    }
    unsafe fn maximumTabCount() -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CPTabBarTemplate").unwrap(), maximumTabCount)
    }
}
pub trait PCPTabBarTemplateDelegate: Sized + std::ops::Deref {
    unsafe fn tabBarTemplate_didSelectTemplate_(
        &self,
        tabBarTemplate: CPTabBarTemplate,
        selectedTemplate: CPTemplate,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, tabBarTemplate : tabBarTemplate, didSelectTemplate : selectedTemplate)
    }
}
pub trait PCPTemplateApplicationDashboardSceneDelegate: Sized + std::ops::Deref {
    unsafe fn templateApplicationDashboardScene_didConnectDashboardController_toWindow_(
        &self,
        templateApplicationDashboardScene: CPTemplateApplicationDashboardScene,
        dashboardController: CPDashboardController,
        window: UIWindow,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, templateApplicationDashboardScene : templateApplicationDashboardScene, didConnectDashboardController : dashboardController, toWindow : window)
    }
    unsafe fn templateApplicationDashboardScene_didDisconnectDashboardController_fromWindow_(
        &self,
        templateApplicationDashboardScene: CPTemplateApplicationDashboardScene,
        dashboardController: CPDashboardController,
        window: UIWindow,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, templateApplicationDashboardScene : templateApplicationDashboardScene, didDisconnectDashboardController : dashboardController, fromWindow : window)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CPTemplateApplicationDashboardScene(pub id);
impl std::ops::Deref for CPTemplateApplicationDashboardScene {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CPTemplateApplicationDashboardScene {}
impl CPTemplateApplicationDashboardScene {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CPTemplateApplicationDashboardScene").unwrap(), alloc) })
    }
}
impl IUIScene for CPTemplateApplicationDashboardScene {}
impl std::convert::TryFrom<UIScene> for CPTemplateApplicationDashboardScene {
    type Error = &'static str;
    fn try_from(parent: UIScene) -> Result<CPTemplateApplicationDashboardScene, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CPTemplateApplicationDashboardScene").unwrap())
        };
        if is_kind_of {
            Ok(CPTemplateApplicationDashboardScene(parent.0))
        } else {
            Err("This UIScene cannot be downcasted to CPTemplateApplicationDashboardScene")
        }
    }
}
impl IUIResponder for CPTemplateApplicationDashboardScene {}
impl PUIResponderStandardEditActions for CPTemplateApplicationDashboardScene {}
impl INSObject for CPTemplateApplicationDashboardScene {}
impl PNSObject for CPTemplateApplicationDashboardScene {}
impl ICPTemplateApplicationDashboardScene for CPTemplateApplicationDashboardScene {}
pub trait ICPTemplateApplicationDashboardScene: Sized + std::ops::Deref {
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
    unsafe fn dashboardController(&self) -> CPDashboardController
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, dashboardController)
    }
    unsafe fn dashboardWindow(&self) -> UIWindow
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, dashboardWindow)
    }
}
pub trait PCPTemplateApplicationInstrumentClusterSceneDelegate: Sized + std::ops::Deref {
    unsafe fn templateApplicationInstrumentClusterScene_didConnectInstrumentClusterController_(
        &self,
        templateApplicationInstrumentClusterScene: CPTemplateApplicationInstrumentClusterScene,
        instrumentClusterController: CPInstrumentClusterController,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, templateApplicationInstrumentClusterScene : templateApplicationInstrumentClusterScene, didConnectInstrumentClusterController : instrumentClusterController)
    }
    unsafe fn templateApplicationInstrumentClusterScene_didDisconnectInstrumentClusterController_(
        &self,
        templateApplicationInstrumentClusterScene: CPTemplateApplicationInstrumentClusterScene,
        instrumentClusterController: CPInstrumentClusterController,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, templateApplicationInstrumentClusterScene : templateApplicationInstrumentClusterScene, didDisconnectInstrumentClusterController : instrumentClusterController)
    }
    unsafe fn contentStyleDidChange_(&self, contentStyle: UIUserInterfaceStyle)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, contentStyleDidChange : contentStyle)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CPTemplateApplicationInstrumentClusterScene(pub id);
impl std::ops::Deref for CPTemplateApplicationInstrumentClusterScene {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CPTemplateApplicationInstrumentClusterScene {}
impl CPTemplateApplicationInstrumentClusterScene {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CPTemplateApplicationInstrumentClusterScene").unwrap(), alloc) })
    }
}
impl IUIScene for CPTemplateApplicationInstrumentClusterScene {}
impl std::convert::TryFrom<UIScene> for CPTemplateApplicationInstrumentClusterScene {
    type Error = &'static str;
    fn try_from(
        parent: UIScene,
    ) -> Result<CPTemplateApplicationInstrumentClusterScene, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CPTemplateApplicationInstrumentClusterScene").unwrap())
        };
        if is_kind_of {
            Ok(CPTemplateApplicationInstrumentClusterScene(parent.0))
        } else {
            Err("This UIScene cannot be downcasted to CPTemplateApplicationInstrumentClusterScene")
        }
    }
}
impl IUIResponder for CPTemplateApplicationInstrumentClusterScene {}
impl PUIResponderStandardEditActions for CPTemplateApplicationInstrumentClusterScene {}
impl INSObject for CPTemplateApplicationInstrumentClusterScene {}
impl PNSObject for CPTemplateApplicationInstrumentClusterScene {}
impl ICPTemplateApplicationInstrumentClusterScene for CPTemplateApplicationInstrumentClusterScene {}
pub trait ICPTemplateApplicationInstrumentClusterScene: Sized + std::ops::Deref {
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
    unsafe fn instrumentClusterController(&self) -> CPInstrumentClusterController
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, instrumentClusterController)
    }
    unsafe fn contentStyle(&self) -> UIUserInterfaceStyle
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, contentStyle)
    }
}
pub trait PCPTemplateApplicationSceneDelegate: Sized + std::ops::Deref {
    unsafe fn templateApplicationScene_didConnectInterfaceController_toWindow_(
        &self,
        templateApplicationScene: CPTemplateApplicationScene,
        interfaceController: CPInterfaceController,
        window: CPWindow,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, templateApplicationScene : templateApplicationScene, didConnectInterfaceController : interfaceController, toWindow : window)
    }
    unsafe fn templateApplicationScene_didConnectInterfaceController_(
        &self,
        templateApplicationScene: CPTemplateApplicationScene,
        interfaceController: CPInterfaceController,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, templateApplicationScene : templateApplicationScene, didConnectInterfaceController : interfaceController)
    }
    unsafe fn templateApplicationScene_didDisconnectInterfaceController_fromWindow_(
        &self,
        templateApplicationScene: CPTemplateApplicationScene,
        interfaceController: CPInterfaceController,
        window: CPWindow,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, templateApplicationScene : templateApplicationScene, didDisconnectInterfaceController : interfaceController, fromWindow : window)
    }
    unsafe fn templateApplicationScene_didDisconnectInterfaceController_(
        &self,
        templateApplicationScene: CPTemplateApplicationScene,
        interfaceController: CPInterfaceController,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, templateApplicationScene : templateApplicationScene, didDisconnectInterfaceController : interfaceController)
    }
    unsafe fn templateApplicationScene_didSelectNavigationAlert_(
        &self,
        templateApplicationScene: CPTemplateApplicationScene,
        navigationAlert: CPNavigationAlert,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, templateApplicationScene : templateApplicationScene, didSelectNavigationAlert : navigationAlert)
    }
    unsafe fn templateApplicationScene_didSelectManeuver_(
        &self,
        templateApplicationScene: CPTemplateApplicationScene,
        maneuver: CPManeuver,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, templateApplicationScene : templateApplicationScene, didSelectManeuver : maneuver)
    }
    unsafe fn contentStyleDidChange_(&self, contentStyle: UIUserInterfaceStyle)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, contentStyleDidChange : contentStyle)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CPTemplateApplicationScene(pub id);
impl std::ops::Deref for CPTemplateApplicationScene {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CPTemplateApplicationScene {}
impl CPTemplateApplicationScene {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CPTemplateApplicationScene").unwrap(), alloc) })
    }
}
impl IUIScene for CPTemplateApplicationScene {}
impl std::convert::TryFrom<UIScene> for CPTemplateApplicationScene {
    type Error = &'static str;
    fn try_from(parent: UIScene) -> Result<CPTemplateApplicationScene, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CPTemplateApplicationScene").unwrap()) };
        if is_kind_of {
            Ok(CPTemplateApplicationScene(parent.0))
        } else {
            Err("This UIScene cannot be downcasted to CPTemplateApplicationScene")
        }
    }
}
impl IUIResponder for CPTemplateApplicationScene {}
impl PUIResponderStandardEditActions for CPTemplateApplicationScene {}
impl INSObject for CPTemplateApplicationScene {}
impl PNSObject for CPTemplateApplicationScene {}
impl ICPTemplateApplicationScene for CPTemplateApplicationScene {}
pub trait ICPTemplateApplicationScene: Sized + std::ops::Deref {
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
    unsafe fn interfaceController(&self) -> CPInterfaceController
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, interfaceController)
    }
    unsafe fn carWindow(&self) -> CPWindow
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, carWindow)
    }
    unsafe fn contentStyle(&self) -> UIUserInterfaceStyle
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, contentStyle)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CPVoiceControlState(pub id);
impl std::ops::Deref for CPVoiceControlState {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CPVoiceControlState {}
impl CPVoiceControlState {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CPVoiceControlState").unwrap(), alloc) })
    }
}
impl PNSSecureCoding for CPVoiceControlState {}
impl INSObject for CPVoiceControlState {}
impl PNSObject for CPVoiceControlState {}
impl std::convert::TryFrom<NSObject> for CPVoiceControlState {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<CPVoiceControlState, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CPVoiceControlState").unwrap()) };
        if is_kind_of {
            Ok(CPVoiceControlState(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to CPVoiceControlState")
        }
    }
}
impl ICPVoiceControlState for CPVoiceControlState {}
pub trait ICPVoiceControlState: Sized + std::ops::Deref {
    unsafe fn initWithIdentifier_titleVariants_image_repeats_(
        &self,
        identifier: NSString,
        titleVariants: NSArray,
        image: UIImage,
        repeats: BOOL,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithIdentifier : identifier, titleVariants : titleVariants, image : image, repeats : repeats)
    }
    unsafe fn titleVariants(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, titleVariants)
    }
    unsafe fn image(&self) -> UIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, image)
    }
    unsafe fn identifier(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, identifier)
    }
    unsafe fn repeats(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, repeats)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CPVoiceControlTemplate(pub id);
impl std::ops::Deref for CPVoiceControlTemplate {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CPVoiceControlTemplate {}
impl CPVoiceControlTemplate {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CPVoiceControlTemplate").unwrap(), alloc) })
    }
}
impl ICPTemplate for CPVoiceControlTemplate {}
impl PNSSecureCoding for CPVoiceControlTemplate {}
impl std::convert::TryFrom<CPTemplate> for CPVoiceControlTemplate {
    type Error = &'static str;
    fn try_from(parent: CPTemplate) -> Result<CPVoiceControlTemplate, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CPVoiceControlTemplate").unwrap()) };
        if is_kind_of {
            Ok(CPVoiceControlTemplate(parent.0))
        } else {
            Err("This CPTemplate cannot be downcasted to CPVoiceControlTemplate")
        }
    }
}
impl INSObject for CPVoiceControlTemplate {}
impl PNSObject for CPVoiceControlTemplate {}
impl ICPVoiceControlTemplate for CPVoiceControlTemplate {}
pub trait ICPVoiceControlTemplate: Sized + std::ops::Deref {
    unsafe fn initWithVoiceControlStates_(&self, voiceControlStates: NSArray) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithVoiceControlStates : voiceControlStates)
    }
    unsafe fn activateVoiceControlStateWithIdentifier_(&self, identifier: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, activateVoiceControlStateWithIdentifier : identifier)
    }
    unsafe fn voiceControlStates(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, voiceControlStates)
    }
    unsafe fn activeStateIdentifier(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, activeStateIdentifier)
    }
}
unsafe extern "C" {
    pub static CPButtonMaximumImageSize: CGSize;
}
unsafe extern "C" {
    pub static CPGridTemplateMaximumItems: NSUInteger;
}
unsafe extern "C" {
    pub static CarPlayErrorDomain: NSString;
}
unsafe extern "C" {
    pub static CPMaximumListSectionImageSize: CGSize;
}
unsafe extern "C" {
    pub static CPMaximumNumberOfGridImages: NSUInteger;
}
unsafe extern "C" {
    pub static CPMaximumMessageItemImageSize: CGSize;
}
unsafe extern "C" {
    pub static CPMaximumMessageItemLeadingDetailTextImageSize: CGSize;
}
unsafe extern "C" {
    pub fn NSStringFromCPManeuverType(maneuverType: CPManeuverType) -> NSString;
}
unsafe extern "C" {
    pub fn NSStringFromCPJunctionType(junctionType: CPJunctionType) -> NSString;
}
unsafe extern "C" {
    pub fn NSStringFromCPTrafficSide(trafficSide: CPTrafficSide) -> NSString;
}
unsafe extern "C" {
    pub fn NSStringFromCPLaneStatus(laneStatus: CPLaneStatus) -> NSString;
}
unsafe extern "C" {
    pub static CPNowPlayingButtonMaximumImageSize: CGSize;
}
unsafe extern "C" {
    pub static CPTemplateApplicationDashboardSceneSessionRoleApplication: UISceneSessionRole;
}
unsafe extern "C" {
    pub static CPTemplateApplicationInstrumentClusterSceneSessionRoleApplication:
        UISceneSessionRole;
}
unsafe extern "C" {
    pub static CPTemplateApplicationSceneSessionRoleApplication: UISceneSessionRole;
}

unsafe impl objc2::encode::RefEncode for MKOverlayView {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MKOverlayView {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MKIconStyle {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MKIconStyle {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MKReverseGeocoder {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MKReverseGeocoder {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MKOverlayPathView {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MKOverlayPathView {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MKPolygonView {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MKPolygonView {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MKPolylineView {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MKPolylineView {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MKCircleView {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MKCircleView {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MKScaleView {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MKScaleView {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MKUserTrackingBarButtonItem {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MKUserTrackingBarButtonItem {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MKUserTrackingButton {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MKUserTrackingButton {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CPAlertAction {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CPAlertAction {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CPTemplate {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CPTemplate {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CPActionSheetTemplate {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CPActionSheetTemplate {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CPAlertTemplate {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CPAlertTemplate {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CPBarButton {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CPBarButton {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CPButton {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CPButton {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CPContact {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CPContact {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CPContactCallButton {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CPContactCallButton {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CPContactMessageButton {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CPContactMessageButton {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CPContactDirectionsButton {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CPContactDirectionsButton {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CPContactTemplate {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CPContactTemplate {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CPDashboardButton {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CPDashboardButton {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CPDashboardController {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CPDashboardController {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CPMessageGridItemConfiguration {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CPMessageGridItemConfiguration {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CPGridButton {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CPGridButton {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CPGridTemplate {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CPGridTemplate {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CPImageSet {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CPImageSet {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CPTextButton {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CPTextButton {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CPInformationItem {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CPInformationItem {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CPInformationRatingItem {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CPInformationRatingItem {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CPInformationTemplate {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CPInformationTemplate {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CPWindow {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CPWindow {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CPInterfaceController {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CPInterfaceController {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CPInstrumentClusterController {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CPInstrumentClusterController {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CPListImageRowItemElement {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CPListImageRowItemElement {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CPListImageRowItemCardElement {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CPListImageRowItemCardElement {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CPListImageRowItemCondensedElement {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CPListImageRowItemCondensedElement {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CPListImageRowItemGridElement {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CPListImageRowItemGridElement {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CPListImageRowItemImageGridElement {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CPListImageRowItemImageGridElement {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CPListImageRowItemRowElement {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CPListImageRowItemRowElement {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CPListSection {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CPListSection {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CPListImageRowItem {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CPListImageRowItem {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CPListItem {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CPListItem {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CPAssistantCellConfiguration {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CPAssistantCellConfiguration {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CPListTemplate {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CPListTemplate {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CPLane {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CPLane {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CPLaneGuidance {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CPLaneGuidance {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CPTravelEstimates {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CPTravelEstimates {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CPManeuver {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CPManeuver {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CPMapButton {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CPMapButton {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CPNavigationAlert {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CPNavigationAlert {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CPRouteInformation {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CPRouteInformation {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CPRouteChoice {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CPRouteChoice {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CPTrip {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CPTrip {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CPNavigationSession {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CPNavigationSession {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CPTripPreviewTextConfiguration {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CPTripPreviewTextConfiguration {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CPMapTemplate {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CPMapTemplate {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CPMessageComposeBarButton {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CPMessageComposeBarButton {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CPMessageListItemLeadingConfiguration {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CPMessageListItemLeadingConfiguration {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CPMessageListItemTrailingConfiguration {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CPMessageListItemTrailingConfiguration {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CPMessageListItem {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CPMessageListItem {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CPNowPlayingButton {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CPNowPlayingButton {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CPNowPlayingShuffleButton {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CPNowPlayingShuffleButton {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CPNowPlayingAddToLibraryButton {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CPNowPlayingAddToLibraryButton {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CPNowPlayingMoreButton {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CPNowPlayingMoreButton {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CPNowPlayingPlaybackRateButton {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CPNowPlayingPlaybackRateButton {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CPNowPlayingRepeatButton {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CPNowPlayingRepeatButton {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CPNowPlayingImageButton {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CPNowPlayingImageButton {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CPNowPlayingMode {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CPNowPlayingMode {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CPNowPlayingModeSports {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CPNowPlayingModeSports {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CPNowPlayingSportsTeam {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CPNowPlayingSportsTeam {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CPNowPlayingSportsEventStatus {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CPNowPlayingSportsEventStatus {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CPNowPlayingSportsClock {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CPNowPlayingSportsClock {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CPNowPlayingSportsTeamLogo {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CPNowPlayingSportsTeamLogo {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CPNowPlayingTemplate {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CPNowPlayingTemplate {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CPPointOfInterest {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CPPointOfInterest {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CPPointOfInterestTemplate {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CPPointOfInterestTemplate {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CPSearchTemplate {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CPSearchTemplate {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CPSessionConfiguration {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CPSessionConfiguration {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CPTabBarTemplate {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CPTabBarTemplate {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CPTemplateApplicationDashboardScene {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CPTemplateApplicationDashboardScene {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CPTemplateApplicationInstrumentClusterScene {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CPTemplateApplicationInstrumentClusterScene {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CPTemplateApplicationScene {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CPTemplateApplicationScene {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CPVoiceControlState {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CPVoiceControlState {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CPVoiceControlTemplate {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CPVoiceControlTemplate {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
