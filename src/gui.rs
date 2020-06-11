use std::cell::RefCell;
use std::rc::Rc;

use super::sprite::Sprite;
use ggez::graphics::{self, DrawParam, Rect, BlendMode,
	Text};
use ggez::error::GameResult;
use ggez::Context;

use ggez::mint::Vector2;

use super::utils::{from_pixel_rect_to_frac, add_point2f};
use super::movable::Movable;
use super::animation::Animatable;

use ggez::mint::Point2;


type MovableText = (Text, DrawParam);

static name_offset: Point2<f32> = Point2::<f32>{
	x: 32.0,
	y: 10.0,
};

static hp_offset: Point2<f32> = Point2::<f32> {
	x: 10.0,
	y: 40.0,
};

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
	
	pub fn set_hp(&mut self, num: i32) {
		*self.hp.borrow_mut() = num;
	}
}

#[derive(Debug)]
pub struct StatusCard {
	pub param: DrawParam,
	pub skin: Rc<RefCell<Sprite>>,
	pub status: Option<Status>,
	pub name_text: Option<MovableText>,
	pub hp_text: Option<MovableText>,
	pub portrait: Option<Sprite>,
}

impl StatusCard {
	pub fn new(ctx: &mut Context, skin: Rc<RefCell<Sprite>>) -> Self {
		skin.clone().borrow_mut().set_cut(ctx, [0.0, 0.0, 128.0, 64.0]);
		//default skin dimensions
		Self {
			param: DrawParam::default(),
			skin: skin.clone(),
			status: None,
			name_text: None,
			hp_text: None,
			portrait: None,
		}
	}
	
	pub fn set_status(&mut self, status: Status) {
		self.status = Some(status);
		/*self.name_text = Some(Text::new(self.status.as_ref().unwrap().name));
		
		self.hp_text = Some(Text::new(self.status.as_ref().unwrap().hp.borrow().to_string()));*/
		self.name_text =  Some(
			(Text::new(self.status.as_ref().unwrap().name),
			DrawParam::default().dest(name_offset).scale(Vector2::<f32>::from_slice(&[0.9, 0.9]))
			)
		);
		//self.name_text.1 = self.name_text.1.dest([10.0, 8.0]);
		self.hp_text = Some(
			(Text::new(self.status.as_ref().unwrap().hp.borrow().to_string()),
			DrawParam::default().dest(hp_offset)
			)
		);
		//self.hp_text.1 = self.hp_text.1.dest([10.0, 40.0]);
		//first get the status as reference, unwrap it, its ok
		//then take the hp rc and borrow it, convert to string
	}
	
	pub fn set_portrait(&mut self, ctx: &mut Context, mut sprite: Sprite){
		sprite.set_cut(ctx, [320.0, 0.0, 32.0, 32.0]);
		sprite.set_position(self.param.dest.clone());
		self.portrait = Some(sprite);
	}
	
	
	pub fn get_status(&self) -> &Status {
		if let Some(result) = &self.status {
			return result;
		} else {
			panic!("no status in status card");
		}
	}
	
	pub fn get_status_mut(&mut self) -> &mut Status {
		if let Some(result) = &mut self.status {
			return result;
		} else {
			panic!("no status in status card");
		}
	}
	
	pub fn update(&mut self) {
		/*let mut hp_texto = &self.hp_text.as_mut().expect("no status set").0;
		hp_texto = &Text::new(self.get_status().hp.borrow().to_string());*/
		self.hp_text.as_mut().expect("couldnt update gui").0 = Text::new(self.get_status().hp().borrow().to_string());
		self.debug_position();
	}
}

impl graphics::Drawable for StatusCard {
	fn draw(&self, ctx: &mut Context, param: DrawParam) -> GameResult<()> {
		//background
		let p = param.clone(); //ignore param for now
		let ref_name = self.name_text.as_ref().expect("no status set");
		//ref_name.0.draw(ctx, ref_name.1.clone())?;
		let ref_hp = self.hp_text.as_ref().expect("no status set");
		ref_hp.0.draw(ctx, ref_hp.1.clone())?;
		graphics::queue_text(ctx, &ref_name.0,
		Point2::<f32>::from_slice(&[0.0, 0.0]),
		None,
		);
		graphics::draw_queued_text(ctx, ref_name.1.clone(),
			Default::default(), graphics::FilterMode::Nearest);
		self.portrait.as_ref().expect("no portrait set")
			.draw(ctx);
		self.skin.borrow().draw(ctx)?;
		/*self.name_text.as_ref().expect("no status").draw(ctx, param.dest(
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
		
		*/
		Ok(())
	}
	
	fn dimensions(&self, ctx: &mut Context) -> Option<Rect> {
		Some(Rect::default())
	}
	
	fn set_blend_mode(&mut self, mode: Option<BlendMode>) {
		//fixme
	}
	
	fn blend_mode(&self) -> Option<BlendMode> {
		self.skin.borrow().drawable().blend_mode()
	}
}

impl Movable for StatusCard {
	fn move_by(&mut self, vector: Point2<f32>) {
		self.skin.borrow_mut().move_by(vector.clone());
		let mut name = self.name_text.as_mut().expect("no status");
		name.1 = name.1.dest(add_point2f(name.1.dest.clone(), 
			vector.clone()));
		let mut hp = self.hp_text.as_mut().expect("no status");
		hp.1 = hp.1.dest(add_point2f(hp.1.dest.clone(),
			vector.clone()));
		self.portrait.as_mut().expect("no portrait set")
			.move_by(vector);
	}
	
	fn move_to(&mut self, position: Point2<f32>) ->
	Point2<f32> {
		unimplemented!();
	}
	
	fn debug_position(&self) {
		//println!("{:#?}", self.skin.borrow().params().dest);
	}
}

impl Animatable for StatusCard {

}
