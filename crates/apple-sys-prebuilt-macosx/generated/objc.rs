#[allow(unused_imports)]
use objc2::msg_send;
#[allow(non_camel_case_types)]
pub type id = *mut objc2::runtime::AnyObject;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct objc_class {
    _unused: [u8; 0],
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct Class(pub id);
impl std::ops::Deref for Class {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for Class {}
impl Class {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"Class").unwrap(), alloc) })
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct objc_object {
    pub isa: Class,
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct Protocol(pub id);
impl std::ops::Deref for Protocol {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for Protocol {}
impl Protocol {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"Protocol").unwrap(), alloc) })
    }
}
impl IProtocol for Protocol {}
pub trait IProtocol: Sized + std::ops::Deref {}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct objc_selector {
    _unused: [u8; 0],
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct BOOL(pub bool);
pub type objc_objectptr_t = *const ::std::os::raw::c_void;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct NSObject(pub id);
impl std::ops::Deref for NSObject {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for NSObject {}
impl NSObject {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"NSObject").unwrap(), alloc) })
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct objc_method {
    _unused: [u8; 0],
}
pub type Method = *mut objc_method;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct objc_ivar {
    _unused: [u8; 0],
}
pub type Ivar = *mut objc_ivar;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct objc_category {
    _unused: [u8; 0],
}
pub type Category = *mut objc_category;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct objc_property {
    _unused: [u8; 0],
}
pub type objc_property_t = *mut objc_property;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct objc_property_attribute_t {
    pub name: *const ::std::os::raw::c_char,
    pub value: *const ::std::os::raw::c_char,
}
pub type objc_AssociationPolicy = usize;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct NXHashTablePrototype {
    pub hash: ::std::option::Option<
        unsafe extern "C" fn(
            info: *const ::std::os::raw::c_void,
            data: *const ::std::os::raw::c_void,
        ) -> usize,
    >,
    pub isEqual: ::std::option::Option<
        unsafe extern "C" fn(
            info: *const ::std::os::raw::c_void,
            data1: *const ::std::os::raw::c_void,
            data2: *const ::std::os::raw::c_void,
        ) -> ::std::os::raw::c_int,
    >,
    pub free: ::std::option::Option<
        unsafe extern "C" fn(
            info: *const ::std::os::raw::c_void,
            data: *mut ::std::os::raw::c_void,
        ),
    >,
    pub style: ::std::os::raw::c_int,
}
pub trait IClass: Sized + std::ops::Deref {}
impl IClass for Class {}

unsafe impl objc2::encode::RefEncode for objc_class {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for objc_class {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("objc_class", &[]);
}
unsafe impl objc2::encode::RefEncode for Class {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for Class {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for objc_object {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for objc_object {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("objc_object", &[]);
}
unsafe impl objc2::encode::RefEncode for Protocol {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for Protocol {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for objc_selector {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for objc_selector {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("objc_selector", &[]);
}
unsafe impl objc2::encode::RefEncode for BOOL {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for BOOL {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("BOOL", &[]);
}
unsafe impl objc2::encode::RefEncode for NSObject {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for NSObject {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for objc_method {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for objc_method {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("objc_method", &[]);
}
unsafe impl objc2::encode::RefEncode for objc_ivar {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for objc_ivar {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("objc_ivar", &[]);
}
unsafe impl objc2::encode::RefEncode for objc_category {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for objc_category {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("objc_category", &[]);
}
unsafe impl objc2::encode::RefEncode for objc_property {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for objc_property {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("objc_property", &[]);
}
unsafe impl objc2::encode::RefEncode for objc_property_attribute_t {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for objc_property_attribute_t {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("objc_property_attribute_t", &[]);
}
unsafe impl objc2::encode::RefEncode for NXHashTablePrototype {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for NXHashTablePrototype {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("NXHashTablePrototype", &[]);
}
