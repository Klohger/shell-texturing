use glium::backend::Facade;
use glium::{
    backend::glutin::SimpleWindowBuilder, implement_vertex, index::PrimitiveType, Surface,
    VertexBuffer,
};
use glium::{uniforms, IndexBuffer, Program as ShaderProgram};
use winit::event_loop::EventLoopBuilder;
use winit::window::WindowBuilder;
#[derive(Clone, Copy)]
pub struct Vertex {
    pub position: [f32; 3],
}
implement_vertex!(Vertex, position);
pub struct Mesh {
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u32>,
    pub primitive_type: PrimitiveType,
}

impl Mesh {
    pub fn vertex_buffer<F: Facade>(
        &self,
        facade: &F,
    ) -> Result<VertexBuffer<Vertex>, glium::vertex::BufferCreationError> {
        VertexBuffer::immutable(facade, &self.vertices)
    }
    pub fn index_buffer<F: Facade>(
        &self,
        facade: &F,
    ) -> Result<IndexBuffer<u32>, glium::index::BufferCreationError> {
        IndexBuffer::immutable(facade, self.primitive_type, &self.indices)
    }
}

fn cam_shells(shell_resolutions: Vec<[usize; 2]>, z_multiplier: f32) -> Vec<Mesh> {
    let mut vec = Vec::with_capacity(shell_resolutions.len());
    for (z, shell_resolution) in
        shell_resolutions
            .into_iter()
            .enumerate()
            .map(|(shell_index, shell_resolution)| {
                let mut z = shell_index as f32;
                z *= z_multiplier;
                (z * z * z * z, shell_resolution)
            })
    {
        vec.push(Mesh {
            vertices: {
                // fill vec with vertices
                (0..(shell_resolution[0] * shell_resolution[1]))
                    .into_iter()
                    .map(|vertex_index| Vertex {
                        position: [
                            (vertex_index % shell_resolution[0]) as f32,
                            (vertex_index / shell_resolution[0]) as f32,
                            z,
                        ],
                    })
                    .collect::<Vec<_>>()
            },
            indices: {
                // fill vec with indices
                let indices = vec![0; shell_resolution[0] * shell_resolution[1]];

                indices
            },
            primitive_type: PrimitiveType::TrianglesList,
        })
    }

    return vec;
}

const VERTEX_SHADER_SRC: &str = r#"
    ##version 140

    in vec3 position;

    void main() {
        gl_Position = vec4(position, 1.0);
    }
"#;
const FRAGMENT_SHADER_SRC: &str = r#"
    ##version 140

    out vec4 color;

    void main() {
        color = vec4(position, 1.0);
    }
"#;

fn main() {
    let event_loop = EventLoopBuilder::new().build();
    let (window, display) = SimpleWindowBuilder::new()
        .set_window_builder(
            WindowBuilder::new()
                .with_title("shell texture swag wow™")
                .with_visible(false),
        )
        .build(&event_loop);
    let cam_shells = cam_shells(
        (1..=16)
            .rev()
            .map(|res| res * res)
            .map(|res| [res, res])
            .collect::<Vec<_>>(),
        0.1875,
    )
    .into_iter()
    .map(|mesh| {
        (
            mesh.vertex_buffer(&display).unwrap(),
            mesh.index_buffer(&display).unwrap(),
        )
    })
    .collect::<Vec<_>>();
    let cam_shell_program =
        ShaderProgram::from_source(&display, VERTEX_SHADER_SRC, FRAGMENT_SHADER_SRC, None).unwrap();

    event_loop.run(move |ev, _, control_flow| match ev {
        winit::event::Event::RedrawRequested(window_id) if window.id() == window_id => {
            let mut frame = display.draw();
            frame.clear_color(0.0, 0.0, 0.0, 1.0);
            for (vertex_buffer, index_buffer) in &cam_shells {
                frame
                    .draw(
                        vertex_buffer,
                        index_buffer,
                        &cam_shell_program,
                        &uniforms::EmptyUniforms,
                        &Default::default(),
                    )
                    .unwrap();
            }
            frame.finish().unwrap();
        }
        winit::event::Event::WindowEvent { event, .. } => match event {
            winit::event::WindowEvent::CloseRequested => {
                *control_flow = winit::event_loop::ControlFlow::Exit;
            }
            _ => (),
        },
        _ => (),
    });
}
