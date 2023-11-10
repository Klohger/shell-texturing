use glium::{
    backend::glutin::SimpleWindowBuilder,
    draw_parameters::{
        ClipControlDepth, ClipControlOrigin, DepthClamp, PolygonOffset, ProvokingVertex, Stencil,
    },
    index::PrimitiveType,
    uniform, BackfaceCullingMode, Blend, BlendingFunction, Depth, DepthTest, DrawParameters,
    LinearBlendingFactor, PolygonMode, Program as ShaderProgram, StencilOperation, StencilTest,
    Surface,
};
use std::time::{self, Duration};
use vertex::Vertex;
use winit::{
    event::{Event, WindowEvent},
    event_loop::EventLoopBuilder,
    window::WindowBuilder,
};

mod mesh;
mod program_state;
mod transform;
mod vertex;

const PARAMS: DrawParameters = DrawParameters {
    depth: Depth {
        test: DepthTest::IfLessOrEqual,
        write: true,
        range: (0.0, 1.0),
        clamp: DepthClamp::NoClamp,
    },
    stencil: Stencil {
        test_clockwise: StencilTest::AlwaysPass,
        reference_value_clockwise: 0,
        write_mask_clockwise: 0xffffffff,
        fail_operation_clockwise: StencilOperation::Keep,
        pass_depth_fail_operation_clockwise: StencilOperation::Keep,
        depth_pass_operation_clockwise: StencilOperation::Keep,
        test_counter_clockwise: StencilTest::AlwaysPass,
        reference_value_counter_clockwise: 0,
        write_mask_counter_clockwise: 0xffffffff,
        fail_operation_counter_clockwise: StencilOperation::Keep,
        pass_depth_fail_operation_counter_clockwise: StencilOperation::Keep,
        depth_pass_operation_counter_clockwise: StencilOperation::Keep,
    },
    blend: Blend {
        color: BlendingFunction::Addition {
            source: LinearBlendingFactor::SourceAlpha,
            destination: LinearBlendingFactor::OneMinusSourceAlpha,
        },
        alpha: BlendingFunction::Addition {
            source: LinearBlendingFactor::SourceAlpha,
            destination: LinearBlendingFactor::OneMinusSourceAlpha,
        },
        constant_value: (0.0, 0.0, 0.0, 0.0),
    },
    color_mask: (true, true, true, true),
    line_width: None,
    point_size: None,
    backface_culling: BackfaceCullingMode::CullCounterClockwise,
    polygon_mode: PolygonMode::Fill,
    clip_planes_bitmask: 0,
    multisampling: false,
    dithering: true,
    viewport: None,
    scissor: None,
    draw_primitives: true,
    samples_passed_query: None,
    time_elapsed_query: None,
    primitives_generated_query: None,
    transform_feedback_primitives_written_query: None,
    condition: None,
    transform_feedback: None,
    smooth: None,
    provoking_vertex: ProvokingVertex::LastVertex,
    primitive_bounding_box: (-1.0..1.0, -1.0..1.0, -1.0..1.0, -1.0..1.0),
    primitive_restart_index: false,
    polygon_offset: PolygonOffset {
        factor: 0.0,
        units: 0.0,
        point: false,
        line: false,
        fill: false,
    },
    clip_control_origin: ClipControlOrigin::LowerLeft,
    clip_control_depth: ClipControlDepth::NegativeOneToOne,
};

fn main() {
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

    let program_state = program_state::ProgramState::new(window, display);

    println!("generating camera shell meshes");
    let shell_resolutions = [[63, 63]; 512];

    let cam_shells = cam_shells(&shell_resolutions, 4)
        .into_iter()
        .map(|mesh| {
            (
                mesh.vertex_buffer(&*program_state.display.borrow())
                    .unwrap(),
                mesh.index_buffer(&*program_state.display.borrow()).unwrap(),
            )
        })
        .collect::<Vec<_>>();

    println!("initializng shader programs");
    let cam_shell_program = ShaderProgram::from_source(
        &*program_state.display.borrow(),
        VERTEX_SHADER_SRC,
        FRAGMENT_SHADER_SRC,
        None,
    )
    .unwrap();

    let mut last_draw_time = None;

    println!("running event loop");

    const DRAWS_PER_SEC: Duration = Duration::from_nanos(16_666_666);

    event_loop.run(move |ev, _, control_flow| {

        let draw = {
            let program_state = program_state.clone();
            let cam_shells = &cam_shells;
            let cam_shell_program = &cam_shell_program;
            let draw = move |first_draw: bool| {
                let mut frame = program_state.display.borrow().draw();
                frame.clear_color_and_depth((0.0, 0.0, 0.1, 1.0), 1.0);

                for (vertex_buffer, index_buffer) in cam_shells {
                    frame
                        .draw(
                            vertex_buffer,
                            index_buffer,
                            cam_shell_program,
                            &uniform! { projection: program_state.world_projection.borrow().0.to_cols_array_2d(), view: program_state.camera.borrow().mat().to_cols_array_2d() },
                            &PARAMS,
                        )
                        .unwrap();
                }

                frame.finish().unwrap();

                if first_draw {
                    program_state.window.borrow_mut().set_visible(true);
                }
                return time::Instant::now();
            };
            draw
        };

        let now = time::Instant::now();
        if let Some(_last_draw_time) = last_draw_time {
            if (now - _last_draw_time) >= DRAWS_PER_SEC {
                last_draw_time = Some(draw(false));
            }
        } else {
            last_draw_time = Some(draw(true));
        };
        match ev {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => {
                    control_flow.set_exit();
                }
                WindowEvent::Resized(size) => {
                    program_state.update_aspect_ratio(size);
                }
                _ => (),
            },
            Event::RedrawRequested(_) => {
                println!("redraw requested");
                last_draw_time = Some(draw(false))
            }
            _ => (),
        }
        match control_flow {
            winit::event_loop::ControlFlow::ExitWithCode(_) => (),
            _ => control_flow.set_wait_until(last_draw_time.unwrap() + DRAWS_PER_SEC),
        }
    });
}

fn cam_shells(shell_resolutions: &[[usize; 2]], z_pow: i32) -> Vec<mesh::Mesh> {
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
                (z + program_state::ProgramState::CAM_NEAR, shell_resolution)
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

        meshes.push(mesh::Mesh {
            vertices: verts,
            indices: indices,
            primitive_type: PrimitiveType::TrianglesList,
        })
    }

    return meshes;
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
    void main() {
        gl_Position = vec4(position, 1.0);
        geometry = snoise((projection * gl_Position).xyz) * 0.5 + 0.5;
        distance = 1.0 - position.z;
    }
"#;
const FRAGMENT_SHADER_SRC: &str = "
    #version 140

    out vec4 color;

    noperspective in float geometry;
    flat in float distance;

    void main() {
        color = vec4(vec3(distance), smoothstep(0.45,0.55,geometry));
    }
";
