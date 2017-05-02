extern crate cgmath;

// A reference to a texture in the collection.
#[derive(Copy, Clone, Debug)]
pub struct Texture {
    // An index that points to the texture.
    pub id: usize,
    // The top-left corner of the texture.
    pub origin: cgmath::Vector2<i32>,
    // The width and height of the texture.
    pub dimensions: cgmath::Vector2<i32>,
}

