use super::sprite::Sprite;
use ggez::graphics::{self, DrawParam, Rect, BlendMode};
use ggez::error::GameResult;
use ggez::Context;

use super::utils::from_pixel_rect_to_frac;

pub struct StatusCard {
	skin: Sprite,
	
}

impl StatusCard {
	pub fn new(ctx: &mut Context, skin: Sprite) -> Self {
		let mut skin = skin;
		let rect = from_pixel_rect_to_frac(ctx, &skin,
		&Rect::new(0.0, 0.0, 160.0, 64.0));
		println!("{:#?}", rect);
		skin.set_draw_param(DrawParam::default().src(
			rect));
		Self {
			skin,
		}
	}
}

impl graphics::Drawable for StatusCard {
	fn draw(&self, ctx: &mut Context, param: DrawParam) -> GameResult {
		self.skin.draw(ctx)
	
	}
	
	fn dimensions(&self, ctx: &mut Context) -> Option<Rect> {
		Some(Rect::default())
	}
	
	fn set_blend_mode(&mut self, mode: Option<BlendMode>) {
		//fixme
	}
	
	fn blend_mode(&self) -> Option<BlendMode> {
		self.skin.drawable().borrow().blend_mode()
	}
}
