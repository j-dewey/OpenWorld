use blocks::{CHUNK_WIDTH, CHUNK_HEIGHT};
use render::{voxel::{VoxelVertex, VoxelMesh, VoxelFaceRenders}, vertex::{TutorialVertex, SimpVertex}};
use wgpu::PipelineLayoutDescriptor;
use winit::{
    event,
    event_loop,
    window
};

mod render;
use render::{
    shader::Uniform,
    voxel::Voxel,
    mesh::MeshTrait
};

mod blocks;
mod input;
mod direction;
mod entity;
mod player;
mod physics;
mod world;
mod time_keep;
use time_keep::TimeKeep;

mod advanced_logging;
use advanced_logging as al;

pub async fn run() {
    env_logger::init();
    // create window and stuff
    let event_loop = event_loop::EventLoop::new();
    let window = window::WindowBuilder::new()
        .with_title("OpenWorld")
        .build(&event_loop).unwrap();
    let mut ws = render::WindowState::new(&window).await;
    let mut time_keeper = TimeKeep::new();
    let mut debug = false;

    let mut world = world::World::new(ws.get_device_ref());
    let mut el = entity::EntityList::new(window.inner_size().width, window.inner_size().height);

    // player and input
    let mut input_handler = input::InputHandler::new();
    let player_ref = el.get_player_mut();
    let camera_ref = player_ref.get_camera_ref_mut();
    let camera_uniform = camera_ref.create_uniform();

    // load shaders
    //ws.register_shader::<TutorialVertex>("tutorial".into(), include_str!("assets/tutorial_shader.wgsl"), Vec::new(), &[]);
    ws.register_shader::<TutorialVertex>("mesh".into(), include_str!("assets/mesh_shader.wgsl"), 
        vec![camera_uniform.get_bind_group(ws.get_device_ref())], 
        &[&render::camera::CameraUniform::get_bind_group_layout(ws.get_device_ref())]
    );
    ws.register_shader::<VoxelVertex>("voxel".into(), include_str!("assets/voxel_shader.wgsl"), 
        vec![camera_uniform.get_bind_group(ws.get_device_ref())], 
        &[&render::camera::CameraUniform::get_bind_group_layout(ws.get_device_ref())]
    );

    event_loop.run(move |event, _, control_flow| match event {
        event::Event::WindowEvent {
            ref event,
            window_id,
        } if window_id == window.id() && !input_handler.handle_input(event) => 
            match event {
                event::WindowEvent::CloseRequested | 
                event::WindowEvent::KeyboardInput {
                    input:
                        event::KeyboardInput {
                            state: event::ElementState::Pressed,
                            virtual_keycode: Some(event::VirtualKeyCode::Escape),
                            ..
                        },
                      ..
                } => *control_flow = event_loop::ControlFlow::Exit,
                event::WindowEvent::Resized(physical_size) => {
                    ws.resize(*physical_size);
                },
                event::WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                    // new_inner_size is &&mut so we have to dereference it twice
                    ws.resize(**new_inner_size);
                },
                _ => {}
            },
        event::Event::RedrawRequested(window_id) if window_id == window.id() => {
            // update camera uniform each frame
            let camera_ref = el.get_player_mut().get_camera_ref_mut();
            let new_camera_bind_group = camera_ref.create_uniform().get_bind_group(ws.get_device_ref_mut());
            ws.update_shader_bind_group(
                "voxel".into(),
                0, 
                new_camera_bind_group
            );

            match ws.render::<render::voxel::VoxelVertex, render::voxel::VoxelMesh>(world.get_chunk_meshes()) {
                Ok(_) => {}
                // Reconfigure the surface if lost
                Err(wgpu::SurfaceError::Lost) => ws.resize(ws.size),
                // The system is out of memory, we should probably quit
                Err(wgpu::SurfaceError::OutOfMemory) => *control_flow = event_loop::ControlFlow::Exit,
                // All other errors (Outdated, Timeout) should be resolved by the next frame                    Err(e) => eprintln!("{:?}", e),
                _ => {}
            }
        }
        event::Event::MainEventsCleared => {
            let dt = time_keeper.update_and_get_dt();
            println!("dt: {}", dt);
            println!("fps: {}", 1.0 / dt);
            // movement
            let forward = input_handler.get_key_event("forward".into());
            let backward = input_handler.get_key_event("backward".into());
            let left = input_handler.get_key_event("strafe-left".into());
            let right = input_handler.get_key_event("strafe-right".into());
            let up = input_handler.get_key_event("up".into());
            let down = input_handler.get_key_event("down".into());
            // rotation
            let look_up = input_handler.get_key_event("rotate-up".into());
            let look_down = input_handler.get_key_event("rotate-down".into());
            let look_left = input_handler.get_key_event("rotate-left".into());
            let look_right = input_handler.get_key_event("rotate-right".into());
            // funnnnnn stuffff
            let toggle_physics = input_handler.check_new_event("toggle-physics".into());
            let toggle_debug = input_handler.check_new_event("toggle-debug".into());

            input_handler.flush_new_presses();

            let player_ref = el.get_player_mut();
            // fun toggles
            if toggle_physics{
                player_ref.physics_on = !player_ref.physics_on;
            }
            if toggle_debug{
                debug = !debug;
            }
            // player input
            player_ref.handle_input(
                [ forward as i32 + -1*backward as i32, right as i32 + -1*left as i32, up as i32 + -1*down as i32],
                [ look_right as i32 + -1*look_left as i32, look_up as i32 + -1*look_down as i32],
                dt
            );

            el.update(&world, dt);

            // RedrawRequested will only trigger once, unless we manually
            // request it.
            window.request_redraw();
        }
        _ => {}
    });
}


fn main() {
    // create window and windowstate
    pollster::block_on(run());
}
