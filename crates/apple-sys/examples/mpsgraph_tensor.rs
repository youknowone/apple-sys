//! Create MPSGraph and placeholder tensors.
//!
//! Demonstrates MetalPerformanceShadersGraph by creating an MPSGraph,
//! adding placeholder tensors, and inspecting the graph structure.

use apple_sys::MetalPerformanceShadersGraph::*;

mod common;
use common::nsstring;
use common::nsstring_to_string;

fn main() {
    unsafe {
        let pool = NSAutoreleasePool::alloc();
        let _pool = INSObject::init(&pool);

        println!("=== MetalPerformanceShadersGraph ===\n");

        // Create MPSGraph
        let graph = MPSGraph::alloc();
        let graph = MPSGraph(INSObject::init(&graph));
        if graph.0.is_null() {
            println!("MPSGraph: failed to create");
            return;
        }

        println!("MPSGraph created: {}", common::nsobj_to_string(graph.0));

        // Check options
        let options = IMPSGraph::options(&graph);
        let opt_name = match options {
            0 => "None",
            1 => "SynchronizeResults",
            2 => "Verbose",
            _ => "Other",
        };
        println!("  Options: {} ({})", opt_name, options);

        // Create placeholder tensors with shape [1, 3, 224, 224]
        let n1 = NSNumber::numberWithInteger_(1);
        let n3 = NSNumber::numberWithInteger_(3);
        let n224 = NSNumber::numberWithInteger_(224);
        let shape_objs: [id; 4] = [n1.0, n3.0, n224.0, n224.0];
        let shape = <NSArray as NSArray_NSArrayCreation<id>>::arrayWithObjects_count_(
            shape_objs.as_ptr() as *const *mut u64,
            4,
        );

        // MPSDataTypeFloat32 = 0x10000000 | 32
        let data_type: u32 = 0x10000000 | 32;
        let name = nsstring(c"input_image");

        let placeholder = MPSGraph_MemoryOps::placeholderWithShape_dataType_name_(
            &graph,
            NSArray(shape),
            data_type,
            name,
        );

        if !placeholder.0.is_null() {
            println!("\nPlaceholder tensor:");
            let tensor_shape = NSArray(IMPSGraphTensor::shape(&placeholder) as id);
            if !tensor_shape.0.is_null() {
                let count = INSArray::<id>::count(&tensor_shape);
                print!("  Shape: [");
                for i in 0..count {
                    let n = NSNumber(INSArray::<id>::objectAtIndex_(&tensor_shape, i));
                    let v = INSNumber::integerValue(&n);
                    if i > 0 {
                        print!(", ");
                    }
                    print!("{}", v);
                }
                println!("]");
            }

            let dt = IMPSGraphTensor::dataType(&placeholder);
            let dt_name = match dt {
                0x10000020 => "Float32",
                0x10000010 => "Float16",
                0x20000020 => "Int32",
                0x20000008 => "Int8",
                _ => "Other",
            };
            println!("  DataType: {} (0x{:X})", dt_name, dt);
        }

        // Create a second placeholder for weights [64, 3, 3, 3]
        let w1 = NSNumber::numberWithInteger_(64);
        let w2 = NSNumber::numberWithInteger_(3);
        let w3 = NSNumber::numberWithInteger_(3);
        let w4 = NSNumber::numberWithInteger_(3);
        let weight_shape_objs: [id; 4] = [w1.0, w2.0, w3.0, w4.0];
        let weight_shape = <NSArray as NSArray_NSArrayCreation<id>>::arrayWithObjects_count_(
            weight_shape_objs.as_ptr() as *const *mut u64,
            4,
        );

        let weight_name = nsstring(c"conv_weights");
        let weight_tensor = MPSGraph_MemoryOps::placeholderWithShape_dataType_name_(
            &graph,
            NSArray(weight_shape),
            data_type,
            weight_name,
        );

        if !weight_tensor.0.is_null() {
            println!("\nWeight tensor:");
            let wt_desc = PNSObject::description(&weight_tensor);
            println!("  {}", nsstring_to_string(wt_desc));
        }

        // Check placeholder feeds
        let placeholder_tensors = IMPSGraph::placeholderTensors(&graph);
        if !placeholder_tensors.0.is_null() {
            let count = INSArray::<id>::count(&placeholder_tensors);
            println!("\nGraph has {} placeholder tensors", count);
        }

        // Set graph options to Verbose
        IMPSGraph::setOptions_(&graph, 2); // MPSGraphOptionsVerbose
        let new_options = IMPSGraph::options(&graph);
        println!("After setOptions(Verbose): {}", new_options);
    }

    println!("\nDone.");
}
