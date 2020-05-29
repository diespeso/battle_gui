use std::rc::Rc;
use std::cell::RefCell;
use ggez::{Context, GameResult};
use ggez::mint::Point2;
use ggez::graphics::{self, Drawable, DrawParam};

pub struct SpriteData {
	pub drawable: Rc<RefCell<dyn Drawable>>,
}

impl SpriteData {
	pub fn new(drawable: Rc<RefCell<dyn Drawable>>) -> Self {
		SpriteData{drawable: drawable}
	}
	
	pub fn drawable(&self) -> Rc<RefCell<dyn Drawable>> {
		self.drawable.clone()
	}
}

pub struct Sprite {
	data: SpriteData,
	param: DrawParam,
}

impl Sprite {
	pub fn new(data: SpriteData, param: DrawParam) -> Self {
		Sprite {
			data: data,
			param: param,
		}
	}
	
	pub fn drawable(&self) -> Rc<RefCell<dyn Drawable>> {
		self.data.drawable()
	}
	
	pub fn from_data(data: SpriteData) -> Self {
		Self::new(data, Default::default())
	}
	
	pub fn from_drawable(drawable: Rc<RefCell<dyn Drawable>>)
		-> Self {
		Self {
			data: SpriteData::new(drawable),
			param: Default::default(),
		}
	}
	
	pub fn draw(&self, ctx: &mut Context) {
		let data = self.data.drawable.borrow();
		data.draw(ctx, self.param);
	}
	
	pub fn pos_mut(&mut self) -> &mut Point2<f32> {
		&mut self.param.dest
	}
}
	





