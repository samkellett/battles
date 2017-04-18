extern crate toml;

use std::fs::File;
use std::path::Path;

use render::materials::MaterialFileSource;
use render::textures::TextureFileSource;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub title: String,
    pub textures: Vec<TextureFileSource>,
    pub materials: Vec<MaterialFileSource>,
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

