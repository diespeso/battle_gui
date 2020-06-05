use ggez::{Context, GameResult};
use ggez::event::{self, EventHandler};
use ggez::graphics::{self, Drawable};

use super::gui::{Status, StatusCard};
use super::tileset_parser::Tileset;
use super::sprite::Sprite;


pub struct Game {
	pub stat_card: Option<StatusCard>,
	pub tileset: Option<Tileset>,
	pub sprites: Vec<Sprite>,
}

impl Game {
	
	pub fn new(ctx: &mut Context) -> Self {
		Self {
			stat_card: None,
			tileset: None,
			sprites: Vec::new(),
		}
	}
	
	pub fn set_status_card(&mut self, stat_card: StatusCard) {
		self.stat_card = Some(stat_card);
	}
	
	pub fn set_tileset(&mut self, tileset: Tileset) {
		self.tileset = Some(tileset);
	}
}

impl EventHandler for Game {
	fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
		let mut card = self.stat_card.as_mut()
			.expect("no status card");
		/* *(self.stat_card.as_ref().expect("no stat card").status.as_ref().expect("no stats in statcard").hp.borrow_mut()) += 1;
		self.stat_card.expect("no stats card)*/
		//card.status.as_mut().expect("no status at all")
		*card.get_status_mut().hp.borrow_mut() += 1;
		println!("{:#?}", *card.get_status_mut().hp);
		card.update();
		Ok(())
	}
	
	fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
		graphics::clear(ctx, graphics::Color::new(0.1, 0.1, 0.1, 1.0));
		if let Some(tileset) = &self.tileset {
			tileset.draw(ctx);
		}
		self.stat_card.as_ref().expect("status card not initialized").draw(ctx, Default::default());
		
		for sprite in &self.sprites {
			sprite.draw(ctx);
		}
		graphics::present(ctx)
	}
}
