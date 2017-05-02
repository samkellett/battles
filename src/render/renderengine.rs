use super::Window;

pub struct RenderEngine<'a, T: 'a>
    where T: Window
{
    pub window: &'a T,
}
