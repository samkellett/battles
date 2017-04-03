extern crate cgmath;

use render::sprites::vertex::Vertex;

#[derive(Debug)]
pub struct Mesh {
    pub verts: Vec<Vertex>,
    pub indices: Vec<u16>,
}

impl Mesh {
    fn new(verts: Vec<Vertex>, indices: Vec<u16>) -> Mesh {
        Mesh {
            verts: verts,
            indices: indices,
        }
    }

    pub fn square(size: f32) -> Mesh {
        fn clamp(x: f32) -> f32 {
            if x < 0.0 { 0.0 } else { x }
        }

        let vx = vec![-1.0, 1.0, 1.0, -1.0];
        let vy = vec![1.0, 1.0, -1.0, -1.0];

        let verts: Vec<Vertex> = vx.into_iter()
            .zip(vy.into_iter())
            .map(|xy| {
                     Vertex {
                         position: (xy.0 * size, xy.1 * size),
                         tex_coords: (clamp(xy.0), clamp(xy.1)),
                     }
                 })
            .collect();

        let indices = vec![0, 1, 2, 2, 3, 0];

        Mesh::new(verts, indices)
    }

    pub fn square_slice(size: f32,
                        origin: &cgmath::Vector2<i32>,
                        dimensions: &cgmath::Vector2<i32>,
                        parent_dimensions: &cgmath::Vector2<i32>)
                        -> Mesh {

        let scaled_origin = (origin.x as f32 / parent_dimensions.x as f32,
                             origin.y as f32 / parent_dimensions.y as f32);
        let scaled_dimensions = (dimensions.x as f32 / parent_dimensions.x as f32,
                                 dimensions.y as f32 / parent_dimensions.y as f32);
        let top_right = (scaled_origin.0 + scaled_dimensions.0,
                         scaled_origin.1 + scaled_dimensions.0);

        let mut verts: Vec<Vertex> = Vec::new();
        verts.push(Vertex {
                       position: (-1.0, 1.0),
                       tex_coords: (scaled_origin.0, top_right.1),
                   });
        verts.push(Vertex {
                       position: (1.0, 1.0),
                       tex_coords: (top_right.0, top_right.1),
                   });
        verts.push(Vertex {
                       position: (1.0, -1.0),
                       tex_coords: (top_right.0, scaled_origin.1),
                   });
        verts.push(Vertex {
                       position: (-1.0, -1.0),
                       tex_coords: (scaled_origin.0, scaled_origin.1),
                   });

        let indices = vec![0, 1, 2, 2, 3, 0];

        Mesh::new(verts, indices)

    }
}
