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

use ggez::mint::Point2;

use super::utils;

static FILE_EXTENSION: &str = ".tc";


#[derive(Debug)]
pub struct Tileset {
	pub terrains: HashMap<String, TilesetData>,
	pub objects: HashMap<String, TilesetData>,
}

impl Tileset {
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

	let mut tileset = Tileset::new();
	for r in result {
		tileset.add_from_data((r.0.clone(), TilesetData::from_data(r).1));
	}
	println!("{:#?}", tileset);
	return Ok(());
	}
}


