use render::textures;

use super::MaterialSource;

// A non-owning implementation of the MaterialSource trait.
pub struct MaterialView<'a> {
    pub name: &'a str,
    pub vertex_shader: &'a str,
    pub fragment_shader: &'a str,
    pub texture: textures::Texture<'a>,
}

impl<'a> MaterialSource<'a> for MaterialView<'a> {
    fn name(&self) -> &str { self.name }
    fn vertex_shader(&self) -> &str { self.vertex_shader }
    fn fragment_shader(&self) -> &str { self.fragment_shader }
    fn texture(&self) -> textures::Texture<'a> { self.texture }
}

