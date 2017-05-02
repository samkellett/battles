use glium::{VertexBuffer, Frame, Vertex, IndexBuffer};

pub trait Window {
    fn create_vertex_buffer<T>(&self, vertices: &Vec<T>) -> VertexBuffer<T> where T: Vertex;

    fn create_index_buffer(&self, indices: &Vec<u16>) -> IndexBuffer<u16>;

    fn draw<F>(&self, draw_function: F) where F: FnOnce(&mut Frame);

    fn get_aspect(&self) -> f32;
}
