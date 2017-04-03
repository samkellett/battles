#[macro_use]
extern crate glium;
extern crate cgmath;
extern crate image;

mod render;

use std::path::Path;
use render::materials::MaterialCollection;
use render::materials::MaterialView;
use render::sprites::Mesh;
use render::sprites::Sprite;

fn main() {
    use glium::{DisplayBuild, Surface};

    let display = glium::glutin::WindowBuilder::new()
        .build_glium()
        .unwrap();

    // Example shaders.
    let v = include_str!("../assets/shaders/basic.vert");
    let f = include_str!("../assets/shaders/basic.frag");

    // An example material.
    let material = MaterialView {
        name: "badger",
        vertex_shader: v,
        fragment_shader: f,
        texture_file: &Path::new("assets/opengl.png"),
        texture_format: image::PNG,
    };

    // Load all materials needed the game.
    let materials = {
        let sources = vec![material];
        MaterialCollection::new(&display, sources.into_iter())
    };

    let badger_mat = materials.material("badger");

    let mesh = Mesh::square(1.0);
    let mut sprite = Sprite::new(&mesh, &badger_mat, &display);

    let params = glium::DrawParameters {
        depth: glium::Depth {
            test: glium::draw_parameters::DepthTest::IfLess,
            write: true,
            ..Default::default()
        },
        ..Default::default()
    };

    // The event loop.
    loop {
        let mut target = display.draw();
        target.clear_color_and_depth((0.99, 0.83, 0.11, 1.0), 1.0);

        let perspective = {
            let (width, height) = target.get_dimensions();
            let aspect = height as f32 / width as f32;
            let span = 10.0; // World units between the left and right sides of the window
            cgmath::ortho(-span / 2.0,
                          span / 2.0,
                          -aspect * span / 2.0,
                          aspect * span / 2.0,
                          -1.0,
                          1.0)
        };

        use cgmath::One;
        sprite.rotation += 1.0 / 90.0 * 360.0;
        sprite.render(&mut target,
                      &cgmath::Matrix4::one(),
                      &perspective.into(),
                      &params);

        target.finish().unwrap();

        for event in display.poll_events() {
            match event {
                // The window has been closed.
                glium::glutin::Event::Closed => return,
                _ => (),
            }
        }
    }
}
