extern crate toml;

use std::fs::File;
use std::path::Path;

use render::sprites::SpriteSource;
use world::GameObjectSource;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub title: String,
    pub sprites: Vec<SpriteSource>,
    pub game_objects: Vec<GameObjectSource>,
}

impl Config {
    pub fn from_file<P>(path: P) -> Config
        where P: AsRef<Path>
    {
        use std::io::Read;

        let mut file = File::open(path).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();

        toml::from_str(&contents).unwrap()
    }
}
