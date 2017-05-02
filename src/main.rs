#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate glium;

extern crate cgmath;
extern crate image;

mod config;
mod render;

use config::Config;

use render::{GliumWindow, Window};
use render::sprites::Sprite;
use render::transform::{Transform, Rotation};

fn main() {
    let config = Config::from_file("assets/example.toml");
    println!("{:?}", config);

    let window = GliumWindow::new(&config);
    let sprites = Sprite::from_config(&window, config.sprites.into_iter());

    let params = glium::DrawParameters {
        depth: glium::Depth {
            test: glium::draw_parameters::DepthTest::IfLess,
            write: true,
            ..Default::default()
        },
        ..Default::default()
    };

    let mut transform = Transform::new();

    loop {
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

        transform.rotate_z(Rotation::Deg(360.0 / 60.0));
        window.draw(|mut target| {
                        for sprite in &sprites {
                            sprite.render(&mut target,
                                          &transform,
                                          &perspective.into(),
                                          &params);
                        }
                    });

        for event in window.facade.poll_events() {
            match event {
                // The window has been closed.
                glium::glutin::Event::Closed => return,
                _ => (),
            }
        }
    }

}
