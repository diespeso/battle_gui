use std::error::Error;
use std::fmt;
use std::io::prelude::*;
use std::io;
use std::fs::File;
use std::collections::HashMap;
use std::io::BufReader;
use std::path::Path;
use std::convert::TryInto;
use std::convert::From;
use ggez::graphics::{self, Image, DrawParam};
use super::sprite::Sprite;
use ggez::filesystem;

use ggez::mint::Point2;
use ggez::Context;

use super::utils;

static FILE_EXTENSION: &str = ".tc";
static IMAGE_EXTENSION: &str = ".png";
static SPRITE_SIZE: f32 = 32.0;

pub struct Tilemap { //

}

#[derive(Debug)]
pub struct Tileset { //set of 
	pub configuration: TilesetConfiguration,
	pub image: Image,
	pub sprites: HashMap<String, Sprite>,
}

impl Tileset {
	pub fn new(ctx: &mut Context, name: String) -> Result<Self, io::Error> {
		let conf_file = format!("{}{}", name, FILE_EXTENSION);
		let mut image_file = format!("/assets/{}{}", name, IMAGE_EXTENSION);
		let image_file = Path::new(&image_file);
		println!("{:#?} {:#?}", image_file, conf_file);
		let mut parse_result = parse_file(Path::new(&conf_file))?.0;

		parse_result.remove_entry(""); //weird bug
		let mut result: Vec<(String, Vec<String>)> =
			parse_result.drain().collect();
			
		let mut tileset_conf = TilesetConfiguration::new();
		let mut image = Image::new(ctx, &image_file).expect("couldn't load image");
		image.set_filter(graphics::FilterMode::Nearest);
		for r in result {
			tileset_conf.add_from_data(
				(r.0.clone(), TilesetData::from_data(r).1)
			);
		}
		
		Ok(
		Self {
			configuration: tileset_conf.clone(),
			image: image.clone(),
			sprites: Self::create_sprites(ctx, &image, &tileset_conf),
		}
		)
	}
	
	fn create_sprites(ctx: &mut Context, image: &Image, conf: &TilesetConfiguration) -> HashMap<String, Sprite> {
		let mut map = HashMap::new();
		let mut sprite = Sprite::new(image.clone());
		for (name, obj) in &conf.objects {
			//sprite = sprite
			sprite = Sprite::new(image.clone());
			sprite.set_cut(ctx, [obj.position.x * SPRITE_SIZE,
				obj.position.y * SPRITE_SIZE,
				obj.size.x * SPRITE_SIZE,
				obj.size.y * SPRITE_SIZE]);
			map.insert(name.to_string(), sprite);
		}
		
		sprite = Sprite::new(image.clone());
		for (name, terr) in &conf.terrains {
			sprite = Sprite::new(image.clone());
			sprite.set_cut(ctx, [terr.position.x * SPRITE_SIZE,
				terr.position.y * SPRITE_SIZE,
				terr.size.x * SPRITE_SIZE,
				terr.size.y * SPRITE_SIZE]);
			map.insert(name.to_string(), sprite);
		}
		
		map
	}
	
	pub fn get(&self, name: String) -> Sprite {
		self.sprites[&name].clone()
	}
	
	pub fn get_mut(&mut self, name: String) -> &mut Sprite {
		self.sprites.get_mut(&name).expect("failed to get sprite from hashmap of tileset")
	}
	
	pub fn draw(&self, ctx: &mut Context) {
		for sprite in self.sprites.values() {
			sprite.draw(ctx);
		}
	}
	
	pub fn into_relation_name_param(&self) -> HashMap<String, DrawParam> {
		let mut result: HashMap<String, DrawParam> = HashMap::new();
		for (name, sprite) in &self.sprites {
			result.insert(name.to_string(), sprite.params());
		}
		result
	}
}


#[derive(Debug, Clone)]
pub struct TilesetConfiguration {
	pub terrains: HashMap<String, TilesetData>,
	pub objects: HashMap<String, TilesetData>,
}

impl TilesetConfiguration {
	pub fn new() -> Self {
		Self {
			terrains: HashMap::new(),
			objects: HashMap::new(),
		}
	}
	
	pub fn add_from_info(&mut self, info: HashMap<String, TilesetData>) {
		let mut _name: String;
		let mut data: TilesetData;
		for name in info.keys() {
			_name = name.clone();
			data = info[&_name].clone();
			match &info[&_name]._type {
				TilesetType::TERRAIN => {
					self.terrains.insert(_name, data);
				},
				TilesetType::OBJECT => {
					self.objects.insert(_name, data);
				},
			}
		}
	}
	
	pub fn add_from_data(&mut self, data: (String, TilesetData)) {
		let _type = data.1._type.clone();
		match _type {
			TilesetType::TERRAIN => {
				self.terrains.insert(data.0.clone(), 
				data.1.clone());
			},
			TilesetType::OBJECT => {
				self.objects.insert(data.0.clone(),
				data.1.clone());
			}
		}
	}
}

#[derive(Debug, Clone)]
pub struct TilesetData {
	pub _type: TilesetType,
	pub position: Point2<f32>,
	pub size: Point2<f32>,
}

impl TilesetData {
	pub fn new(_type: TilesetType, pos: Point2<f32>, size: Point2<f32>) -> Self {
		Self {
			_type: _type,
			position: pos,
			size: size,
		}
	}
	
	pub fn from_data(data: (String, Vec<String>))
	-> (String, Self) {
		let name = data.0.clone();
		let position = utils::from_str_to_point2f(data.1[1].clone());
		let size = utils::from_str_to_point2f(data.1[2].clone());
		(name, Self::new(data.1[0].clone().into(), position, size))
		
	}
}

#[derive(Debug)]
pub struct TilesetParserError {}

impl Error for TilesetParserError {
	
}

#[derive(Debug, PartialEq, Clone)]
pub enum TilesetType {
	TERRAIN,
	OBJECT,
}

impl From<String> for TilesetType {
	fn from(item: String) -> Self {
		match item.as_str() {
			 "terrain" => TilesetType::TERRAIN,
			 "object" => TilesetType:: OBJECT,
			 _ => panic!("TilesetType doesnt exist"),
		}
	}
}

impl fmt::Display for TilesetParserError {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "no se pudo cargar el archivo de .tc")
	}
}

type Position = Point2<f32>;
type Size = Point2<f32>;

fn parse_file(path: &Path) -> Result<(HashMap<String, Vec<String>>, i32), io::Error> {
	let mut f = File::open(path)?;
	let mut buf_reader = BufReader::new(f);
	let mut content = String::new();
	
	buf_reader.read_to_string(&mut content)?;
	let content_sets: Vec<&str> = content.split("set").collect(); // count sets
	let n = content_sets.len() - 1;
	let content: Vec<&str> = content.split(' ').collect();
	
	let mut objects_found = HashMap::<String, Vec<String>>::new();
	
	let mut dummy = (String::new(), Vec::<String>::new());
	let mut i = 0;
	while i < content.len() {
		match content[i] {
			"set" => {
				i += 1;
				(&mut dummy).1.push(content[i].to_string()); //type
				i += 1;
				(&mut dummy).0 = content[i].to_string(); //name
				i += 4;
				(&mut dummy).1.push(format!("{} {}", content[i],
					content[i + 1]));
				i += 4;
				(&mut dummy).1.push(format!("{} {}", content[i],
					content[i + 1]));
			}
			_ => (),
		};
		objects_found.insert(dummy.0, dummy.1);
		dummy = (String::new(), Vec::<String>::new());
		i += 1;

	}
	
	Ok((objects_found, n.try_into().unwrap()))
}


#[cfg(test)]
mod test {
	use ggez::filesystem;
	use std::path::Path;
	use super::{*};
	use std::io;
	use std::io::prelude::*;
	use std::fs::File;
	use std::io::BufReader;
	
	#[test]
	fn test_file_io() -> Result<(), io::Error> {
	//archivo de configuración en directorio principal
	let filename = format!("{}{}", "uno", FILE_EXTENSION);
	//let mut f = File::open(Path::new(&filename))?;
	let mut parse_result = parse_file(Path::new(&filename))?.0;
	parse_result.remove_entry(""); //TODO: Bug, idk why
	//but an empty entry "" gets created and makes everything
	//crash, but this line deletes that entry so its all good
	let mut result: Vec<(String, Vec<String>)> = parse_result.drain().collect::<Vec<(String, Vec<String>)>>(); //converts a hashmap to a vector of pairs (my mind)

	let mut tileset = TilesetConfiguration::new();
	for r in result {
		tileset.add_from_data((r.0.clone(), TilesetData::from_data(r).1));
	}
	
	return Ok(());
	}
}


