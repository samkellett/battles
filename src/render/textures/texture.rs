extern crate cgmath;
extern crate glium;
extern crate image;

use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;

// on-disk representation.
#[derive(Debug, Deserialize)]
pub struct TextureSource {
    pub file: PathBuf,
    pub origin: [i32; 2],
    pub dimensions: [i32; 2],
}

#[derive(Debug)]
pub struct Texture {
    // The source.
    pub texture: glium::texture::Texture2d,
    // The top-left corner of the texture.
    pub origin: cgmath::Vector2<i32>,
    // The width and height of the texture.
    pub dimensions: cgmath::Vector2<i32>,
}

impl Texture {
    pub fn from_source<D>(display: &D, source: &TextureSource) -> Texture
        where D: glium::backend::Facade
    {
        let texture = {
            // Load the texture.
            let image = {
                let file = File::open(&source.file).unwrap();
                let file = BufReader::new(file);

                image::load(file, image::PNG).unwrap().to_rgba()
            };

            let image_dimensions = image.dimensions();
            let image = glium::texture::RawImage2d::from_raw_rgba_reversed(image.into_raw(),
                                                                           image_dimensions);
            glium::texture::Texture2d::new(display, image).unwrap()
        };

        let origin = cgmath::vec2(source.origin[0], source.origin[1]);
        let dimensions = cgmath::vec2(source.dimensions[0], source.dimensions[1]);

        Texture { texture, origin, dimensions }
    }
}

