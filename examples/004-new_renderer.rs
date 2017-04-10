#[macro_use]
extern crate glium;

use glium::backend::glutin_backend as glutin;
use glium::Surface;

#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 2],
}
implement_vertex!(Vertex, position);

struct Window {
    facade: glutin::GlutinFacade,
}

impl Window {
    fn new () -> Window {
        use glium::DisplayBuild;
        let facade = glium::glutin::WindowBuilder::new().build_glium().unwrap();

        Window {
            facade: facade,
        }
    }

    fn create_vertex_buffer (&self, vertices: &Vec<Vertex>) -> glium::VertexBuffer<Vertex> {
        glium::VertexBuffer::new(&self.facade, vertices).unwrap()
    }
    
    fn create_shader_program (&self, vertex_source: &str, fragment_source: &str) -> glium::Program {
        glium::Program::from_source(&self.facade,
                                    vertex_source,
                                    fragment_source, None).unwrap()
    }
    
    fn draw<F> (&self, draw_function: F) where F: FnOnce(&mut glium::Frame) {
        let mut frame = self.facade.draw();
        frame.clear_color(0.0, 0.0, 1.0, 1.0);

        draw_function(&mut frame);

        frame.finish().unwrap();
    }
}

struct Sprite {
    vertex_buffer: glium::VertexBuffer<Vertex>,
    index_buffer: glium::index::NoIndices,
}

impl Sprite {
    fn new (window: &Window) -> Sprite {
        let vertex_buffer = {
            let vertex1 = Vertex { position: [-0.5, 0.5] };
            let vertex2 = Vertex { position: [ 0.5,  0.5] };
            let vertex3 = Vertex { position: [ 0.5, -0.5] };
            let vertex4 = Vertex { position: [ -0.5, -0.5] };
            let shape = vec![vertex1, vertex2, vertex3, vertex3, vertex4, vertex1];

            window.create_vertex_buffer(&shape)
        };
        let index_buffer = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

        Sprite {
            vertex_buffer: vertex_buffer,
            index_buffer: index_buffer,
        }
    }

    fn draw (&self, frame: &mut glium::Frame, program: &glium::Program) {
        frame.draw(&self.vertex_buffer, &self.index_buffer, program, &glium::uniforms::EmptyUniforms, &Default::default()).unwrap();
    }
}

fn main() {
    let window = Window::new();

    let vertex_shader_src = r#"
        #version 140

        in vec2 position;

        void main() {
            gl_Position = vec4(position, 0.0, 1.0);
        }
    "#;

    let fragment_shader_src = r#"
        #version 140

        out vec4 color;

        void main() {
            color = vec4(1.0, 0.0, 0.0, 1.0);
        }
    "#;
    
    let program = window.create_shader_program(&vertex_shader_src, &fragment_shader_src);

    let sprite = Sprite::new(&window);

    loop {
        window.draw(|mut frame| {
            sprite.draw(&mut frame, &program);
        });

        for ev in window.facade.poll_events() {
            match ev {
                glium::glutin::Event::Closed => return,
                _ => ()
            }
        }
    }
}
