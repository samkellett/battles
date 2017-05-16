#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate glium;

extern crate cgmath;
extern crate image;

mod config;
mod render;

use config::Config;
use glium::DrawParameters;
use render::{GliumWindow, Window};
use render::sprites::Sprite;
use render::transform::{Transform, Rotation};
use cgmath::Matrix4;

struct RenderEngine<'a> {
    window: GliumWindow,
    sprites: Vec<Sprite>,
    perspective: Matrix4<f32>,
    draw_parameters: DrawParameters<'a>,
}

impl<'a> RenderEngine<'a> {
    fn draw(&self, transform: &Transform) {
        self.window
            .draw(|mut target| for sprite in &self.sprites {
                      sprite.render(&mut target,
                                    &transform,
                                    &self.perspective,
                                    &self.draw_parameters);
                  });
    }

    fn new(config: &Config) -> RenderEngine<'a> {
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

fn main() {
    let config = Config::from_file("assets/example.toml");
    let render_engine = RenderEngine::new(&config);

    let mut transform = Transform::new();

    loop {
        transform.rotate_z(Rotation::Deg(360.0 / 60.0));

        // This will be
        // render_engine.draw(sprite_index, &transform); // Add command to buffer
        // render_engine.render(); // Render all commands
        render_engine.draw(&transform);

        for event in render_engine.window.facade.poll_events() {
            match event {
                // The window has been closed.
                glium::glutin::Event::Closed => return,
                _ => (),
            }
        }
    }

}
