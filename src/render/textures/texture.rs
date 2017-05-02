extern crate cgmath;
extern crate glium;

// A reference to a texture in the collection.
#[derive(Debug)]
pub struct Texture {
    // An index that points to the texture.
    pub texture: glium::texture::Texture2d,
    // The top-left corner of the texture.
    pub origin: cgmath::Vector2<i32>,
    // The width and height of the texture.
    pub dimensions: cgmath::Vector2<i32>,
}
