use crate::vertex::Vertex;
use glium::{
    backend::Facade,
    index::{BufferCreationError as IndexBufferCreationError, PrimitiveType},
    vertex::BufferCreationError as VertexBufferCreationError,
    IndexBuffer, VertexBuffer,
};

pub struct Mesh {
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u32>,
    pub primitive_type: PrimitiveType,
}

impl Mesh {
    pub fn vertex_buffer<F: Facade>(
        &self,
        facade: &F,
    ) -> Result<VertexBuffer<Vertex>, VertexBufferCreationError> {
        VertexBuffer::new(facade, &self.vertices)
    }
    pub fn index_buffer<F: Facade>(
        &self,
        facade: &F,
    ) -> Result<IndexBuffer<u32>, IndexBufferCreationError> {
        IndexBuffer::immutable(facade, self.primitive_type, &self.indices)
    }
}
