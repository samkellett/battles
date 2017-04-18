extern crate cgmath;
extern crate glium;

use std::rc::Rc;

// A reference to a texture in the collection.
#[derive(Clone, Debug)]
pub struct Texture {
    pub texture: Rc<glium::texture::Texture2d>,
    pub origin: cgmath::Vector2<i32>,
    pub dimensions: cgmath::Vector2<i32>,
}

