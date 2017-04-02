extern crate glium;
extern crate cgmath;

use render::sprites::mesh::Mesh;
use render::sprites::vertex::Vertex;
use render::materials::Material;

#[derive(Debug)]
pub struct Sprite<'a> {
    mesh: &'a Mesh,
    material: &'a Material,
    pub position: [f32; 2],
    pub rotation: f32,
    vertex_buffer: glium::VertexBuffer<Vertex>,
    indices_buffer: glium::IndexBuffer<u16>,
}

impl<'a> Sprite<'a> {
    pub fn new<T: glium::backend::Facade>(mesh: &'a Mesh,
                                          material: &'a Material,
                                          display: &T)
                                          -> Sprite<'a> {
        let positions = glium::VertexBuffer::new(display, &mesh.verts).unwrap();
        let indices = glium::IndexBuffer::new(display,
                                              glium::index::PrimitiveType::TrianglesList,
                                              &mesh.indices)
                .unwrap();

        Sprite {
            mesh: mesh,
            material: material,
            position: [0.0, 0.0],
            rotation: 0.0,
            vertex_buffer: positions,
            indices_buffer: indices,
        }
    }

    pub fn render(&self,
                  target: &mut glium::Frame,
                  //program: &glium::Program,
                  view: &cgmath::Matrix4<f32>,
                  projection: &cgmath::Matrix4<f32>,
                  params: &glium::DrawParameters) {

        let model = cgmath::Matrix4::from_translation(cgmath::vec3(self.position[0],
                                                                   self.position[1],
                                                                   0.0)) *
                    cgmath::Matrix4::from_angle_z(cgmath::Deg(self.rotation));

        use glium::Surface;
        use cgmath::conv::*;
        target
            .draw(&self.vertex_buffer,
                  &self.indices_buffer,
                  &self.material.program,
                  &uniform! { modelView: array4x4(model * view),
                              perspective: array4x4(*projection),
                              diffuse_tex: &self.material.texture },
                  params)
            .unwrap();
    }
}
