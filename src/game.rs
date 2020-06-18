use core::time::Duration;

use std::rc::Rc;
use std::cell::RefCell;

use ggez::{Context, GameResult};
use ggez::event::{self, EventHandler};
use ggez::graphics::{self, Drawable};
use ggez::input::keyboard::{KeyCode, KeyMods};

use ggez::timer;

use super::gui::{Status};
use super::tileset_parser::Tileset;
use super::sprite::Sprite;
use super::animation::*;

use crate::states::battle::gui::{BattleGuiHandler, StatusCard};


pub struct Game {
    pub tileset: Option<Tileset>,
    pub sprites: Vec<Sprite>,
    pub animations: Vec<Animation>,
    pub battle_gui_handler: Option<BattleGuiHandler>,
}

impl Game {
    
    pub fn new(ctx: &mut Context) -> Self {
        Self {
            tileset: None,
            sprites: Vec::new(),
            animations: Vec::new(),
            battle_gui_handler: None,
        }
    }
    
    pub fn add_sprite(&mut self, sprite: Sprite) {
        self.sprites.push(sprite);
    }
    
    pub fn set_tileset(&mut self, tileset: Tileset) {
        self.tileset = Some(tileset);
    }
    
    pub fn add_animation(&mut self, a: Animation) {
        self.animations.push(a);
    }
    
    pub fn set_battle_gui_handler(&mut self, gui: BattleGuiHandler) {
        self.battle_gui_handler = Some(gui);
    }
}

impl EventHandler for Game {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        /*for card in &self.stat_cards {
            *card.borrow_mut().get_status_mut().hp.borrow_mut() += 1;
            card.borrow_mut().update();
        }*/

        if timer::check_update_time(ctx, 60){ //10 fps
        for animation in &mut self.animations {
                animation.run();
                if let Some(handler) = &mut self.battle_gui_handler {
                    handler.update();
                }
        }
        
        }
        Ok(())
    }
    
    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, graphics::Color::new(0.0, 0.0, 0.0, 1.0));
        
        if let Some(handler) = &self.battle_gui_handler {
            handler.draw(ctx);
        }
        graphics::present(ctx)
    }

    fn key_down_event(&mut self, ctx: &mut Context, keycode: KeyCode,
        _keymods: KeyMods,
        _repeat: bool) {
        match keycode {
            KeyCode::S => {
                println!("down");
            },
            KeyCode::W => {
                println!("up");
            },
            _ => ()
        }
    }
}
