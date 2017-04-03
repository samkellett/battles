use render::textures;

// A trait that provides a representation of a material.
pub trait MaterialSource<'a> {
    // Identifier key used to access this material.
    fn name(&self) -> &str;
    // Access the vertex shader source code.
    fn vertex_shader(&self) -> &str;
    // Access the fragment shader source code.
    fn fragment_shader(&self) -> &str;
    // Access the texture slice.
    fn texture(&self) -> textures::Texture<'a>;
}

