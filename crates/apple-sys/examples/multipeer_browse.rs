//! Create a MultipeerConnectivity peer identity and browser.
//!
//! Demonstrates MCPeerID and MCNearbyServiceBrowser creation
//! for local network device discovery.

use apple_sys::MultipeerConnectivity::*;

mod common;
use common::nsstring;
use common::nsstring_to_string;

fn main() {
    unsafe {
        let pool = NSAutoreleasePool::alloc();
        let _pool = INSObject::init(&pool);

        println!("=== MultipeerConnectivity ===\n");

        // Create peer ID
        let display_name = nsstring(c"apple-sys-demo");
        let peer = MCPeerID::alloc();
        let peer = MCPeerID(IMCPeerID::initWithDisplayName_(&peer, display_name));
        assert!(!peer.0.is_null(), "Failed to create MCPeerID");

        let name = IMCPeerID::displayName(&peer);
        println!("Peer ID created: '{}'", nsstring_to_string(name));

        // Create session
        let session = MCSession::alloc();
        let session = MCSession(IMCSession::initWithPeer_(&session, peer));
        assert!(!session.0.is_null(), "Failed to create MCSession");

        let my_peer = IMCSession::myPeerID(&session);
        let my_name = IMCPeerID::displayName(&my_peer);
        println!("Session peer: '{}'", nsstring_to_string(my_name));

        let connected = IMCSession::connectedPeers(&session);
        let count = INSArray::<id>::count(&connected);
        println!("Connected peers: {}", count);

        // Create a nearby service browser
        let service_type = nsstring(c"apple-sys");
        let browser = MCNearbyServiceBrowser::alloc();
        let browser = MCNearbyServiceBrowser(IMCNearbyServiceBrowser::initWithPeer_serviceType_(
            &browser,
            peer,
            service_type,
        ));

        if !browser.0.is_null() {
            let stype = IMCNearbyServiceBrowser::serviceType(&browser);
            println!("Browser service type: '{}'", nsstring_to_string(stype));
            let bpeer = IMCNearbyServiceBrowser::myPeerID(&browser);
            let bname = IMCPeerID::displayName(&bpeer);
            println!("Browser peer: '{}'", nsstring_to_string(bname));
        }

        // Create nearby service advertiser
        let adv = MCNearbyServiceAdvertiser::alloc();
        let adv = MCNearbyServiceAdvertiser(
            IMCNearbyServiceAdvertiser::initWithPeer_discoveryInfo_serviceType_(
                &adv,
                peer,
                NSDictionary(std::ptr::null_mut()),
                service_type,
            ),
        );

        if !adv.0.is_null() {
            let atype = IMCNearbyServiceAdvertiser::serviceType(&adv);
            println!("Advertiser service type: '{}'", nsstring_to_string(atype));
        }
    }

    println!("\nDone.");
}
