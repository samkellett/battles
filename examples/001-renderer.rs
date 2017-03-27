#[macro_use]
extern crate glium;

use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn projection_matrix (fov: f32, aspect_ratio: f32, near: f32, far: f32) -> [[f32; 4]; 4] {
    let f = 1.0 / (fov / 2.0).tan();

    [
        [f *   aspect_ratio   ,    0.0,              0.0              ,   0.0],
        [         0.0         ,     f ,              0.0              ,   0.0],
        [         0.0         ,    0.0,  (far+near)/(far-near)    ,   1.0],
        [         0.0         ,    0.0, -(2.0*far*near)/(far-near),   0.0],
    ]
}

fn load_file (path: &Path) -> String {
    let mut file = match File::open(path) {
        Err(why) => panic!("Could not open {}: {}",
           path.display(), why.description()),
        Ok(file) => file,
};

    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}",
           path.display(), why.description()),
        Ok(_) => (),
    };

    s
}

// Required by glium
#[derive(Copy, Clone, Debug)]
struct Vertex {
    position: (f32, f32, f32),
}
implement_vertex!(Vertex, position);

// Required by glium
#[derive(Copy, Clone, Debug)]
struct Normal {
   normal: (f32, f32, f32), 
}
implement_vertex!(Normal, normal);

#[derive(Debug)]
struct Mesh {
    verts: Vec<Vertex>,
    norms: Vec<Normal>,
    indices: Vec<u16>,
}

impl Mesh {
    fn new (verts: Vec<Vertex>, norms: Vec<Normal>, indices: Vec<u16>) -> Mesh {
        Mesh {
            verts: verts,
            norms: norms,
            indices: indices,
        }
    }

    fn square(size: f32) -> Mesh {
        let vx = vec![-1.0, 1.0, 1.0, -1.0];
        let vy = vec![1.0, 1.0, -1.0, -1.0];

        let verts: Vec<Vertex> = vx.into_iter().zip(vy.into_iter())
          .map(|xy| Vertex{ position:(xy.0*size, xy.1*size, 0.0) })
          .collect();
        let norms: Vec<Normal> = vec![0,0,0,0].into_iter()
          .map(|_| Normal{ normal:(0.0, 0.0, 1.0) })
          .collect();

        let indices = vec![0, 1, 2, 2, 3, 0];

        Mesh::new(verts, norms, indices)
    }
}

#[derive(Debug)]
struct Sprite<'a> {
    mesh: &'a Mesh,
    vertex_buffer: glium::VertexBuffer<Vertex>,
    normals_buffer: glium::VertexBuffer<Normal>,
    indices_buffer: glium::IndexBuffer<u16>,
}

impl<'a> Sprite<'a> {
    fn new<T: glium::backend::Facade> (mesh: &'a Mesh, display: &T) -> Sprite<'a> {
        let positions = glium::VertexBuffer::new(display, &mesh.verts).unwrap();
        let normals = glium::VertexBuffer::new(display, &mesh.norms).unwrap();
        let indices = glium::IndexBuffer::new(display,
            glium::index::PrimitiveType::TrianglesList,
            &mesh.indices).unwrap();
        Sprite {
            mesh: mesh,
            vertex_buffer: positions,
            normals_buffer: normals,
            indices_buffer: indices,
        }
    }
}

fn main()
{
    let vertex_shader_src = load_file(Path::new("assets/shaders/basic.vert"));
    let fragment_shader_src = load_file(Path::new("assets/shaders/basic.frag"));

    use glium::{DisplayBuild, Surface};
    let display = glium::glutin::WindowBuilder::new()
                        .with_depth_buffer(24)
                        .build_glium().unwrap();

    let program = glium::Program::from_source(&display,
        &vertex_shader_src, &fragment_shader_src, None).unwrap();

    let mesh = Mesh::square(1.0);
    let sprite = Sprite::new(&mesh, &display);

    let params = glium::DrawParameters {
        depth: glium::Depth {
            test: glium::draw_parameters::DepthTest::IfLess,
            write: true,
            .. Default::default()
        },
        .. Default::default()
    };

    loop {
        let mut target = display.draw();
        target.clear_color_and_depth((0.0, 0.0, 1.0, 1.0), 1.0);

        let matrix = [
            [0.01, 0.0, 0.0, 0.0],
            [0.0, 0.01, 0.0, 0.0],
            [0.0, 0.0, 0.01, 0.0],
            [0.0, 0.0, 2.0, 1.0f32]
        ];

        let perspective = {
            let (width, height) = target.get_dimensions();
            projection_matrix(3.141592 / 3.0, height as f32 / width as f32,
                              0.1, 1024.0)
        };

        let light = [-1.0, 0.4, 0.9f32];

        target.draw((&sprite.vertex_buffer, &sprite.normals_buffer),
            &sprite.indices_buffer,
            &program,
            &uniform! { matrix: matrix, perspective: perspective,
            u_light: light },
            &params).unwrap();

        target.finish().unwrap();

        for ev in display.poll_events() {
            match ev {
                glium::glutin::Event::Closed => return,
                _ => ()
            }
        }
    }
}
