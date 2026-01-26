//! Create SpriteKit actions and nodes.
//!
//! Demonstrates SKAction, SKSpriteNode, and SKScene creation
//! for 2D game development.

use apple_sys::AppKit::*;
use apple_sys::CoreFoundation::CGSize;
use apple_sys::SpriteKit::*;

mod common;

fn main() {
    unsafe {
        let pool = NSAutoreleasePool::alloc();
        let _pool = INSObject::init(&pool);

        println!("=== SpriteKit Actions & Nodes ===\n");

        // Create SKActions
        let move_action = SKAction::moveByX_y_duration_(100.0, 200.0, 1.0);
        let dur = ISKAction::duration(&move_action);
        println!("Move action: duration={:.1}s", dur);

        let rotate = SKAction::rotateByAngle_duration_(3.14159, 0.5);
        let dur = ISKAction::duration(&rotate);
        println!("Rotate action: duration={:.1}s", dur);

        let fade_out = SKAction::fadeOutWithDuration_(2.0);
        let dur = ISKAction::duration(&fade_out);
        println!("Fade out action: duration={:.1}s", dur);

        let scale = SKAction::scaleBy_duration_(2.0, 0.3);
        let dur = ISKAction::duration(&scale);
        println!("Scale action: duration={:.1}s", dur);

        let wait = SKAction::waitForDuration_(1.5);
        let dur = ISKAction::duration(&wait);
        println!("Wait action: duration={:.1}s", dur);

        // Create a sequence
        let actions: [id; 3] = [move_action.0, rotate.0, fade_out.0];
        let array = <NSArray as NSArray_NSArrayCreation<id>>::arrayWithObjects_count_(
            actions.as_ptr() as *const *mut u64,
            3,
        );
        let sequence = SKAction::sequence_(NSArray(array));
        let dur = ISKAction::duration(&sequence);
        println!("Sequence (3 actions): total duration={:.1}s", dur);

        // Create sprite nodes
        let color = NSColor::redColor();

        let size = CGSize {
            width: 50.0,
            height: 50.0,
        };
        let node = SKSpriteNode(SKSpriteNode::spriteNodeWithColor_size_(color, size));

        let node_size = ISKSpriteNode::size(&node);
        println!("\nSprite node: {}x{}", node_size.width, node_size.height);

        let alpha = ISKNode::alpha(&SKNode(node.0));
        println!("  Alpha: {}", alpha);
        let hidden = ISKNode::isHidden(&SKNode(node.0));
        println!("  Hidden: {}", hidden.0);

        // Create a scene
        let scene_size = CGSize {
            width: 1024.0,
            height: 768.0,
        };
        let scene = SKScene(SKScene::sceneWithSize_(scene_size));
        let s = ISKScene::size(&scene);
        println!("\nScene: {}x{}", s.width, s.height);

        // Add node to scene
        ISKNode::addChild_(&SKNode(scene.0), SKNode(node.0));
        let children = ISKNode::children(&SKNode(scene.0));
        let count = INSArray::<id>::count(&children);
        println!("  Children: {}", count);
    }

    println!("\nDone.");
}
