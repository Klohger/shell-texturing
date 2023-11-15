use crate::{mesh::gen_cam_shells, program_state::AppState};
use glium::{
    backend::glutin::SimpleWindowBuilder,
    draw_parameters::{ProvokingVertex, Stencil},
    uniforms::UniformsStorage,
    BackfaceCullingMode, Blend, Depth, DepthTest, DrawParameters, PolygonMode,
    Program as ShaderProgram, Surface,
};
use std::time::{self, Duration};
use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoopBuilder},
    window::WindowBuilder,
};

mod input;
mod mesh;
mod program_state;
mod transform;
mod update;
mod vertex;

fn main() {
    let draw_parameters = DrawParameters {
        depth: Depth {
            test: DepthTest::IfLessOrEqual,
            write: true,
            ..Default::default()
        },
        stencil: Stencil::default(),
        blend: Blend::alpha_blending(),
        backface_culling: BackfaceCullingMode::CullCounterClockwise,
        polygon_mode: PolygonMode::Fill,
        multisampling: false,
        dithering: false,
        provoking_vertex: ProvokingVertex::LastVertex,
        ..Default::default()
    };
    println!("initializng event loop");
    let event_loop = EventLoopBuilder::new().build();
    println!("building window");
    let (window, display) = SimpleWindowBuilder::new()
        .set_window_builder(
            WindowBuilder::new()
                .with_title("shell texture swag wow™")
                .with_visible(false),
        )
        .build(&event_loop);

    let program_state = AppState::new_ptr(window, display);

    let cam_shells;
    let cam_shell_program;
    {
        let program_state = program_state.borrow();
        println!("generating camera shell meshes");
        let shell_resolutions = [[63, 63]; 512];

        cam_shells = gen_cam_shells(&shell_resolutions, 4)
            .into_iter()
            .map(|mesh| {
                (
                    mesh.vertex_buffer(&program_state.display).unwrap(),
                    mesh.index_buffer(&program_state.display).unwrap(),
                )
            })
            .collect::<Vec<_>>();

        println!("initializng shader programs");
        cam_shell_program = ShaderProgram::from_source(
            &program_state.display,
            VERTEX_SHADER_SRC,
            FRAGMENT_SHADER_SRC,
            None,
        )
        .unwrap();
    }

    println!("running event loop");

    const DRAWS_PER_SEC: Duration = Duration::from_nanos(16_666_666);

    let draw = {
        let program_state = program_state.clone();
        let draw = move |now| {
            let mut program_state = program_state.borrow_mut();
            let mut frame = program_state.display.draw();
            frame.clear_color_and_depth((0.0, 0.0, 0.1, 1.0), 1.0);

            for (vertex_buffer, index_buffer) in &cam_shells {
                frame
                    .draw(
                        vertex_buffer,
                        index_buffer,
                        &cam_shell_program,
                        &UniformsStorage::new(
                            "projection",
                            program_state.camera.world_projection().to_cols_array_2d(),
                        )
                        .add(
                            "view",
                            program_state.camera.transfrom.mat().to_cols_array_2d(),
                        ),
                        &draw_parameters,
                    )
                    .unwrap();
            }

            frame.finish().unwrap();

            program_state.last_draw_time = Some(now);
        };
        draw
    };

    event_loop.run(move |ev, _, control_flow| {
        let now = time::Instant::now();
        let last_draw_time = program_state.borrow().last_draw_time;
        if let Some(last_draw_time) = last_draw_time {
            if (now - last_draw_time) >= DRAWS_PER_SEC {
                draw(now);
            }
        } else {
            draw(now);
            program_state.borrow().window.set_visible(true);
        };
        let mut program_state = program_state.borrow_mut();
        program_state.keyboard_state.downgrade_keys();
        match ev {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::ModifiersChanged(state) => {
                    program_state.keyboard_state.update_modifier_state(state);
                }
                WindowEvent::KeyboardInput { input, .. } => {
                    program_state.keyboard_state.update_keys(input)
                }
                WindowEvent::CloseRequested => {
                    control_flow.set_exit();
                }
                WindowEvent::Resized(size) => {
                    program_state.camera.update_aspect_ratio(size);
                }
                _ => (),
            },
            Event::RedrawRequested(_) => draw(now),

            _ => (),
        }
        match control_flow {
            ControlFlow::ExitWithCode(_) => (),
            _ => control_flow.set_wait_until(program_state.last_draw_time.unwrap() + DRAWS_PER_SEC),
        }
    });
}

const VERTEX_SHADER_SRC: &str = r#"
    #version 140

    //	Simplex 3D Noise
    //	by Ian McEwan, Ashima Arts
    //
    vec4 permute(vec4 x){return mod(((x*34.0)+1.0)*x, 289.0);}
    vec4 taylorInvSqrt(vec4 r){return 1.79284291400159 - 0.85373472095314 * r;}

    float snoise(vec3 v){
    const vec2  C = vec2(1.0/6.0, 1.0/3.0) ;
    const vec4  D = vec4(0.0, 0.5, 1.0, 2.0);

    // First corner
    vec3 i  = floor(v + dot(v, C.yyy) );
    vec3 x0 =   v - i + dot(i, C.xxx) ;

    // Other corners
    vec3 g = step(x0.yzx, x0.xyz);
    vec3 l = 1.0 - g;
    vec3 i1 = min( g.xyz, l.zxy );
    vec3 i2 = max( g.xyz, l.zxy );

    //  x0 = x0 - 0. + 0.0 * C
    vec3 x1 = x0 - i1 + 1.0 * C.xxx;
    vec3 x2 = x0 - i2 + 2.0 * C.xxx;
    vec3 x3 = x0 - 1. + 3.0 * C.xxx;

    // Permutations
    i = mod(i, 289.0 );
    vec4 p = permute( permute( permute(
                i.z + vec4(0.0, i1.z, i2.z, 1.0 ))
            + i.y + vec4(0.0, i1.y, i2.y, 1.0 ))
            + i.x + vec4(0.0, i1.x, i2.x, 1.0 ));

    // Gradients
    // ( N*N points uniformly over a square, mapped onto an octahedron.)
    float n_ = 1.0/7.0; // N=7
    vec3  ns = n_ * D.wyz - D.xzx;

    vec4 j = p - 49.0 * floor(p * ns.z *ns.z);  //  mod(p,N*N)

    vec4 x_ = floor(j * ns.z);
    vec4 y_ = floor(j - 7.0 * x_ );    // mod(j,N)

    vec4 x = x_ *ns.x + ns.yyyy;
    vec4 y = y_ *ns.x + ns.yyyy;
    vec4 h = 1.0 - abs(x) - abs(y);

    vec4 b0 = vec4( x.xy, y.xy );
    vec4 b1 = vec4( x.zw, y.zw );

    vec4 s0 = floor(b0)*2.0 + 1.0;
    vec4 s1 = floor(b1)*2.0 + 1.0;
    vec4 sh = -step(h, vec4(0.0));

    vec4 a0 = b0.xzyw + s0.xzyw*sh.xxyy ;
    vec4 a1 = b1.xzyw + s1.xzyw*sh.zzww ;

    vec3 p0 = vec3(a0.xy,h.x);
    vec3 p1 = vec3(a0.zw,h.y);
    vec3 p2 = vec3(a1.xy,h.z);
    vec3 p3 = vec3(a1.zw,h.w);

    //Normalise gradients
    vec4 norm = taylorInvSqrt(vec4(dot(p0,p0), dot(p1,p1), dot(p2, p2), dot(p3,p3)));
    p0 *= norm.x;
    p1 *= norm.y;
    p2 *= norm.z;
    p3 *= norm.w;

    // Mix final noise value
    vec4 m = max(0.6 - vec4(dot(x0,x0), dot(x1,x1), dot(x2,x2), dot(x3,x3)), 0.0);
    m = m * m;
    return 42.0 * dot( m*m, vec4( dot(p0,x0), dot(p1,x1),
                                    dot(p2,x2), dot(p3,x3) ) );
    }


    in vec3 position;

    noperspective out float geometry;
    flat out float distance;

    uniform mat4 projection;
    uniform mat4 view;

    void main() {
        gl_Position = vec4(position, 1.0);
        geometry = snoise((projection * view * gl_Position).xyz) * 0.5 + 0.5;
        distance = 1.0 - position.z;
    }
"#;
const FRAGMENT_SHADER_SRC: &str = "
    #version 140

    out vec4 color;

    noperspective in float geometry;
    flat in float distance;

    void main() {
        color = vec4(vec3(geometry*distance*distance), smoothstep(0.45,0.55,geometry));
    }
";
