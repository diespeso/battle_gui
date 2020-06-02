mod utils;
mod sprite;
mod gui;
mod tileset_parser;
mod game;

use ggez::mint::Point2;
use ggez::{Context, GameResult, ContextBuilder};
use ggez::event::{self, EventHandler};
use ggez::graphics::{self, Drawable};
use ggez::filesystem;

use std::path::Path;
use std::rc::Rc;
use std::cell::RefCell;

use sprite::{Sprite, SpriteData};
use gui::StatusCard;
use game::Game;

//NOTE: RUN CARGO TEST, DUMBASS
//NOTE: IF YOURE RUNNING CARGO TEST, CHECK THE SPECIAL
//RESOURCES FOLDER (FIX THAT, UNIFY THEM)

fn main() {
    let (mut ctx, mut event_loop) = ContextBuilder::new("game", "diespeso").build().expect("contexto no iniciado");
    let mut game = Game::new(&mut ctx);
    let image = graphics::Image::new(&mut ctx, Path::new("/assets/battle_gui.png")).expect("No se puedo cargar imagen");
    let image = Rc::new(RefCell::new(image));
    let mut sprite_data = SpriteData::new(image.clone());
    let mut sprite = Sprite::from_data(sprite_data);
    let mut status_card = StatusCard::new(&mut ctx, sprite);
    status_card.skin.move_by(Point2::<f32>::from_slice(&[100.0, 50.0]));
    game.set_status_card(status_card);
    
    match event::run(&mut ctx, &mut event_loop,&mut game) {
    	Ok(_) => println!("Clean exit"),
    	Err(e) => println!("Error ocurred: {}", e)
    }
}
