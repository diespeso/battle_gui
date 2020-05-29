mod utils;
mod sprite;

use ggez::mint::Point2;
use ggez::{Context, GameResult, ContextBuilder};
use ggez::event::{self, EventHandler};
use ggez::graphics;
use ggez::filesystem;

use std::path::Path;
use std::rc::Rc;
use std::cell::RefCell;

use sprite::{Sprite, SpriteData};

struct Game {
	pub sprite: Option<Sprite>,
	
}

impl Game {
	pub fn new(_ctx: &mut Context) -> Game {
		Game {
			sprite: None,
		}	
	}
	
	pub fn set_sprite(&mut self, sprite: Sprite) -> &mut Self {
		self.sprite = Some(sprite);
		self
	}
}

impl EventHandler for Game {
	fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
		Ok(())
	}
	
	fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
		graphics::clear(ctx, graphics::BLACK);
		self.sprite.as_mut().expect("no sprite").draw(ctx);
		graphics::present(ctx)
	}
}

fn main() {
    let (mut ctx, mut event_loop) = ContextBuilder::new("game", "diespeso").build().expect("contexto no iniciado");
    let mut game = Game::new(&mut ctx);
    let image = graphics::Image::new(&mut ctx, Path::new("/assets/madera.png")).expect("No se puedo cargar imagen");
    let image = Rc::new(RefCell::new(image));
    let sprite_data = SpriteData::new(image);
    game.set_sprite(Sprite::from_data(sprite_data));
    
    
    match event::run(&mut ctx, &mut event_loop,&mut game) {
    	Ok(_) => println!("Clean exit"),
    	Err(e) => println!("Error ocurred: {}", e)
    }
}
