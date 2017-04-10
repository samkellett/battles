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
use render::transform::Transform;
use render::transform::Rotation;
use render::window::Window;

fn main() {
    let window = Window::new();

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
        MaterialCollection::new(&window.facade, sources.into_iter())
    };

    let badger_mat = materials.material("badger");

    let sprite = Sprite::from_mesh(Mesh::square(1.0), &badger_mat, &window.facade);

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
                        sprite.render(&mut target, &transform, &perspective.into(), &params);
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
