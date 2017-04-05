#[macro_use]
extern crate glium;
extern crate cgmath;

#[derive(Copy, Clone, Debug)]
pub struct Vertex {
    pub position: (f32, f32),
}
implement_vertex!(Vertex, position);

struct Renderer<'a> {
    display: glium::backend::glutin_backend::GlutinFacade,
    draw_parameters: glium::DrawParameters<'a>,
    frame: Option<Box<glium::Frame>>,
}

impl<'a> Renderer<'a> {
    fn new () -> Renderer<'a> {
        use glium::DisplayBuild;

        let display = glium::glutin::WindowBuilder::new()
            .build_glium()
            .unwrap();

        let draw_parameters = glium::DrawParameters {
            depth: glium::Depth {
                test: glium::draw_parameters::DepthTest::IfLess,
                write: true,
                ..Default::default()
            },
            ..Default::default()
        };

        Renderer {
            display: display,
            draw_parameters: draw_parameters,
            frame: None::<Box<glium::Frame>>,
        }
    }

    fn start_frame (&mut self) {
        use glium::Surface;
        self.frame = Some(Box::new(self.display.draw()));
        match self.frame {
            Some(ref mut frame) => frame.clear_color_and_depth((0.99, 0.83, 0.11, 1.0), 1.0),
            None => println!("Failed to get a frame object"),
        };
    }

    fn end_frame (&mut self) {
        use std::mem;
        let frame = mem::replace(&mut self.frame, None::<Box<glium::Frame>>);
        match frame {
            Some(frame) => frame.finish().unwrap(),
            None => (),
        };
    }

    fn generate_vertex_buffer(&self, vertices: &Vec<Vertex>)
        -> glium::VertexBuffer<Vertex> {
        glium::VertexBuffer::new(&self.display, &vertices).unwrap()
    }

    fn generate_index_buffer(&self, indices: &Vec<u16>)
        -> glium::IndexBuffer<u16> {
        glium::IndexBuffer::new(&self.display,
                                glium::index::PrimitiveType::TrianglesList,
                                &indices).unwrap()
    }

    fn generate_shader_program (&self,
                                vertex_shader_source: &str,
                                fragment_shader_source: &str)
        -> glium::Program {
        glium::Program::from_source(&self.display,
                                                  vertex_shader_source,
                                                  fragment_shader_source,
                                                  None)
            .unwrap()
    }

    fn draw (&mut self,
             vertices: &Vec<Vertex>,
             indices: &Vec<u16>,
             program: &glium::Program,
             model_view: &cgmath::Matrix4<f32>,
             perspective: &cgmath::Matrix4<f32>) {
        use glium::Surface;

        let vertex_buffer = self.generate_vertex_buffer(vertices);
        let index_buffer = self.generate_index_buffer(indices);

        use cgmath::conv::*;
        match self.frame {
            Some(ref mut frame) => frame.draw(&vertex_buffer,
                                   &index_buffer,
                                   program,
                              &uniform! { modelView: array4x4(*model_view),
                                          perspective: array4x4(*perspective), },
                              &self.draw_parameters)
                        .unwrap(),
            None => ()
        };
    }
}

fn main() {
    let mut renderer = Renderer::new();

    let verts = vec![
        Vertex { position: (-0.1, 0.1), },
        Vertex { position: (0.1, 0.1), },
        Vertex { position: (0.1, -0.1), },
        Vertex { position: (-0.1, -0.1), },
    ];
    let indices = vec![0, 1, 2, 2, 3, 0];
    let program =  {
        let vertex_shader_source = include_str!("../assets/shaders/nr.vert");
        let fragment_shader_source = include_str!("../assets/shaders/nr.frag");
        renderer.generate_shader_program(vertex_shader_source,
                                         fragment_shader_source)
    };

    use cgmath::One;
    let matrix = cgmath::Matrix4::one();

    loop {
        renderer.start_frame();

        renderer.draw(&verts, &indices, &program, &matrix, &matrix);

        renderer.end_frame();

        for event in renderer.display.poll_events() {
            match event {
                // The window has been closed.
                glium::glutin::Event::Closed => return,
                _ => (),
            }
        }
    }
}
