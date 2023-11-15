use crate::{program_state::AppState, vertex::Vertex};
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

pub fn gen_cam_shells(shell_resolutions: &[[usize; 2]], z_pow: i32) -> Vec<Mesh> {
    let mut meshes = Vec::with_capacity(shell_resolutions.len());
    for (z, shell_resolution) in {
        let num_resolutions = (shell_resolutions.len() - 1) as f32;
        shell_resolutions
            .into_iter()
            .enumerate()
            .map(move |(shell_index, shell_resolution)| {
                let z = if num_resolutions == 0.0 {
                    0.0
                } else {
                    (shell_index as f32 / num_resolutions).powi(z_pow)
                };
                (z + AppState::CAM_NEAR, shell_resolution)
            })
    } {
        let verts = (0..(shell_resolution[0] + 1) * (shell_resolution[1] + 1))
            .into_iter()
            .map(|vertex_index| {
                Vertex::new(
                    ((vertex_index % (shell_resolution[0] + 1)) * 2) as f32
                        / (shell_resolution[0] as f32)
                        - 1.0,
                    ((vertex_index / (shell_resolution[0] + 1)) * 2) as f32
                        / (shell_resolution[1] as f32)
                        - 1.0,
                    z,
                )
            })
            .collect::<Vec<_>>();

        let mut indices = Vec::with_capacity(shell_resolution[0] * shell_resolution[1] * 3 * 2);
        for i in 0..((shell_resolution[0] + 1) * (shell_resolution[1] + 1)) {
            // say no thank you to right side vertices
            if (i % (shell_resolution[0] + 1)) == (shell_resolution[0]) {
                //println!("x_fuck you mr vertex");
                continue;
            }
            // say no thank you to upper side vertices
            if (i / (shell_resolution[0] + 1)) == (shell_resolution[1]) {
                //println!("y_fuck you mr vertexices");
                break;
            }

            // first tri
            // 2--0
            // | /
            // |/
            // 1
            indices.push(i as u32 + (shell_resolution[0] + 1) as u32 + 1);
            indices.push(i as u32);
            indices.push(i as u32 + (shell_resolution[0] + 1) as u32);

            // second tri
            //    1
            //   /|
            //  / |
            // 0--2
            indices.push(i as u32);
            indices.push(i as u32 + (shell_resolution[0] + 1) as u32 + 1);
            indices.push(i as u32 + 1);
        }

        meshes.push(Mesh {
            vertices: verts,
            indices: indices,
            primitive_type: PrimitiveType::TrianglesList,
        })
    }

    return meshes;
}
