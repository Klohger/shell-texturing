use crate::vertex::Vertex;
use glium::index::PrimitiveType;

pub struct Mesh {
    pub vertices: Vec<Vertex>,
    pub indices: Option<Vec<u32>>,
    pub primitive_type: PrimitiveType,
}
/*
impl Mesh {
    pub fn vertex_buffer<F: Facade>(
        &self,
        facade: &F,
    ) -> Result<VertexBuffer<Vertex>, VertexBufferCreationError> {
        VertexBuffer::new(facade, &self.vertices)
    }
    pub fn index_buffer<'a, F: Facade>(
        &'a self,
        facade: &F,
    ) -> Result<IndicesSource<'a>, IndexBufferCreationError> {
        if let Some(indices) = &self.indices {
            if !self.primitive_type.is_supported(facade) {
                return Err(IndexBufferCreationError::PrimitiveTypeNotSupported);
            }

            if !u32::is_supported(facade) {
                return Err(IndexBufferCreationError::IndexTypeNotSupported);
            }
            let buffer = Buffer::new(
                facade,
                &indices,
                BufferType::ElementArrayBuffer,
                BufferMode::Immutable,
            )?;

            Ok(IndicesSource::IndexBuffer {
                buffer: buffer.as_slice_any(),
                data_type: glium::index::IndexType::U32,
                primitives: self.primitive_type,
            })
        } else {
            Ok(IndicesSource::NoIndices {
                primitives: self.primitive_type,
            })
        }
    }
}
*/
