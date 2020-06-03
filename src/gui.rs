use std::cell::RefCell;
use std::rc::Rc;

use super::sprite::Sprite;
use ggez::graphics::{self, DrawParam, Rect, BlendMode,
	Text};
use ggez::error::GameResult;
use ggez::Context;

use super::utils::from_pixel_rect_to_frac;
use super::utils::add_point2f;

use ggez::mint::Point2;

#[derive(Debug)]
pub struct Status {
	pub name: &'static str,
	pub hp: Rc<RefCell<i32>>,
}

impl Status {
	pub fn new(name: &'static str, hp: i32) -> Self {
		Self {
			name,
			hp: Rc::new(RefCell::new(hp)),
		}
	}
	
	pub fn hp(&self) -> Rc<RefCell<i32>> {
		self.hp.clone()
	}
}

pub struct StatusCard {
	pub skin: Sprite,
	pub status: Option<Status>,
	pub name_text: Option<Text>,
	pub hp_text: Option<Text>,
	pub portrait: Option<Sprite>,
}

impl StatusCard {
	pub fn new(ctx: &mut Context, skin: Sprite) -> Self {
		let mut skin = skin;
		skin = skin.with_cut(ctx, [0.0, 0.0, 160.0, 64.0]);
		Self {
			skin,
			status: None,
			name_text: None,
			hp_text: None,
			portrait: None,
		}
	}
	
	pub fn with_status(mut self, status: Status) -> Self {
		self.status = Some(status);
		self.name_text = Some(Text::new(self.status.as_ref().unwrap().name));
		self.hp_text = Some(Text::new(self.status.as_ref().unwrap().hp.borrow().to_string()));
		//first get the status as reference, unwrap it, its ok
		//then take the hp rc and borrow it, convert to string
		self
	}
	
	pub fn with_portrait(mut self, ctx: &mut Context, mut sprite: Sprite) -> Self {
		sprite = sprite.with_cut(ctx, [320.0, 0.0, 64.0, 64.0])
			.with_position([96.0, 0.0]);
		self.portrait = Some(sprite);
		
		self
	}
	
	
	pub fn get_status(&self) -> Option<&Status> {
		self.status.as_ref()
	}
}

impl graphics::Drawable for StatusCard {
	fn draw(&self, ctx: &mut Context, param: DrawParam) -> GameResult<()> {
		//background
		self.skin.draw(ctx)?;
		let p = param.clone();
		self.name_text.as_ref().expect("no status").draw(ctx, param.dest(
			add_point2f(
				p.dest.clone(),
				Point2::<f32>::from_slice(&[10.0, 8.0])
			)
		))?;
		Text::new(self.status.as_ref().expect("no status").hp.borrow().to_string()).draw(ctx,
		param.dest(
			add_point2f(
				p.dest.clone(),
				Point2::<f32>::from_slice(&[10.0, 40.0])
			)
		))?;
		
		self.portrait.as_ref().expect("no portrait set").draw(ctx);
		
		Ok(())
	}
	
	fn dimensions(&self, ctx: &mut Context) -> Option<Rect> {
		Some(Rect::default())
	}
	
	fn set_blend_mode(&mut self, mode: Option<BlendMode>) {
		//fixme
	}
	
	fn blend_mode(&self) -> Option<BlendMode> {
		self.skin.drawable().blend_mode()
	}
}
