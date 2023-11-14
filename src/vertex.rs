use std::fmt::Debug;

use glium::implement_vertex;

#[derive(Clone, Copy)]
pub struct Vertex {
    pub position: [f32; 3],
}

impl Vertex {
    #[inline(always)]
    pub const fn new(position: [f32; 3]) -> Self {
        Self { position }
    }
}

implement_vertex!(Vertex, position);
