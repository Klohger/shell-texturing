use glam::Mat4;

#[derive(Default)]
pub(crate) struct Transform {
    pub(crate) translation: glam::Vec3A,
    pub(crate) rotation: glam::Quat,
}

impl Transform {
    pub(crate) fn mat(&self) -> Mat4 {
        Mat4::from_rotation_translation(self.rotation, self.translation.into())
    }
}
