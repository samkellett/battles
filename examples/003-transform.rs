extern crate cgmath;

#[derive(Debug)]
struct Transform {
    matrix: cgmath::Matrix4<f32>,
}

enum Rotation {
    Deg(f32),
    Rad(f32),
}

impl Transform {
    fn new () -> Transform {
        use cgmath::One;
        Transform {
            matrix: cgmath::Matrix4::one(),
        }
    }

    fn translate (&mut self, x: f32, y: f32, z: f32) {
        let v = cgmath::Vector3::new(x, y, z);
        self.matrix = self.matrix * cgmath::Matrix4::from_translation(v);
    }

    fn rotate_z (&mut self, rotation: Rotation) {
        let cg_rotation = match rotation {
            Rotation::Deg(d) => cgmath::Deg(d).into(),
            Rotation::Rad(r) => cgmath::Rad(r),
        };

        self.matrix = self.matrix * cgmath::Matrix4::from_angle_z(cg_rotation);
    }
}

fn main() {
    let mut transform = Transform::new();
    println!("{:?}", transform);
    transform.translate(3.0, 0.0, 0.0);
    println!("{:?}", transform);
}
