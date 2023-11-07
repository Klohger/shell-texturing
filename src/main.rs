use glium::backend::Facade;
use glium::{
    backend::glutin::SimpleWindowBuilder, implement_vertex, index::PrimitiveType, Surface,
    VertexBuffer,
};
use glium::{uniforms, IndexBuffer, Program as ShaderProgram};
use winit::event_loop::EventLoopBuilder;
#[derive(Clone, Copy)]
pub struct Vertex {
    pub position: [f32; 3],
}
implement_vertex!(Vertex, position);
pub struct Mesh {
    pub vertices: VertexBuffer<Vertex>,
    pub indices: IndexBuffer<u32>,
}

fn cam_shells<F: Facade>(facade: &F, shell_resolutions: Vec<[usize; 2]>) -> Vec<Mesh> {
    let mut vec = Vec::with_capacity(shell_resolutions.len());
    for (shell_index, shell_resolution) in shell_resolutions.into_iter().enumerate() {
        vec.push(Mesh {
            vertices: VertexBuffer::immutable(facade, {
                // fill vec with vertices
                let mut vertices = vec![
                    Vertex {
                        position: [0.0, 0.0, 0.0]
                    };
                    shell_resolution[0] * shell_resolution[1]
                ];
                vertices
                    .iter_mut()
                    .enumerate()
                    .for_each(|(vertex_index, vertex)| {});

                &vertices
            })
            .unwrap(),
            indices: IndexBuffer::new(facade, PrimitiveType::TrianglesList, {
                // fill vec with indices
                let indices = vec![0; shell_resolution[0] * shell_resolution[1]];

                &indices
            })
            .unwrap(),
        })
    }

    return vec;
}

fn gen_plane() -> Mesh {
    todo!()
}

const vertex_shader_src: &str = r#"
    ##version 140

    in vec3 position;

    void main() {
        gl_Position = vec4(position, 1.0);
    }
"#;
const fragment_shader_src: &str = r#"
    ##version 140

    out vec4 color;

    void main() {
        color = vec4(position, 1.0);
    }
"#;

fn main() {
    let event_loop = EventLoopBuilder::new().build();
    let (window, display) = SimpleWindowBuilder::new()
        .with_title("shell texture swag wow™")
        .build(&event_loop);
    let cam_shells = cam_shells(
        &display,
        (1..=16)
            .rev()
            .map(|res| res * res)
            .map(|res| [res, res])
            .collect::<Vec<_>>(),
    );
    let cam_shell_program =
        ShaderProgram::from_source(&display, vertex_shader_src, fragment_shader_src, None).unwrap();

    event_loop.run(move |ev, _, control_flow| match ev {
        winit::event::Event::RedrawRequested(window_id) if window.id() == window_id => {
            let mut frame = display.draw();
            frame.clear_color(0.0, 0.0, 0.0, 1.0);
            for cam_shell in &cam_shells {
                frame
                    .draw(
                        &cam_shell.vertices,
                        &cam_shell.indices,
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
