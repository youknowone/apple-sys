#[allow(unused_imports)]
use crate::CoreFoundation::*;

#[allow(unused_imports)]
use objc2::msg_send;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __LSMMap {
    _unused: [u8; 0],
}
pub type LSMMapRef = *mut __LSMMap;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __LSMText {
    _unused: [u8; 0],
}
pub type LSMTextRef = *mut __LSMText;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __LSMResult {
    _unused: [u8; 0],
}
pub type LSMResultRef = *mut __LSMResult;
pub type LSMCategory = u32;
unsafe extern "C" {
    pub fn LSMMapGetTypeID() -> CFTypeID;
}
unsafe extern "C" {
    pub fn LSMTextGetTypeID() -> CFTypeID;
}
unsafe extern "C" {
    pub fn LSMResultGetTypeID() -> CFTypeID;
}
unsafe extern "C" {
    pub fn LSMMapCreate(alloc: CFAllocatorRef, flags: CFOptionFlags) -> LSMMapRef;
}
unsafe extern "C" {
    pub fn LSMMapSetProperties(mapref: LSMMapRef, properties: CFDictionaryRef);
}
unsafe extern "C" {
    pub fn LSMMapGetProperties(mapref: LSMMapRef) -> CFDictionaryRef;
}
unsafe extern "C" {
    pub fn LSMMapStartTraining(mapref: LSMMapRef) -> OSStatus;
}
unsafe extern "C" {
    pub fn LSMMapAddCategory(mapref: LSMMapRef) -> LSMCategory;
}
unsafe extern "C" {
    pub fn LSMMapGetCategoryCount(mapref: LSMMapRef) -> CFIndex;
}
unsafe extern "C" {
    pub fn LSMMapSetStopWords(mapref: LSMMapRef, textref: LSMTextRef) -> OSStatus;
}
unsafe extern "C" {
    pub fn LSMMapAddText(mapref: LSMMapRef, textref: LSMTextRef, category: LSMCategory)
        -> OSStatus;
}
unsafe extern "C" {
    pub fn LSMMapAddTextWithWeight(
        mapref: LSMMapRef,
        textref: LSMTextRef,
        category: LSMCategory,
        weight: f32,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn LSMMapCompile(mapref: LSMMapRef) -> OSStatus;
}
unsafe extern "C" {
    pub fn LSMMapCreateClusters(
        alloc: CFAllocatorRef,
        mapref: LSMMapRef,
        subset: CFArrayRef,
        numClusters: CFIndex,
        flags: CFOptionFlags,
    ) -> CFArrayRef;
}
unsafe extern "C" {
    pub fn LSMMapApplyClusters(mapref: LSMMapRef, clusters: CFArrayRef) -> OSStatus;
}
unsafe extern "C" {
    pub fn LSMResultCreate(
        alloc: CFAllocatorRef,
        mapref: LSMMapRef,
        textref: LSMTextRef,
        numResults: CFIndex,
        flags: CFOptionFlags,
    ) -> LSMResultRef;
}
unsafe extern "C" {
    pub fn LSMResultGetCount(result: LSMResultRef) -> CFIndex;
}
unsafe extern "C" {
    pub fn LSMResultGetCategory(result: LSMResultRef, n: CFIndex) -> LSMCategory;
}
unsafe extern "C" {
    pub fn LSMResultGetScore(result: LSMResultRef, n: CFIndex) -> f32;
}
unsafe extern "C" {
    pub fn LSMResultCopyWord(result: LSMResultRef, n: CFIndex) -> CFStringRef;
}
unsafe extern "C" {
    pub fn LSMResultCopyToken(result: LSMResultRef, n: CFIndex) -> CFDataRef;
}
unsafe extern "C" {
    pub fn LSMResultCopyWordCluster(result: LSMResultRef, n: CFIndex) -> CFArrayRef;
}
unsafe extern "C" {
    pub fn LSMResultCopyTokenCluster(result: LSMResultRef, n: CFIndex) -> CFArrayRef;
}
unsafe extern "C" {
    pub fn LSMMapWriteToURL(mapref: LSMMapRef, file: CFURLRef, flags: CFOptionFlags) -> OSStatus;
}
unsafe extern "C" {
    pub fn LSMMapCreateFromURL(
        alloc: CFAllocatorRef,
        file: CFURLRef,
        flags: CFOptionFlags,
    ) -> LSMMapRef;
}
unsafe extern "C" {
    pub fn LSMMapWriteToStream(
        mapref: LSMMapRef,
        textref: LSMTextRef,
        stream: CFWriteStreamRef,
        options: CFOptionFlags,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn LSMTextCreate(alloc: CFAllocatorRef, mapref: LSMMapRef) -> LSMTextRef;
}
unsafe extern "C" {
    pub fn LSMTextAddWord(textref: LSMTextRef, word: CFStringRef) -> OSStatus;
}
unsafe extern "C" {
    pub fn LSMTextAddWords(
        textref: LSMTextRef,
        words: CFStringRef,
        locale: CFLocaleRef,
        flags: CFOptionFlags,
    ) -> OSStatus;
}
unsafe extern "C" {
    pub fn LSMTextAddToken(textref: LSMTextRef, token: CFDataRef) -> OSStatus;
}

unsafe impl objc2::encode::RefEncode for __LSMMap {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for __LSMMap {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("__LSMMap", &[]);
}
unsafe impl objc2::encode::RefEncode for __LSMText {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for __LSMText {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("__LSMText", &[]);
}
unsafe impl objc2::encode::RefEncode for __LSMResult {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for __LSMResult {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("__LSMResult", &[]);
}
