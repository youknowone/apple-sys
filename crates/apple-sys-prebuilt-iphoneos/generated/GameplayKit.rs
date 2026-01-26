#[allow(unused_imports)]
use crate::objc::*;
#[allow(unused_imports)]
use crate::AVFoundation::*;
#[allow(unused_imports)]
use crate::CoreGraphics::*;
#[allow(unused_imports)]
use crate::Foundation::*;
#[allow(unused_imports)]
use crate::ModelIO::*;
#[allow(unused_imports)]
use crate::SceneKit::*;
#[allow(unused_imports)]
use crate::Security::*;
#[allow(unused_imports)]
use crate::SpriteKit::*;

#[allow(unused_imports)]
use objc2::msg_send;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct GKComponent(pub id);
impl std::ops::Deref for GKComponent {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for GKComponent {}
impl GKComponent {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"GKComponent").unwrap(), alloc) })
    }
}
impl PNSCopying for GKComponent {}
impl PNSSecureCoding for GKComponent {}
impl INSObject for GKComponent {}
impl PNSObject for GKComponent {}
impl std::convert::TryFrom<NSObject> for GKComponent {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<GKComponent, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"GKComponent").unwrap()) };
        if is_kind_of {
            Ok(GKComponent(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to GKComponent")
        }
    }
}
impl IGKComponent for GKComponent {}
pub trait IGKComponent: Sized + std::ops::Deref {
    unsafe fn updateWithDeltaTime_(&self, seconds: NSTimeInterval)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, updateWithDeltaTime : seconds)
    }
    unsafe fn didAddToEntity(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, didAddToEntity)
    }
    unsafe fn willRemoveFromEntity(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, willRemoveFromEntity)
    }
    unsafe fn entity(&self) -> GKEntity
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, entity)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct GKComponentSystem(pub id);
impl std::ops::Deref for GKComponentSystem {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for GKComponentSystem {}
impl GKComponentSystem {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"GKComponentSystem").unwrap(), alloc) })
    }
}
impl PNSFastEnumeration for GKComponentSystem {}
impl INSObject for GKComponentSystem {}
impl PNSObject for GKComponentSystem {}
impl std::convert::TryFrom<NSObject> for GKComponentSystem {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<GKComponentSystem, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"GKComponentSystem").unwrap()) };
        if is_kind_of {
            Ok(GKComponentSystem(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to GKComponentSystem")
        }
    }
}
impl<ComponentType: 'static> IGKComponentSystem<ComponentType> for GKComponentSystem {}
pub trait IGKComponentSystem<ComponentType: 'static>: Sized + std::ops::Deref {
    unsafe fn objectAtIndexedSubscript_(&self, idx: NSUInteger) -> id
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, objectAtIndexedSubscript : idx)
    }
    unsafe fn initWithComponentClass_(&self, cls: Class) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithComponentClass : cls)
    }
    unsafe fn addComponent_(&self, component: GKComponent)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, addComponent : component)
    }
    unsafe fn addComponentWithEntity_(&self, entity: GKEntity)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, addComponentWithEntity : entity)
    }
    unsafe fn removeComponentWithEntity_(&self, entity: GKEntity)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, removeComponentWithEntity : entity)
    }
    unsafe fn removeComponent_(&self, component: GKComponent)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, removeComponent : component)
    }
    unsafe fn updateWithDeltaTime_(&self, seconds: NSTimeInterval)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, updateWithDeltaTime : seconds)
    }
    unsafe fn classForGenericArgumentAtIndex_(&self, index: NSUInteger) -> Class
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, classForGenericArgumentAtIndex : index)
    }
    unsafe fn componentClass(&self) -> Class
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, componentClass)
    }
    unsafe fn components(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, components)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct GKObstacle(pub id);
impl std::ops::Deref for GKObstacle {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for GKObstacle {}
impl GKObstacle {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"GKObstacle").unwrap(), alloc) })
    }
}
impl INSObject for GKObstacle {}
impl PNSObject for GKObstacle {}
impl std::convert::TryFrom<NSObject> for GKObstacle {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<GKObstacle, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"GKObstacle").unwrap()) };
        if is_kind_of {
            Ok(GKObstacle(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to GKObstacle")
        }
    }
}
impl IGKObstacle for GKObstacle {}
pub trait IGKObstacle: Sized + std::ops::Deref {}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct GKCircleObstacle(pub id);
impl std::ops::Deref for GKCircleObstacle {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for GKCircleObstacle {}
impl GKCircleObstacle {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"GKCircleObstacle").unwrap(), alloc) })
    }
}
impl IGKObstacle for GKCircleObstacle {}
impl From<GKCircleObstacle> for GKObstacle {
    fn from(child: GKCircleObstacle) -> GKObstacle {
        GKObstacle(child.0)
    }
}
impl std::convert::TryFrom<GKObstacle> for GKCircleObstacle {
    type Error = &'static str;
    fn try_from(parent: GKObstacle) -> Result<GKCircleObstacle, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"GKCircleObstacle").unwrap()) };
        if is_kind_of {
            Ok(GKCircleObstacle(parent.0))
        } else {
            Err("This GKObstacle cannot be downcasted to GKCircleObstacle")
        }
    }
}
impl INSObject for GKCircleObstacle {}
impl PNSObject for GKCircleObstacle {}
impl IGKCircleObstacle for GKCircleObstacle {}
pub trait IGKCircleObstacle: Sized + std::ops::Deref {
    unsafe fn initWithRadius_(&self, radius: f32) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithRadius : radius)
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
    unsafe fn position(&self) -> vector_float2
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, position)
    }
    unsafe fn setPosition_(&self, position: vector_float2)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPosition : position)
    }
    unsafe fn obstacleWithRadius_(radius: f32) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"GKCircleObstacle").unwrap(), obstacleWithRadius : radius)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct GKPolygonObstacle(pub id);
impl std::ops::Deref for GKPolygonObstacle {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for GKPolygonObstacle {}
impl GKPolygonObstacle {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"GKPolygonObstacle").unwrap(), alloc) })
    }
}
impl PNSSecureCoding for GKPolygonObstacle {}
impl IGKObstacle for GKPolygonObstacle {}
impl std::convert::TryFrom<GKObstacle> for GKPolygonObstacle {
    type Error = &'static str;
    fn try_from(parent: GKObstacle) -> Result<GKPolygonObstacle, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"GKPolygonObstacle").unwrap()) };
        if is_kind_of {
            Ok(GKPolygonObstacle(parent.0))
        } else {
            Err("This GKObstacle cannot be downcasted to GKPolygonObstacle")
        }
    }
}
impl INSObject for GKPolygonObstacle {}
impl PNSObject for GKPolygonObstacle {}
impl IGKPolygonObstacle for GKPolygonObstacle {}
pub trait IGKPolygonObstacle: Sized + std::ops::Deref {
    unsafe fn initWithPoints_count_(
        &self,
        points: *mut vector_float2,
        numPoints: usize,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithPoints : points, count : numPoints)
    }
    unsafe fn vertexAtIndex_(&self, index: NSUInteger) -> vector_float2
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, vertexAtIndex : index)
    }
    unsafe fn vertexCount(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, vertexCount)
    }
    unsafe fn obstacleWithPoints_count_(
        points: *mut vector_float2,
        numPoints: usize,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"GKPolygonObstacle").unwrap(), obstacleWithPoints : points, count : numPoints)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct GKSphereObstacle(pub id);
impl std::ops::Deref for GKSphereObstacle {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for GKSphereObstacle {}
impl GKSphereObstacle {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"GKSphereObstacle").unwrap(), alloc) })
    }
}
impl IGKObstacle for GKSphereObstacle {}
impl std::convert::TryFrom<GKObstacle> for GKSphereObstacle {
    type Error = &'static str;
    fn try_from(parent: GKObstacle) -> Result<GKSphereObstacle, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"GKSphereObstacle").unwrap()) };
        if is_kind_of {
            Ok(GKSphereObstacle(parent.0))
        } else {
            Err("This GKObstacle cannot be downcasted to GKSphereObstacle")
        }
    }
}
impl INSObject for GKSphereObstacle {}
impl PNSObject for GKSphereObstacle {}
impl IGKSphereObstacle for GKSphereObstacle {}
pub trait IGKSphereObstacle: Sized + std::ops::Deref {
    unsafe fn initWithRadius_(&self, radius: f32) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithRadius : radius)
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
    unsafe fn position(&self) -> vector_float3
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, position)
    }
    unsafe fn setPosition_(&self, position: vector_float3)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPosition : position)
    }
    unsafe fn obstacleWithRadius_(radius: f32) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"GKSphereObstacle").unwrap(), obstacleWithRadius : radius)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct GKBehavior(pub id);
impl std::ops::Deref for GKBehavior {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for GKBehavior {}
impl GKBehavior {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"GKBehavior").unwrap(), alloc) })
    }
}
impl PNSFastEnumeration for GKBehavior {}
impl PNSCopying for GKBehavior {}
impl INSObject for GKBehavior {}
impl PNSObject for GKBehavior {}
impl std::convert::TryFrom<NSObject> for GKBehavior {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<GKBehavior, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"GKBehavior").unwrap()) };
        if is_kind_of {
            Ok(GKBehavior(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to GKBehavior")
        }
    }
}
impl IGKBehavior for GKBehavior {}
pub trait IGKBehavior: Sized + std::ops::Deref {
    unsafe fn setWeight_forGoal_(&self, weight: f32, goal: GKGoal)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setWeight : weight, forGoal : goal)
    }
    unsafe fn weightForGoal_(&self, goal: GKGoal) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, weightForGoal : goal)
    }
    unsafe fn removeGoal_(&self, goal: GKGoal)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, removeGoal : goal)
    }
    unsafe fn removeAllGoals(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, removeAllGoals)
    }
    unsafe fn objectAtIndexedSubscript_(&self, idx: NSUInteger) -> GKGoal
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, objectAtIndexedSubscript : idx)
    }
    unsafe fn setObject_forKeyedSubscript_(&self, weight: NSNumber, goal: GKGoal)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setObject : weight, forKeyedSubscript : goal)
    }
    unsafe fn objectForKeyedSubscript_(&self, goal: GKGoal) -> NSNumber
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, objectForKeyedSubscript : goal)
    }
    unsafe fn goalCount(&self) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, goalCount)
    }
    unsafe fn behaviorWithGoal_weight_(goal: GKGoal, weight: f32) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"GKBehavior").unwrap(), behaviorWithGoal : goal, weight : weight)
    }
    unsafe fn behaviorWithGoals_(goals: NSArray) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"GKBehavior").unwrap(), behaviorWithGoals : goals)
    }
    unsafe fn behaviorWithGoals_andWeights_(goals: NSArray, weights: NSArray) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"GKBehavior").unwrap(), behaviorWithGoals : goals, andWeights : weights)
    }
    unsafe fn behaviorWithWeightedGoals_(weightedGoals: NSDictionary) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"GKBehavior").unwrap(), behaviorWithWeightedGoals : weightedGoals)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct GKGoal(pub id);
impl std::ops::Deref for GKGoal {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for GKGoal {}
impl GKGoal {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"GKGoal").unwrap(), alloc) })
    }
}
impl PNSCopying for GKGoal {}
impl INSObject for GKGoal {}
impl PNSObject for GKGoal {}
impl std::convert::TryFrom<NSObject> for GKGoal {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<GKGoal, Self::Error> {
        let is_kind_of: bool = unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"GKGoal").unwrap()) };
        if is_kind_of {
            Ok(GKGoal(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to GKGoal")
        }
    }
}
impl IGKGoal for GKGoal {}
pub trait IGKGoal: Sized + std::ops::Deref {
    unsafe fn goalToSeekAgent_(agent: GKAgent) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"GKGoal").unwrap(), goalToSeekAgent : agent)
    }
    unsafe fn goalToFleeAgent_(agent: GKAgent) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"GKGoal").unwrap(), goalToFleeAgent : agent)
    }
    unsafe fn goalToAvoidObstacles_maxPredictionTime_(
        obstacles: NSArray,
        maxPredictionTime: NSTimeInterval,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"GKGoal").unwrap(), goalToAvoidObstacles : obstacles, maxPredictionTime : maxPredictionTime)
    }
    unsafe fn goalToAvoidAgents_maxPredictionTime_(
        agents: NSArray,
        maxPredictionTime: NSTimeInterval,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"GKGoal").unwrap(), goalToAvoidAgents : agents, maxPredictionTime : maxPredictionTime)
    }
    unsafe fn goalToSeparateFromAgents_maxDistance_maxAngle_(
        agents: NSArray,
        maxDistance: f32,
        maxAngle: f32,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"GKGoal").unwrap(), goalToSeparateFromAgents : agents, maxDistance : maxDistance, maxAngle : maxAngle)
    }
    unsafe fn goalToAlignWithAgents_maxDistance_maxAngle_(
        agents: NSArray,
        maxDistance: f32,
        maxAngle: f32,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"GKGoal").unwrap(), goalToAlignWithAgents : agents, maxDistance : maxDistance, maxAngle : maxAngle)
    }
    unsafe fn goalToCohereWithAgents_maxDistance_maxAngle_(
        agents: NSArray,
        maxDistance: f32,
        maxAngle: f32,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"GKGoal").unwrap(), goalToCohereWithAgents : agents, maxDistance : maxDistance, maxAngle : maxAngle)
    }
    unsafe fn goalToReachTargetSpeed_(targetSpeed: f32) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"GKGoal").unwrap(), goalToReachTargetSpeed : targetSpeed)
    }
    unsafe fn goalToWander_(speed: f32) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"GKGoal").unwrap(), goalToWander : speed)
    }
    unsafe fn goalToInterceptAgent_maxPredictionTime_(
        target: GKAgent,
        maxPredictionTime: NSTimeInterval,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"GKGoal").unwrap(), goalToInterceptAgent : target, maxPredictionTime : maxPredictionTime)
    }
    unsafe fn goalToFollowPath_maxPredictionTime_forward_(
        path: GKPath,
        maxPredictionTime: NSTimeInterval,
        forward: BOOL,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"GKGoal").unwrap(), goalToFollowPath : path, maxPredictionTime : maxPredictionTime, forward : forward)
    }
    unsafe fn goalToStayOnPath_maxPredictionTime_(
        path: GKPath,
        maxPredictionTime: NSTimeInterval,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"GKGoal").unwrap(), goalToStayOnPath : path, maxPredictionTime : maxPredictionTime)
    }
}
pub trait PGKAgentDelegate: Sized + std::ops::Deref {
    unsafe fn agentWillUpdate_(&self, agent: GKAgent)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, agentWillUpdate : agent)
    }
    unsafe fn agentDidUpdate_(&self, agent: GKAgent)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, agentDidUpdate : agent)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct GKAgent(pub id);
impl std::ops::Deref for GKAgent {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for GKAgent {}
impl GKAgent {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"GKAgent").unwrap(), alloc) })
    }
}
impl PNSSecureCoding for GKAgent {}
impl IGKComponent for GKAgent {}
impl PNSCopying for GKAgent {}
impl From<GKAgent> for GKComponent {
    fn from(child: GKAgent) -> GKComponent {
        GKComponent(child.0)
    }
}
impl std::convert::TryFrom<GKComponent> for GKAgent {
    type Error = &'static str;
    fn try_from(parent: GKComponent) -> Result<GKAgent, Self::Error> {
        let is_kind_of: bool = unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"GKAgent").unwrap()) };
        if is_kind_of {
            Ok(GKAgent(parent.0))
        } else {
            Err("This GKComponent cannot be downcasted to GKAgent")
        }
    }
}
impl INSObject for GKAgent {}
impl PNSObject for GKAgent {}
impl IGKAgent for GKAgent {}
pub trait IGKAgent: Sized + std::ops::Deref {
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
    unsafe fn behavior(&self) -> GKBehavior
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, behavior)
    }
    unsafe fn setBehavior_(&self, behavior: GKBehavior)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setBehavior : behavior)
    }
    unsafe fn mass(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, mass)
    }
    unsafe fn setMass_(&self, mass: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMass : mass)
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
    unsafe fn maxAcceleration(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, maxAcceleration)
    }
    unsafe fn setMaxAcceleration_(&self, maxAcceleration: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMaxAcceleration : maxAcceleration)
    }
    unsafe fn maxSpeed(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, maxSpeed)
    }
    unsafe fn setMaxSpeed_(&self, maxSpeed: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMaxSpeed : maxSpeed)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct GKAgent2D(pub id);
impl std::ops::Deref for GKAgent2D {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for GKAgent2D {}
impl GKAgent2D {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"GKAgent2D").unwrap(), alloc) })
    }
}
impl PNSSecureCoding for GKAgent2D {}
impl IGKAgent for GKAgent2D {}
impl From<GKAgent2D> for GKAgent {
    fn from(child: GKAgent2D) -> GKAgent {
        GKAgent(child.0)
    }
}
impl std::convert::TryFrom<GKAgent> for GKAgent2D {
    type Error = &'static str;
    fn try_from(parent: GKAgent) -> Result<GKAgent2D, Self::Error> {
        let is_kind_of: bool = unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"GKAgent2D").unwrap()) };
        if is_kind_of {
            Ok(GKAgent2D(parent.0))
        } else {
            Err("This GKAgent cannot be downcasted to GKAgent2D")
        }
    }
}
impl IGKComponent for GKAgent2D {}
impl PNSCopying for GKAgent2D {}
impl INSObject for GKAgent2D {}
impl PNSObject for GKAgent2D {}
impl IGKAgent2D for GKAgent2D {}
pub trait IGKAgent2D: Sized + std::ops::Deref {
    unsafe fn updateWithDeltaTime_(&self, seconds: NSTimeInterval)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, updateWithDeltaTime : seconds)
    }
    unsafe fn position(&self) -> vector_float2
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, position)
    }
    unsafe fn setPosition_(&self, position: vector_float2)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPosition : position)
    }
    unsafe fn velocity(&self) -> vector_float2
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, velocity)
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
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct GKAgent3D(pub id);
impl std::ops::Deref for GKAgent3D {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for GKAgent3D {}
impl GKAgent3D {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"GKAgent3D").unwrap(), alloc) })
    }
}
impl IGKAgent for GKAgent3D {}
impl PNSSecureCoding for GKAgent3D {}
impl std::convert::TryFrom<GKAgent> for GKAgent3D {
    type Error = &'static str;
    fn try_from(parent: GKAgent) -> Result<GKAgent3D, Self::Error> {
        let is_kind_of: bool = unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"GKAgent3D").unwrap()) };
        if is_kind_of {
            Ok(GKAgent3D(parent.0))
        } else {
            Err("This GKAgent cannot be downcasted to GKAgent3D")
        }
    }
}
impl IGKComponent for GKAgent3D {}
impl PNSCopying for GKAgent3D {}
impl INSObject for GKAgent3D {}
impl PNSObject for GKAgent3D {}
impl IGKAgent3D for GKAgent3D {}
pub trait IGKAgent3D: Sized + std::ops::Deref {
    unsafe fn updateWithDeltaTime_(&self, seconds: NSTimeInterval)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, updateWithDeltaTime : seconds)
    }
    unsafe fn position(&self) -> vector_float3
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, position)
    }
    unsafe fn setPosition_(&self, position: vector_float3)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPosition : position)
    }
    unsafe fn velocity(&self) -> vector_float3
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, velocity)
    }
    unsafe fn rightHanded(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, rightHanded)
    }
    unsafe fn setRightHanded_(&self, rightHanded: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRightHanded : rightHanded)
    }
    unsafe fn rotation(&self) -> matrix_float3x3
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, rotation)
    }
    unsafe fn setRotation_(&self, rotation: matrix_float3x3)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRotation : rotation)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct GKCompositeBehavior(pub id);
impl std::ops::Deref for GKCompositeBehavior {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for GKCompositeBehavior {}
impl GKCompositeBehavior {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"GKCompositeBehavior").unwrap(), alloc) })
    }
}
impl IGKBehavior for GKCompositeBehavior {}
impl PNSFastEnumeration for GKCompositeBehavior {}
impl PNSCopying for GKCompositeBehavior {}
impl From<GKCompositeBehavior> for GKBehavior {
    fn from(child: GKCompositeBehavior) -> GKBehavior {
        GKBehavior(child.0)
    }
}
impl std::convert::TryFrom<GKBehavior> for GKCompositeBehavior {
    type Error = &'static str;
    fn try_from(parent: GKBehavior) -> Result<GKCompositeBehavior, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"GKCompositeBehavior").unwrap()) };
        if is_kind_of {
            Ok(GKCompositeBehavior(parent.0))
        } else {
            Err("This GKBehavior cannot be downcasted to GKCompositeBehavior")
        }
    }
}
impl INSObject for GKCompositeBehavior {}
impl PNSObject for GKCompositeBehavior {}
impl IGKCompositeBehavior for GKCompositeBehavior {}
pub trait IGKCompositeBehavior: Sized + std::ops::Deref {
    unsafe fn setWeight_forBehavior_(&self, weight: f32, behavior: GKBehavior)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setWeight : weight, forBehavior : behavior)
    }
    unsafe fn weightForBehavior_(&self, behavior: GKBehavior) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, weightForBehavior : behavior)
    }
    unsafe fn removeBehavior_(&self, behavior: GKBehavior)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, removeBehavior : behavior)
    }
    unsafe fn removeAllBehaviors(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, removeAllBehaviors)
    }
    unsafe fn objectAtIndexedSubscript_(&self, idx: NSUInteger) -> GKBehavior
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, objectAtIndexedSubscript : idx)
    }
    unsafe fn setObject_forKeyedSubscript_(&self, weight: NSNumber, behavior: GKBehavior)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setObject : weight, forKeyedSubscript : behavior)
    }
    unsafe fn objectForKeyedSubscript_(&self, behavior: GKBehavior) -> NSNumber
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, objectForKeyedSubscript : behavior)
    }
    unsafe fn behaviorCount(&self) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, behaviorCount)
    }
    unsafe fn behaviorWithBehaviors_(behaviors: NSArray) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"GKCompositeBehavior").unwrap(), behaviorWithBehaviors : behaviors)
    }
    unsafe fn behaviorWithBehaviors_andWeights_(
        behaviors: NSArray,
        weights: NSArray,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"GKCompositeBehavior").unwrap(), behaviorWithBehaviors : behaviors, andWeights : weights)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct GKEntity(pub id);
impl std::ops::Deref for GKEntity {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for GKEntity {}
impl GKEntity {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"GKEntity").unwrap(), alloc) })
    }
}
impl PNSCopying for GKEntity {}
impl PNSSecureCoding for GKEntity {}
impl INSObject for GKEntity {}
impl PNSObject for GKEntity {}
impl std::convert::TryFrom<NSObject> for GKEntity {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<GKEntity, Self::Error> {
        let is_kind_of: bool = unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"GKEntity").unwrap()) };
        if is_kind_of {
            Ok(GKEntity(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to GKEntity")
        }
    }
}
impl IGKEntity for GKEntity {}
pub trait IGKEntity: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn updateWithDeltaTime_(&self, seconds: NSTimeInterval)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, updateWithDeltaTime : seconds)
    }
    unsafe fn addComponent_(&self, component: GKComponent)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, addComponent : component)
    }
    unsafe fn removeComponentForClass_(&self, componentClass: Class)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, removeComponentForClass : componentClass)
    }
    unsafe fn componentForClass_(&self, componentClass: Class) -> GKComponent
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, componentForClass : componentClass)
    }
    unsafe fn components(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, components)
    }
    unsafe fn entity() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"GKEntity").unwrap(), entity)
    }
}
pub trait PGKGameModelUpdate: Sized + std::ops::Deref {
    unsafe fn value(&self) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, value)
    }
    unsafe fn setValue_(&self, value: NSInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setValue : value)
    }
}
pub trait PGKGameModelPlayer: Sized + std::ops::Deref {
    unsafe fn playerId(&self) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, playerId)
    }
}
pub trait PGKGameModel: Sized + std::ops::Deref {
    unsafe fn setGameModel_(&self, gameModel: *mut u64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setGameModel : gameModel)
    }
    unsafe fn gameModelUpdatesForPlayer_(&self, player: *mut u64) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, gameModelUpdatesForPlayer : player)
    }
    unsafe fn applyGameModelUpdate_(&self, gameModelUpdate: *mut u64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, applyGameModelUpdate : gameModelUpdate)
    }
    unsafe fn scoreForPlayer_(&self, player: *mut u64) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, scoreForPlayer : player)
    }
    unsafe fn isWinForPlayer_(&self, player: *mut u64) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, isWinForPlayer : player)
    }
    unsafe fn isLossForPlayer_(&self, player: *mut u64) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, isLossForPlayer : player)
    }
    unsafe fn unapplyGameModelUpdate_(&self, gameModelUpdate: *mut u64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, unapplyGameModelUpdate : gameModelUpdate)
    }
    unsafe fn players(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, players)
    }
    unsafe fn activePlayer(&self) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, activePlayer)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct GKGraph(pub id);
impl std::ops::Deref for GKGraph {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for GKGraph {}
impl GKGraph {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"GKGraph").unwrap(), alloc) })
    }
}
impl PNSCopying for GKGraph {}
impl PNSSecureCoding for GKGraph {}
impl INSObject for GKGraph {}
impl PNSObject for GKGraph {}
impl std::convert::TryFrom<NSObject> for GKGraph {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<GKGraph, Self::Error> {
        let is_kind_of: bool = unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"GKGraph").unwrap()) };
        if is_kind_of {
            Ok(GKGraph(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to GKGraph")
        }
    }
}
impl IGKGraph for GKGraph {}
pub trait IGKGraph: Sized + std::ops::Deref {
    unsafe fn initWithNodes_(&self, nodes: NSArray) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithNodes : nodes)
    }
    unsafe fn connectNodeToLowestCostNode_bidirectional_(
        &self,
        node: GKGraphNode,
        bidirectional: BOOL,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, connectNodeToLowestCostNode : node, bidirectional : bidirectional)
    }
    unsafe fn removeNodes_(&self, nodes: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, removeNodes : nodes)
    }
    unsafe fn addNodes_(&self, nodes: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, addNodes : nodes)
    }
    unsafe fn findPathFromNode_toNode_(
        &self,
        startNode: GKGraphNode,
        endNode: GKGraphNode,
    ) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, findPathFromNode : startNode, toNode : endNode)
    }
    unsafe fn nodes(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, nodes)
    }
    unsafe fn graphWithNodes_(nodes: NSArray) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"GKGraph").unwrap(), graphWithNodes : nodes)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct GKGridGraph(pub id);
impl std::ops::Deref for GKGridGraph {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for GKGridGraph {}
impl GKGridGraph {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"GKGridGraph").unwrap(), alloc) })
    }
}
impl IGKGraph for GKGridGraph {}
impl PNSCopying for GKGridGraph {}
impl PNSSecureCoding for GKGridGraph {}
impl From<GKGridGraph> for GKGraph {
    fn from(child: GKGridGraph) -> GKGraph {
        GKGraph(child.0)
    }
}
impl std::convert::TryFrom<GKGraph> for GKGridGraph {
    type Error = &'static str;
    fn try_from(parent: GKGraph) -> Result<GKGridGraph, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"GKGridGraph").unwrap()) };
        if is_kind_of {
            Ok(GKGridGraph(parent.0))
        } else {
            Err("This GKGraph cannot be downcasted to GKGridGraph")
        }
    }
}
impl INSObject for GKGridGraph {}
impl PNSObject for GKGridGraph {}
impl<NodeType: 'static> IGKGridGraph<NodeType> for GKGridGraph {}
pub trait IGKGridGraph<NodeType: 'static>: Sized + std::ops::Deref {
    unsafe fn initFromGridStartingAt_width_height_diagonalsAllowed_(
        &self,
        position: vector_int2,
        width: ::std::os::raw::c_int,
        height: ::std::os::raw::c_int,
        diagonalsAllowed: BOOL,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initFromGridStartingAt : position, width : width, height : height, diagonalsAllowed : diagonalsAllowed)
    }
    unsafe fn initFromGridStartingAt_width_height_diagonalsAllowed_nodeClass_(
        &self,
        position: vector_int2,
        width: ::std::os::raw::c_int,
        height: ::std::os::raw::c_int,
        diagonalsAllowed: BOOL,
        nodeClass: Class,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initFromGridStartingAt : position, width : width, height : height, diagonalsAllowed : diagonalsAllowed, nodeClass : nodeClass)
    }
    unsafe fn nodeAtGridPosition_(&self, position: vector_int2) -> id
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, nodeAtGridPosition : position)
    }
    unsafe fn connectNodeToAdjacentNodes_(&self, node: GKGridGraphNode)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, connectNodeToAdjacentNodes : node)
    }
    unsafe fn classForGenericArgumentAtIndex_(&self, index: NSUInteger) -> Class
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, classForGenericArgumentAtIndex : index)
    }
    unsafe fn gridOrigin(&self) -> vector_int2
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, gridOrigin)
    }
    unsafe fn gridWidth(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, gridWidth)
    }
    unsafe fn gridHeight(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, gridHeight)
    }
    unsafe fn diagonalsAllowed(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, diagonalsAllowed)
    }
    unsafe fn graphFromGridStartingAt_width_height_diagonalsAllowed_(
        position: vector_int2,
        width: ::std::os::raw::c_int,
        height: ::std::os::raw::c_int,
        diagonalsAllowed: BOOL,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"GKGridGraph").unwrap(), graphFromGridStartingAt : position, width : width, height : height, diagonalsAllowed : diagonalsAllowed)
    }
    unsafe fn graphFromGridStartingAt_width_height_diagonalsAllowed_nodeClass_(
        position: vector_int2,
        width: ::std::os::raw::c_int,
        height: ::std::os::raw::c_int,
        diagonalsAllowed: BOOL,
        nodeClass: Class,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"GKGridGraph").unwrap(), graphFromGridStartingAt : position, width : width, height : height, diagonalsAllowed : diagonalsAllowed, nodeClass : nodeClass)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct GKObstacleGraph(pub id);
impl std::ops::Deref for GKObstacleGraph {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for GKObstacleGraph {}
impl GKObstacleGraph {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"GKObstacleGraph").unwrap(), alloc) })
    }
}
impl IGKGraph for GKObstacleGraph {}
impl PNSCopying for GKObstacleGraph {}
impl PNSSecureCoding for GKObstacleGraph {}
impl std::convert::TryFrom<GKGraph> for GKObstacleGraph {
    type Error = &'static str;
    fn try_from(parent: GKGraph) -> Result<GKObstacleGraph, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"GKObstacleGraph").unwrap()) };
        if is_kind_of {
            Ok(GKObstacleGraph(parent.0))
        } else {
            Err("This GKGraph cannot be downcasted to GKObstacleGraph")
        }
    }
}
impl INSObject for GKObstacleGraph {}
impl PNSObject for GKObstacleGraph {}
impl<NodeType: 'static> IGKObstacleGraph<NodeType> for GKObstacleGraph {}
pub trait IGKObstacleGraph<NodeType: 'static>: Sized + std::ops::Deref {
    unsafe fn initWithObstacles_bufferRadius_(
        &self,
        obstacles: NSArray,
        bufferRadius: f32,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithObstacles : obstacles, bufferRadius : bufferRadius)
    }
    unsafe fn initWithObstacles_bufferRadius_nodeClass_(
        &self,
        obstacles: NSArray,
        bufferRadius: f32,
        nodeClass: Class,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithObstacles : obstacles, bufferRadius : bufferRadius, nodeClass : nodeClass)
    }
    unsafe fn connectNodeUsingObstacles_(&self, node: GKGraphNode2D)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, connectNodeUsingObstacles : node)
    }
    unsafe fn connectNodeUsingObstacles_ignoringObstacles_(
        &self,
        node: GKGraphNode2D,
        obstaclesToIgnore: NSArray,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, connectNodeUsingObstacles : node, ignoringObstacles : obstaclesToIgnore)
    }
    unsafe fn connectNodeUsingObstacles_ignoringBufferRadiusOfObstacles_(
        &self,
        node: GKGraphNode2D,
        obstaclesBufferRadiusToIgnore: NSArray,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, connectNodeUsingObstacles : node, ignoringBufferRadiusOfObstacles : obstaclesBufferRadiusToIgnore)
    }
    unsafe fn addObstacles_(&self, obstacles: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, addObstacles : obstacles)
    }
    unsafe fn removeObstacles_(&self, obstacles: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, removeObstacles : obstacles)
    }
    unsafe fn removeAllObstacles(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, removeAllObstacles)
    }
    unsafe fn nodesForObstacle_(&self, obstacle: GKPolygonObstacle) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, nodesForObstacle : obstacle)
    }
    unsafe fn lockConnectionFromNode_toNode_(
        &self,
        startNode: GKGraphNode2D,
        endNode: GKGraphNode2D,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, lockConnectionFromNode : startNode, toNode : endNode)
    }
    unsafe fn unlockConnectionFromNode_toNode_(
        &self,
        startNode: GKGraphNode2D,
        endNode: GKGraphNode2D,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, unlockConnectionFromNode : startNode, toNode : endNode)
    }
    unsafe fn isConnectionLockedFromNode_toNode_(
        &self,
        startNode: GKGraphNode2D,
        endNode: GKGraphNode2D,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, isConnectionLockedFromNode : startNode, toNode : endNode)
    }
    unsafe fn classForGenericArgumentAtIndex_(&self, index: NSUInteger) -> Class
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, classForGenericArgumentAtIndex : index)
    }
    unsafe fn obstacles(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, obstacles)
    }
    unsafe fn bufferRadius(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, bufferRadius)
    }
    unsafe fn graphWithObstacles_bufferRadius_(
        obstacles: NSArray,
        bufferRadius: f32,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"GKObstacleGraph").unwrap(), graphWithObstacles : obstacles, bufferRadius : bufferRadius)
    }
    unsafe fn graphWithObstacles_bufferRadius_nodeClass_(
        obstacles: NSArray,
        bufferRadius: f32,
        nodeClass: Class,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"GKObstacleGraph").unwrap(), graphWithObstacles : obstacles, bufferRadius : bufferRadius, nodeClass : nodeClass)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct GKGraphNode(pub id);
impl std::ops::Deref for GKGraphNode {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for GKGraphNode {}
impl GKGraphNode {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"GKGraphNode").unwrap(), alloc) })
    }
}
impl PNSSecureCoding for GKGraphNode {}
impl INSObject for GKGraphNode {}
impl PNSObject for GKGraphNode {}
impl std::convert::TryFrom<NSObject> for GKGraphNode {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<GKGraphNode, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"GKGraphNode").unwrap()) };
        if is_kind_of {
            Ok(GKGraphNode(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to GKGraphNode")
        }
    }
}
impl IGKGraphNode for GKGraphNode {}
pub trait IGKGraphNode: Sized + std::ops::Deref {
    unsafe fn addConnectionsToNodes_bidirectional_(&self, nodes: NSArray, bidirectional: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, addConnectionsToNodes : nodes, bidirectional : bidirectional)
    }
    unsafe fn removeConnectionsToNodes_bidirectional_(&self, nodes: NSArray, bidirectional: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, removeConnectionsToNodes : nodes, bidirectional : bidirectional)
    }
    unsafe fn estimatedCostToNode_(&self, node: GKGraphNode) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, estimatedCostToNode : node)
    }
    unsafe fn costToNode_(&self, node: GKGraphNode) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, costToNode : node)
    }
    unsafe fn findPathToNode_(&self, goalNode: GKGraphNode) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, findPathToNode : goalNode)
    }
    unsafe fn findPathFromNode_(&self, startNode: GKGraphNode) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, findPathFromNode : startNode)
    }
    unsafe fn connectedNodes(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, connectedNodes)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct GKGraphNode2D(pub id);
impl std::ops::Deref for GKGraphNode2D {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for GKGraphNode2D {}
impl GKGraphNode2D {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"GKGraphNode2D").unwrap(), alloc) })
    }
}
impl IGKGraphNode for GKGraphNode2D {}
impl PNSSecureCoding for GKGraphNode2D {}
impl From<GKGraphNode2D> for GKGraphNode {
    fn from(child: GKGraphNode2D) -> GKGraphNode {
        GKGraphNode(child.0)
    }
}
impl std::convert::TryFrom<GKGraphNode> for GKGraphNode2D {
    type Error = &'static str;
    fn try_from(parent: GKGraphNode) -> Result<GKGraphNode2D, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"GKGraphNode2D").unwrap()) };
        if is_kind_of {
            Ok(GKGraphNode2D(parent.0))
        } else {
            Err("This GKGraphNode cannot be downcasted to GKGraphNode2D")
        }
    }
}
impl INSObject for GKGraphNode2D {}
impl PNSObject for GKGraphNode2D {}
impl IGKGraphNode2D for GKGraphNode2D {}
pub trait IGKGraphNode2D: Sized + std::ops::Deref {
    unsafe fn initWithPoint_(&self, point: vector_float2) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithPoint : point)
    }
    unsafe fn position(&self) -> vector_float2
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, position)
    }
    unsafe fn setPosition_(&self, position: vector_float2)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPosition : position)
    }
    unsafe fn nodeWithPoint_(point: vector_float2) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"GKGraphNode2D").unwrap(), nodeWithPoint : point)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct GKGraphNode3D(pub id);
impl std::ops::Deref for GKGraphNode3D {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for GKGraphNode3D {}
impl GKGraphNode3D {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"GKGraphNode3D").unwrap(), alloc) })
    }
}
impl IGKGraphNode for GKGraphNode3D {}
impl PNSSecureCoding for GKGraphNode3D {}
impl std::convert::TryFrom<GKGraphNode> for GKGraphNode3D {
    type Error = &'static str;
    fn try_from(parent: GKGraphNode) -> Result<GKGraphNode3D, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"GKGraphNode3D").unwrap()) };
        if is_kind_of {
            Ok(GKGraphNode3D(parent.0))
        } else {
            Err("This GKGraphNode cannot be downcasted to GKGraphNode3D")
        }
    }
}
impl INSObject for GKGraphNode3D {}
impl PNSObject for GKGraphNode3D {}
impl IGKGraphNode3D for GKGraphNode3D {}
pub trait IGKGraphNode3D: Sized + std::ops::Deref {
    unsafe fn initWithPoint_(&self, point: vector_float3) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithPoint : point)
    }
    unsafe fn position(&self) -> vector_float3
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, position)
    }
    unsafe fn setPosition_(&self, position: vector_float3)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPosition : position)
    }
    unsafe fn nodeWithPoint_(point: vector_float3) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"GKGraphNode3D").unwrap(), nodeWithPoint : point)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct GKGridGraphNode(pub id);
impl std::ops::Deref for GKGridGraphNode {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for GKGridGraphNode {}
impl GKGridGraphNode {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"GKGridGraphNode").unwrap(), alloc) })
    }
}
impl IGKGraphNode for GKGridGraphNode {}
impl PNSSecureCoding for GKGridGraphNode {}
impl std::convert::TryFrom<GKGraphNode> for GKGridGraphNode {
    type Error = &'static str;
    fn try_from(parent: GKGraphNode) -> Result<GKGridGraphNode, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"GKGridGraphNode").unwrap()) };
        if is_kind_of {
            Ok(GKGridGraphNode(parent.0))
        } else {
            Err("This GKGraphNode cannot be downcasted to GKGridGraphNode")
        }
    }
}
impl INSObject for GKGridGraphNode {}
impl PNSObject for GKGridGraphNode {}
impl IGKGridGraphNode for GKGridGraphNode {}
pub trait IGKGridGraphNode: Sized + std::ops::Deref {
    unsafe fn initWithGridPosition_(&self, gridPosition: vector_int2) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithGridPosition : gridPosition)
    }
    unsafe fn gridPosition(&self) -> vector_int2
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, gridPosition)
    }
    unsafe fn nodeWithGridPosition_(gridPosition: vector_int2) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"GKGridGraphNode").unwrap(), nodeWithGridPosition : gridPosition)
    }
}
pub trait PGKRandom: Sized + std::ops::Deref {
    unsafe fn nextInt(&self) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, nextInt)
    }
    unsafe fn nextIntWithUpperBound_(&self, upperBound: NSUInteger) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, nextIntWithUpperBound : upperBound)
    }
    unsafe fn nextUniform(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, nextUniform)
    }
    unsafe fn nextBool(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, nextBool)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct GKRandomSource(pub id);
impl std::ops::Deref for GKRandomSource {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for GKRandomSource {}
impl GKRandomSource {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"GKRandomSource").unwrap(), alloc) })
    }
}
impl PGKRandom for GKRandomSource {}
impl PNSSecureCoding for GKRandomSource {}
impl PNSCopying for GKRandomSource {}
impl INSObject for GKRandomSource {}
impl PNSObject for GKRandomSource {}
impl std::convert::TryFrom<NSObject> for GKRandomSource {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<GKRandomSource, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"GKRandomSource").unwrap()) };
        if is_kind_of {
            Ok(GKRandomSource(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to GKRandomSource")
        }
    }
}
impl IGKRandomSource for GKRandomSource {}
pub trait IGKRandomSource: Sized + std::ops::Deref {
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
    unsafe fn arrayByShufflingObjectsInArray_(&self, array: NSArray) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, arrayByShufflingObjectsInArray : array)
    }
    unsafe fn sharedRandom() -> GKRandomSource
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"GKRandomSource").unwrap(), sharedRandom)
    }
}
pub trait NSArray_GameplayKit<ObjectType: 'static>: Sized + std::ops::Deref {
    unsafe fn shuffledArrayWithRandomSource_(&self, randomSource: GKRandomSource) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, shuffledArrayWithRandomSource : randomSource)
    }
    unsafe fn shuffledArray(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, shuffledArray)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct GKARC4RandomSource(pub id);
impl std::ops::Deref for GKARC4RandomSource {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for GKARC4RandomSource {}
impl GKARC4RandomSource {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"GKARC4RandomSource").unwrap(), alloc) })
    }
}
impl IGKRandomSource for GKARC4RandomSource {}
impl PGKRandom for GKARC4RandomSource {}
impl PNSSecureCoding for GKARC4RandomSource {}
impl PNSCopying for GKARC4RandomSource {}
impl From<GKARC4RandomSource> for GKRandomSource {
    fn from(child: GKARC4RandomSource) -> GKRandomSource {
        GKRandomSource(child.0)
    }
}
impl std::convert::TryFrom<GKRandomSource> for GKARC4RandomSource {
    type Error = &'static str;
    fn try_from(parent: GKRandomSource) -> Result<GKARC4RandomSource, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"GKARC4RandomSource").unwrap()) };
        if is_kind_of {
            Ok(GKARC4RandomSource(parent.0))
        } else {
            Err("This GKRandomSource cannot be downcasted to GKARC4RandomSource")
        }
    }
}
impl INSObject for GKARC4RandomSource {}
impl PNSObject for GKARC4RandomSource {}
impl IGKARC4RandomSource for GKARC4RandomSource {}
pub trait IGKARC4RandomSource: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn initWithSeed_(&self, seed: NSData) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithSeed : seed)
    }
    unsafe fn dropValuesWithCount_(&self, count: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, dropValuesWithCount : count)
    }
    unsafe fn seed(&self) -> NSData
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, seed)
    }
    unsafe fn setSeed_(&self, seed: NSData)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSeed : seed)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct GKLinearCongruentialRandomSource(pub id);
impl std::ops::Deref for GKLinearCongruentialRandomSource {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for GKLinearCongruentialRandomSource {}
impl GKLinearCongruentialRandomSource {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"GKLinearCongruentialRandomSource").unwrap(), alloc) })
    }
}
impl IGKRandomSource for GKLinearCongruentialRandomSource {}
impl PGKRandom for GKLinearCongruentialRandomSource {}
impl PNSSecureCoding for GKLinearCongruentialRandomSource {}
impl PNSCopying for GKLinearCongruentialRandomSource {}
impl std::convert::TryFrom<GKRandomSource> for GKLinearCongruentialRandomSource {
    type Error = &'static str;
    fn try_from(parent: GKRandomSource) -> Result<GKLinearCongruentialRandomSource, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"GKLinearCongruentialRandomSource").unwrap())
        };
        if is_kind_of {
            Ok(GKLinearCongruentialRandomSource(parent.0))
        } else {
            Err("This GKRandomSource cannot be downcasted to GKLinearCongruentialRandomSource")
        }
    }
}
impl INSObject for GKLinearCongruentialRandomSource {}
impl PNSObject for GKLinearCongruentialRandomSource {}
impl IGKLinearCongruentialRandomSource for GKLinearCongruentialRandomSource {}
pub trait IGKLinearCongruentialRandomSource: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn initWithSeed_(&self, seed: u64) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithSeed : seed)
    }
    unsafe fn seed(&self) -> u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, seed)
    }
    unsafe fn setSeed_(&self, seed: u64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSeed : seed)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct GKMersenneTwisterRandomSource(pub id);
impl std::ops::Deref for GKMersenneTwisterRandomSource {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for GKMersenneTwisterRandomSource {}
impl GKMersenneTwisterRandomSource {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"GKMersenneTwisterRandomSource").unwrap(), alloc) })
    }
}
impl IGKRandomSource for GKMersenneTwisterRandomSource {}
impl PGKRandom for GKMersenneTwisterRandomSource {}
impl PNSSecureCoding for GKMersenneTwisterRandomSource {}
impl PNSCopying for GKMersenneTwisterRandomSource {}
impl std::convert::TryFrom<GKRandomSource> for GKMersenneTwisterRandomSource {
    type Error = &'static str;
    fn try_from(parent: GKRandomSource) -> Result<GKMersenneTwisterRandomSource, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"GKMersenneTwisterRandomSource").unwrap())
        };
        if is_kind_of {
            Ok(GKMersenneTwisterRandomSource(parent.0))
        } else {
            Err("This GKRandomSource cannot be downcasted to GKMersenneTwisterRandomSource")
        }
    }
}
impl INSObject for GKMersenneTwisterRandomSource {}
impl PNSObject for GKMersenneTwisterRandomSource {}
impl IGKMersenneTwisterRandomSource for GKMersenneTwisterRandomSource {}
pub trait IGKMersenneTwisterRandomSource: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn initWithSeed_(&self, seed: u64) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithSeed : seed)
    }
    unsafe fn seed(&self) -> u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, seed)
    }
    unsafe fn setSeed_(&self, seed: u64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSeed : seed)
    }
}
pub trait PGKStrategist: Sized + std::ops::Deref {
    unsafe fn bestMoveForActivePlayer(&self) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, bestMoveForActivePlayer)
    }
    unsafe fn gameModel(&self) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, gameModel)
    }
    unsafe fn setGameModel_(&self, gameModel: *mut u64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setGameModel : gameModel)
    }
    unsafe fn randomSource(&self) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, randomSource)
    }
    unsafe fn setRandomSource_(&self, randomSource: *mut u64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRandomSource : randomSource)
    }
}
#[repr(C)]
#[repr(align(16))]
#[derive(Debug, Copy, Clone)]
pub struct GKBox {
    pub boxMin: vector_float3,
    pub boxMax: vector_float3,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct GKQuad {
    pub quadMin: vector_float2,
    pub quadMax: vector_float2,
}
#[repr(C)]
#[repr(align(16))]
#[derive(Debug, Copy, Clone)]
pub struct GKTriangle {
    pub points: [vector_float3; 3usize],
}
pub type GKMeshGraphTriangulationMode = NSUInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct GKMeshGraph(pub id);
impl std::ops::Deref for GKMeshGraph {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for GKMeshGraph {}
impl GKMeshGraph {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"GKMeshGraph").unwrap(), alloc) })
    }
}
impl IGKGraph for GKMeshGraph {}
impl PNSCopying for GKMeshGraph {}
impl PNSSecureCoding for GKMeshGraph {}
impl std::convert::TryFrom<GKGraph> for GKMeshGraph {
    type Error = &'static str;
    fn try_from(parent: GKGraph) -> Result<GKMeshGraph, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"GKMeshGraph").unwrap()) };
        if is_kind_of {
            Ok(GKMeshGraph(parent.0))
        } else {
            Err("This GKGraph cannot be downcasted to GKMeshGraph")
        }
    }
}
impl INSObject for GKMeshGraph {}
impl PNSObject for GKMeshGraph {}
impl<NodeType: 'static> IGKMeshGraph<NodeType> for GKMeshGraph {}
pub trait IGKMeshGraph<NodeType: 'static>: Sized + std::ops::Deref {
    unsafe fn initWithBufferRadius_minCoordinate_maxCoordinate_nodeClass_(
        &self,
        bufferRadius: f32,
        min: vector_float2,
        max: vector_float2,
        nodeClass: Class,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithBufferRadius : bufferRadius, minCoordinate : min, maxCoordinate : max, nodeClass : nodeClass)
    }
    unsafe fn initWithBufferRadius_minCoordinate_maxCoordinate_(
        &self,
        bufferRadius: f32,
        min: vector_float2,
        max: vector_float2,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithBufferRadius : bufferRadius, minCoordinate : min, maxCoordinate : max)
    }
    unsafe fn addObstacles_(&self, obstacles: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, addObstacles : obstacles)
    }
    unsafe fn removeObstacles_(&self, obstacles: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, removeObstacles : obstacles)
    }
    unsafe fn connectNodeUsingObstacles_(&self, node: GKGraphNode2D)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, connectNodeUsingObstacles : node)
    }
    unsafe fn triangulate(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, triangulate)
    }
    unsafe fn triangleAtIndex_(&self, index: NSUInteger) -> GKTriangle
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, triangleAtIndex : index)
    }
    unsafe fn classForGenericArgumentAtIndex_(&self, index: NSUInteger) -> Class
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, classForGenericArgumentAtIndex : index)
    }
    unsafe fn obstacles(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, obstacles)
    }
    unsafe fn bufferRadius(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, bufferRadius)
    }
    unsafe fn triangulationMode(&self) -> GKMeshGraphTriangulationMode
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, triangulationMode)
    }
    unsafe fn setTriangulationMode_(&self, triangulationMode: GKMeshGraphTriangulationMode)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setTriangulationMode : triangulationMode)
    }
    unsafe fn triangleCount(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, triangleCount)
    }
    unsafe fn graphWithBufferRadius_minCoordinate_maxCoordinate_nodeClass_(
        bufferRadius: f32,
        min: vector_float2,
        max: vector_float2,
        nodeClass: Class,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"GKMeshGraph").unwrap(), graphWithBufferRadius : bufferRadius, minCoordinate : min, maxCoordinate : max, nodeClass : nodeClass)
    }
    unsafe fn graphWithBufferRadius_minCoordinate_maxCoordinate_(
        bufferRadius: f32,
        min: vector_float2,
        max: vector_float2,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"GKMeshGraph").unwrap(), graphWithBufferRadius : bufferRadius, minCoordinate : min, maxCoordinate : max)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct GKMinmaxStrategist(pub id);
impl std::ops::Deref for GKMinmaxStrategist {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for GKMinmaxStrategist {}
impl GKMinmaxStrategist {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"GKMinmaxStrategist").unwrap(), alloc) })
    }
}
impl PGKStrategist for GKMinmaxStrategist {}
impl INSObject for GKMinmaxStrategist {}
impl PNSObject for GKMinmaxStrategist {}
impl std::convert::TryFrom<NSObject> for GKMinmaxStrategist {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<GKMinmaxStrategist, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"GKMinmaxStrategist").unwrap()) };
        if is_kind_of {
            Ok(GKMinmaxStrategist(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to GKMinmaxStrategist")
        }
    }
}
impl IGKMinmaxStrategist for GKMinmaxStrategist {}
pub trait IGKMinmaxStrategist: Sized + std::ops::Deref {
    unsafe fn bestMoveForPlayer_(&self, player: *mut u64) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, bestMoveForPlayer : player)
    }
    unsafe fn randomMoveForPlayer_fromNumberOfBestMoves_(
        &self,
        player: *mut u64,
        numMovesToConsider: NSInteger,
    ) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, randomMoveForPlayer : player, fromNumberOfBestMoves : numMovesToConsider)
    }
    unsafe fn maxLookAheadDepth(&self) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, maxLookAheadDepth)
    }
    unsafe fn setMaxLookAheadDepth_(&self, maxLookAheadDepth: NSInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMaxLookAheadDepth : maxLookAheadDepth)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct GKMonteCarloStrategist(pub id);
impl std::ops::Deref for GKMonteCarloStrategist {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for GKMonteCarloStrategist {}
impl GKMonteCarloStrategist {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"GKMonteCarloStrategist").unwrap(), alloc) })
    }
}
impl PGKStrategist for GKMonteCarloStrategist {}
impl INSObject for GKMonteCarloStrategist {}
impl PNSObject for GKMonteCarloStrategist {}
impl std::convert::TryFrom<NSObject> for GKMonteCarloStrategist {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<GKMonteCarloStrategist, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"GKMonteCarloStrategist").unwrap()) };
        if is_kind_of {
            Ok(GKMonteCarloStrategist(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to GKMonteCarloStrategist")
        }
    }
}
impl IGKMonteCarloStrategist for GKMonteCarloStrategist {}
pub trait IGKMonteCarloStrategist: Sized + std::ops::Deref {
    unsafe fn budget(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, budget)
    }
    unsafe fn setBudget_(&self, budget: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setBudget : budget)
    }
    unsafe fn explorationParameter(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, explorationParameter)
    }
    unsafe fn setExplorationParameter_(&self, explorationParameter: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setExplorationParameter : explorationParameter)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct GKDecisionNode(pub id);
impl std::ops::Deref for GKDecisionNode {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for GKDecisionNode {}
impl GKDecisionNode {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"GKDecisionNode").unwrap(), alloc) })
    }
}
impl INSObject for GKDecisionNode {}
impl PNSObject for GKDecisionNode {}
impl std::convert::TryFrom<NSObject> for GKDecisionNode {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<GKDecisionNode, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"GKDecisionNode").unwrap()) };
        if is_kind_of {
            Ok(GKDecisionNode(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to GKDecisionNode")
        }
    }
}
impl IGKDecisionNode for GKDecisionNode {}
pub trait IGKDecisionNode: Sized + std::ops::Deref {
    unsafe fn createBranchWithValue_attribute_(
        &self,
        value: NSNumber,
        attribute: *mut u64,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, createBranchWithValue : value, attribute : attribute)
    }
    unsafe fn createBranchWithPredicate_attribute_(
        &self,
        predicate: NSPredicate,
        attribute: *mut u64,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, createBranchWithPredicate : predicate, attribute : attribute)
    }
    unsafe fn createBranchWithWeight_attribute_(
        &self,
        weight: NSInteger,
        attribute: *mut u64,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, createBranchWithWeight : weight, attribute : attribute)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct GKDecisionTree(pub id);
impl std::ops::Deref for GKDecisionTree {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for GKDecisionTree {}
impl GKDecisionTree {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"GKDecisionTree").unwrap(), alloc) })
    }
}
impl PNSSecureCoding for GKDecisionTree {}
impl INSObject for GKDecisionTree {}
impl PNSObject for GKDecisionTree {}
impl std::convert::TryFrom<NSObject> for GKDecisionTree {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<GKDecisionTree, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"GKDecisionTree").unwrap()) };
        if is_kind_of {
            Ok(GKDecisionTree(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to GKDecisionTree")
        }
    }
}
impl IGKDecisionTree for GKDecisionTree {}
pub trait IGKDecisionTree: Sized + std::ops::Deref {
    unsafe fn initWithAttribute_(&self, attribute: *mut u64) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithAttribute : attribute)
    }
    unsafe fn initWithExamples_actions_attributes_(
        &self,
        examples: NSArray,
        actions: NSArray,
        attributes: NSArray,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithExamples : examples, actions : actions, attributes : attributes)
    }
    unsafe fn initWithURL_error_(&self, url: NSURL, error: NSError) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithURL : url, error : error)
    }
    unsafe fn exportToURL_error_(&self, url: NSURL, error: NSError) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, exportToURL : url, error : error)
    }
    unsafe fn findActionForAnswers_(&self, answers: NSDictionary) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, findActionForAnswers : answers)
    }
    unsafe fn rootNode(&self) -> GKDecisionNode
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, rootNode)
    }
    unsafe fn randomSource(&self) -> GKRandomSource
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, randomSource)
    }
    unsafe fn setRandomSource_(&self, randomSource: GKRandomSource)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRandomSource : randomSource)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct GKNoiseSource(pub id);
impl std::ops::Deref for GKNoiseSource {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for GKNoiseSource {}
impl GKNoiseSource {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"GKNoiseSource").unwrap(), alloc) })
    }
}
impl INSObject for GKNoiseSource {}
impl PNSObject for GKNoiseSource {}
impl std::convert::TryFrom<NSObject> for GKNoiseSource {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<GKNoiseSource, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"GKNoiseSource").unwrap()) };
        if is_kind_of {
            Ok(GKNoiseSource(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to GKNoiseSource")
        }
    }
}
impl IGKNoiseSource for GKNoiseSource {}
pub trait IGKNoiseSource: Sized + std::ops::Deref {}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct GKCoherentNoiseSource(pub id);
impl std::ops::Deref for GKCoherentNoiseSource {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for GKCoherentNoiseSource {}
impl GKCoherentNoiseSource {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"GKCoherentNoiseSource").unwrap(), alloc) })
    }
}
impl IGKNoiseSource for GKCoherentNoiseSource {}
impl From<GKCoherentNoiseSource> for GKNoiseSource {
    fn from(child: GKCoherentNoiseSource) -> GKNoiseSource {
        GKNoiseSource(child.0)
    }
}
impl std::convert::TryFrom<GKNoiseSource> for GKCoherentNoiseSource {
    type Error = &'static str;
    fn try_from(parent: GKNoiseSource) -> Result<GKCoherentNoiseSource, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"GKCoherentNoiseSource").unwrap()) };
        if is_kind_of {
            Ok(GKCoherentNoiseSource(parent.0))
        } else {
            Err("This GKNoiseSource cannot be downcasted to GKCoherentNoiseSource")
        }
    }
}
impl INSObject for GKCoherentNoiseSource {}
impl PNSObject for GKCoherentNoiseSource {}
impl IGKCoherentNoiseSource for GKCoherentNoiseSource {}
pub trait IGKCoherentNoiseSource: Sized + std::ops::Deref {
    unsafe fn frequency(&self) -> f64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, frequency)
    }
    unsafe fn setFrequency_(&self, frequency: f64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setFrequency : frequency)
    }
    unsafe fn octaveCount(&self) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, octaveCount)
    }
    unsafe fn setOctaveCount_(&self, octaveCount: NSInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setOctaveCount : octaveCount)
    }
    unsafe fn lacunarity(&self) -> f64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, lacunarity)
    }
    unsafe fn setLacunarity_(&self, lacunarity: f64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setLacunarity : lacunarity)
    }
    unsafe fn seed(&self) -> i32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, seed)
    }
    unsafe fn setSeed_(&self, seed: i32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSeed : seed)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct GKPerlinNoiseSource(pub id);
impl std::ops::Deref for GKPerlinNoiseSource {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for GKPerlinNoiseSource {}
impl GKPerlinNoiseSource {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"GKPerlinNoiseSource").unwrap(), alloc) })
    }
}
impl IGKCoherentNoiseSource for GKPerlinNoiseSource {}
impl From<GKPerlinNoiseSource> for GKCoherentNoiseSource {
    fn from(child: GKPerlinNoiseSource) -> GKCoherentNoiseSource {
        GKCoherentNoiseSource(child.0)
    }
}
impl std::convert::TryFrom<GKCoherentNoiseSource> for GKPerlinNoiseSource {
    type Error = &'static str;
    fn try_from(parent: GKCoherentNoiseSource) -> Result<GKPerlinNoiseSource, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"GKPerlinNoiseSource").unwrap()) };
        if is_kind_of {
            Ok(GKPerlinNoiseSource(parent.0))
        } else {
            Err("This GKCoherentNoiseSource cannot be downcasted to GKPerlinNoiseSource")
        }
    }
}
impl IGKNoiseSource for GKPerlinNoiseSource {}
impl INSObject for GKPerlinNoiseSource {}
impl PNSObject for GKPerlinNoiseSource {}
impl IGKPerlinNoiseSource for GKPerlinNoiseSource {}
pub trait IGKPerlinNoiseSource: Sized + std::ops::Deref {
    unsafe fn initWithFrequency_octaveCount_persistence_lacunarity_seed_(
        &self,
        frequency: f64,
        octaveCount: NSInteger,
        persistence: f64,
        lacunarity: f64,
        seed: i32,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithFrequency : frequency, octaveCount : octaveCount, persistence : persistence, lacunarity : lacunarity, seed : seed)
    }
    unsafe fn persistence(&self) -> f64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, persistence)
    }
    unsafe fn setPersistence_(&self, persistence: f64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPersistence : persistence)
    }
    unsafe fn perlinNoiseSourceWithFrequency_octaveCount_persistence_lacunarity_seed_(
        frequency: f64,
        octaveCount: NSInteger,
        persistence: f64,
        lacunarity: f64,
        seed: i32,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"GKPerlinNoiseSource").unwrap(), perlinNoiseSourceWithFrequency : frequency, octaveCount : octaveCount, persistence : persistence, lacunarity : lacunarity, seed : seed)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct GKBillowNoiseSource(pub id);
impl std::ops::Deref for GKBillowNoiseSource {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for GKBillowNoiseSource {}
impl GKBillowNoiseSource {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"GKBillowNoiseSource").unwrap(), alloc) })
    }
}
impl IGKCoherentNoiseSource for GKBillowNoiseSource {}
impl std::convert::TryFrom<GKCoherentNoiseSource> for GKBillowNoiseSource {
    type Error = &'static str;
    fn try_from(parent: GKCoherentNoiseSource) -> Result<GKBillowNoiseSource, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"GKBillowNoiseSource").unwrap()) };
        if is_kind_of {
            Ok(GKBillowNoiseSource(parent.0))
        } else {
            Err("This GKCoherentNoiseSource cannot be downcasted to GKBillowNoiseSource")
        }
    }
}
impl IGKNoiseSource for GKBillowNoiseSource {}
impl INSObject for GKBillowNoiseSource {}
impl PNSObject for GKBillowNoiseSource {}
impl IGKBillowNoiseSource for GKBillowNoiseSource {}
pub trait IGKBillowNoiseSource: Sized + std::ops::Deref {
    unsafe fn initWithFrequency_octaveCount_persistence_lacunarity_seed_(
        &self,
        frequency: f64,
        octaveCount: NSInteger,
        persistence: f64,
        lacunarity: f64,
        seed: i32,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithFrequency : frequency, octaveCount : octaveCount, persistence : persistence, lacunarity : lacunarity, seed : seed)
    }
    unsafe fn persistence(&self) -> f64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, persistence)
    }
    unsafe fn setPersistence_(&self, persistence: f64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPersistence : persistence)
    }
    unsafe fn billowNoiseSourceWithFrequency_octaveCount_persistence_lacunarity_seed_(
        frequency: f64,
        octaveCount: NSInteger,
        persistence: f64,
        lacunarity: f64,
        seed: i32,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"GKBillowNoiseSource").unwrap(), billowNoiseSourceWithFrequency : frequency, octaveCount : octaveCount, persistence : persistence, lacunarity : lacunarity, seed : seed)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct GKRidgedNoiseSource(pub id);
impl std::ops::Deref for GKRidgedNoiseSource {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for GKRidgedNoiseSource {}
impl GKRidgedNoiseSource {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"GKRidgedNoiseSource").unwrap(), alloc) })
    }
}
impl IGKCoherentNoiseSource for GKRidgedNoiseSource {}
impl std::convert::TryFrom<GKCoherentNoiseSource> for GKRidgedNoiseSource {
    type Error = &'static str;
    fn try_from(parent: GKCoherentNoiseSource) -> Result<GKRidgedNoiseSource, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"GKRidgedNoiseSource").unwrap()) };
        if is_kind_of {
            Ok(GKRidgedNoiseSource(parent.0))
        } else {
            Err("This GKCoherentNoiseSource cannot be downcasted to GKRidgedNoiseSource")
        }
    }
}
impl IGKNoiseSource for GKRidgedNoiseSource {}
impl INSObject for GKRidgedNoiseSource {}
impl PNSObject for GKRidgedNoiseSource {}
impl IGKRidgedNoiseSource for GKRidgedNoiseSource {}
pub trait IGKRidgedNoiseSource: Sized + std::ops::Deref {
    unsafe fn initWithFrequency_octaveCount_lacunarity_seed_(
        &self,
        frequency: f64,
        octaveCount: NSInteger,
        lacunarity: f64,
        seed: i32,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithFrequency : frequency, octaveCount : octaveCount, lacunarity : lacunarity, seed : seed)
    }
    unsafe fn ridgedNoiseSourceWithFrequency_octaveCount_lacunarity_seed_(
        frequency: f64,
        octaveCount: NSInteger,
        lacunarity: f64,
        seed: i32,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"GKRidgedNoiseSource").unwrap(), ridgedNoiseSourceWithFrequency : frequency, octaveCount : octaveCount, lacunarity : lacunarity, seed : seed)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct GKVoronoiNoiseSource(pub id);
impl std::ops::Deref for GKVoronoiNoiseSource {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for GKVoronoiNoiseSource {}
impl GKVoronoiNoiseSource {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"GKVoronoiNoiseSource").unwrap(), alloc) })
    }
}
impl IGKNoiseSource for GKVoronoiNoiseSource {}
impl std::convert::TryFrom<GKNoiseSource> for GKVoronoiNoiseSource {
    type Error = &'static str;
    fn try_from(parent: GKNoiseSource) -> Result<GKVoronoiNoiseSource, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"GKVoronoiNoiseSource").unwrap()) };
        if is_kind_of {
            Ok(GKVoronoiNoiseSource(parent.0))
        } else {
            Err("This GKNoiseSource cannot be downcasted to GKVoronoiNoiseSource")
        }
    }
}
impl INSObject for GKVoronoiNoiseSource {}
impl PNSObject for GKVoronoiNoiseSource {}
impl IGKVoronoiNoiseSource for GKVoronoiNoiseSource {}
pub trait IGKVoronoiNoiseSource: Sized + std::ops::Deref {
    unsafe fn initWithFrequency_displacement_distanceEnabled_seed_(
        &self,
        frequency: f64,
        displacement: f64,
        distanceEnabled: BOOL,
        seed: i32,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithFrequency : frequency, displacement : displacement, distanceEnabled : distanceEnabled, seed : seed)
    }
    unsafe fn frequency(&self) -> f64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, frequency)
    }
    unsafe fn setFrequency_(&self, frequency: f64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setFrequency : frequency)
    }
    unsafe fn displacement(&self) -> f64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, displacement)
    }
    unsafe fn setDisplacement_(&self, displacement: f64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDisplacement : displacement)
    }
    unsafe fn isDistanceEnabled(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isDistanceEnabled)
    }
    unsafe fn setDistanceEnabled_(&self, distanceEnabled: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDistanceEnabled : distanceEnabled)
    }
    unsafe fn seed(&self) -> i32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, seed)
    }
    unsafe fn setSeed_(&self, seed: i32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSeed : seed)
    }
    unsafe fn voronoiNoiseWithFrequency_displacement_distanceEnabled_seed_(
        frequency: f64,
        displacement: f64,
        distanceEnabled: BOOL,
        seed: i32,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"GKVoronoiNoiseSource").unwrap(), voronoiNoiseWithFrequency : frequency, displacement : displacement, distanceEnabled : distanceEnabled, seed : seed)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct GKConstantNoiseSource(pub id);
impl std::ops::Deref for GKConstantNoiseSource {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for GKConstantNoiseSource {}
impl GKConstantNoiseSource {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"GKConstantNoiseSource").unwrap(), alloc) })
    }
}
impl IGKNoiseSource for GKConstantNoiseSource {}
impl std::convert::TryFrom<GKNoiseSource> for GKConstantNoiseSource {
    type Error = &'static str;
    fn try_from(parent: GKNoiseSource) -> Result<GKConstantNoiseSource, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"GKConstantNoiseSource").unwrap()) };
        if is_kind_of {
            Ok(GKConstantNoiseSource(parent.0))
        } else {
            Err("This GKNoiseSource cannot be downcasted to GKConstantNoiseSource")
        }
    }
}
impl INSObject for GKConstantNoiseSource {}
impl PNSObject for GKConstantNoiseSource {}
impl IGKConstantNoiseSource for GKConstantNoiseSource {}
pub trait IGKConstantNoiseSource: Sized + std::ops::Deref {
    unsafe fn initWithValue_(&self, value: f64) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithValue : value)
    }
    unsafe fn value(&self) -> f64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, value)
    }
    unsafe fn setValue_(&self, value: f64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setValue : value)
    }
    unsafe fn constantNoiseWithValue_(value: f64) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"GKConstantNoiseSource").unwrap(), constantNoiseWithValue : value)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct GKCylindersNoiseSource(pub id);
impl std::ops::Deref for GKCylindersNoiseSource {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for GKCylindersNoiseSource {}
impl GKCylindersNoiseSource {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"GKCylindersNoiseSource").unwrap(), alloc) })
    }
}
impl IGKNoiseSource for GKCylindersNoiseSource {}
impl std::convert::TryFrom<GKNoiseSource> for GKCylindersNoiseSource {
    type Error = &'static str;
    fn try_from(parent: GKNoiseSource) -> Result<GKCylindersNoiseSource, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"GKCylindersNoiseSource").unwrap()) };
        if is_kind_of {
            Ok(GKCylindersNoiseSource(parent.0))
        } else {
            Err("This GKNoiseSource cannot be downcasted to GKCylindersNoiseSource")
        }
    }
}
impl INSObject for GKCylindersNoiseSource {}
impl PNSObject for GKCylindersNoiseSource {}
impl IGKCylindersNoiseSource for GKCylindersNoiseSource {}
pub trait IGKCylindersNoiseSource: Sized + std::ops::Deref {
    unsafe fn initWithFrequency_(&self, frequency: f64) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithFrequency : frequency)
    }
    unsafe fn frequency(&self) -> f64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, frequency)
    }
    unsafe fn setFrequency_(&self, frequency: f64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setFrequency : frequency)
    }
    unsafe fn cylindersNoiseWithFrequency_(frequency: f64) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"GKCylindersNoiseSource").unwrap(), cylindersNoiseWithFrequency : frequency)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct GKSpheresNoiseSource(pub id);
impl std::ops::Deref for GKSpheresNoiseSource {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for GKSpheresNoiseSource {}
impl GKSpheresNoiseSource {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"GKSpheresNoiseSource").unwrap(), alloc) })
    }
}
impl IGKNoiseSource for GKSpheresNoiseSource {}
impl std::convert::TryFrom<GKNoiseSource> for GKSpheresNoiseSource {
    type Error = &'static str;
    fn try_from(parent: GKNoiseSource) -> Result<GKSpheresNoiseSource, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"GKSpheresNoiseSource").unwrap()) };
        if is_kind_of {
            Ok(GKSpheresNoiseSource(parent.0))
        } else {
            Err("This GKNoiseSource cannot be downcasted to GKSpheresNoiseSource")
        }
    }
}
impl INSObject for GKSpheresNoiseSource {}
impl PNSObject for GKSpheresNoiseSource {}
impl IGKSpheresNoiseSource for GKSpheresNoiseSource {}
pub trait IGKSpheresNoiseSource: Sized + std::ops::Deref {
    unsafe fn initWithFrequency_(&self, frequency: f64) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithFrequency : frequency)
    }
    unsafe fn frequency(&self) -> f64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, frequency)
    }
    unsafe fn setFrequency_(&self, frequency: f64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setFrequency : frequency)
    }
    unsafe fn spheresNoiseWithFrequency_(frequency: f64) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"GKSpheresNoiseSource").unwrap(), spheresNoiseWithFrequency : frequency)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct GKCheckerboardNoiseSource(pub id);
impl std::ops::Deref for GKCheckerboardNoiseSource {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for GKCheckerboardNoiseSource {}
impl GKCheckerboardNoiseSource {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"GKCheckerboardNoiseSource").unwrap(), alloc) })
    }
}
impl IGKNoiseSource for GKCheckerboardNoiseSource {}
impl std::convert::TryFrom<GKNoiseSource> for GKCheckerboardNoiseSource {
    type Error = &'static str;
    fn try_from(parent: GKNoiseSource) -> Result<GKCheckerboardNoiseSource, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"GKCheckerboardNoiseSource").unwrap()) };
        if is_kind_of {
            Ok(GKCheckerboardNoiseSource(parent.0))
        } else {
            Err("This GKNoiseSource cannot be downcasted to GKCheckerboardNoiseSource")
        }
    }
}
impl INSObject for GKCheckerboardNoiseSource {}
impl PNSObject for GKCheckerboardNoiseSource {}
impl IGKCheckerboardNoiseSource for GKCheckerboardNoiseSource {}
pub trait IGKCheckerboardNoiseSource: Sized + std::ops::Deref {
    unsafe fn initWithSquareSize_(&self, squareSize: f64) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithSquareSize : squareSize)
    }
    unsafe fn squareSize(&self) -> f64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, squareSize)
    }
    unsafe fn setSquareSize_(&self, squareSize: f64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSquareSize : squareSize)
    }
    unsafe fn checkerboardNoiseWithSquareSize_(squareSize: f64) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"GKCheckerboardNoiseSource").unwrap(), checkerboardNoiseWithSquareSize : squareSize)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct GKNoise(pub id);
impl std::ops::Deref for GKNoise {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for GKNoise {}
impl GKNoise {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"GKNoise").unwrap(), alloc) })
    }
}
impl INSObject for GKNoise {}
impl PNSObject for GKNoise {}
impl std::convert::TryFrom<NSObject> for GKNoise {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<GKNoise, Self::Error> {
        let is_kind_of: bool = unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"GKNoise").unwrap()) };
        if is_kind_of {
            Ok(GKNoise(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to GKNoise")
        }
    }
}
impl IGKNoise for GKNoise {}
pub trait IGKNoise: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn initWithNoiseSource_(&self, noiseSource: GKNoiseSource) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithNoiseSource : noiseSource)
    }
    unsafe fn initWithNoiseSource_gradientColors_(
        &self,
        noiseSource: GKNoiseSource,
        gradientColors: NSDictionary,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithNoiseSource : noiseSource, gradientColors : gradientColors)
    }
    unsafe fn valueAtPosition_(&self, position: vector_float2) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, valueAtPosition : position)
    }
    unsafe fn applyAbsoluteValue(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, applyAbsoluteValue)
    }
    unsafe fn clampWithLowerBound_upperBound_(&self, lowerBound: f64, upperBound: f64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, clampWithLowerBound : lowerBound, upperBound : upperBound)
    }
    unsafe fn raiseToPower_(&self, power: f64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, raiseToPower : power)
    }
    unsafe fn invert(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, invert)
    }
    unsafe fn applyTurbulenceWithFrequency_power_roughness_seed_(
        &self,
        frequency: f64,
        power: f64,
        roughness: ::std::os::raw::c_int,
        seed: i32,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, applyTurbulenceWithFrequency : frequency, power : power, roughness : roughness, seed : seed)
    }
    unsafe fn remapValuesToCurveWithControlPoints_(&self, controlPoints: NSDictionary)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, remapValuesToCurveWithControlPoints : controlPoints)
    }
    unsafe fn remapValuesToTerracesWithPeaks_terracesInverted_(
        &self,
        peakInputValues: NSArray,
        inverted: BOOL,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, remapValuesToTerracesWithPeaks : peakInputValues, terracesInverted : inverted)
    }
    unsafe fn moveBy_(&self, delta: vector_double3)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, moveBy : delta)
    }
    unsafe fn scaleBy_(&self, factor: vector_double3)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, scaleBy : factor)
    }
    unsafe fn rotateBy_(&self, radians: vector_double3)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, rotateBy : radians)
    }
    unsafe fn addWithNoise_(&self, noise: GKNoise)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, addWithNoise : noise)
    }
    unsafe fn multiplyWithNoise_(&self, noise: GKNoise)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, multiplyWithNoise : noise)
    }
    unsafe fn minimumWithNoise_(&self, noise: GKNoise)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, minimumWithNoise : noise)
    }
    unsafe fn maximumWithNoise_(&self, noise: GKNoise)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, maximumWithNoise : noise)
    }
    unsafe fn raiseToPowerWithNoise_(&self, noise: GKNoise)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, raiseToPowerWithNoise : noise)
    }
    unsafe fn displaceXWithNoise_yWithNoise_zWithNoise_(
        &self,
        xDisplacementNoise: GKNoise,
        yDisplacementNoise: GKNoise,
        zDisplacementNoise: GKNoise,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, displaceXWithNoise : xDisplacementNoise, yWithNoise : yDisplacementNoise, zWithNoise : zDisplacementNoise)
    }
    unsafe fn gradientColors(&self) -> NSDictionary
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, gradientColors)
    }
    unsafe fn setGradientColors_(&self, gradientColors: NSDictionary)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setGradientColors : gradientColors)
    }
    unsafe fn noiseWithNoiseSource_(noiseSource: GKNoiseSource) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"GKNoise").unwrap(), noiseWithNoiseSource : noiseSource)
    }
    unsafe fn noiseWithNoiseSource_gradientColors_(
        noiseSource: GKNoiseSource,
        gradientColors: NSDictionary,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"GKNoise").unwrap(), noiseWithNoiseSource : noiseSource, gradientColors : gradientColors)
    }
    unsafe fn noiseWithComponentNoises_selectionNoise_(
        noises: NSArray,
        selectionNoise: GKNoise,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"GKNoise").unwrap(), noiseWithComponentNoises : noises, selectionNoise : selectionNoise)
    }
    unsafe fn noiseWithComponentNoises_selectionNoise_componentBoundaries_boundaryBlendDistances_(
        noises: NSArray,
        selectionNoise: GKNoise,
        componentBoundaries: NSArray,
        blendDistances: NSArray,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"GKNoise").unwrap(), noiseWithComponentNoises : noises, selectionNoise : selectionNoise, componentBoundaries : componentBoundaries, boundaryBlendDistances : blendDistances)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct GKNoiseMap(pub id);
impl std::ops::Deref for GKNoiseMap {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for GKNoiseMap {}
impl GKNoiseMap {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"GKNoiseMap").unwrap(), alloc) })
    }
}
impl INSObject for GKNoiseMap {}
impl PNSObject for GKNoiseMap {}
impl std::convert::TryFrom<NSObject> for GKNoiseMap {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<GKNoiseMap, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"GKNoiseMap").unwrap()) };
        if is_kind_of {
            Ok(GKNoiseMap(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to GKNoiseMap")
        }
    }
}
impl IGKNoiseMap for GKNoiseMap {}
pub trait IGKNoiseMap: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn initWithNoise_(&self, noise: GKNoise) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithNoise : noise)
    }
    unsafe fn initWithNoise_size_origin_sampleCount_seamless_(
        &self,
        noise: GKNoise,
        size: vector_double2,
        origin: vector_double2,
        sampleCount: vector_int2,
        seamless: BOOL,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithNoise : noise, size : size, origin : origin, sampleCount : sampleCount, seamless : seamless)
    }
    unsafe fn valueAtPosition_(&self, position: vector_int2) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, valueAtPosition : position)
    }
    unsafe fn interpolatedValueAtPosition_(&self, position: vector_float2) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, interpolatedValueAtPosition : position)
    }
    unsafe fn setValue_atPosition_(&self, value: f32, position: vector_int2)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setValue : value, atPosition : position)
    }
    unsafe fn size(&self) -> vector_double2
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, size)
    }
    unsafe fn origin(&self) -> vector_double2
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, origin)
    }
    unsafe fn sampleCount(&self) -> vector_int2
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, sampleCount)
    }
    unsafe fn isSeamless(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isSeamless)
    }
    unsafe fn noiseMapWithNoise_(noise: GKNoise) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"GKNoiseMap").unwrap(), noiseMapWithNoise : noise)
    }
    unsafe fn noiseMapWithNoise_size_origin_sampleCount_seamless_(
        noise: GKNoise,
        size: vector_double2,
        origin: vector_double2,
        sampleCount: vector_int2,
        seamless: BOOL,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"GKNoiseMap").unwrap(), noiseMapWithNoise : noise, size : size, origin : origin, sampleCount : sampleCount, seamless : seamless)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct GKOctreeNode(pub id);
impl std::ops::Deref for GKOctreeNode {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for GKOctreeNode {}
impl GKOctreeNode {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"GKOctreeNode").unwrap(), alloc) })
    }
}
impl INSObject for GKOctreeNode {}
impl PNSObject for GKOctreeNode {}
impl std::convert::TryFrom<NSObject> for GKOctreeNode {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<GKOctreeNode, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"GKOctreeNode").unwrap()) };
        if is_kind_of {
            Ok(GKOctreeNode(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to GKOctreeNode")
        }
    }
}
impl IGKOctreeNode for GKOctreeNode {}
pub trait IGKOctreeNode: Sized + std::ops::Deref {
    unsafe fn box_(&self) -> GKBox
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, box)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct GKOctree(pub id);
impl std::ops::Deref for GKOctree {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for GKOctree {}
impl GKOctree {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"GKOctree").unwrap(), alloc) })
    }
}
impl INSObject for GKOctree {}
impl PNSObject for GKOctree {}
impl std::convert::TryFrom<NSObject> for GKOctree {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<GKOctree, Self::Error> {
        let is_kind_of: bool = unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"GKOctree").unwrap()) };
        if is_kind_of {
            Ok(GKOctree(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to GKOctree")
        }
    }
}
impl<ElementType: 'static> IGKOctree<ElementType> for GKOctree {}
pub trait IGKOctree<ElementType: 'static>: Sized + std::ops::Deref {
    unsafe fn initWithBoundingBox_minimumCellSize_(
        &self,
        box_: GKBox,
        minCellSize: f32,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithBoundingBox : box_, minimumCellSize : minCellSize)
    }
    unsafe fn addElement_withPoint_(&self, element: NSObject, point: vector_float3) -> GKOctreeNode
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, addElement : element, withPoint : point)
    }
    unsafe fn addElement_withBox_(&self, element: NSObject, box_: GKBox) -> GKOctreeNode
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, addElement : element, withBox : box_)
    }
    unsafe fn elementsAtPoint_(&self, point: vector_float3) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, elementsAtPoint : point)
    }
    unsafe fn elementsInBox_(&self, box_: GKBox) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, elementsInBox : box_)
    }
    unsafe fn removeElement_(&self, element: NSObject) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, removeElement : element)
    }
    unsafe fn removeElement_withNode_(&self, element: NSObject, node: GKOctreeNode) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, removeElement : element, withNode : node)
    }
    unsafe fn octreeWithBoundingBox_minimumCellSize_(box_: GKBox, minCellSize: f32) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"GKOctree").unwrap(), octreeWithBoundingBox : box_, minimumCellSize : minCellSize)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct GKPath(pub id);
impl std::ops::Deref for GKPath {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for GKPath {}
impl GKPath {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"GKPath").unwrap(), alloc) })
    }
}
impl INSObject for GKPath {}
impl PNSObject for GKPath {}
impl std::convert::TryFrom<NSObject> for GKPath {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<GKPath, Self::Error> {
        let is_kind_of: bool = unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"GKPath").unwrap()) };
        if is_kind_of {
            Ok(GKPath(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to GKPath")
        }
    }
}
impl IGKPath for GKPath {}
pub trait IGKPath: Sized + std::ops::Deref {
    unsafe fn initWithPoints_count_radius_cyclical_(
        &self,
        points: *mut vector_float2,
        count: usize,
        radius: f32,
        cyclical: BOOL,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithPoints : points, count : count, radius : radius, cyclical : cyclical)
    }
    unsafe fn initWithFloat3Points_count_radius_cyclical_(
        &self,
        points: *mut vector_float3,
        count: usize,
        radius: f32,
        cyclical: BOOL,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithFloat3Points : points, count : count, radius : radius, cyclical : cyclical)
    }
    unsafe fn initWithGraphNodes_radius_(&self, graphNodes: NSArray, radius: f32) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithGraphNodes : graphNodes, radius : radius)
    }
    unsafe fn pointAtIndex_(&self, index: NSUInteger) -> vector_float2
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, pointAtIndex : index)
    }
    unsafe fn float2AtIndex_(&self, index: NSUInteger) -> vector_float2
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, float2AtIndex : index)
    }
    unsafe fn float3AtIndex_(&self, index: NSUInteger) -> vector_float3
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, float3AtIndex : index)
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
    unsafe fn numPoints(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, numPoints)
    }
    unsafe fn isCyclical(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isCyclical)
    }
    unsafe fn setCyclical_(&self, cyclical: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCyclical : cyclical)
    }
    unsafe fn pathWithPoints_count_radius_cyclical_(
        points: *mut vector_float2,
        count: usize,
        radius: f32,
        cyclical: BOOL,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"GKPath").unwrap(), pathWithPoints : points, count : count, radius : radius, cyclical : cyclical)
    }
    unsafe fn pathWithFloat3Points_count_radius_cyclical_(
        points: *mut vector_float3,
        count: usize,
        radius: f32,
        cyclical: BOOL,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"GKPath").unwrap(), pathWithFloat3Points : points, count : count, radius : radius, cyclical : cyclical)
    }
    unsafe fn pathWithGraphNodes_radius_(graphNodes: NSArray, radius: f32) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"GKPath").unwrap(), pathWithGraphNodes : graphNodes, radius : radius)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct GKQuadtreeNode(pub id);
impl std::ops::Deref for GKQuadtreeNode {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for GKQuadtreeNode {}
impl GKQuadtreeNode {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"GKQuadtreeNode").unwrap(), alloc) })
    }
}
impl INSObject for GKQuadtreeNode {}
impl PNSObject for GKQuadtreeNode {}
impl std::convert::TryFrom<NSObject> for GKQuadtreeNode {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<GKQuadtreeNode, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"GKQuadtreeNode").unwrap()) };
        if is_kind_of {
            Ok(GKQuadtreeNode(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to GKQuadtreeNode")
        }
    }
}
impl IGKQuadtreeNode for GKQuadtreeNode {}
pub trait IGKQuadtreeNode: Sized + std::ops::Deref {
    unsafe fn quad(&self) -> GKQuad
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, quad)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct GKQuadtree(pub id);
impl std::ops::Deref for GKQuadtree {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for GKQuadtree {}
impl GKQuadtree {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"GKQuadtree").unwrap(), alloc) })
    }
}
impl INSObject for GKQuadtree {}
impl PNSObject for GKQuadtree {}
impl std::convert::TryFrom<NSObject> for GKQuadtree {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<GKQuadtree, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"GKQuadtree").unwrap()) };
        if is_kind_of {
            Ok(GKQuadtree(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to GKQuadtree")
        }
    }
}
impl<ElementType: 'static> IGKQuadtree<ElementType> for GKQuadtree {}
pub trait IGKQuadtree<ElementType: 'static>: Sized + std::ops::Deref {
    unsafe fn initWithBoundingQuad_minimumCellSize_(
        &self,
        quad: GKQuad,
        minCellSize: f32,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithBoundingQuad : quad, minimumCellSize : minCellSize)
    }
    unsafe fn addElement_withPoint_(
        &self,
        element: NSObject,
        point: vector_float2,
    ) -> GKQuadtreeNode
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, addElement : element, withPoint : point)
    }
    unsafe fn addElement_withQuad_(&self, element: NSObject, quad: GKQuad) -> GKQuadtreeNode
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, addElement : element, withQuad : quad)
    }
    unsafe fn elementsAtPoint_(&self, point: vector_float2) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, elementsAtPoint : point)
    }
    unsafe fn elementsInQuad_(&self, quad: GKQuad) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, elementsInQuad : quad)
    }
    unsafe fn removeElement_(&self, element: NSObject) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, removeElement : element)
    }
    unsafe fn removeElement_withNode_(&self, data: NSObject, node: GKQuadtreeNode) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, removeElement : data, withNode : node)
    }
    unsafe fn quadtreeWithBoundingQuad_minimumCellSize_(
        quad: GKQuad,
        minCellSize: f32,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"GKQuadtree").unwrap(), quadtreeWithBoundingQuad : quad, minimumCellSize : minCellSize)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct GKRandomDistribution(pub id);
impl std::ops::Deref for GKRandomDistribution {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for GKRandomDistribution {}
impl GKRandomDistribution {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"GKRandomDistribution").unwrap(), alloc) })
    }
}
impl PGKRandom for GKRandomDistribution {}
impl INSObject for GKRandomDistribution {}
impl PNSObject for GKRandomDistribution {}
impl std::convert::TryFrom<NSObject> for GKRandomDistribution {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<GKRandomDistribution, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"GKRandomDistribution").unwrap()) };
        if is_kind_of {
            Ok(GKRandomDistribution(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to GKRandomDistribution")
        }
    }
}
impl IGKRandomDistribution for GKRandomDistribution {}
pub trait IGKRandomDistribution: Sized + std::ops::Deref {
    unsafe fn initWithRandomSource_lowestValue_highestValue_(
        &self,
        source: *mut u64,
        lowestInclusive: NSInteger,
        highestInclusive: NSInteger,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithRandomSource : source, lowestValue : lowestInclusive, highestValue : highestInclusive)
    }
    unsafe fn nextInt(&self) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, nextInt)
    }
    unsafe fn nextIntWithUpperBound_(&self, upperBound: NSUInteger) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, nextIntWithUpperBound : upperBound)
    }
    unsafe fn nextUniform(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, nextUniform)
    }
    unsafe fn nextBool(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, nextBool)
    }
    unsafe fn lowestValue(&self) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, lowestValue)
    }
    unsafe fn highestValue(&self) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, highestValue)
    }
    unsafe fn numberOfPossibleOutcomes(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, numberOfPossibleOutcomes)
    }
    unsafe fn distributionWithLowestValue_highestValue_(
        lowestInclusive: NSInteger,
        highestInclusive: NSInteger,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"GKRandomDistribution").unwrap(), distributionWithLowestValue : lowestInclusive, highestValue : highestInclusive)
    }
    unsafe fn distributionForDieWithSideCount_(sideCount: NSInteger) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"GKRandomDistribution").unwrap(), distributionForDieWithSideCount : sideCount)
    }
    unsafe fn d6() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"GKRandomDistribution").unwrap(), d6)
    }
    unsafe fn d20() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"GKRandomDistribution").unwrap(), d20)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct GKGaussianDistribution(pub id);
impl std::ops::Deref for GKGaussianDistribution {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for GKGaussianDistribution {}
impl GKGaussianDistribution {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"GKGaussianDistribution").unwrap(), alloc) })
    }
}
impl IGKRandomDistribution for GKGaussianDistribution {}
impl PGKRandom for GKGaussianDistribution {}
impl From<GKGaussianDistribution> for GKRandomDistribution {
    fn from(child: GKGaussianDistribution) -> GKRandomDistribution {
        GKRandomDistribution(child.0)
    }
}
impl std::convert::TryFrom<GKRandomDistribution> for GKGaussianDistribution {
    type Error = &'static str;
    fn try_from(parent: GKRandomDistribution) -> Result<GKGaussianDistribution, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"GKGaussianDistribution").unwrap()) };
        if is_kind_of {
            Ok(GKGaussianDistribution(parent.0))
        } else {
            Err("This GKRandomDistribution cannot be downcasted to GKGaussianDistribution")
        }
    }
}
impl INSObject for GKGaussianDistribution {}
impl PNSObject for GKGaussianDistribution {}
impl IGKGaussianDistribution for GKGaussianDistribution {}
pub trait IGKGaussianDistribution: Sized + std::ops::Deref {
    unsafe fn initWithRandomSource_lowestValue_highestValue_(
        &self,
        source: *mut u64,
        lowestInclusive: NSInteger,
        highestInclusive: NSInteger,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithRandomSource : source, lowestValue : lowestInclusive, highestValue : highestInclusive)
    }
    unsafe fn initWithRandomSource_mean_deviation_(
        &self,
        source: *mut u64,
        mean: f32,
        deviation: f32,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithRandomSource : source, mean : mean, deviation : deviation)
    }
    unsafe fn mean(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, mean)
    }
    unsafe fn deviation(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, deviation)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct GKShuffledDistribution(pub id);
impl std::ops::Deref for GKShuffledDistribution {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for GKShuffledDistribution {}
impl GKShuffledDistribution {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"GKShuffledDistribution").unwrap(), alloc) })
    }
}
impl IGKRandomDistribution for GKShuffledDistribution {}
impl PGKRandom for GKShuffledDistribution {}
impl std::convert::TryFrom<GKRandomDistribution> for GKShuffledDistribution {
    type Error = &'static str;
    fn try_from(parent: GKRandomDistribution) -> Result<GKShuffledDistribution, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"GKShuffledDistribution").unwrap()) };
        if is_kind_of {
            Ok(GKShuffledDistribution(parent.0))
        } else {
            Err("This GKRandomDistribution cannot be downcasted to GKShuffledDistribution")
        }
    }
}
impl INSObject for GKShuffledDistribution {}
impl PNSObject for GKShuffledDistribution {}
impl IGKShuffledDistribution for GKShuffledDistribution {}
pub trait IGKShuffledDistribution: Sized + std::ops::Deref {}
pub type GKRTreeSplitStrategy = NSInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct GKRTree(pub id);
impl std::ops::Deref for GKRTree {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for GKRTree {}
impl GKRTree {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"GKRTree").unwrap(), alloc) })
    }
}
impl INSObject for GKRTree {}
impl PNSObject for GKRTree {}
impl std::convert::TryFrom<NSObject> for GKRTree {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<GKRTree, Self::Error> {
        let is_kind_of: bool = unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"GKRTree").unwrap()) };
        if is_kind_of {
            Ok(GKRTree(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to GKRTree")
        }
    }
}
impl<ElementType: 'static> IGKRTree<ElementType> for GKRTree {}
pub trait IGKRTree<ElementType: 'static>: Sized + std::ops::Deref {
    unsafe fn initWithMaxNumberOfChildren_(&self, maxNumberOfChildren: NSUInteger) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithMaxNumberOfChildren : maxNumberOfChildren)
    }
    unsafe fn addElement_boundingRectMin_boundingRectMax_splitStrategy_(
        &self,
        element: NSObject,
        boundingRectMin: vector_float2,
        boundingRectMax: vector_float2,
        splitStrategy: GKRTreeSplitStrategy,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, addElement : element, boundingRectMin : boundingRectMin, boundingRectMax : boundingRectMax, splitStrategy : splitStrategy)
    }
    unsafe fn removeElement_boundingRectMin_boundingRectMax_(
        &self,
        element: NSObject,
        boundingRectMin: vector_float2,
        boundingRectMax: vector_float2,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, removeElement : element, boundingRectMin : boundingRectMin, boundingRectMax : boundingRectMax)
    }
    unsafe fn elementsInBoundingRectMin_rectMax_(
        &self,
        rectMin: vector_float2,
        rectMax: vector_float2,
    ) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, elementsInBoundingRectMin : rectMin, rectMax : rectMax)
    }
    unsafe fn queryReserve(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, queryReserve)
    }
    unsafe fn setQueryReserve_(&self, queryReserve: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setQueryReserve : queryReserve)
    }
    unsafe fn treeWithMaxNumberOfChildren_(maxNumberOfChildren: NSUInteger) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"GKRTree").unwrap(), treeWithMaxNumberOfChildren : maxNumberOfChildren)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct GKRuleSystem(pub id);
impl std::ops::Deref for GKRuleSystem {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for GKRuleSystem {}
impl GKRuleSystem {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"GKRuleSystem").unwrap(), alloc) })
    }
}
impl INSObject for GKRuleSystem {}
impl PNSObject for GKRuleSystem {}
impl std::convert::TryFrom<NSObject> for GKRuleSystem {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<GKRuleSystem, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"GKRuleSystem").unwrap()) };
        if is_kind_of {
            Ok(GKRuleSystem(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to GKRuleSystem")
        }
    }
}
impl IGKRuleSystem for GKRuleSystem {}
pub trait IGKRuleSystem: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn evaluate(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, evaluate)
    }
    unsafe fn addRule_(&self, rule: GKRule)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, addRule : rule)
    }
    unsafe fn addRulesFromArray_(&self, rules: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, addRulesFromArray : rules)
    }
    unsafe fn removeAllRules(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, removeAllRules)
    }
    unsafe fn gradeForFact_(&self, fact: *mut u64) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, gradeForFact : fact)
    }
    unsafe fn minimumGradeForFacts_(&self, facts: NSArray) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, minimumGradeForFacts : facts)
    }
    unsafe fn maximumGradeForFacts_(&self, facts: NSArray) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, maximumGradeForFacts : facts)
    }
    unsafe fn assertFact_(&self, fact: *mut u64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, assertFact : fact)
    }
    unsafe fn assertFact_grade_(&self, fact: *mut u64, grade: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, assertFact : fact, grade : grade)
    }
    unsafe fn retractFact_(&self, fact: *mut u64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, retractFact : fact)
    }
    unsafe fn retractFact_grade_(&self, fact: *mut u64, grade: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, retractFact : fact, grade : grade)
    }
    unsafe fn reset(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, reset)
    }
    unsafe fn state(&self) -> NSMutableDictionary
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, state)
    }
    unsafe fn rules(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, rules)
    }
    unsafe fn agenda(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, agenda)
    }
    unsafe fn executed(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, executed)
    }
    unsafe fn facts(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, facts)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct GKRule(pub id);
impl std::ops::Deref for GKRule {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for GKRule {}
impl GKRule {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"GKRule").unwrap(), alloc) })
    }
}
impl INSObject for GKRule {}
impl PNSObject for GKRule {}
impl std::convert::TryFrom<NSObject> for GKRule {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<GKRule, Self::Error> {
        let is_kind_of: bool = unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"GKRule").unwrap()) };
        if is_kind_of {
            Ok(GKRule(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to GKRule")
        }
    }
}
impl IGKRule for GKRule {}
pub trait IGKRule: Sized + std::ops::Deref {
    unsafe fn evaluatePredicateWithSystem_(&self, system: GKRuleSystem) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, evaluatePredicateWithSystem : system)
    }
    unsafe fn performActionWithSystem_(&self, system: GKRuleSystem)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, performActionWithSystem : system)
    }
    unsafe fn salience(&self) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, salience)
    }
    unsafe fn setSalience_(&self, salience: NSInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSalience : salience)
    }
    unsafe fn ruleWithPredicate_assertingFact_grade_(
        predicate: NSPredicate,
        fact: *mut u64,
        grade: f32,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"GKRule").unwrap(), ruleWithPredicate : predicate, assertingFact : fact, grade : grade)
    }
    unsafe fn ruleWithPredicate_retractingFact_grade_(
        predicate: NSPredicate,
        fact: *mut u64,
        grade: f32,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"GKRule").unwrap(), ruleWithPredicate : predicate, retractingFact : fact, grade : grade)
    }
    unsafe fn ruleWithBlockPredicate_action_(
        predicate: *mut ::std::os::raw::c_void,
        action: *mut ::std::os::raw::c_void,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"GKRule").unwrap(), ruleWithBlockPredicate : predicate, action : action)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct GKNSPredicateRule(pub id);
impl std::ops::Deref for GKNSPredicateRule {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for GKNSPredicateRule {}
impl GKNSPredicateRule {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"GKNSPredicateRule").unwrap(), alloc) })
    }
}
impl IGKRule for GKNSPredicateRule {}
impl From<GKNSPredicateRule> for GKRule {
    fn from(child: GKNSPredicateRule) -> GKRule {
        GKRule(child.0)
    }
}
impl std::convert::TryFrom<GKRule> for GKNSPredicateRule {
    type Error = &'static str;
    fn try_from(parent: GKRule) -> Result<GKNSPredicateRule, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"GKNSPredicateRule").unwrap()) };
        if is_kind_of {
            Ok(GKNSPredicateRule(parent.0))
        } else {
            Err("This GKRule cannot be downcasted to GKNSPredicateRule")
        }
    }
}
impl INSObject for GKNSPredicateRule {}
impl PNSObject for GKNSPredicateRule {}
impl IGKNSPredicateRule for GKNSPredicateRule {}
pub trait IGKNSPredicateRule: Sized + std::ops::Deref {
    unsafe fn initWithPredicate_(&self, predicate: NSPredicate) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithPredicate : predicate)
    }
    unsafe fn evaluatePredicateWithSystem_(&self, system: GKRuleSystem) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, evaluatePredicateWithSystem : system)
    }
    unsafe fn predicate(&self) -> NSPredicate
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, predicate)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct GKState(pub id);
impl std::ops::Deref for GKState {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for GKState {}
impl GKState {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"GKState").unwrap(), alloc) })
    }
}
impl INSObject for GKState {}
impl PNSObject for GKState {}
impl std::convert::TryFrom<NSObject> for GKState {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<GKState, Self::Error> {
        let is_kind_of: bool = unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"GKState").unwrap()) };
        if is_kind_of {
            Ok(GKState(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to GKState")
        }
    }
}
impl IGKState for GKState {}
pub trait IGKState: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn isValidNextState_(&self, stateClass: Class) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, isValidNextState : stateClass)
    }
    unsafe fn didEnterWithPreviousState_(&self, previousState: GKState)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, didEnterWithPreviousState : previousState)
    }
    unsafe fn updateWithDeltaTime_(&self, seconds: NSTimeInterval)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, updateWithDeltaTime : seconds)
    }
    unsafe fn willExitWithNextState_(&self, nextState: GKState)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, willExitWithNextState : nextState)
    }
    unsafe fn stateMachine(&self) -> GKStateMachine
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, stateMachine)
    }
    unsafe fn state() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"GKState").unwrap(), state)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct GKStateMachine(pub id);
impl std::ops::Deref for GKStateMachine {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for GKStateMachine {}
impl GKStateMachine {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"GKStateMachine").unwrap(), alloc) })
    }
}
impl INSObject for GKStateMachine {}
impl PNSObject for GKStateMachine {}
impl std::convert::TryFrom<NSObject> for GKStateMachine {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<GKStateMachine, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"GKStateMachine").unwrap()) };
        if is_kind_of {
            Ok(GKStateMachine(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to GKStateMachine")
        }
    }
}
impl IGKStateMachine for GKStateMachine {}
pub trait IGKStateMachine: Sized + std::ops::Deref {
    unsafe fn initWithStates_(&self, states: NSArray) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithStates : states)
    }
    unsafe fn updateWithDeltaTime_(&self, sec: NSTimeInterval)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, updateWithDeltaTime : sec)
    }
    unsafe fn stateForClass_(&self, stateClass: Class) -> GKState
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, stateForClass : stateClass)
    }
    unsafe fn canEnterState_(&self, stateClass: Class) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, canEnterState : stateClass)
    }
    unsafe fn enterState_(&self, stateClass: Class) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, enterState : stateClass)
    }
    unsafe fn currentState(&self) -> GKState
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, currentState)
    }
    unsafe fn stateMachineWithStates_(states: NSArray) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"GKStateMachine").unwrap(), stateMachineWithStates : states)
    }
}
pub trait PGKSceneRootNodeType: Sized + std::ops::Deref {}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct GKScene(pub id);
impl std::ops::Deref for GKScene {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for GKScene {}
impl GKScene {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"GKScene").unwrap(), alloc) })
    }
}
impl PNSCopying for GKScene {}
impl PNSSecureCoding for GKScene {}
impl INSObject for GKScene {}
impl PNSObject for GKScene {}
impl std::convert::TryFrom<NSObject> for GKScene {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<GKScene, Self::Error> {
        let is_kind_of: bool = unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"GKScene").unwrap()) };
        if is_kind_of {
            Ok(GKScene(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to GKScene")
        }
    }
}
impl IGKScene for GKScene {}
pub trait IGKScene: Sized + std::ops::Deref {
    unsafe fn addEntity_(&self, entity: GKEntity)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, addEntity : entity)
    }
    unsafe fn removeEntity_(&self, entity: GKEntity)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, removeEntity : entity)
    }
    unsafe fn addGraph_name_(&self, graph: GKGraph, name: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, addGraph : graph, name : name)
    }
    unsafe fn removeGraph_(&self, name: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, removeGraph : name)
    }
    unsafe fn entities(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, entities)
    }
    unsafe fn rootNode(&self) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, rootNode)
    }
    unsafe fn setRootNode_(&self, rootNode: *mut u64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRootNode : rootNode)
    }
    unsafe fn graphs(&self) -> NSDictionary
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, graphs)
    }
    unsafe fn sceneWithFileNamed_(filename: NSString) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"GKScene").unwrap(), sceneWithFileNamed : filename)
    }
    unsafe fn sceneWithFileNamed_rootNode_(filename: NSString, rootNode: *mut u64) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"GKScene").unwrap(), sceneWithFileNamed : filename, rootNode : rootNode)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct GKSKNodeComponent(pub id);
impl std::ops::Deref for GKSKNodeComponent {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for GKSKNodeComponent {}
impl GKSKNodeComponent {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"GKSKNodeComponent").unwrap(), alloc) })
    }
}
impl PGKAgentDelegate for GKSKNodeComponent {}
impl IGKComponent for GKSKNodeComponent {}
impl PNSCopying for GKSKNodeComponent {}
impl PNSSecureCoding for GKSKNodeComponent {}
impl std::convert::TryFrom<GKComponent> for GKSKNodeComponent {
    type Error = &'static str;
    fn try_from(parent: GKComponent) -> Result<GKSKNodeComponent, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"GKSKNodeComponent").unwrap()) };
        if is_kind_of {
            Ok(GKSKNodeComponent(parent.0))
        } else {
            Err("This GKComponent cannot be downcasted to GKSKNodeComponent")
        }
    }
}
impl INSObject for GKSKNodeComponent {}
impl PNSObject for GKSKNodeComponent {}
impl IGKSKNodeComponent for GKSKNodeComponent {}
pub trait IGKSKNodeComponent: Sized + std::ops::Deref {
    unsafe fn initWithNode_(&self, node: SKNode) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithNode : node)
    }
    unsafe fn node(&self) -> SKNode
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, node)
    }
    unsafe fn setNode_(&self, node: SKNode)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setNode : node)
    }
    unsafe fn componentWithNode_(node: SKNode) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"GKSKNodeComponent").unwrap(), componentWithNode : node)
    }
}
pub trait SKNode_GameplayKit: Sized + std::ops::Deref {
    unsafe fn entity(&self) -> GKEntity
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, entity)
    }
    unsafe fn setEntity_(&self, entity: GKEntity)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setEntity : entity)
    }
    unsafe fn obstaclesFromSpriteTextures_accuracy_(sprites: NSArray, accuracy: f32) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SKNode").unwrap(), obstaclesFromSpriteTextures : sprites, accuracy : accuracy)
    }
    unsafe fn obstaclesFromNodeBounds_(nodes: NSArray) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SKNode").unwrap(), obstaclesFromNodeBounds : nodes)
    }
    unsafe fn obstaclesFromNodePhysicsBodies_(nodes: NSArray) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SKNode").unwrap(), obstaclesFromNodePhysicsBodies : nodes)
    }
}
pub trait SKScene_GameplayKit: Sized + std::ops::Deref {}
pub trait SKTileMapNode_GameplayKit: Sized + std::ops::Deref {
    unsafe fn tileMapNodesWithTileSet_columns_rows_tileSize_fromNoiseMap_tileTypeNoiseMapThresholds_(
        tileSet: SKTileSet,
        columns: NSUInteger,
        rows: NSUInteger,
        tileSize: CGSize,
        noiseMap: GKNoiseMap,
        thresholds: NSArray,
    ) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SKTileMapNode").unwrap(), tileMapNodesWithTileSet : tileSet, columns : columns, rows : rows, tileSize : tileSize, fromNoiseMap : noiseMap, tileTypeNoiseMapThresholds : thresholds)
    }
}
pub trait SKTexture_GameplayKit: Sized + std::ops::Deref {
    unsafe fn textureWithNoiseMap_(noiseMap: GKNoiseMap) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SKTexture").unwrap(), textureWithNoiseMap : noiseMap)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct GKSCNNodeComponent(pub id);
impl std::ops::Deref for GKSCNNodeComponent {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for GKSCNNodeComponent {}
impl GKSCNNodeComponent {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"GKSCNNodeComponent").unwrap(), alloc) })
    }
}
impl PGKAgentDelegate for GKSCNNodeComponent {}
impl IGKComponent for GKSCNNodeComponent {}
impl PNSCopying for GKSCNNodeComponent {}
impl PNSSecureCoding for GKSCNNodeComponent {}
impl std::convert::TryFrom<GKComponent> for GKSCNNodeComponent {
    type Error = &'static str;
    fn try_from(parent: GKComponent) -> Result<GKSCNNodeComponent, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"GKSCNNodeComponent").unwrap()) };
        if is_kind_of {
            Ok(GKSCNNodeComponent(parent.0))
        } else {
            Err("This GKComponent cannot be downcasted to GKSCNNodeComponent")
        }
    }
}
impl INSObject for GKSCNNodeComponent {}
impl PNSObject for GKSCNNodeComponent {}
impl IGKSCNNodeComponent for GKSCNNodeComponent {}
pub trait IGKSCNNodeComponent: Sized + std::ops::Deref {
    unsafe fn initWithNode_(&self, node: SCNNode) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithNode : node)
    }
    unsafe fn node(&self) -> SCNNode
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, node)
    }
    unsafe fn componentWithNode_(node: SCNNode) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"GKSCNNodeComponent").unwrap(), componentWithNode : node)
    }
}
pub trait SCNNode_GameplayKit: Sized + std::ops::Deref {
    unsafe fn entity(&self) -> GKEntity
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, entity)
    }
    unsafe fn setEntity_(&self, entity: GKEntity)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setEntity : entity)
    }
}
pub trait SCNScene_GameplayKit: Sized + std::ops::Deref {}

unsafe impl objc2::encode::RefEncode for GKComponent {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for GKComponent {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for GKComponentSystem {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for GKComponentSystem {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for GKObstacle {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for GKObstacle {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for GKCircleObstacle {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for GKCircleObstacle {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for GKPolygonObstacle {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for GKPolygonObstacle {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for GKSphereObstacle {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for GKSphereObstacle {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for GKBehavior {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for GKBehavior {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for GKGoal {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for GKGoal {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for GKAgent {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for GKAgent {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for GKAgent2D {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for GKAgent2D {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for GKAgent3D {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for GKAgent3D {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for GKCompositeBehavior {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for GKCompositeBehavior {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for GKEntity {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for GKEntity {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for GKGraph {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for GKGraph {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for GKGridGraph {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for GKGridGraph {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for GKObstacleGraph {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for GKObstacleGraph {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for GKGraphNode {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for GKGraphNode {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for GKGraphNode2D {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for GKGraphNode2D {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for GKGraphNode3D {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for GKGraphNode3D {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for GKGridGraphNode {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for GKGridGraphNode {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for GKRandomSource {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for GKRandomSource {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for GKARC4RandomSource {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for GKARC4RandomSource {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for GKLinearCongruentialRandomSource {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for GKLinearCongruentialRandomSource {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for GKMersenneTwisterRandomSource {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for GKMersenneTwisterRandomSource {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for GKBox {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for GKBox {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("GKBox", &[]);
}
unsafe impl objc2::encode::RefEncode for GKQuad {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for GKQuad {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("GKQuad", &[]);
}
unsafe impl objc2::encode::RefEncode for GKTriangle {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for GKTriangle {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Struct("GKTriangle", &[]);
}
unsafe impl objc2::encode::RefEncode for GKMeshGraph {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for GKMeshGraph {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for GKMinmaxStrategist {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for GKMinmaxStrategist {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for GKMonteCarloStrategist {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for GKMonteCarloStrategist {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for GKDecisionNode {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for GKDecisionNode {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for GKDecisionTree {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for GKDecisionTree {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for GKNoiseSource {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for GKNoiseSource {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for GKCoherentNoiseSource {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for GKCoherentNoiseSource {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for GKPerlinNoiseSource {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for GKPerlinNoiseSource {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for GKBillowNoiseSource {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for GKBillowNoiseSource {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for GKRidgedNoiseSource {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for GKRidgedNoiseSource {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for GKVoronoiNoiseSource {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for GKVoronoiNoiseSource {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for GKConstantNoiseSource {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for GKConstantNoiseSource {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for GKCylindersNoiseSource {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for GKCylindersNoiseSource {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for GKSpheresNoiseSource {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for GKSpheresNoiseSource {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for GKCheckerboardNoiseSource {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for GKCheckerboardNoiseSource {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for GKNoise {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for GKNoise {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for GKNoiseMap {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for GKNoiseMap {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for GKOctreeNode {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for GKOctreeNode {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for GKOctree {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for GKOctree {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for GKPath {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for GKPath {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for GKQuadtreeNode {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for GKQuadtreeNode {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for GKQuadtree {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for GKQuadtree {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for GKRandomDistribution {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for GKRandomDistribution {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for GKGaussianDistribution {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for GKGaussianDistribution {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for GKShuffledDistribution {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for GKShuffledDistribution {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for GKRTree {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for GKRTree {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for GKRuleSystem {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for GKRuleSystem {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for GKRule {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for GKRule {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for GKNSPredicateRule {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for GKNSPredicateRule {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for GKState {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for GKState {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for GKStateMachine {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for GKStateMachine {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for GKScene {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for GKScene {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for GKSKNodeComponent {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for GKSKNodeComponent {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for GKSCNNodeComponent {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for GKSCNNodeComponent {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
