use std::rc::Rc;
use std::cell::RefCell;
use std::borrow::Borrow;

use super::movable::Movable;
use ggez::mint::Point2;
use ggez::timer;
use core::time::Duration;
use ggez::Context;

pub trait Command<T> where T: Animatable {
	fn execute(&self, t: Rc<RefCell<T>>);
}

pub trait Animatable: Movable {
	
}

pub struct MoveCommand {
	vector: Point2<f32>,
}

impl MoveCommand {
	pub fn new(vector: Point2<f32>) -> Self {
		Self{vector}
	}
}

impl<T> Command<T> for MoveCommand where T: Animatable {
	fn execute(&self, t: Rc<RefCell<T>>) {
		println!("moved");
		t.borrow_mut().move_by(self.vector);
	}
}

pub struct LinearAnimation<T: Animatable> {
	animatable: Rc<RefCell<T>>,
	commands: Vec<Box<dyn Command<T>>>
}

impl<T> LinearAnimation<T> where T: Animatable {
	pub fn new(animatable: Rc<RefCell<T>>) -> Self {
		Self{
			animatable: animatable,
			commands: Vec::new(),
		}
	}
	
	pub fn add_command(&mut self, animatable: Box<Command<T>>) {
		self.commands.push(animatable);
	}
	
	pub fn update(&mut self, d: Duration) {
		for command in &self.commands {
			command.as_ref().execute(self.animatable.clone());
		}
	}
}
