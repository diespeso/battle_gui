mod utils;
mod sprite;
mod gui;
mod tileset_parser;
mod game;
mod movable;
mod animation;
mod states;

use ggez::mint::{Point2, Vector2};
use ggez::{Context, GameResult, ContextBuilder};
use ggez::event::{self, EventHandler};
use ggez::graphics::{self, Drawable};
use ggez::filesystem;
use ggez::conf::WindowMode;

use std::path::Path;
use std::rc::Rc;
use std::cell::RefCell;
use std::borrow::Borrow;
use core::time::Duration;

use sprite::{Sprite};
use gui::{Status, StatusCard};
use game::Game;
use movable::Movable;
use states::{*};
use utils::{*};

use animation::{*};

use tileset_parser::{*};

//NOTE: RUN CARGO TEST, DUMBASS
//NOTE: IF YOURE RUNNING CARGO TEST, CHECK THE SPECIAL
//RESOURCES FOLDER (FIX THAT, UNIFY THEM)

fn main() {
   /* let (mut ctx, mut event_loop) = ContextBuilder::new("game", "diespeso").build().expect("contexto no iniciado"); */
   let mut ctx = ContextBuilder::new("game", "diespeso")
   	.window_mode(WindowMode {
   		width: 512.0,
   		height: 288.0,
   		maximized: false,
   		fullscreen_type: ggez::conf::FullscreenType::Windowed,
   		borderless: false,
   		min_width: 0.0,
   		max_width: 0.0,
   		min_height: 0.0,
   		max_height: 0.0,
   		resizable: true,
   	});
   	let (mut ctx, mut event_loop) = ctx.build().expect("couldn't initialize context");
   	graphics::set_default_filter(&mut ctx, graphics::FilterMode::Nearest);
    let mut game = Game::new(&mut ctx);
    let mut back_image = graphics::Image::new(&mut ctx, 
    	Path::new("/assets/the_fool_level_img.png")
    ).expect("couldn't load the fool img");
   // back_image.set_filter(graphics::FilterMode::Nearest);
    let mut back_sprite = Sprite::new(back_image);
    back_sprite.set_scale(Vector2::<f32>::from_slice(&[1.0, 1.0]));
    game.add_sprite(back_sprite);
    let path_portrait = Path::new("/assets/the_fool_melon.png");
    let path_skin = Path::new("/assets/skins.png");
    let mut skin = graphics::Image::new(&mut ctx, path_skin)
    	.expect("Couldnt load skin");
   // skin.set_filter(graphics::FilterMode::Nearest);
   // let skin = Rc::new(RefCell::new(skin));
   let skin = Sprite::new(skin);
   let skin = Rc::new(RefCell::new(skin));
    let mut portrait = graphics::Image::new(&mut ctx, path_portrait).expect("Couldn't load portrait");
   // portrait.set_filter(graphics::FilterMode::Nearest);
    let mut sprite_portrait = Sprite::new(portrait.clone());
    
    	sprite_portrait.set_cut(&mut ctx, [0.0, 0.0, 32.0, 32.0]);
    
    let mut status_card = StatusCard::new(&mut ctx, skin.clone());
    status_card.set_status(Status::new("diespeso1", 100));
    status_card.set_portrait(&mut ctx, sprite_portrait);
   //status_card.move_by([32.0, 536.0].into());
   status_card.move_by([0.0, 224.0].into());
   let mut status_card = Rc::new(RefCell::new(status_card));
   
    game.set_status_card(status_card.clone());
    
    let tileset = Tileset::new(&mut ctx, "the_fool".to_string())
   		.expect("coulndt build tileset");
   	let tile_2 = Tileset::new(&mut ctx, "battle_gui".to_string())
   		.expect("couldn't build tileset");
    game.sprites.push(tileset.sprites["integrity"].clone());
    
    let mut gui_handler = BattleGuiHandler::new(tile_2);
    gui_handler.add_command_banner("attack".to_string());
    let mut att_cmd = gui_handler.get_command("attack".to_string());
    gui_handler.add_command_banner("conceptualize".to_string());
    gui_handler.get_command("conceptualize".to_string())
    	.borrow_mut().move_by(vector_2f(0.0, 40.0));
    
    let mut gui_animation = Animation::new(
    	gui_handler.get_command("attack".to_string())
    );
    //att_cmd.borrow_mut().move_by(vector_2f(32.0, 32.0));
    gui_animation.add_animation(Box::new(
    	TimedMove::new([-136.0, 0.0].into(),
    	 Duration::from_millis(500))
    	 )
    );
    gui_animation.add_animation(Box::new(
    	TimedIdle::new(
    	Duration::from_millis(1000))
    ));
    
    gui_animation.add_animation(Box::new(
    	TimedMove::new([136.0, 0.0].into(),
    	Duration::from_millis(500))
    ));
   	game.set_battle_gui_handler(gui_handler);

    /*
    let mv_cmd = TimedMove::new(MoveCommand::new([0.0, -128.0].into()), Duration::from_millis(300));
    
    game.add_move(Box::new(mv_cmd));
    
    let animatable = status_card.clone();
    */
    

    
    let mut animation = Animation::new(status_card.clone());
    animation.add_animation(Box::new(
    	TimedMove::new([-96.0, 0.0].into(),
    	 Duration::from_millis(300))
    	 )
    );
    animation.add_animation(Box::new(
    	TimedIdle::new(Duration::from_millis(1000))
    	)
    );
    animation.add_animation(Box::new(
    	TimedMove::new([96.0, 0.0].into(),
    	 Duration::from_millis(300))
    	 )
    );
    /*let mut other = Animation::new(status_card.clone());
    other.add_animation(
    	Box::new(
    		TimedMove::new([-128.0, 0.0].into(),
    		Duration::from_millis(500))
    	)
    );*/
    
    game.add_animation(animation);
    game.add_animation(gui_animation);
   	//game.add_animation(other);
   
    
    match event::run(&mut ctx, &mut event_loop,&mut game) {
    	Ok(_) => println!("Clean exit"),
    	Err(e) => println!("Error ocurred: {}", e)
    }
}
