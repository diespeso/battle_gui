use core::time::Duration;

use std::rc::Rc;
use std::cell::RefCell;

use ggez::{Context, GameResult};
use ggez::event::{self, EventHandler};
use ggez::graphics::{self, Drawable};

use ggez::timer;

use super::gui::{Status, StatusCard};
use super::tileset_parser::Tileset;
use super::sprite::Sprite;
use super::animation::*;


pub struct Game {
	pub stat_card: Option<Rc<RefCell<StatusCard>>>,
	pub tileset: Option<Tileset>,
	pub sprites: Vec<Sprite>,
	pub animations: Vec<Box<TimedCommand>>,
}

impl Game {
	
	pub fn new(ctx: &mut Context) -> Self {
		Self {
			stat_card: None,
			tileset: None,
			sprites: Vec::new(),
			animations: Vec::new(),
		}
	}
	
	pub fn set_status_card(&mut self, stat_card: Rc<RefCell<StatusCard>>) {
		self.stat_card = Some(stat_card);
	}
	
	pub fn set_tileset(&mut self, tileset: Tileset) {
		self.tileset = Some(tileset);
	}
	
	
	pub fn add_move(&mut self, cmd: Box<TimedCommand>) {
		self.animations.push(cmd);
	}
}

impl EventHandler for Game {
	fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
		let mut card = self.stat_card.as_mut()
			.expect("no status card");//borrow_mut();
		*card.borrow_mut().get_status_mut().hp.borrow_mut() += 1;
		//println!("ticks: {:#?}", timer::check_update_time(ctx, 30));
		if timer::check_update_time(ctx, 60){ //10 fps
			for cmd in &mut self.animations {
				cmd.step(card.clone());
			}
		
		}
		card.borrow_mut().update();
		Ok(())
	}
	
	fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
		graphics::clear(ctx, graphics::Color::new(0.1, 0.1, 0.1, 1.0));
		if let Some(tileset) = &self.tileset {
			tileset.draw(ctx);
		}
		/*self.stat_card.as_ref().expect("status card not initialized").draw(ctx, Default::default());*/
		self.stat_card.as_ref().expect("no status card set")
			.borrow().draw(ctx, Default::default());
		
		
		for sprite in &self.sprites {
			sprite.draw(ctx);
		}
		graphics::present(ctx)
	}
}
