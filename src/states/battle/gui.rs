use core::time::Duration;
use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;

use crate::tileset_parser::Tileset;
use crate::utils::{*};
use crate::movable::Movable;
use crate::animation::{*};
use crate::animation::{*};
use crate::sprite::Sprite;
use super::SpritePointer;
use crate::gui::{*};

use ggez::graphics::{self, BlendMode, Rect, Font};
use ggez::GameResult;
use ggez::graphics::Scale;

use ggez::Context;
use ggez::graphics::{Drawable, Text, DrawParam, Image,
    spritebatch::{SpriteBatch, SpriteIdx}, Color};
use ggez::mint::{Point2, Vector2};


static name_offset: Point2<f32> = Point2::<f32>{
    x: 32.0,
    y: 10.0,
};

static hp_offset: Point2<f32> = Point2::<f32> {
    x: 10.0,
    y: 40.0,
};

type MovableText = (Text, DrawParam);

#[derive(Debug)]
pub struct BattleGuiHandler {
    tileset: Tileset,
    commands: HashMap<String, Rc<RefCell<GuiCommandBanner>>>,
    allies_status: [Option<StatusCard>; 4],
}

impl BattleGuiHandler {
    pub fn new(ctx: &mut Context, tileset: Tileset) -> Self {
        let mut result = Self {
            tileset,
            commands: HashMap::new(),
            allies_status: [None, None, None, None], 
        };

        result.test_initialize(ctx);
        result
    }

    pub fn test_initialize(&mut self, ctx: &mut Context) {
        self.allies_status[0] = Some(
            StatusCard::from_tileset(ctx, Rc::new(
                RefCell::new(Status::new("diespeso1", 100))
                ), &self.tileset)
        );
        self.allies_status[0].as_mut().unwrap().set_portrait(ctx,
            self.tileset.get("por_devil".to_string()));
        self.allies_status[0].as_mut().unwrap()
            .move_by(vector_2f(6.4, 256.0));
    }
    
    pub fn add_command_banner(&mut self, name: String) {
        self.commands.insert(name, Rc::new(
            RefCell::new(
            GuiCommandBanner::new(&self.tileset
            )
            )
        ));
    }
    
    pub fn get_command(&self, name: String) -> Rc<RefCell<GuiCommandBanner>> {
        self.commands[&name].clone()
    }

    pub fn update(&mut self) {
        for statuses in &mut self.allies_status {
            if let Some(status) = statuses {
                status.update();
            }
        }
    }
    
    pub fn draw(&self, ctx: &mut Context) {
        for command in self.commands.values() {
            command.borrow().draw(ctx);
        }
        for status in &self.allies_status {
            if let Some(status_card) = &status {
                status_card.draw(ctx, DrawParam::default());
            }
        }
    }
}


pub enum CommandBannerType {
    ATTACK (AttackType),
    FORM,
    CONCEPTUALIZE,
    CONSUME,
    EXIST,  
}

pub enum AttackType {
    SWORD,
    GUN,
    BOW,
    CANNON,
    HAMMER,
    STAFF,
    UNHOLY_BOW,
    BLASTER,
    AXE,
    INSTRUMENT,
    BIO_BLADE,
    NULL,
}

pub static PORTRAIT_BOX: &'static str = "skin_command_off";
pub static PORTRAIT: &'static str = "por_sword";
pub static TEXT_BOX: &'static str = "skin_command_on";

#[derive(Debug)]
pub struct GuiCommandBanner {
    position: Point2<f32>,
    pieces: HashMap<String, SpritePointer>,
    /*portrait_box: SpritePointer,
    portrait: SpritePointer,
    text_box: SpritePointer,
    text: Option<Text>,*/
}

impl GuiCommandBanner {
    
    pub fn new(tileset: &Tileset) -> Self {
        
        let mut result = Self {
            position: vector_2f(0.0, 0.0),
            pieces: HashMap::new(),
        };
        
        //TODO: BAD CODE DUPLICATION
        result.pieces.insert(
            PORTRAIT_BOX.to_string(),
            Rc::new(RefCell::new(tileset.get(PORTRAIT_BOX.to_string()))
        ));
        
        result.pieces.insert(
            PORTRAIT.to_string(),
            Rc::new(RefCell::new(
                tileset.get(PORTRAIT.to_string())
            ))
        );
        
        result.pieces.insert(
            TEXT_BOX.to_string(),
            Rc::new(RefCell::new(
                tileset.get(TEXT_BOX.to_string())
            ))
        );
        
        result.pieces[TEXT_BOX].borrow_mut().move_by(vector_2f(32.0, 0.0));
        
        //result.move_by(vector_2f(8.0, 8.0));
        result
    }
    
    pub fn draw(&self, ctx: &mut Context) {
        /*self.pieces[PORTRAIT_BOX].borrow().draw(ctx);
        self.pieces[PORTRAIT].borrow().draw(ctx);
        self.pieces[TEXT_BOX].borrow().draw(ctx);*/
        
        for piece in self.pieces.values() {
            piece.borrow().draw(ctx);
        }
    }
    
    pub fn get_piece(&self, name: String) -> Rc<RefCell<dyn Animatable>> {
        self.pieces[&name].clone()
    }    
}

impl Animatable for GuiCommandBanner {

}


impl Colorable for GuiCommandBanner {
    fn adjust_color(&mut self, adjustment: [f32; 4]) {
        for piece in &mut self.pieces.values() {
            piece.borrow_mut().adjust_color(adjustment);
        }
    }

    fn set_color(&mut self, color: Color) {
        for piece in &mut self.pieces.values() {
            piece.borrow_mut().set_color(color);
        }
    }
}

impl Movable for GuiCommandBanner {
    fn move_by(&mut self, vector: Point2<f32>) {
        /*self.pieces[PORTRAIT_BOX].borrow_mut().move_by(vector.clone());
        self.pieces[PORTRAIT].borrow_mut().move_by(vector);
        self.pieces[TEXT_BOX].borrow_mut().move_by(vector);*/
        for piece in self.pieces.values() {
            piece.borrow_mut().move_by(vector);
        }
    }
}


#[derive(Debug)]
pub struct StatusCard {
    pub param: DrawParam,
    pub skin: Sprite,
    pub status: Rc<RefCell<Status>>,
    pub name_text: Option<MovableText>,
    pub hp_text: Option<MovableText>,
    pub portrait: Option<Sprite>,
    pub font: Option<Rc<RefCell<Font>>>,
}

impl StatusCard {
    pub fn new(ctx: &mut Context, status: Rc<RefCell<Status>>, mut skin: Sprite) -> Self {
        skin.set_cut(ctx, [0.0, 0.0, 128.0, 64.0]);
        //default skin dimensions
        //BROKEN: DONT USE FIXME
        Self {
            param: DrawParam::default(),
            skin: skin,
            status,
            name_text: None,
            hp_text: None,
            portrait: None,
            font: None,
        }
    }

    pub fn from_tileset(ctx: &mut Context, status: Rc<RefCell<Status>>, tileset: &Tileset) -> Self {
        //if this is used, the tileset must have the standard statuscard
        //.tc format. TODO: Define that standard lol.
        let mut result = Self {
            param: DrawParam::default(),
            skin: tileset.get("plain_skin".to_string()),
            status: status.clone(),
            name_text: None,
            hp_text: None,
            portrait: None,
            font: None,
        };
        //uses the same status cause the logic is in set
        //im kinda lazy
        result.set_status(status.clone());
        result
    }
    
    pub fn set_font(&mut self, font: Rc<RefCell<Font>>) {
        self.font = Some(font);
    }
    
    pub fn set_status(&mut self, status: Rc<RefCell<Status>>) {
        self.status = status;
        /*self.name_text =  Some(
            (Text::new(self.status.as_ref().unwrap().name),
            DrawParam::default().dest(name_offset)
            )
        );
        self.hp_text = Some(
            (Text::new(self.status.as_ref().unwrap().hp.borrow().to_string()),
            DrawParam::default().dest(hp_offset)
            )
        );*/

        self.name_text = Some(
            (Text::new((*RefCell::borrow(&self.status)).name),
            DrawParam::default().dest(name_offset))
        );

        self.hp_text = Some(
            (Text::new((*RefCell::borrow(&self.status)).hp.borrow().to_string()),
            DrawParam::default().dest(hp_offset))
        );
        
        if let Some(font) = &self.font {
            self.name_text.as_mut().unwrap().0.set_font(
                *font.borrow(),
                Scale::uniform(6.0),
             );
            
            
        }
    }
    
    pub fn set_portrait(&mut self, ctx: &mut Context, mut sprite: Sprite){
        sprite.set_cut(ctx, [320.0, 0.0, 32.0, 32.0]);
        sprite.set_position(self.param.dest.clone());
        self.portrait = Some(sprite);
    }
    
    
    pub fn get_status(&self) -> Status {
        (*self.status.borrow()).clone()
    }
    
    pub fn get_status_mut(&mut self) -> Rc<RefCell<Status>> {
        self.status.clone()
    }
    
    pub fn update(&mut self) {
        self.status.borrow_mut().test_update();
        //make this better, should change only when it has to
        self.hp_text.as_mut().expect("couldnt update gui").0 = Text::new(self.get_status().hp().borrow().to_string());
        
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
        self.skin.draw(ctx)?;
        Ok(())
    }
    
    fn dimensions(&self, ctx: &mut Context) -> Option<Rect> {
        Some(Rect::default())
    }
    
    fn set_blend_mode(&mut self, mode: Option<BlendMode>) {
        //fixme
    }
    
    fn blend_mode(&self) -> Option<BlendMode> {
        self.skin.drawable().blend_mode()
    }
}

impl Movable for StatusCard {
    fn move_by(&mut self, vector: Point2<f32>) {
        self.skin.move_by(vector.clone());
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

impl Colorable for StatusCard {
    fn adjust_color(&mut self, adjustment: [f32; 4]) {
        self.skin.adjust_color(adjustment);
    }
}

impl Animatable for StatusCard {

}



