#[allow(unused_imports)]
use crate::objc::*;
#[allow(unused_imports)]
use crate::CoreGraphics::*;
#[allow(unused_imports)]
use crate::Foundation::*;
#[allow(unused_imports)]
use crate::Security::*;

#[allow(unused_imports)]
use objc2::msg_send;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct SKAdImpression(pub id);
impl std::ops::Deref for SKAdImpression {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for SKAdImpression {}
impl SKAdImpression {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"SKAdImpression").unwrap(), alloc) })
    }
}
impl INSObject for SKAdImpression {}
impl PNSObject for SKAdImpression {}
impl std::convert::TryFrom<NSObject> for SKAdImpression {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<SKAdImpression, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"SKAdImpression").unwrap()) };
        if is_kind_of {
            Ok(SKAdImpression(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to SKAdImpression")
        }
    }
}
impl ISKAdImpression for SKAdImpression {}
pub trait ISKAdImpression: Sized + std::ops::Deref {
    unsafe fn sourceAppStoreItemIdentifier(&self) -> NSNumber
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, sourceAppStoreItemIdentifier)
    }
    unsafe fn setSourceAppStoreItemIdentifier_(&self, sourceAppStoreItemIdentifier: NSNumber)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSourceAppStoreItemIdentifier : sourceAppStoreItemIdentifier)
    }
    unsafe fn advertisedAppStoreItemIdentifier(&self) -> NSNumber
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, advertisedAppStoreItemIdentifier)
    }
    unsafe fn setAdvertisedAppStoreItemIdentifier_(
        &self,
        advertisedAppStoreItemIdentifier: NSNumber,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAdvertisedAppStoreItemIdentifier : advertisedAppStoreItemIdentifier)
    }
    unsafe fn adNetworkIdentifier(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, adNetworkIdentifier)
    }
    unsafe fn setAdNetworkIdentifier_(&self, adNetworkIdentifier: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAdNetworkIdentifier : adNetworkIdentifier)
    }
    unsafe fn adCampaignIdentifier(&self) -> NSNumber
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, adCampaignIdentifier)
    }
    unsafe fn setAdCampaignIdentifier_(&self, adCampaignIdentifier: NSNumber)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAdCampaignIdentifier : adCampaignIdentifier)
    }
    unsafe fn sourceIdentifier(&self) -> NSNumber
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, sourceIdentifier)
    }
    unsafe fn setSourceIdentifier_(&self, sourceIdentifier: NSNumber)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSourceIdentifier : sourceIdentifier)
    }
    unsafe fn adImpressionIdentifier(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, adImpressionIdentifier)
    }
    unsafe fn setAdImpressionIdentifier_(&self, adImpressionIdentifier: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAdImpressionIdentifier : adImpressionIdentifier)
    }
    unsafe fn adType(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, adType)
    }
    unsafe fn setAdType_(&self, adType: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAdType : adType)
    }
    unsafe fn adDescription(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, adDescription)
    }
    unsafe fn setAdDescription_(&self, adDescription: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAdDescription : adDescription)
    }
    unsafe fn adPurchaserName(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, adPurchaserName)
    }
    unsafe fn setAdPurchaserName_(&self, adPurchaserName: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAdPurchaserName : adPurchaserName)
    }
    unsafe fn timestamp(&self) -> NSNumber
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, timestamp)
    }
    unsafe fn setTimestamp_(&self, timestamp: NSNumber)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setTimestamp : timestamp)
    }
    unsafe fn signature(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, signature)
    }
    unsafe fn setSignature_(&self, signature: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSignature : signature)
    }
    unsafe fn version(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, version)
    }
    unsafe fn setVersion_(&self, version: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setVersion : version)
    }
}
pub type SKAdNetworkCoarseConversionValue = NSString;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct SKAdNetwork(pub id);
impl std::ops::Deref for SKAdNetwork {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for SKAdNetwork {}
impl SKAdNetwork {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"SKAdNetwork").unwrap(), alloc) })
    }
}
impl INSObject for SKAdNetwork {}
impl PNSObject for SKAdNetwork {}
impl std::convert::TryFrom<NSObject> for SKAdNetwork {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<SKAdNetwork, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"SKAdNetwork").unwrap()) };
        if is_kind_of {
            Ok(SKAdNetwork(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to SKAdNetwork")
        }
    }
}
impl ISKAdNetwork for SKAdNetwork {}
pub trait ISKAdNetwork: Sized + std::ops::Deref {
    unsafe fn startImpression_completionHandler_(
        impression: SKAdImpression,
        completion: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SKAdNetwork").unwrap(), startImpression : impression, completionHandler : completion)
    }
    unsafe fn endImpression_completionHandler_(
        impression: SKAdImpression,
        completion: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SKAdNetwork").unwrap(), endImpression : impression, completionHandler : completion)
    }
    unsafe fn registerAppForAdNetworkAttribution()
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SKAdNetwork").unwrap(), registerAppForAdNetworkAttribution)
    }
    unsafe fn updateConversionValue_(conversionValue: NSInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SKAdNetwork").unwrap(), updateConversionValue : conversionValue)
    }
    unsafe fn updatePostbackConversionValue_completionHandler_(
        conversionValue: NSInteger,
        completion: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SKAdNetwork").unwrap(), updatePostbackConversionValue : conversionValue, completionHandler : completion)
    }
    unsafe fn updatePostbackConversionValue_coarseValue_completionHandler_(
        fineValue: NSInteger,
        coarseValue: NSString,
        completion: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SKAdNetwork").unwrap(), updatePostbackConversionValue : fineValue, coarseValue : coarseValue, completionHandler : completion)
    }
    unsafe fn updatePostbackConversionValue_coarseValue_lockWindow_completionHandler_(
        fineValue: NSInteger,
        coarseValue: NSString,
        lockWindow: BOOL,
        completion: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SKAdNetwork").unwrap(), updatePostbackConversionValue : fineValue, coarseValue : coarseValue, lockWindow : lockWindow, completionHandler : completion)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct SKArcadeService(pub id);
impl std::ops::Deref for SKArcadeService {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for SKArcadeService {}
impl SKArcadeService {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"SKArcadeService").unwrap(), alloc) })
    }
}
impl INSObject for SKArcadeService {}
impl PNSObject for SKArcadeService {}
impl std::convert::TryFrom<NSObject> for SKArcadeService {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<SKArcadeService, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"SKArcadeService").unwrap()) };
        if is_kind_of {
            Ok(SKArcadeService(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to SKArcadeService")
        }
    }
}
impl ISKArcadeService for SKArcadeService {}
pub trait ISKArcadeService: Sized + std::ops::Deref {
    unsafe fn registerArcadeAppWithRandomFromLib_randomFromLibLength_resultHandler_(
        randomFromLib: NSData,
        randomFromLibLength: u32,
        resultHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SKArcadeService").unwrap(), registerArcadeAppWithRandomFromLib : randomFromLib, randomFromLibLength : randomFromLibLength, resultHandler : resultHandler)
    }
    unsafe fn arcadeSubscriptionStatusWithNonce_resultHandler_(
        nonce: u64,
        resultHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SKArcadeService").unwrap(), arcadeSubscriptionStatusWithNonce : nonce, resultHandler : resultHandler)
    }
    unsafe fn repairArcadeApp()
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SKArcadeService").unwrap(), repairArcadeApp)
    }
}
pub type SKANError = NSInteger;
pub type SKCloudServiceAuthorizationStatus = NSInteger;
pub type SKCloudServiceCapability = NSUInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct SKCloudServiceController(pub id);
impl std::ops::Deref for SKCloudServiceController {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for SKCloudServiceController {}
impl SKCloudServiceController {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"SKCloudServiceController").unwrap(), alloc) })
    }
}
impl INSObject for SKCloudServiceController {}
impl PNSObject for SKCloudServiceController {}
impl std::convert::TryFrom<NSObject> for SKCloudServiceController {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<SKCloudServiceController, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"SKCloudServiceController").unwrap()) };
        if is_kind_of {
            Ok(SKCloudServiceController(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to SKCloudServiceController")
        }
    }
}
impl ISKCloudServiceController for SKCloudServiceController {}
pub trait ISKCloudServiceController: Sized + std::ops::Deref {
    unsafe fn requestCapabilitiesWithCompletionHandler_(
        &self,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, requestCapabilitiesWithCompletionHandler : completionHandler)
    }
    unsafe fn requestStorefrontCountryCodeWithCompletionHandler_(
        &self,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, requestStorefrontCountryCodeWithCompletionHandler : completionHandler)
    }
    unsafe fn requestStorefrontIdentifierWithCompletionHandler_(
        &self,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, requestStorefrontIdentifierWithCompletionHandler : completionHandler)
    }
    unsafe fn requestUserTokenForDeveloperToken_completionHandler_(
        &self,
        developerToken: NSString,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, requestUserTokenForDeveloperToken : developerToken, completionHandler : completionHandler)
    }
    unsafe fn requestPersonalizationTokenForClientToken_withCompletionHandler_(
        &self,
        clientToken: NSString,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, requestPersonalizationTokenForClientToken : clientToken, withCompletionHandler : completionHandler)
    }
    unsafe fn authorizationStatus() -> SKCloudServiceAuthorizationStatus
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SKCloudServiceController").unwrap(), authorizationStatus)
    }
    unsafe fn requestAuthorization_(completionHandler: *mut ::std::os::raw::c_void)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SKCloudServiceController").unwrap(), requestAuthorization : completionHandler)
    }
}
pub type SKCloudServiceSetupOptionsKey = NSString;
pub type SKCloudServiceSetupAction = NSString;
pub type SKCloudServiceSetupMessageIdentifier = NSString;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct SKCloudServiceSetupViewController(pub id);
impl std::ops::Deref for SKCloudServiceSetupViewController {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for SKCloudServiceSetupViewController {}
impl SKCloudServiceSetupViewController {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"SKCloudServiceSetupViewController").unwrap(), alloc) })
    }
}
impl PNSCoding for SKCloudServiceSetupViewController {}
impl INSObject for SKCloudServiceSetupViewController {}
impl PNSObject for SKCloudServiceSetupViewController {}
impl std::convert::TryFrom<NSObject> for SKCloudServiceSetupViewController {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<SKCloudServiceSetupViewController, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"SKCloudServiceSetupViewController").unwrap())
        };
        if is_kind_of {
            Ok(SKCloudServiceSetupViewController(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to SKCloudServiceSetupViewController")
        }
    }
}
impl ISKCloudServiceSetupViewController for SKCloudServiceSetupViewController {}
pub trait ISKCloudServiceSetupViewController: Sized + std::ops::Deref {
    unsafe fn loadWithOptions_completionHandler_(
        &self,
        options: NSDictionary,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, loadWithOptions : options, completionHandler : completionHandler)
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
pub trait PSKCloudServiceSetupViewControllerDelegate: Sized + std::ops::Deref {
    unsafe fn cloudServiceSetupViewControllerDidDismiss_(
        &self,
        cloudServiceSetupViewController: SKCloudServiceSetupViewController,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, cloudServiceSetupViewControllerDidDismiss : cloudServiceSetupViewController)
    }
}
pub type SKDownloadState = NSInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct SKDownload(pub id);
impl std::ops::Deref for SKDownload {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for SKDownload {}
impl SKDownload {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"SKDownload").unwrap(), alloc) })
    }
}
impl INSObject for SKDownload {}
impl PNSObject for SKDownload {}
impl std::convert::TryFrom<NSObject> for SKDownload {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<SKDownload, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"SKDownload").unwrap()) };
        if is_kind_of {
            Ok(SKDownload(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to SKDownload")
        }
    }
}
impl ISKDownload for SKDownload {}
pub trait ISKDownload: Sized + std::ops::Deref {
    unsafe fn state(&self) -> SKDownloadState
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, state)
    }
    unsafe fn downloadState(&self) -> SKDownloadState
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, downloadState)
    }
    unsafe fn contentLength(&self) -> NSNumber
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, contentLength)
    }
    unsafe fn expectedContentLength(&self) -> ::std::os::raw::c_longlong
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, expectedContentLength)
    }
    unsafe fn contentIdentifier(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, contentIdentifier)
    }
    unsafe fn contentURL(&self) -> NSURL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, contentURL)
    }
    unsafe fn contentVersion(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, contentVersion)
    }
    unsafe fn error(&self) -> NSError
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, error)
    }
    unsafe fn progress(&self) -> f32
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, progress)
    }
    unsafe fn timeRemaining(&self) -> NSTimeInterval
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, timeRemaining)
    }
    unsafe fn transaction(&self) -> SKPaymentTransaction
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, transaction)
    }
    unsafe fn contentURLForProductID_(productID: NSString) -> NSURL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SKDownload").unwrap(), contentURLForProductID : productID)
    }
    unsafe fn deleteContentForProductID_(productID: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SKDownload").unwrap(), deleteContentForProductID : productID)
    }
}
pub trait PSKDownloaderExtension: Sized + std::ops::Deref {}
pub type SKErrorCode = NSInteger;
pub trait PSKOverlayDelegate: Sized + std::ops::Deref {
    unsafe fn storeOverlay_didFailToLoadWithError_(&self, overlay: SKOverlay, error: NSError)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, storeOverlay : overlay, didFailToLoadWithError : error)
    }
    unsafe fn storeOverlay_willStartPresentation_(
        &self,
        overlay: SKOverlay,
        transitionContext: SKOverlayTransitionContext,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, storeOverlay : overlay, willStartPresentation : transitionContext)
    }
    unsafe fn storeOverlay_didFinishPresentation_(
        &self,
        overlay: SKOverlay,
        transitionContext: SKOverlayTransitionContext,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, storeOverlay : overlay, didFinishPresentation : transitionContext)
    }
    unsafe fn storeOverlay_willStartDismissal_(
        &self,
        overlay: SKOverlay,
        transitionContext: SKOverlayTransitionContext,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, storeOverlay : overlay, willStartDismissal : transitionContext)
    }
    unsafe fn storeOverlay_didFinishDismissal_(
        &self,
        overlay: SKOverlay,
        transitionContext: SKOverlayTransitionContext,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, storeOverlay : overlay, didFinishDismissal : transitionContext)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct SKOverlay(pub id);
impl std::ops::Deref for SKOverlay {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for SKOverlay {}
impl SKOverlay {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"SKOverlay").unwrap(), alloc) })
    }
}
impl INSObject for SKOverlay {}
impl PNSObject for SKOverlay {}
impl std::convert::TryFrom<NSObject> for SKOverlay {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<SKOverlay, Self::Error> {
        let is_kind_of: bool = unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"SKOverlay").unwrap()) };
        if is_kind_of {
            Ok(SKOverlay(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to SKOverlay")
        }
    }
}
impl ISKOverlay for SKOverlay {}
pub trait ISKOverlay: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn initWithConfiguration_(&self, configuration: SKOverlayConfiguration) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithConfiguration : configuration)
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
    unsafe fn configuration(&self) -> SKOverlayConfiguration
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, configuration)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SKOverlay").unwrap(), new)
    }
}
pub type SKOverlayPosition = NSInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct SKOverlayConfiguration(pub id);
impl std::ops::Deref for SKOverlayConfiguration {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for SKOverlayConfiguration {}
impl SKOverlayConfiguration {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"SKOverlayConfiguration").unwrap(), alloc) })
    }
}
impl INSObject for SKOverlayConfiguration {}
impl PNSObject for SKOverlayConfiguration {}
impl std::convert::TryFrom<NSObject> for SKOverlayConfiguration {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<SKOverlayConfiguration, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"SKOverlayConfiguration").unwrap()) };
        if is_kind_of {
            Ok(SKOverlayConfiguration(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to SKOverlayConfiguration")
        }
    }
}
impl ISKOverlayConfiguration for SKOverlayConfiguration {}
pub trait ISKOverlayConfiguration: Sized + std::ops::Deref {
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
        msg_send!(&*objc2::runtime::AnyClass::get(c"SKOverlayConfiguration").unwrap(), new)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct SKOverlayAppConfiguration(pub id);
impl std::ops::Deref for SKOverlayAppConfiguration {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for SKOverlayAppConfiguration {}
impl SKOverlayAppConfiguration {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"SKOverlayAppConfiguration").unwrap(), alloc) })
    }
}
impl ISKOverlayConfiguration for SKOverlayAppConfiguration {}
impl From<SKOverlayAppConfiguration> for SKOverlayConfiguration {
    fn from(child: SKOverlayAppConfiguration) -> SKOverlayConfiguration {
        SKOverlayConfiguration(child.0)
    }
}
impl std::convert::TryFrom<SKOverlayConfiguration> for SKOverlayAppConfiguration {
    type Error = &'static str;
    fn try_from(parent: SKOverlayConfiguration) -> Result<SKOverlayAppConfiguration, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"SKOverlayAppConfiguration").unwrap()) };
        if is_kind_of {
            Ok(SKOverlayAppConfiguration(parent.0))
        } else {
            Err("This SKOverlayConfiguration cannot be downcasted to SKOverlayAppConfiguration")
        }
    }
}
impl INSObject for SKOverlayAppConfiguration {}
impl PNSObject for SKOverlayAppConfiguration {}
impl ISKOverlayAppConfiguration for SKOverlayAppConfiguration {}
pub trait ISKOverlayAppConfiguration: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn initWithAppIdentifier_position_(
        &self,
        appIdentifier: NSString,
        position: SKOverlayPosition,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithAppIdentifier : appIdentifier, position : position)
    }
    unsafe fn setAdditionalValue_forKey_(&self, value: id, key: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAdditionalValue : value, forKey : key)
    }
    unsafe fn additionalValueForKey_(&self, key: NSString) -> id
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, additionalValueForKey : key)
    }
    unsafe fn setAdImpression_(&self, impression: SKAdImpression)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAdImpression : impression)
    }
    unsafe fn appIdentifier(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, appIdentifier)
    }
    unsafe fn setAppIdentifier_(&self, appIdentifier: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAppIdentifier : appIdentifier)
    }
    unsafe fn campaignToken(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, campaignToken)
    }
    unsafe fn setCampaignToken_(&self, campaignToken: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCampaignToken : campaignToken)
    }
    unsafe fn providerToken(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, providerToken)
    }
    unsafe fn setProviderToken_(&self, providerToken: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setProviderToken : providerToken)
    }
    unsafe fn customProductPageIdentifier(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, customProductPageIdentifier)
    }
    unsafe fn setCustomProductPageIdentifier_(&self, customProductPageIdentifier: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCustomProductPageIdentifier : customProductPageIdentifier)
    }
    unsafe fn latestReleaseID(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, latestReleaseID)
    }
    unsafe fn setLatestReleaseID_(&self, latestReleaseID: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setLatestReleaseID : latestReleaseID)
    }
    unsafe fn position(&self) -> SKOverlayPosition
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, position)
    }
    unsafe fn setPosition_(&self, position: SKOverlayPosition)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPosition : position)
    }
    unsafe fn userDismissible(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, userDismissible)
    }
    unsafe fn setUserDismissible_(&self, userDismissible: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setUserDismissible : userDismissible)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SKOverlayAppConfiguration").unwrap(), new)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct SKOverlayAppClipConfiguration(pub id);
impl std::ops::Deref for SKOverlayAppClipConfiguration {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for SKOverlayAppClipConfiguration {}
impl SKOverlayAppClipConfiguration {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"SKOverlayAppClipConfiguration").unwrap(), alloc) })
    }
}
impl ISKOverlayConfiguration for SKOverlayAppClipConfiguration {}
impl std::convert::TryFrom<SKOverlayConfiguration> for SKOverlayAppClipConfiguration {
    type Error = &'static str;
    fn try_from(
        parent: SKOverlayConfiguration,
    ) -> Result<SKOverlayAppClipConfiguration, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"SKOverlayAppClipConfiguration").unwrap())
        };
        if is_kind_of {
            Ok(SKOverlayAppClipConfiguration(parent.0))
        } else {
            Err("This SKOverlayConfiguration cannot be downcasted to SKOverlayAppClipConfiguration")
        }
    }
}
impl INSObject for SKOverlayAppClipConfiguration {}
impl PNSObject for SKOverlayAppClipConfiguration {}
impl ISKOverlayAppClipConfiguration for SKOverlayAppClipConfiguration {}
pub trait ISKOverlayAppClipConfiguration: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn initWithPosition_(&self, position: SKOverlayPosition) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithPosition : position)
    }
    unsafe fn setAdditionalValue_forKey_(&self, value: id, key: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setAdditionalValue : value, forKey : key)
    }
    unsafe fn additionalValueForKey_(&self, key: NSString) -> id
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, additionalValueForKey : key)
    }
    unsafe fn campaignToken(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, campaignToken)
    }
    unsafe fn setCampaignToken_(&self, campaignToken: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCampaignToken : campaignToken)
    }
    unsafe fn providerToken(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, providerToken)
    }
    unsafe fn setProviderToken_(&self, providerToken: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setProviderToken : providerToken)
    }
    unsafe fn customProductPageIdentifier(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, customProductPageIdentifier)
    }
    unsafe fn setCustomProductPageIdentifier_(&self, customProductPageIdentifier: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setCustomProductPageIdentifier : customProductPageIdentifier)
    }
    unsafe fn latestReleaseID(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, latestReleaseID)
    }
    unsafe fn setLatestReleaseID_(&self, latestReleaseID: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setLatestReleaseID : latestReleaseID)
    }
    unsafe fn position(&self) -> SKOverlayPosition
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, position)
    }
    unsafe fn setPosition_(&self, position: SKOverlayPosition)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPosition : position)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SKOverlayAppClipConfiguration").unwrap(), new)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct SKOverlayTransitionContext(pub id);
impl std::ops::Deref for SKOverlayTransitionContext {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for SKOverlayTransitionContext {}
impl SKOverlayTransitionContext {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"SKOverlayTransitionContext").unwrap(), alloc) })
    }
}
impl INSObject for SKOverlayTransitionContext {}
impl PNSObject for SKOverlayTransitionContext {}
impl std::convert::TryFrom<NSObject> for SKOverlayTransitionContext {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<SKOverlayTransitionContext, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"SKOverlayTransitionContext").unwrap()) };
        if is_kind_of {
            Ok(SKOverlayTransitionContext(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to SKOverlayTransitionContext")
        }
    }
}
impl ISKOverlayTransitionContext for SKOverlayTransitionContext {}
pub trait ISKOverlayTransitionContext: Sized + std::ops::Deref {
    unsafe fn init(&self) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, init)
    }
    unsafe fn addAnimationBlock_(&self, block: *mut ::std::os::raw::c_void)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, addAnimationBlock : block)
    }
    unsafe fn startFrame(&self) -> CGRect
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, startFrame)
    }
    unsafe fn endFrame(&self) -> CGRect
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, endFrame)
    }
    unsafe fn new() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SKOverlayTransitionContext").unwrap(), new)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct SKPayment(pub id);
impl std::ops::Deref for SKPayment {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for SKPayment {}
impl SKPayment {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"SKPayment").unwrap(), alloc) })
    }
}
impl PNSCopying for SKPayment {}
impl PNSMutableCopying for SKPayment {}
impl INSObject for SKPayment {}
impl PNSObject for SKPayment {}
impl std::convert::TryFrom<NSObject> for SKPayment {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<SKPayment, Self::Error> {
        let is_kind_of: bool = unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"SKPayment").unwrap()) };
        if is_kind_of {
            Ok(SKPayment(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to SKPayment")
        }
    }
}
impl ISKPayment for SKPayment {}
pub trait ISKPayment: Sized + std::ops::Deref {
    unsafe fn productIdentifier(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, productIdentifier)
    }
    unsafe fn requestData(&self) -> NSData
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, requestData)
    }
    unsafe fn quantity(&self) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, quantity)
    }
    unsafe fn applicationUsername(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, applicationUsername)
    }
    unsafe fn simulatesAskToBuyInSandbox(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, simulatesAskToBuyInSandbox)
    }
    unsafe fn paymentDiscount(&self) -> SKPaymentDiscount
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, paymentDiscount)
    }
    unsafe fn paymentWithProduct_(product: SKProduct) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SKPayment").unwrap(), paymentWithProduct : product)
    }
    unsafe fn paymentWithProductIdentifier_(identifier: NSString) -> id
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SKPayment").unwrap(), paymentWithProductIdentifier : identifier)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct SKMutablePayment(pub id);
impl std::ops::Deref for SKMutablePayment {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for SKMutablePayment {}
impl SKMutablePayment {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"SKMutablePayment").unwrap(), alloc) })
    }
}
impl ISKPayment for SKMutablePayment {}
impl PNSCopying for SKMutablePayment {}
impl PNSMutableCopying for SKMutablePayment {}
impl From<SKMutablePayment> for SKPayment {
    fn from(child: SKMutablePayment) -> SKPayment {
        SKPayment(child.0)
    }
}
impl std::convert::TryFrom<SKPayment> for SKMutablePayment {
    type Error = &'static str;
    fn try_from(parent: SKPayment) -> Result<SKMutablePayment, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"SKMutablePayment").unwrap()) };
        if is_kind_of {
            Ok(SKMutablePayment(parent.0))
        } else {
            Err("This SKPayment cannot be downcasted to SKMutablePayment")
        }
    }
}
impl INSObject for SKMutablePayment {}
impl PNSObject for SKMutablePayment {}
impl ISKMutablePayment for SKMutablePayment {}
pub trait ISKMutablePayment: Sized + std::ops::Deref {
    unsafe fn applicationUsername(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, applicationUsername)
    }
    unsafe fn setApplicationUsername_(&self, applicationUsername: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setApplicationUsername : applicationUsername)
    }
    unsafe fn paymentDiscount(&self) -> SKPaymentDiscount
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, paymentDiscount)
    }
    unsafe fn setPaymentDiscount_(&self, paymentDiscount: SKPaymentDiscount)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setPaymentDiscount : paymentDiscount)
    }
    unsafe fn productIdentifier(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, productIdentifier)
    }
    unsafe fn setProductIdentifier_(&self, productIdentifier: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setProductIdentifier : productIdentifier)
    }
    unsafe fn quantity(&self) -> NSInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, quantity)
    }
    unsafe fn setQuantity_(&self, quantity: NSInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setQuantity : quantity)
    }
    unsafe fn requestData(&self) -> NSData
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, requestData)
    }
    unsafe fn setRequestData_(&self, requestData: NSData)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setRequestData : requestData)
    }
    unsafe fn simulatesAskToBuyInSandbox(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, simulatesAskToBuyInSandbox)
    }
    unsafe fn setSimulatesAskToBuyInSandbox_(&self, simulatesAskToBuyInSandbox: BOOL)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setSimulatesAskToBuyInSandbox : simulatesAskToBuyInSandbox)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct SKPaymentDiscount(pub id);
impl std::ops::Deref for SKPaymentDiscount {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for SKPaymentDiscount {}
impl SKPaymentDiscount {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"SKPaymentDiscount").unwrap(), alloc) })
    }
}
impl INSObject for SKPaymentDiscount {}
impl PNSObject for SKPaymentDiscount {}
impl std::convert::TryFrom<NSObject> for SKPaymentDiscount {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<SKPaymentDiscount, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"SKPaymentDiscount").unwrap()) };
        if is_kind_of {
            Ok(SKPaymentDiscount(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to SKPaymentDiscount")
        }
    }
}
impl ISKPaymentDiscount for SKPaymentDiscount {}
pub trait ISKPaymentDiscount: Sized + std::ops::Deref {
    unsafe fn initWithIdentifier_keyIdentifier_nonce_signature_timestamp_(
        &self,
        identifier: NSString,
        keyIdentifier: NSString,
        nonce: NSUUID,
        signature: NSString,
        timestamp: NSNumber,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithIdentifier : identifier, keyIdentifier : keyIdentifier, nonce : nonce, signature : signature, timestamp : timestamp)
    }
    unsafe fn identifier(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, identifier)
    }
    unsafe fn keyIdentifier(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, keyIdentifier)
    }
    unsafe fn nonce(&self) -> NSUUID
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, nonce)
    }
    unsafe fn signature(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, signature)
    }
    unsafe fn timestamp(&self) -> NSNumber
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, timestamp)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct SKPaymentQueue(pub id);
impl std::ops::Deref for SKPaymentQueue {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for SKPaymentQueue {}
impl SKPaymentQueue {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"SKPaymentQueue").unwrap(), alloc) })
    }
}
impl INSObject for SKPaymentQueue {}
impl PNSObject for SKPaymentQueue {}
impl std::convert::TryFrom<NSObject> for SKPaymentQueue {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<SKPaymentQueue, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"SKPaymentQueue").unwrap()) };
        if is_kind_of {
            Ok(SKPaymentQueue(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to SKPaymentQueue")
        }
    }
}
impl ISKPaymentQueue for SKPaymentQueue {}
pub trait ISKPaymentQueue: Sized + std::ops::Deref {
    unsafe fn addPayment_(&self, payment: SKPayment)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, addPayment : payment)
    }
    unsafe fn restoreCompletedTransactions(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, restoreCompletedTransactions)
    }
    unsafe fn restoreCompletedTransactionsWithApplicationUsername_(&self, username: NSString)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, restoreCompletedTransactionsWithApplicationUsername : username)
    }
    unsafe fn finishTransaction_(&self, transaction: SKPaymentTransaction)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, finishTransaction : transaction)
    }
    unsafe fn startDownloads_(&self, downloads: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, startDownloads : downloads)
    }
    unsafe fn pauseDownloads_(&self, downloads: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, pauseDownloads : downloads)
    }
    unsafe fn resumeDownloads_(&self, downloads: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, resumeDownloads : downloads)
    }
    unsafe fn cancelDownloads_(&self, downloads: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, cancelDownloads : downloads)
    }
    unsafe fn addTransactionObserver_(&self, observer: *mut u64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, addTransactionObserver : observer)
    }
    unsafe fn removeTransactionObserver_(&self, observer: *mut u64)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, removeTransactionObserver : observer)
    }
    unsafe fn showPriceConsentIfNeeded(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, showPriceConsentIfNeeded)
    }
    unsafe fn presentCodeRedemptionSheet(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, presentCodeRedemptionSheet)
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
    unsafe fn storefront(&self) -> SKStorefront
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, storefront)
    }
    unsafe fn transactionObservers(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, transactionObservers)
    }
    unsafe fn transactions(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, transactions)
    }
    unsafe fn defaultQueue() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SKPaymentQueue").unwrap(), defaultQueue)
    }
    unsafe fn canMakePayments() -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SKPaymentQueue").unwrap(), canMakePayments)
    }
}
pub trait PSKPaymentQueueDelegate: Sized + std::ops::Deref {
    unsafe fn paymentQueue_shouldContinueTransaction_inStorefront_(
        &self,
        paymentQueue: SKPaymentQueue,
        transaction: SKPaymentTransaction,
        newStorefront: SKStorefront,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, paymentQueue : paymentQueue, shouldContinueTransaction : transaction, inStorefront : newStorefront)
    }
    unsafe fn paymentQueueShouldShowPriceConsent_(&self, paymentQueue: SKPaymentQueue) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, paymentQueueShouldShowPriceConsent : paymentQueue)
    }
}
pub trait PSKPaymentTransactionObserver: Sized + std::ops::Deref {
    unsafe fn paymentQueue_updatedTransactions_(&self, queue: SKPaymentQueue, transactions: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, paymentQueue : queue, updatedTransactions : transactions)
    }
    unsafe fn paymentQueue_removedTransactions_(&self, queue: SKPaymentQueue, transactions: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, paymentQueue : queue, removedTransactions : transactions)
    }
    unsafe fn paymentQueue_restoreCompletedTransactionsFailedWithError_(
        &self,
        queue: SKPaymentQueue,
        error: NSError,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, paymentQueue : queue, restoreCompletedTransactionsFailedWithError : error)
    }
    unsafe fn paymentQueueRestoreCompletedTransactionsFinished_(&self, queue: SKPaymentQueue)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, paymentQueueRestoreCompletedTransactionsFinished : queue)
    }
    unsafe fn paymentQueue_updatedDownloads_(&self, queue: SKPaymentQueue, downloads: NSArray)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, paymentQueue : queue, updatedDownloads : downloads)
    }
    unsafe fn paymentQueue_shouldAddStorePayment_forProduct_(
        &self,
        queue: SKPaymentQueue,
        payment: SKPayment,
        product: SKProduct,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, paymentQueue : queue, shouldAddStorePayment : payment, forProduct : product)
    }
    unsafe fn paymentQueueDidChangeStorefront_(&self, queue: SKPaymentQueue)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, paymentQueueDidChangeStorefront : queue)
    }
    unsafe fn paymentQueue_didRevokeEntitlementsForProductIdentifiers_(
        &self,
        queue: SKPaymentQueue,
        productIdentifiers: NSArray,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, paymentQueue : queue, didRevokeEntitlementsForProductIdentifiers : productIdentifiers)
    }
}
pub type SKPaymentTransactionState = NSInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct SKPaymentTransaction(pub id);
impl std::ops::Deref for SKPaymentTransaction {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for SKPaymentTransaction {}
impl SKPaymentTransaction {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"SKPaymentTransaction").unwrap(), alloc) })
    }
}
impl INSObject for SKPaymentTransaction {}
impl PNSObject for SKPaymentTransaction {}
impl std::convert::TryFrom<NSObject> for SKPaymentTransaction {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<SKPaymentTransaction, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"SKPaymentTransaction").unwrap()) };
        if is_kind_of {
            Ok(SKPaymentTransaction(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to SKPaymentTransaction")
        }
    }
}
impl ISKPaymentTransaction for SKPaymentTransaction {}
pub trait ISKPaymentTransaction: Sized + std::ops::Deref {
    unsafe fn error(&self) -> NSError
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, error)
    }
    unsafe fn originalTransaction(&self) -> SKPaymentTransaction
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, originalTransaction)
    }
    unsafe fn payment(&self) -> SKPayment
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, payment)
    }
    unsafe fn downloads(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, downloads)
    }
    unsafe fn transactionDate(&self) -> NSDate
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, transactionDate)
    }
    unsafe fn transactionIdentifier(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, transactionIdentifier)
    }
    unsafe fn transactionReceipt(&self) -> NSData
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, transactionReceipt)
    }
    unsafe fn transactionState(&self) -> SKPaymentTransactionState
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, transactionState)
    }
}
pub type SKProductPeriodUnit = NSUInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct SKProductSubscriptionPeriod(pub id);
impl std::ops::Deref for SKProductSubscriptionPeriod {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for SKProductSubscriptionPeriod {}
impl SKProductSubscriptionPeriod {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"SKProductSubscriptionPeriod").unwrap(), alloc) })
    }
}
impl INSObject for SKProductSubscriptionPeriod {}
impl PNSObject for SKProductSubscriptionPeriod {}
impl std::convert::TryFrom<NSObject> for SKProductSubscriptionPeriod {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<SKProductSubscriptionPeriod, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"SKProductSubscriptionPeriod").unwrap()) };
        if is_kind_of {
            Ok(SKProductSubscriptionPeriod(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to SKProductSubscriptionPeriod")
        }
    }
}
impl ISKProductSubscriptionPeriod for SKProductSubscriptionPeriod {}
pub trait ISKProductSubscriptionPeriod: Sized + std::ops::Deref {
    unsafe fn numberOfUnits(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, numberOfUnits)
    }
    unsafe fn unit(&self) -> SKProductPeriodUnit
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, unit)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct SKProduct(pub id);
impl std::ops::Deref for SKProduct {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for SKProduct {}
impl SKProduct {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"SKProduct").unwrap(), alloc) })
    }
}
impl INSObject for SKProduct {}
impl PNSObject for SKProduct {}
impl std::convert::TryFrom<NSObject> for SKProduct {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<SKProduct, Self::Error> {
        let is_kind_of: bool = unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"SKProduct").unwrap()) };
        if is_kind_of {
            Ok(SKProduct(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to SKProduct")
        }
    }
}
impl ISKProduct for SKProduct {}
pub trait ISKProduct: Sized + std::ops::Deref {
    unsafe fn localizedDescription(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, localizedDescription)
    }
    unsafe fn localizedTitle(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, localizedTitle)
    }
    unsafe fn price(&self) -> NSDecimalNumber
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, price)
    }
    unsafe fn priceLocale(&self) -> NSLocale
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, priceLocale)
    }
    unsafe fn productIdentifier(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, productIdentifier)
    }
    unsafe fn isDownloadable(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isDownloadable)
    }
    unsafe fn downloadable(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, downloadable)
    }
    unsafe fn isFamilyShareable(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, isFamilyShareable)
    }
    unsafe fn contentLengths(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, contentLengths)
    }
    unsafe fn downloadContentLengths(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, downloadContentLengths)
    }
    unsafe fn contentVersion(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, contentVersion)
    }
    unsafe fn downloadContentVersion(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, downloadContentVersion)
    }
    unsafe fn subscriptionPeriod(&self) -> SKProductSubscriptionPeriod
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, subscriptionPeriod)
    }
    unsafe fn introductoryPrice(&self) -> SKProductDiscount
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, introductoryPrice)
    }
    unsafe fn subscriptionGroupIdentifier(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, subscriptionGroupIdentifier)
    }
    unsafe fn discounts(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, discounts)
    }
}
pub type SKProductDiscountPaymentMode = NSUInteger;
pub type SKProductDiscountType = NSUInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct SKProductDiscount(pub id);
impl std::ops::Deref for SKProductDiscount {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for SKProductDiscount {}
impl SKProductDiscount {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"SKProductDiscount").unwrap(), alloc) })
    }
}
impl INSObject for SKProductDiscount {}
impl PNSObject for SKProductDiscount {}
impl std::convert::TryFrom<NSObject> for SKProductDiscount {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<SKProductDiscount, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"SKProductDiscount").unwrap()) };
        if is_kind_of {
            Ok(SKProductDiscount(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to SKProductDiscount")
        }
    }
}
impl ISKProductDiscount for SKProductDiscount {}
pub trait ISKProductDiscount: Sized + std::ops::Deref {
    unsafe fn price(&self) -> NSDecimalNumber
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, price)
    }
    unsafe fn priceLocale(&self) -> NSLocale
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, priceLocale)
    }
    unsafe fn identifier(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, identifier)
    }
    unsafe fn subscriptionPeriod(&self) -> SKProductSubscriptionPeriod
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, subscriptionPeriod)
    }
    unsafe fn numberOfPeriods(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, numberOfPeriods)
    }
    unsafe fn paymentMode(&self) -> SKProductDiscountPaymentMode
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, paymentMode)
    }
    unsafe fn type_(&self) -> SKProductDiscountType
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, type)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct SKRequest(pub id);
impl std::ops::Deref for SKRequest {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for SKRequest {}
impl SKRequest {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"SKRequest").unwrap(), alloc) })
    }
}
impl INSObject for SKRequest {}
impl PNSObject for SKRequest {}
impl std::convert::TryFrom<NSObject> for SKRequest {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<SKRequest, Self::Error> {
        let is_kind_of: bool = unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"SKRequest").unwrap()) };
        if is_kind_of {
            Ok(SKRequest(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to SKRequest")
        }
    }
}
impl ISKRequest for SKRequest {}
pub trait ISKRequest: Sized + std::ops::Deref {
    unsafe fn cancel(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, cancel)
    }
    unsafe fn start(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, start)
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
pub trait PSKRequestDelegate: Sized + std::ops::Deref {
    unsafe fn requestDidFinish_(&self, request: SKRequest)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, requestDidFinish : request)
    }
    unsafe fn request_didFailWithError_(&self, request: SKRequest, error: NSError)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, request : request, didFailWithError : error)
    }
}
pub trait PSKProductsRequestDelegate: Sized + std::ops::Deref {
    unsafe fn productsRequest_didReceiveResponse_(
        &self,
        request: SKProductsRequest,
        response: SKProductsResponse,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, productsRequest : request, didReceiveResponse : response)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct SKProductsRequest(pub id);
impl std::ops::Deref for SKProductsRequest {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for SKProductsRequest {}
impl SKProductsRequest {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"SKProductsRequest").unwrap(), alloc) })
    }
}
impl ISKRequest for SKProductsRequest {}
impl From<SKProductsRequest> for SKRequest {
    fn from(child: SKProductsRequest) -> SKRequest {
        SKRequest(child.0)
    }
}
impl std::convert::TryFrom<SKRequest> for SKProductsRequest {
    type Error = &'static str;
    fn try_from(parent: SKRequest) -> Result<SKProductsRequest, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"SKProductsRequest").unwrap()) };
        if is_kind_of {
            Ok(SKProductsRequest(parent.0))
        } else {
            Err("This SKRequest cannot be downcasted to SKProductsRequest")
        }
    }
}
impl INSObject for SKProductsRequest {}
impl PNSObject for SKProductsRequest {}
impl ISKProductsRequest for SKProductsRequest {}
pub trait ISKProductsRequest: Sized + std::ops::Deref {
    unsafe fn initWithProductIdentifiers_(&self, productIdentifiers: NSSet) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithProductIdentifiers : productIdentifiers)
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
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct SKProductsResponse(pub id);
impl std::ops::Deref for SKProductsResponse {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for SKProductsResponse {}
impl SKProductsResponse {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"SKProductsResponse").unwrap(), alloc) })
    }
}
impl INSObject for SKProductsResponse {}
impl PNSObject for SKProductsResponse {}
impl std::convert::TryFrom<NSObject> for SKProductsResponse {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<SKProductsResponse, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"SKProductsResponse").unwrap()) };
        if is_kind_of {
            Ok(SKProductsResponse(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to SKProductsResponse")
        }
    }
}
impl ISKProductsResponse for SKProductsResponse {}
pub trait ISKProductsResponse: Sized + std::ops::Deref {
    unsafe fn products(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, products)
    }
    unsafe fn invalidProductIdentifiers(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, invalidProductIdentifiers)
    }
}
pub type SKProductStorePromotionVisibility = NSInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct SKProductStorePromotionController(pub id);
impl std::ops::Deref for SKProductStorePromotionController {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for SKProductStorePromotionController {}
impl SKProductStorePromotionController {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"SKProductStorePromotionController").unwrap(), alloc) })
    }
}
impl INSObject for SKProductStorePromotionController {}
impl PNSObject for SKProductStorePromotionController {}
impl std::convert::TryFrom<NSObject> for SKProductStorePromotionController {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<SKProductStorePromotionController, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"SKProductStorePromotionController").unwrap())
        };
        if is_kind_of {
            Ok(SKProductStorePromotionController(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to SKProductStorePromotionController")
        }
    }
}
impl ISKProductStorePromotionController for SKProductStorePromotionController {}
pub trait ISKProductStorePromotionController: Sized + std::ops::Deref {
    unsafe fn fetchStorePromotionVisibilityForProduct_completionHandler_(
        &self,
        product: SKProduct,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, fetchStorePromotionVisibilityForProduct : product, completionHandler : completionHandler)
    }
    unsafe fn updateStorePromotionVisibility_forProduct_completionHandler_(
        &self,
        promotionVisibility: SKProductStorePromotionVisibility,
        product: SKProduct,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, updateStorePromotionVisibility : promotionVisibility, forProduct : product, completionHandler : completionHandler)
    }
    unsafe fn fetchStorePromotionOrderWithCompletionHandler_(
        &self,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, fetchStorePromotionOrderWithCompletionHandler : completionHandler)
    }
    unsafe fn updateStorePromotionOrder_completionHandler_(
        &self,
        promotionOrder: NSArray,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, updateStorePromotionOrder : promotionOrder, completionHandler : completionHandler)
    }
    unsafe fn defaultController() -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SKProductStorePromotionController").unwrap(), defaultController)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct SKReceiptRefreshRequest(pub id);
impl std::ops::Deref for SKReceiptRefreshRequest {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for SKReceiptRefreshRequest {}
impl SKReceiptRefreshRequest {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"SKReceiptRefreshRequest").unwrap(), alloc) })
    }
}
impl ISKRequest for SKReceiptRefreshRequest {}
impl std::convert::TryFrom<SKRequest> for SKReceiptRefreshRequest {
    type Error = &'static str;
    fn try_from(parent: SKRequest) -> Result<SKReceiptRefreshRequest, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"SKReceiptRefreshRequest").unwrap()) };
        if is_kind_of {
            Ok(SKReceiptRefreshRequest(parent.0))
        } else {
            Err("This SKRequest cannot be downcasted to SKReceiptRefreshRequest")
        }
    }
}
impl INSObject for SKReceiptRefreshRequest {}
impl PNSObject for SKReceiptRefreshRequest {}
impl ISKReceiptRefreshRequest for SKReceiptRefreshRequest {}
pub trait ISKReceiptRefreshRequest: Sized + std::ops::Deref {
    unsafe fn initWithReceiptProperties_(&self, properties: NSDictionary) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithReceiptProperties : properties)
    }
    unsafe fn receiptProperties(&self) -> NSDictionary
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, receiptProperties)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct SKStorefront(pub id);
impl std::ops::Deref for SKStorefront {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for SKStorefront {}
impl SKStorefront {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"SKStorefront").unwrap(), alloc) })
    }
}
impl INSObject for SKStorefront {}
impl PNSObject for SKStorefront {}
impl std::convert::TryFrom<NSObject> for SKStorefront {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<SKStorefront, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"SKStorefront").unwrap()) };
        if is_kind_of {
            Ok(SKStorefront(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to SKStorefront")
        }
    }
}
impl ISKStorefront for SKStorefront {}
pub trait ISKStorefront: Sized + std::ops::Deref {
    unsafe fn countryCode(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, countryCode)
    }
    unsafe fn identifier(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, identifier)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct SKStoreProductViewController(pub id);
impl std::ops::Deref for SKStoreProductViewController {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for SKStoreProductViewController {}
impl SKStoreProductViewController {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"SKStoreProductViewController").unwrap(), alloc) })
    }
}
impl PNSCoding for SKStoreProductViewController {}
impl INSObject for SKStoreProductViewController {}
impl PNSObject for SKStoreProductViewController {}
impl std::convert::TryFrom<NSObject> for SKStoreProductViewController {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<SKStoreProductViewController, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"SKStoreProductViewController").unwrap()) };
        if is_kind_of {
            Ok(SKStoreProductViewController(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to SKStoreProductViewController")
        }
    }
}
impl ISKStoreProductViewController for SKStoreProductViewController {}
pub trait ISKStoreProductViewController: Sized + std::ops::Deref {
    unsafe fn loadProductWithParameters_completionBlock_(
        &self,
        parameters: NSDictionary,
        block: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, loadProductWithParameters : parameters, completionBlock : block)
    }
    unsafe fn loadProductWithParameters_impression_completionBlock_(
        &self,
        parameters: NSDictionary,
        impression: SKAdImpression,
        block: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, loadProductWithParameters : parameters, impression : impression, completionBlock : block)
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
pub trait PSKStoreProductViewControllerDelegate: Sized + std::ops::Deref {
    unsafe fn productViewControllerDidFinish_(&self, viewController: SKStoreProductViewController)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, productViewControllerDidFinish : viewController)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct SKStoreReviewController(pub id);
impl std::ops::Deref for SKStoreReviewController {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for SKStoreReviewController {}
impl SKStoreReviewController {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"SKStoreReviewController").unwrap(), alloc) })
    }
}
impl INSObject for SKStoreReviewController {}
impl PNSObject for SKStoreReviewController {}
impl std::convert::TryFrom<NSObject> for SKStoreReviewController {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<SKStoreReviewController, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"SKStoreReviewController").unwrap()) };
        if is_kind_of {
            Ok(SKStoreReviewController(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to SKStoreReviewController")
        }
    }
}
impl ISKStoreReviewController for SKStoreReviewController {}
pub trait ISKStoreReviewController: Sized + std::ops::Deref {
    unsafe fn requestReview()
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&*objc2::runtime::AnyClass::get(c"SKStoreReviewController").unwrap(), requestReview)
    }
}
pub trait PNSEditor: Sized + std::ops::Deref {
    unsafe fn discardEditing(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, discardEditing)
    }
    unsafe fn commitEditing(&self) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, commitEditing)
    }
    unsafe fn commitEditingWithDelegate_didCommitSelector_contextInfo_(
        &self,
        delegate: id,
        didCommitSelector: objc2::runtime::Sel,
        contextInfo: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, commitEditingWithDelegate : delegate, didCommitSelector : didCommitSelector, contextInfo : contextInfo)
    }
    unsafe fn commitEditingAndReturnError_(&self, error: *mut NSError) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, commitEditingAndReturnError : error)
    }
}
impl PNSEditor for SKCloudServiceSetupViewController {}
impl PNSEditor for SKStoreProductViewController {}
unsafe extern "C" {
    pub static SKAdNetworkCoarseConversionValueHigh: SKAdNetworkCoarseConversionValue;
}
unsafe extern "C" {
    pub static SKAdNetworkCoarseConversionValueMedium: SKAdNetworkCoarseConversionValue;
}
unsafe extern "C" {
    pub static SKAdNetworkCoarseConversionValueLow: SKAdNetworkCoarseConversionValue;
}
unsafe extern "C" {
    pub static SKStoreProductParameterAdNetworkAttributionSignature: NSString;
}
unsafe extern "C" {
    pub static SKStoreProductParameterAdNetworkCampaignIdentifier: NSString;
}
unsafe extern "C" {
    pub static SKStoreProductParameterAdNetworkSourceIdentifier: NSString;
}
unsafe extern "C" {
    pub static SKStoreProductParameterAdNetworkIdentifier: NSString;
}
unsafe extern "C" {
    pub static SKStoreProductParameterAdNetworkNonce: NSString;
}
unsafe extern "C" {
    pub static SKStoreProductParameterAdNetworkTimestamp: NSString;
}
unsafe extern "C" {
    pub static SKStoreProductParameterAdNetworkSourceAppStoreIdentifier: NSString;
}
unsafe extern "C" {
    pub static SKStoreProductParameterAdNetworkVersion: NSString;
}
unsafe extern "C" {
    pub static SKANErrorDomain: NSString;
}
unsafe extern "C" {
    pub static SKCloudServiceCapabilitiesDidChangeNotification: NSNotificationName;
}
unsafe extern "C" {
    pub static SKStorefrontCountryCodeDidChangeNotification: NSNotificationName;
}
unsafe extern "C" {
    pub static SKStorefrontIdentifierDidChangeNotification: NSNotificationName;
}
unsafe extern "C" {
    pub static SKCloudServiceSetupOptionsActionKey: SKCloudServiceSetupOptionsKey;
}
unsafe extern "C" {
    pub static SKCloudServiceSetupOptionsITunesItemIdentifierKey: SKCloudServiceSetupOptionsKey;
}
unsafe extern "C" {
    pub static SKCloudServiceSetupOptionsAffiliateTokenKey: SKCloudServiceSetupOptionsKey;
}
unsafe extern "C" {
    pub static SKCloudServiceSetupOptionsCampaignTokenKey: SKCloudServiceSetupOptionsKey;
}
unsafe extern "C" {
    pub static SKCloudServiceSetupOptionsMessageIdentifierKey: SKCloudServiceSetupOptionsKey;
}
unsafe extern "C" {
    pub static SKCloudServiceSetupActionSubscribe: SKCloudServiceSetupAction;
}
unsafe extern "C" {
    pub static SKCloudServiceSetupMessageIdentifierJoin: SKCloudServiceSetupMessageIdentifier;
}
unsafe extern "C" {
    pub static SKCloudServiceSetupMessageIdentifierConnect: SKCloudServiceSetupMessageIdentifier;
}
unsafe extern "C" {
    pub static SKCloudServiceSetupMessageIdentifierAddMusic: SKCloudServiceSetupMessageIdentifier;
}
unsafe extern "C" {
    pub static SKCloudServiceSetupMessageIdentifierPlayMusic: SKCloudServiceSetupMessageIdentifier;
}
unsafe extern "C" {
    pub static mut SKDownloadTimeRemainingUnknown: NSTimeInterval;
}
unsafe extern "C" {
    pub static SKErrorDomain: NSString;
}
unsafe extern "C" {
    pub fn SKTerminateForInvalidReceipt();
}
unsafe extern "C" {
    pub static SKReceiptPropertyIsExpired: NSString;
}
unsafe extern "C" {
    pub static SKReceiptPropertyIsRevoked: NSString;
}
unsafe extern "C" {
    pub static SKReceiptPropertyIsVolumePurchase: NSString;
}
unsafe extern "C" {
    pub static SKStoreProductParameterITunesItemIdentifier: NSString;
}
unsafe extern "C" {
    pub static SKStoreProductParameterProductIdentifier: NSString;
}
unsafe extern "C" {
    pub static SKStoreProductParameterCustomProductPageIdentifier: NSString;
}
unsafe extern "C" {
    pub static SKStoreProductParameterAffiliateToken: NSString;
}
unsafe extern "C" {
    pub static SKStoreProductParameterCampaignToken: NSString;
}
unsafe extern "C" {
    pub static SKStoreProductParameterProviderToken: NSString;
}
unsafe extern "C" {
    pub static SKStoreProductParameterAdvertisingPartnerToken: NSString;
}

unsafe impl objc2::encode::RefEncode for SKAdImpression {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for SKAdImpression {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for SKAdNetwork {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for SKAdNetwork {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for SKArcadeService {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for SKArcadeService {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for SKCloudServiceController {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for SKCloudServiceController {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for SKCloudServiceSetupViewController {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for SKCloudServiceSetupViewController {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for SKDownload {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for SKDownload {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for SKOverlay {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for SKOverlay {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for SKOverlayConfiguration {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for SKOverlayConfiguration {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for SKOverlayAppConfiguration {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for SKOverlayAppConfiguration {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for SKOverlayAppClipConfiguration {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for SKOverlayAppClipConfiguration {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for SKOverlayTransitionContext {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for SKOverlayTransitionContext {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for SKPayment {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for SKPayment {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for SKMutablePayment {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for SKMutablePayment {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for SKPaymentDiscount {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for SKPaymentDiscount {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for SKPaymentQueue {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for SKPaymentQueue {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for SKPaymentTransaction {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for SKPaymentTransaction {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for SKProductSubscriptionPeriod {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for SKProductSubscriptionPeriod {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for SKProduct {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for SKProduct {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for SKProductDiscount {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for SKProductDiscount {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for SKRequest {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for SKRequest {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for SKProductsRequest {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for SKProductsRequest {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for SKProductsResponse {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for SKProductsResponse {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for SKProductStorePromotionController {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for SKProductStorePromotionController {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for SKReceiptRefreshRequest {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for SKReceiptRefreshRequest {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for SKStorefront {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for SKStorefront {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for SKStoreProductViewController {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for SKStoreProductViewController {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for SKStoreReviewController {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for SKStoreReviewController {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
