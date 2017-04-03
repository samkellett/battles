extern crate cgmath;
extern crate image;

use std::path::PathBuf;

pub struct SliceSource {
    pub name: String,
    pub origin: cgmath::Vector2<i32>,
    pub dimensions: cgmath::Vector2<i32>,
}

pub struct TextureSource {
    pub texture_file: PathBuf,
    pub texture_format: image::ImageFormat,
    pub slices: Vec<SliceSource>,
}

