mod utils;
mod sprite;
mod gui;
mod tileset_parser;

use ggez::mint::Point2;
use ggez::{Context, GameResult, ContextBuilder};
use ggez::event::{self, EventHandler};
use ggez::graphics::{self, Drawable};
use ggez::filesystem;

use std::path::Path;
use std::rc::Rc;
use std::cell::RefCell;

use sprite::{Sprite, SpriteData};
use gui::StatusCard;

struct Game {
	stat_card: Option<StatusCard>,
}

impl Game {
	
	pub fn new(ctx: &mut Context) -> Self {
		Self {
			stat_card: None,
		}
	}
	
	pub fn set_status_card(&mut self, stat_card: StatusCard) {
		self.stat_card = Some(stat_card);
	}
}

impl EventHandler for Game {
	fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
		Ok(())
	}
	
	fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
		graphics::clear(ctx, graphics::Color::new(0.1, 0.1, 0.1, 1.0));
		self.stat_card.as_ref().expect("status card not initialized").draw(ctx, Default::default());
		graphics::present(ctx)
	}
}

fn main() {
    let (mut ctx, mut event_loop) = ContextBuilder::new("game", "diespeso").build().expect("contexto no iniciado");
    let mut game = Game::new(&mut ctx);
    let image = graphics::Image::new(&mut ctx, Path::new("/assets/battle_gui.png")).expect("No se puedo cargar imagen");
    let image = Rc::new(RefCell::new(image));
    let mut sprite_data = SpriteData::new(image.clone());
    let mut sprite = Sprite::from_data(sprite_data);
    game.set_status_card(StatusCard::new(&mut ctx, sprite));
    
    match event::run(&mut ctx, &mut event_loop,&mut game) {
    	Ok(_) => println!("Clean exit"),
    	Err(e) => println!("Error ocurred: {}", e)
    }
}
