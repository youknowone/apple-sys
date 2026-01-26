#[allow(unused_imports)]
use crate::objc::*;
#[allow(unused_imports)]
use crate::AddressBook::*;
#[allow(unused_imports)]
use crate::CoreFoundation::*;
#[allow(unused_imports)]
use crate::CoreGraphics::*;
#[allow(unused_imports)]
use crate::CoreML::*;
#[allow(unused_imports)]
use crate::CoreVideo::*;
#[allow(unused_imports)]
use crate::Foundation::*;
#[allow(unused_imports)]
use crate::IOSurface::*;
#[allow(unused_imports)]
use crate::ImageIO::*;
#[allow(unused_imports)]
use crate::Metal::*;
#[allow(unused_imports)]
use crate::OpenCL::*;
#[allow(unused_imports)]
use crate::OpenGL::*;

#[allow(unused_imports)]
use objc2::msg_send;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CIVector(pub id);
impl std::ops::Deref for CIVector {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CIVector {}
impl CIVector {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CIVector").unwrap(), alloc) })
    }
}
impl PNSCopying for CIVector {}
impl PNSSecureCoding for CIVector {}
impl INSObject for CIVector {}
impl PNSObject for CIVector {}
impl std::convert::TryFrom<NSObject> for CIVector {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<CIVector, Self::Error> {
        let is_kind_of: bool = unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CIVector").unwrap()) };
        if is_kind_of {
            Ok(CIVector(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to CIVector")
        }
    }
}
impl ICIVector for CIVector {}
pub trait ICIVector: Sized + std::ops::Deref {
    unsafe fn initWithValues_count_(&self, values: *const CGFloat, count: usize) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithValues : values, count : count)
    }
    unsafe fn initWithX_(&self, x: CGFloat) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithX : x)
    }
    unsafe fn initWithX_Y_(&self, x: CGFloat, y: CGFloat) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithX : x, Y : y)
    }
    unsafe fn initWithX_Y_Z_(&self, x: CGFloat, y: CGFloat, z: CGFloat) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithX : x, Y : y, Z : z)
    }
    unsafe fn initWithX_Y_Z_W_(
        &self,
        x: CGFloat,
        y: CGFloat,
        z: CGFloat,
        w: CGFloat,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithX : x, Y : y, Z : z, W : w)
    }
    unsafe fn initWithCGPoint_(&self, p: CGPoint) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithCGPoint : p)
    }
    unsafe fn initWithCGRect_(&self, r: CGRect) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithCGRect : r)
    }
    unsafe fn initWithCGAffineTransform_(&self, t: CGAffineTransform) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithCGAffineTransform : t)
    }
    unsafe fn initWithString_(&self, representation: NSString) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithString : representation)
    }
    unsafe fn valueAtIndex_(&self, index: usize) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, valueAtIndex : index)
    }
    unsafe fn count(&self) -> usize
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, count)
    }
    unsafe fn X(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, X)
    }
    unsafe fn Y(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, Y)
    }
    unsafe fn Z(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, Z)
    }
    unsafe fn W(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, W)
    }
    unsafe fn CGPointValue(&self) -> CGPoint
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, CGPointValue)
    }
    unsafe fn CGRectValue(&self) -> CGRect
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, CGRectValue)
    }
    unsafe fn CGAffineTransformValue(&self) -> CGAffineTransform
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, CGAffineTransformValue)
    }
    unsafe fn stringRepresentation(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, stringRepresentation)
    }
    unsafe fn vectorWithValues_count_(values: *const CGFloat, count: usize) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIVector").unwrap(), vectorWithValues : values, count : count)
    }
    unsafe fn vectorWithX_(x: CGFloat) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIVector").unwrap(), vectorWithX : x)
    }
    unsafe fn vectorWithX_Y_(x: CGFloat, y: CGFloat) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIVector").unwrap(), vectorWithX : x, Y : y)
    }
    unsafe fn vectorWithX_Y_Z_(x: CGFloat, y: CGFloat, z: CGFloat) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIVector").unwrap(), vectorWithX : x, Y : y, Z : z)
    }
    unsafe fn vectorWithX_Y_Z_W_(x: CGFloat, y: CGFloat, z: CGFloat, w: CGFloat) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIVector").unwrap(), vectorWithX : x, Y : y, Z : z, W : w)
    }
    unsafe fn vectorWithCGPoint_(p: CGPoint) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIVector").unwrap(), vectorWithCGPoint : p)
    }
    unsafe fn vectorWithCGRect_(r: CGRect) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIVector").unwrap(), vectorWithCGRect : r)
    }
    unsafe fn vectorWithCGAffineTransform_(t: CGAffineTransform) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIVector").unwrap(), vectorWithCGAffineTransform : t)
    }
    unsafe fn vectorWithString_(representation: NSString) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIVector").unwrap(), vectorWithString : representation)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CIColor(pub id);
impl std::ops::Deref for CIColor {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CIColor {}
impl CIColor {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CIColor").unwrap(), alloc) })
    }
}
impl PNSSecureCoding for CIColor {}
impl PNSCopying for CIColor {}
impl INSObject for CIColor {}
impl PNSObject for CIColor {}
impl std::convert::TryFrom<NSObject> for CIColor {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<CIColor, Self::Error> {
        let is_kind_of: bool = unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CIColor").unwrap()) };
        if is_kind_of {
            Ok(CIColor(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to CIColor")
        }
    }
}
impl ICIColor for CIColor {}
pub trait ICIColor: Sized + std::ops::Deref {
    unsafe fn initWithCGColor_(&self, color: CGColorRef) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithCGColor : color)
    }
    unsafe fn initWithRed_green_blue_alpha_(
        &self,
        red: CGFloat,
        green: CGFloat,
        blue: CGFloat,
        alpha: CGFloat,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithRed : red, green : green, blue : blue, alpha : alpha)
    }
    unsafe fn initWithRed_green_blue_(
        &self,
        red: CGFloat,
        green: CGFloat,
        blue: CGFloat,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithRed : red, green : green, blue : blue)
    }
    unsafe fn initWithRed_green_blue_alpha_colorSpace_(
        &self,
        red: CGFloat,
        green: CGFloat,
        blue: CGFloat,
        alpha: CGFloat,
        colorSpace: CGColorSpaceRef,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithRed : red, green : green, blue : blue, alpha : alpha, colorSpace : colorSpace)
    }
    unsafe fn initWithRed_green_blue_colorSpace_(
        &self,
        red: CGFloat,
        green: CGFloat,
        blue: CGFloat,
        colorSpace: CGColorSpaceRef,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithRed : red, green : green, blue : blue, colorSpace : colorSpace)
    }
    unsafe fn numberOfComponents(&self) -> usize
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, numberOfComponents)
    }
    unsafe fn components(&self) -> *const CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, components)
    }
    unsafe fn alpha(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, alpha)
    }
    unsafe fn colorSpace(&self) -> CGColorSpaceRef
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, colorSpace)
    }
    unsafe fn red(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, red)
    }
    unsafe fn green(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, green)
    }
    unsafe fn blue(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, blue)
    }
    unsafe fn stringRepresentation(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, stringRepresentation)
    }
    unsafe fn colorWithCGColor_(color: CGColorRef) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIColor").unwrap(), colorWithCGColor : color)
    }
    unsafe fn colorWithRed_green_blue_alpha_(
        red: CGFloat,
        green: CGFloat,
        blue: CGFloat,
        alpha: CGFloat,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIColor").unwrap(), colorWithRed : red, green : green, blue : blue, alpha : alpha)
    }
    unsafe fn colorWithRed_green_blue_(red: CGFloat, green: CGFloat, blue: CGFloat) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIColor").unwrap(), colorWithRed : red, green : green, blue : blue)
    }
    unsafe fn colorWithRed_green_blue_alpha_colorSpace_(
        red: CGFloat,
        green: CGFloat,
        blue: CGFloat,
        alpha: CGFloat,
        colorSpace: CGColorSpaceRef,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIColor").unwrap(), colorWithRed : red, green : green, blue : blue, alpha : alpha, colorSpace : colorSpace)
    }
    unsafe fn colorWithRed_green_blue_colorSpace_(
        red: CGFloat,
        green: CGFloat,
        blue: CGFloat,
        colorSpace: CGColorSpaceRef,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIColor").unwrap(), colorWithRed : red, green : green, blue : blue, colorSpace : colorSpace)
    }
    unsafe fn colorWithString_(representation: NSString) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIColor").unwrap(), colorWithString : representation)
    }
    unsafe fn blackColor() -> CIColor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIColor").unwrap(), blackColor)
    }
    unsafe fn whiteColor() -> CIColor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIColor").unwrap(), whiteColor)
    }
    unsafe fn grayColor() -> CIColor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIColor").unwrap(), grayColor)
    }
    unsafe fn redColor() -> CIColor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIColor").unwrap(), redColor)
    }
    unsafe fn greenColor() -> CIColor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIColor").unwrap(), greenColor)
    }
    unsafe fn blueColor() -> CIColor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIColor").unwrap(), blueColor)
    }
    unsafe fn cyanColor() -> CIColor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIColor").unwrap(), cyanColor)
    }
    unsafe fn magentaColor() -> CIColor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIColor").unwrap(), magentaColor)
    }
    unsafe fn yellowColor() -> CIColor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIColor").unwrap(), yellowColor)
    }
    unsafe fn clearColor() -> CIColor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIColor").unwrap(), clearColor)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CIImage(pub id);
impl std::ops::Deref for CIImage {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CIImage {}
impl CIImage {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CIImage").unwrap(), alloc) })
    }
}
impl PNSSecureCoding for CIImage {}
impl PNSCopying for CIImage {}
impl INSObject for CIImage {}
impl PNSObject for CIImage {}
impl std::convert::TryFrom<NSObject> for CIImage {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<CIImage, Self::Error> {
        let is_kind_of: bool = unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CIImage").unwrap()) };
        if is_kind_of {
            Ok(CIImage(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to CIImage")
        }
    }
}
impl ICIImage for CIImage {}
pub trait ICIImage: Sized + std::ops::Deref {
    unsafe fn initWithCGImage_(&self, image: CGImageRef) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithCGImage : image)
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
    unsafe fn initWithCGImageSource_index_options_(
        &self,
        source: CGImageSourceRef,
        index: usize,
        dict: NSDictionary,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithCGImageSource : source, index : index, options : dict)
    }
    unsafe fn initWithCGLayer_(&self, layer: CGLayerRef) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithCGLayer : layer)
    }
    unsafe fn initWithCGLayer_options_(
        &self,
        layer: CGLayerRef,
        options: NSDictionary,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithCGLayer : layer, options : options)
    }
    unsafe fn initWithData_(&self, data: NSData) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithData : data)
    }
    unsafe fn initWithData_options_(&self, data: NSData, options: NSDictionary) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithData : data, options : options)
    }
    unsafe fn initWithBitmapData_bytesPerRow_size_format_colorSpace_(
        &self,
        data: NSData,
        bytesPerRow: usize,
        size: CGSize,
        format: CIFormat,
        colorSpace: CGColorSpaceRef,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithBitmapData : data, bytesPerRow : bytesPerRow, size : size, format : format, colorSpace : colorSpace)
    }
    unsafe fn initWithTexture_size_flipped_colorSpace_(
        &self,
        name: ::std::os::raw::c_uint,
        size: CGSize,
        flipped: BOOL,
        colorSpace: CGColorSpaceRef,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithTexture : name, size : size, flipped : flipped, colorSpace : colorSpace)
    }
    unsafe fn initWithTexture_size_flipped_options_(
        &self,
        name: ::std::os::raw::c_uint,
        size: CGSize,
        flipped: BOOL,
        options: NSDictionary,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithTexture : name, size : size, flipped : flipped, options : options)
    }
    unsafe fn initWithMTLTexture_options_(
        &self,
        texture: *mut u64,
        options: NSDictionary,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithMTLTexture : texture, options : options)
    }
    unsafe fn initWithContentsOfURL_(&self, url: NSURL) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithContentsOfURL : url)
    }
    unsafe fn initWithContentsOfURL_options_(
        &self,
        url: NSURL,
        options: NSDictionary,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithContentsOfURL : url, options : options)
    }
    unsafe fn initWithIOSurface_(&self, surface: IOSurfaceRef) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithIOSurface : surface)
    }
    unsafe fn initWithIOSurface_options_(
        &self,
        surface: IOSurfaceRef,
        options: NSDictionary,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithIOSurface : surface, options : options)
    }
    unsafe fn initWithIOSurface_plane_format_options_(
        &self,
        surface: IOSurfaceRef,
        plane: usize,
        format: CIFormat,
        options: NSDictionary,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithIOSurface : surface, plane : plane, format : format, options : options)
    }
    unsafe fn initWithCVImageBuffer_(&self, imageBuffer: CVImageBufferRef) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithCVImageBuffer : imageBuffer)
    }
    unsafe fn initWithCVImageBuffer_options_(
        &self,
        imageBuffer: CVImageBufferRef,
        options: NSDictionary,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithCVImageBuffer : imageBuffer, options : options)
    }
    unsafe fn initWithCVPixelBuffer_(&self, pixelBuffer: CVPixelBufferRef) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithCVPixelBuffer : pixelBuffer)
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
    unsafe fn initWithColor_(&self, color: CIColor) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithColor : color)
    }
    unsafe fn imageByApplyingTransform_(&self, matrix: CGAffineTransform) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, imageByApplyingTransform : matrix)
    }
    unsafe fn imageByApplyingTransform_highQualityDownsample_(
        &self,
        matrix: CGAffineTransform,
        highQualityDownsample: BOOL,
    ) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, imageByApplyingTransform : matrix, highQualityDownsample : highQualityDownsample)
    }
    unsafe fn imageByApplyingOrientation_(&self, orientation: ::std::os::raw::c_int) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, imageByApplyingOrientation : orientation)
    }
    unsafe fn imageTransformForOrientation_(
        &self,
        orientation: ::std::os::raw::c_int,
    ) -> CGAffineTransform
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, imageTransformForOrientation : orientation)
    }
    unsafe fn imageByApplyingCGOrientation_(
        &self,
        orientation: CGImagePropertyOrientation,
    ) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, imageByApplyingCGOrientation : orientation)
    }
    unsafe fn imageTransformForCGOrientation_(
        &self,
        orientation: CGImagePropertyOrientation,
    ) -> CGAffineTransform
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, imageTransformForCGOrientation : orientation)
    }
    unsafe fn imageByCompositingOverImage_(&self, dest: CIImage) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, imageByCompositingOverImage : dest)
    }
    unsafe fn imageByCroppingToRect_(&self, rect: CGRect) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, imageByCroppingToRect : rect)
    }
    unsafe fn imageByClampingToExtent(&self) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, imageByClampingToExtent)
    }
    unsafe fn imageByClampingToRect_(&self, rect: CGRect) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, imageByClampingToRect : rect)
    }
    unsafe fn imageByApplyingFilter_withInputParameters_(
        &self,
        filterName: NSString,
        params: NSDictionary,
    ) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, imageByApplyingFilter : filterName, withInputParameters : params)
    }
    unsafe fn imageByApplyingFilter_(&self, filterName: NSString) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, imageByApplyingFilter : filterName)
    }
    unsafe fn imageByColorMatchingColorSpaceToWorkingSpace_(
        &self,
        colorSpace: CGColorSpaceRef,
    ) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, imageByColorMatchingColorSpaceToWorkingSpace : colorSpace)
    }
    unsafe fn imageByColorMatchingWorkingSpaceToColorSpace_(
        &self,
        colorSpace: CGColorSpaceRef,
    ) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, imageByColorMatchingWorkingSpaceToColorSpace : colorSpace)
    }
    unsafe fn imageByPremultiplyingAlpha(&self) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, imageByPremultiplyingAlpha)
    }
    unsafe fn imageByUnpremultiplyingAlpha(&self) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, imageByUnpremultiplyingAlpha)
    }
    unsafe fn imageBySettingAlphaOneInExtent_(&self, extent: CGRect) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, imageBySettingAlphaOneInExtent : extent)
    }
    unsafe fn imageByApplyingGaussianBlurWithSigma_(&self, sigma: f64) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, imageByApplyingGaussianBlurWithSigma : sigma)
    }
    unsafe fn imageBySettingProperties_(&self, properties: NSDictionary) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, imageBySettingProperties : properties)
    }
    unsafe fn imageBySamplingLinear(&self) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, imageBySamplingLinear)
    }
    unsafe fn imageBySamplingNearest(&self) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, imageBySamplingNearest)
    }
    unsafe fn imageByInsertingIntermediate(&self) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, imageByInsertingIntermediate)
    }
    unsafe fn imageByInsertingIntermediate_(&self, cache: BOOL) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, imageByInsertingIntermediate : cache)
    }
    unsafe fn imageByInsertingTiledIntermediate(&self) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, imageByInsertingTiledIntermediate)
    }
    unsafe fn imageByApplyingGainMap_(&self, gainmap: CIImage) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, imageByApplyingGainMap : gainmap)
    }
    unsafe fn imageByApplyingGainMap_headroom_(&self, gainmap: CIImage, headroom: f32) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, imageByApplyingGainMap : gainmap, headroom : headroom)
    }
    unsafe fn imageBySettingContentHeadroom_(&self, headroom: f32) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, imageBySettingContentHeadroom : headroom)
    }
    unsafe fn imageBySettingContentAverageLightLevel_(&self, average: f32) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, imageBySettingContentAverageLightLevel : average)
    }
    unsafe fn regionOfInterestForImage_inRect_(&self, image: CIImage, rect: CGRect) -> CGRect
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, regionOfInterestForImage : image, inRect : rect)
    }
    unsafe fn extent(&self) -> CGRect
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, extent)
    }
    unsafe fn isOpaque(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isOpaque)
    }
    unsafe fn properties(&self) -> NSDictionary
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, properties)
    }
    unsafe fn definition(&self) -> CIFilterShape
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, definition)
    }
    unsafe fn url(&self) -> NSURL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, url)
    }
    unsafe fn colorSpace(&self) -> CGColorSpaceRef
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, colorSpace)
    }
    unsafe fn contentHeadroom(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, contentHeadroom)
    }
    unsafe fn contentAverageLightLevel(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, contentAverageLightLevel)
    }
    unsafe fn pixelBuffer(&self) -> CVPixelBufferRef
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, pixelBuffer)
    }
    unsafe fn CGImage(&self) -> CGImageRef
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, CGImage)
    }
    unsafe fn metalTexture(&self) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, metalTexture)
    }
    unsafe fn imageWithCGImage_(image: CGImageRef) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIImage").unwrap(), imageWithCGImage : image)
    }
    unsafe fn imageWithCGImage_options_(image: CGImageRef, options: NSDictionary) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIImage").unwrap(), imageWithCGImage : image, options : options)
    }
    unsafe fn imageWithCGImageSource_index_options_(
        source: CGImageSourceRef,
        index: usize,
        dict: NSDictionary,
    ) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIImage").unwrap(), imageWithCGImageSource : source, index : index, options : dict)
    }
    unsafe fn imageWithCGLayer_(layer: CGLayerRef) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIImage").unwrap(), imageWithCGLayer : layer)
    }
    unsafe fn imageWithCGLayer_options_(layer: CGLayerRef, options: NSDictionary) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIImage").unwrap(), imageWithCGLayer : layer, options : options)
    }
    unsafe fn imageWithBitmapData_bytesPerRow_size_format_colorSpace_(
        data: NSData,
        bytesPerRow: usize,
        size: CGSize,
        format: CIFormat,
        colorSpace: CGColorSpaceRef,
    ) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIImage").unwrap(), imageWithBitmapData : data, bytesPerRow : bytesPerRow, size : size, format : format, colorSpace : colorSpace)
    }
    unsafe fn imageWithTexture_size_flipped_colorSpace_(
        name: ::std::os::raw::c_uint,
        size: CGSize,
        flipped: BOOL,
        colorSpace: CGColorSpaceRef,
    ) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIImage").unwrap(), imageWithTexture : name, size : size, flipped : flipped, colorSpace : colorSpace)
    }
    unsafe fn imageWithTexture_size_flipped_options_(
        name: ::std::os::raw::c_uint,
        size: CGSize,
        flipped: BOOL,
        options: NSDictionary,
    ) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIImage").unwrap(), imageWithTexture : name, size : size, flipped : flipped, options : options)
    }
    unsafe fn imageWithMTLTexture_options_(texture: *mut u64, options: NSDictionary) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIImage").unwrap(), imageWithMTLTexture : texture, options : options)
    }
    unsafe fn imageWithContentsOfURL_(url: NSURL) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIImage").unwrap(), imageWithContentsOfURL : url)
    }
    unsafe fn imageWithContentsOfURL_options_(url: NSURL, options: NSDictionary) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIImage").unwrap(), imageWithContentsOfURL : url, options : options)
    }
    unsafe fn imageWithData_(data: NSData) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIImage").unwrap(), imageWithData : data)
    }
    unsafe fn imageWithData_options_(data: NSData, options: NSDictionary) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIImage").unwrap(), imageWithData : data, options : options)
    }
    unsafe fn imageWithCVImageBuffer_(imageBuffer: CVImageBufferRef) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIImage").unwrap(), imageWithCVImageBuffer : imageBuffer)
    }
    unsafe fn imageWithCVImageBuffer_options_(
        imageBuffer: CVImageBufferRef,
        options: NSDictionary,
    ) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIImage").unwrap(), imageWithCVImageBuffer : imageBuffer, options : options)
    }
    unsafe fn imageWithCVPixelBuffer_(pixelBuffer: CVPixelBufferRef) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIImage").unwrap(), imageWithCVPixelBuffer : pixelBuffer)
    }
    unsafe fn imageWithCVPixelBuffer_options_(
        pixelBuffer: CVPixelBufferRef,
        options: NSDictionary,
    ) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIImage").unwrap(), imageWithCVPixelBuffer : pixelBuffer, options : options)
    }
    unsafe fn imageWithIOSurface_(surface: IOSurfaceRef) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIImage").unwrap(), imageWithIOSurface : surface)
    }
    unsafe fn imageWithIOSurface_options_(surface: IOSurfaceRef, options: NSDictionary) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIImage").unwrap(), imageWithIOSurface : surface, options : options)
    }
    unsafe fn imageWithColor_(color: CIColor) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIImage").unwrap(), imageWithColor : color)
    }
    unsafe fn emptyImage() -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIImage").unwrap(), emptyImage)
    }
    unsafe fn blackImage() -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIImage").unwrap(), blackImage)
    }
    unsafe fn whiteImage() -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIImage").unwrap(), whiteImage)
    }
    unsafe fn grayImage() -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIImage").unwrap(), grayImage)
    }
    unsafe fn redImage() -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIImage").unwrap(), redImage)
    }
    unsafe fn greenImage() -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIImage").unwrap(), greenImage)
    }
    unsafe fn blueImage() -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIImage").unwrap(), blueImage)
    }
    unsafe fn cyanImage() -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIImage").unwrap(), cyanImage)
    }
    unsafe fn magentaImage() -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIImage").unwrap(), magentaImage)
    }
    unsafe fn yellowImage() -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIImage").unwrap(), yellowImage)
    }
    unsafe fn clearImage() -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIImage").unwrap(), clearImage)
    }
}
pub type CIImageOption = NSString;
impl CIImage_AutoAdjustment for CIImage {}
pub type CIImageAutoAdjustmentOption = NSString;
impl CIImage_LabConversion for CIImage {}
impl CIImage_AVDepthData for CIImage {}
impl CIImage_AVPortraitEffectsMatte for CIImage {}
impl CIImage_AVSemanticSegmentationMatte for CIImage {}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CIContext(pub id);
impl std::ops::Deref for CIContext {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CIContext {}
impl CIContext {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CIContext").unwrap(), alloc) })
    }
}
impl INSObject for CIContext {}
impl PNSObject for CIContext {}
impl std::convert::TryFrom<NSObject> for CIContext {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<CIContext, Self::Error> {
        let is_kind_of: bool = unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CIContext").unwrap()) };
        if is_kind_of {
            Ok(CIContext(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to CIContext")
        }
    }
}
impl ICIContext for CIContext {}
pub trait ICIContext: Sized + std::ops::Deref {
    unsafe fn initWithOptions_(&self, options: NSDictionary) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithOptions : options)
    }
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn drawImage_atPoint_fromRect_(&self, image: CIImage, atPoint: CGPoint, fromRect: CGRect)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, drawImage : image, atPoint : atPoint, fromRect : fromRect)
    }
    unsafe fn drawImage_inRect_fromRect_(&self, image: CIImage, inRect: CGRect, fromRect: CGRect)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, drawImage : image, inRect : inRect, fromRect : fromRect)
    }
    unsafe fn createCGLayerWithSize_info_(&self, size: CGSize, info: CFDictionaryRef) -> CGLayerRef
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, createCGLayerWithSize : size, info : info)
    }
    unsafe fn render_toBitmap_rowBytes_bounds_format_colorSpace_(
        &self,
        image: CIImage,
        data: *mut ::std::os::raw::c_void,
        rowBytes: isize,
        bounds: CGRect,
        format: CIFormat,
        colorSpace: CGColorSpaceRef,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, render : image, toBitmap : data, rowBytes : rowBytes, bounds : bounds, format : format, colorSpace : colorSpace)
    }
    unsafe fn render_toIOSurface_bounds_colorSpace_(
        &self,
        image: CIImage,
        surface: IOSurfaceRef,
        bounds: CGRect,
        colorSpace: CGColorSpaceRef,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, render : image, toIOSurface : surface, bounds : bounds, colorSpace : colorSpace)
    }
    unsafe fn render_toCVPixelBuffer_(&self, image: CIImage, buffer: CVPixelBufferRef)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, render : image, toCVPixelBuffer : buffer)
    }
    unsafe fn render_toCVPixelBuffer_bounds_colorSpace_(
        &self,
        image: CIImage,
        buffer: CVPixelBufferRef,
        bounds: CGRect,
        colorSpace: CGColorSpaceRef,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, render : image, toCVPixelBuffer : buffer, bounds : bounds, colorSpace : colorSpace)
    }
    unsafe fn render_toMTLTexture_commandBuffer_bounds_colorSpace_(
        &self,
        image: CIImage,
        texture: *mut u64,
        commandBuffer: *mut u64,
        bounds: CGRect,
        colorSpace: CGColorSpaceRef,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, render : image, toMTLTexture : texture, commandBuffer : commandBuffer, bounds : bounds, colorSpace : colorSpace)
    }
    unsafe fn reclaimResources(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, reclaimResources)
    }
    unsafe fn clearCaches(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, clearCaches)
    }
    unsafe fn inputImageMaximumSize(&self) -> CGSize
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inputImageMaximumSize)
    }
    unsafe fn outputImageMaximumSize(&self) -> CGSize
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, outputImageMaximumSize)
    }
    unsafe fn workingColorSpace(&self) -> CGColorSpaceRef
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, workingColorSpace)
    }
    unsafe fn workingFormat(&self) -> CIFormat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, workingFormat)
    }
    unsafe fn contextWithCGLContext_pixelFormat_colorSpace_options_(
        cglctx: CGLContextObj,
        pixelFormat: CGLPixelFormatObj,
        colorSpace: CGColorSpaceRef,
        options: NSDictionary,
    ) -> CIContext
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIContext").unwrap(), contextWithCGLContext : cglctx, pixelFormat : pixelFormat, colorSpace : colorSpace, options : options)
    }
    unsafe fn contextWithCGLContext_pixelFormat_options_(
        cglctx: CGLContextObj,
        pixelFormat: CGLPixelFormatObj,
        options: NSDictionary,
    ) -> CIContext
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIContext").unwrap(), contextWithCGLContext : cglctx, pixelFormat : pixelFormat, options : options)
    }
    unsafe fn contextWithCGContext_options_(cgctx: CGContextRef, options: NSDictionary) -> CIContext
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIContext").unwrap(), contextWithCGContext : cgctx, options : options)
    }
    unsafe fn contextWithOptions_(options: NSDictionary) -> CIContext
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIContext").unwrap(), contextWithOptions : options)
    }
    unsafe fn context() -> CIContext
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIContext").unwrap(), context)
    }
    unsafe fn contextWithMTLDevice_(device: *mut u64) -> CIContext
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIContext").unwrap(), contextWithMTLDevice : device)
    }
    unsafe fn contextWithMTLDevice_options_(device: *mut u64, options: NSDictionary) -> CIContext
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIContext").unwrap(), contextWithMTLDevice : device, options : options)
    }
    unsafe fn contextWithMTLCommandQueue_(commandQueue: *mut u64) -> CIContext
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIContext").unwrap(), contextWithMTLCommandQueue : commandQueue)
    }
    unsafe fn contextWithMTLCommandQueue_options_(
        commandQueue: *mut u64,
        options: NSDictionary,
    ) -> CIContext
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIContext").unwrap(), contextWithMTLCommandQueue : commandQueue, options : options)
    }
}
pub type CIContextOption = NSString;
impl CIContext_createCGImage for CIContext {}
impl CIContext_CalculateHDRStats for CIContext {}
impl CIContext_OfflineGPUSupport for CIContext {}
pub type CIImageRepresentationOption = NSString;
impl CIContext_ImageRepresentation for CIContext {}
impl CIContext_CIDepthBlurEffect for CIContext {}
pub trait PCIFilterConstructor: Sized + std::ops::Deref {
    unsafe fn filterWithName_(&self, name: NSString) -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, filterWithName : name)
    }
}
pub type CIDynamicRangeOption = NSString;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CIFilter(pub id);
impl std::ops::Deref for CIFilter {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CIFilter {}
impl CIFilter {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), alloc) })
    }
}
impl PNSSecureCoding for CIFilter {}
impl PNSCopying for CIFilter {}
impl INSObject for CIFilter {}
impl PNSObject for CIFilter {}
impl std::convert::TryFrom<NSObject> for CIFilter {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<CIFilter, Self::Error> {
        let is_kind_of: bool = unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CIFilter").unwrap()) };
        if is_kind_of {
            Ok(CIFilter(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to CIFilter")
        }
    }
}
impl ICIFilter for CIFilter {}
pub trait ICIFilter: Sized + std::ops::Deref {
    unsafe fn name(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, name)
    }
    unsafe fn setName_(&self, aString: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setName : aString)
    }
    unsafe fn setDefaults(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, setDefaults)
    }
    unsafe fn apply_arguments_options_(
        &self,
        k: CIKernel,
        args: NSArray,
        dict: NSDictionary,
    ) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, apply : k, arguments : args, options : dict)
    }
    unsafe fn apply_(&self, k: CIKernel) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, apply : k)
    }
    unsafe fn outputImage(&self) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, outputImage)
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
    unsafe fn inputKeys(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inputKeys)
    }
    unsafe fn outputKeys(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, outputKeys)
    }
    unsafe fn attributes(&self) -> NSDictionary
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, attributes)
    }
}
pub trait PCIFilter: Sized + std::ops::Deref {
    unsafe fn outputImage(&self) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, outputImage)
    }
    unsafe fn customAttributes() -> NSDictionary
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilter").unwrap(), customAttributes)
    }
}
impl CIFilter_CIFilterRegistry for CIFilter {}
impl CIFilter_CIFilterXMPSerialization for CIFilter {}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CIKernel(pub id);
impl std::ops::Deref for CIKernel {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CIKernel {}
impl CIKernel {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CIKernel").unwrap(), alloc) })
    }
}
impl INSObject for CIKernel {}
impl PNSObject for CIKernel {}
impl std::convert::TryFrom<NSObject> for CIKernel {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<CIKernel, Self::Error> {
        let is_kind_of: bool = unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CIKernel").unwrap()) };
        if is_kind_of {
            Ok(CIKernel(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to CIKernel")
        }
    }
}
impl ICIKernel for CIKernel {}
pub trait ICIKernel: Sized + std::ops::Deref {
    unsafe fn setROISelector_(&self, method: objc2::runtime::Sel)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setROISelector : method)
    }
    unsafe fn applyWithExtent_roiCallback_arguments_(
        &self,
        extent: CGRect,
        callback: CIKernelROICallback,
        args: NSArray,
    ) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, applyWithExtent : extent, roiCallback : callback, arguments : args)
    }
    unsafe fn name(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, name)
    }
    unsafe fn kernelsWithString_(string: NSString) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIKernel").unwrap(), kernelsWithString : string)
    }
    unsafe fn kernelsWithMetalString_error_(source: NSString, error: *mut NSError) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIKernel").unwrap(), kernelsWithMetalString : source, error : error)
    }
    unsafe fn kernelWithString_(string: NSString) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIKernel").unwrap(), kernelWithString : string)
    }
    unsafe fn kernelWithFunctionName_fromMetalLibraryData_error_(
        name: NSString,
        data: NSData,
        error: *mut NSError,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIKernel").unwrap(), kernelWithFunctionName : name, fromMetalLibraryData : data, error : error)
    }
    unsafe fn kernelWithFunctionName_fromMetalLibraryData_outputPixelFormat_error_(
        name: NSString,
        data: NSData,
        format: CIFormat,
        error: *mut NSError,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIKernel").unwrap(), kernelWithFunctionName : name, fromMetalLibraryData : data, outputPixelFormat : format, error : error)
    }
    unsafe fn kernelNamesFromMetalLibraryData_(data: NSData) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIKernel").unwrap(), kernelNamesFromMetalLibraryData : data)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CIColorKernel(pub id);
impl std::ops::Deref for CIColorKernel {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CIColorKernel {}
impl CIColorKernel {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CIColorKernel").unwrap(), alloc) })
    }
}
impl ICIKernel for CIColorKernel {}
impl From<CIColorKernel> for CIKernel {
    fn from(child: CIColorKernel) -> CIKernel {
        CIKernel(child.0)
    }
}
impl std::convert::TryFrom<CIKernel> for CIColorKernel {
    type Error = &'static str;
    fn try_from(parent: CIKernel) -> Result<CIColorKernel, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CIColorKernel").unwrap()) };
        if is_kind_of {
            Ok(CIColorKernel(parent.0))
        } else {
            Err("This CIKernel cannot be downcasted to CIColorKernel")
        }
    }
}
impl INSObject for CIColorKernel {}
impl PNSObject for CIColorKernel {}
impl ICIColorKernel for CIColorKernel {}
pub trait ICIColorKernel: Sized + std::ops::Deref {
    unsafe fn applyWithExtent_arguments_(&self, extent: CGRect, args: NSArray) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, applyWithExtent : extent, arguments : args)
    }
    unsafe fn kernelWithString_(string: NSString) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIColorKernel").unwrap(), kernelWithString : string)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CIWarpKernel(pub id);
impl std::ops::Deref for CIWarpKernel {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CIWarpKernel {}
impl CIWarpKernel {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CIWarpKernel").unwrap(), alloc) })
    }
}
impl ICIKernel for CIWarpKernel {}
impl std::convert::TryFrom<CIKernel> for CIWarpKernel {
    type Error = &'static str;
    fn try_from(parent: CIKernel) -> Result<CIWarpKernel, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CIWarpKernel").unwrap()) };
        if is_kind_of {
            Ok(CIWarpKernel(parent.0))
        } else {
            Err("This CIKernel cannot be downcasted to CIWarpKernel")
        }
    }
}
impl INSObject for CIWarpKernel {}
impl PNSObject for CIWarpKernel {}
impl ICIWarpKernel for CIWarpKernel {}
pub trait ICIWarpKernel: Sized + std::ops::Deref {
    unsafe fn applyWithExtent_roiCallback_inputImage_arguments_(
        &self,
        extent: CGRect,
        callback: CIKernelROICallback,
        image: CIImage,
        args: NSArray,
    ) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, applyWithExtent : extent, roiCallback : callback, inputImage : image, arguments : args)
    }
    unsafe fn kernelWithString_(string: NSString) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIWarpKernel").unwrap(), kernelWithString : string)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CIBlendKernel(pub id);
impl std::ops::Deref for CIBlendKernel {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CIBlendKernel {}
impl CIBlendKernel {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CIBlendKernel").unwrap(), alloc) })
    }
}
impl ICIColorKernel for CIBlendKernel {}
impl From<CIBlendKernel> for CIColorKernel {
    fn from(child: CIBlendKernel) -> CIColorKernel {
        CIColorKernel(child.0)
    }
}
impl std::convert::TryFrom<CIColorKernel> for CIBlendKernel {
    type Error = &'static str;
    fn try_from(parent: CIColorKernel) -> Result<CIBlendKernel, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CIBlendKernel").unwrap()) };
        if is_kind_of {
            Ok(CIBlendKernel(parent.0))
        } else {
            Err("This CIColorKernel cannot be downcasted to CIBlendKernel")
        }
    }
}
impl ICIKernel for CIBlendKernel {}
impl INSObject for CIBlendKernel {}
impl PNSObject for CIBlendKernel {}
impl ICIBlendKernel for CIBlendKernel {}
pub trait ICIBlendKernel: Sized + std::ops::Deref {
    unsafe fn applyWithForeground_background_(
        &self,
        foreground: CIImage,
        background: CIImage,
    ) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, applyWithForeground : foreground, background : background)
    }
    unsafe fn applyWithForeground_background_colorSpace_(
        &self,
        foreground: CIImage,
        background: CIImage,
        colorSpace: CGColorSpaceRef,
    ) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, applyWithForeground : foreground, background : background, colorSpace : colorSpace)
    }
    unsafe fn kernelWithString_(string: NSString) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIBlendKernel").unwrap(), kernelWithString : string)
    }
}
impl CIBlendKernel_BuiltIn for CIBlendKernel {}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CIDetector(pub id);
impl std::ops::Deref for CIDetector {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CIDetector {}
impl CIDetector {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CIDetector").unwrap(), alloc) })
    }
}
impl INSObject for CIDetector {}
impl PNSObject for CIDetector {}
impl std::convert::TryFrom<NSObject> for CIDetector {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<CIDetector, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CIDetector").unwrap()) };
        if is_kind_of {
            Ok(CIDetector(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to CIDetector")
        }
    }
}
impl ICIDetector for CIDetector {}
pub trait ICIDetector: Sized + std::ops::Deref {
    unsafe fn featuresInImage_(&self, image: CIImage) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, featuresInImage : image)
    }
    unsafe fn featuresInImage_options_(&self, image: CIImage, options: NSDictionary) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, featuresInImage : image, options : options)
    }
    unsafe fn detectorOfType_context_options_(
        type_: NSString,
        context: CIContext,
        options: NSDictionary,
    ) -> CIDetector
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIDetector").unwrap(), detectorOfType : type_, context : context, options : options)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CIFeature(pub id);
impl std::ops::Deref for CIFeature {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CIFeature {}
impl CIFeature {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CIFeature").unwrap(), alloc) })
    }
}
impl INSObject for CIFeature {}
impl PNSObject for CIFeature {}
impl std::convert::TryFrom<NSObject> for CIFeature {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<CIFeature, Self::Error> {
        let is_kind_of: bool = unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CIFeature").unwrap()) };
        if is_kind_of {
            Ok(CIFeature(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to CIFeature")
        }
    }
}
impl ICIFeature for CIFeature {}
pub trait ICIFeature: Sized + std::ops::Deref {
    unsafe fn type_(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, type)
    }
    unsafe fn bounds(&self) -> CGRect
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, bounds)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CIFaceFeature(pub id);
impl std::ops::Deref for CIFaceFeature {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CIFaceFeature {}
impl CIFaceFeature {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CIFaceFeature").unwrap(), alloc) })
    }
}
impl ICIFeature for CIFaceFeature {}
impl From<CIFaceFeature> for CIFeature {
    fn from(child: CIFaceFeature) -> CIFeature {
        CIFeature(child.0)
    }
}
impl std::convert::TryFrom<CIFeature> for CIFaceFeature {
    type Error = &'static str;
    fn try_from(parent: CIFeature) -> Result<CIFaceFeature, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CIFaceFeature").unwrap()) };
        if is_kind_of {
            Ok(CIFaceFeature(parent.0))
        } else {
            Err("This CIFeature cannot be downcasted to CIFaceFeature")
        }
    }
}
impl INSObject for CIFaceFeature {}
impl PNSObject for CIFaceFeature {}
impl ICIFaceFeature for CIFaceFeature {}
pub trait ICIFaceFeature: Sized + std::ops::Deref {
    unsafe fn bounds(&self) -> CGRect
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, bounds)
    }
    unsafe fn hasLeftEyePosition(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, hasLeftEyePosition)
    }
    unsafe fn leftEyePosition(&self) -> CGPoint
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, leftEyePosition)
    }
    unsafe fn hasRightEyePosition(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, hasRightEyePosition)
    }
    unsafe fn rightEyePosition(&self) -> CGPoint
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, rightEyePosition)
    }
    unsafe fn hasMouthPosition(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, hasMouthPosition)
    }
    unsafe fn mouthPosition(&self) -> CGPoint
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, mouthPosition)
    }
    unsafe fn hasTrackingID(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, hasTrackingID)
    }
    unsafe fn trackingID(&self) -> ::std::os::raw::c_int
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, trackingID)
    }
    unsafe fn hasTrackingFrameCount(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, hasTrackingFrameCount)
    }
    unsafe fn trackingFrameCount(&self) -> ::std::os::raw::c_int
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, trackingFrameCount)
    }
    unsafe fn hasFaceAngle(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, hasFaceAngle)
    }
    unsafe fn faceAngle(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, faceAngle)
    }
    unsafe fn hasSmile(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, hasSmile)
    }
    unsafe fn leftEyeClosed(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, leftEyeClosed)
    }
    unsafe fn rightEyeClosed(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, rightEyeClosed)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CIRectangleFeature(pub id);
impl std::ops::Deref for CIRectangleFeature {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CIRectangleFeature {}
impl CIRectangleFeature {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CIRectangleFeature").unwrap(), alloc) })
    }
}
impl ICIFeature for CIRectangleFeature {}
impl std::convert::TryFrom<CIFeature> for CIRectangleFeature {
    type Error = &'static str;
    fn try_from(parent: CIFeature) -> Result<CIRectangleFeature, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CIRectangleFeature").unwrap()) };
        if is_kind_of {
            Ok(CIRectangleFeature(parent.0))
        } else {
            Err("This CIFeature cannot be downcasted to CIRectangleFeature")
        }
    }
}
impl INSObject for CIRectangleFeature {}
impl PNSObject for CIRectangleFeature {}
impl ICIRectangleFeature for CIRectangleFeature {}
pub trait ICIRectangleFeature: Sized + std::ops::Deref {
    unsafe fn bounds(&self) -> CGRect
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, bounds)
    }
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
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CIQRCodeFeature(pub id);
impl std::ops::Deref for CIQRCodeFeature {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CIQRCodeFeature {}
impl CIQRCodeFeature {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CIQRCodeFeature").unwrap(), alloc) })
    }
}
impl PNSSecureCoding for CIQRCodeFeature {}
impl PNSCopying for CIQRCodeFeature {}
impl ICIFeature for CIQRCodeFeature {}
impl std::convert::TryFrom<CIFeature> for CIQRCodeFeature {
    type Error = &'static str;
    fn try_from(parent: CIFeature) -> Result<CIQRCodeFeature, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CIQRCodeFeature").unwrap()) };
        if is_kind_of {
            Ok(CIQRCodeFeature(parent.0))
        } else {
            Err("This CIFeature cannot be downcasted to CIQRCodeFeature")
        }
    }
}
impl INSObject for CIQRCodeFeature {}
impl PNSObject for CIQRCodeFeature {}
impl ICIQRCodeFeature for CIQRCodeFeature {}
pub trait ICIQRCodeFeature: Sized + std::ops::Deref {
    unsafe fn bounds(&self) -> CGRect
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, bounds)
    }
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
    unsafe fn messageString(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, messageString)
    }
    unsafe fn symbolDescriptor(&self) -> CIQRCodeDescriptor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, symbolDescriptor)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CITextFeature(pub id);
impl std::ops::Deref for CITextFeature {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CITextFeature {}
impl CITextFeature {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CITextFeature").unwrap(), alloc) })
    }
}
impl ICIFeature for CITextFeature {}
impl std::convert::TryFrom<CIFeature> for CITextFeature {
    type Error = &'static str;
    fn try_from(parent: CIFeature) -> Result<CITextFeature, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CITextFeature").unwrap()) };
        if is_kind_of {
            Ok(CITextFeature(parent.0))
        } else {
            Err("This CIFeature cannot be downcasted to CITextFeature")
        }
    }
}
impl INSObject for CITextFeature {}
impl PNSObject for CITextFeature {}
impl ICITextFeature for CITextFeature {}
pub trait ICITextFeature: Sized + std::ops::Deref {
    unsafe fn bounds(&self) -> CGRect
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, bounds)
    }
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
    unsafe fn subFeatures(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, subFeatures)
    }
}
impl CIImage_CIImageProvider for CIImage {}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CIImageProcessorKernel(pub id);
impl std::ops::Deref for CIImageProcessorKernel {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CIImageProcessorKernel {}
impl CIImageProcessorKernel {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CIImageProcessorKernel").unwrap(), alloc) })
    }
}
impl INSObject for CIImageProcessorKernel {}
impl PNSObject for CIImageProcessorKernel {}
impl std::convert::TryFrom<NSObject> for CIImageProcessorKernel {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<CIImageProcessorKernel, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CIImageProcessorKernel").unwrap()) };
        if is_kind_of {
            Ok(CIImageProcessorKernel(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to CIImageProcessorKernel")
        }
    }
}
impl ICIImageProcessorKernel for CIImageProcessorKernel {}
pub trait ICIImageProcessorKernel: Sized + std::ops::Deref {
    unsafe fn processWithInputs_arguments_output_error_(
        inputs: NSArray,
        arguments: NSDictionary,
        output: *mut u64,
        error: *mut NSError,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIImageProcessorKernel").unwrap(), processWithInputs : inputs, arguments : arguments, output : output, error : error)
    }
    unsafe fn roiForInput_arguments_outputRect_(
        inputIndex: ::std::os::raw::c_int,
        arguments: NSDictionary,
        outputRect: CGRect,
    ) -> CGRect
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIImageProcessorKernel").unwrap(), roiForInput : inputIndex, arguments : arguments, outputRect : outputRect)
    }
    unsafe fn roiTileArrayForInput_arguments_outputRect_(
        inputIndex: ::std::os::raw::c_int,
        arguments: NSDictionary,
        outputRect: CGRect,
    ) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIImageProcessorKernel").unwrap(), roiTileArrayForInput : inputIndex, arguments : arguments, outputRect : outputRect)
    }
    unsafe fn formatForInputAtIndex_(inputIndex: ::std::os::raw::c_int) -> CIFormat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIImageProcessorKernel").unwrap(), formatForInputAtIndex : inputIndex)
    }
    unsafe fn applyWithExtent_inputs_arguments_error_(
        extent: CGRect,
        inputs: NSArray,
        arguments: NSDictionary,
        error: *mut NSError,
    ) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIImageProcessorKernel").unwrap(), applyWithExtent : extent, inputs : inputs, arguments : arguments, error : error)
    }
    unsafe fn outputFormat() -> CIFormat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIImageProcessorKernel").unwrap(), outputFormat)
    }
    unsafe fn outputIsOpaque() -> bool
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIImageProcessorKernel").unwrap(), outputIsOpaque)
    }
    unsafe fn synchronizeInputs() -> bool
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIImageProcessorKernel").unwrap(), synchronizeInputs)
    }
}
impl CIImageProcessorKernel_MultipleOutputSupport for CIImageProcessorKernel {}
pub trait PCIImageProcessorInput: Sized + std::ops::Deref {
    unsafe fn region(&self) -> CGRect
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, region)
    }
    unsafe fn bytesPerRow(&self) -> usize
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, bytesPerRow)
    }
    unsafe fn format(&self) -> CIFormat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, format)
    }
    unsafe fn baseAddress(&self) -> *const ::std::os::raw::c_void
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, baseAddress)
    }
    unsafe fn surface(&self) -> IOSurfaceRef
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, surface)
    }
    unsafe fn pixelBuffer(&self) -> CVPixelBufferRef
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, pixelBuffer)
    }
    unsafe fn metalTexture(&self) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, metalTexture)
    }
    unsafe fn digest(&self) -> u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, digest)
    }
    unsafe fn roiTileIndex(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, roiTileIndex)
    }
    unsafe fn roiTileCount(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, roiTileCount)
    }
}
pub trait PCIImageProcessorOutput: Sized + std::ops::Deref {
    unsafe fn region(&self) -> CGRect
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, region)
    }
    unsafe fn bytesPerRow(&self) -> usize
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, bytesPerRow)
    }
    unsafe fn format(&self) -> CIFormat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, format)
    }
    unsafe fn baseAddress(&self) -> *mut ::std::os::raw::c_void
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, baseAddress)
    }
    unsafe fn surface(&self) -> IOSurfaceRef
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, surface)
    }
    unsafe fn pixelBuffer(&self) -> CVPixelBufferRef
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, pixelBuffer)
    }
    unsafe fn metalTexture(&self) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, metalTexture)
    }
    unsafe fn metalCommandBuffer(&self) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, metalCommandBuffer)
    }
    unsafe fn digest(&self) -> u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, digest)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CIImageAccumulator(pub id);
impl std::ops::Deref for CIImageAccumulator {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CIImageAccumulator {}
impl CIImageAccumulator {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CIImageAccumulator").unwrap(), alloc) })
    }
}
impl INSObject for CIImageAccumulator {}
impl PNSObject for CIImageAccumulator {}
impl std::convert::TryFrom<NSObject> for CIImageAccumulator {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<CIImageAccumulator, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CIImageAccumulator").unwrap()) };
        if is_kind_of {
            Ok(CIImageAccumulator(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to CIImageAccumulator")
        }
    }
}
impl ICIImageAccumulator for CIImageAccumulator {}
pub trait ICIImageAccumulator: Sized + std::ops::Deref {
    unsafe fn initWithExtent_format_(&self, extent: CGRect, format: CIFormat) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithExtent : extent, format : format)
    }
    unsafe fn initWithExtent_format_colorSpace_(
        &self,
        extent: CGRect,
        format: CIFormat,
        colorSpace: CGColorSpaceRef,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithExtent : extent, format : format, colorSpace : colorSpace)
    }
    unsafe fn image(&self) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, image)
    }
    unsafe fn setImage_(&self, image: CIImage)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setImage : image)
    }
    unsafe fn setImage_dirtyRect_(&self, image: CIImage, dirtyRect: CGRect)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setImage : image, dirtyRect : dirtyRect)
    }
    unsafe fn clear(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, clear)
    }
    unsafe fn extent(&self) -> CGRect
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, extent)
    }
    unsafe fn format(&self) -> CIFormat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, format)
    }
    unsafe fn imageAccumulatorWithExtent_format_(extent: CGRect, format: CIFormat) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIImageAccumulator").unwrap(), imageAccumulatorWithExtent : extent, format : format)
    }
    unsafe fn imageAccumulatorWithExtent_format_colorSpace_(
        extent: CGRect,
        format: CIFormat,
        colorSpace: CGColorSpaceRef,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIImageAccumulator").unwrap(), imageAccumulatorWithExtent : extent, format : format, colorSpace : colorSpace)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CIFilterShape(pub id);
impl std::ops::Deref for CIFilterShape {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CIFilterShape {}
impl CIFilterShape {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilterShape").unwrap(), alloc) })
    }
}
impl PNSCopying for CIFilterShape {}
impl INSObject for CIFilterShape {}
impl PNSObject for CIFilterShape {}
impl std::convert::TryFrom<NSObject> for CIFilterShape {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<CIFilterShape, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CIFilterShape").unwrap()) };
        if is_kind_of {
            Ok(CIFilterShape(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to CIFilterShape")
        }
    }
}
impl ICIFilterShape for CIFilterShape {}
pub trait ICIFilterShape: Sized + std::ops::Deref {
    unsafe fn initWithRect_(&self, r: CGRect) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithRect : r)
    }
    unsafe fn transformBy_interior_(&self, m: CGAffineTransform, flag: BOOL) -> CIFilterShape
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, transformBy : m, interior : flag)
    }
    unsafe fn insetByX_Y_(
        &self,
        dx: ::std::os::raw::c_int,
        dy: ::std::os::raw::c_int,
    ) -> CIFilterShape
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, insetByX : dx, Y : dy)
    }
    unsafe fn unionWith_(&self, s2: CIFilterShape) -> CIFilterShape
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, unionWith : s2)
    }
    unsafe fn unionWithRect_(&self, r: CGRect) -> CIFilterShape
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, unionWithRect : r)
    }
    unsafe fn intersectWith_(&self, s2: CIFilterShape) -> CIFilterShape
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, intersectWith : s2)
    }
    unsafe fn intersectWithRect_(&self, r: CGRect) -> CIFilterShape
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, intersectWithRect : r)
    }
    unsafe fn extent(&self) -> CGRect
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, extent)
    }
    unsafe fn shapeWithRect_(r: CGRect) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilterShape").unwrap(), shapeWithRect : r)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CISampler(pub id);
impl std::ops::Deref for CISampler {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CISampler {}
impl CISampler {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CISampler").unwrap(), alloc) })
    }
}
impl PNSCopying for CISampler {}
impl INSObject for CISampler {}
impl PNSObject for CISampler {}
impl std::convert::TryFrom<NSObject> for CISampler {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<CISampler, Self::Error> {
        let is_kind_of: bool = unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CISampler").unwrap()) };
        if is_kind_of {
            Ok(CISampler(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to CISampler")
        }
    }
}
impl ICISampler for CISampler {}
pub trait ICISampler: Sized + std::ops::Deref {
    unsafe fn initWithImage_(&self, im: CIImage) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithImage : im)
    }
    unsafe fn initWithImage_keysAndValues_(&self, im: CIImage, key0: id) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithImage : im, keysAndValues : key0)
    }
    unsafe fn initWithImage_options_(&self, im: CIImage, dict: NSDictionary) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithImage : im, options : dict)
    }
    unsafe fn definition(&self) -> CIFilterShape
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, definition)
    }
    unsafe fn extent(&self) -> CGRect
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, extent)
    }
    unsafe fn samplerWithImage_(im: CIImage) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CISampler").unwrap(), samplerWithImage : im)
    }
    unsafe fn samplerWithImage_keysAndValues_(im: CIImage, key0: id) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CISampler").unwrap(), samplerWithImage : im, keysAndValues : key0)
    }
    unsafe fn samplerWithImage_options_(im: CIImage, dict: NSDictionary) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CISampler").unwrap(), samplerWithImage : im, options : dict)
    }
}
pub type CIRAWFilterOption = NSString;
impl CIFilter_CIRAWFilter for CIFilter {}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CIRAWFilter(pub id);
impl std::ops::Deref for CIRAWFilter {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CIRAWFilter {}
impl CIRAWFilter {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CIRAWFilter").unwrap(), alloc) })
    }
}
impl ICIFilter for CIRAWFilter {}
impl PNSSecureCoding for CIRAWFilter {}
impl PNSCopying for CIRAWFilter {}
impl From<CIRAWFilter> for CIFilter {
    fn from(child: CIRAWFilter) -> CIFilter {
        CIFilter(child.0)
    }
}
impl std::convert::TryFrom<CIFilter> for CIRAWFilter {
    type Error = &'static str;
    fn try_from(parent: CIFilter) -> Result<CIRAWFilter, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CIRAWFilter").unwrap()) };
        if is_kind_of {
            Ok(CIRAWFilter(parent.0))
        } else {
            Err("This CIFilter cannot be downcasted to CIRAWFilter")
        }
    }
}
impl INSObject for CIRAWFilter {}
impl PNSObject for CIRAWFilter {}
impl ICIRAWFilter for CIRAWFilter {}
pub trait ICIRAWFilter: Sized + std::ops::Deref {
    unsafe fn supportedDecoderVersions(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, supportedDecoderVersions)
    }
    unsafe fn nativeSize(&self) -> CGSize
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, nativeSize)
    }
    unsafe fn properties(&self) -> NSDictionary
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, properties)
    }
    unsafe fn orientation(&self) -> CGImagePropertyOrientation
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, orientation)
    }
    unsafe fn setOrientation_(&self, orientation: CGImagePropertyOrientation)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setOrientation : orientation)
    }
    unsafe fn isDraftModeEnabled(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isDraftModeEnabled)
    }
    unsafe fn setDraftModeEnabled_(&self, draftModeEnabled: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDraftModeEnabled : draftModeEnabled)
    }
    unsafe fn decoderVersion(&self) -> CIRAWDecoderVersion
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, decoderVersion)
    }
    unsafe fn setDecoderVersion_(&self, decoderVersion: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDecoderVersion : decoderVersion)
    }
    unsafe fn scaleFactor(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, scaleFactor)
    }
    unsafe fn setScaleFactor_(&self, scaleFactor: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setScaleFactor : scaleFactor)
    }
    unsafe fn exposure(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, exposure)
    }
    unsafe fn setExposure_(&self, exposure: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setExposure : exposure)
    }
    unsafe fn baselineExposure(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, baselineExposure)
    }
    unsafe fn setBaselineExposure_(&self, baselineExposure: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setBaselineExposure : baselineExposure)
    }
    unsafe fn shadowBias(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, shadowBias)
    }
    unsafe fn setShadowBias_(&self, shadowBias: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setShadowBias : shadowBias)
    }
    unsafe fn boostAmount(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, boostAmount)
    }
    unsafe fn setBoostAmount_(&self, boostAmount: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setBoostAmount : boostAmount)
    }
    unsafe fn boostShadowAmount(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, boostShadowAmount)
    }
    unsafe fn setBoostShadowAmount_(&self, boostShadowAmount: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setBoostShadowAmount : boostShadowAmount)
    }
    unsafe fn isHighlightRecoverySupported(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isHighlightRecoverySupported)
    }
    unsafe fn isHighlightRecoveryEnabled(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isHighlightRecoveryEnabled)
    }
    unsafe fn setHighlightRecoveryEnabled_(&self, highlightRecoveryEnabled: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setHighlightRecoveryEnabled : highlightRecoveryEnabled)
    }
    unsafe fn isGamutMappingEnabled(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isGamutMappingEnabled)
    }
    unsafe fn setGamutMappingEnabled_(&self, gamutMappingEnabled: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setGamutMappingEnabled : gamutMappingEnabled)
    }
    unsafe fn isLensCorrectionSupported(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isLensCorrectionSupported)
    }
    unsafe fn isLensCorrectionEnabled(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isLensCorrectionEnabled)
    }
    unsafe fn setLensCorrectionEnabled_(&self, lensCorrectionEnabled: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setLensCorrectionEnabled : lensCorrectionEnabled)
    }
    unsafe fn isLuminanceNoiseReductionSupported(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isLuminanceNoiseReductionSupported)
    }
    unsafe fn luminanceNoiseReductionAmount(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, luminanceNoiseReductionAmount)
    }
    unsafe fn setLuminanceNoiseReductionAmount_(&self, luminanceNoiseReductionAmount: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setLuminanceNoiseReductionAmount : luminanceNoiseReductionAmount)
    }
    unsafe fn isColorNoiseReductionSupported(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isColorNoiseReductionSupported)
    }
    unsafe fn colorNoiseReductionAmount(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, colorNoiseReductionAmount)
    }
    unsafe fn setColorNoiseReductionAmount_(&self, colorNoiseReductionAmount: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setColorNoiseReductionAmount : colorNoiseReductionAmount)
    }
    unsafe fn isSharpnessSupported(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isSharpnessSupported)
    }
    unsafe fn sharpnessAmount(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, sharpnessAmount)
    }
    unsafe fn setSharpnessAmount_(&self, sharpnessAmount: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSharpnessAmount : sharpnessAmount)
    }
    unsafe fn isContrastSupported(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isContrastSupported)
    }
    unsafe fn contrastAmount(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, contrastAmount)
    }
    unsafe fn setContrastAmount_(&self, contrastAmount: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setContrastAmount : contrastAmount)
    }
    unsafe fn isDetailSupported(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isDetailSupported)
    }
    unsafe fn detailAmount(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, detailAmount)
    }
    unsafe fn setDetailAmount_(&self, detailAmount: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDetailAmount : detailAmount)
    }
    unsafe fn isMoireReductionSupported(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isMoireReductionSupported)
    }
    unsafe fn moireReductionAmount(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, moireReductionAmount)
    }
    unsafe fn setMoireReductionAmount_(&self, moireReductionAmount: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMoireReductionAmount : moireReductionAmount)
    }
    unsafe fn isLocalToneMapSupported(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isLocalToneMapSupported)
    }
    unsafe fn localToneMapAmount(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, localToneMapAmount)
    }
    unsafe fn setLocalToneMapAmount_(&self, localToneMapAmount: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setLocalToneMapAmount : localToneMapAmount)
    }
    unsafe fn extendedDynamicRangeAmount(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, extendedDynamicRangeAmount)
    }
    unsafe fn setExtendedDynamicRangeAmount_(&self, extendedDynamicRangeAmount: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setExtendedDynamicRangeAmount : extendedDynamicRangeAmount)
    }
    unsafe fn neutralChromaticity(&self) -> CGPoint
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, neutralChromaticity)
    }
    unsafe fn setNeutralChromaticity_(&self, neutralChromaticity: CGPoint)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setNeutralChromaticity : neutralChromaticity)
    }
    unsafe fn neutralLocation(&self) -> CGPoint
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, neutralLocation)
    }
    unsafe fn setNeutralLocation_(&self, neutralLocation: CGPoint)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setNeutralLocation : neutralLocation)
    }
    unsafe fn neutralTemperature(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, neutralTemperature)
    }
    unsafe fn setNeutralTemperature_(&self, neutralTemperature: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setNeutralTemperature : neutralTemperature)
    }
    unsafe fn neutralTint(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, neutralTint)
    }
    unsafe fn setNeutralTint_(&self, neutralTint: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setNeutralTint : neutralTint)
    }
    unsafe fn linearSpaceFilter(&self) -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, linearSpaceFilter)
    }
    unsafe fn setLinearSpaceFilter_(&self, linearSpaceFilter: CIFilter)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setLinearSpaceFilter : linearSpaceFilter)
    }
    unsafe fn previewImage(&self) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, previewImage)
    }
    unsafe fn portraitEffectsMatte(&self) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, portraitEffectsMatte)
    }
    unsafe fn semanticSegmentationSkinMatte(&self) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, semanticSegmentationSkinMatte)
    }
    unsafe fn semanticSegmentationHairMatte(&self) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, semanticSegmentationHairMatte)
    }
    unsafe fn semanticSegmentationGlassesMatte(&self) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, semanticSegmentationGlassesMatte)
    }
    unsafe fn semanticSegmentationSkyMatte(&self) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, semanticSegmentationSkyMatte)
    }
    unsafe fn semanticSegmentationTeethMatte(&self) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, semanticSegmentationTeethMatte)
    }
    unsafe fn filterWithImageURL_(url: NSURL) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIRAWFilter").unwrap(), filterWithImageURL : url)
    }
    unsafe fn filterWithImageData_identifierHint_(
        data: NSData,
        identifierHint: NSString,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIRAWFilter").unwrap(), filterWithImageData : data, identifierHint : identifierHint)
    }
    unsafe fn filterWithCVPixelBuffer_properties_(
        buffer: CVPixelBufferRef,
        properties: NSDictionary,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIRAWFilter").unwrap(), filterWithCVPixelBuffer : buffer, properties : properties)
    }
    unsafe fn supportedCameraModels() -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIRAWFilter").unwrap(), supportedCameraModels)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CIRenderDestination(pub id);
impl std::ops::Deref for CIRenderDestination {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CIRenderDestination {}
impl CIRenderDestination {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CIRenderDestination").unwrap(), alloc) })
    }
}
impl INSObject for CIRenderDestination {}
impl PNSObject for CIRenderDestination {}
impl std::convert::TryFrom<NSObject> for CIRenderDestination {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<CIRenderDestination, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CIRenderDestination").unwrap()) };
        if is_kind_of {
            Ok(CIRenderDestination(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to CIRenderDestination")
        }
    }
}
impl ICIRenderDestination for CIRenderDestination {}
pub trait ICIRenderDestination: Sized + std::ops::Deref {
    unsafe fn initWithPixelBuffer_(&self, pixelBuffer: CVPixelBufferRef) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithPixelBuffer : pixelBuffer)
    }
    unsafe fn initWithIOSurface_(&self, surface: IOSurface) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithIOSurface : surface)
    }
    unsafe fn initWithMTLTexture_commandBuffer_(
        &self,
        texture: *mut u64,
        commandBuffer: *mut u64,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithMTLTexture : texture, commandBuffer : commandBuffer)
    }
    unsafe fn initWithWidth_height_pixelFormat_commandBuffer_mtlTextureProvider_(
        &self,
        width: NSUInteger,
        height: NSUInteger,
        pixelFormat: MTLPixelFormat,
        commandBuffer: *mut u64,
        block: *mut ::std::os::raw::c_void,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithWidth : width, height : height, pixelFormat : pixelFormat, commandBuffer : commandBuffer, mtlTextureProvider : block)
    }
    unsafe fn initWithGLTexture_target_width_height_(
        &self,
        texture: ::std::os::raw::c_uint,
        target: ::std::os::raw::c_uint,
        width: NSUInteger,
        height: NSUInteger,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithGLTexture : texture, target : target, width : width, height : height)
    }
    unsafe fn initWithBitmapData_width_height_bytesPerRow_format_(
        &self,
        data: *mut ::std::os::raw::c_void,
        width: NSUInteger,
        height: NSUInteger,
        bytesPerRow: NSUInteger,
        format: CIFormat,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithBitmapData : data, width : width, height : height, bytesPerRow : bytesPerRow, format : format)
    }
    unsafe fn width(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, width)
    }
    unsafe fn height(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, height)
    }
    unsafe fn alphaMode(&self) -> CIRenderDestinationAlphaMode
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, alphaMode)
    }
    unsafe fn setAlphaMode_(&self, alphaMode: CIRenderDestinationAlphaMode)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAlphaMode : alphaMode)
    }
    unsafe fn isFlipped(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isFlipped)
    }
    unsafe fn setFlipped_(&self, flipped: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setFlipped : flipped)
    }
    unsafe fn isDithered(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isDithered)
    }
    unsafe fn setDithered_(&self, dithered: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDithered : dithered)
    }
    unsafe fn isClamped(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isClamped)
    }
    unsafe fn setClamped_(&self, clamped: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setClamped : clamped)
    }
    unsafe fn colorSpace(&self) -> CGColorSpaceRef
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, colorSpace)
    }
    unsafe fn setColorSpace_(&self, colorSpace: CGColorSpaceRef)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setColorSpace : colorSpace)
    }
    unsafe fn blendKernel(&self) -> CIBlendKernel
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, blendKernel)
    }
    unsafe fn setBlendKernel_(&self, blendKernel: CIBlendKernel)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setBlendKernel : blendKernel)
    }
    unsafe fn blendsInDestinationColorSpace(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, blendsInDestinationColorSpace)
    }
    unsafe fn setBlendsInDestinationColorSpace_(&self, blendsInDestinationColorSpace: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setBlendsInDestinationColorSpace : blendsInDestinationColorSpace)
    }
    unsafe fn captureTraceURL(&self) -> NSURL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, captureTraceURL)
    }
    unsafe fn setCaptureTraceURL_(&self, captureTraceURL: NSURL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCaptureTraceURL : captureTraceURL)
    }
}
pub type CIRenderDestinationAlphaMode = NSUInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CIRenderInfo(pub id);
impl std::ops::Deref for CIRenderInfo {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CIRenderInfo {}
impl CIRenderInfo {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CIRenderInfo").unwrap(), alloc) })
    }
}
impl INSObject for CIRenderInfo {}
impl PNSObject for CIRenderInfo {}
impl std::convert::TryFrom<NSObject> for CIRenderInfo {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<CIRenderInfo, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CIRenderInfo").unwrap()) };
        if is_kind_of {
            Ok(CIRenderInfo(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to CIRenderInfo")
        }
    }
}
impl ICIRenderInfo for CIRenderInfo {}
pub trait ICIRenderInfo: Sized + std::ops::Deref {
    unsafe fn kernelExecutionTime(&self) -> NSTimeInterval
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, kernelExecutionTime)
    }
    unsafe fn kernelCompileTime(&self) -> NSTimeInterval
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, kernelCompileTime)
    }
    unsafe fn passCount(&self) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, passCount)
    }
    unsafe fn pixelsProcessed(&self) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, pixelsProcessed)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CIRenderTask(pub id);
impl std::ops::Deref for CIRenderTask {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CIRenderTask {}
impl CIRenderTask {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CIRenderTask").unwrap(), alloc) })
    }
}
impl INSObject for CIRenderTask {}
impl PNSObject for CIRenderTask {}
impl std::convert::TryFrom<NSObject> for CIRenderTask {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<CIRenderTask, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CIRenderTask").unwrap()) };
        if is_kind_of {
            Ok(CIRenderTask(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to CIRenderTask")
        }
    }
}
impl ICIRenderTask for CIRenderTask {}
pub trait ICIRenderTask: Sized + std::ops::Deref {
    unsafe fn waitUntilCompletedAndReturnError_(&self, error: *mut NSError) -> CIRenderInfo
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, waitUntilCompletedAndReturnError : error)
    }
}
impl CIContext_CIRenderDestination for CIContext {}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CIBarcodeDescriptor(pub id);
impl std::ops::Deref for CIBarcodeDescriptor {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CIBarcodeDescriptor {}
impl CIBarcodeDescriptor {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CIBarcodeDescriptor").unwrap(), alloc) })
    }
}
impl PNSSecureCoding for CIBarcodeDescriptor {}
impl PNSCopying for CIBarcodeDescriptor {}
impl INSObject for CIBarcodeDescriptor {}
impl PNSObject for CIBarcodeDescriptor {}
impl std::convert::TryFrom<NSObject> for CIBarcodeDescriptor {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<CIBarcodeDescriptor, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CIBarcodeDescriptor").unwrap()) };
        if is_kind_of {
            Ok(CIBarcodeDescriptor(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to CIBarcodeDescriptor")
        }
    }
}
impl ICIBarcodeDescriptor for CIBarcodeDescriptor {}
pub trait ICIBarcodeDescriptor: Sized + std::ops::Deref {}
pub type CIQRCodeErrorCorrectionLevel = NSInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CIQRCodeDescriptor(pub id);
impl std::ops::Deref for CIQRCodeDescriptor {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CIQRCodeDescriptor {}
impl CIQRCodeDescriptor {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CIQRCodeDescriptor").unwrap(), alloc) })
    }
}
impl ICIBarcodeDescriptor for CIQRCodeDescriptor {}
impl PNSSecureCoding for CIQRCodeDescriptor {}
impl PNSCopying for CIQRCodeDescriptor {}
impl From<CIQRCodeDescriptor> for CIBarcodeDescriptor {
    fn from(child: CIQRCodeDescriptor) -> CIBarcodeDescriptor {
        CIBarcodeDescriptor(child.0)
    }
}
impl std::convert::TryFrom<CIBarcodeDescriptor> for CIQRCodeDescriptor {
    type Error = &'static str;
    fn try_from(parent: CIBarcodeDescriptor) -> Result<CIQRCodeDescriptor, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CIQRCodeDescriptor").unwrap()) };
        if is_kind_of {
            Ok(CIQRCodeDescriptor(parent.0))
        } else {
            Err("This CIBarcodeDescriptor cannot be downcasted to CIQRCodeDescriptor")
        }
    }
}
impl INSObject for CIQRCodeDescriptor {}
impl PNSObject for CIQRCodeDescriptor {}
impl ICIQRCodeDescriptor for CIQRCodeDescriptor {}
pub trait ICIQRCodeDescriptor: Sized + std::ops::Deref {
    unsafe fn initWithPayload_symbolVersion_maskPattern_errorCorrectionLevel_(
        &self,
        errorCorrectedPayload: NSData,
        symbolVersion: NSInteger,
        maskPattern: u8,
        errorCorrectionLevel: CIQRCodeErrorCorrectionLevel,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithPayload : errorCorrectedPayload, symbolVersion : symbolVersion, maskPattern : maskPattern, errorCorrectionLevel : errorCorrectionLevel)
    }
    unsafe fn errorCorrectedPayload(&self) -> NSData
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, errorCorrectedPayload)
    }
    unsafe fn symbolVersion(&self) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, symbolVersion)
    }
    unsafe fn maskPattern(&self) -> u8
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, maskPattern)
    }
    unsafe fn errorCorrectionLevel(&self) -> CIQRCodeErrorCorrectionLevel
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, errorCorrectionLevel)
    }
    unsafe fn descriptorWithPayload_symbolVersion_maskPattern_errorCorrectionLevel_(
        errorCorrectedPayload: NSData,
        symbolVersion: NSInteger,
        maskPattern: u8,
        errorCorrectionLevel: CIQRCodeErrorCorrectionLevel,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIQRCodeDescriptor").unwrap(), descriptorWithPayload : errorCorrectedPayload, symbolVersion : symbolVersion, maskPattern : maskPattern, errorCorrectionLevel : errorCorrectionLevel)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CIAztecCodeDescriptor(pub id);
impl std::ops::Deref for CIAztecCodeDescriptor {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CIAztecCodeDescriptor {}
impl CIAztecCodeDescriptor {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CIAztecCodeDescriptor").unwrap(), alloc) })
    }
}
impl ICIBarcodeDescriptor for CIAztecCodeDescriptor {}
impl PNSSecureCoding for CIAztecCodeDescriptor {}
impl PNSCopying for CIAztecCodeDescriptor {}
impl std::convert::TryFrom<CIBarcodeDescriptor> for CIAztecCodeDescriptor {
    type Error = &'static str;
    fn try_from(parent: CIBarcodeDescriptor) -> Result<CIAztecCodeDescriptor, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CIAztecCodeDescriptor").unwrap()) };
        if is_kind_of {
            Ok(CIAztecCodeDescriptor(parent.0))
        } else {
            Err("This CIBarcodeDescriptor cannot be downcasted to CIAztecCodeDescriptor")
        }
    }
}
impl INSObject for CIAztecCodeDescriptor {}
impl PNSObject for CIAztecCodeDescriptor {}
impl ICIAztecCodeDescriptor for CIAztecCodeDescriptor {}
pub trait ICIAztecCodeDescriptor: Sized + std::ops::Deref {
    unsafe fn initWithPayload_isCompact_layerCount_dataCodewordCount_(
        &self,
        errorCorrectedPayload: NSData,
        isCompact: BOOL,
        layerCount: NSInteger,
        dataCodewordCount: NSInteger,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithPayload : errorCorrectedPayload, isCompact : isCompact, layerCount : layerCount, dataCodewordCount : dataCodewordCount)
    }
    unsafe fn errorCorrectedPayload(&self) -> NSData
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, errorCorrectedPayload)
    }
    unsafe fn isCompact(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isCompact)
    }
    unsafe fn layerCount(&self) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, layerCount)
    }
    unsafe fn dataCodewordCount(&self) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, dataCodewordCount)
    }
    unsafe fn descriptorWithPayload_isCompact_layerCount_dataCodewordCount_(
        errorCorrectedPayload: NSData,
        isCompact: BOOL,
        layerCount: NSInteger,
        dataCodewordCount: NSInteger,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIAztecCodeDescriptor").unwrap(), descriptorWithPayload : errorCorrectedPayload, isCompact : isCompact, layerCount : layerCount, dataCodewordCount : dataCodewordCount)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CIPDF417CodeDescriptor(pub id);
impl std::ops::Deref for CIPDF417CodeDescriptor {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CIPDF417CodeDescriptor {}
impl CIPDF417CodeDescriptor {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CIPDF417CodeDescriptor").unwrap(), alloc) })
    }
}
impl ICIBarcodeDescriptor for CIPDF417CodeDescriptor {}
impl PNSSecureCoding for CIPDF417CodeDescriptor {}
impl PNSCopying for CIPDF417CodeDescriptor {}
impl std::convert::TryFrom<CIBarcodeDescriptor> for CIPDF417CodeDescriptor {
    type Error = &'static str;
    fn try_from(parent: CIBarcodeDescriptor) -> Result<CIPDF417CodeDescriptor, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CIPDF417CodeDescriptor").unwrap()) };
        if is_kind_of {
            Ok(CIPDF417CodeDescriptor(parent.0))
        } else {
            Err("This CIBarcodeDescriptor cannot be downcasted to CIPDF417CodeDescriptor")
        }
    }
}
impl INSObject for CIPDF417CodeDescriptor {}
impl PNSObject for CIPDF417CodeDescriptor {}
impl ICIPDF417CodeDescriptor for CIPDF417CodeDescriptor {}
pub trait ICIPDF417CodeDescriptor: Sized + std::ops::Deref {
    unsafe fn initWithPayload_isCompact_rowCount_columnCount_(
        &self,
        errorCorrectedPayload: NSData,
        isCompact: BOOL,
        rowCount: NSInteger,
        columnCount: NSInteger,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithPayload : errorCorrectedPayload, isCompact : isCompact, rowCount : rowCount, columnCount : columnCount)
    }
    unsafe fn errorCorrectedPayload(&self) -> NSData
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, errorCorrectedPayload)
    }
    unsafe fn isCompact(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isCompact)
    }
    unsafe fn rowCount(&self) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, rowCount)
    }
    unsafe fn columnCount(&self) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, columnCount)
    }
    unsafe fn descriptorWithPayload_isCompact_rowCount_columnCount_(
        errorCorrectedPayload: NSData,
        isCompact: BOOL,
        rowCount: NSInteger,
        columnCount: NSInteger,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIPDF417CodeDescriptor").unwrap(), descriptorWithPayload : errorCorrectedPayload, isCompact : isCompact, rowCount : rowCount, columnCount : columnCount)
    }
}
pub type CIDataMatrixCodeECCVersion = NSInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CIDataMatrixCodeDescriptor(pub id);
impl std::ops::Deref for CIDataMatrixCodeDescriptor {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CIDataMatrixCodeDescriptor {}
impl CIDataMatrixCodeDescriptor {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CIDataMatrixCodeDescriptor").unwrap(), alloc) })
    }
}
impl ICIBarcodeDescriptor for CIDataMatrixCodeDescriptor {}
impl PNSSecureCoding for CIDataMatrixCodeDescriptor {}
impl PNSCopying for CIDataMatrixCodeDescriptor {}
impl std::convert::TryFrom<CIBarcodeDescriptor> for CIDataMatrixCodeDescriptor {
    type Error = &'static str;
    fn try_from(parent: CIBarcodeDescriptor) -> Result<CIDataMatrixCodeDescriptor, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CIDataMatrixCodeDescriptor").unwrap()) };
        if is_kind_of {
            Ok(CIDataMatrixCodeDescriptor(parent.0))
        } else {
            Err("This CIBarcodeDescriptor cannot be downcasted to CIDataMatrixCodeDescriptor")
        }
    }
}
impl INSObject for CIDataMatrixCodeDescriptor {}
impl PNSObject for CIDataMatrixCodeDescriptor {}
impl ICIDataMatrixCodeDescriptor for CIDataMatrixCodeDescriptor {}
pub trait ICIDataMatrixCodeDescriptor: Sized + std::ops::Deref {
    unsafe fn initWithPayload_rowCount_columnCount_eccVersion_(
        &self,
        errorCorrectedPayload: NSData,
        rowCount: NSInteger,
        columnCount: NSInteger,
        eccVersion: CIDataMatrixCodeECCVersion,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithPayload : errorCorrectedPayload, rowCount : rowCount, columnCount : columnCount, eccVersion : eccVersion)
    }
    unsafe fn errorCorrectedPayload(&self) -> NSData
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, errorCorrectedPayload)
    }
    unsafe fn rowCount(&self) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, rowCount)
    }
    unsafe fn columnCount(&self) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, columnCount)
    }
    unsafe fn eccVersion(&self) -> CIDataMatrixCodeECCVersion
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, eccVersion)
    }
    unsafe fn descriptorWithPayload_rowCount_columnCount_eccVersion_(
        errorCorrectedPayload: NSData,
        rowCount: NSInteger,
        columnCount: NSInteger,
        eccVersion: CIDataMatrixCodeECCVersion,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIDataMatrixCodeDescriptor").unwrap(), descriptorWithPayload : errorCorrectedPayload, rowCount : rowCount, columnCount : columnCount, eccVersion : eccVersion)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CIFilterGenerator(pub id);
impl std::ops::Deref for CIFilterGenerator {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CIFilterGenerator {}
impl CIFilterGenerator {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilterGenerator").unwrap(), alloc) })
    }
}
impl PNSSecureCoding for CIFilterGenerator {}
impl PNSCopying for CIFilterGenerator {}
impl PCIFilterConstructor for CIFilterGenerator {}
impl INSObject for CIFilterGenerator {}
impl PNSObject for CIFilterGenerator {}
impl std::convert::TryFrom<NSObject> for CIFilterGenerator {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<CIFilterGenerator, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CIFilterGenerator").unwrap()) };
        if is_kind_of {
            Ok(CIFilterGenerator(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to CIFilterGenerator")
        }
    }
}
impl ICIFilterGenerator for CIFilterGenerator {}
pub trait ICIFilterGenerator: Sized + std::ops::Deref {
    unsafe fn initWithContentsOfURL_(&self, aURL: NSURL) -> id
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithContentsOfURL : aURL)
    }
    unsafe fn connectObject_withKey_toObject_withKey_(
        &self,
        sourceObject: id,
        sourceKey: NSString,
        targetObject: id,
        targetKey: NSString,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, connectObject : sourceObject, withKey : sourceKey, toObject : targetObject, withKey : targetKey)
    }
    unsafe fn disconnectObject_withKey_toObject_withKey_(
        &self,
        sourceObject: id,
        sourceKey: NSString,
        targetObject: id,
        targetKey: NSString,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, disconnectObject : sourceObject, withKey : sourceKey, toObject : targetObject, withKey : targetKey)
    }
    unsafe fn exportKey_fromObject_withName_(
        &self,
        key: NSString,
        targetObject: id,
        exportedKeyName: NSString,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, exportKey : key, fromObject : targetObject, withName : exportedKeyName)
    }
    unsafe fn removeExportedKey_(&self, exportedKeyName: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, removeExportedKey : exportedKeyName)
    }
    unsafe fn setAttributes_forExportedKey_(&self, attributes: NSDictionary, key: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAttributes : attributes, forExportedKey : key)
    }
    unsafe fn filter(&self) -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, filter)
    }
    unsafe fn registerFilterName_(&self, name: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, registerFilterName : name)
    }
    unsafe fn writeToURL_atomically_(&self, aURL: NSURL, flag: BOOL) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, writeToURL : aURL, atomically : flag)
    }
    unsafe fn exportedKeys(&self) -> NSDictionary
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, exportedKeys)
    }
    unsafe fn classAttributes(&self) -> NSDictionary
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, classAttributes)
    }
    unsafe fn setClassAttributes_(&self, classAttributes: NSDictionary)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setClassAttributes : classAttributes)
    }
    unsafe fn filterGenerator() -> CIFilterGenerator
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilterGenerator").unwrap(), filterGenerator)
    }
    unsafe fn filterGeneratorWithContentsOfURL_(aURL: NSURL) -> CIFilterGenerator
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIFilterGenerator").unwrap(), filterGeneratorWithContentsOfURL : aURL)
    }
}
pub trait PCIPlugInRegistration: Sized + std::ops::Deref {
    unsafe fn load_(&self, host: *mut ::std::os::raw::c_void) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, load : host)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CIPlugIn(pub id);
impl std::ops::Deref for CIPlugIn {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for CIPlugIn {}
impl CIPlugIn {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"CIPlugIn").unwrap(), alloc) })
    }
}
impl INSObject for CIPlugIn {}
impl PNSObject for CIPlugIn {}
impl std::convert::TryFrom<NSObject> for CIPlugIn {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<CIPlugIn, Self::Error> {
        let is_kind_of: bool = unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"CIPlugIn").unwrap()) };
        if is_kind_of {
            Ok(CIPlugIn(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to CIPlugIn")
        }
    }
}
impl ICIPlugIn for CIPlugIn {}
pub trait ICIPlugIn: Sized + std::ops::Deref {
    unsafe fn loadAllPlugIns()
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIPlugIn").unwrap(), loadAllPlugIns)
    }
    unsafe fn loadNonExecutablePlugIns()
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIPlugIn").unwrap(), loadNonExecutablePlugIns)
    }
    unsafe fn loadPlugIn_allowNonExecutable_(url: NSURL, allowNonExecutable: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIPlugIn").unwrap(), loadPlugIn : url, allowNonExecutable : allowNonExecutable)
    }
    unsafe fn loadPlugIn_allowExecutableCode_(url: NSURL, allowExecutableCode: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIPlugIn").unwrap(), loadPlugIn : url, allowExecutableCode : allowExecutableCode)
    }
    unsafe fn loadNonExecutablePlugIn_(url: NSURL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"CIPlugIn").unwrap(), loadNonExecutablePlugIn : url)
    }
}
pub trait PCIDistanceGradientFromRedMask: Sized + std::ops::Deref {
    unsafe fn inputImage(&self) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inputImage)
    }
    unsafe fn setInputImage_(&self, inputImage: CIImage)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInputImage : inputImage)
    }
    unsafe fn maximumDistance(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, maximumDistance)
    }
    unsafe fn setMaximumDistance_(&self, maximumDistance: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMaximumDistance : maximumDistance)
    }
}
pub trait PCIGaussianGradient: Sized + std::ops::Deref {
    unsafe fn center(&self) -> CGPoint
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, center)
    }
    unsafe fn setCenter_(&self, center: CGPoint)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCenter : center)
    }
    unsafe fn color0(&self) -> CIColor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, color0)
    }
    unsafe fn setColor0_(&self, color0: CIColor)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setColor0 : color0)
    }
    unsafe fn color1(&self) -> CIColor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, color1)
    }
    unsafe fn setColor1_(&self, color1: CIColor)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setColor1 : color1)
    }
    unsafe fn radius(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, radius)
    }
    unsafe fn setRadius_(&self, radius: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRadius : radius)
    }
}
pub trait PCIHueSaturationValueGradient: Sized + std::ops::Deref {
    unsafe fn value(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, value)
    }
    unsafe fn setValue_(&self, value: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setValue : value)
    }
    unsafe fn radius(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, radius)
    }
    unsafe fn setRadius_(&self, radius: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRadius : radius)
    }
    unsafe fn softness(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, softness)
    }
    unsafe fn setSoftness_(&self, softness: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSoftness : softness)
    }
    unsafe fn dither(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, dither)
    }
    unsafe fn setDither_(&self, dither: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDither : dither)
    }
    unsafe fn colorSpace(&self) -> CGColorSpaceRef
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, colorSpace)
    }
    unsafe fn setColorSpace_(&self, colorSpace: CGColorSpaceRef)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setColorSpace : colorSpace)
    }
}
pub trait PCILinearGradient: Sized + std::ops::Deref {
    unsafe fn point0(&self) -> CGPoint
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, point0)
    }
    unsafe fn setPoint0_(&self, point0: CGPoint)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPoint0 : point0)
    }
    unsafe fn point1(&self) -> CGPoint
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, point1)
    }
    unsafe fn setPoint1_(&self, point1: CGPoint)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPoint1 : point1)
    }
    unsafe fn color0(&self) -> CIColor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, color0)
    }
    unsafe fn setColor0_(&self, color0: CIColor)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setColor0 : color0)
    }
    unsafe fn color1(&self) -> CIColor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, color1)
    }
    unsafe fn setColor1_(&self, color1: CIColor)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setColor1 : color1)
    }
}
pub trait PCIRadialGradient: Sized + std::ops::Deref {
    unsafe fn center(&self) -> CGPoint
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, center)
    }
    unsafe fn setCenter_(&self, center: CGPoint)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCenter : center)
    }
    unsafe fn radius0(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, radius0)
    }
    unsafe fn setRadius0_(&self, radius0: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRadius0 : radius0)
    }
    unsafe fn radius1(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, radius1)
    }
    unsafe fn setRadius1_(&self, radius1: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRadius1 : radius1)
    }
    unsafe fn color0(&self) -> CIColor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, color0)
    }
    unsafe fn setColor0_(&self, color0: CIColor)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setColor0 : color0)
    }
    unsafe fn color1(&self) -> CIColor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, color1)
    }
    unsafe fn setColor1_(&self, color1: CIColor)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setColor1 : color1)
    }
}
pub trait PCISignedDistanceGradientFromRedMask: Sized + std::ops::Deref {
    unsafe fn inputImage(&self) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inputImage)
    }
    unsafe fn setInputImage_(&self, inputImage: CIImage)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInputImage : inputImage)
    }
    unsafe fn maximumDistance(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, maximumDistance)
    }
    unsafe fn setMaximumDistance_(&self, maximumDistance: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMaximumDistance : maximumDistance)
    }
}
pub trait PCISmoothLinearGradient: Sized + std::ops::Deref {
    unsafe fn point0(&self) -> CGPoint
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, point0)
    }
    unsafe fn setPoint0_(&self, point0: CGPoint)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPoint0 : point0)
    }
    unsafe fn point1(&self) -> CGPoint
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, point1)
    }
    unsafe fn setPoint1_(&self, point1: CGPoint)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPoint1 : point1)
    }
    unsafe fn color0(&self) -> CIColor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, color0)
    }
    unsafe fn setColor0_(&self, color0: CIColor)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setColor0 : color0)
    }
    unsafe fn color1(&self) -> CIColor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, color1)
    }
    unsafe fn setColor1_(&self, color1: CIColor)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setColor1 : color1)
    }
}
pub trait PCISharpenLuminance: Sized + std::ops::Deref {
    unsafe fn inputImage(&self) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inputImage)
    }
    unsafe fn setInputImage_(&self, inputImage: CIImage)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInputImage : inputImage)
    }
    unsafe fn sharpness(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, sharpness)
    }
    unsafe fn setSharpness_(&self, sharpness: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSharpness : sharpness)
    }
    unsafe fn radius(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, radius)
    }
    unsafe fn setRadius_(&self, radius: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRadius : radius)
    }
}
pub trait PCIUnsharpMask: Sized + std::ops::Deref {
    unsafe fn inputImage(&self) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inputImage)
    }
    unsafe fn setInputImage_(&self, inputImage: CIImage)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInputImage : inputImage)
    }
    unsafe fn radius(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, radius)
    }
    unsafe fn setRadius_(&self, radius: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRadius : radius)
    }
    unsafe fn intensity(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, intensity)
    }
    unsafe fn setIntensity_(&self, intensity: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setIntensity : intensity)
    }
}
pub trait PCICircularScreen: Sized + std::ops::Deref {
    unsafe fn inputImage(&self) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inputImage)
    }
    unsafe fn setInputImage_(&self, inputImage: CIImage)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInputImage : inputImage)
    }
    unsafe fn center(&self) -> CGPoint
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, center)
    }
    unsafe fn setCenter_(&self, center: CGPoint)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCenter : center)
    }
    unsafe fn width(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, width)
    }
    unsafe fn setWidth_(&self, width: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setWidth : width)
    }
    unsafe fn sharpness(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, sharpness)
    }
    unsafe fn setSharpness_(&self, sharpness: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSharpness : sharpness)
    }
}
pub trait PCICMYKHalftone: Sized + std::ops::Deref {
    unsafe fn inputImage(&self) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inputImage)
    }
    unsafe fn setInputImage_(&self, inputImage: CIImage)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInputImage : inputImage)
    }
    unsafe fn center(&self) -> CGPoint
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, center)
    }
    unsafe fn setCenter_(&self, center: CGPoint)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCenter : center)
    }
    unsafe fn width(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, width)
    }
    unsafe fn setWidth_(&self, width: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setWidth : width)
    }
    unsafe fn angle(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, angle)
    }
    unsafe fn setAngle_(&self, angle: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAngle : angle)
    }
    unsafe fn sharpness(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, sharpness)
    }
    unsafe fn setSharpness_(&self, sharpness: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSharpness : sharpness)
    }
    unsafe fn grayComponentReplacement(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, grayComponentReplacement)
    }
    unsafe fn setGrayComponentReplacement_(&self, grayComponentReplacement: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setGrayComponentReplacement : grayComponentReplacement)
    }
    unsafe fn underColorRemoval(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, underColorRemoval)
    }
    unsafe fn setUnderColorRemoval_(&self, underColorRemoval: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setUnderColorRemoval : underColorRemoval)
    }
}
pub trait PCIDotScreen: Sized + std::ops::Deref {
    unsafe fn inputImage(&self) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inputImage)
    }
    unsafe fn setInputImage_(&self, inputImage: CIImage)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInputImage : inputImage)
    }
    unsafe fn center(&self) -> CGPoint
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, center)
    }
    unsafe fn setCenter_(&self, center: CGPoint)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCenter : center)
    }
    unsafe fn angle(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, angle)
    }
    unsafe fn setAngle_(&self, angle: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAngle : angle)
    }
    unsafe fn width(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, width)
    }
    unsafe fn setWidth_(&self, width: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setWidth : width)
    }
    unsafe fn sharpness(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, sharpness)
    }
    unsafe fn setSharpness_(&self, sharpness: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSharpness : sharpness)
    }
}
pub trait PCIHatchedScreen: Sized + std::ops::Deref {
    unsafe fn inputImage(&self) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inputImage)
    }
    unsafe fn setInputImage_(&self, inputImage: CIImage)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInputImage : inputImage)
    }
    unsafe fn center(&self) -> CGPoint
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, center)
    }
    unsafe fn setCenter_(&self, center: CGPoint)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCenter : center)
    }
    unsafe fn angle(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, angle)
    }
    unsafe fn setAngle_(&self, angle: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAngle : angle)
    }
    unsafe fn width(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, width)
    }
    unsafe fn setWidth_(&self, width: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setWidth : width)
    }
    unsafe fn sharpness(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, sharpness)
    }
    unsafe fn setSharpness_(&self, sharpness: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSharpness : sharpness)
    }
}
pub trait PCILineScreen: Sized + std::ops::Deref {
    unsafe fn inputImage(&self) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inputImage)
    }
    unsafe fn setInputImage_(&self, inputImage: CIImage)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInputImage : inputImage)
    }
    unsafe fn center(&self) -> CGPoint
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, center)
    }
    unsafe fn setCenter_(&self, center: CGPoint)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCenter : center)
    }
    unsafe fn angle(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, angle)
    }
    unsafe fn setAngle_(&self, angle: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAngle : angle)
    }
    unsafe fn width(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, width)
    }
    unsafe fn setWidth_(&self, width: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setWidth : width)
    }
    unsafe fn sharpness(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, sharpness)
    }
    unsafe fn setSharpness_(&self, sharpness: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSharpness : sharpness)
    }
}
pub trait PCIFourCoordinateGeometryFilter: Sized + std::ops::Deref {
    unsafe fn inputImage(&self) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inputImage)
    }
    unsafe fn setInputImage_(&self, inputImage: CIImage)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInputImage : inputImage)
    }
    unsafe fn topLeft(&self) -> CGPoint
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, topLeft)
    }
    unsafe fn setTopLeft_(&self, topLeft: CGPoint)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setTopLeft : topLeft)
    }
    unsafe fn topRight(&self) -> CGPoint
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, topRight)
    }
    unsafe fn setTopRight_(&self, topRight: CGPoint)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setTopRight : topRight)
    }
    unsafe fn bottomRight(&self) -> CGPoint
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, bottomRight)
    }
    unsafe fn setBottomRight_(&self, bottomRight: CGPoint)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setBottomRight : bottomRight)
    }
    unsafe fn bottomLeft(&self) -> CGPoint
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, bottomLeft)
    }
    unsafe fn setBottomLeft_(&self, bottomLeft: CGPoint)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setBottomLeft : bottomLeft)
    }
}
pub trait PCIBicubicScaleTransform: Sized + std::ops::Deref {
    unsafe fn inputImage(&self) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inputImage)
    }
    unsafe fn setInputImage_(&self, inputImage: CIImage)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInputImage : inputImage)
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
    unsafe fn aspectRatio(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, aspectRatio)
    }
    unsafe fn setAspectRatio_(&self, aspectRatio: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAspectRatio : aspectRatio)
    }
    unsafe fn parameterB(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, parameterB)
    }
    unsafe fn setParameterB_(&self, parameterB: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setParameterB : parameterB)
    }
    unsafe fn parameterC(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, parameterC)
    }
    unsafe fn setParameterC_(&self, parameterC: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setParameterC : parameterC)
    }
}
pub trait PCIEdgePreserveUpsample: Sized + std::ops::Deref {
    unsafe fn inputImage(&self) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inputImage)
    }
    unsafe fn setInputImage_(&self, inputImage: CIImage)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInputImage : inputImage)
    }
    unsafe fn smallImage(&self) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, smallImage)
    }
    unsafe fn setSmallImage_(&self, smallImage: CIImage)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSmallImage : smallImage)
    }
    unsafe fn spatialSigma(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, spatialSigma)
    }
    unsafe fn setSpatialSigma_(&self, spatialSigma: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSpatialSigma : spatialSigma)
    }
    unsafe fn lumaSigma(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, lumaSigma)
    }
    unsafe fn setLumaSigma_(&self, lumaSigma: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setLumaSigma : lumaSigma)
    }
}
pub trait PCIKeystoneCorrectionCombined: Sized + std::ops::Deref {
    unsafe fn focalLength(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, focalLength)
    }
    unsafe fn setFocalLength_(&self, focalLength: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setFocalLength : focalLength)
    }
}
pub trait PCIKeystoneCorrectionHorizontal: Sized + std::ops::Deref {
    unsafe fn focalLength(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, focalLength)
    }
    unsafe fn setFocalLength_(&self, focalLength: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setFocalLength : focalLength)
    }
}
pub trait PCIKeystoneCorrectionVertical: Sized + std::ops::Deref {
    unsafe fn focalLength(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, focalLength)
    }
    unsafe fn setFocalLength_(&self, focalLength: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setFocalLength : focalLength)
    }
}
pub trait PCILanczosScaleTransform: Sized + std::ops::Deref {
    unsafe fn inputImage(&self) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inputImage)
    }
    unsafe fn setInputImage_(&self, inputImage: CIImage)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInputImage : inputImage)
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
    unsafe fn aspectRatio(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, aspectRatio)
    }
    unsafe fn setAspectRatio_(&self, aspectRatio: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAspectRatio : aspectRatio)
    }
}
pub trait PCIMaximumScaleTransform: Sized + std::ops::Deref {
    unsafe fn inputImage(&self) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inputImage)
    }
    unsafe fn setInputImage_(&self, inputImage: CIImage)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInputImage : inputImage)
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
    unsafe fn aspectRatio(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, aspectRatio)
    }
    unsafe fn setAspectRatio_(&self, aspectRatio: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAspectRatio : aspectRatio)
    }
}
pub trait PCIPerspectiveCorrection: Sized + std::ops::Deref {
    unsafe fn crop(&self) -> bool
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, crop)
    }
    unsafe fn setCrop_(&self, crop: bool)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCrop : crop)
    }
}
pub trait PCIPerspectiveRotate: Sized + std::ops::Deref {
    unsafe fn inputImage(&self) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inputImage)
    }
    unsafe fn setInputImage_(&self, inputImage: CIImage)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInputImage : inputImage)
    }
    unsafe fn focalLength(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, focalLength)
    }
    unsafe fn setFocalLength_(&self, focalLength: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setFocalLength : focalLength)
    }
    unsafe fn pitch(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, pitch)
    }
    unsafe fn setPitch_(&self, pitch: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPitch : pitch)
    }
    unsafe fn yaw(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, yaw)
    }
    unsafe fn setYaw_(&self, yaw: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setYaw : yaw)
    }
    unsafe fn roll(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, roll)
    }
    unsafe fn setRoll_(&self, roll: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRoll : roll)
    }
}
pub trait PCIPerspectiveTransform: Sized + std::ops::Deref {}
pub trait PCIPerspectiveTransformWithExtent: Sized + std::ops::Deref {
    unsafe fn extent(&self) -> CGRect
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, extent)
    }
    unsafe fn setExtent_(&self, extent: CGRect)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setExtent : extent)
    }
}
pub trait PCIStraighten: Sized + std::ops::Deref {
    unsafe fn inputImage(&self) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inputImage)
    }
    unsafe fn setInputImage_(&self, inputImage: CIImage)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInputImage : inputImage)
    }
    unsafe fn angle(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, angle)
    }
    unsafe fn setAngle_(&self, angle: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAngle : angle)
    }
}
pub trait PCITransitionFilter: Sized + std::ops::Deref {
    unsafe fn inputImage(&self) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inputImage)
    }
    unsafe fn setInputImage_(&self, inputImage: CIImage)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInputImage : inputImage)
    }
    unsafe fn targetImage(&self) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, targetImage)
    }
    unsafe fn setTargetImage_(&self, targetImage: CIImage)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setTargetImage : targetImage)
    }
    unsafe fn time(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, time)
    }
    unsafe fn setTime_(&self, time: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setTime : time)
    }
}
pub trait PCIAccordionFoldTransition: Sized + std::ops::Deref {
    unsafe fn bottomHeight(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, bottomHeight)
    }
    unsafe fn setBottomHeight_(&self, bottomHeight: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setBottomHeight : bottomHeight)
    }
    unsafe fn numberOfFolds(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, numberOfFolds)
    }
    unsafe fn setNumberOfFolds_(&self, numberOfFolds: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setNumberOfFolds : numberOfFolds)
    }
    unsafe fn foldShadowAmount(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, foldShadowAmount)
    }
    unsafe fn setFoldShadowAmount_(&self, foldShadowAmount: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setFoldShadowAmount : foldShadowAmount)
    }
}
pub trait PCIBarsSwipeTransition: Sized + std::ops::Deref {
    unsafe fn angle(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, angle)
    }
    unsafe fn setAngle_(&self, angle: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAngle : angle)
    }
    unsafe fn width(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, width)
    }
    unsafe fn setWidth_(&self, width: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setWidth : width)
    }
    unsafe fn barOffset(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, barOffset)
    }
    unsafe fn setBarOffset_(&self, barOffset: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setBarOffset : barOffset)
    }
}
pub trait PCICopyMachineTransition: Sized + std::ops::Deref {
    unsafe fn extent(&self) -> CGRect
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, extent)
    }
    unsafe fn setExtent_(&self, extent: CGRect)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setExtent : extent)
    }
    unsafe fn color(&self) -> CIColor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, color)
    }
    unsafe fn setColor_(&self, color: CIColor)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setColor : color)
    }
    unsafe fn angle(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, angle)
    }
    unsafe fn setAngle_(&self, angle: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAngle : angle)
    }
    unsafe fn width(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, width)
    }
    unsafe fn setWidth_(&self, width: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setWidth : width)
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
}
pub trait PCIDisintegrateWithMaskTransition: Sized + std::ops::Deref {
    unsafe fn maskImage(&self) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, maskImage)
    }
    unsafe fn setMaskImage_(&self, maskImage: CIImage)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMaskImage : maskImage)
    }
    unsafe fn shadowRadius(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, shadowRadius)
    }
    unsafe fn setShadowRadius_(&self, shadowRadius: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setShadowRadius : shadowRadius)
    }
    unsafe fn shadowDensity(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, shadowDensity)
    }
    unsafe fn setShadowDensity_(&self, shadowDensity: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setShadowDensity : shadowDensity)
    }
    unsafe fn shadowOffset(&self) -> CGPoint
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, shadowOffset)
    }
    unsafe fn setShadowOffset_(&self, shadowOffset: CGPoint)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setShadowOffset : shadowOffset)
    }
}
pub trait PCIDissolveTransition: Sized + std::ops::Deref {}
pub trait PCIFlashTransition: Sized + std::ops::Deref {
    unsafe fn center(&self) -> CGPoint
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, center)
    }
    unsafe fn setCenter_(&self, center: CGPoint)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCenter : center)
    }
    unsafe fn extent(&self) -> CGRect
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, extent)
    }
    unsafe fn setExtent_(&self, extent: CGRect)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setExtent : extent)
    }
    unsafe fn color(&self) -> CIColor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, color)
    }
    unsafe fn setColor_(&self, color: CIColor)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setColor : color)
    }
    unsafe fn maxStriationRadius(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, maxStriationRadius)
    }
    unsafe fn setMaxStriationRadius_(&self, maxStriationRadius: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMaxStriationRadius : maxStriationRadius)
    }
    unsafe fn striationStrength(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, striationStrength)
    }
    unsafe fn setStriationStrength_(&self, striationStrength: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setStriationStrength : striationStrength)
    }
    unsafe fn striationContrast(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, striationContrast)
    }
    unsafe fn setStriationContrast_(&self, striationContrast: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setStriationContrast : striationContrast)
    }
    unsafe fn fadeThreshold(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, fadeThreshold)
    }
    unsafe fn setFadeThreshold_(&self, fadeThreshold: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setFadeThreshold : fadeThreshold)
    }
}
pub trait PCIModTransition: Sized + std::ops::Deref {
    unsafe fn center(&self) -> CGPoint
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, center)
    }
    unsafe fn setCenter_(&self, center: CGPoint)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCenter : center)
    }
    unsafe fn angle(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, angle)
    }
    unsafe fn setAngle_(&self, angle: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAngle : angle)
    }
    unsafe fn radius(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, radius)
    }
    unsafe fn setRadius_(&self, radius: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRadius : radius)
    }
    unsafe fn compression(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, compression)
    }
    unsafe fn setCompression_(&self, compression: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCompression : compression)
    }
}
pub trait PCIPageCurlTransition: Sized + std::ops::Deref {
    unsafe fn backsideImage(&self) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, backsideImage)
    }
    unsafe fn setBacksideImage_(&self, backsideImage: CIImage)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setBacksideImage : backsideImage)
    }
    unsafe fn shadingImage(&self) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, shadingImage)
    }
    unsafe fn setShadingImage_(&self, shadingImage: CIImage)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setShadingImage : shadingImage)
    }
    unsafe fn extent(&self) -> CGRect
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, extent)
    }
    unsafe fn setExtent_(&self, extent: CGRect)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setExtent : extent)
    }
    unsafe fn angle(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, angle)
    }
    unsafe fn setAngle_(&self, angle: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAngle : angle)
    }
    unsafe fn radius(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, radius)
    }
    unsafe fn setRadius_(&self, radius: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRadius : radius)
    }
}
pub trait PCIPageCurlWithShadowTransition: Sized + std::ops::Deref {
    unsafe fn backsideImage(&self) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, backsideImage)
    }
    unsafe fn setBacksideImage_(&self, backsideImage: CIImage)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setBacksideImage : backsideImage)
    }
    unsafe fn extent(&self) -> CGRect
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, extent)
    }
    unsafe fn setExtent_(&self, extent: CGRect)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setExtent : extent)
    }
    unsafe fn angle(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, angle)
    }
    unsafe fn setAngle_(&self, angle: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAngle : angle)
    }
    unsafe fn radius(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, radius)
    }
    unsafe fn setRadius_(&self, radius: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRadius : radius)
    }
    unsafe fn shadowSize(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, shadowSize)
    }
    unsafe fn setShadowSize_(&self, shadowSize: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setShadowSize : shadowSize)
    }
    unsafe fn shadowAmount(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, shadowAmount)
    }
    unsafe fn setShadowAmount_(&self, shadowAmount: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setShadowAmount : shadowAmount)
    }
    unsafe fn shadowExtent(&self) -> CGRect
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, shadowExtent)
    }
    unsafe fn setShadowExtent_(&self, shadowExtent: CGRect)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setShadowExtent : shadowExtent)
    }
}
pub trait PCIRippleTransition: Sized + std::ops::Deref {
    unsafe fn shadingImage(&self) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, shadingImage)
    }
    unsafe fn setShadingImage_(&self, shadingImage: CIImage)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setShadingImage : shadingImage)
    }
    unsafe fn center(&self) -> CGPoint
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, center)
    }
    unsafe fn setCenter_(&self, center: CGPoint)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCenter : center)
    }
    unsafe fn extent(&self) -> CGRect
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, extent)
    }
    unsafe fn setExtent_(&self, extent: CGRect)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setExtent : extent)
    }
    unsafe fn width(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, width)
    }
    unsafe fn setWidth_(&self, width: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setWidth : width)
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
}
pub trait PCISwipeTransition: Sized + std::ops::Deref {
    unsafe fn extent(&self) -> CGRect
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, extent)
    }
    unsafe fn setExtent_(&self, extent: CGRect)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setExtent : extent)
    }
    unsafe fn color(&self) -> CIColor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, color)
    }
    unsafe fn setColor_(&self, color: CIColor)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setColor : color)
    }
    unsafe fn angle(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, angle)
    }
    unsafe fn setAngle_(&self, angle: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAngle : angle)
    }
    unsafe fn width(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, width)
    }
    unsafe fn setWidth_(&self, width: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setWidth : width)
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
}
pub trait PCICompositeOperation: Sized + std::ops::Deref {
    unsafe fn inputImage(&self) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inputImage)
    }
    unsafe fn setInputImage_(&self, inputImage: CIImage)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInputImage : inputImage)
    }
    unsafe fn backgroundImage(&self) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, backgroundImage)
    }
    unsafe fn setBackgroundImage_(&self, backgroundImage: CIImage)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setBackgroundImage : backgroundImage)
    }
}
pub trait PCIColorAbsoluteDifference: Sized + std::ops::Deref {
    unsafe fn inputImage(&self) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inputImage)
    }
    unsafe fn setInputImage_(&self, inputImage: CIImage)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInputImage : inputImage)
    }
    unsafe fn inputImage2(&self) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inputImage2)
    }
    unsafe fn setInputImage2_(&self, inputImage2: CIImage)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInputImage2 : inputImage2)
    }
}
pub trait PCIColorClamp: Sized + std::ops::Deref {
    unsafe fn inputImage(&self) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inputImage)
    }
    unsafe fn setInputImage_(&self, inputImage: CIImage)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInputImage : inputImage)
    }
    unsafe fn minComponents(&self) -> CIVector
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, minComponents)
    }
    unsafe fn setMinComponents_(&self, minComponents: CIVector)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMinComponents : minComponents)
    }
    unsafe fn maxComponents(&self) -> CIVector
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, maxComponents)
    }
    unsafe fn setMaxComponents_(&self, maxComponents: CIVector)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMaxComponents : maxComponents)
    }
}
pub trait PCIColorControls: Sized + std::ops::Deref {
    unsafe fn inputImage(&self) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inputImage)
    }
    unsafe fn setInputImage_(&self, inputImage: CIImage)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInputImage : inputImage)
    }
    unsafe fn saturation(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, saturation)
    }
    unsafe fn setSaturation_(&self, saturation: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSaturation : saturation)
    }
    unsafe fn brightness(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, brightness)
    }
    unsafe fn setBrightness_(&self, brightness: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setBrightness : brightness)
    }
    unsafe fn contrast(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, contrast)
    }
    unsafe fn setContrast_(&self, contrast: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setContrast : contrast)
    }
}
pub trait PCIColorMatrix: Sized + std::ops::Deref {
    unsafe fn inputImage(&self) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inputImage)
    }
    unsafe fn setInputImage_(&self, inputImage: CIImage)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInputImage : inputImage)
    }
    unsafe fn RVector(&self) -> CIVector
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, RVector)
    }
    unsafe fn setRVector_(&self, RVector: CIVector)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRVector : RVector)
    }
    unsafe fn GVector(&self) -> CIVector
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, GVector)
    }
    unsafe fn setGVector_(&self, GVector: CIVector)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setGVector : GVector)
    }
    unsafe fn BVector(&self) -> CIVector
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, BVector)
    }
    unsafe fn setBVector_(&self, BVector: CIVector)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setBVector : BVector)
    }
    unsafe fn AVector(&self) -> CIVector
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, AVector)
    }
    unsafe fn setAVector_(&self, AVector: CIVector)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAVector : AVector)
    }
    unsafe fn biasVector(&self) -> CIVector
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, biasVector)
    }
    unsafe fn setBiasVector_(&self, biasVector: CIVector)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setBiasVector : biasVector)
    }
}
pub trait PCIColorPolynomial: Sized + std::ops::Deref {
    unsafe fn inputImage(&self) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inputImage)
    }
    unsafe fn setInputImage_(&self, inputImage: CIImage)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInputImage : inputImage)
    }
    unsafe fn redCoefficients(&self) -> CIVector
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, redCoefficients)
    }
    unsafe fn setRedCoefficients_(&self, redCoefficients: CIVector)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRedCoefficients : redCoefficients)
    }
    unsafe fn greenCoefficients(&self) -> CIVector
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, greenCoefficients)
    }
    unsafe fn setGreenCoefficients_(&self, greenCoefficients: CIVector)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setGreenCoefficients : greenCoefficients)
    }
    unsafe fn blueCoefficients(&self) -> CIVector
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, blueCoefficients)
    }
    unsafe fn setBlueCoefficients_(&self, blueCoefficients: CIVector)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setBlueCoefficients : blueCoefficients)
    }
    unsafe fn alphaCoefficients(&self) -> CIVector
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, alphaCoefficients)
    }
    unsafe fn setAlphaCoefficients_(&self, alphaCoefficients: CIVector)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAlphaCoefficients : alphaCoefficients)
    }
}
pub trait PCIColorThreshold: Sized + std::ops::Deref {
    unsafe fn inputImage(&self) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inputImage)
    }
    unsafe fn setInputImage_(&self, inputImage: CIImage)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInputImage : inputImage)
    }
    unsafe fn threshold(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, threshold)
    }
    unsafe fn setThreshold_(&self, threshold: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setThreshold : threshold)
    }
}
pub trait PCIColorThresholdOtsu: Sized + std::ops::Deref {
    unsafe fn inputImage(&self) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inputImage)
    }
    unsafe fn setInputImage_(&self, inputImage: CIImage)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInputImage : inputImage)
    }
}
pub trait PCIDepthToDisparity: Sized + std::ops::Deref {
    unsafe fn inputImage(&self) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inputImage)
    }
    unsafe fn setInputImage_(&self, inputImage: CIImage)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInputImage : inputImage)
    }
}
pub trait PCIDisparityToDepth: Sized + std::ops::Deref {
    unsafe fn inputImage(&self) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inputImage)
    }
    unsafe fn setInputImage_(&self, inputImage: CIImage)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInputImage : inputImage)
    }
}
pub trait PCIExposureAdjust: Sized + std::ops::Deref {
    unsafe fn inputImage(&self) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inputImage)
    }
    unsafe fn setInputImage_(&self, inputImage: CIImage)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInputImage : inputImage)
    }
    unsafe fn EV(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, EV)
    }
    unsafe fn setEV_(&self, EV: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setEV : EV)
    }
}
pub trait PCIGammaAdjust: Sized + std::ops::Deref {
    unsafe fn inputImage(&self) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inputImage)
    }
    unsafe fn setInputImage_(&self, inputImage: CIImage)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInputImage : inputImage)
    }
    unsafe fn power(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, power)
    }
    unsafe fn setPower_(&self, power: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPower : power)
    }
}
pub trait PCIHueAdjust: Sized + std::ops::Deref {
    unsafe fn inputImage(&self) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inputImage)
    }
    unsafe fn setInputImage_(&self, inputImage: CIImage)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInputImage : inputImage)
    }
    unsafe fn angle(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, angle)
    }
    unsafe fn setAngle_(&self, angle: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAngle : angle)
    }
}
pub trait PCILinearToSRGBToneCurve: Sized + std::ops::Deref {
    unsafe fn inputImage(&self) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inputImage)
    }
    unsafe fn setInputImage_(&self, inputImage: CIImage)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInputImage : inputImage)
    }
}
pub trait PCISRGBToneCurveToLinear: Sized + std::ops::Deref {
    unsafe fn inputImage(&self) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inputImage)
    }
    unsafe fn setInputImage_(&self, inputImage: CIImage)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInputImage : inputImage)
    }
}
pub trait PCISystemToneMap: Sized + std::ops::Deref {
    unsafe fn inputImage(&self) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inputImage)
    }
    unsafe fn setInputImage_(&self, inputImage: CIImage)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInputImage : inputImage)
    }
    unsafe fn displayHeadroom(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, displayHeadroom)
    }
    unsafe fn setDisplayHeadroom_(&self, displayHeadroom: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDisplayHeadroom : displayHeadroom)
    }
    unsafe fn preferredDynamicRange(&self) -> CIDynamicRangeOption
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
}
pub trait PCITemperatureAndTint: Sized + std::ops::Deref {
    unsafe fn inputImage(&self) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inputImage)
    }
    unsafe fn setInputImage_(&self, inputImage: CIImage)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInputImage : inputImage)
    }
    unsafe fn neutral(&self) -> CIVector
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, neutral)
    }
    unsafe fn setNeutral_(&self, neutral: CIVector)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setNeutral : neutral)
    }
    unsafe fn targetNeutral(&self) -> CIVector
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, targetNeutral)
    }
    unsafe fn setTargetNeutral_(&self, targetNeutral: CIVector)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setTargetNeutral : targetNeutral)
    }
}
pub trait PCIToneCurve: Sized + std::ops::Deref {
    unsafe fn inputImage(&self) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inputImage)
    }
    unsafe fn setInputImage_(&self, inputImage: CIImage)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInputImage : inputImage)
    }
    unsafe fn point0(&self) -> CGPoint
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, point0)
    }
    unsafe fn setPoint0_(&self, point0: CGPoint)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPoint0 : point0)
    }
    unsafe fn point1(&self) -> CGPoint
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, point1)
    }
    unsafe fn setPoint1_(&self, point1: CGPoint)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPoint1 : point1)
    }
    unsafe fn point2(&self) -> CGPoint
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, point2)
    }
    unsafe fn setPoint2_(&self, point2: CGPoint)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPoint2 : point2)
    }
    unsafe fn point3(&self) -> CGPoint
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, point3)
    }
    unsafe fn setPoint3_(&self, point3: CGPoint)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPoint3 : point3)
    }
    unsafe fn point4(&self) -> CGPoint
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, point4)
    }
    unsafe fn setPoint4_(&self, point4: CGPoint)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPoint4 : point4)
    }
    unsafe fn extrapolate(&self) -> bool
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, extrapolate)
    }
    unsafe fn setExtrapolate_(&self, extrapolate: bool)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setExtrapolate : extrapolate)
    }
}
pub trait PCIToneMapHeadroom: Sized + std::ops::Deref {
    unsafe fn inputImage(&self) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inputImage)
    }
    unsafe fn setInputImage_(&self, inputImage: CIImage)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInputImage : inputImage)
    }
    unsafe fn sourceHeadroom(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, sourceHeadroom)
    }
    unsafe fn setSourceHeadroom_(&self, sourceHeadroom: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSourceHeadroom : sourceHeadroom)
    }
    unsafe fn targetHeadroom(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, targetHeadroom)
    }
    unsafe fn setTargetHeadroom_(&self, targetHeadroom: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setTargetHeadroom : targetHeadroom)
    }
}
pub trait PCIVibrance: Sized + std::ops::Deref {
    unsafe fn inputImage(&self) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inputImage)
    }
    unsafe fn setInputImage_(&self, inputImage: CIImage)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInputImage : inputImage)
    }
    unsafe fn amount(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, amount)
    }
    unsafe fn setAmount_(&self, amount: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAmount : amount)
    }
}
pub trait PCIWhitePointAdjust: Sized + std::ops::Deref {
    unsafe fn inputImage(&self) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inputImage)
    }
    unsafe fn setInputImage_(&self, inputImage: CIImage)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInputImage : inputImage)
    }
    unsafe fn color(&self) -> CIColor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, color)
    }
    unsafe fn setColor_(&self, color: CIColor)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setColor : color)
    }
}
pub trait PCIColorCrossPolynomial: Sized + std::ops::Deref {
    unsafe fn inputImage(&self) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inputImage)
    }
    unsafe fn setInputImage_(&self, inputImage: CIImage)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInputImage : inputImage)
    }
    unsafe fn redCoefficients(&self) -> CIVector
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, redCoefficients)
    }
    unsafe fn setRedCoefficients_(&self, redCoefficients: CIVector)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRedCoefficients : redCoefficients)
    }
    unsafe fn greenCoefficients(&self) -> CIVector
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, greenCoefficients)
    }
    unsafe fn setGreenCoefficients_(&self, greenCoefficients: CIVector)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setGreenCoefficients : greenCoefficients)
    }
    unsafe fn blueCoefficients(&self) -> CIVector
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, blueCoefficients)
    }
    unsafe fn setBlueCoefficients_(&self, blueCoefficients: CIVector)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setBlueCoefficients : blueCoefficients)
    }
}
pub trait PCIColorCube: Sized + std::ops::Deref {
    unsafe fn inputImage(&self) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inputImage)
    }
    unsafe fn setInputImage_(&self, inputImage: CIImage)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInputImage : inputImage)
    }
    unsafe fn cubeDimension(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, cubeDimension)
    }
    unsafe fn setCubeDimension_(&self, cubeDimension: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCubeDimension : cubeDimension)
    }
    unsafe fn cubeData(&self) -> NSData
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, cubeData)
    }
    unsafe fn setCubeData_(&self, cubeData: NSData)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCubeData : cubeData)
    }
    unsafe fn extrapolate(&self) -> bool
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, extrapolate)
    }
    unsafe fn setExtrapolate_(&self, extrapolate: bool)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setExtrapolate : extrapolate)
    }
}
pub trait PCIColorCubesMixedWithMask: Sized + std::ops::Deref {
    unsafe fn inputImage(&self) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inputImage)
    }
    unsafe fn setInputImage_(&self, inputImage: CIImage)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInputImage : inputImage)
    }
    unsafe fn maskImage(&self) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, maskImage)
    }
    unsafe fn setMaskImage_(&self, maskImage: CIImage)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMaskImage : maskImage)
    }
    unsafe fn cubeDimension(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, cubeDimension)
    }
    unsafe fn setCubeDimension_(&self, cubeDimension: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCubeDimension : cubeDimension)
    }
    unsafe fn cube0Data(&self) -> NSData
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, cube0Data)
    }
    unsafe fn setCube0Data_(&self, cube0Data: NSData)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCube0Data : cube0Data)
    }
    unsafe fn cube1Data(&self) -> NSData
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, cube1Data)
    }
    unsafe fn setCube1Data_(&self, cube1Data: NSData)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCube1Data : cube1Data)
    }
    unsafe fn colorSpace(&self) -> CGColorSpaceRef
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, colorSpace)
    }
    unsafe fn setColorSpace_(&self, colorSpace: CGColorSpaceRef)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setColorSpace : colorSpace)
    }
    unsafe fn extrapolate(&self) -> bool
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, extrapolate)
    }
    unsafe fn setExtrapolate_(&self, extrapolate: bool)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setExtrapolate : extrapolate)
    }
}
pub trait PCIColorCubeWithColorSpace: Sized + std::ops::Deref {
    unsafe fn inputImage(&self) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inputImage)
    }
    unsafe fn setInputImage_(&self, inputImage: CIImage)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInputImage : inputImage)
    }
    unsafe fn cubeDimension(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, cubeDimension)
    }
    unsafe fn setCubeDimension_(&self, cubeDimension: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCubeDimension : cubeDimension)
    }
    unsafe fn cubeData(&self) -> NSData
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, cubeData)
    }
    unsafe fn setCubeData_(&self, cubeData: NSData)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCubeData : cubeData)
    }
    unsafe fn extrapolate(&self) -> bool
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, extrapolate)
    }
    unsafe fn setExtrapolate_(&self, extrapolate: bool)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setExtrapolate : extrapolate)
    }
    unsafe fn colorSpace(&self) -> CGColorSpaceRef
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, colorSpace)
    }
    unsafe fn setColorSpace_(&self, colorSpace: CGColorSpaceRef)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setColorSpace : colorSpace)
    }
}
pub trait PCIColorCurves: Sized + std::ops::Deref {
    unsafe fn inputImage(&self) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inputImage)
    }
    unsafe fn setInputImage_(&self, inputImage: CIImage)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInputImage : inputImage)
    }
    unsafe fn curvesData(&self) -> NSData
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, curvesData)
    }
    unsafe fn setCurvesData_(&self, curvesData: NSData)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCurvesData : curvesData)
    }
    unsafe fn curvesDomain(&self) -> CIVector
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, curvesDomain)
    }
    unsafe fn setCurvesDomain_(&self, curvesDomain: CIVector)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCurvesDomain : curvesDomain)
    }
    unsafe fn colorSpace(&self) -> CGColorSpaceRef
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, colorSpace)
    }
    unsafe fn setColorSpace_(&self, colorSpace: CGColorSpaceRef)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setColorSpace : colorSpace)
    }
}
pub trait PCIColorInvert: Sized + std::ops::Deref {
    unsafe fn inputImage(&self) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inputImage)
    }
    unsafe fn setInputImage_(&self, inputImage: CIImage)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInputImage : inputImage)
    }
}
pub trait PCIColorMap: Sized + std::ops::Deref {
    unsafe fn inputImage(&self) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inputImage)
    }
    unsafe fn setInputImage_(&self, inputImage: CIImage)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInputImage : inputImage)
    }
    unsafe fn gradientImage(&self) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, gradientImage)
    }
    unsafe fn setGradientImage_(&self, gradientImage: CIImage)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setGradientImage : gradientImage)
    }
}
pub trait PCIColorMonochrome: Sized + std::ops::Deref {
    unsafe fn inputImage(&self) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inputImage)
    }
    unsafe fn setInputImage_(&self, inputImage: CIImage)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInputImage : inputImage)
    }
    unsafe fn color(&self) -> CIColor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, color)
    }
    unsafe fn setColor_(&self, color: CIColor)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setColor : color)
    }
    unsafe fn intensity(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, intensity)
    }
    unsafe fn setIntensity_(&self, intensity: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setIntensity : intensity)
    }
}
pub trait PCIColorPosterize: Sized + std::ops::Deref {
    unsafe fn inputImage(&self) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inputImage)
    }
    unsafe fn setInputImage_(&self, inputImage: CIImage)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInputImage : inputImage)
    }
    unsafe fn levels(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, levels)
    }
    unsafe fn setLevels_(&self, levels: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setLevels : levels)
    }
}
pub trait PCIConvertLab: Sized + std::ops::Deref {
    unsafe fn inputImage(&self) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inputImage)
    }
    unsafe fn setInputImage_(&self, inputImage: CIImage)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInputImage : inputImage)
    }
    unsafe fn normalize(&self) -> bool
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, normalize)
    }
    unsafe fn setNormalize_(&self, normalize: bool)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setNormalize : normalize)
    }
}
pub trait PCIDither: Sized + std::ops::Deref {
    unsafe fn inputImage(&self) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inputImage)
    }
    unsafe fn setInputImage_(&self, inputImage: CIImage)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInputImage : inputImage)
    }
    unsafe fn intensity(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, intensity)
    }
    unsafe fn setIntensity_(&self, intensity: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setIntensity : intensity)
    }
}
pub trait PCIDocumentEnhancer: Sized + std::ops::Deref {
    unsafe fn inputImage(&self) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inputImage)
    }
    unsafe fn setInputImage_(&self, inputImage: CIImage)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInputImage : inputImage)
    }
    unsafe fn amount(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, amount)
    }
    unsafe fn setAmount_(&self, amount: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAmount : amount)
    }
}
pub trait PCIFalseColor: Sized + std::ops::Deref {
    unsafe fn inputImage(&self) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inputImage)
    }
    unsafe fn setInputImage_(&self, inputImage: CIImage)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInputImage : inputImage)
    }
    unsafe fn color0(&self) -> CIColor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, color0)
    }
    unsafe fn setColor0_(&self, color0: CIColor)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setColor0 : color0)
    }
    unsafe fn color1(&self) -> CIColor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, color1)
    }
    unsafe fn setColor1_(&self, color1: CIColor)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setColor1 : color1)
    }
}
pub trait PCILabDeltaE: Sized + std::ops::Deref {
    unsafe fn inputImage(&self) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inputImage)
    }
    unsafe fn setInputImage_(&self, inputImage: CIImage)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInputImage : inputImage)
    }
    unsafe fn image2(&self) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, image2)
    }
    unsafe fn setImage2_(&self, image2: CIImage)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setImage2 : image2)
    }
}
pub trait PCIMaskToAlpha: Sized + std::ops::Deref {
    unsafe fn inputImage(&self) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inputImage)
    }
    unsafe fn setInputImage_(&self, inputImage: CIImage)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInputImage : inputImage)
    }
}
pub trait PCIMaximumComponent: Sized + std::ops::Deref {
    unsafe fn inputImage(&self) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inputImage)
    }
    unsafe fn setInputImage_(&self, inputImage: CIImage)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInputImage : inputImage)
    }
}
pub trait PCIMinimumComponent: Sized + std::ops::Deref {
    unsafe fn inputImage(&self) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inputImage)
    }
    unsafe fn setInputImage_(&self, inputImage: CIImage)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInputImage : inputImage)
    }
}
pub trait PCIPaletteCentroid: Sized + std::ops::Deref {
    unsafe fn inputImage(&self) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inputImage)
    }
    unsafe fn setInputImage_(&self, inputImage: CIImage)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInputImage : inputImage)
    }
    unsafe fn paletteImage(&self) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, paletteImage)
    }
    unsafe fn setPaletteImage_(&self, paletteImage: CIImage)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPaletteImage : paletteImage)
    }
    unsafe fn perceptual(&self) -> bool
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, perceptual)
    }
    unsafe fn setPerceptual_(&self, perceptual: bool)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPerceptual : perceptual)
    }
}
pub trait PCIPalettize: Sized + std::ops::Deref {
    unsafe fn inputImage(&self) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inputImage)
    }
    unsafe fn setInputImage_(&self, inputImage: CIImage)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInputImage : inputImage)
    }
    unsafe fn paletteImage(&self) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, paletteImage)
    }
    unsafe fn setPaletteImage_(&self, paletteImage: CIImage)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPaletteImage : paletteImage)
    }
    unsafe fn perceptual(&self) -> bool
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, perceptual)
    }
    unsafe fn setPerceptual_(&self, perceptual: bool)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPerceptual : perceptual)
    }
}
pub trait PCIPhotoEffect: Sized + std::ops::Deref {
    unsafe fn inputImage(&self) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inputImage)
    }
    unsafe fn setInputImage_(&self, inputImage: CIImage)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInputImage : inputImage)
    }
    unsafe fn extrapolate(&self) -> bool
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, extrapolate)
    }
    unsafe fn setExtrapolate_(&self, extrapolate: bool)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setExtrapolate : extrapolate)
    }
}
pub trait PCISepiaTone: Sized + std::ops::Deref {
    unsafe fn inputImage(&self) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inputImage)
    }
    unsafe fn setInputImage_(&self, inputImage: CIImage)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInputImage : inputImage)
    }
    unsafe fn intensity(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, intensity)
    }
    unsafe fn setIntensity_(&self, intensity: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setIntensity : intensity)
    }
}
pub trait PCIThermal: Sized + std::ops::Deref {
    unsafe fn inputImage(&self) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inputImage)
    }
    unsafe fn setInputImage_(&self, inputImage: CIImage)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInputImage : inputImage)
    }
}
pub trait PCIVignette: Sized + std::ops::Deref {
    unsafe fn inputImage(&self) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inputImage)
    }
    unsafe fn setInputImage_(&self, inputImage: CIImage)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInputImage : inputImage)
    }
    unsafe fn intensity(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, intensity)
    }
    unsafe fn setIntensity_(&self, intensity: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setIntensity : intensity)
    }
    unsafe fn radius(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, radius)
    }
    unsafe fn setRadius_(&self, radius: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRadius : radius)
    }
}
pub trait PCIVignetteEffect: Sized + std::ops::Deref {
    unsafe fn inputImage(&self) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inputImage)
    }
    unsafe fn setInputImage_(&self, inputImage: CIImage)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInputImage : inputImage)
    }
    unsafe fn center(&self) -> CGPoint
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, center)
    }
    unsafe fn setCenter_(&self, center: CGPoint)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCenter : center)
    }
    unsafe fn radius(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, radius)
    }
    unsafe fn setRadius_(&self, radius: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRadius : radius)
    }
    unsafe fn intensity(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, intensity)
    }
    unsafe fn setIntensity_(&self, intensity: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setIntensity : intensity)
    }
    unsafe fn falloff(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, falloff)
    }
    unsafe fn setFalloff_(&self, falloff: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setFalloff : falloff)
    }
}
pub trait PCIXRay: Sized + std::ops::Deref {
    unsafe fn inputImage(&self) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inputImage)
    }
    unsafe fn setInputImage_(&self, inputImage: CIImage)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInputImage : inputImage)
    }
}
pub trait PCIBumpDistortion: Sized + std::ops::Deref {
    unsafe fn inputImage(&self) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inputImage)
    }
    unsafe fn setInputImage_(&self, inputImage: CIImage)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInputImage : inputImage)
    }
    unsafe fn center(&self) -> CGPoint
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, center)
    }
    unsafe fn setCenter_(&self, center: CGPoint)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCenter : center)
    }
    unsafe fn radius(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, radius)
    }
    unsafe fn setRadius_(&self, radius: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRadius : radius)
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
}
pub trait PCIBumpDistortionLinear: Sized + std::ops::Deref {
    unsafe fn inputImage(&self) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inputImage)
    }
    unsafe fn setInputImage_(&self, inputImage: CIImage)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInputImage : inputImage)
    }
    unsafe fn center(&self) -> CGPoint
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, center)
    }
    unsafe fn setCenter_(&self, center: CGPoint)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCenter : center)
    }
    unsafe fn radius(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, radius)
    }
    unsafe fn setRadius_(&self, radius: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRadius : radius)
    }
    unsafe fn angle(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, angle)
    }
    unsafe fn setAngle_(&self, angle: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAngle : angle)
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
}
pub trait PCICircleSplashDistortion: Sized + std::ops::Deref {
    unsafe fn inputImage(&self) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inputImage)
    }
    unsafe fn setInputImage_(&self, inputImage: CIImage)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInputImage : inputImage)
    }
    unsafe fn center(&self) -> CGPoint
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, center)
    }
    unsafe fn setCenter_(&self, center: CGPoint)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCenter : center)
    }
    unsafe fn radius(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, radius)
    }
    unsafe fn setRadius_(&self, radius: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRadius : radius)
    }
}
pub trait PCICircularWrap: Sized + std::ops::Deref {
    unsafe fn inputImage(&self) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inputImage)
    }
    unsafe fn setInputImage_(&self, inputImage: CIImage)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInputImage : inputImage)
    }
    unsafe fn center(&self) -> CGPoint
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, center)
    }
    unsafe fn setCenter_(&self, center: CGPoint)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCenter : center)
    }
    unsafe fn radius(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, radius)
    }
    unsafe fn setRadius_(&self, radius: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRadius : radius)
    }
    unsafe fn angle(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, angle)
    }
    unsafe fn setAngle_(&self, angle: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAngle : angle)
    }
}
pub trait PCIDisplacementDistortion: Sized + std::ops::Deref {
    unsafe fn inputImage(&self) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inputImage)
    }
    unsafe fn setInputImage_(&self, inputImage: CIImage)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInputImage : inputImage)
    }
    unsafe fn displacementImage(&self) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, displacementImage)
    }
    unsafe fn setDisplacementImage_(&self, displacementImage: CIImage)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDisplacementImage : displacementImage)
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
}
pub trait PCIDroste: Sized + std::ops::Deref {
    unsafe fn inputImage(&self) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inputImage)
    }
    unsafe fn setInputImage_(&self, inputImage: CIImage)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInputImage : inputImage)
    }
    unsafe fn insetPoint0(&self) -> CGPoint
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, insetPoint0)
    }
    unsafe fn setInsetPoint0_(&self, insetPoint0: CGPoint)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInsetPoint0 : insetPoint0)
    }
    unsafe fn insetPoint1(&self) -> CGPoint
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, insetPoint1)
    }
    unsafe fn setInsetPoint1_(&self, insetPoint1: CGPoint)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInsetPoint1 : insetPoint1)
    }
    unsafe fn strands(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, strands)
    }
    unsafe fn setStrands_(&self, strands: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setStrands : strands)
    }
    unsafe fn periodicity(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, periodicity)
    }
    unsafe fn setPeriodicity_(&self, periodicity: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPeriodicity : periodicity)
    }
    unsafe fn rotation(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, rotation)
    }
    unsafe fn setRotation_(&self, rotation: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRotation : rotation)
    }
    unsafe fn zoom(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, zoom)
    }
    unsafe fn setZoom_(&self, zoom: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setZoom : zoom)
    }
}
pub trait PCIGlassDistortion: Sized + std::ops::Deref {
    unsafe fn inputImage(&self) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inputImage)
    }
    unsafe fn setInputImage_(&self, inputImage: CIImage)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInputImage : inputImage)
    }
    unsafe fn textureImage(&self) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, textureImage)
    }
    unsafe fn setTextureImage_(&self, textureImage: CIImage)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setTextureImage : textureImage)
    }
    unsafe fn center(&self) -> CGPoint
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, center)
    }
    unsafe fn setCenter_(&self, center: CGPoint)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCenter : center)
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
}
pub trait PCIGlassLozenge: Sized + std::ops::Deref {
    unsafe fn inputImage(&self) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inputImage)
    }
    unsafe fn setInputImage_(&self, inputImage: CIImage)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInputImage : inputImage)
    }
    unsafe fn point0(&self) -> CGPoint
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, point0)
    }
    unsafe fn setPoint0_(&self, point0: CGPoint)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPoint0 : point0)
    }
    unsafe fn point1(&self) -> CGPoint
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, point1)
    }
    unsafe fn setPoint1_(&self, point1: CGPoint)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPoint1 : point1)
    }
    unsafe fn radius(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, radius)
    }
    unsafe fn setRadius_(&self, radius: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRadius : radius)
    }
    unsafe fn refraction(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, refraction)
    }
    unsafe fn setRefraction_(&self, refraction: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRefraction : refraction)
    }
}
pub trait PCIHoleDistortion: Sized + std::ops::Deref {
    unsafe fn inputImage(&self) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inputImage)
    }
    unsafe fn setInputImage_(&self, inputImage: CIImage)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInputImage : inputImage)
    }
    unsafe fn center(&self) -> CGPoint
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, center)
    }
    unsafe fn setCenter_(&self, center: CGPoint)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCenter : center)
    }
    unsafe fn radius(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, radius)
    }
    unsafe fn setRadius_(&self, radius: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRadius : radius)
    }
}
pub trait PCILightTunnel: Sized + std::ops::Deref {
    unsafe fn inputImage(&self) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inputImage)
    }
    unsafe fn setInputImage_(&self, inputImage: CIImage)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInputImage : inputImage)
    }
    unsafe fn center(&self) -> CGPoint
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, center)
    }
    unsafe fn setCenter_(&self, center: CGPoint)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCenter : center)
    }
    unsafe fn rotation(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, rotation)
    }
    unsafe fn setRotation_(&self, rotation: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRotation : rotation)
    }
    unsafe fn radius(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, radius)
    }
    unsafe fn setRadius_(&self, radius: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRadius : radius)
    }
}
pub trait PCINinePartStretched: Sized + std::ops::Deref {
    unsafe fn inputImage(&self) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inputImage)
    }
    unsafe fn setInputImage_(&self, inputImage: CIImage)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInputImage : inputImage)
    }
    unsafe fn breakpoint0(&self) -> CGPoint
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, breakpoint0)
    }
    unsafe fn setBreakpoint0_(&self, breakpoint0: CGPoint)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setBreakpoint0 : breakpoint0)
    }
    unsafe fn breakpoint1(&self) -> CGPoint
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, breakpoint1)
    }
    unsafe fn setBreakpoint1_(&self, breakpoint1: CGPoint)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setBreakpoint1 : breakpoint1)
    }
    unsafe fn growAmount(&self) -> CGPoint
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, growAmount)
    }
    unsafe fn setGrowAmount_(&self, growAmount: CGPoint)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setGrowAmount : growAmount)
    }
}
pub trait PCINinePartTiled: Sized + std::ops::Deref {
    unsafe fn inputImage(&self) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inputImage)
    }
    unsafe fn setInputImage_(&self, inputImage: CIImage)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInputImage : inputImage)
    }
    unsafe fn breakpoint0(&self) -> CGPoint
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, breakpoint0)
    }
    unsafe fn setBreakpoint0_(&self, breakpoint0: CGPoint)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setBreakpoint0 : breakpoint0)
    }
    unsafe fn breakpoint1(&self) -> CGPoint
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, breakpoint1)
    }
    unsafe fn setBreakpoint1_(&self, breakpoint1: CGPoint)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setBreakpoint1 : breakpoint1)
    }
    unsafe fn growAmount(&self) -> CGPoint
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, growAmount)
    }
    unsafe fn setGrowAmount_(&self, growAmount: CGPoint)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setGrowAmount : growAmount)
    }
    unsafe fn flipYTiles(&self) -> bool
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, flipYTiles)
    }
    unsafe fn setFlipYTiles_(&self, flipYTiles: bool)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setFlipYTiles : flipYTiles)
    }
}
pub trait PCIPinchDistortion: Sized + std::ops::Deref {
    unsafe fn inputImage(&self) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inputImage)
    }
    unsafe fn setInputImage_(&self, inputImage: CIImage)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInputImage : inputImage)
    }
    unsafe fn center(&self) -> CGPoint
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, center)
    }
    unsafe fn setCenter_(&self, center: CGPoint)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCenter : center)
    }
    unsafe fn radius(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, radius)
    }
    unsafe fn setRadius_(&self, radius: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRadius : radius)
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
}
pub trait PCIStretchCrop: Sized + std::ops::Deref {
    unsafe fn inputImage(&self) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inputImage)
    }
    unsafe fn setInputImage_(&self, inputImage: CIImage)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInputImage : inputImage)
    }
    unsafe fn size(&self) -> CGPoint
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, size)
    }
    unsafe fn setSize_(&self, size: CGPoint)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSize : size)
    }
    unsafe fn cropAmount(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, cropAmount)
    }
    unsafe fn setCropAmount_(&self, cropAmount: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCropAmount : cropAmount)
    }
    unsafe fn centerStretchAmount(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, centerStretchAmount)
    }
    unsafe fn setCenterStretchAmount_(&self, centerStretchAmount: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCenterStretchAmount : centerStretchAmount)
    }
}
pub trait PCITorusLensDistortion: Sized + std::ops::Deref {
    unsafe fn inputImage(&self) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inputImage)
    }
    unsafe fn setInputImage_(&self, inputImage: CIImage)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInputImage : inputImage)
    }
    unsafe fn center(&self) -> CGPoint
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, center)
    }
    unsafe fn setCenter_(&self, center: CGPoint)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCenter : center)
    }
    unsafe fn radius(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, radius)
    }
    unsafe fn setRadius_(&self, radius: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRadius : radius)
    }
    unsafe fn width(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, width)
    }
    unsafe fn setWidth_(&self, width: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setWidth : width)
    }
    unsafe fn refraction(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, refraction)
    }
    unsafe fn setRefraction_(&self, refraction: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRefraction : refraction)
    }
}
pub trait PCITwirlDistortion: Sized + std::ops::Deref {
    unsafe fn inputImage(&self) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inputImage)
    }
    unsafe fn setInputImage_(&self, inputImage: CIImage)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInputImage : inputImage)
    }
    unsafe fn center(&self) -> CGPoint
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, center)
    }
    unsafe fn setCenter_(&self, center: CGPoint)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCenter : center)
    }
    unsafe fn radius(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, radius)
    }
    unsafe fn setRadius_(&self, radius: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRadius : radius)
    }
    unsafe fn angle(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, angle)
    }
    unsafe fn setAngle_(&self, angle: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAngle : angle)
    }
}
pub trait PCIVortexDistortion: Sized + std::ops::Deref {
    unsafe fn inputImage(&self) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inputImage)
    }
    unsafe fn setInputImage_(&self, inputImage: CIImage)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInputImage : inputImage)
    }
    unsafe fn center(&self) -> CGPoint
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, center)
    }
    unsafe fn setCenter_(&self, center: CGPoint)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCenter : center)
    }
    unsafe fn radius(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, radius)
    }
    unsafe fn setRadius_(&self, radius: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRadius : radius)
    }
    unsafe fn angle(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, angle)
    }
    unsafe fn setAngle_(&self, angle: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAngle : angle)
    }
}
pub trait PCIAffineClamp: Sized + std::ops::Deref {
    unsafe fn inputImage(&self) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inputImage)
    }
    unsafe fn setInputImage_(&self, inputImage: CIImage)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInputImage : inputImage)
    }
    unsafe fn transform(&self) -> CGAffineTransform
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, transform)
    }
    unsafe fn setTransform_(&self, transform: CGAffineTransform)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setTransform : transform)
    }
}
pub trait PCIAffineTile: Sized + std::ops::Deref {
    unsafe fn inputImage(&self) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inputImage)
    }
    unsafe fn setInputImage_(&self, inputImage: CIImage)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInputImage : inputImage)
    }
    unsafe fn transform(&self) -> CGAffineTransform
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, transform)
    }
    unsafe fn setTransform_(&self, transform: CGAffineTransform)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setTransform : transform)
    }
}
pub trait PCIEightfoldReflectedTile: Sized + std::ops::Deref {
    unsafe fn inputImage(&self) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inputImage)
    }
    unsafe fn setInputImage_(&self, inputImage: CIImage)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInputImage : inputImage)
    }
    unsafe fn center(&self) -> CGPoint
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, center)
    }
    unsafe fn setCenter_(&self, center: CGPoint)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCenter : center)
    }
    unsafe fn angle(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, angle)
    }
    unsafe fn setAngle_(&self, angle: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAngle : angle)
    }
    unsafe fn width(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, width)
    }
    unsafe fn setWidth_(&self, width: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setWidth : width)
    }
}
pub trait PCIFourfoldReflectedTile: Sized + std::ops::Deref {
    unsafe fn inputImage(&self) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inputImage)
    }
    unsafe fn setInputImage_(&self, inputImage: CIImage)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInputImage : inputImage)
    }
    unsafe fn center(&self) -> CGPoint
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, center)
    }
    unsafe fn setCenter_(&self, center: CGPoint)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCenter : center)
    }
    unsafe fn angle(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, angle)
    }
    unsafe fn setAngle_(&self, angle: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAngle : angle)
    }
    unsafe fn width(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, width)
    }
    unsafe fn setWidth_(&self, width: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setWidth : width)
    }
    unsafe fn acuteAngle(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, acuteAngle)
    }
    unsafe fn setAcuteAngle_(&self, acuteAngle: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAcuteAngle : acuteAngle)
    }
}
pub trait PCIFourfoldRotatedTile: Sized + std::ops::Deref {
    unsafe fn inputImage(&self) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inputImage)
    }
    unsafe fn setInputImage_(&self, inputImage: CIImage)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInputImage : inputImage)
    }
    unsafe fn center(&self) -> CGPoint
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, center)
    }
    unsafe fn setCenter_(&self, center: CGPoint)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCenter : center)
    }
    unsafe fn angle(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, angle)
    }
    unsafe fn setAngle_(&self, angle: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAngle : angle)
    }
    unsafe fn width(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, width)
    }
    unsafe fn setWidth_(&self, width: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setWidth : width)
    }
}
pub trait PCIFourfoldTranslatedTile: Sized + std::ops::Deref {
    unsafe fn inputImage(&self) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inputImage)
    }
    unsafe fn setInputImage_(&self, inputImage: CIImage)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInputImage : inputImage)
    }
    unsafe fn center(&self) -> CGPoint
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, center)
    }
    unsafe fn setCenter_(&self, center: CGPoint)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCenter : center)
    }
    unsafe fn angle(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, angle)
    }
    unsafe fn setAngle_(&self, angle: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAngle : angle)
    }
    unsafe fn width(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, width)
    }
    unsafe fn setWidth_(&self, width: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setWidth : width)
    }
    unsafe fn acuteAngle(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, acuteAngle)
    }
    unsafe fn setAcuteAngle_(&self, acuteAngle: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAcuteAngle : acuteAngle)
    }
}
pub trait PCIGlideReflectedTile: Sized + std::ops::Deref {
    unsafe fn inputImage(&self) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inputImage)
    }
    unsafe fn setInputImage_(&self, inputImage: CIImage)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInputImage : inputImage)
    }
    unsafe fn center(&self) -> CGPoint
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, center)
    }
    unsafe fn setCenter_(&self, center: CGPoint)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCenter : center)
    }
    unsafe fn angle(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, angle)
    }
    unsafe fn setAngle_(&self, angle: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAngle : angle)
    }
    unsafe fn width(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, width)
    }
    unsafe fn setWidth_(&self, width: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setWidth : width)
    }
}
pub trait PCIKaleidoscope: Sized + std::ops::Deref {
    unsafe fn inputImage(&self) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inputImage)
    }
    unsafe fn setInputImage_(&self, inputImage: CIImage)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInputImage : inputImage)
    }
    unsafe fn count(&self) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, count)
    }
    unsafe fn setCount_(&self, count: NSInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCount : count)
    }
    unsafe fn center(&self) -> CGPoint
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, center)
    }
    unsafe fn setCenter_(&self, center: CGPoint)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCenter : center)
    }
    unsafe fn angle(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, angle)
    }
    unsafe fn setAngle_(&self, angle: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAngle : angle)
    }
}
pub trait PCIOpTile: Sized + std::ops::Deref {
    unsafe fn inputImage(&self) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inputImage)
    }
    unsafe fn setInputImage_(&self, inputImage: CIImage)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInputImage : inputImage)
    }
    unsafe fn center(&self) -> CGPoint
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, center)
    }
    unsafe fn setCenter_(&self, center: CGPoint)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCenter : center)
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
    unsafe fn angle(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, angle)
    }
    unsafe fn setAngle_(&self, angle: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAngle : angle)
    }
    unsafe fn width(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, width)
    }
    unsafe fn setWidth_(&self, width: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setWidth : width)
    }
}
pub trait PCIParallelogramTile: Sized + std::ops::Deref {
    unsafe fn inputImage(&self) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inputImage)
    }
    unsafe fn setInputImage_(&self, inputImage: CIImage)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInputImage : inputImage)
    }
    unsafe fn center(&self) -> CGPoint
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, center)
    }
    unsafe fn setCenter_(&self, center: CGPoint)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCenter : center)
    }
    unsafe fn angle(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, angle)
    }
    unsafe fn setAngle_(&self, angle: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAngle : angle)
    }
    unsafe fn acuteAngle(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, acuteAngle)
    }
    unsafe fn setAcuteAngle_(&self, acuteAngle: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAcuteAngle : acuteAngle)
    }
    unsafe fn width(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, width)
    }
    unsafe fn setWidth_(&self, width: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setWidth : width)
    }
}
pub trait PCIPerspectiveTile: Sized + std::ops::Deref {
    unsafe fn inputImage(&self) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inputImage)
    }
    unsafe fn setInputImage_(&self, inputImage: CIImage)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInputImage : inputImage)
    }
    unsafe fn topLeft(&self) -> CGPoint
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, topLeft)
    }
    unsafe fn setTopLeft_(&self, topLeft: CGPoint)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setTopLeft : topLeft)
    }
    unsafe fn topRight(&self) -> CGPoint
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, topRight)
    }
    unsafe fn setTopRight_(&self, topRight: CGPoint)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setTopRight : topRight)
    }
    unsafe fn bottomRight(&self) -> CGPoint
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, bottomRight)
    }
    unsafe fn setBottomRight_(&self, bottomRight: CGPoint)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setBottomRight : bottomRight)
    }
    unsafe fn bottomLeft(&self) -> CGPoint
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, bottomLeft)
    }
    unsafe fn setBottomLeft_(&self, bottomLeft: CGPoint)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setBottomLeft : bottomLeft)
    }
}
pub trait PCISixfoldReflectedTile: Sized + std::ops::Deref {
    unsafe fn inputImage(&self) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inputImage)
    }
    unsafe fn setInputImage_(&self, inputImage: CIImage)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInputImage : inputImage)
    }
    unsafe fn center(&self) -> CGPoint
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, center)
    }
    unsafe fn setCenter_(&self, center: CGPoint)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCenter : center)
    }
    unsafe fn angle(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, angle)
    }
    unsafe fn setAngle_(&self, angle: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAngle : angle)
    }
    unsafe fn width(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, width)
    }
    unsafe fn setWidth_(&self, width: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setWidth : width)
    }
}
pub trait PCISixfoldRotatedTile: Sized + std::ops::Deref {
    unsafe fn inputImage(&self) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inputImage)
    }
    unsafe fn setInputImage_(&self, inputImage: CIImage)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInputImage : inputImage)
    }
    unsafe fn center(&self) -> CGPoint
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, center)
    }
    unsafe fn setCenter_(&self, center: CGPoint)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCenter : center)
    }
    unsafe fn angle(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, angle)
    }
    unsafe fn setAngle_(&self, angle: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAngle : angle)
    }
    unsafe fn width(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, width)
    }
    unsafe fn setWidth_(&self, width: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setWidth : width)
    }
}
pub trait PCITriangleKaleidoscope: Sized + std::ops::Deref {
    unsafe fn inputImage(&self) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inputImage)
    }
    unsafe fn setInputImage_(&self, inputImage: CIImage)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInputImage : inputImage)
    }
    unsafe fn point(&self) -> CGPoint
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, point)
    }
    unsafe fn setPoint_(&self, point: CGPoint)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPoint : point)
    }
    unsafe fn size(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, size)
    }
    unsafe fn setSize_(&self, size: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSize : size)
    }
    unsafe fn rotation(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, rotation)
    }
    unsafe fn setRotation_(&self, rotation: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRotation : rotation)
    }
    unsafe fn decay(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, decay)
    }
    unsafe fn setDecay_(&self, decay: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDecay : decay)
    }
}
pub trait PCITriangleTile: Sized + std::ops::Deref {
    unsafe fn inputImage(&self) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inputImage)
    }
    unsafe fn setInputImage_(&self, inputImage: CIImage)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInputImage : inputImage)
    }
    unsafe fn center(&self) -> CGPoint
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, center)
    }
    unsafe fn setCenter_(&self, center: CGPoint)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCenter : center)
    }
    unsafe fn angle(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, angle)
    }
    unsafe fn setAngle_(&self, angle: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAngle : angle)
    }
    unsafe fn width(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, width)
    }
    unsafe fn setWidth_(&self, width: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setWidth : width)
    }
}
pub trait PCITwelvefoldReflectedTile: Sized + std::ops::Deref {
    unsafe fn inputImage(&self) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inputImage)
    }
    unsafe fn setInputImage_(&self, inputImage: CIImage)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInputImage : inputImage)
    }
    unsafe fn center(&self) -> CGPoint
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, center)
    }
    unsafe fn setCenter_(&self, center: CGPoint)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCenter : center)
    }
    unsafe fn angle(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, angle)
    }
    unsafe fn setAngle_(&self, angle: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAngle : angle)
    }
    unsafe fn width(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, width)
    }
    unsafe fn setWidth_(&self, width: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setWidth : width)
    }
}
pub trait PCIAttributedTextImageGenerator: Sized + std::ops::Deref {
    unsafe fn text(&self) -> NSAttributedString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, text)
    }
    unsafe fn setText_(&self, text: NSAttributedString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setText : text)
    }
    unsafe fn scaleFactor(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, scaleFactor)
    }
    unsafe fn setScaleFactor_(&self, scaleFactor: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setScaleFactor : scaleFactor)
    }
    unsafe fn padding(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, padding)
    }
    unsafe fn setPadding_(&self, padding: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPadding : padding)
    }
}
pub trait PCIAztecCodeGenerator: Sized + std::ops::Deref {
    unsafe fn message(&self) -> NSData
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, message)
    }
    unsafe fn setMessage_(&self, message: NSData)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMessage : message)
    }
    unsafe fn correctionLevel(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, correctionLevel)
    }
    unsafe fn setCorrectionLevel_(&self, correctionLevel: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCorrectionLevel : correctionLevel)
    }
    unsafe fn layers(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, layers)
    }
    unsafe fn setLayers_(&self, layers: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setLayers : layers)
    }
    unsafe fn compactStyle(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, compactStyle)
    }
    unsafe fn setCompactStyle_(&self, compactStyle: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCompactStyle : compactStyle)
    }
}
pub trait PCIBarcodeGenerator: Sized + std::ops::Deref {
    unsafe fn barcodeDescriptor(&self) -> CIBarcodeDescriptor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, barcodeDescriptor)
    }
    unsafe fn setBarcodeDescriptor_(&self, barcodeDescriptor: CIBarcodeDescriptor)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setBarcodeDescriptor : barcodeDescriptor)
    }
}
pub trait PCIBlurredRectangleGenerator: Sized + std::ops::Deref {
    unsafe fn extent(&self) -> CGRect
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, extent)
    }
    unsafe fn setExtent_(&self, extent: CGRect)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setExtent : extent)
    }
    unsafe fn sigma(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, sigma)
    }
    unsafe fn setSigma_(&self, sigma: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSigma : sigma)
    }
    unsafe fn color(&self) -> CIColor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, color)
    }
    unsafe fn setColor_(&self, color: CIColor)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setColor : color)
    }
}
pub trait PCIBlurredRoundedRectangleGenerator: Sized + std::ops::Deref {
    unsafe fn extent(&self) -> CGRect
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, extent)
    }
    unsafe fn setExtent_(&self, extent: CGRect)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setExtent : extent)
    }
    unsafe fn radius(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, radius)
    }
    unsafe fn setRadius_(&self, radius: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRadius : radius)
    }
    unsafe fn smoothness(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, smoothness)
    }
    unsafe fn setSmoothness_(&self, smoothness: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSmoothness : smoothness)
    }
    unsafe fn sigma(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, sigma)
    }
    unsafe fn setSigma_(&self, sigma: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSigma : sigma)
    }
    unsafe fn color(&self) -> CIColor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, color)
    }
    unsafe fn setColor_(&self, color: CIColor)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setColor : color)
    }
}
pub trait PCICheckerboardGenerator: Sized + std::ops::Deref {
    unsafe fn center(&self) -> CGPoint
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, center)
    }
    unsafe fn setCenter_(&self, center: CGPoint)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCenter : center)
    }
    unsafe fn color0(&self) -> CIColor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, color0)
    }
    unsafe fn setColor0_(&self, color0: CIColor)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setColor0 : color0)
    }
    unsafe fn color1(&self) -> CIColor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, color1)
    }
    unsafe fn setColor1_(&self, color1: CIColor)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setColor1 : color1)
    }
    unsafe fn width(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, width)
    }
    unsafe fn setWidth_(&self, width: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setWidth : width)
    }
    unsafe fn sharpness(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, sharpness)
    }
    unsafe fn setSharpness_(&self, sharpness: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSharpness : sharpness)
    }
}
pub trait PCICode128BarcodeGenerator: Sized + std::ops::Deref {
    unsafe fn message(&self) -> NSData
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, message)
    }
    unsafe fn setMessage_(&self, message: NSData)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMessage : message)
    }
    unsafe fn quietSpace(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, quietSpace)
    }
    unsafe fn setQuietSpace_(&self, quietSpace: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setQuietSpace : quietSpace)
    }
    unsafe fn barcodeHeight(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, barcodeHeight)
    }
    unsafe fn setBarcodeHeight_(&self, barcodeHeight: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setBarcodeHeight : barcodeHeight)
    }
}
pub trait PCILenticularHaloGenerator: Sized + std::ops::Deref {
    unsafe fn center(&self) -> CGPoint
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, center)
    }
    unsafe fn setCenter_(&self, center: CGPoint)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCenter : center)
    }
    unsafe fn color(&self) -> CIColor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, color)
    }
    unsafe fn setColor_(&self, color: CIColor)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setColor : color)
    }
    unsafe fn haloRadius(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, haloRadius)
    }
    unsafe fn setHaloRadius_(&self, haloRadius: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setHaloRadius : haloRadius)
    }
    unsafe fn haloWidth(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, haloWidth)
    }
    unsafe fn setHaloWidth_(&self, haloWidth: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setHaloWidth : haloWidth)
    }
    unsafe fn haloOverlap(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, haloOverlap)
    }
    unsafe fn setHaloOverlap_(&self, haloOverlap: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setHaloOverlap : haloOverlap)
    }
    unsafe fn striationStrength(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, striationStrength)
    }
    unsafe fn setStriationStrength_(&self, striationStrength: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setStriationStrength : striationStrength)
    }
    unsafe fn striationContrast(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, striationContrast)
    }
    unsafe fn setStriationContrast_(&self, striationContrast: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setStriationContrast : striationContrast)
    }
    unsafe fn time(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, time)
    }
    unsafe fn setTime_(&self, time: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setTime : time)
    }
}
pub trait PCIMeshGenerator: Sized + std::ops::Deref {
    unsafe fn width(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, width)
    }
    unsafe fn setWidth_(&self, width: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setWidth : width)
    }
    unsafe fn color(&self) -> CIColor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, color)
    }
    unsafe fn setColor_(&self, color: CIColor)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setColor : color)
    }
    unsafe fn mesh(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, mesh)
    }
    unsafe fn setMesh_(&self, mesh: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMesh : mesh)
    }
}
pub trait PCIPDF417BarcodeGenerator: Sized + std::ops::Deref {
    unsafe fn message(&self) -> NSData
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, message)
    }
    unsafe fn setMessage_(&self, message: NSData)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMessage : message)
    }
    unsafe fn minWidth(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, minWidth)
    }
    unsafe fn setMinWidth_(&self, minWidth: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMinWidth : minWidth)
    }
    unsafe fn maxWidth(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, maxWidth)
    }
    unsafe fn setMaxWidth_(&self, maxWidth: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMaxWidth : maxWidth)
    }
    unsafe fn minHeight(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, minHeight)
    }
    unsafe fn setMinHeight_(&self, minHeight: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMinHeight : minHeight)
    }
    unsafe fn maxHeight(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, maxHeight)
    }
    unsafe fn setMaxHeight_(&self, maxHeight: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMaxHeight : maxHeight)
    }
    unsafe fn dataColumns(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, dataColumns)
    }
    unsafe fn setDataColumns_(&self, dataColumns: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDataColumns : dataColumns)
    }
    unsafe fn rows(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, rows)
    }
    unsafe fn setRows_(&self, rows: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRows : rows)
    }
    unsafe fn preferredAspectRatio(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, preferredAspectRatio)
    }
    unsafe fn setPreferredAspectRatio_(&self, preferredAspectRatio: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPreferredAspectRatio : preferredAspectRatio)
    }
    unsafe fn compactionMode(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, compactionMode)
    }
    unsafe fn setCompactionMode_(&self, compactionMode: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCompactionMode : compactionMode)
    }
    unsafe fn compactStyle(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, compactStyle)
    }
    unsafe fn setCompactStyle_(&self, compactStyle: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCompactStyle : compactStyle)
    }
    unsafe fn correctionLevel(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, correctionLevel)
    }
    unsafe fn setCorrectionLevel_(&self, correctionLevel: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCorrectionLevel : correctionLevel)
    }
    unsafe fn alwaysSpecifyCompaction(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, alwaysSpecifyCompaction)
    }
    unsafe fn setAlwaysSpecifyCompaction_(&self, alwaysSpecifyCompaction: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAlwaysSpecifyCompaction : alwaysSpecifyCompaction)
    }
}
pub trait PCIQRCodeGenerator: Sized + std::ops::Deref {
    unsafe fn message(&self) -> NSData
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, message)
    }
    unsafe fn setMessage_(&self, message: NSData)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMessage : message)
    }
    unsafe fn correctionLevel(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, correctionLevel)
    }
    unsafe fn setCorrectionLevel_(&self, correctionLevel: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCorrectionLevel : correctionLevel)
    }
}
pub trait PCIRandomGenerator: Sized + std::ops::Deref {}
pub trait PCIRoundedQRCodeGenerator: Sized + std::ops::Deref {
    unsafe fn message(&self) -> NSData
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, message)
    }
    unsafe fn setMessage_(&self, message: NSData)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMessage : message)
    }
    unsafe fn correctionLevel(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, correctionLevel)
    }
    unsafe fn setCorrectionLevel_(&self, correctionLevel: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCorrectionLevel : correctionLevel)
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
    unsafe fn roundedMarkers(&self) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, roundedMarkers)
    }
    unsafe fn setRoundedMarkers_(&self, roundedMarkers: NSInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRoundedMarkers : roundedMarkers)
    }
    unsafe fn roundedData(&self) -> bool
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, roundedData)
    }
    unsafe fn setRoundedData_(&self, roundedData: bool)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRoundedData : roundedData)
    }
    unsafe fn centerSpaceSize(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, centerSpaceSize)
    }
    unsafe fn setCenterSpaceSize_(&self, centerSpaceSize: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCenterSpaceSize : centerSpaceSize)
    }
    unsafe fn color0(&self) -> CIColor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, color0)
    }
    unsafe fn setColor0_(&self, color0: CIColor)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setColor0 : color0)
    }
    unsafe fn color1(&self) -> CIColor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, color1)
    }
    unsafe fn setColor1_(&self, color1: CIColor)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setColor1 : color1)
    }
}
pub trait PCIRoundedRectangleGenerator: Sized + std::ops::Deref {
    unsafe fn extent(&self) -> CGRect
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, extent)
    }
    unsafe fn setExtent_(&self, extent: CGRect)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setExtent : extent)
    }
    unsafe fn radius(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, radius)
    }
    unsafe fn setRadius_(&self, radius: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRadius : radius)
    }
    unsafe fn smoothness(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, smoothness)
    }
    unsafe fn setSmoothness_(&self, smoothness: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSmoothness : smoothness)
    }
    unsafe fn color(&self) -> CIColor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, color)
    }
    unsafe fn setColor_(&self, color: CIColor)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setColor : color)
    }
}
pub trait PCIRoundedRectangleStrokeGenerator: Sized + std::ops::Deref {
    unsafe fn extent(&self) -> CGRect
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, extent)
    }
    unsafe fn setExtent_(&self, extent: CGRect)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setExtent : extent)
    }
    unsafe fn radius(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, radius)
    }
    unsafe fn setRadius_(&self, radius: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRadius : radius)
    }
    unsafe fn smoothness(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, smoothness)
    }
    unsafe fn setSmoothness_(&self, smoothness: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSmoothness : smoothness)
    }
    unsafe fn color(&self) -> CIColor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, color)
    }
    unsafe fn setColor_(&self, color: CIColor)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setColor : color)
    }
    unsafe fn width(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, width)
    }
    unsafe fn setWidth_(&self, width: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setWidth : width)
    }
}
pub trait PCIStarShineGenerator: Sized + std::ops::Deref {
    unsafe fn center(&self) -> CGPoint
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, center)
    }
    unsafe fn setCenter_(&self, center: CGPoint)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCenter : center)
    }
    unsafe fn color(&self) -> CIColor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, color)
    }
    unsafe fn setColor_(&self, color: CIColor)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setColor : color)
    }
    unsafe fn radius(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, radius)
    }
    unsafe fn setRadius_(&self, radius: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRadius : radius)
    }
    unsafe fn crossScale(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, crossScale)
    }
    unsafe fn setCrossScale_(&self, crossScale: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCrossScale : crossScale)
    }
    unsafe fn crossAngle(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, crossAngle)
    }
    unsafe fn setCrossAngle_(&self, crossAngle: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCrossAngle : crossAngle)
    }
    unsafe fn crossOpacity(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, crossOpacity)
    }
    unsafe fn setCrossOpacity_(&self, crossOpacity: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCrossOpacity : crossOpacity)
    }
    unsafe fn crossWidth(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, crossWidth)
    }
    unsafe fn setCrossWidth_(&self, crossWidth: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCrossWidth : crossWidth)
    }
    unsafe fn epsilon(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, epsilon)
    }
    unsafe fn setEpsilon_(&self, epsilon: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setEpsilon : epsilon)
    }
}
pub trait PCIStripesGenerator: Sized + std::ops::Deref {
    unsafe fn center(&self) -> CGPoint
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, center)
    }
    unsafe fn setCenter_(&self, center: CGPoint)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCenter : center)
    }
    unsafe fn color0(&self) -> CIColor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, color0)
    }
    unsafe fn setColor0_(&self, color0: CIColor)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setColor0 : color0)
    }
    unsafe fn color1(&self) -> CIColor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, color1)
    }
    unsafe fn setColor1_(&self, color1: CIColor)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setColor1 : color1)
    }
    unsafe fn width(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, width)
    }
    unsafe fn setWidth_(&self, width: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setWidth : width)
    }
    unsafe fn sharpness(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, sharpness)
    }
    unsafe fn setSharpness_(&self, sharpness: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSharpness : sharpness)
    }
}
pub trait PCISunbeamsGenerator: Sized + std::ops::Deref {
    unsafe fn center(&self) -> CGPoint
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, center)
    }
    unsafe fn setCenter_(&self, center: CGPoint)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCenter : center)
    }
    unsafe fn color(&self) -> CIColor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, color)
    }
    unsafe fn setColor_(&self, color: CIColor)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setColor : color)
    }
    unsafe fn sunRadius(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, sunRadius)
    }
    unsafe fn setSunRadius_(&self, sunRadius: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSunRadius : sunRadius)
    }
    unsafe fn maxStriationRadius(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, maxStriationRadius)
    }
    unsafe fn setMaxStriationRadius_(&self, maxStriationRadius: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMaxStriationRadius : maxStriationRadius)
    }
    unsafe fn striationStrength(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, striationStrength)
    }
    unsafe fn setStriationStrength_(&self, striationStrength: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setStriationStrength : striationStrength)
    }
    unsafe fn striationContrast(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, striationContrast)
    }
    unsafe fn setStriationContrast_(&self, striationContrast: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setStriationContrast : striationContrast)
    }
    unsafe fn time(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, time)
    }
    unsafe fn setTime_(&self, time: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setTime : time)
    }
}
pub trait PCITextImageGenerator: Sized + std::ops::Deref {
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
    unsafe fn fontName(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, fontName)
    }
    unsafe fn setFontName_(&self, fontName: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setFontName : fontName)
    }
    unsafe fn fontSize(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, fontSize)
    }
    unsafe fn setFontSize_(&self, fontSize: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setFontSize : fontSize)
    }
    unsafe fn scaleFactor(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, scaleFactor)
    }
    unsafe fn setScaleFactor_(&self, scaleFactor: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setScaleFactor : scaleFactor)
    }
    unsafe fn padding(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, padding)
    }
    unsafe fn setPadding_(&self, padding: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPadding : padding)
    }
}
pub trait PCIBlendWithMask: Sized + std::ops::Deref {
    unsafe fn inputImage(&self) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inputImage)
    }
    unsafe fn setInputImage_(&self, inputImage: CIImage)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInputImage : inputImage)
    }
    unsafe fn backgroundImage(&self) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, backgroundImage)
    }
    unsafe fn setBackgroundImage_(&self, backgroundImage: CIImage)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setBackgroundImage : backgroundImage)
    }
    unsafe fn maskImage(&self) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, maskImage)
    }
    unsafe fn setMaskImage_(&self, maskImage: CIImage)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMaskImage : maskImage)
    }
}
pub trait PCIBloom: Sized + std::ops::Deref {
    unsafe fn inputImage(&self) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inputImage)
    }
    unsafe fn setInputImage_(&self, inputImage: CIImage)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInputImage : inputImage)
    }
    unsafe fn radius(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, radius)
    }
    unsafe fn setRadius_(&self, radius: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRadius : radius)
    }
    unsafe fn intensity(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, intensity)
    }
    unsafe fn setIntensity_(&self, intensity: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setIntensity : intensity)
    }
}
pub trait PCICannyEdgeDetector: Sized + std::ops::Deref {
    unsafe fn inputImage(&self) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inputImage)
    }
    unsafe fn setInputImage_(&self, inputImage: CIImage)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInputImage : inputImage)
    }
    unsafe fn gaussianSigma(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, gaussianSigma)
    }
    unsafe fn setGaussianSigma_(&self, gaussianSigma: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setGaussianSigma : gaussianSigma)
    }
    unsafe fn perceptual(&self) -> bool
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, perceptual)
    }
    unsafe fn setPerceptual_(&self, perceptual: bool)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPerceptual : perceptual)
    }
    unsafe fn thresholdHigh(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, thresholdHigh)
    }
    unsafe fn setThresholdHigh_(&self, thresholdHigh: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setThresholdHigh : thresholdHigh)
    }
    unsafe fn thresholdLow(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, thresholdLow)
    }
    unsafe fn setThresholdLow_(&self, thresholdLow: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setThresholdLow : thresholdLow)
    }
    unsafe fn hysteresisPasses(&self) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, hysteresisPasses)
    }
    unsafe fn setHysteresisPasses_(&self, hysteresisPasses: NSInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setHysteresisPasses : hysteresisPasses)
    }
}
pub trait PCIComicEffect: Sized + std::ops::Deref {
    unsafe fn inputImage(&self) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inputImage)
    }
    unsafe fn setInputImage_(&self, inputImage: CIImage)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInputImage : inputImage)
    }
}
pub trait PCIConvolution: Sized + std::ops::Deref {
    unsafe fn inputImage(&self) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inputImage)
    }
    unsafe fn setInputImage_(&self, inputImage: CIImage)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInputImage : inputImage)
    }
    unsafe fn weights(&self) -> CIVector
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, weights)
    }
    unsafe fn setWeights_(&self, weights: CIVector)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setWeights : weights)
    }
    unsafe fn bias(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, bias)
    }
    unsafe fn setBias_(&self, bias: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setBias : bias)
    }
}
pub trait PCICoreMLModel: Sized + std::ops::Deref {
    unsafe fn inputImage(&self) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inputImage)
    }
    unsafe fn setInputImage_(&self, inputImage: CIImage)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInputImage : inputImage)
    }
    unsafe fn model(&self) -> MLModel
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, model)
    }
    unsafe fn setModel_(&self, model: MLModel)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setModel : model)
    }
    unsafe fn headIndex(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, headIndex)
    }
    unsafe fn setHeadIndex_(&self, headIndex: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setHeadIndex : headIndex)
    }
    unsafe fn softmaxNormalization(&self) -> bool
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, softmaxNormalization)
    }
    unsafe fn setSoftmaxNormalization_(&self, softmaxNormalization: bool)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSoftmaxNormalization : softmaxNormalization)
    }
}
pub trait PCICrystallize: Sized + std::ops::Deref {
    unsafe fn inputImage(&self) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inputImage)
    }
    unsafe fn setInputImage_(&self, inputImage: CIImage)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInputImage : inputImage)
    }
    unsafe fn radius(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, radius)
    }
    unsafe fn setRadius_(&self, radius: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRadius : radius)
    }
    unsafe fn center(&self) -> CGPoint
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, center)
    }
    unsafe fn setCenter_(&self, center: CGPoint)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCenter : center)
    }
}
pub trait PCIDepthOfField: Sized + std::ops::Deref {
    unsafe fn inputImage(&self) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inputImage)
    }
    unsafe fn setInputImage_(&self, inputImage: CIImage)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInputImage : inputImage)
    }
    unsafe fn point0(&self) -> CGPoint
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, point0)
    }
    unsafe fn setPoint0_(&self, point0: CGPoint)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPoint0 : point0)
    }
    unsafe fn point1(&self) -> CGPoint
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, point1)
    }
    unsafe fn setPoint1_(&self, point1: CGPoint)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPoint1 : point1)
    }
    unsafe fn saturation(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, saturation)
    }
    unsafe fn setSaturation_(&self, saturation: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSaturation : saturation)
    }
    unsafe fn unsharpMaskRadius(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, unsharpMaskRadius)
    }
    unsafe fn setUnsharpMaskRadius_(&self, unsharpMaskRadius: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setUnsharpMaskRadius : unsharpMaskRadius)
    }
    unsafe fn unsharpMaskIntensity(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, unsharpMaskIntensity)
    }
    unsafe fn setUnsharpMaskIntensity_(&self, unsharpMaskIntensity: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setUnsharpMaskIntensity : unsharpMaskIntensity)
    }
    unsafe fn radius(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, radius)
    }
    unsafe fn setRadius_(&self, radius: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRadius : radius)
    }
}
pub trait PCIEdges: Sized + std::ops::Deref {
    unsafe fn inputImage(&self) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inputImage)
    }
    unsafe fn setInputImage_(&self, inputImage: CIImage)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInputImage : inputImage)
    }
    unsafe fn intensity(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, intensity)
    }
    unsafe fn setIntensity_(&self, intensity: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setIntensity : intensity)
    }
}
pub trait PCIEdgeWork: Sized + std::ops::Deref {
    unsafe fn inputImage(&self) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inputImage)
    }
    unsafe fn setInputImage_(&self, inputImage: CIImage)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInputImage : inputImage)
    }
    unsafe fn radius(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, radius)
    }
    unsafe fn setRadius_(&self, radius: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRadius : radius)
    }
}
pub trait PCIGaborGradients: Sized + std::ops::Deref {
    unsafe fn inputImage(&self) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inputImage)
    }
    unsafe fn setInputImage_(&self, inputImage: CIImage)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInputImage : inputImage)
    }
}
pub trait PCIGloom: Sized + std::ops::Deref {
    unsafe fn inputImage(&self) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inputImage)
    }
    unsafe fn setInputImage_(&self, inputImage: CIImage)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInputImage : inputImage)
    }
    unsafe fn radius(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, radius)
    }
    unsafe fn setRadius_(&self, radius: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRadius : radius)
    }
    unsafe fn intensity(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, intensity)
    }
    unsafe fn setIntensity_(&self, intensity: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setIntensity : intensity)
    }
}
pub trait PCIHeightFieldFromMask: Sized + std::ops::Deref {
    unsafe fn inputImage(&self) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inputImage)
    }
    unsafe fn setInputImage_(&self, inputImage: CIImage)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInputImage : inputImage)
    }
    unsafe fn radius(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, radius)
    }
    unsafe fn setRadius_(&self, radius: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRadius : radius)
    }
}
pub trait PCIHexagonalPixellate: Sized + std::ops::Deref {
    unsafe fn inputImage(&self) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inputImage)
    }
    unsafe fn setInputImage_(&self, inputImage: CIImage)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInputImage : inputImage)
    }
    unsafe fn center(&self) -> CGPoint
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, center)
    }
    unsafe fn setCenter_(&self, center: CGPoint)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCenter : center)
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
}
pub trait PCIHighlightShadowAdjust: Sized + std::ops::Deref {
    unsafe fn inputImage(&self) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inputImage)
    }
    unsafe fn setInputImage_(&self, inputImage: CIImage)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInputImage : inputImage)
    }
    unsafe fn radius(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, radius)
    }
    unsafe fn setRadius_(&self, radius: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRadius : radius)
    }
    unsafe fn shadowAmount(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, shadowAmount)
    }
    unsafe fn setShadowAmount_(&self, shadowAmount: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setShadowAmount : shadowAmount)
    }
    unsafe fn highlightAmount(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, highlightAmount)
    }
    unsafe fn setHighlightAmount_(&self, highlightAmount: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setHighlightAmount : highlightAmount)
    }
}
pub trait PCILineOverlay: Sized + std::ops::Deref {
    unsafe fn inputImage(&self) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inputImage)
    }
    unsafe fn setInputImage_(&self, inputImage: CIImage)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInputImage : inputImage)
    }
    unsafe fn NRNoiseLevel(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, NRNoiseLevel)
    }
    unsafe fn setNRNoiseLevel_(&self, NRNoiseLevel: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setNRNoiseLevel : NRNoiseLevel)
    }
    unsafe fn NRSharpness(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, NRSharpness)
    }
    unsafe fn setNRSharpness_(&self, NRSharpness: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setNRSharpness : NRSharpness)
    }
    unsafe fn edgeIntensity(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, edgeIntensity)
    }
    unsafe fn setEdgeIntensity_(&self, edgeIntensity: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setEdgeIntensity : edgeIntensity)
    }
    unsafe fn threshold(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, threshold)
    }
    unsafe fn setThreshold_(&self, threshold: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setThreshold : threshold)
    }
    unsafe fn contrast(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, contrast)
    }
    unsafe fn setContrast_(&self, contrast: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setContrast : contrast)
    }
}
pub trait PCIMix: Sized + std::ops::Deref {
    unsafe fn inputImage(&self) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inputImage)
    }
    unsafe fn setInputImage_(&self, inputImage: CIImage)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInputImage : inputImage)
    }
    unsafe fn backgroundImage(&self) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, backgroundImage)
    }
    unsafe fn setBackgroundImage_(&self, backgroundImage: CIImage)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setBackgroundImage : backgroundImage)
    }
    unsafe fn amount(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, amount)
    }
    unsafe fn setAmount_(&self, amount: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAmount : amount)
    }
}
pub trait PCIPersonSegmentation: Sized + std::ops::Deref {
    unsafe fn inputImage(&self) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inputImage)
    }
    unsafe fn setInputImage_(&self, inputImage: CIImage)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInputImage : inputImage)
    }
    unsafe fn qualityLevel(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, qualityLevel)
    }
    unsafe fn setQualityLevel_(&self, qualityLevel: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setQualityLevel : qualityLevel)
    }
}
pub trait PCIPixellate: Sized + std::ops::Deref {
    unsafe fn inputImage(&self) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inputImage)
    }
    unsafe fn setInputImage_(&self, inputImage: CIImage)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInputImage : inputImage)
    }
    unsafe fn center(&self) -> CGPoint
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, center)
    }
    unsafe fn setCenter_(&self, center: CGPoint)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCenter : center)
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
}
pub trait PCIPointillize: Sized + std::ops::Deref {
    unsafe fn inputImage(&self) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inputImage)
    }
    unsafe fn setInputImage_(&self, inputImage: CIImage)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInputImage : inputImage)
    }
    unsafe fn radius(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, radius)
    }
    unsafe fn setRadius_(&self, radius: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRadius : radius)
    }
    unsafe fn center(&self) -> CGPoint
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, center)
    }
    unsafe fn setCenter_(&self, center: CGPoint)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCenter : center)
    }
}
pub trait PCISaliencyMap: Sized + std::ops::Deref {
    unsafe fn inputImage(&self) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inputImage)
    }
    unsafe fn setInputImage_(&self, inputImage: CIImage)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInputImage : inputImage)
    }
}
pub trait PCIShadedMaterial: Sized + std::ops::Deref {
    unsafe fn inputImage(&self) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inputImage)
    }
    unsafe fn setInputImage_(&self, inputImage: CIImage)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInputImage : inputImage)
    }
    unsafe fn shadingImage(&self) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, shadingImage)
    }
    unsafe fn setShadingImage_(&self, shadingImage: CIImage)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setShadingImage : shadingImage)
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
}
pub trait PCISobelGradients: Sized + std::ops::Deref {
    unsafe fn inputImage(&self) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inputImage)
    }
    unsafe fn setInputImage_(&self, inputImage: CIImage)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInputImage : inputImage)
    }
}
pub trait PCISpotColor: Sized + std::ops::Deref {
    unsafe fn inputImage(&self) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inputImage)
    }
    unsafe fn setInputImage_(&self, inputImage: CIImage)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInputImage : inputImage)
    }
    unsafe fn centerColor1(&self) -> CIColor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, centerColor1)
    }
    unsafe fn setCenterColor1_(&self, centerColor1: CIColor)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCenterColor1 : centerColor1)
    }
    unsafe fn replacementColor1(&self) -> CIColor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, replacementColor1)
    }
    unsafe fn setReplacementColor1_(&self, replacementColor1: CIColor)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setReplacementColor1 : replacementColor1)
    }
    unsafe fn closeness1(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, closeness1)
    }
    unsafe fn setCloseness1_(&self, closeness1: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCloseness1 : closeness1)
    }
    unsafe fn contrast1(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, contrast1)
    }
    unsafe fn setContrast1_(&self, contrast1: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setContrast1 : contrast1)
    }
    unsafe fn centerColor2(&self) -> CIColor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, centerColor2)
    }
    unsafe fn setCenterColor2_(&self, centerColor2: CIColor)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCenterColor2 : centerColor2)
    }
    unsafe fn replacementColor2(&self) -> CIColor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, replacementColor2)
    }
    unsafe fn setReplacementColor2_(&self, replacementColor2: CIColor)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setReplacementColor2 : replacementColor2)
    }
    unsafe fn closeness2(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, closeness2)
    }
    unsafe fn setCloseness2_(&self, closeness2: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCloseness2 : closeness2)
    }
    unsafe fn contrast2(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, contrast2)
    }
    unsafe fn setContrast2_(&self, contrast2: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setContrast2 : contrast2)
    }
    unsafe fn centerColor3(&self) -> CIColor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, centerColor3)
    }
    unsafe fn setCenterColor3_(&self, centerColor3: CIColor)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCenterColor3 : centerColor3)
    }
    unsafe fn replacementColor3(&self) -> CIColor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, replacementColor3)
    }
    unsafe fn setReplacementColor3_(&self, replacementColor3: CIColor)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setReplacementColor3 : replacementColor3)
    }
    unsafe fn closeness3(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, closeness3)
    }
    unsafe fn setCloseness3_(&self, closeness3: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCloseness3 : closeness3)
    }
    unsafe fn contrast3(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, contrast3)
    }
    unsafe fn setContrast3_(&self, contrast3: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setContrast3 : contrast3)
    }
}
pub trait PCISpotLight: Sized + std::ops::Deref {
    unsafe fn inputImage(&self) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inputImage)
    }
    unsafe fn setInputImage_(&self, inputImage: CIImage)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInputImage : inputImage)
    }
    unsafe fn lightPosition(&self) -> CIVector
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, lightPosition)
    }
    unsafe fn setLightPosition_(&self, lightPosition: CIVector)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setLightPosition : lightPosition)
    }
    unsafe fn lightPointsAt(&self) -> CIVector
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, lightPointsAt)
    }
    unsafe fn setLightPointsAt_(&self, lightPointsAt: CIVector)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setLightPointsAt : lightPointsAt)
    }
    unsafe fn brightness(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, brightness)
    }
    unsafe fn setBrightness_(&self, brightness: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setBrightness : brightness)
    }
    unsafe fn concentration(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, concentration)
    }
    unsafe fn setConcentration_(&self, concentration: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setConcentration : concentration)
    }
    unsafe fn color(&self) -> CIColor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, color)
    }
    unsafe fn setColor_(&self, color: CIColor)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setColor : color)
    }
}
pub trait PCIBokehBlur: Sized + std::ops::Deref {
    unsafe fn inputImage(&self) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inputImage)
    }
    unsafe fn setInputImage_(&self, inputImage: CIImage)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInputImage : inputImage)
    }
    unsafe fn radius(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, radius)
    }
    unsafe fn setRadius_(&self, radius: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRadius : radius)
    }
    unsafe fn ringAmount(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, ringAmount)
    }
    unsafe fn setRingAmount_(&self, ringAmount: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRingAmount : ringAmount)
    }
    unsafe fn ringSize(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, ringSize)
    }
    unsafe fn setRingSize_(&self, ringSize: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRingSize : ringSize)
    }
    unsafe fn softness(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, softness)
    }
    unsafe fn setSoftness_(&self, softness: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSoftness : softness)
    }
}
pub trait PCIBoxBlur: Sized + std::ops::Deref {
    unsafe fn inputImage(&self) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inputImage)
    }
    unsafe fn setInputImage_(&self, inputImage: CIImage)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInputImage : inputImage)
    }
    unsafe fn radius(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, radius)
    }
    unsafe fn setRadius_(&self, radius: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRadius : radius)
    }
}
pub trait PCIDiscBlur: Sized + std::ops::Deref {
    unsafe fn inputImage(&self) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inputImage)
    }
    unsafe fn setInputImage_(&self, inputImage: CIImage)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInputImage : inputImage)
    }
    unsafe fn radius(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, radius)
    }
    unsafe fn setRadius_(&self, radius: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRadius : radius)
    }
}
pub trait PCIGaussianBlur: Sized + std::ops::Deref {
    unsafe fn inputImage(&self) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inputImage)
    }
    unsafe fn setInputImage_(&self, inputImage: CIImage)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInputImage : inputImage)
    }
    unsafe fn radius(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, radius)
    }
    unsafe fn setRadius_(&self, radius: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRadius : radius)
    }
}
pub trait PCIMaskedVariableBlur: Sized + std::ops::Deref {
    unsafe fn inputImage(&self) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inputImage)
    }
    unsafe fn setInputImage_(&self, inputImage: CIImage)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInputImage : inputImage)
    }
    unsafe fn mask(&self) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, mask)
    }
    unsafe fn setMask_(&self, mask: CIImage)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMask : mask)
    }
    unsafe fn radius(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, radius)
    }
    unsafe fn setRadius_(&self, radius: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRadius : radius)
    }
}
pub trait PCIMedian: Sized + std::ops::Deref {
    unsafe fn inputImage(&self) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inputImage)
    }
    unsafe fn setInputImage_(&self, inputImage: CIImage)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInputImage : inputImage)
    }
}
pub trait PCIMorphologyGradient: Sized + std::ops::Deref {
    unsafe fn inputImage(&self) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inputImage)
    }
    unsafe fn setInputImage_(&self, inputImage: CIImage)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInputImage : inputImage)
    }
    unsafe fn radius(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, radius)
    }
    unsafe fn setRadius_(&self, radius: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRadius : radius)
    }
}
pub trait PCIMorphologyMaximum: Sized + std::ops::Deref {
    unsafe fn inputImage(&self) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inputImage)
    }
    unsafe fn setInputImage_(&self, inputImage: CIImage)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInputImage : inputImage)
    }
    unsafe fn radius(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, radius)
    }
    unsafe fn setRadius_(&self, radius: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRadius : radius)
    }
}
pub trait PCIMorphologyMinimum: Sized + std::ops::Deref {
    unsafe fn inputImage(&self) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inputImage)
    }
    unsafe fn setInputImage_(&self, inputImage: CIImage)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInputImage : inputImage)
    }
    unsafe fn radius(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, radius)
    }
    unsafe fn setRadius_(&self, radius: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRadius : radius)
    }
}
pub trait PCIMorphologyRectangleMaximum: Sized + std::ops::Deref {
    unsafe fn inputImage(&self) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inputImage)
    }
    unsafe fn setInputImage_(&self, inputImage: CIImage)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInputImage : inputImage)
    }
    unsafe fn width(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, width)
    }
    unsafe fn setWidth_(&self, width: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setWidth : width)
    }
    unsafe fn height(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, height)
    }
    unsafe fn setHeight_(&self, height: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setHeight : height)
    }
}
pub trait PCIMorphologyRectangleMinimum: Sized + std::ops::Deref {
    unsafe fn inputImage(&self) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inputImage)
    }
    unsafe fn setInputImage_(&self, inputImage: CIImage)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInputImage : inputImage)
    }
    unsafe fn width(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, width)
    }
    unsafe fn setWidth_(&self, width: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setWidth : width)
    }
    unsafe fn height(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, height)
    }
    unsafe fn setHeight_(&self, height: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setHeight : height)
    }
}
pub trait PCIMotionBlur: Sized + std::ops::Deref {
    unsafe fn inputImage(&self) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inputImage)
    }
    unsafe fn setInputImage_(&self, inputImage: CIImage)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInputImage : inputImage)
    }
    unsafe fn radius(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, radius)
    }
    unsafe fn setRadius_(&self, radius: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRadius : radius)
    }
    unsafe fn angle(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, angle)
    }
    unsafe fn setAngle_(&self, angle: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAngle : angle)
    }
}
pub trait PCINoiseReduction: Sized + std::ops::Deref {
    unsafe fn inputImage(&self) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inputImage)
    }
    unsafe fn setInputImage_(&self, inputImage: CIImage)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInputImage : inputImage)
    }
    unsafe fn noiseLevel(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, noiseLevel)
    }
    unsafe fn setNoiseLevel_(&self, noiseLevel: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setNoiseLevel : noiseLevel)
    }
    unsafe fn sharpness(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, sharpness)
    }
    unsafe fn setSharpness_(&self, sharpness: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSharpness : sharpness)
    }
}
pub trait PCIZoomBlur: Sized + std::ops::Deref {
    unsafe fn inputImage(&self) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inputImage)
    }
    unsafe fn setInputImage_(&self, inputImage: CIImage)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInputImage : inputImage)
    }
    unsafe fn center(&self) -> CGPoint
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, center)
    }
    unsafe fn setCenter_(&self, center: CGPoint)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCenter : center)
    }
    unsafe fn amount(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, amount)
    }
    unsafe fn setAmount_(&self, amount: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAmount : amount)
    }
}
pub trait PCIAreaReductionFilter: Sized + std::ops::Deref {
    unsafe fn inputImage(&self) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inputImage)
    }
    unsafe fn setInputImage_(&self, inputImage: CIImage)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInputImage : inputImage)
    }
    unsafe fn extent(&self) -> CGRect
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, extent)
    }
    unsafe fn setExtent_(&self, extent: CGRect)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setExtent : extent)
    }
}
pub trait PCIAreaAverage: Sized + std::ops::Deref {}
pub trait PCIAreaAverageMaximumRed: Sized + std::ops::Deref {}
pub trait PCIAreaBoundsRed: Sized + std::ops::Deref {}
pub trait PCIAreaHistogram: Sized + std::ops::Deref {
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
    unsafe fn count(&self) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, count)
    }
    unsafe fn setCount_(&self, count: NSInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCount : count)
    }
}
pub trait PCIAreaLogarithmicHistogram: Sized + std::ops::Deref {
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
    unsafe fn count(&self) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, count)
    }
    unsafe fn setCount_(&self, count: NSInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCount : count)
    }
    unsafe fn minimumStop(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, minimumStop)
    }
    unsafe fn setMinimumStop_(&self, minimumStop: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMinimumStop : minimumStop)
    }
    unsafe fn maximumStop(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, maximumStop)
    }
    unsafe fn setMaximumStop_(&self, maximumStop: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMaximumStop : maximumStop)
    }
}
pub trait PCIAreaMaximum: Sized + std::ops::Deref {}
pub trait PCIAreaMaximumAlpha: Sized + std::ops::Deref {}
pub trait PCIAreaMinimum: Sized + std::ops::Deref {}
pub trait PCIAreaMinimumAlpha: Sized + std::ops::Deref {}
pub trait PCIAreaMinMax: Sized + std::ops::Deref {}
pub trait PCIAreaMinMaxRed: Sized + std::ops::Deref {}
pub trait PCIColumnAverage: Sized + std::ops::Deref {}
pub trait PCIHistogramDisplay: Sized + std::ops::Deref {
    unsafe fn inputImage(&self) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inputImage)
    }
    unsafe fn setInputImage_(&self, inputImage: CIImage)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInputImage : inputImage)
    }
    unsafe fn height(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, height)
    }
    unsafe fn setHeight_(&self, height: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setHeight : height)
    }
    unsafe fn highLimit(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, highLimit)
    }
    unsafe fn setHighLimit_(&self, highLimit: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setHighLimit : highLimit)
    }
    unsafe fn lowLimit(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, lowLimit)
    }
    unsafe fn setLowLimit_(&self, lowLimit: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setLowLimit : lowLimit)
    }
}
pub trait PCIKMeans: Sized + std::ops::Deref {
    unsafe fn inputMeans(&self) -> CIImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inputMeans)
    }
    unsafe fn setInputMeans_(&self, inputMeans: CIImage)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInputMeans : inputMeans)
    }
    unsafe fn count(&self) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, count)
    }
    unsafe fn setCount_(&self, count: NSInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCount : count)
    }
    unsafe fn passes(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, passes)
    }
    unsafe fn setPasses_(&self, passes: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPasses : passes)
    }
    unsafe fn perceptual(&self) -> bool
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, perceptual)
    }
    unsafe fn setPerceptual_(&self, perceptual: bool)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPerceptual : perceptual)
    }
}
pub trait PCIRowAverage: Sized + std::ops::Deref {}
impl CIFilter_Builtins for CIFilter {}
unsafe extern "C" {
    pub static kCIFormatARGB8: CIFormat;
}
unsafe extern "C" {
    pub static kCIFormatBGRA8: CIFormat;
}
unsafe extern "C" {
    pub static kCIFormatRGBA8: CIFormat;
}
unsafe extern "C" {
    pub static kCIFormatRGBX8: CIFormat;
}
unsafe extern "C" {
    pub static kCIFormatABGR8: CIFormat;
}
unsafe extern "C" {
    pub static kCIFormatRGBAh: CIFormat;
}
unsafe extern "C" {
    pub static kCIFormatRGBA16: CIFormat;
}
unsafe extern "C" {
    pub static kCIFormatRGBAf: CIFormat;
}
unsafe extern "C" {
    pub static kCIFormatRGBX16: CIFormat;
}
unsafe extern "C" {
    pub static kCIFormatRGBXh: CIFormat;
}
unsafe extern "C" {
    pub static kCIFormatRGBXf: CIFormat;
}
unsafe extern "C" {
    pub static kCIFormatRGB10: CIFormat;
}
unsafe extern "C" {
    pub static kCIFormatA8: CIFormat;
}
unsafe extern "C" {
    pub static kCIFormatA16: CIFormat;
}
unsafe extern "C" {
    pub static kCIFormatAh: CIFormat;
}
unsafe extern "C" {
    pub static kCIFormatAf: CIFormat;
}
unsafe extern "C" {
    pub static kCIFormatR8: CIFormat;
}
unsafe extern "C" {
    pub static kCIFormatR16: CIFormat;
}
unsafe extern "C" {
    pub static kCIFormatRh: CIFormat;
}
unsafe extern "C" {
    pub static kCIFormatRf: CIFormat;
}
unsafe extern "C" {
    pub static kCIFormatRG8: CIFormat;
}
unsafe extern "C" {
    pub static kCIFormatRG16: CIFormat;
}
unsafe extern "C" {
    pub static kCIFormatRGh: CIFormat;
}
unsafe extern "C" {
    pub static kCIFormatRGf: CIFormat;
}
unsafe extern "C" {
    pub static kCIFormatL8: CIFormat;
}
unsafe extern "C" {
    pub static kCIFormatL16: CIFormat;
}
unsafe extern "C" {
    pub static kCIFormatLh: CIFormat;
}
unsafe extern "C" {
    pub static kCIFormatLf: CIFormat;
}
unsafe extern "C" {
    pub static kCIFormatLA8: CIFormat;
}
unsafe extern "C" {
    pub static kCIFormatLA16: CIFormat;
}
unsafe extern "C" {
    pub static kCIFormatLAh: CIFormat;
}
unsafe extern "C" {
    pub static kCIFormatLAf: CIFormat;
}
unsafe extern "C" {
    pub static kCIImageColorSpace: CIImageOption;
}
unsafe extern "C" {
    pub static kCIImageApplyCleanAperture: CIImageOption;
}
unsafe extern "C" {
    pub static kCIImageToneMapHDRtoSDR: CIImageOption;
}
unsafe extern "C" {
    pub static kCIImageExpandToHDR: CIImageOption;
}
unsafe extern "C" {
    pub static kCIImageContentHeadroom: CIImageOption;
}
unsafe extern "C" {
    pub static kCIImageContentAverageLightLevel: CIImageOption;
}
unsafe extern "C" {
    pub static kCIImageNearestSampling: CIImageOption;
}
unsafe extern "C" {
    pub static kCIImageCacheImmediately: CIImageOption;
}
unsafe extern "C" {
    pub static kCIImageProperties: CIImageOption;
}
unsafe extern "C" {
    pub static kCIImageApplyOrientationProperty: CIImageOption;
}
unsafe extern "C" {
    pub static kCIImageTextureTarget: CIImageOption;
}
unsafe extern "C" {
    pub static kCIImageTextureFormat: CIImageOption;
}
unsafe extern "C" {
    pub static kCIImageAuxiliaryDepth: CIImageOption;
}
unsafe extern "C" {
    pub static kCIImageAuxiliaryDisparity: CIImageOption;
}
unsafe extern "C" {
    pub static kCIImageAuxiliaryPortraitEffectsMatte: CIImageOption;
}
unsafe extern "C" {
    pub static kCIImageAuxiliarySemanticSegmentationSkinMatte: CIImageOption;
}
unsafe extern "C" {
    pub static kCIImageAuxiliarySemanticSegmentationHairMatte: CIImageOption;
}
unsafe extern "C" {
    pub static kCIImageAuxiliarySemanticSegmentationTeethMatte: CIImageOption;
}
unsafe extern "C" {
    pub static kCIImageAuxiliarySemanticSegmentationGlassesMatte: CIImageOption;
}
unsafe extern "C" {
    pub static kCIImageAuxiliarySemanticSegmentationSkyMatte: CIImageOption;
}
unsafe extern "C" {
    pub static kCIImageAuxiliaryHDRGainMap: CIImageOption;
}
unsafe extern "C" {
    pub static kCIImageAutoAdjustEnhance: CIImageAutoAdjustmentOption;
}
unsafe extern "C" {
    pub static kCIImageAutoAdjustRedEye: CIImageAutoAdjustmentOption;
}
unsafe extern "C" {
    pub static kCIImageAutoAdjustFeatures: CIImageAutoAdjustmentOption;
}
unsafe extern "C" {
    pub static kCIImageAutoAdjustCrop: CIImageAutoAdjustmentOption;
}
unsafe extern "C" {
    pub static kCIImageAutoAdjustLevel: CIImageAutoAdjustmentOption;
}
unsafe extern "C" {
    pub static kCIContextOutputColorSpace: CIContextOption;
}
unsafe extern "C" {
    pub static kCIContextWorkingColorSpace: CIContextOption;
}
unsafe extern "C" {
    pub static kCIContextWorkingFormat: CIContextOption;
}
unsafe extern "C" {
    pub static kCIContextHighQualityDownsample: CIContextOption;
}
unsafe extern "C" {
    pub static kCIContextOutputPremultiplied: CIContextOption;
}
unsafe extern "C" {
    pub static kCIContextCacheIntermediates: CIContextOption;
}
unsafe extern "C" {
    pub static kCIContextUseSoftwareRenderer: CIContextOption;
}
unsafe extern "C" {
    pub static kCIContextPriorityRequestLow: CIContextOption;
}
unsafe extern "C" {
    pub static kCIContextAllowLowPower: CIContextOption;
}
unsafe extern "C" {
    pub static kCIContextName: CIContextOption;
}
unsafe extern "C" {
    pub static kCIContextCVMetalTextureCache: CIContextOption;
}
unsafe extern "C" {
    pub static kCIContextMemoryLimit: CIContextOption;
}
unsafe extern "C" {
    pub static kCIImageRepresentationAVDepthData: CIImageRepresentationOption;
}
unsafe extern "C" {
    pub static kCIImageRepresentationDepthImage: CIImageRepresentationOption;
}
unsafe extern "C" {
    pub static kCIImageRepresentationDisparityImage: CIImageRepresentationOption;
}
unsafe extern "C" {
    pub static kCIImageRepresentationAVPortraitEffectsMatte: CIImageRepresentationOption;
}
unsafe extern "C" {
    pub static kCIImageRepresentationPortraitEffectsMatteImage: CIImageRepresentationOption;
}
unsafe extern "C" {
    pub static kCIImageRepresentationAVSemanticSegmentationMattes: CIImageRepresentationOption;
}
unsafe extern "C" {
    pub static kCIImageRepresentationSemanticSegmentationSkinMatteImage:
        CIImageRepresentationOption;
}
unsafe extern "C" {
    pub static kCIImageRepresentationSemanticSegmentationHairMatteImage:
        CIImageRepresentationOption;
}
unsafe extern "C" {
    pub static kCIImageRepresentationSemanticSegmentationTeethMatteImage:
        CIImageRepresentationOption;
}
unsafe extern "C" {
    pub static kCIImageRepresentationSemanticSegmentationGlassesMatteImage:
        CIImageRepresentationOption;
}
unsafe extern "C" {
    pub static kCIImageRepresentationSemanticSegmentationSkyMatteImage: CIImageRepresentationOption;
}
unsafe extern "C" {
    pub static kCIImageRepresentationHDRImage: CIImageRepresentationOption;
}
unsafe extern "C" {
    pub static kCIImageRepresentationHDRGainMapImage: CIImageRepresentationOption;
}
unsafe extern "C" {
    pub static kCIImageRepresentationHDRGainMapAsRGB: CIImageRepresentationOption;
}
unsafe extern "C" {
    pub static kCIAttributeFilterName: NSString;
}
unsafe extern "C" {
    pub static kCIAttributeFilterDisplayName: NSString;
}
unsafe extern "C" {
    pub static kCIAttributeDescription: NSString;
}
unsafe extern "C" {
    pub static kCIAttributeFilterAvailable_Mac: NSString;
}
unsafe extern "C" {
    pub static kCIAttributeFilterAvailable_iOS: NSString;
}
unsafe extern "C" {
    pub static kCIAttributeReferenceDocumentation: NSString;
}
unsafe extern "C" {
    pub static kCIAttributeFilterCategories: NSString;
}
unsafe extern "C" {
    pub static kCIAttributeClass: NSString;
}
unsafe extern "C" {
    pub static kCIAttributeType: NSString;
}
unsafe extern "C" {
    pub static kCIAttributeMin: NSString;
}
unsafe extern "C" {
    pub static kCIAttributeMax: NSString;
}
unsafe extern "C" {
    pub static kCIAttributeSliderMin: NSString;
}
unsafe extern "C" {
    pub static kCIAttributeSliderMax: NSString;
}
unsafe extern "C" {
    pub static kCIAttributeDefault: NSString;
}
unsafe extern "C" {
    pub static kCIAttributeIdentity: NSString;
}
unsafe extern "C" {
    pub static kCIAttributeName: NSString;
}
unsafe extern "C" {
    pub static kCIAttributeDisplayName: NSString;
}
unsafe extern "C" {
    pub static kCIUIParameterSet: NSString;
}
unsafe extern "C" {
    pub static kCIUISetBasic: NSString;
}
unsafe extern "C" {
    pub static kCIUISetIntermediate: NSString;
}
unsafe extern "C" {
    pub static kCIUISetAdvanced: NSString;
}
unsafe extern "C" {
    pub static kCIUISetDevelopment: NSString;
}
unsafe extern "C" {
    pub static kCIAttributeTypeTime: NSString;
}
unsafe extern "C" {
    pub static kCIAttributeTypeScalar: NSString;
}
unsafe extern "C" {
    pub static kCIAttributeTypeDistance: NSString;
}
unsafe extern "C" {
    pub static kCIAttributeTypeAngle: NSString;
}
unsafe extern "C" {
    pub static kCIAttributeTypeBoolean: NSString;
}
unsafe extern "C" {
    pub static kCIAttributeTypeInteger: NSString;
}
unsafe extern "C" {
    pub static kCIAttributeTypeCount: NSString;
}
unsafe extern "C" {
    pub static kCIAttributeTypePosition: NSString;
}
unsafe extern "C" {
    pub static kCIAttributeTypeOffset: NSString;
}
unsafe extern "C" {
    pub static kCIAttributeTypePosition3: NSString;
}
unsafe extern "C" {
    pub static kCIAttributeTypeRectangle: NSString;
}
unsafe extern "C" {
    pub static kCIAttributeTypeOpaqueColor: NSString;
}
unsafe extern "C" {
    pub static kCIAttributeTypeColor: NSString;
}
unsafe extern "C" {
    pub static kCIAttributeTypeGradient: NSString;
}
unsafe extern "C" {
    pub static kCIAttributeTypeImage: NSString;
}
unsafe extern "C" {
    pub static kCIAttributeTypeTransform: NSString;
}
unsafe extern "C" {
    pub static kCICategoryDistortionEffect: NSString;
}
unsafe extern "C" {
    pub static kCICategoryGeometryAdjustment: NSString;
}
unsafe extern "C" {
    pub static kCICategoryCompositeOperation: NSString;
}
unsafe extern "C" {
    pub static kCICategoryHalftoneEffect: NSString;
}
unsafe extern "C" {
    pub static kCICategoryColorAdjustment: NSString;
}
unsafe extern "C" {
    pub static kCICategoryColorEffect: NSString;
}
unsafe extern "C" {
    pub static kCICategoryTransition: NSString;
}
unsafe extern "C" {
    pub static kCICategoryTileEffect: NSString;
}
unsafe extern "C" {
    pub static kCICategoryGenerator: NSString;
}
unsafe extern "C" {
    pub static kCICategoryReduction: NSString;
}
unsafe extern "C" {
    pub static kCICategoryGradient: NSString;
}
unsafe extern "C" {
    pub static kCICategoryStylize: NSString;
}
unsafe extern "C" {
    pub static kCICategorySharpen: NSString;
}
unsafe extern "C" {
    pub static kCICategoryBlur: NSString;
}
unsafe extern "C" {
    pub static kCICategoryVideo: NSString;
}
unsafe extern "C" {
    pub static kCICategoryStillImage: NSString;
}
unsafe extern "C" {
    pub static kCICategoryInterlaced: NSString;
}
unsafe extern "C" {
    pub static kCICategoryNonSquarePixels: NSString;
}
unsafe extern "C" {
    pub static kCICategoryHighDynamicRange: NSString;
}
unsafe extern "C" {
    pub static kCICategoryBuiltIn: NSString;
}
unsafe extern "C" {
    pub static kCICategoryFilterGenerator: NSString;
}
unsafe extern "C" {
    pub static kCIApplyOptionExtent: NSString;
}
unsafe extern "C" {
    pub static kCIApplyOptionDefinition: NSString;
}
unsafe extern "C" {
    pub static kCIApplyOptionUserInfo: NSString;
}
unsafe extern "C" {
    pub static kCIApplyOptionColorSpace: NSString;
}
unsafe extern "C" {
    pub static kCIOutputImageKey: NSString;
}
unsafe extern "C" {
    pub static kCIInputBackgroundImageKey: NSString;
}
unsafe extern "C" {
    pub static kCIInputImageKey: NSString;
}
unsafe extern "C" {
    pub static kCIInputDepthImageKey: NSString;
}
unsafe extern "C" {
    pub static kCIInputDisparityImageKey: NSString;
}
unsafe extern "C" {
    pub static kCIInputAmountKey: NSString;
}
unsafe extern "C" {
    pub static kCIInputCountKey: NSString;
}
unsafe extern "C" {
    pub static kCIInputThresholdKey: NSString;
}
unsafe extern "C" {
    pub static kCIInputTimeKey: NSString;
}
unsafe extern "C" {
    pub static kCIInputTransformKey: NSString;
}
unsafe extern "C" {
    pub static kCIInputScaleKey: NSString;
}
unsafe extern "C" {
    pub static kCIInputAspectRatioKey: NSString;
}
unsafe extern "C" {
    pub static kCIInputCenterKey: NSString;
}
unsafe extern "C" {
    pub static kCIInputRadiusKey: NSString;
}
unsafe extern "C" {
    pub static kCIInputRadius0Key: NSString;
}
unsafe extern "C" {
    pub static kCIInputRadius1Key: NSString;
}
unsafe extern "C" {
    pub static kCIInputAngleKey: NSString;
}
unsafe extern "C" {
    pub static kCIInputRefractionKey: NSString;
}
unsafe extern "C" {
    pub static kCIInputWidthKey: NSString;
}
unsafe extern "C" {
    pub static kCIInputSharpnessKey: NSString;
}
unsafe extern "C" {
    pub static kCIInputIntensityKey: NSString;
}
unsafe extern "C" {
    pub static kCIInputEVKey: NSString;
}
unsafe extern "C" {
    pub static kCIInputSaturationKey: NSString;
}
unsafe extern "C" {
    pub static kCIInputColorKey: NSString;
}
unsafe extern "C" {
    pub static kCIInputColor0Key: NSString;
}
unsafe extern "C" {
    pub static kCIInputColor1Key: NSString;
}
unsafe extern "C" {
    pub static kCIInputColorSpaceKey: NSString;
}
unsafe extern "C" {
    pub static kCIInputBrightnessKey: NSString;
}
unsafe extern "C" {
    pub static kCIInputContrastKey: NSString;
}
unsafe extern "C" {
    pub static kCIInputExtrapolateKey: NSString;
}
unsafe extern "C" {
    pub static kCIInputPerceptualKey: NSString;
}
unsafe extern "C" {
    pub static kCIInputBiasKey: NSString;
}
unsafe extern "C" {
    pub static kCIInputBiasVectorKey: NSString;
}
unsafe extern "C" {
    pub static kCIInputWeightsKey: NSString;
}
unsafe extern "C" {
    pub static kCIInputGradientImageKey: NSString;
}
unsafe extern "C" {
    pub static kCIInputMaskImageKey: NSString;
}
unsafe extern "C" {
    pub static kCIInputMatteImageKey: NSString;
}
unsafe extern "C" {
    pub static kCIInputShadingImageKey: NSString;
}
unsafe extern "C" {
    pub static kCIInputTargetImageKey: NSString;
}
unsafe extern "C" {
    pub static kCIInputBacksideImageKey: NSString;
}
unsafe extern "C" {
    pub static kCIInputPaletteImageKey: NSString;
}
unsafe extern "C" {
    pub static kCIInputExtentKey: NSString;
}
unsafe extern "C" {
    pub static kCIInputPoint0Key: NSString;
}
unsafe extern "C" {
    pub static kCIInputPoint1Key: NSString;
}
unsafe extern "C" {
    pub static kCIInputVersionKey: NSString;
}
unsafe extern "C" {
    pub static kCIDynamicRangeStandard: CIDynamicRangeOption;
}
unsafe extern "C" {
    pub static kCIDynamicRangeConstrainedHigh: CIDynamicRangeOption;
}
unsafe extern "C" {
    pub static kCIDynamicRangeHigh: CIDynamicRangeOption;
}
unsafe extern "C" {
    pub static CIDetectorTypeFace: NSString;
}
unsafe extern "C" {
    pub static CIDetectorTypeRectangle: NSString;
}
unsafe extern "C" {
    pub static CIDetectorTypeQRCode: NSString;
}
unsafe extern "C" {
    pub static CIDetectorTypeText: NSString;
}
unsafe extern "C" {
    pub static CIDetectorAccuracy: NSString;
}
unsafe extern "C" {
    pub static CIDetectorAccuracyLow: NSString;
}
unsafe extern "C" {
    pub static CIDetectorAccuracyHigh: NSString;
}
unsafe extern "C" {
    pub static CIDetectorTracking: NSString;
}
unsafe extern "C" {
    pub static CIDetectorMinFeatureSize: NSString;
}
unsafe extern "C" {
    pub static CIDetectorMaxFeatureCount: NSString;
}
unsafe extern "C" {
    pub static CIDetectorNumberOfAngles: NSString;
}
unsafe extern "C" {
    pub static CIDetectorImageOrientation: NSString;
}
unsafe extern "C" {
    pub static CIDetectorEyeBlink: NSString;
}
unsafe extern "C" {
    pub static CIDetectorSmile: NSString;
}
unsafe extern "C" {
    pub static CIDetectorFocalLength: NSString;
}
unsafe extern "C" {
    pub static CIDetectorAspectRatio: NSString;
}
unsafe extern "C" {
    pub static CIDetectorReturnSubFeatures: NSString;
}
unsafe extern "C" {
    pub static CIFeatureTypeFace: NSString;
}
unsafe extern "C" {
    pub static CIFeatureTypeRectangle: NSString;
}
unsafe extern "C" {
    pub static CIFeatureTypeQRCode: NSString;
}
unsafe extern "C" {
    pub static CIFeatureTypeText: NSString;
}
unsafe extern "C" {
    pub static kCIImageProviderTileSize: CIImageOption;
}
unsafe extern "C" {
    pub static kCIImageProviderUserInfo: CIImageOption;
}
unsafe extern "C" {
    pub static kCISamplerAffineMatrix: NSString;
}
unsafe extern "C" {
    pub static kCISamplerWrapMode: NSString;
}
unsafe extern "C" {
    pub static kCISamplerFilterMode: NSString;
}
unsafe extern "C" {
    pub static kCISamplerWrapBlack: NSString;
}
unsafe extern "C" {
    pub static kCISamplerWrapClamp: NSString;
}
unsafe extern "C" {
    pub static kCISamplerFilterNearest: NSString;
}
unsafe extern "C" {
    pub static kCISamplerFilterLinear: NSString;
}
unsafe extern "C" {
    pub static kCISamplerColorSpace: NSString;
}
unsafe extern "C" {
    pub static kCIInputAllowDraftModeKey: CIRAWFilterOption;
}
unsafe extern "C" {
    pub static kCIInputDecoderVersionKey: CIRAWFilterOption;
}
unsafe extern "C" {
    pub static kCISupportedDecoderVersionsKey: CIRAWFilterOption;
}
unsafe extern "C" {
    pub static kCIInputBaselineExposureKey: CIRAWFilterOption;
}
unsafe extern "C" {
    pub static kCIInputBoostKey: CIRAWFilterOption;
}
unsafe extern "C" {
    pub static kCIInputBoostShadowAmountKey: CIRAWFilterOption;
}
unsafe extern "C" {
    pub static kCIInputDisableGamutMapKey: CIRAWFilterOption;
}
unsafe extern "C" {
    pub static kCIInputNeutralChromaticityXKey: CIRAWFilterOption;
}
unsafe extern "C" {
    pub static kCIInputNeutralChromaticityYKey: CIRAWFilterOption;
}
unsafe extern "C" {
    pub static kCIInputNeutralTemperatureKey: CIRAWFilterOption;
}
unsafe extern "C" {
    pub static kCIInputNeutralTintKey: CIRAWFilterOption;
}
unsafe extern "C" {
    pub static kCIInputNeutralLocationKey: CIRAWFilterOption;
}
unsafe extern "C" {
    pub static kCIInputScaleFactorKey: CIRAWFilterOption;
}
unsafe extern "C" {
    pub static kCIInputIgnoreImageOrientationKey: CIRAWFilterOption;
}
unsafe extern "C" {
    pub static kCIInputImageOrientationKey: CIRAWFilterOption;
}
unsafe extern "C" {
    pub static kCIInputEnableSharpeningKey: CIRAWFilterOption;
}
unsafe extern "C" {
    pub static kCIInputEnableChromaticNoiseTrackingKey: CIRAWFilterOption;
}
unsafe extern "C" {
    pub static kCIInputNoiseReductionAmountKey: CIRAWFilterOption;
}
unsafe extern "C" {
    pub static kCIInputMoireAmountKey: CIRAWFilterOption;
}
unsafe extern "C" {
    pub static kCIInputEnableVendorLensCorrectionKey: CIRAWFilterOption;
}
unsafe extern "C" {
    pub static kCIInputLuminanceNoiseReductionAmountKey: CIRAWFilterOption;
}
unsafe extern "C" {
    pub static kCIInputColorNoiseReductionAmountKey: CIRAWFilterOption;
}
unsafe extern "C" {
    pub static kCIInputNoiseReductionSharpnessAmountKey: CIRAWFilterOption;
}
unsafe extern "C" {
    pub static kCIInputNoiseReductionContrastAmountKey: CIRAWFilterOption;
}
unsafe extern "C" {
    pub static kCIInputNoiseReductionDetailAmountKey: CIRAWFilterOption;
}
unsafe extern "C" {
    pub static kCIInputEnableEDRModeKey: CIRAWFilterOption;
}
unsafe extern "C" {
    pub static kCIInputLocalToneMapAmountKey: CIRAWFilterOption;
}
unsafe extern "C" {
    pub static kCIInputLinearSpaceFilter: CIRAWFilterOption;
}
unsafe extern "C" {
    pub static kCIOutputNativeSizeKey: CIRAWFilterOption;
}
unsafe extern "C" {
    pub static kCIActiveKeys: CIRAWFilterOption;
}
unsafe extern "C" {
    pub static kCIPropertiesKey: CIRAWFilterOption;
}
unsafe extern "C" {
    pub static CIRAWDecoderVersionNone: CIRAWDecoderVersion;
}
unsafe extern "C" {
    pub static CIRAWDecoderVersion9: CIRAWDecoderVersion;
}
unsafe extern "C" {
    pub static CIRAWDecoderVersion9DNG: CIRAWDecoderVersion;
}
unsafe extern "C" {
    pub static CIRAWDecoderVersion8: CIRAWDecoderVersion;
}
unsafe extern "C" {
    pub static CIRAWDecoderVersion8DNG: CIRAWDecoderVersion;
}
unsafe extern "C" {
    pub static CIRAWDecoderVersion7: CIRAWDecoderVersion;
}
unsafe extern "C" {
    pub static CIRAWDecoderVersion7DNG: CIRAWDecoderVersion;
}
unsafe extern "C" {
    pub static CIRAWDecoderVersion6: CIRAWDecoderVersion;
}
unsafe extern "C" {
    pub static CIRAWDecoderVersion6DNG: CIRAWDecoderVersion;
}
unsafe extern "C" {
    pub static kCIFilterGeneratorExportedKey: NSString;
}
unsafe extern "C" {
    pub static kCIFilterGeneratorExportedKeyTargetObject: NSString;
}
unsafe extern "C" {
    pub static kCIFilterGeneratorExportedKeyName: NSString;
}

unsafe impl objc2::encode::RefEncode for CIVector {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CIVector {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CIColor {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CIColor {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CIImage {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CIImage {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CIContext {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CIContext {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CIFilter {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CIFilter {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CIKernel {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CIKernel {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CIColorKernel {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CIColorKernel {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CIWarpKernel {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CIWarpKernel {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CIBlendKernel {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CIBlendKernel {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CIDetector {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CIDetector {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CIFeature {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CIFeature {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CIFaceFeature {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CIFaceFeature {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CIRectangleFeature {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CIRectangleFeature {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CIQRCodeFeature {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CIQRCodeFeature {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CITextFeature {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CITextFeature {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CIImageProcessorKernel {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CIImageProcessorKernel {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CIImageAccumulator {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CIImageAccumulator {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CIFilterShape {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CIFilterShape {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CISampler {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CISampler {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CIRAWFilter {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CIRAWFilter {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CIRenderDestination {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CIRenderDestination {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CIRenderInfo {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CIRenderInfo {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CIRenderTask {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CIRenderTask {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CIBarcodeDescriptor {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CIBarcodeDescriptor {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CIQRCodeDescriptor {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CIQRCodeDescriptor {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CIAztecCodeDescriptor {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CIAztecCodeDescriptor {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CIPDF417CodeDescriptor {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CIPDF417CodeDescriptor {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CIDataMatrixCodeDescriptor {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CIDataMatrixCodeDescriptor {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CIFilterGenerator {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CIFilterGenerator {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for CIPlugIn {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for CIPlugIn {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
