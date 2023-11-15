use crate::{transform::Transform, input::KeyBoardState};
use core::f32;
use glam::Mat4;
use glium::{glutin::surface::WindowSurface, Display};
use std::{cell::RefCell, rc::Rc, time};
use winit::{dpi::PhysicalSize, window::Window};
pub struct AppState {
    pub window: Window,
    pub display: Display<WindowSurface>,
    pub camera: Camera,
    pub last_draw_time: Option<time::Instant>,
    pub keyboard_state : KeyBoardState,
}



pub enum Scene {

}
pub trait SceneTrait {
    const INITIAL_CAMERA_TRANSFORM : Transform;
}

impl AppState {
    pub const CAM_FOV: f32 = f32::consts::PI / 2.0;
    pub const CAM_NEAR: f32 = 0.01;
    pub const CAM_FAR: f32 = 1.01;
    pub fn new(window: Window, display: Display<WindowSurface>) -> Self {
        let aspect_ratio = {
            let size = window.inner_size();
            size.height as f32 / size.width as f32
        };
        Self {
            window,
            display,
            camera: Camera::new(Transform::default(), aspect_ratio),
            last_draw_time: Default::default(),
            keyboard_state: Default::default(),
        }
    }
    pub fn new_ptr(window: Window, display: Display<WindowSurface>) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self::new(window,display)))
    }
}

pub struct Camera {
    pub transfrom: Transform,
    world_projection: Mat4,
    inv_world_projection: Mat4,
}

impl Camera {
    pub const CAM_FOV: f32 = f32::consts::PI / 2.0;
    pub const CAM_NEAR: f32 = 0.01;
    pub const CAM_FAR: f32 = 1.01;
    pub fn new(transfrom: Transform, aspect_ratio: f32) -> Self {
        let world_projection = Mat4::perspective_lh(
            AppState::CAM_FOV,
            aspect_ratio,
            AppState::CAM_NEAR,
            AppState::CAM_FAR,
        );
        let inv_world_projection = world_projection.inverse();
        Self {
            transfrom: transfrom,
            world_projection,
            inv_world_projection,
        }
    }
    pub fn world_projection(&self) -> &Mat4 {
        &self.world_projection
    }
    pub fn update_aspect_ratio(&mut self, size: PhysicalSize<u32>) {
        self.world_projection = Mat4::perspective_lh(
            AppState::CAM_FOV,
            size.height as f32 / size.width as f32,
            AppState::CAM_NEAR,
            AppState::CAM_FAR,
        );
        self.inv_world_projection = self.world_projection.inverse();
    }
}
