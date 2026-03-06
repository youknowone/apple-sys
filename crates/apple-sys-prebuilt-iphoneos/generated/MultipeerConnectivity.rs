#[allow(unused_imports)]
use crate::objc::*;
#[allow(unused_imports)]
use crate::Foundation::*;
#[allow(unused_imports)]
use crate::Security::*;

#[allow(unused_imports)]
use objc2::msg_send;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MCPeerID(pub id);
impl std::ops::Deref for MCPeerID {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MCPeerID {}
impl MCPeerID {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MCPeerID").unwrap(), alloc) })
    }
}
impl PNSCopying for MCPeerID {}
impl PNSSecureCoding for MCPeerID {}
impl INSObject for MCPeerID {}
impl PNSObject for MCPeerID {}
impl std::convert::TryFrom<NSObject> for MCPeerID {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<MCPeerID, Self::Error> {
        let is_kind_of: bool = unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MCPeerID").unwrap()) };
        if is_kind_of {
            Ok(MCPeerID(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to MCPeerID")
        }
    }
}
impl IMCPeerID for MCPeerID {}
pub trait IMCPeerID: Sized + std::ops::Deref {
    unsafe fn initWithDisplayName_(&self, myDisplayName: NSString) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithDisplayName : myDisplayName)
    }
    unsafe fn displayName(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, displayName)
    }
}
pub type MCErrorCode = NSInteger;
pub type MCSessionSendDataMode = NSInteger;
pub type MCSessionState = NSInteger;
pub type MCEncryptionPreference = NSInteger;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MCSession(pub id);
impl std::ops::Deref for MCSession {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MCSession {}
impl MCSession {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MCSession").unwrap(), alloc) })
    }
}
impl INSObject for MCSession {}
impl PNSObject for MCSession {}
impl std::convert::TryFrom<NSObject> for MCSession {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<MCSession, Self::Error> {
        let is_kind_of: bool = unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MCSession").unwrap()) };
        if is_kind_of {
            Ok(MCSession(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to MCSession")
        }
    }
}
impl IMCSession for MCSession {}
pub trait IMCSession: Sized + std::ops::Deref {
    unsafe fn initWithPeer_(&self, myPeerID: MCPeerID) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithPeer : myPeerID)
    }
    unsafe fn initWithPeer_securityIdentity_encryptionPreference_(
        &self,
        myPeerID: MCPeerID,
        identity: NSArray,
        encryptionPreference: MCEncryptionPreference,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithPeer : myPeerID, securityIdentity : identity, encryptionPreference : encryptionPreference)
    }
    unsafe fn sendData_toPeers_withMode_error_(
        &self,
        data: NSData,
        peerIDs: NSArray,
        mode: MCSessionSendDataMode,
        error: *mut NSError,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, sendData : data, toPeers : peerIDs, withMode : mode, error : error)
    }
    unsafe fn disconnect(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, disconnect)
    }
    unsafe fn sendResourceAtURL_withName_toPeer_withCompletionHandler_(
        &self,
        resourceURL: NSURL,
        resourceName: NSString,
        peerID: MCPeerID,
        completionHandler: *mut ::std::os::raw::c_void,
    ) -> NSProgress
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, sendResourceAtURL : resourceURL, withName : resourceName, toPeer : peerID, withCompletionHandler : completionHandler)
    }
    unsafe fn startStreamWithName_toPeer_error_(
        &self,
        streamName: NSString,
        peerID: MCPeerID,
        error: *mut NSError,
    ) -> NSOutputStream
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, startStreamWithName : streamName, toPeer : peerID, error : error)
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
    unsafe fn myPeerID(&self) -> MCPeerID
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, myPeerID)
    }
    unsafe fn securityIdentity(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, securityIdentity)
    }
    unsafe fn encryptionPreference(&self) -> MCEncryptionPreference
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, encryptionPreference)
    }
    unsafe fn connectedPeers(&self) -> NSArray
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, connectedPeers)
    }
}
pub trait PMCSessionDelegate: Sized + std::ops::Deref {
    unsafe fn session_peer_didChangeState_(
        &self,
        session: MCSession,
        peerID: MCPeerID,
        state: MCSessionState,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, session : session, peer : peerID, didChangeState : state)
    }
    unsafe fn session_didReceiveData_fromPeer_(
        &self,
        session: MCSession,
        data: NSData,
        peerID: MCPeerID,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, session : session, didReceiveData : data, fromPeer : peerID)
    }
    unsafe fn session_didReceiveStream_withName_fromPeer_(
        &self,
        session: MCSession,
        stream: NSInputStream,
        streamName: NSString,
        peerID: MCPeerID,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, session : session, didReceiveStream : stream, withName : streamName, fromPeer : peerID)
    }
    unsafe fn session_didStartReceivingResourceWithName_fromPeer_withProgress_(
        &self,
        session: MCSession,
        resourceName: NSString,
        peerID: MCPeerID,
        progress: NSProgress,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, session : session, didStartReceivingResourceWithName : resourceName, fromPeer : peerID, withProgress : progress)
    }
    unsafe fn session_didFinishReceivingResourceWithName_fromPeer_atURL_withError_(
        &self,
        session: MCSession,
        resourceName: NSString,
        peerID: MCPeerID,
        localURL: NSURL,
        error: NSError,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, session : session, didFinishReceivingResourceWithName : resourceName, fromPeer : peerID, atURL : localURL, withError : error)
    }
    unsafe fn session_didReceiveCertificate_fromPeer_certificateHandler_(
        &self,
        session: MCSession,
        certificate: NSArray,
        peerID: MCPeerID,
        certificateHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, session : session, didReceiveCertificate : certificate, fromPeer : peerID, certificateHandler : certificateHandler)
    }
}
impl MCSession_MCSessionCustomDiscovery for MCSession {}
pub trait MCSession_MCSessionCustomDiscovery: Sized + std::ops::Deref {
    unsafe fn nearbyConnectionDataForPeer_withCompletionHandler_(
        &self,
        peerID: MCPeerID,
        completionHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, nearbyConnectionDataForPeer : peerID, withCompletionHandler : completionHandler)
    }
    unsafe fn connectPeer_withNearbyConnectionData_(&self, peerID: MCPeerID, data: NSData)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, connectPeer : peerID, withNearbyConnectionData : data)
    }
    unsafe fn cancelConnectPeer_(&self, peerID: MCPeerID)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, cancelConnectPeer : peerID)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MCAdvertiserAssistant(pub id);
impl std::ops::Deref for MCAdvertiserAssistant {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MCAdvertiserAssistant {}
impl MCAdvertiserAssistant {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MCAdvertiserAssistant").unwrap(), alloc) })
    }
}
impl INSObject for MCAdvertiserAssistant {}
impl PNSObject for MCAdvertiserAssistant {}
impl std::convert::TryFrom<NSObject> for MCAdvertiserAssistant {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<MCAdvertiserAssistant, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MCAdvertiserAssistant").unwrap()) };
        if is_kind_of {
            Ok(MCAdvertiserAssistant(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to MCAdvertiserAssistant")
        }
    }
}
impl IMCAdvertiserAssistant for MCAdvertiserAssistant {}
pub trait IMCAdvertiserAssistant: Sized + std::ops::Deref {
    unsafe fn initWithServiceType_discoveryInfo_session_(
        &self,
        serviceType: NSString,
        info: NSDictionary,
        session: MCSession,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithServiceType : serviceType, discoveryInfo : info, session : session)
    }
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
    unsafe fn session(&self) -> MCSession
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, session)
    }
    unsafe fn discoveryInfo(&self) -> NSDictionary
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, discoveryInfo)
    }
    unsafe fn serviceType(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, serviceType)
    }
}
pub trait PMCAdvertiserAssistantDelegate: Sized + std::ops::Deref {
    unsafe fn advertiserAssistantWillPresentInvitation_(
        &self,
        advertiserAssistant: MCAdvertiserAssistant,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, advertiserAssistantWillPresentInvitation : advertiserAssistant)
    }
    unsafe fn advertiserAssistantDidDismissInvitation_(
        &self,
        advertiserAssistant: MCAdvertiserAssistant,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, advertiserAssistantDidDismissInvitation : advertiserAssistant)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MCNearbyServiceBrowser(pub id);
impl std::ops::Deref for MCNearbyServiceBrowser {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MCNearbyServiceBrowser {}
impl MCNearbyServiceBrowser {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MCNearbyServiceBrowser").unwrap(), alloc) })
    }
}
impl INSObject for MCNearbyServiceBrowser {}
impl PNSObject for MCNearbyServiceBrowser {}
impl std::convert::TryFrom<NSObject> for MCNearbyServiceBrowser {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<MCNearbyServiceBrowser, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MCNearbyServiceBrowser").unwrap()) };
        if is_kind_of {
            Ok(MCNearbyServiceBrowser(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to MCNearbyServiceBrowser")
        }
    }
}
impl IMCNearbyServiceBrowser for MCNearbyServiceBrowser {}
pub trait IMCNearbyServiceBrowser: Sized + std::ops::Deref {
    unsafe fn initWithPeer_serviceType_(
        &self,
        myPeerID: MCPeerID,
        serviceType: NSString,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithPeer : myPeerID, serviceType : serviceType)
    }
    unsafe fn startBrowsingForPeers(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, startBrowsingForPeers)
    }
    unsafe fn stopBrowsingForPeers(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, stopBrowsingForPeers)
    }
    unsafe fn invitePeer_toSession_withContext_timeout_(
        &self,
        peerID: MCPeerID,
        session: MCSession,
        context: NSData,
        timeout: NSTimeInterval,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, invitePeer : peerID, toSession : session, withContext : context, timeout : timeout)
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
    unsafe fn myPeerID(&self) -> MCPeerID
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, myPeerID)
    }
    unsafe fn serviceType(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, serviceType)
    }
}
pub trait PMCNearbyServiceBrowserDelegate: Sized + std::ops::Deref {
    unsafe fn browser_foundPeer_withDiscoveryInfo_(
        &self,
        browser: MCNearbyServiceBrowser,
        peerID: MCPeerID,
        info: NSDictionary,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, browser : browser, foundPeer : peerID, withDiscoveryInfo : info)
    }
    unsafe fn browser_lostPeer_(&self, browser: MCNearbyServiceBrowser, peerID: MCPeerID)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, browser : browser, lostPeer : peerID)
    }
    unsafe fn browser_didNotStartBrowsingForPeers_(
        &self,
        browser: MCNearbyServiceBrowser,
        error: NSError,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, browser : browser, didNotStartBrowsingForPeers : error)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MCBrowserViewController(pub id);
impl std::ops::Deref for MCBrowserViewController {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MCBrowserViewController {}
impl MCBrowserViewController {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MCBrowserViewController").unwrap(), alloc) })
    }
}
impl PMCNearbyServiceBrowserDelegate for MCBrowserViewController {}
impl PNSCoding for MCBrowserViewController {}
impl INSObject for MCBrowserViewController {}
impl PNSObject for MCBrowserViewController {}
impl std::convert::TryFrom<NSObject> for MCBrowserViewController {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<MCBrowserViewController, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MCBrowserViewController").unwrap()) };
        if is_kind_of {
            Ok(MCBrowserViewController(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to MCBrowserViewController")
        }
    }
}
impl IMCBrowserViewController for MCBrowserViewController {}
pub trait IMCBrowserViewController: Sized + std::ops::Deref {
    unsafe fn initWithServiceType_session_(
        &self,
        serviceType: NSString,
        session: MCSession,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithServiceType : serviceType, session : session)
    }
    unsafe fn initWithBrowser_session_(
        &self,
        browser: MCNearbyServiceBrowser,
        session: MCSession,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithBrowser : browser, session : session)
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
    unsafe fn browser(&self) -> MCNearbyServiceBrowser
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, browser)
    }
    unsafe fn session(&self) -> MCSession
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, session)
    }
    unsafe fn minimumNumberOfPeers(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, minimumNumberOfPeers)
    }
    unsafe fn setMinimumNumberOfPeers_(&self, minimumNumberOfPeers: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMinimumNumberOfPeers : minimumNumberOfPeers)
    }
    unsafe fn maximumNumberOfPeers(&self) -> NSUInteger
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, maximumNumberOfPeers)
    }
    unsafe fn setMaximumNumberOfPeers_(&self, maximumNumberOfPeers: NSUInteger)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, setMaximumNumberOfPeers : maximumNumberOfPeers)
    }
}
pub trait PMCBrowserViewControllerDelegate: Sized + std::ops::Deref {
    unsafe fn browserViewControllerDidFinish_(&self, browserViewController: MCBrowserViewController)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, browserViewControllerDidFinish : browserViewController)
    }
    unsafe fn browserViewControllerWasCancelled_(
        &self,
        browserViewController: MCBrowserViewController,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, browserViewControllerWasCancelled : browserViewController)
    }
    unsafe fn browserViewController_shouldPresentNearbyPeer_withDiscoveryInfo_(
        &self,
        browserViewController: MCBrowserViewController,
        peerID: MCPeerID,
        info: NSDictionary,
    ) -> BOOL
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, browserViewController : browserViewController, shouldPresentNearbyPeer : peerID, withDiscoveryInfo : info)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct MCNearbyServiceAdvertiser(pub id);
impl std::ops::Deref for MCNearbyServiceAdvertiser {
    type Target = objc2::runtime::AnyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc2::Message for MCNearbyServiceAdvertiser {}
impl MCNearbyServiceAdvertiser {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(&*objc2::runtime::AnyClass::get(c"MCNearbyServiceAdvertiser").unwrap(), alloc) })
    }
}
impl INSObject for MCNearbyServiceAdvertiser {}
impl PNSObject for MCNearbyServiceAdvertiser {}
impl std::convert::TryFrom<NSObject> for MCNearbyServiceAdvertiser {
    type Error = &'static str;
    fn try_from(parent: NSObject) -> Result<MCNearbyServiceAdvertiser, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(&*parent, isKindOfClass : objc2::runtime::AnyClass::get(c"MCNearbyServiceAdvertiser").unwrap()) };
        if is_kind_of {
            Ok(MCNearbyServiceAdvertiser(parent.0))
        } else {
            Err("This NSObject cannot be downcasted to MCNearbyServiceAdvertiser")
        }
    }
}
impl IMCNearbyServiceAdvertiser for MCNearbyServiceAdvertiser {}
pub trait IMCNearbyServiceAdvertiser: Sized + std::ops::Deref {
    unsafe fn initWithPeer_discoveryInfo_serviceType_(
        &self,
        myPeerID: MCPeerID,
        info: NSDictionary,
        serviceType: NSString,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, initWithPeer : myPeerID, discoveryInfo : info, serviceType : serviceType)
    }
    unsafe fn startAdvertisingPeer(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, startAdvertisingPeer)
    }
    unsafe fn stopAdvertisingPeer(&self)
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, stopAdvertisingPeer)
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
    unsafe fn myPeerID(&self) -> MCPeerID
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, myPeerID)
    }
    unsafe fn discoveryInfo(&self) -> NSDictionary
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, discoveryInfo)
    }
    unsafe fn serviceType(&self) -> NSString
    where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&**self, serviceType)
    }
}
pub trait PMCNearbyServiceAdvertiserDelegate: Sized + std::ops::Deref {
    unsafe fn advertiser_didReceiveInvitationFromPeer_withContext_invitationHandler_(
        &self,
        advertiser: MCNearbyServiceAdvertiser,
        peerID: MCPeerID,
        context: NSData,
        invitationHandler: *mut ::std::os::raw::c_void,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, advertiser : advertiser, didReceiveInvitationFromPeer : peerID, withContext : context, invitationHandler : invitationHandler)
    }
    unsafe fn advertiser_didNotStartAdvertisingPeer_(
        &self,
        advertiser: MCNearbyServiceAdvertiser,
        error: NSError,
    ) where
        <Self as std::ops::Deref>::Target: objc2::Message + Sized,
    {
        msg_send!(&** self, advertiser : advertiser, didNotStartAdvertisingPeer : error)
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
impl PNSEditor for MCBrowserViewController {}
unsafe extern "C" {
    pub static MCErrorDomain: NSString;
}
unsafe extern "C" {
    pub static kMCSessionMinimumNumberOfPeers: NSUInteger;
}
unsafe extern "C" {
    pub static kMCSessionMaximumNumberOfPeers: NSUInteger;
}

unsafe impl objc2::encode::RefEncode for MCPeerID {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MCPeerID {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MCSession {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MCSession {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MCAdvertiserAssistant {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MCAdvertiserAssistant {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MCNearbyServiceBrowser {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MCNearbyServiceBrowser {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MCBrowserViewController {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MCBrowserViewController {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
unsafe impl objc2::encode::RefEncode for MCNearbyServiceAdvertiser {
    const ENCODING_REF: objc2::encode::Encoding = objc2::encode::Encoding::Pointer(&<Self as objc2::encode::Encode>::ENCODING);
}
unsafe impl objc2::encode::Encode for MCNearbyServiceAdvertiser {
    const ENCODING: objc2::encode::Encoding = objc2::encode::Encoding::Object;
}
