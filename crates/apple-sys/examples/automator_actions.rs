//! Access the Automator shared workspace and inspect available actions.
//!
//! Uses AMWorkspace to query the Automator action registry,
//! creates an AMWorkflow and an AMWorkflowController.

use apple_sys::Automator::*;

mod common;
use common::nsobj_to_string;

fn main() {
    unsafe {
        let pool = NSAutoreleasePool::alloc();
        let _pool = INSObject::init(&pool);

        println!("=== Automator Workspace ===\n");

        let ws = AMWorkspace::sharedWorkspace();
        if ws.0.is_null() {
            println!("Failed to get AMWorkspace.sharedWorkspace.");
            return;
        }
        println!("AMWorkspace.sharedWorkspace obtained.");

        // Create a new empty workflow
        let wf = AMWorkflow::alloc();
        let wf_ptr = IAMWorkflow::init(&wf);
        if wf_ptr.is_null() {
            println!("Failed to create AMWorkflow.");
            return;
        }
        let wf = AMWorkflow(wf_ptr);
        println!("\nEmpty AMWorkflow created.");

        let actions = IAMWorkflow::actions(&wf);
        if !actions.0.is_null() {
            let count = INSArray::<id>::count(&actions);
            println!("Actions in empty workflow: {count}");
        }

        let output = IAMWorkflow::output(&wf);
        println!("Workflow output: {}", nsobj_to_string(output));

        let input = IAMWorkflow::input(&wf);
        println!("Workflow input: {}", nsobj_to_string(input));

        // Create an AMWorkflowController and attach the workflow
        let ctrl = AMWorkflowController::alloc();
        let ctrl_ptr = INSObject::init(&ctrl);
        if !ctrl_ptr.is_null() {
            let ctrl = AMWorkflowController(ctrl_ptr);
            println!("\nAMWorkflowController created.");

            IAMWorkflowController::setWorkflow_(&ctrl, AMWorkflow(wf.0));

            let can_run = IAMWorkflowController::canRun(&ctrl);
            let is_running = IAMWorkflowController::isRunning(&ctrl);
            let is_paused = IAMWorkflowController::isPaused(&ctrl);
            println!("  Can run:    {}", can_run.0);
            println!("  Is running: {}", is_running.0);
            println!("  Is paused:  {}", is_paused.0);
        }
    }

    println!("\nDone.");
}
