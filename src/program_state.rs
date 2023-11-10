use crate::transform::Transform;
use core::f32;
use glam::Mat4;
use glium::{glutin::surface::WindowSurface, Display};
use std::{cell::RefCell, rc::Rc};
use winit::{dpi::PhysicalSize, window::Window};

#[derive(Clone)]
pub(crate) struct ProgramState {
    pub(crate) window: Rc<RefCell<Window>>,
    pub(crate) display: Rc<RefCell<Display<WindowSurface>>>,
    pub(crate) world_projection: Rc<RefCell<(Mat4, Mat4)>>,
    pub(crate) camera: Rc<RefCell<Transform>>,
}

impl ProgramState {
    pub(crate) const CAM_FOV: f32 = f32::consts::PI / 2.0;
    pub(crate) const CAM_NEAR: f32 = 0.01;
    pub(crate) const CAM_FAR: f32 = 1.01;
    pub(crate) fn new(window: Window, display: Display<WindowSurface>) -> Self {
        let aspect_ratio = {
            let size = window.inner_size();
            size.width as f32 / size.height as f32
        };
        let world_projection = glam::Mat4::perspective_lh(
            ProgramState::CAM_FOV,
            aspect_ratio,
            ProgramState::CAM_NEAR,
            ProgramState::CAM_FAR,
        );
        let inv_world_projection = world_projection.inverse();
        Self {
            window: Rc::new(RefCell::new(window)),
            display: Rc::new(RefCell::new(display)),
            world_projection: Rc::new(RefCell::new((world_projection, inv_world_projection))),
            camera: Rc::new(RefCell::new(Transform::default())),
        }
    }
    pub(crate) fn update_aspect_ratio(&self, size: PhysicalSize<u32>) {
        let mut world_projection = self.world_projection.borrow_mut();
        world_projection.0 = glam::Mat4::perspective_lh(
            ProgramState::CAM_FOV,
            size.width as f32 / size.height as f32,
            ProgramState::CAM_NEAR,
            ProgramState::CAM_FAR,
        );
        world_projection.1 = world_projection.0.inverse();
    }
}
