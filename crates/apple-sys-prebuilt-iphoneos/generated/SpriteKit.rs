#[allow(unused_imports)]
use crate::objc::*;
#[allow(unused_imports)]
use crate::AVFAudio::*;
#[allow(unused_imports)]
use crate::AVFoundation::*;
#[allow(unused_imports)]
use crate::AddressBook::*;
#[allow(unused_imports)]
use crate::CoreGraphics::*;
#[allow(unused_imports)]
use crate::CoreImage::*;
#[allow(unused_imports)]
use crate::Foundation::*;
#[allow(unused_imports)]
use crate::GLKit::*;
#[allow(unused_imports)]
use crate::Metal::*;
#[allow(unused_imports)]
use crate::ModelIO::*;
#[allow(unused_imports)]
use crate::SceneKit::*;
#[allow(unused_imports)]
use crate::Security::*;
#[allow(unused_imports)]
use crate::UIKit::*;
#[allow(unused_imports)]
use libc::size_t;

#[allow(unused_imports)]
use objc2::msg_send;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct simd_float2x2 {
    pub columns: [simd_float2; 2usize],
}
pub type matrix_float2x2 = simd_float2x2;
pub type SKBlendMode = NSInteger;
pub type SKNodeFocusBehavior = NSInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct SKNode(pub id);
impl std::ops::Deref for SKNode {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for SKNode {}
impl SKNode {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"SKNode").unwrap(), alloc) })
    }
}
impl PNSCopying for SKNode {}
impl PNSSecureCoding for SKNode {}
impl INSResponder for SKNode {}
impl PNSCoding for SKNode {}
impl std::convert::TryFrom<NSResponder> for SKNode {
    type Error = &'static str;
    fn try_from(parent: NSResponder) -> Result<SKNode, Self::Error> {
        let is_kind_of: bool = unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"SKNode").unwrap()) };
        if is_kind_of {
            Ok(SKNode(parent.0))
        } else {
            Err("This NSResponder cannot be downcasted to SKNode")
        }
    }
}
impl INSObject for SKNode {}
impl PNSObject for SKNode {}
impl ISKNode for SKNode {}
pub trait ISKNode: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn initWithCoder_(&self, aDecoder: NSCoder) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithCoder : aDecoder)
    }
    unsafe fn calculateAccumulatedFrame(&self) -> CGRect
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, calculateAccumulatedFrame)
    }
    unsafe fn valueForAttributeNamed_(&self, key: NSString) -> SKAttributeValue
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, valueForAttributeNamed : key)
    }
    unsafe fn setValue_forAttributeNamed_(&self, value: SKAttributeValue, key: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setValue : value, forAttributeNamed : key)
    }
    unsafe fn setScale_(&self, scale: CGFloat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setScale : scale)
    }
    unsafe fn addChild_(&self, node: SKNode)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, addChild : node)
    }
    unsafe fn insertChild_atIndex_(&self, node: SKNode, index: NSInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, insertChild : node, atIndex : index)
    }
    unsafe fn removeChildrenInArray_(&self, nodes: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, removeChildrenInArray : nodes)
    }
    unsafe fn removeAllChildren(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, removeAllChildren)
    }
    unsafe fn removeFromParent(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, removeFromParent)
    }
    unsafe fn moveToParent_(&self, parent: SKNode)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, moveToParent : parent)
    }
    unsafe fn childNodeWithName_(&self, name: NSString) -> SKNode
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, childNodeWithName : name)
    }
    unsafe fn enumerateChildNodesWithName_usingBlock_(
        &self,
        name: NSString,
        block: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, enumerateChildNodesWithName : name, usingBlock : block)
    }
    unsafe fn objectForKeyedSubscript_(&self, name: NSString) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, objectForKeyedSubscript : name)
    }
    unsafe fn inParentHierarchy_(&self, parent: SKNode) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, inParentHierarchy : parent)
    }
    unsafe fn runAction_(&self, action: SKAction)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, runAction : action)
    }
    unsafe fn runAction_completion_(&self, action: SKAction, block: *mut ::std::os::raw::c_void)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, runAction : action, completion : block)
    }
    unsafe fn runAction_withKey_(&self, action: SKAction, key: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, runAction : action, withKey : key)
    }
    unsafe fn hasActions(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, hasActions)
    }
    unsafe fn actionForKey_(&self, key: NSString) -> SKAction
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, actionForKey : key)
    }
    unsafe fn removeActionForKey_(&self, key: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, removeActionForKey : key)
    }
    unsafe fn removeAllActions(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, removeAllActions)
    }
    unsafe fn containsPoint_(&self, p: CGPoint) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, containsPoint : p)
    }
    unsafe fn nodeAtPoint_(&self, p: CGPoint) -> SKNode
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, nodeAtPoint : p)
    }
    unsafe fn nodesAtPoint_(&self, p: CGPoint) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, nodesAtPoint : p)
    }
    unsafe fn convertPoint_fromNode_(&self, point: CGPoint, node: SKNode) -> CGPoint
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, convertPoint : point, fromNode : node)
    }
    unsafe fn convertPoint_toNode_(&self, point: CGPoint, node: SKNode) -> CGPoint
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, convertPoint : point, toNode : node)
    }
    unsafe fn intersectsNode_(&self, node: SKNode) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, intersectsNode : node)
    }
    unsafe fn isEqualToNode_(&self, node: SKNode) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, isEqualToNode : node)
    }
    unsafe fn frame(&self) -> CGRect
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, frame)
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
    unsafe fn zRotation(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, zRotation)
    }
    unsafe fn setZRotation_(&self, zRotation: CGFloat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setZRotation : zRotation)
    }
    unsafe fn xScale(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, xScale)
    }
    unsafe fn setXScale_(&self, xScale: CGFloat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setXScale : xScale)
    }
    unsafe fn yScale(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, yScale)
    }
    unsafe fn setYScale_(&self, yScale: CGFloat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setYScale : yScale)
    }
    unsafe fn speed(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, speed)
    }
    unsafe fn setSpeed_(&self, speed: CGFloat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSpeed : speed)
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
    unsafe fn isUserInteractionEnabled(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isUserInteractionEnabled)
    }
    unsafe fn setUserInteractionEnabled_(&self, userInteractionEnabled: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setUserInteractionEnabled : userInteractionEnabled)
    }
    unsafe fn focusBehavior(&self) -> SKNodeFocusBehavior
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, focusBehavior)
    }
    unsafe fn setFocusBehavior_(&self, focusBehavior: SKNodeFocusBehavior)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setFocusBehavior : focusBehavior)
    }
    unsafe fn parent(&self) -> SKNode
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, parent)
    }
    unsafe fn children(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, children)
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
    unsafe fn scene(&self) -> SKScene
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, scene)
    }
    unsafe fn physicsBody(&self) -> SKPhysicsBody
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, physicsBody)
    }
    unsafe fn setPhysicsBody_(&self, physicsBody: SKPhysicsBody)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPhysicsBody : physicsBody)
    }
    unsafe fn userData(&self) -> NSMutableDictionary
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, userData)
    }
    unsafe fn setUserData_(&self, userData: NSMutableDictionary)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setUserData : userData)
    }
    unsafe fn reachConstraints(&self) -> SKReachConstraints
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, reachConstraints)
    }
    unsafe fn setReachConstraints_(&self, reachConstraints: SKReachConstraints)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setReachConstraints : reachConstraints)
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
    unsafe fn attributeValues(&self) -> NSDictionary
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, attributeValues)
    }
    unsafe fn setAttributeValues_(&self, attributeValues: NSDictionary)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAttributeValues : attributeValues)
    }
    unsafe fn node() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SKNode").unwrap(), node)
    }
    unsafe fn nodeWithFileNamed_(filename: NSString) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SKNode").unwrap(), nodeWithFileNamed : filename)
    }
    unsafe fn nodeWithFileNamed_securelyWithClasses_andError_(
        filename: NSString,
        classes: NSSet,
        error: *mut NSError,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SKNode").unwrap(), nodeWithFileNamed : filename, securelyWithClasses : classes, andError : error)
    }
}
pub trait NSEvent_SKNodeEvent: Sized + std::ops::Deref {
    unsafe fn locationInNode_(&self, node: SKNode) -> CGPoint
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, locationInNode : node)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct SKCameraNode(pub id);
impl std::ops::Deref for SKCameraNode {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for SKCameraNode {}
impl SKCameraNode {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"SKCameraNode").unwrap(), alloc) })
    }
}
impl ISKNode for SKCameraNode {}
impl PNSCopying for SKCameraNode {}
impl PNSSecureCoding for SKCameraNode {}
impl From<SKCameraNode> for SKNode {
    fn from(child: SKCameraNode) -> SKNode {
        SKNode(child.0)
    }
}
impl std::convert::TryFrom<SKNode> for SKCameraNode {
    type Error = &'static str;
    fn try_from(parent: SKNode) -> Result<SKCameraNode, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"SKCameraNode").unwrap()) };
        if is_kind_of {
            Ok(SKCameraNode(parent.0))
        } else {
            Err("This SKNode cannot be downcasted to SKCameraNode")
        }
    }
}
impl INSResponder for SKCameraNode {}
impl PNSCoding for SKCameraNode {}
impl INSObject for SKCameraNode {}
impl PNSObject for SKCameraNode {}
impl ISKCameraNode for SKCameraNode {}
pub trait ISKCameraNode: Sized + std::ops::Deref {
    unsafe fn containsNode_(&self, node: SKNode) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, containsNode : node)
    }
    unsafe fn containedNodeSet(&self) -> NSSet
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, containedNodeSet)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct SKShader(pub id);
impl std::ops::Deref for SKShader {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for SKShader {}
impl SKShader {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"SKShader").unwrap(), alloc) })
    }
}
impl PNSCopying for SKShader {}
impl PNSSecureCoding for SKShader {}
impl INSObject for SKShader {}
impl PNSObject for SKShader {}
impl std::convert::TryFrom<NSObject> for SKShader {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<SKShader, Self::Error> {
        let is_kind_of: bool = unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"SKShader").unwrap()) };
        if is_kind_of {
            Ok(SKShader(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to SKShader")
        }
    }
}
impl ISKShader for SKShader {}
pub trait ISKShader: Sized + std::ops::Deref {
    unsafe fn initWithSource_(&self, source: NSString) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithSource : source)
    }
    unsafe fn initWithSource_uniforms_(&self, source: NSString, uniforms: NSArray) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithSource : source, uniforms : uniforms)
    }
    unsafe fn addUniform_(&self, uniform: SKUniform)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, addUniform : uniform)
    }
    unsafe fn uniformNamed_(&self, name: NSString) -> SKUniform
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, uniformNamed : name)
    }
    unsafe fn removeUniformNamed_(&self, name: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, removeUniformNamed : name)
    }
    unsafe fn source(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, source)
    }
    unsafe fn setSource_(&self, source: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSource : source)
    }
    unsafe fn uniforms(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, uniforms)
    }
    unsafe fn setUniforms_(&self, uniforms: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setUniforms : uniforms)
    }
    unsafe fn attributes(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, attributes)
    }
    unsafe fn setAttributes_(&self, attributes: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAttributes : attributes)
    }
    unsafe fn shader() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SKShader").unwrap(), shader)
    }
    unsafe fn shaderWithSource_(source: NSString) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SKShader").unwrap(), shaderWithSource : source)
    }
    unsafe fn shaderWithSource_uniforms_(source: NSString, uniforms: NSArray) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SKShader").unwrap(), shaderWithSource : source, uniforms : uniforms)
    }
    unsafe fn shaderWithFileNamed_(name: NSString) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SKShader").unwrap(), shaderWithFileNamed : name)
    }
}
pub type SKActionTimingMode = NSInteger;
pub type SKActionTimingFunction = *mut ::std::os::raw::c_void;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct SKAction(pub id);
impl std::ops::Deref for SKAction {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for SKAction {}
impl SKAction {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"SKAction").unwrap(), alloc) })
    }
}
impl PNSCopying for SKAction {}
impl PNSSecureCoding for SKAction {}
impl INSObject for SKAction {}
impl PNSObject for SKAction {}
impl std::convert::TryFrom<NSObject> for SKAction {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<SKAction, Self::Error> {
        let is_kind_of: bool = unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"SKAction").unwrap()) };
        if is_kind_of {
            Ok(SKAction(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to SKAction")
        }
    }
}
impl ISKAction for SKAction {}
pub trait ISKAction: Sized + std::ops::Deref {
    unsafe fn reversedAction(&self) -> SKAction
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, reversedAction)
    }
    unsafe fn duration(&self) -> NSTimeInterval
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, duration)
    }
    unsafe fn setDuration_(&self, duration: NSTimeInterval)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDuration : duration)
    }
    unsafe fn timingMode(&self) -> SKActionTimingMode
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, timingMode)
    }
    unsafe fn setTimingMode_(&self, timingMode: SKActionTimingMode)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setTimingMode : timingMode)
    }
    unsafe fn timingFunction(&self) -> SKActionTimingFunction
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, timingFunction)
    }
    unsafe fn setTimingFunction_(&self, timingFunction: SKActionTimingFunction)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setTimingFunction : timingFunction)
    }
    unsafe fn speed(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, speed)
    }
    unsafe fn setSpeed_(&self, speed: CGFloat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSpeed : speed)
    }
}
impl SKAction_SKActions for SKAction {}
pub trait SKAction_SKActions: Sized + std::ops::Deref {
    unsafe fn moveBy_duration_(delta: CGVector, duration: NSTimeInterval) -> SKAction
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SKAction").unwrap(), moveBy : delta, duration : duration)
    }
    unsafe fn moveByX_y_duration_(
        deltaX: CGFloat,
        deltaY: CGFloat,
        duration: NSTimeInterval,
    ) -> SKAction
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SKAction").unwrap(), moveByX : deltaX, y : deltaY, duration : duration)
    }
    unsafe fn moveTo_duration_(location: CGPoint, duration: NSTimeInterval) -> SKAction
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SKAction").unwrap(), moveTo : location, duration : duration)
    }
    unsafe fn moveToX_duration_(x: CGFloat, duration: NSTimeInterval) -> SKAction
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SKAction").unwrap(), moveToX : x, duration : duration)
    }
    unsafe fn moveToY_duration_(y: CGFloat, duration: NSTimeInterval) -> SKAction
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SKAction").unwrap(), moveToY : y, duration : duration)
    }
    unsafe fn rotateByAngle_duration_(radians: CGFloat, duration: NSTimeInterval) -> SKAction
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SKAction").unwrap(), rotateByAngle : radians, duration : duration)
    }
    unsafe fn rotateToAngle_duration_(radians: CGFloat, duration: NSTimeInterval) -> SKAction
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SKAction").unwrap(), rotateToAngle : radians, duration : duration)
    }
    unsafe fn rotateToAngle_duration_shortestUnitArc_(
        radians: CGFloat,
        duration: NSTimeInterval,
        shortestUnitArc: BOOL,
    ) -> SKAction
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SKAction").unwrap(), rotateToAngle : radians, duration : duration, shortestUnitArc : shortestUnitArc)
    }
    unsafe fn resizeByWidth_height_duration_(
        width: CGFloat,
        height: CGFloat,
        duration: NSTimeInterval,
    ) -> SKAction
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SKAction").unwrap(), resizeByWidth : width, height : height, duration : duration)
    }
    unsafe fn resizeToWidth_height_duration_(
        width: CGFloat,
        height: CGFloat,
        duration: NSTimeInterval,
    ) -> SKAction
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SKAction").unwrap(), resizeToWidth : width, height : height, duration : duration)
    }
    unsafe fn resizeToWidth_duration_(width: CGFloat, duration: NSTimeInterval) -> SKAction
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SKAction").unwrap(), resizeToWidth : width, duration : duration)
    }
    unsafe fn resizeToHeight_duration_(height: CGFloat, duration: NSTimeInterval) -> SKAction
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SKAction").unwrap(), resizeToHeight : height, duration : duration)
    }
    unsafe fn scaleBy_duration_(scale: CGFloat, duration: NSTimeInterval) -> SKAction
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SKAction").unwrap(), scaleBy : scale, duration : duration)
    }
    unsafe fn scaleXBy_y_duration_(
        xScale: CGFloat,
        yScale: CGFloat,
        duration: NSTimeInterval,
    ) -> SKAction
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SKAction").unwrap(), scaleXBy : xScale, y : yScale, duration : duration)
    }
    unsafe fn scaleTo_duration_(scale: CGFloat, duration: NSTimeInterval) -> SKAction
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SKAction").unwrap(), scaleTo : scale, duration : duration)
    }
    unsafe fn scaleXTo_y_duration_(
        xScale: CGFloat,
        yScale: CGFloat,
        duration: NSTimeInterval,
    ) -> SKAction
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SKAction").unwrap(), scaleXTo : xScale, y : yScale, duration : duration)
    }
    unsafe fn scaleXTo_duration_(scale: CGFloat, duration: NSTimeInterval) -> SKAction
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SKAction").unwrap(), scaleXTo : scale, duration : duration)
    }
    unsafe fn scaleYTo_duration_(scale: CGFloat, duration: NSTimeInterval) -> SKAction
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SKAction").unwrap(), scaleYTo : scale, duration : duration)
    }
    unsafe fn scaleToSize_duration_(size: CGSize, duration: NSTimeInterval) -> SKAction
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SKAction").unwrap(), scaleToSize : size, duration : duration)
    }
    unsafe fn sequence_(actions: NSArray) -> SKAction
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SKAction").unwrap(), sequence : actions)
    }
    unsafe fn group_(actions: NSArray) -> SKAction
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SKAction").unwrap(), group : actions)
    }
    unsafe fn repeatAction_count_(action: SKAction, count: NSUInteger) -> SKAction
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SKAction").unwrap(), repeatAction : action, count : count)
    }
    unsafe fn repeatActionForever_(action: SKAction) -> SKAction
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SKAction").unwrap(), repeatActionForever : action)
    }
    unsafe fn fadeInWithDuration_(duration: NSTimeInterval) -> SKAction
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SKAction").unwrap(), fadeInWithDuration : duration)
    }
    unsafe fn fadeOutWithDuration_(duration: NSTimeInterval) -> SKAction
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SKAction").unwrap(), fadeOutWithDuration : duration)
    }
    unsafe fn fadeAlphaBy_duration_(factor: CGFloat, duration: NSTimeInterval) -> SKAction
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SKAction").unwrap(), fadeAlphaBy : factor, duration : duration)
    }
    unsafe fn fadeAlphaTo_duration_(alpha: CGFloat, duration: NSTimeInterval) -> SKAction
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SKAction").unwrap(), fadeAlphaTo : alpha, duration : duration)
    }
    unsafe fn hide() -> SKAction
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SKAction").unwrap(), hide)
    }
    unsafe fn unhide() -> SKAction
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SKAction").unwrap(), unhide)
    }
    unsafe fn setTexture_(texture: SKTexture) -> SKAction
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SKAction").unwrap(), setTexture : texture)
    }
    unsafe fn setNormalTexture_(texture: SKTexture) -> SKAction
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SKAction").unwrap(), setNormalTexture : texture)
    }
    unsafe fn setTexture_resize_(texture: SKTexture, resize: BOOL) -> SKAction
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SKAction").unwrap(), setTexture : texture, resize : resize)
    }
    unsafe fn setNormalTexture_resize_(texture: SKTexture, resize: BOOL) -> SKAction
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SKAction").unwrap(), setNormalTexture : texture, resize : resize)
    }
    unsafe fn animateWithTextures_timePerFrame_(textures: NSArray, sec: NSTimeInterval) -> SKAction
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SKAction").unwrap(), animateWithTextures : textures, timePerFrame : sec)
    }
    unsafe fn animateWithNormalTextures_timePerFrame_(
        textures: NSArray,
        sec: NSTimeInterval,
    ) -> SKAction
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SKAction").unwrap(), animateWithNormalTextures : textures, timePerFrame : sec)
    }
    unsafe fn animateWithTextures_timePerFrame_resize_restore_(
        textures: NSArray,
        sec: NSTimeInterval,
        resize: BOOL,
        restore: BOOL,
    ) -> SKAction
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SKAction").unwrap(), animateWithTextures : textures, timePerFrame : sec, resize : resize, restore : restore)
    }
    unsafe fn animateWithNormalTextures_timePerFrame_resize_restore_(
        textures: NSArray,
        sec: NSTimeInterval,
        resize: BOOL,
        restore: BOOL,
    ) -> SKAction
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SKAction").unwrap(), animateWithNormalTextures : textures, timePerFrame : sec, resize : resize, restore : restore)
    }
    unsafe fn playSoundFileNamed_waitForCompletion_(soundFile: NSString, wait: BOOL) -> SKAction
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SKAction").unwrap(), playSoundFileNamed : soundFile, waitForCompletion : wait)
    }
    unsafe fn colorizeWithColor_colorBlendFactor_duration_(
        color: NSColor,
        colorBlendFactor: CGFloat,
        duration: NSTimeInterval,
    ) -> SKAction
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SKAction").unwrap(), colorizeWithColor : color, colorBlendFactor : colorBlendFactor, duration : duration)
    }
    unsafe fn colorizeWithColorBlendFactor_duration_(
        colorBlendFactor: CGFloat,
        sec: NSTimeInterval,
    ) -> SKAction
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SKAction").unwrap(), colorizeWithColorBlendFactor : colorBlendFactor, duration : sec)
    }
    unsafe fn falloffTo_duration_(falloff: f32, duration: NSTimeInterval) -> SKAction
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SKAction").unwrap(), falloffTo : falloff, duration : duration)
    }
    unsafe fn falloffBy_duration_(falloff: f32, duration: NSTimeInterval) -> SKAction
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SKAction").unwrap(), falloffBy : falloff, duration : duration)
    }
    unsafe fn followPath_duration_(path: CGPathRef, duration: NSTimeInterval) -> SKAction
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SKAction").unwrap(), followPath : path, duration : duration)
    }
    unsafe fn followPath_asOffset_orientToPath_duration_(
        path: CGPathRef,
        offset: BOOL,
        orient: BOOL,
        duration: NSTimeInterval,
    ) -> SKAction
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SKAction").unwrap(), followPath : path, asOffset : offset, orientToPath : orient, duration : duration)
    }
    unsafe fn followPath_speed_(path: CGPathRef, speed: CGFloat) -> SKAction
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SKAction").unwrap(), followPath : path, speed : speed)
    }
    unsafe fn followPath_asOffset_orientToPath_speed_(
        path: CGPathRef,
        offset: BOOL,
        orient: BOOL,
        speed: CGFloat,
    ) -> SKAction
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SKAction").unwrap(), followPath : path, asOffset : offset, orientToPath : orient, speed : speed)
    }
    unsafe fn speedBy_duration_(speed: CGFloat, duration: NSTimeInterval) -> SKAction
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SKAction").unwrap(), speedBy : speed, duration : duration)
    }
    unsafe fn speedTo_duration_(speed: CGFloat, duration: NSTimeInterval) -> SKAction
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SKAction").unwrap(), speedTo : speed, duration : duration)
    }
    unsafe fn reachTo_rootNode_duration_(
        position: CGPoint,
        root: SKNode,
        duration: NSTimeInterval,
    ) -> SKAction
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SKAction").unwrap(), reachTo : position, rootNode : root, duration : duration)
    }
    unsafe fn reachTo_rootNode_velocity_(
        position: CGPoint,
        root: SKNode,
        velocity: CGFloat,
    ) -> SKAction
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SKAction").unwrap(), reachTo : position, rootNode : root, velocity : velocity)
    }
    unsafe fn reachToNode_rootNode_duration_(
        node: SKNode,
        root: SKNode,
        sec: NSTimeInterval,
    ) -> SKAction
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SKAction").unwrap(), reachToNode : node, rootNode : root, duration : sec)
    }
    unsafe fn reachToNode_rootNode_velocity_(
        node: SKNode,
        root: SKNode,
        velocity: CGFloat,
    ) -> SKAction
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SKAction").unwrap(), reachToNode : node, rootNode : root, velocity : velocity)
    }
    unsafe fn strengthTo_duration_(strength: f32, duration: NSTimeInterval) -> SKAction
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SKAction").unwrap(), strengthTo : strength, duration : duration)
    }
    unsafe fn strengthBy_duration_(strength: f32, duration: NSTimeInterval) -> SKAction
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SKAction").unwrap(), strengthBy : strength, duration : duration)
    }
    unsafe fn waitForDuration_(duration: NSTimeInterval) -> SKAction
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SKAction").unwrap(), waitForDuration : duration)
    }
    unsafe fn waitForDuration_withRange_(
        duration: NSTimeInterval,
        durationRange: NSTimeInterval,
    ) -> SKAction
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SKAction").unwrap(), waitForDuration : duration, withRange : durationRange)
    }
    unsafe fn removeFromParent() -> SKAction
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SKAction").unwrap(), removeFromParent)
    }
    unsafe fn performSelector_onTarget_(selector: objc2::runtime::Sel, target: id) -> SKAction
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SKAction").unwrap(), performSelector : selector, onTarget : target)
    }
    unsafe fn runBlock_(block: dispatch_block_t) -> SKAction
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SKAction").unwrap(), runBlock : block)
    }
    unsafe fn runBlock_queue_(block: dispatch_block_t, queue: NSObject) -> SKAction
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SKAction").unwrap(), runBlock : block, queue : queue)
    }
    unsafe fn runAction_onChildWithName_(action: SKAction, name: NSString) -> SKAction
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SKAction").unwrap(), runAction : action, onChildWithName : name)
    }
    unsafe fn customActionWithDuration_actionBlock_(
        duration: NSTimeInterval,
        block: *mut ::std::os::raw::c_void,
    ) -> SKAction
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SKAction").unwrap(), customActionWithDuration : duration, actionBlock : block)
    }
    unsafe fn actionNamed_(name: NSString) -> SKAction
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SKAction").unwrap(), actionNamed : name)
    }
    unsafe fn actionNamed_duration_(name: NSString, duration: NSTimeInterval) -> SKAction
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SKAction").unwrap(), actionNamed : name, duration : duration)
    }
    unsafe fn actionNamed_fromURL_(name: NSString, url: NSURL) -> SKAction
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SKAction").unwrap(), actionNamed : name, fromURL : url)
    }
    unsafe fn actionNamed_fromURL_duration_(
        name: NSString,
        url: NSURL,
        duration: NSTimeInterval,
    ) -> SKAction
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SKAction").unwrap(), actionNamed : name, fromURL : url, duration : duration)
    }
}
impl SKAction_NodeWithPhysicsBody for SKAction {}
pub trait SKAction_NodeWithPhysicsBody: Sized + std::ops::Deref {
    unsafe fn changeChargeTo_duration_(v: f32, duration: NSTimeInterval) -> SKAction
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SKAction").unwrap(), changeChargeTo : v, duration : duration)
    }
    unsafe fn changeChargeBy_duration_(v: f32, duration: NSTimeInterval) -> SKAction
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SKAction").unwrap(), changeChargeBy : v, duration : duration)
    }
    unsafe fn changeMassTo_duration_(v: f32, duration: NSTimeInterval) -> SKAction
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SKAction").unwrap(), changeMassTo : v, duration : duration)
    }
    unsafe fn changeMassBy_duration_(v: f32, duration: NSTimeInterval) -> SKAction
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SKAction").unwrap(), changeMassBy : v, duration : duration)
    }
    unsafe fn applyForce_duration_(force: CGVector, duration: NSTimeInterval) -> SKAction
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SKAction").unwrap(), applyForce : force, duration : duration)
    }
    unsafe fn applyForce_atPoint_duration_(
        force: CGVector,
        point: CGPoint,
        duration: NSTimeInterval,
    ) -> SKAction
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SKAction").unwrap(), applyForce : force, atPoint : point, duration : duration)
    }
    unsafe fn applyTorque_duration_(torque: CGFloat, duration: NSTimeInterval) -> SKAction
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SKAction").unwrap(), applyTorque : torque, duration : duration)
    }
    unsafe fn applyImpulse_duration_(impulse: CGVector, duration: NSTimeInterval) -> SKAction
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SKAction").unwrap(), applyImpulse : impulse, duration : duration)
    }
    unsafe fn applyImpulse_atPoint_duration_(
        impulse: CGVector,
        point: CGPoint,
        duration: NSTimeInterval,
    ) -> SKAction
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SKAction").unwrap(), applyImpulse : impulse, atPoint : point, duration : duration)
    }
    unsafe fn applyAngularImpulse_duration_(impulse: CGFloat, duration: NSTimeInterval) -> SKAction
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SKAction").unwrap(), applyAngularImpulse : impulse, duration : duration)
    }
}
impl SKAction_PlaybackControl for SKAction {}
pub trait SKAction_PlaybackControl: Sized + std::ops::Deref {
    unsafe fn play() -> SKAction
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SKAction").unwrap(), play)
    }
    unsafe fn pause() -> SKAction
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SKAction").unwrap(), pause)
    }
    unsafe fn stop() -> SKAction
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SKAction").unwrap(), stop)
    }
    unsafe fn changePlaybackRateTo_duration_(v: f32, duration: NSTimeInterval) -> SKAction
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SKAction").unwrap(), changePlaybackRateTo : v, duration : duration)
    }
    unsafe fn changePlaybackRateBy_duration_(v: f32, duration: NSTimeInterval) -> SKAction
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SKAction").unwrap(), changePlaybackRateBy : v, duration : duration)
    }
}
impl SKAction_MixerControl for SKAction {}
pub trait SKAction_MixerControl: Sized + std::ops::Deref {
    unsafe fn changeVolumeTo_duration_(v: f32, duration: NSTimeInterval) -> SKAction
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SKAction").unwrap(), changeVolumeTo : v, duration : duration)
    }
    unsafe fn changeVolumeBy_duration_(v: f32, duration: NSTimeInterval) -> SKAction
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SKAction").unwrap(), changeVolumeBy : v, duration : duration)
    }
}
pub trait PSKWarpable: Sized + std::ops::Deref {
    unsafe fn warpGeometry(&self) -> SKWarpGeometry
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, warpGeometry)
    }
    unsafe fn setWarpGeometry_(&self, warpGeometry: SKWarpGeometry)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setWarpGeometry : warpGeometry)
    }
    unsafe fn subdivisionLevels(&self) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, subdivisionLevels)
    }
    unsafe fn setSubdivisionLevels_(&self, subdivisionLevels: NSInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSubdivisionLevels : subdivisionLevels)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct SKWarpGeometry(pub id);
impl std::ops::Deref for SKWarpGeometry {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for SKWarpGeometry {}
impl SKWarpGeometry {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"SKWarpGeometry").unwrap(), alloc) })
    }
}
impl PNSCopying for SKWarpGeometry {}
impl PNSSecureCoding for SKWarpGeometry {}
impl INSObject for SKWarpGeometry {}
impl PNSObject for SKWarpGeometry {}
impl std::convert::TryFrom<NSObject> for SKWarpGeometry {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<SKWarpGeometry, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"SKWarpGeometry").unwrap()) };
        if is_kind_of {
            Ok(SKWarpGeometry(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to SKWarpGeometry")
        }
    }
}
impl ISKWarpGeometry for SKWarpGeometry {}
pub trait ISKWarpGeometry: Sized + std::ops::Deref {}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct SKWarpGeometryGrid(pub id);
impl std::ops::Deref for SKWarpGeometryGrid {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for SKWarpGeometryGrid {}
impl SKWarpGeometryGrid {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"SKWarpGeometryGrid").unwrap(), alloc) })
    }
}
impl PNSSecureCoding for SKWarpGeometryGrid {}
impl ISKWarpGeometry for SKWarpGeometryGrid {}
impl PNSCopying for SKWarpGeometryGrid {}
impl From<SKWarpGeometryGrid> for SKWarpGeometry {
    fn from(child: SKWarpGeometryGrid) -> SKWarpGeometry {
        SKWarpGeometry(child.0)
    }
}
impl std::convert::TryFrom<SKWarpGeometry> for SKWarpGeometryGrid {
    type Error = &'static str;
    fn try_from(parent: SKWarpGeometry) -> Result<SKWarpGeometryGrid, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"SKWarpGeometryGrid").unwrap()) };
        if is_kind_of {
            Ok(SKWarpGeometryGrid(parent.0))
        } else {
            Err("This SKWarpGeometry cannot be downcasted to SKWarpGeometryGrid")
        }
    }
}
impl INSObject for SKWarpGeometryGrid {}
impl PNSObject for SKWarpGeometryGrid {}
impl ISKWarpGeometryGrid for SKWarpGeometryGrid {}
pub trait ISKWarpGeometryGrid: Sized + std::ops::Deref {
    unsafe fn initWithCoder_(&self, aDecoder: NSCoder) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithCoder : aDecoder)
    }
    unsafe fn initWithColumns_rows_sourcePositions_destPositions_(
        &self,
        cols: NSInteger,
        rows: NSInteger,
        sourcePositions: *const vector_float2,
        destPositions: *const vector_float2,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithColumns : cols, rows : rows, sourcePositions : sourcePositions, destPositions : destPositions)
    }
    unsafe fn sourcePositionAtIndex_(&self, index: NSInteger) -> vector_float2
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, sourcePositionAtIndex : index)
    }
    unsafe fn destPositionAtIndex_(&self, index: NSInteger) -> vector_float2
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, destPositionAtIndex : index)
    }
    unsafe fn gridByReplacingSourcePositions_(
        &self,
        sourcePositions: *const vector_float2,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, gridByReplacingSourcePositions : sourcePositions)
    }
    unsafe fn gridByReplacingDestPositions_(
        &self,
        destPositions: *const vector_float2,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, gridByReplacingDestPositions : destPositions)
    }
    unsafe fn numberOfColumns(&self) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, numberOfColumns)
    }
    unsafe fn numberOfRows(&self) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, numberOfRows)
    }
    unsafe fn vertexCount(&self) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, vertexCount)
    }
    unsafe fn grid() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SKWarpGeometryGrid").unwrap(), grid)
    }
    unsafe fn gridWithColumns_rows_(cols: NSInteger, rows: NSInteger) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SKWarpGeometryGrid").unwrap(), gridWithColumns : cols, rows : rows)
    }
    unsafe fn gridWithColumns_rows_sourcePositions_destPositions_(
        cols: NSInteger,
        rows: NSInteger,
        sourcePositions: *const vector_float2,
        destPositions: *const vector_float2,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SKWarpGeometryGrid").unwrap(), gridWithColumns : cols, rows : rows, sourcePositions : sourcePositions, destPositions : destPositions)
    }
}
impl SKAction_SKWarpable for SKAction {}
pub trait SKAction_SKWarpable: Sized + std::ops::Deref {
    unsafe fn warpTo_duration_(warp: SKWarpGeometry, duration: NSTimeInterval) -> SKAction
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SKAction").unwrap(), warpTo : warp, duration : duration)
    }
    unsafe fn animateWithWarps_times_(warps: NSArray, times: NSArray) -> SKAction
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SKAction").unwrap(), animateWithWarps : warps, times : times)
    }
    unsafe fn animateWithWarps_times_restore_(
        warps: NSArray,
        times: NSArray,
        restore: BOOL,
    ) -> SKAction
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SKAction").unwrap(), animateWithWarps : warps, times : times, restore : restore)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct SKEffectNode(pub id);
impl std::ops::Deref for SKEffectNode {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for SKEffectNode {}
impl SKEffectNode {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"SKEffectNode").unwrap(), alloc) })
    }
}
impl PSKWarpable for SKEffectNode {}
impl ISKNode for SKEffectNode {}
impl PNSCopying for SKEffectNode {}
impl PNSSecureCoding for SKEffectNode {}
impl std::convert::TryFrom<SKNode> for SKEffectNode {
    type Error = &'static str;
    fn try_from(parent: SKNode) -> Result<SKEffectNode, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"SKEffectNode").unwrap()) };
        if is_kind_of {
            Ok(SKEffectNode(parent.0))
        } else {
            Err("This SKNode cannot be downcasted to SKEffectNode")
        }
    }
}
impl INSResponder for SKEffectNode {}
impl PNSCoding for SKEffectNode {}
impl INSObject for SKEffectNode {}
impl PNSObject for SKEffectNode {}
impl ISKEffectNode for SKEffectNode {}
pub trait ISKEffectNode: Sized + std::ops::Deref {
    unsafe fn valueForAttributeNamed_(&self, key: NSString) -> SKAttributeValue
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, valueForAttributeNamed : key)
    }
    unsafe fn setValue_forAttributeNamed_(&self, value: SKAttributeValue, key: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setValue : value, forAttributeNamed : key)
    }
    unsafe fn filter(&self) -> CIFilter
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, filter)
    }
    unsafe fn setFilter_(&self, filter: CIFilter)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setFilter : filter)
    }
    unsafe fn shouldCenterFilter(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, shouldCenterFilter)
    }
    unsafe fn setShouldCenterFilter_(&self, shouldCenterFilter: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setShouldCenterFilter : shouldCenterFilter)
    }
    unsafe fn shouldEnableEffects(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, shouldEnableEffects)
    }
    unsafe fn setShouldEnableEffects_(&self, shouldEnableEffects: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setShouldEnableEffects : shouldEnableEffects)
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
    unsafe fn blendMode(&self) -> SKBlendMode
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, blendMode)
    }
    unsafe fn setBlendMode_(&self, blendMode: SKBlendMode)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setBlendMode : blendMode)
    }
    unsafe fn shader(&self) -> SKShader
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, shader)
    }
    unsafe fn setShader_(&self, shader: SKShader)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setShader : shader)
    }
    unsafe fn attributeValues(&self) -> NSDictionary
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, attributeValues)
    }
    unsafe fn setAttributeValues_(&self, attributeValues: NSDictionary)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAttributeValues : attributeValues)
    }
}
pub type SKSceneScaleMode = NSInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct SKScene(pub id);
impl std::ops::Deref for SKScene {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for SKScene {}
impl SKScene {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"SKScene").unwrap(), alloc) })
    }
}
impl ISKEffectNode for SKScene {}
impl PSKWarpable for SKScene {}
impl From<SKScene> for SKEffectNode {
    fn from(child: SKScene) -> SKEffectNode {
        SKEffectNode(child.0)
    }
}
impl std::convert::TryFrom<SKEffectNode> for SKScene {
    type Error = &'static str;
    fn try_from(parent: SKEffectNode) -> Result<SKScene, Self::Error> {
        let is_kind_of: bool = unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"SKScene").unwrap()) };
        if is_kind_of {
            Ok(SKScene(parent.0))
        } else {
            Err("This SKEffectNode cannot be downcasted to SKScene")
        }
    }
}
impl ISKNode for SKScene {}
impl PNSCopying for SKScene {}
impl PNSSecureCoding for SKScene {}
impl INSResponder for SKScene {}
impl PNSCoding for SKScene {}
impl INSObject for SKScene {}
impl PNSObject for SKScene {}
impl ISKScene for SKScene {}
pub trait ISKScene: Sized + std::ops::Deref {
    unsafe fn initWithSize_(&self, size: CGSize) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithSize : size)
    }
    unsafe fn sceneDidLoad(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, sceneDidLoad)
    }
    unsafe fn convertPointFromView_(&self, point: CGPoint) -> CGPoint
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, convertPointFromView : point)
    }
    unsafe fn convertPointToView_(&self, point: CGPoint) -> CGPoint
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, convertPointToView : point)
    }
    unsafe fn update_(&self, currentTime: NSTimeInterval)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, update : currentTime)
    }
    unsafe fn didEvaluateActions(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, didEvaluateActions)
    }
    unsafe fn didSimulatePhysics(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, didSimulatePhysics)
    }
    unsafe fn didApplyConstraints(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, didApplyConstraints)
    }
    unsafe fn didFinishUpdate(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, didFinishUpdate)
    }
    unsafe fn didMoveToView_(&self, view: SKView)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, didMoveToView : view)
    }
    unsafe fn willMoveFromView_(&self, view: SKView)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, willMoveFromView : view)
    }
    unsafe fn didChangeSize_(&self, oldSize: CGSize)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, didChangeSize : oldSize)
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
    unsafe fn scaleMode(&self) -> SKSceneScaleMode
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, scaleMode)
    }
    unsafe fn setScaleMode_(&self, scaleMode: SKSceneScaleMode)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setScaleMode : scaleMode)
    }
    unsafe fn camera(&self) -> SKCameraNode
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, camera)
    }
    unsafe fn setCamera_(&self, camera: SKCameraNode)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCamera : camera)
    }
    unsafe fn listener(&self) -> SKNode
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, listener)
    }
    unsafe fn setListener_(&self, listener: SKNode)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setListener : listener)
    }
    unsafe fn audioEngine(&self) -> AVAudioEngine
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, audioEngine)
    }
    unsafe fn backgroundColor(&self) -> NSColor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, backgroundColor)
    }
    unsafe fn setBackgroundColor_(&self, backgroundColor: NSColor)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setBackgroundColor : backgroundColor)
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
    unsafe fn physicsWorld(&self) -> SKPhysicsWorld
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, physicsWorld)
    }
    unsafe fn view(&self) -> SKView
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, view)
    }
    unsafe fn sceneWithSize_(size: CGSize) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SKScene").unwrap(), sceneWithSize : size)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct SKSpriteNode(pub id);
impl std::ops::Deref for SKSpriteNode {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for SKSpriteNode {}
impl SKSpriteNode {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"SKSpriteNode").unwrap(), alloc) })
    }
}
impl PSKWarpable for SKSpriteNode {}
impl ISKNode for SKSpriteNode {}
impl PNSCopying for SKSpriteNode {}
impl PNSSecureCoding for SKSpriteNode {}
impl std::convert::TryFrom<SKNode> for SKSpriteNode {
    type Error = &'static str;
    fn try_from(parent: SKNode) -> Result<SKSpriteNode, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"SKSpriteNode").unwrap()) };
        if is_kind_of {
            Ok(SKSpriteNode(parent.0))
        } else {
            Err("This SKNode cannot be downcasted to SKSpriteNode")
        }
    }
}
impl INSResponder for SKSpriteNode {}
impl PNSCoding for SKSpriteNode {}
impl INSObject for SKSpriteNode {}
impl PNSObject for SKSpriteNode {}
impl ISKSpriteNode for SKSpriteNode {}
pub trait ISKSpriteNode: Sized + std::ops::Deref {
    unsafe fn initWithTexture_color_size_(
        &self,
        texture: SKTexture,
        color: NSColor,
        size: CGSize,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithTexture : texture, color : color, size : size)
    }
    unsafe fn initWithTexture_(&self, texture: SKTexture) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithTexture : texture)
    }
    unsafe fn initWithImageNamed_(&self, name: NSString) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithImageNamed : name)
    }
    unsafe fn initWithColor_size_(&self, color: NSColor, size: CGSize) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithColor : color, size : size)
    }
    unsafe fn initWithCoder_(&self, aDecoder: NSCoder) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithCoder : aDecoder)
    }
    unsafe fn scaleToSize_(&self, size: CGSize)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, scaleToSize : size)
    }
    unsafe fn valueForAttributeNamed_(&self, key: NSString) -> SKAttributeValue
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, valueForAttributeNamed : key)
    }
    unsafe fn setValue_forAttributeNamed_(&self, value: SKAttributeValue, key: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setValue : value, forAttributeNamed : key)
    }
    unsafe fn texture(&self) -> SKTexture
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, texture)
    }
    unsafe fn setTexture_(&self, texture: SKTexture)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setTexture : texture)
    }
    unsafe fn normalTexture(&self) -> SKTexture
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, normalTexture)
    }
    unsafe fn setNormalTexture_(&self, normalTexture: SKTexture)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setNormalTexture : normalTexture)
    }
    unsafe fn lightingBitMask(&self) -> u32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, lightingBitMask)
    }
    unsafe fn setLightingBitMask_(&self, lightingBitMask: u32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setLightingBitMask : lightingBitMask)
    }
    unsafe fn shadowCastBitMask(&self) -> u32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, shadowCastBitMask)
    }
    unsafe fn setShadowCastBitMask_(&self, shadowCastBitMask: u32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setShadowCastBitMask : shadowCastBitMask)
    }
    unsafe fn shadowedBitMask(&self) -> u32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, shadowedBitMask)
    }
    unsafe fn setShadowedBitMask_(&self, shadowedBitMask: u32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setShadowedBitMask : shadowedBitMask)
    }
    unsafe fn centerRect(&self) -> CGRect
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, centerRect)
    }
    unsafe fn setCenterRect_(&self, centerRect: CGRect)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCenterRect : centerRect)
    }
    unsafe fn colorBlendFactor(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, colorBlendFactor)
    }
    unsafe fn setColorBlendFactor_(&self, colorBlendFactor: CGFloat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setColorBlendFactor : colorBlendFactor)
    }
    unsafe fn color(&self) -> NSColor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, color)
    }
    unsafe fn setColor_(&self, color: NSColor)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setColor : color)
    }
    unsafe fn blendMode(&self) -> SKBlendMode
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, blendMode)
    }
    unsafe fn setBlendMode_(&self, blendMode: SKBlendMode)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setBlendMode : blendMode)
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
    unsafe fn shader(&self) -> SKShader
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, shader)
    }
    unsafe fn setShader_(&self, shader: SKShader)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setShader : shader)
    }
    unsafe fn attributeValues(&self) -> NSDictionary
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, attributeValues)
    }
    unsafe fn setAttributeValues_(&self, attributeValues: NSDictionary)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAttributeValues : attributeValues)
    }
    unsafe fn spriteNodeWithTexture_size_(texture: SKTexture, size: CGSize) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SKSpriteNode").unwrap(), spriteNodeWithTexture : texture, size : size)
    }
    unsafe fn spriteNodeWithTexture_(texture: SKTexture) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SKSpriteNode").unwrap(), spriteNodeWithTexture : texture)
    }
    unsafe fn spriteNodeWithTexture_normalMap_(
        texture: SKTexture,
        normalMap: SKTexture,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SKSpriteNode").unwrap(), spriteNodeWithTexture : texture, normalMap : normalMap)
    }
    unsafe fn spriteNodeWithImageNamed_(name: NSString) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SKSpriteNode").unwrap(), spriteNodeWithImageNamed : name)
    }
    unsafe fn spriteNodeWithImageNamed_normalMapped_(
        name: NSString,
        generateNormalMap: BOOL,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SKSpriteNode").unwrap(), spriteNodeWithImageNamed : name, normalMapped : generateNormalMap)
    }
    unsafe fn spriteNodeWithColor_size_(color: NSColor, size: CGSize) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SKSpriteNode").unwrap(), spriteNodeWithColor : color, size : size)
    }
}
pub type SKInterpolationMode = NSInteger;
pub type SKRepeatMode = NSInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct SKKeyframeSequence(pub id);
impl std::ops::Deref for SKKeyframeSequence {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for SKKeyframeSequence {}
impl SKKeyframeSequence {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"SKKeyframeSequence").unwrap(), alloc) })
    }
}
impl PNSSecureCoding for SKKeyframeSequence {}
impl PNSCopying for SKKeyframeSequence {}
impl INSObject for SKKeyframeSequence {}
impl PNSObject for SKKeyframeSequence {}
impl std::convert::TryFrom<NSObject> for SKKeyframeSequence {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<SKKeyframeSequence, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"SKKeyframeSequence").unwrap()) };
        if is_kind_of {
            Ok(SKKeyframeSequence(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to SKKeyframeSequence")
        }
    }
}
impl ISKKeyframeSequence for SKKeyframeSequence {}
pub trait ISKKeyframeSequence: Sized + std::ops::Deref {
    unsafe fn initWithKeyframeValues_times_(&self, values: NSArray, times: NSArray) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithKeyframeValues : values, times : times)
    }
    unsafe fn initWithCapacity_(&self, numItems: NSUInteger) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithCapacity : numItems)
    }
    unsafe fn initWithCoder_(&self, aDecoder: NSCoder) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithCoder : aDecoder)
    }
    unsafe fn count(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, count)
    }
    unsafe fn addKeyframeValue_time_(&self, value: id, time: CGFloat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, addKeyframeValue : value, time : time)
    }
    unsafe fn removeLastKeyframe(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, removeLastKeyframe)
    }
    unsafe fn removeKeyframeAtIndex_(&self, index: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, removeKeyframeAtIndex : index)
    }
    unsafe fn setKeyframeValue_forIndex_(&self, value: id, index: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setKeyframeValue : value, forIndex : index)
    }
    unsafe fn setKeyframeTime_forIndex_(&self, time: CGFloat, index: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setKeyframeTime : time, forIndex : index)
    }
    unsafe fn setKeyframeValue_time_forIndex_(&self, value: id, time: CGFloat, index: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setKeyframeValue : value, time : time, forIndex : index)
    }
    unsafe fn getKeyframeValueForIndex_(&self, index: NSUInteger) -> id
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, getKeyframeValueForIndex : index)
    }
    unsafe fn getKeyframeTimeForIndex_(&self, index: NSUInteger) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, getKeyframeTimeForIndex : index)
    }
    unsafe fn sampleAtTime_(&self, time: CGFloat) -> id
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, sampleAtTime : time)
    }
    unsafe fn interpolationMode(&self) -> SKInterpolationMode
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, interpolationMode)
    }
    unsafe fn setInterpolationMode_(&self, interpolationMode: SKInterpolationMode)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInterpolationMode : interpolationMode)
    }
    unsafe fn repeatMode(&self) -> SKRepeatMode
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, repeatMode)
    }
    unsafe fn setRepeatMode_(&self, repeatMode: SKRepeatMode)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRepeatMode : repeatMode)
    }
}
pub type SKParticleRenderOrder = NSUInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct SKEmitterNode(pub id);
impl std::ops::Deref for SKEmitterNode {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for SKEmitterNode {}
impl SKEmitterNode {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"SKEmitterNode").unwrap(), alloc) })
    }
}
impl ISKNode for SKEmitterNode {}
impl PNSCopying for SKEmitterNode {}
impl PNSSecureCoding for SKEmitterNode {}
impl std::convert::TryFrom<SKNode> for SKEmitterNode {
    type Error = &'static str;
    fn try_from(parent: SKNode) -> Result<SKEmitterNode, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"SKEmitterNode").unwrap()) };
        if is_kind_of {
            Ok(SKEmitterNode(parent.0))
        } else {
            Err("This SKNode cannot be downcasted to SKEmitterNode")
        }
    }
}
impl INSResponder for SKEmitterNode {}
impl PNSCoding for SKEmitterNode {}
impl INSObject for SKEmitterNode {}
impl PNSObject for SKEmitterNode {}
impl ISKEmitterNode for SKEmitterNode {}
pub trait ISKEmitterNode: Sized + std::ops::Deref {
    unsafe fn advanceSimulationTime_(&self, sec: NSTimeInterval)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, advanceSimulationTime : sec)
    }
    unsafe fn resetSimulation(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, resetSimulation)
    }
    unsafe fn valueForAttributeNamed_(&self, key: NSString) -> SKAttributeValue
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, valueForAttributeNamed : key)
    }
    unsafe fn setValue_forAttributeNamed_(&self, value: SKAttributeValue, key: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setValue : value, forAttributeNamed : key)
    }
    unsafe fn particleTexture(&self) -> SKTexture
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, particleTexture)
    }
    unsafe fn setParticleTexture_(&self, particleTexture: SKTexture)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setParticleTexture : particleTexture)
    }
    unsafe fn particleBlendMode(&self) -> SKBlendMode
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, particleBlendMode)
    }
    unsafe fn setParticleBlendMode_(&self, particleBlendMode: SKBlendMode)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setParticleBlendMode : particleBlendMode)
    }
    unsafe fn particleColor(&self) -> NSColor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, particleColor)
    }
    unsafe fn setParticleColor_(&self, particleColor: NSColor)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setParticleColor : particleColor)
    }
    unsafe fn particleColorRedRange(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, particleColorRedRange)
    }
    unsafe fn setParticleColorRedRange_(&self, particleColorRedRange: CGFloat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setParticleColorRedRange : particleColorRedRange)
    }
    unsafe fn particleColorGreenRange(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, particleColorGreenRange)
    }
    unsafe fn setParticleColorGreenRange_(&self, particleColorGreenRange: CGFloat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setParticleColorGreenRange : particleColorGreenRange)
    }
    unsafe fn particleColorBlueRange(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, particleColorBlueRange)
    }
    unsafe fn setParticleColorBlueRange_(&self, particleColorBlueRange: CGFloat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setParticleColorBlueRange : particleColorBlueRange)
    }
    unsafe fn particleColorAlphaRange(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, particleColorAlphaRange)
    }
    unsafe fn setParticleColorAlphaRange_(&self, particleColorAlphaRange: CGFloat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setParticleColorAlphaRange : particleColorAlphaRange)
    }
    unsafe fn particleColorRedSpeed(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, particleColorRedSpeed)
    }
    unsafe fn setParticleColorRedSpeed_(&self, particleColorRedSpeed: CGFloat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setParticleColorRedSpeed : particleColorRedSpeed)
    }
    unsafe fn particleColorGreenSpeed(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, particleColorGreenSpeed)
    }
    unsafe fn setParticleColorGreenSpeed_(&self, particleColorGreenSpeed: CGFloat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setParticleColorGreenSpeed : particleColorGreenSpeed)
    }
    unsafe fn particleColorBlueSpeed(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, particleColorBlueSpeed)
    }
    unsafe fn setParticleColorBlueSpeed_(&self, particleColorBlueSpeed: CGFloat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setParticleColorBlueSpeed : particleColorBlueSpeed)
    }
    unsafe fn particleColorAlphaSpeed(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, particleColorAlphaSpeed)
    }
    unsafe fn setParticleColorAlphaSpeed_(&self, particleColorAlphaSpeed: CGFloat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setParticleColorAlphaSpeed : particleColorAlphaSpeed)
    }
    unsafe fn particleColorSequence(&self) -> SKKeyframeSequence
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, particleColorSequence)
    }
    unsafe fn setParticleColorSequence_(&self, particleColorSequence: SKKeyframeSequence)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setParticleColorSequence : particleColorSequence)
    }
    unsafe fn particleColorBlendFactor(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, particleColorBlendFactor)
    }
    unsafe fn setParticleColorBlendFactor_(&self, particleColorBlendFactor: CGFloat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setParticleColorBlendFactor : particleColorBlendFactor)
    }
    unsafe fn particleColorBlendFactorRange(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, particleColorBlendFactorRange)
    }
    unsafe fn setParticleColorBlendFactorRange_(&self, particleColorBlendFactorRange: CGFloat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setParticleColorBlendFactorRange : particleColorBlendFactorRange)
    }
    unsafe fn particleColorBlendFactorSpeed(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, particleColorBlendFactorSpeed)
    }
    unsafe fn setParticleColorBlendFactorSpeed_(&self, particleColorBlendFactorSpeed: CGFloat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setParticleColorBlendFactorSpeed : particleColorBlendFactorSpeed)
    }
    unsafe fn particleColorBlendFactorSequence(&self) -> SKKeyframeSequence
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, particleColorBlendFactorSequence)
    }
    unsafe fn setParticleColorBlendFactorSequence_(
        &self,
        particleColorBlendFactorSequence: SKKeyframeSequence,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setParticleColorBlendFactorSequence : particleColorBlendFactorSequence)
    }
    unsafe fn particlePosition(&self) -> CGPoint
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, particlePosition)
    }
    unsafe fn setParticlePosition_(&self, particlePosition: CGPoint)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setParticlePosition : particlePosition)
    }
    unsafe fn particlePositionRange(&self) -> CGVector
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, particlePositionRange)
    }
    unsafe fn setParticlePositionRange_(&self, particlePositionRange: CGVector)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setParticlePositionRange : particlePositionRange)
    }
    unsafe fn particleSpeed(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, particleSpeed)
    }
    unsafe fn setParticleSpeed_(&self, particleSpeed: CGFloat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setParticleSpeed : particleSpeed)
    }
    unsafe fn particleSpeedRange(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, particleSpeedRange)
    }
    unsafe fn setParticleSpeedRange_(&self, particleSpeedRange: CGFloat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setParticleSpeedRange : particleSpeedRange)
    }
    unsafe fn emissionAngle(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, emissionAngle)
    }
    unsafe fn setEmissionAngle_(&self, emissionAngle: CGFloat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setEmissionAngle : emissionAngle)
    }
    unsafe fn emissionAngleRange(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, emissionAngleRange)
    }
    unsafe fn setEmissionAngleRange_(&self, emissionAngleRange: CGFloat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setEmissionAngleRange : emissionAngleRange)
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
    unsafe fn particleBirthRate(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, particleBirthRate)
    }
    unsafe fn setParticleBirthRate_(&self, particleBirthRate: CGFloat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setParticleBirthRate : particleBirthRate)
    }
    unsafe fn numParticlesToEmit(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, numParticlesToEmit)
    }
    unsafe fn setNumParticlesToEmit_(&self, numParticlesToEmit: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setNumParticlesToEmit : numParticlesToEmit)
    }
    unsafe fn particleLifetime(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, particleLifetime)
    }
    unsafe fn setParticleLifetime_(&self, particleLifetime: CGFloat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setParticleLifetime : particleLifetime)
    }
    unsafe fn particleLifetimeRange(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, particleLifetimeRange)
    }
    unsafe fn setParticleLifetimeRange_(&self, particleLifetimeRange: CGFloat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setParticleLifetimeRange : particleLifetimeRange)
    }
    unsafe fn particleRotation(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, particleRotation)
    }
    unsafe fn setParticleRotation_(&self, particleRotation: CGFloat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setParticleRotation : particleRotation)
    }
    unsafe fn particleRotationRange(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, particleRotationRange)
    }
    unsafe fn setParticleRotationRange_(&self, particleRotationRange: CGFloat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setParticleRotationRange : particleRotationRange)
    }
    unsafe fn particleRotationSpeed(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, particleRotationSpeed)
    }
    unsafe fn setParticleRotationSpeed_(&self, particleRotationSpeed: CGFloat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setParticleRotationSpeed : particleRotationSpeed)
    }
    unsafe fn particleSize(&self) -> CGSize
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, particleSize)
    }
    unsafe fn setParticleSize_(&self, particleSize: CGSize)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setParticleSize : particleSize)
    }
    unsafe fn particleScale(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, particleScale)
    }
    unsafe fn setParticleScale_(&self, particleScale: CGFloat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setParticleScale : particleScale)
    }
    unsafe fn particleScaleRange(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, particleScaleRange)
    }
    unsafe fn setParticleScaleRange_(&self, particleScaleRange: CGFloat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setParticleScaleRange : particleScaleRange)
    }
    unsafe fn particleScaleSpeed(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, particleScaleSpeed)
    }
    unsafe fn setParticleScaleSpeed_(&self, particleScaleSpeed: CGFloat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setParticleScaleSpeed : particleScaleSpeed)
    }
    unsafe fn particleScaleSequence(&self) -> SKKeyframeSequence
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, particleScaleSequence)
    }
    unsafe fn setParticleScaleSequence_(&self, particleScaleSequence: SKKeyframeSequence)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setParticleScaleSequence : particleScaleSequence)
    }
    unsafe fn particleAlpha(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, particleAlpha)
    }
    unsafe fn setParticleAlpha_(&self, particleAlpha: CGFloat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setParticleAlpha : particleAlpha)
    }
    unsafe fn particleAlphaRange(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, particleAlphaRange)
    }
    unsafe fn setParticleAlphaRange_(&self, particleAlphaRange: CGFloat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setParticleAlphaRange : particleAlphaRange)
    }
    unsafe fn particleAlphaSpeed(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, particleAlphaSpeed)
    }
    unsafe fn setParticleAlphaSpeed_(&self, particleAlphaSpeed: CGFloat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setParticleAlphaSpeed : particleAlphaSpeed)
    }
    unsafe fn particleAlphaSequence(&self) -> SKKeyframeSequence
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, particleAlphaSequence)
    }
    unsafe fn setParticleAlphaSequence_(&self, particleAlphaSequence: SKKeyframeSequence)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setParticleAlphaSequence : particleAlphaSequence)
    }
    unsafe fn particleAction(&self) -> SKAction
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, particleAction)
    }
    unsafe fn setParticleAction_(&self, particleAction: SKAction)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setParticleAction : particleAction)
    }
    unsafe fn fieldBitMask(&self) -> u32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, fieldBitMask)
    }
    unsafe fn setFieldBitMask_(&self, fieldBitMask: u32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setFieldBitMask : fieldBitMask)
    }
    unsafe fn targetNode(&self) -> SKNode
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, targetNode)
    }
    unsafe fn setTargetNode_(&self, targetNode: SKNode)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setTargetNode : targetNode)
    }
    unsafe fn shader(&self) -> SKShader
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, shader)
    }
    unsafe fn setShader_(&self, shader: SKShader)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setShader : shader)
    }
    unsafe fn attributeValues(&self) -> NSDictionary
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, attributeValues)
    }
    unsafe fn setAttributeValues_(&self, attributeValues: NSDictionary)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAttributeValues : attributeValues)
    }
    unsafe fn particleZPosition(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, particleZPosition)
    }
    unsafe fn setParticleZPosition_(&self, particleZPosition: CGFloat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setParticleZPosition : particleZPosition)
    }
    unsafe fn particleRenderOrder(&self) -> SKParticleRenderOrder
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, particleRenderOrder)
    }
    unsafe fn setParticleRenderOrder_(&self, particleRenderOrder: SKParticleRenderOrder)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setParticleRenderOrder : particleRenderOrder)
    }
    unsafe fn particleZPositionRange(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, particleZPositionRange)
    }
    unsafe fn setParticleZPositionRange_(&self, particleZPositionRange: CGFloat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setParticleZPositionRange : particleZPositionRange)
    }
    unsafe fn particleZPositionSpeed(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, particleZPositionSpeed)
    }
    unsafe fn setParticleZPositionSpeed_(&self, particleZPositionSpeed: CGFloat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setParticleZPositionSpeed : particleZPositionSpeed)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct SKShapeNode(pub id);
impl std::ops::Deref for SKShapeNode {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for SKShapeNode {}
impl SKShapeNode {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"SKShapeNode").unwrap(), alloc) })
    }
}
impl ISKNode for SKShapeNode {}
impl PNSCopying for SKShapeNode {}
impl PNSSecureCoding for SKShapeNode {}
impl std::convert::TryFrom<SKNode> for SKShapeNode {
    type Error = &'static str;
    fn try_from(parent: SKNode) -> Result<SKShapeNode, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"SKShapeNode").unwrap()) };
        if is_kind_of {
            Ok(SKShapeNode(parent.0))
        } else {
            Err("This SKNode cannot be downcasted to SKShapeNode")
        }
    }
}
impl INSResponder for SKShapeNode {}
impl PNSCoding for SKShapeNode {}
impl INSObject for SKShapeNode {}
impl PNSObject for SKShapeNode {}
impl ISKShapeNode for SKShapeNode {}
pub trait ISKShapeNode: Sized + std::ops::Deref {
    unsafe fn valueForAttributeNamed_(&self, key: NSString) -> SKAttributeValue
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, valueForAttributeNamed : key)
    }
    unsafe fn setValue_forAttributeNamed_(&self, value: SKAttributeValue, key: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setValue : value, forAttributeNamed : key)
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
    unsafe fn blendMode(&self) -> SKBlendMode
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, blendMode)
    }
    unsafe fn setBlendMode_(&self, blendMode: SKBlendMode)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setBlendMode : blendMode)
    }
    unsafe fn isAntialiased(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isAntialiased)
    }
    unsafe fn setAntialiased_(&self, antialiased: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAntialiased : antialiased)
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
    unsafe fn glowWidth(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, glowWidth)
    }
    unsafe fn setGlowWidth_(&self, glowWidth: CGFloat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setGlowWidth : glowWidth)
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
    unsafe fn lineLength(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, lineLength)
    }
    unsafe fn fillTexture(&self) -> SKTexture
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, fillTexture)
    }
    unsafe fn setFillTexture_(&self, fillTexture: SKTexture)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setFillTexture : fillTexture)
    }
    unsafe fn fillShader(&self) -> SKShader
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, fillShader)
    }
    unsafe fn setFillShader_(&self, fillShader: SKShader)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setFillShader : fillShader)
    }
    unsafe fn strokeTexture(&self) -> SKTexture
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, strokeTexture)
    }
    unsafe fn setStrokeTexture_(&self, strokeTexture: SKTexture)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setStrokeTexture : strokeTexture)
    }
    unsafe fn strokeShader(&self) -> SKShader
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, strokeShader)
    }
    unsafe fn setStrokeShader_(&self, strokeShader: SKShader)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setStrokeShader : strokeShader)
    }
    unsafe fn attributeValues(&self) -> NSDictionary
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, attributeValues)
    }
    unsafe fn setAttributeValues_(&self, attributeValues: NSDictionary)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAttributeValues : attributeValues)
    }
    unsafe fn shapeNodeWithPath_(path: CGPathRef) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SKShapeNode").unwrap(), shapeNodeWithPath : path)
    }
    unsafe fn shapeNodeWithPath_centered_(path: CGPathRef, centered: BOOL) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SKShapeNode").unwrap(), shapeNodeWithPath : path, centered : centered)
    }
    unsafe fn shapeNodeWithRect_(rect: CGRect) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SKShapeNode").unwrap(), shapeNodeWithRect : rect)
    }
    unsafe fn shapeNodeWithRectOfSize_(size: CGSize) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SKShapeNode").unwrap(), shapeNodeWithRectOfSize : size)
    }
    unsafe fn shapeNodeWithRect_cornerRadius_(rect: CGRect, cornerRadius: CGFloat) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SKShapeNode").unwrap(), shapeNodeWithRect : rect, cornerRadius : cornerRadius)
    }
    unsafe fn shapeNodeWithRectOfSize_cornerRadius_(
        size: CGSize,
        cornerRadius: CGFloat,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SKShapeNode").unwrap(), shapeNodeWithRectOfSize : size, cornerRadius : cornerRadius)
    }
    unsafe fn shapeNodeWithCircleOfRadius_(radius: CGFloat) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SKShapeNode").unwrap(), shapeNodeWithCircleOfRadius : radius)
    }
    unsafe fn shapeNodeWithEllipseInRect_(rect: CGRect) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SKShapeNode").unwrap(), shapeNodeWithEllipseInRect : rect)
    }
    unsafe fn shapeNodeWithEllipseOfSize_(size: CGSize) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SKShapeNode").unwrap(), shapeNodeWithEllipseOfSize : size)
    }
    unsafe fn shapeNodeWithPoints_count_(points: *mut CGPoint, numPoints: usize) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SKShapeNode").unwrap(), shapeNodeWithPoints : points, count : numPoints)
    }
    unsafe fn shapeNodeWithSplinePoints_count_(
        points: *mut CGPoint,
        numPoints: usize,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SKShapeNode").unwrap(), shapeNodeWithSplinePoints : points, count : numPoints)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct SKFieldNode(pub id);
impl std::ops::Deref for SKFieldNode {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for SKFieldNode {}
impl SKFieldNode {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"SKFieldNode").unwrap(), alloc) })
    }
}
impl ISKNode for SKFieldNode {}
impl PNSCopying for SKFieldNode {}
impl PNSSecureCoding for SKFieldNode {}
impl std::convert::TryFrom<SKNode> for SKFieldNode {
    type Error = &'static str;
    fn try_from(parent: SKNode) -> Result<SKFieldNode, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"SKFieldNode").unwrap()) };
        if is_kind_of {
            Ok(SKFieldNode(parent.0))
        } else {
            Err("This SKNode cannot be downcasted to SKFieldNode")
        }
    }
}
impl INSResponder for SKFieldNode {}
impl PNSCoding for SKFieldNode {}
impl INSObject for SKFieldNode {}
impl PNSObject for SKFieldNode {}
impl ISKFieldNode for SKFieldNode {}
pub trait ISKFieldNode: Sized + std::ops::Deref {
    unsafe fn region(&self) -> SKRegion
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, region)
    }
    unsafe fn setRegion_(&self, region: SKRegion)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRegion : region)
    }
    unsafe fn strength(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, strength)
    }
    unsafe fn setStrength_(&self, strength: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setStrength : strength)
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
    unsafe fn minimumRadius(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, minimumRadius)
    }
    unsafe fn setMinimumRadius_(&self, minimumRadius: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMinimumRadius : minimumRadius)
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
    unsafe fn isExclusive(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isExclusive)
    }
    unsafe fn setExclusive_(&self, exclusive: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setExclusive : exclusive)
    }
    unsafe fn categoryBitMask(&self) -> u32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, categoryBitMask)
    }
    unsafe fn setCategoryBitMask_(&self, categoryBitMask: u32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCategoryBitMask : categoryBitMask)
    }
    unsafe fn direction(&self) -> vector_float3
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, direction)
    }
    unsafe fn setDirection_(&self, direction: vector_float3)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDirection : direction)
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
    unsafe fn animationSpeed(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, animationSpeed)
    }
    unsafe fn setAnimationSpeed_(&self, animationSpeed: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAnimationSpeed : animationSpeed)
    }
    unsafe fn texture(&self) -> SKTexture
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, texture)
    }
    unsafe fn setTexture_(&self, texture: SKTexture)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setTexture : texture)
    }
    unsafe fn dragField() -> SKFieldNode
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SKFieldNode").unwrap(), dragField)
    }
    unsafe fn vortexField() -> SKFieldNode
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SKFieldNode").unwrap(), vortexField)
    }
    unsafe fn radialGravityField() -> SKFieldNode
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SKFieldNode").unwrap(), radialGravityField)
    }
    unsafe fn linearGravityFieldWithVector_(direction: vector_float3) -> SKFieldNode
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SKFieldNode").unwrap(), linearGravityFieldWithVector : direction)
    }
    unsafe fn velocityFieldWithVector_(direction: vector_float3) -> SKFieldNode
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SKFieldNode").unwrap(), velocityFieldWithVector : direction)
    }
    unsafe fn velocityFieldWithTexture_(velocityTexture: SKTexture) -> SKFieldNode
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SKFieldNode").unwrap(), velocityFieldWithTexture : velocityTexture)
    }
    unsafe fn noiseFieldWithSmoothness_animationSpeed_(
        smoothness: CGFloat,
        speed: CGFloat,
    ) -> SKFieldNode
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SKFieldNode").unwrap(), noiseFieldWithSmoothness : smoothness, animationSpeed : speed)
    }
    unsafe fn turbulenceFieldWithSmoothness_animationSpeed_(
        smoothness: CGFloat,
        speed: CGFloat,
    ) -> SKFieldNode
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SKFieldNode").unwrap(), turbulenceFieldWithSmoothness : smoothness, animationSpeed : speed)
    }
    unsafe fn springField() -> SKFieldNode
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SKFieldNode").unwrap(), springField)
    }
    unsafe fn electricField() -> SKFieldNode
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SKFieldNode").unwrap(), electricField)
    }
    unsafe fn magneticField() -> SKFieldNode
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SKFieldNode").unwrap(), magneticField)
    }
    unsafe fn customFieldWithEvaluationBlock_(block: SKFieldForceEvaluator) -> SKFieldNode
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SKFieldNode").unwrap(), customFieldWithEvaluationBlock : block)
    }
}
pub type SKFieldForceEvaluator = *mut ::std::os::raw::c_void;
pub type SKLabelVerticalAlignmentMode = NSInteger;
pub type SKLabelHorizontalAlignmentMode = NSInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct SKLabelNode(pub id);
impl std::ops::Deref for SKLabelNode {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for SKLabelNode {}
impl SKLabelNode {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"SKLabelNode").unwrap(), alloc) })
    }
}
impl ISKNode for SKLabelNode {}
impl PNSCopying for SKLabelNode {}
impl PNSSecureCoding for SKLabelNode {}
impl std::convert::TryFrom<SKNode> for SKLabelNode {
    type Error = &'static str;
    fn try_from(parent: SKNode) -> Result<SKLabelNode, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"SKLabelNode").unwrap()) };
        if is_kind_of {
            Ok(SKLabelNode(parent.0))
        } else {
            Err("This SKNode cannot be downcasted to SKLabelNode")
        }
    }
}
impl INSResponder for SKLabelNode {}
impl PNSCoding for SKLabelNode {}
impl INSObject for SKLabelNode {}
impl PNSObject for SKLabelNode {}
impl ISKLabelNode for SKLabelNode {}
pub trait ISKLabelNode: Sized + std::ops::Deref {
    unsafe fn initWithFontNamed_(&self, fontName: NSString) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithFontNamed : fontName)
    }
    unsafe fn verticalAlignmentMode(&self) -> SKLabelVerticalAlignmentMode
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, verticalAlignmentMode)
    }
    unsafe fn setVerticalAlignmentMode_(&self, verticalAlignmentMode: SKLabelVerticalAlignmentMode)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setVerticalAlignmentMode : verticalAlignmentMode)
    }
    unsafe fn horizontalAlignmentMode(&self) -> SKLabelHorizontalAlignmentMode
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, horizontalAlignmentMode)
    }
    unsafe fn setHorizontalAlignmentMode_(
        &self,
        horizontalAlignmentMode: SKLabelHorizontalAlignmentMode,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setHorizontalAlignmentMode : horizontalAlignmentMode)
    }
    unsafe fn numberOfLines(&self) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, numberOfLines)
    }
    unsafe fn setNumberOfLines_(&self, numberOfLines: NSInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setNumberOfLines : numberOfLines)
    }
    unsafe fn lineBreakMode(&self) -> NSLineBreakMode
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, lineBreakMode)
    }
    unsafe fn setLineBreakMode_(&self, lineBreakMode: NSLineBreakMode)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setLineBreakMode : lineBreakMode)
    }
    unsafe fn preferredMaxLayoutWidth(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, preferredMaxLayoutWidth)
    }
    unsafe fn setPreferredMaxLayoutWidth_(&self, preferredMaxLayoutWidth: CGFloat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPreferredMaxLayoutWidth : preferredMaxLayoutWidth)
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
    unsafe fn attributedText(&self) -> NSAttributedString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, attributedText)
    }
    unsafe fn setAttributedText_(&self, attributedText: NSAttributedString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAttributedText : attributedText)
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
    unsafe fn fontColor(&self) -> NSColor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, fontColor)
    }
    unsafe fn setFontColor_(&self, fontColor: NSColor)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setFontColor : fontColor)
    }
    unsafe fn colorBlendFactor(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, colorBlendFactor)
    }
    unsafe fn setColorBlendFactor_(&self, colorBlendFactor: CGFloat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setColorBlendFactor : colorBlendFactor)
    }
    unsafe fn color(&self) -> NSColor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, color)
    }
    unsafe fn setColor_(&self, color: NSColor)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setColor : color)
    }
    unsafe fn blendMode(&self) -> SKBlendMode
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, blendMode)
    }
    unsafe fn setBlendMode_(&self, blendMode: SKBlendMode)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setBlendMode : blendMode)
    }
    unsafe fn labelNodeWithText_(text: NSString) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SKLabelNode").unwrap(), labelNodeWithText : text)
    }
    unsafe fn labelNodeWithAttributedText_(attributedText: NSAttributedString) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SKLabelNode").unwrap(), labelNodeWithAttributedText : attributedText)
    }
    unsafe fn labelNodeWithFontNamed_(fontName: NSString) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SKLabelNode").unwrap(), labelNodeWithFontNamed : fontName)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct SKVideoNode(pub id);
impl std::ops::Deref for SKVideoNode {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for SKVideoNode {}
impl SKVideoNode {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"SKVideoNode").unwrap(), alloc) })
    }
}
impl ISKNode for SKVideoNode {}
impl PNSCopying for SKVideoNode {}
impl PNSSecureCoding for SKVideoNode {}
impl std::convert::TryFrom<SKNode> for SKVideoNode {
    type Error = &'static str;
    fn try_from(parent: SKNode) -> Result<SKVideoNode, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"SKVideoNode").unwrap()) };
        if is_kind_of {
            Ok(SKVideoNode(parent.0))
        } else {
            Err("This SKNode cannot be downcasted to SKVideoNode")
        }
    }
}
impl INSResponder for SKVideoNode {}
impl PNSCoding for SKVideoNode {}
impl INSObject for SKVideoNode {}
impl PNSObject for SKVideoNode {}
impl ISKVideoNode for SKVideoNode {}
pub trait ISKVideoNode: Sized + std::ops::Deref {
    unsafe fn initWithAVPlayer_(&self, player: AVPlayer) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithAVPlayer : player)
    }
    unsafe fn initWithVideoFileNamed_(&self, videoFile: NSString) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithVideoFileNamed : videoFile)
    }
    unsafe fn initWithFileNamed_(&self, videoFile: NSString) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithFileNamed : videoFile)
    }
    unsafe fn initWithVideoURL_(&self, url: NSURL) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithVideoURL : url)
    }
    unsafe fn initWithURL_(&self, url: NSURL) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithURL : url)
    }
    unsafe fn initWithCoder_(&self, aDecoder: NSCoder) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithCoder : aDecoder)
    }
    unsafe fn play(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, play)
    }
    unsafe fn pause(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, pause)
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
    unsafe fn videoNodeWithAVPlayer_(player: AVPlayer) -> SKVideoNode
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SKVideoNode").unwrap(), videoNodeWithAVPlayer : player)
    }
    unsafe fn videoNodeWithVideoFileNamed_(videoFile: NSString) -> SKVideoNode
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SKVideoNode").unwrap(), videoNodeWithVideoFileNamed : videoFile)
    }
    unsafe fn videoNodeWithFileNamed_(videoFile: NSString) -> SKVideoNode
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SKVideoNode").unwrap(), videoNodeWithFileNamed : videoFile)
    }
    unsafe fn videoNodeWithVideoURL_(videoURL: NSURL) -> SKVideoNode
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SKVideoNode").unwrap(), videoNodeWithVideoURL : videoURL)
    }
    unsafe fn videoNodeWithURL_(videoURL: NSURL) -> SKVideoNode
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SKVideoNode").unwrap(), videoNodeWithURL : videoURL)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct SKAudioNode(pub id);
impl std::ops::Deref for SKAudioNode {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for SKAudioNode {}
impl SKAudioNode {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"SKAudioNode").unwrap(), alloc) })
    }
}
impl PNSSecureCoding for SKAudioNode {}
impl ISKNode for SKAudioNode {}
impl PNSCopying for SKAudioNode {}
impl std::convert::TryFrom<SKNode> for SKAudioNode {
    type Error = &'static str;
    fn try_from(parent: SKNode) -> Result<SKAudioNode, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"SKAudioNode").unwrap()) };
        if is_kind_of {
            Ok(SKAudioNode(parent.0))
        } else {
            Err("This SKNode cannot be downcasted to SKAudioNode")
        }
    }
}
impl INSResponder for SKAudioNode {}
impl PNSCoding for SKAudioNode {}
impl INSObject for SKAudioNode {}
impl PNSObject for SKAudioNode {}
impl ISKAudioNode for SKAudioNode {}
pub trait ISKAudioNode: Sized + std::ops::Deref {
    unsafe fn initWithAVAudioNode_(&self, node: AVAudioNode) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithAVAudioNode : node)
    }
    unsafe fn initWithCoder_(&self, aDecoder: NSCoder) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithCoder : aDecoder)
    }
    unsafe fn initWithFileNamed_(&self, name: NSString) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithFileNamed : name)
    }
    unsafe fn initWithURL_(&self, url: NSURL) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithURL : url)
    }
    unsafe fn avAudioNode(&self) -> AVAudioNode
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, avAudioNode)
    }
    unsafe fn setAvAudioNode_(&self, avAudioNode: AVAudioNode)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAvAudioNode : avAudioNode)
    }
    unsafe fn autoplayLooped(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, autoplayLooped)
    }
    unsafe fn setAutoplayLooped_(&self, autoplayLooped: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAutoplayLooped : autoplayLooped)
    }
    unsafe fn isPositional(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isPositional)
    }
    unsafe fn setPositional_(&self, positional: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPositional : positional)
    }
}
impl SKAction_SKAudioNode for SKAction {}
pub trait SKAction_SKAudioNode: Sized + std::ops::Deref {
    unsafe fn stereoPanTo_duration_(v: f32, duration: NSTimeInterval) -> SKAction
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SKAction").unwrap(), stereoPanTo : v, duration : duration)
    }
    unsafe fn stereoPanBy_duration_(v: f32, duration: NSTimeInterval) -> SKAction
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SKAction").unwrap(), stereoPanBy : v, duration : duration)
    }
    unsafe fn changeReverbTo_duration_(v: f32, duration: NSTimeInterval) -> SKAction
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SKAction").unwrap(), changeReverbTo : v, duration : duration)
    }
    unsafe fn changeReverbBy_duration_(v: f32, duration: NSTimeInterval) -> SKAction
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SKAction").unwrap(), changeReverbBy : v, duration : duration)
    }
    unsafe fn changeObstructionTo_duration_(v: f32, duration: NSTimeInterval) -> SKAction
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SKAction").unwrap(), changeObstructionTo : v, duration : duration)
    }
    unsafe fn changeObstructionBy_duration_(v: f32, duration: NSTimeInterval) -> SKAction
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SKAction").unwrap(), changeObstructionBy : v, duration : duration)
    }
    unsafe fn changeOcclusionTo_duration_(v: f32, duration: NSTimeInterval) -> SKAction
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SKAction").unwrap(), changeOcclusionTo : v, duration : duration)
    }
    unsafe fn changeOcclusionBy_duration_(v: f32, duration: NSTimeInterval) -> SKAction
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SKAction").unwrap(), changeOcclusionBy : v, duration : duration)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct SKCropNode(pub id);
impl std::ops::Deref for SKCropNode {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for SKCropNode {}
impl SKCropNode {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"SKCropNode").unwrap(), alloc) })
    }
}
impl ISKNode for SKCropNode {}
impl PNSCopying for SKCropNode {}
impl PNSSecureCoding for SKCropNode {}
impl std::convert::TryFrom<SKNode> for SKCropNode {
    type Error = &'static str;
    fn try_from(parent: SKNode) -> Result<SKCropNode, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"SKCropNode").unwrap()) };
        if is_kind_of {
            Ok(SKCropNode(parent.0))
        } else {
            Err("This SKNode cannot be downcasted to SKCropNode")
        }
    }
}
impl INSResponder for SKCropNode {}
impl PNSCoding for SKCropNode {}
impl INSObject for SKCropNode {}
impl PNSObject for SKCropNode {}
impl ISKCropNode for SKCropNode {}
pub trait ISKCropNode: Sized + std::ops::Deref {
    unsafe fn maskNode(&self) -> SKNode
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, maskNode)
    }
    unsafe fn setMaskNode_(&self, maskNode: SKNode)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMaskNode : maskNode)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct SKLightNode(pub id);
impl std::ops::Deref for SKLightNode {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for SKLightNode {}
impl SKLightNode {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"SKLightNode").unwrap(), alloc) })
    }
}
impl ISKNode for SKLightNode {}
impl PNSCopying for SKLightNode {}
impl PNSSecureCoding for SKLightNode {}
impl std::convert::TryFrom<SKNode> for SKLightNode {
    type Error = &'static str;
    fn try_from(parent: SKNode) -> Result<SKLightNode, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"SKLightNode").unwrap()) };
        if is_kind_of {
            Ok(SKLightNode(parent.0))
        } else {
            Err("This SKNode cannot be downcasted to SKLightNode")
        }
    }
}
impl INSResponder for SKLightNode {}
impl PNSCoding for SKLightNode {}
impl INSObject for SKLightNode {}
impl PNSObject for SKLightNode {}
impl ISKLightNode for SKLightNode {}
pub trait ISKLightNode: Sized + std::ops::Deref {
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
    unsafe fn lightColor(&self) -> NSColor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, lightColor)
    }
    unsafe fn setLightColor_(&self, lightColor: NSColor)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setLightColor : lightColor)
    }
    unsafe fn ambientColor(&self) -> NSColor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, ambientColor)
    }
    unsafe fn setAmbientColor_(&self, ambientColor: NSColor)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAmbientColor : ambientColor)
    }
    unsafe fn shadowColor(&self) -> NSColor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, shadowColor)
    }
    unsafe fn setShadowColor_(&self, shadowColor: NSColor)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setShadowColor : shadowColor)
    }
    unsafe fn falloff(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, falloff)
    }
    unsafe fn setFalloff_(&self, falloff: CGFloat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setFalloff : falloff)
    }
    unsafe fn categoryBitMask(&self) -> u32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, categoryBitMask)
    }
    unsafe fn setCategoryBitMask_(&self, categoryBitMask: u32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCategoryBitMask : categoryBitMask)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct SKReferenceNode(pub id);
impl std::ops::Deref for SKReferenceNode {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for SKReferenceNode {}
impl SKReferenceNode {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"SKReferenceNode").unwrap(), alloc) })
    }
}
impl ISKNode for SKReferenceNode {}
impl PNSCopying for SKReferenceNode {}
impl PNSSecureCoding for SKReferenceNode {}
impl std::convert::TryFrom<SKNode> for SKReferenceNode {
    type Error = &'static str;
    fn try_from(parent: SKNode) -> Result<SKReferenceNode, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"SKReferenceNode").unwrap()) };
        if is_kind_of {
            Ok(SKReferenceNode(parent.0))
        } else {
            Err("This SKNode cannot be downcasted to SKReferenceNode")
        }
    }
}
impl INSResponder for SKReferenceNode {}
impl PNSCoding for SKReferenceNode {}
impl INSObject for SKReferenceNode {}
impl PNSObject for SKReferenceNode {}
impl ISKReferenceNode for SKReferenceNode {}
pub trait ISKReferenceNode: Sized + std::ops::Deref {
    unsafe fn initWithURL_(&self, url: NSURL) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithURL : url)
    }
    unsafe fn initWithFileNamed_(&self, fileName: NSString) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithFileNamed : fileName)
    }
    unsafe fn initWithCoder_(&self, aDecoder: NSCoder) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithCoder : aDecoder)
    }
    unsafe fn didLoadReferenceNode_(&self, node: SKNode)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, didLoadReferenceNode : node)
    }
    unsafe fn resolveReferenceNode(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, resolveReferenceNode)
    }
    unsafe fn referenceNodeWithFileNamed_(fileName: NSString) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SKReferenceNode").unwrap(), referenceNodeWithFileNamed : fileName)
    }
    unsafe fn referenceNodeWithURL_(referenceURL: NSURL) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SKReferenceNode").unwrap(), referenceNodeWithURL : referenceURL)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct SK3DNode(pub id);
impl std::ops::Deref for SK3DNode {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for SK3DNode {}
impl SK3DNode {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"SK3DNode").unwrap(), alloc) })
    }
}
impl ISKNode for SK3DNode {}
impl PNSCopying for SK3DNode {}
impl PNSSecureCoding for SK3DNode {}
impl std::convert::TryFrom<SKNode> for SK3DNode {
    type Error = &'static str;
    fn try_from(parent: SKNode) -> Result<SK3DNode, Self::Error> {
        let is_kind_of: bool = unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"SK3DNode").unwrap()) };
        if is_kind_of {
            Ok(SK3DNode(parent.0))
        } else {
            Err("This SKNode cannot be downcasted to SK3DNode")
        }
    }
}
impl INSResponder for SK3DNode {}
impl PNSCoding for SK3DNode {}
impl INSObject for SK3DNode {}
impl PNSObject for SK3DNode {}
impl ISK3DNode for SK3DNode {}
pub trait ISK3DNode: Sized + std::ops::Deref {
    unsafe fn initWithViewportSize_(&self, viewportSize: CGSize) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithViewportSize : viewportSize)
    }
    unsafe fn initWithCoder_(&self, aDecoder: NSCoder) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithCoder : aDecoder)
    }
    unsafe fn hitTest_options_(&self, point: CGPoint, options: NSDictionary) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, hitTest : point, options : options)
    }
    unsafe fn projectPoint_(&self, point: vector_float3) -> vector_float3
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, projectPoint : point)
    }
    unsafe fn unprojectPoint_(&self, point: vector_float3) -> vector_float3
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, unprojectPoint : point)
    }
    unsafe fn viewportSize(&self) -> CGSize
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, viewportSize)
    }
    unsafe fn setViewportSize_(&self, viewportSize: CGSize)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setViewportSize : viewportSize)
    }
    unsafe fn scnScene(&self) -> SCNScene
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, scnScene)
    }
    unsafe fn setScnScene_(&self, scnScene: SCNScene)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setScnScene : scnScene)
    }
    unsafe fn sceneTime(&self) -> NSTimeInterval
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, sceneTime)
    }
    unsafe fn setSceneTime_(&self, sceneTime: NSTimeInterval)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSceneTime : sceneTime)
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
    unsafe fn loops(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, loops)
    }
    unsafe fn setLoops_(&self, loops: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setLoops : loops)
    }
    unsafe fn pointOfView(&self) -> SCNNode
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, pointOfView)
    }
    unsafe fn setPointOfView_(&self, pointOfView: SCNNode)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPointOfView : pointOfView)
    }
    unsafe fn autoenablesDefaultLighting(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, autoenablesDefaultLighting)
    }
    unsafe fn setAutoenablesDefaultLighting_(&self, autoenablesDefaultLighting: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAutoenablesDefaultLighting : autoenablesDefaultLighting)
    }
    unsafe fn nodeWithViewportSize_(viewportSize: CGSize) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SK3DNode").unwrap(), nodeWithViewportSize : viewportSize)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct SKTransformNode(pub id);
impl std::ops::Deref for SKTransformNode {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for SKTransformNode {}
impl SKTransformNode {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"SKTransformNode").unwrap(), alloc) })
    }
}
impl ISKNode for SKTransformNode {}
impl PNSCopying for SKTransformNode {}
impl PNSSecureCoding for SKTransformNode {}
impl std::convert::TryFrom<SKNode> for SKTransformNode {
    type Error = &'static str;
    fn try_from(parent: SKNode) -> Result<SKTransformNode, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"SKTransformNode").unwrap()) };
        if is_kind_of {
            Ok(SKTransformNode(parent.0))
        } else {
            Err("This SKNode cannot be downcasted to SKTransformNode")
        }
    }
}
impl INSResponder for SKTransformNode {}
impl PNSCoding for SKTransformNode {}
impl INSObject for SKTransformNode {}
impl PNSObject for SKTransformNode {}
impl ISKTransformNode for SKTransformNode {}
pub trait ISKTransformNode: Sized + std::ops::Deref {
    unsafe fn setEulerAngles_(&self, euler: vector_float3)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setEulerAngles : euler)
    }
    unsafe fn eulerAngles(&self) -> vector_float3
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, eulerAngles)
    }
    unsafe fn setRotationMatrix_(&self, rotationMatrix: matrix_float3x3)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRotationMatrix : rotationMatrix)
    }
    unsafe fn rotationMatrix(&self) -> matrix_float3x3
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, rotationMatrix)
    }
    unsafe fn xRotation(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, xRotation)
    }
    unsafe fn setXRotation_(&self, xRotation: CGFloat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setXRotation : xRotation)
    }
    unsafe fn yRotation(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, yRotation)
    }
    unsafe fn setYRotation_(&self, yRotation: CGFloat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setYRotation : yRotation)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct SKRegion(pub id);
impl std::ops::Deref for SKRegion {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for SKRegion {}
impl SKRegion {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"SKRegion").unwrap(), alloc) })
    }
}
impl PNSCopying for SKRegion {}
impl PNSSecureCoding for SKRegion {}
impl INSObject for SKRegion {}
impl PNSObject for SKRegion {}
impl std::convert::TryFrom<NSObject> for SKRegion {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<SKRegion, Self::Error> {
        let is_kind_of: bool = unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"SKRegion").unwrap()) };
        if is_kind_of {
            Ok(SKRegion(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to SKRegion")
        }
    }
}
impl ISKRegion for SKRegion {}
pub trait ISKRegion: Sized + std::ops::Deref {
    unsafe fn initWithRadius_(&self, radius: f32) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithRadius : radius)
    }
    unsafe fn initWithSize_(&self, size: CGSize) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithSize : size)
    }
    unsafe fn initWithPath_(&self, path: CGPathRef) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithPath : path)
    }
    unsafe fn inverseRegion(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inverseRegion)
    }
    unsafe fn regionByUnionWithRegion_(&self, region: SKRegion) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, regionByUnionWithRegion : region)
    }
    unsafe fn regionByDifferenceFromRegion_(&self, region: SKRegion) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, regionByDifferenceFromRegion : region)
    }
    unsafe fn regionByIntersectionWithRegion_(&self, region: SKRegion) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, regionByIntersectionWithRegion : region)
    }
    unsafe fn containsPoint_(&self, point: CGPoint) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, containsPoint : point)
    }
    unsafe fn path(&self) -> CGPathRef
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, path)
    }
    unsafe fn infiniteRegion() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SKRegion").unwrap(), infiniteRegion)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct SKView(pub id);
impl std::ops::Deref for SKView {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for SKView {}
impl SKView {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"SKView").unwrap(), alloc) })
    }
}
impl PNSSecureCoding for SKView {}
impl INSView for SKView {}
impl PNSAnimatablePropertyContainer for SKView {}
impl PNSUserInterfaceItemIdentification for SKView {}
impl PNSDraggingDestination for SKView {}
impl PNSAppearanceCustomization for SKView {}
impl PNSAccessibilityElement for SKView {}
impl PNSAccessibility for SKView {}
impl std::convert::TryFrom<NSView> for SKView {
    type Error = &'static str;
    fn try_from(parent: NSView) -> Result<SKView, Self::Error> {
        let is_kind_of: bool = unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"SKView").unwrap()) };
        if is_kind_of {
            Ok(SKView(parent.0))
        } else {
            Err("This NSView cannot be downcasted to SKView")
        }
    }
}
impl INSResponder for SKView {}
impl PNSCoding for SKView {}
impl INSObject for SKView {}
impl PNSObject for SKView {}
impl ISKView for SKView {}
pub trait ISKView: Sized + std::ops::Deref {
    unsafe fn presentScene_(&self, scene: SKScene)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, presentScene : scene)
    }
    unsafe fn presentScene_transition_(&self, scene: SKScene, transition: SKTransition)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, presentScene : scene, transition : transition)
    }
    unsafe fn textureFromNode_(&self, node: SKNode) -> SKTexture
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, textureFromNode : node)
    }
    unsafe fn textureFromNode_crop_(&self, node: SKNode, crop: CGRect) -> SKTexture
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, textureFromNode : node, crop : crop)
    }
    unsafe fn convertPoint_toScene_(&self, point: CGPoint, scene: SKScene) -> CGPoint
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, convertPoint : point, toScene : scene)
    }
    unsafe fn convertPoint_fromScene_(&self, point: CGPoint, scene: SKScene) -> CGPoint
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, convertPoint : point, fromScene : scene)
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
    unsafe fn showsFPS(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, showsFPS)
    }
    unsafe fn setShowsFPS_(&self, showsFPS: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setShowsFPS : showsFPS)
    }
    unsafe fn showsDrawCount(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, showsDrawCount)
    }
    unsafe fn setShowsDrawCount_(&self, showsDrawCount: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setShowsDrawCount : showsDrawCount)
    }
    unsafe fn showsNodeCount(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, showsNodeCount)
    }
    unsafe fn setShowsNodeCount_(&self, showsNodeCount: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setShowsNodeCount : showsNodeCount)
    }
    unsafe fn showsQuadCount(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, showsQuadCount)
    }
    unsafe fn setShowsQuadCount_(&self, showsQuadCount: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setShowsQuadCount : showsQuadCount)
    }
    unsafe fn showsPhysics(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, showsPhysics)
    }
    unsafe fn setShowsPhysics_(&self, showsPhysics: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setShowsPhysics : showsPhysics)
    }
    unsafe fn showsFields(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, showsFields)
    }
    unsafe fn setShowsFields_(&self, showsFields: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setShowsFields : showsFields)
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
    unsafe fn allowsTransparency(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, allowsTransparency)
    }
    unsafe fn setAllowsTransparency_(&self, allowsTransparency: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAllowsTransparency : allowsTransparency)
    }
    unsafe fn ignoresSiblingOrder(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, ignoresSiblingOrder)
    }
    unsafe fn setIgnoresSiblingOrder_(&self, ignoresSiblingOrder: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setIgnoresSiblingOrder : ignoresSiblingOrder)
    }
    unsafe fn shouldCullNonVisibleNodes(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, shouldCullNonVisibleNodes)
    }
    unsafe fn setShouldCullNonVisibleNodes_(&self, shouldCullNonVisibleNodes: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setShouldCullNonVisibleNodes : shouldCullNonVisibleNodes)
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
    unsafe fn disableDepthStencilBuffer(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, disableDepthStencilBuffer)
    }
    unsafe fn setDisableDepthStencilBuffer_(&self, disableDepthStencilBuffer: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDisableDepthStencilBuffer : disableDepthStencilBuffer)
    }
    unsafe fn delegate(&self) -> NSObject
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, delegate)
    }
    unsafe fn setDelegate_(&self, delegate: NSObject)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDelegate : delegate)
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
    unsafe fn preferredFrameRate(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, preferredFrameRate)
    }
    unsafe fn setPreferredFrameRate_(&self, preferredFrameRate: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPreferredFrameRate : preferredFrameRate)
    }
    unsafe fn scene(&self) -> SKScene
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, scene)
    }
}
pub trait PSKViewDelegate: Sized + std::ops::Deref {
    unsafe fn view_shouldRenderAtTime_(&self, view: SKView, time: NSTimeInterval) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, view : view, shouldRenderAtTime : time)
    }
}
pub type SKTransitionDirection = NSInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct SKTransition(pub id);
impl std::ops::Deref for SKTransition {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for SKTransition {}
impl SKTransition {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"SKTransition").unwrap(), alloc) })
    }
}
impl PNSCopying for SKTransition {}
impl INSObject for SKTransition {}
impl PNSObject for SKTransition {}
impl std::convert::TryFrom<NSObject> for SKTransition {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<SKTransition, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"SKTransition").unwrap()) };
        if is_kind_of {
            Ok(SKTransition(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to SKTransition")
        }
    }
}
impl ISKTransition for SKTransition {}
pub trait ISKTransition: Sized + std::ops::Deref {
    unsafe fn pausesIncomingScene(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, pausesIncomingScene)
    }
    unsafe fn setPausesIncomingScene_(&self, pausesIncomingScene: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPausesIncomingScene : pausesIncomingScene)
    }
    unsafe fn pausesOutgoingScene(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, pausesOutgoingScene)
    }
    unsafe fn setPausesOutgoingScene_(&self, pausesOutgoingScene: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPausesOutgoingScene : pausesOutgoingScene)
    }
    unsafe fn crossFadeWithDuration_(sec: NSTimeInterval) -> SKTransition
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SKTransition").unwrap(), crossFadeWithDuration : sec)
    }
    unsafe fn fadeWithDuration_(sec: NSTimeInterval) -> SKTransition
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SKTransition").unwrap(), fadeWithDuration : sec)
    }
    unsafe fn fadeWithColor_duration_(color: NSColor, sec: NSTimeInterval) -> SKTransition
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SKTransition").unwrap(), fadeWithColor : color, duration : sec)
    }
    unsafe fn flipHorizontalWithDuration_(sec: NSTimeInterval) -> SKTransition
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SKTransition").unwrap(), flipHorizontalWithDuration : sec)
    }
    unsafe fn flipVerticalWithDuration_(sec: NSTimeInterval) -> SKTransition
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SKTransition").unwrap(), flipVerticalWithDuration : sec)
    }
    unsafe fn revealWithDirection_duration_(
        direction: SKTransitionDirection,
        sec: NSTimeInterval,
    ) -> SKTransition
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SKTransition").unwrap(), revealWithDirection : direction, duration : sec)
    }
    unsafe fn moveInWithDirection_duration_(
        direction: SKTransitionDirection,
        sec: NSTimeInterval,
    ) -> SKTransition
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SKTransition").unwrap(), moveInWithDirection : direction, duration : sec)
    }
    unsafe fn pushWithDirection_duration_(
        direction: SKTransitionDirection,
        sec: NSTimeInterval,
    ) -> SKTransition
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SKTransition").unwrap(), pushWithDirection : direction, duration : sec)
    }
    unsafe fn doorsOpenHorizontalWithDuration_(sec: NSTimeInterval) -> SKTransition
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SKTransition").unwrap(), doorsOpenHorizontalWithDuration : sec)
    }
    unsafe fn doorsOpenVerticalWithDuration_(sec: NSTimeInterval) -> SKTransition
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SKTransition").unwrap(), doorsOpenVerticalWithDuration : sec)
    }
    unsafe fn doorsCloseHorizontalWithDuration_(sec: NSTimeInterval) -> SKTransition
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SKTransition").unwrap(), doorsCloseHorizontalWithDuration : sec)
    }
    unsafe fn doorsCloseVerticalWithDuration_(sec: NSTimeInterval) -> SKTransition
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SKTransition").unwrap(), doorsCloseVerticalWithDuration : sec)
    }
    unsafe fn doorwayWithDuration_(sec: NSTimeInterval) -> SKTransition
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SKTransition").unwrap(), doorwayWithDuration : sec)
    }
    unsafe fn transitionWithCIFilter_duration_(
        filter: CIFilter,
        sec: NSTimeInterval,
    ) -> SKTransition
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SKTransition").unwrap(), transitionWithCIFilter : filter, duration : sec)
    }
}
pub type SKTextureFilteringMode = NSInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct SKTexture(pub id);
impl std::ops::Deref for SKTexture {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for SKTexture {}
impl SKTexture {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"SKTexture").unwrap(), alloc) })
    }
}
impl PNSCopying for SKTexture {}
impl PNSSecureCoding for SKTexture {}
impl INSObject for SKTexture {}
impl PNSObject for SKTexture {}
impl std::convert::TryFrom<NSObject> for SKTexture {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<SKTexture, Self::Error> {
        let is_kind_of: bool = unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"SKTexture").unwrap()) };
        if is_kind_of {
            Ok(SKTexture(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to SKTexture")
        }
    }
}
impl ISKTexture for SKTexture {}
pub trait ISKTexture: Sized + std::ops::Deref {
    unsafe fn textureByApplyingCIFilter_(&self, filter: CIFilter) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, textureByApplyingCIFilter : filter)
    }
    unsafe fn textureByGeneratingNormalMap(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, textureByGeneratingNormalMap)
    }
    unsafe fn textureByGeneratingNormalMapWithSmoothness_contrast_(
        &self,
        smoothness: CGFloat,
        contrast: CGFloat,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, textureByGeneratingNormalMapWithSmoothness : smoothness, contrast : contrast)
    }
    unsafe fn textureRect(&self) -> CGRect
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, textureRect)
    }
    unsafe fn size(&self) -> CGSize
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, size)
    }
    unsafe fn CGImage(&self) -> CGImageRef
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, CGImage)
    }
    unsafe fn preloadWithCompletionHandler_(&self, completionHandler: *mut ::std::os::raw::c_void)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, preloadWithCompletionHandler : completionHandler)
    }
    unsafe fn filteringMode(&self) -> SKTextureFilteringMode
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, filteringMode)
    }
    unsafe fn setFilteringMode_(&self, filteringMode: SKTextureFilteringMode)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setFilteringMode : filteringMode)
    }
    unsafe fn usesMipmaps(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, usesMipmaps)
    }
    unsafe fn setUsesMipmaps_(&self, usesMipmaps: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setUsesMipmaps : usesMipmaps)
    }
    unsafe fn textureWithImageNamed_(name: NSString) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SKTexture").unwrap(), textureWithImageNamed : name)
    }
    unsafe fn textureWithRect_inTexture_(rect: CGRect, texture: SKTexture) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SKTexture").unwrap(), textureWithRect : rect, inTexture : texture)
    }
    unsafe fn textureVectorNoiseWithSmoothness_size_(
        smoothness: CGFloat,
        size: CGSize,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SKTexture").unwrap(), textureVectorNoiseWithSmoothness : smoothness, size : size)
    }
    unsafe fn textureNoiseWithSmoothness_size_grayscale_(
        smoothness: CGFloat,
        size: CGSize,
        grayscale: BOOL,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SKTexture").unwrap(), textureNoiseWithSmoothness : smoothness, size : size, grayscale : grayscale)
    }
    unsafe fn textureWithCGImage_(image: CGImageRef) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SKTexture").unwrap(), textureWithCGImage : image)
    }
    unsafe fn textureWithImage_(image: NSImage) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SKTexture").unwrap(), textureWithImage : image)
    }
    unsafe fn textureWithData_size_(pixelData: NSData, size: CGSize) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SKTexture").unwrap(), textureWithData : pixelData, size : size)
    }
    unsafe fn textureWithData_size_flipped_(
        pixelData: NSData,
        size: CGSize,
        flipped: BOOL,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SKTexture").unwrap(), textureWithData : pixelData, size : size, flipped : flipped)
    }
    unsafe fn textureWithData_size_rowLength_alignment_(
        pixelData: NSData,
        size: CGSize,
        rowLength: ::std::os::raw::c_uint,
        alignment: ::std::os::raw::c_uint,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SKTexture").unwrap(), textureWithData : pixelData, size : size, rowLength : rowLength, alignment : alignment)
    }
    unsafe fn preloadTextures_withCompletionHandler_(
        textures: NSArray,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SKTexture").unwrap(), preloadTextures : textures, withCompletionHandler : completionHandler)
    }
}
pub type SKUniformType = NSInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct SKUniform(pub id);
impl std::ops::Deref for SKUniform {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for SKUniform {}
impl SKUniform {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"SKUniform").unwrap(), alloc) })
    }
}
impl PNSCopying for SKUniform {}
impl PNSSecureCoding for SKUniform {}
impl INSObject for SKUniform {}
impl PNSObject for SKUniform {}
impl std::convert::TryFrom<NSObject> for SKUniform {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<SKUniform, Self::Error> {
        let is_kind_of: bool = unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"SKUniform").unwrap()) };
        if is_kind_of {
            Ok(SKUniform(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to SKUniform")
        }
    }
}
impl ISKUniform for SKUniform {}
pub trait ISKUniform: Sized + std::ops::Deref {
    unsafe fn initWithName_(&self, name: NSString) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithName : name)
    }
    unsafe fn initWithName_texture_(&self, name: NSString, texture: SKTexture) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithName : name, texture : texture)
    }
    unsafe fn initWithName_float_(&self, name: NSString, value: f32) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithName : name, float : value)
    }
    unsafe fn initWithName_vectorFloat2_(
        &self,
        name: NSString,
        value: vector_float2,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithName : name, vectorFloat2 : value)
    }
    unsafe fn initWithName_vectorFloat3_(
        &self,
        name: NSString,
        value: vector_float3,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithName : name, vectorFloat3 : value)
    }
    unsafe fn initWithName_vectorFloat4_(
        &self,
        name: NSString,
        value: vector_float4,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithName : name, vectorFloat4 : value)
    }
    unsafe fn initWithName_matrixFloat2x2_(
        &self,
        name: NSString,
        value: matrix_float2x2,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithName : name, matrixFloat2x2 : value)
    }
    unsafe fn initWithName_matrixFloat3x3_(
        &self,
        name: NSString,
        value: matrix_float3x3,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithName : name, matrixFloat3x3 : value)
    }
    unsafe fn initWithName_matrixFloat4x4_(
        &self,
        name: NSString,
        value: matrix_float4x4,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithName : name, matrixFloat4x4 : value)
    }
    unsafe fn initWithName_floatVector2_(&self, name: NSString, value: GLKVector2) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithName : name, floatVector2 : value)
    }
    unsafe fn initWithName_floatVector3_(&self, name: NSString, value: GLKVector3) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithName : name, floatVector3 : value)
    }
    unsafe fn initWithName_floatVector4_(&self, name: NSString, value: GLKVector4) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithName : name, floatVector4 : value)
    }
    unsafe fn initWithName_floatMatrix2_(&self, name: NSString, value: GLKMatrix2) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithName : name, floatMatrix2 : value)
    }
    unsafe fn initWithName_floatMatrix3_(&self, name: NSString, value: GLKMatrix3) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithName : name, floatMatrix3 : value)
    }
    unsafe fn initWithName_floatMatrix4_(&self, name: NSString, value: GLKMatrix4) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithName : name, floatMatrix4 : value)
    }
    unsafe fn name(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, name)
    }
    unsafe fn uniformType(&self) -> SKUniformType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, uniformType)
    }
    unsafe fn textureValue(&self) -> SKTexture
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, textureValue)
    }
    unsafe fn setTextureValue_(&self, textureValue: SKTexture)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setTextureValue : textureValue)
    }
    unsafe fn floatValue(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, floatValue)
    }
    unsafe fn setFloatValue_(&self, floatValue: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setFloatValue : floatValue)
    }
    unsafe fn vectorFloat2Value(&self) -> vector_float2
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, vectorFloat2Value)
    }
    unsafe fn setVectorFloat2Value_(&self, vectorFloat2Value: vector_float2)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setVectorFloat2Value : vectorFloat2Value)
    }
    unsafe fn vectorFloat3Value(&self) -> vector_float3
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, vectorFloat3Value)
    }
    unsafe fn setVectorFloat3Value_(&self, vectorFloat3Value: vector_float3)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setVectorFloat3Value : vectorFloat3Value)
    }
    unsafe fn vectorFloat4Value(&self) -> vector_float4
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, vectorFloat4Value)
    }
    unsafe fn setVectorFloat4Value_(&self, vectorFloat4Value: vector_float4)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setVectorFloat4Value : vectorFloat4Value)
    }
    unsafe fn matrixFloat2x2Value(&self) -> matrix_float2x2
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, matrixFloat2x2Value)
    }
    unsafe fn setMatrixFloat2x2Value_(&self, matrixFloat2x2Value: matrix_float2x2)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMatrixFloat2x2Value : matrixFloat2x2Value)
    }
    unsafe fn matrixFloat3x3Value(&self) -> matrix_float3x3
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, matrixFloat3x3Value)
    }
    unsafe fn setMatrixFloat3x3Value_(&self, matrixFloat3x3Value: matrix_float3x3)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMatrixFloat3x3Value : matrixFloat3x3Value)
    }
    unsafe fn matrixFloat4x4Value(&self) -> matrix_float4x4
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, matrixFloat4x4Value)
    }
    unsafe fn setMatrixFloat4x4Value_(&self, matrixFloat4x4Value: matrix_float4x4)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMatrixFloat4x4Value : matrixFloat4x4Value)
    }
    unsafe fn floatVector2Value(&self) -> GLKVector2
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, floatVector2Value)
    }
    unsafe fn setFloatVector2Value_(&self, floatVector2Value: GLKVector2)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setFloatVector2Value : floatVector2Value)
    }
    unsafe fn floatVector3Value(&self) -> GLKVector3
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, floatVector3Value)
    }
    unsafe fn setFloatVector3Value_(&self, floatVector3Value: GLKVector3)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setFloatVector3Value : floatVector3Value)
    }
    unsafe fn floatVector4Value(&self) -> GLKVector4
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, floatVector4Value)
    }
    unsafe fn setFloatVector4Value_(&self, floatVector4Value: GLKVector4)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setFloatVector4Value : floatVector4Value)
    }
    unsafe fn floatMatrix2Value(&self) -> GLKMatrix2
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, floatMatrix2Value)
    }
    unsafe fn setFloatMatrix2Value_(&self, floatMatrix2Value: GLKMatrix2)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setFloatMatrix2Value : floatMatrix2Value)
    }
    unsafe fn floatMatrix3Value(&self) -> GLKMatrix3
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, floatMatrix3Value)
    }
    unsafe fn setFloatMatrix3Value_(&self, floatMatrix3Value: GLKMatrix3)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setFloatMatrix3Value : floatMatrix3Value)
    }
    unsafe fn floatMatrix4Value(&self) -> GLKMatrix4
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, floatMatrix4Value)
    }
    unsafe fn setFloatMatrix4Value_(&self, floatMatrix4Value: GLKMatrix4)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setFloatMatrix4Value : floatMatrix4Value)
    }
    unsafe fn uniformWithName_(name: NSString) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SKUniform").unwrap(), uniformWithName : name)
    }
    unsafe fn uniformWithName_texture_(name: NSString, texture: SKTexture) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SKUniform").unwrap(), uniformWithName : name, texture : texture)
    }
    unsafe fn uniformWithName_float_(name: NSString, value: f32) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SKUniform").unwrap(), uniformWithName : name, float : value)
    }
    unsafe fn uniformWithName_vectorFloat2_(name: NSString, value: vector_float2) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SKUniform").unwrap(), uniformWithName : name, vectorFloat2 : value)
    }
    unsafe fn uniformWithName_vectorFloat3_(name: NSString, value: vector_float3) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SKUniform").unwrap(), uniformWithName : name, vectorFloat3 : value)
    }
    unsafe fn uniformWithName_vectorFloat4_(name: NSString, value: vector_float4) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SKUniform").unwrap(), uniformWithName : name, vectorFloat4 : value)
    }
    unsafe fn uniformWithName_matrixFloat2x2_(
        name: NSString,
        value: matrix_float2x2,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SKUniform").unwrap(), uniformWithName : name, matrixFloat2x2 : value)
    }
    unsafe fn uniformWithName_matrixFloat3x3_(
        name: NSString,
        value: matrix_float3x3,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SKUniform").unwrap(), uniformWithName : name, matrixFloat3x3 : value)
    }
    unsafe fn uniformWithName_matrixFloat4x4_(
        name: NSString,
        value: matrix_float4x4,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SKUniform").unwrap(), uniformWithName : name, matrixFloat4x4 : value)
    }
    unsafe fn uniformWithName_floatVector2_(name: NSString, value: GLKVector2) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SKUniform").unwrap(), uniformWithName : name, floatVector2 : value)
    }
    unsafe fn uniformWithName_floatVector3_(name: NSString, value: GLKVector3) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SKUniform").unwrap(), uniformWithName : name, floatVector3 : value)
    }
    unsafe fn uniformWithName_floatVector4_(name: NSString, value: GLKVector4) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SKUniform").unwrap(), uniformWithName : name, floatVector4 : value)
    }
    unsafe fn uniformWithName_floatMatrix2_(name: NSString, value: GLKMatrix2) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SKUniform").unwrap(), uniformWithName : name, floatMatrix2 : value)
    }
    unsafe fn uniformWithName_floatMatrix3_(name: NSString, value: GLKMatrix3) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SKUniform").unwrap(), uniformWithName : name, floatMatrix3 : value)
    }
    unsafe fn uniformWithName_floatMatrix4_(name: NSString, value: GLKMatrix4) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SKUniform").unwrap(), uniformWithName : name, floatMatrix4 : value)
    }
}
pub type SKAttributeType = NSInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct SKAttribute(pub id);
impl std::ops::Deref for SKAttribute {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for SKAttribute {}
impl SKAttribute {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"SKAttribute").unwrap(), alloc) })
    }
}
impl PNSSecureCoding for SKAttribute {}
impl INSObject for SKAttribute {}
impl PNSObject for SKAttribute {}
impl std::convert::TryFrom<NSObject> for SKAttribute {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<SKAttribute, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"SKAttribute").unwrap()) };
        if is_kind_of {
            Ok(SKAttribute(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to SKAttribute")
        }
    }
}
impl ISKAttribute for SKAttribute {}
pub trait ISKAttribute: Sized + std::ops::Deref {
    unsafe fn initWithName_type_(&self, name: NSString, type_: SKAttributeType) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithName : name, r#type : type_)
    }
    unsafe fn name(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, name)
    }
    unsafe fn type_(&self) -> SKAttributeType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, type)
    }
    unsafe fn attributeWithName_type_(name: NSString, type_: SKAttributeType) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SKAttribute").unwrap(), attributeWithName : name, r#type : type_)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct SKAttributeValue(pub id);
impl std::ops::Deref for SKAttributeValue {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for SKAttributeValue {}
impl SKAttributeValue {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"SKAttributeValue").unwrap(), alloc) })
    }
}
impl PNSSecureCoding for SKAttributeValue {}
impl INSObject for SKAttributeValue {}
impl PNSObject for SKAttributeValue {}
impl std::convert::TryFrom<NSObject> for SKAttributeValue {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<SKAttributeValue, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"SKAttributeValue").unwrap()) };
        if is_kind_of {
            Ok(SKAttributeValue(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to SKAttributeValue")
        }
    }
}
impl ISKAttributeValue for SKAttributeValue {}
pub trait ISKAttributeValue: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn floatValue(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, floatValue)
    }
    unsafe fn setFloatValue_(&self, floatValue: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setFloatValue : floatValue)
    }
    unsafe fn vectorFloat2Value(&self) -> vector_float2
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, vectorFloat2Value)
    }
    unsafe fn setVectorFloat2Value_(&self, vectorFloat2Value: vector_float2)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setVectorFloat2Value : vectorFloat2Value)
    }
    unsafe fn vectorFloat3Value(&self) -> vector_float3
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, vectorFloat3Value)
    }
    unsafe fn setVectorFloat3Value_(&self, vectorFloat3Value: vector_float3)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setVectorFloat3Value : vectorFloat3Value)
    }
    unsafe fn vectorFloat4Value(&self) -> vector_float4
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, vectorFloat4Value)
    }
    unsafe fn setVectorFloat4Value_(&self, vectorFloat4Value: vector_float4)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setVectorFloat4Value : vectorFloat4Value)
    }
    unsafe fn valueWithFloat_(value: f32) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SKAttributeValue").unwrap(), valueWithFloat : value)
    }
    unsafe fn valueWithVectorFloat2_(value: vector_float2) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SKAttributeValue").unwrap(), valueWithVectorFloat2 : value)
    }
    unsafe fn valueWithVectorFloat3_(value: vector_float3) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SKAttributeValue").unwrap(), valueWithVectorFloat3 : value)
    }
    unsafe fn valueWithVectorFloat4_(value: vector_float4) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SKAttributeValue").unwrap(), valueWithVectorFloat4 : value)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct SKRenderer(pub id);
impl std::ops::Deref for SKRenderer {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for SKRenderer {}
impl SKRenderer {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"SKRenderer").unwrap(), alloc) })
    }
}
impl INSObject for SKRenderer {}
impl PNSObject for SKRenderer {}
impl std::convert::TryFrom<NSObject> for SKRenderer {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<SKRenderer, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"SKRenderer").unwrap()) };
        if is_kind_of {
            Ok(SKRenderer(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to SKRenderer")
        }
    }
}
impl ISKRenderer for SKRenderer {}
pub trait ISKRenderer: Sized + std::ops::Deref {
    unsafe fn renderWithViewport_commandBuffer_renderPassDescriptor_(
        &self,
        viewport: CGRect,
        commandBuffer: *mut u64,
        renderPassDescriptor: MTLRenderPassDescriptor,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, renderWithViewport : viewport, commandBuffer : commandBuffer, renderPassDescriptor : renderPassDescriptor)
    }
    unsafe fn renderWithViewport_renderCommandEncoder_renderPassDescriptor_commandQueue_(
        &self,
        viewport: CGRect,
        renderCommandEncoder: *mut u64,
        renderPassDescriptor: MTLRenderPassDescriptor,
        commandQueue: *mut u64,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, renderWithViewport : viewport, renderCommandEncoder : renderCommandEncoder, renderPassDescriptor : renderPassDescriptor, commandQueue : commandQueue)
    }
    unsafe fn updateAtTime_(&self, currentTime: NSTimeInterval)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, updateAtTime : currentTime)
    }
    unsafe fn scene(&self) -> SKScene
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, scene)
    }
    unsafe fn setScene_(&self, scene: SKScene)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setScene : scene)
    }
    unsafe fn ignoresSiblingOrder(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, ignoresSiblingOrder)
    }
    unsafe fn setIgnoresSiblingOrder_(&self, ignoresSiblingOrder: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setIgnoresSiblingOrder : ignoresSiblingOrder)
    }
    unsafe fn shouldCullNonVisibleNodes(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, shouldCullNonVisibleNodes)
    }
    unsafe fn setShouldCullNonVisibleNodes_(&self, shouldCullNonVisibleNodes: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setShouldCullNonVisibleNodes : shouldCullNonVisibleNodes)
    }
    unsafe fn showsDrawCount(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, showsDrawCount)
    }
    unsafe fn setShowsDrawCount_(&self, showsDrawCount: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setShowsDrawCount : showsDrawCount)
    }
    unsafe fn showsNodeCount(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, showsNodeCount)
    }
    unsafe fn setShowsNodeCount_(&self, showsNodeCount: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setShowsNodeCount : showsNodeCount)
    }
    unsafe fn showsQuadCount(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, showsQuadCount)
    }
    unsafe fn setShowsQuadCount_(&self, showsQuadCount: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setShowsQuadCount : showsQuadCount)
    }
    unsafe fn showsPhysics(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, showsPhysics)
    }
    unsafe fn setShowsPhysics_(&self, showsPhysics: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setShowsPhysics : showsPhysics)
    }
    unsafe fn showsFields(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, showsFields)
    }
    unsafe fn setShowsFields_(&self, showsFields: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setShowsFields : showsFields)
    }
    unsafe fn rendererWithDevice_(device: *mut u64) -> SKRenderer
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SKRenderer").unwrap(), rendererWithDevice : device)
    }
}
pub type SKTileDefinitionRotation = NSUInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct SKTileDefinition(pub id);
impl std::ops::Deref for SKTileDefinition {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for SKTileDefinition {}
impl SKTileDefinition {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"SKTileDefinition").unwrap(), alloc) })
    }
}
impl PNSCopying for SKTileDefinition {}
impl PNSSecureCoding for SKTileDefinition {}
impl INSObject for SKTileDefinition {}
impl PNSObject for SKTileDefinition {}
impl std::convert::TryFrom<NSObject> for SKTileDefinition {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<SKTileDefinition, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"SKTileDefinition").unwrap()) };
        if is_kind_of {
            Ok(SKTileDefinition(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to SKTileDefinition")
        }
    }
}
impl ISKTileDefinition for SKTileDefinition {}
pub trait ISKTileDefinition: Sized + std::ops::Deref {
    unsafe fn initWithTexture_(&self, texture: SKTexture) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithTexture : texture)
    }
    unsafe fn initWithTexture_size_(&self, texture: SKTexture, size: CGSize) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithTexture : texture, size : size)
    }
    unsafe fn initWithTexture_normalTexture_size_(
        &self,
        texture: SKTexture,
        normalTexture: SKTexture,
        size: CGSize,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithTexture : texture, normalTexture : normalTexture, size : size)
    }
    unsafe fn initWithTextures_size_timePerFrame_(
        &self,
        textures: NSArray,
        size: CGSize,
        timePerFrame: CGFloat,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithTextures : textures, size : size, timePerFrame : timePerFrame)
    }
    unsafe fn initWithTextures_normalTextures_size_timePerFrame_(
        &self,
        textures: NSArray,
        normalTextures: NSArray,
        size: CGSize,
        timePerFrame: CGFloat,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithTextures : textures, normalTextures : normalTextures, size : size, timePerFrame : timePerFrame)
    }
    unsafe fn textures(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, textures)
    }
    unsafe fn setTextures_(&self, textures: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setTextures : textures)
    }
    unsafe fn normalTextures(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, normalTextures)
    }
    unsafe fn setNormalTextures_(&self, normalTextures: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setNormalTextures : normalTextures)
    }
    unsafe fn userData(&self) -> NSMutableDictionary
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, userData)
    }
    unsafe fn setUserData_(&self, userData: NSMutableDictionary)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setUserData : userData)
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
    unsafe fn timePerFrame(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, timePerFrame)
    }
    unsafe fn setTimePerFrame_(&self, timePerFrame: CGFloat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setTimePerFrame : timePerFrame)
    }
    unsafe fn placementWeight(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, placementWeight)
    }
    unsafe fn setPlacementWeight_(&self, placementWeight: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPlacementWeight : placementWeight)
    }
    unsafe fn rotation(&self) -> SKTileDefinitionRotation
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, rotation)
    }
    unsafe fn setRotation_(&self, rotation: SKTileDefinitionRotation)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRotation : rotation)
    }
    unsafe fn flipVertically(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, flipVertically)
    }
    unsafe fn setFlipVertically_(&self, flipVertically: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setFlipVertically : flipVertically)
    }
    unsafe fn flipHorizontally(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, flipHorizontally)
    }
    unsafe fn setFlipHorizontally_(&self, flipHorizontally: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setFlipHorizontally : flipHorizontally)
    }
    unsafe fn tileDefinitionWithTexture_(texture: SKTexture) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SKTileDefinition").unwrap(), tileDefinitionWithTexture : texture)
    }
    unsafe fn tileDefinitionWithTexture_size_(texture: SKTexture, size: CGSize) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SKTileDefinition").unwrap(), tileDefinitionWithTexture : texture, size : size)
    }
    unsafe fn tileDefinitionWithTexture_normalTexture_size_(
        texture: SKTexture,
        normalTexture: SKTexture,
        size: CGSize,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SKTileDefinition").unwrap(), tileDefinitionWithTexture : texture, normalTexture : normalTexture, size : size)
    }
    unsafe fn tileDefinitionWithTextures_size_timePerFrame_(
        textures: NSArray,
        size: CGSize,
        timePerFrame: CGFloat,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SKTileDefinition").unwrap(), tileDefinitionWithTextures : textures, size : size, timePerFrame : timePerFrame)
    }
    unsafe fn tileDefinitionWithTextures_normalTextures_size_timePerFrame_(
        textures: NSArray,
        normalTextures: NSArray,
        size: CGSize,
        timePerFrame: CGFloat,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SKTileDefinition").unwrap(), tileDefinitionWithTextures : textures, normalTextures : normalTextures, size : size, timePerFrame : timePerFrame)
    }
}
pub type SKTileSetType = NSUInteger;
pub type SKTileAdjacencyMask = NSUInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct SKTileSet(pub id);
impl std::ops::Deref for SKTileSet {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for SKTileSet {}
impl SKTileSet {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"SKTileSet").unwrap(), alloc) })
    }
}
impl PNSCopying for SKTileSet {}
impl PNSSecureCoding for SKTileSet {}
impl INSObject for SKTileSet {}
impl PNSObject for SKTileSet {}
impl std::convert::TryFrom<NSObject> for SKTileSet {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<SKTileSet, Self::Error> {
        let is_kind_of: bool = unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"SKTileSet").unwrap()) };
        if is_kind_of {
            Ok(SKTileSet(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to SKTileSet")
        }
    }
}
impl ISKTileSet for SKTileSet {}
pub trait ISKTileSet: Sized + std::ops::Deref {
    unsafe fn initWithTileGroups_(&self, tileGroups: NSArray) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithTileGroups : tileGroups)
    }
    unsafe fn initWithTileGroups_tileSetType_(
        &self,
        tileGroups: NSArray,
        tileSetType: SKTileSetType,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithTileGroups : tileGroups, tileSetType : tileSetType)
    }
    unsafe fn tileGroups(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, tileGroups)
    }
    unsafe fn setTileGroups_(&self, tileGroups: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setTileGroups : tileGroups)
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
    unsafe fn type_(&self) -> SKTileSetType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, type)
    }
    unsafe fn setType_(&self, type_: SKTileSetType)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setType : type_)
    }
    unsafe fn defaultTileGroup(&self) -> SKTileGroup
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, defaultTileGroup)
    }
    unsafe fn setDefaultTileGroup_(&self, defaultTileGroup: SKTileGroup)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDefaultTileGroup : defaultTileGroup)
    }
    unsafe fn defaultTileSize(&self) -> CGSize
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, defaultTileSize)
    }
    unsafe fn setDefaultTileSize_(&self, defaultTileSize: CGSize)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDefaultTileSize : defaultTileSize)
    }
    unsafe fn tileSetWithTileGroups_(tileGroups: NSArray) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SKTileSet").unwrap(), tileSetWithTileGroups : tileGroups)
    }
    unsafe fn tileSetWithTileGroups_tileSetType_(
        tileGroups: NSArray,
        tileSetType: SKTileSetType,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SKTileSet").unwrap(), tileSetWithTileGroups : tileGroups, tileSetType : tileSetType)
    }
    unsafe fn tileSetNamed_(name: NSString) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SKTileSet").unwrap(), tileSetNamed : name)
    }
    unsafe fn tileSetFromURL_(url: NSURL) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SKTileSet").unwrap(), tileSetFromURL : url)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct SKTileGroup(pub id);
impl std::ops::Deref for SKTileGroup {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for SKTileGroup {}
impl SKTileGroup {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"SKTileGroup").unwrap(), alloc) })
    }
}
impl PNSCopying for SKTileGroup {}
impl PNSSecureCoding for SKTileGroup {}
impl INSObject for SKTileGroup {}
impl PNSObject for SKTileGroup {}
impl std::convert::TryFrom<NSObject> for SKTileGroup {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<SKTileGroup, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"SKTileGroup").unwrap()) };
        if is_kind_of {
            Ok(SKTileGroup(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to SKTileGroup")
        }
    }
}
impl ISKTileGroup for SKTileGroup {}
pub trait ISKTileGroup: Sized + std::ops::Deref {
    unsafe fn initWithTileDefinition_(&self, tileDefinition: SKTileDefinition) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithTileDefinition : tileDefinition)
    }
    unsafe fn initWithRules_(&self, rules: NSArray) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithRules : rules)
    }
    unsafe fn rules(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, rules)
    }
    unsafe fn setRules_(&self, rules: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRules : rules)
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
    unsafe fn tileGroupWithTileDefinition_(tileDefinition: SKTileDefinition) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SKTileGroup").unwrap(), tileGroupWithTileDefinition : tileDefinition)
    }
    unsafe fn tileGroupWithRules_(rules: NSArray) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SKTileGroup").unwrap(), tileGroupWithRules : rules)
    }
    unsafe fn emptyTileGroup() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SKTileGroup").unwrap(), emptyTileGroup)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct SKTileGroupRule(pub id);
impl std::ops::Deref for SKTileGroupRule {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for SKTileGroupRule {}
impl SKTileGroupRule {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"SKTileGroupRule").unwrap(), alloc) })
    }
}
impl PNSCopying for SKTileGroupRule {}
impl PNSSecureCoding for SKTileGroupRule {}
impl INSObject for SKTileGroupRule {}
impl PNSObject for SKTileGroupRule {}
impl std::convert::TryFrom<NSObject> for SKTileGroupRule {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<SKTileGroupRule, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"SKTileGroupRule").unwrap()) };
        if is_kind_of {
            Ok(SKTileGroupRule(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to SKTileGroupRule")
        }
    }
}
impl ISKTileGroupRule for SKTileGroupRule {}
pub trait ISKTileGroupRule: Sized + std::ops::Deref {
    unsafe fn initWithAdjacency_tileDefinitions_(
        &self,
        adjacency: SKTileAdjacencyMask,
        tileDefinitions: NSArray,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithAdjacency : adjacency, tileDefinitions : tileDefinitions)
    }
    unsafe fn adjacency(&self) -> SKTileAdjacencyMask
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, adjacency)
    }
    unsafe fn setAdjacency_(&self, adjacency: SKTileAdjacencyMask)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAdjacency : adjacency)
    }
    unsafe fn tileDefinitions(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, tileDefinitions)
    }
    unsafe fn setTileDefinitions_(&self, tileDefinitions: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setTileDefinitions : tileDefinitions)
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
    unsafe fn tileGroupRuleWithAdjacency_tileDefinitions_(
        adjacency: SKTileAdjacencyMask,
        tileDefinitions: NSArray,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SKTileGroupRule").unwrap(), tileGroupRuleWithAdjacency : adjacency, tileDefinitions : tileDefinitions)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct SKTileMapNode(pub id);
impl std::ops::Deref for SKTileMapNode {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for SKTileMapNode {}
impl SKTileMapNode {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"SKTileMapNode").unwrap(), alloc) })
    }
}
impl PNSCopying for SKTileMapNode {}
impl PNSSecureCoding for SKTileMapNode {}
impl ISKNode for SKTileMapNode {}
impl std::convert::TryFrom<SKNode> for SKTileMapNode {
    type Error = &'static str;
    fn try_from(parent: SKNode) -> Result<SKTileMapNode, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"SKTileMapNode").unwrap()) };
        if is_kind_of {
            Ok(SKTileMapNode(parent.0))
        } else {
            Err("This SKNode cannot be downcasted to SKTileMapNode")
        }
    }
}
impl INSResponder for SKTileMapNode {}
impl PNSCoding for SKTileMapNode {}
impl INSObject for SKTileMapNode {}
impl PNSObject for SKTileMapNode {}
impl ISKTileMapNode for SKTileMapNode {}
pub trait ISKTileMapNode: Sized + std::ops::Deref {
    unsafe fn initWithTileSet_columns_rows_tileSize_(
        &self,
        tileSet: SKTileSet,
        columns: NSUInteger,
        rows: NSUInteger,
        tileSize: CGSize,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithTileSet : tileSet, columns : columns, rows : rows, tileSize : tileSize)
    }
    unsafe fn initWithTileSet_columns_rows_tileSize_fillWithTileGroup_(
        &self,
        tileSet: SKTileSet,
        columns: NSUInteger,
        rows: NSUInteger,
        tileSize: CGSize,
        tileGroup: SKTileGroup,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithTileSet : tileSet, columns : columns, rows : rows, tileSize : tileSize, fillWithTileGroup : tileGroup)
    }
    unsafe fn initWithTileSet_columns_rows_tileSize_tileGroupLayout_(
        &self,
        tileSet: SKTileSet,
        columns: NSUInteger,
        rows: NSUInteger,
        tileSize: CGSize,
        tileGroupLayout: NSArray,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithTileSet : tileSet, columns : columns, rows : rows, tileSize : tileSize, tileGroupLayout : tileGroupLayout)
    }
    unsafe fn valueForAttributeNamed_(&self, key: NSString) -> SKAttributeValue
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, valueForAttributeNamed : key)
    }
    unsafe fn setValue_forAttributeNamed_(&self, value: SKAttributeValue, key: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setValue : value, forAttributeNamed : key)
    }
    unsafe fn fillWithTileGroup_(&self, tileGroup: SKTileGroup)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, fillWithTileGroup : tileGroup)
    }
    unsafe fn tileDefinitionAtColumn_row_(
        &self,
        column: NSUInteger,
        row: NSUInteger,
    ) -> SKTileDefinition
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, tileDefinitionAtColumn : column, row : row)
    }
    unsafe fn tileGroupAtColumn_row_(&self, column: NSUInteger, row: NSUInteger) -> SKTileGroup
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, tileGroupAtColumn : column, row : row)
    }
    unsafe fn setTileGroup_forColumn_row_(
        &self,
        tileGroup: SKTileGroup,
        column: NSUInteger,
        row: NSUInteger,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setTileGroup : tileGroup, forColumn : column, row : row)
    }
    unsafe fn setTileGroup_andTileDefinition_forColumn_row_(
        &self,
        tileGroup: SKTileGroup,
        tileDefinition: SKTileDefinition,
        column: NSUInteger,
        row: NSUInteger,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setTileGroup : tileGroup, andTileDefinition : tileDefinition, forColumn : column, row : row)
    }
    unsafe fn tileColumnIndexFromPosition_(&self, position: CGPoint) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, tileColumnIndexFromPosition : position)
    }
    unsafe fn tileRowIndexFromPosition_(&self, position: CGPoint) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, tileRowIndexFromPosition : position)
    }
    unsafe fn centerOfTileAtColumn_row_(&self, column: NSUInteger, row: NSUInteger) -> CGPoint
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, centerOfTileAtColumn : column, row : row)
    }
    unsafe fn numberOfColumns(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, numberOfColumns)
    }
    unsafe fn setNumberOfColumns_(&self, numberOfColumns: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setNumberOfColumns : numberOfColumns)
    }
    unsafe fn numberOfRows(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, numberOfRows)
    }
    unsafe fn setNumberOfRows_(&self, numberOfRows: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setNumberOfRows : numberOfRows)
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
    unsafe fn mapSize(&self) -> CGSize
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, mapSize)
    }
    unsafe fn tileSet(&self) -> SKTileSet
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, tileSet)
    }
    unsafe fn setTileSet_(&self, tileSet: SKTileSet)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setTileSet : tileSet)
    }
    unsafe fn colorBlendFactor(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, colorBlendFactor)
    }
    unsafe fn setColorBlendFactor_(&self, colorBlendFactor: CGFloat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setColorBlendFactor : colorBlendFactor)
    }
    unsafe fn color(&self) -> NSColor
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, color)
    }
    unsafe fn setColor_(&self, color: NSColor)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setColor : color)
    }
    unsafe fn blendMode(&self) -> SKBlendMode
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, blendMode)
    }
    unsafe fn setBlendMode_(&self, blendMode: SKBlendMode)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setBlendMode : blendMode)
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
    unsafe fn shader(&self) -> SKShader
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, shader)
    }
    unsafe fn setShader_(&self, shader: SKShader)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setShader : shader)
    }
    unsafe fn attributeValues(&self) -> NSDictionary
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, attributeValues)
    }
    unsafe fn setAttributeValues_(&self, attributeValues: NSDictionary)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAttributeValues : attributeValues)
    }
    unsafe fn lightingBitMask(&self) -> u32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, lightingBitMask)
    }
    unsafe fn setLightingBitMask_(&self, lightingBitMask: u32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setLightingBitMask : lightingBitMask)
    }
    unsafe fn enableAutomapping(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, enableAutomapping)
    }
    unsafe fn setEnableAutomapping_(&self, enableAutomapping: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setEnableAutomapping : enableAutomapping)
    }
    unsafe fn tileMapNodeWithTileSet_columns_rows_tileSize_(
        tileSet: SKTileSet,
        columns: NSUInteger,
        rows: NSUInteger,
        tileSize: CGSize,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SKTileMapNode").unwrap(), tileMapNodeWithTileSet : tileSet, columns : columns, rows : rows, tileSize : tileSize)
    }
    unsafe fn tileMapNodeWithTileSet_columns_rows_tileSize_fillWithTileGroup_(
        tileSet: SKTileSet,
        columns: NSUInteger,
        rows: NSUInteger,
        tileSize: CGSize,
        tileGroup: SKTileGroup,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SKTileMapNode").unwrap(), tileMapNodeWithTileSet : tileSet, columns : columns, rows : rows, tileSize : tileSize, fillWithTileGroup : tileGroup)
    }
    unsafe fn tileMapNodeWithTileSet_columns_rows_tileSize_tileGroupLayout_(
        tileSet: SKTileSet,
        columns: NSUInteger,
        rows: NSUInteger,
        tileSize: CGSize,
        tileGroupLayout: NSArray,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SKTileMapNode").unwrap(), tileMapNodeWithTileSet : tileSet, columns : columns, rows : rows, tileSize : tileSize, tileGroupLayout : tileGroupLayout)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct SKMutableTexture(pub id);
impl std::ops::Deref for SKMutableTexture {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for SKMutableTexture {}
impl SKMutableTexture {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"SKMutableTexture").unwrap(), alloc) })
    }
}
impl ISKTexture for SKMutableTexture {}
impl PNSCopying for SKMutableTexture {}
impl PNSSecureCoding for SKMutableTexture {}
impl From<SKMutableTexture> for SKTexture {
    fn from(child: SKMutableTexture) -> SKTexture {
        SKTexture(child.0)
    }
}
impl std::convert::TryFrom<SKTexture> for SKMutableTexture {
    type Error = &'static str;
    fn try_from(parent: SKTexture) -> Result<SKMutableTexture, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"SKMutableTexture").unwrap()) };
        if is_kind_of {
            Ok(SKMutableTexture(parent.0))
        } else {
            Err("This SKTexture cannot be downcasted to SKMutableTexture")
        }
    }
}
impl INSObject for SKMutableTexture {}
impl PNSObject for SKMutableTexture {}
impl ISKMutableTexture for SKMutableTexture {}
pub trait ISKMutableTexture: Sized + std::ops::Deref {
    unsafe fn initWithSize_(&self, size: CGSize) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithSize : size)
    }
    unsafe fn initWithSize_pixelFormat_(
        &self,
        size: CGSize,
        format: ::std::os::raw::c_int,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithSize : size, pixelFormat : format)
    }
    unsafe fn modifyPixelDataWithBlock_(&self, block: *mut ::std::os::raw::c_void)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, modifyPixelDataWithBlock : block)
    }
    unsafe fn mutableTextureWithSize_(size: CGSize) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SKMutableTexture").unwrap(), mutableTextureWithSize : size)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct SKTextureAtlas(pub id);
impl std::ops::Deref for SKTextureAtlas {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for SKTextureAtlas {}
impl SKTextureAtlas {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"SKTextureAtlas").unwrap(), alloc) })
    }
}
impl PNSSecureCoding for SKTextureAtlas {}
impl INSObject for SKTextureAtlas {}
impl PNSObject for SKTextureAtlas {}
impl std::convert::TryFrom<NSObject> for SKTextureAtlas {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<SKTextureAtlas, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"SKTextureAtlas").unwrap()) };
        if is_kind_of {
            Ok(SKTextureAtlas(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to SKTextureAtlas")
        }
    }
}
impl ISKTextureAtlas for SKTextureAtlas {}
pub trait ISKTextureAtlas: Sized + std::ops::Deref {
    unsafe fn textureNamed_(&self, name: NSString) -> SKTexture
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, textureNamed : name)
    }
    unsafe fn preloadWithCompletionHandler_(&self, completionHandler: *mut ::std::os::raw::c_void)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, preloadWithCompletionHandler : completionHandler)
    }
    unsafe fn textureNames(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, textureNames)
    }
    unsafe fn atlasNamed_(name: NSString) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SKTextureAtlas").unwrap(), atlasNamed : name)
    }
    unsafe fn atlasWithDictionary_(properties: NSDictionary) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SKTextureAtlas").unwrap(), atlasWithDictionary : properties)
    }
    unsafe fn preloadTextureAtlases_withCompletionHandler_(
        textureAtlases: NSArray,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SKTextureAtlas").unwrap(), preloadTextureAtlases : textureAtlases, withCompletionHandler : completionHandler)
    }
    unsafe fn preloadTextureAtlasesNamed_withCompletionHandler_(
        atlasNames: NSArray,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SKTextureAtlas").unwrap(), preloadTextureAtlasesNamed : atlasNames, withCompletionHandler : completionHandler)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct SKRange(pub id);
impl std::ops::Deref for SKRange {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for SKRange {}
impl SKRange {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"SKRange").unwrap(), alloc) })
    }
}
impl PNSSecureCoding for SKRange {}
impl PNSCopying for SKRange {}
impl INSObject for SKRange {}
impl PNSObject for SKRange {}
impl std::convert::TryFrom<NSObject> for SKRange {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<SKRange, Self::Error> {
        let is_kind_of: bool = unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"SKRange").unwrap()) };
        if is_kind_of {
            Ok(SKRange(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to SKRange")
        }
    }
}
impl ISKRange for SKRange {}
pub trait ISKRange: Sized + std::ops::Deref {
    unsafe fn initWithLowerLimit_upperLimit_(&self, lower: CGFloat, upper: CGFloat) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithLowerLimit : lower, upperLimit : upper)
    }
    unsafe fn lowerLimit(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, lowerLimit)
    }
    unsafe fn setLowerLimit_(&self, lowerLimit: CGFloat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setLowerLimit : lowerLimit)
    }
    unsafe fn upperLimit(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, upperLimit)
    }
    unsafe fn setUpperLimit_(&self, upperLimit: CGFloat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setUpperLimit : upperLimit)
    }
    unsafe fn rangeWithLowerLimit_upperLimit_(lower: CGFloat, upper: CGFloat) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SKRange").unwrap(), rangeWithLowerLimit : lower, upperLimit : upper)
    }
    unsafe fn rangeWithLowerLimit_(lower: CGFloat) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SKRange").unwrap(), rangeWithLowerLimit : lower)
    }
    unsafe fn rangeWithUpperLimit_(upper: CGFloat) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SKRange").unwrap(), rangeWithUpperLimit : upper)
    }
    unsafe fn rangeWithConstantValue_(value: CGFloat) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SKRange").unwrap(), rangeWithConstantValue : value)
    }
    unsafe fn rangeWithValue_variance_(value: CGFloat, variance: CGFloat) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SKRange").unwrap(), rangeWithValue : value, variance : variance)
    }
    unsafe fn rangeWithNoLimits() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SKRange").unwrap(), rangeWithNoLimits)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct SKConstraint(pub id);
impl std::ops::Deref for SKConstraint {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for SKConstraint {}
impl SKConstraint {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"SKConstraint").unwrap(), alloc) })
    }
}
impl PNSSecureCoding for SKConstraint {}
impl PNSCopying for SKConstraint {}
impl INSObject for SKConstraint {}
impl PNSObject for SKConstraint {}
impl std::convert::TryFrom<NSObject> for SKConstraint {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<SKConstraint, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"SKConstraint").unwrap()) };
        if is_kind_of {
            Ok(SKConstraint(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to SKConstraint")
        }
    }
}
impl ISKConstraint for SKConstraint {}
pub trait ISKConstraint: Sized + std::ops::Deref {
    unsafe fn enabled(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, enabled)
    }
    unsafe fn setEnabled_(&self, enabled: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setEnabled : enabled)
    }
    unsafe fn referenceNode(&self) -> SKNode
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, referenceNode)
    }
    unsafe fn setReferenceNode_(&self, referenceNode: SKNode)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setReferenceNode : referenceNode)
    }
    unsafe fn positionX_(range: SKRange) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SKConstraint").unwrap(), positionX : range)
    }
    unsafe fn positionY_(range: SKRange) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SKConstraint").unwrap(), positionY : range)
    }
    unsafe fn positionX_Y_(xRange: SKRange, yRange: SKRange) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SKConstraint").unwrap(), positionX : xRange, Y : yRange)
    }
    unsafe fn distance_toNode_(range: SKRange, node: SKNode) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SKConstraint").unwrap(), distance : range, toNode : node)
    }
    unsafe fn distance_toPoint_(range: SKRange, point: CGPoint) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SKConstraint").unwrap(), distance : range, toPoint : point)
    }
    unsafe fn distance_toPoint_inNode_(range: SKRange, point: CGPoint, node: SKNode) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SKConstraint").unwrap(), distance : range, toPoint : point, inNode : node)
    }
    unsafe fn zRotation_(zRange: SKRange) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SKConstraint").unwrap(), zRotation : zRange)
    }
    unsafe fn orientToNode_offset_(node: SKNode, radians: SKRange) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SKConstraint").unwrap(), orientToNode : node, offset : radians)
    }
    unsafe fn orientToPoint_offset_(point: CGPoint, radians: SKRange) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SKConstraint").unwrap(), orientToPoint : point, offset : radians)
    }
    unsafe fn orientToPoint_inNode_offset_(
        point: CGPoint,
        node: SKNode,
        radians: SKRange,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SKConstraint").unwrap(), orientToPoint : point, inNode : node, offset : radians)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct SKReachConstraints(pub id);
impl std::ops::Deref for SKReachConstraints {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for SKReachConstraints {}
impl SKReachConstraints {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"SKReachConstraints").unwrap(), alloc) })
    }
}
impl PNSSecureCoding for SKReachConstraints {}
impl INSObject for SKReachConstraints {}
impl PNSObject for SKReachConstraints {}
impl std::convert::TryFrom<NSObject> for SKReachConstraints {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<SKReachConstraints, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"SKReachConstraints").unwrap()) };
        if is_kind_of {
            Ok(SKReachConstraints(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to SKReachConstraints")
        }
    }
}
impl ISKReachConstraints for SKReachConstraints {}
pub trait ISKReachConstraints: Sized + std::ops::Deref {
    unsafe fn initWithLowerAngleLimit_upperAngleLimit_(
        &self,
        lowerAngleLimit: CGFloat,
        upperAngleLimit: CGFloat,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithLowerAngleLimit : lowerAngleLimit, upperAngleLimit : upperAngleLimit)
    }
    unsafe fn lowerAngleLimit(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, lowerAngleLimit)
    }
    unsafe fn setLowerAngleLimit_(&self, lowerAngleLimit: CGFloat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setLowerAngleLimit : lowerAngleLimit)
    }
    unsafe fn upperAngleLimit(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, upperAngleLimit)
    }
    unsafe fn setUpperAngleLimit_(&self, upperAngleLimit: CGFloat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setUpperAngleLimit : upperAngleLimit)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct SKPhysicsBody(pub id);
impl std::ops::Deref for SKPhysicsBody {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for SKPhysicsBody {}
impl SKPhysicsBody {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"SKPhysicsBody").unwrap(), alloc) })
    }
}
impl PNSCopying for SKPhysicsBody {}
impl PNSSecureCoding for SKPhysicsBody {}
impl INSObject for SKPhysicsBody {}
impl PNSObject for SKPhysicsBody {}
impl std::convert::TryFrom<NSObject> for SKPhysicsBody {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<SKPhysicsBody, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"SKPhysicsBody").unwrap()) };
        if is_kind_of {
            Ok(SKPhysicsBody(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to SKPhysicsBody")
        }
    }
}
impl ISKPhysicsBody for SKPhysicsBody {}
pub trait ISKPhysicsBody: Sized + std::ops::Deref {
    unsafe fn applyForce_(&self, force: CGVector)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, applyForce : force)
    }
    unsafe fn applyForce_atPoint_(&self, force: CGVector, point: CGPoint)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, applyForce : force, atPoint : point)
    }
    unsafe fn applyTorque_(&self, torque: CGFloat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, applyTorque : torque)
    }
    unsafe fn applyImpulse_(&self, impulse: CGVector)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, applyImpulse : impulse)
    }
    unsafe fn applyImpulse_atPoint_(&self, impulse: CGVector, point: CGPoint)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, applyImpulse : impulse, atPoint : point)
    }
    unsafe fn applyAngularImpulse_(&self, impulse: CGFloat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, applyAngularImpulse : impulse)
    }
    unsafe fn allContactedBodies(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, allContactedBodies)
    }
    unsafe fn isDynamic(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isDynamic)
    }
    unsafe fn setDynamic_(&self, dynamic: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDynamic : dynamic)
    }
    unsafe fn usesPreciseCollisionDetection(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, usesPreciseCollisionDetection)
    }
    unsafe fn setUsesPreciseCollisionDetection_(&self, usesPreciseCollisionDetection: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setUsesPreciseCollisionDetection : usesPreciseCollisionDetection)
    }
    unsafe fn allowsRotation(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, allowsRotation)
    }
    unsafe fn setAllowsRotation_(&self, allowsRotation: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAllowsRotation : allowsRotation)
    }
    unsafe fn pinned(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, pinned)
    }
    unsafe fn setPinned_(&self, pinned: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPinned : pinned)
    }
    unsafe fn isResting(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isResting)
    }
    unsafe fn setResting_(&self, resting: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setResting : resting)
    }
    unsafe fn friction(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, friction)
    }
    unsafe fn setFriction_(&self, friction: CGFloat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setFriction : friction)
    }
    unsafe fn charge(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, charge)
    }
    unsafe fn setCharge_(&self, charge: CGFloat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCharge : charge)
    }
    unsafe fn restitution(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, restitution)
    }
    unsafe fn setRestitution_(&self, restitution: CGFloat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRestitution : restitution)
    }
    unsafe fn linearDamping(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, linearDamping)
    }
    unsafe fn setLinearDamping_(&self, linearDamping: CGFloat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setLinearDamping : linearDamping)
    }
    unsafe fn angularDamping(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, angularDamping)
    }
    unsafe fn setAngularDamping_(&self, angularDamping: CGFloat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAngularDamping : angularDamping)
    }
    unsafe fn density(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, density)
    }
    unsafe fn setDensity_(&self, density: CGFloat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDensity : density)
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
    unsafe fn area(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, area)
    }
    unsafe fn affectedByGravity(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, affectedByGravity)
    }
    unsafe fn setAffectedByGravity_(&self, affectedByGravity: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAffectedByGravity : affectedByGravity)
    }
    unsafe fn fieldBitMask(&self) -> u32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, fieldBitMask)
    }
    unsafe fn setFieldBitMask_(&self, fieldBitMask: u32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setFieldBitMask : fieldBitMask)
    }
    unsafe fn categoryBitMask(&self) -> u32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, categoryBitMask)
    }
    unsafe fn setCategoryBitMask_(&self, categoryBitMask: u32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCategoryBitMask : categoryBitMask)
    }
    unsafe fn collisionBitMask(&self) -> u32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, collisionBitMask)
    }
    unsafe fn setCollisionBitMask_(&self, collisionBitMask: u32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCollisionBitMask : collisionBitMask)
    }
    unsafe fn contactTestBitMask(&self) -> u32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, contactTestBitMask)
    }
    unsafe fn setContactTestBitMask_(&self, contactTestBitMask: u32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setContactTestBitMask : contactTestBitMask)
    }
    unsafe fn joints(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, joints)
    }
    unsafe fn node(&self) -> SKNode
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, node)
    }
    unsafe fn velocity(&self) -> CGVector
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, velocity)
    }
    unsafe fn setVelocity_(&self, velocity: CGVector)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setVelocity : velocity)
    }
    unsafe fn angularVelocity(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, angularVelocity)
    }
    unsafe fn setAngularVelocity_(&self, angularVelocity: CGFloat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAngularVelocity : angularVelocity)
    }
    unsafe fn bodyWithCircleOfRadius_(r: CGFloat) -> SKPhysicsBody
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SKPhysicsBody").unwrap(), bodyWithCircleOfRadius : r)
    }
    unsafe fn bodyWithCircleOfRadius_center_(r: CGFloat, center: CGPoint) -> SKPhysicsBody
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SKPhysicsBody").unwrap(), bodyWithCircleOfRadius : r, center : center)
    }
    unsafe fn bodyWithRectangleOfSize_(s: CGSize) -> SKPhysicsBody
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SKPhysicsBody").unwrap(), bodyWithRectangleOfSize : s)
    }
    unsafe fn bodyWithRectangleOfSize_center_(s: CGSize, center: CGPoint) -> SKPhysicsBody
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SKPhysicsBody").unwrap(), bodyWithRectangleOfSize : s, center : center)
    }
    unsafe fn bodyWithPolygonFromPath_(path: CGPathRef) -> SKPhysicsBody
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SKPhysicsBody").unwrap(), bodyWithPolygonFromPath : path)
    }
    unsafe fn bodyWithEdgeFromPoint_toPoint_(p1: CGPoint, p2: CGPoint) -> SKPhysicsBody
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SKPhysicsBody").unwrap(), bodyWithEdgeFromPoint : p1, toPoint : p2)
    }
    unsafe fn bodyWithEdgeChainFromPath_(path: CGPathRef) -> SKPhysicsBody
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SKPhysicsBody").unwrap(), bodyWithEdgeChainFromPath : path)
    }
    unsafe fn bodyWithEdgeLoopFromPath_(path: CGPathRef) -> SKPhysicsBody
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SKPhysicsBody").unwrap(), bodyWithEdgeLoopFromPath : path)
    }
    unsafe fn bodyWithEdgeLoopFromRect_(rect: CGRect) -> SKPhysicsBody
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SKPhysicsBody").unwrap(), bodyWithEdgeLoopFromRect : rect)
    }
    unsafe fn bodyWithTexture_size_(texture: SKTexture, size: CGSize) -> SKPhysicsBody
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SKPhysicsBody").unwrap(), bodyWithTexture : texture, size : size)
    }
    unsafe fn bodyWithTexture_alphaThreshold_size_(
        texture: SKTexture,
        alphaThreshold: f32,
        size: CGSize,
    ) -> SKPhysicsBody
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SKPhysicsBody").unwrap(), bodyWithTexture : texture, alphaThreshold : alphaThreshold, size : size)
    }
    unsafe fn bodyWithBodies_(bodies: NSArray) -> SKPhysicsBody
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SKPhysicsBody").unwrap(), bodyWithBodies : bodies)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct SKPhysicsJoint(pub id);
impl std::ops::Deref for SKPhysicsJoint {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for SKPhysicsJoint {}
impl SKPhysicsJoint {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"SKPhysicsJoint").unwrap(), alloc) })
    }
}
impl PNSSecureCoding for SKPhysicsJoint {}
impl INSObject for SKPhysicsJoint {}
impl PNSObject for SKPhysicsJoint {}
impl std::convert::TryFrom<NSObject> for SKPhysicsJoint {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<SKPhysicsJoint, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"SKPhysicsJoint").unwrap()) };
        if is_kind_of {
            Ok(SKPhysicsJoint(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to SKPhysicsJoint")
        }
    }
}
impl ISKPhysicsJoint for SKPhysicsJoint {}
pub trait ISKPhysicsJoint: Sized + std::ops::Deref {
    unsafe fn bodyA(&self) -> SKPhysicsBody
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, bodyA)
    }
    unsafe fn setBodyA_(&self, bodyA: SKPhysicsBody)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setBodyA : bodyA)
    }
    unsafe fn bodyB(&self) -> SKPhysicsBody
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, bodyB)
    }
    unsafe fn setBodyB_(&self, bodyB: SKPhysicsBody)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setBodyB : bodyB)
    }
    unsafe fn reactionForce(&self) -> CGVector
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, reactionForce)
    }
    unsafe fn reactionTorque(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, reactionTorque)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct SKPhysicsJointPin(pub id);
impl std::ops::Deref for SKPhysicsJointPin {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for SKPhysicsJointPin {}
impl SKPhysicsJointPin {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"SKPhysicsJointPin").unwrap(), alloc) })
    }
}
impl ISKPhysicsJoint for SKPhysicsJointPin {}
impl PNSSecureCoding for SKPhysicsJointPin {}
impl From<SKPhysicsJointPin> for SKPhysicsJoint {
    fn from(child: SKPhysicsJointPin) -> SKPhysicsJoint {
        SKPhysicsJoint(child.0)
    }
}
impl std::convert::TryFrom<SKPhysicsJoint> for SKPhysicsJointPin {
    type Error = &'static str;
    fn try_from(parent: SKPhysicsJoint) -> Result<SKPhysicsJointPin, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"SKPhysicsJointPin").unwrap()) };
        if is_kind_of {
            Ok(SKPhysicsJointPin(parent.0))
        } else {
            Err("This SKPhysicsJoint cannot be downcasted to SKPhysicsJointPin")
        }
    }
}
impl INSObject for SKPhysicsJointPin {}
impl PNSObject for SKPhysicsJointPin {}
impl ISKPhysicsJointPin for SKPhysicsJointPin {}
pub trait ISKPhysicsJointPin: Sized + std::ops::Deref {
    unsafe fn shouldEnableLimits(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, shouldEnableLimits)
    }
    unsafe fn setShouldEnableLimits_(&self, shouldEnableLimits: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setShouldEnableLimits : shouldEnableLimits)
    }
    unsafe fn lowerAngleLimit(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, lowerAngleLimit)
    }
    unsafe fn setLowerAngleLimit_(&self, lowerAngleLimit: CGFloat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setLowerAngleLimit : lowerAngleLimit)
    }
    unsafe fn upperAngleLimit(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, upperAngleLimit)
    }
    unsafe fn setUpperAngleLimit_(&self, upperAngleLimit: CGFloat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setUpperAngleLimit : upperAngleLimit)
    }
    unsafe fn frictionTorque(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, frictionTorque)
    }
    unsafe fn setFrictionTorque_(&self, frictionTorque: CGFloat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setFrictionTorque : frictionTorque)
    }
    unsafe fn rotationSpeed(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, rotationSpeed)
    }
    unsafe fn setRotationSpeed_(&self, rotationSpeed: CGFloat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRotationSpeed : rotationSpeed)
    }
    unsafe fn jointWithBodyA_bodyB_anchor_(
        bodyA: SKPhysicsBody,
        bodyB: SKPhysicsBody,
        anchor: CGPoint,
    ) -> SKPhysicsJointPin
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SKPhysicsJointPin").unwrap(), jointWithBodyA : bodyA, bodyB : bodyB, anchor : anchor)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct SKPhysicsJointSpring(pub id);
impl std::ops::Deref for SKPhysicsJointSpring {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for SKPhysicsJointSpring {}
impl SKPhysicsJointSpring {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"SKPhysicsJointSpring").unwrap(), alloc) })
    }
}
impl ISKPhysicsJoint for SKPhysicsJointSpring {}
impl PNSSecureCoding for SKPhysicsJointSpring {}
impl std::convert::TryFrom<SKPhysicsJoint> for SKPhysicsJointSpring {
    type Error = &'static str;
    fn try_from(parent: SKPhysicsJoint) -> Result<SKPhysicsJointSpring, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"SKPhysicsJointSpring").unwrap()) };
        if is_kind_of {
            Ok(SKPhysicsJointSpring(parent.0))
        } else {
            Err("This SKPhysicsJoint cannot be downcasted to SKPhysicsJointSpring")
        }
    }
}
impl INSObject for SKPhysicsJointSpring {}
impl PNSObject for SKPhysicsJointSpring {}
impl ISKPhysicsJointSpring for SKPhysicsJointSpring {}
pub trait ISKPhysicsJointSpring: Sized + std::ops::Deref {
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
    unsafe fn frequency(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, frequency)
    }
    unsafe fn setFrequency_(&self, frequency: CGFloat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setFrequency : frequency)
    }
    unsafe fn jointWithBodyA_bodyB_anchorA_anchorB_(
        bodyA: SKPhysicsBody,
        bodyB: SKPhysicsBody,
        anchorA: CGPoint,
        anchorB: CGPoint,
    ) -> SKPhysicsJointSpring
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SKPhysicsJointSpring").unwrap(), jointWithBodyA : bodyA, bodyB : bodyB, anchorA : anchorA, anchorB : anchorB)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct SKPhysicsJointFixed(pub id);
impl std::ops::Deref for SKPhysicsJointFixed {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for SKPhysicsJointFixed {}
impl SKPhysicsJointFixed {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"SKPhysicsJointFixed").unwrap(), alloc) })
    }
}
impl ISKPhysicsJoint for SKPhysicsJointFixed {}
impl PNSSecureCoding for SKPhysicsJointFixed {}
impl std::convert::TryFrom<SKPhysicsJoint> for SKPhysicsJointFixed {
    type Error = &'static str;
    fn try_from(parent: SKPhysicsJoint) -> Result<SKPhysicsJointFixed, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"SKPhysicsJointFixed").unwrap()) };
        if is_kind_of {
            Ok(SKPhysicsJointFixed(parent.0))
        } else {
            Err("This SKPhysicsJoint cannot be downcasted to SKPhysicsJointFixed")
        }
    }
}
impl INSObject for SKPhysicsJointFixed {}
impl PNSObject for SKPhysicsJointFixed {}
impl ISKPhysicsJointFixed for SKPhysicsJointFixed {}
pub trait ISKPhysicsJointFixed: Sized + std::ops::Deref {
    unsafe fn jointWithBodyA_bodyB_anchor_(
        bodyA: SKPhysicsBody,
        bodyB: SKPhysicsBody,
        anchor: CGPoint,
    ) -> SKPhysicsJointFixed
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SKPhysicsJointFixed").unwrap(), jointWithBodyA : bodyA, bodyB : bodyB, anchor : anchor)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct SKPhysicsJointSliding(pub id);
impl std::ops::Deref for SKPhysicsJointSliding {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for SKPhysicsJointSliding {}
impl SKPhysicsJointSliding {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"SKPhysicsJointSliding").unwrap(), alloc) })
    }
}
impl ISKPhysicsJoint for SKPhysicsJointSliding {}
impl PNSSecureCoding for SKPhysicsJointSliding {}
impl std::convert::TryFrom<SKPhysicsJoint> for SKPhysicsJointSliding {
    type Error = &'static str;
    fn try_from(parent: SKPhysicsJoint) -> Result<SKPhysicsJointSliding, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"SKPhysicsJointSliding").unwrap()) };
        if is_kind_of {
            Ok(SKPhysicsJointSliding(parent.0))
        } else {
            Err("This SKPhysicsJoint cannot be downcasted to SKPhysicsJointSliding")
        }
    }
}
impl INSObject for SKPhysicsJointSliding {}
impl PNSObject for SKPhysicsJointSliding {}
impl ISKPhysicsJointSliding for SKPhysicsJointSliding {}
pub trait ISKPhysicsJointSliding: Sized + std::ops::Deref {
    unsafe fn shouldEnableLimits(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, shouldEnableLimits)
    }
    unsafe fn setShouldEnableLimits_(&self, shouldEnableLimits: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setShouldEnableLimits : shouldEnableLimits)
    }
    unsafe fn lowerDistanceLimit(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, lowerDistanceLimit)
    }
    unsafe fn setLowerDistanceLimit_(&self, lowerDistanceLimit: CGFloat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setLowerDistanceLimit : lowerDistanceLimit)
    }
    unsafe fn upperDistanceLimit(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, upperDistanceLimit)
    }
    unsafe fn setUpperDistanceLimit_(&self, upperDistanceLimit: CGFloat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setUpperDistanceLimit : upperDistanceLimit)
    }
    unsafe fn jointWithBodyA_bodyB_anchor_axis_(
        bodyA: SKPhysicsBody,
        bodyB: SKPhysicsBody,
        anchor: CGPoint,
        axis: CGVector,
    ) -> SKPhysicsJointSliding
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SKPhysicsJointSliding").unwrap(), jointWithBodyA : bodyA, bodyB : bodyB, anchor : anchor, axis : axis)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct SKPhysicsJointLimit(pub id);
impl std::ops::Deref for SKPhysicsJointLimit {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for SKPhysicsJointLimit {}
impl SKPhysicsJointLimit {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"SKPhysicsJointLimit").unwrap(), alloc) })
    }
}
impl ISKPhysicsJoint for SKPhysicsJointLimit {}
impl PNSSecureCoding for SKPhysicsJointLimit {}
impl std::convert::TryFrom<SKPhysicsJoint> for SKPhysicsJointLimit {
    type Error = &'static str;
    fn try_from(parent: SKPhysicsJoint) -> Result<SKPhysicsJointLimit, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"SKPhysicsJointLimit").unwrap()) };
        if is_kind_of {
            Ok(SKPhysicsJointLimit(parent.0))
        } else {
            Err("This SKPhysicsJoint cannot be downcasted to SKPhysicsJointLimit")
        }
    }
}
impl INSObject for SKPhysicsJointLimit {}
impl PNSObject for SKPhysicsJointLimit {}
impl ISKPhysicsJointLimit for SKPhysicsJointLimit {}
pub trait ISKPhysicsJointLimit: Sized + std::ops::Deref {
    unsafe fn maxLength(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, maxLength)
    }
    unsafe fn setMaxLength_(&self, maxLength: CGFloat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMaxLength : maxLength)
    }
    unsafe fn jointWithBodyA_bodyB_anchorA_anchorB_(
        bodyA: SKPhysicsBody,
        bodyB: SKPhysicsBody,
        anchorA: CGPoint,
        anchorB: CGPoint,
    ) -> SKPhysicsJointLimit
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SKPhysicsJointLimit").unwrap(), jointWithBodyA : bodyA, bodyB : bodyB, anchorA : anchorA, anchorB : anchorB)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct SKPhysicsContact(pub id);
impl std::ops::Deref for SKPhysicsContact {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for SKPhysicsContact {}
impl SKPhysicsContact {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"SKPhysicsContact").unwrap(), alloc) })
    }
}
impl INSObject for SKPhysicsContact {}
impl PNSObject for SKPhysicsContact {}
impl std::convert::TryFrom<NSObject> for SKPhysicsContact {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<SKPhysicsContact, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"SKPhysicsContact").unwrap()) };
        if is_kind_of {
            Ok(SKPhysicsContact(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to SKPhysicsContact")
        }
    }
}
impl ISKPhysicsContact for SKPhysicsContact {}
pub trait ISKPhysicsContact: Sized + std::ops::Deref {
    unsafe fn bodyA(&self) -> SKPhysicsBody
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, bodyA)
    }
    unsafe fn bodyB(&self) -> SKPhysicsBody
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, bodyB)
    }
    unsafe fn contactPoint(&self) -> CGPoint
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, contactPoint)
    }
    unsafe fn contactNormal(&self) -> CGVector
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, contactNormal)
    }
    unsafe fn collisionImpulse(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, collisionImpulse)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct SKPhysicsWorld(pub id);
impl std::ops::Deref for SKPhysicsWorld {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for SKPhysicsWorld {}
impl SKPhysicsWorld {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"SKPhysicsWorld").unwrap(), alloc) })
    }
}
impl PNSSecureCoding for SKPhysicsWorld {}
impl INSObject for SKPhysicsWorld {}
impl PNSObject for SKPhysicsWorld {}
impl std::convert::TryFrom<NSObject> for SKPhysicsWorld {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<SKPhysicsWorld, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"SKPhysicsWorld").unwrap()) };
        if is_kind_of {
            Ok(SKPhysicsWorld(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to SKPhysicsWorld")
        }
    }
}
impl ISKPhysicsWorld for SKPhysicsWorld {}
pub trait ISKPhysicsWorld: Sized + std::ops::Deref {
    unsafe fn addJoint_(&self, joint: SKPhysicsJoint)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, addJoint : joint)
    }
    unsafe fn removeJoint_(&self, joint: SKPhysicsJoint)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, removeJoint : joint)
    }
    unsafe fn removeAllJoints(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, removeAllJoints)
    }
    unsafe fn sampleFieldsAt_(&self, position: vector_float3) -> vector_float3
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, sampleFieldsAt : position)
    }
    unsafe fn bodyAtPoint_(&self, point: CGPoint) -> SKPhysicsBody
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, bodyAtPoint : point)
    }
    unsafe fn bodyInRect_(&self, rect: CGRect) -> SKPhysicsBody
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, bodyInRect : rect)
    }
    unsafe fn bodyAlongRayStart_end_(&self, start: CGPoint, end: CGPoint) -> SKPhysicsBody
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, bodyAlongRayStart : start, end : end)
    }
    unsafe fn enumerateBodiesAtPoint_usingBlock_(
        &self,
        point: CGPoint,
        block: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, enumerateBodiesAtPoint : point, usingBlock : block)
    }
    unsafe fn enumerateBodiesInRect_usingBlock_(
        &self,
        rect: CGRect,
        block: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, enumerateBodiesInRect : rect, usingBlock : block)
    }
    unsafe fn enumerateBodiesAlongRayStart_end_usingBlock_(
        &self,
        start: CGPoint,
        end: CGPoint,
        block: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, enumerateBodiesAlongRayStart : start, end : end, usingBlock : block)
    }
    unsafe fn gravity(&self) -> CGVector
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, gravity)
    }
    unsafe fn setGravity_(&self, gravity: CGVector)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setGravity : gravity)
    }
    unsafe fn speed(&self) -> CGFloat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, speed)
    }
    unsafe fn setSpeed_(&self, speed: CGFloat)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSpeed : speed)
    }
    unsafe fn contactDelegate(&self) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, contactDelegate)
    }
    unsafe fn setContactDelegate_(&self, contactDelegate: *mut u64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setContactDelegate : contactDelegate)
    }
}
impl SKNode_NSAccessibility for SKNode {}
pub trait SKNode_NSAccessibility: Sized + std::ops::Deref {
    unsafe fn accessibilityHitTest_(&self, point: CGPoint) -> id
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, accessibilityHitTest : point)
    }
    unsafe fn isAccessibilityElement(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isAccessibilityElement)
    }
    unsafe fn setAccessibilityElement_(&self, accessibilityElement: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAccessibilityElement : accessibilityElement)
    }
    unsafe fn accessibilityRole(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, accessibilityRole)
    }
    unsafe fn setAccessibilityRole_(&self, accessibilityRole: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAccessibilityRole : accessibilityRole)
    }
    unsafe fn accessibilityRoleDescription(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, accessibilityRoleDescription)
    }
    unsafe fn setAccessibilityRoleDescription_(&self, accessibilityRoleDescription: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAccessibilityRoleDescription : accessibilityRoleDescription)
    }
    unsafe fn accessibilitySubrole(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, accessibilitySubrole)
    }
    unsafe fn setAccessibilitySubrole_(&self, accessibilitySubrole: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAccessibilitySubrole : accessibilitySubrole)
    }
    unsafe fn accessibilityFrame(&self) -> CGRect
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, accessibilityFrame)
    }
    unsafe fn setAccessibilityFrame_(&self, accessibilityFrame: CGRect)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAccessibilityFrame : accessibilityFrame)
    }
    unsafe fn accessibilityParent(&self) -> id
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, accessibilityParent)
    }
    unsafe fn setAccessibilityParent_(&self, accessibilityParent: id)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAccessibilityParent : accessibilityParent)
    }
    unsafe fn accessibilityChildren(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, accessibilityChildren)
    }
    unsafe fn setAccessibilityChildren_(&self, accessibilityChildren: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAccessibilityChildren : accessibilityChildren)
    }
    unsafe fn accessibilityHelp(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, accessibilityHelp)
    }
    unsafe fn setAccessibilityHelp_(&self, accessibilityHelp: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAccessibilityHelp : accessibilityHelp)
    }
    unsafe fn accessibilityLabel(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, accessibilityLabel)
    }
    unsafe fn setAccessibilityLabel_(&self, accessibilityLabel: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAccessibilityLabel : accessibilityLabel)
    }
    unsafe fn isAccessibilityEnabled(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isAccessibilityEnabled)
    }
    unsafe fn setAccessibilityEnabled_(&self, accessibilityEnabled: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAccessibilityEnabled : accessibilityEnabled)
    }
}

unsafe impl objc2::encode::RefEncode for simd_float2x2 {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for simd_float2x2 {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("simd_float2x2", &[]);
}
unsafe impl objc2::encode::RefEncode for SKNode {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for SKNode {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for SKCameraNode {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for SKCameraNode {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for SKShader {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for SKShader {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for SKAction {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for SKAction {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for SKWarpGeometry {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for SKWarpGeometry {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for SKWarpGeometryGrid {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for SKWarpGeometryGrid {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for SKEffectNode {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for SKEffectNode {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for SKScene {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for SKScene {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for SKSpriteNode {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for SKSpriteNode {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for SKKeyframeSequence {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for SKKeyframeSequence {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for SKEmitterNode {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for SKEmitterNode {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for SKShapeNode {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for SKShapeNode {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for SKFieldNode {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for SKFieldNode {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for SKLabelNode {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for SKLabelNode {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for SKVideoNode {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for SKVideoNode {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for SKAudioNode {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for SKAudioNode {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for SKCropNode {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for SKCropNode {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for SKLightNode {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for SKLightNode {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for SKReferenceNode {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for SKReferenceNode {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for SK3DNode {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for SK3DNode {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for SKTransformNode {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for SKTransformNode {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for SKRegion {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for SKRegion {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for SKView {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for SKView {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for SKTransition {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for SKTransition {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for SKTexture {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for SKTexture {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for SKUniform {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for SKUniform {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for SKAttribute {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for SKAttribute {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for SKAttributeValue {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for SKAttributeValue {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for SKRenderer {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for SKRenderer {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for SKTileDefinition {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for SKTileDefinition {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for SKTileSet {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for SKTileSet {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for SKTileGroup {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for SKTileGroup {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for SKTileGroupRule {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for SKTileGroupRule {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for SKTileMapNode {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for SKTileMapNode {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for SKMutableTexture {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for SKMutableTexture {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for SKTextureAtlas {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for SKTextureAtlas {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for SKRange {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for SKRange {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for SKConstraint {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for SKConstraint {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for SKReachConstraints {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for SKReachConstraints {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for SKPhysicsBody {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for SKPhysicsBody {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for SKPhysicsJoint {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for SKPhysicsJoint {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for SKPhysicsJointPin {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for SKPhysicsJointPin {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for SKPhysicsJointSpring {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for SKPhysicsJointSpring {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for SKPhysicsJointFixed {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for SKPhysicsJointFixed {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for SKPhysicsJointSliding {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for SKPhysicsJointSliding {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for SKPhysicsJointLimit {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for SKPhysicsJointLimit {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for SKPhysicsContact {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for SKPhysicsContact {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for SKPhysicsWorld {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for SKPhysicsWorld {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
