use std::rc::Rc;
use std::cell::RefCell;
use std::borrow::Borrow;
use std::ops::Drop;

use super::movable::Movable;
use ggez::mint::Point2;
use ggez::timer;
use core::time::Duration;
use ggez::Context;

pub static FPS: f64 = 60.0;

pub struct Animation {
	pub animatable: Rc<RefCell<dyn Animatable>>,
	pub animations: Vec<Box<dyn TimedCommand>>,
	counter: i32,
}

impl Animation {
	pub fn new(a: Rc<RefCell<dyn Animatable>>) -> Self {
		Self {
			animatable: a,
			animations: Vec::new(),
			counter: 0,
		}
	}
	
	pub fn add_animation(&mut self, t_c: Box<dyn TimedCommand>) 
	{
		self.animations.push(t_c);
	}
	
	pub fn run(&mut self) {
		if self.counter < self.animations.len() as i32 {
			if self.animations[self.counter as  usize].is_completed() {
			//if this animation is done, get to the next one
				self.counter += 1;
			} else {
				self.animations[self.counter as usize].step(&mut *self.animatable.borrow_mut()); //step current animation
			}
		}
	}
}

pub trait TimedCommand {
	fn step(&mut self, a: &mut dyn Animatable);
	fn get_duration(&self) -> Duration;
	fn get_completion(&self) -> f64;
	fn is_completed(&self) -> bool;
}

pub trait Command {
	fn execute(&self, a: &mut dyn Animatable);
}

pub trait Animatable: Movable {
	
}

pub struct TimedMove {
	vector: Point2<f32>,
	duration: Duration,
	step: Point2<f32>,
	delta: f64,
	completion: f64,
}

impl TimedMove {
	pub fn new(vector: Point2<f32>, duration: Duration)
	-> Self {
		let (mut x, mut y) = (vector.x.clone(),
			vector.y.clone());

		let delta = 1.0 / (FPS * duration.as_secs_f64());
		x = x * delta as f32;
		y = y * delta as f32;		
		Self {
			vector: vector,
			duration: duration,
			step: Point2::<f32>::from_slice(&[x, y]),
			delta: delta,
			completion: 0.0,
		}
	}
}

impl Command for TimedMove {
	fn execute(&self, a: &mut dyn Animatable) {
		a.move_by(self.vector.clone());
	}
}

impl TimedCommand for TimedMove {
	fn step(&mut self, a: &mut dyn Animatable) {
		if self.is_completed() {
			return;
		} else {
			a.move_by(self.step.clone());
			self.completion += self.delta;
		}

	}
	
	fn get_duration(&self) -> Duration {
		self.duration.clone()
	}
	
	fn get_completion(&self) -> f64 {
		self.completion.clone()
	}
	
	fn is_completed(&self) -> bool {
		self.completion >= 1.0
	}
}

pub struct TimedIdle {
	delta: f64,
	duration: Duration,
	completion: f64,
}

impl TimedIdle {
	pub fn new(duration: Duration) -> Self {
		Self {
			delta: 1.0 / (FPS * duration.as_secs_f64()),
			duration,
			completion: 0.0,
		}
	}
}

impl Command for TimedIdle {
	fn execute(&self, a: &mut dyn Animatable) {
		//do nothing, kinda dumb
	}
}

impl TimedCommand for TimedIdle {
	fn step(&mut self, a: &mut dyn Animatable) {
		if self.is_completed() {
			return;
		} else {
			self.completion += self.delta;
		}
	}
	
	fn get_duration(&self) -> Duration {
		self.duration.clone()
	}
	
	fn get_completion(&self) -> f64 {
		self.completion.clone()
	}
	
	fn is_completed(&self) -> bool {
		self.completion >= 1.0
	}
}





