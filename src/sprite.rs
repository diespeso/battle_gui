use ggez::graphics::{Image, DrawParam, Drawable};
use ggez::graphics::Rect;
use ggez::{Context, GameResult};
use ggez::mint::Point2;
use super::utils;

#[derive(Debug)]
pub struct Sprite {
	image: Image,
	params: DrawParam,
}

impl Sprite {
	pub fn new(image: Image) -> Self {
		Self {
			image,
			params: Default::default(),
		}
	}
	
	pub fn with_cut<R>(mut self, ctx: &mut Context, rect: R) -> Self where R: Into<Rect> {
		self.params.src = 
			utils::from_pixel_rect_to_frac(
				ctx, &self.image, &rect.into()
			);
		self
	}
	
	pub fn with_params(mut self, params: DrawParam) -> Self {
		self.params = params;
		self
	}
	
	pub fn with_position<P>(mut self, pos: P) -> Self
		where P: Into<Point2<f32>>{
		self.params.dest = pos.into();
		self
	}
	
	pub fn set_params(&mut self, params: DrawParam) {
		self.params = params;
	}
	
	pub fn dimensions(&self, ctx: &mut Context) -> Rect {
		println!("{:#?}", self.params.src.clone());
		self.params.src.clone()
	}
	
	pub fn image(&self) -> &Image {
		&self.image
	}
	
	pub fn drawable(&self) -> impl Drawable {
		self.image.clone()
	}
	
	
	pub fn draw(&self, ctx: &mut Context) -> GameResult {
		self.image.draw(ctx, self.params)
	}
	
	
}
