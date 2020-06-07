use ggez::mint::Point2;

pub trait Movable {
	fn move_by(&mut self, vector: Point2<f32>);
	fn move_to(&mut self, position: Point2<f32>) -> Point2<f32>;
	
	fn debug_position(&self){}
}