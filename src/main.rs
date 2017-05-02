#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate glium;

extern crate cgmath;
extern crate image;

mod config;
mod render;

use render::{GliumWindow, Window};
use render::sprites::{Mesh, Sprite};
use render::textures::Texture;
use render::transform::{Transform, Rotation};

fn main() {
    let window = GliumWindow::new();

    let sprite = {
        let texture = {
            let image = {
                let source = std::path::PathBuf::from("assets/opengl.png");
                let file = std::fs::File::open(source).unwrap();
                let file = std::io::BufReader::new(file);

                image::load(file, image::PNG)
                    .unwrap()
                    .to_rgba()
            };

            let image_dimensions = image.dimensions();
            let image = glium::texture::RawImage2d::from_raw_rgba_reversed(image.into_raw(),
                                                                           image_dimensions);
            glium::texture::Texture2d::new(&window.facade, image).unwrap()
        };
        let mesh = Mesh::square(1.0);
        
        // Example shaders.
        let v = include_str!("../assets/shaders/basic.vert");
        let f = include_str!("../assets/shaders/basic.frag");

        let program = glium::Program::from_source(&window.facade,
                                                  v,
                                                  f,
                                                  None)
            .unwrap();

        let tex = Texture {
            texture: texture, origin: cgmath::vec2(0, 0), dimensions: cgmath::vec2(256,256),
        };

        Sprite::new(mesh, tex, program, &window)
    };

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
                        sprite.render(&mut target,
                                      &transform,
                                      &perspective.into(),
                                      &params);
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
