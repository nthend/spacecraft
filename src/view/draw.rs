use view::engine::Handle;

pub trait Draw {
	fn draw(&mut self, handle: &Handle) -> Result<(), String>;
}