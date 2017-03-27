extern crate glium;

mod render;

use render::materials::MaterialCollection;

fn main() {
    use glium::DisplayBuild;

    let display = glium::glutin::WindowBuilder::new()
        .build_glium()
        .unwrap();

    // Load all materials needed the game.
    let materials = {
        let sources = vec![];
        MaterialCollection::new(&display, sources.into_iter());
    };
}
