extern crate glium;

mod render;

use render::materials::MaterialCollection;

fn main() {
    use glium::{DisplayBuild, Surface};

    let display = glium::glutin::WindowBuilder::new()
        .build_glium()
        .unwrap();

    // Load all materials needed the game.
    let materials = {
        let sources = vec![];
        MaterialCollection::new(&display, sources.into_iter());
    };

    // The event loop.
    loop {
        let mut target = display.draw();
        target.clear_color_and_depth((0.99, 0.83, 0.11, 1.0), 1.0);

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
