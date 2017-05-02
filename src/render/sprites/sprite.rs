extern crate glium;
extern crate cgmath;

use render::sprites::mesh::Mesh;
use render::textures::{TextureSource, Texture};
use render::transform::Transform;

use glium::{VertexBuffer, IndexBuffer, Frame, DrawParameters, Surface};

use cgmath::Matrix4;
use cgmath::conv::array4x4;

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
    pub fn from_source<D>(display: &D, source: SpriteSource) -> Sprite
        where D: glium::backend::Facade
    {
        let name = source.name;
        let mesh = Mesh::square(1.0);
        let texture = Texture::from_source(display, source.texture);
        let program = {
            let vertex_shader = file_get_contents(source.program.vertex_shader);
            let fragment_shader = file_get_contents(source.program.fragment_shader);

            glium::Program::from_source(display, &vertex_shader, &fragment_shader, None)
                .unwrap()
        };

        Sprite { name, mesh, texture, program }
    }

    pub fn from_config<D, I>(display: &D, sources: I) -> Vec<Sprite>
        where D: glium::backend::Facade,
              I: Iterator<Item = SpriteSource>
    {
        let mut sprites = Vec::new();

        for source in sources {
            sprites.push(Sprite::from_source(display, source))
        }

        sprites
    }
}

