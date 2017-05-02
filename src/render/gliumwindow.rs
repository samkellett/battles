extern crate glium;

use glium::backend::glutin_backend as glutin;
use glium::{Surface, DisplayBuild, VertexBuffer, Frame, Vertex, IndexBuffer};
use glium::glutin::WindowBuilder;
use glium::index::PrimitiveType;
use super::window::Window;

use config::Config;

pub struct GliumWindow {
    pub facade: glutin::GlutinFacade,
}

impl GliumWindow {
    pub fn new(config: &Config) -> GliumWindow {
        let facade = WindowBuilder::new()
            .with_title(config.title.clone())
            .build_glium()
            .unwrap();

        GliumWindow { facade: facade }
    }
}

impl Window for GliumWindow {
    fn create_vertex_buffer<T>(&self, vertices: &Vec<T>) -> VertexBuffer<T>
        where T: Vertex
    {
        VertexBuffer::new(&self.facade, vertices).unwrap()
    }

    fn create_index_buffer(&self, indices: &Vec<u16>) -> IndexBuffer<u16> {
        IndexBuffer::new(&self.facade, PrimitiveType::TrianglesList, indices).unwrap()
    }

    fn draw<F>(&self, draw_function: F)
        where F: FnOnce(&mut Frame)
    {
        let mut frame = self.facade.draw();
        frame.clear_color_and_depth((0.99, 0.83, 0.11, 1.0), 1.0);

        draw_function(&mut frame);

        frame.finish().unwrap();
    }

    fn get_aspect(&self) -> f32 {
        let window = self.facade.get_window().unwrap();
        let (width, height) = window.get_inner_size_pixels().unwrap();
        height as f32 / width as f32
    }
}
