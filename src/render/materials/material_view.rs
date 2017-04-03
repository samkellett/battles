extern crate image;

use std::path::Path;

use super::MaterialSource;

// A non-owning implementation of the MaterialSource trait.
pub struct MaterialView<'a> {
    pub name: &'a str,
    pub vertex_shader: &'a str,
    pub fragment_shader: &'a str,
    pub texture_file: &'a Path,
    pub texture_format: image::ImageFormat,
}

impl<'a> MaterialSource for MaterialView<'a> {
    fn name(&self) -> &str { self.name }
    fn vertex_shader(&self) -> &str { self.vertex_shader }
    fn fragment_shader(&self) -> &str { self.fragment_shader }
    fn texture_file(&self) -> &Path { self.texture_file }
    fn texture_format(&self) -> image::ImageFormat { self.texture_format }
}

