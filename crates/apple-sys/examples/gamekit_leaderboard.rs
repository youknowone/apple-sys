//! Query Game Center player and leaderboard info.
//!
//! Uses GKLocalPlayer and GKLeaderboard to check
//! Game Center authentication status.

use apple_sys::GameKit::*;

mod common;
use common::nsstring_to_string;

fn main() {
    unsafe {
        let pool = NSAutoreleasePool::alloc();
        let _pool = INSObject::init(&pool);

        println!("=== GameKit (Game Center) ===\n");

        // Local player
        let local = GKLocalPlayer::local();

        if !local.0.is_null() {
            let authenticated = <GKLocalPlayer as IGKLocalPlayer>::isAuthenticated(&local);
            let underage = <GKLocalPlayer as IGKLocalPlayer>::isUnderage(&local);
            let multiplayer =
                <GKLocalPlayer as IGKLocalPlayer>::isMultiplayerGamingRestricted(&local);

            println!("Local player:");
            println!("  Authenticated: {}", authenticated.0);
            println!("  Underage: {}", underage.0);
            println!("  Multiplayer restricted: {}", multiplayer.0);

            if authenticated.0 {
                let display = <GKLocalPlayer as IGKPlayer>::displayName(&local);
                let alias = <GKLocalPlayer as IGKPlayer>::alias(&local);
                let game_id = <GKLocalPlayer as IGKPlayer>::gamePlayerID(&local);
                let team_id = <GKLocalPlayer as IGKPlayer>::teamPlayerID(&local);
                println!("  Display name: {}", nsstring_to_string(display));
                println!("  Alias: {}", nsstring_to_string(alias));
                println!("  Game player ID: {}", nsstring_to_string(game_id));
                println!("  Team player ID: {}", nsstring_to_string(team_id));
            } else {
                println!("  (Not signed in to Game Center)");
            }
        }

        // GKAccessPoint
        let shared = GKAccessPoint::shared();
        if !shared.0.is_null() {
            let active = <GKAccessPoint as IGKAccessPoint>::isActive(&shared);
            let visible = <GKAccessPoint as IGKAccessPoint>::isVisible(&shared);
            println!("\nGKAccessPoint:");
            println!("  Active: {}", active.0);
            println!("  Visible: {}", visible.0);
        }
    }

    println!("\nDone.");
}
