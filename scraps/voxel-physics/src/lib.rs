use std::{sync::Arc, time::SystemTime};

use engine_macros::{glue_enginedata, glue_runloop};

use engine_use::{
	concurrency::{self, get_enginedata_object},
	data::{
		object::engine_data::{EngineData, EngineDataObjectLookupResult},
		run::run_loop::RunLoop,
		runtime::graph::GraphData,
	},
};

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct VoxelPhysicsEngineData {
	#[serde(default = "SystemTime::now")]
	pub time: SystemTime,
	pub some_val_a: u32,
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
	fn run(self: Box<Self>, graph_data: Arc<GraphData>, scrap_id: &str, run_loop_id: &str) {
		println!("VoxelPhysicsRunLoop {}/{} entered!", scrap_id, run_loop_id);
		//one-time setup
		let engine_data = get_enginedata_object::<VoxelPhysicsEngineData>(graph_data.clone(), &scrap_id, &run_loop_id, &"voxel-physics".to_owned(), &"default_physics_data".to_owned());

		println!("got enginedata object for {}/{}", scrap_id, run_loop_id);

		let mut default_physics = match engine_data {
			EngineDataObjectLookupResult::Ok(default_physics) => default_physics,
			_ => panic!("Could not get default_physics_data, code {:#?}", engine_data),
		};

		// if run_loop_id == "something_else" {
		// 	panic!("some error");
		// }

		loop {
			let to_drop = concurrency::wait_for_parents(graph_data.clone(), &scrap_id, &run_loop_id);

			if let Err(_e) = to_drop {
				println!("loop {}/{} exiting because it cannot handle errored/closed state", scrap_id, run_loop_id);
				break;
			}

			let duration = default_physics.time.elapsed().unwrap();
			println!("{}:{}", duration.as_secs(), duration.as_nanos());
			//do stuff
			if default_physics.some_val_a != 1 {
				println!("physics (i): {}", default_physics.some_val_a);
				if default_physics.some_val_a % 2 == 0 {
					default_physics.some_val_a /= 2;
				} else {
					default_physics.some_val_a = default_physics.some_val_a * 3 + 1;
				}
				println!("physics (f): {}", default_physics.some_val_a);
			}
			//drop of _drop releases children
		}
	}
}
