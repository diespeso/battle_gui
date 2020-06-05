mod utils;
mod sprite;
mod gui;
mod tileset_parser;
mod game;
mod movable;

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
use movable::Movable;

use tileset_parser::{*};

//NOTE: RUN CARGO TEST, DUMBASS
//NOTE: IF YOURE RUNNING CARGO TEST, CHECK THE SPECIAL
//RESOURCES FOLDER (FIX THAT, UNIFY THEM)

fn main() {
    let (mut ctx, mut event_loop) = ContextBuilder::new("game", "diespeso").build().expect("contexto no iniciado");
    let mut game = Game::new(&mut ctx);
    let path_portrait = Path::new("/assets/the_fool_melon.png");
    let path_skin = Path::new("/assets/skins.png");
    let skin = graphics::Image::new(&mut ctx, path_skin)
    	.expect("Couldnt load skin");
    let portrait = graphics::Image::new(&mut ctx, path_portrait).expect("Couldn't load portrait");
    let mut sprite_portrait = Sprite::new(portrait.clone())
    	.with_cut(&mut ctx, [0.0, 0.0, 32.0, 32.0]);
    let skin = Sprite::new(skin.clone());
    let mut status_card = StatusCard::new(&mut ctx, skin)
    	.with_status(Status::new("diespeso1", 100))
    	.with_portrait(&mut ctx, sprite_portrait);
   //status_card.move_by([32.0, 536.0].into());
   status_card.move_by([32.0, 376.0].into());
    game.set_status_card(status_card);
    
    let tileset = Tileset::new(&mut ctx, "the_fool".to_string())
   		.expect("coulndt build tileset");
    game.sprites.push(tileset.sprites["integrity"].clone());
    game.set_tileset(tileset);
    
    match event::run(&mut ctx, &mut event_loop,&mut game) {
    	Ok(_) => println!("Clean exit"),
    	Err(e) => println!("Error ocurred: {}", e)
    }
}
