extern crate glium;
extern crate cgmath;

use render::sprites::mesh::Mesh;
use render::sprites::vertex::Vertex;
use render::materials::Material;
use render::textures::TextureCollection;
use render::window::Window;
use render::transform::Transform;
use glium::{VertexBuffer, IndexBuffer, Frame, DrawParameters, Surface};
use cgmath::Matrix4;
use cgmath::conv::array4x4;

#[derive(Debug)]
pub struct Sprite<'a> {
    material: &'a Material,
    vertex_buffer: VertexBuffer<Vertex>,
    indices_buffer: IndexBuffer<u16>,
}

impl<'a> Sprite<'a> {
    pub fn from_mesh<T: Window>(mesh: Mesh, material: &'a Material, window: &T) -> Sprite<'a> {
        let vertex_buffer = window.create_vertex_buffer(&mesh.verts);
        let indices_buffer = window.create_index_buffer(&mesh.indices);

        Sprite {
            material: material,
            vertex_buffer: vertex_buffer,
            indices_buffer: indices_buffer,
        }
    }

    pub fn render(&self,
                  target: &mut Frame,
                  textures: &TextureCollection,
                  transform: &Transform,
                  projection: &Matrix4<f32>,
                  params: &DrawParameters) {

        let texture = textures.texture(self.material.texture.id);
        target
            .draw(&self.vertex_buffer,
                  &self.indices_buffer,
                  &self.material.program,
                  &uniform! { modelView: array4x4(transform.matrix),
                              perspective: array4x4(*projection),
                              diffuse_tex: texture },
                  params)
            .unwrap();
    }
}
