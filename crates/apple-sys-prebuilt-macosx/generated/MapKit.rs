#[allow(unused_imports)]
use crate::objc::*;
#[allow(unused_imports)]
use crate::AppKit::*;
#[allow(unused_imports)]
use crate::Contacts::*;
#[allow(unused_imports)]
use crate::CoreData::*;
#[allow(unused_imports)]
use crate::CoreFoundation::*;
#[allow(unused_imports)]
use crate::CoreGraphics::*;
#[allow(unused_imports)]
use crate::CoreLocation::*;
#[allow(unused_imports)]
use crate::Foundation::*;

#[allow(unused_imports)]
use objc2::msg_send;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MKAddress(pub id);
impl std::ops::Deref for MKAddress {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MKAddress {}
impl MKAddress {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MKAddress").unwrap(), alloc) })
    }
}
impl INSObject for MKAddress {}
impl PNSObject for MKAddress {}
impl std::convert::TryFrom<NSObject> for MKAddress {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<MKAddress, Self::Error> {
        let is_kind_of: bool = unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MKAddress").unwrap()) };
        if is_kind_of {
            Ok(MKAddress(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to MKAddress")
        }
    }
}
impl IMKAddress for MKAddress {}
pub trait IMKAddress: Sized + std::ops::Deref {
    unsafe fn initWithFullAddress_shortAddress_(
        &self,
        fullAddress: NSString,
        shortAddress: NSString,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithFullAddress : fullAddress, shortAddress : shortAddress)
    }
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn fullAddress(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, fullAddress)
    }
    unsafe fn shortAddress(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, shortAddress)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MKAddress").unwrap(), new)
    }
}
pub type MKAddressRepresentationsContextStyle = NSInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MKAddressRepresentations(pub id);
impl std::ops::Deref for MKAddressRepresentations {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MKAddressRepresentations {}
impl MKAddressRepresentations {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MKAddressRepresentations").unwrap(), alloc) })
    }
}
impl INSObject for MKAddressRepresentations {}
impl PNSObject for MKAddressRepresentations {}
impl std::convert::TryFrom<NSObject> for MKAddressRepresentations {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<MKAddressRepresentations, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MKAddressRepresentations").unwrap()) };
        if is_kind_of {
            Ok(MKAddressRepresentations(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to MKAddressRepresentations")
        }
    }
}
impl IMKAddressRepresentations for MKAddressRepresentations {}
pub trait IMKAddressRepresentations: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn fullAddressIncludingRegion_singleLine_(
        &self,
        includingRegion: BOOL,
        singleLine: BOOL,
    ) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, fullAddressIncludingRegion : includingRegion, singleLine : singleLine)
    }
    unsafe fn cityWithContextUsingStyle_(
        &self,
        style: MKAddressRepresentationsContextStyle,
    ) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, cityWithContextUsingStyle : style)
    }
    unsafe fn cityName(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, cityName)
    }
    unsafe fn cityWithContext(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, cityWithContext)
    }
    unsafe fn regionName(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, regionName)
    }
    unsafe fn regionCode(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, regionCode)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MKAddressRepresentations").unwrap(), new)
    }
}
pub type MKAddressFilterOption = NSUInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MKAddressFilter(pub id);
impl std::ops::Deref for MKAddressFilter {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MKAddressFilter {}
impl MKAddressFilter {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MKAddressFilter").unwrap(), alloc) })
    }
}
impl PNSSecureCoding for MKAddressFilter {}
impl PNSCopying for MKAddressFilter {}
impl INSObject for MKAddressFilter {}
impl PNSObject for MKAddressFilter {}
impl std::convert::TryFrom<NSObject> for MKAddressFilter {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<MKAddressFilter, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MKAddressFilter").unwrap()) };
        if is_kind_of {
            Ok(MKAddressFilter(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to MKAddressFilter")
        }
    }
}
impl IMKAddressFilter for MKAddressFilter {}
pub trait IMKAddressFilter: Sized + std::ops::Deref {
    unsafe fn initIncludingOptions_(&self, options: MKAddressFilterOption) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initIncludingOptions : options)
    }
    unsafe fn initExcludingOptions_(&self, options: MKAddressFilterOption) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initExcludingOptions : options)
    }
    unsafe fn includesOptions_(&self, options: MKAddressFilterOption) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, includesOptions : options)
    }
    unsafe fn excludesOptions_(&self, options: MKAddressFilterOption) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, excludesOptions : options)
    }
    unsafe fn filterIncludingAll() -> MKAddressFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MKAddressFilter").unwrap(), filterIncludingAll)
    }
    unsafe fn filterExcludingAll() -> MKAddressFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MKAddressFilter").unwrap(), filterExcludingAll)
    }
}
pub trait PMKAnnotation: Sized + std::ops::Deref {
    unsafe fn setCoordinate_(&self, newCoordinate: CLLocationCoordinate2D)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCoordinate : newCoordinate)
    }
    unsafe fn coordinate(&self) -> CLLocationCoordinate2D
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, coordinate)
    }
    unsafe fn title(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, title)
    }
    unsafe fn subtitle(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, subtitle)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MKShape(pub id);
impl std::ops::Deref for MKShape {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MKShape {}
impl MKShape {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MKShape").unwrap(), alloc) })
    }
}
impl PMKAnnotation for MKShape {}
impl INSObject for MKShape {}
impl PNSObject for MKShape {}
impl std::convert::TryFrom<NSObject> for MKShape {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<MKShape, Self::Error> {
        let is_kind_of: bool = unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MKShape").unwrap()) };
        if is_kind_of {
            Ok(MKShape(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to MKShape")
        }
    }
}
impl IMKShape for MKShape {}
pub trait IMKShape: Sized + std::ops::Deref {
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
pub type MKMapType = NSUInteger;
pub type MKErrorCode = NSUInteger;
pub type MKFeatureVisibility = NSInteger;
pub type MKLocalSearchRegionPriority = NSInteger;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct MKCoordinateSpan {
    pub latitudeDelta: CLLocationDegrees,
    pub longitudeDelta: CLLocationDegrees,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct MKCoordinateRegion {
    pub center: CLLocationCoordinate2D,
    pub span: MKCoordinateSpan,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct MKMapPoint {
    pub x: f64,
    pub y: f64,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct MKMapSize {
    pub width: f64,
    pub height: f64,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct MKMapRect {
    pub origin: MKMapPoint,
    pub size: MKMapSize,
}
pub type MKZoomScale = CGFloat;
pub trait NSValue_NSValueMapKitGeometryExtensions: Sized + std::ops::Deref {
    unsafe fn MKCoordinateValue(&self) -> CLLocationCoordinate2D
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, MKCoordinateValue)
    }
    unsafe fn MKCoordinateSpanValue(&self) -> MKCoordinateSpan
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, MKCoordinateSpanValue)
    }
    unsafe fn valueWithMKCoordinate_(coordinate: CLLocationCoordinate2D) -> NSValue
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NSValue").unwrap(), valueWithMKCoordinate : coordinate)
    }
    unsafe fn valueWithMKCoordinateSpan_(span: MKCoordinateSpan) -> NSValue
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"NSValue").unwrap(), valueWithMKCoordinateSpan : span)
    }
}
pub trait PMKOverlay: Sized + std::ops::Deref {
    unsafe fn intersectsMapRect_(&self, mapRect: MKMapRect) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, intersectsMapRect : mapRect)
    }
    unsafe fn canReplaceMapContent(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, canReplaceMapContent)
    }
    unsafe fn coordinate(&self) -> CLLocationCoordinate2D
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, coordinate)
    }
    unsafe fn boundingMapRect(&self) -> MKMapRect
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, boundingMapRect)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MKCircle(pub id);
impl std::ops::Deref for MKCircle {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MKCircle {}
impl MKCircle {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MKCircle").unwrap(), alloc) })
    }
}
impl PMKOverlay for MKCircle {}
impl IMKShape for MKCircle {}
impl PMKAnnotation for MKCircle {}
impl From<MKCircle> for MKShape {
    fn from(child: MKCircle) -> MKShape {
        MKShape(child.0)
    }
}
impl std::convert::TryFrom<MKShape> for MKCircle {
    type Error = &'static str;
    fn try_from(parent: MKShape) -> Result<MKCircle, Self::Error> {
        let is_kind_of: bool = unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MKCircle").unwrap()) };
        if is_kind_of {
            Ok(MKCircle(parent.0))
        } else {
            Err("This MKShape cannot be downcasted to MKCircle")
        }
    }
}
impl INSObject for MKCircle {}
impl PNSObject for MKCircle {}
impl IMKCircle for MKCircle {}
pub trait IMKCircle: Sized + std::ops::Deref {
    unsafe fn coordinate(&self) -> CLLocationCoordinate2D
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, coordinate)
    }
    unsafe fn radius(&self) -> CLLocationDistance
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, radius)
    }
    unsafe fn boundingMapRect(&self) -> MKMapRect
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, boundingMapRect)
    }
    unsafe fn circleWithCenterCoordinate_radius_(
        coord: CLLocationCoordinate2D,
        radius: CLLocationDistance,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MKCircle").unwrap(), circleWithCenterCoordinate : coord, radius : radius)
    }
    unsafe fn circleWithMapRect_(mapRect: MKMapRect) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MKCircle").unwrap(), circleWithMapRect : mapRect)
    }
}
pub type MKDirectionsHandler = *mut ::std::os::raw::c_void;
pub type MKETAHandler = *mut ::std::os::raw::c_void;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MKDirections(pub id);
impl std::ops::Deref for MKDirections {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MKDirections {}
impl MKDirections {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MKDirections").unwrap(), alloc) })
    }
}
impl INSObject for MKDirections {}
impl PNSObject for MKDirections {}
impl std::convert::TryFrom<NSObject> for MKDirections {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<MKDirections, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MKDirections").unwrap()) };
        if is_kind_of {
            Ok(MKDirections(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to MKDirections")
        }
    }
}
impl IMKDirections for MKDirections {}
pub trait IMKDirections: Sized + std::ops::Deref {
    unsafe fn initWithRequest_(&self, request: MKDirectionsRequest) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithRequest : request)
    }
    unsafe fn calculateDirectionsWithCompletionHandler_(
        &self,
        completionHandler: MKDirectionsHandler,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, calculateDirectionsWithCompletionHandler : completionHandler)
    }
    unsafe fn calculateETAWithCompletionHandler_(&self, completionHandler: MKETAHandler)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, calculateETAWithCompletionHandler : completionHandler)
    }
    unsafe fn cancel(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, cancel)
    }
    unsafe fn isCalculating(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isCalculating)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MKMapItemIdentifier(pub id);
impl std::ops::Deref for MKMapItemIdentifier {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MKMapItemIdentifier {}
impl MKMapItemIdentifier {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MKMapItemIdentifier").unwrap(), alloc) })
    }
}
impl PNSCopying for MKMapItemIdentifier {}
impl PNSSecureCoding for MKMapItemIdentifier {}
impl INSObject for MKMapItemIdentifier {}
impl PNSObject for MKMapItemIdentifier {}
impl std::convert::TryFrom<NSObject> for MKMapItemIdentifier {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<MKMapItemIdentifier, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MKMapItemIdentifier").unwrap()) };
        if is_kind_of {
            Ok(MKMapItemIdentifier(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to MKMapItemIdentifier")
        }
    }
}
impl IMKMapItemIdentifier for MKMapItemIdentifier {}
pub trait IMKMapItemIdentifier: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn initWithIdentifierString_(&self, string: NSString) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithIdentifierString : string)
    }
    unsafe fn identifierString(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, identifierString)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MKMapItemIdentifier").unwrap(), new)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MKPlacemark(pub id);
impl std::ops::Deref for MKPlacemark {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MKPlacemark {}
impl MKPlacemark {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MKPlacemark").unwrap(), alloc) })
    }
}
impl PMKAnnotation for MKPlacemark {}
impl ICLPlacemark for MKPlacemark {}
impl PNSCopying for MKPlacemark {}
impl PNSSecureCoding for MKPlacemark {}
impl std::convert::TryFrom<CLPlacemark> for MKPlacemark {
    type Error = &'static str;
    fn try_from(parent: CLPlacemark) -> Result<MKPlacemark, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MKPlacemark").unwrap()) };
        if is_kind_of {
            Ok(MKPlacemark(parent.0))
        } else {
            Err("This CLPlacemark cannot be downcasted to MKPlacemark")
        }
    }
}
impl INSObject for MKPlacemark {}
impl PNSObject for MKPlacemark {}
impl IMKPlacemark for MKPlacemark {}
pub trait IMKPlacemark: Sized + std::ops::Deref {
    unsafe fn initWithCoordinate_(&self, coordinate: CLLocationCoordinate2D) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithCoordinate : coordinate)
    }
    unsafe fn initWithCoordinate_addressDictionary_(
        &self,
        coordinate: CLLocationCoordinate2D,
        addressDictionary: NSDictionary,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithCoordinate : coordinate, addressDictionary : addressDictionary)
    }
    unsafe fn initWithCoordinate_postalAddress_(
        &self,
        coordinate: CLLocationCoordinate2D,
        postalAddress: CNPostalAddress,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithCoordinate : coordinate, postalAddress : postalAddress)
    }
    unsafe fn countryCode(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, countryCode)
    }
}
pub type MKPointOfInterestCategory = NSString;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MKMapItem(pub id);
impl std::ops::Deref for MKMapItem {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MKMapItem {}
impl MKMapItem {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MKMapItem").unwrap(), alloc) })
    }
}
impl INSObject for MKMapItem {}
impl PNSObject for MKMapItem {}
impl std::convert::TryFrom<NSObject> for MKMapItem {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<MKMapItem, Self::Error> {
        let is_kind_of: bool = unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MKMapItem").unwrap()) };
        if is_kind_of {
            Ok(MKMapItem(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to MKMapItem")
        }
    }
}
impl IMKMapItem for MKMapItem {}
pub trait IMKMapItem: Sized + std::ops::Deref {
    unsafe fn initWithPlacemark_(&self, placemark: MKPlacemark) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithPlacemark : placemark)
    }
    unsafe fn initWithLocation_address_(
        &self,
        location: CLLocation,
        address: MKAddress,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithLocation : location, address : address)
    }
    unsafe fn openInMapsWithLaunchOptions_(&self, launchOptions: NSDictionary) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, openInMapsWithLaunchOptions : launchOptions)
    }
    unsafe fn openInMapsWithLaunchOptions_completionHandler_(
        &self,
        launchOptions: NSDictionary,
        completion: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, openInMapsWithLaunchOptions : launchOptions, completionHandler : completion)
    }
    unsafe fn identifier(&self) -> MKMapItemIdentifier
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, identifier)
    }
    unsafe fn alternateIdentifiers(&self) -> NSSet
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, alternateIdentifiers)
    }
    unsafe fn placemark(&self) -> MKPlacemark
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, placemark)
    }
    unsafe fn isCurrentLocation(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isCurrentLocation)
    }
    unsafe fn location(&self) -> CLLocation
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, location)
    }
    unsafe fn address(&self) -> MKAddress
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, address)
    }
    unsafe fn addressRepresentations(&self) -> MKAddressRepresentations
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, addressRepresentations)
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
    unsafe fn phoneNumber(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, phoneNumber)
    }
    unsafe fn setPhoneNumber_(&self, phoneNumber: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPhoneNumber : phoneNumber)
    }
    unsafe fn url(&self) -> NSURL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, url)
    }
    unsafe fn setUrl_(&self, url: NSURL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setUrl : url)
    }
    unsafe fn timeZone(&self) -> NSTimeZone
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, timeZone)
    }
    unsafe fn setTimeZone_(&self, timeZone: NSTimeZone)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setTimeZone : timeZone)
    }
    unsafe fn pointOfInterestCategory(&self) -> MKPointOfInterestCategory
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, pointOfInterestCategory)
    }
    unsafe fn setPointOfInterestCategory_(&self, pointOfInterestCategory: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPointOfInterestCategory : pointOfInterestCategory)
    }
    unsafe fn mapItemForCurrentLocation() -> MKMapItem
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MKMapItem").unwrap(), mapItemForCurrentLocation)
    }
    unsafe fn openMapsWithItems_launchOptions_(
        mapItems: NSArray,
        launchOptions: NSDictionary,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MKMapItem").unwrap(), openMapsWithItems : mapItems, launchOptions : launchOptions)
    }
    unsafe fn openMapsWithItems_launchOptions_completionHandler_(
        mapItems: NSArray,
        launchOptions: NSDictionary,
        completion: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MKMapItem").unwrap(), openMapsWithItems : mapItems, launchOptions : launchOptions, completionHandler : completion)
    }
}
impl MKMapItem_MKMapItemSerialization for MKMapItem {}
pub trait MKMapItem_MKMapItemSerialization: Sized + std::ops::Deref {}
pub type MKDirectionsTransportType = NSUInteger;
pub type MKDirectionsRoutePreference = NSInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MKDirectionsRequest(pub id);
impl std::ops::Deref for MKDirectionsRequest {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MKDirectionsRequest {}
impl MKDirectionsRequest {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MKDirectionsRequest").unwrap(), alloc) })
    }
}
impl INSObject for MKDirectionsRequest {}
impl PNSObject for MKDirectionsRequest {}
impl std::convert::TryFrom<NSObject> for MKDirectionsRequest {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<MKDirectionsRequest, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MKDirectionsRequest").unwrap()) };
        if is_kind_of {
            Ok(MKDirectionsRequest(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to MKDirectionsRequest")
        }
    }
}
impl IMKDirectionsRequest for MKDirectionsRequest {}
pub trait IMKDirectionsRequest: Sized + std::ops::Deref {
    unsafe fn setSource_(&self, source: MKMapItem)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSource : source)
    }
    unsafe fn setDestination_(&self, destination: MKMapItem)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDestination : destination)
    }
    unsafe fn source(&self) -> MKMapItem
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, source)
    }
    unsafe fn destination(&self) -> MKMapItem
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, destination)
    }
}
impl MKDirectionsRequest_MKRequestOptions for MKDirectionsRequest {}
pub trait MKDirectionsRequest_MKRequestOptions: Sized + std::ops::Deref {
    unsafe fn transportType(&self) -> MKDirectionsTransportType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, transportType)
    }
    unsafe fn setTransportType_(&self, transportType: MKDirectionsTransportType)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setTransportType : transportType)
    }
    unsafe fn requestsAlternateRoutes(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, requestsAlternateRoutes)
    }
    unsafe fn setRequestsAlternateRoutes_(&self, requestsAlternateRoutes: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRequestsAlternateRoutes : requestsAlternateRoutes)
    }
    unsafe fn departureDate(&self) -> NSDate
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, departureDate)
    }
    unsafe fn setDepartureDate_(&self, departureDate: NSDate)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDepartureDate : departureDate)
    }
    unsafe fn arrivalDate(&self) -> NSDate
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, arrivalDate)
    }
    unsafe fn setArrivalDate_(&self, arrivalDate: NSDate)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setArrivalDate : arrivalDate)
    }
    unsafe fn tollPreference(&self) -> MKDirectionsRoutePreference
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, tollPreference)
    }
    unsafe fn setTollPreference_(&self, tollPreference: MKDirectionsRoutePreference)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setTollPreference : tollPreference)
    }
    unsafe fn highwayPreference(&self) -> MKDirectionsRoutePreference
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, highwayPreference)
    }
    unsafe fn setHighwayPreference_(&self, highwayPreference: MKDirectionsRoutePreference)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setHighwayPreference : highwayPreference)
    }
}
impl MKDirectionsRequest_MKDirectionsURL for MKDirectionsRequest {}
pub trait MKDirectionsRequest_MKDirectionsURL: Sized + std::ops::Deref {
    unsafe fn initWithContentsOfURL_(&self, url: NSURL) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithContentsOfURL : url)
    }
    unsafe fn isDirectionsRequestURL_(url: NSURL) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MKDirectionsRequest").unwrap(), isDirectionsRequestURL : url)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MKDirectionsResponse(pub id);
impl std::ops::Deref for MKDirectionsResponse {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MKDirectionsResponse {}
impl MKDirectionsResponse {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MKDirectionsResponse").unwrap(), alloc) })
    }
}
impl INSObject for MKDirectionsResponse {}
impl PNSObject for MKDirectionsResponse {}
impl std::convert::TryFrom<NSObject> for MKDirectionsResponse {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<MKDirectionsResponse, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MKDirectionsResponse").unwrap()) };
        if is_kind_of {
            Ok(MKDirectionsResponse(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to MKDirectionsResponse")
        }
    }
}
impl IMKDirectionsResponse for MKDirectionsResponse {}
pub trait IMKDirectionsResponse: Sized + std::ops::Deref {
    unsafe fn source(&self) -> MKMapItem
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, source)
    }
    unsafe fn destination(&self) -> MKMapItem
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, destination)
    }
    unsafe fn routes(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, routes)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MKRoute(pub id);
impl std::ops::Deref for MKRoute {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MKRoute {}
impl MKRoute {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MKRoute").unwrap(), alloc) })
    }
}
impl INSObject for MKRoute {}
impl PNSObject for MKRoute {}
impl std::convert::TryFrom<NSObject> for MKRoute {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<MKRoute, Self::Error> {
        let is_kind_of: bool = unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MKRoute").unwrap()) };
        if is_kind_of {
            Ok(MKRoute(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to MKRoute")
        }
    }
}
impl IMKRoute for MKRoute {}
pub trait IMKRoute: Sized + std::ops::Deref {
    unsafe fn name(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, name)
    }
    unsafe fn advisoryNotices(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, advisoryNotices)
    }
    unsafe fn distance(&self) -> CLLocationDistance
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, distance)
    }
    unsafe fn expectedTravelTime(&self) -> NSTimeInterval
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, expectedTravelTime)
    }
    unsafe fn transportType(&self) -> MKDirectionsTransportType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, transportType)
    }
    unsafe fn polyline(&self) -> MKPolyline
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, polyline)
    }
    unsafe fn steps(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, steps)
    }
    unsafe fn hasTolls(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, hasTolls)
    }
    unsafe fn hasHighways(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, hasHighways)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MKRouteStep(pub id);
impl std::ops::Deref for MKRouteStep {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MKRouteStep {}
impl MKRouteStep {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MKRouteStep").unwrap(), alloc) })
    }
}
impl INSObject for MKRouteStep {}
impl PNSObject for MKRouteStep {}
impl std::convert::TryFrom<NSObject> for MKRouteStep {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<MKRouteStep, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MKRouteStep").unwrap()) };
        if is_kind_of {
            Ok(MKRouteStep(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to MKRouteStep")
        }
    }
}
impl IMKRouteStep for MKRouteStep {}
pub trait IMKRouteStep: Sized + std::ops::Deref {
    unsafe fn instructions(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, instructions)
    }
    unsafe fn notice(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, notice)
    }
    unsafe fn polyline(&self) -> MKPolyline
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, polyline)
    }
    unsafe fn distance(&self) -> CLLocationDistance
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, distance)
    }
    unsafe fn transportType(&self) -> MKDirectionsTransportType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, transportType)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MKETAResponse(pub id);
impl std::ops::Deref for MKETAResponse {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MKETAResponse {}
impl MKETAResponse {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MKETAResponse").unwrap(), alloc) })
    }
}
impl INSObject for MKETAResponse {}
impl PNSObject for MKETAResponse {}
impl std::convert::TryFrom<NSObject> for MKETAResponse {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<MKETAResponse, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MKETAResponse").unwrap()) };
        if is_kind_of {
            Ok(MKETAResponse(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to MKETAResponse")
        }
    }
}
impl IMKETAResponse for MKETAResponse {}
pub trait IMKETAResponse: Sized + std::ops::Deref {
    unsafe fn source(&self) -> MKMapItem
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, source)
    }
    unsafe fn destination(&self) -> MKMapItem
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, destination)
    }
    unsafe fn expectedTravelTime(&self) -> NSTimeInterval
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, expectedTravelTime)
    }
    unsafe fn distance(&self) -> CLLocationDistance
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, distance)
    }
    unsafe fn expectedArrivalDate(&self) -> NSDate
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, expectedArrivalDate)
    }
    unsafe fn expectedDepartureDate(&self) -> NSDate
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, expectedDepartureDate)
    }
    unsafe fn transportType(&self) -> MKDirectionsTransportType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, transportType)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MKDistanceFormatter(pub id);
impl std::ops::Deref for MKDistanceFormatter {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MKDistanceFormatter {}
impl MKDistanceFormatter {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MKDistanceFormatter").unwrap(), alloc) })
    }
}
impl INSFormatter for MKDistanceFormatter {}
impl PNSCopying for MKDistanceFormatter {}
impl PNSCoding for MKDistanceFormatter {}
impl std::convert::TryFrom<NSFormatter> for MKDistanceFormatter {
    type Error = &'static str;
    fn try_from(parent: NSFormatter) -> Result<MKDistanceFormatter, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MKDistanceFormatter").unwrap()) };
        if is_kind_of {
            Ok(MKDistanceFormatter(parent.0))
        } else {
            Err("This NSFormatter cannot be downcasted to MKDistanceFormatter")
        }
    }
}
impl INSObject for MKDistanceFormatter {}
impl PNSObject for MKDistanceFormatter {}
impl IMKDistanceFormatter for MKDistanceFormatter {}
pub trait IMKDistanceFormatter: Sized + std::ops::Deref {
    unsafe fn stringFromDistance_(&self, distance: CLLocationDistance) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, stringFromDistance : distance)
    }
    unsafe fn distanceFromString_(&self, distance: NSString) -> CLLocationDistance
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, distanceFromString : distance)
    }
    unsafe fn locale(&self) -> NSLocale
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, locale)
    }
    unsafe fn setLocale_(&self, locale: NSLocale)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setLocale : locale)
    }
    unsafe fn units(&self) -> MKDistanceFormatterUnits
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, units)
    }
    unsafe fn setUnits_(&self, units: MKDistanceFormatterUnits)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setUnits : units)
    }
    unsafe fn unitStyle(&self) -> MKDistanceFormatterUnitStyle
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, unitStyle)
    }
    unsafe fn setUnitStyle_(&self, unitStyle: MKDistanceFormatterUnitStyle)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setUnitStyle : unitStyle)
    }
}
pub type MKDistanceFormatterUnits = NSUInteger;
pub type MKDistanceFormatterUnitStyle = NSUInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MKGeocodingRequest(pub id);
impl std::ops::Deref for MKGeocodingRequest {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MKGeocodingRequest {}
impl MKGeocodingRequest {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MKGeocodingRequest").unwrap(), alloc) })
    }
}
impl INSObject for MKGeocodingRequest {}
impl PNSObject for MKGeocodingRequest {}
impl std::convert::TryFrom<NSObject> for MKGeocodingRequest {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<MKGeocodingRequest, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MKGeocodingRequest").unwrap()) };
        if is_kind_of {
            Ok(MKGeocodingRequest(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to MKGeocodingRequest")
        }
    }
}
impl IMKGeocodingRequest for MKGeocodingRequest {}
pub trait IMKGeocodingRequest: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn initWithAddressString_(&self, addressString: NSString) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithAddressString : addressString)
    }
    unsafe fn getMapItemsWithCompletionHandler_(
        &self,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, getMapItemsWithCompletionHandler : completionHandler)
    }
    unsafe fn cancel(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, cancel)
    }
    unsafe fn isCancelled(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isCancelled)
    }
    unsafe fn isLoading(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isLoading)
    }
    unsafe fn addressString(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, addressString)
    }
    unsafe fn region(&self) -> MKCoordinateRegion
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, region)
    }
    unsafe fn setRegion_(&self, region: MKCoordinateRegion)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRegion : region)
    }
    unsafe fn preferredLocale(&self) -> NSLocale
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, preferredLocale)
    }
    unsafe fn setPreferredLocale_(&self, preferredLocale: NSLocale)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPreferredLocale : preferredLocale)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MKGeocodingRequest").unwrap(), new)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MKPointAnnotation(pub id);
impl std::ops::Deref for MKPointAnnotation {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MKPointAnnotation {}
impl MKPointAnnotation {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MKPointAnnotation").unwrap(), alloc) })
    }
}
impl IMKShape for MKPointAnnotation {}
impl PMKAnnotation for MKPointAnnotation {}
impl std::convert::TryFrom<MKShape> for MKPointAnnotation {
    type Error = &'static str;
    fn try_from(parent: MKShape) -> Result<MKPointAnnotation, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MKPointAnnotation").unwrap()) };
        if is_kind_of {
            Ok(MKPointAnnotation(parent.0))
        } else {
            Err("This MKShape cannot be downcasted to MKPointAnnotation")
        }
    }
}
impl INSObject for MKPointAnnotation {}
impl PNSObject for MKPointAnnotation {}
impl IMKPointAnnotation for MKPointAnnotation {}
pub trait IMKPointAnnotation: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn initWithCoordinate_(&self, coordinate: CLLocationCoordinate2D) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithCoordinate : coordinate)
    }
    unsafe fn initWithCoordinate_title_subtitle_(
        &self,
        coordinate: CLLocationCoordinate2D,
        title: NSString,
        subtitle: NSString,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithCoordinate : coordinate, title : title, subtitle : subtitle)
    }
    unsafe fn coordinate(&self) -> CLLocationCoordinate2D
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, coordinate)
    }
    unsafe fn setCoordinate_(&self, coordinate: CLLocationCoordinate2D)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCoordinate : coordinate)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MKMultiPoint(pub id);
impl std::ops::Deref for MKMultiPoint {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MKMultiPoint {}
impl MKMultiPoint {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MKMultiPoint").unwrap(), alloc) })
    }
}
impl IMKShape for MKMultiPoint {}
impl PMKAnnotation for MKMultiPoint {}
impl std::convert::TryFrom<MKShape> for MKMultiPoint {
    type Error = &'static str;
    fn try_from(parent: MKShape) -> Result<MKMultiPoint, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MKMultiPoint").unwrap()) };
        if is_kind_of {
            Ok(MKMultiPoint(parent.0))
        } else {
            Err("This MKShape cannot be downcasted to MKMultiPoint")
        }
    }
}
impl INSObject for MKMultiPoint {}
impl PNSObject for MKMultiPoint {}
impl IMKMultiPoint for MKMultiPoint {}
pub trait IMKMultiPoint: Sized + std::ops::Deref {
    unsafe fn points(&self) -> *mut MKMapPoint
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, points)
    }
    unsafe fn getCoordinates_range_(&self, coords: *mut CLLocationCoordinate2D, range: NSRange)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, getCoordinates : coords, range : range)
    }
    unsafe fn locationAtPointIndex_(&self, index: NSUInteger) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, locationAtPointIndex : index)
    }
    unsafe fn locationsAtPointIndexes_(&self, indexes: NSIndexSet) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, locationsAtPointIndexes : indexes)
    }
    unsafe fn pointCount(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, pointCount)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MKPolyline(pub id);
impl std::ops::Deref for MKPolyline {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MKPolyline {}
impl MKPolyline {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MKPolyline").unwrap(), alloc) })
    }
}
impl PMKOverlay for MKPolyline {}
impl IMKMultiPoint for MKPolyline {}
impl From<MKPolyline> for MKMultiPoint {
    fn from(child: MKPolyline) -> MKMultiPoint {
        MKMultiPoint(child.0)
    }
}
impl std::convert::TryFrom<MKMultiPoint> for MKPolyline {
    type Error = &'static str;
    fn try_from(parent: MKMultiPoint) -> Result<MKPolyline, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MKPolyline").unwrap()) };
        if is_kind_of {
            Ok(MKPolyline(parent.0))
        } else {
            Err("This MKMultiPoint cannot be downcasted to MKPolyline")
        }
    }
}
impl IMKShape for MKPolyline {}
impl PMKAnnotation for MKPolyline {}
impl INSObject for MKPolyline {}
impl PNSObject for MKPolyline {}
impl IMKPolyline for MKPolyline {}
pub trait IMKPolyline: Sized + std::ops::Deref {
    unsafe fn polylineWithPoints_count_(
        points: *const MKMapPoint,
        count: NSUInteger,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MKPolyline").unwrap(), polylineWithPoints : points, count : count)
    }
    unsafe fn polylineWithCoordinates_count_(
        coords: *const CLLocationCoordinate2D,
        count: NSUInteger,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MKPolyline").unwrap(), polylineWithCoordinates : coords, count : count)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MKMultiPolyline(pub id);
impl std::ops::Deref for MKMultiPolyline {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MKMultiPolyline {}
impl MKMultiPolyline {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MKMultiPolyline").unwrap(), alloc) })
    }
}
impl PMKOverlay for MKMultiPolyline {}
impl IMKShape for MKMultiPolyline {}
impl PMKAnnotation for MKMultiPolyline {}
impl std::convert::TryFrom<MKShape> for MKMultiPolyline {
    type Error = &'static str;
    fn try_from(parent: MKShape) -> Result<MKMultiPolyline, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MKMultiPolyline").unwrap()) };
        if is_kind_of {
            Ok(MKMultiPolyline(parent.0))
        } else {
            Err("This MKShape cannot be downcasted to MKMultiPolyline")
        }
    }
}
impl INSObject for MKMultiPolyline {}
impl PNSObject for MKMultiPolyline {}
impl IMKMultiPolyline for MKMultiPolyline {}
pub trait IMKMultiPolyline: Sized + std::ops::Deref {
    unsafe fn initWithPolylines_(&self, polylines: NSArray) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithPolylines : polylines)
    }
    unsafe fn polylines(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, polylines)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MKPolygon(pub id);
impl std::ops::Deref for MKPolygon {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MKPolygon {}
impl MKPolygon {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MKPolygon").unwrap(), alloc) })
    }
}
impl PMKOverlay for MKPolygon {}
impl IMKMultiPoint for MKPolygon {}
impl std::convert::TryFrom<MKMultiPoint> for MKPolygon {
    type Error = &'static str;
    fn try_from(parent: MKMultiPoint) -> Result<MKPolygon, Self::Error> {
        let is_kind_of: bool = unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MKPolygon").unwrap()) };
        if is_kind_of {
            Ok(MKPolygon(parent.0))
        } else {
            Err("This MKMultiPoint cannot be downcasted to MKPolygon")
        }
    }
}
impl IMKShape for MKPolygon {}
impl PMKAnnotation for MKPolygon {}
impl INSObject for MKPolygon {}
impl PNSObject for MKPolygon {}
impl IMKPolygon for MKPolygon {}
pub trait IMKPolygon: Sized + std::ops::Deref {
    unsafe fn interiorPolygons(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, interiorPolygons)
    }
    unsafe fn polygonWithPoints_count_(points: *const MKMapPoint, count: NSUInteger) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MKPolygon").unwrap(), polygonWithPoints : points, count : count)
    }
    unsafe fn polygonWithPoints_count_interiorPolygons_(
        points: *const MKMapPoint,
        count: NSUInteger,
        interiorPolygons: NSArray,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MKPolygon").unwrap(), polygonWithPoints : points, count : count, interiorPolygons : interiorPolygons)
    }
    unsafe fn polygonWithCoordinates_count_(
        coords: *const CLLocationCoordinate2D,
        count: NSUInteger,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MKPolygon").unwrap(), polygonWithCoordinates : coords, count : count)
    }
    unsafe fn polygonWithCoordinates_count_interiorPolygons_(
        coords: *const CLLocationCoordinate2D,
        count: NSUInteger,
        interiorPolygons: NSArray,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MKPolygon").unwrap(), polygonWithCoordinates : coords, count : count, interiorPolygons : interiorPolygons)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MKMultiPolygon(pub id);
impl std::ops::Deref for MKMultiPolygon {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MKMultiPolygon {}
impl MKMultiPolygon {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MKMultiPolygon").unwrap(), alloc) })
    }
}
impl PMKOverlay for MKMultiPolygon {}
impl IMKShape for MKMultiPolygon {}
impl PMKAnnotation for MKMultiPolygon {}
impl std::convert::TryFrom<MKShape> for MKMultiPolygon {
    type Error = &'static str;
    fn try_from(parent: MKShape) -> Result<MKMultiPolygon, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MKMultiPolygon").unwrap()) };
        if is_kind_of {
            Ok(MKMultiPolygon(parent.0))
        } else {
            Err("This MKShape cannot be downcasted to MKMultiPolygon")
        }
    }
}
impl INSObject for MKMultiPolygon {}
impl PNSObject for MKMultiPolygon {}
impl IMKMultiPolygon for MKMultiPolygon {}
pub trait IMKMultiPolygon: Sized + std::ops::Deref {
    unsafe fn initWithPolygons_(&self, polygons: NSArray) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithPolygons : polygons)
    }
    unsafe fn polygons(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, polygons)
    }
}
pub trait PMKGeoJSONObject: Sized + std::ops::Deref {}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MKGeoJSONDecoder(pub id);
impl std::ops::Deref for MKGeoJSONDecoder {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MKGeoJSONDecoder {}
impl MKGeoJSONDecoder {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MKGeoJSONDecoder").unwrap(), alloc) })
    }
}
impl INSObject for MKGeoJSONDecoder {}
impl PNSObject for MKGeoJSONDecoder {}
impl std::convert::TryFrom<NSObject> for MKGeoJSONDecoder {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<MKGeoJSONDecoder, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MKGeoJSONDecoder").unwrap()) };
        if is_kind_of {
            Ok(MKGeoJSONDecoder(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to MKGeoJSONDecoder")
        }
    }
}
impl IMKGeoJSONDecoder for MKGeoJSONDecoder {}
pub trait IMKGeoJSONDecoder: Sized + std::ops::Deref {
    unsafe fn geoJSONObjectsWithData_error_(&self, data: NSData, errorPtr: *mut NSError) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, geoJSONObjectsWithData : data, error : errorPtr)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MKGeoJSONFeature(pub id);
impl std::ops::Deref for MKGeoJSONFeature {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MKGeoJSONFeature {}
impl MKGeoJSONFeature {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MKGeoJSONFeature").unwrap(), alloc) })
    }
}
impl PMKGeoJSONObject for MKGeoJSONFeature {}
impl INSObject for MKGeoJSONFeature {}
impl PNSObject for MKGeoJSONFeature {}
impl std::convert::TryFrom<NSObject> for MKGeoJSONFeature {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<MKGeoJSONFeature, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MKGeoJSONFeature").unwrap()) };
        if is_kind_of {
            Ok(MKGeoJSONFeature(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to MKGeoJSONFeature")
        }
    }
}
impl IMKGeoJSONFeature for MKGeoJSONFeature {}
pub trait IMKGeoJSONFeature: Sized + std::ops::Deref {
    unsafe fn identifier(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, identifier)
    }
    unsafe fn properties(&self) -> NSData
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, properties)
    }
    unsafe fn geometry(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, geometry)
    }
}
impl MKPointAnnotation_MKGeoJSONSerialization for MKPointAnnotation {}
pub trait MKPointAnnotation_MKGeoJSONSerialization: Sized + std::ops::Deref {}
impl MKMultiPoint_MKGeoJSONSerialization for MKMultiPoint {}
pub trait MKMultiPoint_MKGeoJSONSerialization: Sized + std::ops::Deref {}
impl MKMultiPolyline_MKGeoJSONSerialization for MKMultiPolyline {}
pub trait MKMultiPolyline_MKGeoJSONSerialization: Sized + std::ops::Deref {}
impl MKMultiPolygon_MKGeoJSONSerialization for MKMultiPolygon {}
pub trait MKMultiPolygon_MKGeoJSONSerialization: Sized + std::ops::Deref {}
impl MKPolyline_MKGeoJSONSerialization for MKPolyline {}
pub trait MKPolyline_MKGeoJSONSerialization: Sized + std::ops::Deref {}
impl MKPolygon_MKGeoJSONSerialization for MKPolygon {}
pub trait MKPolygon_MKGeoJSONSerialization: Sized + std::ops::Deref {}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MKGeodesicPolyline(pub id);
impl std::ops::Deref for MKGeodesicPolyline {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MKGeodesicPolyline {}
impl MKGeodesicPolyline {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MKGeodesicPolyline").unwrap(), alloc) })
    }
}
impl IMKPolyline for MKGeodesicPolyline {}
impl PMKOverlay for MKGeodesicPolyline {}
impl From<MKGeodesicPolyline> for MKPolyline {
    fn from(child: MKGeodesicPolyline) -> MKPolyline {
        MKPolyline(child.0)
    }
}
impl std::convert::TryFrom<MKPolyline> for MKGeodesicPolyline {
    type Error = &'static str;
    fn try_from(parent: MKPolyline) -> Result<MKGeodesicPolyline, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MKGeodesicPolyline").unwrap()) };
        if is_kind_of {
            Ok(MKGeodesicPolyline(parent.0))
        } else {
            Err("This MKPolyline cannot be downcasted to MKGeodesicPolyline")
        }
    }
}
impl IMKMultiPoint for MKGeodesicPolyline {}
impl IMKShape for MKGeodesicPolyline {}
impl PMKAnnotation for MKGeodesicPolyline {}
impl INSObject for MKGeodesicPolyline {}
impl PNSObject for MKGeodesicPolyline {}
impl IMKGeodesicPolyline for MKGeodesicPolyline {}
pub trait IMKGeodesicPolyline: Sized + std::ops::Deref {
    unsafe fn polylineWithPoints_count_(
        points: *const MKMapPoint,
        count: NSUInteger,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MKGeodesicPolyline").unwrap(), polylineWithPoints : points, count : count)
    }
    unsafe fn polylineWithCoordinates_count_(
        coords: *const CLLocationCoordinate2D,
        count: NSUInteger,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MKGeodesicPolyline").unwrap(), polylineWithCoordinates : coords, count : count)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MKPointOfInterestFilter(pub id);
impl std::ops::Deref for MKPointOfInterestFilter {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MKPointOfInterestFilter {}
impl MKPointOfInterestFilter {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MKPointOfInterestFilter").unwrap(), alloc) })
    }
}
impl PNSSecureCoding for MKPointOfInterestFilter {}
impl PNSCopying for MKPointOfInterestFilter {}
impl INSObject for MKPointOfInterestFilter {}
impl PNSObject for MKPointOfInterestFilter {}
impl std::convert::TryFrom<NSObject> for MKPointOfInterestFilter {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<MKPointOfInterestFilter, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MKPointOfInterestFilter").unwrap()) };
        if is_kind_of {
            Ok(MKPointOfInterestFilter(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to MKPointOfInterestFilter")
        }
    }
}
impl IMKPointOfInterestFilter for MKPointOfInterestFilter {}
pub trait IMKPointOfInterestFilter: Sized + std::ops::Deref {
    unsafe fn initIncludingCategories_(&self, categories: NSArray) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initIncludingCategories : categories)
    }
    unsafe fn initExcludingCategories_(&self, categories: NSArray) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initExcludingCategories : categories)
    }
    unsafe fn includesCategory_(&self, category: NSString) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, includesCategory : category)
    }
    unsafe fn excludesCategory_(&self, category: NSString) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, excludesCategory : category)
    }
    unsafe fn filterIncludingAllCategories() -> MKPointOfInterestFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MKPointOfInterestFilter").unwrap(), filterIncludingAllCategories)
    }
    unsafe fn filterExcludingAllCategories() -> MKPointOfInterestFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MKPointOfInterestFilter").unwrap(), filterExcludingAllCategories)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MKLocalPointsOfInterestRequest(pub id);
impl std::ops::Deref for MKLocalPointsOfInterestRequest {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MKLocalPointsOfInterestRequest {}
impl MKLocalPointsOfInterestRequest {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MKLocalPointsOfInterestRequest").unwrap(), alloc) })
    }
}
impl PNSCopying for MKLocalPointsOfInterestRequest {}
impl INSObject for MKLocalPointsOfInterestRequest {}
impl PNSObject for MKLocalPointsOfInterestRequest {}
impl std::convert::TryFrom<NSObject> for MKLocalPointsOfInterestRequest {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<MKLocalPointsOfInterestRequest, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MKLocalPointsOfInterestRequest").unwrap())
        };
        if is_kind_of {
            Ok(MKLocalPointsOfInterestRequest(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to MKLocalPointsOfInterestRequest")
        }
    }
}
impl IMKLocalPointsOfInterestRequest for MKLocalPointsOfInterestRequest {}
pub trait IMKLocalPointsOfInterestRequest: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn initWithCenterCoordinate_radius_(
        &self,
        coordinate: CLLocationCoordinate2D,
        radius: CLLocationDistance,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithCenterCoordinate : coordinate, radius : radius)
    }
    unsafe fn initWithCoordinateRegion_(&self, region: MKCoordinateRegion) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithCoordinateRegion : region)
    }
    unsafe fn coordinate(&self) -> CLLocationCoordinate2D
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, coordinate)
    }
    unsafe fn radius(&self) -> CLLocationDistance
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, radius)
    }
    unsafe fn region(&self) -> MKCoordinateRegion
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, region)
    }
    unsafe fn pointOfInterestFilter(&self) -> MKPointOfInterestFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, pointOfInterestFilter)
    }
    unsafe fn setPointOfInterestFilter_(&self, pointOfInterestFilter: MKPointOfInterestFilter)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPointOfInterestFilter : pointOfInterestFilter)
    }
}
pub type MKLocalSearchCompletionHandler = *mut ::std::os::raw::c_void;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MKLocalSearch(pub id);
impl std::ops::Deref for MKLocalSearch {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MKLocalSearch {}
impl MKLocalSearch {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MKLocalSearch").unwrap(), alloc) })
    }
}
impl INSObject for MKLocalSearch {}
impl PNSObject for MKLocalSearch {}
impl std::convert::TryFrom<NSObject> for MKLocalSearch {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<MKLocalSearch, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MKLocalSearch").unwrap()) };
        if is_kind_of {
            Ok(MKLocalSearch(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to MKLocalSearch")
        }
    }
}
impl IMKLocalSearch for MKLocalSearch {}
pub trait IMKLocalSearch: Sized + std::ops::Deref {
    unsafe fn initWithRequest_(&self, request: MKLocalSearchRequest) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithRequest : request)
    }
    unsafe fn initWithPointsOfInterestRequest_(
        &self,
        request: MKLocalPointsOfInterestRequest,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithPointsOfInterestRequest : request)
    }
    unsafe fn startWithCompletionHandler_(&self, completionHandler: MKLocalSearchCompletionHandler)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, startWithCompletionHandler : completionHandler)
    }
    unsafe fn cancel(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, cancel)
    }
    unsafe fn isSearching(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isSearching)
    }
}
pub type MKLocalSearchResultType = NSUInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MKLocalSearchRequest(pub id);
impl std::ops::Deref for MKLocalSearchRequest {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MKLocalSearchRequest {}
impl MKLocalSearchRequest {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MKLocalSearchRequest").unwrap(), alloc) })
    }
}
impl PNSCopying for MKLocalSearchRequest {}
impl INSObject for MKLocalSearchRequest {}
impl PNSObject for MKLocalSearchRequest {}
impl std::convert::TryFrom<NSObject> for MKLocalSearchRequest {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<MKLocalSearchRequest, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MKLocalSearchRequest").unwrap()) };
        if is_kind_of {
            Ok(MKLocalSearchRequest(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to MKLocalSearchRequest")
        }
    }
}
impl IMKLocalSearchRequest for MKLocalSearchRequest {}
pub trait IMKLocalSearchRequest: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn initWithNaturalLanguageQuery_(&self, naturalLanguageQuery: NSString) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithNaturalLanguageQuery : naturalLanguageQuery)
    }
    unsafe fn initWithNaturalLanguageQuery_region_(
        &self,
        naturalLanguageQuery: NSString,
        region: MKCoordinateRegion,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithNaturalLanguageQuery : naturalLanguageQuery, region : region)
    }
    unsafe fn naturalLanguageQuery(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, naturalLanguageQuery)
    }
    unsafe fn setNaturalLanguageQuery_(&self, naturalLanguageQuery: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setNaturalLanguageQuery : naturalLanguageQuery)
    }
    unsafe fn region(&self) -> MKCoordinateRegion
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, region)
    }
    unsafe fn setRegion_(&self, region: MKCoordinateRegion)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRegion : region)
    }
    unsafe fn regionPriority(&self) -> MKLocalSearchRegionPriority
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, regionPriority)
    }
    unsafe fn setRegionPriority_(&self, regionPriority: MKLocalSearchRegionPriority)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRegionPriority : regionPriority)
    }
    unsafe fn resultTypes(&self) -> MKLocalSearchResultType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, resultTypes)
    }
    unsafe fn setResultTypes_(&self, resultTypes: MKLocalSearchResultType)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setResultTypes : resultTypes)
    }
    unsafe fn pointOfInterestFilter(&self) -> MKPointOfInterestFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, pointOfInterestFilter)
    }
    unsafe fn setPointOfInterestFilter_(&self, pointOfInterestFilter: MKPointOfInterestFilter)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPointOfInterestFilter : pointOfInterestFilter)
    }
    unsafe fn addressFilter(&self) -> MKAddressFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, addressFilter)
    }
    unsafe fn setAddressFilter_(&self, addressFilter: MKAddressFilter)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAddressFilter : addressFilter)
    }
}
pub type MKSearchCompletionFilterType = NSInteger;
pub type MKLocalSearchCompleterResultType = NSUInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MKLocalSearchCompleter(pub id);
impl std::ops::Deref for MKLocalSearchCompleter {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MKLocalSearchCompleter {}
impl MKLocalSearchCompleter {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MKLocalSearchCompleter").unwrap(), alloc) })
    }
}
impl INSObject for MKLocalSearchCompleter {}
impl PNSObject for MKLocalSearchCompleter {}
impl std::convert::TryFrom<NSObject> for MKLocalSearchCompleter {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<MKLocalSearchCompleter, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MKLocalSearchCompleter").unwrap()) };
        if is_kind_of {
            Ok(MKLocalSearchCompleter(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to MKLocalSearchCompleter")
        }
    }
}
impl IMKLocalSearchCompleter for MKLocalSearchCompleter {}
pub trait IMKLocalSearchCompleter: Sized + std::ops::Deref {
    unsafe fn cancel(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, cancel)
    }
    unsafe fn queryFragment(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, queryFragment)
    }
    unsafe fn setQueryFragment_(&self, queryFragment: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setQueryFragment : queryFragment)
    }
    unsafe fn region(&self) -> MKCoordinateRegion
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, region)
    }
    unsafe fn setRegion_(&self, region: MKCoordinateRegion)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRegion : region)
    }
    unsafe fn regionPriority(&self) -> MKLocalSearchRegionPriority
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, regionPriority)
    }
    unsafe fn setRegionPriority_(&self, regionPriority: MKLocalSearchRegionPriority)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRegionPriority : regionPriority)
    }
    unsafe fn filterType(&self) -> MKSearchCompletionFilterType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, filterType)
    }
    unsafe fn setFilterType_(&self, filterType: MKSearchCompletionFilterType)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setFilterType : filterType)
    }
    unsafe fn resultTypes(&self) -> MKLocalSearchCompleterResultType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, resultTypes)
    }
    unsafe fn setResultTypes_(&self, resultTypes: MKLocalSearchCompleterResultType)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setResultTypes : resultTypes)
    }
    unsafe fn pointOfInterestFilter(&self) -> MKPointOfInterestFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, pointOfInterestFilter)
    }
    unsafe fn setPointOfInterestFilter_(&self, pointOfInterestFilter: MKPointOfInterestFilter)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPointOfInterestFilter : pointOfInterestFilter)
    }
    unsafe fn addressFilter(&self) -> MKAddressFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, addressFilter)
    }
    unsafe fn setAddressFilter_(&self, addressFilter: MKAddressFilter)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAddressFilter : addressFilter)
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
    unsafe fn results(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, results)
    }
    unsafe fn isSearching(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isSearching)
    }
}
pub trait PMKLocalSearchCompleterDelegate: Sized + std::ops::Deref {
    unsafe fn completerDidUpdateResults_(&self, completer: MKLocalSearchCompleter)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, completerDidUpdateResults : completer)
    }
    unsafe fn completer_didFailWithError_(&self, completer: MKLocalSearchCompleter, error: NSError)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, completer : completer, didFailWithError : error)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MKLocalSearchCompletion(pub id);
impl std::ops::Deref for MKLocalSearchCompletion {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MKLocalSearchCompletion {}
impl MKLocalSearchCompletion {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MKLocalSearchCompletion").unwrap(), alloc) })
    }
}
impl INSObject for MKLocalSearchCompletion {}
impl PNSObject for MKLocalSearchCompletion {}
impl std::convert::TryFrom<NSObject> for MKLocalSearchCompletion {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<MKLocalSearchCompletion, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MKLocalSearchCompletion").unwrap()) };
        if is_kind_of {
            Ok(MKLocalSearchCompletion(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to MKLocalSearchCompletion")
        }
    }
}
impl IMKLocalSearchCompletion for MKLocalSearchCompletion {}
pub trait IMKLocalSearchCompletion: Sized + std::ops::Deref {
    unsafe fn title(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, title)
    }
    unsafe fn titleHighlightRanges(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, titleHighlightRanges)
    }
    unsafe fn subtitle(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, subtitle)
    }
    unsafe fn subtitleHighlightRanges(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, subtitleHighlightRanges)
    }
}
impl MKLocalSearchRequest_ for MKLocalSearchRequest {}
pub trait MKLocalSearchRequest_: Sized + std::ops::Deref {
    unsafe fn initWithCompletion_(&self, completion: MKLocalSearchCompletion) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithCompletion : completion)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MKLocalSearchResponse(pub id);
impl std::ops::Deref for MKLocalSearchResponse {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MKLocalSearchResponse {}
impl MKLocalSearchResponse {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MKLocalSearchResponse").unwrap(), alloc) })
    }
}
impl INSObject for MKLocalSearchResponse {}
impl PNSObject for MKLocalSearchResponse {}
impl std::convert::TryFrom<NSObject> for MKLocalSearchResponse {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<MKLocalSearchResponse, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MKLocalSearchResponse").unwrap()) };
        if is_kind_of {
            Ok(MKLocalSearchResponse(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to MKLocalSearchResponse")
        }
    }
}
impl IMKLocalSearchResponse for MKLocalSearchResponse {}
pub trait IMKLocalSearchResponse: Sized + std::ops::Deref {
    unsafe fn mapItems(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, mapItems)
    }
    unsafe fn boundingRegion(&self) -> MKCoordinateRegion
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, boundingRegion)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MKMapCamera(pub id);
impl std::ops::Deref for MKMapCamera {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MKMapCamera {}
impl MKMapCamera {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MKMapCamera").unwrap(), alloc) })
    }
}
impl PNSSecureCoding for MKMapCamera {}
impl PNSCopying for MKMapCamera {}
impl INSObject for MKMapCamera {}
impl PNSObject for MKMapCamera {}
impl std::convert::TryFrom<NSObject> for MKMapCamera {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<MKMapCamera, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MKMapCamera").unwrap()) };
        if is_kind_of {
            Ok(MKMapCamera(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to MKMapCamera")
        }
    }
}
impl IMKMapCamera for MKMapCamera {}
pub trait IMKMapCamera: Sized + std::ops::Deref {
    unsafe fn centerCoordinate(&self) -> CLLocationCoordinate2D
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, centerCoordinate)
    }
    unsafe fn setCenterCoordinate_(&self, centerCoordinate: CLLocationCoordinate2D)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCenterCoordinate : centerCoordinate)
    }
    unsafe fn centerCoordinateDistance(&self) -> CLLocationDistance
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, centerCoordinateDistance)
    }
    unsafe fn setCenterCoordinateDistance_(&self, centerCoordinateDistance: CLLocationDistance)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCenterCoordinateDistance : centerCoordinateDistance)
    }
    unsafe fn heading(&self) -> CLLocationDirection
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, heading)
    }
    unsafe fn setHeading_(&self, heading: CLLocationDirection)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setHeading : heading)
    }
    unsafe fn pitch(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, pitch)
    }
    unsafe fn setPitch_(&self, pitch: CGFloat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPitch : pitch)
    }
    unsafe fn altitude(&self) -> CLLocationDistance
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, altitude)
    }
    unsafe fn setAltitude_(&self, altitude: CLLocationDistance)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAltitude : altitude)
    }
    unsafe fn camera() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MKMapCamera").unwrap(), camera)
    }
    unsafe fn cameraLookingAtCenterCoordinate_fromEyeCoordinate_eyeAltitude_(
        centerCoordinate: CLLocationCoordinate2D,
        eyeCoordinate: CLLocationCoordinate2D,
        eyeAltitude: CLLocationDistance,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MKMapCamera").unwrap(), cameraLookingAtCenterCoordinate : centerCoordinate, fromEyeCoordinate : eyeCoordinate, eyeAltitude : eyeAltitude)
    }
    unsafe fn cameraLookingAtCenterCoordinate_fromDistance_pitch_heading_(
        centerCoordinate: CLLocationCoordinate2D,
        distance: CLLocationDistance,
        pitch: CGFloat,
        heading: CLLocationDirection,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MKMapCamera").unwrap(), cameraLookingAtCenterCoordinate : centerCoordinate, fromDistance : distance, pitch : pitch, heading : heading)
    }
    unsafe fn cameraLookingAtMapItem_forViewSize_allowPitch_(
        mapItem: MKMapItem,
        viewSize: CGSize,
        allowPitch: BOOL,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MKMapCamera").unwrap(), cameraLookingAtMapItem : mapItem, forViewSize : viewSize, allowPitch : allowPitch)
    }
}
pub type MKMapElevationStyle = NSInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MKMapConfiguration(pub id);
impl std::ops::Deref for MKMapConfiguration {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MKMapConfiguration {}
impl MKMapConfiguration {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MKMapConfiguration").unwrap(), alloc) })
    }
}
impl PNSSecureCoding for MKMapConfiguration {}
impl PNSCopying for MKMapConfiguration {}
impl INSObject for MKMapConfiguration {}
impl PNSObject for MKMapConfiguration {}
impl std::convert::TryFrom<NSObject> for MKMapConfiguration {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<MKMapConfiguration, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MKMapConfiguration").unwrap()) };
        if is_kind_of {
            Ok(MKMapConfiguration(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to MKMapConfiguration")
        }
    }
}
impl IMKMapConfiguration for MKMapConfiguration {}
pub trait IMKMapConfiguration: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn elevationStyle(&self) -> MKMapElevationStyle
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, elevationStyle)
    }
    unsafe fn setElevationStyle_(&self, elevationStyle: MKMapElevationStyle)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setElevationStyle : elevationStyle)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MKMapConfiguration").unwrap(), new)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MKMapFeatureAnnotation(pub id);
impl std::ops::Deref for MKMapFeatureAnnotation {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MKMapFeatureAnnotation {}
impl MKMapFeatureAnnotation {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MKMapFeatureAnnotation").unwrap(), alloc) })
    }
}
impl IMKMapFeatureAnnotation for MKMapFeatureAnnotation {}
pub trait IMKMapFeatureAnnotation: Sized + std::ops::Deref {}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MKMapItemRequest(pub id);
impl std::ops::Deref for MKMapItemRequest {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MKMapItemRequest {}
impl MKMapItemRequest {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MKMapItemRequest").unwrap(), alloc) })
    }
}
impl INSObject for MKMapItemRequest {}
impl PNSObject for MKMapItemRequest {}
impl std::convert::TryFrom<NSObject> for MKMapItemRequest {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<MKMapItemRequest, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MKMapItemRequest").unwrap()) };
        if is_kind_of {
            Ok(MKMapItemRequest(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to MKMapItemRequest")
        }
    }
}
impl IMKMapItemRequest for MKMapItemRequest {}
pub trait IMKMapItemRequest: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn initWithMapItemIdentifier_(&self, identifier: MKMapItemIdentifier) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithMapItemIdentifier : identifier)
    }
    unsafe fn initWithMapFeatureAnnotation_(
        &self,
        mapFeatureAnnotation: MKMapFeatureAnnotation,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithMapFeatureAnnotation : mapFeatureAnnotation)
    }
    unsafe fn getMapItemWithCompletionHandler_(
        &self,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, getMapItemWithCompletionHandler : completionHandler)
    }
    unsafe fn cancel(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, cancel)
    }
    unsafe fn mapItemIdentifier(&self) -> MKMapItemIdentifier
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, mapItemIdentifier)
    }
    unsafe fn mapFeatureAnnotation(&self) -> MKMapFeatureAnnotation
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, mapFeatureAnnotation)
    }
    unsafe fn featureAnnotation(&self) -> MKMapFeatureAnnotation
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, featureAnnotation)
    }
    unsafe fn isCancelled(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isCancelled)
    }
    unsafe fn isLoading(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isLoading)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MKMapItemRequest").unwrap(), new)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MKMapSnapshot(pub id);
impl std::ops::Deref for MKMapSnapshot {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MKMapSnapshot {}
impl MKMapSnapshot {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MKMapSnapshot").unwrap(), alloc) })
    }
}
impl INSObject for MKMapSnapshot {}
impl PNSObject for MKMapSnapshot {}
impl std::convert::TryFrom<NSObject> for MKMapSnapshot {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<MKMapSnapshot, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MKMapSnapshot").unwrap()) };
        if is_kind_of {
            Ok(MKMapSnapshot(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to MKMapSnapshot")
        }
    }
}
impl IMKMapSnapshot for MKMapSnapshot {}
pub trait IMKMapSnapshot: Sized + std::ops::Deref {
    unsafe fn pointForCoordinate_(&self, coordinate: CLLocationCoordinate2D) -> NSPoint
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, pointForCoordinate : coordinate)
    }
    unsafe fn image(&self) -> NSImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, image)
    }
    unsafe fn appearance(&self) -> NSAppearance
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, appearance)
    }
}
pub type MKAnnotationViewDragState = NSUInteger;
pub type MKFeatureDisplayPriority = f32;
pub type MKAnnotationViewZPriority = f32;
pub type MKAnnotationViewCollisionMode = NSInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MKAnnotationView(pub id);
impl std::ops::Deref for MKAnnotationView {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MKAnnotationView {}
impl MKAnnotationView {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MKAnnotationView").unwrap(), alloc) })
    }
}
impl INSView for MKAnnotationView {}
impl PNSAnimatablePropertyContainer for MKAnnotationView {}
impl PNSUserInterfaceItemIdentification for MKAnnotationView {}
impl PNSDraggingDestination for MKAnnotationView {}
impl PNSAppearanceCustomization for MKAnnotationView {}
impl PNSAccessibilityElement for MKAnnotationView {}
impl PNSAccessibility for MKAnnotationView {}
impl std::convert::TryFrom<NSView> for MKAnnotationView {
    type Error = &'static str;
    fn try_from(parent: NSView) -> Result<MKAnnotationView, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MKAnnotationView").unwrap()) };
        if is_kind_of {
            Ok(MKAnnotationView(parent.0))
        } else {
            Err("This NSView cannot be downcasted to MKAnnotationView")
        }
    }
}
impl INSResponder for MKAnnotationView {}
impl PNSCoding for MKAnnotationView {}
impl INSObject for MKAnnotationView {}
impl PNSObject for MKAnnotationView {}
impl IMKAnnotationView for MKAnnotationView {}
pub trait IMKAnnotationView: Sized + std::ops::Deref {
    unsafe fn initWithAnnotation_reuseIdentifier_(
        &self,
        annotation: *mut u64,
        reuseIdentifier: NSString,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithAnnotation : annotation, reuseIdentifier : reuseIdentifier)
    }
    unsafe fn initWithCoder_(&self, aDecoder: NSCoder) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithCoder : aDecoder)
    }
    unsafe fn prepareForReuse(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, prepareForReuse)
    }
    unsafe fn prepareForDisplay(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, prepareForDisplay)
    }
    unsafe fn setSelected_animated_(&self, selected: BOOL, animated: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSelected : selected, animated : animated)
    }
    unsafe fn setDragState_animated_(&self, newDragState: MKAnnotationViewDragState, animated: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDragState : newDragState, animated : animated)
    }
    unsafe fn reuseIdentifier(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, reuseIdentifier)
    }
    unsafe fn annotation(&self) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, annotation)
    }
    unsafe fn setAnnotation_(&self, annotation: *mut u64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAnnotation : annotation)
    }
    unsafe fn image(&self) -> NSImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, image)
    }
    unsafe fn setImage_(&self, image: NSImage)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setImage : image)
    }
    unsafe fn centerOffset(&self) -> CGPoint
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, centerOffset)
    }
    unsafe fn setCenterOffset_(&self, centerOffset: CGPoint)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCenterOffset : centerOffset)
    }
    unsafe fn accessoryOffset(&self) -> CGPoint
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, accessoryOffset)
    }
    unsafe fn setAccessoryOffset_(&self, accessoryOffset: CGPoint)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAccessoryOffset : accessoryOffset)
    }
    unsafe fn calloutOffset(&self) -> CGPoint
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, calloutOffset)
    }
    unsafe fn setCalloutOffset_(&self, calloutOffset: CGPoint)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCalloutOffset : calloutOffset)
    }
    unsafe fn leftCalloutOffset(&self) -> CGPoint
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, leftCalloutOffset)
    }
    unsafe fn setLeftCalloutOffset_(&self, leftCalloutOffset: CGPoint)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setLeftCalloutOffset : leftCalloutOffset)
    }
    unsafe fn rightCalloutOffset(&self) -> CGPoint
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, rightCalloutOffset)
    }
    unsafe fn setRightCalloutOffset_(&self, rightCalloutOffset: CGPoint)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRightCalloutOffset : rightCalloutOffset)
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
    unsafe fn isHighlighted(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isHighlighted)
    }
    unsafe fn setHighlighted_(&self, highlighted: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setHighlighted : highlighted)
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
    unsafe fn canShowCallout(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, canShowCallout)
    }
    unsafe fn setCanShowCallout_(&self, canShowCallout: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCanShowCallout : canShowCallout)
    }
    unsafe fn leftCalloutAccessoryView(&self) -> NSView
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, leftCalloutAccessoryView)
    }
    unsafe fn setLeftCalloutAccessoryView_(&self, leftCalloutAccessoryView: NSView)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setLeftCalloutAccessoryView : leftCalloutAccessoryView)
    }
    unsafe fn rightCalloutAccessoryView(&self) -> NSView
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, rightCalloutAccessoryView)
    }
    unsafe fn setRightCalloutAccessoryView_(&self, rightCalloutAccessoryView: NSView)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRightCalloutAccessoryView : rightCalloutAccessoryView)
    }
    unsafe fn detailCalloutAccessoryView(&self) -> NSView
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, detailCalloutAccessoryView)
    }
    unsafe fn setDetailCalloutAccessoryView_(&self, detailCalloutAccessoryView: NSView)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDetailCalloutAccessoryView : detailCalloutAccessoryView)
    }
    unsafe fn isDraggable(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isDraggable)
    }
    unsafe fn setDraggable_(&self, draggable: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDraggable : draggable)
    }
    unsafe fn dragState(&self) -> MKAnnotationViewDragState
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, dragState)
    }
    unsafe fn setDragState_(&self, dragState: MKAnnotationViewDragState)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDragState : dragState)
    }
    unsafe fn clusteringIdentifier(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, clusteringIdentifier)
    }
    unsafe fn setClusteringIdentifier_(&self, clusteringIdentifier: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setClusteringIdentifier : clusteringIdentifier)
    }
    unsafe fn clusterAnnotationView(&self) -> MKAnnotationView
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, clusterAnnotationView)
    }
    unsafe fn displayPriority(&self) -> MKFeatureDisplayPriority
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, displayPriority)
    }
    unsafe fn setDisplayPriority_(&self, displayPriority: MKFeatureDisplayPriority)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDisplayPriority : displayPriority)
    }
    unsafe fn zPriority(&self) -> MKAnnotationViewZPriority
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, zPriority)
    }
    unsafe fn setZPriority_(&self, zPriority: MKAnnotationViewZPriority)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setZPriority : zPriority)
    }
    unsafe fn selectedZPriority(&self) -> MKAnnotationViewZPriority
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, selectedZPriority)
    }
    unsafe fn setSelectedZPriority_(&self, selectedZPriority: MKAnnotationViewZPriority)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSelectedZPriority : selectedZPriority)
    }
    unsafe fn collisionMode(&self) -> MKAnnotationViewCollisionMode
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, collisionMode)
    }
    unsafe fn setCollisionMode_(&self, collisionMode: MKAnnotationViewCollisionMode)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCollisionMode : collisionMode)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MKOverlayRenderer(pub id);
impl std::ops::Deref for MKOverlayRenderer {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MKOverlayRenderer {}
impl MKOverlayRenderer {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MKOverlayRenderer").unwrap(), alloc) })
    }
}
impl INSObject for MKOverlayRenderer {}
impl PNSObject for MKOverlayRenderer {}
impl std::convert::TryFrom<NSObject> for MKOverlayRenderer {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<MKOverlayRenderer, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MKOverlayRenderer").unwrap()) };
        if is_kind_of {
            Ok(MKOverlayRenderer(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to MKOverlayRenderer")
        }
    }
}
impl IMKOverlayRenderer for MKOverlayRenderer {}
pub trait IMKOverlayRenderer: Sized + std::ops::Deref {
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
    unsafe fn setNeedsDisplay(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, setNeedsDisplay)
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
    unsafe fn alpha(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, alpha)
    }
    unsafe fn setAlpha_(&self, alpha: CGFloat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAlpha : alpha)
    }
    unsafe fn contentScaleFactor(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, contentScaleFactor)
    }
    unsafe fn blendMode(&self) -> CGBlendMode
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, blendMode)
    }
    unsafe fn setBlendMode_(&self, blendMode: CGBlendMode)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setBlendMode : blendMode)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MKSelectionAccessory(pub id);
impl std::ops::Deref for MKSelectionAccessory {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MKSelectionAccessory {}
impl MKSelectionAccessory {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MKSelectionAccessory").unwrap(), alloc) })
    }
}
impl INSObject for MKSelectionAccessory {}
impl PNSObject for MKSelectionAccessory {}
impl std::convert::TryFrom<NSObject> for MKSelectionAccessory {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<MKSelectionAccessory, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MKSelectionAccessory").unwrap()) };
        if is_kind_of {
            Ok(MKSelectionAccessory(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to MKSelectionAccessory")
        }
    }
}
impl IMKSelectionAccessory for MKSelectionAccessory {}
pub trait IMKSelectionAccessory: Sized + std::ops::Deref {
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
        msg_send!(&*objc2::runtime::AnyClass::get(c"MKSelectionAccessory").unwrap(), new)
    }
    unsafe fn mapItemDetailWithPresentationStyle_(
        presentationStyle: MKMapItemDetailSelectionAccessoryPresentationStyle,
    ) -> MKSelectionAccessory
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MKSelectionAccessory").unwrap(), mapItemDetailWithPresentationStyle : presentationStyle)
    }
}
pub type MKMapItemDetailSelectionAccessoryCalloutStyle = NSInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MKMapItemDetailSelectionAccessoryPresentationStyle(pub id);
impl std::ops::Deref for MKMapItemDetailSelectionAccessoryPresentationStyle {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MKMapItemDetailSelectionAccessoryPresentationStyle {}
impl MKMapItemDetailSelectionAccessoryPresentationStyle {
    pub fn alloc() -> Self {
        Self(unsafe {
            msg_send!(&*objc2::runtime::AnyClass::get(c"MKMapItemDetailSelectionAccessoryPresentationStyle").unwrap(), alloc)
        })
    }
}
impl INSObject for MKMapItemDetailSelectionAccessoryPresentationStyle {}
impl PNSObject for MKMapItemDetailSelectionAccessoryPresentationStyle {}
impl std::convert::TryFrom<NSObject> for MKMapItemDetailSelectionAccessoryPresentationStyle {
    type Error = &'static str;
    fn try_from(
        parent: NSObject,
    ) -> Result<MKMapItemDetailSelectionAccessoryPresentationStyle, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MKMapItemDetailSelectionAccessoryPresentationStyle").unwrap())
        };
        if is_kind_of {
            Ok(MKMapItemDetailSelectionAccessoryPresentationStyle(parent.0))
        } else {
            Err ("This NSObject cannot be downcasted to MKMapItemDetailSelectionAccessoryPresentationStyle" ,)
        }
    }
}
impl IMKMapItemDetailSelectionAccessoryPresentationStyle
    for MKMapItemDetailSelectionAccessoryPresentationStyle
{
}
pub trait IMKMapItemDetailSelectionAccessoryPresentationStyle: Sized + std::ops::Deref {
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
        msg_send!(&*objc2::runtime::AnyClass::get(c"MKMapItemDetailSelectionAccessoryPresentationStyle").unwrap(), new)
    }
    unsafe fn automaticWithPresentationViewController_(
        presentationViewController: NSViewController,
    ) -> MKMapItemDetailSelectionAccessoryPresentationStyle
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MKMapItemDetailSelectionAccessoryPresentationStyle").unwrap(), automaticWithPresentationViewController : presentationViewController)
    }
    unsafe fn calloutWithCalloutStyle_(
        style: MKMapItemDetailSelectionAccessoryCalloutStyle,
    ) -> MKMapItemDetailSelectionAccessoryPresentationStyle
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MKMapItemDetailSelectionAccessoryPresentationStyle").unwrap(), calloutWithCalloutStyle : style)
    }
    unsafe fn sheetPresentedFromViewController_(
        viewController: NSViewController,
    ) -> MKMapItemDetailSelectionAccessoryPresentationStyle
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MKMapItemDetailSelectionAccessoryPresentationStyle").unwrap(), sheetPresentedFromViewController : viewController)
    }
    unsafe fn callout() -> MKMapItemDetailSelectionAccessoryPresentationStyle
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MKMapItemDetailSelectionAccessoryPresentationStyle").unwrap(), callout)
    }
    unsafe fn openInMaps() -> MKMapItemDetailSelectionAccessoryPresentationStyle
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MKMapItemDetailSelectionAccessoryPresentationStyle").unwrap(), openInMaps)
    }
}
pub type MKOverlayLevel = NSInteger;
pub type MKUserTrackingMode = NSInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MKMapView(pub id);
impl std::ops::Deref for MKMapView {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MKMapView {}
impl MKMapView {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MKMapView").unwrap(), alloc) })
    }
}
impl PNSCoding for MKMapView {}
impl INSView for MKMapView {}
impl PNSAnimatablePropertyContainer for MKMapView {}
impl PNSUserInterfaceItemIdentification for MKMapView {}
impl PNSDraggingDestination for MKMapView {}
impl PNSAppearanceCustomization for MKMapView {}
impl PNSAccessibilityElement for MKMapView {}
impl PNSAccessibility for MKMapView {}
impl std::convert::TryFrom<NSView> for MKMapView {
    type Error = &'static str;
    fn try_from(parent: NSView) -> Result<MKMapView, Self::Error> {
        let is_kind_of: bool = unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MKMapView").unwrap()) };
        if is_kind_of {
            Ok(MKMapView(parent.0))
        } else {
            Err("This NSView cannot be downcasted to MKMapView")
        }
    }
}
impl INSResponder for MKMapView {}
impl INSObject for MKMapView {}
impl PNSObject for MKMapView {}
impl IMKMapView for MKMapView {}
pub trait IMKMapView: Sized + std::ops::Deref {
    unsafe fn setRegion_animated_(&self, region: MKCoordinateRegion, animated: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRegion : region, animated : animated)
    }
    unsafe fn setCenterCoordinate_animated_(
        &self,
        coordinate: CLLocationCoordinate2D,
        animated: BOOL,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCenterCoordinate : coordinate, animated : animated)
    }
    unsafe fn regionThatFits_(&self, region: MKCoordinateRegion) -> MKCoordinateRegion
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, regionThatFits : region)
    }
    unsafe fn setVisibleMapRect_animated_(&self, mapRect: MKMapRect, animate: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setVisibleMapRect : mapRect, animated : animate)
    }
    unsafe fn mapRectThatFits_(&self, mapRect: MKMapRect) -> MKMapRect
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, mapRectThatFits : mapRect)
    }
    unsafe fn setVisibleMapRect_edgePadding_animated_(
        &self,
        mapRect: MKMapRect,
        insets: NSEdgeInsets,
        animate: BOOL,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setVisibleMapRect : mapRect, edgePadding : insets, animated : animate)
    }
    unsafe fn mapRectThatFits_edgePadding_(
        &self,
        mapRect: MKMapRect,
        insets: NSEdgeInsets,
    ) -> MKMapRect
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, mapRectThatFits : mapRect, edgePadding : insets)
    }
    unsafe fn setCamera_animated_(&self, camera: MKMapCamera, animated: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCamera : camera, animated : animated)
    }
    unsafe fn setCameraZoomRange_animated_(
        &self,
        cameraZoomRange: MKMapCameraZoomRange,
        animated: BOOL,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCameraZoomRange : cameraZoomRange, animated : animated)
    }
    unsafe fn setCameraBoundary_animated_(
        &self,
        cameraBoundary: MKMapCameraBoundary,
        animated: BOOL,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCameraBoundary : cameraBoundary, animated : animated)
    }
    unsafe fn convertCoordinate_toPointToView_(
        &self,
        coordinate: CLLocationCoordinate2D,
        view: NSView,
    ) -> CGPoint
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, convertCoordinate : coordinate, toPointToView : view)
    }
    unsafe fn convertPoint_toCoordinateFromView_(
        &self,
        point: CGPoint,
        view: NSView,
    ) -> CLLocationCoordinate2D
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, convertPoint : point, toCoordinateFromView : view)
    }
    unsafe fn convertRegion_toRectToView_(&self, region: MKCoordinateRegion, view: NSView) -> CGRect
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, convertRegion : region, toRectToView : view)
    }
    unsafe fn convertRect_toRegionFromView_(&self, rect: CGRect, view: NSView) -> MKCoordinateRegion
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, convertRect : rect, toRegionFromView : view)
    }
    unsafe fn setUserTrackingMode_animated_(&self, mode: MKUserTrackingMode, animated: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setUserTrackingMode : mode, animated : animated)
    }
    unsafe fn addAnnotation_(&self, annotation: *mut u64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, addAnnotation : annotation)
    }
    unsafe fn addAnnotations_(&self, annotations: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, addAnnotations : annotations)
    }
    unsafe fn removeAnnotation_(&self, annotation: *mut u64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, removeAnnotation : annotation)
    }
    unsafe fn removeAnnotations_(&self, annotations: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, removeAnnotations : annotations)
    }
    unsafe fn annotationsInMapRect_(&self, mapRect: MKMapRect) -> NSSet
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, annotationsInMapRect : mapRect)
    }
    unsafe fn viewForAnnotation_(&self, annotation: *mut u64) -> MKAnnotationView
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, viewForAnnotation : annotation)
    }
    unsafe fn dequeueReusableAnnotationViewWithIdentifier_(
        &self,
        identifier: NSString,
    ) -> MKAnnotationView
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, dequeueReusableAnnotationViewWithIdentifier : identifier)
    }
    unsafe fn dequeueReusableAnnotationViewWithIdentifier_forAnnotation_(
        &self,
        identifier: NSString,
        annotation: *mut u64,
    ) -> MKAnnotationView
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, dequeueReusableAnnotationViewWithIdentifier : identifier, forAnnotation : annotation)
    }
    unsafe fn registerClass_forAnnotationViewWithReuseIdentifier_(
        &self,
        viewClass: Class,
        identifier: NSString,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, registerClass : viewClass, forAnnotationViewWithReuseIdentifier : identifier)
    }
    unsafe fn selectAnnotation_animated_(&self, annotation: *mut u64, animated: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, selectAnnotation : annotation, animated : animated)
    }
    unsafe fn deselectAnnotation_animated_(&self, annotation: *mut u64, animated: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, deselectAnnotation : annotation, animated : animated)
    }
    unsafe fn showAnnotations_animated_(&self, annotations: NSArray, animated: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, showAnnotations : annotations, animated : animated)
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
    unsafe fn mapType(&self) -> MKMapType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, mapType)
    }
    unsafe fn setMapType_(&self, mapType: MKMapType)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMapType : mapType)
    }
    unsafe fn preferredConfiguration(&self) -> MKMapConfiguration
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, preferredConfiguration)
    }
    unsafe fn setPreferredConfiguration_(&self, preferredConfiguration: MKMapConfiguration)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPreferredConfiguration : preferredConfiguration)
    }
    unsafe fn region(&self) -> MKCoordinateRegion
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, region)
    }
    unsafe fn setRegion_(&self, region: MKCoordinateRegion)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRegion : region)
    }
    unsafe fn centerCoordinate(&self) -> CLLocationCoordinate2D
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, centerCoordinate)
    }
    unsafe fn setCenterCoordinate_(&self, centerCoordinate: CLLocationCoordinate2D)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCenterCoordinate : centerCoordinate)
    }
    unsafe fn visibleMapRect(&self) -> MKMapRect
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, visibleMapRect)
    }
    unsafe fn setVisibleMapRect_(&self, visibleMapRect: MKMapRect)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setVisibleMapRect : visibleMapRect)
    }
    unsafe fn camera(&self) -> MKMapCamera
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, camera)
    }
    unsafe fn setCamera_(&self, camera: MKMapCamera)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCamera : camera)
    }
    unsafe fn cameraZoomRange(&self) -> MKMapCameraZoomRange
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, cameraZoomRange)
    }
    unsafe fn setCameraZoomRange_(&self, cameraZoomRange: MKMapCameraZoomRange)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCameraZoomRange : cameraZoomRange)
    }
    unsafe fn cameraBoundary(&self) -> MKMapCameraBoundary
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, cameraBoundary)
    }
    unsafe fn setCameraBoundary_(&self, cameraBoundary: MKMapCameraBoundary)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCameraBoundary : cameraBoundary)
    }
    unsafe fn isZoomEnabled(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isZoomEnabled)
    }
    unsafe fn setZoomEnabled_(&self, zoomEnabled: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setZoomEnabled : zoomEnabled)
    }
    unsafe fn isScrollEnabled(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isScrollEnabled)
    }
    unsafe fn setScrollEnabled_(&self, scrollEnabled: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setScrollEnabled : scrollEnabled)
    }
    unsafe fn isRotateEnabled(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isRotateEnabled)
    }
    unsafe fn setRotateEnabled_(&self, rotateEnabled: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRotateEnabled : rotateEnabled)
    }
    unsafe fn isPitchEnabled(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isPitchEnabled)
    }
    unsafe fn setPitchEnabled_(&self, pitchEnabled: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPitchEnabled : pitchEnabled)
    }
    unsafe fn showsUserTrackingButton(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, showsUserTrackingButton)
    }
    unsafe fn setShowsUserTrackingButton_(&self, showsUserTrackingButton: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setShowsUserTrackingButton : showsUserTrackingButton)
    }
    unsafe fn pitchButtonVisibility(&self) -> MKFeatureVisibility
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, pitchButtonVisibility)
    }
    unsafe fn setPitchButtonVisibility_(&self, pitchButtonVisibility: MKFeatureVisibility)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPitchButtonVisibility : pitchButtonVisibility)
    }
    unsafe fn showsPitchControl(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, showsPitchControl)
    }
    unsafe fn setShowsPitchControl_(&self, showsPitchControl: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setShowsPitchControl : showsPitchControl)
    }
    unsafe fn showsZoomControls(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, showsZoomControls)
    }
    unsafe fn setShowsZoomControls_(&self, showsZoomControls: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setShowsZoomControls : showsZoomControls)
    }
    unsafe fn showsCompass(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, showsCompass)
    }
    unsafe fn setShowsCompass_(&self, showsCompass: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setShowsCompass : showsCompass)
    }
    unsafe fn showsScale(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, showsScale)
    }
    unsafe fn setShowsScale_(&self, showsScale: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setShowsScale : showsScale)
    }
    unsafe fn pointOfInterestFilter(&self) -> MKPointOfInterestFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, pointOfInterestFilter)
    }
    unsafe fn setPointOfInterestFilter_(&self, pointOfInterestFilter: MKPointOfInterestFilter)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPointOfInterestFilter : pointOfInterestFilter)
    }
    unsafe fn showsPointsOfInterest(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, showsPointsOfInterest)
    }
    unsafe fn setShowsPointsOfInterest_(&self, showsPointsOfInterest: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setShowsPointsOfInterest : showsPointsOfInterest)
    }
    unsafe fn showsBuildings(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, showsBuildings)
    }
    unsafe fn setShowsBuildings_(&self, showsBuildings: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setShowsBuildings : showsBuildings)
    }
    unsafe fn showsTraffic(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, showsTraffic)
    }
    unsafe fn setShowsTraffic_(&self, showsTraffic: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setShowsTraffic : showsTraffic)
    }
    unsafe fn showsUserLocation(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, showsUserLocation)
    }
    unsafe fn setShowsUserLocation_(&self, showsUserLocation: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setShowsUserLocation : showsUserLocation)
    }
    unsafe fn userLocation(&self) -> MKUserLocation
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, userLocation)
    }
    unsafe fn userTrackingMode(&self) -> MKUserTrackingMode
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, userTrackingMode)
    }
    unsafe fn setUserTrackingMode_(&self, userTrackingMode: MKUserTrackingMode)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setUserTrackingMode : userTrackingMode)
    }
    unsafe fn isUserLocationVisible(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isUserLocationVisible)
    }
    unsafe fn annotations(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, annotations)
    }
    unsafe fn selectedAnnotations(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, selectedAnnotations)
    }
    unsafe fn setSelectedAnnotations_(&self, selectedAnnotations: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSelectedAnnotations : selectedAnnotations)
    }
    unsafe fn annotationVisibleRect(&self) -> CGRect
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, annotationVisibleRect)
    }
}
impl MKMapView_OverlaysAPI for MKMapView {}
pub trait MKMapView_OverlaysAPI: Sized + std::ops::Deref {
    unsafe fn addOverlay_level_(&self, overlay: *mut u64, level: MKOverlayLevel)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, addOverlay : overlay, level : level)
    }
    unsafe fn addOverlays_level_(&self, overlays: NSArray, level: MKOverlayLevel)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, addOverlays : overlays, level : level)
    }
    unsafe fn removeOverlay_(&self, overlay: *mut u64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, removeOverlay : overlay)
    }
    unsafe fn removeOverlays_(&self, overlays: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, removeOverlays : overlays)
    }
    unsafe fn insertOverlay_atIndex_level_(
        &self,
        overlay: *mut u64,
        index: NSUInteger,
        level: MKOverlayLevel,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, insertOverlay : overlay, atIndex : index, level : level)
    }
    unsafe fn insertOverlay_aboveOverlay_(&self, overlay: *mut u64, sibling: *mut u64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, insertOverlay : overlay, aboveOverlay : sibling)
    }
    unsafe fn insertOverlay_belowOverlay_(&self, overlay: *mut u64, sibling: *mut u64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, insertOverlay : overlay, belowOverlay : sibling)
    }
    unsafe fn exchangeOverlay_withOverlay_(&self, overlay1: *mut u64, overlay2: *mut u64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, exchangeOverlay : overlay1, withOverlay : overlay2)
    }
    unsafe fn overlaysInLevel_(&self, level: MKOverlayLevel) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, overlaysInLevel : level)
    }
    unsafe fn rendererForOverlay_(&self, overlay: *mut u64) -> MKOverlayRenderer
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, rendererForOverlay : overlay)
    }
    unsafe fn addOverlay_(&self, overlay: *mut u64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, addOverlay : overlay)
    }
    unsafe fn addOverlays_(&self, overlays: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, addOverlays : overlays)
    }
    unsafe fn insertOverlay_atIndex_(&self, overlay: *mut u64, index: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, insertOverlay : overlay, atIndex : index)
    }
    unsafe fn exchangeOverlayAtIndex_withOverlayAtIndex_(
        &self,
        index1: NSUInteger,
        index2: NSUInteger,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, exchangeOverlayAtIndex : index1, withOverlayAtIndex : index2)
    }
    unsafe fn overlays(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, overlays)
    }
}
pub trait PMKMapViewDelegate: Sized + std::ops::Deref {
    unsafe fn mapView_regionWillChangeAnimated_(&self, mapView: MKMapView, animated: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, mapView : mapView, regionWillChangeAnimated : animated)
    }
    unsafe fn mapView_regionDidChangeAnimated_(&self, mapView: MKMapView, animated: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, mapView : mapView, regionDidChangeAnimated : animated)
    }
    unsafe fn mapViewDidChangeVisibleRegion_(&self, mapView: MKMapView)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, mapViewDidChangeVisibleRegion : mapView)
    }
    unsafe fn mapViewWillStartLoadingMap_(&self, mapView: MKMapView)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, mapViewWillStartLoadingMap : mapView)
    }
    unsafe fn mapViewDidFinishLoadingMap_(&self, mapView: MKMapView)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, mapViewDidFinishLoadingMap : mapView)
    }
    unsafe fn mapViewDidFailLoadingMap_withError_(&self, mapView: MKMapView, error: NSError)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, mapViewDidFailLoadingMap : mapView, withError : error)
    }
    unsafe fn mapViewWillStartRenderingMap_(&self, mapView: MKMapView)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, mapViewWillStartRenderingMap : mapView)
    }
    unsafe fn mapViewDidFinishRenderingMap_fullyRendered_(
        &self,
        mapView: MKMapView,
        fullyRendered: BOOL,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, mapViewDidFinishRenderingMap : mapView, fullyRendered : fullyRendered)
    }
    unsafe fn mapView_viewForAnnotation_(
        &self,
        mapView: MKMapView,
        annotation: *mut u64,
    ) -> MKAnnotationView
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, mapView : mapView, viewForAnnotation : annotation)
    }
    unsafe fn mapView_didAddAnnotationViews_(&self, mapView: MKMapView, views: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, mapView : mapView, didAddAnnotationViews : views)
    }
    unsafe fn mapView_didSelectAnnotationView_(&self, mapView: MKMapView, view: MKAnnotationView)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, mapView : mapView, didSelectAnnotationView : view)
    }
    unsafe fn mapView_didDeselectAnnotationView_(&self, mapView: MKMapView, view: MKAnnotationView)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, mapView : mapView, didDeselectAnnotationView : view)
    }
    unsafe fn mapView_didSelectAnnotation_(&self, mapView: MKMapView, annotation: *mut u64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, mapView : mapView, didSelectAnnotation : annotation)
    }
    unsafe fn mapView_didDeselectAnnotation_(&self, mapView: MKMapView, annotation: *mut u64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, mapView : mapView, didDeselectAnnotation : annotation)
    }
    unsafe fn mapView_selectionAccessoryForAnnotation_(
        &self,
        mapView: MKMapView,
        annotation: *mut u64,
    ) -> MKSelectionAccessory
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, mapView : mapView, selectionAccessoryForAnnotation : annotation)
    }
    unsafe fn mapViewWillStartLocatingUser_(&self, mapView: MKMapView)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, mapViewWillStartLocatingUser : mapView)
    }
    unsafe fn mapViewDidStopLocatingUser_(&self, mapView: MKMapView)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, mapViewDidStopLocatingUser : mapView)
    }
    unsafe fn mapView_didUpdateUserLocation_(
        &self,
        mapView: MKMapView,
        userLocation: MKUserLocation,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, mapView : mapView, didUpdateUserLocation : userLocation)
    }
    unsafe fn mapView_didFailToLocateUserWithError_(&self, mapView: MKMapView, error: NSError)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, mapView : mapView, didFailToLocateUserWithError : error)
    }
    unsafe fn mapView_annotationView_didChangeDragState_fromOldState_(
        &self,
        mapView: MKMapView,
        view: MKAnnotationView,
        newState: MKAnnotationViewDragState,
        oldState: MKAnnotationViewDragState,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, mapView : mapView, annotationView : view, didChangeDragState : newState, fromOldState : oldState)
    }
    unsafe fn mapView_didChangeUserTrackingMode_animated_(
        &self,
        mapView: MKMapView,
        mode: MKUserTrackingMode,
        animated: BOOL,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, mapView : mapView, didChangeUserTrackingMode : mode, animated : animated)
    }
    unsafe fn mapView_rendererForOverlay_(
        &self,
        mapView: MKMapView,
        overlay: *mut u64,
    ) -> MKOverlayRenderer
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, mapView : mapView, rendererForOverlay : overlay)
    }
    unsafe fn mapView_didAddOverlayRenderers_(&self, mapView: MKMapView, renderers: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, mapView : mapView, didAddOverlayRenderers : renderers)
    }
    unsafe fn mapView_clusterAnnotationForMemberAnnotations_(
        &self,
        mapView: MKMapView,
        memberAnnotations: NSArray,
    ) -> MKClusterAnnotation
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, mapView : mapView, clusterAnnotationForMemberAnnotations : memberAnnotations)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MKMapSnapshotOptions(pub id);
impl std::ops::Deref for MKMapSnapshotOptions {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MKMapSnapshotOptions {}
impl MKMapSnapshotOptions {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MKMapSnapshotOptions").unwrap(), alloc) })
    }
}
impl PNSCopying for MKMapSnapshotOptions {}
impl INSObject for MKMapSnapshotOptions {}
impl PNSObject for MKMapSnapshotOptions {}
impl std::convert::TryFrom<NSObject> for MKMapSnapshotOptions {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<MKMapSnapshotOptions, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MKMapSnapshotOptions").unwrap()) };
        if is_kind_of {
            Ok(MKMapSnapshotOptions(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to MKMapSnapshotOptions")
        }
    }
}
impl IMKMapSnapshotOptions for MKMapSnapshotOptions {}
pub trait IMKMapSnapshotOptions: Sized + std::ops::Deref {
    unsafe fn preferredConfiguration(&self) -> MKMapConfiguration
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, preferredConfiguration)
    }
    unsafe fn setPreferredConfiguration_(&self, preferredConfiguration: MKMapConfiguration)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPreferredConfiguration : preferredConfiguration)
    }
    unsafe fn camera(&self) -> MKMapCamera
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, camera)
    }
    unsafe fn setCamera_(&self, camera: MKMapCamera)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCamera : camera)
    }
    unsafe fn mapRect(&self) -> MKMapRect
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, mapRect)
    }
    unsafe fn setMapRect_(&self, mapRect: MKMapRect)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMapRect : mapRect)
    }
    unsafe fn region(&self) -> MKCoordinateRegion
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, region)
    }
    unsafe fn setRegion_(&self, region: MKCoordinateRegion)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRegion : region)
    }
    unsafe fn mapType(&self) -> MKMapType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, mapType)
    }
    unsafe fn setMapType_(&self, mapType: MKMapType)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMapType : mapType)
    }
    unsafe fn pointOfInterestFilter(&self) -> MKPointOfInterestFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, pointOfInterestFilter)
    }
    unsafe fn setPointOfInterestFilter_(&self, pointOfInterestFilter: MKPointOfInterestFilter)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPointOfInterestFilter : pointOfInterestFilter)
    }
    unsafe fn showsPointsOfInterest(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, showsPointsOfInterest)
    }
    unsafe fn setShowsPointsOfInterest_(&self, showsPointsOfInterest: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setShowsPointsOfInterest : showsPointsOfInterest)
    }
    unsafe fn showsBuildings(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, showsBuildings)
    }
    unsafe fn setShowsBuildings_(&self, showsBuildings: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setShowsBuildings : showsBuildings)
    }
    unsafe fn size(&self) -> NSSize
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, size)
    }
    unsafe fn setSize_(&self, size: NSSize)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSize : size)
    }
    unsafe fn appearance(&self) -> NSAppearance
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, appearance)
    }
    unsafe fn setAppearance_(&self, appearance: NSAppearance)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAppearance : appearance)
    }
}
pub type MKMapSnapshotCompletionHandler = *mut ::std::os::raw::c_void;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MKMapSnapshotter(pub id);
impl std::ops::Deref for MKMapSnapshotter {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MKMapSnapshotter {}
impl MKMapSnapshotter {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MKMapSnapshotter").unwrap(), alloc) })
    }
}
impl INSObject for MKMapSnapshotter {}
impl PNSObject for MKMapSnapshotter {}
impl std::convert::TryFrom<NSObject> for MKMapSnapshotter {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<MKMapSnapshotter, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MKMapSnapshotter").unwrap()) };
        if is_kind_of {
            Ok(MKMapSnapshotter(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to MKMapSnapshotter")
        }
    }
}
impl IMKMapSnapshotter for MKMapSnapshotter {}
pub trait IMKMapSnapshotter: Sized + std::ops::Deref {
    unsafe fn initWithOptions_(&self, options: MKMapSnapshotOptions) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithOptions : options)
    }
    unsafe fn startWithCompletionHandler_(&self, completionHandler: MKMapSnapshotCompletionHandler)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, startWithCompletionHandler : completionHandler)
    }
    unsafe fn startWithQueue_completionHandler_(
        &self,
        queue: NSObject,
        completionHandler: MKMapSnapshotCompletionHandler,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, startWithQueue : queue, completionHandler : completionHandler)
    }
    unsafe fn cancel(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, cancel)
    }
    unsafe fn isLoading(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isLoading)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MKReverseGeocodingRequest(pub id);
impl std::ops::Deref for MKReverseGeocodingRequest {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MKReverseGeocodingRequest {}
impl MKReverseGeocodingRequest {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MKReverseGeocodingRequest").unwrap(), alloc) })
    }
}
impl INSObject for MKReverseGeocodingRequest {}
impl PNSObject for MKReverseGeocodingRequest {}
impl std::convert::TryFrom<NSObject> for MKReverseGeocodingRequest {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<MKReverseGeocodingRequest, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MKReverseGeocodingRequest").unwrap()) };
        if is_kind_of {
            Ok(MKReverseGeocodingRequest(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to MKReverseGeocodingRequest")
        }
    }
}
impl IMKReverseGeocodingRequest for MKReverseGeocodingRequest {}
pub trait IMKReverseGeocodingRequest: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn initWithLocation_(&self, location: CLLocation) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithLocation : location)
    }
    unsafe fn getMapItemsWithCompletionHandler_(
        &self,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, getMapItemsWithCompletionHandler : completionHandler)
    }
    unsafe fn cancel(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, cancel)
    }
    unsafe fn isCancelled(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isCancelled)
    }
    unsafe fn isLoading(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isLoading)
    }
    unsafe fn location(&self) -> CLLocation
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, location)
    }
    unsafe fn preferredLocale(&self) -> NSLocale
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, preferredLocale)
    }
    unsafe fn setPreferredLocale_(&self, preferredLocale: NSLocale)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPreferredLocale : preferredLocale)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MKReverseGeocodingRequest").unwrap(), new)
    }
}
pub type MKStandardMapEmphasisStyle = NSInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MKStandardMapConfiguration(pub id);
impl std::ops::Deref for MKStandardMapConfiguration {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MKStandardMapConfiguration {}
impl MKStandardMapConfiguration {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MKStandardMapConfiguration").unwrap(), alloc) })
    }
}
impl IMKMapConfiguration for MKStandardMapConfiguration {}
impl PNSSecureCoding for MKStandardMapConfiguration {}
impl PNSCopying for MKStandardMapConfiguration {}
impl From<MKStandardMapConfiguration> for MKMapConfiguration {
    fn from(child: MKStandardMapConfiguration) -> MKMapConfiguration {
        MKMapConfiguration(child.0)
    }
}
impl std::convert::TryFrom<MKMapConfiguration> for MKStandardMapConfiguration {
    type Error = &'static str;
    fn try_from(parent: MKMapConfiguration) -> Result<MKStandardMapConfiguration, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MKStandardMapConfiguration").unwrap()) };
        if is_kind_of {
            Ok(MKStandardMapConfiguration(parent.0))
        } else {
            Err("This MKMapConfiguration cannot be downcasted to MKStandardMapConfiguration")
        }
    }
}
impl INSObject for MKStandardMapConfiguration {}
impl PNSObject for MKStandardMapConfiguration {}
impl IMKStandardMapConfiguration for MKStandardMapConfiguration {}
pub trait IMKStandardMapConfiguration: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn initWithElevationStyle_(&self, elevationStyle: MKMapElevationStyle) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithElevationStyle : elevationStyle)
    }
    unsafe fn initWithElevationStyle_emphasisStyle_(
        &self,
        elevationStyle: MKMapElevationStyle,
        emphasisStyle: MKStandardMapEmphasisStyle,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithElevationStyle : elevationStyle, emphasisStyle : emphasisStyle)
    }
    unsafe fn initWithEmphasisStyle_(
        &self,
        emphasisStyle: MKStandardMapEmphasisStyle,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithEmphasisStyle : emphasisStyle)
    }
    unsafe fn emphasisStyle(&self) -> MKStandardMapEmphasisStyle
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, emphasisStyle)
    }
    unsafe fn setEmphasisStyle_(&self, emphasisStyle: MKStandardMapEmphasisStyle)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setEmphasisStyle : emphasisStyle)
    }
    unsafe fn pointOfInterestFilter(&self) -> MKPointOfInterestFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, pointOfInterestFilter)
    }
    unsafe fn setPointOfInterestFilter_(&self, pointOfInterestFilter: MKPointOfInterestFilter)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPointOfInterestFilter : pointOfInterestFilter)
    }
    unsafe fn showsTraffic(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, showsTraffic)
    }
    unsafe fn setShowsTraffic_(&self, showsTraffic: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setShowsTraffic : showsTraffic)
    }
}
pub trait NSUserActivity_MKMapItem: Sized + std::ops::Deref {
    unsafe fn mapItem(&self) -> MKMapItem
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, mapItem)
    }
    unsafe fn setMapItem_(&self, mapItem: MKMapItem)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMapItem : mapItem)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MKMarkerAnnotationView(pub id);
impl std::ops::Deref for MKMarkerAnnotationView {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MKMarkerAnnotationView {}
impl MKMarkerAnnotationView {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MKMarkerAnnotationView").unwrap(), alloc) })
    }
}
impl IMKAnnotationView for MKMarkerAnnotationView {}
impl From<MKMarkerAnnotationView> for MKAnnotationView {
    fn from(child: MKMarkerAnnotationView) -> MKAnnotationView {
        MKAnnotationView(child.0)
    }
}
impl std::convert::TryFrom<MKAnnotationView> for MKMarkerAnnotationView {
    type Error = &'static str;
    fn try_from(parent: MKAnnotationView) -> Result<MKMarkerAnnotationView, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MKMarkerAnnotationView").unwrap()) };
        if is_kind_of {
            Ok(MKMarkerAnnotationView(parent.0))
        } else {
            Err("This MKAnnotationView cannot be downcasted to MKMarkerAnnotationView")
        }
    }
}
impl INSView for MKMarkerAnnotationView {}
impl PNSAnimatablePropertyContainer for MKMarkerAnnotationView {}
impl PNSUserInterfaceItemIdentification for MKMarkerAnnotationView {}
impl PNSDraggingDestination for MKMarkerAnnotationView {}
impl PNSAppearanceCustomization for MKMarkerAnnotationView {}
impl PNSAccessibilityElement for MKMarkerAnnotationView {}
impl PNSAccessibility for MKMarkerAnnotationView {}
impl INSResponder for MKMarkerAnnotationView {}
impl PNSCoding for MKMarkerAnnotationView {}
impl INSObject for MKMarkerAnnotationView {}
impl PNSObject for MKMarkerAnnotationView {}
impl IMKMarkerAnnotationView for MKMarkerAnnotationView {}
pub trait IMKMarkerAnnotationView: Sized + std::ops::Deref {
    unsafe fn titleVisibility(&self) -> MKFeatureVisibility
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, titleVisibility)
    }
    unsafe fn setTitleVisibility_(&self, titleVisibility: MKFeatureVisibility)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setTitleVisibility : titleVisibility)
    }
    unsafe fn subtitleVisibility(&self) -> MKFeatureVisibility
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, subtitleVisibility)
    }
    unsafe fn setSubtitleVisibility_(&self, subtitleVisibility: MKFeatureVisibility)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSubtitleVisibility : subtitleVisibility)
    }
    unsafe fn markerTintColor(&self) -> NSColor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, markerTintColor)
    }
    unsafe fn setMarkerTintColor_(&self, markerTintColor: NSColor)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMarkerTintColor : markerTintColor)
    }
    unsafe fn glyphTintColor(&self) -> NSColor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, glyphTintColor)
    }
    unsafe fn setGlyphTintColor_(&self, glyphTintColor: NSColor)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setGlyphTintColor : glyphTintColor)
    }
    unsafe fn glyphText(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, glyphText)
    }
    unsafe fn setGlyphText_(&self, glyphText: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setGlyphText : glyphText)
    }
    unsafe fn glyphImage(&self) -> NSImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, glyphImage)
    }
    unsafe fn setGlyphImage_(&self, glyphImage: NSImage)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setGlyphImage : glyphImage)
    }
    unsafe fn selectedGlyphImage(&self) -> NSImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, selectedGlyphImage)
    }
    unsafe fn setSelectedGlyphImage_(&self, selectedGlyphImage: NSImage)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSelectedGlyphImage : selectedGlyphImage)
    }
    unsafe fn animatesWhenAdded(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, animatesWhenAdded)
    }
    unsafe fn setAnimatesWhenAdded_(&self, animatesWhenAdded: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAnimatesWhenAdded : animatesWhenAdded)
    }
}
pub type MKPinAnnotationColor = NSUInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MKPinAnnotationView(pub id);
impl std::ops::Deref for MKPinAnnotationView {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MKPinAnnotationView {}
impl MKPinAnnotationView {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MKPinAnnotationView").unwrap(), alloc) })
    }
}
impl IMKAnnotationView for MKPinAnnotationView {}
impl std::convert::TryFrom<MKAnnotationView> for MKPinAnnotationView {
    type Error = &'static str;
    fn try_from(parent: MKAnnotationView) -> Result<MKPinAnnotationView, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MKPinAnnotationView").unwrap()) };
        if is_kind_of {
            Ok(MKPinAnnotationView(parent.0))
        } else {
            Err("This MKAnnotationView cannot be downcasted to MKPinAnnotationView")
        }
    }
}
impl INSView for MKPinAnnotationView {}
impl PNSAnimatablePropertyContainer for MKPinAnnotationView {}
impl PNSUserInterfaceItemIdentification for MKPinAnnotationView {}
impl PNSDraggingDestination for MKPinAnnotationView {}
impl PNSAppearanceCustomization for MKPinAnnotationView {}
impl PNSAccessibilityElement for MKPinAnnotationView {}
impl PNSAccessibility for MKPinAnnotationView {}
impl INSResponder for MKPinAnnotationView {}
impl PNSCoding for MKPinAnnotationView {}
impl INSObject for MKPinAnnotationView {}
impl PNSObject for MKPinAnnotationView {}
impl IMKPinAnnotationView for MKPinAnnotationView {}
pub trait IMKPinAnnotationView: Sized + std::ops::Deref {
    unsafe fn pinTintColor(&self) -> NSColor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, pinTintColor)
    }
    unsafe fn setPinTintColor_(&self, pinTintColor: NSColor)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPinTintColor : pinTintColor)
    }
    unsafe fn animatesDrop(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, animatesDrop)
    }
    unsafe fn setAnimatesDrop_(&self, animatesDrop: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAnimatesDrop : animatesDrop)
    }
    unsafe fn pinColor(&self) -> MKPinAnnotationColor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, pinColor)
    }
    unsafe fn setPinColor_(&self, pinColor: MKPinAnnotationColor)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPinColor : pinColor)
    }
    unsafe fn redPinColor() -> NSColor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MKPinAnnotationView").unwrap(), redPinColor)
    }
    unsafe fn greenPinColor() -> NSColor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MKPinAnnotationView").unwrap(), greenPinColor)
    }
    unsafe fn purplePinColor() -> NSColor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MKPinAnnotationView").unwrap(), purplePinColor)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MKUserLocation(pub id);
impl std::ops::Deref for MKUserLocation {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MKUserLocation {}
impl MKUserLocation {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MKUserLocation").unwrap(), alloc) })
    }
}
impl PMKAnnotation for MKUserLocation {}
impl INSObject for MKUserLocation {}
impl PNSObject for MKUserLocation {}
impl std::convert::TryFrom<NSObject> for MKUserLocation {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<MKUserLocation, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MKUserLocation").unwrap()) };
        if is_kind_of {
            Ok(MKUserLocation(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to MKUserLocation")
        }
    }
}
impl IMKUserLocation for MKUserLocation {}
pub trait IMKUserLocation: Sized + std::ops::Deref {
    unsafe fn isUpdating(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isUpdating)
    }
    unsafe fn location(&self) -> CLLocation
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, location)
    }
    unsafe fn heading(&self) -> CLHeading
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, heading)
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
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MKUserLocationView(pub id);
impl std::ops::Deref for MKUserLocationView {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MKUserLocationView {}
impl MKUserLocationView {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MKUserLocationView").unwrap(), alloc) })
    }
}
impl IMKAnnotationView for MKUserLocationView {}
impl std::convert::TryFrom<MKAnnotationView> for MKUserLocationView {
    type Error = &'static str;
    fn try_from(parent: MKAnnotationView) -> Result<MKUserLocationView, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MKUserLocationView").unwrap()) };
        if is_kind_of {
            Ok(MKUserLocationView(parent.0))
        } else {
            Err("This MKAnnotationView cannot be downcasted to MKUserLocationView")
        }
    }
}
impl INSView for MKUserLocationView {}
impl PNSAnimatablePropertyContainer for MKUserLocationView {}
impl PNSUserInterfaceItemIdentification for MKUserLocationView {}
impl PNSDraggingDestination for MKUserLocationView {}
impl PNSAppearanceCustomization for MKUserLocationView {}
impl PNSAccessibilityElement for MKUserLocationView {}
impl PNSAccessibility for MKUserLocationView {}
impl INSResponder for MKUserLocationView {}
impl PNSCoding for MKUserLocationView {}
impl INSObject for MKUserLocationView {}
impl PNSObject for MKUserLocationView {}
impl IMKUserLocationView for MKUserLocationView {}
pub trait IMKUserLocationView: Sized + std::ops::Deref {}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MKClusterAnnotation(pub id);
impl std::ops::Deref for MKClusterAnnotation {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MKClusterAnnotation {}
impl MKClusterAnnotation {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MKClusterAnnotation").unwrap(), alloc) })
    }
}
impl PMKAnnotation for MKClusterAnnotation {}
impl INSObject for MKClusterAnnotation {}
impl PNSObject for MKClusterAnnotation {}
impl std::convert::TryFrom<NSObject> for MKClusterAnnotation {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<MKClusterAnnotation, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MKClusterAnnotation").unwrap()) };
        if is_kind_of {
            Ok(MKClusterAnnotation(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to MKClusterAnnotation")
        }
    }
}
impl IMKClusterAnnotation for MKClusterAnnotation {}
pub trait IMKClusterAnnotation: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn initWithMemberAnnotations_(&self, memberAnnotations: NSArray) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithMemberAnnotations : memberAnnotations)
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
    unsafe fn memberAnnotations(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, memberAnnotations)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MKCompassButton(pub id);
impl std::ops::Deref for MKCompassButton {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MKCompassButton {}
impl MKCompassButton {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MKCompassButton").unwrap(), alloc) })
    }
}
impl INSView for MKCompassButton {}
impl PNSAnimatablePropertyContainer for MKCompassButton {}
impl PNSUserInterfaceItemIdentification for MKCompassButton {}
impl PNSDraggingDestination for MKCompassButton {}
impl PNSAppearanceCustomization for MKCompassButton {}
impl PNSAccessibilityElement for MKCompassButton {}
impl PNSAccessibility for MKCompassButton {}
impl std::convert::TryFrom<NSView> for MKCompassButton {
    type Error = &'static str;
    fn try_from(parent: NSView) -> Result<MKCompassButton, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MKCompassButton").unwrap()) };
        if is_kind_of {
            Ok(MKCompassButton(parent.0))
        } else {
            Err("This NSView cannot be downcasted to MKCompassButton")
        }
    }
}
impl INSResponder for MKCompassButton {}
impl PNSCoding for MKCompassButton {}
impl INSObject for MKCompassButton {}
impl PNSObject for MKCompassButton {}
impl IMKCompassButton for MKCompassButton {}
pub trait IMKCompassButton: Sized + std::ops::Deref {
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
    unsafe fn compassVisibility(&self) -> MKFeatureVisibility
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, compassVisibility)
    }
    unsafe fn setCompassVisibility_(&self, compassVisibility: MKFeatureVisibility)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCompassVisibility : compassVisibility)
    }
    unsafe fn compassButtonWithMapView_(mapView: MKMapView) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MKCompassButton").unwrap(), compassButtonWithMapView : mapView)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MKMapItemAnnotation(pub id);
impl std::ops::Deref for MKMapItemAnnotation {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MKMapItemAnnotation {}
impl MKMapItemAnnotation {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MKMapItemAnnotation").unwrap(), alloc) })
    }
}
impl PMKAnnotation for MKMapItemAnnotation {}
impl INSObject for MKMapItemAnnotation {}
impl PNSObject for MKMapItemAnnotation {}
impl std::convert::TryFrom<NSObject> for MKMapItemAnnotation {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<MKMapItemAnnotation, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MKMapItemAnnotation").unwrap()) };
        if is_kind_of {
            Ok(MKMapItemAnnotation(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to MKMapItemAnnotation")
        }
    }
}
impl IMKMapItemAnnotation for MKMapItemAnnotation {}
pub trait IMKMapItemAnnotation: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn initWithMapItem_(&self, mapItem: MKMapItem) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithMapItem : mapItem)
    }
    unsafe fn mapItem(&self) -> MKMapItem
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, mapItem)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MKMapItemAnnotation").unwrap(), new)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MKOverlayPathRenderer(pub id);
impl std::ops::Deref for MKOverlayPathRenderer {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MKOverlayPathRenderer {}
impl MKOverlayPathRenderer {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MKOverlayPathRenderer").unwrap(), alloc) })
    }
}
impl IMKOverlayRenderer for MKOverlayPathRenderer {}
impl From<MKOverlayPathRenderer> for MKOverlayRenderer {
    fn from(child: MKOverlayPathRenderer) -> MKOverlayRenderer {
        MKOverlayRenderer(child.0)
    }
}
impl std::convert::TryFrom<MKOverlayRenderer> for MKOverlayPathRenderer {
    type Error = &'static str;
    fn try_from(parent: MKOverlayRenderer) -> Result<MKOverlayPathRenderer, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MKOverlayPathRenderer").unwrap()) };
        if is_kind_of {
            Ok(MKOverlayPathRenderer(parent.0))
        } else {
            Err("This MKOverlayRenderer cannot be downcasted to MKOverlayPathRenderer")
        }
    }
}
impl INSObject for MKOverlayPathRenderer {}
impl PNSObject for MKOverlayPathRenderer {}
impl IMKOverlayPathRenderer for MKOverlayPathRenderer {}
pub trait IMKOverlayPathRenderer: Sized + std::ops::Deref {
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
    unsafe fn fillColor(&self) -> NSColor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, fillColor)
    }
    unsafe fn setFillColor_(&self, fillColor: NSColor)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setFillColor : fillColor)
    }
    unsafe fn strokeColor(&self) -> NSColor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, strokeColor)
    }
    unsafe fn setStrokeColor_(&self, strokeColor: NSColor)
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
pub struct MKMultiPolygonRenderer(pub id);
impl std::ops::Deref for MKMultiPolygonRenderer {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MKMultiPolygonRenderer {}
impl MKMultiPolygonRenderer {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MKMultiPolygonRenderer").unwrap(), alloc) })
    }
}
impl IMKOverlayPathRenderer for MKMultiPolygonRenderer {}
impl From<MKMultiPolygonRenderer> for MKOverlayPathRenderer {
    fn from(child: MKMultiPolygonRenderer) -> MKOverlayPathRenderer {
        MKOverlayPathRenderer(child.0)
    }
}
impl std::convert::TryFrom<MKOverlayPathRenderer> for MKMultiPolygonRenderer {
    type Error = &'static str;
    fn try_from(parent: MKOverlayPathRenderer) -> Result<MKMultiPolygonRenderer, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MKMultiPolygonRenderer").unwrap()) };
        if is_kind_of {
            Ok(MKMultiPolygonRenderer(parent.0))
        } else {
            Err("This MKOverlayPathRenderer cannot be downcasted to MKMultiPolygonRenderer")
        }
    }
}
impl IMKOverlayRenderer for MKMultiPolygonRenderer {}
impl INSObject for MKMultiPolygonRenderer {}
impl PNSObject for MKMultiPolygonRenderer {}
impl IMKMultiPolygonRenderer for MKMultiPolygonRenderer {}
pub trait IMKMultiPolygonRenderer: Sized + std::ops::Deref {
    unsafe fn initWithMultiPolygon_(&self, multiPolygon: MKMultiPolygon) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithMultiPolygon : multiPolygon)
    }
    unsafe fn multiPolygon(&self) -> MKMultiPolygon
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, multiPolygon)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MKMultiPolylineRenderer(pub id);
impl std::ops::Deref for MKMultiPolylineRenderer {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MKMultiPolylineRenderer {}
impl MKMultiPolylineRenderer {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MKMultiPolylineRenderer").unwrap(), alloc) })
    }
}
impl IMKOverlayPathRenderer for MKMultiPolylineRenderer {}
impl std::convert::TryFrom<MKOverlayPathRenderer> for MKMultiPolylineRenderer {
    type Error = &'static str;
    fn try_from(parent: MKOverlayPathRenderer) -> Result<MKMultiPolylineRenderer, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MKMultiPolylineRenderer").unwrap()) };
        if is_kind_of {
            Ok(MKMultiPolylineRenderer(parent.0))
        } else {
            Err("This MKOverlayPathRenderer cannot be downcasted to MKMultiPolylineRenderer")
        }
    }
}
impl IMKOverlayRenderer for MKMultiPolylineRenderer {}
impl INSObject for MKMultiPolylineRenderer {}
impl PNSObject for MKMultiPolylineRenderer {}
impl IMKMultiPolylineRenderer for MKMultiPolylineRenderer {}
pub trait IMKMultiPolylineRenderer: Sized + std::ops::Deref {
    unsafe fn initWithMultiPolyline_(&self, multiPolyline: MKMultiPolyline) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithMultiPolyline : multiPolyline)
    }
    unsafe fn multiPolyline(&self) -> MKMultiPolyline
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, multiPolyline)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MKPolylineRenderer(pub id);
impl std::ops::Deref for MKPolylineRenderer {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MKPolylineRenderer {}
impl MKPolylineRenderer {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MKPolylineRenderer").unwrap(), alloc) })
    }
}
impl IMKOverlayPathRenderer for MKPolylineRenderer {}
impl std::convert::TryFrom<MKOverlayPathRenderer> for MKPolylineRenderer {
    type Error = &'static str;
    fn try_from(parent: MKOverlayPathRenderer) -> Result<MKPolylineRenderer, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MKPolylineRenderer").unwrap()) };
        if is_kind_of {
            Ok(MKPolylineRenderer(parent.0))
        } else {
            Err("This MKOverlayPathRenderer cannot be downcasted to MKPolylineRenderer")
        }
    }
}
impl IMKOverlayRenderer for MKPolylineRenderer {}
impl INSObject for MKPolylineRenderer {}
impl PNSObject for MKPolylineRenderer {}
impl IMKPolylineRenderer for MKPolylineRenderer {}
pub trait IMKPolylineRenderer: Sized + std::ops::Deref {
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
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MKGradientPolylineRenderer(pub id);
impl std::ops::Deref for MKGradientPolylineRenderer {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MKGradientPolylineRenderer {}
impl MKGradientPolylineRenderer {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MKGradientPolylineRenderer").unwrap(), alloc) })
    }
}
impl IMKPolylineRenderer for MKGradientPolylineRenderer {}
impl From<MKGradientPolylineRenderer> for MKPolylineRenderer {
    fn from(child: MKGradientPolylineRenderer) -> MKPolylineRenderer {
        MKPolylineRenderer(child.0)
    }
}
impl std::convert::TryFrom<MKPolylineRenderer> for MKGradientPolylineRenderer {
    type Error = &'static str;
    fn try_from(parent: MKPolylineRenderer) -> Result<MKGradientPolylineRenderer, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MKGradientPolylineRenderer").unwrap()) };
        if is_kind_of {
            Ok(MKGradientPolylineRenderer(parent.0))
        } else {
            Err("This MKPolylineRenderer cannot be downcasted to MKGradientPolylineRenderer")
        }
    }
}
impl IMKOverlayPathRenderer for MKGradientPolylineRenderer {}
impl IMKOverlayRenderer for MKGradientPolylineRenderer {}
impl INSObject for MKGradientPolylineRenderer {}
impl PNSObject for MKGradientPolylineRenderer {}
impl IMKGradientPolylineRenderer for MKGradientPolylineRenderer {}
pub trait IMKGradientPolylineRenderer: Sized + std::ops::Deref {
    unsafe fn setColors_atLocations_(&self, colors: NSArray, locations: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setColors : colors, atLocations : locations)
    }
    unsafe fn locations(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, locations)
    }
    unsafe fn colors(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, colors)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MKPolygonRenderer(pub id);
impl std::ops::Deref for MKPolygonRenderer {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MKPolygonRenderer {}
impl MKPolygonRenderer {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MKPolygonRenderer").unwrap(), alloc) })
    }
}
impl IMKOverlayPathRenderer for MKPolygonRenderer {}
impl std::convert::TryFrom<MKOverlayPathRenderer> for MKPolygonRenderer {
    type Error = &'static str;
    fn try_from(parent: MKOverlayPathRenderer) -> Result<MKPolygonRenderer, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MKPolygonRenderer").unwrap()) };
        if is_kind_of {
            Ok(MKPolygonRenderer(parent.0))
        } else {
            Err("This MKOverlayPathRenderer cannot be downcasted to MKPolygonRenderer")
        }
    }
}
impl IMKOverlayRenderer for MKPolygonRenderer {}
impl INSObject for MKPolygonRenderer {}
impl PNSObject for MKPolygonRenderer {}
impl IMKPolygonRenderer for MKPolygonRenderer {}
pub trait IMKPolygonRenderer: Sized + std::ops::Deref {
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
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MKCircleRenderer(pub id);
impl std::ops::Deref for MKCircleRenderer {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MKCircleRenderer {}
impl MKCircleRenderer {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MKCircleRenderer").unwrap(), alloc) })
    }
}
impl IMKOverlayPathRenderer for MKCircleRenderer {}
impl std::convert::TryFrom<MKOverlayPathRenderer> for MKCircleRenderer {
    type Error = &'static str;
    fn try_from(parent: MKOverlayPathRenderer) -> Result<MKCircleRenderer, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MKCircleRenderer").unwrap()) };
        if is_kind_of {
            Ok(MKCircleRenderer(parent.0))
        } else {
            Err("This MKOverlayPathRenderer cannot be downcasted to MKCircleRenderer")
        }
    }
}
impl IMKOverlayRenderer for MKCircleRenderer {}
impl INSObject for MKCircleRenderer {}
impl PNSObject for MKCircleRenderer {}
impl IMKCircleRenderer for MKCircleRenderer {}
pub trait IMKCircleRenderer: Sized + std::ops::Deref {
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
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MKTileOverlay(pub id);
impl std::ops::Deref for MKTileOverlay {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MKTileOverlay {}
impl MKTileOverlay {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MKTileOverlay").unwrap(), alloc) })
    }
}
impl PMKOverlay for MKTileOverlay {}
impl INSObject for MKTileOverlay {}
impl PNSObject for MKTileOverlay {}
impl std::convert::TryFrom<NSObject> for MKTileOverlay {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<MKTileOverlay, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MKTileOverlay").unwrap()) };
        if is_kind_of {
            Ok(MKTileOverlay(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to MKTileOverlay")
        }
    }
}
impl IMKTileOverlay for MKTileOverlay {}
pub trait IMKTileOverlay: Sized + std::ops::Deref {
    unsafe fn initWithURLTemplate_(&self, URLTemplate: NSString) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithURLTemplate : URLTemplate)
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
    unsafe fn minimumZ(&self) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, minimumZ)
    }
    unsafe fn setMinimumZ_(&self, minimumZ: NSInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMinimumZ : minimumZ)
    }
    unsafe fn maximumZ(&self) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, maximumZ)
    }
    unsafe fn setMaximumZ_(&self, maximumZ: NSInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMaximumZ : maximumZ)
    }
    unsafe fn URLTemplate(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, URLTemplate)
    }
    unsafe fn canReplaceMapContent(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, canReplaceMapContent)
    }
    unsafe fn setCanReplaceMapContent_(&self, canReplaceMapContent: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCanReplaceMapContent : canReplaceMapContent)
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct MKTileOverlayPath {
    pub x: NSInteger,
    pub y: NSInteger,
    pub z: NSInteger,
    pub contentScaleFactor: CGFloat,
}
impl MKTileOverlay_CustomLoading for MKTileOverlay {}
pub trait MKTileOverlay_CustomLoading: Sized + std::ops::Deref {
    unsafe fn URLForTilePath_(&self, path: MKTileOverlayPath) -> NSURL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, URLForTilePath : path)
    }
    unsafe fn loadTileAtPath_result_(
        &self,
        path: MKTileOverlayPath,
        result: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, loadTileAtPath : path, result : result)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MKTileOverlayRenderer(pub id);
impl std::ops::Deref for MKTileOverlayRenderer {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MKTileOverlayRenderer {}
impl MKTileOverlayRenderer {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MKTileOverlayRenderer").unwrap(), alloc) })
    }
}
impl IMKOverlayRenderer for MKTileOverlayRenderer {}
impl std::convert::TryFrom<MKOverlayRenderer> for MKTileOverlayRenderer {
    type Error = &'static str;
    fn try_from(parent: MKOverlayRenderer) -> Result<MKTileOverlayRenderer, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MKTileOverlayRenderer").unwrap()) };
        if is_kind_of {
            Ok(MKTileOverlayRenderer(parent.0))
        } else {
            Err("This MKOverlayRenderer cannot be downcasted to MKTileOverlayRenderer")
        }
    }
}
impl INSObject for MKTileOverlayRenderer {}
impl PNSObject for MKTileOverlayRenderer {}
impl IMKTileOverlayRenderer for MKTileOverlayRenderer {}
pub trait IMKTileOverlayRenderer: Sized + std::ops::Deref {
    unsafe fn initWithTileOverlay_(&self, overlay: MKTileOverlay) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithTileOverlay : overlay)
    }
    unsafe fn reloadData(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, reloadData)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MKMapCameraZoomRange(pub id);
impl std::ops::Deref for MKMapCameraZoomRange {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MKMapCameraZoomRange {}
impl MKMapCameraZoomRange {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MKMapCameraZoomRange").unwrap(), alloc) })
    }
}
impl PNSSecureCoding for MKMapCameraZoomRange {}
impl PNSCopying for MKMapCameraZoomRange {}
impl INSObject for MKMapCameraZoomRange {}
impl PNSObject for MKMapCameraZoomRange {}
impl std::convert::TryFrom<NSObject> for MKMapCameraZoomRange {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<MKMapCameraZoomRange, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MKMapCameraZoomRange").unwrap()) };
        if is_kind_of {
            Ok(MKMapCameraZoomRange(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to MKMapCameraZoomRange")
        }
    }
}
impl IMKMapCameraZoomRange for MKMapCameraZoomRange {}
pub trait IMKMapCameraZoomRange: Sized + std::ops::Deref {
    unsafe fn initWithMinCenterCoordinateDistance_maxCenterCoordinateDistance_(
        &self,
        minDistance: CLLocationDistance,
        maxDistance: CLLocationDistance,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithMinCenterCoordinateDistance : minDistance, maxCenterCoordinateDistance : maxDistance)
    }
    unsafe fn initWithMinCenterCoordinateDistance_(
        &self,
        minDistance: CLLocationDistance,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithMinCenterCoordinateDistance : minDistance)
    }
    unsafe fn initWithMaxCenterCoordinateDistance_(
        &self,
        maxDistance: CLLocationDistance,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithMaxCenterCoordinateDistance : maxDistance)
    }
    unsafe fn minCenterCoordinateDistance(&self) -> CLLocationDistance
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, minCenterCoordinateDistance)
    }
    unsafe fn maxCenterCoordinateDistance(&self) -> CLLocationDistance
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, maxCenterCoordinateDistance)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MKMapCameraBoundary(pub id);
impl std::ops::Deref for MKMapCameraBoundary {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MKMapCameraBoundary {}
impl MKMapCameraBoundary {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MKMapCameraBoundary").unwrap(), alloc) })
    }
}
impl PNSSecureCoding for MKMapCameraBoundary {}
impl PNSCopying for MKMapCameraBoundary {}
impl INSObject for MKMapCameraBoundary {}
impl PNSObject for MKMapCameraBoundary {}
impl std::convert::TryFrom<NSObject> for MKMapCameraBoundary {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<MKMapCameraBoundary, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MKMapCameraBoundary").unwrap()) };
        if is_kind_of {
            Ok(MKMapCameraBoundary(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to MKMapCameraBoundary")
        }
    }
}
impl IMKMapCameraBoundary for MKMapCameraBoundary {}
pub trait IMKMapCameraBoundary: Sized + std::ops::Deref {
    unsafe fn initWithMapRect_(&self, mapRect: MKMapRect) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithMapRect : mapRect)
    }
    unsafe fn initWithCoordinateRegion_(&self, region: MKCoordinateRegion) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithCoordinateRegion : region)
    }
    unsafe fn initWithCoder_(&self, coder: NSCoder) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithCoder : coder)
    }
    unsafe fn mapRect(&self) -> MKMapRect
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, mapRect)
    }
    unsafe fn region(&self) -> MKCoordinateRegion
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, region)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MKPitchControl(pub id);
impl std::ops::Deref for MKPitchControl {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MKPitchControl {}
impl MKPitchControl {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MKPitchControl").unwrap(), alloc) })
    }
}
impl INSView for MKPitchControl {}
impl PNSAnimatablePropertyContainer for MKPitchControl {}
impl PNSUserInterfaceItemIdentification for MKPitchControl {}
impl PNSDraggingDestination for MKPitchControl {}
impl PNSAppearanceCustomization for MKPitchControl {}
impl PNSAccessibilityElement for MKPitchControl {}
impl PNSAccessibility for MKPitchControl {}
impl std::convert::TryFrom<NSView> for MKPitchControl {
    type Error = &'static str;
    fn try_from(parent: NSView) -> Result<MKPitchControl, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MKPitchControl").unwrap()) };
        if is_kind_of {
            Ok(MKPitchControl(parent.0))
        } else {
            Err("This NSView cannot be downcasted to MKPitchControl")
        }
    }
}
impl INSResponder for MKPitchControl {}
impl PNSCoding for MKPitchControl {}
impl INSObject for MKPitchControl {}
impl PNSObject for MKPitchControl {}
impl IMKPitchControl for MKPitchControl {}
pub trait IMKPitchControl: Sized + std::ops::Deref {
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
    unsafe fn pitchControlWithMapView_(mapView: MKMapView) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MKPitchControl").unwrap(), pitchControlWithMapView : mapView)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MKZoomControl(pub id);
impl std::ops::Deref for MKZoomControl {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MKZoomControl {}
impl MKZoomControl {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MKZoomControl").unwrap(), alloc) })
    }
}
impl INSView for MKZoomControl {}
impl PNSAnimatablePropertyContainer for MKZoomControl {}
impl PNSUserInterfaceItemIdentification for MKZoomControl {}
impl PNSDraggingDestination for MKZoomControl {}
impl PNSAppearanceCustomization for MKZoomControl {}
impl PNSAccessibilityElement for MKZoomControl {}
impl PNSAccessibility for MKZoomControl {}
impl std::convert::TryFrom<NSView> for MKZoomControl {
    type Error = &'static str;
    fn try_from(parent: NSView) -> Result<MKZoomControl, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MKZoomControl").unwrap()) };
        if is_kind_of {
            Ok(MKZoomControl(parent.0))
        } else {
            Err("This NSView cannot be downcasted to MKZoomControl")
        }
    }
}
impl INSResponder for MKZoomControl {}
impl PNSCoding for MKZoomControl {}
impl INSObject for MKZoomControl {}
impl PNSObject for MKZoomControl {}
impl IMKZoomControl for MKZoomControl {}
pub trait IMKZoomControl: Sized + std::ops::Deref {
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
    unsafe fn zoomControlWithMapView_(mapView: MKMapView) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MKZoomControl").unwrap(), zoomControlWithMapView : mapView)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MKHybridMapConfiguration(pub id);
impl std::ops::Deref for MKHybridMapConfiguration {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MKHybridMapConfiguration {}
impl MKHybridMapConfiguration {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MKHybridMapConfiguration").unwrap(), alloc) })
    }
}
impl IMKMapConfiguration for MKHybridMapConfiguration {}
impl PNSSecureCoding for MKHybridMapConfiguration {}
impl PNSCopying for MKHybridMapConfiguration {}
impl std::convert::TryFrom<MKMapConfiguration> for MKHybridMapConfiguration {
    type Error = &'static str;
    fn try_from(parent: MKMapConfiguration) -> Result<MKHybridMapConfiguration, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MKHybridMapConfiguration").unwrap()) };
        if is_kind_of {
            Ok(MKHybridMapConfiguration(parent.0))
        } else {
            Err("This MKMapConfiguration cannot be downcasted to MKHybridMapConfiguration")
        }
    }
}
impl INSObject for MKHybridMapConfiguration {}
impl PNSObject for MKHybridMapConfiguration {}
impl IMKHybridMapConfiguration for MKHybridMapConfiguration {}
pub trait IMKHybridMapConfiguration: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn initWithElevationStyle_(&self, elevationStyle: MKMapElevationStyle) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithElevationStyle : elevationStyle)
    }
    unsafe fn pointOfInterestFilter(&self) -> MKPointOfInterestFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, pointOfInterestFilter)
    }
    unsafe fn setPointOfInterestFilter_(&self, pointOfInterestFilter: MKPointOfInterestFilter)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPointOfInterestFilter : pointOfInterestFilter)
    }
    unsafe fn showsTraffic(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, showsTraffic)
    }
    unsafe fn setShowsTraffic_(&self, showsTraffic: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setShowsTraffic : showsTraffic)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MKImageryMapConfiguration(pub id);
impl std::ops::Deref for MKImageryMapConfiguration {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MKImageryMapConfiguration {}
impl MKImageryMapConfiguration {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MKImageryMapConfiguration").unwrap(), alloc) })
    }
}
impl IMKMapConfiguration for MKImageryMapConfiguration {}
impl PNSSecureCoding for MKImageryMapConfiguration {}
impl PNSCopying for MKImageryMapConfiguration {}
impl std::convert::TryFrom<MKMapConfiguration> for MKImageryMapConfiguration {
    type Error = &'static str;
    fn try_from(parent: MKMapConfiguration) -> Result<MKImageryMapConfiguration, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MKImageryMapConfiguration").unwrap()) };
        if is_kind_of {
            Ok(MKImageryMapConfiguration(parent.0))
        } else {
            Err("This MKMapConfiguration cannot be downcasted to MKImageryMapConfiguration")
        }
    }
}
impl INSObject for MKImageryMapConfiguration {}
impl PNSObject for MKImageryMapConfiguration {}
impl IMKImageryMapConfiguration for MKImageryMapConfiguration {}
pub trait IMKImageryMapConfiguration: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn initWithElevationStyle_(&self, elevationStyle: MKMapElevationStyle) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithElevationStyle : elevationStyle)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MKLookAroundScene(pub id);
impl std::ops::Deref for MKLookAroundScene {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MKLookAroundScene {}
impl MKLookAroundScene {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MKLookAroundScene").unwrap(), alloc) })
    }
}
impl PNSCopying for MKLookAroundScene {}
impl INSObject for MKLookAroundScene {}
impl PNSObject for MKLookAroundScene {}
impl std::convert::TryFrom<NSObject> for MKLookAroundScene {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<MKLookAroundScene, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MKLookAroundScene").unwrap()) };
        if is_kind_of {
            Ok(MKLookAroundScene(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to MKLookAroundScene")
        }
    }
}
impl IMKLookAroundScene for MKLookAroundScene {}
pub trait IMKLookAroundScene: Sized + std::ops::Deref {
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
        msg_send!(&*objc2::runtime::AnyClass::get(c"MKLookAroundScene").unwrap(), new)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MKLookAroundSceneRequest(pub id);
impl std::ops::Deref for MKLookAroundSceneRequest {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MKLookAroundSceneRequest {}
impl MKLookAroundSceneRequest {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MKLookAroundSceneRequest").unwrap(), alloc) })
    }
}
impl INSObject for MKLookAroundSceneRequest {}
impl PNSObject for MKLookAroundSceneRequest {}
impl std::convert::TryFrom<NSObject> for MKLookAroundSceneRequest {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<MKLookAroundSceneRequest, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MKLookAroundSceneRequest").unwrap()) };
        if is_kind_of {
            Ok(MKLookAroundSceneRequest(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to MKLookAroundSceneRequest")
        }
    }
}
impl IMKLookAroundSceneRequest for MKLookAroundSceneRequest {}
pub trait IMKLookAroundSceneRequest: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn initWithCoordinate_(&self, coordinate: CLLocationCoordinate2D) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithCoordinate : coordinate)
    }
    unsafe fn initWithMapItem_(&self, mapItem: MKMapItem) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithMapItem : mapItem)
    }
    unsafe fn getSceneWithCompletionHandler_(&self, completionHandler: *mut ::std::os::raw::c_void)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, getSceneWithCompletionHandler : completionHandler)
    }
    unsafe fn cancel(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, cancel)
    }
    unsafe fn coordinate(&self) -> CLLocationCoordinate2D
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, coordinate)
    }
    unsafe fn mapItem(&self) -> MKMapItem
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, mapItem)
    }
    unsafe fn isCancelled(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isCancelled)
    }
    unsafe fn isLoading(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isLoading)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MKLookAroundSceneRequest").unwrap(), new)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MKLookAroundSnapshot(pub id);
impl std::ops::Deref for MKLookAroundSnapshot {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MKLookAroundSnapshot {}
impl MKLookAroundSnapshot {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MKLookAroundSnapshot").unwrap(), alloc) })
    }
}
impl INSObject for MKLookAroundSnapshot {}
impl PNSObject for MKLookAroundSnapshot {}
impl std::convert::TryFrom<NSObject> for MKLookAroundSnapshot {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<MKLookAroundSnapshot, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MKLookAroundSnapshot").unwrap()) };
        if is_kind_of {
            Ok(MKLookAroundSnapshot(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to MKLookAroundSnapshot")
        }
    }
}
impl IMKLookAroundSnapshot for MKLookAroundSnapshot {}
pub trait IMKLookAroundSnapshot: Sized + std::ops::Deref {
    unsafe fn image(&self) -> NSImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, image)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MKLookAroundSnapshotOptions(pub id);
impl std::ops::Deref for MKLookAroundSnapshotOptions {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MKLookAroundSnapshotOptions {}
impl MKLookAroundSnapshotOptions {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MKLookAroundSnapshotOptions").unwrap(), alloc) })
    }
}
impl INSObject for MKLookAroundSnapshotOptions {}
impl PNSObject for MKLookAroundSnapshotOptions {}
impl std::convert::TryFrom<NSObject> for MKLookAroundSnapshotOptions {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<MKLookAroundSnapshotOptions, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MKLookAroundSnapshotOptions").unwrap()) };
        if is_kind_of {
            Ok(MKLookAroundSnapshotOptions(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to MKLookAroundSnapshotOptions")
        }
    }
}
impl IMKLookAroundSnapshotOptions for MKLookAroundSnapshotOptions {}
pub trait IMKLookAroundSnapshotOptions: Sized + std::ops::Deref {
    unsafe fn pointOfInterestFilter(&self) -> MKPointOfInterestFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, pointOfInterestFilter)
    }
    unsafe fn setPointOfInterestFilter_(&self, pointOfInterestFilter: MKPointOfInterestFilter)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPointOfInterestFilter : pointOfInterestFilter)
    }
    unsafe fn size(&self) -> CGSize
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, size)
    }
    unsafe fn setSize_(&self, size: CGSize)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSize : size)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MKLookAroundSnapshotter(pub id);
impl std::ops::Deref for MKLookAroundSnapshotter {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MKLookAroundSnapshotter {}
impl MKLookAroundSnapshotter {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MKLookAroundSnapshotter").unwrap(), alloc) })
    }
}
impl INSObject for MKLookAroundSnapshotter {}
impl PNSObject for MKLookAroundSnapshotter {}
impl std::convert::TryFrom<NSObject> for MKLookAroundSnapshotter {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<MKLookAroundSnapshotter, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MKLookAroundSnapshotter").unwrap()) };
        if is_kind_of {
            Ok(MKLookAroundSnapshotter(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to MKLookAroundSnapshotter")
        }
    }
}
impl IMKLookAroundSnapshotter for MKLookAroundSnapshotter {}
pub trait IMKLookAroundSnapshotter: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn initWithScene_options_(
        &self,
        scene: MKLookAroundScene,
        options: MKLookAroundSnapshotOptions,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithScene : scene, options : options)
    }
    unsafe fn getSnapshotWithCompletionHandler_(
        &self,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, getSnapshotWithCompletionHandler : completionHandler)
    }
    unsafe fn cancel(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, cancel)
    }
    unsafe fn isLoading(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isLoading)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"MKLookAroundSnapshotter").unwrap(), new)
    }
}
pub type MKLookAroundBadgePosition = NSInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MKLookAroundViewController(pub id);
impl std::ops::Deref for MKLookAroundViewController {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MKLookAroundViewController {}
impl MKLookAroundViewController {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MKLookAroundViewController").unwrap(), alloc) })
    }
}
impl PNSSecureCoding for MKLookAroundViewController {}
impl PNSCoding for MKLookAroundViewController {}
impl INSViewController for MKLookAroundViewController {}
impl PNSEditor for MKLookAroundViewController {}
impl PNSSeguePerforming for MKLookAroundViewController {}
impl PNSUserInterfaceItemIdentification for MKLookAroundViewController {}
impl std::convert::TryFrom<NSViewController> for MKLookAroundViewController {
    type Error = &'static str;
    fn try_from(parent: NSViewController) -> Result<MKLookAroundViewController, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MKLookAroundViewController").unwrap()) };
        if is_kind_of {
            Ok(MKLookAroundViewController(parent.0))
        } else {
            Err("This NSViewController cannot be downcasted to MKLookAroundViewController")
        }
    }
}
impl INSResponder for MKLookAroundViewController {}
impl INSObject for MKLookAroundViewController {}
impl PNSObject for MKLookAroundViewController {}
impl IMKLookAroundViewController for MKLookAroundViewController {}
pub trait IMKLookAroundViewController: Sized + std::ops::Deref {
    unsafe fn initWithScene_(&self, scene: MKLookAroundScene) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithScene : scene)
    }
    unsafe fn initWithNibName_bundle_(
        &self,
        nibNameOrNil: NSString,
        nibBundleOrNil: NSBundle,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithNibName : nibNameOrNil, bundle : nibBundleOrNil)
    }
    unsafe fn initWithCoder_(&self, coder: NSCoder) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithCoder : coder)
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
    unsafe fn scene(&self) -> MKLookAroundScene
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, scene)
    }
    unsafe fn setScene_(&self, scene: MKLookAroundScene)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setScene : scene)
    }
    unsafe fn isNavigationEnabled(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isNavigationEnabled)
    }
    unsafe fn setNavigationEnabled_(&self, navigationEnabled: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setNavigationEnabled : navigationEnabled)
    }
    unsafe fn showsRoadLabels(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, showsRoadLabels)
    }
    unsafe fn setShowsRoadLabels_(&self, showsRoadLabels: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setShowsRoadLabels : showsRoadLabels)
    }
    unsafe fn pointOfInterestFilter(&self) -> MKPointOfInterestFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, pointOfInterestFilter)
    }
    unsafe fn setPointOfInterestFilter_(&self, pointOfInterestFilter: MKPointOfInterestFilter)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPointOfInterestFilter : pointOfInterestFilter)
    }
    unsafe fn badgePosition(&self) -> MKLookAroundBadgePosition
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, badgePosition)
    }
    unsafe fn setBadgePosition_(&self, badgePosition: MKLookAroundBadgePosition)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setBadgePosition : badgePosition)
    }
}
pub trait PMKLookAroundViewControllerDelegate: Sized + std::ops::Deref {
    unsafe fn lookAroundViewControllerWillUpdateScene_(
        &self,
        viewController: MKLookAroundViewController,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, lookAroundViewControllerWillUpdateScene : viewController)
    }
    unsafe fn lookAroundViewControllerDidUpdateScene_(
        &self,
        viewController: MKLookAroundViewController,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, lookAroundViewControllerDidUpdateScene : viewController)
    }
    unsafe fn lookAroundViewControllerWillPresentFullScreen_(
        &self,
        viewController: MKLookAroundViewController,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, lookAroundViewControllerWillPresentFullScreen : viewController)
    }
    unsafe fn lookAroundViewControllerDidPresentFullScreen_(
        &self,
        viewController: MKLookAroundViewController,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, lookAroundViewControllerDidPresentFullScreen : viewController)
    }
    unsafe fn lookAroundViewControllerWillDismissFullScreen_(
        &self,
        viewController: MKLookAroundViewController,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, lookAroundViewControllerWillDismissFullScreen : viewController)
    }
    unsafe fn lookAroundViewControllerDidDismissFullScreen_(
        &self,
        viewController: MKLookAroundViewController,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, lookAroundViewControllerDidDismissFullScreen : viewController)
    }
}
pub trait PMKMapItemDetailViewControllerDelegate: Sized + std::ops::Deref {
    unsafe fn mapItemDetailViewControllerDidFinish_(
        &self,
        detailViewController: MKMapItemDetailViewController,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, mapItemDetailViewControllerDidFinish : detailViewController)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MKMapItemDetailViewController(pub id);
impl std::ops::Deref for MKMapItemDetailViewController {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MKMapItemDetailViewController {}
impl MKMapItemDetailViewController {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MKMapItemDetailViewController").unwrap(), alloc) })
    }
}
impl INSViewController for MKMapItemDetailViewController {}
impl PNSEditor for MKMapItemDetailViewController {}
impl PNSSeguePerforming for MKMapItemDetailViewController {}
impl PNSUserInterfaceItemIdentification for MKMapItemDetailViewController {}
impl std::convert::TryFrom<NSViewController> for MKMapItemDetailViewController {
    type Error = &'static str;
    fn try_from(parent: NSViewController) -> Result<MKMapItemDetailViewController, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MKMapItemDetailViewController").unwrap())
        };
        if is_kind_of {
            Ok(MKMapItemDetailViewController(parent.0))
        } else {
            Err("This NSViewController cannot be downcasted to MKMapItemDetailViewController")
        }
    }
}
impl INSResponder for MKMapItemDetailViewController {}
impl PNSCoding for MKMapItemDetailViewController {}
impl INSObject for MKMapItemDetailViewController {}
impl PNSObject for MKMapItemDetailViewController {}
impl IMKMapItemDetailViewController for MKMapItemDetailViewController {}
pub trait IMKMapItemDetailViewController: Sized + std::ops::Deref {
    unsafe fn initWithMapItem_displaysMap_(
        &self,
        mapItem: MKMapItem,
        displaysMap: BOOL,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithMapItem : mapItem, displaysMap : displaysMap)
    }
    unsafe fn initWithMapItem_(&self, mapItem: MKMapItem) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithMapItem : mapItem)
    }
    unsafe fn mapItem(&self) -> MKMapItem
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, mapItem)
    }
    unsafe fn setMapItem_(&self, mapItem: MKMapItem)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMapItem : mapItem)
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
}
unsafe extern "C" {
    pub static mut MKErrorDomain: NSString;
}
unsafe extern "C" {
    pub fn MKCoordinateRegionMakeWithDistance(
        centerCoordinate: CLLocationCoordinate2D,
        latitudinalMeters: CLLocationDistance,
        longitudinalMeters: CLLocationDistance,
    ) -> MKCoordinateRegion;
}
unsafe extern "C" {
    pub static MKMapSizeWorld: MKMapSize;
}
unsafe extern "C" {
    pub static MKMapRectWorld: MKMapRect;
}
unsafe extern "C" {
    pub fn MKMapPointForCoordinate(coordinate: CLLocationCoordinate2D) -> MKMapPoint;
}
unsafe extern "C" {
    pub fn MKCoordinateForMapPoint(mapPoint: MKMapPoint) -> CLLocationCoordinate2D;
}
unsafe extern "C" {
    pub fn MKMetersPerMapPointAtLatitude(latitude: CLLocationDegrees) -> CLLocationDistance;
}
unsafe extern "C" {
    pub fn MKMapPointsPerMeterAtLatitude(latitude: CLLocationDegrees) -> f64;
}
unsafe extern "C" {
    pub fn MKMetersBetweenMapPoints(a: MKMapPoint, b: MKMapPoint) -> CLLocationDistance;
}
unsafe extern "C" {
    pub static MKMapRectNull: MKMapRect;
}
unsafe extern "C" {
    pub fn MKMapRectUnion(rect1: MKMapRect, rect2: MKMapRect) -> MKMapRect;
}
unsafe extern "C" {
    pub fn MKMapRectIntersection(rect1: MKMapRect, rect2: MKMapRect) -> MKMapRect;
}
unsafe extern "C" {
    pub fn MKMapRectInset(rect: MKMapRect, dx: f64, dy: f64) -> MKMapRect;
}
unsafe extern "C" {
    pub fn MKMapRectOffset(rect: MKMapRect, dx: f64, dy: f64) -> MKMapRect;
}
unsafe extern "C" {
    pub fn MKMapRectDivide(
        rect: MKMapRect,
        slice: *mut MKMapRect,
        remainder: *mut MKMapRect,
        amount: f64,
        edge: CGRectEdge,
    );
}
unsafe extern "C" {
    pub fn MKMapRectContainsPoint(rect: MKMapRect, point: MKMapPoint) -> BOOL;
}
unsafe extern "C" {
    pub fn MKMapRectContainsRect(rect1: MKMapRect, rect2: MKMapRect) -> BOOL;
}
unsafe extern "C" {
    pub fn MKMapRectIntersectsRect(rect1: MKMapRect, rect2: MKMapRect) -> BOOL;
}
unsafe extern "C" {
    pub fn MKCoordinateRegionForMapRect(rect: MKMapRect) -> MKCoordinateRegion;
}
unsafe extern "C" {
    pub fn MKMapRectSpans180thMeridian(rect: MKMapRect) -> BOOL;
}
unsafe extern "C" {
    pub fn MKMapRectRemainder(rect: MKMapRect) -> MKMapRect;
}
unsafe extern "C" {
    pub static MKPointOfInterestCategoryAnimalService: MKPointOfInterestCategory;
}
unsafe extern "C" {
    pub static MKPointOfInterestCategoryAirport: MKPointOfInterestCategory;
}
unsafe extern "C" {
    pub static MKPointOfInterestCategoryAmusementPark: MKPointOfInterestCategory;
}
unsafe extern "C" {
    pub static MKPointOfInterestCategoryAquarium: MKPointOfInterestCategory;
}
unsafe extern "C" {
    pub static MKPointOfInterestCategoryATM: MKPointOfInterestCategory;
}
unsafe extern "C" {
    pub static MKPointOfInterestCategoryAutomotiveRepair: MKPointOfInterestCategory;
}
unsafe extern "C" {
    pub static MKPointOfInterestCategoryBakery: MKPointOfInterestCategory;
}
unsafe extern "C" {
    pub static MKPointOfInterestCategoryBank: MKPointOfInterestCategory;
}
unsafe extern "C" {
    pub static MKPointOfInterestCategoryBaseball: MKPointOfInterestCategory;
}
unsafe extern "C" {
    pub static MKPointOfInterestCategoryBasketball: MKPointOfInterestCategory;
}
unsafe extern "C" {
    pub static MKPointOfInterestCategoryBeach: MKPointOfInterestCategory;
}
unsafe extern "C" {
    pub static MKPointOfInterestCategoryBeauty: MKPointOfInterestCategory;
}
unsafe extern "C" {
    pub static MKPointOfInterestCategoryBowling: MKPointOfInterestCategory;
}
unsafe extern "C" {
    pub static MKPointOfInterestCategoryBrewery: MKPointOfInterestCategory;
}
unsafe extern "C" {
    pub static MKPointOfInterestCategoryCafe: MKPointOfInterestCategory;
}
unsafe extern "C" {
    pub static MKPointOfInterestCategoryCampground: MKPointOfInterestCategory;
}
unsafe extern "C" {
    pub static MKPointOfInterestCategoryCarRental: MKPointOfInterestCategory;
}
unsafe extern "C" {
    pub static MKPointOfInterestCategoryCastle: MKPointOfInterestCategory;
}
unsafe extern "C" {
    pub static MKPointOfInterestCategoryConventionCenter: MKPointOfInterestCategory;
}
unsafe extern "C" {
    pub static MKPointOfInterestCategoryDistillery: MKPointOfInterestCategory;
}
unsafe extern "C" {
    pub static MKPointOfInterestCategoryEVCharger: MKPointOfInterestCategory;
}
unsafe extern "C" {
    pub static MKPointOfInterestCategoryFairground: MKPointOfInterestCategory;
}
unsafe extern "C" {
    pub static MKPointOfInterestCategoryFireStation: MKPointOfInterestCategory;
}
unsafe extern "C" {
    pub static MKPointOfInterestCategoryFishing: MKPointOfInterestCategory;
}
unsafe extern "C" {
    pub static MKPointOfInterestCategoryFitnessCenter: MKPointOfInterestCategory;
}
unsafe extern "C" {
    pub static MKPointOfInterestCategoryFoodMarket: MKPointOfInterestCategory;
}
unsafe extern "C" {
    pub static MKPointOfInterestCategoryFortress: MKPointOfInterestCategory;
}
unsafe extern "C" {
    pub static MKPointOfInterestCategoryGasStation: MKPointOfInterestCategory;
}
unsafe extern "C" {
    pub static MKPointOfInterestCategoryGolf: MKPointOfInterestCategory;
}
unsafe extern "C" {
    pub static MKPointOfInterestCategoryGoKart: MKPointOfInterestCategory;
}
unsafe extern "C" {
    pub static MKPointOfInterestCategoryHiking: MKPointOfInterestCategory;
}
unsafe extern "C" {
    pub static MKPointOfInterestCategoryHospital: MKPointOfInterestCategory;
}
unsafe extern "C" {
    pub static MKPointOfInterestCategoryHotel: MKPointOfInterestCategory;
}
unsafe extern "C" {
    pub static MKPointOfInterestCategoryKayaking: MKPointOfInterestCategory;
}
unsafe extern "C" {
    pub static MKPointOfInterestCategoryLandmark: MKPointOfInterestCategory;
}
unsafe extern "C" {
    pub static MKPointOfInterestCategoryLaundry: MKPointOfInterestCategory;
}
unsafe extern "C" {
    pub static MKPointOfInterestCategoryLibrary: MKPointOfInterestCategory;
}
unsafe extern "C" {
    pub static MKPointOfInterestCategoryMailbox: MKPointOfInterestCategory;
}
unsafe extern "C" {
    pub static MKPointOfInterestCategoryMarina: MKPointOfInterestCategory;
}
unsafe extern "C" {
    pub static MKPointOfInterestCategoryMiniGolf: MKPointOfInterestCategory;
}
unsafe extern "C" {
    pub static MKPointOfInterestCategoryMovieTheater: MKPointOfInterestCategory;
}
unsafe extern "C" {
    pub static MKPointOfInterestCategoryMuseum: MKPointOfInterestCategory;
}
unsafe extern "C" {
    pub static MKPointOfInterestCategoryMusicVenue: MKPointOfInterestCategory;
}
unsafe extern "C" {
    pub static MKPointOfInterestCategoryNationalMonument: MKPointOfInterestCategory;
}
unsafe extern "C" {
    pub static MKPointOfInterestCategoryNationalPark: MKPointOfInterestCategory;
}
unsafe extern "C" {
    pub static MKPointOfInterestCategoryNightlife: MKPointOfInterestCategory;
}
unsafe extern "C" {
    pub static MKPointOfInterestCategoryPark: MKPointOfInterestCategory;
}
unsafe extern "C" {
    pub static MKPointOfInterestCategoryParking: MKPointOfInterestCategory;
}
unsafe extern "C" {
    pub static MKPointOfInterestCategoryPharmacy: MKPointOfInterestCategory;
}
unsafe extern "C" {
    pub static MKPointOfInterestCategoryPlanetarium: MKPointOfInterestCategory;
}
unsafe extern "C" {
    pub static MKPointOfInterestCategoryPolice: MKPointOfInterestCategory;
}
unsafe extern "C" {
    pub static MKPointOfInterestCategoryPostOffice: MKPointOfInterestCategory;
}
unsafe extern "C" {
    pub static MKPointOfInterestCategoryPublicTransport: MKPointOfInterestCategory;
}
unsafe extern "C" {
    pub static MKPointOfInterestCategoryRestaurant: MKPointOfInterestCategory;
}
unsafe extern "C" {
    pub static MKPointOfInterestCategoryRestroom: MKPointOfInterestCategory;
}
unsafe extern "C" {
    pub static MKPointOfInterestCategoryRockClimbing: MKPointOfInterestCategory;
}
unsafe extern "C" {
    pub static MKPointOfInterestCategoryRVPark: MKPointOfInterestCategory;
}
unsafe extern "C" {
    pub static MKPointOfInterestCategorySchool: MKPointOfInterestCategory;
}
unsafe extern "C" {
    pub static MKPointOfInterestCategorySkatePark: MKPointOfInterestCategory;
}
unsafe extern "C" {
    pub static MKPointOfInterestCategorySkating: MKPointOfInterestCategory;
}
unsafe extern "C" {
    pub static MKPointOfInterestCategorySkiing: MKPointOfInterestCategory;
}
unsafe extern "C" {
    pub static MKPointOfInterestCategorySoccer: MKPointOfInterestCategory;
}
unsafe extern "C" {
    pub static MKPointOfInterestCategorySpa: MKPointOfInterestCategory;
}
unsafe extern "C" {
    pub static MKPointOfInterestCategoryStadium: MKPointOfInterestCategory;
}
unsafe extern "C" {
    pub static MKPointOfInterestCategoryStore: MKPointOfInterestCategory;
}
unsafe extern "C" {
    pub static MKPointOfInterestCategorySurfing: MKPointOfInterestCategory;
}
unsafe extern "C" {
    pub static MKPointOfInterestCategorySwimming: MKPointOfInterestCategory;
}
unsafe extern "C" {
    pub static MKPointOfInterestCategoryTennis: MKPointOfInterestCategory;
}
unsafe extern "C" {
    pub static MKPointOfInterestCategoryTheater: MKPointOfInterestCategory;
}
unsafe extern "C" {
    pub static MKPointOfInterestCategoryUniversity: MKPointOfInterestCategory;
}
unsafe extern "C" {
    pub static MKPointOfInterestCategoryWinery: MKPointOfInterestCategory;
}
unsafe extern "C" {
    pub static MKPointOfInterestCategoryVolleyball: MKPointOfInterestCategory;
}
unsafe extern "C" {
    pub static MKPointOfInterestCategoryZoo: MKPointOfInterestCategory;
}
unsafe extern "C" {
    pub static MKLaunchOptionsDirectionsModeKey: NSString;
}
unsafe extern "C" {
    pub static MKLaunchOptionsMapTypeKey: NSString;
}
unsafe extern "C" {
    pub static MKLaunchOptionsShowsTrafficKey: NSString;
}
unsafe extern "C" {
    pub static MKLaunchOptionsDirectionsModeDefault: NSString;
}
unsafe extern "C" {
    pub static MKLaunchOptionsDirectionsModeDriving: NSString;
}
unsafe extern "C" {
    pub static MKLaunchOptionsDirectionsModeWalking: NSString;
}
unsafe extern "C" {
    pub static MKLaunchOptionsDirectionsModeTransit: NSString;
}
unsafe extern "C" {
    pub static MKLaunchOptionsDirectionsModeCycling: NSString;
}
unsafe extern "C" {
    pub static MKLaunchOptionsMapCenterKey: NSString;
}
unsafe extern "C" {
    pub static MKLaunchOptionsMapSpanKey: NSString;
}
unsafe extern "C" {
    pub static MKLaunchOptionsCameraKey: NSString;
}
unsafe extern "C" {
    pub static MKMapItemTypeIdentifier: NSString;
}
unsafe extern "C" {
    pub static MKPointsOfInterestRequestMaxRadius: CLLocationDistance;
}
unsafe extern "C" {
    pub static MKAnnotationCalloutInfoDidChangeNotification: NSString;
}
unsafe extern "C" {
    pub fn MKRoadWidthAtZoomScale(zoomScale: MKZoomScale) -> CGFloat;
}
unsafe extern "C" {
    pub static MKMapViewDefaultAnnotationViewReuseIdentifier: NSString;
}
unsafe extern "C" {
    pub static MKMapViewDefaultClusterAnnotationViewReuseIdentifier: NSString;
}
unsafe extern "C" {
    pub static MKMapCameraZoomDefault: CLLocationDistance;
}

unsafe impl objc2::encode::RefEncode for MKAddress {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MKAddress {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MKAddressRepresentations {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MKAddressRepresentations {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MKAddressFilter {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MKAddressFilter {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MKShape {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MKShape {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MKCoordinateSpan {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MKCoordinateSpan {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("MKCoordinateSpan", &[]);
}
unsafe impl objc2::encode::RefEncode for MKCoordinateRegion {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MKCoordinateRegion {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("MKCoordinateRegion", &[]);
}
unsafe impl objc2::encode::RefEncode for MKMapPoint {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MKMapPoint {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("MKMapPoint", &[]);
}
unsafe impl objc2::encode::RefEncode for MKMapSize {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MKMapSize {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("MKMapSize", &[]);
}
unsafe impl objc2::encode::RefEncode for MKMapRect {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MKMapRect {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("MKMapRect", &[]);
}
unsafe impl objc2::encode::RefEncode for MKCircle {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MKCircle {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MKDirections {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MKDirections {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MKMapItemIdentifier {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MKMapItemIdentifier {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MKPlacemark {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MKPlacemark {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MKMapItem {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MKMapItem {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MKDirectionsRequest {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MKDirectionsRequest {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MKDirectionsResponse {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MKDirectionsResponse {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MKRoute {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MKRoute {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MKRouteStep {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MKRouteStep {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MKETAResponse {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MKETAResponse {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MKDistanceFormatter {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MKDistanceFormatter {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MKGeocodingRequest {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MKGeocodingRequest {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MKPointAnnotation {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MKPointAnnotation {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MKMultiPoint {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MKMultiPoint {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MKPolyline {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MKPolyline {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MKMultiPolyline {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MKMultiPolyline {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MKPolygon {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MKPolygon {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MKMultiPolygon {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MKMultiPolygon {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MKGeoJSONDecoder {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MKGeoJSONDecoder {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MKGeoJSONFeature {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MKGeoJSONFeature {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MKGeodesicPolyline {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MKGeodesicPolyline {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MKPointOfInterestFilter {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MKPointOfInterestFilter {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MKLocalPointsOfInterestRequest {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MKLocalPointsOfInterestRequest {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MKLocalSearch {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MKLocalSearch {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MKLocalSearchRequest {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MKLocalSearchRequest {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MKLocalSearchCompleter {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MKLocalSearchCompleter {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MKLocalSearchCompletion {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MKLocalSearchCompletion {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MKLocalSearchResponse {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MKLocalSearchResponse {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MKMapCamera {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MKMapCamera {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MKMapConfiguration {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MKMapConfiguration {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MKMapFeatureAnnotation {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MKMapFeatureAnnotation {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MKMapItemRequest {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MKMapItemRequest {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MKMapSnapshot {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MKMapSnapshot {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MKAnnotationView {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MKAnnotationView {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MKOverlayRenderer {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MKOverlayRenderer {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MKSelectionAccessory {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MKSelectionAccessory {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MKMapItemDetailSelectionAccessoryPresentationStyle {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MKMapItemDetailSelectionAccessoryPresentationStyle {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MKMapView {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MKMapView {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MKMapSnapshotOptions {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MKMapSnapshotOptions {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MKMapSnapshotter {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MKMapSnapshotter {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MKReverseGeocodingRequest {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MKReverseGeocodingRequest {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MKStandardMapConfiguration {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MKStandardMapConfiguration {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MKMarkerAnnotationView {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MKMarkerAnnotationView {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MKPinAnnotationView {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MKPinAnnotationView {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MKUserLocation {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MKUserLocation {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MKUserLocationView {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MKUserLocationView {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MKClusterAnnotation {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MKClusterAnnotation {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MKCompassButton {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MKCompassButton {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MKMapItemAnnotation {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MKMapItemAnnotation {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MKOverlayPathRenderer {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MKOverlayPathRenderer {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MKMultiPolygonRenderer {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MKMultiPolygonRenderer {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MKMultiPolylineRenderer {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MKMultiPolylineRenderer {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MKPolylineRenderer {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MKPolylineRenderer {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MKGradientPolylineRenderer {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MKGradientPolylineRenderer {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MKPolygonRenderer {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MKPolygonRenderer {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MKCircleRenderer {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MKCircleRenderer {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MKTileOverlay {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MKTileOverlay {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MKTileOverlayPath {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MKTileOverlayPath {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("MKTileOverlayPath", &[]);
}
unsafe impl objc2::encode::RefEncode for MKTileOverlayRenderer {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MKTileOverlayRenderer {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MKMapCameraZoomRange {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MKMapCameraZoomRange {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MKMapCameraBoundary {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MKMapCameraBoundary {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MKPitchControl {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MKPitchControl {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MKZoomControl {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MKZoomControl {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MKHybridMapConfiguration {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MKHybridMapConfiguration {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MKImageryMapConfiguration {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MKImageryMapConfiguration {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MKLookAroundScene {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MKLookAroundScene {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MKLookAroundSceneRequest {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MKLookAroundSceneRequest {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MKLookAroundSnapshot {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MKLookAroundSnapshot {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MKLookAroundSnapshotOptions {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MKLookAroundSnapshotOptions {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MKLookAroundSnapshotter {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MKLookAroundSnapshotter {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MKLookAroundViewController {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MKLookAroundViewController {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MKMapItemDetailViewController {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MKMapItemDetailViewController {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
