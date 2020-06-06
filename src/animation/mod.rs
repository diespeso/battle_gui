use super::movable::Movable;
use ggez::mint::Point2;
use ggez::timer;
use core::time::Duration;
use ggez::Context;


pub struct AnimationEngine {
	pub animations: Vec<Box< dyn LinearAnimation>>,
}

impl AnimationEngine {
	pub fn new() -> Self {
		Self { animations: Vec::new()}
	}
	
	pub fn update(&mut self, d: Duration) {
		for animation in &mut self.animations {
			animation.update(d);
		}
	}
	
	pub fn push_linear(&mut self, animation: Box<dyn LinearAnimation>) {
		self.animations.push(animation);
	}
}

//A linear animation has a collection of commands 
//that will be made in a given time.
pub trait LinearAnimation {
	fn get_duration(&self) -> Duration;
	fn update(&mut self, d: Duration);
}

impl LinearAnimation for DurableCommand<MoveCommand> {
	fn get_duration(&self) -> Duration {
		self.duration.clone()
	}
	
	fn update(&mut self, d: Duration) {
		println!("{:#?}", "update movable animation");
	}
}

pub struct DurableCommand<T> {
	command: Box<dyn Command<T>>,
	duration: Duration,
}	

impl<T> DurableCommand<T> {
	pub fn new(command: Box<dyn Command<T>>, duration: Duration) -> Self {
		Self {
			command,
			duration,
		}
	}
}

pub trait Command<T> {
	fn execute(&mut self, t: &mut T);
	fn undo(&mut self, t: &mut T) {
		unimplemented!();
	}
}

#[derive(Debug)]
pub struct MoveCommand {
	vector: Point2<f32>,
}

impl MoveCommand {
	pub fn new(vector: Point2<f32>) -> Self {
		Self{vector}
	}
	
}

impl Movable for MoveCommand {
	fn move_by(&mut self, vector: Point2<f32>) {
		unimplemented!();
	}
	
	fn move_to(&mut self, position: Point2<f32>) -> Point2<f32> {
		unimplemented!()
	}
}

impl<T: Movable> Command<T> for MoveCommand {
	fn execute(&mut self, t: &mut T) {
		t.move_by(self.vector);
	}
}

