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

use sprite::{Sprite};
use gui::{Status, StatusCard};
use game::Game;

use tileset_parser::{*};

//NOTE: RUN CARGO TEST, DUMBASS
//NOTE: IF YOURE RUNNING CARGO TEST, CHECK THE SPECIAL
//RESOURCES FOLDER (FIX THAT, UNIFY THEM)

fn main() {
    let (mut ctx, mut event_loop) = ContextBuilder::new("game", "diespeso").build().expect("contexto no iniciado");
    let mut game = Game::new(&mut ctx);
    let path = Path::new("/assets/battle_gui.png");
    let image = graphics::Image::new(&mut ctx, path).expect("No se puedo cargar imagen");
    let mut sprite = Sprite::new(image.clone())
    	.with_cut(&mut ctx, [0.0, 0.0, 64.0, 64.0]);
    let mut status_card = StatusCard::new(&mut ctx, sprite)
    	.with_status(Status::new("diespeso1", 100))
    	.with_portrait(&mut ctx, Sprite::new(image.clone()));
   
    game.set_status_card(status_card);
    
    let tileset = Tileset::new(&mut ctx, "uno".to_string());
    println!("{:#?}", tileset);
    
    game.set_tileset(tileset.expect("no tileset made"));
    
    match event::run(&mut ctx, &mut event_loop,&mut game) {
    	Ok(_) => println!("Clean exit"),
    	Err(e) => println!("Error ocurred: {}", e)
    }
}
