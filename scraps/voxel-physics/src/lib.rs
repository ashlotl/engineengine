use std::sync::Arc;

use engine_macros::{glue_enginedata, glue_runloop};

use engine_use::{
	data::EngineData,
	run::run_loop::RunLoop,
	sync::{self, GraphData},
};

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct VoxelPhysicsEngineData {
	some_val_a: u32,
	some_val_b: bool,
}

glue_enginedata!(VoxelPhysicsEngineData);

impl EngineData for VoxelPhysicsEngineData {}

// Here we define a struct that contains data underlying the RunLoop. Keep in mind that this data cannot be accessed by other RunLoops
#[derive(Debug, Deserialize)]
pub struct VoxelPhysicsRunLoop {
	some_val_1: String,
	some_val_2: Vec<bool>,
}

glue_runloop!(VoxelPhysicsRunLoop);

impl RunLoop for VoxelPhysicsRunLoop {
	fn run(self: Box<Self>, graph_data: Arc<GraphData>, scrap_id: String, run_loop_id: String) {
		println!("VoxelPhysicsRunLoop entered!");
		//one-time setup
		loop {
			let _drop = sync::wait_for_parents(graph_data.clone(), &scrap_id, &run_loop_id);
			//do stuff
			println!("{}", self.some_val_1);
			//drop of _drop releases children
		}
	}
}
