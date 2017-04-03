extern crate image;

use std::path::Path;

// A trait that provides a representation of a material.
pub trait MaterialSource {
    // Identifier key used to access this material.
    fn name(&self) -> &str;
    // Access the vertex shader source code.
    fn vertex_shader(&self) -> &str;
    // Access the fragment shader source code.
    fn fragment_shader(&self) -> &str;
    // Access the path to the texture file.
    fn texture_file(&self) -> &Path;
    // The image type of the texture.
    fn texture_format(&self) -> image::ImageFormat;
}

