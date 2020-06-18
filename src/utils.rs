use ggez::mint::Point2;
use ggez::Context;
use std::vec::Vec;
use ggez::graphics::{Rect, Drawable, Image, Color};
use super::sprite::Sprite;


pub fn get_color_adjustment(color_one: Color, color_two: Color) -> [f32; 4] {
    let one: [f32; 4] = color_one.into();
    let two: [f32; 4] = color_two.into();
    let mut result: [f32; 4] = [0.0, 0.0, 0.0, 0.0];

    for i in 0..result.len() {
        result[i] = one[i] - two[i];
    }

    result
}

pub fn divide_color_adjustment(adjustment: [f32; 4], number: f32) -> [f32; 4] {
    let mut result: [f32; 4] = [0.0, 0.0, 0.0, 0.0];

    for i in 0 .. result.len() {
        result[i] = adjustment[i] / number;
    }

    result
}

pub fn multiply_color_adjustment(adjustment: [f32; 4], number: f32) -> [f32; 4] {
    let mut result = adjustment.clone();
    for i in 0 .. result.len() {
        result[i] = result[i] * number;
    }

    result
}

pub fn adjust_color(color_one: Color, adjustment: [f32; 4]) -> Color {
    let mut color: [f32; 4] = color_one.into();
    for i in 0 .. color.len() {
        color[i] -= adjustment[i];
    }

    color.into()
}

pub fn add_point2f(p1: Point2<f32>, p2: Point2<f32>) -> Point2::<f32> {
    Point2::<f32> {
        x: p1.x + p2.x,
        y: p1.y + p2.y,
    }
}

pub fn vector_2f(p1: f32, p2: f32) -> Point2<f32> {
    Point2::<f32>{x: p1, y: p2}
}

pub fn sub_point2f(p1: Point2<f32>, p2: Point2<f32>) -> Point2::<f32> {
    Point2::<f32> {
        x: p1.x - p2.x,
        y: p1.y - p2.y,
    }
}

pub fn from_str_to_point2f(pos: String) -> Point2<f32> {
    let mut result: Vec<String> = pos.split(' ').map(|x| x.to_string()).collect();
    let mut result: Point2<f32> = Point2::<f32>::from_slice(
        &[result[0].parse::<f32>().expect("failed to make point2f")
        , result[1].parse::<f32>().expect("failed to make point2f")]
    );
    
    return result;
}

pub fn clean_empty(source: &mut Vec<(String, Vec<String>)>) {
    for i in 0..source.len() {
        let mut index = 0;
        if source[i].0 == "" {
            index = i;
        }
        source.remove(index);
    }
}

pub fn from_pixel_rect_to_frac(ctx: &mut Context, image: &Image, rect: &Rect) 
    -> Rect {
    let total_size = &image.dimensions();
    let mut result = Rect::default();
    
    result.x = rect.x / total_size.w;
    if result.x.is_nan() || result.x.is_infinite() {
        result.x = 0.0;
    }
    result.y = rect.y / total_size.h;
    if result.y.is_nan() || result.y.is_infinite() {
        result.y = 0.0;
    }
    result.w = rect.w / total_size.w;
    result.h = rect.h / total_size.h;
    result
}
