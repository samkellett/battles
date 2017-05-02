use super::Window;
use super::textures::TextureCollection;
use super::materials::MaterialCollection;

pub struct RenderEngine<'a, T: 'a>
    where T: Window
{
    pub window: &'a T,
    pub textures: TextureCollection,
    pub materials: MaterialCollection,
}
