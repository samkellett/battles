extern crate cgmath;
extern crate glium;

// A reference to a texture in the collection.
#[derive(Copy, Clone, Debug)]
pub struct Texture<'a> {
    pub texture: &'a glium::texture::Texture2d,
    pub origin: &'a cgmath::Vector2<i32>,
    pub dimensions: &'a cgmath::Vector2<i32>,
}

