extern crate glium;
extern crate image;

use std::collections::HashMap;
use std::fs::File;
use std::path::Path;

use super::{Material, MaterialSource};

use render::textures::TextureCollection;

// Load a file into a string.
fn file_get_contents<P>(path: P) -> String
    where P: AsRef<Path>
{
        use std::io::Read;

        let mut file = File::open(path).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();

        contents
}

// A collection of materials.
pub struct MaterialCollection<'a> {
    materials: HashMap<String, Material<'a>>,
}

impl<'a> MaterialCollection<'a> {
    // Create a new material collection from an iterator of sources.
    pub fn new<D, I>(display: &D, sources: I, textures: &'a TextureCollection)
        -> MaterialCollection<'a>
        where D: glium::backend::Facade,
              I: Iterator<Item = MaterialSource>,
    {
        let mut materials = HashMap::new();
        for source in sources {
            let vertex_shader = file_get_contents(source.vertex_shader);
            let fragment_shader = file_get_contents(source.fragment_shader);

            // Build the shader program.
            let program = glium::Program::from_source(display,
                                                      &vertex_shader,
                                                      &fragment_shader,
                                                      None)
                .unwrap();

            let material = Material {
                program: program,
                texture: textures.texture(&source.texture),
            };
            materials.insert(source.name.clone(), material);
        }

        MaterialCollection { materials: materials }
    }

    // Get a reference to a registered material.
    pub fn material(&self, name: &str) -> &Material {
        &self.materials[name]
    }
}

