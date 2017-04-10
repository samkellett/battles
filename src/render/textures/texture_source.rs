extern crate cgmath;
extern crate image;

use std::path::PathBuf;

#[derive(Debug, Deserialize)]
pub struct SliceSource {
    pub name: String,
    pub origin: [i32; 2],
    pub dimensions: [i32; 2],
}

#[derive(Debug, Deserialize)]
pub struct TextureSource {
    pub texture_file: PathBuf,
    pub slices: Vec<SliceSource>,
}

