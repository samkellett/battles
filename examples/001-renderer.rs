#[macro_use]
extern crate glium;
extern crate cgmath;

use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::f32;

fn load_file<P: AsRef<Path>>(path: P) -> Result<String, std::io::Error> {
    File::open(path).and_then(|mut file| {
                                  let mut contents = String::new();
                                  file.read_to_string(&mut contents).map(|_| contents)
                              })
}

// Required by glium
#[derive(Copy, Clone, Debug)]
struct Vertex {
    position: (f32, f32),
}
implement_vertex!(Vertex, position);

#[derive(Debug)]
struct Mesh {
    verts: Vec<Vertex>,
    indices: Vec<u16>,
}

impl Mesh {
    fn new(verts: Vec<Vertex>, indices: Vec<u16>) -> Mesh {
        Mesh {
            verts: verts,
            indices: indices,
        }
    }

    fn square(size: f32) -> Mesh {
        let vx = vec![-1.0, 1.0, 1.0, -1.0];
        let vy = vec![1.0, 1.0, -1.0, -1.0];

        let verts: Vec<Vertex> = vx.into_iter()
            .zip(vy.into_iter())
            .map(|xy| Vertex { position: (xy.0 * size, xy.1 * size) })
            .collect();

        let indices = vec![0, 1, 2, 2, 3, 0];

        Mesh::new(verts, indices)
    }
}

#[derive(Debug)]
struct Sprite<'a> {
    mesh: &'a Mesh,
    position: [f32; 2],
    rotation: f32,
    vertex_buffer: glium::VertexBuffer<Vertex>,
    indices_buffer: glium::IndexBuffer<u16>,
}

impl<'a> Sprite<'a> {
    fn new<T: glium::backend::Facade>(mesh: &'a Mesh, display: &T) -> Sprite<'a> {
        let positions = glium::VertexBuffer::new(display, &mesh.verts).unwrap();
        let indices = glium::IndexBuffer::new(display,
                                              glium::index::PrimitiveType::TrianglesList,
                                              &mesh.indices)
                .unwrap();

        Sprite {
            mesh: mesh,
            position: [0.0, 0.0],
            rotation: 0.0,
            vertex_buffer: positions,
            indices_buffer: indices,
        }
    }

    fn render(&self,
              target: &mut glium::Frame,
              program: &glium::Program,
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
                  program,
                  &uniform! { modelView: array4x4(model * view), perspective: array4x4(*projection), },
                  params)
            .unwrap();
    }
}

fn main() {
    use glium::{DisplayBuild, Surface};
    let display = glium::glutin::WindowBuilder::new()
        .with_depth_buffer(24)
        .build_glium()
        .unwrap();

    let program = glium::Program::from_source(&display,
                                              &load_file("assets/shaders/basic.vert").unwrap(),
                                              &load_file("assets/shaders/basic.frag").unwrap(),
                                              None)
            .unwrap();

    let mesh = Mesh::square(1.0);
    let mut sprite = Sprite::new(&mesh, &display);

    let params = glium::DrawParameters {
        depth: glium::Depth {
            test: glium::draw_parameters::DepthTest::IfLess,
            write: true,
            ..Default::default()
        },
        ..Default::default()
    };

    let timer = std::time::Instant::now();
    let mut previous_elapsed_time = 0.0;

    let mut rotation = 0.0;
    loop {
        // Work out the delta time
        let current_elapsed_time = timer.elapsed().as_secs() as f32 +
                                   (timer.elapsed().subsec_nanos() as f32 * 0.000_000_001);
        let delta_time = current_elapsed_time - previous_elapsed_time;
        previous_elapsed_time = current_elapsed_time;

        let mut target = display.draw();
        target.clear_color_and_depth((0.0, 0.0, 1.0, 1.0), 1.0);


        let perspective = {
            let (width, height) = target.get_dimensions();
            let aspect = height as f32 / width as f32;
            let span = 10.0; // World units between the left and right sides of the window
            cgmath::ortho(-span / 2.0,
                          span / 2.0,
                          -aspect * span / 2.0,
                          aspect * span / 2.0,
                          -1.0,
                          1.0)
        };

        rotation += delta_time * 90.0; // Rotate 90 degrees per second
        sprite.position = [2.5, 0.0];
        sprite.rotation = rotation;
        use cgmath::One;
        sprite.render(&mut target,
                      &program,
                      &cgmath::Matrix4::one(),
                      &perspective.into(),
                      &params);

        // Move up
        sprite.position = [-1.0, 2.0];
        sprite.rotation = rotation * 2.0;
        sprite.render(&mut target,
                      &program,
                      &cgmath::Matrix4::one(),
                      &perspective.into(),
                      &params);
        target.finish().unwrap();

        for ev in display.poll_events() {
            match ev {
                glium::glutin::Event::Closed => return,
                _ => (),
            }
        }
    }
}
