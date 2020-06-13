use std::rc::Rc;
use std::cell::RefCell;
use crate::sprite::Sprite;

pub mod gui;

pub type SpritePointer = Rc<RefCell<Sprite>>;
