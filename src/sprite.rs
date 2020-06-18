use ggez::graphics::{Image, DrawParam, Drawable, Rect, Color};
use ggez::{Context, GameResult};
use ggez::mint::{Point2, Vector2};

use super::utils;
use super::movable::Movable;

use super::animation::Animatable;
use super::animation::Colorable;

#[derive(Debug, Clone)]
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
    
    pub fn set_cut<R>(&mut self, ctx: &mut Context, rect: R) where R: Into<Rect> {
        self.params.src = 
            utils::from_pixel_rect_to_frac(
                ctx, &self.image, &rect.into()
            );
    }
    
    pub fn with_params(mut self, params: DrawParam) -> Self {
        self.params = params;
        self
    }
    
    pub fn set_position<P>(&mut self, pos: P)
        where P: Into<Point2<f32>>{
        self.params.dest = pos.into();
    }
    
    pub fn set_params(&mut self, params: DrawParam) {
        self.params = params;
    }
    
    pub fn dimensions(&self, ctx: &mut Context) -> Rect {
        self.params.src.clone()
    }
    
    pub fn set_scale(&mut self, scale: Vector2<f32>) {
        self.params.scale = scale;
    }
    
    pub fn image(&self) -> &Image {
        &self.image
    }
    
    pub fn params(&self) -> DrawParam {
        self.params.clone()
    }
    
    pub fn drawable(&self) -> impl Drawable {
        self.image.clone()
    }

    pub fn set_color(&mut self, color: Color) {
        self.params.color = color;
    }
    
    
    pub fn draw(&self, ctx: &mut Context) -> GameResult {
        self.image.draw(ctx, self.params)
    }
}

impl Movable for Sprite {
    fn move_by(&mut self, vector: Point2<f32>) {
        self.params.dest = utils::add_point2f(
        self.params.dest.clone(), vector);
    }
    
    fn move_to(&mut self, position: Point2<f32>) -> Point2<f32> {
        let mut r = utils::sub_point2f(position.clone(),
            self.params.dest.clone());
        self.params.dest = position;
        r
    }
}

impl Colorable for Sprite {
    fn adjust_color(&mut self, adjustment: [f32; 4]) {
        self.params.color = utils::adjust_color(self.params.color, adjustment);
    }
}

impl Animatable for Sprite {
    //dummy
}

