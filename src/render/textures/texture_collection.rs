extern crate cgmath;
extern crate glium;
extern crate image;

use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::rc::Rc;

use super::{Texture, TextureSource};

// Internal representation of a texture in a texture collection.
struct TextureSlice {
    // An index that points to the texture.
    id: usize,
    // The top-left corner of the texture.
    origin: cgmath::Vector2<i32>,
    // The width and height of the texture.
    dimensions: cgmath::Vector2<i32>,
}

// A collection of textures and mappings onto each one.
pub struct TextureCollection {
    // All of the textures available.
    textures: Vec<Rc<glium::texture::Texture2d>>,
    // Maps of texture by name to it's source file and where in the file it is.
    views: HashMap<String, TextureSlice>,
}

impl TextureCollection {
    pub fn new<D, I>(display: &D, sources: I) -> TextureCollection
        where D: glium::backend::Facade,
              I: Iterator<Item=TextureSource>,
    {
        let mut textures = Vec::new();
        let mut views = HashMap::new();

        for source in sources {
            // Load the texture.
            let image = {
                let file = File::open(source.texture_file).unwrap();
                let file = BufReader::new(file);

                image::load(file, image::PNG)
                    .unwrap()
                    .to_rgba()
            };

            let image_dimensions = image.dimensions();
            let texture = {
                let image = glium::texture::RawImage2d::from_raw_rgba_reversed(image.into_raw(),
                                                                               image_dimensions);
                glium::texture::Texture2d::new(display, image).unwrap()
            };

            let next_id = textures.len();
            textures.push(Rc::new(texture));

            for slice in source.slices {
                // Build the slice.
                let view = TextureSlice {
                    id: next_id,
                    origin: cgmath::vec2(slice.origin[0], slice.origin[1]),
                    dimensions: cgmath::vec2(slice.dimensions[0], slice.dimensions[1]),
                };
                views.insert(slice.name.to_owned(), view);
            }
        }

        TextureCollection { textures: textures, views: views }
    }

    // Get a reference to a registered material.
    pub fn texture(&self, name: &str) -> Texture {
        let view = &self.views[name];
        Texture {
            texture: self.textures[view.id].clone(),
            origin: view.origin,
            dimensions: view.dimensions,
        }
    }
}
