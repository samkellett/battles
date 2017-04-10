extern crate glium;

use glium::backend::glutin_backend as glutin;
use glium::{Surface, DisplayBuild, VertexBuffer, Frame, Vertex, IndexBuffer};
use glium::glutin::WindowBuilder;
use glium::index::PrimitiveType;

pub struct Window {
    pub facade: glutin::GlutinFacade,
}

impl Window {
    pub fn new() -> Window {
        let facade = WindowBuilder::new().build_glium().unwrap();

        Window { facade: facade }
    }

    pub fn create_vertex_buffer<T>(&self, vertices: &Vec<T>) -> VertexBuffer<T>
        where T: Vertex
    {
        VertexBuffer::new(&self.facade, vertices).unwrap()
    }

    pub fn create_index_buffer(&self, indices: &Vec<u16>) -> IndexBuffer<u16> {
        IndexBuffer::new(&self.facade, PrimitiveType::TrianglesList, indices).unwrap()
    }

    pub fn draw<F>(&self, draw_function: F)
        where F: FnOnce(&mut Frame)
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
