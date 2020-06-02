use ggez::{Context, ContextBuilder, GameResult};
use ggez::event::{self, EventHandler};
use ggez::graphics::{self, Drawable};

use super::gui::{Status, StatusCard};


pub struct Game {
	pub stat_card: Option<StatusCard>,
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
		*(self.stat_card.as_ref().expect("no stat card").status.as_ref().expect("no stats in statcard").hp.borrow_mut()) += 1;
		Ok(())
	}
	
	fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
		graphics::clear(ctx, graphics::Color::new(0.1, 0.1, 0.1, 1.0));
		self.stat_card.as_ref().expect("status card not initialized").draw(ctx, Default::default());
		graphics::present(ctx)
	}
}
