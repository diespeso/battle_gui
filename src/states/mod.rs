use crate::tileset_parser::Tileset;
use crate::utils::{*};
use crate::movable::Movable;
use crate::animation::{*};

use std::rc::Rc;
use std::cell::RefCell;

use ggez::Context;
use ggez::graphics::{Drawable, Text, DrawParam, Image,
	spritebatch::{SpriteBatch, SpriteIdx}};
use ggez::mint::{Point2, Vector2};
use core::time::Duration;

use std::collections::HashMap;

use crate::sprite::Sprite;

#[derive(Debug)]
pub struct BattleGuiHandler {
	tileset: Tileset,
	commands: HashMap<String, Rc<RefCell<GuiCommandBanner>>>,
}

impl BattleGuiHandler {
	pub fn new(tileset: Tileset) -> Self {
		Self {
			tileset,
			commands: HashMap::new(),
		}
	}
	
	pub fn add_command_banner(&mut self, name: String) {
		self.commands.insert(name, Rc::new(
			RefCell::new(
			GuiCommandBanner::new(&self.tileset
			)
			)
		));
	}
	
	pub fn get_command(&self, name: String) -> Rc<RefCell<GuiCommandBanner>> {
		self.commands[&name].clone()
	}
	
	pub fn draw(&self, ctx: &mut Context) {
		for command in self.commands.values() {
			command.borrow().draw(ctx);
		}
	}
	
	
}

#[derive(Debug)]
pub struct GuiCommandBanner {
	position: Point2<f32>,
	portrait_box: Sprite,
	portrait: Sprite,
	text_box: Sprite,
	text: Option<Text>,
}

impl GuiCommandBanner {
	
	pub fn new(tileset: &Tileset) -> Self {
		let mut result = Self {
			position: Point2::<f32>{x: 0.0, y: 0.0},
			portrait_box: tileset.get("skin_command_off".to_string()),
			portrait: tileset.get("por_sword".to_string()),
			text_box: tileset.get("skin_command_on".to_string()),
			text: None,
		};
		
		result.text_box.move_by(vector_2f(32.0, 0.0));
		
		result.move_by(vector_2f(8.0, 8.0));
		result
	}
	
	pub fn draw(&self, ctx: &mut Context) {
		self.portrait_box.draw(ctx);
		self.portrait.draw(ctx);
		self.text_box.draw(ctx);
	}
}

impl Animatable for GuiCommandBanner {

}

impl Movable for GuiCommandBanner {
	fn move_by(&mut self, vector: Point2<f32>) {
		self.portrait_box.move_by(vector.clone());
		self.portrait.move_by(vector);
		self.text_box.move_by(vector);
	}
}

