//! Create 3D mesh primitives with ModelIO.
//!
//! Demonstrates MDLMesh factory methods for generating common
//! geometric shapes: box, sphere, and plane, then inspects their
//! vertex data and creates an MDLAsset.

use apple_sys::CoreFoundation::INSObject;
use apple_sys::Foundation::{INSArray, NSArray, NSAutoreleasePool};
use apple_sys::ModelIO::*;
use apple_sys::objc::{BOOL, id};

mod common;
use common::nsstring_to_string;

fn main() {
    unsafe {
        let pool = NSAutoreleasePool::alloc();
        let _pool = INSObject::init(&pool);

        println!("=== ModelIO Mesh Primitives ===\n");

        // Create a buffer allocator
        let allocator = MDLMeshBufferDataAllocator::alloc();
        let allocator = MDLMeshBufferDataAllocator(INSObject::init(&allocator));

        // MDLMesh box
        let extent: vector_float3 = [2.0, 2.0, 2.0];
        let segments: vector_uint3 = [1, 1, 1];
        let box_mesh_ptr =
            <MDLMesh as MDLMesh_Generators>::newBoxWithDimensions_segments_geometryType_inwardNormals_allocator_(
                extent,
                segments,
                0, // MDLGeometryTypeTriangles
                BOOL(false as _),
                allocator.0 as *mut u64,
            );
        let box_mesh = MDLMesh(box_mesh_ptr as id);

        if !box_mesh.0.is_null() {
            let vert_count = IMDLMesh::vertexCount(&box_mesh);
            let submeshes = IMDLMesh::submeshes(&box_mesh);
            let submesh_count = if !submeshes.0.is_null() {
                INSArray::<id>::count(&NSArray(submeshes.0))
            } else {
                0
            };
            println!(
                "Box (2x2x2): {} vertices, {} submeshes",
                vert_count, submesh_count
            );
        }

        // MDLMesh sphere
        let radii: vector_float3 = [2.0, 2.0, 2.0];
        let sphere_ptr =
            <MDLMesh as MDLMesh_Generators>::newEllipsoidWithRadii_radialSegments_verticalSegments_geometryType_inwardNormals_hemisphere_allocator_(
                radii,
                16,
                16,
                0, // MDLGeometryTypeTriangles
                BOOL(false as _),
                BOOL(false as _),
                allocator.0 as *mut u64,
            );
        let sphere = MDLMesh(sphere_ptr as id);

        if !sphere.0.is_null() {
            let vert_count = IMDLMesh::vertexCount(&sphere);
            println!("Sphere (r=1, 16x16): {} vertices", vert_count);
        }

        // MDLMesh plane
        let plane_ptr =
            <MDLMesh as MDLMesh_Generators>::newPlaneWithDimensions_segments_geometryType_allocator_(
                [10.0f32, 10.0f32],
                [4u32, 4u32],
                0, // MDLGeometryTypeTriangles
                allocator.0 as *mut u64,
            );
        let plane = MDLMesh(plane_ptr as id);

        if !plane.0.is_null() {
            let vert_count = IMDLMesh::vertexCount(&plane);
            println!("Plane (10x10, 4x4): {} vertices", vert_count);
        }

        // Vertex descriptor from box mesh
        if !box_mesh.0.is_null() {
            let desc = IMDLMesh::vertexDescriptor(&box_mesh);
            if !desc.0.is_null() {
                let attrs = IMDLVertexDescriptor::attributes(&desc);
                if !attrs.0.is_null() {
                    let attr_count = INSArray::<id>::count(&attrs);
                    println!("\nBox vertex attributes: {}", attr_count);
                    for i in 0..attr_count {
                        let attr = INSArray::<id>::objectAtIndex_(&attrs, i);
                        let attr = MDLVertexAttribute(attr);
                        let name = IMDLVertexAttribute::name(&attr);
                        println!("  {}: {}", i, nsstring_to_string(name));
                    }
                }
            }
        }

        // Create an asset and add the box mesh
        let asset = MDLAsset::alloc();
        let asset = MDLAsset(IMDLAsset::initWithBufferAllocator_(
            &asset,
            allocator.0 as *mut u64,
        ));
        if !box_mesh.0.is_null() {
            IMDLAsset::addObject_(&asset, MDLObject(box_mesh.0));
        }
        let obj_count = IMDLAsset::count(&asset);
        println!("\nAsset objects: {}", obj_count);
    }

    println!("\nDone.");
}
