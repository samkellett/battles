extern crate glium;
extern crate image;

use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;

use super::{Material, MaterialSource};

// A collection of materials.
pub struct MaterialCollection {
    materials: HashMap<String, Material>,
}

impl MaterialCollection {
    // Create a new material collection from an iterator of sources.
    pub fn new<D, I>(display: &D, sources: I) -> MaterialCollection
        where D: glium::backend::Facade,
              I: Iterator,
              I::Item: MaterialSource,
    {
        let mut materials = HashMap::new();
        for source in sources {
            // Build the shader program.
            let program = glium::Program::from_source(display,
                                                      source.vertex_shader(),
                                                      source.fragment_shader(),
                                                      None)
                .unwrap();

            // Build the texture.
            let texture = {
                let file = File::open(source.texture_file()).unwrap();
                let file = BufReader::new(file);

                let image = image::load(file, source.texture_format())
                    .unwrap()
                    .to_rgba();

                let image_dimensions = image.dimensions();
                let image = glium::texture::RawImage2d::from_raw_rgba_reversed(image.into_raw(),
                                                                               image_dimensions);

                glium::texture::Texture2d::new(display, image).unwrap()
            };

            let material = Material {
                program: program,
                texture: texture,
            };
            materials.insert(source.name().to_owned(), material);
        }

        MaterialCollection { materials: materials }
    }

    // Get a reference to a registered material.
    pub fn material(&self, name: &str) -> &Material {
        &self.materials[name]
    }
}

