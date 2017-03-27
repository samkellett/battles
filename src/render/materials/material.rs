extern crate glium;

// The internal representation of a material.
#[derive(Debug)]
pub struct Material {
    pub program: glium::Program,
    pub texture: glium::texture::Texture2d,
}

