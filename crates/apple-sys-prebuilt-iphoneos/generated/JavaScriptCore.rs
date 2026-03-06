#[allow(unused_imports)]
use crate::objc::*;
#[allow(unused_imports)]
use crate::CoreFoundation::*;
#[allow(unused_imports)]
use crate::CoreGraphics::*;
#[allow(unused_imports)]
use crate::Foundation::*;
#[allow(unused_imports)]
use crate::Security::*;

#[allow(unused_imports)]
use objc2::msg_send;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OpaqueJSContextGroup {
    _unused: [u8; 0],
}
pub type JSContextGroupRef = *const OpaqueJSContextGroup;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OpaqueJSContext {
    _unused: [u8; 0],
}
pub type JSContextRef = *const OpaqueJSContext;
pub type JSGlobalContextRef = *mut OpaqueJSContext;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OpaqueJSString {
    _unused: [u8; 0],
}
pub type JSStringRef = *mut OpaqueJSString;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OpaqueJSClass {
    _unused: [u8; 0],
}
pub type JSClassRef = *mut OpaqueJSClass;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OpaqueJSPropertyNameArray {
    _unused: [u8; 0],
}
pub type JSPropertyNameArrayRef = *mut OpaqueJSPropertyNameArray;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OpaqueJSPropertyNameAccumulator {
    _unused: [u8; 0],
}
pub type JSPropertyNameAccumulatorRef = *mut OpaqueJSPropertyNameAccumulator;
pub type JSTypedArrayBytesDeallocator = ::std::option::Option<
    unsafe extern "C" fn(
        bytes: *mut ::std::os::raw::c_void,
        deallocatorContext: *mut ::std::os::raw::c_void,
    ),
>;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OpaqueJSValue {
    _unused: [u8; 0],
}
pub type JSValueRef = *const OpaqueJSValue;
pub type JSObjectRef = *mut OpaqueJSValue;
pub type JSType = ::std::os::raw::c_uint;
pub type JSTypedArrayType = ::std::os::raw::c_uint;
pub type JSRelationCondition = u32;
pub type JSPropertyAttributes = ::std::os::raw::c_uint;
pub type JSClassAttributes = ::std::os::raw::c_uint;
pub type JSObjectInitializeCallback =
    ::std::option::Option<unsafe extern "C" fn(ctx: JSContextRef, object: JSObjectRef)>;
pub type JSObjectFinalizeCallback =
    ::std::option::Option<unsafe extern "C" fn(object: JSObjectRef)>;
pub type JSObjectHasPropertyCallback = ::std::option::Option<
    unsafe extern "C" fn(ctx: JSContextRef, object: JSObjectRef, propertyName: JSStringRef) -> bool,
>;
pub type JSObjectGetPropertyCallback = ::std::option::Option<
    unsafe extern "C" fn(
        ctx: JSContextRef,
        object: JSObjectRef,
        propertyName: JSStringRef,
        exception: *mut JSValueRef,
    ) -> JSValueRef,
>;
pub type JSObjectSetPropertyCallback = ::std::option::Option<
    unsafe extern "C" fn(
        ctx: JSContextRef,
        object: JSObjectRef,
        propertyName: JSStringRef,
        value: JSValueRef,
        exception: *mut JSValueRef,
    ) -> bool,
>;
pub type JSObjectDeletePropertyCallback = ::std::option::Option<
    unsafe extern "C" fn(
        ctx: JSContextRef,
        object: JSObjectRef,
        propertyName: JSStringRef,
        exception: *mut JSValueRef,
    ) -> bool,
>;
pub type JSObjectGetPropertyNamesCallback = ::std::option::Option<
    unsafe extern "C" fn(
        ctx: JSContextRef,
        object: JSObjectRef,
        propertyNames: JSPropertyNameAccumulatorRef,
    ),
>;
pub type JSObjectCallAsFunctionCallback = ::std::option::Option<
    unsafe extern "C" fn(
        ctx: JSContextRef,
        function: JSObjectRef,
        thisObject: JSObjectRef,
        argumentCount: usize,
        arguments: *const JSValueRef,
        exception: *mut JSValueRef,
    ) -> JSValueRef,
>;
pub type JSObjectCallAsConstructorCallback = ::std::option::Option<
    unsafe extern "C" fn(
        ctx: JSContextRef,
        constructor: JSObjectRef,
        argumentCount: usize,
        arguments: *const JSValueRef,
        exception: *mut JSValueRef,
    ) -> JSObjectRef,
>;
pub type JSObjectHasInstanceCallback = ::std::option::Option<
    unsafe extern "C" fn(
        ctx: JSContextRef,
        constructor: JSObjectRef,
        possibleInstance: JSValueRef,
        exception: *mut JSValueRef,
    ) -> bool,
>;
pub type JSObjectConvertToTypeCallback = ::std::option::Option<
    unsafe extern "C" fn(
        ctx: JSContextRef,
        object: JSObjectRef,
        type_: JSType,
        exception: *mut JSValueRef,
    ) -> JSValueRef,
>;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct JSStaticValue {
    pub name: *const ::std::os::raw::c_char,
    pub getProperty: JSObjectGetPropertyCallback,
    pub setProperty: JSObjectSetPropertyCallback,
    pub attributes: JSPropertyAttributes,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct JSStaticFunction {
    pub name: *const ::std::os::raw::c_char,
    pub callAsFunction: JSObjectCallAsFunctionCallback,
    pub attributes: JSPropertyAttributes,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct JSClassDefinition {
    pub version: ::std::os::raw::c_int,
    pub attributes: JSClassAttributes,
    pub className: *const ::std::os::raw::c_char,
    pub parentClass: JSClassRef,
    pub staticValues: *const JSStaticValue,
    pub staticFunctions: *const JSStaticFunction,
    pub initialize: JSObjectInitializeCallback,
    pub finalize: JSObjectFinalizeCallback,
    pub hasProperty: JSObjectHasPropertyCallback,
    pub getProperty: JSObjectGetPropertyCallback,
    pub setProperty: JSObjectSetPropertyCallback,
    pub deleteProperty: JSObjectDeletePropertyCallback,
    pub getPropertyNames: JSObjectGetPropertyNamesCallback,
    pub callAsFunction: JSObjectCallAsFunctionCallback,
    pub callAsConstructor: JSObjectCallAsConstructorCallback,
    pub hasInstance: JSObjectHasInstanceCallback,
    pub convertToType: JSObjectConvertToTypeCallback,
}
pub type JSChar = ::std::os::raw::c_ushort;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct JSContext(pub id);
impl std::ops::Deref for JSContext {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for JSContext {}
impl JSContext {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"JSContext").unwrap(), alloc) })
    }
}
impl INSObject for JSContext {}
impl PNSObject for JSContext {}
impl std::convert::TryFrom<NSObject> for JSContext {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<JSContext, Self::Error> {
        let is_kind_of: bool = unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"JSContext").unwrap()) };
        if is_kind_of {
            Ok(JSContext(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to JSContext")
        }
    }
}
impl IJSContext for JSContext {}
pub trait IJSContext: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn initWithVirtualMachine_(&self, virtualMachine: JSVirtualMachine) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithVirtualMachine : virtualMachine)
    }
    unsafe fn evaluateScript_(&self, script: NSString) -> JSValue
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, evaluateScript : script)
    }
    unsafe fn evaluateScript_withSourceURL_(&self, script: NSString, sourceURL: NSURL) -> JSValue
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, evaluateScript : script, withSourceURL : sourceURL)
    }
    unsafe fn globalObject(&self) -> JSValue
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, globalObject)
    }
    unsafe fn exception(&self) -> JSValue
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, exception)
    }
    unsafe fn setException_(&self, exception: JSValue)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setException : exception)
    }
    unsafe fn exceptionHandler(&self) -> *mut ::std::os::raw::c_void
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, exceptionHandler)
    }
    unsafe fn setExceptionHandler_(&self, exceptionHandler: *mut ::std::os::raw::c_void)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setExceptionHandler : exceptionHandler)
    }
    unsafe fn virtualMachine(&self) -> JSVirtualMachine
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, virtualMachine)
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
    unsafe fn isInspectable(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isInspectable)
    }
    unsafe fn setInspectable_(&self, inspectable: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInspectable : inspectable)
    }
    unsafe fn currentContext() -> JSContext
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"JSContext").unwrap(), currentContext)
    }
    unsafe fn currentCallee() -> JSValue
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"JSContext").unwrap(), currentCallee)
    }
    unsafe fn currentThis() -> JSValue
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"JSContext").unwrap(), currentThis)
    }
    unsafe fn currentArguments() -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"JSContext").unwrap(), currentArguments)
    }
}
impl JSContext_SubscriptSupport for JSContext {}
pub trait JSContext_SubscriptSupport: Sized + std::ops::Deref {
    unsafe fn objectForKeyedSubscript_(&self, key: id) -> JSValue
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, objectForKeyedSubscript : key)
    }
    unsafe fn setObject_forKeyedSubscript_(&self, object: id, key: NSObject)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setObject : object, forKeyedSubscript : key)
    }
}
impl JSContext_JSContextRefSupport for JSContext {}
pub trait JSContext_JSContextRefSupport: Sized + std::ops::Deref {
    unsafe fn JSGlobalContextRef(&self) -> JSGlobalContextRef
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, JSGlobalContextRef)
    }
    unsafe fn contextWithJSGlobalContextRef_(jsGlobalContextRef: JSGlobalContextRef) -> JSContext
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"JSContext").unwrap(), contextWithJSGlobalContextRef : jsGlobalContextRef)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct JSValue(pub id);
impl std::ops::Deref for JSValue {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for JSValue {}
impl JSValue {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"JSValue").unwrap(), alloc) })
    }
}
impl INSObject for JSValue {}
impl PNSObject for JSValue {}
impl std::convert::TryFrom<NSObject> for JSValue {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<JSValue, Self::Error> {
        let is_kind_of: bool = unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"JSValue").unwrap()) };
        if is_kind_of {
            Ok(JSValue(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to JSValue")
        }
    }
}
impl IJSValue for JSValue {}
pub trait IJSValue: Sized + std::ops::Deref {
    unsafe fn toObject(&self) -> id
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, toObject)
    }
    unsafe fn toObjectOfClass_(&self, expectedClass: Class) -> id
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, toObjectOfClass : expectedClass)
    }
    unsafe fn toBool(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, toBool)
    }
    unsafe fn toDouble(&self) -> f64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, toDouble)
    }
    unsafe fn toInt32(&self) -> i32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, toInt32)
    }
    unsafe fn toUInt32(&self) -> u32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, toUInt32)
    }
    unsafe fn toInt64(&self) -> i64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, toInt64)
    }
    unsafe fn toUInt64(&self) -> u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, toUInt64)
    }
    unsafe fn toNumber(&self) -> NSNumber
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, toNumber)
    }
    unsafe fn toString(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, toString)
    }
    unsafe fn toDate(&self) -> NSDate
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, toDate)
    }
    unsafe fn toArray(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, toArray)
    }
    unsafe fn toDictionary(&self) -> NSDictionary
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, toDictionary)
    }
    unsafe fn isInstanceOf_(&self, value: id) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, isInstanceOf : value)
    }
    unsafe fn isEqualToObject_(&self, value: id) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, isEqualToObject : value)
    }
    unsafe fn isEqualWithTypeCoercionToObject_(&self, value: id) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, isEqualWithTypeCoercionToObject : value)
    }
    unsafe fn compareJSValue_(&self, other: JSValue) -> JSRelationCondition
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, compareJSValue : other)
    }
    unsafe fn compareInt64_(&self, other: i64) -> JSRelationCondition
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, compareInt64 : other)
    }
    unsafe fn compareUInt64_(&self, other: u64) -> JSRelationCondition
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, compareUInt64 : other)
    }
    unsafe fn compareDouble_(&self, other: f64) -> JSRelationCondition
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, compareDouble : other)
    }
    unsafe fn callWithArguments_(&self, arguments: NSArray) -> JSValue
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, callWithArguments : arguments)
    }
    unsafe fn constructWithArguments_(&self, arguments: NSArray) -> JSValue
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, constructWithArguments : arguments)
    }
    unsafe fn invokeMethod_withArguments_(&self, method: NSString, arguments: NSArray) -> JSValue
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, invokeMethod : method, withArguments : arguments)
    }
    unsafe fn context(&self) -> JSContext
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, context)
    }
    unsafe fn isUndefined(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isUndefined)
    }
    unsafe fn isNull(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isNull)
    }
    unsafe fn isBoolean(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isBoolean)
    }
    unsafe fn isNumber(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isNumber)
    }
    unsafe fn isString(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isString)
    }
    unsafe fn isObject(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isObject)
    }
    unsafe fn isArray(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isArray)
    }
    unsafe fn isDate(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isDate)
    }
    unsafe fn isSymbol(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isSymbol)
    }
    unsafe fn isBigInt(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isBigInt)
    }
    unsafe fn valueWithObject_inContext_(value: id, context: JSContext) -> JSValue
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"JSValue").unwrap(), valueWithObject : value, inContext : context)
    }
    unsafe fn valueWithBool_inContext_(value: BOOL, context: JSContext) -> JSValue
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"JSValue").unwrap(), valueWithBool : value, inContext : context)
    }
    unsafe fn valueWithDouble_inContext_(value: f64, context: JSContext) -> JSValue
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"JSValue").unwrap(), valueWithDouble : value, inContext : context)
    }
    unsafe fn valueWithInt32_inContext_(value: i32, context: JSContext) -> JSValue
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"JSValue").unwrap(), valueWithInt32 : value, inContext : context)
    }
    unsafe fn valueWithUInt32_inContext_(value: u32, context: JSContext) -> JSValue
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"JSValue").unwrap(), valueWithUInt32 : value, inContext : context)
    }
    unsafe fn valueWithNewObjectInContext_(context: JSContext) -> JSValue
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"JSValue").unwrap(), valueWithNewObjectInContext : context)
    }
    unsafe fn valueWithNewArrayInContext_(context: JSContext) -> JSValue
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"JSValue").unwrap(), valueWithNewArrayInContext : context)
    }
    unsafe fn valueWithNewRegularExpressionFromPattern_flags_inContext_(
        pattern: NSString,
        flags: NSString,
        context: JSContext,
    ) -> JSValue
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"JSValue").unwrap(), valueWithNewRegularExpressionFromPattern : pattern, flags : flags, inContext : context)
    }
    unsafe fn valueWithNewErrorFromMessage_inContext_(
        message: NSString,
        context: JSContext,
    ) -> JSValue
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"JSValue").unwrap(), valueWithNewErrorFromMessage : message, inContext : context)
    }
    unsafe fn valueWithNewPromiseInContext_fromExecutor_(
        context: JSContext,
        callback: *mut ::std::os::raw::c_void,
    ) -> JSValue
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"JSValue").unwrap(), valueWithNewPromiseInContext : context, fromExecutor : callback)
    }
    unsafe fn valueWithNewPromiseResolvedWithResult_inContext_(
        result: id,
        context: JSContext,
    ) -> JSValue
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"JSValue").unwrap(), valueWithNewPromiseResolvedWithResult : result, inContext : context)
    }
    unsafe fn valueWithNewPromiseRejectedWithReason_inContext_(
        reason: id,
        context: JSContext,
    ) -> JSValue
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"JSValue").unwrap(), valueWithNewPromiseRejectedWithReason : reason, inContext : context)
    }
    unsafe fn valueWithNewSymbolFromDescription_inContext_(
        description: NSString,
        context: JSContext,
    ) -> JSValue
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"JSValue").unwrap(), valueWithNewSymbolFromDescription : description, inContext : context)
    }
    unsafe fn valueWithNewBigIntFromString_inContext_(
        string: NSString,
        context: JSContext,
    ) -> JSValue
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"JSValue").unwrap(), valueWithNewBigIntFromString : string, inContext : context)
    }
    unsafe fn valueWithNewBigIntFromInt64_inContext_(int64: i64, context: JSContext) -> JSValue
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"JSValue").unwrap(), valueWithNewBigIntFromInt64 : int64, inContext : context)
    }
    unsafe fn valueWithNewBigIntFromUInt64_inContext_(uint64: u64, context: JSContext) -> JSValue
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"JSValue").unwrap(), valueWithNewBigIntFromUInt64 : uint64, inContext : context)
    }
    unsafe fn valueWithNewBigIntFromDouble_inContext_(value: f64, context: JSContext) -> JSValue
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"JSValue").unwrap(), valueWithNewBigIntFromDouble : value, inContext : context)
    }
    unsafe fn valueWithNullInContext_(context: JSContext) -> JSValue
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"JSValue").unwrap(), valueWithNullInContext : context)
    }
    unsafe fn valueWithUndefinedInContext_(context: JSContext) -> JSValue
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"JSValue").unwrap(), valueWithUndefinedInContext : context)
    }
}
impl JSValue_StructSupport for JSValue {}
pub trait JSValue_StructSupport: Sized + std::ops::Deref {
    unsafe fn toPoint(&self) -> CGPoint
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, toPoint)
    }
    unsafe fn toRange(&self) -> NSRange
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, toRange)
    }
    unsafe fn toRect(&self) -> CGRect
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, toRect)
    }
    unsafe fn toSize(&self) -> CGSize
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, toSize)
    }
    unsafe fn valueWithPoint_inContext_(point: CGPoint, context: JSContext) -> JSValue
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"JSValue").unwrap(), valueWithPoint : point, inContext : context)
    }
    unsafe fn valueWithRange_inContext_(range: NSRange, context: JSContext) -> JSValue
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"JSValue").unwrap(), valueWithRange : range, inContext : context)
    }
    unsafe fn valueWithRect_inContext_(rect: CGRect, context: JSContext) -> JSValue
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"JSValue").unwrap(), valueWithRect : rect, inContext : context)
    }
    unsafe fn valueWithSize_inContext_(size: CGSize, context: JSContext) -> JSValue
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"JSValue").unwrap(), valueWithSize : size, inContext : context)
    }
}
impl JSValue_PropertyAccess for JSValue {}
pub trait JSValue_PropertyAccess: Sized + std::ops::Deref {
    unsafe fn valueForProperty_(&self, property: id) -> JSValue
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, valueForProperty : property)
    }
    unsafe fn setValue_forProperty_(&self, value: id, property: id)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setValue : value, forProperty : property)
    }
    unsafe fn deleteProperty_(&self, property: id) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, deleteProperty : property)
    }
    unsafe fn hasProperty_(&self, property: id) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, hasProperty : property)
    }
    unsafe fn defineProperty_descriptor_(&self, property: id, descriptor: id)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, defineProperty : property, descriptor : descriptor)
    }
    unsafe fn valueAtIndex_(&self, index: NSUInteger) -> JSValue
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, valueAtIndex : index)
    }
    unsafe fn setValue_atIndex_(&self, value: id, index: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setValue : value, atIndex : index)
    }
}
impl JSValue_SubscriptSupport for JSValue {}
pub trait JSValue_SubscriptSupport: Sized + std::ops::Deref {
    unsafe fn objectForKeyedSubscript_(&self, key: id) -> JSValue
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, objectForKeyedSubscript : key)
    }
    unsafe fn objectAtIndexedSubscript_(&self, index: NSUInteger) -> JSValue
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, objectAtIndexedSubscript : index)
    }
    unsafe fn setObject_forKeyedSubscript_(&self, object: id, key: id)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setObject : object, forKeyedSubscript : key)
    }
    unsafe fn setObject_atIndexedSubscript_(&self, object: id, index: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setObject : object, atIndexedSubscript : index)
    }
}
impl JSValue_JSValueRefSupport for JSValue {}
pub trait JSValue_JSValueRefSupport: Sized + std::ops::Deref {
    unsafe fn JSValueRef(&self) -> JSValueRef
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, JSValueRef)
    }
    unsafe fn valueWithJSValueRef_inContext_(value: JSValueRef, context: JSContext) -> JSValue
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"JSValue").unwrap(), valueWithJSValueRef : value, inContext : context)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct JSManagedValue(pub id);
impl std::ops::Deref for JSManagedValue {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for JSManagedValue {}
impl JSManagedValue {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"JSManagedValue").unwrap(), alloc) })
    }
}
impl INSObject for JSManagedValue {}
impl PNSObject for JSManagedValue {}
impl std::convert::TryFrom<NSObject> for JSManagedValue {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<JSManagedValue, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"JSManagedValue").unwrap()) };
        if is_kind_of {
            Ok(JSManagedValue(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to JSManagedValue")
        }
    }
}
impl IJSManagedValue for JSManagedValue {}
pub trait IJSManagedValue: Sized + std::ops::Deref {
    unsafe fn initWithValue_(&self, value: JSValue) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithValue : value)
    }
    unsafe fn value(&self) -> JSValue
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, value)
    }
    unsafe fn managedValueWithValue_(value: JSValue) -> JSManagedValue
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"JSManagedValue").unwrap(), managedValueWithValue : value)
    }
    unsafe fn managedValueWithValue_andOwner_(value: JSValue, owner: id) -> JSManagedValue
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"JSManagedValue").unwrap(), managedValueWithValue : value, andOwner : owner)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct JSVirtualMachine(pub id);
impl std::ops::Deref for JSVirtualMachine {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for JSVirtualMachine {}
impl JSVirtualMachine {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"JSVirtualMachine").unwrap(), alloc) })
    }
}
impl INSObject for JSVirtualMachine {}
impl PNSObject for JSVirtualMachine {}
impl std::convert::TryFrom<NSObject> for JSVirtualMachine {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<JSVirtualMachine, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"JSVirtualMachine").unwrap()) };
        if is_kind_of {
            Ok(JSVirtualMachine(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to JSVirtualMachine")
        }
    }
}
impl IJSVirtualMachine for JSVirtualMachine {}
pub trait IJSVirtualMachine: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn addManagedReference_withOwner_(&self, object: id, owner: id)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, addManagedReference : object, withOwner : owner)
    }
    unsafe fn removeManagedReference_withOwner_(&self, object: id, owner: id)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, removeManagedReference : object, withOwner : owner)
    }
}
pub trait PJSExport: Sized + std::ops::Deref {}
unsafe extern "C" {
    pub fn JSEvaluateScript(
        ctx: JSContextRef,
        script: JSStringRef,
        thisObject: JSObjectRef,
        sourceURL: JSStringRef,
        startingLineNumber: ::std::os::raw::c_int,
        exception: *mut JSValueRef,
    ) -> JSValueRef;
}
unsafe extern "C" {
    pub fn JSCheckScriptSyntax(
        ctx: JSContextRef,
        script: JSStringRef,
        sourceURL: JSStringRef,
        startingLineNumber: ::std::os::raw::c_int,
        exception: *mut JSValueRef,
    ) -> bool;
}
unsafe extern "C" {
    pub fn JSGarbageCollect(ctx: JSContextRef);
}
unsafe extern "C" {
    pub fn JSValueGetType(ctx: JSContextRef, value: JSValueRef) -> JSType;
}
unsafe extern "C" {
    pub fn JSValueIsUndefined(ctx: JSContextRef, value: JSValueRef) -> bool;
}
unsafe extern "C" {
    pub fn JSValueIsNull(ctx: JSContextRef, value: JSValueRef) -> bool;
}
unsafe extern "C" {
    pub fn JSValueIsBoolean(ctx: JSContextRef, value: JSValueRef) -> bool;
}
unsafe extern "C" {
    pub fn JSValueIsNumber(ctx: JSContextRef, value: JSValueRef) -> bool;
}
unsafe extern "C" {
    pub fn JSValueIsString(ctx: JSContextRef, value: JSValueRef) -> bool;
}
unsafe extern "C" {
    pub fn JSValueIsSymbol(ctx: JSContextRef, value: JSValueRef) -> bool;
}
unsafe extern "C" {
    pub fn JSValueIsBigInt(ctx: JSContextRef, value: JSValueRef) -> bool;
}
unsafe extern "C" {
    pub fn JSValueIsObject(ctx: JSContextRef, value: JSValueRef) -> bool;
}
unsafe extern "C" {
    pub fn JSValueIsObjectOfClass(
        ctx: JSContextRef,
        value: JSValueRef,
        jsClass: JSClassRef,
    ) -> bool;
}
unsafe extern "C" {
    pub fn JSValueIsArray(ctx: JSContextRef, value: JSValueRef) -> bool;
}
unsafe extern "C" {
    pub fn JSValueIsDate(ctx: JSContextRef, value: JSValueRef) -> bool;
}
unsafe extern "C" {
    pub fn JSValueGetTypedArrayType(
        ctx: JSContextRef,
        value: JSValueRef,
        exception: *mut JSValueRef,
    ) -> JSTypedArrayType;
}
unsafe extern "C" {
    pub fn JSValueIsEqual(
        ctx: JSContextRef,
        a: JSValueRef,
        b: JSValueRef,
        exception: *mut JSValueRef,
    ) -> bool;
}
unsafe extern "C" {
    pub fn JSValueIsStrictEqual(ctx: JSContextRef, a: JSValueRef, b: JSValueRef) -> bool;
}
unsafe extern "C" {
    pub fn JSValueIsInstanceOfConstructor(
        ctx: JSContextRef,
        value: JSValueRef,
        constructor: JSObjectRef,
        exception: *mut JSValueRef,
    ) -> bool;
}
unsafe extern "C" {
    pub fn JSValueCompare(
        ctx: JSContextRef,
        left: JSValueRef,
        right: JSValueRef,
        exception: *mut JSValueRef,
    ) -> JSRelationCondition;
}
unsafe extern "C" {
    pub fn JSValueCompareInt64(
        ctx: JSContextRef,
        left: JSValueRef,
        right: i64,
        exception: *mut JSValueRef,
    ) -> JSRelationCondition;
}
unsafe extern "C" {
    pub fn JSValueCompareUInt64(
        ctx: JSContextRef,
        left: JSValueRef,
        right: u64,
        exception: *mut JSValueRef,
    ) -> JSRelationCondition;
}
unsafe extern "C" {
    pub fn JSValueCompareDouble(
        ctx: JSContextRef,
        left: JSValueRef,
        right: f64,
        exception: *mut JSValueRef,
    ) -> JSRelationCondition;
}
unsafe extern "C" {
    pub fn JSValueMakeUndefined(ctx: JSContextRef) -> JSValueRef;
}
unsafe extern "C" {
    pub fn JSValueMakeNull(ctx: JSContextRef) -> JSValueRef;
}
unsafe extern "C" {
    pub fn JSValueMakeBoolean(ctx: JSContextRef, boolean: bool) -> JSValueRef;
}
unsafe extern "C" {
    pub fn JSValueMakeNumber(ctx: JSContextRef, number: f64) -> JSValueRef;
}
unsafe extern "C" {
    pub fn JSValueMakeString(ctx: JSContextRef, string: JSStringRef) -> JSValueRef;
}
unsafe extern "C" {
    pub fn JSValueMakeSymbol(ctx: JSContextRef, description: JSStringRef) -> JSValueRef;
}
unsafe extern "C" {
    pub fn JSBigIntCreateWithDouble(
        ctx: JSContextRef,
        value: f64,
        exception: *mut JSValueRef,
    ) -> JSValueRef;
}
unsafe extern "C" {
    pub fn JSBigIntCreateWithInt64(
        ctx: JSContextRef,
        integer: i64,
        exception: *mut JSValueRef,
    ) -> JSValueRef;
}
unsafe extern "C" {
    pub fn JSBigIntCreateWithUInt64(
        ctx: JSContextRef,
        integer: u64,
        exception: *mut JSValueRef,
    ) -> JSValueRef;
}
unsafe extern "C" {
    pub fn JSBigIntCreateWithString(
        ctx: JSContextRef,
        string: JSStringRef,
        exception: *mut JSValueRef,
    ) -> JSValueRef;
}
unsafe extern "C" {
    pub fn JSValueMakeFromJSONString(ctx: JSContextRef, string: JSStringRef) -> JSValueRef;
}
unsafe extern "C" {
    pub fn JSValueCreateJSONString(
        ctx: JSContextRef,
        value: JSValueRef,
        indent: ::std::os::raw::c_uint,
        exception: *mut JSValueRef,
    ) -> JSStringRef;
}
unsafe extern "C" {
    pub fn JSValueToBoolean(ctx: JSContextRef, value: JSValueRef) -> bool;
}
unsafe extern "C" {
    pub fn JSValueToNumber(ctx: JSContextRef, value: JSValueRef, exception: *mut JSValueRef)
        -> f64;
}
unsafe extern "C" {
    pub fn JSValueToInt32(ctx: JSContextRef, value: JSValueRef, exception: *mut JSValueRef) -> i32;
}
unsafe extern "C" {
    pub fn JSValueToUInt32(ctx: JSContextRef, value: JSValueRef, exception: *mut JSValueRef)
        -> u32;
}
unsafe extern "C" {
    pub fn JSValueToInt64(ctx: JSContextRef, value: JSValueRef, exception: *mut JSValueRef) -> i64;
}
unsafe extern "C" {
    pub fn JSValueToUInt64(ctx: JSContextRef, value: JSValueRef, exception: *mut JSValueRef)
        -> u64;
}
unsafe extern "C" {
    pub fn JSValueToStringCopy(
        ctx: JSContextRef,
        value: JSValueRef,
        exception: *mut JSValueRef,
    ) -> JSStringRef;
}
unsafe extern "C" {
    pub fn JSValueToObject(
        ctx: JSContextRef,
        value: JSValueRef,
        exception: *mut JSValueRef,
    ) -> JSObjectRef;
}
unsafe extern "C" {
    pub fn JSValueProtect(ctx: JSContextRef, value: JSValueRef);
}
unsafe extern "C" {
    pub fn JSValueUnprotect(ctx: JSContextRef, value: JSValueRef);
}
unsafe extern "C" {
    pub static kJSClassDefinitionEmpty: JSClassDefinition;
}
unsafe extern "C" {
    pub fn JSClassCreate(definition: *const JSClassDefinition) -> JSClassRef;
}
unsafe extern "C" {
    pub fn JSClassRetain(jsClass: JSClassRef) -> JSClassRef;
}
unsafe extern "C" {
    pub fn JSClassRelease(jsClass: JSClassRef);
}
unsafe extern "C" {
    pub fn JSObjectMake(
        ctx: JSContextRef,
        jsClass: JSClassRef,
        data: *mut ::std::os::raw::c_void,
    ) -> JSObjectRef;
}
unsafe extern "C" {
    pub fn JSObjectMakeFunctionWithCallback(
        ctx: JSContextRef,
        name: JSStringRef,
        callAsFunction: JSObjectCallAsFunctionCallback,
    ) -> JSObjectRef;
}
unsafe extern "C" {
    pub fn JSObjectMakeConstructor(
        ctx: JSContextRef,
        jsClass: JSClassRef,
        callAsConstructor: JSObjectCallAsConstructorCallback,
    ) -> JSObjectRef;
}
unsafe extern "C" {
    pub fn JSObjectMakeArray(
        ctx: JSContextRef,
        argumentCount: usize,
        arguments: *const JSValueRef,
        exception: *mut JSValueRef,
    ) -> JSObjectRef;
}
unsafe extern "C" {
    pub fn JSObjectMakeDate(
        ctx: JSContextRef,
        argumentCount: usize,
        arguments: *const JSValueRef,
        exception: *mut JSValueRef,
    ) -> JSObjectRef;
}
unsafe extern "C" {
    pub fn JSObjectMakeError(
        ctx: JSContextRef,
        argumentCount: usize,
        arguments: *const JSValueRef,
        exception: *mut JSValueRef,
    ) -> JSObjectRef;
}
unsafe extern "C" {
    pub fn JSObjectMakeRegExp(
        ctx: JSContextRef,
        argumentCount: usize,
        arguments: *const JSValueRef,
        exception: *mut JSValueRef,
    ) -> JSObjectRef;
}
unsafe extern "C" {
    pub fn JSObjectMakeDeferredPromise(
        ctx: JSContextRef,
        resolve: *mut JSObjectRef,
        reject: *mut JSObjectRef,
        exception: *mut JSValueRef,
    ) -> JSObjectRef;
}
unsafe extern "C" {
    pub fn JSObjectMakeFunction(
        ctx: JSContextRef,
        name: JSStringRef,
        parameterCount: ::std::os::raw::c_uint,
        parameterNames: *const JSStringRef,
        body: JSStringRef,
        sourceURL: JSStringRef,
        startingLineNumber: ::std::os::raw::c_int,
        exception: *mut JSValueRef,
    ) -> JSObjectRef;
}
unsafe extern "C" {
    pub fn JSObjectGetPrototype(ctx: JSContextRef, object: JSObjectRef) -> JSValueRef;
}
unsafe extern "C" {
    pub fn JSObjectSetPrototype(ctx: JSContextRef, object: JSObjectRef, value: JSValueRef);
}
unsafe extern "C" {
    pub fn JSObjectHasProperty(
        ctx: JSContextRef,
        object: JSObjectRef,
        propertyName: JSStringRef,
    ) -> bool;
}
unsafe extern "C" {
    pub fn JSObjectGetProperty(
        ctx: JSContextRef,
        object: JSObjectRef,
        propertyName: JSStringRef,
        exception: *mut JSValueRef,
    ) -> JSValueRef;
}
unsafe extern "C" {
    pub fn JSObjectSetProperty(
        ctx: JSContextRef,
        object: JSObjectRef,
        propertyName: JSStringRef,
        value: JSValueRef,
        attributes: JSPropertyAttributes,
        exception: *mut JSValueRef,
    );
}
unsafe extern "C" {
    pub fn JSObjectDeleteProperty(
        ctx: JSContextRef,
        object: JSObjectRef,
        propertyName: JSStringRef,
        exception: *mut JSValueRef,
    ) -> bool;
}
unsafe extern "C" {
    pub fn JSObjectHasPropertyForKey(
        ctx: JSContextRef,
        object: JSObjectRef,
        propertyKey: JSValueRef,
        exception: *mut JSValueRef,
    ) -> bool;
}
unsafe extern "C" {
    pub fn JSObjectGetPropertyForKey(
        ctx: JSContextRef,
        object: JSObjectRef,
        propertyKey: JSValueRef,
        exception: *mut JSValueRef,
    ) -> JSValueRef;
}
unsafe extern "C" {
    pub fn JSObjectSetPropertyForKey(
        ctx: JSContextRef,
        object: JSObjectRef,
        propertyKey: JSValueRef,
        value: JSValueRef,
        attributes: JSPropertyAttributes,
        exception: *mut JSValueRef,
    );
}
unsafe extern "C" {
    pub fn JSObjectDeletePropertyForKey(
        ctx: JSContextRef,
        object: JSObjectRef,
        propertyKey: JSValueRef,
        exception: *mut JSValueRef,
    ) -> bool;
}
unsafe extern "C" {
    pub fn JSObjectGetPropertyAtIndex(
        ctx: JSContextRef,
        object: JSObjectRef,
        propertyIndex: ::std::os::raw::c_uint,
        exception: *mut JSValueRef,
    ) -> JSValueRef;
}
unsafe extern "C" {
    pub fn JSObjectSetPropertyAtIndex(
        ctx: JSContextRef,
        object: JSObjectRef,
        propertyIndex: ::std::os::raw::c_uint,
        value: JSValueRef,
        exception: *mut JSValueRef,
    );
}
unsafe extern "C" {
    pub fn JSObjectGetPrivate(object: JSObjectRef) -> *mut ::std::os::raw::c_void;
}
unsafe extern "C" {
    pub fn JSObjectSetPrivate(object: JSObjectRef, data: *mut ::std::os::raw::c_void) -> bool;
}
unsafe extern "C" {
    pub fn JSObjectIsFunction(ctx: JSContextRef, object: JSObjectRef) -> bool;
}
unsafe extern "C" {
    pub fn JSObjectCallAsFunction(
        ctx: JSContextRef,
        object: JSObjectRef,
        thisObject: JSObjectRef,
        argumentCount: usize,
        arguments: *const JSValueRef,
        exception: *mut JSValueRef,
    ) -> JSValueRef;
}
unsafe extern "C" {
    pub fn JSObjectIsConstructor(ctx: JSContextRef, object: JSObjectRef) -> bool;
}
unsafe extern "C" {
    pub fn JSObjectCallAsConstructor(
        ctx: JSContextRef,
        object: JSObjectRef,
        argumentCount: usize,
        arguments: *const JSValueRef,
        exception: *mut JSValueRef,
    ) -> JSObjectRef;
}
unsafe extern "C" {
    pub fn JSObjectCopyPropertyNames(
        ctx: JSContextRef,
        object: JSObjectRef,
    ) -> JSPropertyNameArrayRef;
}
unsafe extern "C" {
    pub fn JSPropertyNameArrayRetain(array: JSPropertyNameArrayRef) -> JSPropertyNameArrayRef;
}
unsafe extern "C" {
    pub fn JSPropertyNameArrayRelease(array: JSPropertyNameArrayRef);
}
unsafe extern "C" {
    pub fn JSPropertyNameArrayGetCount(array: JSPropertyNameArrayRef) -> usize;
}
unsafe extern "C" {
    pub fn JSPropertyNameArrayGetNameAtIndex(
        array: JSPropertyNameArrayRef,
        index: usize,
    ) -> JSStringRef;
}
unsafe extern "C" {
    pub fn JSPropertyNameAccumulatorAddName(
        accumulator: JSPropertyNameAccumulatorRef,
        propertyName: JSStringRef,
    );
}
unsafe extern "C" {
    pub fn JSContextGroupCreate() -> JSContextGroupRef;
}
unsafe extern "C" {
    pub fn JSContextGroupRetain(group: JSContextGroupRef) -> JSContextGroupRef;
}
unsafe extern "C" {
    pub fn JSContextGroupRelease(group: JSContextGroupRef);
}
unsafe extern "C" {
    pub fn JSGlobalContextCreate(globalObjectClass: JSClassRef) -> JSGlobalContextRef;
}
unsafe extern "C" {
    pub fn JSGlobalContextCreateInGroup(
        group: JSContextGroupRef,
        globalObjectClass: JSClassRef,
    ) -> JSGlobalContextRef;
}
unsafe extern "C" {
    pub fn JSGlobalContextRetain(ctx: JSGlobalContextRef) -> JSGlobalContextRef;
}
unsafe extern "C" {
    pub fn JSGlobalContextRelease(ctx: JSGlobalContextRef);
}
unsafe extern "C" {
    pub fn JSContextGetGlobalObject(ctx: JSContextRef) -> JSObjectRef;
}
unsafe extern "C" {
    pub fn JSContextGetGroup(ctx: JSContextRef) -> JSContextGroupRef;
}
unsafe extern "C" {
    pub fn JSContextGetGlobalContext(ctx: JSContextRef) -> JSGlobalContextRef;
}
unsafe extern "C" {
    pub fn JSGlobalContextCopyName(ctx: JSGlobalContextRef) -> JSStringRef;
}
unsafe extern "C" {
    pub fn JSGlobalContextSetName(ctx: JSGlobalContextRef, name: JSStringRef);
}
unsafe extern "C" {
    pub fn JSGlobalContextIsInspectable(ctx: JSGlobalContextRef) -> bool;
}
unsafe extern "C" {
    pub fn JSGlobalContextSetInspectable(ctx: JSGlobalContextRef, inspectable: bool);
}
unsafe extern "C" {
    pub fn JSStringCreateWithCharacters(chars: *const JSChar, numChars: usize) -> JSStringRef;
}
unsafe extern "C" {
    pub fn JSStringCreateWithUTF8CString(string: *const ::std::os::raw::c_char) -> JSStringRef;
}
unsafe extern "C" {
    pub fn JSStringRetain(string: JSStringRef) -> JSStringRef;
}
unsafe extern "C" {
    pub fn JSStringRelease(string: JSStringRef);
}
unsafe extern "C" {
    pub fn JSStringGetLength(string: JSStringRef) -> usize;
}
unsafe extern "C" {
    pub fn JSStringGetCharactersPtr(string: JSStringRef) -> *const JSChar;
}
unsafe extern "C" {
    pub fn JSStringGetMaximumUTF8CStringSize(string: JSStringRef) -> usize;
}
unsafe extern "C" {
    pub fn JSStringGetUTF8CString(
        string: JSStringRef,
        buffer: *mut ::std::os::raw::c_char,
        bufferSize: usize,
    ) -> usize;
}
unsafe extern "C" {
    pub fn JSStringIsEqual(a: JSStringRef, b: JSStringRef) -> bool;
}
unsafe extern "C" {
    pub fn JSStringIsEqualToUTF8CString(a: JSStringRef, b: *const ::std::os::raw::c_char) -> bool;
}
unsafe extern "C" {
    pub fn JSObjectMakeTypedArray(
        ctx: JSContextRef,
        arrayType: JSTypedArrayType,
        length: usize,
        exception: *mut JSValueRef,
    ) -> JSObjectRef;
}
unsafe extern "C" {
    pub fn JSObjectMakeTypedArrayWithBytesNoCopy(
        ctx: JSContextRef,
        arrayType: JSTypedArrayType,
        bytes: *mut ::std::os::raw::c_void,
        byteLength: usize,
        bytesDeallocator: JSTypedArrayBytesDeallocator,
        deallocatorContext: *mut ::std::os::raw::c_void,
        exception: *mut JSValueRef,
    ) -> JSObjectRef;
}
unsafe extern "C" {
    pub fn JSObjectMakeTypedArrayWithArrayBuffer(
        ctx: JSContextRef,
        arrayType: JSTypedArrayType,
        buffer: JSObjectRef,
        exception: *mut JSValueRef,
    ) -> JSObjectRef;
}
unsafe extern "C" {
    pub fn JSObjectMakeTypedArrayWithArrayBufferAndOffset(
        ctx: JSContextRef,
        arrayType: JSTypedArrayType,
        buffer: JSObjectRef,
        byteOffset: usize,
        length: usize,
        exception: *mut JSValueRef,
    ) -> JSObjectRef;
}
unsafe extern "C" {
    pub fn JSObjectGetTypedArrayBytesPtr(
        ctx: JSContextRef,
        object: JSObjectRef,
        exception: *mut JSValueRef,
    ) -> *mut ::std::os::raw::c_void;
}
unsafe extern "C" {
    pub fn JSObjectGetTypedArrayLength(
        ctx: JSContextRef,
        object: JSObjectRef,
        exception: *mut JSValueRef,
    ) -> usize;
}
unsafe extern "C" {
    pub fn JSObjectGetTypedArrayByteLength(
        ctx: JSContextRef,
        object: JSObjectRef,
        exception: *mut JSValueRef,
    ) -> usize;
}
unsafe extern "C" {
    pub fn JSObjectGetTypedArrayByteOffset(
        ctx: JSContextRef,
        object: JSObjectRef,
        exception: *mut JSValueRef,
    ) -> usize;
}
unsafe extern "C" {
    pub fn JSObjectGetTypedArrayBuffer(
        ctx: JSContextRef,
        object: JSObjectRef,
        exception: *mut JSValueRef,
    ) -> JSObjectRef;
}
unsafe extern "C" {
    pub fn JSObjectMakeArrayBufferWithBytesNoCopy(
        ctx: JSContextRef,
        bytes: *mut ::std::os::raw::c_void,
        byteLength: usize,
        bytesDeallocator: JSTypedArrayBytesDeallocator,
        deallocatorContext: *mut ::std::os::raw::c_void,
        exception: *mut JSValueRef,
    ) -> JSObjectRef;
}
unsafe extern "C" {
    pub fn JSObjectGetArrayBufferBytesPtr(
        ctx: JSContextRef,
        object: JSObjectRef,
        exception: *mut JSValueRef,
    ) -> *mut ::std::os::raw::c_void;
}
unsafe extern "C" {
    pub fn JSObjectGetArrayBufferByteLength(
        ctx: JSContextRef,
        object: JSObjectRef,
        exception: *mut JSValueRef,
    ) -> usize;
}
unsafe extern "C" {
    pub fn JSStringCreateWithCFString(string: CFStringRef) -> JSStringRef;
}
unsafe extern "C" {
    pub fn JSStringCopyCFString(alloc: CFAllocatorRef, string: JSStringRef) -> CFStringRef;
}
unsafe extern "C" {
    pub static JSPropertyDescriptorWritableKey: NSString;
}
unsafe extern "C" {
    pub static JSPropertyDescriptorEnumerableKey: NSString;
}
unsafe extern "C" {
    pub static JSPropertyDescriptorConfigurableKey: NSString;
}
unsafe extern "C" {
    pub static JSPropertyDescriptorValueKey: NSString;
}
unsafe extern "C" {
    pub static JSPropertyDescriptorGetKey: NSString;
}
unsafe extern "C" {
    pub static JSPropertyDescriptorSetKey: NSString;
}

unsafe impl objc2::encode::RefEncode for OpaqueJSContextGroup {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for OpaqueJSContextGroup {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("OpaqueJSContextGroup", &[]);
}
unsafe impl objc2::encode::RefEncode for OpaqueJSContext {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for OpaqueJSContext {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("OpaqueJSContext", &[]);
}
unsafe impl objc2::encode::RefEncode for OpaqueJSString {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for OpaqueJSString {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("OpaqueJSString", &[]);
}
unsafe impl objc2::encode::RefEncode for OpaqueJSClass {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for OpaqueJSClass {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("OpaqueJSClass", &[]);
}
unsafe impl objc2::encode::RefEncode for OpaqueJSPropertyNameArray {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for OpaqueJSPropertyNameArray {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("OpaqueJSPropertyNameArray", &[]);
}
unsafe impl objc2::encode::RefEncode for OpaqueJSPropertyNameAccumulator {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for OpaqueJSPropertyNameAccumulator {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("OpaqueJSPropertyNameAccumulator", &[]);
}
unsafe impl objc2::encode::RefEncode for OpaqueJSValue {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for OpaqueJSValue {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("OpaqueJSValue", &[]);
}
unsafe impl objc2::encode::RefEncode for JSStaticValue {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for JSStaticValue {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("JSStaticValue", &[]);
}
unsafe impl objc2::encode::RefEncode for JSStaticFunction {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for JSStaticFunction {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("JSStaticFunction", &[]);
}
unsafe impl objc2::encode::RefEncode for JSClassDefinition {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for JSClassDefinition {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("JSClassDefinition", &[]);
}
unsafe impl objc2::encode::RefEncode for JSContext {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for JSContext {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for JSValue {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for JSValue {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for JSManagedValue {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for JSManagedValue {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for JSVirtualMachine {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for JSVirtualMachine {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
