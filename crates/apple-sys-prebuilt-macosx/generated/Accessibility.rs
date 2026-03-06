#[allow(unused_imports)]
use crate::objc::*;
#[allow(unused_imports)]
use crate::CoreFoundation::*;
#[allow(unused_imports)]
use crate::CoreGraphics::*;
#[allow(unused_imports)]
use crate::Foundation::*;

#[allow(unused_imports)]
use objc2::msg_send;
pub trait PAXChart: Sized + std::ops::Deref {
    unsafe fn accessibilityChartDescriptor(&self) -> AXChartDescriptor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, accessibilityChartDescriptor)
    }
    unsafe fn setAccessibilityChartDescriptor_(
        &self,
        accessibilityChartDescriptor: AXChartDescriptor,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAccessibilityChartDescriptor : accessibilityChartDescriptor)
    }
}
pub trait PAXDataAxisDescriptor: Sized + std::ops::Deref {
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
    unsafe fn attributedTitle(&self) -> NSAttributedString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, attributedTitle)
    }
    unsafe fn setAttributedTitle_(&self, attributedTitle: NSAttributedString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAttributedTitle : attributedTitle)
    }
}
pub type AXNumericDataAxisDescriptorScale = NSInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct AXNumericDataAxisDescriptor(pub id);
impl std::ops::Deref for AXNumericDataAxisDescriptor {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for AXNumericDataAxisDescriptor {}
impl AXNumericDataAxisDescriptor {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"AXNumericDataAxisDescriptor").unwrap(), alloc) })
    }
}
impl PAXDataAxisDescriptor for AXNumericDataAxisDescriptor {}
impl INSObject for AXNumericDataAxisDescriptor {}
impl PNSObject for AXNumericDataAxisDescriptor {}
impl std::convert::TryFrom<NSObject> for AXNumericDataAxisDescriptor {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<AXNumericDataAxisDescriptor, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"AXNumericDataAxisDescriptor").unwrap()) };
        if is_kind_of {
            Ok(AXNumericDataAxisDescriptor(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to AXNumericDataAxisDescriptor")
        }
    }
}
impl IAXNumericDataAxisDescriptor for AXNumericDataAxisDescriptor {}
pub trait IAXNumericDataAxisDescriptor: Sized + std::ops::Deref {
    unsafe fn initWithTitle_lowerBound_upperBound_gridlinePositions_valueDescriptionProvider_(
        &self,
        title: NSString,
        lowerbound: f64,
        upperBound: f64,
        gridlinePositions: NSArray,
        valueDescriptionProvider: *mut ::std::os::raw::c_void,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithTitle : title, lowerBound : lowerbound, upperBound : upperBound, gridlinePositions : gridlinePositions, valueDescriptionProvider : valueDescriptionProvider)
    }
    unsafe fn initWithAttributedTitle_lowerBound_upperBound_gridlinePositions_valueDescriptionProvider_(
        &self,
        attributedTitle: NSAttributedString,
        lowerbound: f64,
        upperBound: f64,
        gridlinePositions: NSArray,
        valueDescriptionProvider: *mut ::std::os::raw::c_void,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithAttributedTitle : attributedTitle, lowerBound : lowerbound, upperBound : upperBound, gridlinePositions : gridlinePositions, valueDescriptionProvider : valueDescriptionProvider)
    }
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn scaleType(&self) -> AXNumericDataAxisDescriptorScale
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, scaleType)
    }
    unsafe fn setScaleType_(&self, scaleType: AXNumericDataAxisDescriptorScale)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setScaleType : scaleType)
    }
    unsafe fn lowerBound(&self) -> f64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, lowerBound)
    }
    unsafe fn setLowerBound_(&self, lowerBound: f64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setLowerBound : lowerBound)
    }
    unsafe fn upperBound(&self) -> f64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, upperBound)
    }
    unsafe fn setUpperBound_(&self, upperBound: f64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setUpperBound : upperBound)
    }
    unsafe fn valueDescriptionProvider(&self) -> *mut ::std::os::raw::c_void
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, valueDescriptionProvider)
    }
    unsafe fn setValueDescriptionProvider_(
        &self,
        valueDescriptionProvider: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setValueDescriptionProvider : valueDescriptionProvider)
    }
    unsafe fn gridlinePositions(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, gridlinePositions)
    }
    unsafe fn setGridlinePositions_(&self, gridlinePositions: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setGridlinePositions : gridlinePositions)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"AXNumericDataAxisDescriptor").unwrap(), new)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct AXCategoricalDataAxisDescriptor(pub id);
impl std::ops::Deref for AXCategoricalDataAxisDescriptor {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for AXCategoricalDataAxisDescriptor {}
impl AXCategoricalDataAxisDescriptor {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"AXCategoricalDataAxisDescriptor").unwrap(), alloc) })
    }
}
impl PAXDataAxisDescriptor for AXCategoricalDataAxisDescriptor {}
impl INSObject for AXCategoricalDataAxisDescriptor {}
impl PNSObject for AXCategoricalDataAxisDescriptor {}
impl std::convert::TryFrom<NSObject> for AXCategoricalDataAxisDescriptor {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<AXCategoricalDataAxisDescriptor, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"AXCategoricalDataAxisDescriptor").unwrap())
        };
        if is_kind_of {
            Ok(AXCategoricalDataAxisDescriptor(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to AXCategoricalDataAxisDescriptor")
        }
    }
}
impl IAXCategoricalDataAxisDescriptor for AXCategoricalDataAxisDescriptor {}
pub trait IAXCategoricalDataAxisDescriptor: Sized + std::ops::Deref {
    unsafe fn initWithTitle_categoryOrder_(
        &self,
        title: NSString,
        categoryOrder: NSArray,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithTitle : title, categoryOrder : categoryOrder)
    }
    unsafe fn initWithAttributedTitle_categoryOrder_(
        &self,
        attributedTitle: NSAttributedString,
        categoryOrder: NSArray,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithAttributedTitle : attributedTitle, categoryOrder : categoryOrder)
    }
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn categoryOrder(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, categoryOrder)
    }
    unsafe fn setCategoryOrder_(&self, categoryOrder: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCategoryOrder : categoryOrder)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"AXCategoricalDataAxisDescriptor").unwrap(), new)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct AXDataPointValue(pub id);
impl std::ops::Deref for AXDataPointValue {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for AXDataPointValue {}
impl AXDataPointValue {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"AXDataPointValue").unwrap(), alloc) })
    }
}
impl PNSCopying for AXDataPointValue {}
impl INSObject for AXDataPointValue {}
impl PNSObject for AXDataPointValue {}
impl std::convert::TryFrom<NSObject> for AXDataPointValue {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<AXDataPointValue, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"AXDataPointValue").unwrap()) };
        if is_kind_of {
            Ok(AXDataPointValue(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to AXDataPointValue")
        }
    }
}
impl IAXDataPointValue for AXDataPointValue {}
pub trait IAXDataPointValue: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn number(&self) -> f64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, number)
    }
    unsafe fn setNumber_(&self, number: f64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setNumber : number)
    }
    unsafe fn category(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, category)
    }
    unsafe fn setCategory_(&self, category: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCategory : category)
    }
    unsafe fn valueWithNumber_(number: f64) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"AXDataPointValue").unwrap(), valueWithNumber : number)
    }
    unsafe fn valueWithCategory_(category: NSString) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"AXDataPointValue").unwrap(), valueWithCategory : category)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"AXDataPointValue").unwrap(), new)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct AXDataPoint(pub id);
impl std::ops::Deref for AXDataPoint {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for AXDataPoint {}
impl AXDataPoint {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"AXDataPoint").unwrap(), alloc) })
    }
}
impl PNSCopying for AXDataPoint {}
impl INSObject for AXDataPoint {}
impl PNSObject for AXDataPoint {}
impl std::convert::TryFrom<NSObject> for AXDataPoint {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<AXDataPoint, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"AXDataPoint").unwrap()) };
        if is_kind_of {
            Ok(AXDataPoint(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to AXDataPoint")
        }
    }
}
impl IAXDataPoint for AXDataPoint {}
pub trait IAXDataPoint: Sized + std::ops::Deref {
    unsafe fn initWithX_y_(
        &self,
        xValue: AXDataPointValue,
        yValue: AXDataPointValue,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithX : xValue, y : yValue)
    }
    unsafe fn initWithX_y_additionalValues_(
        &self,
        xValue: AXDataPointValue,
        yValue: AXDataPointValue,
        additionalValues: NSArray,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithX : xValue, y : yValue, additionalValues : additionalValues)
    }
    unsafe fn initWithX_y_additionalValues_label_(
        &self,
        xValue: AXDataPointValue,
        yValue: AXDataPointValue,
        additionalValues: NSArray,
        label: NSString,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithX : xValue, y : yValue, additionalValues : additionalValues, label : label)
    }
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn xValue(&self) -> AXDataPointValue
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, xValue)
    }
    unsafe fn setXValue_(&self, xValue: AXDataPointValue)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setXValue : xValue)
    }
    unsafe fn yValue(&self) -> AXDataPointValue
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, yValue)
    }
    unsafe fn setYValue_(&self, yValue: AXDataPointValue)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setYValue : yValue)
    }
    unsafe fn additionalValues(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, additionalValues)
    }
    unsafe fn setAdditionalValues_(&self, additionalValues: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAdditionalValues : additionalValues)
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
    unsafe fn attributedLabel(&self) -> NSAttributedString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, attributedLabel)
    }
    unsafe fn setAttributedLabel_(&self, attributedLabel: NSAttributedString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAttributedLabel : attributedLabel)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"AXDataPoint").unwrap(), new)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct AXDataSeriesDescriptor(pub id);
impl std::ops::Deref for AXDataSeriesDescriptor {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for AXDataSeriesDescriptor {}
impl AXDataSeriesDescriptor {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"AXDataSeriesDescriptor").unwrap(), alloc) })
    }
}
impl PNSCopying for AXDataSeriesDescriptor {}
impl INSObject for AXDataSeriesDescriptor {}
impl PNSObject for AXDataSeriesDescriptor {}
impl std::convert::TryFrom<NSObject> for AXDataSeriesDescriptor {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<AXDataSeriesDescriptor, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"AXDataSeriesDescriptor").unwrap()) };
        if is_kind_of {
            Ok(AXDataSeriesDescriptor(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to AXDataSeriesDescriptor")
        }
    }
}
impl IAXDataSeriesDescriptor for AXDataSeriesDescriptor {}
pub trait IAXDataSeriesDescriptor: Sized + std::ops::Deref {
    unsafe fn initWithName_isContinuous_dataPoints_(
        &self,
        name: NSString,
        isContinuous: BOOL,
        dataPoints: NSArray,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithName : name, isContinuous : isContinuous, dataPoints : dataPoints)
    }
    unsafe fn initWithAttributedName_isContinuous_dataPoints_(
        &self,
        attributedName: NSAttributedString,
        isContinuous: BOOL,
        dataPoints: NSArray,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithAttributedName : attributedName, isContinuous : isContinuous, dataPoints : dataPoints)
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
    unsafe fn setName_(&self, name: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setName : name)
    }
    unsafe fn attributedName(&self) -> NSAttributedString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, attributedName)
    }
    unsafe fn setAttributedName_(&self, attributedName: NSAttributedString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAttributedName : attributedName)
    }
    unsafe fn isContinuous(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isContinuous)
    }
    unsafe fn setIsContinuous_(&self, isContinuous: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setIsContinuous : isContinuous)
    }
    unsafe fn dataPoints(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, dataPoints)
    }
    unsafe fn setDataPoints_(&self, dataPoints: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDataPoints : dataPoints)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"AXDataSeriesDescriptor").unwrap(), new)
    }
}
pub type AXChartDescriptorContentDirection = NSInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct AXChartDescriptor(pub id);
impl std::ops::Deref for AXChartDescriptor {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for AXChartDescriptor {}
impl AXChartDescriptor {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"AXChartDescriptor").unwrap(), alloc) })
    }
}
impl PNSCopying for AXChartDescriptor {}
impl INSObject for AXChartDescriptor {}
impl PNSObject for AXChartDescriptor {}
impl std::convert::TryFrom<NSObject> for AXChartDescriptor {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<AXChartDescriptor, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"AXChartDescriptor").unwrap()) };
        if is_kind_of {
            Ok(AXChartDescriptor(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to AXChartDescriptor")
        }
    }
}
impl IAXChartDescriptor for AXChartDescriptor {}
pub trait IAXChartDescriptor: Sized + std::ops::Deref {
    unsafe fn initWithTitle_summary_xAxisDescriptor_yAxisDescriptor_series_(
        &self,
        title: NSString,
        summary: NSString,
        xAxis: *mut u64,
        yAxis: AXNumericDataAxisDescriptor,
        series: NSArray,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithTitle : title, summary : summary, xAxisDescriptor : xAxis, yAxisDescriptor : yAxis, series : series)
    }
    unsafe fn initWithAttributedTitle_summary_xAxisDescriptor_yAxisDescriptor_series_(
        &self,
        attributedTitle: NSAttributedString,
        summary: NSString,
        xAxis: *mut u64,
        yAxis: AXNumericDataAxisDescriptor,
        series: NSArray,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithAttributedTitle : attributedTitle, summary : summary, xAxisDescriptor : xAxis, yAxisDescriptor : yAxis, series : series)
    }
    unsafe fn initWithTitle_summary_xAxisDescriptor_yAxisDescriptor_additionalAxes_series_(
        &self,
        title: NSString,
        summary: NSString,
        xAxis: *mut u64,
        yAxis: AXNumericDataAxisDescriptor,
        additionalAxes: NSArray,
        series: NSArray,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithTitle : title, summary : summary, xAxisDescriptor : xAxis, yAxisDescriptor : yAxis, additionalAxes : additionalAxes, series : series)
    }
    unsafe fn initWithAttributedTitle_summary_xAxisDescriptor_yAxisDescriptor_additionalAxes_series_(
        &self,
        attributedTitle: NSAttributedString,
        summary: NSString,
        xAxis: *mut u64,
        yAxis: AXNumericDataAxisDescriptor,
        additionalAxes: NSArray,
        series: NSArray,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithAttributedTitle : attributedTitle, summary : summary, xAxisDescriptor : xAxis, yAxisDescriptor : yAxis, additionalAxes : additionalAxes, series : series)
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
    unsafe fn attributedTitle(&self) -> NSAttributedString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, attributedTitle)
    }
    unsafe fn setAttributedTitle_(&self, attributedTitle: NSAttributedString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAttributedTitle : attributedTitle)
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
    unsafe fn contentDirection(&self) -> AXChartDescriptorContentDirection
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, contentDirection)
    }
    unsafe fn setContentDirection_(&self, contentDirection: AXChartDescriptorContentDirection)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setContentDirection : contentDirection)
    }
    unsafe fn contentFrame(&self) -> CGRect
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, contentFrame)
    }
    unsafe fn setContentFrame_(&self, contentFrame: CGRect)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setContentFrame : contentFrame)
    }
    unsafe fn series(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, series)
    }
    unsafe fn setSeries_(&self, series: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSeries : series)
    }
    unsafe fn xAxis(&self) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, xAxis)
    }
    unsafe fn setXAxis_(&self, xAxis: *mut u64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setXAxis : xAxis)
    }
    unsafe fn yAxis(&self) -> AXNumericDataAxisDescriptor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, yAxis)
    }
    unsafe fn setYAxis_(&self, yAxis: AXNumericDataAxisDescriptor)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setYAxis : yAxis)
    }
    unsafe fn additionalAxes(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, additionalAxes)
    }
    unsafe fn setAdditionalAxes_(&self, additionalAxes: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAdditionalAxes : additionalAxes)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"AXChartDescriptor").unwrap(), new)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct AXLiveAudioGraph(pub id);
impl std::ops::Deref for AXLiveAudioGraph {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for AXLiveAudioGraph {}
impl AXLiveAudioGraph {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"AXLiveAudioGraph").unwrap(), alloc) })
    }
}
impl INSObject for AXLiveAudioGraph {}
impl PNSObject for AXLiveAudioGraph {}
impl std::convert::TryFrom<NSObject> for AXLiveAudioGraph {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<AXLiveAudioGraph, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"AXLiveAudioGraph").unwrap()) };
        if is_kind_of {
            Ok(AXLiveAudioGraph(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to AXLiveAudioGraph")
        }
    }
}
impl IAXLiveAudioGraph for AXLiveAudioGraph {}
pub trait IAXLiveAudioGraph: Sized + std::ops::Deref {
    unsafe fn start()
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"AXLiveAudioGraph").unwrap(), start)
    }
    unsafe fn updateValue_(value: f64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"AXLiveAudioGraph").unwrap(), updateValue : value)
    }
    unsafe fn stop()
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"AXLiveAudioGraph").unwrap(), stop)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct AXBrailleMap(pub id);
impl std::ops::Deref for AXBrailleMap {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for AXBrailleMap {}
impl AXBrailleMap {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"AXBrailleMap").unwrap(), alloc) })
    }
}
impl PNSCopying for AXBrailleMap {}
impl PNSSecureCoding for AXBrailleMap {}
impl INSObject for AXBrailleMap {}
impl PNSObject for AXBrailleMap {}
impl std::convert::TryFrom<NSObject> for AXBrailleMap {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<AXBrailleMap, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"AXBrailleMap").unwrap()) };
        if is_kind_of {
            Ok(AXBrailleMap(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to AXBrailleMap")
        }
    }
}
impl IAXBrailleMap for AXBrailleMap {}
pub trait IAXBrailleMap: Sized + std::ops::Deref {
    unsafe fn setHeight_atPoint_(&self, status: f32, point: CGPoint)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setHeight : status, atPoint : point)
    }
    unsafe fn heightAtPoint_(&self, point: CGPoint) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, heightAtPoint : point)
    }
    unsafe fn presentImage_(&self, image: CGImageRef)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, presentImage : image)
    }
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn dimensions(&self) -> CGSize
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, dimensions)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"AXBrailleMap").unwrap(), new)
    }
}
pub trait PAXBrailleMapRenderer: Sized + std::ops::Deref {
    unsafe fn accessibilityBrailleMapRenderRegion(&self) -> CGRect
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, accessibilityBrailleMapRenderRegion)
    }
    unsafe fn setAccessibilityBrailleMapRenderRegion_(
        &self,
        accessibilityBrailleMapRenderRegion: CGRect,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAccessibilityBrailleMapRenderRegion : accessibilityBrailleMapRenderRegion)
    }
    unsafe fn accessibilityBrailleMapRenderer(&self) -> *mut ::std::os::raw::c_void
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, accessibilityBrailleMapRenderer)
    }
    unsafe fn setAccessibilityBrailleMapRenderer_(
        &self,
        accessibilityBrailleMapRenderer: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAccessibilityBrailleMapRenderer : accessibilityBrailleMapRenderer)
    }
}
pub type AXCustomContentImportance = NSUInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct AXCustomContent(pub id);
impl std::ops::Deref for AXCustomContent {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for AXCustomContent {}
impl AXCustomContent {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"AXCustomContent").unwrap(), alloc) })
    }
}
impl PNSCopying for AXCustomContent {}
impl PNSSecureCoding for AXCustomContent {}
impl INSObject for AXCustomContent {}
impl PNSObject for AXCustomContent {}
impl std::convert::TryFrom<NSObject> for AXCustomContent {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<AXCustomContent, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"AXCustomContent").unwrap()) };
        if is_kind_of {
            Ok(AXCustomContent(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to AXCustomContent")
        }
    }
}
impl IAXCustomContent for AXCustomContent {}
pub trait IAXCustomContent: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn label(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, label)
    }
    unsafe fn attributedLabel(&self) -> NSAttributedString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, attributedLabel)
    }
    unsafe fn value(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, value)
    }
    unsafe fn attributedValue(&self) -> NSAttributedString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, attributedValue)
    }
    unsafe fn importance(&self) -> AXCustomContentImportance
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, importance)
    }
    unsafe fn setImportance_(&self, importance: AXCustomContentImportance)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setImportance : importance)
    }
    unsafe fn customContentWithLabel_value_(label: NSString, value: NSString) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"AXCustomContent").unwrap(), customContentWithLabel : label, value : value)
    }
    unsafe fn customContentWithAttributedLabel_attributedValue_(
        label: NSAttributedString,
        value: NSAttributedString,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"AXCustomContent").unwrap(), customContentWithAttributedLabel : label, attributedValue : value)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"AXCustomContent").unwrap(), new)
    }
}
pub trait PAXCustomContentProvider: Sized + std::ops::Deref {
    unsafe fn accessibilityCustomContent(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, accessibilityCustomContent)
    }
    unsafe fn setAccessibilityCustomContent_(&self, accessibilityCustomContent: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAccessibilityCustomContent : accessibilityCustomContent)
    }
    unsafe fn accessibilityCustomContentBlock(&self) -> AXCustomContentReturnBlock
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, accessibilityCustomContentBlock)
    }
    unsafe fn setAccessibilityCustomContentBlock_(
        &self,
        accessibilityCustomContentBlock: AXCustomContentReturnBlock,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAccessibilityCustomContentBlock : accessibilityCustomContentBlock)
    }
}
pub type AXCustomContentReturnBlock = *mut ::std::os::raw::c_void;
pub type AXHearingDeviceEar = NSUInteger;
pub type AXTechnology = NSString;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct AXRequest(pub id);
impl std::ops::Deref for AXRequest {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for AXRequest {}
impl AXRequest {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"AXRequest").unwrap(), alloc) })
    }
}
impl PNSCopying for AXRequest {}
impl PNSSecureCoding for AXRequest {}
impl INSObject for AXRequest {}
impl PNSObject for AXRequest {}
impl std::convert::TryFrom<NSObject> for AXRequest {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<AXRequest, Self::Error> {
        let is_kind_of: bool = unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"AXRequest").unwrap()) };
        if is_kind_of {
            Ok(AXRequest(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to AXRequest")
        }
    }
}
impl IAXRequest for AXRequest {}
pub trait IAXRequest: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn technology(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, technology)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"AXRequest").unwrap(), new)
    }
    unsafe fn currentRequest() -> AXRequest
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"AXRequest").unwrap(), currentRequest)
    }
}
pub type AXSettingsFeature = NSInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct AXMathExpression(pub id);
impl std::ops::Deref for AXMathExpression {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for AXMathExpression {}
impl AXMathExpression {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"AXMathExpression").unwrap(), alloc) })
    }
}
impl INSObject for AXMathExpression {}
impl PNSObject for AXMathExpression {}
impl std::convert::TryFrom<NSObject> for AXMathExpression {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<AXMathExpression, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"AXMathExpression").unwrap()) };
        if is_kind_of {
            Ok(AXMathExpression(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to AXMathExpression")
        }
    }
}
impl IAXMathExpression for AXMathExpression {}
pub trait IAXMathExpression: Sized + std::ops::Deref {}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct AXMathExpressionNumber(pub id);
impl std::ops::Deref for AXMathExpressionNumber {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for AXMathExpressionNumber {}
impl AXMathExpressionNumber {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"AXMathExpressionNumber").unwrap(), alloc) })
    }
}
impl IAXMathExpression for AXMathExpressionNumber {}
impl From<AXMathExpressionNumber> for AXMathExpression {
    fn from(child: AXMathExpressionNumber) -> AXMathExpression {
        AXMathExpression(child.0)
    }
}
impl std::convert::TryFrom<AXMathExpression> for AXMathExpressionNumber {
    type Error = &'static str;
    fn try_from(parent: AXMathExpression) -> Result<AXMathExpressionNumber, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"AXMathExpressionNumber").unwrap()) };
        if is_kind_of {
            Ok(AXMathExpressionNumber(parent.0))
        } else {
            Err("This AXMathExpression cannot be downcasted to AXMathExpressionNumber")
        }
    }
}
impl INSObject for AXMathExpressionNumber {}
impl PNSObject for AXMathExpressionNumber {}
impl IAXMathExpressionNumber for AXMathExpressionNumber {}
pub trait IAXMathExpressionNumber: Sized + std::ops::Deref {
    unsafe fn initWithContent_(&self, content: NSString) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithContent : content)
    }
    unsafe fn content(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, content)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct AXMathExpressionIdentifier(pub id);
impl std::ops::Deref for AXMathExpressionIdentifier {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for AXMathExpressionIdentifier {}
impl AXMathExpressionIdentifier {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"AXMathExpressionIdentifier").unwrap(), alloc) })
    }
}
impl IAXMathExpression for AXMathExpressionIdentifier {}
impl std::convert::TryFrom<AXMathExpression> for AXMathExpressionIdentifier {
    type Error = &'static str;
    fn try_from(parent: AXMathExpression) -> Result<AXMathExpressionIdentifier, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"AXMathExpressionIdentifier").unwrap()) };
        if is_kind_of {
            Ok(AXMathExpressionIdentifier(parent.0))
        } else {
            Err("This AXMathExpression cannot be downcasted to AXMathExpressionIdentifier")
        }
    }
}
impl INSObject for AXMathExpressionIdentifier {}
impl PNSObject for AXMathExpressionIdentifier {}
impl IAXMathExpressionIdentifier for AXMathExpressionIdentifier {}
pub trait IAXMathExpressionIdentifier: Sized + std::ops::Deref {
    unsafe fn initWithContent_(&self, content: NSString) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithContent : content)
    }
    unsafe fn content(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, content)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct AXMathExpressionOperator(pub id);
impl std::ops::Deref for AXMathExpressionOperator {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for AXMathExpressionOperator {}
impl AXMathExpressionOperator {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"AXMathExpressionOperator").unwrap(), alloc) })
    }
}
impl IAXMathExpression for AXMathExpressionOperator {}
impl std::convert::TryFrom<AXMathExpression> for AXMathExpressionOperator {
    type Error = &'static str;
    fn try_from(parent: AXMathExpression) -> Result<AXMathExpressionOperator, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"AXMathExpressionOperator").unwrap()) };
        if is_kind_of {
            Ok(AXMathExpressionOperator(parent.0))
        } else {
            Err("This AXMathExpression cannot be downcasted to AXMathExpressionOperator")
        }
    }
}
impl INSObject for AXMathExpressionOperator {}
impl PNSObject for AXMathExpressionOperator {}
impl IAXMathExpressionOperator for AXMathExpressionOperator {}
pub trait IAXMathExpressionOperator: Sized + std::ops::Deref {
    unsafe fn initWithContent_(&self, content: NSString) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithContent : content)
    }
    unsafe fn content(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, content)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct AXMathExpressionText(pub id);
impl std::ops::Deref for AXMathExpressionText {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for AXMathExpressionText {}
impl AXMathExpressionText {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"AXMathExpressionText").unwrap(), alloc) })
    }
}
impl IAXMathExpression for AXMathExpressionText {}
impl std::convert::TryFrom<AXMathExpression> for AXMathExpressionText {
    type Error = &'static str;
    fn try_from(parent: AXMathExpression) -> Result<AXMathExpressionText, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"AXMathExpressionText").unwrap()) };
        if is_kind_of {
            Ok(AXMathExpressionText(parent.0))
        } else {
            Err("This AXMathExpression cannot be downcasted to AXMathExpressionText")
        }
    }
}
impl INSObject for AXMathExpressionText {}
impl PNSObject for AXMathExpressionText {}
impl IAXMathExpressionText for AXMathExpressionText {}
pub trait IAXMathExpressionText: Sized + std::ops::Deref {
    unsafe fn initWithContent_(&self, content: NSString) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithContent : content)
    }
    unsafe fn content(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, content)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct AXMathExpressionFenced(pub id);
impl std::ops::Deref for AXMathExpressionFenced {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for AXMathExpressionFenced {}
impl AXMathExpressionFenced {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"AXMathExpressionFenced").unwrap(), alloc) })
    }
}
impl IAXMathExpression for AXMathExpressionFenced {}
impl std::convert::TryFrom<AXMathExpression> for AXMathExpressionFenced {
    type Error = &'static str;
    fn try_from(parent: AXMathExpression) -> Result<AXMathExpressionFenced, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"AXMathExpressionFenced").unwrap()) };
        if is_kind_of {
            Ok(AXMathExpressionFenced(parent.0))
        } else {
            Err("This AXMathExpression cannot be downcasted to AXMathExpressionFenced")
        }
    }
}
impl INSObject for AXMathExpressionFenced {}
impl PNSObject for AXMathExpressionFenced {}
impl IAXMathExpressionFenced for AXMathExpressionFenced {}
pub trait IAXMathExpressionFenced: Sized + std::ops::Deref {
    unsafe fn initWithExpressions_openString_closeString_(
        &self,
        expressions: NSArray,
        openString: NSString,
        closeString: NSString,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithExpressions : expressions, openString : openString, closeString : closeString)
    }
    unsafe fn expressions(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, expressions)
    }
    unsafe fn openString(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, openString)
    }
    unsafe fn closeString(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, closeString)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct AXMathExpressionRow(pub id);
impl std::ops::Deref for AXMathExpressionRow {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for AXMathExpressionRow {}
impl AXMathExpressionRow {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"AXMathExpressionRow").unwrap(), alloc) })
    }
}
impl IAXMathExpression for AXMathExpressionRow {}
impl std::convert::TryFrom<AXMathExpression> for AXMathExpressionRow {
    type Error = &'static str;
    fn try_from(parent: AXMathExpression) -> Result<AXMathExpressionRow, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"AXMathExpressionRow").unwrap()) };
        if is_kind_of {
            Ok(AXMathExpressionRow(parent.0))
        } else {
            Err("This AXMathExpression cannot be downcasted to AXMathExpressionRow")
        }
    }
}
impl INSObject for AXMathExpressionRow {}
impl PNSObject for AXMathExpressionRow {}
impl IAXMathExpressionRow for AXMathExpressionRow {}
pub trait IAXMathExpressionRow: Sized + std::ops::Deref {
    unsafe fn initWithExpressions_(&self, expressions: NSArray) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithExpressions : expressions)
    }
    unsafe fn expressions(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, expressions)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct AXMathExpressionTable(pub id);
impl std::ops::Deref for AXMathExpressionTable {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for AXMathExpressionTable {}
impl AXMathExpressionTable {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"AXMathExpressionTable").unwrap(), alloc) })
    }
}
impl IAXMathExpression for AXMathExpressionTable {}
impl std::convert::TryFrom<AXMathExpression> for AXMathExpressionTable {
    type Error = &'static str;
    fn try_from(parent: AXMathExpression) -> Result<AXMathExpressionTable, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"AXMathExpressionTable").unwrap()) };
        if is_kind_of {
            Ok(AXMathExpressionTable(parent.0))
        } else {
            Err("This AXMathExpression cannot be downcasted to AXMathExpressionTable")
        }
    }
}
impl INSObject for AXMathExpressionTable {}
impl PNSObject for AXMathExpressionTable {}
impl IAXMathExpressionTable for AXMathExpressionTable {}
pub trait IAXMathExpressionTable: Sized + std::ops::Deref {
    unsafe fn initWithExpressions_(&self, expressions: NSArray) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithExpressions : expressions)
    }
    unsafe fn expressions(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, expressions)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct AXMathExpressionTableRow(pub id);
impl std::ops::Deref for AXMathExpressionTableRow {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for AXMathExpressionTableRow {}
impl AXMathExpressionTableRow {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"AXMathExpressionTableRow").unwrap(), alloc) })
    }
}
impl IAXMathExpression for AXMathExpressionTableRow {}
impl std::convert::TryFrom<AXMathExpression> for AXMathExpressionTableRow {
    type Error = &'static str;
    fn try_from(parent: AXMathExpression) -> Result<AXMathExpressionTableRow, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"AXMathExpressionTableRow").unwrap()) };
        if is_kind_of {
            Ok(AXMathExpressionTableRow(parent.0))
        } else {
            Err("This AXMathExpression cannot be downcasted to AXMathExpressionTableRow")
        }
    }
}
impl INSObject for AXMathExpressionTableRow {}
impl PNSObject for AXMathExpressionTableRow {}
impl IAXMathExpressionTableRow for AXMathExpressionTableRow {}
pub trait IAXMathExpressionTableRow: Sized + std::ops::Deref {
    unsafe fn initWithExpressions_(&self, expressions: NSArray) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithExpressions : expressions)
    }
    unsafe fn expressions(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, expressions)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct AXMathExpressionTableCell(pub id);
impl std::ops::Deref for AXMathExpressionTableCell {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for AXMathExpressionTableCell {}
impl AXMathExpressionTableCell {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"AXMathExpressionTableCell").unwrap(), alloc) })
    }
}
impl IAXMathExpression for AXMathExpressionTableCell {}
impl std::convert::TryFrom<AXMathExpression> for AXMathExpressionTableCell {
    type Error = &'static str;
    fn try_from(parent: AXMathExpression) -> Result<AXMathExpressionTableCell, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"AXMathExpressionTableCell").unwrap()) };
        if is_kind_of {
            Ok(AXMathExpressionTableCell(parent.0))
        } else {
            Err("This AXMathExpression cannot be downcasted to AXMathExpressionTableCell")
        }
    }
}
impl INSObject for AXMathExpressionTableCell {}
impl PNSObject for AXMathExpressionTableCell {}
impl IAXMathExpressionTableCell for AXMathExpressionTableCell {}
pub trait IAXMathExpressionTableCell: Sized + std::ops::Deref {
    unsafe fn initWithExpressions_(&self, expressions: NSArray) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithExpressions : expressions)
    }
    unsafe fn expressions(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, expressions)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct AXMathExpressionUnderOver(pub id);
impl std::ops::Deref for AXMathExpressionUnderOver {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for AXMathExpressionUnderOver {}
impl AXMathExpressionUnderOver {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"AXMathExpressionUnderOver").unwrap(), alloc) })
    }
}
impl IAXMathExpression for AXMathExpressionUnderOver {}
impl std::convert::TryFrom<AXMathExpression> for AXMathExpressionUnderOver {
    type Error = &'static str;
    fn try_from(parent: AXMathExpression) -> Result<AXMathExpressionUnderOver, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"AXMathExpressionUnderOver").unwrap()) };
        if is_kind_of {
            Ok(AXMathExpressionUnderOver(parent.0))
        } else {
            Err("This AXMathExpression cannot be downcasted to AXMathExpressionUnderOver")
        }
    }
}
impl INSObject for AXMathExpressionUnderOver {}
impl PNSObject for AXMathExpressionUnderOver {}
impl IAXMathExpressionUnderOver for AXMathExpressionUnderOver {}
pub trait IAXMathExpressionUnderOver: Sized + std::ops::Deref {
    unsafe fn initWithBaseExpression_underExpression_overExpression_(
        &self,
        baseExpression: AXMathExpression,
        underExpression: AXMathExpression,
        overExpression: AXMathExpression,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithBaseExpression : baseExpression, underExpression : underExpression, overExpression : overExpression)
    }
    unsafe fn baseExpression(&self) -> AXMathExpression
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, baseExpression)
    }
    unsafe fn underExpression(&self) -> AXMathExpression
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, underExpression)
    }
    unsafe fn overExpression(&self) -> AXMathExpression
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, overExpression)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct AXMathExpressionSubSuperscript(pub id);
impl std::ops::Deref for AXMathExpressionSubSuperscript {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for AXMathExpressionSubSuperscript {}
impl AXMathExpressionSubSuperscript {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"AXMathExpressionSubSuperscript").unwrap(), alloc) })
    }
}
impl IAXMathExpression for AXMathExpressionSubSuperscript {}
impl std::convert::TryFrom<AXMathExpression> for AXMathExpressionSubSuperscript {
    type Error = &'static str;
    fn try_from(parent: AXMathExpression) -> Result<AXMathExpressionSubSuperscript, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"AXMathExpressionSubSuperscript").unwrap())
        };
        if is_kind_of {
            Ok(AXMathExpressionSubSuperscript(parent.0))
        } else {
            Err("This AXMathExpression cannot be downcasted to AXMathExpressionSubSuperscript")
        }
    }
}
impl INSObject for AXMathExpressionSubSuperscript {}
impl PNSObject for AXMathExpressionSubSuperscript {}
impl IAXMathExpressionSubSuperscript for AXMathExpressionSubSuperscript {}
pub trait IAXMathExpressionSubSuperscript: Sized + std::ops::Deref {
    unsafe fn initWithBaseExpression_subscriptExpressions_superscriptExpressions_(
        &self,
        baseExpression: NSArray,
        subscriptExpressions: NSArray,
        superscriptExpressions: NSArray,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithBaseExpression : baseExpression, subscriptExpressions : subscriptExpressions, superscriptExpressions : superscriptExpressions)
    }
    unsafe fn baseExpression(&self) -> AXMathExpression
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, baseExpression)
    }
    unsafe fn subscriptExpressions(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, subscriptExpressions)
    }
    unsafe fn superscriptExpressions(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, superscriptExpressions)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct AXMathExpressionFraction(pub id);
impl std::ops::Deref for AXMathExpressionFraction {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for AXMathExpressionFraction {}
impl AXMathExpressionFraction {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"AXMathExpressionFraction").unwrap(), alloc) })
    }
}
impl IAXMathExpression for AXMathExpressionFraction {}
impl std::convert::TryFrom<AXMathExpression> for AXMathExpressionFraction {
    type Error = &'static str;
    fn try_from(parent: AXMathExpression) -> Result<AXMathExpressionFraction, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"AXMathExpressionFraction").unwrap()) };
        if is_kind_of {
            Ok(AXMathExpressionFraction(parent.0))
        } else {
            Err("This AXMathExpression cannot be downcasted to AXMathExpressionFraction")
        }
    }
}
impl INSObject for AXMathExpressionFraction {}
impl PNSObject for AXMathExpressionFraction {}
impl IAXMathExpressionFraction for AXMathExpressionFraction {}
pub trait IAXMathExpressionFraction: Sized + std::ops::Deref {
    unsafe fn initWithNumeratorExpression_denimonatorExpression_(
        &self,
        numeratorExpression: AXMathExpression,
        denimonatorExpression: AXMathExpression,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithNumeratorExpression : numeratorExpression, denimonatorExpression : denimonatorExpression)
    }
    unsafe fn numeratorExpression(&self) -> AXMathExpression
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, numeratorExpression)
    }
    unsafe fn denimonatorExpression(&self) -> AXMathExpression
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, denimonatorExpression)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct AXMathExpressionMultiscript(pub id);
impl std::ops::Deref for AXMathExpressionMultiscript {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for AXMathExpressionMultiscript {}
impl AXMathExpressionMultiscript {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"AXMathExpressionMultiscript").unwrap(), alloc) })
    }
}
impl IAXMathExpression for AXMathExpressionMultiscript {}
impl std::convert::TryFrom<AXMathExpression> for AXMathExpressionMultiscript {
    type Error = &'static str;
    fn try_from(parent: AXMathExpression) -> Result<AXMathExpressionMultiscript, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"AXMathExpressionMultiscript").unwrap()) };
        if is_kind_of {
            Ok(AXMathExpressionMultiscript(parent.0))
        } else {
            Err("This AXMathExpression cannot be downcasted to AXMathExpressionMultiscript")
        }
    }
}
impl INSObject for AXMathExpressionMultiscript {}
impl PNSObject for AXMathExpressionMultiscript {}
impl IAXMathExpressionMultiscript for AXMathExpressionMultiscript {}
pub trait IAXMathExpressionMultiscript: Sized + std::ops::Deref {
    unsafe fn initWithBaseExpression_prescriptExpressions_postscriptExpressions_(
        &self,
        baseExpression: AXMathExpression,
        prescriptExpressions: NSArray,
        postscriptExpressions: NSArray,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithBaseExpression : baseExpression, prescriptExpressions : prescriptExpressions, postscriptExpressions : postscriptExpressions)
    }
    unsafe fn baseExpression(&self) -> AXMathExpression
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, baseExpression)
    }
    unsafe fn prescriptExpressions(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, prescriptExpressions)
    }
    unsafe fn postscriptExpressions(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, postscriptExpressions)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct AXMathExpressionRoot(pub id);
impl std::ops::Deref for AXMathExpressionRoot {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for AXMathExpressionRoot {}
impl AXMathExpressionRoot {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"AXMathExpressionRoot").unwrap(), alloc) })
    }
}
impl IAXMathExpression for AXMathExpressionRoot {}
impl std::convert::TryFrom<AXMathExpression> for AXMathExpressionRoot {
    type Error = &'static str;
    fn try_from(parent: AXMathExpression) -> Result<AXMathExpressionRoot, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"AXMathExpressionRoot").unwrap()) };
        if is_kind_of {
            Ok(AXMathExpressionRoot(parent.0))
        } else {
            Err("This AXMathExpression cannot be downcasted to AXMathExpressionRoot")
        }
    }
}
impl INSObject for AXMathExpressionRoot {}
impl PNSObject for AXMathExpressionRoot {}
impl IAXMathExpressionRoot for AXMathExpressionRoot {}
pub trait IAXMathExpressionRoot: Sized + std::ops::Deref {
    unsafe fn initWithRadicandExpressions_rootIndexExpression_(
        &self,
        radicandExpressions: NSArray,
        rootIndexExpression: AXMathExpression,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithRadicandExpressions : radicandExpressions, rootIndexExpression : rootIndexExpression)
    }
    unsafe fn radicandExpressions(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, radicandExpressions)
    }
    unsafe fn rootIndexExpression(&self) -> AXMathExpression
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, rootIndexExpression)
    }
}
pub trait PAXMathExpressionProvider: Sized + std::ops::Deref {
    unsafe fn accessibilityMathExpression(&self) -> AXMathExpression
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, accessibilityMathExpression)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct AXBrailleTable(pub id);
impl std::ops::Deref for AXBrailleTable {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for AXBrailleTable {}
impl AXBrailleTable {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"AXBrailleTable").unwrap(), alloc) })
    }
}
impl PNSCopying for AXBrailleTable {}
impl PNSCoding for AXBrailleTable {}
impl INSObject for AXBrailleTable {}
impl PNSObject for AXBrailleTable {}
impl std::convert::TryFrom<NSObject> for AXBrailleTable {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<AXBrailleTable, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"AXBrailleTable").unwrap()) };
        if is_kind_of {
            Ok(AXBrailleTable(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to AXBrailleTable")
        }
    }
}
impl IAXBrailleTable for AXBrailleTable {}
pub trait IAXBrailleTable: Sized + std::ops::Deref {
    unsafe fn initWithIdentifier_(&self, identifier: NSString) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithIdentifier : identifier)
    }
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
    unsafe fn localizedName(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, localizedName)
    }
    unsafe fn providerIdentifier(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, providerIdentifier)
    }
    unsafe fn localizedProviderName(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, localizedProviderName)
    }
    unsafe fn language(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, language)
    }
    unsafe fn locales(&self) -> NSSet
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, locales)
    }
    unsafe fn isEightDot(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isEightDot)
    }
    unsafe fn supportedLocales() -> NSSet
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"AXBrailleTable").unwrap(), supportedLocales)
    }
    unsafe fn defaultTableForLocale_(locale: NSLocale) -> AXBrailleTable
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"AXBrailleTable").unwrap(), defaultTableForLocale : locale)
    }
    unsafe fn tablesForLocale_(locale: NSLocale) -> NSSet
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"AXBrailleTable").unwrap(), tablesForLocale : locale)
    }
    unsafe fn languageAgnosticTables() -> NSSet
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"AXBrailleTable").unwrap(), languageAgnosticTables)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"AXBrailleTable").unwrap(), new)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct AXBrailleTranslationResult(pub id);
impl std::ops::Deref for AXBrailleTranslationResult {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for AXBrailleTranslationResult {}
impl AXBrailleTranslationResult {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"AXBrailleTranslationResult").unwrap(), alloc) })
    }
}
impl PNSCopying for AXBrailleTranslationResult {}
impl PNSCoding for AXBrailleTranslationResult {}
impl INSObject for AXBrailleTranslationResult {}
impl PNSObject for AXBrailleTranslationResult {}
impl std::convert::TryFrom<NSObject> for AXBrailleTranslationResult {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<AXBrailleTranslationResult, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"AXBrailleTranslationResult").unwrap()) };
        if is_kind_of {
            Ok(AXBrailleTranslationResult(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to AXBrailleTranslationResult")
        }
    }
}
impl IAXBrailleTranslationResult for AXBrailleTranslationResult {}
pub trait IAXBrailleTranslationResult: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn resultString(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, resultString)
    }
    unsafe fn locationMap(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, locationMap)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"AXBrailleTranslationResult").unwrap(), new)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct AXBrailleTranslator(pub id);
impl std::ops::Deref for AXBrailleTranslator {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for AXBrailleTranslator {}
impl AXBrailleTranslator {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"AXBrailleTranslator").unwrap(), alloc) })
    }
}
impl INSObject for AXBrailleTranslator {}
impl PNSObject for AXBrailleTranslator {}
impl std::convert::TryFrom<NSObject> for AXBrailleTranslator {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<AXBrailleTranslator, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"AXBrailleTranslator").unwrap()) };
        if is_kind_of {
            Ok(AXBrailleTranslator(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to AXBrailleTranslator")
        }
    }
}
impl IAXBrailleTranslator for AXBrailleTranslator {}
pub trait IAXBrailleTranslator: Sized + std::ops::Deref {
    unsafe fn initWithBrailleTable_(&self, brailleTable: AXBrailleTable) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithBrailleTable : brailleTable)
    }
    unsafe fn translatePrintText_(&self, printText: NSString) -> AXBrailleTranslationResult
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, translatePrintText : printText)
    }
    unsafe fn backTranslateBraille_(&self, braille: NSString) -> AXBrailleTranslationResult
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, backTranslateBraille : braille)
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
        msg_send!(&*objc2::runtime::AnyClass::get(c"AXBrailleTranslator").unwrap(), new)
    }
}
unsafe extern "C" {
    pub fn AXNameFromColor(color: CGColorRef) -> NSString;
}
unsafe extern "C" {
    pub fn AXMFiHearingDeviceStreamingEar() -> AXHearingDeviceEar;
}
unsafe extern "C" {
    pub static AXMFiHearingDeviceStreamingEarDidChangeNotification: NSNotificationName;
}
unsafe extern "C" {
    pub fn AXSupportsBidirectionalAXMFiHearingDeviceStreaming() -> BOOL;
}
unsafe extern "C" {
    pub fn AXMFiHearingDevicePairedUUIDs() -> NSArray;
}
unsafe extern "C" {
    pub static AXMFiHearingDevicePairedUUIDsDidChangeNotification: NSNotificationName;
}
unsafe extern "C" {
    pub static mut AXTechnologyVoiceOver: AXTechnology;
}
unsafe extern "C" {
    pub static mut AXTechnologySwitchControl: AXTechnology;
}
unsafe extern "C" {
    pub static mut AXTechnologyVoiceControl: AXTechnology;
}
unsafe extern "C" {
    pub static mut AXTechnologyFullKeyboardAccess: AXTechnology;
}
unsafe extern "C" {
    pub static mut AXTechnologySpeakScreen: AXTechnology;
}
unsafe extern "C" {
    pub static mut AXTechnologyAutomation: AXTechnology;
}
unsafe extern "C" {
    pub static mut AXTechnologyHoverText: AXTechnology;
}
unsafe extern "C" {
    pub static mut AXTechnologyZoom: AXTechnology;
}
unsafe extern "C" {
    pub fn AXPrefersHorizontalTextLayout() -> BOOL;
}
unsafe extern "C" {
    pub static AXPrefersHorizontalTextLayoutDidChangeNotification: NSNotificationName;
}
unsafe extern "C" {
    pub fn AXAnimatedImagesEnabled() -> BOOL;
}
unsafe extern "C" {
    pub static AXAnimatedImagesEnabledDidChangeNotification: NSNotificationName;
}
unsafe extern "C" {
    pub fn AXAssistiveAccessEnabled() -> BOOL;
}
unsafe extern "C" {
    pub fn AXPrefersNonBlinkingTextInsertionIndicator() -> BOOL;
}
unsafe extern "C" {
    pub static AXPrefersNonBlinkingTextInsertionIndicatorDidChangeNotification: NSNotificationName;
}
unsafe extern "C" {
    pub fn AXPrefersActionSliderAlternative() -> BOOL;
}
unsafe extern "C" {
    pub static AXPrefersActionSliderAlternativeDidChangeNotification: NSNotificationName;
}
unsafe extern "C" {
    pub fn AXShowBordersEnabled() -> BOOL;
}
unsafe extern "C" {
    pub static AXShowBordersEnabledStatusDidChangeNotification: NSNotificationName;
}
unsafe extern "C" {
    pub fn AXOpenSettingsFeature(
        feature: AXSettingsFeature,
        completionHandler: *mut ::std::os::raw::c_void,
    );
}

unsafe impl objc2::encode::RefEncode for AXNumericDataAxisDescriptor {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for AXNumericDataAxisDescriptor {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for AXCategoricalDataAxisDescriptor {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for AXCategoricalDataAxisDescriptor {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for AXDataPointValue {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for AXDataPointValue {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for AXDataPoint {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for AXDataPoint {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for AXDataSeriesDescriptor {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for AXDataSeriesDescriptor {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for AXChartDescriptor {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for AXChartDescriptor {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for AXLiveAudioGraph {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for AXLiveAudioGraph {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for AXBrailleMap {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for AXBrailleMap {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for AXCustomContent {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for AXCustomContent {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for AXRequest {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for AXRequest {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for AXMathExpression {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for AXMathExpression {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for AXMathExpressionNumber {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for AXMathExpressionNumber {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for AXMathExpressionIdentifier {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for AXMathExpressionIdentifier {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for AXMathExpressionOperator {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for AXMathExpressionOperator {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for AXMathExpressionText {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for AXMathExpressionText {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for AXMathExpressionFenced {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for AXMathExpressionFenced {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for AXMathExpressionRow {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for AXMathExpressionRow {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for AXMathExpressionTable {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for AXMathExpressionTable {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for AXMathExpressionTableRow {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for AXMathExpressionTableRow {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for AXMathExpressionTableCell {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for AXMathExpressionTableCell {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for AXMathExpressionUnderOver {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for AXMathExpressionUnderOver {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for AXMathExpressionSubSuperscript {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for AXMathExpressionSubSuperscript {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for AXMathExpressionFraction {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for AXMathExpressionFraction {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for AXMathExpressionMultiscript {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for AXMathExpressionMultiscript {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for AXMathExpressionRoot {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for AXMathExpressionRoot {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for AXBrailleTable {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for AXBrailleTable {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for AXBrailleTranslationResult {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for AXBrailleTranslationResult {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for AXBrailleTranslator {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for AXBrailleTranslator {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
