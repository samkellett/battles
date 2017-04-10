extern crate glium;

use glium::backend::glutin_backend as glutin;
use glium::{Surface, DisplayBuild};

pub struct Window {
    pub facade: glutin::GlutinFacade,
}

impl Window {
    pub fn new() -> Window {
        let facade = glium::glutin::WindowBuilder::new()
            .build_glium()
            .unwrap();

        Window { facade: facade }
    }

    pub fn draw<F>(&self, draw_function: F)
        where F: FnOnce(&mut glium::Frame)
    {
        let mut frame = self.facade.draw();
        frame.clear_color_and_depth((0.99, 0.83, 0.11, 1.0), 1.0);

        draw_function(&mut frame);

        frame.finish().unwrap();
    }

    pub fn get_aspect(&self) -> f32 {
        let window = self.facade.get_window().unwrap();
        let (width, height) = window.get_inner_size_pixels().unwrap();
        height as f32 / width as f32
    }
}
