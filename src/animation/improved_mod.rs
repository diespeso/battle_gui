extern crate core;

use core::time::Duration;
use std::rc::Rc;
use std::cell::RefCell;

type Vector = (i32, i32);

pub trait Animatable: Movable{
  fn  show_mut(&mut self) {
    println!("uwu");
  }
}

pub trait Movable {
  fn move_by(&mut self, vector: Vector) {panic!();}
}

pub trait Command {
  fn execute(&self, &mut dyn Animatable);
}

pub trait TimedCommand {
  fn step(&mut self, a: &mut dyn Animatable);
}

pub struct TimedMove{
  delta: f64,
  completion: f64,
  vector: Vector,
}

impl TimedMove {
  pub fn new(duration: Duration, vector: Vector)
    -> Self {
      Self {
        delta: 0.054,
        completion: 0.0,
        vector: vector,
      }
  }
}

impl Command for TimedMove {
  fn execute(&self, a: &mut dyn Animatable) {
    a.move_by(self.vector);
  }
}

impl TimedCommand for TimedMove {
  fn step(&mut self, a: &mut dyn Animatable) {
    println!("timed step");
    a.move_by(self.vector);
  }
}

pub struct TimedIdle {
  delta: f64,
  completion: f64,
}

impl TimedIdle {
  pub fn new(duration: Duration) -> Self {
    Self {
      delta: 0.033,
      completion: 0.0,
    }
  }
}

impl Command for TimedIdle {
  fn execute(&self, a: &mut dyn Animatable) {
    a.show_mut();
  }
}

impl TimedCommand for TimedIdle {
  fn step(&mut self, a: &mut dyn Animatable){
    println!("idle time!");

  }
}

#[derive(Debug)]
pub struct Sprite {
  vector: Vector,
}

impl Sprite {
  pub fn new(vector: Vector) -> Self {
    Self{vector,}
  }
}

impl Movable for Sprite {
  fn move_by(&mut self, vector: Vector) {
    self.vector.0 += vector.0.clone();
    self.vector.1 += vector.1.clone();
  }
}

impl Animatable for Sprite {

}

#[derive(Debug)]
pub struct GUI {
  pub sprite: Rc<RefCell<Sprite>>,
}

impl GUI {
  pub fn new(s: Rc<RefCell<Sprite>>) -> Self {
    Self {
      sprite: s,
    }
  }
}

pub struct Motor {
  pub animatable: Rc<RefCell<dyn Animatable>>,
  pub animations: Vec<Box<dyn TimedCommand>>,
}

impl Motor {
  pub fn new(animatable: Rc<RefCell<dyn Animatable>>) -> Self {
    Self{
      animatable,
      animations: Vec::new(),
    }
  }

  pub fn add_animation(&mut self, a: Box<dyn TimedCommand>) {
    self.animations.push(a);
  }

  pub fn run(&mut self) {
    for animation in &mut self.animations {
      animation.step(&mut *self.animatable.borrow_mut());
    }
  }
}

fn main() {
  let ptr = Rc::new(RefCell::new(Sprite::new((10, 10))));
  let mut motor = Motor::new(ptr.clone());
  let mut gui = GUI::new(ptr.clone());
  let mut cmd = TimedIdle::new(Duration::new(1, 0));
  let cmd = Box::new(cmd);
  motor.add_animation(cmd);
  motor.add_animation(Box::new(TimedMove::new(Duration::new(1, 0), (10, 10))));
  println!("{:#?}", ptr);
  motor.run();


  println!("{:#?}", ptr);
  println!("{:#?}", gui);
}
