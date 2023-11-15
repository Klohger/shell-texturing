use glam::{Mat4, Quat, Vec3A};

#[derive(Default, Clone, Copy)]
pub struct Transform {
    pub translation: Vec3A,
    pub rotation: Quat,
}

impl Transform {
    pub fn new(translation: Vec3A, rotation: Quat) -> Self {
        Self {
            translation,
            rotation,
        }
    }

    pub fn mat(&self) -> Mat4 {
        Mat4::from_rotation_translation(self.rotation, self.translation.into())
    }
}
