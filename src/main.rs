use glium::{
    draw_parameters::{
        ClipControlDepth, ClipControlOrigin, DepthClamp, PolygonOffset, ProvokingVertex, Stencil,
    },
    glutin::{
        config::ConfigTemplateBuilder,
        context::{ContextAttributesBuilder, NotCurrentGlContextSurfaceAccessor},
        display::{GetGlDisplay, GlDisplay},
        surface::{SurfaceAttributesBuilder, WindowSurface},
    },
    index::{IndicesSource, PrimitiveType},
    uniform, BackfaceCullingMode, Blend, BlendingFunction, Depth, DepthTest, Display,
    DrawParameters, LinearBlendingFactor, PolygonMode, Program as ShaderProgram, StencilOperation,
    StencilTest, Surface, VertexBuffer,
};
use raw_window_handle::HasRawWindowHandle;

use std::{
    num::NonZeroU32,
    time::{self, Duration},
};
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
        test: DepthTest::IfMore,
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

const VERTEX_SHADER_SRC: &str = r#"
#version 140

precision lowp float;

in vec3 position;
out vec2 v_position;
uniform float scale;
uniform vec3 centre;

void main() {
    gl_Position = vec4(position*scale + centre, 1.0);
    v_position = position.xy;
}
"#;

const FRAGMENT_SHADER_SRC: &str = "
#version 140

precision lowp float;

out vec4 fragment_color;
in vec2 v_position;
uniform vec3 centre;
uniform vec3 color;
uniform float scale;

void main() {
    float circle = 1.0 - length(v_position);
    float depth;
    float alpha = 1;
    vec2 depth_and_alpha = (circle <= 0.0) ? vec2(0.0, 0.0) : vec2(circle*scale + centre.z, 1.0);
    gl_FragDepth = depth_and_alpha.x;
    fragment_color = vec4(color, depth_and_alpha.y);
}
";
fn main() {
    println!("initializng event loop");
    let event_loop = EventLoopBuilder::new().build();
    println!("building window");
    let (window, display) = {
        // First we start by opening a new Window
        let display_builder = glutin_winit::DisplayBuilder::new().with_window_builder(Some(
            WindowBuilder::new()
                .with_title("shell texture swag wow™")
                .with_visible(false),
        ));
        let config_template_builder = ConfigTemplateBuilder::new();

        let (window, gl_config) = display_builder
            .build(&event_loop, config_template_builder, |mut configs| {
                // Just use the first configuration since we don't have any special preferences here
                configs.next().unwrap()
            })
            .unwrap();
        let window = window.unwrap();

        // Now we get the window size to use as the initial size of the Surface
        let (width, height): (u32, u32) = window.inner_size().into();
        let attrs = SurfaceAttributesBuilder::<WindowSurface>::new().build(
            window.raw_window_handle(),
            NonZeroU32::new(width).unwrap(),
            NonZeroU32::new(height).unwrap(),
        );

        // Finally we can create a Surface, use it to make a PossiblyCurrentContext and create the glium Display
        let surface = unsafe {
            gl_config
                .display()
                .create_window_surface(&gl_config, &attrs)
                .unwrap()
        };
        let context_attributes =
            ContextAttributesBuilder::new().build(Some(window.raw_window_handle()));
        let current_context = Some(unsafe {
            gl_config
                .display()
                .create_context(&gl_config, &context_attributes)
                .expect("failed to create context")
        })
        .unwrap()
        .make_current(&surface)
        .unwrap();
        let display = Display::from_context_surface(current_context, surface).unwrap();

        (window, display)
    };

    let program_state = program_state::ProgramState::new(
        window,
        display,
        vec![
            ([0.0_f32, 0.0, 0.5], [1.0_f32, 1.0, 1.0], 0.5_f32, 0.0),
            (
                [0.288675135, 0.288675135, 0.644337567],
                [0.0, 0.0, 0.0],
                0.25,
                -0.5,
            ),
            (
                [-0.288675135, 0.288675135, 0.644337567],
                [0.0, 0.0, 0.0],
                0.25,
                -0.6,
            ),
        ],
    );

    println!("generating camera shell meshes");

    let (vertices, indices) = (
        VertexBuffer::immutable(
            &*program_state.display.borrow(),
            &[
                Vertex::new([1.0, -1.0, 0.0]),
                Vertex::new([-1.0, -1.0, 0.0]),
                Vertex::new([1.0, 1.0, 0.0]),
                Vertex::new([-1.0, 1.0, 0.0]),
            ],
        )
        .unwrap(),
        IndicesSource::NoIndices {
            primitives: PrimitiveType::TriangleStrip,
        },
    );
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
    let mut t: f32 = 0.0;
    event_loop.run(move |ev, _, control_flow| {
        let mut update = {
            let program_state = program_state.clone();
            let t = &mut t;
            let update = move || {
                for (centre, colour, scale, time_offset) in
                    program_state.uniforms.borrow_mut().iter_mut()
                {
                    centre[1] = (*t + *time_offset).sin()*0.5;
                    *t = *t + DRAWS_PER_SEC.as_secs_f32();
                }
            };
            update
        };
        let draw = {
            let program_state = program_state.clone();
            let cam_shell_program_ref = &cam_shell_program;
            let vertices = &vertices;
            let indices = &indices;

            let draw = move |first_draw: bool| {
                let mut frame = program_state.display.borrow().draw();

                frame.clear_color_and_depth((0.0, 0.0, 0.1, 1.0), 0.0);

                for (centre, colour, scale, _) in program_state.uniforms.borrow().iter() {
                    frame
                        .draw(
                            vertices,
                            indices.clone(),
                            cam_shell_program_ref,
                            &uniform! {
                                projection: program_state.world_projection
                                .borrow().0.to_cols_array_2d(),
                                view: program_state.camera.borrow().mat().to_cols_array_2d(),
                                centre: *centre,
                                color: *colour,
                                scale: *scale
                            },
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
                update();
                last_draw_time = Some(draw(false));
            }
        } else {
            update();
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
