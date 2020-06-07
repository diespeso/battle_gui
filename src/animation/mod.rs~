use std::rc::Rc;
use std::cell::RefCell;
use std::borrow::Borrow;

use super::movable::Movable;
use ggez::mint::Point2;
use ggez::timer;
use core::time::Duration;
use ggez::Context;

pub static fps: u32 = 60;

pub trait TimedCommand<A>: Command<A> where A: Animatable {
	fn step(&mut self, t: Rc<RefCell<A>>);
}

pub trait Command {
	fn execute(&self, t: Rc<RefCell<T>>); //it may be better
	//to use mut references
}

pub trait Animatable: Movable {
	
}

pub struct TimedIdle {
	duration: Duration,
	delta: f64,
	completion: f64,
}

impl TimedIdle {
	pub fn new(duration: Duration) -> Self {
		Self {
			duration: duration,
			delta: 1.0 / ((fps as f64) * duration.as_secs_f64()),
			completion: 0.0,
		}
	}	
}

impl<A> Command<A> for TimedIdle where A: Animatable {
	fn execute(&self, t: Rc<RefCell<A>>) {
		//do nothing
	}
}

impl<A> TimedCommand<A> for TimedIdle where A: Animatable {
	fn step(&mut self, t: Rc<RefCell<A>>) {
		if self.completion >= 1.0  {
			return;			
		} else {
			self.completion += self.delta;
		}	
	}
}

pub struct TimedMove {
	command: MoveCommand,
	duration: Duration,
	step_command: MoveCommand,
	delta: f64,
	completion: f64,
}

impl TimedMove {
	pub fn new(command: MoveCommand, duration: Duration)
	-> Self {
		let (mut x, mut y) = (command.vector.x.clone(), command.vector.y.clone());
		//println!("{:#?}", duration.as_millis());
		//x and y how will they change per second?
		/*x = x / duration.as_millis() as f32;
		y = y / duration.as_millis() as f32;*/
		let delta = 1.0 / ((fps as f64) * duration.as_secs_f64());
		x = x * delta as f32;
		y = y * delta as f32;		
		//makes a subcommand that will step its magnitde per second
		Self {
			command: command,
			duration: duration,
			step_command: MoveCommand::new([x, y].into()),
			delta: delta,
			completion: 0.0,
		}
	}
}

impl<A> Command<A> for TimedMove where A: Animatable {
	fn execute(&self, a: Rc<RefCell<A>>) {
		self.command.execute(a);
	}
}

impl<A> TimedCommand<A> for TimedMove where A: Animatable {
	fn step(&mut self, a: Rc<RefCell<A>>) {
		if self.completion >= 1.0  {
			return;			
		} else {
			self.step_command.execute(a);
			self.completion += self.delta;
		}		
	}
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
