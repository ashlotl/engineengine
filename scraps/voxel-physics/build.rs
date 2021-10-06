use engine_build::push_reload_ron;

fn main() {
	push_reload_ron(env!("CARGO_PKG_NAME"));
}
