extern crate cgmath;
extern crate glium;

use super::Window;
use super::GliumWindow;
use super::sprites::{Sprite, SpriteId};
use cgmath::Matrix4;
use glium::DrawParameters;
use render::transform::Transform;
use config::Config;

pub struct RenderEngine<'a> {
    pub window: GliumWindow,
    sprites: Vec<Sprite>,
    perspective: Matrix4<f32>,
    draw_parameters: DrawParameters<'a>,
}

impl<'a> RenderEngine<'a> {
    pub fn get_sprite_id(&self, name: &str) -> SpriteId {
        let index = self.sprites.iter().position(|ref s| s.name == name).unwrap();
        SpriteId(index)
    }

    pub fn draw(&self, transform: &Transform) {
        self.window
            .draw(|mut target| for sprite in &self.sprites {
                      sprite.render(&mut target,
                                    &transform,
                                    &self.perspective,
                                    &self.draw_parameters);
                  });
    }

    pub fn new(config: &Config) -> RenderEngine<'a> {
        let window = GliumWindow::new(&config);
        let sprites = Sprite::from_config(&window, config.sprites.iter());
        let perspective = {
            let aspect = window.get_aspect();
            let span = 10.0; // World units between the left and right sides of the window
            cgmath::ortho(-span / 2.0,
                          span / 2.0,
                          -aspect * span / 2.0,
                          aspect * span / 2.0,
                          -1.0,
                          1.0)
        };

        let draw_parameters = DrawParameters {
            depth: glium::Depth {
                test: glium::draw_parameters::DepthTest::IfLess,
                write: true,
                ..Default::default()
            },
            ..Default::default()
        };

        RenderEngine {
            window,
            sprites,
            perspective,
            draw_parameters,
        }
    }
}

