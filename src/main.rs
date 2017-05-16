#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate glium;

extern crate cgmath;
extern crate image;

mod config;
mod render;
mod world;

use config::Config;
use render::{RenderEngine};
use render::transform::{Transform, Rotation};
use world::GameObject;

fn main() {
    let config = Config::from_file("assets/example.toml");
    println!("{:?}", config);

    let render_engine = RenderEngine::new(&config);
    let game_objects = GameObject::from_config(config.game_objects.into_iter(), 
                                               &|s| render_engine.get_sprite_id(s));

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

