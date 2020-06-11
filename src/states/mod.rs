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

type SpritePointer = Rc<RefCell<Sprite>>;

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

pub enum CommandBannerType {
	ATTACK (AttackType),
	FORM,
	CONCEPTUALIZE,
	CONSUME,
	EXIST,	
}

pub enum AttackType {
	SWORD,
	GUN,
	BOW,
	CANNON,
	HAMMER,
	STAFF,
	UNHOLY_BOW,
	BLASTER,
	AXE,
	INSTRUMENT,
	BIO_BLADE,
	NULL,
}

pub static PORTRAIT_BOX: &'static str = "skin_command_off";
pub static PORTRAIT: &'static str = "por_sword";
pub static TEXT_BOX: &'static str = "skin_command_on";

#[derive(Debug)]
pub struct GuiCommandBanner {
	position: Point2<f32>,
	pieces: HashMap<String, SpritePointer>,
	/*portrait_box: SpritePointer,
	portrait: SpritePointer,
	text_box: SpritePointer,
	text: Option<Text>,*/
}

impl GuiCommandBanner {
	
	pub fn new(tileset: &Tileset) -> Self {
		/*let mut result = Self {
			position: Point2::<f32>{x: 0.0, y: 0.0},
			portrait_box: Rc::new(RefCell::new(tileset.get("skin_command_off".to_string()))),
			portrait: Rc::new(RefCell::new(tileset.get("por_sword".to_string()))),
			text_box: Rc::new(RefCell::new(tileset.get("skin_command_on".to_string()))),
			text: None,
		};*/
		
		let mut result = Self {
			position: vector_2f(0.0, 0.0),
			pieces: HashMap::new(),
		};
		
		//TODO: BAD CODE DUPLICATION
		result.pieces.insert(
			PORTRAIT_BOX.to_string(),
			Rc::new(RefCell::new(tileset.get(PORTRAIT_BOX.to_string()))
		));
		
		result.pieces.insert(
			PORTRAIT.to_string(),
			Rc::new(RefCell::new(
				tileset.get(PORTRAIT.to_string())
			))
		);
		
		result.pieces.insert(
			TEXT_BOX.to_string(),
			Rc::new(RefCell::new(
				tileset.get(TEXT_BOX.to_string())
			))
		);
		
		result.pieces[TEXT_BOX].borrow_mut().move_by(vector_2f(32.0, 0.0));
		
		result.move_by(vector_2f(8.0, 8.0));
		result
	}
	
	pub fn draw(&self, ctx: &mut Context) {
		/*self.pieces[PORTRAIT_BOX].borrow().draw(ctx);
		self.pieces[PORTRAIT].borrow().draw(ctx);
		self.pieces[TEXT_BOX].borrow().draw(ctx);*/
		
		for piece in self.pieces.values() {
			piece.borrow().draw(ctx);
		}
	}
	
	pub fn get_piece(&self, name: String) -> Rc<RefCell<dyn Animatable>> {
		self.pieces[&name].clone()
	}
	
	
}

impl Animatable for GuiCommandBanner {

}

impl Movable for GuiCommandBanner {
	fn move_by(&mut self, vector: Point2<f32>) {
		/*self.pieces[PORTRAIT_BOX].borrow_mut().move_by(vector.clone());
		self.pieces[PORTRAIT].borrow_mut().move_by(vector);
		self.pieces[TEXT_BOX].borrow_mut().move_by(vector);*/
		for piece in self.pieces.values() {
			piece.borrow_mut().move_by(vector);
		}
	}
}

