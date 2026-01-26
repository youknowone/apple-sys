//! Create SceneKit 3D geometry primitives.
//!
//! Demonstrates creating SCNBox, SCNSphere, SCNCylinder, SCNCone
//! and querying their geometric properties.

use apple_sys::SceneKit::*;

mod common;
use common::nsobj_to_string;

fn main() {
    unsafe {
        let pool = NSAutoreleasePool::alloc();
        let _pool = INSObject::init(&pool);

        println!("=== SceneKit Geometry ===\n");

        // SCNBox
        let box_ = SCNBox(SCNBox::boxWithWidth_height_length_chamferRadius_(
            2.0, 3.0, 4.0, 0.5,
        ));
        let w = ISCNBox::width(&box_);
        let h = ISCNBox::height(&box_);
        let l = ISCNBox::length(&box_);
        let c = ISCNBox::chamferRadius(&box_);
        println!("SCNBox: {}x{}x{} (chamfer: {})", w, h, l, c);

        // SCNSphere
        let sphere = SCNSphere(SCNSphere::sphereWithRadius_(5.0));
        let r = ISCNSphere::radius(&sphere);
        let seg = ISCNSphere::segmentCount(&sphere);
        println!("SCNSphere: radius={}, segments={}", r, seg);

        // SCNCylinder
        let cyl = SCNCylinder(SCNCylinder::cylinderWithRadius_height_(1.5, 4.0));
        let r = ISCNCylinder::radius(&cyl);
        let h = ISCNCylinder::height(&cyl);
        println!("SCNCylinder: radius={}, height={}", r, h);

        // SCNCone
        let cone = SCNCone(SCNCone::coneWithTopRadius_bottomRadius_height_(
            0.0, 2.0, 5.0,
        ));
        let tr = ISCNCone::topRadius(&cone);
        let br = ISCNCone::bottomRadius(&cone);
        let h = ISCNCone::height(&cone);
        println!(
            "SCNCone: topRadius={}, bottomRadius={}, height={}",
            tr, br, h
        );

        // SCNTorus
        let torus = SCNTorus(SCNTorus::torusWithRingRadius_pipeRadius_(3.0, 0.5));
        let rr = ISCNTorus::ringRadius(&torus);
        let pr = ISCNTorus::pipeRadius(&torus);
        println!("SCNTorus: ringRadius={}, pipeRadius={}", rr, pr);

        // Create a scene with a node
        let scene = SCNScene(SCNScene::scene());
        let root = ISCNScene::rootNode(&scene);

        let node = SCNNode::nodeWithGeometry_(SCNGeometry(sphere.0));
        ISCNNode::addChildNode_(&root, node);

        let children = ISCNNode::childNodes(&root);
        let count = INSArray::<id>::count(&children);
        println!("\nScene root has {} child node(s)", count);

        // Describe the scene
        println!("Scene: {}", nsobj_to_string(scene.0));
    }

    println!("\nDone.");
}
