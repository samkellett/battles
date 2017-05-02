extern crate glium;
extern crate cgmath;

use render::textures::Texture;
use render::sprites::mesh::Mesh;
use render::sprites::vertex::Vertex;
use render::window::Window;
use render::transform::Transform;
use glium::{VertexBuffer, IndexBuffer, Frame, DrawParameters, Surface, Program};
use cgmath::Matrix4;
use cgmath::conv::array4x4;
use config::Config;

#[derive(Debug)]
pub struct Sprite {
    program: Program,
    texture: Texture, 
    vertex_buffer: VertexBuffer<Vertex>,
    indices_buffer: IndexBuffer<u16>,
}

impl Sprite {
    pub fn new<T: Window>(mesh: Mesh, texture: Texture, program: Program, window: &T) -> Sprite {
        let vertex_buffer = window.create_vertex_buffer(&mesh.verts);
        let indices_buffer = window.create_index_buffer(&mesh.indices);

        Sprite {
            program,
            texture,
            vertex_buffer,
            indices_buffer,
        }
    }

    pub fn render(&self,
                  target: &mut Frame,
                  transform: &Transform,
                  projection: &Matrix4<f32>,
                  params: &DrawParameters) {

        target
            .draw(&self.vertex_buffer,
                  &self.indices_buffer,
                  &self.program,
                  &uniform! { modelView: array4x4(transform.matrix),
                              perspective: array4x4(*projection),
                              diffuse_tex: &self.texture.texture },
                  params)
            .unwrap();
    }
}
