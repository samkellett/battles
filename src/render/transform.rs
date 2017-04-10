extern crate cgmath;

#[allow(dead_code)]
pub enum Rotation {
    Deg(f32),
    Rad(f32),
}

#[derive(Debug)]
pub struct Transform {
    pub matrix: cgmath::Matrix4<f32>,
}

#[allow(dead_code)]
impl Transform {
    pub fn new() -> Transform {
        use cgmath::One;
        Transform { matrix: cgmath::Matrix4::one() }
    }

    pub fn translate(&mut self, x: f32, y: f32, z: f32) {
        let v = cgmath::Vector3::new(x, y, z);
        self.matrix = self.matrix * cgmath::Matrix4::from_translation(v);
    }

    pub fn rotate_z(&mut self, rotation: Rotation) {
        let cg_rotation = match rotation {
            Rotation::Deg(d) => cgmath::Deg(d).into(),
            Rotation::Rad(r) => cgmath::Rad(r),
        };

        self.matrix = self.matrix * cgmath::Matrix4::from_angle_z(cg_rotation);
    }
}
