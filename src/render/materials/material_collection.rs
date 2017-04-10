extern crate glium;
extern crate image;

use std::collections::HashMap;

use super::{Material, MaterialSource};

// A collection of materials.
pub struct MaterialCollection<'a> {
    materials: HashMap<String, Material<'a>>,
}

impl<'a> MaterialCollection<'a> {
    // Create a new material collection from an iterator of sources.
    pub fn new<D, I>(display: &D, sources: I) -> MaterialCollection<'a>
        where D: glium::backend::Facade,
              I: Iterator,
              I::Item: MaterialSource<'a>,
    {
        let mut materials = HashMap::new();
        for source in sources {
            // Build the shader program.
            let program = glium::Program::from_source(display,
                                                      source.vertex_shader(),
                                                      source.fragment_shader(),
                                                      None)
                .unwrap();

            let material = Material {
                program: program,
                texture: source.texture(),
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

