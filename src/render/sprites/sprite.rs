extern crate glium;
extern crate cgmath;

use render::sprites::mesh::Mesh;
use render::sprites::vertex::Vertex;
use render::materials::Material;
use render::transform::Transform;

#[derive(Debug)]
pub struct Sprite<'a> {
    mesh: Mesh,
    material: &'a Material<'a>,
    vertex_buffer: glium::VertexBuffer<Vertex>,
    indices_buffer: glium::IndexBuffer<u16>,
}

impl<'a> Sprite<'a> {
    pub fn from_mesh<T: glium::backend::Facade>(mesh: Mesh,
                                                material: &'a Material<'a>,
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
            vertex_buffer: positions,
            indices_buffer: indices,
        }
    }

    pub fn from_material<T: glium::backend::Facade>(material: &'a Material<'a>,
                                                    display: &T)
                                                    -> Sprite<'a> {

        let pdim = {
            let width = material.texture.texture.get_width() as i32;
            let height = material.texture.texture.get_height().unwrap() as i32;
            cgmath::Vector2::new(width, height)
        };
        let mesh = Mesh::square_slice(1.0, material.texture.origin, material.texture.dimensions, &pdim);

        let positions = glium::VertexBuffer::new(display, &mesh.verts).unwrap();
        let indices = glium::IndexBuffer::new(display,
                                              glium::index::PrimitiveType::TrianglesList,
                                              &mesh.indices)
                .unwrap();


        Sprite {
            mesh: mesh,
            material: material,
            vertex_buffer: positions,
            indices_buffer: indices,
        }
    }

    pub fn render(&self,
                  target: &mut glium::Frame,
                  //program: &glium::Program,
                  transform: &Transform,
                  projection: &cgmath::Matrix4<f32>,
                  params: &glium::DrawParameters) {

        use glium::Surface;
        use cgmath::conv::*;
        target
            .draw(&self.vertex_buffer,
                  &self.indices_buffer,
                  &self.material.program,
                  &uniform! { modelView: array4x4(transform.matrix),
                              perspective: array4x4(*projection),
                              diffuse_tex: self.material.texture.texture },
                  params)
            .unwrap();
    }
}
