#[allow(unused_imports)]
use crate::objc::*;
#[allow(unused_imports)]
use crate::AppKit::*;
#[allow(unused_imports)]
use crate::CoreFoundation::*;
#[allow(unused_imports)]
use crate::Foundation::*;

#[allow(unused_imports)]
use objc2::msg_send;
pub type GKMatchProperties = NSDictionary;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct GKBasePlayer(pub id);
impl std::ops::Deref for GKBasePlayer {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for GKBasePlayer {}
impl GKBasePlayer {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"GKBasePlayer").unwrap(), alloc) })
    }
}
impl PNSCopying for GKBasePlayer {}
impl INSObject for GKBasePlayer {}
impl PNSObject for GKBasePlayer {}
impl std::convert::TryFrom<NSObject> for GKBasePlayer {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<GKBasePlayer, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"GKBasePlayer").unwrap()) };
        if is_kind_of {
            Ok(GKBasePlayer(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to GKBasePlayer")
        }
    }
}
impl IGKBasePlayer for GKBasePlayer {}
pub trait IGKBasePlayer: Sized + std::ops::Deref {
    unsafe fn playerID(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, playerID)
    }
    unsafe fn displayName(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, displayName)
    }
}
pub type GKErrorCode = NSInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct GKPlayer(pub id);
impl std::ops::Deref for GKPlayer {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for GKPlayer {}
impl GKPlayer {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"GKPlayer").unwrap(), alloc) })
    }
}
impl IGKBasePlayer for GKPlayer {}
impl PNSCopying for GKPlayer {}
impl From<GKPlayer> for GKBasePlayer {
    fn from(child: GKPlayer) -> GKBasePlayer {
        GKBasePlayer(child.0)
    }
}
impl std::convert::TryFrom<GKBasePlayer> for GKPlayer {
    type Error = &'static str;
    fn try_from(parent: GKBasePlayer) -> Result<GKPlayer, Self::Error> {
        let is_kind_of: bool = unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"GKPlayer").unwrap()) };
        if is_kind_of {
            Ok(GKPlayer(parent.0))
        } else {
            Err("This GKBasePlayer cannot be downcasted to GKPlayer")
        }
    }
}
impl INSObject for GKPlayer {}
impl PNSObject for GKPlayer {}
impl IGKPlayer for GKPlayer {}
pub trait IGKPlayer: Sized + std::ops::Deref {
    unsafe fn scopedIDsArePersistent(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, scopedIDsArePersistent)
    }
    unsafe fn gamePlayerID(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, gamePlayerID)
    }
    unsafe fn teamPlayerID(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, teamPlayerID)
    }
    unsafe fn displayName(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, displayName)
    }
    unsafe fn alias(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, alias)
    }
    unsafe fn guestIdentifier(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, guestIdentifier)
    }
    unsafe fn isInvitable(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isInvitable)
    }
    unsafe fn anonymousGuestPlayerWithIdentifier_(guestIdentifier: NSString) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"GKPlayer").unwrap(), anonymousGuestPlayerWithIdentifier : guestIdentifier)
    }
}
impl GKPlayer_UI for GKPlayer {}
pub trait GKPlayer_UI: Sized + std::ops::Deref {
    unsafe fn loadPhotoForSize_withCompletionHandler_(
        &self,
        size: GKPhotoSize,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, loadPhotoForSize : size, withCompletionHandler : completionHandler)
    }
}
pub type GKPhotoSize = NSInteger;
impl GKPlayer_Deprecated for GKPlayer {}
pub trait GKPlayer_Deprecated: Sized + std::ops::Deref {
    unsafe fn isFriend(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isFriend)
    }
    unsafe fn playerID(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, playerID)
    }
    unsafe fn loadPlayersForIdentifiers_withCompletionHandler_(
        identifiers: NSArray,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"GKPlayer").unwrap(), loadPlayersForIdentifiers : identifiers, withCompletionHandler : completionHandler)
    }
}
pub type GKReleaseState = NSUInteger;
pub type GKLeaderboardTimeScope = NSInteger;
pub type GKLeaderboardPlayerScope = NSInteger;
pub type GKLeaderboardType = NSInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct GKLeaderboard(pub id);
impl std::ops::Deref for GKLeaderboard {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for GKLeaderboard {}
impl GKLeaderboard {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"GKLeaderboard").unwrap(), alloc) })
    }
}
impl INSObject for GKLeaderboard {}
impl PNSObject for GKLeaderboard {}
impl std::convert::TryFrom<NSObject> for GKLeaderboard {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<GKLeaderboard, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"GKLeaderboard").unwrap()) };
        if is_kind_of {
            Ok(GKLeaderboard(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to GKLeaderboard")
        }
    }
}
impl IGKLeaderboard for GKLeaderboard {}
pub trait IGKLeaderboard: Sized + std::ops::Deref {
    unsafe fn loadPreviousOccurrenceWithCompletionHandler_(
        &self,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, loadPreviousOccurrenceWithCompletionHandler : completionHandler)
    }
    unsafe fn submitScore_context_player_completionHandler_(
        &self,
        score: NSInteger,
        context: NSUInteger,
        player: GKPlayer,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, submitScore : score, context : context, player : player, completionHandler : completionHandler)
    }
    unsafe fn loadEntriesForPlayerScope_timeScope_range_completionHandler_(
        &self,
        playerScope: GKLeaderboardPlayerScope,
        timeScope: GKLeaderboardTimeScope,
        range: NSRange,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, loadEntriesForPlayerScope : playerScope, timeScope : timeScope, range : range, completionHandler : completionHandler)
    }
    unsafe fn loadEntriesForPlayers_timeScope_completionHandler_(
        &self,
        players: NSArray,
        timeScope: GKLeaderboardTimeScope,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, loadEntriesForPlayers : players, timeScope : timeScope, completionHandler : completionHandler)
    }
    unsafe fn title(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, title)
    }
    unsafe fn groupIdentifier(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, groupIdentifier)
    }
    unsafe fn baseLeaderboardID(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, baseLeaderboardID)
    }
    unsafe fn type_(&self) -> GKLeaderboardType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, type)
    }
    unsafe fn startDate(&self) -> NSDate
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, startDate)
    }
    unsafe fn nextStartDate(&self) -> NSDate
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, nextStartDate)
    }
    unsafe fn duration(&self) -> NSTimeInterval
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, duration)
    }
    unsafe fn leaderboardDescription(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, leaderboardDescription)
    }
    unsafe fn releaseState(&self) -> GKReleaseState
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, releaseState)
    }
    unsafe fn activityIdentifier(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, activityIdentifier)
    }
    unsafe fn activityProperties(&self) -> NSDictionary
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, activityProperties)
    }
    unsafe fn isHidden(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isHidden)
    }
    unsafe fn loadLeaderboardsWithIDs_completionHandler_(
        leaderboardIDs: NSArray,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"GKLeaderboard").unwrap(), loadLeaderboardsWithIDs : leaderboardIDs, completionHandler : completionHandler)
    }
    unsafe fn submitScore_context_player_leaderboardIDs_completionHandler_(
        score: NSInteger,
        context: NSUInteger,
        player: GKPlayer,
        leaderboardIDs: NSArray,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"GKLeaderboard").unwrap(), submitScore : score, context : context, player : player, leaderboardIDs : leaderboardIDs, completionHandler : completionHandler)
    }
}
impl GKLeaderboard_Deprecated for GKLeaderboard {}
pub trait GKLeaderboard_Deprecated: Sized + std::ops::Deref {
    unsafe fn initWithPlayerIDs_(&self, playerIDs: NSArray) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithPlayerIDs : playerIDs)
    }
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn initWithPlayers_(&self, players: NSArray) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithPlayers : players)
    }
    unsafe fn loadScoresWithCompletionHandler_(
        &self,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, loadScoresWithCompletionHandler : completionHandler)
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
    unsafe fn timeScope(&self) -> GKLeaderboardTimeScope
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, timeScope)
    }
    unsafe fn setTimeScope_(&self, timeScope: GKLeaderboardTimeScope)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setTimeScope : timeScope)
    }
    unsafe fn playerScope(&self) -> GKLeaderboardPlayerScope
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, playerScope)
    }
    unsafe fn setPlayerScope_(&self, playerScope: GKLeaderboardPlayerScope)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPlayerScope : playerScope)
    }
    unsafe fn identifier(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, identifier)
    }
    unsafe fn setIdentifier_(&self, identifier: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setIdentifier : identifier)
    }
    unsafe fn range(&self) -> NSRange
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, range)
    }
    unsafe fn setRange_(&self, range: NSRange)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRange : range)
    }
    unsafe fn scores(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, scores)
    }
    unsafe fn maxRange(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, maxRange)
    }
    unsafe fn localPlayerScore(&self) -> GKScore
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, localPlayerScore)
    }
    unsafe fn isLoading(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isLoading)
    }
    unsafe fn loadCategoriesWithCompletionHandler_(completionHandler: *mut ::std::os::raw::c_void)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"GKLeaderboard").unwrap(), loadCategoriesWithCompletionHandler : completionHandler)
    }
    unsafe fn setDefaultLeaderboard_withCompletionHandler_(
        leaderboardIdentifier: NSString,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"GKLeaderboard").unwrap(), setDefaultLeaderboard : leaderboardIdentifier, withCompletionHandler : completionHandler)
    }
    unsafe fn loadLeaderboardsWithCompletionHandler_(completionHandler: *mut ::std::os::raw::c_void)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"GKLeaderboard").unwrap(), loadLeaderboardsWithCompletionHandler : completionHandler)
    }
}
impl GKLeaderboard_UI for GKLeaderboard {}
pub trait GKLeaderboard_UI: Sized + std::ops::Deref {
    unsafe fn loadImageWithCompletionHandler_(&self, completionHandler: *mut ::std::os::raw::c_void)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, loadImageWithCompletionHandler : completionHandler)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct GKAchievement(pub id);
impl std::ops::Deref for GKAchievement {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for GKAchievement {}
impl GKAchievement {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"GKAchievement").unwrap(), alloc) })
    }
}
impl PNSCoding for GKAchievement {}
impl PNSSecureCoding for GKAchievement {}
impl INSObject for GKAchievement {}
impl PNSObject for GKAchievement {}
impl std::convert::TryFrom<NSObject> for GKAchievement {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<GKAchievement, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"GKAchievement").unwrap()) };
        if is_kind_of {
            Ok(GKAchievement(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to GKAchievement")
        }
    }
}
impl IGKAchievement for GKAchievement {}
pub trait IGKAchievement: Sized + std::ops::Deref {
    unsafe fn initWithIdentifier_(&self, identifier: NSString) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithIdentifier : identifier)
    }
    unsafe fn initWithIdentifier_player_(
        &self,
        identifier: NSString,
        player: GKPlayer,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithIdentifier : identifier, player : player)
    }
    unsafe fn identifier(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, identifier)
    }
    unsafe fn setIdentifier_(&self, identifier: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setIdentifier : identifier)
    }
    unsafe fn percentComplete(&self) -> f64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, percentComplete)
    }
    unsafe fn setPercentComplete_(&self, percentComplete: f64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPercentComplete : percentComplete)
    }
    unsafe fn isCompleted(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isCompleted)
    }
    unsafe fn lastReportedDate(&self) -> NSDate
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, lastReportedDate)
    }
    unsafe fn showsCompletionBanner(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, showsCompletionBanner)
    }
    unsafe fn setShowsCompletionBanner_(&self, showsCompletionBanner: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setShowsCompletionBanner : showsCompletionBanner)
    }
    unsafe fn player(&self) -> GKPlayer
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, player)
    }
    unsafe fn loadAchievementsWithCompletionHandler_(completionHandler: *mut ::std::os::raw::c_void)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"GKAchievement").unwrap(), loadAchievementsWithCompletionHandler : completionHandler)
    }
    unsafe fn resetAchievementsWithCompletionHandler_(
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"GKAchievement").unwrap(), resetAchievementsWithCompletionHandler : completionHandler)
    }
    unsafe fn reportAchievements_withCompletionHandler_(
        achievements: NSArray,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"GKAchievement").unwrap(), reportAchievements : achievements, withCompletionHandler : completionHandler)
    }
}
impl GKAchievement_Deprecated for GKAchievement {}
pub trait GKAchievement_Deprecated: Sized + std::ops::Deref {
    unsafe fn reportAchievementWithCompletionHandler_(
        &self,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, reportAchievementWithCompletionHandler : completionHandler)
    }
    unsafe fn isHidden(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isHidden)
    }
}
impl GKAchievement_Obsoleted for GKAchievement {}
pub trait GKAchievement_Obsoleted: Sized + std::ops::Deref {
    unsafe fn initWithIdentifier_forPlayer_(
        &self,
        identifier: NSString,
        playerID: NSString,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithIdentifier : identifier, forPlayer : playerID)
    }
    unsafe fn playerID(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, playerID)
    }
}
pub type GKGameCenterViewControllerState = NSInteger;
pub trait PGKViewController: Sized + std::ops::Deref {}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct GKDialogController(pub id);
impl std::ops::Deref for GKDialogController {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for GKDialogController {}
impl GKDialogController {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"GKDialogController").unwrap(), alloc) })
    }
}
impl INSResponder for GKDialogController {}
impl PNSCoding for GKDialogController {}
impl std::convert::TryFrom<NSResponder> for GKDialogController {
    type Error = &'static str;
    fn try_from(parent: NSResponder) -> Result<GKDialogController, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"GKDialogController").unwrap()) };
        if is_kind_of {
            Ok(GKDialogController(parent.0))
        } else {
            Err("This NSResponder cannot be downcasted to GKDialogController")
        }
    }
}
impl INSObject for GKDialogController {}
impl PNSObject for GKDialogController {}
impl IGKDialogController for GKDialogController {}
pub trait IGKDialogController: Sized + std::ops::Deref {
    unsafe fn presentViewController_(&self, viewController: NSViewController) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, presentViewController : viewController)
    }
    unsafe fn dismiss_(&self, sender: id)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, dismiss : sender)
    }
    unsafe fn parentWindow(&self) -> NSWindow
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, parentWindow)
    }
    unsafe fn setParentWindow_(&self, parentWindow: NSWindow)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setParentWindow : parentWindow)
    }
}
impl GKDialogController_SharedDialogController for GKDialogController {}
pub trait GKDialogController_SharedDialogController: Sized + std::ops::Deref {
    unsafe fn sharedDialogController() -> GKDialogController
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"GKDialogController").unwrap(), sharedDialogController)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct GKGameCenterViewController(pub id);
impl std::ops::Deref for GKGameCenterViewController {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for GKGameCenterViewController {}
impl GKGameCenterViewController {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"GKGameCenterViewController").unwrap(), alloc) })
    }
}
impl PGKViewController for GKGameCenterViewController {}
impl INSViewController for GKGameCenterViewController {}
impl PNSEditor for GKGameCenterViewController {}
impl PNSSeguePerforming for GKGameCenterViewController {}
impl PNSUserInterfaceItemIdentification for GKGameCenterViewController {}
impl std::convert::TryFrom<NSViewController> for GKGameCenterViewController {
    type Error = &'static str;
    fn try_from(parent: NSViewController) -> Result<GKGameCenterViewController, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"GKGameCenterViewController").unwrap()) };
        if is_kind_of {
            Ok(GKGameCenterViewController(parent.0))
        } else {
            Err("This NSViewController cannot be downcasted to GKGameCenterViewController")
        }
    }
}
impl INSResponder for GKGameCenterViewController {}
impl PNSCoding for GKGameCenterViewController {}
impl INSObject for GKGameCenterViewController {}
impl PNSObject for GKGameCenterViewController {}
impl IGKGameCenterViewController for GKGameCenterViewController {}
pub trait IGKGameCenterViewController: Sized + std::ops::Deref {}
impl GKGameCenterViewController_ for GKGameCenterViewController {}
pub trait GKGameCenterViewController_: Sized + std::ops::Deref {
    unsafe fn gameCenterDelegate(&self) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, gameCenterDelegate)
    }
    unsafe fn setGameCenterDelegate_(&self, gameCenterDelegate: *mut u64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setGameCenterDelegate : gameCenterDelegate)
    }
    unsafe fn initWithState_(&self, state: GKGameCenterViewControllerState) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithState : state)
    }
    unsafe fn initWithLeaderboardID_playerScope_timeScope_(
        &self,
        leaderboardID: NSString,
        playerScope: GKLeaderboardPlayerScope,
        timeScope: GKLeaderboardTimeScope,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithLeaderboardID : leaderboardID, playerScope : playerScope, timeScope : timeScope)
    }
    unsafe fn initWithLeaderboard_playerScope_(
        &self,
        leaderboard: GKLeaderboard,
        playerScope: GKLeaderboardPlayerScope,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithLeaderboard : leaderboard, playerScope : playerScope)
    }
    unsafe fn initWithLeaderboardSetID_(&self, leaderboardSetID: NSString) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithLeaderboardSetID : leaderboardSetID)
    }
    unsafe fn initWithAchievementID_(&self, achievementID: NSString) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithAchievementID : achievementID)
    }
    unsafe fn initWithPlayer_(&self, player: GKPlayer) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithPlayer : player)
    }
}
impl GKGameCenterViewController_Deprecated for GKGameCenterViewController {}
pub trait GKGameCenterViewController_Deprecated: Sized + std::ops::Deref {
    unsafe fn viewState(&self) -> GKGameCenterViewControllerState
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, viewState)
    }
    unsafe fn setViewState_(&self, viewState: GKGameCenterViewControllerState)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setViewState : viewState)
    }
    unsafe fn leaderboardTimeScope(&self) -> GKLeaderboardTimeScope
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, leaderboardTimeScope)
    }
    unsafe fn setLeaderboardTimeScope_(&self, leaderboardTimeScope: GKLeaderboardTimeScope)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setLeaderboardTimeScope : leaderboardTimeScope)
    }
    unsafe fn leaderboardIdentifier(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, leaderboardIdentifier)
    }
    unsafe fn setLeaderboardIdentifier_(&self, leaderboardIdentifier: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setLeaderboardIdentifier : leaderboardIdentifier)
    }
    unsafe fn leaderboardCategory(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, leaderboardCategory)
    }
    unsafe fn setLeaderboardCategory_(&self, leaderboardCategory: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setLeaderboardCategory : leaderboardCategory)
    }
}
pub trait PGKGameCenterControllerDelegate: Sized + std::ops::Deref {
    unsafe fn gameCenterViewControllerDidFinish_(
        &self,
        gameCenterViewController: GKGameCenterViewController,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, gameCenterViewControllerDidFinish : gameCenterViewController)
    }
}
pub type GKAccessPointLocation = NSInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct GKAccessPoint(pub id);
impl std::ops::Deref for GKAccessPoint {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for GKAccessPoint {}
impl GKAccessPoint {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"GKAccessPoint").unwrap(), alloc) })
    }
}
impl INSObject for GKAccessPoint {}
impl PNSObject for GKAccessPoint {}
impl std::convert::TryFrom<NSObject> for GKAccessPoint {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<GKAccessPoint, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"GKAccessPoint").unwrap()) };
        if is_kind_of {
            Ok(GKAccessPoint(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to GKAccessPoint")
        }
    }
}
impl IGKAccessPoint for GKAccessPoint {}
pub trait IGKAccessPoint: Sized + std::ops::Deref {
    unsafe fn triggerAccessPointWithHandler_(&self, handler: *mut ::std::os::raw::c_void)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, triggerAccessPointWithHandler : handler)
    }
    unsafe fn triggerAccessPointWithState_handler_(
        &self,
        state: GKGameCenterViewControllerState,
        handler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, triggerAccessPointWithState : state, handler : handler)
    }
    unsafe fn triggerAccessPointWithAchievementID_handler_(
        &self,
        achievementID: NSString,
        handler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, triggerAccessPointWithAchievementID : achievementID, handler : handler)
    }
    unsafe fn triggerAccessPointWithLeaderboardSetID_handler_(
        &self,
        leaderboardSetID: NSString,
        handler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, triggerAccessPointWithLeaderboardSetID : leaderboardSetID, handler : handler)
    }
    unsafe fn triggerAccessPointWithLeaderboardID_playerScope_timeScope_handler_(
        &self,
        leaderboardID: NSString,
        playerScope: GKLeaderboardPlayerScope,
        timeScope: GKLeaderboardTimeScope,
        handler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, triggerAccessPointWithLeaderboardID : leaderboardID, playerScope : playerScope, timeScope : timeScope, handler : handler)
    }
    unsafe fn triggerAccessPointWithPlayer_handler_(
        &self,
        player: GKPlayer,
        handler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, triggerAccessPointWithPlayer : player, handler : handler)
    }
    unsafe fn triggerAccessPointForPlayTogetherWithHandler_(
        &self,
        handler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, triggerAccessPointForPlayTogetherWithHandler : handler)
    }
    unsafe fn triggerAccessPointForChallengesWithHandler_(
        &self,
        handler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, triggerAccessPointForChallengesWithHandler : handler)
    }
    unsafe fn triggerAccessPointWithChallengeDefinitionID_handler_(
        &self,
        challengeDefinitionID: NSString,
        handler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, triggerAccessPointWithChallengeDefinitionID : challengeDefinitionID, handler : handler)
    }
    unsafe fn triggerAccessPointWithGameActivityDefinitionID_handler_(
        &self,
        gameActivityDefinitionID: NSString,
        handler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, triggerAccessPointWithGameActivityDefinitionID : gameActivityDefinitionID, handler : handler)
    }
    unsafe fn triggerAccessPointWithGameActivity_handler_(
        &self,
        gameActivity: GKGameActivity,
        handler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, triggerAccessPointWithGameActivity : gameActivity, handler : handler)
    }
    unsafe fn triggerAccessPointForFriendingWithHandler_(
        &self,
        handler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, triggerAccessPointForFriendingWithHandler : handler)
    }
    unsafe fn triggerAccessPointForArcadeWithHandler_(&self, handler: *mut ::std::os::raw::c_void)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, triggerAccessPointForArcadeWithHandler : handler)
    }
    unsafe fn isActive(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isActive)
    }
    unsafe fn setActive_(&self, active: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setActive : active)
    }
    unsafe fn isFocused(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isFocused)
    }
    unsafe fn setFocused_(&self, focused: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setFocused : focused)
    }
    unsafe fn isVisible(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isVisible)
    }
    unsafe fn isPresentingGameCenter(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isPresentingGameCenter)
    }
    unsafe fn showHighlights(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, showHighlights)
    }
    unsafe fn setShowHighlights_(&self, showHighlights: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setShowHighlights : showHighlights)
    }
    unsafe fn location(&self) -> GKAccessPointLocation
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, location)
    }
    unsafe fn setLocation_(&self, location: GKAccessPointLocation)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setLocation : location)
    }
    unsafe fn frameInScreenCoordinates(&self) -> NSRect
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, frameInScreenCoordinates)
    }
    unsafe fn parentWindow(&self) -> NSWindow
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, parentWindow)
    }
    unsafe fn setParentWindow_(&self, parentWindow: NSWindow)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setParentWindow : parentWindow)
    }
    unsafe fn shared() -> GKAccessPoint
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"GKAccessPoint").unwrap(), shared)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct GKAchievementDescription(pub id);
impl std::ops::Deref for GKAchievementDescription {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for GKAchievementDescription {}
impl GKAchievementDescription {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"GKAchievementDescription").unwrap(), alloc) })
    }
}
impl PNSCoding for GKAchievementDescription {}
impl PNSSecureCoding for GKAchievementDescription {}
impl INSObject for GKAchievementDescription {}
impl PNSObject for GKAchievementDescription {}
impl std::convert::TryFrom<NSObject> for GKAchievementDescription {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<GKAchievementDescription, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"GKAchievementDescription").unwrap()) };
        if is_kind_of {
            Ok(GKAchievementDescription(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to GKAchievementDescription")
        }
    }
}
impl IGKAchievementDescription for GKAchievementDescription {}
pub trait IGKAchievementDescription: Sized + std::ops::Deref {
    unsafe fn identifier(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, identifier)
    }
    unsafe fn groupIdentifier(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, groupIdentifier)
    }
    unsafe fn title(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, title)
    }
    unsafe fn achievedDescription(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, achievedDescription)
    }
    unsafe fn unachievedDescription(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, unachievedDescription)
    }
    unsafe fn maximumPoints(&self) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, maximumPoints)
    }
    unsafe fn isHidden(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isHidden)
    }
    unsafe fn isReplayable(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isReplayable)
    }
    unsafe fn rarityPercent(&self) -> NSNumber
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, rarityPercent)
    }
    unsafe fn releaseState(&self) -> GKReleaseState
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, releaseState)
    }
    unsafe fn activityIdentifier(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, activityIdentifier)
    }
    unsafe fn activityProperties(&self) -> NSDictionary
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, activityProperties)
    }
    unsafe fn loadAchievementDescriptionsWithCompletionHandler_(
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"GKAchievementDescription").unwrap(), loadAchievementDescriptionsWithCompletionHandler : completionHandler)
    }
}
impl GKAchievementDescription_UI for GKAchievementDescription {}
pub trait GKAchievementDescription_UI: Sized + std::ops::Deref {
    unsafe fn loadImageWithCompletionHandler_(&self, completionHandler: *mut ::std::os::raw::c_void)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, loadImageWithCompletionHandler : completionHandler)
    }
    unsafe fn image(&self) -> NSImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, image)
    }
    unsafe fn incompleteAchievementImage() -> NSImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"GKAchievementDescription").unwrap(), incompleteAchievementImage)
    }
    unsafe fn placeholderCompletedAchievementImage() -> NSImage
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"GKAchievementDescription").unwrap(), placeholderCompletedAchievementImage)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct GKAchievementViewController(pub id);
impl std::ops::Deref for GKAchievementViewController {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for GKAchievementViewController {}
impl GKAchievementViewController {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"GKAchievementViewController").unwrap(), alloc) })
    }
}
impl IGKGameCenterViewController for GKAchievementViewController {}
impl PGKViewController for GKAchievementViewController {}
impl From<GKAchievementViewController> for GKGameCenterViewController {
    fn from(child: GKAchievementViewController) -> GKGameCenterViewController {
        GKGameCenterViewController(child.0)
    }
}
impl std::convert::TryFrom<GKGameCenterViewController> for GKAchievementViewController {
    type Error = &'static str;
    fn try_from(
        parent: GKGameCenterViewController,
    ) -> Result<GKAchievementViewController, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"GKAchievementViewController").unwrap()) };
        if is_kind_of {
            Ok(GKAchievementViewController(parent.0))
        } else {
            Err ("This GKGameCenterViewController cannot be downcasted to GKAchievementViewController" ,)
        }
    }
}
impl INSViewController for GKAchievementViewController {}
impl PNSEditor for GKAchievementViewController {}
impl PNSSeguePerforming for GKAchievementViewController {}
impl PNSUserInterfaceItemIdentification for GKAchievementViewController {}
impl INSResponder for GKAchievementViewController {}
impl PNSCoding for GKAchievementViewController {}
impl INSObject for GKAchievementViewController {}
impl PNSObject for GKAchievementViewController {}
impl IGKAchievementViewController for GKAchievementViewController {}
pub trait IGKAchievementViewController: Sized + std::ops::Deref {
    unsafe fn achievementDelegate(&self) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, achievementDelegate)
    }
    unsafe fn setAchievementDelegate_(&self, achievementDelegate: *mut u64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAchievementDelegate : achievementDelegate)
    }
}
pub trait PGKAchievementViewControllerDelegate: Sized + std::ops::Deref {
    unsafe fn achievementViewControllerDidFinish_(
        &self,
        viewController: GKAchievementViewController,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, achievementViewControllerDidFinish : viewController)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct GKScore(pub id);
impl std::ops::Deref for GKScore {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for GKScore {}
impl GKScore {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"GKScore").unwrap(), alloc) })
    }
}
impl PNSCoding for GKScore {}
impl PNSSecureCoding for GKScore {}
impl INSObject for GKScore {}
impl PNSObject for GKScore {}
impl std::convert::TryFrom<NSObject> for GKScore {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<GKScore, Self::Error> {
        let is_kind_of: bool = unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"GKScore").unwrap()) };
        if is_kind_of {
            Ok(GKScore(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to GKScore")
        }
    }
}
impl IGKScore for GKScore {}
pub trait IGKScore: Sized + std::ops::Deref {
    unsafe fn initWithLeaderboardIdentifier_(&self, identifier: NSString) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithLeaderboardIdentifier : identifier)
    }
    unsafe fn initWithLeaderboardIdentifier_player_(
        &self,
        identifier: NSString,
        player: GKPlayer,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithLeaderboardIdentifier : identifier, player : player)
    }
    unsafe fn value(&self) -> i64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, value)
    }
    unsafe fn setValue_(&self, value: i64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setValue : value)
    }
    unsafe fn formattedValue(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, formattedValue)
    }
    unsafe fn leaderboardIdentifier(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, leaderboardIdentifier)
    }
    unsafe fn setLeaderboardIdentifier_(&self, leaderboardIdentifier: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setLeaderboardIdentifier : leaderboardIdentifier)
    }
    unsafe fn context(&self) -> u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, context)
    }
    unsafe fn setContext_(&self, context: u64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setContext : context)
    }
    unsafe fn date(&self) -> NSDate
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, date)
    }
    unsafe fn player(&self) -> GKPlayer
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, player)
    }
    unsafe fn rank(&self) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, rank)
    }
    unsafe fn shouldSetDefaultLeaderboard(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, shouldSetDefaultLeaderboard)
    }
    unsafe fn setShouldSetDefaultLeaderboard_(&self, shouldSetDefaultLeaderboard: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setShouldSetDefaultLeaderboard : shouldSetDefaultLeaderboard)
    }
    unsafe fn reportScores_withCompletionHandler_(
        scores: NSArray,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"GKScore").unwrap(), reportScores : scores, withCompletionHandler : completionHandler)
    }
}
impl GKScore_Deprecated for GKScore {}
pub trait GKScore_Deprecated: Sized + std::ops::Deref {
    unsafe fn reportScoreWithCompletionHandler_(
        &self,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, reportScoreWithCompletionHandler : completionHandler)
    }
    unsafe fn initWithCategory_(&self, category: NSString) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithCategory : category)
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
}
impl GKScore_Obsoleted for GKScore {}
pub trait GKScore_Obsoleted: Sized + std::ops::Deref {
    unsafe fn initWithLeaderboardIdentifier_forPlayer_(
        &self,
        identifier: NSString,
        playerID: NSString,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithLeaderboardIdentifier : identifier, forPlayer : playerID)
    }
    unsafe fn playerID(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, playerID)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct GKLeaderboardEntry(pub id);
impl std::ops::Deref for GKLeaderboardEntry {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for GKLeaderboardEntry {}
impl GKLeaderboardEntry {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"GKLeaderboardEntry").unwrap(), alloc) })
    }
}
impl INSObject for GKLeaderboardEntry {}
impl PNSObject for GKLeaderboardEntry {}
impl std::convert::TryFrom<NSObject> for GKLeaderboardEntry {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<GKLeaderboardEntry, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"GKLeaderboardEntry").unwrap()) };
        if is_kind_of {
            Ok(GKLeaderboardEntry(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to GKLeaderboardEntry")
        }
    }
}
impl IGKLeaderboardEntry for GKLeaderboardEntry {}
pub trait IGKLeaderboardEntry: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn player(&self) -> GKPlayer
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, player)
    }
    unsafe fn rank(&self) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, rank)
    }
    unsafe fn score(&self) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, score)
    }
    unsafe fn formattedScore(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, formattedScore)
    }
    unsafe fn context(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, context)
    }
    unsafe fn date(&self) -> NSDate
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, date)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct GKLeaderboardScore(pub id);
impl std::ops::Deref for GKLeaderboardScore {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for GKLeaderboardScore {}
impl GKLeaderboardScore {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"GKLeaderboardScore").unwrap(), alloc) })
    }
}
impl INSObject for GKLeaderboardScore {}
impl PNSObject for GKLeaderboardScore {}
impl std::convert::TryFrom<NSObject> for GKLeaderboardScore {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<GKLeaderboardScore, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"GKLeaderboardScore").unwrap()) };
        if is_kind_of {
            Ok(GKLeaderboardScore(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to GKLeaderboardScore")
        }
    }
}
impl IGKLeaderboardScore for GKLeaderboardScore {}
pub trait IGKLeaderboardScore: Sized + std::ops::Deref {
    unsafe fn player(&self) -> GKPlayer
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, player)
    }
    unsafe fn setPlayer_(&self, player: GKPlayer)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPlayer : player)
    }
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
    unsafe fn context(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, context)
    }
    unsafe fn setContext_(&self, context: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setContext : context)
    }
    unsafe fn leaderboardID(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, leaderboardID)
    }
    unsafe fn setLeaderboardID_(&self, leaderboardID: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setLeaderboardID : leaderboardID)
    }
}
pub type GKChallengeState = NSInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct GKChallenge(pub id);
impl std::ops::Deref for GKChallenge {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for GKChallenge {}
impl GKChallenge {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"GKChallenge").unwrap(), alloc) })
    }
}
impl PNSCoding for GKChallenge {}
impl PNSSecureCoding for GKChallenge {}
impl INSObject for GKChallenge {}
impl PNSObject for GKChallenge {}
impl std::convert::TryFrom<NSObject> for GKChallenge {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<GKChallenge, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"GKChallenge").unwrap()) };
        if is_kind_of {
            Ok(GKChallenge(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to GKChallenge")
        }
    }
}
impl IGKChallenge for GKChallenge {}
pub trait IGKChallenge: Sized + std::ops::Deref {
    unsafe fn decline(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, decline)
    }
    unsafe fn issuingPlayer(&self) -> GKPlayer
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, issuingPlayer)
    }
    unsafe fn receivingPlayer(&self) -> GKPlayer
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, receivingPlayer)
    }
    unsafe fn state(&self) -> GKChallengeState
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, state)
    }
    unsafe fn issueDate(&self) -> NSDate
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, issueDate)
    }
    unsafe fn completionDate(&self) -> NSDate
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, completionDate)
    }
    unsafe fn message(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, message)
    }
    unsafe fn loadReceivedChallengesWithCompletionHandler_(
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"GKChallenge").unwrap(), loadReceivedChallengesWithCompletionHandler : completionHandler)
    }
}
impl GKChallenge_Obsoleted for GKChallenge {}
pub trait GKChallenge_Obsoleted: Sized + std::ops::Deref {
    unsafe fn issuingPlayerID(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, issuingPlayerID)
    }
    unsafe fn receivingPlayerID(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, receivingPlayerID)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct GKScoreChallenge(pub id);
impl std::ops::Deref for GKScoreChallenge {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for GKScoreChallenge {}
impl GKScoreChallenge {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"GKScoreChallenge").unwrap(), alloc) })
    }
}
impl IGKChallenge for GKScoreChallenge {}
impl PNSCoding for GKScoreChallenge {}
impl PNSSecureCoding for GKScoreChallenge {}
impl From<GKScoreChallenge> for GKChallenge {
    fn from(child: GKScoreChallenge) -> GKChallenge {
        GKChallenge(child.0)
    }
}
impl std::convert::TryFrom<GKChallenge> for GKScoreChallenge {
    type Error = &'static str;
    fn try_from(parent: GKChallenge) -> Result<GKScoreChallenge, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"GKScoreChallenge").unwrap()) };
        if is_kind_of {
            Ok(GKScoreChallenge(parent.0))
        } else {
            Err("This GKChallenge cannot be downcasted to GKScoreChallenge")
        }
    }
}
impl INSObject for GKScoreChallenge {}
impl PNSObject for GKScoreChallenge {}
impl IGKScoreChallenge for GKScoreChallenge {}
pub trait IGKScoreChallenge: Sized + std::ops::Deref {
    unsafe fn score(&self) -> GKScore
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, score)
    }
    unsafe fn leaderboardEntry(&self) -> GKLeaderboardEntry
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, leaderboardEntry)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct GKAchievementChallenge(pub id);
impl std::ops::Deref for GKAchievementChallenge {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for GKAchievementChallenge {}
impl GKAchievementChallenge {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"GKAchievementChallenge").unwrap(), alloc) })
    }
}
impl IGKChallenge for GKAchievementChallenge {}
impl PNSCoding for GKAchievementChallenge {}
impl PNSSecureCoding for GKAchievementChallenge {}
impl std::convert::TryFrom<GKChallenge> for GKAchievementChallenge {
    type Error = &'static str;
    fn try_from(parent: GKChallenge) -> Result<GKAchievementChallenge, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"GKAchievementChallenge").unwrap()) };
        if is_kind_of {
            Ok(GKAchievementChallenge(parent.0))
        } else {
            Err("This GKChallenge cannot be downcasted to GKAchievementChallenge")
        }
    }
}
impl INSObject for GKAchievementChallenge {}
impl PNSObject for GKAchievementChallenge {}
impl IGKAchievementChallenge for GKAchievementChallenge {}
pub trait IGKAchievementChallenge: Sized + std::ops::Deref {
    unsafe fn achievement(&self) -> GKAchievement
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, achievement)
    }
}
impl GKScore_GKChallenge for GKScore {}
pub trait GKScore_GKChallenge: Sized + std::ops::Deref {
    unsafe fn reportScores_withEligibleChallenges_withCompletionHandler_(
        scores: NSArray,
        challenges: NSArray,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"GKScore").unwrap(), reportScores : scores, withEligibleChallenges : challenges, withCompletionHandler : completionHandler)
    }
    unsafe fn reportLeaderboardScores_withEligibleChallenges_withCompletionHandler_(
        scores: NSArray,
        challenges: NSArray,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"GKScore").unwrap(), reportLeaderboardScores : scores, withEligibleChallenges : challenges, withCompletionHandler : completionHandler)
    }
}
impl GKAchievement_GKChallenge for GKAchievement {}
pub trait GKAchievement_GKChallenge: Sized + std::ops::Deref {
    unsafe fn selectChallengeablePlayers_withCompletionHandler_(
        &self,
        players: NSArray,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, selectChallengeablePlayers : players, withCompletionHandler : completionHandler)
    }
    unsafe fn reportAchievements_withEligibleChallenges_withCompletionHandler_(
        achievements: NSArray,
        challenges: NSArray,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"GKAchievement").unwrap(), reportAchievements : achievements, withEligibleChallenges : challenges, withCompletionHandler : completionHandler)
    }
}
impl GKScore_GKChallengeObsoleted for GKScore {}
pub trait GKScore_GKChallengeObsoleted: Sized + std::ops::Deref {
    unsafe fn issueChallengeToPlayers_message_(&self, playerIDs: NSArray, message: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, issueChallengeToPlayers : playerIDs, message : message)
    }
}
impl GKAchievement_GKChallengeObsoleted for GKAchievement {}
pub trait GKAchievement_GKChallengeObsoleted: Sized + std::ops::Deref {
    unsafe fn issueChallengeToPlayers_message_(&self, playerIDs: NSArray, message: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, issueChallengeToPlayers : playerIDs, message : message)
    }
    unsafe fn selectChallengeablePlayerIDs_withCompletionHandler_(
        &self,
        playerIDs: NSArray,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, selectChallengeablePlayerIDs : playerIDs, withCompletionHandler : completionHandler)
    }
}
pub type GKChallengeComposeCompletionBlock = *mut ::std::os::raw::c_void;
pub type GKChallengeComposeHandler = *mut ::std::os::raw::c_void;
impl GKScore_GKChallengeUI for GKScore {}
pub trait GKScore_GKChallengeUI: Sized + std::ops::Deref {
    unsafe fn challengeComposeControllerWithMessage_players_completionHandler_(
        &self,
        message: NSString,
        players: NSArray,
        completionHandler: GKChallengeComposeCompletionBlock,
    ) -> NSViewController
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, challengeComposeControllerWithMessage : message, players : players, completionHandler : completionHandler)
    }
    unsafe fn challengeComposeControllerWithMessage_players_completion_(
        &self,
        message: NSString,
        players: NSArray,
        completionHandler: GKChallengeComposeHandler,
    ) -> NSViewController
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, challengeComposeControllerWithMessage : message, players : players, completion : completionHandler)
    }
}
impl GKLeaderboardEntry_GKChallengeUI for GKLeaderboardEntry {}
pub trait GKLeaderboardEntry_GKChallengeUI: Sized + std::ops::Deref {
    unsafe fn challengeComposeControllerWithMessage_players_completionHandler_(
        &self,
        message: NSString,
        players: NSArray,
        completionHandler: GKChallengeComposeCompletionBlock,
    ) -> NSViewController
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, challengeComposeControllerWithMessage : message, players : players, completionHandler : completionHandler)
    }
    unsafe fn challengeComposeControllerWithMessage_players_completion_(
        &self,
        message: NSString,
        players: NSArray,
        completionHandler: GKChallengeComposeHandler,
    ) -> NSViewController
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, challengeComposeControllerWithMessage : message, players : players, completion : completionHandler)
    }
}
impl GKAchievement_GKChallengeUI for GKAchievement {}
pub trait GKAchievement_GKChallengeUI: Sized + std::ops::Deref {
    unsafe fn challengeComposeControllerWithMessage_players_completionHandler_(
        &self,
        message: NSString,
        players: NSArray,
        completionHandler: GKChallengeComposeCompletionBlock,
    ) -> NSViewController
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, challengeComposeControllerWithMessage : message, players : players, completionHandler : completionHandler)
    }
    unsafe fn challengeComposeControllerWithMessage_players_completion_(
        &self,
        message: NSString,
        players: NSArray,
        completionHandler: GKChallengeComposeHandler,
    ) -> NSViewController
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, challengeComposeControllerWithMessage : message, players : players, completion : completionHandler)
    }
}
impl GKScore_GKChallengeObsoletedUI for GKScore {}
pub trait GKScore_GKChallengeObsoletedUI: Sized + std::ops::Deref {
    unsafe fn challengeComposeControllerWithPlayers_message_completionHandler_(
        &self,
        playerIDs: NSArray,
        message: NSString,
        completionHandler: GKChallengeComposeCompletionBlock,
    ) -> NSViewController
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, challengeComposeControllerWithPlayers : playerIDs, message : message, completionHandler : completionHandler)
    }
}
impl GKAchievement_GKChallengeObsoletedUI for GKAchievement {}
pub trait GKAchievement_GKChallengeObsoletedUI: Sized + std::ops::Deref {
    unsafe fn challengeComposeControllerWithPlayers_message_completionHandler_(
        &self,
        playerIDs: NSArray,
        message: NSString,
        completionHandler: GKChallengeComposeCompletionBlock,
    ) -> NSViewController
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, challengeComposeControllerWithPlayers : playerIDs, message : message, completionHandler : completionHandler)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct GKChallengeDefinition(pub id);
impl std::ops::Deref for GKChallengeDefinition {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for GKChallengeDefinition {}
impl GKChallengeDefinition {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"GKChallengeDefinition").unwrap(), alloc) })
    }
}
impl INSObject for GKChallengeDefinition {}
impl PNSObject for GKChallengeDefinition {}
impl std::convert::TryFrom<NSObject> for GKChallengeDefinition {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<GKChallengeDefinition, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"GKChallengeDefinition").unwrap()) };
        if is_kind_of {
            Ok(GKChallengeDefinition(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to GKChallengeDefinition")
        }
    }
}
impl IGKChallengeDefinition for GKChallengeDefinition {}
pub trait IGKChallengeDefinition: Sized + std::ops::Deref {
    unsafe fn loadImageWithCompletionHandler_(&self, completionHandler: *mut ::std::os::raw::c_void)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, loadImageWithCompletionHandler : completionHandler)
    }
    unsafe fn identifier(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, identifier)
    }
    unsafe fn groupIdentifier(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, groupIdentifier)
    }
    unsafe fn title(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, title)
    }
    unsafe fn details(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, details)
    }
    unsafe fn durationOptions(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, durationOptions)
    }
    unsafe fn isRepeatable(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isRepeatable)
    }
    unsafe fn leaderboard(&self) -> GKLeaderboard
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, leaderboard)
    }
    unsafe fn releaseState(&self) -> GKReleaseState
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, releaseState)
    }
}
impl GKChallengeDefinition_State for GKChallengeDefinition {}
pub trait GKChallengeDefinition_State: Sized + std::ops::Deref {
    unsafe fn hasActiveChallengesWithCompletionHandler_(
        &self,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, hasActiveChallengesWithCompletionHandler : completionHandler)
    }
    unsafe fn loadChallengeDefinitionsWithCompletionHandler_(
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"GKChallengeDefinition").unwrap(), loadChallengeDefinitionsWithCompletionHandler : completionHandler)
    }
}
pub trait PGKChallengeEventHandlerDelegate: Sized + std::ops::Deref {
    unsafe fn localPlayerDidSelectChallenge_(&self, challenge: GKChallenge)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, localPlayerDidSelectChallenge : challenge)
    }
    unsafe fn shouldShowBannerForLocallyReceivedChallenge_(&self, challenge: GKChallenge) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, shouldShowBannerForLocallyReceivedChallenge : challenge)
    }
    unsafe fn localPlayerDidReceiveChallenge_(&self, challenge: GKChallenge)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, localPlayerDidReceiveChallenge : challenge)
    }
    unsafe fn shouldShowBannerForLocallyCompletedChallenge_(&self, challenge: GKChallenge) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, shouldShowBannerForLocallyCompletedChallenge : challenge)
    }
    unsafe fn localPlayerDidCompleteChallenge_(&self, challenge: GKChallenge)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, localPlayerDidCompleteChallenge : challenge)
    }
    unsafe fn shouldShowBannerForRemotelyCompletedChallenge_(&self, challenge: GKChallenge) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, shouldShowBannerForRemotelyCompletedChallenge : challenge)
    }
    unsafe fn remotePlayerDidCompleteChallenge_(&self, challenge: GKChallenge)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, remotePlayerDidCompleteChallenge : challenge)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct GKChallengeEventHandler(pub id);
impl std::ops::Deref for GKChallengeEventHandler {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for GKChallengeEventHandler {}
impl GKChallengeEventHandler {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"GKChallengeEventHandler").unwrap(), alloc) })
    }
}
impl INSObject for GKChallengeEventHandler {}
impl PNSObject for GKChallengeEventHandler {}
impl std::convert::TryFrom<NSObject> for GKChallengeEventHandler {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<GKChallengeEventHandler, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"GKChallengeEventHandler").unwrap()) };
        if is_kind_of {
            Ok(GKChallengeEventHandler(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to GKChallengeEventHandler")
        }
    }
}
impl IGKChallengeEventHandler for GKChallengeEventHandler {}
pub trait IGKChallengeEventHandler: Sized + std::ops::Deref {
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
    unsafe fn challengeEventHandler() -> GKChallengeEventHandler
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"GKChallengeEventHandler").unwrap(), challengeEventHandler)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct GKChallengesViewController(pub id);
impl std::ops::Deref for GKChallengesViewController {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for GKChallengesViewController {}
impl GKChallengesViewController {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"GKChallengesViewController").unwrap(), alloc) })
    }
}
impl PGKViewController for GKChallengesViewController {}
impl INSViewController for GKChallengesViewController {}
impl PNSEditor for GKChallengesViewController {}
impl PNSSeguePerforming for GKChallengesViewController {}
impl PNSUserInterfaceItemIdentification for GKChallengesViewController {}
impl std::convert::TryFrom<NSViewController> for GKChallengesViewController {
    type Error = &'static str;
    fn try_from(parent: NSViewController) -> Result<GKChallengesViewController, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"GKChallengesViewController").unwrap()) };
        if is_kind_of {
            Ok(GKChallengesViewController(parent.0))
        } else {
            Err("This NSViewController cannot be downcasted to GKChallengesViewController")
        }
    }
}
impl INSResponder for GKChallengesViewController {}
impl PNSCoding for GKChallengesViewController {}
impl INSObject for GKChallengesViewController {}
impl PNSObject for GKChallengesViewController {}
impl IGKChallengesViewController for GKChallengesViewController {}
pub trait IGKChallengesViewController: Sized + std::ops::Deref {
    unsafe fn challengeDelegate(&self) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, challengeDelegate)
    }
    unsafe fn setChallengeDelegate_(&self, challengeDelegate: *mut u64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setChallengeDelegate : challengeDelegate)
    }
}
pub trait PGKChallengesViewControllerDelegate: Sized + std::ops::Deref {
    unsafe fn challengesViewControllerDidFinish_(&self, viewController: GKChallengesViewController)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, challengesViewControllerDidFinish : viewController)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct GKCloudPlayer(pub id);
impl std::ops::Deref for GKCloudPlayer {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for GKCloudPlayer {}
impl GKCloudPlayer {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"GKCloudPlayer").unwrap(), alloc) })
    }
}
impl IGKBasePlayer for GKCloudPlayer {}
impl PNSCopying for GKCloudPlayer {}
impl std::convert::TryFrom<GKBasePlayer> for GKCloudPlayer {
    type Error = &'static str;
    fn try_from(parent: GKBasePlayer) -> Result<GKCloudPlayer, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"GKCloudPlayer").unwrap()) };
        if is_kind_of {
            Ok(GKCloudPlayer(parent.0))
        } else {
            Err("This GKBasePlayer cannot be downcasted to GKCloudPlayer")
        }
    }
}
impl INSObject for GKCloudPlayer {}
impl PNSObject for GKCloudPlayer {}
impl IGKCloudPlayer for GKCloudPlayer {}
pub trait IGKCloudPlayer: Sized + std::ops::Deref {
    unsafe fn getCurrentSignedInPlayerForContainer_completionHandler_(
        containerName: NSString,
        handler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"GKCloudPlayer").unwrap(), getCurrentSignedInPlayerForContainer : containerName, completionHandler : handler)
    }
}
pub trait PGKChallengeListener: Sized + std::ops::Deref {
    unsafe fn player_wantsToPlayChallenge_(&self, player: GKPlayer, challenge: GKChallenge)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, player : player, wantsToPlayChallenge : challenge)
    }
    unsafe fn player_didReceiveChallenge_(&self, player: GKPlayer, challenge: GKChallenge)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, player : player, didReceiveChallenge : challenge)
    }
    unsafe fn player_didCompleteChallenge_issuedByFriend_(
        &self,
        player: GKPlayer,
        challenge: GKChallenge,
        friendPlayer: GKPlayer,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, player : player, didCompleteChallenge : challenge, issuedByFriend : friendPlayer)
    }
    unsafe fn player_issuedChallengeWasCompleted_byFriend_(
        &self,
        player: GKPlayer,
        challenge: GKChallenge,
        friendPlayer: GKPlayer,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, player : player, issuedChallengeWasCompleted : challenge, byFriend : friendPlayer)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct GKFriendRequestComposeViewController(pub id);
impl std::ops::Deref for GKFriendRequestComposeViewController {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for GKFriendRequestComposeViewController {}
impl GKFriendRequestComposeViewController {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"GKFriendRequestComposeViewController").unwrap(), alloc) })
    }
}
impl PGKViewController for GKFriendRequestComposeViewController {}
impl INSViewController for GKFriendRequestComposeViewController {}
impl PNSEditor for GKFriendRequestComposeViewController {}
impl PNSSeguePerforming for GKFriendRequestComposeViewController {}
impl PNSUserInterfaceItemIdentification for GKFriendRequestComposeViewController {}
impl std::convert::TryFrom<NSViewController> for GKFriendRequestComposeViewController {
    type Error = &'static str;
    fn try_from(
        parent: NSViewController,
    ) -> Result<GKFriendRequestComposeViewController, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"GKFriendRequestComposeViewController").unwrap())
        };
        if is_kind_of {
            Ok(GKFriendRequestComposeViewController(parent.0))
        } else {
            Err ("This NSViewController cannot be downcasted to GKFriendRequestComposeViewController" ,)
        }
    }
}
impl INSResponder for GKFriendRequestComposeViewController {}
impl PNSCoding for GKFriendRequestComposeViewController {}
impl INSObject for GKFriendRequestComposeViewController {}
impl PNSObject for GKFriendRequestComposeViewController {}
impl IGKFriendRequestComposeViewController for GKFriendRequestComposeViewController {}
pub trait IGKFriendRequestComposeViewController: Sized + std::ops::Deref {}
impl GKFriendRequestComposeViewController_ for GKFriendRequestComposeViewController {}
pub trait GKFriendRequestComposeViewController_: Sized + std::ops::Deref {
    unsafe fn setMessage_(&self, message: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMessage : message)
    }
    unsafe fn addRecipientPlayers_(&self, players: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, addRecipientPlayers : players)
    }
    unsafe fn addRecipientsWithPlayerIDs_(&self, playerIDs: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, addRecipientsWithPlayerIDs : playerIDs)
    }
    unsafe fn addRecipientsWithEmailAddresses_(&self, emailAddresses: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, addRecipientsWithEmailAddresses : emailAddresses)
    }
    unsafe fn composeViewDelegate(&self) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, composeViewDelegate)
    }
    unsafe fn setComposeViewDelegate_(&self, composeViewDelegate: *mut u64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setComposeViewDelegate : composeViewDelegate)
    }
    unsafe fn maxNumberOfRecipients() -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"GKFriendRequestComposeViewController").unwrap(), maxNumberOfRecipients)
    }
}
pub trait PGKFriendRequestComposeViewControllerDelegate: Sized + std::ops::Deref {
    unsafe fn friendRequestComposeViewControllerDidFinish_(
        &self,
        viewController: GKFriendRequestComposeViewController,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, friendRequestComposeViewControllerDidFinish : viewController)
    }
}
pub type GKGameActivityState = NSUInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct GKGameActivity(pub id);
impl std::ops::Deref for GKGameActivity {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for GKGameActivity {}
impl GKGameActivity {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"GKGameActivity").unwrap(), alloc) })
    }
}
impl INSObject for GKGameActivity {}
impl PNSObject for GKGameActivity {}
impl std::convert::TryFrom<NSObject> for GKGameActivity {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<GKGameActivity, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"GKGameActivity").unwrap()) };
        if is_kind_of {
            Ok(GKGameActivity(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to GKGameActivity")
        }
    }
}
impl IGKGameActivity for GKGameActivity {}
pub trait IGKGameActivity: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn initWithDefinition_(
        &self,
        activityDefinition: GKGameActivityDefinition,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithDefinition : activityDefinition)
    }
    unsafe fn start(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, start)
    }
    unsafe fn pause(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, pause)
    }
    unsafe fn resume(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, resume)
    }
    unsafe fn end(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, end)
    }
    unsafe fn setScoreOnLeaderboard_toScore_context_(
        &self,
        leaderboard: GKLeaderboard,
        score: NSInteger,
        context: NSUInteger,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setScoreOnLeaderboard : leaderboard, toScore : score, context : context)
    }
    unsafe fn setScoreOnLeaderboard_toScore_(&self, leaderboard: GKLeaderboard, score: NSInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setScoreOnLeaderboard : leaderboard, toScore : score)
    }
    unsafe fn getScoreOnLeaderboard_(&self, leaderboard: GKLeaderboard) -> GKLeaderboardScore
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, getScoreOnLeaderboard : leaderboard)
    }
    unsafe fn removeScoresFromLeaderboards_(&self, leaderboards: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, removeScoresFromLeaderboards : leaderboards)
    }
    unsafe fn setProgressOnAchievement_toPercentComplete_(
        &self,
        achievement: GKAchievement,
        percentComplete: f64,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setProgressOnAchievement : achievement, toPercentComplete : percentComplete)
    }
    unsafe fn setAchievementCompleted_(&self, achievement: GKAchievement)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAchievementCompleted : achievement)
    }
    unsafe fn getProgressOnAchievement_(&self, achievement: GKAchievement) -> f64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, getProgressOnAchievement : achievement)
    }
    unsafe fn removeAchievements_(&self, achievements: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, removeAchievements : achievements)
    }
    unsafe fn identifier(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, identifier)
    }
    unsafe fn activityDefinition(&self) -> GKGameActivityDefinition
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, activityDefinition)
    }
    unsafe fn properties(&self) -> NSDictionary
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, properties)
    }
    unsafe fn setProperties_(&self, properties: NSDictionary)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setProperties : properties)
    }
    unsafe fn state(&self) -> GKGameActivityState
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, state)
    }
    unsafe fn partyCode(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, partyCode)
    }
    unsafe fn partyURL(&self) -> NSURL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, partyURL)
    }
    unsafe fn creationDate(&self) -> NSDate
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, creationDate)
    }
    unsafe fn startDate(&self) -> NSDate
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, startDate)
    }
    unsafe fn lastResumeDate(&self) -> NSDate
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, lastResumeDate)
    }
    unsafe fn endDate(&self) -> NSDate
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, endDate)
    }
    unsafe fn duration(&self) -> NSTimeInterval
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, duration)
    }
    unsafe fn achievements(&self) -> NSSet
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, achievements)
    }
    unsafe fn leaderboardScores(&self) -> NSSet
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, leaderboardScores)
    }
    unsafe fn startWithDefinition_partyCode_error_(
        activityDefinition: GKGameActivityDefinition,
        partyCode: NSString,
        error: *mut NSError,
    ) -> GKGameActivity
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"GKGameActivity").unwrap(), startWithDefinition : activityDefinition, partyCode : partyCode, error : error)
    }
    unsafe fn startWithDefinition_error_(
        activityDefinition: GKGameActivityDefinition,
        error: *mut NSError,
    ) -> GKGameActivity
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"GKGameActivity").unwrap(), startWithDefinition : activityDefinition, error : error)
    }
    unsafe fn isValidPartyCode_(partyCode: NSString) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"GKGameActivity").unwrap(), isValidPartyCode : partyCode)
    }
    unsafe fn validPartyCodeAlphabet() -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"GKGameActivity").unwrap(), validPartyCodeAlphabet)
    }
}
impl GKGameActivity_Multiplayer for GKGameActivity {}
pub trait GKGameActivity_Multiplayer: Sized + std::ops::Deref {
    unsafe fn makeMatchRequest(&self) -> GKMatchRequest
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, makeMatchRequest)
    }
    unsafe fn findMatchWithCompletionHandler_(&self, completionHandler: *mut ::std::os::raw::c_void)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, findMatchWithCompletionHandler : completionHandler)
    }
    unsafe fn findPlayersForHostedMatchWithCompletionHandler_(
        &self,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, findPlayersForHostedMatchWithCompletionHandler : completionHandler)
    }
}
impl GKGameActivity_State for GKGameActivity {}
pub trait GKGameActivity_State: Sized + std::ops::Deref {
    unsafe fn checkPendingGameActivityExistenceWithCompletionHandler_(
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"GKGameActivity").unwrap(), checkPendingGameActivityExistenceWithCompletionHandler : completionHandler)
    }
}
pub type GKGameActivityPlayStyle = NSInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct GKGameActivityDefinition(pub id);
impl std::ops::Deref for GKGameActivityDefinition {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for GKGameActivityDefinition {}
impl GKGameActivityDefinition {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"GKGameActivityDefinition").unwrap(), alloc) })
    }
}
impl INSObject for GKGameActivityDefinition {}
impl PNSObject for GKGameActivityDefinition {}
impl std::convert::TryFrom<NSObject> for GKGameActivityDefinition {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<GKGameActivityDefinition, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"GKGameActivityDefinition").unwrap()) };
        if is_kind_of {
            Ok(GKGameActivityDefinition(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to GKGameActivityDefinition")
        }
    }
}
impl IGKGameActivityDefinition for GKGameActivityDefinition {}
pub trait IGKGameActivityDefinition: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn loadAchievementDescriptionsWithCompletionHandler_(
        &self,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, loadAchievementDescriptionsWithCompletionHandler : completionHandler)
    }
    unsafe fn loadLeaderboardsWithCompletionHandler_(
        &self,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, loadLeaderboardsWithCompletionHandler : completionHandler)
    }
    unsafe fn loadImageWithCompletionHandler_(&self, completionHandler: *mut ::std::os::raw::c_void)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, loadImageWithCompletionHandler : completionHandler)
    }
    unsafe fn identifier(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, identifier)
    }
    unsafe fn groupIdentifier(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, groupIdentifier)
    }
    unsafe fn title(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, title)
    }
    unsafe fn details(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, details)
    }
    unsafe fn defaultProperties(&self) -> NSDictionary
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, defaultProperties)
    }
    unsafe fn fallbackURL(&self) -> NSURL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, fallbackURL)
    }
    unsafe fn supportsPartyCode(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, supportsPartyCode)
    }
    unsafe fn maxPlayers(&self) -> NSNumber
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, maxPlayers)
    }
    unsafe fn minPlayers(&self) -> NSNumber
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, minPlayers)
    }
    unsafe fn supportsUnlimitedPlayers(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, supportsUnlimitedPlayers)
    }
    unsafe fn playStyle(&self) -> GKGameActivityPlayStyle
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, playStyle)
    }
    unsafe fn releaseState(&self) -> GKReleaseState
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, releaseState)
    }
}
impl GKGameActivityDefinition_State for GKGameActivityDefinition {}
pub trait GKGameActivityDefinition_State: Sized + std::ops::Deref {
    unsafe fn loadGameActivityDefinitionsWithCompletionHandler_(
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"GKGameActivityDefinition").unwrap(), loadGameActivityDefinitionsWithCompletionHandler : completionHandler)
    }
    unsafe fn loadGameActivityDefinitionsWithIDs_completionHandler_(
        activityDefinitionIDs: NSArray,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"GKGameActivityDefinition").unwrap(), loadGameActivityDefinitionsWithIDs : activityDefinitionIDs, completionHandler : completionHandler)
    }
}
pub trait PGKGameActivityListener: Sized + std::ops::Deref {
    unsafe fn player_wantsToPlayGameActivity_completionHandler_(
        &self,
        player: GKPlayer,
        activity: GKGameActivity,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, player : player, wantsToPlayGameActivity : activity, completionHandler : completionHandler)
    }
}
pub type GKConnectionState = NSInteger;
pub type GKTransportType = NSInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct GKGameSession(pub id);
impl std::ops::Deref for GKGameSession {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for GKGameSession {}
impl GKGameSession {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"GKGameSession").unwrap(), alloc) })
    }
}
impl INSObject for GKGameSession {}
impl PNSObject for GKGameSession {}
impl std::convert::TryFrom<NSObject> for GKGameSession {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<GKGameSession, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"GKGameSession").unwrap()) };
        if is_kind_of {
            Ok(GKGameSession(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to GKGameSession")
        }
    }
}
impl IGKGameSession for GKGameSession {}
pub trait IGKGameSession: Sized + std::ops::Deref {
    unsafe fn getShareURLWithCompletionHandler_(
        &self,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, getShareURLWithCompletionHandler : completionHandler)
    }
    unsafe fn loadDataWithCompletionHandler_(&self, completionHandler: *mut ::std::os::raw::c_void)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, loadDataWithCompletionHandler : completionHandler)
    }
    unsafe fn saveData_completionHandler_(
        &self,
        data: NSData,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, saveData : data, completionHandler : completionHandler)
    }
    unsafe fn setConnectionState_completionHandler_(
        &self,
        state: GKConnectionState,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setConnectionState : state, completionHandler : completionHandler)
    }
    unsafe fn playersWithConnectionState_(&self, state: GKConnectionState) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, playersWithConnectionState : state)
    }
    unsafe fn sendData_withTransportType_completionHandler_(
        &self,
        data: NSData,
        transport: GKTransportType,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, sendData : data, withTransportType : transport, completionHandler : completionHandler)
    }
    unsafe fn sendMessageWithLocalizedFormatKey_arguments_data_toPlayers_badgePlayers_completionHandler_(
        &self,
        key: NSString,
        arguments: NSArray,
        data: NSData,
        players: NSArray,
        badgePlayers: BOOL,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, sendMessageWithLocalizedFormatKey : key, arguments : arguments, data : data, toPlayers : players, badgePlayers : badgePlayers, completionHandler : completionHandler)
    }
    unsafe fn clearBadgeForPlayers_completionHandler_(
        &self,
        players: NSArray,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, clearBadgeForPlayers : players, completionHandler : completionHandler)
    }
    unsafe fn identifier(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, identifier)
    }
    unsafe fn title(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, title)
    }
    unsafe fn owner(&self) -> GKCloudPlayer
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, owner)
    }
    unsafe fn players(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, players)
    }
    unsafe fn lastModifiedDate(&self) -> NSDate
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, lastModifiedDate)
    }
    unsafe fn lastModifiedPlayer(&self) -> GKCloudPlayer
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, lastModifiedPlayer)
    }
    unsafe fn maxNumberOfConnectedPlayers(&self) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, maxNumberOfConnectedPlayers)
    }
    unsafe fn badgedPlayers(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, badgedPlayers)
    }
    unsafe fn createSessionInContainer_withTitle_maxConnectedPlayers_completionHandler_(
        containerName: NSString,
        title: NSString,
        maxPlayers: NSInteger,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"GKGameSession").unwrap(), createSessionInContainer : containerName, withTitle : title, maxConnectedPlayers : maxPlayers, completionHandler : completionHandler)
    }
    unsafe fn loadSessionsInContainer_completionHandler_(
        containerName: NSString,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"GKGameSession").unwrap(), loadSessionsInContainer : containerName, completionHandler : completionHandler)
    }
    unsafe fn loadSessionWithIdentifier_completionHandler_(
        identifier: NSString,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"GKGameSession").unwrap(), loadSessionWithIdentifier : identifier, completionHandler : completionHandler)
    }
    unsafe fn removeSessionWithIdentifier_completionHandler_(
        identifier: NSString,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"GKGameSession").unwrap(), removeSessionWithIdentifier : identifier, completionHandler : completionHandler)
    }
}
pub type GKGameSessionErrorCode = NSInteger;
pub trait PGKGameSessionEventListener: Sized + std::ops::Deref {
    unsafe fn session_didAddPlayer_(&self, session: GKGameSession, player: GKCloudPlayer)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, session : session, didAddPlayer : player)
    }
    unsafe fn session_didRemovePlayer_(&self, session: GKGameSession, player: GKCloudPlayer)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, session : session, didRemovePlayer : player)
    }
    unsafe fn session_player_didChangeConnectionState_(
        &self,
        session: GKGameSession,
        player: GKCloudPlayer,
        newState: GKConnectionState,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, session : session, player : player, didChangeConnectionState : newState)
    }
    unsafe fn session_player_didSaveData_(
        &self,
        session: GKGameSession,
        player: GKCloudPlayer,
        data: NSData,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, session : session, player : player, didSaveData : data)
    }
    unsafe fn session_didReceiveData_fromPlayer_(
        &self,
        session: GKGameSession,
        data: NSData,
        player: GKCloudPlayer,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, session : session, didReceiveData : data, fromPlayer : player)
    }
    unsafe fn session_didReceiveMessage_withData_fromPlayer_(
        &self,
        session: GKGameSession,
        message: NSString,
        data: NSData,
        player: GKCloudPlayer,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, session : session, didReceiveMessage : message, withData : data, fromPlayer : player)
    }
}
impl GKGameSession_GKGameSessionEventListener for GKGameSession {}
pub trait GKGameSession_GKGameSessionEventListener: Sized + std::ops::Deref {
    unsafe fn addEventListener_(listener: NSObject)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"GKGameSession").unwrap(), addEventListener : listener)
    }
    unsafe fn removeEventListener_(listener: NSObject)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"GKGameSession").unwrap(), removeEventListener : listener)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct GKLeaderboardSet(pub id);
impl std::ops::Deref for GKLeaderboardSet {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for GKLeaderboardSet {}
impl GKLeaderboardSet {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"GKLeaderboardSet").unwrap(), alloc) })
    }
}
impl PNSCoding for GKLeaderboardSet {}
impl PNSSecureCoding for GKLeaderboardSet {}
impl INSObject for GKLeaderboardSet {}
impl PNSObject for GKLeaderboardSet {}
impl std::convert::TryFrom<NSObject> for GKLeaderboardSet {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<GKLeaderboardSet, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"GKLeaderboardSet").unwrap()) };
        if is_kind_of {
            Ok(GKLeaderboardSet(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to GKLeaderboardSet")
        }
    }
}
impl IGKLeaderboardSet for GKLeaderboardSet {}
pub trait IGKLeaderboardSet: Sized + std::ops::Deref {
    unsafe fn loadLeaderboardsWithHandler_(&self, handler: *mut ::std::os::raw::c_void)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, loadLeaderboardsWithHandler : handler)
    }
    unsafe fn title(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, title)
    }
    unsafe fn groupIdentifier(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, groupIdentifier)
    }
    unsafe fn identifier(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, identifier)
    }
    unsafe fn setIdentifier_(&self, identifier: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setIdentifier : identifier)
    }
    unsafe fn loadLeaderboardSetsWithCompletionHandler_(
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"GKLeaderboardSet").unwrap(), loadLeaderboardSetsWithCompletionHandler : completionHandler)
    }
}
impl GKLeaderboardSet_Deprecated for GKLeaderboardSet {}
pub trait GKLeaderboardSet_Deprecated: Sized + std::ops::Deref {
    unsafe fn loadLeaderboardsWithCompletionHandler_(
        &self,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, loadLeaderboardsWithCompletionHandler : completionHandler)
    }
}
impl GKLeaderboardSet_UI for GKLeaderboardSet {}
pub trait GKLeaderboardSet_UI: Sized + std::ops::Deref {
    unsafe fn loadImageWithCompletionHandler_(&self, completionHandler: *mut ::std::os::raw::c_void)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, loadImageWithCompletionHandler : completionHandler)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct GKLeaderboardViewController(pub id);
impl std::ops::Deref for GKLeaderboardViewController {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for GKLeaderboardViewController {}
impl GKLeaderboardViewController {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"GKLeaderboardViewController").unwrap(), alloc) })
    }
}
impl IGKGameCenterViewController for GKLeaderboardViewController {}
impl PGKViewController for GKLeaderboardViewController {}
impl std::convert::TryFrom<GKGameCenterViewController> for GKLeaderboardViewController {
    type Error = &'static str;
    fn try_from(
        parent: GKGameCenterViewController,
    ) -> Result<GKLeaderboardViewController, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"GKLeaderboardViewController").unwrap()) };
        if is_kind_of {
            Ok(GKLeaderboardViewController(parent.0))
        } else {
            Err ("This GKGameCenterViewController cannot be downcasted to GKLeaderboardViewController" ,)
        }
    }
}
impl INSViewController for GKLeaderboardViewController {}
impl PNSEditor for GKLeaderboardViewController {}
impl PNSSeguePerforming for GKLeaderboardViewController {}
impl PNSUserInterfaceItemIdentification for GKLeaderboardViewController {}
impl INSResponder for GKLeaderboardViewController {}
impl PNSCoding for GKLeaderboardViewController {}
impl INSObject for GKLeaderboardViewController {}
impl PNSObject for GKLeaderboardViewController {}
impl IGKLeaderboardViewController for GKLeaderboardViewController {}
pub trait IGKLeaderboardViewController: Sized + std::ops::Deref {}
impl GKLeaderboardViewController_ for GKLeaderboardViewController {}
pub trait GKLeaderboardViewController_: Sized + std::ops::Deref {
    unsafe fn timeScope(&self) -> GKLeaderboardTimeScope
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, timeScope)
    }
    unsafe fn setTimeScope_(&self, timeScope: GKLeaderboardTimeScope)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setTimeScope : timeScope)
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
    unsafe fn leaderboardDelegate(&self) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, leaderboardDelegate)
    }
    unsafe fn setLeaderboardDelegate_(&self, leaderboardDelegate: *mut u64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setLeaderboardDelegate : leaderboardDelegate)
    }
}
pub trait PGKLeaderboardViewControllerDelegate: Sized + std::ops::Deref {
    unsafe fn leaderboardViewControllerDidFinish_(
        &self,
        viewController: GKLeaderboardViewController,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, leaderboardViewControllerDidFinish : viewController)
    }
}
pub type GKTurnBasedMatchStatus = NSInteger;
pub type GKTurnBasedParticipantStatus = NSInteger;
pub type GKTurnBasedMatchOutcome = NSInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct GKTurnBasedParticipant(pub id);
impl std::ops::Deref for GKTurnBasedParticipant {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for GKTurnBasedParticipant {}
impl GKTurnBasedParticipant {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"GKTurnBasedParticipant").unwrap(), alloc) })
    }
}
impl INSObject for GKTurnBasedParticipant {}
impl PNSObject for GKTurnBasedParticipant {}
impl std::convert::TryFrom<NSObject> for GKTurnBasedParticipant {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<GKTurnBasedParticipant, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"GKTurnBasedParticipant").unwrap()) };
        if is_kind_of {
            Ok(GKTurnBasedParticipant(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to GKTurnBasedParticipant")
        }
    }
}
impl IGKTurnBasedParticipant for GKTurnBasedParticipant {}
pub trait IGKTurnBasedParticipant: Sized + std::ops::Deref {
    unsafe fn player(&self) -> GKPlayer
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, player)
    }
    unsafe fn lastTurnDate(&self) -> NSDate
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, lastTurnDate)
    }
    unsafe fn status(&self) -> GKTurnBasedParticipantStatus
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, status)
    }
    unsafe fn matchOutcome(&self) -> GKTurnBasedMatchOutcome
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, matchOutcome)
    }
    unsafe fn setMatchOutcome_(&self, matchOutcome: GKTurnBasedMatchOutcome)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMatchOutcome : matchOutcome)
    }
    unsafe fn timeoutDate(&self) -> NSDate
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, timeoutDate)
    }
}
impl GKTurnBasedParticipant_Obsoleted for GKTurnBasedParticipant {}
pub trait GKTurnBasedParticipant_Obsoleted: Sized + std::ops::Deref {
    unsafe fn playerID(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, playerID)
    }
}
pub trait PGKTurnBasedEventListener: Sized + std::ops::Deref {
    unsafe fn player_didRequestMatchWithOtherPlayers_(
        &self,
        player: GKPlayer,
        playersToInvite: NSArray,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, player : player, didRequestMatchWithOtherPlayers : playersToInvite)
    }
    unsafe fn player_receivedTurnEventForMatch_didBecomeActive_(
        &self,
        player: GKPlayer,
        match_: GKTurnBasedMatch,
        didBecomeActive: BOOL,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, player : player, receivedTurnEventForMatch : match_, didBecomeActive : didBecomeActive)
    }
    unsafe fn player_matchEnded_(&self, player: GKPlayer, match_: GKTurnBasedMatch)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, player : player, matchEnded : match_)
    }
    unsafe fn player_receivedExchangeRequest_forMatch_(
        &self,
        player: GKPlayer,
        exchange: GKTurnBasedExchange,
        match_: GKTurnBasedMatch,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, player : player, receivedExchangeRequest : exchange, forMatch : match_)
    }
    unsafe fn player_receivedExchangeCancellation_forMatch_(
        &self,
        player: GKPlayer,
        exchange: GKTurnBasedExchange,
        match_: GKTurnBasedMatch,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, player : player, receivedExchangeCancellation : exchange, forMatch : match_)
    }
    unsafe fn player_receivedExchangeReplies_forCompletedExchange_forMatch_(
        &self,
        player: GKPlayer,
        replies: NSArray,
        exchange: GKTurnBasedExchange,
        match_: GKTurnBasedMatch,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, player : player, receivedExchangeReplies : replies, forCompletedExchange : exchange, forMatch : match_)
    }
    unsafe fn player_wantsToQuitMatch_(&self, player: GKPlayer, match_: GKTurnBasedMatch)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, player : player, wantsToQuitMatch : match_)
    }
    unsafe fn player_didRequestMatchWithPlayers_(
        &self,
        player: GKPlayer,
        playerIDsToInvite: NSArray,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, player : player, didRequestMatchWithPlayers : playerIDsToInvite)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct GKTurnBasedMatch(pub id);
impl std::ops::Deref for GKTurnBasedMatch {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for GKTurnBasedMatch {}
impl GKTurnBasedMatch {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"GKTurnBasedMatch").unwrap(), alloc) })
    }
}
impl INSObject for GKTurnBasedMatch {}
impl PNSObject for GKTurnBasedMatch {}
impl std::convert::TryFrom<NSObject> for GKTurnBasedMatch {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<GKTurnBasedMatch, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"GKTurnBasedMatch").unwrap()) };
        if is_kind_of {
            Ok(GKTurnBasedMatch(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to GKTurnBasedMatch")
        }
    }
}
impl IGKTurnBasedMatch for GKTurnBasedMatch {}
pub trait IGKTurnBasedMatch: Sized + std::ops::Deref {
    unsafe fn setLocalizableMessageWithKey_arguments_(&self, key: NSString, arguments: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setLocalizableMessageWithKey : key, arguments : arguments)
    }
    unsafe fn rematchWithCompletionHandler_(&self, completionHandler: *mut ::std::os::raw::c_void)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, rematchWithCompletionHandler : completionHandler)
    }
    unsafe fn acceptInviteWithCompletionHandler_(
        &self,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, acceptInviteWithCompletionHandler : completionHandler)
    }
    unsafe fn declineInviteWithCompletionHandler_(
        &self,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, declineInviteWithCompletionHandler : completionHandler)
    }
    unsafe fn removeWithCompletionHandler_(&self, completionHandler: *mut ::std::os::raw::c_void)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, removeWithCompletionHandler : completionHandler)
    }
    unsafe fn loadMatchDataWithCompletionHandler_(
        &self,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, loadMatchDataWithCompletionHandler : completionHandler)
    }
    unsafe fn endTurnWithNextParticipants_turnTimeout_matchData_completionHandler_(
        &self,
        nextParticipants: NSArray,
        timeout: NSTimeInterval,
        matchData: NSData,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, endTurnWithNextParticipants : nextParticipants, turnTimeout : timeout, matchData : matchData, completionHandler : completionHandler)
    }
    unsafe fn participantQuitInTurnWithOutcome_nextParticipants_turnTimeout_matchData_completionHandler_(
        &self,
        matchOutcome: GKTurnBasedMatchOutcome,
        nextParticipants: NSArray,
        timeout: NSTimeInterval,
        matchData: NSData,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, participantQuitInTurnWithOutcome : matchOutcome, nextParticipants : nextParticipants, turnTimeout : timeout, matchData : matchData, completionHandler : completionHandler)
    }
    unsafe fn participantQuitOutOfTurnWithOutcome_withCompletionHandler_(
        &self,
        matchOutcome: GKTurnBasedMatchOutcome,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, participantQuitOutOfTurnWithOutcome : matchOutcome, withCompletionHandler : completionHandler)
    }
    unsafe fn endMatchInTurnWithMatchData_completionHandler_(
        &self,
        matchData: NSData,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, endMatchInTurnWithMatchData : matchData, completionHandler : completionHandler)
    }
    unsafe fn endMatchInTurnWithMatchData_scores_achievements_completionHandler_(
        &self,
        matchData: NSData,
        scores: NSArray,
        achievements: NSArray,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, endMatchInTurnWithMatchData : matchData, scores : scores, achievements : achievements, completionHandler : completionHandler)
    }
    unsafe fn endMatchInTurnWithMatchData_leaderboardScores_achievements_completionHandler_(
        &self,
        matchData: NSData,
        scores: NSArray,
        achievements: NSArray,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, endMatchInTurnWithMatchData : matchData, leaderboardScores : scores, achievements : achievements, completionHandler : completionHandler)
    }
    unsafe fn saveCurrentTurnWithMatchData_completionHandler_(
        &self,
        matchData: NSData,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, saveCurrentTurnWithMatchData : matchData, completionHandler : completionHandler)
    }
    unsafe fn saveMergedMatchData_withResolvedExchanges_completionHandler_(
        &self,
        matchData: NSData,
        exchanges: NSArray,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, saveMergedMatchData : matchData, withResolvedExchanges : exchanges, completionHandler : completionHandler)
    }
    unsafe fn sendExchangeToParticipants_data_localizableMessageKey_arguments_timeout_completionHandler_(
        &self,
        participants: NSArray,
        data: NSData,
        key: NSString,
        arguments: NSArray,
        timeout: NSTimeInterval,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, sendExchangeToParticipants : participants, data : data, localizableMessageKey : key, arguments : arguments, timeout : timeout, completionHandler : completionHandler)
    }
    unsafe fn sendReminderToParticipants_localizableMessageKey_arguments_completionHandler_(
        &self,
        participants: NSArray,
        key: NSString,
        arguments: NSArray,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, sendReminderToParticipants : participants, localizableMessageKey : key, arguments : arguments, completionHandler : completionHandler)
    }
    unsafe fn endTurnWithNextParticipant_matchData_completionHandler_(
        &self,
        nextParticipant: GKTurnBasedParticipant,
        matchData: NSData,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, endTurnWithNextParticipant : nextParticipant, matchData : matchData, completionHandler : completionHandler)
    }
    unsafe fn participantQuitInTurnWithOutcome_nextParticipant_matchData_completionHandler_(
        &self,
        matchOutcome: GKTurnBasedMatchOutcome,
        nextParticipant: GKTurnBasedParticipant,
        matchData: NSData,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, participantQuitInTurnWithOutcome : matchOutcome, nextParticipant : nextParticipant, matchData : matchData, completionHandler : completionHandler)
    }
    unsafe fn matchID(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, matchID)
    }
    unsafe fn creationDate(&self) -> NSDate
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, creationDate)
    }
    unsafe fn participants(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, participants)
    }
    unsafe fn status(&self) -> GKTurnBasedMatchStatus
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, status)
    }
    unsafe fn currentParticipant(&self) -> GKTurnBasedParticipant
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, currentParticipant)
    }
    unsafe fn matchData(&self) -> NSData
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, matchData)
    }
    unsafe fn message(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, message)
    }
    unsafe fn setMessage_(&self, message: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMessage : message)
    }
    unsafe fn matchDataMaximumSize(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, matchDataMaximumSize)
    }
    unsafe fn exchanges(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, exchanges)
    }
    unsafe fn activeExchanges(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, activeExchanges)
    }
    unsafe fn completedExchanges(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, completedExchanges)
    }
    unsafe fn exchangeDataMaximumSize(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, exchangeDataMaximumSize)
    }
    unsafe fn exchangeMaxInitiatedExchangesPerPlayer(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, exchangeMaxInitiatedExchangesPerPlayer)
    }
    unsafe fn findMatchForRequest_withCompletionHandler_(
        request: GKMatchRequest,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"GKTurnBasedMatch").unwrap(), findMatchForRequest : request, withCompletionHandler : completionHandler)
    }
    unsafe fn loadMatchesWithCompletionHandler_(completionHandler: *mut ::std::os::raw::c_void)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"GKTurnBasedMatch").unwrap(), loadMatchesWithCompletionHandler : completionHandler)
    }
    unsafe fn loadMatchWithID_withCompletionHandler_(
        matchID: NSString,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"GKTurnBasedMatch").unwrap(), loadMatchWithID : matchID, withCompletionHandler : completionHandler)
    }
}
pub type GKTurnBasedExchangeStatus = i8;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct GKTurnBasedExchange(pub id);
impl std::ops::Deref for GKTurnBasedExchange {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for GKTurnBasedExchange {}
impl GKTurnBasedExchange {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"GKTurnBasedExchange").unwrap(), alloc) })
    }
}
impl INSObject for GKTurnBasedExchange {}
impl PNSObject for GKTurnBasedExchange {}
impl std::convert::TryFrom<NSObject> for GKTurnBasedExchange {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<GKTurnBasedExchange, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"GKTurnBasedExchange").unwrap()) };
        if is_kind_of {
            Ok(GKTurnBasedExchange(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to GKTurnBasedExchange")
        }
    }
}
impl IGKTurnBasedExchange for GKTurnBasedExchange {}
pub trait IGKTurnBasedExchange: Sized + std::ops::Deref {
    unsafe fn cancelWithLocalizableMessageKey_arguments_completionHandler_(
        &self,
        key: NSString,
        arguments: NSArray,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, cancelWithLocalizableMessageKey : key, arguments : arguments, completionHandler : completionHandler)
    }
    unsafe fn replyWithLocalizableMessageKey_arguments_data_completionHandler_(
        &self,
        key: NSString,
        arguments: NSArray,
        data: NSData,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, replyWithLocalizableMessageKey : key, arguments : arguments, data : data, completionHandler : completionHandler)
    }
    unsafe fn exchangeID(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, exchangeID)
    }
    unsafe fn sender(&self) -> GKTurnBasedParticipant
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, sender)
    }
    unsafe fn recipients(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, recipients)
    }
    unsafe fn status(&self) -> GKTurnBasedExchangeStatus
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, status)
    }
    unsafe fn message(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, message)
    }
    unsafe fn data(&self) -> NSData
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, data)
    }
    unsafe fn sendDate(&self) -> NSDate
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, sendDate)
    }
    unsafe fn timeoutDate(&self) -> NSDate
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, timeoutDate)
    }
    unsafe fn completionDate(&self) -> NSDate
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, completionDate)
    }
    unsafe fn replies(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, replies)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct GKTurnBasedExchangeReply(pub id);
impl std::ops::Deref for GKTurnBasedExchangeReply {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for GKTurnBasedExchangeReply {}
impl GKTurnBasedExchangeReply {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"GKTurnBasedExchangeReply").unwrap(), alloc) })
    }
}
impl INSObject for GKTurnBasedExchangeReply {}
impl PNSObject for GKTurnBasedExchangeReply {}
impl std::convert::TryFrom<NSObject> for GKTurnBasedExchangeReply {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<GKTurnBasedExchangeReply, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"GKTurnBasedExchangeReply").unwrap()) };
        if is_kind_of {
            Ok(GKTurnBasedExchangeReply(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to GKTurnBasedExchangeReply")
        }
    }
}
impl IGKTurnBasedExchangeReply for GKTurnBasedExchangeReply {}
pub trait IGKTurnBasedExchangeReply: Sized + std::ops::Deref {
    unsafe fn recipient(&self) -> GKTurnBasedParticipant
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, recipient)
    }
    unsafe fn message(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, message)
    }
    unsafe fn data(&self) -> NSData
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, data)
    }
    unsafe fn replyDate(&self) -> NSDate
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, replyDate)
    }
}
pub trait PGKTurnBasedEventHandlerDelegate: Sized + std::ops::Deref {
    unsafe fn handleInviteFromGameCenter_(&self, playersToInvite: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, handleInviteFromGameCenter : playersToInvite)
    }
    unsafe fn handleTurnEventForMatch_didBecomeActive_(
        &self,
        match_: GKTurnBasedMatch,
        didBecomeActive: BOOL,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, handleTurnEventForMatch : match_, didBecomeActive : didBecomeActive)
    }
    unsafe fn handleTurnEventForMatch_(&self, match_: GKTurnBasedMatch)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, handleTurnEventForMatch : match_)
    }
    unsafe fn handleMatchEnded_(&self, match_: GKTurnBasedMatch)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, handleMatchEnded : match_)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct GKTurnBasedEventHandler(pub id);
impl std::ops::Deref for GKTurnBasedEventHandler {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for GKTurnBasedEventHandler {}
impl GKTurnBasedEventHandler {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"GKTurnBasedEventHandler").unwrap(), alloc) })
    }
}
impl INSObject for GKTurnBasedEventHandler {}
impl PNSObject for GKTurnBasedEventHandler {}
impl std::convert::TryFrom<NSObject> for GKTurnBasedEventHandler {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<GKTurnBasedEventHandler, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"GKTurnBasedEventHandler").unwrap()) };
        if is_kind_of {
            Ok(GKTurnBasedEventHandler(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to GKTurnBasedEventHandler")
        }
    }
}
impl IGKTurnBasedEventHandler for GKTurnBasedEventHandler {}
pub trait IGKTurnBasedEventHandler: Sized + std::ops::Deref {
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
    unsafe fn sharedTurnBasedEventHandler() -> GKTurnBasedEventHandler
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"GKTurnBasedEventHandler").unwrap(), sharedTurnBasedEventHandler)
    }
}
pub type GKInviteRecipientResponse = NSInteger;
pub use self::GKInviteRecipientResponse as GKInviteeResponse;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct GKMatchRequest(pub id);
impl std::ops::Deref for GKMatchRequest {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for GKMatchRequest {}
impl GKMatchRequest {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"GKMatchRequest").unwrap(), alloc) })
    }
}
impl INSObject for GKMatchRequest {}
impl PNSObject for GKMatchRequest {}
impl std::convert::TryFrom<NSObject> for GKMatchRequest {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<GKMatchRequest, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"GKMatchRequest").unwrap()) };
        if is_kind_of {
            Ok(GKMatchRequest(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to GKMatchRequest")
        }
    }
}
impl IGKMatchRequest for GKMatchRequest {}
pub trait IGKMatchRequest: Sized + std::ops::Deref {
    unsafe fn minPlayers(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, minPlayers)
    }
    unsafe fn setMinPlayers_(&self, minPlayers: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMinPlayers : minPlayers)
    }
    unsafe fn maxPlayers(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, maxPlayers)
    }
    unsafe fn setMaxPlayers_(&self, maxPlayers: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMaxPlayers : maxPlayers)
    }
    unsafe fn playerGroup(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, playerGroup)
    }
    unsafe fn setPlayerGroup_(&self, playerGroup: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPlayerGroup : playerGroup)
    }
    unsafe fn playerAttributes(&self) -> u32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, playerAttributes)
    }
    unsafe fn setPlayerAttributes_(&self, playerAttributes: u32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPlayerAttributes : playerAttributes)
    }
    unsafe fn recipients(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, recipients)
    }
    unsafe fn setRecipients_(&self, recipients: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRecipients : recipients)
    }
    unsafe fn inviteMessage(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inviteMessage)
    }
    unsafe fn setInviteMessage_(&self, inviteMessage: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInviteMessage : inviteMessage)
    }
    unsafe fn defaultNumberOfPlayers(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, defaultNumberOfPlayers)
    }
    unsafe fn setDefaultNumberOfPlayers_(&self, defaultNumberOfPlayers: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDefaultNumberOfPlayers : defaultNumberOfPlayers)
    }
    unsafe fn restrictToAutomatch(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, restrictToAutomatch)
    }
    unsafe fn setRestrictToAutomatch_(&self, restrictToAutomatch: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRestrictToAutomatch : restrictToAutomatch)
    }
    unsafe fn recipientResponseHandler(&self) -> *mut ::std::os::raw::c_void
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, recipientResponseHandler)
    }
    unsafe fn setRecipientResponseHandler_(
        &self,
        recipientResponseHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRecipientResponseHandler : recipientResponseHandler)
    }
    unsafe fn inviteeResponseHandler(&self) -> *mut ::std::os::raw::c_void
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inviteeResponseHandler)
    }
    unsafe fn setInviteeResponseHandler_(&self, inviteeResponseHandler: *mut ::std::os::raw::c_void)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInviteeResponseHandler : inviteeResponseHandler)
    }
    unsafe fn playersToInvite(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, playersToInvite)
    }
    unsafe fn setPlayersToInvite_(&self, playersToInvite: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPlayersToInvite : playersToInvite)
    }
    unsafe fn queueName(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, queueName)
    }
    unsafe fn setQueueName_(&self, queueName: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setQueueName : queueName)
    }
    unsafe fn properties(&self) -> *mut GKMatchProperties
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, properties)
    }
    unsafe fn setProperties_(&self, properties: NSDictionary)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setProperties : properties)
    }
    unsafe fn recipientProperties(&self) -> NSDictionary
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, recipientProperties)
    }
    unsafe fn setRecipientProperties_(&self, recipientProperties: NSDictionary)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRecipientProperties : recipientProperties)
    }
    unsafe fn maxPlayersAllowedForMatchOfType_(matchType: GKMatchType) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"GKMatchRequest").unwrap(), maxPlayersAllowedForMatchOfType : matchType)
    }
}
pub type GKMatchType = NSUInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct GKInvite(pub id);
impl std::ops::Deref for GKInvite {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for GKInvite {}
impl GKInvite {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"GKInvite").unwrap(), alloc) })
    }
}
impl INSObject for GKInvite {}
impl PNSObject for GKInvite {}
impl std::convert::TryFrom<NSObject> for GKInvite {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<GKInvite, Self::Error> {
        let is_kind_of: bool = unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"GKInvite").unwrap()) };
        if is_kind_of {
            Ok(GKInvite(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to GKInvite")
        }
    }
}
impl IGKInvite for GKInvite {}
pub trait IGKInvite: Sized + std::ops::Deref {
    unsafe fn sender(&self) -> GKPlayer
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, sender)
    }
    unsafe fn isHosted(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isHosted)
    }
    unsafe fn playerGroup(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, playerGroup)
    }
    unsafe fn playerAttributes(&self) -> u32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, playerAttributes)
    }
    unsafe fn inviter(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inviter)
    }
}
pub trait PGKInviteEventListener: Sized + std::ops::Deref {
    unsafe fn player_didAcceptInvite_(&self, player: GKPlayer, invite: GKInvite)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, player : player, didAcceptInvite : invite)
    }
    unsafe fn player_didRequestMatchWithRecipients_(
        &self,
        player: GKPlayer,
        recipientPlayers: NSArray,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, player : player, didRequestMatchWithRecipients : recipientPlayers)
    }
    unsafe fn player_didRequestMatchWithPlayers_(
        &self,
        player: GKPlayer,
        playerIDsToInvite: NSArray,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, player : player, didRequestMatchWithPlayers : playerIDsToInvite)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct GKMatchedPlayers(pub id);
impl std::ops::Deref for GKMatchedPlayers {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for GKMatchedPlayers {}
impl GKMatchedPlayers {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"GKMatchedPlayers").unwrap(), alloc) })
    }
}
impl INSObject for GKMatchedPlayers {}
impl PNSObject for GKMatchedPlayers {}
impl std::convert::TryFrom<NSObject> for GKMatchedPlayers {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<GKMatchedPlayers, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"GKMatchedPlayers").unwrap()) };
        if is_kind_of {
            Ok(GKMatchedPlayers(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to GKMatchedPlayers")
        }
    }
}
impl IGKMatchedPlayers for GKMatchedPlayers {}
pub trait IGKMatchedPlayers: Sized + std::ops::Deref {
    unsafe fn properties(&self) -> *mut GKMatchProperties
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, properties)
    }
    unsafe fn players(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, players)
    }
    unsafe fn playerProperties(&self) -> NSDictionary
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, playerProperties)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct GKMatchmaker(pub id);
impl std::ops::Deref for GKMatchmaker {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for GKMatchmaker {}
impl GKMatchmaker {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"GKMatchmaker").unwrap(), alloc) })
    }
}
impl INSObject for GKMatchmaker {}
impl PNSObject for GKMatchmaker {}
impl std::convert::TryFrom<NSObject> for GKMatchmaker {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<GKMatchmaker, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"GKMatchmaker").unwrap()) };
        if is_kind_of {
            Ok(GKMatchmaker(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to GKMatchmaker")
        }
    }
}
impl IGKMatchmaker for GKMatchmaker {}
pub trait IGKMatchmaker: Sized + std::ops::Deref {
    unsafe fn matchForInvite_completionHandler_(
        &self,
        invite: GKInvite,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, matchForInvite : invite, completionHandler : completionHandler)
    }
    unsafe fn findMatchForRequest_withCompletionHandler_(
        &self,
        request: GKMatchRequest,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, findMatchForRequest : request, withCompletionHandler : completionHandler)
    }
    unsafe fn findPlayersForHostedRequest_withCompletionHandler_(
        &self,
        request: GKMatchRequest,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, findPlayersForHostedRequest : request, withCompletionHandler : completionHandler)
    }
    unsafe fn findMatchedPlayers_withCompletionHandler_(
        &self,
        request: GKMatchRequest,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, findMatchedPlayers : request, withCompletionHandler : completionHandler)
    }
    unsafe fn addPlayersToMatch_matchRequest_completionHandler_(
        &self,
        match_: GKMatch,
        matchRequest: GKMatchRequest,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, addPlayersToMatch : match_, matchRequest : matchRequest, completionHandler : completionHandler)
    }
    unsafe fn cancel(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, cancel)
    }
    unsafe fn cancelPendingInviteToPlayer_(&self, player: GKPlayer)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, cancelPendingInviteToPlayer : player)
    }
    unsafe fn finishMatchmakingForMatch_(&self, match_: GKMatch)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, finishMatchmakingForMatch : match_)
    }
    unsafe fn queryPlayerGroupActivity_withCompletionHandler_(
        &self,
        playerGroup: NSUInteger,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, queryPlayerGroupActivity : playerGroup, withCompletionHandler : completionHandler)
    }
    unsafe fn queryActivityWithCompletionHandler_(
        &self,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, queryActivityWithCompletionHandler : completionHandler)
    }
    unsafe fn queryQueueActivity_withCompletionHandler_(
        &self,
        queueName: NSString,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, queryQueueActivity : queueName, withCompletionHandler : completionHandler)
    }
    unsafe fn startBrowsingForNearbyPlayersWithHandler_(
        &self,
        reachableHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, startBrowsingForNearbyPlayersWithHandler : reachableHandler)
    }
    unsafe fn stopBrowsingForNearbyPlayers(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, stopBrowsingForNearbyPlayers)
    }
    unsafe fn startGroupActivityWithPlayerHandler_(&self, handler: *mut ::std::os::raw::c_void)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, startGroupActivityWithPlayerHandler : handler)
    }
    unsafe fn stopGroupActivity(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, stopGroupActivity)
    }
    unsafe fn sharedMatchmaker() -> GKMatchmaker
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"GKMatchmaker").unwrap(), sharedMatchmaker)
    }
}
impl GKMatchmaker_GKDeprecated for GKMatchmaker {}
pub trait GKMatchmaker_GKDeprecated: Sized + std::ops::Deref {
    unsafe fn inviteHandler(&self) -> *mut ::std::os::raw::c_void
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inviteHandler)
    }
    unsafe fn setInviteHandler_(&self, inviteHandler: *mut ::std::os::raw::c_void)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInviteHandler : inviteHandler)
    }
}
impl GKMatchmaker_Obsoleted for GKMatchmaker {}
pub trait GKMatchmaker_Obsoleted: Sized + std::ops::Deref {
    unsafe fn startBrowsingForNearbyPlayersWithReachableHandler_(
        &self,
        reachableHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, startBrowsingForNearbyPlayersWithReachableHandler : reachableHandler)
    }
    unsafe fn cancelInviteToPlayer_(&self, playerID: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, cancelInviteToPlayer : playerID)
    }
    unsafe fn findPlayersForHostedMatchRequest_withCompletionHandler_(
        &self,
        request: GKMatchRequest,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, findPlayersForHostedMatchRequest : request, withCompletionHandler : completionHandler)
    }
}
pub trait PGKSavedGameListener: Sized + std::ops::Deref {
    unsafe fn player_didModifySavedGame_(&self, player: GKPlayer, savedGame: GKSavedGame)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, player : player, didModifySavedGame : savedGame)
    }
    unsafe fn player_hasConflictingSavedGames_(&self, player: GKPlayer, savedGames: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, player : player, hasConflictingSavedGames : savedGames)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct GKLocalPlayer(pub id);
impl std::ops::Deref for GKLocalPlayer {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for GKLocalPlayer {}
impl GKLocalPlayer {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"GKLocalPlayer").unwrap(), alloc) })
    }
}
impl IGKPlayer for GKLocalPlayer {}
impl From<GKLocalPlayer> for GKPlayer {
    fn from(child: GKLocalPlayer) -> GKPlayer {
        GKPlayer(child.0)
    }
}
impl std::convert::TryFrom<GKPlayer> for GKLocalPlayer {
    type Error = &'static str;
    fn try_from(parent: GKPlayer) -> Result<GKLocalPlayer, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"GKLocalPlayer").unwrap()) };
        if is_kind_of {
            Ok(GKLocalPlayer(parent.0))
        } else {
            Err("This GKPlayer cannot be downcasted to GKLocalPlayer")
        }
    }
}
impl IGKBasePlayer for GKLocalPlayer {}
impl PNSCopying for GKLocalPlayer {}
impl INSObject for GKLocalPlayer {}
impl PNSObject for GKLocalPlayer {}
impl IGKLocalPlayer for GKLocalPlayer {}
pub trait IGKLocalPlayer: Sized + std::ops::Deref {
    unsafe fn loadRecentPlayersWithCompletionHandler_(
        &self,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, loadRecentPlayersWithCompletionHandler : completionHandler)
    }
    unsafe fn loadChallengableFriendsWithCompletionHandler_(
        &self,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, loadChallengableFriendsWithCompletionHandler : completionHandler)
    }
    unsafe fn fetchItemsForIdentityVerificationSignature_(
        &self,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, fetchItemsForIdentityVerificationSignature : completionHandler)
    }
    unsafe fn isAuthenticated(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isAuthenticated)
    }
    unsafe fn isUnderage(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isUnderage)
    }
    unsafe fn isMultiplayerGamingRestricted(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isMultiplayerGamingRestricted)
    }
    unsafe fn isPersonalizedCommunicationRestricted(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isPersonalizedCommunicationRestricted)
    }
    unsafe fn local() -> GKLocalPlayer
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"GKLocalPlayer").unwrap(), local)
    }
    unsafe fn localPlayer() -> GKLocalPlayer
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"GKLocalPlayer").unwrap(), localPlayer)
    }
}
pub trait PGKLocalPlayerListener: Sized + std::ops::Deref {}
impl GKLocalPlayer_GKLocalPlayerEvents for GKLocalPlayer {}
pub trait GKLocalPlayer_GKLocalPlayerEvents: Sized + std::ops::Deref {
    unsafe fn registerListener_(&self, listener: *mut u64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, registerListener : listener)
    }
    unsafe fn unregisterListener_(&self, listener: *mut u64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, unregisterListener : listener)
    }
    unsafe fn unregisterAllListeners(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, unregisterAllListeners)
    }
}
impl GKLocalPlayer_Deprecated for GKLocalPlayer {}
pub trait GKLocalPlayer_Deprecated: Sized + std::ops::Deref {
    unsafe fn setDefaultLeaderboardCategoryID_completionHandler_(
        &self,
        categoryID: NSString,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDefaultLeaderboardCategoryID : categoryID, completionHandler : completionHandler)
    }
    unsafe fn loadDefaultLeaderboardCategoryIDWithCompletionHandler_(
        &self,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, loadDefaultLeaderboardCategoryIDWithCompletionHandler : completionHandler)
    }
    unsafe fn authenticateWithCompletionHandler_(
        &self,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, authenticateWithCompletionHandler : completionHandler)
    }
    unsafe fn loadFriendPlayersWithCompletionHandler_(
        &self,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, loadFriendPlayersWithCompletionHandler : completionHandler)
    }
    unsafe fn generateIdentityVerificationSignatureWithCompletionHandler_(
        &self,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, generateIdentityVerificationSignatureWithCompletionHandler : completionHandler)
    }
    unsafe fn loadDefaultLeaderboardIdentifierWithCompletionHandler_(
        &self,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, loadDefaultLeaderboardIdentifierWithCompletionHandler : completionHandler)
    }
    unsafe fn setDefaultLeaderboardIdentifier_completionHandler_(
        &self,
        leaderboardIdentifier: NSString,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDefaultLeaderboardIdentifier : leaderboardIdentifier, completionHandler : completionHandler)
    }
}
impl GKLocalPlayer_Obsoleted for GKLocalPlayer {}
pub trait GKLocalPlayer_Obsoleted: Sized + std::ops::Deref {
    unsafe fn loadFriendsWithCompletionHandler_(
        &self,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, loadFriendsWithCompletionHandler : completionHandler)
    }
    unsafe fn friends(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, friends)
    }
}
pub type GKFriendsAuthorizationStatus = NSInteger;
impl GKLocalPlayer_FriendsList for GKLocalPlayer {}
pub trait GKLocalPlayer_FriendsList: Sized + std::ops::Deref {
    unsafe fn loadFriendsAuthorizationStatus_(&self, completionHandler: *mut ::std::os::raw::c_void)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, loadFriendsAuthorizationStatus : completionHandler)
    }
    unsafe fn loadFriends_(&self, completionHandler: *mut ::std::os::raw::c_void)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, loadFriends : completionHandler)
    }
    unsafe fn loadFriendsWithIdentifiers_completionHandler_(
        &self,
        identifiers: NSArray,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, loadFriendsWithIdentifiers : identifiers, completionHandler : completionHandler)
    }
}
impl GKLocalPlayer_UI for GKLocalPlayer {}
pub trait GKLocalPlayer_UI: Sized + std::ops::Deref {
    unsafe fn presentFriendRequestCreatorFromViewController_error_(
        &self,
        viewController: NSViewController,
        error: *mut NSError,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, presentFriendRequestCreatorFromViewController : viewController, error : error)
    }
    unsafe fn presentFriendRequestCreatorFromWindow_error_(
        &self,
        window: NSWindow,
        error: *mut NSError,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, presentFriendRequestCreatorFromWindow : window, error : error)
    }
    unsafe fn authenticateHandler(&self) -> *mut ::std::os::raw::c_void
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, authenticateHandler)
    }
    unsafe fn setAuthenticateHandler_(&self, authenticateHandler: *mut ::std::os::raw::c_void)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAuthenticateHandler : authenticateHandler)
    }
    unsafe fn isPresentingFriendRequestViewController(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isPresentingFriendRequestViewController)
    }
}
pub type GKMatchSendDataMode = NSInteger;
pub type GKPlayerConnectionState = NSInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct GKMatch(pub id);
impl std::ops::Deref for GKMatch {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for GKMatch {}
impl GKMatch {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"GKMatch").unwrap(), alloc) })
    }
}
impl INSObject for GKMatch {}
impl PNSObject for GKMatch {}
impl std::convert::TryFrom<NSObject> for GKMatch {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<GKMatch, Self::Error> {
        let is_kind_of: bool = unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"GKMatch").unwrap()) };
        if is_kind_of {
            Ok(GKMatch(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to GKMatch")
        }
    }
}
impl IGKMatch for GKMatch {}
pub trait IGKMatch: Sized + std::ops::Deref {
    unsafe fn sendData_toPlayers_dataMode_error_(
        &self,
        data: NSData,
        players: NSArray,
        mode: GKMatchSendDataMode,
        error: *mut NSError,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, sendData : data, toPlayers : players, dataMode : mode, error : error)
    }
    unsafe fn sendDataToAllPlayers_withDataMode_error_(
        &self,
        data: NSData,
        mode: GKMatchSendDataMode,
        error: *mut NSError,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, sendDataToAllPlayers : data, withDataMode : mode, error : error)
    }
    unsafe fn disconnect(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, disconnect)
    }
    unsafe fn chooseBestHostingPlayerWithCompletionHandler_(
        &self,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, chooseBestHostingPlayerWithCompletionHandler : completionHandler)
    }
    unsafe fn rematchWithCompletionHandler_(&self, completionHandler: *mut ::std::os::raw::c_void)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, rematchWithCompletionHandler : completionHandler)
    }
    unsafe fn voiceChatWithName_(&self, name: NSString) -> GKVoiceChat
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, voiceChatWithName : name)
    }
    unsafe fn players(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, players)
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
    unsafe fn expectedPlayerCount(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, expectedPlayerCount)
    }
    unsafe fn properties(&self) -> *mut GKMatchProperties
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, properties)
    }
    unsafe fn playerProperties(&self) -> NSDictionary
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, playerProperties)
    }
}
pub trait PGKMatchDelegate: Sized + std::ops::Deref {
    unsafe fn match_didReceiveData_fromRemotePlayer_(
        &self,
        match_: GKMatch,
        data: NSData,
        player: GKPlayer,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, match : match_, didReceiveData : data, fromRemotePlayer : player)
    }
    unsafe fn match_didReceiveData_forRecipient_fromRemotePlayer_(
        &self,
        match_: GKMatch,
        data: NSData,
        recipient: GKPlayer,
        player: GKPlayer,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, match : match_, didReceiveData : data, forRecipient : recipient, fromRemotePlayer : player)
    }
    unsafe fn match_player_didChangeConnectionState_(
        &self,
        match_: GKMatch,
        player: GKPlayer,
        state: GKPlayerConnectionState,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, match : match_, player : player, didChangeConnectionState : state)
    }
    unsafe fn match_didFailWithError_(&self, match_: GKMatch, error: NSError)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, match : match_, didFailWithError : error)
    }
    unsafe fn match_shouldReinviteDisconnectedPlayer_(
        &self,
        match_: GKMatch,
        player: GKPlayer,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, match : match_, shouldReinviteDisconnectedPlayer : player)
    }
    unsafe fn match_didReceiveData_fromPlayer_(
        &self,
        match_: GKMatch,
        data: NSData,
        playerID: NSString,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, match : match_, didReceiveData : data, fromPlayer : playerID)
    }
    unsafe fn match_player_didChangeState_(
        &self,
        match_: GKMatch,
        playerID: NSString,
        state: GKPlayerConnectionState,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, match : match_, player : playerID, didChangeState : state)
    }
    unsafe fn match_shouldReinvitePlayer_(&self, match_: GKMatch, playerID: NSString) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, match : match_, shouldReinvitePlayer : playerID)
    }
}
impl GKMatch_Obsoleted for GKMatch {}
pub trait GKMatch_Obsoleted: Sized + std::ops::Deref {
    unsafe fn chooseBestHostPlayerWithCompletionHandler_(
        &self,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, chooseBestHostPlayerWithCompletionHandler : completionHandler)
    }
    unsafe fn sendData_toPlayers_withDataMode_error_(
        &self,
        data: NSData,
        playerIDs: NSArray,
        mode: GKMatchSendDataMode,
        error: *mut NSError,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, sendData : data, toPlayers : playerIDs, withDataMode : mode, error : error)
    }
    unsafe fn playerIDs(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, playerIDs)
    }
}
pub type GKMatchmakingMode = NSInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct GKMatchmakerViewController(pub id);
impl std::ops::Deref for GKMatchmakerViewController {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for GKMatchmakerViewController {}
impl GKMatchmakerViewController {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"GKMatchmakerViewController").unwrap(), alloc) })
    }
}
impl PGKViewController for GKMatchmakerViewController {}
impl INSViewController for GKMatchmakerViewController {}
impl PNSEditor for GKMatchmakerViewController {}
impl PNSSeguePerforming for GKMatchmakerViewController {}
impl PNSUserInterfaceItemIdentification for GKMatchmakerViewController {}
impl std::convert::TryFrom<NSViewController> for GKMatchmakerViewController {
    type Error = &'static str;
    fn try_from(parent: NSViewController) -> Result<GKMatchmakerViewController, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"GKMatchmakerViewController").unwrap()) };
        if is_kind_of {
            Ok(GKMatchmakerViewController(parent.0))
        } else {
            Err("This NSViewController cannot be downcasted to GKMatchmakerViewController")
        }
    }
}
impl INSResponder for GKMatchmakerViewController {}
impl PNSCoding for GKMatchmakerViewController {}
impl INSObject for GKMatchmakerViewController {}
impl PNSObject for GKMatchmakerViewController {}
impl IGKMatchmakerViewController for GKMatchmakerViewController {}
pub trait IGKMatchmakerViewController: Sized + std::ops::Deref {
    unsafe fn initWithMatchRequest_(&self, request: GKMatchRequest) -> id
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithMatchRequest : request)
    }
    unsafe fn initWithInvite_(&self, invite: GKInvite) -> id
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithInvite : invite)
    }
    unsafe fn addPlayersToMatch_(&self, match_: GKMatch)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, addPlayersToMatch : match_)
    }
    unsafe fn setHostedPlayer_didConnect_(&self, player: GKPlayer, connected: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setHostedPlayer : player, didConnect : connected)
    }
    unsafe fn matchmakerDelegate(&self) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, matchmakerDelegate)
    }
    unsafe fn setMatchmakerDelegate_(&self, matchmakerDelegate: *mut u64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMatchmakerDelegate : matchmakerDelegate)
    }
    unsafe fn matchRequest(&self) -> GKMatchRequest
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, matchRequest)
    }
    unsafe fn isHosted(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isHosted)
    }
    unsafe fn setHosted_(&self, hosted: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setHosted : hosted)
    }
    unsafe fn matchmakingMode(&self) -> GKMatchmakingMode
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, matchmakingMode)
    }
    unsafe fn setMatchmakingMode_(&self, matchmakingMode: GKMatchmakingMode)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMatchmakingMode : matchmakingMode)
    }
    unsafe fn canStartWithMinimumPlayers(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, canStartWithMinimumPlayers)
    }
    unsafe fn setCanStartWithMinimumPlayers_(&self, canStartWithMinimumPlayers: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCanStartWithMinimumPlayers : canStartWithMinimumPlayers)
    }
    unsafe fn defaultInvitationMessage(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, defaultInvitationMessage)
    }
    unsafe fn setDefaultInvitationMessage_(&self, defaultInvitationMessage: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDefaultInvitationMessage : defaultInvitationMessage)
    }
}
impl GKMatchmakerViewController_Obsoleted for GKMatchmakerViewController {}
pub trait GKMatchmakerViewController_Obsoleted: Sized + std::ops::Deref {
    unsafe fn setHostedPlayer_connected_(&self, playerID: NSString, connected: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setHostedPlayer : playerID, connected : connected)
    }
    unsafe fn setHostedPlayerReady_(&self, playerID: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setHostedPlayerReady : playerID)
    }
}
pub trait PGKMatchmakerViewControllerDelegate: Sized + std::ops::Deref {
    unsafe fn matchmakerViewControllerWasCancelled_(
        &self,
        viewController: GKMatchmakerViewController,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, matchmakerViewControllerWasCancelled : viewController)
    }
    unsafe fn matchmakerViewController_didFailWithError_(
        &self,
        viewController: GKMatchmakerViewController,
        error: NSError,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, matchmakerViewController : viewController, didFailWithError : error)
    }
    unsafe fn matchmakerViewController_didFindMatch_(
        &self,
        viewController: GKMatchmakerViewController,
        match_: GKMatch,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, matchmakerViewController : viewController, didFindMatch : match_)
    }
    unsafe fn matchmakerViewController_didFindHostedPlayers_(
        &self,
        viewController: GKMatchmakerViewController,
        players: NSArray,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, matchmakerViewController : viewController, didFindHostedPlayers : players)
    }
    unsafe fn matchmakerViewController_hostedPlayerDidAccept_(
        &self,
        viewController: GKMatchmakerViewController,
        player: GKPlayer,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, matchmakerViewController : viewController, hostedPlayerDidAccept : player)
    }
    unsafe fn matchmakerViewController_getMatchPropertiesForRecipient_withCompletionHandler_(
        &self,
        viewController: GKMatchmakerViewController,
        recipient: GKPlayer,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, matchmakerViewController : viewController, getMatchPropertiesForRecipient : recipient, withCompletionHandler : completionHandler)
    }
    unsafe fn matchmakerViewController_didFindPlayers_(
        &self,
        viewController: GKMatchmakerViewController,
        playerIDs: NSArray,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, matchmakerViewController : viewController, didFindPlayers : playerIDs)
    }
    unsafe fn matchmakerViewController_didReceiveAcceptFromHostedPlayer_(
        &self,
        viewController: GKMatchmakerViewController,
        playerID: NSString,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, matchmakerViewController : viewController, didReceiveAcceptFromHostedPlayer : playerID)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct GKNotificationBanner(pub id);
impl std::ops::Deref for GKNotificationBanner {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for GKNotificationBanner {}
impl GKNotificationBanner {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"GKNotificationBanner").unwrap(), alloc) })
    }
}
impl INSObject for GKNotificationBanner {}
impl PNSObject for GKNotificationBanner {}
impl std::convert::TryFrom<NSObject> for GKNotificationBanner {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<GKNotificationBanner, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"GKNotificationBanner").unwrap()) };
        if is_kind_of {
            Ok(GKNotificationBanner(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to GKNotificationBanner")
        }
    }
}
impl IGKNotificationBanner for GKNotificationBanner {}
pub trait IGKNotificationBanner: Sized + std::ops::Deref {
    unsafe fn showBannerWithTitle_message_completionHandler_(
        title: NSString,
        message: NSString,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"GKNotificationBanner").unwrap(), showBannerWithTitle : title, message : message, completionHandler : completionHandler)
    }
    unsafe fn showBannerWithTitle_message_duration_completionHandler_(
        title: NSString,
        message: NSString,
        duration: NSTimeInterval,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"GKNotificationBanner").unwrap(), showBannerWithTitle : title, message : message, duration : duration, completionHandler : completionHandler)
    }
}
pub type GKSendDataMode = ::std::os::raw::c_int;
pub type GKSessionMode = ::std::os::raw::c_int;
pub type GKPeerConnectionState = ::std::os::raw::c_int;
pub type GKVoiceChatServiceError = ::std::os::raw::c_int;
pub trait PGKSessionDelegate: Sized + std::ops::Deref {
    unsafe fn session_peer_didChangeState_(
        &self,
        session: GKSession,
        peerID: NSString,
        state: GKPeerConnectionState,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, session : session, peer : peerID, didChangeState : state)
    }
    unsafe fn session_didReceiveConnectionRequestFromPeer_(
        &self,
        session: GKSession,
        peerID: NSString,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, session : session, didReceiveConnectionRequestFromPeer : peerID)
    }
    unsafe fn session_connectionWithPeerFailed_withError_(
        &self,
        session: GKSession,
        peerID: NSString,
        error: NSError,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, session : session, connectionWithPeerFailed : peerID, withError : error)
    }
    unsafe fn session_didFailWithError_(&self, session: GKSession, error: NSError)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, session : session, didFailWithError : error)
    }
}
pub trait PGKVoiceChatClient: Sized + std::ops::Deref {
    unsafe fn voiceChatService_sendData_toParticipantID_(
        &self,
        voiceChatService: GKVoiceChatService,
        data: NSData,
        participantID: NSString,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, voiceChatService : voiceChatService, sendData : data, toParticipantID : participantID)
    }
    unsafe fn participantID(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, participantID)
    }
    unsafe fn voiceChatService_sendRealTimeData_toParticipantID_(
        &self,
        voiceChatService: GKVoiceChatService,
        data: NSData,
        participantID: NSString,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, voiceChatService : voiceChatService, sendRealTimeData : data, toParticipantID : participantID)
    }
    unsafe fn voiceChatService_didStartWithParticipantID_(
        &self,
        voiceChatService: GKVoiceChatService,
        participantID: NSString,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, voiceChatService : voiceChatService, didStartWithParticipantID : participantID)
    }
    unsafe fn voiceChatService_didNotStartWithParticipantID_error_(
        &self,
        voiceChatService: GKVoiceChatService,
        participantID: NSString,
        error: NSError,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, voiceChatService : voiceChatService, didNotStartWithParticipantID : participantID, error : error)
    }
    unsafe fn voiceChatService_didStopWithParticipantID_error_(
        &self,
        voiceChatService: GKVoiceChatService,
        participantID: NSString,
        error: NSError,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, voiceChatService : voiceChatService, didStopWithParticipantID : participantID, error : error)
    }
    unsafe fn voiceChatService_didReceiveInvitationFromParticipantID_callID_(
        &self,
        voiceChatService: GKVoiceChatService,
        participantID: NSString,
        callID: NSInteger,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, voiceChatService : voiceChatService, didReceiveInvitationFromParticipantID : participantID, callID : callID)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct GKSavedGame(pub id);
impl std::ops::Deref for GKSavedGame {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for GKSavedGame {}
impl GKSavedGame {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"GKSavedGame").unwrap(), alloc) })
    }
}
impl PNSCopying for GKSavedGame {}
impl INSObject for GKSavedGame {}
impl PNSObject for GKSavedGame {}
impl std::convert::TryFrom<NSObject> for GKSavedGame {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<GKSavedGame, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"GKSavedGame").unwrap()) };
        if is_kind_of {
            Ok(GKSavedGame(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to GKSavedGame")
        }
    }
}
impl IGKSavedGame for GKSavedGame {}
pub trait IGKSavedGame: Sized + std::ops::Deref {
    unsafe fn loadDataWithCompletionHandler_(&self, handler: *mut ::std::os::raw::c_void)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, loadDataWithCompletionHandler : handler)
    }
    unsafe fn name(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, name)
    }
    unsafe fn deviceName(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, deviceName)
    }
    unsafe fn modificationDate(&self) -> NSDate
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, modificationDate)
    }
}
impl GKLocalPlayer_GKSavedGame for GKLocalPlayer {}
pub trait GKLocalPlayer_GKSavedGame: Sized + std::ops::Deref {
    unsafe fn fetchSavedGamesWithCompletionHandler_(&self, handler: *mut ::std::os::raw::c_void)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, fetchSavedGamesWithCompletionHandler : handler)
    }
    unsafe fn saveGameData_withName_completionHandler_(
        &self,
        data: NSData,
        name: NSString,
        handler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, saveGameData : data, withName : name, completionHandler : handler)
    }
    unsafe fn deleteSavedGamesWithName_completionHandler_(
        &self,
        name: NSString,
        handler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, deleteSavedGamesWithName : name, completionHandler : handler)
    }
    unsafe fn resolveConflictingSavedGames_withData_completionHandler_(
        &self,
        conflictingSavedGames: NSArray,
        data: NSData,
        handler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, resolveConflictingSavedGames : conflictingSavedGames, withData : data, completionHandler : handler)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct GKSession(pub id);
impl std::ops::Deref for GKSession {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for GKSession {}
impl GKSession {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"GKSession").unwrap(), alloc) })
    }
}
impl INSObject for GKSession {}
impl PNSObject for GKSession {}
impl std::convert::TryFrom<NSObject> for GKSession {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<GKSession, Self::Error> {
        let is_kind_of: bool = unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"GKSession").unwrap()) };
        if is_kind_of {
            Ok(GKSession(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to GKSession")
        }
    }
}
impl IGKSession for GKSession {}
pub trait IGKSession: Sized + std::ops::Deref {
    unsafe fn initWithSessionID_displayName_sessionMode_(
        &self,
        sessionID: NSString,
        name: NSString,
        mode: GKSessionMode,
    ) -> id
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithSessionID : sessionID, displayName : name, sessionMode : mode)
    }
    unsafe fn displayNameForPeer_(&self, peerID: NSString) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, displayNameForPeer : peerID)
    }
    unsafe fn sendData_toPeers_withDataMode_error_(
        &self,
        data: NSData,
        peers: NSArray,
        mode: GKSendDataMode,
        error: *mut NSError,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, sendData : data, toPeers : peers, withDataMode : mode, error : error)
    }
    unsafe fn sendDataToAllPeers_withDataMode_error_(
        &self,
        data: NSData,
        mode: GKSendDataMode,
        error: *mut NSError,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, sendDataToAllPeers : data, withDataMode : mode, error : error)
    }
    unsafe fn setDataReceiveHandler_withContext_(
        &self,
        handler: id,
        context: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDataReceiveHandler : handler, withContext : context)
    }
    unsafe fn connectToPeer_withTimeout_(&self, peerID: NSString, timeout: NSTimeInterval)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, connectToPeer : peerID, withTimeout : timeout)
    }
    unsafe fn cancelConnectToPeer_(&self, peerID: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, cancelConnectToPeer : peerID)
    }
    unsafe fn acceptConnectionFromPeer_error_(&self, peerID: NSString, error: *mut NSError) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, acceptConnectionFromPeer : peerID, error : error)
    }
    unsafe fn denyConnectionFromPeer_(&self, peerID: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, denyConnectionFromPeer : peerID)
    }
    unsafe fn disconnectPeerFromAllPeers_(&self, peerID: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, disconnectPeerFromAllPeers : peerID)
    }
    unsafe fn disconnectFromAllPeers(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, disconnectFromAllPeers)
    }
    unsafe fn peersWithConnectionState_(&self, state: GKPeerConnectionState) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, peersWithConnectionState : state)
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
    unsafe fn sessionID(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, sessionID)
    }
    unsafe fn displayName(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, displayName)
    }
    unsafe fn sessionMode(&self) -> GKSessionMode
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, sessionMode)
    }
    unsafe fn peerID(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, peerID)
    }
    unsafe fn isAvailable(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isAvailable)
    }
    unsafe fn setAvailable_(&self, available: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAvailable : available)
    }
    unsafe fn disconnectTimeout(&self) -> NSTimeInterval
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, disconnectTimeout)
    }
    unsafe fn setDisconnectTimeout_(&self, disconnectTimeout: NSTimeInterval)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setDisconnectTimeout : disconnectTimeout)
    }
}
pub type GKSessionError = ::std::os::raw::c_int;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct GKTurnBasedMatchmakerViewController(pub id);
impl std::ops::Deref for GKTurnBasedMatchmakerViewController {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for GKTurnBasedMatchmakerViewController {}
impl GKTurnBasedMatchmakerViewController {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"GKTurnBasedMatchmakerViewController").unwrap(), alloc) })
    }
}
impl PGKViewController for GKTurnBasedMatchmakerViewController {}
impl INSViewController for GKTurnBasedMatchmakerViewController {}
impl PNSEditor for GKTurnBasedMatchmakerViewController {}
impl PNSSeguePerforming for GKTurnBasedMatchmakerViewController {}
impl PNSUserInterfaceItemIdentification for GKTurnBasedMatchmakerViewController {}
impl std::convert::TryFrom<NSViewController> for GKTurnBasedMatchmakerViewController {
    type Error = &'static str;
    fn try_from(
        parent: NSViewController,
    ) -> Result<GKTurnBasedMatchmakerViewController, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"GKTurnBasedMatchmakerViewController").unwrap())
        };
        if is_kind_of {
            Ok(GKTurnBasedMatchmakerViewController(parent.0))
        } else {
            Err("This NSViewController cannot be downcasted to GKTurnBasedMatchmakerViewController")
        }
    }
}
impl INSResponder for GKTurnBasedMatchmakerViewController {}
impl PNSCoding for GKTurnBasedMatchmakerViewController {}
impl INSObject for GKTurnBasedMatchmakerViewController {}
impl PNSObject for GKTurnBasedMatchmakerViewController {}
impl IGKTurnBasedMatchmakerViewController for GKTurnBasedMatchmakerViewController {}
pub trait IGKTurnBasedMatchmakerViewController: Sized + std::ops::Deref {}
impl GKTurnBasedMatchmakerViewController_ for GKTurnBasedMatchmakerViewController {}
pub trait GKTurnBasedMatchmakerViewController_: Sized + std::ops::Deref {
    unsafe fn turnBasedMatchmakerDelegate(&self) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, turnBasedMatchmakerDelegate)
    }
    unsafe fn setTurnBasedMatchmakerDelegate_(&self, turnBasedMatchmakerDelegate: *mut u64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setTurnBasedMatchmakerDelegate : turnBasedMatchmakerDelegate)
    }
    unsafe fn showExistingMatches(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, showExistingMatches)
    }
    unsafe fn setShowExistingMatches_(&self, showExistingMatches: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setShowExistingMatches : showExistingMatches)
    }
    unsafe fn matchmakingMode(&self) -> GKMatchmakingMode
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, matchmakingMode)
    }
    unsafe fn setMatchmakingMode_(&self, matchmakingMode: GKMatchmakingMode)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMatchmakingMode : matchmakingMode)
    }
    unsafe fn initWithMatchRequest_(&self, request: GKMatchRequest) -> id
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithMatchRequest : request)
    }
}
pub trait PGKTurnBasedMatchmakerViewControllerDelegate: Sized + std::ops::Deref {
    unsafe fn turnBasedMatchmakerViewControllerWasCancelled_(
        &self,
        viewController: GKTurnBasedMatchmakerViewController,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, turnBasedMatchmakerViewControllerWasCancelled : viewController)
    }
    unsafe fn turnBasedMatchmakerViewController_didFailWithError_(
        &self,
        viewController: GKTurnBasedMatchmakerViewController,
        error: NSError,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, turnBasedMatchmakerViewController : viewController, didFailWithError : error)
    }
    unsafe fn turnBasedMatchmakerViewController_didFindMatch_(
        &self,
        viewController: GKTurnBasedMatchmakerViewController,
        match_: GKTurnBasedMatch,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, turnBasedMatchmakerViewController : viewController, didFindMatch : match_)
    }
    unsafe fn turnBasedMatchmakerViewController_playerQuitForMatch_(
        &self,
        viewController: GKTurnBasedMatchmakerViewController,
        match_: GKTurnBasedMatch,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, turnBasedMatchmakerViewController : viewController, playerQuitForMatch : match_)
    }
}
pub type GKVoiceChatPlayerState = NSInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct GKVoiceChat(pub id);
impl std::ops::Deref for GKVoiceChat {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for GKVoiceChat {}
impl GKVoiceChat {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"GKVoiceChat").unwrap(), alloc) })
    }
}
impl INSObject for GKVoiceChat {}
impl PNSObject for GKVoiceChat {}
impl std::convert::TryFrom<NSObject> for GKVoiceChat {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<GKVoiceChat, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"GKVoiceChat").unwrap()) };
        if is_kind_of {
            Ok(GKVoiceChat(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to GKVoiceChat")
        }
    }
}
impl IGKVoiceChat for GKVoiceChat {}
pub trait IGKVoiceChat: Sized + std::ops::Deref {
    unsafe fn start(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, start)
    }
    unsafe fn stop(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, stop)
    }
    unsafe fn setPlayer_muted_(&self, player: GKPlayer, isMuted: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPlayer : player, muted : isMuted)
    }
    unsafe fn playerVoiceChatStateDidChangeHandler(&self) -> *mut ::std::os::raw::c_void
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, playerVoiceChatStateDidChangeHandler)
    }
    unsafe fn setPlayerVoiceChatStateDidChangeHandler_(
        &self,
        playerVoiceChatStateDidChangeHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPlayerVoiceChatStateDidChangeHandler : playerVoiceChatStateDidChangeHandler)
    }
    unsafe fn name(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, name)
    }
    unsafe fn isActive(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isActive)
    }
    unsafe fn setActive_(&self, active: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setActive : active)
    }
    unsafe fn volume(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, volume)
    }
    unsafe fn setVolume_(&self, volume: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setVolume : volume)
    }
    unsafe fn players(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, players)
    }
    unsafe fn isVoIPAllowed() -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"GKVoiceChat").unwrap(), isVoIPAllowed)
    }
}
impl GKVoiceChat_Deprecated for GKVoiceChat {}
pub trait GKVoiceChat_Deprecated: Sized + std::ops::Deref {
    unsafe fn playerStateUpdateHandler(&self) -> *mut ::std::os::raw::c_void
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, playerStateUpdateHandler)
    }
    unsafe fn setPlayerStateUpdateHandler_(
        &self,
        playerStateUpdateHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPlayerStateUpdateHandler : playerStateUpdateHandler)
    }
}
impl GKVoiceChat_Obsoleted for GKVoiceChat {}
pub trait GKVoiceChat_Obsoleted: Sized + std::ops::Deref {
    unsafe fn setMute_forPlayer_(&self, isMuted: BOOL, playerID: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMute : isMuted, forPlayer : playerID)
    }
    unsafe fn playerIDs(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, playerIDs)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct GKVoiceChatService(pub id);
impl std::ops::Deref for GKVoiceChatService {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for GKVoiceChatService {}
impl GKVoiceChatService {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"GKVoiceChatService").unwrap(), alloc) })
    }
}
impl INSObject for GKVoiceChatService {}
impl PNSObject for GKVoiceChatService {}
impl std::convert::TryFrom<NSObject> for GKVoiceChatService {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<GKVoiceChatService, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"GKVoiceChatService").unwrap()) };
        if is_kind_of {
            Ok(GKVoiceChatService(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to GKVoiceChatService")
        }
    }
}
impl IGKVoiceChatService for GKVoiceChatService {}
pub trait IGKVoiceChatService: Sized + std::ops::Deref {
    unsafe fn startVoiceChatWithParticipantID_error_(
        &self,
        participantID: NSString,
        error: *mut NSError,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, startVoiceChatWithParticipantID : participantID, error : error)
    }
    unsafe fn stopVoiceChatWithParticipantID_(&self, participantID: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, stopVoiceChatWithParticipantID : participantID)
    }
    unsafe fn acceptCallID_error_(&self, callID: NSInteger, error: *mut NSError) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, acceptCallID : callID, error : error)
    }
    unsafe fn denyCallID_(&self, callID: NSInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, denyCallID : callID)
    }
    unsafe fn receivedRealTimeData_fromParticipantID_(&self, audio: NSData, participantID: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, receivedRealTimeData : audio, fromParticipantID : participantID)
    }
    unsafe fn receivedData_fromParticipantID_(&self, arbitraryData: NSData, participantID: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, receivedData : arbitraryData, fromParticipantID : participantID)
    }
    unsafe fn client(&self) -> *mut u64
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, client)
    }
    unsafe fn setClient_(&self, client: *mut u64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setClient : client)
    }
    unsafe fn isMicrophoneMuted(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isMicrophoneMuted)
    }
    unsafe fn setMicrophoneMuted_(&self, microphoneMuted: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMicrophoneMuted : microphoneMuted)
    }
    unsafe fn remoteParticipantVolume(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, remoteParticipantVolume)
    }
    unsafe fn setRemoteParticipantVolume_(&self, remoteParticipantVolume: f32)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRemoteParticipantVolume : remoteParticipantVolume)
    }
    unsafe fn isOutputMeteringEnabled(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isOutputMeteringEnabled)
    }
    unsafe fn setOutputMeteringEnabled_(&self, outputMeteringEnabled: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setOutputMeteringEnabled : outputMeteringEnabled)
    }
    unsafe fn isInputMeteringEnabled(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isInputMeteringEnabled)
    }
    unsafe fn setInputMeteringEnabled_(&self, inputMeteringEnabled: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setInputMeteringEnabled : inputMeteringEnabled)
    }
    unsafe fn outputMeterLevel(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, outputMeterLevel)
    }
    unsafe fn inputMeterLevel(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, inputMeterLevel)
    }
    unsafe fn defaultVoiceChatService() -> GKVoiceChatService
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"GKVoiceChatService").unwrap(), defaultVoiceChatService)
    }
    unsafe fn isVoIPAllowed() -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"GKVoiceChatService").unwrap(), isVoIPAllowed)
    }
}

unsafe impl objc2::encode::RefEncode for GKBasePlayer {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for GKBasePlayer {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for GKPlayer {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for GKPlayer {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for GKLeaderboard {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for GKLeaderboard {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for GKAchievement {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for GKAchievement {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for GKDialogController {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for GKDialogController {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for GKGameCenterViewController {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for GKGameCenterViewController {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for GKAccessPoint {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for GKAccessPoint {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for GKAchievementDescription {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for GKAchievementDescription {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for GKAchievementViewController {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for GKAchievementViewController {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for GKScore {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for GKScore {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for GKLeaderboardEntry {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for GKLeaderboardEntry {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for GKLeaderboardScore {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for GKLeaderboardScore {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for GKChallenge {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for GKChallenge {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for GKScoreChallenge {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for GKScoreChallenge {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for GKAchievementChallenge {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for GKAchievementChallenge {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for GKChallengeDefinition {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for GKChallengeDefinition {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for GKChallengeEventHandler {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for GKChallengeEventHandler {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for GKChallengesViewController {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for GKChallengesViewController {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for GKCloudPlayer {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for GKCloudPlayer {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for GKFriendRequestComposeViewController {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for GKFriendRequestComposeViewController {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for GKGameActivity {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for GKGameActivity {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for GKGameActivityDefinition {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for GKGameActivityDefinition {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for GKGameSession {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for GKGameSession {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for GKLeaderboardSet {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for GKLeaderboardSet {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for GKLeaderboardViewController {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for GKLeaderboardViewController {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for GKTurnBasedParticipant {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for GKTurnBasedParticipant {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for GKTurnBasedMatch {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for GKTurnBasedMatch {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for GKTurnBasedExchange {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for GKTurnBasedExchange {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for GKTurnBasedExchangeReply {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for GKTurnBasedExchangeReply {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for GKTurnBasedEventHandler {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for GKTurnBasedEventHandler {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for GKMatchRequest {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for GKMatchRequest {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for GKInvite {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for GKInvite {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for GKMatchedPlayers {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for GKMatchedPlayers {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for GKMatchmaker {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for GKMatchmaker {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for GKLocalPlayer {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for GKLocalPlayer {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for GKMatch {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for GKMatch {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for GKMatchmakerViewController {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for GKMatchmakerViewController {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for GKNotificationBanner {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for GKNotificationBanner {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for GKSavedGame {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for GKSavedGame {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for GKSession {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for GKSession {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for GKTurnBasedMatchmakerViewController {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for GKTurnBasedMatchmakerViewController {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for GKVoiceChat {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for GKVoiceChat {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for GKVoiceChatService {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for GKVoiceChatService {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
