use ggez::{Context, GameResult};
use ggez::event::{self, EventHandler};
use ggez::graphics::{self, Drawable};

use super::gui::{Status, StatusCard};
use super::tileset_parser::Tileset;


pub struct Game {
	pub stat_card: Option<StatusCard>,
	pub tileset: Option<Tileset>
}

impl Game {
	
	pub fn new(ctx: &mut Context) -> Self {
		Self {
			stat_card: None,
			tileset: None,
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
		*(self.stat_card.as_ref().expect("no stat card").status.as_ref().expect("no stats in statcard").hp.borrow_mut()) += 1;
		Ok(())
	}
	
	fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
		graphics::clear(ctx, graphics::Color::new(0.1, 0.1, 0.1, 1.0));
		if let Some(tileset) = &self.tileset {
			tileset.draw(ctx);
		}
		self.stat_card.as_ref().expect("status card not initialized").draw(ctx, Default::default());
		graphics::present(ctx)
	}
}
