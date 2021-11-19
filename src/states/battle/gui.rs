use core::time::Duration;
use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;
use std::path::Path;

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
use ggez::filesystem;

use ggez::Context;
use ggez::graphics::{Drawable, Text, DrawParam, Image,
    spritebatch::{SpriteBatch, SpriteIdx}, Color};
use ggez::mint::{Point2, Vector2};


static name_offset: Point2<f32> = Point2::<f32>{
    x: 57.0,
    y: 0.0,
};

static hp_offset: Point2<f32> = Point2::<f32> {
    x: 10.0,
    y: 40.0,
};

type MovableText = (Text, DrawParam);

#[derive(Debug)]
pub struct BattleGUIData {
    fonts: HashMap<String, Rc<RefCell<Font>>>,
}

impl BattleGUIData {
    pub fn new(ctx: &mut Context) -> Self {
        let mut result = Self {
            fonts: HashMap::new(),
        };
        result.fonts.insert("racing".to_string(), 
            Rc::new(
                RefCell::new(
                    Font::new(ctx, Path::new("/RacingSansOne-Regular.ttf"))
            .expect("couldn't load font")
                )));

        result
    }

    pub fn get_font(&self, s: String) -> Rc<RefCell<Font>> {
        self.fonts[&s].clone()
    }
}

#[derive(Debug)]
pub struct BattleGuiHandler {
    tileset: Tileset,
    commands: HashMap<String, Rc<RefCell<GuiCommandBanner>>>,
    allies_status: [Option<StatusCard>; 4],
    battle_gui_data: BattleGUIData,
    background: Image,
}

impl BattleGuiHandler {
    pub fn new(ctx: &mut Context, tileset: Tileset) -> Self {
        let mut result = Self {
            tileset,
            commands: HashMap::new(),
            allies_status: [None, None, None, None],
            battle_gui_data: BattleGUIData::new(ctx),
            background: Image::new(ctx, Path::new("/assets/background_test.png")).expect("no background could be loaded"),
        };

        result.test_initialize(ctx);
        result
    }

    pub fn test_initialize(&mut self, ctx: &mut Context) {
        self.allies_status[0] = Some(
            StatusCard::from_tileset(ctx, Rc::new(
                RefCell::new(Status::new("Nordmend", 100))
                ), &self.tileset)
        );

        self.allies_status[1] = Some(
            StatusCard::from_tileset(ctx, Rc::new(
                RefCell::new(Status::new("Drandurl", 100))
                ), &self.tileset)
        );

        self.allies_status[2] = Some(
            StatusCard::from_tileset(ctx, Rc::new(
                RefCell::new(Status::new("Tsydysma", 100))
                ), &self.tileset)
        );

        self.allies_status[3] = Some(
            StatusCard::from_tileset(ctx, Rc::new(
                RefCell::new(Status::new("Znoromah", 100))
                ), &self.tileset)
        );

        self.allies_status[0].as_mut().unwrap().set_portrait(ctx,
            self.tileset.get("por_devil"));
        self.allies_status[0].as_mut().unwrap().set_components(
            "background_color",
            self.tileset.get("background_fluent")
        );
        self.allies_status[0].as_mut().unwrap()
            .move_by(vector_2f(0.0, 416.0));


        self.allies_status[1].as_mut().unwrap().set_portrait(ctx,
            self.tileset.get("por_devil"));
        self.allies_status[1].as_mut().unwrap().set_components(
            "background_color",
            self.tileset.get("background_crispy")
        );
        self.allies_status[1].as_mut().unwrap()
            .move_by(vector_2f(128.0, 416.0));


        self.allies_status[2].as_mut().unwrap().set_portrait(ctx,
            self.tileset.get("por_devil"));
        self.allies_status[2].as_mut().unwrap().set_components(
            "background_color",
            self.tileset.get("background_charged")
        );
        self.allies_status[2].as_mut().unwrap()
            .move_by(vector_2f(256.0, 416.0));

        self.allies_status[3].as_mut().unwrap().set_portrait(ctx,
            self.tileset.get("por_devil"));
        self.allies_status[3].as_mut().unwrap().set_components(
            "background_color",
            self.tileset.get("background_horrid")
        );
        self.allies_status[3].as_mut().unwrap()
            .move_by(vector_2f(384.0, 416.0));


        self.decorate();
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

    /// Gives decoration to the GUI Text, sets de fonts.
    pub fn decorate(&mut self) {
        for opt_status in &mut self.allies_status {
            if let Some(status) = opt_status.as_mut() {
                status.name_text.as_mut().unwrap().0.set_font(
                    *self.battle_gui_data.get_font("racing".to_string()
                    ).borrow(),
                    //graphics::Scale::uniform(18.0)
                    graphics::Scale{x: 18.0, y: 24.0}
                );

                status.hp_text.as_mut().unwrap().0.set_font(
                Font::default(),
                graphics::Scale::uniform(6.0));
            }
        }
    }
    
    pub fn draw(&self, ctx: &mut Context) {
        graphics::draw(ctx, &self.background, DrawParam::default());
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
            Rc::new(RefCell::new(tileset.get(PORTRAIT_BOX))
        ));
        
        result.pieces.insert(
            PORTRAIT.to_string(),
            Rc::new(RefCell::new(
                tileset.get(PORTRAIT)
            ))
        );
        
        result.pieces.insert(
            TEXT_BOX.to_string(),
            Rc::new(RefCell::new(
                tileset.get(TEXT_BOX)
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
    pub components: HashMap<String, Sprite>,
}

impl StatusCard {
    pub fn new(ctx: &mut Context, status: Rc<RefCell<Status>>, mut skin: Sprite) -> Self {
        skin.set_cut(ctx, [0.0, 0.0, 128.0, 64.0]);
        //default skin dimensions
        //BROKEN: DONT USE FIXME
        let mut result = Self {
            param: DrawParam::default(),
            skin: skin,
            status,
            name_text: None,
            hp_text: None,
            portrait: None,
            font: None,
            components: HashMap::new(),
        };

        result
    }

    pub fn from_tileset(ctx: &mut Context, status: Rc<RefCell<Status>>, tileset: &Tileset) -> Self {
        //if this is used, the tileset must have the standard statuscard
        //.tc format. TODO: Define that standard lol.
        let mut result = Self {
            param: DrawParam::default(),
            skin: tileset.get("plain_skin"),
            status: status.clone(),
            name_text: None,
            hp_text: None,
            portrait: None,
            font: None,
            components: HashMap::new(),
        };

        result.components.insert("background_mask".to_string(),
            tileset.get("background_mask"));
        result.components.insert("background_color".to_string(),
            tileset.get("background_color"));
        result.components.insert("light".to_string(),
            tileset.get("light_charged"));
        result.components.insert("shadow_mask".to_string(),
            tileset.get("shadow_mask"));
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

        self.name_text = Some(
            (Text::new((*RefCell::borrow(&self.status)).name),
            DrawParam::default().dest(name_offset))
        );

        self.hp_text = Some(
            (Text::new((*RefCell::borrow(&self.status)).hp.borrow().to_string()),
            DrawParam::default().dest(hp_offset))
        );

        self.hp_text.as_mut().unwrap().0.set_font(
            Font::default(),
            Scale::uniform(6.0),
        );
        
        //probs doesnt do a thing
        if let Some(font) = &self.font {
            self.name_text.as_mut().unwrap().0.set_font(
                *font.borrow(),
                Scale::uniform(6.0),
             );
            
            
        }
    }

    ///sets the component in the map of components for the given component name.
    pub fn set_components(&mut self, name: &'static str, component: Sprite) -> Option<Sprite> {
        self.components.insert(name.to_string(), component)
    }
    
    pub fn set_portrait(&mut self, ctx: &mut Context, mut sprite: Sprite){
       // sprite.set_cut(ctx, [320.0, 0.0, 32.0, 32.0]);
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
        let mut text = Text::new(self.get_status().hp().borrow().to_string());
        text.set_font(Font::default(), Scale::uniform(12.0));
        self.hp_text.as_mut().expect("couldnt update gui").0 = text;
        
    }
}

impl graphics::Drawable for StatusCard {
    fn draw(&self, ctx: &mut Context, param: DrawParam) -> GameResult<()> {
        //background
        self.components.get("background_color").unwrap().draw(ctx);
        self.components.get("background_mask").unwrap().draw(ctx);

        self.components.get("light").unwrap().draw(ctx);
        
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

        self.components.get("shadow_mask").unwrap().draw(ctx);

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

        for component in self.components.values_mut() {
            component.move_by(vector.clone());
        }
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



