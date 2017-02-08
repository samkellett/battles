#[macro_use]
extern crate glium;

use std::path::Path;

#[path = "../assets/teapot.rs"]
mod teapot;

mod battles;

fn main() {
    use glium::{DisplayBuild, Surface};
    use glium::index::PrimitiveType;

    let display = glium::glutin::WindowBuilder::new()
        .with_depth_buffer(24)
        .build_glium()
        .unwrap();

    let positions = glium::VertexBuffer::new(&display, &teapot::VERTICES).unwrap();
    let normals = glium::VertexBuffer::new(&display, &teapot::NORMALS).unwrap();
    let indices = glium::IndexBuffer::new(&display, PrimitiveType::TrianglesList, &teapot::INDICES).unwrap();

    let vertex_shader_src = battles::file_get_contents(&Path::new("assets/simple.vert"));
    let fragment_shader_src = battles::file_get_contents(&Path::new("assets/simple.frag"));

    let program = glium::Program::from_source(&display, &vertex_shader_src, &fragment_shader_src, None).unwrap();

    loop {
        let mut target = display.draw();
        target.clear_color_and_depth((0.99, 0.83, 0.11, 1.0), 1.0);

        let model = [
            [0.01, 0.0, 0.0, 0.0],
            [0.0, 0.01, 0.0, 0.0],
            [0.0, 0.0, 0.01, 0.0],
            [0.0, 0.0, 2.0, 1.0f32],
        ];

        let view = battles::view_matrix(&[2.0, 1.0, 1.0], &[-2.0, -1.0, 1.0], &[0.0, 1.0, 0.0]);

        let perspective = {
            let (width, height) = target.get_dimensions();
            let aspect_ratio = height as f32 / width as f32;

            let fov: f32 = 3.141592 / 3.0;
            let zfar = 1024.0;
            let znear = 0.1;

            let f = 1.0 / (fov / 2.0).tan();

            [
                [f * aspect_ratio, 0.0, 0.0, 0.0],
                [0.0, f, 0.0, 0.0],
                [0.0, 0.0, (zfar + znear) / (zfar - znear), 1.0],
                [0.0, 0.0, -(2.0 * zfar * znear) / (zfar - znear), 0.0],
            ]
        };

        let light = [1.4, 0.4, -0.7f32];

        let params = glium::DrawParameters {
            depth: glium::Depth {
                test: glium::draw_parameters::DepthTest::IfLess,
                write: true,
                .. Default::default()
            },
            .. Default::default()
        };

        target.draw((&positions, &normals), &indices, &program,
                    &uniform! { model: model, view: view, perspective: perspective, u_light: light },
                    &params).unwrap();
        target.finish().unwrap();

        // listing the events produced by the window and waiting to be recv'ed.
        for ev in display.poll_events() {
            match ev {
                // the window has been closed by the user.
                glium::glutin::Event::Closed => return,
                _ => ()
            }
        }
    }
}

