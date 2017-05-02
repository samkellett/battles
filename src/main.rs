#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate glium;

extern crate cgmath;
extern crate image;

mod config;
mod render;

use config::Config;

use render::materials::MaterialCollection;
use render::sprites::{Mesh, Sprite};
use render::textures::TextureCollection;
use render::transform::{Rotation, Transform};
use render::GliumWindow;
use render::Window;
use render::RenderEngine;

fn main() {
    let config = Config::from_file("assets/example.toml");
    println!("{:?}", config);

    let window = GliumWindow::new(&config);

    let render_engine = {
        let textures = TextureCollection::new(&window.facade, config.textures.into_iter());

        // Load all our textures and materials.
        let materials =
            MaterialCollection::new(&window.facade, config.materials.into_iter(), &textures);

        RenderEngine {
            window: &window,
            textures: textures,
            materials: materials,
        }
    };

    let badger_mat = render_engine.materials.material("badger_mat");

    let sprite = Sprite::from_mesh(Mesh::square(1.0), &badger_mat, render_engine.window);

    let params = glium::DrawParameters {
        depth: glium::Depth {
            test: glium::draw_parameters::DepthTest::IfLess,
            write: true,
            ..Default::default()
        },
        ..Default::default()
    };

    // The event loop.
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
                                      &render_engine.textures,
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
