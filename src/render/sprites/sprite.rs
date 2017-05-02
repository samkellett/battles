extern crate glium;
extern crate cgmath;

use render::{GliumWindow, Window};
use render::sprites::mesh::Mesh;
use render::sprites::vertex::Vertex;
use render::textures::{TextureSource, Texture};
use render::transform::Transform;

use cgmath::Matrix4;
use cgmath::conv::array4x4;

use glium::{VertexBuffer, IndexBuffer, Frame, DrawParameters, Surface};

use std::fs::File;
use std::path::Path;

// on-disk representation of a shader program.
#[derive(Debug, Deserialize)]
pub struct ProgramSource {
    pub vertex_shader: String,
    pub fragment_shader: String,
}

// on-disk representation of a sprite.
#[derive(Debug, Deserialize)]
pub struct SpriteSource {
    pub name: String,
    pub texture: TextureSource,
    pub program: ProgramSource,
}

pub struct Sprite {
    pub name: String,
    pub mesh: Mesh,
    pub texture: Texture,
    pub program: glium::Program,
    vertex_buffer: VertexBuffer<Vertex>,
    indices_buffer: IndexBuffer<u16>,
}

// Load a file into a string.
fn file_get_contents<P>(path: P) -> String
    where P: AsRef<Path>
{
    use std::io::Read;

    let mut file = File::open(path).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    contents
}

impl Sprite {
    pub fn from_source(window: &GliumWindow, source: SpriteSource) -> Sprite
    {
        let name = source.name;
        let mesh = Mesh::square(1.0);
        let vertex_buffer = window.create_vertex_buffer(&mesh.verts);
        let indices_buffer = window.create_index_buffer(&mesh.indices);
        let texture = Texture::from_source(&window.facade, source.texture);
        let program = {
            let vertex_shader = file_get_contents(source.program.vertex_shader);
            let fragment_shader = file_get_contents(source.program.fragment_shader);

            glium::Program::from_source(&window.facade, &vertex_shader, &fragment_shader, None)
                .unwrap()
        };

        Sprite { name, mesh, texture, program, vertex_buffer, indices_buffer }
    }

    pub fn from_config<I>(window: &GliumWindow, sources: I) -> Vec<Sprite>
        where I: Iterator<Item = SpriteSource>
    {
        let mut sprites = Vec::new();

        for source in sources {
            sprites.push(Sprite::from_source(window, source))
        }

        sprites
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
