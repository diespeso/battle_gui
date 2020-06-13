use std::cell::RefCell;
use std::rc::Rc;

use super::sprite::Sprite;
use ggez::graphics::{self, DrawParam, Rect, BlendMode,
    Text, Font, Scale};
use ggez::error::GameResult;
use ggez::Context;

use ggez::mint::{Vector2, Point2};

use super::utils::{*};
use super::movable::Movable;
use super::animation::Animatable;

#[derive(Debug, Clone)]
pub struct Status {
    pub name: &'static str,
    pub hp: Rc<RefCell<i32>>,
}

impl Status {
    pub fn new(name: &'static str, hp: i32) -> Self {
        Self {
            name,
            hp: Rc::new(RefCell::new(hp)),
        }
    }
    
    pub fn hp(&self) -> Rc<RefCell<i32>> {
        self.hp.clone()
    }

    pub fn test_update(&mut self) {
        *self.hp.borrow_mut() += 1;
    }
    
    pub fn set_hp(&mut self, num: i32) {
        *self.hp.borrow_mut() = num;
    }
}
