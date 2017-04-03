extern crate glium;

use render::textures;

// The internal representation of a material.
#[derive(Debug)]
pub struct Material<'a> {
    pub program: glium::Program,
    pub texture: textures::Texture<'a>,
}

