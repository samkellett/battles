extern crate image;

use std::path::Path;

// A non-owning implementation of the MaterialSource trait.
pub struct MaterialView<'a> {
    // String key used to access this material.
    pub name: &'a str,
    // Access the vertex shader source code.
    pub vertex_shader: &'a str,
    // Access the fragment shader source code.
    pub fragment_shader: &'a str,
    // Access the path to the texture file.
    pub texture_file: &'a Path,
    // The image type of the texture.
    pub texture_format: image::ImageFormat,
}

